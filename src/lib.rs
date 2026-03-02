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
//! autogen-squareup = { version = "0.1", default-features = false, features = ["payments", "native-tls"] }
//! ```

#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

pub mod apis;
pub mod models;
pub mod client;

pub use client::{SquareClient, Environment};
