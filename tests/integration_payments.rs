#![cfg(feature = "integration")]

mod common;

use autogen_squareup::apis::payments_api;
use autogen_squareup::models;

/// Helper to build a CreatePaymentRequest with the given source nonce, amount in cents,
/// and location ID. The `autocomplete` flag controls whether the payment is captured immediately.
fn payment_request(
    source_id: &str,
    amount_cents: i64,
    location_id: &str,
    autocomplete: Option<bool>,
) -> models::CreatePaymentRequest {
    let mut req = models::CreatePaymentRequest::new(
        source_id.to_string(),
        common::unique_key(),
    );
    req.amount_money = Some(Box::new(models::Money {
        amount: Some(Some(amount_cents)),
        currency: Some(models::Currency::Usd),
    }));
    req.location_id = Some(location_id.to_string());
    req.autocomplete = autocomplete;
    req
}

#[tokio::test]
async fn test_create_payment() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();
    let location_id = common::default_location_id(&client).await;

    let req = payment_request("cnon:card-nonce-ok", 100, &location_id, None);

    let resp = payments_api::create_payment(config, req)
        .await
        .expect("create_payment should succeed");

    let payment = resp.payment.expect("response should contain a payment");
    assert!(payment.id.is_some(), "payment should have an id");
}

#[tokio::test]
async fn test_get_payment() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();
    let location_id = common::default_location_id(&client).await;

    // Create a payment first.
    let req = payment_request("cnon:card-nonce-ok", 200, &location_id, None);
    let create_resp = payments_api::create_payment(config, req)
        .await
        .expect("create_payment should succeed");

    let created = create_resp.payment.expect("response should contain a payment");
    let payment_id = created.id.as_ref().expect("payment should have an id");

    // Retrieve the payment by ID.
    let get_resp = payments_api::get_payment(config, payment_id)
        .await
        .expect("get_payment should succeed");

    let fetched = get_resp.payment.expect("response should contain a payment");
    assert_eq!(
        fetched.id.as_deref(),
        Some(payment_id.as_str()),
        "fetched payment ID should match the created payment ID"
    );

    // Verify the amount matches what we requested.
    let amount_money = fetched.amount_money.expect("payment should have amount_money");
    assert_eq!(
        amount_money.amount,
        Some(Some(200)),
        "payment amount should be 200 cents"
    );
    assert_eq!(
        amount_money.currency,
        Some(models::Currency::Usd),
        "payment currency should be USD"
    );
}

#[tokio::test]
async fn test_list_payments() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();

    let resp = payments_api::list_payments(
        config,
        None,  // begin_time
        None,  // end_time
        None,  // sort_order
        None,  // cursor
        None,  // location_id
        None,  // total
        None,  // last_4
        None,  // card_brand
        Some(5), // limit
        None,  // is_offline_payment
        None,  // offline_begin_time
        None,  // offline_end_time
        None,  // updated_at_begin_time
        None,  // updated_at_end_time
        None,  // sort_field
    )
    .await
    .expect("list_payments should succeed");

    // The response itself is valid; payments may or may not be present depending on
    // sandbox state, so we just confirm the call did not error.
    assert!(
        resp.errors.is_none(),
        "list_payments should not return errors"
    );
}

#[tokio::test]
async fn test_create_and_cancel_payment() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();
    let location_id = common::default_location_id(&client).await;

    // Create a payment with autocomplete disabled so it stays in APPROVED state.
    let req = payment_request("cnon:card-nonce-ok", 300, &location_id, Some(false));
    let create_resp = payments_api::create_payment(config, req)
        .await
        .expect("create_payment should succeed");

    let created = create_resp.payment.expect("response should contain a payment");
    let payment_id = created.id.as_ref().expect("payment should have an id");

    // Cancel (void) the approved payment.
    let cancel_resp = payments_api::cancel_payment(config, payment_id)
        .await
        .expect("cancel_payment should succeed");

    assert!(
        cancel_resp.errors.is_none(),
        "cancel_payment should not return errors"
    );
}

#[tokio::test]
async fn test_create_and_complete_payment() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();
    let location_id = common::default_location_id(&client).await;

    // Create a payment with autocomplete disabled so we can manually complete it.
    let req = payment_request("cnon:card-nonce-ok", 400, &location_id, Some(false));
    let create_resp = payments_api::create_payment(config, req)
        .await
        .expect("create_payment should succeed");

    let created = create_resp.payment.expect("response should contain a payment");
    let payment_id = created.id.as_ref().expect("payment should have an id");

    // Complete (capture) the approved payment.
    let complete_req = models::CompletePaymentRequest::default();
    let complete_resp = payments_api::complete_payment(config, payment_id, complete_req)
        .await
        .expect("complete_payment should succeed");

    assert!(
        complete_resp.errors.is_none(),
        "complete_payment should not return errors"
    );
}

#[tokio::test]
async fn test_payment_declined() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();
    let location_id = common::default_location_id(&client).await;

    let req = payment_request("cnon:card-nonce-declined", 500, &location_id, None);

    let result = payments_api::create_payment(config, req).await;

    assert!(
        result.is_err(),
        "create_payment with a declined nonce should return an error"
    );
}
