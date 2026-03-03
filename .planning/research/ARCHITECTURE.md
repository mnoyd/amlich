# Architecture Research: Dai Van Integration

**Domain:** Vietnamese Almanac (Đại Vân/Major Luck period calculation)
**Researched:** 2026-03-03
**Confidence:** HIGH

## Executive Summary

Dai Van (Đại Vân/大运) is a 10-year luck cycle calculation that requires new computation components but integrates cleanly with the existing DayFortune architecture using the established optional field pattern. The implementation follows the same additive-only integration approach used for Ten Gods and Kua in v1.2, ensuring backward compatibility while adding birth-based capabilities.

**Key architectural insight:** Dai Van introduces a new input type (birth date + gender) that is semantically distinct from the day-based almanac calculations. The recommended approach creates a separate calculation pathway that can optionally populate DayFortune when birth context is provided, rather than modifying the core day-fortune calculation logic.

---

## Current Architecture Context

### Existing Component Structure

```
amlich-core/src/almanac/
├── types.rs          # Core type definitions (DayFortune, Ten Gods, Kua)
├── calc.rs           # Main entry point: calculate_day_fortune()
├── thap_than.rs      # Ten Gods calculation (v1.2)
├── tu_menh.rs        # Kua/Tu Mệnh calculation (v1.2)
├── canchi.rs         # Can/Chi utilities
├── tietkhi.rs        # Tiết Khí (solar term) calculations
├── data.rs           # Golden dataset and rules
├── [other modules]   # Stars, taboos, Xung/Hop, etc.
└── mod.rs            # Module exports
```

### DayFortune Structure (v1.2)

```rust
pub struct DayFortune {
    // Core day-based fields (v1.0)
    pub ruleset_id: String,
    pub ruleset_version: String,
    pub profile: String,
    pub day_element: DayElement,
    pub conflict: DayConflict,
    pub travel: TravelDirection,
    pub stars: DayStars,
    pub day_deity: Option<DayDeity>,
    pub taboos: Vec<DayTaboo>,
    pub xung_hop: XungHopResult,
    pub truc: TrucInfo,
    pub tang_can: Option<TangCan>,

    // v1.2 additions (optional fields)
    pub ten_gods: Option<DayTenGods>,     // Populated when day stem available
    pub tu_menh: Option<super::tu_menh::KuaResult>, // Populated when birth context provided

    // v1.3 addition (this milestone)
    pub dai_van: Option<DaYunResult>,      // Populated when birth date + gender provided
}
```

### Integration Pattern from v1.2

Both Ten Gods and Kua followed the same pattern:

1. **New module created** (`thap_than.rs`, `tu_menh.rs`)
2. **Optional field added to DayFortune**
3. **`calculate_day_fortune()` modified** to conditionally compute new feature
4. **Input expansion** (year_can for Ten Gods, birth_year+gender for Kua)
5. **API layer updated** with matching DTO types
6. **Tests added** to verify optional field population logic

Dai Van should follow this same pattern.

---

## Recommended Architecture for Dai Van

### System Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    API / CLI Layer                         │
├─────────────────────────────────────────────────────────────┤
│  Date Query → calculate_day_fortune(date)                 │
│  Birth Query → calculate_day_fortune(date, birth_ctx)      │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│              Calculation Layer (calc.rs)                    │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐     │
│  │ Day-based   │  │ Ten Gods    │  │ Kua         │     │
│  │ calculation │  │ (optional)  │  │ (optional)  │     │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘     │
│         │                 │                 │              │
│         └─────────────────┴─────────────────┘              │
│                           ↓                               │
│  ┌─────────────────────────────────────────────┐           │
│  │      Dai Van (NEW, optional)             │           │
│  │  - Requires: birth_date + gender          │           │
│  │  - Reuses: Can Chi, Tiết Khí modules     │           │
│  │  - Integrates: Ten Gods, Kua results     │           │
│  └─────────────────────────────────────────────┘           │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│           Foundation Modules (no changes)                  │
├─────────────────────────────────────────────────────────────┤
│  canchi.rs          │ tietkhi.rs   │ julian.rs          │
│  lunar.rs           │ types.rs      │ data.rs            │
└─────────────────────────────────────────────────────────────┘
```

### Component Responsibilities

| Component | Responsibility | Typical Implementation |
|-----------|----------------|------------------------|
| `dai_van.rs` (NEW) | Core Dai Van calculation logic | Pure functions, deterministic, testable in isolation |
| `calc.rs` (MODIFIED) | Orchestrate all calculations including Dai Van | Conditionally call dai_van::calculate when inputs provided |
| `types.rs` (MODIFIED) | Add dai_van optional field to DayFortune | Add new Dai Van type definitions |
| `thap_than.rs` (REUSED) | Ten Gods calculation for each pillar | Called from Dai Van pillar correlation |
| `tu_menh.rs` (REUSED) | Kua calculation for birth context | Called from Dai Van for directional analysis |

---

## Detailed Integration Analysis

### 1. Integration Point with DayFortune

**Decision:** Add optional field following v1.2 pattern

```rust
// crates/amlich-core/src/almanac/types.rs

pub struct DayFortune {
    // ... existing fields ...

    /// Dai Van (Major Luck) result (populated only when birth date and gender provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dai_van: Option<DaYunResult>,
}
```

**Rationale:**
- ✅ Consistent with ten_gods and tu_menh pattern (proven in v1.2)
- ✅ Backward compatible (existing clients see None)
- ✅ Single return type for both date-only and date+birth queries
- ✅ Matches "additive-only integration" project decision

**Alternative Considered and Rejected:**
- Separate `BirthFortune` type from research report
  - ❌ Creates API fragmentation (clients need to know which type to request)
  - ❌ Breaks unified query model
  - ❌ More complex for existing consumers
- New API function `calculate_birth_fortune()`
  - ❌ Unnecessary for current scope (DayFortune works with optional fields)
  - ❌ Can be added later if needed for optimization

### 2. New Components Required

#### Component 1: dai_van.rs (NEW MODULE)

**File location:** `crates/amlich-core/src/almanac/dai_van.rs`

**Responsibilities:**
- Core Dai Van calculation algorithm
- 6-step computation: lunar conversion → year/month Can Chi → chieuthu → start age → pillars
- Ten Gods correlation for each pillar
- Kua integration for directional analysis

**Key types:**
```rust
pub enum Gender { Male, Female }
pub enum ChieuThu { Thuan, Nghich }  // Direction of progression
pub struct DaYunPillar {
    pub thu_tu: u8,              // Pillar number (1-8)
    pub start_age: u8,
    pub end_age: u8,
    pub can: HeavenlyStem,
    pub chi: EarthlyBranch,
    pub can_chi_name: String,
    pub ten_gods: Option<ThapThanResult>,  // Correlation with day stem
}
pub struct DaYunResult {
    pub chieu_thu: ChieuThu,
    pub start_age: u8,
    pub num_pillars: u8,
    pub pillars: Vec<DaYunPillar>,
    pub convention: ConventionMetadata,
}
```

**Key functions:**
```rust
pub fn calculate_dai_yun(
    birth_date: NaiveDate,
    gender: Gender,
    day_stem: Option<HeavenlyStem>,  // For Ten Gods correlation
) -> DaYunResult

pub fn get_current_pillar(dai_yun: &DaYunResult, current_age: u8) -> Option<&DaYunPillar>
```

**Estimated LOC:** 400-600 lines (including tests)

#### Component 2: API DTO Types (NEW)

**File location:** `crates/amlich-api/src/dto.rs`

**Additions:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaYunPillarDto {
    pub thu_tu: u8,
    pub start_age: u8,
    pub end_age: u8,
    pub can: String,
    pub chi: String,
    pub can_chi_name: String,
    pub ten_gods: Option<ThapThanResultDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaYunResultDto {
    pub chieu_thu: String,
    pub start_age: u8,
    pub num_pillars: u8,
    pub pillars: Vec<DaYunPillarDto>,
    pub convention: ConventionMetadataDto,
}
```

**Add to DayFortuneDto:**
```rust
pub struct DayFortuneDto {
    // ... existing fields ...

    /// Dai Van result (populated only when birth date and gender provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dai_van: Option<DaYunResultDto>,
}
```

#### Component 3: Conversion Layer (NEW)

**File location:** `crates/amlich-api/src/convert.rs`

**Additions:**
```rust
impl From<almanac::dai_van::DaYunResult> for DaYunResultDto {
    fn from(result: almanac::dai_van::DaYunResult) -> Self {
        // Conversion logic
    }
}

impl From<almanac::dai_van::DaYunPillar> for DaYunPillarDto {
    fn from(pillar: almanac::dai_van::DaYunPillar) -> Self {
        // Conversion logic
    }
}
```

### 3. Modified Components

#### Component 1: calc.rs (MODIFIED)

**Changes:**
1. Add new optional parameters to `calculate_day_fortune()`:
```rust
pub fn calculate_day_fortune(
    jd: i32,
    day_canchi: &CanChi,
    lunar_day: i32,
    lunar_month: i32,
    year_can: &str,
    tiet_khi_name: &str,
    // NEW optional parameters for Dai Van
    birth_date: Option<NaiveDate>,  // Birth date (solar)
    birth_year: Option<i32>,          // Birth year (solar, for Kua)
    gender: Option<Gender>,           // Gender (for Dai Van and Kua)
) -> DayFortune
```

2. Add Dai Van computation logic:
```rust
// After existing calculations, add:

dai_van: {
    match (birth_date, gender) {
        (Some(bd), Some(g)) => {
            // Calculate day stem for Ten Gods correlation
            let day_stem = HeavenlyStem::try_from(day_canchi.can.as_str()).ok();

            Some(dai_van::calculate_dai_yun(bd, g, day_stem))
        }
        _ => None,
    }
},
```

3. Update Kua computation to use new parameters (simplifies existing logic):
```rust
tu_menh: {
    match (birth_year, gender) {
        (Some(by), Some(g)) => Some(tu_menh::compute_kua(by, g)),
        _ => None,
    }
},
```

**Rationale for signature change:**
- ⚠️ This is a **breaking change** to `calculate_day_fortune()`
- Mitigation: Update all call sites in one PR
- Call sites to update:
  - `crates/amlich-core/tests/*.rs` (test files)
  - `crates/amlich-api/src/convert.rs` (API conversion)
  - Any other direct consumers

**Alternative:** Add overloaded function
```rust
// Keep original signature for backward compatibility
pub fn calculate_day_fortune_v1(...) -> DayFortune { /* original */ }

// New signature with birth context
pub fn calculate_day_fortune(
    /* ... new signature ... */
) -> DayFortune { /* calls v1 internally */ }
```
- ❌ Creates confusion (which version to call?)
- ❌ Breaks "additive-only" pattern
- ✅ **Better:** Update signature in coordinated PR

#### Component 2: types.rs (MODIFIED)

**Changes:**
1. Add Dai Van type definitions (can be in separate file, but exported from types.rs for convenience):
```rust
// At end of types.rs
pub use crate::almanac::dai_van::{
    Gender, ChieuThu, DaYunPillar, DaYunResult, ConventionMetadata
};
```

2. Add dai_van field to DayFortune (already shown above)

#### Component 3: almanac/mod.rs (MODIFIED)

**Changes:**
```rust
mod dai_van;  // New module
pub use dai_van::*;  // Export public types
```

#### Component 4: API Layer (MODIFIED)

**Files:**
- `crates/amlich-api/src/lib.rs` - Update exports
- `crates/amlich-api/src/convert.rs` - Add Dai Van conversion logic
- `crates/amlich-api/tests/almanac_contract.rs` - Add Dai Van contract tests

### 4. Data Flow Changes

#### Current Data Flow (v1.2)

```
Date Query (day, month, year)
    ↓
calculate_day_fortune()
    ↓
┌─────────────────────────────────────┐
│ Day-based calculations (v1.0)      │
│ - DayElement                      │
│ - Conflict, Travel, Stars         │
│ - Taboos, Xung/Hop, Truc        │
└─────────────────────────────────────┘
    ↓
┌─────────────────────────────────────┐
│ Optional: Ten Gods (v1.2)        │
│ Input: day_stem, year_stem        │
│ Output: DayTenGods                │
└─────────────────────────────────────┘
    ↓
┌─────────────────────────────────────┐
│ Optional: Kua (v1.2)            │
│ Input: birth_year, gender         │
│ Output: KuaResult                 │
└─────────────────────────────────────┘
    ↓
DayFortune { all fields }
```

#### New Data Flow (v1.3)

```
Date + Birth Query
  ├─ Date: day, month, year
  └─ Birth: birth_date, gender
         ↓
    calculate_day_fortune()
         ↓
┌─────────────────────────────────────┐
│ Day-based calculations (unchanged)  │
│ - DayElement, Conflict, etc.       │
└─────────────────────────────────────┘
         ↓
┌─────────────────────────────────────┐
│ Optional: Ten Gods (unchanged)    │
│ Input: day_stem, year_stem        │
│ Output: DayTenGods                │
└─────────────────────────────────────┘
         ↓
┌─────────────────────────────────────┐
│ Optional: Kua (unchanged)         │
│ Input: birth_year, gender         │
│ Output: KuaResult                 │
└─────────────────────────────────────┘
         ↓
┌─────────────────────────────────────┐
│ Optional: Dai Van (NEW)           │
│ Input: birth_date, gender        │
│         ↓                         │
│ Step 1: Lunar conversion          │
│         ↓                         │
│ Step 2: Year/month Can Chi        │
│         ↓                         │
│ Step 3: Chieuthu calculation     │
│         ↓                         │
│ Step 4: Days to nearest Tiet Khi │
│         ↓                         │
│ Step 5: Start age calculation    │
│         ↓                         │
│ Step 6: Generate 8 pillars      │
│         ↓                         │
│ Step 7: Ten Gods correlation     │
│   (calls thap_than::get_thap_than)│
│         ↓                         │
│ Output: DaYunResult              │
└─────────────────────────────────────┘
         ↓
    DayFortune { all fields including dai_van: Option<DaYunResult> }
```

#### Dependency Flow (Module Level)

```
dai_van.rs
    ├── uses: canchi.rs (get_year_canchi, get_month_canchi)
    ├── uses: tietkhi.rs (get_days_to_nearest_tiet_khi)
    ├── uses: lunar.rs (get_lunar_date)
    ├── uses: julian.rs (jd_from_date)
    ├── uses: thap_than.rs (get_thap_than) ← REUSE
    └── uses: tu_menh.rs (compute_kua) ← REUSE

calc.rs
    ├── calls: dai_van::calculate_dai_yun()
    ├── calls: thap_than::get_thap_than()
    ├── calls: tu_menh::compute_kua()
    └── assembles: DayFortune
```

### 5. Relationship with Ten Gods

**Approach:** Direct reuse of existing `get_thap_than()` function

**How it works:**
```rust
// In dai_van.rs pillar generation

for thu_tu in 1..=8 {
    // ... calculate pillar.can and pillar.chi ...

    // Correlate with day stem (if available)
    let ten_gods = day_stem.map(|ds| {
        thap_than::get_thap_than(ds, pillar.can)  // ← REUSE
    });

    pillars.push(DaYunPillar {
        thu_tu,
        start_age,
        end_age,
        can: pillar.can,
        chi: pillar.chi,
        can_chi_name,
        ten_gods,  // Option<ThapThanResult>
    });
}
```

**Integration points:**
- **Input:** Day stem from birth date (optional, depends on birth hour)
- **Call:** `thap_than::get_thap_than(day_stem, pillar.can)`
- **Output:** Ten Gods label, relation, polarity for each pillar
- **Reuse:** No modification to `thap_than.rs` required

**Example correlation:**
```
Day Stem: Giáp (甲)
Pillar 1 Can: Mậu (戊)
  → get_thap_than(Giáp, Mậu) = Thiên Tài (Day Generates Target)

Pillar 2 Can: Đinh (丁)
  → get_thap_than(Giáp, Đinh) = Thục Thần (Day Generates Target)
```

### 6. Relationship with Kua

**Approach:** Direct reuse of existing `compute_kua()` function

**How it works:**
```rust
// In dai_van.rs calculate_dai_yun()

let kua = tu_menh::compute_kua(birth_date.year(), gender);  // ← REUSE

// Can be used for directional analysis per pillar
// (optional enhancement for future milestone)
```

**Integration points:**
- **Input:** Birth year (from birth_date), gender
- **Call:** `tu_menh::compute_kua(birth_year, gender)`
- **Output:** Kua number, group, favorable/unfavorable directions
- **Reuse:** No modification to `tu_menh.rs` required

**Potential enhancement (future):**
- Correlate pillar elements with Kua directions
- Example: "Pillar 1 (Mộc) is favorable for your Kua 9 (East Group)"
- This would require additional logic beyond current scope

---

## Build Order and Dependencies

### Dependency Graph

```
dai_van.rs (NEW)
    ├── canchi.rs (exists)
    ├── tietkhi.rs (exists)
    ├── lunar.rs (exists)
    ├── julian.rs (exists)
    ├── thap_than.rs (exists, v1.2)
    └── tu_menh.rs (exists, v1.2)

calc.rs (MODIFIED)
    ├── dai_van.rs (NEW)
    ├── thap_than.rs (exists)
    └── tu_menh.rs (exists)

types.rs (MODIFIED)
    ├── dai_van.rs types (NEW)
    └── DayFortune (MODIFIED)

API Layer (MODIFIED)
    ├── calc.rs (MODIFIED)
    ├── types.rs (MODIFIED)
    └── dto.rs (NEW DTO types)

Tests (MODIFIED/NEW)
    ├── unit tests for dai_van.rs
    ├── integration tests for calc.rs
    └── contract tests for API
```

### Recommended Build Order

**Phase 1: Core Dai Van Module (can happen in parallel with nothing)**
```markdown
Priority: CRITICAL (blocks all other phases)

Tasks:
1. Create crates/amlich-core/src/almanac/dai_van.rs
2. Implement core types (Gender, ChieuThu, DaYunPillar, DaYunResult)
3. Implement 6-step calculation algorithm
4. Implement Ten Gods correlation (call thap_than::get_thap_than)
5. Implement helper functions (get_current_pillar, etc.)
6. Write comprehensive unit tests

Estimated effort: 12-16 hours
Dependencies: None (uses existing modules)

Deliverables:
- dai_van.rs module (400-600 LOC)
- Unit tests passing
- Manual verification with sample calculations
```

**Phase 2: Integration into calc.rs (blocks Phase 3)**
```markdown
Priority: CRITICAL (blocks API layer)

Tasks:
1. Update calculate_day_fortune() signature (add birth_date, birth_year, gender)
2. Update all call sites (test files, API conversion)
3. Add Dai Van computation logic
4. Simplify Kua computation using new parameters
5. Update Tu Mệnh computation in same refactor
6. Write integration tests

Estimated effort: 8-10 hours
Dependencies: Phase 1 complete

Deliverables:
- Updated calc.rs
- All call sites updated
- Integration tests passing
- No regressions in existing tests
```

**Phase 3: Type System Updates (blocks Phase 4)**
```markdown
Priority: HIGH (blocks API layer)

Tasks:
1. Add dai_van field to DayFortune struct
2. Add type re-exports in types.rs
3. Update almanac/mod.rs exports
4. Run type checker (cargo check)
5. Update serialization tests

Estimated effort: 4-6 hours
Dependencies: Phase 1, 2 complete

Deliverables:
- Updated types.rs
- Updated mod.rs
- Type-safe compilation
```

**Phase 4: API Layer Updates (blocks Phase 5)**
```markdown
Priority: HIGH (blocks CLI and consumers)

Tasks:
1. Create DaiYunResultDto and DaYunPillarDto in dto.rs
2. Add dai_van field to DayFortuneDto
3. Implement From<> conversion traits
4. Update API lib.rs exports
5. Update API contract tests

Estimated effort: 6-8 hours
Dependencies: Phase 1, 2, 3 complete

Deliverables:
- Updated dto.rs
- Updated convert.rs
- API contract tests passing
```

**Phase 5: Test Coverage and Validation (can happen after Phase 4)**
```markdown
Priority: MEDIUM (can be parallel with some Phase 4 work)

Tasks:
1. Write golden fixture tests for Dai Van (10+ cases)
2. Verify Ten Gods correlation for each pillar
3. Verify Kua integration
4. Run full test suite (cargo test --package amlich-core)
5. Run API tests (cargo test --package amlich-api)
6. Check for regressions in KHCBPPT validators

Estimated effort: 8-12 hours
Dependencies: Phase 1, 2, 3, 4 complete

Deliverables:
- Golden fixture file
- All tests passing
- Zero regressions
```

### What Can Happen in Parallel

**Parallel Track A: Test Fixtures**
- Can start after Phase 1 (need calculation logic to generate fixtures)
- Independent of Phase 2, 3, 4
- Can be developed while API layer work happens

**Parallel Track B: Documentation**
- Can start after Phase 1 (module documentation)
- Can continue through all phases
- Independent of implementation details

**Parallel Track C: Golden Dataset Validation**
- Can happen after Phase 5 (or manually in parallel)
- Depends only on calculation correctness (Phase 1)

### Critical Path (minimum time to production-ready)

```
Phase 1 (16h) → Phase 2 (10h) → Phase 3 (6h) → Phase 4 (8h) → Phase 5 (12h)
    │             │             │             │             │
    └─────────────┴─────────────┴─────────────┴─────────────┘
                    Sequential dependencies only
                    Total: 52 hours minimum

With parallel tracks:
- Parallel test fixtures: -8h (overlap with Phase 2-4)
- Parallel documentation: -4h (overlap with all phases)
- Effective total: ~40 hours
```

---

## Code Boundaries and Modularity

### Clear Boundaries

**Boundary 1: Dai Van Module (dai_van.rs)**
- **Responsibility:** Core Dai Van calculation only
- **Inputs:** birth_date, gender, optional day_stem
- **Outputs:** DaYunResult (pure calculation result)
- **Does NOT:**
  - ❌ Know about DayFortune structure
  - ❌ Handle API serialization
  - ❌ Manage business logic beyond calculation
- **Does:**
  - ✅ Call other calculation modules (canchi, tietkhi, thap_than, tu_menh)
  - ✅ Provide deterministic, testable functions
  - ✅ Include evidence metadata (convention)

**Boundary 2: Calculation Orchestration (calc.rs)**
- **Responsibility:** Assemble all calculations into DayFortune
- **Inputs:** Raw date components, optional birth context
- **Outputs:** Complete DayFortune struct
- **Does NOT:**
  - ❌ Implement calculation logic (delegates to modules)
  - ❌ Handle API concerns (serialization, DTOs)
- **Does:**
  - ✅ Call all calculation modules in correct order
  - ✅ Handle optional field population logic
  - ✅ Ensure backward compatibility

**Boundary 3: API Layer (amlich-api)**
- **Responsibility:** Public API surface and DTOs
- **Inputs:** Public API types (DateQuery, etc.)
- **Outputs:** Serialized DTOs (JSON)
- **Does NOT:**
  - ❌ Implement calculation logic
  - ❌ Know about internal module details
- **Does:**
  - ✅ Convert core types to DTOs
  - ✅ Handle API-specific concerns
  - ✅ Provide public interface

### Data Flow Across Boundaries

```
Public API (DateQuery + BirthContext)
    ↓
API Layer (convert.rs) → converts inputs
    ↓
calc.rs (calculate_day_fortune) → orchestrates
    ↓
    ├── dai_van.rs → pure calculation → DaYunResult
    ├── thap_than.rs → pure calculation → ThapThanResult
    ├── tu_menh.rs → pure calculation → KuaResult
    └── [other modules] → pure calculations
    ↓
calc.rs → assembles DayFortune
    ↓
API Layer (convert.rs) → converts DayFortune → DayFortuneDto
    ↓
Public API (JSON output)
```

### Separation of Concerns

| Concern | Owner | Location |
|---------|-------|----------|
| Calculation logic | Individual modules | dai_van.rs, thap_than.rs, tu_menh.rs, etc. |
| Orchestration | calc.rs | calc.rs |
| Type definitions | types.rs | types.rs |
| API contracts | amlich-api | dto.rs, lib.rs |
| Serialization | amlich-api | convert.rs |
| Testing | test files | tests/*.rs |

---

## Architectural Patterns

### Pattern 1: Optional Field Additive Integration

**What:** Adding new features to DayFortune as optional fields rather than breaking existing API

**When to use:**
- Adding features that require additional inputs
- Maintaining backward compatibility
- Extending existing output types

**Trade-offs:**
- ✅ Pros:
  - No breaking changes to existing clients
  - Single return type for all queries
  - Consistent with project "additive-only" decision
- ❌ Cons:
  - Function signatures can get long (many optional parameters)
  - Optional fields require runtime null checks in consumers
  - Type system doesn't enforce presence of required inputs

**Example (Dai Van):**
```rust
// Bad: Breaking change
pub fn calculate_day_fortune_with_dai_van(
    // ... new required parameters ...
) -> DayFortuneWithDaiVan

// Good: Additive integration
pub struct DayFortune {
    // ... existing fields ...

    // NEW: Optional field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dai_van: Option<DaYunResult>,
}

pub fn calculate_day_fortune(
    // ... existing parameters ...

    // NEW: Optional parameters
    birth_date: Option<NaiveDate>,
    birth_year: Option<i32>,
    gender: Option<Gender>,
) -> DayFortune {
    // Conditionally compute dai_van based on inputs
    dai_van: match (birth_date, gender) {
        (Some(bd), Some(g)) => Some(dai_van::calculate_dai_yun(bd, g, day_stem)),
        _ => None,
    },
}
```

**Pattern used by:** Ten Gods (v1.2), Kua (v1.2), Dai Van (v1.3)

### Pattern 2: Pure Calculation Modules

**What:** Isolate calculation logic in pure functions with no side effects

**When to use:**
- Core almanac calculations
- Deterministic, testable logic
- Functions that don't depend on external state

**Trade-offs:**
- ✅ Pros:
  - Easy to test (no mocking required)
  - Deterministic (same input = same output)
  - No hidden dependencies
  - Easy to reason about
- ❌ Cons:
  - Requires input/output struct definitions
  - More boilerplate (types, conversions)

**Example (Dai Van):**
```rust
// Pure function - no side effects, no I/O
pub fn calculate_dai_yun(
    birth_date: NaiveDate,
    gender: Gender,
    day_stem: Option<HeavenlyStem>,
) -> DaYunResult {
    // All calculations are deterministic
    // All dependencies are explicit (passed as parameters or called via functions)
    // No global state, no external I/O

    let lunar_date = get_lunar_date(...);  // Pure function
    let year_canchi = get_year_canchi(...);  // Pure function
    // ... etc ...

    DaYunResult { /* ... */ }
}

// Testable without mocking
#[test]
fn test_dai_yun_calculation() {
    let result = calculate_dai_yun(
        NaiveDate::from_ymd(1990, 3, 15),
        Gender::Male,
        Some(HeavenlyStem::Giap),
    );

    assert_eq!(result.chieu_thu, ChieuThu::Nghich);
    // ... more assertions ...
}
```

**Pattern used by:** All core calculation modules (canchi, tietkhi, thap_than, tu_menh, dai_van)

### Pattern 3: Evidence Metadata for Traceability

**What:** Every calculation result includes metadata about how it was computed (source, method, profile)

**When to use:**
- Auditability requirements
- Multiple calculation methods or sources
- Need to trace output to specific input rules

**Trade-offs:**
- ✅ Pros:
  - Full audit trail for every output
  - Debugging assistance
  - Verification against reference sources
- ❌ Cons:
  - More boilerplate (metadata structs)
  - Larger output size
  - Must be maintained with every calculation

**Example (Dai Van):**
```rust
pub struct ConventionMetadata {
    pub year_basis: String,           // "lunar" or "solar"
    pub start_age_method: String,      // "3-days-per-year"
    pub gender_encoding: String,       // "enum(Male,Female)"
    pub source_id: String,            // "khcbppt" or other
    pub method: String,               // "bai-quyet" or other
}

pub struct DaYunResult {
    // ... calculation results ...

    pub convention: ConventionMetadata,  // ← Evidence metadata
}

// Use in calculation
let convention = ConventionMetadata {
    year_basis: "lunar".to_string(),
    start_age_method: "3-days-per-year".to_string(),
    gender_encoding: "enum(Male,Female)".to_string(),
    source_id: "khcbppt".to_string(),  // Tracked for audit
    method: "bai-quyet".to_string(),    // Tracked for audit
};
```

**Pattern used by:** All major outputs (DayFortune, Ten Gods, Kua, Dai Van)

### Pattern 4: Module-Level Reuse Without Modification

**What:** New features call existing module functions directly without modifying those modules

**When to use:**
- Features that depend on existing calculations
- No need to modify existing calculation logic
- Existing module is stable and correct

**Trade-offs:**
- ✅ Pros:
  - No risk of breaking existing functionality
  - Clear module boundaries
  - Easier to maintain (changes isolated)
  - Existing tests continue to pass
- ❌ Cons:
  - Must work within existing API
  - May need wrapper functions if API doesn't fit exact need

**Example (Ten Gods in Dai Van):**
```rust
// thap_than.rs exists, is stable, tested
// NO modifications needed

// dai_van.rs reuses it directly
use crate::almanac::thap_than::get_thap_than;

pub fn calculate_dai_yun(
    birth_date: NaiveDate,
    gender: Gender,
    day_stem: Option<HeavenlyStem>,
) -> DaYunResult {
    // ... calculate pillars ...

    for pillar in &mut pillars {
        // Reuse existing function - NO modification to thap_than.rs
        pillar.ten_gods = day_stem.map(|ds| get_thap_than(ds, pillar.can));
    }

    // ...
}
```

**Pattern used by:**
- Dai Van → Ten Gods (get_thap_than)
- Dai Van → Kua (compute_kua)
- Any future features can reuse existing modules

---

## Scaling Considerations

### Current Scale Requirements

**Date range:** 1899-2100 (201 years)
**Queries:**
- Day-based almanac: ~73,000 possible dates
- Birth-based queries: ~73,000 × 2 genders = ~146,000 combinations
- Dai Van calculations: ~146,000 unique results

**Performance characteristics:**
- Day fortune calculation: < 1ms per date (measured in v1.2)
- Kua calculation: < 0.1ms per birth (measured in v1.2)
- Dai Van calculation: Estimated < 5ms per birth (8 pillars × Ten Gods correlation)

### No Scaling Required

**Rationale:**
- Almanac calculations are **stateless** (no database, no shared state)
- Calculations are **deterministic** (same input = same output)
- **Read-only** workload (no mutations)
- **Client-side** use case (API, CLI, not server-side serving millions)

**Scaling strategy if needed:**
1. **Caching:** Precompute and cache all Dai Van results (146k entries)
   - Cache key: birth_date + gender
   - Cache hit rate: 100% for repeated queries
   - Cache size: ~10-20 MB (estimated)

2. **Lazy computation:** Calculate on-demand only when requested
   - Date-only queries: No Dai Van computation
   - Birth queries: Compute Dai Van once, cache result

3. **Precomputation:** Generate all Dai Van results at build time
   - Create static lookup table
   - Include in binary (small enough)
   - Zero runtime cost for Dai Van

**Recommendation:** Start with lazy computation (no caching). Add caching if performance issues arise.

---

## Anti-Patterns to Avoid

### Anti-Pattern 1: Modifying Existing Calculation Modules

**What people do:** Modify `thap_than.rs` or `tu_menh.rs` to add Dai Van-specific logic

**Why it's wrong:**
- Breaks existing functionality and tests
- Violates module boundaries
- Creates tight coupling between features
- Makes debugging harder (changes not isolated)

**Do this instead:**
```rust
// BAD: Modifying thap_than.rs
impl ThapThanResult {
    pub fn to_dai_van_pillar(...) { /* Dai Van logic */ }
}

// GOOD: Using existing function from dai_van.rs
use crate::almanac::thap_than::get_thap_than;

pub fn calculate_dai_yun(...) -> DaYunResult {
    pillar.ten_gods = day_stem.map(|ds| get_thap_than(ds, pillar.can));
}
```

### Anti-Pattern 2: Creating Separate BirthFortune Type

**What people do:** Create new `BirthFortune` type separate from `DayFortune`

**Why it's wrong:**
- API fragmentation (clients need to know which type to request)
- Breaks unified query model
- More complex for existing consumers
- Requires two separate code paths (maintenance burden)

**Do this instead:**
```rust
// BAD: Separate type
pub struct BirthFortune {
    pub day_fortune: DayFortune,
    pub dai_van: DaYunResult,
}

// GOOD: Additive optional field
pub struct DayFortune {
    // ... existing fields ...
    pub dai_van: Option<DaYunResult>,  // Works for both cases
}

// Single function handles both cases
pub fn calculate_day_fortune(
    // ... parameters ...
    birth_date: Option<NaiveDate>,  // Optional
) -> DayFortune {
    dai_van: match (birth_date, gender) {
        (Some(bd), Some(g)) => Some(calculate_dai_yun(bd, g, day_stem)),
        _ => None,  // Date-only query
    },
}
```

### Anti-Pattern 3: Skipping Evidence Metadata

**What people do:** Omit `convention` metadata from Dai Van result to save time

**Why it's wrong:**
- Breaks auditability requirement
- Makes verification impossible
- Violates project pattern (all other features have evidence)
- Cannot trace output to source (KHCBPPT)

**Do this instead:**
```rust
// BAD: No metadata
pub struct DaYunResult {
    pub chieu_thu: ChieuThu,
    pub start_age: u8,
    pub pillars: Vec<DaYunPillar>,
    // Missing: convention
}

// GOOD: Include evidence
pub struct DaYunResult {
    pub chieu_thu: ChieuThu,
    pub start_age: u8,
    pub pillars: Vec<DaYunPillar>,
    pub convention: ConventionMetadata,  // Source, method, profile
}
```

### Anti-Pattern 4: Mixing Concerns in Calculation Logic

**What people do:** Add API serialization logic inside `dai_van.rs` calculation functions

**Why it's wrong:**
- Violates single responsibility principle
- Makes testing harder (need to mock serialization)
- Tight coupling between layers
- Cannot reuse calculation logic for other purposes (CLI, tests)

**Do this instead:**
```rust
// BAD: Serialization in calculation module
pub fn calculate_dai_yun(...) -> String {
    let result = DaYunResult { /* ... */ };
    serde_json::to_string(&result).unwrap()  // API concern here
}

// GOOD: Separation of concerns
pub fn calculate_dai_yun(...) -> DaYunResult {
    // Pure calculation only
    DaYunResult { /* ... */ }
}

// Serialization in API layer
impl From<DaYunResult> for DaYunResultDto {
    fn from(result: DaYunResult) -> Self {
        // Conversion logic here
    }
}
```

### Anti-Pattern 5: Hardcoding Gender and Date Types

**What people do:** Use `bool` for gender, custom date types instead of standard types

**Why it's wrong:**
- Type-unsafe (bool is unclear: true=male or true=female?)
- Non-standard (creates confusion)
- Harder to serialize (need custom serializers)
- Violates project conventions (use enums, chrono)

**Do this instead:**
```rust
// BAD: Non-standard types
pub fn calculate_dai_yun(
    birth_date: (u32, u32, u32),  // Tuple is unclear
    gender: bool,                   // Is true=male or true=female?
) -> DaYunResult

// GOOD: Standard types
pub fn calculate_dai_yun(
    birth_date: chrono::NaiveDate,  // Clear, standard
    gender: Gender,                 // Enum is clear
) -> DaYunResult

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Gender {
    Male,
    Female,
}
```

---

## Integration Points Summary

### Internal Module Dependencies

| Module | Depends On | Type of Dependency |
|--------|-----------|-------------------|
| `dai_van.rs` | `canchi.rs` | Function calls (get_year_canchi, get_month_canchi) |
| `dai_van.rs` | `tietkhi.rs` | Function call (get_days_to_nearest_tiet_khi) |
| `dai_van.rs` | `lunar.rs` | Function call (get_lunar_date) |
| `dai_van.rs` | `julian.rs` | Function call (jd_from_date) |
| `dai_van.rs` | `thap_than.rs` | Function call (get_thap_than) - REUSE |
| `dai_van.rs` | `tu_menh.rs` | Function call (compute_kua) - REUSE |
| `calc.rs` | `dai_van.rs` | Function call (calculate_dai_yun) |
| `calc.rs` | `thap_than.rs` | Function call (get_thap_than) |
| `calc.rs` | `tu_menh.rs` | Function call (compute_kua) |
| `convert.rs` | `calc.rs` | Uses DayFortune output |

### External Dependencies

| Dependency | Used By | Purpose |
|------------|----------|---------|
| `chrono` | `dai_van.rs`, `calc.rs` | Date types (NaiveDate) |
| `serde` | All types | Serialization (Serialize, Deserialize) |

### API Surface Changes

| File | Change | Type |
|------|---------|------|
| `calc.rs` | Function signature update | Breaking |
| `types.rs` | Add dai_van field | Additive |
| `dto.rs` | Add DaiYunResultDto, DaYunPillarDto | New |
| `convert.rs` | Add From<> implementations | Additive |
| `lib.rs` (core) | Export dai_van module | Additive |
| `lib.rs` (api) | Export new DTO types | Additive |

### Backward Compatibility

**What breaks:**
- ⚠️ `calculate_day_fortune()` signature change (breaking)
  - All call sites must be updated in coordinated PR
  - Impact: ~5-10 call sites in test files and API layer

**What's compatible:**
- ✅ DayFortune type is extended, not replaced (additive)
- ✅ JSON serialization is additive (new optional fields)
- ✅ Existing calculations are unchanged
- ✅ No changes to canchi, tietkhi, thap_than, tu_menh modules

---

## Sources

- **DAI_VAN_RESEARCH.md** (project research file) - Comprehensive Dai Van formulas and implementation details
- **v1.2-REQUIREMENTS.md** - Ten Gods and Kua integration patterns
- **crates/amlich-core/src/almanac/** - Existing implementation analysis (types.rs, calc.rs, thap_than.rs, tu_menh.rs)
- **crates/amlich-api/src/dto.rs** - API DTO patterns
- **Project decision record** - "Additive-only integration changes" (v1.2)

---

## Confidence Assessment

| Area | Confidence | Reason |
|------|------------|--------|
| Integration points | HIGH | Clear analysis of existing DayFortune patterns from v1.2 |
| New components | HIGH | Detailed specification from DAI_VAN_RESEARCH.md |
| Data flow changes | HIGH | Direct analysis of calc.rs and module dependencies |
| Ten Gods relationship | HIGH | Direct reuse pattern proven in v1.2 |
| Kua relationship | HIGH | Direct reuse pattern proven in v1.2 |
| Build order | HIGH | Clear dependency graph with sequential phases |
| Code boundaries | HIGH | Well-defined module responsibilities from existing code |
| Backward compatibility | HIGH | Breaking change identified and mitigated |

**Overall Confidence: HIGH**

**Key Assumptions:**
- KHCBPPT source verification will follow Dai Van implementation (as noted in research)
- No additional birth-based features planned for v1.3 beyond Dai Van
- Team accepts calculate_day_fortune() signature change as breaking but necessary

**Gaps:**
- None identified - all integration points analyzed

---

*Architecture research for: Dai Van Integration (v1.3 milestone)*
*Researched: 2026-03-03*
*Confidence: HIGH*
