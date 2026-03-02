# Requirements: Milestone v1.1 - Foundation Extensions

**Status:** Active
**Created:** 2026-03-02
**Milestone:** v1.1 - Foundation Extensions for Vietnamese Lunar Calendar Engine

---

## Overview

This milestone extends the existing amlich almanac with low-to-medium complexity subsystems that provide foundational infrastructure for advanced features (Thập Thần, Tứ Mệnh, Đại Vận) in future milestones.

**Goal:** Add 3 subsystems to complete foundation for advanced astrological calculations

**Dependencies:** None (builds on v1.0 completion)

---

## Objectives

1. **Complete Xung Hợp relationship system** — Add missing relationship types (Lục hợp, Tương hại, Tương hình) to provide full branch relationship analysis
2. **Implement Tàng Can (Hidden Stems)** — Add hidden Heavenly Stems for each Địa Chi with strength values, enabling advanced Bazi calculations
3. **Add Tiết Khí helper functions** — Calculate days from date to nearest Solar Term for Đại Vận compatibility

---

## Requirements

### Category 1: Enhanced Xung Hợp Relationships

| ID | Requirement | Description | Priority |
|----|-------------|-------------|----------|
| XH-01 | Lục Hợp (Six Harmonies) | Implement 6 harmony pairs: 子丑, 寅亥, 卯戌, 辰酉, 巳申, 午未. Each pair combines to form a specific Ngũ Hành (element). | HIGH |
| XH-02 | Tương Hại (Mutual Harms) | Implement 6 harm pairs: 子未, 丑午, 寅酉, 卯申, 辰亥, 巳戌. These are weaker conflicts than Lục xung. | HIGH |
| XH-03 | Tương Hình (Mutual Punishments) | Implement 4 punishment groups: 寅卯巳 (Vô恩之刑), 子辰丑 (恃势之刑), 申酉亥 (无礼之刑), 午午 (自刑 - self-punishment). | HIGH |
| XH-04 | Update XungHopResult struct | Extend existing `XungHopResult` to include `liu_he`, `xiang_hai`, and `xiang_xing` fields alongside existing `luc_xung`, `tam_hop`, `tu_hanh_xung`. | HIGH |
| XH-05 | Add constants to xung_hop.rs | Define `LIUHE`, `XIANGHAI`, and `XIANGXING` constant tables following existing pattern of `LIUCHONG`, `SANHE`, `TUHANHXUNG`. | HIGH |
| XH-06 | Update get_xung_hop() function | Modify `get_xung_hop(chi_index)` to return all 6 relationship types instead of current 3. | HIGH |
| XH-07 | Add tests for new relationships | Create unit tests for all 6 relationship types with symmetry verification (like existing `luc_xung_all_pairs_symmetric`). | MEDIUM |
| XH-08 | Integration verification | Ensure new relationships are populated in `calculate_day_fortune()` output (JSON serialization includes new fields). | MEDIUM |

**Acceptance Criteria:**
- All 6 relationship types (Lục xung, Tam hợp, Tứ hành xung, Lục hợp, Tương hại, Tương hình) are calculated correctly
- Tests pass for symmetry and coverage
- JSON output includes all 6 relationship types
- No breaking changes to existing `luc_xung`, `tam_hop`, `tu_hanh_xung` calculations

---

### Category 2: Tàng Can (Hidden Stems)

| ID | Requirement | Description | Priority |
|----|-------------|-------------|----------|
| TC-01 | TangCan struct definition | Define `TangCan` struct with fields: `main: &'static str`, `central: &'static str`, `residual: &'static str`, `strength: [u8; 3]`. | HIGH |
| TC-02 | TangCan strength constants | Define `CANGAN_STRENGTH: [[u8; 3]; 12]` constant table with strength values [100, 0, 0] for 子, [60, 25, 15] for others. | HIGH |
| TC-03 | TangCan content constants | Define `CANGAN: [[&str; 3]; 12]` constant table with hidden stems for all 12 Địa Chi. | HIGH |
| TC-04 | Add TangCan module | Create `almanac/tang_can.rs` module with `get_tang_can(chi: &str) -> TangCan` function. | HIGH |
| TC-05 | Update baseline.json schema | Add `tang_can_meta` section with `source_id: "khcbppt"` and `tang_can_by_chi` lookup table. | HIGH |
| TC-06 | TangCan to baseline.json | Populate `tang_can_by_chi` with all 12 branches' hidden stem arrays (e.g., 子: ["癸", "", ""], 丑: ["己", "癸", "辛"]). | HIGH |
| TC-07 | Integration into DayFortune | Add `pub tang_can: Option<TangCan>` field to `DayFortune` struct in `almanac/types.rs`. | HIGH |
| TC-08 | Populated in calculate_day_fortune() | Modify `calculate_day_fortune()` to call `get_tang_can(&day_canchi.chi)` and populate `tang_can` field. | MEDIUM |
| TC-09 | Add TangCan tests | Create unit tests verifying all 12 branches return correct hidden stems and strengths. | MEDIUM |

**Acceptance Criteria:**
- TangCan struct defined with all required fields
- All 12 Địa Chi return correct hidden stems per `vietnamese_lunar_engine_tables.md`
- Strength values match specification ([100, 0, 0], [60, 25, 15])
- `get_tang_can()` function exists and is callable
- JSON output includes `tang_can` field with correct data
- Tests pass for all 12 branches

---

### Category 3: Tiết Khí Helper Functions

| ID | Requirement | Description | Priority |
|----|-------------|-------------|----------|
| TK-01 | Define helper function | Add `get_days_to_nearest_tiet_khi(jd: i32) -> i32` function to `tietkhi.rs`. | HIGH |
| TK-02 | Find nearest Tiết Khí before | Locate most recent Tiết Khí with JD <= input JD. | HIGH |
| TK-03 | Find nearest Tiết Khí after | Locate next Tiết Khí with JD > input JD. | HIGH |
| TK-04 | Calculate signed difference | Return signed difference (negative = before, positive = after) with smaller absolute value preferred. | HIGH |
| TK-05 | Handle edge cases | Correctly handle dates exactly on a Tiết Khí (return 0), and dates far from boundaries. | MEDIUM |
| TK-06 | Add unit tests | Create tests for dates before, on, and after Tiết Khí boundaries. | MEDIUM |

**Acceptance Criteria:**
- `get_days_to_nearest_tiet_khi()` function exists in `tietkhi.rs`
- Returns signed integer (negative/zero/positive)
- Returns 0 when input JD exactly matches a Tiết Khí
- Returns negative value when nearest is before, positive when after
- Returns value in days (not hours or other units)
- Tests pass for edge cases (on boundary, far from boundary)
- Function is accessible from other modules (pub fn)

---

## Out of Scope

The following are explicitly NOT in scope for this milestone:

- **Thập Thần (Ten Gods)** — Deferred to Milestone v1.2 (High complexity)
- **Tứ Mệnh (Kua)** — Deferred to Milestone v1.2 (Medium complexity)
- **Đại Vận (Major Luck)** — Deferred to Milestone v1.3 (Very High complexity)
- **Hour pillar calculations** — Not needed for day-level almanac
- **Birth chart analysis** — Requires Tứ Mệnh and Đại Vận (future milestones)
- **UI/CLI changes** — Pure backend enhancements only

---

## Success Criteria

Milestone v1.1 is complete when ALL of the following are TRUE:

1. ✅ All 6 Xung Hợp relationship types are implemented and tested (XH-01 through XH-08)
2. ✅ Tàng Can subsystem is implemented with correct data for all 12 branches (TC-01 through TC-09)
3. ✅ Tiết Khí helper function exists and is tested (TK-01 through TK-06)
4. ✅ All new subsystems are integrated into `DayFortune` struct and populated in `calculate_day_fortune()`
5. ✅ JSON serialization includes all new fields (`liu_he`, `xiang_hai`, `xiang_xing`, `tang_can`)
6. ✅ All unit tests pass (cargo test --package amlich-core)
7. ✅ No breaking changes to existing functionality (all v1.0 tests still pass)
8. ✅ Code follows existing patterns (evidence tracking, const tables, clear separation of concerns)

---

## Dependencies

### External Dependencies
- **None** — No new external libraries required

### Internal Dependencies
- **v1.0 milestone completion** — All KHCBPPT-verified subsystems operational
- **Existing type system** — `types.rs`, `almanac/types.rs` provide foundation
- **Existing modules** — `xung_hop.rs`, `tietkhi.rs` to be extended

---

## Technical Constraints

- **Rust edition:** 2021 (same as existing codebase)
- **No-std compatibility:** Not required (this is library code)
- **Serialization:** Must use existing `serde` pattern for JSON output
- **Evidence tracking:** All new calculations should include `evidence: Option<RuleEvidence>` field
- **Test coverage:** Follow v1.0 pattern (unit tests + golden dataset validation)

---

## Data Requirements

### Xung Hợp Data
- **Lục hợp table:** 6 pairs (子丑, 寅亥, 卯戌, 辰酉, 巳申, 午未)
- **Tương hại table:** 6 pairs (子未, 丑午, 寅酉, 卯申, 辰亥, 巳戌)
- **Tương hình table:** 4 groups with 3-4 members each
- **Source:** KHCBPPT vols 1-8 (if available) or universal Bazi sources

### Tàng Can Data
- **12 branches:** Complete coverage of all Địa Chi (子 through 亥)
- **3 stems per branch:** Main (chính), Central (trung), Residual (dư) - some branches have empty strings for missing stems
- **Strength values:** [100, 0, 0], [60, 25, 15] per specification
- **Source:** KHCBPPT vols 1-2 (Bon Nguyên section) for verification

### Tiết Khí Data
- **24 solar terms:** Already defined in existing `JIEQI` constant
- **JD values:** Can reuse existing Tiết Khí calculation logic
- **No new data needed:** Pure algorithmic calculation

---

## Risk Assessment

### Low Risk ✅
- **Xung Hợp extensions:** Simple constant tables, well-tested pattern
- **Tiết Khí helper:** Extension of existing module, no new data

### Medium Risk ⚠️
- **Tàng Can data:** Requires KHCBPPT verification for hidden stem values
- **Integration complexity:** Multiple new fields in `DayFortune` may require careful testing

### High Risk ❌
- **None** — All subsystems in this milestone are low-to-medium complexity

---

## Estimated Effort

| Subsystem | Complexity | Estimated Time |
|-----------|-------------|-----------------|
| Enhanced Xung Hợp | Low | 1-2 hours |
| Tàng Can | Medium | 2-3 hours |
| Tiết Khí helper | Low | 1 hour |
| Integration & Testing | Medium | 2-3 hours |
| Documentation & Verification | Low | 1 hour |

**Total Estimated Effort:** 7-10 hours

---

## Verification Strategy

### Phase 1: Unit Tests
- Create test functions for each new requirement
- Test edge cases and boundary conditions
- Ensure all tests pass with `cargo test`

### Phase 2: Integration Tests
- Verify new fields populate in `calculate_day_fortune()`
- Check JSON serialization includes new fields
- Ensure no breaking changes to existing tests

### Phase 3: Manual Verification
- Review generated JSON output for sample dates
- Verify Tàng Can values match specification
- Check Xung Hợp relationships are correct

### Phase 4: Regression Testing
- Run full test suite to ensure v1.0 tests still pass
- Verify no degradation in existing functionality

---

## Open Questions

1. **KHCBPPT verification needed?** Should we research KHCBPPT vols 1-2 for Tàng Can and enhanced Xung Hợp, or trust the specification document as authoritative?

2. **API surface?** Should all new subsystems be exposed through `calculate_day_fortune()` (simpler for users), or create separate APIs (cleaner separation)?

3. **Empty Tàng Can handling?** How should branches with empty hidden stems (子, 卯, 酉) be represented in JSON - as empty strings, null, or omitted fields?

---

*Requirements created: 2026-03-02*
*Next: Phase 1 planning (requirements → tasks → plans)*
