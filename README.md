# Amlich - Vietnamese Lunar Calendar

Amlich is a monorepo for Vietnamese lunar calendar tooling across Rust, CLI, desktop, and JS/WASM consumers.
The shared domain model is centered on `amlich-api` DTOs and reused across runtime targets.

## Repository Layout

```text
amlich/
├── crates/
│   ├── amlich-core/       # Calendar math and domain calculations
│   ├── amlich-api/        # Stable DTO contract
│   ├── amlich-presenter/  # Presentation/text formatting
│   ├── amlich-cli/        # CLI + Waybar JSON output
│   ├── amlich-tui/        # Terminal UI app
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

# Build CLI binary
cargo build --release --package amlich-cli

# Run desktop app (dev)
pnpm dev:app

# Build WASM package
pnpm build:wasm
```

## Usage Entry Points

CLI examples:

```bash
amlich today
amlich date 2026-02-20
amlich json 2026-02-20
amlich toggle
```

Waybar integration: see `waybar/README.md`.

Desktop app details: see `apps/desktop/README.md`.

Shared data contract and validation: see `data/README.md`.

## License

MIT. See `LICENSE`.
