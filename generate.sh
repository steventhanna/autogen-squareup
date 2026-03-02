#!/bin/bash
set -euo pipefail

SPEC_URL="https://raw.githubusercontent.com/square/connect-api-specification/master/api.json"

echo "==> Fetching Square OpenAPI spec..."
curl -sS -o api.json "$SPEC_URL"
echo "    Downloaded $(wc -c < api.json | tr -d ' ') bytes"

echo "==> Running openapi-generator..."
openapi-generator generate \
  -i api.json \
  -g rust \
  --library reqwest \
  --skip-validate-spec \
  --additional-properties=packageName=square-api,packageVersion=0.1.0,supportAsync=true \
  -o . \
  2>&1 | tail -5

echo "==> Cleaning up generated files we don't need..."
rm -f .travis.yml git_push.sh

echo "==> Verifying compilation..."
cargo check

echo "==> Done. Review changes with: git diff"
