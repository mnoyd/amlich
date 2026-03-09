# Recommendation Conflict Triage

## Purpose

Turn current recommendation-system research conflicts into explicit implementation decisions.

This document is intended to answer:

- what exactly is in conflict
- what type of conflict it is
- what the codebase should do now
- what code and tests would be touched if the item is promoted

This is an execution document, not a research note.

## Status Labels

- `default_core`
- `optional_pack`
- `personalization_layer`
- `defer`
- `reject`

## Conflict Types

- `architecture`
- `source_authority`
- `strength`
- `scope`
- `safety`

## Triage Table

| Item | Conflict Type | Tension | Disposition | Why | Code Impact | Test Impact |
|---|---|---|---|---|---|---|
| Score-driven engine vs precedence-first engine | architecture | One research path prefers threshold scoring; another prefers explicit precedence and suppression rules | `default_core` = precedence-first, score only as bounded support | Avoids fake precision and fits current deterministic merger model | `crates/amlich-core/src/almanac/recommendation/synthesize.rs` | update synthesis tests only if bounded tie-break logic is introduced |
| `truc` as a primary routing signal | strength | No meaningful conflict; only differences in exact breadth of mappings | `default_core` | Strong consensus and already compatible with current activity model | `crates/amlich-core/src/almanac/recommendation/synthesize.rs` | corpus and mapping tests when new `truc` mappings are added |
| `hoang_dao/hac_dao` as major authority vs bounded modifier | strength | Some material treats it as a general day baseline; deeper research treats it as non-dominant | `default_core` as bounded modifier | High user recognition, but weaker than activity-specific rules and explicit taboos | `crates/amlich-core/src/almanac/recommendation/synthesize.rs` | corpus cases for mixed-signal days |
| `nhi_thap_bat_tu` in default v1 vs optional/versioned pack | scope | Some research treats it as core; other material treats it as high-risk without stronger validation | `defer` from `default_core`; prepare as `optional_pack` | High impact, high variant risk, likely to cause unstable outputs if rushed | new recommendation rule layer or pack data; likely `crates/amlich-core/src/almanac/recommendation/*` plus fortune inputs | dedicated corpus fixtures and parity tests required before promotion |
| Folk taboo sets (`tam_nuong`, `nguyet_ky`, Yang Gong style sets) as universal truth vs explicit tradition pack | source_authority | Strong modern VN expectation, but not equal to court-standard source authority | keep frozen baseline families explicit; future additions = `optional_pack` | Users expect them, but they should not be disguised as universal consensus | existing taboo handling plus future pack registration | tests per taboo family and ruleset/version behavior |
| Modern VN practice sources as default logic authority | source_authority | Valuable for practice expectations but structurally weaker than curated standards | `reject` as sole authority; use for UX and pack design | Prevents silent drift into SEO-consensus logic | docs and future source metadata only | none unless promoted |
| Xieji-style source hierarchy for strong claims | source_authority | Some docs imply it; deep research states it clearly | `default_core` policy | Provides a stable basis for strong claims and conflict handling | docs first; later rule provenance and pack structure | source-policy docs and regression expectations |
| Burial/funeral recommendation automation | safety | Some traditions contain detailed logic, but product risk is high and cross-source variance is high | `defer` for normal recommendation flow; conservative handling only | High cultural sensitivity, high trust risk | recommendation layer split or product gating if ever added | dedicated safety tests and explicit product policy tests |
| Personalized `xung tuoi` / birth-year logic in day-wide output | scope | Valuable but requires user profile and separate consent/UX model | `personalization_layer` | Not appropriate as silent date-only recommendation logic | new profile-aware `RecommendationLayer` implementation | profile-aware tests separate from date-only corpus |
| Long-tail `than_sat` expansion | scope | Research supports importance, but breadth is large and conflict-prone | `defer` until curated source table exists | Large surface area, easy to overfit, hard to explain | new rule tables and provenance metadata | family-specific fixtures required |
| Numeric confidence scores in API/UI | architecture | Could be useful for UX, but easy to overstate certainty | `defer` unless derived from explicit rule classes rather than opaque math | Confidence should reflect source mode and evidence convergence, not invented percentages | DTO and rendering changes if added | UI/API contract tests required |
| Multiple tradition modes or packs | source_authority | Research strongly suggests separate modes are cleaner than flattening | `default_core` policy direction | Best way to preserve determinism and explain differences | ruleset/pack plumbing over time | versioning and parity tests by pack |

## Immediate Working Rules

Until a conflicting item is explicitly promoted, the codebase should apply these defaults:

1. Keep the current deterministic recommendation engine structure.
2. Prefer precedence and explicit blockers over additive scoring.
3. Do not promote `nhi_thap_bat_tu` into default core.
4. Keep `hoang_dao/hac_dao` as a modifier.
5. Keep folk taboo families explicit, versioned, and provenance-carrying.
6. Keep personalization separate from the date-only engine.
7. Keep burial/funeral logic conservative.

## Promotion Checklist

Before a `defer` or `optional_pack` item becomes `default_core`, require:

- a source-family decision
- a deterministic mapping to canonical activity IDs
- explicit override semantics
- corpus fixtures
- API parity coverage
- documentation of behavior differences if the rule family is variant-sensitive

## Suggested Execution Order

1. Tighten `truc` mappings in default core.
2. Keep refining taboo-family provenance and severity handling.
3. Add pack plumbing for future optional rule families.
4. Design `nhi_thap_bat_tu` as a separate, versioned recommendation pack.
5. Add profile-aware personalization only after the date-only baseline is stable.
