//! Black-box integration tests for v1.11 `amlich-xlag.2.2.6`
//! (EXPLAIN-01) — the additive `DaySnapshot.point_opening` projection
//! and its citation-only semantic-graph wiring.
//!
//! ## Success criteria (bead acceptance)
//!
//! 1. **Additive compatibility** — the `point_opening` field is
//!    `None` on ordinary snapshots, absent from their JSON, and
//!    populated snapshots round-trip byte-equal for both open and
//!    closed slots.
//! 2. **Citation-only graph semantics** — open rows project
//!    `ClassicallyCitedPoint` nodes wired to the day root via
//!    `ClassicallyCitedOpenAt`; closed rows project exactly one
//!    `ClassicallyCitedClosedSlot` node wired via
//!    `ClassicallyCitedClosedAt`. Nothing in the graph asserts
//!    physiological flow, organ performance, or treatment suitability.
//! 3. **Assessment isolation** — enriching with the point-opening
//!    context leaves `traditional_wellness` untouched (and vice
//!    versa) and never perturbs the Day Assessment surfaces
//!    (`day_fortune`, `daily_recommendations`).
//!
//! Fixture: 2024-02-10 is the verified Giáp-day Julian date shared
//! with `point_opening_civil_time.rs`; the frozen corpus gives every
//! day both open and closed hour branches.

use amlich_core::canchi::get_day_canchi;
use amlich_core::point_opening::PointOpeningSlotState;
use amlich_core::semantic_graph::build_day_snapshot_graph;
use amlich_core::semantic_graph::{EdgeConcept, NodeConcept};
use amlich_core::sources::SOURCE_TY_NGO_LUU_CHU;
use amlich_core::{calculate_day_snapshot, enrich_day_snapshot_with_point_opening};

/// 2024-02-10 — the verified Giáp-day fixture.
fn sample_snapshot() -> amlich_core::DaySnapshot {
    calculate_day_snapshot(10, 2, 2024)
}

/// One representative local civil hour per hour branch (Tý at 23:30,
/// Sửu at 01:30, … Hợi at 21:30), per the existing hour-branch
/// convention.
const BRANCH_HOURS: [u8; 12] = [23, 1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21];

fn day_root_id(snapshot: &amlich_core::DaySnapshot) -> String {
    format!(
        "day:{:04}-{:02}-{:02}:+7",
        snapshot.context.solar.year, snapshot.context.solar.month, snapshot.context.solar.day
    )
}

// ───────────────────────────────────────────────────────────────────────
// 1. Additive compatibility — absence
// ───────────────────────────────────────────────────────────────────────

#[test]
fn ordinary_snapshot_leaves_point_opening_absent_everywhere() {
    let snap = sample_snapshot();
    assert!(
        snap.point_opening.is_none(),
        "ordinary snapshots must not auto-populate point_opening"
    );

    let json = serde_json::to_string(&snap).expect("serialise ordinary");
    assert!(
        !json.contains("\"point_opening\""),
        "point_opening must NOT appear in JSON when None; got {json}"
    );
    let parsed: amlich_core::DaySnapshot =
        serde_json::from_str(&json).expect("v1.10-era JSON must still deserialize");
    assert!(parsed.point_opening.is_none());
    assert_eq!(
        serde_json::to_string(&parsed).unwrap(),
        json,
        "ordinary snapshot must round-trip byte-equal (v1.10 → v1.11 wire compat)"
    );

    let graph = build_day_snapshot_graph(&snap);
    let point_nodes = graph
        .nodes()
        .values()
        .filter(|n| {
            matches!(
                n.concept,
                NodeConcept::ClassicallyCitedPoint | NodeConcept::ClassicallyCitedClosedSlot
            )
        })
        .count();
    assert_eq!(point_nodes, 0, "no implicit point-opening nodes");
    let citation_edges = graph
        .edges()
        .values()
        .filter(|e| {
            matches!(
                e.label.concept,
                EdgeConcept::ClassicallyCitedOpenAt | EdgeConcept::ClassicallyCitedClosedAt
            )
        })
        .count();
    assert_eq!(citation_edges, 0, "no implicit point-opening edges");
    graph.validate().expect("ordinary graph must validate");
}

// ───────────────────────────────────────────────────────────────────────
// 2. Citation-only graph semantics — open and closed rows
// ───────────────────────────────────────────────────────────────────────

#[test]
fn open_slots_project_point_citation_nodes_with_open_edges() {
    let snap = sample_snapshot();
    let mut open_hour = None;
    for &hour in &BRANCH_HOURS {
        let enriched = enrich_day_snapshot_with_point_opening(&snap, hour, 30).unwrap();
        if matches!(
            enriched.point_opening.as_ref().unwrap().context.state,
            PointOpeningSlotState::Open { .. }
        ) {
            open_hour = Some(hour);
            break;
        }
    }
    let hour = open_hour.expect("fixture day must have an open branch");
    let enriched = enrich_day_snapshot_with_point_opening(&snap, hour, 30).unwrap();
    let ctx = enriched.point_opening.as_ref().unwrap();
    let PointOpeningSlotState::Open { points, .. } = &ctx.context.state else {
        panic!("fixture must be open");
    };
    assert!(!points.is_empty());

    let graph = build_day_snapshot_graph(&enriched);
    let point_nodes: Vec<_> = graph
        .nodes()
        .values()
        .filter(|n| matches!(n.concept, NodeConcept::ClassicallyCitedPoint))
        .collect();
    assert_eq!(
        point_nodes.len(),
        points.len(),
        "one citation node per point identity"
    );
    assert_eq!(
        graph
            .nodes()
            .values()
            .filter(|n| matches!(n.concept, NodeConcept::ClassicallyCitedClosedSlot))
            .count(),
        0,
        "open rows never emit closed-slot nodes"
    );

    let root = day_root_id(&snap);
    let open_edges: Vec<_> = graph
        .edges()
        .values()
        .filter(|e| matches!(e.label.concept, EdgeConcept::ClassicallyCitedOpenAt))
        .collect();
    assert_eq!(open_edges.len(), points.len());
    for edge in &open_edges {
        assert_eq!(
            edge.to_node_id, root,
            "edges wire citations to the day root"
        );
        let from = graph.nodes().get(&edge.from_node_id).expect("from exists");
        assert!(matches!(from.concept, NodeConcept::ClassicallyCitedPoint));
    }

    for node in &point_nodes {
        assert!(
            node.tags
                .iter()
                .any(|t| t == "safety_class=historical_procedural_citation"),
            "citation nodes must carry the canonical safety class tag; got {:?}",
            node.tags
        );
        assert!(
            node.tags.iter().any(|t| t.starts_with("xue_ming_zh=")),
            "citation nodes must carry the printed 穴名 tag"
        );
        assert!(
            node.tags.iter().any(|t| t.starts_with("code_gloss=")),
            "citation nodes must carry the lookup-gloss tag"
        );
        assert!(!node.provenance.is_empty());
        for entry in &node.provenance {
            assert_eq!(
                entry.source_id, *SOURCE_TY_NGO_LUU_CHU,
                "graph provenance must cite the TNLC primitive source only"
            );
        }
    }
    graph.validate().expect("open-slot graph must validate");
}

#[test]
fn closed_slots_project_one_closed_citation_node_with_closed_edge() {
    let snap = sample_snapshot();
    let mut closed_hour = None;
    for &hour in &BRANCH_HOURS {
        let enriched = enrich_day_snapshot_with_point_opening(&snap, hour, 30).unwrap();
        if matches!(
            enriched.point_opening.as_ref().unwrap().context.state,
            PointOpeningSlotState::Closed { .. }
        ) {
            closed_hour = Some(hour);
            break;
        }
    }
    let hour = closed_hour.expect("fixture day must have a closed branch");
    let enriched = enrich_day_snapshot_with_point_opening(&snap, hour, 30).unwrap();
    let ctx = enriched.point_opening.as_ref().unwrap();
    let PointOpeningSlotState::Closed { running_tables, .. } = &ctx.context.state else {
        panic!("fixture must be closed");
    };
    assert!(!running_tables.is_empty());

    let graph = build_day_snapshot_graph(&enriched);
    let closed_nodes: Vec<_> = graph
        .nodes()
        .values()
        .filter(|n| matches!(n.concept, NodeConcept::ClassicallyCitedClosedSlot))
        .collect();
    assert_eq!(
        closed_nodes.len(),
        1,
        "exactly one closed-slot citation node"
    );
    assert_eq!(
        graph
            .nodes()
            .values()
            .filter(|n| matches!(n.concept, NodeConcept::ClassicallyCitedPoint))
            .count(),
        0,
        "closed rows never emit point nodes"
    );

    let node = closed_nodes[0];
    assert!(
        node.tags
            .iter()
            .any(|t| t.starts_with("running_tables=") && t != "running_tables="),
        "closed node must carry the running-tables tag; got {:?}",
        node.tags
    );
    assert!(node
        .tags
        .iter()
        .any(|t| t == "safety_class=historical_procedural_citation"));
    assert_eq!(node.provenance.len(), 1);
    assert_eq!(node.provenance[0].source_id, *SOURCE_TY_NGO_LUU_CHU);

    let root = day_root_id(&snap);
    let closed_edges: Vec<_> = graph
        .edges()
        .values()
        .filter(|e| matches!(e.label.concept, EdgeConcept::ClassicallyCitedClosedAt))
        .collect();
    assert_eq!(closed_edges.len(), 1);
    assert_eq!(closed_edges[0].to_node_id, root);
    assert_eq!(closed_edges[0].from_node_id, node.node_id);

    graph.validate().expect("closed-slot graph must validate");
}

#[test]
fn every_branch_of_the_fixture_day_projects_exactly_one_state() {
    let snap = sample_snapshot();
    let mut seen_open = 0;
    let mut seen_closed = 0;
    for &hour in &BRANCH_HOURS {
        let enriched = enrich_day_snapshot_with_point_opening(&snap, hour, 30)
            .expect("every branch hour is valid");
        let graph = build_day_snapshot_graph(&enriched);
        let point_count = graph
            .nodes()
            .values()
            .filter(|n| matches!(n.concept, NodeConcept::ClassicallyCitedPoint))
            .count();
        let closed_count = graph
            .nodes()
            .values()
            .filter(|n| matches!(n.concept, NodeConcept::ClassicallyCitedClosedSlot))
            .count();
        let open_edge_count = graph
            .edges()
            .values()
            .filter(|e| matches!(e.label.concept, EdgeConcept::ClassicallyCitedOpenAt))
            .count();
        let closed_edge_count = graph
            .edges()
            .values()
            .filter(|e| matches!(e.label.concept, EdgeConcept::ClassicallyCitedClosedAt))
            .count();
        graph.validate().expect("every branch graph must validate");

        match enriched.point_opening.as_ref().unwrap().context.state {
            PointOpeningSlotState::Open { .. } => {
                seen_open += 1;
                assert!(point_count >= 1 && closed_count == 0);
                assert_eq!(open_edge_count, point_count);
                assert_eq!(closed_edge_count, 0);
            }
            PointOpeningSlotState::Closed { .. } => {
                seen_closed += 1;
                assert_eq!(closed_count, 1);
                assert_eq!(point_count, 0);
                assert_eq!(closed_edge_count, 1);
                assert_eq!(open_edge_count, 0);
            }
        }
    }
    assert!(
        seen_open > 0 && seen_closed > 0,
        "fixture day covers both states"
    );
}

// ───────────────────────────────────────────────────────────────────────
// 3. Graph wording stays citation-only
// ───────────────────────────────────────────────────────────────────────

#[test]
fn graph_json_carries_citation_labels_and_no_treatment_semantics() {
    let snap = sample_snapshot();
    for &hour in &BRANCH_HOURS {
        let enriched = enrich_day_snapshot_with_point_opening(&snap, hour, 30).unwrap();
        let graph = build_day_snapshot_graph(&enriched);
        let json = serde_json::to_string(&graph).unwrap();
        for forbidden in [
            "physiological",
            "flow_through",
            "best_time",
            "peak_at",
            "treats",
            "cures",
            "prescribes",
            "should_be_needled",
            "nên châm",
            "nên bấm",
        ] {
            assert!(
                !json.contains(forbidden),
                "graph JSON must not carry treatment wording `{forbidden}`; got {json}"
            );
        }
    }

    // The citation labels are the schema surface consumers depend on.
    let enriched = enrich_day_snapshot_with_point_opening(&snap, BRANCH_HOURS[0], 30).unwrap();
    let graph = build_day_snapshot_graph(&enriched);
    let json = serde_json::to_string(&graph).unwrap();
    assert!(
        json.contains("classically_cited_point") || json.contains("classically_cited_closed_slot")
    );
    assert!(
        json.contains("classically_cited_open_at") || json.contains("classically_cited_closed_at")
    );
}

// ───────────────────────────────────────────────────────────────────────
// 4. Additive compatibility — populated round trips
// ───────────────────────────────────────────────────────────────────────

#[test]
fn populated_snapshots_round_trip_byte_equal_for_open_and_closed() {
    let snap = sample_snapshot();
    let mut seen_open = false;
    let mut seen_closed = false;
    for &hour in &BRANCH_HOURS {
        let enriched = enrich_day_snapshot_with_point_opening(&snap, hour, 30).unwrap();
        match enriched.point_opening.as_ref().unwrap().context.state {
            PointOpeningSlotState::Open { .. } => seen_open = true,
            PointOpeningSlotState::Closed { .. } => seen_closed = true,
        }
        let json1 = serde_json::to_string(&enriched).expect("serialise enriched");
        assert!(
            json1.contains("\"point_opening\""),
            "point_opening must appear in JSON when Some"
        );
        let parsed: amlich_core::DaySnapshot =
            serde_json::from_str(&json1).expect("deserialise enriched");
        let json2 = serde_json::to_string(&parsed).expect("re-serialise");
        assert_eq!(json1, json2, "enriched snapshot must round-trip byte-equal");
        assert!(parsed.point_opening.is_some());
    }
    assert!(seen_open && seen_closed);
}

// ───────────────────────────────────────────────────────────────────────
// 5. Assessment isolation
// ───────────────────────────────────────────────────────────────────────

#[test]
fn point_opening_and_traditional_wellness_never_populate_each_other() {
    let snap = sample_snapshot();

    let with_point_opening =
        enrich_day_snapshot_with_point_opening(&snap, BRANCH_HOURS[0], 30).unwrap();
    assert!(
        with_point_opening.point_opening.is_some()
            && with_point_opening.traditional_wellness.is_none(),
        "point-opening enrichment must leave traditional_wellness untouched"
    );

    let with_wellness = amlich_core::enrich_day_snapshot_with_traditional_wellness(
        &snap,
        snap.context.jd,
        7.0,
        9,
        30,
    )
    .unwrap();
    assert!(
        with_wellness.traditional_wellness.is_some() && with_wellness.point_opening.is_none(),
        "traditional-wellness enrichment must leave point_opening untouched"
    );

    // Day Assessment surfaces are byte-identical to the ordinary
    // snapshot after point-opening enrichment.
    let baseline_fortune = serde_json::to_string(&snap.day_fortune).unwrap();
    let baseline_reco = serde_json::to_string(&snap.daily_recommendations).unwrap();
    assert_eq!(
        serde_json::to_string(&with_point_opening.day_fortune).unwrap(),
        baseline_fortune,
        "Day Assessment must be untouched"
    );
    assert_eq!(
        serde_json::to_string(&with_point_opening.daily_recommendations).unwrap(),
        baseline_reco,
        "daily recommendations must be untouched"
    );
}

// ───────────────────────────────────────────────────────────────────────
// 6. Civil-boundary behaviour at the snapshot boundary
// ───────────────────────────────────────────────────────────────────────

#[test]
fn late_night_enrichment_discloses_the_frozen_day_transition() {
    let snap = sample_snapshot();
    let enriched = enrich_day_snapshot_with_point_opening(&snap, 23, 30).unwrap();
    let ctx = enriched.point_opening.as_ref().unwrap();
    assert!(ctx.late_night_day_transition);
    assert_eq!(ctx.hour_branch_vi, "Tý");
    assert_eq!(
        ctx.slot_day_canchi.full,
        get_day_canchi(snap.context.jd + 1).full,
        "the 23:00 block belongs to the upcoming civil date (TNLC-DIV-03)"
    );
    assert_eq!(
        ctx.civil_day_canchi.full,
        get_day_canchi(snap.context.jd).full
    );
    assert!(ctx
        .calendar_evidence
        .first()
        .and_then(|e| e.note.as_deref())
        .unwrap_or_default()
        .contains("TNLC-DIV-03"));

    let ordinary = enrich_day_snapshot_with_point_opening(&snap, 12, 30).unwrap();
    assert!(
        !ordinary
            .point_opening
            .as_ref()
            .unwrap()
            .late_night_day_transition
    );
}

#[test]
fn invalid_civil_times_are_rejected_at_the_snapshot_boundary() {
    let snap = sample_snapshot();
    for (hour, minute) in [(24u8, 0u8), (23, 60)] {
        assert!(
            enrich_day_snapshot_with_point_opening(&snap, hour, minute).is_err(),
            "{hour}:{minute:02} must be rejected"
        );
    }
}
