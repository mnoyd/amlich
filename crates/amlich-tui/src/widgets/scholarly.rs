use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use amlich_api::v2::DayBundleDto;
use crate::layout::LayoutMode;
use crate::state::AppState;

pub struct ScholarlyWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> ScholarlyWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for ScholarlyWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Some(bundle) = &self.app.bundle else { return };

        let chunks = Layout::vertical([
            Constraint::Length(7), // Trạch Cát (Stars/Deities)
            Constraint::Length(1), // pad
            Constraint::Length(5), // Phương Vị (Directions)
        ])
        .split(area);

        self.render_trach_cat(chunks[0], buf, bundle);
        self.render_phuong_vi(chunks[2], buf, bundle);
    }
}

impl<'a> ScholarlyWidget<'a> {
    fn render_trach_cat(&self, area: Rect, buf: &mut Buffer, bundle: &DayBundleDto) {
        let mut lines = vec![];
        let header_style = Style::default().fg(Color::DarkGray);

        lines.push(Line::from(vec![
            Span::styled("── Tinh Tú & Thần Sát ", header_style),
            Span::styled(format!("{:─<50}", ""), header_style),
        ]));

        if let Some(insight) = &bundle.insight {
            // Trực
            if let Some(truc) = &insight.truc {
                lines.push(Line::from(vec![
                    Span::raw("   Trực:     "),
                    Span::styled(&truc.name, Style::default().fg(Color::Cyan)),
                    Span::raw(" ("),
                    Span::raw(&truc.quality),
                    Span::raw(")"),
                ]));
            } else {
                lines.push(Line::from("   Trực:     ---"));
            }

            // Stars
            if let Some(stars) = &insight.stars {
                let cat_tinh = stars.cat_tinh.join(", ");
                let cat_str = if stars.cat_tinh.is_empty() { "Không" } else { &cat_tinh };
                lines.push(Line::from(vec![
                    Span::raw("   Cát Tinh: "),
                    Span::styled(cat_str.to_string(), Style::default().fg(Color::Green)),
                ]));

                let sat_tinh = stars.sat_tinh.join(", ");
                let sat_str = if stars.sat_tinh.is_empty() { "Không" } else { &sat_tinh };
                lines.push(Line::from(vec![
                    Span::raw("   Sát Tinh: "),
                    Span::styled(sat_str.to_string(), Style::default().fg(Color::Red)),
                ]));
            }
            
            // Deity
            if let Some(deity) = &insight.day_deity {
                lines.push(Line::from(vec![
                    Span::raw("   Thần:     "),
                    Span::styled(&deity.name, Style::default().fg(Color::Yellow)),
                    Span::raw(" ("),
                    Span::raw(&deity.classification),
                    Span::raw(")"),
                ]));
            }
        } else {
            lines.push(Line::from("   (Không có dữ liệu Tinh Tú)"));
        }

        Paragraph::new(lines).render(area, buf);
    }

    fn render_phuong_vi(&self, area: Rect, buf: &mut Buffer, bundle: &DayBundleDto) {
        let mut lines = vec![];
        let header_style = Style::default().fg(Color::DarkGray);

        lines.push(Line::from(vec![
            Span::styled("── Phương Vị Xuất Hành ", header_style),
            Span::styled(format!("{:─<48}", ""), header_style),
        ]));

        if let Some(fortune) = &bundle.day_fortune {
            let travel = &fortune.travel;
            let col_width = if self.mode == LayoutMode::Small { 0 } else { 35 };

            if self.mode == LayoutMode::Small {
                lines.push(Line::from(format!("   Hỷ Thần:  {}", travel.hy_than)));
                lines.push(Line::from(format!("   Tài Thần: {}", travel.tai_than)));
                lines.push(Line::from(format!("   Hạc Thần: {}", travel.xuat_hanh_huong))); // Simplified
            } else {
                lines.push(Line::from(vec![
                    Span::raw(format!("   Hỷ Thần:  {:<width$}", travel.hy_than, width=col_width-13)),
                    Span::raw(format!("Tài Thần: {}", travel.tai_than)),
                ]));
                lines.push(Line::from(vec![
                    Span::raw(format!("   Hạc Thần: {:<width$}", "(Tránh)", width=col_width-13)),
                    Span::raw(format!("Sát Hướng: {}", fortune.conflict.sat_huong)),
                ]));
            }
        } else {
            lines.push(Line::from("   (Không có dữ liệu Xuất Hành)"));
        }

        Paragraph::new(lines).render(area, buf);
    }
}
