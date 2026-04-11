# Direction Families — Extended

**Source ID:** `khcbppt` (Phúc Thần, Sát Phương) + existing (Tài Thần, Hỷ Thần, Xuất Hành)
**Citation:** KHCBPPT, Quyển 9, Lập Thành (立成) — directional deity tables
**Confidence:** HIGH
**Decision:** DEC-0018

---

## Overview

| Direction Family | Basis | Source | Status |
|---|---|---|---|
| Tài Thần (財神) | Day Stem | KHCBPPT Q9 | ✅ Already in project |
| Hỷ Thần (喜神) | Day Stem | KHCBPPT Q9 | ✅ Already in project |
| Xuất Hành Hướng (出行向) | Day Stem | KHCBPPT Q33-34 | ✅ Already in project |
| **Phúc Thần (福神)** | Day Stem | KHCBPPT Q9 | **NEW** |
| **Sát Phương (煞方)** | Day Branch | KHCBPPT Q9 | **NEW** |

## NEW: Phúc Thần (福神 / Fortune God)

**Mnemonic:** 甲己正北是福神，丙辛西北乾宮存，乙庚坤位戊癸艮，丁壬巽上妙追尋

| Day Stem Pair | Direction | Trigram |
|---|---|---|
| Giáp (甲) / Kỷ (己) | Bắc (N) | Khảm (坎) |
| Ất (乙) / Canh (庚) | Tây Nam (SW) | Khôn (坤) |
| Bính (丙) / Tân (辛) | Tây Bắc (NW) | Càn (乾) |
| Đinh (丁) / Nhâm (壬) | Đông Nam (SE) | Tốn (巽) |
| Mậu (戊) / Quý (癸) | Đông Bắc (NE) | Cấn (艮) |

## NEW: Sát Phương (煞方 / Killing Direction)

**Note:** Branch-based (unlike other stem-based direction families).

| Day Branch Group (Tam Hợp) | Sát Direction |
|---|---|
| Tỵ (巳), Dậu (酉), Sửu (丑) | Sát Đông (Kill East) |
| Hợi (亥), Mão (卯), Mùi (未) | Sát Tây (Kill West) |
| Thân (申), Tý (子), Thìn (辰) | Sát Nam (Kill South) |
| Dần (寅), Ngọ (午), Tuất (戌) | Sát Bắc (Kill North) |

Formula: `branch_index % 4` determines triad → opposite cardinal direction.

Same formula works for both **daily Sát** (day branch) and **annual Sát** (year branch).

## Tài Thần Variant Note (DEC-0018)

KHCBPPT variant (default) and folk variant disagree on 3 stems:

| Stem | KHCBPPT (default) | Folk (optional pack) |
|---|---|---|
| Ất | Tây Nam (SW) | Đông Bắc (NE) |
| Bính | Tây (W) | Tây Nam (SW) |
| Đinh | Tây (W) | Tây Nam (SW) |

Folk variant mnemonic: 甲乙東北是財神，丙丁向在西南尋，戊己正北坐方位，庚辛正東去安身，壬癸原來正南坐

## Implementation Notes

- Phúc Thần: add to `travel_by_can` in baseline.json or as separate lookup
- Sát Phương: new lookup by `day_chi_index % 4` → 4 cardinal directions
- Both integrate into Direction Merge Matrix for personal day synthesis
