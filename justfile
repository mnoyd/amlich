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
    cargo build --release --package amlich

# Build WASM package
build-wasm:
    cd crates/amlich-wasm && wasm-pack build --target web --out-dir ../../dist/wasm

# Build desktop app (current platform)
build-app:
    cd apps/desktop && pnpm install && pnpm exec svelte-kit sync && pnpm tauri build

# Build desktop app for Linux (AppImage + deb)
build-app-linux:
    cd apps/desktop && pnpm install && pnpm exec svelte-kit sync && pnpm tauri build --bundles appimage,deb
    @echo ""
    @echo "Build outputs:"
    @ls -1 apps/desktop/src-tauri/target/release/bundle/appimage/*.AppImage 2>/dev/null || true
    @ls -1 apps/desktop/src-tauri/target/release/bundle/deb/*.deb 2>/dev/null || true

# Build desktop app for macOS (Universal: Intel + Apple Silicon)
build-app-macos:
    rustup target add aarch64-apple-darwin x86_64-apple-darwin 2>/dev/null || true
    cd apps/desktop && pnpm install && pnpm exec svelte-kit sync && pnpm tauri build --target universal-apple-darwin
    @echo ""
    @echo "Build outputs:"
    @ls -1 apps/desktop/src-tauri/target/universal-apple-darwin/release/bundle/dmg/*.dmg 2>/dev/null || true

# Build everything (current platform)
build-all: build build-wasm build-app

# ============== Test ==============

# Run all tests
test:
    cargo test --workspace --exclude am-lich

# Run core library tests
test-core:
    cargo test --package amlich-core

# Run tests with output
test-verbose:
    cargo test --workspace --exclude am-lich -- --nocapture

# ============== Development ==============

# Run desktop app in dev mode
dev:
    cd apps/desktop && pnpm tauri dev

# Check code without building
check:
    cargo check --workspace --exclude am-lich

# Format code
fmt:
    cargo fmt --all

# Lint code
lint:
    cargo clippy --workspace --exclude am-lich -- -D warnings

# ============== Install ==============

# Install CLI to ~/.local/bin
install-cli:
    cargo install --path crates/amlich --root ~/.local

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
    cargo run --release --package amlich -- query
