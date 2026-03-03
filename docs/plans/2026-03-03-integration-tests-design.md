# Integration Tests Design

## Goal
Add integration tests against the Square sandbox API for the 10 highest-value API modules, running in CI via GitHub Actions.

## API Modules Covered
1. **Locations** — list, retrieve, create, update
2. **Customers** — create, retrieve, list, search, update, delete
3. **Catalog** — upsert, retrieve, list, search, batch_upsert, delete
4. **Orders** — create, retrieve, search, update, calculate
5. **Payments** — create, get, list, cancel (uses sandbox nonces)
6. **Refunds** — refund_payment, list, get
7. **Invoices** — create, get, list, search, update, publish, cancel, delete
8. **Cards** — create, list, retrieve, disable
9. **Customer Groups** — create, list, retrieve, update, delete
10. **Webhook Subscriptions** — create, list, retrieve, update, test, delete

## Test Structure
```
tests/
├── client_test.rs              (existing)
├── common/
│   └── mod.rs                  (shared helpers)
├── integration_locations.rs
├── integration_customers.rs
├── integration_catalog.rs
├── integration_orders.rs
├── integration_payments.rs
├── integration_refunds.rs
├── integration_invoices.rs
├── integration_cards.rs
├── integration_customer_groups.rs
└── integration_webhooks.rs
```

## Feature Gate
All integration tests gated behind `--features integration`. Normal `cargo test` won't run them.

## CI
Separate job in `ci.yml`, runs after unit tests pass, skipped on fork PRs. Uses `SQUARE_SANDBOX_TOKEN` secret. Sequential execution (`--test-threads=1`).

## Dependencies Added
- `uuid` (dev) — idempotency keys
