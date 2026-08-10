---
gsd_state_version: 1.0
milestone: v1.9
milestone_name: Multi-dimensional Day, Hour, and Direction Assessment
current_plan: Complete
status: milestone_complete
last_updated: "2026-08-11T01:05:00+07:00"
last_activity: 2026-08-11
progress:
  total_tracks: 4
  completed_tracks: 4
  total_requirements: 7
  completed_requirements: 7
---

# Project State

## Current position

**v1.9 Multi-dimensional Day, Hour, and Direction Assessment is complete and
audited.** All seven requirements and all seven release gates passed on
2026-08-11. Day Assessment, Hour Ranking, and Direction Assessment now own
their own axes, contributions, availability, confidence, and warning
semantics, and every personal-day / hour / direction projection carries a
`policy_version` field so consumers can pin the contract they consume.

- Audit: `.planning/milestones/v1.9-MILESTONE-AUDIT.md`
- Milestone summary: `.planning/MILESTONES.md` (v1.9 entry)

## Delivered

1. **Canonical factor roles** — `Role::Veto`, `Role::Contribution`, and
   `Role::Informational` ship across core, API DTOs, desktop EvidenceGraph,
   and TUI summary; missing evidence surfaces as explicit `Unavailable`,
   never as zero.
2. **Personal Day Assessment v2.3 + v2.4** — Bazi target-day observations
   (`BaziTargetDayTenGod`, `BaziTargetDayPillarRelation`,
   `BaziTargetDayElementResonance`) and per-system non-Bazi annual pressures
   (Tam Tai, Kim Lâu, Hoàng Ốc, Thái Tuế, Cửu Diệu / sao hạn) project into
   the PersonalAlignment and AnnualPressure axes with branch-relation dedup
   by kind.
3. **Personal Hour Ranking v2.4** — three typed, source-attributed
   full-profile observations (`HourPillarTenGodToDayMaster`,
   `HourChiBranchRelationToBirthHour`, `HourStemElementSupport`) fold into
   the Personal Hour Alignment axis; v1 birth-year-chi fallback stays
   byte-identical for date-only callers.
4. **Intent-aware Direction Assessment v1** — travel deities, Kua
   compatibility, Tam Sát and Thái Tuế constraints, and available flying-star
   overlays compose into a direction result that never silently mutates the
   Day Assessment or Hour Ranking.
5. **Explanation projection v1** — `AssessmentExplanation`,
   `DirectionExplanation`, and `HourExplanation` carry
   `Precedence::VetoOverridesAggregation`, `vetoes_applied` separate from
   weighted factors, deduplicated facts per active policy family, and
   per-input confidence breakdown; identical surface across API DTOs, TUI
   `render_explanation_summary`, and desktop EvidenceGraph.
6. **GitHub Actions runtime chore** — `actions/checkout@v6`,
   `actions/setup-node@v6`, and `pnpm/action-setup@v6` are pinned across CI,
   release, and opencode workflows with `node-version: 24`; Node 20
   deprecation annotations are gone.

## Policy version matrix

| Domain | Versions |
|---|---|
| `ASSESSMENT_POLICY_*` | `v2`, `v2.1`, `v2.2`, `v2.3`, `v2.4` |
| `HOUR_RANKING_POLICY_*` | `v1`, `v2.4` |
| `DIRECTION_ASSESSMENT_POLICY_*` | `v1` |
| `EXPLANATION_PROJECTION_*` | `v1` |

Every older version remains a valid input contract. Glossary entries live in
`CONTEXT.md` under the v2.3 / v2.4 / Hour Ranking Policy / Direction
Assessment / Explanation Projection sections.

## External dependencies

Domain-review deferrals remain registered in
`docs/architecture/external-review-lifecycle.md`. They are due 2026-12-31 and
are not unresolved v1.9 implementation work.

## Next step

Select and define a fresh milestone. v1.9 closed the multi-dimensional
assessment arc, so the next direction should target an adjacent capability
(feng-shui period overlays, semantic-graph editorial surfaces, performance
budgets, or a new domain pillar). Start with product questioning and create
new requirements before opening delivery beads. Candidate domains are
recorded in `.planning/PROJECT.md` and `.planning/ROADMAP.md`.
