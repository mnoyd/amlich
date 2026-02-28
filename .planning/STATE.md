# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-02-28)

**Core value:** Every almanac subsystem in amlich must produce output that matches KHCBPPT for the 2020–2030 date range
**Current focus:** Phase 1 — Source Establishment

## Current Position

Phase: 1 of 4 (Source Establishment)
Plan: 2 of 2 in current phase (01-01 COMPLETE; 01-02 PAUSED at Task 3 checkpoint:human-verify)
Status: Paused — Plan 01-02 Tasks 1–2 complete; awaiting human verification of 6 reference files
Last activity: 2026-02-28 — Plan 01-02 Tasks 1+2: 6 reference files created (commits 972515c, 3c88c20)

Progress: [███░░░░░░░] 25%

## Performance Metrics

**Velocity:**
- Total plans completed: 1 (01-02 in progress — paused at Task 3 checkpoint)
- Average duration: ~45 min (01-01)
- Total execution time: ~0.75 hours

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 01-source-establishment | 1 complete | ~45 min | ~45 min |

**Recent Trend:**
- Last 5 plans: 01-01 (~45 min)
- Trend: Single data point

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

### Pending Todos

None yet.

### Blockers/Concerns

- Phase 1 is entirely manual research — KHCBPPT edition must be pinned before any automated work is possible
- Star rule completeness: baseline.json contextual buckets have only 1 entry each; may indicate missing rules, not just incorrect values — Phase 3 validator must detect missing rules, not just mismatches
- Trực quality (`TRUC_QUALITY` const in `truc.rs`) is hardcoded in Rust source, not JSON — correction in Phase 4 requires code change + recompile

## Session Continuity

Last session: 2026-02-28
Stopped at: Plan 01-02 Task 3 — checkpoint:human-verify (6 reference files created; awaiting user approval of reference table content)
Resume file: .planning/phases/01-source-establishment/01-02-PLAN.md (Task 3 checkpoint — user must verify reference files and respond "approved" or with specific issues)
