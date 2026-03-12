# Actionability-First TUI Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Redesign `amlich-tui` into a denser, actionability-first single-day almanac page that surfaces much more of the data already available from `amlich-api` without moving domain logic into widgets.

**Architecture:** Keep the existing boundary `amlich-core -> amlich-api DTOs -> amlich-tui`. Build the redesign in layers: first stabilize page structure and interaction state, then add presentation-oriented view-model helpers / DTO expansions only where the TUI lacks structured inputs, then implement new section widgets in top-down priority order, and finally tighten responsive behavior and tests. Preserve the current `amlich-cli` launcher path (`cargo run -p amlich-cli -- tui`).

**Tech Stack:** Rust workspace, `amlich-tui`, `amlich-api`, `amlich-cli`, `ratatui`, `crossterm`, serde DTOs, existing widget/unit tests, cargo test.

---

### Task 1: Lock the target page skeleton with failing layout/state tests

**Files:**
- Modify: `crates/amlich-tui/src/state.rs`
- Modify: `crates/amlich-tui/src/layout.rs`
- Modify: `crates/amlich-tui/src/widgets/page.rs`
- Test: `crates/amlich-tui/src/widgets/page.rs`
- Test: `crates/amlich-tui/src/state.rs`
- Read for context: `docs/plans/2026-03-12-actionability-first-tui-design.md`

**Step 1: Write the failing tests**

In `crates/amlich-tui/src/state.rs` add focused tests for the new page state:
- `section_focus_cycles_in_order()`
- `evidence_toggle_changes_visibility_flag()`
- `zoom_mode_tracks_focused_section()`
- `expand_toggle_is_scoped_to_focused_section()`

In `crates/amlich-tui/src/widgets/page.rs` add tests that exercise a deterministic sample `AppState` and assert page section order for the new home screen:
- Hero
- Recommendations
- Timing
- Travel
- Risks
- Traditional Evidence
- Expanded Details

Keep the tests small and structural; they do not need golden screenshots yet.

**Step 2: Run tests to verify failure**

Run: `cargo test -p amlich-tui page`
Run: `cargo test -p amlich-tui state`
Expected: FAIL because the new section model, focus tracking, and page skeleton do not exist yet.

**Step 3: Write minimal implementation**

Implement only the smallest scaffolding needed:
- add section/focus enums and state fields to `AppState`
- add state methods for cycling focus, toggling evidence, toggling zoom, and expanding the focused section
- update `PageWidget` so the non-calendar view is driven by the new top-to-bottom section order
- keep placeholder section bodies where necessary; do not build all final rendering yet

**Step 4: Run tests to verify pass**

Run: `cargo test -p amlich-tui page`
Run: `cargo test -p amlich-tui state`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/amlich-tui/src/state.rs crates/amlich-tui/src/layout.rs crates/amlich-tui/src/widgets/page.rs
git commit -m "feat(tui): add actionability-first page skeleton state"
```

---

### Task 2: Wire the new interaction model with test-first event handling

**Files:**
- Modify: `crates/amlich-tui/src/events.rs`
- Modify: `crates/amlich-tui/src/state.rs`
- Test: `crates/amlich-tui/src/events.rs`
- Read for context: `crates/amlich-tui/src/widgets/ribbon.rs`

**Step 1: Write the failing tests**

In `crates/amlich-tui/src/events.rs` add key-handling tests for:
- `tab_moves_panel_focus()`
- `enter_toggles_focused_section()`
- `char_e_toggles_evidence_visibility()`
- `char_z_toggles_zoom_for_current_section()`
- `char_a_expands_recommendation_section()`

If event tests are awkward at the crossterm boundary, factor the key dispatch into a pure helper and test that helper directly.

**Step 2: Run tests to verify failure**

Run: `cargo test -p amlich-tui events`
Expected: FAIL because the new keybindings do not exist.

**Step 3: Write minimal implementation**

Make only these changes:
- keep existing navigation/search/calendar controls working
- map `Tab`, `Enter`, `e`, `z`, and `a` to the new state transitions
- preserve current `t`, `h/l`, `j/k`, `/`, `c`, and quit behavior
- do not add any extra modes beyond what the design calls for

**Step 4: Run tests to verify pass**

Run: `cargo test -p amlich-tui events`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/amlich-tui/src/events.rs crates/amlich-tui/src/state.rs
git commit -m "feat(tui): add actionability-first panel controls"
```

---

### Task 3: Add presentation-oriented recommendation and verdict helpers without moving core logic into widgets

**Files:**
- Modify: `crates/amlich-api/src/dto.rs`
- Modify: `crates/amlich-api/src/convert.rs` (only if DTO additions need population)
- Modify: `crates/amlich-tui/src/state.rs`
- Test: `crates/amlich-api/src/dto.rs`
- Test: `crates/amlich-tui/src/state.rs`
- Read for context: `crates/amlich-tui/src/widgets/guidance.rs`
- Read for context: `docs/almanac/recommendation-tui-spec.md`

**Step 1: Write the failing tests**

In `crates/amlich-tui/src/state.rs`, add tests for view-model helpers such as:
- `top_recommendation_rows_follow_bucket_order()`
- `hero_verdict_prefers_summary_and_strongest_rows()`
- `risk_summary_surfaces_ky_manh_and_taboos_first()`

If you need richer structured fields in DTOs, first add API-side tests in `crates/amlich-api/src/dto.rs` (or a nearby test module) for any new presentation DTOs, ensuring stable serialization and optionality.

**Step 2: Run tests to verify failure**

Run: `cargo test -p amlich-tui state`
Run: `cargo test -p amlich-api dto`
Expected: FAIL because the helper layer / DTO fields are not present.

**Step 3: Write minimal implementation**

Implement only presentation-support data:
- add pure helper methods on `AppState` or small local view-model structs for hero verdict, top rows, and risk summaries
- if the TUI truly lacks structured inputs, add the smallest DTO extensions needed in `amlich-api`
- do not encode new recommendation policy in the TUI
- do not add new astrology logic in `convert.rs`

**Step 4: Run tests to verify pass**

Run: `cargo test -p amlich-tui state`
Run: `cargo test -p amlich-api dto`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/amlich-api/src/dto.rs crates/amlich-api/src/convert.rs crates/amlich-tui/src/state.rs
git commit -m "feat(tui): add recommendation view-model helpers"
```

---

### Task 4: Rebuild the hero section as a decision-support cover block

**Files:**
- Modify: `crates/amlich-tui/src/widgets/hero.rs`
- Modify: `crates/amlich-tui/src/widgets/page.rs`
- Test: `crates/amlich-tui/src/widgets/hero.rs`
- Read for context: `crates/amlich-tui/src/widgets/guidance.rs`

**Step 1: Write the failing tests**

Add widget-level tests for:
- `hero_shows_solar_lunar_and_summary_verdict()`
- `hero_includes_key_identity_facts()`
- `hero_handles_missing_optional_badges()`

Test for the presence of the intended strings / lines, not exact colors.

**Step 2: Run tests to verify failure**

Run: `cargo test -p amlich-tui hero`
Expected: FAIL because the current hero is too shallow and does not render the richer cover-block content.

**Step 3: Write minimal implementation**

Update `HeroWidget` to render:
- solar date and weekday
- lunar date
- recommendation summary
- strongest recommendation/risk chips
- compact identity row with can-chi / trực / tiết khí / holiday badge where available

Keep rendering Vietnamese-only. Do not add unrelated decorative output.

**Step 4: Run tests to verify pass**

Run: `cargo test -p amlich-tui hero`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/amlich-tui/src/widgets/hero.rs crates/amlich-tui/src/widgets/page.rs
git commit -m "feat(tui): redesign hero as actionability cover block"
```

---

### Task 5: Turn recommendations into the dominant on-page section

**Files:**
- Modify: `crates/amlich-tui/src/widgets/guidance.rs`
- Modify: `crates/amlich-tui/src/widgets/page.rs`
- Test: `crates/amlich-tui/src/widgets/guidance.rs`
- Read for context: `docs/almanac/recommendation-tui-spec.md`

**Step 1: Write the failing tests**

Extend `crates/amlich-tui/src/widgets/guidance.rs` tests to cover:
- `collapsed_render_keeps_bucket_order_and_counts()`
- `expanded_render_shows_all_rows_for_focused_section()`
- `evidence_toggle_hides_and_shows_reason_chips()`
- `primary_rows_are_visually_marked_first_per_bucket()`

**Step 2: Run tests to verify failure**

Run: `cargo test -p amlich-tui guidance`
Expected: FAIL because the widget does not yet support the new focus/expand/evidence behaviors and more prominent rendering.

**Step 3: Write minimal implementation**

Update `GuidanceWidget` so it becomes the main decision panel:
- keep existing bucket order and row semantics from the recommendation spec
- support focused-section expansion rather than only global collapse/expand behavior
- obey the new evidence visibility toggle
- improve section prominence and spacing without changing recommendation semantics

**Step 4: Run tests to verify pass**

Run: `cargo test -p amlich-tui guidance`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/amlich-tui/src/widgets/guidance.rs crates/amlich-tui/src/widgets/page.rs
git commit -m "feat(tui): promote recommendations into primary day section"
```

---

### Task 6: Replace the hour strip with a richer timing/action window section

**Files:**
- Modify: `crates/amlich-tui/src/widgets/timeline.rs`
- Modify: `crates/amlich-tui/src/widgets/page.rs`
- Test: `crates/amlich-tui/src/widgets/timeline.rs`
- Read for context: `crates/amlich-api/src/dto.rs`

**Step 1: Write the failing tests**

Add tests for:
- `timeline_surfaces_top_good_windows()`
- `timeline_renders_visual_distribution_for_medium_and_large_modes()`
- `timeline_falls_back_to_compact_text_on_small_mode()`
- `timeline_handles_absent_hour_data_gracefully()`

**Step 2: Run tests to verify failure**

Run: `cargo test -p amlich-tui timeline`
Expected: FAIL because the current widget is too thin and does not frame the data as action windows.

**Step 3: Write minimal implementation**

Enhance `TimelineWidget` to show:
- top good-hour ranges
- a compact action-window summary sentence
- the existing visual distribution in medium/large mode or a refined equivalent
- no new rules; only better presentation of `gio_hoang_dao`

**Step 4: Run tests to verify pass**

Run: `cargo test -p amlich-tui timeline`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/amlich-tui/src/widgets/timeline.rs crates/amlich-tui/src/widgets/page.rs
git commit -m "feat(tui): add richer timing windows section"
```

---

### Task 7: Promote travel, clashes, and taboo signals into explicit risk/direction sections

**Files:**
- Create: `crates/amlich-tui/src/widgets/travel.rs`
- Create: `crates/amlich-tui/src/widgets/risk.rs`
- Modify: `crates/amlich-tui/src/widgets/mod.rs`
- Modify: `crates/amlich-tui/src/widgets/page.rs`
- Test: `crates/amlich-tui/src/widgets/travel.rs`
- Test: `crates/amlich-tui/src/widgets/risk.rs`
- Read for context: `crates/amlich-tui/src/widgets/scholarly.rs`

**Step 1: Write the failing tests**

Create tests for the new widgets:
- `travel_widget_shows_hy_than_tai_than_and_xuat_hanh()`
- `risk_widget_prioritizes_ky_manh_taboos_and_major_clashes()`
- `risk_widget_shows_sensitive_domain_note_when_needed()`
- `widgets_render_empty_state_when_fortune_data_is_missing()`

**Step 2: Run tests to verify failure**

Run: `cargo test -p amlich-tui travel`
Run: `cargo test -p amlich-tui risk`
Expected: FAIL because the widgets do not exist.

**Step 3: Write minimal implementation**

Create two focused widgets:
- `travel.rs` for xuất hành / Hỷ Thần / Tài Thần / directional guidance
- `risk.rs` for taboos, xung-hợp highlights, and caution summaries

Reuse existing `bundle.day_fortune`, `bundle.daily_recommendations`, and helper methods from state. Do not duplicate scholarly rendering logic.

**Step 4: Run tests to verify pass**

Run: `cargo test -p amlich-tui travel`
Run: `cargo test -p amlich-tui risk`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/amlich-tui/src/widgets/travel.rs crates/amlich-tui/src/widgets/risk.rs crates/amlich-tui/src/widgets/mod.rs crates/amlich-tui/src/widgets/page.rs
git commit -m "feat(tui): add travel and risk day sections"
```

---

### Task 8: Recast scholarly content as expandable traditional evidence, not an alternate primary screen

**Files:**
- Modify: `crates/amlich-tui/src/widgets/scholarly.rs`
- Modify: `crates/amlich-tui/src/widgets/tietkhi.rs`
- Modify: `crates/amlich-tui/src/widgets/page.rs`
- Test: `crates/amlich-tui/src/widgets/scholarly.rs`
- Test: `crates/amlich-tui/src/widgets/tietkhi.rs`

**Step 1: Write the failing tests**

Add tests for:
- `scholarly_widget_groups_truc_stars_and_deity_as_evidence()`
- `tietkhi_widget_collapses_to_summary_and_expands_details()`
- `evidence_sections_respect_focus_and_zoom_flags()`

**Step 2: Run tests to verify failure**

Run: `cargo test -p amlich-tui scholarly`
Run: `cargo test -p amlich-tui tietkhi`
Expected: FAIL because these widgets currently behave more like independent panels than secondary evidence sections.

**Step 3: Write minimal implementation**

Refactor the lower-page sections so they read as “why the day looks this way”:
- scholarly block becomes a traditional evidence section
- tiết khí remains expandable but fits the same evidence hierarchy
- keep data sourcing from `bundle.insight` / `bundle.day_fortune`
- do not remove useful existing fields; just reframe and reorganize them

**Step 4: Run tests to verify pass**

Run: `cargo test -p amlich-tui scholarly`
Run: `cargo test -p amlich-tui tietkhi`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/amlich-tui/src/widgets/scholarly.rs crates/amlich-tui/src/widgets/tietkhi.rs crates/amlich-tui/src/widgets/page.rs
git commit -m "refactor(tui): reorganize traditional evidence sections"
```

---

### Task 9: Update ribbon/help copy and responsive layout behavior to match the new mental model

**Files:**
- Modify: `crates/amlich-tui/src/layout.rs`
- Modify: `crates/amlich-tui/src/widgets/ribbon.rs`
- Modify: `crates/amlich-tui/src/widgets/page.rs`
- Test: `crates/amlich-tui/src/layout.rs`
- Test: `crates/amlich-tui/src/widgets/ribbon.rs`

**Step 1: Write the failing tests**

Add tests for:
- `layout_modes_preserve_actionability_first_order()`
- `large_layout_uses_internal_density_without_calendar_dominance()`
- `ribbon_shows_new_focus_expand_evidence_controls()`

**Step 2: Run tests to verify failure**

Run: `cargo test -p amlich-tui layout`
Run: `cargo test -p amlich-tui ribbon`
Expected: FAIL because the help/ribbon and layout assumptions still reflect the older lens-centric page.

**Step 3: Write minimal implementation**

Update only what is necessary:
- keep the scroll/page layout approach
- adjust width handling so large screens allow denser internal sections without reverting to a calendar-dominant UI
- refresh ribbon copy to teach `Tab`, `Enter`, `a`, `e`, and `z`
- preserve calendar mode as a utility path, not the default mental model

**Step 4: Run tests to verify pass**

Run: `cargo test -p amlich-tui layout`
Run: `cargo test -p amlich-tui ribbon`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/amlich-tui/src/layout.rs crates/amlich-tui/src/widgets/ribbon.rs crates/amlich-tui/src/widgets/page.rs
git commit -m "refactor(tui): align layout and ribbon with actionability-first flow"
```

---

### Task 10: Verify the end-to-end TUI surface from API contract to CLI launch

**Files:**
- Modify only if verification exposes real gaps: `crates/amlich/src/main.rs`
- Read for context: `crates/amlich-tui/src/lib.rs`
- Read for context: `crates/amlich/src/main.rs`
- Optional docs update if controls changed materially: `docs/plans/2026-03-12-actionability-first-tui-design.md`

**Step 1: Run targeted crate tests first**

Run:
```bash
cargo test -p amlich-api
cargo test -p amlich-tui
```
Expected: PASS

**Step 2: Run CLI integration checks**

Run:
```bash
cargo test -p amlich-cli
cargo run -p amlich-cli -- tui --date 2026-03-12
```
Expected:
- tests pass
- TUI launches from the CLI package, not from `amlich-tui` directly
- the main screen shows the new actionability-first ordering

**Step 3: Fix only real integration issues**

If verification exposes a launcher or contract issue:
- make the smallest fix in `crates/amlich/src/main.rs` or the affected crate
- do not broaden scope into unrelated CLI redesign work

**Step 4: Run final verification again**

Run:
```bash
cargo test -p amlich-api
cargo test -p amlich-tui
cargo test -p amlich-cli
```
Expected: PASS

**Step 5: Commit**

```bash
git add crates/amlich/src/main.rs crates/amlich-tui/src/lib.rs docs/plans/2026-03-12-actionability-first-tui-design.md
git commit -m "test: verify actionability-first tui integration"
```

---

## Final Verification

Run all relevant checks before claiming completion:

```bash
cargo test -p amlich-api
cargo test -p amlich-tui
cargo test -p amlich-cli
cargo run -p amlich-cli -- tui --date 2026-03-12
```

If the full API test suite is too broad, use the narrowest stable targets that still cover any changed DTO/contract surface and record the exact commands used.

## Notes for the implementing agent

- Keep the redesign focused on **presentation of existing data**, not new divination logic.
- Do not move recommendation or astrology policy into widgets.
- Avoid large file rewrites when a sequence of focused widget updates will do.
- Prefer adding small view-model helpers in `AppState` over embedding selection logic directly into rendering code.
- Keep Vietnamese-first labels and copy for this iteration.
- If a genuinely separate follow-up is discovered, create a bead with `bd create ... --json` instead of leaving markdown TODOs.
- Do not commit or push unless the user explicitly asks for it.
