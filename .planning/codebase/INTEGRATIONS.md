# External Integrations

**Analysis Date:** 2026-02-28

## APIs & External Services

**Release Distribution:**
- GitHub Releases - Binary distribution
  - Updater endpoint: `https://github.com/mnoyd/amlich/releases/latest/download/latest.json`
  - Config: `apps/desktop/src-tauri/tauri.conf.json` (plugins.updater.endpoints)
  - Auth: GitHub token (injected via `GITHUB_TOKEN` in CI)
  - Purpose: Desktop app auto-update mechanism

**CI/CD:**
- GitHub Actions - Build and test automation
  - Workflows: `.github/workflows/ci.yml`, `.github/workflows/release.yml`, `.github/workflows/opencode.yml`
  - Platforms: ubuntu-22.04, macos-latest, windows-latest
  - Artifacts: AppImage, deb, dmg, NSIS, CLI binaries

## Data Storage

**Databases:**
- None - All calendar data is vendored JSON files

**File Storage:**
- Local filesystem only - No cloud storage

**Static Data Assets:**
- `data/canchi.json` - Can Chi (Heavenly Stems & Earthly Branches) reference data
- `data/tiet-khi.json` - 24 Solar Terms (Tiết Khí) data with weather guidance
- `data/holidays/lunar-festivals.json` - Vietnamese lunar festival dates
- `data/holidays/solar-holidays.json` - Vietnamese solar holiday dates
- `data/schemas/` - JSON schemas for data validation
- `crates/amlich-core/data/` - Vendored copies of above (synced via `scripts/check-core-data-sync.sh`)

**Caching:**
- None - All calculations are performed in-memory

## Authentication & Identity

**Auth Provider:**
- None - Application is fully local with no user accounts

**Package Publishing:**
- crates.io - Potential Rust crate publishing (via `scripts/publish-crates.sh`)
- npm - Potential JS package publishing (via `scripts/publish-npm.sh`)
  - Packages: `@amlich/core`, `@amlich/wasm`

## Monitoring & Observability

**Error Tracking:**
- None - No external error reporting service

**Logs:**
- stdout/stderr only (CLI output)
- No centralized logging

**Analytics:**
- None - No usage tracking or analytics

## CI/CD & Deployment

**Hosting:**
- GitHub Releases - Primary distribution channel
  - Repository: https://github.com/mnoyd/amlich
  - Release assets: Multi-platform binaries and installers

**Desktop App Signing:**
- Tauri code signing (enabled in `tauri.conf.json`)
  - Env vars: `TAURI_SIGNING_PRIVATE_KEY`, `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`
  - Platform support: macOS, Windows
  - Updater: Minimal signing pubkey embedded in config

**Build Platforms:**
- GitHub Actions runners
  - Linux (ubuntu-22.04) - AppImage, deb
  - macOS (macos-latest) - Universal dmg (Intel + Apple Silicon)
  - Windows (windows-latest) - NSIS installer

**Package Registries:**
- npm (planned) - `@amlich/core`, `@amlich/wasm`
- crates.io (planned) - `amlich`, `amlich-core`, `amlich-api`, `amlich-wasm`

## Environment Configuration

**Required env vars:**
- None for runtime
- CI/CD only:
  - `GITHUB_TOKEN` - GitHub release creation
  - `TAURI_SIGNING_PRIVATE_KEY` - Desktop app signing
  - `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` - Desktop app signing
  - `TAURI_DEV_HOST` - Optional: custom dev host for Vite HMR

**Secrets location:**
- GitHub Secrets repository settings
  - `TAURI_SIGNING_PRIVATE_KEY`
  - `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`

## Webhooks & Callbacks

**Incoming:**
- None

**Outgoing:**
- GitHub Releases API (via updater plugin)
  - Endpoint: `https://github.com/mnoyd/amlich/releases/latest/download/latest.json`
  - Trigger: Desktop app startup (checks for updates)
  - Method: HTTP GET (via Tauri updater plugin)

## System Integrations

**Waybar (Linux):**
- Waybar module integration
  - Config: `waybar/modules/amlich.jsonc`
  - Styles: `waybar/styles/amlich.css`
  - Binary: `amlich` CLI (outputs JSON for Waybar)
  - Purpose: Display lunar calendar in Linux status bar

**File System:**
- Config directory (via `dirs` crate):
  - Linux: `~/.config/amlich/`
  - macOS: `~/Library/Application Support/amlich/`
  - Windows: `%APPDATA%\amlich\`
- Purpose: User preferences for CLI mode

---

*Integration audit: 2026-02-28*
