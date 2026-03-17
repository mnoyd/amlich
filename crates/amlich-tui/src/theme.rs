use ratatui::style::{Color, Style};

pub struct Theme;

impl Theme {
    pub const RED: Color = Color::Red;
    pub const GOLD: Color = Color::Yellow;
    pub const BLACK: Color = Color::Black;
    pub const CHARCOAL: Color = Color::DarkGray;
    pub const WHITE: Color = Color::White;
    pub const CYAN: Color = Color::Cyan;
    pub const GREEN: Color = Color::Green;

    pub fn bg_style() -> Style {
        Style::default().bg(Self::BLACK).fg(Self::WHITE)
    }

    pub fn primary_border() -> Style {
        Style::default().fg(Self::RED)
    }

    pub fn secondary_border() -> Style {
        Style::default().fg(Self::GOLD)
    }

    pub fn title_style() -> Style {
        Style::default().fg(Self::GOLD)
    }

    pub fn highlight() -> Style {
        Style::default().fg(Self::BLACK).bg(Self::GOLD)
    }
}
