---
phase: 15-semantic-graph-wiring-dto-integration-e2e-validation
verified: 2026-05-28T00:00:00Z
status: passed
score: 5/5 must-haves verified
re_verification: false
---

# Phase 15: Semantic Graph Wiring + DTO Integration + E2E Validation Verification Report

**Phase Goal:** User of `DaySnapshot` can observe ritual + flying-star surfaces additively, and a 2026 smoke test confirms the milestone holds end-to-end across Tết, Sóc/Vọng, Vận transitions, leap months, and all 24 Tiết Khí boundaries.
**Verified:** 2026-05-28
**Status:** PASSED
**Re-verification:** No — initial verification

---

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | A caller can deserialize a v1.5 DaySnapshot and find `flying_stars: Option<FlyingStarsSummary>` and `applicable_rituals: Option<Vec<String>>` with `#[serde(default, skip_serializing_if = "Option::is_none")]` — additive only | VERIFIED | `lib.rs` lines 163–167: both fields exist with exact serde attributes; test `day_snapshot_populates_additive_surfaces` passes; `additive_fields_absent_when_none` confirms skip-when-None behavior |
| 2 | A semantic-graph reader can find `NodeConcept::Ritual`, `NodeConcept::FlyingStar`, and `EdgeConcept::PrescribedFor`, `EdgeConcept::OccupiesPalace`, `EdgeConcept::CarriesElement` with exhaustive matches enforced by the compiler | VERIFIED | `ontology.rs` lines 40–41, 115–117, 148–150, 217–221, 286–290: all six ontology locations updated; `v15_concepts_present_in_ontology_slices` test passes; code compiles with zero non-exhaustive-match errors |
| 3 | A FlyingStar node carries only `source_id "huyen-khong"`; a Ritual node only `"vn-folk-ritual"`; the shared Direction node carries both provenance entries (multi-source, len==2); Phi Tinh absent from `direction_merge.rs` | VERIFIED | `day_snapshot.rs` builder: SOURCE_HUYEN_KHONG/SOURCE_VN_FOLK_RITUAL constants used throughout; no bare literals in production code; `grep "direction_merge"` returns nothing for builder file; test `v15_pillar_nodes_carry_disjoint_source_ids_and_direction_is_multi_source` passes; Direction node provenance.len()==2 asserted and confirmed |
| 4 | A caller can load any v1.4 JSON fixture into v1.5 structs and re-serialize without unexpected fields (backward-compat round-trip) | VERIFIED | All 3 tests in `day_snapshot_v14_compat.rs` pass: `v15_round_trip_byte_equal`, `additive_fields_absent_when_none`, `v14_json_without_new_fields_deserializes` |
| 5 | The 2026 calendar smoke test passes on >= 30 representative dates covering Tết, Sóc/Vọng×12, Vận 8→9 transition, leap-month dates, and all 24 Tiết Khí boundaries | VERIFIED | All 3 tests in `integration_2026_smoke.rs` pass: `e2e_2026_smoke_all_categories` (>=30 dates, 9-length palace arrays, no panics), `tet_2026_is_lunar_1_1`, `van_boundary_8_to_9` |

**Score:** 5/5 truths verified

---

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `crates/amlich-core/src/lib.rs` | DaySnapshot + FlyingStarsSummary DTO + two additive fields + serde derives on full type chain | VERIFIED | Lines 109–167: all types derive Serialize+Deserialize; FlyingStarsSummary defined at line 140; fields with skip_serializing_if at 163–167; both populated in calculate_day_snapshot_internal |
| `crates/amlich-core/src/types.rs` | serde derives on CanChi + NguHanh | VERIFIED | Line 58: `use serde::{Deserialize, Serialize}`; line 61/68: both types derive Serialize+Deserialize |
| `crates/amlich-core/src/lunar.rs` | serde derives on LunarDate | VERIFIED | Line 120: `serde::Serialize, serde::Deserialize` |
| `crates/amlich-core/src/tietkhi.rs` | serde derives on SolarTerm | VERIFIED | Line 18: `serde::Serialize, serde::Deserialize` |
| `crates/amlich-core/src/gio_hoang_dao.rs` | serde derives on GioHoangDao and nested types | VERIFIED | Lines 15, 137, 149: StarType, HourInfo, GioHoangDao all derive serde |
| `crates/amlich-core/src/semantic_graph/ontology.rs` | Two new NodeConcept variants, three new EdgeConcept variants, ConceptLabel mirrors, as_str strings, GraphOntology slice entries, completeness test | VERIFIED | All six locations confirmed: enum variants (40–41, 115–117), label() arms (81–82, 148–150), ConceptLabel enum (217–221), as_str() strings (286–290), static slices (353–354, 385–387), test at line 303–312 |
| `crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs` | add_ritual_facts + add_flying_star_facts methods, dual-provenance Direction node, SOURCE_HUYEN_KHONG/SOURCE_VN_FOLK_RITUAL imports | VERIFIED | Lines 5, 41–42: imports and registration; line 414: huyen_khong_prov appended to Direction node; line 474: add_flying_star_facts; line 513: add_ritual_facts; SOURCE_HUYEN_KHONG in production code throughout |
| `crates/amlich-core/tests/day_snapshot_v14_compat.rs` | INT-05 backward-compat round-trip + additive-field-absence assertions | VERIFIED | File exists, 3 substantive tests, all pass |
| `crates/amlich-core/tests/integration_2026_smoke.rs` | INT-06 >=30-date 2026 E2E smoke across all required categories | VERIFIED | File exists, 3 substantive tests using `terms_for_year`, `compute_combined_overlay`, `find_van_khan_for_snapshot`; all pass |

---

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| `DaySnapshot.flying_stars` | `compute_combined_overlay` | FlyingStarsSummary::from layout in `calculate_day_snapshot_internal` | WIRED | `lib.rs` line 321–344: overlay computed, FlyingStarsSummary built and assigned to snap.flying_stars |
| `DaySnapshot.applicable_rituals` | `find_van_khan_for_snapshot` | ritual_id collection in `calculate_day_snapshot_internal` | WIRED | `lib.rs` line 343–350: find_van_khan_for_snapshot called, ritual_ids collected, assigned to snap.applicable_rituals |
| `NodeConcept::Ritual` | `GraphOntology::node_concepts()` | static slice entry | WIRED | `ontology.rs` line 353: NodeConcept::Ritual in node_concepts() slice |
| `EdgeConcept::OccupiesPalace` | `GraphOntology::edge_concepts()` | static slice entry | WIRED | `ontology.rs` line 386: EdgeConcept::OccupiesPalace in edge_concepts() slice |
| `add_flying_star_facts` | `NodeConcept::FlyingStar + SOURCE_HUYEN_KHONG` | `SemanticNode::new(...).with_provenance(ProvenanceEntry::almanac_rule(SOURCE_HUYEN_KHONG, ...))` | WIRED | `day_snapshot.rs` line 489–492: SOURCE_HUYEN_KHONG used with NodeConcept::FlyingStar |
| `add_travel_direction_fact (Direction node)` | second ProvenanceEntry with SOURCE_HUYEN_KHONG | `.with_provenance(huyen_khong_prov)` appended to same node | WIRED | `day_snapshot.rs` lines 413–428: huyen_khong_prov created and chained with `.with_provenance()` before single `add_node` call |
| `add_ritual_facts` | `NodeConcept::Ritual + SOURCE_VN_FOLK_RITUAL` | `SemanticNode::new(...).with_provenance(ProvenanceEntry::almanac_rule(SOURCE_VN_FOLK_RITUAL, ...))` | WIRED | `day_snapshot.rs` lines 525–528 |
| `integration_2026_smoke.rs` | `TietKhiScanner::terms_for_year(2026)` | 24 Tiết Khí boundary enumeration | WIRED | Test file line 91: `scanner.terms_for_year(2026)` called in `collect_tiet_khi_dates()` |
| `integration_2026_smoke.rs` | `compute_combined_overlay + compute_palace_aspects + find_van_khan_for_snapshot` | per-date pillar API calls | WIRED | Test file lines 109, 116, 125: all three called in `assert_date_pillar_apis_ok` |

---

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|------------|-------------|--------|----------|
| INT-01 | 15-01 | `flying_stars: Option<FlyingStarsSummary>` additive field with skip_serializing_if | SATISFIED | `lib.rs` lines 163–164; test `day_snapshot_populates_additive_surfaces` passes |
| INT-02 | 15-01 | Ritual-surfacing additive optional field on DaySnapshot | SATISFIED | `lib.rs` lines 166–167: `applicable_rituals: Option<Vec<String>>` with serde(default, skip_serializing_if) |
| INT-03 | 15-02 | NodeConcept::Ritual, NodeConcept::FlyingStar, EdgeConcept::PrescribedFor, OccupiesPalace, CarriesElement in semantic graph | SATISFIED | `ontology.rs` all six locations; test `v15_concepts_present_in_ontology_slices` passes |
| INT-04 | 15-03 | Source-disjoint pillar nodes + multi-source Direction node in graph builder | SATISFIED | `day_snapshot.rs` builder: FlyingStar=huyen-khong only, Ritual=vn-folk-ritual only, Direction has len==2 provenance; test passes |
| INT-05 | 15-04 | v1.4 JSON fixture loads into v1.5 structs; byte-equal round-trip | SATISFIED | All 3 tests in `day_snapshot_v14_compat.rs` pass |
| INT-06 | 15-04 | >=30 representative 2026 dates smoke test with Tết, Sóc/Vọng, Vận, leap month, 24 Tiết Khí | SATISFIED | All 3 tests in `integration_2026_smoke.rs` pass; date count >= 30 asserted at runtime |

All 6 requirements mapped to Phase 15 in REQUIREMENTS.md are SATISFIED. No orphaned requirements.

---

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| — | — | — | — | None found |

No anti-patterns found in any of the five key files scanned. No TODOs, FIXMEs, placeholder returns, or empty implementations. Bare source-id string literals ("huyen-khong", "vn-folk-ritual") appear only inside `#[cfg(test)]` blocks in the builder (line 552 marks the test module boundary), which is the explicitly permitted pattern per the source_id_guard contract.

---

### Human Verification Required

None. All success criteria are verifiable programmatically and all tests pass.

---

### Test Results Summary

| Test | Result |
|------|--------|
| `cargo build -p amlich-core` | PASS (0.12s, no errors) |
| `tests::day_snapshot_populates_additive_surfaces` (lib) | PASS |
| `semantic_graph::ontology::tests::v15_concepts_present_in_ontology_slices` (lib) | PASS |
| `semantic_graph::builders::day_snapshot::tests::v15_pillar_nodes_carry_disjoint_source_ids_and_direction_is_multi_source` (lib) | PASS |
| `tests/day_snapshot_v14_compat.rs` — 3 tests (INT-05) | PASS (3/3) |
| `tests/integration_2026_smoke.rs` — 3 tests (INT-06) | PASS (3/3) |
| `tests/source_id_guard.rs` — source ID discipline | PASS |
| Full suite `cargo test -p amlich-core` | PASS (696 lib + all integration tests, 0 failures) |

---

### Gaps Summary

None. All five observable truths are verified, all artifacts are substantive and wired, all six requirements are satisfied, and the complete test suite is green with no regressions.

---

_Verified: 2026-05-28_
_Verifier: Claude (gsd-verifier)_
