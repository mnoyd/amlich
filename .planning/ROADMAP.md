# ROADMAP: Amlich v1.3 - Dai Van Core

**Created:** 2026-03-03
**Milestone:** v1.3 Dai Van Core
**Depth:** Quick
**Status:** In Progress

## Phases

- [ ] **Phase 4: Core Dai Van Module** - Core calculation engine with 8-pillar generation, Chieuthu direction, and metadata traceability
- [ ] **Phase 5: Ten Gods Integration and Helpers** - Ten Gods correlation per pillar and helper functions for age-based queries
- [ ] **Phase 6: Kua Analysis** - Kua-based directional analysis per pillar with favorable/unfavorable directions

## Phase Details

### Phase 4: Core Dai Van Module

**Goal**: Deliver deterministic Dai Van calculation engine with period transitions, Chieuthu direction, and evidence metadata

**Depends on**: Nothing (first phase of v1.3)

**Requirements**: DV-CALC-01, DV-CALC-02, DV-CALC-03, DV-CALC-04, DV-CALC-05, DV-CALC-06, DV-META-01, DV-META-02, DV-META-03, DV-META-04

**Success Criteria** (what must be TRUE):
1. System can generate 8 Dai Van pillars with contiguous 10-year age ranges from birth date and gender
2. Chieuthu direction (Thuận/Nghịch) is correctly determined from year polarity × gender
3. Start age is accurately calculated from Tiết Khí distance using 3 days = 1 year conversion
4. All calculation results include convention metadata (year_basis, start_age_method, gender_encoding) and evidence metadata (source_id, method)
5. Edge cases (Tiết Khí boundaries, leap months, year polarity transitions) are handled correctly

**Plans**: 2 plans

- [x] `04-01-PLAN.md` - Define Dai Van core types, direction matrix, start-age conversion, and metadata contracts
- [x] `04-02-PLAN.md` - Implement 8-pillar generation flow, helper lookups, and edge-case coverage

### Phase 5: Ten Gods Integration and Helpers

**Goal**: Integrate Ten Gods correlation with Dai Van pillars and provide helper functions for age-based queries

**Depends on**: Phase 4

**Requirements**: DV-TG-01, DV-TG-02, DV-TG-03, DV-HELP-01, DV-HELP-02, DV-HELP-03, DV-HELP-04

**Success Criteria** (what must be TRUE):
1. Each pillar's Heavenly Stem can be correlated with birth day stem via Thap Than (lazy/on-demand)
2. Users can find the current pillar for any given age using helper functions
3. System can calculate years until next transition between pillars
4. Helper functions gracefully handle out-of-range ages using Option returns
5. Unknown birth hour is supported (Ten Gods = None or day_fortune-based targets)

**Plans**: 2 plans

- [ ] `05-01-PLAN.md` - Add lazy Ten Gods-per-pillar helpers in Dai Van with unknown-birth-hour Option handling
- [x] `05-02-PLAN.md` - Lock and harden helper query contracts for age lookup and transition boundaries

### Phase 6: Kua Analysis

**Goal**: Deliver Kua-based directional analysis per pillar with favorable/unfavorable directions

**Depends on**: Phase 4

**Requirements**: DV-KUA-01, DV-KUA-02, DV-KUA-03, DV-KUA-04

**Success Criteria** (what must be TRUE):
1. Each pillar's elements can be analyzed against birth Kua directions
2. Kua analysis provides favorable and unfavorable directions per pillar
3. Birth Kua is calculated once per person and reused for all pillars
4. Kua 5 resolution follows project convention (male→8, female→2)

**Plans**: TBD

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 4. Core Dai Van Module | 2/2 | Completed | 2026-03-03 |
| 5. Ten Gods Integration and Helpers | 1/2 | In Progress | - |
| 6. Kua Analysis | 0/0 | Not started | - |

## Dependencies

```
Phase 4 (Core Dai Van Module)
  ↓
Phase 5 (Ten Gods Integration and Helpers)
  ↓
Phase 6 (Kua Analysis)
```

## Milestone Context

This roadmap implements v1.3 Dai Van Core milestone, continuing from v1.2 (Ten Gods and Kua Foundation) which completed 3 phases. Phase numbering starts at 4 to maintain continuity.

**Goal**: Implement core Dai Van computation with period transitions, Ten Gods correlation, and Kua integration.

**Target features**:
- Dai Van period transitions (9 cycles of 10 years each)
- Ten Gods correlation for Dai Van periods
- Kua-based fortune direction mapping
- Deterministic computation with evidence metadata

## Coverage Summary

- v1 requirements: 20 total
- Mapped to phases: 20/20 (100%)
- Phases: 3

| Phase | Requirements | Category |
|-------|--------------|----------|
| 4 | DV-CALC-01 through DV-CALC-06, DV-META-01 through DV-META-04 | Core Calculation, Metadata & Traceability |
| 5 | DV-TG-01 through DV-TG-03, DV-HELP-01 through DV-HELP-04 | Ten Gods Integration, Helper Functions |
| 6 | DV-KUA-01 through DV-KUA-04 | Kua Analysis |

---
*Roadmap created: 2026-03-03*
