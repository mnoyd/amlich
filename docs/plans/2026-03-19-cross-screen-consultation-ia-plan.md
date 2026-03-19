# Cross-Screen Consultation IA Plan

**Date:** 2026-03-19
**Status:** Proposed
**Issue:** `amlich-7jj`
**Companion Doc:** [Scholar Consultation Redesign Plan](./2026-03-19-scholar-consultation-redesign-plan.md)

**Goal:** Extend the consultation-style redesign beyond the `Scholar` tab and define a coherent information architecture for the top-level almanac screens: `Scholar`, `Giờ Tốt`, `Ngũ Hành`, `Phong Thủy`, and `Tiết Khí`.

## Executive Summary

The app should not evolve into five “show all information” screens.

That would only recreate the same problem at a larger scale: rich data rendered as parallel catalogs, with no clear sense of user intent, reading order, or practical value.

The redesign should instead treat the five tabs as a layered reading system:

1. `Scholar` — synthesis and verdict
2. `Giờ Tốt` — timing and execution
3. `Ngũ Hành` — interpretive reading of the day's khí
4. `Phong Thủy` — orientation and personal overlay
5. `Tiết Khí` — seasonal and environmental context

Each screen should have a distinct job, a controlled verbosity target, and a clearly bounded content ownership model.

## Problem

Today, the top-level screens already exist in `amlich-tui` as separate views, but their long-term purpose is still underdefined. The risk is that they become parallel content dumps:

- all verbose
- all equally important
- all partly overlapping
- all trying to answer different user questions at once

This would make the UI feel “full” but not “knowledgeable.” A strong almanac practitioner does not present all systems at once with equal weight. They answer in layers:

1. verdict
2. application
3. explanation
4. deeper doctrine when asked

The screen model should follow the same logic.

## Design Principle

The five tabs should be organized by **user intent**, not by raw DTO namespace.

### Correct Model

- `Scholar` answers: what is today's judgment?
- `Giờ Tốt` answers: if I act, when should I act?
- `Ngũ Hành` answers: what kind of day is this energetically?
- `Phong Thủy` answers: what directional or personal alignment matters?
- `Tiết Khí` answers: what seasonal context frames the day?

### Incorrect Model

- one tab per data family
- each tab tries to show everything it can
- every screen repeats the same conclusion with different labels
- “verbose” equals “wise”

It does not. It only becomes bloated.

## Product Model

The top-level reading flow should be:

1. `Hôm nay nên làm gì?` → `Scholar`
2. `Nếu làm thì làm lúc nào?` → `Giờ Tốt`
3. `Vì sao khí ngày lại như vậy?` → `Ngũ Hành`
4. `Theo hướng nào, và phần nào là cá nhân hóa?` → `Phong Thủy`
5. `Thời khí hiện tại đang đẩy ngày này theo hướng nào?` → `Tiết Khí`

This gives every screen a reason to exist.

## Existing Capability Surface

This IA plan is grounded in capability already exposed by `amlich-core` and `amlich-api`.

### Shared Foundation

The current stack already provides:

- deterministic day facts from `amlich-core`
- interpretation DTOs from `amlich-api`
- recommendation synthesis from `amlich-core`

Relevant DTO families already available:

- `DayBundleDto`
- `DailyRecommendationsDto`
- `HoursInsightDto`
- `NaAmInsightDto`
- `CanChiInsightDto`
- `TrucInsightDto`
- `StarsInsightDto`
- `TravelInsightDto`
- `TietKhiInsightDto`
- `TangCanInsightDto`
- `TenGodsInsightDto`
- `TuMenhInsightDto`
- `DaiVanInsightDto`

This means the IA redesign is primarily a presentation and orchestration problem, not a missing-domain-logic problem.

### Capability by Screen

#### `Scholar`

Already supported by:

- `daily_recommendations`
- `contextual_recommendations`
- `DayConflictDto`
- `DayTabooDto`
- `TravelDirectionDto`
- `CanChiInsightDto`
- `NaAmInsightDto`
- `TrucInsightDto`
- `StarsInsightDto`

This screen can already become a high-quality consultation surface.

#### `Giờ Tốt`

Already supported by:

- `GioHoangDaoDto`
- `HoursInsightDto`
- recommendation evidence from `GioHoangDao`

This is enough to build a timing-first screen now.

#### `Ngũ Hành`

Already supported by:

- `DayElementDto`
- `NaAmInsightDto`
- `CanChiInsightDto`
- `TangCanInsightDto`
- `TenGodsInsightDto`
- `XungHopInsightDto`

This is enough to build a serious interpretive screen now.

#### `Phong Thủy`

Already supported by:

- `TravelInsightDto`
- `DayDeityInsightDto`
- `TuMenhInsightDto`
- `DaiVanInsightDto`

This is enough for a restrained day-level directional screen now, and a richer personal screen when user profile data exists.

#### `Tiết Khí`

Already supported by:

- `TietKhiInsightDto`
- recommendation evidence from `Tiết Khí`
- current solar term metadata

This is enough to build a contextual seasonal screen now.

## Global Screen Contract

Every screen should follow the same reading skeleton, but with different emphasis:

1. `Nhận định`
2. `Luận giải`
3. `Ứng dụng`
4. `Căn cứ`

The relative weight changes by screen.

### Shared Rules

- the first visible block answers the core user question of that tab
- default mode is curated, not exhaustive
- expanded mode can expose more evidence and detail
- evidence should always be available, but not always dominant
- personal reading must be explicitly labeled as personal
- if data is absent, say so directly instead of faking completeness

## Screen Roles

### 1. `Scholar`

**Primary job:** synthesis

**User question:** “Rốt cuộc hôm nay nên làm gì?”

**Verbosity target:** concise to medium

**Why it exists:**

This is the consultation cover page. It is the only screen that should try to answer the entire day in one reading.

**Owns:**

- verdict
- recommendation buckets
- strongest risks
- concise direction/timing support
- compact explanation of why

**Must show:**

- top verdict
- `Nên / Có thể / Tránh / Kỵ mạnh`
- strongest caution
- best timing summary
- best direction summary
- compact interpretive evidence

**Must not become:**

- a raw full-detail hour screen
- a full feng shui analysis page
- a star encyclopedia
- a seasonal essay page

**UI posture:**

- high confidence
- strong hierarchy
- not too verbose
- best for first read

### 2. `Giờ Tốt`

**Primary job:** execution timing

**User question:** “Nếu làm hôm nay thì nên làm lúc nào?”

**Verbosity target:** concise to medium

**Why it exists:**

Timing is practical and highly actionable. It deserves its own screen rather than being buried as a subpanel.

**Owns:**

- auspicious hours
- inauspicious hours
- action windows
- timing tradeoffs
- relationship between day quality and hour quality

**Must show:**

- top good windows first
- compact 12-hour timeline
- best hour suggestions by type of action when possible
- short caution when whole-day quality and hour quality are in tension

**Must not become:**

- a generic recommendations screen
- a restatement of `Scholar` buckets
- a long doctrinal explanation page

**UI posture:**

- procedural
- task-oriented
- clear and concrete

### 3. `Ngũ Hành`

**Primary job:** interpretive reading of the day’s khí

**User question:** “Khí ngày này là gì, và hợp kiểu việc nào?”

**Verbosity target:** medium to high

**Why it exists:**

This screen is where the app earns the feeling of knowledge. It should feel interpretive, but still connected to use.

**Owns:**

- `Can Chi` reading
- `Ngũ Hành`
- `Nạp Âm`
- can/chi nature
- optional `Tàng Can`
- optional `Thập Thần`
- structural element reading

**Must show:**

- overall khí verdict
- `Nạp Âm` meaning
- can element / chi element split
- element texture and tone
- if expanded: `Tàng Can`, `Thập Thần`, deeper structural factors

**Must not become:**

- the final action board
- the taboo screen
- a giant raw data list

**UI posture:**

- learned
- reflective
- more verbose than `Scholar`
- still tied back to practical implications

### 4. `Phong Thủy`

**Primary job:** orientation and personal overlay

**User question:** “Theo hướng nào, và phần nào là của riêng mình?”

**Verbosity target:** adaptive

**Why it exists:**

This screen should handle the boundary between general day-level directional advice and true personal readings.

**Owns:**

- `xuất hành`
- `Tài Thần`
- `Hỷ Thần`
- deity framing where relevant
- `Tu Mệnh`
- `Đại Vận`

**Two modes:**

#### Without profile

The screen should stay restrained:

- day-level directions
- directional usefulness
- no pretense of personal feng shui analysis

#### With profile

The screen can expand into:

- favorable and unfavorable directions
- Kua group context
- personal direction overlay
- major cycle (`Đại Vận`) references where useful

**Must show:**

- what is general
- what is personal
- what is unavailable without profile data

**Must not become:**

- a fake personalized reading without inputs
- a duplicate of the recommendation screen
- a giant doctrinal dump by default

**UI posture:**

- careful
- scoped
- explicit about evidence and limits

### 5. `Tiết Khí`

**Primary job:** seasonal context and environmental framing

**User question:** “Thời khí hiện tại đang đẩy ngày này theo hướng nào?”

**Verbosity target:** medium

**Why it exists:**

This screen explains the larger environmental background rather than the single-day recommendation itself.

**Owns:**

- current solar term
- solar-term meaning
- astronomy / weather framing
- agriculture and health lists
- seasonal interpretation
- how seasonal context supports or constrains plans

**Must show:**

- current `Tiết Khí`
- what the season encourages
- what the season makes awkward or premature
- optional practical effects on work, travel, health, or atmosphere

**Must not become:**

- a second recommendations screen
- a duplicate of `Ngũ Hành`
- a long raw metadata page without interpretation

**UI posture:**

- contextual
- environmental
- explanatory

## Verbosity Strategy

The app should not make every screen equally verbose.

Recommended default verbosity:

- `Scholar` — concise to medium
- `Giờ Tốt` — concise to medium
- `Ngũ Hành` — medium to high
- `Phong Thủy` — low without profile, medium to high with profile
- `Tiết Khí` — medium

### Why this matters

If all five tabs are verbose, none of them feels focused.

The screen system should support both:

- quick reading
- deep reading

That requires uneven density by design.

## Cross-Screen Ownership Boundaries

To avoid duplication, the screens should divide responsibilities this way:

| Screen | Owns | Should not own |
|---|---|---|
| `Scholar` | final verdict and action ranking | full drill-down detail from every subsystem |
| `Giờ Tốt` | timing and action windows | whole-day recommendation hierarchy |
| `Ngũ Hành` | energetic / structural reading | final decision board |
| `Phong Thủy` | direction and personal overlay | whole-day action board or generic lore dump |
| `Tiết Khí` | seasonal context | repeated taboo/recommendation content |

## Expanded Mode Policy

Expanded or verbose mode should be used selectively.

Good candidates for expanded mode:

- full taboo reasons
- star provenance (`matched_rules`)
- `Tàng Can`
- `Thập Thần`
- contextual-vs-baseline recommendation comparisons
- long `Tiết Khí` context
- `Tu Mệnh` and `Đại Vận` details when profile exists

Bad candidates for default mode:

- all raw lists at once
- every secondary factor on first paint
- repeated restatement of the same recommendation judgment

## Navigation Philosophy

The screen system should guide the user naturally.

Recommended reading path:

1. start in `Scholar`
2. if acting today, check `Giờ Tốt`
3. if wanting to understand the internal logic, check `Ngũ Hành`
4. if orientation or profile matters, check `Phong Thủy`
5. if wanting broader environmental context, check `Tiết Khí`

This creates a clean mental model:

- `Scholar` = answer
- `Giờ Tốt` = execution
- `Ngũ Hành` = interpretation
- `Phong Thủy` = orientation/personalization
- `Tiết Khí` = seasonal framing

## Risks

1. `Phong Thủy` may overclaim if profile-less mode is not carefully constrained.
   - Mitigation: separate general day direction from personal destiny guidance.

2. `Ngũ Hành` may become too abstract if it does not reconnect to practical implications.
   - Mitigation: every section should end with `Ứng dụng`.

3. `Scholar` may still absorb too much if the other screens are not given strong ownership.
   - Mitigation: treat the other tabs as drill-down lenses, not clones.

4. `Giờ Tốt` may become a raw timeline screen if not given execution-oriented copy.
   - Mitigation: prioritize “best windows” and action framing over hour tables.

5. `Tiết Khí` may become a decorative screen if it does not influence judgment context.
   - Mitigation: explicitly connect seasonal context to day-level planning tone.

## Recommendation

Extend the consultation redesign to all five top-level tabs.

Do **not** make them five verbose “show all information” surfaces.

Use this role split:

1. `Scholar` — synthesis
2. `Giờ Tốt` — execution timing
3. `Ngũ Hành` — interpretive reading
4. `Phong Thủy` — directional / personal overlay
5. `Tiết Khí` — contextual seasonality

This will make the app feel more like a strong practitioner:

- decisive on the first screen
- practical on the second
- explanatory on the later screens
- deep only when the user chooses depth

## Suggested Next Plan

The next implementation plan should define:

1. exact widget/file ownership per screen
2. which existing widgets can be repurposed
3. which new view-model helpers are needed in `AppState`
4. whether `Giờ Tốt` needs a dedicated action-window helper
5. whether `Phong Thủy` should split into profile and non-profile render paths
6. whether expanded detail toggles should be shared across screens or local to each screen
