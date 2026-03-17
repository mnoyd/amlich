use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Row, Table, Widget},
};

use crate::layout::LayoutMode;
use crate::state::AppState;

pub struct AlmanacGridWidget<'a> {
    app: &'a AppState,
}

impl<'a> AlmanacGridWidget<'a> {
    pub fn new(app: &'a AppState, _mode: LayoutMode) -> Self {
        Self { app }
    }
}

impl Widget for AlmanacGridWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Some(bundle) = &self.app.bundle else {
            return;
        };

        let block = Block::default()
            .title(" Can Chi & Nạp Âm ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let Some(canchi) = &bundle.canchi else {
            return;
        };

        let header = Row::new(vec!["", "Năm", "Tháng", "Ngày"])
            .style(Style::default().fg(Color::Gray))
            .bottom_margin(1);

        let row_canchi = Row::new(vec![
            Cell::from("Can Chi").style(Style::default().fg(Color::Gray)),
            Cell::from(canchi.year.full.clone()).style(Style::default().fg(Color::Yellow)),
            Cell::from(canchi.month.full.clone()).style(Style::default().fg(Color::Yellow)),
            Cell::from(canchi.day.full.clone()).style(Style::default().fg(Color::Yellow)),
        ]);

        let row_nguhanh = Row::new(vec![
            Cell::from("Ngũ Hành").style(Style::default().fg(Color::Gray)),
            Cell::from(format!(
                "{} - {}",
                canchi.year.ngu_hanh.can, canchi.year.ngu_hanh.chi
            )),
            Cell::from(format!(
                "{} - {}",
                canchi.month.ngu_hanh.can, canchi.month.ngu_hanh.chi
            )),
            Cell::from(format!(
                "{} - {}",
                canchi.day.ngu_hanh.can, canchi.day.ngu_hanh.chi
            )),
        ]);

        let mut rows = vec![row_canchi, row_nguhanh];

        if let Some(fortune) = &bundle.day_fortune {
            let row_napam = Row::new(vec![
                Cell::from("Nạp Âm").style(Style::default().fg(Color::Gray)),
                Cell::from(""), // Year nap am not available
                Cell::from(""), // Month nap am not available
                Cell::from(fortune.day_element.na_am.clone())
                    .style(Style::default().fg(Color::Cyan)),
            ]);
            rows.push(row_napam);
        }

        let widths = [
            Constraint::Length(10),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ];

        let table = Table::new(rows, widths)
            .header(header)
            .block(block)
            .column_spacing(1);

        table.render(area, buf);
    }
}
