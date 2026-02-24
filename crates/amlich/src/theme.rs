use ratatui::style::{Color, Modifier, Style};

// Terminal-native: use Color::Reset for default fg/bg (respects user theme)

// Text hierarchy
pub const PRIMARY_FG: Color = Color::Reset; // default terminal fg
pub const SECONDARY_FG: Color = Color::DarkGray; // dimmed text, labels
pub const ACCENT_FG: Color = Color::Rgb(212, 168, 85); // warm amber — section headers, highlights

// Calendar
pub const SOLAR_FG: Color = Color::Reset;
pub const LUNAR_FG: Color = Color::DarkGray;
pub const TODAY_FG: Color = Color::Black; // inverted
pub const TODAY_BG: Color = Color::White; // inverted
pub const SELECTED_FG: Color = Color::Reset; // bold + underline, no bg change
pub const WEEKEND_FG: Color = Color::Rgb(224, 112, 112); // soft coral
pub const HOLIDAY_FG: Color = Color::Rgb(212, 168, 85); // amber (same as accent)

// Day guidance
pub const GOOD_FG: Color = Color::Rgb(109, 191, 139); // soft green
pub const BAD_FG: Color = Color::Rgb(224, 112, 112); // soft coral

// Hours
pub const GOOD_HOUR_FG: Color = Color::Rgb(109, 191, 139);
pub const BAD_HOUR_FG: Color = Color::DarkGray;

// Borders
pub const BORDER_COLOR: Color = Color::DarkGray;

// Section header style: amber, bold
pub fn section_style() -> Style {
    Style::default().fg(ACCENT_FG).add_modifier(Modifier::BOLD)
}

pub fn border_style() -> Style {
    Style::default().fg(BORDER_COLOR)
}

// Temporary aliases — remove after all widgets are migrated
pub const HEADER_BG: Color = Color::Reset;
pub const HEADER_FG: Color = Color::Reset;
pub const VALUE_FG: Color = PRIMARY_FG;
pub const LABEL_FG: Color = SECONDARY_FG;
pub const TITLE_FG: Color = ACCENT_FG;
pub const SELECTED_BG: Color = Color::Reset;

pub fn title_style() -> Style {
    section_style()
}
