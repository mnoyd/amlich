use amlich_core::semantic_graph::{
    ConceptLabel, EdgeConcept, GraphMergeError, GraphOntology, LlmGraphSlice, NodeConcept,
    NodeOrigin, ProvenanceEntry, ProvenanceSource, SemanticEdge, SemanticEdgeId, SemanticEdgeLabel,
    SemanticGraph, SemanticId, SemanticNode, SemanticNodeId, SubgraphView, VisualizationGraph,
};

#[test]
fn test_semantic_id_day_root() {
    let id = SemanticId::day_root("2024-05-13", "tz+7");
    assert_eq!(id.to_node_id(), "day:2024-05-13:tz+7");
}

#[test]
fn test_semantic_id_day_child_fact() {
    let id = SemanticId::solar_term_day("2024-05-13", "tz+7");
    assert_eq!(id.to_node_id(), "solar_term:day:2024-05-13:tz+7");
}

#[test]
fn test_semantic_id_bazi_profile() {
    let id = SemanticId::bazi_profile("1990-01-01T09:30", "tz+7");
    assert_eq!(id.to_node_id(), "bazi_profile:1990-01-01T09:30:tz+7");
}

#[test]
fn test_semantic_id_pillar_bazi() {
    let id = SemanticId::pillar_bazi("year", "1990-01-01T09:30", "tz+7");
    assert_eq!(
        id.to_node_id(),
        "pillar:bazi_profile:1990-01-01T09:30:tz+7:year"
    );
}

#[test]
fn test_semantic_id_stability() {
    let id1 = SemanticId::day_root("2024-05-13", "tz+7");
    let id2 = SemanticId::day_root("2024-05-13", "tz+7");
    assert_eq!(id1.to_node_id(), id2.to_node_id());
    assert_eq!(id1, id2);
}

#[test]
fn test_semantic_id_taboo_with_name() {
    let id = SemanticId::taboo_day("2024-05-13", "tz+7", "tam_nuong");
    assert_eq!(id.to_node_id(), "taboo:day:2024-05-13:tz+7:tam_nuong");
}

#[test]
fn test_semantic_node_new() {
    let id = SemanticId::day_root("2024-05-13", "tz+7");
    let node = SemanticNode::new(
        id,
        NodeConcept::DayCanchi,
        NodeOrigin::Fact,
        "Ngày 2024-05-13",
    );
    assert_eq!(node.node_id, "day:2024-05-13:tz+7");
    assert_eq!(node.concept, NodeConcept::DayCanchi);
    assert_eq!(node.origin, NodeOrigin::Fact);
    assert_eq!(node.summary_vi, "Ngày 2024-05-13");
}

#[test]
fn test_semantic_node_with_provenance() {
    let id = SemanticId::day_root("2024-05-13", "tz+7");
    let prov = ProvenanceEntry::snapshot("day:2024-05-13:tz+7", "amlich-core");
    let node = SemanticNode::new(
        id,
        NodeConcept::DayCanchi,
        NodeOrigin::Fact,
        "Ngày 2024-05-13",
    )
    .with_provenance(prov.clone());
    assert_eq!(node.provenance.len(), 1);
    assert_eq!(node.provenance[0].source, ProvenanceSource::Snapshot);
}

#[test]
fn test_semantic_edge_new() {
    let edge = SemanticEdge::new(
        "day:2024-05-13:tz+7",
        "truc:day:2024-05-13:tz+7",
        EdgeConcept::Composes,
    );
    assert_eq!(edge.from_node_id, "day:2024-05-13:tz+7");
    assert_eq!(edge.to_node_id, "truc:day:2024-05-13:tz+7");
    assert_eq!(edge.label.concept, EdgeConcept::Composes);
    assert_eq!(
        edge.edge_id,
        "day:2024-05-13:tz+7->truc:day:2024-05-13:tz+7"
    );
}

#[test]
fn test_semantic_edge_id_derives() {
    let edge = SemanticEdge::new("a", "b", EdgeConcept::Derives);
    let edge_id: SemanticEdgeId = (&edge).into();
    assert_eq!(edge_id.0, edge.edge_id);
}

#[test]
fn test_semantic_graph_new() {
    let graph = SemanticGraph::new();
    assert_eq!(graph.node_count(), 0);
    assert_eq!(graph.edge_count(), 0);
}

#[test]
fn test_semantic_graph_add_node() {
    let mut graph = SemanticGraph::new();
    let id = SemanticId::day_root("2024-05-13", "tz+7");
    let node = SemanticNode::new(
        id,
        NodeConcept::DayCanchi,
        NodeOrigin::Fact,
        "Ngày 2024-05-13",
    );
    graph.add_node(node);
    assert_eq!(graph.node_count(), 1);
    assert!(graph.has_node("day:2024-05-13:tz+7"));
}

#[test]
fn test_semantic_graph_add_edge_requires_existing_nodes() {
    let mut graph = SemanticGraph::new();
    let edge = SemanticEdge::new("missing", "nodes", EdgeConcept::Composes);
    graph.add_edge(edge);
    assert_eq!(graph.edge_count(), 0);
}

#[test]
fn test_semantic_graph_add_edge_with_nodes() {
    let mut graph = SemanticGraph::new();
    let id1 = SemanticId::day_root("2024-05-13", "tz+7");
    let id2 = SemanticId::truc_day("2024-05-13", "tz+7");
    let node1 = SemanticNode::new(
        id1,
        NodeConcept::DayCanchi,
        NodeOrigin::Fact,
        "Ngày 2024-05-13",
    );
    let node2 = SemanticNode::new(id2, NodeConcept::Truc, NodeOrigin::Fact, "Trực");
    graph.add_node(node1);
    graph.add_node(node2);
    let edge = SemanticEdge::new(
        "day:2024-05-13:tz+7",
        "truc:day:2024-05-13:tz+7",
        EdgeConcept::Composes,
    );
    graph.add_edge(edge);
    assert_eq!(graph.edge_count(), 1);
}

#[test]
fn test_semantic_graph_validate_ok() {
    let mut graph = SemanticGraph::new();
    let id1 = SemanticId::day_root("2024-05-13", "tz+7");
    let id2 = SemanticId::truc_day("2024-05-13", "tz+7");
    let node1 = SemanticNode::new(
        id1,
        NodeConcept::DayCanchi,
        NodeOrigin::Fact,
        "Ngày 2024-05-13",
    );
    let node2 = SemanticNode::new(id2, NodeConcept::Truc, NodeOrigin::Fact, "Trực");
    graph.add_node(node1);
    graph.add_node(node2);
    let edge = SemanticEdge::new(
        "day:2024-05-13:tz+7",
        "truc:day:2024-05-13:tz+7",
        EdgeConcept::Composes,
    );
    graph.add_edge(edge);
    assert!(graph.validate().is_ok());
}

#[test]
fn test_semantic_graph_validate_missing_source() {
    let mut graph = SemanticGraph::new();
    let id1 = SemanticId::day_root("2024-05-13", "tz+7");
    let id2 = SemanticId::truc_day("2024-05-13", "tz+7");
    let node1 = SemanticNode::new(
        id1,
        NodeConcept::DayCanchi,
        NodeOrigin::Fact,
        "Ngày 2024-05-13",
    );
    let node2 = SemanticNode::new(id2, NodeConcept::Truc, NodeOrigin::Fact, "Trực");
    graph.add_node(node1);
    graph.add_node(node2);
    let edge = SemanticEdge::new(
        "nonexistent_node",
        "day:2024-05-13:tz+7",
        EdgeConcept::Composes,
    );
    graph.add_edge(edge);
    assert_eq!(graph.edge_count(), 0);
    assert!(graph.validate().is_ok());
}

#[test]
fn test_semantic_graph_add_edge_drops_dangling_edges() {
    let mut graph = SemanticGraph::new();
    let id = SemanticId::day_root("2024-05-13", "tz+7");
    let node = SemanticNode::new(
        id,
        NodeConcept::DayCanchi,
        NodeOrigin::Fact,
        "Ngày 2024-05-13",
    );
    graph.add_node(node);
    let edge = SemanticEdge::new("missing", "day:2024-05-13:tz+7", EdgeConcept::Composes);
    graph.add_edge(edge);
    assert_eq!(graph.edge_count(), 0);
}

#[test]
fn test_semantic_graph_merge_adds_new_nodes() {
    let mut graph1 = SemanticGraph::new();
    let id1 = SemanticId::day_root("2024-05-13", "tz+7");
    let node1 = SemanticNode::new(
        id1,
        NodeConcept::DayCanchi,
        NodeOrigin::Fact,
        "Ngày 2024-05-13",
    );
    graph1.add_node(node1);

    let mut graph2 = SemanticGraph::new();
    let id2 = SemanticId::truc_day("2024-05-13", "tz+7");
    let node2 = SemanticNode::new(id2, NodeConcept::Truc, NodeOrigin::Fact, "Trực");
    graph2.add_node(node2);

    graph1.merge(graph2).unwrap();
    assert_eq!(graph1.node_count(), 2);
}

#[test]
fn test_semantic_graph_merge_merges_provenance_on_existing_node() {
    let mut graph1 = SemanticGraph::new();
    let id1 = SemanticId::day_root("2024-05-13", "tz+7");
    let prov1 = ProvenanceEntry::snapshot("day:2024-05-13:tz+7", "method_a");
    let node1 = SemanticNode::new(
        id1,
        NodeConcept::DayCanchi,
        NodeOrigin::Fact,
        "Ngày 2024-05-13",
    )
    .with_provenance(prov1);
    graph1.add_node(node1);

    let mut graph2 = SemanticGraph::new();
    let id2 = SemanticId::day_root("2024-05-13", "tz+7");
    let prov2 = ProvenanceEntry::bazi("bazi_method", "method_b");
    let node2 = SemanticNode::new(
        id2,
        NodeConcept::DayCanchi,
        NodeOrigin::Fact,
        "Ngày 2024-05-13",
    )
    .with_provenance(prov2);
    graph2.add_node(node2);

    graph1.merge(graph2).unwrap();
    let merged = graph1.get_node("day:2024-05-13:tz+7").unwrap();
    assert_eq!(merged.provenance.len(), 2);
}

#[test]
fn test_semantic_graph_merge_rejects_kind_conflict() {
    let mut graph1 = SemanticGraph::new();
    let id1 = SemanticId::day_root("2024-05-13", "tz+7");
    let node1 = SemanticNode::new(
        id1,
        NodeConcept::DayCanchi,
        NodeOrigin::Fact,
        "Ngày 2024-05-13",
    );
    graph1.add_node(node1);

    let mut graph2 = SemanticGraph::new();
    let id2 = SemanticId::day_root("2024-05-13", "tz+7");
    let node2 = SemanticNode::new(id2, NodeConcept::Truc, NodeOrigin::Fact, "Trực");
    graph2.add_node(node2);

    let result = graph1.merge(graph2);
    assert!(matches!(
        result,
        Err(GraphMergeError::NodeKindConflict { .. })
    ));
}

#[test]
fn test_semantic_graph_merge_rejects_edge_conflict() {
    let mut graph1 = SemanticGraph::new();
    let id1 = SemanticId::day_root("2024-05-13", "tz+7");
    let id2 = SemanticId::truc_day("2024-05-13", "tz+7");
    let node1 = SemanticNode::new(
        id1,
        NodeConcept::DayCanchi,
        NodeOrigin::Fact,
        "Ngày 2024-05-13",
    );
    let node2 = SemanticNode::new(id2, NodeConcept::Truc, NodeOrigin::Fact, "Trực");
    graph1.add_node(node1);
    graph1.add_node(node2);
    let edge1 = SemanticEdge::new(
        "day:2024-05-13:tz+7",
        "truc:day:2024-05-13:tz+7",
        EdgeConcept::Composes,
    );
    graph1.add_edge(edge1);

    let mut graph2 = SemanticGraph::new();
    let id3 = SemanticId::day_root("2024-05-13", "tz+7");
    let id4 = SemanticId::truc_day("2024-05-13", "tz+7");
    let node3 = SemanticNode::new(
        id3,
        NodeConcept::DayCanchi,
        NodeOrigin::Fact,
        "Ngày 2024-05-13",
    );
    let node4 = SemanticNode::new(id4, NodeConcept::Truc, NodeOrigin::Fact, "Trực");
    graph2.add_node(node3);
    graph2.add_node(node4);
    let edge2 = SemanticEdge::new(
        "day:2024-05-13:tz+7",
        "truc:day:2024-05-13:tz+7",
        EdgeConcept::Resonates,
    );
    graph2.add_edge(edge2);

    let result = graph1.merge(graph2);
    assert!(matches!(
        result,
        Err(GraphMergeError::EdgePayloadConflict { .. })
    ));
}

#[test]
fn test_semantic_graph_outgoing_edges() {
    let mut graph = SemanticGraph::new();
    let id1 = SemanticId::day_root("2024-05-13", "tz+7");
    let id2 = SemanticId::truc_day("2024-05-13", "tz+7");
    let id3 = SemanticId::day_deity_day("2024-05-13", "tz+7");
    let node1 = SemanticNode::new(
        id1,
        NodeConcept::DayCanchi,
        NodeOrigin::Fact,
        "Ngày 2024-05-13",
    );
    let node2 = SemanticNode::new(id2, NodeConcept::Truc, NodeOrigin::Fact, "Trực");
    let node3 = SemanticNode::new(id3, NodeConcept::DayDeity, NodeOrigin::Fact, "Thần bế");
    graph.add_node(node1);
    graph.add_node(node2);
    graph.add_node(node3);
    let edge1 = SemanticEdge::new(
        "day:2024-05-13:tz+7",
        "truc:day:2024-05-13:tz+7",
        EdgeConcept::Composes,
    );
    let edge2 = SemanticEdge::new(
        "day:2024-05-13:tz+7",
        "day_deity:day:2024-05-13:tz+7",
        EdgeConcept::Composes,
    );
    graph.add_edge(edge1);
    graph.add_edge(edge2);

    let outgoing = graph.outgoing_edges("day:2024-05-13:tz+7");
    assert_eq!(outgoing.len(), 2);
}

#[test]
fn test_semantic_graph_incoming_edges() {
    let mut graph = SemanticGraph::new();
    let id1 = SemanticId::day_root("2024-05-13", "tz+7");
    let id2 = SemanticId::truc_day("2024-05-13", "tz+7");
    let node1 = SemanticNode::new(
        id1,
        NodeConcept::DayCanchi,
        NodeOrigin::Fact,
        "Ngày 2024-05-13",
    );
    let node2 = SemanticNode::new(id2, NodeConcept::Truc, NodeOrigin::Fact, "Trực");
    graph.add_node(node1);
    graph.add_node(node2);
    let edge = SemanticEdge::new(
        "day:2024-05-13:tz+7",
        "truc:day:2024-05-13:tz+7",
        EdgeConcept::Composes,
    );
    graph.add_edge(edge);

    let incoming = graph.incoming_edges("truc:day:2024-05-13:tz+7");
    assert_eq!(incoming.len(), 1);
}

#[test]
fn test_provenance_entry_helpers() {
    let snap = ProvenanceEntry::snapshot("id", "method");
    assert_eq!(snap.source, ProvenanceSource::Snapshot);

    let bazi = ProvenanceEntry::bazi("id", "method");
    assert_eq!(bazi.source, ProvenanceSource::Bazi);

    let almanac = ProvenanceEntry::almanac_rule("id", "method");
    assert_eq!(almanac.source, ProvenanceSource::AlmanacRule);

    let derived = ProvenanceEntry::derived("id", "method");
    assert_eq!(derived.source, ProvenanceSource::Derived);

    let with_note = ProvenanceEntry::snapshot("id", "method").with_note("test note");
    assert_eq!(with_note.note, Some("test note".to_string()));
}

#[test]
fn test_node_concept_label() {
    assert_eq!(NodeConcept::DayCanchi.label(), ConceptLabel::DayCanchi);
    assert_eq!(NodeConcept::Truc.label(), ConceptLabel::Truc);
    assert_eq!(NodeConcept::DayDeity.label(), ConceptLabel::DayDeity);
}

#[test]
fn test_edge_concept_label() {
    assert_eq!(EdgeConcept::Resonates.label(), ConceptLabel::Resonates);
    assert_eq!(EdgeConcept::Conflicts.label(), ConceptLabel::Conflicts);
    assert_eq!(EdgeConcept::Composes.label(), ConceptLabel::Composes);
}

#[test]
fn test_concept_label_as_str() {
    assert_eq!(ConceptLabel::DayCanchi.as_str(), "day_canchi");
    assert_eq!(ConceptLabel::Truc.as_str(), "truc");
    assert_eq!(ConceptLabel::Resonates.as_str(), "resonates");
    assert_eq!(ConceptLabel::Composes.as_str(), "composes");
}

#[test]
fn test_graph_ontology_lists() {
    let node_concepts = GraphOntology::node_concepts();
    assert!(node_concepts.contains(&NodeConcept::DayCanchi));
    assert!(node_concepts.contains(&NodeConcept::Truc));
    assert!(node_concepts.contains(&NodeConcept::DayDeity));

    let edge_concepts = GraphOntology::edge_concepts();
    assert!(edge_concepts.contains(&EdgeConcept::Resonates));
    assert!(edge_concepts.contains(&EdgeConcept::Composes));
}

#[test]
fn test_semantic_node_id_from_node() {
    let id = SemanticId::day_root("2024-05-13", "tz+7");
    let node = SemanticNode::new(
        id,
        NodeConcept::DayCanchi,
        NodeOrigin::Fact,
        "Ngày 2024-05-13",
    );
    let node_id: SemanticNodeId = (&node).into();
    assert_eq!(node_id.0, "day:2024-05-13:tz+7");
}

#[test]
fn test_semantic_node_id_display() {
    let node_id = SemanticNodeId("day:2024-05-13:tz+7".to_string());
    assert_eq!(format!("{}", node_id), "day:2024-05-13:tz+7");
}

#[test]
fn test_semantic_edge_id_display() {
    let edge = SemanticEdge::new("a", "b", EdgeConcept::Composes);
    let edge_id: SemanticEdgeId = (&edge).into();
    assert_eq!(format!("{}", edge_id), edge.edge_id);
}

#[test]
fn test_semantic_graph_serde_roundtrip() {
    let mut graph = SemanticGraph::new();
    let id1 = SemanticId::day_root("2024-05-13", "tz+7");
    let id2 = SemanticId::truc_day("2024-05-13", "tz+7");
    let prov = ProvenanceEntry::snapshot("day:2024-05-13:tz+7", "amlich-core");
    let node1 = SemanticNode::new(
        id1,
        NodeConcept::DayCanchi,
        NodeOrigin::Fact,
        "Ngày 2024-05-13",
    )
    .with_provenance(prov);
    let node2 = SemanticNode::new(id2, NodeConcept::Truc, NodeOrigin::Fact, "Trực");
    graph.add_node(node1);
    graph.add_node(node2);
    let edge = SemanticEdge::new(
        "day:2024-05-13:tz+7",
        "truc:day:2024-05-13:tz+7",
        EdgeConcept::Composes,
    );
    graph.add_edge(edge);

    let serialized = serde_json::to_string(&graph).unwrap();
    let deserialized: SemanticGraph = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.node_count(), 2);
    assert_eq!(deserialized.edge_count(), 1);
}

#[test]
fn test_semantic_graph_merge_with_edges() {
    let mut graph1 = SemanticGraph::new();
    let id1 = SemanticId::day_root("2024-05-13", "tz+7");
    let id2 = SemanticId::truc_day("2024-05-13", "tz+7");
    let node1 = SemanticNode::new(
        id1,
        NodeConcept::DayCanchi,
        NodeOrigin::Fact,
        "Ngày 2024-05-13",
    );
    let node2 = SemanticNode::new(id2, NodeConcept::Truc, NodeOrigin::Fact, "Trực");
    graph1.add_node(node1);
    graph1.add_node(node2);
    let edge1 = SemanticEdge::new(
        "day:2024-05-13:tz+7",
        "truc:day:2024-05-13:tz+7",
        EdgeConcept::Composes,
    );
    graph1.add_edge(edge1);

    let mut graph2 = SemanticGraph::new();
    let id3 = SemanticId::day_root("2024-05-13", "tz+7");
    let id4 = SemanticId::day_deity_day("2024-05-13", "tz+7");
    let node3 = SemanticNode::new(
        id3,
        NodeConcept::DayCanchi,
        NodeOrigin::Fact,
        "Ngày 2024-05-13",
    );
    let node4 = SemanticNode::new(id4, NodeConcept::DayDeity, NodeOrigin::Fact, "Thần bế");
    graph2.add_node(node3);
    graph2.add_node(node4);
    let edge2 = SemanticEdge::new(
        "day:2024-05-13:tz+7",
        "day_deity:day:2024-05-13:tz+7",
        EdgeConcept::Composes,
    );
    graph2.add_edge(edge2);

    graph1.merge(graph2).unwrap();
    assert_eq!(graph1.node_count(), 3);
    assert_eq!(graph1.edge_count(), 2);
}

#[test]
fn test_semantic_graph_get_edge() {
    let mut graph = SemanticGraph::new();
    let id1 = SemanticId::day_root("2024-05-13", "tz+7");
    let id2 = SemanticId::truc_day("2024-05-13", "tz+7");
    let node1 = SemanticNode::new(
        id1,
        NodeConcept::DayCanchi,
        NodeOrigin::Fact,
        "Ngày 2024-05-13",
    );
    let node2 = SemanticNode::new(id2, NodeConcept::Truc, NodeOrigin::Fact, "Trực");
    graph.add_node(node1);
    graph.add_node(node2);
    let edge = SemanticEdge::new(
        "day:2024-05-13:tz+7",
        "truc:day:2024-05-13:tz+7",
        EdgeConcept::Composes,
    );
    graph.add_edge(edge.clone());

    assert!(graph.get_edge(edge.edge_id()).is_some());
    assert!(graph.get_edge("nonexistent").is_none());
}

#[test]
fn test_semantic_graph_has_edge() {
    let mut graph = SemanticGraph::new();
    let id1 = SemanticId::day_root("2024-05-13", "tz+7");
    let id2 = SemanticId::truc_day("2024-05-13", "tz+7");
    let node1 = SemanticNode::new(
        id1,
        NodeConcept::DayCanchi,
        NodeOrigin::Fact,
        "Ngày 2024-05-13",
    );
    let node2 = SemanticNode::new(id2, NodeConcept::Truc, NodeOrigin::Fact, "Trực");
    graph.add_node(node1);
    graph.add_node(node2);
    let edge = SemanticEdge::new(
        "day:2024-05-13:tz+7",
        "truc:day:2024-05-13:tz+7",
        EdgeConcept::Composes,
    );
    graph.add_edge(edge.clone());

    assert!(graph.has_edge(edge.edge_id()));
    assert!(!graph.has_edge("nonexistent"));
}

#[test]
fn test_semantic_edge_label_with_weight() {
    let label = SemanticEdgeLabel::new(EdgeConcept::Resonates).with_weight(3);
    assert_eq!(label.weight, 3);
}

#[test]
fn test_semantic_edge_with_justification() {
    let edge =
        SemanticEdge::new("a", "b", EdgeConcept::Composes).with_justification("test justification");
    assert_eq!(edge.justification.len(), 1);
    assert_eq!(edge.justification[0], "test justification");
}

#[test]
fn test_semantic_edge_with_provenance() {
    let prov = ProvenanceEntry::snapshot("a", "method");
    let edge = SemanticEdge::new("a", "b", EdgeConcept::Composes).with_provenance(prov);
    assert_eq!(edge.provenance.len(), 1);
}

#[test]
fn test_semantic_node_with_tags() {
    let id = SemanticId::day_root("2024-05-13", "tz+7");
    let node = SemanticNode::new(
        id,
        NodeConcept::DayCanchi,
        NodeOrigin::Fact,
        "Ngày 2024-05-13",
    )
    .with_tags(vec!["test".to_string(), "example".to_string()]);
    assert_eq!(node.tags.len(), 2);
}

#[test]
fn test_semantic_node_with_severity() {
    let id = SemanticId::day_root("2024-05-13", "tz+7");
    let node = SemanticNode::new(
        id,
        NodeConcept::DayCanchi,
        NodeOrigin::Fact,
        "Ngày 2024-05-13",
    )
    .with_severity("hard");
    assert_eq!(node.severity, Some("hard".to_string()));
}

fn make_test_graph() -> SemanticGraph {
    let mut graph = SemanticGraph::new();
    let id1 = SemanticId::day_root("2024-05-13", "tz+7");
    let id2 = SemanticId::truc_day("2024-05-13", "tz+7");
    let id3 = SemanticId::day_deity_day("2024-05-13", "tz+7");
    let node1 = SemanticNode::new(
        id1,
        NodeConcept::DayCanchi,
        NodeOrigin::Fact,
        "Ngày 2024-05-13",
    );
    let node2 = SemanticNode::new(id2, NodeConcept::Truc, NodeOrigin::Fact, "Trực");
    let node3 = SemanticNode::new(id3, NodeConcept::DayDeity, NodeOrigin::Fact, "Thần bế");
    graph.add_node(node1);
    graph.add_node(node2);
    graph.add_node(node3);
    let edge1 = SemanticEdge::new(
        "day:2024-05-13:tz+7",
        "truc:day:2024-05-13:tz+7",
        EdgeConcept::Composes,
    );
    let edge2 = SemanticEdge::new(
        "day:2024-05-13:tz+7",
        "day_deity:day:2024-05-13:tz+7",
        EdgeConcept::Composes,
    );
    graph.add_edge(edge1);
    graph.add_edge(edge2);
    graph
}

#[test]
fn test_subgraph_view_extract() {
    let graph = make_test_graph();
    let view = SubgraphView::extract(&graph, &["day:2024-05-13:tz+7"], 1);
    assert!(view.root_ids.contains(&"day:2024-05-13:tz+7".to_string()));
    assert!(view
        .node_ids
        .contains(&"truc:day:2024-05-13:tz+7".to_string()));
    assert!(view
        .node_ids
        .contains(&"day_deity:day:2024-05-13:tz+7".to_string()));
}

#[test]
fn test_subgraph_view_extract_depth_limit() {
    let graph = make_test_graph();
    let view = SubgraphView::extract(&graph, &["day:2024-05-13:tz+7"], 0);
    assert_eq!(view.node_ids.len(), 1);
    assert_eq!(view.node_ids[0], "day:2024-05-13:tz+7");
}

#[test]
fn test_subgraph_view_empty() {
    let graph = SemanticGraph::new();
    let view = SubgraphView::extract(&graph, &["nonexistent"], 1);
    assert!(view.is_empty());
    assert_eq!(view.node_count(), 0);
    assert_eq!(view.edge_count(), 0);
}

#[test]
fn test_visualization_graph_from_semantic_graph() {
    let graph = make_test_graph();
    let vis = VisualizationGraph::from_semantic_graph(&graph);
    assert_eq!(vis.nodes.len(), 3);
    assert_eq!(vis.edges.len(), 2);
    assert!(vis.nodes.iter().all(|n| n.cluster == "day-core"));
}

#[test]
fn test_llm_graph_slice_from_subgraph() {
    let graph = make_test_graph();
    let view = SubgraphView::extract(&graph, &["day:2024-05-13:tz+7"], 1);
    let slice = LlmGraphSlice::from_subgraph(&graph, &view);
    assert!(!slice.is_empty());
    assert_eq!(slice.node_count(), 3);
    assert_eq!(slice.root_ids, view.root_ids);
    assert!(!slice.summary_points.is_empty());
}

#[test]
fn test_llm_graph_slice_empty() {
    let graph = SemanticGraph::new();
    let view = SubgraphView::extract(&graph, &["nonexistent"], 1);
    let slice = LlmGraphSlice::from_subgraph(&graph, &view);
    assert!(slice.is_empty());
    assert_eq!(slice.node_count(), 0);
}
