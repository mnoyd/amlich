# Codebase Analysis: Engine Expansion Planning

**Date:** 2026-03-02
**Purpose:** Analyze current implementation and plan expansion with new subsystems

---

## Executive Summary

The current amlich implementation (`~6,000 LOC`) is **well-structured** with:
- ✅ Verified KHCBPPT alignment (v1.0 milestone complete)
- ✅ Clean separation of concerns (domain modules)
- ✅ Strong type safety with evidence tracking
- ✅ Comprehensive test coverage (184/184 passing)

The new subsystems from `vietnamese_lunar_engine_tables.md` can integrate smoothly with **minimal refactoring**.

---

## Current Implementation Architecture

### Core Modules (Verified ✓)

| Module | File | Purpose | Status |
|---------|-------|---------|---------|
| **Can Chi Core** | `types.ts`, `canchi.rs` | Year/Month/Day pillars | ✓ Complete |
| **Ngũ Hành** | `types.ts` | Five Elements | ✓ Complete |
| **Xung Hợp** | `xung_hop.rs` | Lục xung, Tam hợp, Tứ hành xung | ⚠ Partial |
| **Thần Hướng** | `than_huong.rs` | Travel directions | ✓ Complete |
| **Trực** | `truc.rs` | 12 Duty Officers | ✓ Complete |
| **Nạp Âm** | `data.rs` (baseline.json) | 60 pairs | ✓ Complete |
| **Stars** | `star.rs`, `than_sat.rs` | 28-star system | ✓ Complete |
| **Taboos** | `taboo.rs` | Tam Nương, Nguyệt Kỵ, Sát Chủ, Thọ Tử | ✓ Complete |
| **Day Deity** | `day_deity.rs` | Hoàng Đạo/Hắc Đạo | ✓ Complete |

### Data Flow

```
User Input (date)
    ↓
canchi.rs → Year/Month/Day Can Chi
    ↓
calc.rs → calculate_day_fortune()
    ├─→ star.rs → Day stars
    ├─→ taboo.rs → Taboo rules
    ├─→ day_deity.rs → Day deity
    ├─→ truc.rs → Trực officer
    ├─→ xung_hop.rs → Xung/hợp
    └─→ than_huong.rs → Travel directions
    ↓
DayFortune struct (JSON output)
```

### Type System Strengths

```rust
// Evidence tracking (excellent pattern)
pub struct RuleEvidence {
    pub source_id: String,      // "khcbppt", "tam-menh-thong-hoi"
    pub method: String,          // "table-lookup", "jd-cycle"
    pub profile: String,         // "baseline"
}

// All major types have evidence field
DayElement { evidence: Option<RuleEvidence> }
DayConflict { evidence: Option<RuleEvidence> }
TravelDirection { evidence: Option<RuleEvidence> }
DayStar { evidence: Option<RuleEvidence> }
```

**Key Pattern:** Every calculation result includes evidence → full audit trail ✓

---

## New Subsystems Analysis

### 1. Tàng Can (Hidden Stems) 🔶

**Complexity:** Medium
**Integration:** Low (extends existing Can Chi data)

**What It Is:**
- Each Địa Chi contains "hidden" (tàng) Heavenly Stems
- Structure: `[Chính (Main), Trung (Central), Dư (Residual)]`
- Strength: `[100, 60, 25, 15]` depending on Chi

**Implementation Path:**

```rust
// Add to types.rs
pub struct TangCan {
    pub main: &'static str,     // Chính
    pub central: &'static str,   // Trung
    pub residual: &'static str,  // Dư
    pub strength: [u8; 3],
}

// Add to baseline.json
"tang_can_meta": {
    "source_id": "khcbppt",    // Check KHCBPPT vols 1-2
    "method": "table-lookup"
}

"tang_can_by_chi": {
    "子": ["癸", "", ""],           // Tý - only Quý
    "丑": ["己", "癸", "辛"],       // Sửu - Kỷ chính, Quý trung, Tân dư
    // ... all 12 branches
}
```

**Integration Point:** Add to `calculate_day_fortune()` after day_canchi is resolved.

---

### 2. Thập Thần (Ten Gods) 🔶🔶

**Complexity:** High
**Integration:** High (requires Ngũ Hành relationship engine)

**What It Is:**
- Relationships between day Can (Heavenly Stem) and target Can/Chi
- 10 gods: Tỷ, Kiếp, Thực, Thương, Tài, Sát, Quan, Ấn
- Determined by: `day_can_wuxing × target_wuxing × yin_yang`

**Algorithm:**

```
For each relationship (day_can → target):
1. Compare Ngũ Hành (5 elements)
2. Check same_yinyang (both Dương or both Âm)
3. Map to Thập Thần name

Example:
Day Giáp (Mộc Dương) → Target Thổ (Âm):
- Mộc controls Thổ (Tôi khắc) → Thiên Tài
- Different yin/yang → Chính Tài
```

**Implementation Path:**

```rust
// New module: almanac/thap_than.rs

pub enum ThapThan {
    Ty, Kiep, Thuc, Thuong, Tai, Sat, Quan, An,
}

pub struct TenGods {
    pub day_gan_wuxing: &'static str,
    pub target_wuxing: &'static str,
    pub same_yinyang: bool,
    pub god: ThapThan,
    pub short_name: &'static str,
}

pub fn get_ten_god(
    day_gan_index: usize,
    target_wuxing: &str,
) -> TenGods {
    let day_wuxing = NGU_HANH_CAN[day_gan_index];
    let day_yinyang = is_yang_can(day_gan_index);
    let target_yinyang = is_yang_wuxing(target_wuxing);
    let same_yinyang = day_yinyang == target_yinyang;

    // 5×5 Ngũ Hành relationship matrix
    let god = match (day_wuxing, target_wuxing) {
        // Sinh ra tôi (我生 - Ấn)
        ("Mộc", "Thủy") | ("Hỏa", "Mộc") | ... => {
            if same_yinyang { ThapThan::An } else { ThapThan::ChinhAn }
        }
        // ... all 25 combinations
    };

    TenGods {
        day_gan_wuxing: day_wuxing,
        target_wuxing,
        same_yinyang,
        god,
        short_name: match god {
            ThapThan::Ty => "Tỷ Kiên",
            ThapThan::Tai => "Thiên Tài",
            // ...
        },
    }
}
```

**Integration Point:** Add to `DayFortune` struct as new field.

---

### 3. Enhanced Lục Xung/Tam Hợp 🔶

**Complexity:** Low
**Integration:** Very Low (extends existing `xung_hop.rs`)

**What's Missing:**

| Relationship | Current | Missing |
|-------------|----------|-----------|
| Lục xung | ✓ | - |
| Tam hợp | ✓ | - |
| Tứ hành xung | ✓ | - |
| **Lục hợp** (6 pairs) | ✗ | 子丑, 寅亥, 卯戌, 辰酉, 巳申, 午未 |
| **Tương hại** (6 pairs) | ✗ | 子未, 丑午, 寅酉, 卯申, 辰亥, 巳戌 |
| **Tương hình** (4 groups) | ✗ | 寅卯巳, 子辰丑, 申酉亥, 午午 |

**Implementation Path:**

```rust
// Extend xung_hop.rs

pub const LIUHE: [(usize, usize); 6] = [
    (0, 1),   // 子丑
    (2, 11),  // 寅亥
    (3, 10),  // 卯戌
    (4, 9),   // 辰酉
    (5, 8),   // 巳申
    (6, 7),   // 午未
];

pub const XIANGHAI: [(usize, usize); 6] = [
    (0, 7),   // 子未
    (1, 6),   // 丑午
    (2, 9),   // 寅酉
    (3, 8),   // 卯申
    (4, 11),  // 辰亥
    (5, 10),  // 巳戌
];

pub const XIANGXING: [[usize; 3]; 4] = [
    [2, 3, 5],   // 寅卯巳
    [0, 1, 4],   // 子辰丑
    [8, 9, 11],  // 申酉亥
    [6, 6, 6],   // 午午 (self-punishment)
];

// Update XungHopResult struct
pub struct XungHopResult {
    pub luc_xung: String,
    pub tam_hop: Vec<String>,
    pub tu_hanh_xung: Vec<String>,
    // NEW FIELDS:
    pub liu_he: Option<String>,
    pub xiang_hai: Option<String>,
    pub xiang_xing: Option<Vec<String>>,
}
```

**Integration Point:** Update `get_xung_hop()` to include all 6 relationship types.

---

### 4. Đông/Tây Tứ Mệnh (Eight Mansions/Kua) 🔶

**Complexity:** Medium
**Integration:** Medium (requires year, gender)

**What It Is:**
- 8 Kua numbers (1-9, excluding 5)
- Direction/Group classification: Đông Tứ vs Tây Tứ
- Calculation based on: Year + Gender

**Formula:**

```
Male:  (100 - year_last_two) % 9
Female: (year_last_two - 4) % 9

If remainder = 0 → Kua 9
```

**8 Kua Mapping:**

| Kua | Direction | Group |
|------|-----------|--------|
| 1 (Khảm) | Bắc | Đông Tứ |
| 2 (Khôn) | Tây Nam | Tây Tứ |
| 3 (Chấn) | Đông | Đông Tứ |
| 4 (Tốn) | Đông Nam | Đông Tứ |
| 5 (Trung) | - | Special case |
| 6 (Càn) | Tây Bắc | Tây Tứ |
| 7 (Đoài) | Tây | Tây Tứ |
| 8 (Cấn) | Đông Bắc | Tây Tứ |
| 9 (Ly) | Nam | Đông Tứ |

**Implementation Path:**

```rust
// New module: almanac/tu_menh.rs

pub enum KuaGroup {
    DongTu,  // East Group (1, 3, 4, 9)
    TayTu,   // West Group (2, 6, 7, 8)
}

pub struct KuaInfo {
    pub kua: usize,
    pub direction: &'static str,
    pub group: KuaGroup,
}

pub fn get_kua(year: i32, gender: char) -> KuaInfo {
    let year_last_two = year % 100;

    let remainder = match gender {
        'M' | 'm' => (100 - year_last_two) % 9,
        'F' | 'f' => (year_last_two - 4) % 9,
        _ => return KuaInfo { kua: 0, direction: "", group: KuaGroup::DongTu },
    };

    let kua = if remainder == 0 { 9 } else { remainder };

    match kua {
        1 => KuaInfo { kua, direction: "Bắc", group: KuaGroup::DongTu },
        2 => KuaInfo { kua, direction: "Tây Nam", group: KuaGroup::TayTu },
        3 => KuaInfo { kua, direction: "Đông", group: KuaGroup::DongTu },
        4 => KuaInfo { kua, direction: "Đông Nam", group: KuaGroup::DongTu },
        5 => panic!("Kua 5 is invalid - special case"),
        6 => KuaInfo { kua, direction: "Tây Bắc", group: KuaGroup::TayTu },
        7 => KuaInfo { kua, direction: "Tây", group: KuaGroup::TayTu },
        8 => KuaInfo { kua, direction: "Đông Bắc", group: KuaGroup::TayTu },
        9 => KuaInfo { kua, direction: "Nam", group: KuaGroup::DongTu },
        _ => KuaInfo { kua: 0, direction: "", group: KuaGroup::DongTu },
    }
}
```

**Integration Point:** Add to `calculate_day_fortune()` OR as separate birth calculation.

---

### 5. Đại Vận (Major Luck) 🔶🔶🔶

**Complexity:** Very High
**Integration:** Very High (requires birth date, gender, Tiết Khí)

**What It Is:**
- 10-year luck cycles (8 trụ typically)
- Each trụ has its own Can Chi pillar
- Direction: Thuận (forward +1) or Nghịch (backward -1)
- Start age calculated from birth date to nearest Tiết Khí

**Algorithm:**

```
1. Determine order (Thuận/Nghịch):
   Yang Year + Male → Thuận (+)
   Yang Year + Female → Nghịch (-)
   Yin Year + Female → Thuận (+)
   Yin Year + Male → Nghịch (-)

2. Calculate start age:
   Days from birth to nearest Tiết Khí / 3
   (3 days = 1 year)

3. Generate 8 trụ (10 years each = 80 years):
   Starting from month Can Chi pillar
   Add/subtract 1 stem/branch per trụ
```

**Implementation Path:**

```rust
// New module: almanac/dai_van.rs

pub struct DaYun {
    pub order: i32,        // 1 or -1
    pub start_age: i32,    // Age when Đại Vận starts
    pub pillars: Vec<CanChi>, // 8 trụ × 10 years
}

pub struct DaYunPillar {
    pub trụ_index: i32,     // 1-8
    pub start_age: i32,
    pub end_age: i32,
    pub can_chi: CanChi,
}

pub fn calculate_dai_yun(
    birth_date: chrono::NaiveDate,
    gender: char,
) -> DaYun {
    // 1. Get year chi and determine yang/yin
    let lunar_year = get_lunar_year(birth_date);
    let year_chi = get_year_canchi(lunar_year);
    let is_yang_year = year_chi.chi_index % 2 == 0;

    // 2. Determine order
    let order = match (is_yang_year, gender) {
        (true, 'M') | (false, 'F') => 1,  // Thuận
        _ => -1,                            // Nghịch
    };

    // 3. Get birth to nearest Tiết Khí days
    let birth_jd = date_to_jd(birth_date);
    let tiet_khi_days = get_days_to_nearest_tiet_khi(birth_jd);

    // 4. Calculate start age
    let start_age = tiet_khi_days / 3;

    // 5. Get month pillar as starting point
    let month_pillar = get_month_canchi(birth_date);

    // 6. Generate 8 trụ
    let mut pillars = Vec::new();
    let mut current_can = month_pillar.can_index;
    let mut current_chi = month_pillar.chi_index;

    for i in 0..8 {
        // Add order (+1 or -1)
        current_can = (current_can as i32 + order + 10) as usize % 10;
        current_chi = (current_chi as i32 + order + 12) as usize % 12;

        let pillar = CanChi::new(current_can, current_chi);
        let trụ = DaYunPillar {
            trụ_index: i + 1,
            start_age: start_age + (i * 10),
            end_age: start_age + ((i + 1) * 10),
            can_chi: pillar,
        };
        pillars.push(trụ);
    }

    DaYun {
        order,
        start_age,
        pillars,
    }
}
```

**Integration Point:** Major new feature - separate API endpoint or birth calculation flow.

---

### 6. Tiết Khí (Solar Terms) ✅

**Complexity:** Low
**Integration:** Very Low (already exists)

**Current Status:**
- ✅ `tietkhi.rs` module exists
- ✅ 24 solar terms defined
- ✅ JD-based calculation working

**What's Needed:**
- Nothing! Already implemented ✓
- May need to expose `get_days_to_nearest_tiet_khi()` for Đại Vận

**Implementation Path:**

```rust
// Add to tietkhi.rs
pub fn get_days_to_nearest_tiet_khi(jd: i32) -> i32 {
    // Find nearest Tiết Khí before and after JD
    // Return days to nearest (signed)
    // For Đại Vận calculation
}
```

---

## Integration Strategy

### Recommended Phasing

#### Phase 1: Foundation Extensions (Low-Medium complexity)
**Goal:** Extend existing subsystems with missing relationships

| Subsystem | Complexity | Effort | Dependencies |
|-----------|-------------|----------|--------------|
| Enhanced Xung Hợp (Lục hợp, Tương hại, Tương hình) | Low | 1-2 hours | Existing xung_hop.rs |
| Tàng Can (Hidden Stems) | Medium | 2-3 hours | New constants |
| Tiết Khí helper (days to nearest) | Low | 1 hour | Existing tietkhi.rs |

**Output:**
- Updated `XungHopResult` with 3 new relationship types
- `tang_can.rs` module with constants
- `get_days_to_nearest_tiet_khi()` function

---

#### Phase 2: Advanced Calculations (High complexity)
**Goal:** Add Thập Thần and Tứ Mệnh engines

| Subsystem | Complexity | Effort | Dependencies |
|-----------|-------------|----------|--------------|
| Thập Thần (Ten Gods) | High | 4-6 hours | Ngũ Hành system |
| Tứ Mệnh (Kua) | Medium | 2-3 hours | Year + Gender |

**Output:**
- `thap_than.rs` module with 10-god relationship engine
- `tu_menh.rs` module with Kua calculation
- Integration into `DayFortune` struct

---

#### Phase 3: Luck Cycle Engine (Very High complexity)
**Goal:** Implement Đại Vận system

| Subsystem | Complexity | Effort | Dependencies |
|-----------|-------------|----------|--------------|
| Đại Vận (Major Luck) | Very High | 8-12 hours | Tiết Khí, Month pillar, Year chi |

**Output:**
- `dai_van.rs` module
- `DaYun` struct with 8 trụ
- Birth calculation API

---

## Data Structure Updates

### Required Changes to `baseline.json`

```json
{
  // NEW
  "tang_can_meta": {
    "source_id": "khcbppt",
    "method": "table-lookup"
  },
  "tang_can_by_chi": {
    "子": ["癸", "", ""],
    "丑": ["己", "癸", "辛"],
    // ... all 12 branches
  },

  // EXTEND xung_hop
  "xung_hop_meta": {
    "source_id": "khcbppt",
    "method": "table-lookup",
    "relationships": ["luc_xung", "tam_hop", "tu_hanh_xung", "liu_he", "xiang_hai", "xiang_xing"]
  },

  // NEW (Thap Than may not need data - purely algorithmic)
  "thap_than_meta": {
    "source_id": "universal",  // Not KHCBPPT-specific
    "method": "algorithmic"
  },

  // NEW (Tu Menh is purely algorithmic - no data needed)
  "tu_menh_meta": {
    "source_id": "universal",
    "method": "formula-based"
  },

  // NEW (Dai Van is purely algorithmic - no data needed)
  "dai_van_meta": {
    "source_id": "universal",
    "method": "formula-based"
  }
}
```

### Required Changes to `DayFortune` struct

```rust
pub struct DayFortune {
    // EXISTING FIELDS...
    ruleset_id: String,
    ruleset_version: String,
    profile: String,
    day_element: DayElement,
    conflict: DayConflict,
    travel: TravelDirection,
    stars: DayStars,
    day_deity: Option<DayDeity>,
    taboos: Vec<DayTaboo>,
    xung_hop: XungHopResult,
    truc: TrucInfo,

    // NEW FIELDS:
    pub tang_can: Option<TangCan>,           // Phase 1
    pub ten_god: Option<TenGods>,           // Phase 2
    pub kua: Option<KuaInfo>,                // Phase 2 (birth-level)
    pub dai_yun: Option<DaYun>,            // Phase 3 (birth-level)
}
```

---

## Testing Strategy

### Follow v1.0 Pattern

1. **Golden Dataset Generation**
   - Create golden entries with expected values for new subsystems
   - Reference sources: KHCBPPT for Tàng Can, universal for others

2. **Validator Tests**
   - One test file per new subsystem
   - `cargo test --test khcbppt_tang_can`
   - `cargo test --test khcbppt_thap_than`
   - `cargo test --test khcbppt_tu_menh`
   - `cargo test --test khcbppt_dai_van`

3. **Integration Tests**
   - Verify new subsystems populate in `calculate_day_fortune()`
   - Check JSON serialization includes new fields

### Edge Cases to Test

| Subsystem | Edge Cases |
|-----------|-------------|
| Tàng Can | All 12 branches, empty strings for missing stems |
| Thập Thần | All 25 Ngũ Hành combinations, yin/yang differences |
| Enhanced Xung Hợp | Self-punishment (Ngọ Ngọ), all 6 relationship types |
| Tứ Mệnh | Year ending in 00, Kua 5 (invalid), both genders |
| Đại Vận | Leap years, birth near Tiết Khí boundary, both orders |

---

## Risk Assessment

### Low Risk ✅
- Enhanced Xung Hợp: Simple table lookups
- Tiết Khí helper: Extension of existing module

### Medium Risk ⚠️
- Tàng Can: Need KHCBPPT verification for hidden stem data
- Tứ Mệnh: Gender handling, edge cases (year ending in 00)

### High Risk 🔶
- Thập Thần: Complex relationship matrix (5×5×2 combinations)
- Đại Vận: Most complex - birth date, gender, Tiết Khí integration

---

## Recommended Next Steps

### Option A: Incremental (Recommended)
1. **Phase 1** (Foundation): Enhanced Xung Hợp + Tàng Can + Tiết Khí helper
2. **Phase 2** (Advanced): Thập Thần + Tứ Mệnh
3. **Phase 3** (Luck Cycles): Đại Vận

**Pros:**
- Each phase is manageable (3-6 hours)
- Can validate each subsystem independently
- Follows v1.0 pattern (small, focused plans)

**Cons:**
- 3 separate milestones
- More milestone overhead

---

### Option B: Single Big Milestone
**Milestone v1.1: Complete Engine Expansion** - All 6 subsystems in one milestone

**Pros:**
- Clear scope
- One validation pass
- Faster delivery (if no blockers)

**Cons:**
- Large scope (15-25 hours of work)
- Risk of overwhelm
- Harder to debug issues

---

## Final Recommendation

**I strongly recommend Option A (Incremental)** with this structure:

### Milestone v1.1: Foundation Extensions
**Goal:** Complete Xung Hợp relationships, add Tàng Can, Tiết Khí helpers

**Plans:**
1. Add enhanced Xung Hợp (Lục hợp, Tương hại, Tương hình)
2. Add Tàng Can module and data
3. Add Tiết Khí helper functions

**Estimated:** 3-5 hours total

---

### Milestone v1.2: Advanced Astrological Calculations
**Goal:** Implement Thập Thần engine and Tứ Mệnh calculations

**Plans:**
1. Implement Thập Thần (Ten Gods) relationship engine
2. Implement Tứ Mệnh (Kua) calculation system
3. Integrate both into DayFortune struct

**Estimated:** 6-9 hours total

---

### Milestone v1.3: Luck Cycle System
**Goal:** Implement Đại Vận (Major Luck) with 8-trụ calculation

**Plans:**
1. Implement Đại Vận core algorithm
2. Add birth-to-Tiết Khí calculation
3. Generate 8 trụ with order determination
4. Create DaiYun struct and tests

**Estimated:** 8-12 hours total

---

## Summary

**Current State:** Solid foundation (v1.0 complete, 184/184 tests passing)

**New Subsystems:** 6 major features ready for implementation

**Recommended Approach:** 3 incremental milestones (v1.1, v1.2, v1.3)

**Total Estimated Effort:** 17-26 hours across 3 milestones

**Confidence:** High - all subsystems are well-specified with clear algorithms

---

*Analysis completed: 2026-03-02*
*Next: User decision on approach (Option A vs Option B)*
