use std::collections::{BTreeMap, HashMap};

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use chrono::Datelike;

use crate::layout::LayoutMode;
use crate::state::{AppState, GraphInspectorFocus, RecommendationLensVerdict, UserExplanationLens};
use crate::view_models::decision_stack::DecisionRole;

pub struct GraphInspectorScreenWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

#[derive(Debug, Clone)]
struct DecisionHeaderSummary {
    primary_conclusion: String,
    bucket_label: String,
    confidence_label: String,
}

#[derive(Debug, Clone)]
struct SourceLensGroup {
    family: String,
    source_id: String,
    method: String,
    factors: Vec<String>,
}

impl<'a> GraphInspectorScreenWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for GraphInspectorScreenWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let day = self.app.date.day() as i32;
        let month = self.app.date.month() as i32;
        let year = self.app.date.year();

        let include_recommendations = self.app.show_graph_recommendations
            || (!self.app.dev_inspector_mode
                && matches!(
                    self.app.explanation_lens,
                    UserExplanationLens::HoatDong | UserExplanationLens::Nguon
                ));

        let inspection =
            amlich_core::debug_inspect_semantic_graph(day, month, year, include_recommendations);

        if !self.app.dev_inspector_mode {
            match self.app.explanation_lens {
                UserExplanationLens::ViSao => self.render_vi_sao_view(&inspection, area, buf),
                UserExplanationLens::YeuTo => {
                    self.render_causality_view(&inspection, area, buf, true)
                }
                UserExplanationLens::HoatDong => self.render_hoat_dong_view(&inspection, area, buf),
                UserExplanationLens::Nguon => self.render_nguon_view(&inspection, area, buf),
            }
            return;
        }

        match &self.app.graph_inspector_focus {
            GraphInspectorFocus::Summary => {
                self.render_summary_view(&inspection, area, buf);
            }
            GraphInspectorFocus::ClusterList => {
                self.render_cluster_list_view(&inspection, area, buf);
            }
            GraphInspectorFocus::ClusterNodes { cluster } => {
                self.render_cluster_nodes_view(&inspection, cluster, area, buf);
            }
            GraphInspectorFocus::NodeDetail { node_id } => {
                self.render_node_detail_view(&inspection, node_id, area, buf);
            }
            GraphInspectorFocus::NodeSubgraph { node_id } => {
                self.render_node_subgraph_view(&inspection, node_id, area, buf);
            }
            GraphInspectorFocus::NodeEdges { node_id } => {
                self.render_node_edges_view(&inspection, node_id, area, buf);
            }
            GraphInspectorFocus::Search => {
                self.render_search_view(&inspection, area, buf);
            }
            GraphInspectorFocus::ReasoningLens => {
                self.render_reasoning_lens_view(&inspection, area, buf);
            }
            GraphInspectorFocus::RecommendationLens => {
                self.render_recommendation_lens_view(&inspection, area, buf);
            }
            GraphInspectorFocus::ConvergenceLens => {
                self.render_convergence_lens_view(&inspection, area, buf);
            }
        }
    }
}

impl GraphInspectorScreenWidget<'_> {
    fn render_summary_view(
        &self,
        inspection: &amlich_core::DebugSemanticGraphInspection,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let rows = Layout::vertical([
            Constraint::Length(8),
            Constraint::Min(14),
            Constraint::Min(8),
        ])
        .split(area);

        render_header(
            inspection,
            self.app.show_graph_recommendations,
            rows[0],
            buf,
            true,
        );
        let overview = if self.mode == LayoutMode::Small {
            Layout::vertical([Constraint::Length(7), Constraint::Min(7)]).split(rows[1])
        } else {
            Layout::horizontal([Constraint::Percentage(38), Constraint::Percentage(62)])
                .split(rows[1])
        };
        render_summary(&inspection.summary, overview[0], buf);
        render_graph_preview(inspection, default_focal_node(inspection), overview[1], buf);

        let bottom = if self.mode == LayoutMode::Small {
            Layout::vertical([Constraint::Min(6), Constraint::Min(6)]).split(rows[2])
        } else {
            Layout::horizontal([Constraint::Percentage(42), Constraint::Percentage(58)])
                .split(rows[2])
        };
        render_cluster_counts(&inspection.cluster_counts, bottom[0], buf);
        if !inspection.severity_counts.is_empty() {
            render_severity_counts(&inspection.severity_counts, bottom[1], buf);
        } else {
            render_semantic_kind_counts(&inspection.semantic_kind_counts, bottom[1], buf);
        }
    }

    fn render_cluster_list_view(
        &self,
        inspection: &amlich_core::DebugSemanticGraphInspection,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let rows = Layout::vertical([Constraint::Length(4), Constraint::Min(4)]).split(area);

        render_header(
            inspection,
            self.app.show_graph_recommendations,
            rows[0],
            buf,
            false,
        );
        render_selectable_cluster_list(
            &inspection.cluster_counts,
            self.app.graph_inspector_cursor,
            rows[1],
            buf,
        );
    }

    fn render_cluster_nodes_view(
        &self,
        inspection: &amlich_core::DebugSemanticGraphInspection,
        cluster: &str,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let rows = Layout::vertical([Constraint::Length(4), Constraint::Min(4)]).split(area);

        let nodes: Vec<_> = inspection
            .visualization
            .nodes
            .iter()
            .filter(|n| n.cluster == cluster)
            .collect();

        render_header(
            inspection,
            self.app.show_graph_recommendations,
            rows[0],
            buf,
            false,
        );
        render_selectable_node_list(
            cluster,
            &nodes,
            self.app.graph_inspector_cursor,
            rows[1],
            buf,
        );
    }

    fn render_node_detail_view(
        &self,
        inspection: &amlich_core::DebugSemanticGraphInspection,
        node_id: &str,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let rows = Layout::vertical([Constraint::Length(4), Constraint::Min(4)]).split(area);

        render_header(
            inspection,
            self.app.show_graph_recommendations,
            rows[0],
            buf,
            false,
        );

        let node = inspection
            .visualization
            .nodes
            .iter()
            .find(|n| n.node_id == node_id);

        let mut connected_edges: Vec<_> = inspection
            .visualization
            .edges
            .iter()
            .filter(|e| e.from_id == node_id || e.to_id == node_id)
            .collect();
        connected_edges.sort_by(|a, b| {
            let a_key = if a.from_id == node_id {
                (&a.to_id, &a.label, &a.semantic_kind)
            } else {
                (&a.from_id, &a.label, &a.semantic_kind)
            };
            let b_key = if b.from_id == node_id {
                (&b.to_id, &b.label, &b.semantic_kind)
            } else {
                (&b.from_id, &b.label, &b.semantic_kind)
            };
            a_key.cmp(&b_key)
        });

        let detail_body = if rows[1].width < 100 || rows[1].height < 18 {
            Layout::vertical([Constraint::Percentage(54), Constraint::Percentage(46)])
                .split(rows[1])
        } else {
            Layout::horizontal([Constraint::Percentage(44), Constraint::Percentage(56)])
                .split(rows[1])
        };

        render_node_detail(node, &connected_edges, node_id, detail_body[0], buf);
        render_local_subgraph(
            node,
            &connected_edges,
            &inspection.visualization.nodes,
            detail_body[1],
            buf,
        );
    }

    fn render_node_edges_view(
        &self,
        inspection: &amlich_core::DebugSemanticGraphInspection,
        node_id: &str,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let rows = Layout::vertical([Constraint::Length(4), Constraint::Min(4)]).split(area);

        render_header(
            inspection,
            self.app.show_graph_recommendations,
            rows[0],
            buf,
            false,
        );

        let connected_edges = sorted_connected_edges(inspection, node_id);

        render_selectable_edge_list(
            node_id,
            &connected_edges,
            &inspection.visualization.nodes,
            self.app.graph_inspector_cursor,
            rows[1],
            buf,
        );
    }

    fn render_node_subgraph_view(
        &self,
        inspection: &amlich_core::DebugSemanticGraphInspection,
        node_id: &str,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let rows = Layout::vertical([Constraint::Length(4), Constraint::Min(8)]).split(area);

        render_header(
            inspection,
            self.app.show_graph_recommendations,
            rows[0],
            buf,
            false,
        );

        let node = inspection
            .visualization
            .nodes
            .iter()
            .find(|n| n.node_id == node_id);

        let connected_edges: Vec<_> = inspection
            .visualization
            .edges
            .iter()
            .filter(|e| e.from_id == node_id || e.to_id == node_id)
            .collect();

        render_local_subgraph(
            node,
            &connected_edges,
            &inspection.visualization.nodes,
            rows[1],
            buf,
        );
    }
    fn render_search_view(
        &self,
        inspection: &amlich_core::DebugSemanticGraphInspection,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let rows = Layout::vertical([Constraint::Length(3), Constraint::Min(4)]).split(area);

        render_search_header(&self.app.graph_inspector_search_query, rows[0], buf);

        let results = self.app.graph_inspector_search_results(inspection);
        render_search_result_list(
            &results,
            &self.app.graph_inspector_search_query,
            self.app.graph_inspector_search_cursor,
            rows[1],
            buf,
        );
    }

    fn render_reasoning_lens_view(
        &self,
        inspection: &amlich_core::DebugSemanticGraphInspection,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let rows = Layout::vertical([Constraint::Length(4), Constraint::Min(6)]).split(area);

        render_lens_header(
            "Lý Giải (Reasoning Lens)",
            self.app.graph_inspector_lens,
            inspection.date.day,
            inspection.date.month,
            inspection.date.year,
            rows[0],
            buf,
        );

        let entries = self.app.reasoning_lens_entries(inspection);
        render_reasoning_lens_entries(&entries, self.app.graph_inspector_cursor, rows[1], buf);
    }

    fn render_recommendation_lens_view(
        &self,
        inspection: &amlich_core::DebugSemanticGraphInspection,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let rows = Layout::vertical([Constraint::Length(4), Constraint::Min(6)]).split(area);

        render_lens_header(
            "Khuyến Nghị (Recommendation Lens)",
            self.app.graph_inspector_lens,
            inspection.date.day,
            inspection.date.month,
            inspection.date.year,
            rows[0],
            buf,
        );

        let entries = self.app.recommendation_lens_entries(inspection);
        render_recommendation_lens_entries(&entries, self.app.graph_inspector_cursor, rows[1], buf);
    }

    fn render_convergence_lens_view(
        &self,
        inspection: &amlich_core::DebugSemanticGraphInspection,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let rows = Layout::vertical([Constraint::Length(4), Constraint::Min(6)]).split(area);

        render_lens_header(
            "Hội Tụ (Convergence Lens)",
            self.app.graph_inspector_lens,
            inspection.date.day,
            inspection.date.month,
            inspection.date.year,
            rows[0],
            buf,
        );

        let entries = self.app.convergence_lens_entries(inspection);
        render_convergence_lens_entries(&entries, self.app.graph_inspector_cursor, rows[1], buf);
    }
}

fn render_search_header(query: &str, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Tìm Kiếm (Search) ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));
    let inner = block.inner(area);
    block.render(area, buf);

    let cursor_char = "▎";
    let display = format!(
        "  Tìm: {}{}  (Esc: quay lại  Enter: chọn  ↑↓: di chuyển)",
        query, cursor_char
    );
    let lines = vec![Line::from(vec![Span::styled(
        &display,
        Style::default().fg(Color::Yellow),
    )])];

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_search_result_list(
    results: &[amlich_core::semantic_graph::VisualizationNode],
    query: &str,
    cursor: usize,
    area: Rect,
    buf: &mut Buffer,
) {
    let title = if query.is_empty() {
        " Kết Quả — Gõ từ khóa để tìm node ".to_string()
    } else {
        format!(
            " Kết Quả: {} kết quả cho '{}' ",
            results.len(),
            truncate_label(query, 30)
        )
    };
    let block = Block::default()
        .title(title.as_str())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));
    let inner = block.inner(area);
    block.render(area, buf);

    let mut lines = Vec::new();

    if query.is_empty() {
        lines.push(Line::from(Span::styled(
            "  Gõ từ khóa để tìm theo node_id, label, cluster, hoặc semantic_kind.",
            Style::default().fg(Color::DarkGray),
        )));
    } else if results.is_empty() {
        lines.push(Line::from(Span::styled(
            format!("  Không tìm thấy kết quả cho '{}'.", query),
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        for (idx, node) in results.iter().enumerate() {
            let selected = idx == cursor;
            let marker = if selected { ">" } else { " " };
            let style = if selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let kind_style = if selected {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            let cluster_style = if selected {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            lines.push(Line::from(vec![
                Span::styled(format!("{} ", marker), style),
                Span::styled(format!("{:<24}", truncate_label(&node.node_id, 24)), style),
                Span::styled(format!("{:<18}", truncate_label(&node.label, 18)), style),
                Span::styled(
                    format!("{:<16}", truncate_label(&node.cluster, 16)),
                    cluster_style,
                ),
                Span::styled(truncate_label(&node.semantic_kind, 14), kind_style),
            ]));
        }
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_header(
    inspection: &amlich_core::DebugSemanticGraphInspection,
    include_recs: bool,
    area: Rect,
    buf: &mut Buffer,
    is_summary: bool,
) {
    let block = Block::default()
        .title(" Đồ Thị Ngữ Nghĩa (Semantic Graph Inspector) ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    let inner = block.inner(area);
    block.render(area, buf);

    let rec_status = if include_recs {
        ("BẬT", Color::Green)
    } else {
        ("TẮT", Color::DarkGray)
    };

    let help_line = if is_summary {
        "←/→: đổi ngày  t: hôm nay  r: toggle recs  Enter/l: drill-down  preview graph bên dưới  /: tìm kiếm  1-6: đổi màn"
    } else {
        "↑/k ↓/j: chọn  Enter/l: vào  Esc/h/Backspace: quay lại  r: toggle recs  /: tìm kiếm"
    };

    let lines = vec![
        Line::from(vec![
            Span::styled("  Ngày: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!(
                    "{}/{}/{}",
                    inspection.date.day, inspection.date.month, inspection.date.year
                ),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("   "),
            Span::styled("Surface: ", Style::default().fg(Color::DarkGray)),
            Span::styled(&inspection.surface, Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  Chứng cứ khuyến nghị: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                rec_status.0,
                Style::default()
                    .fg(rec_status.1)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("   "),
            Span::styled("[r] để bật/tắt", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![Span::styled(
            format!("  {}", help_line),
            Style::default().fg(Color::DarkGray),
        )]),
    ];

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_summary(summary: &amlich_core::DebugInspectionSummary, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Tổng Quan ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let rec_label = if summary.has_recommendation_evidence {
        ("Có", Color::Green)
    } else {
        ("Không", Color::DarkGray)
    };

    let mut lines = vec![
        Line::from(vec![
            Span::styled("  Tổng nodes: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                summary.total_nodes.to_string(),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("    "),
            Span::styled("Tổng edges: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                summary.total_edges.to_string(),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Clusters: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                summary.clusters.len().to_string(),
                Style::default().fg(Color::White),
            ),
            Span::raw("    "),
            Span::styled("Semantic kinds: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                summary.semantic_kinds.len().to_string(),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Chứng cứ khuyến nghị: ", Style::default().fg(Color::Cyan)),
            Span::styled(rec_label.0, Style::default().fg(rec_label.1)),
        ]),
    ];

    if !summary.clusters.is_empty() {
        let cluster_list = summary.clusters.join(", ");
        lines.push(Line::from(vec![
            Span::styled("  Clusters: ", Style::default().fg(Color::DarkGray)),
            Span::styled(cluster_list, Style::default().fg(Color::DarkGray)),
        ]));
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_cluster_counts(counts: &HashMap<String, usize>, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Cluster Counts ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let mut entries: Vec<_> = counts.iter().collect();
    entries.sort_by(|a, b| b.1.cmp(a.1));

    let mut lines = Vec::new();
    for (cluster, count) in entries.iter().take(12) {
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(
                format!("{:<30}", truncate_label(cluster, 30)),
                Style::default().fg(Color::White),
            ),
            Span::styled(format!("{:>5}", count), Style::default().fg(Color::Yellow)),
        ]));
    }
    if entries.len() > 12 {
        lines.push(Line::from(vec![Span::styled(
            format!("  ... +{} more", entries.len() - 12),
            Style::default().fg(Color::DarkGray),
        )]));
    }

    if lines.is_empty() {
        lines.push(Line::from(Span::styled(
            "  Không có cluster nào.",
            Style::default().fg(Color::DarkGray),
        )));
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_selectable_cluster_list(
    counts: &HashMap<String, usize>,
    cursor: usize,
    area: Rect,
    buf: &mut Buffer,
) {
    let block = Block::default()
        .title(" Chọn Cluster (Enter để xem nodes) ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    let inner = block.inner(area);
    block.render(area, buf);

    let mut entries: Vec<_> = counts.iter().collect();
    entries.sort_by(|a, b| b.1.cmp(a.1));

    let mut lines = Vec::new();
    for (idx, (cluster, count)) in entries.iter().enumerate() {
        let selected = idx == cursor;
        let marker = if selected { ">" } else { " " };
        let style = if selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        let count_style = if selected {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Yellow)
        };
        lines.push(Line::from(vec![
            Span::styled(format!("{} ", marker), style),
            Span::styled(format!("{:<36}", truncate_label(cluster, 36)), style),
            Span::styled(format!("{:>5}", count), count_style),
        ]));
    }

    if lines.is_empty() {
        lines.push(Line::from(Span::styled(
            "  Không có cluster nào.",
            Style::default().fg(Color::DarkGray),
        )));
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_selectable_node_list(
    cluster: &str,
    nodes: &[&amlich_core::semantic_graph::VisualizationNode],
    cursor: usize,
    area: Rect,
    buf: &mut Buffer,
) {
    let title = format!(" Cluster: {} — Nodes (Enter để xem chi tiết) ", cluster);
    let block = Block::default()
        .title(title.as_str())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    let inner = block.inner(area);
    block.render(area, buf);

    let mut lines = Vec::new();
    for (idx, node) in nodes.iter().enumerate() {
        let selected = idx == cursor;
        let marker = if selected { ">" } else { " " };
        let style = if selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        let kind_style = if selected {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        lines.push(Line::from(vec![
            Span::styled(format!("{} ", marker), style),
            Span::styled(format!("{:<28}", truncate_label(&node.node_id, 28)), style),
            Span::styled(format!("{:<20}", truncate_label(&node.label, 20)), style),
            Span::styled(truncate_label(&node.semantic_kind, 14), kind_style),
        ]));
    }

    if lines.is_empty() {
        lines.push(Line::from(Span::styled(
            "  Không có node nào trong cluster này.",
            Style::default().fg(Color::DarkGray),
        )));
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_node_detail(
    node: Option<&amlich_core::semantic_graph::VisualizationNode>,
    connected_edges: &[&amlich_core::semantic_graph::VisualizationEdge],
    node_id: &str,
    area: Rect,
    buf: &mut Buffer,
) {
    let title = format!(" Node Detail: {} ", truncate_label(node_id, 30));
    let block = Block::default()
        .title(title.as_str())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    let inner = block.inner(area);
    block.render(area, buf);

    let mut lines = Vec::new();

    if let Some(n) = node {
        lines.push(Line::from(vec![
            Span::styled("  node_id:       ", Style::default().fg(Color::Cyan)),
            Span::styled(&n.node_id, Style::default().fg(Color::White)),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  label:         ", Style::default().fg(Color::Cyan)),
            Span::styled(&n.label, Style::default().fg(Color::White)),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  cluster:       ", Style::default().fg(Color::Cyan)),
            Span::styled(&n.cluster, Style::default().fg(Color::White)),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  semantic_kind: ", Style::default().fg(Color::Cyan)),
            Span::styled(&n.semantic_kind, Style::default().fg(Color::White)),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  severity:      ", Style::default().fg(Color::Cyan)),
            Span::styled(
                n.severity.as_deref().unwrap_or("(none)"),
                match n.severity.as_deref() {
                    Some("critical") => Style::default().fg(Color::Red),
                    Some("caution") => Style::default().fg(Color::Yellow),
                    Some("favorable") | Some("positive") => Style::default().fg(Color::Green),
                    Some("neutral") => Style::default().fg(Color::White),
                    _ => Style::default().fg(Color::DarkGray),
                },
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  shape_hint:    ", Style::default().fg(Color::Cyan)),
            Span::styled(
                n.shape_hint.as_deref().unwrap_or("(none)"),
                Style::default().fg(Color::DarkGray),
            ),
        ]));
    } else {
        lines.push(Line::from(vec![
            Span::styled("  Node không tìm thấy: ", Style::default().fg(Color::Red)),
            Span::styled(node_id, Style::default().fg(Color::White)),
        ]));
    }

    lines.push(Line::from(""));

    let edge_count = connected_edges.len();
    lines.push(Line::from(vec![
        Span::styled("  Connected edges: ", Style::default().fg(Color::Cyan)),
        Span::styled(
            edge_count.to_string(),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
    ]));

    for edge in connected_edges.iter().take(10) {
        let direction = if edge.from_id == node_id {
            "→"
        } else {
            "←"
        };
        let other = if edge.from_id == node_id {
            &edge.to_id
        } else {
            &edge.from_id
        };
        lines.push(Line::from(vec![
            Span::styled("    ", Style::default()),
            Span::styled(direction, Style::default().fg(Color::Cyan)),
            Span::raw(" "),
            Span::styled(
                format!("{:<24}", truncate_label(other, 24)),
                Style::default().fg(Color::White),
            ),
            Span::styled(
                format!(" [{:<14}]", truncate_label(&edge.semantic_kind, 14)),
                Style::default().fg(Color::DarkGray),
            ),
        ]));
    }
    if edge_count > 10 {
        lines.push(Line::from(vec![Span::styled(
            format!("    ... +{} more edges", edge_count - 10),
            Style::default().fg(Color::DarkGray),
        )]));
    }

    if edge_count > 0 {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![Span::styled(
            "  Enter/l: xem chi tiết edges / nhảy sang node khác",
            Style::default().fg(Color::DarkGray),
        )]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![Span::styled(
        "  Provenance:",
        Style::default().fg(Color::Cyan),
    )]));

    match node {
        Some(n) if !n.provenance.is_empty() => {
            for (idx, entry) in n.provenance.iter().enumerate() {
                lines.push(Line::from(vec![
                    Span::styled(
                        format!("    [{}] family: ", idx + 1),
                        Style::default().fg(Color::DarkGray),
                    ),
                    Span::styled(
                        provenance_source_family_label(entry),
                        Style::default().fg(Color::White),
                    ),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("        source_id: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(&entry.source_id, Style::default().fg(Color::White)),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("        method:    ", Style::default().fg(Color::DarkGray)),
                    Span::styled(&entry.method, Style::default().fg(Color::White)),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("        note:      ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        entry.note.as_deref().unwrap_or("(none)"),
                        if entry.note.is_some() {
                            Style::default().fg(Color::White)
                        } else {
                            Style::default().fg(Color::DarkGray)
                        },
                    ),
                ]));
            }
        }
        Some(_) => {
            lines.push(Line::from(vec![Span::styled(
                "    Không có provenance cho node này.",
                Style::default().fg(Color::DarkGray),
            )]));
        }
        None => {
            lines.push(Line::from(vec![Span::styled(
                "    Không có provenance vì node không tồn tại.",
                Style::default().fg(Color::DarkGray),
            )]));
        }
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_selectable_edge_list(
    node_id: &str,
    connected_edges: &[&amlich_core::semantic_graph::VisualizationEdge],
    all_nodes: &[amlich_core::semantic_graph::VisualizationNode],
    cursor: usize,
    area: Rect,
    buf: &mut Buffer,
) {
    let title = format!(
        " Edges: {} (Enter để nhảy sang neighbor) ",
        truncate_label(node_id, 24)
    );
    let block = Block::default()
        .title(title.as_str())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    let inner = block.inner(area);
    block.render(area, buf);

    let mut lines = Vec::new();

    for (idx, edge) in connected_edges.iter().enumerate() {
        let is_outgoing = edge.from_id == node_id;
        let direction = if is_outgoing { "→ out" } else { "← in " };
        let other_id = if is_outgoing {
            &edge.to_id
        } else {
            &edge.from_id
        };
        let other_label = all_nodes
            .iter()
            .find(|n| &n.node_id == other_id)
            .map(|n| n.label.as_str())
            .unwrap_or("?");

        let selected = idx == cursor;
        let style = if selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        let dir_style = if selected {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else if is_outgoing {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::Magenta)
        };
        let detail_style = if selected {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let marker = if selected { ">" } else { " " };

        lines.push(Line::from(vec![
            Span::styled(format!("{} ", marker), style),
            Span::styled(direction, dir_style),
            Span::raw(" "),
            Span::styled(format!("{:<24}", truncate_label(other_id, 24)), style),
        ]));

        lines.push(Line::from(vec![
            Span::raw("     "),
            Span::styled(
                format!("label: {}", truncate_label(&edge.label, 30)),
                detail_style,
            ),
        ]));

        lines.push(Line::from(vec![
            Span::raw("     "),
            Span::styled(
                format!(
                    "kind: {}  weight: {}  other: {}",
                    truncate_label(&edge.semantic_kind, 18),
                    edge.weight,
                    truncate_label(other_label, 24)
                ),
                detail_style,
            ),
        ]));

        if selected {
            lines.push(Line::from(""));
        }
    }

    if lines.is_empty() {
        lines.push(Line::from(Span::styled(
            "  Không có edge nào kết nối node này.",
            Style::default().fg(Color::DarkGray),
        )));
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_semantic_kind_counts(counts: &HashMap<String, usize>, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Semantic Kind Counts ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let mut entries: Vec<_> = counts.iter().collect();
    entries.sort_by(|a, b| b.1.cmp(a.1));

    let mut lines = Vec::new();
    for (kind, count) in entries.iter().take(12) {
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(
                format!("{:<30}", truncate_label(kind, 30)),
                Style::default().fg(Color::White),
            ),
            Span::styled(format!("{:>5}", count), Style::default().fg(Color::Yellow)),
        ]));
    }
    if entries.len() > 12 {
        lines.push(Line::from(vec![Span::styled(
            format!("  ... +{} more", entries.len() - 12),
            Style::default().fg(Color::DarkGray),
        )]));
    }

    if lines.is_empty() {
        lines.push(Line::from(Span::styled(
            "  Không có semantic kind nào.",
            Style::default().fg(Color::DarkGray),
        )));
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_graph_preview(
    inspection: &amlich_core::DebugSemanticGraphInspection,
    focal_node: Option<&amlich_core::semantic_graph::VisualizationNode>,
    area: Rect,
    buf: &mut Buffer,
) {
    let block = Block::default()
        .title(" Graph Preview ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(node) = focal_node else {
        Paragraph::new(vec![Line::from(Span::styled(
            "  Không có node để preview.",
            Style::default().fg(Color::DarkGray),
        ))])
        .wrap(Wrap { trim: true })
        .render(inner, buf);
        return;
    };

    let connected_edges = sorted_connected_edges(inspection, &node.node_id);
    let header_height = inner.height.min(3);
    let header_area = Rect {
        x: inner.x,
        y: inner.y,
        width: inner.width,
        height: header_height,
    };
    Paragraph::new(vec![
        Line::from(vec![
            Span::styled("  Focal: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                truncate_label(&node.label, 28),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(
                truncate_label(&node.node_id, 22),
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        Line::from(vec![Span::styled(
            "  Đây là preview cục bộ của node nổi bật nhất trong graph hiện tại.",
            Style::default().fg(Color::DarkGray),
        )]),
    ])
    .wrap(Wrap { trim: true })
    .render(header_area, buf);

    let graph_area = Rect {
        x: inner.x,
        y: inner.y.saturating_add(header_height),
        width: inner.width,
        height: inner.height.saturating_sub(header_height),
    };
    if graph_area.height > 0 {
        render_local_subgraph(
            Some(node),
            &connected_edges,
            &inspection.visualization.nodes,
            graph_area,
            buf,
        );
    }
}

fn render_local_subgraph(
    node: Option<&amlich_core::semantic_graph::VisualizationNode>,
    connected_edges: &[&amlich_core::semantic_graph::VisualizationEdge],
    all_nodes: &[amlich_core::semantic_graph::VisualizationNode],
    area: Rect,
    buf: &mut Buffer,
) {
    let title = format!(
        " Local Subgraph: {} ",
        truncate_label(node.map(|n| n.node_id.as_str()).unwrap_or("(missing)"), 28)
    );
    let block = Block::default()
        .title(title.as_str())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(node) = node else {
        Paragraph::new(vec![Line::from(Span::styled(
            "  Node không tìm thấy nên không thể dựng local subgraph.",
            Style::default().fg(Color::Red),
        ))])
        .wrap(Wrap { trim: true })
        .render(inner, buf);
        return;
    };

    let max_neighbors = if inner.width >= 110 {
        4
    } else if inner.width >= 84 {
        3
    } else {
        2
    };

    let mut incoming = Vec::new();
    let mut outgoing = Vec::new();
    let mut truncated_incoming = 0usize;
    let mut truncated_outgoing = 0usize;

    for edge in connected_edges {
        let (is_outgoing, neighbor_id) = if edge.from_id == node.node_id {
            (true, edge.to_id.as_str())
        } else {
            (false, edge.from_id.as_str())
        };
        let neighbor = all_nodes
            .iter()
            .find(|candidate| candidate.node_id == neighbor_id);
        let entry = LocalNeighborEntry {
            edge,
            neighbor_id,
            neighbor,
            semantic_marker: semantic_marker(
                neighbor.map(|item| item.semantic_kind.as_str()),
                neighbor.and_then(|item| item.shape_hint.as_deref()),
                neighbor.and_then(|item| item.severity.as_deref()),
            ),
        };

        if is_outgoing {
            if outgoing.len() < max_neighbors {
                outgoing.push(entry);
            } else {
                truncated_outgoing += 1;
            }
        } else if incoming.len() < max_neighbors {
            incoming.push(entry);
        } else {
            truncated_incoming += 1;
        }
    }

    let compact = inner.width < 92;
    let lines = if compact {
        render_compact_local_subgraph_lines(
            node,
            &incoming,
            &outgoing,
            truncated_incoming,
            truncated_outgoing,
        )
    } else {
        render_column_local_subgraph_lines(
            node,
            &incoming,
            &outgoing,
            truncated_incoming,
            truncated_outgoing,
            inner.width as usize,
        )
    };

    Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .render(inner, buf);
}

#[derive(Clone, Copy)]
struct LocalNeighborEntry<'a> {
    edge: &'a amlich_core::semantic_graph::VisualizationEdge,
    neighbor_id: &'a str,
    neighbor: Option<&'a amlich_core::semantic_graph::VisualizationNode>,
    semantic_marker: &'static str,
}

fn render_compact_local_subgraph_lines(
    node: &amlich_core::semantic_graph::VisualizationNode,
    incoming: &[LocalNeighborEntry<'_>],
    outgoing: &[LocalNeighborEntry<'_>],
    truncated_incoming: usize,
    truncated_outgoing: usize,
) -> Vec<Line<'static>> {
    let mut lines = vec![
        Line::from(vec![Span::styled(
            format!(
                "  ┏━ FOCAL {} ━┓  {}",
                semantic_marker(
                    Some(node.semantic_kind.as_str()),
                    node.shape_hint.as_deref(),
                    node.severity.as_deref()
                ),
                truncate_label(&node.label, 42)
            ),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            format!(
                "  {}  {}  {}",
                truncate_label(&node.node_id, 24),
                truncate_label(&node.semantic_kind, 18),
                truncate_label(&node.cluster, 16)
            ),
            Style::default().fg(Color::DarkGray),
        )]),
        Line::from(""),
    ];

    lines.push(Line::from(vec![Span::styled(
        "  ← Incoming",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    )]));
    push_neighbor_list(&mut lines, incoming, false);
    if truncated_incoming > 0 {
        lines.push(Line::from(vec![Span::styled(
            format!("    ... +{} incoming edges", truncated_incoming),
            Style::default().fg(Color::DarkGray),
        )]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![Span::styled(
        "  Outgoing →",
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    )]));
    push_neighbor_list(&mut lines, outgoing, true);
    if truncated_outgoing > 0 {
        lines.push(Line::from(vec![Span::styled(
            format!("    ... +{} outgoing edges", truncated_outgoing),
            Style::default().fg(Color::DarkGray),
        )]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![Span::styled(
        "  Enter/l: xem edge list  Esc/h: quay lại node detail",
        Style::default().fg(Color::DarkGray),
    )]));
    lines
}

fn render_column_local_subgraph_lines(
    node: &amlich_core::semantic_graph::VisualizationNode,
    incoming: &[LocalNeighborEntry<'_>],
    outgoing: &[LocalNeighborEntry<'_>],
    truncated_incoming: usize,
    truncated_outgoing: usize,
    width: usize,
) -> Vec<Line<'static>> {
    let center_width = 28usize.min(width.saturating_sub(24));
    let side_width = width.saturating_sub(center_width + 6) / 2;
    let row_count = incoming.len().max(outgoing.len()).max(1);
    let mut lines = vec![
        Line::from(vec![Span::styled(
            format!(
                "  Legend: incoming {:<10}  [FOCAL {}] {:<18}  outgoing {:<10}",
                "←",
                semantic_marker(
                    Some(node.semantic_kind.as_str()),
                    node.shape_hint.as_deref(),
                    node.severity.as_deref()
                ),
                truncate_label(&node.label, 18),
                "→"
            ),
            Style::default().fg(Color::DarkGray),
        )]),
        Line::from(""),
    ];

    for row in 0..row_count {
        let left = incoming
            .get(row)
            .map(|entry| format_neighbor_cell(entry, false, side_width))
            .unwrap_or_else(|| " ".repeat(side_width));
        let center = if row == row_count / 2 {
            format!(
                "[FOCAL {}] {:<width$}",
                semantic_marker(
                    Some(node.semantic_kind.as_str()),
                    node.shape_hint.as_deref(),
                    node.severity.as_deref()
                ),
                truncate_label(&node.label, center_width.saturating_sub(11)),
                width = center_width
            )
        } else if row == (row_count / 2).saturating_add(1) {
            format!(
                "{:<width$}",
                format!(
                    "{}  {}",
                    truncate_label(&node.node_id, center_width / 2),
                    truncate_label(&node.semantic_kind, center_width / 2)
                ),
                width = center_width
            )
        } else {
            " ".repeat(center_width)
        };
        let right = outgoing
            .get(row)
            .map(|entry| format_neighbor_cell(entry, true, side_width))
            .unwrap_or_else(|| " ".repeat(side_width));
        let connector_left = if incoming.get(row).is_some() {
            " ==> "
        } else {
            "     "
        };
        let connector_right = if outgoing.get(row).is_some() {
            " ==> "
        } else {
            "     "
        };

        lines.push(Line::from(
            left + connector_left + &center + connector_right + &right,
        ));
    }

    if truncated_incoming > 0 || truncated_outgoing > 0 {
        lines.push(Line::from(""));
        let mut extras = Vec::new();
        if truncated_incoming > 0 {
            extras.push(format!("+{} incoming", truncated_incoming));
        }
        if truncated_outgoing > 0 {
            extras.push(format!("+{} outgoing", truncated_outgoing));
        }
        lines.push(Line::from(vec![Span::styled(
            format!("  Hidden for stability: {}", extras.join("  ")),
            Style::default().fg(Color::DarkGray),
        )]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![Span::styled(
        "  Edge markers use neighbor kind/shape/severity. Enter/l opens the full edge list.",
        Style::default().fg(Color::DarkGray),
    )]));
    lines
}

fn push_neighbor_list(
    lines: &mut Vec<Line<'static>>,
    entries: &[LocalNeighborEntry<'_>],
    is_outgoing: bool,
) {
    if entries.is_empty() {
        lines.push(Line::from(vec![Span::styled(
            "    (none)",
            Style::default().fg(Color::DarkGray),
        )]));
        return;
    }

    for entry in entries {
        let neighbor_label = entry
            .neighbor
            .map(|item| truncate_label(&item.label, 26))
            .unwrap_or_else(|| truncate_label(entry.neighbor_id, 26));
        let dir = if is_outgoing { "->" } else { "<-" };
        lines.push(Line::from(vec![
            Span::styled(
                format!("    [{}] {} {}", entry.semantic_marker, dir, neighbor_label),
                Style::default().fg(Color::White),
            ),
            Span::styled(
                format!("  <{}>", truncate_label(&entry.edge.label, 18)),
                Style::default().fg(if is_outgoing {
                    Color::Green
                } else {
                    Color::Magenta
                }),
            ),
        ]));
        lines.push(Line::from(vec![Span::styled(
            format!(
                "       {}  {}",
                truncate_label(entry.neighbor_id, 24),
                truncate_label(&entry.edge.semantic_kind, 16)
            ),
            Style::default().fg(Color::DarkGray),
        )]));
    }
}

fn format_neighbor_cell(entry: &LocalNeighborEntry<'_>, is_outgoing: bool, width: usize) -> String {
    let arrow = if is_outgoing { ">>" } else { "<<" };
    let label = entry
        .neighbor
        .map(|item| truncate_label(&item.label, width.saturating_sub(14)))
        .unwrap_or_else(|| truncate_label(entry.neighbor_id, width.saturating_sub(14)));
    let edge = truncate_label(&entry.edge.label, 12);
    format!(
        "{:<width$}",
        format!("[{} {}] {} {}", arrow, entry.semantic_marker, label, edge),
        width = width
    )
}

fn semantic_marker(
    semantic_kind: Option<&str>,
    shape_hint: Option<&str>,
    severity: Option<&str>,
) -> &'static str {
    match severity {
        Some("critical") => "!!",
        Some("caution") => "!",
        Some("favorable") | Some("positive") => "+",
        _ => match shape_hint {
            Some("diamond") => "<>",
            Some("hexagon") => "{6}",
            Some("ellipse") => "()",
            Some("box") => "[]",
            _ => match semantic_kind.unwrap_or_default() {
                kind if kind.contains("recommendation") => "<>",
                kind if kind.contains("activity") => "[]",
                kind if kind.contains("relation") || kind.contains("signal") => "()",
                _ => "..",
            },
        },
    }
}

fn default_focal_node(
    inspection: &amlich_core::DebugSemanticGraphInspection,
) -> Option<&amlich_core::semantic_graph::VisualizationNode> {
    inspection.visualization.nodes.iter().max_by(|a, b| {
        let a_degree = inspection
            .visualization
            .edges
            .iter()
            .filter(|edge| edge.from_id == a.node_id || edge.to_id == a.node_id)
            .count();
        let b_degree = inspection
            .visualization
            .edges
            .iter()
            .filter(|edge| edge.from_id == b.node_id || edge.to_id == b.node_id)
            .count();
        a_degree.cmp(&b_degree).then_with(|| a.label.cmp(&b.label))
    })
}

fn sorted_connected_edges<'a>(
    inspection: &'a amlich_core::DebugSemanticGraphInspection,
    node_id: &str,
) -> Vec<&'a amlich_core::semantic_graph::VisualizationEdge> {
    let mut connected_edges: Vec<_> = inspection
        .visualization
        .edges
        .iter()
        .filter(|e| e.from_id == node_id || e.to_id == node_id)
        .collect();
    connected_edges.sort_by(|a, b| {
        let a_key = if a.from_id == node_id {
            (&a.to_id, &a.label, &a.semantic_kind)
        } else {
            (&a.from_id, &a.label, &a.semantic_kind)
        };
        let b_key = if b.from_id == node_id {
            (&b.to_id, &b.label, &b.semantic_kind)
        } else {
            (&b.from_id, &b.label, &b.semantic_kind)
        };
        a_key.cmp(&b_key)
    });
    connected_edges
}

fn render_severity_counts(counts: &HashMap<String, usize>, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Severity Counts ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let mut entries: Vec<_> = counts.iter().collect();
    entries.sort_by(|a, b| b.1.cmp(a.1));

    let mut lines = Vec::new();
    for (severity, count) in entries.iter() {
        let color = match severity.as_str() {
            "critical" => Color::Red,
            "caution" => Color::Yellow,
            "favorable" | "positive" => Color::Green,
            "neutral" => Color::White,
            _ => Color::DarkGray,
        };
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(
                format!("{:<20}", truncate_label(severity, 20)),
                Style::default().fg(color),
            ),
            Span::styled(format!("{:>5}", count), Style::default().fg(Color::Yellow)),
        ]));
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn truncate_label(s: &str, max_len: usize) -> String {
    let char_count = s.chars().count();
    if char_count <= max_len {
        s.to_string()
    } else {
        let trimmed: String = s.chars().take(max_len.saturating_sub(1)).collect();
        format!("{trimmed}…")
    }
}

fn render_lens_header(
    title: &str,
    lens: crate::state::GraphInspectorLens,
    day: i32,
    month: i32,
    year: i32,
    area: Rect,
    buf: &mut Buffer,
) {
    let block = Block::default()
        .title(format!(" {} ", title))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta));
    let inner = block.inner(area);
    block.render(area, buf);

    let lens_label = lens.label();
    let help_line =
        "/?: đổi lens  Tab: đổi màn  ←/→: đổi ngày  t: hôm nay  ↑↓: chọn  Esc: quay lại";

    let lines = vec![
        Line::from(vec![
            Span::styled("  Ngày: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{}/{}/{}", day, month, year),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("   "),
            Span::styled("Lens: ", Style::default().fg(Color::Magenta)),
            Span::styled(lens_label, Style::default().fg(Color::White)),
        ]),
        Line::from(vec![Span::styled(
            format!("  {}", help_line),
            Style::default().fg(Color::DarkGray),
        )]),
    ];

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_reasoning_lens_entries(
    entries: &[crate::state::ReasoningLensEntry],
    cursor: usize,
    area: Rect,
    buf: &mut Buffer,
) {
    let block = Block::default()
        .title(" Reasoning Slice ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    let inner = block.inner(area);
    block.render(area, buf);

    let mut lines = Vec::new();

    if entries.is_empty() {
        lines.push(Line::from(Span::styled(
            "  Không có reasoning slice cho ngày này.",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        for (idx, entry) in entries.iter().enumerate() {
            let selected = idx == cursor;
            let marker = if selected { ">" } else { " " };
            let style = if selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let kind_color = match entry.kind.as_str() {
                "support" => Color::Green,
                "resistance" => Color::Red,
                "override" => Color::Magenta,
                "conflict" => Color::Yellow,
                "refinement" => Color::Cyan,
                _ => Color::White,
            };

            lines.push(Line::from(vec![
                Span::styled(format!("{} ", marker), style),
                Span::styled(
                    format!("{:<12}", truncate_label(&entry.kind, 12)),
                    Style::default().fg(kind_color),
                ),
                Span::styled(format!("{:<36}", truncate_label(&entry.label, 36)), style),
            ]));

            if selected && !entry.provenance.is_empty() {
                for prov in entry.provenance.iter().take(3) {
                    lines.push(Line::from(vec![
                        Span::raw("     "),
                        Span::styled(
                            format!("[{:?}] {}", prov.source_family, prov.source_id),
                            Style::default().fg(Color::DarkGray),
                        ),
                    ]));
                }
            }
        }
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_recommendation_lens_entries(
    entries: &[crate::state::RecommendationLensEntry],
    cursor: usize,
    area: Rect,
    buf: &mut Buffer,
) {
    let block = Block::default()
        .title(" Recommendation Slice ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    let inner = block.inner(area);
    block.render(area, buf);

    let mut lines = Vec::new();

    if entries.is_empty() {
        lines.push(Line::from(Span::styled(
            "  Không có recommendation slice cho ngày này.",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        for (idx, entry) in entries.iter().enumerate() {
            let selected = idx == cursor;
            let marker = if selected { ">" } else { " " };
            let style = if selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let direction_color = match entry.verdict {
                RecommendationLensVerdict::Nen => Color::Green,
                RecommendationLensVerdict::CoThe => Color::Yellow,
                RecommendationLensVerdict::Tranh | RecommendationLensVerdict::KyManh => Color::Red,
            };
            let hard_stop_marker = if entry.verdict == RecommendationLensVerdict::KyManh {
                " [KỴ MẠNH]"
            } else {
                ""
            };

            lines.push(Line::from(vec![
                Span::styled(format!("{} ", marker), style),
                Span::styled(entry.verdict.label(), Style::default().fg(direction_color)),
                Span::raw(" "),
                Span::styled(
                    format!(
                        "{}{}",
                        truncate_label(&entry.activity, 20),
                        hard_stop_marker
                    ),
                    style,
                ),
            ]));

            if selected {
                lines.push(Line::from(vec![
                    Span::raw("     "),
                    Span::styled(
                        format!("{} ", truncate_label(&entry.headline_reason, 40)),
                        Style::default().fg(Color::DarkGray),
                    ),
                ]));

                lines.push(Line::from(vec![
                    Span::raw("     "),
                    Span::styled(
                        format!("source: {} ", entry.source),
                        Style::default().fg(Color::DarkGray),
                    ),
                ]));

                if !entry.provenance.is_empty() {
                    lines.push(Line::from(vec![Span::styled(
                        "     provenance:",
                        Style::default().fg(Color::Cyan),
                    )]));
                    for prov in entry.provenance.iter().take(2) {
                        lines.push(Line::from(vec![
                            Span::raw("       "),
                            Span::styled(
                                format!("[{:?}] {}", prov.source_family, prov.source_id),
                                Style::default().fg(Color::DarkGray),
                            ),
                        ]));
                    }
                }
            }
        }
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_convergence_lens_entries(
    entries: &[crate::state::ConvergenceLensEntry],
    cursor: usize,
    area: Rect,
    buf: &mut Buffer,
) {
    let block = Block::default()
        .title(" Convergence Slice ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    let inner = block.inner(area);
    block.render(area, buf);

    let mut lines = Vec::new();

    if entries.is_empty() {
        lines.push(Line::from(Span::styled(
            "  Không có convergence slice cho ngày này.",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        for (idx, entry) in entries.iter().enumerate() {
            let selected = idx == cursor;
            let marker = if selected { ">" } else { " " };
            let style = if selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let kind_color = match entry.kind.as_str() {
                "shared_fact" => Color::Magenta,
                "rec_hit" => Color::Cyan,
                _ => Color::White,
            };

            lines.push(Line::from(vec![
                Span::styled(format!("{} ", marker), style),
                Span::styled(
                    format!("{:<14}", truncate_label(&entry.kind, 14)),
                    Style::default().fg(kind_color),
                ),
                Span::styled(format!("{:<32}", truncate_label(&entry.label, 32)), style),
                if !entry.activity.is_empty() {
                    Span::styled(
                        format!(" → {}", truncate_label(&entry.activity, 16)),
                        Style::default().fg(Color::DarkGray),
                    )
                } else {
                    Span::raw("")
                },
            ]));

            if selected && !entry.provenance.is_empty() {
                lines.push(Line::from(vec![Span::styled(
                    "     provenance:",
                    Style::default().fg(Color::Cyan),
                )]));
                for prov in entry.provenance.iter().take(3) {
                    lines.push(Line::from(vec![
                        Span::raw("       "),
                        Span::styled(
                            format!("[{:?}] {}", prov.source_family, prov.source_id),
                            Style::default().fg(Color::DarkGray),
                        ),
                    ]));
                }
            }
        }
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn provenance_source_family_label(entry: &amlich_core::ReasoningEvidenceEnvelope) -> &'static str {
    use amlich_core::ReasoningEvidenceSourceFamily as Family;

    match entry.source_family {
        Family::Snapshot => "snapshot",
        Family::Interaction => "interaction",
        Family::Bazi => "bazi",
        Family::Axis => "axis",
        Family::AlmanacRule => "almanac_rule",
        Family::Insight => "insight",
        Family::Derived => "derived",
        Family::IChing => "iching",
    }
}

impl GraphInspectorScreenWidget<'_> {
    fn render_vi_sao_view(
        &self,
        inspection: &amlich_core::DebugSemanticGraphInspection,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let block = Block::default()
            .title(" Vì Sao Kết Luận ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));
        let inner = block.inner(area);
        block.render(area, buf);

        let entries = crate::view_models::decision_stack::extract_decision_stack(inspection);
        let decision = build_decision_header_summary(
            inspection.date.day,
            inspection.date.month,
            inspection.date.year,
        );

        let rows = Layout::vertical([
            Constraint::Length(5),
            Constraint::Min(6),
            Constraint::Length(2),
        ])
        .split(inner);

        let good = entries
            .iter()
            .filter(|e| matches!(e.role, DecisionRole::Support | DecisionRole::Refinement))
            .count();
        let risk = entries
            .iter()
            .filter(|e| {
                matches!(
                    e.role,
                    DecisionRole::Override | DecisionRole::Resistance | DecisionRole::Conflict
                )
            })
            .count();

        let header_lines = vec![
            Line::from(vec![
                Span::styled("  Ngày: ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    format!(
                        "{}/{}/{}",
                        inspection.date.day, inspection.date.month, inspection.date.year
                    ),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("   "),
                Span::styled(
                    format!("{} yếu tố", entries.len()),
                    Style::default().fg(Color::White),
                ),
                Span::raw("   "),
                Span::styled(
                    format!("{} hỗ trợ", good),
                    Style::default().fg(Color::Green),
                ),
                Span::raw("  "),
                Span::styled(format!("{} rủi ro", risk), Style::default().fg(Color::Red)),
            ]),
            Line::from(vec![
                Span::styled("  Kết luận: ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    decision
                        .as_ref()
                        .map(|item| truncate_label(&item.primary_conclusion, 52))
                        .unwrap_or_else(|| "Chưa có kết luận".to_string()),
                    Style::default().fg(Color::White),
                ),
            ]),
            Line::from(vec![
                Span::styled("  Phân loại: ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    decision
                        .as_ref()
                        .map(|item| item.bucket_label.clone())
                        .unwrap_or_else(|| "Chưa rõ".to_string()),
                    Style::default().fg(Color::Yellow),
                ),
                Span::raw("   "),
                Span::styled("Tin cậy: ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    decision
                        .as_ref()
                        .map(|item| item.confidence_label.clone())
                        .unwrap_or_else(|| "Chưa rõ".to_string()),
                    Style::default().fg(Color::White),
                ),
            ]),
            Line::from(vec![Span::styled(
                format!("  Góc nhìn: {}", self.app.explanation_lens.label()),
                Style::default().fg(Color::DarkGray),
            )]),
        ];

        let header_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let header_inner = header_block.inner(rows[0]);
        header_block.render(rows[0], buf);
        Paragraph::new(header_lines)
            .wrap(Wrap { trim: true })
            .render(header_inner, buf);

        let body_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let body_inner = body_block.inner(rows[1]);
        body_block.render(rows[1], buf);

        let body_panes = if body_inner.width < 84 {
            Layout::vertical([Constraint::Percentage(48), Constraint::Percentage(52)])
                .split(body_inner)
        } else {
            Layout::horizontal([Constraint::Percentage(44), Constraint::Percentage(56)])
                .split(body_inner)
        };

        let mut lines = Vec::new();
        let mut current_role = None;
        let selected_idx = self
            .app
            .graph_inspector_cursor
            .min(entries.len().saturating_sub(1));

        for (idx, entry) in entries.iter().enumerate() {
            if current_role != Some(entry.role) {
                current_role = Some(entry.role);
                if !lines.is_empty() {
                    lines.push(Line::from(""));
                }
                let role_color = match entry.role {
                    DecisionRole::Override => Color::Red,
                    DecisionRole::Resistance => Color::Yellow,
                    DecisionRole::Conflict => Color::Magenta,
                    DecisionRole::Support => Color::Green,
                    DecisionRole::Refinement => Color::Cyan,
                };
                lines.push(Line::from(vec![Span::styled(
                    format!("  {}", entry.role.label()),
                    Style::default().fg(role_color).add_modifier(Modifier::BOLD),
                )]));
            }

            let selected = idx == self.app.graph_inspector_cursor;
            let style = if selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let impact_span = entry
                .impact
                .as_ref()
                .map(|imp| {
                    Span::styled(format!(" [{}]", imp), Style::default().fg(Color::DarkGray))
                })
                .unwrap_or_else(|| Span::raw(""));

            lines.push(Line::from(vec![
                Span::styled(if selected { ">" } else { " " }, style),
                Span::raw(" "),
                Span::styled(
                    truncate_label(&entry.label, body_panes[0].width.saturating_sub(8) as usize),
                    style,
                ),
                impact_span,
            ]));
        }

        if lines.is_empty() {
            lines.push(Line::from(Span::styled(
                "  Không có quyết định nào cho ngày này.",
                Style::default().fg(Color::DarkGray),
            )));
        }

        Paragraph::new(lines)
            .wrap(Wrap { trim: true })
            .render(body_panes[0], buf);

        render_vi_sao_detail(entries.get(selected_idx), body_panes[1], buf);

        render_causality_footer(
            "/?: đổi góc nhìn  Tab: đổi màn  d: chế độ debug  ←/→: đổi ngày",
            rows[2],
            buf,
        );
    }

    fn render_hoat_dong_view(
        &self,
        inspection: &amlich_core::DebugSemanticGraphInspection,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let block = Block::default()
            .title(" Hoạt Động ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));
        let inner = block.inner(area);
        block.render(area, buf);

        let entries = self.app.recommendation_lens_entries(inspection);

        let rows = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(6),
            Constraint::Length(2),
        ])
        .split(inner);

        let nen = entries
            .iter()
            .filter(|e| e.verdict == RecommendationLensVerdict::Nen)
            .count();
        let tranh = entries
            .iter()
            .filter(|e| {
                matches!(
                    e.verdict,
                    RecommendationLensVerdict::KyManh | RecommendationLensVerdict::Tranh
                )
            })
            .count();
        let co_the = entries
            .iter()
            .filter(|e| e.verdict == RecommendationLensVerdict::CoThe)
            .count();

        let header_lines = vec![
            Line::from(vec![
                Span::styled("  Ngày: ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    format!(
                        "{}/{}/{}",
                        inspection.date.day, inspection.date.month, inspection.date.year
                    ),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("   "),
                Span::styled(
                    format!("{} hoạt động", entries.len()),
                    Style::default().fg(Color::White),
                ),
                Span::raw("   "),
                Span::styled(format!("{} nên", nen), Style::default().fg(Color::Green)),
                Span::raw("  "),
                Span::styled(format!("{} tránh", tranh), Style::default().fg(Color::Red)),
                Span::raw("  "),
                Span::styled(
                    format!("{} cân nhắc", co_the),
                    Style::default().fg(Color::Yellow),
                ),
            ]),
            Line::from(vec![Span::styled(
                format!("  Góc nhìn: {}", self.app.explanation_lens.label()),
                Style::default().fg(Color::DarkGray),
            )]),
        ];

        let header_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let header_inner = header_block.inner(rows[0]);
        header_block.render(rows[0], buf);
        Paragraph::new(header_lines)
            .wrap(Wrap { trim: true })
            .render(header_inner, buf);

        if entries.is_empty() {
            let body_block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray));
            let body_inner = body_block.inner(rows[1]);
            body_block.render(rows[1], buf);
            Paragraph::new(Line::from(Span::styled(
                "  Không có hoạt động cho ngày này.",
                Style::default().fg(Color::DarkGray),
            )))
            .wrap(Wrap { trim: true })
            .render(body_inner, buf);
        } else {
            let panes = if rows[1].width < 100 {
                Layout::vertical([Constraint::Percentage(45), Constraint::Percentage(55)])
                    .split(rows[1])
            } else {
                Layout::horizontal([Constraint::Percentage(39), Constraint::Percentage(61)])
                    .split(rows[1])
            };

            render_hoat_dong_master_list(&entries, self.app.graph_inspector_cursor, panes[0], buf);

            let selected = entries.get(
                self.app
                    .graph_inspector_cursor
                    .min(entries.len().saturating_sub(1)),
            );
            render_hoat_dong_detail(selected, panes[1], buf);
        }

        render_causality_footer(
            "/?: đổi góc nhìn  Tab: đổi màn  d: debug mode  ←/→: đổi ngày",
            rows[2],
            buf,
        );
    }

    fn render_nguon_view(
        &self,
        inspection: &amlich_core::DebugSemanticGraphInspection,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let block = Block::default()
            .title(" Xuất Xứ ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));
        let inner = block.inner(area);
        block.render(area, buf);
        let source_groups = collect_source_groups(inspection);

        let rows = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(6),
            Constraint::Length(2),
        ])
        .split(inner);

        let header_lines = vec![
            Line::from(vec![
                Span::styled("  Ngày: ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    format!(
                        "{}/{}/{}",
                        inspection.date.day, inspection.date.month, inspection.date.year
                    ),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("   "),
                Span::styled(
                    format!("{} nhóm xuất xứ", source_groups.len()),
                    Style::default().fg(Color::White),
                ),
                Span::raw("   "),
                Span::styled(
                    format!("{} yếu tố", inspection.visualization.nodes.len()),
                    Style::default().fg(Color::DarkGray),
                ),
            ]),
            Line::from(vec![Span::styled(
                format!("  Góc nhìn: {}", self.app.explanation_lens.label()),
                Style::default().fg(Color::DarkGray),
            )]),
        ];

        let header_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let header_inner = header_block.inner(rows[0]);
        header_block.render(rows[0], buf);
        Paragraph::new(header_lines)
            .wrap(Wrap { trim: true })
            .render(header_inner, buf);

        let body_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let body_inner = body_block.inner(rows[1]);
        body_block.render(rows[1], buf);
        let body_panes = if body_inner.width < 92 {
            Layout::vertical([Constraint::Percentage(44), Constraint::Percentage(56)])
                .split(body_inner)
        } else {
            Layout::horizontal([Constraint::Percentage(42), Constraint::Percentage(58)])
                .split(body_inner)
        };

        render_nguon_master_list(
            &source_groups,
            self.app.graph_inspector_cursor,
            body_panes[0],
            buf,
        );
        let selected = source_groups.get(
            self.app
                .graph_inspector_cursor
                .min(source_groups.len().saturating_sub(1)),
        );
        render_nguon_detail(selected, body_panes[1], buf);

        render_causality_footer(
            "/?: đổi góc nhìn  Tab: đổi màn  d: debug mode  ←/→: đổi ngày",
            rows[2],
            buf,
        );
    }
}

impl GraphInspectorScreenWidget<'_> {
    fn render_causality_view(
        &self,
        inspection: &amlich_core::DebugSemanticGraphInspection,
        area: Rect,
        buf: &mut Buffer,
        user_facing: bool,
    ) {
        let title = if user_facing {
            " Yếu Tố "
        } else {
            " Bản Đồ Nhân Quả (Causality Map) "
        };
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));
        let inner = block.inner(area);
        block.render(area, buf);

        let nodes = crate::view_models::causality::extract_causality_tree(inspection);
        let selected_node = nodes.get(
            self.app
                .graph_inspector_cursor
                .min(nodes.len().saturating_sub(1)),
        );

        let rows = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(2),
        ])
        .split(inner);

        render_causality_header(inspection, selected_node, rows[0], buf);

        use crate::state::CausalityFocus;
        match &self.app.causality_focus {
            CausalityFocus::SummaryList => {
                let panes = if rows[1].width < 100 {
                    Layout::vertical([Constraint::Percentage(45), Constraint::Percentage(55)])
                        .split(rows[1])
                } else {
                    Layout::horizontal([Constraint::Percentage(39), Constraint::Percentage(61)])
                        .split(rows[1])
                };

                render_causality_master_list(
                    &nodes,
                    self.app.graph_inspector_cursor,
                    panes[0],
                    buf,
                );
                render_causality_detail_preview(selected_node, panes[1], buf, user_facing);

                let summary_help = if user_facing {
                    "/?: đổi góc nhìn  Tab: đổi màn  ↑↓: chọn  Enter: khóa chi tiết  d: debug mode"
                } else {
                    "↑↓: chọn  Enter: khóa vào chi tiết  d: debug mode  ←/→: đổi ngày"
                };
                render_causality_footer(summary_help, rows[2], buf);
            }
            CausalityFocus::DetailFlow(node_id) => {
                let node = nodes.iter().find(|n| &n.node_id == node_id);
                render_causality_detail_preview(node, rows[1], buf, user_facing);
                let detail_help = if user_facing {
                    "/?: đổi góc nhìn  Tab: đổi màn  Esc: quay lại  d: debug mode  ←/→: đổi ngày"
                } else {
                    "Esc: quay lại danh sách  d: debug mode  ←/→: đổi ngày"
                };
                render_causality_footer(detail_help, rows[2], buf);
            }
        }
    }
}

fn render_causality_header(
    inspection: &amlich_core::DebugSemanticGraphInspection,
    selected: Option<&crate::view_models::causality::CausalityNode>,
    area: Rect,
    buf: &mut Buffer,
) {
    let block = Block::default()
        .title(" Tóm Tắt ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let nodes = crate::view_models::causality::extract_causality_tree(inspection);
    let good = nodes
        .iter()
        .filter(|node| {
            matches!(
                node.severity.as_deref(),
                Some("favorable") | Some("positive")
            )
        })
        .count();
    let risk = nodes
        .iter()
        .filter(|node| matches!(node.severity.as_deref(), Some("critical") | Some("caution")))
        .count();
    let neutral = nodes.len().saturating_sub(good + risk);

    let focus = selected
        .map(|node| truncate_label(&node.label, 22))
        .unwrap_or_else(|| "chưa có mục nào".to_string());

    let lines = vec![
        Line::from(vec![
            Span::styled("  Ngày: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!(
                    "{}/{}/{}",
                    inspection.date.day, inspection.date.month, inspection.date.year
                ),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("   "),
            Span::styled(
                format!("{} yếu tố trọng tâm", nodes.len()),
                Style::default().fg(Color::White),
            ),
            Span::raw("   "),
            Span::styled(format!("{} tốt", good), Style::default().fg(Color::Green)),
            Span::raw("  "),
            Span::styled(format!("{} xấu", risk), Style::default().fg(Color::Red)),
            Span::raw("  "),
            Span::styled(
                format!("{} trung tính", neutral),
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Đang chọn: ", Style::default().fg(Color::Cyan)),
            Span::styled(focus, Style::default().fg(Color::Yellow)),
            Span::raw("   "),
            Span::styled("[d] debug mode", Style::default().fg(Color::DarkGray)),
        ]),
    ];

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_causality_master_list(
    nodes: &[crate::view_models::causality::CausalityNode],
    cursor: usize,
    area: Rect,
    buf: &mut Buffer,
) {
    let block = Block::default()
        .title(" Các Yếu Tố ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let mut items = Vec::new();
    let mut current_cluster = String::new();
    let mut selected_item = None;

    for (idx, node) in nodes.iter().enumerate() {
        if node.cluster != current_cluster {
            current_cluster = node.cluster.clone();
            if !items.is_empty() {
                items.push(ratatui::widgets::ListItem::new(Line::from("")));
            }
            items.push(ratatui::widgets::ListItem::new(Line::from(vec![
                Span::styled(
                    format!("  {}", causality_cluster_title(&node.cluster)),
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                ),
            ])));
        }

        let selected = idx == cursor;
        if selected {
            selected_item = Some(items.len());
        }

        let (severity_tag, color, icon) = causality_severity_badge(node.severity.as_deref());
        let style = if selected {
            Style::default()
                .fg(Color::Yellow)
                .bg(Color::Rgb(30, 30, 40))
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let relation_count = node.incoming.len() + node.outgoing.len();
        let density_label = match relation_count {
            0 => "tĩnh",
            1..=2 => "ít",
            3..=4 => "vừa",
            _ => "đậm",
        };
        let relation_style = if relation_count > 0 {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        items.push(ratatui::widgets::ListItem::new(Line::from(vec![
            Span::styled(if selected { ">" } else { " " }, style),
            Span::raw(" "),
            Span::styled(format!("{:<6}", severity_tag), Style::default().fg(color)),
            Span::raw(" "),
            Span::styled(icon, Style::default().fg(color)),
            Span::raw(" "),
            Span::styled(
                truncate_label(&node.label, inner.width.saturating_sub(26) as usize),
                style,
            ),
            Span::raw(" "),
            Span::styled(format!("{:>2}q", relation_count), relation_style),
            Span::raw(" "),
            Span::styled(density_label, Style::default().fg(Color::DarkGray)),
        ])));
    }

    if items.is_empty() {
        items.push(ratatui::widgets::ListItem::new(Line::from(Span::styled(
            "  Không tìm thấy dữ liệu nhân quả nào.",
            Style::default().fg(Color::DarkGray),
        ))));
    }

    let mut state = ratatui::widgets::ListState::default();
    state.select(selected_item);
    ratatui::widgets::StatefulWidget::render(
        ratatui::widgets::List::new(items),
        inner,
        buf,
        &mut state,
    );
}

fn render_causality_detail_preview(
    node: Option<&crate::view_models::causality::CausalityNode>,
    area: Rect,
    buf: &mut Buffer,
    user_facing: bool,
) {
    let block = Block::default()
        .title(" Chi Tiết Nhân Quả ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(node) = node else {
        Paragraph::new(Line::from(Span::styled(
            "  Chưa có yếu tố nào để hiển thị.",
            Style::default().fg(Color::DarkGray),
        )))
        .render(inner, buf);
        return;
    };

    if node.incoming.is_empty() && node.outgoing.is_empty() {
        let sections: Vec<Rect> = if inner.height < 14 {
            Layout::vertical([Constraint::Length(5), Constraint::Min(6)])
                .split(inner)
                .iter()
                .copied()
                .collect()
        } else {
            Layout::vertical([Constraint::Length(5), Constraint::Min(8)])
                .split(inner)
                .iter()
                .copied()
                .collect()
        };

        render_causality_selected_summary(node, sections[0], buf, user_facing);
        render_causality_empty_explanation(node, sections[1], buf);
        return;
    }

    let sections: Vec<Rect> = if inner.width < 70 || inner.height < 18 {
        Layout::vertical([
            Constraint::Length(5),
            Constraint::Min(6),
            Constraint::Min(6),
        ])
        .split(inner)
        .iter()
        .copied()
        .collect()
    } else {
        let cols = Layout::horizontal([Constraint::Percentage(46), Constraint::Percentage(54)])
            .split(inner);
        let right = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(cols[1]);
        vec![cols[0], right[0], right[1]]
    };

    render_causality_selected_summary(node, sections[0], buf, user_facing);
    render_causality_relation_block(" Đến Từ Đâu ", &node.incoming, true, sections[1], buf);
    render_causality_relation_block(" Dẫn Tới Điều Gì ", &node.outgoing, false, sections[2], buf);
}

fn render_causality_selected_summary(
    node: &crate::view_models::causality::CausalityNode,
    area: Rect,
    buf: &mut Buffer,
    user_facing: bool,
) {
    let cluster_label = if user_facing { "Nhóm" } else { "Cụm" };
    let kind_label = if user_facing {
        "Kiểu yếu tố"
    } else {
        "Loại"
    };
    let conn_label = if user_facing {
        "Liên kết"
    } else {
        "Quan hệ"
    };
    let (severity_tag, color, icon) = causality_severity_badge(node.severity.as_deref());
    let lines = vec![
        Line::from(vec![
            Span::styled(format!("  {} ", icon), Style::default().fg(color)),
            Span::styled(
                truncate_label(&node.label, area.width.saturating_sub(8) as usize),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                format!("  {}: ", cluster_label),
                Style::default().fg(Color::Cyan),
            ),
            Span::styled(
                causality_cluster_title(&node.cluster),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                format!("  {}: ", kind_label),
                Style::default().fg(Color::Cyan),
            ),
            Span::styled(&node.semantic_kind, Style::default().fg(Color::White)),
            Span::raw("   "),
            Span::styled("Mức: ", Style::default().fg(Color::Cyan)),
            Span::styled(severity_tag, Style::default().fg(color)),
        ]),
        Line::from(vec![
            Span::styled(
                format!("  {}: ", conn_label),
                Style::default().fg(Color::Cyan),
            ),
            Span::styled(
                format!("{} vào, {} ra", node.incoming.len(), node.outgoing.len()),
                Style::default().fg(Color::White),
            ),
        ]),
    ];

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(area, buf);
}

fn render_causality_relation_block(
    title: &str,
    edges: &[crate::view_models::causality::CausalityEdge],
    incoming: bool,
    area: Rect,
    buf: &mut Buffer,
) {
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let mut lines = Vec::new();
    if edges.is_empty() {
        let empty = if incoming {
            "  Không có nguyên nhân trực tiếp nổi bật."
        } else {
            "  Chưa thấy ảnh hưởng hoặc lời khuyên trực tiếp."
        };
        lines.push(Line::from(Span::styled(
            empty,
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        for edge in edges.iter().take(inner.height.saturating_sub(1) as usize) {
            let (icon, color) = if !incoming
                && (edge.neighbor_label.starts_with("Kỵ") || edge.edge_label.contains("avoid"))
            {
                ("x", Color::Red)
            } else if !incoming
                && (edge.neighbor_label.starts_with("Nên") || edge.edge_label.contains("support"))
            {
                ("+", Color::Green)
            } else if incoming {
                ("<", Color::Cyan)
            } else {
                (">", Color::White)
            };

            lines.push(Line::from(vec![
                Span::styled(format!("{} ", icon), Style::default().fg(color)),
                Span::styled(
                    truncate_label(
                        &edge.neighbor_label,
                        inner.width.saturating_sub(12) as usize,
                    ),
                    Style::default().fg(Color::White),
                ),
            ]));
            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(
                    format!(
                        "{} [{}]",
                        truncate_label(&edge.edge_label, 24),
                        truncate_label(&edge.neighbor_kind, 12)
                    ),
                    Style::default().fg(Color::DarkGray),
                ),
            ]));
        }
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_causality_empty_explanation(
    node: &crate::view_models::causality::CausalityNode,
    area: Rect,
    buf: &mut Buffer,
) {
    let block = Block::default()
        .title(" Trạng Thái Liên Kết ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let fact_label = match node.semantic_kind.as_str() {
        "hoang_dao_hour" => "một fact node về giờ hoàng đạo của ngày",
        "day_deity" | "deity" => "một fact node về thần sát / sao trong ngày",
        "star" => "một fact node về sao chi phối",
        "truc" => "một fact node về trực của ngày",
        "xung_hop" => "một fact node về tương tác can chi",
        "taboo" => "một fact node về điều kiêng kỵ",
        _ => "một fact node trong lớp causality hiện tại",
    };

    let lines = vec![
        Line::from(vec![
            Span::styled("  Node này hiện được biểu diễn như ", Style::default().fg(Color::White)),
            Span::styled(fact_label, Style::default().fg(Color::Yellow)),
            Span::raw("."),
        ]),
        Line::from(Span::styled(
            "  Chưa có cạnh vào/ra trực tiếp, nên màn hình không tách riêng phần nguyên nhân và hệ quả.",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Gợi ý đọc: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                "xem đây là dữ kiện nền để đối chiếu với các node có quan hệ nhiều hơn trong danh sách bên trái.",
                Style::default().fg(Color::White),
            ),
        ]),
    ];

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_causality_footer(help: &str, area: Rect, buf: &mut Buffer) {
    Paragraph::new(Line::from(Span::styled(
        format!("  {}", help),
        Style::default().fg(Color::DarkGray),
    )))
    .render(area, buf);
}

fn render_hoat_dong_master_list(
    entries: &[crate::state::RecommendationLensEntry],
    cursor: usize,
    area: Rect,
    buf: &mut Buffer,
) {
    let block = Block::default()
        .title(" Các Hoạt Động ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let mut lines = Vec::new();
    let mut current_bucket: Option<RecommendationLensVerdict> = None;

    for (idx, entry) in entries.iter().enumerate() {
        if current_bucket != Some(entry.verdict) {
            current_bucket = Some(entry.verdict);
            if !lines.is_empty() {
                lines.push(Line::from(""));
            }
            let (bucket_label, bucket_color) = match entry.verdict {
                RecommendationLensVerdict::KyManh => ("KỴ MẠNH", Color::Red),
                RecommendationLensVerdict::Tranh => ("TRÁNH", Color::Red),
                RecommendationLensVerdict::CoThe => ("CÂN NHẮC", Color::Yellow),
                RecommendationLensVerdict::Nen => ("NÊN", Color::Green),
            };
            lines.push(Line::from(vec![Span::styled(
                format!("  {}", bucket_label),
                Style::default()
                    .fg(bucket_color)
                    .add_modifier(Modifier::BOLD),
            )]));
        }

        let selected = idx == cursor;
        let marker = if selected { ">" } else { " " };
        let style = if selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        let hard_stop_marker = if entry.verdict == RecommendationLensVerdict::KyManh {
            " !"
        } else {
            ""
        };

        lines.push(Line::from(vec![
            Span::styled(format!("{} ", marker), style),
            Span::styled(
                format!(
                    "{}{}",
                    truncate_label(&entry.activity, inner.width.saturating_sub(6) as usize),
                    hard_stop_marker
                ),
                style,
            ),
        ]));
    }

    if lines.is_empty() {
        lines.push(Line::from(Span::styled(
            "  Không có hoạt động cho ngày này",
            Style::default().fg(Color::DarkGray),
        )));
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_hoat_dong_detail(
    entry: Option<&crate::state::RecommendationLensEntry>,
    area: Rect,
    buf: &mut Buffer,
) {
    let block = Block::default()
        .title(" Chi Tiết ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(entry) = entry else {
        Paragraph::new(Line::from(Span::styled(
            "  Chọn một hoạt động để xem chi tiết.",
            Style::default().fg(Color::DarkGray),
        )))
        .render(inner, buf);
        return;
    };

    let (bucket_label, bucket_color) = match entry.verdict {
        RecommendationLensVerdict::KyManh => ("Kỵ mạnh", Color::Red),
        RecommendationLensVerdict::Tranh => ("Tránh", Color::Red),
        RecommendationLensVerdict::CoThe => ("Có thể", Color::Yellow),
        RecommendationLensVerdict::Nen => ("Nên", Color::Green),
    };

    let mut lines = vec![
        Line::from(vec![
            Span::styled("  Hoạt động: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                truncate_label(&entry.activity, inner.width.saturating_sub(16) as usize),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Phân loại: ", Style::default().fg(Color::Cyan)),
            Span::styled(bucket_label, Style::default().fg(bucket_color)),
            Span::raw(if entry.verdict == RecommendationLensVerdict::KyManh {
                "  [KỴ MẠNH]"
            } else {
                ""
            }),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Lý do: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                truncate_label(
                    &entry.headline_reason,
                    inner.width.saturating_sub(12) as usize,
                ),
                Style::default().fg(Color::White),
            ),
        ]),
    ];

    if entry.reasons.len() > 1 {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![Span::styled(
            "  Tín hiệu liên quan:",
            Style::default().fg(Color::Cyan),
        )]));
        for reason in entry.reasons.iter().take(4) {
            lines.push(Line::from(vec![
                Span::raw("    "),
                Span::styled(
                    truncate_label(reason, inner.width.saturating_sub(8) as usize),
                    Style::default().fg(Color::White),
                ),
            ]));
        }
    }

    if !entry.source.is_empty() {
        lines.push(Line::from(vec![
            Span::styled("  Nguồn: ", Style::default().fg(Color::Cyan)),
            Span::styled(&entry.source, Style::default().fg(Color::DarkGray)),
        ]));
    }

    if !entry.provenance.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![Span::styled(
            "  Nguồn gốc:",
            Style::default().fg(Color::Cyan),
        )]));
        for prov in entry.provenance.iter().take(3) {
            lines.push(Line::from(vec![
                Span::raw("    "),
                Span::styled(
                    format!(
                        "[{}] {}",
                        provenance_source_family_label(prov),
                        prov.source_id
                    ),
                    Style::default().fg(Color::DarkGray),
                ),
            ]));
        }
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn causality_cluster_title(cluster: &str) -> String {
    match cluster {
        "day-core" => "THẦN SÁT CHUNG CỦA NGÀY".to_string(),
        "bazi" => "TƯƠNG TÁC CÁ NHÂN (BÁT TỰ)".to_string(),
        "day_person_matrix" => "MA TRẬN NGƯỜI - NGÀY".to_string(),
        "domain_day_boost" => "MA TRẬN NĂNG LƯỢNG".to_string(),
        other => other.to_uppercase(),
    }
}

fn causality_severity_badge(severity: Option<&str>) -> (&'static str, Color, &'static str) {
    match severity {
        Some("favorable") | Some("positive") => ("TỐT", Color::Green, "*"),
        Some("critical") | Some("caution") => ("XẤU", Color::Red, "!"),
        _ => ("BÌNH", Color::DarkGray, "~"),
    }
}

fn build_decision_header_summary(day: i32, month: i32, year: i32) -> Option<DecisionHeaderSummary> {
    let snapshot = amlich_core::calculate_day_snapshot(day, month, year);
    let bundle = amlich_core::build_initiation_opening_reasoning_bundle(&snapshot, None).ok()?;
    Some(DecisionHeaderSummary {
        primary_conclusion: bundle.decision_export.primary_conclusion,
        bucket_label: match bundle.decision_export.recommendation_bucket {
            amlich_core::RecommendationBucket::Avoid => "Tránh".to_string(),
            amlich_core::RecommendationBucket::Cautious => "Thận trọng".to_string(),
            amlich_core::RecommendationBucket::Mixed => "Trái chiều".to_string(),
            amlich_core::RecommendationBucket::Favorable => "Thuận".to_string(),
        },
        confidence_label: match bundle.decision_export.confidence {
            amlich_core::DecisionConfidence::Low => "Thấp".to_string(),
            amlich_core::DecisionConfidence::Medium => "Trung bình".to_string(),
            amlich_core::DecisionConfidence::High => "Cao".to_string(),
        },
    })
}

fn render_vi_sao_detail(
    entry: Option<&crate::view_models::decision_stack::DecisionStackEntry>,
    area: Rect,
    buf: &mut Buffer,
) {
    let block = Block::default()
        .title(" Chi Tiết ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(entry) = entry else {
        Paragraph::new(Line::from(Span::styled(
            "  Chọn một yếu tố để xem chi tiết.",
            Style::default().fg(Color::DarkGray),
        )))
        .render(inner, buf);
        return;
    };

    let role_label = match entry.role {
        DecisionRole::Override => "Lấn át",
        DecisionRole::Resistance => "Cản trở",
        DecisionRole::Conflict => "Xung đột",
        DecisionRole::Support => "Hỗ trợ",
        DecisionRole::Refinement => "Bổ sung",
    };

    let mut lines = vec![
        Line::from(vec![
            Span::styled("  Yếu tố: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                truncate_label(&entry.label, inner.width.saturating_sub(12) as usize),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Vai trò: ", Style::default().fg(Color::Cyan)),
            Span::styled(role_label, Style::default().fg(Color::White)),
        ]),
    ];

    if let Some(impact) = &entry.impact {
        lines.push(Line::from(vec![
            Span::styled("  Ảnh hưởng: ", Style::default().fg(Color::Cyan)),
            Span::styled(impact, Style::default().fg(Color::White)),
        ]));
    }

    if !entry.outgoing_targets.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![Span::styled(
            "  Dẫn tới:",
            Style::default().fg(Color::Cyan),
        )]));
        for target in entry.outgoing_targets.iter().take(4) {
            lines.push(Line::from(vec![
                Span::raw("    -> "),
                Span::styled(
                    truncate_label(
                        &target.target_label,
                        inner.width.saturating_sub(14) as usize,
                    ),
                    Style::default().fg(Color::White),
                ),
                Span::styled(
                    format!(" <{}>", truncate_label(&target.edge_label, 16)),
                    Style::default().fg(Color::DarkGray),
                ),
            ]));
        }
    }

    if let Some(first) = entry.provenance.first() {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("  Nguồn: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!(
                    "[{}] {}",
                    provenance_source_family_label(first),
                    first.source_id
                ),
                Style::default().fg(Color::DarkGray),
            ),
        ]));
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn collect_source_groups(
    inspection: &amlich_core::DebugSemanticGraphInspection,
) -> Vec<SourceLensGroup> {
    let mut groups: BTreeMap<(String, String, String), BTreeMap<String, String>> = BTreeMap::new();

    for node in &inspection.visualization.nodes {
        if node.provenance.is_empty() {
            groups
                .entry((
                    "(không rõ)".to_string(),
                    "(không rõ)".to_string(),
                    "(không rõ)".to_string(),
                ))
                .or_default()
                .insert(node.node_id.clone(), node.label.clone());
            continue;
        }

        for prov in &node.provenance {
            groups
                .entry((
                    provenance_source_family_label(prov).to_string(),
                    prov.source_id.clone(),
                    prov.method.clone(),
                ))
                .or_default()
                .insert(node.node_id.clone(), node.label.clone());
        }
    }

    let mut groups: Vec<_> = groups
        .into_iter()
        .map(|((family, source_id, method), factors)| SourceLensGroup {
            family,
            source_id,
            method,
            factors: factors.into_values().collect(),
        })
        .collect();

    groups.sort_by(|a, b| {
        b.factors
            .len()
            .cmp(&a.factors.len())
            .then_with(|| a.family.cmp(&b.family))
            .then_with(|| a.source_id.cmp(&b.source_id))
            .then_with(|| a.method.cmp(&b.method))
    });
    groups
}

fn render_nguon_master_list(
    groups: &[SourceLensGroup],
    cursor: usize,
    area: Rect,
    buf: &mut Buffer,
) {
    let block = Block::default()
        .title(" Các Nhóm Xuất Xứ ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let mut lines = Vec::new();

    if groups.is_empty() {
        lines.push(Line::from(Span::styled(
            "  Không có dấu vết xuất xứ.",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        for (idx, group) in groups.iter().enumerate() {
            let selected = idx == cursor.min(groups.len().saturating_sub(1));
            let style = if selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            lines.push(Line::from(vec![
                Span::styled(if selected { "> " } else { "  " }, style),
                Span::styled(
                    truncate_label(&group.family, inner.width.saturating_sub(18) as usize),
                    style,
                ),
                Span::styled(
                    format!(" {:>3} yếu tố", group.factors.len()),
                    Style::default().fg(Color::Cyan),
                ),
            ]));
            lines.push(Line::from(vec![
                Span::raw("    "),
                Span::styled(
                    truncate_label(&group.source_id, inner.width.saturating_sub(8) as usize),
                    Style::default().fg(Color::DarkGray),
                ),
            ]));
        }
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_nguon_detail(entry: Option<&SourceLensGroup>, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Chi Tiết Xuất Xứ ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(entry) = entry else {
        Paragraph::new(Line::from(Span::styled(
            "  Chọn một nhóm xuất xứ để xem chi tiết.",
            Style::default().fg(Color::DarkGray),
        )))
        .render(inner, buf);
        return;
    };

    let mut lines = vec![
        Line::from(vec![
            Span::styled("  Loại nguồn: ", Style::default().fg(Color::Cyan)),
            Span::styled(&entry.family, Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("  Mã nguồn: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                truncate_label(&entry.source_id, inner.width.saturating_sub(14) as usize),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Cách suy ra: ", Style::default().fg(Color::Cyan)),
            Span::styled(&entry.method, Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  Các yếu tố được suy ra:",
            Style::default().fg(Color::Cyan),
        )]),
    ];

    for factor in entry.factors.iter().take(8) {
        lines.push(Line::from(vec![
            Span::raw("    "),
            Span::styled(
                truncate_label(factor, inner.width.saturating_sub(8) as usize),
                Style::default().fg(Color::White),
            ),
        ]));
    }
    if entry.factors.len() > 8 {
        lines.push(Line::from(vec![Span::styled(
            format!("    ... +{} yếu tố nữa", entry.factors.len() - 8),
            Style::default().fg(Color::DarkGray),
        )]));
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{AppState, ExplorerAction, ExplorerField, ExplorerSelection, PageSection};
    use amlich_api::{
        RecommendationPackCatalogEntryDto, RulesetCatalogEntryDto, RulesetDefaultsDto,
    };
    use chrono::NaiveDate;

    fn sample_app() -> AppState {
        let date = NaiveDate::from_ymd_opt(2026, 3, 12).expect("valid date");
        let ruleset_catalog = vec![RulesetCatalogEntryDto {
            id: "vn_baseline_v1".to_string(),
            canonical_id: "vn_baseline_v1".to_string(),
            version: "v1".to_string(),
            region: "vn".to_string(),
            profile: "baseline".to_string(),
            schema_version: "amlich.engine/v1".to_string(),
            is_default: true,
            aliases: vec![],
            defaults: RulesetDefaultsDto {
                tz_offset: 7.0,
                meridian: None,
            },
            source_notes: vec![],
        }];
        let recommendation_pack_catalog = vec![RecommendationPackCatalogEntryDto {
            pack_id: "pack.nhi_thap_bat_tu.v1".to_string(),
            request_field: "enabled_pack_ids".to_string(),
            version: "v1".to_string(),
            source_family: "traditional".to_string(),
            mode: "advisory".to_string(),
        }];
        let selection = ExplorerSelection::defaults(date, &ruleset_catalog);
        AppState {
            running: true,
            date,
            scroll_offset: 0,
            content_height: 0,
            viewport_height: 0,
            bundle: None,
            personal_matrix: None,
            is_loading: false,
            error_msg: None,
            ruleset_catalog,
            recommendation_pack_catalog,
            applied_selection: selection.clone(),
            staged_selection: selection,
            explorer_focus: ExplorerField::Date,
            explorer_action: ExplorerAction::Apply,
            pack_cursor: 0,
            show_guidance_details: false,
            show_tietkhi_details: false,
            show_evidence: false,
            show_week_strip: true,
            show_graph_recommendations: false,
            verbosity: crate::state::ui_prefs::VerbosityMode::Compact,
            focused_section: PageSection::Hero,
            zoomed_section: None,
            expanded_sections: Default::default(),
            app_mode: crate::state::AppMode::Normal,
            search_input: String::new(),
            personal_focus: crate::state::PersonalField::BirthYear,
            personal_draft: crate::state::PersonalDraft {
                birth_year: String::new(),
                birth_month: String::new(),
                birth_day: String::new(),
                birth_hour: String::new(),
                birth_minute: String::new(),
                gender: None,
            },
            calendar_cursor: date,
            navigation_history: Vec::new(),
            active_view: crate::state::ActiveView::GraphInspector,
            view_history: Vec::new(),
            graph_inspector_focus: crate::state::GraphInspectorFocus::Summary,
            graph_inspector_cursor: 0,
            graph_inspector_search_query: String::new(),
            graph_inspector_search_cursor: 0,
            graph_inspector_focus_before_search: None,
            graph_inspector_lens: crate::state::GraphInspectorLens::General,
            dev_inspector_mode: true,
            explanation_lens: crate::state::UserExplanationLens::ViSao,
            causality_focus: crate::state::CausalityFocus::SummaryList,
        }
    }

    fn render_text(app: &AppState) -> String {
        let area = Rect::new(0, 0, 100, 40);
        let mut buf = Buffer::empty(area);
        GraphInspectorScreenWidget::new(app, LayoutMode::Large).render(area, &mut buf);
        (0..area.height)
            .map(|y| {
                (0..area.width)
                    .map(|x| buf[(x, y)].symbol())
                    .collect::<String>()
                    .trim_end()
                    .to_string()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn graph_inspector_renders_header_and_summary() {
        let app = sample_app();
        let text = render_text(&app);

        assert!(text.contains("Đồ Thị Ngữ Nghĩa"));
        assert!(text.contains("Tổng Quan"));
        assert!(text.contains("Graph Preview"));
        assert!(text.contains("Tổng nodes:"));
        assert!(text.contains("Tổng edges:"));
        assert!(text.contains("Cluster Counts"));
        assert!(text.contains("Severity Counts") || text.contains("Semantic Kind Counts"));
    }

    #[test]
    fn graph_inspector_shows_recommendation_toggle_status() {
        let app = sample_app();
        let text = render_text(&app);

        assert!(text.contains("TẮT"));
    }

    #[test]
    fn graph_inspector_with_recommendations_shows_enabled() {
        let mut app = sample_app();
        app.show_graph_recommendations = true;
        let text = render_text(&app);

        assert!(text.contains("BẬT"));
    }

    #[test]
    fn render_node_detail_shows_provenance_fields() {
        let node = amlich_core::semantic_graph::VisualizationNode {
            node_id: "node:test".to_string(),
            label: "Node Test".to_string(),
            cluster: "reasoning".to_string(),
            semantic_kind: "recommendation_hit".to_string(),
            severity: Some("caution".to_string()),
            provenance: vec![amlich_core::ReasoningEvidenceEnvelope {
                source_family: amlich_core::ReasoningEvidenceSourceFamily::AlmanacRule,
                source_id: "rule:demo".to_string(),
                method: "derive_demo".to_string(),
                note: Some("Example provenance note".to_string()),
            }],
            shape_hint: Some("diamond".to_string()),
        };
        let area = Rect::new(0, 0, 100, 30);
        let mut buf = Buffer::empty(area);

        render_node_detail(Some(&node), &[], "node:test", area, &mut buf);

        let text = (0..area.height)
            .map(|y| {
                (0..area.width)
                    .map(|x| buf[(x, y)].symbol())
                    .collect::<String>()
                    .trim_end()
                    .to_string()
            })
            .collect::<Vec<_>>()
            .join("\n");

        assert!(text.contains("Provenance:"));
        assert!(text.contains("family: almanac_rule"));
        assert!(text.contains("source_id: rule:demo"));
        assert!(text.contains("method:    derive_demo"));
        assert!(text.contains("note:      Example provenance note"));
    }

    #[test]
    fn render_node_detail_shows_absent_provenance_message() {
        let node = amlich_core::semantic_graph::VisualizationNode {
            node_id: "node:test".to_string(),
            label: "Node Test".to_string(),
            cluster: "reasoning".to_string(),
            semantic_kind: "recommendation_hit".to_string(),
            severity: None,
            provenance: vec![],
            shape_hint: None,
        };
        let area = Rect::new(0, 0, 100, 30);
        let mut buf = Buffer::empty(area);

        render_node_detail(Some(&node), &[], "node:test", area, &mut buf);

        let text = (0..area.height)
            .map(|y| {
                (0..area.width)
                    .map(|x| buf[(x, y)].symbol())
                    .collect::<String>()
                    .trim_end()
                    .to_string()
            })
            .collect::<Vec<_>>()
            .join("\n");

        assert!(text.contains("Không có provenance cho node này."));
    }

    #[test]
    fn render_local_subgraph_shows_focal_and_directional_neighbors() {
        let node = amlich_core::semantic_graph::VisualizationNode {
            node_id: "node:focal".to_string(),
            label: "Focal Node".to_string(),
            cluster: "reasoning".to_string(),
            semantic_kind: "recommendation_hit".to_string(),
            severity: Some("caution".to_string()),
            provenance: vec![],
            shape_hint: Some("diamond".to_string()),
        };
        let incoming_node = amlich_core::semantic_graph::VisualizationNode {
            node_id: "node:source".to_string(),
            label: "Source Node".to_string(),
            cluster: "snapshot".to_string(),
            semantic_kind: "day_snapshot".to_string(),
            severity: None,
            provenance: vec![],
            shape_hint: Some("box".to_string()),
        };
        let outgoing_node = amlich_core::semantic_graph::VisualizationNode {
            node_id: "node:target".to_string(),
            label: "Target Activity".to_string(),
            cluster: "recommendation".to_string(),
            semantic_kind: "activity".to_string(),
            severity: Some("favorable".to_string()),
            provenance: vec![],
            shape_hint: Some("box".to_string()),
        };
        let incoming_edge = amlich_core::semantic_graph::VisualizationEdge {
            edge_id: "edge:in".to_string(),
            from_id: incoming_node.node_id.clone(),
            to_id: node.node_id.clone(),
            label: "supports".to_string(),
            semantic_kind: "support".to_string(),
            weight: 1,
        };
        let outgoing_edge = amlich_core::semantic_graph::VisualizationEdge {
            edge_id: "edge:out".to_string(),
            from_id: node.node_id.clone(),
            to_id: outgoing_node.node_id.clone(),
            label: "targets_activity".to_string(),
            semantic_kind: "targets_activity".to_string(),
            weight: 2,
        };
        let area = Rect::new(0, 0, 120, 20);
        let mut buf = Buffer::empty(area);

        render_local_subgraph(
            Some(&node),
            &[&incoming_edge, &outgoing_edge],
            &[node.clone(), incoming_node, outgoing_node],
            area,
            &mut buf,
        );

        let text = (0..area.height)
            .map(|y| {
                (0..area.width)
                    .map(|x| buf[(x, y)].symbol())
                    .collect::<String>()
                    .trim_end()
                    .to_string()
            })
            .collect::<Vec<_>>()
            .join("\n");

        assert!(text.contains("Local Subgraph"));
        assert!(text.contains("[FOCAL !] Focal Node"));
        assert!(text.contains("Source Node"));
        assert!(text.contains("Target Activity"));
        assert!(text.contains("Enter/l opens the full edge list."));
    }

    #[test]
    fn render_local_subgraph_uses_compact_layout_on_narrow_width() {
        let node = amlich_core::semantic_graph::VisualizationNode {
            node_id: "node:focal".to_string(),
            label: "Focal Node".to_string(),
            cluster: "reasoning".to_string(),
            semantic_kind: "recommendation_hit".to_string(),
            severity: None,
            provenance: vec![],
            shape_hint: Some("diamond".to_string()),
        };
        let outgoing_node = amlich_core::semantic_graph::VisualizationNode {
            node_id: "node:target".to_string(),
            label: "Target Activity".to_string(),
            cluster: "recommendation".to_string(),
            semantic_kind: "activity".to_string(),
            severity: None,
            provenance: vec![],
            shape_hint: Some("box".to_string()),
        };
        let outgoing_edge = amlich_core::semantic_graph::VisualizationEdge {
            edge_id: "edge:out".to_string(),
            from_id: node.node_id.clone(),
            to_id: outgoing_node.node_id.clone(),
            label: "targets_activity".to_string(),
            semantic_kind: "targets_activity".to_string(),
            weight: 2,
        };
        let area = Rect::new(0, 0, 72, 16);
        let mut buf = Buffer::empty(area);

        render_local_subgraph(
            Some(&node),
            &[&outgoing_edge],
            &[node.clone(), outgoing_node],
            area,
            &mut buf,
        );

        let text = (0..area.height)
            .map(|y| {
                (0..area.width)
                    .map(|x| buf[(x, y)].symbol())
                    .collect::<String>()
                    .trim_end()
                    .to_string()
            })
            .collect::<Vec<_>>()
            .join("\n");

        assert!(text.contains("Incoming"));
        assert!(text.contains("Outgoing"));
        assert!(text.contains("Enter/l: xem edge list"));
    }

    #[test]
    fn hoat_dong_view_renders_title() {
        let mut app = sample_app();
        app.dev_inspector_mode = false;
        app.explanation_lens = UserExplanationLens::HoatDong;
        let text = render_text(&app);
        assert!(text.contains("Hoạt Động"));
    }

    #[test]
    fn hoat_dong_view_empty_state() {
        let mut app = sample_app();
        app.dev_inspector_mode = false;
        app.explanation_lens = UserExplanationLens::HoatDong;
        let text = render_text(&app);
        assert!(!text.contains("Không có hoạt động cho ngày này"));
        assert!(
            text.contains("Kỵ mạnh")
                || text.contains("Tránh")
                || text.contains("Có thể")
                || text.contains("Nên")
        );
    }

    #[test]
    fn nguon_view_renders_title() {
        let mut app = sample_app();
        app.dev_inspector_mode = false;
        app.explanation_lens = UserExplanationLens::Nguon;
        let text = render_text(&app);
        assert!(text.contains("Xuất Xứ"));
    }

    #[test]
    fn nguon_view_shows_source_data() {
        let mut app = sample_app();
        app.dev_inspector_mode = false;
        app.explanation_lens = UserExplanationLens::Nguon;
        let text = render_text(&app);
        assert!(text.contains("nhóm xuất xứ") || text.contains("Không có dấu vết xuất xứ"));
    }

    #[test]
    fn yeu_to_lens_shows_user_facing_title() {
        let mut app = sample_app();
        app.dev_inspector_mode = false;
        app.explanation_lens = UserExplanationLens::YeuTo;
        let text = render_text(&app);
        assert!(text.contains("Yếu Tố"));
        assert!(!text.contains("Causality Map"));
    }

    #[test]
    fn default_non_dev_renders_vi_sao() {
        let mut app = sample_app();
        app.dev_inspector_mode = false;
        app.explanation_lens = UserExplanationLens::ViSao;
        let text = render_text(&app);
        assert!(text.contains("Vì Sao Kết Luận"));
        assert!(text.contains("Kết luận:"));
        assert!(text.contains("Tin cậy:"));
    }

    #[test]
    fn d_switches_to_debug_graph_mode() {
        let mut app = sample_app();
        app.dev_inspector_mode = false;
        app.explanation_lens = UserExplanationLens::ViSao;
        let text_non_dev = render_text(&app);
        assert!(text_non_dev.contains("Vì Sao Kết Luận"));

        app.dev_inspector_mode = true;
        let text_dev = render_text(&app);
        assert!(text_dev.contains("Đồ Thị Ngữ Nghĩa"));
    }

    #[test]
    fn small_layout_renders_without_panic() {
        let mut app = sample_app();
        app.dev_inspector_mode = false;
        let area = Rect::new(0, 0, 40, 15);
        let mut buf = Buffer::empty(area);
        GraphInspectorScreenWidget::new(&app, LayoutMode::Small).render(area, &mut buf);
        let text = (0..area.height)
            .map(|y| {
                (0..area.width)
                    .map(|x| buf[(x, y)].symbol())
                    .collect::<String>()
                    .trim_end()
                    .to_string()
            })
            .collect::<Vec<_>>()
            .join("\n");
        assert!(!text.is_empty());
    }
}
