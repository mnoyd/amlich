use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap},
};

use crate::{
    layout::LayoutMode,
    state::{AppState, PersonalField},
};

pub struct PersonalProfileModalWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> PersonalProfileModalWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for PersonalProfileModalWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup = Rect::new(
            area.x + (area.width.saturating_sub(64)) / 2,
            area.y + (area.height.saturating_sub(17)) / 2,
            64,
            17,
        );

        Clear.render(popup, buf);

        let block = Block::default()
            .title(" Hồ Sơ Cá Nhân ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(popup);
        block.render(popup, buf);

        let year_style = if self.app.personal_focus == PersonalField::BirthYear {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        let month_style = if self.app.personal_focus == PersonalField::BirthMonth {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        let day_style = if self.app.personal_focus == PersonalField::BirthDay {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        let gender_style = if self.app.personal_focus == PersonalField::Gender {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let gender_label = match self.app.personal_draft.gender {
            Some(amlich_core::almanac::tu_menh::Gender::Male) => "Nam",
            Some(amlich_core::almanac::tu_menh::Gender::Female) => "Nữ",
            None => "Chưa chọn",
        };

        let lines = vec![
            Line::from("Nhập hồ sơ tối thiểu để mở Tứ Mệnh, hướng hợp và Đại Vận."),
            Line::from(""),
            Line::from(vec![
                Span::styled("Năm sinh: ", year_style),
                Span::raw(if self.app.personal_draft.birth_year.is_empty() {
                    "____".to_string()
                } else {
                    self.app.personal_draft.birth_year.clone()
                }),
            ]),
            Line::from(vec![
                Span::styled("Tháng sinh: ", month_style),
                Span::raw(if self.app.personal_draft.birth_month.is_empty() {
                    "__".to_string()
                } else {
                    self.app.personal_draft.birth_month.clone()
                }),
            ]),
            Line::from(vec![
                Span::styled("Ngày sinh: ", day_style),
                Span::raw(if self.app.personal_draft.birth_day.is_empty() {
                    "__".to_string()
                } else {
                    self.app.personal_draft.birth_day.clone()
                }),
            ]),
            Line::from(vec![
                Span::styled("Giới tính: ", gender_style),
                Span::raw(gender_label),
            ]),
            Line::from(""),
            Line::from("Tab/h/l: đổi trường · j/k: đổi giới tính"),
            Line::from("Tháng/ngày có thể để trống nếu chỉ cần Tứ Mệnh cơ bản."),
            Line::from("Enter: áp dụng · Esc: hủy"),
        ];

        Paragraph::new(lines)
            .wrap(Wrap { trim: true })
            .render(inner, buf);
    }
}
