---
phase: 07-hour-pillar-parity-core
plan: 01
subsystem: almanac
tags: [hour-pillar, canchi, parity, deterministic, rust]
requires:
  - phase: v1.3-dai-van-core
    provides: Typed stem/evidence conventions and deterministic helper patterns
provides:
  - Deterministic hour-pillar core calculator from day stem and local hour/minute
  - Explicit day-stem seed-group mapping and modulo-10 stem progression helpers
  - Slot resolution contract with Tý rollover-safe boundary behavior
affects: [phase-07-hour-pillar-parity-core, v1.4-lunar-engine-table-parity]
tech-stack:
  added: []
  patterns: [table-driven-slot-mapping, seed-group-index-mapping, evidence-carrying-result]
key-files:
  created:
    - crates/amlich-core/src/almanac/hour_pillar.rs
  modified:
    - crates/amlich-core/src/almanac/mod.rs
requirements-completed: [HP-01, HP-02, HP-04]
completed: 2026-03-03
---

# Phase 07 Plan 01: Hour Pillar Core Summary

Shipped a deterministic hour-pillar core module that maps local clock time into canonical branch slots, applies the five day-stem seed groups, and returns `CanChi` plus `RuleEvidence` metadata.

## Accomplishments

- Added `crates/amlich-core/src/almanac/hour_pillar.rs` with typed contracts: `HourBranchSlot`, `HourPillarResult`, `resolve_hour_branch_slot`, and `compute_hour_pillar`.
- Implemented explicit seed mapping (Giáp/Kỷ, Ất/Canh, Bính/Tân, Đinh/Nhâm, Mậu/Quý) and modulo-10 stem progression using existing index helpers.
- Added focused unit coverage for seed rules, progression rollover, transition boundaries, and evidence token stability.
- Exported the new module through `crates/amlich-core/src/almanac/mod.rs`.

## Verification

- `cargo test --package amlich-core --lib hour_pillar::tests`

All checks passed.
