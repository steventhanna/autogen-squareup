# autogen-squareup

Auto-generated, strongly-typed Rust client for the [Square API](https://developer.squareup.com/reference/square).

Generated from the [official Square OpenAPI spec](https://github.com/square/connect-api-specification) using [openapi-generator](https://openapi-generator.tech/).

## Installation

```toml
[dependencies]
autogen-squareup = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

To use only specific API groups (reduces compile time):

```toml
[dependencies]
autogen-squareup = { version = "0.1", default-features = false, features = ["payments", "customers", "native-tls"] }
```

## Quick Start

```rust
use autogen_squareup::SquareClient;
use autogen_squareup::apis::payments_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SquareClient::new("your-access-token");

    // List payments
    let response = payments_api::list_payments(
        client.config(),
        None, None, None, None, None, None, None, None, None,
    ).await?;

    println!("{:?}", response.payments);
    Ok(())
}
```

## Environments

```rust
use autogen_squareup::SquareClient;

// Production (default)
let client = SquareClient::new("sq0atp-...");

// Sandbox
let client = SquareClient::sandbox("sandbox-sq0atp-...");

// Explicit environment
use autogen_squareup::Environment;
let client = SquareClient::with_env("sq0atp-...", Environment::Production);
```

## Usage Examples

### Create a Payment

```rust
use autogen_squareup::SquareClient;
use autogen_squareup::apis::payments_api;
use autogen_squareup::models::{CreatePaymentRequest, Money};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SquareClient::new("your-access-token");

    let request = CreatePaymentRequest {
        source_id: Some(Some("cnon:card-nonce-ok".to_string())),
        idempotency_key: Some(Some("unique-key-123".to_string())),
        amount_money: Some(Box::new(Money {
            amount: Some(Some(1000)), // $10.00
            currency: Some(Some(autogen_squareup::models::Currency::Usd)),
        })),
        ..Default::default()
    };

    let response = payments_api::create_payment(client.config(), request).await?;
    println!("Payment ID: {:?}", response.payment);
    Ok(())
}
```

### List Locations

```rust
use autogen_squareup::SquareClient;
use autogen_squareup::apis::locations_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SquareClient::new("your-access-token");

    let response = locations_api::list_locations(client.config()).await?;

    if let Some(locations) = response.locations {
        for loc in locations {
            println!("{}: {:?}", loc.id.unwrap_or_default().unwrap_or_default(), loc.name);
        }
    }
    Ok(())
}
```

### List Customers

```rust
use autogen_squareup::SquareClient;
use autogen_squareup::apis::customers_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SquareClient::new("your-access-token");

    let response = customers_api::list_customers(
        client.config(),
        None, None, None, None,
    ).await?;

    if let Some(customers) = response.customers {
        for c in customers {
            println!("{:?} {:?}", c.given_name, c.family_name);
        }
    }
    Ok(())
}
```

## Feature Flags

All 44 API groups are enabled by default. Use `default-features = false` and select only what you need:

| Feature | API |
|---------|-----|
| `payments` | Payments |
| `orders` | Orders |
| `invoices` | Invoices |
| `customers` | Customers |
| `catalog` | Catalog |
| `inventory` | Inventory |
| `subscriptions` | Subscriptions |
| `bookings` | Bookings |
| `loyalty` | Loyalty |
| `gift-cards` | Gift Cards |
| `gift-card-activities` | Gift Card Activities |
| `labor` | Labor |
| `team` | Team |
| `locations` | Locations |
| `merchants` | Merchants |
| `oauth` | OAuth |
| `terminal` | Terminal |
| `disputes` | Disputes |
| `refunds` | Refunds |
| `checkout` | Checkout |
| `cards` | Cards |
| `bank-accounts` | Bank Accounts |
| `apple-pay` | Apple Pay |
| `cash-drawers` | Cash Drawers |
| `customer-groups` | Customer Groups |
| `customer-segments` | Customer Segments |
| `customer-custom-attributes` | Customer Custom Attributes |
| `booking-custom-attributes` | Booking Custom Attributes |
| `location-custom-attributes` | Location Custom Attributes |
| `merchant-custom-attributes` | Merchant Custom Attributes |
| `order-custom-attributes` | Order Custom Attributes |
| `devices` | Devices |
| `employees` | Employees |
| `events` | Events |
| `channels` | Channels |
| `mobile-authorization` | Mobile Authorization |
| `payouts` | Payouts |
| `sites` | Sites |
| `snippets` | Snippets |
| `transactions` | Transactions |
| `transfer-order` | Transfer Order |
| `v1-transactions` | V1 Transactions (deprecated) |
| `vendors` | Vendors |
| `webhook-subscriptions` | Webhook Subscriptions |

**TLS features:**

| Feature | Description |
|---------|-------------|
| `native-tls` | Use system TLS (default) |
| `rustls` | Use rustls instead |

## Nullable Fields

The Square API marks most fields as both optional and nullable. This crate correctly distinguishes between the three states using `Option<Option<T>>`:

```rust
// Field omitted from JSON (not sent)
field: None

// Field explicitly set to null
field: Some(None)

// Field set to a value
field: Some(Some(value))
```

This uses `serde_with::double_option` under the hood, so serialization works correctly:
- `None` — field is omitted from the request body
- `Some(None)` — field is sent as `null`
- `Some(Some(v))` — field is sent with the value

## Regenerating from the Spec

To regenerate the client from the latest Square OpenAPI spec:

```bash
# Requires: brew install openapi-generator
./generate.sh
```

This fetches the latest spec, runs the generator, applies post-processing fixes for known spec bugs, re-applies feature gates, and verifies compilation.

## Architecture

- **Types and API methods** are auto-generated by openapi-generator from the [official Square OpenAPI spec](https://github.com/square/connect-api-specification)
- **`SquareClient`** is a thin hand-written wrapper that handles auth (Bearer token) and environment switching (production/sandbox)
- **Generated code is committed** — no build-time generation, no proc macros. Run `./generate.sh` to update when the spec changes.

## Why Not `squareup`?

The existing [`squareup`](https://crates.io/crates/squareup) crate is hand-written. This crate is auto-generated from the spec, which avoids several issues:

| Issue | `squareup` | `autogen-squareup` |
|-------|-----------|-------------|
| `due_date` type | `DateTime` (wrong — spec says string) | `String` |
| Nullable fields | Sends `null` (no `skip_serializing_if`) | Omits field when `None` |
| `Money.amount` | `i32` (truncates large values) | `i64` |
| Thread safety | `env::set_var` (unsafe in async) | No global state |
| Nullable semantics | Single `Option<T>` | `Option<Option<T>>` (omit vs null vs value) |

## License

MIT
