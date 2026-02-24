# TUI Front Page Insight Density Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Upgrade the large-screen (`120+`) TUI front page to a polished `35/65` dashboard that surfaces richer insight (verdict, expanded hours, holiday context, 3-day preview) with minimal overlay dependency.

**Architecture:** Keep current breakpoints and core navigation, but add a large-screen dashboard layer composed of focused cards on top of existing day data + insight cache. Introduce a pure `dashboard` domain module for derivations (verdict, upcoming holidays, preview rows) so rendering and key handling stay deterministic and testable. Wire new focus/activation behavior in `App` + `event`, and render the new composition in `ui` + `InfoPanel`.

**Tech Stack:** Rust, Ratatui 0.30, Crossterm 0.29, Chrono, `cargo test`

---

Implementation notes:
- Follow @test-driven-development for every task.
- Run @verification-before-completion checks before claiming done.
- Keep commits small and task-scoped.

### Task 1: Add Pure Dashboard Derivation Module

**Files:**
- Create: `crates/amlich/src/dashboard.rs`
- Modify: `crates/amlich/src/main.rs`
- Test: `crates/amlich/src/dashboard.rs` (inline `#[cfg(test)]`)

**Step 1: Write the failing tests**

Add tests first in `dashboard.rs`:

```rust
#[test]
fn verdict_is_good_when_good_outnumbers_avoid_by_two() {
    let verdict = derive_verdict(Some((&vec!["a".into(); 4], &vec!["b".into(); 1])));
    assert_eq!(verdict.label_vi, "Tốt");
}

#[test]
fn verdict_is_caution_when_avoid_outnumbers_good_by_two() {
    let verdict = derive_verdict(Some((&vec!["a".into(); 1], &vec!["b".into(); 4])));
    assert_eq!(verdict.label_vi, "Cần cân nhắc");
}

#[test]
fn upcoming_holidays_cross_year_boundary() {
    let from = NaiveDate::from_ymd_opt(2026, 12, 31).unwrap();
    let items = select_upcoming_holidays(from, &sample_holidays(), 2);
    assert!(!items.is_empty());
    assert!(items[0].days_left >= 1);
}

#[test]
fn preview_rows_roll_over_month_end() {
    let from = NaiveDate::from_ymd_opt(2026, 1, 31).unwrap();
    let rows = build_preview_rows(from, 3);
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[1].date.day(), 1);
    assert_eq!(rows[1].date.month(), 2);
}
```

**Step 2: Run tests to verify failure**

Run: `cargo test -p amlich dashboard::tests:: -q`  
Expected: FAIL (missing module/functions/types).

**Step 3: Implement minimal dashboard domain**

Implement in `dashboard.rs`:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerdictLevel { Good, Neutral, Caution, Unknown }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verdict {
    pub level: VerdictLevel,
    pub label_vi: &'static str,
    pub icon: &'static str,
}

pub fn derive_verdict(guidance: Option<(&Vec<String>, &Vec<String>)>) -> Verdict { /* ... */ }
pub fn select_upcoming_holidays(from: NaiveDate, holidays: &[HolidayDto], limit: usize) -> Vec<UpcomingHoliday> { /* ... */ }
pub fn build_preview_rows(from: NaiveDate, days: usize) -> Vec<PreviewRow> { /* ... */ }
```

Also wire module in `main.rs`:

```rust
mod dashboard;
```

**Step 4: Run tests to verify pass**

Run: `cargo test -p amlich dashboard::tests:: -q`  
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/amlich/src/dashboard.rs crates/amlich/src/main.rs
git commit -m "feat(tui): add pure dashboard derivation module for verdict holidays and preview"
```

### Task 2: Add App State for Layout Mode and Dashboard Focus

**Files:**
- Modify: `crates/amlich/src/app.rs`
- Test: `crates/amlich/src/app.rs` (existing `#[cfg(test)]` module)

**Step 1: Write failing tests**

Add tests for:

```rust
#[test]
fn dashboard_focus_cycles_forward_and_backward() {
    let mut app = test_app();
    app.set_layout_mode(LayoutMode::Large);
    assert_eq!(app.focused_card(), DashboardCard::Hero);
    app.focus_next_card();
    assert_eq!(app.focused_card(), DashboardCard::Guidance);
    app.focus_prev_card();
    assert_eq!(app.focused_card(), DashboardCard::Hero);
}

#[test]
fn numeric_focus_ignored_outside_large_layout() {
    let mut app = test_app();
    app.set_layout_mode(LayoutMode::Medium);
    app.focus_card_index(3);
    assert_eq!(app.focused_card(), DashboardCard::Hero);
}
```

**Step 2: Run tests to verify failure**

Run: `cargo test -p amlich dashboard_focus_ -q`  
Expected: FAIL (missing enums/methods).

**Step 3: Implement minimal app state**

Add to `app.rs`:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum LayoutMode { Small, Medium, #[default] Large }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DashboardCard { #[default] Hero, Guidance, Hours, Holiday, Preview }
```

Add fields:

```rust
pub layout_mode: LayoutMode,
pub focused_card: DashboardCard,
```

Add methods:

```rust
pub fn set_layout_mode(&mut self, mode: LayoutMode) { /* ... */ }
pub fn focused_card(&self) -> DashboardCard { self.focused_card }
pub fn focus_next_card(&mut self) { /* ... */ }
pub fn focus_prev_card(&mut self) { /* ... */ }
pub fn focus_card_index(&mut self, one_based: u8) { /* ... */ }
```

**Step 4: Run tests to verify pass**

Run: `cargo test -p amlich dashboard_focus_ -q`  
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/amlich/src/app.rs
git commit -m "feat(tui): add app layout-mode and dashboard card focus state"
```

### Task 3: Wire Large-Screen Focus Keys in Event Handling

**Files:**
- Modify: `crates/amlich/src/event.rs`
- Modify: `crates/amlich/src/app.rs` (if activation helper needed)
- Test: `crates/amlich/src/event.rs` (new inline tests)

**Step 1: Write failing tests**

Add event tests that simulate key presses:

```rust
#[test]
fn tab_cycles_cards_in_large_mode_without_changing_month() {
    let mut app = test_app();
    app.set_layout_mode(LayoutMode::Large);
    let month = app.view_month;
    handle_key(&mut app, KeyEvent::from(KeyCode::Tab));
    assert_eq!(app.view_month, month);
    assert_eq!(app.focused_card(), DashboardCard::Guidance);
}

#[test]
fn tab_keeps_month_navigation_in_medium_mode() {
    let mut app = test_app();
    app.set_layout_mode(LayoutMode::Medium);
    let month = app.view_month;
    handle_key(&mut app, KeyEvent::from(KeyCode::Tab));
    assert_ne!(app.view_month, month);
}

#[test]
fn enter_opens_insight_from_guidance_card() {
    let mut app = test_app();
    app.set_layout_mode(LayoutMode::Large);
    app.focus_card_index(2);
    handle_key(&mut app, KeyEvent::from(KeyCode::Enter));
    assert!(app.show_insight);
}
```

**Step 2: Run tests to verify failure**

Run: `cargo test -p amlich tab_cycles_cards_in_large_mode_without_changing_month -q`  
Expected: FAIL.

**Step 3: Implement key behavior**

In `event.rs`, branch by `app.layout_mode`:

```rust
if app.layout_mode == LayoutMode::Large {
    match key.code {
        KeyCode::Tab => { app.focus_next_card(); return; }
        KeyCode::BackTab => { app.focus_prev_card(); return; }
        KeyCode::Enter => { app.activate_focused_card(); return; }
        KeyCode::Char(c @ '1'..='5') => { app.focus_card_index(c.to_digit(10).unwrap() as u8); return; }
        _ => {}
    }
}
```

Add `activate_focused_card()` in `app.rs` with mapping:
- Hero/Guidance/Hours/Holiday -> `toggle_insight()`, set proper tab
- Preview -> jump selection to preview row 0/1/2 policy (documented in method comment)

**Step 4: Run tests to verify pass**

Run: `cargo test -p amlich tab_cycles_cards_in_large_mode_without_changing_month -q`  
Run: `cargo test -p amlich enter_opens_insight_from_guidance_card -q`  
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/amlich/src/event.rs crates/amlich/src/app.rs
git commit -m "feat(tui): add large-screen dashboard focus and activation keybindings"
```

### Task 4: Update UI Large Breakpoint Composition and Footer

**Files:**
- Modify: `crates/amlich/src/ui.rs`
- Modify: `crates/amlich/src/app.rs` (if needed for layout sync)
- Test: `crates/amlich/src/ui.rs` (new inline tests for pure helpers)

**Step 1: Write failing tests**

Add helper tests:

```rust
#[test]
fn large_mode_split_is_35_65() {
    let (left, right) = large_split_percentages();
    assert_eq!((left, right), (35, 65));
}

#[test]
fn layout_mode_thresholds_match_spec() {
    assert_eq!(layout_mode(79), LayoutMode::Small);
    assert_eq!(layout_mode(80), LayoutMode::Medium);
    assert_eq!(layout_mode(119), LayoutMode::Medium);
    assert_eq!(layout_mode(120), LayoutMode::Large);
}
```

**Step 2: Run tests to verify failure**

Run: `cargo test -p amlich large_mode_split_is_35_65 -q`  
Expected: FAIL.

**Step 3: Implement UI updates**

- Expose/align `LayoutMode` usage with `app::LayoutMode`.
- In `draw_body`, set `app.layout_mode` every frame (or in app state update call path).
- Change large split to:

```rust
let cols = Layout::horizontal([Constraint::Percentage(35), Constraint::Percentage(65)]).split(area);
```

- Update footer text in large mode to include:
`Tab/Shift+Tab focus`, `Enter mở chi tiết`, `1-5 chọn thẻ`.

**Step 4: Run tests to verify pass**

Run: `cargo test -p amlich large_mode_split_is_35_65 -q`  
Run: `cargo test -p amlich layout_mode_thresholds_match_spec -q`  
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/amlich/src/ui.rs crates/amlich/src/app.rs
git commit -m "feat(tui): set large layout to 35/65 and show dashboard-focus footer hints"
```

### Task 5: Refactor InfoPanel into Card-Based Large Dashboard Content

**Files:**
- Modify: `crates/amlich/src/widgets/info_panel.rs`
- Modify: `crates/amlich/src/theme.rs` (if card focus style tokens are needed)
- Test: `crates/amlich/src/widgets/info_panel.rs` (inline tests on pure line builders)

**Step 1: Write failing tests**

Add pure builder tests:

```rust
#[test]
fn hero_card_includes_verdict_label() {
    let lines = build_hero_lines(&sample_day_info(), &sample_insight());
    assert!(lines.iter().any(|l| l.to_string().contains("Tốt")));
}

#[test]
fn hours_card_shows_time_ranges_and_stars() {
    let lines = build_hours_lines(&sample_day_info(), 3);
    let text = lines.iter().map(|l| l.to_string()).collect::<Vec<_>>().join("\n");
    assert!(text.contains("23:00-01:00"));
    assert!(text.contains("Thanh Long"));
}

#[test]
fn preview_card_renders_three_rows() {
    let rows = sample_preview_rows();
    let lines = build_preview_lines(&rows);
    assert!(lines.len() >= 3);
}
```

**Step 2: Run tests to verify failure**

Run: `cargo test -p amlich hero_card_includes_verdict_label -q`  
Expected: FAIL.

**Step 3: Implement card builders and render composition**

Inside `info_panel.rs`:

```rust
fn build_hero_lines(/* ... */) -> Vec<Line<'static>> { /* ... */ }
fn build_guidance_lines(/* ... */) -> Vec<Line<'static>> { /* ... */ }
fn build_hours_lines(/* ... */) -> Vec<Line<'static>> { /* ... */ }
fn build_holiday_lines(/* ... */) -> Vec<Line<'static>> { /* ... */ }
fn build_preview_lines(/* ... */) -> Vec<Line<'static>> { /* ... */ }
```

Render in large mode as stacked card blocks with section headers and truncation rules from design doc.

**Step 4: Run tests to verify pass**

Run: `cargo test -p amlich hero_card_includes_verdict_label -q`  
Run: `cargo test -p amlich hours_card_shows_time_ranges_and_stars -q`  
Run: `cargo test -p amlich preview_card_renders_three_rows -q`  
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/amlich/src/widgets/info_panel.rs crates/amlich/src/theme.rs
git commit -m "feat(tui): render large-screen info panel as dense dashboard cards"
```

### Task 6: End-to-End Verification and Final Cleanup

**Files:**
- Modify: `crates/amlich/src/widgets/help.rs` (only if keybinding docs changed)
- Modify: `README.md` or `apps/desktop/README.md` only if TUI key docs are documented there

**Step 1: Write/adjust failing tests for final behavior**

Add one integration-oriented assertion in existing unit suites:

```rust
#[test]
fn large_mode_tab_does_not_change_month_after_full_wiring() {
    let mut app = test_app();
    app.set_layout_mode(LayoutMode::Large);
    let month = app.view_month;
    handle_key(&mut app, KeyEvent::from(KeyCode::Tab));
    assert_eq!(app.view_month, month);
}
```

**Step 2: Run targeted test to verify baseline**

Run: `cargo test -p amlich large_mode_tab_does_not_change_month_after_full_wiring -q`  
Expected: PASS.

**Step 3: Run full verification suite**

Run:

```bash
cargo test -p amlich
cargo test --workspace
cargo fmt --all --check
```

Expected:
- All `amlich` tests pass
- Workspace tests pass
- Formatting check passes

**Step 4: Manual verification**

Run and verify visually:

```bash
cargo run -p amlich -- tui --date 2026-02-24
```

Manual checks:
- Width `120+`: `35/65` split applied
- `Tab`/`Shift+Tab` focus cards
- `Enter` triggers mapped detail behavior
- Hero/guidance/hours/holiday/preview cards all populated
- Fallback text visible when data missing

**Step 5: Commit**

```bash
git add crates/amlich/src/event.rs crates/amlich/src/ui.rs crates/amlich/src/widgets/info_panel.rs crates/amlich/src/app.rs crates/amlich/src/widgets/help.rs README.md
git commit -m "chore(tui): verify and document large-screen dashboard interaction model"
```

