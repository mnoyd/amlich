#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
INSTALL_DIR="${HOME}/.local/bin"

cd "$ROOT_DIR"

echo "=== Installing Amlich CLI ==="

# Build release
echo "→ Building CLI..."
cargo build --release --package amlich

# Create install directory
mkdir -p "$INSTALL_DIR"

# Copy binary
echo "→ Installing to $INSTALL_DIR/amlich..."
cp target/release/amlich "$INSTALL_DIR/amlich"
chmod +x "$INSTALL_DIR/amlich"

# Verify installation
if command -v amlich &> /dev/null; then
    echo ""
    echo "✓ Installation successful!"
    echo "  Run 'amlich' to get started"
else
    echo ""
    echo "⚠ Binary installed but not in PATH"
    echo "  Add this to your shell config:"
    echo "    export PATH=\"\$HOME/.local/bin:\$PATH\""
fi
