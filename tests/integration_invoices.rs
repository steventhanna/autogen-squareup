#![cfg(feature = "integration")]

mod common;

use autogen_squareup::apis::invoices_api;
use autogen_squareup::apis::orders_api;
use autogen_squareup::models;

/// Helper: create an order suitable for invoicing at the given location.
///
/// The order contains a single ad-hoc line item worth $10.00 (1000 cents).
/// Square requires an existing order in OPEN state before an invoice can
/// reference it.
async fn create_order_for_invoice(
    config: &autogen_squareup::apis::configuration::Configuration,
    location_id: &str,
) -> String {
    let mut line_item = models::OrderLineItem::new("1".to_string());
    line_item.name = Some(Some("Invoice Test Item".to_string()));
    line_item.base_price_money = Some(Box::new(models::Money {
        amount: Some(Some(1000)),
        currency: Some(models::Currency::Usd),
    }));

    let mut order = models::Order::new(location_id.to_string());
    order.line_items = Some(Some(vec![line_item]));

    let create_req = models::CreateOrderRequest {
        order: Some(Box::new(order)),
        idempotency_key: Some(common::unique_key()),
    };

    let resp = orders_api::create_order(config, create_req)
        .await
        .expect("create_order should succeed for invoice test");

    resp.order
        .expect("create_order response should contain an order")
        .id
        .expect("created order should have an id")
}

/// Helper: build an Invoice struct ready for creation.
///
/// Sets location_id, order_id, a single BALANCE payment request with a
/// due date 30 days in the future, and delivery_method = SHARE_MANUALLY
/// (to avoid needing a customer email in the sandbox).
fn build_invoice(location_id: &str, order_id: &str) -> models::Invoice {
    let mut payment_request = models::InvoicePaymentRequest::new();
    payment_request.request_type = Some(models::InvoiceRequestType::Balance);
    payment_request.due_date = Some(Some("2026-04-02".to_string()));

    let accepted = models::InvoiceAcceptedPaymentMethods {
        card: Some(Some(true)),
        square_gift_card: Some(Some(false)),
        bank_account: Some(Some(false)),
        buy_now_pay_later: Some(Some(false)),
        cash_app_pay: Some(Some(false)),
    };

    let mut invoice = models::Invoice::new();
    invoice.location_id = Some(Some(location_id.to_string()));
    invoice.order_id = Some(Some(order_id.to_string()));
    invoice.payment_requests = Some(Some(vec![payment_request]));
    invoice.delivery_method = Some(models::InvoiceDeliveryMethod::ShareManually);
    invoice.accepted_payment_methods = Some(Box::new(accepted));
    invoice.title = Some(Some("Integration Test Invoice".to_string()));
    invoice
}

#[tokio::test]
async fn test_list_invoices() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();
    let location_id = common::default_location_id(&client).await;

    let resp = invoices_api::list_invoices(config, &location_id, None, None)
        .await
        .expect("list_invoices should succeed");

    // The call should succeed. There may or may not be invoices yet.
    // If invoices are returned, verify the basic structure.
    if let Some(invoices) = &resp.invoices {
        for inv in invoices {
            assert!(
                inv.id.is_some(),
                "each returned invoice should have an id"
            );
        }
    }
}

#[tokio::test]
async fn test_invoice_lifecycle() {
    skip_if_no_token!();
    let client = common::sandbox_client();
    let config = client.config();
    let location_id = common::default_location_id(&client).await;

    // ---- Step 1: Create an order for the invoice to reference ----
    let order_id = create_order_for_invoice(config, &location_id).await;

    // ---- Step 2: Create a draft invoice ----
    let invoice = build_invoice(&location_id, &order_id);
    let create_req = models::CreateInvoiceRequest {
        invoice: Box::new(invoice),
        idempotency_key: Some(common::unique_key()),
    };

    let create_resp = invoices_api::create_invoice(config, create_req)
        .await
        .expect("create_invoice should succeed");

    let created_invoice = create_resp
        .invoice
        .expect("create_invoice response should contain an invoice");

    let invoice_id = created_invoice
        .id
        .as_ref()
        .expect("created invoice should have an id")
        .clone();

    let version = created_invoice
        .version
        .expect("created invoice should have a version");

    assert_eq!(
        created_invoice.status,
        Some(models::InvoiceStatus::Draft),
        "newly created invoice should be in DRAFT status"
    );

    assert_eq!(
        created_invoice.order_id,
        Some(Some(order_id.clone())),
        "invoice order_id should match the order we created"
    );

    assert_eq!(
        created_invoice.title,
        Some(Some("Integration Test Invoice".to_string())),
        "invoice title should match"
    );

    // ---- Step 3: Get the invoice by ID ----
    let get_resp = invoices_api::get_invoice(config, &invoice_id)
        .await
        .expect("get_invoice should succeed");

    let fetched_invoice = get_resp
        .invoice
        .expect("get_invoice response should contain an invoice");

    assert_eq!(
        fetched_invoice.id.as_deref(),
        Some(invoice_id.as_str()),
        "fetched invoice ID should match the created invoice ID"
    );

    assert_eq!(
        fetched_invoice.location_id,
        Some(Some(location_id.clone())),
        "fetched invoice location_id should match"
    );

    // ---- Step 4: Update the invoice (change the title) ----
    let mut update_invoice = models::Invoice::new();
    update_invoice.version = Some(version);
    update_invoice.title = Some(Some("Updated Test Invoice".to_string()));

    let update_req = models::UpdateInvoiceRequest {
        invoice: Box::new(update_invoice),
        idempotency_key: Some(Some(common::unique_key())),
        fields_to_clear: None,
    };

    let update_resp = invoices_api::update_invoice(config, &invoice_id, update_req)
        .await
        .expect("update_invoice should succeed");

    let updated_invoice = update_resp
        .invoice
        .expect("update_invoice response should contain an invoice");

    assert_eq!(
        updated_invoice.title,
        Some(Some("Updated Test Invoice".to_string())),
        "invoice title should be updated"
    );

    let updated_version = updated_invoice
        .version
        .expect("updated invoice should have a version");

    assert!(
        updated_version > version,
        "version should be incremented after update"
    );

    // ---- Step 5: Search for the invoice ----
    let filter = models::InvoiceFilter::new(vec![location_id.clone()]);
    let query = models::InvoiceQuery::new(filter);
    let search_req = models::SearchInvoicesRequest::new(query);

    let search_resp = invoices_api::search_invoices(config, search_req)
        .await
        .expect("search_invoices should succeed");

    // The search should return at least the invoice we just created.
    let search_invoices = search_resp
        .invoices
        .expect("search_invoices response should contain invoices");

    let found = search_invoices
        .iter()
        .any(|inv| inv.id.as_deref() == Some(invoice_id.as_str()));

    assert!(
        found,
        "search results should include the invoice we created"
    );

    // ---- Step 6: Delete the draft invoice (cleanup) ----
    let delete_resp = invoices_api::delete_invoice(config, &invoice_id, Some(updated_version))
        .await
        .expect("delete_invoice should succeed for a draft invoice");

    // DeleteInvoiceResponse only has an errors field; absence of errors means success.
    assert!(
        delete_resp.errors.is_none(),
        "delete_invoice should not return errors"
    );

    // Verify the invoice is gone by attempting to fetch it.
    let get_after_delete = invoices_api::get_invoice(config, &invoice_id).await;
    assert!(
        get_after_delete.is_err(),
        "get_invoice should fail after the invoice is deleted"
    );
}
