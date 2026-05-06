use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Position, Rect},
    widgets::{Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget},
    Frame,
};

use crate::state::AppState;
use crate::widgets::{
    context::ContextModalWidget, help::HelpModalWidget, page::render_screen_content,
    page::screen_natural_height, page::PageWidget, ribbon::RibbonWidget,
    search::SearchOverlayWidget, week_strip::WeekStripWidget,
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

pub fn draw(frame: &mut Frame, app: &mut AppState) {
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

    let mode = layout_mode(size.width);
    let content_area = page_content_area(page_area, mode);

    // Non-scrollable views: loading, error, no data, calendar
    let needs_scroll = app.bundle.is_some()
        && !app.is_loading
        && app.error_msg.is_none()
        && !app.is_calendar_view();

    if needs_scroll {
        // --- Scrollable content path ---

        // 1. Render week strip directly to frame (fixed, not scrolled)
        let screen_area = if app.show_week_strip {
            let chunks =
                Layout::vertical([Constraint::Length(4), Constraint::Min(1)]).split(content_area);
            frame.render_widget(WeekStripWidget::new(app), chunks[0]);
            chunks[1]
        } else {
            content_area
        };

        // 2. Create virtual buffer sized to the screen's natural content height
        let viewport_h = screen_area.height;
        let natural_h = screen_natural_height(app, mode, screen_area.width);
        let virtual_h = viewport_h.max(natural_h);
        let virtual_rect = Rect::new(0, 0, screen_area.width, virtual_h);
        let mut virtual_buf = Buffer::empty(virtual_rect);

        // 3. Render screen content into the virtual buffer
        render_screen_content(app, mode, virtual_rect, &mut virtual_buf);

        // 4. Detect actual content height and clamp scroll
        let content_h = detect_content_height(&virtual_buf, virtual_rect);
        app.content_height = content_h;
        app.viewport_height = viewport_h;
        app.clamp_scroll();

        // 5. Blit visible rows from virtual buffer to frame
        let scroll = app.scroll_offset;
        let frame_buf = frame.buffer_mut();
        for y in 0..viewport_h {
            let src_y = y + scroll;
            if src_y >= virtual_h {
                break;
            }
            for x in 0..screen_area.width {
                let src_cell = &virtual_buf[(x, src_y)];
                if let Some(target) =
                    frame_buf.cell_mut(Position::new(screen_area.x + x, screen_area.y + y))
                {
                    target
                        .set_symbol(src_cell.symbol())
                        .set_style(src_cell.style());
                    if src_cell.skip {
                        target.set_skip(true);
                    }
                }
            }
        }

        // 6. Render scrollbar if content overflows
        if content_h > viewport_h {
            let mut scrollbar_state = vertical_scrollbar_state(content_h, viewport_h, scroll);
            Scrollbar::new(ScrollbarOrientation::VerticalRight).render(
                screen_area,
                frame.buffer_mut(),
                &mut scrollbar_state,
            );
        }
    } else {
        // --- Non-scrollable path (loading / error / calendar) ---
        let page_render_area = if app.is_calendar_view() {
            page_area
        } else {
            content_area
        };
        frame.render_widget(PageWidget::new(app, mode), page_render_area);
    }

    // Render the fixed ribbon at the bottom
    frame.render_widget(RibbonWidget::new(app, mode), ribbon_area);

    // Render overlays (Search, Context, Help) on top if active
    match app.app_mode {
        crate::state::AppMode::SearchModal => {
            frame.render_widget(SearchOverlayWidget::new(app, mode), size);
        }
        crate::state::AppMode::ContextModal => {
            frame.render_widget(ContextModalWidget::new(app, mode), size);
        }
        crate::state::AppMode::PersonalProfileModal => {
            frame.render_widget(
                crate::widgets::personal_profile::PersonalProfileModalWidget::new(app, mode),
                size,
            );
        }
        crate::state::AppMode::HelpModal => {
            frame.render_widget(HelpModalWidget::new(app, mode), size);
        }
        _ => {}
    }
}

/// Scan from the bottom of the virtual buffer to find the last row with non-empty content.
fn detect_content_height(buf: &Buffer, area: Rect) -> u16 {
    for y in (0..area.height).rev() {
        for x in 0..area.width {
            if buf[(x, y)].symbol() != " " {
                return y + 1;
            }
        }
    }
    0
}

fn vertical_scrollbar_state(content_h: u16, viewport_h: u16, scroll: u16) -> ScrollbarState {
    ScrollbarState::new(content_h as usize)
        .viewport_content_length(viewport_h as usize)
        .position(scroll as usize)
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
    use crate::state::{AppMode, ExplorerAction, ExplorerField, ExplorerSelection, PageSection};
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

            scroll_offset: 0,
            content_height: 0,
            viewport_height: 0,
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
                upcoming_events: vec![],
            }),
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
            active_view: crate::state::ActiveView::Today,
            view_history: Vec::new(),
            graph_inspector_focus: crate::state::GraphInspectorFocus::Summary,
            graph_inspector_cursor: 0,
            graph_inspector_search_query: String::new(),
            graph_inspector_search_cursor: 0,
            graph_inspector_focus_before_search: None,
            graph_inspector_lens: crate::state::GraphInspectorLens::General,
            dev_inspector_mode: false,
            explanation_lens: crate::state::UserExplanationLens::ViSao,
            causality_focus: crate::state::CausalityFocus::SummaryList,
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

    #[test]
    fn scrollbar_state_uses_total_content_and_viewport_length() {
        let state = vertical_scrollbar_state(25, 20, 3);

        assert_eq!(
            format!("{state:?}"),
            "ScrollbarState { content_length: 25, position: 3, viewport_content_length: 20 }"
        );
    }
}
