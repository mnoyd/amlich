use crate::dto::{
    DebugInspectionDateDto, DebugInspectionSummaryDto, DebugSemanticGraphQueryDto,
    DebugSemanticGraphResponseDto, DebugVisualizationDto, DebugVisualizationEdgeDto,
    DebugVisualizationNodeDto,
};

pub fn get_debug_semantic_graph_inspection(
    query: &DebugSemanticGraphQueryDto,
) -> Result<DebugSemanticGraphResponseDto, String> {
    if !(1..=12).contains(&query.month) {
        return Err("month must be 1-12".to_string());
    }
    if !(1..=31).contains(&query.day) {
        return Err("day must be 1-31".to_string());
    }

    let inspection = amlich_core::debug_inspect_semantic_graph(
        query.day,
        query.month,
        query.year,
        query.include_recommendations,
    );

    Ok(DebugSemanticGraphResponseDto {
        surface: inspection.surface,
        date: DebugInspectionDateDto {
            year: inspection.date.year,
            month: inspection.date.month,
            day: inspection.date.day,
        },
        visualization: DebugVisualizationDto {
            nodes: inspection
                .visualization
                .nodes
                .into_iter()
                .map(|n| DebugVisualizationNodeDto {
                    node_id: n.node_id,
                    label: n.label,
                    cluster: n.cluster,
                    semantic_kind: n.semantic_kind,
                    severity: n.severity,
                    shape_hint: n.shape_hint,
                })
                .collect(),
            edges: inspection
                .visualization
                .edges
                .into_iter()
                .map(|e| DebugVisualizationEdgeDto {
                    edge_id: e.edge_id,
                    from_id: e.from_id,
                    to_id: e.to_id,
                    label: e.label,
                    semantic_kind: e.semantic_kind,
                    weight: e.weight,
                })
                .collect(),
        },
        summary: DebugInspectionSummaryDto {
            total_nodes: inspection.summary.total_nodes,
            total_edges: inspection.summary.total_edges,
            clusters: inspection.summary.clusters,
            semantic_kinds: inspection.summary.semantic_kinds,
            has_recommendation_evidence: inspection.summary.has_recommendation_evidence,
        },
        cluster_counts: inspection.cluster_counts,
        semantic_kind_counts: inspection.semantic_kind_counts,
        severity_counts: inspection.severity_counts,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_inspection_returns_valid_response() {
        let query = DebugSemanticGraphQueryDto {
            day: 10,
            month: 2,
            year: 2024,
            include_recommendations: true,
        };
        let response = get_debug_semantic_graph_inspection(&query).expect("should succeed");
        assert_eq!(response.surface, "debug_semantic_graph_inspector");
        assert!(response.summary.total_nodes > 0);
        assert!(response.summary.has_recommendation_evidence);
        assert!(!response.visualization.nodes.is_empty());
    }

    #[test]
    fn debug_inspection_rejects_invalid_month() {
        let query = DebugSemanticGraphQueryDto {
            day: 10,
            month: 13,
            year: 2024,
            include_recommendations: false,
        };
        let err = get_debug_semantic_graph_inspection(&query).expect_err("should fail");
        assert!(err.contains("month must be 1-12"));
    }

    #[test]
    fn debug_inspection_without_recommendations() {
        let query = DebugSemanticGraphQueryDto {
            day: 10,
            month: 2,
            year: 2024,
            include_recommendations: false,
        };
        let response = get_debug_semantic_graph_inspection(&query).expect("should succeed");
        assert!(response.summary.total_nodes > 0);
    }
}
