---
phase: 20-foundation-schema-lock-source-ids-adrs-ontology
verified: 2026-07-16T03:05:00Z
status: passed
score: 13/13 truths verified (4/4 success criteria)
re_verification: false
---

# Phase 20: Foundation — Schema Lock + Source IDs + ADRs + Ontology Verification Report

**Phase Goal:** User-of-foundation can find the v1.7 IChing pillar and cross-link fully scaffolded at the type/ADR/ontology level BEFORE any of the 64 corpus entries are authored — source IDs registered and CI-guarded, ADRs accepted, schema locked with a passing 1-entry serde round-trip probe, the typed trigram/hexagram newtype boundary enforced by the compiler, and the 6-slice ontology extended.
**Verified:** 2026-07-16T03:05:00Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | `SOURCE_KINH_DICH = "kinh-dich"` + `SOURCE_MAI_HOA_DICH_SO = "mai-hoa-dich-so"` as `pub const` in sources.rs | ✓ VERIFIED | `crates/amlich-core/src/sources.rs:29,32`; both consts declared with v1.7 doc-comments |
| 2 | `source_id_guard.rs::FORBIDDEN_LITERALS` extended so bare literals fail CI at provenance call-sites | ✓ VERIFIED | `crates/amlich-core/tests/source_id_guard.rs:21-22` (9 entries total); `cargo test -p amlich-core --test source_id_guard` passes (1 test) |
| 3 | Three accepted ADRs in `.planning/adrs/` (ADR-0005, ADR-0006, ADR-0007) in Nygard short-form | ✓ VERIFIED | All three files exist with `**Status:** Accepted` + Title/Context/Decision/Consequences sections |
| 4 | Three new DEC-0026/0027/0028 rows in MILESTONES.md cross-referencing the three ADRs | ✓ VERIFIED | `.planning/MILESTONES.md:280-282` (DEC-0026→0005, DEC-0027→0006, DEC-0028→0007) |
| 5 | Locked `HexagramEntry` with `#[serde(deny_unknown_fields)]` that round-trips BEFORE any of the 64 corpus entries are authored | ✓ VERIFIED | `schema.rs:283-284` (deny_unknown_fields at struct level); 64 corpus entries absent from `data/iching/` (only `.gitkeep` present) |
| 6 | Three distinct newtypes — `TienThienTrigram`, `HauThienTrigram`, `KingWenHexagram` — with NO `From` impl between them (CRIT-3) | ✓ VERIFIED | `schema.rs:45-163`; `rg "impl From<(TienThienTrigram\|HauThienTrigram\|KingWenHexagram)"` returns only doc-comments asserting absence (3 matches, all `///` lines) — compiler-enforced |
| 7 | 64-entry Tiên Thiên-pair → King Wen composition table validates bijective at load (`cargo test`) | ✓ VERIFIED | `schema.rs:182-247` (64 hand-authored entries); `composition_table_is_bijective` test passes (asserts len==64, distinct HashSet, exhaustive 8×8 surjectivity) |
| 8 | 1-entry serde round-trip probe passes with hexagram #2 Khôn (7 `hao_tu`, NFC diacritics, `DeferralMarker`) | ✓ VERIFIED | `tests/iching_schema_probe.rs` — 4 tests pass: round-trip, deny_unknown_fields rejection, reserved `*_en` absent→None, Hậu Thiên snake_case deserialise |
| 9 | `NodeConcept::Hexagram` across all 6 ontology slices with compiler-enforced exhaustiveness, NO `#[non_exhaustive]` | ✓ VERIFIED | `ontology.rs`: slice1 enum (line 43), slice2 label() (line 86), slice3 ConceptLabel enum (line 234), slice4 as_str() (line 308), slice5 node_concepts() (line 411); `rg "#\[non_exhaustive\]" ontology.rs` returns nothing |
| 10 | `EdgeConcept::LocatedAt` + `EdgeConcept::Transforms` across their 6 edge-slice locations | ✓ VERIFIED | `ontology.rs`: enum (lines 123-124), label() (lines 159-160), ConceptLabel (lines 235-236), as_str() (lines 309-310), edge_concepts() (lines 446-447) |
| 11 | `ReasoningEvidenceSourceFamily::IChing` + `ActionId::IChing` variants in reasoning/types.rs | ✓ VERIFIED | `reasoning/types.rs:7` (`ActionId::IChing`), `reasoning/types.rs:143` (`ReasoningEvidenceSourceFamily::IChing`); both enums carry `#[serde(rename_all = "snake_case")]` so serialise to `"i_ching"` |
| 12 | A v1.7 ontology test asserting Hexagram/LocatedAt/Transforms presence + label round-trips | ✓ VERIFIED | `ontology.rs:349-368` (`v17_concepts_present_in_ontology_slices`); test runs + passes (1 test, 721 filtered out) |
| 13 | Whole crate compiles — all exhaustive match arms in semantic_graph/views/ updated for the new NodeConcept variant | ✓ VERIFIED | `helpers.rs:45` (`Ritual \| FlyingStar \| Offering \| Hexagram => "day-core"`); `visualization.rs:117` (`... \| Hexagram => Some("box")`); `cargo build -p amlich-core` exits 0 |

**Score:** 13/13 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `crates/amlich-core/src/sources.rs` | `SOURCE_KINH_DICH` + `SOURCE_MAI_HOA_DICH_SO` pub const | ✓ VERIFIED | Lines 29 + 32; both consts declared; `all_constants_have_expected_values` test extended (lines 63-64); test passes |
| `crates/amlich-core/tests/source_id_guard.rs` | `FORBIDDEN_LITERALS` extended with two new source ids | ✓ VERIFIED | Lines 21-22; 9 entries total; guard test passes |
| `.planning/adrs/0005-hexagram-entry-schema-v1.md` | Accepted HexagramEntry schema v1 decision | ✓ VERIFIED | 143 lines; Status: Accepted; contains `deny_unknown_fields`, `hao_tu` length rule (§2), naming divergence (§3), reviewer free-text (§4), Lo Shu pin (§5) |
| `.planning/adrs/0006-mai-hoa-casting-convention.md` | Accepted Mai Hoa casting convention decision | ✓ VERIFIED | 169 lines; Status: Accepted; contains Thiệu Khang Tiết citation + nhantu.net; `((n-1)%k)+1` worked boundary example with `(23%8)+1 = 7+1 = 8` arithmetic (§4) |
| `.planning/adrs/0007-cross-link-crit3-carve-out.md` | Accepted cross-link CRIT-3 carve-out decision | ✓ VERIFIED | 106 lines; Status: Accepted; contains `build_direction_cross_link` placement, `rule.composite.direction_cross_link` envelope pattern, sibling grep guard |
| `.planning/MILESTONES.md` | DEC-0026/0027/0028 ADR cross-reference rows | ✓ VERIFIED | Lines 280-282; no DEC collision (DEC-0025 was highest prior) |
| `crates/amlich-core/src/iching/schema.rs` | `HexagramEntry` + 3 newtypes + `COMPOSITION_TABLE` + `compose()` | ✓ VERIFIED | 429 lines; contains `deny_unknown_fields` (line 284); 64-entry composition table (lines 182-247); 5 inline tests pass |
| `crates/amlich-core/src/iching/mod.rs` | Module re-export surface | ✓ VERIFIED | 17 lines; `pub use schema::{compose, HauThienTrigram, HexagramEntry, KingWenHexagram, TienThienTrigram, COMPOSITION_TABLE}` |
| `crates/amlich-core/src/lib.rs` | `pub mod iching;` registration | ✓ VERIFIED | Line 18 (`pub mod iching;`); alphabetically positioned |
| `crates/amlich-core/tests/iching_schema_probe.rs` | 1-entry serde round-trip probe | ✓ VERIFIED | 220 lines (≥ 40-line minimum); 4 integration tests pass |
| `crates/amlich-core/data/iching/.gitkeep` | Reserved corpus directory for Phase 21 | ✓ VERIFIED | File present (169 bytes); directory reserved |
| `crates/amlich-core/src/semantic_graph/ontology.rs` | 6-slice extension for Hexagram node + LocatedAt/Transforms edges + v1.7 test | ✓ VERIFIED | 450 lines; Hexagram/LocatedAt/Transforms present in all 6 slices; v1.7 test passes |
| `crates/amlich-core/src/reasoning/types.rs` | `IChing` variants on ActionId + ReasoningEvidenceSourceFamily | ✓ VERIFIED | Line 7 (ActionId::IChing), line 143 (ReasoningEvidenceSourceFamily::IChing) |
| `crates/amlich-core/src/semantic_graph/views/helpers.rs` | `cluster_for_node_id` arm for Hexagram | ✓ VERIFIED | Line 45: `Ritual \| FlyingStar \| Offering \| Hexagram => "day-core"` |
| `crates/amlich-core/src/semantic_graph/views/visualization.rs` | `shape_hint_for_node` arm for Hexagram | ✓ VERIFIED | Line 117: `Ritual \| FlyingStar \| Offering \| Hexagram => Some("box")` |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|----|--------|---------|
| `crates/amlich-core/src/sources.rs` | `tests/source_id_guard.rs` | FORBIDDEN_LITERALS mirrors every SOURCE_* const value | ✓ WIRED | Both `kinh-dich` + `mai-hoa-dich-so` appear in FORBIDDEN_LITERALS (lines 21-22); guard test passes |
| `.planning/adrs/0005-hexagram-entry-schema-v1.md` | `.planning/MILESTONES.md` | DEC-0026 row cross-references the ADR | ✓ WIRED | MILESTONES.md:280 links to `adrs/0005-hexagram-entry-schema-v1.md` |
| `crates/amlich-core/src/iching/schema.rs` | `crates/amlich-core/src/almanac/fengshui/golden.rs` | import `DeferralMarker` for `pending_review` field | ✓ WIRED | `schema.rs:17` — `use crate::almanac::fengshui::golden::DeferralMarker;` |
| `crates/amlich-core/src/iching/schema.rs` | `crates/amlich-core/src/lib.rs` | `pub mod iching;` makes schema accessible at crate root | ✓ WIRED | `lib.rs:18` — `pub mod iching;` |
| `crates/amlich-core/tests/iching_schema_probe.rs` | `crates/amlich-core/src/iching/schema.rs` | probe imports `HexagramEntry` + newtypes + round-trips | ✓ WIRED | `iching_schema_probe.rs:28` — `use amlich_core::iching::{HauThienTrigram, HexagramEntry, KingWenHexagram};` |
| `crates/amlich-core/src/semantic_graph/ontology.rs` | `crates/amlich-core/src/semantic_graph/views/helpers.rs` | `cluster_for_node_id` exhaustive match gained a Hexagram arm | ✓ WIRED | `helpers.rs:45` includes `NodeConcept::Hexagram` |
| `crates/amlich-core/src/semantic_graph/ontology.rs` | `crates/amlich-core/src/semantic_graph/views/visualization.rs` | `shape_hint_for_node` exhaustive match gained a Hexagram arm | ✓ WIRED | `visualization.rs:117` includes `NodeConcept::Hexagram` |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|------------|-------------|--------|----------|
| FND-09 | 20-01 | Two new `pub const` source_ids + CI guard extension | ✓ SATISFIED | `sources.rs:29,32` register both consts; `source_id_guard.rs:21-22` extend FORBIDDEN_LITERALS to 9 entries; both tests pass |
| FND-10 | 20-01 | Three accepted ADRs (0005 schema, 0006 casting, 0007 cross-link) | ✓ SATISFIED | All three ADRs exist with Status: Accepted; ADR-0005 carries deny_unknown_fields + hao_tu rule; ADR-0006 carries Tiên Thiên pin + `((n-1)%k)+1` + lunar inputs; ADR-0007 carries `reasoning/direction_composite.rs` placement + composite envelope |
| FND-11 | 20-02 | Locked HexagramEntry + 1-entry probe + 3 newtypes (no From) + 64-entry bijective composition table | ✓ SATISFIED | `schema.rs` ships all elements; 4-test probe passes; bijectivity test passes; `rg` confirms no cross-newtype From impl |
| FND-12 | 20-03 | 6-slice ontology extension (Hexagram/LocatedAt/Transforms) + IChing enum variants | ✓ SATISFIED | `ontology.rs` extended across all 6 slices; `reasoning/types.rs:7,143` add IChing variants; v1.7 ontology test passes; crate compiles with no `#[non_exhaustive]` |

**Orphaned requirements:** None. All four phase-20 requirement IDs (FND-09, FND-10, FND-11, FND-12) appear in plan frontmatter AND are satisfied by implementation evidence. REQUIREMENTS.md Traceability table marks all four as Complete (✓ matching `cargo test` reality).

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| (none) | — | — | — | No TODO/FIXME/PLACEHOLDER/HACK, no `unimplemented!`/`todo!`, no `return null`/`return []`/`=> {}` stubs, no "coming soon" markers found in any Phase 20 file |

ℹ️ **Pre-existing warnings (NOT introduced by Phase 20):** `cargo build` reports 3 unused-import warnings (`ProvenanceSource` in `helpers.rs:115`, `ReasoningNodeSeverity`, `GraphValidationError`) — all in pre-existing test code, not in any Phase 20-modified file. Severity: ℹ️ Info. No action required for this phase.

### Automated Test Results

| Suite | Result | Notes |
|-------|--------|-------|
| `cargo test -p amlich-core --lib` | 722 passed / 0 failed | Includes new `all_constants_have_expected_values` (9 consts), `v17_concepts_present_in_ontology_slices`, 5 iching schema tests |
| `cargo test -p amlich-core --test iching_schema_probe` | 4 passed / 0 failed | Round-trip + deny_unknown_fields rejection + reserved *_en None + Hậu Thiên snake_case |
| `cargo test -p amlich-core --test source_id_guard` | 1 passed / 0 failed | 9 FORBIDDEN_LITERALS enforced |
| `cargo build -p amlich-core` | exit 0 | Crate compiles cleanly (3 pre-existing unused-import warnings only) |
| `cargo tree -p amlich-core --depth 1` | unchanged | Only `chrono`, `serde`, `serde_json`, `unicode-normalization` — no new crates |

### Task Commits Verified

All 9 task commits + 3 docs commits present in `git log`:
- 20-01: `4eff1d4` (test RED), `cbfbcdb` (feat GREEN), `370a486` (docs ADRs)
- 20-02: `99efa74` (test RED), `da20ce3` (feat GREEN), `c35d8c8` (test probe)
- 20-03: `7f4c562` (test RED), `668dcbc` (feat GREEN), `06cb209` (feat IChing variants)
- Docs: `f9fc111`, `afd4a7e`, `5f8d125` (plan completion commits)

### Human Verification Required

None required. All Phase 20 deliverables are type-level / CI-guarded / proven by automated tests:
- Source IDs + guard: enforced by `cargo test --test source_id_guard` (CI gate)
- ADRs: prose substance verified by grep for required tokens (deny_unknown_fields, Thiệu Khang Tiết, `((n-1)%k)+1`, direction_cross_link) — all present
- Schema lock: 1-entry serde round-trip probe is the CRIT-1 gate (4 tests, passes)
- Newtype CRIT-3 isolation: structurally enforced by compiler (no `From` impl = grep-verified)
- 6-slice ontology: compiler-enforced exhaustiveness (crate compiles, no `#[non_exhaustive]`)

The CRIT-2 worked boundary example in ADR-0006 §4 is self-contained — a reader can verify `(23 % 8) + 1 = 8` arithmetic by inspection (no external source lookup needed for the boundary proof).

### Gaps Summary

No gaps found. All 13 observable truths verified, all 15 required artifacts pass the three levels (exists + substantive + wired), all 7 key links wired, all 4 requirements satisfied with implementation evidence, no anti-patterns, full test suite green with zero regressions, and no new crate dependencies introduced.

Phase 20 achieves its goal: the v1.7 IChing pillar and cross-link are fully scaffolded at the type/ADR/ontology level BEFORE any of the 64 corpus entries are authored. The foundation is locked and ready for Phase 21 corpus authoring.

---

_Verified: 2026-07-16T03:05:00Z_
_Verifier: Claude (gsd-verifier)_
