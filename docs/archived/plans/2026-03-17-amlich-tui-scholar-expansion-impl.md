# AmLich-TUI Scholar Expansion Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add 4 new top-level tabs (Giờ Tốt, Ngũ Hành, Phong Thủy, Tiết Khí) to amlich-tui and enrich the Scholar tab from 2x2 to 3x2 grid, surfacing all unused DayInsightDto/DayBundleDto data.

**Architecture:** Extend ActiveView enum in amlich-tui with 4 new variants. Each new tab is a screen widget file in `widgets/screens/`. Scholar screen is redesigned to 3x2 with new panel widgets. Ribbon adapts to 8+ tabs with responsive abbreviation per LayoutMode. All data comes from `bundle: Option<DayBundleDto>` already cached in AppState.

**Tech Stack:** Rust, ratatui 0.29, amlich-api DTOs (DayBundleDto, DayInsightDto), crossterm 0.28

**IMPORTANT:** The cargo package name is `amlich-tui`. All cargo commands use `-p amlich-tui`. The crate directory is `crates/amlich-tui/`.

---

### Task 1: Extend ActiveView Enum and Navigation

**Files:**
- Modify: `crates/amlich-tui/src/state.rs:37-56` (ActiveView enum + label)
- Modify: `crates/amlich-tui/src/state.rs:463-474` (available_views)

**Step 1: Add new variants and labels**

Replace ActiveView enum and impl at lines 37-56:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveView {
    Dashboard,
    Event,
    Scholar,
    Hours,
    Elements,
    FengShui,
    SolarTerms,
    Planning,
    Calendar,
}

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

**Step 2: Update available_views()**

Replace available_views() at lines 463-474:

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

**Step 3: Build and fix exhaustive match errors**

Run: `cargo build -p amlich-tui 2>&1 | head -60`

The compiler will flag exhaustive matches in `is_calendar_view()` and any other place matching ActiveView. Add the new variants returning `false` (or whatever the default arm is) in each case.

**Step 4: Run tests**

Run: `cargo test -p amlich-tui 2>&1 | tail -20`

**Step 5: Commit**

```bash
git add crates/amlich-tui/src/state.rs && git commit -m "feat(amlich-tui): extend ActiveView with Hours, Elements, FengShui, SolarTerms"
```

---

### Task 2: Update Keybindings

**Files:**
- Modify: `crates/amlich-tui/src/events.rs:55-71`

**Step 1: Remap number keys 1-8**

Replace the keybinding block at lines 55-71:

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

**Step 2: Build and test**

Run: `cargo build -p amlich-tui && cargo test -p amlich-tui 2>&1 | tail -10`

**Step 3: Commit**

```bash
git add crates/amlich-tui/src/events.rs && git commit -m "feat(amlich-tui): remap number keys 1-8 for expanded tab set"
```

---

### Task 3: Responsive Ribbon

**Files:**
- Modify: `crates/amlich-tui/src/widgets/ribbon.rs:17-81`

**Step 1: Store LayoutMode and use responsive labels**

Replace the struct, constructor, and tab rendering section (lines 17-81):

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

Then replace the tab rendering loop inside `render()` (lines 59-81):

```rust
let available = self.app.available_views();
let mut view_spans = vec![];
for v in available.iter() {
    let label = match self.mode {
        LayoutMode::Small => {
            if v != &self.app.active_view {
                continue;
            }
            format!("< [{}] >", v.short_label())
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

let mut all_spans = view_spans;
all_spans.push(Span::styled(
    "| Tab: màn  1-8: chọn  ←/→: ngày  t: hôm nay  ?: trợ giúp",
    Style::default().fg(Color::DarkGray),
));
```

**Step 2: Update ribbon tests**

The existing tests check for `[Dashboard]` text. Update them to pass with the new behavior. The tests use `LayoutMode::Large` so full labels should still appear.

**Step 3: Build and test**

Run: `cargo build -p amlich-tui && cargo test -p amlich-tui -- ribbon 2>&1`

**Step 4: Commit**

```bash
git add crates/amlich-tui/src/widgets/ribbon.rs && git commit -m "feat(amlich-tui): responsive ribbon with abbreviated labels for 8+ tabs"
```

---

### Task 4: Create Stub Screen Widgets and Wire Routing

**Files:**
- Create: `crates/amlich-tui/src/widgets/screens/hours.rs`
- Create: `crates/amlich-tui/src/widgets/screens/elements.rs`
- Create: `crates/amlich-tui/src/widgets/screens/feng_shui.rs`
- Create: `crates/amlich-tui/src/widgets/screens/solar_terms.rs`
- Modify: `crates/amlich-tui/src/widgets/screens/mod.rs`
- Modify: `crates/amlich-tui/src/widgets/page.rs:12-108`

**Step 1: Create 4 stub screen files**

Each stub follows the same pattern. Create `crates/amlich-tui/src/widgets/screens/hours.rs`:

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
    _mode: LayoutMode,
}

impl<'a> HoursScreenWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
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

Create `crates/amlich-tui/src/widgets/screens/elements.rs` (same pattern, title " Ngũ Hành ", text "Đang phát triển — Phân tích Ngũ Hành").

Create `crates/amlich-tui/src/widgets/screens/feng_shui.rs` (title " Phong Thủy ", text "Đang phát triển — Tứ Mệnh & Đại Vận").

Create `crates/amlich-tui/src/widgets/screens/solar_terms.rs` (title " Tiết Khí ", text "Đang phát triển — Tiết Khí & Sức Khỏe").

**Step 2: Register screen modules**

Replace `crates/amlich-tui/src/widgets/screens/mod.rs`:

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

In `crates/amlich-tui/src/widgets/page.rs`, add imports at top (after existing imports around line 12-18):

```rust
use super::screens::{
    dashboard::DashboardScreenWidget,
    elements::ElementsScreenWidget,
    feng_shui::FengShuiScreenWidget,
    hours::HoursScreenWidget,
    insight::InsightScreenWidget,
    recommendations::RecommendationsScreenWidget,
    solar_terms::SolarTermsScreenWidget,
};
```

Replace the match block at lines 91-108:

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

Run: `cargo build -p amlich-tui && cargo test -p amlich-tui 2>&1 | tail -15`

**Step 5: Commit**

```bash
git add -A crates/amlich-tui/ && git commit -m "feat(amlich-tui): add stub screen widgets for 4 new tabs and wire routing"
```

---

### Task 5: Enrich Scholar Screen to 3x2 Grid

**Files:**
- Modify: `crates/amlich-tui/src/widgets/screens/insight.rs` (full rewrite)
- Modify: `crates/amlich-tui/src/widgets/scholarly.rs` (enrich with CanChi insight)
- Modify: `crates/amlich-tui/src/widgets/mod.rs` (add new modules)
- Create: `crates/amlich-tui/src/widgets/stars_panel.rs`
- Create: `crates/amlich-tui/src/widgets/naam_panel.rs`
- Create: `crates/amlich-tui/src/widgets/direction_panel.rs`
- Create: `crates/amlich-tui/src/widgets/guidance_panel.rs`

**Step 1: Create stars_panel.rs**

Create `crates/amlich-tui/src/widgets/stars_panel.rs`:

```rust
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct StarsPanelWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> StarsPanelWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for StarsPanelWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
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
                Span::raw("  Trực: "),
                Span::styled(&truc.name, Style::default().fg(Color::Cyan)),
                Span::raw(" ("),
                Span::raw(&truc.quality),
                Span::raw(")"),
            ]));
            lines.push(Line::from(format!("  {}", truc.meaning.vi)));
            lines.push(Line::from(""));
        }

        if let Some(stars) = &insight.stars {
            if let Some(day_star) = &stars.day_star {
                let q = stars.day_star_quality.as_deref().unwrap_or("");
                lines.push(Line::from(vec![
                    Span::raw("  Sao ngày: "),
                    Span::styled(day_star.as_str(), Style::default().fg(Color::Yellow)),
                    Span::raw(format!(" ({q})")),
                ]));
                lines.push(Line::from(""));
            }

            let cat = stars.cat_tinh.join(", ");
            lines.push(Line::from(vec![
                Span::raw("  Cát tinh: "),
                Span::styled(
                    if stars.cat_tinh.is_empty() { "Không".to_string() } else { cat },
                    Style::default().fg(Color::Green),
                ),
            ]));
            let sat = stars.sat_tinh.join(", ");
            lines.push(Line::from(vec![
                Span::raw("  Sát tinh: "),
                Span::styled(
                    if stars.sat_tinh.is_empty() { "Không".to_string() } else { sat },
                    Style::default().fg(Color::Red),
                ),
            ]));
        }

        if let Some(deity) = &insight.day_deity {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::raw("  Thần sát: "),
                Span::styled(&deity.name, Style::default().fg(Color::Yellow)),
                Span::raw(format!(" ({})", deity.classification)),
            ]));
            if let Some(m) = &deity.deity_meaning {
                lines.push(Line::from(format!("   {}", m.vi)));
            }
        }

        Paragraph::new(lines).render(inner, buf);
    }
}
```

**Step 2: Create naam_panel.rs**

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

        if let Some(fortune) = &bundle.day_fortune {
            lines.push(Line::from(vec![
                Span::raw("  Nạp âm: "),
                Span::styled(&fortune.day_element.na_am, Style::default().fg(Color::Yellow)),
            ]));
            lines.push(Line::from(vec![
                Span::raw("  Ngũ hành: "),
                Span::styled(&fortune.day_element.element, Style::default().fg(Color::Cyan)),
            ]));
        }

        if let Some(insight) = &bundle.insight {
            if let Some(na_am) = &insight.na_am {
                lines.push(Line::from(vec![
                    Span::raw("  Hành: "),
                    Span::styled(&na_am.element, Style::default().fg(Color::Green)),
                ]));
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled("  Ý nghĩa:", Style::default().fg(Color::DarkGray))));
                lines.push(Line::from(format!("  {}", na_am.meaning.vi)));
            }
        }

        if let Some(canchi) = &bundle.canchi {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::raw("  Con giáp ngày: "),
                Span::styled(&canchi.day.con_giap, Style::default().fg(Color::Cyan)),
            ]));
            lines.push(Line::from(vec![
                Span::raw("  Con giáp tháng: "),
                Span::styled(&canchi.month.con_giap, Style::default().fg(Color::Cyan)),
            ]));
            lines.push(Line::from(vec![
                Span::raw("  Con giáp năm: "),
                Span::styled(&canchi.year.con_giap, Style::default().fg(Color::Cyan)),
            ]));
        }

        Paragraph::new(lines).render(inner, buf);
    }
}
```

**Step 3: Create direction_panel.rs**

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
        let Some(insight) = &bundle.insight else { return };
        let mut lines: Vec<Line<'_>> = vec![];

        if let Some(travel) = &insight.travel {
            lines.push(Line::from(vec![
                Span::raw("  Xuất hành: "),
                Span::styled(&travel.xuat_hanh_huong, Style::default().fg(Color::Green)),
            ]));
            lines.push(Line::from(vec![
                Span::raw("  Hỷ Thần: "),
                Span::styled(&travel.hy_than, Style::default().fg(Color::Green)),
            ]));
            lines.push(Line::from(vec![
                Span::raw("  Tài Thần: "),
                Span::styled(&travel.tai_than, Style::default().fg(Color::Yellow)),
            ]));
        }

        if let Some(deity) = &insight.day_deity {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::raw("  Thần sát: "),
                Span::styled(&deity.name, Style::default().fg(Color::Yellow)),
            ]));
            lines.push(Line::from(vec![
                Span::raw("  Phân loại: "),
                Span::styled(&deity.classification_meaning.vi, Style::default().fg(Color::Cyan)),
            ]));
            if let Some(meaning) = &deity.deity_meaning {
                lines.push(Line::from(format!("  {}", meaning.vi)));
            }
        }

        Paragraph::new(lines).render(inner, buf);
    }
}
```

**Step 4: Create guidance_panel.rs**

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
            lines.push(Line::from(Span::styled("  Nên làm:", Style::default().fg(Color::Green))));
            for item in &guidance.good_for.vi {
                lines.push(Line::from(format!("   \u{251C} {item}")));
            }
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled("  Tránh làm:", Style::default().fg(Color::Red))));
            for item in &guidance.avoid_for.vi {
                lines.push(Line::from(format!("   \u{251C} {item}")));
            }
        } else if let Some(truc) = &insight.truc {
            lines.push(Line::from(Span::styled("  Nên làm:", Style::default().fg(Color::Green))));
            for item in &truc.good_for.vi {
                lines.push(Line::from(format!("   \u{251C} {item}")));
            }
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled("  Tránh làm:", Style::default().fg(Color::Red))));
            for item in &truc.avoid_for.vi {
                lines.push(Line::from(format!("   \u{251C} {item}")));
            }
        }

        Paragraph::new(lines).render(inner, buf);
    }
}
```

**Step 5: Enrich scholarly.rs with CanChi insight**

In `crates/amlich-tui/src/widgets/scholarly.rs`, modify `render_evidence()` (line 34-119).

After the existing Can Chi ngày line (after line 53), insert:

```rust
// CanChi insight detail
if let Some(insight) = &bundle.insight {
    if let Some(ci) = &insight.canchi {
        lines.push(Line::from(vec![
            Span::raw("    \u{251C} Can: "),
            Span::styled(&ci.can.name, Style::default().fg(Color::Cyan)),
            Span::raw(" \u{2014} "),
            Span::raw(&ci.can.meaning.vi),
        ]));
        lines.push(Line::from(vec![
            Span::raw("    \u{2514} Chi: "),
            Span::styled(&ci.chi.name, Style::default().fg(Color::Cyan)),
            Span::raw(" \u{2014} "),
            Span::raw(&ci.chi.meaning.vi),
            Span::raw(format!(" ({})", ci.chi.animal.vi)),
        ]));
    }
}

// Month and year Can Chi
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
```

**Step 6: Rewrite insight.rs to 3x2 grid**

Replace `crates/amlich-tui/src/widgets/screens/insight.rs` entirely:

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
                ]).split(area);
                let r0 = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(rows[0]);
                let r1 = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(rows[1]);
                let r2 = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(rows[2]);

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
                ]).split(area);

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
```

**Step 7: Register new widget modules**

Add to `crates/amlich-tui/src/widgets/mod.rs`:

```rust
pub mod direction_panel;
pub mod guidance_panel;
pub mod naam_panel;
pub mod stars_panel;
```

**Step 8: Build and test**

Run: `cargo build -p amlich-tui && cargo test -p amlich-tui 2>&1 | tail -15`

**Step 9: Commit**

```bash
git add -A crates/amlich-tui/ && git commit -m "feat(amlich-tui): redesign Scholar to 3x2 grid with Stars, NaAm, Direction, Guidance panels"
```

---

### Task 6: Implement Hours Screen (Giờ Tốt)

**Files:**
- Modify: `crates/amlich-tui/src/widgets/screens/hours.rs` (replace stub)

**Step 1: Replace stub with full implementation**

Replace `crates/amlich-tui/src/widgets/screens/hours.rs`:

```rust
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
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
        let Some(bundle) = &self.app.bundle else {
            Paragraph::new("Chưa có dữ liệu.").render(area, buf);
            return;
        };
        let Some(gio) = &bundle.gio_hoang_dao else {
            Paragraph::new("Chưa có dữ liệu giờ hoàng đạo.").render(area, buf);
            return;
        };

        let rows = Layout::vertical([
            Constraint::Length(7),
            Constraint::Min(10),
        ]).split(area);

        // Top: Timeline overview
        {
            let block = Block::default()
                .title(format!(" Tổng Quan 12 Giờ — {} giờ tốt ", gio.good_hour_count))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray));
            let inner = block.inner(rows[0]);
            block.render(rows[0], buf);

            let mut chi_spans: Vec<Span<'_>> = vec![Span::raw(" ")];
            let mut marker_spans: Vec<Span<'_>> = vec![Span::raw(" ")];
            let mut star_spans: Vec<Span<'_>> = vec![Span::raw(" ")];

            let col_w = 10;
            for h in &gio.all_hours {
                let style = if h.is_good {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::DarkGray)
                };
                chi_spans.push(Span::styled(format!("{:^w$}", h.hour_chi, w = col_w), style));
                let m = if h.is_good { "\u{2605} Tốt" } else { "  Xấu" };
                marker_spans.push(Span::styled(
                    format!("{:^w$}", m, w = col_w),
                    if h.is_good {
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Red)
                    },
                ));
                star_spans.push(Span::styled(format!("{:^w$}", h.star, w = col_w), style));
            }

            Paragraph::new(vec![
                Line::from(chi_spans),
                Line::from(marker_spans),
                Line::from(star_spans),
            ]).render(inner, buf);
        }

        // Bottom: Detail columns
        match self.mode {
            LayoutMode::Large | LayoutMode::Medium => {
                let cols = Layout::horizontal([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ]).split(rows[1]);
                render_hour_list(&gio.all_hours, true, cols[0], buf);
                render_hour_list(&gio.all_hours, false, cols[1], buf);
            }
            LayoutMode::Small => {
                let detail = Layout::vertical([
                    Constraint::Percentage(60),
                    Constraint::Percentage(40),
                ]).split(rows[1]);
                render_hour_list(&gio.all_hours, true, detail[0], buf);
                render_hour_list(&gio.all_hours, false, detail[1], buf);
            }
        }
    }
}

fn render_hour_list(
    all_hours: &[amlich_api::v2::HourInfoDto],
    show_good: bool,
    area: Rect,
    buf: &mut Buffer,
) {
    let title = if show_good { " \u{2605} Giờ Hoàng Đạo " } else { " Giờ Hắc Đạo " };
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let mut lines: Vec<Line<'_>> = vec![];
    let filtered: Vec<_> = all_hours.iter().filter(|h| h.is_good == show_good).collect();

    for h in &filtered {
        let (marker, color) = if show_good {
            ("\u{2605}", Color::Green)
        } else {
            ("\u{00B7}", Color::Red)
        };
        lines.push(Line::from(vec![
            Span::styled(format!("  {marker} "), Style::default().fg(color)),
            Span::styled(
                format!("{:<6}", h.hour_chi),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ),
            Span::styled(format!("({}) ", h.time_range), Style::default().fg(Color::DarkGray)),
            Span::raw(format!("\u{2014} {}", h.star)),
        ]));
    }

    if show_good {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            format!("  Tổng: {}/{} giờ tốt", filtered.len(), all_hours.len()),
            Style::default().fg(Color::DarkGray),
        )));
    }

    Paragraph::new(lines).render(inner, buf);
}
```

**Step 2: Build and test**

Run: `cargo build -p amlich-tui && cargo test -p amlich-tui 2>&1 | tail -10`

**Step 3: Commit**

```bash
git add crates/amlich-tui/src/widgets/screens/hours.rs && git commit -m "feat(amlich-tui): implement Hours screen with timeline and detail panels"
```

---

### Task 7: Implement Elements Screen (Ngũ Hành)

**Files:**
- Modify: `crates/amlich-tui/src/widgets/screens/elements.rs` (replace stub)

**Step 1: Replace stub with full implementation**

Replace `crates/amlich-tui/src/widgets/screens/elements.rs`:

```rust
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

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
        let Some(bundle) = &self.app.bundle else {
            Paragraph::new("Chưa có dữ liệu.").render(area, buf);
            return;
        };

        match self.mode {
            LayoutMode::Large => {
                let rows = Layout::vertical([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ]).split(area);
                let top = Layout::horizontal([
                    Constraint::Percentage(34), Constraint::Percentage(33), Constraint::Percentage(33),
                ]).split(rows[0]);
                let bottom = Layout::horizontal([
                    Constraint::Percentage(34), Constraint::Percentage(33), Constraint::Percentage(33),
                ]).split(rows[1]);

                render_tang_can(bundle, top[0], buf);
                render_ten_gods(bundle, top[1], buf);
                render_xung_hop(bundle, top[2], buf);
                render_element_relations(bundle, bottom[0], buf);
                render_pillars(bundle, bottom[1], buf);
                render_element_chart(bundle, bottom[2], buf);
            }
            LayoutMode::Medium => {
                let rows = Layout::vertical([
                    Constraint::Percentage(34), Constraint::Percentage(33), Constraint::Percentage(33),
                ]).split(area);
                let r0 = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(rows[0]);
                let r1 = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(rows[1]);
                let r2 = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(rows[2]);

                render_tang_can(bundle, r0[0], buf);
                render_ten_gods(bundle, r0[1], buf);
                render_xung_hop(bundle, r1[0], buf);
                render_element_relations(bundle, r1[1], buf);
                render_pillars(bundle, r2[0], buf);
                render_element_chart(bundle, r2[1], buf);
            }
            LayoutMode::Small => {
                let rows = Layout::vertical([
                    Constraint::Min(8), Constraint::Min(10), Constraint::Min(8),
                    Constraint::Min(8), Constraint::Min(8), Constraint::Min(8),
                ]).split(area);
                render_tang_can(bundle, rows[0], buf);
                render_ten_gods(bundle, rows[1], buf);
                render_xung_hop(bundle, rows[2], buf);
                render_element_relations(bundle, rows[3], buf);
                render_pillars(bundle, rows[4], buf);
                render_element_chart(bundle, rows[5], buf);
            }
        }
    }
}

fn render_tang_can(bundle: &amlich_api::v2::DayBundleDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default().title(" Tàng Can ").borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(insight) = &bundle.insight else { return };
    let Some(tc) = &insight.tang_can else {
        Paragraph::new("  Chưa có dữ liệu").render(inner, buf);
        return;
    };
    let mut lines: Vec<Line<'_>> = vec![];

    if let Some(canchi) = &bundle.canchi {
        lines.push(Line::from(vec![
            Span::raw("  Chi ngày: "),
            Span::styled(&canchi.day.chi, Style::default().fg(Color::Cyan)),
        ]));
        lines.push(Line::from(""));
    }

    let labels = ["Chính", "Trung", "Dư"];
    let values = [&tc.main, &tc.central, &tc.residual];
    for (i, (label, value)) in labels.iter().zip(values.iter()).enumerate() {
        let s = tc.strength[i];
        let bar_len = (s as usize * 10) / 100;
        let bar = "\u{2588}".repeat(bar_len) + &"\u{2591}".repeat(10 - bar_len);
        lines.push(Line::from(vec![
            Span::raw(format!("  {label}: ")),
            Span::styled(
                format!("{value}"),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!(" {bar} {s}%")),
        ]));
    }

    Paragraph::new(lines).render(inner, buf);
}

fn render_ten_gods(bundle: &amlich_api::v2::DayBundleDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default().title(" Thập Thần ").borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(insight) = &bundle.insight else { return };
    let Some(tg) = &insight.ten_gods else {
        Paragraph::new("  Chưa có dữ liệu").render(inner, buf);
        return;
    };
    let mut lines: Vec<Line<'_>> = vec![];

    if let Some(e) = &tg.to_year_stem {
        lines.push(Line::from(Span::styled("  Với năm sinh:", Style::default().fg(Color::DarkGray))));
        lines.push(Line::from(vec![
            Span::raw("   "),
            Span::styled(&e.label, Style::default().fg(Color::Yellow)),
            Span::raw(format!(": {}", e.name.vi)),
        ]));
        lines.push(Line::from(format!("   Nghĩa: {}", e.meaning.vi)));
        lines.push(Line::from(format!("   Quan hệ: {} ({})",
            e.relation,
            if e.same_polarity { "đồng cực" } else { "khác cực" },
        )));
        lines.push(Line::from(""));
    }

    if let Some(e) = &tg.to_self {
        lines.push(Line::from(Span::styled("  Với bản thân:", Style::default().fg(Color::DarkGray))));
        lines.push(Line::from(vec![
            Span::raw("   "),
            Span::styled(&e.label, Style::default().fg(Color::Yellow)),
            Span::raw(format!(": {}", e.name.vi)),
        ]));
        lines.push(Line::from(format!("   Nghĩa: {}", e.meaning.vi)));
        lines.push(Line::from(format!("   Quan hệ: {} ({})",
            e.relation,
            if e.same_polarity { "đồng cực" } else { "khác cực" },
        )));
    }

    Paragraph::new(lines).render(inner, buf);
}

fn render_xung_hop(bundle: &amlich_api::v2::DayBundleDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default().title(" Xung Hợp ").borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(insight) = &bundle.insight else { return };
    let Some(xh) = &insight.xung_hop else {
        Paragraph::new("  Chưa có dữ liệu").render(inner, buf);
        return;
    };
    let mut lines: Vec<Line<'_>> = vec![];

    lines.push(Line::from(vec![
        Span::raw("  Lục xung: "),
        Span::styled(&xh.luc_xung, Style::default().fg(Color::Red)),
    ]));
    if !xh.tam_hop.is_empty() {
        lines.push(Line::from(vec![
            Span::raw("  Tam hợp: "),
            Span::styled(xh.tam_hop.join(" \u{2014} "), Style::default().fg(Color::Green)),
        ]));
    }
    if let Some(lh) = &xh.liu_he {
        lines.push(Line::from(vec![
            Span::raw("  Lục hợp: "),
            Span::styled(lh.as_str(), Style::default().fg(Color::Green)),
        ]));
    }
    if let Some(xhai) = &xh.xiang_hai {
        lines.push(Line::from(vec![
            Span::raw("  Tương hại: "),
            Span::styled(xhai.as_str(), Style::default().fg(Color::Red)),
        ]));
    }

    Paragraph::new(lines).render(inner, buf);
}

fn render_element_relations(bundle: &amlich_api::v2::DayBundleDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default().title(" Ngũ Hành Tương Quan ").borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(canchi) = &bundle.canchi else { return };
    let mut lines: Vec<Line<'_>> = vec![];

    let can_e = &canchi.day.ngu_hanh.can;
    let chi_e = &canchi.day.ngu_hanh.chi;

    lines.push(Line::from(vec![
        Span::raw("  Can ngày: "),
        Span::styled(format!("{} ({})", canchi.day.can, can_e), Style::default().fg(Color::Cyan)),
    ]));
    lines.push(Line::from(vec![
        Span::raw("  Chi ngày: "),
        Span::styled(format!("{} ({})", canchi.day.chi, chi_e), Style::default().fg(Color::Cyan)),
    ]));

    let rel = element_relation(can_e, chi_e);
    let rel_color = if rel.contains("sinh") { Color::Green } else if rel.contains("khắc") { Color::Red } else { Color::Yellow };
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::raw("  Quan hệ: "),
        Span::styled(format!("{can_e} {rel} {chi_e}"), Style::default().fg(rel_color)),
    ]));

    Paragraph::new(lines).render(inner, buf);
}

fn render_pillars(bundle: &amlich_api::v2::DayBundleDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default().title(" Can Chi 3 Trụ ").borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(canchi) = &bundle.canchi else { return };
    let mut lines: Vec<Line<'_>> = vec![];

    lines.push(Line::from(vec![
        Span::raw("            "),
        Span::styled("Can    Chi    Hành", Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD)),
    ]));
    for (label, p) in [("Năm:  ", &canchi.year), ("Tháng:", &canchi.month), ("Ngày: ", &canchi.day)] {
        lines.push(Line::from(vec![
            Span::raw(format!("  {label} ")),
            Span::styled(format!("{:<6}", p.can), Style::default().fg(Color::Cyan)),
            Span::styled(format!("{:<6}", p.chi), Style::default().fg(Color::Cyan)),
            Span::styled(format!("{}/{}", p.ngu_hanh.can, p.ngu_hanh.chi), Style::default().fg(Color::Yellow)),
        ]));
    }

    if let Some(fortune) = &bundle.day_fortune {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::raw("  Nạp âm: "),
            Span::styled(&fortune.day_element.na_am, Style::default().fg(Color::Yellow)),
        ]));
    }

    Paragraph::new(lines).render(inner, buf);
}

fn render_element_chart(bundle: &amlich_api::v2::DayBundleDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default().title(" Ngũ Hành Tổng Hợp ").borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(canchi) = &bundle.canchi else { return };
    let mut lines: Vec<Line<'_>> = vec![];

    let elements = [
        &canchi.year.ngu_hanh.can, &canchi.year.ngu_hanh.chi,
        &canchi.month.ngu_hanh.can, &canchi.month.ngu_hanh.chi,
        &canchi.day.ngu_hanh.can, &canchi.day.ngu_hanh.chi,
    ];

    let names = ["Kim", "Mộc", "Thủy", "Hỏa", "Thổ"];
    let colors = [Color::White, Color::Green, Color::Blue, Color::Red, Color::Yellow];
    let mut dominant = ("", 0usize);

    for (i, name) in names.iter().enumerate() {
        let count = elements.iter().filter(|e| e.as_str() == *name).count();
        if count > dominant.1 { dominant = (name, count); }
        let bar = "\u{2588}".repeat(count * 3) + &"\u{2591}".repeat(18usize.saturating_sub(count * 3));
        lines.push(Line::from(vec![
            Span::styled(format!("  {name:<4} "), Style::default().fg(colors[i]).add_modifier(Modifier::BOLD)),
            Span::raw(format!("{bar} {count}/6")),
        ]));
    }

    if !dominant.0.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::raw("  Hành vượng: "),
            Span::styled(dominant.0, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]));
    }

    Paragraph::new(lines).render(inner, buf);
}

fn element_relation(a: &str, b: &str) -> &'static str {
    match (a, b) {
        ("Kim", "Thủy") | ("Thủy", "Mộc") | ("Mộc", "Hỏa") | ("Hỏa", "Thổ") | ("Thổ", "Kim") => "sinh",
        ("Thủy", "Kim") | ("Mộc", "Thủy") | ("Hỏa", "Mộc") | ("Thổ", "Hỏa") | ("Kim", "Thổ") => "được sinh",
        ("Kim", "Mộc") | ("Mộc", "Thổ") | ("Thổ", "Thủy") | ("Thủy", "Hỏa") | ("Hỏa", "Kim") => "khắc",
        ("Mộc", "Kim") | ("Thổ", "Mộc") | ("Thủy", "Thổ") | ("Hỏa", "Thủy") | ("Kim", "Hỏa") => "bị khắc",
        _ if a == b => "tỷ hòa",
        _ => "\u{2014}",
    }
}
```

**Step 2: Build and test**

Run: `cargo build -p amlich-tui && cargo test -p amlich-tui 2>&1 | tail -10`

**Step 3: Commit**

```bash
git add crates/amlich-tui/src/widgets/screens/elements.rs && git commit -m "feat(amlich-tui): implement Elements screen with Tang Can, Ten Gods, Xung Hop, Pillars"
```

---

### Task 8: Implement FengShui Screen (Phong Thủy)

**Files:**
- Modify: `crates/amlich-tui/src/widgets/screens/feng_shui.rs` (replace stub)

**Step 1: Replace stub with full implementation**

Replace `crates/amlich-tui/src/widgets/screens/feng_shui.rs`:

```rust
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

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
        let Some(bundle) = &self.app.bundle else {
            Paragraph::new("Chưa có dữ liệu.").render(area, buf);
            return;
        };
        let Some(insight) = &bundle.insight else {
            Paragraph::new("Chưa có dữ liệu insight.").render(area, buf);
            return;
        };

        if insight.tu_menh.is_none() && insight.dai_van.is_none() {
            let block = Block::default().title(" Phong Thủy ").borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray));
            let text = "Chưa cấu hình hồ sơ cá nhân.\n\nCần birth year + gender để tính Tứ Mệnh và Đại Vận.";
            Paragraph::new(text).block(block).render(area, buf);
            return;
        }

        match self.mode {
            LayoutMode::Large | LayoutMode::Medium => {
                let rows = Layout::vertical([
                    Constraint::Percentage(50), Constraint::Percentage(50),
                ]).split(area);
                let top = Layout::horizontal([
                    Constraint::Percentage(50), Constraint::Percentage(50),
                ]).split(rows[0]);
                let bottom = Layout::horizontal([
                    Constraint::Percentage(50), Constraint::Percentage(50),
                ]).split(rows[1]);

                render_kua(insight, top[0], buf);
                render_directions(insight, top[1], buf);
                render_dai_van(insight, bottom[0], buf);
                render_compass(insight, bottom[1], buf);
            }
            LayoutMode::Small => {
                let rows = Layout::vertical([
                    Constraint::Min(9), Constraint::Min(10), Constraint::Min(14),
                ]).split(area);
                render_kua(insight, rows[0], buf);
                render_directions(insight, rows[1], buf);
                render_dai_van(insight, rows[2], buf);
            }
        }
    }
}

fn render_kua(insight: &amlich_api::DayInsightDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default().title(" Tứ Mệnh / Kua ").borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(tm) = &insight.tu_menh else { return };
    let lines = vec![
        Line::from(vec![
            Span::raw("  Quẻ số: "),
            Span::styled(tm.kua.to_string(), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::raw("  Quẻ: "),
            Span::styled(&tm.trigram.vi, Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::raw("  Nhóm: "),
            Span::styled(&tm.group, Style::default().fg(Color::Green)),
        ]),
        Line::from(format!("   \u{2514} {}", tm.group_meaning.vi)),
        Line::from(""),
        Line::from(vec![
            Span::raw("  Hướng mệnh: "),
            Span::styled(&tm.direction.vi, Style::default().fg(Color::Yellow)),
        ]),
        Line::from(format!("   \u{2514} {}", tm.meaning.vi)),
    ];
    Paragraph::new(lines).render(inner, buf);
}

fn render_directions(insight: &amlich_api::DayInsightDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default().title(" Hướng Tốt / Xấu ").borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(tm) = &insight.tu_menh else { return };
    let mut lines: Vec<Line<'_>> = vec![];

    lines.push(Line::from(Span::styled("  Hướng tốt:", Style::default().fg(Color::Green))));
    for d in &tm.favorable_directions {
        lines.push(Line::from(vec![
            Span::styled("   \u{2605} ", Style::default().fg(Color::Green)),
            Span::raw(d.as_str()),
        ]));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("  Hướng xấu:", Style::default().fg(Color::Red))));
    for d in &tm.unfavorable_directions {
        lines.push(Line::from(vec![
            Span::styled("   \u{2716} ", Style::default().fg(Color::Red)),
            Span::raw(d.as_str()),
        ]));
    }

    Paragraph::new(lines).render(inner, buf);
}

fn render_dai_van(insight: &amlich_api::DayInsightDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default().title(" Đại Vận ").borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(dv) = &insight.dai_van else { return };
    let mut lines: Vec<Line<'_>> = vec![];

    lines.push(Line::from(vec![
        Span::raw("  Hướng vận: "),
        Span::styled(&dv.direction, Style::default().fg(Color::Yellow)),
    ]));
    lines.push(Line::from(format!("   \u{2514} {}", dv.direction_meaning.vi)));

    if let Some(cur) = &dv.current_pillar {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled(format!("  \u{25B6} {} ", cur.can_chi),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(format!("({}-{} tuổi) ", cur.start_age as u32, cur.end_age as u32)),
            Span::styled(&cur.element, Style::default().fg(Color::Yellow)),
        ]));
        lines.push(Line::from(format!("    {}", cur.element_meaning.vi)));
    }

    lines.push(Line::from(""));
    for p in &dv.all_pillars {
        let is_cur = dv.current_pillar.as_ref().map(|c| c.index == p.index).unwrap_or(false);
        let marker = if is_cur { "\u{25C4}" } else { " " };
        let style = if is_cur {
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        lines.push(Line::from(Span::styled(
            format!("  {}. {:<10} ({:>2}-{:>2}) {:>4} {marker}", p.index, p.can_chi, p.start_age as u32, p.end_age as u32, p.element),
            style,
        )));
    }

    Paragraph::new(lines).render(inner, buf);
}

fn render_compass(insight: &amlich_api::DayInsightDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default().title(" La Bàn Hướng ").borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(tm) = &insight.tu_menh else { return };
    let good: Vec<&str> = tm.favorable_directions.iter().map(|s| s.as_str()).collect();
    let bad: Vec<&str> = tm.unfavorable_directions.iter().map(|s| s.as_str()).collect();

    let ds = |name: &str| -> Style {
        if good.iter().any(|d| d.contains(name)) {
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
        } else if bad.iter().any(|d| d.contains(name)) {
            Style::default().fg(Color::Red)
        } else {
            Style::default().fg(Color::DarkGray)
        }
    };
    let mk = |name: &str| -> &str {
        if good.iter().any(|d| d.contains(name)) { "\u{2605}" }
        else if bad.iter().any(|d| d.contains(name)) { "\u{2716}" }
        else { "\u{00B7}" }
    };

    let lines = vec![
        Line::from(""),
        Line::from(vec![Span::raw("          "), Span::styled(format!("{} Bắc", mk("Bắc")), ds("Bắc"))]),
        Line::from(vec![
            Span::raw("     "), Span::styled(format!("{} TB", mk("Tây Bắc")), ds("Tây Bắc")),
            Span::raw("    |    "), Span::styled(format!("ĐB {}", mk("Đông Bắc")), ds("Đông Bắc")),
        ]),
        Line::from("             |"),
        Line::from(vec![
            Span::raw("    "), Span::styled(format!("{} Tây", mk("Tây")), ds("Tây")),
            Span::raw(" \u{2014}\u{2014}\u{25CF}\u{2014}\u{2014} "),
            Span::styled(format!("Đông {}", mk("Đông")), ds("Đông")),
        ]),
        Line::from("             |"),
        Line::from(vec![
            Span::raw("     "), Span::styled(format!("{} TN", mk("Tây Nam")), ds("Tây Nam")),
            Span::raw("    |    "), Span::styled(format!("ĐN {}", mk("Đông Nam")), ds("Đông Nam")),
        ]),
        Line::from(vec![Span::raw("          "), Span::styled(format!("{} Nam", mk("Nam")), ds("Nam"))]),
        Line::from(""),
        Line::from(vec![
            Span::styled(" \u{2605} Tốt ", Style::default().fg(Color::Green)),
            Span::styled(" \u{2716} Xấu", Style::default().fg(Color::Red)),
        ]),
    ];
    Paragraph::new(lines).render(inner, buf);
}
```

**Step 2: Build and test**

Run: `cargo build -p amlich-tui && cargo test -p amlich-tui 2>&1 | tail -10`

**Step 3: Commit**

```bash
git add crates/amlich-tui/src/widgets/screens/feng_shui.rs && git commit -m "feat(amlich-tui): implement FengShui screen with Kua, directions, Dai Van, compass"
```

---

### Task 9: Implement SolarTerms Screen (Tiết Khí)

**Files:**
- Modify: `crates/amlich-tui/src/widgets/screens/solar_terms.rs` (replace stub)

**Step 1: Replace stub with full implementation**

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
        let Some(bundle) = &self.app.bundle else {
            Paragraph::new("Chưa có dữ liệu.").render(area, buf);
            return;
        };

        match self.mode {
            LayoutMode::Large | LayoutMode::Medium => {
                let rows = Layout::vertical([
                    Constraint::Percentage(50), Constraint::Percentage(50),
                ]).split(area);
                let top = Layout::horizontal([
                    Constraint::Percentage(50), Constraint::Percentage(50),
                ]).split(rows[0]);
                let bottom = Layout::horizontal([
                    Constraint::Percentage(50), Constraint::Percentage(50),
                ]).split(rows[1]);

                render_current(bundle, top[0], buf);
                render_astronomy(bundle, top[1], buf);
                render_agriculture(bundle, bottom[0], buf);
                render_health(bundle, bottom[1], buf);
            }
            LayoutMode::Small => {
                let rows = Layout::vertical([
                    Constraint::Min(10), Constraint::Min(8),
                    Constraint::Min(8), Constraint::Min(8),
                ]).split(area);
                render_current(bundle, rows[0], buf);
                render_astronomy(bundle, rows[1], buf);
                render_agriculture(bundle, rows[2], buf);
                render_health(bundle, rows[3], buf);
            }
        }
    }
}

fn render_current(bundle: &amlich_api::v2::DayBundleDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default().title(" Tiết Khí Hiện Tại ").borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let mut lines: Vec<Line<'_>> = vec![];

    if let Some(tk) = &bundle.tiet_khi {
        lines.push(Line::from(vec![
            Span::raw("  Tiết khí: "),
            Span::styled(&tk.name, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]));
        lines.push(Line::from(vec![
            Span::raw("  Kinh độ: "),
            Span::styled(format!("{}°", tk.longitude), Style::default().fg(Color::Yellow)),
        ]));
        lines.push(Line::from(vec![
            Span::raw("  Mùa: "),
            Span::styled(&tk.season, Style::default().fg(Color::Green)),
        ]));
        lines.push(Line::from(""));
        lines.push(Line::from(format!("  {}", tk.description)));
    }

    if let Some(insight) = &bundle.insight {
        if let Some(tki) = &insight.tiet_khi {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled("  Ý nghĩa:", Style::default().fg(Color::DarkGray))));
            lines.push(Line::from(format!("  {}", tki.meaning.vi)));
        }
    }

    Paragraph::new(lines).render(inner, buf);
}

fn render_astronomy(bundle: &amlich_api::v2::DayBundleDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default().title(" Thiên Văn & Thời Tiết ").borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(insight) = &bundle.insight else { return };
    let Some(tki) = &insight.tiet_khi else { return };
    let mut lines: Vec<Line<'_>> = vec![];

    lines.push(Line::from(Span::styled("  Thiên văn:", Style::default().fg(Color::DarkGray))));
    lines.push(Line::from(format!("  {}", tki.astronomy.vi)));

    if let Some(tk) = &bundle.tiet_khi {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::raw("  Kinh độ hiện tại: "),
            Span::styled(format!("{:.1}°", tk.current_longitude), Style::default().fg(Color::Yellow)),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("  Thời tiết:", Style::default().fg(Color::DarkGray))));
    lines.push(Line::from(format!("  {}", tki.weather.vi)));

    Paragraph::new(lines).render(inner, buf);
}

fn render_agriculture(bundle: &amlich_api::v2::DayBundleDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default().title(" Nông Nghiệp ").borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(insight) = &bundle.insight else { return };
    let Some(tki) = &insight.tiet_khi else { return };
    let mut lines: Vec<Line<'_>> = vec![];

    lines.push(Line::from(Span::styled("  Hoạt động nông vụ:", Style::default().fg(Color::Green))));
    for item in &tki.agriculture.vi {
        lines.push(Line::from(format!("   \u{251C} {item}")));
    }

    Paragraph::new(lines).render(inner, buf);
}

fn render_health(bundle: &amlich_api::v2::DayBundleDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default().title(" Sức Khỏe ").borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(insight) = &bundle.insight else { return };
    let Some(tki) = &insight.tiet_khi else { return };
    let mut lines: Vec<Line<'_>> = vec![];

    lines.push(Line::from(Span::styled("  Lời khuyên sức khỏe:", Style::default().fg(Color::Cyan))));
    for item in &tki.health.vi {
        lines.push(Line::from(format!("   \u{251C} {item}")));
    }

    Paragraph::new(lines).render(inner, buf);
}
```

**Step 2: Build and test**

Run: `cargo build -p amlich-tui && cargo test -p amlich-tui 2>&1 | tail -10`

**Step 3: Commit**

```bash
git add crates/amlich-tui/src/widgets/screens/solar_terms.rs && git commit -m "feat(amlich-tui): implement SolarTerms screen with astronomy, agriculture, health panels"
```

---

### Task 10: Final Build, Test, and Polish

**Step 1: Full build**

Run: `cargo build -p amlich-tui`

**Step 2: Run all tests**

Run: `cargo test -p amlich-tui`

Fix any failing tests.

**Step 3: Run clippy**

Run: `cargo clippy -p amlich-tui -- -W clippy::all 2>&1 | head -40`

Fix any warnings.

**Step 4: Manual smoke test**

Run: `cargo run -p amlich-tui`

Verify:
- Tab/Shift+Tab cycles through all tabs (Dashboard, Scholar, Giờ Tốt, Ngũ Hành, Phong Thủy, Tiết Khí, Planning, Calendar)
- Number keys 1-8 jump to correct tabs
- Scholar tab shows 3x2 grid (Large) or 2x3 (Medium) or stacked (Small)
- Each new tab renders its panels with data from bundle
- No panics on missing data (all fields are Option)
- Ribbon shows abbreviated labels on medium terminal
- Ribbon shows only active tab on small terminal
- Resize terminal to verify responsive layouts

**Step 5: Commit any fixes**

```bash
git add -A crates/amlich-tui/ && git commit -m "chore(amlich-tui): fix clippy warnings and test failures for scholar expansion"
```
