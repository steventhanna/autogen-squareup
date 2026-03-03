#![cfg(feature = "integration")]

mod common;

use autogen_squareup::apis::orders_api;
use autogen_squareup::models;

/// Helper: build a simple order with one ad-hoc line item at the given location.
fn sample_order(location_id: &str) -> models::Order {
    let mut line_item = models::OrderLineItem::new("1".to_string());
    line_item.name = Some(Some("Integration Test Widget".to_string()));
    line_item.base_price_money = Some(Box::new(models::Money {
        amount: Some(Some(500)),
        currency: Some(models::Currency::Usd),
    }));

    let mut order = models::Order::new(location_id.to_string());
    order.line_items = Some(Some(vec![line_item]));
    order
}

#[tokio::test]
async fn test_create_order() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();
    let location_id = common::default_location_id(&client).await;

    let request = models::CreateOrderRequest {
        order: Some(Box::new(sample_order(&location_id))),
        idempotency_key: Some(common::unique_key()),
    };

    let resp = orders_api::create_order(config, request)
        .await
        .expect("create_order should succeed");

    let order = resp.order.expect("response should contain an order");

    assert_eq!(
        order.location_id, location_id,
        "order location_id should match the requested location"
    );

    let line_items = order
        .line_items
        .expect("order should have line_items outer option")
        .expect("order should have line_items inner option");

    assert_eq!(line_items.len(), 1, "order should have exactly one line item");
    assert_eq!(
        line_items[0].name,
        Some(Some("Integration Test Widget".to_string())),
        "line item name should match"
    );
}

#[tokio::test]
async fn test_retrieve_order() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();
    let location_id = common::default_location_id(&client).await;

    // First, create an order so we have something to retrieve.
    let create_req = models::CreateOrderRequest {
        order: Some(Box::new(sample_order(&location_id))),
        idempotency_key: Some(common::unique_key()),
    };

    let create_resp = orders_api::create_order(config, create_req)
        .await
        .expect("create_order should succeed");

    let created_order = create_resp
        .order
        .expect("create response should contain an order");
    let order_id = created_order.id.as_ref().expect("created order should have an id");

    // Now retrieve it by ID.
    let retrieve_resp = orders_api::retrieve_order(config, order_id)
        .await
        .expect("retrieve_order should succeed");

    let retrieved_order = retrieve_resp
        .order
        .expect("retrieve response should contain an order");

    assert_eq!(
        retrieved_order.id.as_deref(),
        Some(order_id.as_str()),
        "retrieved order ID should match the created order ID"
    );
    assert_eq!(
        retrieved_order.location_id, location_id,
        "retrieved order location_id should match"
    );

    let line_items = retrieved_order
        .line_items
        .expect("retrieved order should have line_items outer option")
        .expect("retrieved order should have line_items inner option");

    assert_eq!(
        line_items.len(),
        1,
        "retrieved order should have exactly one line item"
    );
}

#[tokio::test]
async fn test_search_orders() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();
    let location_id = common::default_location_id(&client).await;

    let search_req = models::SearchOrdersRequest {
        location_ids: Some(vec![location_id]),
        limit: Some(1),
        return_entries: Some(false),
        ..Default::default()
    };

    let resp = orders_api::search_orders(config, search_req)
        .await
        .expect("search_orders should succeed");

    // The search may return orders or an empty result, but it should not error.
    // If orders exist, verify the response structure is sound.
    if let Some(orders) = &resp.orders {
        for order in orders {
            assert!(
                !order.location_id.is_empty(),
                "each returned order should have a non-empty location_id"
            );
        }
    }
}

#[tokio::test]
async fn test_calculate_order() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();
    let location_id = common::default_location_id(&client).await;

    let order = sample_order(&location_id);

    let calc_req = models::CalculateOrderRequest::new(order);

    let resp = orders_api::calculate_order(config, calc_req)
        .await
        .expect("calculate_order should succeed");

    let calculated_order = resp
        .order
        .expect("calculate response should contain an order");

    // The calculated order should have total_money populated by the server.
    let total = calculated_order
        .total_money
        .expect("calculated order should have total_money");

    assert_eq!(
        total.currency,
        Some(models::Currency::Usd),
        "calculated total currency should be USD"
    );

    // We sent a single item at 500 cents ($5.00), so the total amount should be 500.
    assert_eq!(
        total.amount,
        Some(Some(500)),
        "calculated total amount should be 500 cents"
    );
}
