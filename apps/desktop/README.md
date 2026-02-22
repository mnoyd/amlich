# Desktop App (`apps/desktop`)

This app is the desktop target for Amlich, built with Tauri (Rust backend) and Svelte.

## Prerequisites

- `pnpm`
- Rust toolchain (`rustup`, `cargo`)
- System dependencies required by Tauri/WebKit for your OS

## Commands

From repository root:

```bash
pnpm install
pnpm dev:app
pnpm build:app
```

App-local equivalents:

```bash
cd apps/desktop
pnpm tauri dev
pnpm tauri build
```

## Updater Note

Desktop updater integration logic lives in `apps/desktop/src/lib/updater.ts`.

## macOS Updater Recovery (Universal Build)

If an older Apple Silicon install shows an updater error like missing `darwin-aarch64` platform keys,
reinstall once from the latest GitHub Releases page using the universal mac build. After reinstalling,
future in-app updates should work again.

Safe cleanup (preserves app data/preferences):

```bash
./scripts/macos-cleanup-desktop.sh
```

This removes:
- `/Applications/AmLich.app` (if present)
- `~/Applications/AmLich.app` (if present)
- `~/Library/Caches/com.amlich.calendar`

It preserves:
- `~/Library/Application Support/com.amlich.calendar`
- `~/Library/Preferences/com.amlich.calendar*`

Full reset (removes app data too):

```bash
./scripts/macos-cleanup-desktop.sh full
```

For general setup and workspace commands, see `README.md`; for issues, use the repository issue tracker.
