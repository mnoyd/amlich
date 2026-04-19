# TUI Redesign: Info-First Dashboard — Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Reorganize the TUI from calendar-dominant to info-first layout, surface day guidance into the default view, rethink visual style, and add `c` key to toggle calendar on small screens.

**Architecture:** Replace the current 3-column layout (calendar | detail | hours) with a 2-zone layout (calendar sidebar | info panel). Create a new `InfoPanel` widget that combines date headline, Can Chi, day guidance summary, compact hours, and Tiet Khi into one scrollable view. The calendar widget stays mostly unchanged but shrinks to sidebar width. The hours widget is removed as a standalone column.

**Tech Stack:** Rust, Ratatui 0.30, Crossterm 0.29. All changes in `crates/amlich/src/`.

---

### Task 1: Update Theme Colors

**Files:**
- Modify: `crates/amlich/src/theme.rs` (all lines)

**Step 1: Replace theme.rs with the new color palette**

Replace the entire contents of `theme.rs` with:

```rust
use ratatui::style::{Color, Modifier, Style};

// Terminal-native: use Color::Reset for default fg/bg (respects user theme)

// Text hierarchy
pub const PRIMARY_FG: Color = Color::Reset;       // default terminal fg
pub const SECONDARY_FG: Color = Color::DarkGray;  // dimmed text, labels
pub const ACCENT_FG: Color = Color::Rgb(212, 168, 85); // warm amber — section headers, highlights

// Calendar
pub const SOLAR_FG: Color = Color::Reset;
pub const LUNAR_FG: Color = Color::DarkGray;
pub const TODAY_FG: Color = Color::Black;          // inverted
pub const TODAY_BG: Color = Color::White;          // inverted
pub const SELECTED_FG: Color = Color::Reset;       // bold + underline, no bg change
pub const WEEKEND_FG: Color = Color::Rgb(224, 112, 112); // soft coral
pub const HOLIDAY_FG: Color = Color::Rgb(212, 168, 85);  // amber (same as accent)

// Day guidance
pub const GOOD_FG: Color = Color::Rgb(109, 191, 139);  // soft green
pub const BAD_FG: Color = Color::Rgb(224, 112, 112);   // soft coral

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
```

**Step 2: Build to check for compilation errors**

Run: `cargo build -p amlich 2>&1 | head -30`
Expected: Compilation errors because old color names (HEADER_BG, HEADER_FG, VALUE_FG, LABEL_FG, TITLE_FG, SELECTED_BG, SELECTED_FG with old meaning, GOOD_HOUR_FG as old name) are referenced in other files. **Do NOT fix these yet** — they'll be fixed in subsequent tasks as we rewrite each widget.

**Step 3: Add backward-compatible aliases temporarily**

Add these aliases at the bottom of theme.rs so existing code compiles while we migrate:

```rust
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
```

**Step 4: Build to verify compilation**

Run: `cargo build -p amlich`
Expected: Compiles with no errors (warnings OK).

**Step 5: Commit**

```bash
git add crates/amlich/src/theme.rs
git commit -m "refactor(tui): update color palette to terminal-native with amber/green/coral accents"
```

---

### Task 2: Create InfoPanel Widget

This is the new hero widget that replaces detail + hours panels.

**Files:**
- Create: `crates/amlich/src/widgets/info_panel.rs`
- Modify: `crates/amlich/src/widgets/mod.rs`

**Step 1: Add module declaration**

In `crates/amlich/src/widgets/mod.rs`, add:
```rust
pub mod info_panel;
```

**Step 2: Create info_panel.rs**

Create `crates/amlich/src/widgets/info_panel.rs` with this content:

```rust
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget, Wrap},
};

use crate::app::App;
use crate::theme;

pub struct InfoPanel<'a> {
    app: &'a App,
}

impl<'a> InfoPanel<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }

    /// Build section divider: "── Section Name ──────..."
    fn section_line(name: &str, width: u16) -> Line<'static> {
        let prefix = format!("── {} ", name);
        let remaining = (width as usize).saturating_sub(prefix.chars().count());
        let line_str = format!("{}{}", prefix, "─".repeat(remaining));
        Line::from(Span::styled(line_str, theme::section_style()))
    }

    fn build_lines(&self, width: u16) -> Vec<Line<'_>> {
        let mut lines = Vec::new();

        let Some(info) = self.app.selected_info() else {
            lines.push(Line::from("Không có dữ liệu"));
            return lines;
        };

        // ── Tier 1: Date headline ──
        // Solar date + day of week
        lines.push(Line::from(vec![
            Span::styled(
                format!("{}", info.solar.date_string),
                Style::default()
                    .fg(theme::PRIMARY_FG)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("  {}", info.solar.day_of_week_name),
                Style::default().fg(theme::SECONDARY_FG),
            ),
        ]));

        // Lunar date (with moon emoji)
        lines.push(Line::from(vec![
            Span::raw("🌙 "),
            Span::styled(
                &info.lunar.date_string,
                Style::default()
                    .fg(theme::ACCENT_FG)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));

        lines.push(Line::from(""));

        // ── Tier 2: Can Chi ──
        lines.push(Self::section_line("Can Chi", width));

        // Day / Month / Year Can Chi with elements inline
        lines.push(Line::from(vec![
            Span::styled(
                &info.canchi.day.full,
                Style::default().fg(theme::PRIMARY_FG),
            ),
            Span::styled(
                format!("  {} · {}", info.canchi.day.ngu_hanh.can, info.canchi.day.ngu_hanh.chi),
                Style::default().fg(theme::SECONDARY_FG),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled(
                &info.canchi.month.full,
                Style::default().fg(theme::PRIMARY_FG),
            ),
            Span::styled(
                format!("  {} · {}", info.canchi.month.ngu_hanh.can, info.canchi.month.ngu_hanh.chi),
                Style::default().fg(theme::SECONDARY_FG),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled(
                format!("{} ({})", info.canchi.year.full, info.canchi.year.con_giap),
                Style::default().fg(theme::PRIMARY_FG),
            ),
            Span::styled(
                format!("  {} · {}", info.canchi.year.ngu_hanh.can, info.canchi.year.ngu_hanh.chi),
                Style::default().fg(theme::SECONDARY_FG),
            ),
        ]));

        lines.push(Line::from(""));

        // ── Tier 2: Nên / Tránh (Day Guidance summary) ──
        lines.push(Self::section_line("Nên / Tránh", width));

        // Fetch insight data for guidance
        if let Some(insight) = self.app.selected_insight() {
            if let Some(guidance) = &insight.day_guidance {
                // Show top 3-4 good activities
                for item in guidance.good_for.vi.iter().take(4) {
                    lines.push(Line::from(vec![
                        Span::styled("✓ ", Style::default().fg(theme::GOOD_FG)),
                        Span::styled(
                            item.as_str(),
                            Style::default().fg(theme::PRIMARY_FG),
                        ),
                    ]));
                }
                // Show top 3-4 avoid activities
                for item in guidance.avoid_for.vi.iter().take(4) {
                    lines.push(Line::from(vec![
                        Span::styled("✗ ", Style::default().fg(theme::BAD_FG)),
                        Span::styled(
                            item.as_str(),
                            Style::default().fg(theme::PRIMARY_FG),
                        ),
                    ]));
                }
                // Show [i] hint if there's more
                let total = guidance.good_for.vi.len() + guidance.avoid_for.vi.len();
                if total > 8 {
                    lines.push(Line::from(Span::styled(
                        "                           [i] xem thêm",
                        Style::default().fg(theme::SECONDARY_FG),
                    )));
                }
            } else {
                lines.push(Line::from(Span::styled(
                    "Không có thông tin",
                    Style::default().fg(theme::SECONDARY_FG),
                )));
            }
        } else {
            lines.push(Line::from(Span::styled(
                "Không có thông tin",
                Style::default().fg(theme::SECONDARY_FG),
            )));
        }

        lines.push(Line::from(""));

        // ── Tier 2: Giờ tốt (compact one-liner) ──
        let ghd = &info.gio_hoang_dao;
        let good_names: Vec<&str> = ghd
            .good_hours
            .iter()
            .map(|h| h.hour_chi.as_str())
            .collect();
        let hours_str = good_names.join(" · ");

        lines.push(Line::from(vec![
            Span::styled("── Giờ tốt ", theme::section_style()),
            Span::styled(
                hours_str,
                Style::default().fg(theme::GOOD_HOUR_FG),
            ),
        ]));

        lines.push(Line::from(""));

        // ── Tier 2: Tiết khí ──
        lines.push(Line::from(vec![
            Span::styled("── Tiết khí ", theme::section_style()),
            Span::styled(
                format!("{} · {}", info.tiet_khi.name, info.tiet_khi.season),
                Style::default().fg(theme::PRIMARY_FG),
            ),
        ]));

        if !info.tiet_khi.description.is_empty() {
            lines.push(Line::from(Span::styled(
                format!("   {}", info.tiet_khi.description),
                Style::default().fg(theme::SECONDARY_FG),
            )));
        }

        // ── Holiday (if applicable) ──
        if let Some(holiday) = self.app.holiday_for_day(self.app.selected_day) {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled("── Ngày lễ ", theme::section_style()),
                Span::styled(
                    &holiday.name,
                    Style::default()
                        .fg(theme::HOLIDAY_FG)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
            if !holiday.description.is_empty() {
                lines.push(Line::from(Span::styled(
                    format!("   {}", holiday.description),
                    Style::default().fg(theme::SECONDARY_FG),
                )));
            }
        }

        lines
    }
}

impl Widget for InfoPanel<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let lines = self.build_lines(area.width);
        let p = Paragraph::new(lines).wrap(Wrap { trim: false });
        p.render(area, buf);
    }
}
```

**Step 3: Build to verify**

Run: `cargo build -p amlich`
Expected: Compiles (the widget is created but not used yet).

**Step 4: Commit**

```bash
git add crates/amlich/src/widgets/info_panel.rs crates/amlich/src/widgets/mod.rs
git commit -m "feat(tui): add InfoPanel widget with date headline, Can Chi, guidance, hours, tiet khi"
```

---

### Task 3: Add Calendar Toggle State to App

**Files:**
- Modify: `crates/amlich/src/app.rs:55-95` (App struct)
- Modify: `crates/amlich/src/app.rs:97-130` (App::new_with_date)

**Step 1: Add show_calendar field to App struct**

In `app.rs`, after `pub show_help: bool,` (line 94), add:
```rust
    // Calendar toggle (small screens)
    pub show_calendar: bool,
```

**Step 2: Initialize show_calendar in new_with_date**

In `App::new_with_date`, add to the struct initializer (after `show_help: false,`):
```rust
            show_calendar: false,
```

**Step 3: Add toggle method**

After the `toggle_help` method (around line 463), add:
```rust
    pub fn toggle_calendar(&mut self) {
        self.show_calendar = !self.show_calendar;
    }
```

**Step 4: Build to verify**

Run: `cargo build -p amlich`
Expected: Compiles with a warning about unused field (that's fine).

**Step 5: Commit**

```bash
git add crates/amlich/src/app.rs
git commit -m "feat(tui): add show_calendar toggle state to App"
```

---

### Task 4: Add 'c' Keybinding for Calendar Toggle

**Files:**
- Modify: `crates/amlich/src/event.rs:168-206` (main key handler)

**Step 1: Add 'c' key handler**

In the main `match key.code` block in `handle_key` (around line 168), add a new arm after the search toggle (`KeyCode::Char('/') => app.toggle_search()`) line:

```rust
        // Toggle calendar view (for small screens)
        KeyCode::Char('c') => app.toggle_calendar(),
```

**Step 2: Build to verify**

Run: `cargo build -p amlich`
Expected: Compiles.

**Step 3: Commit**

```bash
git add crates/amlich/src/event.rs
git commit -m "feat(tui): add 'c' keybinding to toggle calendar view"
```

---

### Task 5: Rewrite ui.rs Layout Logic

This is the core change — replacing the old layout with the info-first dashboard.

**Files:**
- Modify: `crates/amlich/src/ui.rs` (full rewrite of draw_header, draw_body, draw_footer)

**Step 1: Rewrite ui.rs**

Replace the entire contents of `crates/amlich/src/ui.rs`:

```rust
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use crate::theme;
use crate::widgets::{
    bookmarks::BookmarksOverlay, calendar::CalendarWidget, date_jump::DateJumpPopup,
    help::HelpOverlay, holidays::HolidayOverlay, info_panel::InfoPanel,
    insight_overlay::InsightOverlay, search::SearchPopup,
};

const MONTH_NAMES: [&str; 12] = [
    "Tháng 1",  "Tháng 2",  "Tháng 3",  "Tháng 4",
    "Tháng 5",  "Tháng 6",  "Tháng 7",  "Tháng 8",
    "Tháng 9",  "Tháng 10", "Tháng 11", "Tháng 12",
];

const MIN_TERM_W: u16 = 40;
const MIN_TERM_H: u16 = 15;

const BP_MEDIUM: u16 = 80;
const BP_LARGE: u16 = 120;

#[derive(Clone, Copy)]
enum LayoutMode {
    Small,
    Medium,
    Large,
}

fn layout_mode(width: u16) -> LayoutMode {
    if width < BP_MEDIUM {
        LayoutMode::Small
    } else if width < BP_LARGE {
        LayoutMode::Medium
    } else {
        LayoutMode::Large
    }
}

pub fn draw(frame: &mut Frame, app: &App) {
    let size = frame.area();

    if size.width < MIN_TERM_W || size.height < MIN_TERM_H {
        let msg = Paragraph::new("Terminal quá nhỏ.\nCần tối thiểu 40×15.")
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(theme::border_style()),
            );
        frame.render_widget(msg, size);
        return;
    }

    let vertical = Layout::vertical([
        Constraint::Length(1), // header — just 1 line, no border
        Constraint::Min(10),  // body
        Constraint::Length(1), // footer
    ])
    .split(size);

    draw_header(frame, app, vertical[0]);
    draw_body(frame, app, vertical[1]);
    draw_footer(frame, app, vertical[2]);

    // Overlays (render on top of body area)
    if app.show_holidays {
        frame.render_widget(HolidayOverlay::new(app), vertical[1]);
    }
    if app.show_insight {
        frame.render_widget(InsightOverlay::new(app), vertical[1]);
    }
    if app.show_bookmarks {
        frame.render_widget(BookmarksOverlay::new(app), vertical[1]);
    }
    if app.show_date_jump {
        frame.render_widget(DateJumpPopup::new(app), vertical[1]);
    }
    if app.show_search {
        frame.render_widget(SearchPopup::new(app), vertical[1]);
    }
    if app.show_help {
        frame.render_widget(HelpOverlay::new(), vertical[1]);
    }
}

fn draw_header(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let month_name = MONTH_NAMES
        .get(app.view_month.saturating_sub(1) as usize)
        .copied()
        .unwrap_or("Tháng ?");

    let mut spans = vec![
        Span::styled(
            " Âm Lịch",
            Style::default()
                .fg(theme::ACCENT_FG)
                .add_modifier(Modifier::BOLD),
        ),
    ];

    // Bookmark indicator
    if !app.bookmarks.is_empty() {
        spans.push(Span::styled(
            format!("  ◆{}", app.bookmarks.len()),
            Style::default().fg(theme::SECONDARY_FG),
        ));
    }

    // Search result indicator
    if !app.search_results.is_empty() {
        spans.push(Span::styled(
            format!("  /{}", app.search_results.len()),
            Style::default().fg(theme::SECONDARY_FG),
        ));
    }

    // Right-align: navigation
    let nav_text = format!("◂ {} {} ▸ ", month_name, app.view_year);
    let left_len: usize = spans.iter().map(|s| s.content.chars().count()).sum();
    let right_len = nav_text.chars().count();
    let padding = (area.width as usize).saturating_sub(left_len + right_len);

    spans.push(Span::raw(" ".repeat(padding)));
    spans.push(Span::styled(
        nav_text,
        Style::default().fg(theme::SECONDARY_FG),
    ));

    let header = Paragraph::new(Line::from(spans));
    frame.render_widget(header, area);
}

fn draw_body(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let mode = layout_mode(area.width);

    match mode {
        LayoutMode::Small => {
            if app.show_calendar {
                // Full-width calendar overlay (toggled by 'c')
                frame.render_widget(CalendarWidget::new(app), area);
            } else {
                // Info panel only — the hero view
                let inner = padded_area(area, 1, 0);
                frame.render_widget(InfoPanel::new(app), inner);
            }
        }
        LayoutMode::Medium => {
            // Mini calendar sidebar (~35%) + info panel (~65%)
            let cols = Layout::horizontal([
                Constraint::Percentage(35),
                Constraint::Percentage(65),
            ])
            .split(area);

            frame.render_widget(CalendarWidget::new(app), cols[0]);

            let inner = padded_area(cols[1], 1, 0);
            frame.render_widget(InfoPanel::new(app), inner);
        }
        LayoutMode::Large => {
            // Full calendar (~40%) + spacious info panel (~60%)
            let cols = Layout::horizontal([
                Constraint::Percentage(40),
                Constraint::Percentage(60),
            ])
            .split(area);

            frame.render_widget(CalendarWidget::new(app), cols[0]);

            let inner = padded_area(cols[1], 1, 0);
            frame.render_widget(InfoPanel::new(app), inner);
        }
    }
}

/// Add horizontal padding to an area
fn padded_area(area: ratatui::layout::Rect, h_pad: u16, v_pad: u16) -> ratatui::layout::Rect {
    ratatui::layout::Rect {
        x: area.x + h_pad,
        y: area.y + v_pad,
        width: area.width.saturating_sub(h_pad * 2),
        height: area.height.saturating_sub(v_pad * 2),
    }
}

fn draw_footer(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let mode = layout_mode(area.width);

    let spans = match mode {
        LayoutMode::Small => vec![
            Span::styled("c ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled(
                if app.show_calendar { "info" } else { "lịch" },
                Style::default().fg(theme::SECONDARY_FG),
            ),
            Span::raw("  "),
            Span::styled("t ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("nay", Style::default().fg(theme::SECONDARY_FG)),
            Span::raw("  "),
            Span::styled("g ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("nhảy", Style::default().fg(theme::SECONDARY_FG)),
            Span::raw("  "),
            Span::styled("? ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("help", Style::default().fg(theme::SECONDARY_FG)),
            Span::raw("  "),
            Span::styled("q ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("thoát", Style::default().fg(theme::SECONDARY_FG)),
        ],
        _ => vec![
            Span::styled(" hjkl ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("nav", Style::default().fg(theme::SECONDARY_FG)),
            Span::raw("  "),
            Span::styled("t ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("hôm nay", Style::default().fg(theme::SECONDARY_FG)),
            Span::raw("  "),
            Span::styled("/ ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("tìm", Style::default().fg(theme::SECONDARY_FG)),
            Span::raw("  "),
            Span::styled("g ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("nhảy", Style::default().fg(theme::SECONDARY_FG)),
            Span::raw("  "),
            Span::styled("b ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("bm", Style::default().fg(theme::SECONDARY_FG)),
            Span::raw("  "),
            Span::styled("? ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("help", Style::default().fg(theme::SECONDARY_FG)),
            Span::raw("  "),
            Span::styled("q ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("thoát", Style::default().fg(theme::SECONDARY_FG)),
        ],
    };

    let footer = Paragraph::new(Line::from(spans)).alignment(Alignment::Center);
    frame.render_widget(footer, area);
}
```

**Step 2: Build to verify**

Run: `cargo build -p amlich`
Expected: Compiles. The old `DetailWidget` and `HoursWidget` imports are no longer in ui.rs.

**Step 3: Run existing tests**

Run: `cargo test -p amlich`
Expected: All CLI contract tests pass (they don't touch the TUI rendering).

**Step 4: Commit**

```bash
git add crates/amlich/src/ui.rs
git commit -m "refactor(tui): rewrite layout to info-first dashboard with calendar sidebar"
```

---

### Task 6: Update Calendar Widget Styling

Update the calendar widget to use the new visual style (bold+underline for selected, inverted for today, no emoji in title).

**Files:**
- Modify: `crates/amlich/src/widgets/calendar.rs`

**Step 1: Update title**

Replace the block title (lines 27-33):
```rust
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(theme::border_style())
            .title(Line::from(Span::styled(
                " Lịch ",
                theme::section_style(),
            )));
```

**Step 2: Update selected day style**

Replace the solar_style computation (lines 103-125). Change the selected case to use bold+underline without background:
```rust
                let solar_style = if is_today {
                    Style::default()
                        .fg(theme::TODAY_FG)
                        .bg(theme::TODAY_BG)
                        .add_modifier(Modifier::BOLD)
                } else if is_selected {
                    Style::default()
                        .fg(theme::SOLAR_FG)
                        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
                } else if is_search_result {
                    Style::default()
                        .fg(theme::ACCENT_FG)
                        .add_modifier(Modifier::BOLD)
                } else if has_holiday {
                    Style::default()
                        .fg(theme::HOLIDAY_FG)
                        .add_modifier(Modifier::BOLD)
                } else if is_weekend {
                    Style::default().fg(theme::WEEKEND_FG)
                } else {
                    Style::default().fg(theme::SOLAR_FG)
                };
```

Note: today takes priority over selected now (inverted style is more visible).

**Step 3: Update lunar style for selected day**

Replace the lunar_style computation (lines 149-157):
```rust
                    let lunar_style = if is_today {
                        Style::default().fg(theme::TODAY_FG).bg(theme::TODAY_BG)
                    } else if is_selected {
                        Style::default()
                            .fg(theme::LUNAR_FG)
                            .add_modifier(Modifier::UNDERLINED)
                    } else {
                        Style::default().fg(theme::LUNAR_FG)
                    };
```

**Step 4: Build to verify**

Run: `cargo build -p amlich`
Expected: Compiles.

**Step 5: Commit**

```bash
git add crates/amlich/src/widgets/calendar.rs
git commit -m "refactor(tui): update calendar widget styling — inverted today, underline selected"
```

---

### Task 7: Update Help Overlay with New 'c' Keybinding

**Files:**
- Modify: `crates/amlich/src/widgets/help.rs`

**Step 1: Add 'c' to the Navigation section**

In the sections vec (around line 43), add to the Navigation shortcuts:
```rust
                    ("c", "bật/tắt lịch (small)"),
```

**Step 2: Update title style (remove emoji)**

Replace the title Span (line 34-36):
```rust
                Span::styled(
                    " Keyboard Shortcuts ",
                    theme::section_style(),
                ),
```

**Step 3: Build to verify**

Run: `cargo build -p amlich`
Expected: Compiles.

**Step 4: Commit**

```bash
git add crates/amlich/src/widgets/help.rs
git commit -m "refactor(tui): update help overlay with 'c' keybinding and remove emoji from title"
```

---

### Task 8: Update Remaining Overlay Widgets Styling

Update bookmarks, holidays, date_jump, search, insight overlays to use new theme constants.

**Files:**
- Modify: `crates/amlich/src/widgets/bookmarks.rs`
- Modify: `crates/amlich/src/widgets/holidays.rs`
- Modify: `crates/amlich/src/widgets/date_jump.rs`
- Modify: `crates/amlich/src/widgets/search.rs`
- Modify: `crates/amlich/src/widgets/insight_overlay.rs`

**Step 1: Update bookmarks.rs**

Replace the title (line 39-41):
```rust
              .title(Line::from(Span::styled(
                  " Bookmarks ",
                  theme::section_style(),
              )))
```

Replace `theme::VALUE_FG` → `theme::PRIMARY_FG`, `theme::LABEL_FG` → `theme::SECONDARY_FG`, `theme::SELECTED_FG` → `theme::ACCENT_FG` throughout the file.

**Step 2: Update holidays.rs**

Replace the title (lines 37-43) — remove emoji:
```rust
              .title(Line::from(Span::styled(
                  format!(
                      " Ngày Lễ — Tháng {}/{} ",
                      self.app.view_month, self.app.view_year
                  ),
                  theme::section_style(),
              )))
```

Replace `theme::VALUE_FG` → `theme::PRIMARY_FG`, `theme::LABEL_FG` → `theme::SECONDARY_FG`.

**Step 3: Update date_jump.rs**

Replace the title (lines 44-47) — remove emoji:
```rust
              .title(Line::from(Span::styled(
                  " Jump to Date ",
                  theme::section_style(),
              )))
```

Replace `theme::VALUE_FG` → `theme::PRIMARY_FG`, `theme::LABEL_FG` → `theme::SECONDARY_FG`.

**Step 4: Update search.rs**

Replace the title (lines 70-73) — remove emoji:
```rust
              .title(Line::from(Span::styled(
                  " Search Holidays ",
                  theme::section_style(),
              )))
```

Replace `theme::VALUE_FG` → `theme::PRIMARY_FG`, `theme::LABEL_FG` → `theme::SECONDARY_FG`, and `theme::SELECTED_BG` → `Color::Reset` (or remove background entirely for selected search result, keeping just bold+fg).

**Step 5: Update insight_overlay.rs**

Replace the title (lines 342-344):
```rust
              .title(Line::from(Span::styled(
                  format!(" Insight ({title_lang}) "),
                  theme::section_style(),
              )))
```

Replace `theme::TITLE_FG` → `theme::ACCENT_FG`, `theme::VALUE_FG` → `theme::PRIMARY_FG`, `theme::LABEL_FG` → `theme::SECONDARY_FG` throughout.

**Step 6: Build to verify**

Run: `cargo build -p amlich`
Expected: Compiles with no errors.

**Step 7: Commit**

```bash
git add crates/amlich/src/widgets/bookmarks.rs crates/amlich/src/widgets/holidays.rs crates/amlich/src/widgets/date_jump.rs crates/amlich/src/widgets/search.rs crates/amlich/src/widgets/insight_overlay.rs
git commit -m "refactor(tui): update overlay widgets to new theme — remove emoji from titles, use amber/green/coral"
```

---

### Task 9: Remove Temporary Theme Aliases

Now that all widgets are migrated, remove the backward-compatible aliases.

**Files:**
- Modify: `crates/amlich/src/theme.rs`

**Step 1: Remove aliases**

Remove the "Temporary aliases" section from `theme.rs`:
```rust
// Remove these lines:
pub const HEADER_BG: Color = ...
pub const HEADER_FG: Color = ...
pub const VALUE_FG: Color = ...
pub const LABEL_FG: Color = ...
pub const TITLE_FG: Color = ...
pub const SELECTED_BG: Color = ...
pub fn title_style() -> Style { ... }
```

**Step 2: Build to verify no references remain**

Run: `cargo build -p amlich`
Expected: Compiles. If any old constant references remain, fix them by replacing with new names.

**Step 3: Run all tests**

Run: `cargo test -p amlich`
Expected: All tests pass.

**Step 4: Commit**

```bash
git add crates/amlich/src/theme.rs
git commit -m "refactor(tui): remove temporary theme aliases — migration complete"
```

---

### Task 10: Final Verification

**Step 1: Full build**

Run: `cargo build -p amlich`
Expected: Clean build, no errors.

**Step 2: Run all tests**

Run: `cargo test -p amlich`
Expected: All tests pass.

**Step 3: Run clippy**

Run: `cargo clippy -p amlich -- -D warnings 2>&1 | head -40`
Expected: No warnings. Fix any clippy issues.

**Step 4: Manual test checklist**

Run: `cargo run -p amlich -- tui`

Verify:
- [ ] Header shows "Âm Lịch" left-aligned with month/year right-aligned, no colored bar
- [ ] Small screen (<80): shows info card only, no calendar
- [ ] Small screen: press `c` toggles calendar, press `c` again returns to info
- [ ] Medium screen (80-119): mini calendar sidebar + info panel
- [ ] Large screen (120+): full calendar + spacious info panel
- [ ] Info panel shows: date headline, lunar date with moon emoji, Can Chi with elements, Nên/Tránh summary, compact hours, Tiết Khí
- [ ] `i` opens full insight overlay
- [ ] All existing keybindings work: hjkl, n/p, N/P, t, g, /, b/B, H, ?
- [ ] Footer shows correct hints (includes `c` on small screens)
- [ ] Colors: amber headers, green for good, coral for bad, terminal-native text
- [ ] Today marker: inverted colors
- [ ] Selected day: bold + underline

**Step 5: Commit any final fixes**

```bash
git add -A
git commit -m "fix(tui): address final clippy warnings and polish"
```
