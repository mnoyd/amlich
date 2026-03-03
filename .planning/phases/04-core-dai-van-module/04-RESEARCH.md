# Phase 4 Research - Core Dai Van Module

**Date:** 2026-03-03  
**Phase:** 04 - Core Dai Van Module  
**Status:** Complete

## Implementation Baseline

- Primary formula source: `vietnamese_lunar_engine_tables.md` section 15 (Dai Van).
- Project source policy: keep `source_id: "khcbppt"` as explicit placeholder with traceability notes until manual KHCBPPT chapter verification is complete.
- Reuse existing modules: `canchi`, `tietkhi`, `lunar`, `julian`, and shared metadata patterns in `almanac/types.rs` and `almanac/tu_menh.rs`.

## Locked Context Requirements (must be honored)

1. Age ranges are `[start_age, end_age)` and exact transition age belongs to incoming pillar.
2. Start age uses precise decimal years (not integer-only storage).
3. Output carries both canonical direction semantics (forward/backward) and Vietnamese labels (`Thuan`/`Nghich` display equivalents).
4. Missing/invalid gender is explicit error or absence, never silent default.
5. Convention and evidence metadata are mandatory in results.

## Recommended Phase 4 Scope

- Build a pure core module: `crates/amlich-core/src/almanac/dai_van.rs`.
- Deliver deterministic Dai Van result with:
  - Chieuthu direction from year polarity x gender
  - Start-age calculation from signed nearest Tiet Khi distance using 3-days-per-year conversion
  - Eight contiguous 10-year pillars derived from month Can Chi base pillar
  - Convention/evidence metadata fields for traceability
- Add helper lookups that respect boundary semantics:
  - `get_pillar_at_age`
  - `get_current_pillar`
  - `years_to_next_transition`

## Risk Controls

- Add matrix tests for all 4 polarity x gender direction combinations.
- Add edge-case tests for:
  - exact Tiet Khi boundary (`distance == 0`)
  - leap-month birth inputs (month pillar acquisition path)
  - contiguous/non-overlapping pillar ranges
  - determinism (same input => same output)
- Keep API additive and isolated to core module and exports; no v2 API integration in this phase.

## Validation Targets for Plan Checker

- Every phase requirement ID appears in at least one plan frontmatter `requirements` field.
- Every task has explicit `<automated>` verification command.
- `must_haves` reflect outcome truths, concrete artifacts, and key wiring links.

---

*Phase: 04-core-dai-van-module*  
*Research generated: 2026-03-03*
