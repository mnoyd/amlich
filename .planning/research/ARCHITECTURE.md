# Architecture Research

**Domain:** v1.2 integration architecture for Ten Gods (Thập Thần) + Tử Mệnh/Kua in existing Rust almanac engine  
**Researched:** 2026-03-02  
**Confidence:** HIGH (based on current codebase structure and existing integration patterns)

## Standard Architecture (for this milestone)

### System Overview

```
┌────────────────────────────────────────────────────────────────────┐
│                        Core Calculation Layer                       │
├────────────────────────────────────────────────────────────────────┤
│  almanac/thap_than.rs   almanac/tu_menh.rs (new)   almanac/calc.rs│
│          │                       │                      │           │
│          └───────────────┬───────┴───────────────┬──────┘           │
├──────────────────────────┴───────────────────────┴──────────────────┤
│                        Typed Contract Layer                          │
├────────────────────────────────────────────────────────────────────┤
│                 almanac/types.rs (DayFortune extensions)            │
├────────────────────────────────────────────────────────────────────┤
│                        API/Presentation Layer                        │
├────────────────────────────────────────────────────────────────────┤
│ amlich-api dto/convert  │  CLI JSON contract  │  TUI widgets/overlay│
└────────────────────────────────────────────────────────────────────┘
```

### Component Responsibilities

| Component | Responsibility | Integration Role |
|---|---|---|
| `crates/amlich-core/src/almanac/thap_than.rs` | Deterministic stem-to-stem Ten Gods mapping | Existing engine reused by DayFortune integration |
| `crates/amlich-core/src/almanac/tu_menh.rs` (new) | Kua number/group/directions from birth year + gender | New standalone calculation module |
| `crates/amlich-core/src/almanac/calc.rs` | Orchestrates day-level fortune assembly | Modified to attach Ten Gods/Kua outputs |
| `crates/amlich-core/src/almanac/types.rs` | Canonical typed domain structs + serde | Modified with optional nested fields |
| `crates/amlich-api/src/dto.rs` + `convert.rs` | External wire contract and mapping | Modified for additive API surface |
| `crates/amlich*/tests` | Regression and contract safety | Expanded for backward compatibility + new assertions |

## New vs Modified Components (explicit)

### New (recommended)

1. `crates/amlich-core/src/almanac/tu_menh.rs`
   - `get_kua(birth_year: i32, gender: Gender) -> KuaResult`
   - Century-transition normalization logic lives here (TM-02).

2. `crates/amlich-core/tests/kua_matrix.rs` (or similar)
   - Fixture-driven representative years across 1900–2099.

3. `crates/amlich-api/tests/kua_contract.rs` (or expand existing contract file)
   - API exposure and schema assertions for Kua endpoints/fields.

### Modified (required)

1. `crates/amlich-core/src/almanac/types.rs`
   - Add **nested optional structs** to avoid flat-field schema churn:
   - `DayTenGods` (e.g., day stem + relations needed by day-level output)
   - `KuaResult` + subtypes (`KuaGroup`, direction sets)
   - Extend `DayFortune` with:
     - `ten_gods: Option<DayTenGods>`
     - `tu_menh: Option<KuaResult>`

2. `crates/amlich-core/src/almanac/calc.rs`
   - Populate `ten_gods` from existing can-chi context.
   - Populate `tu_menh` **only when required inputs are available** (birth year + gender).

3. `crates/amlich-core/src/lib.rs`
   - Re-export Kua types/functions similarly to existing Ten Gods export style.

4. `crates/amlich-api/src/dto.rs`
   - Add `TenGodsDto`, `KuaDto` (optional in `DayFortuneDto` for backward compatibility).

5. `crates/amlich-api/src/convert.rs`
   - Add `From` conversions for new typed objects.

6. Client-facing contract tests/UI references:
   - `crates/amlich-api/tests/almanac_contract.rs`
   - `crates/amlich/tests/cli_contract.rs`
   - TUI widgets (`info_panel.rs`, `almanac_overlay.rs`) should remain tolerant of missing fields.

## Recommended Project Structure (integration delta)

```
crates/amlich-core/src/almanac/
├── thap_than.rs            # Existing Ten Gods engine (already implemented)
├── tu_menh.rs              # NEW Kua/Tu Menh calculations
├── calc.rs                 # Modified orchestrator
├── types.rs                # Modified typed outputs
└── mod.rs                  # Export tu_menh module

crates/amlich-api/src/
├── dto.rs                  # Modified API schema (additive optional fields)
└── convert.rs              # Modified DTO mapping

crates/amlich-core/tests/
├── ...existing khcbppt_* regression suites
└── kua fixtures/tests      # NEW TM-04 safety net
```

### Structure Rationale

- **Keep domain logic in `amlich-core/almanac`**: avoids API/UI coupling and preserves existing engine boundaries.
- **Additive optional fields in contracts**: minimizes regression risk for existing callers and snapshot tests.

## Architectural Patterns to Follow

### Pattern 1: Additive Contract Evolution (required)
**What:** New data appears under optional nested objects, not replacing existing fields.  
**When to use:** All v1.2 DayFortune/API changes.  
**Trade-off:** Slightly deeper JSON, much lower break risk.

### Pattern 2: Orchestrator-only Wiring
**What:** `calc.rs` composes results; rule engines (`thap_than.rs`, `tu_menh.rs`) remain pure and independently testable.  
**When to use:** Any new astrological subsystem integration.

### Pattern 3: Evidence-first Typed Output
**What:** Follow existing `RuleEvidence` conventions for new outputs where source attribution exists.  
**When to use:** Ten Gods and Kua result structs.

## Data Flow Updates

### Updated Request Flow

```
Date query (+ optional birth_year, gender)
    ↓
amlich_core::get_day_info_with_timezone()
    ↓
calculate_day_fortune(...)
    ├─ existing day modules (taboo/star/truc/...)
    ├─ get_thap_than(day_can, target_can)  [existing, now surfaced]
    └─ get_kua(birth_year, gender)         [new, conditional]
    ↓
DayFortune { ..., ten_gods?: ..., tu_menh?: ... }
    ↓
amlich-api convert.rs → DayFortuneDto
    ↓
CLI/TUI/JSON consumers (unchanged for old fields)
```

### Key Integration Points

1. **Core type boundary:** `almanac/types.rs` is the canonical schema contract.
2. **Orchestrator boundary:** only `calc.rs` should know how to combine subsystems.
3. **API boundary:** `dto.rs`/`convert.rs` mirrors core, never recalculates domain logic.
4. **Consumer boundary:** UI/CLI should feature-detect optional fields, never assume presence.

## Dependency-Safe Build Order (recommended)

1. **Type scaffolding first** (`types.rs`, DTO placeholders)
   - Define `KuaResult`/`DayTenGods` with `Option` fields.
2. **Implement pure Kua engine** (`tu_menh.rs` + unit fixtures)
   - No integration side effects yet.
3. **Wire core orchestrator** (`calc.rs`)
   - Populate new fields conditionally.
4. **Wire API conversion** (`dto.rs`, `convert.rs`)
   - Ensure serialization stability.
5. **Update integration/contract tests**
   - Keep old assertions + add new presence/absence assertions.
6. **Update UI surfaces last**
   - Read-only display additions after contracts are stable.
7. **Run full regression gate**
   - `cargo test --package amlich-core`
   - API and CLI contract tests.

This ordering prevents interface churn and catches schema breakage before UI changes.

## Regression Risk Controls (most important)

1. **Never make new fields required** in `DayFortune`/DTO during v1.2.
2. **Do not change existing field names/semantics** (`stars`, `truc`, `xung_hop`, etc.).
3. **Keep Ten Gods/Kua as separate nested payloads** (avoid polluting unrelated structs).
4. **Preserve existing khcbppt regression suites untouched**; add tests, don’t repurpose.

## Anti-Patterns to Avoid

### Anti-Pattern 1: Flat field sprawl in `DayFortune`
**Why bad:** Increases accidental breakage in API conversion and UI rendering.  
**Instead:** Use `ten_gods` and `tu_menh` nested objects.

### Anti-Pattern 2: Recomputing domain logic in API layer
**Why bad:** Divergence risk between core and API.  
**Instead:** API only maps core types (`From<&...>` pattern).

### Anti-Pattern 3: Gating old behavior on new inputs
**Why bad:** Existing callers without birth metadata regress.  
**Instead:** New features are strictly additive and optional.

## Sources

- Internal architecture evidence (current codebase):
  - `crates/amlich-core/src/almanac/types.rs`
  - `crates/amlich-core/src/almanac/calc.rs`
  - `crates/amlich-core/src/almanac/thap_than.rs`
  - `crates/amlich-core/src/almanac/data.rs`
  - `crates/amlich-core/src/lib.rs`
  - `crates/amlich-api/src/dto.rs`
  - `crates/amlich-api/src/convert.rs`
  - `crates/amlich-api/tests/almanac_contract.rs`
  - `crates/amlich/tests/cli_contract.rs`
- Milestone requirements:
  - `.planning/REQUIREMENTS-v1.2.md` (TM-01..TM-05, INT-01..INT-06)

---
*Architecture research for: v1.2 Ten Gods and Kua integration*
