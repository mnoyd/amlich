use std::collections::HashMap;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use chrono::Datelike;

use crate::layout::LayoutMode;
use crate::state::AppState;

pub struct GraphInspectorScreenWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
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

        let inspection = amlich_core::debug_inspect_semantic_graph(
            day,
            month,
            year,
            self.app.show_graph_recommendations,
        );

        let rows = Layout::vertical([
            Constraint::Length(8),
            Constraint::Length(7),
            Constraint::Min(6),
            Constraint::Min(5),
        ])
        .split(area);

        render_header(&inspection, self.app.show_graph_recommendations, rows[0], buf);
        render_summary(&inspection.summary, rows[1], buf);

        let bottom = if self.mode == LayoutMode::Small {
            Layout::vertical([Constraint::Min(6), Constraint::Min(5)]).split(rows[2])
        } else {
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(rows[2])
        };
        render_cluster_counts(&inspection.cluster_counts, bottom[0], buf);
        render_semantic_kind_counts(&inspection.semantic_kind_counts, bottom[1], buf);

        if !inspection.severity_counts.is_empty() {
            render_severity_counts(&inspection.severity_counts, rows[3], buf);
        } else {
            render_node_sample(&inspection, rows[3], buf);
        }
    }
}

fn render_header(
    inspection: &amlich_core::DebugSemanticGraphInspection,
    include_recs: bool,
    area: Rect,
    buf: &mut Buffer,
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
            Span::styled(
                "[r] để bật/tắt",
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  ←/→: đổi ngày  t: hôm nay  r: toggle recommendations  1-6: đổi màn",
                Style::default().fg(Color::DarkGray),
            ),
        ]),
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
            Span::styled(
                format!("{:>5}", count),
                Style::default().fg(Color::Yellow),
            ),
        ]));
    }
    if entries.len() > 12 {
        lines.push(Line::from(vec![
            Span::styled(
                format!("  ... +{} more", entries.len() - 12),
                Style::default().fg(Color::DarkGray),
            ),
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
            Span::styled(
                format!("{:>5}", count),
                Style::default().fg(Color::Yellow),
            ),
        ]));
    }
    if entries.len() > 12 {
        lines.push(Line::from(vec![
            Span::styled(
                format!("  ... +{} more", entries.len() - 12),
                Style::default().fg(Color::DarkGray),
            ),
        ]));
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
            Span::styled(
                format!("{:>5}", count),
                Style::default().fg(Color::Yellow),
            ),
        ]));
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_node_sample(
    inspection: &amlich_core::DebugSemanticGraphInspection,
    area: Rect,
    buf: &mut Buffer,
) {
    let block = Block::default()
        .title(" Top Nodes ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let mut lines = Vec::new();
    for node in inspection.visualization.nodes.iter().take(8) {
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(
                format!("{:<20}", truncate_label(&node.node_id, 20)),
                Style::default().fg(Color::White),
            ),
            Span::styled(
                format!("{:<16}", truncate_label(&node.cluster, 16)),
                Style::default().fg(Color::Cyan),
            ),
            Span::styled(
                truncate_label(&node.semantic_kind, 14),
                Style::default().fg(Color::DarkGray),
            ),
        ]));
    }
    let remaining = inspection.visualization.nodes.len().saturating_sub(8);
    if remaining > 0 {
        lines.push(Line::from(vec![
            Span::styled(
                format!("  ... +{} more nodes", remaining),
                Style::default().fg(Color::DarkGray),
            ),
        ]));
    }

    if lines.is_empty() {
        lines.push(Line::from(Span::styled(
            "  Không có node nào.",
            Style::default().fg(Color::DarkGray),
        )));
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn truncate_label(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}…", &s[..max_len.saturating_sub(1)])
    }
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
        assert!(text.contains("Tổng nodes:"));
        assert!(text.contains("Tổng edges:"));
        assert!(text.contains("Cluster Counts"));
        assert!(text.contains("Semantic Kind Counts"));
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
}
