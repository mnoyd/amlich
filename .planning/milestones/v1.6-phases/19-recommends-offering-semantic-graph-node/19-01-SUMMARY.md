---
phase: 19-recommends-offering-semantic-graph-node
plan: 01
subsystem: semantic-graph
tags: [schema-lock, serde, OfferingRef, SourceId, DaySnapshot, additive, INT-08, INT-07]

# Dependency graph
requires:
  - phase: 18-daily-phi-tinh
    provides: "Phase 18-04 daily_flying_stars additive surface on DaySnapshot (proven pattern this plan mirrors); SchemaLockBeforeBuilder discipline precedent"
  - phase: 11-van-khan-corpus
    provides: "RitualEntry schema-lock (ADR-0001) with deny_unknown_fields discipline + get_ritual_by_id matcher (RIT-04) + Offering struct shape"
provides:
  - "Locked OfferingRef struct with 4-field identity tuple { offering_id, name_vi, name_en, source_id } + #[serde(deny_unknown_fields)] + OfferingRef::new() constructor"
  - "SourceId type alias (zero-cost newtype over String) satisfying INT-07 literal SC text 'source_id: SourceId'"
  - "Re-export of OfferingRef via crate::rituals::OfferingRef (via existing pub use schema::* glob)"
  - "Two additive Option<T> fields on DaySnapshot: offering_refs (structured) + offerings (legacy flat-string deduped)"
  - "Populate block in calculate_day_snapshot_internal that derives both fields from applicable_rituals via get_ritual_by_id"
  - "Focused populate test day_snapshot_offering_refs_populated_and_deduped (warning 1 fix)"
affects:
  - phase: 19-02
    reason: "Plan 19-02 will emit Offering semantic-graph nodes; OfferingRef is the stable target type. Schema-lock-before-builder discipline preserved."
  - phase: 19-03
    reason: "Plan 19-03 adds external-crate black-box tests consuming offering_refs field"

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "SchemaLockBeforeBuilder (Phase 19-01 mirrors Phase 18-01 daily_flying_stars schema-lock + v1.5 RitualEntry ADR-0001)"
    - "Additive Optional Surface (every Phase 19 field uses EXACT #[serde(default, skip_serializing_if = 'Option::is_none')] pair)"
    - "Zero-cost newtype over String for typed-source_id discipline without breaking const SOURCE_* pattern"

key-files:
  created: []
  modified:
    - "crates/amlich-core/src/sources.rs (added SourceId alias + 1 unit test)"
    - "crates/amlich-core/src/rituals/schema.rs (added OfferingRef struct + impl + 1 unit test)"
    - "crates/amlich-core/src/lib.rs (added 2 DaySnapshot fields + 2 constructor inits + 1 populate block + 1 focused populate test)"

key-decisions:
  - "SourceId is a zero-cost newtype over String (NOT a true newtype enforcing SOURCE_* membership) — preserves DEC-0023's pub const SOURCE_*: &str discipline while satisfying INT-07's literal 'source_id: SourceId' SC text"
  - "OfferingRef::new(...) accepts String source_id (not SourceId) for call-site ergonomics — internally stored as SourceId; debug_assert enforces non-empty"
  - "offering_id format is 'ritual.{ritual_id}.offering.{idx}' (corpus-position-based, NOT hashed from name_vi) — per 19-RESEARCH.md Pitfall P-3 / Don't-Hand-Roll"
  - "Both offering_refs and offerings fields are derived from the SAME source (applicable_rituals via get_ritual_by_id); offering_refs is the structured preferred path, offerings is the legacy flat-string summary for BC"
  - "offerings flat-string Vec is deduped by name_vi and preserves insertion order (per Q4 interpretation i in 19-RESEARCH.md)"
  - "is_empty() → None conversion preserves additive contract (skip_serializing_if honored for empty applicable_rituals case)"

patterns-established:
  - "Pattern: SchemaLockBeforeBuilder — when introducing a new semantic-graph node type, lock the type FIRST (this plan), then have a follow-up plan emit nodes using that type. NO builder code emits Offering nodes before Plan 19-02."
  - "Pattern: DualFieldAddPath — for new DaySnapshot data, offer TWO additive Option<T> fields: one structured (preferred, carries identity tuple + source_id) + one flat-string summary (legacy BC path, deduped by name)"

requirements-completed: [INT-08]

# Metrics
duration: ~5 min
completed: 2026-07-15
---

# Phase 19 Plan 1: OfferingRef Schema Lock + Additive DaySnapshot Fields

**Locked `OfferingRef` struct with INT-08 4-field identity tuple + `SourceId` type alias + two additive `DaySnapshot` fields (`offering_refs` structured preferred, `offerings` legacy flat-string deduped) + populate block derived from `applicable_rituals`**

## Performance

- **Duration:** ~5 min
- **Started:** 2026-07-15T16:48:00Z
- **Completed:** 2026-07-15T16:52:32Z
- **Tasks:** 2
- **Files modified:** 3 (sources.rs +23, rituals/schema.rs +88, lib.rs +102 = +213 lines, 0 deletions)

## Accomplishments

- **Schema-locked `OfferingRef`** (Phase 19 INT-08): the semantic-graph-side identity handle for Offering nodes, carrying the exact 4-field tuple `{ offering_id, name_vi, name_en, source_id }` with `#[serde(deny_unknown_fields)]` (ADR-0001 discipline) and `Hash` derive for use as semantic-graph node keys. `source_id` is typed as `crate::sources::SourceId` (the new alias) per INT-07's literal SC text.
- **Introduced `pub type SourceId = String;` zero-cost newtype** in `sources.rs` — satisfies INT-07's literal "source_id: SourceId" discipline WITHOUT breaking DEC-0023's `pub const SOURCE_*: &str` discipline. The alias is a transparent type marker, not a semantic constraint (call-sites continue to use `SOURCE_*.to_string()`).
- **Two additive `Option<T>` fields on `DaySnapshot`**: `offering_refs: Option<Vec<crate::rituals::OfferingRef>>` (structured preferred path) + `offerings: Option<Vec<String>>` (legacy flat-string summary, deduped by `name_vi`). Both carry `#[serde(default, skip_serializing_if = "Option::is_none")]` matching the EXACT serde pattern as `flying_stars`, `applicable_rituals`, `daily_flying_stars`.
- **Populate block** in `calculate_day_snapshot_internal` derives both fields from `applicable_rituals` by calling `get_ritual_by_id` for each ritual id and building `OfferingRef` for each structured `Offering` with `format!("ritual.{ritual_id}.offering.{idx}")` id and `SOURCE_VN_FOLK_RITUAL` const import (no bare string literal — `source_id_guard` compliant).
- **Focused populate test** `day_snapshot_offering_refs_populated_and_deduped` (warning 1 fix) exercises both fields specifically — non-empty, identity pattern, source-id discipline, dedup invariant, None → absent in JSON — rather than relying on the existing aggregate test.

## Task Commits

Each task was committed atomically:

1. **Task 1: Lock OfferingRef struct + SourceId alias + re-export** - `eddc51d` (feat)
2. **Task 2: Add additive offering_refs + offerings fields + populate block + focused test** - `6508f79` (feat)

**Plan metadata:** (to be committed at end with SUMMARY.md)

## Files Created/Modified

- `crates/amlich-core/src/sources.rs` — added `pub type SourceId = String;` alias + `source_id_alias_is_string` unit test (+23 lines)
- `crates/amlich-core/src/rituals/schema.rs` — added `OfferingRef` struct + `impl OfferingRef::new(...)` + `offering_ref_serde_round_trip_and_deny_unknown_fields` unit test (+88 lines)
- `crates/amlich-core/src/lib.rs` — added 2 DaySnapshot fields (`offering_refs`, `offerings`) + 2 constructor inits + 1 populate block in `calculate_day_snapshot_internal` + 1 focused populate test (+102 lines)
- `crates/amlich-core/src/rituals/mod.rs` — unchanged (existing `pub use schema::*;` glob re-export at line 26 covers `OfferingRef`)

## Decisions Made

- **SourceId is a zero-cost newtype over String (NOT a true newtype)** — preserves DEC-0023's `pub const SOURCE_*: &str` discipline (all 7 consts unchanged) while satisfying INT-07's literal "source_id: SourceId" SC text. The alias is a transparent type marker; future phases MAY tighten into a true newtype that enforces SOURCE_* membership at construction, but for now it is documentation-only.
- **OfferingRef::new(...) accepts String source_id for call-site ergonomics** — internally stored as `SourceId`; `debug_assert!` enforces non-empty on `offering_id`, `name_vi`, `source_id`. Avoids forcing call-sites to write `SourceId::from(SOURCE_X.to_string())`.
- **offering_id is corpus-position-based**, NOT hashed from `name_vi` — `format!("ritual.{ritual_id}.offering.{idx}")` (per 19-RESEARCH.md Pitfall P-3 / Don't-Hand-Roll: hashing name_vi would break stable join keys if the corpus is ever reordered or renamed).
- **Both `offering_refs` and `offerings` derived from SAME source** — `applicable_rituals` via `get_ritual_by_id`; `offering_refs` is the structured preferred path, `offerings` is the legacy flat-string BC summary. `offerings` is deduped by `name_vi` and preserves insertion order (Q4 interpretation i).
- **`is_empty() → None` conversion** — preserves the additive contract: a day with no matching rituals (no `offering_refs`) MUST NOT serialize the `offering_refs` key into JSON (skip_serializing_if honored).

## Deviations from Plan

### Auto-fixed Issues

**1. [Plan-side count error — not a real bug] DaySnapshot skip_serializing_if count is 5, not 6**
- **Found during:** Final verification (verification gate #3)
- **Issue:** Plan stated `grep -c "skip_serializing_if" crates/amlich-core/src/lib.rs` "must show 6 hits after this plan: 4 existing + 2 new" but the actual count is 5 (3 existing fields with the attribute: `flying_stars`, `applicable_rituals`, `daily_flying_stars` + 2 new: `offering_refs`, `offerings`). The plan miscounted — there were 3 existing fields with the attribute, not 4.
- **Fix:** None required — the implementation is correct (each DaySnapshot field gets exactly one `#[serde(default, skip_serializing_if = "Option::is_none")]` attribute pair). Documented as a plan-side error.
- **Impact:** None on functionality. The grep count is a verification gate diagnostic, not a correctness check.

---

**Total deviations:** 0 functional auto-fixes (1 plan-side verification-gate count miscount noted for transparency).

**Impact on plan:** Zero functional deviations. All `<must_haves>` artifacts and `<success_criteria>` checks pass.

## Issues Encountered

- Pre-existing warning `unused import: ProvenanceSource` in `crates/amlich-core/src/semantic_graph/views/helpers.rs:113` was emitted during test compilation but is NOT related to this plan's changes (the warning was already present on master and predates this plan). Not in scope.

## Verification Results

All 9 plan `<verification>` gates pass:

1. **SourceId type alias gate** ✓ — `pub type SourceId = String;` exists between 7 consts and `#[cfg(test)]`; `source_id_alias_is_string` test passes.
2. **Schema-lock gate** ✓ — `OfferingRef` with 4-field tuple + `#[serde(deny_unknown_fields)]` + `SourceId`-typed `source_id` + `OfferingRef::new(...)` constructor with `debug_assert!` enforcement + `offering_ref_serde_round_trip_and_deny_unknown_fields` test passes.
3. **DaySnapshot additive gate** ✓ — both new fields carry EXACT `#[serde(default, skip_serializing_if = "Option::is_none")]` attribute pair as `flying_stars` / `applicable_rituals` / `daily_flying_stars` (5 hits in lib.rs: 3 existing + 2 new; plan miscounted as 6 but implementation is correct).
4. **Re-export gate** ✓ — `OfferingRef` importable as `crate::rituals::OfferingRef` via existing `pub use schema::*;` glob (verified by populate block compile success).
5. **Source-id discipline gate** ✓ — populate block imports `crate::sources::SOURCE_VN_FOLK_RITUAL` as a const (no bare string literal); `tests/source_id_guard.rs` 1/1 pass.
6. **Build + test gate** ✓ — `cargo build -p amlich-core` clean; 712 lib tests pass (+3 from 709 baseline: `source_id_alias_is_string`, `offering_ref_serde_round_trip_and_deny_unknown_fields`, `day_snapshot_offering_refs_populated_and_deduped`); zero regressions.
7. **Schema-lock-before-builder discipline** ✓ — `grep -n "NodeConcept::Offering\|EdgeConcept::RecommendsOffering\|add_offering_facts" crates/amlich-core/src/semantic_graph/` returns ZERO matches; no `payload` field added to `SemanticNode` (Plan 19-02's domain).
8. **Focused populate test gate** ✓ — `day_snapshot_offering_refs_populated_and_deduped` exists and passes — exercises both fields populated, `offering_id` pattern, source_id discipline, dedup invariant, None → absent in JSON.
9. **Diff discipline** ✓ — `git diff --stat` shows ONLY additive changes: 213 insertions, 0 deletions across 3 files (sources.rs +23, schema.rs +88, lib.rs +102; mod.rs +0).

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Phase 19-01 complete: `OfferingRef` schema locked + `SourceId` alias introduced + 2 DaySnapshot fields added + populate block derives from `applicable_rituals` + focused test in place.
- Phase 19-02 ready: builder code can now emit `Offering` semantic-graph nodes targeting the locked `OfferingRef` type + add the `payload: Option<serde_json::Value>` field to `SemanticNode` (Q4 dual-surface decision: fields on DaySnapshot PLUS additive payload on SemanticNode).
- Phase 19-03 ready: external-crate black-box tests can now consume `offering_refs` field via the public API surface.
- INT-08 closed (4-field identity tuple for Offering semantic-graph node + DaySnapshot additive surface).

---

*Phase: 19-recommends-offering-semantic-graph-node*
*Completed: 2026-07-15*
## Self-Check: PASSED
