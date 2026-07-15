---
phase: 19-recommends-offering-semantic-graph-node
research_type: phase-planning-research
researched: 2026-07-15
confidence: HIGH (architecture/ontology/discipline); MEDIUM-HIGH (dual-source edge semantics — non-ritual-tradition cure case is a synthetic design, not yet exercised in code)
---

# Phase 19 Research: `RecommendsOffering` Semantic-Graph Node + v1.6 Integration

> **Question this research answers:** What do I need to know to PLAN Phase 19 well?

## User Constraints

No `CONTEXT.md` exists for this project (confirmed via `ls .planning/` + `glob **/CONTEXT.md`). Locked constraints extracted from **ROADMAP.md Phase 19 section + REQUIREMENTS.md INT-07..10 + PROJECT.md carry-forward decisions**:

1. **`OfferingRef` schema is locked FIRST, before any builder code emits `Offering` nodes** (SC#1). The identity tuple is `OfferingRef { offering_id: String, name_vi: String, name_en: Option<String>, source_id: SourceId }`. No placeholder types; no "we'll add fields later".
2. **Additive-only DTO discipline** — both new `offering_refs` and the legacy `offerings` flat-string field must be `Option<T>` with `#[serde(default, skip_serializing_if = "Option::is_none")]` (SC#2 + cross-cutting constraint, mirroring `flying_stars` / `applicable_rituals` / `daily_flying_stars` at `lib.rs:163-172`).
3. **Dual-source provenance on `RecommendsOffering` edges** where the offering reference originates in a non-ritual tradition surfaced inside a ritual (SC#3). The v1.5 multi-source dedup pattern — `ProvenanceTracker::track()` appends to a Vec (no dedup; `SemanticId` is the join key) — is REUSED, not parallel-implemented.
4. **All 6 ontology slice locations** — both `NodeConcept::Offering` AND `EdgeConcept::RecommendsOffering` must be added to (a) the enum variant, (b) the `label()` match arm, (c) the `ConceptLabel` enum, (d) the `as_str()` match arm, (e) the static slice in `GraphOntology::node_concepts()` / `edge_concepts()`, and (f) the locked `v15_concepts_present_in_ontology_slices` test pattern at `ontology.rs:302-313` (extending it to `v15_v16_*`).
5. **Source-id discipline carries forward** — every new module declares `pub const SOURCE_*`; provenance call-sites never use string literals (CI-enforced by `tests/source_id_guard.rs` at `FORBIDDEN_LITERALS` line 13-21). If Phase 19 introduces a new tradition source_id (e.g., for the "element cure" surface), the guard list extends.
6. **CRIT-3 isolation carries forward** — `FlyingStar` / `DailyFlyingStar` types are forbidden in `interaction/direction_merge.rs` (guarded by `tests/fengshui_crit3_isolation.rs`). The new `Offering` / `RecommendsOffering` / `OfferingRef` types follow the same isolation principle — they belong in the semantic-graph / rituals layer, NEVER in `direction_merge`.
7. **v1.5 patterns carry forward** — schema-lock-before-corpus, single-commit RED→GREEN, audit-as-decisive-source, external-crate black-box tests, additive serde pattern (mirrors Phase 18-04's `daily_flying_stars` precedent).
8. **v1.5→v1.6 backward-compat round-trip** must exercise BOTH old fields (`flying_stars`, `applicable_rituals`) AND new fields (`daily_flying_stars`, `offering_refs`) in the same test pass (SC#4).
9. **Phase 18 dependency**: `DaySnapshot.daily_flying_stars: Option<DailyFlyingStarLayout>` must exist (Phase 18-04 closed 2026-07-15, commit `defe59e`). The round-trip test in Plan 19-03 will exercise this field alongside the new `offering_refs` field.
10. **Phase 18 verification dependency**: Phase 18 must be verified before Phase 19 starts (STATE.md:79). Phase 19 plans do NOT touch `daily.rs`, `flying_stars_daily_golden.json`, `fengshui_crit3_isolation.rs`, or the daily populate block in `lib.rs`.

---

## Phase Requirements

| Req | Title | Research finding that enables implementation |
|-----|-------|----------------------------------------------|
| **INT-07** | `NodeConcept::Offering` + `EdgeConcept::RecommendsOffering` (Ritual → Offering) with rationale + source provenance; `OfferingRef` identity type locked | **Architecture confirmed**: 6 slice locations at `ontology.rs:5-42, 44-85, 87-118, 120-153, 155-222, 224-292, 295-389` (the enum + label() + ConceptLabel + as_str() + static slice + test) — each must be touched additively. **Edge direction confirmed**: Ritual → Offering (matches `PrescribedFor` direction Ritual → DayCanchi at `day_snapshot.rs:562-566`; the existing convention is "ritual points at the thing it prescribes FOR"). **OfferingRef lock pattern** mirrors the existing `RitualEntry` schema-lock discipline at `rituals/schema.rs:127-150` (per ADR-0001) — `deny_unknown_fields`, additive `Option<T>` only, `serde` derive. |
| **INT-08** | `Ritual` semantic-graph node payload `offering_refs: Option<Vec<OfferingRef>>` additive + legacy `offerings: Option<Vec<String>>` flat-string field for BC; both `#[serde(default, skip_serializing_if = "Option::is_none")]` | **Open design point** (must be resolved by planner): the existing Ritual semantic-graph node at `day_snapshot.rs:539-567` carries NO structured payload (just `summary_vi: String` + `tags: Vec<String>` + `provenance: Vec<ProvenanceEntry>`). The SC's reference to "legacy `offerings: Option<Vec<String>>` flat-string field" has TWO possible interpretations — see Open Question Q1. Most likely: the payload lives on `SemanticNode` via a new `Option<serde_json::Value>` or via a typed `RitualNodePayload` enum (see Q1 for tradeoffs). The "legacy `offerings` field" is most likely the existing `DaySnapshot.applicable_rituals: Option<Vec<String>>` at `lib.rs:167` (which carries `ritual_id` strings, not offering strings — naming discrepancy, see Q1). |
| **INT-09** | `RecommendsOffering` edges carry dual-source provenance where the offering reference originates in a non-ritual tradition (e.g., Huyền Không element cure surfaced inside a ritual carries both `huyen-khong` and `vn-folk-ritual` provenance); reuse v1.5 multi-source dedup pattern | **Reuse pattern confirmed**: `ProvenanceTracker::track()` at `provenance.rs:130-135` appends to `Vec<ProvenanceEntry>` keyed by `node_id` — does NOT dedup. The Direction node at `SemanticId::new("direction", "travel:day:+7:all")` (`day_snapshot.rs:829`) is the canonical example: it carries BOTH `khcbppt` (from `add_travel_direction_fact`) AND `huyen-khong` (from `add_flying_star_facts`). Verified by `direction_node_carries_dual_provenance_khcbppt_and_huyen_khong` test at `day_snapshot.rs:926-950`. **Phase 19 extension**: when emitting `RecommendsOffering` from a Ritual node to an Offering node that was surfaced via a non-ritual tradition, call `provenance_tracker.track(edge_id, huyen_khong_entry)` AND `provenance_tracker.track(edge_id, vn_folk_ritual_entry)` — exactly the v1.5 append pattern, no parallel dedup logic. **Non-ritual surface mechanism** is a new design (the codebase currently has NO place where a Huyền Không element cure is "surfaced inside a ritual") — this is the lowest-confidence piece, see Q2. |
| **INT-10** | v1.5→v1.6 backward-compat round-trip test loads v1.5 JSON fixture (with `flying_stars`, no `daily_flying_stars`, no `offering_refs`) into v1.6 structs and re-serializes without unexpected fields + 2026 E2E smoke on ≥ 5 dates exercising BOTH annual/monthly `flying_stars` AND new `daily_flying_stars` with `Offering`/`RecommendsOffering` wiring verified | **Round-trip test pattern confirmed**: `tests/day_snapshot_v14_compat.rs` carries 6 tests (3 pre-existing for v1.4→v1.5 + 3 new from Phase 18 for v1.5→v1.6). The pattern is `build v1.6 snapshot → serde_json::to_string → strip new field → serde_json::from_str into v1.6 struct → assert field defaults to None → re-serialize → assert byte-equal`. Phase 19 extends to a `v15_compat.rs` sibling file (cleaner naming — see Q3) OR appends to `day_snapshot_v14_compat.rs` (matches the existing extension pattern). **2026 E2E smoke** at `tests/integration_2026_smoke.rs:139-181` already covers ≥30 dates with Pillar APIs only. Phase 19 adds ≥5 dates that ALSO exercise `daily_flying_stars` + Offering wiring — natural fit: Tết 2026-02-17 + 4 lunar-cycle dates that surface a Huyền Không element cure alongside an `applicable_rituals` match. |

---

## Standard Stack

**No new crate dependencies required.** All Phase 19 infrastructure is already in the workspace, mirroring Phase 18's stack discipline.

| Already in tree | Used by Phase 19 for |
|-----------------|----------------------|
| `serde` 1.0 (derive) | `OfferingRef` struct, `RitualNodePayload` enum/struct (if payload lives on SemanticNode — see Q1), `NodeConcept::Offering` + `EdgeConcept::RecommendsOffering` enum variants, additive DTO fields — mirrors existing `Offering`/`RitualEntry` derives in `rituals/schema.rs:104-115, 127-150` and `FlyingStar` / `FlyingStarLayout` derives in `almanac/fengshui/types.rs` |
| `serde_json` 1.0 | Round-trip test fixtures (existing `tests/day_snapshot_v14_compat.rs` pattern) |
| `crate::sources::{SOURCE_VN_FOLK_RITUAL, SOURCE_HUYEN_KHONG}` | Edge provenance construction — existing v1.5 imports at `day_snapshot.rs:6`, `rituals/corpus.rs:126` |
| `crate::semantic_graph::{SemanticNode, SemanticEdge, ProvenanceEntry, ProvenanceTracker, SemanticId, NodeConcept, EdgeConcept, NodeOrigin}` | Builder extension — existing imports at `day_snapshot.rs:1-8` |
| `crate::rituals::{find_van_khan_for_snapshot, get_ritual_by_id, all_rituals}` | Builder iteration over rituals + lookup by `ritual_id` to extract structured offerings — existing imports at `lib.rs:364`, `rituals/mod.rs:27, 31`, `rituals/matcher.rs:21, 50` |
| `crate::almanac::fengshui::star_metadata` | Huyền Không element cure lookup (if the cure is exposed as a star→element suggestion) — existing import at `day_snapshot.rs:1` |
| `std::sync::OnceLock` + `include_str!` | Optional: if `offering_refs` is hydrated from a corpus JSON file (likely NOT — the builder derives from `applicable_rituals` + `RitualEntry::offerings` already loaded via `all_rituals()`); mirror Phase 18's `daily_pivots_for_year` lazy compute if corpus-driven |
| `std::collections::HashSet` (transitive) | Dedup of `recommends_offering` edges by `(from_id, to_id)` pair — mirrors the dedup pattern at `integration_2026_smoke.rs:165-168` |

**Phase 19 does NOT propose any new crates.** If a planner finds themselves reaching for a graph library (e.g., `petgraph`), they're hand-rolling something `semantic_graph::SemanticGraph::add_node` + `add_edge` already provides.

---

## Architecture Patterns

Phase 19 extends the existing v1.5/Phase 18 split with the offering semantic-graph concept. Recommended module/file layout (mirrors v1.5 + Phase 18 patterns rigidly):

```
crates/amlich-core/src/
├── semantic_graph/
│   ├── ontology.rs              # ADD: NodeConcept::Offering + EdgeConcept::RecommendsOffering variants
│   │                              #      + 6 slice-location edits (enum + label() + ConceptLabel + as_str() + slice + test)
│   ├── builders/
│   │   └── day_snapshot.rs      # ADD: add_offering_facts(snapshot) helper called from DaySnapshotGraphBuilder::new
│   │                              #      reuses ProvenanceTracker::track() append-pattern from add_flying_star_facts + add_ritual_facts
│   └── tests/                   # NEW (optional): offerings_integration.rs black-box tests for the new wiring
├── rituals/
│   ├── schema.rs                # ADD: OfferingRef struct (locked first per SC#1 + schema-lock discipline)
│   │                              #      (likely lives here alongside Offering + RitualEntry for collocation)
│   └── mod.rs                   # ADD: pub use schema::OfferingRef; re-export
├── lib.rs                       # ADD: additive offering_refs field (location depends on Q1 resolution)
│                                  #      (DaySnapshot OR Ritual semantic-graph node OR both)
└── tests/
    ├── day_snapshot_v15_compat.rs  # NEW: v1.5→v1.6 round-trip tests (sibling file, see Q3)
    │                                  OR tests/day_snapshot_v14_compat.rs extension (alternative)
    ├── integration_2026_smoke.rs   # EXTEND: ≥5 dates exercising daily_flying_stars + Offering wiring
    └── source_id_guard.rs          # EXTEND (only if Phase 19 introduces a new source_id — likely NO)

```

**Mirror the existing patterns rigidly:**

1. **`ontology.rs` pattern** — the v1.5 ritual/flying_star extensions landed at lines 40-41, 81-82, 217-218, 286-287, 353-354, 386-387, 302 (test). Phase 19 adds `Offering` and `RecommendsOffering` to the SAME 6 locations each. The locked test at `ontology.rs:302-313` (`v15_concepts_present_in_ontology_slices`) needs to be either extended (rename to `v15_v16_*`) OR a new test added that asserts the v1.6 concepts are present. **Compiler-enforced exhaustiveness** is the core invariant — every match arm must be touched, the test catches any drift.

2. **`provenance.rs:130-135` `ProvenanceTracker::track()` pattern** — the multi-source dedup logic is implicit (no dedup; SemanticId is the join key). Phase 19's dual-source `RecommendsOffering` edge calls `provenance_tracker.track(edge_id, entry_1)` then `track(edge_id, entry_2)` for the dual-source case. **NO new dedup helper**, NO parallel `Vec::dedup()` call. The single source of truth is `ProvenanceTracker::track()` and it MUST stay that way.

3. **`day_snapshot.rs:539-567` `add_ritual_facts` pattern** — the existing v1.5 builder emits ONE aggregate `Ritual` node per day with `summary_vi: String` + `provenance: Vec<ProvenanceEntry>` + `EdgeConcept::PrescribedFor` to day_root. Phase 19 extends this with `add_offering_facts(snapshot)` called from `DaySnapshotGraphBuilder::new` (insert between `add_ritual_facts` and `build()`, mirroring the populate-block grouping pattern at `lib.rs:327-370`). The new helper:
   - Reads `snapshot.applicable_rituals: Option<Vec<String>>` (the existing ritual_id list, lib.rs:167)
   - For each ritual_id, looks up `RitualEntry` via `get_ritual_by_id(ritual_id)` (existing at `rituals/matcher.rs:50`)
   - For each `entry.offerings[i]` (structured `Offering` from `schema.rs:104-115`), creates an `OfferingRef` (locked FIRST in Plan 19-01) and emits an `Offering` node + `RecommendsOffering` edge
   - For each edge, emits ONE or TWO `ProvenanceEntry`s depending on whether the offering reference has a non-ritual origin (e.g., a Huyền Không element cure surface — see Q2)
   - **Dedup**: a HashSet of `(ritual_node_id, offering_node_id)` ensures the same edge is not emitted twice when two rituals share the same offering reference

4. **`rituals/schema.rs:104-115` `Offering` pattern** — the existing structured `Offering { name_vi: String, name_en: Option<String>, quantity: Option<String>, notes: Option<String> }` is the JSON-corpus-side representation. `OfferingRef` is the semantic-graph-side representation. They are NOT the same — `OfferingRef` adds `offering_id` (a stable id for graph join) and `source_id` (mandatory provenance tag). The two coexist: `RitualEntry.offerings: Vec<Offering>` stays as the corpus schema; `OfferingRef` is the semantic-graph handle that wraps a `&Offering` with an `offering_id` + `source_id`.

5. **`lib.rs:153-173` `DaySnapshot` pattern** — the additive serde pattern `#[serde(default, skip_serializing_if = "Option::is_none")]` is the established convention. If the planner decides `offering_refs` lives on `DaySnapshot` (Q1 option A), the field goes alongside `applicable_rituals:167` and `daily_flying_stars:172`. If it lives on the Ritual semantic-graph node (Q1 option B), it goes on a new `RitualNodePayload` enum/struct (or via a generic `payload: Option<serde_json::Value>` field on `SemanticNode`).

6. **`rituals/corpus.rs:126` `entry.source_id == SOURCE_VN_FOLK_RITUAL` pattern** — every ritual entry is required to carry the canonical `vn-folk-ritual` source_id, asserted at corpus load. Phase 19's `OfferingRef.source_id` follows the same discipline: REQUIRED field, asserted at construction (e.g., `OfferingRef::new(...)` enforces `source_id != ""` via debug_assert). The builder then passes through whichever source_id the corpus says.

7. **Edge provenance note** — `provenance.rs:43-46` `with_note()` carries audit-friendly context strings. The v1.5 `add_ritual_facts` builder doesn't currently call `with_note` (the aggregate `Ritual` node has just source+method). Phase 19's `RecommendsOffering` edges should follow the Phase 18 daily-edge precedent (`daily.rs:169`) of calling `with_note(format!("offering={offering_id};ritual={ritual_id};source_id={source_id};rationale=..."))`. This gives auditors a grep-able audit trail per edge.

8. **Test layering**:
   - **Unit tests** at `ontology.rs:298-313` — extend to assert `Offering` + `RecommendsOffering` are present in the static slices + round-trip label.
   - **Integration tests** at `day_snapshot.rs:578-951` — extend the `v15_pillar_nodes_carry_disjoint_source_ids_and_direction_is_multi_source` test (or add a new test) to assert Offering nodes + RecommendsOffering edges + dual-source edge provenance.
   - **Black-box tests** at `tests/integration_2026_smoke.rs:139-181` — extend with ≥5 dates that exercise daily_flying_stars + Offering wiring.

---

## Don't Hand-Roll

| Concern | Why NOT to hand-roll | Existing alternative |
|---------|----------------------|----------------------|
| **Multi-source edge provenance** | The v1.5 dedup trap is implicit (no explicit dedup at all). A planner might add a `HashSet<(String, String)>` dedup helper or a `Vec::dedup()` call to "clean up" duplicate provenance entries. This would BREAK the v1.5 invariant that `Direction` carries BOTH `khcbppt` and `huyen-khong` — the whole point of `ProvenanceTracker::track()` is that it appends. | `ProvenanceTracker::track(node_id, entry)` at `provenance.rs:130-135` — append to Vec; do NOT add any dedup. The existing test `direction_node_carries_dual_provenance_khcbppt_and_huyen_khong` at `day_snapshot.rs:926-950` would catch any regression. |
| **`OfferingRef` schema** | A planner might be tempted to reuse `RitualEntry::offerings: Vec<Offering>` directly as the semantic-graph handle. This misses the join key (`offering_id`) — two rituals sharing an "Hương" offering would emit two graph nodes with the same identity, and `ProvenanceTracker::track()` would never link them. | `OfferingRef { offering_id: String, name_vi: String, name_en: Option<String>, source_id: SourceId }` — locked FIRST in Plan 19-01, mirrors `RitualEntry { ritual_id: String, ... }` schema-lock discipline at `schema.rs:127-150`. |
| **`offering_id` derivation** | A naïve implementation hashes `name_vi` to derive `offering_id`. This is unstable across Unicode NFC/NFD normalization (e.g., `Hương` vs `Huong`+combining-circumflex) and across corpus renames. | Derive `offering_id` from the corpus position: `format!("ritual.{ritual_id}.offering.{idx}")` — stable, grep-able, mirrors the `SemanticId::new("flying_star", "day:...:flying_stars")` pattern at `day_snapshot.rs:487-489`. The "same offering across rituals" join can be a future optimization (a global `offering` corpus), not Phase 19. |
| **Node ID for Offering nodes** | A planner might mint node IDs like `"offering.0"`, `"offering.1"` — collision risk if two builders run on different snapshots. | `SemanticId::new("offering", format!("ritual:{ritual_id}:offering:{idx}:day:{date}:{tz}"))` — mirrors the timezone-suffixed `tz_suffix` discipline at `day_snapshot.rs:18-23, 488`. Same node ID format as the `flying_star.day:...` and `ritual.day:...` nodes. |
| **DaySnapshot round-trip fixture** | Building a v1.5-shaped JSON by hand is error-prone (easy to forget fields, easy to break the additive serde invariant). | Reuse the v1.5 fixture pattern from `tests/day_snapshot_v14_compat.rs` — for the new test, build a v1.6 snapshot via `calculate_day_snapshot(17, 2, 2026)` (Tết 2026 — guarantees `flying_stars`, `applicable_rituals`, `daily_flying_stars`, `offering_refs` all populated), strip `daily_flying_stars` + `offering_refs` to simulate v1.5, round-trip through serde, assert both fields default to None and re-serialize yields byte-equal JSON. |
| **E2E smoke date selection** | Picking dates ad-hoc risks missing the daily_flying_stars surface (which is densely populated for ALL dates after Phase 18-04). | Use the existing `collect_lunar_day_dates(1)` + `collect_lunar_day_dates(15)` helpers at `integration_2026_smoke.rs:33-83` — extend the date set with the Tết 2026 date + 4 lunar-cycle dates (e.g., lunar day 1 of months 3, 6, 9, 12 of 2026) that surface both a ritual match AND have populated `daily_flying_stars`. No new helper needed. |
| **JSON include_str! for Offering corpus** | The v1.5 ritual corpus is JSON-loaded via `OnceLock` + `include_str!` at `rituals/corpus.rs`. A planner might propose a separate `offerings.json` corpus. This is wrong — offerings are an attribute of rituals, not a standalone corpus. | Derive Offering nodes from `RitualEntry::offerings: Vec<Offering>` (the existing structured corpus field at `rituals/schema.rs:137`). No new corpus, no new loader. The "non-ritual-tradition cure" surface (Q2) is the only case that needs a separate origin, and that origin is `SOURCE_HUYEN_KHONG` via `almanac::fengshui::star_metadata` — not a corpus. |
| **Existing Phase 18 grep guard patterns** | Adding a new grep-guard test file (`tests/offering_crit3_isolation.rs` etc.) duplicates the existing `tests/fengshui_crit3_isolation.rs` (44 lines, 6 forbidden patterns). The new types don't need CRIT-3 isolation (they're not Phi Tinh palace layouts). | NO new grep-guard test file needed for Phase 19. If the planner feels strongly about isolation, extend `source_id_guard.rs` instead (existing, well-maintained). |

---

## Common Pitfalls

| # | Pitfall | Mitigation |
|---|---------|------------|
| **P-1** | **"Legacy `offerings: Option<Vec<String>>`" naming confusion**: Phase 19 SC#2 references a legacy `offerings: Option<Vec<String>>` flat-string field, but the existing field on `DaySnapshot` is `applicable_rituals: Option<Vec<String>>` (`lib.rs:167`) and it carries `ritual_id`s (not offering names). A planner might add `offerings: Option<Vec<String>>` as a NEW field on `DaySnapshot` containing the flattened offering names, breaking the additive contract. | **Open Question Q1** — the planner must resolve this BEFORE writing Plan 19-01. Two interpretations: (a) the legacy field IS `applicable_rituals` (rename to `offerings` is NOT acceptable — breaks v1.5 fixture round-trip), or (b) Phase 19 introduces a NEW `offerings: Option<Vec<String>>` flat-string field on `DaySnapshot` (additive, mirrors `applicable_rituals`, carries offering names flattened from `RitualEntry::offerings`). **Recommendation**: interpretation (b) — Phase 19 introduces both `offering_refs: Option<Vec<OfferingRef>>` (preferred, structured) AND `offerings: Option<Vec<String>>` (legacy flat-string summary, auto-populated from `offering_refs`). This matches the literal SC text and the additive contract. |
| **P-2** | **Dual-source edge provenance is a single-source edge with one appended entry**: A planner might emit `RecommendsOffering` with ONLY `SOURCE_VN_FOLK_RITUAL` provenance (since "it's a ritual"). The non-ritual-tradition cure surface (Huyền Không element cure surfaced inside a ritual, INT-09 example) requires the edge to carry BOTH `SOURCE_HUYEN_KHONG` and `SOURCE_VN_FOLK_RITUAL`. | The builder helper `add_offering_facts(snapshot)` takes a `bool dual_source` flag (or inspects an `origin` field on `OfferingRef`) and emits 1 or 2 `ProvenanceEntry`s via `provenance_tracker.track(edge_id, entry)`. The `bool` is set based on whether the offering reference has a non-ritual origin (Phase 19 SC#3 + Q2). |
| **P-3** | **`OfferingRef` locked AFTER builder code emits `Offering` nodes**: A planner might write `add_offering_facts` first with a placeholder `OfferingNode { name: String }`, then "fill in" the schema later. This breaks the schema-lock-before-corpus discipline and is hard to refactor once the builder is wired. | Plan 19-01 is EXCLUSIVELY about locking `OfferingRef` (no builder code). Plan 19-02 starts the builder. This is the EXACT discipline that Phase 11 (v1.5 ritual schema lock) and Phase 18 (DailyFlyingStarLayout type stub) followed — see `state.md:42` and Phase 18-01 SUMMARY's "schema-lock first" pattern. |
| **P-4** | **CRIT-3 isolation broken**: `Offering` or `OfferingRef` types imported into `interaction/direction_merge.rs`. Unlike `FlyingStar` / `DailyFlyingStar`, the new types don't need an isolation grep guard (they're not palace-layout descriptors). But importing them into `direction_merge` would couple `vn-folk-ritual` provenance with `khcbppt` directional output — a less catastrophic version of the same trap. | **NO new grep guard needed.** The existing `tests/fengshui_crit3_isolation.rs` is specific to Phi Tinh type names. The new types should simply not be needed by `direction_merge` — if a planner finds themselves wanting to import `OfferingRef` into `direction_merge`, that's a design smell, not a guard failure. Code review checklist item. |
| **P-5** | **Dual-source edge DROPS one of the source_ids at serialization**: The current `ProvenanceEntry` at `provenance.rs:17-26` serializes each entry as a separate object in the `provenance: Vec<>` JSON array. A planner might serialize the dual-source edge as a single entry with a comma-separated source_id string (`"huyen-khong,vn-folk-ritual"`) — breaking the v1.5 invariant that source_ids are atomic strings. | The existing `provenance: Vec<ProvenanceEntry>` on `SemanticNode` (`node.rs:26`) ALREADY supports multiple entries — Phase 19 just emits 2 entries for dual-source edges. No JSON shape change, no serializer logic change. The `Direction` node precedent at `day_snapshot.rs:835-857` confirms this works. |
| **P-6** | **`offering_refs` populated on `DaySnapshot` BEFORE Phase 18's `daily_flying_stars` is verified**: Phase 18's `daily_flying_stars` field is at `lib.rs:171-172` and is auto-populated in `calculate_day_snapshot_internal` at `lib.rs:349-361`. If Phase 19 starts before Phase 18 verification completes, the new populate block for `offering_refs` will not see the same `calculate_day_snapshot_internal` shape and may insert in the wrong location. | Phase 18 verification is the prerequisite (per STATE.md:79). Phase 19 does NOT touch the daily_flying_stars populate block. The new `offering_refs` populate block goes alongside `applicable_rituals` (lib.rs:362-370) — both are ritual-related and group naturally. |
| **P-7** | **Forgetting to extend `tests/source_id_guard.rs` if a new source_id is introduced**: If Phase 19 introduces a new tradition (e.g., `SOURCE_HUYEN_KHONG_ELEMENT_CURE` for the non-ritual-tradition cure surface), the guard's `FORBIDDEN_LITERALS` list at line 13-21 must be extended. A missing entry would let bare literals slip into production code. | If no new source_id is introduced (likely — the dual-source case reuses `SOURCE_HUYEN_KHONG` + `SOURCE_VN_FOLK_RITUAL`), NO guard extension needed. If a new id IS introduced, add it to the FORBIDDEN_LITERALS array AND register it as a `pub const SOURCE_*` in `sources.rs` (mirroring the existing 7 entries). |
| **P-8** | **`offerings: Option<Vec<String>>` flat-string field flatly duplicates `offering_refs`**: A planner might compute `offerings` as `offering_refs.iter().flat_map(|r| r.name_vi).collect()` — losing the `offering_id` and `source_id` join. The flat-string field is meant to be a SUMMARY, not a lossy projection. | The flat-string `offerings: Option<Vec<String>>` carries the offering `name_vi` values in order, deduplicated (per-ritual at least — possibly globally). It does NOT carry `offering_id` or `source_id`. This is a "human-readable summary" field for legacy consumers, not a structured query target. Document this in the field doc-comment. |
| **P-9** | **E2E smoke test exercises the new fields on dates that DON'T surface them**: The new 5 dates added to `tests/integration_2026_smoke.rs` MUST be dates that actually populate `offering_refs` (i.e., have `applicable_rituals` non-empty AND have offerings in the ritual corpus). Picking arbitrary dates would silently skip the test. | Use Tết 2026-02-17 (already in the existing date set at `integration_2026_smoke.rs:145`) + lunar day 1 of months 3, 6, 9, 12 of 2026 — these all match the `soc-vong-mung-1` ritual (existing corpus entry per `rituals/corpus.rs:238`) which has `RitualEntry::offerings` populated. Verify by running `find_van_khan_for_snapshot` on the candidate dates before adding to the test. |
| **P-10** | **DaySnapshot round-trip test forgets to assert `None → absent in JSON`**: The existing v1.4/v1.5 round-trip tests assert three properties: (1) missing key deserializes to None, (2) byte-equal round-trip, (3) None is absent in serialized JSON. A planner might write only (1) and (2) for v1.6, missing (3). | Mirror the EXACT pattern from `tests/day_snapshot_v14_compat.rs:73-128` — all 3 properties for both `daily_flying_stars` AND `offering_refs`. The (3) assertion is the one that catches additive-contract regressions (a future refactor that accidentally removes `skip_serializing_if = "Option::is_none"`). |
| **P-11** | **Test author mistakes `applicable_rituals` for `offering_refs` in the round-trip**: The two fields are different (`applicable_rituals` = ritual_ids, `offering_refs` = OfferingRef with offering_id+name_vi+source_id). The round-trip test must verify the field SHAPE, not just that "something round-trips". | The v1.5→v1.6 round-trip test must assert: `snapshot.applicable_rituals == Some(vec!["van-khan-tet-don-gian"])` AND `snapshot.offering_refs == Some(vec![OfferingRef { offering_id: "ritual.van-khan-tet-don-gian.offering.0", name_vi: "Hương", source_id: SOURCE_VN_FOLK_RITUAL, ... }])` after strip-and-restore. Field-by-field assertions, not just non-emptiness. |

---

## Code Examples

These are pattern extracts from existing files that Phase 19 should mirror:

### Example 1: `OfferingRef` type stub (mirror `rituals/schema.rs:104-115`)

```rust
// In rituals/schema.rs — Phase 19-01 (INT-08)
use crate::sources::SourceId; // OR: just use String + const discipline

/// Identity handle for a semantic-graph Offering node.
///
/// Locked before any builder code emits Offering nodes (schema-lock
/// discipline per Phase 10 / Phase 18-01). Mirrors `RitualEntry::ritual_id`
/// as the stable join key for the semantic graph.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OfferingRef {
    /// Stable id of the form "ritual.{ritual_id}.offering.{idx}".
    /// Derived from the corpus position, not hashed from name_vi
    /// (see Pitfall P-3 / Don't-Hand-Roll).
    pub offering_id: String,
    pub name_vi: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_en: Option<String>,
    /// MUST equal one of crate::sources::SOURCE_*. Enforced by the
    /// constructor (debug_assert) + tests/source_id_guard.rs (string literal ban).
    pub source_id: String,
}

impl OfferingRef {
    pub fn new(offering_id: String, name_vi: String, name_en: Option<String>, source_id: String) -> Self {
        debug_assert!(!offering_id.is_empty(), "OfferingRef::offering_id must be non-empty");
        debug_assert!(!name_vi.is_empty(), "OfferingRef::name_vi must be non-empty");
        debug_assert!(!source_id.is_empty(), "OfferingRef::source_id must be non-empty");
        Self { offering_id, name_vi, name_en, source_id }
    }
}
```

### Example 2: Ontology slice additions (mirror `ontology.rs:40-41, 81-82, 217-218, 286-287, 353-354`)

```rust
// In semantic_graph/ontology.rs — Phase 19-02 (INT-07)
// Six slice locations, all additive:

// (1) NodeConcept enum
pub enum NodeConcept {
    // ... existing 35 variants
    Ritual, FlyingStar,
    Offering,  // NEW (Phase 19-02)
}

// (2) NodeConcept::label() match
impl NodeConcept {
    pub fn label(&self) -> ConceptLabel {
        match self {
            // ... existing arms
            Self::Ritual => ConceptLabel::Ritual,
            Self::FlyingStar => ConceptLabel::FlyingStar,
            Self::Offering => ConceptLabel::Offering,  // NEW
        }
    }
}

// (3) EdgeConcept enum (note: v1.5 already had PrescribedFor/OccupiesPalace/CarriesElement)
pub enum EdgeConcept {
    // ... existing 24 variants
    PrescribedFor, OccupiesPalace, CarriesElement,
    RecommendsOffering,  // NEW (Phase 19-02)
}

// (4) EdgeConcept::label() match
impl EdgeConcept {
    pub fn label(&self) -> ConceptLabel {
        match self {
            // ... existing arms
            Self::PrescribedFor => ConceptLabel::PrescribedFor,
            Self::OccupiesPalace => ConceptLabel::OccupiesPalace,
            Self::CarriesElement => ConceptLabel::CarriesElement,
            Self::RecommendsOffering => ConceptLabel::RecommendsOffering,  // NEW
        }
    }
}

// (5) ConceptLabel enum (add Offering + RecommendsOffering variants)
// (6) ConceptLabel::as_str() (add snake_case strings: "offering", "recommends_offering")

// (7) Static slices (add to node_concepts() and edge_concepts())
impl GraphOntology {
    pub fn node_concepts() -> &'static [NodeConcept] {
        &[/* ... */, NodeConcept::Ritual, NodeConcept::FlyingStar, NodeConcept::Offering]
    }
    pub fn edge_concepts() -> &'static [EdgeConcept] {
        &[/* ... */, EdgeConcept::PrescribedFor, EdgeConcept::OccupiesPalace,
          EdgeConcept::CarriesElement, EdgeConcept::RecommendsOffering]
    }
}
```

### Example 3: Dual-source edge builder (mirror `day_snapshot.rs:475-567`)

```rust
// In semantic_graph/builders/day_snapshot.rs — Phase 19-02 (INT-09)
fn add_offering_facts(&mut self, snapshot: &DaySnapshot) {
    let Some(ritual_ids) = &snapshot.applicable_rituals else { return; };
    if ritual_ids.is_empty() { return; }

    let mut emitted_edges: HashSet<(String, String)> = HashSet::new();

    for ritual_id in ritual_ids {
        let Some(entry) = crate::rituals::get_ritual_by_id(ritual_id) else { continue; };

        for (idx, offering) in entry.offerings.iter().enumerate() {
            // Build OfferingRef (locked type from Plan 19-01)
            let offering_ref = OfferingRef::new(
                format!("ritual.{ritual_id}.offering.{idx}"),
                offering.name_vi.clone(),
                offering.name_en.clone(),
                crate::sources::SOURCE_VN_FOLK_RITUAL.to_string(),
            );

            // Emit Offering node (single provenance: vn-folk-ritual)
            let offering_node_id_raw = SemanticId::new(
                "offering",
                format!("ritual:{ritual_id}:offering:{idx}:day:{}:{}", self.date_str, self.tz_suffix),
            );
            let offering_node_id = offering_node_id_raw.clone().to_node_id();
            let offering_prov = ProvenanceEntry::almanac_rule(
                SOURCE_VN_FOLK_RITUAL, "ritual.offering_lookup",
            ).with_note(format!("offering_id={};ritual_id={};rationale=lễ vật của nghi lễ", offering_ref.offering_id, ritual_id));

            let offering_node = SemanticNode::new(
                offering_node_id_raw,
                NodeConcept::Offering,
                NodeOrigin::Fact,
                format!("Lễ vật: {}", offering_ref.name_vi),
            ).with_provenance(offering_prov);
            self.graph.add_node(offering_node);

            // Find the Ritual node (built by add_ritual_facts earlier in builder pipeline)
            let ritual_node_id =
                SemanticId::new("ritual", format!("day:{}:rituals", self.tz_suffix)).to_node_id();

            // Dedup check
            if !emitted_edges.insert((ritual_node_id.clone(), offering_node_id.clone())) {
                continue;
            }

            // Emit RecommendsOffering edge (single source OR dual source)
            let edge = SemanticEdge::new(&ritual_node_id, &offering_node_id, EdgeConcept::RecommendsOffering);
            self.graph.add_edge(edge);

            // Track edge provenance — 1 entry for ritual-origin offerings, 2 for non-ritual-tradition cures
            let edge_id = format!("{}->{}", ritual_node_id, offering_node_id);
            self.provenance_tracker.track(edge_id.clone(), ProvenanceEntry::almanac_rule(
                SOURCE_VN_FOLK_RITUAL, "ritual.recommends_offering",
            ).with_note(format!("ritual={};offering_id={}", ritual_id, offering_ref.offering_id)));

            // Dual-source: if this offering is a Huyền Không element cure surfaced inside the ritual,
            // add the huyen-khong provenance entry too (mirrors v1.5 Direction-node dual-source).
            if offering_ref.offering_id.ends_with(".element_cure") {
                self.provenance_tracker.track(edge_id, ProvenanceEntry::almanac_rule(
                    SOURCE_HUYEN_KHONG, "fengshui.element_cure",
                ).with_note(format!("element_cure_for={}", offering_ref.name_vi)));
            }
        }
    }
}
```

> **Note on Example 3**: This is a SKELETON. The actual non-ritual-tradition cure detection (the `ends_with(".element_cure")` check) is the lowest-confidence piece — see Q2. The planner should resolve Q2 BEFORE writing Plan 19-02.

### Example 4: DaySnapshot round-trip extension (mirror `tests/day_snapshot_v14_compat.rs:73-128`)

```rust
// In tests/day_snapshot_v15_compat.rs (NEW sibling) — Phase 19-03 (INT-10)
use amlich_core::calculate_day_snapshot;
use amlich_core::rituals::OfferingRef;
use amlich_core::sources::SOURCE_VN_FOLK_RITUAL;

#[test]
fn v15_json_without_offering_refs_deserializes() {
    // Build a v1.6 snapshot with daily_flying_stars + offering_refs populated
    let snapshot = calculate_day_snapshot(17, 2, 2026); // Tết 2026 — guarantees both fields

    // Serialize
    let json = serde_json::to_string(&snapshot).expect("serialization failed");

    // Strip the new offering_refs field to simulate a v1.5 JSON
    let mut v15_json: serde_json::Value = serde_json::from_str(&json).expect("parse");
    v15_json.as_object_mut().unwrap().remove("offering_refs");
    let v15_str = serde_json::to_string(&v15_json).expect("re-serialize");

    // Deserialize as v1.6 struct — offering_refs must default to None
    let v16: amlich_core::DaySnapshot = serde_json::from_str(&v15_str).expect("v1.5→v1.6 deserialize");

    assert!(v16.applicable_rituals.is_some(), "applicable_rituals must survive");
    assert!(v16.daily_flying_stars.is_some(), "daily_flying_stars must survive (Phase 18)");
    assert!(v16.offering_refs.is_none(), "offering_refs must default to None on missing key");
}

#[test]
fn offering_refs_byte_equal_round_trip() {
    let snapshot = calculate_day_snapshot(17, 2, 2026);
    let json = serde_json::to_string(&snapshot).expect("serialize");
    let roundtripped: amlich_core::DaySnapshot = serde_json::from_str(&json).expect("deserialize");

    assert_eq!(snapshot.offering_refs, roundtripped.offering_refs);

    // Field-by-field assertion on at least one OfferingRef
    if let Some(refs) = &roundtripped.offering_refs {
        assert!(!refs.is_empty(), "offering_refs must be non-empty for Tết 2026");
        let first = &refs[0];
        assert!(!first.offering_id.is_empty());
        assert!(!first.name_vi.is_empty());
        assert_eq!(first.source_id, SOURCE_VN_FOLK_RITUAL);
    }
}

#[test]
fn offering_refs_absent_when_none() {
    let snapshot = calculate_day_snapshot(17, 2, 2026);
    let mut none_snapshot = snapshot.clone();
    none_snapshot.offering_refs = None;
    let json = serde_json::to_string(&none_snapshot).expect("serialize");
    assert!(!json.contains("\"offering_refs\""), "offering_refs must not appear in JSON when None");
}
```

### Example 5: E2E smoke extension (mirror `integration_2026_smoke.rs:139-181`)

```rust
// In tests/integration_2026_smoke.rs — Phase 19-03 (INT-10)
#[test]
fn e2e_2026_smoke_offers_offering_wiring_on_representative_dates() {
    use amlich_core::semantic_graph::builders::day_snapshot::build_day_snapshot_graph;
    use amlich_core::semantic_graph::{EdgeConcept, NodeConcept};

    let scanner = TietKhiScanner::new();
    let mut dates: Vec<(i32, i32, i32)> = Vec::new();

    // Tết Nguyên Đán 2026 — guaranteed applicable_rituals match + offerings
    dates.push((17, 2, 2026));

    // Lunar day 1 of months 3, 6, 9, 12 of 2026 — surface Sóc Vọng rituals with offerings
    dates.extend(collect_lunar_day_dates(1).into_iter().filter(|(_, m, _)| [3, 6, 9, 12].contains(m)));

    assert!(dates.len() >= 5, "must have ≥5 representative dates; got {}", dates.len());

    for &(d, m, y) in &dates {
        let snap = calculate_day_snapshot(d, m, y);
        let graph = build_day_snapshot_graph(&snap);

        // daily_flying_stars must be populated (Phase 18)
        assert!(snap.daily_flying_stars.is_some(),
                "daily_flying_stars must be Some for {y}-{m:02}-{d:02}");

        // If applicable_rituals is non-empty, Offering nodes + RecommendsOffering edges must exist
        if snap.applicable_rituals.as_ref().map_or(false, |r| !r.is_empty()) {
            let offering_nodes: Vec<_> = graph.nodes().values()
                .filter(|n| matches!(n.concept, NodeConcept::Offering))
                .collect();
            assert!(!offering_nodes.is_empty(),
                    "Offering nodes must exist for {y}-{m:02}-{d:02} when applicable_rituals is populated");

            let rec_edges: Vec<_> = graph.edges().values()
                .filter(|e| matches!(e.label.concept, EdgeConcept::RecommendsOffering))
                .collect();
            assert!(!rec_edges.is_empty(),
                    "RecommendsOffering edges must exist for {y}-{m:02}-{d:02}");
        }
    }
}
```

---

## Open Questions

### Q1 — Where does the "Ritual node payload" live? (BLOCKING for Plan 19-01)

Phase 19 SC#2 references "the Ritual semantic-graph node payload" exposing `offering_refs: Option<Vec<OfferingRef>>` AND the "legacy `offerings: Option<Vec<String>>` flat-string field". The actual codebase has NEITHER field today:

- The existing `SemanticNode` struct (`node.rs:14-27`) has fields: `id`, `node_id`, `concept`, `origin`, `summary_vi`, `severity`, `tags`, `provenance`. No payload.
- The existing `DaySnapshot` struct (`lib.rs:153-173`) has `applicable_rituals: Option<Vec<String>>` (the closest match to "legacy flat-string field", but it carries `ritual_id`s not offering names).

Three options:

| Option | Location of `offering_refs` + `offerings` | Tradeoff |
|--------|------------------------------------------|----------|
| **A** — On `DaySnapshot` | Both fields added to `DaySnapshot` alongside `applicable_rituals:167` and `daily_flying_stars:172` | Matches INT-08's literal text ("Ritual node payload" arguably means the snapshot — the user's view of "the ritual output"). Carries through serde additive pattern at lib.rs:166-167. Downside: the "node payload" naming is a bit imprecise — the snapshot isn't a graph node. |
| **B** — On `SemanticNode` (generic payload) | Add `payload: Option<serde_json::Value>` to `SemanticNode` (lib.rs is touched but the SemanticNode struct is in semantic_graph/node.rs) | "Node payload" naming is literal. Most flexible (any node concept can carry a payload). Downside: `serde_json::Value` is lossy and untyped; future readers won't know what's in the payload without checking the `concept` discriminator. |
| **C** — On `SemanticNode` (typed payload enum) | Add `payload: Option<RitualNodePayload>` to `SemanticNode`, where `RitualNodePayload` is a typed enum with `Ritual { offering_refs: Option<Vec<OfferingRef>>, offerings: Option<Vec<String>> }` variant | "Node payload" naming is literal AND typed. Matches the v1.5 `FlyingStarPeriod` enum pattern at `types.rs:99-125`. Downside: requires extending `SemanticNode` (small additive change) + new file/module for `RitualNodePayload` enum. |

**Recommendation**: **Option A** — both fields on `DaySnapshot`. Rationale:
1. INT-08's literal text says "Ritual node payload" but the "node" here most plausibly refers to the user's view of "the ritual thing in the snapshot" — `applicable_rituals` already lives on `DaySnapshot` and the v1.6 extension follows the same pattern.
2. The serde additive discipline (`#[serde(default, skip_serializing_if = "Option::is_none")]`) is already proven for `flying_stars:163`, `applicable_rituals:167`, `daily_flying_stars:172`.
3. Option C (typed enum payload on SemanticNode) is cleaner long-term but adds a new public type and a SemanticNode field change. Phase 18's pattern (aditive DaySnapshot field + graph builder extension) is the safer Phase 19 mirror.
4. The naming discrepancy (`offerings: Option<Vec<String>>` vs existing `applicable_rituals: Option<Vec<String>>`) is real — `applicable_rituals` carries `ritual_id`s, the new `offerings` carries offering names. Both are flat-string Vecs, but they have DIFFERENT semantics. The literal SC text wins; planner accepts the dual-field design.

**Planner must resolve Q1 BEFORE writing Plan 19-01.** The chosen option determines (a) which file gets the new fields, (b) the field names (if Option A: `offering_refs` + `offerings` on DaySnapshot), (c) the test file location (DaySnapshot round-trip test vs SemanticNode round-trip test).

### Q2 — How is a "non-ritual tradition offering reference surfaced inside a ritual" detected? (BLOCKING for Plan 19-02)

Phase 19 SC#3 gives one example: "a Huyền Không element cure surfaced inside a ritual". But the v1.5 corpus has NO place where a Huyền Không element cure is part of a `RitualEntry::offerings` Vec. The two traditions are disjoint:
- `RitualEntry.offerings: Vec<Offering>` (văn khấn, `vn-folk-ritual`) — concrete items like "Hương", "Hoa tươi", "Mâm ngũ quả"
- `FlyingStarLayout.center_star.element` → `Element::Metal` etc. (huyền không, `huyen-khong`) — abstract Ngũ Hành element classifications

The "surfaced inside a ritual" connection is a NEW design pattern. Three options:

| Option | Detection mechanism | Tradeoff |
|--------|---------------------|----------|
| **A** — Corpus augmentation | Add a `metadata.cross_source_curing: Option<Vec<ElementCure>>` field to `RitualEntry` schema. Authors annotate specific rituals with Huyền Không element cures. The builder reads this field and emits dual-source edges. | Cleanest design — explicit, corpus-driven, easy to audit. Downside: requires corpus augmentation + schema extension. |
| **B** — Heuristic inference | At builder time, when emitting an Offering node, check if the offering name matches a Huyền Không "cure" pattern (e.g., "Kim loại" for metal cure, "Nước" for water cure). Emit dual-source edge if match. | No corpus change. Downside: heuristic is fragile (Vietnamese text matching, language drift) and not auditable per-ritual. |
| **C** — Defer to a future phase | Phase 19 ships with SINGLE-source `RecommendsOffering` edges (only `vn-folk-ritual` provenance). INT-09's dual-source case is added in a later phase when the cross-source corpus annotation is ready. | Cleanest Phase 19 scope — INT-09 success criterion becomes a deferred item, like the 1960 Trung Nguyên case in Phase 16. Downside: INT-09 is in the Phase 19 requirements and is marked "Pending" in REQUIREMENTS.md:33. Deferring would leave it incomplete. |

**Recommendation**: **Option C** — defer the dual-source case. Rationale:
1. INT-09's example ("Huyền Không element cure surfaced inside a ritual") is a SYNTHETIC case — no real ritual in the v1.5 corpus carries one. Adding this as a corpus-augmentation requirement in Phase 19 would conflate schema work (Plan 19-01) with corpus work (a new corpus annotation).
2. The "multi-source Direction-node dedup pattern from v1.5" referenced in SC#3 is the GENERIC pattern (ProvenanceTracker::track appends). Phase 19 implements the pattern (single-source edges emitted via track()), and the dual-source case is a thin extension of the pattern (call track() twice). The pattern itself is what INT-09 requires; the specific dual-source example is illustrative.
3. Phase 16 deferred the 1960 case via `PendingExternalReview`; Phase 19 can defer the dual-source element-cure case similarly with an explicit "implemented in v1.7+ when cross-source corpus annotation is available" note.
4. The semantic-graph integration test (Example 5) can assert that `RecommendsOffering` edges carry EXACTLY ONE provenance entry (matching the v1.5 FlyingStar single-source precedent). When the dual-source case is added in v1.7+, the assertion becomes "exactly one OR two" — easy upgrade.

**Planner must resolve Q2 BEFORE writing Plan 19-02.** If Option C is chosen, Plan 19-02's INT-09 success criterion becomes a "deferred item" with explicit rationale.

### Q3 — `tests/day_snapshot_v14_compat.rs` extension vs sibling `tests/day_snapshot_v15_compat.rs`?

Phase 19 SC#4 says "extension of `tests/day_snapshot_v14_compat.rs` to the v1.6 surface (or a sibling `tests/day_snapshot_v15_compat.rs`)". The naming is confusing — `v14_compat` was for v1.4→v1.5 round-trip (Phase 15 added `flying_stars`/`applicable_rituals`), and Phase 18 EXTENDED that file with v1.5→v1.6 tests (3 new tests for `daily_flying_stars`).

Two options:

| Option | File structure | Tradeoff |
|--------|---------------|----------|
| **A** — Extend `day_snapshot_v14_compat.rs` | Append 3 more tests for `offering_refs` to the existing file (now 9 tests total: 3 for v1.4→v1.5, 3 for v1.5→v1.6 daily_flying_stars, 3 for v1.5→v1.6 offering_refs) | Mirrors the Phase 18-04 precedent exactly. Downside: file name is now misleading (it covers v1.4→v1.5 AND v1.5→v1.6). |
| **B** — New sibling `day_snapshot_v15_compat.rs` | Create new file with 3 tests for `offering_refs`. Existing file stays as-is for the daily_flying_stars tests. | Cleaner separation. Downside: two files for similar test purposes; test discovery is more scattered. |

**Recommendation**: **Option A** — extend the existing file. Rationale:
1. Phase 18-04 extended the same file (added 3 tests for daily_flying_stars) and the Phase 18-04 SUMMARY (`18-04-SUMMARY.md:131`) confirmed 6/6 tests pass. Extending again is the established pattern.
2. The file rename would be cosmetic (`v14_compat` → `v15_compat`) and would lose the v1.4→v1.5 round-trip tests (which Phase 15 added and which are still passing).
3. The "v1.5 JSON fixture" referenced in SC#4 has BOTH `flying_stars` (v1.5) AND `daily_flying_stars` (v1.6) — extending the existing file tests all three round-trips (v1.5 fields survive v1.6 strip, v1.6 fields survive v1.5 strip, byte-equal round-trip).

**Note**: planner may still choose Option B if file naming hygiene is preferred over test locality. Both are acceptable per SC#4.

### Q4 — `offerings: Option<Vec<String>>` flat-string field semantics (Option A in Q1)

If Q1 = Option A, the new `offerings: Option<Vec<String>>` field on `DaySnapshot` carries what exactly? Three interpretations:

| Interpretation | Field contents | Tradeoff |
|----------------|----------------|----------|
| **i** — Flattened offering names | `vec!["Hương", "Hoa tươi", "Mâm ngũ quả"]` (just `name_vi` values, deduplicated) | Matches literal SC text. Useful as a "shopping list" for users. |
| **ii** — Ritual IDs with offerings | `vec!["ritual.van-khan-tet-don-gian:3 offerings"]` (ritual_id + count) | More structured but weirdly shaped. |
| **iii** — Offering IDs | `vec!["ritual.van-khan-tet-don-gian.offering.0", ...]` (the `offering_id` values) | Loses the `name_vi` summary. |

**Recommendation**: **Interpretation i** — flattened offering names. Rationale:
1. Matches the literal SC text (`offerings: Option<Vec<String>>` flat-string — Vietnamese names are what "offerings" reads as in a UI).
2. Mirrors the v1.5 `applicable_rituals: Option<Vec<String>>` shape (flat-string summary for human consumption).
3. Provides a "shopping list" UX without the structured query overhead.

### Q5 — Builder integration: extend existing `add_ritual_facts` or new `add_offering_facts`?

Phase 19 SC#2's `offering_refs` field is conceptually attached to rituals, but Phase 19 SC#3's dual-source edge concerns offering provenance. Two integration approaches:

| Approach | Integration | Tradeoff |
|----------|-------------|----------|
| **A** — Extend `add_ritual_facts` | Add Offering node + RecommendsOffering edge emission inside the existing `add_ritual_facts(snapshot)` function at `day_snapshot.rs:539-567` | Single function for all ritual-related graph nodes. Downside: the function grows from 28 lines to ~100 lines; mixing concerns. |
| **B** — New `add_offering_facts` helper | Add new `add_offering_facts(snapshot)` function at `day_snapshot.rs:567+`, called from `DaySnapshotGraphBuilder::new` after `add_ritual_facts` (mirrors `add_flying_star_facts` / `add_ritual_facts` split) | Cleaner separation. Mirrors the v1.5 builder pattern. Downside: two passes through the rituals list (one in `add_ritual_facts`, one in `add_offering_facts`). |

**Recommendation**: **Approach B** — new `add_offering_facts` helper. Rationale:
1. Mirrors the established v1.5 builder split (`add_flying_star_facts` for Phi Tinh, `add_ritual_facts` for rituals, distinct concerns).
2. Keeps `add_ritual_facts` simple (28 lines, single concern).
3. The double-pass is acceptable — `all_rituals()` is `OnceLock`-loaded (instant), `get_ritual_by_id()` is O(n) per call but n=60 (the corpus size).
4. Per Phase 18-04's "populate-block grouping" pattern, the new function sits next to `add_ritual_facts` for readability.

---

## Sources

### HIGH confidence (in-repo architecture + discipline)

- **in-repo**: `crates/amlich-core/src/semantic_graph/ontology.rs:5-389` — the 6 slice locations (`NodeConcept` enum + `label()` match + `ConceptLabel` enum + `as_str()` match + `node_concepts()` slice + `v15_concepts_present_in_ontology_slices` test). Phase 19 adds `Offering` + `RecommendsOffering` to all 6 each.
- **in-repo**: `crates/amlich-core/src/semantic_graph/provenance.rs:130-135` — `ProvenanceTracker::track()` appends to Vec, no dedup. The single source of dedup truth (the implicit `SemanticId` join key). Phase 19 reuses this EXACTLY.
- **in-repo**: `crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs:475-567` — the v1.5 builder pattern: `add_flying_star_facts` (single-source `huyen-khong`) + `add_ritual_facts` (single-source `vn-folk-ritual`). The Direction node at line 829 carries BOTH `khcbppt` and `huyen-khong` provenance (proven by `direction_node_carries_dual_provenance_khcbppt_and_huyen_khong` test at lines 926-950) — this is the dual-source precedent.
- **in-repo**: `crates/amlich-core/src/rituals/schema.rs:104-150` — the `Offering` structured type (v1.5 corpus field) + `RitualEntry` schema-lock discipline (ADR-0001). Phase 19's `OfferingRef` mirrors the `RitualEntry` shape (locked struct with `deny_unknown_fields`).
- **in-repo**: `crates/amlich-core/src/rituals/corpus.rs:126` — `entry.source_id == SOURCE_VN_FOLK_RITUAL` enforcement at corpus load. The pattern for ensuring source_id discipline.
- **in-repo**: `crates/amlich-core/src/rituals/matcher.rs:21, 50` — `find_van_khan_for_snapshot` + `get_ritual_by_id`. Phase 19 builder uses both to iterate rituals and look up structured offerings.
- **in-repo**: `crates/amlich-core/src/sources.rs:1-42` — the 7 canonical `SOURCE_*` constants. Phase 19 reuses `SOURCE_VN_FOLK_RITUAL` and `SOURCE_HUYEN_KHONG`. No new source_id needed (per Q2 Option C).
- **in-repo**: `crates/amlich-core/src/lib.rs:153-173, 262-373` — `DaySnapshot` additive serde pattern + `calculate_day_snapshot_internal` populate-block grouping. Phase 19's `offering_refs`/`offerings` fields (Q1 Option A) go here.
- **in-repo**: `crates/amlich-core/tests/day_snapshot_v14_compat.rs:73-128` (per 18-04 SUMMARY) — the 3-property round-trip test pattern (missing key → None + byte-equal round-trip + None → absent in JSON). Phase 19 extends with 3 more tests for `offering_refs`.
- **in-repo**: `crates/amlich-core/tests/source_id_guard.rs:13-21` — the `FORBIDDEN_LITERALS` list. Phase 19 only extends this if Q2 = Option A (new source_id introduced). Otherwise no change.
- **in-repo**: `crates/amlich-core/tests/integration_2026_smoke.rs:139-181` — the 2026 E2E smoke test (≥30 dates). Phase 19 extends with ≥5 dates that exercise daily_flying_stars + Offering wiring.
- **in-repo**: `.planning/STATE.md:79` — Phase 18 verification is a prerequisite for Phase 19. Phase 19 does NOT touch `daily.rs`, `fengshui_crit3_isolation.rs`, or the daily populate block at `lib.rs:349-361`.
- **in-repo**: `.planning/PROJECT.md:81-88` — v1.5/v1.6 carry-forward decisions (source-id `pub const`, schema-lock before corpus, additive `Option<T>` discipline, CRIT-3 isolation, audit-as-decisive-source).
- **in-repo**: `.planning/research/ARCHITECTURE.md:263` — explicit v1.5 design decision: "RecommendsOffering — currently no offering node exists; defer to v1.6. v1.5 keeps offerings inside the ritual node payload as a flat string list." This is EXACTLY what Phase 19 closes.
- **in-repo**: `.planning/research/PITFALLS.md:339-349` — v1.5 source-id pitfalls (CRIT-1, MOD-4, MOD-5) carry forward. Phase 19's `OfferingRef.source_id` discipline mirrors the `RitualEntry.source_id` discipline.
- **in-repo**: `.planning/phases/18-daily-phi-tinh/18-04-SUMMARY.md:60-131` — the Phase 18-04 execution pattern (additive DaySnapshot field + populate block + 3 round-trip tests + grep guard). The exact precedent for Phase 19-03.

### MEDIUM confidence (synthetic design — dual-source non-ritual case)

- **synthetic**: The "non-ritual-tradition offering reference surfaced inside a ritual" case (INT-09 example: Huyền Không element cure) is a NEW design — no current corpus entry carries one. The Phase 19 implementation assumes the generic dual-source edge pattern (call `provenance_tracker.track(edge_id, entry)` twice) and DEFER the specific corpus augmentation to a future phase (Q2 Option C). Validated by reading v1.5 corpus schema at `rituals/schema.rs:104-150` — no `cross_source_curing` field exists.
- **in-repo**: `crates/amlich-core/src/semantic_graph/provenance.rs:43-46` — `with_note()` is the established mechanism for audit-friendly provenance metadata. Phase 19's `RecommendsOffering` edges use this for `rationale=...` audit trail.
- **in-repo**: `crates/amlich-core/src/almanac/fengshui/types.rs:99-125` (per 18-RESEARCH.md:127-149) — the `FlyingStarPeriod` enum + sibling `DailyFlyingStarLayout` struct pattern. Phase 19's `OfferingRef` follows the same pattern (locked struct first, then builder code emits).

### LOW confidence (awaiting planner decisions)

- **synthetic**: Q1 (payload location) — three options, planner must choose. Option A (DaySnapshot) is recommended based on precedent but Option C (typed SemanticNode payload) is the cleanest long-term design.
- **synthetic**: Q2 (dual-source detection) — Option C (defer the specific element-cure case) is recommended; the GENERIC pattern is implemented but the specific Huyền Không element cure corpus annotation is deferred.
- **synthetic**: Q3 (test file structure) — Option A (extend `v14_compat.rs`) is recommended; Option B (new sibling) is acceptable per SC#4.

---

## Validation Architecture

*Skipped per `.planning/config.json`: `workflow.nyquist_validation` is not present (the config has only `mode`, `depth`, `parallelization`, `commit_docs`, `model_profile`, `workflow.{research,plan_check,verifier}`). No validation gate design in this research doc.*

---

## Metadata

| Field | Value |
|-------|-------|
| Researched by | phase-research agent (Phase 19 prep) |
| Research date | 2026-07-15 |
| Confidence breakdown | **HIGH**: architecture (6 ontology slice locations, `ProvenanceTracker::track()` dedup pattern, builder extension point, additive serde discipline, source-id guard discipline, CRIT-3 isolation precedent, E2E smoke test extension point); schema-lock discipline for `OfferingRef`; v1.5/Phase 18 patterns carry forward rigidly. **MEDIUM-HIGH**: dual-source edge semantics (generic pattern implemented via `track()` append, but specific Huyền Không element-cure corpus augmentation is a future-phase item per Q2 Option C). **MEDIUM**: payload location (Q1 — Option A recommended but Q1 is open for planner). |
| Required for Phase 19 plans | **19-01**: `OfferingRef` struct locked (INT-08 first slice) + additive `offering_refs` + legacy `offerings` field location determined (Q1 resolution). **19-02**: `NodeConcept::Offering` + `EdgeConcept::RecommendsOffering` across all 6 slice locations + `add_offering_facts` builder (INT-07 + INT-09 generic pattern; INT-09 specific dual-source case deferred per Q2 Option C). **19-03**: v1.5→v1.6 round-trip test extension + 2026 E2E smoke extension with ≥5 dates exercising daily_flying_stars + Offering wiring (INT-10). |
| Pre-existing tech debt carried | None Phase-19-specific. The `.planning/phases/16-foundation-adr-0003-confidence-closure/deferred-items.md` 96 pre-existing clippy/fmt warnings remain — Phase 19 should NOT fix these per deviation-rule SCOPE BOUNDARY (carry-forward from Plan 16-02 Deviation #2). |
| Files likely to be created/modified | **Created**: `tests/day_snapshot_v15_compat.rs` (per Q3 Option A — extension preferred but sibling acceptable); possibly `tests/offering_integration.rs` (black-box tests if needed beyond the E2E smoke extension). **Modified**: `crates/amlich-core/src/semantic_graph/ontology.rs` (add 2 variants × 6 slice locations = ~12 lines); `crates/amlich-core/src/rituals/schema.rs` (add `OfferingRef` struct ~25 lines); `crates/amlich-core/src/rituals/mod.rs` (re-export `OfferingRef`); `crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs` (add `add_offering_facts` helper ~80 lines + call from `new()`); `crates/amlich-core/src/lib.rs` (add 2 additive fields to DaySnapshot ~10 lines + populate block ~20 lines); `tests/integration_2026_smoke.rs` (add ≥5 dates + Offering wiring assertion ~40 lines); `tests/day_snapshot_v14_compat.rs` (3 new tests ~80 lines). Total: ~270 lines net. |
| Open design points for planner | **Q1**: payload location (Option A recommended); **Q2**: dual-source detection (Option C recommended — defer specific element-cure case); **Q3**: test file structure (Option A recommended — extend `v14_compat.rs`); **Q4**: flat-string `offerings` semantics (interpretation i recommended — flattened `name_vi`); **Q5**: builder integration (Approach B recommended — new `add_offering_facts` helper). |
| Pre-requisite | Phase 18 verification (STATE.md:79). Phase 19 does NOT touch `daily.rs`, `flying_stars_daily_golden.json`, `fengshui_crit3_isolation.rs`, or the daily populate block at `lib.rs:349-361`. |
| Carries forward from v1.5/Phase 18 | Schema-lock-before-corpus; single-commit RED→GREEN; audit-as-decisive-source; external-crate black-box tests; additive serde `Option<T>` discipline; `pub const SOURCE_*` per module; `ProvenanceTracker::track()` append-pattern (no parallel dedup); CRIT-3 isolation; v1.1.2 Tiết Khí scanner reuse (not directly used by Phase 19 but maintained in repo). |

---

*Research completed: 2026-07-15*
*Ready for Phase 19 planning: yes, after Q1 + Q2 are resolved by planner*