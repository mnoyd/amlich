use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};

use crate::layout::LayoutMode;
use crate::state::{AppState, FocusLens};

use super::{
    hero::HeroWidget,
    battu::BattuWidget,
    guidance::GuidanceWidget,
    timeline::TimelineWidget,
    tietkhi::TietKhiWidget,
    scholarly::ScholarlyWidget,
};

pub struct PageWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> PageWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for PageWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use ratatui::widgets::Paragraph;
        
        if self.app.is_loading {
            Paragraph::new("Đang tải dữ liệu...").render(area, buf);
            return;
        }

        if let Some(err) = &self.app.error_msg {
            Paragraph::new(format!("Lỗi: {}", err)).render(area, buf);
            return;
        }

        if self.app.bundle.is_none() {
            Paragraph::new("Không có dữ liệu.").render(area, buf);
            return;
        }

        // We use a vertical layout to stack the widgets.
        // In a true scrolling virtual layout, we'd render to a large off-screen buffer
        // and blit a window of it. For simplicity in this alpha, we just constrain based
        // on the terminal height directly, using fixed block heights.
        // A full terminal scrollview implementation is complex, so we'll approximate it
        // by mapping sections to constraints.

        let chunks = Layout::vertical([
            Constraint::Length(7),  // Hero (Bordered, 5 lines + 2 borders)
            Constraint::Length(1),  // Padding
            Constraint::Length(7),  // Battu (Header + 3 lines + Na Am + Padding)
            Constraint::Length(10), // Guidance (Header + List)
            Constraint::Length(1),  // Padding
            Constraint::Length(6),  // Timeline (Header + 4 lines + Padding)
            Constraint::Min(0),     // Rest (Tiet Khi / Scholarly)
        ])
        .split(area);

        // 1. Hero Date
        HeroWidget::new(self.app, self.mode).render(chunks[0], buf);
        
        // 2. Can Chi (Bát Tự)
        BattuWidget::new(self.app, self.mode).render(chunks[2], buf);

        // 3/4. Depend on Lens
        match self.app.lens {
            FocusLens::General => {
                GuidanceWidget::new(self.app, self.mode).render(chunks[3], buf);
                TimelineWidget::new(self.app, self.mode).render(chunks[5], buf);
                TietKhiWidget::new(self.app, self.mode).render(chunks[6], buf);
            }
            FocusLens::Planning => {
                TimelineWidget::new(self.app, self.mode).render(chunks[3], buf);
                GuidanceWidget::new(self.app, self.mode).render(chunks[5], buf);
            }
            FocusLens::Scholarly => {
                ScholarlyWidget::new(self.app, self.mode).render(chunks[3], buf);
            }
            FocusLens::Personal => {
                Paragraph::new("Tính năng Cá Nhân đang được phát triển...").render(chunks[3], buf);
            }
        }
    }
}
