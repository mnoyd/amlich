# Requirements: Amlich v1.4 - Lunar Engine Table Parity

**Defined:** 2026-03-03
**Core Value:** Every almanac subsystem in amlich must produce output matching KHCBPPT for 2020-2030 with test-backed, traceable evidence.

## v1 Requirements

Requirements for Lunar engine table parity milestone. Each maps to roadmap phases.

### Hour Pillar Parity

- [x] **HP-01**: System can compute hour pillar (Can Chi) from day stem and local hour using deterministic branch-slot mapping
- [x] **HP-02**: System applies correct stem-seed rule from day stem group (Giáp/Kỷ, Ất/Canh, Bính/Tân, Đinh/Nhâm, Mậu/Quý)
- [x] **HP-03**: System handles edge boundaries for all 12 two-hour windows without overlap or gap
- [x] **HP-04**: Hour pillar output includes evidence metadata (source_id, method, profile)
- [x] **HP-05**: Hour pillar fixtures cover all day-stem groups and boundary times

### Sexagenary 60-Cycle Parity

- [x] **SC-01**: System can convert cycle index (1-60) to canonical stem-branch pair
- [x] **SC-02**: System can convert stem-branch pair to cycle index (1-60)
- [x] **SC-03**: Forward/backward progression preserves modular correctness across rollover boundaries (10/12/60)
- [x] **SC-04**: Cycle utilities expose deterministic helpers reusable by hour pillar and Na Am APIs
- [x] **SC-05**: Validation suite confirms full-table parity against canonical 60-cycle references

### Na Am APIs

- [x] **NAM-API-01**: API exposes Na Am lookup by stem-branch pair
- [x] **NAM-API-02**: API exposes Na Am lookup by cycle index (1-60)
- [x] **NAM-API-03**: API returns normalized source metadata and method for each Na Am response
- [x] **NAM-API-04**: API conversion layer preserves backward compatibility for existing DayFortune consumers
- [x] **NAM-API-05**: API returns explicit validation error for invalid pair/index requests
- [x] **NAM-API-06**: Contract tests verify response schema and stable serialization for both lookup modes

### Metadata and Verification

- [x] **PAR-01**: Parity validators are added for hour pillar and full 60-cycle tables
- [x] **PAR-02**: Golden fixtures include representative plus boundary cases for hour pillar and Na Am lookups
- [x] **PAR-03**: Traceability links every new requirement to one roadmap phase
- [x] **PAR-04**: Milestone artifacts document parity decisions and known source ambiguities

## v2 Requirements

Deferred to future release. Tracked but not in current roadmap.

### Extended Fortune APIs

- **FUT-01**: Unified birth-context endpoint combining Dai Van, hour pillar, and Ten Gods interpretation payloads
- **FUT-02**: Human-language interpretation templates backed by deterministic feature flags
- **FUT-03**: Batch parity verification endpoint for external dataset ingestion

## Out of Scope

Explicitly excluded. Documented to prevent scope creep.

| Feature | Reason |
|---------|--------|
| Human-language interpretation text generation | Non-deterministic and outside correctness-first parity objective |
| New scoring/fortune ranking engine | Introduces pseudo-precision and source ambiguity |
| Multi-timezone historical DST normalization | Out of scope for this parity milestone; keep deterministic local-hour contract |
| New public endpoint family beyond Na Am lookup surfaces | Avoids API sprawl while parity work is stabilized |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| HP-01 | Phase 7 | Complete |
| HP-02 | Phase 7 | Complete |
| HP-03 | Phase 7 | Complete |
| HP-04 | Phase 7 | Complete |
| HP-05 | Phase 7 | Complete |
| SC-01 | Phase 8 | Complete |
| SC-02 | Phase 8 | Complete |
| SC-03 | Phase 8 | Complete |
| SC-04 | Phase 8 | Complete |
| SC-05 | Phase 8 | Planned |
| NAM-API-01 | Phase 9 | Complete |
| NAM-API-02 | Phase 9 | Complete |
| NAM-API-03 | Phase 9 | Complete |
| NAM-API-04 | Phase 9 | Complete |
| NAM-API-05 | Phase 9 | Complete |
| NAM-API-06 | Phase 9 | Complete |
| PAR-01 | Phase 8 | Complete |
| PAR-02 | Phase 7 | Complete |
| PAR-03 | Phase 9 | Complete |
| PAR-04 | Phase 9 | Complete |

**Coverage:**
- v1 requirements: 20 total
- Mapped to phases: 20/20 ✓

---
*Requirements defined: 2026-03-03*
*Last updated: 2026-03-03 after Phase 8 Plan 1 execution*
