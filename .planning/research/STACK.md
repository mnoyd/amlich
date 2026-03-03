# Technology Stack

**Project:** amlich v1.3 - Dai Van Core
**Researched:** 2026-03-03
**Overall confidence:** HIGH

## Recommended Stack

### Core Framework
| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| Rust workspace (`edition = 2021`) | existing baseline | Implement Dai Van calculation logic in `amlich-core` | Already project's correctness-critical execution layer; no FFI/language boundary needed for deterministic Dai Van rule engine. |
| `serde` | 1.0 (workspace) | Serialize/deserialize Dai Van types + evidence metadata | Existing DTO/JSON contract already depends on serde; adding Dai Van fields follows established patterns. |
| `serde_json` | 1.0 (workspace) | Fixture loading and golden-style testing for Dai Van | Existing test strategy uses JSON fixtures; reuse keeps tests consistent and auditable. |
| `chrono` | 0.4 (workspace) | Birth date handling and Tiết Khí distance calculation | Already available in workspace; date conversion and day difference logic needed for start age calculation. |

### Database
| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| None | — | Dai Van is computed, not stored | Calculation is deterministic from birth date + gender; no persistence needed. |

### Infrastructure
| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| None | — | No new infrastructure | Use existing Rust module structure; Dai Van computed on-demand from birth inputs. |

### Supporting Libraries
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| None required | — | All dependencies exist | Use standard Rust + existing workspace dependencies only. |

## Stack Additions/Changes for Dai Van Integration

### 1) New Data Structures in `amlich-core`

**Create module:** `crates/amlich-core/src/almanac/dai_van.rs`

**Add to `almanac/types.rs`:**
```rust
/// Chiều (Direction) of Đại Vân progression
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChieuThu {
    Thuan,   // Forward (+1)
    Nghich,  // Backward (-1)
}

impl ChieuThu {
    pub fn to_i32(&self) -> i32 {
        match self {
            ChieuThu::Thuan => 1,
            ChieuThu::Nghich => -1,
        }
    }
}

/// Individual Đại Vân pillar (10-year period)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DaYunPillar {
    /// Pillar number (1-8)
    pub thu_tu: u8,
    /// Age when this pillar begins
    pub start_age: u8,
    /// Age when this pillar ends
    pub end_age: u8,
    /// Heavenly Stem (Can)
    pub can: HeavenlyStem,
    /// Earthly Branch (Chi)
    pub chi: EarthlyBranch,
    /// Full Can Chi name in Vietnamese (e.g., "Giáp Tý")
    pub can_chi_name: String,
    /// Optional: Ten Gods relationship to Day Stem
    pub ten_gods: Option<crate::almanac::thap_than::ThapThanResult>,
}

/// Complete Đại Vân calculation result
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DaYunResult {
    /// Chiều of progression (Thuận or Nghịch)
    pub chieu_thu: ChieuThu,
    /// Age when first Đại Vân pillar begins
    pub start_age: u8,
    /// Number of pillars generated (typically 8)
    pub num_pillars: u8,
    /// All 8 trụ (pillars)
    pub pillars: Vec<DaYunPillar>,
    /// Convention metadata documenting calculation method
    pub convention: ConventionMetadata,
}

/// Convention metadata for Đại Vân calculation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConventionMetadata {
    /// Year basis convention used
    pub year_basis: String,
    /// How start age is calculated
    pub start_age_method: String,
    /// Gender encoding scheme
    pub gender_encoding: String,
    /// Source of calculation rules
    pub source_id: String,
    /// Method citation
    pub method: String,
}

impl ConventionMetadata {
    pub fn project_default() -> Self {
        Self {
            year_basis: "lunar".to_string(),
            start_age_method: "3-days-per-year".to_string(),
            gender_encoding: "enum(Male,Female)".to_string(),
            source_id: "khcbppt".to_string(),
            method: "bai-quyet".to_string(),
        }
    }
}
```

**Why these structures:**
- Follow existing patterns from `thap_than.rs` and `tu_menh.rs`
- Type-safe enums for ChieuThu (like KuaGroup)
- Comprehensive result structure with evidence metadata
- Optional Ten Gods integration in each pillar
- Convention metadata for auditability (matches existing RuleEvidence pattern)

### 2) Integration with Existing Systems

**Reuse existing types:**
- `Gender` enum: Already exists in `tu_menh.rs`, reuse directly
- `HeavenlyStem`, `EarthlyBranch`: Already in `types.rs`, use directly
- `ThapThanResult`: Already in `thap_than.rs`, embed as optional field

**Integration points:**
```rust
// In dai_van.rs - main calculation function
use crate::almanac::thap_than;
use crate::almanac::tu_menh;
use crate::canchi;      // For year/month Can Chi
use crate::tietkhi;    // For days to nearest solar term

pub fn calculate_dai_yun(
    birth_date: chrono::NaiveDate,
    gender: tu_menh::Gender,  // Reuse existing Gender enum
) -> DaYunResult {
    // Step 1: Get lunar date
    let lunar_date = get_lunar_date(birth_date);

    // Step 2: Get year Can Chi
    let year_canchi = canchi::get_year_canchi(lunar_date.lunar_year);
    let is_yang_year = year_canchi.chi_index % 2 == 0;

    // Step 3: Determine chieuthu (year polarity × gender)
    let chieu_thu = determine_chieuthu(is_yang_year, gender);

    // Step 4: Get month Can Chi (base pillar)
    let month_canchi = canchi::get_month_canchi(
        lunar_date.lunar_month,
        lunar_date.lunar_year,
        lunar_date.is_leap_month
    );

    // Step 5: Calculate days to nearest Tiết Khí
    let birth_jd = jd_from_date(birth_date);
    let days_to_tiet_khi = tietkhi::get_days_to_nearest_tiet_khi(birth_jd);

    // Step 6: Calculate start age
    let start_age = (days_to_tiet_khi.abs() / 3) as u8;

    // Step 7: Generate 8 pillars
    let pillars = generate_pillars(month_canchi, chieu_thu, start_age);

    // Step 8: Add Ten Gods correlation (optional, compute on demand)
    for pillar in &mut pillars {
        if let Some(day_stem) = get_birth_day_stem(birth_date) {
            pillar.ten_gods = Some(thap_than::get_thap_than(day_stem, pillar.can));
        }
    }

    DaYunResult { chieu_thu, start_age, num_pillars: 8, pillars, convention }
}
```

### 3) Computation Algorithms

**Algorithm 1: Year Polarity Determination**
```rust
fn is_yang_year(year_chi_index: usize) -> bool {
    // Yang years: Tý (0), Dần (2), Thìn (4), Ngọ (6), Thân (8), Tuất (10)
    year_chi_index % 2 == 0
}
```

**Algorithm 2: Chiều (Direction) Determination**
```rust
fn determine_chieuthu(is_yang_year: bool, gender: tu_menh::Gender) -> ChieuThu {
    match (is_yang_year, gender) {
        (true, tu_menh::Gender::Male) | (false, tu_menh::Gender::Female) => ChieuThu::Thuan,
        _ => ChieuThu::Nghich,
    }
}
```

**Algorithm 3: Start Age Calculation**
```rust
fn calculate_start_age(days_to_nearest_tiet_khi: i32) -> u8 {
    // 3 days = 1 year (standard conversion)
    (days_to_nearest_tiet_khi.abs() / 3) as u8
}
```

**Algorithm 4: Pillar Generation**
```rust
fn generate_pillars(
    month_pillar: CanChi,
    chieu_thu: i32,
    start_age: u8
) -> Vec<DaYunPillar> {
    let mut pillars = Vec::with_capacity(8);
    let mut current_stem = month_pillar.stem_index;
    let mut current_branch = month_pillar.branch_index;

    for thu_tu in 1..=8 {
        // Apply chieuthu (add +1 or -1)
        current_stem = (current_stem as i32 + chieu_thu + 10) as usize % 10;
        current_branch = (current_branch as i32 + chieu_thu + 12) as usize % 12;

        let pillar = DaYunPillar {
            thu_tu,
            start_age: start_age + (thu_tu - 1) * 10,
            end_age: start_age + thu_tu * 10,
            can: HeavenlyStem::from_index(current_stem),
            chi: EarthlyBranch::from_index(current_branch),
            can_chi_name: format!("{} {}",
                HeavenlyStem::VN_NAME[current_stem],
                EarthlyBranch::VN_NAME[current_branch]
            ),
            ten_gods: None,  // Calculate separately if needed
        };

        pillars.push(pillar);
    }

    pillars
}
```

**Why these algorithms:**
- Pure deterministic logic (no randomness or I/O)
- Modular and testable (each step is a pure function)
- Follows existing patterns (canchi, tietkhi modules)
- Leverages existing validated capabilities (Ten Gods, Kua)

### 4) External Dependencies

**No new dependencies required.**

All needed capabilities exist in the workspace:
- **Rust stdlib**: Core data structures, algorithms
- **serde**: Serialization for DTOs and fixtures
- **chrono**: Date handling for birth date and Tiết Khí calculations
- **Existing modules**:
  - `canchi`: Year/month Can Chi calculation
  - `tietkhi`: Days to nearest solar term
  - `thap_than`: Ten Gods correlation
  - `tu_menh`: Kua calculation and Gender enum

### 5) What NOT to Add

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| New dependencies (date crates, numerics, etc.) | Existing stack (chrono, serde) is sufficient; adds maintenance risk and complexity to correctness-critical milestone. | Use chrono for dates, Rust stdlib for algorithms. |
| BirthFortune API surface | Milestone context specifies "Dai Van period transitions, Ten Gods correlation, and Kua integration" - not new public API. | Add Dai Van as optional field to existing DayFortune, keep API minimal. |
| Non-deterministic runtime features | Weakens auditability for correctness milestone; adds complexity. | Pure deterministic calculation functions. |
| Database or persistence | Dai Van is computed on-demand from birth date + gender; no storage needed. | Compute when requested from birth inputs. |
| Breaking DTO changes | Violates backward compatibility; breaks existing consumers. | Add optional fields with `#[serde(skip_serializing_if = "Option::is_none")]`. |
| External rule engines or scripting | Adds non-deterministic surface; harder to audit. | Compile-time Rust mapping logic with explicit evidence metadata. |

## Alternatives Considered

| Category | Recommended | Alternative | Why Not |
|----------|-------------|-------------|---------|
| API Design | Optional field in DayFortune | Separate BirthFortune API | Milestone scope is Dai Van integration, not new API surface; optional field keeps changes minimal and backward-compatible. |
| Ten Gods Integration | Lazy calculation on demand | Pre-calculate all 8 pillars | Most users only need current pillar; lazy calculation reduces unnecessary computation. |
| Kua Integration | Analyze pillar elements against Kua directions | Store Kua in each pillar | Kua is birth-level, not pillar-level; analyze on-demand without bloating data structure. |

## Installation

```bash
# No new packages to install - use existing workspace dependencies

# Run tests for Dai Van module
cargo test --package amlich-core --lib almanac::dai_van

# Run integration tests with existing subsystems
cargo test --package amlich-core --lib dai_van_integration

# Verify no regressions in existing tests
cargo test --package amlich-core
```

## Integration with Existing Types

### Extend `DayFortune` in `almanac/types.rs`

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DayFortune {
    // ... existing fields ...

    /// Đại Vân (Major Luck) result (populated only when birth date and gender provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dai_van: Option<super::dai_van::DaYunResult>,
}
```

### Extend `DayFortuneDto` in `amlich-api/src/dto.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayFortuneDto {
    // ... existing fields ...

    /// Đại Vân (Major Luck) result (populated only when birth date and gender provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dai_van: Option<DaYunResultDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaYunResultDto {
    pub chieu_thu: String,
    pub start_age: u8,
    pub num_pillars: u8,
    pub pillars: Vec<DaYunPillarDto>,
    pub convention: ConventionMetadataDto,
}

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
```

## Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chieuthu_rules() {
        // Yang year + Male = Thuan
        assert_eq!(determine_chieuthu(true, tu_menh::Gender::Male), ChieuThu::Thuan);
        // Yang year + Female = Nghich
        assert_eq!(determine_chieuthu(true, tu_menh::Gender::Female), ChieuThu::Nghich);
        // Yin year + Male = Nghich
        assert_eq!(determine_chieuthu(false, tu_menh::Gender::Male), ChieuThu::Nghich);
        // Yin year + Female = Thuan
        assert_eq!(determine_chieuthu(false, tu_menh::Gender::Female), ChieuThu::Thuan);
    }

    #[test]
    fn test_pillar_progression() {
        // Verify stems and branches advance correctly by chieuthu
        // Test both Thuan (+1) and Nghich (-1) directions
    }

    #[test]
    fn test_ten_gods_integration() {
        // Verify Ten Gods calculation for each pillar
        let birth_date = chrono::NaiveDate::from_ymd(1990, 3, 15);
        let result = calculate_dai_yun(birth_date, tu_menh::Gender::Male);

        for pillar in &result.pillars {
            assert!(pillar.ten_gods.is_some(), "Each pillar should have Ten Gods");
        }
    }
}
```

### Golden Fixture Tests
Create `crates/amlich-core/tests/fixtures/dai_van_fixtures.json`:
```json
{
  "fixtures": [
    {
      "id": "dv_001",
      "description": "Male born 1990, Yin year case",
      "input": {
        "birth_date": "1990-03-15",
        "gender": "Male"
      },
      "expected": {
        "chieu_thu": "nghich",
        "start_age": 2,
        "num_pillars": 8,
        "pillars": [...]
      }
    }
  ]
}
```

## Sources

- Workspace dependencies: `/home/noy/Work/junks/amlich/Cargo.toml` (HIGH)
- Core crate structure: `/home/noy/Work/junks/amlich/crates/amlich-core/Cargo.toml`, `src/almanac/mod.rs` (HIGH)
- Existing Ten Gods implementation: `/home/noy/Work/junks/amlich/crates/amlich-core/src/almanac/thap_than.rs` (HIGH)
- Existing Kua implementation: `/home/noy/Work/junks/amlich/crates/amlich-core/src/almanac/tu_menh.rs` (HIGH)
- Existing DTO patterns: `/home/noy/Work/junks/amlich/crates/amlich-api/src/dto.rs` (HIGH)
- Existing types: `/home/noy/Work/junks/amlich/crates/amlich-core/src/almanac/types.rs` (HIGH)
- Dai Van research: `/home/noy/Work/junks/amlich/.planning/research/DAI_VAN_RESEARCH.md` (HIGH)
- Milestone context: `/home/noy/Work/junks/amlich/.planning/PROJECT.md` (HIGH)

---
*Stack research for: v1.3 Dai Van Core*
*Researched: 2026-03-03*
