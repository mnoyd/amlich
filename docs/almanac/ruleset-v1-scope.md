# Almanac Ruleset v1 Scope

## Ruleset Identifier

- `ruleset_id`: `vn_baseline_v1`
- `ruleset_version`: `v1`
- `region`: `vn`
- `intent`: practical Vietnamese almanac baseline with explainable provenance

## Scope Freeze Summary

This scope is aligned to `docs/almanac/master-plan.md` and split into:

- **Included in v1 baseline**: required families for `vn_baseline_v1` delivery across phases.
- **Deferred from v1 baseline**: explicitly out of scope for this ruleset version.
- **Backlog with owner**: unresolved families tracked by bead/phase ownership.

## Included in v1

Deterministic layer (Phase 0-1 foundation):

- Solar <-> lunar conversion (timezone-parametric)
- Leap month logic
- Can Chi year/month/day
- Tiet khi active term (fast mode)

Rule-based day layer (Phases 1-3):

- Gio hoang dao
- Day hoang dao/hac dao (day deity mapping frozen in Phase 2)
- 12 truc
- Xung/hop relations
- Ngu hanh and nap am day element
- Core taboo flags:
  - Tam Nuong
  - Nguyet Ky
  - Sat Chu
  - Tho Tu

Person layer (Phases 4-5, still v1 target):

- Tuoi xung
- Tam Tai
- Kim Lau
- Hoang Oc
- Cuu Dieu
- Yearly Han

Evaluation layer (Phase 6, still v1 target):

- Event-type day evaluation with hard filters and scoring

## Deferred or Optional for v1

- High-accuracy tiet khi instant calculation
- Astronomical mode for nhi thap bat tu
- Official vs astronomical historic split mode
- Full VN/CN dual-behavior parity guarantees at launch

## Backlog and Ownership for Unresolved Families

The following are in v1 target scope but still require source freeze before implementation:

- `tam_tai`, `kim_lau`, `hoang_oc`, `cuu_dieu`, `yearly_han`
  - Owner: Phase 4-5 research/implementation beads (`R-4001`, `R-5001` and dependents)
- `sat_chu`, `tho_tu` variant lineage hardening
  - Owner: Phase 3 research bead (`R-3001`) and docs follow-up (`D-3006`)
- Event hard/soft scoring policy
  - Owner: Phase 6 research beads (`R-6001`, `R-6002`)

If any item above is not finalized in-bead, it must remain backlog and cannot be silently folded into `vn_baseline_v1` behavior.

## Ruleset Contract Requirements

Every ruleset pack must include:

- identity: `id`, `version`, `region`
- defaults: `tz_offset`, optional `meridian`
- source metadata per rule family (`source_id`, `method`)
- data tables for each enabled rule family
- validation constraints and schema version

Minimum required family keys for v1 packs:

- `travel`
- `conflict`
- `nap_am`
- `stars` (including day star cycle + rule buckets)
- `day_deity`
- `taboo_rules`

## Output Contract Requirements

Every computed almanac output should include:

- `ruleset_id`
- `ruleset_version`
- `profile`
- per-field or per-rule evidence metadata

For day-level output, v1 additionally requires:

- `day_deity.name`
- `day_deity.classification` (`hoang_dao | hac_dao`)
- `day_deity.evidence`

## Known Variation Policy

- If a rule family has known variants, v1 picks one canonical mapping and documents it.
- Alternate mappings must be added as new versions or variants, not silent replacement.
- Breaking output changes require ruleset version bump.

Reference freeze points:

- Day deity mapping: `docs/almanac/day-deity-v1-table.md`
- Terminology/tokens: `docs/almanac/glossary.md`

## Acceptance Criteria for v1 Scope Freeze

Scope freeze is complete when:

1. Included rule families are finalized.
2. Deferred items are explicitly listed.
3. Unresolved in-scope families are assigned to backlog beads/owners.
4. Versioning and evidence requirements are approved.
5. Initial golden test categories are defined.
