# Amlich - Vietnamese Lunar Calendar
# Run `just` to see available commands

set dotenv-load

# Default: show help
default:
    @just --list

# ============== Build ==============

# Build all Rust crates (release)
build:
    cargo build --release

# Build only the CLI
build-cli:
    cargo build --release --package amlich-cli

# Build WASM package
build-wasm:
    cd crates/amlich-wasm && wasm-pack build --target web --out-dir ../../dist/wasm

# Build desktop app
build-app:
    cd apps/desktop && pnpm tauri build

# Build everything
build-all: build build-wasm build-app

# ============== Test ==============

# Run all tests
test:
    cargo test --workspace --exclude app

# Run core library tests
test-core:
    cargo test --package amlich-core

# Run tests with output
test-verbose:
    cargo test --workspace --exclude app -- --nocapture

# ============== Development ==============

# Run desktop app in dev mode
dev:
    cd apps/desktop && pnpm tauri dev

# Check code without building
check:
    cargo check --workspace --exclude app

# Format code
fmt:
    cargo fmt --all

# Lint code
lint:
    cargo clippy --workspace --exclude app -- -D warnings

# ============== Install ==============

# Install CLI to ~/.local/bin
install-cli:
    cargo install --path crates/amlich-cli --root ~/.local

# Install CLI with waybar integration
install-waybar:
    ./scripts/install_with_waybar.sh

# ============== Clean ==============

# Clean all build artifacts
clean:
    cargo clean
    rm -rf dist
    rm -rf apps/desktop/.svelte-kit
    rm -rf apps/desktop/src-tauri/target

# ============== Release ==============

# Bump version (usage: just bump 0.2.0)
bump version:
    sed -i 's/^version = .*/version = "{{version}}"/' Cargo.toml
    sed -i 's/"version": .*/"version": "{{version}}",/' package.json
    @echo "Version bumped to {{version}}"

# Create a release tag
tag version:
    git tag -a v{{version}} -m "Release v{{version}}"
    @echo "Created tag v{{version}}"

# ============== Info ==============

# Show project info
info:
    @echo "Amlich - Vietnamese Lunar Calendar"
    @echo ""
    @echo "Crates:"
    @ls -1 crates/
    @echo ""
    @echo "Apps:"
    @ls -1 apps/

# Run CLI with today's date
today:
    cargo run --release --package amlich-cli
