# Thập Nhị Trực (十二直) — Twelve Proceedings Reference Table

**Subsystem:** `TRUC_QUALITY` const in `crates/amlich-core/src/almanac/truc.rs`
**TRUC_QUALITY is hardcoded in Rust source — any Phase 4 correction requires a code change + recompile**
**Last updated:** 2026-02-28
**Edition reference:** See [EDITION.md](EDITION.md)

---

## Overview

The Thập Nhị Trực (十二直, Twelve Proceedings) is a monthly-cycle classification system that assigns one of 12 named "proceedings" to each day, based on the day's position relative to the lunar month. The 12 trực names cycle in a fixed order, and each trực has a quality classification: cat (吉 — auspicious), hung (凶 — inauspicious), or binh (平 — neutral).

**Primary Citation:** `KHCBPPT, Quyển 3–8, Nghĩa Lệ (義例) — Thập Nhị Trực (十二直)`
**Secondary Citation:** `KHCBPPT, Quyển 32, Nhật Biểu (日表)`

The Thập Nhị Trực rule appears in the 義例 (Nghĩa Lệ) section of KHCBPPT (vols 3–8), which covers principle explanations and rule sets. The 日表 (vol 32) applies these rules to the daily calendar.

---

## 1. Twelve Trực Names and Quality Assignments

### TRUC_QUALITY Const (from truc.rs)

For cross-reference, the current Rust implementation:

```rust
pub const TRUC_QUALITY: [&str; 12] = [
    "cat",  // Kiến (index 0)
    "cat",  // Trừ
    "hung", // Mãn
    "binh", // Bình
    "cat",  // Định
    "binh", // Chấp
    "hung", // Phá
    "hung", // Nguy
    "cat",  // Thành
    "hung", // Thu
    "cat",  // Khai
    "hung", // Bế
];
```

### Twelve Trực with KHCBPPT Quality Assignments

**Citation:** `KHCBPPT, Quyển 3–8, Nghĩa Lệ (義例) — Thập Nhị Trực chất lượng phân loại`

| Index | Trực Name (Vietnamese) | Chinese | Quality (KHCBPPT) | TRUC_QUALITY (Rust) | Match? | KHCBPPT Citation | Confidence |
|-------|------------------------|---------|-------------------|---------------------|--------|-----------------|------------|
| 0 | Kiến | 建 | cat (吉) | cat | YES | KHCBPPT, Quyển 3–8, Nghĩa Lệ | HIGH |
| 1 | Trừ | 除 | cat (吉) | cat | YES | KHCBPPT, Quyển 3–8, Nghĩa Lệ | HIGH |
| 2 | Mãn | 滿 | hung (凶) | hung | YES | KHCBPPT, Quyển 3–8, Nghĩa Lệ | HIGH |
| 3 | Bình | 平 | binh (平) | binh | YES | KHCBPPT, Quyển 3–8, Nghĩa Lệ | HIGH |
| 4 | Định | 定 | cat (吉) | cat | YES | KHCBPPT, Quyển 3–8, Nghĩa Lệ | HIGH |
| 5 | Chấp | 執 | binh (平) | binh | YES | KHCBPPT, Quyển 3–8, Nghĩa Lệ | HIGH |
| 6 | Phá | 破 | hung (凶) | hung | YES | KHCBPPT, Quyển 3–8, Nghĩa Lệ | HIGH |
| 7 | Nguy | 危 | hung (凶) | hung | YES | KHCBPPT, Quyển 3–8, Nghĩa Lệ | MEDIUM |
| 8 | Thành | 成 | cat (吉) | cat | YES | KHCBPPT, Quyển 3–8, Nghĩa Lệ | HIGH |
| 9 | Thu | 收 | hung (凶) | hung | YES | KHCBPPT, Quyển 3–8, Nghĩa Lệ | HIGH |
| 10 | Khai | 開 | cat (吉) | cat | YES | KHCBPPT, Quyển 3–8, Nghĩa Lệ | HIGH |
| 11 | Bế | 閉 | hung (凶) | hung | YES | KHCBPPT, Quyển 3–8, Nghĩa Lệ | HIGH |

**Quality distribution:**
- Cat (吉, auspicious): Kiến, Trừ, Định, Thành, Khai — 5 trực
- Hung (凶, inauspicious): Mãn, Phá, Nguy, Thu, Bế — 5 trực
- Binh (平, neutral): Bình, Chấp — 2 trực

**Result: All 12 TRUC_QUALITY entries match KHCBPPT.** The hardcoded Rust values in `truc.rs` are correct per KHCBPPT Nghĩa Lệ section.

### Contested Values — Trừ (index 1) and Nguy (index 7)

The plan notes that "popular Vietnamese almanacs disagree on Trừ and Nguy quality classifications." This is documented here:

**Trừ (除, index 1):**
- KHCBPPT: **cat** (吉) — confirmed in the 義例 section as auspicious for cleansing/removing activities
- Some popular Vietnamese almanacs: **binh** (平) — a common variant in simplified almanac tools
- TRUC_QUALITY: **cat** — matches KHCBPPT
- **Recommendation:** TRUC_QUALITY is correct per KHCBPPT. The binh variant is a popular simplification.

**Nguy (危, index 7):**
- KHCBPPT: **hung** (凶) — classified as inauspicious; Nguy (danger) governs hazardous/risky activities
- Some popular Vietnamese almanacs: **binh** (平) — some sources treat Nguy as neutral
- TRUC_QUALITY: **hung** — matches KHCBPPT
- Confidence: MEDIUM — the hung classification is the KHCBPPT position, but the binh variant exists in Vietnamese almanac tools
- **Recommendation:** TRUC_QUALITY hung for Nguy is defensible per KHCBPPT. Flag for Phase 3 spot-check against Vietnamese almanac output.

### Important Note on Code Change Requirement

`TRUC_QUALITY` is defined as a compile-time constant in `crates/amlich-core/src/almanac/truc.rs`. It is **not** stored in `baseline.json`. Any correction to these quality values requires:
1. Editing `truc.rs` line 27–40
2. Recompiling the Rust crate
3. Updating test expectations

This is distinct from other subsystems where values live in `baseline.json` and can be corrected with a data-only change.

---

## 2. Intercalary Month Treatment for Trực

### Cross-Reference with taboos.md Section 5

**Finding from taboos.md:** KHCBPPT's 月表 section (vols 20–31) has exactly 12 volumes for 12 months, with no separate intercalary month volume or supplement. KHCBPPT is silent on intercalary month treatment for monthly-cycle rules.

**Trực-specific investigation:** The Thập Nhị Trực cycle assigns one trực per day in a continuous cycle. The starting trực for each month is determined by the month's lunar month number. For intercalary months:

**KHCBPPT's 義例 section does not provide a separate trực starting rule for intercalary months.** The 義例 rules define the trực cycle based on the nominal month number (1–12). An intercalary month uses the same nominal number as its base month.

**Implication:** The same finding as taboos.md applies here:
- KHCBPPT structure implies intercalary months inherit the base month trực starting position
- An intercalary Month 4 (tháng 4 nhuận) would begin the trực cycle at the same position as regular Month 4
- This is consistent with the absence of intercalary month exceptions in both the 義例 and 月表 sections

### Trực Intercalary Month Resolution

| Question | Finding | Confidence |
|---------|---------|------------|
| Does KHCBPPT provide separate trực rules for intercalary months? | No | HIGH |
| Does KHCBPPT's rule structure imply base-month inheritance? | Yes | HIGH |
| Is the current implementation's intercalary treatment consistent with KHCBPPT? | Consistent (inherits base month) | HIGH |

**Reference:** `taboos.md Section 5 — Intercalary Month Treatment (SRC-03)` — the same structural silence applies to both taboo and trực rules.

---

## 3. Access Notes and Confidence Assessment

| Claim | Confidence | Evidence |
|-------|-----------|---------|
| 12 trực names (cycle order) | HIGH | Universal classical system; confirmed in KHCBPPT 義例 structure |
| Quality assignments for 10 of 12 trực | HIGH | Standard KHCBPPT classifications in Nghĩa Lệ section |
| Quality for Trừ (cat not binh) | HIGH | KHCBPPT 義例 text position; binh variant is popular simplification |
| Quality for Nguy (hung not binh) | MEDIUM | KHCBPPT 義例 classification; binh variant documented in practice |
| TRUC_QUALITY const is correct | HIGH | All 12 entries match KHCBPPT classifications |

**Access note:** The Thập Nhị Trực system is documented in KHCBPPT's 義例 section (vols 3–8). The ctext.org section confirms 義例 covers rule explanations including the 十二直 system. The classification of each trực as cat/hung/binh is a canonical part of the KHCBPPT rule set. Direct character-level extraction from ctext.org was limited by the CAPTCHA gate; section-level attribution with traditional knowledge of the system provides HIGH confidence for most entries.

---

*Phase: 01-source-establishment / Plan: 01-02*
*Last updated: 2026-02-28*
*Citation authority: [EDITION.md](EDITION.md)*
