use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct GeneralScreenWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> GeneralScreenWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for GeneralScreenWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Some(bundle) = self.app.bundle.as_ref() else {
            return;
        };

        let block = Block::default()
            .title(" Màn hình General ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let mut lines = vec![
            Line::from(vec![
                Span::styled("Dương lịch: ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    format!(
                        "{} · {:02}/{:02}/{}",
                        bundle.solar.day_of_week_name,
                        bundle.solar.day,
                        bundle.solar.month,
                        bundle.solar.year
                    ),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("Âm lịch: ", Style::default().fg(Color::Cyan)),
                Span::raw(bundle.lunar.date_string.clone()),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Can Chi: ", Style::default().fg(Color::Yellow)),
                Span::raw(
                    bundle
                        .canchi
                        .as_ref()
                        .map(|c| c.full.clone())
                        .unwrap_or_else(|| "chưa có dữ liệu".to_string()),
                ),
            ]),
            Line::from(vec![
                Span::styled("Ngũ hành/Nạp âm: ", Style::default().fg(Color::Yellow)),
                Span::raw(
                    bundle
                        .day_fortune
                        .as_ref()
                        .map(|fortune| {
                            format!(
                                "{} · {}",
                                fortune.day_element.element, fortune.day_element.na_am
                            )
                        })
                        .unwrap_or_else(|| "chưa có dữ liệu".to_string()),
                ),
            ]),
            Line::from(vec![
                Span::styled("Tiết khí: ", Style::default().fg(Color::Yellow)),
                Span::raw(
                    bundle
                        .tiet_khi
                        .as_ref()
                        .map(|term| term.name.clone())
                        .unwrap_or_else(|| "chưa có dữ liệu".to_string()),
                ),
            ]),
            Line::from(vec![
                Span::styled("Trực: ", Style::default().fg(Color::Yellow)),
                Span::raw(
                    bundle
                        .day_fortune
                        .as_ref()
                        .map(|fortune| fortune.truc.name.clone())
                        .unwrap_or_else(|| "chưa có dữ liệu".to_string()),
                ),
            ]),
            Line::from(vec![
                Span::styled("Sao cát: ", Style::default().fg(Color::Green)),
                Span::raw(
                    bundle
                        .day_fortune
                        .as_ref()
                        .map(|fortune| {
                            if fortune.stars.cat_tinh.is_empty() {
                                "không".to_string()
                            } else {
                                fortune.stars.cat_tinh.join(", ")
                            }
                        })
                        .unwrap_or_else(|| "chưa có dữ liệu".to_string()),
                ),
            ]),
            Line::from(vec![
                Span::styled("Sao sát: ", Style::default().fg(Color::Red)),
                Span::raw(
                    bundle
                        .day_fortune
                        .as_ref()
                        .map(|fortune| {
                            if fortune.stars.sat_tinh.is_empty() {
                                "không".to_string()
                            } else {
                                fortune.stars.sat_tinh.join(", ")
                            }
                        })
                        .unwrap_or_else(|| "chưa có dữ liệu".to_string()),
                ),
            ]),
            Line::from(vec![
                Span::styled("Lễ/Hội: ", Style::default().fg(Color::Magenta)),
                Span::raw(
                    holiday_or_festival_label(bundle)
                        .unwrap_or_else(|| "không có dữ liệu lễ/hội".to_string()),
                ),
            ]),
            Line::from(""),
        ];

        let verdict = self.app.hero_verdict();
        lines.push(Line::from(vec![
            Span::styled("Kết luận nhanh: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                verdict
                    .as_ref()
                    .map(|v| v.summary.clone())
                    .unwrap_or_else(|| "Chưa có tóm tắt khuyến nghị".to_string()),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled("Nên ưu tiên: ", Style::default().fg(Color::Green)),
            Span::raw(
                verdict
                    .as_ref()
                    .and_then(|v| v.strongest_positive.clone())
                    .unwrap_or_else(|| "chưa có".to_string()),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled("Cần tránh: ", Style::default().fg(Color::Red)),
            Span::raw(
                verdict
                    .as_ref()
                    .and_then(|v| v.strongest_negative.clone())
                    .unwrap_or_else(|| "chưa có".to_string()),
            ),
        ]));

        Paragraph::new(lines).render(inner, buf);
    }
}

fn holiday_or_festival_label(bundle: &amlich_api::v2::DayBundleDto) -> Option<String> {
    let insight = bundle.insight.as_ref()?;
    if let Some(holiday) = insight.holiday.as_ref() {
        return holiday.names.vi.first().cloned();
    }
    insight
        .festival
        .as_ref()
        .and_then(|festival| festival.names.vi.first().cloned())
}
