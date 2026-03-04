# Amlich - Vietnamese Lunar Calendar

Amlich is a monorepo for Vietnamese lunar calendar tooling across Rust, CLI, desktop, and JS/WASM consumers.
The shared domain model is centered on `amlich-api` DTOs and reused across runtime targets.

## Repository Layout

```text
amlich/
├── crates/
│   ├── amlich/           # Unified binary (TTY-aware TUI + headless CLI)
│   ├── amlich-core/       # Calendar math and domain calculations
│   ├── amlich-api/        # Stable DTO contract
│   └── amlich-wasm/       # WASM bindings
├── apps/
│   └── desktop/           # Tauri + Svelte desktop app
├── packages/
│   └── core/              # JavaScript package
├── data/                  # Shared static datasets and schemas
└── waybar/                # Waybar integration assets
```

## Quick Start

Run from repository root.

```bash
pnpm install

# Build Rust workspace
cargo build --release --workspace

# Run tests
cargo test --workspace
pnpm test

# Build CLI binary (package `amlich-cli`, command `amlich`)
cargo build --release --package amlich-cli

# Run desktop app (dev)
pnpm dev:app

# Build WASM package
pnpm build:wasm
```

## Usage Entry Points

CLI examples:

```bash
amlich               # TTY => TUI, non-TTY => Waybar JSON
amlich tui --date 2026-02-20
amlich day 2026-02-20 --format json --pretty
amlich day --format waybar
amlich range --start 2026-02-20 --end 2026-02-24 --format ndjson
amlich convert solar-to-lunar 2026-02-20 --format text
amlich lookup na-am --index 1 --format text
amlich lookup ten-gods --day-can Giáp --target-can Ất --format json
amlich lookup kua --birth-year 1990 --gender male --format json
amlich config mode toggle
```

Waybar integration: see `waybar/README.md`.

Desktop app details: see `apps/desktop/README.md`.

Shared data contract and validation: see `data/README.md`.

## CLI Migration

Old commands were replaced by explicit subcommands:

```bash
amlich-tui                      -> amlich tui
amlich today                    -> amlich day
amlich date 2026-02-20          -> amlich day 2026-02-20
amlich json 2026-02-20          -> amlich day 2026-02-20 --format json --pretty
amlich mode                     -> amlich config mode show
amlich set-mode minimal         -> amlich config mode set minimal
amlich toggle                   -> amlich config mode toggle
```

Compatibility note:

- `amlich query` is still available as a deprecated alias and prints a migration warning.

## License

MIT. See `LICENSE`.
