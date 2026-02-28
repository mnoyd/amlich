# Roadmap: Amlich Almanac Correctness Audit

## Overview

Four phases transform the amlich almanac from internally-consistent but unverified to KHCBPPT-validated. Phase 1 pins the authoritative source and extracts raw reference tables — this manual research work cannot be automated or skipped. Phase 2 serializes those tables into a machine-readable golden dataset with a Rust loader. Phase 3 writes per-subsystem validator harnesses and surfaces the full divergence inventory without making any corrections. Phase 4 applies all fixes to baseline.json and source constants until `cargo test` passes with zero divergences.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [ ] **Phase 1: Source Establishment** - Pin the KHCBPPT edition and extract raw reference tables per subsystem
- [ ] **Phase 2: Golden Dataset and Loader** - Serialize reference tables into khcbppt-golden.json and build Rust loader
- [ ] **Phase 3: Validator Harness and Divergence Inventory** - Write per-subsystem validators and surface all divergences
- [ ] **Phase 4: Correction and Zero-Divergence Verification** - Fix all divergences in baseline.json and source constants

## Phase Details

### Phase 1: Source Establishment
**Goal**: The KHCBPPT edition is pinned and all raw reference tables are extracted, so no downstream work rests on an unstable foundation
**Depends on**: Nothing (first phase)
**Requirements**: SRC-01, SRC-02, SRC-03
**Success Criteria** (what must be TRUE):
  1. A specific KHCBPPT edition is identified, documented, and recorded in the golden dataset metadata — subsequent work cites this edition consistently
  2. The nạp âm scope question is resolved: either nạp âm is confirmed in scope (source is KHCBPPT) or formally deferred, and the golden dataset schema reflects this decision
  3. KHCBPPT's treatment of intercalary months for taboo and trực rules is documented from the text, not inferred from the implementation
**Plans**: TBD

### Phase 2: Golden Dataset and Loader
**Goal**: A machine-readable, KHCBPPT-cited golden dataset with ~200 representative dates exists and compiles cleanly into typed Rust structs
**Depends on**: Phase 1
**Requirements**: DATA-01, DATA-02, DATA-03, DATA-04
**Success Criteria** (what must be TRUE):
  1. `khcbppt-golden.json` contains ~200 entries covering all 12 chi, 10 can, 12 lunar months, and 28 JD-cycle positions for dates in 2020–2030
  2. Every entry in the golden dataset carries a `khcbppt_ref` citation field pointing to the source text
  3. `golden_loader.rs` deserializes the dataset into typed `GoldenEntry` Rust structs and `cargo test --package amlich-core` passes cleanly
**Plans**: TBD

### Phase 3: Validator Harness and Divergence Inventory
**Goal**: Per-subsystem validator test files exist, compile, and run — producing a complete divergence inventory across all subsystems from a single `cargo test` run
**Depends on**: Phase 2
**Requirements**: TAB-01, TAB-02, TAB-03, TAB-04, DEI-01, DEI-02, TRC-01, STR-01, STR-02, STR-03, THH-01, XH-01, NAM-01
**Success Criteria** (what must be TRUE):
  1. All `khcbppt_*.rs` validator files (`khcbppt_stars.rs`, `khcbppt_taboos.rs`, `khcbppt_deity.rs`, `khcbppt_truc.rs`, `khcbppt_xung_hop.rs`, `khcbppt_than_huong.rs`, `khcbppt_na_am.rs`) compile and run under `cargo test`
  2. Running `cargo test --package amlich-core` produces a readable divergence report — every mismatch between golden dataset and implementation output is visible, not just the first failure
  3. The 28-star JD epoch offset is verified against 3+ real KHCBPPT dated entries before any other star validation proceeds
  4. No corrections are applied to baseline.json or source constants during this phase — inventory completeness is the goal
**Plans**: TBD

### Phase 4: Correction and Zero-Divergence Verification
**Goal**: Every divergence found in Phase 3 is fixed, `cargo test --package amlich-core` passes with zero divergences including all new validators, and all pre-existing regression tests still pass
**Depends on**: Phase 3
**Requirements**: TAB-05, DEI-03, TRC-02, STR-04, THH-02, XH-02, NAM-02
**Success Criteria** (what must be TRUE):
  1. `cargo test --package amlich-core` passes with zero failures including all `khcbppt_*.rs` validators
  2. All pre-existing golden tests and regression tests (`almanac_golden.rs`, `ruleset_determinism.rs`, `taboo_boundary.rs`) continue to pass after corrections
  3. Every correction to baseline.json or source constants is traceable to a specific KHCBPPT citation in the golden dataset
**Plans**: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3 → 4

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Source Establishment | 0/TBD | Not started | - |
| 2. Golden Dataset and Loader | 0/TBD | Not started | - |
| 3. Validator Harness and Divergence Inventory | 0/TBD | Not started | - |
| 4. Correction and Zero-Divergence Verification | 0/TBD | Not started | - |
