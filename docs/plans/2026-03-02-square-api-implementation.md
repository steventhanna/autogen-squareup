# square-api Crate Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build an auto-generated, strongly-typed Rust client for the Square API, published to crates.io as `square-api`.

**Architecture:** Use openapi-generator (Java CLI) to generate Rust types and API methods from Square's OpenAPI 3.0 spec. Commit the generated code. Wrap it with a thin auth/env client layer. Feature-gate each API group.

**Tech Stack:** openapi-generator v7.20.0, reqwest 0.13, serde, serde_with (double-option nullable), Rust 2024 edition

---

### Task 1: Install openapi-generator and fetch the Square spec

**Files:**
- Create: `api.json`
- Create: `.openapi-generator-ignore`

**Step 1: Verify openapi-generator is installed**

Run: `openapi-generator version`
Expected: A version string like `7.x.x`

If not installed:
```bash
brew install openapi-generator
```

**Step 2: Fetch the Square OpenAPI spec**

```bash
curl -o api.json https://raw.githubusercontent.com/square/connect-api-specification/master/api.json
```

**Step 3: Verify the spec is valid OpenAPI 3.0**

```bash
head -c 200 api.json
```

Expected: Should contain `"openapi": "3.0.0"` near the top.

**Step 4: Create .openapi-generator-ignore to protect our files**

Create `.openapi-generator-ignore`:
```
# Protect our hand-maintained files from being overwritten on re-generation
Cargo.toml
src/lib.rs
src/client.rs
README.md
.gitignore
.travis.yml
git_push.sh
```

**Step 5: Commit**

```bash
git add api.json .openapi-generator-ignore
git commit -m "feat: add Square OpenAPI spec and generator ignore file"
```

---

### Task 2: Run initial generation and evaluate output

**Files:**
- Create: `src/apis/` (generated)
- Create: `src/models/` (generated)

**Step 1: Run openapi-generator against the Square spec**

```bash
openapi-generator generate \
  -i api.json \
  -g rust \
  --library reqwest \
  --additional-properties=packageName=square-api,packageVersion=0.1.0,supportAsync=true \
  -o .
```

Note: We generate into the project root (`.`) because the generator creates `src/apis/` and `src/models/`. Our `.openapi-generator-ignore` protects our Cargo.toml and lib.rs.

**Step 2: Inspect the generated structure**

```bash
ls src/apis/
ls src/models/ | head -30
```

Expected: `src/apis/` contains `mod.rs`, `configuration.rs`, and one file per API tag (e.g., `payments_api.rs`). `src/models/` contains `mod.rs` and one file per schema.

**Step 3: Count generated files to verify completeness**

```bash
ls src/models/*.rs | wc -l
ls src/apis/*.rs | wc -l
```

Expected: models ~300-600 files, apis ~20-50 files.

**Step 4: Spot-check a model with nullable fields**

Read `src/models/money.rs` and verify:
- `amount` is `Option<i64>` (not i32)
- Fields have `skip_serializing_if = "Option::is_none"`
- Nullable+optional fields use `serde_with::rust::double_option`

**Step 5: Spot-check a model with enum fields**

Read a model that uses enums (e.g., look for `Currency` in models). Verify enums have `#[serde(rename = "...")]` attributes.

**Step 6: Spot-check an API file**

Read `src/apis/payments_api.rs` (or whatever the payments file is named). Verify:
- Functions take `&configuration::Configuration`
- Bearer auth is applied via `bearer_access_token`
- Request/response types reference `models::*`

**Step 7: Do NOT commit yet — we need to set up Cargo.toml first**

---

### Task 3: Set up Cargo.toml and lib.rs

**Files:**
- Modify: `Cargo.toml`
- Modify: `src/lib.rs`

**Step 1: Update Cargo.toml with correct dependencies and metadata**

Replace the contents of `Cargo.toml` with:

```toml
[package]
name = "square-api"
version = "0.1.0"
edition = "2021"
license = "MIT"
description = "Auto-generated, strongly-typed Rust client for the Square API"
keywords = ["square", "payments", "api", "sdk"]
categories = ["api-bindings"]
repository = "https://github.com/YOUR_USERNAME/square-api"

[dependencies]
reqwest = { version = "^0.13", default-features = false, features = ["json", "multipart", "query", "form"] }
serde = { version = "^1.0", features = ["derive"] }
serde_json = "^1.0"
serde_with = { version = "^3.8", default-features = false, features = ["base64", "std", "macros"] }
serde_repr = "^0.1"
url = "^2.5"
uuid = { version = "^1.8", features = ["serde", "v4"] }

[dev-dependencies]
tokio = { version = "^1", features = ["macros", "rt-multi-thread"] }

[features]
default = ["native-tls"]
native-tls = ["reqwest/native-tls"]
rustls = ["reqwest/rustls-tls"]
```

Note: We use edition 2021 (not 2024) because the generated code uses `extern crate` declarations which are idiomatic in 2021. We can clean this up later if desired.

**Step 2: Write src/lib.rs**

```rust
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

pub mod apis;
pub mod models;
pub mod client;

pub use client::{SquareClient, Environment};
```

**Step 3: Create a placeholder src/client.rs so it compiles**

```rust
pub enum Environment {
    Production,
    Sandbox,
}

pub struct SquareClient;
```

**Step 4: Run cargo check**

```bash
cargo check 2>&1 | tail -20
```

Expected: Either compiles clean, or there are specific errors to fix. If there are generated code errors, note them — we'll fix in the next task.

**Step 5: Fix any compilation errors in generated code**

Common issues to watch for:
- Missing imports
- Duplicate type names from nullable enums
- Unsupported OpenAPI features

For each error, fix the generated file directly. Document the fix so we can add it to the post-processing script later.

**Step 6: Verify it compiles clean**

```bash
cargo check
```

Expected: No errors.

**Step 7: Commit**

```bash
git add -A
git commit -m "feat: initial openapi-generator output for Square API"
```

---

### Task 4: Write the client wrapper

**Files:**
- Modify: `src/client.rs`
- Create: `tests/client_test.rs`

**Step 1: Write the failing test**

Create `tests/client_test.rs`:

```rust
use square_api::{SquareClient, Environment};

#[test]
fn test_production_client_has_correct_base_path() {
    let client = SquareClient::new("test-token");
    assert_eq!(client.config().base_path, "https://connect.squareup.com/v2");
}

#[test]
fn test_sandbox_client_has_correct_base_path() {
    let client = SquareClient::sandbox("test-token");
    assert_eq!(client.config().base_path, "https://connect.squareupsandbox.com/v2");
}

#[test]
fn test_client_has_bearer_token() {
    let client = SquareClient::new("my-secret-token");
    assert_eq!(
        client.config().bearer_access_token,
        Some("my-secret-token".to_string())
    );
}

#[test]
fn test_with_env_production() {
    let client = SquareClient::with_env("tok", Environment::Production);
    assert_eq!(client.config().base_path, "https://connect.squareup.com/v2");
}

#[test]
fn test_with_env_sandbox() {
    let client = SquareClient::with_env("tok", Environment::Sandbox);
    assert_eq!(client.config().base_path, "https://connect.squareupsandbox.com/v2");
}
```

**Step 2: Run tests to verify they fail**

```bash
cargo test --test client_test 2>&1 | tail -10
```

Expected: FAIL — `SquareClient::new` doesn't exist yet, `config()` method doesn't exist.

**Step 3: Implement client.rs**

```rust
use crate::apis::configuration::Configuration;

const PRODUCTION_URL: &str = "https://connect.squareup.com/v2";
const SANDBOX_URL: &str = "https://connect.squareupsandbox.com/v2";

#[derive(Debug, Clone)]
pub enum Environment {
    Production,
    Sandbox,
}

impl Environment {
    fn base_url(&self) -> &'static str {
        match self {
            Environment::Production => PRODUCTION_URL,
            Environment::Sandbox => SANDBOX_URL,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SquareClient {
    configuration: Configuration,
}

impl SquareClient {
    /// Create a client for the production Square API.
    pub fn new(access_token: &str) -> Self {
        Self::with_env(access_token, Environment::Production)
    }

    /// Create a client for the Square sandbox API.
    pub fn sandbox(access_token: &str) -> Self {
        Self::with_env(access_token, Environment::Sandbox)
    }

    /// Create a client with a specific environment.
    pub fn with_env(access_token: &str, env: Environment) -> Self {
        let mut configuration = Configuration::new();
        configuration.base_path = env.base_url().to_string();
        configuration.bearer_access_token = Some(access_token.to_string());
        configuration.user_agent = Some(format!("square-api-rust/{}", env!("CARGO_PKG_VERSION")));
        Self { configuration }
    }

    /// Access the underlying configuration for use with generated API functions.
    pub fn config(&self) -> &Configuration {
        &self.configuration
    }
}
```

**Step 4: Run tests to verify they pass**

```bash
cargo test --test client_test
```

Expected: All 5 tests pass.

**Step 5: Commit**

```bash
git add src/client.rs tests/client_test.rs
git commit -m "feat: add SquareClient wrapper with auth and env switching"
```

---

### Task 5: Add Cargo feature flags for API groups

**Files:**
- Modify: `Cargo.toml`
- Modify: `src/lib.rs`
- Modify: `src/apis/mod.rs`

**Step 1: Identify all API tags from generated files**

```bash
ls src/apis/*_api.rs | sed 's|src/apis/||;s|_api.rs||' | sort
```

This gives us the list of API groups to create features for.

**Step 2: Add feature flags to Cargo.toml**

Add a `[features]` section. The exact feature names depend on what Step 1 reveals, but it will look like:

```toml
[features]
default = ["all", "native-tls"]
native-tls = ["reqwest/native-tls"]
rustls = ["reqwest/rustls-tls"]
all = ["payments", "orders", "invoices", "customers", "catalog",
       "inventory", "subscriptions", "bookings", "loyalty",
       "gift-cards", "labor", "team", "locations", "merchants",
       "oauth", "terminal", "disputes", "refunds", "checkout",
       "cards", "bank-accounts"]
payments = []
orders = []
invoices = []
customers = []
catalog = []
inventory = []
subscriptions = []
bookings = []
loyalty = []
gift-cards = []
labor = []
team = []
locations = []
merchants = []
oauth = []
terminal = []
disputes = []
refunds = []
checkout = []
cards = []
bank-accounts = []
```

Adjust feature names to match the actual API tag filenames from Step 1.

**Step 3: Add cfg gates to src/apis/mod.rs**

For each API module declaration in `src/apis/mod.rs`, add a `#[cfg(feature = "...")]` gate. For example:

```rust
#[cfg(feature = "payments")]
pub mod payments_api;

#[cfg(feature = "invoices")]
pub mod invoices_api;

// configuration.rs is always available (no feature gate)
pub mod configuration;
```

**Step 4: Verify it compiles with all features**

```bash
cargo check --all-features
```

Expected: Compiles clean.

**Step 5: Verify it compiles with a single feature**

```bash
cargo check --no-default-features --features "payments,native-tls"
```

Expected: Compiles clean. Only payments types and APIs are included.

Note: Models are NOT feature-gated in this initial pass. All models compile always. This is a pragmatic choice — the cross-referencing between models is complex (Money is used everywhere, Order is used by orders and payments, etc.). Feature-gating models can be added later as an optimization if compile times are an issue. Feature-gating APIs is the 80/20 win.

**Step 6: Commit**

```bash
git add Cargo.toml src/lib.rs src/apis/mod.rs
git commit -m "feat: add Cargo feature flags for API groups"
```

---

### Task 6: Create the generate.sh script

**Files:**
- Create: `generate.sh`

**Step 1: Write generate.sh**

```bash
#!/bin/bash
set -euo pipefail

SPEC_URL="https://raw.githubusercontent.com/square/connect-api-specification/master/api.json"
OUTPUT_DIR="."

echo "==> Fetching Square OpenAPI spec..."
curl -sS -o api.json "$SPEC_URL"
echo "    Downloaded $(wc -c < api.json | tr -d ' ') bytes"

echo "==> Running openapi-generator..."
openapi-generator generate \
  -i api.json \
  -g rust \
  --library reqwest \
  --additional-properties=packageName=square-api,packageVersion=0.1.0,supportAsync=true \
  -o "$OUTPUT_DIR" \
  2>&1 | tail -5

echo "==> Cleaning up generated files we don't need..."
rm -f .travis.yml git_push.sh

echo "==> Verifying compilation..."
cargo check

echo "==> Done. Review changes with: git diff"
```

**Step 2: Make it executable**

```bash
chmod +x generate.sh
```

**Step 3: Commit**

```bash
git add generate.sh
git commit -m "feat: add generate.sh for re-running code generation"
```

---

### Task 7: Update .gitignore and clean up

**Files:**
- Modify: `.gitignore`

**Step 1: Update .gitignore**

```
/target
.travis.yml
git_push.sh
```

We explicitly commit the generated code (that's the CLI one-shot strategy), so `src/apis/` and `src/models/` are NOT in .gitignore.

**Step 2: Remove any junk files the generator created**

```bash
rm -f .travis.yml git_push.sh
```

**Step 3: Final cargo check**

```bash
cargo check --all-features
```

**Step 4: Commit**

```bash
git add -A
git commit -m "chore: clean up gitignore and remove generated junk files"
```

---

### Task 8: Verify key type correctness

**Files:** None (read-only verification)

**Step 1: Verify Money.amount is i64**

```bash
grep -A5 'pub amount' src/models/money.rs
```

Expected: `Option<i64>` (not i32).

**Step 2: Verify due_date is String**

Find an invoice-related model with due_date:

```bash
grep -rl 'due_date' src/models/ | head -3
```

Then read the file and verify `due_date` is `Option<String>` or `Option<Option<String>>` (not DateTime).

**Step 3: Verify skip_serializing_if is present**

```bash
grep -c 'skip_serializing_if' src/models/money.rs
```

Expected: At least 1 occurrence.

**Step 4: Verify double-option pattern for nullable+optional**

```bash
grep -c 'double_option' src/models/ -r
```

Expected: Many occurrences (Square marks almost everything nullable+optional).

**Step 5: Verify bearer auth in API methods**

```bash
grep 'bearer_access_token\|bearer_auth' src/apis/payments_api.rs | head -3
```

Expected: Shows bearer token being applied to requests.

If any of these checks fail, go back and fix the issue before proceeding.

---

### Task 9: Add crate-level documentation

**Files:**
- Modify: `src/lib.rs`

**Step 1: Add crate-level doc comment to lib.rs**

Add to the top of `src/lib.rs`:

```rust
//! # square-api
//!
//! Auto-generated, strongly-typed Rust client for the [Square API](https://developer.squareup.com/reference/square).
//!
//! ## Quick Start
//!
//! ```no_run
//! use square_api::SquareClient;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = SquareClient::new("your-access-token");
//!     // Use client.config() with generated API functions
//! }
//! ```
//!
//! ## Feature Flags
//!
//! By default all API groups are enabled. To reduce compile time, select only what you need:
//!
//! ```toml
//! [dependencies]
//! square-api = { version = "0.1", default-features = false, features = ["payments", "native-tls"] }
//! ```
```

**Step 2: Run cargo doc to verify**

```bash
cargo doc --no-deps 2>&1 | tail -5
```

Expected: Documentation builds without errors.

**Step 3: Commit**

```bash
git add src/lib.rs
git commit -m "docs: add crate-level documentation with quick start example"
```

---

### Task 10: Final validation

**Step 1: Full build**

```bash
cargo build --all-features
```

**Step 2: Run all tests**

```bash
cargo test --all-features
```

**Step 3: Check with minimal features**

```bash
cargo check --no-default-features --features "payments,native-tls"
cargo check --no-default-features --features "invoices,native-tls"
```

**Step 4: Dry-run publish**

```bash
cargo publish --dry-run
```

Expected: No errors (or only warnings about missing fields we can fix).

**Step 5: Review the full diff since initial commit**

```bash
git log --oneline
```

Verify commit history is clean and tells the story of the implementation.
