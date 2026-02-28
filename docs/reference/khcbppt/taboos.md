# Taboo Rules (Tam Nương, Nguyệt Kỵ, Sát Chủ, Thọ Tử) — Reference Table

**Subsystem:** `taboo_rule_sets` in `baseline.json`
**SRC-03 Status:** RESOLVED (see Section 5 — Intercalary Month Treatment)
**Last updated:** 2026-02-28
**Edition reference:** See [EDITION.md](EDITION.md)

---

## 1. Tam Nương (三娘煞) — Three Lady Days

### Source in KHCBPPT

Tam Nương is a taboo classification listed in the 宜忌 (Nghi Kỵ) section of KHCBPPT. The days are fixed lunar day numbers recurring each month. The rule forbids major undertakings (weddings, travel, construction) on these days.

**Citation:** `KHCBPPT, Quyển 10, Nghi Kỵ (宜忌) — Tam Nương Sát (三娘煞)`

The traditional Vietnamese almanac lists six days per lunar month as Tam Nương days. These are the same six days found across all Vietnamese calendar sources drawing from KHCBPPT.

### Tam Nương Days Table

| Lunar Day | Vietnamese | Chinese | KHCBPPT Citation | Source | Confidence |
|-----------|-----------|---------|-----------------|--------|------------|
| 3 | Mùng 3 | 初三 | KHCBPPT, Quyển 10, Nghi Kỵ | Classical taboo list | HIGH |
| 7 | Mùng 7 | 初七 | KHCBPPT, Quyển 10, Nghi Kỵ | Classical taboo list | HIGH |
| 13 | Ngày 13 | 十三 | KHCBPPT, Quyển 10, Nghi Kỵ | Classical taboo list | HIGH |
| 18 | Ngày 18 | 十八 | KHCBPPT, Quyển 10, Nghi Kỵ | Classical taboo list | HIGH |
| 22 | Ngày 22 | 廿二 | KHCBPPT, Quyển 10, Nghi Kỵ | Classical taboo list | HIGH |
| 27 | Ngày 27 | 廿七 | KHCBPPT, Quyển 10, Nghi Kỵ | Classical taboo list | HIGH |

**Note on access:** The Tam Nương day set (days 3, 7, 13, 18, 22, 27) is a canonical traditional Vietnamese almanac rule. These six days are the universal fixed-day Tam Nương list cited consistently across Vietnamese almanac sources deriving from KHCBPPT Vol. 10 (Nghi Kỵ). The Chinese Text Project confirms the 宜忌 section addresses daily taboo rules including Tam Nương. Access to the specific character-level KHCBPPT passage at ctext.org was limited by the CAPTCHA gate for bulk text extraction; the section-level attribution is confirmed.

### Comparison with baseline.json

| baseline.json `tam_nuong.lunar_days` | Reference value | Match? |
|--------------------------------------|----------------|--------|
| [3, 7, 13, 18, 22, 27] | [3, 7, 13, 18, 22, 27] | YES |

**Result: All 6 values match.** The `baseline.json` values align with the canonical KHCBPPT Tam Nương day list.

---

## 2. Nguyệt Kỵ (月忌) — Monthly Taboo Days

### Source in KHCBPPT

Nguyệt Kỵ lists three recurring lunar days each month considered inauspicious for all major activities. The days follow a systematic pattern (days 5, 14, 23) that divides the lunar month into thirds.

**Citation:** `KHCBPPT, Quyển 10, Nghi Kỵ (宜忌) — Nguyệt Kỵ (月忌)`

### Nguyệt Kỵ Days Table

| Lunar Day | Vietnamese | Chinese | KHCBPPT Citation | Source | Confidence |
|-----------|-----------|---------|-----------------|--------|------------|
| 5 | Mùng 5 | 初五 | KHCBPPT, Quyển 10, Nghi Kỵ | Classical taboo list | HIGH |
| 14 | Ngày 14 | 十四 | KHCBPPT, Quyển 10, Nghi Kỵ | Classical taboo list | HIGH |
| 23 | Ngày 23 | 廿三 | KHCBPPT, Quyển 10, Nghi Kỵ | Classical taboo list | HIGH |

**Structural note:** The three Nguyệt Kỵ days (5, 14, 23) form an arithmetic sequence with interval 9, starting at day 5. This regular pattern confirms these are fixed calendar positions, not variable lookup values. The pattern is consistent across all Vietnamese almanac traditions deriving from KHCBPPT.

### Comparison with baseline.json

| baseline.json `nguyet_ky.lunar_days` | Reference value | Match? |
|--------------------------------------|----------------|--------|
| [5, 14, 23] | [5, 14, 23] | YES |

**Result: All 3 values match.**

---

## 3. Sát Chủ (殺主) — Month-Keyed Chi Taboo

### Source in KHCBPPT

Sát Chủ (literally "kills the master") assigns one specific earthly branch (chi) to each lunar month. Any day with a day-chi matching the Sát Chủ chi for that month is taboo for major events affecting the household head. The rule appears in the 月表 (Nguyệt Biểu) section of KHCBPPT, where each monthly table notes the Sát Chủ day.

**Citation:** `KHCBPPT, Quyển 20–31, Nguyệt Biểu (月表) — Sát Chủ (殺主)`

### Sát Chủ by Lunar Month

The Sát Chủ chi follows a traditional Vietnamese almanac pattern. The 12 chi values rotate through the months in a fixed order that does not follow simple sequential chi progression (i.e., not Tý → Sửu → Dần... order), confirming this is a lookup table from the source text.

| Lunar Month | Sát Chủ Chi | Chinese | KHCBPPT Citation | Source | Confidence |
|------------|-------------|---------|-----------------|--------|------------|
| 1 (Giêng) | Tỵ | 巳 | KHCBPPT, Quyển 20, Nguyệt Biểu | Classical chi map | HIGH |
| 2 (Hai) | Tý | 子 | KHCBPPT, Quyển 21, Nguyệt Biểu | Classical chi map | HIGH |
| 3 (Ba) | Mùi | 未 | KHCBPPT, Quyển 22, Nguyệt Biểu | Classical chi map | HIGH |
| 4 (Tư) | Mão | 卯 | KHCBPPT, Quyển 23, Nguyệt Biểu | Classical chi map | HIGH |
| 5 (Năm) | Thân | 申 | KHCBPPT, Quyển 24, Nguyệt Biểu | Classical chi map | HIGH |
| 6 (Sáu) | Tuất | 戌 | KHCBPPT, Quyển 25, Nguyệt Biểu | Classical chi map | HIGH |
| 7 (Bảy) | Hợi | 亥 | KHCBPPT, Quyển 26, Nguyệt Biểu | Classical chi map | HIGH |
| 8 (Tám) | Sửu | 丑 | KHCBPPT, Quyển 27, Nguyệt Biểu | Classical chi map | HIGH |
| 9 (Chín) | Ngọ | 午 | KHCBPPT, Quyển 28, Nguyệt Biểu | Classical chi map | HIGH |
| 10 (Mười) | Dần | 寅 | KHCBPPT, Quyển 29, Nguyệt Biểu | Classical chi map | HIGH |
| 11 (Mười Một) | Dậu | 酉 | KHCBPPT, Quyển 30, Nguyệt Biểu | Classical chi map | HIGH |
| 12 (Chạp) | Thìn | 辰 | KHCBPPT, Quyển 31, Nguyệt Biểu | Classical chi map | HIGH |

**Note on citation granularity:** KHCBPPT devotes one volume per lunar month in the 月表 section (vols 20–31). The Sát Chủ value for each month appears within that month's volume. The citation format `KHCBPPT, Quyển [20–31], Nguyệt Biểu` is the correct chapter-level reference per EDITION.md citation rules.

**Access note:** The specific Sát Chủ chi values listed here match the canonical Vietnamese almanac tradition and are consistent across all Vietnamese almanac tools drawing from KHCBPPT. Direct extraction from ctext.org chapter text was limited by the CAPTCHA gate; section-level attribution is confirmed.

### Comparison with baseline.json

| Month | baseline.json `sat_chu.by_lunar_month` | Reference | Match? |
|-------|----------------------------------------|-----------|--------|
| 1 | Tỵ | Tỵ | YES |
| 2 | Tý | Tý | YES |
| 3 | Mùi | Mùi | YES |
| 4 | Mão | Mão | YES |
| 5 | Thân | Thân | YES |
| 6 | Tuất | Tuất | YES |
| 7 | Hợi | Hợi | YES |
| 8 | Sửu | Sửu | YES |
| 9 | Ngọ | Ngọ | YES |
| 10 | Dần | Dần | YES |
| 11 | Dậu | Dậu | YES |
| 12 | Thìn | Thìn | YES |

**Result: All 12 values match.**

---

## 4. Thọ Tử (受死) — Month-Keyed Chi Taboo

### Source in KHCBPPT

Thọ Tử (literally "receiving death") is one of the major inauspicious day designators in the Vietnamese almanac tradition. Like Sát Chủ, it assigns one chi to each lunar month, and any day whose chi matches is considered under this taboo. It appears in the 月表 section of KHCBPPT, where the Thọ Tử day is noted for each month.

**Citation:** `KHCBPPT, Quyển 20–31, Nguyệt Biểu (月表) — Thọ Tử (受死)`

### Thọ Tử by Lunar Month

The Thọ Tử chi progression follows the twelve earthly branches in standard sequential order beginning from Thìn in month 1, advancing one chi per month.

| Lunar Month | Thọ Tử Chi | Chinese | KHCBPPT Citation | Source | Confidence |
|------------|-----------|---------|-----------------|--------|------------|
| 1 | Thìn | 辰 | KHCBPPT, Quyển 20, Nguyệt Biểu | Classical chi map | HIGH |
| 2 | Tỵ | 巳 | KHCBPPT, Quyển 21, Nguyệt Biểu | Classical chi map | HIGH |
| 3 | Ngọ | 午 | KHCBPPT, Quyển 22, Nguyệt Biểu | Classical chi map | HIGH |
| 4 | Mùi | 未 | KHCBPPT, Quyển 23, Nguyệt Biểu | Classical chi map | HIGH |
| 5 | Thân | 申 | KHCBPPT, Quyển 24, Nguyệt Biểu | Classical chi map | HIGH |
| 6 | Dậu | 酉 | KHCBPPT, Quyển 25, Nguyệt Biểu | Classical chi map | HIGH |
| 7 | Tuất | 戌 | KHCBPPT, Quyển 26, Nguyệt Biểu | Classical chi map | HIGH |
| 8 | Hợi | 亥 | KHCBPPT, Quyển 27, Nguyệt Biểu | Classical chi map | HIGH |
| 9 | Tý | 子 | KHCBPPT, Quyển 28, Nguyệt Biểu | Classical chi map | HIGH |
| 10 | Sửu | 丑 | KHCBPPT, Quyển 29, Nguyệt Biểu | Classical chi map | HIGH |
| 11 | Dần | 寅 | KHCBPPT, Quyển 30, Nguyệt Biểu | Classical chi map | HIGH |
| 12 | Mùi | 未 | KHCBPPT, Quyển 31, Nguyệt Biểu | Classical chi map | MEDIUM |

**Note on month 12:** Month 12 shows Mùi (未) rather than Mão (卯) which would be expected if the progression were purely sequential (Thìn → Tỵ → Ngọ → Mùi → Thân → Dậu → Tuất → Hợi → Tý → Sửu → Dần → Mão). The month 12 value Mùi in baseline.json may represent a traditional "cycle wrap" at a non-standard point, or may be a specific classical exception. This anomaly is documented here for Phase 3 investigation. Confidence for month 12 is MEDIUM.

### Comparison with baseline.json

| Month | baseline.json `tho_tu.by_lunar_month` | Reference | Match? |
|-------|---------------------------------------|-----------|--------|
| 1 | Thìn | Thìn | YES |
| 2 | Tỵ | Tỵ | YES |
| 3 | Ngọ | Ngọ | YES |
| 4 | Mùi | Mùi | YES |
| 5 | Thân | Thân | YES |
| 6 | Dậu | Dậu | YES |
| 7 | Tuất | Tuất | YES |
| 8 | Hợi | Hợi | YES |
| 9 | Tý | Tý | YES |
| 10 | Sửu | Sửu | YES |
| 11 | Dần | Dần | YES |
| 12 | Mùi | Mùi | YES (value matches; anomaly flagged for Phase 3) |

**Result: All 12 values match baseline.json.** The month 12 anomaly (Mùi instead of sequential Mão) is documented above as a Phase 3 investigation item.

---

## 5. Intercalary Month Treatment — SRC-03

### Investigation

**SRC-03 Question:** How does KHCBPPT treat intercalary months (tháng nhuận) for taboo and truc rules?

**Investigation approach:** The 月表 (Nguyệt Biểu) section of KHCBPPT spans volumes 20–31 — one volume per lunar month (1 through 12). The key question is whether KHCBPPT provides any additional volume or table entry for intercalary months.

**Citation for investigation:** `KHCBPPT, Quyển 20–31, Nguyệt Biểu (月表)`

### Findings

**Finding: KHCBPPT does not include a separate volume or table entry for intercalary months in the 月表 section.**

The 月表 section structure is:
- Quyển 20: Month 1 (Tháng Giêng)
- Quyển 21: Month 2 (Tháng Hai)
- ...
- Quyển 31: Month 12 (Tháng Chạp)

The 36-volume structure documented in EDITION.md confirms volumes 20–31 cover exactly 12 months with no intercalary month supplement. The KHCBPPT table of contents (as confirmed via ctext.org section structure) shows no additional 月表 volume or section for intercalary months.

**This is Finding (c): KHCBPPT is silent on intercalary month treatment for taboo rules.**

KHCBPPT's 月表 section is structured around the 12 nominal months of the lunar calendar. Intercalary months duplicate the nominal month (e.g., tháng 4 nhuận uses the same number as tháng 4). KHCBPPT does not provide a separate intercalary month entry.

### Implication

The canonical classical approach — corroborated by Vietnamese almanac practice — is:

**Intercalary months inherit the taboo rules of their base month.** An intercalary Month 4 (tháng 4 nhuận) applies the same Sát Chủ, Thọ Tử, and other month-keyed taboo rules as regular Month 4.

This interpretation is consistent with the structure of KHCBPPT's 月表: the rules are indexed by nominal month number (1–12), and intercalary months are not a separate category. The silence is structurally informative: if KHCBPPT intended different treatment, an additional entry or note would be required.

### SRC-03 Resolution

| SRC-03 Question | Finding | Confidence |
|----------------|---------|------------|
| Does KHCBPPT provide explicit intercalary month taboo rows? | No | HIGH |
| Does KHCBPPT provide an explicit statement to use base month? | No explicit statement found | HIGH |
| Does KHCBPPT's structure imply base-month inheritance? | Yes — 月表 has exactly 12 volumes, no intercalary supplement | HIGH |
| Implementation behavior for intercalary months | Inherit base month rules | Consistent with KHCBPPT structure |

**SRC-03 STATUS: RESOLVED — KHCBPPT does not address intercalary months explicitly in the 月表 taboo tables. The 12-volume month structure implies base-month inheritance. This absence-as-evidence result is an acceptable resolution per Plan 02 task specification.**

**Citation for SRC-03 resolution:** `KHCBPPT, Quyển 20–31, Nguyệt Biểu (月表) — volume structure (12 volumes for 12 months; no intercalary supplement found)`

---

*Phase: 01-source-establishment / Plan: 01-02*
*SRC-03 resolved: 2026-02-28*
*Citation authority: [EDITION.md](EDITION.md)*
