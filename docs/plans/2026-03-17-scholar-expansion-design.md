# Scholar Expansion Design — New Top-Level Tabs

**Date:** 2026-03-17
**Goal:** Expand Scholar view and add 4 new top-level tabs to surface all unused DayInsightDto data

## Overview

The current Scholar screen displays only ~25-30 data items across 4 panels, while DayInsightDto and DayBundleDto contain 20+ rich data structures that are never shown. This design adds 4 new top-level tabs and enriches the existing Scholar tab.

## Tab Structure

```
Dashboard  Event  Scholar  Giờ Tốt  Ngũ Hành  Phong Thủy  Tiết Khí  Planning  Calendar
```

### ActiveView Enum

```rust
pub enum ActiveView {
    Dashboard,
    Event,       // conditional: has_event_today()
    Scholar,     // enriched
    Hours,       // NEW — Giờ Tốt
    Elements,    // NEW — Ngũ Hành
    FengShui,    // NEW — Phong Thủy
    SolarTerms,  // NEW — Tiết Khí
    Planning,
    Calendar,
}
```

### Navigation

- Tab/Shift+Tab: cycle through tabs
- Number keys 1-9: direct jump
- Ribbon adapts per LayoutMode:
  - Large (>100 cols): full names
  - Medium (60-100): abbreviated — Dash Evt Sch Giờ NHành PThủy TKhí Plan Cal
  - Small (<60): only active tab with arrows `< [Scholar] >`

---

## Tab 1: Scholar (Enriched) — 3x2 Grid

### Panel Layout (Large)

| Position | Panel | Content |
|----------|-------|---------|
| Top-left | Can Chi | Can Chi ngày + insight (meaning, nature), Chi (meaning, animal, hours), Can Chi tháng/năm |
| Top-center | Sao & Trực | Trực + meaning, Day star + quality, Cát tinh, Sát tinh, Thần sát + deity_meaning |
| Top-right | Rủi Ro | Lục xung, Sát hướng, Tương hại, Taboos (name, severity, reason), Lưu ý |
| Bottom-left | Nạp Âm & Ngũ Hành | Nạp âm + full meaning, Ngũ hành Can/Chi, quan hệ sinh khắc, Con giáp 3 trụ |
| Bottom-center | Hướng & Thần | Xuất hành, Hỷ Thần, Tài Thần, Deity meaning, Classification meaning |
| Bottom-right | Nên Làm / Tránh | DayGuidanceDto full lists (good_for, avoid_for) |

### Data Sources
- `CanChiInsightDto` — can/chi meaning, nature, element
- `CanChiInfoDto` — 3 pillars (year, month, day)
- `NaAmInsightDto` — na_am, element, meaning
- `StarsInsightDto` — cat_tinh, sat_tinh, day_star, day_star_quality
- `TrucInsightDto` — name, quality, meaning
- `DayDeityInsightDto` — name, classification, classification_meaning, deity_meaning
- `TabooInsightItemDto[]` — name, severity, reason
- `TravelInsightDto` — xuat_hanh_huong, tai_than, hy_than
- `DayGuidanceDto` — good_for, avoid_for

### Responsive
- Large: 3x2 grid
- Medium: 2x3 grid
- Small: stack vertical, scroll j/k

---

## Tab 2: Giờ Tốt (Hours) — Timeline + Detail

### Layout (Large)

**Top section:** Timeline bar showing all 12 hours
- Each hour: Chi name, time range, good/bad indicator, star name

**Bottom section:** 2 columns
- Left: Good hours detail — chi, time_range, star, description
- Right: Bad hours detail

### Data Sources
- `GioHoangDaoDto` — day_chi, good_hour_count, all_hours
- `HourInfoDto` — hour_index, hour_chi, time_range, star, is_good
- `HoursInsightDto` — good_hour_count, good_hours
- `HourInsightEntryDto` — chi, time_range, star

### Responsive
- Large: timeline + 2 columns
- Medium: timeline + 1 column list
- Small: vertical list only, scroll

---

## Tab 3: Ngũ Hành (Elements) — 3x2 Grid

### Panel Layout (Large)

| Position | Panel | Content |
|----------|-------|---------|
| Top-left | Tàng Can | Chi ngày, 3 hidden stems (main/central/residual) with strength bars (0-100%) |
| Top-center | Thập Thần | Ten Gods to year stem + to self: label, name, meaning, relation, polarity |
| Top-right | Xung Hợp | Lục xung, Tam hợp triad, Lục hợp partner, Tương hại |
| Bottom-left | Ngũ Hành Tương Quan | 5 elements with strength bars, sinh/khắc relationships |
| Bottom-center | Can Chi 3 Trụ | Year/Month/Day Can+Chi table with Hành, Nạp âm |
| Bottom-right | Ngũ Hành Tổng Hợp | Element distribution chart, dominant element analysis |

### Data Sources
- `TangCanInsightDto` — main, central, residual, strength[3]
- `TenGodsInsightDto` — to_year_stem, to_self (TenGodsEntryInsightDto)
- `XungHopInsightDto` — luc_xung, tam_hop, liu_he, xiang_hai
- `CanChiInfoDto` — 3 pillars with ngu_hanh
- `CanChiInsightDto` — element insight

### Responsive
- Large: 3x2 grid
- Medium: 2x3 grid
- Small: stack vertical, scroll

---

## Tab 4: Phong Thủy (FengShui) — 2x2 Grid

### Panel Layout (Large)

| Position | Panel | Content |
|----------|-------|---------|
| Top-left | Tứ Mệnh / Kua | Kua number, trigram, group (East/West), direction, meanings |
| Top-right | Hướng Tốt/Xấu | favorable_directions list, unfavorable_directions list |
| Bottom-left | Đại Vận | Current pillar highlighted, all pillars with age ranges, element meanings |
| Bottom-right | La Bàn Tổng Hợp | ASCII compass showing 8 directions with good/bad markers + dai van direction |

### Data Sources
- `TuMenhInsightDto` — kua, group, trigram, direction, meaning, group_meaning, favorable/unfavorable_directions
- `DaiVanInsightDto` — direction, direction_meaning, start_age, current_pillar, all_pillars, phases_meaning
- `DaiVanPillarInsightDto` — index, can_chi, start_age, end_age, element, element_meaning

### Responsive
- Large: 2x2 grid with ASCII compass
- Medium: 2x2, simplified compass
- Small: stack vertical, no compass (list only)

---

## Tab 5: Tiết Khí (SolarTerms) — 2x2 Grid

### Panel Layout (Large)

| Position | Panel | Content |
|----------|-------|---------|
| Top-left | Tiết Khí Hiện Tại | Name (vi+han), longitude, full meaning, description |
| Top-right | Thiên Văn | Astronomy text, current longitude, season, weather |
| Bottom-left | Nông Nghiệp | Agriculture activity list (LocalizedListDto) |
| Bottom-right | Sức Khỏe | Health advice list (LocalizedListDto) |

### Data Sources
- `TietKhiInsightDto` — id, name, longitude, meaning, astronomy, agriculture, health, weather
- `TietKhiDto` — index, name, description, current_longitude, season

### Responsive
- Large: 2x2 grid
- Medium: 2x2, shorter lists
- Small: stack vertical, scroll

---

## Implementation Notes

### Files to modify
- `state.rs` — extend ActiveView enum, available_views(), navigation keybindings
- `events.rs` — add keybindings for new views (1-9)
- `ribbon.rs` — adapt tab display for 8-9 tabs, responsive abbreviation
- `page.rs` — add match arms for new views
- `widgets/screens/insight.rs` — redesign Scholar from 2x2 to 3x2 with enriched panels
- `widgets/scholarly.rs` — enrich with CanChi insight, NaAm meaning, etc.

### New files to create
- `widgets/screens/hours.rs` — Hours screen
- `widgets/screens/elements.rs` — Elements screen
- `widgets/screens/feng_shui.rs` — FengShui screen
- `widgets/screens/solar_terms.rs` — SolarTerms screen
- New widget files for each panel as needed

### New widgets to create
- `widgets/hours_timeline.rs` — 12-hour timeline with stars
- `widgets/hours_detail.rs` — good/bad hours detail lists
- `widgets/tang_can.rs` — hidden stems with strength bars
- `widgets/ten_gods.rs` — ten gods relations
- `widgets/xung_hop.rs` — detailed xung/hop display
- `widgets/element_chart.rs` — element distribution
- `widgets/pillars_table.rs` — 3-pillar can chi table
- `widgets/kua.rs` — Kua/Tu Menh display
- `widgets/dai_van.rs` — Dai Van pillars timeline
- `widgets/compass.rs` — ASCII compass visualization
- `widgets/tiet_khi.rs` — solar term display
- `widgets/agriculture.rs` — agriculture activities
- `widgets/health.rs` — health advice
