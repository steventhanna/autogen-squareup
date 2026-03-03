#![cfg(feature = "integration")]

mod common;

use autogen_squareup::apis::{payments_api, refunds_api};
use autogen_squareup::models;

/// Helper: create a completed payment in the sandbox for the given amount (in cents).
/// Returns the payment ID.
async fn create_completed_payment(
    config: &autogen_squareup::apis::configuration::Configuration,
    location_id: &str,
    amount_cents: i64,
) -> String {
    let mut req = models::CreatePaymentRequest::new(
        "cnon:card-nonce-ok".to_string(),
        common::unique_key(),
    );
    req.amount_money = Some(Box::new(models::Money {
        amount: Some(Some(amount_cents)),
        currency: Some(models::Currency::Usd),
    }));
    req.location_id = Some(location_id.to_string());
    req.autocomplete = Some(true);

    let resp = payments_api::create_payment(config, req)
        .await
        .expect("create_payment should succeed");

    let payment = resp.payment.expect("response should contain a payment");
    payment.id.expect("payment should have an id")
}

#[tokio::test]
async fn test_list_refunds() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();

    let resp = refunds_api::list_payment_refunds(
        config,
        None, // begin_time
        None, // end_time
        None, // sort_order
        None, // cursor
        None, // location_id
        None, // status
        None, // source_type
        None, // limit
        None, // updated_at_begin_time
        None, // updated_at_end_time
        None, // sort_field
    )
    .await;

    assert!(resp.is_ok(), "list_payment_refunds should succeed: {:?}", resp.err());
}

#[tokio::test]
async fn test_refund_payment() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();
    let location_id = common::default_location_id(&client).await;

    // Create a completed payment for 200 cents ($2.00).
    let payment_id = create_completed_payment(config, &location_id, 200).await;

    // Refund 100 cents ($1.00) of it.
    let mut refund_req = models::RefundPaymentRequest::new(
        common::unique_key(),
        models::Money {
            amount: Some(Some(100)),
            currency: Some(models::Currency::Usd),
        },
    );
    refund_req.payment_id = Some(Some(payment_id.clone()));
    refund_req.reason = Some(Some("integration test partial refund".to_string()));

    let resp = refunds_api::refund_payment(config, refund_req)
        .await
        .expect("refund_payment should succeed");

    let refund = resp.refund.expect("response should contain a refund");

    // Verify the refund amount matches what we requested.
    assert_eq!(
        refund.amount_money.amount,
        Some(Some(100)),
        "refund amount should be 100 cents"
    );
    assert_eq!(
        refund.amount_money.currency,
        Some(models::Currency::Usd),
        "refund currency should be USD"
    );

    // Verify the refund is linked to the correct payment.
    assert_eq!(
        refund.payment_id,
        Some(Some(payment_id)),
        "refund should reference the original payment"
    );
}

#[tokio::test]
async fn test_get_refund() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();
    let location_id = common::default_location_id(&client).await;

    // Create a completed payment for 300 cents ($3.00).
    let payment_id = create_completed_payment(config, &location_id, 300).await;

    // Refund the full 300 cents.
    let mut refund_req = models::RefundPaymentRequest::new(
        common::unique_key(),
        models::Money {
            amount: Some(Some(300)),
            currency: Some(models::Currency::Usd),
        },
    );
    refund_req.payment_id = Some(Some(payment_id.clone()));
    refund_req.reason = Some(Some("integration test full refund".to_string()));

    let refund_resp = refunds_api::refund_payment(config, refund_req)
        .await
        .expect("refund_payment should succeed");

    let refund = refund_resp.refund.expect("response should contain a refund");
    let refund_id = refund.id.clone();

    // Retrieve the refund by its ID.
    let get_resp = refunds_api::get_payment_refund(config, &refund_id)
        .await
        .expect("get_payment_refund should succeed");

    let retrieved = get_resp.refund.expect("response should contain a refund");

    // Verify the retrieved refund matches the one we created.
    assert_eq!(
        retrieved.id, refund_id,
        "retrieved refund ID should match"
    );
    assert_eq!(
        retrieved.amount_money.amount,
        Some(Some(300)),
        "retrieved refund amount should be 300 cents"
    );
    assert_eq!(
        retrieved.amount_money.currency,
        Some(models::Currency::Usd),
        "retrieved refund currency should be USD"
    );
    assert_eq!(
        retrieved.payment_id,
        Some(Some(payment_id)),
        "retrieved refund should reference the original payment"
    );
    assert_eq!(
        retrieved.reason,
        Some(Some("integration test full refund".to_string())),
        "retrieved refund reason should match"
    );
}
