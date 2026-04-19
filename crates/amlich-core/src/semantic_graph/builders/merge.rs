use crate::bazi::{analyze_bazi_chart, build_bazi_chart, BaziInput};
use crate::semantic_graph::SemanticGraph;
use crate::DaySnapshot;

pub struct ReasoningInputGraph {
    pub graph: SemanticGraph,
    pub day_root_id: String,
    pub profile_root_id: Option<String>,
}

impl ReasoningInputGraph {
    pub fn from_day_snapshot(snapshot: &DaySnapshot) -> Self {
        use crate::semantic_graph::builders::build_day_snapshot_graph;

        let graph = build_day_snapshot_graph(snapshot);
        let date_str = format!(
            "{:04}-{:02}-{:02}",
            snapshot.context.solar.year,
            snapshot.context.solar.month,
            snapshot.context.solar.day
        );
        let day_root_id = format!("day:{}:+7", date_str);

        Self {
            graph,
            day_root_id,
            profile_root_id: None,
        }
    }

    pub fn from_day_and_bazi(
        snapshot: &DaySnapshot,
        bazi_input: &BaziInput,
    ) -> Result<Self, String> {
        use crate::semantic_graph::builders::{build_bazi_profile_graph, build_day_snapshot_graph};

        let mut day_graph = build_day_snapshot_graph(snapshot);
        let chart = build_bazi_chart(bazi_input.clone())?;
        let analysis = analyze_bazi_chart(&chart);
        let bazi_graph = build_bazi_profile_graph(&chart, &analysis);

        day_graph.merge(bazi_graph).map_err(|e| format!("graph merge error: {:?}", e))?;

        let date_str = format!(
            "{:04}-{:02}-{:02}",
            snapshot.context.solar.year,
            snapshot.context.solar.month,
            snapshot.context.solar.day
        );
        let day_root_id = format!("day:{}:+7", date_str);

        let dob_str = format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}",
            bazi_input.year, bazi_input.month, bazi_input.day,
            bazi_input.hour, bazi_input.minute
        );
        let tz = format!("tz{:.1}", bazi_input.timezone);
        let profile_root_id = format!("bazi_profile:{}:{}", dob_str, tz);

        Ok(Self {
            graph: day_graph,
            day_root_id,
            profile_root_id: Some(profile_root_id),
        })
    }

    pub fn has_profile(&self) -> bool {
        self.profile_root_id.is_some()
    }
}

pub fn build_reasoning_input_graph(
    snapshot: &DaySnapshot,
    bazi_input: Option<&BaziInput>,
) -> Result<SemanticGraph, String> {
    let reasoning = match bazi_input {
        Some(input) => ReasoningInputGraph::from_day_and_bazi(snapshot, input)?,
        None => ReasoningInputGraph::from_day_snapshot(snapshot),
    };
    Ok(reasoning.graph)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calculate_day_snapshot;
    use crate::bazi::BaziInput;

    #[test]
    fn reasoning_input_graph_from_day_only() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let graph = build_reasoning_input_graph(&snapshot, None).expect("valid graph");

        let date_str = "2024-02-10";
        let day_root_id = format!("day:{}:+7", date_str);
        assert!(graph.has_node(&day_root_id), "should have day root");
        assert!(graph.node_count() >= 3, "should have day root plus canchi/solar term");
    }

    #[test]
    fn reasoning_input_graph_merges_bazi() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let bazi_input = BaziInput {
            day: 15,
            month: 8,
            year: 1990,
            hour: 9,
            minute: 30,
            timezone: 7.0,
            longitude: None,
            use_solar_time: false,
            gender: Some(crate::almanac::tu_menh::Gender::Male),
        };

        let graph = build_reasoning_input_graph(&snapshot, Some(&bazi_input)).expect("valid graph");

        let date_str = "2024-02-10";
        let day_root_id = format!("day:{}:+7", date_str);
        assert!(graph.has_node(&day_root_id), "should have day root");

        let dob_str = "1990-08-15T09:30";
        let tz = "tz7.0";
        let profile_root_id = format!("bazi_profile:{}:{}", dob_str, tz);
        assert!(graph.has_node(&profile_root_id), "should have bazi profile root");
    }

    #[test]
    fn reasoning_input_graph_deterministic() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let bazi_input = BaziInput {
            day: 15,
            month: 8,
            year: 1990,
            hour: 9,
            minute: 30,
            timezone: 7.0,
            longitude: None,
            use_solar_time: false,
            gender: Some(crate::almanac::tu_menh::Gender::Male),
        };

        let graph1 = build_reasoning_input_graph(&snapshot, Some(&bazi_input)).expect("valid graph1");
        let graph2 = build_reasoning_input_graph(&snapshot, Some(&bazi_input)).expect("valid graph2");

        assert_eq!(graph1.node_count(), graph2.node_count(), "node count should be deterministic");
        assert_eq!(graph1.edge_count(), graph2.edge_count(), "edge count should be deterministic");
    }

    #[test]
    fn reasoning_input_graph_different_days_different_graphs() {
        let snap1 = calculate_day_snapshot(10, 2, 2024);
        let snap2 = calculate_day_snapshot(11, 2, 2024);

        let graph1 = build_reasoning_input_graph(&snap1, None).expect("valid graph1");
        let graph2 = build_reasoning_input_graph(&snap2, None).expect("valid graph2");

        let ids1: Vec<_> = graph1.nodes().keys().collect();
        let ids2: Vec<_> = graph2.nodes().keys().collect();

        assert_ne!(ids1, ids2, "different days should produce different node IDs");
    }

    #[test]
    fn reasoning_input_graph_preserves_provenance() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let graph = build_reasoning_input_graph(&snapshot, None).expect("valid graph");

        for (_, node) in graph.nodes() {
            assert!(!node.provenance.is_empty(), "node {} should have provenance", node.node_id);
        }
    }

    #[test]
    fn reasoning_input_graph_with_bazi_preserves_provenance() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let bazi_input = BaziInput {
            day: 15,
            month: 8,
            year: 1990,
            hour: 9,
            minute: 30,
            timezone: 7.0,
            longitude: None,
            use_solar_time: false,
            gender: Some(crate::almanac::tu_menh::Gender::Male),
        };

        let graph = build_reasoning_input_graph(&snapshot, Some(&bazi_input)).expect("valid graph");

        for (_, node) in graph.nodes() {
            assert!(!node.provenance.is_empty(), "node {} should have provenance", node.node_id);
        }
    }
}