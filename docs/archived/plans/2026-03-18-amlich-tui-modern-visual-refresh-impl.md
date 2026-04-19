# Amlich-TUI Modern Visual Refresh Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Deliver a cohesive modern visual system for `crates/amlich-tui` (shell, cards, overlays, focus states, and responsive density) without changing domain logic.

**Architecture:** Build this in layers: first establish semantic design tokens, then modernize global shell chrome (status strip + ribbon), then apply a reusable card/modal system to high-traffic screens before completing parity across remaining screens. Keep all rendering data-driven from existing `AppState`/DTOs, and treat this as presentation-only refactoring.

**Tech Stack:** Rust, ratatui 0.29, crossterm 0.28, chrono, `amlich-api` DTOs, cargo test/clippy/fmt.

---

## Implementation Rules For Executor

- Use `@test-driven-development` for every behavior change.
- Use `@verification-before-completion` before claiming each task complete.
- Keep commits small and scoped per task.
- Do not change recommendation policy or core almanac computation.

---

### Task 1: Semantic Theme Tokens and Style Helpers

**Files:**
- Modify: `crates/amlich-tui/src/theme.rs`
- Test: `crates/amlich-tui/src/theme.rs` (existing/new `#[cfg(test)]` module)

**Step 1: Write failing tests for token semantics**

Add tests asserting all required token helpers exist and return deterministic styles:

```rust
#[test]
fn semantic_tokens_have_expected_roles() {
    assert_eq!(Theme::accent_good().fg, Some(Color::Green));
    assert_eq!(Theme::accent_warn().fg, Some(Color::Yellow));
    assert_eq!(Theme::accent_bad().fg, Some(Color::Red));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p amlich-tui theme::tests::semantic_tokens_have_expected_roles`
Expected: FAIL because new helpers do not exist yet.

**Step 3: Implement semantic palette and helper constructors**

Add neutral and accent helpers with explicit intent:

```rust
pub fn text_primary() -> Style { Style::default().fg(Self::WHITE) }
pub fn text_muted() -> Style { Style::default().fg(Self::CHARCOAL) }
pub fn accent_info() -> Style { Style::default().fg(Self::CYAN) }
pub fn accent_good() -> Style { Style::default().fg(Self::GREEN) }
pub fn accent_warn() -> Style { Style::default().fg(Self::GOLD) }
pub fn accent_bad() -> Style { Style::default().fg(Self::RED) }
```

Also add border hierarchy helpers such as `border_primary()`, `border_secondary()`, and `border_ghost()`.

**Step 4: Run tests for theme module**

Run: `cargo test -p amlich-tui theme`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/amlich-tui/src/theme.rs
git commit -m "feat(amlich-tui): add semantic theme tokens and border hierarchy helpers"
```

---

### Task 2: Add Modern Shell Chrome (Top Status Strip + Refined Ribbon)

**Files:**
- Create: `crates/amlich-tui/src/widgets/status_strip.rs`
- Modify: `crates/amlich-tui/src/widgets/mod.rs`
- Modify: `crates/amlich-tui/src/layout.rs`
- Modify: `crates/amlich-tui/src/widgets/ribbon.rs`
- Test: `crates/amlich-tui/src/widgets/ribbon.rs`
- Test: `crates/amlich-tui/src/layout.rs`

**Step 1: Write failing tests for shell placement**

Add tests that assert layout now renders three bands: status strip, page area, ribbon.

```rust
#[test]
fn layout_renders_top_status_strip_and_bottom_ribbon() {
    let rect = Rect::new(0, 0, 120, 40);
    let chunks = Layout::vertical([Constraint::Length(1), Constraint::Min(10), Constraint::Length(2)]).split(rect);
    assert_eq!(chunks[0].height, 1);
    assert_eq!(chunks[2].height, 2);
}
```

**Step 2: Run test to verify failure**

Run: `cargo test -p amlich-tui layout_renders_top_status_strip_and_bottom_ribbon`
Expected: FAIL (layout currently has no top strip).

**Step 3: Implement status strip widget and wire in layout**

Create status strip with compact chips (date, ruleset, event kind, packs) and render it before page body.

```rust
frame.render_widget(StatusStripWidget::new(app, mode), status_area);
frame.render_widget(PageWidget::new(app, mode), page_area);
frame.render_widget(RibbonWidget::new(app, mode), ribbon_area);
```

Update ribbon to show mode-aware shortcut hints only relevant to current app mode.

**Step 4: Run focused tests**

Run: `cargo test -p amlich-tui layout`
Run: `cargo test -p amlich-tui ribbon`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/amlich-tui/src/widgets/status_strip.rs crates/amlich-tui/src/widgets/mod.rs crates/amlich-tui/src/layout.rs crates/amlich-tui/src/widgets/ribbon.rs
git commit -m "feat(amlich-tui): add status strip and modernize mode-aware ribbon"
```

---

### Task 3: Reusable Card Shell and Page Status Cards

**Files:**
- Create: `crates/amlich-tui/src/widgets/card_shell.rs`
- Modify: `crates/amlich-tui/src/widgets/mod.rs`
- Modify: `crates/amlich-tui/src/widgets/page.rs`
- Test: `crates/amlich-tui/src/widgets/page.rs`

**Step 1: Write failing tests for loading/error presentation**

Add tests asserting page loading/error states render card-framed status labels (not plain text).

```rust
#[test]
fn loading_state_uses_status_card() {
    let mut app = sample_app_state();
    app.is_loading = true;
    let text = render_text(&app);
    assert!(text.contains("ĐANG TẢI"));
}
```

**Step 2: Run test to verify failure**

Run: `cargo test -p amlich-tui loading_state_uses_status_card`
Expected: FAIL.

**Step 3: Implement shared card shell and migrate page shell_message**

Introduce helper API:

```rust
pub fn render_status_card(area: Rect, buf: &mut Buffer, title: &str, body: Vec<Line>) { /* ... */ }
```

Replace `shell_message(...)` path in `page.rs` with card shell rendering for loading/error/empty states.

**Step 4: Run page tests**

Run: `cargo test -p amlich-tui page`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/amlich-tui/src/widgets/card_shell.rs crates/amlich-tui/src/widgets/mod.rs crates/amlich-tui/src/widgets/page.rs
git commit -m "refactor(amlich-tui): add reusable status card shell for page loading and error states"
```

---

### Task 4: High-Traffic Screen Card Consistency and Focus Styling

**Files:**
- Modify: `crates/amlich-tui/src/widgets/screens/dashboard.rs`
- Modify: `crates/amlich-tui/src/widgets/screens/insight.rs`
- Modify: `crates/amlich-tui/src/widgets/screens/recommendations.rs`
- Modify: `crates/amlich-tui/src/widgets/page.rs`
- Test: `crates/amlich-tui/src/widgets/screens/dashboard.rs`
- Test: `crates/amlich-tui/src/widgets/screens/recommendations.rs`

**Step 1: Write failing tests for selected/focused visual marker**

Add tests that assert focused blocks render a marker/chrome change beyond color-only style.

```rust
#[test]
fn dashboard_focus_state_renders_primary_marker() {
    let app = sample_app_state();
    let text = render_dashboard_text(&app);
    assert!(text.contains("▌") || text.contains("▶"));
}
```

**Step 2: Run tests to verify failure**

Run: `cargo test -p amlich-tui dashboard_focus_state_renders_primary_marker`
Expected: FAIL.

**Step 3: Implement consistent card anatomy on these screens**

Ensure each screen card follows:

1. title
2. metric/verdict line
3. detail body
4. evidence/footer

Use `Theme` semantic helpers + `card_shell` for consistent frame/border behavior.

**Step 4: Run targeted tests**

Run: `cargo test -p amlich-tui dashboard`
Run: `cargo test -p amlich-tui recommendations`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/amlich-tui/src/widgets/screens/dashboard.rs crates/amlich-tui/src/widgets/screens/insight.rs crates/amlich-tui/src/widgets/screens/recommendations.rs crates/amlich-tui/src/widgets/page.rs
git commit -m "feat(amlich-tui): standardize high-traffic screens with modern card anatomy and focus markers"
```

---

### Task 5: Overlay Visual Unification (Search, Context, Help)

**Files:**
- Create: `crates/amlich-tui/src/widgets/modal_shell.rs`
- Modify: `crates/amlich-tui/src/widgets/mod.rs`
- Modify: `crates/amlich-tui/src/widgets/search.rs`
- Modify: `crates/amlich-tui/src/widgets/context.rs`
- Modify: `crates/amlich-tui/src/widgets/help.rs`
- Modify: `crates/amlich-tui/src/widgets/explorer.rs`
- Test: `crates/amlich-tui/src/widgets/search.rs`
- Test: `crates/amlich-tui/src/widgets/help.rs`

**Step 1: Write failing tests for modal shell consistency**

Add tests that assert overlays use shared title framing and centered bounds policy.

```rust
#[test]
fn search_modal_uses_shared_shell_title_format() {
    let app = sample_search_app();
    let text = render_search_overlay(&app);
    assert!(text.contains("Tìm kiếm"));
}
```

**Step 2: Run tests to verify failure**

Run: `cargo test -p amlich-tui search_modal_uses_shared_shell_title_format`
Expected: FAIL.

**Step 3: Implement `modal_shell` helper and migrate overlays**

Provide helper for width/height per layout mode and consistent block style:

```rust
pub fn centered_modal(area: Rect, mode: LayoutMode, preset: ModalPreset) -> Rect { /* ... */ }
```

Use it in Search/Context/Help; keep existing behavior but normalize chrome and spacing.

**Step 4: Run overlay tests**

Run: `cargo test -p amlich-tui search`
Run: `cargo test -p amlich-tui help`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/amlich-tui/src/widgets/modal_shell.rs crates/amlich-tui/src/widgets/mod.rs crates/amlich-tui/src/widgets/search.rs crates/amlich-tui/src/widgets/context.rs crates/amlich-tui/src/widgets/help.rs crates/amlich-tui/src/widgets/explorer.rs
git commit -m "refactor(amlich-tui): unify overlay chrome with shared modal shell"
```

---

### Task 6: Remaining Screen Parity (Hours, Elements, FengShui, SolarTerms, Calendar)

**Files:**
- Modify: `crates/amlich-tui/src/widgets/screens/hours.rs`
- Modify: `crates/amlich-tui/src/widgets/screens/elements.rs`
- Modify: `crates/amlich-tui/src/widgets/screens/feng_shui.rs`
- Modify: `crates/amlich-tui/src/widgets/screens/solar_terms.rs`
- Modify: `crates/amlich-tui/src/widgets/calendar.rs`
- Modify: `crates/amlich-tui/src/widgets/week_strip.rs`
- Test: per-screen `#[cfg(test)]` modules where present

**Step 1: Write failing tests for shared card/border conventions**

For each screen, add one assertion that title and block hierarchy follow the shared style conventions.

**Step 2: Run tests to verify failure**

Run: `cargo test -p amlich-tui hours`
Run: `cargo test -p amlich-tui elements`
Expected: at least one new test fails before implementation.

**Step 3: Implement style parity changes**

Apply `Theme` semantic helpers and consistent card shell usage. Ensure small/medium/large spacing remains deterministic.

**Step 4: Re-run screen tests**

Run: `cargo test -p amlich-tui hours`
Run: `cargo test -p amlich-tui elements`
Run: `cargo test -p amlich-tui feng_shui`
Run: `cargo test -p amlich-tui solar_terms`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/amlich-tui/src/widgets/screens/hours.rs crates/amlich-tui/src/widgets/screens/elements.rs crates/amlich-tui/src/widgets/screens/feng_shui.rs crates/amlich-tui/src/widgets/screens/solar_terms.rs crates/amlich-tui/src/widgets/calendar.rs crates/amlich-tui/src/widgets/week_strip.rs
git commit -m "feat(amlich-tui): apply modern visual parity across remaining screens"
```

---

### Task 7: Width Regression and Keyboard Flow Verification Tests

**Files:**
- Modify: `crates/amlich-tui/src/layout.rs`
- Modify: `crates/amlich-tui/src/events.rs`
- Test: `crates/amlich-tui/src/layout.rs`
- Test: `crates/amlich-tui/src/events.rs`

**Step 1: Write failing tests for width breakpoints and shell integrity**

Add tests that verify expected mode selection at boundaries and that status/ribbon are still rendered under small width constraints.

**Step 2: Write failing tests for key flow in modal modes**

Ensure navigation hints and mode switches remain coherent after ribbon/status changes.

**Step 3: Run tests to verify failure**

Run: `cargo test -p amlich-tui layout_mode`
Run: `cargo test -p amlich-tui tab_cycles_screen`
Expected: failing assertions before fixes.

**Step 4: Implement minimal fixes and re-run tests**

Run: `cargo test -p amlich-tui layout`
Run: `cargo test -p amlich-tui events`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/amlich-tui/src/layout.rs crates/amlich-tui/src/events.rs
git commit -m "test(amlich-tui): add layout width and keyboard flow regression coverage for modern shell"
```

---

### Task 8: Final Verification and Documentation

**Files:**
- Modify: `docs/plans/2026-03-18-amlich-tui-modern-visual-refresh-design.md` (if any approved deltas)
- Optional modify: `README.md` (if shortcut legend materially changed)

**Step 1: Run full quality suite**

Run:

```bash
cargo fmt --all --check
cargo clippy -p amlich-tui -- -W clippy::all
cargo test -p amlich-tui
```

Expected: all PASS.

**Step 2: Manual smoke test**

Run: `cargo run -p amlich-cli -- tui --date 2026-03-18`

Check:

- top strip visible and legible
- ribbon remains mode-aware
- overlays share visual shell
- focus states obvious in keyboard navigation
- small/medium/large terminal widths remain usable

**Step 3: Commit final polish**

```bash
git add docs/plans/2026-03-18-amlich-tui-modern-visual-refresh-design.md README.md
git commit -m "docs(amlich-tui): align modern visual refresh docs and shortcut guidance"
```

---

## Completion Checklist

- [ ] All tasks executed with TDD loop (fail -> implement -> pass)
- [ ] Commits are task-scoped and descriptive
- [ ] No changes to domain policy logic
- [ ] `cargo test -p amlich-tui` passes at end
- [ ] Visual checks passed on small/medium/large terminal widths
