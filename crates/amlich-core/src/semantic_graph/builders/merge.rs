use crate::bazi::BaziInput;
use crate::reasoning::PersonalAssessmentFacts;
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
            snapshot.context.solar.year, snapshot.context.solar.month, snapshot.context.solar.day
        );
        let day_root_id = format!("day:{}:+7", date_str);

        Self {
            graph,
            day_root_id,
            profile_root_id: None,
        }
    }

    /// Build the reasoning input graph from a snapshot and a Bazi input,
    /// reusing a precomputed [`PersonalAssessmentFacts`] so the chart and
    /// interaction matrices are not recomputed. The [`BaziInput`] is only
    /// used to derive the profile root id; the Bazi chart and matrices come
    /// from `facts` (a single source of truth shared with the fact-node
    /// projection and the evaluator).
    pub fn from_day_and_bazi_with_facts(
        snapshot: &DaySnapshot,
        bazi_input: &BaziInput,
        facts: &PersonalAssessmentFacts,
    ) -> Result<Self, String> {
        use crate::semantic_graph::builders::{
            build_bazi_profile_graph, build_day_person_matrix_graph, build_day_snapshot_graph,
            build_direction_merge_matrix_graph, build_personal_hour_matrix_graph,
        };

        let mut day_graph = build_day_snapshot_graph(snapshot);
        let bazi_graph = build_bazi_profile_graph(&facts.chart, &facts.analysis);

        day_graph
            .merge(bazi_graph)
            .map_err(|e| format!("graph merge error: {:?}", e))?;

        let date_str = format!(
            "{:04}-{:02}-{:02}",
            snapshot.context.solar.year, snapshot.context.solar.month, snapshot.context.solar.day
        );
        let day_root_id = format!("day:{}:+7", date_str);

        let dob_str = format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}",
            bazi_input.year, bazi_input.month, bazi_input.day, bazi_input.hour, bazi_input.minute
        );
        let tz = format!("tz{:.1}", bazi_input.timezone);
        let profile_root_id = format!("bazi_profile:{}:{}", dob_str, tz);

        let day_person_graph = build_day_person_matrix_graph(
            &day_root_id,
            &profile_root_id,
            &facts.day_person_matrix,
        )?;
        day_graph
            .merge(day_person_graph)
            .map_err(|e| format!("matrix merge error: {:?}", e))?;

        if let Some(personal_hour_matrix) = &facts.personal_hour_matrix {
            let personal_hour_graph = build_personal_hour_matrix_graph(
                &day_root_id,
                &profile_root_id,
                personal_hour_matrix,
            )?;
            day_graph
                .merge(personal_hour_graph)
                .map_err(|e| format!("matrix merge error: {:?}", e))?;
        }

        if let Some(direction_matrix) = &facts.direction_merge_matrix {
            let direction_graph = build_direction_merge_matrix_graph(
                &day_root_id,
                &profile_root_id,
                direction_matrix,
            )?;
            day_graph
                .merge(direction_graph)
                .map_err(|e| format!("matrix merge error: {:?}", e))?;
        }

        Ok(Self {
            graph: day_graph,
            day_root_id,
            profile_root_id: Some(profile_root_id),
        })
    }

    pub fn from_day_and_bazi(
        snapshot: &DaySnapshot,
        bazi_input: &BaziInput,
    ) -> Result<Self, String> {
        let personal = crate::reasoning::PersonalReasoningInput::from_birth(
            crate::BirthInput {
                day: bazi_input.day,
                month: bazi_input.month,
                year: bazi_input.year,
                hour: bazi_input.time_known.then_some(bazi_input.hour),
                minute: bazi_input.time_known.then_some(bazi_input.minute),
                timezone: bazi_input.timezone,
                gender: bazi_input.gender,
                location_name: None,
            },
            crate::ConsultationIntent::OpeningBusiness,
        );
        let facts = PersonalAssessmentFacts::build(&personal, snapshot)?;
        Self::from_day_and_bazi_with_facts(snapshot, bazi_input, &facts)
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

/// Build the reasoning input graph reusing a precomputed
/// [`PersonalAssessmentFacts`], so the chart and matrices are not recomputed
/// alongside the graph build. Callers that already have facts (per-request
/// request paths) must use this entry point to avoid duplicate work.
pub fn build_reasoning_input_graph_with_facts(
    snapshot: &DaySnapshot,
    bazi_input: Option<&BaziInput>,
    facts: Option<&PersonalAssessmentFacts>,
) -> Result<SemanticGraph, String> {
    let reasoning = match (bazi_input, facts) {
        (Some(input), Some(facts)) => {
            ReasoningInputGraph::from_day_and_bazi_with_facts(snapshot, input, facts)?
        }
        (Some(input), None) => ReasoningInputGraph::from_day_and_bazi(snapshot, input)?,
        (None, _) => ReasoningInputGraph::from_day_snapshot(snapshot),
    };
    Ok(reasoning.graph)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bazi::BaziInput;
    use crate::calculate_day_snapshot;

    #[test]
    fn reasoning_input_graph_from_day_only() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let graph = build_reasoning_input_graph(&snapshot, None).expect("valid graph");

        let date_str = "2024-02-10";
        let day_root_id = format!("day:{}:+7", date_str);
        assert!(graph.has_node(&day_root_id), "should have day root");
        assert!(
            graph.node_count() >= 3,
            "should have day root plus canchi/solar term"
        );
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
            time_known: true,
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
        assert!(
            graph.has_node(&profile_root_id),
            "should have bazi profile root"
        );
    }

    #[test]
    fn reasoning_input_graph_merges_interaction_matrices() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let bazi_input = BaziInput {
            day: 15,
            month: 8,
            year: 1990,
            hour: 9,
            minute: 30,
            time_known: true,
            timezone: 7.0,
            longitude: None,
            use_solar_time: false,
            gender: Some(crate::almanac::tu_menh::Gender::Male),
        };

        let graph = build_reasoning_input_graph(&snapshot, Some(&bazi_input)).expect("valid graph");

        let has_day_person_matrix = graph
            .nodes()
            .values()
            .any(|n| n.concept == crate::semantic_graph::ontology::NodeConcept::DayPersonMatrix);
        assert!(has_day_person_matrix, "should have day-person matrix node");

        let has_personal_hour_matrix = graph
            .nodes()
            .values()
            .any(|n| n.concept == crate::semantic_graph::ontology::NodeConcept::PersonalHourMatrix);
        assert!(
            has_personal_hour_matrix,
            "should have personal-hour matrix node"
        );

        let has_direction_merge_matrix = graph.nodes().values().any(|n| {
            n.concept == crate::semantic_graph::ontology::NodeConcept::DirectionMergeMatrix
        });
        assert!(
            has_direction_merge_matrix,
            "should have direction-merge matrix node"
        );
    }

    #[test]
    fn reasoning_input_graph_without_gender_skips_direction_matrix() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let bazi_input = BaziInput {
            day: 15,
            month: 8,
            year: 1990,
            hour: 9,
            minute: 30,
            time_known: true,
            timezone: 7.0,
            longitude: None,
            use_solar_time: false,
            gender: None,
        };

        let graph = build_reasoning_input_graph(&snapshot, Some(&bazi_input)).expect("valid graph");

        let has_direction_merge_matrix = graph.nodes().values().any(|n| {
            n.concept == crate::semantic_graph::ontology::NodeConcept::DirectionMergeMatrix
        });
        assert!(
            !has_direction_merge_matrix,
            "should NOT have direction-merge matrix without gender"
        );
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
            time_known: true,
            timezone: 7.0,
            longitude: None,
            use_solar_time: false,
            gender: Some(crate::almanac::tu_menh::Gender::Male),
        };

        let graph1 =
            build_reasoning_input_graph(&snapshot, Some(&bazi_input)).expect("valid graph1");
        let graph2 =
            build_reasoning_input_graph(&snapshot, Some(&bazi_input)).expect("valid graph2");

        assert_eq!(
            graph1.node_count(),
            graph2.node_count(),
            "node count should be deterministic"
        );
        assert_eq!(
            graph1.edge_count(),
            graph2.edge_count(),
            "edge count should be deterministic"
        );
    }

    #[test]
    fn reasoning_input_graph_different_days_different_graphs() {
        let snap1 = calculate_day_snapshot(10, 2, 2024);
        let snap2 = calculate_day_snapshot(11, 2, 2024);

        let graph1 = build_reasoning_input_graph(&snap1, None).expect("valid graph1");
        let graph2 = build_reasoning_input_graph(&snap2, None).expect("valid graph2");

        let ids1: Vec<_> = graph1.nodes().keys().collect();
        let ids2: Vec<_> = graph2.nodes().keys().collect();

        assert_ne!(
            ids1, ids2,
            "different days should produce different node IDs"
        );
    }

    #[test]
    fn reasoning_input_graph_preserves_provenance() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let graph = build_reasoning_input_graph(&snapshot, None).expect("valid graph");

        for node in graph.nodes().values() {
            assert!(
                !node.provenance.is_empty(),
                "node {} should have provenance",
                node.node_id
            );
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
            time_known: true,
            timezone: 7.0,
            longitude: None,
            use_solar_time: false,
            gender: Some(crate::almanac::tu_menh::Gender::Male),
        };

        let graph = build_reasoning_input_graph(&snapshot, Some(&bazi_input)).expect("valid graph");

        for node in graph.nodes().values() {
            assert!(
                !node.provenance.is_empty(),
                "node {} should have provenance",
                node.node_id
            );
        }
    }
}
