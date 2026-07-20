# Phase 24: IChing Evaluator + Semantic-Graph Wiring + DTO Integration - Research

**Researched:** 2026-07-16
**Domain:** Rust integration of the shipped Mai Hoa I Ching primitives into an explicit evaluator, `DaySnapshot`, and semantic graph
**Confidence:** HIGH overall; MEDIUM for the Phase 23 return-type boundary because Phase 23 is not shipped

<user_constraints>
## User Constraints (from CONTEXT.md)

### Phase Boundary

Add the Tier-0 I Ching integration surface: an explicit `IChingQuery`/`IChingEvaluator` path over the already-shipped Mai Hoa casting, biến quẻ, Thể/Dụng, and corpus APIs; semantic-graph facts for the chủ quẻ, biến quẻ, and the Phase 23 directional cross-link; and additive `DaySnapshot` summaries with v1.6-to-v1.7 backward-compatible serialization.

This phase does not add coin/yarrow/RNG casting, alternative casting variants, LLM-generated interpretation, Bazi enrichment, visual/TUI graph presentation, or any `FlyingStar` wiring into `interaction/direction_merge.rs`. The Phase 23 directional cross-link implementation is a prerequisite/input, not new scope for this phase.

### Locked Decisions

#### DaySnapshot population and compatibility
- Ordinary `calculate_day_snapshot(...)` calls must not invent an I Ching hour or silently auto-cast. `iching_cast` remains `None` unless the caller explicitly requests I Ching enrichment.
- An explicit I Ching operation returns an enriched `DaySnapshot` value; it must not mutate the caller's original snapshot.
- `direction_cross_link` is populated only through an explicit enrichment request that supplies the required personal/birth-chi input. An I Ching-only enrichment must not invent personal context or populate the directional cross-link.
- New optional fields are omitted from JSON when absent and serialized normally when present. Missing fields in legacy v1.6-shaped JSON deserialize to `None`; re-serialization must not introduce new `null` keys.

### Claude's Discretion
- **Query/evaluator contract:** Choose the exact public shape for `IChingQuery`, the evaluator result, and any convenience enrichment constructor while preserving an explicit-call boundary, Tier-0 operation without birth data, and the already-locked sibling-newtype decision. Prefer clear domain types and a compound result struct over ambiguous primitive/boolean parameters.
- **Evidence semantics:** Preserve distinct primitive provenance for casting (`mai-hoa-dich-so`) and corpus text lookup (`kinh-dich`), plus exactly one composite envelope for the combined consultation. The planner may define the precise per-step method names, notes, and ordering by following the existing `ReasoningEvidenceEnvelope` and provenance patterns; do not collapse the primitive sources into the composite.
- **Semantic graph shape:** Follow existing `SemanticId`, `DaySnapshotGraphBuilder::add_*_facts`, `SemanticNode`, `SemanticEdge`, and `SemanticGraph::merge` patterns. The planner may choose the stable-key details and exact anchoring of the chủ/biến Hexagram nodes and Phase 23 composite fact, provided the graph exposes both Hexagram nodes, `LocatedAt`/`Transforms` edges, and the cross-link without changing ontology or violating CRIT-3 isolation.
- **Error and validation details:** Use existing crate conventions and choose appropriate validation for explicit query input, missing Phase 23 data, and impossible/invariant-breaking states. Do not add external dependencies or invent a new error framework.

### Deferred Ideas (OUT OF SCOPE)

- Visual semantic-graph/TUI workspace presentation — separate UI/product phase.
- Coin, yarrow, RNG, sound-number, or user-selectable casting variants — outside the v1.7 Mai Hoa time-numerology boundary.
- Free-form LLM interpretation, Hỗ Quả/nuclear hexagram, and Tier-2 Bazi enrichment — later phases/milestones.
- Spatial Feng Shui composition involving `FlyingStar` in `interaction/direction_merge.rs` — explicitly prohibited by the phase boundary.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| ICH-05 | A caller can construct an `IChingQuery` (sibling newtype, NOT a `ConsultationIntent` variant) and run it through an `IChingEvaluator` that emits per-step `ReasoningEvidenceEnvelope` instances with distinct source_ids (`mai-hoa-dich-so` for casting + `kinh-dich` for text lookup) plus one composite envelope (CRIT-6) — works fully at Tier 0 (no birth data required). | Use `src/iching/evaluator.rs` as the Phase 22-aligned home for the sibling query, stateful evaluator, compound consultation result, and explicit snapshot enrichment. Use the shipped Phase 22 functions and constants; emit at least the two primitive source IDs plus exactly one composite envelope. Keep the evaluator independent of `PersonalReasoningInput`'s birth-required Bazi path. |
| INT-11 | User-of-semantic-graph can find Hexagram nodes (chủ quẻ + biến quẻ) wired via `LocatedAt`/`Transforms` edges and a composite cross-link fact node, emitted by additive `add_iching_facts()` + `add_direction_composite_facts()` builder methods (v1.5 FlyingStar/Offering precedent). | `NodeConcept::Hexagram`, `EdgeConcept::LocatedAt`, and `EdgeConcept::Transforms` are already present in the six ontology slices. Extend `DaySnapshotGraphBuilder` with additive methods that early-return on absent fields, add nodes before edges, use deterministic `SemanticId` keys, and consume the Phase 23 output rather than recomputing it. |
| INT-12 | User-of-`DaySnapshot` can find additive `iching_cast: Option<IChingCastSummary>` and `direction_cross_link: Option<DirectionCrossLinkSummary>` fields (`#[serde(default, skip_serializing_if = "Option::is_none")]`), with a v1.6→v1.7 backward-compat round-trip test proving a v1.6 producer JSON deserializes cleanly and re-serializes without unexpected fields. | Add both fields at the end of `DaySnapshot`, initialize them to `None` in ordinary snapshot construction, and extend `tests/day_snapshot_v14_compat.rs` using the Phase 19-03 combined-strip pattern to remove both v1.7 keys from a fully populated JSON object before deserializing and semantically comparing the re-serialization. |
</phase_requirements>

## Summary

Phase 24 is an integration phase, not a new I Ching algorithm phase. The actual checkout has already shipped the Phase 22 surface under `crates/amlich-core/src/iching/`: `MaiHoaCast`/`cast_mai_hoa`, `BienQue`/`derive_bien_que`, `TheDungClassification`/`classify_the_dung`, corpus lookup, and `load_mai_hoa_golden`. Do not create a second `reasoning/iching/` tree from the older v1.7 research sketch, do not duplicate any casting or transformation logic, and do not change the locked ontology/source constants already delivered in Phase 20.

The recommended implementation is a sibling `IChingQuery` plus a query-owned `IChingEvaluator` in a new `src/iching/evaluator.rs`. The evaluator should return a compound, owned consultation result containing the cast, transformed hexagram, Thể/Dụng classification, corpus projections, and evidence. A separate explicit enrichment operation should clone a `DaySnapshot`, attach an `IChingCastSummary`, and leave the input unchanged. `ActionEvaluator` can be implemented as an adapter over that query-owned evaluator, but the rich I Ching result must not be reduced to the generic `ActionEvaluation` shape.

The semantic graph should remain additive: builder methods read already-populated optional DTO fields, create both Hexagram nodes and a distinct directional composite fact node, then add only valid edges. The Phase 23 cross-link is an upstream dependency, not work to reproduce in Phase 24. Phase 24 can ship and test the I Ching-only path independently; the directional enrichment and graph slice must consume the public Phase 23 function/output once that parallel phase is merged.

**Primary recommendation:** Add the evaluator and explicit immutable enrichment first, then add the two optional DTO fields and compatibility contract, then wire graph facts behind those fields with a compile-time dependency block for the Phase 23 API.

## Standard Stack

### Core

| Existing facility | Version/status | Phase 24 use | Why it is standard |
|---|---|---|---|
| Rust `serde` / `serde_json` | Workspace `1.0` | Serialize query/result/summary DTOs and run compatibility tests | All public DTOs and corpus types already use Serde derives. |
| `unicode-normalization` | `0.1.25` | NFC-normalize query text if the constructor accepts free text | Already a direct dependency and the corpus loader applies the same policy. |
| `std::sync::OnceLock` + `include_str!` | Already shipped | Indirectly consumed through corpus and golden lookup | `all_hexagrams()` and `load_mai_hoa_golden()` are already cached compile-embedded surfaces. |
| `ReasoningEvidenceEnvelope` | Existing | Per-step I Ching provenance | Its fields are `source_family`, `source_id`, `method`, and optional `note`; use `ReasoningEvidenceSourceFamily::IChing` for primitive I Ching evidence and `Derived` for the composite. |
| `SemanticGraph` substrate | Existing | Hexagram and directional graph facts | Existing `SemanticId`, `SemanticNode`, `SemanticEdge`, provenance, edge endpoint validation, and merge behavior are the required graph primitives. |

### Existing Phase 22 API to consume

| API | Location | Use in Phase 24 |
|---|---|---|
| `cast_mai_hoa(...) -> MaiHoaCast` | `src/iching/mai_hoa.rs:84-131` | Casting step; preserve `SOURCE_MAI_HOA_DICH_SO` provenance. |
| `derive_bien_que(&MaiHoaCast) -> BienQue` | `src/iching/bien_que.rs:91-140` | Primary-to-transformed derivation; do not reimplement line flipping. |
| `classify_the_dung(&MaiHoaCast) -> TheDungClassification` | `src/iching/the_dung.rs:203-280` | Thể/Dụng and verdict in the compound result. |
| `get_hexagram(KingWenHexagram)` | `src/iching/corpus.rs:75-82` | Primary and biến corpus text lookup; use `SOURCE_KINH_DICH`. |
| `load_mai_hoa_golden()` | `src/iching/golden.rs:160-169` | Evaluator contract tests can use the shipped 12-case dataset; it is not a new runtime evaluator dependency. |

### No installation or dependency change

`crates/amlich-core/Cargo.toml:16-20` already contains the complete required dependency set: workspace `serde`, `serde_json`, `chrono`, and direct `unicode-normalization = "0.1.25"`. Do not add an I Ching crate, graph crate, RNG crate, schema-validator crate, or calendar crate.

## Architecture Patterns

### 1. Sibling query and stateful evaluator

The locked architecture decision is the sibling-newtype pattern in `.planning/research/ARCHITECTURE.md:219-250` and the anti-pattern guidance at `.planning/research/ARCHITECTURE.md:436-440`: do not add a payload-carrying `ConsultationIntent::IChing` variant. `ConsultationIntent` remains a closed, `Copy` activity enum.

The research sketch names the conceptual query fields `question_vi` and query-time data (`ARCHITECTURE.md:232-239`). Adapt that shape to the shipped code rather than copying it literally: `SolarDate` currently contains only day/month/year (`src/lib.rs:110-116`), while the shipped casting API requires lunar year-branch, lunar month, lunar day, and an explicit chi-hour index. The recommended public shape is:

- `IChingQuery` is a non-`Copy` sibling type with `question_vi: String` and an explicit chi-hour domain value.
- A constructor such as `from_snapshot(snapshot, question_vi, chi_hour_index)` derives the lunar date and lunar year branch from the supplied snapshot, avoiding duplicated or mismatched date primitives.
- Validate the hour index at construction (`0..=11`) and reject an empty/whitespace-only question if question text is part of the public contract; normalize accepted text to NFC.
- If a direct lunar-input constructor is needed for golden tests, keep it explicit and validate it against the Phase 22 input ranges. Do not make the evaluator silently infer an hour from a date-only snapshot.
- `IChingEvaluator` owns an `IChingQuery` and delegates to an inherent rich-evaluation method. This is necessary because the existing `ActionEvaluator` trait has no query argument (`src/reasoning/action_evaluator.rs:51-67`).

Recommended conceptual result:

```rust
pub struct IChingEvaluation {
    pub query: IChingQuery,
    pub cast: MaiHoaCast,
    pub bien_que: BienQue,
    pub the_dung: TheDungClassification,
    pub chu_hexagram: HexagramEntry,
    pub bien_hexagram: HexagramEntry,
    pub evidence: Vec<ReasoningEvidenceEnvelope>,
}
```

The exact field names can follow project convention, but the result should be owned and serializable. Clone the two corpus entries or project them into an owned `HexagramSummary`; do not expose `&'static HexagramEntry` in the public snapshot DTO. The `IChingCastSummary` stored on `DaySnapshot` should be a stable, additive projection of this result, not a borrowed corpus handle.

`ActionEvaluator::evaluate` should delegate to the same rich evaluation and map the result into `ActionEvaluation` only as an adapter. `select_subgraph` should operate on the already-built graph and must not require `PersonalReasoningInput`. The optional personal argument is ignored for the Tier-0 baseline. Do not put the I Ching path inside the current `PersonalReasoningInput::build_fact_nodes` without first removing its unconditional Bazi construction (`src/reasoning/personal.rs:31-38`), because that path requires birth data and would violate MOD-7.

### 2. Explicit immutable snapshot enrichment

Existing `DaySnapshot` is `Clone` and its ordinary constructor is centralized in `calculate_day_snapshot_internal` (`src/lib.rs:275-340`). Keep the normal calculation path unchanged except for initializing the new fields to `None`.

Use an explicit operation with value semantics:

```rust
pub fn enrich_day_snapshot_with_iching(
    snapshot: &DaySnapshot,
    query: IChingQuery,
) -> Result<DaySnapshot, String>
```

The operation should evaluate against the input snapshot, clone it, assign the owned `IChingCastSummary`, and return the clone. It must not add an implicit query to `calculate_day_snapshot(...)`, invent a chi hour, or mutate the original. A combined enrichment request may be added if needed, but it should use named domain fields such as `iching: Option<IChingQuery>` and `direction_cross_link: Option<DirectionCrossLinkQuery>` rather than booleans or a positional tuple.

The directional operation is separate and requires the caller's birth-chi input. If both enrichments are requested, compute I Ching from the date snapshot and call the Phase 23 cross-link with the explicit birth-chi input; I Ching-only requests leave `direction_cross_link` as `None`.

### 3. Evidence envelope contract

The consultation evidence must retain step provenance. Use the existing constants from `src/sources.rs:28-32`; never use the two source literals at production call sites.

Recommended ordering:

1. `ReasoningEvidenceSourceFamily::IChing`, `SOURCE_MAI_HOA_DICH_SO`, method describing the time-number cast and its lunar/hour inputs.
2. `ReasoningEvidenceSourceFamily::IChing`, `SOURCE_MAI_HOA_DICH_SO`, method describing biến quẻ and/or Thể/Dụng derivation if those are surfaced as separate steps.
3. `ReasoningEvidenceSourceFamily::IChing`, `SOURCE_KINH_DICH`, method describing the primary and biến corpus lookups and selected text.
4. `ReasoningEvidenceSourceFamily::Derived`, composite source ID `rule.composite.iching_consultation`, method describing the combined consultation.

The minimum contract is at least two primitive entries whose distinct IDs include `mai-hoa-dich-so` and `kinh-dich`, plus exactly one composite entry. Tests should assert the ID set and composite count, not over-constrain future per-step method count. The composite must not replace the primitive envelopes.

For graph provenance, preserve the same three-source distinction. The current `semantic_graph::ProvenanceSource` enum does not yet have an I Ching variant (`src/semantic_graph/provenance.rs:6-15`). Add an `IChing` variant and map it to `ReasoningEvidenceSourceFamily::IChing` in `to_reasoning_evidence()` (`src/semantic_graph/provenance.rs:101-116`) so Hexagram node provenance does not get mislabeled as an almanac rule. Use `ProvenanceSource::Derived` for the composite envelope. This is an additive provenance-enum extension, not an ontology change.

### 4. Additive `DaySnapshot` DTO fields

`DaySnapshot` currently establishes the exact pattern at `src/lib.rs:154-185`:

```rust
#[serde(default, skip_serializing_if = "Option::is_none")]
pub iching_cast: Option<IChingCastSummary>,

#[serde(default, skip_serializing_if = "Option::is_none")]
pub direction_cross_link: Option<DirectionCrossLinkSummary>,
```

Place both fields after the existing v1.6 fields to keep the declaration and serialized ordering additive. Initialize both to `None` in the ordinary snapshot literal at `src/lib.rs:327-340`. Do not auto-populate either field in `calculate_day_snapshot_internal`.

The summary types should own their serialized data. Include enough stable scalar/structured information for consumers to identify the primary and transformed King Wen indices, names, moving line, Thể/Dụng classification, verdict, and directional cross-link content without storing borrowed corpus references. If evidence is stored in the summary, use the same envelope contract; otherwise the graph builder must reconstruct only provenance metadata from the summary and must not recompute the underlying rule.

### 5. Combined-strip v1.6-to-v1.7 compatibility

The authoritative precedent is `crates/amlich-core/tests/day_snapshot_v14_compat.rs:193-280`, which strips every new v1.6 field together, deserializes the v1.5-shaped JSON, and compares parsed JSON values after re-serialization. Phase 24 should extend the same test rather than create a narrower one:

1. Start from a populated snapshot and serialize it as the v1.7-shaped baseline.
2. Remove `iching_cast` and `direction_cross_link` together, in addition to the existing v1.6 fields when the fixture is used as a historical producer shape.
3. Deserialize into the current `DaySnapshot`.
4. Assert both new fields default to `None`.
5. Assert existing fields survive.
6. Re-serialize and compare parsed `serde_json::Value` objects, because map-key ordering can differ after stripping through `Value`.
7. Assert neither new key appears and no `null` key is introduced.

Also add a populated I Ching round-trip test proving present values survive byte-equivalent serialization. Keep ordinary snapshots free of the new keys.

### 6. Additive semantic-graph builder methods

`DaySnapshotGraphBuilder::new` calls one method per optional surface at `src/semantic_graph/builders/day_snapshot.rs:32-44`. Add:

```rust
builder.add_iching_facts(snapshot);
builder.add_direction_composite_facts(snapshot);
```

Each method should early-return when the corresponding optional field is absent. For I Ching:

- Create a deterministic primary Hexagram node and transformed Hexagram node using `SemanticId` stable keys scoped to the day and timezone.
- Use `NodeConcept::Hexagram`, `NodeOrigin::Fact`, summaries/tags that identify `chu_que` versus `bien_que`, King Wen index, moving line, and names.
- Add provenance entries for casting/derivation and corpus lookup using the registered source constants.
- Add both nodes before adding edges. `SemanticGraph::add_edge` silently drops an edge unless both endpoint nodes already exist (`src/semantic_graph/graph.rs:19-27`).
- Add `EdgeConcept::Transforms` from primary to transformed.
- Add `EdgeConcept::LocatedAt` from each Hexagram node to the day-root or another stable day anchor selected by the plan. The plan must use one consistent direction and test it.

For the directional composite:

- Use an existing ontology concept, preferably a distinct `NodeConcept::Direction` node with a cross-link-specific stable key, rather than adding a new concept.
- Preserve the Phase 23 result as structured payload when needed; `SemanticNode.payload` is already the generic additive JSON mechanism (`src/semantic_graph/node.rs:27-38`) and `nodes_mut()` is the established post-population pattern (`src/semantic_graph/graph.rs:50-58`).
- Add the cross-link node only when `snapshot.direction_cross_link` is `Some`.
- Attach separate primitive provenance entries plus the single composite provenance entry. Do not merge KHCBPPT and Huyền Không into a new tradition source ID.
- Anchor the composite fact to the day with the selected existing edge pattern. Do not reuse the existing general travel-direction node ID; separate stable IDs keep the two directional surfaces auditable.

No changes are needed to `semantic_graph/ontology.rs`: `Hexagram`, `LocatedAt`, and `Transforms` are already present and registered in all required slices (`src/semantic_graph/ontology.rs:3-44`, `91-125`, `165-236`, `371-448`).

### 7. Phase 23 dependency boundary

Phase 23 is currently absent from the checkout: there is no `src/reasoning/direction_composite.rs` and no `build_direction_cross_link` implementation. The locked contract is in `.planning/adrs/0007-cross-link-crit3-carve-out.md:21-83` and the Phase 23 roadmap at `.planning/ROADMAP.md:102-116`.

Phase 24 planning must include a dependency context block with these rules:

- Treat `build_direction_cross_link(snapshot, birth_chi_index)` as an upstream public API supplied by Phase 23.
- Expect a read-only `reasoning/` implementation with primitive `khcbppt` and `huyen-khong` evidence plus exactly one `rule.composite.direction_cross_link` envelope.
- Do not create or copy `direction_composite.rs`, Tam Sát logic, directional Thái Tuế logic, or the Phase 23 CRIT-3 guard in Phase 24.
- Do not import any Phi Tinh type into `interaction/direction_merge.rs`; the existing `tests/fengshui_crit3_isolation.rs` contract remains unchanged.
- Because the exact Phase 23 return type is explicitly left to that phase's discretion, make the directional Phase 24 task consume the shipped public type or its documented conversion, not a guessed internal structure. If Phase 23 follows the roadmap's `PersonalFactNode` return, Phase 24 may perform a pure DTO projection from that node; it must not recalculate directional signals.
- Keep the I Ching evaluator, I Ching DTO, and I Ching graph tests independently buildable so Phase 24's IChing-only surface does not wait on the parallel track.
- The graph builder takes only `&DaySnapshot`, so it should read a previously populated `direction_cross_link` summary. The explicit enrichment operation is where the caller supplies `birth_chi_index` and invokes Phase 23.

## Don't Hand-Roll

| Problem | Do not build | Use instead |
|---|---|---|
| Mai Hoa casting | A second modulo implementation or date/hour inference | `crate::iching::cast_mai_hoa` and the validated Phase 22 input path |
| Biến quẻ | New bit packing, line order, or trigram composition | `crate::iching::derive_bien_que` and its 384-case contract |
| Thể/Dụng | A second five-element relation table | `crate::iching::classify_the_dung` and `TheDungClassification` |
| Corpus lookup | Copying corpus fields into evaluator logic or loading JSON again | `get_hexagram`, which uses the cached normalized corpus |
| Golden data | A new Phase 24 fixture duplicating Phase 22's expected values | `load_mai_hoa_golden()` for evaluator contract coverage; Phase 25 owns expanded E2E golden validation |
| Source IDs | Bare literals or a hybrid I Ching source ID | `SOURCE_MAI_HOA_DICH_SO`, `SOURCE_KINH_DICH`, and one local composite rule ID |
| Intent dispatch | A `ConsultationIntent::IChing` variant | The sibling `IChingQuery` newtype |
| Graph model | `petgraph`, a new ontology concept, or an ad hoc graph map | Existing `SemanticGraph`, `SemanticId`, `SemanticNode`, `SemanticEdge`, and `SemanticGraph::merge` |
| Phase 23 cross-link | Recomputed directional joins or copied Tam Sát code | The Phase 23 `build_direction_cross_link` API and a pure DTO adapter |
| Compatibility | A custom deserializer or nullable placeholder fields | `Option<T>` with `serde(default, skip_serializing_if = "Option::is_none")` and the combined-strip test |

## Common Pitfalls

### CRIT-6: source IDs collapsed or reversed

**Failure:** The evaluator labels the complete consultation as only `kinh-dich`, only `mai-hoa-dich-so`, or a made-up combined tradition.

**Prevention:** Emit separate primitive envelopes. Casting/derivation uses `SOURCE_MAI_HOA_DICH_SO`; corpus text lookup uses `SOURCE_KINH_DICH`; the combined reading gets exactly one `rule.composite.iching_consultation` envelope. Assert the primitive ID set and composite count in a black-box test.

**Warning sign:** A single helper accepts `source_id` from the caller or every evidence entry uses the same method/source pair.

### MOD-7: birth data accidentally required

**Failure:** The implementation routes I Ching through `PersonalReasoningInput::build_fact_nodes`, which constructs a Bazi chart before dispatch, or returns an unsupported/error result when `personal_input` is `None`.

**Prevention:** Keep the query/evaluator path independent of birth input. Make the `ActionEvaluator` adapter ignore the optional personal argument for baseline evaluation. Test the same query with no birth data and verify a complete result.

**Warning sign:** The public entry point accepts `BirthInput` as a required argument or calls `to_bazi_input()` before evaluating the query.

### Implicit casting in ordinary snapshots

**Failure:** `calculate_day_snapshot(...)` chooses a default hour, populates `iching_cast`, or makes ordinary graph snapshots contain I Ching nodes.

**Prevention:** Initialize the fields to `None` and only populate through a new explicit enrichment operation. Add a test that an ordinary snapshot serializes without `iching_cast`.

### Mutating the caller snapshot

**Failure:** An enrichment function takes `&mut DaySnapshot` or modifies the input before returning.

**Prevention:** Evaluate from `&DaySnapshot`, clone after successful evaluation, assign the summary to the clone, and assert the source snapshot remains `None` in the test.

### Date-only `SolarDate` treated as an hour-bearing query

**Failure:** The evaluator invents an hour from a date-only `SolarDate`, or silently maps an absent hour to a default.

**Prevention:** Carry an explicit chi-hour in `IChingQuery` or a dedicated query-time type. Derive the lunar date and year branch from the snapshot; validate the explicit hour before calling Phase 22.

### Graph edges dropped due to insertion order

**Failure:** Nodes are added after `SemanticEdge`, so `SemanticGraph::add_edge` drops `LocatedAt` or `Transforms` without an error.

**Prevention:** Add and assert both Hexagram endpoints before each edge. Test edge count and exact endpoint IDs, not only node presence.

### Primary/biến identity conflated

**Failure:** Both nodes use the same stable key or the transformed node is looked up using the primary King Wen index.

**Prevention:** Keep `MaiHoaCast.chu_que`, `BienQue.king_wen`, and the two corpus lookup results distinct. Use role-bearing stable keys such as `...:chu` and `...:bien` and test that the node IDs differ for a known moving line.

### Directional logic recreated in Phase 24

**Failure:** The planner adds a fallback direction algorithm because Phase 23 is unavailable locally, causing two implementations or a hidden Phase 23 dependency.

**Prevention:** Split the plan into an I Ching-independent wave and a directional wiring wave. The latter has an explicit Phase 23 dependency and only adapts the shipped output into `DirectionCrossLinkSummary` and graph provenance.

### Backward compatibility weakened by `null`

**Failure:** New fields are non-optional, lack `serde(default)`, or serialize as `null` when absent.

**Prevention:** Use the exact field attributes and extend the combined-strip test. Compare parsed JSON values after round-trip and assert the new keys are absent when `None`.

### Graph provenance mislabeled

**Failure:** I Ching nodes use `ProvenanceSource::AlmanacRule`, so converting graph provenance to reasoning evidence loses the already-registered I Ching family.

**Prevention:** Add the additive `ProvenanceSource::IChing` mapping or otherwise prove the graph-to-reasoning conversion preserves `ReasoningEvidenceSourceFamily::IChing`. Test both primitive source IDs on the graph node.

## Files to Create and Modify

### Create

| File | Purpose |
|---|---|
| `crates/amlich-core/src/iching/evaluator.rs` | `IChingQuery`, query-owned `IChingEvaluator`, compound evaluation result, `IChingCastSummary`, explicit immutable enrichment helpers, evidence construction, and `ActionEvaluator` adapter. |
| `crates/amlich-core/tests/iching_evaluator_integration.rs` | External-crate contract tests for sibling query construction, deterministic composition of Phase 22 APIs, Tier-0 no-birth operation, evidence source IDs, and no mutation. |
| `crates/amlich-core/tests/semantic_graph_iching_integration.rs` | External-crate graph tests for primary/biến Hexagram nodes, `LocatedAt`/`Transforms` edges, provenance, and conditional directional composite node wiring. |

Do not create `reasoning/iching/`, `direction_composite.rs`, Tam Sát modules, new ontology variants, corpus files, or new dependency manifests in Phase 24.

### Modify

| File | Change |
|---|---|
| `crates/amlich-core/src/iching/mod.rs` | Register `evaluator` and re-export the public query/evaluator/summary/result types alongside the shipped Phase 22 APIs. |
| `crates/amlich-core/src/reasoning/mod.rs` | Deliberately re-export evaluator-facing reasoning types if the public API convention requires them; do not add a birth-required personal branch. |
| `crates/amlich-core/src/lib.rs` | Add owned summary types or their public re-exports, add the two additive `DaySnapshot` fields, initialize them to `None`, and expose explicit enrichment entry points. Keep ordinary calculation behavior unchanged. |
| `crates/amlich-core/src/semantic_graph/provenance.rs` | Add `ProvenanceSource::IChing` and map it to `ReasoningEvidenceSourceFamily::IChing`, if using graph provenance conversion as recommended. |
| `crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs` | Wire `add_iching_facts()` and `add_direction_composite_facts()` into `new()` and implement optional node/edge construction. Consume the Phase 23 public output through the populated summary; do not recreate it. |
| `crates/amlich-core/tests/day_snapshot_v14_compat.rs` | Extend the existing combined-strip v1.6 compatibility test and add present/absent round-trip assertions for both Phase 24 fields. |

### Do not modify

`src/iching/mai_hoa.rs`, `bien_que.rs`, `the_dung.rs`, `corpus.rs`, and `golden.rs` are shipped Phase 22 implementation surfaces. `src/semantic_graph/ontology.rs`, `src/reasoning/types.rs` source-family/action variants, and `src/sources.rs` constants are already delivered by Phase 20. `src/interaction/direction_merge.rs` remains CRIT-3 protected.

## Code Examples

### Evaluator flow

```rust
let evaluation = IChingEvaluator::new(query).evaluate_consultation(snapshot)?;
let enriched = evaluation.enrich_snapshot(snapshot)?;
```

The implementation should use the Phase 22 pipeline in this order:

```rust
let cast = cast_mai_hoa(year_branch, month, day, hour);
let bien_que = derive_bien_que(&cast);
let the_dung = classify_the_dung(&cast);
let chu = get_hexagram(cast.chu_que).ok_or_else(|| "missing primary hexagram".to_string())?;
let bien = get_hexagram(bien_que.king_wen).ok_or_else(|| "missing transformed hexagram".to_string())?;
```

### Evidence contract assertion

```rust
let primitive_ids: std::collections::HashSet<_> = evaluation
    .evidence
    .iter()
    .filter(|e| e.source_id != "rule.composite.iching_consultation")
    .map(|e| e.source_id.as_str())
    .collect();

assert!(primitive_ids.contains(SOURCE_MAI_HOA_DICH_SO));
assert!(primitive_ids.contains(SOURCE_KINH_DICH));
assert_eq!(
    evaluation
        .evidence
        .iter()
        .filter(|e| e.source_id == "rule.composite.iching_consultation")
        .count(),
    1
);
```

The production composite ID may be held in a local named constant; the two primitive IDs must use the registered `SOURCE_*` constants.

### Graph construction order

```rust
self.graph.add_node(primary_node);
self.graph.add_node(transformed_node);
self.graph.add_edge(SemanticEdge::new(
    &primary_id,
    &transformed_id,
    EdgeConcept::Transforms,
));
self.graph.add_edge(SemanticEdge::new(
    &primary_id,
    &self.day_root_id,
    EdgeConcept::LocatedAt,
));
self.graph.add_edge(SemanticEdge::new(
    &transformed_id,
    &self.day_root_id,
    EdgeConcept::LocatedAt,
));
```

The exact anchor is a plan-level choice, but both endpoint nodes must exist first and the test must assert the selected direction.

## Validation Strategy

The project config does not enable the optional `workflow.nyquist_validation` section, so the standard validation-architecture template is omitted. The following checks are still required for planning and implementation:

| Requirement | Test surface | Expected command |
|---|---|---|
| ICH-05 | `tests/iching_evaluator_integration.rs` | `cargo test -p amlich-core --test iching_evaluator_integration` |
| INT-11 | `tests/semantic_graph_iching_integration.rs` plus builder unit tests | `cargo test -p amlich-core --test semantic_graph_iching_integration` |
| INT-12 | `tests/day_snapshot_v14_compat.rs` | `cargo test -p amlich-core --test day_snapshot_v14_compat` |
| Existing contracts | Phase 22 I Ching integrations and graph/source guards | `cargo test -p amlich-core` |

Minimum assertions should cover:

- An ordinary snapshot has `iching_cast.is_none()` and serializes without the key.
- An explicit I Ching request succeeds with no birth data and does not mutate its source snapshot.
- The evaluation evidence contains both primitive IDs and exactly one I Ching composite envelope.
- Primary and transformed corpus entries survive summary serialization.
- The graph contains two distinct `NodeConcept::Hexagram` nodes, one `Transforms` edge, and the planned `LocatedAt` edges.
- Directional enrichment is absent for I Ching-only requests and present only when the caller supplies the Phase 23 personal/birth-chi input.
- A v1.6-shaped JSON object with both new keys absent deserializes to `None` fields and re-serializes without either key or a `null` value.
- Existing `source_id_guard.rs` and `fengshui_crit3_isolation.rs` remain green; no Phase 24 code touches the quarantined interaction module.

## Open Questions

1. **What exact public return type will Phase 23 expose?**
   - What we know: ADR-0007 locks the read-only module, three-envelope pattern, and explicit `birth_chi_index`; the roadmap describes a `PersonalFactNode` return, while the ADR leaves the exact signature/type to Phase 23.
   - What is unclear: whether Phase 23 ships `PersonalFactNode`, a dedicated `DirectionCrossLink`, or a conversion pair.
   - Recommendation: make the directional Phase 24 plan depend on Phase 23's merged public API. Add only a pure DTO projection if necessary; do not recreate any directional computation. Keep I Ching-only tasks independent.

2. **Should `IChingCastSummary` carry full evidence or only the owned cast projection?**
   - What we know: evaluator evidence is mandatory; existing `FlyingStarsSummary` is intentionally slim, while graph provenance must remain queryable.
   - What is unclear: whether downstream snapshot consumers need envelopes in the DTO itself.
   - Recommendation: keep the summary owned and stable, include evidence if the public contract wants end-to-end auditability, and otherwise ensure the graph builder can attach the same fixed provenance without recomputation. Do not store borrowed corpus entries.

No external library research is needed for Phase 24. The remaining uncertainty is an internal parallel-track API boundary, not a missing technology choice.

## Key References

### Locked phase and requirements

- `.planning/phases/24-iching-evaluator-semantic-graph-wiring-dto-integration/24-CONTEXT.md:6-84` — phase boundary, immutable enrichment, no auto-cast, evidence/graph discretion, deferred scope.
- `.planning/REQUIREMENTS.md:24-35` — ICH-05, INT-11, and INT-12 acceptance language.
- `.planning/ROADMAP.md:118-132` — Phase 24 dependencies and success criteria.
- `.planning/STATE.md:33-65,89-105` — v1.7 dependency graph, shipped Phase 22 surfaces, and CRIT-6/MOD-7 history.

### Phase 24 architecture and precedent

- `.planning/research/ARCHITECTURE.md:219-250` — sibling-newtype and evaluator contract sketch.
- `.planning/research/ARCHITECTURE.md:253-318` — composite provenance pattern.
- `.planning/research/ARCHITECTURE.md:322-415` — I Ching and directional data flow.
- `.planning/research/ARCHITECTURE.md:434-487` — forbidden `ConsultationIntent` and CRIT-3 interaction patterns.
- `.planning/research/STACK.md:10-20,23-40` — no-new-dependency stack and existing standard library facilities.
- `.planning/research/PITFALLS.md:158-206,302-344` — CRIT-6, additive DTO, evidence granularity, and Tier-0 pitfalls.
- `.planning/research/FEATURES.md:350-364` — Tier-0 versus Tier-2 boundary.

### Shipped implementation anchors

- `crates/amlich-core/src/iching/mod.rs:1-27` — actual Phase 22 module boundary and re-exports.
- `crates/amlich-core/src/iching/mai_hoa.rs:59-131` — `MaiHoaCast` and deterministic cast function.
- `crates/amlich-core/src/iching/bien_que.rs:40-140` — `BienQue` and transformation function.
- `crates/amlich-core/src/iching/the_dung.rs:175-280` — compound Thể/Dụng classification.
- `crates/amlich-core/src/iching/schema.rs:283-324` — owned, locked `HexagramEntry` schema.
- `crates/amlich-core/src/iching/corpus.rs:26-82` — cached normalized corpus lookup.
- `crates/amlich-core/src/iching/golden.rs:55-169` — reusable golden input/result types and loader.
- `crates/amlich-core/src/reasoning/action_evaluator.rs:10-67` — generic evaluator result and trait shape.
- `crates/amlich-core/src/reasoning/personal.rs:20-38` — birth-required current personal path; reason not to force Tier-0 through it.
- `crates/amlich-core/src/reasoning/types.rs:3-8,133-153` — already-shipped `ActionId::IChing`, I Ching evidence family, and envelope fields.
- `crates/amlich-core/src/lib.rs:110-185,225-340` — date-only `SolarDate`, additive `DaySnapshot` fields, and ordinary snapshot construction.
- `crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs:16-46,476-747` — additive builder dispatch and FlyingStar/Ritual/Offering precedent.
- `crates/amlich-core/src/semantic_graph/graph.rs:19-27,46-70,94-145` — endpoint insertion rule, mutable post-population, provenance, and merge behavior.
- `crates/amlich-core/src/semantic_graph/node.rs:14-38` — node payload and additive serialization.
- `crates/amlich-core/src/semantic_graph/ontology.rs:3-44,91-125,165-236,371-448` — already-complete Hexagram/LocatedAt/Transforms ontology slices.
- `crates/amlich-core/src/semantic_graph/provenance.rs:6-16,101-116` — graph provenance source mapping; candidate I Ching family extension point.
- `crates/amlich-core/tests/day_snapshot_v14_compat.rs:193-280` — combined-strip backward-compatibility precedent.
- `crates/amlich-core/tests/semantic_graph_substrate.rs:8-268` — semantic ID, node, edge, merge, and provenance black-box contract style.
- `crates/amlich-core/tests/source_id_guard.rs:13-100` — bare source literal guard.
- `crates/amlich-core/tests/fengshui_crit3_isolation.rs:11-44` — protected interaction boundary.

### Phase 23 contract

- `.planning/adrs/0007-cross-link-crit3-carve-out.md:21-83` — required placement, read-only behavior, three-envelope contract, and explicit Phase 24 wiring responsibility.
- `.planning/ROADMAP.md:102-116` — Phase 23 is parallel and not yet planned/shipped.

### Official serialization/API guidance supplied by phase context

- https://serde.rs/attr-default.html
- https://serde.rs/attr-skip-serializing.html
- https://serde.rs/field-attrs.html
- https://rust-lang.github.io/api-guidelines/predictability.html
- https://rust-lang.github.io/api-guidelines/type-safety.html
- https://rust-lang.github.io/api-guidelines/future-proofing.html

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH — existing Cargo dependencies and shipped Phase 22 APIs are directly verified.
- Evaluator architecture: HIGH — sibling-newtype decision and `ActionEvaluator` trait are documented and code-backed; the date/hour adaptation is a checkout-specific correction.
- Evidence semantics: HIGH — source constants, IChing source family, and CRIT-6 requirement are already shipped/locked.
- DTO compatibility: HIGH — exact `Option<T>` attributes and combined-strip precedent are present in code.
- Semantic graph wiring: HIGH for Hexagram/edge mechanics; MEDIUM for the final directional node shape until Phase 23 ships.
- Phase 23 dependency: MEDIUM — placement and envelope contract are locked, concrete return type is not.

**Research date:** 2026-07-16
**Valid until:** 2026-08-15 for stable project architecture; revisit the Phase 23 API boundary when that phase lands.
