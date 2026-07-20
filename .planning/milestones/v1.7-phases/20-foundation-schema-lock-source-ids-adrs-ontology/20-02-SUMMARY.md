---
phase: 20-foundation-schema-lock-source-ids-adrs-ontology
plan: 02
subsystem: database
tags: [iching, kinh-dich, mai-hoa-dich-so, schema-lock, newtype, serde, crit-1, crit-3, king-wen, tien-thien, hau-thien]

# Dependency graph
requires:
  - phase: 10-foundation
    provides: Palace enum #[repr(u8)] + explicit-discriminant precedent + Palace::ALL static-array pattern (fengshui/types.rs:15-43)
  - phase: 17-rituals-crit3-source-id
    provides: DeferralMarker struct verbatim (almanac/fengshui/golden.rs:85-95) reused as HexagramEntry.pending_review
provides:
  - "Locked HexagramEntry struct with #[serde(deny_unknown_fields)] (CRIT-1 schema-lock-first gate)"
  - "Three CRIT-3-isolating newtypes: TienThienTrigram (Tiên Thiên 1..8), HauThienTrigram (Lo Shu 1..9 skipping 5), KingWenHexagram (1..=64) with NO cross-newtype From impl"
  - "64-entry bijective Tiên Thiên-pair → King Wen composition table + compose() bridge"
  - "Reserved data/iching/ corpus directory for Phase 21 ICH-01 authoring"
affects:
  - 21-iching-corpus-and-loader
  - 22-mai-hoa-casting-bien-que-the-dung
  - 24-iching-evaluator-semantic-graph-wiring-dto

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Three-distinct-newtype CRIT-3 isolation: no From impl between newtypes sharing a 1..N form; composition table is the ONLY bridge"
    - "TDD-RED via placeholder const array: stub COMPOSITION_TABLE with [(Kien, Kien); 64] to force bijectivity-test failure, then GREEN by replacing with real data"
    - "Additive Option<T> + #[serde(default, skip_serializing_if = \"Option::is_none\")] discipline for reserved *_en corpus translation fields"
    - "deny_unknown_fields as the CRIT-1 gate: field-name typos fail loudly during corpus authoring, not silently coerced"
    - "ExternalReviewPending(reason=\"...\"; expected_review_date=\"...\"; assigned_to=\"...\") free-text reviewer marker (mirrors Văn khấn corpus)"

key-files:
  created:
    - crates/amlich-core/src/iching/mod.rs
    - crates/amlich-core/src/iching/schema.rs
    - crates/amlich-core/tests/iching_schema_probe.rs
    - crates/amlich-core/data/iching/.gitkeep
  modified:
    - crates/amlich-core/src/lib.rs

key-decisions:
  - "KingWenHexagram is a pub struct(u8) newtype with const fn new(n) -> Option<Self>, NOT a 64-variant enum — 64 named variants is too verbose; the composition table carries the readable Tiên Thiên-pair → King Wen mapping"
  - "TienThienTrigram and HauThienTrigram are #[repr(u8)] enums with explicit discriminants + #[serde(rename_all = \"snake_case\)] + ALL static arrays, mirroring the Palace precedent verbatim (pattern reuse, NOT type reuse — reusing Palace directly would re-open CRIT-3)"
  - "HauThienTrigram encoding pinned to the exact Lo Shu palace numbers (Khảm=1, Khôn=2, Chấn=3, Tốn=4, Kiền=6, Đoài=7, Cấn=8, Ly=9 — skipping 5/center), matching Palace exactly per Pitfall 1"
  - "COMPOSITION_TABLE is a pub const [(TienThienTrigram, TienThienTrigram); 64] indexed by King Wen number (index 0 = #1), NOT a runtime-parsed JSON file — WASM-safe by construction, compile-checked, mirrors Palace::ALL"
  - "compose() uses a linear scan over the 64-entry table (premature to pre-compute a reverse map); panics on missing pair as a contract-violation signal (unreachable per bijectivity test)"
  - "HexagramEntry carries Hậu Thiên trigrams on upper_trigram/lower_trigram (display metadata, King Wen text tradition); TienThienTrigram does NOT appear on HexagramEntry — closes the CRIT-3 round-trip trap"
  - "Probe fixture is hexagram #2 Khôn (NOT #1 Kiền) — exercises the 7-hao_tu length rule (dụng lục seventh line) + NFC-sensitive diacritics + populated pending_review simultaneously"

patterns-established:
  - "TDD-RED via placeholder const array for any future bijectivity/correctness-proven static table (pattern: stub with sentinel values, write the contract test, see it fail, then fill in real data)"
  - "CRIT-3 newtype-isolation pattern: when two semantic spaces share a 1..N numeric form but mean different things, declare two distinct newtypes + a const composition table as the only bridge; no From impl"
  - "deny_unknown_fields + additive Option<T> as the schema-lock-first pair: locks the field set while preserving additive-safety for future translations"

requirements-completed: [FND-11]

# Metrics
duration: 7 min
completed: 2026-07-15
---

# Phase 20 Plan 02: IChing Schema Lock + Composition Table Summary

**Locked HexagramEntry schema with deny_unknown_fields + three CRIT-3-isolating newtypes (TienThienTrigram / HauThienTrigram / KingWenHexagram, no cross-From) + 64-entry bijective Tiên Thiên-pair → King Wen composition table, all proven by a passing 1-entry serde round-trip probe (hexagram #2 Khôn) BEFORE any of the 64 corpus entries are authored**

## Performance

- **Duration:** 7 min (475 s)
- **Started:** 2026-07-15T19:40:42Z
- **Completed:** 2026-07-15T19:48:37Z
- **Tasks:** 2 (both TDD)
- **Files modified:** 5 (4 created, 1 modified)

## Accomplishments

- **HexagramEntry schema locked** with `#[serde(deny_unknown_fields)]` — the CRIT-1 schema-lock-first gate is in place; Phase 21 corpus authoring cannot introduce silent field-name typos.
- **Three CRIT-3-isolating newtypes** declared with NO `impl From<...>` between them — the composition table is the ONLY bridge between Tiên Thiên pairs and the King Wen index space.
- **64-entry composition table** hand-authored from the classical King Wen sequence; bijectivity is proven by `composition_table_is_bijective` (64 distinct pairs + exhaustive 8×8 surjectivity loop).
- **1-entry serde round-trip probe** for hexagram #2 Khôn (7 hao_tu + NFC diacritics + DeferralMarker + reviewer free-text marker) — the trickiest schema case round-trips cleanly.
- **Reserved `data/iching/` corpus directory** for Phase 21 ICH-01 authoring.

## Task Commits

Each task was committed atomically (TDD: RED → GREEN):

1. **Task 1 RED: Failing bijectivity test** — `99efa74` (test)
   - Schema.rs + mod.rs + lib.rs registration + reserved corpus dir
   - Placeholder `[(Kien, Kien); 64]` table fails `composition_table_is_bijective`
2. **Task 1 GREEN: 64-entry King Wen composition table** — `da20ce3` (feat)
   - Replaced placeholder with hand-authored classical King Wen composition table
   - All 5 inline tests pass (bijectivity + 3 newtype serde stability + CRIT-3 distinct encodings)
3. **Task 2 GREEN: 1-entry serde round-trip probe** — `c35d8c8` (test)
   - 4 external probe tests in `tests/iching_schema_probe.rs`
   - RED phase collapsed (Task 1 GREEN already locked the schema; probe is regression-protection)

_The plan's per-task commit protocol produced 3 commits (TDD discipline: Task 1 split into RED + GREEN; Task 2's RED collapsed because Task 1 GREEN already implemented the locked HexagramEntry schema)._

## Files Created/Modified

- `crates/amlich-core/src/iching/mod.rs` — Module re-export surface (`pub use schema::{...}`); documents Phase 20/21/22/24 ownership split + CRIT-3 isolation invariant.
- `crates/amlich-core/src/iching/schema.rs` — Three newtypes + `COMPOSITION_TABLE` + `compose()` + `HexagramEntry` + 5 inline tests. The schema-lock centerpiece (455 lines).
- `crates/amlich-core/tests/iching_schema_probe.rs` — 1-entry serde round-trip probe + deny_unknown_fields rejection + reserved `*_en` absent→None + Hậu Thiên snake_case deserialise.
- `crates/amlich-core/data/iching/.gitkeep` — Reserved corpus directory for Phase 21 ICH-01.
- `crates/amlich-core/src/lib.rs` — One-line append: `pub mod iching;` (alphabetically positioned between `holidays` and `insight_data`).

## Decisions Made

- **KingWenHexagram is a struct newtype, NOT a 64-variant enum.** 64 named variants is too verbose to maintain ergonomically; the composition table already carries the readable Tiên Thiên-pair → King Wen mapping. Both shapes satisfy the "no From between them" CRIT-3 lock; the struct newtype is lighter. Per 20-RESEARCH.md Open Question #1 recommendation.
- **TienThienTrigram/HauThienTrigram reuse the Palace enum PATTERN but not the Palace type.** `#[repr(u8)]` + explicit discriminants + `#[serde(rename_all = "snake_case")]` + `ALL: [...; 8]` static array — pattern-level reuse only. Reusing `Palace` directly would re-open CRIT-3 by making `HauThienTrigram` interchangeable with a palace-layout descriptor.
- **HauThienTrigram encoding pinned to Lo Shu palace numbers (Pitfall 1).** Khảm=1, Khôn=2, Chấn=3, Tốn=4, Kiền=6, Đoài=7, Cấn=8, Ly=9 — exactly the assignment `Palace` uses, skipping 5/center. This pre-empts the vi.wikipedia sub-school variance that places Ly at 5.
- **COMPOSITION_TABLE is a `pub const [...]` array, NOT a runtime-parsed JSON file.** WASM-safe by construction (no `std::fs`, no `OnceLock`, no `serde_json::from_str` at load); compile-checked; mirrors the `Palace::ALL` precedent. The bijectivity test runs in `cargo test`, not at runtime.
- **`compose()` uses a linear scan (64 iterations), NOT a pre-computed reverse map.** Premature optimisation per 20-RESEARCH.md "Don't Hand-Roll" — branch-predictor-friendly, sub-microsecond, and the const array remains the single source of truth.
- **HexagramEntry's `upper_trigram`/`lower_trigram` are `HauThienTrigram`, NOT `TienThienTrigram`.** The corpus follows the King Wen text tradition (Ngô Tất Tố *Kinh Dịch Trọn Bộ*); displaying trigram numbers in Hậu Thiên (Lo Shu) numbering is consistent with that tradition and closes the CRIT-3 round-trip trap (a future maintainer cannot "round-trip" cast → corpus → re-compose).
- **Probe fixture is hexagram #2 Khôn (NOT #1 Kiền).** Per 20-RESEARCH.md Pitfall 5: Khôn exercises the 7-hao_tu length rule (the dụng lục seventh line) AND NFC-sensitive diacritics simultaneously, proving the schema handles the trickiest case before any of the 64 real entries exist.

## Deviations from Plan

None - plan executed exactly as written.

The two TDD tasks produced the expected 3-commit pattern (Task 1 RED + GREEN; Task 2 collapsed RED→GREEN). No deviation rules (1-4) were triggered. No authentication gates. No deferred issues.

## Issues Encountered

None.

The TDD reference's "RED doesn't fail → investigate" pattern fired once during Task 2: the probe test passed on first run because Task 1 GREEN (`da20ce3`) had already locked the `HexagramEntry` schema with all required properties (deny_unknown_fields, additive Option<T>, DeferralMarker reuse, reviewer free-text marker shape). Investigation per TDD error-handling guidance: "feature may already exist" — confirmed. The probe's purpose is regression-protection for Phase 21 corpus authoring (the literal CRIT-1 gate), not new-feature driving; the probe existing BEFORE Phase 21 IS the schema-lock-first discipline.

## User Setup Required

None - no external service configuration required. The plan adds no new dependencies (verified via `cargo tree -p amlich-core --depth 1`: chrono, serde, serde_json, unicode-normalization — all pre-existing).

## Next Phase Readiness

- **Schema is locked** — Phase 21 ICH-01 can author the 64 corpus entries against a frozen `HexagramEntry` shape. Any field-name typo will fail loudly via `deny_unknown_fields`.
- **Composition table is proven bijective** — Phase 22 ICH-02 Mai Hoa casting can rely on `compose(upper, lower) -> KingWenHexagram` to map cast results to corpus entries without panic.
- **CRIT-3 isolation is structurally enforced** — no `impl From<...>` between the three newtypes; the only bridge is the composition table. Verified by `rg "impl From<(TienThienTrigram|HauThienTrigram|KingWenHexagram)> for ..." crates/amlich-core/src/iching/` returning zero matches.
- **No blockers.** Plan 20-03 (FND-12 ontology extension for `NodeConcept::Hexagram` + `LocatedAt` + `Transforms`) is independent of this plan and can proceed in parallel.

---
*Phase: 20-foundation-schema-lock-source-ids-adrs-ontology*
*Completed: 2026-07-15*

## Self-Check: PASSED

- All 4 created files exist on disk (mod.rs, schema.rs, iching_schema_probe.rs at 220 lines >= 40-line minimum, .gitkeep).
- Modified file lib.rs contains `pub mod iching;`.
- All 3 task commits exist (99efa74, da20ce3, c35d8c8).
- All must_have artifact grep assertions pass: `deny_unknown_fields` in schema.rs; `pub use` in mod.rs; `pub mod iching` in lib.rs; `use amlich_core::iching` in probe; `use crate::almanac::fengshui::golden::DeferralMarker` in schema.rs.
