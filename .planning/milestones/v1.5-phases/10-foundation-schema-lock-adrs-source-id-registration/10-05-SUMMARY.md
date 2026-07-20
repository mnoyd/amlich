---
phase: 10-foundation-schema-lock-adrs-source-id-registration
plan: "05"
subsystem: planning
tags: [adr, milestones, decisions, cross-references, documentation]

# Dependency graph
requires:
  - phase: 10-03
    provides: ADR-0001 ritual schema v1 at .planning/adrs/0001-ritual-schema-v1.md
  - phase: 10-04
    provides: ADR-0002 and ADR-0003 at .planning/adrs/0002-* and 0003-*
provides:
  - "ADR Cross-References subsection in MILESTONES.md registering DEC-0023, DEC-0024, DEC-0025"
  - "Single entry point from milestone-level docs into .planning/adrs/ directory"
  - "Table registry pattern for future ADRs (DEC-0026+)"
affects:
  - phase-11
  - phase-12
  - phase-13
  - phase-14
  - phase-15

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "ADR registry as markdown TABLE in MILESTONES.md under ### ADR Cross-References"
    - "DEC-NNNN stable IDs for project-wide ADR reference (starting DEC-0023)"
    - "Relative links from .planning/MILESTONES.md to .planning/adrs/000X-*.md"

key-files:
  created: []
  modified:
    - .planning/MILESTONES.md

key-decisions:
  - "TABLE subsection appended after Key Decisions numbered list — keeps v1.0 history intact, gives ADR registry its own markdown shape"
  - "DEC-0023 is the next safe ID (DEC-0015, 0016, 0022 referenced in planning docs; DEC-0017-0021 unreferenced; 0023 is next safe)"

patterns-established:
  - "Future Phase 11+ ADRs will append rows to the ### ADR Cross-References table starting at DEC-0026"
  - "Relative link pattern: adrs/000X-name.md (relative to .planning/)"

requirements-completed:
  - FND-01
  - FND-02
  - FND-04
  - FND-05

# Metrics
duration: 3min
completed: 2026-05-26
---

# Phase 10 Plan 05: MILESTONES.md ADR Cross-References Summary

**Three v1.5 ADRs registered in MILESTONES.md as DEC-0023/0024/0025 with a new TABLE subsection, completing Phase 10 Foundation and closing the hard gate for Phases 11-15.**

## Performance

- **Duration:** ~3 min
- **Started:** 2026-05-26T15:01:30Z
- **Completed:** 2026-05-26T15:04:00Z
- **Tasks:** 1
- **Files modified:** 1

## Accomplishments

- Appended `### ADR Cross-References` subsection to `.planning/MILESTONES.md` at line 221, immediately after the existing `### Key Decisions` numbered list (items 1-6) and before `### Files Created/Modified`
- Registered DEC-0023 (ADR-0001 ritual schema v1), DEC-0024 (ADR-0002 Phi Tinh monthly anchor), DEC-0025 (ADR-0003 Nien Tu Bach polarity matrix) as table rows with relative links
- Preserved the existing 6-item Key Decisions numbered list unchanged — diff was a single localized 10-line insertion

## Task Commits

Each task was committed atomically:

1. **Task 1: Append ADR Cross-References subsection to MILESTONES.md** - sandbox denied `git add .planning/`; file written to disk, orchestrator to commit.

## Files Created/Modified

- `.planning/MILESTONES.md` — Added `### ADR Cross-References` subsection (lines 221-230) with DEC-0023, DEC-0024, DEC-0025 table rows linking to the three ADR files

## Insertion Details

- **Subsection heading:** Line 221 (`### ADR Cross-References`)
- **Table header:** Lines 225-226
- **DEC-0023 row:** Line 227 — links to `adrs/0001-ritual-schema-v1.md`
- **DEC-0024 row:** Line 228 — links to `adrs/0002-phi-tinh-monthly-anchor.md`
- **DEC-0025 row:** Line 229 — links to `adrs/0003-nien-tu-bach-polarity.md`
- **Blank line before `### Files Created/Modified`:** Line 230

## DEC-NNNN Assignments

| Decision ID | ADR File | Description |
|-------------|----------|-------------|
| DEC-0023 | ADR-0001 | Ritual JSON schema v1 locked — typed event_keys[], structured offerings[]/preparation_steps[], closed RitualVariantTag, #[serde(deny_unknown_fields)] |
| DEC-0024 | ADR-0002 | Phi Tinh monthly anchor uses solar-term boundaries per Tham Thi Huyen Khong Hoc, resolved by v1.1.2 Tiet Khi scanner |
| DEC-0025 | ADR-0003 | Nien Tu Bach direction rule is Tam Nguyen x year-polarity matrix; Thuong/Trung Nguyen MEDIUM confidence pending Phase 13 cross-check |

## Decisions Made

- TABLE subsection pattern chosen over narrative list for the new ADR registry — keeps v1.0 history intact (numbered list stays), gives Phase 10+ decisions a structured machine-readable shape per CONTEXT.md §specifics
- No reformatting of the existing Key Decisions list — DEC-NNNN ids are for ADR-style decisions only (Phase 10 onward); old items keep descriptive titles

## Deviations from Plan

None — plan executed exactly as written. MILESTONES.md diff is a single localized 10-line insertion.

## Issues Encountered

Sandbox denied `git add .planning/` during task commit. Files are written to disk; the orchestrator will commit them. This is documented behavior per the execution objective.

## Next Phase Readiness

- Phase 10 Foundation is now complete — all 5 plans executed (10-01 through 10-05)
- The hard gate for Phases 11-15 is satisfied: ADRs locked, source IDs registered, schema types defined
- Phases 11+12 (Van khan) and 13+14 (Phi Tinh) may execute concurrently; Phase 15 is the join point
- Future ADRs append rows to `### ADR Cross-References` starting at DEC-0026

## Self-Check: PASSED

- FOUND: `.planning/MILESTONES.md`
- FOUND: `.planning/phases/10-foundation-schema-lock-adrs-source-id-registration/10-05-SUMMARY.md`
- FOUND: `.planning/adrs/0001-ritual-schema-v1.md`
- FOUND: `.planning/adrs/0002-phi-tinh-monthly-anchor.md`
- FOUND: `.planning/adrs/0003-nien-tu-bach-polarity.md`
- PASS: `### ADR Cross-References` heading present (1 occurrence)
- PASS: DEC-0023, DEC-0024, DEC-0025 all present (3 occurrences)
- PASS: Item 6 `**Metadata correction**` intact

---
*Phase: 10-foundation-schema-lock-adrs-source-id-registration*
*Completed: 2026-05-26*
