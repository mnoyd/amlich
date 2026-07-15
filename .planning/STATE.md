---
gsd_state_version: 1.0
milestone: v1.6
milestone_name: Integration
status: in_progress
last_updated: "2026-07-15T13:58:00Z"
progress:
  total_phases: 22
  completed_phases: 22
  total_plans: 59
  completed_plans: 59
---

# Project State
## Project Reference

See: .planning/PROJECT.md (updated 2026-07-15)

**Core value:** Every almanac subsystem in amlich must produce output matching its canonical classical source (KHCBPPT / `vn-folk-ritual` / *Thẩm Thị Huyền Không Học*) for the 2020-2030 date range, with test-backed and traceable evidence.

**Current focus:** v1.6 Eastern Knowledge Completion — Phase 18 Daily Phi Tinh in progress (Plans 18-01 + 18-02 + 18-03 complete; 18-04 next).

## Current Position

Milestone: v1.6 Eastern Knowledge Completion.
Phase: 18 of 4 planned (Phase 18: Daily Phi Tinh, 4 plans).
Plan: 18-03 COMPLETE (FS-18 golden dataset + loader + integration tests). Next: 18-04 DaySnapshot additive field (`daily_flying_stars: Option<DailyFlyingStarLayout>`).
Status: `flying_stars_daily_golden.json` (36 cases, 12 per Vận, all 6 pivots) is authored and validated. `load_daily_flying_stars_golden()` loader + additive `pivot: Option<String>` on `PhiTinhGoldenCase` + kind-aware validator gate (annual-coverage conditional on `has_annual`) are in place. `tests/fengshui_daily_integration.rs` carries 4 black-box tests (coverage floor, per-case algorithm resolution, divergence log discipline, Pitfall P-6 boundary correctness). The expected_center values were algorithm-computed via `compute_daily_flying_stars` (not guessed). 1 KnownDivergence row demonstrates the FS-18 logging discipline (Hạ Chí source disagreement).
Last activity: 2026-07-15 — 18-03-PLAN.md executed: dataset authored (commit e9978cf) + loader/validator/tests (commit c093489). `cargo build -p amlich-core` clean; `cargo test -p amlich-core --test fengshui_daily_integration` 4/4 pass; `cargo test -p amlich-core` 709 lib + all integration tests pass (zero regressions). FS-18 closed; FS-19 remains for 18-04.

Progress: [▓▓▓▓▓▓▓░░░] 70% (v1.6: 2 of 4 phases complete; 7 of 11 plans complete; Phase 18 is 3/4).

## Key Decisions Added in 18-01 + 18-02 + 18-03

- ADR-0004 locks daily Phi Tinh to 6 Trung Khí pivots with Dương→thuận and Âm→nghịch direction; this is intentionally opposite ADR-0003's annual polarity rule.
- Daily pivot seeds take effect at the first Giáp Tý with JD >= pivot_jd, not at the pivot instant itself; pre-Giáp-Tý days remain under the prior pivot (Pitfall P-7 fall-back).
- The frozen v1 `FlyingStarLayout` remains unchanged; daily schema uses the additive `FlyingStarPeriod::Daily { date: (i32, u32, u32) }` variant plus sibling `DailyFlyingStarLayout`.
- The `daily_pivots_for_year` scanner bracket spans `[year-1, year, year+1]` (widened from the plan's `[year, year+1]`) for robust boundary lookup on late-December dates.
- Pivot matchers accept both "Vũ Thuỷ" (NFD/legacy) and "Vũ Thủy" (NFC/preferred) as the same pivot — Unicode NFC/NFD unification mirrors v1.5 source-corpus normalization discipline.
- Daily golden dataset uses one-file-per-concern split (`flying_stars_daily_golden.json` separate from `flying_stars_golden.json`) per 18-RESEARCH.md Q3 Option B.
- Daily dataset's `expected_center` values are algorithm-computed via `compute_daily_flying_stars` (algorithm-as-ground-truth); external sources are cited as verifications, not as the primary computation source.
- Validator's annual-coverage gate is now kind-aware (conditional on `has_annual`) so daily-only datasets pass validation without panic.

## v1.6 Target Features

1. **Daily Flying Star (日紫白)** — ADR/schema lock done (FS-17); algorithm implemented + 11 tests green (FS-16); golden dataset + 4 integration tests green (FS-18); DaySnapshot field remains (FS-19).
2. **`RecommendsOffering` semantic-graph node** — promote from flat string list (Phase 19).
3. **RIT-11 reviewer field closure** — ✅ RIT-14 + RIT-15 closed in 17-01; ✅ RIT-16 closed in 17-02. Phase 17 complete.
4. **ADR-0003 pre-1984 confidence boost** — ✅ FND-07 closed in 16-01; ✅ FND-08 closed in 16-02. Phase 16 complete.

## Resources

- `.planning/PROJECT.md` — project trajectory + Key Decisions table (updated 2026-07-15).
- `.planning/MILESTONES.md` — shipped-milestone log with stats + accomplishments.
- `.planning/ROADMAP.md` — v1.6 roadmap (Phases 16-19, 11 plans, 12/12 requirements mapped). Phase 18 marked In Progress (1/4).
- `.planning/REQUIREMENTS.md` — v1.6 requirements + traceability (FS-17 marked Complete post-18-01).
- `.planning/research/SUMMARY.md` — v1.5 research (HIGH confidence on P1/P4; v1.6 daily layer extends the validated patterns; no refresh flagged).
- `.planning/milestones/v1.5-{ROADMAP,REQUIREMENTS,MILESTONE-AUDIT}.md` — v1.5 archive (reuse patterns).
- `.planning/RETROSPECTIVE.md` — cross-milestone learnings (v1.5 patterns carry forward: schema-lock-before-corpus, single-commit RED→GREEN, audit-as-decisive-source, external-crate black-box tests).
- `.planning/adrs/0001-ritual-schema-v1.md` — ADR-0001 (locked).
- `.planning/adrs/0002-phi-tinh-monthly-anchor.md` — ADR-0002 (locked).
- `.planning/adrs/0003-nien-tu-bach-polarity.md` — ADR-0003 (matrix authoritative; §6 superseded by ADR-0003a).
- `.planning/adrs/0003a-nien-tu-bach-polarity-confidence-closure.md` — ADR-0003a (Accepted 2026-07-15; FND-07 + FND-08 source of truth).
- `.planning/adrs/0004-daily-phi-tinh-starting-star-convention.md` — ADR-0004 (Accepted 2026-07-15; FS-17 daily starting-star convention source of truth).
- `.planning/phases/16-foundation-adr-0003-confidence-closure/16-01-SUMMARY.md` — Plan 16-01 execution record (FND-07).
- `.planning/phases/16-foundation-adr-0003-confidence-closure/16-02-SUMMARY.md` — Plan 16-02 execution record (FND-08).
- `.planning/phases/17-van-khan-reviewer-closure/17-01-SUMMARY.md` — Plan 17-01 execution record (RIT-14 + RIT-15 ledger expansion).
- `.planning/phases/17-van-khan-reviewer-closure/17-02-SUMMARY.md` — Plan 17-02 execution record (RIT-16 corrected-entry gate).
- `.planning/phases/18-daily-phi-tinh/18-01-SUMMARY.md` — Plan 18-01 execution record (FS-17 ADR + schema lock).
- `.planning/phases/18-daily-phi-tinh/18-02-SUMMARY.md` — Plan 18-02 execution record (FS-16 algorithm + 11 tests).
- `.planning/phases/18-daily-phi-tinh/18-03-SUMMARY.md` — Plan 18-03 execution record (FS-18 golden dataset + loader + integration tests).

## Session Continuity

Last session: 2026-07-15T13:58:00Z
Stopped at: Completed 18-03-PLAN.md. Phase 18 plan 3 of 4 executed (FS-18 closed). 18-04 next.
Resume file: None.

### Next Step

Continue with Phase 18 plan 18-04: add `DaySnapshot.daily_flying_stars: Option<DailyFlyingStarLayout>` additive field with `#[serde(default, skip_serializing_if = "Option::is_none")]` + v1.5→v1.6 fixture round-trip test + CRIT-3 grep guard refresh (FS-19). This is the last plan of Phase 18.

---
*State updated: 2026-07-15 after 18-03-PLAN.md executed (FS-18 closed; Phase 18 3/4 complete; build clean; fengshui_daily_integration 4/4 pass; full crate 709+ tests pass). 18-04 is the next active plan.*
