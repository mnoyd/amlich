# Day Hoang Dao / Hac Dao Documentation (`D-2006`)

## Purpose

Document the chosen v1 day-level hoang dao/hac dao behavior for contributors and client consumers:

- canonical mapping source/freeze
- resolver formula used in code
- output shape and evidence semantics
- known variants and caveats
- worked examples and testing guidance

## Scope (v1)

This document describes the day-level `day_deity` family in `vn_baseline_v1` only.

It does **not** redefine hour-level `gio_hoang_dao` (documented/implemented separately).

## Canonical v1 Mapping

The canonical mapping is frozen in:

- `docs/almanac/day-deity-v1-table.md`

This freeze is accepted by:

- `DEC-0006` in `docs/almanac/decision-log.md`

## Terminology (Normalized)

From `docs/almanac/glossary.md`:

- family token: `day_deity`
- classification tokens: `hoang_dao`, `hac_dao`
- alternate descriptive term: `thần trực nhật`

## Resolver Inputs and Formula

### Inputs

Resolver input context (v1):

- `lunar_month` (numeric lunar month, normalized to `1..=12`)
- `day_chi` (day earthly branch token: `Tý`..`Hợi`)

### Month-branch derivation

The resolver derives the lunar month branch from the lunar month number using the standard v1 convention:

- lunar month `1` -> branch `Dần`
- lunar month `12` -> branch `Sửu`

### Lookup model

Implementation uses a ruleset-backed table with:

1. fixed 12-deity cycle order
2. `month_group_start_by_chi` start offsets keyed by month branch
3. day branch index offset (`Tý`..`Hợi`) into the cycle

Equivalent computation:

- `cycle_index = (start_offset_for_month_branch + day_branch_index) % 12`
- resolve deity name and classification from `cycle[cycle_index]`

Reference implementation:

- `crates/amlich-core/src/almanac/day_deity.rs`

## Ruleset Data Shape (v1 baseline)

The day-deity family is stored in baseline ruleset data under:

- `day_deity_meta`
- `day_deity_rule_set`

With key subfields:

- `day_deity_rule_set.cycle[]` (12 entries)
- `day_deity_rule_set.month_group_start_by_chi` (12 chi keys -> start offsets)

Validation checks currently enforce:

- exactly 12 cycle entries
- classification token in `hoang_dao | hac_dao`
- all 12 chi keys present in `month_group_start_by_chi`
- offset values in `0..12`

## Output Contract (Day Fortune)

`DayFortune` / API day-info includes additive day-deity fields in v1:

- `day_deity.name`
- `day_deity.classification` (`hoang_dao | hac_dao` in API DTOs)
- `day_deity.evidence`

Evidence metadata is sourced from `day_deity_meta` and includes:

- `source_id`
- `method`
- `profile`

## Worked Examples (v1)

See full month coverage in `docs/almanac/day-deity-v1-table.md`.

Representative examples:

- lunar month `1` (`Dần`), day chi `Tý` -> `Thanh Long` -> `hoang_dao`
- lunar month `2` (`Mão`), day chi `Tý` -> `Thiên Hình` -> `hac_dao`
- lunar month `9` (`Tuất`), day chi `Tuất` -> `Thiên Hình` -> `hac_dao`

Real-date coverage is additionally checked by golden tests.

## Variant Caveats and Policy

### Naming variants

Known label variants include (non-exhaustive):

- `Kim Đường` vs `Bảo Quang`
- `Huyền Vũ` vs `Nguyên Vũ`
- `Câu Trần` vs `Câu Trận`

v1 policy:

- Keep slot ordering and hoang/hac classification fixed.
- Treat label-only differences as source/locale variants.
- Do not silently change labels or slot assignments for `vn_baseline_v1` once released.

### Mapping variants

If a source uses a different month-group start mapping or deity ordering:

- treat as a different ruleset variant/version
- document explicitly
- add separate tests/fixtures

## Testing and Regression Guidance

Current protection layers include:

- unit tests for resolver behavior in `crates/amlich-core/src/almanac/day_deity.rs`
- loader validation tests for day-deity schema in `crates/amlich-core/src/almanac/data.rs`
- golden mapping tests across month groups/day branches in `crates/amlich-core/tests/almanac_golden.rs`

When changing this family, update docs + ruleset data + tests together.

## Related Documents

- `docs/almanac/day-deity-v1-table.md`
- `docs/almanac/glossary.md`
- `docs/almanac/ruleset-v1-scope.md`
- `docs/almanac/decision-log.md`
- `docs/almanac/beads/phase-2.md`
