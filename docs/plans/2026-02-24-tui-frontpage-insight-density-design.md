# TUI Front Page Enhancement: Visual Polish + Insight Density (Large Screen)

**Date:** 2026-02-24  
**Status:** Approved  
**Scope:** `crates/amlich` TUI front page presentation, large breakpoint first (`120+` cols)

## Objective

Improve the large-screen TUI front page so it is both:

1. More polished visually (clear hierarchy, cleaner composition)
2. More information-dense (more useful insight visible without opening overlays)

## Constraints and Decisions

- Primary target is **large terminals** (`>=120` columns).
- Layout split changes from **40/60** to **35/65** (calendar/info).
- Existing keybindings may be extended if helpful.
- Use existing data from `amlich-core` / `amlich-api`; no new data sources.
- Keep deep-detail overlays, but reduce dependency on them for day-to-day use.

## Data Capabilities Confirmed

Already available for front-page usage:

- `DayInsightDto.day_guidance`: `good_for` / `avoid_for`
- `DayInsightDto.tiet_khi`: `weather`, `agriculture`, `health`, `meaning`, `astronomy`
- `DayInsightDto.festival` / `holiday`: names, significance/origin, activities/traditions, food/taboos/proverbs/regions
- `DayInfoDto.gio_hoang_dao.all_hours`: full hour ranges, star names, good/bad flags
- `get_holidays(year, false)`: enough to derive near-term holiday context

Known gap:

- No explicit day score field; verdict must be **derived** from existing guidance data.

## Large-Screen Architecture (Recommended Option 1)

### Layout

- Left column (`35%`): calendar (navigation anchor)
- Right column (`65%`): info dashboard cards in 3 rows

### Card Grid

1. **Hero row (full width)**
2. **Insight row (2 columns):**
   - Guidance card
   - Auspicious hours card
3. **Context row (2 columns):**
   - Holiday context card
   - 3-day preview card

## Card Content Model

### 1) Hero Card

- Solar date + weekday
- Lunar date
- Day Can Chi quick line
- Derived verdict badge: `Tốt` / `Trung bình` / `Cần cân nhắc`
- Keep concise: max ~2 text lines + 1 badge row

### 2) Guidance Card

- `Nên`: top 4 items
- `Tránh`: top 4 items
- Truncation hint when more items exist (`Enter` opens full guidance view)
- Balanced presentation so both positive/negative guidance remain visible

### 3) Auspicious Hours Card

- First: next 3 good hours from current time
- Then up to 6 rows with `Chi (time range) · star`
- Prioritize near-future planning value

### 4) Holiday Context Card

- If today is holiday/festival: show today label + one-line significance
- Always show next 1-2 upcoming holidays with countdown days
- Keep front page planning-oriented; long cultural text stays in overlay

### 5) Mini 3-Day Preview Card

- Rows for today, +1, +2
- Each row: solar short date, lunar short date, verdict icon
- Fixed-height compact table for quick scan

## Interaction Model

Keep current core controls and add large-screen card focus:

- Existing keys retained: `hjkl`, arrows, `t`, `g`, `/`, `b`, `?`, `q`
- New large-screen-only:
  - `Tab` / `Shift+Tab`: cycle focus across right-side cards
  - `Enter`: open deep view for focused card
  - Optional direct focus keys: `1..5` for Hero/Guidance/Hours/Holiday/Preview

Card action mapping on `Enter`:

- Hero -> Insight overlay (Guidance tab default)
- Guidance -> Full guidance view
- Hours -> Hour-rich detailed view (or mapped insight view)
- Holiday -> Festival/Holiday deep details
- Preview -> Jump selected date to chosen row

## Rendering and Behavior Rules

- Keep existing breakpoints and behavior for small/medium unchanged initially.
- Only large layout is restructured in this phase.
- If `DayInsight` missing, show explicit neutral placeholders; never blank cards.
- If no near-term holiday, show neutral “no upcoming holiday nearby” message.
- If height is constrained, reduce per-card rows before dropping card blocks.
- If verdict cannot be derived, show `Chưa đủ dữ liệu`.

## Derived Verdict Rules (Initial)

Use a simple deterministic heuristic based on guidance counts:

- `good = good_for.len()`
- `avoid = avoid_for.len()`

Proposed mapping:

- `Tốt`: `good >= avoid + 2`
- `Trung bình`: difference within `[-1, +1]`
- `Cần cân nhắc`: `avoid >= good + 2`
- Fallback: `Chưa đủ dữ liệu` when guidance is absent

This keeps logic transparent and testable; tuning can come later.

## Testing Strategy

Add tests for:

- Verdict derivation thresholds
- Upcoming-holiday selection:
  - today holiday
  - near future holiday
  - cross-year boundary
- 3-day preview rollover at month/year edges
- Card line builders for truncation and placeholder states

Manual verification matrix:

- Widths: `120`, `140`
- Heights: `24`, `30`
- With/without insight data
- With/without nearby holidays

## Non-Goals

- No core calendar algorithm changes
- No API contract changes
- No new remote or external data
- No redesign of small/medium layouts in this phase

