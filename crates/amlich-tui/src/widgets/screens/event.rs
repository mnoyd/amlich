use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct EventScreenWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> EventScreenWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for EventScreenWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Some(bundle) = &self.app.bundle else {
            return;
        };

        let block = Block::default()
            .title(" Thông tin Sự kiện ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));
        let inner = block.inner(area);
        block.render(area, buf);

        let mut lines = vec![];

        let mut has_event = false;

        if let Some(insight) = &bundle.insight {
            if let Some(festival) = &insight.festival {
                has_event = true;
                lines.push(Line::from(vec![Span::styled(
                    festival.names.vi.join(" / "),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )]));
                lines.push(Line::from(""));

                if let Some(origin) = &festival.origin {
                    lines.push(Line::from(Span::styled(
                        "Nguồn gốc & Ý nghĩa:",
                        Style::default().fg(Color::Cyan),
                    )));
                    lines.push(Line::from(origin.vi.clone()));
                    lines.push(Line::from(""));
                }

                if let Some(activities) = &festival.activities {
                    lines.push(Line::from(Span::styled(
                        "Nên làm:",
                        Style::default().fg(Color::Green),
                    )));
                    for act in &activities.vi {
                        lines.push(Line::from(format!("• {}", act)));
                    }
                    lines.push(Line::from(""));
                }

                if !festival.taboos.is_empty() {
                    lines.push(Line::from(Span::styled(
                        "Kiêng kỵ:",
                        Style::default().fg(Color::LightRed),
                    )));
                    for taboo in &festival.taboos {
                        lines.push(Line::from(format!(
                            "• {}: {}",
                            taboo.action.vi, taboo.reason.vi
                        )));
                    }
                    lines.push(Line::from(""));
                }
            } else if let Some(holiday) = &insight.holiday {
                has_event = true;
                lines.push(Line::from(vec![Span::styled(
                    holiday.names.vi.join(" / "),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )]));
                lines.push(Line::from(""));

                if let Some(origin) = &holiday.origin {
                    lines.push(Line::from(Span::styled(
                        "Nguồn gốc & Ý nghĩa:",
                        Style::default().fg(Color::Cyan),
                    )));
                    lines.push(Line::from(origin.vi.clone()));
                    lines.push(Line::from(""));
                }

                if let Some(activities) = &holiday.activities {
                    lines.push(Line::from(Span::styled(
                        "Nên làm:",
                        Style::default().fg(Color::Green),
                    )));
                    for act in &activities.vi {
                        lines.push(Line::from(format!("• {}", act)));
                    }
                    lines.push(Line::from(""));
                }
            }
        }

        if !has_event {
            if bundle.lunar.day == 1 {
                lines.push(Line::from(vec![Span::styled(
                    "Mùng Một (Ngày Sóc)",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )]));
                lines.push(Line::from(""));
                lines.push(Line::from("Theo phong tục truyền thống, ngày mùng 1 đầu tháng âm lịch (ngày Sóc) là ngày tưởng nhớ tổ tiên, cầu bình an."));
                lines.push(Line::from(
                    "Nên làm: thắp hương, dọn dẹp ban thờ, ăn chay, làm việc thiện.",
                ));
                lines.push(Line::from("Kiêng kỵ: sát sinh, nói lời không hay, cãi vã."));
            } else if bundle.lunar.day == 15 {
                lines.push(Line::from(vec![Span::styled(
                    "Ngày Rằm (Ngày Vọng)",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )]));
                lines.push(Line::from(""));
                lines.push(Line::from(
                    "Ngày Rằm là ngày mặt trăng tròn nhất trong tháng, mang ý nghĩa viên mãn.",
                ));
                lines.push(Line::from(
                    "Nên làm: cúng bái tổ tiên, đi chùa lễ Phật, ăn chay, phóng sinh.",
                ));
                lines.push(Line::from("Kiêng kỵ: sát sinh, mâu thuẫn gia đình."));
            }
        }

        Paragraph::new(lines)
            .wrap(Wrap { trim: true })
            .render(inner, buf);
    }
}
