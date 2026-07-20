# Phase 2: Golden Dataset and Loader - Research

**Researched:** 2026-03-01
**Domain:** Rust JSON deserialization, golden test dataset design, Vietnamese almanac subsystem coverage
**Confidence:** HIGH

## Summary

Phase 2 transforms the 8 Phase 1 reference files (EDITION.md, na_am.md, taboos.md, day_deity.md, truc.md, stars.md, xung_hop.md, than_huong.md) into a single machine-readable `khcbppt-golden.json` file and a Rust loader (`golden_loader.rs`) that deserializes it into typed structs. The dataset must contain ~200 entries covering 2020--2030 with systematic coverage of all 12 chi, 10 can, 12 lunar months, and 28 JD-cycle star positions. Every entry must carry a `khcbppt_ref` citation field.

The project already has a robust pattern for JSON data loading: `baseline.json` is loaded via `include_str!` + `serde_json::from_str` into typed structs with extensive validation. The golden dataset follows this same pattern but serves a different purpose -- it is a *test oracle* not a *runtime config*. The golden dataset lives in the test infrastructure (`crates/amlich-core/tests/` or `crates/amlich-core/data/`), is loaded by test code, and each entry is compared against `calculate_day_fortune()` + `get_day_info()` output to surface divergences.

The key technical challenge is not the JSON/Rust plumbing (which is straightforward serde) but the *dataset construction*: selecting ~200 dates that systematically cover all required dimensions (12 chi, 10 can, 12 lunar months, 28 JD-cycle positions) while staying within 2020--2030, and attaching correct KHCBPPT citations to each entry.

**Primary recommendation:** Model each golden entry after the `DayFortune` output structure (which already has `Serialize`/`Deserialize` derives), add `solar_date`, `lunar_date`, and `khcbppt_ref` metadata fields, and use `get_day_info()` to generate candidate values that are then manually verified against Phase 1 reference files. The loader is a straightforward serde deserialize into a `Vec<GoldenEntry>` with validation.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| DATA-01 | Golden reference dataset created with ~200 representative dates covering 2020--2030 | Date selection algorithm in Architecture Patterns section; coverage matrix approach ensures all dimensions hit |
| DATA-02 | Dataset covers all 12 chi, 10 can, 12 lunar months, 28 JD-cycle positions | Coverage analysis in Architecture Patterns shows these are achievable within 2020--2030; LCM analysis confirms minimum dates needed |
| DATA-03 | Every golden entry includes KHCBPPT citation (`khcbppt_ref` field) | Citation format from Phase 1 EDITION.md; per-subsystem citations from 8 reference files; schema includes `khcbppt_ref` at entry level + per-subsystem level |
| DATA-04 | Golden loader (`golden_loader.rs`) deserializes dataset into typed Rust structs | Existing `baseline.json` / `data.rs` pattern provides exact template; serde + include_str! pattern verified working |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| serde | 1.0 (workspace) | JSON serialization/deserialization | Already in workspace dependencies with `derive` feature |
| serde_json | 1.0 (workspace) | JSON parsing | Already in workspace dependencies |
| chrono | 0.4 (workspace) | Date handling for solar date generation | Already in workspace dependencies |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| amlich-core (self) | workspace | `get_day_info()` for generating candidate golden values | During dataset construction and test assertions |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| JSON golden file | TOML/YAML | JSON matches existing baseline.json pattern; serde_json already a dependency; no benefit to switching |
| include_str! (compile-time embed) | std::fs::read at runtime | For test-only data, either works; include_str! is simpler and matches baseline.json pattern; use std::fs if file is too large for compile |
| Single golden file | Per-subsystem golden files | Single file is simpler; ~200 entries at ~50 fields each is ~10K lines of JSON -- manageable |

**Installation:**
No new dependencies needed. All required crates are already in `Cargo.toml`.

## Architecture Patterns

### Recommended Project Structure
```
crates/amlich-core/
  data/
    almanac/
      baseline.json          # existing runtime config
      khcbppt-golden.json    # NEW: golden test oracle
  src/
    almanac/
      golden_loader.rs       # NEW: deserialization + validation
      mod.rs                 # ADD: pub mod golden_loader
  tests/
    almanac_golden.rs        # existing (will be extended or new file added)
```

### Pattern 1: Golden Entry Schema
**What:** Each golden entry represents one date with all subsystem outputs and KHCBPPT citations
**When to use:** Every entry in khcbppt-golden.json
**Example:**
```rust
// Modeled after existing DayFortune + DayInfo structures
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenDataset {
    pub metadata: GoldenMetadata,
    pub entries: Vec<GoldenEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenMetadata {
    pub edition: String,           // "ctext.org 四庫全書 (Qianlong 1741)"
    pub secondary_edition: String, // "1998 NXB Mui Ca Mau"
    pub citation_format: String,   // "KHCBPPT, Quyen [N], [Section name]"
    pub date_range: String,        // "2020-2030"
    pub entry_count: usize,
    pub generated: String,         // ISO date
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenEntry {
    // Date identification
    pub solar_date: String,           // "2024-02-10"
    pub solar_day: i32,
    pub solar_month: i32,
    pub solar_year: i32,
    pub lunar_day: i32,
    pub lunar_month: i32,
    pub lunar_year: i32,
    pub is_leap_month: bool,
    pub jd: i32,

    // Can Chi
    pub day_canchi: String,           // "Giap Thin"
    pub day_can: String,              // "Giap"
    pub day_chi: String,              // "Thin"
    pub day_chi_index: usize,
    pub year_can: String,

    // Tiet khi
    pub tiet_khi: String,

    // Subsystem expected values
    pub expected_truc_name: String,
    pub expected_truc_index: usize,
    pub expected_truc_quality: String,

    pub expected_day_deity_name: String,
    pub expected_day_deity_classification: String, // "hoang_dao" | "hac_dao"

    pub expected_luc_xung: String,
    pub expected_tam_hop: Vec<String>,
    pub expected_tu_hanh_xung: Vec<String>,

    pub expected_na_am: String,
    pub expected_element: String,

    pub expected_travel_xuat_hanh: String,
    pub expected_tai_than: String,
    pub expected_hy_than: String,

    pub expected_star_index: usize,
    pub expected_star_name: String,
    pub expected_star_quality: String,

    // Taboo expectations (which taboo rules should fire)
    pub expected_taboos: Vec<String>,  // rule_ids that should fire

    // Citation
    pub khcbppt_ref: GoldenCitation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenCitation {
    /// General entry-level citation
    pub entry_note: String,
    /// Per-subsystem citations from Phase 1 reference files
    pub truc: String,
    pub day_deity: String,
    pub taboos: String,
    pub stars: String,
    pub xung_hop: String,
    pub than_huong: String,
    pub na_am: String,
}
```

### Pattern 2: Coverage-Driven Date Selection
**What:** Systematic date selection algorithm to ensure all dimensions are covered
**When to use:** Constructing the ~200 date list

The coverage constraints are:
- 12 chi (day earthly branches): cycle every 12 days -- any 12 consecutive days covers all
- 10 can (day heavenly stems): cycle every 10 days -- any 10 consecutive days covers all
- 12 lunar months: need at least one date from each lunar month across 2020--2030
- 28 JD-cycle positions: JD mod 28 cycles every 28 days -- any 28 consecutive days covers all
- LCM(12, 10, 28) = 420 days -- so 420 consecutive days guarantees full dimensional coverage

**Selection strategy:**
1. Start with 60 dates (one per sexagenary day pair) from a 60-day window to cover all can-chi combinations
2. Add 12 dates (one per lunar month, 1st of each month) across different years
3. Fill remaining ~128 dates to ensure 28-star coverage and edge cases (leap months, year boundaries, tiet khi transitions)
4. Target: at least 7 dates per chi (12x7=84), at least 5 dates per can (10x5=50), at least 3 dates per lunar month (12x3=36), at least 3 dates per JD-mod-28 position (28x3=84)
5. Total with overlap: ~200 is achievable

### Pattern 3: Loader Pattern (Matching baseline.json)
**What:** Use the same `include_str!` + serde pattern as baseline.json
**When to use:** golden_loader.rs implementation
**Example:**
```rust
// Source: crates/amlich-core/src/almanac/data.rs (existing pattern)
use std::sync::OnceLock;

const GOLDEN_JSON: &str = include_str!("../../data/almanac/khcbppt-golden.json");

static GOLDEN_DATA: OnceLock<GoldenDataset> = OnceLock::new();

pub fn golden_dataset() -> &'static GoldenDataset {
    GOLDEN_DATA.get_or_init(|| {
        let dataset: GoldenDataset =
            serde_json::from_str(GOLDEN_JSON).expect("Failed to parse golden dataset");
        validate_golden_dataset(&dataset);
        dataset
    })
}

fn validate_golden_dataset(dataset: &GoldenDataset) {
    assert!(!dataset.entries.is_empty(), "golden dataset must not be empty");
    assert!(
        dataset.entries.len() >= 150,
        "golden dataset must have at least 150 entries, got {}",
        dataset.entries.len()
    );
    // Validate coverage
    validate_coverage(dataset);
}
```

### Anti-Patterns to Avoid
- **Generating golden values programmatically without verification:** The golden dataset is the *truth source* -- values must come from Phase 1 reference files (KHCBPPT citations), not from `calculate_day_fortune()`. Use `get_day_info()` to generate *candidates*, then verify each value against the reference tables. For subsystems already confirmed correct in Phase 1 (all 33 taboo values match, all 12 truc qualities match, all 24 deity values match, etc.), the implementation output IS the verified value.
- **Putting golden dataset in src/ production code:** The golden dataset is test infrastructure. It should be loadable from `src/` (so Phase 3 validators can use it) but its purpose is test oracle, not runtime config.
- **Omitting per-subsystem citations:** A single `khcbppt_ref` string per entry is insufficient. Each subsystem's expected value needs its own citation trail back to Phase 1 reference files.
- **Hardcoding Vietnamese diacritics without verification:** All string values (chi names, deity names, truc names) must exactly match the constants in `types.rs` (`CAN`, `CHI` arrays) and the baseline.json values.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| JSON deserialization | Custom parser | serde + serde_json (already in workspace) | Battle-tested, derive macros handle all edge cases |
| Date arithmetic for coverage selection | Manual JD calculation | `jd_from_date()` from julian.rs + `convert_solar_to_lunar()` from lunar.rs | Already tested and verified in this codebase |
| Can/Chi cycle computation | Manual modular arithmetic | `get_day_canchi(jd)` from canchi.rs | Already handles all edge cases including wrap-around |
| Golden value generation | Manual lookup tables | `get_day_info(day, month, year)` to get candidate values | Produces all subsystem outputs in one call; Phase 1 verified these match KHCBPPT for confirmed subsystems |

**Key insight:** The existing codebase already computes all subsystem values. The golden dataset's primary purpose is not to provide *new* values but to *freeze known-correct values* with KHCBPPT citations so that Phase 3 validators can detect regressions and divergences. For subsystems confirmed correct in Phase 1 (all values matched), the implementation output IS the golden value. For subsystems with gaps (JD epoch, star rule sparsity), the golden dataset documents the current value with appropriate confidence level.

## Common Pitfalls

### Pitfall 1: Incomplete Dimensional Coverage
**What goes wrong:** Selecting 200 dates without checking that all 28 JD positions are covered, or missing a lunar month
**Why it happens:** Random or convenience-based date selection
**How to avoid:** After selecting dates, run a coverage check: `assert!(chi_coverage.len() == 12 && can_coverage.len() == 10 && month_coverage.len() == 12 && star_coverage.len() == 28)`
**Warning signs:** Any dimension with fewer than 2 representative dates

### Pitfall 2: Vietnamese Diacritic Mismatches
**What goes wrong:** Golden dataset uses "Giap" but code uses "Giap" with diacritics "Gi\u{00e1}p", causing silent comparison failures
**Why it happens:** Copy-paste from different sources, ASCII vs UTF-8 confusion
**How to avoid:** All string values in the golden dataset must be generated or verified against the CAN/CHI constants in `types.rs`. Use `get_day_info()` output as the canonical string source.
**Warning signs:** Tests pass with `.contains()` but fail with `==`

### Pitfall 3: Treating Implementation Output as Ground Truth
**What goes wrong:** Using `calculate_day_fortune()` output directly as golden values without KHCBPPT verification makes the golden dataset a tautological mirror of the code, not an independent oracle
**Why it happens:** It's faster than manual verification
**How to avoid:** For each subsystem, explicitly document: "Phase 1 verified all N values match KHCBPPT" or "Phase 1 identified gap -- current value is MEDIUM/LOW confidence"
**Warning signs:** Golden dataset creation finishes suspiciously fast with zero discrepancies

### Pitfall 4: include_str! Size Limits
**What goes wrong:** Very large JSON files can slow compilation or hit memory limits
**Why it happens:** `include_str!` embeds the file in the binary at compile time
**How to avoid:** ~200 entries at ~2KB each = ~400KB JSON -- well within limits. baseline.json is already ~8KB and works fine. Only a concern if entries grow beyond ~1MB total.
**Warning signs:** Noticeably slower `cargo test` compilation

### Pitfall 5: Leap Month Edge Cases
**What goes wrong:** Dates falling in a leap (intercalary) lunar month may not be covered, leading to untested taboo/truc behavior during leap months
**Why it happens:** Leap months are rare (about 7 in 19 years) and easy to forget
**How to avoid:** Explicitly include at least 2-3 dates from leap months in the 2020-2030 range. Known leap months in this range: 2020/4 (leap month 4), 2023/2 (leap month 2), 2025/6 (leap month 6), 2028/5 (leap month 5).
**Warning signs:** `is_leap_month` is always `false` in the dataset

### Pitfall 6: JD Star Epoch Confidence
**What goes wrong:** Golden dataset records star assignments as HIGH confidence when the JD epoch itself is MEDIUM confidence (Ho Ngoc Duc artifact, not KHCBPPT-defined)
**Why it happens:** Phase 1 documented this gap but it gets forgotten during dataset construction
**How to avoid:** Mark star-related golden values with explicit confidence: "star_confidence: MEDIUM -- JD epoch not verified against KHCBPPT dated entries". Phase 3 needs this to prioritize epoch verification.
**Warning signs:** All confidence levels are uniformly HIGH

## Code Examples

### Loading and Validating Golden Dataset
```rust
// Based on existing baseline.json loading pattern in data.rs
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GoldenDataset {
    pub metadata: GoldenMetadata,
    pub entries: Vec<GoldenEntry>,
}

// Load with include_str! (compile-time embed)
const GOLDEN_JSON: &str = include_str!("../../data/almanac/khcbppt-golden.json");

pub fn load_golden() -> GoldenDataset {
    serde_json::from_str(GOLDEN_JSON).expect("Failed to parse khcbppt-golden.json")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn golden_dataset_loads_and_has_entries() {
        let dataset = load_golden();
        assert!(dataset.entries.len() >= 150);
    }

    #[test]
    fn golden_dataset_covers_all_chi() {
        let dataset = load_golden();
        let chi_set: std::collections::HashSet<&str> =
            dataset.entries.iter().map(|e| e.day_chi.as_str()).collect();
        assert_eq!(chi_set.len(), 12, "must cover all 12 chi");
    }
}
```

### Generating Candidate Golden Values
```rust
// Use get_day_info to produce candidate values for manual verification
use amlich_core::get_day_info;

fn generate_candidate(day: i32, month: i32, year: i32) {
    let info = get_day_info(day, month, year);
    let fortune = &info.day_fortune;

    // These are CANDIDATE values -- must be verified against Phase 1 reference files
    println!("solar: {}-{:02}-{:02}", year, month, day);
    println!("lunar: {}/{}/{}", info.lunar.day, info.lunar.month, info.lunar.year);
    println!("day_canchi: {}", info.canchi.day.full);
    println!("truc: {} ({})", fortune.truc.name, fortune.truc.quality);
    println!("day_deity: {:?}", fortune.day_deity.as_ref().map(|d| &d.name));
    println!("star: {:?}", fortune.stars.day_star.as_ref().map(|s| (&s.name, s.index)));
    println!("jd_mod_28: {}", info.jd.rem_euclid(28));
}
```

### Coverage Validation
```rust
// Validate that the golden dataset covers all required dimensions
fn validate_coverage(dataset: &GoldenDataset) {
    use std::collections::HashSet;

    let mut chi_seen: HashSet<String> = HashSet::new();
    let mut can_seen: HashSet<String> = HashSet::new();
    let mut month_seen: HashSet<i32> = HashSet::new();
    let mut star_pos_seen: HashSet<usize> = HashSet::new();

    for entry in &dataset.entries {
        chi_seen.insert(entry.day_chi.clone());
        can_seen.insert(entry.day_can.clone());
        month_seen.insert(entry.lunar_month);
        star_pos_seen.insert(entry.expected_star_index);
    }

    assert_eq!(chi_seen.len(), 12, "must cover all 12 chi");
    assert_eq!(can_seen.len(), 10, "must cover all 10 can");
    assert_eq!(month_seen.len(), 12, "must cover all 12 lunar months");
    assert_eq!(star_pos_seen.len(), 28, "must cover all 28 JD-cycle positions");
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Inline test assertions (almanac_golden.rs) | Externalized golden dataset (khcbppt-golden.json) | Phase 2 (now) | Enables Phase 3 bulk validation across all subsystems |
| No source attribution in tests | `khcbppt_ref` citation field per entry | Phase 2 (now) | Every divergence traceable to authoritative source |
| Manual test case selection | Systematic coverage-driven selection | Phase 2 (now) | Guarantees dimensional completeness |

**Existing test infrastructure (preserved, not replaced):**
- `almanac_golden.rs` — 7 manual golden tests for truc, xung_hop, day_deity (stays; Phase 2 adds bulk coverage alongside)
- `ruleset_determinism.rs` — 5 tests for ruleset loading determinism (stays; unrelated to golden dataset)
- `taboo_boundary.rs` — 5 boundary tests for taboo rules (stays; unrelated to golden dataset)

## Open Questions

1. **Star JD epoch verification in golden dataset**
   - What we know: JD epoch (JD 0 = Giac/index 0) is Ho Ngoc Duc implementation artifact, not KHCBPPT-defined (Phase 1 finding, MEDIUM confidence)
   - What's unclear: Should the golden dataset record the *implementation's* star assignment or leave star fields as "unverified" until Phase 3 epoch verification?
   - Recommendation: Record implementation values but mark star confidence as MEDIUM in the citation. Phase 3 will verify epoch against 3+ dated KHCBPPT entries before trusting star assertions.

2. **Leap month taboo behavior**
   - What we know: SRC-03 resolved -- KHCBPPT is silent on intercalary months; base-month inheritance is the implementation behavior
   - What's unclear: Should golden entries for leap month dates assert taboo rules based on the base month number or the leap month?
   - Recommendation: Use the base month number (which is what `lunar_month` in `get_day_info()` returns for taboo calculation). Document this decision explicitly in the golden dataset metadata.

3. **Golden dataset file location**
   - What we know: baseline.json is at `crates/amlich-core/data/almanac/baseline.json` and loaded via `include_str!`
   - What's unclear: Should `khcbppt-golden.json` go in the same `data/` directory (making it compiled into the binary) or in `tests/` (test-only)?
   - Recommendation: Place in `data/almanac/` alongside baseline.json. The loader uses `include_str!` which is conditional on compilation anyway. This keeps the "data" directory as the single source for all JSON data files, matching existing project convention.

4. **Entry count precision**
   - What we know: Requirement says "~200" entries
   - What's unclear: Is there a hard minimum? What's the upper bound?
   - Recommendation: Target 200 +/- 20. Minimum 180 to ensure adequate dimensional coverage. Maximum 240 to avoid unnecessary bulk. The coverage validation (all 12 chi, 10 can, 12 months, 28 stars) is the real constraint, not the count.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust built-in test framework (`#[test]`) + cargo test |
| Config file | `Cargo.toml` (workspace) — already configured |
| Quick run command | `cargo test --package amlich-core golden_loader -q` |
| Full suite command | `cargo test --package amlich-core` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| DATA-01 | Golden dataset has ~200 entries in 2020--2030 range | unit | `cargo test --package amlich-core golden_dataset_loads -q` | No -- Wave 0 |
| DATA-02 | Covers all 12 chi, 10 can, 12 months, 28 stars | unit | `cargo test --package amlich-core golden_coverage -q` | No -- Wave 0 |
| DATA-03 | Every entry has khcbppt_ref citation | unit | `cargo test --package amlich-core golden_citations -q` | No -- Wave 0 |
| DATA-04 | golden_loader.rs deserializes into typed structs | unit | `cargo test --package amlich-core golden_loader -q` | No -- Wave 0 |

### Sampling Rate
- **Per task commit:** `cargo test --package amlich-core -q`
- **Per wave merge:** `cargo test --package amlich-core`
- **Phase gate:** Full suite green before `/gsd:verify-work`

### Wave 0 Gaps
- [ ] `crates/amlich-core/src/almanac/golden_loader.rs` -- GoldenEntry structs + load_golden() function
- [ ] `crates/amlich-core/data/almanac/khcbppt-golden.json` -- the golden dataset file
- [ ] Test assertions in golden_loader.rs or new test file covering DATA-01 through DATA-04

*(Existing test infrastructure -- cargo test, serde, serde_json -- is already in place. No framework install needed.)*

## Sources

### Primary (HIGH confidence)
- Project source: `crates/amlich-core/src/almanac/data.rs` -- verified baseline.json loading pattern with include_str! + serde + OnceLock + validation
- Project source: `crates/amlich-core/src/almanac/types.rs` -- verified DayFortune struct with Serialize/Deserialize derives
- Project source: `crates/amlich-core/src/lib.rs` -- verified get_day_info() API producing DayInfo with all subsystem outputs
- Project source: `crates/amlich-core/Cargo.toml` -- confirmed serde 1.0, serde_json 1.0, chrono 0.4 as workspace dependencies
- Project source: `crates/amlich-core/tests/almanac_golden.rs` -- existing golden test patterns
- Project source: `crates/amlich-core/src/types.rs` -- CAN[10], CHI[12] constants (canonical Vietnamese diacritic strings)

### Secondary (MEDIUM confidence)
- Phase 1 reference files (8 files in `docs/reference/khcbppt/`) -- KHCBPPT citations for all subsystem values; MEDIUM because based on structural analysis and classical mnemonic verification, not character-level KHCBPPT text extraction

### Tertiary (LOW confidence)
- 28-star JD epoch correctness -- documented as Ho Ngoc Duc implementation artifact; awaiting Phase 3 verification against dated KHCBPPT entries

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- all libraries already in workspace; no new dependencies needed
- Architecture: HIGH -- golden_loader.rs pattern is a direct copy of baseline.json/data.rs pattern which is verified working
- Dataset design: HIGH -- dimensional coverage requirements are mathematically analyzable; ~200 dates is sufficient for all 4 dimensions
- Pitfalls: HIGH -- based on direct codebase inspection; Vietnamese diacritic and JD epoch issues are concretely documented in Phase 1 findings

**Research date:** 2026-03-01
**Valid until:** 2026-04-01 (stable -- serde/Rust ecosystem is mature; project dependencies are locked)
