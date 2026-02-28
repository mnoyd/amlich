# Pitfalls Research

**Domain:** Vietnamese almanac correctness validation against KHCBPPT
**Researched:** 2026-02-28
**Confidence:** HIGH (codebase analysis), MEDIUM (classical text interpretation)

## Critical Pitfalls

### Pitfall 1: Confusing Internal Consistency Tests with Source Fidelity

**Problem:** Existing tests verify the implementation is internally consistent — not that it matches KHCBPPT. Evidence metadata saying `source_id: "khcbppt"` creates the appearance of traceability without actual cross-referencing.

**Warning signs:** Golden test expected values that match current output but have no KHCBPPT citation.

**Prevention:** Build the golden dataset from KHCBPPT text first, then compare — never the reverse. Every golden entry must have a `khcbppt_ref` citation.

**Phase:** Golden dataset creation.

### Pitfall 2: The JD Epoch Offset Trap for Nhị Thập Bát Tú

**Problem:** `jd.rem_euclid(28)` in `calc.rs:46` is only correct if JD mod 28 = 0 corresponds to star index 0 (Giác). The only existing test checks `index < 28`, not the actual star name for any real date. If the offset is wrong by 1, every 28-star entry is shifted.

**Warning signs:** Star names that are consistently off by 1 position across all dates.

**Prevention:** Verify with 3+ real dated entries from KHCBPPT before proceeding with other star validation.

**Phase:** Star rule cross-referencing (first priority within stars).

### Pitfall 3: Day Deity Month Anchor Ambiguity

**Problem:** `month_group_start_by_chi` anchors the 12-deity cycle to lunar month branches. Tests only verify tháng 1 / day Tý → Thanh Long. If the anchor map is wrong for other months, all deity assignments for those months are wrong.

**Warning signs:** Day deity correct for month 1 but wrong for other months.

**Prevention:** Verify at least one date per month group (12 months) against KHCBPPT.

**Phase:** Day deity cross-referencing.

### Pitfall 4: Trực Quality Assignments Unverified

**Problem:** The trực formula `(day_chi - month_chi + 12) % 12` is correct (proven by structural invariant tests). But the `TRUC_QUALITY` array mapping indices to cat/hung/binh is unverified against KHCBPPT. Popular Vietnamese almanacs disagree on whether Trừ is cat or binh, whether Nguy is hung or binh.

**Warning signs:** Quality values that disagree with KHCBPPT's thập nhị trực chapter.

**Prevention:** Look up the thập nhị trực chapter specifically; verify all 12 quality assignments.

**Phase:** Trực cross-referencing.

### Pitfall 5: Star Rule Data Is Sparse — By Design or By Omission?

**Problem:** `star_rule_sets` in baseline.json has only 1 entry per contextual category (1 CanChi pair, 1 year, 1 month, 1 tiết khí). This was likely seeded for precedence testing, not as a production dataset. Cross-referencing could miss that hundreds of entries are missing entirely.

**Warning signs:** Golden dataset comparison only catches quality errors, not missing stars.

**Prevention:** Establish completeness contract (expected entry counts per category from KHCBPPT) before any correction work. Count entries, don't just check values.

**Phase:** Reference data compilation.

### Pitfall 6: Misidentifying Which KHCBPPT Edition to Trust

**Problem:** Multiple editions/reprints of KHCBPPT exist. Vietnamese almanacs that cite "KHCBPPT" may actually derive from 20th-century adaptations or modern compilations, not the Qing dynasty original.

**Warning signs:** Conflicting values between different "KHCBPPT" sources.

**Prevention:** Document the specific edition/source explicitly in the golden dataset metadata before compiling any data.

**Phase:** Source establishment (project kickoff).

### Pitfall 7: Intercalary Month Handling Undefined

**Problem:** Taboo rules are keyed 1–12 with no intercalary month variant. `month_chi_index()` has no intercalary handling. KHCBPPT's treatment of leap months may differ from simply repeating the base month's rules.

**Warning signs:** Intercalary month dates producing identical output to the base month without verification.

**Prevention:** Verify KHCBPPT's intercalary month rules explicitly. Add an intercalary month date to the golden dataset (e.g., intercalary April 2020).

**Phase:** Taboo and trực cross-referencing.

### Pitfall 8: Schema Rigidity Causes Cascading Panics

**Problem:** Multiple `.expect()` calls on HashMap lookups in `calc.rs` will panic at runtime if any baseline.json structural change removes or renames a key. No compile-time safety for data schema.

**Warning signs:** `cargo test` passes before JSON edit, panics after.

**Prevention:** Run `cargo test` after every JSON edit without exception. Consider adding a schema validation test that runs before all other tests.

**Phase:** Standing protocol for all correction phases.

## Phase-Specific Warnings

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| Tests ≠ KHCBPPT validation | Golden dataset creation | Entries have citation fields |
| JD offset for 28-star cycle | Star cross-referencing | 3+ real dated entries verified first |
| Day deity anchor | Day deity cross-referencing | All 12 months have verified date-deity pair |
| Trực quality table | Trực cross-referencing | TRUC_QUALITY has direct KHCBPPT citation |
| Sparse star data | Reference data compilation | Completeness audit precedes correction |
| Wrong KHCBPPT edition | Source establishment | Edition documented in dataset metadata |
| Intercalary month undefined | Taboo + trực cross-referencing | Intercalary month date in golden dataset |
| Schema panics on edit | All correction phases | `cargo test` after every JSON edit |

---
*Pitfalls research: 2026-02-28*
