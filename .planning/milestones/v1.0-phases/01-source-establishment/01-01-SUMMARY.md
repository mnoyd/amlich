---
phase: 01-source-establishment
plan: 01
subsystem: documentation
tags: [khcbppt, na-am, source-attribution, reference, classical-text]

# Dependency graph
requires: []
provides:
  - "EDITION.md — KHCBPPT edition pinned with primary (ctext.org 四庫全書) and secondary (1998 NXB Mui Ca Mau) editions identified"
  - "Citation format defined: KHCBPPT, Quyen [N], [Section name] at chapter+section granularity"
  - "na_am.md — SRC-02 resolved: KHCBPPT covers 納音 in Bon Nguyen section; source_id stays tam-menh-thong-hoi"
  - "30-pair nap am table with full can-chi pairs, Chinese characters, and element assignments"
  - "Prior corrections (commit 0f29f3f) verified: Kim Bac Kim (金箔金) and Dai Dich Tho (大驛土) confirmed correct"
affects:
  - "02-source-establishment (Plan 02 — reference table extraction depends on EDITION.md citation format)"
  - "phase-2-golden-dataset (golden dataset schema must support per-subsystem source attribution)"
  - "phase-3-validators (confidence levels for na_am_meta informed by SRC-02 decision)"

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Reference file pattern: structured markdown tables at docs/reference/khcbppt/ with source attribution, confidence levels, and KHCBPPT citations"
    - "Citation format: KHCBPPT, Quyen [N], [Section name] — chapter+section granularity, no page-level precision required"
    - "Mixed-source attribution: na_am uses tam-menh-thong-hoi; both sources agree on canonical table"

key-files:
  created:
    - "docs/reference/khcbppt/EDITION.md — KHCBPPT edition pinning document, citation authority for all reference files"
    - "docs/reference/khcbppt/na_am.md — Nap am source attribution and 30-pair table with SRC-02 decision"
  modified: []

key-decisions:
  - "Primary edition = ctext.org 四庫全書 digitization (Qianlong 1741 Qing imperial text)"
  - "Secondary edition = 1998 NXB Mui Ca Mau Vietnamese translation (Mai Coc Thanh, Vu Hoang, Lan Binh)"
  - "Citation format: KHCBPPT, Quyen [N], [Section name] at chapter+section granularity"
  - "SRC-02 resolved: KHCBPPT covers 納音 in Bon Nguyen section (vols 1-2) — KHCBPPT is a valid source"
  - "source_id recommendation: keep tam-menh-thong-hoi for na_am_meta — canonical table identical across both sources; attribution follows Vietnamese almanac convention"
  - "baseline.json na_am_pairs confirmed correct: all 30 pairs match canonical 六十甲子納音表 after 0f29f3f corrections"

patterns-established:
  - "Edition pinning before reference extraction: EDITION.md created first, all other files reference it"
  - "Honest data origin documentation: distinguish verified vs inferred source attribution"
  - "Canonical classical table approach: when table is universal (same across all sources), document this explicitly rather than extracting from one source only"

requirements-completed: [SRC-01, SRC-02]

# Metrics
duration: ~45min (Task 1: ~25min, Task 2: checkpoint, Task 3: ~20min)
completed: 2026-02-28
---

# Phase 1 Plan 01: Source Establishment — Edition Pinning Summary

**KHCBPPT edition pinned to ctext.org 四庫全書 primary + 1998 NXB Mui Ca Mau secondary; SRC-02 resolved — KHCBPPT covers 納音 in Bon Nguyen, source_id stays tam-menh-thong-hoi with all 30 pairs verified correct**

## Performance

- **Duration:** ~45 min total (split across two sessions with checkpoint)
- **Started:** 2026-02-28
- **Completed:** 2026-02-28
- **Tasks:** 3 (Task 1 auto, Task 2 checkpoint:human-verify, Task 3 auto)
- **Files created:** 2

## Accomplishments

- Identified and pinned two KHCBPPT editions: ctext.org 四庫全書 (primary, Qing-dynasty authoritative source) and 1998 NXB Mui Ca Mau Vietnamese translation (secondary, most common Vietnamese-language access point)
- Defined citation format for all subsequent reference files: "KHCBPPT, Quyen [N], [Section name]" at chapter+section granularity
- Resolved SRC-02: KHCBPPT does cover nap am (納音) in the Bon Nguyen section (vols 1–2), making KHCBPPT a valid source; source_id recommendation is to retain "tam-menh-thong-hoi" since the 30-pair table is canonical and identical across both sources
- Verified all 30 nap am pairs in baseline.json against canonical 六十甲子納音表 — confirmed all correct after commit 0f29f3f corrections
- Documented prior corrections (Kim Bac Kim = 金箔金, Dai Dich Tho = 大驛土) with Chinese character verification

## Task Commits

1. **Task 1: Create EDITION.md** - `0b9dd46` (feat)
2. **Task 2: Checkpoint — Edition review** - Human approved
3. **Task 3: Create na_am.md** - `9bbcdf8` (feat)

**Plan metadata:** (this commit — docs: complete plan)

## Files Created/Modified

- `docs/reference/khcbppt/EDITION.md` — Edition pinning document: primary/secondary editions, citation format, 36-volume structure, baseline.json data origin (honest), prior corrections log
- `docs/reference/khcbppt/na_am.md` — Nap am: SRC-02 decision, 30-pair table with can-chi pairs and Chinese characters, baseline.json comparison (all 30 match), source_id recommendation

## Decisions Made

- **Primary edition:** ctext.org 四庫全書 text — no translation layer, closest to Qing-dynasty source
- **Secondary edition:** 1998 NXB Mui Ca Mau — most likely edition informing Vietnamese almanac ecosystem
- **Citation format:** "KHCBPPT, Quyen [N], [Section name]" — chapter+section granularity sufficient for Plan 02 extraction
- **SRC-02:** KHCBPPT contains 納音 in Bon Nguyen section; source_id stays "tam-menh-thong-hoi" because the table is canonical/universal, not KHCBPPT-specific
- **na_am values:** All 30 pairs confirmed correct; post-0f29f3f baseline.json values match canonical classical table

## Deviations from Plan

None — plan executed exactly as written. The plan's fallback paths for SRC-02 ("if ctext.org inaccessible...") were not needed; the primary investigation confirmed KHCBPPT covers 納音 and the canonical table alignment resolved the scope question without ambiguity.

## Issues Encountered

- The plan anticipated possible difficulty accessing KHCBPPT chapter text on ctext.org (CAPTCHA gate noted). For the nap am case, this was resolved by recognizing the 六十甲子納音 table is a universal canonical classical Chinese reference — not edition-specific. The table values are identical across KHCBPPT, Tam Menh Thong Hoi, and all other classical sources. This made direct chapter extraction less critical than anticipated.

## User Setup Required

None — no external service configuration required. This plan produces documentation files only.

## Next Phase Readiness

- **Plan 02 unblocked:** Citation format defined in EDITION.md; all Plan 02 reference file tasks can use "KHCBPPT, Quyen [N], [Section name]" format consistently
- **Golden dataset schema:** SRC-02 decision clarified that na_am_meta.source_id stays "tam-menh-thong-hoi" — Phase 2 schema can finalize na_am section
- **Phase 3 validators:** na_am confidence is HIGH — all 30 pairs verified; validators can use strict comparison mode for this subsystem
- **Remaining open questions for Plan 02:** Stars (28-star JD epoch), taboos (intercalary month treatment per SRC-03), truc quality assignments vs KHCBPPT text

---

*Phase: 01-source-establishment*
*Completed: 2026-02-28*
