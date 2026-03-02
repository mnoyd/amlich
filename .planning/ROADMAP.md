# Roadmap: Amlich Almanac Correctness Audit

## Overview

Four phases transform the amlich almanac from internally-consistent but unverified to KHCBPPT-validated. Phase 1 pins the authoritative source and extracts raw reference tables — this manual research work cannot be automated or skipped. Phase 2 serializes those tables into a machine-readable golden dataset with a Rust loader. Phase 3 writes per-subsystem validator harnesses and surfaces the full divergence inventory without making any corrections. Phase 4 applies all fixes to baseline.json and source constants until `cargo test` passes with zero divergences.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [x] **Phase 1: Source Establishment** - Pin the KHCBPPT edition and extract raw reference tables per subsystem [COMPLETE 2026-03-01]
- [x] **Phase 2: Golden Dataset and Loader** - Serialize reference tables into khcbppt-golden.json and build Rust loader [COMPLETE 2026-03-01]
- [x] **Phase 3: Validator Harness and Divergence Inventory** - Write per-subsystem validators and surface all divergences [COMPLETE 2026-03-01]
- [x] **Phase 4: Correction and Zero-Divergence Verification** - Fix all divergences in baseline.json and source constants [COMPLETE 2026-03-02]
- [ ] **Phase v1.1: Foundation Extensions** - Add enhanced Xung Hợp, Tàng Can, and Tiết Khí helper functions [BLOCKED/PENDING ACCEPTANCE 2026-03-02]
- [x] **Phase v1.1.1: Verification and Traceability Closure** - Add missing verification artifacts and reconcile milestone metadata after v1.1 execution [COMPLETE 2026-03-02]
- [ ] **Phase v1.1.2: Tiết Khí Regression Fix and Acceptance Gate** - Fix failing tietkhi tests and re-open v1.1 test-backed acceptance gate [REQUIRED ACCEPTANCE GATE 2026-03-02]
- [ ] **Phase v1.2: Ten Gods and Kua Foundation** - Add Thập Thần, Tứ Mệnh, and integration surfaces after v1.1 [AT-RISK: DEPENDENCY-BLOCKED BY v1.1.2 2026-03-02]

## Phase Details

### Phase 1: Source Establishment
**Goal**: The KHCBPPT edition is pinned and all raw reference tables are extracted, so no downstream work rests on an unstable foundation
**Depends on**: Nothing (first phase)
**Requirements**: SRC-01, SRC-02, SRC-03
**Success Criteria** (what must be TRUE):
  1. A specific KHCBPPT edition is identified, documented, and recorded in the golden dataset metadata — subsequent work cites this edition consistently
  2. The nạp âm scope question is resolved: either nạp âm is confirmed in scope (source is KHCBPPT) or formally deferred, and the golden dataset schema reflects this decision
  3. KHCBPPT's treatment of intercalary months for taboo and trực rules is documented from the text, not inferred from the implementation
**Plans**: 2 plans
- [x] 01-01-PLAN.md — Pin KHCBPPT edition and resolve nap am scope (SRC-01, SRC-02) [COMPLETE 2026-02-28]
- [x] 01-02-PLAN.md — Extract subsystem reference tables and document intercalary month handling (SRC-03) [COMPLETE 2026-03-01]

### Phase 2: Golden Dataset and Loader
**Goal**: A machine-readable, KHCBPPT-cited golden dataset with ~200 representative dates exists and compiles cleanly into typed Rust structs
**Depends on**: Phase 1
**Requirements**: DATA-01, DATA-02, DATA-03, DATA-04
**Success Criteria** (what must be TRUE):
  1. `khcbppt-golden.json` contains ~200 entries covering all 12 chi, 10 can, 12 lunar months, and 28 JD-cycle positions for dates in 2020–2030
  2. Every entry in the golden dataset carries a `khcbppt_ref` citation field pointing to the source text
  3. `golden_loader.rs` deserializes the dataset into typed `GoldenEntry` Rust structs and `cargo test --package amlich-core` passes cleanly
**Plans**: 2 plans
- [x] 02-01-PLAN.md — Define GoldenEntry structs and generate ~200-entry khcbppt-golden.json dataset (DATA-01, DATA-02, DATA-03) [COMPLETE 2026-03-01]
- [x] 02-02-PLAN.md — Wire golden loader with include_str!, validation, and test coverage (DATA-04) [COMPLETE 2026-03-01]

### Phase 3: Validator Harness and Divergence Inventory
**Goal**: Per-subsystem validator test files exist, compile, and run — producing a complete divergence inventory across all subsystems from a single `cargo test` run
**Depends on**: Phase 2
**Requirements**: TAB-01, TAB-02, TAB-03, TAB-04, DEI-01, DEI-02, TRC-01, STR-01, STR-02, STR-03, THH-01, XH-01, NAM-01
**Success Criteria** (what must be TRUE):
  1. All `khcbppt_*.rs` validator files (`khcbppt_stars.rs`, `khcbppt_taboos.rs`, `khcbppt_deity.rs`, `khcbppt_truc.rs`, `khcbppt_xung_hop.rs`, `khcbppt_than_huong.rs`, `khcbppt_na_am.rs`) compile and run under `cargo test`
  2. Running `cargo test --package amlich-core` produces a readable divergence report — every mismatch between golden dataset and implementation output is visible, not just the first failure
  3. The 28-star JD epoch offset is verified against 3+ real KHCBPPT dated entries before any other star validation proceeds
  4. No corrections are applied to baseline.json or source constants during this phase — inventory completeness is the goal
**Plans**: 3 plans
- [x] 03-01-PLAN.md — Star and taboo validators with JD epoch verification and set-based taboo comparison (STR-01, STR-02, STR-03, TAB-01, TAB-02, TAB-03, TAB-04) [COMPLETE 2026-03-01]
- [x] 03-02-PLAN.md — Deity, truc, and xung hop validators with enum conversion and sorted vec comparison (DEI-01, DEI-02, TRC-01, XH-01) [COMPLETE 2026-03-01]
- [x] 03-03-PLAN.md — Than huong and na am validators plus full suite verification (THH-01, NAM-01) [COMPLETE 2026-03-01]

### Phase 4: Correction and Zero-Divergence Verification
**Goal**: Every divergence found in Phase 3 is fixed, `cargo test --package amlich-core` passes with zero divergences including all new validators, and all pre-existing regression tests still pass
**Depends on**: Phase 3
**Requirements**: TAB-05, DEI-03, TRC-02, STR-04, THH-02, XH-02, NAM-02
**Success Criteria** (what must be TRUE):
  1. `cargo test --package amlich-core` passes with zero failures including all `khcbppt_*.rs` validators
  2. All pre-existing golden tests and regression tests (`almanac_golden.rs`, `ruleset_determinism.rs`, `taboo_boundary.rs`) continue to pass after corrections
  3. Every correction to baseline.json or source constants is traceable to a specific KHCBPPT citation in the golden dataset
**Plans**: 1 plan
- [x] 04-01-PLAN.md — Execute one coordinated correction batch with citation ledger to reach strict zero divergence (TAB-05, DEI-03, TRC-02, STR-04, THH-02, XH-02, NAM-02) [COMPLETE 2026-03-02]

### Phase v1.1: Foundation Extensions
**Goal**: Complete Xung Hợp relationship system, implement Tàng Can (Hidden Stems), and add Tiết Khí helper functions to provide foundational infrastructure for advanced features
**Depends on**: v1.0 (all phases complete)
**Requirements**: XH-01 through XH-08, TC-01 through TC-09, TK-01 through TK-06
**Verification Authority**: `.planning/phases/v1.1-foundation-extensions/v1.1-VERIFICATION.md`
**Current Status**: Blocked/pending acceptance — implementation artifacts exist but TK-01..TK-06 remain blocked until v1.1.2 restores a green full-package gate
**Success Criteria** (what must be TRUE):
  1. All 6 Xung Hợp relationship types (Lục xung, Tam hợp, Tứ hành xung, Lục hợp, Tương hại, Tương hình) are implemented and tested
  2. Tàng Can subsystem is implemented with correct data for all 12 branches with strength values
  3. Tiết Khí helper function exists and returns signed days to nearest Solar Term
  4. All new subsystems are integrated into `DayFortune` struct and populated in `calculate_day_fortune()`
  5. JSON serialization includes all new fields (`liu_he`, `xiang_hai`, `xiang_xing`, `tang_can`)
  6. All unit tests pass and v1.0 regression tests continue to pass
**Plans**: 3 plans in Wave 1 (parallel execution)
- [x] v1.1-01-PLAN.md (Wave 1) — Enhanced Xung Hợp relationships (Lục hợp, Tương hại, Tương hình) [IMPLEMENTED 2026-03-02, ACCEPTANCE TRACKED IN v1.1-VERIFICATION.md]
- [x] v1.1-02-PLAN.md (Wave 1) — Tàng Can (Hidden Stems) implementation [IMPLEMENTED 2026-03-02, ACCEPTANCE TRACKED IN v1.1-VERIFICATION.md]
- [x] v1.1-03-PLAN.md (Wave 1) — Tiết Khí helper functions and public API [IMPLEMENTED 2026-03-02, TK ACCEPTANCE BLOCKED]

### Phase v1.1.1: Verification and Traceability Closure
**Goal**: Restore milestone verifiability by adding missing phase verification artifacts and machine-readable traceability links for v1.1 requirements
**Depends on**: Phase v1.1 implementation outputs (`v1.1-01..03-SUMMARY.md`)
**Requirements**: XH-01..XH-08, TC-01..TC-09, TK-01..TK-06
**Gap Closure:** Closes audit gaps for missing `VERIFICATION.md`, missing `requirements-completed` frontmatter, and roadmap/state completion mismatch
**Verification Authority**: `.planning/phases/v1.1-foundation-extensions/v1.1-VERIFICATION.md`
**Success Criteria** (what must be TRUE):
  1. Canonical `v1.1-VERIFICATION.md` exists with per-requirement implementation evidence and fresh test evidence for XH/TC/TK scopes
  2. `v1.1-01..03-SUMMARY.md` files include machine-readable `requirements-completed` frontmatter linked to the canonical verification artifact
  3. `REQUIREMENTS-v1.1.md` traceability statuses align with verification truth, with TK requirements marked blocked/partial until v1.1.2 fixes the gate
  4. `ROADMAP.md` and `STATE.md` status metadata are reconciled to verification-led truth (v1.1 blocked/pending, not complete)
**Plans**: 5 plans in 2 waves + post-cycle closure
- [x] v1.1.1-01-PLAN.md (Wave 1) — Canonical XH/TC verification matrix and summary frontmatter linkage [COMPLETE 2026-03-02]
- [x] v1.1.1-02-PLAN.md (Wave 2) — TK acceptance evidence closure and audit handoff to v1.1.2 [COMPLETE 2026-03-02]
- [x] v1.1.1-03-PLAN.md (Wave 2) — ROADMAP/STATE status reconciliation from verification truth [COMPLETE 2026-03-02]
- [x] v1.1.1-04-PLAN.md (Post-cycle) — Re-verify metadata alignment and close residual status gap [COMPLETE 2026-03-02]
- [x] v1.1.1-05-PLAN.md (Post-cycle) — Master requirements accounting closure and ID collision disambiguation [COMPLETE 2026-03-02]

Execution note: The active execution set for this cycle is `v1.1.1-01` through `v1.1.1-03`. Post-cycle artifacts run sequentially only after the active set completes: `v1.1.1-04` after `v1.1.1-03`, then `v1.1.1-05` after `v1.1.1-04`.

### Phase v1.1.2: Tiết Khí Regression Fix and Acceptance Gate
**Goal**: Restore green test gate for v1.1 by fixing failing `tietkhi` assertions and recording acceptance evidence
**Depends on**: Phase v1.1.1 (verification scaffolding in place)
**Requirements**: TK-01..TK-06
**Gap Closure:** Closes audit gaps for failing `cargo test --package amlich-core` tietkhi cases and v1.2 dependency gating
**Current Status**: Required next gate — v1.1 remains blocked/pending until this phase is green

### Phase v1.2: Ten Gods and Kua Foundation
**Goal**: Add advanced foundational calculations (Thập Thần and Tứ Mệnh) and integrate them into day-level/public outputs without breaking validated behavior
**Depends on**: v1.1 (Foundation Extensions complete)
**Dependency Risk Note**: At-risk/dependency-blocked while v1.1 TK acceptance gate is red (`v1.1-VERIFICATION.md` status: partial)
**Requirements**: TT-01 through TT-05, TM-01 through TM-05, INT-01 through INT-06
**Success Criteria** (what must be TRUE):
  1. Ten Gods engine computes deterministic day-stem-to-target-stem relationships and passes complete matrix tests
  2. Tứ Mệnh/Kua calculations produce typed outputs with documented conventions and representative-year coverage
  3. DayFortune/public API/JSON surfaces include new fields in backward-compatible form
  4. Full regression suite passes, including all KHCBPPT correctness validators from v1.0
  5. Đại Vận remains explicitly deferred and unimplemented in this phase
**Plans**: 3 plans in Wave 1 (parallel where safe)
- [ ] v1.2-01-PLAN.md (Wave 1) — Thập Thần engine and matrix validation (TT-01..TT-05)
- [ ] v1.2-02-PLAN.md (Wave 1) — Tứ Mệnh/Kua calculation module and fixtures (TM-01..TM-05)
- [ ] v1.2-03-PLAN.md (Wave 1) — DayFortune/API integration, serialization, and regressions (INT-01..INT-06)

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3 → 4

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Source Establishment | 2/2 | Complete | 2026-03-01 |
| 2. Golden Dataset and Loader | 2/2 | Complete | 2026-03-01 |
| 3. Validator Harness and Divergence Inventory | 3/3 | Complete | 2026-03-01 |
| 4. Correction and Zero-Divergence Verification | 1/1 | Complete | 2026-03-02 |
| v1.1: Foundation Extensions | 3/3 | Blocked/Pending Acceptance (TK gate red) | 2026-03-02 |
| v1.1.1: Verification and Traceability Closure | 5/5 | Complete | 2026-03-02 |
| v1.1.2: Tiết Khí Regression Fix and Acceptance Gate | 0/1 | Required Gate (Pending) | - |
| v1.2: Ten Gods and Kua Foundation | 0/3 | At Risk / Dependency-Blocked by v1.1.2 | 2026-03-02 |
