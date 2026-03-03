use autogen_squareup::{SquareClient, Environment};

#[test]
fn test_production_client_has_correct_base_path() {
    let client = SquareClient::new("test-token");
    assert_eq!(client.config().base_path, "https://connect.squareup.com");
}

#[test]
fn test_sandbox_client_has_correct_base_path() {
    let client = SquareClient::sandbox("test-token");
    assert_eq!(client.config().base_path, "https://connect.squareupsandbox.com");
}

#[test]
fn test_client_has_oauth_access_token() {
    let client = SquareClient::new("my-secret-token");
    assert_eq!(
        client.config().oauth_access_token,
        Some("my-secret-token".to_string())
    );
}

#[test]
fn test_with_env_production() {
    let client = SquareClient::with_env("tok", Environment::Production);
    assert_eq!(client.config().base_path, "https://connect.squareup.com");
}

#[test]
fn test_with_env_sandbox() {
    let client = SquareClient::with_env("tok", Environment::Sandbox);
    assert_eq!(client.config().base_path, "https://connect.squareupsandbox.com");
}
