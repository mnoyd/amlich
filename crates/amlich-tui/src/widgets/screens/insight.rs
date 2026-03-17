use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};

use crate::widgets::{
    direction_panel::DirectionPanelWidget, guidance_panel::GuidancePanelWidget,
    naam_panel::NaAmPanelWidget, risk::RiskWidget, scholarly::ScholarlyWidget,
    stars_panel::StarsPanelWidget,
};
use crate::{layout::LayoutMode, state::AppState};

pub struct InsightScreenWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> InsightScreenWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for InsightScreenWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.mode {
            LayoutMode::Large => {
                let rows = Layout::vertical([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])
                .split(area);
                let top = Layout::horizontal([
                    Constraint::Percentage(34),
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                ])
                .split(rows[0]);
                let bottom = Layout::horizontal([
                    Constraint::Percentage(34),
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                ])
                .split(rows[1]);

                ScholarlyWidget::new(self.app, self.mode).render(top[0], buf);
                StarsPanelWidget::new(self.app, self.mode).render(top[1], buf);
                RiskWidget::new(self.app, self.mode).render(top[2], buf);
                NaAmPanelWidget::new(self.app, self.mode).render(bottom[0], buf);
                DirectionPanelWidget::new(self.app, self.mode).render(bottom[1], buf);
                GuidancePanelWidget::new(self.app, self.mode).render(bottom[2], buf);
            }
            LayoutMode::Medium => {
                let rows = Layout::vertical([
                    Constraint::Percentage(34),
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                ])
                .split(area);
                let r0 = Layout::horizontal([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])
                .split(rows[0]);
                let r1 = Layout::horizontal([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])
                .split(rows[1]);
                let r2 = Layout::horizontal([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])
                .split(rows[2]);

                ScholarlyWidget::new(self.app, self.mode).render(r0[0], buf);
                StarsPanelWidget::new(self.app, self.mode).render(r0[1], buf);
                RiskWidget::new(self.app, self.mode).render(r1[0], buf);
                NaAmPanelWidget::new(self.app, self.mode).render(r1[1], buf);
                DirectionPanelWidget::new(self.app, self.mode).render(r2[0], buf);
                GuidancePanelWidget::new(self.app, self.mode).render(r2[1], buf);
            }
            LayoutMode::Small => {
                let rows = Layout::vertical([
                    Constraint::Min(12),
                    Constraint::Min(10),
                    Constraint::Min(8),
                    Constraint::Min(10),
                    Constraint::Min(8),
                    Constraint::Min(10),
                ])
                .split(area);

                ScholarlyWidget::new(self.app, self.mode).render(rows[0], buf);
                StarsPanelWidget::new(self.app, self.mode).render(rows[1], buf);
                RiskWidget::new(self.app, self.mode).render(rows[2], buf);
                NaAmPanelWidget::new(self.app, self.mode).render(rows[3], buf);
                DirectionPanelWidget::new(self.app, self.mode).render(rows[4], buf);
                GuidancePanelWidget::new(self.app, self.mode).render(rows[5], buf);
            }
        }
    }
}
