# Requirements: Amlich v1.3 - Dai Van Core

**Defined:** 2026-03-03
**Core Value:** Every almanac subsystem in amlich must produce output matching KHCBPPT for 2020-2030 with test-backed, traceable evidence.

## v1 Requirements

Requirements for Dai Van Core milestone. Each maps to roadmap phases.

### Core Calculation

- [ ] **DV-CALC-01**: System can calculate 8 Dai Van pillars from birth date (Gregorian) and gender
- [ ] **DV-CALC-02**: System determines Chieuthu direction (Thuận/Nghịch) from year polarity × gender
- [ ] **DV-CALC-03**: System calculates start age from Tiết Khí distance using 3 days = 1 year conversion
- [ ] **DV-CALC-04**: System generates 8 pillars with contiguous 10-year age ranges (start_age inclusive, end_age exclusive)
- [ ] **DV-CALC-05**: System uses month Can Chi as base pillar for Dai Van progression
- [ ] **DV-CALC-06**: System handles edge cases (Tiết Khí boundaries, leap months, year polarity transitions)

### Ten Gods Integration

- [x] **DV-TG-01**: System can correlate each pillar's Heavenly Stem with birth day stem via Thap Than
- [x] **DV-TG-02**: Ten Gods calculation is lazy/on-demand (not pre-computed for all pillars)
- [x] **DV-TG-03**: System supports unknown birth hour gracefully (Ten Gods = None or day_fortune-based targets)

### Kua Analysis

- [ ] **DV-KUA-01**: System can analyze pillar elements against birth Kua directions
- [ ] **DV-KUA-02**: Kua analysis provides favorable/unfavorable directions per pillar
- [ ] **DV-KUA-03**: Birth Kua is calculated once per person and reused for all pillars
- [ ] **DV-KUA-04**: Kua 5 resolution follows project convention (male→8, female→2)

### Helper Functions

- [x] **DV-HELP-01**: System can find current pillar for given age
- [x] **DV-HELP-02**: System can calculate years until next transition
- [x] **DV-HELP-03**: System can find pillar at specific age (range lookup)
- [x] **DV-HELP-04**: Helper functions return Option to handle out-of-range ages gracefully

### Metadata & Traceability

- [ ] **DV-META-01**: System includes convention metadata (year_basis, start_age_method, gender_encoding)
- [ ] **DV-META-02**: System includes evidence metadata (source_id, method)
- [ ] **DV-META-03**: Source_id uses "khcbppt" placeholder with TODO comment for manual verification
- [ ] **DV-META-04**: Method field documents calculation approach (e.g., "bai-quyet" for Bazi formulas)

## v2 Requirements

Deferred to future release. Tracked but not in current roadmap.

### API Integration

- **DV-API-01**: System exposes Dai Van through optional dai_van field in DayFortune
- **DV-API-02**: Optional field uses #[serde(skip_serializing_if = "Option::is_none")] for backward compatibility
- **DV-API-03**: System adds DaYunResultDto and DaYunPillarDto in API layer
- **DV-API-04**: System implements From<> conversion traits for core→DTO mapping
- **DV-API-05**: calculate_day_fortune() signature updated with optional birth inputs (breaking change)

### Extended Features

- **DV-EXT-01**: System supports birth-hour aware Ten Gods calculation (full day pillar analysis)
- **DV-EXT-02**: Tiểu Vận (yearly/decadal luck) cycles implemented
- **DV-EXT-03**: Composite fortune scores combining Dai Van + Ten Gods + Kua

## Out of Scope

Explicitly excluded. Documented to prevent scope creep.

| Feature | Reason |
|---------|--------|
| API Integration (DayFortune optional field) | Deferred to v1.4 to focus on core computation first |
| Human-language fortune interpretation | Non-deterministic, outside correctness-critical scope; belongs to later separate milestone |
| Composite "fortune score" | Pseudo-precision and heavy source ambiguity; likely causes trust regressions |
| Separate BirthFortune API | Adds new API surface to maintain; optional field approach keeps API minimal |
| Real-time prediction of life events | Over-claims from deterministic calculation; not scientifically verifiable |
| Tiểu Vận (小运 - yearly luck) | Major scope expansion, different calculation rules, unclear KHCBPPT coverage |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| DV-CALC-01 | Phase 4 | Pending |
| DV-CALC-02 | Phase 4 | Pending |
| DV-CALC-03 | Phase 4 | Pending |
| DV-CALC-04 | Phase 4 | Pending |
| DV-CALC-05 | Phase 4 | Pending |
| DV-CALC-06 | Phase 4 | Pending |
| DV-TG-01 | Phase 5 | Complete |
| DV-TG-02 | Phase 5 | Complete |
| DV-TG-03 | Phase 5 | Complete |
| DV-KUA-01 | Phase 6 | Pending |
| DV-KUA-02 | Phase 6 | Pending |
| DV-KUA-03 | Phase 6 | Pending |
| DV-KUA-04 | Phase 6 | Pending |
| DV-HELP-01 | Phase 5 | Complete |
| DV-HELP-02 | Phase 5 | Complete |
| DV-HELP-03 | Phase 5 | Complete |
| DV-HELP-04 | Phase 5 | Complete |
| DV-META-01 | Phase 4 | Pending |
| DV-META-02 | Phase 4 | Pending |
| DV-META-03 | Phase 4 | Pending |
| DV-META-04 | Phase 4 | Pending |

**Coverage:**
- v1 requirements: 20 total
- Mapped to phases: 20/20 ✓

---
*Requirements defined: 2026-03-03*
*Last updated: 2026-03-03 after roadmap creation*
