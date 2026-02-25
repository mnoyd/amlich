# Day Hoang Dao / Hac Dao v1 Mapping Table (`R-2001`)

## Scope

This document freezes the canonical day-level hoang dao/hac dao (thần trực nhật) mapping for ruleset `vn_baseline_v1`.

It defines:

- the 12 deity names used in outputs
- hoang/hac classification for each deity
- the month-branch-group offset table
- worked lookup examples for all 12 lunar months

## v1 Naming Set (Canonical)

Canonical deity cycle order (12 positions):

1. `Thanh Long` (Hoang Dao)
2. `Minh Đường` (Hoang Dao)
3. `Thiên Hình` (Hac Dao)
4. `Chu Tước` (Hac Dao)
5. `Kim Quỹ` (Hoang Dao)
6. `Kim Đường` (Hoang Dao)
7. `Bạch Hổ` (Hac Dao)
8. `Ngọc Đường` (Hoang Dao)
9. `Thiên Lao` (Hac Dao)
10. `Huyền Vũ` (Hac Dao)
11. `Tư Mệnh` (Hoang Dao)
12. `Câu Trần` (Hac Dao)

### Classification Freeze (v1)

- Hoang Dao: `Thanh Long`, `Minh Đường`, `Kim Quỹ`, `Kim Đường`, `Ngọc Đường`, `Tư Mệnh`
- Hac Dao: `Thiên Hình`, `Chu Tước`, `Bạch Hổ`, `Thiên Lao`, `Huyền Vũ`, `Câu Trần`

## Lookup Model (Month Branch + Day Branch)

The resolver uses:

- lunar month branch (`Dần`..`Sửu`)
- day branch (`Tý`..`Hợi`)

v1 uses a month-group start offset model:

- Each month-branch pair shares one mapping row.
- Within a row, deity assignment advances in the fixed 12-deity cycle as day branch advances `Tý -> Hợi`.

## Month-Group Start Table (v1 canonical)

The table below lists the deity assigned at day branch `Tý` for each lunar month-branch group.

| Lunar month branch group | Day `Tý` starts at | Start index in deity cycle |
|---|---|---|
| `Dần`, `Thân` | `Thanh Long` | 0 |
| `Mão`, `Dậu` | `Thiên Hình` | 2 |
| `Thìn`, `Tuất` | `Kim Quỹ` | 4 |
| `Tỵ`, `Hợi` | `Bạch Hổ` | 6 |
| `Ngọ`, `Tý` | `Thiên Lao` | 8 |
| `Mùi`, `Sửu` | `Tư Mệnh` | 10 |

Equivalent formula (for implementation):

- Let `month_group_index` be `0..5` in the row order above.
- `start_offset = month_group_index * 2`
- `deity_index = (start_offset + day_branch_index_from_ty) % 12`

## Expanded Mapping Matrix (12 day branches)

Day branch order: `Tý, Sửu, Dần, Mão, Thìn, Tỵ, Ngọ, Mùi, Thân, Dậu, Tuất, Hợi`

### Group `Dần`, `Thân`

`Thanh Long, Minh Đường, Thiên Hình, Chu Tước, Kim Quỹ, Kim Đường, Bạch Hổ, Ngọc Đường, Thiên Lao, Huyền Vũ, Tư Mệnh, Câu Trần`

### Group `Mão`, `Dậu`

`Thiên Hình, Chu Tước, Kim Quỹ, Kim Đường, Bạch Hổ, Ngọc Đường, Thiên Lao, Huyền Vũ, Tư Mệnh, Câu Trần, Thanh Long, Minh Đường`

### Group `Thìn`, `Tuất`

`Kim Quỹ, Kim Đường, Bạch Hổ, Ngọc Đường, Thiên Lao, Huyền Vũ, Tư Mệnh, Câu Trần, Thanh Long, Minh Đường, Thiên Hình, Chu Tước`

### Group `Tỵ`, `Hợi`

`Bạch Hổ, Ngọc Đường, Thiên Lao, Huyền Vũ, Tư Mệnh, Câu Trần, Thanh Long, Minh Đường, Thiên Hình, Chu Tước, Kim Quỹ, Kim Đường`

### Group `Ngọ`, `Tý`

`Thiên Lao, Huyền Vũ, Tư Mệnh, Câu Trần, Thanh Long, Minh Đường, Thiên Hình, Chu Tước, Kim Quỹ, Kim Đường, Bạch Hổ, Ngọc Đường`

### Group `Mùi`, `Sửu`

`Tư Mệnh, Câu Trần, Thanh Long, Minh Đường, Thiên Hình, Chu Tước, Kim Quỹ, Kim Đường, Bạch Hổ, Ngọc Đường, Thiên Lao, Huyền Vũ`

## Worked Examples (12 lunar months)

The examples below intentionally cover every lunar month branch (`Dần` through `Sửu`).

1. Lunar month `1` (`Dần`), day branch `Tý` -> `Thanh Long` -> `Hoang Dao`
2. Lunar month `2` (`Mão`), day branch `Tý` -> `Thiên Hình` -> `Hac Dao`
3. Lunar month `3` (`Thìn`), day branch `Tý` -> `Kim Quỹ` -> `Hoang Dao`
4. Lunar month `4` (`Tỵ`), day branch `Tý` -> `Bạch Hổ` -> `Hac Dao`
5. Lunar month `5` (`Ngọ`), day branch `Tý` -> `Thiên Lao` -> `Hac Dao`
6. Lunar month `6` (`Mùi`), day branch `Tý` -> `Tư Mệnh` -> `Hoang Dao`
7. Lunar month `7` (`Thân`), day branch `Tuất` -> `Tư Mệnh` -> `Hoang Dao`
8. Lunar month `8` (`Dậu`), day branch `Tuất` -> `Thanh Long` -> `Hoang Dao`
9. Lunar month `9` (`Tuất`), day branch `Tuất` -> `Thiên Hình` -> `Hac Dao`
10. Lunar month `10` (`Hợi`), day branch `Tuất` -> `Kim Quỹ` -> `Hoang Dao`
11. Lunar month `11` (`Tý`), day branch `Tuất` -> `Bạch Hổ` -> `Hac Dao`
12. Lunar month `12` (`Sửu`), day branch `Tuất` -> `Thiên Lao` -> `Hac Dao`

## Variant Notes (Documented but NOT selected for v1)

Known naming variants in circulation:

- `Kim Đường` vs `Bảo Quang` (same slot in many tables)
- `Huyền Vũ` vs `Nguyên Vũ`
- `Câu Trần` vs `Câu Trận`

v1 policy:

- Keep the slot ordering/classification fixed.
- Treat label differences as source or locale variants.
- If a variant changes slot assignment (not just naming), it must ship as a new ruleset/version.

## Source Lineage / Provenance Notes

- v1 source family label for this table: practical Vietnamese almanac day-deity convention (month-branch + day-branch lookup)
- Project canonicalization target: `vn_baseline_v1`
- This freeze is recorded in `docs/almanac/decision-log.md` and will be encoded as a dedicated ruleset family in Phase 2.

## Implementation Handoff Notes (`I-2002`)

- Store this as a ruleset-backed family (not hard-coded in resolver logic).
- Prefer storing canonical cycle order + month-group offsets, then derive matrix in code/tests.
- Emit typed classification (`hoang_dao` / `hac_dao`) plus deity name and evidence metadata.
