use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Clear, Paragraph, Widget},
};

use crate::layout::LayoutMode;
use crate::state::AppState;
use crate::theme::Theme;
use crate::widgets::modal_shell::{centered_modal, modal_block, ModalPreset};

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

        let popup_area = centered_modal(area, self._mode, ModalPreset::Help);

        Clear.render(popup_area, buf);

        let block = modal_block(" Trợ Giúp (Phím Tắt) ");

        let lines = vec![
            Line::from(Span::styled(
                "Điều hướng trang:",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Theme::WARN),
            )),
            Line::from("  [1] Bảng điều khiển (Dashboard)"),
            Line::from("  [2] Chuyên gia (Scholar)"),
            Line::from("  [3] Giờ Tốt (Hours)"),
            Line::from("  [4] Ngũ Hành (Elements)"),
            Line::from("  [5] Phong Thủy (FengShui)"),
            Line::from("  [6] Tiết Khí (SolarTerms)"),
            Line::from("  [7] Kế hoạch (Planning)"),
            Line::from("  [8] Lịch tháng (Calendar)"),
            Line::from("  [Tab]/[Shift+Tab] Chuyển màn theo vòng"),
            Line::from(""),
            Line::from(Span::styled(
                "Điều hướng thời gian:",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Theme::WARN),
            )),
            Line::from("  [h]/[l], [Left]/[Right] : Lùi/Tiến 1 ngày"),
            Line::from("  [j]/[k], [Down]/[Up]    : Cuộn nội dung"),
            Line::from("  [H]/[L], [p]/[n], [[]]  : Lùi/Tiến 1 tháng"),
            Line::from("  [t]                     : Hôm nay"),
            Line::from("  [u]                     : Hoàn tác (Undo)"),
            Line::from("  [/] hoặc [s]            : Tìm kiếm ngày"),
            Line::from(""),
            Line::from(Span::styled(
                "Tính năng khác:",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Theme::WARN),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::AppMode;
    use chrono::NaiveDate;
    use ratatui::{buffer::Buffer, layout::Rect};

    #[test]
    fn help_modal_reflects_expanded_screen_shortcuts() {
        let mut app = AppState::new(NaiveDate::from_ymd_opt(2026, 3, 18));
        app.app_mode = AppMode::HelpModal;
        let area = Rect::new(0, 0, 120, 40);
        let mut buf = Buffer::empty(area);

        HelpModalWidget::new(&app, LayoutMode::Large).render(area, &mut buf);

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

        assert!(rendered.contains("[8] Lịch tháng (Calendar)"));
    }
}
