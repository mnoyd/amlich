use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::layout::LayoutMode;
use crate::state::AppState;

pub struct HeaderTabsWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> HeaderTabsWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for HeaderTabsWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.app.is_calendar_view() {
            let line = Line::from(vec![Span::styled(
                " Lịch ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )]);
            Paragraph::new(line)
                .alignment(Alignment::Center)
                .render(area, buf);
            return;
        }

        let available = self.app.available_views();
        let mut view_spans = vec![];
        for (i, v) in available.iter().enumerate() {
            let index = i + 1;
            let label = match self.mode {
                LayoutMode::Small => {
                    if v != &self.app.active_view {
                        format!(
                            " {}.{} ",
                            index,
                            v.short_label().chars().take(3).collect::<String>()
                        )
                    } else {
                        format!("[{}.{}]", index, v.short_label())
                    }
                }
                LayoutMode::Medium => {
                    if v == &self.app.active_view {
                        format!(" [{}:{}] ", index, v.short_label())
                    } else {
                        format!(" {}:{} ", index, v.short_label())
                    }
                }
                LayoutMode::Large => {
                    if v == &self.app.active_view {
                        format!(" [{}:{}] ", index, v.label())
                    } else {
                        format!(" {}:{} ", index, v.label())
                    }
                }
            };

            let style = if v == &self.app.active_view {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            view_spans.push(Span::styled(label, style));
        }

        use crate::state::ui_prefs::VerbosityMode;
        let verbosity_label = match self.app.active_verbosity() {
            VerbosityMode::Compact => "Compact",
            VerbosityMode::Verbose => "Verbose",
        };
        view_spans.push(Span::styled(
            format!("  [v: {verbosity_label}]"),
            Style::default().fg(Color::DarkGray),
        ));

        let top_line = Line::from(view_spans);

        Paragraph::new(top_line)
            .alignment(Alignment::Center)
            .render(area, buf);
    }
}
