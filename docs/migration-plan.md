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
- Consumers should migrate to Rust/WASM direct bindings where possible.

## Next removals
1. Remove legacy fallback after downstream consumers guarantee Rust CLI/WASM availability.
2. Add Node-target WASM package for direct Rust-backed JS without CLI spawning.
3. Remove duplicate metadata assumptions in JS (`_meta.methods`) once callers no longer depend on that shape.
