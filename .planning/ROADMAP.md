# Roadmap: Amlich Almanac Correctness Audit

## Milestones

- ✅ **v1.0 KHCBPPT Alignment Complete** - archived in `.planning/MILESTONES.md`
- ✅ **v1.1 Foundation Extensions** - shipped 2026-03-02 (9/9 plans)
- ✅ **v1.2 Ten Gods and Kua Foundation** - shipped 2026-03-02 (3/3 plans)

## Overview

Milestone v1.2 delivers deterministic Ten Gods and Tu Menh/Kua foundations, then integrates these outputs into DayFortune and API/serialization contracts in additive form so existing consumers remain stable.

## Phases

**Phase Numbering:**
- Integer milestone phase: `v1.2`
- Decimal insertions within milestone: `v1.2.1`, `v1.2.2`

- [ ] **Phase v1.2: Ten Gods Deterministic Foundation** - Deliver deterministic Ten Gods computation with typed semantics and full matrix correctness evidence.
- [ ] **Phase v1.2.1: Tu Menh/Kua Deterministic Foundation** - Deliver typed Kua computation, documented conventions, and direction outputs with fixture-backed confidence.
- [ ] **Phase v1.2.2: Additive Integration and Compatibility Gate** - Expose Ten Gods/Kua through DayFortune and API/serialization with deterministic population and regression safety.

## Phase Details

### Phase v1.2: Ten Gods Deterministic Foundation
**Goal**: Users can reliably obtain deterministic Ten Gods relations with stable typed semantics and auditable evidence.
**Depends on**: v1.1 complete
**Requirements**: TT-01, TT-02, TT-03, TT-04, TT-05
**Success Criteria** (what must be TRUE):
  1. User can request Ten Gods relation from a day stem to any target stem and receive deterministic, repeatable results.
  2. User can observe stable Ten Gods labels and polarity semantics in typed outputs across repeated runs.
  3. User can inspect rule-evidence metadata attached to Ten Gods results for auditability.
  4. User can trust correctness because all 10x10 stem combinations are validated and pass.
  5. User can consume Ten Gods JSON fields with stable, backward-compatible names.
**Plans**: 3
- [x] v1.2-01-PLAN.md — Ten Gods foundation engine and matrix tests (Complete)
- [x] v1.2-02-PLAN.md — Tu Menh/Kua calculator and fixture coverage (Complete)
- [x] v1.2-03-PLAN.md — DayFortune/API integration with backward compatibility (Complete)

### Phase v1.2.1: Tu Menh/Kua Deterministic Foundation
**Goal**: Users can compute and interpret Tu Menh/Kua deterministically under one explicit project convention.
**Depends on**: Phase v1.2
**Requirements**: TM-01, TM-02, TM-03, TM-04, TM-05
**Success Criteria** (what must be TRUE):
  1. User can compute Kua from birth year and gender using one documented convention.
  2. User can see and verify documented behavior for year-boundary and edge-case handling.
  3. User can receive typed Kua output including Kua number and East/West group.
  4. User can receive favorable and unfavorable direction sets derived from computed Kua.
  5. User can verify Kua behavior against representative fixtures and source-linked assumptions.
**Plans**: TBD

### Phase v1.2.2: Additive Integration and Compatibility Gate
**Goal**: Users can access new Ten Gods/Kua outputs through DayFortune and API contracts without breaking existing integrations.
**Depends on**: Phase v1.2.1
**Requirements**: INT-01, INT-02, INT-03, INT-04, INT-05, INT-06
**Success Criteria** (what must be TRUE):
  1. User receives Ten Gods and Kua fields in DayFortune and API/DTO outputs as additive, non-breaking extensions.
  2. User observes deterministic population of new fields only when required inputs are present.
  3. User can rely on stable JSON serialization for all newly introduced fields.
  4. User can trust compatibility via passing contract/integration tests and green `cargo test --package amlich-core` regression gate.
**Plans**: TBD

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| v1.2 Ten Gods Deterministic Foundation | 3/3 | Complete | 2026-03-02 |
| v1.2.1 Tu Menh/Kua Deterministic Foundation | 0/TBD | Not started | - |
| v1.2.2 Additive Integration and Compatibility Gate | 0/TBD | Not started | - |
