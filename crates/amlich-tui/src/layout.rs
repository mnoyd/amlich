use ratatui::{
    layout::{Constraint, Layout},
    Frame,
};

use crate::state::AppState;
use crate::widgets::{
    page::PageWidget,
    ribbon::RibbonWidget,
    search::SearchOverlayWidget,
};

const MIN_TERM_W: u16 = 40;
const MIN_TERM_H: u16 = 15;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutMode {
    Small,  // < 60 cols (Phone/Tiny pane)
    Medium, // 60-100 cols
    Large,  // > 100 cols (Desktop full)
}

pub fn layout_mode(width: u16) -> LayoutMode {
    if width < 60 {
        LayoutMode::Small
    } else if width < 100 {
        LayoutMode::Medium
    } else {
        LayoutMode::Large
    }
}

pub fn draw(frame: &mut Frame, app: &AppState) {
    let size = frame.area();

    // Enforce minimum terminal size
    if size.width < MIN_TERM_W || size.height < MIN_TERM_H {
        use ratatui::layout::Alignment;
        use ratatui::widgets::{Block, Borders, Paragraph};

        let msg = Paragraph::new("Terminal quá nhỏ.\nCần tối thiểu 40×15.")
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(msg, size);
        return;
    }

    // Main vertical layout: Page area (scrollable) + Ribbon area (fixed bottom)
    let main_layout = Layout::vertical([
        Constraint::Min(10),   // Main scrolling page
        Constraint::Length(2), // Fixed bottom ribbon (includes top padding line)
    ])
    .split(size);

    let page_area = main_layout[0];
    let ribbon_area = main_layout[1];

    // Determine the layout constraints based on mode
    let mode = layout_mode(size.width);

    let content_area = page_content_area(page_area, mode);

    // Render the main page widget within the content area
    frame.render_widget(PageWidget::new(app, mode), content_area);

    // Render the fixed ribbon at the bottom
    frame.render_widget(RibbonWidget::new(app, mode), ribbon_area);

    // Render overlays (Search, etc) on top if active
    if app.show_search {
        frame.render_widget(SearchOverlayWidget::new(app, mode), size);
    }
}

fn page_content_area(page_area: ratatui::layout::Rect, mode: LayoutMode) -> ratatui::layout::Rect {
    match mode {
        LayoutMode::Large => {
            let max_width = page_area.width.min(120);
            let pad = (page_area.width.saturating_sub(max_width)) / 2;
            Layout::horizontal([
                Constraint::Length(pad),
                Constraint::Length(max_width),
                Constraint::Length(pad),
            ])
            .split(page_area)[1]
        }
        _ => {
            let pad = if mode == LayoutMode::Small { 1 } else { 2 };
            Layout::horizontal([
                Constraint::Length(pad),
                Constraint::Min(10),
                Constraint::Length(pad),
            ])
            .split(page_area)[1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amlich_api::v2::{ApiMetaDto, DayBundleDto};
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
            bundle: Some(DayBundleDto {
                meta: ApiMetaDto {
                    schema_version: "amlich.api/v2".to_string(),
                    ruleset_id: "test".to_string(),
                    ruleset_version: "v1".to_string(),
                    profile: "baseline".to_string(),
                    generated_at: "2026-03-12T00:00:00Z".to_string(),
                },
                solar: amlich_api::SolarDto {
                    day: 12,
                    month: 3,
                    year: 2026,
                    day_of_week: 4,
                    day_of_week_name: "Thứ Năm".to_string(),
                    date_string: "2026-03-12".to_string(),
                },
                lunar: amlich_api::LunarDto {
                    day: 4,
                    month: 2,
                    year: 2026,
                    is_leap_month: false,
                    date_string: "Mùng 4 tháng Hai".to_string(),
                },
                jd: 0,
                canchi: None,
                tiet_khi: None,
                gio_hoang_dao: None,
                day_fortune: None,
                daily_recommendations: None,
                insight: None,
            }),
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
    fn layout_modes_preserve_actionability_first_order() {
        let app = sample_app_state();
        let expected = vec![
            PageSection::Hero,
            PageSection::Recommendations,
            PageSection::Timing,
            PageSection::Travel,
            PageSection::Risks,
            PageSection::TraditionalEvidence,
            PageSection::ExpandedDetails,
        ];

        assert_eq!(crate::widgets::page::home_section_order(&app), expected);
        assert_eq!(layout_mode(50), LayoutMode::Small);
        assert_eq!(layout_mode(80), LayoutMode::Medium);
        assert_eq!(layout_mode(140), LayoutMode::Large);
    }

    #[test]
    fn large_layout_uses_internal_density_without_calendar_dominance() {
        let rect = ratatui::layout::Rect::new(0, 0, 160, 40);
        let content = page_content_area(rect, LayoutMode::Large);

        assert_eq!(content.width, 120);
        assert_eq!(content.x, 20);
    }
}
