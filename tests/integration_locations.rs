#![cfg(feature = "integration")]

mod common;

use autogen_squareup::apis::locations_api;
use autogen_squareup::models;

#[tokio::test]
async fn test_list_locations() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();

    let resp = locations_api::list_locations(config)
        .await
        .expect("list_locations should succeed");

    let locations = resp.locations.expect("response should contain locations");
    assert!(
        !locations.is_empty(),
        "sandbox account should have at least one location"
    );
}

#[tokio::test]
async fn test_retrieve_location() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();

    // First, list locations to get an existing location ID.
    let list_resp = locations_api::list_locations(config)
        .await
        .expect("list_locations should succeed");

    let locations = list_resp.locations.expect("response should contain locations");
    let first = locations.first().expect("should have at least one location");
    let location_id = first.id.as_ref().expect("location should have an id");

    // Retrieve that specific location by ID.
    let retrieve_resp = locations_api::retrieve_location(config, location_id)
        .await
        .expect("retrieve_location should succeed");

    let retrieved = retrieve_resp
        .location
        .expect("response should contain a location");

    assert_eq!(
        retrieved.id.as_deref(),
        Some(location_id.as_str()),
        "retrieved location ID should match the requested ID"
    );
}

#[tokio::test]
async fn test_create_and_update_location() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();

    // Create a new location with a unique name.
    let create_name = format!("Test Location {}", common::unique_key());
    let create_req = models::CreateLocationRequest {
        location: Some(Box::new(models::Location {
            name: Some(Some(create_name.clone())),
            ..Default::default()
        })),
    };

    let create_resp = locations_api::create_location(config, create_req)
        .await
        .expect("create_location should succeed");

    let created = create_resp
        .location
        .expect("response should contain the created location");

    let created_id = created.id.as_ref().expect("created location should have an id");

    assert_eq!(
        created.name,
        Some(Some(create_name.clone())),
        "created location name should match the requested name"
    );

    // Update the location with a new unique name.
    let updated_name = format!("Updated Location {}", common::unique_key());
    let update_req = models::UpdateLocationRequest {
        location: Some(Box::new(models::Location {
            name: Some(Some(updated_name.clone())),
            ..Default::default()
        })),
    };

    let update_resp = locations_api::update_location(config, created_id, update_req)
        .await
        .expect("update_location should succeed");

    let updated = update_resp
        .location
        .expect("response should contain the updated location");

    assert_eq!(
        updated.id.as_deref(),
        Some(created_id.as_str()),
        "updated location should have the same ID"
    );
    assert_eq!(
        updated.name,
        Some(Some(updated_name)),
        "updated location name should match the new name"
    );
}
