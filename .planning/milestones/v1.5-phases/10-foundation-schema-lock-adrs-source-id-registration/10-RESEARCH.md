# Phase 10: Foundation — Schema Lock + ADRs + Source-ID Registration - Research

**Researched:** 2026-05-26
**Domain:** Rust schema authoring, ADR documentation, source-id registry, additive DTO extension
**Confidence:** HIGH — every claim anchored to a file:line or explicit planning document

---

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

**Ritual variant model (RIT-12)**
- Variants are **separate `RitualEntry` records**, each with its own `ritual_id`. No nested `variants: Vec<…>` substructure; no parent/child record split.
- A **closed Rust enum** `RitualVariantTag { Simple, Full, Buddhist, Folk, Regional(String) }` discriminates variants. JSON `variant: "simple" | "full" | "buddhist" | "folk" | { "regional": "<area>" }` deserialized via serde tag-renaming. Unknown tags fail load (`#[serde(deny_unknown_fields)]` at corpus level).
- Variants link to their parent event **only via shared `event_keys[]`**. No `event_group_id`, no `ritual_id`-prefix naming convention as load-bearing grouping signal.
- `find_van_khan_for_snapshot()` / `find_van_khan_for_event()` **return all matching variants in one `Vec<&RitualEntry>`**. No `variant_filter` parameter. No `_canonical_` convenience method. Caller ranks and filters.

**ADR storage format & location**
- New canonical directory: **`.planning/adrs/`** (does not yet exist — create in Phase 10).
- Three ADRs land in Phase 10: `0001-ritual-schema-v1.md`, `0002-phi-tinh-monthly-anchor.md`, `0003-nien-tu-bach-polarity.md`.
- **Nygard short-form** template: `Title / Status / Context / Decision / Consequences`. Roughly one page each. No MADR-extended sections.
- Each ADR ships with **`Status: Accepted`**. Future revisions write new ADRs that mark prior as `Superseded by NNNN`.
- ADR numbering is its own sequence starting at `0001` (independent of existing `DEC-NNNN` ids in `.planning/MILESTONES.md`).
- **Cross-referenced** in `.planning/MILESTONES.md` Key Decisions table — one new row per ADR. Single index for project-level discoverability.

**source_id constants placement**
- New module: **`crates/amlich-core/src/sources.rs`** — single home for every `source_id` in the codebase.
- Plain **`pub const SOURCE_*: &str`** form. No `SourceId` enum, no helper APIs. Drop-in replacement for current string literals.
- Constants exposed: existing (`SOURCE_KHCBPPT`, `SOURCE_NGOC_HAP_KY`, `SOURCE_VN_FOLK`, `SOURCE_CUU_DIEU`, `SOURCE_TAM_MENH_THONG_HOI`) **plus** new (`SOURCE_VN_FOLK_RITUAL = "vn-folk-ritual"`, `SOURCE_HUYEN_KHONG = "huyen-khong"`). All seven canonical source_ids live here.
- **Full migration sweep in Phase 10:** every bare string literal replaced with corresponding constant.
- **CI grep test** in `crates/amlich-core/tests/` walks `crates/amlich-core/src/` (excluding `sources.rs`) and asserts no occurrence of any sanctioned source_id string literal outside the module.

### Claude's Discretion

- **Bilingual schema scope** — Research recommendation: ship VN-only at v1.5 with `body_en: Option<String>` reserved per RIT-13.
- **Phi Tinh star metadata field shape** — `polarity` vs `auspice` as one combined `nature: StarNature` enum or two separate fields.
- **Polarity matrix encoding inside ADR 0003** — markdown table inline in the ADR body, or referenced separate JSON.
- **`provenance_audit.md` ledger format** — deferred to Phase 12.
- **Sóc/Vọng generated holiday IDs** — `Holiday.id` remains `None` for auto-generated Mùng 1 / Rằm entries. Confirmed not a gap.
- **`FlyingStarLayout` struct multiplicity** — one parameterized struct or three distinct types. FND-02 requires the field set `(period, palaces[9], center_star, evidence)` is frozen.

### Deferred Ideas (OUT OF SCOPE)

- Full bilingual ritual corpus — deferred indefinitely.
- `provenance_audit.md` ledger format — Phase 12.
- Custom clippy lint for source_id literals — defer unless grep test proves insufficient.
- `SourceId` enum + ergonomics — rejected for v1.5.
- Daily / Hourly Phi Tinh — out of scope per REQUIREMENTS.md.
- Spatial Phi Tinh (Tier 3, Sơn-Hướng) — deferred to post-v1.5.
- Phi Tinh wired into `interaction/direction_merge.rs` — forbidden in v1.5.
</user_constraints>

---

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| FND-01 | Frozen `RitualEntry` JSON schema v1 (typed `event_keys[]`, structured `offerings[]`, structured `preparation_steps[]`, required `source_id`, `original_citation`, `confidence`) — locked before corpus authoring | Schema shape verified against ARCHITECTURE.md §1.2; `RitualVariantTag` closed enum locked in CONTEXT.md; `#[serde(deny_unknown_fields)]` pattern verified in `golden_loader.rs` |
| FND-02 | Frozen `FlyingStarLayout` API shape (`Period`, `[FlyingStar; 9]` palace array, `center_star`, `evidence`) — locked before algorithm work | Exact struct shape from ARCHITECTURE.md §2.3; Palace enum Lo Shu ordering (Center=5, N=1, NE=8, E=3, SE=4, S=9, SW=2, W=7, NW=6) confirmed in ARCHITECTURE.md and REQUIREMENTS.md FS-02 |
| FND-03 | `vn-folk-ritual` and `huyen-khong` registered as distinct source IDs with `pub const SOURCE_*` constants | Migration sweep scope: 11 production `source_id: "khcbppt"` assignments + 1 `"tam-menh-thong-hoi"` in baseline.json; `vn-folk`/`cuu-dieu`/`ngoc-hap-ky` currently have ZERO code-side literals — only mentioned in planning docs |
| FND-04 | Documented decision for monthly anchor convention (solar-term boundaries per *Thẩm Thị Huyền Không Học*, reusing v1.1.2 Tiết Khí scanner) as ADR | `get_all_tiet_khi_for_year(year, time_zone)` at `tietkhi.rs:227` is the exact scanner to reference; 12 solar-month Tiết Khí boundaries enumerated from `TIET_KHI` const array |
| FND-05 | Documented Niên Tử Bạch direction rule per Tam Nguyên × year polarity, with polarity matrix, as ADR | Tam Nguyên structure: Thượng (1-3), Trung (4-6), Hạ (7-9); year polarity = dương (odd) / âm (even) by can-chi index; matrix = 9 yuans × 2 polarities = 18 cells; reference: phongthuycaivan.org |
| FND-06 | `Holiday.id: Option<String>` (additive, `#[serde(default)]`) populated from `lunar_festivals[].id` — round-trip compatible with v1.4 JSON fixtures | `holidays.rs:14-25` exact struct location; `id` field absent from struct today; `lunar-festivals.json` already has `"id"` per-festival (e.g. `"tet-nguyen-dan"`); auto-generated Mùng 1/Rằm stay `None` |
</phase_requirements>

---

## Summary

Phase 10 is a **pure authoring phase** — it writes no algorithm code and populates no corpus content. It produces four concrete artifacts: (1) a new `crates/amlich-core/src/sources.rs` module with all seven canonical `pub const SOURCE_*` constants plus a migration sweep replacing the 11 bare `"khcbppt"` and 1 `"tam-menh-thong-hoi"` string literal assignments in production source; (2) three ADR files in a new `.planning/adrs/` directory; (3) an additive `pub id: Option<String>` field on the `Holiday` struct in `holidays.rs`; and (4) three new rows in the MILESTONES.md Key Decisions table cross-referencing the ADRs.

The research confirms that **`vn-folk`, `cuu-dieu`, and `ngoc-hap-ky` have no bare string literal call-sites in production Rust code** — they exist only in planning documentation and one JSON data file (`baseline.json`). The migration sweep is therefore smaller than the CONTEXT.md description might suggest: 11 `"khcbppt"` assignments in `crates/amlich-core/src/` (across 7 files) plus `"tam-menh-thong-hoi"` in `baseline.json`. The `sources.rs` module must still declare all seven constants so all new call-sites (Phases 11–15) can use them without re-introducing literals.

The three ADRs are decision-recording documents, not design decisions — all decisions are already locked in CONTEXT.md/REQUIREMENTS.md. The ADR work is focused authoring: transcribing locked decisions into Nygard-format markdown with the correct field names, matrix tables, and scanner references.

**Primary recommendation:** Execute in one wave with three parallel tasks: (A) `sources.rs` + migration sweep + CI grep test, (B) `Holiday.id` field addition, (C) three ADR files + MILESTONES.md rows. All three are independent and can land in a single commit.

---

## Standard Stack

### Core (no new dependencies)

| Library | Purpose | Why Standard |
|---------|---------|--------------|
| `serde` 1.0 (already in workspace) | `#[derive(Serialize, Deserialize)]` on `RitualEntry`, `RitualVariantTag`, `LunarDateMatch`, `Holiday` | All existing structs use this; `#[serde(default)]`, `#[serde(deny_unknown_fields)]` built-in |
| `std::sync::OnceLock` | Used by Phase 11 corpus loader (not Phase 10 directly, but ADR 0001 must specify it) | Stable since Rust 1.70; exact pattern in `golden_loader.rs:6` |
| `rustfmt` / `clippy` | Code formatting + lint (CI enforces) | Existing CI at `.github/workflows/ci.yml:40-43`; `clippy --workspace -- -D warnings` |

### No New Crate Dependencies

Phase 10 adds zero new `Cargo.toml` entries. The `sources.rs` module is plain `pub const` declarations. The `Holiday` struct change is a one-field addition. The ADRs are markdown files. The CI grep test uses standard Rust `std::fs::read_dir` + string matching.

---

## Architecture Patterns

### Pattern 1: Plain `pub const` source_id registry (sources.rs)

**What:** A new top-level `crates/amlich-core/src/sources.rs` module containing every canonical `source_id` as a `pub const &str`.

**Style reference (from CONVENTIONS.md):** `pub const SCREAMING_SNAKE_CASE: &str = "value";` — same pattern as `pub const VIETNAM_TIMEZONE: &str = "+07:00"` in `types.rs` and `pub const CAN: [&str; 10]` pattern in `canchi.rs`.

**Exact constants to declare:**
```rust
// crates/amlich-core/src/sources.rs
//! Canonical source_id constants for all classical traditions in amlich-core.
//!
//! Every `ProvenanceEntry::almanac_rule(source_id, method)` call-site in this
//! crate MUST use one of these constants. Bare string literals are forbidden
//! (enforced by `tests/source_id_guard.rs`).

/// Khâm Định Hiệp Kỷ Biện Phương Thư — primary Vietnamese almanac reference
pub const SOURCE_KHCBPPT: &str = "khcbppt";

/// Ngọc Hạp Ký — secondary classical reference for directional compatibility
pub const SOURCE_NGOC_HAP_KY: &str = "ngoc-hap-ky";

/// Vietnamese folk tradition (Hoàng Ốc and similar)
pub const SOURCE_VN_FOLK: &str = "vn-folk";

/// Cửu Diệu (九曜) — Buddhist/Indian astronomical tradition
pub const SOURCE_CUU_DIEU: &str = "cuu-dieu";

/// Tam Mệnh Thông Hội — Na Am / sexagenary sound source
pub const SOURCE_TAM_MENH_THONG_HOI: &str = "tam-menh-thong-hoi";

/// Văn khấn cổ truyền Việt Nam — ritual content corpus (new in v1.5)
pub const SOURCE_VN_FOLK_RITUAL: &str = "vn-folk-ritual";

/// Thẩm Thị Huyền Không Học — Phi Tinh / Flying Stars source (new in v1.5)
pub const SOURCE_HUYEN_KHONG: &str = "huyen-khong";
```

**Registration in lib.rs:** Add `pub mod sources;` alphabetically at `lib.rs:11` (between `pub mod semantic_graph;` and `pub mod sun;`). Re-export as `pub use crate::sources::*;` or leave as module access only — both acceptable; module access (`crate::sources::SOURCE_KHCBPPT`) is recommended to keep namespacing clear.

### Pattern 2: Migration sweep — replacing string literals with constants

**What:** Replace all 11 bare `"khcbppt".to_string()` and 1 `"tam-menh-thong-hoi"` (in `baseline.json`) with constants.

**Exact call-sites in `crates/amlich-core/src/` (verified 2026-05-26):**

| File | Line | Current | Replace With |
|------|------|---------|--------------|
| `src/almanac/thap_than.rs` | 15 | `source_id: "khcbppt".to_string()` | `source_id: crate::sources::SOURCE_KHCBPPT.to_string()` |
| `src/almanac/hour_pillar.rs` | 67 | `source_id: "khcbppt".to_string()` | `source_id: crate::sources::SOURCE_KHCBPPT.to_string()` |
| `src/interaction/personal_hour.rs` | 88 | `source_id: "khcbppt".to_string()` | `source_id: crate::sources::SOURCE_KHCBPPT.to_string()` |
| `src/almanac/data.rs` | 224 | `source_id: "khcbppt".to_string()` | `source_id: crate::sources::SOURCE_KHCBPPT.to_string()` |
| `src/almanac/data.rs` | 229 | `source_id: "khcbppt".to_string()` | `source_id: crate::sources::SOURCE_KHCBPPT.to_string()` |
| `src/almanac/dai_van.rs` | 68 | `source_id: "khcbppt".to_string()` | `source_id: crate::sources::SOURCE_KHCBPPT.to_string()` |
| `src/almanac/types.rs` | 358 | `source_id: "khcbppt".to_string()` | `source_id: crate::sources::SOURCE_KHCBPPT.to_string()` |
| `src/interaction/domain_day_boost.rs` | 57 | `source_id: "khcbppt".to_string()` | `source_id: crate::sources::SOURCE_KHCBPPT.to_string()` |
| `src/interaction/day_person.rs` | 32 | `source_id: "khcbppt".to_string()` | `source_id: crate::sources::SOURCE_KHCBPPT.to_string()` |
| `src/interaction/direction_merge.rs` | 82 | `source_id: "khcbppt".to_string()` | `source_id: crate::sources::SOURCE_KHCBPPT.to_string()` |
| `src/interaction/element_resonance.rs` | 51 | `source_id: "khcbppt".to_string()` | `source_id: crate::sources::SOURCE_KHCBPPT.to_string()` |

**JSON data file requiring update:**
- `data/almanac/baseline.json:12` — `"source_id": "tam-menh-thong-hoi"` remains a string literal (it's JSON data, not Rust code) but the CI grep test must explicitly allow JSON files.

**Not in scope for migration sweep (confirmed absent):**
- `"vn-folk"`, `"ngoc-hap-ky"`, `"cuu-dieu"` have **zero bare string literal assignments** in `crates/amlich-core/src/` production code — they exist only in planning docs and comments. The constants are declared in `sources.rs` for future use by Phases 11–14.
- `"tam-menh-thong-hoi"` has zero Rust source code assignments; it appears only in `baseline.json` (JSON data, not Rust source) and in test assertions. The constant is declared for future use.

**Important note on test files:** Lines like `assert_eq!(evidence.source_id, "khcbppt")` in `tests/` are comparison assertions, not assignments. The CI grep test must exclude test files from the forbidden-literal assertion (CONTEXT.md §specifics confirms test fixtures are allow-listed).

### Pattern 3: CI grep test (source_id_guard.rs)

**What:** A `crates/amlich-core/tests/source_id_guard.rs` integration test that walks `crates/amlich-core/src/` and asserts no file (except `sources.rs`) contains any of the 7 canonical source_id string literals as a bare string.

**Pattern example (follows `recommendation_taxonomy_audit.rs` style):**
```rust
// crates/amlich-core/tests/source_id_guard.rs
//! CI guard: ensures all source_id constants are declared in sources.rs only.
//! No bare source_id string literals allowed in src/ (except sources.rs itself).

const FORBIDDEN_LITERALS: &[&str] = &[
    "\"khcbppt\"",
    "\"vn-folk\"",
    "\"ngoc-hap-ky\"",
    "\"cuu-dieu\"",
    "\"tam-menh-thong-hoi\"",
    "\"vn-folk-ritual\"",
    "\"huyen-khong\"",
];

// Walk crates/amlich-core/src/, skip sources.rs, assert no forbidden literal found.
// Allow-list: test fixtures in tests/ and doc-comment examples (// or ///) allowed.
```

**Exclusion logic (per CONTEXT.md §specifics):**
1. `sources.rs` itself — always excluded (it IS the definitions)
2. `tests/` directories — JSON snapshot fixtures legitimately contain source_id strings as data
3. Lines starting with `//` or `///` — doc-comment examples allowed
4. The `baseline.json` data file — JSON data, not Rust source, not covered by this test

### Pattern 4: Holiday.id additive field

**What:** Add `pub id: Option<String>` to the `Holiday` struct in `holidays.rs:14-25`, populate from `lunar_festivals[].id` at creation sites, leave `None` for auto-generated Mùng 1/Rằm and solar holidays.

**Exact struct location (verified):**
```rust
// crates/amlich-core/src/holidays.rs:14-25 (CURRENT)
pub struct Holiday {
    pub name: String,
    pub description: String,
    pub lunar_date: Option<LunarDate>,
    pub solar_day: i32,
    pub solar_month: i32,
    pub solar_year: i32,
    pub is_solar: bool,
    pub category: String,
    pub is_major: bool,
}
```

**Required addition:**
```rust
pub struct Holiday {
    pub id: Option<String>,               // NEW — from lunar_festivals[].id
    // ... all existing fields unchanged
    #[serde(default, skip_serializing_if = "Option::is_none")]
    // Note: Holiday currently derives Debug, Clone — NOT Serialize/Deserialize.
    // Verify whether serde annotation is needed or if Holiday is internal-only.
}
```

**IMPORTANT finding:** `Holiday` at `holidays.rs:14-25` currently derives only `Debug` and `Clone` — NOT `Serialize`/`Deserialize`. This means `#[serde(default)]` is only needed if serde is added to Holiday in this phase. The `id` field should still be `Option<String>` but the serde annotation may be a no-op if Holiday is not yet serialized. Verify before adding the annotation.

**Creation sites to update (verified from code):**
- `holidays.rs:148-162` — lunar festivals loop using `create_lunar_holiday()` — add `id: Some(festival.id.clone())` if the `LunarFestivalEntry` struct exposes `id`. Need to check `holiday_data.rs` struct for `id` field.
- `holidays.rs:164-181` — Thanh Minh (solar-based) — `id: Some("thanh-minh".to_string())` or `id: None`
- `holidays.rs:183-199` — solar holidays loop — `id` from `holiday_data.id` if available, else `None`
- `holidays.rs:227-260` — Mùng 1/Rằm auto-generation — `id: None` (confirmed by CONTEXT.md)

**Data source verified:** `crates/amlich-core/data/holidays/lunar-festivals.json` has `"id": "tet-nguyen-dan"` as the first field of each festival object. The `holiday_data.rs` module must expose this `id` field through its `LunarFestivalEntry` struct.

**Round-trip compatibility:** `Holiday` is currently internal (not serialized). The v1.4 JSON fixture at `crates/amlich-api/tests/fixtures/day-info-golden.json` tests the API layer (`DayInfoDto`), not `Holiday` directly. FND-06 round-trip test validates that deserializing a v1.4 `DaySnapshot` JSON into v1.5 structs re-serializes without unexpected fields — this is about `DaySnapshot`, not `Holiday`. The `Holiday.id` change is additive and backward-compatible because `Holiday` is not in the JSON serialization surface today.

### Pattern 5: ADR authoring (Nygard short-form)

**What:** Three markdown files in `.planning/adrs/` following the Nygard short-form template.

**Template structure (exactly as locked in CONTEXT.md):**
```markdown
# ADR-000X: [Title]

**Status:** Accepted
**Date:** 2026-05-26

## Context

[What situation necessitated this decision? What forces are at play?]

## Decision

[The decision made, stated clearly in present tense.]

## Consequences

[What becomes easier, harder, or must be watched after this decision.]
```

**ADR 0001 — Ritual JSON Schema v1**

Must specify all schema-locking decisions locked in CONTEXT.md. Body of ADR must include:

1. **`event_keys[]` discriminated-union shape** (from ARCHITECTURE.md §1.2 and CONTEXT.md):
   - `{ "kind": "holiday_id", "value": "<id>" }` — matches `lunar-festivals.json` `id` field
   - `{ "kind": "lunar_date", "month": N, "day": N, "leap_ok": bool }` — `LunarDateMatch::MonthDay`
   - `{ "kind": "tiet_khi", "name": "<term-name>" }` — `LunarDateMatch::SolarTerm`
   - `{ "kind": "life_event", "event": "<kind>" }` — `LifeEventKind` enum
   - `{ "kind": "always" }` — matches every day

2. **`LunarDateMatch` variants** (from REQUIREMENTS.md RIT-07):
   - `MonthDay { month: u8, day: u8, leap_month_policy: LeapPolicy }`
   - `SolarTerm` — keyed by `tietkhi.name`
   - `GregorianFixed { month: u8, day: u8 }`
   - Default `LeapPolicy::CanonicalMonthOnly`

3. **`RitualVariantTag` closed enum** (from CONTEXT.md): `Simple | Full | Buddhist | Folk | Regional(String)` — serialized as `"simple" | "full" | "buddhist" | "folk" | { "regional": "<area>" }`; `#[serde(deny_unknown_fields)]`; `Regional(String)` NFC-normalized at load

4. **Confidence tiers** (from ARCHITECTURE.md §1.2): `"primary" | "regional-variant" | "synthesized"`

5. **`source_citation` structure** (from ARCHITECTURE.md §1.2):
   ```json
   { "title": "...", "publisher": "...", "edition": "...", "page": "..." }
   ```

6. **Required fields:** `ritual_id`, `title_vi`, `event_keys[]`, `offerings[]`, `preparation_steps[]`, `invocation_text_vi`, `source_id` (always `"vn-folk-ritual"`), `original_citation`, `confidence`, `variant`

7. **Optional/reserved fields:** `title_en` (optional), `body_en: Option<String>` (reserved, always null in v1.5 per RIT-13), `notes[]` (optional)

8. **Schema versioning:** `$schema_version: "rituals-v1"` at file level; `#[serde(deny_unknown_fields)]` at entry level

**ADR 0002 — Monthly Phi Tinh Anchor Convention**

Must specify:
1. **Decision:** Monthly Phi Tinh star computation uses **solar-term month boundaries** (節氣, tháng tiết khí), not lunar calendar months and not civil Gregorian months. Per *Thẩm Thị Huyền Không Học*.
2. **Boundary resolver:** The v1.1.2 Tiết Khí scanner `get_all_tiet_khi_for_year(year: i32, time_zone: f64) -> Vec<SolarTermWithDate>` in `crates/amlich-core/src/tietkhi.rs:227`. No new term-scanning code is written.
3. **12 monthly periods and their opening Tiết Khí** (from the 24-term `TIET_KHI` constant array at `tietkhi.rs:37-158` — every other term is a month-opener):

| Month | Tiết Khí | Longitude |
|-------|----------|-----------|
| Tháng Dần (month 1 solar) | Lập Xuân | 315° |
| Tháng Mão (month 2) | Kinh Trập | 345° |
| Tháng Thìn (month 3) | Thanh Minh | 15° |
| Tháng Tỵ (month 4) | Lập Hạ | 45° |
| Tháng Ngọ (month 5) | Mang Chủng | 75° |
| Tháng Mùi (month 6) | Tiểu Thử | 105° |
| Tháng Thân (month 7) | Lập Thu | 135° |
| Tháng Dậu (month 8) | Bạch Lộ | 165° |
| Tháng Tuất (month 9) | Hàn Lộ | 195° |
| Tháng Hợi (month 10) | Lập Đông | 225° |
| Tháng Tý (month 11) | Đại Tuyết | 255° |
| Tháng Sửu (month 12) | Tiểu Hàn | 285° |

4. **Consequences:** Year-branch group rule for monthly star (groups start at 8/5/2, descend mod-9) applies to these solar months, not to lunar months.
5. **Why not lunar months:** They shift Gregorian dates year-to-year; ADR 0002 documents that *Thẩm Thị Huyền Không Học* uses solar-month anchoring per classical convention (PITFALLS MOD-2, REQUIREMENTS FND-04).

**ADR 0003 — Niên Tử Bạch Direction Polarity Matrix**

Must specify:
1. **Decision:** Annual flying star direction (Niên Tử Bạch thuận/nghịch hành) is determined by a matrix keyed on (Tam Nguyên yuan period) × (year polarity: dương/âm). Not a single sign flag.
2. **Tam Nguyên structure** (from PITFALLS MOD-3 + phongthuycaivan.org):
   - **Thượng Nguyên** (Upper Yuan): Vận 1 (1864–1883), Vận 2 (1884–1903), Vận 3 (1904–1923)
   - **Trung Nguyên** (Middle Yuan): Vận 4 (1924–1943), Vận 5 (1944–1963), Vận 6 (1964–1983)
   - **Hạ Nguyên** (Lower Yuan): Vận 7 (1984–2003), Vận 8 (2004–2023), Vận 9 (2024–2043)
   - Boundaries align with Lập Xuân, not Jan 1 (per ADR 0002)
3. **Year polarity:** A year is dương (odd can-chi index in the 10-stem sequence: Giáp, Bính, Mậu, Canh, Nhâm) or âm (Ất, Đinh, Kỷ, Tân, Quý). 2024 = Giáp Thìn = dương; 2025 = Ất Tỵ = âm.
4. **Polarity matrix** (from phongthuycaivan.org/cach-tra-phi-tinh-nien-nguyet-nhat-thoi/ — MEDIUM confidence, locked as decision for Phase 13 implementation):

| Yuan | Starting Star | Dương Year Direction | Âm Year Direction |
|------|--------------|---------------------|-------------------|
| Thượng Nguyên | 1-White | Nghịch hành (retrograde) | Thuận hành (forward) |
| Trung Nguyên | 4-Green | Nghịch hành | Thuận hành |
| Hạ Nguyên | 7-Red | Nghịch hành | Thuận hành |

Note: The pattern is consistent — dương years always go nghịch hành, âm years always go thuận hành. The Yuan determines the starting star number, not the direction. Confidence: MEDIUM (single Vietnamese source; to be cross-checked against *Thẩm Thị Huyền Không Học* during Phase 13 implementation and logged as `KnownDivergence` if source disagrees). The ADR acknowledges this and names Phase 13 as the phase where golden validation occurs.

5. **Consequences:** Phase 13 must implement a `(van_number, year_polarity) -> (starting_star, direction)` lookup, not a single `is_retrograde: bool`. ADR does NOT claim the matrix is final — it claims the matrix shape is final and this is the Phase 10 locked convention.

### Pattern 6: MILESTONES.md Key Decisions table addition

**Current table format** (from `MILESTONES.md:212-219`): Narrative numbered list (not a markdown table).

**Column shape from CONTEXT.md §specifics:**
```
| DEC-0023 | 2026-05-26 | Ritual JSON schema v1 locked | [adrs/0001-ritual-schema-v1.md](adrs/0001-ritual-schema-v1.md) |
```

**Finding:** The MILESTONES.md "Key Decisions" section (line 212) uses a **narrative numbered list**, not a markdown table. The CONTEXT.md example shows a table format. Phase 10 should introduce the table format OR append to the numbered list. Since CONTEXT.md specifies a table with DEC-NNNN IDs, the planner should add a new **Project Decisions Registry** section as a markdown table, or convert the existing Key Decisions list.

**Next free DEC-NNNN:** PROJECT.md Key Decisions table (line 43) uses descriptive text without DEC-NNNN numbering. STATE.md references DEC-0015, DEC-0016, DEC-0022. REQUIREMENTS.md/ROADMAP.md reference DEC-0015/0016/0022. ARCHITECTURE.md references DEC-0015. No DEC-0017 through DEC-0022 are explicitly documented in PROJECT.md Key Decisions — they appear to be referenced in planning docs but not given formal table rows. **The next safe DEC-NNNN is DEC-0023** (three new rows: DEC-0023, DEC-0024, DEC-0025 for the three ADRs).

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Source-id registry | Custom enum + conversion logic | Plain `pub const SOURCE_*: &str` | CONTEXT.md locked decision; matches `SCREAMING_SNAKE_CASE` convention in `CONVENTIONS.md` |
| ADR template | Custom format | Nygard short-form (Title/Status/Context/Decision/Consequences) | CONTEXT.md locked decision |
| CI source-id enforcement | Complex AST-based lint | `std::fs::read_dir` + string search in integration test | CONTEXT.md specifies grep test; dylin/custom lint deferred explicitly |
| Holiday `id` propagation | New data structure | Simple `Option<String>` field on existing struct | `#[serde(default)]` pattern established in v1.2 |
| Schema version enforcement | Runtime version checking logic | `$schema_version` string field + `#[serde(deny_unknown_fields)]` | PITFALLS MIN-1 and CRIT-5 prevention; golden_loader pattern |

---

## Common Pitfalls

### Pitfall 1: Migration sweep misses test assertions vs. production assignments

**What goes wrong:** The grep test scans all `.rs` files and finds `assert_eq!(evidence.source_id, "khcbppt")` in test files. Either the test false-positives, or the developer replaces assertion strings with constants (breaking test semantics).

**How to avoid:** The CI grep test excludes `tests/` directories and lines starting with `//` or `///`. Only production `source_id: "khcbppt".to_string()` assignments in `src/` need replacing.

**Warning signs:** If `cargo test` passes before migration but the grep test shows 0 matches in `src/`, the test is scanning the wrong paths.

### Pitfall 2: Holiday struct has no serde derive — #[serde(default)] is a no-op

**What goes wrong:** Adding `#[serde(default, skip_serializing_if = "Option::is_none")]` to `Holiday.id` but `Holiday` only derives `Debug, Clone` (verified at `holidays.rs:14-25`). The annotation is syntactically valid but does nothing until `Serialize`/`Deserialize` is added.

**How to avoid:** FND-06 only requires the field to exist and be populated. The round-trip test is about `DaySnapshot`, not `Holiday` itself. Add the field without serde annotations unless Phase 11 explicitly needs `Holiday` to serialize.

**Warning signs:** If the round-trip test for FND-06 is "load a v1.4 DaySnapshot JSON" — Holiday is not in DaySnapshot JSON today. The real round-trip test is that existing JSON consumers don't break when `Holiday` gets the new field. Verify what types reference `Holiday` in serialized form.

### Pitfall 3: ADR 0003 polarity matrix is MEDIUM confidence — must say so

**What goes wrong:** ADR 0003 presents the Tam Nguyên polarity matrix as settled fact. Phase 13 implements from it and discovers the matrix is wrong for Trung Nguyên years (a case nobody cross-checked because the practical era is Hạ Nguyên).

**How to avoid:** ADR 0003 body must explicitly state confidence level and name the validation plan: "This matrix is to be validated against *Thẩm Thị Huyền Không Học* during Phase 13 implementation. Disagreements will be logged as `KnownDivergence` per EXPANSION_FRAMEWORK §7. The ADR is Accepted at v1.5 to unblock Phase 13 development; a superseding ADR-0003a will be written if the matrix is revised."

**Warning signs:** ADR 0003 claims HIGH confidence without a named classical text cross-check for the Thượng/Trung Nguyên rows.

### Pitfall 4: DEC-NNNN collision in MILESTONES.md

**What goes wrong:** Phase 10 adds DEC-0023/24/25 rows but an intermediate planning document already used DEC-0023 for something else.

**How to avoid:** Research confirmed DEC-0015, 0016, 0022 are explicitly referenced across planning docs. DEC-0017–0021 are not referenced anywhere. DEC-0023 is the next safe number. The CONTEXT.md §specifics already uses `DEC-0023` as the example row, confirming this expectation.

### Pitfall 5: `lunar-festivals.json` `id` field may not be exposed by `LunarFestivalEntry` in `holiday_data.rs`

**What goes wrong:** `lunar-festivals.json` has `"id"` at the JSON level, but the Rust struct `LunarFestivalEntry` in `holiday_data.rs` may not include it (the JSON is parsed selectively). The `create_lunar_holiday()` function then has no `festival.id` to propagate.

**How to avoid:** Verify `holiday_data.rs` struct before writing the Holiday.id population code. If `id` is absent from the Rust struct, add it there first, then propagate to `Holiday`.

---

## Code Examples

### sources.rs module registration (lib.rs)
```rust
// crates/amlich-core/src/lib.rs — add alphabetically between semantic_graph and sun
pub mod sources;
```

### Holiday.id field addition
```rust
// crates/amlich-core/src/holidays.rs:14-25 — modified struct
#[derive(Debug, Clone)]
pub struct Holiday {
    pub id: Option<String>,       // NEW: stable id from lunar-festivals.json; None for auto-generated entries
    pub name: String,
    pub description: String,
    pub lunar_date: Option<LunarDate>,
    pub solar_day: i32,
    pub solar_month: i32,
    pub solar_year: i32,
    pub is_solar: bool,
    pub category: String,
    pub is_major: bool,
}
```

### Creation site update pattern
```rust
// In the lunar festivals loop (holidays.rs:148-162)
Holiday {
    id: Some(festival.id.clone()),  // from LunarFestivalEntry.id
    name: name.clone(),
    // ... all other fields unchanged
}

// For auto-generated Mùng 1/Rằm (holidays.rs:227-260)
Holiday {
    id: None,  // no stable id for generated entries
    // ... all other fields unchanged
}
```

### CI grep test skeleton
```rust
// crates/amlich-core/tests/source_id_guard.rs
use std::fs;
use std::path::Path;

const FORBIDDEN_LITERALS: &[&str] = &[
    r#""khcbppt""#,
    r#""vn-folk""#,
    r#""ngoc-hap-ky""#,
    r#""cuu-dieu""#,
    r#""tam-menh-thong-hoi""#,
    r#""vn-folk-ritual""#,
    r#""huyen-khong""#,
];

#[test]
fn no_bare_source_id_literals_in_src() {
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    // Walk src/, skip sources.rs, fail on any FORBIDDEN_LITERAL found on a non-comment line
    // ...
}
```

---

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust built-in `#[test]` (no separate framework) |
| Config file | None — CI runs `cargo test --workspace --exclude am-lich` |
| Quick run command | `cargo test --package amlich-core` |
| Full suite command | `cargo test --workspace --exclude am-lich` |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| FND-01 | Deserializing a sample `RitualEntry` JSON with v1 schema succeeds; extra field fails | unit (deserialization roundtrip) | `cargo test --package amlich-core ritual_schema` | ❌ Wave 0 — new file needed |
| FND-02 | `FlyingStarLayout { period, palaces: [FlyingStar; 9], center_star, evidence }` can be constructed | unit (struct construction) | `cargo test --package amlich-core flying_star_layout` | ❌ Wave 0 — new file needed |
| FND-03 | `SOURCE_VN_FOLK_RITUAL` and `SOURCE_HUYEN_KHONG` constants exist at correct string values; no bare literals in src/ | unit + integration grep | `cargo test --package amlich-core sources` + `cargo test --package amlich-core source_id_guard` | ❌ Wave 0 — two new files needed |
| FND-04 | ADR 0002 file exists with correct content; no test for semantic correctness (editorial, not algorithmic) | smoke (file existence) | `cargo test --package amlich-core` (test asserts `.planning/adrs/0002-*.md` exists) OR manual check | ❌ Wave 0 — optional existence test |
| FND-05 | ADR 0003 file exists with polarity matrix | smoke (file existence) | manual check | N/A — documentation artifact |
| FND-06 | `Holiday` has `id: Option<String>` field; `get_vietnamese_holidays(2024)` returns at least one entry with `id == Some("tet-nguyen-dan")` | unit | `cargo test --package amlich-core holiday_id_field` | ❌ Wave 0 — new test in existing `holidays.rs` #[cfg(test)] block |

### Sampling Rate
- **Per task commit:** `cargo test --package amlich-core` (< 30 seconds)
- **Per wave merge:** `cargo test --workspace --exclude am-lich`
- **Phase gate:** Full suite green + `cargo clippy --workspace -- -D warnings` before `/gsd:verify-work`

### Wave 0 Gaps
- [ ] `crates/amlich-core/src/sources.rs` — new file with 7 constants
- [ ] `crates/amlich-core/tests/source_id_guard.rs` — CI grep test for FND-03
- [ ] Add `id: Option<String>` to `Holiday` struct and populate in creation sites (FND-06)
- [ ] `RitualEntry` Rust struct definition (types needed for FND-01 schema lock test) — minimal stub sufficient in Phase 10; Phase 11 fills out the full module
- [ ] `FlyingStarLayout` Rust struct definition (minimal stub for FND-02 freeze test) — Phase 13 fills implementation
- [ ] `.planning/adrs/` directory + three ADR files

---

## Open Questions

1. **Does `holiday_data.rs` expose `LunarFestivalEntry.id`?**
   - What we know: `lunar-festivals.json` has `"id": "tet-nguyen-dan"` per entry (verified). The `holiday_data.rs` module parses this JSON.
   - What's unclear: Whether the `LunarFestivalEntry` Rust struct includes the `id` field. If not, it must be added to the struct before `Holiday.id` can be populated.
   - Recommendation: Planner adds a task to verify `holiday_data.rs` struct and add `id: String` to `LunarFestivalEntry` if absent.

2. **Does Holiday struct need serde derive for FND-06?**
   - What we know: `Holiday` currently derives `Debug, Clone` only. The v1.4 round-trip test (INT-05) tests `DaySnapshot` JSON round-trip, which does not directly contain `Holiday`.
   - What's unclear: Whether FND-06's "round-trip compatible with v1.4 JSON fixtures" means `Holiday` itself must serialize, or just that existing consumers of `get_vietnamese_holidays()` are not broken.
   - Recommendation: `Holiday.id` field addition is sufficient for FND-06. Serde derive on `Holiday` is NOT required in Phase 10 — that would be a breaking scope expansion.

3. **MILESTONES.md Key Decisions format mismatch**
   - What we know: The existing section at line 212 uses a numbered list format, not a markdown table. CONTEXT.md §specifics shows a table format with columns `| DEC-NNNN | date | description | link |`.
   - What's unclear: Whether Phase 10 should convert the existing section to a table or add a new subsection.
   - Recommendation: Add a new **### ADR Cross-References** subsection as a markdown table immediately after the existing Key Decisions list. Do not reformat the existing v1.0–v1.4 decisions.

4. **ADR 0003 polarity matrix row for Thượng / Trung Nguyên**
   - What we know: Current era is Hạ Nguyên (Vận 7-9). The dương=nghịch / âm=thuận pattern is MEDIUM confidence (single source: phongthuycaivan.org).
   - What's unclear: Whether Thượng/Trung Nguyên follow the same pattern. The source implies they do (same rule, different starting star).
   - Recommendation: ADR 0003 documents the matrix as stated, notes MEDIUM confidence for Thượng/Trung rows, and names Phase 13 as the validation phase. This is not a blocker for Phase 10.

---

## Sources

### Primary (HIGH confidence)
- `crates/amlich-core/src/holidays.rs:14-25` — exact `Holiday` struct shape, no `id` field today
- `crates/amlich-core/src/tietkhi.rs:37-158` — `TIET_KHI` const array with all 24 terms and longitudes
- `crates/amlich-core/src/tietkhi.rs:227` — `get_all_tiet_khi_for_year(year: i32, time_zone: f64)` function signature
- `crates/amlich-core/src/semantic_graph/provenance.rs:65-67` — `ProvenanceEntry::almanac_rule(source_id, method)` exact signature
- `crates/amlich-core/src/almanac/golden_loader.rs:5-21` — OnceLock + include_str! + validate pattern
- `crates/amlich-core/src/lib.rs:10-26` — module list for alphabetical insertion of `pub mod sources;`
- `crates/amlich-core/data/holidays/lunar-festivals.json` — `"id"` field present on each festival object
- `.github/workflows/ci.yml` — existing CI steps (fmt, clippy, test)
- `.planning/codebase/CONVENTIONS.md` — `SCREAMING_SNAKE_CASE` constant naming convention
- `.planning/phases/10-foundation-schema-lock-adrs-source-id-registration/10-CONTEXT.md` — all locked decisions
- `.planning/REQUIREMENTS.md` — FND-01..06 exact requirement text
- `.planning/research/ARCHITECTURE.md` — schema shapes, file:line integration points
- `.planning/research/PITFALLS.md` — CRIT-1, CRIT-5, MOD-1, MOD-2, MOD-3, MOD-6, MIN-1 guidance

### Secondary (MEDIUM confidence)
- `phongthuycaivan.org/cach-tra-phi-tinh-nien-nguyet-nhat-thoi/` — Tam Nguyên yuan-conditional rule for Niên Tử Bạch; single Vietnamese-language source; awaiting cross-check against *Thẩm Thị Huyền Không Học* in Phase 13
- `.planning/PROJECT.md:43-56` — existing Key Decisions table format; existing DEC-NNNN numbering gap analysis

### Tertiary (LOW confidence)
- None for this phase — all claims are anchored to file:line or explicit planning documents.

---

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — no new dependencies; all patterns verified against existing code
- Architecture: HIGH — all file:line references verified 2026-05-26
- Migration sweep scope: HIGH — grep exhaustively confirmed 11 `"khcbppt"` production assignments and 1 `"tam-menh-thong-hoi"` in baseline.json; zero bare `"vn-folk"`, `"cuu-dieu"`, `"ngoc-hap-ky"` in Rust source
- ADR content: HIGH for 0001 and 0002 (all decisions locked); MEDIUM for 0003 polarity matrix (single source for Tam Nguyên rows)
- Pitfalls: HIGH — anchored in actual code inspection

**Research date:** 2026-05-26
**Valid until:** Stable — this is a schema-lock phase with no fast-moving dependencies
