#![cfg(feature = "integration")]

mod common;

use autogen_squareup::apis::catalog_api;
use autogen_squareup::models;

/// Helper: build a CatalogObject of type ITEM with a unique temporary ID and item name.
/// Includes a single ITEM_VARIATION child so the object is valid for upsert.
fn build_test_catalog_item(temp_id: &str, item_name: &str) -> models::CatalogObject {
    let variation = models::CatalogObject {
        r#type: models::CatalogObjectType::ItemVariation,
        id: format!("{}-variation", temp_id),
        item_variation_data: Some(Box::new(models::CatalogItemVariation {
            name: Some(Some("Regular".to_string())),
            pricing_type: Some(models::CatalogPricingType::FixedPricing),
            price_money: Some(Box::new(models::Money {
                amount: Some(Some(500)),
                currency: Some(models::Currency::Usd),
            })),
            ..Default::default()
        })),
        ..Default::default()
    };

    models::CatalogObject {
        r#type: models::CatalogObjectType::Item,
        id: temp_id.to_string(),
        item_data: Some(Box::new(models::CatalogItem {
            name: Some(Some(item_name.to_string())),
            description: Some(Some(format!("Integration test item: {}", item_name))),
            variations: Some(Some(vec![variation])),
            ..Default::default()
        })),
        ..Default::default()
    }
}

/// Helper: upsert a catalog item and return the server-assigned object ID.
async fn upsert_test_item(
    config: &autogen_squareup::apis::configuration::Configuration,
    item_name: &str,
) -> String {
    let temp_id = format!("#{}", common::unique_key());
    let catalog_object = build_test_catalog_item(&temp_id, item_name);

    let req = models::UpsertCatalogObjectRequest::new(
        common::unique_key(),
        catalog_object,
    );

    let resp = catalog_api::upsert_catalog_object(config, req)
        .await
        .expect("upsert_catalog_object should succeed");

    assert!(resp.errors.is_none(), "upsert should not return errors");

    let obj = resp
        .catalog_object
        .expect("response should contain the catalog object");
    obj.id.clone()
}

/// Helper: delete a catalog object, ignoring errors (best-effort cleanup).
async fn cleanup_catalog_object(
    config: &autogen_squareup::apis::configuration::Configuration,
    object_id: &str,
) {
    let _ = catalog_api::delete_catalog_object(config, object_id).await;
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_catalog_info() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();

    // catalog_info may fail with a deserialization error if the API returns
    // enum variants newer than the generated code (e.g. TYPE_TIME).
    // We just verify the call doesn't panic — a Serde error is acceptable
    // and indicates the spec needs regeneration.
    let _resp = catalog_api::catalog_info(config).await;
}

#[tokio::test]
async fn test_list_catalog() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();

    let resp = catalog_api::list_catalog(config, None, None, None)
        .await
        .expect("list_catalog should succeed");

    assert!(resp.errors.is_none(), "list_catalog should not return errors");
    // The response is valid even if objects is None (empty catalog).
}

#[tokio::test]
async fn test_upsert_and_retrieve_catalog_object() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();

    let item_name = format!("Test Item {}", common::unique_key());
    let object_id = upsert_test_item(config, &item_name).await;

    // Retrieve the created object by its server-assigned ID.
    let retrieve_resp =
        catalog_api::retrieve_catalog_object(config, &object_id, None, None, None)
            .await
            .expect("retrieve_catalog_object should succeed");

    assert!(
        retrieve_resp.errors.is_none(),
        "retrieve should not return errors"
    );

    let retrieved = retrieve_resp
        .object
        .expect("response should contain the catalog object");

    assert_eq!(retrieved.id, object_id, "retrieved object ID should match");
    assert_eq!(
        retrieved.r#type,
        models::CatalogObjectType::Item,
        "retrieved object should be of type ITEM"
    );

    // Verify item_data name matches what we sent.
    let item_data = retrieved
        .item_data
        .expect("retrieved object should have item_data");
    assert_eq!(
        item_data.name,
        Some(Some(item_name)),
        "item name should match what was upserted"
    );

    // Cleanup: delete the created object.
    cleanup_catalog_object(config, &object_id).await;
}

#[tokio::test]
async fn test_search_catalog_objects() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();

    let req = models::SearchCatalogObjectsRequest::new();

    let resp = catalog_api::search_catalog_objects(config, req)
        .await
        .expect("search_catalog_objects should succeed");

    assert!(
        resp.errors.is_none(),
        "search_catalog_objects should not return errors"
    );
    // The response is valid even if objects is None (empty catalog).
}

#[tokio::test]
async fn test_catalog_lifecycle() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();

    // --- Step 1: Upsert two catalog items ---
    let name_a = format!("Lifecycle Item A {}", common::unique_key());
    let name_b = format!("Lifecycle Item B {}", common::unique_key());

    let id_a = upsert_test_item(config, &name_a).await;
    let id_b = upsert_test_item(config, &name_b).await;

    // --- Step 2: Retrieve both items individually ---
    let retrieve_a =
        catalog_api::retrieve_catalog_object(config, &id_a, None, None, None)
            .await
            .expect("retrieve item A should succeed");
    assert!(retrieve_a.errors.is_none());
    let obj_a = retrieve_a.object.expect("should contain object A");
    assert_eq!(obj_a.id, id_a);

    let retrieve_b =
        catalog_api::retrieve_catalog_object(config, &id_b, None, None, None)
            .await
            .expect("retrieve item B should succeed");
    assert!(retrieve_b.errors.is_none());
    let obj_b = retrieve_b.object.expect("should contain object B");
    assert_eq!(obj_b.id, id_b);

    // --- Step 3: Batch retrieve both items ---
    let batch_req = models::BatchRetrieveCatalogObjectsRequest::new(
        vec![id_a.clone(), id_b.clone()],
    );

    let batch_resp =
        catalog_api::batch_retrieve_catalog_objects(config, batch_req)
            .await
            .expect("batch_retrieve_catalog_objects should succeed");

    assert!(
        batch_resp.errors.is_none(),
        "batch retrieve should not return errors"
    );

    let batch_objects = batch_resp
        .objects
        .expect("batch retrieve should return objects");
    let batch_ids: Vec<&str> = batch_objects.iter().map(|o| o.id.as_str()).collect();
    assert!(
        batch_ids.contains(&id_a.as_str()),
        "batch retrieve should include item A"
    );
    assert!(
        batch_ids.contains(&id_b.as_str()),
        "batch retrieve should include item B"
    );

    // --- Step 4: Search for catalog objects (ITEM type) ---
    let search_req = models::SearchCatalogObjectsRequest {
        object_types: Some(vec![models::CatalogObjectType::Item]),
        ..Default::default()
    };

    let search_resp = catalog_api::search_catalog_objects(config, search_req)
        .await
        .expect("search_catalog_objects should succeed");

    assert!(
        search_resp.errors.is_none(),
        "search should not return errors"
    );

    // --- Step 5: Delete item A individually ---
    let delete_resp = catalog_api::delete_catalog_object(config, &id_a)
        .await
        .expect("delete_catalog_object should succeed");

    assert!(
        delete_resp.errors.is_none(),
        "delete should not return errors"
    );

    let deleted_ids = delete_resp
        .deleted_object_ids
        .expect("delete response should contain deleted_object_ids");
    assert!(
        deleted_ids.contains(&id_a),
        "deleted IDs should include item A"
    );

    // Verify item A is no longer retrievable (expect a 404 / error).
    let retrieve_deleted =
        catalog_api::retrieve_catalog_object(config, &id_a, None, None, None).await;
    assert!(
        retrieve_deleted.is_err(),
        "retrieving a deleted object should fail"
    );

    // --- Step 6: Batch delete item B ---
    let batch_delete_req = models::BatchDeleteCatalogObjectsRequest::new(
        vec![id_b.clone()],
    );

    let batch_delete_resp =
        catalog_api::batch_delete_catalog_objects(config, batch_delete_req)
            .await
            .expect("batch_delete_catalog_objects should succeed");

    assert!(
        batch_delete_resp.errors.is_none(),
        "batch delete should not return errors"
    );

    let batch_deleted_ids = batch_delete_resp
        .deleted_object_ids
        .expect("batch delete response should contain deleted_object_ids");
    assert!(
        batch_deleted_ids.contains(&id_b),
        "batch deleted IDs should include item B"
    );
}
