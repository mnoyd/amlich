use ratatui::{
    layout::{Constraint, Layout},
    Frame,
};

use crate::state::AppState;
use crate::widgets::{page::PageWidget, ribbon::RibbonWidget, search::SearchOverlayWidget};

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
    let page_render_area = if app.is_calendar_view() {
        page_area
    } else {
        content_area
    };

    // Render the main page widget within the centered content area or fullscreen calendar overlay.
    frame.render_widget(PageWidget::new(app, mode), page_render_area);

    // Render the fixed ribbon at the bottom
    frame.render_widget(RibbonWidget::new(app, mode), ribbon_area);

    // Render overlays (Search, etc) on top if active
    if app.app_mode == crate::state::AppMode::SearchModal {
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
    use crate::state::{
        AppMode, ExplorerAction, ExplorerField, ExplorerSelection, FocusLens, PageSection,
    };
    use amlich_api::v2::DayBundleDto;
    use amlich_api::{
        RecommendationPackCatalogEntryDto, RulesetCatalogEntryDto, RulesetDefaultsDto,
    };
    use chrono::NaiveDate;

    fn sample_app_state() -> AppState {
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
            app_mode: AppMode::Normal,
            date,
            lens: FocusLens::General,

            scroll_offset: 0,
            bundle: Some(DayBundleDto {
                schema_version: "amlich.engine/v1".to_string(),
                ruleset_id: "test".to_string(),
                ruleset_version: "v1".to_string(),
                profile: "baseline".to_string(),
                generated_at: "2026-03-12T00:00:00Z".to_string(),
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
                contextual_recommendations: None,
                insight: None,
            }),
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
            focused_section: PageSection::Hero,
            zoomed_section: None,
            expanded_sections: Default::default(),

            search_input: String::new(),
            calendar_cursor: date,
            navigation_history: Vec::new(),
            active_view: crate::state::ActiveView::Dashboard,
            view_history: Vec::new(),
        }
    }

    #[test]
    fn layout_modes_preserve_actionability_first_order() {
        let app = sample_app_state();
        let expected = vec![
            PageSection::Explorer,
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
