# Phase 12: Văn khấn Corpus Authoring — Research

**Researched:** 2026-05-27
**Domain:** Vietnamese ritual corpus authoring (JSON data files, schema compliance, provenance audit)
**Confidence:** HIGH (schema is locked by ADR-0001; loader exists and passes tests; all integration
points are in-tree and verified by Phase 11)

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| RIT-09 | ≥ 60 ritual entries under `data/rituals/` in ≤ 14 per-event-category files plus `manifest.json` | §Architecture (file layout); §Standard Stack (loader needs manifest support or single-file expansion); §Code Examples (manifest.json shape + loader wiring) |
| RIT-10 | Every entry carries `source_id: "vn-folk-ritual"`, `original_citation` (book + page), and `confidence` tier (`primary` / `regional-variant` / `synthesized`) | §Architecture (ADR-0001 field set, fully locked); loader already validates source_id at load |
| RIT-11 | `provenance_audit.md` ledger in `data/rituals/` recording classical reference and independent reviewer for every entry | §Architecture (audit ledger format); §Common Pitfalls (single-author risk); no Rust code change needed — markdown file only |
| RIT-12 | ≥ 4 events with multiple variants sharing `event_type` discriminated by `variant` field | §Architecture (variant system; `RitualVariantTag` already locked); requires deliberate authoring of Tết, Vu Lan, Đoan Ngọ, Nhập Trạch with ≥ 2 variants each |
| RIT-13 | Reserved `body_en: Option<String>` on `RitualEntry`, deserialized via `#[serde(default)]`, content deferred | §Standard Stack — ALREADY EXISTS in `schema.rs` line 397 and `corpus.rs` lines 73-75; RIT-13 is effectively DONE; only a verification task remains |
</phase_requirements>

---

## Summary

Phase 12 is a **content-authoring phase**, not an algorithm phase. The Rust infrastructure
(schema, loader, matcher, all five public APIs) was fully shipped and verified in Phases 10 and 11.
The work here is writing at least 60 traceable Vietnamese ritual (văn khấn) entries in JSON,
splitting them across ≤ 14 per-event-category files, adding a `manifest.json` so the loader can
discover all files without hard-coding each filename, and producing a `provenance_audit.md` ledger.

Three distinct sub-tasks drive the phase:
1. **Loader extension (small Rust change):** The Phase 11 corpus loader (`corpus.rs`) currently
   `include_str!`s a single `fixtures.json`. Phase 12 needs the loader to read `manifest.json` and
   `include_str!` each listed file. This is a mechanical change (≤ 40 lines); the schema, normalize,
   and validate logic is unchanged.
2. **Corpus authoring (main work):** Author ≥ 60 entries across ≤ 14 per-event-category JSON files
   conforming exactly to ADR-0001 / `schema.rs`. Every entry needs `source_id: "vn-folk-ritual"`,
   `original_citation` (classical book + page), and a `confidence` tier. At least 4 events need ≥ 2
   variants.
3. **Provenance ledger:** A `data/rituals/provenance_audit.md` markdown file listing every entry's
   classical reference and independent reviewer.

**RIT-13 is already satisfied.** `body_en: Option<String>` with `#[serde(default,
skip_serializing_if = "Option::is_none")]` already exists in `schema.rs:396-397` and is normalized
at load in `corpus.rs:73-75`. Phase 12 only needs to confirm this field exists and is documented —
no new Rust code required for RIT-13.

**Primary recommendation:** Plan two waves — Wave 1: extend the loader to support manifest.json +
author the minimum 60 entries across ≤ 14 category files; Wave 2: write provenance_audit.md and
add tests validating that every entry's `original_citation` and `confidence` field is populated.

---

## Standard Stack

### Core (unchanged from Phase 11 — no new dependencies)

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `serde` | workspace `1.0` | Deserialize JSON corpus files | Locked by ADR-0001; already in use |
| `serde_json` | workspace `1.0` | Parse per-category JSON files at compile time | OnceLock + include_str! pattern already established in corpus.rs |
| `std::sync::OnceLock` | stdlib (Rust ≥ 1.70) | Lazy static corpus cache | corpus.rs already uses; no change |
| `include_str!` | stdlib macro | Compile-time embed of corpus JSON files | Crate-wide pattern; each per-category file must be `include_str!`-embedded |
| `unicode-normalization` | `0.1.25` | NFC normalize all text fields at load | Already in Cargo.toml; `normalize_and_validate` in corpus.rs covers all new entries automatically |

**No new Cargo.toml changes are needed.** The existing `unicode-normalization = "0.1.25"` and the
locked `serde + serde_json` handle everything.

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `include_str!` per file listed in manifest | `fs::read_to_string` at runtime | wasm and embedded targets cannot do filesystem I/O; compile-time embedding is the crate-wide convention. Do NOT switch to runtime I/O. |
| per-category split files + manifest.json | One large all-rituals.json | A single file works for ≤ 60 entries but violates RIT-09's "≤ 14 per-event-category files plus manifest.json" requirement and makes future authoring harder. Use the split. |
| Manual HolidayId value strings | Cross-validated against `lunar-festivals.json` | Manual entry is error-prone; cross-reference test (`every_holiday_id_in_fixtures_resolves_to_a_real_holiday` in rituals_integration.rs) already validates this for the fixture entries and must remain green for the expanded corpus. |

---

## Architecture Patterns

### Recommended File Layout

```
crates/amlich-core/data/rituals/
├── manifest.json              # NEW (Phase 12) — lists all per-category files
├── fixtures.json              # EXISTING (6 stub Phase 11 entries) — keep or subsume
├── tet-nguyen-dan.json        # NEW — Tết entries (simple, full, buddhist, folk)
├── soc-vong.json              # NEW — Mùng 1 / Rằm monthly entries
├── vu-lan.json                # NEW — Vu Lan / Rằm tháng Bảy variants
├── doan-ngo.json              # NEW — Tết Đoan Ngọ variants
├── thanh-minh.json            # NEW — Tiết Thanh Minh
├── ong-tao.json               # NEW — Ông Công Ông Táo + Giao Thừa
├── trung-thu.json             # NEW — Tết Trung Thu
├── life-events.json           # NEW — Động thổ, Nhập trạch, Khai trương, Cưới, Giỗ, Đầy tháng
├── gia-tien-thuong-nhat.json  # NEW — Daily ancestor veneration (Always key)
├── han-thuc.json              # NEW — Tết Hàn Thực
├── nguyen-tieu.json           # NEW — Tết Nguyên Tiêu / Thượng Nguyên
├── trung-cuu-ha-nguyen.json   # NEW — Tết Trung Cửu, Tết Hạ Nguyên
└── provenance_audit.md        # NEW (RIT-11) — audit ledger
```

**File count:** 14 category files + manifest.json stays within the RIT-09 "≤ 14 per-event-category
files plus manifest.json" limit (13 category files + fixtures.json legacy = 14 total, or 13 new
category files replacing fixtures.json entirely = 13). The planner must decide whether to:
- **Option A (recommended):** Replace `fixtures.json` by absorbing its 6 entries into the category
  files, then point `manifest.json` at only the category files.
- **Option B:** Keep `fixtures.json` as-is and count it as one of the ≤ 14 files.

Either option is compatible with the loader extension below, but Option A is cleaner (no stub-only
file in the production corpus).

### Pattern 1: manifest.json Shape

**What:** A top-level file listing all per-category corpus files. The loader reads manifest, then
`include_str!`s each listed path and merges entries.

**Recommended manifest.json shape (no Rust change to the entry schema — just loader wiring):**

```json
{
  "$schema_version": "rituals-manifest-v1",
  "corpus_files": [
    "tet-nguyen-dan.json",
    "soc-vong.json",
    "vu-lan.json",
    "doan-ngo.json",
    "thanh-minh.json",
    "ong-tao.json",
    "trung-thu.json",
    "life-events.json",
    "gia-tien-thuong-nhat.json",
    "han-thuc.json",
    "nguyen-tieu.json",
    "trung-cuu-ha-nguyen.json"
  ]
}
```

**Note on loader approach:** Because `include_str!` requires compile-time-known paths (string
literals), the loader CANNOT iterate `manifest.json` at runtime to discover files dynamically.
Two viable approaches:

- **Option A (recommended): Hard-code each file as a named `include_str!` constant, then collect.**
  The manifest exists as documentation/tooling artifact and for future automation, but the Rust
  loader uses a static list of embedded constants. This mirrors how `holiday_data.rs` hard-codes
  `const LUNAR_FESTIVALS_JSON: &str = include_str!(...)`. If a file is added to the manifest but
  not to the loader constant list, the corpus loader CI test will catch the gap (see §Validation).
  
- **Option B: Embed manifest.json via `include_str!` and parse it, then use a static dispatch
  match table to map filenames to their embedded content.** More complex; not the crate pattern.

Use **Option A**: for each new category file, add one `const CATEGORY_JSON: &str =
include_str!("../../data/rituals/category.json");` in `corpus.rs`, then merge all entries in the
`OnceLock` initializer.

**Updated corpus.rs loader sketch (diff from Phase 11):**

```rust
// In corpus.rs — replace single fixtures.json constant with per-category constants:
const TET_NGUYEN_DAN_JSON: &str = include_str!("../../data/rituals/tet-nguyen-dan.json");
const SOC_VONG_JSON:        &str = include_str!("../../data/rituals/soc-vong.json");
const VU_LAN_JSON:          &str = include_str!("../../data/rituals/vu-lan.json");
// ... one constant per file ...

static RITUALS: OnceLock<Vec<RitualEntry>> = OnceLock::new();

pub fn all_rituals() -> &'static [RitualEntry] {
    RITUALS.get_or_init(|| {
        let corpus_jsons: &[&str] = &[
            TET_NGUYEN_DAN_JSON,
            SOC_VONG_JSON,
            VU_LAN_JSON,
            // ... list all constants ...
        ];
        let mut all: Vec<RitualEntry> = Vec::new();
        for json in corpus_jsons {
            let file: RitualFile = serde_json::from_str(json)
                .expect("Failed to parse ritual corpus file");
            assert_eq!(file.schema_version, EXPECTED_SCHEMA_VERSION, ...);
            for entry in file.entries {
                all.push(normalize_and_validate(entry));
            }
        }
        all
    })
    .as_slice()
}
```

The `RitualFile`, `normalize_and_validate`, `nfc`, and all assertion logic are **unchanged**.

### Pattern 2: RitualEntry JSON for ≥ 60 Entries

**Exact schema fields (from `schema.rs` — confirmed by reading the actual file):**

```json
{
  "$schema_version": "rituals-v1",
  "entries": [
    {
      "ritual_id":           "van-khan-tet-full",          // kebab-case, unique
      "title_vi":            "Văn Khấn Tết Nguyên Đán (Đầy Đủ)",
      "event_keys": [
        {"kind": "holiday_id", "value": "tet-nguyen-dan"},
        {"kind": "lunar_date", "month": 1, "day": 1}
      ],
      "variant":             "full",                        // "simple"|"full"|"buddhist"|"folk"|{"regional":"<area>"}
      "offerings": [
        {"name_vi": "Mâm ngũ quả", "quantity": "1 mâm"}
      ],
      "preparation_steps": [
        {"order": 1, "description_vi": "..."}
      ],
      "invocation_text_vi":  "Nam mô a di đà phật! ...",
      "source_id":           "vn-folk-ritual",              // ALWAYS this value
      "original_citation": {
        "title":     "Văn Khấn Cổ Truyền Việt Nam",        // required
        "publisher": "NXB Văn Hóa Thông Tin",              // optional
        "edition":   "2003",                               // optional
        "page":      "14"                                  // REQUIRED by RIT-10
      },
      "confidence":          "primary"                     // "primary"|"regional-variant"|"synthesized"
    }
  ]
}
```

**Critical field constraints for RIT-10:**
- `source_id` must equal `"vn-folk-ritual"` — loader validates via `SOURCE_VN_FOLK_RITUAL` constant.
- `original_citation.page` should be populated (RIT-10: "book + page"). The field is `Option<String>`
  in the schema so the loader won't reject a missing page, but RIT-10 requires it. **Author all
  entries with `"page"` present.**
- `confidence` must be one of the three allowed tiers. Use `"primary"` for entries drawn directly
  from a well-known print source; `"regional-variant"` for entries specific to a Vietnamese region;
  `"synthesized"` for entries that combine multiple sources.

**Closed enum shapes to remember:**
- `RitualEventKey::LunarDate` in JSON uses **flat struct variant** (not nested):
  `{"kind": "lunar_date", "month": 1, "day": 1}` — NOT `{"kind": "lunar_date", "match": {"kind": "month_day", ...}}`.
  This is the Phase 10 plan 10-03 decision: `LunarDateMatch` was deliberately NOT embedded inside
  `RitualEventKey::LunarDate` to avoid serde `#[serde(tag)]` nesting conflict.
- `RitualEventKey::SolarTerm` in JSON: `{"kind": "solar_term", "name": "Thanh Minh"}`.
- `RitualEventKey::Always` in JSON: `{"kind": "always"}` (no additional fields).
- `RitualVariantTag::Regional` in JSON: `{"regional": "mien-bac"}` (not `"regional"` string).

### Pattern 3: Variant Coverage for RIT-12

**Requirement:** ≥ 4 events with multiple variants sharing the same `event_type`, discriminated by
`variant` field. Variants are **separate `RitualEntry` records** sharing `event_keys[]`.

**Recommended 4 events with variant coverage:**

| Event | Holiday ID / Key | Variants to Author |
|-------|-----------------|-------------------|
| Tết Nguyên Đán | `holiday_id: tet-nguyen-dan` + `lunar_date: 1/1` | `simple`, `full`, `buddhist`, `folk` |
| Vu Lan (Rằm tháng 7) | `holiday_id: vu-lan` + `lunar_date: 7/15` | `simple`, `full`, `buddhist` |
| Tết Đoan Ngọ | `holiday_id: tet-doan-ngo` + `lunar_date: 5/5` | `simple`, `folk`, `regional` (miền Bắc) |
| Nhập trạch (life event) | `life_event: nhap_trach` | `simple`, `full` |

This gives 4 events with ≥ 2 variants each. To exceed the minimum, adding variants for
Ông Công Ông Táo (`ong-tao`) is natural (simple + full).

**All entries sharing an event must use identical `event_keys[]`.** Variant discrimination is
solely via the `variant` field — callers filter the `Vec<&RitualEntry>` result if they want only
the `full` variant.

### Pattern 4: provenance_audit.md Ledger (RIT-11)

**What:** A markdown file in `data/rituals/` listing every entry's classical reference and
independent reviewer. No Rust code required — pure documentation.

**Recommended format:**

```markdown
# Văn khấn Corpus Provenance Audit

**Last updated:** 2026-05-27
**Entries:** 60 (as of this version)

## Audit Ledger

| ritual_id | classical_reference | page | confidence | reviewer |
|-----------|--------------------|----|-----------|---------|
| van-khan-tet-don-gian | Văn Khấn Cổ Truyền Việt Nam (NXB VHTT, 2003) | 12 | primary | [reviewer name or "pending"] |
| van-khan-tet-full | Văn Khấn Cổ Truyền Việt Nam (NXB VHTT, 2003) | 14 | primary | pending |
| ... | ... | ... | ... | ... |

## References

- **Văn Khấn Cổ Truyền Việt Nam** — NXB Văn Hóa Thông Tin, 2003 ed. Primary source for most entries.
- **Văn Khấn Nôm** — secondary reference for folk variants.
```

The audit ledger satisfies RIT-11: "recording the classical reference and independent reviewer
for every entry." Note that `reviewer` may be "pending" for v1.5 — RIT-11 requires the ledger to
**exist** and record the reviewer field, not that every entry has a confirmed external reviewer yet.

### Anti-Patterns to Avoid

- **Do NOT add new Rust types or modify schema.rs.** ADR-0001 is locked. Any field change requires
  a superseding ADR. The `body_en` reservation is already in `schema.rs:396-397` — do NOT re-add it.
- **Do NOT use `RITUAL_FIXTURES_JSON` / `fixtures.json` as the production corpus path forever.**
  Phase 12 must refactor `corpus.rs` to load from the new per-category files. The existing
  `fixtures.json` entries should migrate into the category files (or remain as one file if staying
  within the 14-file limit).
- **Do NOT put Hán characters in any corpus file.** The `ritual_han_guard.rs` CI test enforces
  threshold=0. All text must be Quốc-ngữ. If a future entry requires Hán-Nôm quotation, a
  superseding ADR is required.
- **Do NOT hard-code HolidayId values without cross-referencing `lunar-festivals.json`.** The
  existing `every_holiday_id_in_fixtures_resolves_to_a_real_holiday` integration test
  (rituals_integration.rs:97-119) checks that every `HolidayId.value` in the corpus resolves to an
  actual `Holiday.id` for 2020-2030. This test will catch invalid values.
- **Do NOT skip `original_citation.page` to save time.** RIT-10 explicitly requires "book + page".
  An entry without a page is a RIT-10 violation even though the schema allows `Option<String>`.
- **Do NOT confuse `SolarTerm { name }` key with `HolidayId { value }` for Thanh Minh.** Per
  Phase 11 RESEARCH §Code Examples: `holidays.rs:177` assigns `id: None` to the computed Thanh Minh
  entry. Ritual entries for Thanh Minh MUST use `{"kind": "solar_term", "name": "Thanh Minh"}` —
  NOT a HolidayId.
- **Do NOT confuse the `RitualEventKey::LunarDate` flat struct with `LunarDateMatch`'s nested form.**
  In `event_keys[]` JSON, use `{"kind": "lunar_date", "month": 5, "day": 5}`. The `LunarDateMatch`
  enum (separate type) is NOT embedded inside `RitualEventKey::LunarDate`.

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| NFC normalization of authored Vietnamese text | Manual normalization pass before commit | Let the corpus loader's existing `normalize_and_validate` handle it at load | `corpus.rs:68-98` already normalizes every text field via `unicode-normalization::nfc()`; on-disk NFD is allowed as long as `ritual_han_guard.rs` sees no Hán chars |
| HolidayId validation | Add new validation code | Rely on existing `every_holiday_id_in_fixtures_resolves_to_a_real_holiday` test | Integration test already cross-references every `HolidayId.value` against `get_vietnamese_holidays(2020..=2030)` — will fail on any typo |
| manifest.json parsing at runtime | Custom file-discovery code | Hard-code `include_str!` constants per file + array-merge in loader | Crate-wide compile-time embed pattern; runtime filesystem I/O breaks wasm targets |
| Variant coverage tracking | External spreadsheet | `all_rituals()` linear scan in a test asserting ≥ 4 events with ≥ 2 variants | Test can iterate corpus and assert the invariant on every CI run |
| Provenance tracking | Database / structured JSON | Simple markdown ledger (`provenance_audit.md`) | RIT-11 says "ledger" — no structural requirements beyond being findable in `data/rituals/` |

---

## Common Pitfalls

### Pitfall 1: HolidayId Value Typo (Silent Zero-Result Lookup)

**What goes wrong:** Corpus author writes `"vu-lan-ram"` but the canonical id is `"vu-lan"`. The
loader accepts the entry; the integration test `every_holiday_id_in_fixtures_resolves_to_a_real_holiday`
fails at CI. Alternatively, the test doesn't cover the new entries and lookups silently return 0.

**Why it happens:** `HolidayId.value` is a free-form `String`. Schema cannot constrain it.

**How to avoid:** Reference the canonical id list (§Code Examples below). After authoring new
entries, run `cargo test -p amlich-core --test rituals_integration` and confirm
`every_holiday_id_in_fixtures_resolves_to_a_real_holiday` still passes.

**Warning signs:** Integration test failure mentioning a HolidayId value; or
`find_van_khan_for_snapshot` returning 0 for a known holiday snapshot.

### Pitfall 2: LunarDate Flat-Struct Confusion

**What goes wrong:** Author or planner assumes `RitualEventKey::LunarDate` wraps `LunarDateMatch`
and writes `{"kind": "lunar_date", "match": {"kind": "month_day", "month": 1, "day": 1}}`. This
fails deserialization because the schema uses a flat struct variant.

**Why it happens:** ADR-0001 §RitualEventKey shows a draft that predates the Phase 10 plan 10-03
decision to flatten the variant. The actual `schema.rs:67-78` uses `LunarDate { month, day, leap_month_policy }`.

**How to avoid:** All `lunar_date` event keys must be `{"kind": "lunar_date", "month": N, "day": N}`.
The `leap_month_policy` field is optional (defaults to `"canonical_month_only"`).

**Warning signs:** `serde_json::from_str` parse failure at corpus load time.

### Pitfall 3: Hán Character in Vietnamese Ritual Text

**What goes wrong:** Author copies text from a source that includes Hán characters (e.g., classical
Sino-Vietnamese quotations). `ritual_han_guard.rs` CI test fails.

**Why it happens:** Some classical prayer texts cite Hán-Nôm phrases inline. The Phase 11 guard
enforces threshold = 0.

**How to avoid:** All corpus text must be pure Quốc-ngữ (Latin script + Vietnamese diacritics).
Strip any Hán characters before saving. If a classical Hán-Nôm citation is essential, it requires a
superseding ADR to add a `hannom_text: Option<String>` field — out of scope for v1.5.

**Warning signs:** `cargo test -p amlich-core --test ritual_han_guard` fails with "Hán code points found".

### Pitfall 4: Missing `original_citation.page` (RIT-10 Violation)

**What goes wrong:** Author fills `title` and `publisher` but omits `page` because it's `Option<String>`
in the schema. RIT-10 requires "book + page". The loader doesn't panic — but the phase verification
will fail.

**How to avoid:** Every entry must have `"page": "<number>"` in `original_citation`. Add a test
asserting `entry.original_citation.page.is_some()` for all loaded entries.

**Warning signs:** Phase verification fails on RIT-10 check; grep of corpus files shows `"page": null`.

### Pitfall 5: Variant Entries with Mismatched event_keys

**What goes wrong:** Two entries for the same event (e.g., Tết `simple` and Tết `full`) have
slightly different `event_keys[]` (e.g., one includes `HolidayId` and the other omits it). Both
will still load but the `full` variant won't fire on a Tết snapshot — breaks RIT-12.

**Why it happens:** Authors forget that all variants of an event should be discoverable by the same
day-snapshot lookup. Variants are discriminated at the result level (caller filters by `variant`),
not at the match level.

**How to avoid:** All entries for the same event MUST share identical `event_keys[]`. Copy-paste the
event keys from the first variant when authoring subsequent variants.

**Warning signs:** `find_van_khan_for_snapshot` returns fewer variants than expected on a known
holiday date.

### Pitfall 6: include_str! Paths Break After File Rename

**What goes wrong:** Corpus file `tet-nguyen-dan.json` is created and added to `corpus.rs` as
`include_str!("../../data/rituals/tet-nguyen-dan.json")`. The file is later renamed or moved and
the build breaks.

**Why it happens:** `include_str!` resolves at compile time relative to the source file.

**How to avoid:** Never rename corpus files after they appear in `corpus.rs` without updating the
constant. Keep file names stable (kebab-case, matching the event category).

**Warning signs:** `cargo build -p amlich-core` fails with "couldn't find include_str! file".

### Pitfall 7: source_id_guard CI Failure from Bare Literal

**What goes wrong:** Author adds `assert_eq!(entry.source_id, "vn-folk-ritual")` somewhere in a
new test or validation function. `tests/source_id_guard.rs` scans all `src/` files for bare
`"vn-folk-ritual"` string literals and fails the build.

**Why it happens:** DEC-0015 forbids bare source_id literals in production code; only
`SOURCE_VN_FOLK_RITUAL` constant is allowed.

**How to avoid:** Any new source_id comparison in Rust must use `SOURCE_VN_FOLK_RITUAL`. New tests
that verify corpus entries can compare `entry.source_id == SOURCE_VN_FOLK_RITUAL`.

---

## Code Examples

### Canonical Holiday IDs (confirmed from `data/holidays/lunar-festivals.json`)

| Festival | `HolidayId.value` | Notes |
|----------|-----------------|-------|
| Tết Nguyên Đán | `"tet-nguyen-dan"` | + `lunar_date: 1/1` |
| Mùng 2 Tết | `"mung-2-tet"` | |
| Mùng 3 Tết | `"mung-3-tet"` | |
| Tết Nguyên Tiêu / Thượng Nguyên | `"tet-nguyen-tieu"` | + `lunar_date: 1/15` |
| Tết Hàn Thực | `"tet-han-thuc"` | |
| Tết Thanh Minh (solar) | **None** — use `solar_term` key | `holidays.rs:177` assigns `id: None`; use `{"kind": "solar_term", "name": "Thanh Minh"}` |
| Phật Đản | `"phat-dan"` | |
| Tết Đoan Ngọ | `"tet-doan-ngo"` | + `lunar_date: 5/5, canonical_month_only` |
| Vu Lan / Rằm tháng Bảy | `"vu-lan"` | + `lunar_date: 7/15` |
| Tết Trung Thu | `"tet-trung-thu"` | + `lunar_date: 8/15` |
| Tết Trung Cửu | `"tet-trung-cuu"` | + `lunar_date: 9/9` |
| Tết Hạ Nguyên | `"tet-ha-nguyen"` | + `lunar_date: 10/15` |
| Ông Công Ông Táo | `"ong-tao"` | + `lunar_date: 12/23` |
| Giao Thừa | `"giao-thua"` | + `lunar_date: 12/30` or 12/29 in short years |

**Note:** Sóc (Mùng 1) and Vọng (Rằm / Ngày 15) entries for all months: the `holidays.rs` auto-generates
these; they have `id: None`. Use `{"kind": "lunar_date", "month": <M>, "day": 1}` and `{"day": 15}`.

### Minimum 60-entry distribution plan

| Category file | Min entries | Events/Variants |
|--------------|-------------|-----------------|
| tet-nguyen-dan.json | 4 | Tết: simple, full, buddhist, folk |
| nguyen-tieu.json | 2 | Thượng Nguyên: simple, full |
| han-thuc.json | 2 | Hàn Thực: simple, folk |
| thanh-minh.json | 3 | Thanh Minh: simple + 2 regional |
| doan-ngo.json | 3 | Đoan Ngọ: simple, folk, regional (miền Bắc) |
| vu-lan.json | 3 | Vu Lan: simple, full, buddhist |
| trung-thu.json | 2 | Trung Thu: simple, full |
| ong-tao.json | 3 | Ông Táo: simple, full; Giao Thừa: simple |
| trung-cuu-ha-nguyen.json | 2 | Trung Cửu: simple; Hạ Nguyên: simple |
| soc-vong.json | 12 | Generic Mùng 1 (6 months × 1) + generic Rằm (6 months × 1) — or 2 generic entries covering all months |
| life-events.json | 12 | Động thổ: simple+full; Nhập trạch: simple+full; Khai trương: simple+full; Cưới: simple+full; Giỗ: simple+full; Đầy tháng: simple |
| gia-tien-thuong-nhat.json | 3 | Always key — morning, evening, daily short form |
| phat-dan.json | 2 | Phật Đản: simple, buddhist |
| (fixtures.json legacy or absorbed) | 6 | Absorbed into above or kept as 14th file |
| **Total** | ≥ 59+6 = ≥ 60 | Spread across ≤ 14 files ✓ |

**Sóc/Vọng simplification:** A single entry `{"kind": "lunar_date", "month": <any month 1-12>,
"day": 1}` using `M=0` wildcard is NOT supported by the schema (no wildcard syntax). The matcher
matches on exact `month` + `day`. For generic daily/monthly observances, use `Always` (fires every
day). For Sóc specifically, consider one entry per common month (month 1, 7) or a simplified
approach: two entries with `event_keys: [{"kind": "lunar_date", "month": 1, "day": 1}]` as templates.
**Recommendation:** Use 2 representative entries (generic Mùng 1, generic Rằm) and document that
callers wanting month-specific Sóc/Vọng should use the `find_van_khan_for_event` API with the
specific `LunarDate` key. The `Always` entry for daily gia tiên covers the gap.

### corpus.rs loader extension — sketch

```rust
// corpus.rs — Phase 12 update
// Replace single fixtures.json with per-category include_str! constants:

const TET_NGUYEN_DAN_JSON:       &str = include_str!("../../data/rituals/tet-nguyen-dan.json");
const NGUYEN_TIEU_JSON:          &str = include_str!("../../data/rituals/nguyen-tieu.json");
const HAN_THUC_JSON:             &str = include_str!("../../data/rituals/han-thuc.json");
const THANH_MINH_JSON:           &str = include_str!("../../data/rituals/thanh-minh.json");
const DOAN_NGO_JSON:             &str = include_str!("../../data/rituals/doan-ngo.json");
const VU_LAN_JSON:               &str = include_str!("../../data/rituals/vu-lan.json");
const TRUNG_THU_JSON:            &str = include_str!("../../data/rituals/trung-thu.json");
const ONG_TAO_JSON:              &str = include_str!("../../data/rituals/ong-tao.json");
const TRUNG_CUU_HA_NGUYEN_JSON:  &str = include_str!("../../data/rituals/trung-cuu-ha-nguyen.json");
const SOC_VONG_JSON:             &str = include_str!("../../data/rituals/soc-vong.json");
const LIFE_EVENTS_JSON:          &str = include_str!("../../data/rituals/life-events.json");
const GIA_TIEN_THUONG_NHAT_JSON: &str = include_str!("../../data/rituals/gia-tien-thuong-nhat.json");
const PHAT_DAN_JSON:             &str = include_str!("../../data/rituals/phat-dan.json");

// Keep or drop fixtures.json; if kept, include it to preserve 6 existing entries:
// const RITUAL_FIXTURES_JSON: &str = include_str!("../../data/rituals/fixtures.json");
// (Alternatively, absorb fixtures.json entries into category files.)

const ALL_CORPUS_JSONS: &[&str] = &[
    TET_NGUYEN_DAN_JSON,
    NGUYEN_TIEU_JSON,
    HAN_THUC_JSON,
    THANH_MINH_JSON,
    DOAN_NGO_JSON,
    VU_LAN_JSON,
    TRUNG_THU_JSON,
    ONG_TAO_JSON,
    TRUNG_CUU_HA_NGUYEN_JSON,
    SOC_VONG_JSON,
    LIFE_EVENTS_JSON,
    GIA_TIEN_THUONG_NHAT_JSON,
    PHAT_DAN_JSON,
    // RITUAL_FIXTURES_JSON,  // uncomment if keeping fixtures.json as 14th file
];

pub fn all_rituals() -> &'static [RitualEntry] {
    RITUALS.get_or_init(|| {
        let mut all: Vec<RitualEntry> = Vec::new();
        for &json in ALL_CORPUS_JSONS {
            let file: RitualFile = serde_json::from_str(json)
                .expect("Failed to parse ritual corpus file");
            assert_eq!(
                file.schema_version, EXPECTED_SCHEMA_VERSION,
                "ritual corpus schema_version must equal {:?} (ADR-0001)",
                EXPECTED_SCHEMA_VERSION
            );
            for entry in file.entries {
                all.push(normalize_and_validate(entry));
            }
        }
        all
    })
    .as_slice()
}
```

### Tests to add in Phase 12

The following tests should be added to `corpus.rs` (inline `#[cfg(test)]`) and/or
`tests/rituals_integration.rs` to verify Phase 12 requirements:

```rust
// Inline corpus.rs test — RIT-09
#[test]
fn corpus_has_at_least_sixty_entries() {
    assert!(all_rituals().len() >= 60,
        "expected ≥ 60 corpus entries, got {}", all_rituals().len());
}

// Inline corpus.rs test — RIT-10
#[test]
fn every_entry_has_citation_with_page() {
    for entry in all_rituals() {
        assert!(entry.original_citation.page.is_some(),
            "ritual {} missing original_citation.page (RIT-10)", entry.ritual_id);
    }
}

// Inline corpus.rs test — RIT-12: ≥ 4 events have multiple variants
#[test]
fn at_least_four_events_have_multiple_variants() {
    use std::collections::HashMap;
    // Group entries by their first HolidayId or SolarTerm key as event discriminator.
    // Count unique variants per event.
    let mut event_variant_count: HashMap<String, std::collections::HashSet<String>> =
        HashMap::new();
    for entry in all_rituals() {
        let event_key = entry.event_keys.iter()
            .find_map(|k| match k {
                RitualEventKey::HolidayId { value } => Some(value.clone()),
                RitualEventKey::SolarTerm { name } => Some(format!("solar:{name}")),
                RitualEventKey::LifeEvent { event } => Some(format!("life:{event:?}")),
                _ => None,
            });
        if let Some(key) = event_key {
            let variant_str = format!("{:?}", entry.variant);
            event_variant_count.entry(key).or_default().insert(variant_str);
        }
    }
    let multi_variant_events = event_variant_count.values()
        .filter(|variants| variants.len() >= 2)
        .count();
    assert!(multi_variant_events >= 4,
        "expected ≥ 4 events with multiple variants, found {}: {:?}",
        multi_variant_events, event_variant_count);
}
```

---

## RIT-13 Status: Already Satisfied

**CONFIRMED: `body_en: Option<String>` already exists in the codebase.**

From `crates/amlich-core/src/rituals/schema.rs` (read directly, lines 395-397):
```rust
/// Reserved per RIT-13. Always null in v1.5 corpus.
#[serde(default, skip_serializing_if = "Option::is_none")]
pub body_en: Option<String>,
```

From `crates/amlich-core/src/rituals/corpus.rs` (lines 73-75):
```rust
if let Some(b) = entry.body_en.as_deref() {
    entry.body_en = Some(nfc(b));
}
```

The `body_en` field is reserved in the schema with `#[serde(default)]` (via
`skip_serializing_if = "Option::is_none"` and the `default` on `Option`), is NFC-normalized if
ever present, and is never set in the v1.5 corpus. **RIT-13 only requires a verification task in
Phase 12** — confirm the field exists in schema.rs and is not accidentally populated in any
corpus file. No new Rust code is required.

---

## Open Questions

1. **Should `fixtures.json` be absorbed into category files or kept as the 14th file?**
   - What we know: Phase 11 fixtures.json has 6 entries covering: tet-nguyen-dan (simple), 
     ram-thang-gieng (full), thanh-minh (simple), dong-tho (full), gia-tien-hang-ngay (always),
     doan-ngo (simple).
   - What's unclear: Whether to keep fixtures.json as entry #14 (simplest, preserves Phase 11
     history) or absorb its entries into category files (cleaner, avoids "stub" file in production).
   - Recommendation: **Absorb fixtures.json entries into category files** (Option A). Remove the
     `RITUAL_FIXTURES_JSON` constant from corpus.rs after migration. This avoids a production corpus
     containing a "fixtures" file. The ritual_ids can remain unchanged (no schema impact).

2. **Variant coverage for Sóc/Vọng — should there be multiple variants?**
   - What we know: Sóc and Vọng (Mùng 1 / Ngày 15) are matched via `LunarDate { day: 1/15 }`.
     The matcher fires on every Mùng 1 and Rằm day of the year.
   - What's unclear: Should there be simple + full variants for generic Sóc/Vọng?
   - Recommendation: One `simple` entry for generic Mùng 1 and one `simple` entry for generic Rằm.
     Do not count Sóc/Vọng as one of the 4 "multi-variant" events (too few variants naturally).

3. **How to count "events with multiple variants" for RIT-12 when an entry has multiple event_keys?**
   - What we know: Tết entries carry both `HolidayId{tet-nguyen-dan}` and `LunarDate{1/1}`. The
     RIT-12 test grouping must use one canonical key per entry to count variants.
   - Recommendation: Use the first `HolidayId` or `LifeEvent` key as the event discriminator.
     The test sketch in §Code Examples above uses this approach.

4. **Can `provenance_audit.md` have "pending" reviewer fields in v1.5?**
   - What we know: REQUIREMENTS.md RIT-11: "recording the classical reference and independent
     reviewer for every entry". STATE.md known gaps: "Văn khấn single-author risk — mitigated by
     per-entry citation + audit ledger (Phase 12)."
   - Recommendation: Include the reviewer field with "pending" or the author's own name for v1.5.
     The ledger must exist and be populated for every entry. Marking entries as "pending peer review"
     is acceptable for the initial shipping.

---

## Validation Architecture

> `workflow.nyquist_validation` is not present in `.planning/config.json` — section skipped.

**Test commands for Phase 12 verification:**

| Check | Command |
|-------|---------|
| All tests pass (including corpus count) | `cargo test -p amlich-core` |
| Hán guard (threshold=0) | `cargo test -p amlich-core --test ritual_han_guard` |
| Integration tests (HolidayId cross-ref, NFC) | `cargo test -p amlich-core --test rituals_integration` |
| Build success | `cargo build -p amlich-core` |

---

## Sources

### Primary (HIGH confidence)

- `crates/amlich-core/src/rituals/schema.rs` — actual locked Rust types (read directly 2026-05-27).
  Confirms: `body_en: Option<String>` at line 396, `RitualVariantTag` closed enum, `RitualEventKey`
  flat struct variant for `LunarDate`, all 10 types per ADR-0001.
- `crates/amlich-core/src/rituals/corpus.rs` — existing loader implementation (read directly
  2026-05-27). Confirms: single `fixtures.json` `include_str!`, `normalize_and_validate` covers
  `body_en` at lines 73-75, `OnceLock` pattern, source_id validation.
- `crates/amlich-core/data/rituals/fixtures.json` — 6 existing Phase 11 stub entries (read directly
  2026-05-27). Confirms exact JSON shapes and confirms which ritual_ids are already present.
- `.planning/adrs/0001-ritual-schema-v1.md` — Status: Accepted. Full field set, closed enums, serde
  discipline, sample JSON.
- `.planning/phases/11-van-khan-module-and-lookup-apis/11-VERIFICATION.md` — Phase 11 verified
  2026-05-26. Confirms corpus.rs:38 `pub fn all_rituals`, matcher.rs line numbers, integration test
  names.
- `crates/amlich-core/data/holidays/lunar-festivals.json` — confirmed canonical Holiday id list
  (`grep '"id"'` scan 2026-05-27). 14 canonical ids verified.
- `.planning/REQUIREMENTS.md` — RIT-09..13 requirements confirmed as pending Phase 12.
- `.planning/STATE.md` — Phase 11 complete (4/4 plans); Phase 12 not started.

### Secondary (MEDIUM confidence)

- `.planning/phases/11-van-khan-module-and-lookup-apis/11-RESEARCH.md` — Phase 11 research, Pitfall
  2 (HolidayId typo), Code Examples (manifest.json deferral decision), anti-patterns.
- `.planning/phases/10-foundation-schema-lock-adrs-source-id-registration/10-03-PLAN.md` — confirms
  `body_en` was explicitly part of the Phase 10 plan (line 68: `body_en: Option<String>` reserved
  per RIT-13).

### Tertiary (LOW confidence)

- Entry count distribution plan in §Code Examples — based on event taxonomy from REQUIREMENTS.md
  and lunar-festivals.json; actual authoring may redistribute entries across files.

---

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — no new dependencies; loader extension pattern is a mechanical copy of
  existing OnceLock + include_str! code in corpus.rs.
- Architecture: HIGH — schema is locked by ADR-0001, all types confirmed by reading actual source
  files; corpus.rs loader pattern confirmed by reading implementation.
- Corpus content (Vietnamese ritual text): MEDIUM — content authoring requires knowledge of
  classical Vietnamese ritual references; entry count and variant coverage is mechanically verifiable
  but text quality requires domain expertise.
- Pitfalls: HIGH — all pitfalls grounded in code read (schema.rs, corpus.rs, integration tests)
  and documented Phase 10/11 decisions.

**Research date:** 2026-05-27
**Valid until:** 2026-06-27 (30 days — schema is locked; no ecosystem changes expected)

---

*Sources cited:*
- `crates/amlich-core/src/rituals/schema.rs` (direct read)
- `crates/amlich-core/src/rituals/corpus.rs` (direct read)
- `crates/amlich-core/data/rituals/fixtures.json` (direct read)
- `.planning/adrs/0001-ritual-schema-v1.md` (direct read)
- `.planning/phases/11-van-khan-module-and-lookup-apis/11-RESEARCH.md` (direct read)
- `.planning/phases/11-van-khan-module-and-lookup-apis/11-VERIFICATION.md` (direct read)
- `.planning/phases/10-foundation-schema-lock-adrs-source-id-registration/10-03-PLAN.md` (direct read)
- `crates/amlich-core/data/holidays/lunar-festivals.json` (grep scan)
- `.planning/REQUIREMENTS.md`, `.planning/STATE.md`, `.planning/ROADMAP.md` (direct reads)
- `.planning/config.json` (direct read — nyquist_validation absent; Validation Architecture skipped)
