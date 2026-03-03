# Na Am Parity Decisions - v1.4

**Created:** 2026-03-04
**Phase:** 9 - Na Am API Surfaces and Contracts
**Status:** Complete

## Overview

Na Am (Nạp Âm) lookups expose the 60 sexagenary cycle combinations with their corresponding five-element classifications. This document records parity decisions and source ambiguities for the Na Am API implementation.

## Source Identification

**Primary Source:** Tâm Mệnh Thông Hội (Vietnamese almanac compilation)
- Source ID: `tam-menh-thong-hoi`
- Method: `table-lookup`
- Profile: `baseline`

**Source Note:** Na Am pairs follow the standard 60-cycle progression used in Vietnamese almanac calculations. The mapping of cycle positions to Na Am values is derived from the canonical sexagenary cycle tables.

## Na Am Data Structure

The 60 Na Am entries are stored in `AlmanacData.sexagenary_na_am` with the following structure:

```rust
pub struct NaAmEntry {
    pub can: String,      // Heavenly stem (e.g., "Giáp")
    pub chi: String,      // Earthly branch (e.g., "Tý")
    pub na_am: String,    // Na Am value (e.g., "Hải Trung Kim")
    pub element: String,  // Five element (last word of na_am, e.g., "Kim")
}
```

## Key Parity Decisions

### 1. Cycle Index Convention

**Decision:** Use 1-based cycle indices (1-60) to match Vietnamese convention.

**Rationale:**
- Vietnamese almanac references use 1-based numbering for cycle positions
- Consistent with Phase 8 sexagenary_cycle utilities
- API consumers expect 1-60 range, not 0-59

**Implementation:**
- Public API functions accept `u8` in range [1, 60]
- Internal arithmetic converts to 0-based for modulo operations
- Validation rejects indices outside [1, 60] with explicit error

### 2. Stem-Branch Pair Naming

**Decision:** Use Vietnamese stem and branch names in pair lookup.

**Rationale:**
- Vietnamese almanac data uses Vietnamese names (Giáp, Ất, Bính, Tý, Sửu, etc.)
- Data storage keys use Vietnamese names (e.g., "Giáp Tý")
- Internationalization handled at presentation layer, not core

**Implementation:**
- Pair lookup accepts Vietnamese names as strings
- Validation against CAN and CHI constant arrays
- Error returns "unknown_stem" or "unknown_branch" for invalid names

### 3. Canonical Combination Validation

**Decision:** Only validate canonical combinations (same polarity stems and branches).

**Rationale:**
- 60 of 120 possible stem/branch combinations are valid in sexagenary cycle
- Stems and branches must share polarity (both odd or both even)
- Prevents returning non-existent Na Am entries

**Implementation:**
- Use `sexagenary_cycle::canchi_to_cycle_index` for validation
- Returns `None` for non-canonical combinations
- API returns "invalid_stem_branch_pair" error for non-canonical input

### 4. Error Contract Determinism

**Decision:** Use explicit, deterministic error types for all validation failures.

**Rationale:**
- API consumers can programatically handle different error cases
- Consistent with DayFortune API error patterns
- Enables contract tests to verify error handling

**Implementation:**
- `invalid_cycle_index`: Index outside [1, 60] range
- `invalid_stem_branch_pair`: Non-canonical combination (odd/even mismatch)
- `unknown_stem`: Invalid stem name not in CAN array
- `unknown_branch`: Invalid branch name not in CHI array

### 5. Evidence Metadata Inclusion

**Decision:** Include source_id, method, and profile in every Na Am response.

**Rationale:**
- Traceability to source material (Tâm Mệnh Thông Hội)
- Method documentation for auditability
- Profile identification for multi-ruleset support (future)

**Implementation:**
- `NaAmLookupResultDto` includes source_id, method, profile fields
- Source data: `AlmanacData.na_am_meta` from almanac/data.rs
- Values: source_id="tam-menh-thong-hoi", method="table-lookup", profile="baseline"

### 6. Backward Compatibility Preservation

**Decision:** Na Am API is additive only, no changes to existing DayFortune API.

**Rationale:**
- Existing consumers of `get_day_info` and `DayFortune` must continue working
- Na Am API provides new standalone lookups
- Avoids breaking changes in v1.4 parity milestone

**Implementation:**
- New public functions: `get_na_am_by_index`, `get_na_am_by_pair`
- No modifications to existing DTOs or API functions
- Contract tests verify DayFortune API unchanged

## Known Source Ambiguities

### 1. Na Am Element Extraction Convention

**Ambiguity:** How to extract the five element from Na Am value (e.g., "Hải Trung Kim" → "Kim" or "Hải Trung Kim"?)

**Decision:** Extract last word of Na Am string as element.

**Rationale:**
- Na Am values follow pattern "[modifier] [modifier] [element]" (e.g., "Hải Trung Kim")
- Element is always the last word (Kim, Mộc, Thủy, Hỏa, Thổ)
- Simple, deterministic extraction: `na_am.split_whitespace().last()`

**Validation:** Contract tests verify element field matches last word of na_am field.

### 2. Roundtrip Conversion Consistency

**Ambiguity:** Does pair lookup return the same result as index lookup for the same position?

**Decision:** Yes, both lookup modes return identical data.

**Rationale:**
- Index → CanChi conversion uses `sexagenary_cycle::cycle_index_to_canchi`
- CanChi → Index conversion uses `sexagenary_cycle::canchi_to_cycle_index`
- Both lookups retrieve from same `sexagenary_na_am` HashMap
- Roundtrip consistency verified by contract tests

**Validation:** Contract tests iterate all 60 positions, verify index and pair lookups return identical cycle_index, can, chi, na_am, and element.

## Test Coverage

**Unit Tests (amlich-core):**
- 5 tests in `na_am.rs` module
  - get_na_am_by_index valid index
  - get_na_am_by_index invalid index
  - get_na_am_by_pair valid canonical pair
  - get_na_am_by_pair non-canonical pair
  - get_na_am_by_pair invalid stem/branch names

**Integration Tests (amlich-api):**
- 18 tests in `na_am_api_tests.rs`
  - Schema stability (index and pair lookup)
  - Serialization/deserialization
  - All 60 positions (index lookup)
  - Canonical pairs sampling (pair lookup)
  - Roundtrip conversion
  - Error contracts (all 4 error types)
  - Backward compatibility (DayFortune API)
  - Consistency between lookup modes

## Future Considerations

1. **Multi-Ruleset Support:** Current implementation uses "vn_baseline_v1" ruleset. Future versions may support multiple rulesets with different Na Am mappings.

2. **Internationalization:** Stem and branch names currently use Vietnamese. Future versions may add English or other language variants.

3. **Caching:** Na Am lookups retrieve from static HashMap. No caching layer needed, but may add if performance profiling shows value.

## Compliance

**v1.4 Requirements Satisfied:**
- [x] NAM-API-01: API exposes Na Am lookup by stem-branch pair
- [x] NAM-API-02: API exposes Na Am lookup by cycle index (1-60)
- [x] NAM-API-03: API returns normalized source metadata and method
- [x] NAM-API-04: API preserves backward compatibility
- [x] NAM-API-05: API returns explicit validation errors
- [x] NAM-API-06: Contract tests verify schema stability
- [x] PAR-03: Traceability links all requirements to Phase 9
- [x] PAR-04: Milestone artifact documents parity decisions

**Next Phase:** Phase 10 (if planned) or milestone completion.
