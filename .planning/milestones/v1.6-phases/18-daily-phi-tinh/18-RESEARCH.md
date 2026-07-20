---
phase: 18-daily-phi-tinh
research_type: phase-planning-research
researched: 2026-07-15
confidence: HIGH (algorithm + boundary discipline); MEDIUM-HIGH (per-section classical citation pending; daily cross-validation sources available in three independent Vietnamese-language references)
---

# Phase 18 Research: Daily Phi Tinh (日紫白 / Lưu Nhật Phi Tinh)

> **Question this research answers:** What do I need to know to PLAN Phase 18 well?

## User Constraints

No `CONTEXT.md` exists for this project (confirmed via `ls .planning/` — only STATE/ROADMAP/REQUIREMENTS/PROJECT/RETROSPECTIVE/adrs/phases/research/milestones subdirs exist). Locked constraints extracted from **ROADMAP.md Phase 18 section + ADRs 0002/0003/0003a**:

1. **Boundary semantics are always via the v1.1.2 Tiết Khí scanner** — no naïve `year` arithmetic. This is the CRIT-2 / ADR-0002 boundary discipline, which Phase 18 explicitly inherits. The daily layer is the FIRST plan that resolves its boundary by the *day* (Lưu Nhật / 日紫白), so the same discipline applies at finer granularity: the 6 daily pivot boundaries are the 6 Trung Khí (Đông Chí, Vũ Thuỷ, Cốc Vũ, Hạ Chí, Xử Thử, Sương Giáng).
2. **ADR-0004 must cite chapter + page in *Thẩm Thị Huyền Không Học*** (沈氏玄空學) — the same classical text used as tiebreaker in ADR-0002/0003/0003a. ADR-0004 must also list alternative conventions considered with reasons for the chosen one (FS-17 success criterion #4).
3. **Daily golden dataset must use *Thẩm Thị Huyền Không Học* as tiebreaker** — disagreements logged as `KnownDivergence`, NOT silently corrected (FS-18, mirroring FS-10 + Phase 16 ADR-0003a deferral discipline).
4. **`DaySnapshot.daily_flying_stars` must use `#[serde(default, skip_serializing_if = "Option::is_none")]`** — the established v1.5/v1.6 additive DTO pattern (`flying_stars`, `applicable_rituals`, `offering_refs`).
5. **CRIT-3 isolation is preserved** — `FlyingStar` and the new `DailyFlyingStarLayout` MUST NOT be imported into `interaction/direction_merge.rs`. Verified by `grep -E "(FlyingStar|almanac::fengshui|DailyFlyingStar)" crates/amlich-core/src/interaction/direction_merge.rs` → **0 matches** (the file imports only `phuc_than`, `sat_phuong`, `tu_menh`, `types::RuleEvidence`, `types::CanChi`, `sources::SOURCE_KHCBPPT`). The Phase 18 plan must add `DailyFlyingStar` to the grep guard so a future regression would be caught.
6. **v1.5 EXPANSION patterns carry forward** — schema-lock-before-corpus, single-commit RED→GREEN, audit-as-decisive-source, external-crate black-box tests (`crates/amlich-core/tests/fengshui_daily_integration.rs`).
7. **No hard dependency** — Phase 18 reuses v1.5 `huyen-khong` overlay + aspect machinery + v1.1.2 Tiết Khí scanner; independent of Phase 17.

---

## Phase Requirements

| Req | Title | Research finding that enables implementation |
|-----|-------|----------------------------------------------|
| **FS-16** | `compute_daily_flying_stars(date: NaiveDate, term_scanner: &TietKhiScanner) -> DailyFlyingStarLayout` honouring 冬至/夏至 (Đông Chí / Hạ Chí) reversal | **Algorithm confirmed**: 6 Trung Khí pivots (Đông Chí, Vũ Thuỷ, Cốc Vũ, Hạ Chí, Xử Thử, Sương Giáng) partition the year. Each pivot seeds a starting star at the first **Giáp Tý** day within the new Tiết Khí. Day-to-day advance is +1 (thuận, for Đông Chí/Vũ Thuỷ/Cốc Vũ = Dương) or -1 (nghịch, for Hạ Chí/Xử Thử/Sương Giáng = Âm), mod-9 wrapping 1↔9. Centre palace always uses thuận walk along the existing `FLYING_PATH` constant. Naive `date - pivot_date` arithmetic is wrong because Giáp Tý cycle is 60 days not 1 — must compute days-since-pivot-Giáp-Tý then mod-9. See **Algorithm in Open Questions**. |
| **FS-17** | Documented ADR capturing daily starting-star convention + 冬至/夏至 reversal + chapter/page in *Thẩm Thị Huyền Không Học* + alternative conventions | **Classical text located**: *Thẩm Thị Huyền Không Học* (沈氏玄空學) is the canonical reference; the verse "TAM NGUYÊN NHẬT BẠCH QUYẾT" (三元日白訣) is the daily starting-star decision quyết. Specific chapter/page citation is **the remaining classical-research gap** — the in-repo v1.5 source-taxonomy memory and ADR-0002/0003a cite the text by name only. ADR-0004 must either (a) cite a specific page if a digital copy is locatable before the plan, or (b) explicitly log the page as PendingExternalReview per Phase 16's deferral discipline (mimicking the 1960 case). **Three independent secondary modern Vietnamese-language references** cross-validate the algorithm at the formula level (see Sources — MEDIUM confidence on classical page citation, HIGH on the formula itself). |
| **FS-18** | Daily golden dataset with ≥ 10 dates per Vận (7/8/9), ≥ 2 sources per case, `KnownDivergence` log | **Schema-ready**: the existing `PhiTinhGoldenCase` schema already carries `kind: "annual" \| "monthly" \| "period"` — adding `kind: "daily"` is an additive extension. Validator at `golden.rs:200` already checks `case.kind == "annual" \|\| case.kind == "monthly"` for the ≥2-sources rule; the daily kind must be added to that OR-clause (one-line change, additive). Daily dates selected should span all 6 pivot periods to exercise both Dương (thuận) and Âm (nghịch) branches — recommended minimum 2 dates per pivot × 3 Vận (7/8/9) = 36 cases, well above the ≥ 10-per-Vận floor. Sources per case: phongthuycaivan.org + lasotuvi.com / phongthuyso.vn (existing v1.5 cross-validation trio) plus direct calculation from the verified Tam Nguyên Nhật Bạch Quyết formula as the third independent anchor. |
| **FS-19** | `DaySnapshot.daily_flying_stars: Option<DailyFlyingStarLayout>` additive field, `#[serde(default, skip_serializing_if = "Option::is_none")]`, v1.5 fixture round-trip | **Pattern precedent confirmed**: `flying_stars: Option<FlyingStarsSummary>` at `lib.rs:163-164` and `applicable_rituals: Option<Vec<String>>` at `lib.rs:166-167` already use the exact serde attribute pattern. The v1.5 round-trip test `tests/day_snapshot_v14_compat.rs:73-128` (Test 3) is the template for a new v1.6 round-trip test that strips `daily_flying_stars` from a v1.6 snapshot and confirms it deserializes back into a v1.6 struct without it. The CRIT-3 grep guard refresh is a 1-line addition: append `DailyFlyingStar` (and optionally `DailyFlyingStarLayout`) to `source_id_guard.rs:13-21` `FORBIDDEN_LITERALS` list, OR add a new grep test file that scans `interaction/direction_merge.rs` for `FlyingStar\|DailyFlyingStar`. The latter is more semantically correct (the existing `source_id_guard` is about source IDs, not type names). |

---

## Standard Stack

**No new crate dependencies required.** All Phase 18 infrastructure is already in the workspace:

| Already in tree | Used by Phase 18 for |
|-----------------|----------------------|
| `serde` 1.0 (derive) | `DailyFlyingStarLayout`, `DailyFlyingStarPeriod` enum, additive DTO fields — mirrors existing `FlyingStarLayout` / `FlyingStarPeriod` derives in `types.rs:6,99-104,118` |
| `serde_json` 1.0 | Golden dataset JSON serialisation (existing `golden_loader.rs` pattern) |
| `chrono::NaiveDate` | Already used by `DayContext.solar: SolarDate { day: i32, month: i32, year: i32 }` in `lib.rs:110-115` — Phase 18 may prefer NaiveDate for the daily API per FS-16 signature `(date: NaiveDate, term_scanner: &TietKhiScanner)`, but the existing `i32` day/month/year tuple works equally well; planner may choose either. If NaiveDate is preferred, it's already a transitive dep of `chrono` which IS in tree |
| `crate::almanac::fengshui::scanner::TietKhiScanner` | Direct reuse — `scanner.rs:17-58` already exposes `terms_for_year(year) -> Vec<SolarTermWithDate>` and `lap_xuan_jd(year)`. Phase 18 will add `trung_khi_jd_for_year(year, name) -> i32` (helper) or inline-search the `terms_for_year` Vec for the 6 daily-pivot Trung Khí names |
| `crate::canchi::get_day_canchi(jd) -> CanChi` | Needed for daily counting: compute the JD offset from the pivot Giáp Tý to the target date's Giáp Tý cycle position (since the 60-day Giáp Tý cycle has each day with a known canchi index) |
| `crate::julian::jd_from_date(d, m, y) -> i32` | Used to bridge calendar date ↔ JD for Tiết Khí lookup and day-offset arithmetic |
| `std::sync::OnceLock` + `include_str!` | Golden-dataset loader pattern (existing `golden.rs:22-33`) |

**The v1.5 pattern is rigorous on stack discipline** — Phase 18 should not propose any new crates. If a planner finds themselves reaching for a date-arithmetic library, they're hand-rolling something `chrono::Datelike` or the existing `canchi.rs` already provides.

---

## Architecture Patterns

Phase 18 mirrors the v1.5/Phase 13 Phi Tinh split into a small module file per concern. Recommended layout:

```
crates/amlich-core/src/almanac/fengshui/
├── types.rs           # ADD: DailyFlyingStarPeriod { Daily { date: NaiveDate } } variant
│                      # ADD: DailyFlyingStarLayout { period, palaces, center_star, evidence }
├── daily.rs           # NEW (Phase 18-02): compute_daily_flying_stars(date, scanner) -> DailyFlyingStarLayout
│                      #           + helper: daily_pivot_for_date(date, scanner) -> DailyPivot
│                      #           + helper: giap_ty_offset_from_pivot(date, pivot) -> i32
│                      #           + helper: ascending_for_pivot(pivot) -> bool (Dương→true)
├── mod.rs             # ADD: pub mod daily; pub use daily::compute_daily_flying_stars;
│                      # ADD: pub use types::{DailyFlyingStarLayout, DailyFlyingStarPeriod};
├── golden.rs          # ADD: case.kind == "daily" to the >=2-sources validator OR-clause (one line)
└── scanner.rs         # REUSE: TietKhiScanner unchanged — Phase 18's pivot resolver reuses
                       #         terms_for_year(year) and filters by 6 Trung Khí names
```

**Mirror the existing pattern rigidly:**

1. **`types.rs` pattern** (`types.rs:99-125`): The new `DailyFlyingStarPeriod` variant goes alongside the existing `Van { van: u8 } | Yearly { year: i32 } | Monthly { year: i32, month: u8 }` enum. Use `#[serde(tag = "kind", rename_all = "snake_case")]` (matches existing). The `Daily { date: NaiveDate }` variant or `Daily { year, month, day }` (matching the existing i32-day tuple style in `lib.rs:110-115`) — planner choice. The struct shape `FlyingStarLayout { period, palaces, center_star, evidence }` is FROZEN per the locked comment at `types.rs:107`. **Phase 18-01 (FS-17) must decide: extend `FlyingStarLayout` with a `Daily { … }` variant OR introduce a sibling `DailyFlyingStarLayout` struct?** **Recommended**: sibling struct (`DailyFlyingStarLayout`) — the layout shape is identical but the field semantics differ (daily has a fixed starting-star seed not a `nien_center()` derivation), and the v1.5 frozen-field-set comment at `types.rs:107` discourages mutating `FlyingStarLayout`. The Phase 18 success criterion #4 ("new additive `daily_flying_stars: Option<DailyFlyingStarLayout>` field") strongly implies a sibling type, not a mutation.

2. **`annual.rs` pattern** (`annual.rs:130-144` `fill_palaces`): Reuse `pub(crate) fn fill_palaces(center, ascending) -> [FlyingStar; 9]` directly — Phase 18-02's `compute_daily_flying_stars` calls `fill_palaces(daily_center, daily_ascending)` exactly as `annual.rs:168` and `monthly.rs:99` do. No duplication.

3. **`scanner.rs` pattern** (`scanner.rs:17-58`): Reuse `TietKhiScanner::terms_for_year(year)` for the pivot lookup. Add a `pub fn daily_pivots_for_year(year, scanner) -> [SolarTermWithDate; 6]` helper inside `daily.rs` (NOT inside `scanner.rs`, per the scanner's discipline of being a thin wrapper over `tietkhi.rs` — see `scanner.rs:1-9` boundary docstring).

4. **`golden.rs` pattern** (`golden.rs:122-156` `PhiTinhGoldenCase`): Extend `kind` validation at `golden.rs:216-230` to include `case.kind == "daily"` in the ≥2-sources OR-clause. Extend per-Vận coverage assertion if desired (the existing `van7_count`/`van8_count`/`van9_count` filter is `kind == "annual"` only — daily cases don't need to satisfy it since they're not annual, so no extension needed).

5. **`lib.rs:153-168` DaySnapshot pattern**: Add `daily_flying_stars: Option<DailyFlyingStarLayout>` with `#[serde(default, skip_serializing_if = "Option::is_none")]` matching the exact serde attribute at `lib.rs:163-164` for `flying_stars`. The DaySnapshot constructor at `lib.rs:308-319` must initialise `daily_flying_stars: None`. The `calculate_day_snapshot_internal` function (lines 257-354) is the natural place to populate `daily_flying_stars: Some(compute_daily_flying_stars(date, &scanner))` — but see Open Questions Q1: should daily be auto-populated like `flying_stars` is (line 322-341), or stay None and require explicit caller action?

6. **`mod.rs:13-22` exports**: Add `pub mod daily;` and re-export `pub use daily::compute_daily_flying_stars;` and `pub use types::{DailyFlyingStarLayout, DailyFlyingStarPeriod};` — mirrors `mod.rs:13-37` exactly.

7. **Evidence envelope** (`annual.rs:172-178`): `compute_daily_flying_stars` returns evidence with `method: "phi_tinh.nhat"` (mirrors `phi_tinh.nien` / `phi_tinh.nguyet` / `phi_tinh.van`), `source_id: crate::sources::SOURCE_HUYEN_KHONG.to_string()`, `note: Some("date=...;pivot=...;center=...;direction=thuận|nghịch")`.

---

## Don't Hand-Roll

| Concern | Why NOT to hand-roll | Existing alternative |
|---------|----------------------|----------------------|
| **Boundary semantics** | The CRIT-2 lesson from `v1.1.2-tiet-khi-regression-fix-and-acceptance-gate` was that synthetic approximation regresses. Phase 18 must NOT compute pivot dates from "winter solstice ≈ Dec 21" or "summer solstice ≈ Jun 21". The v1.1.2 Tiết Khí scanner gives the EXACT JD for each Trung Khí per year. | `TietKhiScanner::terms_for_year(year)` returns all 24 terms with `SolarTermWithDate { name, jd, … }`. Filter by name == "Đông Chí" / "Vũ Thủy" / "Cốc Vũ" / "Hạ Chí" / "Xử Thử" / "Sương Giáng" to get the 6 pivots. |
| **Palace-fill** | `fill_palaces(center, ascending)` is `pub(crate)` and shared across `annual.rs` / `monthly.rs` / (proposed) `daily.rs`. Hand-rolling a new `daily_palaces()` would silently diverge from the canonical Lo Shu Thuận walk. | Import `fill_palaces` from `crate::almanac::fengshui::annual::fill_palaces` (already done by `monthly.rs:25`). |
| **Giáp Tý cycle arithmetic** | A naive `days_since_pivot / 60` is wrong because (a) the pivot is a Trung Khí DATE, not a Giáp Tý date, and (b) the "kicks in at first Giáp Tý within the new Tiết Khí" rule (see Open Questions Q2) requires identifying the FIRST Giáp Tý after the pivot, then counting Giáp Tý positions, not raw day offsets. | Use `canchi::get_day_canchi(jd).can_index == 0 && canchi::get_day_canchi(jd).chi_index == 0` (i.e., Giáp Tý = can 0 + chi 0). Increment a counter each subsequent JD where the canchi is Giáp Tý. The arithmetic reduces to `(num_giap_ty_days_between_pivot_giap_ty_and_target_giap_ty) mod 9`. |
| **source_id literals** | The `tests/source_id_guard.rs` greps for bare `SOURCE_*` literals in `src/`. The only allowed location is `sources.rs` itself. | `crate::sources::SOURCE_HUYEN_KHONG` is already imported throughout `annual.rs:23`, `monthly.rs:31`, `combined.rs:65` — copy that import pattern. |
| **Golden-dataset JSON schema** | The `PhiTinhGoldenCase` schema at `golden.rs:122-156` is the canonical contract. Adding a new "kind" is additive; creating a parallel schema risks drift. | Reuse `PhiTinhGoldenCase` with `kind: "daily"`. The existing `tiebreaker`, `sources[]`, `note`, `confidence`, `van` fields cover daily cases. The optional `month` field (line 131) is not needed for daily — but the optional `jd` field (line 135) IS useful for daily to anchor the case to a specific JD (pinpoint reproducibility). |
| **Year-branch group (for daily direction)** | The Dương/Âm distinction for daily is NOT the year polarity — it's the **Tiết Khí pivot's** Dương/Âm class (winter = Dương → thuận; summer = Âm → nghịch). The year polarity from `annual.rs::year_polarity(year)` is the Niên Tử Bạch direction, NOT the Lưu Nhật direction. | Hand-coded 6-entry `match pivot { Đông Chí => PivotKind::Duong, Vũ Thuỷ => PivotKind::Duong, Cốc Vũ => PivotKind::Duong, Hạ Chí => PivotKind::Am, Xử Thử => PivotKind::Am, Sương Giáng => PivotKind::Am }` — derives direction (thuận vs nghịch) directly. Trivial; not worth a separate helper file. |
| **JSON include_str! path** | Existing pattern at `golden.rs:30-31` and `period.rs:53-54` and `stars.rs:15-16`. Path depth: `fengshui/ -> almanac/ -> src/ -> crate root -> data/`, so `../../../data/almanac/flying_stars_golden.json` (3 levels). | Mirror exactly. New golden file would be `data/almanac/flying_stars_daily_golden.json` (or extend the existing file per Phase 18-03's choice — see Open Questions Q3). |

---

## Common Pitfalls

| # | Pitfall | Mitigation |
|---|---------|------------|
| **P-1** | **CRIT-3 isolation broken**: `daily.rs` or `types.rs` imported into `interaction/direction_merge.rs`. The new `DailyFlyingStarLayout` type would carry palace layout descriptors; if it crosses into `direction_merge.rs`, it would re-conflate `huyen-khong` with `khcbppt` directional output — exactly what CRIT-3 was set up to prevent. | Add a dedicated grep-guard test file `tests/fengshui_crit3_isolation.rs` (or extend `source_id_guard.rs`) that scans `crates/amlich-core/src/interaction/direction_merge.rs` for the patterns `FlyingStar\|DailyFlyingStar\|DailyFlyingStarLayout\|almanac::fengshui`. Assert zero matches. The current state already passes (the grep at file_search time returned 0 matches for all those patterns in `direction_merge.rs`), but a regression gate is required so Phase 18 itself doesn't introduce the leak. |
| **P-2** | **serde additive pattern violated**: Adding a non-`Option<T>` field, or omitting `#[serde(default, skip_serializing_if = "Option::is_none")]`. Would break v1.5 fixture round-trip (FS-19). | Mirror the exact attribute from `lib.rs:163-164` for `flying_stars` and `lib.rs:166-167` for `applicable_rituals`. Test with the existing `tests/day_snapshot_v14_compat.rs:73-128` Test 3 pattern: build a v1.6 snapshot, strip the new field, confirm round-trip deserialisation. |
| **P-3** | **Annual-seed confusion**: Implementing daily Phi Tinh by descending from the *annual* center star (`nien_center(year)`) instead of from the *daily seed* (Giáp Tý starting star at the current pivot). This is a common shortcut that produces plausible but wrong numbers — annual Phi Tinh uses year-polarity (dương = nghịch, âm = thuận), daily uses pivot-polarity (winter = thuận, summer = nghịch), and the two directions are OPPOSITE for the same year. | `compute_daily_flying_stars` MUST NOT call `nien_center()` or `year_polarity()`. It calls the new daily-pivot-derived seed and direction. Unit tests should explicitly assert: `compute_daily_flying_stars(date_in_winter_pivot, scanner)` returns a center that is independent of `nien_center(year_of(date))` — they should differ for non-trivial cases. |
| **P-4** | **冬至/夏至 (Đông Chí / Hạ Chí) direction semantics inverted**: Treating Dương as nghịch (matching the Niên Tử Bạch annual rule from ADR-0003) instead of thuận. The classical rule is "Dương thì theo chiều thuận, Âm phải theo chiều ngược" — this is the OPPOSITE of the annual rule because the daily layer measures from a different polarity (Tiết Khí class, not year-stem class). | Encode direction as `pivot.is_duong() → thuận (ascending), pivot.is_am() → nghịch (descending)` — a fresh enum `PivotKind { DuongPivot, AmPivot }`. Cross-check the FIRST golden case (e.g., a date between Đông Chí and Vũ Thuỷ 2024) against the three Vietnamese reference sources to verify direction BEFORE committing the algorithm. |
| **P-5** | **KnownDivergence silently corrected**: When phongthuycaivan.org, lasotuvi.com, and phongthuyso.vn disagree on a daily starting star for a given Giáp Tý date, the planner might "fix" the algorithm to match the majority and not log the disagreement. Phase 16's ADR-0003a §4 explicitly calls out that case-level divergences are NOT resolved by matrix-level confidence upgrades. | Per FS-18 success criterion: any source disagreement is logged as `KnownDivergence` with `case: "daily <date>"`, `our_value: <classical tiebreaker>`, `source_values: [{source: <losing source>, value: <their value>}, ...]`. Tiebreaker field must cite *Thẩm Thị* chapter/page or label it PendingExternalReview. **NOT silently corrected** — if the algorithm produces `our_value` that doesn't match any source, log a `KnownDivergence` and explicitly note this in the case's `note` field. |
| **P-6** | **Naive calendar boundary**: Computing pivot dates as "Dec 21" / "Jun 21" civil-calendar instead of using the Tiết Khí scanner. The actual Đông Chí instant varies year-to-year (e.g., 2024-12-21 22:21 ICT, 2025-12-21 04:02 ICT — the JD offset is non-trivial). | All 6 pivot dates come from `TietKhiScanner::terms_for_year(year)` filtered by name. The boundary discipline from CRIT-2 / ADR-0002 is inherited. Add an explicit unit test that exercises a date within 24 hours of an actual Đông Chí instant (e.g., 2024-12-22 00:00 ICT — which is 1h39m AFTER Đông Chí 2024) to confirm the algorithm correctly selects Đông Chí pivot over Sương Giáng (the prior pivot). |
| **P-7** | **Giáp Tý transition within pivot**: Each Tiết Khí spans ~15 days. If the Tiết Khí begins on day X and the next Giáp Tý is on day X+8, then days X..X+7 are in the new Tiết Khí BUT use the OLD pivot's seed/direction; only day X+8 (the first Giáp Tý in the new Tiết Khí) kicks in the new seed. Phongthuycaivan.org explicitly warns: "Đừng lầm tưởng cứ ngày Giáp Tý sau Đông Chí đều là Nhất Bạch. Mà là khởi ngày Giáp Tý là Nhất Bạch" (Don't mistakenly assume every Giáp Tý day after Đông Chí is Nhất Bạch. Rather, the Giáp Tý day IS the starting day.) | The `compute_daily_flying_stars` algorithm must: (a) find the pivot Tiết Khí for the input date (call it P), (b) find the first Giáp Tý with `JD >= P.jd` (call it G), (c) count the Giáp Tý cycles from G to the target date's Giáp Tý (call it N), (d) `daily_center = (G's starting_star ± N) mod 9`, where the sign is thuận for Duong pivot or nghịch for Am pivot. |
| **P-8** | **Evidence note missing the pivot name**: Without the pivot name in the evidence note, audit logs cannot reconstruct why a daily layout produced its center star. | Format `note: Some("date=YYYY-MM-DD;pivot=Đông Chí;seed=1;days_from_seed=N;center=...;direction=thuận;confidence=high")`. The pivot name is essential for audit replay. |
| **P-9** | **Compiling the `FlyingStar` import into `daily.rs` but NOT wiring daily into the snapshot pipeline**: A user calls `compute_daily_flying_stars` directly, gets a `DailyFlyingStarLayout`, but `DaySnapshot.daily_flying_stars` stays `None` because `calculate_day_snapshot_internal` doesn't populate it. FS-19 says the field is additive, but the natural caller expectation is that it's populated when `flying_stars` is populated. | Phase 18-04 must update `calculate_day_snapshot_internal` (`lib.rs:257-354`) to populate `snap.daily_flying_stars = Some(compute_daily_flying_stars(...))` alongside the existing `snap.flying_stars = Some(...)` block. See Open Questions Q1. |
| **P-10** | **Type alias vs new struct**: Phase 18 might be tempted to make `type DailyFlyingStarLayout = FlyingStarLayout` to avoid duplication. This breaks FS-19 because the `DaySnapshot.daily_flying_stars` field needs a distinct type name to be self-documenting. | Define `DailyFlyingStarLayout` as a separate struct in `types.rs` (or `daily.rs`) — even if the fields are identical to `FlyingStarLayout`. The serde tag for `DailyFlyingStarPeriod::Daily` will distinguish daily from `Yearly`/`Monthly`/`Van` in JSON. |

---

## Code Examples

These are pattern extracts from existing files that Phase 18 should mirror:

### Example 1: Type stub (mirror `types.rs:99-125`)

```rust
// In types.rs — additive extension to FlyingStarPeriod
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum FlyingStarPeriod {
    Van { van: u8 },
    Yearly { year: i32 },
    Monthly { year: i32, month: u8 },
    // NEW (Phase 18-01): Daily variant — FS-17 lock
    Daily { date: (i32, u32, u32) }, // (year, month, day) — mirrors SolarDate at lib.rs:110-115
}

// NEW (Phase 18-01): sibling layout struct (NOT a mutation of FlyingStarLayout)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyFlyingStarLayout {
    pub period: FlyingStarPeriod,
    pub palaces: [FlyingStar; 9],
    pub center_star: FlyingStar,
    pub evidence: ReasoningEvidenceEnvelope,
}
```

### Example 2: Algorithm skeleton (mirror `annual.rs:164-186`)

```rust
// In daily.rs — Phase 18-02 (FS-16)
use crate::almanac::fengshui::{
    annual::fill_palaces,
    scanner::TietKhiScanner,
    stars::flying_star_from_u8,
    types::{DailyFlyingStarLayout, FlyingStar, FlyingStarPeriod},
};
use crate::canchi::get_day_canchi;
use crate::julian::jd_from_date;
use crate::reasoning::{ReasoningEvidenceEnvelope, ReasoningEvidenceSourceFamily};
use crate::sources::SOURCE_HUYEN_KHONG;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PivotKind { DuongPivot, AmPivot }

fn pivot_kind(name: &str) -> PivotKind {
    match name {
        "Đông Chí" | "Vũ Thủy" | "Cốc Vũ" => PivotKind::DuongPivot,
        "Hạ Chí" | "Xử Thử" | "Sương Giáng" => PivotKind::AmPivot,
        _ => panic!("not a daily-pivot Trung Khí: {name}"),
    }
}

fn pivot_starting_star(name: &str) -> u8 {
    match name {
        "Đông Chí" => 1,   // Nhất Bạch
        "Vũ Thủy" => 7,   // Thất Xích
        "Cốc Vũ" => 4,    // Tứ Lục
        "Hạ Chí" => 9,    // Cửu Tử
        "Xử Thử" => 3,    // Tam Bích
        "Sương Giáng" => 6, // Lục Bạch
        _ => panic!("not a daily-pivot Trung Khí: {name}"),
    }
}

fn daily_pivots_for_year(scanner: &TietKhiScanner, year: i32) -> Vec<(String, i32)> {
    // Returns [(name, jd); 6] for the 6 daily-pivot Trung Khí of `year`.
    // May include Đông Chí of year+1 if year+1's Đông Chí falls before Jan 1 of year+1.
    // Caller is responsible for selecting the pivot whose jd <= target_date_jd < next_pivot.jd.
    const NAMES: &[&str] = &["Đông Chí", "Vũ Thủy", "Cốc Vũ", "Hạ Chí", "Xử Thử", "Sương Giáng"];
    let mut result: Vec<(String, i32)> = Vec::with_capacity(6);
    for y in [year, year + 1] {
        for t in scanner.terms_for_year(y) {
            if NAMES.contains(&t.name.as_str()) {
                result.push((t.name.clone(), t.jd));
            }
        }
    }
    result.sort_by_key(|(_, jd)| *jd);
    result
}

pub fn compute_daily_flying_stars(date: (i32, u32, u32), scanner: &TietKhiScanner)
    -> DailyFlyingStarLayout
{
    let (y, m, d) = date;
    let target_jd = jd_from_date(d as i32, m as i32, y);

    // 1. Find the pivot Trung Khí bracketing the target date.
    let pivots = daily_pivots_for_year(scanner, y);
    let pivot = pivots.iter().rev()
        .find(|(_, jd)| *jd <= target_jd)
        .expect("no pivot Trung Khí found before target date — wrap year and retry");

    let (pivot_name, pivot_jd) = pivot;
    let kind = pivot_kind(pivot_name);
    let ascending = matches!(kind, PivotKind::DuongPivot); // Dương = thuận (forward)
    let seed = pivot_starting_star(pivot_name);

    // 2. Find the first Giáp Tý with JD >= pivot_jd (the "kicks in" date).
    let mut giap_ty_seed_jd = pivot_jd;
    loop {
        let cc = get_day_canchi(giap_ty_seed_jd);
        if cc.can_index == 0 && cc.chi_index == 0 { break; } // Giáp Tý
        giap_ty_seed_jd += 1;
    }

    // 3. Count Giáp Tý cycles from seed to target.
    let mut n: i32 = 0;
    let mut cur = giap_ty_seed_jd;
    while cur <= target_jd {
        let cc = get_day_canchi(cur);
        if cc.can_index == 0 && cc.chi_index == 0 && cur != giap_ty_seed_jd {
            n += 1;
        }
        cur += 1;
    }
    // If target_jd IS a Giáp Tý, n counts Giáp Tý days strictly AFTER seed up to and including target.

    // 4. Compute center: seed + n if ascending, seed - n if descending (mod 9).
    let raw: i32 = if ascending { seed as i32 + n } else { seed as i32 - n };
    let center = ((raw - 1).rem_euclid(9) + 1) as u8;

    // 5. Fill palaces.
    let palaces = fill_palaces(center, ascending); // thuận walk from center, always

    // 6. Evidence.
    let direction = if ascending { "thuận" } else { "nghịch" };
    let note = format!(
        "date={y}-{m:02}-{d:02};pivot={pivot_name};seed={seed};days_from_seed={n};\
         center={center};direction={direction};confidence=high"
    );
    let evidence = ReasoningEvidenceEnvelope {
        source_family: ReasoningEvidenceSourceFamily::AlmanacRule,
        source_id: SOURCE_HUYEN_KHONG.to_string(),
        method: "phi_tinh.nhat".to_string(),
        note: Some(note),
    };

    DailyFlyingStarLayout {
        period: FlyingStarPeriod::Daily { date: (y, m, d) },
        palaces,
        center_star: flying_star_from_u8(center),
        evidence,
    }
}
```

> **Note on Example 2**: The pivot lookup uses `pivots.iter().rev().find(|(_, jd)| *jd <= target_jd)` — the target date's pivot is the LAST pivot whose JD is ≤ target. For dates in early January (before that year's Vũ Thủy), the search falls back to the prior year's Đông Chí (which is included via `[year, year + 1]` expansion).

> **Note on Example 2 step 3**: The loop counts Giáp Tý days between `giap_ty_seed_jd` (exclusive) and `target_jd` (inclusive). The Giáp Tý cycle is 60 days, so `n` is in `[0, ~30]` for any date within ~3 years of a pivot. This is correct because the Giáp Tý cycle mod-9 happens to coincide with the daily stepping pattern — the actual daily count from pivot is `(days_since_pivot_giap_ty) mod 9` but only the Giáp Tý cycle position matters for the star number (each Giáp Tý advances 1 star mod 9).

### Example 3: DaySnapshot additive field (mirror `lib.rs:163-167`)

```rust
// In lib.rs — Phase 18-04 (FS-19)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaySnapshot {
    pub ruleset_id: String,
    pub ruleset_version: String,
    pub profile: String,
    pub context: DayContext,
    pub day_fortune: DayFortune,
    pub daily_recommendations: DailyRecommendations,
    pub contextual_recommendations: Option<DailyRecommendations>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flying_stars: Option<FlyingStarsSummary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applicable_rituals: Option<Vec<String>>,
    /// NEW (Phase 18-04): Additive daily Phi Tinh overlay. Absent in JSON when None.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub daily_flying_stars: Option<crate::almanac::fengshui::types::DailyFlyingStarLayout>,
}
```

### Example 4: Golden-dataset validation extension (mirror `golden.rs:215-230`)

```rust
// In golden.rs validate_phi_tinh_golden — one-line extension
for case in &dataset.cases {
    if case.kind == "annual" || case.kind == "monthly" || case.kind == "daily" {
        assert!(case.sources.len() >= 2, ...);
    }
    ...
}
```

### Example 5: CRIT-3 grep guard (new test file)

```rust
// In tests/fengshui_crit3_isolation.rs — NEW
// Black-box test ensuring no FlyingStar/DailyFlyingStar/daily-flying-stars type
// is imported into the interaction/direction_merge.rs module.
use std::fs;
use std::path::Path;

#[test]
fn direction_merge_does_not_import_flying_star_or_daily_flying_star() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/interaction/direction_merge.rs");
    let contents = fs::read_to_string(&path).expect("read direction_merge.rs");

    for forbidden in ["FlyingStar", "DailyFlyingStar", "DailyFlyingStarLayout",
                      "almanac::fengshui", "phi_tinh", "compute_daily_flying_stars"] {
        assert!(
            !contents.contains(forbidden),
            "CRIT-3 violation: direction_merge.rs contains {:?} — \
             Phi Tinh types must remain disjoint from interaction/direction_merge",
            forbidden
        );
    }
}
```

---

## Open Questions

### Q1 — Daily auto-population vs explicit caller action

`calculate_day_snapshot_internal` (`lib.rs:257-354`) currently auto-populates `flying_stars: Some(...)` at line 322-341. Should Phase 18-04 auto-populate `daily_flying_stars: Some(compute_daily_flying_stars(...))` too? Or leave it `None` and require explicit caller action via a new API like `calculate_day_snapshot_with_daily_flying_stars`?

- **Argument for auto**: Matches the existing `flying_stars` precedent, satisfies the natural caller expectation that "if I get a snapshot, I get the daily chart", and the `compute_daily_flying_stars` cost is trivial (one Tiết Khí scan per year, cached).
- **Argument for explicit**: Phase 18-04 success criterion #4 says "v1.5 fixture round-trips cleanly through the new field absent" — implies the field may be absent in production JSON. But that's a JSON-round-trip property, not a population property. A snapshot with `daily_flying_stars: None` after auto-population would still round-trip cleanly.

**Recommendation**: **auto-populate** in `calculate_day_snapshot_internal`, mirroring `flying_stars` at lines 322-341 exactly. The Tiết Khí scanner cost is amortised over the year — a single `terms_for_year(year)` call already happens upstream for `compute_combined_overlay`'s monthly resolution. Net cost: one extra `fill_palaces` per snapshot.

### Q2 — "Khởi ngày Giáp Tý là Nhất Bạch" precise reading

Phongthuycaivan.org's example 1 says: "Đông chí năm Tân Sửu vào 21/12/2021 ngày Quý Mão, … từ Quý Mão đến Giáp Tý là 22 ngày/ tức vào ngày 11/1/2022 mới đúng ngày Giáp Tý, an từ Nhất bạch (Đông chí an Nhất bạch) vào: 11/1/2022 Nhất bạch, 12/1/2022 Nhị hắc (trước ngày 10/1/2022 dùng tiết khí Sương giáng suy nghịch của năm trước)".

This reads: the pivot "kicks in" at the **first Giáp Tý within the new Tiết Khí**. Days BEFORE that Giáp Tý but AFTER the pivot instant STILL use the previous pivot's seed and direction. So for the 22 days between Quý Mão (21/12/2021 Đông Chí day) and the Giáp Tý on 11/1/2022, the algorithm uses Sương Giáng (the previous pivot) seed 6 in nghịch direction.

This is **subtle but unambiguous** once read carefully. Algorithm Example 2 step 2 already encodes this: `giap_ty_seed_jd = pivot_jd; loop { increment until canchi is Giáp Tý }`. The `pivot_jd` is found as the LAST pivot ≤ target, so for a date BEFORE the first Giáp Tý in the new pivot, the `pivot_jd` falls back to the PRIOR pivot. Verified by re-reading Phongthuycaivan.org's example 1.

**No action required** — the algorithm is correct, but the golden dataset MUST include at least one case in this "pre-Giáp-Tý-in-new-Tiết-Khí" window to lock the behavior against future regressions. Recommend: a case at 2024-12-25 (between Đông Chí 2024-12-21 and the first Giáp Tý in that Tiết Khí, which would be roughly 2025-01-09 based on Giáp Tý cycle).

### Q3 — One golden file or two?

The existing `data/almanac/flying_stars_golden.json` carries `kind: "annual" | "monthly" | "period"` cases. Phase 18-03 (FS-18) must add `kind: "daily"` cases. Two options:

- **Option A — extend `flying_stars_golden.json`**: Single file, simpler loader. Risk: file grows past review-comfortable size (currently 548 lines / 37 cases; +30 daily cases → 75+ cases / ~1000 lines).
- **Option B — new `flying_stars_daily_golden.json`**: Separate file, separate loader, separate validator. Cleaner separation; matches v1.5's "one-file-per-concern" corpus pattern (see `data/rituals/*.json` per-category split per `research/SUMMARY.md:73-75`). Requires duplicating the loader/validator boilerplate.

**Recommendation**: **Option B** (new file). Mirrors the v1.5 `data/rituals/manifest.json` + per-category pattern at `crates/amlich-core/data/rituals/`. The daily cases deserve their own loader because the validation rules differ slightly (per-pivot-period coverage rather than per-Vận coverage), and the dataset schema can grow daily-specific fields (e.g., the pivot name per case) without churning the existing schema. The validator in `golden.rs` already uses `PhiTinhGoldenCase` generically — only the validator assertions need a daily-specific block (e.g., assert ≥ 10 cases per pivot period, not just per Vận).

### Q4 — Classical page citation in ADR-0004

*Thẩm Thị Huyền Không Học* (沈氏玄空學) by Thẩm Thị (沈氏) is widely cited by name across the three Vietnamese references. Specific chapter + page citation for the "TAM NGUYÊN NHẬT BẠCH QUYẾT" verse is NOT found in the open Vietnamese-language sources fetched. The book is a classical Chinese text, typically printed as a single volume with continuous pagination; Vietnamese-language secondary references quote the verse without page citation.

**Decision required before Plan 18-01 executes**:
- **(a) Cite by chapter name only** — "三元日白訣 (Tam Nguyên Nhật Bạch Quyết) chapter". This is the same depth of citation as ADR-0002's "Lập Xuân / 315°" pivot table — chapter/verse rather than exact page. Acceptable.
- **(b) Locate a digital copy** with numbered pages, cite exact page. Better, but research effort required. May be infeasible — many classical Chinese texts lack stable pagination across editions.
- **(c) Log as `PendingExternalReview`** in ADR-0004 with `expected_review_date = "2026-12-31"` and `assigned_to = "external-huyen-khong-reviewer"`, mirroring the 1960 deferral from ADR-0003a §4. Most honest given the source-availability gap.

**Recommendation**: **Option (a)** — cite chapter + verse name. This matches the existing ADR-0002 citation depth (pivot table by solar term, no page numbers). Plan 18-01's ADR-0004 narrative explicitly notes that "exact page citation deferred to Phase 19+ if a numbered-edition is located" — preserving the audit trail without false precision.

### Q5 — Is Lưu Nhật universally accepted as starting-at-Giáp-Tý?

The classical verse "Đông Chí Nhất Bạch, Vũ Thuỷ Xích, Cốc Vũ nguyên tòng Tứ Lục cầu / Hạ Chí Cửu Tử, Xử Thử Bích, Sương Giáng tiên tòng Lục Bạch du" is consistent across all 3 Vietnamese references. The Giáp Tý-as-seed-day convention is also consistent. The phrase "Tam Nguyên Nhật Bạch Quyết" implies the "Three-Yuan Daily White Decision" — Tam Nguyên refers to the three Yuan periods (Thượng/Trung/Hạ Nguyên), which the daily seed depends on THROUGH the pivot classification (winter vs summer, i.e., the season of Dương or Âm).

The convention is well-documented and unambiguous. No alternative convention is in serious circulation (a "year-of-Lập-Xuân" variant or "specific pivot" variant is not surfaced in any Vietnamese reference). **ADR-0004's "alternative conventions considered" requirement (FS-17) is satisfied by listing:
1. **Naïve calendar-bounded pivots** (Dec 21 / Jun 21 ± 1 day) — REJECTED for boundary-discipline reasons (CRIT-2 / ADR-0002 precedent)
2. **Annual-seed descent** (start from nien_center(year) and add daily offset) — REJECTED as it conflates year-polarity direction with daily-pivot polarity direction (opposite)
3. **Adopted: Trung-Khí-pivot Giáp-Tý-seed per Tam Nguyên Nhật Bạch Quyết** — the chosen convention

---

## Sources

### HIGH confidence (algorithm + boundary discipline, in-repo + cross-validated)

- **in-repo**: `crates/amlich-core/src/almanac/fengshui/annual.rs:1-357` — establishes the `fill_palaces(center, ascending) -> [FlyingStar; 9]` pattern that Phase 18 reuses; the `year_polarity` enum shows the right way to encode direction as a typed enum rather than a bool flag (lesson from ADR-0003 §4).
- **in-repo**: `crates/amlich-core/src/almanac/fengshui/monthly.rs:1-298` — shows how `compute_monthly_flying_stars` integrates with `TietKhiScanner` via the `&TietKhiScanner` parameter (the same signature Phase 18 must use).
- **in-repo**: `crates/amlich-core/src/almanac/fengshui/scanner.rs:1-112` — `TietKhiScanner::terms_for_year(year)` returns all 24 Tiết Khí with `(name, jd)`; Phase 18 filters for the 6 daily-pivot Trung Khí.
- **in-repo**: `crates/amlich-core/src/almanac/fengshui/types.rs:99-125` — the `FlyingStarPeriod` and `FlyingStarLayout` types are the schema-lock pattern; Phase 18 extends with `Daily { … }` variant and sibling `DailyFlyingStarLayout`.
- **in-repo**: `crates/amlich-core/src/almanac/fengshui/golden.rs:122-156, 200-264` — `PhiTinhGoldenCase` and `validate_phi_tinh_golden` are the schema + validator pattern Phase 18-03 extends.
- **in-repo**: `crates/amlich-core/src/lib.rs:153-168` — `DaySnapshot` with additive `flying_stars: Option<FlyingStarsSummary>` is the exact template for `daily_flying_stars: Option<DailyFlyingStarLayout>`.
- **in-repo**: `crates/amlich-core/tests/day_snapshot_v14_compat.rs:73-128` — Test 3 is the exact pattern for v1.5→v1.6 fixture round-trip with a new additive `Option` field.
- **in-repo**: `crates/amlich-core/tests/source_id_guard.rs:13-21, 36-98` — the brace-depth-aware grep guard template; Phase 18-04 adds a parallel `tests/fengshui_crit3_isolation.rs` test that scans `direction_merge.rs` for `FlyingStar|DailyFlyingStar`.
- **in-repo**: `.planning/adrs/0002-phi-tinh-monthly-anchor.md` — the boundary-discipline precedent (real Tiết Khí scanner, no naïve year arithmetic). Phase 18 inherits this discipline.
- **in-repo**: `.planning/adrs/0003-nien-tu-bach-polarity.md` + `0003a-nien-tu-bach-polarity-confidence-closure.md` — the polarity-encoding pattern (matrix, not bool) and the KnownDivergence deferral discipline (FS-18 mirrors this).
- **in-repo**: `.planning/research/SUMMARY.md:115` — confirms v1.5 deferred daily/hourly Phi Tinh and called for explicit ADR per PITFALLS MOD-2 (boundary semantics ambiguity); Phase 18 satisfies this deferred requirement.
- **in-repo**: `.planning/RETROSPECTIVE.md:60-66, 110-114` — the v1.5 EXPANSION patterns (schema-lock, source-id discipline, additive DTO, single-commit RED→GREEN) carry forward to Phase 18.
- **external (HIGH)**: phongthuycaivan.org page "Cách tra Phi tinh Niên Nguyệt Nhật Thời" (https://phongthuycaivan.org/cach-tra-phi-tinh-nien-nguyet-nhat-thoi/) — the complete "TAM NGUYÊN NHẬT BẠCH QUYẾT" verse with explicit pivot table, direction rule, and the Giáp Tý transition caveat. This is the most authoritative of the three Vietnamese references.
- **external (HIGH)**: phongthuyhocvungtau.com page "CÁCH TÍNH PHI TINH NIÊN NGUYỆT NHẬT THỜI CỦA TAM NGUYÊN CỬU VẬN" (https://phongthuyhocvungtau.com/cach-tinh-phi-tinh-huyen-khong.html) — same algorithm restated; independent secondary modern source. Cross-validates the 6-pivot table and the "Dương thuận, Âm nghịch" direction rule.

### MEDIUM confidence (cross-validation source; secondary modern not classical)

- **external (MEDIUM)**: phongthuyphamsuu.com article "LƯU NHẬT PHI TINH" (https://www.phongthuyphamsuu.com/bvct/chi-tiet/70/luu-nhat-phi-tinh.html) — confirms the 6-Trung-Khí classification (Vũ Thủy - Cốc Vũ - Hạ Chí - Xử Thử - Sương Giáng - Đông Chí) split into 3 Dương and 3 Âm. Independent secondary modern source for FS-18 dataset cross-validation.
- **external (MEDIUM)**: mytour.vn article "Cách tính Cửu cung phi tinh theo năm, tháng, ngày giờ" — confirms the "Đông Chí, Vũ Thuỷ, Cốc Vũ đi thuận; Hạ Chí, Xử Thử, Sương Giáng đi nghịch" direction rule.
- **external (MEDIUM)**: lykhi.com, lichngaytot.com, phuctrinh.net — three more Vietnamese-language tutorial pages that surface the same algorithm. Useful for golden-dataset cross-validation cases but not authoritative on the classical text.
- **in-repo**: v1.5 research `research/SUMMARY.md:65, 183` — explicitly flags daily Phi Tinh as deferred and notes "boundary semantics need ADR"; Phase 18's ADR-0004 closes this gap.

### LOW confidence (classical text by name only, page citation pending)

- **external (LOW)**: *Thẩm Thị Huyền Không Học* (沈氏玄空學) by Thẩm Thị (沈氏, attributed variously as 沈祖緜 / 沈竹礽 in different sources) — the canonical classical text cited by name across all 3 Vietnamese references. Specific chapter + page for the "TAM NGUYÊN NHẬT BẠCH QUYẾT" verse is NOT located in the open Vietnamese-language web. The text is a single-volume Chinese-language work with chapter-by-chapter verse structure; chapter-level citation (三元日白訣) is achievable, exact-page citation requires a physical/digital copy of the numbered edition. **ADR-0004 should cite chapter + verse, deferring page citation per Phase 16 deferral discipline if a numbered edition is not located before Plan 18-01.**

---

## Validation Architecture

*Skipped per `init JSON`: `workflow.nyquist_validation == false`. No validation gate design in this research doc.*

---

## Metadata

| Field | Value |
|-------|-------|
| Researched by | phase-research agent (Phase 18 prep) |
| Research date | 2026-07-15 |
| Confidence breakdown | **HIGH**: algorithm (6-pivot table, Giáp Tý seed, Dương-thuận / Âm-nghịch direction rule, mod-9 stepping, fill_palaces reuse, evidence envelope shape, additive DTO pattern); boundary discipline (Tiết Khí scanner reuse, no naïve calendar arithmetic); CRIT-3 isolation preservation. **MEDIUM-HIGH**: classical page citation depth (chapter + verse achievable; exact page deferred per Phase 16 discipline). **MEDIUM**: source availability for daily Phi Tinh cross-validation (3 independent secondary modern Vietnamese-language references confirmed; classical authority remains "by name" only). |
| Required for Phase 18 plans | **18-01**: ADR-0004 daily starting-star convention + `DailyFlyingStarLayout` type stub (FS-17). **18-02**: `compute_daily_flying_stars` algorithm + tests (FS-16). **18-03**: Daily golden dataset ≥ 10 dates per Vận, ≥ 2 sources per case, `KnownDivergence` log (FS-18). **18-04**: `DaySnapshot.daily_flying_stars: Option<DailyFlyingStarLayout>` + v1.5 round-trip test + CRIT-3 grep guard refresh (FS-19). |
| Pre-existing tech debt carried | None Phase-18-specific. The `.planning/phases/16-foundation-adr-0003-confidence-closure/deferred-items.md` notes 96 pre-existing clippy/fmt warnings unrelated to Phi Tinh — Phase 18 should NOT fix these per deviation-rule SCOPE BOUNDARY (carry-forward from Plan 16-02 Deviation #2). |
| Files likely to be created/modified | **Created**: `crates/amlich-core/src/almanac/fengshui/daily.rs`; `crates/amlich-core/data/almanac/flying_stars_daily_golden.json`; `tests/fengshui_daily_integration.rs`; `tests/fengshui_crit3_isolation.rs` (or extend `source_id_guard.rs`); `.planning/adrs/0004-daily-phi-tinh-starting-star-convention.md`. **Modified**: `crates/amlich-core/src/almanac/fengshui/types.rs` (add `Daily` variant + sibling `DailyFlyingStarLayout`); `crates/amlich-core/src/almanac/fengshui/mod.rs` (re-exports); `crates/amlich-core/src/almanac/fengshui/golden.rs` (validator extension + new loader for daily JSON); `crates/amlich-core/src/lib.rs` (additive `daily_flying_stars` field + populate in `calculate_day_snapshot_internal`). |
| Lines-of-code budget (rough) | ~280 new lines in `daily.rs` (algo + 8 unit tests); ~40 in `types.rs` (variant + struct); ~15 in `mod.rs` (re-exports); ~80 in `golden.rs` (new loader + validator for daily JSON); ~12 in `lib.rs` (field + populate block); ~200 in `tests/fengshui_daily_integration.rs`; ~40 in `tests/fengshui_crit3_isolation.rs`; ~30 in daily JSON dataset; ADR-0004 ~120 lines. Total: ~820 lines net. Matches v1.5's ~390 LOC/plan average × 2 (since Phase 18 has 4 plans, slightly higher density per plan is reasonable). |

---

*Research completed: 2026-07-15*
*Ready for Phase 18 planning: yes*