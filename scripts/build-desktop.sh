#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DESKTOP_DIR="$ROOT_DIR/apps/desktop"

info() { echo "=> $1"; }
err()  { echo "ERROR: $1" >&2; exit 1; }

detect_os() {
    case "$(uname -s)" in
        Linux*)  echo "linux" ;;
        Darwin*) echo "macos" ;;
        *)       err "Unsupported platform: $(uname -s)" ;;
    esac
}

check_deps() {
    for cmd in cargo pnpm node; do
        command -v "$cmd" &>/dev/null || err "Missing dependency: $cmd"
    done
}

prepare_frontend() {
    cd "$DESKTOP_DIR"
    info "Installing frontend dependencies..."
    pnpm install
    info "Syncing SvelteKit..."
    pnpm exec svelte-kit sync
}

build_linux() {
    info "Building for Linux (AppImage + deb)..."
    prepare_frontend
    pnpm tauri build --bundles appimage,deb

    local bundle_dir="$DESKTOP_DIR/src-tauri/target/release/bundle"
    echo ""
    info "Build outputs:"
    ls -1 "$bundle_dir/appimage/"*.AppImage 2>/dev/null || true
    ls -1 "$bundle_dir/deb/"*.deb 2>/dev/null || true
}

build_macos() {
    info "Building for macOS (Universal Binary)..."
    rustup target add aarch64-apple-darwin x86_64-apple-darwin 2>/dev/null || true
    prepare_frontend
    pnpm tauri build --target universal-apple-darwin

    local bundle_dir="$DESKTOP_DIR/src-tauri/target/universal-apple-darwin/release/bundle"
    echo ""
    info "Build outputs:"
    ls -1 "$bundle_dir/dmg/"*.dmg 2>/dev/null || true
}

usage() {
    cat <<EOF
Usage: $0 [linux|macos]

Build the Amlich desktop app.

  linux   Build AppImage + deb packages
  macos   Build macOS universal binary (Intel + Apple Silicon)

If no argument is given, builds for the current platform.
EOF
}

main() {
    check_deps
    local cmd="${1:-$(detect_os)}"

    case "$cmd" in
        linux)
            [[ "$(detect_os)" == "linux" ]] || info "Warning: cross-compiling to Linux from $(detect_os)"
            build_linux
            ;;
        macos)
            [[ "$(detect_os)" == "macos" ]] || err "macOS builds must be done on macOS"
            build_macos
            ;;
        -h|--help)
            usage
            ;;
        *)
            err "Unknown option: $cmd. Use 'linux', 'macos', or '--help'."
            ;;
    esac
}

main "$@"
