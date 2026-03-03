# Domain Pitfalls

**Domain:** Dai Van (Great Cycle) integration into KHCBPPT-verified deterministic almanac system
**Researched:** 2026-03-03
**Confidence:** HIGH (integration patterns from v1.2), MEDIUM (Dai Van KHCBPPT coverage uncertainty)

## Critical Pitfalls

### Pitfall 1: Period transition boundary errors cause incorrect pillar assignments

**What goes wrong:**
The 8 Dai Van pillars each span 10 years, with age ranges calculated from the start age. Off-by-one errors in age range calculations or pillar progression logic cause individuals to be assigned to the wrong pillar, producing incorrect Ten Gods correlations and Kua recommendations for their current life phase.

**Why it happens:**
Dai Van requires precise age range calculations: `start_age + (thu_tu-1)*10` to `start_age + thu_tu*10`. Common mistakes include:
- Using `<=` instead of `<` for end_age bounds (assigns person to next pillar a year early)
- Incorrect modulo arithmetic when advancing stem/branch indices (overflow/underflow with -1 direction)
- Mixing 0-indexed vs 1-indexed pillar numbering (thu_tu should be 1-8, not 0-7)

**How to avoid:**
```rust
// CORRECT: Age range uses start_age inclusive, end_age exclusive
pillar.start_age = start_age + (thu_tu - 1) * 10;
pillar.end_age = start_age + thu_tu * 10;

// CORRECT: Modulo handles negative values for Nghich direction
current_stem = (current_stem as i32 + chieu_thu + 10) as usize % 10;
current_branch = (current_branch as i32 + chieu_thu + 12) as usize % 12;

// CORRECT: thu_tu is 1-indexed (1-8)
assert!((1..=8).contains(&thu_tu));
```

Add property-based tests:
- Verify age ranges are contiguous (end_age[i] == start_age[i+1])
- Verify all ages 0-120 map to exactly one pillar or none
- Verify pillar count is always 8

**Warning signs:**
- Age gaps between pillars (e.g., pillar 1 ends at 12, pillar 2 starts at 14)
- Overlapping age ranges between pillars
- `get_current_pillar()` returns None for ages 30-80 (middle of life)
- Ten Gods results shift when moving from age 29 to 30 (should stay in same pillar)

**Phase to address:**
**v1.3 Phase 1** (Core module development) - Add age range validation tests and property-based checks

---

### Pitfall 2: Ten Gods correlation uses wrong stem (day vs pillar vs year)

**What goes wrong:**
Ten Gods for each Dai Van pillar are computed incorrectly because the wrong stem is used as the reference. Common mistakes include:
- Using pillar Chi instead of pillar Can (Ten Gods only applies to Can/Stems)
- Using year stem instead of day stem as the base (day stem is authoritative for personal fortune)
- Using query date's day stem instead of birth day stem (birth day stem is static, query day changes)
- Computing Ten Gods from pillar Can to pillar Can (should be pillar Can → birth day Can)

**Why it happens:**
Dai Van adds a new layer of indirection. In v1.2, Ten Gods had clear targets: day stem → year stem, day stem → self. With Dai Van, there are now 8 pillars, each with their own Can. Without explicit field naming, developers may:
- Copy-paste from v1.2 without updating target stem
- Assume pillar Can compares to itself (ty_kien/kiep_tai heavy results)
- Use the day stem of the query date instead of the birth date

**How to avoid:**
```rust
// WRONG: Ten Gods from pillar Can to pillar Can (always ty_kien or kiep_tai)
let ten_gods = get_thap_than(pillar.can, pillar.can);

// WRONG: Ten Gods from query day stem to pillar Can (dynamic, not birth-based)
let today = chrono::Utc::now().naive_utc().date();
let query_day_stem = get_day_can_from_date(today);
let ten_gods = get_thap_than(query_day_stem, pillar.can);

// CORRECT: Ten Gods from birth day stem to pillar Can (static, birth-based)
let birth_day_stem = get_day_can_from_date(birth_date, birth_hour);
let ten_gods = get_thap_than(birth_day_stem, pillar.can);

// WRONG: Generic field loses provenance
pub struct DaYunPillar {
    pub ten_gods: Option<ThapThanResult>,  // Which stem is this compared to?
}

// CORRECT: Explicit field naming preserves provenance
pub struct DaYunPillar {
    pub ten_gods_vs_day_stem: Option<ThapThanResult>,  // Clear: pillar Can → birth day Can
}
```

Add integration fixtures that assert:
- Each pillar's Ten Gods is computed against birth day stem
- Ten Gods results differ between pillars (not all ty_kien/kiep_tai)
- Ten Gods remain constant across different query dates for same birth date

**Warning signs:**
- All 8 pillars return the same Ten Gods label (ty_kien or kiep_tai most common)
- Ten Gods for a person change when queried on different dates (should be static)
- Ten Gods results don't match independent online calculators for same birth date
- No documentation of which stem is used as base for Ten Gods calculation

**Phase to address:**
**v1.3 Phase 2** (Integration and Ten Gods correlation) - Add explicit field naming and integration tests asserting correct target stem

---

### Pitfall 3: Kua integration mismatched to Dai Van pillar periods

**What goes wrong:**
Kua-based directional recommendations are applied to Dai Van pillars incorrectly, causing users to follow wrong directions during specific 10-year periods. Mistakes include:
- Using query date's Kua instead of birth Kua (Kua is static for a person)
- Applying Kua directions to pillar Chi instead of pillar Can element
- Mixing East/West group conventions (some systems use birth year, some use birth hour)
- Using Kua 5 directly without resolution (male→8, female→2 per project policy)

**Why it happens:**
Kua was implemented in v1.2 with clear conventions (birth year, solar year basis, Kua 5 resolution). Dai Van adds time periods, and developers may:
- Forget that Kua is computed once per person, not per pillar or query date
- Apply Kua directions to pillar Chi (earthly branch) instead of pillar Can (heavenly stem)
- Recompute Kua for each pillar (wasteful, wrong semantics)

**How to avoid:**
```rust
// WRONG: Kua recomputed per pillar (wrong semantics, expensive)
for pillar in &mut pillars {
    let kua = compute_kua(birth_year, gender);  // Same result 8 times
    pillar.kua_analysis = Some(analyze_with_kua(pillar, &kua));
}

// CORRECT: Kua computed once per person, applied to all pillars
let kua = compute_kua(birth_year, gender);
for pillar in &mut pillars {
    pillar.kua_analysis = Some(analyze_with_kua(pillar, &kua));
}

// WRONG: Use pillar Chi for Kua analysis (Chi is not in Five Element mapping)
let pillar_element = pillar.chi.element();  // Chi has no element()

// CORRECT: Use pillar Can (heavenly stem) for element
let pillar_element = pillar.can.element();  // Can → FiveElement

// WRONG: Kua 5 appears directly in output
pub enum KuaNumber {
    Kua1, Kua2, Kua3, Kua4, Kua5, Kua6, Kua7, Kua8, Kua9,
}

// CORRECT: Kua 5 resolved at computation time (per project policy)
pub enum KuaNumber {
    Kua1, Kua2, Kua3, Kua4, Kua6, Kua7, Kua8, Kua9,  // Kua5 never appears
}
```

Add integration fixtures asserting:
- Kua is computed once per birth year + gender, not per pillar or query date
- Kua 5 is resolved to 8 (male) or 2 (female) per project policy
- Kua analysis uses pillar Can element, not pillar Chi

**Warning signs:**
- Kua number differs between pillars for same birth date/gender
- Kua 5 appears in output without resolution
- Kua analysis references pillar Chi (earthly branch) instead of pillar Can (heavenly stem)
- Kua computation uses query year instead of birth year

**Phase to address:**
**v1.3 Phase 3** (Kua integration and API surface) - Add fixtures and tests asserting Kua convention compliance

---

### Pitfall 4: KHCBPPT source verification gap for Dai Van rules

**What goes wrong:**
Dai Van calculation rules are implemented using standard Bazi formulas, but KHCBPPT verification is incomplete or missing. This violates the project's core principle: "Every almanac subsystem must produce output that matches KHCBPPT for the 2020-2030 date range."

**Why it happens:**
DAI_VAN_RESEARCH.md notes that KHCBPPT coverage is uncertain (no explicit section found in online search). Developers may:
- Implement using generic Bazi formulas (Yuan Hai Zi Ping, etc.) without KHCBPPT citation
- Use `source_id: "khcbppt"` as placeholder without verification
- Skip manual KHCBPPT research for this milestone (time pressure)
- Assume "Bazi is Bazi" and KHCBPPT rules match modern sources

**How to avoid:**
1. **Immediate mitigation for v1.3:**
   - Use standard Bazi formulas from `vietnamese_lunar_engine_tables.md` Section 15 as primary source
   - Document source_id as "khcbppt" placeholder with TODO comment
   - Create tracking issue for manual KHCBPPT verification
   - Note uncertainty in ConventionMetadata and README

2. **Long-term resolution (v1.3+):**
   - Manual research of KHCBPPT physical or high-quality digital scan
   - Search for "大运" (Da Yùn), "十年大运" (Ten-Year Major Luck), "顺逆" (Forward/Backward)
   - Extract exact calculation rules and compare with implementation
   - Update source documentation if differences found
   - Add golden fixtures verified against KHCBPPT text

```rust
// Document uncertainty transparently
impl ConventionMetadata {
    pub fn project_default() -> Self {
        Self {
            year_basis: "lunar".to_string(),
            start_age_method: "3-days-per-year".to_string(),
            gender_encoding: "enum(Male,Female)".to_string(),
            source_id: "khcbppt".to_string(),  // TODO: Manual verification pending
            method: "bai-quyet".to_string(),     // TODO: Verify in KHCBPPT volumes 12-13
        }
    }
}
```

Add tests that will fail if KHCBPPT verification reveals differences:
- Create representative golden fixtures (10+ cases across different birth years/genders)
- Document expected outputs based on current understanding
- Note which fixtures need KHCBPPT verification

**Warning signs:**
- Evidence metadata has `source_id: "khcbppt"` but no citation or volume reference
- Dai Van fixtures differ from independent online calculators without explanation
- No KHCBPPT-specific calculation rules documented (only generic Bazi)
- Test fixtures use "TODO" or "pending verification" comments

**Phase to address:**
**v1.3 Phase 1** (Research and foundation) - Document uncertainty and create tracking issue for manual KHCBPPT verification (future phase)

---

### Pitfall 5: Start age calculation uses wrong Tiết Khí direction or date basis

**What goes wrong:**
Dai Van start age calculation (when the first pillar begins) depends on the distance from birth date to the nearest Tiết Khí (solar term). Mistakes include:
- Using absolute value incorrectly (days before vs after Tiết Khí)
- Using Gregorian date instead of lunar date for Tiết Khí lookup
- Using wrong Tiết Khí (current vs previous vs next)
- Incorrect conversion of days to years (3 days = 1 year standard)

**Why it happens:**
The formula is: `start_age = |days_to_nearest_tiet_khi| / 3`. But implementation details matter:
- `get_days_to_nearest_tiet_khi()` must return signed difference (negative if born before Tiết Khí)
- Days calculation must be based on Julian Day Number, not naive date math
- "Nearest" Tiết Khí could be previous or next, not always the next one
- Some traditions round differently (floor vs ceil vs nearest integer)

**How to avoid:**
```rust
// WRONG: Always use next Tiết Khí (can be 6 months away)
let days_to_tiet_khi = get_days_to_next_tiet_khi(birth_jd);

// WRONG: Always use previous Tiết Khí
let days_to_tiet_khi = get_days_from_prev_tiet_khi(birth_jd);

// CORRECT: Use nearest (could be previous or next)
let days_to_tiet_khi = get_days_to_nearest_tiet_khi(birth_jd);
// Returns signed value: negative if born before term, positive if after

// WRONG: Naive date difference (timezone issues)
let days_diff = (birth_date - tiet_khi_date).num_days();

// CORRECT: Julian Day Number (timezone-independent)
let birth_jd = jd_from_date(birth_date.day(), birth_date.month(), birth_date.year());
let tiet_khi_jd = get_tiet_khi_jd(tiet_khi_name, birth_year);
let days_diff = birth_jd - tiet_khi_jd;

// WRONG: Use Gregorian month/year for Tiết Khí lookup
let tiet_khi = get_tiet_khi_for_gregorian_month(gregorian_month, gregorian_year);

// CORRECT: Use lunar month/year for Tiết Khí lookup
let lunar_date = get_lunar_date(birth_date.day(), birth_date.month(), birth_date.year());
let tiet_khi = get_tiet_khi_for_lunar_month(lunar_date.lunar_month, lunar_date.lunar_year);
```

Add edge case fixtures:
- Born exactly on a Tiết Khí (days = 0, start_age = 0)
- Born 1 day before/after Tiết Khí (start_age = 0 after truncation, or 1 after rounding)
- Born 2 days before/after Tiết Khí (start_age = 0 after floor, or 1 after rounding)
- Born 5 days before/after Tiết Khí (start_age = 1 after floor/2 after rounding)
- Born 10 days before/after Tiết Khí (start_age = 3 after floor, 4 after nearest)
- Born 30 days before/after Tiết Khí (start_age = 10)

Document rounding convention explicitly in ConventionMetadata.

**Warning signs:**
- Start age is always 0 or always 10 for all test cases (implies always using previous or always using next)
- Start age values are implausibly large (> 15 years) or negative
- Tiết Khí lookup uses Gregorian month/year instead of lunar
- No fixtures for birth dates exactly on or near Tiết Khí boundaries

**Phase to address:**
**v1.3 Phase 1** (Core module) - Add comprehensive edge case fixtures and document rounding convention

---

### Pitfall 6: Chiều (direction) rule matrix errors cause wrong pillar progression

**What goes wrong:**
Chiều (forward/nghịch or backward/thuận) determines whether Dai Van pillars progress forward (+1) or backward (-1) in the 60-year cycle. The rule matrix is: `(Year Yang/Âm × Gender) → Thuận/Nghịch`. Errors here cause all 8 pillars to be wrong.

**Why it happens:**
The rule matrix is counterintuitive:
- Yang year + Male = Thuận (forward)
- Yang year + Female = Nghịch (backward)
- Âm year + Male = Nghịch (backward)
- Âm year + Female = Thuận (forward)

Developers may:
- Misremember the matrix (e.g., think Yang+Male = Nghịch)
- Use solar year polarity instead of lunar year polarity (year Can Chi is lunar-based)
- Confuse polarity of Can (heavenly stem) vs Chi (earthly branch) - it's the Chi that matters
- Hardcode the matrix wrong in implementation

**How to avoid:**
```rust
// WRONG: Use solar year polarity
let solar_year = birth_date.year();
let is_yang = solar_year % 2 == 0;

// CORRECT: Use lunar year polarity from Chi
let lunar_date = get_lunar_date(birth_date.day(), birth_date.month(), birth_date.year());
let year_chi = get_year_chi(lunar_date.lunar_year);
let is_yang_year = is_yang_chi(year_chi);  // Yang: Tý, Dần, Thìn, Ngọ, Thân, Tuất (even indices)

// WRONG: Use Can polarity instead of Chi
let year_can = get_year_can(lunar_date.lunar_year);
let is_yang = year_can.polarity() == Polarity::Duong;

// WRONG: Incorrect matrix
let chieu_thu = match (is_yang_year, gender) {
    (true, Gender::Male) => ChieuThu::Nghich,  // WRONG!
    ...
};

// CORRECT: Documented rule matrix
let chieu_thu = match (is_yang_year, gender) {
    (true, Gender::Male) | (false, Gender::Female) => ChieuThu::Thuan,   // Forward
    (true, Gender::Female) | (false, Gender::Male) => ChieuThu::Nghich, // Backward
};

// Add unit tests for all 4 combinations
#[test]
fn test_chieuthu_matrix() {
    // Yang year + Male = Thuan
    assert_eq!(determine_chieuthu(/* yang year */, Gender::Male), ChieuThu::Thuan);
    // Yang year + Female = Nghich
    assert_eq!(determine_chieuthu(/* yang year */, Gender::Female), ChieuThu::Nghich);
    // Yin year + Male = Nghich
    assert_eq!(determine_chieuthu(/* yin year */, Gender::Male), ChieuThu::Nghich);
    // Yin year + Female = Thuan
    assert_eq!(determine_chieuthu(/* yin year */, Gender::Female), ChieuThu::Thuan);
}
```

Add integration fixtures asserting:
- Pillar progression follows chieuthu (stem/branch advance by +1 for Thuận, -1 for Nghịch)
- All 4 (Yang/Yin × Male/Female) combinations produce correct results
- Yang years: Tý, Dần, Thìn, Ngọ, Thân, Tuất (even Chi indices 0, 2, 4, 6, 8, 10)

**Warning signs:**
- All 4 gender/year combinations produce the same chieuthu
- Pillar Can/Chi progress in same direction for all test cases (should differ)
- Unit tests only cover 1-2 combinations instead of all 4
- No documentation of which Chi indices are Yang vs Âm

**Phase to address:**
**v1.3 Phase 1** (Core module) - Add chieuthu rule matrix tests and document Yang/Âm Chi indices

---

### Pitfall 7: Backward compatibility broken by adding required birth inputs

**What goes wrong:**
Dai Van requires birth date and gender inputs, but existing `calculate_day_fortune()` API only takes a date. Adding birth inputs as required fields breaks all existing callers (CLI, API, tests) that don't have birth context.

**Why it happens:**
Developers may:
- Add `birth_date: NaiveDate` and `gender: Gender` as required parameters to existing functions
- Make DayFortune fields required instead of optional
- Remove existing API surface and replace with new API
- Assume all callers will migrate immediately

**How to avoid:**
```rust
// WRONG: Add required birth inputs to existing function
pub fn calculate_day_fortune(
    date: NaiveDate,
    birth_date: NaiveDate,  // REQUIRED - breaks existing callers!
    gender: Gender,          // REQUIRED - breaks existing callers!
) -> DayFortune {
    ...
}

// CORRECT: Keep existing function unchanged, add new function for birth-based features
pub fn calculate_day_fortune(date: NaiveDate) -> DayFortune {
    // Existing implementation, unchanged
    DayFortune {
        ...existing fields...,
        dai_van: None,  // Always None without birth context
    }
}

// NEW: Separate API for birth-based fortune
pub fn calculate_birth_fortune(
    birth_date: NaiveDate,
    birth_hour: Option<u8>,
    gender: Gender,
    reference_date: Option<NaiveDate>,  // Default: today
) -> BirthFortune {
    let ref_date = reference_date.unwrap_or_else(|| chrono::Utc::now().naive_utc().date());
    BirthFortune {
        day_fortune: calculate_day_fortune(ref_date),
        dai_van: calculate_dai_yun(birth_date, gender),
        kua: compute_kua(birth_date.year(), gender),
        ten_gods_summary: calculate_ten_gods_summary(birth_date, birth_hour),
    }
}

// CORRECT: Additive optional field in DayFortune
pub struct DayFortune {
    ...existing fields...,
    /// Dai Van result (populated only when birth context provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dai_van: Option<DaYunResult>,
}
```

Add contract tests asserting:
- `calculate_day_fortune(date)` with single parameter still works
- DayFortune.dai_van is None when no birth context provided
- JSON serialization with optional fields remains backward-compatible
- Existing CLI/API consumers continue to work without modification

**Warning signs:**
- Function signatures change in ways that break compilation of existing callers
- New required fields added to existing structs
- API surface removed or replaced without deprecation period
- No tests asserting backward compatibility

**Phase to address:**
**v1.3 Phase 3** (API surface) - Add new BirthFortune API, keep DayFortune API unchanged, add backward compatibility tests

---

### Pitfall 8: Determinism violations from hidden state or time-based calculations

**What goes wrong:**
Dai Van calculations produce different results on different runs or at different times, violating the project's deterministic computation requirement. Causes include:
- Using `chrono::Utc::now()` as default reference date instead of explicit parameter
- Caching with implicit expiration
- Using floating-point arithmetic without proper rounding
- Relying on system locale or timezone

**Why it happens:**
Developers may use convenient defaults that introduce nondeterminism:
- Default reference date to "today" (`Utc::now()`) - changes every day
- Use `rand` for tie-breaking (should never happen in deterministic system)
- Use floating-point division without consistent rounding strategy
- Compare timestamps or durations with millisecond precision

**How to avoid:**
```rust
// WRONG: Default to "today" - nondeterministic across runs
pub fn calculate_birth_fortune(
    birth_date: NaiveDate,
    birth_hour: Option<u8>,
    gender: Gender,
) -> BirthFortune {
    let today = chrono::Utc::now().naive_utc().date();  // Changes every day!
    let ref_date = today;
    ...
}

// CORRECT: Require explicit reference date, or default to None and compute birth-only
pub fn calculate_birth_fortune(
    birth_date: NaiveDate,
    birth_hour: Option<u8>,
    gender: Gender,
    reference_date: Option<NaiveDate>,  // Explicit or None
) -> BirthFortune {
    // If reference_date is None, return birth-only results (no day fortune)
    let day_fortune = reference_date
        .map(|date| calculate_day_fortune(date))
        .ok_or_else(|| Error::MissingReferenceDate)?;
    ...
}

// WRONG: Floating-point division without rounding
let start_age = days_to_tiet_khi.abs() / 3.0;  // 1.666...

// CORRECT: Integer division with explicit rounding convention
let start_age = (days_to_tiet_khi.abs() / 3) as u8;  // Truncates (floor)

// CORRECT: If rounding to nearest is needed, document convention explicitly
let start_age = ((days_to_tiet_khi.abs() as f64) / 3.0).round() as u8;
// Document: "Rounding convention: nearest integer, ties round up (ceiling)"
```

Add determinism tests:
- Run calculation 1000 times with same inputs, assert identical results
- Run in different timezones, assert identical results (no local time dependencies)
- Compare serialization output byte-for-byte across runs

**Warning signs:**
- Test results flaky (pass sometimes, fail sometimes)
- `Utc::now()`, `Local::now()`, or similar used in production code
- Floating-point arithmetic without explicit rounding convention
- Cache invalidation based on time

**Phase to address:**
**v1.3 Phase 1** (Core module) - Add determinism tests and review all time-related code

---

### Pitfall 9: Schema mismatch between core types and API DTO layers

**What goes wrong:**
Core `DaYunResult` type adds fields, but `amlich-api` DTO/convert layer is not updated consistently, causing silent field drops, breaking API consumers, or inconsistent JSON shape.

**Why it happens:**
Current architecture duplicates type surfaces (core structs + API DTO structs + conversion impls). Every new field requires 3 synchronized edits:
1. Core type definition (`types.rs`)
2. DTO type definition (`dto.rs`)
3. Conversion implementation (`convert.rs`)

Developers may:
- Update core type only, forget DTO
- Add field in core, skip conversion logic
- Rename field in core but not DTO (or vice versa)
- Add field with different name/semantics in core vs DTO

**How to avoid:**
```rust
// WRONG: Update core type only
pub struct DaYunResult {
    pub chieu_thu: ChieuThu,
    pub start_age: u8,
    pub pillars: Vec<DaYunPillar>,
    // New field added here
    pub calculation_metadata: CalculationMetadata,
}

// DTO not updated - field silently dropped in API response!

// CORRECT: Update all three layers in one change
// 1. Core type (types.rs)
pub struct DaYunResult {
    pub chieu_thu: ChieuThu,
    pub start_age: u8,
    pub pillars: Vec<DaYunPillar>,
    pub calculation_metadata: CalculationMetadata,
}

// 2. DTO type (dto.rs)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaYunResultDto {
    pub chieu_thu: ChieuThu,
    pub start_age: u8,
    pub pillars: Vec<DaYunPillarDto>,
    pub calculation_metadata: CalculationMetadataDto,
}

// 3. Conversion (convert.rs)
impl From<&DaYunResult> for DaYunResultDto {
    fn from(core: &DaYunResult) -> Self {
        DaYunResultDto {
            chieu_thu: core.chieu_thu,
            start_age: core.start_age,
            pillars: core.pillars.iter().map(|p| p.into()).collect(),
            calculation_metadata: core.calculation_metadata.clone().into(),
        }
    }
}
```

Add compile-failing test or snapshot test to catch drift:
```rust
#[test]
fn test_dai_van_dto_conversion_complete() {
    let core_result = create_test_dai_van_result();
    let dto: DaYunResultDto = (&core_result).into();

    // Assert all core fields map to DTO
    assert_eq!(core.chieu_thu, dto.chieu_thu);
    assert_eq!(core.start_age, dto.start_age);
    assert_eq!(core.pillars.len(), dto.pillars.len());
    assert_eq!(core.calculation_metadata, dto.calculation_metadata);
}
```

**Warning signs:**
- Field exists in core JSON but missing from API JSON
- Test coverage in `amlich-api` is lower than `amlich-core`
- PR changes core types without corresponding dto.rs + convert.rs changes
- API consumers report "missing field" errors after update

**Phase to address:**
**v1.3 Phase 3** (API surface) - Add DTO conversion tests, require all three layers in one PR

---

### Pitfall 10: Performance degradation from repeated Tiết Khí or Can Chi calculations

**What goes wrong:**
Dai Van calculation requires computing Tiết Khí distance, lunar date conversion, and Can Chi for year/month. If computed repeatedly (e.g., in a loop or on each API call), performance degrades unacceptably, especially for batch queries or date range requests.

**Why it happens:**
Developers may:
- Compute Tiết Khí inside pillar generation loop (should compute once per birth date)
- Call `get_lunar_date()` for each pillar (same birth date, same result 8 times)
- Recompute month Can Chi in integration code (already computed in core)
- Use inefficient date operations (naive iteration over days to find Tiết Khí)

**How to avoid:**
```rust
// WRONG: Tiết Khí computed in loop
fn generate_pillars(month_can_chi: CanChi, chieu_thu: i32, birth_date: NaiveDate) -> Vec<DaYunPillar> {
    let mut pillars = Vec::new();
    for thu_tu in 1..=8 {
        // WRONG: Computed 8 times!
        let days_to_tiet_khi = get_days_to_nearest_tiet_khi(birth_date);
        ...
    }
    pillars
}

// CORRECT: Compute once, use in loop
fn generate_pillars(
    month_can_chi: CanChi,
    chieu_thu: i32,
    start_age: u8,  // Computed once outside
) -> Vec<DaYunPillar> {
    let mut pillars = Vec::new();
    for thu_tu in 1..=8 {
        // Reuse precomputed value
        ...
    }
    pillars
}

// WRONG: Lunar date computed per pillar
fn calculate_dai_van(birth_date: NaiveDate, gender: Gender) -> DaYunResult {
    let mut pillars = Vec::new();
    for thu_tu in 1..=8 {
        // WRONG: Same birth date computed 8 times!
        let lunar_date = get_lunar_date(birth_date.day(), birth_date.month(), birth_date.year());
        let year_can_chi = get_year_canchi(lunar_date.lunar_year);
        ...
    }
    ...
}

// CORRECT: Compute once at start
fn calculate_dai_van(birth_date: NaiveDate, gender: Gender) -> DaYunResult {
    let lunar_date = get_lunar_date(birth_date.day(), birth_date.month(), birth_date.year());
    let year_can_chi = get_year_canchi(lunar_date.lunar_year);

    let mut pillars = Vec::new();
    for thu_tu in 1..=8 {
        // Reuse precomputed values
        ...
    }
    ...
}
```

Add benchmark tests:
```rust
#[bench]
fn bench_dai_van_calculation(b: &mut Bencher) {
    let birth_date = NaiveDate::from_ymd(1990, 3, 15);
    let gender = Gender::Male;
    b.iter(|| {
        calculate_dai_van(birth_date, gender);
    });
}

// Assert reasonable performance (e.g., < 1ms per calculation)
#[test]
fn test_dai_van_performance() {
    let start = Instant::now();
    for year in 1900..=2100 {
        for gender in [Gender::Male, Gender::Female] {
            let birth_date = NaiveDate::from_ymd(year, 6, 15);
            calculate_dai_van(birth_date, gender);
        }
    }
    let elapsed = start.elapsed();
    assert!(elapsed < Duration::from_secs(5), "Performance too slow: {:?}", elapsed);
}
```

**Warning signs:**
- Profiling shows `get_days_to_nearest_tiet_khi` or `get_lunar_date` called 8+ times per calculation
- Benchmark times increase linearly with number of pillars (should be constant)
- Date range queries take unexpectedly long
- No performance tests added for new calculation

**Phase to address:**
**v1.3 Phase 1** (Core module) - Add performance tests, profile Tiết Khí and Can Chi computation

---

### Pitfall 11: Testing gaps miss edge cases and boundary conditions

**What goes wrong:**
Dai Van test coverage appears sufficient but misses critical edge cases, allowing bugs to slip through to production. Common gaps include:
- Missing leap month test cases (birth during lunar leap month)
- Missing year boundary tests (born on Dec 31 or Jan 1)
- Missing Tiết Khí boundary tests (born exactly on or within 1-2 days of solar term)
- Missing Kua 5 edge cases (birth years requiring resolution)
- Missing chieuthu rule matrix tests (only 1-2 of 4 combinations tested)
- Missing age boundary tests (ages 9, 10, 19, 20, etc. - pillar transitions)

**Why it happens:**
Developers may focus on "happy path" cases and overlook edge cases that are statistically rare but commonly requested by users:
- Most births are not on Tiết Khí or near year boundaries
- Most births are not during leap months
- Most people are not Kua 5

**How to avoid:**
Create comprehensive golden fixture set covering:
```json
{
  "fixtures": [
    {
      "id": "dv_001",
      "description": "Standard case: Male, Yang year, not near boundaries",
      "input": {"birth_date": "1990-06-15", "gender": "Male"},
      "expected": {...}
    },
    {
      "id": "dv_002",
      "description": "Yang year, Female (chieuthu different)",
      "input": {"birth_date": "1990-08-20", "gender": "Female"},
      "expected": {...}
    },
    {
      "id": "dv_003",
      "description": "Âm year, Male (chieuthu different)",
      "input": {"birth_date": "1991-03-15", "gender": "Male"},
      "expected": {...}
    },
    {
      "id": "dv_004",
      "description": "Âm year, Female (chieuthu different)",
      "input": {"birth_date": "1991-09-10", "gender": "Female"},
      "expected": {...}
    },
    {
      "id": "dv_005",
      "description": "Born exactly on Tiết Khí (days = 0, start_age = 0)",
      "input": {"birth_date": "2024-02-04", "gender": "Male"},
      "expected": {"start_age": 0, ...}
    },
    {
      "id": "dv_006",
      "description": "Born 1 day before Tiết Khí (start_age = 0)",
      "input": {"birth_date": "2024-02-03", "gender": "Female"},
      "expected": {"start_age": 0, ...}
    },
    {
      "id": "dv_007",
      "description": "Born 1 day after Tiết Khí (start_age = 0)",
      "input": {"birth_date": "2024-02-05", "gender": "Male"},
      "expected": {"start_age": 0, ...}
    },
    {
      "id": "dv_008",
      "description": "Born 5 days before Tiết Khí (start_age = 1)",
      "input": {"birth_date": "2024-01-30", "gender": "Female"},
      "expected": {"start_age": 1, ...}
    },
    {
      "id": "dv_009",
      "description": "Born during lunar leap month",
      "input": {"birth_date": "2023-04-20", "gender": "Male"},
      "is_leap_month": true,
      "expected": {...}
    },
    {
      "id": "dv_010",
      "description": "Kua 5 case: Male (should resolve to Kua 8)",
      "input": {"birth_date": "2002-08-10", "gender": "Male"},
      "expected": {"kua_number": 8, ...}
    },
    {
      "id": "dv_011",
      "description": "Kua 5 case: Female (should resolve to Kua 2)",
      "input": {"birth_date": "2002-08-10", "gender": "Female"},
      "expected": {"kua_number": 2, ...}
    },
    {
      "id": "dv_012",
      "description": "Year boundary: Born Dec 31",
      "input": {"birth_date": "1999-12-31", "gender": "Male"},
      "expected": {...}
    },
    {
      "id": "dv_013",
      "description": "Year boundary: Born Jan 1",
      "input": {"birth_date": "2000-01-01", "gender": "Female"},
      "expected": {...}
    },
    {
      "id": "dv_014",
      "description": "Age boundary test: Person age 9 (last day of pillar 1)",
      "birth_date": "1990-03-15",
      "gender": "Male",
      "current_age": 9,
      "expected_current_pillar": {"thu_tu": 1, ...}
    },
    {
      "id": "dv_015",
      "description": "Age boundary test: Person age 10 (first day of pillar 2)",
      "birth_date": "1990-03-15",
      "gender": "Male",
      "current_age": 10,
      "expected_current_pillar": {"thu_tu": 2, ...}
    }
  ]
}
```

Add property-based tests using proptest:
```rust
#[cfg(test)]
mod proptest_tests {
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_age_ranges_contiguous(start_age in 0u8..10) {
            let pillars = generate_test_pillars(start_age);
            for i in 0..pillars.len()-1 {
                prop_assert_eq!(pillars[i].end_age, pillars[i+1].start_age);
            }
        }

        #[test]
        fn test_all_ages_covered(start_age in 0u8..10) {
            let pillars = generate_test_pillars(start_age);
            for age in 0..120 {
                let pillar = get_current_pillar(&pillars, age);
                if age >= start_age && age < start_age + 80 {
                    prop_assert!(pillar.is_some());
                }
            }
        }
    }
}
```

**Warning signs:**
- Test fixtures all have similar birth dates (e.g., all in June, none near Tiết Khí)
- No tests for leap months
- Only 1-2 of 4 chieuthu rule combinations tested
- No property-based tests for invariants (age ranges contiguous, all ages covered)
- No benchmark or performance tests

**Phase to address:**
**v1.3 Phase 2** (Integration and testing) - Add comprehensive golden fixtures and property-based tests

---

## Technical Debt Patterns

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Use generic `ten_gods` field without provenance | Less schema work | Ambiguous semantics, wrong target stem errors | Never (use explicit field naming) |
| Add birth inputs as required to existing API | Simpler function signatures | Breaks all existing callers | Never (additive-only, new API) |
| Use `source_id: "khcbppt"` as placeholder | Faster implementation | Can't verify correctness, violates project standard | Only as short-term mitigation with TODO issue |
| Recompute Tiết Khí per pillar | Simpler code | 8x performance overhead | Never (compute once) |
| Omit convention metadata for Kua integration | Smaller payload | Can't explain divergences, convention drift | Never (evidence-first design) |
| Use floating-point division without rounding | Quick implementation | Nondeterministic results across platforms | Never (integer arithmetic or documented rounding) |

## Integration Gotchas

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| Ten Gods correlation | Compare pillar Can to pillar Can (always ty_kien) | Compare pillar Can to birth day Can (explicit field naming) |
| Kua integration | Recompute Kua per pillar or use query year | Compute once from birth year/gender, apply to all pillars |
| Chiều determination | Use solar year polarity or Can polarity | Use lunar year Chi polarity (Tý, Dần, Thìn, Ngọ, Thân, Tuất = Yang) |
| Start age calculation | Always use next Tiết Khí or always previous | Use nearest (could be previous or next), get signed distance |
| Core → API DTO | Update core type only, forget DTO | Update core + DTO + conversion in one PR, add conversion test |
| Birth vs day API | Add birth inputs to existing `calculate_day_fortune` | Keep existing API unchanged, add new `calculate_birth_fortune` |

## Performance Traps

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Recomputing Tiết Khí in pillar loop | 8x slow than necessary | Compute Tiết Khí once per birth date | Any Dai Van calculation |
| Recomputing lunar date per pillar | Unnecessary conversions | Compute lunar date once per birth date | Birth date range queries |
| Repeated Can Chi lookups | Cache misses, CPU waste | Cache year/month Can Chi results | Batch date range calculations |
| Floating-point arithmetic | Nondeterministic results | Use integer arithmetic or documented rounding | Cross-platform consistency |

## Security Mistakes

| Mistake | Risk | Prevention |
|---------|------|------------|
| Accepting arbitrary gender string and defaulting silently | Data integrity errors in personal astrology output | Strict enum parsing + explicit error (no silent fallback) |
| Panicking on invalid Kua edge case | Service instability if exposed through API | Return typed Result, no panic paths in production |
| Unvalidated birth date ranges (future dates) | Nonsensical results for future births | Validate birth date is not in future and within reasonable range (1900-2100) |

## UX Pitfalls

| Pitfall | User Impact | Better Approach |
|---------|-------------|-----------------|
| Showing Kua 5 directly instead of resolved value | Confusion, unexpected Kua numbers | Always resolve to 8 (male) or 2 (female) per project policy |
| Ten Gods without target stem provenance | Users misinterpret results | Include `ten_gods_vs_day_stem` with explicit documentation |
| Mixing birth-based and day-based concepts in same UI | Users confuse query date with birth date | Separate "Day Fortune" from "Birth Fortune" in UI/API |

## "Looks Done But Isn't" Checklist

- [ ] **Dai Van pillar progression:** All 8 pillars generated, but chieuthu rule matrix only tested for 1-2 combinations — verify all 4 (Yang/Yin × Male/Female).
- [ ] **Age range calculations:** Pillars have start_age and end_age, but not tested for contiguity or boundary conditions — verify end_age[i] == start_age[i+1] and all ages 0-120 map correctly.
- [ ] **Ten Gods correlation:** Ten Gods field present, but unclear which stem is base — verify explicit field naming (`ten_gods_vs_day_stem`) and birth day stem used.
- [ ] **Tiết Khí calculation:** Start age computed, but no fixtures for births exactly on or near Tiết Khí — add edge cases for 0, ±1, ±2, ±5, ±10, ±30 days.
- [ ] **Kua integration:** Kua computed, but may be per-pillar instead of per-person — verify Kua computed once from birth year/gender.
- [ ] **KHCBPPT verification:** `source_id: "khcbppt"` used but no volume reference — document uncertainty and create tracking issue for manual verification.
- [ ] **Backward compatibility:** New API surface added, but no tests asserting old API still works — add contract tests for `calculate_day_fortune(date)` with single parameter.
- [ ] **Core/API schema:** Core types updated but DTO/convert layer missing fields — add conversion completeness test.

## Recovery Strategies

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| Wrong chieuthu rule matrix | HIGH | Verify against authoritative source, update implementation, regenerate all fixtures, add matrix tests |
| Ten Gods wrong target stem | MEDIUM | Rename field to explicit provenance, update calculation logic, backfill fixtures |
| Kua per-pillar instead of per-person | MEDIUM | Move Kua computation to before pillar loop, update tests |
| Core/API schema drift | MEDIUM | Patch DTO/convert parity, add snapshot tests to prevent recurrence |
| KHCBPPT verification missing | LOW | Document uncertainty, create tracking issue for future research |
| Age range boundaries wrong | MEDIUM | Verify start_age/end_age calculations, add property-based tests |

## Pitfall-to-Phase Mapping

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| Period transition boundary errors | v1.3 Phase 1 | Property-based tests assert age ranges contiguous and all ages covered |
| Ten Gods wrong target stem | v1.3 Phase 2 | Integration fixtures assert Ten Gods computed against birth day stem |
| Kua integration mismatched | v1.3 Phase 3 | Fixtures assert Kua computed once per birth, not per pillar |
| KHCBPPT source verification gap | v1.3 Phase 1 | Document uncertainty, create tracking issue, use placeholder source_id |
| Start age Tiết Khí errors | v1.3 Phase 1 | Edge case fixtures for births on/within ±1, ±2, ±5, ±10, ±30 days of Tiết Khí |
| Chiều rule matrix errors | v1.3 Phase 1 | Unit tests for all 4 (Yang/Yin × Male/Female) combinations |
| Backward compatibility breaks | v1.3 Phase 3 | Contract tests assert existing API still works, new fields optional |
| Determinism violations | v1.3 Phase 1 | Determinism tests: run 1000x with same inputs, assert identical results |
| Core/API schema drift | v1.3 Phase 3 | DTO conversion completeness test, update core+DTO+convert in one PR |
| Performance degradation | v1.3 Phase 1 | Benchmark tests, profile Tiết Khí/Can Chi computation |
| Testing gaps | v1.3 Phase 2 | Comprehensive golden fixtures (15+ cases), property-based tests |

## Sources

- `.planning/PROJECT.md` (v1.3 Dai Van Core goals and constraints)
- `.planning/research/DAI_VAN_RESEARCH.md` (comprehensive Dai Van calculation research)
- `.planning/research/PITFALLS.md` (v1.2 pitfalls patterns for Ten Gods and Kua)
- `.planning/research/ARCHITECTURE.md` (v1.2 architecture patterns, additive integration)
- `crates/amlich-core/src/almanac/types.rs` (DayFortune pattern, optional fields, evidence metadata)
- `crates/amlich-core/src/almanac/calc.rs` (orchestrator pattern, Ten Gods integration example)
- `crates/amlich-core/src/almanac/thap_than.rs` (deterministic Ten Gods engine)
- `crates/amlich-core/src/almanac/tu_menh.rs` (Kua calculator with convention handling)
- `crates/amlich-core/src/almanac/golden_loader.rs` (golden dataset pattern for KHCBPPT alignment)
- `vietnamese_lunar_engine_tables.md` Section 15 (Dai Van calculation formulas, used as source until KHCBPPT verified)

---
*Pitfalls research: Dai Van (Great Cycle) integration (v1.3)*
