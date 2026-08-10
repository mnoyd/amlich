---
gsd_state_version: 1.0
milestone: v1.8
milestone_name: Surface & Debt Closure
current_plan: Complete
status: milestone_complete
last_updated: "2026-08-10T12:50:03+07:00"
last_activity: 2026-08-10
progress:
  total_tracks: 3
  completed_tracks: 3
  total_requirements: 9
  completed_requirements: 9
---

# Project State

## Current position

**v1.8 Surface & Debt Closure is complete and audited.** All nine requirements
and all six release gates passed on 2026-08-10.

- Audit: `.planning/milestones/v1.8-MILESTONE-AUDIT.md`
- Roadmap archive: `.planning/milestones/v1.8-ROADMAP.md`
- Requirements archive: `.planning/milestones/v1.8-REQUIREMENTS.md`

## Delivered

1. **Desktop Observatory closure** — Evidence Graph workspace, quality gates,
   and user-facing I Ching plus Thái Tuế/Tam Sát ⇄ Phi Tinh cross-link data.
2. **TUI Explanation Views closure** — Vì Sao, Yếu Tố, Hoạt Động, and Nguồn
   user lenses with raw semantic inspection behind debug mode.
3. **Engineering debt closure** — strict clippy and formatting gates, true
   `SourceId` newtype, and the canonical external-review deferral lifecycle.

## External dependencies

Domain-review deferrals remain registered in
`docs/architecture/external-review-lifecycle.md`. They are due 2026-12-31 and
are not unresolved v1.8 implementation work.

## Next step

Select and define a fresh milestone. No v1.9 direction has been chosen. Start
with product questioning and create new requirements before opening delivery
beads. Candidate domains are recorded in `.planning/PROJECT.md` and
`.planning/ROADMAP.md`.
