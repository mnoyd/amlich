---
phase: 12-van-khan-corpus-authoring
plan: "04"
subsystem: corpus-documentation
tags: [van-khan, provenance, rituals, documentation, audit-ledger]

# Dependency graph
requires:
  - phase: 12-01
    provides: "26 ritual entries across 6 spring/summer JSON files"
  - phase: 12-02
    provides: "34 ritual entries across 7 autumn/winter + life-event + daily JSON files"
provides:
  - "RIT-11: provenance_audit.md ledger covering all 60 unique ritual_ids"
  - "Classical reference + page + confidence + reviewer (pending) per corpus entry"
  - "4 classical works enumerated with publisher and confidence tier definitions"
affects: [phase-12-03, phase-15]

# Tech tracking
tech-stack:
  added: []
  patterns: ["Provenance audit ledger: grouped by event category, one row per ritual_id, pending reviewer deferred"]

key-files:
  created:
    - "crates/amlich-core/data/rituals/provenance_audit.md"
  modified: []

key-decisions:
  - "Fixtures.json duplicates excluded from ledger: 6 ritual_ids appear in both fixtures.json and a canonical category file; ledger uses canonical file as authoritative source, yielding 60 unique rows from 66 total corpus entries"
  - "Reviewer field set to pending for all entries per research Q4: peer review deferred post-v1.5; citation coordinates provided to enable future independent review"
  - "Pure Quoc-ngu: all classical references rendered without Han characters per project convention, using ASCII where the JSON source had ASCII (Van Khan Co Truyen Viet Nam), Quoc-ngu where JSON had diacritics"

patterns-established:
  - "Provenance ledger pattern: grouped by category sub-heading for readability; one table per source file; References section enumerates all classical works with confidence tier definitions"

requirements-completed: [RIT-11]

# Metrics
duration: 5min
completed: 2026-05-27
---

# Phase 12 Plan 04: Van Khan Provenance Audit Ledger Summary

**60-entry provenance ledger for the Van Khan corpus covering all ritual_ids from 13 event categories, with classical reference, page, confidence tier, and pending reviewer field per entry**

## Performance

- **Duration:** ~5 min
- **Started:** 2026-05-27T16:30:00Z
- **Completed:** 2026-05-27T16:35:00Z
- **Tasks:** 1
- **Files modified:** 1

## Accomplishments

- Parsed all 14 corpus JSON files programmatically to derive the authoritative 60 unique ritual_id set
- Identified 6 duplicate ritual_ids shared between fixtures.json and canonical category files; excluded fixtures.json duplicates from ledger rows to produce clean 1:1 coverage
- Created `provenance_audit.md` (209 lines): header with date and entry count, 13 category sub-sections each with Audit Ledger table, References section with 4 classical works
- Automated verification confirms all 60 unique corpus ritual_ids appear in the ledger (no orphans, no extras)

## Task Commits

Each task was committed atomically:

1. **Task 1: Write provenance_audit.md ledger for every corpus entry** — `ff4464f` (docs)

**Plan metadata:** (pending docs commit)

## Files Created/Modified

- `crates/amlich-core/data/rituals/provenance_audit.md` — 60-entry provenance audit ledger grouped by 13 event categories; columns: ritual_id, classical_reference, page, confidence, reviewer

## Decisions Made

- **Fixtures.json duplicate exclusion:** 6 ritual_ids appear in both `fixtures.json` (Phase 11 stub data) and their canonical category files. The ledger uses the canonical file as authoritative source, yielding 60 unique rows (not 66). The automated verification check uses the full 60-ID unique set so no ritual_id is uncovered.
- **Reviewer field set to pending:** Per research Q4, independent peer review is deferred to a post-v1.5 editorial pass. The ledger field exists to enable future review via exact citation coordinates.
- **ASCII titles preserved where JSON used ASCII:** `gia-tien-thuong-nhat.json`, `life-events.json`, `ong-tao.json`, `soc-vong.json`, and `trung-cuu-ha-nguyen.json` stored the citation title as ASCII `Van Khan Co Truyen Viet Nam`. The ledger matches each file's data faithfully; the References section uses the full Quoc-ngu title as the canonical form.

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- RIT-11 satisfied: provenance ledger complete with 1:1 ritual_id coverage
- Phase 12 plans remaining: 12-03 (loader wiring) is the only outstanding wave-2 plan
- Phase 12 fully completes when 12-03 lands (loader wiring for all 13 category JSON files)

---
*Phase: 12-van-khan-corpus-authoring*
*Completed: 2026-05-27*
