---
phase: 15-semantic-graph-wiring-dto-integration-e2e-validation
plan: "04"
subsystem: integration-tests
tags: [INT-05, INT-06, backward-compat, e2e, serde, 2026-smoke]
dependency_graph:
  requires: ["15-01"]
  provides: ["INT-05", "INT-06"]
  affects: []
tech_stack:
  added: []
  patterns: ["external-consumer black-box tests", "programmatic date-set assembly", "dedup scan over julian day range"]
key_files:
  created:
    - crates/amlich-core/tests/day_snapshot_v14_compat.rs
    - crates/amlich-core/tests/integration_2026_smoke.rs
  modified: []
decisions:
  - "INT-05 uses None-fielded clone to simulate v1.4 producer payload rather than hand-crafting JSON — avoids fragile string manipulation and exercises the actual serde(default) path"
  - "INT-06 date-set built programmatically via jd cursor over 2026 — Soc/Vong collection scans entire year and deduplicates by (lunar_month, is_leap) key; leap month 6 collection scans Jun-Sep 2026 window"
  - "lunar_month clamped to 1..=12 for fengshui API calls — domain invariants guarantee 1..=12 but clamp guard makes contract explicit and avoids any edge-case panic"
  - "Vun boundary straddle dates (2024-02-03 and 2024-02-05) included in main smoke loop AND in dedicated van_boundary_8_to_9 test for localization"
metrics:
  duration: "137 seconds (~2 min)"
  completed_date: "2026-05-28"
  tasks: 2
  files_changed: 2
---

# Phase 15 Plan 04: INT-05 + INT-06 Integration Tests Summary

INT-05 backward-compat serde round-trip and INT-06 2026 E2E calendar smoke across >=30 representative dates.

## Tasks Completed

| Task | Description | Commit | Files |
|------|-------------|--------|-------|
| 1 | INT-05: day_snapshot_v14_compat.rs — 3 backward-compat tests | 5eac552 | crates/amlich-core/tests/day_snapshot_v14_compat.rs |
| 2 | INT-06: integration_2026_smoke.rs — 3 E2E smoke tests | 5eac552 | crates/amlich-core/tests/integration_2026_smoke.rs |

## What Was Built

### INT-05: DaySnapshot v1.4 Backward-Compat Round-Trip

Three tests in `day_snapshot_v14_compat.rs`:

1. `v15_round_trip_byte_equal` — serialize a default-populated DaySnapshot (10, 2, 2024), deserialize, re-serialize; asserts both JSON strings are byte-equal.
2. `additive_fields_absent_when_none` — clones snapshot, sets `flying_stars = None` and `applicable_rituals = None`, serializes; asserts neither `"flying_stars"` nor `"applicable_rituals"` key appears in JSON.
3. `v14_json_without_new_fields_deserializes` — strips both new keys from a serialized snapshot (producing v1.4-shaped JSON), then `serde_json::from_str::<DaySnapshot>` succeeds; confirms `#[serde(default)]` lenience and that the recovered struct has both new fields as None.

### INT-06: 2026 E2E Calendar Smoke

Three test functions in `integration_2026_smoke.rs`:

1. `e2e_2026_smoke_all_categories` — assembles >=30 distinct dates across all 5 required categories, deduplicates, asserts `dates.len() >= 30`, then exercises all four pillar APIs per date:
   - `calculate_day_snapshot` (no panic)
   - `find_van_khan_for_snapshot` (no panic, result may be empty)
   - `compute_combined_overlay(year, lunar_month, &scanner).palace_overlays.len() == 9`
   - `compute_palace_aspects(year, lunar_month, &scanner).len() == 9`

2. `tet_2026_is_lunar_1_1` — asserts solar 2026-02-17 maps to lunar day=1, month=1, is_leap=false.

3. `van_boundary_8_to_9` — asserts `compute_period(jd_from_date(3,2,2024)).van == 8` and `compute_period(jd_from_date(5,2,2024)).van == 9`.

**Date categories covered:**
- Tet 2026: (17, 2, 2026)
- Soc x12 + Vong x12: collected by scanning solar 2026-01-01..2026-12-31, recording first hit per (lunar_month, is_leap) pair for lunar.day == 1 and == 15
- Van boundary straddle: (3,2,2024) + (5,2,2024)
- Leap month 6: 3 dates with lunar.month==6 && lunar.is_leap==true from Jun-Sep 2026 scan
- 24 Tiet Khi boundaries: TietKhiScanner::new().terms_for_year(2026).iter().map(|t| jd_to_date(t.jd))

## Test Results

```
test additive_fields_absent_when_none ... ok
test v14_json_without_new_fields_deserializes ... ok
test v15_round_trip_byte_equal ... ok
test result: ok. 3 passed; 0 failed; 0 ignored (INT-05)

test van_boundary_8_to_9 ... ok
test tet_2026_is_lunar_1_1 ... ok
test e2e_2026_smoke_all_categories ... ok
test result: ok. 3 passed; 0 failed; 0 ignored (INT-06)

Full suite: ok (all tests pass)
```

## Deviations from Plan

None — plan executed exactly as written. The library already had all the public API surface needed (15-01 landed FlyingStarsSummary + serde derives; 15-03 landed ritual/fengshui builder methods). Both test files compile and pass against the existing public API without any source modifications.

## Self-Check: PASSED

- `crates/amlich-core/tests/day_snapshot_v14_compat.rs` — FOUND
- `crates/amlich-core/tests/integration_2026_smoke.rs` — FOUND
- Commit `5eac552` — FOUND (`test(15-04): INT-05 backward-compat round-trip + INT-06 2026 E2E smoke`)
