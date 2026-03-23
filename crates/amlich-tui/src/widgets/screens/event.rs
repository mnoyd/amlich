use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::{layout::LayoutMode, state::{ui_prefs::VerbosityMode, AppState}};

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
        let compact = matches!(self.app.active_verbosity(), VerbosityMode::Compact);

        let mut has_event = false;

        if let Some(insight) = &bundle.insight {
            if let Some(festival) = &insight.festival {
                has_event = true;
                lines.push(Line::from(vec![Span::styled(
                    festival.names.vi.join(" / "),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )]));
                lines.push(Line::from(vec![
                    Span::styled("Phân loại: ", Style::default().fg(Color::Yellow)),
                    Span::raw(format!(
                        "{}{}",
                        festival.category,
                        if festival.is_major {
                            " · lễ lớn"
                        } else {
                            ""
                        }
                    )),
                ]));
                lines.push(Line::from(""));

                if let Some(origin) = &festival.origin {
                    if compact {
                        lines.push(Line::from(Span::styled(
                            "Ý nghĩa:",
                            Style::default().fg(Color::Cyan),
                        )));
                        lines.push(Line::from(first_sentence(&origin.vi)));
                        lines.push(Line::from(""));
                    } else {
                    lines.push(Line::from(Span::styled(
                        "Nguồn gốc & Ý nghĩa:",
                        Style::default().fg(Color::Cyan),
                    )));
                    lines.push(Line::from(origin.vi.clone()));
                    lines.push(Line::from(""));
                    }
                }

                if let Some(activities) = &festival.activities {
                    let limit = if compact { 2 } else { activities.vi.len() };
                    lines.push(Line::from(Span::styled(
                        "Nên làm:",
                        Style::default().fg(Color::Green),
                    )));
                    for act in activities.vi.iter().take(limit) {
                        lines.push(Line::from(format!("• {}", act)));
                    }
                    lines.push(Line::from(""));
                }

                if !compact && !festival.food.is_empty() {
                    lines.push(Line::from(Span::styled(
                        "Ẩm thực / Lễ vật:",
                        Style::default().fg(Color::Yellow),
                    )));
                    for food in festival.food.iter().take(3) {
                        lines.push(Line::from(format!(
                            "• {}: {}",
                            food.name.vi, food.description.vi
                        )));
                    }
                    lines.push(Line::from(""));
                }

                if !festival.taboos.is_empty() {
                    let limit = if compact { 2 } else { festival.taboos.len() };
                    lines.push(Line::from(Span::styled(
                        "Kiêng kỵ:",
                        Style::default().fg(Color::LightRed),
                    )));
                    for taboo in festival.taboos.iter().take(limit) {
                        lines.push(Line::from(format!(
                            "• {}: {}",
                            taboo.action.vi, taboo.reason.vi
                        )));
                    }
                    lines.push(Line::from(""));
                }

                if !compact && !festival.proverbs.is_empty() {
                    lines.push(Line::from(Span::styled(
                        "Câu truyền tụng:",
                        Style::default().fg(Color::Cyan),
                    )));
                    for proverb in festival.proverbs.iter().take(2) {
                        lines.push(Line::from(format!("• {}", proverb.text)));
                        lines.push(Line::from(format!("  {}", proverb.meaning.vi)));
                    }
                    lines.push(Line::from(""));
                }

                if !compact {
                if let Some(regions) = &festival.regions {
                    lines.push(Line::from(Span::styled(
                        "Sắc thái vùng miền:",
                        Style::default().fg(Color::Magenta),
                    )));
                    lines.push(Line::from(format!("• Bắc: {}", regions.north.vi)));
                    lines.push(Line::from(format!("• Trung: {}", regions.central.vi)));
                    lines.push(Line::from(format!("• Nam: {}", regions.south.vi)));
                }
                }
            } else if let Some(holiday) = &insight.holiday {
                has_event = true;
                lines.push(Line::from(vec![Span::styled(
                    holiday.names.vi.join(" / "),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )]));
                lines.push(Line::from(vec![
                    Span::styled("Phân loại: ", Style::default().fg(Color::Yellow)),
                    Span::raw(format!(
                        "{}{}",
                        holiday.category,
                        if holiday.is_major {
                            " · ngày lớn"
                        } else {
                            ""
                        }
                    )),
                ]));
                lines.push(Line::from(""));

                if let Some(origin) = &holiday.origin {
                    if compact {
                        lines.push(Line::from(Span::styled(
                            "Ý nghĩa:",
                            Style::default().fg(Color::Cyan),
                        )));
                        lines.push(Line::from(first_sentence(&origin.vi)));
                        lines.push(Line::from(""));
                    } else {
                    lines.push(Line::from(Span::styled(
                        "Nguồn gốc & Ý nghĩa:",
                        Style::default().fg(Color::Cyan),
                    )));
                    lines.push(Line::from(origin.vi.clone()));
                    lines.push(Line::from(""));
                    }
                }

                if !compact {
                if let Some(significance) = &holiday.significance {
                    lines.push(Line::from(Span::styled(
                        "Ý nghĩa xã hội:",
                        Style::default().fg(Color::Cyan),
                    )));
                    lines.push(Line::from(significance.vi.clone()));
                    lines.push(Line::from(""));
                }
                }

                if let Some(activities) = &holiday.activities {
                    let limit = if compact { 2 } else { activities.vi.len() };
                    lines.push(Line::from(Span::styled(
                        "Nên làm:",
                        Style::default().fg(Color::Green),
                    )));
                    for act in activities.vi.iter().take(limit) {
                        lines.push(Line::from(format!("• {}", act)));
                    }
                    lines.push(Line::from(""));
                }

                if !compact {
                if let Some(traditions) = &holiday.traditions {
                    lines.push(Line::from(Span::styled(
                        "Tập tục / Truyền thống:",
                        Style::default().fg(Color::Yellow),
                    )));
                    for item in traditions.vi.iter().take(4) {
                        lines.push(Line::from(format!("• {}", item)));
                    }
                    lines.push(Line::from(""));
                }
                }

                if !compact && !holiday.food.is_empty() {
                    lines.push(Line::from(Span::styled(
                        "Ẩm thực / Biểu trưng:",
                        Style::default().fg(Color::Yellow),
                    )));
                    for food in holiday.food.iter().take(3) {
                        lines.push(Line::from(format!(
                            "• {}: {}",
                            food.name.vi, food.description.vi
                        )));
                    }
                    lines.push(Line::from(""));
                }

                if !holiday.taboos.is_empty() {
                    let limit = if compact { 2 } else { holiday.taboos.len() };
                    lines.push(Line::from(Span::styled(
                        "Điều nên kiêng:",
                        Style::default().fg(Color::LightRed),
                    )));
                    for taboo in holiday.taboos.iter().take(limit) {
                        lines.push(Line::from(format!(
                            "• {}: {}",
                            taboo.action.vi, taboo.reason.vi
                        )));
                    }
                    lines.push(Line::from(""));
                }

                if !compact && !holiday.proverbs.is_empty() {
                    lines.push(Line::from(Span::styled(
                        "Câu truyền tụng:",
                        Style::default().fg(Color::Cyan),
                    )));
                    for proverb in holiday.proverbs.iter().take(2) {
                        lines.push(Line::from(format!("• {}", proverb.text)));
                        lines.push(Line::from(format!("  {}", proverb.meaning.vi)));
                    }
                    lines.push(Line::from(""));
                }

                if !compact {
                if let Some(regions) = &holiday.regions {
                    lines.push(Line::from(Span::styled(
                        "Khác biệt vùng miền:",
                        Style::default().fg(Color::Magenta),
                    )));
                    lines.push(Line::from(format!("• Bắc: {}", regions.north.vi)));
                    lines.push(Line::from(format!("• Trung: {}", regions.central.vi)));
                    lines.push(Line::from(format!("• Nam: {}", regions.south.vi)));
                }
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

fn first_sentence(text: &str) -> String {
    text.split_terminator(['.', '!', '?', '\n'])
        .next()
        .unwrap_or(text)
        .trim()
        .to_string()
}
