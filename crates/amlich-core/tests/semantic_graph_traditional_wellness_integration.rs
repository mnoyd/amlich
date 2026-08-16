//! Black-box integration tests for v1.10 `amlich-l2zc.3` (EXPLAIN-01)
//! semantic-graph wiring. These tests exercise the
//! `DaySnapshotGraphBuilder::add_traditional_wellness_facts` method
//! from the external crate path.
//!
//! ## Success criteria
//!
//! 1. After `enrich_day_snapshot_with_traditional_wellness`, the graph
//!    emits exactly one `NodeConcept::TraditionalChannel` node when the
//!    civil time resolves a branch, and one
//!    `NodeConcept::SeasonalProfile` node when the date resolves a
//!    solar term.
//! 2. Each `TraditionalChannel` node is wired to the day root via one
//!    `EdgeConcept::AssociatedWithHourBranch` edge.
//! 3. Each `SeasonalProfile` node is wired to the day root via one
//!    `EdgeConcept::JoinedByTermToSeason` edge.
//! 4. `TraditionalChannel` nodes carry provenance entries with
//!    `source_id == "shi-er-jing-na-di-zhi"`. `SeasonalProfile` nodes
//!    carry three provenance entries: the solar-term primitive
//!    (`amlich-solar-term-engine`), the Suwen primitive
//!    (`huangdi-neijing-suwen`), and the composite
//!    (`rule.composite.seasonal_wellness`).
//! 5. Ordinary `calculate_day_snapshot(...)` (no Traditional Wellness
//!    enrichment) produces ZERO `TraditionalChannel` and ZERO
//!    `SeasonalProfile` nodes — no implicit wiring.
//! 6. The `DaySnapshot.traditional_wellness` field is additive
//!    `Option<...>`, omitted from JSON when None, and byte-equal
//!    round-trips when populated.

use amlich_core::semantic_graph::build_day_snapshot_graph;
use amlich_core::semantic_graph::{EdgeConcept, NodeConcept};
use amlich_core::sources::{SOURCE_HUANGDI_NEIJING_SUWEN, SOURCE_SHI_ER_JING_NA_DI_ZHI};
use amlich_core::traditional_wellness::COMPOSITE_SEASONAL_WELLNESS;
use amlich_core::{calculate_day_snapshot, enrich_day_snapshot_with_traditional_wellness};

/// Convenience: a populated snapshot for testing.
fn sample_snapshot() -> amlich_core::DaySnapshot {
    calculate_day_snapshot(16, 8, 2026)
}

/// Convenience: enrich a snapshot with the unified Traditional Wellness
/// Context for (16 Aug 2026, +07, 09:30 = Tỵ).
fn enriched_snapshot() -> amlich_core::DaySnapshot {
    let snap = sample_snapshot();
    let jd = snap.context.jd;
    enrich_day_snapshot_with_traditional_wellness(&snap, jd, 7.0, 9, 30)
        .expect("unified enrichment must succeed")
}

// ───────────────────────────────────────────────────────────────────────
// 1. TraditionalChannel node — branch-channel primitive on the graph
// ───────────────────────────────────────────────────────────────────────

/// 1. Enriched snapshot produces EXACTLY one `NodeConcept::TraditionalChannel`
///    node when the civil time resolves a branch.
#[test]
fn enriched_graph_has_traditional_channel_node_when_branch_resolves() {
    let enriched = enriched_snapshot();
    let graph = build_day_snapshot_graph(&enriched);

    let channel_nodes: Vec<_> = graph
        .nodes()
        .values()
        .filter(|n| matches!(n.concept, NodeConcept::TraditionalChannel))
        .collect();
    assert_eq!(
        channel_nodes.len(),
        1,
        "Expected exactly 1 TraditionalChannel node; got {}",
        channel_nodes.len()
    );

    let node = channel_nodes[0];
    let source_ids: Vec<&str> = node
        .provenance
        .iter()
        .map(|p| p.source_id.as_str())
        .collect();
    assert!(
        source_ids.contains(&SOURCE_SHI_ER_JING_NA_DI_ZHI),
        "TraditionalChannel provenance must include {}; got {:?}",
        SOURCE_SHI_ER_JING_NA_DI_ZHI,
        source_ids
    );
    assert!(
        node.tags
            .iter()
            .any(|t| t == "safety_class=historical_cultural_non_clinical"),
        "TraditionalChannel node must carry the canonical safety_class tag; got {:?}",
        node.tags
    );
}

/// 2. The `TraditionalChannel` node is wired to the day root via one
///    `EdgeConcept::AssociatedWithHourBranch` edge.
#[test]
fn enriched_graph_has_associated_with_hour_branch_edge_to_day_root() {
    let enriched = enriched_snapshot();
    let graph = build_day_snapshot_graph(&enriched);

    let date_str = "2026-08-16";
    let day_root_id = format!("day:{}:+7", date_str);

    let edges: Vec<_> = graph
        .edges()
        .values()
        .filter(|e| matches!(e.label.concept, EdgeConcept::AssociatedWithHourBranch))
        .collect();
    assert_eq!(
        edges.len(),
        1,
        "Expected exactly 1 AssociatedWithHourBranch edge; got {}",
        edges.len()
    );

    let edge = edges[0];
    let from_node = graph
        .nodes()
        .get(&edge.from_node_id)
        .expect("AssociatedWithHourBranch from_node must exist");
    assert!(
        matches!(from_node.concept, NodeConcept::TraditionalChannel),
        "AssociatedWithHourBranch from_node must be a TraditionalChannel node; got {:?}",
        from_node.concept
    );
    assert_eq!(
        edge.to_node_id, day_root_id,
        "AssociatedWithHourBranch to_node_id must be the day root"
    );
}

// ───────────────────────────────────────────────────────────────────────
// 3. SeasonalProfile node — seasonal primitive on the graph
// ───────────────────────────────────────────────────────────────────────

/// 3. Enriched snapshot produces EXACTLY one `NodeConcept::SeasonalProfile`
///    node (one of four canonical profiles) and carries three provenance
///    entries: solar-term primitive + Suwen primitive + composite.
#[test]
fn enriched_graph_has_seasonal_profile_node_with_triple_provenance() {
    let enriched = enriched_snapshot();
    let graph = build_day_snapshot_graph(&enriched);

    let seasonal_nodes: Vec<_> = graph
        .nodes()
        .values()
        .filter(|n| matches!(n.concept, NodeConcept::SeasonalProfile))
        .collect();
    assert_eq!(
        seasonal_nodes.len(),
        1,
        "Expected exactly 1 SeasonalProfile node; got {}",
        seasonal_nodes.len()
    );

    let node = seasonal_nodes[0];
    assert_eq!(
        node.provenance.len(),
        3,
        "SeasonalProfile node must carry 3 provenance entries (solar-term + Suwen + composite); got {:?}",
        node.provenance
            .iter()
            .map(|p| &p.source_id)
            .collect::<Vec<_>>()
    );
    let source_ids: Vec<&str> = node
        .provenance
        .iter()
        .map(|p| p.source_id.as_str())
        .collect();
    assert!(
        source_ids.contains(&"amlich-solar-term-engine"),
        "SeasonalProfile provenance must include the solar-term engine attribution; got {:?}",
        source_ids
    );
    assert!(
        source_ids.contains(&SOURCE_HUANGDI_NEIJING_SUWEN),
        "SeasonalProfile provenance must include the Suwen primitive; got {:?}",
        source_ids
    );
    assert!(
        source_ids.contains(&COMPOSITE_SEASONAL_WELLNESS),
        "SeasonalProfile provenance must include the seasonal composite; got {:?}",
        source_ids
    );
    assert!(
        node.tags
            .iter()
            .any(|t| t == "safety_class=historical_cultural_non_clinical"),
        "SeasonalProfile node must carry the canonical safety_class tag; got {:?}",
        node.tags
    );
}

/// 4. The `SeasonalProfile` node is wired to the day root via one
///    `EdgeConcept::JoinedByTermToSeason` edge.
#[test]
fn enriched_graph_has_joined_by_term_to_season_edge_to_day_root() {
    let enriched = enriched_snapshot();
    let graph = build_day_snapshot_graph(&enriched);

    let date_str = "2026-08-16";
    let day_root_id = format!("day:{}:+7", date_str);

    let edges: Vec<_> = graph
        .edges()
        .values()
        .filter(|e| matches!(e.label.concept, EdgeConcept::JoinedByTermToSeason))
        .collect();
    assert_eq!(
        edges.len(),
        1,
        "Expected exactly 1 JoinedByTermToSeason edge; got {}",
        edges.len()
    );

    let edge = edges[0];
    let from_node = graph
        .nodes()
        .get(&edge.from_node_id)
        .expect("JoinedByTermToSeason from_node must exist");
    assert!(
        matches!(from_node.concept, NodeConcept::SeasonalProfile),
        "JoinedByTermToSeason from_node must be a SeasonalProfile node; got {:?}",
        from_node.concept
    );
    assert_eq!(
        edge.to_node_id, day_root_id,
        "JoinedByTermToSeason to_node_id must be the day root"
    );
}

// ───────────────────────────────────────────────────────────────────────
// 5. Implicit-wiring isolation
// ───────────────────────────────────────────────────────────────────────

/// 5. Ordinary `calculate_day_snapshot(...)` (no Traditional Wellness
///    enrichment) produces ZERO Traditional Wellness nodes — no
///    implicit wiring.
#[test]
fn ordinary_snapshot_has_no_traditional_wellness_nodes() {
    let snap = sample_snapshot();
    let graph = build_day_snapshot_graph(&snap);

    let traditional_count = graph
        .nodes()
        .values()
        .filter(|n| {
            matches!(
                n.concept,
                NodeConcept::TraditionalChannel | NodeConcept::SeasonalProfile
            )
        })
        .count();
    assert_eq!(
        traditional_count, 0,
        "Ordinary snapshots must emit ZERO Traditional Wellness nodes; got {traditional_count}"
    );

    let assoc_count = graph
        .edges()
        .values()
        .filter(|e| {
            matches!(
                e.label.concept,
                EdgeConcept::AssociatedWithHourBranch | EdgeConcept::JoinedByTermToSeason
            )
        })
        .count();
    assert_eq!(
        assoc_count, 0,
        "Ordinary snapshots must emit ZERO Traditional Wellness edges; got {assoc_count}"
    );
}

// ───────────────────────────────────────────────────────────────────────
// 6. Additive DTO discipline
// ───────────────────────────────────────────────────────────────────────

/// 6. `traditional_wellness` is absent in JSON when `None` (additive DTO
///    discipline) and byte-equal round-trips when populated.
#[test]
fn traditional_wellness_absent_in_ordinary_snapshot_json_and_round_trips_when_populated() {
    let snap = sample_snapshot();
    let json = serde_json::to_string(&snap).expect("serialise ordinary");
    assert!(
        !json.contains("\"traditional_wellness\""),
        "traditional_wellness must NOT appear in JSON when None; got {json}"
    );

    let enriched = enriched_snapshot();
    let json1 = serde_json::to_string(&enriched).expect("serialise enriched");
    let parsed: amlich_core::DaySnapshot =
        serde_json::from_str(&json1).expect("deserialise enriched");
    let json2 = serde_json::to_string(&parsed).expect("re-serialise");
    assert_eq!(
        json1, json2,
        "traditional_wellness enriched snapshot must round-trip byte-equal"
    );
    assert!(
        json1.contains("\"traditional_wellness\""),
        "traditional_wellness must appear in JSON when Some; got {json1}"
    );
    assert!(
        parsed.traditional_wellness.is_some(),
        "Round-tripped snapshot must retain traditional_wellness"
    );
}

/// 7. The two new concept labels round-trip through the JSON
///    serialization (schema lock for the public graph surface).
#[test]
fn traditional_wellness_concept_labels_round_trip() {
    let enriched = enriched_snapshot();
    let graph = build_day_snapshot_graph(&enriched);
    let json = serde_json::to_string(&graph).expect("serialise graph");
    assert!(
        json.contains("traditional_channel"),
        "graph JSON must carry the snake_case concept label; got {json}"
    );
    assert!(
        json.contains("seasonal_profile"),
        "graph JSON must carry the snake_case concept label; got {json}"
    );
    assert!(
        json.contains("associated_with_hour_branch"),
        "graph JSON must carry the snake_case edge label; got {json}"
    );
    assert!(
        json.contains("joined_by_term_to_season"),
        "graph JSON must carry the snake_case edge label; got {json}"
    );
}
