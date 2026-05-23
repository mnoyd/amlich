# Architecture Research: v1.5 Eastern Knowledge Expansion (P1 Văn khấn + P4 Phi Tinh thời gian)

**Domain:** Vietnamese Almanac — ritual content lookup + time-based Flying Stars
**Researched:** 2026-05-23
**Confidence:** HIGH for both pillars (every integration point grounded in current source).

## Executive Summary

v1.5 adds two **content-and-lookup-shaped** pillars to the existing `amlich-core` Rust workspace. Neither requires algorithmic novelty: P1 Văn khấn is a corpus-with-rules pillar (lookup by event/date → ritual entries) and P4 Phi Tinh thời gian is a finite-table pillar (Vận/Năm/Tháng → 3×3 palace assignment of the nine stars). Both are **Tier 0** (no birth data) so they must be reachable from `DaySnapshot` alone.

Architectural shape:

1. **P1 Văn khấn** — new sibling crate-internal module `crates/amlich-core/src/rituals/`, JSON corpus under `crates/amlich-core/data/rituals/`, embedded via `include_str!` + `OnceLock` (same as `golden_loader.rs`). Holiday integration is **read-only**: rituals discover events from existing `holidays::Holiday.name + category + lunar_date`. Holiday module is NOT modified in v1.5 — the ritual module imports holidays, not vice-versa, to keep the dependency arrow one-way and the v1.0–v1.4 surface frozen.
2. **P4 Phi Tinh thời gian** — new submodule **folder** `crates/amlich-core/src/almanac/fengshui/` containing `mod.rs` + `flying_stars.rs`. Use a folder (not a single file) because (a) Huyền Không has multiple table layers (Vận base palace, yearly, monthly, daily) and (b) the §3.3 Tier-3 expansion will add `spatial_compose` siblings without restructuring. Base palace tables stay as `const` Rust arrays (Vận 1–9 is a fully-known finite set of 9 × 9 = 81 cells with stable mathematical structure); the JSON file `crates/amlich-core/data/almanac/flying_stars.json` holds only star metadata (name, element, polarity, default interpretation) — i.e. the part that humans actually edit.
3. **Boundary with existing direction modules** is crystal clear once stated:
   - `sat_phuong.rs` / `than_huong.rs` / `thai_tue.rs` answer **"which cardinal direction is auspicious / forbidden TODAY?"** Output is a *direction string* tagged by `source_id = "khcbppt"`.
   - `flying_stars.rs` answers **"which star occupies each of the 9 palaces in the Lo Shu grid for this time period?"** Output is a *palace-to-star map* (9 cells) tagged by `source_id = "huyen-khong"`. It is a *spatial layout descriptor*, not a single direction recommendation.
   - There is zero overlap because the output cardinality differs (1 direction vs 9 palaces) and the source traditions are disjoint (Hiệp Kỷ vs Thẩm Thị Huyền Không).
4. **Semantic graph** receives two new `NodeConcept`s (`Ritual`, `FlyingStar`) and at minimum two new `EdgeConcept`s (`PrescribedFor`, `OccupiesPalace`). The `EventType` concept is *not* a new node — events are reused from existing `Holiday` records (festival id) and a small ritual-only `RitualEventKey` enum embedded inside ritual node payload. This avoids polluting the ontology with a 2nd holiday-shaped node kind.
5. **Build order:** schema lock for ritual JSON → ritual corpus + module → ritual semantic graph wiring → Phi Tinh tables → Phi Tinh module + tests → Phi Tinh semantic graph wiring → snapshot integration (both pillars read-only consumers of `DaySnapshot`). Schema lock first because the JSON shape gates corpus production cost; once locked, corpus authoring and Phi Tinh table work parallelise.

**Key architectural principle (carried from v1.0–v1.4):** Additive-only. No existing public type changes. No existing JSON files mutate. No existing `source_id` reused. Every new node carries a `ProvenanceEntry { source: AlmanacRule, source_id: "vn-folk-ritual" | "huyen-khong", method, ... }`.

---

## Current Architecture Context (verified against source)

### Module hierarchy

`crates/amlich-core/src/lib.rs:10-26` — top-level modules already include `almanac`, `holidays`, `holiday_data`, `interaction`, `reasoning`, `semantic_graph`. Adding `pub mod rituals;` is one additive line; adding `pub mod almanac::fengshui` requires registering a submodule inside `almanac/mod.rs:1-28`.

`crates/amlich-core/src/almanac/mod.rs:1-28` — flat list of 27 calculator submodules. Pattern: each calculator gets its own file at this level. Phi Tinh deviates intentionally by introducing a **folder** because the long-term §3.3 plan needs multiple sibling files (`flying_stars.rs`, future `nine_palaces.rs`, future `spatial_compose.rs`).

### Provenance & evidence

`crates/amlich-core/src/semantic_graph/provenance.rs:65-67` — `Provenance::almanac_rule(source_id, method)` is the constructor both new pillars use. Both `vn-folk-ritual` and `huyen-khong` flow through this exact path because they are *almanac-rule* in nature (date-driven content / table lookups), not interaction outputs.

`reasoning/personal.rs:176-192` — pattern for emitting `ReasoningEvidenceEnvelope` shows the simple struct shape (source_family + source_id + method + note). New pillars use `ReasoningEvidenceSourceFamily::AlmanacRule` (already exists, no enum extension needed).

### Holiday detection (P1 dependency, read-only)

`holidays.rs:14-25` — `Holiday { name, lunar_date, solar_day/month/year, is_solar, category, is_major }`. Categories observed in current code: `"festival"`, `"social"`, `"lunar-cycle"` (lines 179, 210, 237). Mùng 1 (lunar day 1) and Rằm (lunar day 15) holidays are auto-generated at `holidays.rs:228-260` with `category = "lunar-cycle"` — this is the **natural join key** for monthly Sóc/Vọng ritual lookups.

`crates/amlich-core/data/holidays/lunar-festivals.json` — each festival has stable `id` field (e.g. `"tet-nguyen-dan"`). P1 corpus references these IDs in its `event_keys` array, never the localised name string. This survives translation/rename of `names.vi`.

### Direction modules (P4 boundary check)

`almanac/sat_phuong.rs:1-104` — input `chi_index: usize` → output `SatPhuongResult { direction: String }`. **One direction, daily resolution, source = KHCBPPT.**

`almanac/than_huong.rs:1-117` — input `can: &str` → output `TravelDirection { xuat_hanh_huong, tai_than, hy_than }`. **Three named directions, daily resolution, source = Khâm Định Hiệp Kỷ Biện Phương Thư.**

`almanac/thai_tue.rs:53-112` — input `(birth_chi_index, current_year_chi_index)` → output `ThaiTueResult { conflicts: Vec<…> }`. **Conflict list, yearly resolution, source = KHCBPPT + folk.** Note: this is technically Tier 1 (uses birth year) — but does NOT take spatial input.

**Confirmed:** none of these modules produce a 9-palace layout. None use a Vận period. Phi Tinh has zero output overlap.

### Semantic graph ontology

`semantic_graph/ontology.rs:5-40` — current `NodeConcept` enum has 34 variants (DayCanchi, Truc, Direction, Recommendation, Taboo, etc.). Extensions are additive single-line entries inside the enum + `label()` match + the static `node_concepts()` slice at line 278.

`semantic_graph/ontology.rs:85-111` — `EdgeConcept` enum has 25 variants. Same pattern for additions.

### JSON loading convention

`almanac/golden_loader.rs:5-21` — pattern: `const X_JSON: &str = include_str!("../../data/...");` + `static X: OnceLock<T> = OnceLock::new();` + `pub fn load_x() -> &'static T { X.get_or_init(|| { … validate(&dataset); dataset }) }`. Both new pillars **must** follow this exact pattern (panic on validation failure — the data is a test oracle, not user-faceable).

---

## P1 Văn khấn — Module Design

### 1.1 Module location

**Decision:** `crates/amlich-core/src/rituals/` (new top-level submodule), NOT under `almanac/`.

**Rationale:**
- `almanac/` is reserved for **rule calculators** that consume a day's Can-Chi/Tiết-Khí/lunar coordinates and emit *computed* almanac outputs (truc, deity, na am, …). Văn khấn is a **content corpus retriever** — it does not compute, it filters and returns prose. Conceptually closer to `holiday_data.rs` (which is also a top-level submodule) than to `truc.rs`.
- Future ritual sub-pillars (lễ vật trình tự, văn tế gia tiên, văn cúng đầu năm) belong here. Putting them under `almanac/` would corrupt the "almanac = computed" invariant that v1.0–v1.4 carefully maintained.
- The EXPANSION_FRAMEWORK at line 65 explicitly proposes `crates/amlich-core/src/rituals/` — this matches.

**Module layout:**

```
crates/amlich-core/src/rituals/
├── mod.rs              # public API: find_van_khan, list_events_on, RitualEntry types
├── corpus.rs           # OnceLock + include_str! + validation (mirrors golden_loader)
├── event_match.rs      # day → Vec<RitualEventKey> resolver (joins holidays.rs)
├── types.rs            # RitualEntry, RitualEventKey, LunarDateMatch, RitualMetadata
└── tests.rs            # golden coverage tests
```

### 1.2 JSON corpus structure

**Decision:** **One file per event category, plus a manifest.** Not one big file. Not one per individual ritual.

Layout under `crates/amlich-core/data/rituals/`:

```
data/rituals/
├── manifest.json                       # registry + corpus-level metadata
├── tet_nguyen_dan.json                 # all rituals for Tết
├── soc_vong_monthly.json               # Mùng 1 / Rằm monthly cycles
├── thanh_minh.json
├── trung_thu.json
├── vu_lan.json
├── ong_cong_ong_tao.json               # 23 tháng Chạp
├── thuong_nguyen.json                  # Rằm tháng Giêng
├── doan_ngo.json
├── ha_nguyen.json                      # Rằm tháng Mười
├── household_general.json              # Cúng gia tiên generic, không gắn event
├── life_events_dong_tho.json           # Động thổ (groundbreaking)
├── life_events_cuoi_hoi.json
├── life_events_khai_truong.json
└── life_events_an_tang.json
```

**Why one-per-event-category:**
- Each file is small enough to review in one diff (typically 5–20 entries), large enough to provide useful context (related variants, regional differences).
- File boundaries match the **authoring & verification unit** — a contributor cross-checks one event family against the source book at a time.
- Avoids the giant-file merge-conflict trap (one 50KB file = constant rebase pain).
- `manifest.json` provides O(1) discovery without scanning all files.

**Frontmatter / per-entry schema** (locked in this milestone — gates corpus authoring):

```jsonc
{
  "$schema_version": "rituals-v1",
  "source_id": "vn-folk-ritual",
  "source_citation": {
    "title": "Văn khấn cổ truyền Việt Nam",
    "publisher": "NXB Văn Hóa Dân Tộc",
    "edition": "2018",
    "page": "115-118"
  },
  "category": "festival" | "lunar-cycle" | "life-event" | "ancestor" | "deity-worship",
  "entries": [
    {
      "ritual_id": "van-khan-giao-thua",                 // stable, kebab-case, globally unique
      "title_vi": "Văn khấn Giao thừa ngoài trời",
      "title_en": "New Year's Eve outdoor invocation",
      "event_keys": [                                    // any-of match — empty list means "always available"
        { "kind": "holiday_id", "value": "tet-nguyen-dan" },
        { "kind": "lunar_date", "month": 12, "day": 30, "leap_ok": false },
        { "kind": "lunar_date", "month": 1,  "day": 1,  "leap_ok": false }
      ],
      "time_of_day": "giao-thua" | "morning" | "noon" | "afternoon" | "evening" | "any",
      "offerings": [
        { "vi": "Mâm ngũ quả", "en": "Five-fruit tray" },
        ...
      ],
      "preparation_steps": [
        { "vi": "Bày bàn thờ hướng ra ngoài cửa", "en": "Set the altar facing outward" },
        ...
      ],
      "invocation_text_vi": "...full prayer body...",
      "invocation_text_en_summary": "...short English gloss, NOT a translation...",
      "notes": [],
      "confidence": "primary" | "regional-variant" | "synthesized"
    }
  ]
}
```

**Schema-locking decisions (must be ADR'd before corpus authoring starts):**

- `event_keys` is an **any-of** list of typed match clauses. This is the load-bearing piece: it must support (a) named holiday ids, (b) raw lunar dates (Mùng 1, Rằm of every month), (c) tiet-khi anchors (Thanh Minh, Đông Chí), and (d) life-event tags (always-available, picked manually by app). Treating it as a typed enum keeps the matcher exhaustive in Rust.
- `confidence` is REQUIRED. Per §7 of EXPANSION_FRAMEWORK: cross-check ≥ 2 sources; mark divergences instead of "fixing" them.
- `invocation_text_en` is intentionally a *gloss summary*, not a literal translation. Translating ritual prayer text into English is out of scope and frankly risky (different liturgical register). Locked decision: VN-only liturgy, EN summary.
- NO `valid_year_range` field. Văn khấn is timeless; year-gating belongs in the holiday detector, not the ritual entry.

### 1.3 API shape

```rust
// crates/amlich-core/src/rituals/types.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RitualEventKey {
    HolidayId(String),                            // matches holidays/*.json `id` field
    LunarDate { month: i32, day: i32, leap_ok: bool },
    TietKhiAnchor(String),                        // matches tietkhi name
    LifeEvent(LifeEventKind),                     // Dong Tho, Cuoi Hoi, Khai Truong, An Tang
    Always,                                       // household_general entries
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RitualEntry {
    pub ritual_id: String,
    pub title_vi: String,
    pub title_en: String,
    pub category: RitualCategory,
    pub event_keys: Vec<RitualEventKey>,
    pub time_of_day: TimeOfDay,
    pub offerings: Vec<BilingualText>,
    pub preparation_steps: Vec<BilingualText>,
    pub invocation_text_vi: String,
    pub invocation_text_en_summary: String,
    pub notes: Vec<BilingualText>,
    pub confidence: RitualConfidence,
    pub source_citation: SourceCitation,
}

// crates/amlich-core/src/rituals/mod.rs — public API

pub fn find_van_khan_for_snapshot(snapshot: &DaySnapshot) -> Vec<&'static RitualEntry>;
pub fn find_van_khan_for_event(event: &RitualEventKey) -> Vec<&'static RitualEntry>;
pub fn find_van_khan_for_life_event(kind: LifeEventKind) -> Vec<&'static RitualEntry>;
pub fn get_ritual_by_id(ritual_id: &str) -> Option<&'static RitualEntry>;
pub fn all_rituals() -> &'static [RitualEntry];
```

**API shape choices:**

- **NOT** `find_van_khan(date, event_type)` — that signature is fragile (what's the type of `event_type`? a free string?). The `snapshot`-based call is canonical because it lets the matcher consult `holidays`, `lunar_date`, and `tiet_khi` in one place. The `_for_event` and `_for_life_event` variants give callers explicit control when they want a single event (e.g. UI shows "rituals for Tết Nguyên Đán").
- **Return type is `&'static`.** Corpus is `OnceLock`-loaded and never mutated. This matches `golden_loader.rs` and `holiday_data.rs:lunar_festivals()` patterns.
- **Vec, not Option.** Multiple rituals routinely apply to one day (e.g. Giao Thừa indoor + outdoor + ông Công ông Táo wrap-up). Ranking/filtering is the caller's job.

### 1.4 Holiday integration — read-only consumer pattern

**Decision:** Rituals depend on `holidays`; `holidays` does NOT depend on rituals. Holidays module stays untouched in v1.5.

**Why one-way:**
- Keeps the v1.0–v1.4 surface frozen (PROJECT.md "Additive-only integration changes").
- The "holiday detection auto-recommends prayers" UX lives in the **caller layer** (CLI/desktop), not inside `holidays.rs`. The library exposes both APIs side-by-side; the app composes them.
- A future v1.6 could introduce a thin façade `compute_day_with_rituals()` in `lib.rs` if needed without touching `holidays.rs`.

**Concrete integration recipe:**

```rust
// In rituals/event_match.rs

fn resolve_event_keys_for_day(snapshot: &DaySnapshot) -> Vec<RitualEventKey> {
    let mut keys = Vec::new();
    let solar_year = snapshot.context.solar.year;

    // 1. Named holidays via crate::holidays::get_vietnamese_holidays(solar_year)
    //    Filter to those whose (solar_day, solar_month, solar_year) match snapshot.context.solar.
    //    Map name → holiday_id by joining against holiday_data::lunar_festivals().
    //    (Note: `Holiday` struct does NOT carry `id` today — see "Modified files" below.)

    // 2. Raw lunar Mùng 1 / Rằm from snapshot.context.lunar.day
    if snapshot.context.lunar.day == 1 {
        keys.push(RitualEventKey::LunarDate { month: snapshot.context.lunar.month, day: 1, leap_ok: true });
    }
    if snapshot.context.lunar.day == 15 {
        keys.push(RitualEventKey::LunarDate { month: snapshot.context.lunar.month, day: 15, leap_ok: true });
    }

    // 3. Tiết Khí anchor (Thanh Minh, Đông Chí, Hạ Chí, …)
    keys.push(RitualEventKey::TietKhiAnchor(snapshot.context.tiet_khi.name.clone()));

    keys
}
```

**Required tiny modification to `holidays.rs`:** add a stable `id: Option<String>` field to `Holiday` so the ritual matcher can join by id rather than by Vietnamese display name (which is fragile — `"Mùng 1 tháng 3"` is a generated label, not a stable key). The `lunar_festivals()` source data already has `id` (e.g. `"tet-nguyen-dan"`); it just isn't propagated into the `Holiday` struct today. This is the only existing file modification in v1.5.

### 1.5 Semantic graph wiring (P1)

**New `NodeConcept` (one):** `Ritual` — a node that points at an applicable ritual for the day.

**Reused `NodeConcept`s:** `DayCanchi` (existing), `SolarTerm` (existing). No `EventType` node — the event match key is stored *inside* the Ritual node's payload, not as a separate node, because it has no behavior of its own and creating cardinality-12 event nodes for each Mùng-1/Rằm would explode the graph.

**New `EdgeConcept`s (two):**
- `PrescribedFor` — `Ritual --PrescribedFor--> DayCanchi` (or `SolarTerm` when matched via tiet khi). Direction: ritual is *prescribed for* this day's context.
- `RecommendsOffering` — currently no offering node exists; defer to v1.6. v1.5 keeps offerings inside the ritual node payload as a flat string list.

**Provenance contract:**

```rust
ProvenanceEntry::almanac_rule("vn-folk-ritual", "rituals.find_van_khan_for_snapshot")
    .with_note(format!("matched via {match_kind}"))
```

The `source_id = "vn-folk-ritual"` is a NEW id (DEC-0015 discipline — not `vn-folk`, which is in use by Hoàng Ốc). Add to the source taxonomy memory doc.

---

## P4 Phi Tinh thời gian — Module Design

### 2.1 Folder vs single file

**Decision:** Folder `crates/amlich-core/src/almanac/fengshui/` with explicit `mod.rs`.

**Rationale:**
- EXPANSION_FRAMEWORK §2.3 already calls out two files inside the namespace: `almanac/fengshui/flying_stars.rs` (this milestone) and future `interaction/spatial_compose.rs` (Tier 3, deferred). Even within the time-only scope of v1.5, the Phi Tinh code splits naturally into ≥ 3 files:
  ```
  almanac/fengshui/
  ├── mod.rs                    # re-exports + glue
  ├── lo_shu.rs                 # the Lo Shu 3×3 grid abstraction (palace ↔ direction mapping)
  ├── flying_stars.rs           # Vận / yearly / monthly star computation
  ├── star_meta.rs              # star metadata loader (name, element, polarity)
  └── tests/                    # golden tests
  ```
- The Lo Shu grid is a **reusable primitive**. Other future feng-shui modules (Bát Trạch's 8 zones, Cửu Cung) will reuse it. Burying it inside `flying_stars.rs` would force a later refactor.

### 2.2 Tables: `const` arrays vs JSON

**Decision: HYBRID.**

| Data | Form | Why |
|---|---|---|
| Vận 1–9 base palace tables (which star sits in center for each Vận) | **`const` Rust array `[u8; 9]`** | Mathematically determined (Lo Shu permutation rotated by Vận). 9 vận × 1 center = 9 numbers. Never edited. Encoding as JSON would lie about its derived nature. |
| Yearly star (年紫白): which star is center for year N | **`const` table or pure function** | Closed-form: `center_star = 11 - (year - 1864) % 9` for upper men (上元) with full formula spanning 上中下元. Pure function, no JSON. |
| Monthly star (月紫白): which star is center for lunar month M of year-stem-branch | **`const` 24-element lookup table** | Driven by year-branch group (4 groups × 12 months = 48 entries, but most groups collapse). |
| Daily star (日紫白) | **DEFERRED to v1.6** | Adds complexity (冬至/夏至 reversal); not in EXPANSION_FRAMEWORK §2.3 scope for "Tier 0". |
| Star metadata: name (一白貪狼水), element, polarity, default polarity, interpretation text | **JSON: `data/almanac/flying_stars.json`** | Human-edited, bilingual, citation-bearing. Exactly the case `golden_loader.rs` solves. |

**Why not all-JSON:** The Vận tables are not data — they are a deterministic permutation. Putting them in JSON invites typos that the type system can't catch, and forces a runtime load for what should be a `const`. The metadata IS data (different sources use slightly different star names, different element associations), so it goes to JSON where citations live.

**Why not all-const:** Interpretation text is exactly the kind of content that the corpus contributor edits without recompiling. Forcing it into `const &str` arrays would couple data edits to Rust releases.

### 2.3 API shape

```rust
// crates/amlich-core/src/almanac/fengshui/flying_stars.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FlyingStar {
    NhatBach = 1, NhiHac = 2, TamBich = 3,
    TuLuc = 4,   NguHoang = 5, LucBach = 6,
    ThatXich = 7, BatBach = 8, CuuTu = 9,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Palace {
    Center, North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlyingStarLayout {
    pub period: FlyingStarPeriod,        // Yearly | Monthly | Van
    pub period_index: i32,               // year number, lunar month, or vận number
    pub palaces: [FlyingStar; 9],        // index = Palace as usize
    pub center_star: FlyingStar,
    pub evidence: RuleEvidence,          // source_id = "huyen-khong"
}

pub fn compute_van_layout(van: u8) -> FlyingStarLayout;            // Vận 1..=9
pub fn compute_yearly_flying_stars(year: i32) -> FlyingStarLayout;
pub fn compute_monthly_flying_stars(lunar_year_branch_index: usize, lunar_month: i32) -> FlyingStarLayout;

// Convenience: snapshot-based
pub fn compute_flying_stars_for_snapshot(snapshot: &DaySnapshot) -> FlyingStarsForDay;

#[derive(Debug, Clone)]
pub struct FlyingStarsForDay {
    pub van: FlyingStarLayout,        // current Vận 9 (2024-2043)
    pub yearly: FlyingStarLayout,
    pub monthly: FlyingStarLayout,
}
```

**API shape choices:**
- **`[FlyingStar; 9]`** not `[u8; 9]` (the question proposed `[u8; 9]`). Typed enum prevents callers from mistreating star numbers as raw bytes; serde gives identical JSON shape (1..=9) for free.
- **`Palace as usize` indexing** with a fixed order (Center, N, NE, E, SE, S, SW, W, NW) — this order matches the Lo Shu number canonical layout (5 center, 1 N, 8 NE, 3 E, 4 SE, 9 S, 2 SW, 7 W, 6 NW). Document this loudly in `lo_shu.rs` so callers don't index by direction-name string.
- **Three separate `FlyingStarLayout` outputs** (Vận / yearly / monthly) rather than one merged 9×3 grid — they are semantically different time scales and combining them obscures which scale a given star comes from.

### 2.4 Boundary statement (load-bearing — must be in module docs)

Document this verbatim at the top of `almanac/fengshui/mod.rs`:

```rust
//! # Boundary with other direction-bearing almanac modules
//!
//! This module computes **9-palace star layouts** for time periods (Vận / Năm /
//! Tháng). Its output is a *spatial assignment* — which of the nine stars sits
//! in each of the nine palaces (8 cardinals + center) of the Lo Shu grid for
//! the queried period.
//!
//! This is **disjoint from** the following existing modules:
//!
//! - `almanac::sat_phuong`  — one "killing direction" per day (input: day chi).
//! - `almanac::than_huong`  — three named directions per day (Xuất hành / Tài /
//!   Hỷ) keyed on day stem. Source: Khâm Định Hiệp Kỷ Biện Phương Thư.
//! - `almanac::thai_tue`    — yearly conflict list keyed on year branch +
//!   birth-year branch. Source: KHCBPPT + folk.
//!
//! These modules answer "which compass direction is auspicious today?". This
//! module answers "what is the time-period's Lo Shu palace layout?".
//! They are NOT alternatives. They are NOT substitutable. Their `source_id`s
//! are disjoint (`khcbppt` vs `huyen-khong`). Composing them is the job of
//! `interaction::direction_merge` (existing) and the future
//! `interaction::spatial_compose` (Tier 3, deferred).
```

This docstring is the operational definition of "no overlap, no duplication" that the quality gate demands.

### 2.5 Semantic graph wiring (P4)

**New `NodeConcept` (one):** `FlyingStar`. One node per (period, palace) pair that the calling builder decides to materialize (usually 9 cells × 3 period layers = 27 nodes per day's snapshot — but builders may choose only Vận and yearly for compact graphs).

**Reused `NodeConcept`s:** `Direction` (existing — each Palace except Center maps to a Direction). `Element` (existing — each star has an element association).

**New `EdgeConcept`s (two):**
- `OccupiesPalace` — `FlyingStar --OccupiesPalace--> Direction` (the Palace's compass-direction node).
- `CarriesElement` — `FlyingStar --CarriesElement--> Element`. Reuses the existing `Element` node.

**Provenance:**

```rust
ProvenanceEntry::almanac_rule("huyen-khong", "fengshui.compute_yearly_flying_stars")
    .with_note(format!("center_star={center}, year={year}"))
```

Add `huyen-khong` to the source taxonomy memory doc.

---

## Integration Points (at file:line granularity)

### New files

| Path | Purpose |
|---|---|
| `crates/amlich-core/src/rituals/mod.rs` | Public ritual API |
| `crates/amlich-core/src/rituals/corpus.rs` | OnceLock-backed corpus loader (mirrors `golden_loader.rs`) |
| `crates/amlich-core/src/rituals/event_match.rs` | Day → event-key resolver |
| `crates/amlich-core/src/rituals/types.rs` | `RitualEntry`, `RitualEventKey`, `LifeEventKind`, … |
| `crates/amlich-core/src/rituals/tests.rs` | Golden coverage tests |
| `crates/amlich-core/src/almanac/fengshui/mod.rs` | Sub-folder root + boundary docstring |
| `crates/amlich-core/src/almanac/fengshui/lo_shu.rs` | Lo Shu grid primitive (Palace enum, direction mapping) |
| `crates/amlich-core/src/almanac/fengshui/flying_stars.rs` | Vận/yearly/monthly computations |
| `crates/amlich-core/src/almanac/fengshui/star_meta.rs` | Star metadata loader |
| `crates/amlich-core/data/rituals/manifest.json` | Corpus manifest |
| `crates/amlich-core/data/rituals/*.json` | Per-event-category corpora (~14 files) |
| `crates/amlich-core/data/almanac/flying_stars.json` | Star metadata + interpretations |

### Modified files (additive only)

| Path:line | Change | Risk |
|---|---|---|
| `crates/amlich-core/src/lib.rs:11` (alphabetical) | Add `pub mod rituals;` | None — pure addition |
| `crates/amlich-core/src/lib.rs:36-50` re-exports block | Add `pub use crate::rituals::{find_van_khan_for_snapshot, RitualEntry, ...};` | None |
| `crates/amlich-core/src/lib.rs:41` (sorted with other almanac re-exports) | Add `pub use crate::almanac::fengshui::{compute_yearly_flying_stars, FlyingStarLayout, FlyingStar, Palace, ...};` | None |
| `crates/amlich-core/src/almanac/mod.rs:1-28` | Add `pub mod fengshui;` line | None |
| `crates/amlich-core/src/holidays.rs:14-25` (`Holiday` struct) | Add `pub id: Option<String>` field; populate from `lunar_festivals[].id` and from solar_holidays. Default `None` for generated Mùng 1/Rằm entries. | LOW — additive field; existing downstream consumers ignore it; serde with `#[serde(default)]` keeps JSON snapshots compatible |
| `crates/amlich-core/src/holidays.rs:148-198` (creation sites) | Populate the new `id` field from the source JSON's `id` | LOW |
| `crates/amlich-core/src/semantic_graph/ontology.rs:5-40` `NodeConcept` | Add `Ritual`, `FlyingStar` variants + `label()` arms + `node_concepts()` slice entries | LOW — exhaustive match enforced by compiler |
| `crates/amlich-core/src/semantic_graph/ontology.rs:85-111` `EdgeConcept` | Add `PrescribedFor`, `OccupiesPalace`, `CarriesElement` variants + `label()` arms + `edge_concepts()` slice entries | LOW |
| `crates/amlich-core/src/semantic_graph/ontology.rs:145-273` `ConceptLabel` + `as_str()` | Add matching label variants + string forms | LOW |
| `crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs` (or new sibling) | Wire ritual + flying-star nodes into the day snapshot graph build | MEDIUM — first time non-`khcbppt` nodes co-exist in the day graph; verify provenance separation |

### Not modified (deliberately frozen)

| Path | Why kept untouched |
|---|---|
| `interaction/direction_merge.rs` | Tier 0 milestone; spatial merge is Tier 3 (v1.7+) |
| `interaction/personal_hour.rs` | No personal layer in v1.5 |
| `reasoning/personal.rs` | Both pillars are Tier 0; nothing to wire through personal reasoning yet |
| `almanac/calc.rs` (DayFortune assembly) | DayFortune semantics frozen since v1.2; v1.5 outputs surface beside DaySnapshot, not inside DayFortune |
| All existing `almanac/*.rs` calculators | Frozen — no source-tradition cross-contamination |

---

## Semantic Graph Extension Plan

### New nodes (2)

| Node | Concept | Stable id format | Provenance source_id |
|---|---|---|---|
| Ritual | `NodeConcept::Ritual` | `ritual.{ritual_id}` (e.g. `ritual.van-khan-giao-thua`) | `vn-folk-ritual` |
| FlyingStar | `NodeConcept::FlyingStar` | `flying_star.{period}.{period_index}.{palace}` (e.g. `flying_star.yearly.2026.east`) | `huyen-khong` |

### New edges (3)

| Edge | From → To | Concept | Meaning |
|---|---|---|---|
| PrescribedFor | Ritual → DayCanchi or Ritual → SolarTerm | `EdgeConcept::PrescribedFor` | "This ritual is prescribed for this day-coordinate." |
| OccupiesPalace | FlyingStar → Direction | `EdgeConcept::OccupiesPalace` | "This star sits in this compass direction for the queried period." |
| CarriesElement | FlyingStar → Element | `EdgeConcept::CarriesElement` | "This star carries this Five-Element nature." |

### Reused nodes (zero new code in these)

- `DayCanchi` — already produced by `builders/day_snapshot.rs`. Rituals link to it.
- `SolarTerm` — same.
- `Direction` — already used by `direction_merge`. FlyingStars link to existing direction nodes (verify dedup via `ProvenanceTracker` — same Direction node may now carry both KHCBPPT and Huyền Không provenance entries; the tracker handles this with its multi-entry vector at `provenance.rs:130-135`).
- `Element` — same.

### Builder placement

New builder file `crates/amlich-core/src/semantic_graph/builders/expansion_v15.rs` (or extend `day_snapshot.rs` — TBD by orchestrator based on builder-file size budget). Pattern follows existing builders. Critical: emit ProvenanceEntry with the **correct** new source_id; never reuse `khcbppt`.

---

## Build Order (with dependencies)

Each numbered item is a discrete chunk; (deps) lists what must be done first. Items at the same indent without deps can parallelise.

```
1. SCHEMA LOCK (gates everything downstream)
   1a. ADR: ritual JSON schema (event_keys enum, confidence levels, citation shape)
   1b. ADR: source_id additions ("vn-folk-ritual", "huyen-khong") to taxonomy doc
   1c. Add Holiday.id field + propagate from JSON (small, low-risk modification of holidays.rs)

2. P1 VÃN KHÂN MODULE (deps: 1a, 1c)
   2a. rituals/types.rs + rituals/corpus.rs (mirror golden_loader pattern)
   2b. rituals/event_match.rs (joins holidays.rs)
   2c. rituals/mod.rs public API
   2d. Authoring: ritual corpus JSON files (the long-pole work; ≥ 60 entries to start)
   2e. Tests: golden coverage tests, deterministic loader tests

3. P1 SEMANTIC GRAPH WIRING (deps: 2c, ontology additions)
   3a. Add Ritual NodeConcept + PrescribedFor EdgeConcept to ontology.rs
   3b. Builder: ritual node materialization in day_snapshot graph build
   3c. Provenance verification tests

4. P4 PHI TINH PRIMITIVES (deps: 1b; can parallel with 2*)
   4a. almanac/fengshui/lo_shu.rs (Palace enum, Lo Shu canonical ordering, direction mapping)
   4b. almanac/fengshui/star_meta.rs (JSON loader for metadata)
   4c. data/almanac/flying_stars.json authoring

5. P4 PHI TINH COMPUTATION (deps: 4a, 4b)
   5a. almanac/fengshui/flying_stars.rs — Vận layout
   5b. almanac/fengshui/flying_stars.rs — yearly star
   5c. almanac/fengshui/flying_stars.rs — monthly star
   5d. snapshot convenience wrapper
   5e. Golden tests cross-checked with fengshui.net / phongthuyhomemy.com per §7

6. P4 SEMANTIC GRAPH WIRING (deps: 5*, ontology additions)
   6a. Add FlyingStar NodeConcept + OccupiesPalace, CarriesElement EdgeConcept
   6b. Builder: flying-star node materialization
   6c. Provenance separation tests (huyen-khong nodes must NEVER carry khcbppt provenance)

7. INTEGRATION TESTS (deps: 3*, 6*)
   7a. Day snapshot includes ritual + flying-star nodes
   7b. LLM view / debug inspector handles new node concepts cleanly
   7c. End-to-end: 2026 calendar smoke test on at least 30 representative dates
```

**Schema lock first because:** corpus authoring (2d) is the longest-pole item by far (an editorial task with ≥ 60 ritual entries). It cannot start until the schema is locked. If the schema slips after authoring starts, every entry needs revision.

**P1 and P4 parallelisable from step 2 onwards** because they share no code paths. They first re-converge at step 7 (integration) and at the ontology PR (steps 3a + 6a should ideally land in one commit to keep `ConceptLabel` exhaustive matches clean).

**Phi Tinh primitives (4a Lo Shu) precede computation (5)** because the computation depends on a well-defined palace ordering. Authoring `flying_stars.json` (4c) parallelises with 5* once `star_meta.rs` declares the schema.

---

## Risks & Mitigations

| Risk | Severity | Mitigation |
|---|---|---|
| Ritual JSON schema churns after corpus authoring starts | HIGH | Lock schema with ADR (step 1a) before any 2d work. Treat schema as a frozen v1; deprecation requires `$schema_version` bump. |
| `Holiday.id` field break downstream consumers (CLI, desktop) | LOW | Field is `Option<String>`, default `None`, serde `#[serde(default)]`. Existing consumers see no change. |
| Phi Tinh yearly formula 上中下元 boundary errors | MEDIUM | Golden tests at known transition dates (1864/1923/1984/2043). Cross-check ≥ 2 sources per §7. |
| Lo Shu palace ordering disagreement across sources | MEDIUM | Pick Sòng-canon (5 center, 1 N…) and document it in `lo_shu.rs` header. Mark any divergence in metadata JSON as `KnownDivergence`. |
| `source_id` typo silently mints a fake source | HIGH | Add compile-time const `pub const SOURCE_RITUAL: &str = "vn-folk-ritual";` in each module; ban string literals at the `Provenance::almanac_rule(...)` callsite via a lint or test. |
| Semantic graph provenance dedup collapses different sources | MEDIUM | `ProvenanceTracker::track()` (provenance.rs:130) appends to a vector — confirmed it does NOT dedup, so multi-source nodes are safe. Add a test that asserts a Direction node carries BOTH `khcbppt` and `huyen-khong` provenance entries when both apply. |
| Ritual content quality varies by region (Bắc/Trung/Nam) | LOW | `confidence: regional-variant` schema field flags this. Keep regional variants as separate entries; the matcher returns them all; UI ranks. |

---

## Validation References

- **Văn khấn:** *Văn khấn cổ truyền Việt Nam* (NXB Văn Hóa Dân Tộc, 2018). Cross-check ≥ 2 traditional household editions per entry per EXPANSION_FRAMEWORK §7.
- **Phi Tinh:** *Thẩm Thị Huyền Không Học*. Online cross-check: fengshui.net for yearly tables, phongthuyhomemy.com for VN-language interpretation.

Golden test minimum: 10 cases per pillar, ≥ 2 independent sources per case (§7). Divergences logged as `KnownDivergence`, NOT "fixed" toward either source.

---

## Open Questions for Orchestrator / Future Decisions

1. **Builder file size budget.** Should expansion v1.5 nodes live in a new builder file (`builders/expansion_v15.rs`) or extend `builders/day_snapshot.rs`? Either works; depends on team style.
2. **Mùng 1 / Rằm leap-month behavior.** When lunar month is leap, should Sóc/Vọng rituals still apply? Default proposed: `leap_ok: true` in `RitualEventKey::LunarDate`. Confirm during schema-lock ADR.
3. **Daily flying star (日紫白) inclusion.** EXPANSION_FRAMEWORK §2.3 lists "Vận/Năm/Tháng" only. Confirm daily is deferred to v1.6 (recommended) or pulled in (adds 1–2 weeks).
4. **Should `find_van_khan_for_snapshot` rank/score the returned rituals**, or strictly return all matches and let callers rank? Recommended: return all, no ranking inside the library (avoids premature opinion).

---

*Sources: PROJECT.md, EXPANSION_FRAMEWORK.md §2.3 §2.4 §3 §5 §7, current source files at `crates/amlich-core/src/lib.rs`, `almanac/mod.rs`, `almanac/sat_phuong.rs`, `almanac/than_huong.rs`, `almanac/thai_tue.rs`, `almanac/golden_loader.rs`, `holidays.rs`, `holiday_data.rs`, `semantic_graph/provenance.rs`, `semantic_graph/ontology.rs`, `reasoning/personal.rs`. All file:line references verified against working-tree source on 2026-05-23.*
