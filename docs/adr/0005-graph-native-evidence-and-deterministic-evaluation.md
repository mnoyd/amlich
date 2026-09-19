# Graph-Native Evidence with Deterministic Evaluation

Status: accepted

Amlich separates the knowledge graph used for source discovery from the reasoning graph used for assessment. GraphRAG may act as an adapter that extracts candidate entities, relationships, communities, and source spans from unstructured material, but it is not a dependency of the core evaluator and its unreviewed output must not affect assessment results. The core owns a canonical, typed graph interface and deterministic policy evaluation; it emits a complete `AssessmentTrace` containing contributions, axis aggregation, interactions, vetoes, precedence, and provenance. Desktop surfaces consume that trace through separate projections: Influence Graph, Contribution Matrix, and Evidence Path.

## Considered Options

- **Let GraphRAG directly drive assessment calculations** — rejected. LLM-extracted relationships are probabilistic and can change with model, prompt, or index version; they cannot silently become policy, weights, vetoes, or precedence.
- **Keep graph concerns only in the UI** — rejected. Recomputing relationships or contributions in each surface would duplicate policy and break cross-surface parity.
- **Use a vendor-specific graph schema throughout core** — rejected. Core must remain usable with canonical JSON, database, manual fixtures, or a future retrieval adapter even if GraphRAG is removed.
- **Canonical graph plus deterministic evaluator, with GraphRAG as an evidence adapter** — chosen. This preserves reproducibility while allowing richer source discovery and dynamic evidence exploration.

## Consequences

- Core graph nodes and edges need stable identifiers, typed relations, weights/conditions, provenance, policy version, and review state.
- Extracted knowledge passes through canonicalization and human review before it can be allowed into computation.
- `AssessmentTrace` is the contract between calculation and presentation; desktop and TUI must not recompute scores from graph data.
- GraphRAG indexing/query infrastructure can evolve independently from the Rust core and can be introduced incrementally.
- The primary explanation surface remains 2D and semantic; a future 3D observatory is an optional projection, not a replacement for precise graph, matrix, and evidence views.
