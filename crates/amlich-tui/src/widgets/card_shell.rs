use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

use crate::theme::Theme;

pub fn render_status_card(area: Rect, buf: &mut Buffer, title: &str, lines: Vec<Line<'static>>) {
    let popup = centered_status_rect(area);
    Clear.render(popup, buf);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Theme::border_secondary())
        .title(format!(" {title} "));

    let inner = Layout::vertical([Constraint::Min(1)]).split(block.inner(popup))[0];
    block.render(popup, buf);
    Paragraph::new(lines)
        .style(Theme::text_primary())
        .render(inner, buf);
}

fn centered_status_rect(area: Rect) -> Rect {
    let width = area.width.min(72).max(40);
    let height = area.height.min(10).max(6);
    Rect {
        x: area.x + (area.width.saturating_sub(width)) / 2,
        y: area.y + (area.height.saturating_sub(height)) / 2,
        width,
        height,
    }
}
