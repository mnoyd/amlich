# Taboo Rules for `vn_baseline_v1`

## Purpose

Document taboo-rule behavior for API/UI consumers and contributors:

- what each taboo family means in v1
- how `hard` vs `soft` severity should be interpreted
- explanation message conventions
- source caveats and variant policy

This doc corresponds to Phase 3 `D-3006`.

## Output Shape (Canonical)

Day outputs may include `taboos[]` entries with this structure:

- `rule_id` (`tam_nuong | nguyet_ky | sat_chu | tho_tu`)
- `name` (Vietnamese display label)
- `severity` (`hard | soft`)
- `reason` (deterministic human-readable explanation)
- `evidence` (`source_id`, `method`, `profile`)

## Rule Families in v1

### 1) Tam Nương (`tam_nuong`)

- Match criteria: lunar day in `{3, 7, 13, 18, 22, 27}`
- Current severity: `hard`
- Resolver context keys: `lunar_day`

### 2) Nguyệt Kỵ (`nguyet_ky`)

- Match criteria: lunar day in `{5, 14, 23}`
- Current severity: `hard`
- Resolver context keys: `lunar_day`

### 3) Sát Chủ (`sat_chu`)

- Match criteria: `day_chi == sat_chu.by_lunar_month[lunar_month]`
- Current severity: `hard`
- Resolver context keys: `lunar_month`, `day_chi`
- v1 table is ruleset data-driven (month -> chi mapping)

### 4) Thọ Tử (`tho_tu`)

- Match criteria: `day_chi == tho_tu.by_lunar_month[lunar_month]`
- Current severity: `soft`
- Resolver context keys: `lunar_month`, `day_chi`
- v1 table is ruleset data-driven (month -> chi mapping)

## Severity Semantics

Severity values are **machine-readable policy hints**; they are not final event recommendations by themselves.

### `hard`

- Treat as strong negative signal.
- Default consumer behavior:
  - mark as avoid/high-risk in UI badges
  - allow event evaluators to use as hard filter candidates
- Does not mandate hiding the date; clients may still display with explicit warning.

### `soft`

- Treat as cautionary signal.
- Default consumer behavior:
  - show warning/caveat text
  - apply score penalty in evaluators (rather than outright exclusion)

## Explanation Message Conventions

To keep messages deterministic and testable, v1 uses fixed templates by rule family.

### Template rules

1. Start with objective context (lunar day or day-branch/month relation).
2. Use canonical rule label as in `name`.
3. Avoid recommendation wording in `reason`; recommendation belongs to consumer/policy layer.
4. Keep formatting stable so fixtures/tests can compare exact strings.

### v1 templates

- `tam_nuong`: `Ngày âm lịch {lunar_day} thuộc Tam Nương`
- `nguyet_ky`: `Ngày âm lịch {lunar_day} thuộc Nguyệt Kỵ`
- `sat_chu`: `Chi ngày {day_chi} trùng chi Sát Chủ của tháng âm lịch {lunar_month}`
- `tho_tu`: `Chi ngày {day_chi} trùng chi Thọ Tử của tháng âm lịch {lunar_month}`

## Determinism Guarantees

For a fixed `(ruleset_id, ruleset_version, lunar_day, lunar_month, day_chi)`:

- matched taboo set is deterministic
- output ordering is deterministic (current family order from resolver)
- `reason` strings are deterministic by template

## Source and Caveat Notes

- Tam Nương and Nguyệt Kỵ are widely-used practical conventions and modeled as fixed lunar-day sets in v1.
- Sát Chủ and Thọ Tử are variant-sensitive across sources/traditions; v1 freezes one mapping table in `vn_baseline_v1` and treats alternatives as future variant/version work.
- Contributors must not silently replace month->chi mappings in-place for existing ruleset versions.

## Consumer Guidance (API/UI)

### For API consumers

- Always key logic by `rule_id` and `severity`, not by display `name` text.
- Assume additional taboo families may appear in future ruleset versions; unknown `rule_id` should be handled gracefully.

### For UI consumers

- Show `name` + `reason` together.
- Render severity with distinct visual treatment (`hard` stronger than `soft`).
- Preserve Vietnamese diacritics in display labels/reasons.

## Contributor Guidance

When changing taboo behavior:

1. Update ruleset data and this document together.
2. Keep explanation templates stable unless a documented versioned change is intended.
3. Add/update tests around affected family and boundary contexts.
4. If behavior is breaking, bump ruleset version rather than mutating existing version semantics.

## Related Documents

- `docs/almanac/ruleset-v1-scope.md`
- `docs/almanac/decision-log.md`
- `docs/almanac/glossary.md`
- `docs/almanac/beads/phase-3.md`
