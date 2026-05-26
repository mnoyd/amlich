# ADR-0001: Ritual JSON Schema v1

**Status:** Accepted
**Date:** 2026-05-26

## Context

Phase 12 will author ≥60 ritual entries (`source_id: vn-folk-ritual`) for the Vietnamese cultural
almanac. Per PITFALLS.md CRIT-1 and CRIT-5:

- **CRIT-1:** Source-ID cross-contamination between `vn-folk-ritual`, `vn-folk`, and `khcbppt` is
  a silent data-corruption risk. Schema-lock ensures source provenance fields are typed, not
  free-form strings.
- **CRIT-5:** Storing lễ vật (offerings) and trình tự (preparation steps) as freeform strings is
  brittle — consumers cannot enumerate items, validate order, or render structured UI components.
  Structured types must be locked before corpus authoring starts.

DEC-0015 registered `source_id: "vn-folk-ritual"` as the canonical tradition for Vietnamese ritual
prayer texts (Văn khấn cổ truyền). Re-editing 60+ corpus entries after a schema slip is
prohibitively expensive. This ADR locks the v1 schema before any corpus authoring begins.

The corpus loader (Phase 11) will use the `OnceLock + include_str!` pattern from
`golden_loader.rs:5-21`. Phase 12 cannot begin until Phase 11 ships a functioning loader against
this locked schema.

## Decision

### Field Set

**Required fields on `RitualEntry`:**

| Field                | Type                       | Notes                                          |
|----------------------|----------------------------|------------------------------------------------|
| `ritual_id`          | `String`                   | Stable kebab-case identifier                   |
| `title_vi`           | `String`                   | Vietnamese title                               |
| `event_keys`         | `Vec<RitualEventKey>`      | Discriminated union — see below                |
| `variant`            | `RitualVariantTag`         | Closed enum — see below                        |
| `offerings`          | `Vec<Offering>`            | Structured lễ vật                              |
| `preparation_steps`  | `Vec<PreparationStep>`     | Ordered trình tự                               |
| `invocation_text_vi` | `String`                   | Full Vietnamese văn khấn body                  |
| `source_id`          | `String`                   | Always `"vn-folk-ritual"`; loader validates    |
| `original_citation`  | `SourceCitation`           | Structured provenance reference                |
| `confidence`         | `RitualConfidenceTier`     | Closed enum                                    |

**Optional / reserved fields on `RitualEntry`:**

| Field       | Type            | Notes                                           |
|-------------|-----------------|-------------------------------------------------|
| `title_en`  | `Option<String>`| English title; optional, v1.5 corpus unpopulated |
| `body_en`   | `Option<String>`| Reserved per RIT-13; always null in v1.5 corpus  |
| `notes`     | `Vec<String>`   | Freeform editorial notes; omitted if empty       |

### Closed Enums

**`RitualVariantTag`** — discriminates entries sharing an event (RIT-12):
```
"simple"              → Simple
"full"                → Full
"buddhist"            → Buddhist
"folk"                → Folk
{"regional": "<area>"} → Regional(String)
```
Unknown variant tags fail deserialization. Variants are **separate `RitualEntry` records** sharing
`event_keys[]` — no nested `variants` substructure, no `event_group_id`.

**`RitualEventKey`** — typed discriminated union for `event_keys[]` (RIT-06):
```json
{"kind": "holiday_id",  "value": "tet-nguyen-dan"}
{"kind": "lunar_date",  "kind": "month_day", "month": 1, "day": 1}
{"kind": "tiet_khi",    "name": "Lập Xuân"}
{"kind": "life_event",  "event": "dong_tho"}
{"kind": "always"}
```

**`LunarDateMatch`** — three variants with `kind` tag:
- `MonthDay { month: u8, day: u8, leap_month_policy: LeapPolicy }` — default policy is
  `canonical_month_only` (field may be omitted in JSON)
- `SolarTerm { name: String }`
- `GregorianFixed { month: u8, day: u8 }`

**`LeapPolicy`**: `canonical_month_only` (default) | `leap_month_only` | `either`

**`LifeEventKind`**: `dong_tho` | `nhap_trach` | `khai_truong` | `cuoi` | `gio` | `day_thang`

**`RitualConfidenceTier`**: `primary` | `regional-variant` | `synthesized`

### Structured Sub-types

**`Offering`** — structured lễ vật:
```json
{"name_vi": "Xôi gấc", "name_en": null, "quantity": "1 đĩa", "notes": null}
```

**`PreparationStep`** — ordered trình tự:
```json
{"order": 1, "description_vi": "Bày lễ vật lên bàn thờ", "description_en": null}
```

**`SourceCitation`** — classical reference provenance:
```json
{"title": "Văn Khấn Cổ Truyền Việt Nam", "publisher": "NXB Văn Hóa Thông Tin", "edition": "2003", "page": "45"}
```

### Serde Discipline

- `RitualEntry` has `#[serde(deny_unknown_fields)]` — typos in field names fail immediately.
- All enums use `#[serde(rename_all = "snake_case")]` or explicit tag annotations.
- Optional fields use `#[serde(default, skip_serializing_if = "Option::is_none")]`.
- File-level: corpus files carry `$schema_version: "rituals-v1"` (loader validates).

### Sample JSON Entry

```json
{
  "ritual_id": "van-khan-tet-don-gian",
  "title_vi": "Văn Khấn Tết Nguyên Đán (Đơn Giản)",
  "title_en": null,
  "event_keys": [
    {"kind": "holiday_id", "value": "tet-nguyen-dan"},
    {"kind": "lunar_date", "month": 1, "day": 1}
  ],
  "variant": "simple",
  "offerings": [
    {"name_vi": "Hương", "quantity": "3 nén"},
    {"name_vi": "Hoa tươi", "quantity": "1 bình"},
    {"name_vi": "Quả", "quantity": "5 loại"},
    {"name_vi": "Nước sạch", "quantity": "3 chén"}
  ],
  "preparation_steps": [
    {"order": 1, "description_vi": "Tắm rửa sạch sẽ, mặc quần áo chỉnh tề"},
    {"order": 2, "description_vi": "Bày lễ vật lên bàn thờ gia tiên"},
    {"order": 3, "description_vi": "Thắp hương, đợi hương cháy một phần ba rồi vái lạy"}
  ],
  "invocation_text_vi": "Nam mô a di đà phật! Con lạy chín phương trời, mười phương chư phật...",
  "body_en": null,
  "source_id": "vn-folk-ritual",
  "original_citation": {
    "title": "Văn Khấn Cổ Truyền Việt Nam",
    "publisher": "NXB Văn Hóa Thông Tin",
    "edition": "2003",
    "page": "12"
  },
  "confidence": "primary"
}
```

## Consequences

- **Phase 11** corpus loader follows `OnceLock + include_str!` pattern (per `golden_loader.rs:5-21`).
  The loader deserializes each entry as `RitualEntry` and fails fast on unknown fields.
- **Phase 12** corpus authors target this locked schema. Any field addition or type change after
  Phase 12 corpus authoring has begun requires a superseding ADR (e.g., ADR-0001a) and a full
  corpus migration — expected cost: re-editing all entries affected.
- **`find_van_khan_for_snapshot` / `find_van_khan_for_event`** return all matching variants in one
  `Vec<&RitualEntry>` — caller (UI/CLI) ranks and filters by `variant` if desired. No
  `variant_filter` parameter. No `_canonical_` convenience method.
- **Unknown JSON fields fail** at deserialization — field-name typos are caught immediately, not
  silently discarded.
- **English content authoring is deferred** indefinitely; `body_en` is reserved schema real estate
  only. The `title_en` field is present in the schema but the v1.5 corpus leaves it unpopulated.
- **Variants are separate records** — no parent/child record split, no `event_group_id` — simplifies
  the loader and removes a class of join bugs.
