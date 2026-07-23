//! Black-box integration tests for Phase 24-02 IChing semantic-graph wiring
//! (INT-11). These tests exercise the `DaySnapshotGraphBuilder::add_iching_facts`
//! + `add_direction_composite_facts` methods from the external crate path.
//!
//! ## Success criteria
//!
//! 1. Two distinct `NodeConcept::Hexagram` nodes (primary chu + bien) wired
//!    via `EdgeConcept::Transforms` + two `EdgeConcept::LocatedAt` edges.
//! 2. Dual-source provenance (CRIT-6) on each Hexagram node:
//!    SOURCE_MAI_HOA_DICH_SO + SOURCE_KINH_DICH.
//! 3. Role-bearing stable keys ("iching:chu:<kw>:<date>:<tz>" +
//!    "iching:bien:<kw>:<date>:<tz>") so the same primary/bien pair cannot
//!    collide in `graph.node_count()`.
//! 4. No implicit wiring on plain `calculate_day_snapshot(...)` snapshots.
//! 5. CRIT-3 isolation preserved: `add_iching_facts` does NOT reference
//!    `FlyingStar` (the existing `add_flying_star_facts` is the only
//!    `FlyingStar` consumer in this file).
//! 6. `add_direction_composite_facts` early-returns when
//!    `snapshot.direction_cross_link` is `None`; the IChing-only enrichment
//!    path produces ZERO directional composite nodes.
//! 7. The `direction_cross_link` field on `DaySnapshot` is additive
//!    `Option<DirectionCrossLinkSummary>`, omitted from JSON when None, and
//!    byte-equal round-trips when populated.

use std::fs;
use std::path::Path;

use amlich_core::iching::IChingQuery;
use amlich_core::semantic_graph::build_day_snapshot_graph;
use amlich_core::semantic_graph::{EdgeConcept, NodeConcept, SemanticGraph};
use amlich_core::sources::{SOURCE_KINH_DICH, SOURCE_MAI_HOA_DICH_SO};
use amlich_core::{calculate_day_snapshot, enrich_day_snapshot_with_iching, DaySnapshot};

/// Convenience: a populated snapshot for testing.
fn sample_snapshot() -> DaySnapshot {
    calculate_day_snapshot(10, 2, 2024)
}

/// Convenience: enrich a snapshot with an explicit IChing query (Tier-0,
/// no birth data).
fn enriched_snapshot() -> DaySnapshot {
    let snap = sample_snapshot();
    let query =
        IChingQuery::from_snapshot(&snap, Some("việc".to_string()), 9).expect("valid query");
    enrich_day_snapshot_with_iching(&snap, query).expect("enrichment succeeds")
}

// ───────────────────────────────────────────────────────────────────────
// IChing portion — add_iching_facts (tests 1-9)
// ───────────────────────────────────────────────────────────────────────

/// 1. Enriched snapshot produces EXACTLY 2 distinct `NodeConcept::Hexagram`
///    nodes (primary chu + bien) and their node_ids differ.
#[test]
fn iching_graph_has_two_distinct_hexagram_nodes_when_enriched() {
    let enriched = enriched_snapshot();
    let graph = build_day_snapshot_graph(&enriched);

    let hex_nodes: Vec<_> = graph
        .nodes()
        .values()
        .filter(|n| matches!(n.concept, NodeConcept::Hexagram))
        .collect();
    assert_eq!(
        hex_nodes.len(),
        2,
        "Expected exactly 2 Hexagram nodes (chu + bien); got {}",
        hex_nodes.len()
    );

    let mut ids: Vec<&String> = hex_nodes.iter().map(|n| &n.node_id).collect();
    ids.sort();
    ids.dedup();
    assert_eq!(
        ids.len(),
        2,
        "Primary + bien Hexagram node ids must be distinct; got duplicates"
    );
}

/// 2. The graph contains EXACTLY 1 `EdgeConcept::Transforms` edge whose
///    `from_node_id` is the primary Hexagram node AND `to_node_id` is the
///    bien Hexagram node.
#[test]
fn iching_graph_has_transforms_edge_between_chu_and_bien() {
    let enriched = enriched_snapshot();
    let graph = build_day_snapshot_graph(&enriched);

    let hex_ids: Vec<String> = graph
        .nodes()
        .values()
        .filter(|n| matches!(n.concept, NodeConcept::Hexagram))
        .map(|n| n.node_id.clone())
        .collect();
    assert_eq!(hex_ids.len(), 2);

    let transforms_edges: Vec<_> = graph
        .edges()
        .values()
        .filter(|e| matches!(e.label.concept, EdgeConcept::Transforms))
        .collect();
    assert_eq!(
        transforms_edges.len(),
        1,
        "Expected exactly 1 Transforms edge; got {}",
        transforms_edges.len()
    );

    let e = transforms_edges[0];
    assert!(
        hex_ids.contains(&e.from_node_id),
        "Transforms from_node_id must be one of the Hexagram nodes; got {}",
        e.from_node_id
    );
    assert!(
        hex_ids.contains(&e.to_node_id),
        "Transforms to_node_id must be one of the Hexagram nodes; got {}",
        e.to_node_id
    );
    assert_ne!(
        e.from_node_id, e.to_node_id,
        "Transforms edge must connect two distinct Hexagram nodes"
    );
}

/// 3. The graph contains EXACTLY 2 `EdgeConcept::LocatedAt` edges — one
///    from each Hexagram node to the day root.
#[test]
fn iching_graph_has_located_at_edges_from_each_hexagram_to_day_root() {
    let enriched = enriched_snapshot();
    let graph = build_day_snapshot_graph(&enriched);

    let located_at_edges: Vec<_> = graph
        .edges()
        .values()
        .filter(|e| matches!(e.label.concept, EdgeConcept::LocatedAt))
        .collect();
    assert_eq!(
        located_at_edges.len(),
        2,
        "Expected exactly 2 LocatedAt edges (one per Hexagram); got {}",
        located_at_edges.len()
    );

    // Every LocatedAt edge must originate at a Hexagram node and terminate
    // at the day root (day:YYYY-MM-DD:+7).
    let date_str = "2024-02-10";
    let day_root_id = format!("day:{}:+7", date_str);
    for edge in &located_at_edges {
        let from_node = graph
            .nodes()
            .get(&edge.from_node_id)
            .expect("LocatedAt from_node must exist");
        assert!(
            matches!(from_node.concept, NodeConcept::Hexagram),
            "LocatedAt from_node must be a Hexagram node; got {:?}",
            from_node.concept
        );
        assert_eq!(
            edge.to_node_id, day_root_id,
            "LocatedAt to_node_id must be the day root"
        );
    }
}

/// 4. Each Hexagram node carries EXACTLY 2 provenance entries — one with
///    `source_id == SOURCE_MAI_HOA_DICH_SO` AND one with `source_id ==
/// SOURCE_KINH_DICH` (CRIT-6 dual-source pattern).
#[test]
fn iching_graph_hexagram_nodes_carry_dual_source_provenance() {
    let enriched = enriched_snapshot();
    let graph = build_day_snapshot_graph(&enriched);

    let hex_nodes: Vec<_> = graph
        .nodes()
        .values()
        .filter(|n| matches!(n.concept, NodeConcept::Hexagram))
        .collect();
    assert_eq!(hex_nodes.len(), 2);

    for node in &hex_nodes {
        assert_eq!(
            node.provenance.len(),
            2,
            "Hexagram node {} must carry exactly 2 provenance entries (CRIT-6); got {:?}",
            node.node_id,
            node.provenance
        );
        let source_ids: Vec<&str> = node
            .provenance
            .iter()
            .map(|p| p.source_id.as_str())
            .collect();
        assert!(
            source_ids.contains(&SOURCE_MAI_HOA_DICH_SO),
            "Hexagram node {} provenance must include SOURCE_MAI_HOA_DICH_SO; got {:?}",
            node.node_id,
            source_ids
        );
        assert!(
            source_ids.contains(&SOURCE_KINH_DICH),
            "Hexagram node {} provenance must include SOURCE_KINH_DICH; got {:?}",
            node.node_id,
            source_ids
        );
    }
}

/// 5. The two Hexagram node ids match `"iching:chu:<kw>:YYYY-MM-DD:+7"`
///    and `"iching:bien:<kw>:YYYY-MM-DD:+7"` patterns respectively.
#[test]
fn iching_graph_hexagram_stable_keys_are_role_bearing() {
    let enriched = enriched_snapshot();
    let graph = build_day_snapshot_graph(&enriched);

    let hex_nodes: Vec<_> = graph
        .nodes()
        .values()
        .filter(|n| matches!(n.concept, NodeConcept::Hexagram))
        .collect();
    assert_eq!(hex_nodes.len(), 2);

    let ids: Vec<&String> = hex_nodes.iter().map(|n| &n.node_id).collect();
    let chu_id = ids
        .iter()
        .find(|id| id.starts_with("hexagram:iching:chu:"))
        .expect("primary chu Hexagram node id must match 'iching:chu:<kw>:...'");
    let bien_id = ids
        .iter()
        .find(|id| id.starts_with("hexagram:iching:bien:"))
        .expect("bien Hexagram node id must match 'iching:bien:<kw>:...'");

    assert!(
        chu_id.contains(":2024-02-10:+7"),
        "chu Hexagram node id must contain the date + tz suffix; got {chu_id}"
    );
    assert!(
        bien_id.contains(":2024-02-10:+7"),
        "bien Hexagram node id must contain the date + tz suffix; got {bien_id}"
    );
}

/// 6. An ordinary `calculate_day_snapshot(...)` (no IChing enrichment)
///    produces a graph with ZERO `NodeConcept::Hexagram` nodes — no implicit
///    wiring.
#[test]
fn iching_graph_ordinary_snapshot_has_no_hexagram_nodes() {
    let snap = sample_snapshot();
    let graph = build_day_snapshot_graph(&snap);

    let hex_count = graph
        .nodes()
        .values()
        .filter(|n| matches!(n.concept, NodeConcept::Hexagram))
        .count();
    assert_eq!(
        hex_count, 0,
        "Ordinary snapshots (iching_cast=None) must emit ZERO Hexagram nodes; got {hex_count}"
    );
}

/// 7. CRIT-3 grep guard — `add_iching_facts` MUST NOT reference
///    `FlyingStar`. The substring `"FlyingStar"` in day_snapshot.rs is
///    expected to occur ONLY inside `add_flying_star_facts`.
#[test]
fn iching_graph_no_flyingstar_in_iching_method() {
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("src/semantic_graph/builders/day_snapshot.rs");
    let src = fs::read_to_string(&path).expect("read day_snapshot.rs");

    // Find the byte ranges of the two methods.
    let ichting_facts_start = src
        .find("fn add_iching_facts")
        .expect("add_iching_facts method must exist");
    let iching_facts_next_method = src[ichting_facts_start..]
        .find("\n    fn add_direction_composite_facts")
        .map(|o| ichting_facts_start + o)
        .expect("add_direction_composite_facts method must exist");

    let iching_method_body = &src[ichting_facts_start..iching_facts_next_method];
    assert!(
        !iching_method_body.contains("FlyingStar"),
        "CRIT-3 isolation violation: `FlyingStar` substring found inside \
         add_iching_facts (or its preceding stub). The IChing method must \
         not reference FlyingStar; the cross-link surface is opt-in via \
         add_direction_composite_facts."
    );
}

/// 8. Both Hexagram nodes' tags include the King Wen index
///    (`"king_wen=<N>"`) AND the role marker (`"role=chu"` / `"role=bien"`).
#[test]
fn iching_graph_hexagram_node_tags_include_king_wen_and_role() {
    let enriched = enriched_snapshot();
    let graph = build_day_snapshot_graph(&enriched);

    let hex_nodes: Vec<_> = graph
        .nodes()
        .values()
        .filter(|n| matches!(n.concept, NodeConcept::Hexagram))
        .collect();
    assert_eq!(hex_nodes.len(), 2);

    let chu = hex_nodes
        .iter()
        .find(|n| n.node_id.starts_with("hexagram:iching:chu:"))
        .expect("chu Hexagram node must exist");
    let bien = hex_nodes
        .iter()
        .find(|n| n.node_id.starts_with("hexagram:iching:bien:"))
        .expect("bien Hexagram node must exist");

    assert!(
        chu.tags.iter().any(|t| t.starts_with("king_wen=")),
        "chu Hexagram tags must include king_wen=<N>; got {:?}",
        chu.tags
    );
    assert!(
        chu.tags.iter().any(|t| t == "role=chu"),
        "chu Hexagram tags must include role=chu; got {:?}",
        chu.tags
    );
    assert!(
        bien.tags.iter().any(|t| t.starts_with("king_wen=")),
        "bien Hexagram tags must include king_wen=<N>; got {:?}",
        bien.tags
    );
    assert!(
        bien.tags.iter().any(|t| t == "role=bien"),
        "bien Hexagram tags must include role=bien; got {:?}",
        bien.tags
    );
}

/// 9. The Transforms + LocatedAt edges are present after enrichment (not
///    silently dropped by `SemanticGraph::add_edge` because both endpoints
///    exist). Asserted via `graph.edges().len()` counts: enriched snapshots
///    must have strictly more edges than ordinary snapshots, and the new
///    edges must be in the Transforms / LocatedAt sets.
#[test]
fn iching_graph_edges_only_present_after_nodes_inserted() {
    let ordinary = sample_snapshot();
    let enriched = enriched_snapshot();
    let g_ord = build_day_snapshot_graph(&ordinary);
    let g_enr = build_day_snapshot_graph(&enriched);

    let ord_edges = g_ord.edges().len();
    let enr_edges = g_enr.edges().len();
    assert!(
        enr_edges > ord_edges,
        "Enriched graph must have more edges than ordinary (Transforms + 2 LocatedAt); \
         ordinary={ord_edges}, enriched={enr_edges}"
    );

    // Specifically: at least 1 Transforms + 2 LocatedAt added.
    let transforms_count = g_enr
        .edges()
        .values()
        .filter(|e| matches!(e.label.concept, EdgeConcept::Transforms))
        .count();
    let located_at_count = g_enr
        .edges()
        .values()
        .filter(|e| matches!(e.label.concept, EdgeConcept::LocatedAt))
        .count();
    assert!(
        transforms_count >= 1,
        "Enriched graph must have >=1 Transforms edge; got {transforms_count}"
    );
    assert!(
        located_at_count >= 2,
        "Enriched graph must have >=2 LocatedAt edges; got {located_at_count}"
    );

    // The ordinary graph must NOT have any Transforms edges (sanity check
    // — Hexagram nodes don't exist, so Transforms is not emitted).
    let ord_transforms = g_ord
        .edges()
        .values()
        .filter(|e| matches!(e.label.concept, EdgeConcept::Transforms))
        .count();
    assert_eq!(
        ord_transforms, 0,
        "Ordinary snapshots must have ZERO Transforms edges; got {ord_transforms}"
    );
}

// ───────────────────────────────────────────────────────────────────────
// Directional composite portion — add_direction_composite_facts (tests 10-13)
// ───────────────────────────────────────────────────────────────────────

/// 10. IChing-only enrichment does NOT wire a directional composite node
///     from `add_direction_composite_facts`. The `enrich_day_snapshot_with_iching`
///     helper does NOT populate `direction_cross_link`. The graph contains
///     ZERO `cross_link:` stable-key nodes (the existing daily travel direction
///     node is allowed — that node is NOT from this method).
#[test]
fn iching_only_enrichment_does_not_wire_directional_composite() {
    let enriched = enriched_snapshot();
    let graph = build_day_snapshot_graph(&enriched);

    // count Direction nodes whose stable key matches "cross_link:"
    let cross_link_nodes = graph
        .nodes()
        .values()
        .filter(|n| matches!(n.concept, NodeConcept::Direction))
        .filter(|n| n.node_id.contains("cross_link:"))
        .count();
    assert_eq!(
        cross_link_nodes, 0,
        "IChing-only enrichment must NOT wire a cross_link Direction node; got {cross_link_nodes}"
    );
}

/// 11. The populated directional composite test exercises the
///     `add_direction_composite_facts` method when `direction_cross_link` is
///     `Some(...)`. Phase 23 has shipped `DirectionCrossLinkSummary` + the
///     `enrich_day_snapshot_with_direction_cross_link` helper. The test is
///     active here (Phase 23 fully shipped) — verifies the composite wiring
///     end-to-end.
#[test]
fn direction_composite_facts_wires_populated_state() {
    let snap = sample_snapshot();
    // Build the personal cross-link for branch 0 (Tý).
    let enriched = amlich_core::enrich_day_snapshot_with_direction_cross_link(&snap, 0)
        .expect("enrich_day_snapshot_with_direction_cross_link succeeds");
    assert!(enriched.direction_cross_link.is_some());
    let graph = build_day_snapshot_graph(&enriched);

    let cross_link_node = graph
        .nodes()
        .values()
        .find(|n| matches!(n.concept, NodeConcept::Direction) && n.node_id.contains("cross_link:"))
        .expect("Directional composite node must exist after direction_cross_link enrichment");

    // The composite node must carry 3 provenance entries: KHCBPPT + Huyền-Không
    // primitives PLUS ONE composite envelope (Phase 23's locked contract).
    let source_ids: Vec<&str> = cross_link_node
        .provenance
        .iter()
        .map(|p| p.source_id.as_str())
        .collect();
    assert!(
        source_ids.contains(&"khcbppt"),
        "Cross-link Direction node provenance must include khcbppt; got {:?}",
        source_ids
    );
    assert!(
        source_ids.contains(&"huyen-khong"),
        "Cross-link Direction node provenance must include huyen-khong; got {:?}",
        source_ids
    );
    assert!(
        source_ids.iter().any(|s| s.starts_with("rule.composite.")),
        "Cross-link Direction node provenance must include the Phase 23 composite envelope; got {:?}",
        source_ids
    );

    // The cross-link node must be wired to the day root via a LocatedAt edge.
    let cross_link_id = cross_link_node.node_id.clone();
    let located_at_to_day_root = graph
        .edges()
        .values()
        .find(|e| {
            matches!(e.label.concept, EdgeConcept::LocatedAt)
                && e.from_node_id == cross_link_id
                && e.to_node_id.contains("day:2024-02-10")
        })
        .expect("Cross-link Direction node must have a LocatedAt edge to the day root");

    // Ensure the cross-link Direction node is distinct from the daily
    // travel direction node (which uses the stable key
    // "direction:travel:day:+7:all").
    assert!(
        !cross_link_id.contains("travel:day:"),
        "Cross-link Direction node id must NOT collide with the daily travel direction node; got {cross_link_id}"
    );

    // Touch the LocatedAt edge so the unused-binding warning is satisfied.
    let _ = located_at_to_day_root;
}

/// 12. `direction_cross_link` is absent in JSON when `None` (additive DTO
///     discipline).
#[test]
fn direction_cross_link_absent_in_ordinary_snapshot_json() {
    let snap = sample_snapshot();
    let json = serde_json::to_string(&snap).expect("serialise");
    assert!(
        !json.contains("\"direction_cross_link\""),
        "direction_cross_link must NOT appear in JSON when None; got {json}"
    );
}

/// 13. `direction_cross_link` byte-equal round-trips when populated. The
///     summary is serde-compatible end-to-end.
#[test]
fn direction_cross_link_round_trip_when_populated() {
    let snap = sample_snapshot();
    let enriched = amlich_core::enrich_day_snapshot_with_direction_cross_link(&snap, 0)
        .expect("enrich_day_snapshot_with_direction_cross_link succeeds");

    let json1 = serde_json::to_string(&enriched).expect("serialise");
    let parsed: DaySnapshot = serde_json::from_str(&json1).expect("deserialise");
    let json2 = serde_json::to_string(&parsed).expect("re-serialise");

    assert_eq!(
        json1, json2,
        "direction_cross_link enriched snapshot must round-trip byte-equal"
    );
    assert!(
        json1.contains("\"direction_cross_link\""),
        "direction_cross_link must appear in JSON when Some; got {json1}"
    );
    assert!(
        parsed.direction_cross_link.is_some(),
        "Round-tripped snapshot must retain direction_cross_link"
    );

    // Touch SemanticGraph type to silence unused-import warning.
    let _g: SemanticGraph = SemanticGraph::new();
}
