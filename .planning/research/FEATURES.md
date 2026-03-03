# Features Research

**Domain:** Dai Van (Đại Vận/大運 - 10-Year Major Luck Cycles) for Vietnamese almanac
**Researched:** 2026-03-03
**Confidence:** MEDIUM

## Feature Landscape

### Table Stakes (Users Expect These)

Features users assume exist. Missing these = product feels incomplete.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| 8-pillar Dai Van calculation | Any production Bazi-style engine provides 10-year luck cycle projections | LOW | Already researched with clear 6-step formula. Matches standard Bazi methods. **Confidence: HIGH** |
| Chieuthu (Chiều) direction determination (Thuận/Nghịch) | Classical Bazi systems always specify forward/backward progression | LOW | Simple rule: (Year polarity × gender) → Thuận (+1) or Nghịch (-1). **Confidence: HIGH** |
| Start age calculation from Tiết Khí distance | 10-year cycles must begin at specific age, not birth | MEDIUM | Uses 3-days = 1-year conversion to nearest solar term. Reuses existing `get_days_to_nearest_tiet_khi()` helper. **Confidence: HIGH** |
| Pillar generation with contiguous age ranges | Users expect complete life cycle coverage (8 pillars, 0-80+ years) | LOW | Stem/branch advance by chieuthu (+1 or -1) across 8 iterations. **Confidence: HIGH** |
| Ten Gods correlation per pillar | Bazi analysis always relates pillar Can to Day Stem via Thập Thần | MEDIUM | Already implemented `get_thap_than()` can be called for each pillar. **Confidence: HIGH** |
| Backward-compatible DayFortune/API integration | Milestone goal is additive integration without regression | MEDIUM | New field must be optional with `#[serde(skip_serializing_if = "Option::is_none")]`. **Confidence: HIGH** |
| Evidence metadata for traceability | Matches amlich's KHCBPPT correctness posture | LOW | Follow existing `RuleEvidence` pattern with source_id/method/profile. **Confidence: HIGH** |

### Differentiators (Competitive Advantage)

Features that set the product apart. Not required, but valuable.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Kua-based directional analysis per pillar | Links 10-year luck cycles with feng-shui fortune directions | HIGH | Birth Kua (calculated once) provides constant favorable/unfavorable directions. Each pillar's element can be analyzed against these for period-specific recommendations. **Confidence: MEDIUM** |
| Current pillar identification for any age | Users can instantly know which 10-year period they're in | LOW | Helper function `get_current_pillar()` finds pillar for given age. **Confidence: HIGH** |
| Years-to-next-transition calculation | Planning tool for upcoming life phase changes | LOW | `years_to_next_transition()` counts down to pillar boundary. **Confidence: HIGH** |
| Convention-tagged metadata | Makes calculation assumptions auditable and reproducible | LOW | Document year_basis, start_age_method, gender_encoding, source_id, method. **Confidence: HIGH** |
| Pillar-specific Ten Gods on-demand | Efficiency: only calculate when needed, not pre-compute all | MEDIUM | Lazy calculation via `get_ten_gods_for_pillar(thu_tu, day_stem)` reduces unnecessary computation. **Confidence: HIGH** |
| Birth-hour flexibility | Advanced users need day stem (from birth hour) for complete Ten Gods analysis | LOW | Optional birth_hour parameter enables day pillar calculation when birth time known. **Confidence: MEDIUM** |

### Anti-Features (Commonly Requested, Often Problematic)

Features that seem good but create problems.

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| Human-language fortune interpretation paragraphs | Users ask for "full reading" once Dai Van exists | Non-deterministic, unverifiable, and outside v1.3 correctness scope | Return structured factors only (pillars, Ten Gods, Kua). Interpretation belongs to later separate milestone |
| Composite "fortune score" combining Dai Van + Ten Gods + Kua | Seems user-friendly for ranking phases | Pseudo-precision and heavy source ambiguity; likely causes trust regressions | Keep atomic outputs; let clients build scoring with explicit disclaimers |
| Real-time prediction of life events | Users want career/marriage timing from pillars | Over-claims from deterministic calculation; not scientifically verifiable | Provide only pillar characteristics (energies, Ten Gods nature, directional favorability). Let practitioners interpret timing |
| Silent auto-correction for missing/invalid birth data | Convenience API design | Hides assumptions and creates inconsistent results between clients | Fail fast with typed validation errors and require explicit birth date + gender |
| Pre-calculating all 8 pillars' Ten Gods eagerly | Optimization idea | Most users only need current pillar; eager calculation wastes cycles | Lazy on-demand calculation per pillar is more efficient for typical use cases |
| Separate public API for birth fortune | Separation of concerns principle | Adds new API surface to maintain, duplicates DayFortune fields | Add Dai Van as optional field to existing DayFortune instead. Keep API minimal. |
| Expanding to "Tiểu Vận" (小运 - yearly/decadal luck) | Natural next request after major cycles | Major scope expansion, different calculation rules, unclear KHCBPPT coverage | Strict v1.3 boundary: only 10-year Đại Vân. Defer smaller cycles to future milestone |

## Feature Dependencies

```
[Existing Lunar Date Conversion]
    └──requires──> [Year Can Chi + Month Can Chi]
                       └──requires──> [Year Polarity (Âm/Dương)]
                                      └──requires──> [Chieuthu Determination (Year × Gender)]

[Existing Tiết Khí Helper]
    └──requires──> [Start Age Calculation]
                       └──requires──> [3 Days per Year Conversion]

[Existing Ten Gods Engine]
    └──enhances──> [Pillar Ten Gods Correlation]
                        └──requires──> [Day Stem from Birth Hour]

[Existing Kua Calculator]
    └──enhances──> [Kua-Based Directional Analysis]

[Base Pillar: Month Can Chi + Chieuthu + Start Age]
    └──requires──> [8-Pillar Generation (Stem/Branch Advancement)]
                       └──produces──> [Dai Van Result]
                                          └──optionally-enhances──> [Ten Gods per Pillar]
                                          └──optionally-enhances──> [Kua Directional Analysis per Pillar]

[Dai Van Result]
    └──integrates-with──> [DayFortune as Optional Field]
```

### Dependency Notes

- **Dai Van requires birth date + gender (not just day):** Different calculation model than existing DayFortune. Cannot be computed from single date alone.
- **Ten Gods correlation requires birth hour (optional):** Day stem needed for pillar-to-Day Ten Gods. Must support unknown birth hour gracefully (Ten Gods = None or day_fortune-based targets).
- **Tiết Khí helper already implemented in v1.1.3:** Reuse `get_days_to_nearest_tiet_khi()` directly. No new solar term calculation needed.
- **Kua integration is birth-level, not pillar-level:** Kua calculated once from birth year + gender. Each pillar analysis reuses same Kua result.
- **Backward compatibility is critical:** Must not break existing DayFortune consumers. Use optional field with skip_serializing_if.
- **KHCBPPT source verification pending:** Classical reference (卷六 or Quyển 12-13) needs manual lookup. Use standard Bazi formulas with placeholder source_id initially.

## MVP Definition (for this milestone)

### Launch With (v1.3)

- [ ] 8-pillar Dai Van calculation with Chieuthu direction
  - Input: birth_date (Gregorian) + gender
  - Output: DaYunResult with pillars, start_age, chieu_thu, convention metadata
  - Why essential: Core Dai Van system; users expect complete life cycle projection
  - Confidence: HIGH

- [ ] Start age calculation from Tiết Khí distance
  - Reuse: get_days_to_nearest_tiet_khi() from v1.1.3
  - Formula: |days_to_tiet_khi| / 3
  - Why essential: Determines when first pillar begins
  - Confidence: HIGH

- [ ] Pillar generation with contiguous 10-year age ranges
  - Input: month Can Chi (base pillar) + chieu_thu (+1 or -1)
  - Process: Advance stem/chi by chieuthu, iterate 8 times
  - Output: 8 DaYunPillar structs with thu_tu, start_age, end_age, can, chi, can_chi_name
  - Why essential: Users need to know each period's characteristics
  - Confidence: HIGH

- [ ] Optional Ten Gods correlation per pillar (lazy/on-demand)
  - Function: get_ten_gods_for_pillar(thu_tu, day_stem) -> Option<ThapThanResult>
  - Integration: Reuse existing get_thap_than() from thap_than.rs
  - Why essential: Classical Bazi always relates pillar to Day Stem
  - Confidence: HIGH

- [ ] Backward-compatible DayFortune/API integration
  - Add: pub dai_van: Option<DaYunResult> to DayFortune
  - Attribute: #[serde(skip_serializing_if = "Option::is_none")]
  - Test: Existing DayFortune serialization unchanged when dai_van = None
  - Why essential: No regression for existing day-only queries
  - Confidence: HIGH

- [ ] Convention metadata with evidence traceability
  - Fields: year_basis, start_age_method, gender_encoding, source_id, method
  - Default values: "lunar", "3-days-per-year", "enum(Male,Female)", "khcbppt", "bai-quyet"
  - Why essential: Matches amlich's correctness posture and auditability
  - Confidence: HIGH

### Add After Validation (v1.3.x)

- [ ] Kua-based directional analysis per pillar
  - Function: analyze_pillar_with_kua(pillar, birth_year, gender)
  - Output: Favorable/unfavorable directions for specific period
  - Trigger: User requests directional guidance for life phase planning
  - Complexity: MEDIUM (requires mapping pillar elements to Kua direction sets)

- [ ] Birth-hour aware Ten Gods calculation
  - Input: birth_date + birth_hour + gender
  - Output: Full Ten Gods analysis (Day Pillar + all 8 Dai Van pillars)
  - Trigger: Advanced users know exact birth time
  - Complexity: LOW (day pillar extraction exists)

- [ ] Helper functions for common queries
  - get_current_pillar(dai_yun, current_age) -> Option<DaYunPillar>
  - years_to_next_transition(dai_yun, current_age) -> Option<u8>
  - get_pillar_at_age(dai_yun, target_age) -> Option<DaYunPillar>
  - Trigger: UI needs to show user's current life phase or countdown
  - Complexity: LOW (simple lookup/iteration)

### Future Consideration (v2.0+)

- [ ] Tiểu Vận (小运 - yearly luck) cycles
  - Different calculation rules (monthly/seasonal)
  - Requires separate research and KHCBPPT verification
  - Why defer: Out of scope for v1.3 milestone
  - Complexity: HIGH (new subsystem, different rules)

- [ ] Fate interpretation/reporting layer
  - Natural language readings, career/marriage timing predictions
  - Practitioner-specific, highly subjective
  - Why defer: Non-deterministic, outside correctness-critical scope
  - Complexity: VERY HIGH (requires practitioner knowledge base, AI/ML for language generation)

- [ ] Interactive CLI/API for birth fortune queries
  - Separate birth-based API surface (calculate_birth_fortune vs calculate_day_fortune)
  - User education on when to use which API
  - Why defer: API design decision affects broader system, needs UX research
  - Complexity: MEDIUM (new CLI commands, documentation, migration guide)

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
|---------|------------|---------------------|----------|
| 8-pillar Dai Van calculation (Chieuthu, pillars, start age) | HIGH | MEDIUM | P1 |
| Start age from Tiết Khí distance (reuse v1.1.3 helper) | HIGH | LOW | P1 |
| Backward-compatible DayFortune/API integration (optional field) | HIGH | MEDIUM | P1 |
| Convention metadata with evidence traceability | HIGH | LOW | P1 |
| Ten Gods correlation per pillar (lazy, on-demand) | MEDIUM | LOW | P2 |
| Helper functions (current_pillar, years_to_next, get_pillar_at_age) | MEDIUM | LOW | P2 |
| Kua-based directional analysis per pillar | MEDIUM | MEDIUM | P3 |
| Birth-hour aware Ten Gods (day pillar extraction) | LOW | MEDIUM | P3 |
| Fate interpretation/reporting layer | LOW | VERY HIGH | P4 |
| Tiểu Vận (yearly/decadal luck) | LOW | HIGH | P4 |

**Priority key:**
- P1: Must have for launch (v1.3)
- P2: Should have, add when possible (v1.3.x)
- P3: Nice to have, future consideration (v2.0+)
- P4: Out of scope or defer until later milestone

## Recommended Phase Structure for v1.3

Based on dependencies and priorities, recommend 3-phase implementation:

### Phase 1: Core Dai Van Calculation (Weeks 1-2)
**Focus:** Implement deterministic calculation logic without integration

**Features:**
- 8-pillar generation from month Can Chi + chieu_thu
- Start age calculation from Tiết Khí distance
- Chieuthu direction determination (year polarity × gender)
- Unit tests for all calculation steps

**Rationale:**
- Addresses: Core Dai Van computation (P1)
- Avoids: Integration complexity and API surface changes
- Builds: Foundation that Ten Gods/Kua can enhance later

**Prerequisites:**
- v1.1.3 completion (Tiết Khí helper already available)
- Can Chi module (already implemented)

### Phase 2: Ten Gods Integration and Helpers (Weeks 3-4)
**Focus:** Connect Dai Van with existing Ten Gods engine

**Features:**
- Lazy Ten Gods correlation per pillar (get_ten_gods_for_pillar)
- Helper functions: get_current_pillar, years_to_next_transition, get_pillar_at_age
- Integration tests between dai_van and thap_than modules

**Rationale:**
- Addresses: Ten Gods correlation (P2)
- Addresses: Helper functions (P2)
- Leverages: Existing thap_than.rs implementation
- Efficient: On-demand calculation, not pre-compute all

**Prerequisites:**
- Phase 1 complete (core Dai Van working)

### Phase 3: API Integration and Kua Analysis (Weeks 5-6)
**Focus:** Public API exposure and optional Kua directional analysis

**Features:**
- Backward-compatible DayFortune integration (optional dai_van field)
- Kua-based directional analysis per pillar (optional enhancement)
- Convention metadata documentation
- Full regression suite including all subsystems

**Rationale:**
- Addresses: API integration (P1)
- Addresses: Kua directional analysis (P3)
- Addresses: Convention metadata (P1)
- Ensures: Zero regressions in existing v1.2 features

**Prerequisites:**
- Phase 2 complete (Ten Gods integration tested)
- Existing Kua calculator (already implemented)

**Phase ordering rationale:**
- Core calculation first → ensures algorithm is correct before connecting to other systems
- Ten Gods second → builds on working core, adds deterministic integration
- API/Kua third → last to integrate, ensures no breaking changes until everything works

**Research flags for phases:**
- Phase 1: Standard Bazi formulas, LOW research risk (algorithms well-documented)
- Phase 2: Existing ThapThan module, LOW research risk (integration pattern clear)
- Phase 3: API design decision (optional field vs. separate API), MEDIUM research risk (consider backward compatibility patterns)

## Sources

### High Confidence

- Internal research: `.planning/research/DAI_VAN_RESEARCH.md` — Comprehensive Dai Van calculation formulas, types, and integration approach (HIGH)
- Existing implementation: `.planning/research/STACK.md` — Detailed Rust code templates and data structures for dai_van.rs (HIGH)
- Existing Ten Gods implementation: `crates/amlich-core/src/almanac/thap_than.rs` — Deterministic 10x10 matrix mapping (HIGH)
- Existing Kua implementation: `crates/amlich-core/src/almanac/tu_menh.rs` — Kua number, group, directions with convention metadata (HIGH)
- v1.1.3 milestone plan: `.planning/phases/v1.1-foundation-extensions/v1.1-03-PLAN.md` — Tiết Khí helper get_days_to_nearest_tiet_khi() specification (HIGH)
- PROJECT.md context: `.planning/PROJECT.md` — Milestone scope and existing capabilities (KHCBPPT alignment, Ten Gods, Kua) (HIGH)

### Medium Confidence

- Wikipedia "Four Pillars of Destiny" — Mentions "10-year luck cycle (Chinese: 十年大运)" and shows example from Hirohito's chart with 8 pillars 辛卯, 庚寅, etc. (MEDIUM)
- Wikipedia "Sexagenary cycle" — Explains Can Chi (干支) system of 10 Heavenly Stems + 12 Earthly Branches used in Dai Van pillars (MEDIUM)

### Low Confidence (Unverified, Needs Validation)

- KHCBPPT classical reference:卷六 (Volume 6) or Quyển 12-13 (Công Quy section) — Cited in DAI_VAN_RESEARCH.md but requires manual lookup to verify exact calculation rules (LOW)
- Vietnamese lunar engine tables: Section 15 "Đại Vận" — Provides formulas and code templates but needs cross-verification with classical sources (LOW)
- Modern Vietnamese numerology sites — Often have simplified explanations but should be cross-checked against classical sources (LOW)

### Conflicting/Ambiguous Sources (Need Resolution)

- Start age edge cases: Different schools may round differently (truncate vs. nearest integer). Project should pick one convention and document explicitly.
- Leap month handling for base pillar: Traditional Bazi may treat leap month differently for month Can Chi calculation. Use existing get_month_canchi() with leap_month indicator.

---
*Feature research for: Dai Van (Đại Vận/大運) integration — v1.3 milestone*
*Researched: 2026-03-03*
