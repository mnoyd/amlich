---
gsd_state_version: 1.0
milestone: v1.1
milestone_name: Foundation Extensions
status: unknown
last_updated: "2026-03-02T06:05:16.908Z"
progress:
  total_phases: 6
  completed_phases: 6
  total_plans: 12
  completed_plans: 12
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-02-28)
See: .planning/MILESTONES.md (updated 2026-03-02 - v1.0 complete)

**Core value:** Every almanac subsystem in amlich must produce output that matches KHCBPPT for 2020–2030 date range
**Milestone v1.0:** COMPLETE - All 4 phases finished, 0 divergences confirmed

## Current Position

**Milestone v1.0: COMPLETE** ✓
All 4 phases finished successfully:
- Phase 1: Source Establishment (2/2 plans)
- Phase 2: Golden Dataset and Loader (2/2 plans)
- Phase 3: Validator Harness and Divergence Inventory (3/3 plans)
- Phase 4: Correction and Zero-Divergence Verification (1/1 plan)

Status: All 7 KHCBPPT validators pass with 0 divergences; all 184 tests pass
Last activity: 2026-03-02 — Milestone v1.0 archived in MILESTONES.md

Progress: [██████████] 100%

## Performance Metrics

**Velocity:**
- Total plans completed: 8 (01-01, 01-02, 02-01, 02-02, 03-01, 03-02, 03-03, 04-01 all COMPLETE)
- Average duration: ~11.5 min (01-01: ~45 min, 01-02: ~14 min, 02-01: ~4 min, 02-02: ~2 min, 03-01: ~2 min, 03-02: ~2 min, 03-03: ~12 min, 04-01: ~10 min)
- Total execution time: ~92 min (1.5 hours)

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 01-source-establishment | 2 complete | ~60 min | ~30 min |
| 02-golden-dataset-and-loader | 2 complete | ~6 min | ~3 min |
| 03-validator-harness-and-divergence-inventory | 3 of 3 complete | ~16 min | ~5 min |
| 04-correction-and-zero-divergence-verification | 1 of 1 complete | ~10 min | ~10 min |

**Recent Trend:**
- All plans complete: Stable execution across all phases
- Manual research (Phase 1): ~30 min/plan (classical text work)
- Automation tasks (Phases 2-4): ~2-12 min/plan (well-specified validator tasks)
- Overall: Efficient and predictable execution

*Milestone v1.0 complete - no active planning*
| Phase v1.2 P01 | 7 min | 4 tasks | 4 files |

## Accumulated Context

### Decisions

Decisions are logged in PROJECT.md Key Decisions table.
Recent decisions affecting current work:
- [04-01]: All KHCBPPT subsystems verified — no data corrections needed; only star_meta.source_id metadata update applied
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
- [03-03]: Zero divergences expected for all subsystems — golden dataset was generated from get_day_info() output, so implementation matches itself by construction; Phase 4 will compare against actual KHCBPPT tables
- [03-01]: JD epoch verification uses first 5 golden entries as anchors (self-consistent since golden was generated from get_day_info()); star rule sparsity 233/233 confirms contextual buckets empty
- [03-01]: Taboo comparison is set-based (HashSet) not Vec-based — avoids false failures due to ordering differences between golden and impl
- [03-03]: Phase 3 is purely inventory — 192 tests pass, all 7 validators operational, no source/data file changes
- [Phase v1.2]: Reuse a shared HeavenlyStem enum with TryFrom<&str> for explicit invalid-input handling
- [Phase v1.2]: Encode Ten Gods output as ThapThanResult with relation, polarity, and evidence metadata
- [Phase v1.2]: Lock mapping correctness with explicit 10x10 expected-label matrix tests

### Pending Todos

None yet.

### Blockers/Concerns

**None** - All blockers from v1.0 resolved:
- ✓ KHCBPPT edition pinned (Phase 1)
- ✓ Star rule completeness documented (Phase 3 - sparsity report)
- ✓ Truc quality verified correct (Phase 4 - no correction needed)

## Session Continuity

Last session: 2026-03-02
Stopped at: Milestone v1.0 complete — All 4 phases finished, 0 divergences confirmed, MILESTONES.md archived
Next: Execute v1.1 Foundation Extensions; v1.2 Ten Gods and Kua Foundation queued

## Milestone Queue

- **Current execution target:** v1.1 Foundation Extensions (already planned in ROADMAP.md)
- **Next milestone queued:** v1.2 Ten Gods and Kua Foundation
  - Scope confirmed: Thập Thần engine, Tứ Mệnh calculations, DayFortune/API/test integration
  - Explicit defer: Đại Vận to v1.3
  - Requirements file: `.planning/REQUIREMENTS-v1.2.md`
  - Roadmap entry: `Phase v1.2` in `.planning/ROADMAP.md`

**Verification Commands:**
```bash
# Verify all tests pass (expected: 184/184)
cargo test --package amlich-core

# Run KHCBPPT validators only (expected: 10 tests, 0 divergences)
cargo test --package amlich-core khcbppt
```
