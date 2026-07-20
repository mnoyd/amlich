---
phase: 07-hour-pillar-parity-core
plan: 02
subsystem: testing
tags: [hour-pillar, parity, fixtures, boundaries, integration-tests]
requires:
  - phase: 07-hour-pillar-parity-core
    provides: Public hour-pillar APIs and deterministic slot/seed contracts
provides:
  - Integration fixture matrix across all day-stem seed groups and rollover times
  - Exhaustive transition-boundary validator across all 12 hour slots
  - Metadata contract assertions for hour-pillar parity outputs
affects: [phase-07-hour-pillar-parity-core, phase-08-sexagenary-cycle-parity-and-validators]
tech-stack:
  added: []
  patterns: [explicit-fixture-oracle, boundary-pair-validation, metadata-contract-assertion]
key-files:
  created:
    - crates/amlich-core/tests/hour_pillar_parity.rs
  modified:
    - crates/amlich-core/src/almanac/hour_pillar.rs
requirements-completed: [HP-03, HP-05, PAR-02]
completed: 2026-03-03
---

# Phase 07 Plan 02: Hour Pillar Parity Validators Summary

Locked hour-pillar parity behavior with a dedicated integration validator suite that enforces boundary correctness, seed-group fixture expectations, and stable evidence metadata.

## Accomplishments

- Added `crates/amlich-core/tests/hour_pillar_parity.rs` with an explicit parity fixture matrix for Giáp/Ất/Bính/Đinh/Mậu representatives plus rollover cases at `23:xx` and `00:xx`.
- Added `validate_hour_slot_boundaries_all_transitions` to verify every `xx:59 -> xx+1:00` transition, including Hợi -> Tý rollover, maps to adjacent slots without overlap/gap.
- Added `validate_hour_pillar_parity_matrix` to assert full 12-slot canonical progression for one baseline day stem.
- Added metadata contract checks asserting non-empty and stable `source_id`/`method`/`profile` values for all validator paths.

## Verification

- `cargo test --package amlich-core --test hour_pillar_parity -- --nocapture`
- `cargo check --package amlich-core`

All checks passed.
