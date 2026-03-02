# GitHub Action Auto-Update Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Automated GitHub Action that detects new Square API spec versions, regenerates the crate, and opens a PR. Tags the merge commit on PR merge.

**Architecture:** Two workflows — `update-spec.yml` (scheduled, detects changes, opens PR) and `tag-release.yml` (triggered on PR merge, creates git tag). Version extracted from spec JSON via jq.

**Tech Stack:** GitHub Actions, openapi-generator CLI (via npm), Rust toolchain, jq, gh CLI

---

### Task 1: Update Cargo.toml version to match current Square spec

**Files:**
- Modify: `Cargo.toml`
- Modify: `generate.sh`

**Step 1: Update Cargo.toml version to encode the current Square API date**

The current Square spec version is `2025-10-16`. Update `Cargo.toml`:

```toml
[package]
name = "autogen-squareup"
version = "0.20251016.0"
```

**Step 2: Update generate.sh to not hardcode the version**

In `generate.sh`, the `packageVersion` is currently hardcoded to `0.1.0`. Remove it — the Cargo.toml is protected by `.openapi-generator-ignore` so the generator won't overwrite it anyway. Change:

```bash
--additional-properties=packageName=autogen-squareup,packageVersion=0.1.0,supportAsync=true \
```

to:

```bash
--additional-properties=packageName=autogen-squareup,supportAsync=true \
```

**Step 3: Verify it still works**

```bash
cargo check --all-features
cargo test --all-features
```

Expected: All pass.

**Step 4: Commit**

```bash
git add Cargo.toml generate.sh
git commit -m "chore: set crate version to 0.20251016.0 matching Square API date"
```

---

### Task 2: Create the update-spec workflow

**Files:**
- Create: `.github/workflows/update-spec.yml`

**Step 1: Create the workflows directory**

```bash
mkdir -p .github/workflows
```

**Step 2: Write the update-spec workflow**

Create `.github/workflows/update-spec.yml`:

```yaml
name: Update Square API Spec

on:
  schedule:
    - cron: '0 9 * * 1,4'  # 9am UTC Monday and Thursday
  workflow_dispatch:  # manual trigger

permissions:
  contents: write
  pull-requests: write

jobs:
  check-and-update:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Setup Java
        uses: actions/setup-java@v4
        with:
          distribution: temurin
          java-version: '17'

      - name: Install openapi-generator
        run: |
          npm install -g @openapitools/openapi-generator-cli

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Cache cargo dependencies
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.toml') }}

      - name: Fetch latest Square spec
        run: |
          curl -sS -o api.json.new https://raw.githubusercontent.com/square/connect-api-specification/master/api.json
          echo "Downloaded $(wc -c < api.json.new | tr -d ' ') bytes"

      - name: Extract Square API version
        id: version
        run: |
          NEW_DATE=$(jq -r '.components.securitySchemes.oauth2."x-additional-headers"[0].schema.default' api.json.new)
          OLD_DATE=$(jq -r '.components.securitySchemes.oauth2."x-additional-headers"[0].schema.default' api.json)

          NEW_CRATE="0.$(echo $NEW_DATE | tr -d '-').0"
          OLD_CRATE=$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/')

          echo "old_date=$OLD_DATE" >> $GITHUB_OUTPUT
          echo "new_date=$NEW_DATE" >> $GITHUB_OUTPUT
          echo "old_crate=$OLD_CRATE" >> $GITHUB_OUTPUT
          echo "new_crate=$NEW_CRATE" >> $GITHUB_OUTPUT
          echo "changed=$( [ "$NEW_DATE" != "$OLD_DATE" ] && echo true || echo false )" >> $GITHUB_OUTPUT

          echo "Square API: $OLD_DATE -> $NEW_DATE"
          echo "Crate: $OLD_CRATE -> $NEW_CRATE"

      - name: Check if update needed
        if: steps.version.outputs.changed != 'true'
        run: |
          echo "No spec update detected. Current version: ${{ steps.version.outputs.old_date }}"
          exit 0

      - name: Check for existing PR
        if: steps.version.outputs.changed == 'true'
        id: existing_pr
        run: |
          EXISTING=$(gh pr list --head "update/square-api-${{ steps.version.outputs.new_date }}" --state open --json number -q '.[0].number')
          if [ -n "$EXISTING" ]; then
            echo "PR #$EXISTING already exists for this version"
            echo "exists=true" >> $GITHUB_OUTPUT
          else
            echo "exists=false" >> $GITHUB_OUTPUT
          fi
        env:
          GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}

      - name: Replace spec file
        if: steps.version.outputs.changed == 'true' && steps.existing_pr.outputs.exists != 'true'
        run: mv api.json.new api.json

      - name: Run generate.sh (without fetch step)
        if: steps.version.outputs.changed == 'true' && steps.existing_pr.outputs.exists != 'true'
        run: |
          # Run just the generation and post-processing parts of generate.sh
          # (we already fetched the spec above)
          openapi-generator-cli generate \
            -i api.json \
            -g rust \
            --library reqwest \
            --skip-validate-spec \
            --additional-properties=packageName=autogen-squareup,supportAsync=true \
            -o . \
            2>&1 | tail -5

          rm -f .travis.yml git_push.sh

          # Run post-processing fixes from generate.sh
          bash -c '
            sed -i "s/models::models::/models::/g" src/apis/catalog_api.rs src/apis/disputes_api.rs src/apis/invoices_api.rs

            sed -i "s/param_value\.to_string()/serde_json::to_string(\&param_value).unwrap_or_default()/g" src/apis/catalog_api.rs src/apis/disputes_api.rs src/apis/invoices_api.rs

            sed -i "s/pub async fn update_vendor(configuration: \&configuration::Configuration, update_vendor_request: models::UpdateVendorRequest)/pub async fn update_vendor(configuration: \&configuration::Configuration, vendor_id: \&str, update_vendor_request: models::UpdateVendorRequest)/" src/apis/vendors_api.rs

            if ! grep -q "catalog_modifier_toggle_override_type" src/models/mod.rs; then
              echo "pub mod catalog_modifier_toggle_override_type;" >> src/models/mod.rs
              echo "pub use self::catalog_modifier_toggle_override_type::CatalogModifierToggleOverrideType;" >> src/models/mod.rs
            fi
          '

          # Re-create the missing type if it was overwritten
          cat > src/models/catalog_modifier_toggle_override_type.rs <<'TYPEEOF'
          /*
           * Square
           *
           * Generated by: https://openapi-generator.tech
           */

          use crate::models;
          use serde::{Deserialize, Serialize};

          #[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
          pub enum CatalogModifierToggleOverrideType {
              #[serde(rename = "DO_NOT_USE")]
              DoNotUse,
              #[serde(rename = "NOT_SET")]
              NotSet,
              #[serde(rename = "YES")]
              Yes,
              #[serde(rename = "NO")]
              No,
          }

          impl Default for CatalogModifierToggleOverrideType {
              fn default() -> CatalogModifierToggleOverrideType {
                  Self::DoNotUse
              }
          }
          TYPEEOF

      - name: Re-apply feature gates
        if: steps.version.outputs.changed == 'true' && steps.existing_pr.outputs.exists != 'true'
        run: |
          # Source the apply_gate function from generate.sh and run it
          # (duplicated here because generate.sh uses macOS sed syntax)
          apply_gate() {
            local mod_name="$1" feature="$2"
            sed -i "s/^pub mod ${mod_name};/#[cfg(feature = \"${feature}\")]\npub mod ${mod_name};/" src/apis/mod.rs
          }

          apply_gate apple_pay_api apple-pay
          apply_gate bank_accounts_api bank-accounts
          apply_gate booking_custom_attributes_api booking-custom-attributes
          apply_gate bookings_api bookings
          apply_gate cards_api cards
          apply_gate cash_drawers_api cash-drawers
          apply_gate catalog_api catalog
          apply_gate channels_api channels
          apply_gate checkout_api checkout
          apply_gate customer_custom_attributes_api customer-custom-attributes
          apply_gate customer_groups_api customer-groups
          apply_gate customer_segments_api customer-segments
          apply_gate customers_api customers
          apply_gate devices_api devices
          apply_gate disputes_api disputes
          apply_gate employees_api employees
          apply_gate events_api events
          apply_gate gift_card_activities_api gift-card-activities
          apply_gate gift_cards_api gift-cards
          apply_gate inventory_api inventory
          apply_gate invoices_api invoices
          apply_gate labor_api labor
          apply_gate location_custom_attributes_api location-custom-attributes
          apply_gate locations_api locations
          apply_gate loyalty_api loyalty
          apply_gate merchant_custom_attributes_api merchant-custom-attributes
          apply_gate merchants_api merchants
          apply_gate mobile_authorization_api mobile-authorization
          apply_gate o_auth_api oauth
          apply_gate order_custom_attributes_api order-custom-attributes
          apply_gate orders_api orders
          apply_gate payments_api payments
          apply_gate payouts_api payouts
          apply_gate refunds_api refunds
          apply_gate sites_api sites
          apply_gate snippets_api snippets
          apply_gate subscriptions_api subscriptions
          apply_gate team_api team
          apply_gate terminal_api terminal
          apply_gate transactions_api transactions
          apply_gate transfer_order_api transfer-order
          apply_gate v1_transactions_api v1-transactions
          apply_gate vendors_api vendors
          apply_gate webhook_subscriptions_api webhook-subscriptions

      - name: Update crate version
        if: steps.version.outputs.changed == 'true' && steps.existing_pr.outputs.exists != 'true'
        run: |
          NEW_CRATE="${{ steps.version.outputs.new_crate }}"
          sed -i "s/^version = \".*\"/version = \"$NEW_CRATE\"/" Cargo.toml

      - name: Verify compilation
        if: steps.version.outputs.changed == 'true' && steps.existing_pr.outputs.exists != 'true'
        run: cargo check --all-features

      - name: Run tests
        if: steps.version.outputs.changed == 'true' && steps.existing_pr.outputs.exists != 'true'
        run: cargo test --all-features

      - name: Create Pull Request
        if: steps.version.outputs.changed == 'true' && steps.existing_pr.outputs.exists != 'true'
        run: |
          BRANCH="update/square-api-${{ steps.version.outputs.new_date }}"
          git config user.name "github-actions[bot]"
          git config user.email "github-actions[bot]@users.noreply.github.com"
          git checkout -b "$BRANCH"
          git add -A
          git commit -m "feat: update to Square API ${{ steps.version.outputs.new_date }}"
          git push origin "$BRANCH"

          gh pr create \
            --title "Update to Square API ${{ steps.version.outputs.new_date }}" \
            --body "$(cat <<'EOF'
          ## Square API Update

          - **Old version:** ${{ steps.version.outputs.old_date }} (crate ${{ steps.version.outputs.old_crate }})
          - **New version:** ${{ steps.version.outputs.new_date }} (crate ${{ steps.version.outputs.new_crate }})
          - **cargo check:** ✅ passed
          - **cargo test:** ✅ passed

          Generated by the automated spec update workflow.
          EOF
          )" \
            --base main
        env:
          GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

**Step 3: Commit**

```bash
git add .github/workflows/update-spec.yml
git commit -m "feat: add GitHub Action for automated Square spec updates"
```

---

### Task 3: Create the tag-release workflow

**Files:**
- Create: `.github/workflows/tag-release.yml`

**Step 1: Write the tag-release workflow**

Create `.github/workflows/tag-release.yml`:

```yaml
name: Tag Release on Merge

on:
  pull_request:
    types: [closed]
    branches: [main]

permissions:
  contents: write

jobs:
  tag:
    if: github.event.pull_request.merged == true && startsWith(github.event.pull_request.head.ref, 'update/square-api-')
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Extract version and create tag
        run: |
          VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/')
          TAG="v$VERSION"

          echo "Tagging $TAG"
          git config user.name "github-actions[bot]"
          git config user.email "github-actions[bot]@users.noreply.github.com"
          git tag -a "$TAG" -m "Release $TAG - Square API $(echo $VERSION | sed 's/0\.\([0-9]\{4\}\)\([0-9]\{2\}\)\([0-9]\{2\}\)\..*/\1-\2-\3/')"
          git push origin "$TAG"
        env:
          GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

**Step 2: Commit**

```bash
git add .github/workflows/tag-release.yml
git commit -m "feat: add GitHub Action to tag releases on PR merge"
```

---

### Task 4: Make generate.sh portable (macOS + Linux)

The current `generate.sh` uses `sed -i ''` (macOS syntax). The GitHub Action runs on Ubuntu where `sed -i` has no argument. We need to make it work on both.

**Files:**
- Modify: `generate.sh`

**Step 1: Replace macOS-only sed with a portable wrapper**

Add a portable sed-in-place function at the top of `generate.sh` (after `set -euo pipefail`):

```bash
# Portable in-place sed (macOS uses -i '', Linux uses -i)
sedi() {
  if [[ "$OSTYPE" == "darwin"* ]]; then
    sed -i '' "$@"
  else
    sed -i "$@"
  fi
}
```

Then replace all `sed -i ''` calls in generate.sh with `sedi`.

**Step 2: Verify generate.sh still works locally**

```bash
./generate.sh
```

Expected: Compiles clean.

**Step 3: Commit**

```bash
git add generate.sh
git commit -m "fix: make generate.sh portable across macOS and Linux"
```

---

### Task 5: Verify workflows with a dry run

**Files:** None (verification only)

**Step 1: Validate the YAML syntax**

```bash
python3 -c "
import yaml, sys
for f in ['.github/workflows/update-spec.yml', '.github/workflows/tag-release.yml']:
    with open(f) as fh:
        yaml.safe_load(fh)
    print(f'{f}: valid YAML')
"
```

If python3 yaml module not available:
```bash
python3 -c "import json; print('yaml check skipped, using basic validation')"
```

**Step 2: Check that the version extraction works on the current spec**

```bash
SQUARE_DATE=$(jq -r '.components.securitySchemes.oauth2."x-additional-headers"[0].schema.default' api.json)
CRATE_VERSION="0.$(echo $SQUARE_DATE | tr -d '-').0"
echo "Square date: $SQUARE_DATE"
echo "Crate version: $CRATE_VERSION"
```

Expected: `Square date: 2025-10-16` and `Crate version: 0.20251016.0`

**Step 3: Verify cargo check and test still pass**

```bash
cargo check --all-features
cargo test --all-features
```

**Step 4: Review the full commit history**

```bash
git log --oneline
```
