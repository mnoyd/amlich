# Requirements: Milestone v1.2 - Ten Gods and Kua Foundation

**Status:** Queued (after v1.1 completion)
**Defined:** 2026-03-02
**Milestone:** v1.2 - Ten Gods and Kua Foundation
**Core Value Alignment:** Extend the KHCBPPT-verified almanac engine with foundational astrological calculations without regressing v1.0/v1.1 correctness.

---

## v1.2 Requirements

### Thap Than Engine

- [x] **TT-01**: Define Ten Gods relationship model from day stem to target stem (Ty Kien, Kiep Tai, Thuc Than, Thuong Quan, Chinh Tai, Thien Tai, Chinh Quan, That Sat, Chinh An, Thien An).
- [x] **TT-02**: Implement deterministic mapping logic based on stem polarity and generating/controlling element relationships.
- [x] **TT-03**: Provide `get_thap_than(day_can, target_can)` API returning typed result suitable for JSON serialization.
- [x] **TT-04**: Add table-driven tests that verify all 10 day stems x 10 target stems (100 cases) with expected labels.
- [x] **TT-05**: Add evidence metadata for Ten Gods rules/source attribution consistent with existing rule-evidence patterns.

### Tu Menh (Kua) Calculations

- [ ] **TM-01**: Implement Kua number calculation from birth year and gender using established Vietnamese/Asian feng-shui conventions adopted by project sources.
- [ ] **TM-02**: Normalize edge cases (century transitions, year boundaries) with explicit documented behavior.
- [ ] **TM-03**: Provide typed API for Kua result including number, group (East/West), and favorable/unfavorable direction sets.
- [ ] **TM-04**: Add golden-style fixtures and unit tests for representative years across 1900-2099.
- [ ] **TM-05**: Document source references and assumptions for Kua algorithm in code/docs.

### Integration (DayFortune, API, Tests)

- [ ] **INT-01**: Extend `DayFortune` (and related public types) with optional Ten Gods fields needed for day-level output.
- [ ] **INT-02**: Extend relevant API surface to expose Kua calculation outputs without breaking existing callers.
- [ ] **INT-03**: Update `calculate_day_fortune()` (or orchestrating layer) to populate new fields when required inputs are present.
- [ ] **INT-04**: Ensure JSON serialization includes newly added Ten Gods/Kua fields in stable schema form.
- [ ] **INT-05**: Add integration tests confirming new fields are present and backward-compatible with existing outputs.
- [ ] **INT-06**: Full regression pass (`cargo test --package amlich-core`) remains green including v1.0 validators.

## Deferred to v1.3

### Dai Van

- **DV-01**: Đại Vận period computation and transition rules.
- **DV-02**: Đại Vận integration with Ten Gods/Kua context.
- **DV-03**: Public API and serialization for luck-cycle projections.

## Out of Scope (v1.2)

| Feature | Reason |
|---------|--------|
| Đại Vận implementation | Explicitly deferred to milestone v1.3 |
| Birth-chart full reading/report generation | Requires broader subsystem set beyond v1.2 scope |
| UI/CLI presentation upgrades | Backend correctness and API foundation are priority |

## Traceability

| Requirement | Planned Phase | Status |
|-------------|---------------|--------|
| TT-01 | v1.2-P1 | Complete (v1.2-01) |
| TT-02 | v1.2-P1 | Complete (v1.2-01) |
| TT-03 | v1.2-P1 | Complete (v1.2-01) |
| TT-04 | v1.2-P1 | Complete (v1.2-01) |
| TT-05 | v1.2-P1 | Complete (v1.2-01) |
| TM-01 | v1.2-P2 | Pending |
| TM-02 | v1.2-P2 | Pending |
| TM-03 | v1.2-P2 | Pending |
| TM-04 | v1.2-P2 | Pending |
| TM-05 | v1.2-P2 | Pending |
| INT-01 | v1.2-P3 | Pending |
| INT-02 | v1.2-P3 | Pending |
| INT-03 | v1.2-P3 | Pending |
| INT-04 | v1.2-P3 | Pending |
| INT-05 | v1.2-P3 | Pending |
| INT-06 | v1.2-P3 | Pending |

**Coverage:**
- v1.2 requirements: 16 total
- Mapped to phases: 16
- Unmapped: 0

---
*Requirements defined: 2026-03-02*
*Last updated: 2026-03-02 after v1.2-01 completion*
