use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::layout::LayoutMode;
use crate::state::{AppState, PageSection};

use super::{
    calendar::CalendarViewWidget, guidance::GuidanceWidget, hero::HeroWidget,
    risk::RiskWidget, scholarly::ScholarlyWidget, tietkhi::TietKhiWidget,
    timeline::TimelineWidget, travel::TravelWidget,
};

pub struct PageWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> PageWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for PageWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.app.is_loading {
            Paragraph::new("Đang tải dữ liệu...").render(area, buf);
            return;
        }

        if let Some(err) = &self.app.error_msg {
            Paragraph::new(format!("Lỗi: {}", err)).render(area, buf);
            return;
        }

        if self.app.bundle.is_none() {
            Paragraph::new("Không có dữ liệu.").render(area, buf);
            return;
        }

        if self.app.is_calendar_view() {
            CalendarViewWidget::new(self.app, self.mode).render(area, buf);
            return;
        }

        let is_large = self.mode == LayoutMode::Large;

        if is_large && self.app.zoomed_section.is_none() {
            let cols = Layout::horizontal([
                Constraint::Percentage(50), 
                Constraint::Percentage(50)
            ])
            .margin(1)
            .split(area);

            let mut left_area = cols[0];
            left_area.width = left_area.width.saturating_sub(1); // Gap
            let mut right_area = cols[1];
            right_area.x += 1;
            right_area.width = right_area.width.saturating_sub(1);

            let left_sections = vec![
                PageSection::Hero,
                PageSection::Timing,
                PageSection::Travel,
                PageSection::TraditionalEvidence,
            ];
            let right_sections = vec![
                PageSection::Recommendations,
                PageSection::Risks,
            ];

            let left_constraints: Vec<Constraint> = left_sections.iter().enumerate().map(|(i, s)| {
                if i + 1 == left_sections.len() { Constraint::Min(section_height(self.app, self.mode, *s)) } 
                else { Constraint::Length(section_height(self.app, self.mode, *s)) }
            }).collect();
            let right_constraints: Vec<Constraint> = right_sections.iter().enumerate().map(|(i, s)| {
                if i + 1 == right_sections.len() { Constraint::Min(section_height(self.app, self.mode, *s)) } 
                else { Constraint::Length(section_height(self.app, self.mode, *s)) }
            }).collect();

            let left_chunks = Layout::vertical(left_constraints).split(left_area);
            let right_chunks = Layout::vertical(right_constraints).split(right_area);

            for (chunk, section) in left_chunks.iter().zip(left_sections.into_iter()) {
                self.render_section(section, *chunk, buf);
            }
            for (chunk, section) in right_chunks.iter().zip(right_sections.into_iter()) {
                self.render_section(section, *chunk, buf);
            }

        } else {
            let sections = if let Some(section) = self.app.zoomed_section {
                vec![section]
            } else {
                home_section_order(self.app)
            };
            let constraints: Vec<Constraint> = sections
                .iter()
                .enumerate()
                .map(|(index, section)| {
                    if index + 1 == sections.len() {
                        Constraint::Min(section_height(self.app, self.mode, *section))
                    } else {
                        Constraint::Length(section_height(self.app, self.mode, *section))
                    }
                })
                .collect();
            let chunks = Layout::vertical(constraints).split(area);

            for (chunk, section) in chunks.iter().zip(sections.into_iter()) {
                self.render_section(section, *chunk, buf);
            }
        }
    }
}

impl<'a> PageWidget<'a> {
    fn render_section(&self, section: PageSection, chunk: ratatui::layout::Rect, buf: &mut Buffer) {
        match section {
            PageSection::Hero => HeroWidget::new(self.app, self.mode).render(chunk, buf),
            PageSection::Recommendations => {
                GuidanceWidget::new(self.app, self.mode).render(chunk, buf)
            }
            PageSection::Timing => TimelineWidget::new(self.app, self.mode).render(chunk, buf),
            PageSection::Travel => TravelWidget::new(self.app, self.mode).render(chunk, buf),
            PageSection::Risks => RiskWidget::new(self.app, self.mode).render(chunk, buf),
            PageSection::TraditionalEvidence => {
                render_traditional_evidence(chunk, buf, self.app, self.mode)
            }
            PageSection::ExpandedDetails => render_placeholder_section(
                chunk,
                buf,
                section_title(section),
                &expanded_detail_lines(self.app),
            ),
        }
    }
}

pub(crate) fn home_section_order(_app: &AppState) -> Vec<PageSection> {
    vec![
        PageSection::Hero,
        PageSection::Recommendations,
        PageSection::Timing,
        PageSection::Travel,
        PageSection::Risks,
        PageSection::TraditionalEvidence,
        PageSection::ExpandedDetails,
    ]
}

fn section_height(app: &AppState, mode: LayoutMode, section: PageSection) -> u16 {
    match section {
        PageSection::Hero => 7,
        PageSection::Recommendations => match (mode, app.is_section_expanded(section)) {
            (LayoutMode::Small, true) => 12,
            (LayoutMode::Medium, true) => 11,
            (LayoutMode::Large, true) => 12,
            (LayoutMode::Small, false) => 9,
            (LayoutMode::Medium, false) => 8,
            (LayoutMode::Large, false) => 9,
        },
        PageSection::Timing => 7,
        PageSection::Travel => 5,
        PageSection::Risks => 6,
        PageSection::TraditionalEvidence => {
            if app.is_section_expanded(section) {
                12
            } else {
                8
            }
        }
        PageSection::ExpandedDetails => 4,
    }
}

fn section_title(section: PageSection) -> &'static str {
    match section {
        PageSection::Hero => "Tóm Tắt",
        PageSection::Recommendations => "Khuyến Nghị",
        PageSection::Timing => "Khung Giờ Và Hành Động",
        PageSection::Travel => "Xuất Hành Và Hướng",
        PageSection::Risks => "Rủi Ro & Kiêng Kỵ",
        PageSection::TraditionalEvidence => "Chứng Cứ Truyền Thống",
        PageSection::ExpandedDetails => "Chi Tiết Mở Rộng",
    }
}

fn render_placeholder_section(area: Rect, buf: &mut Buffer, title: &str, lines: &[String]) {
    use ratatui::widgets::{Block, Borders};
    let block = Block::default()
        .title(format!(" {} ", title))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
        
    let inner = block.inner(area);
    block.render(area, buf);
    
    let body_style = Style::default().fg(Color::White);
    let mut rendered = vec![];

    for line in lines {
        rendered.push(Line::from(vec![
            Span::raw("   "),
            Span::styled(line.clone(), body_style),
        ]));
    }

    Paragraph::new(rendered).render(inner, buf);
}

fn render_traditional_evidence(area: Rect, buf: &mut Buffer, app: &AppState, mode: LayoutMode) {
    let chunks = Layout::vertical([Constraint::Length(4), Constraint::Min(3)]).split(area);
    ScholarlyWidget::new(app, mode).render(chunks[0], buf);
    TietKhiWidget::new(app, mode).render(chunks[1], buf);
}

fn expanded_detail_lines(app: &AppState) -> Vec<String> {
    let mut lines = vec!["Chi tiết mở rộng sẽ được làm dày ở các task sau.".to_string()];
    if app.show_evidence {
        lines.push("Đang hiển thị chế độ chứng cứ.".to_string());
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use crate::state::{FocusLens, PageSection, ViewMode};

    fn sample_app_state() -> AppState {
        let date = NaiveDate::from_ymd_opt(2026, 3, 12).expect("valid date");
        AppState {
            running: true,
            date,
            lens: FocusLens::General,
            view_mode: ViewMode::Day,
            scroll_offset: 0,
            bundle: None,
            is_loading: false,
            error_msg: None,
            show_guidance_details: false,
            show_tietkhi_details: false,
            show_evidence: false,
            focused_section: PageSection::Hero,
            zoomed_section: None,
            expanded_sections: Default::default(),
            show_search: false,
            search_input: String::new(),
            calendar_cursor: date,
        }
    }

    #[test]
    fn home_screen_sections_follow_actionability_first_order() {
        let app = sample_app_state();

        assert_eq!(
            home_section_order(&app),
            vec![
                PageSection::Hero,
                PageSection::Recommendations,
                PageSection::Timing,
                PageSection::Travel,
                PageSection::Risks,
                PageSection::TraditionalEvidence,
                PageSection::ExpandedDetails,
            ]
        );
    }
}
