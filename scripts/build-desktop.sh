#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
DESKTOP_DIR="$ROOT_DIR/apps/desktop"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_header() {
    echo -e "\n${BLUE}═══════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}  $1${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}\n"
}

print_step() {
    echo -e "${GREEN}→${NC} $1"
}

print_warn() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

# Detect OS
detect_os() {
    case "$(uname -s)" in
        Linux*)  echo "linux" ;;
        Darwin*) echo "macos" ;;
        *)       echo "unknown" ;;
    esac
}

# Check dependencies
check_deps() {
    local missing=()
    
    if ! command -v cargo &> /dev/null; then
        missing+=("rust/cargo")
    fi
    
    if ! command -v pnpm &> /dev/null; then
        missing+=("pnpm")
    fi
    
    if ! command -v node &> /dev/null; then
        missing+=("node")
    fi
    
    if [ ${#missing[@]} -ne 0 ]; then
        print_error "Missing dependencies: ${missing[*]}"
        exit 1
    fi
}

# Build for Linux
build_linux() {
    print_header "Building for Linux (AppImage, deb)"
    
    cd "$DESKTOP_DIR"
    
    print_step "Installing frontend dependencies..."
    pnpm install
    
    print_step "Syncing SvelteKit..."
    pnpm exec svelte-kit sync
    
    print_step "Building Tauri app for Linux..."
    pnpm tauri build
    
    local bundle_dir="$DESKTOP_DIR/src-tauri/target/release/bundle"
    
    echo ""
    print_step "Build complete! Outputs:"
    
    if [ -d "$bundle_dir/appimage" ]; then
        echo "  AppImage: $bundle_dir/appimage/"
        ls -1 "$bundle_dir/appimage/"*.AppImage 2>/dev/null || true
    fi
    
    if [ -d "$bundle_dir/deb" ]; then
        echo "  Deb:      $bundle_dir/deb/"
        ls -1 "$bundle_dir/deb/"*.deb 2>/dev/null || true
    fi
    
    if [ -d "$bundle_dir/rpm" ]; then
        echo "  RPM:      $bundle_dir/rpm/"
        ls -1 "$bundle_dir/rpm/"*.rpm 2>/dev/null || true
    fi
}

# Build for macOS (Universal)
build_macos() {
    print_header "Building for macOS (Universal Binary)"
    
    # Check/add macOS targets
    print_step "Checking Rust targets..."
    if ! rustup target list --installed | grep -q "aarch64-apple-darwin"; then
        print_step "Adding aarch64-apple-darwin target..."
        rustup target add aarch64-apple-darwin
    fi
    
    if ! rustup target list --installed | grep -q "x86_64-apple-darwin"; then
        print_step "Adding x86_64-apple-darwin target..."
        rustup target add x86_64-apple-darwin
    fi
    
    cd "$DESKTOP_DIR"
    
    print_step "Installing frontend dependencies..."
    pnpm install
    
    print_step "Syncing SvelteKit..."
    pnpm exec svelte-kit sync
    
    print_step "Building Tauri app for macOS Universal..."
    pnpm tauri build --target universal-apple-darwin
    
    local bundle_dir="$DESKTOP_DIR/src-tauri/target/universal-apple-darwin/release/bundle"
    
    echo ""
    print_step "Build complete! Outputs:"
    
    if [ -d "$bundle_dir/macos" ]; then
        echo "  App:  $bundle_dir/macos/"
        ls -1 "$bundle_dir/macos/"*.app 2>/dev/null || true
    fi
    
    if [ -d "$bundle_dir/dmg" ]; then
        echo "  DMG:  $bundle_dir/dmg/"
        ls -1 "$bundle_dir/dmg/"*.dmg 2>/dev/null || true
    fi
}

# Build for specific target
build_target() {
    local target="$1"
    
    print_header "Building for target: $target"
    
    cd "$DESKTOP_DIR"
    
    print_step "Installing frontend dependencies..."
    pnpm install
    
    print_step "Syncing SvelteKit..."
    pnpm exec svelte-kit sync
    
    print_step "Building Tauri app for $target..."
    pnpm tauri build --target "$target"
    
    echo ""
    print_step "Build complete!"
}

# Show usage
usage() {
    echo "Usage: $0 [OPTION]"
    echo ""
    echo "Build the Amlich desktop app for different platforms."
    echo ""
    echo "Options:"
    echo "  linux           Build for Linux (AppImage, deb, rpm)"
    echo "  macos           Build for macOS (Universal binary - Intel + Apple Silicon)"
    echo "  macos-arm       Build for macOS Apple Silicon only"
    echo "  macos-intel     Build for macOS Intel only"
    echo "  all             Build for current platform"
    echo "  --target <T>    Build for specific Rust target"
    echo "  -h, --help      Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 linux                    # Build Linux packages"
    echo "  $0 macos                    # Build macOS universal binary"
    echo "  $0 --target aarch64-apple-darwin"
    echo ""
    echo "Platform-specific dependencies:"
    echo ""
    echo "  Linux (Arch):"
    echo "    sudo pacman -S webkit2gtk-4.1 gtk3 libappindicator-gtk3 \\"
    echo "                   librsvg pango cairo gdk-pixbuf2"
    echo ""
    echo "  macOS:"
    echo "    Xcode Command Line Tools required"
    echo "    rustup target add aarch64-apple-darwin x86_64-apple-darwin"
}

# Main
main() {
    check_deps
    
    local os=$(detect_os)
    local cmd="${1:-}"
    
    case "$cmd" in
        linux)
            if [ "$os" != "linux" ]; then
                print_warn "Cross-compiling to Linux from $os may not work"
            fi
            build_linux
            ;;
        macos)
            if [ "$os" != "macos" ]; then
                print_error "macOS builds must be done on macOS"
                exit 1
            fi
            build_macos
            ;;
        macos-arm)
            if [ "$os" != "macos" ]; then
                print_error "macOS builds must be done on macOS"
                exit 1
            fi
            build_target "aarch64-apple-darwin"
            ;;
        macos-intel)
            if [ "$os" != "macos" ]; then
                print_error "macOS builds must be done on macOS"
                exit 1
            fi
            build_target "x86_64-apple-darwin"
            ;;
        all|"")
            case "$os" in
                linux)  build_linux ;;
                macos)  build_macos ;;
                *)      print_error "Unknown platform: $os"; exit 1 ;;
            esac
            ;;
        --target)
            if [ -z "${2:-}" ]; then
                print_error "Missing target argument"
                usage
                exit 1
            fi
            build_target "$2"
            ;;
        -h|--help)
            usage
            ;;
        *)
            print_error "Unknown option: $cmd"
            usage
            exit 1
            ;;
    esac
}

main "$@"
