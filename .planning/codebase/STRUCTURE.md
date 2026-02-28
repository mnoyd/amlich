# Codebase Structure

**Analysis Date:** 2026-02-28

## Directory Layout

```
amlich/
├── crates/                  # Rust workspace crates
│   ├── amlich/              # CLI binary (TUI + headless)
│   ├── amlich-core/         # Core calendar library
│   ├── amlich-api/          # API/DTO layer
│   └── amlich-wasm/         # WASM bindings
├── apps/
│   └── desktop/             # Tauri desktop app
│       ├── src/             # Svelte frontend
│       └── src-tauri/       # Rust backend
├── packages/
│   └── core/                # JavaScript NPM package
├── data/                    # Static JSON datasets
│   ├── holidays/            # Holiday data files
│   ├── schemas/             # JSON schemas
│   └── *.json               # Can Chi, Tiết Khí data
├── waybar/                  # Waybar integration
├── docs/                    # Documentation
├── scripts/                 # Build/utility scripts
├── patches/                 # Dependency patches
├── .github/                 # GitHub Actions/Issue templates
├── .planning/               # GSD planning documents
├── Cargo.toml               # Workspace definition
├── package.json             # JS workspace config
└── justfile                 # Just command runner recipes
```

## Directory Purposes

**`crates/amlich-core/`:**
- Purpose: Core domain logic for Vietnamese lunar calendar calculations
- Contains: Pure calendar math, almanac ruleset engine, holiday calculations, Can Chi, Tiết Khí, Giờ Hoàng Đạo
- Key files: `/Users/noy/work/junks/amlich/crates/amlich-core/src/lib.rs` (main entry point), `/Users/noy/work/junks/amlich/crates/amlich-core/src/almanac/` (fortune calculation)
- Tests: `/Users/noy/work/junks/amlich/crates/amlich-core/tests/` (golden files, determinism tests)
- Data: `/Users/noy/work/junks/amlich/crates/amlich-core/data/` (holiday JSONs)

**`crates/amlich-api/`:**
- Purpose: Stable public API with serializable DTOs
- Contains: Data transfer objects, conversion functions, insight queries
- Key files: `/Users/noy/work/junks/amlich/crates/amlich-api/src/lib.rs`, `/Users/noy/work/junks/amlich/crates/amlich-api/src/dto.rs`, `/Users/noy/work/junks/amlich/crates/amlich-api/src/convert.rs`
- Tests: `/Users/noy/work/junks/amlich/crates/amlich-api/tests/` (contract tests, parity tests)

**`crates/amlich/`:**
- Purpose: Unified CLI binary with TUI and headless modes
- Contains: TUI app state, event handling, widgets, headless formatting
- Key files: `/Users/noy/work/junks/amlich/crates/amlich/src/main.rs` (CLI entry), `/Users/noy/work/junks/amlich/crates/amlich/src/app.rs` (TUI state), `/Users/noy/work/junks/amlich/crates/amlich/src/ui.rs` (rendering)
- Widgets: `/Users/noy/work/junks/amlich/crates/amlich/src/widgets/` (modular UI components)

**`crates/amlich-wasm/`:**
- Purpose: WebAssembly bindings for JavaScript/browser usage
- Contains: wasm-bindgen exports, JS-compatible wrappers
- Key files: `/Users/noy/work/junks/amlich/crates/amlich-wasm/src/lib.rs`

**`apps/desktop/`:**
- Purpose: Native desktop application (Tauri + Svelte)
- Contains: SvelteKit frontend, Tauri Rust backend
- Frontend: `/Users/noy/work/junks/amlich/apps/desktop/src/`
- Backend: `/Users/noy/work/junks/amlich/apps/desktop/src-tauri/`

**`packages/core/`:**
- Purpose: JavaScript NPM package wrapping WASM
- Contains: JS bindings, TypeScript definitions, WASM bundle
- Key files: `/Users/noy/work/junks/amlich/packages/core/index.js`

**`data/`:**
- Purpose: Shared static datasets used by core calculations
- Contains: Holiday definitions, solar terms, Can Chi mappings, schemas
- Key files: `/Users/noy/work/junks/amlich/data/holidays/lunar-festivals.json`, `/Users/noy/work/junks/amlich/data/holidays/solar-holidays.json`, `/Users/noy/work/junks/amlich/data/canchi.json`

## Key File Locations

**Entry Points:**
- `/Users/noy/work/junks/amlich/crates/amlich/src/main.rs`: CLI binary entry (clap parser, TUI/headless routing)
- `/Users/noy/work/junks/amlich/crates/amlich-core/src/lib.rs`: Core library entry (get_day_info function)
- `/Users/noy/work/junks/amlich/crates/amlich-api/src/lib.rs`: API entry (DTO functions)
- `/Users/noy/work/junks/amlich/crates/amlich-wasm/src/lib.rs`: WASM entry (wasm-bindgen exports)

**Configuration:**
- `/Users/noy/work/junks/amlich/Cargo.toml`: Workspace crate definitions
- `/Users/noy/work/junks/amlich/crates/amlich/Cargo.toml`: CLI binary dependencies
- `/Users/noy/work/junks/amlich/crates/amlich-core/Cargo.toml`: Core library dependencies
- `/Users/noy/work/junks/amlich/package.json`: JavaScript workspace config
- `/Users/noy/work/junks/amlich/apps/desktop/src-tauri/tauri.conf.json`: Tauri app config
- `/Users/noy/work/junks/amlich/justfile`: Command runner recipes

**Core Logic:**
- `/Users/noy/work/junks/amlich/crates/amlich-core/src/lunar.rs`: Solar/lunar date conversion
- `/Users/noy/work/junks/amlich/crates/amlich-core/src/canchi.rs`: Can Chi (Heavenly Stems & Earthly Branches)
- `/Users/noy/work/junks/amlich/crates/amlich-core/src/tietkhi.rs`: 24 Solar Terms calculation
- `/Users/noy/work/junks/amlich/crates/amlich-core/src/gio_hoang_dao.rs`: Auspicious Hours
- `/Users/noy/work/junks/amlich/crates/amlich-core/src/almanac/calc.rs`: Day fortune calculation engine
- `/Users/noy/work/junks/amlich/crates/amlich-core/src/almanac/data.rs`: Almanac ruleset data loading

**Testing:**
- `/Users/noy/work/junks/amlich/crates/amlich-core/tests/`: Core library integration tests
- `/Users/noy/work/junks/amlich/crates/amlich-api/tests/`: API contract tests
- `/Users/noy/work/junks/amlich/crates/amlich/tests/`: CLI contract tests

## Naming Conventions

**Files:**
- `snake_case.rs`: All Rust source files use lowercase with underscores
- `lib.rs`: Crate root files
- `main.rs`: Binary entry points
- `mod.rs`: Module definitions within subdirectories

**Directories:**
- `kebab-case/`: All directories use lowercase with hyphens (e.g., `amlich-core`, `src-tauri`)
- `src/`: Source code for each crate
- `tests/`: Integration test files

**Crates:**
- `amlich-<name>`: Workspace crate naming pattern
- Core domain types live in `amlich-core`
- API contracts live in `amlich-api`

**Structs/Types:**
- `PascalCase`: All Rust types (e.g., `DayInfo`, `SolarDto`, `DayFortune`)
- DTOs have `Dto` suffix (e.g., `DayInfoDto`, `SolarDto`)

**Functions:**
- `snake_case`: All functions (e.g., `get_day_info`, `convert_solar_to_lunar`)
- Public API functions use `get_<entity>` pattern (e.g., `get_day_info`, `get_holidays`)

## Where to Add New Code

**New Feature (Calendar Calculation):**
- Primary code: `/Users/noy/work/junks/amlich/crates/amlich-core/src/`
- Tests: `/Users/noy/work/junks/amlich/crates/amlich-core/tests/` or inline `#[cfg(test)]` modules
- Example: Adding new fortune calculation logic → create new module in `amlich-core/src/`, export from `lib.rs`

**New Feature (API/DTO):**
- Implementation: `/Users/noy/work/junks/amlich/crates/amlich-api/src/`
- DTO types: Add to `/Users/noy/work/junks/amlich/crates/amlich-api/src/dto.rs`
- Conversion: Add to `/Users/noy/work/junks/amlich/crates/amlich-api/src/convert.rs`
- Public functions: Add to `/Users/noy/work/junks/amlich/crates/amlich-api/src/lib.rs`

**New Feature (TUI Widget):**
- Implementation: `/Users/noy/work/junks/amlich/crates/amlich/src/widgets/`
- Pattern: Create `<feature>.rs` with widget rendering function
- Wire up: Import in `/Users/noy/work/junks/amlich/crates/amlich/src/widgets/mod.rs`, call from `/Users/noy/work/junks/amlich/crates/amlich/src/ui.rs`

**New Feature (CLI Command):**
- Subcommand: Add variant to `Command` enum in `/Users/noy/work/junks/amlich/crates/amlich/src/main.rs`
- Handler: Add function in appropriate module (headless, tui_runtime, etc.)
- Args: Define `Args` struct with clap derives

**New Component/Module (Almanac Ruleset):**
- Implementation: `/Users/noy/work/junks/amlich/crates/amlich-core/src/almanac/`
- Pattern: Create `<rule>.rs` with calculation logic and data loading
- Wire up: Import in `/Users/noy/work/junks/amlich/crates/amlich-core/src/almanac/mod.rs`

**Utilities:**
- Shared helpers (CLI): `/Users/noy/work/junks/amlich/crates/amlich/src/<module>.rs`
- Shared helpers (Core): `/Users/noy/work/junks/amlich/crates/amlich-core/src/<module>.rs`

## Special Directories

**`target/`:**
- Purpose: Cargo build output (compiled binaries, libraries)
- Generated: Yes
- Committed: No (in .gitignore)

**`node_modules/`:**
- Purpose: JavaScript dependencies for desktop app and NPM package
- Generated: Yes
- Committed: No

**`data/`:**
- Purpose: Static JSON datasets for holidays, Can Chi, solar terms
- Generated: No
- Committed: Yes

**`scripts/`:**
- Purpose: Build scripts, WASM compilation, release automation
- Generated: No
- Committed: Yes

**`patches/`:**
- Purpose: Dependency patches for npm packages
- Generated: No
- Committed: Yes

**`.planning/`:**
- Purpose: GSD planning documents, codebase analysis
- Generated: No
- Committed: Yes

---

*Structure analysis: 2026-02-28*
