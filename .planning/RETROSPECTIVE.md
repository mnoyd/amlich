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

## Milestone: v1.3 - Dai Van Core

**Shipped:** 2026-03-03
**Phases:** 3 completed / 3 planned | **Plans:** 5 completed | **Sessions:** 1

### What Was Built
- Delivered the core Dai Van engine with deterministic direction and start-age primitives plus auditable convention/evidence metadata.
- Implemented end-to-end Dai Van generation and helper APIs for age lookup, current pillar, and transition countdown semantics.
- Added lazy Ten Gods adapters for Dai Van pillars/ages with explicit Option handling for unknown birth-day stem and out-of-range paths.
- Implemented Kua-based per-pillar directional analysis with birth-Kua single-computation reuse and age/index lookup helpers.

### What Worked
- Plan-level decomposition (04-01, 04-02, 05-01, 05-02) kept implementation scope tight and verification fast.
- Contract-focused tests (half-open ranges, transition boundaries, lazy behavior) prevented silent semantic drift.
- Reusing existing helper boundary semantics for Kua lookup-by-age kept Phase 6 integration low-risk and deterministic.

### What Was Inefficient
- Strict float equality assumptions initially caused avoidable test churn before epsilon-based assertions were adopted.
- A patch-merge context mismatch temporarily removed helper functions and required blocking recovery during plan execution.
- Direction-order assumptions in Kua intersection tests needed one alignment pass to match deterministic source ordering.

### Patterns Established
- Keep Ten Gods out of base Dai Van payloads and expose correlation via lazy helper APIs only.
- Use deterministic in-memory DaiVanResult fixtures to lock helper contracts independent of upstream conversion variability.
- Compute birth Kua once per analysis and derive per-pillar direction results through pure intersections.

### Key Lessons
1. Deterministic domain contracts are easier to extend when core payloads remain minimal and optional features are derived lazily.
2. Boundary semantics (`[start_age, end_age)`) need dedicated contract tests, not just integration-path coverage.
3. For directional analysis, preserving canonical ordering from source sets reduces flaky expectation drift in tests.

### Cost Observations
- Model mix: not measured in this repo snapshot
- Sessions: 1
- Notable: execution remained efficient; rework was limited to correctness guardrails (float precision, patch-context recovery, direction-order alignment)

---

## Cross-Milestone Trends

### Process Evolution

| Milestone | Sessions | Phases | Key Change |
|-----------|----------|--------|------------|
| v1.0 | 4 | 4 | Built KHCBPPT baseline and validator harness workflow |
| v1.1 | 1 | 3 | Added verification-led governance closure as a first-class phase pattern |
| v1.3 | 1 | 3/3 completed | Introduced deterministic Dai Van core contracts, lazy Ten Gods helpers, and per-pillar Kua direction analysis |

### Cumulative Quality

| Milestone | Tests | Coverage | Zero-Dep Additions |
|-----------|-------|----------|-------------------|
| v1.0 | 184 passing | Core KHCBPPT validator coverage established | 0 |
| v1.1 | 188 passing | Extended subsystem and regression acceptance coverage | 0 |
| v1.3 | 242 passing (`amlich-core --lib`) | Dai Van core + helper contracts + Kua per-pillar analysis verified | 0 |

### Top Lessons (Verified Across Milestones)

1. Verification artifacts and executable tests must remain synchronized to avoid status drift.
2. Small focused phases with explicit acceptance criteria improve recovery when regressions appear.
3. Deterministic helper contracts should be asserted with fixture-based boundary tests to prevent edge-case regressions.
4. Reuse-first integration (new feature over existing deterministic helpers) lowers scope risk while keeping behavior auditable.
