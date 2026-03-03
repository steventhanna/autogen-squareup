#![cfg(feature = "integration")]

mod common;

use autogen_squareup::apis::customer_groups_api;
use autogen_squareup::models::{
    CreateCustomerGroupRequest, CustomerGroup, UpdateCustomerGroupRequest,
};

#[tokio::test]
async fn test_list_customer_groups() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();

    let resp = customer_groups_api::list_customer_groups(config, None, None).await;
    assert!(resp.is_ok(), "list_customer_groups should succeed: {:?}", resp.err());
}

#[tokio::test]
async fn test_customer_group_lifecycle() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();

    let group_name = format!("TestGroup-{}", common::unique_key());

    // --- Create ---
    let create_req = CreateCustomerGroupRequest {
        idempotency_key: Some(common::unique_key()),
        group: Box::new(CustomerGroup::new(group_name.clone())),
    };

    let create_resp = customer_groups_api::create_customer_group(config, create_req)
        .await
        .expect("create_customer_group should succeed");

    let group = create_resp.group.expect("Response should contain group");
    let group_id = group.id.clone().expect("Group should have an id");
    assert_eq!(group.name, group_name);

    // --- Retrieve ---
    let retrieve_resp = customer_groups_api::retrieve_customer_group(config, &group_id)
        .await
        .expect("retrieve_customer_group should succeed");

    let retrieved = retrieve_resp.group.expect("Response should contain group");
    assert_eq!(retrieved.id.as_deref(), Some(group_id.as_str()));
    assert_eq!(retrieved.name, group_name);

    // --- Update ---
    let updated_name = format!("UpdatedGroup-{}", common::unique_key());
    let update_req = UpdateCustomerGroupRequest::new(CustomerGroup::new(updated_name.clone()));

    let update_resp =
        customer_groups_api::update_customer_group(config, &group_id, update_req)
            .await
            .expect("update_customer_group should succeed");

    let updated = update_resp.group.expect("Response should contain group");
    assert_eq!(updated.name, updated_name);

    // --- List (verify group appears) ---
    let list_resp = customer_groups_api::list_customer_groups(config, None, Some(50))
        .await
        .expect("list_customer_groups should succeed");

    let groups = list_resp.groups.unwrap_or_default();
    let found = groups.iter().any(|g| g.id.as_deref() == Some(group_id.as_str()));
    assert!(
        found,
        "Created customer group should appear in list_customer_groups results"
    );

    // --- Delete ---
    let delete_resp = customer_groups_api::delete_customer_group(config, &group_id).await;
    assert!(
        delete_resp.is_ok(),
        "delete_customer_group should succeed: {:?}",
        delete_resp.err()
    );
}
