use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

use crate::layout::LayoutMode;
use crate::state::AppState;
use crate::theme::Theme;

pub struct HelpModalWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> HelpModalWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for HelpModalWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.app.app_mode != crate::state::AppMode::HelpModal {
            return;
        }

        let popup_width = 70;
        let popup_height = 25;

        let x = area.x + (area.width.saturating_sub(popup_width)) / 2;
        let y = area.y + (area.height.saturating_sub(popup_height)) / 2;

        let popup_area = Rect::new(x, y, popup_width, popup_height);

        Clear.render(popup_area, buf);

        let block = Block::default()
            .title(" Trợ Giúp (Phím Tắt) ")
            .title_alignment(ratatui::layout::Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Theme::secondary_border());

        let lines = vec![
            Line::from(Span::styled(
                "Điều hướng trang:",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Theme::GOLD),
            )),
            Line::from("  [1] Hôm Nay"),
            Line::from("  [2] Chi Tiết Ngày"),
            Line::from("  [3] Giờ Tốt"),
            Line::from("  [4] hoặc [c] Lịch"),
            Line::from("  [5] Cá Nhân"),
            Line::from(""),
            Line::from(Span::styled(
                "Điều hướng thời gian:",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Theme::GOLD),
            )),
            Line::from("  [h]/[l], [Left]/[Right] : Lùi/Tiến 1 ngày"),
            Line::from("  [j]/[k], [Down]/[Up]    : Lùi/Tiến 1 tuần"),
            Line::from("  [H]/[L], [p]/[n]        : Lùi/Tiến 1 tháng"),
            Line::from("  [t]                     : Hôm nay"),
            Line::from("  [u]                     : Hoàn tác (Undo)"),
            Line::from("  [/] hoặc [s]            : Tìm kiếm ngày"),
            Line::from(""),
            Line::from(Span::styled(
                "Tính năng khác:",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Theme::GOLD),
            )),
            Line::from("  [o] : Mở bộ lọc hồ sơ (Context Modal)"),
            Line::from("  [y] : Sao chép thông tin (Yank)"),
            Line::from("  [e] : Hiện/Ẩn bằng chứng (Evidence)"),
            Line::from("  [w] : Hiện/Ẩn thanh tuần (Week strip)"),
            Line::from("  [?] : Hiện/Ẩn Trợ giúp này"),
            Line::from("  [q] : Thoát ứng dụng"),
        ];

        Paragraph::new(lines).block(block).render(popup_area, buf);
    }
}
