# Almanac Ruleset v1 Scope

## Ruleset Identifier

- `ruleset_id`: `vn_baseline_v1`
- `region`: `vn`
- `intent`: practical Vietnamese almanac baseline with explainable provenance

## Included in v1

Deterministic layer:

- Solar <-> lunar conversion (timezone-parametric)
- Leap month logic
- Can Chi year/month/day
- Tiet khi active term (fast mode)

Rule-based day layer:

- Gio hoang dao
- Day hoang dao/hac dao (after Phase 2 table freeze)
- 12 truc
- Xung/hop relations
- Ngu hanh and nap am day element
- Core taboo flags:
  - Tam Nuong
  - Nguyet Ky
  - Sat Chu
  - Tho Tu

Person layer:

- Tuoi xung
- Tam Tai
- Kim Lau
- Hoang Oc
- Cuu Dieu
- Yearly Han

Evaluation layer:

- Event-type day evaluation with hard filters and scoring

## Deferred or Optional for v1

- High-accuracy tiet khi instant calculation
- Astronomical mode for nhi thap bat tu
- Official vs astronomical historic split mode
- Full VN/CN dual-behavior parity guarantees at launch

## Ruleset Contract Requirements

Every ruleset pack must include:

- identity: `id`, `version`, `region`
- defaults: `tz_offset`, optional `meridian`
- source metadata per rule family (`source_id`, `method`)
- data tables for each enabled rule family
- validation constraints and schema version

## Output Contract Requirements

Every computed almanac output should include:

- `ruleset_id`
- `ruleset_version`
- `profile`
- per-field or per-rule evidence metadata

## Known Variation Policy

- If a rule family has known variants, v1 picks one canonical mapping and documents it.
- Alternate mappings must be added as new versions or variants, not silent replacement.
- Breaking output changes require ruleset version bump.

## Acceptance Criteria for v1 Scope Freeze

Scope freeze is complete when:

1. Included rule families are finalized.
2. Deferred items are explicitly listed.
3. Versioning and evidence requirements are approved.
4. Initial golden test categories are defined.
