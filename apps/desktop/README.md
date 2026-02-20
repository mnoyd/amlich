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

For general setup and workspace commands, see `README.md`; for issues, use the repository issue tracker.
