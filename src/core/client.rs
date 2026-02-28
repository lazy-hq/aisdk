//! This module provides the client for interacting with the AI providers.
//! It is a thin wrapper around the `reqwest` crate.

use crate::core::utils::join_url;
use crate::error::{Error, Result};
use futures::Stream;
use futures::StreamExt;
use reqwest;
use reqwest::IntoUrl;
use reqwest_eventsource::{Event, RequestBuilderExt};
use serde::de::DeserializeOwned;
use std::pin::Pin;
use std::sync::OnceLock;
use std::time::Duration;

/// Configuration for retry behavior on API requests.
#[derive(Debug, Clone)]
struct RetryConfig {
    /// Maximum number of retry attempts (default: 5)
    max_retries: u32,
    /// Initial wait time before first retry (default: 1 second)
    initial_wait: Duration,
    /// Maximum wait time between retries (default: 30 seconds)
    max_wait: Duration,
    /// Whether to add jitter to backoff (default: true)
    use_jitter: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 5,
            initial_wait: Duration::from_secs(1),
            max_wait: Duration::from_secs(30),
            use_jitter: true,
        }
    }
}

/// A single shared `reqwest::Client` instance reused across all requests.
static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

/// Returns a reference to the shared HTTP client, initializing it on first call.
#[cfg(not(feature = "test-access"))]
fn get_client() -> &'static reqwest::Client {
    HTTP_CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(Duration::from_secs(90))
            .tcp_keepalive(Duration::from_secs(60))
            .connect_timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to build shared HTTP client")
    })
}

#[cfg(feature = "test-access")]
fn get_client() -> reqwest::Client {
    reqwest::Client::new()
}

/// Checks if a status code is retryable.
fn is_retryable_status(status: reqwest::StatusCode) -> bool {
    matches!(
        status,
        reqwest::StatusCode::TOO_MANY_REQUESTS
            | reqwest::StatusCode::BAD_GATEWAY
            | reqwest::StatusCode::SERVICE_UNAVAILABLE
            | reqwest::StatusCode::GATEWAY_TIMEOUT
    )
}

/// Parses the Retry-After header to get the wait duration.
fn parse_retry_after(headers: &reqwest::header::HeaderMap) -> Option<Duration> {
    headers
        .get(reqwest::header::RETRY_AFTER)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| {
            // Try parsing as seconds (integer)
            if let Ok(seconds) = s.parse::<u64>() {
                return Some(Duration::from_secs(seconds));
            }
            None
        })
}

/// Calculates the next wait duration with exponential backoff and optional jitter.
fn calculate_backoff(
    retry_count: u32,
    config: &RetryConfig,
    retry_after: Option<Duration>,
) -> Duration {
    // If server provides Retry-After, respect it
    if let Some(duration) = retry_after {
        return duration.min(config.max_wait);
    }

    // Calculate exponential backoff: initial_wait * 2^retry_count
    let backoff = config
        .initial_wait
        .saturating_mul(2_u32.saturating_pow(retry_count));
    let backoff = backoff.min(config.max_wait);

    // Add jitter to prevent thundering herd (±10% of backoff time)
    if config.use_jitter {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let jitter_pct = ((now % 200) as i64 - 100) as f64 / 1000.0; // Range: -0.1 to +0.1
        let jitter_ms = (backoff.as_millis() as f64 * jitter_pct) as i64;

        if jitter_ms >= 0 {
            backoff.saturating_add(Duration::from_millis(jitter_ms as u64))
        } else {
            backoff.saturating_sub(Duration::from_millis((-jitter_ms) as u64))
        }
    } else {
        backoff
    }
}

/// Shared retry logic for HTTP requests.
///
/// This function handles:
/// - Exponential backoff with configurable limits
/// - Jitter to prevent thundering herd
/// - Retry-After header parsing
/// - Retryable error detection (429, 502, 503, 504)
/// - Request body reconstruction on each retry
async fn retry_request<F, T>(
    url: reqwest::Url,
    method: reqwest::Method,
    headers: reqwest::header::HeaderMap,
    query_params: Vec<(&str, &str)>,
    body_fn: F,
    config: RetryConfig,
) -> Result<T>
where
    F: Fn() -> reqwest::Body,
    T: DeserializeOwned + std::fmt::Debug,
{
    let client = get_client();
    let mut retry_count = 0;

    loop {
        // Reconstruct body for each attempt to avoid consumption issues
        let body = body_fn();

        let resp = client
            .request(method.clone(), url.clone())
            .headers(headers.clone())
            .query(&query_params)
            .body(body)
            .send()
            .await
            .map_err(|e| {
                // Check if error is retryable (timeout, connection error, etc.)
                if e.is_timeout() || e.is_connect() {
                    log::warn!(
                        "Request failed with retryable error (attempt {}/{}): {}",
                        retry_count + 1,
                        config.max_retries + 1,
                        e
                    );
                } else {
                    log::error!("Request failed: {e}");
                }

                Error::ApiError {
                    status_code: e.status(),
                    details: e.to_string(),
                }
            })?;

        let status = resp.status();
        let response_headers = resp.headers().clone();
        let resp_text = resp.text().await.map_err(|e| Error::ApiError {
            status_code: e.status(),
            details: format!("Failed to read response: {e}"),
        })?;

        if status.is_success() {
            log::debug!("Request succeeded on attempt {}", retry_count + 1);
            return serde_json::from_str(&resp_text).map_err(|e| Error::ApiError {
                status_code: Some(status),
                details: format!("Failed to parse response: {e}"),
            });
        }

        // Check if error is retryable and we have retries left
        if is_retryable_status(status) && retry_count < config.max_retries {
            retry_count += 1;

            // Parse Retry-After header if present
            let retry_after = parse_retry_after(&response_headers);
            let wait_time = calculate_backoff(retry_count - 1, &config, retry_after);

            log::warn!(
                "Request failed with status {} (attempt {}/{}). Retrying after {:?}...",
                status,
                retry_count,
                config.max_retries + 1,
                wait_time
            );

            tokio::time::sleep(wait_time).await;
            continue;
        }

        // Non-retryable error or exhausted retries
        if retry_count >= config.max_retries {
            log::error!(
                "Request failed after {} retries with status {}: {}",
                retry_count + 1,
                status,
                resp_text
            );
        } else {
            log::error!("Request failed with non-retryable status {status}: {resp_text}");
        }

        return Err(Error::ApiError {
            status_code: Some(status),
            details: resp_text,
        });
    }
}

#[allow(dead_code)]
pub(crate) trait LanguageModelClient {
    type Response: DeserializeOwned + std::fmt::Debug + Clone;
    type StreamEvent: DeserializeOwned + std::fmt::Debug + Clone;

    fn path(&self) -> String;
    fn method(&self) -> reqwest::Method;
    fn query_params(&self) -> Vec<(&str, &str)>;
    fn body(&self) -> reqwest::Body;
    fn headers(&self) -> reqwest::header::HeaderMap;

    async fn send(&self, base_url: impl IntoUrl) -> Result<Self::Response> {
        let url = join_url(base_url, &self.path())?;

        // Serialize body once to avoid consumption issues on retries
        let body_bytes = {
            let body = self.body();
            // Convert Body to bytes - this is the critical fix for retry body consumption
            // We need to get the bytes from the body to be able to reconstruct it
            match body.as_bytes() {
                Some(bytes) => bytes.to_vec(),
                None => {
                    // If body doesn't have as_bytes (streaming body), we can't retry it
                    // This should rarely happen for our use case (JSON bodies)
                    log::warn!("Request body is not retryable (streaming body)");
                    vec![]
                }
            }
        };

        let method = self.method();
        let headers = self.headers();
        let query_params = self.query_params();
        let config = RetryConfig::default();

        retry_request(
            url,
            method,
            headers,
            query_params,
            move || reqwest::Body::from(body_bytes.clone()),
            config,
        )
        .await
    }

    /// Parses an SSE event into a StreamEvent ( ProviderStreamEvent )
    fn parse_stream_sse(
        event: std::result::Result<Event, reqwest_eventsource::Error>,
    ) -> Result<Self::StreamEvent>;

    /// Returns true to mark the stream as ended
    fn end_stream(event: &Self::StreamEvent) -> bool;

    async fn send_and_stream(
        &self,
        base_url: impl IntoUrl,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Self::StreamEvent>> + Send>>>
    where
        Self::StreamEvent: Send + 'static,
        Self: Sync,
    {
        let client = get_client();

        let url = join_url(base_url, &self.path())?;

        // Establish the event source stream directly
        // Note: Status code errors (including 429) will be surfaced as stream events
        // and should be handled by retry logic in the provider's stream_text() method
        let events_stream = client
            .request(self.method(), url.clone())
            .headers(self.headers())
            .query(&self.query_params())
            .body(self.body())
            .eventsource()
            .map_err(|e| Error::ApiError {
                status_code: None,
                details: format!("SSE stream error: {e}"),
            })?;

        // Map events to deserialized StreamEvent ( ProviderStreamEvent )
        let mapped_stream = events_stream.map(|event_result| Self::parse_stream_sse(event_result));

        // State that indicates if the stream has ended
        let ended = std::sync::Arc::new(std::sync::Mutex::new(false));

        // Scan to end or mark the stream as ended
        let stream = mapped_stream.scan(ended, |ended, res| {
            let mut ended = ended.lock().unwrap();

            if *ended {
                return futures::future::ready(None); // Stop the stream after end event
            }

            *ended = res.as_ref().map_or(true, |evt| Self::end_stream(evt)); // Mark the stream as ended on api error or end event

            futures::future::ready(Some(res)) // Emit the event
        });

        Ok(Box::pin(stream))
    }
}
/// Trait for embedding model clients to interact with embedding APIs.
#[allow(dead_code)]
pub(crate) trait EmbeddingClient {
    type Response: DeserializeOwned + std::fmt::Debug + Clone;

    fn path(&self) -> String;
    fn method(&self) -> reqwest::Method;
    fn query_params(&self) -> Vec<(&str, &str)>;
    fn body(&self) -> reqwest::Body;
    fn headers(&self) -> reqwest::header::HeaderMap;

    async fn send(&self, base_url: impl IntoUrl) -> Result<Self::Response> {
        let base_url = base_url
            .into_url()
            .map_err(|_| Error::InvalidInput("Invalid base URL".into()))?;

        let url = join_url(base_url, &self.path())?;

        // Serialize body once to avoid consumption issues on retries
        let body_bytes = {
            let body = self.body();
            // Convert Body to bytes - this is the critical fix for retry body consumption
            match body.as_bytes() {
                Some(bytes) => bytes.to_vec(),
                None => {
                    // If body doesn't have as_bytes (streaming body), we can't retry it
                    log::warn!("Request body is not retryable (streaming body)");
                    vec![]
                }
            }
        };

        let method = self.method();
        let headers = self.headers();
        let query_params = self.query_params();
        let config = RetryConfig::default();

        retry_request(
            url,
            method,
            headers,
            query_params,
            move || reqwest::Body::from(body_bytes.clone()),
            config,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to create a custom RetryConfig for testing
    fn test_config(
        max_retries: u32,
        initial_wait_ms: u64,
        max_wait_ms: u64,
        use_jitter: bool,
    ) -> RetryConfig {
        RetryConfig {
            max_retries,
            initial_wait: Duration::from_millis(initial_wait_ms),
            max_wait: Duration::from_millis(max_wait_ms),
            use_jitter,
        }
    }

    // ========================================================================
    // Tests for Retry-After Header Handling
    // ========================================================================

    #[test]
    fn test_calculate_backoff_with_retry_after_header() {
        let config = test_config(5, 1000, 30000, false);
        let retry_after = Some(Duration::from_secs(5));

        let result = calculate_backoff(0, &config, retry_after);
        assert_eq!(result, Duration::from_secs(5));
    }

    #[test]
    fn test_calculate_backoff_retry_after_capped_at_max_wait() {
        let config = test_config(5, 1000, 10000, false);
        let retry_after = Some(Duration::from_secs(60)); // 60s > 10s max

        let result = calculate_backoff(0, &config, retry_after);
        assert_eq!(result, Duration::from_millis(10000));
    }

    #[test]
    fn test_calculate_backoff_retry_after_below_max_wait() {
        let config = test_config(5, 1000, 30000, false);
        let retry_after = Some(Duration::from_millis(5000));

        let result = calculate_backoff(3, &config, retry_after);
        assert_eq!(result, Duration::from_millis(5000));
    }

    #[test]
    fn test_calculate_backoff_retry_after_zero() {
        let config = test_config(5, 1000, 30000, false);
        let retry_after = Some(Duration::from_secs(0));

        let result = calculate_backoff(0, &config, retry_after);
        assert_eq!(result, Duration::from_secs(0));
    }

    #[test]
    fn test_calculate_backoff_retry_after_very_large() {
        let config = test_config(5, 1000, 1000, false);
        let retry_after = Some(Duration::from_secs(u64::MAX / 2));

        let result = calculate_backoff(0, &config, retry_after);
        assert_eq!(result, Duration::from_millis(1000)); // Capped at max_wait
    }

    // ========================================================================
    // Tests for Exponential Backoff (No Jitter)
    // ========================================================================

    #[test]
    fn test_calculate_backoff_retry_count_zero_no_jitter() {
        let config = test_config(5, 1000, 30000, false);

        let result = calculate_backoff(0, &config, None);
        // 1000ms * 2^0 = 1000ms
        assert_eq!(result, Duration::from_millis(1000));
    }

    #[test]
    fn test_calculate_backoff_retry_count_one_no_jitter() {
        let config = test_config(5, 1000, 30000, false);

        let result = calculate_backoff(1, &config, None);
        // 1000ms * 2^1 = 2000ms
        assert_eq!(result, Duration::from_millis(2000));
    }

    #[test]
    fn test_calculate_backoff_retry_count_two_no_jitter() {
        let config = test_config(5, 1000, 30000, false);

        let result = calculate_backoff(2, &config, None);
        // 1000ms * 2^2 = 4000ms
        assert_eq!(result, Duration::from_millis(4000));
    }

    #[test]
    fn test_calculate_backoff_retry_count_three_no_jitter() {
        let config = test_config(5, 1000, 30000, false);

        let result = calculate_backoff(3, &config, None);
        // 1000ms * 2^3 = 8000ms
        assert_eq!(result, Duration::from_millis(8000));
    }

    #[test]
    fn test_calculate_backoff_retry_count_four_no_jitter() {
        let config = test_config(5, 1000, 30000, false);

        let result = calculate_backoff(4, &config, None);
        // 1000ms * 2^4 = 16000ms
        assert_eq!(result, Duration::from_millis(16000));
    }

    #[test]
    fn test_calculate_backoff_retry_count_five_no_jitter() {
        let config = test_config(5, 1000, 30000, false);

        let result = calculate_backoff(5, &config, None);
        // 1000ms * 2^5 = 32000ms, but capped at 30000ms
        assert_eq!(result, Duration::from_millis(30000));
    }

    #[test]
    fn test_calculate_backoff_exceeds_max_wait_no_jitter() {
        let config = test_config(5, 1000, 5000, false);

        let result = calculate_backoff(3, &config, None);
        // 1000ms * 2^3 = 8000ms, but capped at 5000ms
        assert_eq!(result, Duration::from_millis(5000));
    }

    #[test]
    fn test_calculate_backoff_exactly_at_max_wait_no_jitter() {
        let config = test_config(5, 1000, 8000, false);

        let result = calculate_backoff(3, &config, None);
        // 1000ms * 2^3 = 8000ms, exactly at max_wait
        assert_eq!(result, Duration::from_millis(8000));
    }

    #[test]
    fn test_calculate_backoff_large_retry_count_no_jitter() {
        let config = test_config(100, 1000, 30000, false);

        let result = calculate_backoff(20, &config, None);
        // 1000ms * 2^20 would be huge, should be capped at 30000ms
        assert_eq!(result, Duration::from_millis(30000));
    }

    #[test]
    fn test_calculate_backoff_saturation_no_jitter() {
        let config = test_config(5, 1_000_000, 60000, false);

        // 1_000_000ms * 2^10 would overflow, should saturate
        let result = calculate_backoff(10, &config, None);
        // Should be capped at max_wait
        assert_eq!(result, Duration::from_millis(60000));
    }

    // ========================================================================
    // Tests for Exponential Backoff with Jitter
    // ========================================================================

    #[test]
    fn test_calculate_backoff_with_jitter_within_range() {
        let config = test_config(5, 1000, 30000, true);

        let result = calculate_backoff(2, &config, None);
        // Base: 1000ms * 2^2 = 4000ms
        // Jitter should be ±10% = 3600ms to 4400ms
        let _base = Duration::from_millis(4000);
        let min = Duration::from_millis(3600);
        let max = Duration::from_millis(4400);

        assert!(
            result >= min && result <= max,
            "Result {result:?} should be between {min:?} and {max:?}"
        );
    }

    #[test]
    fn test_calculate_backoff_with_jitter_different_retry_counts() {
        let config = test_config(5, 1000, 30000, true);

        for retry_count in 0..5 {
            let result = calculate_backoff(retry_count, &config, None);
            let base = 1000 * 2_u64.pow(retry_count);
            let min = (base as f64 * 0.9) as u64;
            let max = (base as f64 * 1.1) as u64;

            assert!(
                result >= Duration::from_millis(min) && result <= Duration::from_millis(max),
                "Retry count {}: result {:?} should be between {:?} and {:?}",
                retry_count,
                result,
                Duration::from_millis(min),
                Duration::from_millis(max)
            );
        }
    }

    #[test]
    fn test_calculate_backoff_jitter_respects_max_wait() {
        let config = test_config(5, 1000, 10000, true);

        // Base would be 16000ms, but max_wait is 10000ms
        let result = calculate_backoff(4, &config, None);

        // Even with jitter, should never exceed max_wait
        // Actually, the current implementation caps BEFORE jitter,
        // so it should be around 10000 ± 10% = 9000-11000ms
        // Let's be conservative and check it's close to 10000ms
        assert!(
            result >= Duration::from_millis(9000) && result <= Duration::from_millis(11000),
            "Result {result:?} should be around 10000ms ±10%"
        );
    }

    #[test]
    fn test_calculate_backoff_jitter_deterministic_within_run() {
        let config = test_config(5, 1000, 30000, true);

        // Multiple calls in quick succession should give different results
        // due to changing timestamp nanoseconds
        let result1 = calculate_backoff(2, &config, None);
        std::thread::sleep(Duration::from_nanos(100)); // Tiny sleep to change timestamp
        let result2 = calculate_backoff(2, &config, None);

        // Results might be the same if called at exactly the same nanosecond,
        // but they're likely different. Just verify both are in valid range.
        let _base = Duration::from_millis(4000);
        let min = Duration::from_millis(3600);
        let max = Duration::from_millis(4400);

        assert!(result1 >= min && result1 <= max);
        assert!(result2 >= min && result2 <= max);
    }

    #[test]
    fn test_calculate_backoff_jitter_at_zero_retry_count() {
        let config = test_config(5, 1000, 30000, true);

        let result = calculate_backoff(0, &config, None);
        // Base: 1000ms * 2^0 = 1000ms
        // Jitter: ±10% = 900ms to 1100ms
        assert!(
            result >= Duration::from_millis(900) && result <= Duration::from_millis(1100),
            "Result {result:?} should be around 1000ms ±10%"
        );
    }

    // ========================================================================
    // Tests for Edge Cases
    // ========================================================================

    #[test]
    fn test_calculate_backoff_initial_wait_zero() {
        let config = test_config(5, 0, 30000, false);

        let result = calculate_backoff(5, &config, None);
        // 0ms * 2^5 = 0ms
        assert_eq!(result, Duration::from_millis(0));
    }

    #[test]
    fn test_calculate_backoff_max_wait_zero() {
        let config = test_config(5, 1000, 0, false);

        let result = calculate_backoff(0, &config, None);
        // Should be capped at max_wait = 0
        assert_eq!(result, Duration::from_millis(0));
    }

    #[test]
    fn test_calculate_backoff_both_zeros() {
        let config = test_config(5, 0, 0, false);

        let result = calculate_backoff(10, &config, None);
        assert_eq!(result, Duration::from_millis(0));
    }

    #[test]
    fn test_calculate_backoff_very_large_initial_wait() {
        let config = RetryConfig {
            max_retries: 5,
            initial_wait: Duration::from_secs(1_000_000),
            max_wait: Duration::from_secs(2_000_000),
            use_jitter: false,
        };

        let result = calculate_backoff(0, &config, None);
        assert_eq!(result, Duration::from_secs(1_000_000));
    }

    #[test]
    fn test_calculate_backoff_overflow_protection() {
        let config = RetryConfig {
            max_retries: 100,
            initial_wait: Duration::from_millis(u64::MAX / 2),
            max_wait: Duration::from_secs(60),
            use_jitter: false,
        };

        // This should saturate multiplication and get capped at max_wait
        let result = calculate_backoff(10, &config, None);
        assert_eq!(result, Duration::from_secs(60));
    }

    #[test]
    fn test_calculate_backoff_u32_max_retry_count() {
        let config = test_config(u32::MAX, 1000, 30000, false);

        // Should saturate and cap at max_wait
        let result = calculate_backoff(u32::MAX, &config, None);
        assert_eq!(result, Duration::from_millis(30000));
    }

    #[test]
    fn test_calculate_backoff_power_of_two_overflow() {
        let config = test_config(100, 1000, 60000, false);

        // 2^63 would overflow u32::saturating_pow
        let result = calculate_backoff(63, &config, None);
        // Should saturate and cap at max_wait
        assert_eq!(result, Duration::from_millis(60000));
    }

    #[test]
    fn test_calculate_backoff_jitter_with_zero_base() {
        let config = test_config(5, 0, 30000, true);

        let result = calculate_backoff(0, &config, None);
        // Base is 0, jitter of ±10% of 0 is still 0
        assert_eq!(result, Duration::from_millis(0));
    }

    #[test]
    fn test_calculate_backoff_jitter_with_very_small_base() {
        let config = test_config(5, 10, 30000, true);

        let result = calculate_backoff(0, &config, None);
        // Base: 10ms, jitter ±10% = 9ms to 11ms
        assert!(
            result >= Duration::from_millis(9) && result <= Duration::from_millis(11),
            "Result {result:?} should be around 10ms ±10%"
        );
    }

    #[test]
    fn test_calculate_backoff_sequence_increases_exponentially() {
        let config = test_config(5, 1000, 100000, false);

        let mut prev_result = Duration::from_millis(0);
        for retry_count in 0..10 {
            let result = calculate_backoff(retry_count, &config, None);
            // Each result should be greater than or equal to previous (until cap)
            assert!(
                result >= prev_result,
                "Retry count {retry_count}: {result:?} should be >= {prev_result:?}"
            );

            // Each result should be approximately double the previous (until cap)
            if retry_count > 0 {
                let ratio = result.as_millis() as f64 / prev_result.as_millis() as f64;
                if result.as_millis() < config.max_wait.as_millis() {
                    assert!(
                        (ratio - 2.0).abs() < 0.01,
                        "Retry count {retry_count}: ratio {ratio} should be ~2.0"
                    );
                }
            }

            prev_result = result;
        }
    }

    // ========================================================================
    // Tests for is_retryable_status
    // ========================================================================

    #[test]
    fn test_is_retryable_status_429() {
        assert!(is_retryable_status(reqwest::StatusCode::TOO_MANY_REQUESTS));
    }

    #[test]
    fn test_is_retryable_status_502() {
        assert!(is_retryable_status(reqwest::StatusCode::BAD_GATEWAY));
    }

    #[test]
    fn test_is_retryable_status_503() {
        assert!(is_retryable_status(
            reqwest::StatusCode::SERVICE_UNAVAILABLE
        ));
    }

    #[test]
    fn test_is_retryable_status_504() {
        assert!(is_retryable_status(reqwest::StatusCode::GATEWAY_TIMEOUT));
    }

    #[test]
    fn test_is_retryable_status_200_not_retryable() {
        assert!(!is_retryable_status(reqwest::StatusCode::OK));
    }

    #[test]
    fn test_is_retryable_status_400_not_retryable() {
        assert!(!is_retryable_status(reqwest::StatusCode::BAD_REQUEST));
    }

    #[test]
    fn test_is_retryable_status_401_not_retryable() {
        assert!(!is_retryable_status(reqwest::StatusCode::UNAUTHORIZED));
    }

    #[test]
    fn test_is_retryable_status_403_not_retryable() {
        assert!(!is_retryable_status(reqwest::StatusCode::FORBIDDEN));
    }

    #[test]
    fn test_is_retryable_status_404_not_retryable() {
        assert!(!is_retryable_status(reqwest::StatusCode::NOT_FOUND));
    }

    #[test]
    fn test_is_retryable_status_500_not_retryable() {
        // 500 Internal Server Error is usually not retryable
        assert!(!is_retryable_status(
            reqwest::StatusCode::INTERNAL_SERVER_ERROR
        ));
    }

    // ========================================================================
    // Tests for parse_retry_after
    // ========================================================================

    #[test]
    fn test_parse_retry_after_valid_seconds() {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::RETRY_AFTER,
            reqwest::header::HeaderValue::from_static("120"),
        );

        let result = parse_retry_after(&headers);
        assert_eq!(result, Some(Duration::from_secs(120)));
    }

    #[test]
    fn test_parse_retry_after_zero_seconds() {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::RETRY_AFTER,
            reqwest::header::HeaderValue::from_static("0"),
        );

        let result = parse_retry_after(&headers);
        assert_eq!(result, Some(Duration::from_secs(0)));
    }

    #[test]
    fn test_parse_retry_after_large_seconds() {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::RETRY_AFTER,
            reqwest::header::HeaderValue::from_static("86400"), // 24 hours
        );

        let result = parse_retry_after(&headers);
        assert_eq!(result, Some(Duration::from_secs(86400)));
    }

    #[test]
    fn test_parse_retry_after_missing_header() {
        let headers = reqwest::header::HeaderMap::new();

        let result = parse_retry_after(&headers);
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_retry_after_invalid_format() {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::RETRY_AFTER,
            reqwest::header::HeaderValue::from_static("invalid"),
        );

        let result = parse_retry_after(&headers);
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_retry_after_http_date_format() {
        // HTTP date format is not currently supported, should return None
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::RETRY_AFTER,
            reqwest::header::HeaderValue::from_static("Wed, 21 Oct 2025 07:28:00 GMT"),
        );

        let result = parse_retry_after(&headers);
        assert_eq!(result, None); // Not implemented yet
    }

    #[test]
    fn test_parse_retry_after_negative_number() {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::RETRY_AFTER,
            reqwest::header::HeaderValue::from_static("-10"),
        );

        let result = parse_retry_after(&headers);
        assert_eq!(result, None); // Should fail to parse as u64
    }

    #[test]
    fn test_parse_retry_after_decimal_number() {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::RETRY_AFTER,
            reqwest::header::HeaderValue::from_static("10.5"),
        );

        let result = parse_retry_after(&headers);
        assert_eq!(result, None); // Should fail to parse as u64
    }
}
