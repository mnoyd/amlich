# Phase 13: Phi Tinh Primitives + Period + Annual/Monthly — Research

**Researched:** 2026-05-27
**Domain:** Huyền Không Phi Tinh (Flying Stars) — time-based period/annual/monthly palace layout computation in Rust
**Confidence:** HIGH (every integration point verified against current codebase; algorithm mechanics HIGH confidence for Hạ Nguyên era, MEDIUM for older Yuans per ADR-0003 acknowledgment)

---

## Summary

Phase 13 ships the core Flying Stars computation layer inside the already-created `almanac/fengshui/` folder. Phase 10 delivered frozen types (`FlyingStarLayout`, `Palace`, `FlyingStar`, `FlyingStarPeriod`) in `types.rs`; the module's `mod.rs` explicitly states "Phase 13 will add `period.rs`, `annual.rs`, `monthly.rs`, `combined.rs`". Phase 13 therefore has a clear, bounded addition scope: no type schema changes, no new dependencies, no DTO modification.

The critical algorithmic decisions are already locked by ADR-0002 (solar-term month boundaries via `get_all_tiet_khi_for_year`) and ADR-0003 (Niên Tử Bạch direction matrix by Tam Nguyên × year polarity). The Tiết Khí scanner API (`crates/amlich-core/src/tietkhi.rs`) is the sole runtime dependency for all boundary resolution. `canchi.rs` provides stem-index lookup for year polarity. No new crate dependencies are needed.

The phase has three technical risk areas: (1) Lo Shu base palace table accuracy — mitigated by JSON data file + load-time invariant checks (PITFALLS CRIT-4); (2) Vận period boundary off-by-one at Lập Xuân — mitigated by `get_all_tiet_khi_for_year` reuse (PITFALLS CRIT-2); (3) Niên Tử Bạch direction matrix for Thượng/Trung Nguyên rows — MEDIUM confidence, Phase 13 is explicitly the designated cross-validation phase for those rows per ADR-0003. The multi-source golden dataset (FS-10) is the primary quality gate.

**Primary recommendation:** Follow the `golden_loader.rs` OnceLock + JSON pattern for star metadata and base palace tables; implement Vận/Niên/Nguyệt computations as pure functions with explicit `get_all_tiet_khi_for_year` calls for all boundary resolution; attach separate `ReasoningEvidenceEnvelope` per sub-star layer.

---

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| FS-01 | `compute_period(year, term_scanner) -> Period` using Lập Xuân boundary | ADR-0003 defines Vận ranges; `get_all_tiet_khi_for_year` resolves the exact boundary instant for each year |
| FS-02 | `Palace` enum with canonical Lo Shu numbering + `palace_to_direction()` | **Already delivered by Phase 10** — `types.rs` has the full `Palace` enum with `#[repr(u8)]` values 1-9, `Palace::ALL` array, and `palace_to_direction()` stub |
| FS-03 | `FlyingStar` enum (NhatBach=1…CuuTu=9) with element+polarity+auspice metadata from `data/almanac/flying_stars.json` | **Enum already delivered by Phase 10** in `types.rs`; Phase 13 creates the JSON loader and the data file |
| FS-04 | Lo Shu invariants enforced at load: sum=45, each 1-9 once, center=Vận | Load-time validation function on the base palace JSON (mirrors `validate_golden_dataset` pattern) |
| FS-05 | Vận 7 (1984-2003), Vận 8 (2004-2023), Vận 9 (2024-2043) populated and golden-tested at boundary instants | Boundary instants resolved via `get_all_tiet_khi_for_year`; golden dataset required at boundary years |
| FS-06 | `compute_yearly_flying_stars(year, term_scanner)` verified ≥10 dates per Vận, ≥2 sources | ADR-0003 polarity matrix; pure function reading year Can from `canchi::get_year_canchi` |
| FS-07 | `compute_monthly_flying_stars(year, month, term_scanner)` with solar-term month boundaries, year-branch-group rule (8/5/2 groups, descend mod-9) | ADR-0002 defines the exact 12 solar-month opening terms and the group rule |
| FS-08 | `compute_combined_overlay(year, month, term_scanner) -> CombinedFlyingStarLayout` returning `[(annual_star, monthly_star); 9]` | Composition of FS-06 and FS-07 results |
| FS-09 | Per-sub-star `ReasoningEvidenceEnvelope` (Vận, Niên, Nguyệt) + composite `rule.composite.flying_stars` envelope on aggregate | `ReasoningEvidenceEnvelope` struct is in `reasoning/types.rs`, imported via `crate::reasoning`; `method` field distinguishes sub-stars |
| FS-10 | Golden dataset ≥10 dates per Vận, ≥2 sources, *Thẩm Thị Huyền Không Học* tiebreaker, `KnownDivergence` entries | New golden JSON file under `data/almanac/`; `KnownDivergence` is a new type to introduce in Phase 13 |
</phase_requirements>

---

## Standard Stack

### Core
| Library/Module | Version/Location | Purpose | Why Standard |
|---|---|---|---|
| `serde` + `serde_json` | workspace dependency | JSON deserialization for star metadata + base palace tables + golden dataset | Project-wide pattern; already in `Cargo.toml` |
| `std::sync::OnceLock` | std | Lazy static corpus loading | Used by `golden_loader.rs`, `corpus.rs`, `holiday_data.rs` — project canonical pattern |
| `include_str!` | std | Embed JSON at compile time | Same as golden_loader; data is test oracle, not user-facing config |
| `tietkhi::get_all_tiet_khi_for_year` | `crates/amlich-core/src/tietkhi.rs:227` | Resolve Lập Xuân instant for any year (Vận boundaries + annual + monthly anchors) | Mandated by ADR-0002 and ADR-0003; v1.1.2 fix already proven correct |
| `canchi::get_year_canchi` | `crates/amlich-core/src/canchi.rs:103` | Get year Heavenly Stem for polarity determination | Already available; formula verified against 2024 (Giáp Thìn) and 2025 (Ất Tỵ) |
| `crate::reasoning::{ReasoningEvidenceEnvelope, ReasoningEvidenceSourceFamily}` | `crates/amlich-core/src/reasoning/mod.rs` | Attach evidence envelopes to layout outputs | Re-exported from `reasoning/mod.rs`; used in Phase 10 `minimal_evidence()` stub |
| `crate::sources::SOURCE_HUYEN_KHONG` | `crates/amlich-core/src/sources.rs` | Constant `"huyen-khong"` — enforced by `source_id_guard.rs` CI | No bare string literals allowed; CI test enforces this |

### No New Dependencies
Phase 13 requires zero new crate additions to `Cargo.toml`. All needed types, scanner functions, and evidence infrastructure are already in the codebase. This matches the stated project policy (SUMMARY.md: "No new crate dependencies").

---

## Architecture Patterns

### Recommended Module Structure (inside `almanac/fengshui/`)

Phase 10's `mod.rs` comment explicitly states the planned layout:

```
crates/amlich-core/src/almanac/fengshui/
├── mod.rs          # existing — re-exports; add pub mod declarations for new files
├── types.rs        # existing — Palace, FlyingStar, FlyingStarLayout, FlyingStarPeriod (FROZEN)
├── period.rs       # NEW — compute_period(), Vận boundary table, Lo Shu invariant validator
├── annual.rs       # NEW — compute_yearly_flying_stars(), Niên star formula
├── monthly.rs      # NEW — compute_monthly_flying_stars(), solar-term month resolver
└── combined.rs     # NEW — compute_combined_overlay(), CombinedFlyingStarLayout type
```

Data files:
```
crates/amlich-core/data/almanac/
├── flying_stars.json           # NEW — star metadata: name_vi, element, polarity, auspice per star 1-9
└── flying_stars_base.json      # NEW — base palace tables for Vận 1-9 (Lo Shu invariant-checked at load)
└── flying_stars_golden.json    # NEW — golden dataset ≥10 dates per Vận (FS-10)
```

Integration test:
```
crates/amlich-core/tests/
└── fengshui_invariants.rs      # NEW — black-box tests for FS-04, FS-05, FS-10
```

### Pattern 1: OnceLock JSON Loader (from `golden_loader.rs`)

All data loading follows the established pattern:

```rust
// Source: crates/amlich-core/src/almanac/golden_loader.rs:1-21
const FLYING_STARS_BASE_JSON: &str = include_str!("../../../data/almanac/flying_stars_base.json");
static FLYING_STARS_BASE: OnceLock<FlyingStarsBaseTable> = OnceLock::new();

pub fn load_flying_stars_base() -> &'static FlyingStarsBaseTable {
    FLYING_STARS_BASE.get_or_init(|| {
        let table: FlyingStarsBaseTable =
            serde_json::from_str(FLYING_STARS_BASE_JSON).expect("Failed to parse flying_stars_base.json");
        validate_lo_shu_invariants(&table);  // panics on invariant violation
        table
    })
}
```

Validation must check for each Vận: sum=45, each 1-9 appears exactly once, center = Vận number.

### Pattern 2: Evidence Envelope Attachment (from Phase 10 `types.rs`)

Per FS-09, each sub-star layer gets its own envelope. The composite carries a `rule.composite.*` envelope:

```rust
// Source: crates/amlich-core/src/almanac/fengshui/types.rs:128-135 (minimal_evidence pattern)
// Per-layer method strings:
fn van_evidence() -> ReasoningEvidenceEnvelope {
    ReasoningEvidenceEnvelope {
        source_family: ReasoningEvidenceSourceFamily::AlmanacRule,
        source_id: crate::sources::SOURCE_HUYEN_KHONG.to_string(),
        method: "phi_tinh.van".to_string(),
        note: Some(format!("van={van}")),
    }
}
// Composite: method = "rule.composite.flying_stars"
```

Three separate envelopes (Vận, Niên, Nguyệt) in `CombinedFlyingStarLayout`; each individual `FlyingStarLayout` carries its own single envelope per layer.

### Pattern 3: Tiết Khí Scanner for Vận/Annual Boundary

The Vận boundary and annual star anchor both use Lập Xuân, located at ecliptic longitude 315° (index 21 in `TIET_KHI` const array). The scanner returns `Vec<SolarTermWithDate>`:

```rust
// Source: crates/amlich-core/src/tietkhi.rs:227
// Lập Xuân is TIET_KHI index 21 (longitude 315°)
let terms = get_all_tiet_khi_for_year(year, 7.0);
let lap_xuan = terms.iter().find(|t| t.name == "Lập Xuân")
    .expect("Lập Xuân must exist for any year");
// lap_xuan.jd is the Julian Day of the Lập Xuân boundary
```

The scanner is day-granularity only (no sub-day resolution). The Vận boundary instant in the success criteria ("2024-02-04 16:27 ICT") is more precise than `jd`; however, the scanner's day-boundary is sufficient for `compute_period(year, ...)` where the input is a year integer. For sub-day boundary tests in the golden dataset, note that `jd_from_date` is also day-granular — the success criteria's "before/after 16:27" language is aspirational; the actual implementation should document that boundary resolution is day-granular unless the API takes a `DayContext` or `jd: i32` input.

**Important API decision for planners:** The success criteria signature is `compute_period(2024, &term_scanner)`. Since the scanner takes `year: i32`, the simplest correct implementation is:
- `compute_period(year: i32, ...)` returns the Vận active for **most of that year** (after Lập Xuân).
- If the caller needs the Vận for a specific day (e.g., 2024-01-15 = Vận 8, 2024-02-05 = Vận 9), the API should accept a `jd: i32` or `DayContext` parameter.
- The success criterion says `compute_period(2024, &term_scanner)` returns Vận 8 before Lập Xuân 2024 — which implies the API must accept an instant, not just a year. Recommend the planner define `compute_period(jd: i32, term_scanner_year: i32)` or similar to allow both before- and after-Lập-Xuân resolution within the same year.

### Pattern 4: Year Polarity via canchi.rs

```rust
// Source: crates/amlich-core/src/canchi.rs:103
// can_index = (lunar_year + 6) % 10
// Odd can_index (0-based 0..9): Giáp(0), Bính(2), Mậu(4), Canh(6), Nhâm(8) → dương
// Per ADR-0003: dương = odd can_index values (1-based indices 1,3,5,7,9 = Giáp,Bính,Mậu,Canh,Nhâm)
// NOTE: canchi.rs uses 0-based indexing; can_index 0 = Giáp (odd in 1-based = dương)
// Dương stems: can_index % 2 == 0 (Giáp=0, Bính=2, Mậu=4, Canh=6, Nhâm=8)
fn year_polarity(solar_year: i32) -> YearPolarity {
    let canchi = get_year_canchi(solar_year);
    if canchi.can_index % 2 == 0 { YearPolarity::Duong } else { YearPolarity::Am }
}
```

### Pattern 5: Monthly Star Group Rule (ADR-0002 §4)

Groups start at 8/5/2 and descend mod-9. The year's branch (Chi) determines the group:
- **Group 8**: Dần/Tỵ/Thân/Hợi years — month 1 (Dần) center starts at 8
- **Group 5**: Mão/Ngọ/Dậu/Tý years — month 1 (Dần) center starts at 5
- **Group 2**: Thìn/Mùi/Tuất/Sửu years — month 1 (Dần) center starts at 2

Then for each subsequent solar month, center descends: `next_center = ((current - 1 - 1 + 9) % 9) + 1` (always descending mod-9, wrapping 1→9).

The 12 solar-month boundary terms (from ADR-0002 §3) are at specific ecliptic longitudes. Use `get_all_tiet_khi_for_year` to find which solar month a given JD falls in, then apply the group rule to get the monthly center star.

### Anti-Patterns to Avoid

- **Naïve `year >= 2024` for Vận 9**: Always use Lập Xuân scan. Forbidden by PITFALLS CRIT-2 and project state.
- **Hardcoded Rust `const` arrays for base palace tables**: Use JSON + load-time invariant check per PITFALLS CRIT-4. Exception: the computation formula itself (center star formula) may be a pure function since it's mathematically derived.
- **Single `ReasoningEvidenceEnvelope` for the combined output**: Per FS-09, separate envelopes per sub-star layer. One flat envelope violates PITFALLS MOD-5.
- **Wiring `FlyingStar` into `direction_merge.rs`**: Explicitly forbidden by PITFALLS CRIT-3, ADR boundary comment in `mod.rs`, and ROADMAP cross-cutting constraints. Phase 15 handles graph wiring.
- **Bare string `"huyen-khong"` at call sites**: Must use `SOURCE_HUYEN_KHONG` constant. `source_id_guard.rs` CI test will fail otherwise.
- **`is_retrograde: bool` flag for Niên direction**: ADR-0003 requires a `(yuan, polarity) → (starting_star, direction)` matrix, not a single bool. See PITFALLS MOD-3.

---

## Phase 10 Deliverables Already In Place

These exist in the codebase and Phase 13 MUST NOT re-implement or modify them:

| Deliverable | Location | Phase 13 Usage |
|---|---|---|
| `Palace` enum (N=1..S=9, `Palace::ALL`, `palace_to_direction()`) | `almanac/fengshui/types.rs:17-62` | Use directly — array indexing convention: index i = `Palace::ALL[i]` = Lo Shu position i+1 |
| `FlyingStar` enum (NhatBach=1..CuuTu=9) | `almanac/fengshui/types.rs:72-91` | Use directly — `FlyingStar::NhatBach as u8 == 1` confirmed |
| `FlyingStarPeriod` enum (Van/Yearly/Monthly) | `almanac/fengshui/types.rs:98-104` | Use directly — serde round-trip confirmed in Phase 10 tests |
| `FlyingStarLayout` struct (period, palaces, center_star, evidence) | `almanac/fengshui/types.rs:119-125` | Phase 13 fills this struct; field set is FROZEN |
| `minimal_evidence()` helper | `almanac/fengshui/types.rs:128-135` | Pattern reference only — Phase 13 replaces stubs with real evidence |
| `SOURCE_HUYEN_KHONG` constant | `sources.rs:26` | Use at every `source_id` assignment |
| ADR-0002 (monthly anchor) | `.planning/adrs/0002-phi-tinh-monthly-anchor.md` | Locked — solar-term boundaries per `get_all_tiet_khi_for_year` |
| ADR-0003 (Niên polarity matrix) | `.planning/adrs/0003-nien-tu-bach-polarity.md` | Locked — Hạ Nguyên (starting star 7) HIGH confidence; Thượng/Trung rows MEDIUM, Phase 13 must cross-validate |

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---|---|---|---|
| Lập Xuân instant for year N | New solar-term scanner | `tietkhi::get_all_tiet_khi_for_year(year, 7.0)` | v1.1.2 fix already proven; ADR-0002/0003 mandate reuse |
| Year Heavenly Stem + polarity | Year-to-stem lookup | `canchi::get_year_canchi(year).can_index` | Already implemented, verified for 2023-2025 |
| Serde JSON loading with validation | Custom parser | `OnceLock` + `include_str!` + `serde_json::from_str` + validation fn | Project canonical pattern (golden_loader.rs, corpus.rs) |
| Evidence envelope construction | New evidence type | `ReasoningEvidenceEnvelope { source_family: AlmanacRule, source_id: SOURCE_HUYEN_KHONG, method: "...", note: None }` | Exact struct from `reasoning/types.rs:145-151` |
| Lo Shu invariant arithmetic | Manual per-Vận assertions | Parametric validator: for each vận, assert sum==45, all 1-9 present, center==vận | One function, reused for all 9 Vận rows |

---

## Common Pitfalls

### Pitfall 1: Vận Period Boundary Off-By-One (CRIT-2)
**What goes wrong:** `year >= 2024 → Vận 9` misclassifies Jan/early-Feb 2024 as Vận 9.
**Why it happens:** "Vận 9 = 2024-2043" is a slogan, not an algorithm.
**How to avoid:** `compute_period` must call `get_all_tiet_khi_for_year` to locate Lập Xuân JD; compare query JD against that boundary.
**Warning signs:** Test passes for 2024-03-01 but fails for 2024-01-15.

### Pitfall 2: Base Palace Table Typo (CRIT-4)
**What goes wrong:** One transposition in the 3×3 Lo Shu table silently corrupts all derived stars.
**Why it happens:** Manual data entry; no algorithmic generation catches the typo.
**How to avoid:** Store in JSON, validate at load (sum=45, each 1-9 once, center=Vận). Do not test table against itself — test against multi-source cross-check.
**Warning signs:** A Vận layout where center_star ≠ Vận number.

### Pitfall 3: Single Evidence Envelope on Combined Output (MOD-5)
**What goes wrong:** One `ReasoningEvidenceEnvelope` for the entire `CombinedFlyingStarLayout` hides which sub-star came from which rule.
**Why it happens:** Easy implementation uses `minimal_evidence()` once.
**How to avoid:** `CombinedFlyingStarLayout` carries `van_evidence`, `nien_evidence`, `nguyet_evidence`, and `composite_evidence` (with `method: "rule.composite.flying_stars"`).
**Warning signs:** A test asserting `evidence.method == "phi_tinh.nien"` fails.

### Pitfall 4: Niên Direction as Bool Flag (MOD-3)
**What goes wrong:** `is_retrograde: bool` hard-codes one Yuan's rule across all Yuans.
**Why it happens:** Reading only contemporary-era (Hạ Nguyên) sources.
**How to avoid:** Implement the `(yuan, year_polarity) -> (starting_star, direction)` matrix table from ADR-0003. For practical use (2024-2043 all Hạ Nguyên), the Hạ Nguyên row always applies — but the matrix design prevents the bug from appearing when historical dates are queried.
**Warning signs:** Pre-1984 dates return wrong yearly star.

### Pitfall 5: Phi Tinh Contaminating KHCBPPT Direction Modules (CRIT-3)
**What goes wrong:** Flying star outputs get merged into `direction_merge.rs`.
**Why it happens:** Both emit direction-adjacent content.
**How to avoid:** `almanac/fengshui/` only exports `FlyingStarLayout`; NEVER calls or is called by `direction_merge.rs`. Phase 15 handles graph wiring.
**Warning signs:** Any `use` statement in fengshui that imports from `interaction/`.

### Pitfall 6: Solar Month vs Lunar Month for Monthly Stars (MOD-2)
**What goes wrong:** Using lunar month boundaries instead of solar-term month boundaries.
**Why it happens:** "Month 1 = Dần" is written ambiguously in some Vietnamese sources.
**How to avoid:** ADR-0002 explicitly locks solar-term boundaries. The 12 opening terms are at specific ecliptic longitudes (Lập Xuân=315°, Kinh Trập=345°, etc.). Use `get_all_tiet_khi_for_year` to find boundaries.
**Warning signs:** Monthly star differs from reference for any date near a solar-term month boundary.

---

## Code Examples

### Lo Shu Invariant Validator
```rust
// Source pattern: crates/amlich-core/src/almanac/golden_loader.rs:153-186
fn validate_van_table(van: u8, palaces: &[u8; 9]) {
    let sum: u32 = palaces.iter().map(|&s| s as u32).sum();
    assert_eq!(sum, 45, "Vận {van} Lo Shu sum must be 45, got {sum}");
    let mut seen = [false; 10];
    for &s in palaces {
        assert!(s >= 1 && s <= 9, "Vận {van} star {s} out of range 1-9");
        assert!(!seen[s as usize], "Vận {van} star {s} appears twice");
        seen[s as usize] = true;
    }
    // Center = palaces[4] (index of Palace::Center = 5 - 1 = 4 in Palace::ALL order)
    assert_eq!(palaces[4], van, "Vận {van} center palace must equal Vận number, got {}", palaces[4]);
}
```

### Evidence Envelope Per Layer
```rust
// Source pattern: crates/amlich-core/src/almanac/fengshui/types.rs:128-135
// Source: crates/amlich-core/src/reasoning/types.rs:144-151
fn van_evidence(van: u8) -> ReasoningEvidenceEnvelope {
    ReasoningEvidenceEnvelope {
        source_family: ReasoningEvidenceSourceFamily::AlmanacRule,
        source_id: crate::sources::SOURCE_HUYEN_KHONG.to_string(),
        method: "phi_tinh.van".to_string(),
        note: Some(format!("van={van}")),
    }
}
fn composite_evidence() -> ReasoningEvidenceEnvelope {
    ReasoningEvidenceEnvelope {
        source_family: ReasoningEvidenceSourceFamily::AlmanacRule,
        source_id: crate::sources::SOURCE_HUYEN_KHONG.to_string(),
        method: "rule.composite.flying_stars".to_string(),
        note: None,
    }
}
```

### Lập Xuân Boundary Resolution
```rust
// Source: crates/amlich-core/src/tietkhi.rs:227
// Lập Xuân = TIET_KHI[21], longitude 315°
fn find_lap_xuan_jd(year: i32) -> i32 {
    let terms = get_all_tiet_khi_for_year(year, 7.0);
    terms.iter()
        .find(|t| t.name == "Lập Xuân")
        .map(|t| t.jd)
        .unwrap_or_else(|| panic!("Lập Xuân not found for year {year}"))
}
```

### CombinedFlyingStarLayout Type (new in Phase 13)
```rust
// New type — per FS-08
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombinedFlyingStarLayout {
    pub year: i32,
    pub month: u8,                             // solar-term month number (1-12)
    /// palace_overlays[i] = (annual_star, monthly_star) for Palace::ALL[i]
    pub palace_overlays: [(FlyingStar, FlyingStar); 9],
    pub annual_layout: FlyingStarLayout,
    pub monthly_layout: FlyingStarLayout,
    pub van_layout: FlyingStarLayout,
    pub evidence: ReasoningEvidenceEnvelope,   // composite evidence
}
```

---

## Open Questions

1. **`compute_period` signature: year vs JD**
   - What we know: success criterion says `compute_period(2024, &term_scanner)` returns Vận 8 before Lập Xuân and Vận 9 after.
   - What's unclear: a `year: i32` parameter alone cannot express "before Lập Xuân 2024" — either the function must also take a `jd: i32` or the two boundary cases must be split into `compute_period_for_jd(jd, year)`.
   - Recommendation: Define `compute_period(jd: i32, year: i32, ...) -> Period` where `year` is the solar year used to look up the Lập Xuân JD, OR make `compute_period(jd: i32, ...)` and derive year internally from `jd_to_date`. Either way, the scanner call takes `jd_to_date(jd).2` (the year) to look up Lập Xuân.

2. **`KnownDivergence` type definition**
   - What we know: PITFALLS and ROADMAP reference `KnownDivergence` as a golden dataset annotation type.
   - What's unclear: Is it a Rust struct in the golden dataset loader, or a JSON field? It does not exist in the codebase yet.
   - Recommendation: Define as a Rust struct alongside the golden dataset loader — `KnownDivergence { date: String, our_value: u8, source_a_value: u8, source_b_value: u8, tiebreaker: String, note: String }`. Mirror it in the JSON golden file with a `known_divergences: []` array.

3. **Thượng/Trung Nguyên cross-validation (ADR-0003 MEDIUM confidence)**
   - What we know: ADR-0003 explicitly states that Phase 13 must cross-check Thượng/Trung Nguyên rows against *Thẩm Thị Huyền Không Học*. The current rows are sourced from a single Vietnamese website.
   - What's unclear: Whether the cross-validation will confirm or require correction.
   - Recommendation: Include at least 2 Thượng/Trung Nguyên dates in the golden dataset (e.g., 1920 = Vận 3 Thượng Nguyên, 1960 = Vận 6 Trung Nguyên) with explicit `known_divergences` entries if sources disagree. If *Thẩm Thị Huyền Không Học* tables are inaccessible, mark the Thượng/Trung rows as LOW confidence in evidence envelopes and log a `KnownDivergence`.

4. **`term_scanner` parameter type in public API signatures**
   - The success criteria write `compute_period(2024, &term_scanner)` and `compute_yearly_flying_stars(year, term_scanner)`.
   - The existing scanner is a free function (`get_all_tiet_khi_for_year`), not a struct with a `&self` interface.
   - Recommendation: The `term_scanner` parameter is likely just `i32` (the year for scanner context) or may be elided entirely if the function derives the year from JD. The planner should decide whether to introduce a thin `TietKhiScanner` struct wrapping the free function (for testability/injection) or simply call `get_all_tiet_khi_for_year` directly inside the computation functions.

---

## State of the Art

| Old Approach | Current Approach | Impact for Phase 13 |
|---|---|---|
| Naïve `year >= 2024` for Vận boundary | `get_all_tiet_khi_for_year` Lập Xuân scan | Mandated by ADR-0003; scanner already in tietkhi.rs |
| `is_retrograde: bool` for Niên direction | `(yuan, year_polarity) → (starting_star, direction)` matrix | ADR-0003 defines the matrix; Phase 13 implements it |
| Civil calendar months for monthly stars | Solar-term month boundaries (tháng tiết khí) | ADR-0002 defines 12 opening terms; Phase 13 uses `get_all_tiet_khi_for_year` |
| Single evidence envelope per output | Separate envelopes per sub-star layer + composite | PITFALLS MOD-5 mitigation; required by FS-09 |

---

## Sources

### Primary (HIGH confidence)
- `crates/amlich-core/src/almanac/fengshui/types.rs` — Phase 10 frozen types (Palace, FlyingStar, FlyingStarLayout, FlyingStarPeriod) — verified current
- `crates/amlich-core/src/almanac/fengshui/mod.rs` — Phase 13 file list described in module comment
- `crates/amlich-core/src/tietkhi.rs:227` — `get_all_tiet_khi_for_year` API; `TIET_KHI[21]` = Lập Xuân at 315°
- `crates/amlich-core/src/canchi.rs:103` — `get_year_canchi(year)` returns `CanChi { can_index, ... }`
- `crates/amlich-core/src/reasoning/types.rs:144-151` — `ReasoningEvidenceEnvelope` struct definition
- `crates/amlich-core/src/reasoning/mod.rs` — public re-export path for evidence types
- `crates/amlich-core/src/sources.rs:26` — `SOURCE_HUYEN_KHONG = "huyen-khong"`
- `crates/amlich-core/src/almanac/golden_loader.rs:1-21,153-186` — OnceLock + validation canonical pattern
- `.planning/adrs/0002-phi-tinh-monthly-anchor.md` — solar-term month boundary rule (Accepted)
- `.planning/adrs/0003-nien-tu-bach-polarity.md` — Niên direction matrix, Hạ Nguyên HIGH / Thượng-Trung MEDIUM
- `.planning/research/PITFALLS.md` — CRIT-2, CRIT-3, CRIT-4, MOD-3, MOD-5 fully documented
- `.planning/research/ARCHITECTURE.md` — hybrid JSON/const table decision, module layout rationale
- `.planning/REQUIREMENTS.md` — FS-01 through FS-10 formal definitions
- `.planning/ROADMAP.md` — Phase 13 success criteria (5 items)

### Secondary (MEDIUM confidence)
- `.planning/STATE.md` — Known gaps: Thượng/Trung Nguyên polarity rows single-source, deferred cross-validation to Phase 13
- `.planning/research/SUMMARY.md` — "No new crate dependencies" policy confirmed
- `crates/amlich-core/Cargo.toml` — confirms `serde`, `serde_json`, `chrono` workspace deps; no flying-star-specific deps exist yet

---

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — no new deps, all existing patterns verified in source
- Architecture: HIGH — Phase 10 module comment explicitly lists Phase 13 files; types frozen
- Pitfalls: HIGH for CRIT-2/4/5, MOD-5 (anchored in codebase); MEDIUM for MOD-3 Thượng/Trung Nguyên rows (single-source, designated for Phase 13 cross-validation)
- Algorithm correctness: HIGH for Hạ Nguyên (2024-2043 practical era); MEDIUM for Thượng/Trung Nguyên (ADR-0003 §6 acknowledgment)

**Research date:** 2026-05-27
**Valid until:** 2026-06-27 (stable domain; ADRs are locked)
