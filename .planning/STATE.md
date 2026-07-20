---
gsd_state_version: 1.0
milestone: v1.8
milestone_name: Surface & Debt Closure
current_plan: Not started
status: defining_requirements
last_updated: "2026-07-20T12:00:00.000Z"
last_activity: 2026-07-20
progress:
  total_phases: 0
  completed_phases: 0
  total_plans: 0
  completed_plans: 0
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-07-20 for v1.8 milestone start).

**Core value:** Every almanac subsystem in amlich must produce output matching its canonical classical source (KHCBPPT / `vn-folk-ritual` / *Thẩm Thị Huyền Không Học* / *Kinh Dịch Trọn Bộ* / *Mai Hoa Dịch Số*) for the 2020-2030 date range, with test-backed and traceable evidence.

**Current focus:** v1.8 Surface & Debt Closure — land v1.7 backend into desktop + TUI surfaces, close in-flight UX epics, retire engineering debt carried since v1.5.

## Current Position

**Milestone:** v1.8 Surface & Debt Closure — defining requirements.
**Status:** Defining requirements after milestone kickoff.
**Phase:** Not started.
**Plan:** —.

Progress: [ ] 0% (defining requirements).

### v1.8 Scope (3 tracks)

1. **Desktop Observatory closure** — finish `amlich-00j` epic (Evidence Graph workspace `amlich-01mx` + quality gates `amlich-2nqy`); get v1.7 IChing + Thái Tuế / Tam Sát cross-link payloads user-visible in Observatory workspaces.
2. **TUI Explanation Views closure** — finish `amlich-5no` epic (Yếu Tố lens `amlich-0qv` + rendering/navigation tests `amlich-jet`); default non-dev TUI shows "Vì Sao Kết Luận" with four Vietnamese-labelled lenses.
3. **Engineering debt phase** — `amlich-081` (~96 clippy warnings) + `SourceId = String` → true newtype + documented `[PendingExternalReview]` workflow.

### v1.8 Out of Scope (locked)

P3 Y học, P5 Spatial Phi Tinh, P6 Tử Vi, Hỗ Quả, Tier-2 Bazi enrichment of IChing, LLM interpretation, domain-expert text resolution for the 64-hexagram deferrals (external dependency — v1.8 only tightens the workflow).

## Prior Milestone

v1.7 Kinh Dịch (I-Ching Divination) — SHIPPED 2026-07-19 (audited 2026-07-20). 6 phases (20-25), 14 plans, 15 requirements satisfied; 1120 crate tests green; zero new deps. Archives: `.planning/milestones/v1.7-*.md`.

## Known Gaps (carry-forward into v1.8 where addressable)

- **~96 cargo clippy/fmt warnings on master** (carry-forward from v1.5 baseline; v1.8 will close via DEBT-01/02).
- **`SourceId = String` transparent alias** (carry-forward from v1.6; v1.8 will close via DEBT-03).
- **Undocumented `[PendingExternalReview]` workflow** for the 4 carry-forward domain-expert deferrals (64-hexagram Ngô Tất Tố text, Tam Sát page citation, 1960 Trung Nguyên polarity split, ADR-0004 daily page citation); v1.8 will close via DEBT-04.
- **64-hexagram Ngô Tất Tố interpretive text** (AF-05) — `thoai_tu` / `hao_tu` / `cat_hung` carrying `[PendingExternalReview]` placeholders pending **external** domain-expert verification. NOT resolvable in v1.8 (external dependency).
- **Tam Sát KHCBPPT page-level citation** deferred — `PendingExternalReview` per ADR-0006 §5. NOT resolvable in v1.8 (external dependency).

(See `.planning/milestones/v1.7-MILESTONE-AUDIT.md` for the full v1.7 audit report — status `tech_debt`, 15/15 requirements satisfied, 0 integration gaps, 4/4 E2E flows complete.)

## Resources

- `.planning/PROJECT.md` — project trajectory + Key Decisions table (v1.8 milestone scope added 2026-07-20).
- `.planning/MILESTONES.md` — shipped-milestone log (v1.0-v1.7 archived).
- `.planning/ROADMAP.md` — v1.7 in `<details>`; ready for v1.8 active section.
- `.planning/RETROSPECTIVE.md` — v1.7 milestone section + cross-milestone trends updated 2026-07-20.
- `.planning/milestones/v1.{0..7}-{ROADMAP,REQUIREMENTS}.md` — shipped milestone archives.
- `.planning/adrs/` — ADRs 0001-0007 + 0003a (all locked).

## Session Continuity

Last session: 2026-07-20 — v1.7 retrospective milestone audit complete; milestone selection discussion resulted in choosing v1.8 = Surface & Debt Closure over P3 Y học / P6 Tử Vi / P5 Spatial alternatives.
Stopped at: v1.8 milestone kickoff — PROJECT.md + STATE.md updated; requirements + roadmap next.
Resume file: None (milestone kickoff; requirements being defined inline).

### Next Step

v1.8 milestone kickoff in progress. After requirements + roadmap are committed:
- `/gsd-discuss-phase 26` — gather context for first v1.8 phase
- `/gsd-plan-phase 26` — plan directly (skip discussion)

For v1.8+ per v1.7 retrospective lesson #10: `/gsd-audit-milestone` MUST run BEFORE `/gsd-complete-milestone`.
