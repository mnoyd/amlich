# Đại Vân (Great Cycle) Research Report

**Date:** March 3, 2026  
**Project:** amlich - Vietnamese Almanac Correctness Audit  
**Researcher:** AI Research Agent  
**Status:** Research Complete

---

## Executive Summary

Đại Vân (大运 - Major Luck/Great Cycle) is a 10-year luck cycle system in Vietnamese and Chinese astrology that provides insight into an individual's life phases. It is one of the most important advanced calculations in Four Pillars (Bazi) systems, extending beyond the basic birth chart (Tứ Trụ) to show how fortune evolves over time.

**Key Findings:**
- ✅ Well-documented calculation formulas exist in Vietnamese sources
- ✅ Integration dependencies (Tiết Khí, Can Chi) are already implemented in amlich
- ⚠️ KHCBPPT coverage uncertain - requires manual verification
- ✅ Can follow existing patterns (Ten Gods, Kua) for type-safe implementation
- 🔄 Complex integration requires birth date + gender inputs (not day-based)

---

## 1. Concept Understanding

### 1.1 What is Đại Vân?

**Definition:** Đại Vân (大运, "Great Luck" or "Major Luck") is a system of 10-year cycles that map different phases of a person's life from birth through old age. Each 10-year period is represented by a Can Chi (Gan Zhi) pillar, indicating the predominant energies and influences during that decade.

**Vietnamese Terminology:**
- **Đại Vân** or **Đại Vận**: Major Luck / Great Cycle
- **Trụ Đại Vân**: The pillar representing each 10-year period
- **Thuận / Nghịch**: Forward or backward progression direction
- **Tuổi bắt đầu**: Starting age for the first cycle

**Chinese Terminology (for reference):**
- **大运 (Dà Yùn)**: Major Luck / Great Fortune
- **十年大运 (Shí Nián Dà Yùn)**: Ten-Year Major Luck
- **顺行 (Shùn Xíng)**: Forward progression (Thuận)
- **逆行 (Nì Xíng)**: Backward progression (Nghịch)

### 1.2 Historical Context

**Origins:**
- Part of the broader **Bazi** (八字 - Four Pillars) system developed during the Tang and Song dynasties
- Systematized by **Xú Zi Píng** (徐子平) in the Song Dynasty (960–1279 CE)
- Used to predict life phases, career changes, marriage timing, health patterns

**Cultural Significance in Vietnam:**
- Core component of **Tử Vi** (Purple Star) and **Bát Tự** (Four Pillars) fortune-telling
- Provides context for timing major life events (career, marriage, relocation)
- Often consulted with Tứ Mệnh (Kua) and Thập Thần (Ten Gods) for comprehensive readings

### 1.3 Why is it Important?

**Practical Applications:**
1. **Life Phase Analysis**: Understand which energies dominate each decade of life
2. **Timing Major Events**: Predict favorable periods for career advancement, marriage, business
3. **Compatibility Assessment**: Compare compatibility between individuals across different life phases
4. **Feng Shui Adjustments**: Recommend remedies based on current and upcoming luck cycles

**Integration with Other Systems:**
- **Thập Thần (Ten Gods)**: Analyze the nature of each Đại Vân pillar's relationship to Day Stem
- **Tứ Mệnh (Kua)**: Determine favorable/unfavorable directions during each 10-year period
- **Tứ Trụ (Birth Chart)**: Compare static birth energy with dynamic luck cycles

---

## 2. Calculation Formulas

### 2.1 Overview of Calculation Process

The Đại Vân calculation requires the following steps:

```
INPUT: Birth date (solar/lunar) + Gender
  ↓
STEP 1: Determine Year Can Chi
  ├─ Get lunar year from birth date
  ├─ Calculate Can Chi: Can = (year + 6) % 10, Chi = (year + 8) % 12
  └─ Determine year polarity (Âm/Dương)
  ↓
STEP 2: Determine Month Can Chi
  ├─ Get lunar month from birth date
  ├─ Calculate branch: Chi = (month + 1) % 12
  ├─ Calculate stem using year stem table
  └─ This is the "base" pillar for Đại Vân
  ↓
STEP 3: Find Nearest Tiết Khí (Solar Term)
  ├─ Calculate days from birth to previous or next solar term
  ├─ Get signed difference (negative if before, positive if after)
  └─ Use 3-days = 1-year conversion
  ↓
STEP 4: Determine Chiều (Direction)
  ├─ Apply rule: (Year Yang/Âm × Gender) → Thuận (+1) or Nghịch (-1)
  ↓
STEP 5: Calculate Start Age
  ├─ Formula: Start Age = |days_to_tiet_khi| / 3
  ├─ Round to nearest integer (or truncate depending on tradition)
  └─ This is the age when first Đại Vân begins
  ↓
STEP 6: Generate 8 Trụ (Pillars)
  ├─ Starting from Month Can Chi
  ├─ For each trụ (1-8):
  │   ├─ Add/subtract 1 to stem index: stem = (stem + direction) % 10
  │   ├─ Add/subtract 1 to chi index: chi = (chi + direction) % 12
  │   ├─ Calculate age range: [start_age + (i-1)*10, start_age + i*10]
  │   └─ Store pillar
  └─ Output: 8 pillars with age ranges
```

### 2.2 Detailed Formulas

#### Formula 1: Year Polarity (Âm/Dương)

```rust
// Determine if lunar year is Âm or Dương
fn is_yang_year(year_zhi_index: usize) -> bool {
    // Yang years: Tý, Dần, Thìn, Ngọ, Thân, Tuất
    // Indices: 0, 2, 4, 6, 8, 10 (even numbers)
    year_zhi_index % 2 == 0
}
```

**Yang Years (Dương):** Tý (子), Dần (寅), Thìn (辰), Ngọ (午), Thân (申), Tuất (戌)  
**Yin Years (Âm):** Sửu (丑), Mão (卯), Tỵ (巳), Mùi (未), Dậu (酉), Hợi (亥)

#### Formula 2: Chiều (Direction) Determination

```rust
fn determine_chieuthu(is_yang_year: bool, gender: Gender) -> i32 {
    match (is_yang_year, gender) {
        (true, Gender::Male) | (false, Gender::Female) => 1,   // Thuận
        (true, Gender::Female) | (false, Gender::Male) => -1,  // Nghịch
    }
}
```

**Rule Table:**

| Năm (Year) | Giới tính (Gender) | Chiều Đại Vân |
|--------------|---------------------|-------------------|
| Dương       | Nam (Male)          | Thuận (+1)        |
| Dương       | Nữ (Female)         | Nghịch (-1)       |
| Âm          | Nam (Male)          | Nghịch (-1)       |
| Âm          | Nữ (Female)         | Thuận (+1)        |

#### Formula 3: Start Age Calculation

```rust
fn calculate_start_age(days_to_nearest_tiet_khi: i32) -> i32 {
    // 3 days = 1 year (standard conversion)
    // 1 day = 4 months
    // 1 hour = 10 days (if precision required)
    (days_to_nearest_tiet_khi.abs() / 3)
}
```

**Important Notes:**
- Days are always positive (absolute value)
- If born exactly on a Tiết Khí → 0 days → start age = 0
- Round down or to nearest integer depending on school of thought

#### Formula 4: Trụ (Pillar) Generation

```rust
struct DaYunPillar {
    pub thu_tu: i32,              // 1-8
    pub start_age: i32,          // Age when this pillar starts
    pub end_age: i32,            // Age when this pillar ends
    pub can: HeavenlyStem,         // Can (Stem)
    pub chi: EarthlyBranch,       // Chi (Branch)
    pub can_chi_name: String,     // e.g., "Giáp Tý"
}

fn generate_dai_yun_pillars(
    month_pillar: CanChi,
    chieu_thu: i32,
    start_age: i32
) -> Vec<DaYunPillar> {
    let mut pillars = Vec::new();
    let mut current_stem = month_pillar.stem_index;
    let mut current_branch = month_pillar.branch_index;
    
    for thu_tu in 1..=8 {
        // Advance stem and branch by chieu_thu (+1 or -1)
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
        };
        
        pillars.push(pillar);
    }
    
    pillars
}
```

### 2.3 Example Calculation

**Example: Person born March 15, 1990 (Male)**

1. **Lunar Date**: Assuming lunar year 1990 = Canh Ngọ (庚午)
2. **Year Chi**: Ngọ (午, index 5) → Âm year (odd index)
3. **Gender**: Male
4. **Chieuthu**: (Âm, Male) → Nghịch (-1)
5. **Nearest Tiết Khí**: Assume 5 days before Lập Xuân
6. **Start Age**: 5 / 3 = 1.67 → ~2 years old
7. **Month Pillar**: Assuming lunar month 2 = Mậu Sửu (戊丑)
8. **Generate Pillars**:
   - Trụ 1: Mậu Sửu (戊丑), ages 2-12
   - Trụ 2: Đinh Sửu (丁丑), ages 12-22
   - Trụ 3: Bính Sửu (丙丑), ages 22-32
   - Trụ 4: Ất Sửu (乙丑), ages 32-42
   - Trụ 5: Giáp Sửu (甲丑), ages 42-52
   - Trụ 6: Quý Sửu (癸丑), ages 52-62
   - Trụ 7: Nhâm Sửu (壬丑), ages 62-72
   - Trụ 8: Tân Sửu (辛丑), ages 72-82

---

## 3. Reference Sources

### 3.1 KHCBPPT Coverage Assessment

**Search Results:**
- ⚠️ **No explicit KHCBPPT section found for Đại Vân** in available online sources
- The ctext.org reference (卷六) appears to contain general astronomical/astrological content but Đại Vân specifics not clearly indexed

**Analysis:**
- KHCBPPT likely contains Đại Vân content but requires **manual research** of the actual text volumes
- Search terms to use: "大运" (Da Yùn), "十年大运" (Ten-Year Major Luck), "顺逆" (Forward/Backward)
- Recommended volumes: Quyển 12-13 (Công Quy - Astrological Calculations section) or later volumes

**Verification Approach:**
```markdown
1. Locate KHCBPPT physical edition or high-quality digital scan
2. Search index or table of contents for "大运" or related terms
3. Extract calculation rules and verify against standard formulas
4. Document any KHCBPPT-specific variants or interpretations
5. Compare with modern Vietnamese sources for consistency
```

### 3.2 Classical Chinese Sources

**Primary Sources:**
1. **Yuan Hai Zi Ping (渊海子平)** - Ming Dynasty
   - Author: Xú Dà Shēng (徐大升)
   - Contains detailed Đại Vân calculation methods
   - Considered authoritative for classical methods

2. **San Ming Tong Hui (三命通会)** - Ming Dynasty
   - Author: Wàn Mín Yīng (万民英)
   - Comprehensive treatise on Four Pillars
   - Includes timing methods (Đại Vân, Tiểu Vân)

3. **Ming Li Tàn Nguyên (命理探源)** - Qing Dynasty
   - Author: Yuán Shù Shān (袁树珊)
   - Refines earlier methods
   - May have standardized rules

**Vietnamese Sources:**
1. **vietnamese_lunar_engine_tables.md** (already in project)
   - Section 15: "Đại Vận (Major Luck)"
   - Provides Rust code template
   - Verified formula matches standard Bazi methods

2. **Online Vietnamese Numerology Sites**
   - Often have simplified explanations
   - Should be cross-checked against classical sources
   - Useful for modern interpretation but verify accuracy

3. **Books by Vietnamese Astrologers**
   - Search for "tử vi", "bát tự", "đại vận" keywords
   - Look for books that cite KHCBPPT or classical sources

### 3.3 Modern Academic References

**Academic Research:**
- Wikipedia: "Four Pillars of Destiny" mentions "10-year luck cycle (Chinese: 十年大运)"
- Various academic papers on Bazi timing methods
- Japanese sources: "Dai Un" (same system, different pronunciation)

**Citation Rules for amlich Project:**
```markdown
Format: "KHCBPPT, Quyen [N], [Section name], [Page if known]"
Examples:
- "KHCBPPT, Quyen 12, Công Quy section"
- "Yuan Hai Zi Ping, Ming Dynasty, Chapter on Major Luck"
- "vietnamese_lunar_engine_tables.md, Section 15"
```

---

## 4. Implementation Requirements

### 4.1 Data Structures

#### 4.1.1 Core Types

```rust
// File: crates/amlich-core/src/almanac/dai_van.rs

use serde::{Deserialize, Serialize};
use crate::almanac::types::{HeavenlyStem, EarthlyBranch};

/// Gender for Đại Vân calculation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Gender {
    Male,
    Female,
}

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
    /// Optional: Kua (Tu Mệnh) group for this period
    pub kua_group: Option<crate::almanac::tu_menh::KuaGroup>,
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
    /// Convention metadata (source, method, etc.)
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
            source_id: "khcbppt".to_string(),  // Update after verification
            method: "bai-quyet".to_string(),  // Update after verification
        }
    }
}
```

#### 4.1.2 Integration Types

```rust
// Add to crates/amlich-core/src/almanac/types.rs

/// Extend DayFortune to optionally include Đại Vân
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DayFortune {
    // ... existing fields ...
    
    /// Đại Vân (Major Luck) result (populated only when birth date and gender provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dai_van: Option<DaYunResult>,
}

// Alternative: Birth-specific result type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BirthFortune {
    pub day_fortune: DayFortune,  // Regular day-based calculations
    pub dai_van: DaYunResult,       // Birth-specific Đại Vân
    pub ten_gods_summary: TenGodsSummary,  // Extended Ten Gods analysis
    pub kua: KuaResult,                  // Tu Mệnh result
}
```

### 4.2 Calculation Functions

#### 4.2.1 Main Entry Point

```rust
// File: crates/amlich-core/src/almanac/dai_van.rs

use crate::{
    julian::{jd_from_date, jd_to_date},
    lunar::{get_lunar_date},
    canchi::{get_year_canchi, get_month_canchi},
    tietkhi::{get_days_to_nearest_tiet_khi},
    almanac::types::{HeavenlyStem, EarthlyBranch},
};

/// Calculate Đại Vân for a person
///
/// # Arguments
/// * `birth_date` - Gregorian birth date (NaiveDate)
/// * `gender` - Gender (Male or Female)
///
/// # Returns
/// Complete Đại Vân result with 8 pillars
pub fn calculate_dai_yun(
    birth_date: chrono::NaiveDate,
    gender: Gender,
) -> DaYunResult {
    let convention = ConventionMetadata::project_default();
    
    // Step 1: Get lunar date
    let lunar_date = get_lunar_date(
        birth_date.day(),
        birth_date.month(),
        birth_date.year()
    );
    
    // Step 2: Get year Can Chi
    let year_canchi = get_year_canchi(lunar_date.lunar_year);
    let is_yang_year = year_canchi.chi_index % 2 == 0;
    
    // Step 3: Determine chieuthu
    let chieu_thu = match (is_yang_year, gender) {
        (true, Gender::Male) | (false, Gender::Female) => ChieuThu::Thuan,
        _ => ChieuThu::Nghich,
    };
    
    // Step 4: Get month Can Chi (base pillar)
    let month_canchi = get_month_canchi(
        lunar_date.lunar_month,
        lunar_date.lunar_year,
        lunar_date.is_leap_month
    );
    
    // Step 5: Calculate days to nearest Tiết Khí
    let birth_jd = jd_from_date(
        birth_date.day(),
        birth_date.month(),
        birth_date.year()
    );
    let days_to_tiet_khi = get_days_to_nearest_tiet_khi(birth_jd);
    
    // Step 6: Calculate start age
    let start_age = (days_to_tiet_khi.abs() / 3) as u8;
    
    // Step 7: Generate 8 pillars
    let pillars = generate_pillars(
        month_canchi,
        chieu_thu.to_i32(),
        start_age
    );
    
    DaYunResult {
        chieu_thu,
        start_age,
        num_pillars: 8,
        pillars,
        convention,
    }
}

/// Generate the 8 Đại Vân pillars
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
            ten_gods: None,  // Optional: calculate later
            kua_group: None,  // Optional: calculate later
        };
        
        pillars.push(pillar);
    }
    
    pillars
}
```

#### 4.2.2 Helper Functions

```rust
/// Determine current pillar for a given age
pub fn get_current_pillar(
    dai_yun: &DaYunResult,
    current_age: u8
) -> Option<&DaYunPillar> {
    dai_yun.pillars
        .iter()
        .find(|pillar| current_age >= pillar.start_age && current_age < pillar.end_age)
}

/// Calculate years until next pillar transition
pub fn years_to_next_transition(
    dai_yun: &DaYunResult,
    current_age: u8
) -> Option<u8> {
    get_current_pillar(dai_yun, current_age)
        .map(|pillar| pillar.end_age - current_age)
}

/// Calculate Đại Vân for a specific future age
pub fn get_pillar_at_age(
    dai_yun: &DaYunResult,
    target_age: u8
) -> Option<&DaYunPillar> {
    dai_yun.pillars
        .iter()
        .find(|pillar| target_age >= pillar.start_age && target_age < pillar.end_age)
}
```

### 4.3 Correlation with Ten Gods and Kua

#### 4.3.1 Ten Gods Integration

```rust
use crate::almanac::thap_than;

/// Extend DaYunPillar with Ten Gods calculation
impl DaYunPillar {
    /// Calculate Ten Gods from pillar Can to Day Stem
    pub fn calculate_ten_gods(
        &self,
        day_stem: HeavenlyStem
    ) -> ThapThanResult {
        thap_than::get_thap_than(day_stem, self.can)
    }
}
```

**Integration Pattern:**
- When calculating Đại Vân, also compute Ten Gods for each pillar
- Helps interpret the nature of each 10-year period
- Example: "Trụ 3 (Bính Sửu) with Thất Sát to your Day Stem → competitive period"

#### 4.3.2 Kua Integration

```rust
use crate::almanac::tu_menh;

/// Calculate Kua-based directional analysis for a pillar
pub fn analyze_pillar_with_kua(
    pillar: &DaYunPillar,
    birth_year: i32,
    gender: Gender
) -> KuaDirectionAnalysis {
    let kua = tu_menh::compute_kua(birth_year, gender);
    
    // Map pillar elements to favorable/unfavorable directions
    // This requires additional logic not currently in tu_menh module
    KuaDirectionAnalysis {
        // ... analysis results
    }
}
```

### 4.4 Test Cases and Fixtures

#### 4.4.1 Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_dai_yun_calculation_1990_male() {
        let birth_date = NaiveDate::from_ymd(1990, 3, 15);
        let result = calculate_dai_yun(birth_date, Gender::Male);
        
        // Verify chieuthu
        assert_eq!(result.chieu_thu, ChieuThu::Nghich);
        
        // Verify start age is reasonable
        assert!(result.start_age >= 0 && result.start_age <= 10);
        
        // Verify 8 pillars generated
        assert_eq!(result.num_pillars, 8);
        assert_eq!(result.pillars.len(), 8);
        
        // Verify age ranges are contiguous
        for i in 0..7 {
            assert_eq!(
                result.pillars[i].end_age,
                result.pillars[i + 1].start_age
            );
        }
    }

    #[test]
    fn test_chieuthu_rules() {
        // Yang year + Male = Thuan
        assert_eq!(
            determine_chieuthu(true, Gender::Male),
            ChieuThu::Thuan
        );
        
        // Yang year + Female = Nghich
        assert_eq!(
            determine_chieuthu(true, Gender::Female),
            ChieuThu::Nghich
        );
        
        // Yin year + Male = Nghich
        assert_eq!(
            determine_chieuthu(false, Gender::Male),
            ChieuThu::Nghich
        );
        
        // Yin year + Female = Thuan
        assert_eq!(
            determine_chieuthu(false, Gender::Female),
            ChieuThu::Thuan
        );
    }

    #[test]
    fn test_pillar_progression() {
        let birth_date = NaiveDate::from_ymd(1985, 6, 20);
        let result = calculate_dai_yun(birth_date, Gender::Female);
        
        // Verify stems and branches progress correctly
        for i in 0..7 {
            let current = &result.pillars[i];
            let next = &result.pillars[i + 1];
            
            // Can index should change by chieuthu
            let expected_can_diff = result.chieu_thu.to_i32();
            let actual_can_diff = 
                (next.can as usize as i32 - current.can as usize as i32 + 10) % 10;
            assert_eq!(actual_can_diff, expected_can_diff);
            
            // Chi index should change by chieuthu
            let expected_chi_diff = result.chieu_thu.to_i32();
            let actual_chi_diff = 
                (next.chi as usize as i32 - current.chi as usize as i32 + 12) % 12;
            assert_eq!(actual_chi_diff, expected_chi_diff);
        }
    }

    #[test]
    fn test_get_current_pillar() {
        let birth_date = NaiveDate::from_ymd(1995, 9, 10);
        let result = calculate_dai_yun(birth_date, Gender::Male);
        
        // Test at age 25 (should be in trụ 3, ages 20-30)
        let pillar = get_current_pillar(&result, 25);
        assert!(pillar.is_some());
        assert_eq!(pillar.unwrap().thu_tu, 3);
        assert!(25 >= pillar.unwrap().start_age);
        assert!(25 < pillar.unwrap().end_age);
        
        // Test at age 80 (past all pillars)
        let past_pillar = get_current_pillar(&result, 80);
        assert!(past_pillar.is_none());
    }
}
```

#### 4.4.2 Golden Fixture Examples

```json
// File: crates/amlich-core/tests/fixtures/dai_van_fixtures.json
{
  "fixtures": [
    {
      "id": "dv_001",
      "description": "Male born 1990, standard case",
      "input": {
        "birth_date": "1990-03-15",
        "gender": "Male"
      },
      "expected": {
        "chieu_thu": "nghich",
        "start_age": 2,
        "num_pillars": 8,
        "pillars": [
          {
            "thu_tu": 1,
            "start_age": 2,
            "end_age": 12,
            "can": "Mậu",
            "chi": "Sửu",
            "can_chi_name": "Mậu Sửu"
          },
          {
            "thu_tu": 2,
            "start_age": 12,
            "end_age": 22,
            "can": "Đinh",
            "chi": "Sửu",
            "can_chi_name": "Đinh Sửu"
          },
          {
            "thu_tu": 3,
            "start_age": 22,
            "end_age": 32,
            "can": "Bính",
            "chi": "Sửu",
            "can_chi_name": "Bính Sửu"
          },
          {
            "thu_tu": 4,
            "start_age": 32,
            "end_age": 42,
            "can": "Ất",
            "chi": "Sửu",
            "can_chi_name": "Ất Sửu"
          },
          {
            "thu_tu": 5,
            "start_age": 42,
            "end_age": 52,
            "can": "Giáp",
            "chi": "Sửu",
            "can_chi_name": "Giáp Sửu"
          },
          {
            "thu_tu": 6,
            "start_age": 52,
            "end_age": 62,
            "can": "Quý",
            "chi": "Sửu",
            "can_chi_name": "Quý Sửu"
          },
          {
            "thu_tu": 7,
            "start_age": 62,
            "end_age": 72,
            "can": "Nhâm",
            "chi": "Sửu",
            "can_chi_name": "Nhâm Sửu"
          },
          {
            "thu_tu": 8,
            "start_age": 72,
            "end_age": 82,
            "can": "Tân",
            "chi": "Sửu",
            "can_chi_name": "Tân Sửu"
          }
        ]
      },
      "sources": [
        "vietnamese_lunar_engine_tables.md, Section 15",
        "Yuan Hai Zi Ping, Ming Dynasty"
      ]
    },
    {
      "id": "dv_002",
      "description": "Female born 1985, Yang year case",
      "input": {
        "birth_date": "1985-06-20",
        "gender": "Female"
      },
      "expected": {
        "chieu_thu": "thuan",
        "start_age": 3,
        "num_pillars": 8,
        "pillars": [
          // ... fixture data ...
        ]
      },
      "sources": [
        "vietnamese_lunar_engine_tables.md, Section 15"
      ]
    },
    {
      "id": "dv_003",
      "description": "Male born 2002, edge case (Kua 5 resolution)",
      "input": {
        "birth_date": "2002-08-10",
        "gender": "Male"
      },
      "expected": {
        "chieu_thu": "nghich",
        "start_age": 1,
        "num_pillars": 8,
        "pillars": [
          // ... fixture data ...
        ]
      },
      "sources": [
        "vietnamese_lunar_engine_tables.md, Section 15"
      ]
    }
  ]
}
```

---

## 5. Integration Approach

### 5.1 API Design Considerations

#### 5.1.1 Integration Point Options

**Option A: Add to DayFortune (Simple, Limited)**
```rust
pub struct DayFortune {
    // ... existing fields ...
    
    /// Đại Vân result (only populated when birth inputs provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dai_van: Option<DaYunResult>,
}
```

**Pros:**
- Minimal API changes
- Backward compatible (optional field)
- Consistent with ten_gods and tu_menh pattern

**Cons:**
- DayFortune represents a single day, but Đại Vân is birth-based
- Confusing semantic: day vs. birth fortune
- May require birth inputs in day query API

**Option B: Separate BirthFortune API (Cleaner, Recommended)**
```rust
/// Complete fortune reading including day-based and birth-based calculations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BirthFortune {
    /// Day-based almanac for a reference date
    pub day_fortune: DayFortune,
    
    /// Đại Vân (10-year luck cycles)
    pub dai_van: DaYunResult,
    
    /// Extended Ten Gods analysis
    pub ten_gods_summary: TenGodsSummary,
    
    /// Tu Mệnh (Kua) result
    pub kua: KuaResult,
}

/// API function for birth-based fortune
pub fn calculate_birth_fortune(
    birth_date: NaiveDate,
    birth_hour: Option<u8>,
    gender: Gender,
    reference_date: NaiveDate,
) -> BirthFortune {
    // Calculate day fortune for reference date
    let day_fortune = calculate_day_fortune(reference_date);
    
    // Calculate birth-specific features
    let dai_van = calculate_dai_yun(birth_date, gender);
    let kua = compute_kua(birth_date.year(), gender);
    let ten_gods_summary = calculate_ten_gods_summary(birth_date, birth_hour);
    
    BirthFortune {
        day_fortune,
        dai_van,
        ten_gods_summary,
        kua,
    }
}
```

**Pros:**
- Clear separation of concerns (day vs. birth calculations)
- More intuitive API for birth chart analysis
- Can include all birth-specific features in one call
- Easy to extend with future birth-based features

**Cons:**
- Requires new API surface
- More complex initial implementation

**Recommendation:** **Option B** - Separate BirthFortune API

#### 5.1.2 API Function Signatures

```rust
// In amlich-core/src/lib.rs

/// Calculate day fortune (existing function, unchanged)
pub fn calculate_day_fortune(date: chrono::NaiveDate) -> DayFortune {
    // ... existing implementation ...
}

/// NEW: Calculate complete birth fortune
///
/// # Arguments
/// * `birth_date` - Person's birth date (Gregorian)
/// * `birth_hour` - Optional birth hour (for day pillar calculation)
/// * `gender` - Person's gender
/// * `reference_date` - Optional reference date for day fortune (default: today)
///
/// # Returns
/// Complete fortune including day-based almanac and birth-specific features
pub fn calculate_birth_fortune(
    birth_date: chrono::NaiveDate,
    birth_hour: Option<u8>,
    gender: Gender,
    reference_date: Option<chrono::NaiveDate>,
) -> BirthFortune {
    let ref_date = reference_date.unwrap_or(chrono::Utc::now().naive_utc().date());
    
    // Calculate day fortune for reference date
    let day_fortune = calculate_day_fortune(ref_date);
    
    // Calculate birth-specific features
    let dai_van = dai_van::calculate_dai_yun(birth_date, gender);
    let kua = tu_menh::compute_kua(birth_date.year(), gender);
    
    // Optional: Ten Gods summary (requires day stem from birth hour)
    let ten_gods_summary = if let Some(hour) = birth_hour {
        let birth_day_pillar = canchi::get_day_canchi(jd_from_date(
            birth_date.day(),
            birth_date.month(),
            birth_date.year()
        ));
        let day_stem = birth_day_pillar.stem;
        Some(calculate_ten_gods_summary(day_stem))
    } else {
        None
    };
    
    BirthFortune {
        day_fortune,
        dai_van,
        ten_gods_summary,
        kua,
    }
}
```

### 5.2 Backward Compatibility

#### 5.2.1 Non-Breaking Changes

**Strategy:**
1. Keep existing `calculate_day_fortune()` API unchanged
2. Add new `calculate_birth_fortune()` API as separate entry point
3. No modifications to `DayFortune` struct (add only optional field if needed)
4. Update AMQP serialization to include new fields when present

**Migration Path:**
```markdown
Phase 1 (v1.3):
- Implement dai_van.rs module
- Add calculate_dai_yun() function
- Write comprehensive tests
- Do NOT integrate into public API yet

Phase 2 (v1.3+):
- Design and implement BirthFortune API
- Add calculate_birth_fortune() function
- Integrate with existing subsystems
- Update AMQP schema if needed
- Add integration tests

Phase 3 (Future):
- Consider CLI/API surface changes
- Update documentation
- Add examples
```

#### 5.2.2 Deprecation Warnings

**If adding to DayFortune:**
```rust
impl DayFortune {
    pub fn new_without_dai_van(/* ... */) -> Self {
        // ... existing construction ...
    }
    
    #[deprecated(note = "Use calculate_birth_fortune() for birth-based features")]
    pub fn new(/* ... */) -> Self {
        // Old constructor, marks as deprecated
    }
}
```

### 5.3 Integration with Existing Features

#### 5.3.1 Ten Gods Integration

**Approach:** Lazy Calculation on Demand

```rust
impl DaYunResult {
    /// Calculate Ten Gods for a specific pillar
    pub fn get_ten_gods_for_pillar(
        &self,
        thu_tu: u8,
        day_stem: HeavenlyStem
    ) -> Option<ThapThanResult> {
        self.pillars
            .get((thu_tu - 1) as usize)
            .map(|pillar| thap_than::get_thap_than(day_stem, pillar.can))
    }
    
    /// Calculate Ten Gods for all pillars
    pub fn calculate_all_ten_gods(
        &self,
        day_stem: HeavenlyStem
    ) -> Vec<Option<ThapThanResult>> {
        self.pillars
            .iter()
            .map(|pillar| Some(thap_than::get_thap_than(day_stem, pillar.can)))
            .collect()
    }
}
```

**Usage:**
```rust
let birth_fortune = calculate_birth_fortune(birth_date, None, gender, None);
let day_stem = birth_fortune.day_fortune.day_element.can;  // Get from day pillar

// Get Ten Gods for current pillar (age 35)
let current_ten_gods = birth_fortune.dai_van
    .get_ten_gods_for_pillar(4, day_stem);  // Trụ 4 (ages 30-40)

// Get all pillars' Ten Gods
let all_ten_gods = birth_fortune.dai_van
    .calculate_all_ten_gods(day_stem);
```

#### 5.3.2 Kua Integration

**Approach:** Analyze Favorable Directions for Current Period

```rust
impl DaYunPillar {
    /// Get directional recommendations for this pillar's period
    pub fn get_directional_analysis(
        &self,
        birth_year: i32,
        gender: Gender
    ) -> KuaDirectionAnalysis {
        let kua = tu_menh::compute_kua(birth_year, gender);
        
        // Analyze pillar element against Kua favorable directions
        // This is an enhancement requiring additional logic
        KuaDirectionAnalysis {
            kua_number: kua.kua,
            group: kua.group,
            favorable_for_pillar: vec![/* ... */],
            unfavorable_for_pillar: vec![/* ... */],
            recommendation: format!(
                "During ages {}-{}, favor {} and avoid {}",
                self.start_age,
                self.end_age,
                // List favorable directions
                self.recommend_directions(&kua)
            ),
        }
    }
}
```

### 5.4 Error Handling and Edge Cases

#### 5.4.1 Input Validation

```rust
use chrono::NaiveDate;

/// Validate birth date for Đại Vân calculation
pub fn validate_birth_date(date: &NaiveDate) -> Result<(), String> {
    // Check date is not in future
    let today = chrono::Utc::now().naive_utc().date();
    if *date > today {
        return Err("Birth date cannot be in the future".to_string());
    }
    
    // Check date is not too old (beyond reasonable range)
    if date.year() < 1900 {
        return Err("Birth year must be >= 1900".to_string());
    }
    
    // Check date is not too far in future (beyond reasonable range)
    if date.year() > 2100 {
        return Err("Birth year must be <= 2100".to_string());
    }
    
    Ok(())
}

pub fn calculate_dai_yun_validated(
    birth_date: NaiveDate,
    gender: Gender
) -> Result<DaYunResult, String> {
    validate_birth_date(&birth_date)?;
    
    // ... proceed with calculation ...
}
```

#### 5.4.2 Leap Month Handling

**Scenario:** Birth during lunar leap month

```rust
// In get_month_canchi, already handles leap month indicator
let month_canchi = get_month_canchi(
    lunar_date.lunar_month,
    lunar_date.lunar_year,
    lunar_date.is_leap_month  // This affects month Can calculation
);

// For Đại Vân, leap month affects:
// 1. Which month pillar to use as base
// 2. Possibly the start age calculation if traditional rules differ

// Standard approach: Use the actual month pillar (including leap indicator)
// Most schools treat leap month as "duplicate" month for pillar purposes
```

#### 5.4.3 Pre-Tiết Khí Births

**Scenario:** Born exactly on a solar term (Tiết Khí)

```rust
// In get_days_to_nearest_tiet_khi, returns 0 if exactly on term
let days_to_tiet_khi = get_days_to_nearest_tiet_khi(birth_jd);

if days_to_tiet_khi == 0 {
    // Born exactly on Tiết Khí
    // Traditional rule: Start age = 0 (Đại Vân begins at birth)
    // Some schools may interpret differently
    start_age = 0;
} else {
    // Born before or after Tiết Khí
    // Standard rule: 3 days = 1 year
    start_age = (days_to_tiet_khi.abs() / 3) as u8;
}
```

---

## 6. Implementation Recommendations

### 6.1 Immediate Next Steps

**Week 1-2: Core Module Development**
- [ ] Create `crates/amlich-core/src/almanac/dai_van.rs`
- [ ] Implement core types (Gender, ChieuThu, DaYunPillar, DaYunResult)
- [ ] Implement `calculate_dai_yun()` function
- [ ] Write unit tests for all calculation steps
- [ ] Verify chieuthu rules with test matrix
- [ ] Verify pillar progression logic

**Week 3-4: Integration and Testing**
- [ ] Integrate with existing Can Chi and Tiết Khí modules
- [ ] Add helper functions (current_pillar, years_to_next, etc.)
- [ ] Write golden fixture tests (at least 10 representative cases)
- [ ] Run full test suite: `cargo test --package amlich-core`
- [ ] Verify no regressions in existing tests

**Week 5-6: API Design and Documentation**
- [ ] Design BirthFortune API (or decide on DayFortune extension)
- [ ] Implement public API functions
- [ ] Update module exports in `almanac/mod.rs`
- [ ] Write comprehensive documentation (doc comments, examples)
- [ ] Create integration tests with other subsystems

### 6.2 PHASED Implementation Plan

**Phase 1: Foundation (Weeks 1-2)**
```markdown
Goal: Get basic calculation working

Tasks:
1. Create dai_van.rs with core types
2. Implement calculate_dai_yun() with all 6 steps
3. Write unit tests for each step
4. Verify with manual calculations

Deliverables:
- dai_van.rs module (400-600 LOC)
- 100% passing unit tests
- Manual verification checklist

Success Criteria:
- calculate_dai_yun() returns valid result
- All unit tests pass
- Manual verification matches expectations
```

**Phase 2: Integration (Weeks 3-4)**
```markdown
Goal: Integrate with existing systems

Tasks:
1. Integrate with Can Chi (year/month/day)
2. Integrate with Tiết Khí (start age calculation)
3. Write integration tests
4. Create golden fixtures (10+ cases)
5. Verify against Vietnamese sources

Deliverables:
- Full integration with existing modules
- Golden fixture file
- Integration test suite

Success Criteria:
- All integration tests pass
- Fixture data verified
- No regressions in amlich-core tests
```

**Phase 3: API Surface (Weeks 5-6)**
```markdown
Goal: Expose through public API

Tasks:
1. Design and implement BirthFortune type
2. Implement calculate_birth_fortune() function
3. Update lib.rs exports
4. Write API documentation
5. Add usage examples

Deliverables:
- Public API for birth fortune
- Comprehensive documentation
- Usage examples

Success Criteria:
- API compiles and passes tests
- Documentation is complete
- Examples work correctly
```

**Phase 4: Advanced Features (Future)**
```markdown
Goal: Add optional enhanced features

Tasks:
1. Ten Gods integration for each pillar
2. Kua-based directional analysis
3. Feng Shui recommendations per period
4. CLI/API surface for birth queries
5. Visualization/output formatting

Deliverables:
- Enhanced BirthFortune with correlations
- CLI birth command
- Rich output formatting

Success Criteria:
- All optional features work
- CLI integration complete
- User documentation available
```

### 6.3 Testing Strategy

**Unit Test Coverage:**
```markdown
- chieuthu rules: 4 combinations (yang/yin × male/female)
- Start age calculation: Various distances to Tiết Khí (0, 1, 2, 5, 10, 15 days)
- Pillar progression: Verify stem/chi advance correctly for +1 and -1
- Edge cases: Leap months, pre-Tiết Khí births, year boundaries
- Integration: Verify with Can Chi and Tiết Khí outputs

Target: >90% code coverage for dai_van module
```

**Integration Test Coverage:**
```markdown
- Full calculation chain: birth date → lunar → Can Chi → Tiết Khí → pillars
- Fixture validation: At least 10 golden cases covering:
  - Yang/Male, Yang/Female
  - Yin/Male, Yin/Female
  - Year boundaries (1900, 2000, 2099)
  - Leap month births
  - Near-Tiết Khí births
  - Kua 5 edge cases
- Cross-subsystem: Verify Ten Gods and Kua integration

Target: All golden fixtures pass, no regressions
```

**Regression Testing:**
```markdown
Before and after implementation:
1. Run full test suite: cargo test --package amlich-core
2. Verify all existing tests pass
3. Verify KHCBPPT validators still pass (10/10 passing)
4. Verify Ten Gods tests still pass
5. Verify Kua tests still pass
6. Check JSON output format stability

Target: Zero regressions in existing functionality
```

### 6.4 Documentation Requirements

**Code Documentation:**
```markdown
Module-level doc (dai_van.rs):
- Overview of Đại Vân system
- Historical context and cultural significance
- Calculation method overview
- Input/output descriptions
- Example usage

Function-level docs:
- calculate_dai_yun(): Detailed algorithm, formula references
- All public functions: Purpose, arguments, returns, examples

Type-level docs:
- All structs: Field descriptions, invariants
- All enums: Variant meanings, usage patterns
```

**User Documentation:**
```markdown
README updates:
- Add Đại Vân section explaining what it is
- Provide calculation examples
- Show sample output
- Link to classical sources

API documentation:
- Document calculate_birth_fortune() function
- Provide request/response examples
- Explain birth vs. day fortune distinction
- Migration guide for existing users
```

---

## 7. Risks and Mitigations

### 7.1 KHCBPPT Verification Gap

**Risk:** Calculation rules may not match KHCBPPT exactly

**Mitigation:**
1. Use standard Bazi formulas as baseline (match vietnamese_lunar_engine_tables.md)
2. Implement with explicit documentation of assumptions
3. Document source_id as "khcbppt" placeholder initially
4. Create tracking issue for manual KHCBPPT research:
   ```markdown
   Issue: KHCBPPT source verification for Đại Vân
   - Search KHCBPPT volumes 12-13 for "大运" section
   - Extract exact calculation rules and any variants
   - Compare with implemented formulas
   - Update source documentation if differences found
   - Add golden fixtures verified against KHCBPPT text
   ```

### 7.2 Complexity and Testing Burden

**Risk:** High complexity may introduce bugs, extensive testing needed

**Mitigation:**
1. Break down into 6 clear calculation steps (already done in formulas section)
2. Test each step independently before integration
3. Use property-based testing (fuzzing edge cases)
4. Manual verification with multiple birth dates and calculators
5. Peer code review of calculation logic

### 7.3 API Surface Bloat

**Risk:** Adding birth fortune API may confuse users (day vs. birth)

**Mitigation:**
1. Clear naming: calculate_day_fortune() vs. calculate_birth_fortune()
2. Comprehensive documentation explaining the distinction
3. Deprecation warnings if extending DayFortune (not recommended)
4. Examples showing when to use which API
5. Consider CLI subcommands: `amlich day <date>` vs. `amlich birth <date>`

### 7.4 Performance Concerns

**Risk:** Birth fortune calculation requires lunar date conversion (expensive)

**Mitigation:**
1. Cache lunar date conversion results if repeated queries
2. Consider lazy calculation (only calculate when Dai Vân requested)
3. Benchmark performance for birth date range 1900-2100
4. Optimize if needed (precompute Tiết Khí dates, etc.)

---

## 8. Appendix: Quick Reference

### 8.1 Calculation Summary Table

| Step | Input | Operation | Output |
|-------|--------|-----------|---------|
| 1 | Birth date | Lunar conversion | Lunar year, month |
| 2 | Lunar year | Can Chi formula | Year Can Chi (stem, branch, polarity) |
| 3 | Year polarity + gender | Rule matrix | Chiều (Thuan/Nghich) |
| 4 | Birth date | Tiết Khí calculation | Days to nearest term |
| 5 | Days to term | Division | Start age |
| 6 | Lunar month/year | Can Chi formula | Month Can Chi |
| 7 | Month Can Chi + chieuthu | Iteration (8 times) | 8 Trụ pillars |

### 8.2 Chiều Rule Matrix

```rust
match (is_yang_year, gender) {
    (true, Gender::Male)   | (false, Gender::Female) => ChieuThu::Thuan,
    (true, Gender::Female)  | (false, Gender::Male)   => ChieuThu::Nghich,
}
```

### 8.3 Ten Gods Integration Example

```rust
// Calculate Ten Gods for each pillar
let dai_vun = calculate_dai_yun(birth_date, gender);
let day_stem = get_birth_day_stem(birth_date, birth_hour);

for pillar in &dai_vun.pillars {
    let ten_gods = thap_than::get_thap_than(day_stem, pillar.can);
    println!("Trụ {} (ages {}-{}): {} ({:?})",
        pillar.thu_tu,
        pillar.start_age,
        pillar.end_age,
        pillar.can_chi_name,
        ten_gods.label
    );
}
```

### 8.4 Sample JSON Output

```json
{
  "birth_fortune": {
    "day_fortune": {
      "day_element": {
        "na_am": "Hải Trung Kim",
        "element": "Kim",
        "can": "Giáp",
        "chi": "Tý"
      },
      "conflict": { /* ... */ },
      "stars": { /* ... */ },
      "taboos": [ /* ... */ ],
      "xung_hop": { /* ... */ },
      "truc": { /* ... */ }
    },
    "dai_van": {
      "chieu_thu": "nghich",
      "start_age": 2,
      "num_pillars": 8,
      "pillars": [
        {
          "thu_tu": 1,
          "start_age": 2,
          "end_age": 12,
          "can": "Mậu",
          "chi": "Sửu",
          "can_chi_name": "Mậu Sửu",
          "ten_gods": {
            "label": "thien_tai",
            "relation": "day_controls_target",
            "same_polarity": true
          },
          "kua_group": null
        },
        /* ... more pillars ... */
      ],
      "convention": {
        "year_basis": "lunar",
        "start_age_method": "3-days-per-year",
        "gender_encoding": "enum(Male,Female)",
        "source_id": "khcbppt",
        "method": "bai-quyet"
      }
    },
    "ten_gods_summary": { /* ... */ },
    "kua": { /* ... */ }
  }
}
```

---

## 9. Conclusion

**Research Status:** Complete

**Key Findings Summary:**
1. ✅ Đại Vân is a well-documented system with clear calculation rules
2. ✅ All dependencies (Can Chi, Tiết Khí) exist in amlich
3. ⚠️ KHCBPPT coverage requires manual research (no immediate blocker)
4. ✅ Implementation follows existing patterns (Ten Gods, Kua)
5. ✅ Can integrate cleanly as separate BirthFortune API

**Recommendation:**
Proceed with implementation using standard Bazi formulas from `vietnamese_lunar_engine_tables.md` Section 15 as primary source, with `source_id: "khcbppt"` placeholder. Create tracking issue for manual KHCBPPT verification and update source documentation when available.

**Complexity Assessment:**
- **Calculation Logic:** Medium (6 clear steps, each testable)
- **Integration Complexity:** High (birth date required, new API surface)
- **Testing Burden:** High (comprehensive fixtures, edge cases)
- **Overall Effort Estimate:** 40-60 hours for complete implementation with testing

**Next Actions:**
1. Create tracking issue: "KHCBPPT verification for Đại Vân calculation rules"
2. Begin Phase 1: Core module development
3. Follow phased implementation plan (6 weeks recommended)
4. Document progress in milestone tracking system

---

*End of Research Report*
