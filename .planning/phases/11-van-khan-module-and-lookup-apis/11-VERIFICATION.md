---
phase: 11-van-khan-module-and-lookup-apis
verified: 2026-05-26T00:00:00Z
status: passed
score: 5/5 success criteria verified
---

# Phase 11: Văn khấn Module + Lookup APIs — Verification Report

**Phase Goal:** User can call five public APIs from `crates/amlich-core/src/rituals/` to look up rituals by snapshot, event key, or life event — backed by an `OnceLock` corpus loader with NFC normalization and Hán-character guard.

**Verified:** 2026-05-26
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths (from ROADMAP Success Criteria)

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | A caller can pass a `DaySnapshot` for Tết Nguyên Đán and `find_van_khan_for_snapshot(&snapshot)` returns the matching ritual entries (joined via `Holiday.id` + `RitualEntry.event_keys[]`). | VERIFIED | Integration test `tet_nguyen_dan_2024_snapshot_returns_tet_ritual` (tests/rituals_integration.rs:147) PASSES — calls `calculate_day_snapshot(10, 2, 2024)`, finds entry with `HolidayId{"tet-nguyen-dan"}`. Inline test `tet_snapshot_returns_tet_rituals` confirms wiring at white-box level. |
| 2 | A caller can resolve `find_van_khan_for_event`, `find_van_khan_for_life_event`, `get_ritual_by_id`, and `all_rituals()` against the loaded corpus. | VERIFIED | All four APIs `pub`-defined: `matcher.rs:21,35,44,50` and `corpus.rs:38`. All re-exported via `rituals/mod.rs:26-33`. Integration test `every_holiday_id_in_fixtures_resolves_to_a_real_holiday` calls `all_rituals()`. White-box tests `find_van_khan_for_event_holiday_id_tet`, `find_van_khan_for_life_event_dong_tho`, `get_ritual_by_id_known`/`_unknown_is_none` all PASS. |
| 3 | A code reader can find a closed `RitualEventKey` enum covering Sóc/Vọng, the 8 major lunar festivals, Tiết Khí anchors, life events, and `Always` — with the matcher's exhaustiveness enforced by the compiler. | VERIFIED | `schema.rs:67-78` defines closed 5-variant enum: `HolidayId`, `LunarDate { month, day, leap_month_policy }`, `SolarTerm`, `LifeEvent`, `Always`. Sóc=`LunarDate{day:1}`, Vọng=`LunarDate{day:15}`, 8 festivals via `HolidayId` (verified by `every_holiday_id_in_fixtures_resolves_to_a_real_holiday` test against 2020-2030 holidays corpus). `event_key_matches` in `matcher.rs:114-140` covers all variants under `_` wildcard — Rust exhaustiveness compile-time enforced. |
| 4 | A caller can rely on `LunarDateMatch` having `MonthDay { month, day, leap_month_policy }`, `SolarTerm`, and `GregorianFixed` variants, with leap-month policy defaulting to `CanonicalMonthOnly`. | VERIFIED | `schema.rs:30-46` defines `LunarDateMatch` enum with `MonthDay { month, day, leap_month_policy }`, `SolarTerm`, `GregorianFixed` variants. Default tested via `lunar_date_month_day_defaults_leap_policy_to_canonical_month_only` schema test (PASSES). Locked by Phase 10 ADR-0001. |
| 5 | CI rejects any ritual JSON whose body contains Hán characters above the configured threshold; loaded text is NFC-normalized and verifiable via a round-trip byte-equal test. | VERIFIED | `tests/ritual_han_guard.rs` enforces threshold=0 Hán per `is_han_char` covering CJK Unified+Ext-A+Ext-B+Compatibility blocks. Test PASSES. `corpus.rs:56-98` `normalize_and_validate` runs every text field through `nfc()`. White-box test `every_text_field_is_nfc_normalized` PASSES. Integration test `every_entry_round_trips_byte_equal_through_serde_json` (tests/rituals_integration.rs) PASSES — verifies byte-equal NFC round-trip via serde_json. |

**Score:** 5/5 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
| --- | --- | --- | --- |
| `crates/amlich-core/Cargo.toml` | `unicode-normalization = "0.1.25"` dep line | VERIFIED | Line 20 contains exact match. `cargo build -p amlich-core` succeeds. |
| `crates/amlich-core/data/rituals/fixtures.json` | RitualFile with 6 entries, NFC, all required ritual_ids | VERIFIED | NFC validated (Python `unicodedata.is_normalized` returns True). `$schema_version == "rituals-v1"`. 6 entries with exact ritual_ids: `van-khan-tet-don-gian`, `van-khan-ram-thang-gieng`, `van-khan-thanh-minh`, `van-khan-dong-tho`, `van-khan-gia-tien-hang-ngay`, `van-khan-doan-ngo`. All have `source_id == "vn-folk-ritual"`. |
| `crates/amlich-core/tests/ritual_han_guard.rs` | Hán-guard integration test | VERIFIED | 61 lines, contains `ritual_corpus_rejects_han_characters` test (line 28). `cargo test -p amlich-core --test ritual_han_guard` passes (1/1). |
| `crates/amlich-core/src/rituals/corpus.rs` | OnceLock + include_str! loader with NFC + source_id validation | VERIFIED | 172 lines (≥80 required). Contains `pub fn all_rituals`, `SOURCE_VN_FOLK_RITUAL` (no bare literal), `include_str!("../../data/rituals/fixtures.json")`, `unicode_normalization::{is_nfc, UnicodeNormalization}`. 5 white-box tests PASS. |
| `crates/amlich-core/src/rituals/matcher.rs` | Four lookup APIs + event_key_matches + derive_event_keys | VERIFIED | 254 lines (≥120 required). All four `pub fn` present. 9 white-box tests PASS. |
| `crates/amlich-core/src/rituals/mod.rs` | Registers `mod corpus;`, `mod matcher;`, re-exports schema + APIs | VERIFIED | 34 lines. Contains `pub mod schema;`, `mod corpus;`, `mod matcher;`, `pub use schema::*;`, `pub use corpus::all_rituals;`, `pub use matcher::{...}`. |
| `crates/amlich-core/tests/rituals_integration.rs` | Six integration tests | VERIFIED | 198 lines (≥100 required). Six `#[test]` functions all PASS. Imports via `amlich_core::rituals::*` (external-crate path). |

### Key Link Verification

| From | To | Via | Status | Details |
| --- | --- | --- | --- | --- |
| `tests/ritual_han_guard.rs` | `data/rituals/*.json` | `fs::read_dir` over `CARGO_MANIFEST_DIR/data/rituals` | WIRED | Line 30: `Path::new(env!("CARGO_MANIFEST_DIR")).join("data/rituals")`. Test scans every `.json` file; `is_han_char` covers 4 CJK blocks. |
| `data/rituals/fixtures.json` | ADR-0001 RitualEntry shape | RitualFile envelope | WIRED | `$schema_version: "rituals-v1"` present; entries parse against locked `RitualEntry` (schema tests pass; corpus loads without panic). |
| `corpus.rs` | `data/rituals/fixtures.json` | `include_str!("../../data/rituals/fixtures.json")` | WIRED | Line 19 contains exact path; relative path resolves correctly (corpus loads at runtime, all corpus tests pass). |
| `corpus.rs` | `crate::sources::SOURCE_VN_FOLK_RITUAL` | Import + `assert_eq!` | WIRED | Line 16 imports the constant; line 60 asserts `entry.source_id == SOURCE_VN_FOLK_RITUAL` for every entry. No bare `"vn-folk-ritual"` literal in `src/`. |
| `corpus.rs` | `unicode_normalization::{is_nfc, UnicodeNormalization}` | use clause + nfc() helper | WIRED | Line 13 imports; `nfc()` helper (line 100) applied to every text field in `normalize_and_validate`. |
| `mod.rs` | `corpus.rs` | `mod corpus;` declaration | WIRED | Line 23 declares submodule. |
| `matcher.rs::find_van_khan_for_snapshot` | `crate::rituals::corpus::all_rituals` | linear filter | WIRED | Line 15 imports `all_rituals`; line 23 invokes; integration tests confirm hits are non-empty for known snapshots. |
| `matcher.rs::derive_event_keys` | `crate::holidays::get_vietnamese_holidays` | join on solar date + Holiday.id | WIRED | Line 14 import; line 67 loop joins `(solar_day, solar_month)`; integration test `every_holiday_id_in_fixtures_resolves_to_a_real_holiday` confirms IDs cross-resolve. |
| `matcher.rs::find_van_khan_for_life_event` | `RitualEventKey::LifeEvent` | wrap + delegate | WIRED | Line 45 wraps and delegates to `find_van_khan_for_event`. Integration test `find_van_khan_for_life_event_dong_tho` confirms. |
| `mod.rs` | matcher exports | `pub use matcher::{...}` | WIRED | Lines 28-33 re-export all four lookup APIs. Integration test imports via `amlich_core::rituals::{find_van_khan_for_event, find_van_khan_for_snapshot, ...}` and resolves. |
| `tests/rituals_integration.rs` | `amlich_core::rituals::find_van_khan_for_snapshot` | external `use amlich_core::rituals::*` | WIRED | Line 139 import block; integration tests resolve all four APIs externally. |
| `tests/rituals_integration.rs` | `amlich_core::holidays::get_vietnamese_holidays` | cross-reference holiday ids | WIRED | Line 138 imports; Test 4 verifies every `HolidayId` in fixtures resolves to a real `Holiday.id` for 2020-2030. |

### Requirements Coverage

| Requirement | Source Plan(s) | Description | Status | Evidence |
| --- | --- | --- | --- | --- |
| RIT-01 | 11-03, 11-04 | `find_van_khan_for_snapshot(&DaySnapshot) -> Vec<RitualEntry>` | SATISFIED | `matcher.rs:21` `pub fn find_van_khan_for_snapshot`; integration test `tet_nguyen_dan_2024_snapshot_returns_tet_ritual` PASSES end-to-end. |
| RIT-02 | 11-03 | `find_van_khan_for_event(&RitualEventKey) -> Vec<RitualEntry>` | SATISFIED | `matcher.rs:35` `pub fn find_van_khan_for_event`; white-box + integration tests pass. |
| RIT-03 | 11-03 | `find_van_khan_for_life_event(LifeEventKind)` | SATISFIED | `matcher.rs:44`; test `find_van_khan_for_life_event_dong_tho` PASSES. `LifeEventKind` enum covers `DongTho`, `NhapTrach`, `KhaiTruong`, `Cuoi`, `Gio`, `DayThang` (six required). |
| RIT-04 | 11-03 | `get_ritual_by_id(&str) -> Option<RitualEntry>` | SATISFIED | `matcher.rs:50`; both `get_ritual_by_id_known` and `_unknown_is_none` tests PASS. |
| RIT-05 | 11-02 | `all_rituals() -> &'static [RitualEntry]` | SATISFIED | `corpus.rs:38` `pub fn all_rituals`; OnceLock idempotency verified by `get_or_init_is_idempotent`. |
| RIT-06 | 11-03 | Closed `RitualEventKey` enum (Sóc/Vọng, 8 festivals, Tiết Khí, life events, Always) | SATISFIED | `schema.rs:67` 5-variant closed enum; compile-time exhaustiveness enforced (rustc `match`). Sóc/Vọng via `LunarDate{day:1}`/`{day:15}` paths; 8 festivals via `HolidayId` (cross-referenced for 2020-2030 in test 4). |
| RIT-07 | 11-03, 11-04 | `LunarDateMatch` variants + default `CanonicalMonthOnly` | SATISFIED | `schema.rs:30-46` defines `LunarDateMatch::{MonthDay, SolarTerm, GregorianFixed}`. Default verified by `lunar_date_month_day_defaults_leap_policy_to_canonical_month_only`. API-surface leap-policy semantics verified by `leap_month_only_needle_does_not_match_canonical_only_entry`. |
| RIT-08 | 11-01, 11-02, 11-04 | NFC at load + Hán-character CI guard | SATISFIED | CI guard at `tests/ritual_han_guard.rs` (threshold 0). NFC normalize at load in `corpus.rs:68-96`. Round-trip byte-equal verified by `every_entry_round_trips_byte_equal_through_serde_json`. |

**Coverage:** 8/8 RIT-01..08 requirements satisfied. No orphaned requirements per REQUIREMENTS.md mapping (`Phase 11 | Complete`).

### Anti-Patterns Found

None. Scanned `corpus.rs`, `matcher.rs`, `mod.rs`, `tests/rituals_integration.rs`, `tests/ritual_han_guard.rs` for `TODO|FIXME|XXX|HACK|PLACEHOLDER|unimplemented!|todo!` — zero hits.

### Notable Deviations from Plan

| Deviation | Plan claimed | Actual | Acceptable? |
| --- | --- | --- | --- |
| `event_key_matches` Always semantics | Symmetric (`(Always, _) | (_, Always) => true`) | Asymmetric (only `(Always, _) => true` — haystack-only) | YES — documented inline (`matcher.rs:99-113`). The integration test `vong_snapshot_returns_ram_thang_gieng_via_snapshot_path` falsifies the prior symmetric behavior (it would have caused every entry to fire on every snapshot via the `Always`-needle derived in `derive_event_keys`). The `always_sentinel_matches_anything` inline test (matcher.rs:194-214) was updated accordingly and PASSES. All five Phase 11 success criteria still satisfied. |

### Test Run Summary

- `cargo test -p amlich-core --lib rituals` → **19/19 PASS** (corpus 5, matcher 9, schema 5).
- `cargo test -p amlich-core --test rituals_integration` → **6/6 PASS**.
- `cargo test -p amlich-core --test ritual_han_guard` → **1/1 PASS**.

### Gaps Summary

None. Phase 11 goal fully achieved:
- All 5 ROADMAP success criteria observable and tested.
- All 7 required artifacts exist, substantive, and wired.
- All 12 key links verified.
- All 8 requirements (RIT-01..08) satisfied with code + test evidence.
- Zero anti-patterns; zero `TODO`/stubs in shipped Phase-11 files.
- One deviation (asymmetric `Always`) is intentional, documented, tested, and strictly improves correctness over the planned symmetric version.

REQUIREMENTS.md already marks RIT-01..08 as "Complete (Phase 11)" — verification confirms this status.

---

_Verified: 2026-05-26_
_Verifier: Claude (gsd-verifier)_
