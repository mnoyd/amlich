use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

use crate::theme;

pub struct HelpOverlay;

impl HelpOverlay {
    pub fn new() -> Self {
        Self
    }
}

impl Widget for HelpOverlay {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Center the overlay
        let width = 60;
        let height = 28;
        let x = area.x + (area.width.saturating_sub(width)) / 2;
        let y = area.y + (area.height.saturating_sub(height)) / 2;
        let overlay_area = Rect::new(x, y, width, height);

        // Clear background
        Clear.render(overlay_area, buf);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme::ACCENT_FG))
            .title(Line::from(vec![Span::styled(
                " ⌨️  Keyboard Shortcuts ",
                theme::title_style(),
            )]))
            .title_bottom(Line::from(" ? or Esc to close ").alignment(Alignment::Center));

        let mut lines = Vec::new();

        // Section headers and shortcuts
        let sections = vec![
            (
                "Navigation",
                vec![
                    ("←↓↑→ / hjkl", "di chuyển ngày"),
                    ("n / p", "tháng trước/sau"),
                    ("N / P", "năm trước/sau"),
                    ("t", "hôm nay"),
                    ("[ / ]", "lịch sử trước/sau"),
                ],
            ),
            (
                "Jump & Search",
                vec![
                    ("g", "nhảy đến ngày (dd/mm/yyyy)"),
                    ("/", "tìm kiếm ngày lễ"),
                    ("Tab", "chuyển kết quả tìm"),
                    ("b / B", "bookmark / danh sách"),
                ],
            ),
            (
                "Display",
                vec![
                    ("H", "danh sách ngày lễ"),
                    ("i", "bật/tắt insight"),
                    ("L", "đổi ngôn ngữ VI/EN"),
                ],
            ),
            (
                "General",
                vec![
                    ("q / Esc", "thoát"),
                    ("Ctrl+C", "thoát tức thì"),
                    ("?", "trợ giúp"),
                ],
            ),
        ];

        for (section_name, shortcuts) in sections {
            lines.push(Line::from(vec![Span::styled(
                section_name,
                Style::default()
                    .fg(theme::TITLE_FG)
                    .add_modifier(Modifier::BOLD),
            )]));
            lines.push(Line::from(""));

            for (key, desc) in shortcuts {
                lines.push(Line::from(vec![
                    Span::styled(
                        format!("  {:12} ", key),
                        Style::default()
                            .fg(theme::ACCENT_FG)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(desc, Style::default().fg(theme::VALUE_FG)),
                ]));
            }
            lines.push(Line::from(""));
        }

        let p = Paragraph::new(lines).block(block);
        p.render(overlay_area, buf);
    }
}

impl Default for HelpOverlay {
    fn default() -> Self {
        Self::new()
    }
}
