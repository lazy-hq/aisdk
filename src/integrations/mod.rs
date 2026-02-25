//! Provides extra integrations for seamless use with common libraries and frameworks.

#[cfg(feature = "axum")]
pub mod axum;
#[cfg(feature = "dioxus")]
pub mod dioxus;
pub mod vercel_aisdk_ui;
