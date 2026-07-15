---
gsd_state_version: 1.0
milestone: v1.6
milestone_name: Integration
status: in_progress
last_updated: "2026-07-15T11:45:35.904Z"
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

**Current focus:** v1.6 Eastern Knowledge Completion — Phase 17 complete; Phase 18 next.

## Current Position

Milestone: v1.6 Eastern Knowledge Completion.
Phase: 17 of 4 planned (Phase 17: Văn khấn Reviewer Closure, 2 plans).
Plan: 17-02 COMPLETE (RIT-16 closed via ledger-driven corrected-entry round-trip test). Phase 17 done.
Status: `provenance_audit.md` (8-column, 60 rows all `ExternalReviewPending`) now has a black-box reader-of-record test (`every_ledger_row_passes_invariants`) and a forward-compatible RIT-16 corrected-entry gate (`every_corrected_entry_passes_schema_and_nfc_round_trip`). Phase 17 closes RIT-14 + RIT-15 (Plan 17-01) AND RIT-16 (Plan 17-02). `RitualEntry` JSON schema remains locked per ADR-0001; the ledger is the canonical reviewer record.
Last activity: 2026-07-15 — 17-02-PLAN.md executed: test-only Markdown pipe-table parser + 2 new tests in `crates/amlich-core/tests/rituals_integration.rs` (commits 57496f7 + 0c3d483). `cargo build -p amlich-core` clean; `cargo test -p amlich-core --test rituals_integration` 8/8 pass (6 pre-existing + 2 new); full crate gate 890/890 (888 Phase-16 baseline + 2 new, zero regressions). RIT-16 closed.

Progress: [▓▓▓▓▓░░░░░] 50% (v1.6: 2 of 4 phases complete; 4 of 11 plans complete).

## v1.6 Target Features

1. **Daily Flying Star (日紫白)** — per-day Phi Tinh overlay with 冬至/夏至 reversal (Phase 18, next).
2. **`RecommendsOffering` semantic-graph node** — promote from flat string list (Phase 19).
3. **RIT-11 reviewer field closure** — ✅ RIT-14 + RIT-15 closed in 17-01; ✅ RIT-16 closed in 17-02. Phase 17 complete.
4. **ADR-0003 pre-1984 confidence boost** — ✅ FND-07 closed in 16-01; ✅ FND-08 closed in 16-02. Phase 16 complete.

## Resources

- `.planning/PROJECT.md` — project trajectory + Key Decisions table (updated 2026-07-15).
- `.planning/MILESTONES.md` — shipped-milestone log with stats + accomplishments.
- `.planning/ROADMAP.md` — v1.6 roadmap (Phases 16-19, 11 plans, 12/12 requirements mapped). Phase 17 marked Complete (2/2).
- `.planning/REQUIREMENTS.md` — v1.6 requirements + traceability (RIT-16 marked Complete post-17-02).
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
- `.planning/phases/17-van-khan-reviewer-closure/17-02-SUMMARY.md` — Plan 17-02 execution record (RIT-16 corrected-entry gate).

## Session Continuity

Last session: 2026-07-15T11:36:53Z
Stopped at: 17-02-PLAN.md complete. Phase 17 plan 2 of 2 executed (RIT-14 + RIT-15 + RIT-16 all closed).
Resume file: None.

### Next Step

Phase 17 complete. v1.6 milestone has 4 phases; Phases 16 + 17 done (FND-07/08 + RIT-14/15/16 closed). Remaining: Phase 18 (Daily Phi Tinh — FS-16/17/18/19, 4 plans), Phase 19 (`RecommendsOffering` semantic-graph node — INT-07/08/09/10, 3 plans). Proceed to Phase 18 with `/gsd-plan-phase 18` or `/gsd-discuss-phase 18` per workflow.

---
*State updated: 2026-07-15 after 17-02-PLAN.md executed (RIT-16 closed; Phase 17 complete; 890/890 tests pass).*