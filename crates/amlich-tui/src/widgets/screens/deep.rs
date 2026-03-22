use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct DeepScreenWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> DeepScreenWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for DeepScreenWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Màn hình Deep ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let mut lines: Vec<Line<'_>> = vec![Line::from(vec![Span::styled(
            "Lý giải sâu theo Can Chi · Khí tiết · Sao số",
            Style::default().fg(Color::Cyan),
        )])];

        if let Some(bundle) = self.app.bundle.as_ref() {
            if let Some(fortune) = bundle.day_fortune.as_ref() {
                lines.push(Line::from(format!(
                    "Trực {} ({})",
                    fortune.truc.name, fortune.truc.quality
                )));
                lines.push(Line::from(format!(
                    "Xung-hợp: lục xung {} · sát hướng {}",
                    fortune.xung_hop.luc_xung, fortune.conflict.sat_huong
                )));
            }

            if let Some(insight) = bundle.insight.as_ref() {
                if let Some(ten_gods) = insight.ten_gods.as_ref() {
                    if let Some(to_year) = ten_gods.to_year_stem.as_ref() {
                        lines.push(Line::from(format!(
                            "Thập thần (năm): {} · {}",
                            to_year.label, to_year.meaning.vi
                        )));
                    }
                    if let Some(to_self) = ten_gods.to_self.as_ref() {
                        lines.push(Line::from(format!(
                            "Thập thần (nhật chủ): {} · {}",
                            to_self.label, to_self.meaning.vi
                        )));
                    }
                }

                if let Some(dai_van) = insight.dai_van.as_ref() {
                    lines.push(Line::from(format!(
                        "Đại vận: {} · bắt đầu {}",
                        dai_van.direction, dai_van.start_age
                    )));
                    if let Some(current) = dai_van.current_pillar.as_ref() {
                        lines.push(Line::from(format!(
                            "Trụ hiện tại: {} ({}-{})",
                            current.can_chi, current.start_age, current.end_age
                        )));
                    }
                }
            }

            for row in self.app.top_recommendation_rows() {
                lines.push(Line::from(""));
                lines.push(Line::from(vec![Span::styled(
                    row.label,
                    Style::default().fg(Color::Yellow),
                )]));
                for reason in row.reason_details.into_iter().take(2) {
                    lines.push(Line::from(format!("  ↳ {}", reason)));
                }
            }
        }

        if lines.len() == 1 {
            lines.push(Line::from("Chưa đủ dữ liệu để mở lớp phân tích sâu."));
        }

        Paragraph::new(lines)
            .wrap(Wrap { trim: true })
            .render(inner, buf);
    }
}
