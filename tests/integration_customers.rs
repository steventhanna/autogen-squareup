#![cfg(feature = "integration")]

mod common;

use autogen_squareup::apis::customers_api;
use autogen_squareup::models::{CreateCustomerRequest, SearchCustomersRequest, UpdateCustomerRequest};

/// Helper: create a customer with the given name fields and return (customer_id, version).
async fn create_test_customer(
    given: &str,
    family: &str,
    email: &str,
) -> (String, Option<i64>) {
    let client = common::sandbox_client();
    let config = client.config();

    let req = CreateCustomerRequest {
        idempotency_key: Some(common::unique_key()),
        given_name: Some(given.to_string()),
        family_name: Some(family.to_string()),
        email_address: Some(email.to_string()),
        ..Default::default()
    };

    let resp = customers_api::create_customer(config, req)
        .await
        .expect("Failed to create test customer");

    let customer = resp.customer.expect("Response missing customer");
    let id = customer.id.clone().expect("Customer missing id");
    let version = customer.version;
    (id, version)
}


#[tokio::test]
async fn test_create_customer() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();

    let req = CreateCustomerRequest {
        idempotency_key: Some(common::unique_key()),
        given_name: Some("TestCreate".to_string()),
        family_name: Some("Customer".to_string()),
        email_address: Some(format!("create-{}@example.com", common::unique_key())),
        ..Default::default()
    };

    let resp = customers_api::create_customer(config, req).await;
    assert!(resp.is_ok(), "create_customer should succeed");

    let resp = resp.unwrap();
    let customer = resp.customer.expect("Response should contain customer");
    assert_eq!(customer.given_name, Some(Some("TestCreate".to_string())));

    // Cleanup
    if let Some(id) = &customer.id {
        let _ = customers_api::delete_customer(config, id, None).await;
    }
}

#[tokio::test]
async fn test_retrieve_customer() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();

    let (customer_id, _version) = create_test_customer(
        "TestRetrieve",
        "Customer",
        &format!("retrieve-{}@example.com", common::unique_key()),
    )
    .await;

    let resp = customers_api::retrieve_customer(config, &customer_id).await;
    assert!(resp.is_ok(), "retrieve_customer should succeed");

    let resp = resp.unwrap();
    let customer = resp.customer.expect("Response should contain customer");
    assert_eq!(customer.id.as_deref(), Some(customer_id.as_str()));
    assert_eq!(customer.given_name, Some(Some("TestRetrieve".to_string())));
    assert_eq!(customer.family_name, Some(Some("Customer".to_string())));

    // Cleanup
    let _ = customers_api::delete_customer(config, &customer_id, None).await;
}

#[tokio::test]
async fn test_list_customers() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();

    let resp = customers_api::list_customers(config, None, Some(1), None, None, None).await;
    assert!(resp.is_ok(), "list_customers should succeed");
}

#[tokio::test]
async fn test_search_customers() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();

    let req = SearchCustomersRequest {
        limit: Some(1),
        ..Default::default()
    };

    let resp = customers_api::search_customers(config, req).await;
    assert!(resp.is_ok(), "search_customers should succeed");
}

#[tokio::test]
async fn test_update_customer() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();

    let (customer_id, version) = create_test_customer(
        "TestUpdate",
        "BeforeUpdate",
        &format!("update-{}@example.com", common::unique_key()),
    )
    .await;

    let update_req = UpdateCustomerRequest {
        family_name: Some(Some("AfterUpdate".to_string())),
        version,
        ..Default::default()
    };

    let resp = customers_api::update_customer(config, &customer_id, update_req).await;
    assert!(resp.is_ok(), "update_customer should succeed");

    let resp = resp.unwrap();
    let customer = resp.customer.expect("Response should contain customer");
    assert_eq!(customer.family_name, Some(Some("AfterUpdate".to_string())));

    // Cleanup
    let _ = customers_api::delete_customer(config, &customer_id, None).await;
}

#[tokio::test]
async fn test_delete_customer() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();

    let (customer_id, _version) = create_test_customer(
        "TestDelete",
        "Customer",
        &format!("delete-{}@example.com", common::unique_key()),
    )
    .await;

    let resp = customers_api::delete_customer(config, &customer_id, None).await;
    assert!(resp.is_ok(), "delete_customer should succeed");
}

#[tokio::test]
async fn test_customer_lifecycle() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();

    // --- Create ---
    let create_req = CreateCustomerRequest {
        idempotency_key: Some(common::unique_key()),
        given_name: Some("Lifecycle".to_string()),
        family_name: Some("Original".to_string()),
        email_address: Some(format!("lifecycle-{}@example.com", common::unique_key())),
        ..Default::default()
    };

    let create_resp = customers_api::create_customer(config, create_req)
        .await
        .expect("create_customer should succeed");
    let customer = create_resp.customer.expect("Response should contain customer");
    let customer_id = customer.id.clone().expect("Customer should have an id");
    let version = customer.version;
    assert_eq!(customer.given_name, Some(Some("Lifecycle".to_string())));

    // --- Retrieve ---
    let retrieve_resp = customers_api::retrieve_customer(config, &customer_id)
        .await
        .expect("retrieve_customer should succeed");
    let retrieved = retrieve_resp.customer.expect("Response should contain customer");
    assert_eq!(retrieved.id.as_deref(), Some(customer_id.as_str()));
    assert_eq!(retrieved.family_name, Some(Some("Original".to_string())));

    // --- Update ---
    let update_req = UpdateCustomerRequest {
        family_name: Some(Some("Updated".to_string())),
        version,
        ..Default::default()
    };

    let update_resp = customers_api::update_customer(config, &customer_id, update_req)
        .await
        .expect("update_customer should succeed");
    let updated = update_resp.customer.expect("Response should contain customer");
    assert_eq!(updated.family_name, Some(Some("Updated".to_string())));

    // --- List (verify list_customers works; eventual consistency means the
    //     newly created customer may not appear immediately) ---
    let list_resp = customers_api::list_customers(config, None, Some(10), None, None, None)
        .await
        .expect("list_customers should succeed");
    assert!(list_resp.customers.is_some() || list_resp.errors.is_none(),
        "list_customers should return a valid response");

    // --- Delete ---
    let delete_resp = customers_api::delete_customer(config, &customer_id, None).await;
    assert!(delete_resp.is_ok(), "delete_customer should succeed");
}
