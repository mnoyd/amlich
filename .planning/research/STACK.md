# Stack Research

**Domain:** Rust almanac engine extension for Thập Thần + Tử Mệnh/Kua
**Researched:** 2026-03-02
**Confidence:** HIGH

## Recommended Stack

### Core Technologies

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| Rust workspace (`edition = 2021`) | existing workspace baseline | Implement deterministic Ten Gods + Kua logic in `amlich-core` | Already the project’s correctness-critical execution layer; no FFI/language boundary needed for these rule engines. |
| `serde` | 1.0 (already in workspace) | Serialize/deserialize new typed outputs + fixtures | Existing DTO/JSON contract already depends on serde; adding Kua/Thập Thần fields is a straightforward extension of current patterns. |
| `serde_json` | 1.0 (already in workspace) | Fixture loading and contract/golden-style testing | Existing test strategy already uses JSON fixtures and `include_str!`-style embedding; reuse keeps tests consistent and reviewable. |
| `chrono` | 0.4 (already in workspace) | Birth-year/date boundary normalization for TM-02 | Already available if year-boundary handling needs explicit date reasoning; no need for additional date crates. |

### Supporting Libraries

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| None required | — | No new crate needed for v1.2 scope | Use standard Rust + existing workspace dependencies only. |

### Development Tools

| Tool | Purpose | Notes |
|------|---------|-------|
| `cargo test --package amlich-core` | Regression + new module correctness | Mandatory gate for INT-06 and KHCBPPT non-regression. |
| `cargo test --package amlich-api` | API/DTO contract safety after schema extensions | Ensures new optional fields remain backward-compatible for callers. |

## Stack Additions/Changes for v1.2

### 1) `amlich-core` module strategy (additions, not dependencies)
- Keep Thập Thần in `almanac/thap_than.rs` as pure deterministic mapping (already in place).
- Add Kua/Tử Mệnh as a sibling module (recommended: `almanac/tu_menh.rs`) with typed inputs/outputs in `almanac/types.rs`.
- Keep rule evidence metadata aligned with existing `RuleEvidence`/`SourceMeta` conventions.

### 2) Public API integration points
- `amlich-core/src/lib.rs`: export Kua API similarly to `get_thap_than` (typed, serialization-safe).
- `amlich-api/src/dto.rs`: add optional DTO fields for Ten Gods + Kua to avoid breaking existing consumers.
- `amlich-api/src/convert.rs`: add `From` conversions for new types; preserve stable snake_case JSON tokens.
- `amlich-wasm/src/lib.rs`: no new crate; piggyback on existing `DayInfoDto` JSON bridge.

### 3) Data/fixture strategy
- Prefer JSON fixtures under existing test paths (`crates/amlich-api/tests/fixtures/` and/or `crates/amlich-core/data/almanac/`).
- Use `include_str!` + `serde_json` for deterministic, versioned fixture execution.
- For TM-04 (1900–2099 representative years), add dedicated fixture file rather than hard-coding large tables in Rust source.

### 4) Testing/tooling implications
- Keep matrix-style deterministic tests (existing Thập Thần 10x10 pattern is the right model).
- Add Kua fixture-driven tests for:
  - gender split,
  - century/year boundary behavior,
  - direction group and favorable/unfavorable sets.
- Add API contract tests to verify new fields are present when requested and absent/optional-compatible for older clients.

## Alternatives Considered

| Recommended | Alternative | When to Use Alternative |
|-------------|-------------|-------------------------|
| No new dependencies | Add enum/helper crates (`strum`, etc.) | Only if enum/string conversion logic becomes duplicated and error-prone across multiple modules (not currently true). |
| Existing JSON fixtures + serde_json | New fixture/assertion frameworks (`insta`, `rstest`) | Only if test authoring overhead becomes dominant; today’s tests are simple and readable with std + serde_json. |
| Deterministic rule code in core | External rule engine / scripting | Not recommended for v1.2; only consider if future milestones require end-user editable rules at runtime. |

## What NOT to Add

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| New date-time crates (`time`, timezone DB crates) | Scope is year/gender-based Kua with deterministic conventions; extra date stack increases maintenance risk. | `chrono` already present + explicit boundary docs/tests. |
| Runtime rule engines / embedded scripting | Adds non-deterministic surface and weakens auditability for correctness milestone. | Compile-time Rust mapping logic + fixture evidence. |
| Heavy numeric/scientific libs | Thập Thần/Kua are table/formula driven, not numeric heavy. | Plain Rust functions and lookup tables. |
| Non-optional breaking DTO changes | Would violate INT-02/INT-05 backward compatibility goals. | Add optional fields and preserve existing field semantics. |

## Version Compatibility

| Package A | Compatible With | Notes |
|-----------|-----------------|-------|
| `serde` 1.0 | `serde_json` 1.0 | Existing workspace baseline; sufficient for all new serialization needs. |
| `amlich-core` 0.1.2 workspace | `amlich-api` / `amlich-wasm` path deps | Current architecture already wired for type-forward integration from core to API/WASM. |

## Recommendation Summary

For v1.2, **do not add any new dependencies**. The required capabilities (typed models, deterministic mapping, JSON fixtures, contract tests, API propagation) are fully covered by the existing Rust + serde/serde_json + chrono stack already in this workspace.

This minimizes regression risk in a correctness milestone and keeps Ten Gods/Kua logic auditable, deterministic, and consistent with established `amlich-core` and `amlich-api` patterns.

## Sources

- Workspace dependency baseline: `/home/noy/Work/junks/amlich/Cargo.toml` (HIGH)
- Core crate dependencies/modules: `/home/noy/Work/junks/amlich/crates/amlich-core/Cargo.toml`, `src/almanac/mod.rs`, `src/almanac/types.rs`, `src/almanac/thap_than.rs`, `src/almanac/calc.rs` (HIGH)
- API/DTO integration surface: `/home/noy/Work/junks/amlich/crates/amlich-api/src/dto.rs`, `src/convert.rs`, `src/lib.rs` (HIGH)
- Existing fixture/test patterns: `/home/noy/Work/junks/amlich/crates/amlich-api/tests/golden_parity.rs`, `tests/fixtures/day-info-golden.json`, `/home/noy/Work/junks/amlich/crates/amlich-core/src/almanac/golden_loader.rs` (HIGH)
- Milestone scope/requirements: `/home/noy/Work/junks/amlich/.planning/PROJECT.md`, `.planning/ROADMAP.md`, `.planning/REQUIREMENTS-v1.2.md` (HIGH)

---
*Stack research for: v1.2 Ten Gods and Kua Foundation*
*Researched: 2026-03-02*
