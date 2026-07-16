# Phase 24: IChing Evaluator + Semantic-Graph Wiring + DTO Integration - Context

**Gathered:** 2026-07-16
**Status:** Ready for planning

<domain>
## Phase Boundary

Add the Tier-0 I Ching integration surface: an explicit `IChingQuery`/`IChingEvaluator` path over the already-shipped Mai Hoa casting, biến quẻ, Thể/Dụng, and corpus APIs; semantic-graph facts for the chủ quẻ, biến quẻ, and the Phase 23 directional cross-link; and additive `DaySnapshot` summaries with v1.6-to-v1.7 backward-compatible serialization.

This phase does not add coin/yarrow/RNG casting, alternative casting variants, LLM-generated interpretation, Bazi enrichment, visual/TUI graph presentation, or any `FlyingStar` wiring into `interaction/direction_merge.rs`. The Phase 23 directional cross-link implementation is a prerequisite/input, not new scope for this phase.

</domain>

<decisions>
## Implementation Decisions

### DaySnapshot population and compatibility
- Ordinary `calculate_day_snapshot(...)` calls must not invent an I Ching hour or silently auto-cast. `iching_cast` remains `None` unless the caller explicitly requests I Ching enrichment.
- An explicit I Ching operation returns an enriched `DaySnapshot` value; it must not mutate the caller's original snapshot.
- `direction_cross_link` is populated only through an explicit enrichment request that supplies the required personal/birth-chi input. An I Ching-only enrichment must not invent personal context or populate the directional cross-link.
- New optional fields are omitted from JSON when absent and serialized normally when present. Missing fields in legacy v1.6-shaped JSON deserialize to `None`; re-serialization must not introduce new `null` keys.

### Claude's Discretion
- **Query/evaluator contract:** Choose the exact public shape for `IChingQuery`, the evaluator result, and any convenience enrichment constructor while preserving an explicit-call boundary, Tier-0 operation without birth data, and the already-locked sibling-newtype decision. Prefer clear domain types and a compound result struct over ambiguous primitive/boolean parameters.
- **Evidence semantics:** Preserve distinct primitive provenance for casting (`mai-hoa-dich-so`) and corpus text lookup (`kinh-dich`), plus exactly one composite envelope for the combined consultation. The planner may define the precise per-step method names, notes, and ordering by following the existing `ReasoningEvidenceEnvelope` and provenance patterns; do not collapse the primitive sources into the composite.
- **Semantic graph shape:** Follow existing `SemanticId`, `DaySnapshotGraphBuilder::add_*_facts`, `SemanticNode`, `SemanticEdge`, and `SemanticGraph::merge` patterns. The planner may choose the stable-key details and exact anchoring of the chủ/biến Hexagram nodes and Phase 23 composite fact, provided the graph exposes both Hexagram nodes, `LocatedAt`/`Transforms` edges, and the cross-link without changing ontology or violating CRIT-3 isolation.
- **Error and validation details:** Use existing crate conventions and choose appropriate validation for explicit query input, missing Phase 23 data, and impossible/invariant-breaking states. Do not add external dependencies or invent a new error framework.

</decisions>

<code_context>
## Existing Code Insights

### Reusable Assets
- `crates/amlich-core/src/iching/mai_hoa.rs`: `MaiHoaCast` and deterministic `cast_mai_hoa(...)` over lunar year-branch, lunar month, lunar day, and chi-hour index.
- `crates/amlich-core/src/iching/bien_que.rs`: `BienQue` and `derive_bien_que(...)`, including the 384-case contract discipline.
- `crates/amlich-core/src/iching/the_dung.rs`: `classify_the_dung(...)`, `TheDungClassification`, and the Ngũ Hành relationship/verdict surface.
- `crates/amlich-core/src/iching/corpus.rs`: lazy `OnceLock` lookup via `all_hexagrams()` / `get_hexagram(...)` with NFC normalization and schema validation.
- `crates/amlich-core/src/reasoning/types.rs`: serializable `ReasoningEvidenceEnvelope` with `source_family`, `source_id`, `method`, and optional `note`; `ReasoningEvidenceSourceFamily::IChing` already exists.
- `crates/amlich-core/src/semantic_graph/`: `SemanticGraph`, `SemanticNode`, `SemanticEdge`, `SemanticId`, `ProvenanceEntry`, and `SemanticGraph::merge` provide the established graph construction and provenance primitives.

### Established Patterns
- `DaySnapshot` uses additive `Option<T>` fields with `#[serde(default, skip_serializing_if = "Option::is_none")]`; existing compatibility tests strip newly added fields together and assert deserialize plus byte-equal re-serialization.
- `DaySnapshotGraphBuilder::new` invokes one `add_*_facts` method per optional surface and skips absent data. New Phase 24 builder methods should follow this additive pattern.
- Semantic graph IDs are deterministic `concept_label:stable_key` strings built through `SemanticId`; edges are only inserted when both endpoint nodes exist.
- Provenance is append-oriented. `ProvenanceEntry::to_reasoning_evidence()` converts graph provenance into reasoning envelopes; source literals should use registered `SOURCE_*` constants at call sites.
- Reasoning evaluators currently implement `ActionEvaluator` around `SemanticGraph`, `DaySnapshot`, and optional personal input, but the I Ching requirement explicitly permits Tier-0 operation without birth data. The planner should avoid forcing I Ching into personal-only assumptions.
- Core public APIs use pure/value-oriented returns and descriptive `Result<T, String>` validation where input can fail; no async or external service layer exists.

### Integration Points
- `crates/amlich-core/src/lib.rs`: `DaySnapshot` definition and `calculate_day_snapshot_internal` construction/population path. Explicit enrichment should preserve ordinary snapshot behavior.
- `crates/amlich-core/src/reasoning/mod.rs`: public reasoning exports; any new evaluator/query types need deliberate re-exports.
- `crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs`: constructor wiring for additive `add_iching_facts()` and `add_direction_composite_facts()` methods.
- `crates/amlich-core/src/semantic_graph/ontology.rs`: `NodeConcept::Hexagram`, `EdgeConcept::LocatedAt`, and `EdgeConcept::Transforms` already exist; do not add duplicate ontology concepts.
- `crates/amlich-core/src/semantic_graph/provenance.rs` and `sources.rs`: provenance conversion and registered source-ID constants.
- `crates/amlich-core/tests/day_snapshot_v14_compat.rs`: precedent for the combined-strip v1.6-to-v1.7 compatibility test.
- `crates/amlich-core/tests/semantic_graph_substrate.rs`, `reasoning_graph_contract.rs`, and related reasoning graph tests: existing black-box contract style for graph nodes, edges, provenance, and public APIs.
- Phase 23's `build_direction_cross_link(snapshot, birth_chi_index)` output is an upstream dependency for the directional summary and graph builder; Phase 24 planning must state this dependency explicitly rather than recreating the cross-link logic.

</code_context>

<specifics>
## Specific Ideas

- The enriched-snapshot behavior should feel like the existing pure calculation APIs: take an existing snapshot, return a new value with explicit additive data, and leave the source value unchanged.
- The compatibility contract is absence-preserving: old producers that never emitted the new keys should continue to round-trip without gaining `null` fields.
- Internet research used for the compatibility/API guidance:
  - Serde field defaults: https://serde.rs/attr-default.html
  - Serde conditional omission: https://serde.rs/attr-skip-serializing.html
  - Serde field attribute reference: https://serde.rs/field-attrs.html
  - Rust API Guidelines (specific receiver methods, compound returns, constructors): https://rust-lang.github.io/api-guidelines/predictability.html
  - Rust API Guidelines (newtypes and explicit domain types): https://rust-lang.github.io/api-guidelines/type-safety.html
  - Rust API Guidelines (future-proofing public structs/newtypes): https://rust-lang.github.io/api-guidelines/future-proofing.html

</specifics>

<deferred>
## Deferred Ideas

- Visual semantic-graph/TUI workspace presentation — separate UI/product phase.
- Coin, yarrow, RNG, sound-number, or user-selectable casting variants — outside the v1.7 Mai Hoa time-numerology boundary.
- Free-form LLM interpretation, Hỗ Quả/nuclear hexagram, and Tier-2 Bazi enrichment — later phases/milestones.
- Spatial Feng Shui composition involving `FlyingStar` in `interaction/direction_merge.rs` — explicitly prohibited by the phase boundary.

</deferred>

---

*Phase: 24-iching-evaluator-semantic-graph-wiring-dto-integration*
*Context gathered: 2026-07-16 via discuss-phase*
