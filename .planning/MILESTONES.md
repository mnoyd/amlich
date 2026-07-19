# Milestones: Amlich Almanac Correctness Audit

## v1.7 Kinh Dịch (I-Ching Divination) (Shipped: 2026-07-19)

**Delivered:** Added the P2 Kinh Dịch pillar — Mai Hoa Dịch Số casting (`cast_mai_hoa`) + Biến Quẻ derivation + Thể/Dụng classification + Ngô Tất Tố 64-hexagram corpus + `IChingEvaluator` (Tier-0, no birth data required) — as a new Tier-0 reasoning capability, plus the Thái Tuế / Tam Sát ⇄ Phi Tinh read-only directional cross-link (`build_direction_cross_link`) carrying dual-source provenance (KHCBPPT + Huyền-Không). All 15 requirements satisfied; full crate 1120 tests pass; zero new crate dependencies.

**Phases completed:** 6 phases (20-25), 14 plans

**Key accomplishments:**
- **Foundation schema lock + ADRs (Phase 20):** ADR-0005 locked `HexagramEntry` JSON schema v1 (`deny_unknown_fields`); ADR-0006 locked Mai Hoa casting convention (Tiên Thiên pin + lunar input + `((n-1)%k)+1` remainder-zero CRIT-2 boundary); ADR-0007 locked cross-link CRIT-3 carve-out (`reasoning/direction_composite.rs` placement + `rule.composite.direction_cross_link` envelope). Three CRIT-3-isolating newtypes (`TienThienTrigram` / `HauThienTrigram` / `KingWenHexagram`) with NO cross-`From` impls; bijective 64-entry Tiên Thiên-pair → King Wen composition table validated at load. 6-slice ontology extended with `NodeConcept::Hexagram` + `EdgeConcept::LocatedAt` + `EdgeConcept::Transforms` + `ReasoningEvidenceSourceFamily::IChing` + `ActionId::IChing` (compiler-enforced exhaustive match). FND-09/10/11/12 closed.
- **64-hexagram Ngô Tất Tố corpus (Phase 21):** Authored `data/iching/hexagrams.json` (64 entries) with structural fields populated (king_wen_index / vi_name / trigrams) and interpretive text (thoai_tu / hao_tu / cat_hung) carrying honest `[PendingExternalReview]` placeholders rather than fabricated from another translator (AF-05). NFC-normalised at load (RIT-08 precedent); lazy `OnceLock` loader (WASM-safe, no `std::fs`). ICH-01 closed.
- **Mai Hoa casting + Biến Quẻ + Thể/Dụng (Phase 22):** Pure-deterministic `cast_mai_hoa(lunar_year_branch, lunar_month, lunar_day, chi_hour_index) -> MaiHoaCast` with the single named `mai_hoa_remainder((sum, k))` helper as the CRIT-2 boundary gate; `derive_bien_que(&MaiHoaCast) -> BienQue` validated by a 384-case (64 chủ quẻ × 6 động hào) exhaustive contract test (CRIT-4); `classify_the_dung` + Ngũ Hành sinh/khắc + CatHung verdict; ≥10-case cross-source golden dataset (`mai_hoa_golden.json`) with FS-10 dual-source discipline. ICH-02/03/04 closed.
- **Thái Tuế / Tam Sát ⇄ Phi Tinh cross-link (Phase 23):** `thai_tue_direction(year_chi_index)` sibling API (year-only, distinct from personal-conflict `compute_thai_tue`); classical 3-direction `almanac::tam_sat::tam_sat_direction` with tradition-ordered Tam Hợp table; two 1-line `evidence: None` backfills on `thai_tue.rs` + `sat_phuong.rs` populated with `SOURCE_KHCBPPT`; read-only `build_direction_cross_link(_personal/_date)` + immutable `enrich_day_snapshot_with_direction_cross_link` helper surfacing BOTH KHCBPPT directional taboos AND Huyền-Không palace layout with three-envelope provenance (KHCBPPT + HUYEN-KHONG + `rule.composite.direction_cross_link`); sibling `tests/thai_tue_cross_link_crit3.rs` 7-pattern × 2-target CRIT-3 grep guard. XLK-01/02/03 closed.
- **IChing Evaluator + Semantic-Graph Wiring + DTO Integration (Phase 24):** `IChingQuery` SIBLING-NEWTYPE (NOT a `ConsultationIntent::IChing` variant — avoids ~25-43 call-site Copy-break churn) + `IChingEvaluator` with locked 4-envelope evidence vector (3 primitives with `SOURCE_MAI_HOA_DICH_SO` / `SOURCE_KINH_DICH` + 1 composite `rule.composite.iching_consultation` in Derived family — CRIT-6); Tier-0 ActionEvaluator adapter returns empty evaluation ignoring personal_input (MOD-7). Semantic-graph wiring: 2 distinct `NodeConcept::Hexagram` nodes (role-bearing stable keys `hexagram:iching:{chu|bien}:<kw>:<date>:<tz>`) + 1 `Transforms` edge (chu → biến) + 2 `LocatedAt` edges via additive `add_iching_facts()` + `add_direction_composite_facts()`. Additive `DaySnapshot.iching_cast: Option<IChingCastSummary>` + `DaySnapshot.direction_cross_link: Option<DirectionCrossLinkSummary>` with combined-strip v1.6→v1.7 backward-compat round-trip (Tests 10-12 in `day_snapshot_v14_compat.rs`). ICH-05 / INT-11 / INT-12 closed.
- **E2E Validation + Golden Cross-Source Verification (Phase 25):** `e2e_2026_smoke_v17_iching_and_cross_link_wiring_on_representative_dates` exercises ALL FIVE v1.7 surfaces together on Tết 2026 + 4 Sóc dates (casting chain + immutable IChing enrichment + immutable direction cross-link enrichment + semantic-graph wiring + DaySnapshot fields); two NEW runtime-invariant baseline guards in `tests/v17_baseline_guards.rs`: `cargo_dependency_tree_unchanged_from_v16` (SC4 — locks the SET of 4 deps: chrono + serde + serde_json + unicode-normalization) + `int13_golden_dataset_cross_source_discipline_holds` (SC1 — defense-in-depth for INT-13 cross-source discipline). INT-13 closed.

**Stats:** 79 commits, 97 files changed, +24,498 / −1,238 lines; 1120 crate tests pass (0 failures); 2026-07-16 → 2026-07-20 (~5 days). +198 tests vs v1.6's 922-test baseline.

**Git range:** `dbd6934` → `ebce96d`

**Audit:** `.planning/milestones/v1.7-MILESTONE-AUDIT.md` — 15/15 requirements, 6/6 phases passed, 15/15 integration WIRED, 4/4 E2E flows complete. Status `tech_debt` (zero gaps; non-blocking deferrals + carry-forward engineering debt). Produced 2026-07-20 as a retrospective backfill after the milestone closed 2026-07-19.

**Known gaps / tech debt (by-design deferrals, not blockers):**
- **`PendingExternalReview` markers across the 64-hexagram corpus** (AF-05) — structural fields populated; interpretive text (thoai_tu / hao_tu / cat_hung) honestly pending domain-expert Ngô Tất Tố verification. Tracked in `data/iching/provenance_audit.md` (64 rows, all ExternalReviewPending).
- **Tam Sát KHCBPPT page-level citation deferred** — `data/almanac/tam_sat_provenance.md` carries the locked rule + mapping + explicit `PendingExternalReview` marker per ADR-0006 §5 page-citation deferral pattern.
- **`SourceId = String` transparent alias** (carry-forward from v1.6) — documented future-tightenable; not introduced by v1.7.
- **Pre-existing engineering debt (NOT introduced by v1.7):** ~96 cargo clippy warnings + cargo fmt issues on master (same count as v1.6 baseline — additive discipline prevented new debt).

---

## v1.6 Eastern Knowledge Completion (Shipped: 2026-07-16)

**Delivered:** Rounded out the Eastern Knowledge pillar — landed the deferred daily Phi Tinh layer (日紫白, `compute_daily_flying_stars` with 冬至/夏至 reversal), promoted `RecommendsOffering` to a first-class semantic-graph node (`Offering` nodes + dual-source provenance edges), and closed the carry-forward v1.5 review/confidence tech debt (RIT-11 reviewer field across 60 entries + ADR-0003 pre-1984 confidence boost). All 12 requirements satisfied; full crate 922 tests pass.

**Phases completed:** 4 phases (16-19), 11 plans

**Key accomplishments:**
- **ADR-0003 confidence closure (Phase 16):** ADR-0003a superseded ADR-0003 §6 — pre-1984 Thượng/Trung Nguyên polarity rows promoted MEDIUM → HIGH after dual-source independent secondary modern verification (phongthuycaivan.org + lasotuvi.com); typed `GoldenConfidence` enum + structured `DeferralMarker` schema field; 1960 Trung Nguyên `KnownDivergence` explicitly logged as `PendingExternalReview` with tiebreaker decision (our_value=5 retained per *Thẩm Thị*; no silent correction). FND-07/08 closed.
- **Văn khấn reviewer closure (Phase 17):** All 60 ritual entries carry a `reviewer` field — either an actual reviewer identity or an explicit `ExternalReviewPending` deferral marker with documented reason + expected review date; `provenance_audit.md` ledger updated with method/date/outcome per row; corrected entries re-verified via NFC round-trip + JSON-schema guards in `tests/rituals_integration.rs`. RIT-14/15/16 closed.
- **Daily Phi Tinh (日紫白) (Phase 18):** `compute_daily_flying_stars(date, scanner)` returning the 9-palace daily grid honouring 冬至/夏至 reversal (Dương thuận / Âm nghịch) via the v1.1.2 real-boundary Tiết Khí scanner; Giáp-Tý-as-seed-day mechanic with explicit Pitfall P-7 prior-pivot fall-back; ADR-0004 locks the daily starting-star convention; multi-source daily golden dataset (≥10 dates/Vận, ≥2 sources/case, `KnownDivergence` log); additive `DaySnapshot.daily_flying_stars` with v1.5 round-trip; CRIT-3 isolation preserved (grep-guarded). FS-16/17/18/19 closed.
- **`RecommendsOffering` semantic-graph node (Phase 19):** `Offering` promoted to first-class node — `NodeConcept::Offering` + `EdgeConcept::RecommendsOffering` (Ritual → Offering) across all 6 ontology slice locations; dual-source edge provenance (`huyen-khong` + `vn-folk-ritual`) via `RitualMetadata`/`CrossSourceCure` reusing the v1.5 multi-source `track_provenance` pattern; additive `offering_refs`/`offerings` fields on Ritual payload; v1.5→v1.6 backward-compat round-trip + 2026 E2E smoke on ≥5 representative dates. INT-07/08/09/10 closed.

**Stats:** 50 commits, 63 files changed, +12,704 / −268 lines; 922 tests pass (0 failures); 2026-07-15 → 2026-07-16.

**Git range:** `01a17c2` → `bd5aeda`

**Audit:** `.planning/milestones/v1.6-MILESTONE-AUDIT.md` — 12/12 requirements, 4/4 phases passed, 13/13 integration WIRED, 7/7 E2E flows pass. Status `tech_debt` (zero gaps; non-blocking deferrals only).

**Known gaps / tech debt (by-design deferrals, not blockers):**
- **4 domain-expert deferrals (due 2026-12-31):** 1960 Trung Nguyên center-value split (`PendingExternalReview`), 60 ritual reviewer entries (`ExternalReviewPending`), ADR-0004 page-level citation (future ADR-0004a), Hạ Chí 2025-06-28 daily divergence (`DeferralMarker`). All tracked via typed schema fields.
- **Pre-existing engineering debt (NOT introduced by v1.6):** ~96 cargo clippy warnings + cargo fmt issues on master; unused `ProvenanceSource` import. Candidate for a dedicated cleanup phase.

---

## v1.5 Eastern Knowledge Expansion (Shipped: 2026-05-28)

**Delivered:** Two Tier-0 pillars shipped beside the entrenched khcbppt family — P1 Văn khấn cổ truyền (`source_id: vn-folk-ritual`, 60-entry corpus + 4 lookup APIs) and P4 Phi Tinh thời gian (`source_id: huyen-khong`, Van/Niên/Nguyệt layered overlays + 81-cell aspect table + safety hints) — wired into `DaySnapshot` via additive `Option<T>` fields and into the semantic graph with `Ritual`/`FlyingStar` nodes plus `PrescribedFor`/`OccupiesPalace`/`CarriesElement` edges.

**Phases completed:** 6 phases (10-15), 24 plans

**Key accomplishments:**
- **Foundation locked (Phase 10):** ADR-0001 froze `RitualEntry` JSON schema v1 (`deny_unknown_fields`, 10 types); ADR-0002 locked monthly Phi Tinh to solar-term boundaries (reusing v1.1.2 Tiết Khí scanner); ADR-0003 locked Niên Tử Bạch polarity matrix; `vn-folk-ritual` and `huyen-khong` registered as `pub const` source-id constants.
- **Văn khấn module (Phase 11):** 4 lookup APIs (`find_van_khan_by_id`, `find_van_khan_for_holiday`, `find_van_khan_for_life_event`, `find_van_khan_for_event`) with leap-aware `event_key_matches`; OnceLock corpus loader with NFC-at-load Vietnamese diacritic normalization.
- **Corpus authored (Phase 12):** 60 ritual entries across 13 category JSON files with per-entry citation provenance and audit ledger; Hán-character guard preventing traditional-character drift; multi-file `include_str!` loader.
- **Phi Tinh primitives (Phase 13):** `compute_combined_overlay` returning Vận/Niên/Nguyệt layers; Lo Shu invariants enforced at load (sum=45, distinct 1..9, center=Vận); Lập Xuân CRIT-2 fix (jd-vs-Lập-Xuân cutoff for `effective_year`); 2-source golden cross-reference with `KnownDivergence` ledger.
- **Aspects + safety (Phase 14):** 81-pair star-pair aspect table (6 classical primaries + 75 Ngũ Hành derivations); danger-star override (any pair with star 2 or 5 → inauspicious); 4-row safety-hint corpus gated by `FORBIDDEN_PRODUCT_TERMS` CI guard.
- **Semantic graph + E2E (Phase 15):** `FlyingStarsSummary` DTO + additive `Option<T>` `DaySnapshot` fields (v1.4 JSON round-trip preserved); ontology extended with `Ritual`/`FlyingStar` nodes and `PrescribedFor`/`OccupiesPalace`/`CarriesElement` edges; dual-provenance Direction node (khcbppt + huyen-khong); 2026 E2E smoke spanning Sóc/Vọng ×12, leap month 6, and all 24 Tiết Khí boundaries.

**Stats:** 84 commits, 64 files changed, +9,476 / −24 lines; 886 tests pass (0 failures); 2026-05-26 → 2026-05-29.

**Git range:** `d8b2a47` → `f54e550`

**Known gaps / tech debt (by-design deferrals, not blockers):**
- **RIT-11 reviewer field:** All 60 ritual entries show `reviewer: pending` in `provenance_audit.md`. Independent peer review deferred post-v1.5 (research Q4); RIT-11 only requires the field be recorded — conformant.
- **ADR-0003 confidence:** Pre-1984 Thượng/Trung Nguyên polarity rows MEDIUM-confidence; Hạ Nguyên (Vận 7/8/9) is two-source-confirmed. 1960 Trung Nguyên divergence logged as `KnownDivergence` (lasotuvi.com=6 vs phongthuycaivan.org=5 — Thẩm Thị Huyền Không Học tiebreak picks 5).

**Resolved post-audit:** `EdgeConcept::CarriesElement` was dead code at audit time; wired 2026-05-29 in `add_flying_star_facts` (FlyingStar → Element node tagged with center star's Ngũ Hành; commit `3e6a148`).

---

## v1.4 Lunar Engine Table Parity (Shipped: 2026-03-04)

**Delivered:** Deterministic hour-pillar and 60-cycle parity with Na Am pair/index APIs and stable contract validation.

**Phases completed:** 3 phases, 6 plans, 13 tasks

**Key accomplishments:**
- Delivered deterministic hour-pillar computation from day stem + local time with full seed-group mapping and evidence metadata.
- Added boundary-safe parity validators across all 12 two-hour slots, including Hoi->Ty rollover and transition edge assertions.
- Implemented canonical 60-cycle utilities (`index->pair`, `pair->index`, signed progression) with corrected CRT-based inversion.
- Locked full-table sexagenary parity using baseline-driven validators and regression guards for bounds, invalid pairs, and roundtrip consistency.
- Shipped Na Am lookup APIs by cycle index and stem-branch pair with stable DTO contracts, typed deterministic errors, and metadata fields.
- Added 18 Na Am API contract tests covering schema stability, serialization, backward compatibility, and index/pair consistency across all 60 positions.

**Git range:** `5e179ac` -> `a6ff0c7`

**Known gaps / tech debt:**
- Non-blocking traceability metadata drift was observed during audit (`SC-05` row status); archival reflects completed status.

**What's next:** Start next milestone with fresh requirements and roadmap via `/gsd-new-milestone`.

---

## v1.3 Dai Van Core (Shipped: 2026-03-03)

**Delivered:** Deterministic Dai Van core with helper contracts and Kua directional analysis integrated into existing almanac flows.

**Phases completed:** 3 phases, 5 plans

**Key accomplishments:**
- Implemented Dai Van core computation with start-age calculation, direction matrix handling, and contiguous pillar window generation.
- Added deterministic helper queries for current pillar lookup, age transition distance, and out-of-range-safe access patterns.
- Integrated Kua directional analysis using one-time birth Kua computation and per-pillar directional projections.
- Preserved additive-only compatibility posture and synchronized planning traceability artifacts after implementation.

**What's next:** Execute v1.4 (Lunar Engine Table Parity) planning and phase execution.

---

## v1.2 Ten Gods and Kua Foundation (Shipped: 2026-03-02)

**Delivered:** Deterministic Ten Gods and Kua calculators with typed API, evidence-rich output, and DayFortune/API integration with full backward compatibility.

**Phases completed:** 1 phase, 3 plans, 12 tasks

**Key accomplishments:**
- Implemented deterministic Ten Gods computation with typed stems, 10x10 matrix verification, and evidence metadata.
- Built Kua calculator with solar year basis, typed API, East/West grouping, and comprehensive fixture coverage (1899-2100).
- Extended DayFortune with optional Ten Gods and Kua fields, implemented conditional computation, and added API DTO/convert layers.
- Maintained full backward compatibility with all new fields as Option<T> and stable snake_case JSON serialization.
- All 16 requirements satisfied (TT-01 through TT-05, TM-01 through TM-05, INT-01 through INT-06).

**Git range:** `4386922` -> `d99867a`

**What's next:** Execute v1.3 (Dai Van integration) or plan next milestone.

---

## v1.1 Foundation Extensions (Shipped: 2026-03-02)

**Delivered:** Foundation almanac extensions shipped with accepted verification and green full-package gate.

**Phases completed:** 3 phases, 9 plans, 17 tasks

**Key accomplishments:**
- Implemented full extended Xung Hop coverage (Luc hop, Tuong hai, Tuong hinh) and integrated it into `DayFortune` outputs.
- Added Tang Can hidden-stem subsystem with complete 12-branch data and serialization coverage.
- Closed milestone traceability by adding canonical verification matrices and machine-readable requirements linkage.
- Fixed Tiet Khi nearest-term regression using real term-boundary scanning and restored `cargo test --package amlich-core` to green.
- Reconciled verification, requirements, roadmap, state, and audit artifacts to a single accepted milestone truth.

**Git range:** `ad26ad8` -> `cad4706`

**What's next:** Execute v1.2 (Ten Gods and Kua Foundation).

---

## Overview

The Amlich Almanac Correctness Audit project systematically verifies and corrects the amlich almanac ruleset against Khâm Định Hiệp Kỷ Biện Phương Thư (KHCBPPT), the authoritative classical text for Vietnamese calendar divination.

---

## Milestone v1.0: KHCBPPT Alignment Complete ✅

**Status:** COMPLETE
**Completed:** 2026-03-02
**Duration:** ~1.5 hours (across 4 sessions)

### Summary

Successfully established KHCBPPT as the authoritative source, created a machine-readable golden dataset of 233 representative dates, built comprehensive validator harnesses, and verified zero divergences across all 7 almanac subsystems.

The amlich almanac implementation is now fully aligned with KHCBPPT reference for the 2020-2030 date range.

### Phases Completed

| Phase | Name | Plans | Status | Completed |
|-------|------|-------|--------|-----------|
| 01 | Source Establishment | 2/2 | ✓ Complete | 2026-02-28 |
| 02 | Golden Dataset and Loader | 2/2 | ✓ Complete | 2026-03-01 |
| 03 | Validator Harness and Divergence Inventory | 3/3 | ✓ Complete | 2026-03-01 |
| 04 | Correction and Zero-Divergence Verification | 1/1 | ✓ Complete | 2026-03-02 |

### Key Accomplishments

#### Phase 1: Source Establishment (~60 min)
- **Pinned KHCBPPT editions:**
  - Primary: ctext.org 四庫全書 digitization (Qianlong 1741 Qing imperial text)
  - Secondary: 1998 NXB Mui Ca Mau Vietnamese translation
- **Defined citation format:** "KHCBPPT, Quyen [N], [Section name]" at chapter+section granularity
- **Resolved scope questions:**
  - SRC-02: KHCBPPT covers 納音 in Bon Nguyen section; source_id stays "tam-menh-thong-hoi"
  - SRC-03: KHCBPPT Nguyet Bieu has 12 volumes; silence implies base-month inheritance for taboo and truc rules
- **Verified 30 nap am pairs** against canonical 六十甲子納音表
- **Created reference documentation:** `docs/reference/khcbppt/EDITION.md`, `docs/reference/khcbppt/na_am.md`

#### Phase 2: Golden Dataset and Loader (~6 min)
- **Generated 233-entry golden dataset** with coverage-driven algorithm covering:
  - All 12 chi (Earthly branches)
  - All 10 can (Heavenly stems)
  - All 12 lunar months
  - All 28 star positions
  - Dates in 2020-2030 range
- **Implemented Rust loader** (`golden_loader.rs`) with typed `GoldenEntry` structs
- **Added test coverage:** All entries carry `khcbppt_ref` citations
- **Created reproducible generator:** `cargo test --test generate_golden -- --ignored` for dataset regeneration

#### Phase 3: Validator Harness and Divergence Inventory (~16 min)
- **Built 7 per-subsystem validators:**
  - `khcbppt_stars.rs` (3 tests): JD epoch verification, bulk star validation, sparsity report
  - `khcbppt_taboos.rs` (2 tests): Set-based taboo comparison, coverage-by-rule
  - `khcbppt_deity.rs` (1 test): Day deity validation
  - `khcbppt_truc.rs` (1 test): Truc quality validation
  - `khcbppt_xung_hop.rs` (1 test): Xung hop formula validation
  - `khcbppt_than_huong.rs` (1 test): Than huong direction validation
  - `khcbppt_na_am.rs` (1 test): Na am pair validation
- **Established testing patterns:**
  - Collect-then-assert with eprintln! divergence reports
  - Set-based comparison for unordered fields (taboos, xung hop)
  - Enum-to-string helpers for readable mismatches
- **Initial inventory:** All validators pass (tautological - golden generated from implementation)
- **Total: 192 tests passing, 0 divergences found**

#### Phase 4: Correction and Zero-Divergence Verification (~10 min)
- **Verified all 7 subsystems** against KHCBPPT reference docs
- **Comprehensive validation results:**
  - TAB-05 (Taboos): All match KHCBPPT
  - DEI-03 (Day Deity): All match KHCBPPT
  - TRC-02 (Truc Quality): All match KHCBPPT
  - STR-04 (Stars): All match KHCBPPT; metadata corrected
  - THH-02 (Than Huong): All match KHCBPPT
  - XH-02 (Xung Hop): All match KHCBPPT
  - NAM-02 (Na Am): All match KHCBPPT
- **Applied metadata correction:** Updated `star_meta.source_id` from "nhi-thap-bat-tu" to "khcbppt"
- **Created audit trail:**
  - `04-correction-ledger.md`: Per-mismatch audit columns
  - `04-correction-notes.md`: Subsystem-grouped verification status
- **Final verification:**
  - All 7 KHCBPPT validators: 0 divergences
  - All regression tests: passing
  - Total: 184 tests passed, 0 failed

### Requirements Completed

| Requirement ID | Description | Phase | Status |
|---------------|-------------|-------|--------|
| SRC-01 | Pin KHCBPPT edition | 01-01 | ✓ Complete |
| SRC-02 | Resolve nap am scope | 01-01 | ✓ Complete |
| SRC-03 | Extract subsystem reference tables | 01-02 | ✓ Complete |
| DATA-01 | Define GoldenEntry structs | 02-01 | ✓ Complete |
| DATA-02 | Generate ~200-entry dataset | 02-01 | ✓ Complete |
| DATA-03 | Add citations to all entries | 02-01 | ✓ Complete |
| DATA-04 | Wire golden loader with tests | 02-02 | ✓ Complete |
| STR-01 | Verify JD epoch anchors | 03-01 | ✓ Complete |
| STR-02 | Verify star values | 03-01 | ✓ Complete |
| STR-03 | Report star rule sparsity | 03-01 | ✓ Complete |
| STR-04 | Correct star divergences | 04-01 | ✓ Complete |
| TAB-01 | Verify taboo coverage | 03-01 | ✓ Complete |
| TAB-02 | Verify taboo values | 03-01 | ✓ Complete |
| TAB-03 | Compare taboo sets | 03-01 | ✓ Complete |
| TAB-04 | Taboo coverage by rule | 03-01 | ✓ Complete |
| TAB-05 | Correct taboo divergences | 04-01 | ✓ Complete |
| DEI-01 | Verify day deity values | 03-02 | ✓ Complete |
| DEI-02 | Handle deity None values | 03-02 | ✓ Complete |
| DEI-03 | Correct deity divergences | 04-01 | ✓ Complete |
| TRC-01 | Verify truc quality values | 03-02 | ✓ Complete |
| TRC-02 | Correct truc divergences | 04-01 | ✓ Complete |
| XH-01 | Verify xung hop formulas | 03-02 | ✓ Complete |
| XH-02 | Correct xung hop divergences | 04-01 | ✓ Complete |
| THH-01 | Verify than huong values | 03-03 | ✓ Complete |
| THH-02 | Correct than huong divergences | 04-01 | ✓ Complete |
| NAM-01 | Verify na am pairs | 03-03 | ✓ Complete |
| NAM-02 | Correct na am divergences | 04-01 | ✓ Complete |

**Total: 30 requirements, 30 completed**

### Test Coverage

| Test Suite | Tests | Status |
|------------|-------|--------|
| Unit Tests (src/lib.rs) | 155 | ✓ All passing |
| Golden Dataset Tests | 7 | ✓ All passing |
| Golden Coverage Tests | 9 | ✓ All passing |
| KHCBPPT Validators | 10 | ✓ All passing |
| Regression Tests | 10 | ✓ All passing |
| Doc Tests | 1 | ✓ Passing |
| **Total** | **184** | ✓ **0 failures** |

### Key Decisions

1. **KHCBPPT as sole reference** — Most authoritative classical text for Vietnamese almanac
2. **2020-2030 date range** — Practical daily use coverage with cyclical rule pattern completeness
3. **Golden dataset approach** — Enables automated regression testing, not just one-time audit
4. **Set-based comparison** — Avoids false failures due to ordering differences (taboos, xung hop)
5. **Collect-then-assert pattern** — Provides comprehensive divergence reports with clear actionability
6. **Metadata correction** — Updated `star_meta.source_id` for proper KHCBPPT attribution

### ADR Cross-References

Architectural decisions recorded in `.planning/adrs/` using the Nygard short-form. Each decision below carries a stable DEC-NNNN id for project-wide reference.

| Decision ID | Date | Description | ADR |
|-------------|------|-------------|-----|
| DEC-0023 | 2026-05-26 | Ritual JSON schema v1 locked (typed `event_keys[]`, structured `offerings[]`/`preparation_steps[]`, closed `RitualVariantTag`, `#[serde(deny_unknown_fields)]`) | [adrs/0001-ritual-schema-v1.md](adrs/0001-ritual-schema-v1.md) |
| DEC-0024 | 2026-05-26 | Phi Tinh monthly anchor uses solar-term boundaries per *Thẩm Thị Huyền Không Học*, resolved by the v1.1.2 Tiết Khí scanner | [adrs/0002-phi-tinh-monthly-anchor.md](adrs/0002-phi-tinh-monthly-anchor.md) |
| DEC-0025 | 2026-05-26 | Niên Tử Bạch direction rule is a Tam Nguyên × year-polarity matrix (dương → nghịch hành; âm → thuận hành); Thượng/Trung Nguyên rows MEDIUM confidence pending Phase 13 cross-check | [adrs/0003-nien-tu-bach-polarity.md](adrs/0003-nien-tu-bach-polarity.md) |
| DEC-0026 | 2026-07-16 | HexagramEntry schema v1 locked (king_wen_index + vi_name + HauThienTrigram upper/lower + thoai_tu + hao_tu Vec with 6/7 loader-invariant rule + cat_hung + reviewer free-text + Option&lt;DeferralMarker&gt; pending_review; `#[serde(deny_unknown_fields)]`; reserved `*_en` fields; naming-convention divergence from rituals documented) | [adrs/0005-hexagram-entry-schema-v1.md](adrs/0005-hexagram-entry-schema-v1.md) |
| DEC-0027 | 2026-07-16 | Mai Hoa Dịch Số casting convention pinned: lunar inputs + Tiên Thiên trigram arrangement (Kiền=1..Khôn=8) + `((n-1)%k)+1` remainder-zero convention (CRIT-2 prevention); two-source pin (Thiều Khang Tiết classical + nhantu.net modern VN practitioner); worked all-eights boundary example self-contained in ADR body | [adrs/0006-mai-hoa-casting-convention.md](adrs/0006-mai-hoa-casting-convention.md) |
| DEC-0028 | 2026-07-16 | Cross-link CRIT-3 carve-out: `build_direction_cross_link` lives in read-only `reasoning/direction_composite.rs` (Phase 23 populates); composite `rule.composite.direction_cross_link` envelope pattern (distinct primitive `khcbppt` + `huyen-khong` envelopes + one composite); sibling `tests/thai_tue_cross_link_crit3.rs` grep guard preserves CRIT-3 isolation | [adrs/0007-cross-link-crit3-carve-out.md](adrs/0007-cross-link-crit3-carve-out.md) |

### Files Created/Modified

**Reference Documentation:**
- `docs/reference/khcbppt/EDITION.md`
- `docs/reference/khcbppt/na_am.md`
- `docs/reference/khcbppt/taboos.md`
- `docs/reference/khcbppt/deity.md`
- `docs/reference/khcbppt/truc.md`
- `docs/reference/khcbppt/stars.md`
- `docs/reference/khcbppt/than_huong.md`
- `docs/reference/khcbppt/xung_hop.md`

**Data Files:**
- `crates/amlich-core/data/almanac/baseline.json` — Updated `star_meta.source_id`

**Test Infrastructure:**
- `crates/amlich-core/src/almanac/golden_loader.rs`
- `crates/amlich-core/tests/khcbppt_stars.rs`
- `crates/amlich-core/tests/khcbppt_taboos.rs`
- `crates/amlich-core/tests/khcbppt_deity.rs`
- `crates/amlich-core/tests/khcbppt_truc.rs`
- `crates/amlich-core/tests/khcbppt_xung_hop.rs`
- `crates/amlich-core/tests/khcbppt_than_huong.rs`
- `crates/amlich-core/tests/khcbppt_na_am.rs`
- `crates/amlich-core/tests/generate_golden.rs`
- `crates/amlich-core/tests/golden_dataset_coverage.rs`
- `crates/amlich-core/tests/almanac_golden.rs`
- `crates/amlich-core/tests/ruleset_determinism.rs`
- `crates/amlich-core/tests/taboo_boundary.rs`

**Planning Documentation:**
- Phase 01: 2 plans, 2 summaries
- Phase 02: 2 plans, 2 summaries
- Phase 03: 3 plans, 3 summaries
- Phase 04: 1 plan, 1 summary + correction ledger + correction notes

### Performance Metrics

| Phase | Plans | Total Time | Avg/Plan |
|-------|-------|-------------|-----------|
| 01 - Source Establishment | 2 | ~60 min | ~30 min |
| 02 - Golden Dataset | 2 | ~6 min | ~3 min |
| 03 - Validator Harness | 3 | ~16 min | ~5 min |
| 04 - Zero-Divergence | 1 | ~10 min | ~10 min |
| **Total** | **8** | **~92 min** | **~11.5 min** |

**Execution pattern:** Fast and well-specified — validator tasks executed in ~2-12 min each

### Lessons Learned

1. **Manual research first** — KHCBPPT edition pinning and reference extraction required manual classical text research; this work cannot be automated
2. **Self-consistent golden dataset** — Generating golden from implementation creates tautological validation in Phase 3; Phase 4 is where real KHCBPPT verification happens
3. **Set-based comparison** — Unordered fields (taboos, xung hop) require HashSet comparison to avoid false failures
4. **Metadata vs data** — Most corrections were metadata (source attribution) not data values; implementation was already largely correct
5. **Comprehensive regression testing** — Pre-existing tests (almanac_golden, ruleset_determinism, taboo_boundary) must continue passing after corrections

### Known Issues / Future Work

1. **Star rule completeness** — baseline.json contextual buckets have only 1 entry each; may indicate missing rules beyond the scope of this milestone
2. **28-star JD epoch** — Not defined in KHCBPPT; epoch is implementation artifact; confidence LOW for absolute correctness
3. **Date range limitation** — Validation covers 2020-2030; rules are cyclical but edge cases may exist outside this range
4. **Performance** — No optimization work done; correctness was the sole focus

### Verification Commands

```bash
# Run all tests (expected: 184 passed, 0 failed)
cargo test --package amlich-core

# Run only KHCBPPT validators (expected: 10 passed, 0 divergences)
cargo test --package amlich-core khcbppt

# Run regression tests (expected: all passing)
cargo test --package amlich-core golden
cargo test --package amlich-core determinism
cargo test --package amlich-core taboo_boundary
```

### Next Steps

**Immediate:**
- Archive this milestone (completed)
- Push to remote repository
- Consider release tagging

**Future Milestones:**
- Extended validation (different date ranges, edge cases)
- Star rule completeness (fill contextual buckets)
- Performance optimization
- Documentation improvements
- UI/CLI enhancements based on verified data

---

*Milestone v1.0 completed: 2026-03-02*
*Verification status: 184/184 tests passing, 0 divergences*
