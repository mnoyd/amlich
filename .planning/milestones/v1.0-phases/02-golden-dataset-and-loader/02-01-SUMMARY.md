---
phase: 02-golden-dataset-and-loader
plan: 01
subsystem: testing
tags: [serde, json, golden-dataset, khcbppt, tdd, coverage-driven]

# Dependency graph
requires:
  - phase: 01-source-establishment
    provides: "8 KHCBPPT reference files with per-subsystem citations"
provides:
  - "GoldenDataset/GoldenEntry/GoldenCitation Rust structs with serde derives"
  - "khcbppt-golden.json with 233 KHCBPPT-cited entries covering all dimensions"
  - "9 coverage validation tests for dimensional completeness"
  - "Generator script (ignored test) for reproducible dataset regeneration"
affects: [02-02, 03-validators, phase-3]

# Tech tracking
tech-stack:
  added: []
  patterns: ["coverage-driven date selection", "golden dataset as test oracle with per-subsystem citations"]

key-files:
  created:
    - "crates/amlich-core/src/almanac/golden_loader.rs"
    - "crates/amlich-core/data/almanac/khcbppt-golden.json"
    - "crates/amlich-core/tests/golden_dataset_coverage.rs"
    - "crates/amlich-core/tests/generate_golden.rs"
  modified:
    - "crates/amlich-core/src/almanac/mod.rs"

key-decisions:
  - "233 entries selected (within 180-240 target) using coverage-driven algorithm: 60-day contiguous window + lunar month representatives + JD-mod-28 fill + leap months + year boundaries + tiet khi transitions"
  - "Generator implemented as #[ignore] test for reproducible regeneration via cargo test --test generate_golden -- --ignored"
  - "Star entries marked MEDIUM confidence for JD epoch per Phase 1 finding"

patterns-established:
  - "Golden dataset pattern: structs in golden_loader.rs, JSON in data/almanac/, generator as #[ignore] test, coverage validation as integration tests"
  - "Citation pattern: per-subsystem KHCBPPT citations in GoldenCitation struct with references to Phase 1 docs/reference/khcbppt/ files"

requirements-completed: [DATA-01, DATA-02, DATA-03]

# Metrics
duration: 4min
completed: 2026-03-01
---

# Phase 2 Plan 01: Golden Dataset and Structs Summary

**GoldenEntry structs with serde derives + 233-entry khcbppt-golden.json covering all 12 chi, 10 can, 12 lunar months, 28 star positions, and 19 leap months with per-subsystem KHCBPPT citations**

## Performance

- **Duration:** 4 min
- **Started:** 2026-03-01T15:39:14Z
- **Completed:** 2026-03-01T15:43:36Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments
- Defined GoldenDataset, GoldenMetadata, GoldenEntry, GoldenCitation structs with Debug/Clone/Serialize/Deserialize derives
- Generated 233-entry golden dataset covering 2020-2030 with systematic dimensional coverage verified by 9 automated tests
- Every entry carries per-subsystem KHCBPPT citations linking back to Phase 1 reference files
- Star entries explicitly note MEDIUM confidence for JD epoch (Ho Ngoc Duc implementation artifact)
- All 174 existing tests pass with zero regressions

## Task Commits

Each task was committed atomically:

1. **Task 1: Define GoldenEntry structs** - `5c59cf0` (feat)
2. **Task 2 RED: Coverage tests** - `a626c10` (test)
3. **Task 2 GREEN: Generate golden dataset** - `a4a3ad5` (feat)

**Plan metadata:** [pending] (docs: complete plan)

_TDD Task 2 has separate RED and GREEN commits._

## Files Created/Modified
- `crates/amlich-core/src/almanac/golden_loader.rs` - GoldenDataset/GoldenMetadata/GoldenEntry/GoldenCitation struct definitions
- `crates/amlich-core/src/almanac/mod.rs` - Added `pub mod golden_loader` declaration
- `crates/amlich-core/data/almanac/khcbppt-golden.json` - 233-entry golden dataset (~490KB, pretty-printed)
- `crates/amlich-core/tests/golden_dataset_coverage.rs` - 9 coverage validation tests
- `crates/amlich-core/tests/generate_golden.rs` - Dataset generator (#[ignore] test)

## Decisions Made
- **Entry count: 233** - Coverage-driven algorithm produced 233 entries (within 180-240 target). The extra entries ensure every lunar month has at least 3 representatives and every star position has at least 2.
- **Generator as #[ignore] test** - Chose `#[ignore]` test over standalone binary for simplicity. Run with `cargo test --test generate_golden -- --ignored --nocapture`.
- **All subsystem values from get_day_info()** - Per Phase 1 findings, all subsystems (taboos, day deity, truc, xung hop, than huong, na am) had their values confirmed against KHCBPPT. Implementation output IS the golden value.
- **Star confidence: MEDIUM** - JD epoch (jd.rem_euclid(28)) is Ho Ngoc Duc implementation artifact, not KHCBPPT-defined. Documented in every entry's citation.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Insufficient lunar month coverage for months 7 and 10**
- **Found during:** Task 2 (TDD GREEN phase)
- **Issue:** Initial date selection produced only 2 entries for lunar months 7 and 10 (requirement: at least 3 per month). Solar-to-lunar mapping varies by year, so guessed solar dates didn't always land in the target lunar month.
- **Fix:** Added 3 additional dates in August (for lunar month 7) and 3 in November (for lunar month 10) across different years
- **Files modified:** crates/amlich-core/tests/generate_golden.rs
- **Verification:** Coverage test golden_dataset_covers_all_12_lunar_months passes with all months having 3+ entries
- **Committed in:** a4a3ad5 (Task 2 GREEN commit)

---

**Total deviations:** 1 auto-fixed (1 bug)
**Impact on plan:** Minor date selection adjustment. No scope creep.

## Issues Encountered
None beyond the coverage gap auto-fixed above.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- GoldenDataset structs ready for Plan 02 (loader with include_str! + OnceLock + validation)
- khcbppt-golden.json at `crates/amlich-core/data/almanac/` ready for include_str! embedding
- Coverage tests provide regression safety for any dataset changes
- Phase 3 validators can use GoldenEntry structs to compare against implementation output

---
*Phase: 02-golden-dataset-and-loader*
*Completed: 2026-03-01*
