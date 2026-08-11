# Requirements: Amlich v1.10 — Traditional Wellness Context (Tier 0)

**Defined:** 2026-08-11
**Status:** Delivery beads published

The canonical active requirements are maintained in
[`milestones/v1.10-REQUIREMENTS.md`](milestones/v1.10-REQUIREMENTS.md).

## Requirement index

- **BOUND-01** — Tier-0 calendar/time inputs only; missing personal or medical
  data never blocks the result.
- **BOUND-02** — Stable bilingual disclaimer, non-clinical safety class, and no
  clinical/procedural public fields.
- **SOURCE-01** — Distinct primitive source IDs and a transparent seasonal
  composite; `ty-ngo-luu-chu` remains reserved.
- **SOURCE-02** — Per-record provenance, reviewer state, safety class, and
  `KnownDivergence` references.
- **ASSOC-01** — Complete selected-hour Twelve-Branch Channel Association.
- **SEASON-01** — Complete selected-date four-season context across all 24
  solar terms.
- **EXPLAIN-01** — Cross-surface explanation and association-safe semantic
  graph.
- **VERIFY-01** — Finite goldens, transition/boundary coverage, safety guards,
  compatibility, parity, and full release gates.

Active parent epic: `amlich-l2zc`. Child beads: `.1` and `.2` are independent
HITL slices; `.3` depends on both; `.4` depends on `.3`.

Research and source boundaries are recorded in
[`research/LUNAR_HEALTH_RESEARCH.md`](research/LUNAR_HEALTH_RESEARCH.md). The
naming/scope decision is recorded in
[`../docs/adr/0003-separate-branch-channel-association-from-ty-ngo-luu-chu.md`](../docs/adr/0003-separate-branch-channel-association-from-ty-ngo-luu-chu.md).
