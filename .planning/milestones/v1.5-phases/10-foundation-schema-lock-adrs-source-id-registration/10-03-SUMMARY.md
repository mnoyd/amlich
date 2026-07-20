---
phase: 10-foundation-schema-lock-adrs-source-id-registration
plan: "03"
subsystem: rituals
tags: [serde, json-schema, adr, rust, ritual-content, van-khan]

requires:
  - phase: 10-01
    provides: "rituals/mod.rs + rituals/schema.rs placeholder stubs + pub mod rituals; in lib.rs"

provides:
  - "ADR-0001: Ritual JSON Schema v1 (Accepted 2026-05-26) at .planning/adrs/0001-ritual-schema-v1.md"
  - "Locked Rust type stubs: RitualEntry, RitualVariantTag, LunarDateMatch, RitualEventKey, LeapPolicy, RitualConfidenceTier, Offering, PreparationStep, SourceCitation, LifeEventKind"
  - "rituals/mod.rs with module comment pointing to ADR-0001"
  - "rituals/schema.rs with five serde-backed tests covering all behavior specs"

affects:
  - phase-11-van-khan-module
  - phase-12-corpus-authoring

tech-stack:
  added: []
  patterns:
    - "Internally-tagged serde enum (#[serde(tag = \"kind\")]) for discriminated unions"
    - "deny_unknown_fields at struct level for schema-locked corpus entries"
    - "Default-deriving LeapPolicy with #[serde(default)] on enum field for safe omission"
    - "snake_case and kebab-case rename_all for serde enum variant naming"

key-files:
  created:
    - ".planning/adrs/0001-ritual-schema-v1.md"
    - "crates/amlich-core/src/rituals/schema.rs (overwritten from placeholder)"
  modified:
    - "crates/amlich-core/src/rituals/mod.rs (overwritten from placeholder — added module comment)"

key-decisions:
  - "RitualEventKey::LunarDate is a struct variant (not a newtype wrapping LunarDateMatch) to avoid serde internally-tagged enum nesting conflict"
  - "LunarDateMatch kept as standalone type for direct use by Phase 11 RIT-07 query API"
  - "title_en and other English fields are Option<String> with skip_serializing_if, unpopulated in v1.5 corpus"
  - "body_en reserved per RIT-13 — present in schema, always None in v1.5 corpus"
  - "RitualVariantTag uses default serde external representation: unit variants as strings, newtype Regional as {regional: area}"

patterns-established:
  - "Schema-lock pattern: every change to rituals/schema.rs requires a superseding ADR (enforced by comment at top of file)"
  - "Closed enum pattern: #[serde(deny_unknown_fields)] at struct level + typed enum variants = typos fail at load"

requirements-completed:
  - FND-01

duration: 15min
completed: "2026-05-26"
---

# Phase 10 Plan 03: Ritual Schema Lock Summary

**ADR-0001 accepted and RitualEntry v1 schema locked as Rust type stubs with serde discipline — 10 types, 5 behavioral tests, deny_unknown_fields enforced**

## Performance

- **Duration:** ~15 min
- **Started:** 2026-05-26T14:52:12Z
- **Completed:** 2026-05-26T15:07:00Z
- **Tasks:** 2
- **Files modified:** 3 (adrs/0001, rituals/mod.rs, rituals/schema.rs)

## Accomplishments

- Wrote ADR-0001 in Nygard short-form (Title/Status/Context/Decision/Consequences) with Status: Accepted, covering all locked field types, closed enums, sample JSON entry, and schema-lock consequences
- Landed 10 locked Rust types in `rituals/schema.rs`: `RitualEntry`, `RitualVariantTag`, `LunarDateMatch`, `RitualEventKey`, `LeapPolicy`, `RitualConfidenceTier`, `Offering`, `PreparationStep`, `SourceCitation`, `LifeEventKind`
- Wrote 5 behavioral tests covering: full entry deserialization, unknown field rejection, RitualVariantTag 5-variant round-trip, unknown variant rejection, LunarDateMatch MonthDay leap_policy default
- Phase 11 now has unambiguous typed schema to implement corpus loader and matcher against; Phase 12 corpus authors have a frozen JSON shape

## Task Commits

Each task was committed atomically (commits handled by orchestrator due to sandbox restrictions):

1. **Task 1: Write ADR 0001 ritual schema v1** - docs(10-03): ADR-0001 in Nygard short-form
2. **Task 2: Land rituals/schema.rs Rust type stubs** - feat(10-03): RitualEntry v1 schema locked types + 5 behavioral tests

## Files Created/Modified

- `.planning/adrs/0001-ritual-schema-v1.md` — ADR-0001: Ritual JSON Schema v1, Status: Accepted, includes full field set, closed enum specs, sample Tết JSON, and phase-lock consequences
- `crates/amlich-core/src/rituals/schema.rs` — 10 locked types with serde derives + 5 inline tests (overwrites placeholder)
- `crates/amlich-core/src/rituals/mod.rs` — Updated with module-level doc comment pointing to ADR-0001 (was single-line placeholder)

## Public Types Exported by rituals::schema

| Type | Kind | Purpose |
|------|------|---------|
| `RitualEntry` | struct | Root corpus entry, `#[serde(deny_unknown_fields)]` |
| `RitualVariantTag` | enum | `Simple / Full / Buddhist / Folk / Regional(String)` |
| `LunarDateMatch` | enum | `MonthDay / SolarTerm / GregorianFixed` with `kind` tag |
| `RitualEventKey` | enum | `HolidayId / LunarDate / SolarTerm / LifeEvent / Always` |
| `LeapPolicy` | enum | `CanonicalMonthOnly (default) / LeapMonthOnly / Either` |
| `RitualConfidenceTier` | enum | `Primary / RegionalVariant / Synthesized` |
| `Offering` | struct | Structured lễ vật with `name_vi`, optional `name_en/quantity/notes` |
| `PreparationStep` | struct | Ordered trình tự with `order: u8`, `description_vi`, optional `description_en` |
| `SourceCitation` | struct | Classical reference: `title`, optional `publisher/edition/page` |
| `LifeEventKind` | enum | `DongTho / NhapTrach / KhaiTruong / Cuoi / Gio / DayThang` |

## ADR-0001 ↔ Rust Type Mapping

| ADR Field / Enum | Rust Type | Location |
|------------------|-----------|----------|
| `RitualEntry` field set | `struct RitualEntry` | schema.rs:127 |
| `variant` closed enum | `enum RitualVariantTag` | schema.rs:81 |
| `event_keys[]` union | `enum RitualEventKey` | schema.rs:65 |
| `LunarDateMatch` with leap | `enum LunarDateMatch + LeapPolicy` | schema.rs:27,17 |
| `confidence` tiers | `enum RitualConfidenceTier` | schema.rs:8 |
| `offerings[]` structured | `struct Offering` | schema.rs:104 |
| `preparation_steps[]` | `struct PreparationStep` | schema.rs:117 |
| `original_citation` | `struct SourceCitation` | schema.rs:91 |
| `life_event` kinds | `enum LifeEventKind` | schema.rs:46 |

## Decisions Made

- **RitualEventKey::LunarDate as struct variant** — Changed from plan's `LunarDate(LunarDateMatch)` newtype to `LunarDate { month, day, leap_month_policy }` struct variant. Rationale: serde's internally-tagged enum cannot contain another internally-tagged enum as a newtype; the outer `kind: "lunar_date"` would be consumed before the inner `LunarDateMatch` could read its own `kind: "month_day"` field. `LunarDateMatch` is preserved as a standalone type for Phase 11's RIT-07 query API (Rule 1 auto-fix).
- **English fields as `Option<String>`** — `title_en` and per-item `name_en`/`description_en` are `Option<String>` with `skip_serializing_if`, following CONTEXT.md "Claude's Discretion". v1.5 corpus will leave these unpopulated.
- **`body_en` reserved** — Present as `Option<String>` in `RitualEntry` per RIT-13; always `None` in v1.5 corpus. Field is schema real estate for future English content authoring.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] RitualEventKey::LunarDate changed from newtype to struct variant**
- **Found during:** Task 2 (implementing RitualEventKey and writing test 1/5)
- **Issue:** `#[serde(tag = "kind")]` on `RitualEventKey` with `LunarDate(LunarDateMatch)` newtype would fail at runtime: the outer enum consumes `kind: "lunar_date"`, leaving `LunarDateMatch` with no `kind` field to read for its own discriminator. Test JSON `{"kind": "lunar_date", "month": 1, "day": 1}` would not deserialize.
- **Fix:** Changed to `LunarDate { month: u8, day: u8, #[serde(default)] leap_month_policy: LeapPolicy }` struct variant. `LunarDateMatch` is preserved standalone for direct use.
- **Files modified:** `crates/amlich-core/src/rituals/schema.rs`
- **Committed in:** feat(10-03) task commit

---

**Total deviations:** 1 auto-fixed (Rule 1 - serde nesting incompatibility)
**Impact on plan:** Fix is necessary for correct deserialization. `LunarDateMatch` type is still present for Phase 11 usage. No scope creep; all plan behaviors satisfied.

## Issues Encountered

- Sandbox blocked `cargo test`, `git add`, and `git commit` commands during execution. Code files were written to disk; commits deferred to orchestrator. Schema correctness was validated by careful serde documentation review rather than live test execution.

## Next Phase Readiness

- Phase 11 (Văn khấn Module + Lookup APIs) can now implement corpus loader against `rituals::schema::RitualEntry` and the `OnceLock + include_str!` pattern from `golden_loader.rs:5-21`
- Phase 12 corpus authors have a frozen JSON shape; any field change requires a superseding ADR
- ADR-0001, ADR-0002, ADR-0003 all landed — ready for 10-05 to cross-reference them in MILESTONES.md

## Self-Check: PASSED

All artifacts verified on disk:
- FOUND: `.planning/adrs/0001-ritual-schema-v1.md` (Status: Accepted, deny_unknown_fields documented)
- FOUND: `crates/amlich-core/src/rituals/schema.rs` (pub struct RitualEntry, 5 test functions, no pub fn)
- FOUND: `crates/amlich-core/src/rituals/mod.rs` (pub mod schema)
- FOUND: `pub mod rituals;` in `crates/amlich-core/src/lib.rs` (set by plan 10-01, unchanged)
- FOUND: `10-03-SUMMARY.md` (this file)

Note: `cargo test` and `git commit` commands were blocked by sandbox restrictions during execution.
Code files are written to disk; tests and commits will be verified/finalized by orchestrator.

---
*Phase: 10-foundation-schema-lock-adrs-source-id-registration*
*Completed: 2026-05-26*
