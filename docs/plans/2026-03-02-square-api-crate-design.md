# Square API Rust Client — Design Document

## Problem

The community `squareup` crate (v2.15.1) is hand-written and has correctness bugs:

1. `due_date` typed as `DateTime` instead of `String` (spec says YYYY-MM-DD)
2. No `skip_serializing_if` on `Option` fields — sends `null` instead of omitting
3. `Money.amount` is `i32` instead of `i64`
4. `env::set_var` usage is not thread-safe in async contexts

## Solution

Auto-generate a strongly-typed Rust client from the official Square OpenAPI spec using `openapi-generator`, published to crates.io as `square-api`.

## Architecture

### Generator: openapi-generator (Java CLI, v7.20.0)

Chosen over progenitor due to:
- Correct nullable handling via double-option pattern (`Option<Option<T>>`)
- Battle-tested on large specs (~3MB, 500+ schemas)
- Generates `skip_serializing_if` on all `Option` fields
- Active maintenance

Progenitor was rejected due to open bugs with nullable fields (#851, #1197, #398, #66) that directly affect the Square spec.

### Generation Strategy: CLI One-Shot

- `generate.sh` fetches the Square spec and runs openapi-generator
- Generated code is committed to the repo
- Re-run when the Square spec updates, review the diff

### Crate Structure

```
square-api/
├── Cargo.toml
├── src/
│   ├── lib.rs              # re-exports, feature gates
│   ├── client.rs           # SquareClient wrapper (auth + env)
│   ├── core/               # shared types (Money, Address, Error, etc.)
│   └── generated/          # openapi-generator output
│       ├── models/
│       └── apis/
├── generate.sh             # fetch spec + run generator + post-process
└── api.json                # committed copy of Square spec
```

### Feature Flags

Each Square API group is a Cargo feature. Core types (Money, Address, Error, etc.) are always compiled.

```toml
[features]
default = ["all"]
all = ["payments", "orders", "invoices", "customers", "catalog",
       "inventory", "subscriptions", "bookings", "loyalty",
       "gift-cards", "labor", "team", "locations", "merchants",
       "oauth", "terminal", "disputes", "refunds", "checkout",
       "cards", "bank-accounts"]
payments = []
orders = []
invoices = []
# ... etc
```

Users opt in:
```toml
square-api = { version = "0.1", default-features = false, features = ["payments", "invoices"] }
```

Shared type resolution: types referenced by 2+ API groups go into `core`. The post-processing step in `generate.sh` analyzes `$ref` dependencies to determine placement.

### Wrapper Layer (client.rs)

Thin wrapper handling only auth and environment:

```rust
pub enum Environment {
    Production,  // https://connect.squareup.com
    Sandbox,     // https://connect.squareupsandbox.com
}

pub struct SquareClient {
    inner: generated::apis::configuration::Configuration,
}

impl SquareClient {
    pub fn new(access_token: &str) -> Self { ... }
    pub fn sandbox(access_token: &str) -> Self { ... }
    pub fn with_env(access_token: &str, env: Environment) -> Self { ... }
}
```

No pagination helpers, no custom error types, no retry logic.

### Type Mapping

From the Square spec:

| OpenAPI | Rust |
|---------|------|
| `nullable: true` + not required | `Option<Option<T>>` with `serde_with::double_option` |
| `nullable: true` + required | `Option<T>` |
| not nullable + not required | `Option<T>` with `skip_serializing_if` |
| `type: string` (date fields) | `String` |
| `type: integer, format: int64` | `i64` |
| `enum` (string) | Rust `enum` with serde rename |

### Dependencies

```toml
[dependencies]
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_with = "3"
url = "2"
```

### Crate Metadata

- Name: `square-api`
- License: MIT
- Keywords: square, payments, api, sdk
- Categories: api-bindings
- Rust edition: 2024

## Generation Pipeline

```bash
#!/bin/bash
# generate.sh

# 1. Fetch latest spec
curl -o api.json https://raw.githubusercontent.com/square/connect-api-specification/master/api.json

# 2. Generate
openapi-generator generate \
  -i api.json \
  -g rust \
  --library reqwest \
  --additional-properties=packageName=square-api \
  -o src/generated/

# 3. Post-process: add #[cfg(feature = "...")] gates
#    and move shared types to core/

# 4. Verify
cargo check
```

## Decisions Log

| Decision | Choice | Reason |
|----------|--------|--------|
| Generator | openapi-generator | Correct nullable handling, proven on large specs |
| Strategy | CLI one-shot | Full control, reviewable diffs |
| Wrapper scope | Thin (auth + env) | YAGNI — users add middleware as needed |
| Feature flags | Per API group | Compile only what you use |
| Crate name | square-api | Available on crates.io |
| Nullable pattern | double-option | Distinguishes omit vs null vs value |
