# Screen-Based TUI Redesign Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Replace the current section-heavy day view with a screen-based flow `General -> Insight -> Recommendations -> Deep`, with `Tab/Shift+Tab` cycling screens and clearly different content per screen.

**Architecture:** Introduce a dedicated `AppScreen` router state in `AppState` and render exactly one screen widget at a time in `PageWidget`. Keep calendar/search overlays and day navigation behaviors, but remove section-centric day-view interactions (`focused_section`, zoom/expand semantics in events/ribbon/page rendering path).

**Tech Stack:** Rust, ratatui, crossterm, chrono, amlich-api DTOs.

---

### Task 1: Add Router State And Navigation APIs

**Files:**
- Modify: `crates/amlich-tui/src/state.rs`
- Test: `crates/amlich-tui/src/state.rs` (unit tests module)

**Step 1: Write the failing test**
- Add tests for `next_screen()` and `prev_screen()` order: `General -> Insight -> Recommendations -> Deep`.

**Step 2: Run test to verify it fails**
- Run: `cargo test -p amlich-tui screen_cycle`
- Expected: FAIL because `AppScreen` and cycle methods do not exist.

**Step 3: Write minimal implementation**
- Add `AppScreen` enum.
- Add `active_screen` and `screen_history` to `AppState`.
- Add `next_screen`, `prev_screen`, `screen_name` helpers.

**Step 4: Run test to verify it passes**
- Run: `cargo test -p amlich-tui screen_cycle`
- Expected: PASS.

### Task 2: Remap Events To Screen Cycling

**Files:**
- Modify: `crates/amlich-tui/src/events.rs`
- Test: `crates/amlich-tui/src/events.rs` (unit tests module)

**Step 1: Write the failing test**
- Add tests proving `Tab` and `BackTab` cycle screens in day view.
- Add tests proving legacy section behavior is not used for tab focus anymore.

**Step 2: Run test to verify it fails**
- Run: `cargo test -p amlich-tui tab_cycles_screen`
- Expected: FAIL.

**Step 3: Write minimal implementation**
- Remap `Tab/BackTab` to `next_screen/prev_screen`.
- Remove section-based day-view expand/zoom shortcuts.

**Step 4: Run test to verify it passes**
- Run: `cargo test -p amlich-tui tab_cycles_screen`
- Expected: PASS.

### Task 3: Replace Page Section Renderer With Screen Router

**Files:**
- Modify: `crates/amlich-tui/src/widgets/page.rs`
- Modify: `crates/amlich-tui/src/widgets/mod.rs`
- Create: `crates/amlich-tui/src/widgets/screens/mod.rs`
- Create: `crates/amlich-tui/src/widgets/screens/general.rs`
- Create: `crates/amlich-tui/src/widgets/screens/insight.rs`
- Create: `crates/amlich-tui/src/widgets/screens/recommendations.rs`
- Create: `crates/amlich-tui/src/widgets/screens/deep.rs`
- Test: `crates/amlich-tui/src/widgets/page.rs` and per-screen files

**Step 1: Write the failing test**
- Add tests asserting each screen renders its own title/signature.

**Step 2: Run test to verify it fails**
- Run: `cargo test -p amlich-tui screen_renders`
- Expected: FAIL.

**Step 3: Write minimal implementation**
- Route by `active_screen` and render one screen widget only.
- Keep week strip and calendar behavior.

**Step 4: Run test to verify it passes**
- Run: `cargo test -p amlich-tui screen_renders`
- Expected: PASS.

### Task 4: Update Ribbon To Display Active Screen

**Files:**
- Modify: `crates/amlich-tui/src/widgets/ribbon.rs`
- Test: `crates/amlich-tui/src/widgets/ribbon.rs`

**Step 1: Write the failing test**
- Add test expecting screen name label on hotkey line and clean weekday line unchanged.

**Step 2: Run test to verify it fails**
- Run: `cargo test -p amlich-tui ribbon_places_hotkeys_on_the_top_line_only`
- Expected: FAIL due old section label.

**Step 3: Write minimal implementation**
- Replace section label with active screen label.

**Step 4: Run test to verify it passes**
- Run: `cargo test -p amlich-tui ribbon_places_hotkeys_on_the_top_line_only`
- Expected: PASS.

### Task 5: Verify Full Suite

**Files:**
- No code changes

**Step 1: Run targeted tests**
- Run: `cargo test -p amlich-tui`

**Step 2: Verify output**
- Expected: all tests pass; no regression in calendar/search/day navigation.
