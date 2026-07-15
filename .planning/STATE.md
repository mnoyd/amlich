---
gsd_state_version: 1.0
milestone: v1.6
milestone_name: Integration
status: in_progress
last_updated: "2026-07-15T11:29:40.000Z"
progress:
  total_phases: 21
  completed_phases: 21
  total_plans: 58
  completed_plans: 58
---

# Project State
## Project Reference

See: .planning/PROJECT.md (updated 2026-07-15)

**Core value:** Every almanac subsystem in amlich must produce output matching its canonical classical source (KHCBPPT / `vn-folk-ritual` / *Thẩm Thị Huyền Không Học*) for the 2020-2030 date range, with test-backed and traceable evidence.

**Current focus:** v1.6 Eastern Knowledge Completion — Phase 16 executing (2/2 plans complete; FND-07 + FND-08 closed).

## Current Position

Milestone: v1.6 Eastern Knowledge Completion — Phase 17 in progress.
Phase: 17 of 4 planned (Phase 17: Văn khấn Reviewer Closure, 2 plans).
Plan: 17-01 COMPLETE; 17-02 next. RIT-14 + RIT-15 closed via 8-column reviewer-audit ledger expansion. RIT-16 awaits Plan 17-02 corrected-entry re-verification test.
Status: `provenance_audit.md` rewritten to 8-column shape (ritual_id | classical_reference | page | confidence | reviewer | method_of_review | date_reviewed | outcome). All 60 reviewer cells now carry the typed `ExternalReviewPending(reason="..."; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer")` marker; outcome=ExternalReviewPending, method=desk-check, date_reviewed=2026-07-15. Outcome counts: 0 confirmed / 0 corrected / 0 disputed / 60 ExternalReviewPending. No fabricated reviewer identities (DEC-0015/0016 + ADR-0001). `RitualEntry` JSON schema unchanged; ledger is the canonical reviewer record.
Last activity: 2026-07-15 — 17-01-PLAN.md executed: editorial ledger pass on `crates/amlich-core/data/rituals/provenance_audit.md` (commit 1777666). `cargo build -p amlich-core` clean; `cargo test -p amlich-core --test rituals_integration` 6/6 pass. 888/888 tests still pass (no test changes in this plan; Plan 17-02 will add 2 more). RIT-14 + RIT-15 closed.

Progress: [▓▓░░░░░░░░] 25% (v1.6: 0 phases, 3 plans).

## v1.6 Target Features

1. **Daily Flying Star (日紫白)** — per-day Phi Tinh overlay with 冬至/夏至 reversal (Phase 18).
2. **`RecommendsOffering` semantic-graph node** — promote from flat string list (Phase 19).
3. **RIT-11 reviewer field closure** — ✅ RIT-14 + RIT-15 closed in 17-01 (ExternalReviewPending markers on all 60 entries); RIT-16 awaits Plan 17-02. Phase 17 in progress.
4. **ADR-0003 pre-1984 confidence boost** — ✅ FND-07 closed in 16-01; ✅ FND-08 closed in 16-02. Phase 16 complete.

## Resources

- `.planning/PROJECT.md` — project trajectory + Key Decisions table (updated 2026-07-15).
- `.planning/MILESTONES.md` — shipped-milestone log with stats + accomplishments.
- `.planning/ROADMAP.md` — v1.6 roadmap (Phases 16-19, 11 plans, 12/12 requirements mapped).
- `.planning/REQUIREMENTS.md` — v1.6 requirements + traceability (FND-07 + FND-08 marked Complete post-16-01 + 16-02).
- `.planning/research/SUMMARY.md` — v1.5 research (HIGH confidence on P1/P4; v1.6 daily layer extends the validated patterns; no refresh flagged).
- `.planning/milestones/v1.5-{ROADMAP,REQUIREMENTS,MILESTONE-AUDIT}.md` — v1.5 archive (reuse patterns).
- `.planning/RETROSPECTIVE.md` — cross-milestone learnings (v1.5 patterns carry forward: schema-lock-before-corpus, single-commit RED→GREEN, audit-as-decisive-source, external-crate black-box tests).
- `.planning/adrs/0001-ritual-schema-v1.md` — ADR-0001 (locked).
- `.planning/adrs/0002-phi-tinh-monthly-anchor.md` — ADR-0002 (locked).
- `.planning/adrs/0003-nien-tu-bach-polarity.md` — ADR-0003 (matrix authoritative; §6 superseded by ADR-0003a).
- `.planning/adrs/0003a-nien-tu-bach-polarity-confidence-closure.md` — ADR-0003a (Accepted 2026-07-15; FND-07 + FND-08 source of truth).
- `.planning/phases/16-foundation-adr-0003-confidence-closure/16-01-SUMMARY.md` — Plan 16-01 execution record (FND-07).
- `.planning/phases/16-foundation-adr-0003-confidence-closure/16-02-SUMMARY.md` — Plan 16-02 execution record (FND-08).
- `.planning/phases/17-van-khan-reviewer-closure/17-01-SUMMARY.md` — Plan 17-01 execution record (RIT-14 + RIT-15 ledger expansion).

## Session Continuity

Last session: 2026-07-15T11:29:40Z
Stopped at: 17-01-PLAN.md complete. Phase 17 plan 1 of 2 executed (RIT-14 + RIT-15 closed).
Resume file: None.

### Next Step

Phase 17 plan 1 of 2 complete. Continue with 17-02-PLAN.md (corrected-entry `body_vi` re-verification test for RIT-16). v1.6 milestone has 4 phases; Phase 16 (FND-07/08) + Phase 17 partial (RIT-14/15) executed. Remaining: 17-02 (RIT-16), Phase 18 (FS-16/17/18/19), Phase 19 (INT-07/08/09/10).

---
*State updated: 2026-07-15 after 17-01-PLAN.md executed (RIT-14 + RIT-15 closed; 8-column reviewer-audit ledger live).*