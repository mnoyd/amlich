# Actionability-First Am Lich TUI Design

**Date:** 2026-03-12
**Status:** Approved

## Problem

The current `amlich` TUI exposes only a small slice of the information already available through `amlich-core` and `amlich-api`. It works as a lightweight date viewer, but it does not feel like a rich Vietnamese almanac. The user experience is too simple, too sparse, and too timid about surfacing the system’s strongest data: structured recommendations, supporting evidence, auspicious timing, travel guidance, day conflicts, stars, deities, taboos, and deeper interpretive context.

The immediate need is not to invent new rule systems. It is to redesign the TUI presentation layer so it can present much more of the already-available information in a dense but readable way.

## User Intent

The validated product direction for the next TUI is:

- **Dense by default** rather than minimalist
- **Single selected day** as the primary canvas
- **Decision-support first** rather than calendar-first or lore-first
- **Actionability-first** organization on the home screen
- **Vietnamese-only for now**
- **Stacked priority layout** instead of a tab-heavy or equal-weight dashboard

This means the first screen should answer, in order:

1. What is the character of today?
2. What should I do today?
3. What should I avoid today?
4. When and in which direction should I act?
5. What traditional evidence supports those recommendations?
6. Where can I drill down if I want the full almanac context?

## Recommended Approach

### Option A — Mega-dashboard with many equal panels

Show many boxes simultaneously: recommendation buckets, giờ tốt, xuất hành, can-chi, trực, sao, thần sát, tiết khí, calendar, holiday, and profile-related data all at once.

**Pros:** feels rich immediately, high information density, impressive on large terminals.

**Cons:** weak hierarchy, visually noisy, hard to scan, breaks down badly on medium terminals, and makes recommendation priority less clear.

### Option B — Actionability-first stacked command center **(recommended)**

Use a vertically stacked single-day page with strong ordering. Top sections are recommendation-oriented and highly scannable. Lower sections progressively expose timing, risks, and traditional evidence. Richness comes from depth and section quality, not from cramming every concept into equal-width boxes.

**Pros:** keeps decision-support obvious, scales down better, makes room for more prose and evidence, works naturally with scrolling, and still allows dense content.

**Cons:** some secondary data moves below the fold; large-screen horizontal density is lower than a pure dashboard.

### Option C — Lens/tab-first interface

Make the top-level interaction about switching modes such as `Chung`, `Hành sự`, `Học thuật`, and `Cá nhân`, with each lens showing different sections.

**Pros:** very powerful long-term model, good for future personalization.

**Cons:** hides information behind mode switches, adds state complexity, and is not the right first move when the main complaint is “the TUI shows too little.”

### Why Option B wins

It best matches the stated preference: rich, serious, and “fancy,” but still primarily useful for deciding what to do with the day. It also composes well with the existing `amlich-tui` architecture, where widgets can stay presentation-only while the API surface grows to supply richer structured data.

## Information Architecture

The main screen should be a single scrollable day document with this fixed top-to-bottom hierarchy:

### 1. Hero Summary

The hero is a dense summary block that establishes the day immediately:

- solar date and weekday
- lunar date
- concise recommendation summary (`daily_recommendations.summary_vi`)
- standout verdict chips such as strongest `Nên`, strongest `Tránh`, strongest `Kỵ mạnh`
- compact identity facts: can-chi day, trực, tiết khí, optional festival/holiday badge

This section should feel like the “cover page” of the day.

### 2. Hôm Nay Nên Làm Gì

Primary recommendation section, visually dominant.

Content:

- `Nên`
- `Có thể`
- `Tránh`
- `Kỵ mạnh`

Rendering principles:

- preserve recommendation bucket ordering already defined in the TUI spec
- keep strongest item visually emphasized
- show evidence chips inline when provenance is enabled
- make collapsed/expanded behavior section-aware rather than using overlays as the primary path

This is the single most important section in the redesign.

### 3. Khung Giờ Và Hành Động

A timing section that converts raw auspicious-hour data into usable action support.

Content:

- top good hours
- compact visual timeline of good/bad hour distribution across the day
- recommended “best windows” for acting
- optional relationship between recommendation intensity and timing windows

This section should answer: “If I do it today, when should I do it?”

### 4. Xuất Hành Và Hướng

Dedicated directional guidance section.

Content:

- `xuat_hanh_huong`
- `tai_than`
- `hy_than`
- compact warning if direction-related evidence is weak or absent

This should be promoted from obscure detail into a visible decision-support block.

### 5. Rủi Ro, Xung, Kiêng Kỵ

A high-signal caution section.

Content:

- key taboos
- `xung_hop` highlights (`lục xung`, `tam hợp`, major clashes)
- day deity or other strong blockers when relevant
- sensitive-domain notices such as medical/burial disclaimers when recommendation activities indicate them

This section should make risks impossible to miss.

### 6. Chứng Cứ Truyền Thống

A deeper evidence section for users who want the “why.”

Content groups:

- can-chi and ngũ hành / nạp âm
- trực
- stars (`cat_tinh`, `sat_tinh`, `day_star`)
- thần sát / day deity
- tiết khí
- ten gods / tàng can when present

This area should remain compact by default but expandable.

### 7. Chi Tiết Mở Rộng

Long-form or secondary content.

Potential content:

- festival/holiday detail
- insight text
- evidence/provenance breakdown
- optional future personal sections when profile-aware surfaces are richer

## Interaction Model

The interaction model should support a dense reading surface, not a modal maze.

### Keep

- `h/l` or equivalent day navigation
- `j/k` scrolling
- today jump
- date jump
- search/help as existing affordances

### Promote

- `Tab` to move panel focus between major sections
- `Enter` to expand/collapse the focused section
- `a` to expand recommendation detail quickly
- `e` to toggle provenance/evidence chips on the page
- `z` to focus the current panel in an isolated “zoom” mode

### De-emphasize

- overlays as the default path to core information
- forcing users into separate popups for things that belong in the main reading flow

## Screen Structure by Width

### Small terminals

Still single-column, but dense. The order above remains unchanged. Sections collapse aggressively and evidence chips default off if space is tight.

### Medium terminals

Primary reading column remains dominant. Some panels can gain sub-columns internally, for example `Nên` and `Tránh` side-by-side or recommendation/timing split rows.

### Large terminals

Still actionability-first, but richer internal compositions become available:

- recommendation columns can become two-up
- timing and direction blocks can share a row
- evidence sections can render in paired subpanels

The design should not revert to a calendar-dominant layout.

## Data and Architecture Strategy

The technical boundary remains:

`amlich-core` -> `amlich-api` DTOs -> `amlich` / `amlich-tui` presentation

Key design principle:

- **Do not move almanac logic into widgets.**
- Widgets should render and compose already-structured data.
- If the TUI needs richer content, first enrich `DayBundleDto` and related DTOs with already-available core data.

This keeps the system maintainable and lets CLI, TUI, and future surfaces share the same structured contracts.

### Likely API/TUI surface expansions

The redesign will likely need richer structured fields from `amlich-api` and/or TUI view-model helpers for:

- stronger “verdict” summaries derived from recommendations
- explicit top recommendation rows per bucket
- richer taboos and conflict summaries
- better timing visualization inputs from `gio_hoang_dao`
- grouped traditional evidence for stars, deity, trực, tiết khí, and xung-hợp
- drilldown-ready metadata so the TUI can expand details without bespoke widget logic

These are presentation-support expansions, not new astrology logic.

## Panel Design Notes

### Hero block

Must feel prestigious and useful, not decorative. It should summarize the day in about 4–8 lines and immediately communicate confidence and direction.

### Recommendation buckets

Should look like ranked action cards, not just bullet lists. Existing `guidance.rs` behavior around primary emphasis and evidence chips is a good seed, but the visual treatment should become much more central.

### Timing visualization

A simple terminal-native bar or segmented strip is preferable to prose-only hour lists. It should make good-hour clustering obvious at a glance.

### Traditional evidence

Should feel like a scholar’s appendix below the practical sections, not like unrelated noise. The user asked for “all the fancy information,” but because the home screen is actionability-first, this information should explain decisions rather than compete with them.

## Non-Goals

This redesign does **not** require:

- new researched rule families in core
- switching to English or bilingual UI now
- profile-heavy personalization as a first step
- replacing the whole navigation model with a calendar-centric workflow
- inventing confidence scores or unsupported recommendation semantics

## Testing Strategy

The redesign should be validated with focused UI and contract tests.

### UI/layout tests

- section ordering remains stable
- collapsed vs expanded behavior works per section
- small/medium/large width behavior preserves hierarchy
- empty-state rendering is graceful when optional fields are absent

### Presentation logic tests

- recommendation hero and bucket summaries are deterministic
- evidence chip visibility toggles correctly
- risk-sensitive notices appear only for relevant recommendation/activity sets
- timing summary chooses the intended top windows

### Contract tests

- richer DTO surfaces serialize stably
- optional sections stay optional
- TUI does not depend on core-only types directly

## Proposed Execution Shape

The implementation should happen in phases rather than in one giant widget rewrite:

1. define the richer IA and page skeleton
2. identify missing DTO/view-model fields
3. expand API surfaces minimally
4. build the new section widgets in recommendation-first order
5. wire interactions for expand/focus/evidence toggles
6. refine responsive behavior and tests

## Decision

Proceed with an **actionability-first, single-day, Vietnamese-only almanac command center** built as a vertically prioritized reading surface. The main page should foreground recommendations, timing, and directional guidance, while still exposing the deeper traditional evidence already present in the system through expandable lower sections.
