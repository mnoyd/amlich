# Core API Migration and Deprecation Plan

## Target architecture
- `amlich-core`: pure calendar math and domain calculations only.
- `amlich-api`: stable DTO contract consumed by CLI, desktop, and WASM.
- `amlich-presenter`: text formatting and presentation-only output logic.

## Migration status
- CLI now reads day/holiday data from `amlich-api`.
- Desktop Tauri backend now reads day/holiday data from `amlich-api`.
- WASM now serializes `amlich-api` DTOs and uses `amlich-presenter` for text formatting.
- `packages/core/engine` now prefers Rust-backed output through the CLI bridge.

## JS engine deprecation
- Legacy JS algorithm remains at `packages/core/engine/legacy-engine.js` for compatibility fallback.
- New entrypoint `packages/core/engine/index.js` prefers Rust CLI (`amlich json`) and falls back to legacy only when unavailable.
- Rust-path output now maps from canonical `DayInfoDto` fields only; legacy-shape placeholders are no longer synthesized.
- Consumers should migrate to Rust/WASM direct bindings where possible.

## Canonical JSON contract
- `amlich json YYYY-MM-DD` now emits canonical `DayInfoDto` shape directly.
- Top-level keys are: `solar`, `lunar`, `jd`, `canchi`, `tiet_khi`, `gio_hoang_dao`.
- No legacy projection keys (`day_can`, `year_can`, synthetic `_meta.methods`, etc.) are emitted in the Rust path.

## Next removals
1. Remove legacy fallback after downstream consumers guarantee Rust CLI/WASM availability.
2. Add Node-target WASM package for direct Rust-backed JS without CLI spawning.
