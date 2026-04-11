# Interaction Matrices — Implementation Plan

**Status:** Ready for implementation
**Prerequisites:** Phase 5 complete (fd7648f, a94116f) — all personal rule families in core + API
**Decision refs:** DEC-0015–0021

## Goal

Build the "Interaction Layer" that cross-references day almanac data (Can Chi, Trực, Sao, Thần, Giờ) with personal Bazi data (4 trụ, domain scores, element distribution) to produce **interconnected personal matrices**. Output is structured data that LLMs or brief generators can consume to produce evidence-based narratives.

## What Exists Now

### Day-level (Cụm 1) — computed in `almanac/calc.rs → DayFortune`
- Can Chi ngày/tháng/năm, Na Âm, Ngũ Hành
- Thập Nhị Trực (quality: cát/hung/bình)
- Sao tốt/xấu (cat_tinh, sat_tinh)
- Thần ngày (Hoàng Đạo/Hắc Đạo)
- Giờ Hoàng Đạo (12 giờ × star quality)
- Hướng: Tài Thần, Hỷ Thần, Phúc Thần (NEW), Xuất Hành, Sát Phương (NEW)
- Xung Hợp (Lục Xung/Tam Hợp/Lục Hợp/Tương Hại/Tương Hình)
- Thập Thần (ngày→năm)
- Taboos (Tam Nương, Nguyệt Kỵ, Sát Chủ)

### Person-level (Cụm 2) — computed in `bazi/` + `almanac/`
- Bazi 4 trụ (Năm/Tháng/Ngày/Giờ) + Nhật Chủ
- Ngũ Hành phân bố (mộc/hỏa/thổ/kim/thủy scores)
- Nhật Chủ mạnh/yếu (score + label)
- Thập Thần distribution (10 gods count)
- Domain scores (career/wealth/relationship/health/timing × 0-100)
- Kua số + 4 hướng tốt/4 hướng xấu
- Đại Vận (life phases)
- Yearly Hạn composite (NEW: Cửu Diệu + Tam Tai + Kim Lâu + Hoàng Ốc + Thái Tuế)

### Recommendation layer (Cụm 3) — `almanac/recommendation/`
- Activity buckets (Nên/Có Thể/Tránh/Kỵ Mạnh)
- Evidence chain per activity
- Advisory scoring (0-100 + verdict)

## The 4 Matrices to Build

### Matrix 1: Day-Person Interaction Matrix
**What:** How today's Can Chi interacts with each of the 4 personal pillars.

**Computation:**
```
For each pillar in [year, month, day, hour]:
  - Thập Thần: Can ngày hôm nay → Nhật Chủ (and Can ngày → Can từng trụ)
  - Xung/Hợp: Chi ngày hôm nay × Chi từng trụ
    → Lục Xung, Lục Hợp, Tương Hại, Tương Hình, Tam Hợp
  - Ngũ Hành: element ngày vs element trụ → sinh/khắc/đồng
```

**Output:** `DayPersonMatrix` — 4 rows (pillars) × N interaction columns

**Key files:**
- `bazi/chart.rs` — BaziChart with 4 pillars
- `almanac/xung_hop.rs` — all relationship functions
- `almanac/thap_than.rs` — get_thap_than(can, can)

### Matrix 2: Element Resonance Matrix
**What:** Whether today's elemental energy supports or depletes the person's weak/strong elements.

**Computation:**
```
day_element = Ngũ Hành ngày (from Na Âm or Can ngày)
person_distribution = Bazi element distribution (mộc/hỏa/thổ/kim/thủy)
season_factor = SeasonStrengthMatrix[day_element][current_month_chi]
element_relation = ElementRelationMatrix[day_element] × person_distribution
resonance_score = weighted sum
```

**Output:** `ElementResonanceMatrix` — per-element resonance scores + summary

**Key files:**
- `bazi/scoring.rs` — ElementRelationMatrix, SeasonStrengthMatrix (already defined!)
- `almanac/calc.rs` — day_element computation

### Matrix 3: Personal Hour Matrix
**What:** Rank each of 12 hours by personal compatibility, not just generic Hoàng Đạo.

**Computation:**
```
For each of 12 hours:
  - Generic: is_hoang_dao (existing)
  - Can Chi giờ: compute via Ngũ Thử Độn Thời (hour_pillar.rs)
  - Thập Thần: Can giờ → Nhật Chủ
  - Xung/Hợp: Chi giờ × Chi trụ giờ sinh
  - Element: Ngũ Hành giờ vs mệnh thiếu/thừa
  - Composite score: weighted combination
```

**Output:** `PersonalHourMatrix` — 12 rows × score + reasons

**Key files:**
- `almanac/hour_pillar.rs` — compute_hour_pillar
- `gio_hoang_dao.rs` — existing hour quality

### Matrix 4: Direction Merge + Domain-Day Boost
**What:** Unified direction score (8 hướng) + which life domains get boosted today.

**Direction Merge:**
```
For each of 8 directions:
  - Kua: favorable/unfavorable (static, from tu_menh)
  - Tài Thần hôm nay (dynamic)
  - Hỷ Thần hôm nay (dynamic)
  - Phúc Thần hôm nay (dynamic, NEW)
  - Sát Phương hôm nay (dynamic, NEW)
  - Composite: count of favorable/unfavorable signals
```

**Domain-Day Boost:**
```
For each of 5 domains (career/wealth/relationship/health/timing):
  - Base: Bazi domain score (existing)
  - Day modifier: sao tốt → activity mapping → domain relevance
  - Trực modifier: quality → domain weight
  - Thần modifier: Hoàng Đạo/Hắc Đạo → domain weight
  - Yearly Hạn modifier: if active, reduce confidence
  - Composite: base × (1 + sum of modifiers)
```

**Output:** `DirectionMergeMatrix` (8 rows) + `DomainDayBoostMatrix` (5 rows)

## Architecture

```
crates/amlich-core/src/
  interaction/           ← NEW module
    mod.rs
    day_person.rs        ← Matrix 1
    element_resonance.rs ← Matrix 2
    personal_hour.rs     ← Matrix 3
    direction_merge.rs   ← Matrix 4a
    domain_day_boost.rs  ← Matrix 4b
    types.rs             ← shared structs

crates/amlich-api/src/
  dto.rs                 ← add matrix DTOs
  lib.rs                 ← add personal day matrix surface
```

## Input Requirements

All matrices require:
- `DayContext` (or `DayFortune`) — the day's almanac data
- `BaziChart` — the person's 4 pillars (requires full birth datetime)
- `BaziAnalysisReport` — element distribution, day master strength

Some matrices additionally need:
- `BaziComputedMetrics` — domain scores (Matrix 4b)
- `KuaResult` — direction preferences (Matrix 4a)
- `YearlyHanAssessment` — affliction status (Matrix 4b)

## Implementation Order

1. **Matrix 1 (Day-Person)** — simplest, reuses existing xung_hop + thap_than
2. **Matrix 3 (Personal Hour)** — builds on Matrix 1 pattern
3. **Matrix 2 (Element Resonance)** — reuses existing scoring matrices
4. **Matrix 4a (Direction Merge)** — straightforward aggregation
5. **Matrix 4b (Domain-Day Boost)** — most complex, needs all prior data

## Open Questions for Implementation

1. Should matrices live in `amlich-core/src/interaction/` (new top-level module) or under `almanac/`?
2. Should the API expose individual matrices or a single unified "personal day matrix report"?
3. How to handle partial birth data? (e.g., only birth year → can compute yearly_han but not full Bazi chart for Matrix 1-3)
