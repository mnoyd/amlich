use ratatui::{
    layout::{Constraint, Layout, Rect},
    Frame,
};

use crate::state::{AppState, FocusLens};
use crate::widgets::{
    page::PageWidget,
    ribbon::RibbonWidget,
    calendar::CalendarOverlayWidget,
    search::SearchOverlayWidget,
};

const MIN_TERM_W: u16 = 40;
const MIN_TERM_H: u16 = 15;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LayoutMode {
    Small,  // < 60 cols (Phone/Tiny pane)
    Medium, // 60-100 cols
    Large,  // > 100 cols (Desktop full)
}

pub fn layout_mode(width: u16) -> LayoutMode {
    if width < 60 {
        LayoutMode::Small
    } else if width < 100 {
        LayoutMode::Medium
    } else {
        LayoutMode::Large
    }
}

pub fn draw(frame: &mut Frame, app: &AppState) {
    let size = frame.area();

    // Enforce minimum terminal size
    if size.width < MIN_TERM_W || size.height < MIN_TERM_H {
        use ratatui::widgets::{Block, Borders, Paragraph};
        use ratatui::layout::Alignment;
        
        let msg = Paragraph::new("Terminal quá nhỏ.\nCần tối thiểu 40×15.")
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
            
        frame.render_widget(msg, size);
        return;
    }

    // Main vertical layout: Page area (scrollable) + Ribbon area (fixed bottom)
    let main_layout = Layout::vertical([
        Constraint::Min(10),    // Main scrolling page
        Constraint::Length(2),  // Fixed bottom ribbon (includes top padding line)
    ])
    .split(size);

    let page_area = main_layout[0];
    let ribbon_area = main_layout[1];

    // Determine the layout constraints based on mode
    let mode = layout_mode(size.width);
    
    // For large screens, we constrain the maximum width of the reading area
    // to make it more readable (like a web page container)
    let content_area = match mode {
        LayoutMode::Large => {
            // Center the content with a max width of 100
            let pad = (size.width - 100) / 2;
            Layout::horizontal([
                Constraint::Length(pad),
                Constraint::Length(100),
                Constraint::Length(pad),
            ])
            .split(page_area)[1]
        }
        _ => {
            // Small/Medium uses full width with slight padding
            let pad = if mode == LayoutMode::Small { 1 } else { 2 };
            Layout::horizontal([
                Constraint::Length(pad),
                Constraint::Min(10),
                Constraint::Length(pad),
            ])
            .split(page_area)[1]
        }
    };

    // Render the main page widget within the content area
    frame.render_widget(PageWidget::new(app, mode), content_area);
    
    // Render the fixed ribbon at the bottom
    frame.render_widget(RibbonWidget::new(app, mode), ribbon_area);
    
    // Render overlays (Calendar, Search, etc) on top if active
    if app.show_calendar {
        frame.render_widget(CalendarOverlayWidget::new(app, mode), size);
    }
    
    if app.show_search {
        frame.render_widget(SearchOverlayWidget::new(app, mode), size);
    }
}
