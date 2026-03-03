#![cfg(feature = "integration")]

mod common;

use autogen_squareup::apis::cards_api;
use autogen_squareup::apis::customers_api;
use autogen_squareup::models::{Address, Card, CreateCardRequest, CreateCustomerRequest};

/// Helper: create a sandbox customer for card association and return the customer ID.
async fn create_test_customer() -> String {
    let client = common::sandbox_client();
    let config = client.config();

    let req = CreateCustomerRequest {
        idempotency_key: Some(common::unique_key()),
        given_name: Some("CardTest".to_string()),
        family_name: Some("Customer".to_string()),
        email_address: Some(format!("cardtest-{}@example.com", common::unique_key())),
        ..Default::default()
    };

    let resp = customers_api::create_customer(config, req)
        .await
        .expect("Failed to create test customer for card tests");

    resp.customer
        .expect("Response missing customer")
        .id
        .expect("Customer missing id")
}

/// Helper: delete a customer, swallowing any errors (used for cleanup).
async fn cleanup_customer(customer_id: &str) {
    let client = common::sandbox_client();
    let _ = customers_api::delete_customer(client.config(), customer_id, None).await;
}

#[tokio::test]
async fn test_list_cards() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();

    let resp = cards_api::list_cards(config, None, None, None, None, None).await;
    assert!(resp.is_ok(), "list_cards should succeed: {:?}", resp.err());
}

#[tokio::test]
async fn test_card_lifecycle() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();

    // --- Setup: create a customer to attach the card to ---
    let customer_id = create_test_customer().await;

    // --- Create card ---
    let billing_address = Address {
        postal_code: Some(Some("94103".to_string())),
        ..Default::default()
    };

    let card = Card {
        customer_id: Some(Some(customer_id.clone())),
        billing_address: Some(Box::new(billing_address)),
        ..Default::default()
    };

    let create_req = CreateCardRequest::new(
        common::unique_key(),
        "cnon:card-nonce-ok".to_string(),
        card,
    );

    let create_resp = cards_api::create_card(config, create_req)
        .await
        .expect("create_card should succeed");

    let created_card = create_resp.card.expect("Response should contain card");
    let card_id = created_card.id.clone().expect("Card should have an id");
    assert!(
        created_card.enabled == Some(true),
        "Newly created card should be enabled"
    );

    // --- Retrieve card ---
    let retrieve_resp = cards_api::retrieve_card(config, &card_id)
        .await
        .expect("retrieve_card should succeed");

    let retrieved_card = retrieve_resp.card.expect("Response should contain card");
    assert_eq!(
        retrieved_card.id.as_deref(),
        Some(card_id.as_str()),
        "Retrieved card ID should match the created card ID"
    );

    // --- List cards (verify our card appears) ---
    let list_resp = cards_api::list_cards(config, None, Some(&customer_id), None, None, None)
        .await
        .expect("list_cards should succeed");

    let cards = list_resp.cards.unwrap_or_default();
    let found = cards.iter().any(|c| c.id.as_deref() == Some(card_id.as_str()));
    assert!(
        found,
        "Created card should appear in list_cards results for the customer"
    );

    // --- Disable card ---
    let disable_resp = cards_api::disable_card(config, &card_id)
        .await
        .expect("disable_card should succeed");

    let disabled_card = disable_resp.card.expect("Response should contain card");
    assert_eq!(
        disabled_card.enabled,
        Some(false),
        "Disabled card should have enabled=false"
    );

    // --- Cleanup: delete the test customer ---
    cleanup_customer(&customer_id).await;
}
