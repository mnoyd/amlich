#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$ROOT_DIR"

echo "=== Publishing Amlich WASM to npm ==="
echo ""

# Build WASM first
echo "→ Building WASM..."
cd crates/amlich-wasm
wasm-pack build --target web --out-dir ../../dist/wasm
cd "$ROOT_DIR/dist/wasm"

# Update package.json for npm
echo "→ Preparing package..."
cat > package.json << 'EOF'
{
  "name": "@amlich/wasm",
  "version": "0.1.0",
  "description": "Vietnamese Lunar Calendar - WASM bindings",
  "main": "amlich_wasm.js",
  "types": "amlich_wasm.d.ts",
  "files": [
    "amlich_wasm_bg.wasm",
    "amlich_wasm.js",
    "amlich_wasm.d.ts"
  ],
  "repository": {
    "type": "git",
    "url": "https://github.com/mnoyd/amlich.git"
  },
  "license": "MIT",
  "keywords": ["vietnamese", "lunar", "calendar", "wasm"]
}
EOF

echo "→ Publishing to npm..."
npm publish --access public

cd "$ROOT_DIR"
echo ""
echo "=== Publish Complete ==="
echo "Install with: npm install @amlich/wasm"
