#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$ROOT_DIR"

echo "=== Building Amlich ==="
echo ""

echo "→ Building Rust crates..."
cargo build --release

echo ""
echo "→ Building WASM..."
mkdir -p dist/wasm
cd crates/amlich-wasm
wasm-pack build --target web --out-dir ../../dist/wasm
cd "$ROOT_DIR"

echo ""
echo "→ Building desktop app..."
cd apps/desktop
pnpm install
pnpm tauri build
cd "$ROOT_DIR"

echo ""
echo "=== Build Complete ==="
echo "Outputs:"
echo "  - CLI:     target/release/amlich"
echo "  - WASM:    dist/wasm/"
echo "  - Desktop: apps/desktop/src-tauri/target/release/bundle/"
