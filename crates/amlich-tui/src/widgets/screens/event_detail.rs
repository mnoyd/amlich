use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct EventDetailScreenWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> EventDetailScreenWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for EventDetailScreenWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Some(bundle) = &self.app.bundle else {
            Paragraph::new("Chưa có dữ liệu sự kiện.").render(area, buf);
            return;
        };
        let event = EventContent::from_app(self.app);

        match self.mode {
            LayoutMode::Small => {
                let rows = Layout::vertical([
                    Constraint::Length(7),
                    Constraint::Length(9),
                    Constraint::Length(10),
                    Constraint::Length(10),
                    Constraint::Min(10),
                ])
                .split(area);
                render_overview(bundle, event.as_ref(), rows[0], buf);
                render_origin(event.as_ref(), rows[1], buf);
                render_regions(event.as_ref(), rows[2], buf);
                render_activities(event.as_ref(), rows[3], buf);
                render_taboos_and_figures(event.as_ref(), rows[4], buf);
            }
            LayoutMode::Medium | LayoutMode::Large => {
                let rows = Layout::vertical([
                    Constraint::Length(7),
                    Constraint::Length(10),
                    Constraint::Min(17),
                ])
                .split(area);
                render_overview(bundle, event.as_ref(), rows[0], buf);

                let context =
                    Layout::horizontal([Constraint::Percentage(65), Constraint::Percentage(35)])
                        .split(rows[1]);
                render_origin(event.as_ref(), context[0], buf);
                render_figures(event.as_ref(), context[1], buf);

                let details = Layout::horizontal([
                    Constraint::Percentage(34),
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                ])
                .split(rows[2]);
                render_regions(event.as_ref(), details[0], buf);
                render_activities(event.as_ref(), details[1], buf);
                render_taboos(event.as_ref(), details[2], buf);
            }
        }
    }
}

struct EventContent<'a> {
    names: &'a [String],
    origin: Option<&'a str>,
    activities: &'a [String],
    traditions: &'a [String],
    taboos: Vec<(&'a str, &'a str)>,
    regions: Option<(&'a str, &'a str, &'a str)>,
    figures: &'a [amlich_api::FigureInsightDto],
}

impl<'a> EventContent<'a> {
    fn from_app(app: &'a AppState) -> Option<Self> {
        let insight = app.bundle.as_ref()?.insight.as_ref()?;
        if let Some(festival) = &insight.festival {
            return Some(Self {
                names: &festival.names.vi,
                origin: festival.origin.as_ref().map(|origin| origin.vi.as_str()),
                activities: festival
                    .activities
                    .as_ref()
                    .map(|items| items.vi.as_slice())
                    .unwrap_or_default(),
                traditions: &[],
                taboos: festival
                    .taboos
                    .iter()
                    .map(|taboo| (taboo.action.vi.as_str(), taboo.reason.vi.as_str()))
                    .collect(),
                regions: festival.regions.as_ref().map(|regions| {
                    (
                        regions.north.vi.as_str(),
                        regions.central.vi.as_str(),
                        regions.south.vi.as_str(),
                    )
                }),
                figures: &festival.figures,
            });
        }

        insight.holiday.as_ref().map(|holiday| Self {
            names: &holiday.names.vi,
            origin: holiday
                .origin
                .as_ref()
                .map(|origin| origin.vi.as_str())
                .or_else(|| {
                    holiday
                        .significance
                        .as_ref()
                        .map(|meaning| meaning.vi.as_str())
                }),
            activities: holiday
                .activities
                .as_ref()
                .map(|items| items.vi.as_slice())
                .unwrap_or_default(),
            traditions: holiday
                .traditions
                .as_ref()
                .map(|items| items.vi.as_slice())
                .unwrap_or_default(),
            taboos: holiday
                .taboos
                .iter()
                .map(|taboo| (taboo.action.vi.as_str(), taboo.reason.vi.as_str()))
                .collect(),
            regions: holiday.regions.as_ref().map(|regions| {
                (
                    regions.north.vi.as_str(),
                    regions.central.vi.as_str(),
                    regions.south.vi.as_str(),
                )
            }),
            figures: &holiday.figures,
        })
    }
}

fn render_overview(
    bundle: &amlich_api::v2::DayBundleDto,
    event: Option<&EventContent<'_>>,
    area: Rect,
    buf: &mut Buffer,
) {
    let block = section_block(" Sự Kiện Chi Tiết ", Color::Yellow);
    let inner = block.inner(area);
    block.render(area, buf);

    let name = event
        .and_then(|event| event.names.first())
        .map(String::as_str)
        .unwrap_or("Không có lễ hội hoặc ngày lễ được định danh");
    let (phase, phase_detail) = lunar_phase(bundle.lunar.day);
    Paragraph::new(vec![
        Line::from(Span::styled(
            format!("  {name}"),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(format!("  Âm lịch: {}", bundle.lunar.date_string)),
        Line::from(vec![
            Span::raw("  Pha trăng: "),
            Span::styled(phase, Style::default().fg(Color::Cyan)),
            Span::raw(format!(" · {phase_detail}")),
        ]),
    ])
    .wrap(Wrap { trim: true })
    .render(inner, buf);
}

fn lunar_phase(day: i32) -> (&'static str, &'static str) {
    match day {
        1 => ("Trăng mới / Sóc", "Mùng Một mở đầu chu kỳ trăng"),
        2..=7 => ("Trăng non", "Ánh trăng tăng dần đầu tháng"),
        8..=14 => ("Thượng huyền", "Tiến tới đêm vọng"),
        15 => ("Trăng tròn / Vọng", "Ngày Rằm, điểm viên mãn của chu kỳ"),
        16..=22 => ("Hạ huyền", "Ánh trăng giảm dần sau ngày Rằm"),
        _ => ("Trăng tàn", "Cuối chu kỳ, chuẩn bị sang tháng mới"),
    }
}

fn render_origin(event: Option<&EventContent<'_>>, area: Rect, buf: &mut Buffer) {
    render_text_section(
        " Nguồn Gốc / Bối Cảnh ",
        event
            .and_then(|event| event.origin)
            .unwrap_or("Chưa có bối cảnh lịch sử chi tiết cho sự kiện này."),
        area,
        buf,
    );
}

fn render_regions(event: Option<&EventContent<'_>>, area: Rect, buf: &mut Buffer) {
    let block = section_block(" Biến Thể Vùng Miền ", Color::Cyan);
    let inner = block.inner(area);
    block.render(area, buf);
    let lines = event
        .and_then(|event| event.regions)
        .map(|(north, central, south)| {
            vec![
                Line::from(format!("  Miền Bắc: {north}")),
                Line::from(format!("  Miền Trung: {central}")),
                Line::from(format!("  Miền Nam: {south}")),
            ]
        })
        .unwrap_or_else(|| vec![Line::from("  Chưa có ghi chú khác biệt vùng miền.")]);
    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_activities(event: Option<&EventContent<'_>>, area: Rect, buf: &mut Buffer) {
    let block = section_block(" Nên Làm / Truyền Thống ", Color::Green);
    let inner = block.inner(area);
    block.render(area, buf);
    let mut lines = Vec::new();
    if let Some(event) = event {
        for item in event.activities.iter().chain(event.traditions.iter()) {
            lines.push(Line::from(format!("  • {item}")));
        }
    }
    if lines.is_empty() {
        lines.push(Line::from("  Chưa có chỉ dẫn riêng cho sự kiện."));
    }
    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_taboos(event: Option<&EventContent<'_>>, area: Rect, buf: &mut Buffer) {
    let block = section_block(" Kiêng Kỵ / Lưu Ý ", Color::Red);
    let inner = block.inner(area);
    block.render(area, buf);
    let mut lines = Vec::new();
    if let Some(event) = event {
        for (action, reason) in &event.taboos {
            lines.push(Line::from(format!("  • {action}")));
            lines.push(Line::from(Span::styled(
                format!("    {reason}"),
                Style::default().fg(Color::DarkGray),
            )));
        }
    }
    if lines.is_empty() {
        lines.push(Line::from("  Chưa ghi nhận kiêng kỵ riêng cho sự kiện."));
    }
    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_figures(event: Option<&EventContent<'_>>, area: Rect, buf: &mut Buffer) {
    let block = section_block(" Nhân Vật / Thần Linh Liên Hệ ", Color::Magenta);
    let inner = block.inner(area);
    block.render(area, buf);
    let lines = event
        .filter(|event| !event.figures.is_empty())
        .map(|event| {
            let mut lines = Vec::new();
            for figure in event.figures {
                lines.push(Line::from(vec![
                    Span::styled(
                        format!("  {} ", figure.name.vi),
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("({})", figure.role.vi),
                        Style::default().fg(Color::DarkGray),
                    ),
                ]));
                lines.push(Line::from(format!("    {}", figure.description.vi)));
            }
            lines
        })
        .unwrap_or_else(|| {
            vec![Line::from(
                "  Chưa gắn nhân vật hoặc thần linh riêng cho sự kiện này.",
            )]
        });
    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_taboos_and_figures(event: Option<&EventContent<'_>>, area: Rect, buf: &mut Buffer) {
    let columns =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);
    render_taboos(event, columns[0], buf);
    render_figures(event, columns[1], buf);
}

fn render_text_section(title: &str, text: &str, area: Rect, buf: &mut Buffer) {
    let block = section_block(title, Color::DarkGray);
    let inner = block.inner(area);
    block.render(area, buf);
    Paragraph::new(format!("  {text}"))
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn section_block(title: &str, color: Color) -> Block<'_> {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(color))
}
