use ratatui::style::{Color, Style};

pub struct Theme;

impl Theme {
    pub const SURFACE_BG: Color = Color::Black;
    pub const TEXT_PRIMARY: Color = Color::White;
    pub const TEXT_MUTED: Color = Color::Gray;
    pub const TEXT_DIM: Color = Color::DarkGray;
    pub const LINE: Color = Color::DarkGray;

    pub const INFO: Color = Color::Cyan;
    pub const GOOD: Color = Color::Green;
    pub const WARN: Color = Color::Yellow;
    pub const BAD: Color = Color::Red;

    pub const RED: Color = Color::Red;
    pub const GOLD: Color = Color::Yellow;
    pub const BLACK: Color = Color::Black;
    pub const CHARCOAL: Color = Color::DarkGray;
    pub const WHITE: Color = Color::White;
    pub const CYAN: Color = Color::Cyan;
    pub const GREEN: Color = Color::Green;

    pub fn bg_style() -> Style {
        Style::default().bg(Self::SURFACE_BG).fg(Self::TEXT_PRIMARY)
    }

    pub fn text_primary() -> Style {
        Style::default().fg(Self::TEXT_PRIMARY)
    }

    pub fn text_muted() -> Style {
        Style::default().fg(Self::TEXT_MUTED)
    }

    pub fn text_dim() -> Style {
        Style::default().fg(Self::TEXT_DIM)
    }

    pub fn accent_info() -> Style {
        Style::default().fg(Self::INFO)
    }

    pub fn accent_good() -> Style {
        Style::default().fg(Self::GOOD)
    }

    pub fn accent_warn() -> Style {
        Style::default().fg(Self::WARN)
    }

    pub fn accent_bad() -> Style {
        Style::default().fg(Self::BAD)
    }

    pub fn border_primary() -> Style {
        Style::default().fg(Self::WARN)
    }

    pub fn border_secondary() -> Style {
        Style::default().fg(Self::TEXT_MUTED)
    }

    pub fn border_ghost() -> Style {
        Style::default().fg(Self::LINE)
    }

    pub fn primary_border() -> Style {
        Self::border_primary()
    }

    pub fn secondary_border() -> Style {
        Self::border_secondary()
    }

    pub fn title_style() -> Style {
        Self::accent_warn()
    }

    pub fn highlight() -> Style {
        Style::default().fg(Self::SURFACE_BG).bg(Self::WARN)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semantic_tokens_have_expected_roles() {
        assert_eq!(Theme::accent_info().fg, Some(Color::Cyan));
        assert_eq!(Theme::accent_good().fg, Some(Color::Green));
        assert_eq!(Theme::accent_warn().fg, Some(Color::Yellow));
        assert_eq!(Theme::accent_bad().fg, Some(Color::Red));
    }

    #[test]
    fn border_hierarchy_uses_distinct_emphasis_levels() {
        assert_ne!(Theme::border_primary().fg, Theme::border_secondary().fg);
        assert_ne!(Theme::border_secondary().fg, Theme::border_ghost().fg);
    }
}
