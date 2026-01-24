//! Helper functions for `aisdk`

use crate::{
    Error,
    core::{Message, language_model::LanguageModelOptions, messages::TaggedMessage},
};

/// Creates a hook that returns `true` if the number of conversation steps exceeds the given count.
///
/// This function is useful for stopping generation after a certain number of tool-calling
/// iterations or conversation turns. The returned closure can be used with the `stop_when`
/// option in `LanguageModelOptions`.
///
/// # Parameters
///
/// * `step` - The step count threshold. Returns `true` when the actual step count is greater than this value.
///
/// # Returns
///
/// A closure that takes `&LanguageModelOptions` and returns `bool`.
///
/// # Examples
///
/// ```rust,no_run
/// use aisdk::core::{LanguageModelRequest, utils::step_count_is};
/// // use aisdk::providers::OpenAI; // Requires "openai" feature
///
/// // #[tokio::main]
/// // async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// //     let result = LanguageModelRequest::builder()
/// //         .model(OpenAI::gpt_5())
/// //         .system("You are a helpful assistant with access to tools.")
/// //         .prompt("What is the weather in New York?")
/// //         .stop_when(step_count_is(3)) // Limit agent loop to 3 steps
/// //         .build()
/// //         .generate_text()
/// //         .await?;
/// //
/// //     println!("{}", result.text().unwrap());
/// //     Ok(())
/// // }
/// ```
pub fn step_count_is(step: usize) -> impl Fn(&LanguageModelOptions) -> bool {
    move |options| options.steps().len() > step
}

/// Resolves the message to be used for text generation.
///
/// This function takes a prompt and a list of messages and returns a vector of
/// messages that can be used for LanguageModelCallOptions.
/// if no messages are provided, a default message is created with the prompt and system prompt.
pub(crate) fn resolve_message(
    options: &LanguageModelOptions,
    prompt: &Option<String>,
) -> (String, Vec<TaggedMessage>) {
    let messages = if options.messages.is_empty() {
        let mut msgs = Vec::new();

        // Only add system message if system prompt is provided and not empty
        if let Some(ref system) = options.system
            && !system.is_empty()
        {
            msgs.push(TaggedMessage::initial_step_msg(Message::System(
                system.clone().into(),
            )));
        }

        // Add user message
        msgs.push(TaggedMessage::initial_step_msg(Message::User(
            prompt.to_owned().unwrap_or_default().into(),
        )));

        msgs
    } else {
        options.messages.to_vec()
    };

    let system = options.system.to_owned().unwrap_or_else(|| {
        messages
            .iter()
            .find_map(|m| match m.message {
                Message::System(ref s) => Some(s.content.to_string()),
                _ => None,
            })
            .unwrap_or_default()
    });

    (system, messages)
}

/// Sums two options, returning the sum if both are Some, otherwise returns the first option.
pub(crate) fn sum_options(a: Option<usize>, b: Option<usize>) -> Option<usize> {
    match (a, b) {
        (Some(x), Some(y)) => Some(x + y),
        _ => a.or(b),
    }
}

#[allow(dead_code)]
/// Validates the base URL.
pub(crate) fn validate_base_url(s: &str) -> crate::error::Result<String> {
    use reqwest::Url;

    let url = s
        .parse::<Url>()
        .map_err(|_| Error::InvalidInput("Invalid base URL".into()))?;

    if !matches!(url.scheme(), "http" | "https") {
        return Err(Error::InvalidInput(
            "Base URL must start with http:// or https://".into(),
        ));
    }

    if url.host_str().is_none() {
        return Err(Error::InvalidInput("Base URL must include a host".into()));
    }

    Ok(url.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_options_both_some() {
        assert_eq!(sum_options(Some(1), Some(2)), Some(3));
        assert_eq!(sum_options(Some(0), Some(0)), Some(0));
        assert_eq!(sum_options(Some(10), Some(20)), Some(30));
    }

    #[test]
    fn test_sum_options_first_some_second_none() {
        assert_eq!(sum_options(Some(5), None), Some(5));
        assert_eq!(sum_options(Some(0), None), Some(0));
    }

    #[test]
    fn test_sum_options_first_none_second_some() {
        assert_eq!(sum_options(None, Some(7)), Some(7));
        assert_eq!(sum_options(None, Some(0)), Some(0));
    }

    #[test]
    fn test_sum_options_both_none() {
        assert_eq!(sum_options(None, None), None);
    }
}
