# Enriched Insight System Design

**Date:** 2026-03-04
**Status:** Approved
**Goal:** Expand the DayInsightDto to surface all computed almanac subsystems with bilingual interpretive text, driven by a user profile for birth-dependent features.

---

## Problem

The insight system (`DayInsightDto`) only surfaces basic cultural data (Can Chi meanings, day guidance, festivals, Tiet Khi seasonal info) from static JSON files. Meanwhile, the core engine has 10+ computed subsystems (Ten Gods, Na Am, Truc, Stars, Taboos, Day Deity, Travel Directions, Xung Hop, Tang Can, Hour Pillars, Kua, Dai Van) that are not exposed through the insight layer.

## Approach

**Approach A: Merge Fortune into Insight** — Expand `DayInsightDto` with optional fields for each computed subsystem, paired with bilingual interpretive text from new JSON data files. Birth-dependent features (Kua, Dai Van) are populated only when a user profile is configured.

---

## Section 1: User Profile Config

New file: `~/.config/amlich/profile.json`

```json
{
  "birth_year": 1990,
  "birth_month": 5,
  "birth_day": 15,
  "gender": "male"
}
```

- All fields optional — missing fields mean birth-dependent insight sections are omitted
- CLI management: `amlich config profile set --birth-year 1990 --gender male`
- CLI display: `amlich config profile show`
- Loaded lazily, same pattern as existing `read_mode()`

---

## Section 2: Enriched DayInsightDto

```rust
pub struct DayInsightDto {
    // === EXISTING ===
    pub solar: SolarDto,
    pub lunar: LunarDto,
    pub festival: Option<FestivalInsightDto>,
    pub holiday: Option<HolidayInsightDto>,
    pub canchi: Option<CanChiInsightDto>,
    pub day_guidance: Option<DayGuidanceDto>,
    pub tiet_khi: Option<TietKhiInsightDto>,

    // === NEW: Day-only (always populated) ===
    pub na_am: Option<NaAmInsightDto>,         // Element name + meaning
    pub truc: Option<TrucInsightDto>,          // Duty officer + quality + interpretation
    pub day_deity: Option<DayDeityInsightDto>, // Hoang Dao/Hac Dao + what it means
    pub stars: Option<StarsInsightDto>,        // Cat tinh/Sat tinh summary
    pub taboos: Option<Vec<TabooInsightDto>>,  // Active taboo warnings with reasons
    pub travel: Option<TravelInsightDto>,      // Directions + interpretations
    pub xung_hop: Option<XungHopInsightDto>,   // Conflict/harmony summary
    pub tang_can: Option<TangCanInsightDto>,   // Hidden stems + interpretation
    pub ten_gods: Option<TenGodsInsightDto>,   // Day stem relations + meaning
    pub hours: Option<HoursInsightDto>,        // Auspicious hours with detail

    // === NEW: Birth-dependent (from profile.json) ===
    pub tu_menh: Option<TuMenhInsightDto>,     // Kua group, directions, meaning
    pub dai_van: Option<DaiVanInsightDto>,     // Current luck cycle position
}
```

Each `*InsightDto` includes computed data from core + bilingual `meaning`/`interpretation` from JSON data files. All new fields use `Option<T>` with `skip_serializing_if = "Option::is_none"` for backward compatibility.

Example:
```rust
pub struct TrucInsightDto {
    pub name: String,               // "Kien"
    pub quality: String,            // "cat" / "hung"
    pub meaning: LocalizedTextDto,  // What this Truc implies
    pub activities: DayGuidanceDto, // Good for / avoid for
}
```

---

## Section 3: New Bilingual Data Files

New JSON files in `crates/amlich-core/data/`:

| File | Content |
|------|---------|
| `truc-insight.json` | 12 Truc officers: meaning, good-for/avoid-for (vi/en) |
| `stars-insight.json` | Cat tinh & sat tinh descriptions, day star interpretations (vi/en) |
| `day-deity-insight.json` | Hoang Dao / Hac Dao meaning and guidance (vi/en) |
| `na-am-insight.json` | 30 Na Am pair interpretations: element + nature (vi/en) |
| `ten-gods-insight.json` | 10 God names with personality/energy descriptions (vi/en) |
| `taboo-insight.json` | Enriched taboo descriptions beyond current `reason` (vi/en) |
| `travel-insight.json` | Direction meanings and guidance (vi/en) |
| `tang-can-insight.json` | Hidden stem interpretations per Chi (vi/en) |
| `tu-menh-insight.json` | 8 Kua descriptions, East/West group meanings (vi/en) |
| `dai-van-insight.json` | Luck cycle phase descriptions, transition guidance (vi/en) |

Each follows the existing pattern: `include_str!` at compile time, `OnceLock` lazy load, `LocalizedTextDto` structure.

---

## Section 4: Insight Builder Wiring

Data flow for enriched `get_day_insight()`:

```
get_day_insight(query)
  +-- get_day_info(query)          -> DayFortune (computed)
  +-- load static insight data     -> meanings/interpretations (JSON)
  +-- load user profile            -> birth_year, gender (if configured)
  +-- merge: computed + interpret  -> enriched *InsightDto fields
  +-- if profile: compute Kua      -> TuMenhInsightDto
  +-- if profile: compute Dai Van  -> DaiVanInsightDto
```

CLI changes:
- `amlich insight` output gains new sections automatically
- `amlich config profile set/show` new subcommands
- TUI insight overlay expanded with new tabs or sections

---

## Section 5: Testing Strategy

1. **Data file validation** — Schema checks for all new JSON files
2. **Insight DTO contract tests** — Serialize/deserialize roundtrip for every new `*InsightDto`
3. **Integration tests** — `get_day_insight()` with known dates, verify new fields populated
4. **Profile tests** — Missing profile -> birth fields are None; valid profile -> Kua/Dai Van populated
5. **Bilingual coverage** — Every insight entry has both `vi` and `en` non-empty

No golden dataset changes needed — additive layer on top of already-verified computations.

---

## Key Decisions

| Decision | Rationale |
|----------|-----------|
| Merge fortune into insight DTO | Single unified "day insight" response, clean API |
| User profile at ~/.config/amlich/profile.json | Follows existing config pattern (bookmarks.json), familiar location |
| All new fields Option<T> | Backward compatible, birth-dependent fields omitted when no profile |
| Separate JSON data files per subsystem | Maintainable, follows existing pattern (canchi.json, tiet-khi.json) |
| include_str! + OnceLock | Zero runtime I/O, consistent with existing data loading |

---

## Scope Boundaries

**In scope:**
- User profile config (read/write/CLI)
- All 10 day-only insight subsystems
- 2 birth-dependent insight subsystems (Kua, Dai Van)
- New bilingual JSON data files
- CLI `insight` command enrichment
- TUI insight overlay expansion
- Contract and integration tests

**Out of scope:**
- LLM/AI integration
- Desktop app (Tauri/Svelte) changes
- WASM binding changes
- New golden dataset entries
- Enhanced Xung Hop (Luc Hop, Tuong Hai, Tuong Hinh) — separate milestone
