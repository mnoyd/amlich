# Roadmap: Amlich Almanac Correctness Audit

## Milestones

- ✅ **v1.1 Foundation Extensions** — shipped 2026-01 (Phases 1-3 + v1.1.1/v1.1.2 hotfixes), archived in [`milestones/v1.1-ROADMAP.md`](milestones/v1.1-ROADMAP.md)
- ✅ **v1.2 Ten Gods + Kua Foundation** — shipped 2026-02 (Phases v1.2-01..03), archived in [`milestones/v1.2-ROADMAP.md`](milestones/v1.2-ROADMAP.md)
- ✅ **v1.4 Lunar Engine Table Parity** — shipped 2026-03-04 (Phases 7-9, 6/6 plans), archived in [`milestones/v1.4-ROADMAP.md`](milestones/v1.4-ROADMAP.md)
- ✅ **v1.5 Eastern Knowledge Expansion** — shipped 2026-05-28 (Phases 10-15, 24/24 plans), archived in [`milestones/v1.5-ROADMAP.md`](milestones/v1.5-ROADMAP.md)
- 🚧 **v1.6 Eastern Knowledge Completion** — started 2026-07-15 (Phases 16-19)

## Current Focus

- v1.6 closes v1.5 tech debt (RIT-11 reviewer field + ADR-0003 pre-1984 confidence) and rounds out the Eastern Knowledge pillar with the deferred daily Phi Tinh layer (日紫白) and the `RecommendsOffering` semantic-graph node promotion.
- Reuse v1.5 patterns: schema-lock-before-corpus, single-commit RED→GREEN, audit-as-decisive-source, external-crate black-box tests (`crates/<crate>/tests/<feature>_integration.rs`).

---

# v1.6 Eastern Knowledge Completion

**Milestone goal:** Round out the Eastern Knowledge pillar — add the deferred daily Phi Tinh layer (日紫白), promote `RecommendsOffering` to a first-class semantic-graph node, and close the v1.5 review/confidence tech debt (RIT-11 reviewer field + ADR-0003 pre-1984 confidence).

**Phase numbering:** Continues from v1.5 (last phase 15). v1.6 starts at Phase 16.

**Depth:** quick (compressed; 4 phases for 12 requirements).

**Coverage:** 12 / 12 requirements mapped ✓

## Phases

- [x] **Phase 16: Foundation — ADR-0003 Confidence Closure** - Promote pre-1984 Thượng/Trung Nguyên polarity rows MEDIUM → HIGH after external cross-check; resolve or explicitly defer the 1960 Trung Nguyên `KnownDivergence`.
- [x] **Phase 17: Văn khấn Reviewer Closure** - Populate the `reviewer` field on every ritual entry (identity OR explicit `ExternalReviewPending` marker); re-verify corrected entries pass existing JSON-schema + NFC guards.
- [ ] **Phase 18: Daily Phi Tinh (日紫白)** - Land the deferred daily layer: `compute_daily_flying_stars` with 冬至/夏至 reversal, ADR-0004 daily starting-star convention, multi-source daily golden dataset, additive `DaySnapshot.daily_flying_stars` field.
- [ ] **Phase 19: `RecommendsOffering` Semantic-Graph Node + v1.6 Integration** - Promote offerings to first-class nodes (Ritual → Offering via `RecommendsOffering` edge with dual-source provenance); v1.5→v1.6 backward-compat round-trip + 2026 E2E smoke.

## Phase Details

### Phase 16: Foundation — ADR-0003 Confidence Closure

**Goal**: User-of-ADR-0003 can rely on pre-1984 Thượng/Trung Nguyên polarity rows being HIGH confidence with traceable external cross-check evidence, and the 1960 Trung Nguyên `KnownDivergence` is either resolved with source attribution or explicitly logged as `PendingExternalReview` with reason + tiebreaker decision.

**Depends on**: Nothing (first phase of milestone; closes carry-forward v1.5 tech debt item per MILESTONES.md §v1.5 gaps).

**Requirements**: FND-07 ✅ (closed in 16-01), FND-08 (next: 16-02)

**Success Criteria** (what must be TRUE):
  1. ✅ A reader can open ADR-0003a and find pre-1984 Thượng/Trung Nguyên polarity rows promoted from MEDIUM to HIGH confidence, with cross-check trail via dual-source independent secondary modern verification (phongthuycaivan.org + lasotuvi.com / phongthuyso.vn); *Thẩm Thị Huyền Không Học* retained as classical tiebreaker (no additional classical authority claimed).
  2. ✅ A reader of the golden dataset's `known_divergences` can find the 1960 Trung Nguyên case carrying an explicit `PendingExternalReview` deferral marker with documented reason + expected review date (Plan 16-02 adds the structured `DeferralMarker` schema field; ADR-0003a §4 locks the narrative disposition).
  3. ✅ A reader can find ADR-0003a (the superseding decision) recording both the disposition of the 1960 divergence (PendingExternalReview, our_value=5 retained per *Thẩm Thị* tiebreaker, no silent correction) and the high-confidence upgrade path (dual-source independent secondary modern verification).
  4. ✅ A cargo test run on the Phi Tinh golden dataset (`tests/fengshui_invariants.rs`) passes with the upgraded confidence annotations (11/11 tests; Test F added for FND-07 gate, Test G added for FND-08 gate); no regression in any v1.5 KHCBPPT or `vn-folk-ritual` test (888/888 tests pass).

**Plans**: 2 plans
- [x] 16-01-PLAN.md — ADR-0003a authored + cross-check citation trail collected + golden dataset confidence annotations updated (FND-07) — commits c76e741 (docs) + 3d3d565 (feat)
- [x] 16-02-PLAN.md — 1960 Trung Nguyên `KnownDivergence` disposition (`PendingExternalReview` deferral) + structured `DeferralMarker` schema field + tests/fengshui_invariants.rs gate (FND-08) — commits e504fe4 (feat) + 424010c (docs)

### Phase 17: Văn khấn Reviewer Closure

**Goal**: User-of-corpus can find every ritual entry carries a `reviewer` field — either an actual reviewer identity (name + date + outcome) or an explicit `ExternalReviewPending` deferral marker with documented reason and expected review date — with a complete audit record and corrected entries re-verified against their cited source.

**Depends on**: Nothing (independent editorial work; v1.5 schema already locked per ADR-0001, so no JSON schema changes required — reviewer info lives in the audit ledger as the canonical record per "no schema changes expected" guidance).

**Requirements**: RIT-14, RIT-15, RIT-16

**Success Criteria** (what must be TRUE):
  1. A reader can open `data/rituals/provenance_audit.md` and find one row per `ritual_id` (all 60 entries) carrying a `reviewer` value that is one of: an actual reviewer identity (name + date + outcome), or an explicit `ExternalReviewPending` marker with documented reason + expected review date.
  2. A reader can find on each row: reviewer identity (or deferral marker), method-of-review (`independent-peer` / `cross-source` / `desk-check`), date reviewed, and outcome (`confirmed` / `corrected` / `disputed`); the `pending` placeholder used in v1.5 is no longer present.
  3. A caller can filter or count entries by review outcome (`confirmed` / `corrected` / `disputed` / `ExternalReviewPending`) and get stable counts matching the audit ledger (no drift between corpus and ledger).
  4. Any entry whose review outcome was `corrected` has its `invocation_text_vi` re-verified against the cited `original_citation`, and the corrected entry passes the existing `RitualEntry` JSON-schema + NFC-at-load guards — verifiable by extending the existing `tests/rituals_integration.rs` to enumerate corrected entries and assert each passes schema + NFC round-trip.

**Plans**: 2 plans
- [x] 17-01-PLAN.md — Audit-of-record pass: identify reviewer (or `ExternalReviewPending`) for each of the 60 entries; update `provenance_audit.md` with method/date/outcome per row (RIT-14, RIT-15) — commit 1777666 (docs)
- [x] 17-02-PLAN.md — Corrected-entry `invocation_text_vi` re-verification + NFC round-trip guard test in `tests/rituals_integration.rs` (RIT-16) — commits 57496f7 + 0c3d483 (feat)

### Phase 18: Daily Phi Tinh (日紫白)

**Goal**: User can call `compute_daily_flying_stars(date, term_scanner)` to get the 9-palace daily grid with 冬至/夏至 reversal semantics, find a documented ADR capturing the daily starting-star convention, query a multi-source daily golden dataset, and observe daily charts in `DaySnapshot` via an additive field — all without breaking CRIT-3 isolation.

**Depends on**: Nothing (independent; reuses v1.5 `huyen-khong` overlay + aspect machinery + v1.1.2 Tiết Khí scanner per CRIT-2 / ADR-0002 boundary discipline).

**Requirements**: FS-16, FS-17, FS-18, FS-19

**Success Criteria** (what must be TRUE):
  1. A caller can invoke `compute_daily_flying_stars(date: NaiveDate, term_scanner: &TietKhiScanner) -> DailyFlyingStarLayout` and receive a 9-palace daily grid honouring 冬至/夏至 reversal semantics; boundary semantics are always computed via the v1.1.2 real-Tiết-Khí scanner (no naïve `year` arithmetic — covered by an explicit grep/wrapper-test guard).
  2. A reader can open ADR-0004 and find: which year's annual chart seeds the daily count, how the 冬至/夏至 pivot reverses the forward sequence, a chapter+page citation in *Thẩm Thị Huyền Không Học*, and a list of alternative conventions considered with reasons for the chosen one.
  3. A reader can find a daily-chart golden dataset (extending `data/almanac/flying_stars_golden.json` with `kind: "daily"` cases, or a new `data/almanac/flying_stars_daily_golden.json`) with ≥ 10 reference dates per Vận (7/8/9), ≥ 2 independent classical sources per case, *Thẩm Thị Huyền Không Học* as tiebreaker, and any source disagreements logged as `KnownDivergence` (not silently corrected).
  4. A caller can deserialize a v1.5 `DaySnapshot` JSON (with `flying_stars` but no `daily_flying_stars`) into a v1.6 `DaySnapshot` struct and re-serialize without unexpected fields; the v1.6 struct has an additive `daily_flying_stars: Option<DailyFlyingStarLayout>` field with `#[serde(default, skip_serializing_if = "Option::is_none")]` — verifiable by an extension of `tests/day_snapshot_v14_compat.rs` (or a v1.6 round-trip variant).
  5. The new `daily_flying_stars` path does NOT introduce `FlyingStar` or `DailyFlyingStar` into `interaction/direction_merge.rs`; CRIT-3 isolation is preserved and grep-verified (`tests/source_id_guard.rs` or a dedicated grep-test).

**Plans**: 4 plans
- [x] 18-01-PLAN.md — ADR-0004 daily starting-star convention + `DailyFlyingStarLayout` type stub in `almanac/fengshui/types.rs` (FS-17) — commits b2265eb (docs) + a593a13 (feat)
- [ ] 18-02-PLAN.md — `compute_daily_flying_stars` algorithm with 冬至/夏至 reversal via v1.1.2 Tiết Khí scanner (FS-16)
- [ ] 18-03-PLAN.md — Daily golden dataset (≥ 10 dates per Vận, ≥ 2 sources per case, `KnownDivergence` log) + tests/fengshui_daily_integration.rs (FS-18)
- [ ] 18-04-PLAN.md — `DaySnapshot.daily_flying_stars: Option<DailyFlyingStarLayout>` additive field + v1.5 fixture round-trip test + CRIT-3 grep guard refresh (FS-19)

### Phase 19: `RecommendsOffering` Semantic-Graph Node + v1.6 Integration

**Goal**: User-of-semantic-graph can find `Offering` nodes connected to `Ritual` nodes via `RecommendsOffering` edges carrying rationale + source provenance; edges originating from a non-ritual tradition (e.g., a Huyền Không element cure surfaced inside a ritual) carry dual-source provenance via the v1.5 multi-source dedup pattern; a v1.5→v1.6 round-trip test passes and the 2026 E2E smoke exercises daily + annual fields together.

**Depends on**: Phase 18 (FS-19's `daily_flying_stars` field must exist for the round-trip test to exercise both old and new DTO fields).

**Requirements**: INT-07, INT-08, INT-09, INT-10

**Success Criteria** (what must be TRUE):
  1. A reader of the ontology can find a new `NodeConcept::Offering` variant and a new `EdgeConcept::RecommendsOffering` (Ritual → Offering) edge concept, both with exhaustive matches enforced by the compiler across all six ontology slice locations; the `OfferingRef { offering_id: String, name_vi: String, name_en: Option<String>, source_id: SourceId }` identity type is locked before any builder code emits `Offering` nodes.
  2. A reader of the `Ritual` semantic-graph node payload can find an additive `offering_refs: Option<Vec<OfferingRef>>` field (preferred path) coexisting with the legacy `offerings: Option<Vec<String>>` flat-string summary field for backward compatibility — both `#[serde(default, skip_serializing_if = "Option::is_none")]`.
  3. A semantic-graph reader can find a `RecommendsOffering` edge carrying dual-source provenance (both `huyen-khong` and `vn-folk-ritual`) where an offering reference originates in a non-ritual tradition surfaced inside a ritual; the multi-source Direction-node dedup logic from v1.5 (`add_flying_star_facts` + `add_ritual_facts`) is reused/extended (single source of dedup truth — no parallel implementation).
  4. A caller can load a v1.5 JSON fixture (with `flying_stars` populated, no `daily_flying_stars`, no `offering_refs`) into v1.6 structs and re-serialize without unexpected fields — verifiable by an extension of `tests/day_snapshot_v14_compat.rs` to the v1.6 surface (or a sibling `tests/day_snapshot_v15_compat.rs`).
  5. The end-to-end 2026 calendar smoke test (extension of `tests/integration_2026_smoke.rs`) passes on ≥ 5 representative dates that exercise BOTH the existing annual/monthly `flying_stars` fields AND the new `daily_flying_stars` field, with the `Offering`/`RecommendsOffering` graph wiring verified for any day that surfaces a non-ritual-origin offering.

**Plans**: 3 plans
- [ ] 19-01-PLAN.md — Schema-first: `OfferingRef` struct + additive `offering_refs` on `Ritual` semantic-graph node payload (coexists with legacy `offerings` flat-string field for BC) (INT-08)
- [ ] 19-02-PLAN.md — Ontology: `NodeConcept::Offering` + `EdgeConcept::RecommendsOffering` across all six slice locations + dual-source edge provenance builder extending v1.5 multi-source dedup logic (INT-07, INT-09)
- [ ] 19-03-PLAN.md — v1.5→v1.6 backward-compat round-trip test + 2026 E2E smoke (≥ 5 dates exercising daily + annual + `Offering`/`RecommendsOffering` wiring) (INT-10)

## Progress Table

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 16. Foundation — ADR-0003 Confidence Closure | 2/2 | Complete    | 2026-07-15 |
| 17. Văn khấn Reviewer Closure | 2/2 | Complete    | 2026-07-15 |
| 18. Daily Phi Tinh (日紫白) | 1/4 | In Progress|  |
| 19. `RecommendsOffering` Semantic-Graph Node + v1.6 Integration | 0/3 | Not started | - |

## Requirement Coverage

| Phase | Requirements | Count |
|-------|--------------|-------|
| 16 | FND-07, FND-08 | 2 |
| 17 | RIT-14, RIT-15, RIT-16 | 3 |
| 18 | FS-16, FS-17, FS-18, FS-19 | 4 |
| 19 | INT-07, INT-08, INT-09, INT-10 | 4 |
| **Total** | | **12 / 12** ✓ |

## Cross-Cutting Constraints (carry-forward from v1.5)

- **Additive-only DTOs** — all new fields `Option<T>` with `#[serde(default, skip_serializing_if = "Option::is_none")]` (v1.2 precedent, re-validated in v1.5 INT-05; re-validated again here as INT-10).
- **Source-id discipline** — every new module declares `pub const SOURCE_*` for its tradition; provenance call-sites never use string literals (CI-enforced by `tests/source_id_guard.rs`; extend guard if v1.6 introduces a new tradition).
- **CRIT-3 isolation** — `FlyingStar` (and the new `DailyFlyingStarLayout`) remain palace-layout descriptors; never wired into `interaction/direction_merge.rs` (`khcbppt`-family directional logic stays disjoint from `huyen-khong`).
- **Tiết Khí scanner reuse** — daily boundary semantics reuse the v1.1.2 real-boundary scanner; no naïve year arithmetic (Phase 18 explicitly guards this).
- **Audit-as-decisive-source** — `provenance_audit.md` and ADR narrative are the canonical records for confidence + reviewer disposition (Phase 16 ADR-0003a + Phase 17 ledger).
- **Schema-lock before corpus/algorithm** — `DailyFlyingStarLayout` stub (Phase 18-01) and `OfferingRef` type (Phase 19-01) precede builder emission.

---

*Last updated: 2026-07-15 — v1.6 roadmap drafted (Phases 16-19, 11 plans, 12/12 requirements mapped).*
