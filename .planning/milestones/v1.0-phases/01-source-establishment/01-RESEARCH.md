# Phase 1: Source Establishment - Research

**Researched:** 2026-02-28
**Domain:** Classical Vietnamese almanac text (KHCBPPT) — edition identification, raw table extraction, source attribution
**Confidence:** MEDIUM (text structure), HIGH (codebase integration points)

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

- A specific KHCBPPT edition (publisher, year, translator) must be identified and recorded — different editions may have different table values
- Citation granularity: chapter + section level (e.g., "Quyển 4, Thiên Văn") — enough to locate data in the text without requiring page-level precision
- The current baseline.json data origin (direct transcription vs secondary sources) must be documented honestly — this affects confidence levels in Phase 3 validators
- Nạp âm currently cites "tam-menh-thong-hoi" not "khcbppt" — Phase 1 must first check whether KHCBPPT contains nạp âm tables; if yes: re-source from KHCBPPT and update source_id; if no: document the gap and validate against Tam Mệnh Thông Hội as a separate source
- Golden dataset should support per-subsystem source attribution — pragmatic and honest about mixed sources
- Reference table format: structured markdown tables, one file per subsystem (stars.md, taboos.md, day_deity.md, truc.md, xung_hop.md, than_huong.md, na_am.md)
- Include original Vietnamese text alongside extracted values where available — provides audit trail
- Location: `docs/reference/khcbppt/` — permanent project documentation, not ephemeral planning artifacts
- Stars cite "nhi-thap-bat-tu" with JD-cycle method — need to verify whether the 28-star system appears within KHCBPPT or is a separate tradition
- JD epoch origin must be traced (likely from Ho Ngoc Duc's implementation or similar reference code)
- Extract complete chi→star mappings for all fixed_by_chi stars (all 12 chi values)
- Include star quality assignments (cát/hung/bình) in Phase 1 extraction
- Star rule sparsity (1 entry per contextual bucket) flagged for investigation — determine if rules are missing or intentionally minimal
- KHCBPPT's treatment of intercalary months for taboo and trực rules must be documented from the text, not inferred from the implementation
- This is a known gap — the current code may handle intercalary months differently than KHCBPPT prescribes
- Exact file naming within `docs/reference/khcbppt/` — Claude's discretion
- Internal organization of each subsystem reference file (table headers, section ordering) — Claude's discretion
- Whether to create an index/overview file linking all subsystem references — Claude's discretion

### Claude's Discretion

- Exact file naming within `docs/reference/khcbppt/`
- Internal organization of each subsystem reference file (table headers, section ordering)
- Whether to create an index/overview file linking all subsystem references

### Deferred Ideas (OUT OF SCOPE)

- Full star rule completeness audit (all 60 sexagenary pairs, all 10 stems, etc.) — v2 scope (STR-V2-01 through STR-V2-05)
- Sát Hướng directional verification — v2 scope (EXT-V2-01)
- Extended date range beyond 2030 — v2 scope (EXT-V2-02)
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| SRC-01 | KHCBPPT edition identified and documented in golden dataset metadata | Edition identification workflow, known available editions and sources documented below |
| SRC-02 | Nạp Âm scope determined (KHCBPPT or "tam-menh-thong-hoi" — in or out of audit) | KHCBPPT does cover 納音 in its 本原 section; investigation pathway described below; na_am_meta.source_id in baseline.json is currently "tam-menh-thong-hoi" |
| SRC-03 | Intercalary month handling researched and documented from KHCBPPT text | KHCBPPT月表 volumes (vols 9–20) provide monthly taboo data; intercalary month treatment must be read from those volumes; no automated shortcut exists |
</phase_requirements>

## Summary

Phase 1 is pure research and documentation — no code changes. The deliverable is a set of permanent reference files at `docs/reference/khcbppt/` that pin the KHCBPPT edition, extract raw reference tables for every almanac subsystem, and resolve the two open sourcing questions (nạp âm scope, nhị thập bát tú source). All downstream phases depend on this foundation being stable; if the edition is not pinned or a table is ambiguously sourced, every Phase 3 validator will have uncertain expected values.

The single most actionable finding is that a concrete edition of KHCBPPT is accessible: the 1998 NXB Mũi Cà Mau two-volume Vietnamese translation by Mai Cốc Thành (translators: Vũ Hoàng and Lân Bình), based on a 1995 Shanghai Ancient Texts Publishing House edition and a 1994 Guangxi People's Publishing House edition. This is the most likely edition that informed the Vietnamese almanac ecosystem that the amlich codebase draws from. An alternative (or complementary) access path is the full-text digitization of the 四庫全書 edition of 欽定協紀辨方書 available on the Chinese Text Project (ctext.org), which provides the authoritative Qing-dynasty source text.

The nạp âm question requires direct text inspection: KHCBPPT's 本原 (Origin Principles) section explicitly covers 納音 (nạp âm) according to the Chinese Text Project entry for the work. Whether the 30-pair table in KHCBPPT matches the values currently in baseline.json (sourced from "tam-menh-thong-hoi") must be determined by reading both sources and comparing. The JD epoch for nhị thập bát tú is not documented anywhere in the codebase — it originates from `jd.rem_euclid(28)` in `calc.rs:46` without citation, and the epoch must be traced to its historical or implementation origin.

**Primary recommendation:** Open the 1998 Mai Cốc Thành edition or the ctext.org digitization of KHCBPPT, systematically locate each subsystem's tables, extract values into the seven markdown reference files, then write `EDITION.md` as the pinning document that all other files reference.

## Standard Stack

This phase introduces no software dependencies. The deliverables are documentation files.

### Core

| Tool | Purpose | Why Standard |
|------|---------|--------------|
| Markdown tables | Reference data format in `docs/reference/khcbppt/` | Decided in CONTEXT.md; matches existing `docs/almanac/*.md` pattern in this project |
| JSON (baseline.json) | Target format for `_meta` source attribution updates | Already the project data format; `source_id` field is the update point |
| ctext.org | Access to full digitized 欽定協紀辨方書 (四庫全書本) | HIGH-quality primary source; Qing-dynasty authoritative text; searchable full text |

### Supporting

| Tool | Purpose | When to Use |
|------|---------|-------------|
| archive.org (Internet Archive) | Access to 1998 Mai Cốc Thành Vietnamese edition PDF | When Vietnamese-language rendering is needed for cross-checking; also confirms edition details |
| Git history review | Document what corrections were already made and why | Commit 0f29f3f fixed thần hướng, nạp âm names — establishes a prior correction baseline |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| ctext.org (四庫全書 edition) | Archive.org 1998 NXB Vietnamese translation | Vietnamese translation is more accessible but introduces translator interpretation layer; ctext.org is the closest to Qing-dynasty source text |
| Manual extraction to Markdown | Automated OCR/extraction | Automation is too error-prone on classical Chinese/Vietnamese text; manual is mandatory |

## Architecture Patterns

### Recommended Output Structure

```
docs/reference/khcbppt/
├── EDITION.md          # Edition pinning document (standalone, referenced by all other files)
├── stars.md            # fixed_by_chi (12 chi) + 28-star quality table + star source attribution
├── taboos.md           # Tam Nương days, Nguyệt Kỵ days, Sát Chủ map, Thọ Tử map
├── day_deity.md        # 12-deity cycle order + 12 month-start offsets
├── truc.md             # 12 trực quality assignments (cát/hung/bình)
├── xung_hop.md         # Lục Xung, Tam Hợp, Tứ Hành Xung formula basis
├── than_huong.md       # 10 stems × 3 directions (30 values)
└── na_am.md            # 30 nạp âm pairs (source: KHCBPPT or Tam Mệnh Thông Hội per SRC-02 decision)
```

This mirrors the subsystem breakdown in `crates/amlich-core/src/almanac/` and baseline.json's top-level keys, making Phase 2 transformation straightforward.

### Pattern 1: Reference Table Format

**What:** Each subsystem file uses a structured markdown table with columns: Vietnamese name, Chinese character (where applicable), value, source_id, confidence, and KHCBPPT citation.

**When to use:** All seven subsystem files.

**Example:**
```markdown
## Tam Nương (三娘)

| Lunar Day | Source | KHCBPPT Ref | Confidence |
|-----------|--------|-------------|------------|
| 3         | khcbppt | Quyển X, ... | HIGH |
| 7         | khcbppt | Quyển X, ... | HIGH |
| 13        | khcbppt | Quyển X, ... | HIGH |
| 18        | khcbppt | Quyển X, ... | HIGH |
| 22        | khcbppt | Quyển X, ... | HIGH |
| 27        | khcbppt | Quyển X, ... | HIGH |
```

### Pattern 2: EDITION.md Pinning Document

**What:** A standalone file that records all edition metadata — used as the authoritative citation for all other reference files.

**When to use:** Created first; referenced by all other files in the directory.

**Example:**
```markdown
# KHCBPPT Edition Record

**Full title:** 欽定協紀辨方書 (Khâm Định Hiệp Kỷ Biện Phương Thư)
**Edition used:** [one of the editions below]
**Access method:** [ctext.org URL / archive.org URL / physical copy]
**Citation format:** [how to cite chapters in other reference files]
```

### Pattern 3: Mixed-Source Attribution

**What:** When a subsystem is validated against a non-KHCBPPT source (e.g., nạp âm from Tam Mệnh Thông Hội), the reference file explicitly states the source and confidence.

**When to use:** na_am.md (pending SRC-02 decision) and stars.md (28-star system attribution).

**Example:**
```markdown
## Source Attribution

- **KHCBPPT coverage:** Not confirmed (pending investigation)
- **Validated source:** Tam Mệnh Thông Hội (三命通會)
- **baseline.json source_id:** "tam-menh-thong-hoi" (current) / "khcbppt" (if confirmed in KHCBPPT)
```

### Anti-Patterns to Avoid

- **Inferring from implementation:** Do not reverse-engineer table values from the current code output. CONTEXT.md is explicit: document from the text, not the implementation.
- **Conflating editions:** The 1998 Vietnamese translation may differ from the ctext.org 四庫全書 text in subtle ways. Document which edition each value came from.
- **Treating one corrected subsystem as validated:** Commit 0f29f3f corrected thần hướng and nạp âm values — but that correction itself was "verified against classical sources" without specifying which source. Phase 1 must re-verify these values against KHCBPPT specifically and document the citation.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Accessing KHCBPPT text | Custom scraper or OCR pipeline | ctext.org browse + copy, or archive.org PDF | Full text is already digitized and accessible; OCR error rates on classical Chinese/Vietnamese are too high |
| Cross-referencing nạp âm source | Writing a diff script | Manual visual comparison of 30 pairs | Only 30 pairs to check; automation risk exceeds benefit |
| Star JD epoch verification | Custom astronomical calculation | Look up 3 known dated entries in KHCBPPT's 日表 (day tables) and compare with jd.rem_euclid(28) | The epoch question is definitional, not computational |

**Key insight:** This phase is irreducibly manual. The risk is not technical complexity but source ambiguity — and the prevention is careful documentation, not automation.

## Common Pitfalls

### Pitfall 1: Wrong Edition Selected

**What goes wrong:** Different KHCBPPT editions (1995 Shanghai, 1994 Guangxi, 1998 Vietnamese translation, ctext.org 四庫全書 digitization) may have different table values. If the edition is not pinned explicitly, all subsequent cross-referencing is on an unstable foundation.

**Why it happens:** Multiple editions are easily accessible; the easiest to access may not be the most authoritative.

**How to avoid:** Write EDITION.md before any other reference file. Record publisher, year, translator, and access method. Note that the 四庫全書 text on ctext.org is the primary Qing-dynasty source; the 1998 NXB Mũi Cà Mau translation is the most common Vietnamese-language access point. The planner should designate one edition as primary and document any secondary editions used for cross-checking.

**Warning signs:** Reference files cite "KHCBPPT" without specifying which edition; two files cite different editions for the same subsystem.

### Pitfall 2: Nạp Âm Decision Blocked on Edition Access

**What goes wrong:** If neither ctext.org nor archive.org is accessible for the specific volume containing 納音 tables, SRC-02 cannot be resolved and the golden dataset schema is blocked.

**Why it happens:** ctext.org requires CAPTCHA for direct chapter text access; the 1998 Vietnamese edition PDF may have quality issues.

**How to avoid:** Pre-identify fallback access paths: (1) ctext.org full-text search for "納音" in the 欽定協紀辨方書 index; (2) the archive.org scan of 欽定協紀辨方書·卷一 (06056502.cn); (3) the 1998 Tập 1 PDF at archive.org/details/hiepkybienphuongthu_1998_t1.

**Warning signs:** SRC-02 remains unresolved after inspecting the 本原 (Origin Principles) section of KHCBPPT — at that point, treat "tam-menh-thong-hoi" as the validated source for nạp âm and document the gap explicitly.

### Pitfall 3: 28-Star JD Epoch Left Unresolved

**What goes wrong:** `jd.rem_euclid(28)` in `calc.rs:46` has no citation. If the offset origin is not traced, the Phase 3 star validator cannot detect whether the epoch is correct.

**Why it happens:** JD epoch for 28-star cycles is an implementation-specific constant that originates from a reference code base (likely Ho Ngoc Duc's lunar calendar implementation) — not from KHCBPPT text directly.

**How to avoid:** Stars.md should explicitly document: (1) whether KHCBPPT defines the 28-star cycle as a JD-mod system or a different positional system; (2) what JD value maps to star index 0 (Giác) according to any calculable real-world dated entry; (3) the origin of the current `rem_euclid(28)` approach (Ho Ngoc Duc or other).

**Warning signs:** stars.md records star names and qualities but does not include a section on "28-star epoch source and verification method."

### Pitfall 4: Intercalary Month Treatment Left as "Same as Base Month"

**What goes wrong:** The current code has no intercalary month handling in `month_chi_index()` — it maps month 1–12 with no intercalary variant. KHCBPPT's 月表 volumes (vols 9–20) provide monthly tables; whether those volumes include explicit intercalary month rows or are silent on the matter must be determined from the text, not assumed.

**Why it happens:** Intercalary months occur rarely (~once every 3 years), so the gap is not visible in everyday testing.

**How to avoid:** Taboos.md should include a dedicated "Intercalary Month Treatment" section documenting exactly what KHCBPPT says (even if that is "not mentioned" or "same rules as base month"). This satisfies SRC-03 as a documented finding, not an assumption.

**Warning signs:** Taboos.md and truc.md have no intercalary month section; SRC-03 is marked complete without a documented source citation.

### Pitfall 5: Undocumented Corrections in baseline.json

**What goes wrong:** Commit 0f29f3f made prior corrections to thần hướng (Tài thần, Hỷ thần values for 6 stems) and nạp âm pair names (#20 Kim Bạc Kim, #23 Đại Dịch Thổ) "verified against classical sources." If Phase 1 doesn't document what the verified source was, Phase 3 validators won't know if these values are KHCBPPT-verified or verified against a different source.

**Why it happens:** The commit message references "classical sources" without specifying KHCBPPT. It may have been verified against Tam Mệnh Thông Hội, a Chinese reference app, or another text.

**How to avoid:** Each reference file should explicitly note if the current baseline.json value matches the Phase 1 extracted value, and whether the corrected value came from KHCBPPT or another source. This enables Phase 3 to assign correct confidence levels.

**Warning signs:** than_huong.md shows values that match baseline.json exactly but cites no chapter/section in KHCBPPT for those values.

### Pitfall 6: Star Rule Sparsity Misread as Correct

**What goes wrong:** baseline.json has only 1 entry per contextual star bucket (1 CanChi pair, 1 year stem, 1 month, 1 tiết khí). These are almost certainly seed values for testing, not a complete production ruleset. Phase 1 should document what KHCBPPT actually contains for these categories so Phase 3 can detect missing rules, not just incorrect values.

**Why it happens:** The `star_rule_sets` section in baseline.json exists for precedence testing; the sparse data was intentional for test seeding, not intended as a complete almanac dataset.

**How to avoid:** Stars.md should note the expected entry count per category from KHCBPPT (e.g., "KHCBPPT lists N CanChi-keyed star rules"). Even if the exact count isn't known, documenting "KHCBPPT has extensive contextual star tables" versus "baseline.json has 1 entry each" establishes the completeness gap for Phase 3.

**Warning signs:** Stars.md only documents the values found in baseline.json without noting that KHCBPPT likely has many more entries.

## Code Examples

### Where baseline.json source_id Fields Live (Reference for Phase 1 Update)

```rust
// Source: crates/amlich-core/data/almanac/baseline.json (lines 2–13)
{
  "travel_meta": { "source_id": "khcbppt", "method": "bai-quyet" },
  "conflict_meta": { "source_id": "khcbppt", "method": "table-lookup" },
  "na_am_meta": { "source_id": "tam-menh-thong-hoi", "method": "table-lookup" },  // UPDATE if SRC-02 resolves to KHCBPPT
  "star_meta": { "source_id": "nhi-thap-bat-tu", "method": "jd-cycle" },           // Investigate in stars.md
  "day_deity_meta": { "source_id": "khcbppt", "method": "table-lookup" },
  "taboo_rule_meta": {
    "tam_nuong": { "source_id": "khcbppt", "method": "table-lookup" },
    "nguyet_ky": { "source_id": "khcbppt", "method": "table-lookup" },
    "sat_chu": { "source_id": "khcbppt", "method": "table-lookup" },
    "tho_tu": { "source_id": "khcbppt", "method": "table-lookup" }
  }
}
```

After Phase 1, these source_id values should reflect the actual verified sources per Phase 1 findings. The `na_am_meta.source_id` in particular may need updating.

### Where JD 28-Star Epoch Is Used (Reference for Epoch Investigation)

```rust
// Source: crates/amlich-core/src/almanac/calc.rs:46
let day_star_index = jd.rem_euclid(28) as usize;
let day_star_rule = &data.nhi_thap_bat_tu[day_star_index];
```

This assigns star index 0 (Giác/角) when `jd mod 28 = 0`. The epoch is the JD value for which `jd mod 28 = 0` and the star should actually be Giác. Stars.md must document whether this epoch origin is traceable to KHCBPPT or to Ho Ngoc Duc's implementation.

### Where TRUC_QUALITY Lives (Reference for truc.md Extraction)

```rust
// Source: crates/amlich-core/src/almanac/truc.rs:27–40
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

This is hardcoded in Rust source, not in baseline.json. Truc.md must extract the KHCBPPT values for these 12 quality assignments. Any correction found in Phase 3 will require a code change and recompile (Phase 4 work), not just a JSON edit.

### Where fixed_by_chi Stars Are Stored (Reference for stars.md Extraction)

```rust
// Source: crates/amlich-core/data/almanac/baseline.json - conflict_by_chi section
// Example for Tý:
"Tý": {
  "opposing_chi": "Ngọ",
  "sat_huong": "Nam",
  "cat_tinh": ["Thiên Đức", "Nguyệt Đức"],
  "sat_tinh": ["Thiên Hình", "Chu Tước"]
}
```

The `fixed_by_chi` stars live inside `conflict_by_chi` — not in a separate star table. Stars.md must extract all 12 chi entries and cross-reference the 2 cat_tinh + 2 sat_tinh per chi (24 values × quality) against KHCBPPT. Note: these values are loaded via `data.star_rule_meta.fixed_by_chi.source_id` which is "khcbppt" — but unverified.

## State of the Art

| Old Approach | Current Approach | Notes |
|--------------|------------------|-------|
| Single-source assumption (all "khcbppt") | Multi-source reality (nạp âm = tam-menh-thong-hoi, 28-star = nhi-thap-bat-tu) | Phase 1 formalizes this honest attribution |
| No intercalary month variant | Code silently uses base month rules | SRC-03 must document what KHCBPPT actually prescribes |
| Corrections verified against unspecified "classical sources" (commit 0f29f3f) | Phase 1 re-verifies against identified KHCBPPT edition | Creates traceable audit trail |

**Known unresolved in current implementation:**
- `star_meta.source_id` = "nhi-thap-bat-tu": The 28-star JD cycle is attributed to a tradition name, not a citable text. The JD epoch has no citation.
- `na_am_meta.source_id` = "tam-menh-thong-hoi": Nạp âm is sourced from a different classical text. Whether KHCBPPT also contains these tables is unknown.
- `TRUC_QUALITY` hardcoded in Rust: Never verified against KHCBPPT. Popular Vietnamese almanacs disagree on Trừ and Nguy quality classifications.

## Open Questions

1. **Does KHCBPPT contain nạp âm (納音) tables? (SRC-02)**
   - What we know: baseline.json sources nạp âm from "tam-menh-thong-hoi"; KHCBPPT's 本原 section covers fundamental principles including 五行 (Five Elements) theory; commit 0f29f3f corrected two nạp âm pair names but cited unspecified "classical sources"
   - What's unclear: Whether KHCBPPT has the full 30-pair nạp âm lookup table or only discusses principles
   - Recommendation: Inspect KHCBPPT 本原 section (vols 1–2) via ctext.org. If the 30-pair table is present, compare against baseline.json values and update source_id. If absent, treat Tam Mệnh Thông Hội as the validated source and document this explicitly in na_am.md.

2. **Where does the 28-star JD epoch come from?**
   - What we know: `jd.rem_euclid(28)` in calc.rs:46; star index 0 = Giác (角); the 28-star source_id is "nhi-thap-bat-tu" (a tradition name, not a text citation); KHCBPPT vol 13 references 28 mansions (二十八宿)
   - What's unclear: Whether KHCBPPT provides a tabular JD-cycle anchor, or whether the current epoch traces to Ho Ngoc Duc's implementation or another codebase
   - Recommendation: Check KHCBPPT vol 13 for 28-mansion tables. Cross-verify the current implementation's epoch against 3 dated historical entries from KHCBPPT's 日表 (day tables, vol ~36). If no clear epoch is in KHCBPPT, document that the epoch origin is Ho Ngoc Duc's implementation (or whichever codebase it traces to) and mark confidence LOW for the epoch in stars.md.

3. **What does KHCBPPT say about intercalary months and taboo/trực rules? (SRC-03)**
   - What we know: The current code has no intercalary month variant in `month_chi_index()`; KHCBPPT's 月表 section (vols 9–20) covers monthly tables; no intercalary month date is in current test fixtures
   - What's unclear: Whether KHCBPPT's monthly tables include explicit intercalary month rows, or whether intercalary months are treated as identical to their base month
   - Recommendation: Locate the 月表 section in KHCBPPT (approximately vols 9–20 based on the work's structure). Document findings in a dedicated "Intercalary Month Treatment" section in taboos.md. Even a finding of "KHCBPPT does not address intercalary months explicitly" is a valid and valuable SRC-03 answer.

4. **Are the contextual star rule buckets in baseline.json intentionally sparse or missing data?**
   - What we know: baseline.json has 1 entry per contextual star category (1 CanChi, 1 year stem, 1 month, 1 tiết khí); these appear to have been seeded for precedence testing (see than_sat.rs test `emits_rules_from_all_context_categories_when_data_matches`)
   - What's unclear: How many contextual star rules KHCBPPT actually defines per category
   - Recommendation: When extracting star tables from KHCBPPT, note the approximate entry count per category. Document this count in stars.md so Phase 3 can design a completeness check (expected count vs actual count), not just a value correctness check. Do not attempt a full completeness audit in Phase 1 (that is v2 scope per STR-V2-01 through STR-V2-05).

## Sources

### Primary (HIGH confidence)
- `crates/amlich-core/data/almanac/baseline.json` — current rule data, source metadata, all subsystem values; used to identify what Phase 1 must cross-reference
- `crates/amlich-core/src/almanac/truc.rs` — TRUC_QUALITY hardcoded const; directly shows what Phase 1 must extract from KHCBPPT
- `crates/amlich-core/src/almanac/calc.rs:46` — `jd.rem_euclid(28)` with no citation; identifies the epoch question
- `crates/amlich-core/src/almanac/than_sat.rs` — fixed_by_chi star loading pattern; shows all 12 chi stars come from conflict_by_chi in baseline.json
- Git commit 0f29f3f — prior data corrections to thần hướng (6 values) and nạp âm (2 names); establishes correction history

### Secondary (MEDIUM confidence)
- KHCBPPT structure (36 volumes, 11 major sections): 本原 (vols 1–2), 義例 (vols 3–8), 立成 (vol 9), 宜忌 (vol 10), 用事 (vol 11), 公規 (vols 12–13), 年表 (vols 14–19), 月表 (vols 20–31), 日表 (vol 32), 利用 (vols 33–34), 附錄/辨訛 (vols 35–36) — sourced from ctext.org metadata; verified structure via WebFetch
- 1998 NXB Mũi Cà Mau Vietnamese translation: 2-volume set, Vol 1 (979 pp), Vol 2 (1001 pp); translators Vũ Hoàng and Lân Bình; editor Mai Cốc Thành; based on 1995 Shanghai Ancient Texts and 1994 Guangxi People's Publishing House editions — confirmed from archive.org metadata
- ctext.org 四庫全書本 digitization: Full text accessible online; 欽定協紀辨方書 vol structure confirmed; CAPTCHA gate exists for full-text access but overview confirmed

### Tertiary (LOW confidence — requires text inspection)
- KHCBPPT chapter-level location of specific subsystem tables (nạp âm, 28-star, taboo rules, intercalary month treatment) — identified at section level (本原, 月表, 日表) but not chapter level; must be verified by reading
- Whether KHCBPPT's nạp âm table matches "tam-menh-thong-hoi" values — unverified; the two texts are from different traditions (KHCBPPT is trạch nhật; Tam Mệnh Thông Hội is bát tự)
- JD epoch for 28-star cycle — the current `jd.rem_euclid(28)` has no cited origin; LOW confidence that it comes from KHCBPPT vs Ho Ngoc Duc's implementation

## Metadata

**Confidence breakdown:**
- Standard stack (tools/access paths): HIGH — all tools already present or directly accessible
- Architecture (reference file structure and patterns): HIGH — follows existing `docs/almanac/` patterns in project
- KHCBPPT edition identification: MEDIUM — 1998 NXB edition and ctext.org digitization confirmed as accessible; which to use as primary requires judgment at execution time
- Nạp âm scope (SRC-02): LOW — whether KHCBPPT contains the 30-pair table is unverified; must read 本原 section
- 28-star epoch source: LOW — epoch origin untraceable from code alone; requires KHCBPPT 日表 inspection
- Intercalary month treatment (SRC-03): LOW — no existing documentation; requires 月表 inspection
- Pitfalls: HIGH — derived from direct codebase analysis and known classical text sourcing challenges

**Research date:** 2026-02-28
**Valid until:** 2026-04-30 (stable domain; edition availability is stable; LOW confidence items remain open until text inspection)
