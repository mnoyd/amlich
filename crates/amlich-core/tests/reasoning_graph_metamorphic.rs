//! Metamorphic invariants for the initiation/opening reasoning path
//! (amlich-mwbp.8).
//!
//! These tests freeze the design-independent acceptance-criteria invariants
//! for the reasoning-graph migration as defined in
//! docs/architecture/personal-day-audit/REPAIR-PLAN.md ("freeze the failing
//! regression/metamorphic cases" before implementation). They were landed
//! RED and `#[ignore]`-gated under amlich-s5v1, then un-ignored as each
//! implementation phase turned them green (amlich-2q5n, amlich-zakn).
//!
//! Invariants locked here:
//!   1. Duplicate-evidence monotonicity — re-emitting an existing fact node
//!      must not change the initiation/opening bucket or axis scores.
//!   2. Action subgraph isolation — `select_subgraph` must drop concepts
//!      outside the initiation/opening allowlist (e.g. IChing hexagram).
//!   3. Strongest-notes-resolve-to-provenance — every axis carrying a
//!      non-zero score must point at a node id, and every strongest note
//!      must carry provenance.

use amlich_core::reasoning::{ActionEvaluator, InitiationOpeningEvaluator};
use amlich_core::{
    build_reasoning_input_graph, calculate_day_snapshot, NodeConcept, NodeOrigin, SemanticGraph,
    SemanticId, SemanticNode,
};

/// Build a day snapshot + merged reasoning input graph for the given date.
fn graph_for(day: i32, month: i32, year: i32) -> (amlich_core::DaySnapshot, SemanticGraph) {
    let snapshot = calculate_day_snapshot(day, month, year);
    let graph = build_reasoning_input_graph(&snapshot, None).expect("valid reasoning graph");
    (snapshot, graph)
}

/// Clone a node with a fresh id so it represents a duplicate emission of the
/// same underlying fact (same concept, severity, tags, provenance).
fn duplicate_node(node: &SemanticNode, suffix: &str) -> SemanticNode {
    let id = SemanticId::new(
        node.id.concept_label.clone(),
        format!("{}::dup::{}", node.id.stable_key, suffix),
    );
    SemanticNode {
        node_id: id.to_node_id(),
        id,
        concept: node.concept,
        origin: node.origin,
        summary_vi: node.summary_vi.clone(),
        severity: node.severity.clone(),
        tags: node.tags.clone(),
        provenance: node.provenance.clone(),
        payload: node.payload.clone(),
    }
}

#[test]
fn duplicate_evidence_does_not_inflate_decision() {
    let (snapshot, graph) = graph_for(3, 1, 2024);
    let evaluator = InitiationOpeningEvaluator::new();

    let before = evaluator
        .evaluate(&graph, &snapshot, None)
        .expect("baseline evaluation");

    // Pick a fact concept the evaluator currently counts (Taboo drives
    // resistance, override, and the Stability axis). 3/1/2024 is a
    // known taboo-bearing day.
    let taboo = graph
        .nodes()
        .values()
        .find(|n| n.concept == NodeConcept::Taboo)
        .expect("fixture day must expose a taboo node");

    let mut inflated = graph.clone();
    inflated.add_node(duplicate_node(taboo, "1"));

    let after = evaluator
        .evaluate(&inflated, &snapshot, None)
        .expect("evaluation after duplicate injection");

    assert_eq!(
        before.bucket, after.bucket,
        "duplicate evidence must not change the recommendation bucket"
    );
    assert_eq!(
        before.axis_scores.len(),
        after.axis_scores.len(),
        "axis set must be stable"
    );
    for (a, b) in before.axis_scores.iter().zip(after.axis_scores.iter()) {
        assert_eq!(a.axis, b.axis, "axis ordering/identity must be stable");
        assert_eq!(
            a.score, b.score,
            "duplicate evidence must not inflate the {:?} axis score",
            a.axis
        );
    }
}

#[test]
fn select_subgraph_drops_unrelated_concepts() {
    let (snapshot, graph) = graph_for(3, 1, 2024);

    // Inject an unrelated concept (IChing hexagram) that must never influence
    // an initiation/opening verdict.
    let hexagram = SemanticNode::new(
        SemanticId::new("hexagram", "iching:chu:1:3-1-2024:local"),
        NodeConcept::Hexagram,
        NodeOrigin::Fact,
        "Hexagram unmatched to initiation/opening",
    );
    let mut with_unrelated = graph.clone();
    with_unrelated.add_node(hexagram);

    let evaluator = InitiationOpeningEvaluator::new();
    let sub = evaluator
        .select_subgraph(&with_unrelated, &snapshot, None)
        .expect("subgraph selection");

    let leaked = sub
        .nodes()
        .values()
        .filter(|n| {
            matches!(
                n.concept,
                NodeConcept::Hexagram | NodeConcept::Ritual | NodeConcept::Offering
            )
        })
        .map(|n| n.node_id.clone())
        .collect::<Vec<_>>();

    assert!(
        leaked.is_empty(),
        "initiation/opening subgraph must exclude unrelated concepts; leaked: {leaked:?}"
    );
}

#[test]
fn strongest_notes_resolve_to_provenance() {
    let (snapshot, graph) = graph_for(3, 1, 2024);
    let evaluator = InitiationOpeningEvaluator::new();
    let evaluation = evaluator
        .evaluate(&graph, &snapshot, None)
        .expect("evaluation");

    let axes_without_provenance = evaluation
        .axis_scores
        .iter()
        .filter(|a| a.score > 0.0 && a.strongest_node_id.is_none())
        .map(|a| format!("{:?}", a.axis))
        .collect::<Vec<_>>();

    assert!(
        axes_without_provenance.is_empty(),
        "every axis with a non-zero score must resolve to a node id; offending axes: {axes_without_provenance:?}"
    );

    for note in evaluation
        .strongest_supports
        .iter()
        .chain(evaluation.strongest_resistances.iter())
        .chain(evaluation.override_factors.iter())
    {
        assert!(
            note.node_id.is_some(),
            "strongest note must carry a node id: {:?}",
            note.summary_vi
        );
        assert!(
            !note.provenance.is_empty(),
            "strongest note must carry provenance: {:?}",
            note.summary_vi
        );
    }
}

/// Edge provenance is carried on the exported graph's edges
/// (`ReasoningEdgeExport.evidence`), not on `ActionEvaluation`. This locks
/// the parity property that justifies retiring the dead
/// `ActionEvaluation::referenced_edge_ids` contract (always empty, never
/// read) — `amlich-0q2f` / `amlich-mwbp.8`.
#[test]
fn edge_provenance_lives_on_graph_export_not_evaluation() {
    use amlich_core::build_initiation_opening_reasoning_bundle;

    let (snapshot, _graph) = graph_for(3, 1, 2024);
    let bundle = build_initiation_opening_reasoning_bundle(&snapshot, None).expect("bundle");
    let export = &bundle.graph;

    // The export is the canonical provenance surface for edges: every edge
    // carries its own evidence envelope, so a parallel edge-id list on the
    // evaluation would be redundant.
    assert!(
        !export.edges.is_empty(),
        "fixture graph must expose at least one reasoning edge"
    );
    assert!(
        export
            .edges
            .iter()
            .all(|edge| !edge.evidence.is_empty() || !edge.tags.is_empty()),
        "edges must self-describe provenance via evidence/tags"
    );
}

/// Severity slots must never carry numeric counts. The hoang-dao-hours
/// favorability is signaled by a non-numeric presence marker, and personal
/// fact nodes carry no severity at all. Locks the "no overloaded numeric
/// severities" acceptance from `amlich-0q2f`.
#[test]
fn severity_slots_never_carry_numeric_counts() {
    use amlich_core::build_reasoning_input_graph;

    let (snapshot, _g) = graph_for(10, 2, 2024);
    let graph = build_reasoning_input_graph(&snapshot, None).expect("graph");

    for node in graph.nodes().values() {
        if let Some(sev) = node.severity.as_deref() {
            assert!(
                sev.parse::<usize>().is_err(),
                "severity slot must not carry a numeric value (concept={:?}, severity={sev:?}); \
                 counts belong in the summary/snapshot, not the severity field",
                node.concept
            );
        }
    }
}
