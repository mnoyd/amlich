# Roadmap: Amlich Almanac Correctness Audit

## Milestones

- ✅ **v1.1 Foundation Extensions** — shipped 2026-01 (Phases 1-3 + v1.1.1/v1.1.2 hotfixes), archived in [`milestones/v1.1-ROADMAP.md`](milestones/v1.1-ROADMAP.md)
- ✅ **v1.2 Ten Gods + Kua Foundation** — shipped 2026-02 (Phases v1.2-01..03), archived in [`milestones/v1.2-ROADMAP.md`](milestones/v1.2-ROADMAP.md)
- ✅ **v1.4 Lunar Engine Table Parity** — shipped 2026-03-04 (Phases 7-9, 6/6 plans), archived in [`milestones/v1.4-ROADMAP.md`](milestones/v1.4-ROADMAP.md)
- ✅ **v1.5 Eastern Knowledge Expansion** — shipped 2026-05-28 (Phases 10-15, 24/24 plans), archived in [`milestones/v1.5-ROADMAP.md`](milestones/v1.5-ROADMAP.md)
- ✅ **v1.6 Eastern Knowledge Completion** — shipped 2026-07-16 (Phases 16-19, 11/11 plans), archived in [`milestones/v1.6-ROADMAP.md`](milestones/v1.6-ROADMAP.md)
- 🚧 **v1.7 Kinh Dịch (I-Ching Divination)** — started 2026-07-16 (Phases 20-25)

## Current Focus

- v1.7 adds the P2 Kinh Dịch pillar — Mai Hoa Dịch Số casting + 64-hexagram Ngô Tất Tố lookup — as a new Tier-0 reasoning capability, plus the Thái Tuế / Tam Sát ⇄ Phi Tinh read-only directional cross-link (carry-forward "should-have" from v1.5 research).
- Reuse v1.5/v1.6 patterns: schema-lock-before-corpus (CRIT-1 × 7 amplification — 64 hexagrams × ~7 text fields = 448 corpus fields), additive `Option<T>` DTOs, sibling-newtype over closed-enum extension, dual-source composite-envelope provenance, CRIT-3 isolation (no `FlyingStar` in `interaction/direction_merge.rs`).
- Two parallel tracks after Foundation: IChing pillar (corpus → casting → evaluator+wiring) and Thái Tuế cross-link (XLK-01..03), merging at the evaluator+wiring phase.

---

# v1.7 Kinh Dịch (I-Ching Divination)

**Milestone goal:** Add the P2 Kinh Dịch pillar — Mai Hoa Dịch Số casting + 64-hexagram lookup — as a new Tier-0 reasoning capability, plus the Thái Tuế / Tam Sát ⇄ Phi Tinh directional cross-link. Both ship as additive, schema-locked surfaces with no new crate dependencies and CRIT-3 isolation preserved.

**Phase numbering:** Continues from v1.6 (last phase 19). v1.7 starts at Phase 20.

**Depth:** quick (compressed; 6 phases for 15 requirements — mirrors v1.6's 4-phases-for-12-reqs density with one extra split for the long-pole 64-hexagram corpus epic).

**Coverage:** 15 / 15 requirements mapped ✓

## Phases

- [x] **Phase 20: Foundation — Schema Lock + Source IDs + ADRs + Ontology** - Lock the IChing + cross-link foundation: register `SOURCE_KINH_DICH` / `SOURCE_MAI_HOA_DICH_SO`, accept ADR-0005/0006/0007, lock `HexagramEntry` schema with the three trigram/hexagram newtypes + the 64-entry Tiên Thiên → King Wen composition table, extend the 6-slice ontology with Hexagram/LocatedAt/Transforms + IChing enum variants. (BLOCKING) (completed 2026-07-15)
- [x] **Phase 21: IChing Corpus + Loader** - Author the 64-hexagram Ngô Tất Tố corpus with reviewer signatures + `PendingExternalReview` gaps; lazy `OnceLock` loader consuming the locked Phase 20 schema. (completed 2026-07-16)
- [x] **Phase 22: Mai Hoa Casting + Biến Quẻ + Thể/Dụng** - Pure-deterministic `cast_mai_hoa` + biến quẻ derivation + Thể/Dụng classification; 384-case biến quẻ contract test + ≥10 cross-source golden cases. (Critical path; parallel to Phase 23.) Completed 2026-07-16.
- [x] **Phase 23: Thái Tuế / Tam Sát ⇄ Phi Tinh Cross-Link** - Directional Thái Tuế + classical 3-direction Tam Sát module (both `source_id: khcbppt`) + read-only `build_direction_cross_link` composite with CRIT-3 isolation grep-guarded. (Parallel to Phases 21-22.) Completed 2026-07-16.
- [~] **Phase 24: IChing Evaluator + Semantic-Graph Wiring + DTO Integration** - `IChingQuery` sibling newtype + `IChingEvaluator` with per-step source_id envelopes + composite; Hexagram/LocatedAt/Transforms semantic-graph wiring; additive `DaySnapshot.iching_cast` + `direction_cross_link` fields with v1.6→v1.7 round-trip. (Plans 24-01 + 24-02 complete 2026-07-16 — ICH-05 closed + INT-11 closed + INT-12 additive field in place; Plan 24-03 remaining for INT-12 full close.)
- [ ] **Phase 25: E2E Validation + Golden Cross-Source Verification** - ≥10 IChing golden casting cases × ≥2 sources, 2026 E2E smoke exercising the full v1.7 surface, zero regressions on v1.6 tests, no new crate deps.

## Phase Details

### Phase 20: Foundation — Schema Lock + Source IDs + ADRs + Ontology

**Goal**: User-of-foundation can find the v1.7 IChing pillar and cross-link fully scaffolded at the type/ADR/ontology level BEFORE any of the 64 corpus entries are authored — source IDs registered and CI-guarded, ADRs accepted, schema locked with a passing 1-entry serde round-trip probe, the typed trigram/hexagram newtype boundary enforced by the compiler, and the 6-slice ontology extended.

**Depends on**: Nothing (first phase of milestone; schema-lock-first is CRIT-1 × 7 prevention per the v1.5 lesson).

**Requirements**: FND-09, FND-10, FND-11, FND-12

**Success Criteria** (what must be TRUE):
  1. A reader can find `SOURCE_KINH_DICH = "kinh-dich"` and `SOURCE_MAI_HOA_DICH_SO = "mai-hoa-dich-so"` registered as `pub const` in `sources.rs`, with `tests/source_id_guard.rs::FORBIDDEN_LITERALS` extended so bare literals at provenance call-sites fail CI (DEC-0023 discipline).
  2. A reader can find three accepted ADRs: ADR-0005 (`HexagramEntry` schema v1 with `deny_unknown_fields`), ADR-0006 (Mai Hoa casting convention: Tiên Thiên arrangement pinned to a Thiệu Khang Tiết page reference, lunar input, `((n-1)%k)+1` remainder-zero convention), and ADR-0007 (cross-link CRIT-3 carve-out: read-only `reasoning/` placement + composite `rule.composite.direction_cross_link` envelope).
  3. A reader can compile a 1-entry `HexagramEntry` serde round-trip probe and observe it passes BEFORE any of the 64 corpus entries are authored (CRIT-1 schema-lock-first); the three newtypes `TienThienTrigram(u8)` / `HauThienTrigram(u8)` / `KingWenHexagram(u8)` carry NO `From` impl between them (CRIT-3 prevention), and the 64-entry Tiên Thiên-pair → King Wen composition table validates at load.
  4. A reader of the 6-slice ontology can find `NodeConcept::Hexagram`, `EdgeConcept::LocatedAt`, `EdgeConcept::Transforms` extended across all six slice locations (compiler-enforced exhaustive match with no `#[non_exhaustive]` escape), plus `ReasoningEvidenceSourceFamily::IChing` and `ActionId::IChing` enum variants.

**Plans**: 3 plans (all Wave 1, parallel — no file conflicts)

Plans:
- [x] 20-01-PLAN.md — Registration & Decisions: SOURCE_* consts + source_id_guard + ADRs 0005/0006/0007 + MILESTONES DEC rows (FND-09, FND-10)
- [x] 20-02-PLAN.md — IChing Schema Lock: 3 trigram/hexagram newtypes + HexagramEntry + bijective 64-entry composition table + 1-entry serde probe (FND-11)
- [x] 20-03-PLAN.md — Ontology 6-Slice Extension: Hexagram node + LocatedAt/Transforms edges + IChing enum variants (FND-12)

### Phase 21: IChing Corpus + Loader

**Goal**: User-of-corpus can load the 64-hexagram Ngô Tất Tố corpus via a lazy `OnceLock` loader and look up any hexagram by King Wen index, with every entry reviewer-signed and any Ngô Tất Tố source gaps surfaced as `PendingExternalReview` rather than silently filled from another translator.

**Depends on**: Phase 20 (locked `HexagramEntry` schema + 64-entry composition table are prerequisites — CRIT-1 schema-lock-first).

**Requirements**: ICH-01

**Success Criteria** (what must be TRUE):
  1. A caller can look up any of the 64 King Wen hexagram indices (1..=64) and receive a populated `HexagramEntry` carrying `king_wen_index`, `vi_name`, `upper/lower_trigram`, `thoai_tu` (judgment), `hao_tu` (6 line texts; 7 for hexagrams 1 & 2), and `cat_hung` — loaded via `include_str!("data/iching/hexagrams.json")` parsed through a `OnceLock` cache (v1.5 `rituals/corpus.rs` pattern).
  2. A reader of `data/iching/hexagrams.json` can find each entry carries a reviewer signature (per ADR-0005 schema), and any entry where Ngô Tất Tố's source is silent carries a `PendingExternalReview` marker (v1.6 RIT-14 pattern) — never silently filled from another translator (AF-05).
  3. A reader can find every Vietnamese text field is NFC-normalized at load (RIT-08 precedent) and a `data/iching/provenance_audit.md` ledger accompanies the corpus.
  4. A caller can observe corpus load is lazy (only triggered on first lookup) and WASM-safe (no `std::fs`, no `Utc::now`).

**Plans:** 2/2 plans complete

Plans:
- [x] 21-01-PLAN.md — Author 64-hexagram corpus JSON (deterministic fields + AF-05 deferred text) + provenance audit ledger
- [ ] 21-02-PLAN.md — OnceLock loader + get_hexagram/all_hexagrams lookup API + NFC/hao_tu-invariant + black-box integration tests

### Phase 22: Mai Hoa Casting + Biến Quẻ + Thể/Dụng

**Goal**: A caller can cast a Mai Hoa hexagram (chủ quẻ) from lunar inputs, derive its biến quẻ (transforming hexagram) from the động hào, and read the Thể/Dụng classification plus Ngũ Hành sinh/khắc relationship driving the cát/hùng reading — all as pure deterministic functions with no RNG, validated by a 384-case biến quẻ contract test and ≥10 cross-source golden cases.

**Depends on**: Phase 21 (casting consumes `get_hexagram_by_number` from the corpus loader).

**Requirements**: ICH-02, ICH-03, ICH-04

**Success Criteria** (what must be TRUE):
  1. A caller can invoke `cast_mai_hoa(lunar_year_branch, lunar_month, lunar_day, chi_hour_index) -> MaiHoaCast` and receive the upper/lower Tiên Thiên trigram pair + động hào (moving line 1..=6), deterministic with no RNG, honouring the `((n-1)%k)+1` remainder-zero convention — verifiable by a `month=8/day=8/hour=8` boundary case producing Tiên Thiên 8 (Khôn), not 1 (Kiền) (CRIT-2 prevention).
  2. A caller can derive the biến quẻ from any `MaiHoaCast` by flipping the động hào bit and re-composing, verifiable by a 384-case (64 chủ quẻ × 6 động hào) exhaustive contract test (CRIT-4 prevention).
  3. A reader of a `MaiHoaCast` can find the Thể (the trigram NOT containing the động hào) and Dụng (the trigram containing it) classification plus the Ngũ Hành sinh/khắc relationship driving the cát/hùng reading.
  4. A reader can find ≥10 cross-source golden casting cases (each cross-checked against ≥2 independent sources — `nhantu.net` + a second Mai Hoa reference); divergences are logged as `KnownDivergence`, never silently corrected.

**Plans**: 2 plans

Plans:
- [x] 22-01-PLAN.md — MaiHoaCast + `cast_mai_hoa` + biến quẻ derivation (ICH-02, ICH-03; CRIT-2 boundary + CRIT-4 384-case contract)
- [x] 22-02-PLAN.md — Thể/Dụng classification + Ngũ Hành sinh/khắc + ≥10 cross-source golden cases (ICH-04 + SC4)

### Phase 23: Thái Tuế / Tam Sát ⇄ Phi Tinh Cross-Link

**Goal**: User-of-directions can find a Thái Tuế directional derivation + classical 3-direction Tam Sát module (both carrying `source_id: khcbppt` evidence with the two `evidence: None` backfills populated) AND a read-only `build_direction_cross_link` that surfaces BOTH the KHCBPPT Thái Tuế/Tam Sát directional taboos AND the `huyen-khong` Phi Tinh palace layout in one composite picture — with CRIT-3 isolation preserved.

**Depends on**: Phase 20 (source IDs + ontology slots required). PARALLEL to Phases 21-22 (cross-link does NOT touch `reasoning/iching/`; consumes only already-shipped `compute_thai_tue`, `get_sat_phuong`, and v1.5 `snapshot.flying_stars`).

**Requirements**: XLK-01, XLK-02, XLK-03

**Success Criteria** (what must be TRUE):
  1. A caller can invoke a directional Thái Tuế `pub fn` on `thai_tue.rs` mapping year-chi → `Direction8`, distinct from the existing personal-conflict-only Thái Tuế, carrying `source_id: khcbppt` evidence — and the two 1-line `evidence: None` backfills on `thai_tue.rs:107-111` + `sat_phuong.rs:49-53` are populated.
  2. A caller can invoke a classical 3-direction Tam Sát module (`almanac/tam_sat.rs`) returning the THREE contiguous sơn/directions per year from the Tam Hợp triad opposition, carrying `source_id: khcbppt` with a KHCBPPT-pinned citation; the existing single-direction `sat_phuong.rs` day-chi feature stays intact.
  3. A caller can invoke `build_direction_cross_link(snapshot, birth_chi_index) -> PersonalFactNode` in `reasoning/direction_composite.rs` that surfaces BOTH the KHCBPPT Thái Tuế/Tam Sát directional taboos AND the `huyen-khong` Phi Tinh palace layout in one composite fact node, emitting distinct primitive `source_id` envelopes (`khcbppt` + `huyen-khong`) plus one `rule.composite.direction_cross_link` envelope.
  4. A reader can confirm CRIT-3 isolation is preserved: no `FlyingStar` reference in `interaction/direction_merge.rs` (verified by a sibling `tests/thai_tue_cross_link_crit3.rs` grep guard), and `build_direction_cross_link` takes only `&` references (read-only by design).

**Plans**: 3 plans (Wave 1: 23-01 + 23-02 parallel; Wave 2: 23-03)

Plans:
- [x] 23-01-PLAN.md — Directional Thái Tuế + Tam Sát primitives, evidence backfills, and pending-review provenance (XLK-01, XLK-02). Completed 2026-07-16.
- [x] 23-02-PLAN.md — Cross-link DTO contracts, annual safety-hint transport, and default-None DaySnapshot field (XLK-03 contracts half; XLK-03 closes with 23-03). Completed 2026-07-16.
- [x] 23-03-PLAN.md — Read-only composite builders, immutable enrichment, evidence tests, and CRIT-3 sibling guard (XLK-03). Completed 2026-07-16.

### Phase 24: IChing Evaluator + Semantic-Graph Wiring + DTO Integration

**Goal**: A caller can run an `IChingQuery` through an `IChingEvaluator` that emits per-step `ReasoningEvidenceEnvelope` instances (distinct source_ids + one composite), and a semantic-graph reader can find Hexagram nodes (chủ quẻ + biến quẻ) wired via `LocatedAt`/`Transforms` edges plus a composite cross-link fact node — both surfaced additively on `DaySnapshot` with v1.6→v1.7 backward-compat preserved.

**Depends on**: Phase 22 (algorithm) + Phase 23 (cross-link) — both feed the evaluator + builders; wiring earlier would compile against stubs.

**Requirements**: ICH-05, INT-11, INT-12

**Success Criteria** (what must be TRUE):
  1. A caller can construct an `IChingQuery` (sibling newtype, NOT a `ConsultationIntent::IChing` variant per the research reconciliation) and run it through an `IChingEvaluator` that emits per-step `ReasoningEvidenceEnvelope` instances with distinct source_ids (`mai-hoa-dich-so` for casting + `kinh-dich` for text lookup) plus one composite envelope — verifiable by a contract test asserting ≥2 distinct primitive source_ids + 1 composite per consultation (CRIT-6), and the evaluator works fully at Tier 0 (no birth data required — MOD-7).
  2. A reader of the semantic graph can find Hexagram nodes (chủ quẻ + biến quẻ) wired via `LocatedAt`/`Transforms` edges, plus a composite cross-link fact node — emitted by additive `add_iching_facts()` + `add_direction_composite_facts()` builder methods (v1.5 FlyingStar/Offering + v1.6 RecommendsOffering precedent).
  3. A reader of `DaySnapshot` can find additive `iching_cast: Option<IChingCastSummary>` and `direction_cross_link: Option<DirectionCrossLinkSummary>` fields (`#[serde(default, skip_serializing_if = "Option::is_none")]`).
  4. A caller can deserialize a v1.6 `DaySnapshot` JSON (no `iching_cast` / `direction_cross_link` fields) into the v1.7 struct and re-serialize without unexpected fields (combined-strip v1.6→v1.7 round-trip, mirroring the v1.5→v1.6 pattern from Phase 19-03).

**Plans**: 3 plans

Plans:
- [x] 24-01-PLAN.md — `IChingQuery` sibling newtype + `IChingEvaluator` (Tier-0, no birth data) + per-step evidence envelopes (`mai-hoa-dich-so` + `kinh-dich` primitives + `rule.composite.iching_consultation` composite) + additive `DaySnapshot.iching_cast` field + immutable `enrich_day_snapshot_with_iching` helper (ICH-05 Closed, INT-12 partial). Completed 2026-07-16.
- [x] 24-02-PLAN.md — Semantic-graph wiring: `add_iching_facts()` (2 distinct Hexagram nodes + role-bearing stable keys + Transforms + LocatedAt edges + dual-source provenance) + `add_direction_composite_facts()` (Phase 23 composite envelope + KHCBPPT + huyen-khong primitives); `SemanticId::iching_hexagram(role, king_wen, date, tz)` constructor; `IChingCastSummary` accessors (INT-11 Closed). Completed 2026-07-16.
- [ ] 24-03-PLAN.md — Combined-strip v1.6→v1.7 round-trip tests (3 new tests in `day_snapshot_v14_compat.rs`) + full INT-12 close-out (both additive `DaySnapshot.iching_cast` + `direction_cross_link` populated simultaneously, byte-equal round-trip with v1.6 fields intact).

### Phase 25: E2E Validation + Golden Cross-Source Verification

**Goal**: User-of-validation can find ≥10 IChing golden casting cases cross-checked against ≥2 independent sources, a 2026 E2E smoke exercising the full new surface (IChing casting + biến quẻ + Thái Tuế cross-link + semantic graph + DaySnapshot), and the full crate test suite green with zero regressions.

**Depends on**: Phase 24 (everything must be wired before final validation).

**Requirements**: INT-13

**Success Criteria** (what must be TRUE):
  1. A reader can find ≥10 IChing golden casting cases (in `data/iching/` golden dataset), each cross-checked against ≥2 independent sources (`nhantu.net` + a second Mai Hoa reference), with divergences logged as `KnownDivergence` (not silently corrected).
  2. A reader can find a 2026 E2E smoke (extending `tests/integration_2026_smoke.rs`) exercising IChing casting + biến quẻ + Thái Tuế cross-link + semantic-graph wiring + DaySnapshot fields together on representative dates.
  3. A caller running `cargo test --package amlich-core` observes the full test suite green with zero regressions on existing v1.6 tests (the 922-test v1.6 baseline continues passing plus new v1.7 tests added).
  4. A reader can confirm the cargo dependency tree is unchanged from v1.6 (no new crates — `cargo tree -p amlich-core --depth 1` still shows only `serde` / `serde_json` / `chrono` / `unicode-normalization`).

**Plans**: TBD

## Progress Table

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 20. Foundation — Schema Lock + Source IDs + ADRs + Ontology | 3/3 | Complete   | 2026-07-15 |
| 21. IChing Corpus + Loader | 2/2 | Complete    | 2026-07-16 |
| 22. Mai Hoa Casting + Biến Quẻ + Thể/Dụng | 2/2 | Complete    | 2026-07-16 |
| 23. Thái Tuế / Tam Sát ⇄ Phi Tinh Cross-Link | 3/3 | Complete    | 2026-07-16 |
| 24. IChing Evaluator + Semantic-Graph Wiring + DTO Integration | 2/3 | In Progress|  |
| 25. E2E Validation + Golden Cross-Source Verification | 0/TBD | Not started | - |

## Requirement Coverage

| Phase | Requirements | Count |
|-------|--------------|-------|
| 20 | FND-09, FND-10, FND-11, FND-12 | 4 |
| 21 | ICH-01 | 1 |
| 22 | ICH-02, ICH-03, ICH-04 | 3 |
| 23 | XLK-01, XLK-02, XLK-03 | 3 |
| 24 | ICH-05, INT-11, INT-12 | 3 |
| 25 | INT-13 | 1 |
| **Total** | | **15 / 15** ✓ |

## Dependency Graph

```
Phase 20 (Foundation, BLOCKING)
   │
   ├──→ Phase 21 (Corpus)
   │       │
   │       └──→ Phase 22 (Casting) ──┐
   │                                 │
   └──→ Phase 23 (Cross-link, PARALLEL to 21-22) ──→ Phase 24 (Evaluator + Wiring + DTO)
                                                              │
                                                              └──→ Phase 25 (E2E)
```

- **Critical path:** 20 → 21 → 22 → 24 → 25 (5 hops).
- **Parallel track:** 20 → 23 → 24 (cross-link independent of the IChing pillar).
- **Merge point:** Phase 24 consumes both the IChing pillar (Phase 22) and the cross-link (Phase 23).

<details>
<summary>✅ v1.6 Eastern Knowledge Completion (Phases 16-19) — SHIPPED 2026-07-16</summary>

- [x] Phase 16: Foundation — ADR-0003 Confidence Closure (2/2 plans) — completed 2026-07-15
- [x] Phase 17: Văn khấn Reviewer Closure (2/2 plans) — completed 2026-07-15
- [x] Phase 18: Daily Phi Tinh 日紫白 (4/4 plans) — completed 2026-07-15
- [x] Phase 19: RecommendsOffering Semantic-Graph Node + v1.6 Integration (3/3 plans) — completed 2026-07-16

**Delivered:** Daily Phi Tinh layer + `RecommendsOffering` first-class node + v1.5 review/confidence debt closed. 12/12 requirements satisfied; 922 tests pass. Full details in [`milestones/v1.6-ROADMAP.md`](milestones/v1.6-ROADMAP.md).

</details>

<details>
<summary>✅ v1.5 Eastern Knowledge Expansion (Phases 10-15) — SHIPPED 2026-05-28</summary>

- [x] Phase 10: Foundation — Schema Lock + ADRs + Source-ID Registration (5/5 plans)
- [x] Phase 11: Văn khấn Module + Lookup APIs (3/3 plans)
- [x] Phase 12: Văn khấn Corpus Authoring (4/4 plans)
- [x] Phase 13: Phi Tinh Primitives + Period + Annual/Monthly (4/4 plans)
- [x] Phase 14: Phi Tinh 81-Cell Aspects + Safety Hints (4/4 plans)
- [x] Phase 15: Semantic Graph Wiring + DTO Integration + E2E Validation (4/4 plans)

Full details in [`milestones/v1.5-ROADMAP.md`](milestones/v1.5-ROADMAP.md).

</details>

## Cross-Cutting Constraints (carry-forward)

- **Additive-only DTOs** — all new fields `Option<T>` with `#[serde(default, skip_serializing_if = "Option::is_none")]` (v1.2 precedent; re-validated in v1.5 INT-05 + v1.6 INT-10; v1.7 re-validates as INT-12).
- **Source-id discipline** — every new module declares `pub const SOURCE_*` for its tradition; provenance call-sites never use string literals (CI-enforced by `tests/source_id_guard.rs`; v1.7 extends the guard with `SOURCE_KINH_DICH` + `SOURCE_MAI_HOA_DICH_SO`).
- **CRIT-3 isolation** — `FlyingStar` / `DailyFlyingStarLayout` remain palace-layout descriptors; never wired into `interaction/direction_merge.rs`. v1.7 extends the grep guard with a sibling `tests/thai_tue_cross_link_crit3.rs` to cover the new `reasoning/direction_composite.rs` read-only cross-link.
- **Schema-lock before corpus/algorithm** — type stubs + ADR + 1-entry serde round-trip probe precede corpus authoring (v1.5 CRIT-1 lesson × 7 amplification for v1.7's 448 corpus text fields).
- **Sibling-newtype over closed-enum extension** — `IChingQuery` newtype + `IChingEvaluator` rather than extending `ConsultationIntent` (v1.6 `DailyFlyingStarLayout` sibling precedent; avoids ~25–43 call-site `Copy`-break churn).
- **Composite-envelope multi-source provenance** — cross-link emits multiple `ReasoningEvidenceEnvelope` instances per `PersonalFactNode`, each with its distinct primitive `source_id`, plus ONE composite envelope with `source_id: "rule.composite.*"` (the only pattern compatible with the CRIT-3 grep guard; v1.5 INT-09 dual-source Direction-node precedent).

---

*Last updated: 2026-07-16T17:58:04Z — Phase 24 Plan 24-02 complete (DaySnapshotGraphBuilder::add_iching_facts + add_direction_composite_facts wired into the builder dispatch; 2 distinct Hexagram nodes + Transforms + LocatedAt edges + dual-source provenance + Direction composite fact; INT-11 Closed). Phase 24 is 2/3 plans done — 24-01 + 24-02 complete; 24-03 remaining for INT-12 full close. v1.7: 4/6 phases done; 13/15 requirements closed (ICH-05 + INT-11 just added; INT-12 partial + INT-13 still Pending). Next: `/gsd-execute-phase 24-03` (combined-strip v1.6→v1.7 round-trip tests + INT-12 full close).*
