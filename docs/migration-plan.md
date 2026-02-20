# Core API Architecture Snapshot

## Architecture

- `amlich-core`: calendar math and domain calculations.
- `amlich-api`: stable DTO contract consumed by CLI, desktop, and WASM.
- `amlich-presenter`: presentation-only formatting logic.

## Current Data Flow

- CLI reads day and holiday data from `amlich-api`.
- Desktop Tauri backend reads day and holiday data from `amlich-api`.
- WASM serializes `amlich-api` DTOs and uses `amlich-presenter` for formatting.
- `packages/core/engine` prefers Rust-backed output through the CLI bridge and uses `legacy-engine.js` only as fallback.

## JSON Contract (`amlich json`)

`amlich json YYYY-MM-DD` emits canonical `DayInfoDto`-aligned output.

Top-level keys:

- `solar`
- `lunar`
- `jd`
- `canchi`
- `tiet_khi`
- `gio_hoang_dao`

Legacy projection keys are not part of the canonical Rust-path JSON contract.
