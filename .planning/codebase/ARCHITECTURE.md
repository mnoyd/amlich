# Architecture

**Analysis Date:** 2026-02-28

## Pattern Overview

**Overall:** Layered workspace architecture with core domain logic, API abstraction, and multiple runtime targets

**Key Characteristics:**
- Rust workspace with 4 core crates plus Tauri desktop app
- Core library provides pure calendar calculations with no external dependencies
- API crate provides stable DTO contract for cross-platform consumption
- Multiple frontends (CLI TUI, WASM, Tauri desktop) share the same domain model
- Data-driven almanac ruleset with JSON source files

## Layers

**Domain Core Layer (`amlich-core`):**
- Purpose: Pure Vietnamese lunar calendar calculations and almanac rules
- Location: `/Users/noy/work/junks/amlich/crates/amlich-core/src`
- Contains: Lunar/solar conversions, Can Chi calculations, Tiết Khí (solar terms), Giờ Hoàng Đạo (auspicious hours), holidays, almanac day fortune calculations
- Depends on: Standard library only, chrono (for date types)
- Used by: All other layers (amlich-api, amlich-wasm, amlich CLI)

**API Abstraction Layer (`amlich-api`):**
- Purpose: Stable data transfer objects (DTOs) and serialization-friendly API
- Location: `/Users/noy/work/junks/amlich/crates/amlich-api/src`
- Contains: DTO types, conversion functions from core types to serializable structs, holiday/insight queries
- Depends on: amlich-core (domain types), serde (serialization)
- Used by: amlich CLI, amlich-wasm, external consumers

**Application Layer (`amlich` CLI):**
- Purpose: Terminal user interface and headless CLI operations
- Location: `/Users/noy/work/junks/amlich/crates/amlich/src`
- Contains: TUI app state, event handling, widgets, headless output formatting
- Depends on: amlich-api, ratatui (TUI), crossterm (terminal events), clap (CLI parsing)
- Used by: End users via CLI

**WebAssembly Layer (`amlich-wasm`):**
- Purpose: Browser/JavaScript bindings for web usage
- Location: `/Users/noy/work/junks/amlich/crates/amlich-wasm/src`
- Contains: wasm-bindgen exports, JS-compatible wrappers
- Depends on: amlich-api, wasm-bindgen, serde-wasm-bindgen
- Used by: Web applications via NPM package

**Desktop Application Layer (`desktop`):**
- Purpose: Native desktop app using Tauri + Svelte
- Location: `/Users/noy/work/junks/amlich/apps/desktop/src-tauri`
- Contains: Tauri IPC handlers, native windowing
- Depends on: amlich-core or amlich-api, Tauri framework
- Used by: Desktop app users

## Data Flow

**Core Calculation Flow:**

1. User input (date) → Entry point (`main.rs` or WASM function)
2. Date validation → `DateQuery` DTO
3. Core calculation → `amlich_core::get_day_info()`
   - Julian Day calculation (`jd_from_date`)
   - Lunar conversion (`convert_solar_to_lunar`)
   - Can Chi calculation (`get_day_canchi`, `get_month_canchi`, `get_year_canchi`)
   - Solar terms (`get_tiet_khi`)
   - Auspicious hours (`get_gio_hoang_dao`)
   - Day fortune/almanac (`calculate_day_fortune`)
4. API conversion → `DayInfoDto` serialization
5. Output formatting → JSON/Text/TUI/Waybar

**TUI Event Flow:**

1. Terminal input → `crossterm::event::read()` in `/Users/noy/work/junks/amlich/crates/amlich/src/event.rs`
2. Key routing → `handle_key()` matches mode-specific handlers
3. State mutation → `App` methods (navigation, toggles, data loading)
4. Cache refresh → `refresh_selected_insight_cache()` calls `get_day_insight_for_date()`
5. Render → `ui::render()` builds widget tree via ratatui
6. Terminal draw → `terminal.draw()`

**State Management:**
- TUI: Single `App` struct owned by main loop, mutable via event handlers
- CLI: Stateless - each query creates new data from scratch
- Desktop: Tauri-managed state with Rust backend handlers

## Key Abstractions

**DayInfo (Core Domain):**
- Purpose: Complete calendar information for a single solar date
- Examples: `/Users/noy/work/junks/amlich/crates/amlich-core/src/lib.rs`
- Pattern: Struct with solar/lunar dates, Can Chi, solar terms, auspicious hours, day fortune
- Factory: `get_day_info(day, month, year)` returns fully populated struct

**DayInfoDto (API Contract):**
- Purpose: Serializable version of DayInfo for external consumption
- Examples: `/Users/noy/work/junks/amlich/crates/amlich-api/src/dto.rs`
- Pattern: JSON-serializable structs with `serde` derives, all fields public
- Conversion: `From<&DayInfo>` trait implementation in `/Users/noy/work/junks/amlich/crates/amlich-api/src/convert.rs`

**DayFortune (Almanac Ruleset):**
- Purpose: Calculated fortune/prediction data with source evidence
- Examples: `/Users/noy/work/junks/amlich/crates/amlich-core/src/almanac/types.rs`
- Pattern: Nested structs containing elements, conflicts, stars, taboos, travel directions
- Evidence tracking: Each component includes `Option<RuleEvidence>` for auditability

**App State (TUI):**
- Purpose: Centralized state for terminal UI
- Examples: `/Users/noy/work/junks/amlich/crates/amlich/src/app.rs`
- Pattern: Single struct with view state, cached data, overlay modes
- Methods: Navigation (`next_day`, `prev_month`), toggles (`toggle_holidays`), cache management

## Entry Points

**CLI Entry Point:**
- Location: `/Users/noy/work/junks/amlich/crates/amlich/src/main.rs`
- Triggers: `amlich` command execution
- Responsibilities: Parse CLI args with clap, route to TUI or headless mode, auto-detect TTY

**TUI Runtime:**
- Location: `/Users/noy/work/junks/amlich/crates/amlich/src/tui_runtime.rs`
- Triggers: `amlich tui` subcommand or auto-detection when stdin/stdout are terminals
- Responsibilities: Initialize ratatui terminal, run event loop, restore terminal on exit

**WASM Entry Points:**
- Location: `/Users/noy/work/junks/amlich/crates/amlich-wasm/src/lib.rs`
- Triggers: JavaScript calls from browser
- Responsibilities: Export `#[wasm_bindgen]` functions, convert between Rust/JS types

**Desktop Entry Point:**
- Location: `/Users/noy/work/junks/amlich/apps/desktop/src-tauri/src`
- Triggers: Native desktop app launch
- Responsibilities: Tauri setup, IPC command handlers, window management

## Error Handling

**Strategy:** Result-based error propagation with String error messages

**Patterns:**
- Core functions return `Result<T, String>` for recoverable errors
- CLI/main uses `?` operator for early exit with user-friendly messages
- WASM functions return `JsValue::NULL` on error (simplified for JS interop)
- Panics reserved for truly unrecoverable states (e.g., invariant violations)

**Example from `/Users/noy/work/junks/amlich/crates/amlich-api/src/lib.rs`:**
```rust
pub fn get_day_info(query: &DateQuery) -> Result<DayInfoDto, String> {
    if !(1..=12).contains(&query.month) {
        return Err("month must be 1-12".to_string());
    }
    // ... calculation logic
    Ok(DayInfoDto::from(&info))
}
```

## Cross-Cutting Concerns

**Logging:** No structured logging framework; uses `eprintln!` for warnings/errors in CLI

**Validation:** Input validation at API boundaries (month/day range checks), then propagated through Result types

**Authentication:** Not applicable - local-only application with no user accounts or network services

**Serialization:** Centralized in amlich-api crate using serde; all DTOs derive Serialize/Deserialize

**Localization:** Partial support via insight DTOs with `vi`/`en` fields (`LocalizedTextDto`, `LocalizedListDto`)

**Configuration:** CLI mode stored in user config file (`~/.config/amlich/mode`), read/written via `bookmark_store.rs` helpers

---

*Architecture analysis: 2026-02-28*
