use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget, Wrap},
};

use crate::app::App;
use crate::theme;

pub struct InfoPanel<'a> {
    app: &'a App,
}

impl<'a> InfoPanel<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }

    /// Build section divider: "── Section Name ──────..."
    fn section_line(name: &str, width: u16) -> Line<'static> {
        let prefix = format!("── {} ", name);
        let remaining = (width as usize).saturating_sub(prefix.chars().count());
        let line_str = format!("{}{}", prefix, "─".repeat(remaining));
        Line::from(Span::styled(line_str, theme::section_style()))
    }
}

impl Widget for InfoPanel<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut lines: Vec<Line<'_>> = Vec::new();
        let width = area.width;

        let Some(info) = self.app.selected_info() else {
            let p = Paragraph::new("Không có dữ liệu");
            p.render(area, buf);
            return;
        };

        // ── Tier 1: Date headline ──
        lines.push(Line::from(vec![
            Span::styled(
                info.solar.date_string.clone(),
                Style::default()
                    .fg(theme::PRIMARY_FG)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("  {}", info.solar.day_of_week_name),
                Style::default().fg(theme::SECONDARY_FG),
            ),
        ]));

        // Lunar date (with moon emoji)
        lines.push(Line::from(vec![
            Span::raw("🌙 "),
            Span::styled(
                info.lunar.date_string.clone(),
                Style::default()
                    .fg(theme::ACCENT_FG)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));

        lines.push(Line::from(""));

        // ── Tier 2: Can Chi ──
        lines.push(Self::section_line("Can Chi", width));

        lines.push(Line::from(vec![
            Span::styled(
                info.canchi.day.full.clone(),
                Style::default().fg(theme::PRIMARY_FG),
            ),
            Span::styled(
                format!(
                    "  {} · {}",
                    info.canchi.day.ngu_hanh.can, info.canchi.day.ngu_hanh.chi
                ),
                Style::default().fg(theme::SECONDARY_FG),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled(
                info.canchi.month.full.clone(),
                Style::default().fg(theme::PRIMARY_FG),
            ),
            Span::styled(
                format!(
                    "  {} · {}",
                    info.canchi.month.ngu_hanh.can, info.canchi.month.ngu_hanh.chi
                ),
                Style::default().fg(theme::SECONDARY_FG),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled(
                format!("{} ({})", info.canchi.year.full, info.canchi.year.con_giap),
                Style::default().fg(theme::PRIMARY_FG),
            ),
            Span::styled(
                format!(
                    "  {} · {}",
                    info.canchi.year.ngu_hanh.can, info.canchi.year.ngu_hanh.chi
                ),
                Style::default().fg(theme::SECONDARY_FG),
            ),
        ]));

        lines.push(Line::from(""));

        // ── Tier 2: Nên / Tránh (Day Guidance summary) ──
        lines.push(Self::section_line("Nên / Tránh", width));

        if let Some(insight) = self.app.selected_insight() {
            if let Some(guidance) = &insight.day_guidance {
                for item in guidance.good_for.vi.iter().take(4) {
                    lines.push(Line::from(vec![
                        Span::styled("✓ ", Style::default().fg(theme::GOOD_FG)),
                        Span::styled(item.clone(), Style::default().fg(theme::PRIMARY_FG)),
                    ]));
                }
                for item in guidance.avoid_for.vi.iter().take(4) {
                    lines.push(Line::from(vec![
                        Span::styled("✗ ", Style::default().fg(theme::BAD_FG)),
                        Span::styled(item.clone(), Style::default().fg(theme::PRIMARY_FG)),
                    ]));
                }
                let total = guidance.good_for.vi.len() + guidance.avoid_for.vi.len();
                if total > 8 {
                    lines.push(Line::from(Span::styled(
                        "                           [i] xem thêm",
                        Style::default().fg(theme::SECONDARY_FG),
                    )));
                }
            } else {
                lines.push(Line::from(Span::styled(
                    "Không có thông tin",
                    Style::default().fg(theme::SECONDARY_FG),
                )));
            }
        } else {
            lines.push(Line::from(Span::styled(
                "Không có thông tin",
                Style::default().fg(theme::SECONDARY_FG),
            )));
        }

        lines.push(Line::from(""));

        // ── Tier 2: Giờ tốt (compact one-liner) ──
        let good_names: Vec<&str> = info
            .gio_hoang_dao
            .good_hours
            .iter()
            .map(|h| h.hour_chi.as_str())
            .collect();
        let hours_str = good_names.join(" · ");

        lines.push(Line::from(vec![
            Span::styled("── Giờ tốt ", theme::section_style()),
            Span::styled(hours_str, Style::default().fg(theme::GOOD_HOUR_FG)),
        ]));

        lines.push(Line::from(""));

        // ── Tier 2: Tiết khí ──
        lines.push(Line::from(vec![
            Span::styled("── Tiết khí ", theme::section_style()),
            Span::styled(
                format!("{} · {}", info.tiet_khi.name, info.tiet_khi.season),
                Style::default().fg(theme::PRIMARY_FG),
            ),
        ]));

        if !info.tiet_khi.description.is_empty() {
            lines.push(Line::from(Span::styled(
                format!("   {}", info.tiet_khi.description),
                Style::default().fg(theme::SECONDARY_FG),
            )));
        }

        // ── Holiday (if applicable) ──
        if let Some(holiday) = self.app.holiday_for_day(self.app.selected_day) {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled("── Ngày lễ ", theme::section_style()),
                Span::styled(
                    holiday.name.clone(),
                    Style::default()
                        .fg(theme::HOLIDAY_FG)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
            if !holiday.description.is_empty() {
                lines.push(Line::from(Span::styled(
                    format!("   {}", holiday.description),
                    Style::default().fg(theme::SECONDARY_FG),
                )));
            }
        }

        let p = Paragraph::new(lines).wrap(Wrap { trim: false });
        p.render(area, buf);
    }
}
