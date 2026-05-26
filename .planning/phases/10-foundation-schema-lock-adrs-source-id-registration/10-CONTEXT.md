# Phase 10: Foundation — Schema Lock + ADRs + Source-ID Registration - Context

**Gathered:** 2026-05-26
**Status:** Ready for planning

<domain>
## Phase Boundary

Lock the v1 JSON schema for ritual entries, lock the public `FlyingStarLayout` API shape, register two new `source_id` constants (`vn-folk-ritual`, `huyen-khong`), write three ADRs (ritual schema v1, monthly Phi Tinh anchor convention, Niên Tử Bạch polarity matrix), and add the additive `Holiday.id: Option<String>` field.

This phase produces **no corpus content** and **no algorithm code**. Phase 11 (Văn khấn module + lookup APIs) consumes the locked ritual schema; Phase 12 (corpus authoring) cannot start until the schema lands; Phase 13 (Phi Tinh primitives + period) consumes the locked `FlyingStarLayout` shape and both Phi Tinh ADRs. Hard gate per `.planning/research/PITFALLS.md` CRIT-1, CRIT-5.

</domain>

<decisions>
## Implementation Decisions

### Ritual variant model (RIT-12)
- Variants are **separate `RitualEntry` records**, each with its own `ritual_id` (e.g., `van-khan-tet-don-gian`, `van-khan-tet-day-du`, `van-khan-tet-phat-giao`). No nested `variants: Vec<…>` substructure; no parent/child record split.
- A **closed Rust enum** `RitualVariantTag { Simple, Full, Buddhist, Folk, Regional(String) }` discriminates variants. JSON `variant: "simple" | "full" | "buddhist" | "folk" | { "regional": "<area>" }` deserialized via serde tag-renaming. Unknown tags fail load (`#[serde(deny_unknown_fields)]` at corpus level).
- Variants link to their parent event **only via shared `event_keys[]`**. No `event_group_id`, no `ritual_id`-prefix naming convention as load-bearing grouping signal.
- `find_van_khan_for_snapshot()` / `find_van_khan_for_event()` **return all matching variants in one `Vec<&RitualEntry>`**. No `variant_filter` parameter. No `_canonical_` convenience method. Caller (UI/CLI) ranks and filters.

### ADR storage format & location
- New canonical directory: **`.planning/adrs/`** (does not yet exist — create in Phase 10).
- Three ADRs land in Phase 10:
  - `0001-ritual-schema-v1.md`
  - `0002-phi-tinh-monthly-anchor.md`
  - `0003-nien-tu-bach-polarity.md`
- **Nygard short-form** template: `Title / Status / Context / Decision / Consequences`. Roughly one page each. No MADR-extended sections.
- Each ADR ships with **`Status: Accepted`**. Future revisions will write new ADRs that mark prior as `Superseded by NNNN`.
- ADR numbering is its own sequence starting at `0001` (independent of existing `DEC-NNNN` ids in `.planning/MILESTONES.md`).
- **Cross-referenced** in `.planning/MILESTONES.md` Key Decisions table — one new row per ADR, linking to the file. Single index for project-level discoverability.

### source_id constants placement
- New module: **`crates/amlich-core/src/sources.rs`** — single home for every `source_id` in the codebase.
- Plain **`pub const SOURCE_*: &str`** form (matches the existing `pub const CAN: [&str; 10]` pattern from `CONVENTIONS.md`). No `SourceId` enum, no helper APIs. Drop-in replacement for current string literals.
- Constants exposed: existing (`SOURCE_KHCBPPT`, `SOURCE_NGOC_HAP_KY`, `SOURCE_VN_FOLK`, `SOURCE_CUU_DIEU`, `SOURCE_TAM_MENH_THONG_HOI`) **plus** new (`SOURCE_VN_FOLK_RITUAL = "vn-folk-ritual"`, `SOURCE_HUYEN_KHONG = "huyen-khong"`). All seven canonical source_ids live here, full stop.
- **Full migration sweep in Phase 10:** every bare `"khcbppt"` / `"vn-folk"` / `"ngoc-hap-ky"` / `"cuu-dieu"` / `"tam-menh-thong-hoi"` string literal in v1.0–v1.4 source code is replaced with the corresponding constant. Blast radius is deliberate — closes CRIT-1 typo-injection risk completely.
- **CI grep test** in `crates/amlich-core/tests/` walks `crates/amlich-core/src/` (excluding `sources.rs`) and asserts no occurrence of any sanctioned source_id string literal outside the module. Test owns its exclusion list (test fixtures / doc-comment examples explicitly allow-listed).

### Claude's Discretion
The following implementation details were not user-selected for discussion. Planner has flexibility within research-recommended defaults:

- **Bilingual schema scope** — Research recommendation: ship VN-only at v1.5 with `body_en: Option<String>` reserved per RIT-13. Other English fields (`title_en`, `offerings[].en`, `preparation_steps[].en`, `invocation_text_en_summary`) optional in the schema but unpopulated in the v1.5 corpus.
- **Phi Tinh star metadata field shape** — `polarity` vs `auspice` as one combined `nature: StarNature` enum or two separate fields. Planner decides; both must surface through the JSON metadata file.
- **Polarity matrix encoding inside ADR 0003** — markdown table inline in the ADR body, or referenced separate JSON. Planner picks based on size and readability.
- **`provenance_audit.md` ledger format** — deferred to Phase 12 (corpus authoring); Phase 10 only locks that the ledger is required by RIT-11.
- **Sóc/Vọng generated holiday IDs** — Holiday.id remains `None` for the auto-generated Mùng 1 / Rằm entries (no source-data id exists). Ritual matcher joins those days via `RitualEventKey::LunarDate { month, day }` instead of `HolidayId`. Confirmed not a gap — RIT-06 covers Sóc/Vọng via lunar_date.
- **`FlyingStarLayout` struct multiplicity** — one parameterized struct (research recommendation) with a `period: FlyingStarPeriod { Van | Yearly | Monthly }` discriminator is acceptable. Three distinct types is also acceptable if the planner prefers stronger type-level period discrimination. FND-02 only requires the field set `(period, palaces[9], center_star, evidence)` is frozen.

</decisions>

<specifics>
## Specific Ideas

- Constants in `sources.rs` follow the existing project pattern: `pub const SOURCE_HUYEN_KHONG: &str = "huyen-khong";` — same style as `pub const VIETNAM_TIMEZONE: &str = "+07:00";`.
- ADR cross-reference rows in `MILESTONES.md` look like: `| DEC-0023 | 2026-05-26 | Ritual JSON schema v1 locked | [adrs/0001-ritual-schema-v1.md](adrs/0001-ritual-schema-v1.md) |` (column shape matches the existing Key Decisions table).
- `RitualVariantTag::Regional(String)` accepts arbitrary region names but the contained string is NFC-normalized at load (matches the corpus-wide normalization rule from `PITFALLS.md` MOD-4).
- Phase 10 CI grep test must allow `sources.rs` itself, and must allow `tests/` directories where snapshot fixtures legitimately contain source_id strings as JSON data.

</specifics>

<code_context>
## Existing Code Insights

### Reusable Assets
- **`crates/amlich-core/src/almanac/golden_loader.rs:5-21`** — `include_str!` + `OnceLock` + validate-on-load pattern. Phase 10 doesn't load corpus content, but the ADR for ritual schema v1 must specify that Phase 11's `rituals/corpus.rs` follows this exact shape.
- **v1.1.2 Tiết Khí scanner** (referenced by `STATE.md` and `MILESTONES.md`) — already produces deterministic Lập Xuân instants. ADR 0002 (monthly Phi Tinh anchor) and ADR 0003 (Niên Tử Bạch polarity, which is solar-year-anchored) must declare this scanner as their boundary resolver — no new term-scanning code in v1.5.
- **`provenance.rs:65-67`** — `Provenance::almanac_rule(source_id: &'static str, method: &str)` constructor signature. Constants in `sources.rs` plug into this without API change.

### Established Patterns
- **`pub const NAME: [&str; N]` for static data** (per `CONVENTIONS.md`) — drives the plain-const choice for `sources.rs` and forbids adding `SourceId` enum machinery.
- **`Option<T>` + `#[serde(default, skip_serializing_if = "Option::is_none")]`** for additive DTO fields (v1.2 Ten Gods / Kua precedent) — applies to `Holiday.id` (FND-06).
- **`#[serde(deny_unknown_fields)]` at the corpus entry level** — already used by other golden datasets; ritual schema v1 inherits this discipline, so unknown variant tags / unknown event_key kinds fail at deserialization.

### Integration Points
- **`crates/amlich-core/src/lib.rs:10-26`** — add `pub mod sources;` (alphabetical) and reserve `pub mod rituals;` location for Phase 11. Phase 10 only adds `sources`.
- **`crates/amlich-core/src/almanac/mod.rs:1-28`** — reserve location for `pub mod fengshui;` (added in Phase 13, not here).
- **`crates/amlich-core/src/holidays.rs:14-25`** (`Holiday` struct) — add `pub id: Option<String>` field with `#[serde(default, skip_serializing_if = "Option::is_none")]`. Populate from `lunar_festivals[].id` at creation sites (lines 148-198). Default `None` for the generated Mùng 1 / Rằm entries.
- **`.planning/MILESTONES.md` Key Decisions table** — append three new DEC-* rows linking to `.planning/adrs/000X-*.md`.
- **`.planning/research/`** — RESEARCH.md for Phase 10 (when written by gsd-phase-researcher) lives next to this CONTEXT.md in `.planning/phases/10-foundation-.../`.

</code_context>

<deferred>
## Deferred Ideas

- **Full bilingual ritual corpus** — RIT-13 reserves `body_en`; actual English authoring is deferred indefinitely (no milestone scheduled).
- **`provenance_audit.md` ledger format** — required by RIT-11 but ledger structure is defined in Phase 12 (corpus authoring), not here.
- **Custom clippy lint for source_id literals** — Phase 10 uses a CI grep test; promoting to a true lint (dylint or rustc-internal) is deferred unless the grep test proves insufficient.
- **`SourceId` enum + `From<SourceId> for &'static str` ergonomics** — rejected for v1.5 to match existing `pub const` convention; revisit if/when call sites grow tired of bare-string ergonomics.
- **Daily / Hourly Phi Tinh** — out of scope per `REQUIREMENTS.md`; future milestone.
- **Spatial Phi Tinh (Tier 3, Sơn-Hướng)** — deferred to a post-v1.5 milestone per `EXPANSION_FRAMEWORK.md` §3.3.
- **Phi Tinh wired into `interaction/direction_merge.rs`** — explicitly forbidden in v1.5 per `PITFALLS.md` CRIT-3; future-milestone work behind its own DEC.

</deferred>

---

*Phase: 10-foundation-schema-lock-adrs-source-id-registration*
*Context gathered: 2026-05-26*
