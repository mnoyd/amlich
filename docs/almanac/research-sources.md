# Almanac Research Sources Tracker

## Purpose

Track source material used for each rule family so implementation decisions are auditable.

## Source Table

| Rule Family | Status | Primary Source | Secondary Source | Decision Needed | Target Ruleset |
|---|---|---|---|---|---|
| Solar <-> Lunar core | selected | Existing core implementation references | Existing regression tests | keep as baseline | `vn_baseline_v1` |
| Leap month and month 11 | selected | Existing core implementation references | Existing regression tests | preserve behavior | `vn_baseline_v1` |
| Can Chi year/month/day | selected | Existing core formulas | Existing fixtures | preserve behavior | `vn_baseline_v1` |
| Can Chi hour | open | TBD | TBD | choose v1 formula convention | `vn_baseline_v1` |
| Tiet khi (fast mode) | selected | Existing core implementation | Existing fixtures | preserve behavior | `vn_baseline_v1` |
| Day hoang dao/hac dao | selected | `docs/almanac/day-deity-v1-table.md` (v1 canonicalized practical VN mapping) | Project decision log (`DEC-0006`) | encode as ruleset family + add golden examples | `vn_baseline_v1` |
| Gio hoang dao | selected | Existing implementation | Existing tests | integrate with ruleset metadata | `vn_baseline_v1` |
| 12 truc | selected | Existing implementation | Existing tests | integrate evidence metadata | `vn_baseline_v1` |
| Nhi thap bat tu (cycle) | open | Existing cycle approach | TBD anchor reference | choose anchor and doc | `vn_baseline_v1` |
| Tam Nuong / Nguyet Ky | open | Practical VN convention | TBD source note | encode as taboo rules | `vn_baseline_v1` |
| Sat Chu / Tho Tu | open | TBD | TBD | freeze one table variant | `vn_baseline_v1` |
| Tam Tai | open | TBD | TBD | freeze mapping table | `vn_baseline_v1` |
| Kim Lau | open | TBD | TBD | freeze formula + age policy | `vn_baseline_v1` |
| Hoang Oc | open | TBD | TBD | freeze table variant | `vn_baseline_v1` |
| Cuu Dieu | open | TBD | TBD | freeze gender-age cycle table | `vn_baseline_v1` |
| Yearly Han | open | TBD | TBD | freeze table variant | `vn_baseline_v1` |
| Direction families | open | Existing baseline table | TBD variant table | split VN/CN variants | `vn_baseline_v1` + future |
| Event scoring policy | open | Product policy doc | TBD domain reference | define hard/soft/weights | `vn_baseline_v1` |

## Decision Log Linkage

When a source is frozen for implementation:

1. Add/Update entry in this file.
2. Add a decision in `docs/almanac/decision-log.md`.
3. Link decision id in relevant bead file.

## Notes

- "selected" means good enough for implementation in current phase.
- "open" means research bead required before implementation bead.
