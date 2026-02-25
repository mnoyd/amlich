# Almanac Glossary (v1 Normalized)

## Purpose

Define one canonical vocabulary for `vn_baseline_v1` planning and implementation so docs, code, and tests use the same terms.

Conventions in this file:

- `vi_term`: canonical Vietnamese display term
- `token`: canonical internal token (snake_case)
- aliases are accepted for reading/search, but should not be used as canonical field names

## Core Deterministic Calendar Terms

| vi_term | token | Definition | Notes |
|---|---|---|---|
| Âm lịch | `lunar_calendar` | Vietnamese lunar calendar date system used by core conversion logic | Deterministic layer |
| Dương lịch | `solar_calendar` | Gregorian/solar date input and output | Deterministic layer |
| Thiên can | `heavenly_stem` | 10-stem cycle component (`Giáp`..`Quý`) | Usually paired with địa chi |
| Địa chi | `earthly_branch` | 12-branch cycle component (`Tý`..`Hợi`) | Usually paired with thiên can |
| Can chi | `can_chi` | Stem-branch pair for year/month/day/hour | Canonical combined term |
| Tiết khí | `solar_term` | One of 24 solar terms used in day context | Fast active-term mode in v1 |
| Nạp âm | `nap_am` | 60-pair elemental designation of a can-chi pair | Day element in current scope |
| Ngũ hành | `five_elements` | Elemental system (`Kim/Mộc/Thủy/Hỏa/Thổ`) | Applies to can/chi and related rules |

## Day Rule Terms

| vi_term | token | Definition | Notes |
|---|---|---|---|
| Giờ hoàng đạo | `auspicious_hours` | Favorable hours within a day | Already implemented before Phase 2 |
| Ngày hoàng đạo/hắc đạo | `day_deity` | Day-level thần trực nhật classification | Includes deity name + hoang/hac class |
| Thần trực nhật | `day_deity` | Deity assigned by lunar-month branch + day branch mapping | Alias of day hoang dao/hac dao mapping family |
| Hoàng đạo | `hoang_dao` | Auspicious day-deity classification | Use exact token in outputs |
| Hắc đạo | `hac_dao` | Inauspicious day-deity classification | Use exact token in outputs |
| Thập nhị trực | `truc` | 12 trực cycle label for the day | Existing struct token is `truc` |
| Xung/hợp | `xung_hop` | Branch relation bundle (lục xung, tam hợp, tứ hành xung) | Structural relation family |
| Tuổi xung | `tuoi_xung` | Conflicting zodiac/person labels derived from day relation rules | Currently in day conflict output |
| Sát hướng | `sat_huong` | Inauspicious direction related to day context | In day conflict output |
| Xuất hành hướng | `xuat_hanh_huong` | Recommended departure direction for the day stem | Travel-direction family |
| Tài thần | `tai_than` | Wealth deity direction | Travel-direction family |
| Hỷ thần | `hy_than` | Joy deity direction | Travel-direction family |
| Kỵ thần | `ky_than` | Avoid direction/deity marker (optional) | Optional field in output |
| Cát tinh | `cat_tinh` | Auspicious stars active in current context | Star family list |
| Sát tinh | `sat_tinh` | Inauspicious stars active in current context | Star family list |
| Bình tinh | `binh_tinh` | Neutral stars (recorded in rule tables) | Excluded from final cat/sat output lists |
| Nhị thập bát tú | `nhi_thap_bat_tu` | 28-star cycle system used for day-star assignment | Current method: JD-cycle |

## Taboo Rule Terms

| vi_term | token | Definition | Notes |
|---|---|---|---|
| Kiêng kỵ ngày | `day_taboo` | Structured taboo hit in day output | Includes severity/reason/evidence |
| Tam Nương | `tam_nuong` | Fixed lunar-day taboo family | Core taboo in v1 scope |
| Nguyệt Kỵ | `nguyet_ky` | Fixed lunar-day taboo family | Core taboo in v1 scope |
| Sát Chủ | `sat_chu` | Lunar-month/branch keyed taboo family | Variant-sensitive family |
| Thọ Tử | `tho_tu` | Lunar-month/branch keyed taboo family | Variant-sensitive family |
| Mức độ cấm kỵ | `taboo_severity` | Severity level for taboo hits | Allowed tokens in v1: `hard`, `soft` |

## Person Rule Terms (Phase 4+ Planning)

| vi_term | token | Definition | Notes |
|---|---|---|---|
| Tam Tai | `tam_tai` | Multi-year unfavorable cycle rule | Source freeze required before implementation |
| Kim Lâu | `kim_lau` | Age-related caution rule | Depends on age policy |
| Hoang Ốc | `hoang_oc` | House-related age/year caution family | Variant-sensitive |
| Cửu Diệu | `cuu_dieu` | Yearly star assignment family | Source freeze required |
| Hạn năm | `yearly_han` | Yearly caution mapping family | Source freeze required |
| Tuổi mụ | `tuoi_mu` | Lunar-age policy used by person rules | Policy freeze required |

## Evaluation and Scoring Terms (Phase 6+ Planning)

| vi_term | token | Definition | Notes |
|---|---|---|---|
| Đánh giá ngày | `day_evaluation` | Event-oriented day suitability evaluation | Separate from raw day facts |
| Bộ lọc cứng | `hard_filter` | Rule that can disqualify a day outright | Policy-driven |
| Điểm mềm | `soft_score` | Weighted scoring signal when not disqualifying | Policy-driven |
| Trọng số | `weight` | Score contribution coefficient for an evaluation feature | Policy-driven |
| Lý do giải thích | `explanation_reason` | Human-readable explanation attached to evaluation result | Must be deterministic for same input+ruleset |

## Ruleset and Contract Terms

| vi_term | token | Definition | Notes |
|---|---|---|---|
| Bộ quy tắc | `ruleset` | Versioned pack of almanac rule families and metadata | Core architecture concept |
| Mã bộ quy tắc | `ruleset_id` | Stable identifier of a ruleset pack | v1 id: `vn_baseline_v1` |
| Phiên bản bộ quy tắc | `ruleset_version` | Version string of ruleset behavior/data | Must bump on breaking behavior change |
| Hồ sơ | `profile` | Human-facing profile label in outputs | Transitional label; do not replace ruleset id/version |
| Bằng chứng quy tắc | `rule_evidence` | Provenance metadata (`source_id`, `method`, `profile`) | Attached per field/rule as available |
| Nguồn | `source_id` | Source lineage identifier for a rule family | e.g. `khcbppt` |
| Phương pháp | `method` | Derivation method token | e.g. `table-lookup`, `bai-quyet`, `jd-cycle` |

## Alias and Synonym Mapping (Canonicalization)

Use this table to normalize input language from docs/research into canonical tokens.

| Alias / synonym seen in docs | Canonical token |
|---|---|
| canchi, can-chi, can chi | `can_chi` |
| tiet khi, tiết-khí | `solar_term` |
| gio hoang dao, giờ tốt | `auspicious_hours` |
| ngay hoang dao, hắc đạo ngày, thần trực nhật | `day_deity` |
| hoang dao, ngày tốt (day-deity context) | `hoang_dao` |
| hac dao, hắc đạo | `hac_dao` |
| 12 truc, thập nhị trực | `truc` |
| luc xung, tam hop, tu hanh xung | `xung_hop` |
| tuoi xung, tuổi kỵ ngày | `tuoi_xung` |
| tai than, tài thần phương vị | `tai_than` |
| hy than, hỷ thần | `hy_than` |
| ky than, kỵ thần | `ky_than` |
| cat tinh, sao tốt | `cat_tinh` |
| sat tinh, sao xấu | `sat_tinh` |
| binh tinh, sao trung tính | `binh_tinh` |
| nhi thap bat tu, 28 sao | `nhi_thap_bat_tu` |
| taboo, kiêng kỵ, ngày kỵ | `day_taboo` |
| sat chu, sát chủ | `sat_chu` |
| tho tu, thọ tử | `tho_tu` |
| ruleset, profile pack, rule pack | `ruleset` |
| ruleset id | `ruleset_id` |
| ruleset version | `ruleset_version` |

## Ambiguity Notes and Non-goals

Ambiguous terms that must be disambiguated in implementation/design docs:

1. `Sao` is ambiguous:
   - day star system (`nhi_thap_bat_tu`)
   - cát/sát star-rule lists (`cat_tinh`/`sat_tinh`)
   - future person yearly stars (`cuu_dieu`)

2. `Hoàng đạo` is ambiguous:
   - hour-level (`auspicious_hours`)
   - day-deity classification (`hoang_dao` in `day_deity`)

3. `Mệnh` is ambiguous:
   - in current day output scope canonicalized as `nap_am` day element
   - person destiny schools are out of scope for v1 core contract

Non-goals for this glossary:

- It does not freeze source tables by itself (that belongs to research/freeze beads).
- It does not define scoring policy values (only terminology).
- It does not replace per-family implementation docs.
