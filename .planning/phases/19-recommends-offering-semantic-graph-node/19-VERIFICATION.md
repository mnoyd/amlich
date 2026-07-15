---
phase: 19-recommends-offering-semantic-graph-node
verified: 2026-07-16T00:45:00Z
status: passed
score: 5/5 must-haves verified
---

# Phase 19: RecommendsOffering Semantic-Graph Node + v1.6 Integration — Verification Report

**Phase Goal:** User-of-semantic-graph can find `Offering` nodes connected to `Ritual` nodes via `RecommendsOffering` edges carrying rationale + source provenance; edges originating from a non-ritual tradition (e.g., a Huyền Không element cure surfaced inside a ritual) carry dual-source provenance via the v1.5 multi-source dedup pattern; a v1.5→v1.6 round-trip test passes and the 2026 E2E smoke exercises daily + annual fields together.
**Verified:** 2026-07-16T00:45:00Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | `NodeConcept::Offering` + `EdgeConcept::RecommendsOffering` across all 6 ontology slice locations + `OfferingRef` identity type locked before builder code emits Offering nodes | ✓ VERIFIED | All 6 slice locations confirmed: NodeConcept enum (`ontology.rs:42`), label() match (`:84`), ConceptLabel enum (`:223`), as_str() match (`:294`), node_concepts() slice (`:375`), edge_concepts() slice (`:409`). `OfferingRef` locked at `rituals/schema.rs:172` with the exact 4-field tuple `{offering_id, name_vi, name_en, source_id: SourceId}` + `deny_unknown_fields` + `Hash` derive. `v16_concepts_present_in_ontology_slices` test passes. Schema-lock-before-builder discipline intact: Plan 19-01 commits (`eddc51d`, `6508f79`) predate Plan 19-02 builder commits (`3dce869`, `18028b3`). |
| 2 | Additive `offering_refs: Option<Vec<OfferingRef>>` (preferred) + `offerings: Option<Vec<String>>` (legacy) on the Ritual semantic-graph node payload, both with `#[serde(default, skip_serializing_if = "Option::is_none")]` | ✓ VERIFIED | Dual-surface implementation per Phase 19 locked decision (Blocker 2 fix / Option B from 19-RESEARCH.md): (a) `DaySnapshot.offering_refs` + `offerings` declared at `lib.rs:177-184` with exact `#[serde(default, skip_serializing_if = "Option::is_none")]` attribute pair matching `flying_stars`/`applicable_rituals`/`daily_flying_stars`; (b) additive `SemanticNode.payload: Option<serde_json::Value>` at `node.rs:38` with same serde attribute; (c) `add_ritual_facts` populates aggregate Ritual node payload with `{"offering_refs": [...], "offerings": [...]}` JSON at `day_snapshot.rs:570-593` via new `nodes_mut()` accessor (`graph.rs:56`). |
| 3 | A `RecommendsOffering` edge carries dual-source provenance (both `huyen-khong` and `vn-folk-ritual`) for offerings originating in a non-ritual tradition surfaced inside a ritual; v1.5 multi-source Direction-node dedup logic reused (no parallel implementation) | ✓ VERIFIED | `add_offering_facts` (`day_snapshot.rs:603-747`) implements dual-source pattern: collects `entry.metadata.cross_source_curing` annotations (line 630-640), then for each Offering node emits RecommendsOffering edge + calls `self.graph.track_provenance(...)` ONCE for `SOURCE_VN_FOLK_RITUAL` (line 718-728) + ONE EXTRA call per cure annotation (line 732-744). Uses existing `ProvenanceTracker::track()` (`provenance.rs:130-135`) — NO parallel dedup helper introduced. Corpus annotation on `van-khan-tet-day-du` (`tet-nguyen-dan.json:60-68`) carries `source_id="huyen-khong"`, `element_cure_for="Kim"`. `recommends_offering_edge_carries_dual_source_provenance` test asserts at least one edge carries BOTH `vn-folk-ritual` AND `huyen-khong` source_ids on Tết 2026 (PASSES). |
| 4 | A v1.5 JSON fixture loads into v1.6 structs and re-serializes without unexpected fields — verifiable via extension of `tests/day_snapshot_v14_compat.rs` | ✓ VERIFIED | 3 NEW round-trip tests appended to `day_snapshot_v14_compat.rs`: `v15_json_without_v16_fields_deserializes_and_round_trips` (Test 7, BLOCKER 5 fix — strips daily_flying_stars + offering_refs + offerings TOGETHER to simulate v1.5 fixture shape, re-serializes recovered v1.6, asserts byte-equal round-trip + no unexpected fields), `offering_refs_byte_equal_round_trip` (Test 8, field-by-field shape discipline on `offering_refs[0]`), `offering_refs_absent_when_none` (Test 9, skip_serializing_if honored). All 9 tests in `day_snapshot_v14_compat.rs` PASS (3 v1.4→v1.5 + 3 v1.5→v1.6 daily_flying_stars + 3 NEW v1.5→v1.6 offering_refs). |
| 5 | End-to-end 2026 smoke test (`tests/integration_2026_smoke.rs`) passes on ≥ 5 representative dates exercising BOTH annual/monthly `flying_stars` AND new `daily_flying_stars`, with `Offering`/`RecommendsOffering` graph wiring verified for any day surfacing a non-ritual-origin offering | ✓ VERIFIED | `e2e_2026_smoke_offering_wiring_on_representative_dates` (`integration_2026_smoke.rs:274-469`) asserts `dates.len() >= 5` (Tết 2026 + 4 Sóc dates from solar months 3/6/9/12). Per date: asserts `daily_flying_stars.is_some()` + `flying_stars.palace_overlays.len() == 9` + each `(annual, monthly)` tuple member is a valid `FlyingStar` variant (BLOCKER 7 fix via `matches!` against all 9 variants). When applicable_rituals non-empty: asserts `offering_refs` + `offerings` populated + ≥1 `NodeConcept::Offering` node + ≥1 `EdgeConcept::RecommendsOffering` edge + endpoint shape (from_node=Ritual, to_node=Offering, BLOCKER 6 fix) + at least one date's edge provenance contains BOTH source_ids. Test PASSES. |

**Score:** 5/5 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
| --- | --- | --- | --- |
| `crates/amlich-core/src/sources.rs` | `pub type SourceId = String;` alias | ✓ VERIFIED | Line 41: `pub type SourceId = String;` declared after the 7 SOURCE_* consts. Zero-cost newtype preserving DEC-0023 const discipline while satisfying INT-07's literal "source_id: SourceId" text. |
| `crates/amlich-core/src/rituals/schema.rs` (OfferingRef) | Locked `OfferingRef` struct + `new()` constructor + tests | ✓ VERIFIED | Line 172: `pub struct OfferingRef { offering_id, name_vi, name_en, source_id }` with `deny_unknown_fields` + `Hash` derive. Lines 183-203: `impl OfferingRef::new(...)` with `debug_assert!` on all 3 required fields. `offering_ref_serde_round_trip_and_deny_unknown_fields` test passes. |
| `crates/amlich-core/src/rituals/schema.rs` (RitualMetadata + CrossSourceCure) | INT-09 corpus-augmentation schema structs | ✓ VERIFIED | Lines 129-142: `CrossSourceCure { element_cure_for, source_id: SourceId, rationale_vi }`. Lines 150-155: `RitualMetadata { cross_source_curing: Option<Vec<CrossSourceCure>> }`. Line 244: additive `metadata: Option<RitualMetadata>` on `RitualEntry` with `#[serde(default, skip_serializing_if = "Option::is_none")]`. `ritual_metadata_and_cross_source_cure_serde_round_trip` test passes. |
| `crates/amlich-core/data/rituals/tet-nguyen-dan.json` | `van-khan-tet-day-du` annotated with `metadata.cross_source_curing` referencing `huyen-khong` | ✓ VERIFIED | Lines 60-68: `metadata.cross_source_curing[0]` has `element_cure_for="Kim"`, `source_id="huyen-khong"`, `rationale_vi="Huyền Không Ngũ Hành: ..."`. Confirmed annotation lives on the 5-offering `van-khan-tet-day-du` entry (the canonical Tết full-variant). |
| `crates/amlich-core/src/lib.rs` | Additive DaySnapshot fields + populate block + focused test | ✓ VERIFIED | Lines 177-184: `offering_refs: Option<Vec<OfferingRef>>` + `offerings: Option<Vec<String>>` with serde pair. Lines 386-418: populate block derives both fields from `applicable_rituals` via `get_ritual_by_id`. Lines 552-601: `day_snapshot_offering_refs_populated_and_deduped` focused test passes. |
| `crates/amlich-core/src/semantic_graph/ontology.rs` | `NodeConcept::Offering` + `EdgeConcept::RecommendsOffering` across all 6 slices + extended test | ✓ VERIFIED | All 6 slice locations extended. New sibling test `v16_concepts_present_in_ontology_slices` at lines 323-333 asserts slice membership + label round-trip. Compiler-enforced exhaustiveness: view helpers `cluster_for_node_id` and `shape_hint_for_node` updated for `Offering` per SUMMARY (no compile errors). |
| `crates/amlich-core/src/semantic_graph/node.rs` | Additive `payload: Option<serde_json::Value>` field on `SemanticNode` | ✓ VERIFIED | Line 38: `#[serde(default, skip_serializing_if = "Option::is_none")] pub payload: Option<serde_json::Value>`. Constructor inits to `None` at line 58. |
| `crates/amlich-core/src/semantic_graph/graph.rs` | New `nodes_mut()` accessor for post-population | ✓ VERIFIED | Line 56: `pub fn nodes_mut(&mut self) -> &mut HashMap<String, SemanticNode>` — additive companion to `nodes()`. |
| `crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs` | `add_offering_facts` helper emitting Offering nodes + RecommendsOffering edges with single/dual-source provenance + payload population + dual-source test | ✓ VERIFIED | Line 44: `builder.add_offering_facts(snapshot)` called from `new()` AFTER `add_ritual_facts`. Lines 540-601: `add_ritual_facts` populates aggregate Ritual node payload. Lines 603-747: `add_offering_facts` helper emits Offering nodes (single vn-folk-ritual provenance via constructor) + RecommendsOffering edges with edge dedup via `HashSet<(from_id, to_id)>` + 1 mandatory + N optional `track_provenance` calls. Edge provenance carries rationale (`rationale=...` substring — Blocker 4 fix). `recommends_offering_edge_carries_dual_source_provenance` test PASSES. |
| `crates/amlich-core/src/semantic_graph/mod.rs` | `build_day_snapshot_graph` re-exported at crate root | ✓ VERIFIED | Line 25: re-exported (Rule 3 auto-fix documented in 19-03-SUMMARY). Required for external-crate black-box tests to import the builder. |
| `crates/amlich-core/tests/day_snapshot_v14_compat.rs` | 3 NEW round-trip tests for offering_refs + offerings additive fields | ✓ VERIFIED | Tests 7-9 appended at lines 209, 294, 345. 2 new imports at top (`OfferingRef` + `SOURCE_VN_FOLK_RITUAL`). All 9 tests pass. |
| `crates/amlich-core/tests/integration_2026_smoke.rs` | NEW E2E smoke test exercising Offering wiring on ≥5 representative 2026 dates | ✓ VERIFIED | `e2e_2026_smoke_offering_wiring_on_representative_dates` at lines 274-469. 2 new imports (`build_day_snapshot_graph` + `EdgeConcept, NodeConcept`). All 4 integration_2026_smoke tests pass. |

### Key Link Verification

| From | To | Via | Status | Details |
| --- | --- | --- | --- | --- |
| `rituals/schema.rs` (OfferingRef) | `rituals/mod.rs` public API | `pub use schema::*;` glob re-export | ✓ WIRED | Line 26 of `rituals/mod.rs` re-exports via glob; `crate::rituals::OfferingRef` importable in `lib.rs` populate block + builder (`day_snapshot.rs:644`). |
| `calculate_day_snapshot_internal` (`lib.rs`) | `get_ritual_by_id` | `use crate::rituals::get_ritual_by_id` inside populate block (line 392) | ✓ WIRED | Populate block calls `get_ritual_by_id(ritual_id)` for each applicable ritual, iterates `entry.offerings`, builds `OfferingRef::new(...)`, populates both `offering_refs` + `offerings` fields. |
| `add_offering_facts` builder | `crate::rituals::OfferingRef` | `OfferingRef::new(...)` call (line 644) | ✓ WIRED | Each Offering node's `offering_id` derives from `format!("ritual.{ritual_id}.offering.{idx}")` (matches DaySnapshot populate block pattern). |
| `add_offering_facts` builder | `SOURCE_VN_FOLK_RITUAL` + `SOURCE_HUYEN_KHONG` | Two `track_provenance` calls per cross_source_curing edge (lines 718, 733) | ✓ WIRED | INT-09 dual-source provenance. First call: `SOURCE_VN_FOLK_RITUAL` always. Subsequent calls: one per `cross_source_curing` annotation source_id (e.g., `huyen-khong`). Verified passing by `recommends_offering_edge_carries_dual_source_provenance` test. |
| `add_ritual_facts` builder | `SemanticNode.payload` | `self.graph.nodes_mut().get_mut(&node_id).payload = Some(payload_value)` (line 590-592) | ✓ WIRED | Aggregate Ritual node payload carries `{"offering_refs": [...], "offerings": [...]}` derived from `snapshot.offering_refs`. Satisfies INT-08 SC#2 literal interpretation. |
| `DaySnapshotGraphBuilder::new` | `add_offering_facts` | `builder.add_offering_facts(snapshot);` line 44 (after `add_ritual_facts` line 43) | ✓ WIRED | New helper called from `new()` mirroring v1.5 split pattern. |
| `tests/integration_2026_smoke.rs` | `build_day_snapshot_graph` + `NodeConcept::Offering` + `EdgeConcept::RecommendsOffering` | `use amlich_core::semantic_graph::{build_day_snapshot_graph, EdgeConcept, NodeConcept}` at lines 25-26 | ✓ WIRED | E2E test asserts graph carries Offering nodes + RecommendsOffering edges on Tết 2026. |
| `tests/day_snapshot_v14_compat.rs` | `OfferingRef` field-shape assertions | `use amlich_core::rituals::OfferingRef` at line 15; field-by-field asserts in Test 8 (lines 303-313) | ✓ WIRED | Test asserts `offering_id` non-empty + `ritual.<id>.offering.<idx>` pattern + `source_id == SOURCE_VN_FOLK_RITUAL`. |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| --- | --- | --- | --- | --- |
| INT-07 | 19-02 | `NodeConcept::Offering` + `EdgeConcept::RecommendsOffering` + `OfferingRef { offering_id, name_vi, name_en, source_id: SourceId }` | ✓ SATISFIED | All 6 ontology slice locations extended + `OfferingRef` struct locked in 19-01 + `SourceId` alias in `sources.rs:41`. `v16_concepts_present_in_ontology_slices` test passes. |
| INT-08 | 19-01 + 19-02 | Dual-surface `offering_refs`/`offerings` on DaySnapshot AND Ritual semantic-graph node payload, both `#[serde(default, skip_serializing_if = "Option::is_none")]` | ✓ SATISFIED | `DaySnapshot.offering_refs` + `offerings` at `lib.rs:177-184` + `SemanticNode.payload` at `node.rs:38` + `add_ritual_facts` payload population. Test 7-9 round-trip tests pass. |
| INT-09 | 19-02 | Dual-source edge provenance (`vn-folk-ritual` + `huyen-khong`) for non-ritual-tradition offerings; reuse v1.5 multi-source Direction-node dedup logic | ✓ SATISFIED | `RitualMetadata` + `CrossSourceCure` schema + `van-khan-tet-day-du` corpus annotation + two `track_provenance` calls per cross_source_curing edge. Reuses `ProvenanceTracker::track()` — NO parallel dedup helper. `recommends_offering_edge_carries_dual_source_provenance` test confirms both source_ids present on Tết 2026 edge. |
| INT-10 | 19-03 | v1.5→v1.6 backward-compat round-trip test + 2026 E2E smoke on ≥5 dates exercising BOTH annual/monthly + new daily fields | ✓ SATISFIED | 3 NEW round-trip tests in `day_snapshot_v14_compat.rs` (BLOCKER 5 combined-strip + byte-equal + skip-if-none). 1 NEW E2E smoke test in `integration_2026_smoke.rs` on ≥5 dates (Tết + 4 Sóc) with daily + annual/monthly assertions + Offering wiring + endpoint verification + dual-source verification. All 9 + 4 tests pass. |

No orphaned requirements: REQUIREMENTS.md lines 60-63 map INT-07/08/09/10 to Phase 19 and all are claimed by plans (19-01 claims INT-08; 19-02 claims INT-07/08/09; 19-03 claims INT-10). All 4 IDs cross-referenced against REQUIREMENTS.md descriptions (lines 31-34) and verified against codebase evidence above.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| --- | --- | --- | --- | --- |
| `crates/amlich-core/src/semantic_graph/views/helpers.rs` | 115 | Pre-existing unused import `ProvenanceSource` (warning) | ℹ️ Info | Not introduced by this phase (noted in 19-01-SUMMARY). No functional impact. Out of scope. |

No TODO/FIXME/placeholder/HACK/not-implemented patterns found in any modified Phase 19 source file (`sources.rs`, `rituals/schema.rs`, `lib.rs`, `semantic_graph/ontology.rs`, `semantic_graph/node.rs`, `semantic_graph/graph.rs`, `semantic_graph/builders/day_snapshot.rs`, `semantic_graph/mod.rs`).

### Human Verification Required

None — all 5 success criteria are verifiable programmatically via Rust's type system (compiler-enforced exhaustive matches across the 6 ontology slice locations) + cargo tests (716 lib + 9 compat + 4 smoke + 1 source_id_guard all green). No visual UX, real-time behavior, or external-service integration requires human judgment for this phase.

### Gaps Summary

No gaps. All 5 success criteria verified end-to-end:

1. **Schema lock + ontology completeness** (SC#1): `NodeConcept::Offering` + `EdgeConcept::RecommendsOffering` present in all 6 slice locations, compiler-enforced exhaustive matches verified by clean `cargo build` + the new `v16_concepts_present_in_ontology_slices` test. `OfferingRef` locked in 19-01 before any builder code (commits `eddc51d`/`6508f79` predate `3dce869`/`18028b3`).

2. **Dual-surface additive fields** (SC#2): `offering_refs` + `offerings` on `DaySnapshot` (`lib.rs:177-184`) AND `SemanticNode.payload` (`node.rs:38`) both with `#[serde(default, skip_serializing_if = "Option::is_none")]`. The Phase 19 locked decision (Blocker 2 fix / Option B from 19-RESEARCH.md) to put fields on BOTH surfaces is implemented per the literal ROADMAP SC#2 wording.

3. **Dual-source edge provenance** (SC#3): `add_offering_facts` (`day_snapshot.rs:603-747`) emits 1 mandatory + N optional `track_provenance` calls per edge, reusing the v1.5 `ProvenanceTracker::track()` append-pattern with NO parallel dedup helper. Corpus annotation on `van-khan-tet-day-du` (`tet-nguyen-dan.json:60-68`) wired end-to-end (proven by `recommends_offering_edge_carries_dual_source_provenance` test passing).

4. **v1.5→v1.6 round-trip** (SC#4): 3 NEW tests in `day_snapshot_v14_compat.rs` (Tests 7-9) including BLOCKER 5 combined-strip discipline (strips `daily_flying_stars` + `offering_refs` + `offerings` together to simulate v1.5 fixture shape). Byte-equal round-trip verified.

5. **2026 E2E smoke** (SC#5): `e2e_2026_smoke_offering_wiring_on_representative_dates` exercises ≥5 dates with BOTH annual/monthly `flying_stars` (9 palace_overlays, valid FlyingStar tuples — BLOCKER 7 fix) AND new `daily_flying_stars` (Phase 18-04 invariant), plus Offering/RecommendsOffering wiring with endpoint shape verification (BLOCKER 6 fix) AND at least one edge with dual-source provenance on Tết 2026.

**Test suite status:** 716 lib tests + 9 day_snapshot_v14_compat tests + 4 integration_2026_smoke tests + 1 source_id_guard test all PASS. Zero regressions vs Phase 19-03 SUMMARY baseline.

**Discipline gates:** `tests/source_id_guard.rs` passes (no bare-string literals introduced — all SOURCE_* references use the `pub const` imports). Schema-lock-before-builder discipline preserved across plans 19-01→19-02. Additive-only DTO discipline preserved (`#[serde(default, skip_serializing_if = "Option::is_none")]` on every new field).

---

_Verified: 2026-07-16T00:45:00Z_
_Verifier: Claude (gsd-verifier)_
