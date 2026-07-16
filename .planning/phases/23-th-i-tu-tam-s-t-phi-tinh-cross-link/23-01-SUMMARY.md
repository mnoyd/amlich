---
phase: 23-th-i-tu-tam-s-t-phi-tinh-cross-link
plan: 01
subsystem: almanac
tags: [almanac, thai-tue, tam-sat, sat-phuong, khcbppt, evidence-backfill, directional]

# Dependency graph
requires:
  - phase: 20-foundation-schema-lock-source-ids-adrs-ontology
    provides: SOURCE_KHCBPPT const + RuleEvidence shape + Direction enum + CHI table
provides:
  - "almanac::thai_tue::ThaiTueDirectionResult + thai_tue_direction(year_chi_index) — year-only directional Thái Tuế sibling"
  - "almanac::tam_sat::TamSatDirectionResult + tam_sat_direction(year_chi_index) — classical three-direction Tam Sát"
  - "compute_thai_tue + get_sat_phuong now carry populated KHCBPPT RuleEvidence (XLK-01 backfill)"
  - "data/almanac/tam_sat_provenance.md — discoverable pending-review provenance ledger"
affects:
  - 23-th-i-tu-tam-s-t-phi-tinh-cross-link (Plan 23-02 consumes thai_tue_direction + tam_sat_direction + the backfilled evidence)
  - 24-iching-evaluator-semantic-graph-wiring-dto-integration (Phase 24 wires direction_cross_link into semantic graph)

# Tech tracking
tech-stack:
  added: []  # zero new dependencies — pure-Rust additive module
  patterns:
    - "Sibling directional API pattern (year-only pub fn alongside personal-conflict compute_thai_tue)"
    - "Tradition-ordered Tam Hợp table with explicit branch membership (NOT chi%4 sort order)"
    - "Evidence profile text encodes pending-review marker + provenance artifact reference"
    - "TDD RED→GREEN discipline with backward-compat round-trip gate before production backfill"

key-files:
  created:
    - crates/amlich-core/src/almanac/tam_sat.rs
    - crates/amlich-core/tests/almanac_backfill_compat.rs
    - crates/amlich-core/tests/tam_sat_integration.rs
    - crates/amlich-core/data/almanac/tam_sat_provenance.md
  modified:
    - crates/amlich-core/src/almanac/thai_tue.rs
    - crates/amlich-core/src/almanac/sat_phuong.rs
    - crates/amlich-core/src/almanac/mod.rs

key-decisions:
  - "ThaiTueDirectionResult is a sibling of compute_thai_tue (year-only, no birth context); the personal-conflict API is unchanged"
  - "Tam Sát Tam Hợp triad arrays preserve tradition order (Thân, Tý, Thìn) NOT branch-index sort (Tý, Thìn, Thân) per CONTEXT.md"
  - "Evidence backfills use method names 'thai_tue_year_branch_conflict' and 'sat_phuong_day_chi' to keep the existing APIs' semantic identity"
  - "Tam Sát exact KHCBPPT page citation honestly deferred — evidence profile text references data/almanac/tam_sat_provenance.md + PendingExternalReview marker (no fabricated page)"
  - "Out-of-range year_chi_index panics with a useful message via direction_for_year_chi match before CHI indexing (avoids the opaque 'index out of bounds')"

patterns-established:
  - "Pattern: tradition-ordered Tam Hợp lookup via explicit branch-membership search (NOT chi%4) when source convention differs from sorted order"
  - "Pattern: evidence profile text doubles as in-band provenance pointer (artifact path + PendingExternalReview marker) for deferred citations"
  - "Pattern: TDD RED-phase compatibility gate (legacy JSON without evidence → deserializes to None) before production backfill"

requirements-completed: [XLK-01, XLK-02]

# Metrics
duration: 15 min
completed: 2026-07-16
---

# Phase 23 Plan 01: Thái Tuế Directional Sibling + Tam Sát Module + KHCBPPT Evidence Backfills Summary

**Directional Thái Tuế sibling API + classical three-direction Tam Sát module + two KHCBPPT evidence backfills, all backward-compatible with v1.6 JSON and traceable to a discoverable pending-review provenance ledger**

## Performance

- **Duration:** ~15 min
- **Started:** 2026-07-16T15:16:19Z
- **Completed:** 2026-07-16T15:31:03Z
- **Tasks:** 2 (both TDD: RED → GREEN)
- **Files modified:** 7 (3 src modifications, 3 new test/src files, 1 new data artifact)

## Accomplishments

- **XLK-01 closed:** `thai_tue_direction(year_chi_index)` sibling API returns the year-branch → 8-point `Direction` mapping (cardinals unique, intercardinals collapse in pairs per CONTEXT.md) with non-optional KHCBPPT provenance; `compute_thai_tue` personal-conflict API unchanged.
- **XLK-01 closed:** Both `compute_thai_tue` + `get_sat_phuong` evidence fields backfilled from `None` → `Some(RuleEvidence { source_id: SOURCE_KHCBPPT, ... })` at the locked return sites; legacy v1.6 JSON without `evidence` still deserializes to `None` (BC guarantee).
- **XLK-02 closed:** New `almanac::tam_sat` module returns the classical three-direction Tam Sát for every year-chi via a tradition-ordered Tam Hợp table (Water/Wood/Fire/Metal rows). Mirrors `tam_tai.rs::TAI_YEARS` lục-xung opposite-triad concept as a distinct year-only directional module (NOT the 3-year Tam Tai cycle).
- **XLK-02 closed:** Honest `data/almanac/tam_sat_provenance.md` artifact (1-page discoverable ledger) states the locked rule, the four mapping rows, the search criteria, and an explicit `PendingExternalReview` marker for the exact KHCBPPT edition/page pin — no fabricated citation.

## Task Commits

Each task was committed atomically (TDD discipline — RED then GREEN):

1. **Task 1 RED — backfill-compat gate** — `f4adf03` (test)
   - `crates/amlich-core/tests/almanac_backfill_compat.rs` — 211 lines, 6 tests
2. **Task 1 GREEN — directional sibling + evidence backfills** — `4d324cb` (feat)
   - `crates/amlich-core/src/almanac/thai_tue.rs`, `sat_phuong.rs`
3. **Task 2 RED — Tam Sát integration gate** — `7461548` (test)
   - `crates/amlich-core/src/almanac/tam_sat.rs` (RED stub), `mod.rs`, `tests/tam_sat_integration.rs`
4. **Task 2 GREEN — classical Tam Sát module + provenance** — `f67c72d` (feat)
   - `crates/amlich-core/src/almanac/tam_sat.rs`, `data/almanac/tam_sat_provenance.md`

**Plan metadata:** (this commit) — `docs(23-01): complete plan`

## Files Created/Modified

- `crates/amlich-core/src/almanac/thai_tue.rs` — added `ThaiTueDirectionResult` + `thai_tue_direction(year_chi_index)` + `direction_for_year_chi` helper; backfilled `compute_thai_tue` evidence; updated inline `evidence_defaults_to_none` test.
- `crates/amlich-core/src/almanac/sat_phuong.rs` — backfilled `get_sat_phuong` evidence; updated inline `evidence_defaults_to_none` test.
- `crates/amlich-core/src/almanac/mod.rs` — registered `pub mod tam_sat;`.
- `crates/amlich-core/src/almanac/tam_sat.rs` — new module (~260 lines): `TamSatDirectionResult` + `TAM_SAT_ROWS` tradition-ordered table + `find_triad_row` + `tam_sat_direction`; 5 inline tests.
- `crates/amlich-core/tests/almanac_backfill_compat.rs` — new BC + populated round-trip gate (211 lines, 6 tests).
- `crates/amlich-core/tests/tam_sat_integration.rs` — new black-box integration gate (255 lines, 10 tests).
- `crates/amlich-core/data/almanac/tam_sat_provenance.md` — discoverable pending-review provenance ledger (~95 lines).

## Decisions Made

- **Tradition-ordered Tam Hợp triad rows** (Thân, Tý, Thìn) — NOT branch-index sorted (Tý, Thìn, Thân). CONTEXT.md note warns the existing `xung_hop::tam_hop` returns sorted order; Tam Sát preserves tradition order to match the source convention.
- **Distinct evidence method names** for the two backfills (`thai_tue_year_branch_conflict`, `sat_phuong_day_chi`) — preserves the existing APIs' semantic identity even though both share `source_id: SOURCE_KHCBPPT`.
- **Out-of-range panic discipline** — `thai_tue_direction` validates via `direction_for_year_chi` match BEFORE indexing `CHI[year_chi_index]`, so the panic message ("year_chi_index N not in 0..=11") is useful rather than the opaque "index out of bounds" from `CHI[12]`.
- **Honest citation deferral** — the Tam Sát module's `evidence.profile` text references the discoverable provenance artifact and carries an explicit `PendingExternalReview` marker. The Rust code never fabricates a KHCBPPT Quyển/Trang/Câu pin.
- **TDD discipline with RED stub** — Task 2's RED commit uses `unimplemented!("RED phase: ...")` in production code so the integration test target compiles and runs (cleaner RED state than a compile failure).

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Tam Sát test assertion incorrectly claimed Tý excludes South**
- **Found during:** Task 2 GREEN phase
- **Issue:** The `tam_sat_result_is_distinct_module_from_sat_phuong` test asserted Tam Sát for Tý year does NOT include `Direction::South`, but per the locked mapping (Water triad → opposite Ngọ → South) South IS one of the three Tam Sát directions for Tý.
- **Fix:** Rewrote the assertion to assert South IS present (Ngọ → South) and reframed the test's intent: Tam Sát and Sát Phương are sibling APIs operating on different chi axes (year vs day) with different cardinalities, NOT mutually exclusive direction sets.
- **Files modified:** `crates/amlich-core/tests/tam_sat_integration.rs`
- **Verification:** 10/10 integration tests pass after the fix.
- **Committed in:** `f67c72d` (Task 2 GREEN commit — part of the GREEN task commit, not a separate fix).

**2. [Rule 1 - Bug] Out-of-range index panic message was opaque**
- **Found during:** Task 1 GREEN phase
- **Issue:** Initial `thai_tue_direction(12)` implementation panicked with "index out of bounds: the len is 12 but the index is 12" because `CHI[year_chi_index]` evaluated before the `direction_for_year_chi` match arm could panic with a useful message. The `should_panic(expected = "not in 0..=11")` test failed.
- **Fix:** Reordered `thai_tue_direction` to bind `direction = direction_for_year_chi(year_chi_index)` (which panics with the useful message) BEFORE accessing `CHI[year_chi_index]`.
- **Files modified:** `crates/amlich-core/src/almanac/thai_tue.rs`
- **Verification:** `out_of_range_year_chi_index_panics` test passes; the 12-branch coverage test still passes.
- **Committed in:** `4d324cb` (Task 1 GREEN commit).

---

**Total deviations:** 2 auto-fixed (2 Rule 1 — Bug)
**Impact on plan:** Both auto-fixes necessary for the plan's own verification gates (test assertions + should_panic contract) to pass. No scope creep, no behavior change to the locked mappings or APIs.

## Issues Encountered

None beyond the two auto-fixed deviations above.

## User Setup Required

None — no external service configuration required. Zero new crate dependencies (`cargo tree -p amlich-core --depth 1` unchanged).

## Next Phase Readiness

- **Plan 23-02 unblocked:** `thai_tue_direction` + `tam_sat_direction` + the backfilled KHCBPPT evidence envelopes are now available for the read-only `reasoning::direction_composite::build_direction_cross_link` to consume. Plan 23-02 was running in parallel; its in-flight `direction_composite.rs` stub compiled successfully against this plan's primitives.
- **Phase 24 (IChing Evaluator + Semantic-Graph Wiring) ready once Phase 23 closes:** Phase 24 wires the `DirectionCrossLinkSummary` (Plan 23-02's output) into the semantic graph via `add_direction_composite_facts`.
- **No blockers.** The Tam Sát KHCBPPT page citation remains deferred per the provenance ledger's PendingExternalReview marker — this is the documented honest-deferral discipline (mirrors ADR-0006 §5), not a blocker for the runtime contract.

## Self-Check: PASSED

- All 7 declared `files_modified` exist on disk (verified via `[ -f ]`).
- All 4 task commits present in `git log` (`f4adf03`, `4d324cb`, `7461548`, `f67c72d`).
- `tests/almanac_backfill_compat.rs` is 211 lines (≥70 line minimum per artifact spec).
- `data/almanac/tam_sat_provenance.md` contains "PendingExternalReview" (3 occurrences).
- `pub fn thai_tue_direction` exists in `thai_tue.rs`.
- `pub fn tam_sat_direction` exists in `tam_sat.rs`.
- Full crate test suite green: 774 lib tests + all integration targets pass, 0 failures.

---
*Phase: 23-th-i-tu-tam-s-t-phi-tinh-cross-link*
*Completed: 2026-07-16*
