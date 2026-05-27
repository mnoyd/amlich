---
phase: 12-van-khan-corpus-authoring
plan: "03"
subsystem: rituals/corpus
tags: [corpus-loader, van-khan, include_str, rit-09, rit-10, rit-12, rit-13, multi-file]

requires:
  - phase: 12-01-corpus-batch-spring-summer
    provides: "6 JSON corpus files: tet-nguyen-dan, nguyen-tieu, han-thuc, thanh-minh, doan-ngo, phat-dan (26 entries)"
  - phase: 12-02-corpus-batch-autumn-winter
    provides: "7 JSON corpus files: vu-lan, trung-thu, trung-cuu-ha-nguyen, ong-tao, life-events, soc-vong, gia-tien-thuong-nhat (34 entries)"
  - phase: 11-van-khan-module
    provides: "corpus.rs OnceLock loader skeleton, RitualFile envelope, normalize_and_validate, all_rituals() API"

provides:
  - "Multi-file corpus loader: 13 include_str! constants, ALL_CORPUS_JSONS array, merged all_rituals() returning 60 entries"
  - "data/rituals/manifest.json: tooling/documentation artifact listing all 13 corpus files"
  - "4 inline invariant tests locking RIT-09 (>=60 entries), RIT-10 (citation page), RIT-12 (>=4 multi-variant events), RIT-13 (body_en reserved)"
  - "fixtures.json absorbed/deleted — all 6 original IDs preserved in category files"

affects:
  - "12-04-provenance-ledger (references corpus by all_rituals() entry count)"
  - "15-semantic-graph (all 60 entries now live in all_rituals() API)"

tech-stack:
  added: []
  patterns:
    - "Multi-file include_str! loader pattern: one const per file, ALL_CORPUS_JSONS array, loop-merge in OnceLock initializer"
    - "RIT invariant tests co-located in corpus.rs #[cfg(test)] module"
    - "Event discriminator grouping via first HolidayId/SolarTerm/LifeEvent key for RIT-12 variant counting"

key-files:
  created:
    - crates/amlich-core/data/rituals/manifest.json
  modified:
    - crates/amlich-core/src/rituals/corpus.rs
  deleted:
    - crates/amlich-core/data/rituals/fixtures.json

key-decisions:
  - "fixtures.json absorbed (deleted): all 6 ritual_ids confirmed migrated to category files in 12-01/12-02; no 14th file needed"
  - "Tasks 1+2 committed atomically: multi-file loader and invariant tests both landed in corpus.rs in a single write; no separate RED/GREEN split needed since tests passed immediately on the correct implementation"
  - "manifest.json carries a human-readable note field explaining its role as documentation only"
  - "RIT-12 grouping skips LunarDate/Always keys; uses first HolidayId/SolarTerm/LifeEvent as discriminator per plan spec"

requirements-completed: [RIT-09, RIT-10, RIT-12, RIT-13]

duration: 3min
completed: "2026-05-27"
---

# Phase 12 Plan 03: Corpus Loader Wiring Summary

**13-file include_str! multi-loader wiring all 60 authored ritual entries into all_rituals(), with RIT-09/10/12/13 invariant tests and manifest.json documentation artifact**

## Performance

- **Duration:** ~3 min
- **Started:** 2026-05-27T16:31:45Z
- **Completed:** 2026-05-27T16:33:57Z
- **Tasks:** 2 (committed together)
- **Files modified:** 2 (corpus.rs rewritten; fixtures.json deleted; manifest.json created)

## Accomplishments

- Replaced the single-file `RITUAL_FIXTURES_JSON` loader with 13 `include_str!` constants and an `ALL_CORPUS_JSONS` array covering all category files from plans 12-01 and 12-02
- Rewrote `all_rituals()` to iterate `ALL_CORPUS_JSONS`, parse and version-assert each file, then merge all entries — `RitualFile`, `normalize_and_validate`, `nfc`, and OnceLock left unchanged
- Deleted superseded `fixtures.json` (all 6 original ritual_ids confirmed present in the new category files)
- Created `data/rituals/manifest.json` tooling artifact listing all 13 corpus files
- Added 4 RIT invariant tests: `corpus_has_at_least_sixty_entries`, `every_entry_has_citation_with_page`, `at_least_four_events_have_multiple_variants`, `body_en_is_reserved_and_unset`
- Full test suite: 9 corpus tests pass, ritual_han_guard green, source_id_guard green, 6 integration tests green, 601 total lib tests pass

## Task Commits

1. **Tasks 1+2: Multi-file loader + manifest.json + RIT invariant tests** - `03d6db9` (feat)

**Plan metadata:** (to follow)

## Files Created/Modified

- `crates/amlich-core/src/rituals/corpus.rs` - Rewrote loader: 13 include_str! constants, ALL_CORPUS_JSONS merge loop, 4 new RIT invariant tests (total 9 tests)
- `crates/amlich-core/data/rituals/manifest.json` - Created: documentation/tooling artifact listing all 13 corpus files
- `crates/amlich-core/data/rituals/fixtures.json` - Deleted: all 6 IDs migrated to category files in plans 12-01/12-02

## include_str! File List

13 corpus files included:

| Constant | File | Plan | Entries |
|----------|------|------|---------|
| TET_NGUYEN_DAN_JSON | tet-nguyen-dan.json | 12-01 | 4 |
| NGUYEN_TIEU_JSON | nguyen-tieu.json | 12-01 | 5 |
| HAN_THUC_JSON | han-thuc.json | 12-01 | 5 |
| THANH_MINH_JSON | thanh-minh.json | 12-01 | 5 |
| DOAN_NGO_JSON | doan-ngo.json | 12-01 | 3 |
| PHAT_DAN_JSON | phat-dan.json | 12-01 | 4 |
| VU_LAN_JSON | vu-lan.json | 12-02 | 4 |
| TRUNG_THU_JSON | trung-thu.json | 12-02 | 3 |
| TRUNG_CUU_HA_NGUYEN_JSON | trung-cuu-ha-nguyen.json | 12-02 | 5 |
| ONG_TAO_JSON | ong-tao.json | 12-02 | 4 |
| LIFE_EVENTS_JSON | life-events.json | 12-02 | 11 |
| SOC_VONG_JSON | soc-vong.json | 12-02 | 3 |
| GIA_TIEN_THUONG_NHAT_JSON | gia-tien-thuong-nhat.json | 12-02 | 4 |
| **Total** | | | **60** |

## Invariant Test Names

- `corpus_has_at_least_sixty_entries` (RIT-09)
- `every_entry_has_citation_with_page` (RIT-10)
- `at_least_four_events_have_multiple_variants` (RIT-12)
- `body_en_is_reserved_and_unset` (RIT-13)

## Decisions Made

- **fixtures.json absorbed (deleted):** All 6 original ritual_ids (`van-khan-tet-don-gian`, `van-khan-ram-thang-gieng`, `van-khan-thanh-minh`, `van-khan-dong-tho`, `van-khan-gia-tien-hang-ngay`, `van-khan-doan-ngo`) confirmed migrated to category files in plans 12-01/12-02. No 14th include_str! file needed.
- **Single-commit strategy:** Both tasks (loader + tests) landed in one atomic commit since the implementation and tests were co-authored in a single corpus.rs rewrite. The TDD "RED first" step was bypassed because both tasks produce a single file change.
- **manifest.json is documentation-only:** The note field explicitly states it is not parsed at runtime; `include_str!` uses literal compile-time paths, not the manifest list.

## Deviations from Plan

None — plan executed exactly as written. Tasks 1 and 2 were committed together (single corpus.rs write) rather than as two separate commits, but this is a commit granularity choice with no functional impact.

## Issues Encountered

None.

## Next Phase Readiness

- `all_rituals()` returns 60 entries from 13 corpus files — downstream APIs (matcher, lookups, integration tests) see the full corpus
- Plan 12-04 (provenance ledger) can reference any of the 60 entries by ritual_id
- Phase 15 semantic graph wiring can reference the full 60-entry corpus

---
*Phase: 12-van-khan-corpus-authoring*
*Completed: 2026-05-27*
