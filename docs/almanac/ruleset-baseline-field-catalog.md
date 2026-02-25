# Baseline Almanac Dataset Field Catalog (R-1001)

## Purpose

Catalog the current `baseline.json` dataset and `almanac::data` loader responsibilities before refactoring to a multi-ruleset registry.

Primary references:

- `crates/amlich-core/data/almanac/baseline.json`
- `crates/amlich-core/src/almanac/data.rs`

## Current Dataset Inventory (`baseline.json`)

### Top-level keys (complete)

1. `profile` (`string`)
   - Current value: `"baseline"`
   - Loader use: copied into `AlmanacData.profile`, then propagated into evidence/profile outputs.

2. `travel_meta` (`SourceMeta`)
   - Shape: `{ source_id, method }`
   - Provenance for `travel_by_can` / thần hướng + xuất hành direction rules.

3. `conflict_meta` (`SourceMeta`)
   - Shape: `{ source_id, method }`
   - Provenance for `conflict_by_chi` / tuổi xung + sát hướng rules.

4. `na_am_meta` (`SourceMeta`)
   - Shape: `{ source_id, method }`
   - Provenance for `na_am_pairs` -> expanded nạp âm cycle map.

5. `star_meta` (`SourceMeta`)
   - Shape: `{ source_id, method }`
   - Provenance for JD-cycle based `nhi_thap_bat_tu` (nhị thập bát tú day star).

6. `star_rule_meta` (`StarRuleMetaSet`)
   - Shape keys:
     - `fixed_by_chi`
     - `fixed_by_canchi`
     - `by_year`
     - `by_month`
     - `by_tiet_khi`
   - Each value is `SourceMeta`.
   - Used to annotate evidence for merged day-level cát tinh/sát tinh rule families.

7. `star_rule_sets` (`StarRuleSetsRaw`)
   - Current shape keys (implemented in loader):
     - `fixed_by_canchi`
     - `by_year_can`
     - `by_lunar_month`
     - `by_tiet_khi`
   - Value type per key: `StarRuleBucketRaw` (`cat_tinh[]`, `sat_tinh[]`, optional `binh_tinh[]`).
   - Note: `fixed_by_chi` is *not* stored here today; fixed-by-chi stars are currently derived from `conflict_by_chi` lists.

8. `travel_by_can` (`map<string, TravelRule>`)
   - Keyed by 10 thiên can names.
   - `TravelRule` fields:
     - `xuat_hanh_huong`
     - `tai_than`
     - `hy_than`
     - `ky_than` (nullable)

9. `conflict_by_chi` (`map<string, ConflictRule>`)
   - Keyed by 12 địa chi names.
   - `ConflictRule` fields:
     - `opposing_chi`
     - `sat_huong`
     - `cat_tinh[]`
     - `sat_tinh[]`
   - Currently serves two roles:
     - conflict/tuổi xung data
     - base `fixed_by_chi` star rule source for `than_sat.rs`

10. `na_am_pairs` (`string[30]`)
   - 30 paired nạp âm labels, expanded by loader into 60 can-chi entries.

11. `nhi_thap_bat_tu` (`DayStarRule[28]`)
   - Each entry: `{ name, quality }`
   - Indexed by `jd % 28` in current calculator.

## Loader Output Model (`AlmanacData`)

The loader converts the JSON into a typed in-memory struct with normalized maps:

- metadata passthrough:
  - `profile`
  - `travel_meta`, `conflict_meta`, `na_am_meta`, `star_meta`
  - `star_rule_meta`
- direct maps:
  - `travel_by_can`
  - `conflict_by_chi`
  - `nhi_thap_bat_tu`
- derived/normalized maps:
  - `sexagenary_na_am` (`HashMap<String, NaAmEntry>`) expanded from `na_am_pairs`
  - `star_rules_fixed_by_canchi` (`HashMap<String, StarRuleBucket>`)
  - `star_rules_by_year_can` (`HashMap<String, StarRuleBucket>`)
  - `star_rules_by_lunar_month` (`HashMap<u8, StarRuleBucket>`) parsed from string keys
  - `star_rules_by_tiet_khi` (`HashMap<String, StarRuleBucket>`)

## Loader Responsibilities and Validations (`almanac::data`)

### Loading behavior

- Static embedding via `include_str!("../../data/almanac/baseline.json")`
- Single initialization via `OnceLock<AlmanacData>`
- Parse JSON into `RawAlmanacData`, then validate, then normalize

### Validation responsibilities (current)

1. Source metadata validation
   - `source_id` must be non-empty for all `SourceMeta`
   - `method` must be one of allowed tokens: `table-lookup`, `bai-quyet`, `jd-cycle`

2. Shape/completeness checks for core rule maps
   - `travel_by_can` must contain exactly 10 valid can keys
   - `conflict_by_chi` must contain exactly 12 valid chi keys

3. Direction token validation
   - All travel directions and conflict `sat_huong` must match the allowed 8 compass labels

4. Conflict star list validation
   - Each `conflict_by_chi[*]` must have non-empty `cat_tinh` and `sat_tinh`

5. Nạp âm cycle validation
   - `na_am_pairs` must contain exactly 30 non-empty strings
   - Loader expands 30 pairs into 60 can-chi rows deterministically

6. Nhị thập bát tú validation
   - `nhi_thap_bat_tu` must contain exactly 28 entries
   - Each `name` non-empty
   - `quality` constrained to `cat | hung | binh`

7. Star-rule metadata validation
   - Each `star_rule_meta.*` entry validated as `SourceMeta`

8. Star-rule set validation
   - `fixed_by_canchi` keys must be valid sexagenary names (60-cycle tokens)
   - `by_year_can` keys must be valid can names
   - `by_lunar_month` keys must be numeric and in `1..=12`
   - `by_tiet_khi` keys must exist in the known `TIET_KHI` table
   - Every bucket validates:
     - no empty star names in `cat_tinh/sat_tinh/binh_tinh`
     - no duplicate star names across categories within the same bucket

### Normalization responsibilities

- `StarRuleBucketRaw` -> `StarRuleBucket` (ensures optional `binh_tinh` becomes concrete vec)
- Parse month map keys from `String` to `u8`
- Expand `na_am_pairs` into `sexagenary_na_am` with cached `can`, `chi`, `na_am`, `element`

## Current Couplings / Schema Risks (important for multi-ruleset)

1. `profile` is a free string and doubles as provenance label
   - Risk: not enough structure for multi-ruleset identity/version requirements.
   - Future need: explicit `ruleset_id`, `ruleset_version`, `region`, schema version.

2. Baseline loader is singleton and hard-coded
   - `baseline_data()` always returns one embedded dataset.
   - Risk: no registry, no lookup path, no explicit unknown-ruleset errors.

3. `conflict_by_chi` is overloaded (conflict + fixed-by-chi stars)
   - This mixes two rule families in one table.
   - Risk: schema evolution for conflict logic can unintentionally affect star rules (and vice versa).
   - Strong extension point: split fixed-by-chi star rules into `star_rule_sets.fixed_by_chi` (or separate family table).

4. `star_rule_meta` and `star_rule_sets` naming mismatch
   - Meta uses `by_year` / `by_month`; data uses `by_year_can` / `by_lunar_month`.
   - Risk: confusion when adding validators, docs, or alternate packs.
   - Recommendation: unify names in ruleset schema and keep aliases only for migration.

5. Validation uses `assert!`/`expect!` (panic path)
   - Fine for internal embedded baseline, but weak for a registry API.
   - Future need: typed validation/load errors with field path context for unknown or invalid packs.

6. Method token validation is global and closed
   - Current allowed methods are hard-coded.
   - Risk: adding new rule families may require new methods (`formula`, `calendar-anchor`, etc.) and force code changes.
   - Future need: centralized method-token schema/version policy and explicit extensibility.

7. No ruleset-level defaults/config metadata
   - Missing pack-level defaults like timezone/meridian/schema version.
   - This conflicts with the v1 ruleset contract requirements in `docs/almanac/ruleset-v1-scope.md`.

8. No per-family enablement/feature flags
   - Loader assumes all current families exist and are valid.
   - Multi-ruleset support may need optional families with explicit enablement/absence semantics.

## Extension Points for Multi-Ruleset Registry (Phase 1+)

### A. Add a ruleset descriptor wrapper around current payload

Recommended outer schema (minimal transition-friendly):

- `identity`: `{ id, version, region }`
- `defaults`: `{ tz_offset, meridian? }`
- `schema`: `{ version }`
- `families`: existing almanac payload (or family subtrees)

This preserves current family tables while making ruleset identity/version explicit.

### B. Split rule families by responsibility

Current baseline can be reorganized into distinct families:

- `travel` (currently `travel_meta`, `travel_by_can`)
- `conflict` (currently `conflict_meta`, conflict-only fields)
- `star_rules_fixed_by_chi` (currently embedded in `conflict_by_chi` star lists)
- `star_rules_contextual` (current `star_rule_sets`)
- `na_am` (`na_am_meta`, `na_am_pairs`)
- `day_star_jd_cycle` (`star_meta`, `nhi_thap_bat_tu`)

This reduces cross-family coupling and makes per-family provenance clearer.

### C. Replace `baseline_data()` with registry lookup

Transition path:

- Keep `baseline_data()` as a compatibility shim to `get_ruleset("vn_baseline_v1")`
- Introduce registry map keyed by ruleset id
- Return typed `Result` errors for unknown ids / validation failures

### D. Make validator entry points reusable per ruleset pack

Extract current validators into reusable functions that accept:

- ruleset descriptor + family payload
- error accumulator / typed error builder
- schema version / token policy

This supports `I-1003` negative fixture tests and better diagnostics.

### E. Add explicit provenance fields to outputs

Current `profile` output is insufficient for versioned behavior.

Planned output additions (per `ruleset-v1-scope.md` / master plan):

- `ruleset_id`
- `ruleset_version`
- existing `profile` (optional alias during migration)

## Suggested Migration Notes for Next Issues

- `I-1002` can start by wrapping the current baseline payload unchanged and registering it as `vn_baseline_v1`.
- `I-1003` should preserve current token checks, but convert panic-based validation into typed errors.
- `I-1004` should thread `ruleset_id` through core compute APIs while defaulting to the registered baseline.
