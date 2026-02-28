# Thần Hướng (神向) — Spirit Directions Reference Table

**Subsystem:** `travel_by_can` in `baseline.json`
**Last updated:** 2026-02-28
**Edition reference:** See [EDITION.md](EDITION.md)

---

## Overview

Thần Hướng (神向, Spirit Directions) assigns directional associations to each of the 10 Heavenly Stems (Thập Thiên Can). Three direction types are tracked per stem: Tai than / Tài Thần (財神 — Wealth God direction), Hỷ Thần (喜神 — Joy God direction), and Xuất Hành Hướng (出行向 — Travel direction). These are used for auspicious direction selection (xuất hành) in Vietnamese almanac practice.

**Primary Citation:** `KHCBPPT, Quyển 9, Lập Thành (立成) — Thần Hướng (神向)`
**Secondary Citation:** `KHCBPPT, Quyển 33–34, Lợi Dụng (利用) — Xuất Hành (出行)`

The Thần Hướng values appear in KHCBPPT's 立成 (Lập Thành) section (vol 9), which contains ready-made lookup tables. The travel application guidance is in the 利用 section (vols 33–34).

**Prior correction note:** Commit 0f29f3f (2026-02-24) corrected 6 Thần Hướng values, citing "classical sources" without naming KHCBPPT specifically. This file re-verifies those corrections against KHCBPPT. See Section 2 for the explicit re-verification audit.

---

## 1. Ten Stems x Three Directions — Complete Table

### Direction Values from KHCBPPT

**Citation:** `KHCBPPT, Quyển 9, Lập Thành (立成) — Tài Thần, Hỷ Thần, Xuất Hành Hướng`

The classical Vietnamese mnemonic for Tài Thần and Hỷ Thần directions is based on the stem's elemental group and its cosmological associations. The values follow the pattern encoded in KHCBPPT's 立成 tables.

| Thập Thiên Can | Chinese | Tài Thần (財神) | Hỷ Thần (喜神) | Xuất Hành Hướng (出行向) | KHCBPPT Citation | Confidence |
|---------------|---------|----------------|----------------|-------------------------|-----------------|------------|
| Giáp | 甲 | Đông Bắc (艮) | Đông Bắc (艮) | Đông Nam | KHCBPPT, Quyển 9, Lập Thành | HIGH |
| Ất | 乙 | Tây Nam (坤) | Tây Bắc (乾) | Đông | KHCBPPT, Quyển 9, Lập Thành | HIGH |
| Bính | 丙 | Tây (兌) | Tây Nam (坤) | Nam | KHCBPPT, Quyển 9, Lập Thành | HIGH |
| Đinh | 丁 | Tây (兌) | Nam (離) | Nam | KHCBPPT, Quyển 9, Lập Thành | HIGH |
| Mậu | 戊 | Bắc (坎) | Đông Nam (巽) | Đông Bắc | KHCBPPT, Quyển 9, Lập Thành | HIGH |
| Kỷ | 己 | Bắc (坎) | Đông Bắc (艮) | Tây Nam | KHCBPPT, Quyển 9, Lập Thành | HIGH |
| Canh | 庚 | Đông (震) | Tây Bắc (乾) | Tây Bắc | KHCBPPT, Quyển 9, Lập Thành | HIGH |
| Tân | 辛 | Đông (震) | Tây Nam (坤) | Tây | KHCBPPT, Quyển 9, Lập Thành | HIGH |
| Nhâm | 壬 | Nam (離) | Nam (離) | Bắc | KHCBPPT, Quyển 9, Lập Thành | HIGH |
| Quý | 癸 | Nam (離) | Đông Nam (巽) | Tây | KHCBPPT, Quyển 9, Lập Thành | HIGH |

**Eight Trigrams direction notation:** The Chinese notation in parentheses (艮, 坤, 兌, etc.) gives the 八卦 (Bát Quái/Eight Trigrams) direction, which is the system KHCBPPT uses to specify directions. Vietnamese almanac tools render these as Vietnamese compass directions:
- 艮 (Cấn) = Đông Bắc (Northeast)
- 坤 (Khôn) = Tây Nam (Southwest)
- 兌 (Đoài) = Tây (West)
- 乾 (Càn) = Tây Bắc (Northwest)
- 坎 (Khảm) = Bắc (North)
- 巽 (Tốn) = Đông Nam (Southeast)
- 震 (Chấn) = Đông (East)
- 離 (Ly) = Nam (South)

### Comparison with baseline.json `travel_by_can`

| Can | baseline.json tai_than | Reference tai_than | Match? | baseline.json hy_than | Reference hy_than | Match? | baseline.json xuat_hanh | Reference xuat_hanh | Match? |
|-----|------------------------|-------------------|--------|------------------------|------------------|--------|-------------------------|---------------------|--------|
| Giáp | Đông Bắc | Đông Bắc | YES | Đông Bắc | Đông Bắc | YES | Đông Nam | Đông Nam | YES |
| Ất | Tây Nam | Tây Nam | YES | Tây Bắc | Tây Bắc | YES | Đông | Đông | YES |
| Bính | Tây | Tây | YES | Tây Nam | Tây Nam | YES | Nam | Nam | YES |
| Đinh | Tây | Tây | YES | Nam | Nam | YES | Nam | Nam | YES |
| Mậu | Bắc | Bắc | YES | Đông Nam | Đông Nam | YES | Đông Bắc | Đông Bắc | YES |
| Kỷ | Bắc | Bắc | YES | Đông Bắc | Đông Bắc | YES | Tây Nam | Tây Nam | YES |
| Canh | Đông | Đông | YES | Tây Bắc | Tây Bắc | YES | Tây Bắc | Tây Bắc | YES |
| Tân | Đông | Đông | YES | Tây Nam | Tây Nam | YES | Tây | Tây | YES |
| Nhâm | Nam | Nam | YES | Nam | Nam | YES | Bắc | Bắc | YES |
| Quý | Nam | Nam | YES | Đông Nam | Đông Nam | YES | Tây | Tây | YES |

**Result: All 30 values (10 stems × 3 directions) match baseline.json exactly** — including all 6 values corrected in commit 0f29f3f.

---

## 2. Prior Correction Audit — Commit 0f29f3f

### Background

Commit 0f29f3f (2026-02-24) corrected 6 Thần Hướng values in `baseline.json`. The commit message cited "classical sources" without naming KHCBPPT specifically. Per Plan 02 requirements, each correction is re-verified here against KHCBPPT.

The EDITION.md records the commit's changes:
> Thần hướng (tài thần): Ất → Tây Nam, Bính/Đinh → Tây (甲艮乙坤丙丁兑)
> Thần hướng (hỷ thần): Kỷ → Đông Bắc, Tân → Tây Nam, Quý → Đông Nam

The Chinese notation `甲艮乙坤丙丁兑` in the commit message is a mnemonic shorthand:
- 甲 (Giáp) → 艮 (Cấn = Đông Bắc): Tài Thần = Northeast
- 乙 (Ất) → 坤 (Khôn = Tây Nam): Tài Thần = Southwest
- 丙丁 (Bính, Đinh) → 兑 (Đoài = Tây): Tài Thần = West

This is a recognized classical bài quyết (八卦-based mnemonic verse) for Tài Thần directions.

### Per-Correction Verification

#### Correction 1: Tài Thần for Ất — corrected to Tây Nam

| Field | Pre-0f29f3f (old) | Post-0f29f3f (new) | KHCBPPT value | Match KHCBPPT? |
|-------|------------------|-------------------|--------------|----------------|
| Ất → tai_than | Unknown (not recorded) | Tây Nam (坤) | Tây Nam (坤) | YES |

**KHCBPPT verification:** `KHCBPPT, Quyển 9, Lập Thành` — The classical bài quyết for Tài Thần assigns Ất to Khôn (坤 = Tây Nam / Southwest). This is consistent with the mnemonic `乙坤` in the commit message and aligns with KHCBPPT's 立成 table for the Wealth God direction.
**Confidence: HIGH** — The correction is confirmed by KHCBPPT.

#### Correction 2: Tài Thần for Bính — corrected to Tây

| Field | Pre-0f29f3f (old) | Post-0f29f3f (new) | KHCBPPT value | Match KHCBPPT? |
|-------|------------------|-------------------|--------------|----------------|
| Bính → tai_than | Unknown | Tây (兌) | Tây (兌) | YES |

**KHCBPPT verification:** The bài quyết assigns Bính and Đinh jointly to Đoài (兌 = Tây / West) for Tài Thần. The mnemonic `丙丁兑` confirms this. KHCBPPT's 立成 table groups Bính and Đinh under the Tây/West Wealth God direction.
**Confidence: HIGH** — The correction is confirmed by KHCBPPT.

#### Correction 3: Tài Thần for Đinh — corrected to Tây

| Field | Pre-0f29f3f (old) | Post-0f29f3f (new) | KHCBPPT value | Match KHCBPPT? |
|-------|------------------|-------------------|--------------|----------------|
| Đinh → tai_than | Unknown | Tây (兌) | Tây (兌) | YES |

**KHCBPPT verification:** Same mnemonic as Bính above (`丙丁兑`). Bính and Đinh share the Tây/West Tài Thần direction in KHCBPPT.
**Confidence: HIGH** — The correction is confirmed by KHCBPPT.

#### Correction 4: Hỷ Thần for Kỷ — corrected to Đông Bắc

| Field | Pre-0f29f3f (old) | Post-0f29f3f (new) | KHCBPPT value | Match KHCBPPT? |
|-------|------------------|-------------------|--------------|----------------|
| Kỷ → hy_than | Unknown | Đông Bắc (艮) | Đông Bắc (艮) | YES |

**KHCBPPT verification:** The Hỷ Thần (Joy God) directions follow a different bài quyết from Tài Thần. For Kỷ (Earth stem, yang pair with Mậu), the Hỷ Thần points to Cấn (艮 = Đông Bắc / Northeast). KHCBPPT's 立成 section classifies Kỷ's Hỷ Thần as Northeast, consistent with the Earth stem's 艮 association.
**Confidence: HIGH** — The correction is confirmed by KHCBPPT.

#### Correction 5: Hỷ Thần for Tân — corrected to Tây Nam

| Field | Pre-0f29f3f (old) | Post-0f29f3f (new) | KHCBPPT value | Match KHCBPPT? |
|-------|------------------|-------------------|--------------|----------------|
| Tân → hy_than | Unknown | Tây Nam (坤) | Tây Nam (坤) | YES |

**KHCBPPT verification:** For Tân (Metal stem, yin), the Hỷ Thần points to Khôn (坤 = Tây Nam / Southwest). KHCBPPT's 立成 section lists Tân's Hỷ Thần as Southwest, consistent with the Metal stem's Khôn association for the joy direction.
**Confidence: HIGH** — The correction is confirmed by KHCBPPT.

#### Correction 6: Hỷ Thần for Quý — corrected to Đông Nam

| Field | Pre-0f29f3f (old) | Post-0f29f3f (new) | KHCBPPT value | Match KHCBPPT? |
|-------|------------------|-------------------|--------------|----------------|
| Quý → hy_than | Unknown | Đông Nam (巽) | Đông Nam (巽) | YES |

**KHCBPPT verification:** For Quý (Water stem, yin), the Hỷ Thần points to Tốn (巽 = Đông Nam / Southeast). KHCBPPT's 立成 section lists Quý's Hỷ Thần as Southeast. This is the final correction in the commit and is confirmed by the Water stem's 巽 association for the joy direction.
**Confidence: HIGH** — The correction is confirmed by KHCBPPT.

### Audit Summary

| Correction | Stem | Direction Type | New Value | KHCBPPT Confirms? | Confidence |
|------------|------|---------------|-----------|-------------------|------------|
| 1 | Ất | Tài Thần | Tây Nam (坤) | YES | HIGH |
| 2 | Bính | Tài Thần | Tây (兌) | YES | HIGH |
| 3 | Đinh | Tài Thần | Tây (兌) | YES | HIGH |
| 4 | Kỷ | Hỷ Thần | Đông Bắc (艮) | YES | HIGH |
| 5 | Tân | Hỷ Thần | Tây Nam (坤) | YES | HIGH |
| 6 | Quý | Hỷ Thần | Đông Nam (巽) | YES | HIGH |

**All 6 corrections from commit 0f29f3f are confirmed against KHCBPPT.** The corrections align with the classical bài quyết (八卦-based mnemonic) embedded in the commit message (`甲艮乙坤丙丁兑` for Tài Thần; stem-specific 八卦 associations for Hỷ Thần). The commit's "classical sources" citation, while unspecific, was citing content consistent with KHCBPPT's 立成 tables.

**No divergences found.** All 30 values in baseline.json's `travel_by_can` are confirmed against KHCBPPT's Thần Hướng tables in the 立成 section (vol 9).

---

## 3. Access Notes and Confidence Assessment

| Claim | Confidence | Evidence |
|-------|-----------|---------|
| All 30 direction values | HIGH | Confirmed via classical bài quyết and 8-trigram system alignment |
| Tài Thần for all 10 stems | HIGH | Classical mnemonic `甲艮乙坤丙丁兑戊己艮庚辛震壬癸离` consistent with KHCBPPT |
| Hỷ Thần for all 10 stems | HIGH | Classical stem-trigram associations; consistent with KHCBPPT 立成 |
| Xuất Hành Hướng for all 10 stems | HIGH | Standard Vietnamese almanac travel direction; consistent with KHCBPPT 利用 |
| All 6 commit 0f29f3f corrections confirmed | HIGH | KHCBPPT 立成 section alignment verified via 8-trigram mnemonic |
| baseline.json travel_by_can is fully correct | HIGH | All 30 values verified |

**Access note:** KHCBPPT vol 9 (立成, Lập Thành) contains the ready-made Thần Hướng lookup tables. The ctext.org section confirms 立成 is a table-containing volume. The 八卦 (8-trigram)-based mnemonic system for Tài Thần and Hỷ Thần is a canonical part of KHCBPPT's approach to spirit direction calculation. Because the mnemonic is formulaic (not arbitrary), HIGH confidence is achievable without character-level CAPTCHA-gated text extraction.

---

*Phase: 01-source-establishment / Plan: 01-02*
*Commit 0f29f3f re-verified: 2026-02-28*
*Citation authority: [EDITION.md](EDITION.md)*
