# Coding Conventions

**Analysis Date:** 2026-02-28

## Naming Patterns

**Files:**
- **Rust source files:** `snake_case.rs` (e.g., `types.rs`, `canchi.rs`, `gio_hoang_dao.rs`)
- **Test files:** Same as source with inline tests in `#[cfg(test)]` modules, or separate tests in `tests/` directory
- **Module directories:** `snake_case` (e.g., `almanac/` module with submodules like `calc.rs`, `types.rs`)

**Functions:**
- **Public functions:** `snake_case` with descriptive names (e.g., `get_day_info`, `convert_solar_to_lunar`, `get_tiet_khi`)
- **Private functions:** `snake_case`
- **Constructor functions:** `new()` for structs, or `from()` for conversion traits

**Variables:**
- **Local variables:** `snake_case` (e.g., `lunar_year`, `day_number`, `sun_long_deg`)
- **Constants:** `SCREAMING_SNAKE_CASE` for module-level constants (e.g., `VIETNAM_TIMEZONE`, `CAN`, `CHI`, `TIET_KHI`)

**Types:**
- **Structs:** `PascalCase` (e.g., `CanChi`, `LunarDate`, `DayInfo`, `SolarTerm`)
- **Enums:** `PascalCase` (e.g., `StarQuality::Cat`, `DayDeityClassification::HoangDao`)
- **Type aliases:** `PascalCase`

## Code Style

**Formatting:**
- **Tool:** `rustfmt` (checked in CI via `cargo fmt --all -- --check`)
- **Edition:** Rust 2021
- **Max line length:** Default (100 characters typically)
- **Indent:** 4 spaces

**Linting:**
- **Tool:** `clippy` with `-D warnings` (fail on warnings)
- **CI command:** `cargo clippy --workspace --exclude am-lich -- -D warnings`

**Documentation:**
- **Module-level docs:** `//!` comments at top of files explaining purpose
- **Function docs:** `///` with `# Arguments` and `# Returns` sections for public functions
- **Example usage:** `# Example` code blocks in doc comments

## Import Organization

**Order:**
1. Standard library imports (`use std::...`)
2. External crate imports (`use serde::...`)
3. Workspace dependencies (`use crate::...` for internal modules)
4. Re-exports (`pub use ...`)

**Example from `/Users/noy/work/junks/amlich/crates/amlich-core/src/lib.rs`:**
```rust
use crate::almanac::calc::calculate_day_fortune;
use crate::almanac::types::DayFortune;
use canchi::{get_day_canchi, get_month_canchi, get_year_canchi};
```

**Path Aliases:**
- None used - full crate paths are explicit
- Internal re-exports used at lib.rs level: `pub use types::*;`

## Error Handling

**Patterns:**
- **Core library:** Functions return values directly (no `Result` for most core calculations)
- **API layer:** Functions return `Result<T, String>` for validation errors
- **Parse/conversion errors:** Return `(0, 0, 0)` tuples or `Option<T>` for invalid inputs

**Validation example from `/Users/noy/work/junks/amlich/crates/amlich-api/src/lib.rs`:**
```rust
pub fn get_day_info(query: &DateQuery) -> Result<DayInfoDto, String> {
    if !(1..=12).contains(&query.month) {
        return Err("month must be 1-12".to_string());
    }
    if !(1..=31).contains(&query.day) {
        return Err("day must be 1-31".to_string());
    }
    // ...
}
```

**Error strings:** Descriptive messages, not error types - simple `String` errors

## Logging

**Framework:** No logging framework used currently (console-only)
- Debug output via test assertions (`assert_eq!`, `assert!`)
- CI tests run with `cargo test` without special output capture

**Patterns:**
- Tests use `assert!` with custom messages for diagnostics
- No production logging - library is pure computation

## Comments

**When to Comment:**
- **Module headers:** Always - explain purpose and context
- **Complex algorithms:** Yes - reference sources (e.g., "Based on implementation by Ho Ngoc Duc")
- **Constants:** Yes - explain meaning
- **Trivial code:** No

**Example from `/Users/noy/work/junks/amlich/crates/amlich-core/src/lunar.rs`:**
```rust
/**
 * Lunar Calendar Conversion
 *
 * Algorithms from "Astronomical Algorithms" by Jean Meeus, 1998
 * Based on implementation by Ho Ngoc Duc
 */
```

**JSDoc/TSDoc:** Not applicable (Rust codebase)

## Function Design

**Size:**
- Most functions under 50 lines
- Complex calculations broken into smaller helper functions
- Large data tables in separate modules (e.g., `almanac/data.rs` at 1199 lines)

**Parameters:**
- Use individual primitive parameters for simple functions (e.g., `day: i32, month: i32, year: i32`)
- Use struct parameters for complex queries (e.g., `query: &DateQuery`)
- References (`&T`) for read-only large structs

**Return Values:**
- **Simple values:** Direct return (e.g., `i32`, `f64`, `String`)
- **Multi-value:** Tuple returns (e.g., `(i32, i32, i32)` for day/month/year)
- **Complex data:** Struct returns (e.g., `DayInfo`, `LunarDate`)
- **Optional:** `Option<T>` when value may not exist
- **API layer:** `Result<T, String>` for fallible operations

## Module Design

**Exports:**
- **Public API:** Export via `pub fn`, `pub struct`
- **Internal details:** Private (`fn` without `pub`)
- **Re-exports:** `pub use` in lib.rs for convenience

**Barrel Files:**
- Each crate has a `lib.rs` or `mod.rs` that re-exports common types
- Example from `/Users/noy/work/junks/amlich/crates/amlich-core/src/lib.rs`:
  ```rust
  pub mod almanac;
  pub mod canchi;
  // ...
  pub use types::*;  // Re-export main types
  ```

**Workspace structure:**
- Shared dependencies in workspace `Cargo.toml`
- Version managed via `version.workspace = true`
- Common dependencies shared: `serde`, `serde_json`, `chrono`

## Serde Serialization

**Pattern:** Use `#[derive(Serialize, Deserialize)]` on DTOs
- Core types use `#[derive(Clone, PartialEq, Debug)]` (no serialization)
- API layer DTOs add serialization derives
- Conversion via `impl From<&CoreType> for Dto`

**Example from `/Users/noy/work/junks/amlich/crates/amlich-api/src/dto.rs`:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayInfoDto {
    pub ruleset_id: String,
    pub solar: SolarDto,
    // ...
}
```

## Constants and Static Data

**Pattern:** Module-level `const` arrays for lookup tables
- `const` arrays with `&str` data (Vietnamese text)
- Fixed-size arrays with explicit lengths
- Compile-time data embedding (no runtime file I/O)

**Example from `/Users/noy/work/junks/amlich/crates/amlich-core/src/types.rs`:**
```rust
pub const CAN: [&str; 10] = [
    "Giáp", "Ất", "Bính", "Đinh", "Mậu", "Kỷ", "Canh", "Tân", "Nhâm", "Quý",
];
```

---

*Convention analysis: 2026-02-28*
