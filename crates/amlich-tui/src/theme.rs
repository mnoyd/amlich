use ratatui::style::{Color, Modifier, Style};

// Header / brand
pub const HEADER_BG: Color = Color::Rgb(139, 0, 0); // deep red
pub const HEADER_FG: Color = Color::White;

// Calendar grid
pub const SOLAR_FG: Color = Color::Rgb(245, 245, 245);
pub const LUNAR_FG: Color = Color::Rgb(135, 145, 165);
pub const TODAY_BG: Color = Color::Rgb(139, 0, 0);
pub const TODAY_FG: Color = Color::White;
pub const SELECTED_BG: Color = Color::Rgb(35, 105, 95); // jade
pub const SELECTED_FG: Color = Color::White;
pub const WEEKEND_FG: Color = Color::Rgb(255, 150, 115);
pub const HOLIDAY_FG: Color = Color::Rgb(255, 212, 95); // gold

// Detail panel
pub const LABEL_FG: Color = Color::Rgb(150, 150, 150);
pub const VALUE_FG: Color = Color::White;
pub const ACCENT_FG: Color = Color::Rgb(42, 180, 160); // jade accent

// Hours panel
pub const GOOD_HOUR_FG: Color = Color::Rgb(100, 255, 100);
pub const BAD_HOUR_FG: Color = Color::Rgb(100, 100, 100);

// Borders
pub const BORDER_COLOR: Color = Color::Rgb(60, 60, 60);
pub const TITLE_FG: Color = Color::Rgb(212, 175, 55); // gold

pub fn title_style() -> Style {
    Style::default().fg(TITLE_FG).add_modifier(Modifier::BOLD)
}

pub fn border_style() -> Style {
    Style::default().fg(BORDER_COLOR)
}
