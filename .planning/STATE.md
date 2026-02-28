# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-02-28)

**Core value:** Every almanac subsystem in amlich must produce output that matches KHCBPPT for the 2020–2030 date range
**Current focus:** Phase 1 — Source Establishment

## Current Position

Phase: 1 of 4 (Source Establishment)
Plan: 2 of 2 in current phase (01-01 COMPLETE; 01-02 next)
Status: Ready — Plan 01-01 complete, Plan 01-02 (reference table extraction) not yet started
Last activity: 2026-02-28 — Plan 01-01 complete: EDITION.md + na_am.md created (commits 0b9dd46, 9bbcdf8)

Progress: [██░░░░░░░░] 15%

## Performance Metrics

**Velocity:**
- Total plans completed: 1
- Average duration: ~45 min
- Total execution time: ~0.75 hours

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 01-source-establishment | 1 | ~45 min | ~45 min |

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

### Pending Todos

None yet.

### Blockers/Concerns

- Phase 1 is entirely manual research — KHCBPPT edition must be pinned before any automated work is possible
- Star rule completeness: baseline.json contextual buckets have only 1 entry each; may indicate missing rules, not just incorrect values — Phase 3 validator must detect missing rules, not just mismatches
- Trực quality (`TRUC_QUALITY` const in `truc.rs`) is hardcoded in Rust source, not JSON — correction in Phase 4 requires code change + recompile

## Session Continuity

Last session: 2026-02-28
Stopped at: Plan 01-01 COMPLETE — all tasks executed, SUMMARY.md created, STATE.md updated
Resume file: .planning/phases/01-source-establishment/01-02-PLAN.md (Plan 02: Reference table extraction — Wave 2)
