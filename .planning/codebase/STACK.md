# Technology Stack

**Analysis Date:** 2026-02-28

## Languages

**Primary:**
- Rust 1.93.0 - Core calculation engine, CLI, TUI, WASM bindings, desktop app backend

**Secondary:**
- JavaScript (Node.js >=12.0.0) - Shared packages, validation scripts
- TypeScript 5.6.2 - Desktop app frontend

## Runtime

**Environment:**
- Rust 1.93.0 (stable channel)
- Node.js v20.14.0 (via `.nvmrc` in `apps/desktop/.nvmrc`)
- pnpm 10.12.4 - JavaScript package manager

**Package Manager:**
- Cargo 1.93.0 - Rust workspace
- pnpm 10.12.4 - JavaScript monorepo
- Lockfiles: `Cargo.lock`, `pnpm-lock.yaml`

## Frameworks

**Core:**
- Tauri 2.10.0 - Desktop app framework (Rust + WebView)
- SvelteKit 2.50.2 - Desktop app frontend framework
- Svelte 5.50.0 - UI component framework
- Vite 6.4.1 - Frontend build tool and dev server
- ratatui 0.30 - Terminal UI framework for CLI TUI
- crossterm 0.29 - Cross-platform terminal manipulation
- clap 4.5 - CLI argument parsing
- wasm-bindgen 0.2 - Rust ↔ WASM bindings
- wasm-pack - WASM build tool

**Testing:**
- Rust built-in `cargo test`
- wasm-bindgen-test 0.3 - WASM unit tests

**Build/Dev:**
- just 1.0.0+ - Command runner (via `justfile`)
- tauri-build 2 - Tauri build dependencies
- esbuild - Fast bundler (via Vite)
- lightningcss - CSS minification

## Key Dependencies

**Critical:**
- chrono 0.4 - Date/time calculations
- serde 1.0 - Serialization framework (derive feature enabled)
- serde_json 1.0 - JSON serialization
- serde-wasm-bindgen 0.6 - WASM serde bridge

**CLI/TUI:**
- dirs 5.0 - Standard directories (config, cache, home)
- deunicode 1.6.2 - Unicode diacritic removal

**Desktop App:**
- @tauri-apps/api 2.10.1 - Tauri frontend API
- @tauri-apps/plugin-dialog 2.6.0 - Native dialogs
- @tauri-apps/plugin-opener 2.5.3 - Open URLs in default browser
- @tauri-apps/plugin-process 2.3.1 - Process control
- @tauri-apps/plugin-updater 2.10.0 - Auto-update mechanism
- @sveltejs/adapter-static 3.0.10 - Static site adapter for SPA mode
- @sveltejs/vite-plugin-svelte 6.2.4 - Svelte Vite plugin

**Infrastructure:**
- None - All data is vendored JSON in `data/` and `crates/amlich-core/data/`

## Configuration

**Environment:**
- No `.env` files detected
- Configuration via command-line args (CLI) and Tauri APIs (desktop)
- Waybar integration via `waybar/` config snippets

**Build:**
- `Cargo.toml` - Workspace configuration with shared dependencies
- `justfile` - Development commands (build, test, install, clean)
- `tsconfig.json` - TypeScript config (extends SvelteKit defaults)
- `svelte.config.js` - SvelteKit adapter configuration (static SPA)
- `vite.config.js` - Vite build config with manual chunks for calendar data
- `tauri.conf.json` - Desktop app bundling, updater, and window config

**Rust Toolchain:**
- Edition 2021
- Workspace members defined in root `Cargo.toml`
- Release profile: opt-level 3, LTO enabled, single codegen unit, stripped symbols

## Platform Requirements

**Development:**
- Rust 1.93.0+ (stable)
- Node.js 20+ (via `.nvmrc` in `apps/desktop`)
- pnpm 9+ (for workspace management)
- wasm-pack (for WASM builds)
- just (optional, for command runner)
- Linux desktop build: `libwebkit2gtk-4.1-dev`, `librsvg2-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`

**Production:**
- CLI: Standalone binary (Linux, macOS, Windows, universal macOS)
- Desktop: Tauri bundles (AppImage, deb, dmg, NSIS installer)
- WASM: Browser with WebAssembly support
- Waybar: Linux with Waybar installed
- macOS minimum: 10.15 (Catalina)

---

*Stack analysis: 2026-02-28*
