---
gsd_state_version: 1.0
milestone: v1.10
milestone_name: Traditional Wellness Context (Tier 0)
current_plan: Phase 01 in progress (branch-channel, core done); Phase 02 in progress (seasonal cultivation, core done)
status: phase_in_progress
last_updated: "2026-08-16T16:20:00+07:00"
last_activity: 2026-08-16
progress:
  total_tracks: 4
  completed_tracks: 0
  total_requirements: 8
  completed_requirements: 0
---

# Project State

## Current position

**v1.9 Multi-dimensional Day, Hour, and Direction Assessment is complete and
audited.** All seven requirements and all seven release gates passed on
2026-08-11. The next milestone, v1.10 Traditional Wellness Context (Tier 0),
is now defined from primary-source research but has not entered delivery until
its tracer-bullet bead breakdown is approved.

- Audit: `.planning/milestones/v1.9-MILESTONE-AUDIT.md`
- Milestone summary: `.planning/MILESTONES.md` (v1.9 entry)
- v1.10 requirements: `.planning/milestones/v1.10-REQUIREMENTS.md`
- v1.10 research: `.planning/research/LUNAR_HEALTH_RESEARCH.md`
- Scope ADR: `docs/adr/0003-separate-branch-channel-association-from-ty-ngo-luu-chu.md`

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

The v1.10 release gates include four explicit reviews: the 12-record
branch-channel corpus, four *Suwen* paraphrases, bilingual disclaimer/legal
wording, and health-safety/schema review. Until those reviews are signed,
affected records remain `ExternalReviewPending` or unavailable.

## Next step

The parent epic `amlich-l2zc` is published. Bead `.1` (branch-channel) has its
Phase 01-01 core complete — awaiting the three human review gates. Bead `.2`
(seasonal cultivation) has its Phase 02-01 core complete as of 2026-08-16:
`SOURCE_HUANGDI_NEIJING_SUWEN` registered, the 4-profile Suwen corpus shipped
`ExternalReviewPending`, the frozen 24-term → 4-season composition (Lập
boundaries, `rule.composite.seasonal_wellness` composite envelope) locked by
`tests/seasonal_cultivation_integration.rs`, the unsourced `health` lists in
`tiet-khi.json` emptied (both copies), and the seasonal REVIEWER-PACK published
for Gates 2–4. Bead `.3` (unified explanation + API/TUI/desktop/graph
projection, including the `DaySnapshot.traditional_wellness` additive field)
is unblocked once its parents' cores are accepted; bead `.4` remains blocked
by `.3`. The four human review gates are the critical path for closing `.1`
and `.2`.
