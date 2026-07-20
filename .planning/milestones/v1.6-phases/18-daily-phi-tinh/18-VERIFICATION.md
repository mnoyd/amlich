---
phase: 18-daily-phi-tinh
verified: 2026-07-15T14:13:29Z
status: passed
score: 5/5 must-haves verified
re_verification: No — initial verification
---

# Phase 18: Daily Phi Tinh (日紫白) Verification Report

**Phase Goal:** User can call `compute_daily_flying_stars(date, term_scanner)` to get the 9-palace daily grid with 冬至/夏至 reversal semantics, find a documented ADR capturing the daily starting-star convention, query a multi-source daily golden dataset, and observe daily charts in `DaySnapshot` via an additive field — all without breaking CRIT-3 isolation.

**Verified:** 2026-07-15T14:13:29Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

The 5 success criteria from ROADMAP.md (lines 81-86) serve as the goal-backward truths.

| #   | Truth                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | Status     | Evidence                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| --- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 1   | A caller can invoke `compute_daily_flying_stars(date, term_scanner) -> DailyFlyingStarLayout` and receive a 9-palace daily grid honouring 冬至/夏至 reversal semantics; boundary semantics always via the v1.1.2 real-Tiết-Khí scanner (no naïve `year` arithmetic — covered by grep/wrapper-test guard)                                                                                                                                                                                  | ✓ VERIFIED | `crates/amlich-core/src/almanac/fengshui/daily.rs:80-179` implements the public `compute_daily_flying_stars(date: (i32, u32, u32), scanner: &TietKhiScanner) -> DailyFlyingStarLayout`. Boundary resolution uses `scanner.terms_for_year(y)` via `daily_pivots_for_year` (daily.rs:56-76) — no naïve calendar arithmetic. 6-pivot classification (Dương = Đông Chí/Vũ Thuỷ/Cốc Vũ = thuận; Âm = Hạ Chí/Xử Thử/Sương Giáng = nghịch) at daily.rs:36-54 honours the 冬至/夏至 reversal. 11/11 daily unit tests pass, including `test_compute_daily_boundary_discipline_via_tiet_khi_scanner` (P-6 guard) and `test_compute_daily_direction_inversion_duong_vs_am` (P-4 guard). `test_compute_daily_pivot_in_winter_differs_from_nien_center` (P-3 guard) confirms daily center ≠ annual `nien_center` (the daily seed does not descend from the annual chart). |
| 2   | A reader can open ADR-0004 and find: which year's annual chart seeds the daily count, how the 冬至/夏至 pivot reverses the forward sequence, a chapter+page citation in *Thẩm Thị Huyền Không Học*, and a list of alternative conventions considered with reasons for the chosen one                                                                                                                                      | ✓ VERIFIED | `.planning/adrs/0004-daily-phi-tinh-starting-star-convention.md` (131 lines) carries: §1 boundary semantics (6 Trung Khí pivots via scanner), §2 the 6-pivot table (all 6 pivot names + all 6 classical star names verified present), §3 Dương-thuận / Âm-nghịch direction rule (explicitly noted OPPOSITE of ADR-0003 annual), §4 Giáp-Tý-as-seed-day mechanic with worked 2021 Đông Chí example, §5 *Thẩm Thị Huyền Không Học* chapter "三元日白訣 / Tam Nguyên Nhật Bạch Quyết" citation (10 mentions of the verse name; 8 mentions of the classical text), §6 three alternative conventions each carrying literal `REJECTED` token (8 `REJECTED` literals — well above the ≥3 floor). **Note:** literal `chapter + page` text does not appear; the ADR cites "chapter + verse" with an explicit page-deferral phrase per Phase 16 deferral discipline (`exact page-level citation is deferred` verified present once). This is a documented, intentional deviation — see "Deviations from literal success-criterion text" below. |
| 3   | A reader can find a daily-chart golden dataset (extending `data/almanac/flying_stars_golden.json` with `kind: "daily"` cases, or a new `data/almanac/flying_stars_daily_golden.json`) with ≥ 10 reference dates per Vận (7/8/9), ≥ 2 independent classical sources per case, *Thẩm Thị Huyền Không Học* as tiebreaker, and any source disagreements logged as `KnownDivergence` (not silently corrected) | ✓ VERIFIED | `crates/amlich-core/data/almanac/flying_stars_daily_golden.json` (601 lines, 36 cases) verified: `case_count=36`, **12 cases per Vận 7/8/9** (≥ 10 floor met), all cases `kind: "daily"`, all cases carry ≥ 2 sources (phongthuycaivan.org + lasotuvi.com / phongthuyso.vn), all cases cite *Thẩm Thị* or *Tam Nguyên Nhật Bạch Quyết* in `tiebreaker`, all 6 pivot names spanned (Cốc Vũ / Hạ Chí / Sương Giáng / Vũ Thủy / Xử Thử / Đông Chí), 1 `KnownDivergence` row present (Hạ Chí 2025-06-28, phongthuycaivan.org=9 vs lasotuvi.com=8, logged with `DeferralMarker` — NOT silently corrected). Loaded via `load_daily_flying_stars_golden()` (golden.rs) with OnceLock + validate_phi_tinh_golden. `daily_golden_dataset_meets_coverage_floor` + `daily_golden_dataset_divergence_log_supports_fs18_discipline` integration tests pass. |
| 4   | A caller can deserialize a v1.5 `DaySnapshot` JSON (with `flying_stars` but no `daily_flying_stars`) into a v1.6 `DaySnapshot` struct and re-serialize without unexpected fields; the v1.6 struct has an additive `daily_flying_stars: Option<DailyFlyingStarLayout>` field with `#[serde(default, skip_serializing_if = "Option::is_none")]` — verifiable by an extension of `tests/day_snapshot_v14_compat.rs`                                       | ✓ VERIFIED | `crates/amlich-core/src/lib.rs:171-172` declares `#[serde(default, skip_serializing_if = "Option::is_none")] pub daily_flying_stars: Option<crate::almanac::fengshui::types::DailyFlyingStarLayout>` — EXACT serde attribute pair matching `flying_stars` (lib.rs:163-164) and `applicable_rituals` (lib.rs:166-167). Auto-populated in `calculate_day_snapshot_internal` (lib.rs:349-361) via `compute_daily_flying_stars((year, month, day), &TietKhiScanner::new())`. `tests/day_snapshot_v14_compat.rs` carries 6 tests (3 pre-existing + 3 new: `v15_json_without_daily_flying_stars_deserializes`, `daily_flying_stars_byte_equal_round_trip`, `daily_flying_stars_absent_when_none`) — all 6/6 pass. v1.5→v1.6 backward-compat (missing key → None), byte-equal round-trip, and None→absent-in-JSON all verified at runtime. |
| 5   | The new `daily_flying_stars` path does NOT introduce `FlyingStar` or `DailyFlyingStar` into `interaction/direction_merge.rs`; CRIT-3 isolation is preserved and grep-verified (`tests/source_id_guard.rs` or a dedicated grep-test)                                                                                                                                                                                                      | ✓ VERIFIED | `crates/amlich-core/src/interaction/direction_merge.rs` (250 lines) contains ZERO matches for `FlyingStar|DailyFlyingStar|DailyFlyingStarLayout|almanac::fengshui|phi_tinh|compute_daily_flying_stars` (rg returns count 0, exit 1). `crates/amlich-core/tests/fengshui_crit3_isolation.rs` (new, 44 lines) enforces this with a `FORBIDDEN_TYPE_NAMES` list of 6 patterns covering every leak vector (type name, sibling struct name, module path, snake-case module path, function name) and a single `direction_merge_does_not_import_flying_star_or_daily_flying_star` test that reads `direction_merge.rs` at compile time via `env!("CARGO_MANIFEST_DIR")`. Test passes 1/1 — CRIT-3 isolation preserved and grep-guarded for future regressions. |

**Score:** 5/5 truths verified

### Required Artifacts

All artifacts verified at three levels: exists (Level 1), substantive (Level 2), wired (Level 3).

| Artifact | Expected | Status | Details |
| --- | --- | --- | --- |
| `.planning/adrs/0004-daily-phi-tinh-starting-star-convention.md` | ADR-0004 daily starting-star convention (6-pivot table + Dương-thuận/Âm-nghịch + Giáp-Tý-seed + chapter+verse citation + 3 alternatives) | ✓ VERIFIED | 131 lines; Status/Date/Deciders frontmatter present; all 6 pivots + 6 classical stars + Tam Nguyên Nhật Bạch Quyết (10×) + Thẩm Thị Huyền Không Học (8×) + 8 REJECTED + page-deferral phrase. |
| `crates/amlich-core/src/almanac/fengshui/types.rs` | Additive `Daily { date: (i32, u32, u32) }` variant + sibling `DailyFlyingStarLayout` struct | ✓ VERIFIED | `Daily { date: (i32, u32, u32) }` at line 105; `pub struct DailyFlyingStarLayout` at line 137; locked `FlyingStarLayout` field set UNCHANGED (exactly 4 `pub` fields: period, palaces, center_star, evidence). |
| `crates/amlich-core/src/almanac/fengshui/mod.rs` | Re-export `DailyFlyingStarLayout` + `compute_daily_flying_stars` + `load_daily_flying_stars_golden` | ✓ VERIFIED | `pub mod daily;` (line 14); `pub use daily::compute_daily_flying_stars;` (line 27); `load_daily_flying_stars_golden` in golden re-export block (line 31); `DailyFlyingStarLayout` in types re-export block (line 45). |
| `crates/amlich-core/src/almanac/fengshui/daily.rs` | `compute_daily_flying_stars` algorithm + pivot_kind + pivot_starting_star + daily_pivots_for_year helpers + 11 unit tests | ✓ VERIFIED | 419 lines; `pub fn compute_daily_flying_stars` (line 80); `fn pivot_kind` (line 36); `fn pivot_starting_star` (line 44); `fn daily_pivots_for_year` (line 56); method `"phi_tinh.nhat"` (line 169); `fill_palaces(center ...)` reuse (line 158); `get_day_canchi` + `jd_from_date` + `terms_for_year` wired; 11 `#[test]` functions confirmed. |
| `crates/amlich-core/data/almanac/flying_stars_daily_golden.json` | Multi-source daily golden dataset (≥ 30 cases, ≥ 10 per Vận, ≥ 2 sources per case, spans all 6 pivots) | ✓ VERIFIED | 601 lines; 36 cases (12 per Vận 7/8/9); all kind=daily; all ≥2 sources; all 6 pivots spanned; 1 KnownDivergence row. |
| `crates/amlich-core/src/almanac/fengshui/golden.rs` | Validator OR-clause extension to "daily" + load_daily_flying_stars_golden loader + pivot additive field | ✓ VERIFIED | `pub fn load_daily_flying_stars_golden` present; OR-clause `case.kind == "annual" \|\| case.kind == "monthly" \|\| case.kind == "daily"` (verified by passing 18-03 tests). |
| `crates/amlich-core/tests/fengshui_daily_integration.rs` | 4 external-crate black-box tests | ✓ VERIFIED | 4 test functions: `daily_golden_dataset_meets_coverage_floor`, `daily_golden_dataset_per_case_algorithm_resolution`, `daily_golden_dataset_divergence_log_supports_fs18_discipline`, `daily_algorithm_boundary_date_correctness_p6`. |
| `crates/amlich-core/src/lib.rs` | Additive `daily_flying_stars: Option<DailyFlyingStarLayout>` field on DaySnapshot + auto-populate in `calculate_day_snapshot_internal` | ✓ VERIFIED | Field decl (lib.rs:171-172) with exact serde attributes; `daily_flying_stars: None` constructor init (lib.rs:324); populate block (lib.rs:349-361) calling `compute_daily_flying_stars`. No other DaySnapshot field mutated. |
| `crates/amlich-core/tests/day_snapshot_v14_compat.rs` | Extension with 3 new round-trip tests | ✓ VERIFIED | 3 new test functions appended (lines 139, 160, 181); total 6 tests; all pass. |
| `crates/amlich-core/tests/fengshui_crit3_isolation.rs` | New grep guard test for CRIT-3 isolation | ✓ VERIFIED | 44 lines; `FORBIDDEN_TYPE_NAMES` with 6 patterns (FlyingStar, DailyFlyingStar, DailyFlyingStarLayout, almanac::fengshui, phi_tinh, compute_daily_flying_stars); `direction_merge_does_not_import_flying_star_or_daily_flying_star` test passes 1/1. |

### Key Link Verification

| From | To | Via | Status | Details |
| --- | --- | --- | --- | --- |
| `daily.rs` | `scanner.rs` | `scanner.terms_for_year(year)` call to filter 6 pivot names | ✓ WIRED | daily.rs:68 `for t in scanner.terms_for_year(y)`; verified by `test_daily_pivots_for_year_returns_six_pivots`. |
| `daily.rs` | `annual.rs` | `fill_palaces(center, ascending)` reuse for 9-palace walking fill | ✓ WIRED | daily.rs:20 `use crate::almanac::fengshui::annual::fill_palaces;`; daily.rs:158 `fill_palaces(center as u8, ascending)`. |
| `daily.rs` | `canchi.rs` | `get_day_canchi(jd)` for Giáp Tý (can_index==0 && chi_index==0) seed detection | ✓ WIRED | daily.rs:25 `use crate::canchi::get_day_canchi;`; called at daily.rs:99, 125, 142. |
| `daily.rs` | `julian.rs` | `jd_from_date(day, month, year)` to bridge calendar ↔ JD | ✓ WIRED | daily.rs:26 `use crate::julian::jd_from_date;`; daily.rs:85 `jd_from_date(d as i32, m as i32, y)`. |
| `mod.rs` | `daily.rs` | `pub mod daily;` + `pub use daily::compute_daily_flying_stars;` | ✓ WIRED | mod.rs:14 + mod.rs:27. |
| `ADR-0004` | `types.rs` | ADR names `Daily { date: (i32, u32, u32) }` variant shape + sibling struct name + additive-only field-set discipline | ✓ WIRED | ADR §7 + Consequences §FS-17 reference the additive extensions; types.rs carries exactly those two extensions. |
| `flying_stars_daily_golden.json` | `golden.rs` | `include_str!` loader | ✓ WIRED | `FLYING_STARS_DAILY_GOLDEN_JSON` const + `load_daily_flying_stars_golden()` loader present; passes 4 integration tests. |
| `tests/fengshui_daily_integration.rs` | `daily.rs` | `compute_daily_flying_stars(date, &scanner)` called with case dates | ✓ WIRED | Imports `use amlich_core::almanac::fengshui::{compute_daily_flying_stars, ...}`; `daily_golden_dataset_per_case_algorithm_resolution` calls it. |
| `lib.rs` (calculate_day_snapshot_internal) | `daily.rs` | `compute_daily_flying_stars((year, month, day), &scanner)` called inside snapshot builder | ✓ WIRED | lib.rs:351 `use crate::almanac::fengshui::{compute_daily_flying_stars, TietKhiScanner};`; lib.rs:356 `snap.daily_flying_stars = Some(compute_daily_flying_stars(...))`. |
| `tests/fengshui_crit3_isolation.rs` | `interaction/direction_merge.rs` | file-read at compile time via `env!("CARGO_MANIFEST_DIR")` | ✓ WIRED | fengshui_crit3_isolation.rs:25-27 reads `src/interaction/direction_merge.rs`; 0 forbidden patterns found → test passes. |

### Requirements Coverage

All 4 Phase 18 requirement IDs cross-referenced against `.planning/REQUIREMENTS.md` and the actual codebase. No orphaned requirements.

| Requirement | Source Plan | Description | Status | Evidence |
| --- | --- | --- | --- | --- |
| **FS-16** | 18-02 | `compute_daily_flying_stars(date, scanner) -> DailyFlyingStarLayout` returning 9-palace daily grid honouring 冬至/夏至 reversal per daily-boundary ADR (FS-17) | ✓ SATISFIED | REQUIREMENTS.md:24 (marked `[x]` Complete) + REQUIREMENTS.md:56 traceability table (Complete, 2026-07-15, 18-02). daily.rs implements the algorithm; 11 unit tests pass; full fengshui suite green. |
| **FS-17** | 18-01 | Documented ADR-0004 capturing daily starting-star convention + 冬至/夏至 reversal + chapter/page in *Thẩm Thị Huyền Không Học* + alternative conventions | ✓ SATISFIED | REQUIREMENTS.md:25 (marked `[x]` Complete) + REQUIREMENTS.md:57 traceability (Complete). ADR-0004 authored with 6-pivot table, direction rule, Giáp-Tý-seed mechanic, chapter+verse citation, page-deferral note, 3 alternatives (8 REJECTED literals). |
| **FS-18** | 18-03 | Daily golden dataset with ≥ 10 dates per Vận, ≥ 2 sources per case, *Thẩm Thị Huyền Không Học* tiebreaker, `KnownDivergence` log | ✓ SATISFIED | REQUIREMENTS.md:26 (marked `[x]` Complete) + REQUIREMENTS.md:58 traceability (Complete, 2026-07-15, 18-03). `flying_stars_daily_golden.json` (36 cases, 12 per Vận, all ≥2 sources, classical tiebreaker, 1 KnownDivergence row); `fengshui_daily_integration.rs` 4/4 pass. |
| **FS-19** | 18-04 | Additive `daily_flying_stars: Option<DailyFlyingStarLayout>` field with v1.5 fixtures round-tripping cleanly through the new field absent | ✓ SATISFIED | REQUIREMENTS.md:27 (marked `[x]` Complete) + REQUIREMENTS.md:59 traceability (Complete). DaySnapshot field with exact serde attributes; auto-populate; `day_snapshot_v14_compat.rs` 6/6 pass; `fengshui_crit3_isolation.rs` 1/1 pass. |

**Orphaned requirements check:** REQUIREMENTS.md maps exactly FS-16/17/18/19 to Phase 18 (line 79 of ROADMAP.md). All 4 appear in plan frontmatter (`requirements: [FS-17]`, `[FS-16]`, `[FS-18]`, `[FS-19]`). No orphaned requirements.

### Anti-Patterns Found

Scanned all Phase 18 source/test files for TODO/FIXME/XXX/HACK/PLACEHOLDER/placeholder/coming soon/will be here/return null/return {}/return []/=> {} patterns.

| File | Line | Pattern | Severity | Impact |
| --- | --- | --- | --- | --- |
| (none) | — | — | — | Zero anti-patterns found across all 7 Phase 18 files. |

### Deviations from Literal Success-Criterion Text

Two cosmetic deviations from the literal ROADMAP.md success-criterion text were observed. Both are documented in the plans/SUMMARYs, functionally equivalent, and do not block goal achievement.

1. **[INFO] `date` parameter type — `(i32, u32, u32)` tuple vs `NaiveDate`**
   - **Success criterion 1 text:** `compute_daily_flying_stars(date: NaiveDate, term_scanner: &TietKhiScanner)`
   - **Actual signature:** `pub fn compute_daily_flying_stars(date: (i32, u32, u32), scanner: &TietKhiScanner) -> DailyFlyingStarLayout`
   - **Why this is INFO, not a gap:** The 18-RESEARCH.md (line 45) explicitly evaluated both options and noted "the existing `i32` day/month/year tuple works equally well; planner may choose either." The 18-01-PLAN.md (lines 51, 117-126) locked the i32-tuple form to avoid introducing a `chrono::NaiveDate` dependency — the tuple mirrors the existing `SolarDate { day: i32, month: i32, year: i32 }` style at lib.rs:110-115. The phase goal ("user can call compute_daily_flying_stars(date, term_scanner)") is type-agnostic; the user CAN call it and gets the daily grid with reversal semantics. Functional equivalence preserved.

2. **[INFO] Citation level — "chapter + verse" with explicit page deferral vs literal "chapter+page"**
   - **Success criterion 2 text:** "a chapter+page citation in *Thẩm Thị Huyền Không Học*"
   - **Actual ADR:** Cites chapter "三元日白訣 / Tam Nguyên Nhật Bạch Quyết" with explicit page-deferral note (`exact page-level citation is deferred`) per Phase 16 deferral discipline.
   - **Why this is INFO, not a gap:** The 18-RESEARCH.md (line 31) flagged this as the "remaining classical-research gap" and pre-authorized the deferral: "ADR-0004 must either (a) cite a specific page if a digital copy is locatable before the plan, or (b) explicitly log the page as PendingExternalReview per Phase 16's deferral discipline (mimicking the 1960 case)." The chosen path (b) is documented, mirrors the precedent ADR-0003a §4 1960 Trung Nguyên case, and the algorithm itself is unaffected (the convention is consistent across all 3 independent secondary modern sources and the classical-text-by-chapter citation). A numbered-edition page upgrade is explicitly slated for a future ADR-0004a, not an amendment.

### Human Verification Required

No items require human verification. All 5 success criteria are verifiable programmatically (algorithm tests, ADR content grep, dataset coverage counts, serde round-trip tests, CRIT-3 grep guard). The two INFO-level deviations are documented decisions, not behavioural items needing a human in the loop.

If a future reviewer wishes to upgrade the ADR-0004 citation to exact-page, that is a follow-up task (ADR-0004a) tracked under Phase 16 deferral discipline — not a blocker for Phase 18 goal achievement.

### Gaps Summary

**No gaps found.** All 5 observable truths verified. All 10 required artifacts exist, are substantive (Level 2), and are wired (Level 3). All 10 key links verified. All 4 requirement IDs (FS-16/17/18/19) satisfied and marked Complete in REQUIREMENTS.md. Zero anti-patterns. CRIT-3 isolation preserved (0 forbidden patterns in `direction_merge.rs`, grep-guarded for future regressions).

**Build & test status at verification time:**
- `cargo build -p amlich-core`: clean (0 warnings blocking).
- `cargo test -p amlich-core --lib almanac::fengshui::daily::`: 11/11 pass.
- `cargo test -p amlich-core --test fengshui_daily_integration`: 4/4 pass.
- `cargo test -p amlich-core --test day_snapshot_v14_compat`: 6/6 pass (3 pre-existing + 3 new).
- `cargo test -p amlich-core --test fengshui_crit3_isolation`: 1/1 pass.
- `cargo test -p amlich-core` (full crate): 709 lib tests + all integration tests pass, zero regressions.

**Phase 18 goal achieved.** The user can call `compute_daily_flying_stars(date, term_scanner)`, find ADR-0004 documenting the daily convention, query the multi-source daily golden dataset, and observe daily charts in `DaySnapshot` via the additive `daily_flying_stars` field — all without breaking CRIT-3 isolation.

---

_Verified: 2026-07-15T14:13:29Z_
_Verifier: Claude (gsd-verifier)_
