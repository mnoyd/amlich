# Independent Trực and Ngũ Hành Boundaries at the API Contract

The enriched TUI Ngũ Hành screen combines `TrucInsightDto` and `DayGuidanceDto` in presentation, but the API contract keeps them as two independent insights with no typed Trực × day-element interaction. This is the deliberate boundary; this ADR records why.

## Considered Options

- **Add a typed `TrucElementInteractionDto` carrying a derived explanation with evidence** — rejected. No source corpus carries this cross-rule:
  - `crates/amlich-core/data/truc-insight.json` has 12 entries with only `meaning` / `good_for` / `avoid_for`; no element affinity.
  - `DayGuidance` is keyed by `day.chi` and contains only `good_for` / `avoid_for`; no Trực-aware cross-reference.
  - `docs/almanac/recommendation-rule-matrix.json` lists `signal.truc.primary_router` as a `default_core` primary activity-routing signal — independent authority, no element dimension.
  - `docs/almanac/interaction-matrices-plan.md` defines an "Element Resonance Matrix" as a Day × Person cross, not a Trực × Ngũ Hành intrinsic interaction. Trực is documented only as a quality → domain modifier.
  - `docs/almanac/daily-guidance-research-and-rust-proposal.md` rules are keyed by `(Truc, Activity)`; Ngũ Hành appears only in seasonal personal-strength framing.
  - `docs/almanac/research-gap-matrix.md` marks 12 Trực as READY and lists no Trực × element gap.
  Inventing a derived cross would violate `source.xieji_hierarchy.default_policy` (`use_structured_court_standard_style_authority_for_strong_claims`) and `source.modern_vn_practice.authority` (`disallow: sole_default_logic_authority`).
- **Document Trực and Ngũ Hành as independent at the contract, with TUI composition kept as presentation-only** — chosen. This matches existing source authority, keeps `TrucInsightDto.good_for` / `avoid_for` and `DayGuidanceDto.good_for` / `avoid_for` as the canonical surfaces, and confines "Kết H�p Trực + Hành" to the TUI presentation layer where users see it as grouped advice rather than a semantic interaction.

## Decision

`TrucInsightDto` and `DayGuidanceDto` are independent surfaces in `DayInsightDto`. No interaction DTO is introduced. The TUI may render them together in a single panel but must label the section as presentation composition. The API contract test asserts the two DTOs serialize independently so future contributors cannot quietly couple them without a contract test change.

## Consequences

- Strong claims about Trực × day-element interaction must come from a new curated corpus entry with provenance, not from inference over the existing datasets.
- The TUI "Kết Hợp Trực + Hành" panel keeps its current behaviour (prefers `day_guidance` when both are present, falls back to `truc`) but its header now states the section is composition, not a source-derived interaction.
- `recommendation-rule-matrix.json` gains a `signal.truc_x_element.no_default_interaction` item documenting this decision so the policy is discoverable from the existing rule matrix.
