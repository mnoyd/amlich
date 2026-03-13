# Engine Contract RFC

## Status

Accepted for the current `amlich-api` redesign and consumer migration boundary.

## Goals

- Define the canonical engine request/response model for day execution.
- Make ruleset selection and recommendation controls first-class inputs.
- Expose discovery surfaces so CLI and API consumers can build valid engine requests without probing runtime behavior blindly.
- Preserve deterministic baseline output while allowing contextual recommendation overlays.
- Set explicit migration boundaries between engine-backed surfaces and retained legacy entrypoints.

## Non-goals

- This RFC does not require every future extension surface to ship now.
- This RFC does not migrate desktop or WASM consumers to full engine-discovery workflows.
- This RFC does not remove all legacy entrypoints immediately.

## Canonical request model

The engine request model is represented by `amlich_api::DateQuery`.

```rust
pub struct DateQuery {
    pub day: i32,
    pub month: i32,
    pub year: i32,
    pub timezone: Option<f64>,
    pub ruleset_id: Option<String>,
    pub event_kind: Option<String>,
    pub enabled_pack_ids: Vec<String>,
}
```

### Request semantics

- `day/month/year`: required civil date inputs.
- `timezone`: optional execution timezone; defaults to the ruleset/core default when omitted.
- `ruleset_id`: optional canonical id or alias. Aliases are normalized before execution.
- `event_kind`: optional contextual recommendation control.
- `enabled_pack_ids`: optional recommendation extension selectors.

## Selector normalization and validation

Validation is intentionally strict.

- Unknown rulesets fail explicitly.
- Unknown recommendation packs fail explicitly.
- Duplicate recommendation packs fail explicitly.
- Empty pack ids fail explicitly.
- Unsupported `event_kind` values fail explicitly.
- Invalid include dependencies fail explicitly.
- Unknown projection field paths fail explicitly.

The engine must not silently fall back from invalid selectors to default behavior.

## Canonical response model

The engine-backed day surface is `amlich_api::v2::DayBundleDto` with schema version `amlich.engine/v1`.

Top-level metadata is canonical and always explicit:

- `schema_version`
- `ruleset_id`
- `ruleset_version`
- `profile`
- `generated_at`

Always-on base fields:

- `solar`
- `lunar`
- `jd`

Optional sections are gated by includes:

- `canchi`
- `tiet_khi`
- `gio_hoang_dao`
- `day_fortune`
- `daily_recommendations`
- `contextual_recommendations`
- `insight`

`daily_recommendations` and `contextual_recommendations` are only exposed when the `fortune` include is active.

## Include and projection rules

Default engine includes for `day` are:

- `base`
- `canchi`
- `tiet-khi`
- `hours`
- `fortune`

Rules:

- Omitted sections are absent from the serialized response.
- Field projection operates against the gated response shape.
- Projection cannot leak omitted sections.
- `include=evidence` requires `include=fortune`.

This keeps the response contract capability-driven rather than a flat always-on DTO.

## Range model

`amlich_api::v2::DayRangeDto` extends the same engine contract to multi-day execution.

- The range envelope carries canonical metadata.
- `days` contains one `DayBundleDto` per day.
- Results are ordered ascending by date.
- Results are inclusive of `start` and `end`.
- Selector state is propagated to each day row.

Overlapping metadata between `day` and `range` must remain consistent, excluding freshness-only timestamp equality.

## Core-to-API seam mapping

The redesigned API is a thin contract layer over `amlich-core`.

### Rulesets

- Core registry source: `amlich_core::almanac::data::ruleset_registry()`
- Selector resolution: `amlich_core::almanac::data::get_ruleset()`
- API discovery surface: `amlich_api::get_ruleset_catalog()`

### Recommendation packs

- Core descriptor source: `amlich_core::almanac::recommendation::pack::recommendation_pack_descriptors()`
- Runtime activation/provenance comes from synthesized recommendation output.
- API discovery surface: `amlich_api::get_recommendation_pack_catalog()`

### Day execution

- API request normalization happens in `amlich_api::get_day_info()`.
- Core execution happens through `amlich_core::get_day_info_with_recommendation_request()`.
- `v2::get_day_bundle()` shapes the gated engine response.

## Discovery surfaces

The discovery contract lets consumers discover valid selectors before executing a day query.

### Ruleset discovery

CLI:

```bash
amlich lookup rulesets --format json
```

Current response fields include:

- `id`
- `version`
- `region`
- `profile`
- `schema_version`
- `is_default`
- `aliases`
- `defaults`
- `source_notes`

### Recommendation pack discovery

CLI:

```bash
amlich lookup recommendation-packs --format json
```

Current response fields include:

- `pack_id`
- `version`
- `source_family`
- `mode`

## Discovery-to-execution round trip

Consumers should be able to discover selectors and then feed them directly into engine execution.

Example:

```bash
amlich lookup rulesets --format json
amlich lookup recommendation-packs --format json
amlich day 2026-02-20 --format json \
  --ruleset-id baseline \
  --event-kind contract_signing \
  --recommendation-packs pack.nhi_thap_bat_tu.v1
```

Round-trip expectations:

- discovered ruleset alias `baseline` resolves to canonical `vn_baseline_v1`
- runtime `ruleset_id` matches the discovered canonical id
- runtime contextual output exposes `active_packs`
- active pack metadata matches discovery metadata

## Baseline vs contextual recommendation semantics

The redesign separates baseline and contextual recommendation output.

- `daily_recommendations` represents the baseline day result.
- `contextual_recommendations` is absent unless contextual controls are supplied.
- Baseline output must remain stable when contextual controls are added.
- Contextual output must carry visible provenance for activated controls.

This prevents recommendation controls from mutating the canonical baseline branch in-place.

## Extension safety and conflict semantics

Extension behavior is constrained by safety-first rules.

- Invalid selectors are rejected, not ignored.
- Pack activation is explicit and attributable.
- Contextual output is additive and separated from baseline output.
- Safety-sensitive guidance remains governed by the synthesized policy layer rather than by pack presence alone.

### Conflict policy

Current policy:

- duplicate extension ids are rejected
- unknown extension ids are rejected
- unsupported extension ids are rejected
- validated extension layers may influence contextual bucket outcomes, but only on the contextual branch

Future extension families should follow the same model: explicit activation, explicit provenance, and deterministic conflict handling.

## Metadata and versioning policy

There are two independent version concepts.

### Engine response schema version

- `schema_version` on day/range responses identifies the contract shape.
- Current value: `amlich.engine/v1`

### Ruleset and extension versioning

- rulesets expose `id`, `version`, and descriptor metadata
- recommendation packs expose `pack_id`, `version`, `source_family`, and `mode`
- runtime metadata must remain comparable to discovery metadata

Consumers should treat schema versioning and ruleset/pack versioning as separate compatibility axes.

## CLI consumer guidance

Preferred engine-backed surfaces:

- `amlich day`
- `amlich range`
- `amlich lookup rulesets`
- `amlich lookup recommendation-packs`

Engine selector flags on `day` and `range`:

- `--ruleset-id`
- `--event-kind`
- `--recommendation-packs`
- `--include`
- `--fields`

CLI validation is the intended user-facing boundary for invalid selector values and invalid include/projection combinations.

## TUI consumer guidance

`amlich-tui` is migrated to consume the redesigned engine-backed API shape rather than depending on the older nested metadata assumptions.

The TUI is an implementation consumer of the engine contract, but this mission does not make TUI itself the primary user-testing acceptance surface.

## Legacy surface mapping and migration boundaries

Legacy entrypoints such as `amlich query` and headless aliases remain available for compatibility, but they are not the canonical engine surface.

Boundary rules:

- overlapping calendar facts must remain aligned with `amlich day`
- deprecation or migration messaging must be present where applicable
- engine-only selector workflows are centered on `day`/`range`, not legacy wrappers

This allows gradual migration without treating legacy paths as the contract-authoritative surface.

## Deferred future hooks

The redesigned contract is shaped to grow into additional engine capabilities without another major rewrite.

Deferred but anticipated areas include:

- richer ruleset families beyond the baseline registry
- additional recommendation pack families
- broader discovery metadata for future consumer UX
- hour-pillar/profile/dai-van-oriented request expansion where a canonical surface becomes necessary

Those additions should preserve the same principles:

- discover before execute
- canonical metadata at the top level
- explicit validation
- explicit provenance
- baseline/contextual separation where overlays are introduced

## Worked examples

### Example: baseline day request

```bash
amlich day 2026-02-20 --format json
```

Expected properties:

- top-level engine metadata present
- default sections present
- no `contextual_recommendations`

### Example: strict invalid selector failure

```bash
amlich day 2026-02-20 --format json --ruleset-id nope
```

Expected behavior:

- non-zero exit
- deterministic error such as `unknown almanac ruleset id: nope`

### Example: contextual overlay request

```bash
amlich day 2026-02-20 --format json \
  --ruleset-id baseline \
  --event-kind contract_signing \
  --recommendation-packs pack.nhi_thap_bat_tu.v1
```

Expected properties:

- canonical `ruleset_id` in top-level metadata
- baseline `daily_recommendations` remains present
- `contextual_recommendations` is present
- `contextual_recommendations.active_packs` includes the selected pack

## Decision summary

The canonical contract for the redesign is the engine-backed `day`/`range` surface with explicit metadata, strict selector validation, discovery-first selector workflows, and separated baseline/contextual recommendation branches. Legacy paths remain bounded compatibility surfaces, not the source of truth for future contract growth.
