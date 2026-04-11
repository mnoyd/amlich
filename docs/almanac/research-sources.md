# Almanac Research Sources Tracker

## Purpose

Track source material used for each rule family so implementation decisions are auditable.

## Source Table

| Rule Family | Status | Primary Source | Secondary Source | Decision Needed | Target Ruleset |
|---|---|---|---|---|---|
| Solar <-> Lunar core | selected | Existing core implementation references | Existing regression tests | keep as baseline | `vn_baseline_v1` |
| Leap month and month 11 | selected | Existing core implementation references | Existing regression tests | preserve behavior | `vn_baseline_v1` |
| Can Chi year/month/day | selected | Existing core formulas | Existing fixtures | preserve behavior | `vn_baseline_v1` |
| Can Chi hour | selected | KHCBPPT Q3-8, 五鼠遁時 (Ngũ Thử Độn Thời) | Multiple VN/CN sources unanimous | 23:00 = new day (DEC-0017) | `vn_baseline_v1` |
| Tiet khi (fast mode) | selected | Existing core implementation | Existing fixtures | preserve behavior | `vn_baseline_v1` |
| Day hoang dao/hac dao | selected | `docs/almanac/day-deity-v1-table.md` (v1 canonicalized practical VN mapping) | Project decision log (`DEC-0006`) | encode as ruleset family + add golden examples | `vn_baseline_v1` |
| Gio hoang dao | selected | Existing implementation | Existing tests | integrate with ruleset metadata | `vn_baseline_v1` |
| 12 truc | selected | Existing implementation | Existing tests | integrate evidence metadata | `vn_baseline_v1` |
| Nhi thap bat tu (cycle) | open | Existing cycle approach | TBD anchor reference | choose anchor and doc | `vn_baseline_v1` |
| Tam Nuong / Nguyet Ky | selected | `docs/almanac/taboo-v1-table-freeze.md` (fixed lunar-day sets) | `docs/almanac/taboo-rules.md` | keep deterministic family ids/severity defaults | `vn_baseline_v1` |
| Sat Chu / Tho Tu | selected | `docs/almanac/taboo-v1-table-freeze.md` (v1 month->chi freeze) | Decision log (`DEC-0008`) | add variant as new ruleset version, not replacement | `vn_baseline_v1` |
| Tam Tai | selected | KHCBPPT (三殺) + VN adaptation | Multiple sources unanimous | Tam Hợp triad → opposite direction years (DEC-0021) | `vn_baseline_v1` |
| Kim Lau | selected | Ngọc Hạp Ký (玉匣記) | Vietnamese folk practice | tuổi mụ mod 9, dư 1/3/6/8; NOT in KHCBPPT (DEC-0015) | `ngoc_hap_ky_v1` |
| Hoang Oc | selected | Vietnamese folk tradition | Multiple VN sources | digit sum mod 6, 6-cung cycle; NOT in KHCBPPT (DEC-0015) | `vn_folk_v1` |
| Cuu Dieu | selected | Buddhist/Indian astronomical tradition (九曜) | Vietnamese practice (lyso.vn, vietlac.net) | tuổi mụ mod 9, gender-differentiated tables (DEC-0016) | `cuu_dieu_v1` |
| Yearly Han | selected | Composite of Cửu Diệu + Tam Tai + Kim Lâu + Hoàng Ốc + Thái Tuế | Vietnamese practice | umbrella aggregator, not unified system (DEC-0021) | composite |
| Direction families | selected | KHCBPPT Q9 (Tài/Hỷ/Phúc Thần, Sát Phương) | Folk variant for Tài Thần | KHCBPPT default; folk as optional pack (DEC-0018) | `vn_baseline_v1` + `vn_folk_v1` |
| Sensitive recommendation domains | selected | `docs/almanac/recommendation-safety-policy.md` | Decision log (`DEC-0010`, `DEC-0011`, `DEC-0012`) | keep conservative defaults for burial/funeral, confidence, and wording | `vn_baseline_v1` |
| Event scoring policy | selected | KHCBPPT Q10 (宜忌) 3-tier precedence + Q11 (用事) 37 Dân dụng | Wikisource full text | qualitative validation layer + 37-activity baseline (DEC-0019, DEC-0020) | `vn_baseline_v1` |

## Decision Log Linkage

When a source is frozen for implementation:

1. Add/Update entry in this file.
2. Add a decision in `docs/almanac/decision-log.md`.
3. Link decision id in relevant bead file.

## Notes

- "selected" means good enough for implementation in current phase.
- "open" means research bead required before implementation bead.
