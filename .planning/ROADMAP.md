# ROADMAP: Amlich v1.4 - Lunar Engine Table Parity

**Created:** 2026-03-03
**Milestone:** v1.4 Lunar Engine Table Parity
**Depth:** Quick
**Status:** In Progress

## Phases

- [x] **Phase 7: Hour Pillar Parity Core** - Deterministic hour-pillar mapping with day-stem seed rules and boundary-safe slot handling (completed 2026-03-03)
- [ ] **Phase 8: Sexagenary Cycle Parity and Validators** - Canonical 60-cycle utilities, rollover correctness, and parity validator suite
- [ ] **Phase 9: Na Am API Surfaces and Contracts** - Pair/index Na Am APIs, schema contracts, and traceability artifact sync

## Phase Details

### Phase 7: Hour Pillar Parity Core

**Goal**: Deliver table-parity hour pillar calculation with complete day-stem grouping and slot boundary correctness

**Depends on**: Nothing (first phase of v1.4)

**Requirements**: HP-01, HP-02, HP-03, HP-04, HP-05, PAR-02

**Success Criteria** (what must be TRUE):
1. Hour pillar is computed deterministically from day stem plus local hour using 12 fixed two-hour branch windows
2. Day-stem seed mapping (Giáp/Kỷ, Ất/Canh, Bính/Tân, Đinh/Nhâm, Mậu/Quý) produces correct stem progression across all 12 windows
3. Boundary times at each window transition are covered by tests with no overlap/gap behavior
4. Returned hour pillar includes evidence metadata fields aligned with existing RuleEvidence conventions
5. Fixture matrix includes all day-stem groups and representative rollover cases

**Plans**: 2 plans

- [x] `07-01-PLAN.md` - Implement hour-slot and seed-mapping core with typed contracts
- [x] `07-02-PLAN.md` - Add boundary fixtures, parity validators, and metadata assertions

### Phase 8: Sexagenary Cycle Parity and Validators

**Goal**: Deliver canonical 60-cycle conversion/progression utilities and full-table parity verification

**Depends on**: Phase 7

**Requirements**: SC-01, SC-02, SC-03, SC-04, SC-05, PAR-01

**Success Criteria** (what must be TRUE):
1. Cycle index to stem-branch conversion and inverse conversion are both implemented with 1-60 bounded contracts
2. Forward/backward progression remains correct across 10/12/60 rollover boundaries
3. Utility APIs are reusable by hour-pillar and Na Am pathways without duplicate logic
4. Full 60-entry parity validator confirms exact canonical table matching
5. Regression tests guard cycle index normalization and invalid-input handling

**Plans**: 2 plans

- [x] `08-01-PLAN.md` - Build canonical cycle conversion/progression helpers and invariants
- [ ] `08-02-PLAN.md` - Implement full-table parity validators and regression coverage

### Phase 9: Na Am API Surfaces and Contracts

**Goal**: Expose Na Am lookups via pair/index APIs with stable schema, errors, and milestone traceability

**Depends on**: Phase 8

**Requirements**: NAM-API-01, NAM-API-02, NAM-API-03, NAM-API-04, NAM-API-05, NAM-API-06, PAR-03, PAR-04

**Success Criteria** (what must be TRUE):
1. API supports Na Am lookup by stem-branch pair and by cycle index with consistent payload semantics
2. Responses include source_id and method metadata aligned to existing evidence conventions
3. Invalid pair/index inputs return explicit validation errors with deterministic formatting
4. Existing DayFortune consumers remain backward compatible with additive API changes
5. Contract tests assert schema stability and serialization consistency for both lookup modes

**Plans**: 2 plans

- [ ] `09-01-PLAN.md` - Add core->DTO Na Am API models and lookup handlers
- [ ] `09-02-PLAN.md` - Harden error contracts, schema tests, and requirement traceability sync

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 7. Hour Pillar Parity Core | 2/2 | Complete | 2026-03-03 |
| 8. Sexagenary Cycle Parity and Validators | 1/2 | In Progress | 2026-03-03 |
| 9. Na Am API Surfaces and Contracts | 0/2 | Planned | - |

## Dependencies

```
Phase 7 (Hour Pillar Parity Core)
  ↓
Phase 8 (Sexagenary Cycle Parity and Validators)
  ↓
Phase 9 (Na Am API Surfaces and Contracts)
```

## Milestone Context

This roadmap implements v1.4 Lunar Engine Table Parity milestone, continuing from v1.3 (Dai Van Core). Phase numbering starts at 7 to maintain continuity.

**Goal**: Reach deterministic table-level parity for hour pillar and 60-cycle calculations, then expose Na Am APIs with evidence-backed outputs.

**Target features**:
- Hour pillar (gio tru) parity against Vietnamese lunar engine tables
- Full sexagenary 60-cycle parity for stem-branch progression contracts
- Na Am API endpoints/types for direct pair and cycle-index lookup
- Validator and fixture evidence for parity claims

## Coverage Summary

- v1 requirements: 20 total
- Mapped to phases: 20/20 (100%)
- Phases: 3

| Phase | Requirements | Category |
|-------|--------------|----------|
| 7 | HP-01 through HP-05, PAR-02 | Hour Pillar Parity, Metadata and Verification |
| 8 | SC-01 through SC-05, PAR-01 | Sexagenary 60-Cycle Parity, Metadata and Verification |
| 9 | NAM-API-01 through NAM-API-06, PAR-03, PAR-04 | Na Am APIs, Metadata and Verification |

---
*Roadmap created: 2026-03-03*
*Last updated: 2026-03-03 after v1.4 milestone initialization*
