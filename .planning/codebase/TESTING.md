# Testing Patterns

**Analysis Date:** 2026-02-28

## Test Framework

**Runner:**
- **Framework:** Built-in Rust `cargo test`
- **Config:** None (default Rust test harness)
- **Version:** Rust 2021 edition

**Assertion Library:**
- Standard library `assert!`, `assert_eq!`, `assert_ne!`
- Custom assertion messages for diagnostics

**Run Commands:**
```bash
# Run all tests
cargo test --workspace --exclude am-lich

# Run specific package tests
cargo test --package amlich-core

# Verbose output
cargo test --workspace --exclude am-lich -- --nocapture

# Run tests via justfile
just test
just test-core
```

**CI Pipeline:**
- GitHub Actions (`.github/workflows/ci.yml`)
- Runs: format check, data validation, clippy, tests
- Command: `cargo test --workspace --exclude am-lich`

## Test File Organization

**Location:**
- **Inline tests:** In `#[cfg(test)]` modules within each source file
- **Integration tests:** In `tests/` directory alongside `src/`
- **Package structure:**
  - `crates/amlich-core/tests/` - Core library integration tests
  - `crates/amlich-api/tests/` - API contract tests
  - `crates/amlich/tests/` - CLI contract tests

**Naming:**
- **Inline modules:** `mod tests` within source files
- **Integration test files:** `*_contract.rs`, `*_parity.rs`, `*_golden.rs`, `*_determinism.rs`

**Structure:**
```
crates/
├── amlich-core/
│   ├── src/
│   │   ├── lib.rs          # contains #[cfg(test)] mod tests
│   │   ├── lunar.rs         # contains #[cfg(test)] mod tests
│   │   └── ...
│   └── tests/
│       ├── almanac_golden.rs
│       ├── ruleset_determinism.rs
│       └── taboo_boundary.rs
└── amlich-api/
    ├── src/
    │   └── lib.rs          # contains #[cfg(test)] mod tests
    └── tests/
        ├── insight_parity.rs
        ├── almanac_contract.rs
        └── golden_parity.rs
```

## Test Structure

**Suite Organization:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specific_behavior() {
        // Arrange
        let input = /* ... */;

        // Act
        let result = function_under_test(input);

        // Assert
        assert_eq!(result.expected, actual);
    }
}
```

**Patterns:**
- **Setup:** Minimal - most functions are pure computation
- **Teardown:** Not needed (no shared state)
- **Assertion pattern:** `assert_eq!` for equality, `assert!` for boolean conditions

**Example from `/Users/noy/work/junks/amlich/crates/amlich-core/src/lib.rs`:**
```rust
#[test]
fn test_get_day_info_tet_2024() {
    let info = get_day_info(10, 2, 2024);
    assert_eq!(info.solar.day, 10);
    assert_eq!(info.lunar.day, 1);
    assert_eq!(info.lunar.year, 2024);
    assert!(!info.lunar.is_leap_month);
}
```

## Mocking

**Framework:** None - pure computation, no external dependencies

**Patterns:**
- No mocking needed for core calculations
- Test data hardcoded in test functions
- Use fixed dates for predictable results (e.g., Tết 2024, Tết 2025)

**What to Mock:**
- Nothing - codebase is pure functions with no I/O or external services

**What NOT to Mock:**
- Core calculation functions - test them directly
- Data tables - they're static `const` arrays

## Fixtures and Factories

**Test Data:**
- Hardcoded known dates used throughout
- Standard test dates:
  - Tết 2024: February 10, 2024 (1/1/2024 lunar)
  - Tết 2025: January 29, 2025 (1/1/2025 lunar)
  - Y2K: January 1, 2000 (25/11/1999 lunar)

**Location:**
- Inline in test functions
- No shared fixture modules

**Example from `/Users/noy/work/junks/amlich/crates/amlich-core/tests/almanac_golden.rs`:**
```rust
/// Tết 2024 (2024-02-10): Giáp Thìn, lunar 1/1/2024
/// chi_index=4, lunar_month=1 -> trực=Mãn(2,hung), lục_xung=Tuất, tam_hợp={Tý,Thìn,Thân}
#[test]
fn golden_tet_2024_truc_and_xung_hop() {
    let info = get_day_info(10, 2, 2024);
    // ... assertions based on known expected values
}
```

## Coverage

**Requirements:** No enforced coverage target

**View Coverage:**
- No coverage tool configured
- CI does not report coverage metrics

**Coverage approach:**
- Golden master tests for critical calculations
- Contract tests for API boundaries
- Determinism tests for reproducibility

## Test Types

**Unit Tests:**
- **Scope:** Individual functions and modules
- **Location:** Inline `#[cfg(test)]` modules
- **Approach:** Test specific calculations with known inputs/outputs
- **Example:**
  ```rust
  #[test]
  fn test_new_moon_calculation() {
      let nm = new_moon(0);
      assert!(nm > 2415000.0 && nm < 2416000.0);
  }
  ```

**Integration Tests:**
- **Scope:** Cross-module behavior and API contracts
- **Location:** `tests/` directory
- **Examples:**
  - `almanac_contract.rs` - Verifies API structure and fields
  - `insight_parity.rs` - Ensures data consistency between modules
  - `ruleset_determinism.rs` - Verifies reproducible outputs

**Golden Master Tests:**
- **Scope:** Verified outputs for known dates
- **Location:** `tests/almanac_golden.rs`
- **Purpose:** Catch regressions in complex calculations
- **Example:**
  ```rust
  #[test]
  fn golden_tet_2024_truc_and_xung_hop() {
      assert_eq!(fortune.truc.name, "Mãn");
      assert_eq!(fortune.xung_hop.luc_xung, "Tuất");
  }
  ```

**Contract Tests:**
- **Scope:** API stability and serialization
- **Location:** `tests/almanac_contract.rs`
- **Purpose:** Ensure DTOs maintain expected fields
- **Example:**
  ```rust
  #[test]
  fn day_info_exposes_day_fortune_contract() {
      let fortune = tet_2024_fortune();
      assert!(!fortune.day_element.na_am.is_empty());
      assert!(!fortune.conflict.tuoi_xung.is_empty());
  }
  ```

**E2E Tests:**
- Not used (library has no external interactions)

## Common Patterns

**Async Testing:**
- Not applicable (no async code)

**Error Testing:**
- Use `assert!(result.is_err())` or `assert!(result.is_ok())`
- Example from `/Users/noy/work/junks/amlich/crates/amlich-core/tests/ruleset_determinism.rs`:
  ```rust
  #[test]
  fn unknown_ruleset_id_returns_explicit_error() {
      let err = get_ruleset_data("not-a-ruleset").expect_err("unknown ruleset must fail");
      assert_eq!(err.to_string(), "unknown almanac ruleset id: not-a-ruleset");
  }
  ```

**Roundtrip Testing:**
- Test bidirectional conversions
- Example from `/Users/noy/work/junks/amlich/crates/amlich-core/src/lunar.rs`:
  ```rust
  #[test]
  fn test_roundtrip_conversion() {
      for (d, m, y) in test_dates {
          let lunar = convert_solar_to_lunar(d, m, y, 7.0);
          let (d2, m2, y2) = convert_lunar_to_solar(
              lunar.day, lunar.month, lunar.year, lunar.is_leap, 7.0
          );
          assert_eq!((d, m, y), (d2, m2, y2));
      }
  }
  ```

**Determinism Testing:**
- Verify same input produces same output
- Example from `/Users/noy/work/junks/amlich/crates/amlich-core/tests/ruleset_determinism.rs`:
  ```rust
  #[test]
  fn day_output_is_deterministic_for_same_input() {
      let a = get_day_info(10, 2, 2024);
      let b = get_day_info(10, 2, 2024);
      assert_eq!(a.ruleset_id, b.ruleset_id);
      assert_eq!(a.canchi.day.full, b.canchi.day.full);
  }
  ```

**Data Structure Validation:**
- Ensure data sets are complete and valid
- Example from `/Users/noy/work/junks/amlich/crates/amlich-api/tests/insight_parity.rs`:
  ```rust
  #[test]
  fn insight_datasets_have_expected_sizes() {
      assert_eq!(amlich_core::insight_data::all_can().len(), 10);
      assert_eq!(amlich_core::insight_data::all_chi().len(), 12);
      assert_eq!(amlich_core::insight_data::all_elements().len(), 5);
  }
  ```

**Custom Test Helpers:**
- Helper functions in test modules to reduce duplication
- Example from `/Users/noy/work/junks/amlich/crates/amlich-api/tests/almanac_contract.rs`:
  ```rust
  fn tet_2024_fortune() -> amlich_api::DayFortuneDto {
      let info = get_day_info(&DateQuery {
          day: 10, month: 2, year: 2024, timezone: Some(7.0)
      }).expect("day info should be available");
      info.day_fortune.expect("day_fortune should exist")
  }
  ```

## Test Naming Conventions

- **Test functions:** `test_<function>_<scenario>` or `golden_<scenario>`
- **Integration tests:** `<module>_<type>.rs` (e.g., `almanac_contract.rs`, `golden_parity.rs`)
- **Test names:** Descriptive, snake_case, explain what is being tested

## Test Data Management

- **Holiday data:** Validated via Node.js scripts in CI
- **Cross-language parity:** `check-holiday-parity.mjs` ensures Rust and JS data match
- **Embedded data:** Vendored into `amlich-core/src/data/` at build time

---

*Testing analysis: 2026-02-28*
