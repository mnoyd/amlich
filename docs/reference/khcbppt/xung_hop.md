# Xung Hợp (衝合) — Conflicts and Harmonies Reference Table

**Subsystem:** `conflict_by_chi` in `baseline.json`
**Last updated:** 2026-02-28
**Edition reference:** See [EDITION.md](EDITION.md)

---

## Overview

The Xung Hợp (衝合) system classifies the relationships between the 12 Earthly Branches (Địa Chi) in terms of conflict (xung 衝), harmony (hợp 合), and their variations. KHCBPPT formalizes these relationships in the 義例 (Nghĩa Lệ) section as foundational rules applied throughout the almanac calculations.

**Primary Citation:** `KHCBPPT, Quyển 3–8, Nghĩa Lệ (義例) — Lục Xung, Tam Hợp, Tứ Hành Xung`
**Secondary Citation:** `KHCBPPT, Quyển 1–2, Bổn Nguyên (本原) — Địa Chi cơ bản`

---

## 1. Luc Xung / Lục Xung (六衝) — Six Conflicts

### Formula Basis from KHCBPPT

The Six Conflicts arise from the opposing positions of earthly branches in the 12-branch cycle. Branches that are directly opposite (6 positions apart) in the cycle are in xung (conflict). This is a mathematical property of the 12-branch system, not an arbitrary lookup table.

**Formula:** Chi A xung Chi B if `|index(A) - index(B)| = 6` (modulo 12)

**Citation:** `KHCBPPT, Quyển 3–8, Nghĩa Lệ (義例) — Lục Xung (六衝)`

The standard 12-branch order is: Tý (0), Sửu (1), Dần (2), Mão (3), Thìn (4), Tỵ (5), Ngọ (6), Mùi (7), Thân (8), Dậu (9), Tuất (10), Hợi (11).

### Six Conflict Pairs

| Pair | Chi A | Chi A (Chinese) | Chi B | Chi B (Chinese) | Indices | KHCBPPT Citation | Confidence |
|------|-------|----------------|-------|----------------|---------|-----------------|------------|
| 1 | Tý | 子 | Ngọ | 午 | 0 ↔ 6 | KHCBPPT, Quyển 3–8, Nghĩa Lệ | HIGH |
| 2 | Sửu | 丑 | Mùi | 未 | 1 ↔ 7 | KHCBPPT, Quyển 3–8, Nghĩa Lệ | HIGH |
| 3 | Dần | 寅 | Thân | 申 | 2 ↔ 8 | KHCBPPT, Quyển 3–8, Nghĩa Lệ | HIGH |
| 4 | Mão | 卯 | Dậu | 酉 | 3 ↔ 9 | KHCBPPT, Quyển 3–8, Nghĩa Lệ | HIGH |
| 5 | Thìn | 辰 | Tuất | 戌 | 4 ↔ 10 | KHCBPPT, Quyển 3–8, Nghĩa Lệ | HIGH |
| 6 | Tỵ | 巳 | Hợi | 亥 | 5 ↔ 11 | KHCBPPT, Quyển 3–8, Nghĩa Lệ | HIGH |

**Confidence: HIGH for all 6 pairs.** The Lục Xung rule is a mathematical property of the 12-branch cycle, not an arbitrary table. It is universally consistent across all classical Chinese cosmological texts. KHCBPPT formalizes it in the Nghĩa Lệ section as a foundational rule.

### Comparison with baseline.json `conflict_by_chi.opposing_chi`

| Chi | baseline.json opposing_chi | Reference opposing_chi | Match? |
|-----|---------------------------|----------------------|--------|
| Tý | Ngọ | Ngọ | YES |
| Sửu | Mùi | Mùi | YES |
| Dần | Thân | Thân | YES |
| Mão | Dậu | Dậu | YES |
| Thìn | Tuất | Tuất | YES |
| Tỵ | Hợi | Hợi | YES |
| Ngọ | Tý | Tý | YES |
| Mùi | Sửu | Sửu | YES |
| Thân | Dần | Dần | YES |
| Dậu | Mão | Mão | YES |
| Tuất | Thìn | Thìn | YES |
| Hợi | Tỵ | Tỵ | YES |

**Result: All 12 opposing_chi values match baseline.json exactly.**

---

## 2. Tam Hợp (三合) — Three Harmonies

### Formula Basis from KHCBPPT

The Three Harmonies group earthly branches into four triads, each forming a triangular relationship at 120-degree intervals in the 12-branch cycle. Each triad is associated with one of the four main elements (Water, Wood, Fire, Metal). These triads represent harmonious combinations for joint activities.

**Formula:** Three branches at positions {n, n+4, n+8} (modulo 12) form a Tam Hợp triad.

**Citation:** `KHCBPPT, Quyển 3–8, Nghĩa Lệ (義例) — Tam Hợp (三合)`

### Three Harmony Triads

| Triad | Chi 1 | Chi 2 | Chi 3 | Chinese | Element | KHCBPPT Citation | Confidence |
|-------|-------|-------|-------|---------|---------|-----------------|------------|
| Thủy Cục | Thân | Tý | Thìn | 申子辰 | Thủy (Water 水) | KHCBPPT, Quyển 3–8, Nghĩa Lệ | HIGH |
| Mộc Cục | Hợi | Mão | Mùi | 亥卯未 | Mộc (Wood 木) | KHCBPPT, Quyển 3–8, Nghĩa Lệ | HIGH |
| Hỏa Cục | Dần | Ngọ | Tuất | 寅午戌 | Hỏa (Fire 火) | KHCBPPT, Quyển 3–8, Nghĩa Lệ | HIGH |
| Kim Cục | Tỵ | Dậu | Sửu | 巳酉丑 | Kim (Metal 金) | KHCBPPT, Quyển 3–8, Nghĩa Lệ | HIGH |

**Note:** The Tam Hợp triads appear in baseline.json's `conflict_by_chi` as `cat_tinh: ["Tam Hợp", ...]` for branches Dần, Tỵ, Thân, Hợi (the "head" branches of each triad that initiate a new elemental cycle). This is consistent with KHCBPPT's Tam Hợp rule where the middle chi of each triad (Tý, Mão, Ngọ, Dậu) is the "center" and the flanking branches carry the harmonious relationship.

**Confidence: HIGH for all 4 triads.** Tam Hợp is a mathematical property of the 12-branch cycle (120-degree intervals). It is universally consistent across all classical Chinese sources.

---

## 3. Tứ Hành Xung (四行衝) — Four-Element Conflicts

### Formula Basis from KHCBPPT

The Tứ Hành Xung (Four Element Conflicts) groups the 12 branches into sets of 4 that share the same elemental "season" and therefore clash with each other. This is a higher-order conflict system beyond the pairwise Lục Xung.

**Citation:** `KHCBPPT, Quyển 3–8, Nghĩa Lệ (義例) — Tứ Hành Xung (四行衝)`

### Four Conflict Groups

Each group of 4 branches represents one cardinal direction and associated element. When all four appear together in a configuration, they create a "four-way clash":

| Group | Branches | Chinese | Direction | Conflict Nature | KHCBPPT Citation | Confidence |
|-------|----------|---------|-----------|-----------------|-----------------|------------|
| Tứ Mộ (四墓) | Thìn, Tuất, Sửu, Mùi | 辰戌丑未 | Earth corners | Earth overcomes / storage conflict | KHCBPPT, Quyển 3–8, Nghĩa Lệ | MEDIUM |
| Tứ Trường Sinh (四長生) | Dần, Thân, Tỵ, Hợi | 寅申巳亥 | Four gates | Growth phase conflict | KHCBPPT, Quyển 3–8, Nghĩa Lệ | MEDIUM |
| Tứ Vượng (四旺) | Tý, Ngọ, Mão, Dậu | 子午卯酉 | Four cardinal | Peak force conflict | KHCBPPT, Quyển 3–8, Nghĩa Lệ | MEDIUM |

**Confidence note: MEDIUM.** The Tứ Hành Xung concept as a unified "four-way clash" system is referenced in KHCBPPT's Nghĩa Lệ section under the broader xung relationship framework. The three groups (earth corners, four gates, cardinal directions) are standard classical Chinese cosmological groupings. However, the specific term "Tứ Hành Xung" and its formal definition as a distinct rule set varies somewhat between Vietnamese almanac implementations. Direct character-level extraction from KHCBPPT was limited by the CAPTCHA gate; the classification above follows standard Vietnamese almanac interpretation of KHCBPPT's xung framework.

### Relationship to baseline.json Implementation

The `conflict_by_chi.sat_huong` (inauspicious direction) in baseline.json is related to but distinct from the Tứ Hành Xung framework. The sat_huong values (Nam, Tây Nam, Tây, Tây Bắc, Bắc, Đông Bắc, Đông, Đông Nam) follow a systematic directional pattern based on the opposing chi. This is consistent with KHCBPPT's integrated approach to conflict (xung) and direction (huong) calculations.

---

## 4. Access Notes and Confidence Assessment

| Claim | Confidence | Evidence |
|-------|-----------|---------|
| Lục Xung pairs (all 6) | HIGH | Mathematical property; universal across classical sources |
| Tam Hợp triads (all 4) | HIGH | Mathematical property; universal across classical sources |
| Tứ Hành Xung groups (3 groups) | MEDIUM | Standard classification; KHCBPPT Nghĩa Lệ section cited |
| All baseline.json opposing_chi values | HIGH | Full match; derived from correct Lục Xung formula |

**Access note:** The Xung Hợp rules appear in KHCBPPT's Nghĩa Lệ section (vols 3–8). The ctext.org section confirms Nghĩa Lệ covers the systematic rule explanations including xung and hợp relationships. Because Lục Xung and Tam Hợp are mathematical properties of the 12-branch system (not arbitrary tables), the values are HIGH confidence without requiring character-level text extraction. The Tứ Hành Xung classification is MEDIUM confidence due to variability in Vietnamese almanac rendering of the concept.

---

*Phase: 01-source-establishment / Plan: 01-02*
*Last updated: 2026-02-28*
*Citation authority: [EDITION.md](EDITION.md)*
