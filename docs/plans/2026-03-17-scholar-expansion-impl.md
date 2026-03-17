# Scholar Expansion Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add 4 new top-level tabs (Giờ Tốt, Ngũ Hành, Phong Thủy, Tiết Khí) and enrich the existing Scholar tab to surface all unused DayInsightDto data.

**Architecture:** Extend ActiveView enum with 4 new variants. Each new tab gets a screen widget file and 2-4 panel widgets. Scholar screen is redesigned from 2x2 to 3x2 grid. Ribbon adapts to 8-9 tabs with responsive abbreviation.

**Tech Stack:** Rust, ratatui (TUI framework), amlich-api DTOs

---

### Task 1: Extend ActiveView Enum and Navigation

**Files:**
- Modify: `crates/amlich-tui/src/state.rs:37-56` (ActiveView enum + label())
- Modify: `crates/amlich-tui/src/state.rs:463-474` (available_views())

**Step 1: Add new variants to ActiveView enum**

In `crates/amlich-tui/src/state.rs`, change the `ActiveView` enum (line 37-44):

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveView {
    Dashboard,
    Event,
    Scholar,
    Hours,       // NEW — Giờ Tốt
    Elements,    // NEW — Ngũ Hành
    FengShui,    // NEW — Phong Thủy
    SolarTerms,  // NEW — Tiết Khí
    Planning,
    Calendar,
}
```

**Step 2: Update label() method**

Change `label()` (line 46-56):

```rust
impl ActiveView {
    pub fn label(self) -> &'static str {
        match self {
            Self::Dashboard => "Dashboard",
            Self::Event => "Event",
            Self::Scholar => "Scholar",
            Self::Hours => "Giờ Tốt",
            Self::Elements => "Ngũ Hành",
            Self::FengShui => "Phong Thủy",
            Self::SolarTerms => "Tiết Khí",
            Self::Planning => "Planning",
            Self::Calendar => "Calendar",
        }
    }

    /// Short label for medium-width terminals
    pub fn short_label(self) -> &'static str {
        match self {
            Self::Dashboard => "Dash",
            Self::Event => "Evt",
            Self::Scholar => "Sch",
            Self::Hours => "Giờ",
            Self::Elements => "NHành",
            Self::FengShui => "PThủy",
            Self::SolarTerms => "TKhí",
            Self::Planning => "Plan",
            Self::Calendar => "Cal",
        }
    }
}
```

**Step 3: Update available_views()**

Change `available_views()` (line 463-474):

```rust
pub fn available_views(&self) -> Vec<ActiveView> {
    let mut views = vec![ActiveView::Dashboard];
    if self.has_event_today() {
        views.push(ActiveView::Event);
    }
    views.extend(vec![
        ActiveView::Scholar,
        ActiveView::Hours,
        ActiveView::Elements,
        ActiveView::FengShui,
        ActiveView::SolarTerms,
        ActiveView::Planning,
        ActiveView::Calendar,
    ]);
    views
}
```

**Step 4: Build and fix any exhaustive match errors**

Run: `cargo build -p amlich-tui 2>&1 | head -60`

Fix all exhaustive match errors by adding placeholder arms. In particular:
- `is_calendar_view()` — should return false for new variants
- Any other match on ActiveView

**Step 5: Commit**

```bash
git add -A && git commit -m "feat(amlich-tui): extend ActiveView with 4 new tab variants"
```

---

### Task 2: Update Keybindings for New Tabs

**Files:**
- Modify: `crates/amlich-tui/src/events.rs:55-71`

**Step 1: Remap number keys to cover all tabs**

Change the keybinding block (lines 55-71) to:

```rust
KeyCode::Char('1') => {
    app.go_to_view(crate::state::ActiveView::Dashboard);
    return false;
}
KeyCode::Char('2') => {
    app.go_to_view(crate::state::ActiveView::Scholar);
    return false;
}
KeyCode::Char('3') => {
    app.go_to_view(crate::state::ActiveView::Hours);
    return false;
}
KeyCode::Char('4') => {
    app.go_to_view(crate::state::ActiveView::Elements);
    return false;
}
KeyCode::Char('5') => {
    app.go_to_view(crate::state::ActiveView::FengShui);
    return false;
}
KeyCode::Char('6') => {
    app.go_to_view(crate::state::ActiveView::SolarTerms);
    return false;
}
KeyCode::Char('7') => {
    app.go_to_view(crate::state::ActiveView::Planning);
    return false;
}
KeyCode::Char('8') => {
    app.go_to_view(crate::state::ActiveView::Calendar);
    app.calendar_cursor = app.date;
    return false;
}
```

**Step 2: Build and verify**

Run: `cargo build -p amlich-tui`

**Step 3: Commit**

```bash
git add crates/amlich-tui/src/events.rs && git commit -m "feat(amlich-tui): remap number keys 1-8 for new tabs"
```

---

### Task 3: Update Ribbon for Responsive Tab Display

**Files:**
- Modify: `crates/amlich-tui/src/widgets/ribbon.rs`

**Step 1: Accept LayoutMode in RibbonWidget**

The ribbon already accepts `_mode` but ignores it. Store it:

```rust
pub struct RibbonWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> RibbonWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}
```

**Step 2: Update tab rendering logic for responsive labels**

Replace the view_spans building loop (lines 59-75) with:

```rust
let available = self.app.available_views();
let mut view_spans = vec![];
for v in available.iter() {
    let label = match self.mode {
        LayoutMode::Small => {
            // Small: only show active tab
            if v != &self.app.active_view {
                continue;
            }
            format!("< [{}] >", v.label())
        }
        LayoutMode::Medium => {
            if v == &self.app.active_view {
                format!(" [{}] ", v.short_label())
            } else {
                format!(" {} ", v.short_label())
            }
        }
        LayoutMode::Large => {
            if v == &self.app.active_view {
                format!(" [{}] ", v.label())
            } else {
                format!(" {} ", v.label())
            }
        }
    };

    let style = if v == &self.app.active_view {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    view_spans.push(Span::styled(label, style));
}
```

**Step 3: Build and run tests**

Run: `cargo test -p amlich-tui -- ribbon`

Note: Existing tests may need updating since they check for `[Dashboard]` — update assertions for the new behavior.

**Step 4: Commit**

```bash
git add crates/amlich-tui/src/widgets/ribbon.rs && git commit -m "feat(amlich-tui): responsive ribbon with abbreviated tab labels"
```

---

### Task 4: Create Stub Screen Widgets for New Tabs

**Files:**
- Create: `crates/amlich-tui/src/widgets/screens/hours.rs`
- Create: `crates/amlich-tui/src/widgets/screens/elements.rs`
- Create: `crates/amlich-tui/src/widgets/screens/feng_shui.rs`
- Create: `crates/amlich-tui/src/widgets/screens/solar_terms.rs`
- Modify: `crates/amlich-tui/src/widgets/screens/mod.rs`
- Modify: `crates/amlich-tui/src/widgets/page.rs:91-108`

**Step 1: Create stub screen widget files**

Each file follows the same pattern. Example for `hours.rs`:

```rust
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct HoursScreenWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> HoursScreenWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for HoursScreenWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Giờ Tốt ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let text = if self.app.bundle.is_some() {
            "Đang phát triển — Giờ Hoàng Đạo chi tiết"
        } else {
            "Chưa có dữ liệu."
        };

        Paragraph::new(text).block(block).render(area, buf);
    }
}
```

Create similar stubs for:
- `elements.rs` — title " Ngũ Hành ", placeholder "Đang phát triển — Phân tích Ngũ Hành"
- `feng_shui.rs` — title " Phong Thủy ", placeholder "Đang phát triển — Tứ Mệnh & Đại Vận"
- `solar_terms.rs` — title " Tiết Khí ", placeholder "Đang phát triển — Tiết Khí & Sức Khỏe"

**Step 2: Register modules**

In `crates/amlich-tui/src/widgets/screens/mod.rs`:

```rust
pub mod dashboard;
pub mod elements;
pub mod event;
pub mod feng_shui;
pub mod hours;
pub mod insight;
pub mod recommendations;
pub mod solar_terms;
```

**Step 3: Wire up page.rs routing**

In `crates/amlich-tui/src/widgets/page.rs`, add imports and match arms:

Add to imports (line 12-18):
```rust
use super::{
    calendar::CalendarViewWidget,
    screens::{
        dashboard::DashboardScreenWidget,
        elements::ElementsScreenWidget,
        feng_shui::FengShuiScreenWidget,
        hours::HoursScreenWidget,
        insight::InsightScreenWidget,
        recommendations::RecommendationsScreenWidget,
        solar_terms::SolarTermsScreenWidget,
    },
    week_strip::WeekStripWidget,
};
```

Expand the match block (line 91-108):
```rust
match self.app.active_view {
    crate::state::ActiveView::Dashboard => {
        DashboardScreenWidget::new(self.app, self.mode).render(content_area, buf)
    }
    crate::state::ActiveView::Event => {
        super::screens::event::EventScreenWidget::new(self.app, self.mode)
            .render(content_area, buf)
    }
    crate::state::ActiveView::Scholar => {
        InsightScreenWidget::new(self.app, self.mode).render(content_area, buf)
    }
    crate::state::ActiveView::Hours => {
        HoursScreenWidget::new(self.app, self.mode).render(content_area, buf)
    }
    crate::state::ActiveView::Elements => {
        ElementsScreenWidget::new(self.app, self.mode).render(content_area, buf)
    }
    crate::state::ActiveView::FengShui => {
        FengShuiScreenWidget::new(self.app, self.mode).render(content_area, buf)
    }
    crate::state::ActiveView::SolarTerms => {
        SolarTermsScreenWidget::new(self.app, self.mode).render(content_area, buf)
    }
    crate::state::ActiveView::Planning => {
        RecommendationsScreenWidget::new(self.app, self.mode).render(content_area, buf)
    }
    crate::state::ActiveView::Calendar => {
        CalendarViewWidget::new(self.app, self.mode).render(area, buf)
    }
}
```

**Step 4: Build and test**

Run: `cargo build -p amlich-tui && cargo test -p amlich-tui`

**Step 5: Commit**

```bash
git add -A && git commit -m "feat(amlich-tui): add stub screen widgets for 4 new tabs"
```

---

### Task 5: Enrich Scholar Screen — 3x2 Grid Layout

**Files:**
- Modify: `crates/amlich-tui/src/widgets/screens/insight.rs`
- Modify: `crates/amlich-tui/src/widgets/scholarly.rs`
- Create: `crates/amlich-tui/src/widgets/naam_panel.rs`
- Create: `crates/amlich-tui/src/widgets/direction_panel.rs`
- Create: `crates/amlich-tui/src/widgets/guidance_panel.rs`
- Modify: `crates/amlich-tui/src/widgets/mod.rs`

**Step 1: Create Nạp Âm & Ngũ Hành panel widget**

Create `crates/amlich-tui/src/widgets/naam_panel.rs`:

```rust
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct NaAmPanelWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> NaAmPanelWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for NaAmPanelWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Nạp Âm & Ngũ Hành ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let Some(bundle) = &self.app.bundle else { return };
        let mut lines: Vec<Line<'_>> = vec![];

        // Nạp âm from day_fortune
        if let Some(fortune) = &bundle.day_fortune {
            lines.push(Line::from(vec![
                Span::raw("   Nạp âm: "),
                Span::styled(&fortune.day_element.na_am, Style::default().fg(Color::Yellow)),
            ]));
            lines.push(Line::from(vec![
                Span::raw("   Ngũ hành Can: "),
                Span::styled(&fortune.day_element.element, Style::default().fg(Color::Cyan)),
            ]));
        }

        // Nạp âm insight meaning
        if let Some(insight) = &bundle.insight {
            if let Some(na_am) = &insight.na_am {
                lines.push(Line::from(vec![
                    Span::raw("   Hành: "),
                    Span::styled(&na_am.element, Style::default().fg(Color::Green)),
                ]));
                lines.push(Line::from(""));
                lines.push(Line::from(vec![
                    Span::styled("   Ý nghĩa: ", Style::default().fg(Color::DarkGray)),
                ]));
                // Wrap meaning text
                let meaning = &na_am.meaning.vi;
                for chunk in textwrap_simple(meaning, (inner.width as usize).saturating_sub(4)) {
                    lines.push(Line::from(format!("   {chunk}")));
                }
            }
        }

        // Con giáp 3 trụ
        if let Some(canchi) = &bundle.canchi {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::raw("   Con giáp ngày: "),
                Span::styled(&canchi.day.con_giap, Style::default().fg(Color::Cyan)),
            ]));
            lines.push(Line::from(vec![
                Span::raw("   Con giáp tháng: "),
                Span::styled(&canchi.month.con_giap, Style::default().fg(Color::Cyan)),
            ]));
            lines.push(Line::from(vec![
                Span::raw("   Con giáp năm: "),
                Span::styled(&canchi.year.con_giap, Style::default().fg(Color::Cyan)),
            ]));
        }

        Paragraph::new(lines).render(inner, buf);
    }
}

fn textwrap_simple(text: &str, width: usize) -> Vec<String> {
    if width == 0 { return vec![text.to_string()]; }
    let mut result = vec![];
    let mut current = String::new();
    for word in text.split_whitespace() {
        if current.len() + word.len() + 1 > width && !current.is_empty() {
            result.push(current);
            current = String::new();
        }
        if !current.is_empty() { current.push(' '); }
        current.push_str(word);
    }
    if !current.is_empty() { result.push(current); }
    result
}
```

**Step 2: Create Hướng & Thần panel widget**

Create `crates/amlich-tui/src/widgets/direction_panel.rs`:

```rust
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct DirectionPanelWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> DirectionPanelWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for DirectionPanelWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Hướng & Thần ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let Some(bundle) = &self.app.bundle else { return };
        let mut lines: Vec<Line<'_>> = vec![];

        // Travel directions from insight
        if let Some(insight) = &bundle.insight {
            if let Some(travel) = &insight.travel {
                lines.push(Line::from(vec![
                    Span::raw("   Xuất hành: "),
                    Span::styled(&travel.xuat_hanh_huong, Style::default().fg(Color::Green)),
                ]));
                lines.push(Line::from(vec![
                    Span::raw("   Hỷ Thần: "),
                    Span::styled(&travel.hy_than, Style::default().fg(Color::Green)),
                ]));
                lines.push(Line::from(vec![
                    Span::raw("   Tài Thần: "),
                    Span::styled(&travel.tai_than, Style::default().fg(Color::Yellow)),
                ]));
            }

            // Day deity details
            if let Some(deity) = &insight.day_deity {
                lines.push(Line::from(""));
                lines.push(Line::from(vec![
                    Span::raw("   Thần sát: "),
                    Span::styled(&deity.name, Style::default().fg(Color::Yellow)),
                ]));
                lines.push(Line::from(vec![
                    Span::raw("   Phân loại: "),
                    Span::styled(&deity.classification_meaning.vi, Style::default().fg(Color::Cyan)),
                ]));
                if let Some(meaning) = &deity.deity_meaning {
                    lines.push(Line::from(vec![
                        Span::styled("   Ý nghĩa: ", Style::default().fg(Color::DarkGray)),
                    ]));
                    lines.push(Line::from(format!("   {}", meaning.vi)));
                }
            }
        }

        Paragraph::new(lines).render(inner, buf);
    }
}
```

**Step 3: Create Nên Làm / Tránh panel widget**

Create `crates/amlich-tui/src/widgets/guidance_panel.rs`:

```rust
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct GuidancePanelWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> GuidancePanelWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for GuidancePanelWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Nên Làm / Tránh ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let Some(bundle) = &self.app.bundle else { return };
        let Some(insight) = &bundle.insight else { return };
        let mut lines: Vec<Line<'_>> = vec![];

        if let Some(guidance) = &insight.day_guidance {
            lines.push(Line::from(Span::styled(
                "   Nên làm:",
                Style::default().fg(Color::Green),
            )));
            for item in &guidance.good_for.vi {
                lines.push(Line::from(format!("    {} {item}", "\u{251C}")));
            }

            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "   Tránh làm:",
                Style::default().fg(Color::Red),
            )));
            for item in &guidance.avoid_for.vi {
                lines.push(Line::from(format!("    {} {item}", "\u{251C}")));
            }
        } else if let Some(truc) = &insight.truc {
            // Fallback to truc good_for/avoid_for
            lines.push(Line::from(Span::styled(
                "   Nên làm:",
                Style::default().fg(Color::Green),
            )));
            for item in &truc.good_for.vi {
                lines.push(Line::from(format!("    {} {item}", "\u{251C}")));
            }
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "   Tránh làm:",
                Style::default().fg(Color::Red),
            )));
            for item in &truc.avoid_for.vi {
                lines.push(Line::from(format!("    {} {item}", "\u{251C}")));
            }
        }

        Paragraph::new(lines).render(inner, buf);
    }
}
```

**Step 4: Enrich ScholarlyWidget with Can Chi insight**

Modify `crates/amlich-tui/src/widgets/scholarly.rs` — replace `render_evidence()` to include:
- Can Chi ngày with insight (meaning, nature from CanChiInsightDto)
- Can Chi tháng/năm
- Ngũ hành/Nạp âm (keep existing)
- Trực + meaning (keep existing)
- Cát tinh, Sát tinh (keep existing)
- Day star + quality (new, from StarsInsightDto)
- Thần sát (keep existing)

Key additions to the render_evidence method after existing code:

```rust
// After existing Can Chi ngày line, add insight meaning
if let Some(insight) = &bundle.insight {
    if let Some(canchi_insight) = &insight.canchi {
        lines.push(Line::from(vec![
            Span::raw("    \u{251C} Can: "),
            Span::styled(&canchi_insight.can.name, Style::default().fg(Color::Cyan)),
            Span::raw(" — "),
            Span::raw(&canchi_insight.can.meaning.vi),
        ]));
        lines.push(Line::from(vec![
            Span::raw("    \u{2514} Chi: "),
            Span::styled(&canchi_insight.chi.name, Style::default().fg(Color::Cyan)),
            Span::raw(" — "),
            Span::raw(&canchi_insight.chi.meaning.vi),
        ]));
    }
}

// Can Chi tháng and năm
if let Some(canchi) = &bundle.canchi {
    lines.push(Line::from(vec![
        Span::raw("   Can Chi tháng: "),
        Span::styled(&canchi.month.full, Style::default().fg(Color::Cyan)),
    ]));
    lines.push(Line::from(vec![
        Span::raw("   Can Chi năm: "),
        Span::styled(&canchi.year.full, Style::default().fg(Color::Cyan)),
    ]));
}

// Day star (after existing stars block)
if let Some(stars) = &insight.stars {
    if let Some(day_star) = &stars.day_star {
        let quality = stars.day_star_quality.as_deref().unwrap_or("");
        lines.push(Line::from(vec![
            Span::raw("   Sao chủ ngày: "),
            Span::styled(day_star, Style::default().fg(Color::Yellow)),
            Span::raw(format!(" ({quality})")),
        ]));
    }
}
```

**Step 5: Redesign InsightScreenWidget to 3x2 grid**

Replace the entire render method in `crates/amlich-tui/src/widgets/screens/insight.rs`:

```rust
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};

use crate::widgets::{
    direction_panel::DirectionPanelWidget,
    guidance_panel::GuidancePanelWidget,
    naam_panel::NaAmPanelWidget,
    risk::RiskWidget,
    scholarly::ScholarlyWidget,
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
            LayoutMode::Large => self.render_large(area, buf),
            LayoutMode::Medium => self.render_medium(area, buf),
            LayoutMode::Small => self.render_small(area, buf),
        }
    }
}

impl InsightScreenWidget<'_> {
    fn render_large(self, area: Rect, buf: &mut Buffer) {
        // 3x2 grid
        let rows = Layout::vertical([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]).split(area);

        let top = Layout::horizontal([
            Constraint::Percentage(34),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ]).split(rows[0]);

        let bottom = Layout::horizontal([
            Constraint::Percentage(34),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ]).split(rows[1]);

        ScholarlyWidget::new(self.app, self.mode).render(top[0], buf);
        // Top-center: Sao & Trực — reuse parts from scholarly or create new
        // For now, keep the existing scholarly + add a Sao panel
        RiskWidget::new(self.app, self.mode).render(top[2], buf);

        NaAmPanelWidget::new(self.app, self.mode).render(bottom[0], buf);
        DirectionPanelWidget::new(self.app, self.mode).render(bottom[1], buf);
        GuidancePanelWidget::new(self.app, self.mode).render(bottom[2], buf);

        // top[1] — Sao & Trực panel (extract from scholarly)
        self.render_stars_panel(top[1], buf);
    }

    fn render_medium(self, area: Rect, buf: &mut Buffer) {
        // 2x3 grid
        let rows = Layout::vertical([
            Constraint::Percentage(34),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ]).split(area);

        let row0 = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]).split(rows[0]);
        let row1 = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]).split(rows[1]);
        let row2 = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]).split(rows[2]);

        ScholarlyWidget::new(self.app, self.mode).render(row0[0], buf);
        self.render_stars_panel(row0[1], buf);
        RiskWidget::new(self.app, self.mode).render(row1[0], buf);
        NaAmPanelWidget::new(self.app, self.mode).render(row1[1], buf);
        DirectionPanelWidget::new(self.app, self.mode).render(row2[0], buf);
        GuidancePanelWidget::new(self.app, self.mode).render(row2[1], buf);
    }

    fn render_small(self, area: Rect, buf: &mut Buffer) {
        // Vertical stack — show panels in order, let them take what they need
        let rows = Layout::vertical([
            Constraint::Min(10),
            Constraint::Min(8),
            Constraint::Min(8),
            Constraint::Min(8),
            Constraint::Min(8),
            Constraint::Min(8),
        ]).split(area);

        ScholarlyWidget::new(self.app, self.mode).render(rows[0], buf);
        self.render_stars_panel(rows[1], buf);
        RiskWidget::new(self.app, self.mode).render(rows[2], buf);
        NaAmPanelWidget::new(self.app, self.mode).render(rows[3], buf);
        DirectionPanelWidget::new(self.app, self.mode).render(rows[4], buf);
        GuidancePanelWidget::new(self.app, self.mode).render(rows[5], buf);
    }

    fn render_stars_panel(&self, area: Rect, buf: &mut Buffer) {
        use ratatui::style::{Color, Style};
        use ratatui::text::{Line, Span};
        use ratatui::widgets::{Block, Borders, Paragraph};

        let block = Block::default()
            .title(" Sao & Trực ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let Some(bundle) = &self.app.bundle else { return };
        let Some(insight) = &bundle.insight else { return };
        let mut lines: Vec<Line<'_>> = vec![];

        if let Some(truc) = &insight.truc {
            lines.push(Line::from(vec![
                Span::raw("   Trực: "),
                Span::styled(&truc.name, Style::default().fg(Color::Cyan)),
                Span::raw(" ("),
                Span::raw(&truc.quality),
                Span::raw(")"),
            ]));
            lines.push(Line::from(format!("   {}", truc.meaning.vi)));
        }

        if let Some(stars) = &insight.stars {
            lines.push(Line::from(""));
            if let Some(day_star) = &stars.day_star {
                let quality_str = stars.day_star_quality.as_deref().unwrap_or("");
                lines.push(Line::from(vec![
                    Span::raw("   Sao ngày: "),
                    Span::styled(day_star.as_str(), Style::default().fg(Color::Yellow)),
                    Span::raw(format!(" ({quality_str})")),
                ]));
            }

            lines.push(Line::from(""));
            let cat_tinh = stars.cat_tinh.join(", ");
            lines.push(Line::from(vec![
                Span::raw("   Cát tinh: "),
                Span::styled(
                    if stars.cat_tinh.is_empty() { "Không".to_string() } else { cat_tinh },
                    Style::default().fg(Color::Green),
                ),
            ]));
            let sat_tinh = stars.sat_tinh.join(", ");
            lines.push(Line::from(vec![
                Span::raw("   Sát tinh: "),
                Span::styled(
                    if stars.sat_tinh.is_empty() { "Không".to_string() } else { sat_tinh },
                    Style::default().fg(Color::Red),
                ),
            ]));
        }

        if let Some(deity) = &insight.day_deity {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::raw("   Thần sát: "),
                Span::styled(&deity.name, Style::default().fg(Color::Yellow)),
                Span::raw(" ("),
                Span::raw(&deity.classification),
                Span::raw(")"),
            ]));
        }

        Paragraph::new(lines).render(inner, buf);
    }
}
```

**Step 6: Register new widget modules**

In `crates/amlich-tui/src/widgets/mod.rs`, add:
```rust
pub mod direction_panel;
pub mod guidance_panel;
pub mod naam_panel;
```

**Step 7: Build and test**

Run: `cargo build -p amlich-tui && cargo test -p amlich-tui`

**Step 8: Commit**

```bash
git add -A && git commit -m "feat(amlich-tui): redesign Scholar screen to 3x2 grid with enriched panels"
```

---

### Task 6: Implement Hours Screen (Giờ Tốt)

**Files:**
- Modify: `crates/amlich-tui/src/widgets/screens/hours.rs`
- Create: `crates/amlich-tui/src/widgets/hours_timeline.rs`
- Create: `crates/amlich-tui/src/widgets/hours_detail.rs`
- Modify: `crates/amlich-tui/src/widgets/mod.rs`

**Step 1: Create hours timeline widget**

Create `crates/amlich-tui/src/widgets/hours_timeline.rs`:

This widget renders all 12 hours as a horizontal bar. Each hour shows: Chi name, time range, good/bad marker, star name.

```rust
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct HoursTimelineWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> HoursTimelineWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for HoursTimelineWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Tổng Quan 12 Giờ ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let Some(bundle) = &self.app.bundle else { return };
        let Some(gio) = &bundle.gio_hoang_dao else { return };

        // Build 4 rows: Chi names, time ranges, good/bad markers, star names
        let mut chi_spans: Vec<Span<'_>> = vec![];
        let mut time_spans: Vec<Span<'_>> = vec![];
        let mut marker_spans: Vec<Span<'_>> = vec![];
        let mut star_spans: Vec<Span<'_>> = vec![];

        for hour in &gio.all_hours {
            let col_width = 10;
            let style = if hour.is_good {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            chi_spans.push(Span::styled(
                format!("{:^width$}", hour.hour_chi, width = col_width),
                style,
            ));
            time_spans.push(Span::styled(
                format!("{:^width$}", hour.time_range, width = col_width),
                Style::default().fg(Color::DarkGray),
            ));
            let marker = if hour.is_good { "\u{2605} Tốt" } else { "  Xấu" };
            marker_spans.push(Span::styled(
                format!("{:^width$}", marker, width = col_width),
                if hour.is_good {
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Red)
                },
            ));
            star_spans.push(Span::styled(
                format!("{:^width$}", hour.star, width = col_width),
                style,
            ));
        }

        let lines = vec![
            Line::from(chi_spans),
            Line::from(time_spans),
            Line::from(marker_spans),
            Line::from(star_spans),
        ];

        Paragraph::new(lines).render(inner, buf);
    }
}
```

**Step 2: Create hours detail widget**

Create `crates/amlich-tui/src/widgets/hours_detail.rs`:

```rust
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct HoursDetailWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
    show_good: bool,
}

impl<'a> HoursDetailWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode, show_good: bool) -> Self {
        Self { app, _mode: mode, show_good }
    }
}

impl Widget for HoursDetailWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = if self.show_good { " Giờ Tốt Chi Tiết " } else { " Giờ Xấu " };
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let Some(bundle) = &self.app.bundle else { return };
        let Some(gio) = &bundle.gio_hoang_dao else { return };
        let mut lines: Vec<Line<'_>> = vec![];

        let filtered: Vec<_> = gio.all_hours.iter()
            .filter(|h| h.is_good == self.show_good)
            .collect();

        for hour in &filtered {
            let marker = if self.show_good { "\u{2605}" } else { "\u{2022}" };
            let color = if self.show_good { Color::Green } else { Color::Red };
            lines.push(Line::from(vec![
                Span::styled(
                    format!("  {marker} {} ", hour.hour_chi),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("({}) ", hour.time_range),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::raw("— "),
                Span::styled(&hour.star, Style::default().fg(Color::Yellow)),
            ]));
        }

        if self.show_good {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled(
                    format!("  Tổng: {}/{} giờ tốt", filtered.len(), gio.all_hours.len()),
                    Style::default().fg(Color::DarkGray),
                ),
            ]));
        }

        Paragraph::new(lines).render(inner, buf);
    }
}
```

**Step 3: Wire up HoursScreenWidget**

Replace `crates/amlich-tui/src/widgets/screens/hours.rs`:

```rust
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};

use crate::widgets::hours_detail::HoursDetailWidget;
use crate::widgets::hours_timeline::HoursTimelineWidget;
use crate::{layout::LayoutMode, state::AppState};

pub struct HoursScreenWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> HoursScreenWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for HoursScreenWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let rows = Layout::vertical([
            Constraint::Length(7), // Timeline
            Constraint::Min(10),  // Detail
        ]).split(area);

        HoursTimelineWidget::new(self.app, self.mode).render(rows[0], buf);

        match self.mode {
            LayoutMode::Large | LayoutMode::Medium => {
                let cols = Layout::horizontal([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ]).split(rows[1]);
                HoursDetailWidget::new(self.app, self.mode, true).render(cols[0], buf);
                HoursDetailWidget::new(self.app, self.mode, false).render(cols[1], buf);
            }
            LayoutMode::Small => {
                let detail_rows = Layout::vertical([
                    Constraint::Percentage(60),
                    Constraint::Percentage(40),
                ]).split(rows[1]);
                HoursDetailWidget::new(self.app, self.mode, true).render(detail_rows[0], buf);
                HoursDetailWidget::new(self.app, self.mode, false).render(detail_rows[1], buf);
            }
        }
    }
}
```

**Step 4: Register modules**

In `crates/amlich-tui/src/widgets/mod.rs`, add:
```rust
pub mod hours_detail;
pub mod hours_timeline;
```

**Step 5: Build and test**

Run: `cargo build -p amlich-tui && cargo test -p amlich-tui`

**Step 6: Commit**

```bash
git add -A && git commit -m "feat(amlich-tui): implement Hours screen with timeline and detail panels"
```

---

### Task 7: Implement Elements Screen (Ngũ Hành)

**Files:**
- Modify: `crates/amlich-tui/src/widgets/screens/elements.rs`
- Create: `crates/amlich-tui/src/widgets/tang_can_panel.rs`
- Create: `crates/amlich-tui/src/widgets/ten_gods_panel.rs`
- Create: `crates/amlich-tui/src/widgets/xung_hop_panel.rs`
- Create: `crates/amlich-tui/src/widgets/pillars_panel.rs`
- Create: `crates/amlich-tui/src/widgets/element_chart_panel.rs`
- Modify: `crates/amlich-tui/src/widgets/mod.rs`

**Step 1: Create Tàng Can panel**

Create `crates/amlich-tui/src/widgets/tang_can_panel.rs`:

```rust
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct TangCanPanelWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> TangCanPanelWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for TangCanPanelWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Tàng Can ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let Some(bundle) = &self.app.bundle else { return };
        let Some(insight) = &bundle.insight else { return };
        let Some(tang_can) = &insight.tang_can else { return };
        let mut lines: Vec<Line<'_>> = vec![];

        if let Some(canchi) = &bundle.canchi {
            lines.push(Line::from(vec![
                Span::raw("   Chi ngày: "),
                Span::styled(&canchi.day.chi, Style::default().fg(Color::Cyan)),
            ]));
            lines.push(Line::from(""));
        }

        let labels = ["Chính", "Trung", "Dư"];
        let values = [&tang_can.main, &tang_can.central, &tang_can.residual];
        let strengths = tang_can.strength;

        for (i, (label, value)) in labels.iter().zip(values.iter()).enumerate() {
            let s = strengths[i];
            let bar_len = (s as usize * 10) / 100;
            let bar: String = "\u{2588}".repeat(bar_len) + &"\u{2591}".repeat(10 - bar_len);
            lines.push(Line::from(vec![
                Span::raw(format!("   {label}: ")),
                Span::styled(*value, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(format!(" {bar} {s}%")),
            ]));
        }

        Paragraph::new(lines).render(inner, buf);
    }
}
```

**Step 2: Create Thập Thần panel**

Create `crates/amlich-tui/src/widgets/ten_gods_panel.rs`:

```rust
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use amlich_api::TenGodsEntryInsightDto;
use crate::{layout::LayoutMode, state::AppState};

pub struct TenGodsPanelWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> TenGodsPanelWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for TenGodsPanelWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Thập Thần ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let Some(bundle) = &self.app.bundle else { return };
        let Some(insight) = &bundle.insight else { return };
        let Some(ten_gods) = &insight.ten_gods else { return };
        let mut lines: Vec<Line<'_>> = vec![];

        if let Some(to_year) = &ten_gods.to_year_stem {
            lines.push(Line::from(Span::styled(
                "   Với năm sinh (Can năm):",
                Style::default().fg(Color::DarkGray),
            )));
            render_ten_god_entry(&mut lines, to_year);
        }

        if let Some(to_self) = &ten_gods.to_self {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "   Với bản thân (Can ngày):",
                Style::default().fg(Color::DarkGray),
            )));
            render_ten_god_entry(&mut lines, to_self);
        }

        Paragraph::new(lines).render(inner, buf);
    }
}

fn render_ten_god_entry<'a>(lines: &mut Vec<Line<'a>>, entry: &'a TenGodsEntryInsightDto) {
    lines.push(Line::from(vec![
        Span::raw("    "),
        Span::styled(&entry.label, Style::default().fg(Color::Yellow)),
        Span::raw(": "),
        Span::styled(&entry.name.vi, Style::default().fg(Color::Cyan)),
    ]));
    lines.push(Line::from(format!("     Nghĩa: {}", entry.meaning.vi)));
    lines.push(Line::from(format!("     Quan hệ: {}", entry.relation)));
    let polarity = if entry.same_polarity { "đồng cực" } else { "khác cực" };
    lines.push(Line::from(format!("     Cực tính: {polarity}")));
}
```

**Step 3: Create Xung Hợp panel**

Create `crates/amlich-tui/src/widgets/xung_hop_panel.rs`:

```rust
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct XungHopPanelWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> XungHopPanelWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for XungHopPanelWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Xung Hợp ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let Some(bundle) = &self.app.bundle else { return };
        let Some(insight) = &bundle.insight else { return };
        let Some(xung_hop) = &insight.xung_hop else { return };
        let mut lines: Vec<Line<'_>> = vec![];

        lines.push(Line::from(vec![
            Span::raw("   Lục xung: "),
            Span::styled(&xung_hop.luc_xung, Style::default().fg(Color::Red)),
        ]));

        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::raw("   Tam hợp: "),
            Span::styled(
                xung_hop.tam_hop.join(" — "),
                Style::default().fg(Color::Green),
            ),
        ]));

        if let Some(liu_he) = &xung_hop.liu_he {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::raw("   Lục hợp: "),
                Span::styled(liu_he, Style::default().fg(Color::Green)),
            ]));
        }

        if let Some(xiang_hai) = &xung_hop.xiang_hai {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::raw("   Tương hại: "),
                Span::styled(xiang_hai, Style::default().fg(Color::Red)),
            ]));
        }

        Paragraph::new(lines).render(inner, buf);
    }
}
```

**Step 4: Create Pillars panel (3 trụ Can Chi)**

Create `crates/amlich-tui/src/widgets/pillars_panel.rs`:

```rust
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct PillarsPanelWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> PillarsPanelWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for PillarsPanelWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Can Chi 3 Trụ ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let Some(bundle) = &self.app.bundle else { return };
        let Some(canchi) = &bundle.canchi else { return };
        let mut lines: Vec<Line<'_>> = vec![];

        // Header
        lines.push(Line::from(vec![
            Span::raw("            "),
            Span::styled("Can    ", Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD)),
            Span::styled("Chi    ", Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD)),
            Span::styled("Hành", Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD)),
        ]));

        let pillars = [
            ("Năm:  ", &canchi.year),
            ("Tháng:", &canchi.month),
            ("Ngày: ", &canchi.day),
        ];

        for (label, p) in &pillars {
            lines.push(Line::from(vec![
                Span::raw(format!("   {label} ")),
                Span::styled(format!("{:<7}", p.can), Style::default().fg(Color::Cyan)),
                Span::styled(format!("{:<7}", p.chi), Style::default().fg(Color::Cyan)),
                Span::styled(
                    format!("{}/{}", p.ngu_hanh.can, p.ngu_hanh.chi),
                    Style::default().fg(Color::Yellow),
                ),
            ]));
        }

        // Nạp âm
        if let Some(fortune) = &bundle.day_fortune {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::raw("   Nạp âm: "),
                Span::styled(&fortune.day_element.na_am, Style::default().fg(Color::Yellow)),
            ]));
        }

        Paragraph::new(lines).render(inner, buf);
    }
}
```

**Step 5: Create Element Chart panel (Ngũ Hành tổng hợp)**

Create `crates/amlich-tui/src/widgets/element_chart_panel.rs`:

```rust
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct ElementChartPanelWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> ElementChartPanelWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for ElementChartPanelWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Ngũ Hành Tổng Hợp ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let Some(bundle) = &self.app.bundle else { return };
        let Some(canchi) = &bundle.canchi else { return };
        let mut lines: Vec<Line<'_>> = vec![];

        // Count elements from 3 pillars (6 elements total: 3 can + 3 chi)
        let elements = [
            &canchi.year.ngu_hanh.can,
            &canchi.year.ngu_hanh.chi,
            &canchi.month.ngu_hanh.can,
            &canchi.month.ngu_hanh.chi,
            &canchi.day.ngu_hanh.can,
            &canchi.day.ngu_hanh.chi,
        ];

        let names = ["Kim", "Mộc", "Thủy", "Hỏa", "Thổ"];
        let colors = [Color::White, Color::Green, Color::Blue, Color::Red, Color::Yellow];

        for (i, name) in names.iter().enumerate() {
            let count = elements.iter().filter(|e| e.as_str() == *name).count();
            let bar_len = count * 3;
            let bar: String = "\u{2588}".repeat(bar_len) + &"\u{2591}".repeat(18usize.saturating_sub(bar_len));
            lines.push(Line::from(vec![
                Span::styled(format!("   {name:<4} "), Style::default().fg(colors[i]).add_modifier(Modifier::BOLD)),
                Span::raw(format!("{bar} {count}/6")),
            ]));
        }

        // Dominant element
        let mut max_count = 0;
        let mut dominant = "";
        for name in &names {
            let count = elements.iter().filter(|e| e.as_str() == *name).count();
            if count > max_count {
                max_count = count;
                dominant = name;
            }
        }
        if !dominant.is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::raw("   Hành vượng: "),
                Span::styled(dominant, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]));
        }

        Paragraph::new(lines).render(inner, buf);
    }
}
```

**Step 6: Wire up ElementsScreenWidget**

Replace `crates/amlich-tui/src/widgets/screens/elements.rs`:

```rust
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};

use crate::widgets::element_chart_panel::ElementChartPanelWidget;
use crate::widgets::pillars_panel::PillarsPanelWidget;
use crate::widgets::tang_can_panel::TangCanPanelWidget;
use crate::widgets::ten_gods_panel::TenGodsPanelWidget;
use crate::widgets::xung_hop_panel::XungHopPanelWidget;
use crate::{layout::LayoutMode, state::AppState};

pub struct ElementsScreenWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> ElementsScreenWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for ElementsScreenWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.mode {
            LayoutMode::Large => {
                // 3x2 grid
                let rows = Layout::vertical([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ]).split(area);

                let top = Layout::horizontal([
                    Constraint::Percentage(34),
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                ]).split(rows[0]);

                let bottom = Layout::horizontal([
                    Constraint::Percentage(34),
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                ]).split(rows[1]);

                TangCanPanelWidget::new(self.app, self.mode).render(top[0], buf);
                TenGodsPanelWidget::new(self.app, self.mode).render(top[1], buf);
                XungHopPanelWidget::new(self.app, self.mode).render(top[2], buf);
                PillarsPanelWidget::new(self.app, self.mode).render(bottom[1], buf);
                ElementChartPanelWidget::new(self.app, self.mode).render(bottom[2], buf);

                // bottom[0]: Ngũ Hành Tương Quan (element relationships)
                self.render_element_relations(bottom[0], buf);
            }
            LayoutMode::Medium => {
                // 2x3 grid
                let rows = Layout::vertical([
                    Constraint::Percentage(34),
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                ]).split(area);

                let row0 = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(rows[0]);
                let row1 = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(rows[1]);
                let row2 = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(rows[2]);

                TangCanPanelWidget::new(self.app, self.mode).render(row0[0], buf);
                TenGodsPanelWidget::new(self.app, self.mode).render(row0[1], buf);
                XungHopPanelWidget::new(self.app, self.mode).render(row1[0], buf);
                self.render_element_relations(row1[1], buf);
                PillarsPanelWidget::new(self.app, self.mode).render(row2[0], buf);
                ElementChartPanelWidget::new(self.app, self.mode).render(row2[1], buf);
            }
            LayoutMode::Small => {
                let rows = Layout::vertical([
                    Constraint::Min(8),
                    Constraint::Min(10),
                    Constraint::Min(8),
                    Constraint::Min(8),
                    Constraint::Min(8),
                    Constraint::Min(8),
                ]).split(area);

                TangCanPanelWidget::new(self.app, self.mode).render(rows[0], buf);
                TenGodsPanelWidget::new(self.app, self.mode).render(rows[1], buf);
                XungHopPanelWidget::new(self.app, self.mode).render(rows[2], buf);
                self.render_element_relations(rows[3], buf);
                PillarsPanelWidget::new(self.app, self.mode).render(rows[4], buf);
                ElementChartPanelWidget::new(self.app, self.mode).render(rows[5], buf);
            }
        }
    }
}

impl ElementsScreenWidget<'_> {
    fn render_element_relations(&self, area: Rect, buf: &mut Buffer) {
        use ratatui::style::{Color, Style};
        use ratatui::text::{Line, Span};
        use ratatui::widgets::{Block, Borders, Paragraph};

        let block = Block::default()
            .title(" Ngũ Hành Tương Quan ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let Some(bundle) = &self.app.bundle else { return };
        let Some(canchi) = &bundle.canchi else { return };
        let mut lines: Vec<Line<'_>> = vec![];

        let can_element = &canchi.day.ngu_hanh.can;
        let chi_element = &canchi.day.ngu_hanh.chi;

        lines.push(Line::from(vec![
            Span::raw("   Can ngày: "),
            Span::styled(format!("{} ({})", canchi.day.can, can_element), Style::default().fg(Color::Cyan)),
        ]));
        lines.push(Line::from(vec![
            Span::raw("   Chi ngày: "),
            Span::styled(format!("{} ({})", canchi.day.chi, chi_element), Style::default().fg(Color::Cyan)),
        ]));

        // Determine relationship
        let relation = element_relation(can_element, chi_element);
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::raw("   Quan hệ: "),
            Span::styled(
                format!("{can_element} {relation} {chi_element}"),
                Style::default().fg(if relation.contains("sinh") { Color::Green } else { Color::Red }),
            ),
        ]));

        Paragraph::new(lines).render(inner, buf);
    }
}

fn element_relation(a: &str, b: &str) -> &'static str {
    match (a, b) {
        ("Kim", "Thủy") | ("Thủy", "Mộc") | ("Mộc", "Hỏa") | ("Hỏa", "Thổ") | ("Thổ", "Kim") => "sinh",
        ("Thủy", "Kim") | ("Mộc", "Thủy") | ("Hỏa", "Mộc") | ("Thổ", "Hỏa") | ("Kim", "Thổ") => "được sinh bởi",
        ("Kim", "Mộc") | ("Mộc", "Thổ") | ("Thổ", "Thủy") | ("Thủy", "Hỏa") | ("Hỏa", "Kim") => "khắc",
        ("Mộc", "Kim") | ("Thổ", "Mộc") | ("Thủy", "Thổ") | ("Hỏa", "Thủy") | ("Kim", "Hỏa") => "bị khắc bởi",
        _ if a == b => "tỷ hòa",
        _ => "—",
    }
}
```

**Step 7: Register modules in mod.rs**

Add to `crates/amlich-tui/src/widgets/mod.rs`:
```rust
pub mod element_chart_panel;
pub mod pillars_panel;
pub mod tang_can_panel;
pub mod ten_gods_panel;
pub mod xung_hop_panel;
```

**Step 8: Build and test**

Run: `cargo build -p amlich-tui && cargo test -p amlich-tui`

**Step 9: Commit**

```bash
git add -A && git commit -m "feat(amlich-tui): implement Elements screen with 6 analysis panels"
```

---

### Task 8: Implement FengShui Screen (Phong Thủy)

**Files:**
- Modify: `crates/amlich-tui/src/widgets/screens/feng_shui.rs`
- Create: `crates/amlich-tui/src/widgets/kua_panel.rs`
- Create: `crates/amlich-tui/src/widgets/dai_van_panel.rs`
- Create: `crates/amlich-tui/src/widgets/compass_panel.rs`
- Create: `crates/amlich-tui/src/widgets/directions_panel.rs`
- Modify: `crates/amlich-tui/src/widgets/mod.rs`

**Step 1: Create Kua panel**

Create `crates/amlich-tui/src/widgets/kua_panel.rs`:

```rust
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct KuaPanelWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> KuaPanelWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for KuaPanelWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Tứ Mệnh / Kua ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let Some(bundle) = &self.app.bundle else { return };
        let Some(insight) = &bundle.insight else { return };
        let Some(tu_menh) = &insight.tu_menh else { return };
        let mut lines: Vec<Line<'_>> = vec![];

        lines.push(Line::from(vec![
            Span::raw("   Quẻ số: "),
            Span::styled(
                tu_menh.kua.to_string(),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::raw("   Quẻ: "),
            Span::styled(&tu_menh.trigram.vi, Style::default().fg(Color::Cyan)),
        ]));
        lines.push(Line::from(vec![
            Span::raw("   Nhóm: "),
            Span::styled(&tu_menh.group, Style::default().fg(Color::Green)),
        ]));
        lines.push(Line::from(format!("    \u{2514} {}", tu_menh.group_meaning.vi)));

        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::raw("   Hướng mệnh: "),
            Span::styled(&tu_menh.direction.vi, Style::default().fg(Color::Yellow)),
        ]));
        lines.push(Line::from(format!("    \u{2514} {}", tu_menh.meaning.vi)));

        Paragraph::new(lines).render(inner, buf);
    }
}
```

**Step 2: Create Directions panel (hướng tốt/xấu)**

Create `crates/amlich-tui/src/widgets/directions_panel.rs`:

```rust
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct DirectionsPanelWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> DirectionsPanelWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for DirectionsPanelWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Hướng Tốt / Xấu ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let Some(bundle) = &self.app.bundle else { return };
        let Some(insight) = &bundle.insight else { return };
        let Some(tu_menh) = &insight.tu_menh else { return };
        let mut lines: Vec<Line<'_>> = vec![];

        lines.push(Line::from(Span::styled(
            "   Hướng tốt:",
            Style::default().fg(Color::Green),
        )));
        for dir in &tu_menh.favorable_directions {
            lines.push(Line::from(vec![
                Span::raw("    \u{251C} "),
                Span::styled(dir, Style::default().fg(Color::Green)),
            ]));
        }

        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "   Hướng xấu:",
            Style::default().fg(Color::Red),
        )));
        for dir in &tu_menh.unfavorable_directions {
            lines.push(Line::from(vec![
                Span::raw("    \u{251C} "),
                Span::styled(dir, Style::default().fg(Color::Red)),
            ]));
        }

        Paragraph::new(lines).render(inner, buf);
    }
}
```

**Step 3: Create Đại Vận panel**

Create `crates/amlich-tui/src/widgets/dai_van_panel.rs`:

```rust
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct DaiVanPanelWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> DaiVanPanelWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for DaiVanPanelWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Đại Vận ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let Some(bundle) = &self.app.bundle else { return };
        let Some(insight) = &bundle.insight else { return };
        let Some(dai_van) = &insight.dai_van else { return };
        let mut lines: Vec<Line<'_>> = vec![];

        lines.push(Line::from(vec![
            Span::raw("   Hướng vận: "),
            Span::styled(&dai_van.direction, Style::default().fg(Color::Yellow)),
        ]));
        lines.push(Line::from(format!("    \u{2514} {}", dai_van.direction_meaning.vi)));

        // Current pillar
        if let Some(current) = &dai_van.current_pillar {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "   Đại vận hiện tại:",
                Style::default().fg(Color::DarkGray),
            )));
            lines.push(Line::from(vec![
                Span::styled(
                    format!("   \u{25B6} {} ", current.can_chi),
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                ),
                Span::raw(format!(
                    "({}-{} tuổi) ",
                    current.start_age as u32, current.end_age as u32
                )),
                Span::styled(&current.element, Style::default().fg(Color::Yellow)),
            ]));
            lines.push(Line::from(format!("     {}", current.element_meaning.vi)));
        }

        // All pillars
        lines.push(Line::from(""));
        for pillar in &dai_van.all_pillars {
            let is_current = dai_van.current_pillar.as_ref()
                .map(|c| c.index == pillar.index)
                .unwrap_or(false);

            let marker = if is_current { "\u{25C4}" } else { " " };
            let style = if is_current {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };

            lines.push(Line::from(vec![
                Span::styled(
                    format!(
                        "   {}. {:<10} ({:>2}-{:>2}) {:>4} {marker}",
                        pillar.index,
                        pillar.can_chi,
                        pillar.start_age as u32,
                        pillar.end_age as u32,
                        pillar.element,
                    ),
                    style,
                ),
            ]));
        }

        Paragraph::new(lines).render(inner, buf);
    }
}
```

**Step 4: Create Compass panel (ASCII la bàn)**

Create `crates/amlich-tui/src/widgets/compass_panel.rs`:

```rust
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct CompassPanelWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> CompassPanelWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for CompassPanelWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" La Bàn Hướng ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let Some(bundle) = &self.app.bundle else { return };
        let Some(insight) = &bundle.insight else { return };
        let Some(tu_menh) = &insight.tu_menh else { return };

        let good: Vec<&str> = tu_menh.favorable_directions.iter().map(|s| s.as_str()).collect();
        let bad: Vec<&str> = tu_menh.unfavorable_directions.iter().map(|s| s.as_str()).collect();

        let dir_style = |name: &str| -> Style {
            if good.iter().any(|d| d.contains(name)) {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            } else if bad.iter().any(|d| d.contains(name)) {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::DarkGray)
            }
        };

        let marker = |name: &str| -> &'static str {
            if good.iter().any(|d| d.contains(name)) { "\u{2605}" }
            else if bad.iter().any(|d| d.contains(name)) { "\u{2716}" }
            else { "\u{00B7}" }
        };

        // Simple ASCII compass
        let lines = vec![
            Line::from(""),
            Line::from(vec![
                Span::raw("          "),
                Span::styled(format!("{} Bắc", marker("Bắc")), dir_style("Bắc")),
            ]),
            Line::from(vec![
                Span::raw("     "),
                Span::styled(format!("{} TB", marker("Tây Bắc")), dir_style("Tây Bắc")),
                Span::raw("    |    "),
                Span::styled(format!("ĐB {}", marker("Đông Bắc")), dir_style("Đông Bắc")),
            ]),
            Line::from(vec![
                Span::raw("          "),
                Span::raw("   |"),
            ]),
            Line::from(vec![
                Span::raw("    "),
                Span::styled(format!("{} Tây", marker("Tây")), dir_style("Tây")),
                Span::raw(" ——\u{25CF}—— "),
                Span::styled(format!("Đông {}", marker("Đông")), dir_style("Đông")),
            ]),
            Line::from(vec![
                Span::raw("          "),
                Span::raw("   |"),
            ]),
            Line::from(vec![
                Span::raw("     "),
                Span::styled(format!("{} TN", marker("Tây Nam")), dir_style("Tây Nam")),
                Span::raw("    |    "),
                Span::styled(format!("ĐN {}", marker("Đông Nam")), dir_style("Đông Nam")),
            ]),
            Line::from(vec![
                Span::raw("          "),
                Span::styled(format!("{} Nam", marker("Nam")), dir_style("Nam")),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(" \u{2605} Tốt ", Style::default().fg(Color::Green)),
                Span::styled(" \u{2716} Xấu", Style::default().fg(Color::Red)),
            ]),
        ];

        Paragraph::new(lines).render(inner, buf);
    }
}
```

**Step 5: Wire up FengShuiScreenWidget**

Replace `crates/amlich-tui/src/widgets/screens/feng_shui.rs`:

```rust
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};

use crate::widgets::compass_panel::CompassPanelWidget;
use crate::widgets::dai_van_panel::DaiVanPanelWidget;
use crate::widgets::directions_panel::DirectionsPanelWidget;
use crate::widgets::kua_panel::KuaPanelWidget;
use crate::{layout::LayoutMode, state::AppState};

pub struct FengShuiScreenWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> FengShuiScreenWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for FengShuiScreenWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.mode {
            LayoutMode::Large | LayoutMode::Medium => {
                // 2x2 grid
                let rows = Layout::vertical([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ]).split(area);

                let top = Layout::horizontal([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ]).split(rows[0]);

                let bottom = Layout::horizontal([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ]).split(rows[1]);

                KuaPanelWidget::new(self.app, self.mode).render(top[0], buf);
                DirectionsPanelWidget::new(self.app, self.mode).render(top[1], buf);
                DaiVanPanelWidget::new(self.app, self.mode).render(bottom[0], buf);
                CompassPanelWidget::new(self.app, self.mode).render(bottom[1], buf);
            }
            LayoutMode::Small => {
                let rows = Layout::vertical([
                    Constraint::Min(9),
                    Constraint::Min(10),
                    Constraint::Min(12),
                ]).split(area);

                KuaPanelWidget::new(self.app, self.mode).render(rows[0], buf);
                DirectionsPanelWidget::new(self.app, self.mode).render(rows[1], buf);
                DaiVanPanelWidget::new(self.app, self.mode).render(rows[2], buf);
                // Skip compass in small mode
            }
        }
    }
}
```

**Step 6: Register modules**

Add to `crates/amlich-tui/src/widgets/mod.rs`:
```rust
pub mod compass_panel;
pub mod dai_van_panel;
pub mod directions_panel;
pub mod kua_panel;
```

**Step 7: Build and test**

Run: `cargo build -p amlich-tui && cargo test -p amlich-tui`

**Step 8: Commit**

```bash
git add -A && git commit -m "feat(amlich-tui): implement FengShui screen with Kua, directions, Dai Van, compass"
```

---

### Task 9: Implement SolarTerms Screen (Tiết Khí)

**Files:**
- Modify: `crates/amlich-tui/src/widgets/screens/solar_terms.rs`

**Step 1: Implement full SolarTermsScreenWidget**

Replace `crates/amlich-tui/src/widgets/screens/solar_terms.rs`:

```rust
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct SolarTermsScreenWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> SolarTermsScreenWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for SolarTermsScreenWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.mode {
            LayoutMode::Large | LayoutMode::Medium => {
                let rows = Layout::vertical([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ]).split(area);

                let top = Layout::horizontal([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ]).split(rows[0]);

                let bottom = Layout::horizontal([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ]).split(rows[1]);

                self.render_current(top[0], buf);
                self.render_astronomy(top[1], buf);
                self.render_agriculture(bottom[0], buf);
                self.render_health(bottom[1], buf);
            }
            LayoutMode::Small => {
                let rows = Layout::vertical([
                    Constraint::Min(10),
                    Constraint::Min(8),
                    Constraint::Min(8),
                    Constraint::Min(8),
                ]).split(area);

                self.render_current(rows[0], buf);
                self.render_astronomy(rows[1], buf);
                self.render_agriculture(rows[2], buf);
                self.render_health(rows[3], buf);
            }
        }
    }
}

impl SolarTermsScreenWidget<'_> {
    fn render_current(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Tiết Khí Hiện Tại ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let Some(bundle) = &self.app.bundle else { return };
        let mut lines: Vec<Line<'_>> = vec![];

        // From TietKhiDto (bundle level)
        if let Some(tk) = &bundle.tiet_khi {
            lines.push(Line::from(vec![
                Span::raw("   Tiết khí: "),
                Span::styled(
                    &tk.name,
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                ),
            ]));
            lines.push(Line::from(vec![
                Span::raw("   Kinh độ: "),
                Span::styled(
                    format!("{}°", tk.longitude),
                    Style::default().fg(Color::Yellow),
                ),
            ]));
            lines.push(Line::from(vec![
                Span::raw("   Mùa: "),
                Span::styled(&tk.season, Style::default().fg(Color::Green)),
            ]));
            lines.push(Line::from(""));
            lines.push(Line::from(format!("   {}", tk.description)));
        }

        // From TietKhiInsightDto (insight level)
        if let Some(insight) = &bundle.insight {
            if let Some(tki) = &insight.tiet_khi {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    "   Ý nghĩa:",
                    Style::default().fg(Color::DarkGray),
                )));
                lines.push(Line::from(format!("   {}", tki.meaning.vi)));
            }
        }

        Paragraph::new(lines).render(inner, buf);
    }

    fn render_astronomy(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Thiên Văn & Thời Tiết ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let Some(bundle) = &self.app.bundle else { return };
        let Some(insight) = &bundle.insight else { return };
        let Some(tki) = &insight.tiet_khi else { return };
        let mut lines: Vec<Line<'_>> = vec![];

        lines.push(Line::from(Span::styled(
            "   Thiên văn:",
            Style::default().fg(Color::DarkGray),
        )));
        lines.push(Line::from(format!("   {}", tki.astronomy.vi)));

        if let Some(tk) = &bundle.tiet_khi {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::raw("   Kinh độ hiện tại: "),
                Span::styled(
                    format!("{:.1}°", tk.current_longitude),
                    Style::default().fg(Color::Yellow),
                ),
            ]));
        }

        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "   Thời tiết:",
            Style::default().fg(Color::DarkGray),
        )));
        lines.push(Line::from(format!("   {}", tki.weather.vi)));

        Paragraph::new(lines).render(inner, buf);
    }

    fn render_agriculture(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Nông Nghiệp ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let Some(bundle) = &self.app.bundle else { return };
        let Some(insight) = &bundle.insight else { return };
        let Some(tki) = &insight.tiet_khi else { return };
        let mut lines: Vec<Line<'_>> = vec![];

        lines.push(Line::from(Span::styled(
            "   Hoạt động nông vụ:",
            Style::default().fg(Color::Green),
        )));
        for item in &tki.agriculture.vi {
            lines.push(Line::from(format!("    \u{251C} {item}")));
        }

        Paragraph::new(lines).render(inner, buf);
    }

    fn render_health(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Sức Khỏe ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let Some(bundle) = &self.app.bundle else { return };
        let Some(insight) = &bundle.insight else { return };
        let Some(tki) = &insight.tiet_khi else { return };
        let mut lines: Vec<Line<'_>> = vec![];

        lines.push(Line::from(Span::styled(
            "   Lời khuyên sức khỏe:",
            Style::default().fg(Color::Cyan),
        )));
        for item in &tki.health.vi {
            lines.push(Line::from(format!("    \u{251C} {item}")));
        }

        Paragraph::new(lines).render(inner, buf);
    }
}
```

**Step 2: Build and test**

Run: `cargo build -p amlich-tui && cargo test -p amlich-tui`

**Step 3: Commit**

```bash
git add -A && git commit -m "feat(amlich-tui): implement SolarTerms screen with 4 insight panels"
```

---

### Task 10: Update Help Text and Final Polish

**Files:**
- Modify: `crates/amlich-tui/src/widgets/help.rs` (if exists)
- Modify: `crates/amlich-tui/src/widgets/ribbon.rs` (help text in ribbon)

**Step 1: Update ribbon help text**

In ribbon.rs, update the help text span to reflect new key range:

```rust
all_spans.push(Span::styled(
    "| Tab: màn  1-8: chọn  ←/→: ngày  t: hôm nay  m: tháng  ?: trợ giúp",
    Style::default().fg(Color::DarkGray),
));
```

**Step 2: Update any existing tests**

Run: `cargo test -p amlich-tui 2>&1 | head -40`

Fix any failing tests (especially ribbon tests that check for old text).

**Step 3: Full build and test**

Run: `cargo build -p amlich-tui && cargo test -p amlich-tui`

**Step 4: Manual smoke test**

Run: `cargo run -p amlich-tui`

Verify:
- Tab/Shift+Tab cycles through all 8-9 tabs
- Number keys 1-8 jump to correct tabs
- Each new tab renders its panels
- Large/Medium/Small layouts respond correctly (resize terminal to test)
- Scholar tab shows enriched 3x2 grid
- No panics on missing data (all fields are Option)

**Step 5: Commit**

```bash
git add -A && git commit -m "feat(amlich-tui): update help text and polish for new tab system"
```
