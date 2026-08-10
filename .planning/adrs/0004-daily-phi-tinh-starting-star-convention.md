# ADR-0004: Daily Phi Tinh Starting-Star Convention

Status: Accepted
Date: 2026-07-15
Deciders: Phase 18 Daily Phi Tinh

---

## Context

v1.5 deferred Daily Phi Tinh (Lưu Nhật / 日紫白) because the boundary semantics were ambiguous: a "daily" layer could pivot on the naïve calendar date, the lunar calendar, or a solar-term anchor, and the Vietnamese-language secondary modern sources reviewed in v1.5 did not surface a single, unambiguous convention that was also compatible with the v1.5 boundary discipline (CRIT-2 / ADR-0002: no naïve calendar arithmetic, all boundaries resolve via the v1.1.2 `TietKhiScanner`). The v1.5 PITFALLS register explicitly logged this as MOD-2 "boundary semantics ambiguity" and called for an explicit ADR before the daily layer could be implemented (see `research/SUMMARY.md:65, 183`).

Phase 13 (2026-05-28) locked the v1 Phi Tinh primitives and the annual + monthly time-layers. Phase 14 (2026-06) added the 81-cell star-pair aspect table. Phase 16 (2026-07-15, ADR-0003a) promoted the pre-1984 Thượng/Trung Nguyên polarity rows from MEDIUM to HIGH confidence via independent secondary modern cross-check. Phase 17 (2026-07-15) closed RIT-14/15/16 reviewer field requirements and shipped v1.5 / v1.6 ledger integration. All four prior v1.6 phases are complete; Phase 18 inherits the validated boundary discipline, the additive-DTO pattern (`#[serde(default, skip_serializing_if = "Option::is_none")]`), and the `evidence` envelope convention.

Phase 18 closes the v1.5 daily-layer deferral by adopting the **Trung Khí pivot / Giáp Tý seed** convention from the classical verse "TAM NGUYÊN NHẬT BẠCH QUYẾT" (三元日白訣) in *Thẩm Thị Huyền Không Học* (沈氏玄空學, attributed to Thẩm Thị / 沈氏). The daily layer is a **sibling time-layer** to the annual (ADR-0003) and monthly (ADR-0002) layers — it does NOT replace or supersede either. The locked v1 `FlyingStarLayout` field set remains frozen; this ADR only authorises (a) the 6-pivot partition of the year, (b) the Dương-thuận / Âm-nghịch direction rule, (c) the Giáp Tý-as-seed-day mechanic, and (d) the additive `FlyingStarPeriod::Daily { date: (i32, u32, u32) }` variant + sibling `DailyFlyingStarLayout` struct in `types.rs`.

---

## Decision

### 1. Boundary semantics — 6 Trung Khí pivots, NOT calendar quarters

Daily Phi Tinh pivots on the **6 Trung Khí** (Đông Chí, Vũ Thuỷ, Cốc Vũ, Hạ Chí, Xử Thử, Sương Giáng) — NOT on the naïve calendar dates Dec 21 / Jun 21 or on lunar-month boundaries. The boundary resolution is always via the v1.1.2 `TietKhiScanner::terms_for_year(year)` (which returns all 24 Tiết Khí with `(name, jd)` pairs); for each year, filter the 24-term list by the 6 pivot names to obtain the 6 pivot JDs. This inherits the CRIT-2 / ADR-0002 boundary discipline: no synthetic approximation, exact JD per year from the existing scanner. The pivot JDs vary year-to-year — for example Đông Chí 2024 = 2024-12-21 22:21 ICT vs. Đông Chí 2025 = 2025-12-21 04:02 ICT — and using naïve calendar arithmetic regresses when the JD offset is non-trivial.

### 2. Pivot table — seeded starting star per pivot

The following table (locked from `18-RESEARCH.md` §Phase Requirements + the "Tam Nguyên Nhật Bạch Quyết" verse) defines the starting star seeded at the FIRST Giáp Tý of each Tiết Khí pivot:

| Pivot        | Starting Star | Classical name   | Polarity | Direction   |
|--------------|---------------|------------------|----------|-------------|
| Đông Chí      | 1 (Nhất Bạch) | White Water      | Dương    | thuận       |
| Vũ Thuỷ      | 7 (Thất Xích) | Red Fire         | Dương    | thuận       |
| Cốc Vũ       | 4 (Tứ Lục)    | Green Wood       | Dương    | thuận       |
| Hạ Chí       | 9 (Cửu Tử)    | Purple Fire      | Âm       | nghịch      |
| Xử Thử       | 3 (Tam Bích)  | Jade/Green Wood  | Âm       | nghịch      |
| Sương Giáng  | 6 (Lục Bạch)  | White Metal      | Âm       | nghịch      |

The starting star is anchored at the Lo Shu number, then advanced (`thuận`) or descended (`nghịch`) once per Giáp Tý cycle as described in §3 and §4 below. The centre palace of the daily layout therefore depends on (a) which pivot is current for the input date and (b) how many Giáp Tý days have elapsed since the pivot's seed Giáp Tý.

### 3. Direction rule — Dương = thuận (forward), Âm = nghịch (descending)

**Dương pivot → thuận hành (forward, +1 per Giáp Tý cycle, mod 9 wrapping 1↔9).
Âm pivot → nghịch hành (descending, -1 per Giáp Tý cycle, mod 9 wrapping 1↔9).**

This is the **OPPOSITE** direction rule from the annual layer (ADR-0003 §4: dương year = nghịch, âm year = thuận). The daily rule uses **Tiết-Khí pivot polarity** (winter/summer-class), NOT year-stem polarity. A reader encountering both layers must NOT confuse the two rules. The pivot's polarity is encoded as `PivotKind { DuongPivot, AmPivot }`, derived directly from the pivot name (`Đông Chí | Vũ Thuỷ | Cốc Vũ` ⇒ DuongPivot; `Hạ Chí | Xử Thử | Sương Giáng` ⇒ AmPivot). The palace fill always uses `pub(crate) fn fill_palaces(center, ascending)` in `annual.rs`, which encodes the thuận walk along the Lo Shu path; the `ascending` parameter is `true` for Dương pivots, `false` for Âm pivots.

### 4. Giáp Tý-as-seed-day mechanic (Pitfall P-7)

Each Tiết Khí spans approximately 15 days. The pivot "kicks in" at the **FIRST Giáp Tý** (Can=0, Chi=0) day with `JD >= pivot_jd` — NOT at the pivot instant itself. Days in the new Tiết Khí but BEFORE the first Giáp Tý of that Tiết Khí STILL use the PREVIOUS pivot's seed and direction.

The concrete worked example from phongthuycaivan.org for Đông Chí 2021 (21/12/2021, Quý Mão day) is illustrative: "từ Quý Mão đến Giáp Tý là 22 ngày… ngày 11/1/2022 mới đúng ngày Giáp Tý, an từ Nhất Bạch… trước ngày 10/1/2022 dùng tiết khí Sương Giáng suy nghịch của năm trước" — i.e., from 21/12/2021 to 10/1/2022 (a 21-day window) the algorithm uses Sương Giáng (the prior pivot, seed 6, nghịch direction), and only on 11/1/2022 (the first Giáp Tý within the new Đông Chí Tiết Khí) does the new pivot seed = 1 take effect.

The algorithm reduces to (a) find the pivot Tiết Khí P bracketing the input date, (b) find the first Giáp Tý G with `JD >= P.jd`, (c) count Giáp Tý cycles from G to the target date (call it N), (d) `daily_center = (seed ± N) mod 9` where the sign is + for Dương pivot (thuận) and - for Âm pivot (nghịch).

### 5. Classical citation — *Thẩm Thị Huyền Không Học*, chapter "三元日白訣 / Tam Nguyên Nhật Bạch Quyết"

The verse governing the daily starting star and direction rule is:

> **"三元日白訣 / Tam Nguyên Nhật Bạch Quyết"** (Three-Yuan Daily White Decision)
>
> *"Đông Chí Nhất Bạch, Vũ Thuỷ Xích, Cốc Vũ nguyên tòng Tứ Lục cầu / Hạ Chí Cửu Tử, Xử Thử Bích, Sương Giáng tiên tòng Lục Bạch du."*
>
> (Winter solstice seeds Nhất Bạch; Rain Water seeds Thất Xích; Awakening of Insects seeds Tứ Lục — all advance forward. Summer solstice seeds Cửu Tử; Limit of Heat seeds Tam Bích; Frost Descent seeds Lục Bạch — all retreat backwards.)

This verse appears in *Thẩm Thị Huyền Không Học* (沈氏玄空學) by Thẩm Thị (沈氏), chapter **"三元日白訣 / Tam Nguyên Nhật Bạch Quyết"** ("Three-Yuan Daily White Decision").

**Note that exact page-level citation is deferred per Phase 16 deferral discipline** (mirrors the 1960 Trung Nguyên `PendingExternalReview` marker in ADR-0003a §4). A numbered edition of *Thẩm Thị Huyền Không Học* is not located in the open Vietnamese-language references reviewed for Phase 18 research; the three Vietnamese-language modern references (phongthuycaivan.org, phongthuyhocvungtau.com, phongthuyphamsuu.com) all cite the classical text by chapter + verse name only, without page numbers. The chapter + verse citation IS achievable from the open references; the exact page awaits a numbered-edition lookup. This deferral does not weaken the algorithm itself — the convention is consistent across all three independent secondary modern sources and across the classical-text-by-chapter citation — but the audit trail acknowledges that classical page authority remains "by chapter" pending physical or digital access to the numbered edition.

Review owner: `external-huyen-khong-reviewer`. Expected review date:
2026-12-31. Review, resolution, and escalation follow
[`docs/architecture/external-review-lifecycle.md`](../../docs/architecture/external-review-lifecycle.md).

### 6. Alternative conventions considered — explicitly REJECTED

The FS-17 success criterion mandates listing at least three alternative daily starting-star conventions that were explicitly considered and rejected. Each carries the literal `REJECTED` token so a future reader can audit the reasoning trail.

#### 6.1 — Naïve calendar-bounded pivots (Dec 21 / Jun 21 ± 1 day) — **REJECTED**

Computing pivot dates as naïve calendar dates ("winter solstice ≈ Dec 21", "summer solstice ≈ Jun 21") inherits the CRIT-2 / ADR-0002 boundary discipline failure mode exactly. Synthetic approximation regresses when real Tiết Khí instants shift year-to-year: the JD offset between the calendar date and the actual JD is non-trivial (e.g., Đông Chí 2024 = 2024-12-21 22:21 ICT, Đông Chí 2025 = 2025-12-21 04:02 ICT — a ~5h30m swing from midnight UTC+7 that can flip a daily boundary depending on the input timezone). The v1.1.2 `TietKhiScanner` gives the EXACT JD per year from the underlying astronomical computation; using anything less is a self-imposed precision loss. **REJECTED.**

#### 6.2 — Annual-seed descent (start from `nien_center(year)` and add a daily offset) — **REJECTED**

Implementing daily Phi Tinh by descending from the annual centre star (`nien_center(year)`) is a tempting shortcut: "the year is already computed, just add a daily offset." This conflates two distinct polarity frameworks — the **year polarity** (ADR-0003 §4: dương year = nghịch, âm year = thuận) and the **daily-pivot polarity** (this ADR: Dương pivot = thuận, Âm pivot = nghịch). The two directions are **OPPOSITE** for the same calendar year in the same way for the same season. For example, year 2024 = Giáp Thìn (dương = nghịch per ADR-0003), but the winter-pivot season Đông Chí = dương = thuận per this ADR — the daily direction at the winter pivot is thuận, the annual direction for the same year is nghịch. Annual-seed descent would silently invert the daily convention. **REJECTED.**

#### 6.3 — Lunar-month bounded pivots (each lunar-month 1 = Đông Chí-like reset) — **REJECTED**

Using lunar-month boundaries (the 1st of each lunar month = a daily pivot reset) appears in some Vietnamese-language tutorial discussions but produces school-dependent results (PITFALLS MOD-2). Lunar-month boundaries shift Gregorian dates by ±1 day each month and the 1st-of-month rule is not consistent with the classical "Tam Nguyên Nhật Bạch Quyết" verse, which pivots on solar Trung Khí. Inconsistent with the ADR-0002 / ADR-0003 boundary discipline that already standardises on solar-term anchors for monthly and annual layers. Introducing a lunar-month rule for daily would also create a cross-layer indexing mismatch between `flying_stars` (monthly, solar-term anchored) and `daily_flying_stars` (daily, lunar-month anchored) within the same `DaySnapshot`. **REJECTED.**

### 7. Adopted convention — Trung-Khí-pivot / Giáp-Tý-seed per "Tam Nguyên Nhật Bạch Quyết"

The adopted convention is the Trung-Khí-pivot / Giáp-Tý-seed convention from §1–§5 above, anchored in the classical chapter "三元日白訣 / Tam Nguyên Nhật Bạch Quyết" of *Thẩm Thị Huyền Không Học*. The locked schema impact is exactly two additive extensions to `crates/amlich-core/src/almanac/fengshui/types.rs` (one enum variant, one sibling struct) and one re-export in `mod.rs` — the locked `FlyingStarLayout` field set is not mutated.

---

## Consequences

### FS-17 (closed in Plan 18-01)

- **FS-17 satisfied.** A reader of this ADR can find: which year's Tiết Khí scanner output seeds the daily count (§1), the 6-pivot table with classical names (§2), the Dương-thuận / Âm-nghịch direction rule (§3), the Giáp Tý-as-seed-day mechanic with a worked example (§4), a chapter + verse citation in *Thẩm Thị Huyền Không Học* with explicit page-deferral note (§5), and 3 alternative conventions explicitly considered with reasons for the chosen one (§6).
- **Plan 18-01 lands** the ADR + the `DailyFlyingStarLayout` sibling struct + the additive `FlyingStarPeriod::Daily { date: (i32, u32, u32) }` variant + one `mod.rs` re-export line + one extended unit test (`test_flying_star_period_serde_round_trip`) and one new test (`test_daily_flying_star_layout_period_serde`) in `types.rs`. Locked `FlyingStarLayout` field set is unchanged (verified by grep — exactly 4 `pub` fields remain).

### FS-16 / FS-18 / FS-19 (forward pointers to subsequent Phase 18 plans)

- **FS-16** (Plan 18-02): `compute_daily_flying_stars(date, scanner) -> DailyFlyingStarLayout` lands in a new `crates/amlich-core/src/almanac/fengshui/daily.rs`. The algorithm uses the 6-pivot partition from §2, the direction rule from §3, and the Giáp-Tý-seed mechanic from §4. Reuses `pub(crate) fn fill_palaces(center, ascending)` from `annual.rs` for palace layout — no duplication.
- **FS-18** (Plan 18-03): Daily golden dataset with ≥ 10 dates per Vận (7/8/9) and ≥ 2 sources per case lands in `crates/amlich-core/data/almanac/flying_stars_daily_golden.json`. Sources per case: phongthuycaivan.org + lasotuvi.com / phongthuyso.vn + the direct calculation from the locked Tam Nguyên Nhật Bạch Quyết formula as the third independent anchor. Source disagreements logged as `KnownDivergence` per FS-10 / ADR-0003a §4 (NOT silently corrected). The `kind: "daily"` validation rule is added to `golden.rs::validate_phi_tinh_golden` (one-line OR-clause extension).
- **FS-19** (Plan 18-04): `DaySnapshot.daily_flying_stars: Option<DailyFlyingStarLayout>` lands in `crates/amlich-core/src/lib.rs` with the exact `#[serde(default, skip_serializing_if = "Option::is_none")]` attribute already used for `flying_stars` at `lib.rs:163-164`. The `tests/day_snapshot_v14_compat.rs` Test 3 pattern is reused for the v1.5→v1.6 fixture round-trip test. A new `tests/fengshui_crit3_isolation.rs` (or extension to `source_id_guard.rs`) is added that scans `crates/amlich-core/src/interaction/direction_merge.rs` for `FlyingStar|DailyFlyingStar|DailyFlyingStarLayout|almanac::fengshui` — a zero-match assertion preserving CRIT-3 isolation.

### Backward compatibility

- **ADR-0002 (monthly anchor)** remains authoritative for the monthly time-layer. No monthly rules change.
- **ADR-0003 (annual polarity matrix)** remains authoritative for the Niên Tử Bạch year polarity rule. §6 is superseded by ADR-0003a; nothing in ADR-0003 is otherwise affected by this ADR.
- **ADR-0003a (confidence closure)** remains authoritative for the pre-1984 / 1960 case disposition. No deferral marker is changed by this ADR.
- **This ADR is a SIBLING decision for the daily time-layer.** It is not a supersession of any prior ADR. The two file-level artefacts are: (a) this document at `.planning/adrs/0004-daily-phi-tinh-starting-star-convention.md`, and (b) the additive Rust extensions in `crates/amlich-core/src/almanac/fengshui/types.rs` + `mod.rs`.
- **No future classical reference is promised by this ADR.** The classical citation trails to "chapter + verse name in *Thẩm Thị Huyền Không Học*"; an exact-page upgrade awaits a numbered-edition lookup and would land in ADR-0004a, not as an amendment to this document.

---

## References

- **Classical (chapter + verse, page-level citation deferred)**:
  - *Thẩm Thị Huyền Không Học* (沈氏玄空學) by Thẩm Thị (沈氏), chapter "三元日白訣 / Tam Nguyên Nhật Bạch Quyết" — canonical verse governing the daily starting-star decision rule and direction.
- **Vietnamese-language secondary modern sources (independent)**:
  - phongthuycaivan.org — "Cách tra Phi tinh Niên Nguyệt Nhật Thời" — the complete "Tam Nguyên Nhật Bạch Quyết" verse with explicit pivot table, direction rule, and the Giáp Tý transition warning ("Đừng lầm tưởng cứ ngày Giáp Tý sau Đông Chí đều là Nhất Bạch").
  - phongthuyhocvungtau.com — "CÁCH TÍNH PHI TINH NIÊN NGUYỆT NHẬT THỜI CỦA TAM NGUYÊN CỬU VẬN" — restates the same 6-pivot table and the "Dương thuận, Âm nghịch" direction rule.
  - phongthuyphamsuu.com — "LƯU NHẬT PHI TINH" — confirms the 6-Trung-Khí classification split into 3 Dương and 3 Âm.
- **In-repo cross-references**:
  - `.planning/adrs/0002-phi-tinh-monthly-anchor.md` — boundary-discipline precedent (real Tiết Khí scanner, no naïve year arithmetic). Inherited by this ADR for the daily layer.
  - `.planning/adrs/0003-nien-tu-bach-polarity.md` — annual polarity rule, opposite direction from daily.
  - `.planning/adrs/0003a-nien-tu-bach-polarity-confidence-closure.md` — deferral discipline precedent (page-citation deferral mirrors the 1960 case deferral).
  - `.planning/phases/18-daily-phi-tinh/18-RESEARCH.md` — Phase 18 research that produces the 6-pivot table, the direction rule, the Giáp-Tý-seed mechanic, and the alternative-conventions enumeration.

---

*Adopted: 2026-07-15 (Phase 18-01)*
*No supersessions. Sibling to ADR-0002 (monthly) and ADR-0003 (annual).*
