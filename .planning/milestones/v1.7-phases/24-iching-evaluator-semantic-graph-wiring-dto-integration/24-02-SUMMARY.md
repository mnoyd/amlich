---
phase: 24-iching-evaluator-semantic-graph-wiring-dto-integration
plan: 02
subsystem: semantic-graph
tags: [iching, semantic-graph, hexagram, transforms, located-at, day-snapshot, kinh-dich, mai-hoa-dich-so, crit-3, crit-6, crit-4, dual-source-provenance, role-bearing-stable-key, direction-cross-link, adr-0007, wasm-safe]

# Dependency graph
requires:
  - phase: 24-iching-evaluator-semantic-graph-wiring-dto-integration (24-01)
    provides: "IChingCastSummary (slim owned DTO) + IChingEvaluator (rich path) + enrich_day_snapshot_with_iching immutable clone-and-attach helper + DaySnapshot.iching_cast additive field + ProvenanceSource::IChing variant"
  - phase: 23-th-i-tu-tam-s-t-phi-tinh-cross-link (23-02 + 23-03)
    provides: "DaySnapshot.direction_cross_link additive field + DirectionCrossLinkSummary + enrich_day_snapshot_with_direction_cross_link helper (Phase 23-02 + 23-03 already shipped — Task 2's forward-compatibility placeholder was unnecessary because Phase 23 shipped before this plan executed)"
  - phase: 22-mai-hoa-casting-bien-que-the-dung (22-01 + 22-02)
    provides: "MaiHoaCast + BienQue + cast_mai_hoa + derive_bien_que + classify_the_dung + 12-case golden dataset (Phase 22 SC4 + INT-13 cross-source gate)"
  - phase: 21-iching-corpus-loader (21-02)
    provides: "OnceLock-cached 64-hexagram Ngô Tất Tố corpus + get_hexagram(KingWenHexagram)"
  - phase: 20-foundation-schema-lock-source-ids-adrs-ontology (20-02 + 20-03)
    provides: "SOURCE_KINH_DICH + SOURCE_MAI_HOA_DICH_SO consts (FND-09) + three CRIT-3-isolating newtypes + NodeConcept::Hexagram + EdgeConcept::LocatedAt/Transforms ontology variants (FND-12)"
provides:
  - "INT-11 partial close (IChing portion): DaySnapshotGraphBuilder::add_iching_facts emits 2 distinct NodeConcept::Hexagram nodes (primary chu + bien) wired via 1 EdgeConcept::Transforms + 2 EdgeConcept::LocatedAt edges with CRIT-6 dual-source provenance"
  - "INT-11 partial close (directional portion): DaySnapshotGraphBuilder::add_direction_composite_facts emits 1 NodeConcept::Direction composite fact node with KHCBPPT + Huyền-Không primitives + ONE composite envelope (Phase 23's locked contract)"
  - "SemanticId::iching_hexagram(role, king_wen, date, tz) constructor producing role-bearing stable keys ('hexagram:iching:chu|bien:<kw>:<date>:<tz>')"
  - "IChingCastSummary::chu_king_wen_index() + bien_king_wen_index() accessor helpers (so the builder reads structured King Wen indices without re-deriving them)"
  - "DaySnapshotGraphBuilder::new dispatch wires both add_iching_facts() + add_direction_composite_facts() at the end of the existing additive pattern (after add_offering_facts)"
affects:
  - 24-iching-evaluator-semantic-graph-wiring-dto-integration (24-03 — combined-strip v1.6→v1.7 round-trip test that closes INT-12 fully)
  - 25-e2e-validation-golden-cross-source-verification (INT-13 E2E consumes the semantic-graph Hexagram wiring + the combined-strip round-trip)

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Role-bearing stable-key pattern: 'iching:{role}:{king_wen}:{date}:{tz}' with `role in {chu, bien}` so the primary + transformed hexagrams cannot collide in graph.node_count(). Mirrors v1.6/v1.7 sibling-DTO discipline (locked pitfall P-3)"
    - "Edge insertion order discipline: BOTH endpoint Hexagram nodes added BEFORE any edges (Transforms + LocatedAt). `SemanticGraph::add_edge` silently drops edges with missing endpoints (semantic_graph/graph.rs:23-28) — the integration test's iching_graph_edges_only_present_after_nodes_inserted test pins this"
    - "Phase 23 forward-compatibility resolved cleanly: DirectionCrossLinkSummary already shipped by Phase 23-02 (the field declaration lives at crates/amlich-core/src/lib.rs:201 referencing crate::reasoning::DirectionCrossLinkSummary directly). Plan 24-02's 'add a placeholder type' was a no-op because Phase 23 fully shipped before this plan executed — the placeholder was never created and was never deleted"
    - "CRIT-6 dual-source provenance: each Hexagram node carries 2 entries — SOURCE_MAI_HOA_DICH_SO (cast_mai_hoa / derive_bien_que method) + SOURCE_KINH_DICH (corpus_lookup method). Never bare literals at production call-sites — uses the registered consts"
    - "CRIT-3 isolation preserved by doc-comment scrubbing: the grep guard test (iching_graph_no_flyingstar_in_iching_method) reads the literal 'FlyingStar' substring inside the method body. Doc-comments had to use phrase-level names ('the v1.5 Phi Tinh aggregator surface') instead of the literal type name to avoid self-tripping the guard. Mirrors the v1.6/v1.7 discipline codified across corpus.rs / mai_hoa.rs / bien_que.rs / the_dung.rs / golden.rs / evaluator.rs"

key-files:
  created:
    - crates/amlich-core/tests/semantic_graph_iching_integration.rs
  modified:
    - crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs
    - crates/amlich-core/src/semantic_graph/ids.rs
    - crates/amlich-core/src/iching/evaluator.rs

key-decisions:
  - "add_iching_facts implementation mirrors the v1.6 FlyingStar/Offering precedent (add_flying_star_facts + add_offering_facts pattern): date_str from snapshot.context.solar.{year,month,day}, tz_suffix=self.tz_suffix, stable key encoding role-bearing King Wen index for collision-resistance"
  - "Dual-source provenance (CRIT-6) on each Hexagram node: SOURCE_MAI_HOA_DICH_SO entry with method 'iching.cast_mai_hoa' (primary) / 'iching.derive_bien_que' (bien) + SOURCE_KINH_DICH entry with method 'iching.corpus_lookup'. Both entries use the registered SOURCE_* consts — no bare literals at production call-sites"
  - "Transforms edge direction: chu → bien (primary → transformed). Locked in test #2 (iching_graph_has_transforms_edge_between_chu_and_bien)"
  - "Edge insertion order discipline: both Hexagram nodes are added BEFORE the Transforms + LocatedAt edges. `SemanticGraph::add_edge` silently drops edges whose endpoint nodes are missing (semantic_graph/graph.rs:23-28). The test suite explicitly asserts edge presence to guard against regression"
  - "add_direction_composite_facts uses Phase 23's real DirectionCrossLinkSummary (no placeholder declared, no placeholder deleted — Phase 23-02 shipped the field + type before this plan executed). The composite node's `cross_link_source` field carries the composite envelope's source_id (typically `rule.composite.direction_cross_link` per ADR-0007)"
  - "CRIT-3 isolation preserved by doc-comment scrubbing: the grep guard test reads the literal 'FlyingStar' substring inside add_iching_facts + add_direction_composite_facts method bodies. Initial GREEN-phase doc-comments used the literal '`FlyingStar`' when describing what the methods DON'T reference; the grep guard correctly flagged this as a self-tripping false-positive. Fixed by rephrasing to phrase-level ('the v1.5 Phi Tinh aggregator surface') — mirrors the runtime-built needle pattern from v1.6/v1.7 (Phase 21-02 corpus.rs, Phase 22-02 golden.rs, Phase 24-01 evaluator.rs)"
  - "direction_composite_facts_wires_populated_state test is ACTIVE (not #[ignore]'d): Phase 23 fully shipped DirectionCrossLinkSummary + enrich_day_snapshot_with_direction_cross_link helper before Plan 24-02 executed. The test exercises the end-to-end composite wiring (KHCBPPT + huyen-khong + rule.composite. direction_cross_link + LocatedAt edge to day root) and verifies the node id is distinct from the daily travel direction node"

patterns-established:
  - "Two-Node + Transforms + LocatedAt pattern for transformation graphs: primary node carries 'role=chu' tag, transformed node carries 'role=bien' tag, both anchored at the day_root via LocatedAt edges. Future transformation-type domains (e.g. Bazi day→year pillar transformation) can follow this pattern"
  - "Forward-compatibility resolution via Phase shipping: when Phase X already ships a field that Plan Y was supposed to declare as a placeholder, Plan Y's placeholder work is a no-op (no declaration, no deletion). Document the deviation explicitly in the SUMMARY"

requirements-completed: [INT-11]

# Metrics
duration: 9min
completed: 2026-07-16
---
# Phase 24 Plan 02: IChing Semantic-Graph Wiring + Directional Composite Summary

**Additive `DaySnapshotGraphBuilder::add_iching_facts()` + `add_direction_composite_facts()` methods wire the v1.7 evaluation-layer output into the semantic-graph substrate: 2 distinct `NodeConcept::Hexagram` nodes (primary chủ quẻ + biến quẻ) connected by `EdgeConcept::Transforms` + `EdgeConcept::LocatedAt` edges with CRIT-6 dual-source provenance (`mai-hoa-dich-so` + `kinh-dich`), plus 1 `NodeConcept::Direction` composite fact node for the Phase 23 directional cross-link (KHCBPPT + Huyền-Không primitives + one `rule.composite.direction_cross_link` composite envelope). Closes INT-11.**

## Performance

- **Duration:** 9 min 3 s (543 s)
- **Started:** 2026-07-16T17:49:01Z
- **Completed:** 2026-07-16T17:58:04Z
- **Tasks:** 2 (Task 1 = TDD red→green for IChing portion; Task 2 = directional composite wiring — merged into Task 1 GREEN because Phase 23 already shipped the field + type, making Task 2's "forward-compatible placeholder" a no-op)
- **Task commits:** 2 (RED `278f4a5` + GREEN `46ad421` covering both Task 1 + Task 2 surfaces)
- **Files created:** 1 (`semantic_graph_iching_integration.rs`)
- **Files modified:** 3 (`semantic_graph/builders/day_snapshot.rs`, `semantic_graph/ids.rs`, `iching/evaluator.rs`)
- **Net tests added:** 13 (all in the new `semantic_graph_iching_integration.rs` integration suite — 9 IChing + 4 directional composite)
- **Crate test suite:** 1114 passing tests across 49 test groups, 0 failures, 0 regressions vs Plan 24-01's 1101 baseline (+13 net additions)

## Accomplishments

- **`crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs`** (modified, +~200 lines):
  - New `fn add_iching_facts(&mut self, snapshot: &DaySnapshot)` method on `DaySnapshotGraphBuilder`:
    - Early-returns when `snapshot.iching_cast.is_none()` (no implicit wiring on ordinary snapshots)
    - When `snapshot.iching_cast = Some(summary)`, emits EXACTLY 2 `NodeConcept::Hexagram` nodes:
      - **Primary chủ** with stable key `hexagram:iching:chu:<kw>:<date>:<tz>`, tags `king_wen=<N>` + `role=chu` + `verdict=<cat|binh|hung>` + `moving_line=<N>`
      - **Biến** with stable key `hexagram:iching:bien:<kw>:<date>:<tz>`, tags `king_wen=<N>` + `role=bien` + `flipped_dong_hao=<N>`
    - Each Hexagram node carries CRIT-6 dual-source provenance (2 entries):
      - `SOURCE_MAI_HOA_DICH_SO` entry (`iching.cast_mai_hoa` / `iching.derive_bien_que` method per role)
      - `SOURCE_KINH_DICH` entry (`iching.corpus_lookup` method)
    - Emits 1 `EdgeConcept::Transforms` edge (primary chủ → biến) + 2 `EdgeConcept::LocatedAt` edges (each Hexagram → day_root)
    - Edge insertion order discipline: both nodes added BEFORE the edges (prevents silent drops by `SemanticGraph::add_edge`)
  - New `fn add_direction_composite_facts(&mut self, snapshot: &DaySnapshot)` method:
    - Early-returns when `snapshot.direction_cross_link.is_none()` (the IChing-only enrichment path does NOT auto-infer directional wiring — opt-in via Phase 23's `enrich_day_snapshot_with_direction_cross_link`)
    - When `snapshot.direction_cross_link = Some(cross)`, emits 1 `NodeConcept::Direction` composite fact node with stable key `direction:cross_link:<date>:+7` — DISTINCT from the existing daily travel direction node (`direction:travel:day:+7:all`)
    - The composite node carries 3 provenance entries (Phase 23's locked CRIT-6 dual-source pattern):
      - `SOURCE_KHCBPPT` primitive (`thai_tue_tam_sat_directional` method)
      - `SOURCE_HUYEN_KHONG` primitive (`phi_tinh.palace_overlay` method)
      - 1 composite envelope sourced from `DirectionCrossLinkSummary.cross_link_source` (typically `rule.composite.direction_cross_link` per ADR-0007)
    - Emits 1 `EdgeConcept::LocatedAt` edge (cross_link → day_root)
  - Both methods wired into `DaySnapshotGraphBuilder::new` dispatch at the end of the additive pattern (after `add_offering_facts(snapshot)`)
- **`crates/amlich-core/src/semantic_graph/ids.rs`** (modified, +11 lines):
  - New `SemanticId::iching_hexagram(role, king_wen, date, tz)` constructor producing role-bearing stable keys with shape `"hexagram:iching:{role}:{king_wen}:{date}:{tz}"`. The role parameter (`"chu"` vs `"bien"`) is the FIRST segment of the stable key so the primary + transformed hexagrams cannot collide in `graph.node_count()`. Replaces the RED-phase `unimplemented!()` stub
- **`crates/amlich-core/src/iching/evaluator.rs`** (modified, +21 lines):
  - New `impl IChingCastSummary` block with 2 accessor methods:
    - `fn chu_king_wen_index(&self) -> u8` — returns `self.cast.chu_que.0` (the primary chủ quẻ's King Wen index, 1..=64)
    - `fn bien_king_wen_index(&self) -> u8` — returns `self.bien_que.king_wen.0` (the biến quẻ's King Wen index, 1..=64)
  - The builder reads these accessors instead of re-deriving the King Wen indices from `cast.chu_que.0` / `bien_que.king_wen.0` directly — a small ergonomic seam that keeps the builder readable
- **`crates/amlich-core/tests/semantic_graph_iching_integration.rs`** (created, ~440 lines) — 13 black-box integration tests covering all INT-11 success criteria:
  1. `iching_graph_has_two_distinct_hexagram_nodes_when_enriched` — exactly 2 Hexagram nodes with distinct node_ids after `enrich_day_snapshot_with_iching`
  2. `iching_graph_has_transforms_edge_between_chu_and_bien` — exactly 1 `EdgeConcept::Transforms` edge connecting the two Hexagram nodes
  3. `iching_graph_has_located_at_edges_from_each_hexagram_to_day_root` — exactly 2 `EdgeConcept::LocatedAt` edges (one per Hexagram → day_root)
  4. `iching_graph_hexagram_nodes_carry_dual_source_provenance` — each Hexagram has exactly 2 provenance entries (one `SOURCE_MAI_HOA_DICH_SO` + one `SOURCE_KINH_DICH`)
  5. `iching_graph_hexagram_stable_keys_are_role_bearing` — node ids match `"hexagram:iching:chu:<kw>:..."` and `"hexagram:iching:bien:<kw>:..."` patterns
  6. `iching_graph_ordinary_snapshot_has_no_hexagram_nodes` — ordinary `calculate_day_snapshot(...)` produces ZERO Hexagram nodes (no implicit wiring)
  7. `iching_graph_no_flyingstar_in_iching_method` — CRIT-3 grep guard: `"FlyingStar"` substring does NOT appear in the `add_iching_facts` method body
  8. `iching_graph_hexagram_node_tags_include_king_wen_and_role` — both Hexagram nodes' tags contain `"king_wen=<N>"` AND `"role=chu"` / `"role=bien"`
  9. `iching_graph_edges_only_present_after_nodes_inserted` — edge count strictly increases after enrichment; ordinary snapshots have ZERO Transforms edges (sanity check)
  10. `iching_only_enrichment_does_not_wire_directional_composite` — `enrich_day_snapshot_with_iching` does NOT populate `direction_cross_link`; the graph contains ZERO `cross_link:` stable-key nodes from this method
  11. `direction_composite_facts_wires_populated_state` — **ACTIVE** (not `#[ignore]`'d) — verifies the end-to-end directional composite wiring: KHCBPPT + huyen-khong + `rule.composite.*` composite envelope + LocatedAt edge to day root. The cross-link node id is distinct from the daily travel direction node
  12. `direction_cross_link_absent_in_ordinary_snapshot_json` — additive DTO discipline: `"direction_cross_link"` key is absent from JSON when None
  13. `direction_cross_link_round_trip_when_populated` — byte-equal serde round-trip when the field is populated; the key appears in JSON when Some
- **TDD discipline observed:** RED commit `278f4a5` (10 of 13 tests panic with "not implemented: RED phase: DaySnapshotGraphBuilder::add_iching_facts"; 3 pass because they exercise pre-existing JSON-serialisation or the Phase 23-shipped `direction_cross_link` field); GREEN commit `46ad421` (full implementation; all 13 tests pass; 0 regressions). Two atomic commits
- **CRIT-3 isolation preserved across the new module:** `rg "FlyingStar" crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs` returns matches ONLY in `add_flying_star_facts` (lines 499, 509, 519) + the inline tests (lines 1186+). Zero matches in `add_iching_facts` (line 780) or `add_direction_composite_facts` (line 894). The integration test's `iching_graph_no_flyingstar_in_iching_method` grep guard pins this discipline at runtime
- **CRIT-6 source-id discipline preserved:** the production call-sites use `SOURCE_MAI_HOA_DICH_SO` and `SOURCE_KINH_DICH` consts (no bare literals). `tests/source_id_guard.rs` still passes (1/1)
- **CRIT-4 isolation preserved** at the stable-key boundary: the biến Hexagram node uses `summary.bien_que.king_wen.0` (the biến's own King Wen index) as the stable key, NOT the primary's index. The two node ids cannot collide even when the cast happens to produce chu == biến in degenerate cases (which CRIT-4 prohibits via bijectivity, but the role-bearing stable key is the structural defense)
- **WASM-safety + determinism discipline preserved:** `rg "rand::|Utc::now|std::fs::" crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs` returns ZERO matches (filesystem-free, wall-clock-free, RNG-free)
- **No new crate dependencies:** `cargo tree -p amlich-core --depth 1` shows the existing `chrono` + `serde` + `serde_json` + `unicode-normalization` set unchanged
- **Full crate test result:** 1114 passing tests across 49 test groups, 0 failures, 0 regressions vs Plan 24-01's 1101-test baseline (+13 net additions = 13 new integration tests). `cargo build -p amlich-core` clean. `cargo test -p amlich-core --test semantic_graph_iching_integration` → 13/13 pass

## Task Commits

Each task was committed atomically (Task 1's RED → GREEN pair was preserved; Task 2's work was merged into Task 1 GREEN because Phase 23 had already shipped the field + type, making Task 2's "forward-compatible placeholder" a no-op):

1. **Task 1 RED — failing tests for IChing + directional composite semantic-graph wiring** — `278f4a5` (test)
   - `crates/amlich-core/src/semantic_graph/ids.rs` — new `SemanticId::iching_hexagram` constructor stub returning `unimplemented!("RED phase: SemanticId::iching_hexagram")`
   - `crates/amlich-core/src/iching/evaluator.rs` — new `impl IChingCastSummary` block with `chu_king_wen_index()` + `bien_king_wen_index()` accessor stubs (real implementations — they were in the same commit because the accessors' implementation is trivial)
   - `crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs` — wires `add_iching_facts(snapshot)` + `add_direction_composite_facts(snapshot)` into `DaySnapshotGraphBuilder::new` dispatch with RED-phase `unimplemented!()` bodies
   - `crates/amlich-core/tests/semantic_graph_iching_integration.rs` (NEW, ~440 lines) — 13 black-box integration tests covering all INT-11 success criteria
   - 10 of 13 tests fail with "not implemented: RED phase: DaySnapshotGraphBuilder::add_iching_facts" (the 3 that pass test JSON-level behavior or the Phase-23-shipped `direction_cross_link` field)
2. **Task 1+2 GREEN — full implementation of IChing + directional composite wiring** — `46ad421` (feat)
   - `crates/amlich-core/src/semantic_graph/ids.rs` — real `SemanticId::iching_hexagram` constructor producing `"hexagram:iching:{role}:{king_wen}:{date}:{tz}"` stable keys
   - `crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs`:
     - Real `add_iching_facts` impl: 2 Hexagram nodes with role-bearing stable keys + dual-source provenance (CRIT-6) + 1 Transforms edge + 2 LocatedAt edges + edge insertion order discipline (nodes before edges)
     - Real `add_direction_composite_facts` impl: 1 Direction composite node with 3-provenance (KHCBPPT + huyen-khong + composite envelope from `cross_link_source`) + 1 LocatedAt edge to day_root
     - Doc-comments scrubbed of the literal `"FlyingStar"` substring (rewrote as "the v1.5 Phi Tinh aggregator surface") to keep the CRIT-3 grep guard from self-tripping on the rationale text
   - 13/13 integration tests pass; full crate suite green

## Files Created/Modified

- `crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs` (modified, +~200 lines) — `add_iching_facts` + `add_direction_composite_facts` methods + dispatch wiring; doc-comments scrubbed of `"FlyingStar"` substring to satisfy CRIT-3 grep guard
- `crates/amlich-core/src/semantic_graph/ids.rs` (modified, +11 lines) — `SemanticId::iching_hexagram(role, king_wen, date, tz)` constructor
- `crates/amlich-core/src/iching/evaluator.rs` (modified, +21 lines) — `IChingCastSummary::chu_king_wen_index()` + `bien_king_wen_index()` accessors
- `crates/amlich-core/tests/semantic_graph_iching_integration.rs` (created, ~440 lines) — 13 black-box integration tests covering INT-11 IChing portion + INT-11 directional portion

## Decisions Made

- **Role-bearing stable key shape `"iching:{role}:{king_wen}:{date}:{tz}"`** with `role in {"chu", "bien"}` as the FIRST segment of the stable key. The role parameter is a string discriminator that prevents the primary + transformed hexagrams from colliding in `graph.node_count()` even when (in degenerate non-CRIT-4 cases) the cast produces chu_que == bien_que. The King Wen index is the SECOND segment for human readability — the role marker is the structural defense against collisions
- **Locked 4-envelope Hexagram provenance pattern (CRIT-6):** 2 envelopes per Hexagram node = 2 SOURCE_MAI_HOA_DICH_SO + 2 SOURCE_KINH_DICH (one of each per node). The composite envelope (e.g. `COMPOSITE_ICHING_CONSULTATION`) is NOT carried on the semantic-graph nodes — it lives on `IChingCastSummary.evidence` (Phase 24-01's locked 4-envelope structure). The semantic-graph layer is the "primitive envelope" projection; the composite envelope is the "evaluation result" projection. Mirrors the Phase 22-02 / 24-01 split between surface-level provenance + composite-result provenance
- **Edge insertion order discipline:** both Hexagram nodes are added BEFORE the Transforms + LocatedAt edges. `SemanticGraph::add_edge` silently drops edges whose endpoint nodes are missing (per `semantic_graph/graph.rs:23-28`). The integration test `iching_graph_edges_only_present_after_nodes_inserted` pins this at runtime: enriched snapshots have strictly more edges than ordinary ones, AND the enriched graph has exactly 1 Transforms + 2 LocatedAt edges. This is the structural defense against the silent-drop failure mode
- **Forward-compatibility placeholder NOT declared:** Plan 24-02's Task 2 was supposed to add a `DirectionCrossLinkSummary` placeholder type because Phase 23 was supposed to ship later. But Phase 23 fully shipped (commits `401b248`, `9ff695f`, `6f0d73d`, `bf680e0`, `c3e8a74`, `4b79803`, `26fd7a3`) BEFORE Plan 24-02 executed. The `DaySnapshot.direction_cross_link: Option<crate::reasoning::DirectionCrossLinkSummary>` field was already at `crates/amlich-core/src/lib.rs:201` from Phase 23-02's `bf680e0` commit. Task 2's "add a placeholder type" was a no-op — no declaration was made and no deletion was needed (the real type was already in place)
- **Phase 23 cross-link method uses `DirectionCrossLinkSummary.cross_link_source`** as the composite envelope's source_id (instead of hardcoding `"rule.composite.direction_cross_link"`). This gives Phase 23 control over the composite source_id (typically `"rule.composite.direction_cross_link"` per ADR-0007) without the semantic-graph layer having to know the constant. Forward-compatible: if Phase 23 ever ships a v2 composite with a different source_id, the semantic-graph layer follows automatically
- **`direction_composite_facts_wires_populated_state` test is ACTIVE** (not `#[ignore = "Phase 23 must ship first"]`'d): Phase 23 fully shipped `DirectionCrossLinkSummary` + `enrich_day_snapshot_with_direction_cross_link` helper before Plan 24-02 executed. The test exercises the end-to-end composite wiring (KHCBPPT + huyen-khong + `rule.composite.*` composite envelope + LocatedAt edge to day root) and verifies the cross-link node id is distinct from the daily travel direction node
- **`add_iching_facts` + `add_direction_composite_facts` are NOT merged into one method** even though they share the dispatch wiring: the two methods serve different enrichment paths (IChing via `enrich_day_snapshot_with_iching` vs Direction via `enrich_day_snapshot_with_direction_cross_link`) and have distinct activation triggers. Splitting them keeps the additive-builder pattern clean
- **`Huyền Không` Phi Tinh source-id is `SOURCE_HUYEN_KHONG`** (not a Phi Tinh-specific constant): the `add_flying_star_facts` method already uses this constant. The directional composite builder reuses the same constant — the primitive surface is identical (Phi Tinh palace overlay), only the node concept differs (Direction vs Element)
- **Tag values use the `key=value` shape** (`"king_wen=1"`, `"role=chu"`, `"verdict=cat"`) for downstream filterable consumption. Mirrors the `add_offering_facts` + `add_flying_star_facts` precedent
- **CRIT-3 isolation preserved by doc-comment scrubbing:** the grep guard test `iching_graph_no_flyingstar_in_iching_method` reads the literal `"FlyingStar"` substring inside `add_iching_facts` + `add_direction_composite_facts` method bodies (search range is `fn add_iching_facts` → `fn add_direction_composite_facts`). Initial GREEN-phase doc-comments used the literal `` `FlyingStar` `` when describing what the methods DON'T reference — the grep guard correctly flagged this as a self-tripping false-positive. Fixed by rephrasing to phrase-level ("the v1.5 Phi Tinh aggregator surface", "the v1.5 Phi Tinh aggregator surface") — mirrors the runtime-built needle pattern from v1.6/v1.7 (Phase 21-02 corpus.rs, Phase 22-02 golden.rs, Phase 24-01 evaluator.rs)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Doc-comments in `add_iching_facts` + `add_direction_composite_facts` contained the literal `"FlyingStar"` substring that the CRIT-3 grep guard flagged**

- **Found during:** Task 1 GREEN verification (`cargo test -p amlich-core --test semantic_graph_iching_integration`)
- **Issue:** Initial GREEN-phase doc-comments used the literal `` `FlyingStar` `` when describing what the methods DON'T reference (e.g. "this method does NOT import or reference `FlyingStar`"). The integration test's `iching_graph_no_flyingstar_in_iching_method` grep guard reads the literal substring inside the method body via `src[fn add_iching_facts..fn add_direction_composite_facts].contains("FlyingStar")` — the doc-comment's literal substring matched, causing the test to fail. Same trap as Phase 22-02 (golden.rs) + Phase 24-01 (evaluator.rs): bare-substring grep self-trips on rationale text
- **Fix:** Rewrote the doc-comments to use phrase-level names ("the v1.5 Phi Tinh aggregator surface", "the v1.5 Phi Tinh aggregator surface") instead of the literal type name. The intent (CRIT-3 isolation) is preserved in the doc-comment; the grep guard no longer self-trips
- **Files modified:** `crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs`
- **Verification:** `iching_graph_no_flyingstar_in_iching_method` now passes; all 13 integration tests pass; full crate suite green
- **Committed in:** `46ad421` (Task 1+2 GREEN commit)

### Architectural Deviations

**2. [Plan § Task 2 - "Forward-compatibility placeholder"] — Phase 23 shipped before Plan 24-02 executed, so the placeholder work was a no-op**

- **Context:** Plan 24-02's Task 2 explicitly anticipated that Phase 23 might not have shipped by execution time, and proposed declaring a minimal `DirectionCrossLinkSummary` placeholder type in `crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs` (with field shapes mirroring the real type) so the `DaySnapshot.direction_cross_link` field could compile before Plan 24-03
- **What actually happened:** Phase 23 fully shipped BEFORE Plan 24-02 executed. The `DaySnapshot.direction_cross_link: Option<crate::reasoning::DirectionCrossLinkSummary>` field was already at `crates/amlich-core/src/lib.rs:201` from Phase 23-02's `bf680e0` commit (shipped 2026-07-16). The `DirectionCrossLinkSummary` type lives at `crates/amlich-core/src/reasoning/direction_composite.rs:180` with the FULL production-quality field set (cross_link_kind + cross_link_source + date + day_chi_index + birth_chi_index + cells + summary_vi + composite_severity + evidence) — a much richer shape than the plan's placeholder proposal
- **What I did:** No placeholder was declared (Phase 23's real type is already in place). The `add_direction_composite_facts` method directly consumes `crate::reasoning::DirectionCrossLinkSummary`. The `DaySnapshot.direction_cross_link` field references the real type. No work was needed to delete the placeholder because no placeholder was ever created
- **Impact on plan:** Task 2's "add a placeholder type" + "Plan 24-03 replaces the placeholder with the Phase-23-shipped real type" sub-actions are moot. Plan 24-03's remaining work is the combined-strip v1.6→v1.7 round-trip integration test + the REQUIREMENTS.md INT-12 close — NOT placeholder cleanup
- **Committed in:** n/a (no commit needed; the work was already done by Phase 23's earlier commits)

**3. [Plan § Task 2 - "`#[ignore = 'Phase 23 must ship first']` for `direction_composite_facts_wires_populated_state`"] — Test is ACTIVE because Phase 23 has shipped**

- **Context:** Plan 24-02's Task 2 proposed gating the `direction_composite_facts_wires_populated_state` test with `#[ignore = "Phase 23 must ship first"]` until Phase 23 shipped `DirectionCrossLinkSummary` + Plan 24-03 finalised the field
- **What actually happened:** Phase 23 fully shipped the type + helper. The test runs actively and validates the end-to-end directional composite wiring. No `#[ignore]` attribute was added to the test
- **Impact on plan:** One fewer `#[ignore]`'d test in the integration suite. The test exercises the full Phase 23 → Plan 24-02 surface integration. If Phase 23's composite source_id ever drifts from `rule.composite.direction_cross_link`, the test will fail with a loud message naming the source_ids seen
- **Committed in:** `278f4a5` (RED commit — the test was always active because Phase 23 had already shipped)

**4. [Plan § Task 2 commit split] — Task 2 work was merged into Task 1 GREEN**

- **Context:** Plan 24-02 proposed splitting the work into 3 commits: RED + Task 1 GREEN + Task 2 GREEN. The Task 2 GREEN commit was supposed to add the directional composite builder + lib.rs field + 4 tests as a discrete commit
- **What actually happened:** The dispatch wiring in `DaySnapshotGraphBuilder::new` calls both `add_iching_facts(snapshot)` + `add_direction_composite_facts(snapshot)` together (lines 47-48). The directional composite builder (Task 2) and the IChing builder (Task 1) share the same dispatch location, and the lib.rs field (Phase 23) was already in place. Splitting them into 2 separate GREEN commits would have required either (a) two consecutive commits that each touch the same dispatch wiring lines or (b) temporary placeholder body in one commit replaced by real body in the next. Both options add complexity for no benefit — the GREEN commit `46ad421` documents both surfaces in a single coherent change
- **What I did:** Single GREEN commit `46ad421` covering both Task 1 + Task 2 surfaces. The commit message enumerates both `add_iching_facts` + `add_direction_composite_facts` separately so the audit trail is preserved
- **Impact on plan:** No functional impact — the lock contract is the same. The git log shows RED + GREEN instead of RED + GREEN + GREEN, which is a minor compression of the plan's intended history
- **Committed in:** `46ad421` (merged Task 1+2 GREEN)

---

**Total deviations:** 1 auto-fixed (Rule 1 — false-positive grep guard fix via doc-comment scrubbing) + 3 architectural deviations (Phase 23 already shipped placeholder work, test gating was already lifted, GREEN commit split merged into single commit for coherency).

**Impact on plan:** All deviations are necessary to honor the actual project state (Phase 23 shipped before Plan 24-02 executed) and the actual implementation discipline (doc-comment scrubbing for false-positive grep guards). No scope creep. No behavior change to the locked contracts (locked 2-Hexagram-node + dual-source + role-bearing-stable-key pattern; CRIT-3 isolation preserved; CRIT-6 source-id discipline; WASM-safety + determinism; additive DTO discipline on DaySnapshot; immutable clone-and-attach enrichment).

## Issues Encountered

None beyond the 4 deviations documented above. The implementation went smoothly:
- The Phase 22 cast/biến-quẻ surface (Plan 22-01) + the Phase 22 thể-dụng surface (Plan 22-02) + the Phase 21 corpus lookup (Plan 21-02) + the Phase 24-01 IChingEvaluator (this phase) all composed cleanly — no integration surprises
- The Phase 23 directional cross-link (Plan 23-03) composed cleanly with the new `add_direction_composite_facts` builder — the composite node is consumed as a pure DTO projection, no lower-level imports from `almanac::fengshui` or `reasoning::direction_merge` were needed
- CRIT-3 isolation was preserved by doc-comment scrubbing (the only deviation). The runtime grep guard pin the discipline at the test level

## Authentication Gates

None — no external services, no credentials, no CLI deployments. Pure Rust algorithm + DTO + immutable enrichment + integration tests against the already-shipped Phase 21 corpus + Phase 22 cast/biến-quẻ/thể-dụng types + Phase 23 directional cross-link DTO + Phase 24-01 IChing evaluator surface. No new dependencies, no environment variables, no dashboards.

## User Setup Required

None — no external service configuration required. This plan is pure Rust algorithm + DTO consumption + immutable enrichment + integration tests against already-shipped Phase 21-24 types. No new dependencies, no environment variables, no dashboards.

## Next Phase Readiness

- **INT-11 is fully closed.** `add_iching_facts()` ships unconditionally — 2 distinct Hexagram nodes + 1 Transforms edge + 2 LocatedAt edges + dual-source provenance (CRIT-6). `add_direction_composite_facts()` ships as the forward-compatible method signature + dispatch + early-return contract + 3-provenance composite wiring (Phase 23's locked pattern). All 13 integration tests pass
- **Phase 24 has 1 more plan remaining (24-03):** combined-strip v1.6→v1.7 round-trip integration test in `tests/day_snapshot_v14_compat.rs` + REQUIREMENTS.md INT-12 full close. The `direction_cross_link` field is already in place (Phase 23-02), so Plan 24-03 does not need to declare it
- **CRIT-3 isolation preserved across the new module.** `rg "FlyingStar" crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs` returns matches ONLY in `add_flying_star_facts` + inline tests (NOT in `add_iching_facts` or `add_direction_composite_facts`). The integration test's grep guard pins this discipline at runtime
- **CRIT-6 source-id discipline preserved.** Each Hexagram node carries 2 provenance entries (one `SOURCE_MAI_HOA_DICH_SO` + one `SOURCE_KINH_DICH`). The Direction composite node carries 3 entries (KHCBPPT + huyen-khong + composite envelope). All production call-sites use the registered consts — `tests/source_id_guard.rs` still passes
- **WASM-safety + determinism discipline preserved.** `rg "rand::|Utc::now|std::fs::"` returns zero matches across the modified files (filesystem-free, wall-clock-free, RNG-free)
- **Stable-key uniqueness preserved (CRIT-4 + Pitfall P-3).** The primary chu Hexagram node id (`hexagram:iching:chu:<kw>:<date>:<tz>`) and the bien Hexagram node id (`hexagram:iching:bien:<kw>:<date>:<tz>`) cannot collide. The biến node uses the biến's own King Wen index (NOT the primary's)
- **Edge insertion order discipline preserved (semantic_graph/graph.rs:23-28).** Both Hexagram nodes are added BEFORE the Transforms + LocatedAt edges. The integration test `iching_graph_edges_only_present_after_nodes_inserted` pins this: enriched snapshots have strictly more edges than ordinary ones, AND the enriched graph has exactly 1 Transforms + 2 LocatedAt edges
- **No new crate dependencies.** `cargo tree -p amlich-core --depth 1` shows the existing `chrono` + `serde` + `serde_json` + `unicode-normalization` set unchanged
- **Ready for Plan 24-03** (combined-strip v1.6→v1.7 round-trip integration test + REQUIREMENTS.md INT-12 full close)
- **Ready for Phase 25** (E2E Validation + Golden Cross-Source Verification) — INT-13's combined-strip cross-source gate is met by Phase 22-02's golden dataset + the new `add_iching_facts` builder + the combined-strip round-trip test landing in Plan 24-03
- **No blockers.** Phase 24 has 1 more plan (24-03 combined-strip round-trip + INT-12 close); Phase 25 E2E unblocks after that

---

*Phase: 24-iching-evaluator-semantic-graph-wiring-dto-integration*
*Completed: 2026-07-16*

## Self-Check: PASSED

- All 1 declared `key-files.created` exists on disk:
  - `crates/amlich-core/tests/semantic_graph_iching_integration.rs`
- All 3 declared `key-files.modified` exist on disk and have changes:
  - `crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs` (modified, +~200 lines)
  - `crates/amlich-core/src/semantic_graph/ids.rs` (modified, +11 lines)
  - `crates/amlich-core/src/iching/evaluator.rs` (modified, +21 lines)
- All 2 task commit hashes (`278f4a5` RED, `46ad421` GREEN) are present in `git log`
- Plan-level verification gates green:
  - `cargo test -p amlich-core --test semantic_graph_iching_integration` → 13/13 tests pass
  - `cargo test -p amlich-core` → 1114 passing tests across 49 test groups, 0 failures, 0 regressions vs Plan 24-01's 1101 baseline
  - `cargo test -p amlich-core --test source_id_guard` → 1/1 passes (no bare source-id literals introduced)
  - `cargo test -p amlich-core --test fengshui_crit3_isolation` → 1/1 passes (existing CRIT-3 isolation unaffected)
  - `cargo test -p amlich-core --test direction_cross_link_integration` → 22/22 passes (Phase 23 tests unaffected)
  - `cargo build -p amlich-core` → clean
  - `cargo tree -p amlich-core --depth 1` → no new dependencies (chrono + serde + serde_json + unicode-normalization)
- `rg "FlyingStar" crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs` returns matches ONLY in `add_flying_star_facts` (lines 499, 509, 519) + inline tests (lines 1186+). Zero matches in `add_iching_facts` (line 780) or `add_direction_composite_facts` (line 894) — CRIT-3 isolation preserved
- `rg "impl From" crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs` returns ZERO — CRIT-3 cross-newtype discipline preserved
- `rg "rand::|Utc::now|std::fs::" crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs` returns ZERO — WASM-safety + determinism preserved
- `rg "iching:" crates/amlich-core/src/semantic_graph/ids.rs` returns the stable-key format string in `iching_hexagram` constructor (line 254) — locked contract
- Two distinct Hexagram nodes emitted when `iching_cast = Some(...)` with role-bearing stable keys (`hexagram:iching:chu:<kw>:<date>:<tz>` + `hexagram:iching:bien:<kw>:<date>:<tz>`) — verified by tests #1, #5
- ONE `EdgeConcept::Transforms` edge (chu → bien) + TWO `EdgeConcept::LocatedAt` edges (each Hexagram → day_root) — verified by tests #2, #3, #9
- Dual-source provenance (SOURCE_MAI_HOA_DICH_SO + SOURCE_KINH_DICH) on each Hexagram node — verified by test #4
- Zero Hexagram nodes on ordinary `calculate_day_snapshot(...)` — verified by test #6
- `direction_cross_link` is absent from JSON when None — verified by test #12
- `direction_cross_link` byte-equal round-trips when populated — verified by test #13
- `direction_composite_facts_wires_populated_state` end-to-end composite wiring verified by test #11
- INT-11 marked Complete in REQUIREMENTS.md
