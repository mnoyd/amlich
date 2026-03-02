# Requirements: Amlich Almanac Correctness Audit

**Defined:** 2026-03-02
**Core Value:** Every almanac subsystem in amlich must match KHCBPPT for 2020-2030 with test-backed evidence.

## v1 Requirements

Requirements for milestone `v1.2` (Ten Gods and Kua Foundation). Each maps to exactly one phase in this milestone roadmap.

### Ten Gods Engine

- [ ] **TT-01**: User can get deterministic Ten Gods relation from day stem to any target stem.
- [ ] **TT-02**: User can rely on stable Ten Gods labels and polarity semantics in typed output.
- [ ] **TT-03**: User can consume Ten Gods results with rule evidence metadata for auditability.
- [ ] **TT-04**: User can trust full matrix correctness validated across all 10x10 stem combinations.
- [ ] **TT-05**: User can consume Ten Gods JSON output with backward-compatible, stable field names.

### Tu Menh and Kua

- [ ] **TM-01**: User can compute Kua from birth year and gender with one explicit project convention.
- [ ] **TM-02**: User can see documented behavior for year-boundary and Kua edge-case handling.
- [ ] **TM-03**: User can get typed Kua output including number and East/West group.
- [ ] **TM-04**: User can get favorable and unfavorable direction sets derived from computed Kua.
- [ ] **TM-05**: User can verify Kua correctness via representative fixtures and source-linked assumptions.

### Integration and Compatibility

- [ ] **INT-01**: User can receive new Ten Gods and Kua fields in DayFortune without breaking existing consumers.
- [ ] **INT-02**: User can access new Ten Gods and Kua outputs through API/DTO surfaces in additive form.
- [ ] **INT-03**: User can get deterministic population of new fields only when required inputs are present.
- [ ] **INT-04**: User can rely on stable JSON serialization for all newly added fields.
- [ ] **INT-05**: User can trust backward compatibility validated by contract and integration tests.
- [ ] **INT-06**: User can trust full regression safety with `cargo test --package amlich-core` green.

## v2 Requirements

Deferred to later milestones.

### Dai Van Expansion

- **DV-01**: User can compute Dai Van period transitions from birth context.
- **DV-02**: User can correlate Dai Van periods with Ten Gods and Kua outputs.
- **DV-03**: User can consume Dai Van via stable API and serialization contracts.

## Out of Scope

| Feature | Reason |
|---------|--------|
| Full interpretive fortune narratives | Not deterministic/auditable for this correctness milestone |
| Composite fortune scoring | Creates pseudo-precision and weak traceability |
| Alternate Kua convention modes | Defer until a concrete interoperability need appears |
| UI/CLI redesign | Backend correctness and contract stability are priority |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| TT-01 | Phase v1.2 | Pending |
| TT-02 | Phase v1.2 | Pending |
| TT-03 | Phase v1.2 | Pending |
| TT-04 | Phase v1.2 | Pending |
| TT-05 | Phase v1.2 | Pending |
| TM-01 | Phase v1.2.1 | Pending |
| TM-02 | Phase v1.2.1 | Pending |
| TM-03 | Phase v1.2.1 | Pending |
| TM-04 | Phase v1.2.1 | Pending |
| TM-05 | Phase v1.2.1 | Pending |
| INT-01 | Phase v1.2.2 | Pending |
| INT-02 | Phase v1.2.2 | Pending |
| INT-03 | Phase v1.2.2 | Pending |
| INT-04 | Phase v1.2.2 | Pending |
| INT-05 | Phase v1.2.2 | Pending |
| INT-06 | Phase v1.2.2 | Pending |

**Coverage:**
- v1 requirements: 16 total
- Mapped to phases: 16
- Unmapped: 0

---
*Requirements defined: 2026-03-02*
*Last updated: 2026-03-02 after milestone v1.2 initialization*
