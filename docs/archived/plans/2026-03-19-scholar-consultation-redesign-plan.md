# Scholar Consultation Redesign Plan

**Date:** 2026-03-19
**Status:** Proposed
**Issue:** `amlich-eu9`

**Goal:** Redesign the `Scholar` screen in `amlich-tui` from a flat equal-panel dashboard into a consultation-style reading flow that feels like a knowledgeable Vietnamese almanac reader, while reusing as much of the existing `amlich-core` and `amlich-api` capability surface as possible.

## Executive Summary

The current `Scholar` screen is not information-poor because the engine is weak. It is information-poor because the presentation is weak.

Today, the app already has three strong layers:

1. `Facts` from `amlich-core` (`Can Chi`, `Ngũ Hành`, `Nạp Âm`, `Trực`, `Sao`, `Thần`, `xung/hợp`, `kiêng kỵ`, `hướng`, `giờ tốt`)
2. `Interpretation` from `amlich-api` insight DTOs (`meaning`, `nature`, `good_for`, `avoid_for`, bilingual explanations)
3. `Advice` from recommendation synthesis (`Nên`, `Có thể`, `Tránh`, `Kỵ mạnh`, reasons, evidence, contextual layers)

The current screen wastes this by splitting six equal boxes across the viewport and forcing the user to infer the hierarchy. A serious almanac experience should read like a consultation:

1. What is the verdict for today?
2. What should I do?
3. What should I avoid?
4. If I act, when and in which direction should I act?
5. Why is the day being judged this way?
6. What detailed evidence supports the judgment?

The redesign should therefore preserve the six knowledge domains, but discard the current six-box grid.

## Problem Statement

The current `Scholar` screen layout is defined in [`crates/amlich-tui/src/widgets/screens/insight.rs`](../../crates/amlich-tui/src/widgets/screens/insight.rs). In `Large` mode it renders a `3x2` grid, in `Medium` mode a `2x3` grid, and in `Small` mode a vertical stack.

Current visual structure:

- `Can Chi · Ngũ Hành · Sao`
- `Sao & Trực`
- `Rủi Ro & Kiêng Kỵ`
- `Nạp Âm & Ngũ Hành`
- `Hướng & Thần`
- `Nên Làm / Tránh`

This layout has several structural problems:

- all six panels appear equally important, even though user intent is not equal
- decision support and traditional evidence compete instead of layering
- panel ownership overlaps (`Trực`, `Sao`, `Thần sát`, `Ngũ hành` are repeated)
- the screen reads like a DTO explorer, not a consultation
- the strongest engine output (`daily_recommendations`) is not the dominant panel
- the strongest negative signals (`taboos`, `Kỵ mạnh`, `xung`, `sát hướng`) are visually diluted
- medium and small widths preserve the same mental model instead of reordering into a reading flow

## Current Findings

### Current `Scholar` Screen Inventory

The current panels and their rough behavior are:

1. `Can Chi · Ngũ Hành · Sao`
   - mixed summary panel with `Can Chi ngày/tháng/năm`, one `Ngũ hành/Nạp âm` line, `Trực`, `Cát tinh`, `Sát tinh`, `Thần sát`
   - broad but shallow
   - duplicates content owned more clearly by other panels

2. `Sao & Trực`
   - `Trực`, day star, cát tinh, sát tinh, deity again
   - stronger thematic focus than panel 1 but still mostly a list

3. `Rủi Ro & Kiêng Kỵ`
   - currently built from top recommendation rows plus `taboos`, `lục xung`, `sát hướng`
   - useful, but much more shallow than the underlying engine permits

4. `Nạp Âm & Ngũ Hành`
   - `Nạp âm`, `Ngũ hành`, one prose meaning, and `con giáp`
   - underuses `can_element`, `chi_element`, `element nature`, and synthesis opportunities

5. `Hướng & Thần`
   - `xuất hành`, `Hỷ Thần`, `Tài Thần`, day deity classification and meaning
   - useful, but isolated from timing and decision flow

6. `Nên Làm / Tránh`
   - currently based on `day_guidance` or `truc` fallback in the panel widget
   - this is materially weaker than the recommendation engine already available elsewhere in the app

### Domain-Level Conclusion

The six domains are still valid, but they should be treated as content ownership only:

- day identity
- traditional evidence
- risk / taboo
- element reading
- direction / deity
- final recommendations

The equal-panel grid should be replaced with a hierarchical document.

## `amlich-core` Capability Audit

The redesign should start from what already exists in `amlich-core` and `amlich-api`, not from new speculative rule systems.

### Capability Model

The current product already exposes a three-layer stack:

#### Layer 1: Deterministic Almanac Facts (`amlich-core`)

Primary structures live in [`crates/amlich-core/src/almanac/types.rs`](../../crates/amlich-core/src/almanac/types.rs):

- `DayElement`
  - `na_am`
  - `element`
  - `can_element`
  - `chi_element`
  - `evidence`
- `DayConflict`
  - `opposing_chi`
  - `opposing_con_giap`
  - `tuoi_xung`
  - `sat_huong`
- `TravelDirection`
  - `xuat_hanh_huong`
  - `tai_than`
  - `hy_than`
- `DayStars`
  - `cat_tinh`
  - `sat_tinh`
  - `day_star`
  - `star_system`
  - `matched_rules`
- `DayDeity`
  - deity name and classification
- `DayTaboo`
  - `rule_id`
  - `name`
  - `severity`
  - `reason`
- `XungHopResult`
  - `luc_xung`
  - `tam_hop`
  - `tu_hanh_xung`
  - `liu_he`
  - `xiang_hai`
  - `xiang_xing`
- `TrucInfo`
  - name and quality
- `TangCan`
  - hidden stems plus strength
- `DayTenGods`
  - predefined `Thập Thần` relations
- optional `tu_menh`
  - available only with birth-dependent context

Computation path in [`crates/amlich-core/src/almanac/calc.rs`](../../crates/amlich-core/src/almanac/calc.rs):

- computes `Nạp Âm` by sexagenary day pair
- computes `xung/hợp` by day branch
- computes `Trực` by day branch and lunar month
- computes day deity by lunar month and day branch
- resolves star rules from layered sources:
  - fixed by day branch
  - fixed by full canchi
  - by year stem
  - by lunar month
  - by `tiết khí`
- builds `matched_rules` provenance entries for stars
- attaches source evidence metadata across subsystems

This is already a rich deterministic almanac engine.

#### Layer 2: Human-Readable Interpretation (`amlich-api` insight layer)

Primary interpretation assets live in `amlich-core` data files and are assembled in [`crates/amlich-api/src/lib.rs`](../../crates/amlich-api/src/lib.rs):

- `canchi.json`
  - can meaning
  - can nature
  - chi meaning
  - chi animal
  - chi hours
  - element nature
  - day branch `good_for` / `avoid_for`
- `na-am-insight.json`
  - prose meaning for each `Nạp Âm`
- `truc-insight.json`
  - `Trực` meaning, `good_for`, `avoid_for`
- `day-deity-insight.json`
  - `Hoàng Đạo` / `Hắc Đạo` explanation
  - deity-specific meaning
- `ten-gods-insight.json`
  - named `Thập Thần` meanings
- `tu-menh-insight.json`
  - Kua/trigram/direction interpretation
- `dai-van-insight.json`
  - direction and pillar meaning for major cycles

These are surfaced into `DayInsightDto` in [`crates/amlich-api/src/dto.rs`](../../crates/amlich-api/src/dto.rs):

- `CanChiInsightDto`
- `DayGuidanceDto`
- `NaAmInsightDto`
- `TrucInsightDto`
- `DayDeityInsightDto`
- `StarsInsightDto`
- `TabooInsightItemDto`
- `TravelInsightDto`
- `XungHopInsightDto`
- `TangCanInsightDto`
- `TenGodsInsightDto`
- `HoursInsightDto`
- `TuMenhInsightDto`
- `DaiVanInsightDto`

This means the app already has structured prose and not just raw fields.

#### Layer 3: Recommendation Synthesis (`amlich-core` recommendation engine)

The strongest action layer lives in [`crates/amlich-core/src/almanac/recommendation/synthesize.rs`](../../crates/amlich-core/src/almanac/recommendation/synthesize.rs).

It already converts:

- `Trực`
- stars
- day deity
- taboos
- `xung/hợp`
- `giờ hoàng đạo`
- travel direction
- `tiết khí`
- optional packs and contextual layers

into structured recommendation outputs:

- `Nên`
- `Có thể`
- `Tránh`
- `Kỵ mạnh`
- reasoning lines
- evidence source
- evidence code
- summary strings
- active packs / profile layers

This is the most powerful “what should I do” layer in the product today.

### Capability by Scholar Subject

#### 1. `Can Chi · Ngũ Hành · Sao`

Already available today:

- day / month / year `Can Chi`
- can meaning
- can nature
- chi meaning
- chi animal
- chi element
- chi hours
- element nature
- `Nạp Âm`
- day `Ngũ hành`
- `can_element`
- `chi_element`
- optional `Tàng Can`
- optional `Thập Thần`

Current panel underuses:

- `nature`
- `chi.hours`
- `element.nature`
- `can_element`
- `chi_element`
- `tang_can`
- `ten_gods`

Conclusion:

- this domain can already become much more verbose and insightful without new core work
- stars should be removed from this panel except where needed to summarize the day identity

#### 2. `Sao & Trực`

Already available today:

- `Trực` name and quality
- `Trực` meaning / good-for / avoid-for
- `day_star`
- `day_star_quality`
- `cat_tinh`
- `sat_tinh`
- star rule provenance via `matched_rules`
- precedence from multiple rule categories

Current limitation:

- no broad prose encyclopedia for every star name
- current DTOs carry names and provenance, not deep star-by-star essays

Conclusion:

- this domain can already become much better at explanation and evidence
- full classical star lore would require additional data, but is not required for the redesign

#### 3. `Rủi Ro & Kiêng Kỵ`

Already available today:

- taboo names
- taboo reasons
- taboo severity
- taboo source evidence
- `tuoi_xung`
- `luc_xung`
- `tam_hop`
- `tu_hanh_xung`
- `xiang_hai`
- `xiang_xing`
- `sat_huong`
- recommendation-layer negative evidence and bucket ranking

Conclusion:

- this domain is severely under-rendered today
- it can become a first-class risk panel with no new core logic

#### 4. `Nạp Âm & Ngũ Hành`

Already available today:

- `Nạp Âm`
- element meaning
- overall day element
- `can_element`
- `chi_element`
- element nature text
- `con giáp` references

Current limitation:

- advisory logic here is weaker than in the recommendation engine
- this domain is naturally more descriptive than prescriptive

Conclusion:

- this domain can become deeper and more elegant immediately
- it should support the day identity reading, not compete with decision panels

#### 5. `Hướng & Thần`

Already available today:

- `xuat_hanh_huong`
- `tai_than`
- `hy_than`
- deity classification
- classification meaning
- deity meaning
- optional `Tu Mệnh` directions when user profile exists

Conclusion:

- this domain can be stronger immediately
- without birth context it should stay day-level, not pretend to be personalized feng shui

#### 6. `Nên Làm / Tránh`

Already available today:

- `daily_recommendations`
- `contextual_recommendations`
- summary strings
- bucket ordering
- evidence chips
- reason lists
- active pack labels
- profile layers

Current panel problem:

- the current scholar widget ignores this rich engine and falls back to static `day_guidance` or `truc`

Conclusion:

- this domain should be rebuilt on top of recommendation synthesis
- this is the single highest-value presentation rewrite in the Scholar screen

### Capability Boundaries and Missing Pieces

The redesign must also be honest about what the engine does **not** yet provide.

Missing or shallow today:

- a deep prose catalog for every cát tinh / hung tinh
- richer interaction logic between day element texture and activity classes
- fully personalized destiny reading without birth context
- stronger interpretation for relationship/business/medical nuance beyond the current recommendation matrix

These should be treated as future domain expansions, not prerequisites for the UI redesign.

## Design Principles

The new Scholar experience should follow these principles:

1. `Verdict first`
   - the user should know the day judgment before reading the evidence

2. `Action before lore`
   - recommendation and risk panels come before interpretive panels

3. `Interpretation before raw data`
   - the app should synthesize what the day means, not dump fields first

4. `Evidence always available`
   - the app should never hide the basis of its judgment, only defer it below the fold or into a collapsible block

5. `Do not move domain logic into widgets`
   - `amlich-core` remains the computation layer
   - `amlich-api` remains the interpretation and DTO assembly layer
   - `amlich-tui` remains presentation plus light view-model helpers

6. `Do not fake personal metaphysics`
   - if birth context is not present, say that explicitly

## Recommended Information Architecture

The new Scholar screen should become a vertically ordered consultation page.

### 1. `Tổng Luận Hôm Nay`

Purpose:

- establish the day in one compact verdict

Content:

- summary verdict
- strongest positive
- strongest negative
- brief evidence line (`Can Chi`, `Trực`, `Sao`, `Kiêng kỵ`)

### 2. `Nên Làm / Tránh`

Purpose:

- provide the dominant action board

Content:

- `Nên`
- `Có thể`
- `Tránh`
- `Kỵ mạnh`
- strongest reason chips and evidence

### 3. `Rủi Ro & Kiêng Kỵ`

Purpose:

- isolate high-signal warnings

Content:

- taboo severity and reasons
- `tuoi_xung`
- `lục xung`
- `sát hướng`
- domain-sensitive notices

### 4. `Ứng Dụng Khi Hành Sự`

Two coordinated sections:

- `Hướng & Thần`
- `Giờ / Khung hành động`

This should answer:

- if the user acts today, how should they move, orient, and time the action?

### 5. `Khí Ngày`

Purpose:

- explain the day identity

Content:

- `Can Chi`
- `Ngũ hành`
- `Nạp âm`
- can / chi nature
- optional `Tàng Can` and `Thập Thần`

### 6. `Sao & Trực`

Purpose:

- explain the traditional evidence stack

Content:

- `Trực`
- `Sao ngày`
- top cát / hung stars
- star provenance and precedence

### 7. `Căn Cứ Chi Tiết`

Purpose:

- provide expandable evidence and source details

Content:

- star `matched_rules`
- evidence sources
- raw factor tags
- contextual vs baseline layer references

## Responsive Wireframe Direction

### `Large` (`>= 100` cols)

Use a hierarchical page with grouped rows:

1. full-width hero
2. two-column decision row (`Nên Làm / Tránh` + `Rủi Ro & Kiêng Kỵ`)
3. two-column application row (`Hướng & Thần` + timing)
4. two-column interpretation row (`Khí Ngày` + `Sao & Trực`)
5. full-width evidence drawer

### `Medium` (`60-99` cols)

Use a top-down document with limited side-by-side pairings:

1. hero
2. `Nên Làm / Tránh`
3. `Rủi Ro & Kiêng Kỵ`
4. paired `Hướng & Thần` + timing
5. paired `Khí Ngày` + `Sao & Trực`
6. evidence drawer

### `Small` (`< 60` cols)

Use a pure vertical reading flow:

1. hero
2. `Nên Làm / Tránh`
3. `Rủi Ro & Kiêng Kỵ`
4. `Hướng & Thần`
5. timing
6. `Khí Ngày`
7. `Sao & Trực`
8. evidence drawer

## Domain Ownership Matrix

The redesign should preserve the six current domains as ownership only.

| Domain | Current widget | Future role |
|---|---|---|
| Day identity | `ScholarlyWidget` + `NaAmPanelWidget` | merge into `Khí Ngày` |
| Traditional evidence | `StarsPanelWidget` | evolve into `Sao & Trực` |
| Risk and taboo | `RiskWidget` | expand into first-class caution board |
| Direction and deity | `DirectionPanelWidget` | expand and pair with timing |
| Decision support | `GuidancePanelWidget` | replace with recommendation-engine board |
| Raw evidence | spread across widgets | collapse into dedicated evidence drawer |

## Implementation Architecture

The implementation should preserve the current boundary:

- `amlich-core`
  - deterministic calculations and recommendation synthesis
- `amlich-api`
  - DTO assembly and insight enrichment
- `amlich-tui`
  - view-model helpers and rendering only

This means:

- do not move recommendation logic into widgets
- do not duplicate `Trực` or star classification logic in the TUI
- add new API DTO surface only when existing DTOs are insufficient for presentation
- prefer view-model helpers in `AppState` for summaries and row ordering

## Proposed Delivery Plan

### Stage 0: Lock the redesign target and migration constraints

Deliverables:

- this plan document
- final acceptance of consultation-style IA
- confirmation that the six-box grid will be retired

### Stage 1: Rebuild the screen skeleton

Files:

- `crates/amlich-tui/src/widgets/screens/insight.rs`

Tasks:

- replace equal-grid layout with hierarchical layout per breakpoint
- add dedicated section slots for hero, decision, risk, application, interpretation, evidence
- keep existing widgets temporarily if needed as placeholders

### Stage 2: Make recommendations the dominant Scholar panel

Files:

- `crates/amlich-tui/src/widgets/guidance_panel.rs`
- or replace with a new consultation-oriented widget
- `crates/amlich-tui/src/state.rs`

Tasks:

- route the panel to `daily_recommendations` / `contextual_recommendations`
- show bucket ordering, reasons, evidence chips, and contextual override notes
- stop using static `day_guidance` as the primary Scholar advice block

### Stage 3: Upgrade risk into a real caution board

Files:

- `crates/amlich-tui/src/widgets/risk.rs`
- `crates/amlich-tui/src/state.rs`

Tasks:

- surface taboo reasons and severity
- show negative recommendation evidence more explicitly
- add structured conflict guidance (`tuoi_xung`, `xiang_hai`, `tu_hanh_xung` if available)
- maintain sensitive-domain disclaimers

### Stage 4: Merge identity and element reading into `Khí Ngày`

Files:

- `crates/amlich-tui/src/widgets/scholarly.rs`
- `crates/amlich-tui/src/widgets/naam_panel.rs`

Tasks:

- move from mixed fact dump to synthesized day identity panel
- show can/chi meaning and nature, day element texture, `Nạp Âm`, and optional deeper scholarly fields
- remove duplicated stars / deity content from this section

### Stage 5: Reframe `Sao & Trực` as traditional evidence

Files:

- `crates/amlich-tui/src/widgets/stars_panel.rs`

Tasks:

- focus the widget on `Trực`, day star, top cát / hung stars, and provenance
- explain why those stars are present today using `matched_rules`
- avoid repeating general recommendation output

### Stage 6: Pair direction and timing

Files:

- `crates/amlich-tui/src/widgets/direction_panel.rs`
- existing or new timing widget

Tasks:

- coordinate direction with hour windows
- allow day-level guidance now
- leave personal direction overlay optional on birth-context support later

### Stage 7: Add a compact evidence drawer

Files:

- new evidence widget or section in `insight.rs`

Tasks:

- show provenance and raw factor references without polluting top sections
- keep collapsed by default

## Acceptance Criteria

The redesign is successful when:

1. the first visible section gives a verdict, not a list of labels
2. `Nên Làm / Tránh` is clearly the dominant action block
3. `Rủi Ro & Kiêng Kỵ` clearly distinguishes strong caution from soft caution
4. direction and timing are exposed as practical application support
5. `Khí Ngày` explains the day's character without duplicating recommendation buckets
6. `Sao & Trực` reads as evidence, not as a second verdict board
7. the screen scales cleanly at all current breakpoints
8. the redesign reuses existing `amlich-core` and `amlich-api` capability before proposing new core rules
9. any remaining gaps are explicitly documented as future data work, not silently patched in the TUI

## Risks and Open Questions

1. `StarsInsightDto` is still shallow for prose-heavy star interpretation.
   - Mitigation: use `matched_rules` and top-star summaries now; treat deeper star lore as future data expansion.

2. `Tàng Can` and `Thập Thần` may make the day identity panel too dense.
   - Mitigation: keep them optional or move to expanded detail/evidence mode.

3. There is a tension between `Scholar` as a lore screen and `Scholar` as a practical screen.
   - Recommendation: resolve this by making the screen consultation-first, with evidence below action.

4. Personal feng shui surfaces (`Tu Mệnh`, `Đại Vận`) require birth context.
   - Recommendation: do not mix them into the default Scholar flow unless profile data is present.

## Recommendation

Proceed with a full Scholar screen redesign.

Do **not** preserve the current `6 equal panels` layout.

Preserve the six domains as content ownership, but rebuild the screen around this hierarchy:

1. verdict
2. action
3. caution
4. application
5. interpretation
6. evidence

This approach aligns with the actual strength of the current engine and is the fastest path to making the app feel like a knowledgeable almanac practitioner instead of a dashboard of disconnected widgets.
