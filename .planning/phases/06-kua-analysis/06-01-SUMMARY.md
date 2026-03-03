---
phase: 06-kua-analysis
plan: 01
subsystem: almanac
tags: [dai-van, kua, helpers, contracts, rust]
requires:
  - phase: 05-ten-gods-integration-and-helpers
    provides: Stable Dai Van helper boundaries and lazy extension pattern
provides:
  - Dai Van Kua analysis types for per-pillar directional guidance
  - Birth-Kua-once analysis path with reusable precomputed API
  - Age/index helper queries for Kua analysis retrieval
affects: [phase-06-kua-analysis, v1.3-milestone]
tech-stack:
  added: []
  patterns: [single-computation-reuse, element-direction-intersection, helper-query-reuse]
key-files:
  created:
    - .planning/phases/06-kua-analysis/06-01-PLAN.md
    - .planning/phases/06-kua-analysis/06-01-SUMMARY.md
  modified:
    - crates/amlich-core/src/almanac/dai_van.rs
requirements-completed: [DV-KUA-01, DV-KUA-02, DV-KUA-03, DV-KUA-04]
completed: 2026-03-03
---

# Phase 06 Plan 01: Kua Analysis Summary

Implemented Phase 6 Kua analysis on top of Dai Van by introducing deterministic per-pillar element analysis against birth Kua direction sets, with reusable helper APIs and full contract tests.

## Accomplishments

- Added `DaiVanKuaAnalysis` and `DaiVanKuaPillarAnalysis` output types in `crates/amlich-core/src/almanac/dai_van.rs`.
- Added `analyze_dai_van_with_kua` (compute Kua once) and `analyze_dai_van_with_precomputed_kua` (explicit reuse path).
- Added lookup helpers `get_kua_analysis_for_pillar` and `get_kua_analysis_for_age` that preserve existing half-open pillar semantics.
- Added deterministic tests for Kua-5 convention behavior, per-pillar favorable/unfavorable intersections, and age/index query behavior.

## Verification

- `cargo test --package amlich-core --lib dai_van::tests`
- `cargo test --package amlich-core --lib`

All checks passed.
