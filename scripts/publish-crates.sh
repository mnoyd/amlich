#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$ROOT_DIR"

echo "=== Publishing Amlich Crates to crates.io ==="
echo ""

# Check if logged in
if ! cargo login --help &> /dev/null; then
    echo "Error: cargo-login not available. Run 'cargo login' first."
    exit 1
fi

# Publish in dependency order
echo "→ Publishing amlich-core..."
cd crates/amlich-core
cargo publish
cd "$ROOT_DIR"

echo ""
echo "⏳ Waiting for crates.io to index amlich-core..."
sleep 30

echo "→ Publishing amlich-api..."
cd crates/amlich-api
cargo publish
cd "$ROOT_DIR"

echo ""
echo "⏳ Waiting for crates.io to index amlich-api..."
sleep 30

echo "→ Publishing amlich-cli (binary: amlich)..."
cd crates/amlich
cargo publish
cd "$ROOT_DIR"

echo ""
echo "=== Publish Complete ==="
echo "View at: https://crates.io/crates/amlich-cli"
