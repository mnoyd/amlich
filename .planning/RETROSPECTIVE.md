# Project Retrospective

*A living document updated after each milestone. Lessons feed forward into future planning.*

## Milestone: v1.1 - Foundation Extensions

**Shipped:** 2026-03-02
**Phases:** 3 | **Plans:** 9 | **Sessions:** 1

### What Was Built
- Added complete extended Xung Hop relationship coverage and integrated new fields into day-level outputs.
- Added Tang Can hidden-stem data model, lookup logic, baseline data, and serialization-ready DayFortune integration.
- Fixed Tiet Khi nearest-term regression and restored full-package acceptance gate with updated governance artifacts.

### What Worked
- Verification-first status reconciliation prevented roadmap/state drift after implementation work.
- Gap-closure phases (v1.1.1, v1.1.2) isolated documentation/governance debt from code fix execution.

### What Was Inefficient
- Initial Tiet Khi implementation used synthetic approximation and required a follow-up correction cycle.
- Milestone completion metadata needed multiple post-cycle passes before reaching stable machine-readable consistency.

### Patterns Established
- Treat canonical verification artifacts as status authority, then propagate to roadmap/state/requirements.
- Use scoped requirement IDs when identifiers overlap across milestone families.

### Key Lessons
1. Acceptance gates should validate both implementation behavior and governance artifacts in the same execution cycle.
2. Milestone-specific requirement files reduce ambiguity and keep master registries lightweight.

### Cost Observations
- Model mix: not measured in this repo snapshot
- Sessions: 1
- Notable: most effort in this milestone came from reconciliation and acceptance hardening, not net-new feature volume

---

## Cross-Milestone Trends

### Process Evolution

| Milestone | Sessions | Phases | Key Change |
|-----------|----------|--------|------------|
| v1.0 | 4 | 4 | Built KHCBPPT baseline and validator harness workflow |
| v1.1 | 1 | 3 | Added verification-led governance closure as a first-class phase pattern |

### Cumulative Quality

| Milestone | Tests | Coverage | Zero-Dep Additions |
|-----------|-------|----------|-------------------|
| v1.0 | 184 passing | Core KHCBPPT validator coverage established | 0 |
| v1.1 | 188 passing | Extended subsystem and regression acceptance coverage | 0 |

### Top Lessons (Verified Across Milestones)

1. Verification artifacts and executable tests must remain synchronized to avoid status drift.
2. Small focused phases with explicit acceptance criteria improve recovery when regressions appear.
