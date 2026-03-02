use crate::apis::configuration::Configuration;

const PRODUCTION_URL: &str = "https://connect.squareup.com/v2";
const SANDBOX_URL: &str = "https://connect.squareupsandbox.com/v2";

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
        let mut configuration = Configuration::new();
        configuration.base_path = env.base_url().to_string();
        configuration.oauth_access_token = Some(access_token.to_string());
        configuration.user_agent = Some(format!("autogen-squareup/{}", env!("CARGO_PKG_VERSION")));
        Self { configuration }
    }

    /// Access the underlying configuration for use with generated API functions.
    pub fn config(&self) -> &Configuration {
        &self.configuration
    }
}
