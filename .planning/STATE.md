# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-02-28)

**Core value:** Every almanac subsystem in amlich must produce output that matches KHCBPPT for the 2020–2030 date range
**Current focus:** Phase 3 IN PROGRESS — Validator Harness and Divergence Inventory

## Current Position

Phase: 3 of 4 (Validator Harness and Divergence Inventory) — IN PROGRESS
Plan: 2 of 3 in current phase — 03-01 COMPLETE; 03-02 COMPLETE; 03-03 pending
Status: Phase 3 plan 02 complete — deity, truc, and xung hop validators all passing with 0 divergences
Last activity: 2026-03-01 — Plan 03-02 complete (deity with enum-to-str, truc 3-field, xung hop sorted Vec)

Progress: [████████░░] 75%

## Performance Metrics

**Velocity:**
- Total plans completed: 6 (01-01, 01-02, 02-01, 02-02, 03-01, 03-02 all COMPLETE)
- Average duration: ~11 min (01-01: ~45 min, 01-02: ~14 min, 02-01: ~4 min, 02-02: ~2 min, 03-01: ~2 min, 03-02: ~2 min)
- Total execution time: ~1 hour 10 min

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 01-source-establishment | 2 complete | ~60 min | ~30 min |
| 02-golden-dataset-and-loader | 2 complete | ~6 min | ~3 min |
| 03-validator-harness-and-divergence-inventory | 2 of 3 complete | ~4 min | ~2 min |

**Recent Trend:**
- Last 5 plans: 01-02 (~14 min), 02-01 (~4 min), 02-02 (~2 min), 03-01 (~2 min), 03-02 (~2 min)
- Trend: Stable fast — well-specified validator tasks execute in ~2 min each

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
- [02-02]: Combined TDD execution: loader + validation + tests written together (Rust requires co-compilation)
- [02-02]: Validation follows data.rs pattern: panic on invariant violation during OnceLock::get_or_init()
- [03-02]: Deity Option<None> handled as mismatch string ("expected X, got NONE") not panic — consistent with collect-then-assert needing all 233 entries to process
- [03-02]: Sorted Vec used for tam_hop/tu_hanh_xung comparison (not HashSet) — preserves duplicate detection while being order-independent

### Pending Todos

None yet.

### Blockers/Concerns

- Phase 1 is entirely manual research — KHCBPPT edition must be pinned before any automated work is possible
- Star rule completeness: baseline.json contextual buckets have only 1 entry each; may indicate missing rules, not just incorrect values — Phase 3 validator must detect missing rules, not just mismatches
- Trực quality (`TRUC_QUALITY` const in `truc.rs`) is hardcoded in Rust source, not JSON — correction in Phase 4 requires code change + recompile

## Session Continuity

Last session: 2026-03-01
Stopped at: Completed 03-02-PLAN.md — deity, truc, and xung hop validators passing with 0 divergences
Resume file: Phase 3 Plan 03 — stars, than_huong, na_am validators
