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
        // Derive the Square API version from the crate version (0.YYYYMMDD.0 → YYYY-MM-DD).
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

        let mut configuration = Configuration::new();
        configuration.base_path = env.base_url().to_string();
        configuration.oauth_access_token = Some(access_token.to_string());
        configuration.user_agent = Some(format!("autogen-squareup/{}", env!("CARGO_PKG_VERSION")));
        configuration.client = http_client;
        Self { configuration }
    }

    /// Access the underlying configuration for use with generated API functions.
    pub fn config(&self) -> &Configuration {
        &self.configuration
    }
}
