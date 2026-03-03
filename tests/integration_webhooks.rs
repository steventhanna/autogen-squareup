#![cfg(feature = "integration")]

mod common;

use autogen_squareup::apis::webhook_subscriptions_api;
use autogen_squareup::models;

#[tokio::test]
async fn test_list_webhook_event_types() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();

    let resp = webhook_subscriptions_api::list_webhook_event_types(config, None)
        .await
        .expect("list_webhook_event_types should succeed");

    let event_types = resp
        .event_types
        .expect("response should contain event_types");
    assert!(
        !event_types.is_empty(),
        "event_types should not be empty"
    );
}

#[tokio::test]
async fn test_list_webhook_subscriptions() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();

    let resp = webhook_subscriptions_api::list_webhook_subscriptions(
        config, None, None, None, None,
    )
    .await;
    assert!(resp.is_ok(), "list_webhook_subscriptions should succeed");
}

#[tokio::test]
async fn test_webhook_subscription_lifecycle() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();

    // --- Cleanup stale test subscriptions (sandbox has a limit) ---
    if let Ok(list_resp) = webhook_subscriptions_api::list_webhook_subscriptions(
        config, None, None, None, None,
    ).await {
        if let Some(subs) = list_resp.subscriptions {
            for sub in &subs {
                if let Some(name) = sub.name.as_ref().and_then(|n| n.as_ref()) {
                    if name.starts_with("test-webhook-") || name.starts_with("updated-webhook-") {
                        if let Some(id) = &sub.id {
                            let _ = webhook_subscriptions_api::delete_webhook_subscription(config, id).await;
                        }
                    }
                }
            }
        }
    }

    // --- Create ---
    let sub_name = format!("test-webhook-{}", common::unique_key());
    let subscription = models::WebhookSubscription {
        name: Some(Some(sub_name.clone())),
        notification_url: Some(Some("https://example.com/webhook".to_string())),
        event_types: Some(Some(vec!["payment.created".to_string()])),
        ..Default::default()
    };

    let create_req = models::CreateWebhookSubscriptionRequest {
        idempotency_key: Some(common::unique_key()),
        subscription: Box::new(subscription),
    };

    let create_resp = webhook_subscriptions_api::create_webhook_subscription(config, create_req)
        .await
        .expect("create_webhook_subscription should succeed");

    let created = create_resp
        .subscription
        .expect("response should contain the created subscription");

    let subscription_id = created.id.as_ref().expect("subscription should have an id").clone();

    assert_eq!(
        created.name,
        Some(Some(sub_name.clone())),
        "created subscription name should match"
    );
    assert_eq!(
        created.notification_url,
        Some(Some("https://example.com/webhook".to_string())),
        "created subscription notification_url should match"
    );

    // --- Retrieve ---
    let retrieve_resp =
        webhook_subscriptions_api::retrieve_webhook_subscription(config, &subscription_id)
            .await
            .expect("retrieve_webhook_subscription should succeed");

    let retrieved = retrieve_resp
        .subscription
        .expect("response should contain the subscription");

    assert_eq!(
        retrieved.id.as_deref(),
        Some(subscription_id.as_str()),
        "retrieved subscription ID should match"
    );
    assert_eq!(
        retrieved.name,
        Some(Some(sub_name)),
        "retrieved subscription name should match"
    );

    // --- Update ---
    let updated_name = format!("updated-webhook-{}", common::unique_key());
    let update_sub = models::WebhookSubscription {
        name: Some(Some(updated_name.clone())),
        ..Default::default()
    };

    let update_req = models::UpdateWebhookSubscriptionRequest {
        subscription: Some(Box::new(update_sub)),
    };

    let update_resp = webhook_subscriptions_api::update_webhook_subscription(
        config,
        &subscription_id,
        update_req,
    )
    .await
    .expect("update_webhook_subscription should succeed");

    let updated = update_resp
        .subscription
        .expect("response should contain the updated subscription");

    assert_eq!(
        updated.id.as_deref(),
        Some(subscription_id.as_str()),
        "updated subscription should have the same ID"
    );
    assert_eq!(
        updated.name,
        Some(Some(updated_name)),
        "updated subscription name should match the new name"
    );

    // --- Test ---
    let test_req = models::TestWebhookSubscriptionRequest {
        event_type: Some(Some("payment.created".to_string())),
    };

    // The test endpoint sends a test event to the notification URL.
    // With a fake URL like example.com it may not return a test result,
    // so we just verify the API call itself succeeds.
    let _test_resp = webhook_subscriptions_api::test_webhook_subscription(
        config,
        &subscription_id,
        test_req,
    )
    .await
    .expect("test_webhook_subscription should succeed");

    // --- Delete ---
    let delete_resp =
        webhook_subscriptions_api::delete_webhook_subscription(config, &subscription_id)
            .await
            .expect("delete_webhook_subscription should succeed");

    assert!(
        delete_resp.errors.is_none(),
        "delete response should have no errors"
    );

    // Verify the subscription is gone by attempting to retrieve it.
    let after_delete =
        webhook_subscriptions_api::retrieve_webhook_subscription(config, &subscription_id).await;
    assert!(
        after_delete.is_err(),
        "retrieving a deleted subscription should fail"
    );
}
