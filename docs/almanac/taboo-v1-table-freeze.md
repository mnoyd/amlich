# Taboo Tables v1 Freeze (`R-3001` / `amlich-dl5`)

## Purpose

Freeze the v1 matching definitions and table-backed mappings for the four baseline taboo families:

- `tam_nuong`
- `nguyet_ky`
- `sat_chu`
- `tho_tu`

This document records explicit matching criteria, v1 table values, severity defaults, and source caveats so later implementation/docs/tests are auditable.

## v1 Canonical Ruleset Context

- `ruleset_id`: `vn_baseline_v1`
- `ruleset_version`: `v1`
- Terminology and tokens: `docs/almanac/glossary.md`
- Taboo output/explanation conventions: `docs/almanac/taboo-rules.md`

## Matching Definitions (Frozen for v1)

### 1) Tam Nương (`tam_nuong`)

- Match input: `lunar_day`
- Match rule: `lunar_day ∈ {3, 7, 13, 18, 22, 27}`
- v1 default severity: `hard`
- Notes:
  - Treated as a fixed lunar-day family in v1 (no month/year/day-branch conditions)
  - Suitable for deterministic fixture coverage and no table variants inside this ruleset version

### 2) Nguyệt Kỵ (`nguyet_ky`)

- Match input: `lunar_day`
- Match rule: `lunar_day ∈ {5, 14, 23}`
- v1 default severity: `hard`
- Notes:
  - Treated as a fixed lunar-day family in v1
  - Deterministic and low-variance in current baseline scope

### 3) Sát Chủ (`sat_chu`)

- Match inputs: `lunar_month`, `day_chi`
- Match rule: `day_chi == sat_chu.by_lunar_month[lunar_month]`
- v1 default severity: `hard`
- Notes:
  - Variant-sensitive family; v1 freezes one month->chi mapping
  - Any alternate table must be shipped as new variant/version, not silent replacement

### 4) Thọ Tử (`tho_tu`)

- Match inputs: `lunar_month`, `day_chi`
- Match rule: `day_chi == tho_tu.by_lunar_month[lunar_month]`
- v1 default severity: `soft`
- Notes:
  - Variant-sensitive family; v1 freezes one month->chi mapping
  - Severity is a policy hint (not direct event recommendation)

## v1 Month-to-Branch Tables (Frozen)

### Sát Chủ (`sat_chu.by_lunar_month`)

| Lunar month | Day branch (`day_chi`) that matches |
|---|---|
| 1 | `Tỵ` |
| 2 | `Tý` |
| 3 | `Mùi` |
| 4 | `Mão` |
| 5 | `Thân` |
| 6 | `Tuất` |
| 7 | `Hợi` |
| 8 | `Sửu` |
| 9 | `Ngọ` |
| 10 | `Dần` |
| 11 | `Dậu` |
| 12 | `Thìn` |

### Thọ Tử (`tho_tu.by_lunar_month`)

| Lunar month | Day branch (`day_chi`) that matches |
|---|---|
| 1 | `Thìn` |
| 2 | `Tỵ` |
| 3 | `Ngọ` |
| 4 | `Mùi` |
| 5 | `Thân` |
| 6 | `Dậu` |
| 7 | `Tuất` |
| 8 | `Hợi` |
| 9 | `Tý` |
| 10 | `Sửu` |
| 11 | `Dần` |
| 12 | `Mùi` |

## Severity Policy Proposal (v1 Defaults)

These defaults are frozen for `vn_baseline_v1` data and match current implementation behavior.

| Rule family | `rule_id` | Default severity | Rationale (v1 practical baseline) |
|---|---|---|---|
| Tam Nương | `tam_nuong` | `hard` | Widely treated as a strong avoid signal |
| Nguyệt Kỵ | `nguyet_ky` | `hard` | Widely treated as a strong avoid signal |
| Sát Chủ | `sat_chu` | `hard` | High-caution family in practical calendars; keep strict by default |
| Thọ Tử | `tho_tu` | `soft` | Keep visible as caution while allowing future event-policy weighting |

Policy notes:

- `hard` / `soft` are **ruleset severity hints**, not final event scoring decisions.
- Event engines may interpret them differently by event type, but must preserve raw taboo hits.

## Source and Caveat Notes (v1)

### Tam Nương / Nguyệt Kỵ

- Practical VN convention selected for v1.
- These are modeled as fixed lunar-day sets and considered stable enough for baseline implementation.

### Sát Chủ / Thọ Tử

- Variant-sensitive across sources and lineages.
- v1 freezes the above month->chi tables as the baseline mapping used by current ruleset data.
- Contributor rule: no in-place mutation of these tables for `vn_baseline_v1`; use a new ruleset version/variant for changes.

## Acceptance Criteria Trace (`R-3001`)

1. **Each rule family has explicit matching criteria** -> documented in "Matching Definitions".
2. **Sát Chủ/Thọ Tử tables frozen for v1** -> documented in "Month-to-Branch Tables (Frozen)".
3. **Sources and caveats recorded** -> documented in "Source and Caveat Notes" + linked docs.

## Related Docs

- `docs/almanac/beads/phase-3.md`
- `docs/almanac/taboo-rules.md`
- `docs/almanac/glossary.md`
- `docs/almanac/research-sources.md`
- `docs/almanac/research-gap-matrix.md`
