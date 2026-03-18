use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::theme::Theme;
use crate::widgets::guidance::GuidanceWidget;
use crate::{layout::LayoutMode, state::AppState};

pub struct RecommendationsScreenWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> RecommendationsScreenWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for RecommendationsScreenWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let shell = Layout::vertical([Constraint::Length(1), Constraint::Min(1)]).split(area);
        Paragraph::new(Line::from(vec![
            Span::styled(
                "▶ Planning",
                Theme::accent_warn().add_modifier(Modifier::BOLD),
            ),
            Span::styled(" · Bucket khuyến nghị theo mức độ", Theme::text_dim()),
        ]))
        .render(shell[0], buf);

        // Delegate fully to GuidanceWidget, but tell app to always expand
        // actually we don't need to mutate app, we can just use GuidanceWidget
        GuidanceWidget::new(self.app, self.mode).render(shell[1], buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use ratatui::{buffer::Buffer, layout::Rect};

    #[test]
    fn recommendations_screen_renders_focus_marker() {
        let date = NaiveDate::from_ymd_opt(2026, 3, 18).expect("valid date");
        let app = AppState::new(Some(date));
        let area = Rect::new(0, 0, 120, 30);
        let mut buf = Buffer::empty(area);

        RecommendationsScreenWidget::new(&app, LayoutMode::Large).render(area, &mut buf);

        let rendered = (0..area.height)
            .map(|y| {
                (0..area.width)
                    .map(|x| buf[(x, y)].symbol())
                    .collect::<String>()
                    .trim_end()
                    .to_string()
            })
            .collect::<Vec<_>>()
            .join("\n");

        assert!(rendered.contains("▶ Planning"));
    }
}
