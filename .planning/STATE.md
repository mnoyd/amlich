---
gsd_state_version: 1.0
milestone: v1.6
milestone_name: Integration
status: in_progress
last_updated: "2026-07-15T13:35:07Z"
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

**Current focus:** v1.6 Eastern Knowledge Completion — Phase 18 Daily Phi Tinh in progress (Plan 18-01 complete; 18-02 next).

## Current Position

Milestone: v1.6 Eastern Knowledge Completion.
Phase: 18 of 4 planned (Phase 18: Daily Phi Tinh, 4 plans).
Plan: 18-01 COMPLETE (FS-17 schema-lock/ADR closure). Next: 18-02 algorithm (`compute_daily_flying_stars`).
Status: ADR-0004 is accepted with the 6 Trung Khí pivot table, Dương-thuận/Âm-nghịch direction rule, Giáp-Tý seed mechanic, *Thẩm Thị Huyền Không Học* chapter+verse citation, page-level citation deferral note, and 3 rejected alternative conventions. `FlyingStarPeriod` gained the additive `Daily { date: (i32, u32, u32) }` variant and `DailyFlyingStarLayout` was added as a sibling to the frozen `FlyingStarLayout` field set; `almanac::fengshui` re-exports it for external-crate tests.
Last activity: 2026-07-15 — 18-01-PLAN.md executed: ADR-0004 authored (commit b2265eb) and schema lock added in `types.rs`/`mod.rs` (commit a593a13). `cargo build -p amlich-core` clean; `cargo test -p amlich-core --lib almanac::fengshui::types::tests` 6/6 pass. FS-17 closed; FS-16/18/19 remain for 18-02/03/04.

Progress: [▓▓▓▓▓░░░░░] 50% (v1.6: 2 of 4 phases complete; 5 of 11 plans complete; Phase 18 is 1/4).

## Key Decisions Added in 18-01

- ADR-0004 locks daily Phi Tinh to 6 Trung Khí pivots with Dương→thuận and Âm→nghịch direction; this is intentionally opposite ADR-0003's annual polarity rule.
- Daily pivot seeds take effect at the first Giáp Tý with JD >= pivot_jd, not at the pivot instant itself; pre-Giáp-Tý days remain under the prior pivot.
- The frozen v1 `FlyingStarLayout` remains unchanged; daily schema uses the additive `FlyingStarPeriod::Daily { date: (i32, u32, u32) }` variant plus sibling `DailyFlyingStarLayout`.

## v1.6 Target Features

1. **Daily Flying Star (日紫白)** — ADR/schema lock done (FS-17); algorithm, golden dataset, and DaySnapshot field remain (FS-16/18/19).
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

## Session Continuity

Last session: 2026-07-15T13:35:07Z
Stopped at: Completed 18-01-PLAN.md. Phase 18 plan 1 of 4 executed (FS-17 closed).
Resume file: None.

### Next Step

Continue with Phase 18 plan 18-02 only: implement `compute_daily_flying_stars` against the ADR-0004 / `DailyFlyingStarLayout` schema lock. Do not skip to 18-03/18-04 until 18-02 is complete.

---
*State updated: 2026-07-15 after 18-01-PLAN.md executed (FS-17 closed; Phase 18 1/4 complete; build clean; fengshui types tests 6/6 pass).*
