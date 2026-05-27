---
phase: 12-van-khan-corpus-authoring
verified: 2026-05-27T17:00:00Z
status: passed
score: 5/5 must-haves verified
re_verification: false
---

# Phase 12: Van Khan Corpus Authoring Verification Report

**Phase Goal:** User can find at least 60 traceable, peer-reviewed ritual entries shipped under `data/rituals/` with full citation discipline and variant coverage for at least 4 events.
**Verified:** 2026-05-27
**Status:** PASSED
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| #  | Truth | Status | Evidence |
|----|-------|--------|----------|
| 1 | A reader can find >= 60 entries under `data/rituals/` spread across <= 14 per-event-category files plus `manifest.json` | VERIFIED | 13 category files + manifest.json = 14 JSON files total; Python count confirms exactly 60 entries |
| 2 | A reader can open any entry and find `source_id: "vn-folk-ritual"`, an `original_citation` (book + page), and a `confidence` tier of `primary` / `regional-variant` / `synthesized` | VERIFIED | Python scan: zero entries with wrong source_id or missing page; all confidence tiers are valid enum values; `every_entry_has_citation_with_page` and `every_entry_has_correct_source_id` tests pass |
| 3 | A reviewer can find a `provenance_audit.md` ledger in `data/rituals/` recording the classical reference and reviewer for every entry | VERIFIED | File exists at 210 lines; Python 1:1 check: all 60 corpus ritual_ids present in ledger, no orphans; ledger has ritual_id/classical_reference/page/confidence/reviewer columns; References section lists 4 classical works |
| 4 | A caller can iterate `all_rituals()` and find >= 4 events with multiple variants sharing the same `event_type` discriminated by a `variant` field on `RitualEntry` | VERIFIED | Python grouping finds 17 events with >= 2 variants; `at_least_four_events_have_multiple_variants` test passes; confirmed groups include: tet-nguyen-dan (4), vu-lan (4), tet-nguyen-tieu (5), tet-han-thuc (5), tet-doan-ngo (3), nhap_trach (2), ong-tao (2), solar:Thanh Minh (5), and 9 more |
| 5 | A code reader can find a reserved `body_en: Option<String>` field on `RitualEntry`, deserialized via `#[serde(default)]`, content authoring deferred | VERIFIED | schema.rs line 141-142: `#[serde(default, skip_serializing_if = "Option::is_none")]` / `pub body_en: Option<String>`; `body_en_is_reserved_and_unset` test passes — zero entries have body_en set |

**Score:** 5/5 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `crates/amlich-core/data/rituals/tet-nguyen-dan.json` | 4 Tet variants with identical event_keys | VERIFIED | 4 entries (simple/full/buddhist/folk), all share identical event_keys `[holiday_id:tet-nguyen-dan, lunar_date:1/1]` |
| `crates/amlich-core/data/rituals/nguyen-tieu.json` | >= 2 Nguyen Tieu entries | VERIFIED | 5 entries; `tet-nguyen-tieu` holiday_id present |
| `crates/amlich-core/data/rituals/han-thuc.json` | >= 2 Han Thuc entries | VERIFIED | 5 entries; `tet-han-thuc` holiday_id present |
| `crates/amlich-core/data/rituals/thanh-minh.json` | SolarTerm keys only (no holiday_id) | VERIFIED | 5 entries all use `solar_term` kind exclusively; zero holiday_id keys |
| `crates/amlich-core/data/rituals/doan-ngo.json` | 3 Doan Ngo variants | VERIFIED | 3 entries (simple/folk/regional:mien-bac), identical event_keys `[holiday_id:tet-doan-ngo, lunar_date:5/5]` |
| `crates/amlich-core/data/rituals/phat-dan.json` | >= 2 Phat Dan entries | VERIFIED | 4 entries; `phat-dan` holiday_id present |
| `crates/amlich-core/data/rituals/vu-lan.json` | 3+ Vu Lan variants with identical event_keys | VERIFIED | 4 entries (simple/full/buddhist/folk), all share `[holiday_id:vu-lan, lunar_date:7/15]` |
| `crates/amlich-core/data/rituals/trung-thu.json` | Trung Thu entries | VERIFIED | 3 entries; `tet-trung-thu` present |
| `crates/amlich-core/data/rituals/ong-tao.json` | Ong Tao 2 variants + Giao Thua | VERIFIED | 4 entries: ong-tao simple+full (identical event_keys), giao-thua simple+full |
| `crates/amlich-core/data/rituals/trung-cuu-ha-nguyen.json` | Trung Cuu + Ha Nguyen entries | VERIFIED | 5 entries; `tet-ha-nguyen` present |
| `crates/amlich-core/data/rituals/life-events.json` | All 6 LifeEventKind values; Nhap trach 2 variants | VERIFIED | 11 entries; all 6 kinds present (dong_tho/nhap_trach/khai_truong/cuoi/gio/day_thang); nhap_trach has 2 variants (simple/full) with identical event_keys |
| `crates/amlich-core/data/rituals/soc-vong.json` | Mung-1 + Ram LunarDate entries | VERIFIED | 3 entries using lunar_date keys only; no holiday_id |
| `crates/amlich-core/data/rituals/gia-tien-thuong-nhat.json` | Daily entries with Always key | VERIFIED | 4 entries, all use `{"kind":"always"}` event_key |
| `crates/amlich-core/data/rituals/manifest.json` | Lists all 13 corpus files, `corpus_files` field | VERIFIED | Valid JSON; `corpus_files` array listing all 13 category files; `$schema_version: "rituals-manifest-v1"` |
| `crates/amlich-core/data/rituals/provenance_audit.md` | One row per ritual_id; classical reference + reviewer | VERIFIED | 210 lines; 60-row 1:1 ledger covering all corpus ritual_ids; grouped by 13 category sub-headings; References section with 4 classical works; all reviewer fields set to `pending` |
| `crates/amlich-core/src/rituals/corpus.rs` | Multi-file include_str! loader + 4 RIT invariant tests | VERIFIED | 329 lines; 13 include_str! constants; ALL_CORPUS_JSONS array; merged all_rituals() OnceLock; 4 new tests: corpus_has_at_least_sixty_entries, every_entry_has_citation_with_page, at_least_four_events_have_multiple_variants, body_en_is_reserved_and_unset |
| `crates/amlich-core/src/rituals/schema.rs` | `body_en: Option<String>` with `#[serde(default)]` | VERIFIED | Line 141-142: `#[serde(default, skip_serializing_if = "Option::is_none")]` / `pub body_en: Option<String>` |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| corpus.rs ALL_CORPUS_JSONS array | data/rituals/*.json files (13 files) | one include_str! constant per file | WIRED | All 13 include_str! constants present and pointing to correct relative paths (`../../data/rituals/`); compile succeeds |
| corpus.rs normalize_and_validate | crate::sources::SOURCE_VN_FOLK_RITUAL | assert_eq! on every entry's source_id | WIRED | Line 124: `assert_eq!(entry.source_id, SOURCE_VN_FOLK_RITUAL, ...)` — constant used, no bare literal; source_id_guard test green |
| data/rituals/tet-nguyen-dan.json | data/holidays/lunar-festivals.json | HolidayId.value 'tet-nguyen-dan' must resolve to a real Holiday.id | WIRED | Integration test `every_holiday_id_in_fixtures_resolves_to_a_real_holiday` passes on expanded corpus |
| data/rituals/thanh-minh.json | matcher SolarTerm path | uses `{kind:solar_term,name:Thanh Minh}`, never a HolidayId | WIRED | All 5 thanh-minh entries confirmed solar_term only; no holiday_id keys |
| data/rituals/life-events.json | schema LifeEventKind enum | event uses one of dong_tho/nhap_trach/khai_truong/cuoi/gio/day_thang | WIRED | All 6 LifeEventKind snake_case values confirmed present; serde parses without error (corpus loads) |
| data/rituals/ong-tao.json | data/holidays/lunar-festivals.json | HolidayId values 'ong-tao' and 'giao-thua' must resolve | WIRED | Integration tests pass; rituals_integration test suite green |
| data/rituals/provenance_audit.md rows | ritual_ids in corpus files | one ledger row per ritual_id | WIRED | Python 1:1 check: 60 corpus IDs == 60 ledger IDs, no orphans in either direction |

### Requirements Coverage

| Requirement | Source Plan(s) | Description | Status | Evidence |
|-------------|----------------|-------------|--------|----------|
| RIT-09 | 12-01, 12-02, 12-03 | >= 60 ritual entries across <= 14 per-event-category files + manifest.json | SATISFIED | 60 entries in 13 category files + manifest.json = 14 JSON files; `corpus_has_at_least_sixty_entries` test passes |
| RIT-10 | 12-01, 12-02, 12-03 | Every entry carries source_id, original_citation (book + page), confidence tier | SATISFIED | Python scan: 0 missing pages, 0 wrong source_ids; `every_entry_has_citation_with_page` test passes |
| RIT-11 | 12-04 | Per-entry provenance_audit.md ledger with classical reference and reviewer | SATISFIED | provenance_audit.md: 60 rows, 1:1 corpus coverage, 4 classical works enumerated, reviewer=pending per research Q4 |
| RIT-12 | 12-01, 12-02, 12-03 | >= 4 events with multiple variants (same event_type, variant field discriminates) | SATISFIED | 17 events have >= 2 variants; `at_least_four_events_have_multiple_variants` test passes; confirmed: tet-nguyen-dan(4), vu-lan(4), tet-han-thuc(5), tet-nguyen-tieu(5), tet-doan-ngo(3), nhap_trach(2), ong-tao(2), etc. |
| RIT-13 | 12-03 (schema already from Phase 11) | Reserved `body_en: Option<String>` on RitualEntry, serde(default), content deferred | SATISFIED | schema.rs line 141-142 confirmed; `body_en_is_reserved_and_unset` test passes; zero corpus entries set body_en |

### Anti-Patterns Found

No blocker anti-patterns detected.

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| provenance_audit.md | all reviewer cells | reviewer=`pending` for all 60 entries | INFO | By-design per research Q4: peer review deferred post-v1.5. RIT-11 only requires the field be recorded; this is conformant. |

### Human Verification Required

None. All success criteria are programmatically verifiable for this corpus-authoring phase (file existence, entry counts, field values, test passage). Visual/UX verification is not applicable.

### Gaps Summary

No gaps. All 5 observable truths verified, all artifacts exist and are substantive, all key links wired, all 5 requirement IDs satisfied, full test suite green (601 lib tests + guard + integration tests pass with 0 failures).

**Key evidence summary:**
- 60 entries across 13 category JSON files (exactly at the 60-entry threshold)
- 17 events with multiple variants (well exceeds >= 4 requirement)
- provenance_audit.md: 1:1 coverage confirmed by automated check
- `body_en` has `#[serde(default)]` at schema.rs:141; never set in any corpus entry
- All 9 corpus tests pass, including 4 new RIT-09/10/12/13 invariant tests
- ritual_han_guard, source_id_guard, and rituals_integration all green

---

_Verified: 2026-05-27T17:00:00Z_
_Verifier: Claude (gsd-verifier)_
