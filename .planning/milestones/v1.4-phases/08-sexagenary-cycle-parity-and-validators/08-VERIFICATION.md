---
phase: 08-sexagenary-cycle-parity-and-validators
verified: 2026-03-04T00:30:00Z
status: passed
score: 7/7 truths verified
---

# Phase 8: Sexagenary Cycle Parity and Validators Verification Report

**Phase Goal:** Deliver canonical 60-cycle conversion/progression utilities and full-table parity verification
**Verified:** 2026-03-04T00:30:00Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| #   | Truth                                                                                                                                                   | Status     | Evidence                                                                                                     |
| --- | ------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- | ------------------------------------------------------------------------------------------------------------ |
| 1   | System converts cycle index (1-60) to canonical stem-branch pair with 1-based public contract                                                         | ✓ VERIFIED | `cycle_index_to_canchi` validates bounds (1..=60), converts to 0-based internally, returns `CanChi` struct  |
| 2   | System converts stem-branch pair to cycle index (1-60) rejecting invalid combinations                                                                  | ✓ VERIFIED | `canchi_to_cycle_index` validates parity (can_index % 2 == chi_index % 2), returns None for non-canonical   |
| 3   | Forward/backward progression preserves modular correctness across 10/12/60 rollover boundaries                                                          | ✓ VERIFIED | `progress_cycle_index` uses `rem_euclid(60)` for correct signed handling, verified at boundaries (1→60, 60→1) |
| 4   | Utilities expose deterministic helpers reusable by hour pillar and Na Am APIs                                                                          | ✓ VERIFIED | Three public functions: `cycle_index_to_canchi`, `canchi_to_cycle_index`, `progress_cycle_index`, all pure functions |
| 5   | Full 60-entry parity validator confirms exact canonical table matching                                                                                   | ✓ VERIFIED | `validate_full_60_cycle_parity` iterates all 60 positions against baseline data, reports any divergences       |
| 6   | Validator iterates over all 60 positions against baseline na_am_pairs reference                                                                           | ✓ VERIFIED | Test loads `baseline_data()`, checks `sexagenary_na_am` for all cycle indices 1-60, verifies stem/branch/element |
| 7   | Regression tests guard cycle index normalization and invalid-input handling                                                                              | ✓ VERIFIED | 6 regression tests covering bounds, invalid pairs, rollover, roundtrip, and boundary cases                   |

**Score:** 7/7 truths verified

### Required Artifacts

| Artifact                                                   | Expected                                                                   | Status   | Details                                                                 |
| ---------------------------------------------------------- | ------------------------------------------------------------------------- | -------- | ----------------------------------------------------------------------- |
| `crates/amlich-core/src/almanac/sexagenary_cycle.rs`       | 60-cycle conversion/progression utilities with bounded contracts (≥180 lines) | ✓ VERIFIED | 382 lines, 3 public functions, 12 unit tests, exports to almanac module |
| `crates/amlich-core/src/almanac/mod.rs`                   | Module export for sexagenary_cycle                                         | ✓ VERIFIED | Contains `pub mod sexagenary_cycle;`                                    |
| `crates/amlich-core/tests/sexagenary_cycle_parity.rs`     | Full-table parity validator and regression test suite (≥200 lines)       | ✓ VERIFIED | 386 lines, 7 integration tests, validates against baseline.json       |

### Key Link Verification

| From                                              | To                                                | Via                                                           | Status | Details                                                                 |
| ------------------------------------------------- | ------------------------------------------------- | ------------------------------------------------------------- | ------ | ----------------------------------------------------------------------- |
| `sexagenary_cycle.rs`                            | `types.rs`                                        | Canonical CAN/CHI arrays via CanChi::new                      | ✓ WIRED | `use crate::types::CanChi;` → `CanChi::new(can_idx, chi_idx)` uses CAN[can_idx] and CHI[chi_idx] |
| `sexagenary_cycle_parity.rs`                     | `sexagenary_cycle.rs`                             | Integration tests call public APIs across all 60 cycle positions | ✓ WIRED | Imports all 3 functions, calls them in loops for indices 1-60        |
| `sexagenary_cycle_parity.rs`                     | `baseline.json` (via data.rs)                     | Validator loads na_am_pairs baseline for 30 entries × 2 = 60 positions | ✓ WIRED | `use amlich_core::almanac::data::baseline_data;`, loads `sexagenary_na_am` HashMap |
| `sexagenary_cycle_parity.rs`                     | REQUIREMENTS.md                                   | Full-table coverage mapped to SC-05 and PAR-01 expectations   | ✓ WIRED | Test docstrings reference SC-05 and PAR-01 requirements                |

### Requirements Coverage

| Requirement | Source Plan | Description                                                                                           | Status | Evidence                                                                                           |
| ----------- | ---------- | ----------------------------------------------------------------------------------------------------- | ------ | -------------------------------------------------------------------------------------------------- |
| SC-01       | 08-01-PLAN | System can convert cycle index (1-60) to canonical stem-branch pair                                   | ✓ SATISFIED | `cycle_index_to_canchi` function with bounds validation (1..=60), returns `CanChi` struct        |
| SC-02       | 08-01-PLAN | System can convert stem-branch pair to cycle index (1-60)                                              | ✓ SATISFIED | `canchi_to_cycle_index` validates parity (odd/even match), returns 1-based index                 |
| SC-03       | 08-01-PLAN | Forward/backward progression preserves modular correctness across rollover boundaries (10/12/60)      | ✓ SATISFIED | `progress_cycle_index` uses `rem_euclid(60)`, verified at 1/60 boundaries with ±N deltas         |
| SC-04       | 08-01-PLAN | Cycle utilities expose deterministic helpers reusable by hour pillar and Na Am APIs                  | ✓ SATISFIED | Three public pure functions, exported via almanac::sexagenary_cycle namespace, no side effects    |
| SC-05       | 08-02-PLAN | Validation suite confirms full-table parity against canonical 60-cycle references                    | ✓ SATISFIED | `validate_full_60_cycle_parity` iterates all 60 positions, verifies stem/branch/na_am/element  |
| PAR-01      | 08-02-PLAN | Parity validators are added for hour pillar and full 60-cycle tables                                | ✓ SATISFIED | `hour_pillar_parity.rs` (3 tests) + `sexagenary_cycle_parity.rs` (7 tests) = 10 parity validators |

**All 6 requirements (SC-01 through SC-05, PAR-01) are satisfied with implementation evidence.**

**Note:** REQUIREMENTS.md shows PAR-01 as "Planned" not "Complete" - this is a metadata inconsistency, not a gap. Both hour pillar and 60-cycle parity validators exist and all tests pass.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| ---- | ---- | ------- | -------- | ------ |
| N/A  | N/A  | None    | N/A      | No TODO/FIXME/placeholder comments, no empty returns, no console.log-only implementations found |

### Human Verification Required

None - all requirements are fully automatable and verified programmatically through test execution and code analysis.

### Gaps Summary

**No gaps found.** All observable truths are verified, all required artifacts exist with substantive implementations, all key links are wired correctly, all requirements are satisfied, and no anti-patterns detected.

**Phase 8 goal achieved:** Canonical 60-cycle conversion/progression utilities and full-table parity verification are delivered with test-backed evidence.

---

_Verified: 2026-03-04T00:30:00Z_
_Verifier: Claude (gsd-verifier)_
