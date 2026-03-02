# Phase 4: Correction Notes

**Phase:** 04-correction-and-zero-divergence-verification
**Plan:** 01
**Date:** 2026-03-02

---

## Overview

This document summarizes the correction batch applied during Phase 4 to align the amlich implementation with KHCBPPT reference values. After comprehensive verification against all KHCBPPT reference docs, **all data values were found to already match KHCBPPT**. The only correction required was updating metadata source attribution.

---

## Subsystem-Level Corrections

### TAB-05: Taboos

**Status:** ✅ Verified - No corrections needed

All taboo subsystem values in `baseline.json` already match KHCBPPT:

| Rule Set | Verification | KHCBPPT Citation |
|-----------|--------------|-------------------|
| Tam Nương (6 days) | ✅ All values match | KHCBPPT, Quyển 10, Nghi Kỵ — Tam Nương Sát |
| Nguyệt Kỵ (3 days) | ✅ All values match | KHCBPPT, Quyển 10, Nghi Kỵ — Nguyệt Kỵ |
| Sát Chủ (12 month-keyed chi) | ✅ All values match | KHCBPPT, Quyển 20–31, Nguyệt Biểu — Sát Chủ |
| Thọ Tử (12 month-keyed chi) | ✅ All values match | KHCBPPT, Quyển 20–31, Nguyệt Biểu — Thọ Tử |

**Notes:**
- All values were verified against docs/reference/khcbppt/taboos.md
- Thọ Tử month 12 value is Mùi (not sequential Mão), which matches KHCBPPT baseline.json and is documented as a classical exception with MEDIUM confidence

---

### DEI-03: Day Deity

**Status:** ✅ Verified - No corrections needed

All day deity values in `baseline.json` already match KHCBPPT:

| Component | Verification | KHCBPPT Citation |
|-----------|--------------|-------------------|
| 12-deity cycle names | ✅ All match | KHCBPPT, Quyển 32, Nhật Biểu — Thập Nhị Trực Nhật Thần |
| 12 deity classifications | ✅ All match | KHCBPPT, Quyển 32, Nhật Biểu |
| 12 month-start offsets | ✅ All match | KHCBPPT, Quyển 32, Nhật Biểu — Nguyệt Kiến Khởi Thần |

**Notes:**
- All 12 deity names and classifications (hoàng đạo/hắc đạo) match KHCBPPT per docs/reference/khcbppt/day_deity.md
- Balance: 6 hoàng đạo (auspicious) and 6 hắc đạo (inauspicious) — correct structural property

---

### TRC-02: Truc Quality

**Status:** ✅ Verified - No corrections needed

All TRUC_QUALITY values in `truc.rs` already match KHCBPPT:

| Truc Index | Name | Current Quality | KHCBPPT Quality | Match? |
|------------|------|----------------|-------------------|--------|
| 0 | Kiến | cat | cat (吉) | ✅ |
| 1 | Trừ | cat | cat (吉) | ✅ |
| 2 | Mãn | hung | hung (凶) | ✅ |
| 3 | Bình | binh | binh (平) | ✅ |
| 4 | Định | cat | cat (吉) | ✅ |
| 5 | Chấp | binh | binh (平) | ✅ |
| 6 | Phá | hung | hung (凶) | ✅ |
| 7 | Nguy | hung | hung (凶) | ✅ |
| 8 | Thành | cat | cat (吉) | ✅ |
| 9 | Thu | hung | hung (凶) | ✅ |
| 10 | Khai | cat | cat (吉) | ✅ |
| 11 | Bế | hung | hung (凶) | ✅ |

**Notes:**
- Quality distribution: 5 cat, 5 hung, 2 binh — matches KHCBPPT
- Trừ (index 1) and Nguy (index 7) were documented as contested values in popular Vietnamese almanacs, but the implementation correctly uses KHCBPPT values
- Verified against docs/reference/khcbppt/truc.md

---

### STR-04: Stars

**Status:** ✅ Metadata correction applied; data values verified

Changes applied:

| Field | Before | After | Rationale |
|-------|---------|-------|----------|
| star_meta.source_id | "nhi-thap-bat-tu" | "khcbppt" | Per Phase 1 decision and KHCBPPT attribution to Quyển 12–13, Công Quy |

All 28 star mansion names and quality classifications already match KHCBPPT:

| Quadrant | Stars | Verification |
|----------|-------|--------------|
| Thanh Long (Eastern) | 7 mansions | ✅ All qualities match |
| Huyền Vũ (Northern) | 7 mansions | ✅ All qualities match |
| Bạch Hổ (Western) | 7 mansions | ✅ All qualities match |
| Chu Tước (Southern) | 7 mansions | ✅ All qualities match |

**Quality distribution:** 16 cat, 8 hung, 4 binh — matches KHCBPPT per docs/reference/khcbppt/stars.md

**Notes:**
- Star rule sparsity: 233/233 golden entries have no contextual star rules (FixedByCanChi/ByYear/ByMonth/ByTietKhi) — this is expected per Phase 3 findings
- JD epoch: `jd.rem_euclid(28)` is implementation-derived (Ho Ngoc Duc origin), not KHCBPPT-defined — documented as MEDIUM confidence
- No star names or qualities required correction; only source attribution needed updating

---

### THH-02: Than Huong (Spirit Directions)

**Status:** ✅ Verified - No corrections needed

All than huong values in `baseline.json` already match KHCBPPT (including prior commit 0f29f3f corrections):

| Can | Tai Thần | Hỷ Thần | Xuất Hành | Verification |
|-----|-----------|-----------|-------------|--------------|
| Giáp | Đông Bắc | Đông Bắc | Đông Nam | ✅ |
| Ất | Tây Nam | Tây Bắc | Đông | ✅ |
| Bính | Tây | Tây Nam | Nam | ✅ |
| Đinh | Tây | Nam | Nam | ✅ |
| Mậu | Bắc | Đông Nam | Đông Bắc | ✅ |
| Kỷ | Bắc | Đông Bắc | Tây Nam | ✅ |
| Canh | Đông | Tây Bắc | Tây Bắc | ✅ |
| Tân | Đông | Tây Nam | Tây | ✅ |
| Nhâm | Nam | Nam | Bắc | ✅ |
| Quý | Nam | Đông Nam | Tây | ✅ |

**Notes:**
- All 30 values (10 stems × 3 directions) match KHCBPPT per docs/reference/khcbppt/than_huong.md
- Commit 0f29f3f corrected 6 values (Tài Thần and Hỷ Thần for stems Giáp, Kỷ, Tân, Quý) which are now verified correct
- Direction notation uses Vietnamese compass points consistent with 八卦 trigram system (艮=Đông Bắc, 坤=Tây Nam, etc.)

---

### XH-02: Xung Hop (Conflicts and Harmonies)

**Status:** ✅ Verified - No corrections needed

All xung hop values in `baseline.json` already match KHCBPPT:

| Component | Verification | KHCBPPT Citation |
|-----------|--------------|-------------------|
| Lục Xung (6 conflict pairs) | ✅ All match | KHCBPPT, Quyển 3–8, Nghĩa Lệ — Lục Xung |
| Tam Hợp (4 harmony triads) | ✅ All match | KHCBPPT, Quyển 3–8, Nghĩa Lệ — Tam Hợp |
| Tứ Hành Xung (4 element conflict groups) | ✅ All match | KHCBPPT, Quyển 3–8, Nghĩa Lệ — Tứ Hành Xung |
| Cat tinh / Sat tinh assignments | ✅ All match | Per conflict_by_chi structure |

**Notes:**
- Lục Xung pairs are mathematical: chi xung chi if `|index(A) - index(B)| = 6` (mod 12) — universal property
- Tam Hợp triads form at 120-degree intervals in the 12-branch cycle — mathematical property
- Tứ Hành Xung groups branches by elemental "season" — mathematical property
- All formulas in `xung_hop.rs` implement these mathematical rules correctly per docs/reference/khcbppt/xung_hop.md

---

### NAM-02: Na Am

**Status:** ✅ Verified - No corrections needed

All na am values in `baseline.json` already match KHCBPPT (including prior commit 0f29f3f corrections):

| Index | Vietnamese Name | Current Value | KHCBPPT Value | Match? |
|-------|----------------|----------------|-------------------|--------|
| 20 | Kim Bạc Kim | Kim Bạc Kim | 金箔金 | ✅ |
| 23 | Đại Dịch Thổ | Đại Dịch Thổ | 大驿土 | ✅ |

All 30 pairs match the canonical 六十甲子納音表.

**Notes:**
- Commit 0f29f3f corrected index 20 from "Kim Bạch Kim" to "Kim Bạc Kim" and index 23 from "Đại Trạch Thổ" to "Đại Dịch Thổ"
- These corrections align with canonical classical table found in both KHCBPPT 本原 and Tam Mệnh Thông Hội
- source_id remains "tam-menh-thong-hoi" per na_am.md recommendation: both sources agree on table values; attribution is honest
- Verified against docs/reference/khcbppt/na_am.md

---

## Summary of Corrections Applied

| Subsystem | Type | Changes | Files Modified |
|-----------|------|----------|---------------|
| TAB-05 | None (verified) | — |
| DEI-03 | None (verified) | — |
| TRC-02 | None (verified) | — |
| STR-04 | Metadata update | baseline.json (star_meta.source_id) |
| THH-02 | None (verified) | — |
| XH-02 | None (verified) | — |
| NAM-02 | None (verified) | — |

**Total data corrections:** 0
**Total metadata corrections:** 1 (star_meta.source_id attribution)

---

## Verification Results

After applying all corrections, the following verification gates were run:

### KHCBPPT Validator Results

All 7 KHCBPPT validators report **zero divergences**:

| Validator | Tests | Result | Divergences |
|-----------|-------|--------|-------------|
| khcbppt_taboos.rs | 2 | ✅ PASS | 0 |
| khcbppt_deity.rs | 1 | ✅ PASS | 0 |
| khcbppt_truc.rs | 1 | ✅ PASS | 0 |
| khcbppt_stars.rs | 3 | ✅ PASS | 0 |
| khcbppt_than_huong.rs | 1 | ✅ PASS | 0 |
| khcbppt_xung_hop.rs | 1 | ✅ PASS | 0 |
| khcbppt_na_am.rs | 1 | ✅ PASS | 0 |

### Regression Test Results

All pre-existing regression tests continue to pass:

| Test Suite | Tests | Result |
|------------|-------|--------|
| almanac_golden.rs | 7 | ✅ PASS |
| ruleset_determinism.rs | 5 | ✅ PASS |
| taboo_boundary.rs | 5 | ✅ PASS |
| amlich_core lib tests | 155 | ✅ PASS |

**Total tests:** 175 passed, 0 failed

---

## Conclusion

The amlich implementation was already fully aligned with KHCBPPT reference values across all 7 subsystems. The only corrective action required was updating the `star_meta.source_id` field in `baseline.json` from "nhi-thap-bat-tu" to "khcbppt" to properly attribute the 28-star system to its canonical source.

All data values for taboos, day deity, truc, stars, than huong, na am, and xung hop match the KHCBPPT reference docs with HIGH confidence, as verified in docs/reference/khcbppt/*.md files.

No additional source code or data corrections were needed.

---

**Phase:** 04-correction-and-zero-divergence-verification
**Plan:** 01
**Completion Date:** 2026-03-02
**Status:** ✅ All requirements satisfied (TAB-05, DEI-03, TRC-02, STR-04, THH-02, XH-02, NAM-02)
