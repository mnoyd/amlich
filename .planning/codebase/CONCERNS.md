# Codebase Concerns

**Analysis Date:** 2026-02-28

## Tech Debt

**Panic-based error handling in core domain:**
- Issue: Several core calculation functions use `panic!` for invalid input instead of returning `Result`
- Files: `crates/amlich-core/src/almanac/taboo.rs`, `crates/amlich-core/src/almanac/day_deity.rs`, `crates/amlich-core/src/almanac/than_huong.rs`
- Impact: Invalid JSON data or programming errors cause immediate process termination, not graceful degradation
- Fix approach: Replace `panic!` with `Result` returns or `expect` with descriptive messages. Create domain-specific error types for almanac calculations.

**Heavy clone usage in conversion layer:**
- Issue: `convert.rs` contains extensive `.clone()` calls for every DTO field conversion
- Files: `crates/amlich-api/src/convert.rs` (482 lines)
- Impact: Unnecessary heap allocations when converting from core types to DTOs, especially for nested structures like `DayFortune` with multiple layers
- Fix approach: Implement `From` traits that borrow where possible, or use `Cow<'_, str>` for string fields that could be borrowed.

**Large monolithic data file:**
- Issue: `almanac/data.rs` is 1199 lines, mixing data structures, deserialization logic, and tests
- Files: `crates/amlich-core/src/almanac/data.rs`
- Impact: Difficult to navigate, mixes concerns (data models, parsing, registry, tests), high cognitive load
- Fix approach: Split into `mod.rs` (exports), `types.rs` (structs), `load.rs` (deserialization), `registry.rs` (runtime access).

**Dead code accumulated during development:**
- Issue: Three `#[allow(dead_code)]` attributes in `app.rs`
- Files: `crates/amlich/src/app.rs:67`, `crates/amlich/src/app.rs:352`, `crates/amlich/src/app.rs:411`
- Impact: Unclear if code is pending use or should be removed; creates maintenance burden
- Fix approach: Remove truly dead code, or document why it's suppressed (e.g., "used by unreleased feature").

## Known Bugs

**No explicit bugs documented:**
- No `TODO`, `FIXME`, `XXX`, or `HACK` comments found in the Rust codebase
- Search across all `.rs` files in `/crates` returned no matches

## Security Considerations

**Silent error handling in bookmark persistence:**
- Risk: Failed bookmark saves are only logged to stderr with `eprintln!`, user receives no feedback
- Files: `crates/amlich/src/app.rs:427`
- Current mitigation: Error is logged, function continues silently
- Recommendations: Return error from `toggle_bookmark()`, display error message to user in TUI, consider retry mechanism for transient failures

**No input sanitization for JSON data sources:**
- Risk: `baseline.json` is loaded via `include_str!` and deserialized directly; malformed JSON or oversized data could cause issues
- Files: `crates/amlich-core/src/almanac/data.rs:14`
- Current mitigation: Build-time inclusion means runtime compromise limited, but large data affects binary size
- Recommendations: Add JSON schema validation at build time, consider size limits for data arrays

**Hardcoded timezone assumption:**
- Risk: `VIETNAM_TIMEZONE` (UTC+7) is default; no validation that users in other timezones understand implications
- Files: `crates/amlich-core/src/lib.rs:97`
- Current mitigation: Timezone parameter exists but rarely used
- Recommendations: Document timezone behavior clearly, consider warning when non-Vietnam timezone is used

**No secrets management:**
- Risk: No `.env` files or secret handling detected (positive for this use case), but no documented approach for future needs
- Files: None found
- Current mitigation: Not applicable for current feature set
- Recommendations: Document that no secrets are needed, establish pattern if future features require API keys

## Performance Bottlenecks

**Repeated baseline data lookups:**
- Problem: `baseline_data()` called in multiple hot-path functions, potentially redundant global lookups
- Files: `crates/amlich-core/src/almanac/taboo.rs:34`, `crates/amlich-core/src/almanac/day_deity.rs:6`, `crates/amlich-core/src/almanac/than_huong.rs:21`
- Cause: Each function independently fetches global static data; OnceLock ensures single initialization but lookup happens on every call
- Improvement path: Consider passing data reference down through call chain, or benchmark to confirm if this is actually a bottleneck

**Monthly data loading recalculates unnecessarily:**
- Problem: `App::load_month()` recalculates entire month data on every navigation
- Files: `crates/amlich/src/app.rs:189-232`
- Cause: Clear and rebuild approach is simple but wasteful when only changing selected day
- Improvement path: Implement incremental loading, cache month data separately from selection state

**String allocations in hot path:**
- Problem: Date strings formatted repeatedly even when not displayed
- Files: `crates/amlich-core/src/lib.rs:145-161` (date_string fields in `SolarInfo` and `LunarInfo`)
- Cause: Eager string formatting in core data structures
- Improvement path: Defer string formatting to display layer, use `Display` impl instead of storing strings

## Fragile Areas

**Almanac calculation chain:**
- Files: `crates/amlich-core/src/almanac/calc.rs` (329 lines), `crates/amlich-core/src/almanac/data.rs` (1199 lines)
- Why fragile: Multi-step calculation with many data lookups; changes to JSON schema can cause runtime panics
- Safe modification: Add tests for each calculation phase, use golden files for output validation, make data access more type-safe
- Test coverage: Good coverage in `crates/amlich-core/tests/almanac_golden.rs`, but integration tests could be more comprehensive

**TUI event handling logic:**
- Files: `crates/amlich/src/event.rs` (280 lines), `crates/amlich/src/app.rs` (602 lines)
- Why fragile: Complex state machine with multiple overlay modes; easy to introduce bugs when adding new keybindings
- Safe modification: Extract mode handlers into separate modules, add state transition tests, document keybinding matrix
- Test coverage: Limited; event handling tested only manually via TUI interaction

**Widget rendering with large inline widgets:**
- Files: `crates/amlich/src/widgets/insight_overlay.rs` (615 lines), `crates/amlich/src/widgets/almanac_overlay.rs` (495 lines)
- Why fragile: Large procedural rendering functions; changes can affect multiple display paths; hard to unit test rendering logic
- Safe modification: Extract rendering sub-functions, add rendering tests using buffer inspection, consider component-based approach
- Test coverage: Minimal; rendering logic is largely untested

**Bookmark file I/O:**
- Files: `crates/amlich/src/bookmark_store.rs` (83 lines)
- Why fragile: File system operations can fail; uses temporary file with process ID in filename (race condition risk); silent failure on load errors
- Safe modification: Use atomic file replacement library, return errors instead of swallowing them, add file locking for concurrent access
- Test coverage: Basic normalization tests; no file I/O error simulation tests

## Scaling Limits

**TUI performance with large datasets:**
- Current capacity: Month view limited to 31 days; search limited to holidays in ±1 year window
- Limit: Search algorithm creates HashMap for each query; could degrade with larger year ranges or more data
- Scaling path: Implement pagination for search results, consider indexing holidays for faster lookup, cache normalized search terms

**WASM binary size:**
- Current capacity: Baseline data embedded at compile time in `baseline.json`
- Limit: Each additional ruleset or expanded data increases binary size; affects browser load times
- Scaling path: Consider lazy-loading ruleset data, use compression for JSON payloads, evaluate tree-shaking effectiveness

**Desktop app resource usage:**
- Current capacity: Single-date calculation is fast; month view loads 31 days
- Limit: No multi-threading for bulk calculations; year view or bulk export would be slow
- Scaling path: Add parallel calculation for batch operations, implement incremental loading, consider pagination for large date ranges

## Dependencies at Risk

**Ratatui version stability:**
- Risk: `ratatui` v0.30 is actively developed; breaking changes in future versions could require significant widget updates
- Impact: TUI rendering code (`widgets/*.rs`) would need updates
- Migration plan: Monitor ratatui changelog; pin version in workspace dependencies; consider abstracting rendering behind trait

**Tauri 2.x migration:**
- Risk: Desktop app uses Tauri 2.10.2; API changes in 2.x series have been significant
- Impact: `apps/desktop/src-tauri/` may need updates for future Tauri versions
- Migration plan: Monitor Tauri release notes; test on each minor version bump; keep Tauri code isolated

**Chrono dependency for date handling:**
- Risk: Heavy dependency for what could be lighter date math; potential for calendar edge cases
- Impact: Core library depends on chrono for `NaiveDate` and timezone handling
- Migration plan: Consider implementing custom lightweight date types if binary size becomes critical; current usage is appropriate

## Missing Critical Features

**No comprehensive error reporting:**
- Problem: Errors from core calculations are strings (`Result<T, String>`); no structured error types
- Blocks: User-friendly error messages, error logging/monitoring, programmatic error handling
- Impact: Difficult to distinguish between different failure modes; all errors appear as opaque strings

**No date range validation in API:**
- Problem: `get_day_info` validates day (1-31) and month (1-12) but not valid date combinations (e.g., Feb 30)
- Files: `crates/amlich-api/src/lib.rs:14-25`
- Blocks: Guaranteed valid dates passed to core; could return unexpected results for invalid dates
- Impact: API accepts invalid dates that core might calculate incorrectly

**No timezone-aware date conversion:**
- Problem: While `get_day_info_with_timezone` exists, no timezone database for historical DST or region-specific offsets
- Blocks: Accurate lunar conversions for regions outside Vietnam or for historical dates
- Impact: Users in different timezones may get incorrect lunar dates

**No cancellation for long operations:**
- Problem: Search and month loading are synchronous; no way to cancel in-progress operations
- Blocks: Responsive UI when user quickly switches views or enters new search
- Impact: App may feel sluggish if operations become slow

## Test Coverage Gaps

**TUI rendering logic untested:**
- What's not tested: Widget rendering code, layout calculations, text wrapping, color application
- Files: `crates/amlich/src/widgets/*.rs` (8 widget files, ~2500 lines total)
- Risk: Visual regressions, layout breaks, text truncation bugs
- Priority: Medium (manual testing catches major issues)

**Event handling flow untested:**
- What's not tested: Key press routing, mode transitions, state changes in response to events
- Files: `crates/amlich/src/event.rs`, TUI-specific methods in `crates/amlich/src/app.rs`
- Risk: Broken keyboard shortcuts, mode transitions that don't update state, unreachable UI states
- Priority: High (user-facing, hard to test manually)

**Error paths not tested:**
- What's not tested: Bookmark save failures, file I/O errors, invalid input handling
- Files: `crates/amlich/src/bookmark_store.rs`, error handling in `crates/amlich-api/src/lib.rs`
- Risk: Silent failures, data loss, confusing error messages
- Priority: Medium (file I/O errors are rare but impactful)

**Date boundary conditions:**
- What's not tested: Leap year handling, month boundaries, year transitions, timezone edge cases
- Files: Core calculation logic in `crates/amlich-core/src/`
- Risk: Incorrect calculations for edge dates, crashes on boundary inputs
- Priority: High (core functionality; partial coverage exists but could be more comprehensive)

**Integration tests missing for CLI:**
- What's not tested: Full CLI invocation paths, output formatting, argument parsing
- Files: `crates/amlich/src/main.rs`, `crates/amlich/tests/cli_contract.rs` (partial coverage)
- Risk: CLI interface regressions, broken user workflows
- Priority: Low (CLI is less commonly used than TUI)

**Search functionality testing:**
- What's not tested: Search result ranking, holiday name normalization, Tết special handling
- Files: `crates/amlich/src/search.rs`
- Risk: Search returns unexpected results, poor ranking, missed holidays
- Priority: Low (basic tests exist; edge cases covered by manual testing)

---

*Concerns audit: 2026-02-28*
