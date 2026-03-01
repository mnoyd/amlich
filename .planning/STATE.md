# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-02-28)

**Core value:** Every almanac subsystem in amlich must produce output that matches KHCBPPT for the 2020–2030 date range
**Current focus:** Phase 2 — Golden Dataset and Loader

## Current Position

Phase: 2 of 4 (Golden Dataset and Loader) — IN PROGRESS
Plan: 1 of 2 in current phase — 02-01 COMPLETE; 02-02 pending
Status: Plan 02-01 complete — GoldenEntry structs defined, 233-entry khcbppt-golden.json generated
Last activity: 2026-03-01 — Plan 02-01 complete (structs + golden dataset + coverage tests)

Progress: [██████░░░░] 60%

## Performance Metrics

**Velocity:**
- Total plans completed: 3 (01-01 COMPLETE, 01-02 COMPLETE, 02-01 COMPLETE)
- Average duration: ~21 min (01-01: ~45 min, 01-02: ~14 min, 02-01: ~4 min)
- Total execution time: ~1 hour 4 min

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 01-source-establishment | 2 complete | ~60 min | ~30 min |
| 02-golden-dataset-and-loader | 1 complete | ~4 min | ~4 min |

**Recent Trend:**
- Last 5 plans: 01-01 (~45 min), 01-02 (~14 min tasks + checkpoint), 02-01 (~4 min)
- Trend: Accelerating — automated tasks execute faster than manual research

*Updated after each plan completion*

## Accumulated Context

### Decisions

Decisions are logged in PROJECT.md Key Decisions table.
Recent decisions affecting current work:

- [Pre-phase]: KHCBPPT as sole reference — RESOLVED: ctext.org 四庫全書 primary edition identified
- [01-01]: SRC-02 RESOLVED — KHCBPPT covers 納音 in Bon Nguyen section; source_id stays "tam-menh-thong-hoi" (canonical table identical across both sources; Vietnamese almanac convention)
- [01-01]: Primary edition = ctext.org 四庫全書 digitization (Qianlong 1741); Secondary = 1998 NXB Mui Ca Mau Vietnamese translation (Mai Coc Thanh, Vu Hoang, Lan Binh)
- [01-01]: Citation format defined: "KHCBPPT, Quyen [N], [Section name]" at chapter+section granularity
- [01-01]: All 30 nap am pairs verified correct against canonical 六十甲子納音表; commit 0f29f3f corrections (Kim Bac Kim, Dai Dich Tho) confirmed by Chinese character evidence
- [01-02]: SRC-03 RESOLVED — KHCBPPT Nguyet Bieu (vols 20-31) has 12 volumes for 12 months; no intercalary supplement; silence implies base-month inheritance for taboo and truc rules
- [01-02]: 28-star JD epoch NOT defined in KHCBPPT — jd.rem_euclid(28) epoch is Ho Ngoc Duc implementation artifact; confidence LOW for epoch correctness
- [01-02]: TRUC_QUALITY const confirmed correct per KHCBPPT Nghia Le; Tru (cat) and Nguy (hung) contested values documented with KHCBPPT position
- [01-02]: All 6 commit 0f29f3f Thanh Huong corrections confirmed against KHCBPPT Lap Thanh tables
- [01-02]: star_meta.source_id should change from "nhi-thap-bat-tu" to "khcbppt" in Phase 4
- [01-02]: Star rule sparsity: fixed_by_chi complete; 4 other categories have only 1 seed entry each — Phase 3 must detect absence, not just value mismatch
- [02-01]: 233-entry golden dataset generated with coverage-driven algorithm; all subsystem values from get_day_info() (Phase 1 confirmed correctness)
- [02-01]: Star entries marked MEDIUM confidence for JD epoch; all other subsystems HIGH confidence
- [02-01]: Generator as #[ignore] test for reproducible regeneration: cargo test --test generate_golden -- --ignored

### Pending Todos

None yet.

### Blockers/Concerns

- Phase 1 is entirely manual research — KHCBPPT edition must be pinned before any automated work is possible
- Star rule completeness: baseline.json contextual buckets have only 1 entry each; may indicate missing rules, not just incorrect values — Phase 3 validator must detect missing rules, not just mismatches
- Trực quality (`TRUC_QUALITY` const in `truc.rs`) is hardcoded in Rust source, not JSON — correction in Phase 4 requires code change + recompile

## Session Continuity

Last session: 2026-03-01
Stopped at: Completed 02-01-PLAN.md — GoldenEntry structs + 233-entry golden dataset
Resume file: Continue Phase 2 — Plan 02-02 (wire golden loader with include_str!, validation, test coverage)
