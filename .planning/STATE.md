---
gsd_state_version: 1.0
milestone: v1.7
milestone_name: Kinh Dịch (I-Ching Divination)
status: ready_to_plan
last_updated: "2026-07-16T03:00:00Z"
progress:
  total_phases: 6
  completed_phases: 0
  total_plans: 0
  completed_plans: 0
---

# Project State
## Project Reference

See: .planning/PROJECT.md (updated 2026-07-16)

**Core value:** Every almanac subsystem in amlich must produce output matching its canonical classical source (KHCBPPT / `vn-folk-ritual` / *Thẩm Thị Huyền Không Học* / *Kinh Dịch Trọn Bộ* / *Mai Hoa Dịch Số*) for the 2020-2030 date range, with test-backed and traceable evidence.

**Current focus:** v1.7 Kinh Dịch — add the P2 pillar (Mai Hoa Dịch Số casting + 64-hexagram Ngô Tất Tố lookup) as a Tier-0 reasoning capability, plus the Thái Tuế / Tam Sát ⇄ Phi Tinh read-only directional cross-link. Phase numbering continues from v1.6 (starts at Phase 20).

## Current Position

Milestone: v1.7 Kinh Dịch (I-Ching Divination).
Phase: 20 — Foundation (Schema Lock + Source IDs + ADRs + Ontology) — Not started.
Plan: —
Status: Roadmap ready (6 phases, 15/15 requirements mapped). Next: `/gsd-plan-phase 20`.
Last activity: 2026-07-16 — v1.7 roadmap created (Phases 20-25; schema-lock-first ordering; parallel cross-link track at Phase 23).

Progress: [░░░░░░░░░░] 0% (v1.7: 0/6 phases complete; 0/15 requirements closed).

## v1.7 Roadmap Summary

| Phase | Goal | Requirements | Depends on | Track |
|-------|------|--------------|------------|-------|
| 20 | Foundation — Schema Lock + Source IDs + ADRs + Ontology | FND-09, FND-10, FND-11, FND-12 | — (BLOCKING) | Both |
| 21 | IChing Corpus + Loader | ICH-01 | 20 | IChing pillar |
| 22 | Mai Hoa Casting + Biến Quẻ + Thể/Dụng | ICH-02, ICH-03, ICH-04 | 21 | IChing pillar |
| 23 | Thái Tuế / Tam Sát ⇄ Phi Tinh Cross-Link | XLK-01, XLK-02, XLK-03 | 20 (PARALLEL to 21-22) | Cross-link |
| 24 | IChing Evaluator + Semantic-Graph Wiring + DTO | ICH-05, INT-11, INT-12 | 22 + 23 (merge) | Both |
| 25 | E2E Validation + Golden Cross-Source Verification | INT-13 | 24 | Both |

**Critical path:** 20 → 21 → 22 → 24 → 25 (5 hops).
**Parallel track:** 20 → 23 → 24.

## v1.7 Target Features

1. **Mai Hoa Dịch Số casting** — `cast_mai_hoa(lunar_year_branch, lunar_month, lunar_day, chi_hour_index) -> MaiHoaCast` (pure deterministic; `((n-1)%k)+1` remainder-zero convention; CRIT-2 boundary-tested).
2. **Biến quẻ derivation** — flip động hào bit + re-compose; 384-case (64 chủ quẻ × 6 động hào) contract test (CRIT-4).
3. **64-hexagram Ngô Tất Tố corpus** — `data/iching/hexagrams.json`, NFC-normalized, reviewer-signed, `PendingExternalReview` for source gaps (AF-05).
4. **`IChingQuery` + `IChingEvaluator`** — sibling newtype (NOT `ConsultationIntent::IChing` variant); per-step source_id envelopes + composite (CRIT-6).
5. **Thái Tuế directional + classical 3-direction Tam Sát** — new `pub fn` on `thai_tue.rs` + new `almanac/tam_sat.rs` (both `source_id: khcbppt`); two 1-line evidence backfills populated.
6. **Read-only directional cross-link** — `reasoning/direction_composite.rs::build_direction_cross_link` surfacing KHCBPPT directional taboos + `huyen-khong` palace layout; CRIT-3 isolation preserved.
7. **Semantic-graph + DTO integration** — `Hexagram` nodes + `LocatedAt`/`Transforms` edges; additive `DaySnapshot.iching_cast` + `direction_cross_link`; v1.6→v1.7 round-trip.

## Critical Pitfalls (v1.7-specific, from research SUMMARY.md)

- **CRIT-1** — Schema-slip after corpus authored (448 text fields × re-edit cost). Gated by Phase 20 schema-lock-first.
- **CRIT-2** — Mai Hoa `% 8 == 0` / `% 6 == 0` remainder-zero convention (silently corrupts ~1/8 of castings). Gated by Phase 22 boundary golden case.
- **CRIT-3** — Tiên Thiên trigram numbers vs King Wen hexagram numbers (different mappings, shared "1..N" form). Gated by Phase 20 three-distinct-newtypes with NO `From` impls.
- **CRIT-4** — Biến quẻ bit-position correctness. Gated by Phase 22 384-case exhaustive contract test.
- **CRIT-5** — Cross-link collapses CRIT-3 isolation. Gated by Phase 23 `reasoning/` placement + sibling grep guard.
- **CRIT-6** — `kinh-dich` vs `mai-hoa-dich-so` source-id cross-contamination. Gated by Phase 20 dual `pub const` + Phase 24 per-step envelopes + contract test.

## Open Research Questions (to resolve during planning)

- **Phase 20 ADR-0006:** Tiên Thiên number arrangement pinning (1=Kiền..8=Khôn dominant, but at least one Vietnamese sub-school differs) — cite Thiệu Khang Tiết page reference. Hán-Việt orthography choice (modern `thuỷ` vs pre-1975 `thủy`).
- **Phase 21 corpus:** Ngô Tất Tố corpus completeness (all 64 hexagrams with both thoán từ AND all 6 hào từ? hexagrams 1 & 2's 7th "dụng" hào?).
- **Phase 22 casting:** Mai Hoa edge-case tiebreaks (raw sum mod 8 == 0; hour chi indexing with DEC-0017); lunar-vs-solar input convention (recommend lunar per Thiệu Khang Tiết; existing `lunar.rs` does correct Vietnamese conversion).
- **Phase 23 cross-link:** **FS-10 3-vs-1 direction decision** (recommend option b: new `almanac/tam_sat.rs`); classical 3-direction Tam Sát KHCBPPT citation to locate. Note: existing `almanac/tam_tai.rs` (Tam Tai, three killings in a different sense) is distinct from `almanac/tam_sat.rs` (classical Tam Sát directional) — confirm no overlap.

## Resources

- `.planning/PROJECT.md` — project trajectory + Key Decisions table (v1.7 milestone scope added 2026-07-16).
- `.planning/MILESTONES.md` — shipped-milestone log (v1.0-v1.6 archived).
- `.planning/ROADMAP.md` — v1.7 active roadmap (Phases 20-25); v1.6 + v1.5 collapsed in `<details>`.
- `.planning/REQUIREMENTS.md` — v1.7 requirements (15 total; traceability filled).
- `.planning/research/SUMMARY.md` — v1.7 research (HIGH confidence; 8-phase structure suggested, consolidated to 6 for quick depth).
- `.planning/milestones/v1.{0..6}-{ROADMAP,REQUIREMENTS,MILESTONE-AUDIT}.md` — shipped milestone archives.
- `.planning/adrs/` — ADRs 0001-0004 + 0003a (locked); ADRs 0005/0006/0007 to be authored in Phase 20.

## Session Continuity

Last session: 2026-07-16T03:00:00Z
Stopped at: v1.7 roadmap created (6 phases, 15/15 requirements mapped). Files written: ROADMAP.md, REQUIREMENTS.md (traceability), STATE.md.
Resume file: None.

### Next Step

Run `/gsd-plan-phase 20` to plan the Foundation phase (schema lock + source IDs + ADRs + ontology). Phase 20 is BLOCKING — it gates the entire v1.7 milestone via CRIT-1 schema-lock-first.

---

<details>
<summary>Archived v1.6 Key Decisions (baked into ADRs/code; preserved for reference)</summary>

## Key Decisions Added in 18-01 + 18-02 + 18-03 + 18-04

- ADR-0004 locks daily Phi Tinh to 6 Trung Khí pivots with Dương→thuận and Âm→nghịch direction; this is intentionally opposite ADR-0003's annual polarity rule.
- Daily pivot seeds take effect at the first Giáp Tý with JD >= pivot_jd, not at the pivot instant itself; pre-Giáp-Tý days remain under the prior pivot (Pitfall P-7 fall-back).
- The frozen v1 `FlyingStarLayout` remains unchanged; daily schema uses the additive `FlyingStarPeriod::Daily { date: (i32, u32, u32) }` variant plus sibling `DailyFlyingStarLayout`.
- The `daily_pivots_for_year` scanner bracket spans `[year-1, year, year+1]` (widened from the plan's `[year, year+1]`) for robust boundary lookup on late-December dates.
- Pivot matchers accept both "Vũ Thuỷ" (NFD/legacy) and "Vũ Thủy" (NFC/preferred) as the same pivot — Unicode NFC/NFD unification mirrors v1.5 source-corpus normalization discipline.
- Daily golden dataset uses one-file-per-concern split (`flying_stars_daily_golden.json` separate from `flying_stars_golden.json`) per 18-RESEARCH.md Q3 Option B.
- Daily dataset's `expected_center` values are algorithm-computed via `compute_daily_flying_stars` (algorithm-as-ground-truth); external sources are cited as verifications, not as the primary computation source.
- Validator's annual-coverage gate is now kind-aware (conditional on `has_annual`) so daily-only datasets pass validation without panic.
- `DaySnapshot.daily_flying_stars` uses the EXACT serde additive pattern as `flying_stars` / `applicable_rituals`; populate block sits BETWEEN the two existing blocks for readability; solar Y/M/D extracted from `snap.context.solar` to match the snapshot's own context.
- `tests/fengshui_crit3_isolation.rs` is semantically distinct from `tests/source_id_guard.rs` — the former forbids Phi Tinh TYPE NAMES leaking into `direction_merge.rs`; the latter forbids bare source_id STRING LITERALS. Both guards are complementary.

## Key Decisions Added in 19-01

- `pub type SourceId = String;` is a zero-cost newtype over String (NOT a true newtype enforcing SOURCE_* membership) — preserves DEC-0023's `pub const SOURCE_*: &str` discipline (all 7 consts unchanged) while satisfying INT-07's literal "source_id: SourceId" SC text. The alias is a transparent type marker; future phases MAY tighten into a true newtype that enforces SOURCE_* membership at construction.
- `OfferingRef::new(...)` accepts `String` source_id for call-site ergonomics — internally stored as `SourceId`; `debug_assert!` enforces non-empty on `offering_id`, `name_vi`, `source_id`. Avoids forcing call-sites to write `SourceId::from(SOURCE_X.to_string())`.
- `offering_id` is corpus-position-based (`format!("ritual.{ritual_id}.offering.{idx}")`), NOT hashed from `name_vi` — per 19-RESEARCH.md Pitfall P-3 / Don't-Hand-Roll (hashing name_vi would break stable join keys if the corpus is reordered or renamed).
- Both `offering_refs` and `offerings` are derived from the SAME source — `applicable_rituals` via `get_ritual_by_id`; `offering_refs` is the structured preferred path, `offerings` is the legacy flat-string BC summary. `offerings` is deduped by `name_vi` and preserves insertion order (Q4 interpretation i from 19-RESEARCH.md).
- `is_empty() → None` conversion in the populate block preserves the additive contract — a day with no matching rituals (no `offering_refs`) MUST NOT serialize the `offering_refs` key into JSON (skip_serializing_if honored).
- Schema-lock-before-builder discipline preserved: NO builder code emits `Offering` semantic-graph nodes in Plan 19-01; this is reserved for Plan 19-02 (Q4 dual-surface decision: fields on DaySnapshot PLUS additive `payload: Option<serde_json::Value>` on `SemanticNode`).

## Key Decisions Added in 19-02

- INT-07 closed: `NodeConcept::Offering` + `EdgeConcept::RecommendsOffering` added to all 6 ontology slice locations in `ontology.rs` (enum + label() match + ConceptLabel enum + as_str() match + node_concepts()/edge_concepts() static slices + extended locked test). Compiler-enforced exhaustiveness forced updates to `views/helpers.rs::cluster_for_node_id` + `views/visualization.rs::shape_hint_for_node` — both updated for `Offering` variant (no `#[allow(non_exhaustive)]` escape).
- INT-08 SC#2 literal interpretation (Blocker 2 fix): SemanticNode payload uses generic `serde_json::Value` (Option B from 19-RESEARCH.md) — NOT a typed `RitualNodePayload` enum. Matches v1.5 additive `Option<T>` discipline; other concepts can use the same field for concept-specific structured data.
- INT-09 closed (Blocker 1 fix — supersedes Q2 Option C deferral): `RitualMetadata { cross_source_curing: Option<Vec<CrossSourceCure>> }` + `CrossSourceCure { element_cure_for: String, source_id: SourceId, rationale_vi: String }` structs added to `rituals/schema.rs`; additive `metadata: Option<RitualMetadata>` field on `RitualEntry`. The `van-khan-tet-day-du` corpus entry (5 offerings) annotated with one `cross_source_curing` entry whose `source_id = "huyen-khong"`. The `add_offering_facts` builder emits 1 `track_provenance` call for `vn-folk-ritual` (always) + 1 extra call per `cross_source_curing` annotation — dual-source pattern reuses v1.5 multi-source append-pattern (NO parallel dedup helper).
- Payload post-population via new `pub fn nodes_mut(&mut self) -> &mut HashMap<String, SemanticNode>` accessor on `SemanticGraph` — additive companion to `nodes()`. The Ritual node is constructed first (without payload) via `add_node()`, then mutated via `nodes_mut()` after `offering_refs` is known.
- Rationale carried on the EDGE provenance note (Blocker 4 fix) — not just on the Offering node. The dual-source rationale `"lễ vật của nghi lễ, hỗ trợ chữa trị ngũ hành tương ứng"` is embedded in the vn-folk-ritual entry's note via `rationale=...` substring, ensuring any consumer querying the edge can recover the rationale without a node lookup.
- Edge dedup via `HashSet<(ritual_node_id, offering_node_id)>` (NOT provenance dedup) — keys on edge endpoints, not provenance entries. The v1.5 multi-source append-pattern remains the single source of truth for provenance.

## Key Decisions Added in 19-03

- `build_day_snapshot_graph` is re-exported at the `semantic_graph` crate root (`pub use builders::{build_day_snapshot_graph, ...}`) instead of flipping the private `builders` + `day_snapshot` modules to `pub mod`. The plan's literal import path (`amlich_core::semantic_graph::builders::day_snapshot::build_day_snapshot_graph`) is unreachable from external consumers because both modules are `mod` (private). The re-export is the minimal, idiomatic fix and mirrors the existing `build_reasoning_input_graph` re-export pattern; keeps the builder subtree private. Rule 3 (Blocking) auto-fix.
- Combined-strip v1.5→v1.6 round-trip test pattern (BLOCKER 5 FIX): Test 7 removes ALL v1.6-new additive fields together (`daily_flying_stars` + `offering_refs` + `offerings`) to simulate the v1.5 fixture shape, then re-serializes the recovered v1.6 value and asserts byte-equal round-trip + no unexpected fields. Extends Phase 18-04's single-strip pattern into a single canonical "strip every new field, re-serialize, assert byte-equal" discipline for additive DTO verification going forward.
- INT-10 closed: both sub-criteria satisfied — (1) v1.5→v1.6 backward-compat round-trip via 3 new tests in `day_snapshot_v14_compat.rs`, (2) >=5-date 2026 E2E smoke in `integration_2026_smoke.rs` exercising BOTH annual/monthly `flying_stars` AND new `daily_flying_stars` AND new `offering_refs` fields with semantic-graph `Offering` + `RecommendsOffering` wiring verified (BLOCKER 6 endpoint shape + INT-09 dual-source provenance + BLOCKER 7 annual/monthly FlyingStar components).

</details>

---
*State updated: 2026-07-16 — v1.7 roadmap created (Phases 20-25, 6 phases, 15/15 requirements mapped). v1.6 Key Decisions archived into `<details>` (baked into ADRs/code). Next: `/gsd-plan-phase 20`.*
