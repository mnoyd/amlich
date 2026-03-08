# Recommendation Taxonomy Audit (v1)

## Scope

This audit covers canonical activity IDs used by `amlich-core` recommendation synthesis, plus alias normalization from:

- `data/canchi.json` (`dayGuidance`)
- `data/truc-insight.json`
- modifier sources (stars/day deity/taboos/xung-hop/hours/travel/tiet-khi)

## Canonical Activity IDs

Current v1 canonical IDs are retained as the stable public contract:

- `travel`
- `meeting_social`
- `opening_start`
- `contract_agreement`
- `business_trade`
- `finance_investment`
- `construction_groundbreaking`
- `repair_renovation`
- `move_relocation`
- `wedding_engagement`
- `lawsuit_dispute`
- `prayer_offering`
- `medical_treatment`
- `burial_memorial`
- `cleaning_purging`

## Coverage Findings

1. **Direct action phrases from `truc-insight` are mappable** to the canonical set (for recommendation emission).
2. **`dayGuidance` includes many abstract behavioral phrases** (for example: "Việc cần bình tĩnh", "Linh hoạt, ứng biến", "Cầu toàn quá mức").
3. Abstract behavioral phrases should not be force-mapped into activity IDs because that creates unstable and low-signal recommendations.

## Decision

Keep the canonical activity set unchanged for v1.

- Do not split/merge IDs yet.
- Do not add personality/behavior IDs in the day-only contract.
- Keep non-actionable `dayGuidance` phrases as informational insight, not recommendation evidence.

This decision avoids churn across core/API fixtures/TUI labels while preserving deterministic output semantics.

## Follow-up Rules

- New aliases may be added only when they map to a concrete user action.
- New canonical IDs require:
  - at least two independent evidence sources,
  - DTO + fixture + TUI label updates,
  - corpus coverage in `recommendation-corpus-v1.json`.
