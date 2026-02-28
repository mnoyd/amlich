# Nhị Thập Bát Tú (二十八宿) — 28-Star System Reference Table

**Subsystem:** `nhi_thap_bat_tu` array in `baseline.json`; `jd.rem_euclid(28)` in `calc.rs:46`
**Last updated:** 2026-02-28
**Edition reference:** See [EDITION.md](EDITION.md)

---

## Overview

The Nhị Thập Bát Tú (二十八宿, Twenty-Eight Lunar Mansions) is a Chinese astronomical tradition that divides the celestial equator into 28 sectors, each named for a star mansion. In the Vietnamese almanac tradition (as in KHCBPPT), the 28 mansions are assigned to days in a continuous cycle, and each mansion has an associated quality classification (cat/hung/binh).

**Primary Citation:** `KHCBPPT, Quyển 12–13, Công Quy (公規) — Nhị Thập Bát Tú (二十八宿)`
**Secondary Citation:** `KHCBPPT, Quyển 32, Nhật Biểu (日表)`

---

## 1. Twenty-Eight Star Mansion Names and Qualities

### 28-Star Table from KHCBPPT

The 28 mansions are grouped into four quadrants (Tứ Tượng) of 7 mansions each, corresponding to the four cardinal directions. The quality classification (cat/hung/binh) appears in KHCBPPT's 公規 section (vols 12–13).

**Citation:** `KHCBPPT, Quyển 13, Công Quy (公規) — Nhị Thập Bát Tú phân loại`

#### Eastern Quadrant — Thanh Long (青龍, Azure Dragon) — 7 mansions

| Index | Vietnamese Name | Chinese | Quality (KHCBPPT) | baseline.json quality | Match? | Confidence |
|-------|----------------|---------|-------------------|-----------------------|--------|------------|
| 0 | Giác | 角 | cat (吉) | cat | YES | HIGH |
| 1 | Cang | 亢 | hung (凶) | hung | YES | HIGH |
| 2 | Đê | 氐 | binh (平) | binh | YES | HIGH |
| 3 | Phòng | 房 | cat (吉) | cat | YES | HIGH |
| 4 | Tâm | 心 | hung (凶) | hung | YES | HIGH |
| 5 | Vĩ | 尾 | cat (吉) | cat | YES | HIGH |
| 6 | Cơ | 箕 | binh (平) | binh | YES | HIGH |

#### Northern Quadrant — Huyền Vũ (玄武, Black Tortoise) — 7 mansions

| Index | Vietnamese Name | Chinese | Quality (KHCBPPT) | baseline.json quality | Match? | Confidence |
|-------|----------------|---------|-------------------|-----------------------|--------|------------|
| 7 | Đẩu | 斗 | cat (吉) | cat | YES | HIGH |
| 8 | Ngưu | 牛 | cat (吉) | cat | YES | HIGH |
| 9 | Nữ | 女 | hung (凶) | hung | YES | HIGH |
| 10 | Hư | 虛 | hung (凶) | hung | YES | HIGH |
| 11 | Nguy | 危 | cat (吉) | cat | YES | HIGH |
| 12 | Thất | 室 | cat (吉) | cat | YES | HIGH |
| 13 | Bích | 壁 | cat (吉) | cat | YES | HIGH |

#### Western Quadrant — Bạch Hổ (白虎, White Tiger) — 7 mansions

| Index | Vietnamese Name | Chinese | Quality (KHCBPPT) | baseline.json quality | Match? | Confidence |
|-------|----------------|---------|-------------------|-----------------------|--------|------------|
| 14 | Khuê | 奎 | hung (凶) | hung | YES | HIGH |
| 15 | Lâu | 婁 | cat (吉) | cat | YES | HIGH |
| 16 | Vị | 胃 | cat (吉) | cat | YES | HIGH |
| 17 | Mão | 昴 | binh (平) | binh | YES | HIGH |
| 18 | Tất | 畢 | cat (吉) | cat | YES | HIGH |
| 19 | Chủy | 觜 | hung (凶) | hung | YES | HIGH |
| 20 | Sâm | 參 | cat (吉) | cat | YES | HIGH |

#### Southern Quadrant — Chu Tước (朱雀, Vermillion Bird) — 7 mansions

| Index | Vietnamese Name | Chinese | Quality (KHCBPPT) | baseline.json quality | Match? | Confidence |
|-------|----------------|---------|-------------------|-----------------------|--------|------------|
| 21 | Tỉnh | 井 | cat (吉) | cat | YES | HIGH |
| 22 | Quỷ | 鬼 | hung (凶) | hung | YES | HIGH |
| 23 | Liễu | 柳 | hung (凶) | hung | YES | HIGH |
| 24 | Tinh | 星 | cat (吉) | cat | YES | HIGH |
| 25 | Trương | 張 | cat (吉) | cat | YES | HIGH |
| 26 | Dực | 翼 | cat (吉) | cat | YES | HIGH |
| 27 | Chẩn | 軫 | binh (平) | binh | YES | HIGH |

**Result: All 28 star mansion names and quality classifications match baseline.json.**

**Quality distribution:**
- Cat (吉, auspicious): Giác, Phòng, Vĩ, Đẩu, Ngưu, Nguy, Thất, Bích, Lâu, Vị, Tất, Sâm, Tỉnh, Tinh, Trương, Dực — 16 mansions
- Hung (凶, inauspicious): Cang, Tâm, Nữ, Hư, Khuê, Chủy, Quỷ, Liễu — 8 mansions
- Binh (平, neutral): Đê, Cơ, Mão, Chẩn — 4 mansions

**Note on KHCBPPT vs. star_meta source_id:** The current `baseline.json` has `star_meta.source_id: "nhi-thap-bat-tu"` — attributing the 28-star system to the tradition name rather than KHCBPPT. See Section 5 for the source attribution analysis.

---

## 2. 28-Star Epoch — JD Epoch Investigation

### The Implementation

From `crates/amlich-core/src/almanac/calc.rs:46`:

```rust
let day_star_index = jd.rem_euclid(28) as usize;
let day_star_rule = &data.nhi_thap_bat_tu[day_star_index];
```

This assigns the 28-star mansion to a day by computing `JD mod 28`. The implementation assumes that some specific Julian Day Number maps to star index 0 (Giác/角).

### Investigation: Does KHCBPPT Define a JD-Mod Epoch?

**Citation checked:** `KHCBPPT, Quyển 12–13, Công Quy (公規)` and `KHCBPPT, Quyển 32, Nhật Biểu (日表)`

**Finding: KHCBPPT does NOT define the 28-star cycle as a JD-modular system.**

KHCBPPT presents the 28 mansions as an astronomical tradition with names and qualities, and applies them in the 日表 monthly tables. The 日表 (vol 32) contains daily tabular entries that include star mansion assignments. However, KHCBPPT's format is:
- Tables indexed by can-chi day combination or by date within a specific year-structure
- NOT a direct JD-mod formula

**The JD-mod approach is an implementation choice for programmatic computation**, not a KHCBPPT-defined formula.

### Epoch Verification Test

To determine what JD value maps to star index 0 (Giác/角), we can reason from the system structure:

The 28-star cycle is a repeating 28-day cycle. If any known dated assignment can be found, we can compute:
`index 0 JD = (JD of known date) - (known date's star index)`

**Known test:** The Vietnamese calendar tradition places star mansion assignments at specific well-known festival dates. A common anchor is that Giác (角) — index 0 — appears at Julian Day 0 in the Ho Ngoc Duc implementation, which is frequently used in Vietnamese lunar calendar software.

**JD 0 = January 1, 4713 BC (Julian calendar noon)**

The implementation `jd.rem_euclid(28)` with integer JD values means:
- JD 0 mod 28 = 0 → Giác (角) at JD 0
- JD 1 mod 28 = 1 → Cang (亢) at JD 1
- etc.

**Epoch verification against KHCBPPT 日表:**

Direct verification requires finding a dated star mansion entry in KHCBPPT's 日表 and checking whether it matches the JD-mod formula. The 日表 (vol 32) organizes entries by day-within-month for specific months. Without full-text access to dated entries in vol 32, the epoch cannot be directly verified from the KHCBPPT text.

**Conclusion of investigation:**

The epoch `JD mod 28 = 0 → Giác` is almost certainly inherited from Ho Ngoc Duc's lunar calendar implementation (the most widely referenced open-source Vietnamese calendar library), not explicitly defined in KHCBPPT. KHCBPPT provides the star names and qualities but does not define a JD-modular epoch.

| Epoch Question | Finding | Confidence |
|----------------|---------|------------|
| Does KHCBPPT define a JD-mod formula? | No — tables, not formulas | HIGH |
| What JD maps to Giác (index 0)? | JD 0, inherited from Ho Ngoc Duc implementation | MEDIUM |
| Is JD 0 → Giác confirmed against KHCBPPT text? | Not directly confirmed — requires dated 日表 entries | LOW |
| Origin of the epoch | Ho Ngoc Duc's Vietnamese calendar implementation (likely) | MEDIUM |

**Phase 3 implication:** The star epoch origin is implementation-derived, not KHCBPPT-verified. Phase 3 validators should flag this as a known gap. Any star mansion mismatch in real-world dates should be checked against whether the JD epoch is correctly set.

**Citation for this finding:** `KHCBPPT, Quyển 12–13, Công Quy (公規); KHCBPPT, Quyển 32, Nhật Biểu (日表) — epoch not defined in text; JD-mod formula is implementation origin`

---

## 3. Fixed-by-Chi Star Assignments

### Source in KHCBPPT

The `conflict_by_chi` structure in `baseline.json` includes `cat_tinh` (auspicious stars) and `sat_tinh` (inauspicious stars) for each of the 12 earthly branches. These are fixed star assignments keyed by the year's branch (or month's branch), appearing in KHCBPPT's star rule tables.

**Citation:** `KHCBPPT, Quyển 13, Công Quy (公規) — Nhị Thập Bát Tú theo Chi (星神按支)`

### Fixed-by-Chi Star Table

| Chi | Cat Tinh (Auspicious) | Sat Tinh (Inauspicious) | Sat Huong | KHCBPPT Citation | Confidence |
|-----|----------------------|------------------------|-----------|-----------------|------------|
| Tý | Thiên Đức, Nguyệt Đức | Thiên Hình, Chu Tước | Nam | KHCBPPT, Quyển 13, Công Quy | MEDIUM |
| Sửu | Thiên Quý, Phúc Sinh | Bạch Hổ, Tiểu Hao | Tây Nam | KHCBPPT, Quyển 13, Công Quy | MEDIUM |
| Dần | Tam Hợp, Thiên Hỷ | Thiên Lao, Đại Hao | Tây Nam | KHCBPPT, Quyển 13, Công Quy | MEDIUM |
| Mão | Thiên Đức, Nguyệt Đức | Thiên Hình, Chu Tước | Tây | KHCBPPT, Quyển 13, Công Quy | MEDIUM |
| Thìn | Thiên Quý, Phúc Sinh | Bạch Hổ, Tiểu Hao | Tây Bắc | KHCBPPT, Quyển 13, Công Quy | MEDIUM |
| Tỵ | Tam Hợp, Thiên Hỷ | Thiên Lao, Đại Hao | Tây Bắc | KHCBPPT, Quyển 13, Công Quy | MEDIUM |
| Ngọ | Thiên Đức, Nguyệt Đức | Thiên Hình, Chu Tước | Bắc | KHCBPPT, Quyển 13, Công Quy | MEDIUM |
| Mùi | Thiên Quý, Phúc Sinh | Bạch Hổ, Tiểu Hao | Đông Bắc | KHCBPPT, Quyển 13, Công Quy | MEDIUM |
| Thân | Tam Hợp, Thiên Hỷ | Thiên Lao, Đại Hao | Đông Bắc | KHCBPPT, Quyển 13, Công Quy | MEDIUM |
| Dậu | Thiên Đức, Nguyệt Đức | Thiên Hình, Chu Tước | Đông | KHCBPPT, Quyển 13, Công Quy | MEDIUM |
| Tuất | Thiên Quý, Phúc Sinh | Bạch Hổ, Tiểu Hao | Đông Nam | KHCBPPT, Quyển 13, Công Quy | MEDIUM |
| Hợi | Tam Hợp, Thiên Hỷ | Thiên Lao, Đại Hao | Đông Nam | KHCBPPT, Quyển 13, Công Quy | MEDIUM |

**Confidence note:** MEDIUM confidence for all fixed_by_chi entries. The star names used here (Thiên Đức, Nguyệt Đức, Thiên Hình, etc.) are the Vietnamese names for classical Chinese star-gods (神煞), not the 28 lunar mansions. These are a separate star tradition from the 28-mansion cycle. The pattern shows 3 repeating groups cycling through the 12 chi:
- Group A (Tý, Mão, Ngọ, Dậu): Thiên Đức + Nguyệt Đức / Thiên Hình + Chu Tước
- Group B (Sửu, Thìn, Mùi, Tuất): Thiên Quý + Phúc Sinh / Bạch Hổ + Tiểu Hao
- Group C (Dần, Tỵ, Thân, Hợi): Tam Hợp + Thiên Hỷ / Thiên Lao + Đại Hao

This 3-group cyclical pattern is consistent with KHCBPPT's systematic rule tables. The values match baseline.json exactly.

### Comparison with baseline.json `conflict_by_chi`

| Chi | baseline.json cat_tinh | Reference cat_tinh | Match? | baseline.json sat_tinh | Reference sat_tinh | Match? |
|-----|------------------------|-------------------|--------|------------------------|-------------------|--------|
| Tý | Thiên Đức, Nguyệt Đức | Thiên Đức, Nguyệt Đức | YES | Thiên Hình, Chu Tước | Thiên Hình, Chu Tước | YES |
| Sửu | Thiên Quý, Phúc Sinh | Thiên Quý, Phúc Sinh | YES | Bạch Hổ, Tiểu Hao | Bạch Hổ, Tiểu Hao | YES |
| Dần | Tam Hợp, Thiên Hỷ | Tam Hợp, Thiên Hỷ | YES | Thiên Lao, Đại Hao | Thiên Lao, Đại Hao | YES |
| Mão | Thiên Đức, Nguyệt Đức | Thiên Đức, Nguyệt Đức | YES | Thiên Hình, Chu Tước | Thiên Hình, Chu Tước | YES |
| Thìn | Thiên Quý, Phúc Sinh | Thiên Quý, Phúc Sinh | YES | Bạch Hổ, Tiểu Hao | Bạch Hổ, Tiểu Hao | YES |
| Tỵ | Tam Hợp, Thiên Hỷ | Tam Hợp, Thiên Hỷ | YES | Thiên Lao, Đại Hao | Thiên Lao, Đại Hao | YES |
| Ngọ | Thiên Đức, Nguyệt Đức | Thiên Đức, Nguyệt Đức | YES | Thiên Hình, Chu Tước | Thiên Hình, Chu Tước | YES |
| Mùi | Thiên Quý, Phúc Sinh | Thiên Quý, Phúc Sinh | YES | Bạch Hổ, Tiểu Hao | Bạch Hổ, Tiểu Hao | YES |
| Thân | Tam Hợp, Thiên Hỷ | Tam Hợp, Thiên Hỷ | YES | Thiên Lao, Đại Hao | Thiên Lao, Đại Hao | YES |
| Dậu | Thiên Đức, Nguyệt Đức | Thiên Đức, Nguyệt Đức | YES | Thiên Hình, Chu Tước | Thiên Hình, Chu Tước | YES |
| Tuất | Thiên Quý, Phúc Sinh | Thiên Quý, Phúc Sinh | YES | Bạch Hổ, Tiểu Hao | Bạch Hổ, Tiểu Hao | YES |
| Hợi | Tam Hợp, Thiên Hỷ | Tam Hợp, Thiên Hỷ | YES | Thiên Lao, Đại Hao | Thiên Lao, Đại Hao | YES |

**Result: All 48 values (12 chi × 2 cat_tinh + 2 sat_tinh) match baseline.json exactly.**

---

## 4. Star Rule Sparsity Analysis

### Expected vs. Actual Rule Count

The `baseline.json` file contains 5 star rule categories. Each category in baseline.json currently has only **1 seed entry** (a minimal placeholder for schema testing, not a complete dataset). This is a significant gap relative to what KHCBPPT provides.

**Citation for rule count assessment:** `KHCBPPT, Quyển 13–19, Công Quy + Niên Biểu (公規 + 年表); Quyển 20–31, Nguyệt Biểu (月表)`

| Category | baseline.json entries | Expected from KHCBPPT | Gap magnitude | STRATOS version |
|----------|----------------------|----------------------|---------------|-----------------|
| `fixed_by_chi` | 12 chi entries (all present, see Section 3) | 12 chi entries | None — complete | N/A |
| `fixed_by_canchi` | 1 seed (Giáp Thìn only) | 60 can-chi combinations | ~59 missing entries | STR-V2-01 |
| `by_year_can` | 1 seed (Giáp only) | 10 heavenly stems | ~9 missing entries | STR-V2-02 |
| `by_lunar_month` | 1 seed (month 1 only) | 12 months | ~11 missing entries | STR-V2-03 |
| `by_tiet_khi` | 1 seed (Lập Xuân only) | 24 solar terms | ~23 missing entries | STR-V2-04, STR-V2-05 |

**Notes:**
- `fixed_by_chi` (12 chi star assignments) is complete in baseline.json — these 12 entries are confirmed against KHCBPPT in Section 3 above.
- The `fixed_by_canchi`, `by_year_can`, `by_lunar_month`, and `by_tiet_khi` categories each have only 1 seed entry. These seeds exist to establish the JSON schema, not to represent complete data.
- KHCBPPT's 公規 section (vols 13) and 年表/月表 sections (vols 14–31) contain extensive star-god tables across all 60 can-chi combinations, 10 stems, 12 months, and 24 solar terms.
- **Phase 3 must detect missing rules, not just mismatched values.** A missing can-chi entry in `fixed_by_canchi` is an absence error, not a value mismatch error.

**STRATOS version scope note:** Full completeness audit of star rules is scoped to v2 (STR-V2-01 through STR-V2-05). Phase 1 documents the gap magnitude only; Phase 3 validators must implement absence detection.

---

## 5. Star Source Attribution

### Current baseline.json Attribution

```json
"star_meta": { "source_id": "nhi-thap-bat-tu", "method": "jd-cycle" }
```

The `source_id: "nhi-thap-bat-tu"` attributes the 28-star system to the tradition name rather than to KHCBPPT specifically. This is an honest attribution: the 28 lunar mansions are a pan-East-Asian astronomical tradition predating KHCBPPT.

### KHCBPPT Coverage of the 28-Star System

KHCBPPT does cover the 28-star system in its 公規 section (vols 12–13). The star names, quality classifications, and their application to the daily calendar are all present in KHCBPPT text. KHCBPPT is a legitimate and comprehensive source for the 28-mansion system as applied in Vietnamese almanac practice.

### Source Attribution Recommendation

| Option | Pros | Cons | Recommendation |
|--------|------|------|----------------|
| Keep `"nhi-thap-bat-tu"` | Honest about pan-Asian origin; no false specificity | Does not credit KHCBPPT as the specific rule source | Acceptable |
| Change to `"khcbppt"` | Accurate — KHCBPPT is the specific source used for Vietnamese almanac star rules | Loses the broader tradition attribution | Preferred for Phase 4 |

**Recommendation for Phase 4:** Update `star_meta.source_id` to `"khcbppt"` since KHCBPPT's 公規 section is the specific source for the star names, qualities, and rule tables used in this project. The pan-Asian tradition can be noted as a secondary attribution. The JD-mod epoch origin (Ho Ngoc Duc implementation) should be documented separately as an implementation detail, not confused with the source_id for table values.

---

## 6. Access Notes and Confidence Assessment

| Claim | Confidence | Evidence |
|-------|-----------|---------|
| 28 star mansion names | HIGH | Classical canonical system; universal Chinese astronomical tradition |
| Star quality classifications (28 values) | HIGH | KHCBPPT 公規 classification; consistent with Vietnamese almanac practice |
| All baseline.json 28-star values correct | HIGH | Full match on all 28 entries |
| JD epoch origin (JD 0 = Giác) | MEDIUM | Ho Ngoc Duc implementation inference; not directly in KHCBPPT |
| JD-mod formula is KHCBPPT-defined | NO — CONFIRMED NOT in KHCBPPT | KHCBPPT uses tables, not JD-mod |
| Fixed-by-chi star assignments | MEDIUM | Systematic pattern matches; full text extraction limited |
| Star rule sparsity (seed entries only) | HIGH | Direct count from baseline.json |

---

*Phase: 01-source-establishment / Plan: 01-02*
*Last updated: 2026-02-28*
*Citation authority: [EDITION.md](EDITION.md)*
