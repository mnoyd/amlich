# Phase 8: Sexagenary Cycle Parity and Validators - Research

**Researched:** 2026-03-03
**Domain:** Deterministic sexagenary 60-cycle conversion/progression utilities with full-table parity verification
**Confidence:** HIGH

## Summary

Phase 8 should introduce canonical sexagenary cycle utilities that provide bidirectional conversion between cycle index (1-60) and stem-branch pairs, forward/backward progression with modular correctness, and full-table parity validators. The codebase already contains partial 60-cycle infrastructure (`CanChi::sexagenary_index` in `types.rs`, `expand_sexagenary_na_am` in `data.rs`) but lacks a dedicated, reusable cycle utilities module with bounded contracts and progression invariants.

The key implementation risk is ensuring cycle indices use 1-based bounds (1-60) for user-facing APIs while maintaining 0-based internal arithmetic. The second key risk is modular progression correctness at rollover boundaries:
- Stem rollover: index 59 (Quý Hợi) → index 1 (Giáp Tý) wraps correctly across modulo 10
- Branch rollover: index 59 (Quý Hợi) → index 1 (Giáp Tý) wraps correctly across modulo 12
- Full 60-cycle rollover: index 60 → index 1 maintains consistent stem/branch advancement

The 30 `na_am_pairs` in `baseline.json` provide the canonical reference table (each entry maps to 2 consecutive cycle positions), which should drive parity validation against the generated 60-cycle mapping.

## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| SC-01 | Convert cycle index (1-60) to canonical stem-branch pair | Create `cycle_index_to_canchi(index: u8) -> CanChi` with 1-60 contract validation and error handling. |
| SC-02 | Convert stem-branch pair to cycle index (1-60) | Create `canchi_to_cycle_index(can: &str, chi: &str) -> Result<u8, Error>` validating against 60 canonical combinations only. |
| SC-03 | Forward/backward progression preserves modular correctness across rollover boundaries | Implement `progress_cycle_index(index: u8, delta: i32) -> Result<u8, Error>` with `(index + delta - 1 + 60) % 60 + 1` semantics and verify at boundaries (1, 60, ±N). |
| SC-04 | Cycle utilities expose deterministic helpers reusable by hour pillar and Na Am APIs | Create dedicated module (recommended: `crates/amlich-core/src/almanac/sexagenary_cycle.rs`) exporting public functions; avoid duplication in `hour_pillar.rs` and `data.rs`. |
| SC-05 | Validation suite confirms full-table parity against canonical 60-cycle references | Add parity validator test comparing generated cycle against `na_am_pairs` baseline for all 60 entries; verify stem/branch/na_am/element alignment. |
| PAR-01 | Parity validators are added for hour pillar and full 60-cycle tables | Extend existing validator harness (`khcbptest_na_am.rs` pattern) with full-table coverage test; add regression tests for index normalization and invalid inputs. |

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust workspace | edition 2021 | Deterministic 60-cycle arithmetic | Existing core baseline, test infrastructure, and error handling patterns. |
| `amlich_core::types::{CanChi, CAN, CHI}` | in-repo | Canonical stem/branch constants and CanChi construction | Reuse existing index-based stem/branch representation. |
| `amlich_core::types::normalize_index` | in-repo | Modular arithmetic for stem/branch rollover | Already handles modulo correctly for Can Chi calculations. |
| `RuleEvidence` | in-repo | Metadata contract alignment for parity validation | Matches existing evidence conventions in day fortune and hour pillar modules. |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `cargo test` | workspace | Boundary and parity verification | Primary validation path for all 60-cycle invariants. |
| `baseline.json::na_am_pairs` | v1 | Canonical reference table (30 entries × 2 = 60 cycle positions) | Source of truth for full-table parity validation. |
| Existing `khcbptest_na_am.rs` validator | v1 | Golden dataset comparison pattern | Reuse test harness structure for new 60-cycle validator. |

## Architecture Patterns

### Recommended Project Structure
```
crates/amlich-core/src/almanac/
├── sexagenary_cycle.rs   # NEW: 60-cycle utilities (SC-01..05)
├── hour_pillar.rs         # EXISTING: consumes sexagenary helpers
└── data.rs                # EXISTING: expand_sexagenary_na_am can be refactored to use sexagenary helpers

crates/amlich-core/tests/
├── sexagenary_cycle_parity.rs  # NEW: full-table parity validator (PAR-01, SC-05)
└── khcbppt_na_am.rs            # EXISTING: extends to cover full 60-cycle
```

### Pattern 1: Bounded 1-Based Public Contract with 0-Based Internal Arithmetic
**What:** Public APIs accept/return 1-60 cycle indices for user clarity, but internal arithmetic uses 0-59 range for modulo correctness.

**When to use:** All cycle index public functions.

**Example:**
```rust
/// Convert 1-based cycle index to stem-branch pair
pub fn cycle_index_to_canchi(index: u8) -> Option<CanChi> {
    if !(1..=60).contains(&index) {
        return None;
    }
    let zero_based = (index - 1) as usize;
    let can_idx = zero_based % 10;
    let chi_idx = zero_based % 12;
    Some(CanChi::new(can_idx, chi_idx))
}

/// Forward/backward progression with bounded 1-based result
pub fn progress_cycle_index(index: u8, delta: i32) -> Option<u8> {
    if !(1..=60).contains(&index) {
        return None;
    }
    let zero_based = (index - 1) as i32;
    let progressed = ((zero_based + delta).rem_euclid(60)) as u8;
    Some(progressed + 1)
}
```

### Pattern 2: Validation-First Inversion Function
**What:** `canchi_to_cycle_index` validates against canonical 60 combinations only (all `can_index % 2 == chi_index % 2`) and rejects invalid stem/branch pairs with explicit error.

**When to use:** All inverse lookups requiring parity verification.

**Example:**
```rust
pub fn canchi_to_cycle_index(can_index: usize, chi_index: usize) -> Option<u8> {
    // Validate canonical combination: same parity (odd/even)
    if can_index % 2 != chi_index % 2 {
        return None;
    }

    // Compute 60-cycle position
    let zero_based = ((can_index * 6) + (chi_index / 2)) % 60;
    Some((zero_based + 1) as u8)
}
```

### Pattern 3: Parity Validator Table Drive
**What:** Validator tests iterate over baseline reference data (30 `na_am_pairs`) and compare generated cycle output against expected values for all 60 positions.

**When to use:** SC-05 and PAR-01 full-table validation.

**Example:**
```rust
#[test]
fn validate_full_60_cycle_parity() {
    let data = baseline_data();
    let na_am_pairs = &data.na_am_pairs; // 30 entries

    for (zero_based_index, na_am) in na_am_pairs.iter().enumerate() {
        for offset in 0..2 {
            let cycle_index = (zero_based_index * 2 + offset) as u8 + 1;
            let canchi = cycle_index_to_canchi(cycle_index).expect("valid index");

            // Verify against expanded baseline data
            let expected_key = format!("{} {}", canchi.can, canchi.chi);
            let entry = data.sexagenary_na_am.get(&expected_key)
                .expect("key should exist in baseline");

            assert_eq!(entry.na_am, *na_am);
            assert_eq!(entry.can, canchi.can);
            assert_eq!(entry.chi, canchi.chi);
        }
    }
}
```

### Anti-Patterns to Avoid
- **Direct string-to-index lookups** for cycle arithmetic: fragile; use existing `CAN`/`CHI` arrays and `normalize_index`.
- **0-based public APIs** for cycle indices: user expects 1-60; expose 1-based, convert internally.
- **Skipping parity validation** in `canchi_to_cycle_index`: must reject non-canonical combinations (odd/even mismatch).
- **Duplicating 60-cycle logic** in `hour_pillar.rs` or `data.rs`: centralize in `sexagenary_cycle.rs`.
- **Missing evidence metadata** in validator tests: include `RuleEvidence` alignment assertions.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Stem/branch index arithmetic | Custom modulo formulas | `normalize_index` from `types.rs` | Handles negative deltas and boundary rollovers correctly. |
| 60-cycle progression math | Manual `(index + delta) % 60` | `rem_euclid` for signed arithmetic | Correctly handles negative progression (backward steps). |
| Na Am element extraction | String parsing on `na_am_pairs` | Existing `expand_sexagenary_na_am` in `data.rs` | Already extracts element field and builds 60-entry map. |
| Validator harness structure | Custom test framework | Existing `khcbppt_na_am.rs` pattern | Provides divergence reporting and fixture-driven validation. |

**Key insight:** The codebase already has modular arithmetic helpers and parity validation patterns; reusing them reduces edge-case risk and maintains consistency with existing features.

## Common Pitfalls

### Pitfall 1: 0-Based vs 1-Based Index Confusion
**What goes wrong:** Public API returns 0-based indices (0-59) but callers expect 1-based (1-60), or internal arithmetic uses 1-based causing off-by-one errors.

**Why it happens:** Rust arrays are 0-based but Vietnamese 60-cycle convention is 1-based (Giáp Tý = 1, Quý Hợi = 60).

**How to avoid:**
- Public functions: enforce 1-60 bounds and return 1-60 values
- Internal arithmetic: convert to 0-based immediately after validation
- Document contract clearly in docstrings with examples

**Warning signs:** Tests expecting `cycle_index_to_canchi(0)` to succeed, or progression wrapping at 59→0 instead of 60→1.

### Pitfall 2: Non-Canonical Pair Acceptance
**What goes wrong:** `canchi_to_cycle_index` returns an index for invalid pairs like "Giáp Sửu" (odd/even mismatch), breaking parity with baseline.

**Why it happens:** Only 60 of 120 possible stem/branch combinations (10×12) are valid in the sexagenary cycle (stems and branches must share polarity: both odd or both even).

**How to avoid:**
- Validate `can_index % 2 == chi_index % 2` before computing index
- Return `None` or explicit error for invalid pairs
- Add regression test verifying all 60 invalid pairs are rejected

**Warning signs:** Validator test passes but includes non-canonical combinations, or `expand_sexagenary_na_am` generates more than 60 entries.

### Pitfall 3: Boundary Rollover Incorrectness
**What goes wrong:** Forward progression from index 60 returns 0 instead of 1, or backward from 1 returns 60 correctly but stem/branch don't advance correctly.

**Why it happens:** Using `% 60` instead of `rem_euclid(60)` for signed arithmetic, or forgetting to add 1 after internal 0-based computation.

**How to avoid:**
- Use `rem_euclid(60)` for signed delta handling
- Test boundaries explicitly: `progress_cycle_index(1, -1)` should return 60, `progress_cycle_index(60, 1)` should return 1
- Verify stem and branch advance independently at boundaries

**Warning signs:** Progression tests fail at edges, or parity validator shows divergence at index 60.

### Pitfall 4: Missing Full-Table Validation
**What goes wrong:** Validators test only a handful of "representative" cases but miss mismatches in other entries, leading to undetected parity drift.

**Why it happens:** Manual fixture selection misses edge cases; baseline `na_am_pairs` has 30 entries that expand to 60 cycle positions.

**How to avoid:**
- Write validator to iterate over all 60 positions (not just samples)
- Compare against `sexagenary_na_am` map generated from baseline
- Run validator with `--nocapture` to see full divergence report

**Warning signs:** Test coverage is <60 entries, or validator only checks 5-10 "sample" cases.

### Pitfall 5: Duplicated Logic Across Modules
**What goes wrong:** `hour_pillar.rs` implements its own cycle progression, `data.rs` has inline 60-cycle math, causing inconsistency.

**Why it happens:** No central module for 60-cycle utilities; each feature builds local helpers.

**How to avoid:**
- Create `sexagenary_cycle.rs` as single source of truth
- Export public functions for hour pillar and Na Am lookup
- Refactor existing inline logic to use new utilities

**Warning signs:** Multiple modules compute cycle indices independently, or search for `sexagenary_index` finds duplicated code.

## Code Examples

Verified patterns from existing codebase:

### Canonical 60-Cycle Index Computation
```rust
// Source: types.rs CanChi::new() lines 91-92
let sexagenary_index = ((can_idx * 6) + (chi_idx / 2)) % 60;
```

### Na Am Pair Expansion to 60-Entry Map
```rust
// Source: data.rs expand_sexagenary_na_am() lines 872-890
fn expand_sexagenary_na_am(na_am_pairs: &[String]) -> HashMap<String, NaAmEntry> {
    let mut out = HashMap::with_capacity(60);
    for i in 0..60 {
        let can = CAN[i % 10];
        let chi = CHI[i % 12];
        let na_am = na_am_pairs[i / 2].clone(); // 30 pairs × 2 consecutive positions
        let element = na_am.split_whitespace().last().unwrap_or("").to_string();
        out.insert(format!("{can} {chi}"), NaAmEntry { can, chi, na_am, element });
    }
    out
}
```

### Normalize Index with Signed Handling
```rust
// Source: types.rs normalize_index() lines 124-126
pub fn normalize_index(value: i32, modulo: i32) -> usize {
    (((value % modulo) + modulo) % modulo) as usize
}
```

### Validator Divergence Reporting Pattern
```rust
// Source: khcbppt_na_am.rs validate_na_am_against_golden() lines 19-57
fn validate_na_am_against_golden() {
    let dataset = load_golden_dataset();
    let mut mismatches: Vec<String> = Vec::new();

    for entry in &dataset.entries {
        let info = get_day_info(entry.solar_day, entry.solar_month, entry.solar_year);

        if day_element.na_am != entry.expected_na_am {
            mismatches.push(format!("[{}] na_am: expected '{}', got '{}'", ...));
        }
    }

    if !mismatches.is_empty() {
        eprintln!("\n=== DIVERGENCE REPORT ({} mismatches) ===", mismatches.len());
        for m in &mismatches { eprintln!("  {m}"); }
    }
    assert!(mismatches.is_empty(), "Found {} divergence(s)", mismatches.len());
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Inline 60-cycle math in multiple modules | Centralized `sexagenary_cycle.rs` utilities | Phase 8 (planned) | Reduces duplication, improves testability |
| Sample-based parity validation | Full-table 60-entry validator | Phase 8 (planned) | Catches edge-case drift, aligns with PAR-01 |
| 0-based internal cycle indices exposed publicly | 1-based public contract, 0-based internal | Phase 8 (planned) | Matches Vietnamese convention, improves UX |

**Deprecated/outdated:**
- None identified; existing CanChi implementation is sound for core use cases.

## Open Questions

1. **Should `sexagenary_cycle.rs` be placed in `amlich-core/src/almanac/` or top-level `amlich-core/src/`?**
   - What we know: Hour pillar and Na Am lookup are in `almanac/`; `canchi.rs` is top-level.
   - What's unclear: Placement affects import paths and module organization conventions.
   - Recommendation: Place in `almanac/` to group with other lookup utilities; re-export from lib.rs if needed for simpler paths.

2. **Should existing `expand_sexagenary_na_am` in `data.rs` be refactored to use new utilities?**
   - What we know: Current implementation generates correct 60-entry map; new utilities will duplicate some logic.
   - What's unclear: Whether refactoring is in-scope for Phase 8 or deferred to tech debt cleanup.
   - Recommendation: Keep existing implementation unchanged in Phase 8 (stability), add TODO comment for future refactoring to use `sexagenary_cycle` helpers.

## Sources

### Primary (HIGH confidence)
- `crates/amlich-core/src/types.rs` - CanChi::new() sexagenary_index formula (line 92)
- `crates/amlich-core/src/types.rs` - normalize_index() function (lines 124-126)
- `crates/amlich-core/src/almanac/data.rs` - expand_sexagenary_na_am() 60-entry generation (lines 872-890)
- `crates/amlich-core/src/almanac/data.rs` - NaAmEntry struct and baseline loading (lines 49-54, 191, 402)
- `crates/amlich-core/data/almanac/baseline.json` - na_am_pairs canonical 30 entries (lines extracted from full JSON)

### Secondary (MEDIUM confidence)
- `crates/amlich-core/tests/khcbppt_na_am.rs` - Validator divergence reporting pattern (lines 19-57)
- `crates/amlich-core/tests/hour_pillar_parity.rs` - Fixture matrix and boundary test patterns (lines 14-70)
- `.planning/REQUIREMENTS.md` - SC-01..05 and PAR-01 requirement definitions
- `.planning/ROADMAP.md` - Phase 8 success criteria and plan structure

### Tertiary (LOW confidence)
- None; all research verified from codebase or official documentation.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Rust workspace and in-repo types are well-established.
- Architecture: HIGH - Existing CanChi implementation provides proven pattern; module placement follows almanac conventions.
- Pitfalls: HIGH - Identified from boundary cases in hour pillar and parity validation patterns.

**Research date:** 2026-03-03
**Valid until:** 2026-04-02 (30 days - stable mathematical contracts)
