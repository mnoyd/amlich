---
phase: 12-van-khan-corpus-authoring
plan: "01"
subsystem: rituals/corpus
tags: [corpus-authoring, van-khan, json-data, rit-09, rit-10, rit-12]
dependency_graph:
  requires: [11-04]
  provides: [corpus-batch-1-spring-summer]
  affects: [12-03-loader-wiring, 12-04-provenance-ledger]
tech_stack:
  added: []
  patterns: [rituals-v1-schema, multi-variant-event-entries, solar-term-key-for-thanh-minh]
key_files:
  created:
    - crates/amlich-core/data/rituals/tet-nguyen-dan.json
    - crates/amlich-core/data/rituals/doan-ngo.json
    - crates/amlich-core/data/rituals/nguyen-tieu.json
    - crates/amlich-core/data/rituals/han-thuc.json
    - crates/amlich-core/data/rituals/thanh-minh.json
    - crates/amlich-core/data/rituals/phat-dan.json
  modified: []
decisions:
  - "Expanded nguyen-tieu, han-thuc, thanh-minh to 5 entries each (from planned 2-3) to reach >=26 total"
  - "thanh-minh entries use solar_term key exclusively — no holiday_id (consistent with holidays.rs:177 None assignment)"
  - "phat-dan extended to 4 variants (simple/buddhist/full/folk) for RIT-12 coverage"
  - "nguyen-tieu extended to 5 variants including regional:mien-nam for regional coverage"
metrics:
  duration: "9 min"
  completed_date: "2026-05-27"
  tasks_completed: 2
  files_changed: 6
---

# Phase 12 Plan 01: Văn Khấn Corpus — Spring/Summer Festival Batch Summary

**One-liner:** 26 ADR-0001-conformant ritual entries across 6 spring/summer festival JSON files, establishing 4-variant Tết and 3-variant Đoan Ngọ multi-event coverage for RIT-12.

## What Was Built

6 new `$schema_version: "rituals-v1"` JSON corpus files under `crates/amlich-core/data/rituals/`, shipping **26 RitualEntry records** in this batch (plan target: ≥ 26).

### Files Created

| File | Entries | Variants Authored | Event Key Type |
|------|---------|-------------------|----------------|
| tet-nguyen-dan.json | 4 | simple, full, buddhist, folk | holiday_id + lunar_date |
| nguyen-tieu.json | 5 | simple, full, buddhist, folk, regional:mien-nam | holiday_id + lunar_date |
| han-thuc.json | 5 | simple, full, folk, buddhist, regional:mien-bac | holiday_id + lunar_date |
| thanh-minh.json | 5 | simple, full, folk, regional:mien-trung, regional:mien-bac | solar_term ONLY |
| doan-ngo.json | 3 | simple, folk, regional:mien-bac | holiday_id + lunar_date |
| phat-dan.json | 4 | simple, buddhist, full, folk | holiday_id + lunar_date |
| **Total** | **26** | | |

### Ritual IDs in This Plan

**tet-nguyen-dan.json:**
- `van-khan-tet-don-gian` (simple) — anchor id used by rituals_integration Test 1
- `van-khan-tet-day-du` (full)
- `van-khan-tet-phat-giao` (buddhist)
- `van-khan-tet-dan-gian` (folk)

**nguyen-tieu.json:**
- `van-khan-nguyen-tieu-don-gian` (simple)
- `van-khan-ram-thang-gieng` (full) — anchor id used by rituals_integration Test 2
- `van-khan-thuong-nguyen-phat-giao` (buddhist)
- `van-khan-nguyen-tieu-dan-gian` (folk)
- `van-khan-nguyen-tieu-mien-nam` (regional:mien-nam)

**han-thuc.json:**
- `van-khan-han-thuc-don-gian` (simple)
- `van-khan-han-thuc-day-du` (full)
- `van-khan-han-thuc-dan-gian` (folk)
- `van-khan-han-thuc-phat-giao` (buddhist)
- `van-khan-han-thuc-mien-bac` (regional:mien-bac)

**thanh-minh.json:**
- `van-khan-thanh-minh` (simple) — anchor id used by rituals_integration Test 3
- `van-khan-thanh-minh-day-du` (full)
- `van-khan-thanh-minh-mien-trung` (regional:mien-trung)
- `van-khan-thanh-minh-dan-gian` (folk)
- `van-khan-thanh-minh-mien-bac` (regional:mien-bac)

**doan-ngo.json:**
- `van-khan-doan-ngo` (simple) — anchor id used by rituals_integration Test fixture
- `van-khan-doan-ngo-dan-gian` (folk)
- `van-khan-doan-ngo-mien-bac` (regional:mien-bac)

**phat-dan.json:**
- `van-khan-phat-dan-don-gian` (simple)
- `van-khan-phat-dan-phat-giao` (buddhist)
- `van-khan-phat-dan-day-du` (full)
- `van-khan-phat-dan-dan-gian` (folk)

## Success Criteria Verification

- [x] 6 JSON files exist under `crates/amlich-core/data/rituals/`, each with `$schema_version: "rituals-v1"`
- [x] 26 RitualEntry records total (>= 26 required)
- [x] Tết Nguyên Đán: exactly 4 variants (simple/full/buddhist/folk) with identical event_keys
- [x] Tết Đoan Ngọ: 3 variants (simple/folk/regional:mien-bac) with identical event_keys
- [x] Every entry has source_id `vn-folk-ritual`, original_citation.page populated, valid confidence tier
- [x] Thanh Minh entries use solar_term key only — no holiday_id (holidays.rs:177 id:None)
- [x] Zero Hán characters in all 6 files

## RIT-12 Multi-Variant Coverage Contributed

This plan contributes 4 events with multi-variant coverage:
1. Tết Nguyên Đán — 4 variants (simple/full/buddhist/folk)
2. Tết Đoan Ngọ — 3 variants
3. Tết Nguyên Tiêu — 5 variants
4. Hàn Thực — 5 variants

## Deviations from Plan

### Auto-expanded entry counts (Rule 2 — missing critical functionality)

The plan's minimum entry distribution (Task 1: 7, Task 2: min 2+2+3+2 = 9) would have produced only 16 entries — well below the 26 minimum. The files were expanded:

- `nguyen-tieu.json`: planned ≥ 2, delivered 5 (added folk + regional:mien-nam variants)
- `han-thuc.json`: planned ≥ 2, delivered 5 (added buddhist + regional:mien-bac variants)
- `thanh-minh.json`: planned ≥ 3, delivered 5 (added folk + regional:mien-bac variants)
- `phat-dan.json`: planned ≥ 2, delivered 4 (added folk variant)

This expansion also strengthened RIT-12 (more events with >= 2 variants) and RIT-10 coverage quality.

### No changes to existing codebase

This plan is data-only — no Rust code changes, no corpus.rs modifications (those are wired by plan 12-03 in wave 2). The existing `fixtures.json` is preserved; plan 12-03 will decide the loader migration strategy.

## Commits

| Task | Commit | Files |
|------|--------|-------|
| Task 1: Tết + Đoan Ngọ | 61d81f1 | tet-nguyen-dan.json, doan-ngo.json |
| Task 2: Nguyên Tiêu + Hàn Thực + Thanh Minh + Phật Đản | 1be9486 | nguyen-tieu.json, han-thuc.json, thanh-minh.json, phat-dan.json |

## Self-Check: PASSED
