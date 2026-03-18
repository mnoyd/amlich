use ratatui::{
    layout::{Alignment, Rect},
    widgets::{Block, Borders},
};

use crate::layout::LayoutMode;
use crate::theme::Theme;

#[derive(Clone, Copy, Debug)]
pub enum ModalPreset {
    Search,
    Context,
    Help,
}

pub fn centered_modal(area: Rect, mode: LayoutMode, preset: ModalPreset) -> Rect {
    let (width, height) = match (preset, mode) {
        (ModalPreset::Search, LayoutMode::Small) => (area.width.saturating_sub(2).max(30), 7),
        (ModalPreset::Search, LayoutMode::Medium) => (48, 7),
        (ModalPreset::Search, LayoutMode::Large) => (54, 7),

        (ModalPreset::Context, LayoutMode::Small) => (area.width.saturating_sub(2).max(36), 18),
        (ModalPreset::Context, LayoutMode::Medium) => (64, 20),
        (ModalPreset::Context, LayoutMode::Large) => (72, 22),

        (ModalPreset::Help, LayoutMode::Small) => (area.width.saturating_sub(2).max(44), 24),
        (ModalPreset::Help, LayoutMode::Medium) => (72, 26),
        (ModalPreset::Help, LayoutMode::Large) => (78, 28),
    };

    Rect {
        x: area.x + area.width.saturating_sub(width) / 2,
        y: area.y + area.height.saturating_sub(height) / 2,
        width: width.min(area.width),
        height: height.min(area.height),
    }
}

pub fn modal_block(title: &str) -> Block<'_> {
    Block::default()
        .title(title)
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Theme::border_primary())
}
