use std::sync::Arc;

use crate::apis::configuration::Configuration;

/// Derive the Square API version from the crate version.
/// Crate version format: 0.YYYYMMDD.0 → Square API version: YYYY-MM-DD
fn square_api_version() -> String {
    let version = env!("CARGO_PKG_VERSION"); // e.g. "0.20251016.0"
    let date_part = version
        .split('.')
        .nth(1)
        .expect("crate version should have format 0.YYYYMMDD.0");
    format!(
        "{}-{}-{}",
        &date_part[0..4],
        &date_part[4..6],
        &date_part[6..8]
    )
}

const PRODUCTION_URL: &str = "https://connect.squareup.com";
const SANDBOX_URL: &str = "https://connect.squareupsandbox.com";

#[derive(Debug, Clone)]
pub enum Environment {
    Production,
    Sandbox,
}

impl Environment {
    fn base_url(&self) -> &'static str {
        match self {
            Environment::Production => PRODUCTION_URL,
            Environment::Sandbox => SANDBOX_URL,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SquareClient {
    configuration: Configuration,
}

impl SquareClient {
    /// Create a client for the production Square API.
    pub fn new(access_token: &str) -> Self {
        Self::with_env(access_token, Environment::Production)
    }

    /// Create a client for the Square sandbox API.
    pub fn sandbox(access_token: &str) -> Self {
        Self::with_env(access_token, Environment::Sandbox)
    }

    /// Create a client with a specific environment.
    pub fn with_env(access_token: &str, env: Environment) -> Self {
        Self::builder(access_token).environment(env).build()
    }

    /// Start building a client with a custom middleware chain.
    pub fn builder(access_token: impl Into<String>) -> SquareClientBuilder {
        SquareClientBuilder {
            access_token: access_token.into(),
            environment: Environment::Production,
            middleware: Vec::new(),
        }
    }

    /// Access the underlying configuration for use with generated API functions.
    pub fn config(&self) -> &Configuration {
        &self.configuration
    }
}

/// Builder for [`SquareClient`] that allows attaching `reqwest_middleware` middleware
/// (for example a tracing middleware installed by the application).
pub struct SquareClientBuilder {
    access_token: String,
    environment: Environment,
    middleware: Vec<Arc<dyn reqwest_middleware::Middleware>>,
}

impl std::fmt::Debug for SquareClientBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SquareClientBuilder")
            .field("environment", &self.environment)
            .field("middleware_count", &self.middleware.len())
            .finish()
    }
}

impl SquareClientBuilder {
    /// Set the target environment. Default is [`Environment::Production`].
    pub fn environment(mut self, env: Environment) -> Self {
        self.environment = env;
        self
    }

    /// Append a middleware to the chain (order preserved).
    pub fn with<M: reqwest_middleware::Middleware>(mut self, middleware: M) -> Self {
        self.middleware.push(Arc::new(middleware));
        self
    }

    /// Append a middleware to the chain via an existing `Arc` (order preserved).
    pub fn with_arc(mut self, middleware: Arc<dyn reqwest_middleware::Middleware>) -> Self {
        self.middleware.push(middleware);
        self
    }

    /// Build the [`SquareClient`], applying all attached middleware.
    pub fn build(self) -> SquareClient {
        let api_version = square_api_version();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Square-Version",
            reqwest::header::HeaderValue::from_str(&api_version)
                .expect("valid Square-Version header"),
        );

        let http_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("failed to build reqwest client");

        let mut middleware_builder = reqwest_middleware::ClientBuilder::new(http_client);
        for middleware in self.middleware {
            middleware_builder = middleware_builder.with_arc(middleware);
        }

        let mut configuration = Configuration::new();
        configuration.base_path = self.environment.base_url().to_string();
        configuration.oauth_access_token = Some(self.access_token);
        configuration.user_agent = Some(format!("autogen-squareup/{}", env!("CARGO_PKG_VERSION")));
        configuration.client = middleware_builder.build();
        SquareClient { configuration }
    }
}
