---
gsd_state_version: 1.0
milestone: v1.6
milestone_name: Integration
status: in_progress
last_updated: "2026-07-15T14:04:45Z"
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

**Current focus:** v1.6 Eastern Knowledge Completion — Phase 18 Daily Phi Tinh COMPLETE (all 4 plans + FS-16/17/18/19 closed). Next: Phase 19 RecommendsOffering.

## Current Position

Milestone: v1.6 Eastern Knowledge Completion.
Phase: 18 of 4 planned (Phase 18: Daily Phi Tinh, 4 plans) — COMPLETE.
Plan: 18-04 COMPLETE (FS-19 DaySnapshot additive field). Next: Phase 19 plan 19-01 (`OfferingRef` schema + additive `offering_refs` on Ritual node payload, INT-08).
Status: `DaySnapshot.daily_flying_stars: Option<DailyFlyingStarLayout>` additive field on the v1.6 DTO with the EXACT serde pattern as `flying_stars` / `applicable_rituals`. `calculate_day_snapshot_internal` auto-populates it via `compute_daily_flying_stars` (solar Y/M/D extracted from `snap.context.solar`). `tests/day_snapshot_v14_compat.rs` extended with 3 round-trip tests (v1.5→v1.6 backward-compat, byte-equal round-trip, None absent in JSON). `tests/fengshui_crit3_isolation.rs` authored with 1 grep guard asserting `interaction/direction_merge.rs` contains none of 6 forbidden Phi Tinh patterns (CRIT-3 / P-1 isolation discipline preserved).
Last activity: 2026-07-15 — 18-04-PLAN.md executed: DaySnapshot field + auto-populate (commit defe59e) + round-trip tests + CRIT-3 grep guard (commit e655140). `cargo build -p amlich-core` clean; `cargo test -p amlich-core --test day_snapshot_v14_compat` 6/6 pass; `cargo test -p amlich-core --test fengshui_crit3_isolation` 1/1 pass; `cargo test -p amlich-core` 709 lib + all integration tests pass (zero regressions). FS-19 closed; Phase 18 complete (4/4 plans).

Progress: [▓▓▓▓▓▓▓▓░░] 80% (v1.6: 3 of 4 phases complete; 8 of 11 plans complete; Phase 18 is 4/4).

## Key Decisions Added in 18-01 + 18-02 + 18-03 + 18-04

- ADR-0004 locks daily Phi Tinh to 6 Trung Khí pivots with Dương→thuận and Âm→nghịch direction; this is intentionally opposite ADR-0003's annual polarity rule.
- Daily pivot seeds take effect at the first Giáp Tý with JD >= pivot_jd, not at the pivot instant itself; pre-Giáp-Tý days remain under the prior pivot (Pitfall P-7 fall-back).
- The frozen v1 `FlyingStarLayout` remains unchanged; daily schema uses the additive `FlyingStarPeriod::Daily { date: (i32, u32, u32) }` variant plus sibling `DailyFlyingStarLayout`.
- The `daily_pivots_for_year` scanner bracket spans `[year-1, year, year+1]` (widened from the plan's `[year, year+1]`) for robust boundary lookup on late-December dates.
- Pivot matchers accept both "Vũ Thuỷ" (NFD/legacy) and "Vũ Thủy" (NFC/preferred) as the same pivot — Unicode NFC/NFD unification mirrors v1.5 source-corpus normalization discipline.
- Daily golden dataset uses one-file-per-concern split (`flying_stars_daily_golden.json` separate from `flying_stars_golden.json`) per 18-RESEARCH.md Q3 Option B.
- Daily dataset's `expected_center` values are algorithm-computed via `compute_daily_flying_stars` (algorithm-as-ground-truth); external sources are cited as verifications, not as the primary computation source.
- Validator's annual-coverage gate is now kind-aware (conditional on `has_annual`) so daily-only datasets pass validation without panic.
- `DaySnapshot.daily_flying_stars` uses the EXACT serde additive pattern as `flying_stars` / `applicable_rituals`; populate block sits BETWEEN the two existing blocks for readability; solar Y/M/D extracted from `snap.context.solar` to match the snapshot's own context.
- `tests/fengshui_crit3_isolation.rs` is semantically distinct from `tests/source_id_guard.rs` — the former forbids Phi Tinh TYPE NAMES leaking into `direction_merge.rs`; the latter forbids bare source_id STRING LITERALS. Both guards are complementary.

## v1.6 Target Features

1. **Daily Flying Star (日紫白)** — ✅ ADR/schema lock done (FS-17); ✅ algorithm + 11 tests green (FS-16); ✅ golden dataset + 4 integration tests green (FS-18); ✅ DaySnapshot field + CRIT-3 grep guard green (FS-19). Phase 18 complete.
2. **`RecommendsOffering` semantic-graph node** — promote from flat string list (Phase 19, next).
3. **RIT-11 reviewer field closure** — ✅ RIT-14 + RIT-15 closed in 17-01; ✅ RIT-16 closed in 17-02. Phase 17 complete.
4. **ADR-0003 pre-1984 confidence boost** — ✅ FND-07 closed in 16-01; ✅ FND-08 closed in 16-02. Phase 16 complete.

## Resources

- `.planning/PROJECT.md` — project trajectory + Key Decisions table (updated 2026-07-15).
- `.planning/MILESTONES.md` — shipped-milestone log with stats + accomplishments.
- `.planning/ROADMAP.md` — v1.6 roadmap (Phases 16-19, 11 plans, 12/12 requirements mapped). Phase 18 marked Complete (4/4).
- `.planning/REQUIREMENTS.md` — v1.6 requirements + traceability (FS-19 marked Complete post-18-04).
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
- `.planning/phases/18-daily-phi-tinh/18-04-SUMMARY.md` — Plan 18-04 execution record (FS-19 DaySnapshot additive field + CRIT-3 grep guard).

## Session Continuity

Last session: 2026-07-15T14:04:45Z
Stopped at: Completed 18-04-PLAN.md. Phase 18 plan 4 of 4 executed (FS-19 closed; Phase 18 4/4 complete). Phase 19 (RecommendsOffering) next.
Resume file: None.

### Next Step

Run phase verification on Phase 18 (all 4 plans complete; FS-16/17/18/19 closed). Then start Phase 19 plan 19-01: schema-first `OfferingRef` struct + additive `offering_refs: Option<Vec<OfferingRef>>` on the `Ritual` semantic-graph node payload (INT-08), coexisting with the legacy `offerings: Option<Vec<String>>` flat-string field for backward compatibility.

---
*State updated: 2026-07-15 after 18-04-PLAN.md executed (FS-19 closed; Phase 18 4/4 complete; build clean; day_snapshot_v14_compat 6/6 pass; fengshui_crit3_isolation 1/1 pass; full crate 709+ tests pass). Phase 18 verification is the next step before starting Phase 19.*
