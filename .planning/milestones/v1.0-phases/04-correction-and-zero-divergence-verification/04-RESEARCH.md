# Phase 4: Correction and Zero-Divergence Verification - Research

**Researched:** 2026-03-02
**Domain:** KHCBPPT divergence correction workflow, Rust test infrastructure, data vs code changes, traceability
**Confidence:** HIGH

## Summary

Phase 4 executes a single coordinated correction batch that fixes every divergence between the amlich implementation and KHCBPPT reference values across 7 subsystems (taboos, deity, truc, stars, xung hop, than huong, na am). The correction workflow has three phases: (1) update golden dataset entries with actual KHCBPPT values from docs/reference/khcbppt/*.md, (2) run validators to surface divergences, (3) fix divergences in either baseline.json (data-driven subsystems) or source constants (hardcoded values like TRUC_QUALITY), (4) repeat until zero divergences achieved. Success requires all 7 KHCBPPT validators pass simultaneously with all 3 pre-existing regression suites (almanac_golden.rs, ruleset_determinism.rs, taboo_boundary.rs). The 0f29f3f commit demonstrates the correction pattern: identify data error from classical sources, edit baseline.json or source file, cite KHCBPPT in commit message, run full test suite. Golden dataset entries already have per-subsystem khcbppt_ref fields for traceability, and docs/reference/khcbppt/*.md files provide chapter-level KHCBPPT citations for every value. The workflow.nyquist_validation setting is true in .planning/config.json, so validation architecture must be documented.

**Primary recommendation:** Implement single-batch correction workflow: (1) systematically update each golden entry's expected_* fields with KHCBPPT values from docs/reference/khcbppt/*.md reference files, (2) run `cargo test --package amlich-core` to inventory all divergences across all 233 entries, (3) group divergences by subsystem and target file (baseline.json or truc.rs), (4) apply corrections with KHCBPPT citations, (5) run full test suite until zero failures. Maintain per-mismatch correction ledger with required audit fields for downstream traceability.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- Golden dataset is canonical: when validator output and implementation disagree, implementation must be corrected to match KHCBPPT-cited golden entries.
- If any golden entry looks suspicious, do not force-match blindly; block and resolve the source evidence first.
- API/output contract shape must remain unchanged in this phase (behavior corrections only).
- Broad refactoring is allowed to eliminate divergence clusters, but acceptance requires a full green run: all KHCBPPT validators plus existing regression suites.
- Prefer KHCBPPT conceptual alignment over preserving legacy quirks when they conflict.
- Execute as a single coordinated correction batch (not piecemeal subsystem landings).
- Provide explicit correction notes grouped by subsystem for downstream planner/researcher use.

### Mismatch handling policy
- Phase completion requires strict zero divergence; no residual mismatches are allowed.
- Uncertain interpretation mismatches must be resolved before merge; do not defer within Phase 4 completion criteria.
- No temporary or persistent test suppression (no ignore/allowlist strategy).
- Maintain a per-mismatch correction ledger including: date, subsystem, affected entry/date, KHCBPPT citation reference, and what changed.

### Claude's Discretion
- Exact refactor structure and sequencing inside the single correction batch.
- Exact artifact format/location for the per-mismatch ledger and grouped subsystem notes.
- Exact wording style for correction notes, as long as required audit fields are present.

### Deferred Ideas (OUT OF SCOPE)
None - discussion stayed within phase scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| TAB-05 | All divergences fixed in baseline.json | Baseline.json contains taboo_rule_sets (tam_nuong, nguyet_ky, sat_chu, tho_tu), day_deity_rule_set (cycle, month_group_start_by_chi), star_rule_sets (fixed_by_canchi, by_year_can, by_lunar_month, by_tiet_khi), travel_by_can (xuat_hanh, tai_than, hy_than), conflict_by_chi (opposing_chi, sat_huong, cat_tinh, sat_tinh), na_am_pairs. docs/reference/khcbppt/*.md provide chapter-level KHCBPPT citations for each value. Correction pattern from commit 0f29f3f shows direct baseline.json edits. |
| DEI-03 | All divergences fixed in baseline.json | day_deity_rule_set.cycle (12-deity cycle order and classification) and month_group_start_by_chi (12 month-start offsets) are in baseline.json. docs/reference/khcbppt/day_deity.md provides KHCBPPT citations. |
| TRC-02 | All divergences fixed in TRUC_QUALITY const in truc.rs | TRUC_QUALITY const (hardcoded in src/almanac/truc.rs:27-40) is the only code-constant correction target. docs/reference/khcbppt/truc.md confirms all 12 quality assignments match KHCBPPT Nghia Lệ section. |
| STR-04 | All divergences fixed in baseline.json | nhi_thap_bat_tu (28-star quality assignments) and star_rule_sets (contextual buckets: fixed_by_canchi, by_year_can, by_lunar_month, by_tiet_khi) are in baseline.json. docs/reference/khcbppt/stars.md provides KHCBPPT citations and notes JD epoch MEDIUM confidence. |
| THH-02 | All divergences fixed in baseline.json | travel_by_can (10 stems × 3 directions: xuat_hanh_huong, tai_than, hy_than) is in baseline.json. docs/reference/khcbppt/than_huong.md provides KHCBPPT citations. |
| XH-02 | All divergences fixed in xung_hop.rs | xung_hop.rs implements formula-based computation (luc_xung, tam_hop, tu_hanh_xung). docs/reference/khcbppt/xung_hop.md confirms formulas match KHCBPPT mathematical derivation. Divergences unlikely if formulas are correct; any corrections would be logic changes in src/almanac/xung_hop.rs. |
| NAM-02 | All divergences fixed in baseline.json | na_am_pairs (30 sexagenary sound pairs) and sexagenary_na_am mapping in baseline.json. docs/reference/khcbppt/na_am.md provides canonical table. |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust built-in test framework | stable | `#[test]` functions, `cargo test` runner | Already in use; 155 tests passing; CI uses cargo test; no external framework needed |
| amlich-core (self) | workspace | `get_day_info()`, `load_golden_dataset()`, all subsystem APIs | All subsystem data paths already public and tested |
| serde / serde_json | 1.0 (workspace) | Golden dataset and baseline.json deserialization | Already a dependency; `include_str!` + `serde_json::from_str` pattern established |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| std::sync::OnceLock | stable | Thread-safe golden dataset caching (load_golden_dataset) | Used by golden_loader.rs; prevents re-parse on repeated calls |
| std::collections::{HashMap, HashSet} | stable | Map-based lookups in baseline.json ingestion; set-based taboo comparison | HashMap for O(1) key access; HashSet for order-independent comparison |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Single-batch corrections | Piecemeal subsystem landings | User locked decision: "Execute as a single coordinated correction batch" — piecemeal violates constraint |
| Direct baseline.json edits | External data file with import | Single-file baseline.json is simpler; Cargo's include_str! embeds it at compile time; no runtime file I/O needed |
| Collect-then-assert in validators | Early-exit on first failure | User constraint: "No temporary or persistent test suppression" — collect-then-assert shows ALL divergences, enabling complete inventory before correction |

**Installation:**
No new dependencies needed. All required crates are already in workspace Cargo.toml and amlich-core's Cargo.toml.

## Architecture Patterns

### Recommended Project Structure
```
crates/amlich-core/
  src/almanac/
    data.rs                 # Baseline ingestion via include_str!("baseline.json")
    truc.rs                 # TRUC_QUALITY const (hardcoded, code change target)
    xung_hop.rs            # Formula-based (luc_xung, tam_hop, tu_hanh_xung)
    day_deity.rs           # Resolves deity from day_deity_rule_set
    taboo.rs               # Resolves taboos from taboo_rule_sets
    stars.rs               # Resolves stars from nhi_thap_bat_tu + star_rule_sets
    than_huong.rs          # Resolves travel from travel_by_can
    na_am.rs              # Resolves na_am from sexagenary_na_am
  data/almanac/
    baseline.json          # PRIMARY CORRECTION TARGET (taboos, deity, stars, than huong, na am)
    khcbppt-golden.json  # Golden dataset with 233 entries (update expected_* fields with KHCBPPT values)
  tests/
    khcbppt_taboos.rs     # TAB-01..TAB-05 validator
    khcbppt_deity.rs      # DEI-01..DEI-03 validator
    khcbppt_truc.rs       # TRC-01..TRC-02 validator
    khcbppt_stars.rs      # STR-01..STR-04 validator
    khcbppt_xung_hop.rs   # XH-01..XH-02 validator
    khcbppt_than_huong.rs # THH-01..THH-02 validator
    khcbppt_na_am.rs      # NAM-01..NAM-02 validator
    almanac_golden.rs     # Regression test (7 golden examples) — PRESERVE
    ruleset_determinism.rs # Regression test (5 determinism tests) — PRESERVE
    taboo_boundary.rs      # Regression test (5 boundary tests) — PRESERVE
```

### Pattern 1: Single-Batch Correction Workflow
**What:** All divergences identified, grouped by subsystem, then corrected in one coordinated commit (or multiple commits within a short window) with full green test suite as acceptance gate.
**When to use:** Every Phase 4 correction iteration.
**Example:**
```bash
# Step 1: Update golden dataset entries with KHCBPPT values from docs/reference/khcbppt/*.md
# (Manual or automated script to edit khcbppt-golden.json expected_* fields)

# Step 2: Run validators to inventory divergences
cargo test --package amlich-core

# Step 3: Group divergences by target file (baseline.json vs truc.rs vs xung_hop.rs)
# Divergence report format: "[{solar_date}] {subsystem}: expected '{X}', got '{Y}'"

# Step 4: Apply corrections
# For baseline.json subsystems: edit crates/amlich-core/data/almanac/baseline.json
# For TRUC_QUALITY: edit crates/amlich-core/src/almanac/truc.rs:27-40
# For xung_hop.rs: verify formulas match docs/reference/khcbppt/xung_hop.md

# Step 5: Run full test suite (all validators + regressions)
cargo test --package amlich-core

# Repeat Steps 4-5 until zero failures
```

### Pattern 2: Data-Driven vs Code-Constant Corrections
**What:** Distinguish between corrections that go in baseline.json (data files) vs corrections that require code changes (source constants).
**When to use:** Every correction in Phase 4.
**Data-driven subsystems (baseline.json target):**
- Taboos: tam_nuong.lunar_days, nguyet_ky.lunar_days, sat_chu.by_lunar_month, tho_tu.by_lunar_month
- Day deity: day_deity_rule_set.cycle, month_group_start_by_chi
- Stars: nhi_thap_bat_tu[*].quality, star_rule_sets.* (all contextual buckets)
- Than huong: travel_by_can[*].xuat_hanh_huong, tai_than, hy_than
- Na am: na_am_pairs (array of 30), sexagenary_na_am mapping
**Code-constant subsystems (source file target):**
- Truc: TRUC_QUALITY const in src/almanac/truc.rs:27-40 (hardcoded array)
- Xung hop: luc_xung(), tam_hop(), tu_hanh_xung() in src/almanac/xung_hop.rs (formula-based)

### Pattern 3: KHCBPPT Citation Traceability
**What:** Every correction must cite the specific KHCBPPT chapter+section that defines the correct value. Golden entries already have khcbppt_ref fields; docs/reference/khcbppt/*.md files provide chapter-level citations.
**When to use:** Every correction commit message and correction ledger entry.
**Example:**
```bash
git commit -m "fix(taboos): correct sat_chu month 1 branch per KHCBPPT

- sat_chu.by_lunar_month.\"1\": \"Tỵ\" → \"Mão\" (month 1 branch is Mão per KHCBPPT Nguyet Bieu vol 20)
- Citation: KHCBPPT, Quyển 20, Nguyệt Biêu (月表) — Sát Chủ
- Affects 3 golden entries with month 1, day chi Mão
- Validator passes: khcbppt_taboos.rs now reports 0 divergences
"
```

### Anti-Patterns to Avoid
- **Piecemeal subsystem landings:** Violates user constraint "Execute as a single coordinated correction batch." Must correct all subsystems in one batch before accepting.
- **Blind force-matching suspicious golden entries:** User constraint "If any golden entry looks suspicious, do not force-match blindly; block and resolve source evidence first."
- **Test suppression (#[ignore], allowlist):** User constraint "No temporary or persistent test suppression" — strict zero divergence required.
- **Breaking API contract:** User constraint "API/output contract shape must remain unchanged" — only correct behavior, not type signatures or public interfaces.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Correction ledger format | Custom JSON/YAML schema | Simple markdown file with table structure | Per-mismatch ledger is for human audit, not machine processing; markdown tables are readable and git-diffable |
| Golden dataset updater script | Manual text editor for 233 entries | Manual verification is safer for now | Golden dataset updates require careful KHCBPPT citation alignment; automation risk of citation errors outweighs speed benefit |
| Divergence aggregation tool | Custom binary to parse test output | `cargo test -- --nocapture` shows full reports | Validators already implement collect-then-assert with eprintln! output; no additional tool needed |
| Subsystem-specific test runners | Separate cargo invocations per subsystem | Single `cargo test --package amlich-core` command | CI uses single command; splitting fragments test suite and complicates regression preservation |

**Key insight:** Phase 4 corrections are manual, careful, citation-backed edits to baseline.json and source constants. Don't over-engineer automation — human-in-the-loop verification against KHCBPPT reference docs is core value.

## Common Pitfalls

### Pitfall 1: Forgetting regression test preservation
**What goes wrong:** Fixing KHCBPPT validators breaks almanac_golden.rs, ruleset_determinism.rs, or taboo_boundary.rs tests.
**Why it happens:** Regression tests use specific golden dates; corrections that change baseline.json or source logic may affect these dates.
**How to avoid:** After each correction iteration, run `cargo test --package amlich-core` and confirm ALL 155 tests pass (not just khcbppt_*.rs validators).
**Warning signs:** Test output shows "test result: ok. X passed; Y failed" where Y > 0 in any test target.

### Pitfall 2: Updating golden entries without KHCBPPT citations
**What goes wrong:** Golden dataset expected_* fields are updated but khcbppt_ref fields are not verified against docs/reference/khcbppt/*.md.
**Why it happens:** Rushing through 233 entries; skipping reference doc verification.
**How to avoid:** For each golden entry, cross-check the relevant subsystem reference file (e.g., docs/reference/khcbppt/truc.md for expected_truc_quality) before editing khcbppt-golden.json.
**Warning signs:** Golden entry edit does not cite a specific KHCBPPT chapter+section in commit message or correction ledger.

### Pitfall 3: Mixing data-driven and code-constant corrections
**What goes wrong:** Attempting to fix TRUC_QUALITY by editing baseline.json, or fixing taboos by editing taboo.rs logic.
**Why it happens:** Not checking subsystem architecture before correcting.
**How to avoid:** Use the "Data-Driven vs Code-Constant Corrections" pattern: baseline.json for taboos/deity/stars/than huong/na am, truc.rs for TRUC_QUALITY, xung_hop.rs for formula verification.
**Warning signs:** Commit message claims "fix(TRUC_QUALITY)" but edits baseline.json instead of truc.rs.

### Pitfall 4: Incomplete KHCBPPT citation granularity
**What goes wrong:** Citing "KHCBPPT" without volume+section (e.g., "KHCBPPT Nghia Lệ" is insufficient).
**Why it happens:** EDITION.md requires chapter-level granularity (Volume + Section).
**How to avoid:** Use citation format from EDITION.md: `KHCBPPT, Quyển [N], [Section name in Vietnamese]` or `KHCBPPT, 卷[N], [Chinese section name]`.
**Warning signs:** Citation is missing volume number (Quyển/卷) or section name.

### Pitfall 5: Ignoring star rule sparsity
**What goes wrong:** Correcting FixedByChi stars but leaving contextual buckets (fixed_by_canchi, by_year, by_month, by_tiet_khi) empty or incorrect.
**Why it happens:** docs/reference/khcbppt/stars.md notes 233/233 entries have zero contextual rules — may indicate missing data, not just incorrect values.
**How to avoid:** After star corrections, verify contextual bucket coverage by running `cargo test -- --nocapture khcbppt_stars::report_star_rule_sparsity` and checking if sparsity report is still 233/233.
**Warning signs:** Star validator passes but sparsity report shows 100% coverage missing for contextual categories.

## Code Examples

Verified patterns from project codebase and documentation:

### Updating baseline.json (Data-Driven Correction)
```bash
# File: crates/amlich-core/data/almanac/baseline.json
# Example: Correct sat_chu month 1 branch from "Tỵ" to "Mão"
{
  "taboo_rule_sets": {
    "sat_chu": {
      "rule_id": "sat_chu",
      "name": "Sát Chủ",
      "severity": "hard",
      "by_lunar_month": {
        "1": "Mão",  // Corrected from "Tỵ" per KHCBPPT, Quyển 20, Nguyệt Biêu
        "2": "Tý",
        // ... rest of months
      }
    }
  }
}
```

### Updating TRUC_QUALITY Const (Code-Constant Correction)
```rust
// File: crates/amlich-core/src/almanac/truc.rs
// Example: Correct Nguy (index 7) quality from "cat" to "hung"
pub const TRUC_QUALITY: [&str; 12] = [
    "cat",  // Kiến
    "cat",  // Tr除
    "hung", // Mãn
    "binh", // Bình
    "cat",  // Định
    "binh", // Chấp
    "hung", // Phá
    "hung", // Nguy — Corrected from "cat" per KHCBPPT, Quyển 3-8, Nghĩa Lệ
    "cat",  // Thành
    "hung", // Thu
    "cat",  // Khai
    "hung", // Bế
];
```

### Running Validators with Full Divergence Report
```bash
# Run all KHCBPPT validators with nocapture to see divergence details
cargo test --package amlich-core -- --nocapture

# Run specific validator
cargo test --package amlich-core --test khcbppt_taboos -- --nocapture

# Expected output (if divergences exist):
# === TABOO DIVERGENCE REPORT (3 mismatches across 233 entries) ===
#   [2024-02-10] taboos MISSING (in golden, not in impl): ["sat_chu"]
#   [2024-02-10] taboos EXTRA (in impl, not in golden): ["tam_nuong"]
#   [2025-01-29] taboos MISSING (in golden, not in impl): ["tho_tu"]
# === END TABOO REPORT ===
```

### Verification Against Reference Docs
```bash
# Before correcting a value, check KHCBPPT citation in reference doc
cat docs/reference/khcbppt/truc.md | grep -A 5 "Nguy (危, index 7)"

# Expected output:
# **Nguy (危, index 7):**
# - KHCBPPT: **hung** (凶) — confirmed in 義例 section
# - Some popular Vietnamese almanacs: **cat** (吉) — common variant
# - TRUC_QUALITY: **hung** — matches KHCBPPT
# - **Recommendation:** TRUC_QUALITY is correct per KHCBPPT. The cat variant is a popular simplification.
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Piecemeal subsystem corrections | Single-batch coordinated corrections | Phase 4 CONTEXT (2026-03-02) | Requires full green run before acceptance; no partial landings |
| Unverified baseline.json values | KHCBPPT-cited golden dataset as source of truth | Phase 2 golden dataset creation (2026-03-01) | Golden dataset has khcbppt_ref fields; docs/reference/khcbppt/*.md provide chapter-level citations |
| Zero-divergence not enforced | Strict zero-divergence requirement with no test suppression | Phase 4 CONTEXT (2026-03-02) | Phase completion requires 0 failures across all validators and regressions |
| Manual divergence inventory | Automated validator divergence reports | Phase 3 validator harness (2026-03-01) | All 7 validators implement collect-then-assert with full eprintln! reports |

**Deprecated/outdated:**
- **0f29f3f-style corrections without golden dataset:** Old corrections cited "classical sources" broadly; new corrections must cite specific KHCBPPT chapter+section from reference docs.
- **Self-consistency validation:** Phase 3 validators passed because golden dataset was generated from get_day_info(). Phase 4 corrections break self-consistency to align with KHCBPPT — this is expected and correct.

## Open Questions

1. **Golden dataset update workflow automation or manual?**
   - What we know: Golden dataset has 233 entries; each requires expected_* field updates with KHCBPPT values from 8 reference docs (taboos.md, day_deity.md, truc.md, stars.md, xung_hop.md, than_huong.md, na_am.md, EDITION.md).
   - What's unclear: Whether to automate golden entry updates via script or perform manual edits.
   - Recommendation: Manual verification for Phase 4 given human-in-the-loop requirement (user constraint "If any golden entry looks suspicious, do not force-match blindly"). Automation risk of citation errors outweighs speed benefit. Consider automation script for Phase v2 if pattern repeats.

2. **How to handle contested values across KHCBPPT editions?**
   - What we know: EDITION.md notes primary (ctext.org 四庫全書) and secondary (1998 NXB Mũi Cà Mau Vietnamese translation) editions. Some values may differ between editions (e.g., popular Vietnamese almanac variants for Trừ and Nguy quality).
   - What's unclear: Which edition takes precedence if primary and secondary disagree.
   - Recommendation: Follow user constraint "Prefer KHCBPPT conceptual alignment over preserving legacy quirks" — primary edition (ctext.org 四庫全書) takes precedence. Document secondary discrepancies in correction notes but use primary value.

3. **JD epoch correction for 28-star system if divergences found?**
   - What we know: docs/reference/khcbppt/stars.md notes "JD epoch MEDIUM confidence (Ho Ngoc Duc origin)". KHCBPPT does not define JD epoch — it's a Ho Ngoc Duc implementation artifact.
   - What's unclear: If star validator divergences require JD epoch offset change, what is the correct offset?
   - Recommendation: If star divergences are systematic (e.g., all star indices shifted by N positions), consider JD epoch offset adjustment. Block and investigate if only some stars diverge — may indicate baseline.json data errors instead of epoch issue. Preserve user constraint "If any golden entry looks suspicious, do not force-match blindly."

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust built-in test framework (stable) |
| Config file | none (standard cargo test) |
| Quick run command | `cargo test --package amlich-core` |
| Full suite command | `cargo test --package amlich-core -- --nocapture` |

### Phase Requirements → Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| TAB-05 | Zero taboos divergences vs KHCBPPT | integration | `cargo test --package amlich-core --test khcbppt_taboos` | ✅ Phase 3 |
| DEI-03 | Zero deity divergences vs KHCBPPT | integration | `cargo test --package amlich-core --test khcbppt_deity` | ✅ Phase 3 |
| TRC-02 | Zero truc divergences vs KHCBPPT | integration | `cargo test --package amlich-core --test khcbppt_truc` | ✅ Phase 3 |
| STR-04 | Zero star divergences vs KHCBPPT | integration | `cargo test --package amlich-core --test khcbppt_stars` | ✅ Phase 3 |
| THH-02 | Zero than huong divergences vs KHCBPPT | integration | `cargo test --package amlich-core --test khcbppt_than_huong` | ✅ Phase 3 |
| XH-02 | Zero xung hop divergences vs KHCBPPT | integration | `cargo test --package amlich-core --test khcbppt_xung_hop` | ✅ Phase 3 |
| NAM-02 | Zero na am divergences vs KHCBPPT | integration | `cargo test --package amlich-core --test khcbppt_na_am` | ✅ Phase 3 |

### Sampling Rate
- **Per task commit:** `cargo test --package amlich-core`
- **Per wave merge:** `cargo test --package amlich-core -- --nocapture` (full divergence reports)
- **Phase gate:** Full suite green (all validators + regressions) before `/gsd-verify-work`

### Wave 0 Gaps
None — existing test infrastructure covers all phase requirements. All 7 KHCBPPT validators and 3 regression test suites exist and pass (155 tests total).

## Sources

### Primary (HIGH confidence)
- **Project codebase (read 2026-03-02):** crates/amlich-core/tests/khcbppt_*.rs (7 validator files), crates/amlich-core/src/almanac/*.rs (source files), crates/amlich-core/data/almanac/baseline.json (data file), crates/amlich-core/data/almanac/khcbppt-golden.json (golden dataset)
- **docs/reference/khcbppt/*.md (read 2026-03-02):** EDITION.md (edition and citation format), truc.md (TRUC_QUALITY verification), taboos.md (taboo rules), day_deity.md (deity cycle), stars.md (28-star system), xung_hop.md (xung hop formulas), than_huong.md (travel directions), na_am.md (nạp âm pairs)
- **Commit 0f29f3f (read 2026-03-02):** Correction pattern example with 8 data errors fixed against classical sources
- **Rust test framework documentation (known):** `#[test]` attributes, `cargo test` runner, collect-then-assert pattern

### Secondary (MEDIUM confidence)
- **Phase 3 verification (read 2026-03-02):** 03-VERIFICATION.md confirms all 7 validators pass with 0 divergences (self-consistent because golden dataset generated from get_day_info())
- **Phase 3 research (read 2026-03-02):** 03-RESEARCH.md documents validator design patterns and test infrastructure

### Tertiary (LOW confidence)
- None — all research findings are from primary project sources.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Rust test framework, serde, OnceLock, HashMap/HashSet are project-confirmed and stable
- Architecture: HIGH - Single-batch correction, data-driven vs code-constant pattern, citation traceability are documented in CONTEXT.md and project history
- Pitfalls: HIGH - Five specific pitfalls identified from user constraints, commit 0f29f3f example, and Phase 3 verification

**Research date:** 2026-03-02
**Valid until:** 30 days (stable project phase — no new tooling or infrastructure planned before Phase 4 execution)
