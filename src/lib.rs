//! # autogen-squareup
//!
//! Auto-generated, strongly-typed Rust client for the [Square API](https://developer.squareup.com/reference/square).
//!
//! ## Quick Start
//!
//! ```no_run
//! use autogen_squareup::SquareClient;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = SquareClient::new("your-access-token");
//!     // Use client.config() with generated API functions
//! }
//! ```
//!
//! ## Feature Flags
//!
//! By default all API groups are enabled. To reduce compile time, select only what you need:
//!
//! ```toml
//! [dependencies]
//! autogen-squareup = { version = "0.20260715", default-features = false, features = ["payments", "native-tls"] }
//! ```
//!
//! ## Middleware
//!
//! Every generated `Configuration.client` is a `reqwest_middleware::ClientWithMiddleware`.
//! Use [`SquareClient::builder`] to attach middleware, for example a
//! `reqwest_tracing::TracingMiddleware` installed by the application:
//!
//! ```no_run
//! use autogen_squareup::{SquareClient, Environment};
//!
//! # fn example(my_middleware: impl autogen_squareup::reqwest_middleware::Middleware) {
//! let client = SquareClient::builder("your-access-token")
//!     .environment(Environment::Sandbox)
//!     .with(my_middleware)
//!     .build();
//! # }
//! ```
//!
//! This crate emits no spans and has no opentelemetry dependency; it only routes requests
//! through whatever middleware the application attaches.

#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

pub mod apis;
pub mod models;
pub mod client;

pub use client::{SquareClient, SquareClientBuilder, Environment};

/// Re-exported so callers build middleware against the same `reqwest_middleware`
/// version this crate links.
pub use reqwest_middleware;
