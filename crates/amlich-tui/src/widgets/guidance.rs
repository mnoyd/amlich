use std::cmp::max;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use amlich_api::{v2::DayBundleDto, LocalizedListDto};

use crate::layout::LayoutMode;
use crate::state::AppState;

const MEDIUM_COL_WIDTH: usize = 27;
const LARGE_COL_WIDTH: usize = 34;
const SMALL_LIMIT: usize = 3;
const MEDIUM_LIMIT: usize = 5;
const LARGE_LIMIT: usize = 6;

#[derive(Clone, Copy, PartialEq, Eq)]
enum DecisionSide {
    Nen,
    Tranh,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum DecisionEmphasis {
    Primary,
    Normal,
}

#[derive(Clone)]
struct DecisionRow {
    side: DecisionSide,
    text: String,
    priority: i32,
    emphasis: DecisionEmphasis,
    reason_chip: Option<String>,
}

pub struct GuidanceWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> GuidanceWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for GuidanceWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Some(bundle) = &self.app.bundle else {
            return;
        };
        let Some(insight) = &bundle.insight else {
            return;
        };
        let Some(guidance) = &insight.day_guidance else {
            return;
        };

        let mut nen_rows = build_decision_rows(
            bundle,
            localized_items(&guidance.good_for),
            DecisionSide::Nen,
        );
        let mut tranh_rows = build_decision_rows(
            bundle,
            localized_items(&guidance.avoid_for),
            DecisionSide::Tranh,
        );

        if nen_rows.is_empty() && tranh_rows.is_empty() {
            return;
        }

        mark_primary(&mut nen_rows);
        mark_primary(&mut tranh_rows);

        let expanded = self.app.show_guidance_details;
        let limit = display_limit(self.mode, expanded);
        let summary = build_guidance_summary(bundle);

        let mut lines = vec![];
        let header_style = Style::default().fg(Color::DarkGray);
        let summary_style = Style::default().fg(Color::Yellow);
        let hint_style = Style::default().fg(Color::DarkGray);

        let expand_hint = if expanded {
            "▼ Thu gọn (a)"
        } else {
            "▶ Mở rộng (a)"
        };

        lines.push(Line::from(vec![
            Span::styled("── Hành Sự ", header_style),
            Span::styled(format!("{:─<24}", ""), header_style),
            Span::styled(expand_hint, hint_style),
        ]));

        if let Some(summary) = summary {
            lines.push(Line::from(vec![
                Span::styled("   ", summary_style),
                Span::styled(summary, summary_style),
            ]));
            lines.push(Line::from(""));
        }

        match self.mode {
            LayoutMode::Small => render_stacked(&mut lines, &nen_rows, &tranh_rows, limit),
            LayoutMode::Medium => {
                if area.width < 68 {
                    render_stacked(&mut lines, &nen_rows, &tranh_rows, limit);
                } else {
                    render_side_by_side(
                        &mut lines,
                        &nen_rows,
                        &tranh_rows,
                        limit,
                        MEDIUM_COL_WIDTH,
                    );
                }
            }
            LayoutMode::Large => {
                render_side_by_side(&mut lines, &nen_rows, &tranh_rows, limit, LARGE_COL_WIDTH);
            }
        }

        if let Some(footer) = build_footer_hint(bundle) {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled("   ", hint_style),
                Span::styled(footer, hint_style.add_modifier(Modifier::ITALIC)),
            ]));
        }

        Paragraph::new(lines).render(area, buf);
    }
}

fn render_stacked(
    lines: &mut Vec<Line<'static>>,
    nen_rows: &[DecisionRow],
    tranh_rows: &[DecisionRow],
    limit: usize,
) {
    let header_style = Style::default().fg(Color::DarkGray);
    let nen_style = Style::default().fg(Color::Green);
    let tranh_style = Style::default().fg(Color::Red);
    let text_style = Style::default().fg(Color::White);
    let chip_style = Style::default().fg(Color::Cyan);

    if !nen_rows.is_empty() {
        lines.push(Line::from(vec![
            Span::styled("── Nên ", header_style),
            Span::styled(format!("{:─<24}", ""), header_style),
        ]));
        for row in nen_rows.iter().take(limit) {
            for rendered in render_row_lines(row, nen_style, text_style, chip_style, 48) {
                lines.push(rendered);
            }
        }
    }

    if !tranh_rows.is_empty() {
        if !nen_rows.is_empty() {
            lines.push(Line::from(""));
        }
        lines.push(Line::from(vec![
            Span::styled("── Tránh ", header_style),
            Span::styled(format!("{:─<22}", ""), header_style),
        ]));
        for row in tranh_rows.iter().take(limit) {
            for rendered in render_row_lines(row, tranh_style, text_style, chip_style, 48) {
                lines.push(rendered);
            }
        }
    }
}

fn render_side_by_side(
    lines: &mut Vec<Line<'static>>,
    nen_rows: &[DecisionRow],
    tranh_rows: &[DecisionRow],
    limit: usize,
    col_width: usize,
) {
    let header_style = Style::default().fg(Color::DarkGray);
    let text_style = Style::default().fg(Color::White);
    let nen_style = Style::default().fg(Color::Green);
    let tranh_style = Style::default().fg(Color::Red);
    let chip_style = Style::default().fg(Color::Cyan);

    let left_header = format!(
        "── Nên {:─<width$}",
        "",
        width = col_width.saturating_sub(7)
    );
    let right_header = format!(
        "── Tránh {:─<width$}",
        "",
        width = col_width.saturating_sub(9)
    );
    lines.push(Line::from(vec![
        Span::styled(left_header, header_style),
        Span::raw("   "),
        Span::styled(right_header, header_style),
    ]));

    let left_blocks = rows_to_blocks(
        nen_rows, limit, col_width, nen_style, text_style, chip_style,
    );
    let right_blocks = rows_to_blocks(
        tranh_rows,
        limit,
        col_width,
        tranh_style,
        text_style,
        chip_style,
    );
    let render_len = max(left_blocks.len(), right_blocks.len());

    for i in 0..render_len {
        let left = left_blocks
            .get(i)
            .cloned()
            .unwrap_or_else(|| pad_line(col_width));
        let right = right_blocks
            .get(i)
            .cloned()
            .unwrap_or_else(|| Line::from(""));

        let mut spans = left.spans;
        spans.push(Span::raw("   "));
        spans.extend(right.spans);
        lines.push(Line::from(spans));
    }
}

fn rows_to_blocks(
    rows: &[DecisionRow],
    limit: usize,
    col_width: usize,
    marker_style: Style,
    text_style: Style,
    chip_style: Style,
) -> Vec<Line<'static>> {
    let mut rendered = Vec::new();
    for row in rows.iter().take(limit) {
        rendered.extend(render_row_lines(
            row,
            marker_style,
            text_style,
            chip_style,
            col_width,
        ));
    }
    rendered
}

fn render_row_lines(
    row: &DecisionRow,
    marker_style: Style,
    text_style: Style,
    chip_style: Style,
    width: usize,
) -> Vec<Line<'static>> {
    let marker = match (row.side, row.emphasis) {
        (DecisionSide::Nen, DecisionEmphasis::Primary) => "★ ",
        (DecisionSide::Nen, DecisionEmphasis::Normal) => "✓ ",
        (DecisionSide::Tranh, DecisionEmphasis::Primary) => "★ ",
        (DecisionSide::Tranh, DecisionEmphasis::Normal) => "✗ ",
    };
    let base_indent = 3;
    let content_width = width.saturating_sub(base_indent + marker.len());
    let chip = row.reason_chip.as_ref().map(|c| format!(" [{}]", c));

    let mut first = row.text.clone();
    if let Some(chip) = chip.as_ref() {
        let needed = chip.chars().count();
        if first.chars().count() + needed <= content_width {
            first.push_str(chip);
        }
    }

    let wrapped = wrap_text(&first, content_width.max(8));
    let mut lines = Vec::new();

    for (i, segment) in wrapped.into_iter().enumerate() {
        let mut spans = vec![Span::raw("   ")];
        if i == 0 {
            spans.push(Span::styled(marker.to_string(), marker_style));
        } else {
            spans.push(Span::raw("  "));
        }

        if let Some(chip_text) = chip.as_ref() {
            if i == 0 && segment.ends_with(chip_text) {
                let item_len = segment.len() - chip_text.len();
                spans.push(Span::styled(segment[..item_len].to_string(), text_style));
                spans.push(Span::styled(chip_text.to_string(), chip_style));
            } else {
                spans.push(Span::styled(segment, text_style));
            }
        } else {
            spans.push(Span::styled(segment, text_style));
        }

        lines.push(Line::from(spans));
    }

    lines
}

fn wrap_text(text: &str, width: usize) -> Vec<String> {
    if text.chars().count() <= width {
        return vec![text.to_string()];
    }

    let mut lines = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        let candidate_len = if current.is_empty() {
            word.chars().count()
        } else {
            current.chars().count() + 1 + word.chars().count()
        };

        if candidate_len > width && !current.is_empty() {
            lines.push(current);
            current = word.to_string();
        } else {
            if !current.is_empty() {
                current.push(' ');
            }
            current.push_str(word);
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }

    lines
}

fn pad_line(width: usize) -> Line<'static> {
    Line::from(Span::raw(format!("{:<width$}", "", width = width + 3)))
}

fn build_decision_rows(
    bundle: &DayBundleDto,
    items: &[String],
    side: DecisionSide,
) -> Vec<DecisionRow> {
    let mut rows: Vec<DecisionRow> = items
        .iter()
        .enumerate()
        .map(|(idx, item)| {
            let (priority, reason_chip) = score_item(bundle, item, side);
            DecisionRow {
                side,
                text: item.clone(),
                priority: priority * 10 - idx as i32,
                emphasis: DecisionEmphasis::Normal,
                reason_chip,
            }
        })
        .collect();

    rows.sort_by(|a, b| b.priority.cmp(&a.priority));
    rows
}

fn mark_primary(rows: &mut [DecisionRow]) {
    if let Some(first) = rows.first_mut() {
        first.emphasis = DecisionEmphasis::Primary;
    }
}

fn score_item(bundle: &DayBundleDto, item: &str, side: DecisionSide) -> (i32, Option<String>) {
    let mut score = 0;
    let lower = item.to_lowercase();

    if let Some(fortune) = &bundle.day_fortune {
        let truc_quality = fortune.truc.quality.as_str();
        let cat_count = fortune.stars.cat_tinh.len() as i32;
        let sat_count = fortune.stars.sat_tinh.len() as i32;
        let taboo_count = fortune.taboos.len() as i32;

        match side {
            DecisionSide::Nen => {
                if truc_quality == "cat" {
                    score += 2;
                }
                if cat_count > sat_count {
                    score += 2;
                }
                if matches_any(&lower, &["xuất hành", "đi", "di chuyển"]) {
                    score += 2;
                    return (score, Some("giờ tốt".to_string()));
                }
                if matches_any(&lower, &["gặp", "họp", "giao tiếp"]) && cat_count > 0 {
                    score += 1;
                    return (score, Some("cát tinh".to_string()));
                }
                if matches_any(&lower, &["khai", "khởi", "mở"]) && truc_quality == "cat" {
                    score += 2;
                    return (score, Some("trực tốt".to_string()));
                }
            }
            DecisionSide::Tranh => {
                if truc_quality == "hung" {
                    score += 2;
                }
                if sat_count >= cat_count {
                    score += 2;
                }
                if taboo_count > 0 {
                    score += 1;
                }
                if matches_any(&lower, &["động thổ", "sửa", "xây", "đào"]) {
                    score += 2;
                    return (score, Some("trực xấu".to_string()));
                }
                if matches_any(&lower, &["tranh chấp", "kiện", "cãi"]) {
                    score += 2;
                    return (score, Some("xung ngày".to_string()));
                }
                if matches_any(&lower, &["quyết định", "đầu tư", "ký"]) && sat_count > 0 {
                    score += 2;
                    return (score, Some("sát tinh".to_string()));
                }
            }
        }

        if side == DecisionSide::Nen && cat_count > 0 {
            return (score + 1, Some("cát tinh".to_string()));
        }
        if side == DecisionSide::Tranh && taboo_count > 0 {
            return (score + 1, Some("kiêng kỵ".to_string()));
        }
    }

    if let Some(hours) = &bundle.gio_hoang_dao {
        if side == DecisionSide::Nen && hours.good_hour_count >= 6 {
            score += 1;
        }
    }

    (score, None)
}

fn matches_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| haystack.contains(needle))
}

fn build_guidance_summary(bundle: &DayBundleDto) -> Option<String> {
    let fortune = bundle.day_fortune.as_ref()?;
    let good_hours = bundle
        .gio_hoang_dao
        .as_ref()
        .map(|hours| hours.good_hour_count)
        .unwrap_or(0);
    let cat_count = fortune.stars.cat_tinh.len();
    let sat_count = fortune.stars.sat_tinh.len();
    let taboo_count = fortune.taboos.len();
    let truc = fortune.truc.name.as_str();

    let summary = if fortune.truc.quality == "cat" && cat_count >= sat_count {
        format!("Hợp việc triển khai gọn; trực {truc}, {good_hours} giờ tốt")
    } else if taboo_count > 0 || fortune.truc.quality == "hung" {
        format!("Nên giữ nhịp an toàn; trực {truc}, có dấu hiệu cần tránh việc lớn")
    } else {
        format!("Ngày trung hòa; trực {truc}, ưu tiên việc nhỏ và đều tay")
    };

    Some(summary)
}

fn build_footer_hint(bundle: &DayBundleDto) -> Option<String> {
    let hours = bundle.gio_hoang_dao.as_ref()?;
    let best: Vec<String> = hours
        .good_hours
        .iter()
        .take(3)
        .map(|hour| hour.time_range.clone())
        .collect();

    if best.is_empty() {
        None
    } else {
        Some(format!("Giờ tốt nhất: {}", best.join(", ")))
    }
}

fn display_limit(mode: LayoutMode, expanded: bool) -> usize {
    if expanded {
        return usize::MAX;
    }

    match mode {
        LayoutMode::Small => SMALL_LIMIT,
        LayoutMode::Medium => MEDIUM_LIMIT,
        LayoutMode::Large => LARGE_LIMIT,
    }
}

fn localized_items(list: &LocalizedListDto) -> &[String] {
    if !list.vi.is_empty() {
        &list.vi
    } else {
        &list.en
    }
}
