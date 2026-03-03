#![allow(dead_code)]

use autogen_squareup::apis::locations_api;
use autogen_squareup::SquareClient;

/// Try to create a sandbox client. Returns `None` if SQUARE_SANDBOX_TOKEN is not set,
/// allowing tests to skip gracefully in environments without credentials.
pub fn try_sandbox_client() -> Option<SquareClient> {
    std::env::var("SQUARE_SANDBOX_TOKEN")
        .ok()
        .map(|token| SquareClient::sandbox(&token))
}

/// Create a sandbox client, panicking if SQUARE_SANDBOX_TOKEN is not set.
pub fn sandbox_client() -> SquareClient {
    try_sandbox_client().expect("SQUARE_SANDBOX_TOKEN must be set to run integration tests")
}

pub async fn default_location_id(client: &SquareClient) -> String {
    let resp = locations_api::list_locations(client.config())
        .await
        .expect("Failed to list locations");
    resp.locations
        .expect("No locations in response")
        .into_iter()
        .next()
        .expect("Sandbox account has no locations")
        .id
        .expect("Location has no id")
}

pub fn unique_key() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Macro to skip a test if SQUARE_SANDBOX_TOKEN is not set.
/// Returns early from the test function without failing.
#[macro_export]
macro_rules! skip_if_no_token {
    () => {
        if std::env::var("SQUARE_SANDBOX_TOKEN").is_err() {
            eprintln!("Skipping test: SQUARE_SANDBOX_TOKEN not set");
            return;
        }
    };
}
