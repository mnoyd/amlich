# KHCBPPT Edition Record

**Full title:** 欽定協紀辨方書 (Khâm Định Hiệp Kỷ Biện Phương Thư)
**Abbreviation used in this project:** KHCBPPT
**Last updated:** 2026-02-28

---

## 1. Primary Edition — ctext.org 四庫全書 Digitization

**Edition:** 四庫全書 (Siku Quanshu) text
**Origin:** Imperially commissioned Qing-dynasty compilation, Qianlong era (1741)
**Digitization:** Chinese Text Project (ctext.org)
**URL:** https://ctext.org/wiki.pl?if=gb&res=455082
**Format:** Full-text Chinese, searchable and browsable by volume
**Access:** Free online; chapter-level browsing available without CAPTCHA; full-text copy may require registration

**Bibliographic status:** This is the closest available text to the authoritative Qing imperial source. No translator interpretation layer. Used as the primary source for Chinese-character verification in all subsystem reference files.

**Note on access:** ctext.org confirmed accessible during Phase 1 research (2026-02-28). The 36-volume structure is navigable via the work's table of contents. The CAPTCHA gate applies to bulk export, not to chapter-level browsing.

---

## 2. Secondary Edition — 1998 NXB Mũi Cà Mau Vietnamese Translation

**Edition:** Vietnamese translation, 2-volume set
**Publisher:** NXB Mũi Cà Mau (Nhà Xuất Bản Mũi Cà Mau), 1998
**Editor / Compiler:** Mai Cốc Thành (麥穀成)
**Translators:** Vũ Hoàng and Lân Bình
**Based on:** 1995 Shanghai Ancient Texts Publishing House (上海古籍出版社) edition and 1994 Guangxi People's Publishing House (廣西人民出版社) edition
**Volume 1:** ~979 pages
**Volume 2:** ~1001 pages
**archive.org URLs:**
- Volume 1: https://archive.org/details/hiepkybienphuongthu_1998_t1
- Volume 2: https://archive.org/details/hiepkybienphuongthu_1998_t2 (or hiepkybienphuongthu_1998_t2 variant)

**Bibliographic status:** This is the most widely referenced Vietnamese-language edition of KHCBPPT in the Vietnamese almanac ecosystem. It introduces a translator interpretation layer (Vietnamese terminology, romanized equivalents). Used as secondary reference for Vietnamese terminology cross-checking and to confirm which edition likely informed the Vietnamese almanac tools from which the amlich codebase may draw.

---

## 3. Edition Selection Rationale

**Primary for this audit:** ctext.org 四庫全書 text

**Rationale:** The 四庫全書 digitization is the authoritative Qing-dynasty source text with no translation layer. All Chinese-character values in the subsystem reference files are drawn from or verified against this edition. When a table value is extracted, the ctext.org URL or volume/chapter reference is cited as the source.

**Secondary for this audit:** 1998 NXB Mũi Cà Mau Vietnamese translation

**Rationale:** The 1998 Mai Cốc Thành edition is the most likely edition that informed the Vietnamese almanac ecosystem from which amlich's baseline data descends. It is used for Vietnamese terminology cross-reference and to understand how classical Chinese concepts were rendered in the Vietnamese tradition. Values extracted exclusively from the 1998 edition are noted with confidence MEDIUM (translation layer introduces interpretation risk).

**Not used as primary:** The 1995 Shanghai or 1994 Guangxi editions. These are the base texts for the 1998 Vietnamese translation but are not independently accessible for this audit.

---

## 4. Citation format

All subsystem reference files in `docs/reference/khcbppt/` use the following citation format:

### Format A — Volume + Section (preferred, for locating tables)

```
KHCBPPT, Quyển [N], [Section name in Vietnamese]
```

Example: `KHCBPPT, Quyển 1, Bổn Nguyên (本原)`

### Format B — Chinese volume reference (for ctext.org cross-reference)

```
KHCBPPT, 卷[N], [Chinese section name]
```

Example: `KHCBPPT, 卷1, 本原`

### Format C — 1998 Vietnamese edition (for secondary cross-reference)

```
KHCBPPT (1998 Mai Cốc Thành), Tập [1|2], tr. [page range]
```

Example: `KHCBPPT (1998 Mai Cốc Thành), Tập 1, tr. 45–48`

### Granularity rule

Citations must be at **chapter + section level** (sufficient to locate data in the text). Page-level precision is desirable but not required. Volume-only citations are insufficient. A citation such as "KHCBPPT, Quyển 32, Nhật Biểu" is acceptable; "KHCBPPT" alone is not.

---

## 5. Volume Structure (36 Volumes, 11 Major Sections)

| Section | Vietnamese | Chinese | Volumes | Content |
|---------|-----------|---------|---------|---------|
| 本原 | Bổn Nguyên | 本原 | 1–2 | Origin Principles — cosmological foundations |
| 義例 | Nghĩa Lệ | 義例 | 3–8 | Principles & Examples — rule explanations |
| 立成 | Lập Thành | 立成 | 9 | Ready-Made Tables — precomputed lookup tables |
| 宜忌 | Nghi Kỵ | 宜忌 | 10 | Auspicious & Inauspicious — activity classifications |
| 用事 | Dụng Sự | 用事 | 11 | Practical Applications — usage contexts |
| 公規 | Công Quy | 公規 | 12–13 | Official Regulations — formal rule sets |
| 年表 | Niên Biểu | 年表 | 14–19 | Year Tables — annual calendrical data |
| 月表 | Nguyệt Biểu | 月表 | 20–31 | Month Tables — monthly taboo and rule data |
| 日表 | Nhật Biểu | 日表 | 32 | Day Tables — daily star and almanac data |
| 利用 | Lợi Dụng | 利用 | 33–34 | Practical Use — applied guidance |
| 附錄/辨訛 | Phụ Lục / Biện Ngoa | 附錄/辨訛 | 35–36 | Appendices & Error Corrections |

**Subsystem relevance:**
- Nạp âm (納音): Found in 本原 (vols 1–2) — fundamental principles section
- Nhị Thập Bát Tú (二十八宿): Referenced in 公規 (vols 12–13) per RESEARCH.md note on vol 13
- Taboo rules (tam nương, nguyệt kỵ, sát chủ, thọ tử): Likely in 月表 (vols 20–31) and 宜忌 (vol 10)
- Day deity cycle (thần sát): Likely in 日表 (vol 32)
- Thần hướng (than_huong): Likely in 立成 (vol 9) or 利用 (vols 33–34)

---

## 6. Baseline.json Data Origin

**Status: Honestly undocumented — partially inferred from git history**

The `baseline.json` file in `crates/amlich-core/data/almanac/` uses `source_id: "khcbppt"` for most subsystems. The actual transcription chain — whether values were copied directly from KHCBPPT text, derived from a Vietnamese almanac application, or sourced from reference code — is **not documented** in the codebase.

**What is documented:**
- Commit `0f29f3f` (2026-02-24): corrected 8 data errors "verified against classical sources":
  - Nạp âm #20: `Kim Bạch Kim` → `Kim Bạc Kim` (金箔金)
  - Nạp âm #23: `Đại Trạch Thổ` → `Đại Dịch Thổ` (大驿土)
  - Thần hướng (tài thần): Ất → Tây Nam, Bính/Đinh → Tây (甲艮乙坤丙丁兑)
  - Thần hướng (hỷ thần): Kỷ → Đông Bắc, Tân → Tây Nam, Quý → Đông Nam
  - The commit message cites "classical sources" without naming a specific text
- The two non-KHCBPPT source_ids in baseline.json:
  - `na_am_meta.source_id: "tam-menh-thong-hoi"` — nạp âm sourced from Tam Mệnh Thông Hội (三命通會)
  - `star_meta.source_id: "nhi-thap-bat-tu"` — 28-star cycle attributed to the tradition name, not a specific text

**What is not documented:**
- The initial transcription of `baseline.json` data has no commit-level source citation
- Whether the `source_id: "khcbppt"` values were verified against any specific edition of KHCBPPT or inferred from a Vietnamese almanac application or reference implementation
- The JD epoch origin for `jd.rem_euclid(28)` in `calc.rs:46`

**Implication for Phase 3:** Confidence in `source_id: "khcbppt"` values is MEDIUM until Phase 1 reference files provide explicit chapter-level citations. Phase 3 validators must treat currently uncited values as "sourced from KHCBPPT (unverified edition)" rather than "verified against identified edition."

---

## 7. Prior Corrections Log

| Commit | Date | Subsystem | Change | Source cited |
|--------|------|-----------|--------|-------------|
| 0f29f3f | 2026-02-24 | Nạp âm #20 | Kim Bạch Kim → Kim Bạc Kim (金箔金) | "classical sources" (unspecified) |
| 0f29f3f | 2026-02-24 | Nạp âm #23 | Đại Trạch Thổ → Đại Dịch Thổ (大驿土) | "classical sources" (unspecified) |
| 0f29f3f | 2026-02-24 | Thần hướng (tài thần) | 3 stem corrections | "classical sources" (unspecified) |
| 0f29f3f | 2026-02-24 | Thần hướng (hỷ thần) | 3 stem corrections | "classical sources" (unspecified) |

Phase 1 re-verification against the identified editions above will establish whether these corrections align with KHCBPPT specifically.

---

## 8. Files That Reference This Document

All subsystem reference files in `docs/reference/khcbppt/` reference EDITION.md as their citation authority:

- `na_am.md` — nạp âm 30-pair table and SRC-02 decision
- `stars.md` — nhị thập bát tú star tables (to be created in Plan 02)
- `taboos.md` — tam nương, nguyệt kỵ, sát chủ, thọ tử tables (Plan 02)
- `day_deity.md` — 12-deity cycle (Plan 02)
- `truc.md` — 12 trực quality assignments (Plan 02)
- `xung_hop.md` — lục xung, tam hợp formulas (Plan 02)
- `than_huong.md` — thần hướng 30 values (Plan 02)
