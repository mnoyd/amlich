# Architecture Research

**Domain:** Vietnamese almanac correctness validation against KHCBPPT
**Researched:** 2026-02-28
**Confidence:** HIGH (codebase integration), MEDIUM (dataset design)

## Component Overview

### 1. Golden Dataset (`khcbppt-golden.json`)

**Purpose:** Machine-readable KHCBPPT reference data for automated validation

**Schema:** Fully typed `GoldenEntry` with fields for every subsystem:
- `stars`: cat_tinh, sat_tinh, day_star (28-star)
- `taboos`: rule_id array
- `day_deity`: name + classification
- `truc`: index, name, quality
- `xung_hop`: luc_xung, tam_hop, tu_hanh_xung
- `than_huong`: xuat_hanh_huong, tai_than, hy_than
- `na_am`: element name (if in scope)
- `khcbppt_ref`: citation per entry

**Location:** `crates/amlich-core/data/almanac/khcbppt-golden.json`

**Loading:** `include_str!` at compile time (matches existing `baseline.json` pattern)

### 2. Golden Loader (`golden_loader.rs`)

**Purpose:** Deserialize golden dataset into typed Rust structs

**Pattern:** Shared module imported by all validator test files

**Location:** `crates/amlich-core/tests/golden_loader.rs` (test-only module)

### 3. Subsystem Validators (`khcbppt_*.rs`)

**Purpose:** One test file per almanac subsystem comparing implementation output against golden data

**Files:**
- `tests/khcbppt_stars.rs` — star rules validation
- `tests/khcbppt_taboos.rs` — taboo rules validation
- `tests/khcbppt_deity.rs` — day deity validation
- `tests/khcbppt_truc.rs` — trực validation
- `tests/khcbppt_xung_hop.rs` — xung hợp validation
- `tests/khcbppt_than_huong.rs` — thần hướng validation
- `tests/khcbppt_na_am.rs` — nạp âm validation (if in scope)

**Pattern:** Each validator uses collect-all failure reporting (not early-exit assertions) so the full divergence scope is visible in a single test run.

### 4. Integration with Existing Test Infrastructure

- New files are **additive** — `cargo test --package amlich-core` discovers them automatically
- No `Cargo.toml` changes needed
- Existing `almanac_golden.rs`, `ruleset_determinism.rs`, `taboo_boundary.rs` remain untouched as regression guards
- New tests complement (not replace) existing internal consistency tests

## Data Flow

```
KHCBPPT classical text
    ↓ (manual extraction)
khcbppt-golden.json
    ↓ (include_str! + serde)
golden_loader.rs → GoldenEntry structs
    ↓ (consumed by)
khcbppt_*.rs validators
    ↓ (call)
get_day_info() / calculate_day_fortune()
    ↓ (compare)
Implementation output vs Golden reference
    ↓ (report)
Divergence list → Fix baseline.json / code
```

## Build Order

| Phase | Depends On | Produces |
|-------|-----------|----------|
| 1. Research KHCBPPT reference tables | Nothing | Raw reference tables per subsystem |
| 2. Compile golden dataset + loader | Phase 1 | `khcbppt-golden.json` + `golden_loader.rs` |
| 3. Write validators, surface divergences | Phase 2 | Divergence inventory per subsystem |
| 4. Fix baseline.json / code, re-run to zero divergences | Phase 3 | Corrected implementation |

**Enforcement:** Data dependencies enforce ordering — cannot write dataset without reference tables, cannot fix before full divergence inventory.

## Key Anti-Patterns to Avoid

- **Extending `almanac_golden.rs`** — conflates internal consistency with KHCBPPT correctness
- **Fixing before full audit** — hides total scope; must surface all divergences first
- **Iterating all 3,650 dates without reference values** — tests non-panic, not correctness
- **Mixing golden data with baseline.json** — golden dataset is the judge, baseline is the defendant

## Representative Date Selection

~200 dates should cover all pattern combinations for 2020–2030:
- All 12 chi × at least 1 date each
- All 10 can × at least 1 date each
- All 12 lunar months × at least 1 date each
- 28 JD-cycle positions × at least 1 date each
- Key dates: Tết, equinoxes, solstices, leap month dates
- Edge cases: month boundaries, tiết khí transitions

---
*Architecture research: 2026-02-28*
