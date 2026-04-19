# Cross-Screen Widget And File Migration Map

**Date:** 2026-03-19
**Status:** Proposed
**Issue:** `amlich-0c7`
**Companion Docs:**
- [Scholar Consultation Redesign Plan](./2026-03-19-scholar-consultation-redesign-plan.md)
- [Cross-Screen Consultation IA Plan](./2026-03-19-cross-screen-consultation-ia-plan.md)

**Goal:** Map the current `amlich-tui` screen/widget structure to the new consultation-style 5-screen IA, identifying which files should be kept, merged, split, replaced, or retired, plus the view-model/helper work required in `AppState`.

## Executive Summary

The current codebase already has the right broad pieces, but they are assembled with the wrong mental model.

Today, the screen layer is organized like this:

- one screen per top-level view
- each screen composed of equal-weight panels or subsystem grids
- several widgets duplicate overlapping ownership
- the strongest recommendation view-model helpers already exist, but are not the default input for the `Scholar` screen

The migration should therefore avoid rewriting everything from scratch.

Instead:

- keep the current top-level `ActiveView` routing model
- preserve most screen file names
- preserve most subsystem widget files as temporary data/rendering units
- rewrite ownership and composition around the new IA
- add a small set of consultation-oriented summary widgets and state helpers

## Current File Topology

### Existing Top-Level Screens

Current screen entrypoints:

- `crates/amlich-tui/src/widgets/screens/insight.rs`
- `crates/amlich-tui/src/widgets/screens/hours.rs`
- `crates/amlich-tui/src/widgets/screens/elements.rs`
- `crates/amlich-tui/src/widgets/screens/feng_shui.rs`
- `crates/amlich-tui/src/widgets/screens/solar_terms.rs`

These already line up with the desired 5-screen model:

- `Scholar`
- `Giờ Tốt`
- `Ngũ Hành`
- `Phong Thủy`
- `Tiết Khí`

So the redesign is **not** a router problem. It is primarily a screen-composition and ownership problem.

### Existing Reusable Widgets

Relevant current widgets:

- `guidance_panel.rs`
- `guidance.rs`
- `risk.rs`
- `direction_panel.rs`
- `timeline.rs`
- `travel.rs`
- `scholarly.rs`
- `naam_panel.rs`
- `stars_panel.rs`
- `tietkhi.rs`
- `hero.rs`
- `inspection.rs`

### Existing State/View-Model Helpers

Relevant `AppState` helpers already present in `crates/amlich-tui/src/state.rs`:

- `recommendation_layers()`
- `top_recommendation_rows()`
- `hero_verdict()`
- `risk_summary()`
- `active_pack_summary()`
- `active_bundle_packs_summary()`
- `show_guidance_details`
- `show_tietkhi_details`
- `show_evidence`

This is an important constraint: the redesign should reuse these helpers where possible before inventing new state.

## Migration Strategy

### Principle 1: Keep Screen Files, Change Their Job

Do not rename or delete the five screen entrypoints first.

Instead:

- keep `insight.rs` as the `Scholar` screen entrypoint
- keep `hours.rs` as the `Giờ Tốt` screen entrypoint
- keep `elements.rs` as the `Ngũ Hành` screen entrypoint
- keep `feng_shui.rs` as the `Phong Thủy` screen entrypoint
- keep `solar_terms.rs` as the `Tiết Khí` screen entrypoint

This minimizes routing churn and lets the redesign happen one screen at a time.

### Principle 2: Replace Equal-Weight Grids With Sectioned Reading Flow

The current grid-heavy approach should be retired in favor of:

- top verdict block
- dominant task block
- support sections below
- evidence or drill-down at the bottom

### Principle 3: Preserve Useful Widgets As Sources, Not Final UX

Some current widgets are good subsystem renderers but poor final surfaces.

They should be reused temporarily, then either:

- merged into new consultation sections
- narrowed into deeper drill-down widgets
- retired once their content is absorbed elsewhere

## File-Level Migration Map

## 1. `Scholar` Screen

### Screen Entry

**File:** `crates/amlich-tui/src/widgets/screens/insight.rs`

**Current role:**
- renders a 6-panel equal-weight grid (`Large`) or equivalent stacked layout (`Medium`/`Small`)

**Future role:**
- render the consultation-style `Scholar` document:
  1. hero verdict
  2. recommendation board
  3. risk board
  4. application row (direction + timing)
  5. interpretation row (`Khí Ngày` + `Sao & Trực`)
  6. evidence drawer

**Action:** `replace composition, keep file`

### Recommendation Section

**Current files:**
- `crates/amlich-tui/src/widgets/guidance_panel.rs`
- `crates/amlich-tui/src/widgets/guidance.rs`

**Current role:**
- `guidance_panel.rs` is a shallow `day_guidance`/`truc` panel
- `guidance.rs` is already a richer recommendation-engine widget used elsewhere

**Future role:**
- `guidance.rs` becomes the basis for the dominant `Scholar` action section
- `guidance_panel.rs` should either be:
  - replaced by a thin wrapper around the richer recommendation widget, or
  - retired after migration

**Action:** `replace shallow panel with richer widget`

**Recommendation:**
- keep `guidance.rs`
- retire or hollow out `guidance_panel.rs`

### Risk Section

**Current file:** `crates/amlich-tui/src/widgets/risk.rs`

**Current role:**
- compact summary panel using `risk_summary()` and some raw taboo/conflict fields

**Future role:**
- first-class caution board in `Scholar`
- may also provide a reusable `RiskBoardWidget`

**Action:** `keep file, expand responsibility`

**Notes:**
- do not fork risk logic into multiple widgets unless necessary
- enrich from recommendation evidence and taboo severity rather than adding new domain logic

### Direction Section

**Current file:** `crates/amlich-tui/src/widgets/direction_panel.rs`

**Current role:**
- compact panel for `xuất hành`, `Tài Thần`, `Hỷ Thần`, deity meaning

**Future role:**
- reusable day-level direction widget for:
  - `Scholar` application row
  - possibly `Phong Thủy` non-profile mode

**Action:** `keep file, narrow ownership to day-level directional advice`

### Timing Section

**Current files:**
- `crates/amlich-tui/src/widgets/screens/hours.rs`
- `crates/amlich-tui/src/widgets/timeline.rs`

**Current role:**
- `hours.rs` is a standalone screen with timeline + detail columns
- `timeline.rs` contains timing display logic elsewhere in the app

**Future role in `Scholar`:**
- a compact timing summary widget, not the whole Hours screen

**Action:** `split responsibility`

**Plan:**
- keep `hours.rs` for the dedicated tab
- extract or add a smaller `ActionWindowSummaryWidget` or reuse `timeline.rs` for `Scholar`
- do not embed the full hours screen inside `Scholar`

### Day Identity / Element Reading

**Current files:**
- `crates/amlich-tui/src/widgets/scholarly.rs`
- `crates/amlich-tui/src/widgets/naam_panel.rs`

**Current role:**
- two overlapping panels: mixed day identity + nạp âm / ngũ hành details

**Future role:**
- merge conceptually into one `Khí Ngày` section in `Scholar`

**Action:** `merge ownership, likely keep both files temporarily`

**Recommended path:**
- first, rewrite `scholarly.rs` to own high-level day identity
- then reduce `naam_panel.rs` to supporting detail or absorb it fully
- eventual outcome may be:
  - one merged widget file, or
  - one primary widget plus one embedded subrenderer

### Traditional Evidence

**Current file:** `crates/amlich-tui/src/widgets/stars_panel.rs`

**Current role:**
- `Trực`, day star, cát tinh, sát tinh, deity duplication

**Future role:**
- `Sao & Trực` evidence section in `Scholar`

**Action:** `keep file, refocus`

**Notes:**
- remove duplicated deity coverage that belongs to direction/day-deity context
- focus on `Trực`, day star, top cát/hung stars, and provenance

### Evidence Drawer

**Current files:**
- no dedicated file yet
- partial evidence toggles exist in `state.rs`
- inspection-style renderers exist in `inspection.rs`

**Future role:**
- compact expandable evidence section for `Scholar`

**Action:** `new widget`

**Recommended file:**
- `crates/amlich-tui/src/widgets/evidence_drawer.rs`

**Potential reuse:**
- `inspection.rs` may provide some evidence rendering patterns or chips

## 2. `Giờ Tốt` Screen

### Screen Entry

**File:** `crates/amlich-tui/src/widgets/screens/hours.rs`

**Current role:**
- renders timeline + good/bad hour lists

**Future role:**
- remain the `Giờ Tốt` screen entrypoint, but evolve from raw timeline/detail into an execution-timing screen

**Action:** `keep file, redesign content hierarchy`

### Keep / Reuse

**Keep:**
- existing timeline overview structure
- hour list rendering logic

**Add / Change:**
- top `Nhận định` block
- “best action windows” summary
- small “when whole-day vs hour-level quality conflict” note

### Supporting Widgets

**Files:**
- `timeline.rs`
- optionally new `action_windows.rs`

**Action:**
- keep `timeline.rs`
- consider extracting action-window synthesis display into a new helper widget instead of bloating `hours.rs`

## 3. `Ngũ Hành` Screen

### Screen Entry

**File:** `crates/amlich-tui/src/widgets/screens/elements.rs`

**Current role:**
- 3x2 subsystem grid: `Tàng Can`, `Thập Thần`, `Xung Hợp`, element relations, pillars, chart

**Future role:**
- interpretive `Ngũ Hành` screen with controlled depth

**Action:** `keep file, change emphasis`

### Current Subrenderers Inside `elements.rs`

Embedded render functions:
- `render_tang_can`
- `render_ten_gods`
- `render_xung_hop`
- `render_element_relations`
- `render_pillars`
- `render_element_chart`

**Assessment:**
- useful content blocks
- currently too subsystem-equal and dashboard-like

**Future treatment:**
- keep these renderers as internal detail blocks or extract later
- add a top-level `Khí Ngày` verdict block before them
- do not let all six blocks remain equal by default

**Action:** `keep content, reorder hierarchy`

### Potential File Split Later

If `elements.rs` grows too large:

- extract `element_identity.rs`
- extract `tang_can.rs`
- extract `ten_gods.rs`
- extract `xung_hop.rs`

But this is optional. First move should be IA, not file explosion.

## 4. `Phong Thủy` Screen

### Screen Entry

**File:** `crates/amlich-tui/src/widgets/screens/feng_shui.rs`

**Current role:**
- profile-dependent `Tứ Mệnh / Kua`, direction lists, `Đại Vận`, compass

**Future role:**
- adaptive screen with two paths:
  - no-profile mode: restrained day-level directional and scope-explaining view
  - profile mode: richer personal overlay

**Action:** `keep file, split render paths explicitly`

### Current Subrenderers

Embedded render functions:
- `render_kua`
- `render_directions`
- `render_dai_van`
- `render_compass`

**Assessment:**
- good deep widgets for profile mode
- weak non-profile experience today (`Chưa cấu hình hồ sơ cá nhân`)

**Future treatment:**
- keep profile widgets
- add a non-profile day-level section using day direction data from bundle
- likely reuse `direction_panel.rs` in no-profile mode

**Action:** `merge general-day directional widget with profile-only detail widgets`

### Recommended Supporting Files

**Keep:**
- `direction_panel.rs` for day-level direction
- `feng_shui.rs` for orchestration

**Optional new file:**
- `personal_feng_shui.rs` if profile-specific rendering becomes too large

## 5. `Tiết Khí` Screen

### Screen Entry

**File:** `crates/amlich-tui/src/widgets/screens/solar_terms.rs`

**Current role:**
- 2x2 grid for current term, astronomy/weather, agriculture, health

**Future role:**
- contextual seasonal screen with stronger top-level interpretation

**Action:** `keep file, change reading order`

### Current Subrenderers

Embedded render functions:
- `render_current`
- `render_astronomy`
- `render_agriculture`
- `render_health`

**Assessment:**
- content inventory is already correct
- current composition is still subsystem-grid oriented

**Future treatment:**
- keep current content blocks
- add top `Nhận định` and `seasonal implication` block
- move raw detail lower

**Action:** `keep content, reduce equal-weight presentation`

### Supporting Widget Reuse

**Current file:** `crates/amlich-tui/src/widgets/tietkhi.rs`

**Assessment:**
- may already contain useful recommendation/context patterns
- inspect during implementation for reuse before duplicating seasonal logic

## Shared File Decisions

### `crates/amlich-tui/src/state.rs`

**Status:** central to migration

**Action:** `extend`

Needed additions likely include:

- `hours_verdict()` or `action_window_summary()`
- `day_identity_summary()`
- `traditional_evidence_summary()`
- `direction_verdict()`
- `seasonal_verdict()`
- profile-availability helper for `Phong Thủy`
- possibly per-screen expanded detail toggles if current global toggles prove too coarse

Current helpers already reusable:

- `hero_verdict()`
- `top_recommendation_rows()`
- `recommendation_layers()`
- `risk_summary()`

### `crates/amlich-tui/src/widgets/hero.rs`

**Status:** candidate for reuse

**Action:** `reuse or adapt for Scholar top block`

If its current content is too dashboard-oriented, keep the file and repurpose it rather than creating another hero widget first.

### `crates/amlich-tui/src/widgets/inspection.rs`

**Status:** potential evidence rendering source

**Action:** `reuse patterns, not necessarily surface`

Good candidate for:
- evidence chips
- reason breakdown patterns
- baseline/contextual comparison rendering

### `crates/amlich-tui/src/widgets/travel.rs`

**Status:** likely partial overlap with day-direction concerns

**Action:** `review during implementation`

Potential outcomes:
- reuse in `Scholar` application row or `Phong Thủy` non-profile mode
- absorb pieces into `direction_panel.rs`
- retire if redundant

## Retire / Reduce Candidates

The following files are likely not end-state surfaces in their current form:

- `guidance_panel.rs`
  - too shallow compared to `guidance.rs`
- `naam_panel.rs`
  - likely absorbed into `Khí Ngày`
- equal-grid composition inside `insight.rs`
  - definitely retired

These should not necessarily be deleted immediately, but they should lose ownership as soon as replacement sections exist.

## New File Recommendations

The redesign probably needs a few new files, but only where composition requires them.

Recommended additions:

- `crates/amlich-tui/src/widgets/evidence_drawer.rs`
  - shared evidence panel for `Scholar`
- `crates/amlich-tui/src/widgets/action_window_summary.rs`
  - compact timing summary for `Scholar`
- `crates/amlich-tui/src/widgets/day_identity.rs`
  - only if `scholarly.rs` + `naam_panel.rs` merge becomes too awkward
- `crates/amlich-tui/src/widgets/seasonal_verdict.rs`
  - only if `solar_terms.rs` top block should be reusable elsewhere

These should be added only after proving the composition cannot stay clean with the current files.

## Screen-By-Screen Migration Table

| Screen | Entry file | Keep | Merge | Replace | Retire/Reduce |
|---|---|---|---|---|---|
| `Scholar` | `screens/insight.rs` | `risk.rs`, `direction_panel.rs`, `stars_panel.rs`, `guidance.rs` | `scholarly.rs` + `naam_panel.rs` | equal 6-panel grid, shallow guidance panel path | `guidance_panel.rs`, raw split identity panels |
| `Giờ Tốt` | `screens/hours.rs` | `hours.rs`, `timeline.rs` | maybe timing summary into new compact widget | raw list-first presentation | none immediately |
| `Ngũ Hành` | `screens/elements.rs` | `elements.rs` internal renderers | maybe extract later only if too large | equal-weight 3x2 subsystem feel | none immediately |
| `Phong Thủy` | `screens/feng_shui.rs` | `render_kua`, `render_directions`, `render_dai_van`, `render_compass`, `direction_panel.rs` | general-day direction + profile-specific widgets | profile-less blank state | none immediately |
| `Tiết Khí` | `screens/solar_terms.rs` | current content blocks | seasonal verdict + existing sections | equal-weight 2x2 grid feel | none immediately |

## Recommended Implementation Order

### Phase 1: `Scholar` first

Files:
- `screens/insight.rs`
- `guidance.rs`
- `risk.rs`
- `scholarly.rs`
- `naam_panel.rs`
- `stars_panel.rs`
- `direction_panel.rs`
- `state.rs`

Why first:
- highest user-value screen
- strongest overlap with existing recommendation helpers
- defines the consultation language for the rest

### Phase 2: `Giờ Tốt`

Files:
- `screens/hours.rs`
- `timeline.rs`
- `state.rs`

Why second:
- strong practical companion to `Scholar`
- low routing risk
- likely smaller than `Ngũ Hành`

### Phase 3: `Ngũ Hành`

Files:
- `screens/elements.rs`
- maybe later extracted subwidgets
- `state.rs`

Why third:
- interpretive screen depends on the consultation voice established in `Scholar`

### Phase 4: `Phong Thủy`

Files:
- `screens/feng_shui.rs`
- `direction_panel.rs`
- `state.rs`

Why fourth:
- requires clearer policy around profile vs non-profile rendering

### Phase 5: `Tiết Khí`

Files:
- `screens/solar_terms.rs`
- possibly `tietkhi.rs`
- `state.rs`

Why fifth:
- already has a coherent content inventory
- mostly a hierarchy and copy problem rather than a missing-widget problem

## Acceptance Criteria

This migration map is successful if implementation can follow it without ambiguity about:

1. which files remain screen entrypoints
2. which existing widgets are temporary sources vs end-state surfaces
3. which overlaps should be resolved by merge rather than duplication
4. which new helper/state work belongs in `AppState`
5. which files are safe to defer or retire later
6. what order minimizes churn while preserving the `core -> api -> tui` boundary

## Recommendation

Proceed with the redesign by **recomposing the existing screen files** rather than renaming the whole screen layer.

Treat current widgets as follows:

- keep the ones that already render strong subsystem content
- merge overlapping identity widgets
- replace shallow recommendation surfaces with the richer recommendation widget path
- add only a few new summary widgets where the new IA truly needs them

This is the lowest-risk path to evolving the app from subsystem dashboarding into a real consultation-style almanac UI.
