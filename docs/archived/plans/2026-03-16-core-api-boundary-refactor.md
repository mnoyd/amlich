# Core/API Boundary Refactor Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Move aggregate day assembly and DTO-shaped outputs out of `amlich-core` so `amlich-core` exposes only domain/functional primitives and `amlich-api` becomes the sole orchestration + DTO layer.

**Architecture:** Introduce explicit domain primitives in `amlich-core` for calendar context, almanac evaluation, and recommendation synthesis inputs. Migrate `amlich-api` to assemble `DayInfoDto` and related outputs directly from those primitives, then update downstream crates/tests to stop depending on removed aggregate helpers from core. Remove presentation-oriented fields and transport-shaped aggregate APIs from `amlich-core` once all consumers are migrated.

**Tech Stack:** Rust workspace (`amlich-core`, `amlich-api`, `amlich`, `amlich-wasm`), serde/serde_json, existing golden/recommendation contract tests, cargo test.

---

## Task 1: Inventory current core aggregate surface and freeze migration targets

**Files:**
- Modify: `crates/amlich-core/src/lib.rs`
- Modify: `crates/amlich-api/src/lib.rs`
- Test: `crates/amlich-core/tests/ruleset_determinism.rs`
- Test: `crates/amlich-api/tests/almanac_contract.rs`

**Step 1: Write failing API-shape tests that describe the new layering**

Add or update tests to assert:
- `amlich-api` remains the public source of `DayInfoDto`
- ruleset selection is applied through API orchestration
- no new tests rely on `amlich_core::get_day_info`

**Step 2: Run targeted tests to verify current layering assumptions before refactor**

Run: `cargo test -p amlich-core ruleset_determinism --offline`
Expected: PASS on current behavior and give a baseline before edits.

Run: `cargo test -p amlich-api almanac_contract --offline`
Expected: PASS on current DTO contract.

**Step 3: Add temporary TODO-free migration notes in test names/fixtures only where needed**

Keep this step minimal: rename tests or add new tests that clearly encode the desired boundary, without changing production code yet.

**Step 4: Run the same targeted tests again**

Run: `cargo test -p amlich-api almanac_contract --offline`
Expected: PASS or clearly isolate the first intended failing assertion for the next task.

**Step 5: Commit**

```bash
git add crates/amlich-core/src/lib.rs crates/amlich-api/src/lib.rs crates/amlich-core/tests/ruleset_determinism.rs crates/amlich-api/tests/almanac_contract.rs
git commit -m "test: freeze core api boundary expectations"
```

### Task 2: Introduce pure calendar context primitives in `amlich-core`

**Files:**
- Modify: `crates/amlich-core/src/lib.rs`
- Modify: `crates/amlich-core/src/types.rs`
- Modify: `crates/amlich-core/src/lunar.rs`
- Test: `crates/amlich-core/src/lib.rs`

**Step 1: Write failing unit tests for explicit domain primitives**

Add tests for a new shape such as:
- `DayContext` (or equivalent) with solar date parts, lunar date, JD, weekday index, Can Chi, Tiết Khí, Giờ Hoàng Đạo
- `compute_day_context(day, month, year, tz)` returns the same deterministic facts that old `get_day_info` returned for those subsystems

**Step 2: Run the new unit tests to verify they fail**

Run: `cargo test -p amlich-core compute_day_context --offline`
Expected: FAIL because the new primitive API does not exist yet.

**Step 3: Implement minimal context primitive API**

Create a core-facing API that:
- does not include recommendation request DTOs
- does not include presentation strings like `date_string` or `day_of_week_name`
- computes only structured day context

Suggested public API shape:

```rust
pub struct DayContext {
    pub solar: SolarDate,
    pub lunar: LunarDate,
    pub jd: i32,
    pub weekday_index: usize,
    pub canchi: DayCanChiSet,
    pub tiet_khi: SolarTerm,
    pub gio_hoang_dao: GioHoangDao,
}

pub fn compute_day_context(day: i32, month: i32, year: i32, time_zone: f64) -> DayContext
```

Keep names aligned with existing code conventions if nearby types suggest better naming.

**Step 4: Run the new unit tests again**

Run: `cargo test -p amlich-core compute_day_context --offline`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/amlich-core/src/lib.rs crates/amlich-core/src/types.rs crates/amlich-core/src/lunar.rs
git commit -m "refactor(core): add pure day context primitives"
```

### Task 3: Make almanac evaluation consume explicit ruleset input

**Files:**
- Modify: `crates/amlich-core/src/almanac/calc.rs`
- Modify: `crates/amlich-core/src/almanac/data.rs`
- Modify: `crates/amlich-core/src/almanac/types.rs`
- Test: `crates/amlich-core/src/almanac/calc.rs`
- Test: `crates/amlich-core/tests/ruleset_determinism.rs`

**Step 1: Write failing tests proving ruleset is an explicit dependency**

Add tests that fail until:
- `calculate_day_fortune` no longer hardcodes `default_ruleset()`
- almanac evaluation accepts a ruleset/registry entry/data reference explicitly
- invalid or alternate ruleset paths are handled by the caller, not hidden inside calculation

**Step 2: Run targeted tests to verify they fail**

Run: `cargo test -p amlich-core almanac::calc --offline`
Expected: FAIL on the new explicit-ruleset expectations.

**Step 3: Implement minimal signature change**

Refactor `calculate_day_fortune` to accept either:
- `&RulesetRegistryEntry`, or
- `&AlmanacData` plus descriptor/profile metadata

Do not add transport concerns. Keep the function purely domain-facing.

**Step 4: Run targeted tests again**

Run: `cargo test -p amlich-core almanac::calc --offline`
Expected: PASS.

Run: `cargo test -p amlich-core ruleset_determinism --offline`
Expected: PASS and demonstrate deterministic behavior under explicit ruleset input.

**Step 5: Commit**

```bash
git add crates/amlich-core/src/almanac/calc.rs crates/amlich-core/src/almanac/data.rs crates/amlich-core/src/almanac/types.rs crates/amlich-core/tests/ruleset_determinism.rs
git commit -m "refactor(core): make almanac evaluation require explicit ruleset"
```

### Task 4: Isolate recommendation synthesis from removed aggregate helpers

**Files:**
- Modify: `crates/amlich-core/src/almanac/recommendation/synthesize.rs`
- Modify: `crates/amlich-core/src/almanac/recommendation/event_kind.rs`
- Modify: `crates/amlich-core/src/almanac/recommendation/packs/nhi_thap_bat_tu.rs`
- Test: `crates/amlich-core/tests/recommendation_corpus.rs`
- Test: `crates/amlich-core/tests/recommendation_safety_policy.rs`

**Step 1: Write failing tests or test helpers that stop depending on `get_day_info`**

Replace aggregate-driven setup with explicit fixture builders using:
- `compute_day_context`
- `calculate_day_fortune`
- `RecommendationSynthesisContext`

**Step 2: Run targeted tests to verify failures are isolated**

Run: `cargo test -p amlich-core recommendation_corpus --offline`
Expected: FAIL until tests and helpers are migrated.

**Step 3: Implement minimal helper migration**

Update recommendation tests and any internal helper code so recommendation synthesis depends only on domain primitives, not a removed top-level convenience aggregate.

**Step 4: Run recommendation tests again**

Run: `cargo test -p amlich-core recommendation_corpus --offline`
Expected: PASS.

Run: `cargo test -p amlich-core recommendation_safety_policy --offline`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/amlich-core/src/almanac/recommendation/synthesize.rs crates/amlich-core/src/almanac/recommendation/event_kind.rs crates/amlich-core/src/almanac/recommendation/packs/nhi_thap_bat_tu.rs crates/amlich-core/tests/recommendation_corpus.rs crates/amlich-core/tests/recommendation_safety_policy.rs
git commit -m "refactor(core): decouple recommendation tests from day aggregate"
```

### Task 5: Move day assembly to `amlich-api`

**Files:**
- Modify: `crates/amlich-api/src/lib.rs`
- Modify: `crates/amlich-api/src/convert.rs`
- Modify: `crates/amlich-api/src/dto.rs`
- Test: `crates/amlich-api/tests/almanac_contract.rs`
- Test: `crates/amlich-api/tests/recommendation_contract.rs`
- Test: `crates/amlich-api/tests/recommendation_corpus_parity.rs`

**Step 1: Write failing API tests for fully assembled DTOs from primitives**

Add/update tests so `amlich-api::get_day_info`:
- validates input/query
- chooses ruleset
- computes day context via core primitive API
- computes almanac + recommendations via core primitive API
- returns the same DTO contract as before

**Step 2: Run targeted tests to verify they fail**

Run: `cargo test -p amlich-api recommendation_contract --offline`
Expected: FAIL until API orchestration is updated.

**Step 3: Implement API-side orchestration**

Refactor `amlich-api` to:
- normalize request values
- call `amlich_core::compute_day_context`
- look up rulesets and recommendation packs at the API layer
- call explicit core evaluation functions
- build DTOs without relying on `amlich_core::DayInfo`

Add small internal API-layer structs if helpful, but do not reintroduce transport-shaped core types.

**Step 4: Run targeted API tests again**

Run: `cargo test -p amlich-api almanac_contract --offline`
Expected: PASS.

Run: `cargo test -p amlich-api recommendation_contract --offline`
Expected: PASS.

Run: `cargo test -p amlich-api recommendation_corpus_parity --offline`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/amlich-api/src/lib.rs crates/amlich-api/src/convert.rs crates/amlich-api/src/dto.rs crates/amlich-api/tests/almanac_contract.rs crates/amlich-api/tests/recommendation_contract.rs crates/amlich-api/tests/recommendation_corpus_parity.rs
git commit -m "refactor(api): assemble day dto from core primitives"
```

### Task 6: Move presentation-derived fields out of `amlich-core`

**Files:**
- Modify: `crates/amlich-core/src/lib.rs`
- Modify: `crates/amlich-api/src/convert.rs`
- Modify: `crates/amlich-api/src/dto.rs`
- Test: `crates/amlich-api/tests/na_am_api_tests.rs`
- Test: `crates/amlich/tests/cli_contract.rs`

**Step 1: Write failing tests that compute presentation strings outside core**

Cover fields such as:
- `solar.day_of_week_name`
- `solar.date_string`
- `lunar.date_string`

The tests should assert the DTO/CLI contract still exposes them, but they are derived in API/adapter code.

**Step 2: Run targeted tests to verify they fail**

Run: `cargo test -p amlich-api na_am_api_tests --offline`
Expected: FAIL until API-side formatting is implemented.

**Step 3: Implement minimal formatting migration**

Remove stored presentation strings from core-facing structs. Compute those strings in DTO conversion or small API-local helpers instead.

Do not change external serialized field names in this task.

**Step 4: Run targeted tests again**

Run: `cargo test -p amlich-api na_am_api_tests --offline`
Expected: PASS.

Run: `cargo test -p am-lich cli_contract --offline`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/amlich-core/src/lib.rs crates/amlich-api/src/convert.rs crates/amlich-api/src/dto.rs crates/amlich-api/tests/na_am_api_tests.rs crates/amlich/tests/cli_contract.rs
git commit -m "refactor(api): derive presentation strings outside core"
```

### Task 7: Remove aggregate transport APIs from `amlich-core`

**Files:**
- Modify: `crates/amlich-core/src/lib.rs`
- Modify: `crates/amlich-core/tests/almanac_golden.rs`
- Modify: `crates/amlich-core/tests/golden_dataset_coverage.rs`
- Modify: `crates/amlich-core/tests/generate_golden.rs`
- Modify: `crates/amlich-core/tests/khcbppt_*.rs`
- Modify: `crates/amlich-core/tests/taboo_boundary.rs`
- Modify: `crates/amlich-core/tests/ruleset_determinism.rs`

**Step 1: Write failing compile-time cleanup by removing public aggregate items**

Delete from public core API:
- `SolarInfo`
- `LunarInfo`
- `CanChiInfo`
- `DayInfo`
- `RecommendationRequest`
- `get_day_info`
- `get_day_info_with_timezone`
- `get_day_info_with_recommendation_request`

Let remaining compile errors reveal all lingering dependencies.

**Step 2: Run targeted core tests to collect all breakages**

Run: `cargo test -p amlich-core --offline`
Expected: FAIL with references to removed aggregate API.

**Step 3: Migrate tests to primitive builders**

Update core tests to use explicit domain setup helpers. Prefer a shared test helper module if duplication becomes painful, but only after at least two concrete repeated cases appear.

**Step 4: Run the full core test suite**

Run: `cargo test -p amlich-core --offline`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/amlich-core/src/lib.rs crates/amlich-core/tests
git commit -m "refactor(core): remove day aggregate transport api"
```

### Task 8: Update downstream runtime consumers to depend on `amlich-api`

**Files:**
- Modify: `crates/amlich/src/main.rs`
- Modify: `crates/amlich/src/headless.rs`
- Modify: `crates/amlich-wasm/src/lib.rs`
- Test: `crates/amlich/tests/cli_contract.rs`
- Test: `crates/amlich-api/tests/golden_parity.rs`

**Step 1: Write failing integration tests or compile fixes for downstream crates**

Ensure CLI/WASM consume API contracts rather than removed core aggregate helpers.

**Step 2: Run targeted tests to verify failures**

Run: `cargo test -p amlich cli_contract --offline`
Expected: FAIL until downstream code is migrated.

**Step 3: Implement minimal downstream migration**

Update imports/usages so runtime crates call `amlich-api` entrypoints for assembled day payloads.

**Step 4: Run targeted tests again**

Run: `cargo test -p am-lich cli_contract --offline`
Expected: PASS.

Run: `cargo test -p amlich-api golden_parity --offline`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/amlich/src/main.rs crates/amlich/src/headless.rs crates/amlich-wasm/src/lib.rs crates/amlich/tests/cli_contract.rs crates/amlich-api/tests/golden_parity.rs
git commit -m "refactor(app): route assembled day payloads through api layer"
```

### Task 9: Lock down internal-only data access and clean public exports

**Files:**
- Modify: `crates/amlich-core/src/lib.rs`
- Modify: `crates/amlich-core/src/almanac/mod.rs`
- Modify: `crates/amlich-core/src/almanac/data.rs`
- Modify: `crates/amlich-api/src/lib.rs`
- Test: `crates/amlich-api/tests/catalog_contract.rs`

**Step 1: Write failing tests for the final public surface**

Add coverage that the supported public entrypoints are still available where needed:
- `amlich-api` catalog functions still work
- `amlich-core` exports only domain primitives and domain types intended for consumers

**Step 2: Run targeted tests to verify assumptions**

Run: `cargo test -p amlich-api catalog_contract --offline`
Expected: PASS baseline or isolated failure after export cleanup.

**Step 3: Tighten visibility**

Make data-loader and helper modules `pub(crate)` where possible. Only keep public what is required for domain composition from API.

Avoid breaking API-layer needs accidentally; change visibility only after confirming actual imports.

**Step 4: Run targeted tests again**

Run: `cargo test -p amlich-api catalog_contract --offline`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/amlich-core/src/lib.rs crates/amlich-core/src/almanac/mod.rs crates/amlich-core/src/almanac/data.rs crates/amlich-api/src/lib.rs crates/amlich-api/tests/catalog_contract.rs
git commit -m "refactor(core): narrow public exports after api migration"
```

### Task 10: Final verification and regression sweep

## Implementation Notes

- Add this section only after the refactor is implemented and verified.
- If the final implementation removes transport-shaped aggregate APIs from `amlich-core` and moves DTO/presentation assembly into `amlich-api`, record that here as the main boundary outcome.
- If core tests are migrated away from `get_day_info`, describe the new helper strategy generically unless the helper names are introduced earlier in this plan.
- This plan previously used `cargo test -p amlich --offline` for runtime verification, but the correct package name in this workspace is `am-lich`.

**Files:**
- Modify: `docs/plans/2026-03-16-core-api-boundary-refactor.md`

**Step 1: Run focused workspace verification**

Run: `cargo test -p amlich-core --offline`
Expected: PASS.

Run: `cargo test -p amlich-api --offline`
Expected: PASS.

Run: `cargo test -p am-lich --offline`
Expected: PASS.

Run: `cargo test -p amlich-wasm --offline`
Expected: PASS or zero tests with successful compile.

**Step 2: Run broader verification if time/cost is acceptable**

Run: `cargo test --workspace --offline`
Expected: PASS.

**Step 3: Update this plan with any deviations discovered during execution**

Add a short “Implementation Notes” section only if the final code differs materially from the planned boundary.

**Step 4: Review git diff for accidental architectural regressions**

Check that:
- core no longer exposes transport aggregate APIs
- api owns DTO assembly
- ruleset is explicit in domain evaluation
- presentation strings are derived outside core

**Step 5: Commit**

```bash
git add docs/plans/2026-03-16-core-api-boundary-refactor.md
git commit -m "docs: finalize core api boundary refactor plan"
```
