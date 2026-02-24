# Weighted Day Guidance Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Replace static `chi -> good/avoid` lookup with a configurable weighted scoring engine that outputs `Nên làm / Không nên làm` per date and remains explainable.

**Architecture:** Add a new scoring module in `amlich-core` that loads rule config from JSON, computes activity scores from existing day signals (`can-chi`, `gio_hoang_dao`, `tiet_khi`, holiday/festival context), and maps ranked activities into `good_for`/`avoid_for`. Wire it through `amlich-api` so `get_day_insight` uses scored output instead of static day-guidance text. Keep DTO compatibility for desktop by preserving existing `day_guidance.good_for` and `day_guidance.avoid_for`.

**Tech Stack:** Rust (`serde`, `serde_json`), existing `amlich-core` day calculation APIs, `amlich-api` DTO conversion/tests, Cargo test runner.

---

### Task 1: Baseline and Contract Tests

**Files:**
- Modify: `crates/amlich-api/tests/insight_parity.rs`
- Create: `crates/amlich-api/tests/weighted_guidance_contract.rs`

**Step 1: Write failing contract tests for weighted guidance shape**

```rust
#[test]
fn weighted_guidance_returns_ranked_lists() {
    let insight = get_day_insight(&DateQuery { day: 10, month: 2, year: 2024, timezone: None })
        .expect("day insight should work");
    let g = insight.day_guidance.expect("guidance should exist");
    assert!(!g.good_for.vi.is_empty());
    assert!(!g.avoid_for.vi.is_empty());
}
```

**Step 2: Add determinism test for stable ranking**

```rust
#[test]
fn weighted_guidance_is_deterministic() {
    let a = get_day_insight(&DateQuery { day: 29, month: 1, year: 2025, timezone: None }).unwrap();
    let b = get_day_insight(&DateQuery { day: 29, month: 1, year: 2025, timezone: None }).unwrap();
    assert_eq!(a.day_guidance.unwrap().good_for.vi, b.day_guidance.unwrap().good_for.vi);
}
```

**Step 3: Run tests to verify new tests fail before implementation**

Run: `cargo test -p amlich-api --test weighted_guidance_contract -q`  
Expected: FAIL with missing weighted guidance behavior.

**Step 4: Run parity tests to capture current behavior snapshot**

Run: `cargo test -p amlich-api --test insight_parity -q`  
Expected: PASS (baseline unchanged before refactor).

**Step 5: Commit**

```bash
git add crates/amlich-api/tests/insight_parity.rs crates/amlich-api/tests/weighted_guidance_contract.rs
git commit -m "test: add weighted guidance contract coverage"
```

### Task 2: Introduce Weighted Rules Data Model

**Files:**
- Create: `crates/amlich-core/data/activity-rules.v1.json`
- Create: `crates/amlich-core/src/weighted_guidance.rs`
- Modify: `crates/amlich-core/src/lib.rs`

**Step 1: Write failing parser test for rules file**

```rust
#[test]
fn parses_activity_rules_file() {
    let rules = load_activity_rules().expect("rules should parse");
    assert!(!rules.activities.is_empty());
    assert!(!rules.weights_by_activity.is_empty());
}
```

**Step 2: Define serde structs for rules schema**

```rust
#[derive(Debug, Deserialize)]
pub struct ActivityRulesV1 {
    pub version: u32,
    pub activities: Vec<String>,
    pub factors: Vec<String>,
    pub weights_by_activity: HashMap<String, HashMap<String, f64>>,
}
```

**Step 3: Add static JSON include and loader**

Run: Implement `include_str!("../data/activity-rules.v1.json")` and `load_activity_rules()`.

**Step 4: Export module from core crate**

Run: add `pub mod weighted_guidance;` in `crates/amlich-core/src/lib.rs`.

**Step 5: Run tests and verify parser passes**

Run: `cargo test -p amlich-core weighted_guidance::tests::parses_activity_rules_file -q`  
Expected: PASS.

**Step 6: Commit**

```bash
git add crates/amlich-core/data/activity-rules.v1.json crates/amlich-core/src/weighted_guidance.rs crates/amlich-core/src/lib.rs
git commit -m "feat(core): add weighted guidance rules schema and loader"
```

### Task 3: Implement Factor Extraction and Scoring Engine

**Files:**
- Modify: `crates/amlich-core/src/weighted_guidance.rs`
- Modify: `crates/amlich-core/src/insight_data.rs` (only if small helper reuse is needed)

**Step 1: Write failing test for score calculation range**

```rust
#[test]
fn activity_scores_are_normalized() {
    let scores = score_activities_for_day(&sample_day_context()).unwrap();
    for (_, score) in scores {
        assert!((-100.0..=100.0).contains(&score));
    }
}
```

**Step 2: Implement day context builder from existing signals**

Run: derive factor values from:
- day `chi` (`day_info.canchi.day.chi`)
- `gio_hoang_dao.good_hour_count` ratio
- `tiet_khi` season/name
- holiday/festival presence

**Step 3: Implement weighted aggregation**

```rust
final_score = sum(weight(activity, factor) * normalized_value(factor));
```

**Step 4: Add hard-block support from config**

Run: if hard-block condition matches date/activity, clamp score (example `-80.0`).

**Step 5: Run focused tests**

Run: `cargo test -p amlich-core weighted_guidance::tests -q`  
Expected: PASS.

**Step 6: Commit**

```bash
git add crates/amlich-core/src/weighted_guidance.rs crates/amlich-core/src/insight_data.rs
git commit -m "feat(core): implement weighted activity scoring engine"
```

### Task 4: Map Scores to Day Guidance Output

**Files:**
- Modify: `crates/amlich-api/src/lib.rs`
- Modify: `crates/amlich-api/src/dto.rs`
- Modify: `crates/amlich-api/src/convert.rs`

**Step 1: Write failing API test for top-N do/avoid mapping**

```rust
#[test]
fn day_guidance_comes_from_weighted_ranking() {
    let insight = get_day_insight(&DateQuery { day: 5, month: 4, year: 2024, timezone: None }).unwrap();
    let g = insight.day_guidance.unwrap();
    assert!(g.good_for.vi.len() <= 3);
    assert!(g.avoid_for.vi.len() <= 3);
}
```

**Step 2: Replace static `get_day_guidance(...)` call**

Run: in `get_day_insight`, compute weighted scores from day context and map top positive/negative activities to localized labels.

**Step 3: Keep backward-compatible DTO fields**

Run: preserve:
- `day_guidance.good_for`
- `day_guidance.avoid_for`

Optional for future: add reason metadata as optional field only.

**Step 4: Convert activity keys to bilingual labels**

Run: add a stable label map (`vi`/`en`) in core or API layer.

**Step 5: Run API tests**

Run: `cargo test -p amlich-api --test insight_parity -q`  
Expected: PASS (or updated assertions to new deterministic output).

**Step 6: Commit**

```bash
git add crates/amlich-api/src/lib.rs crates/amlich-api/src/dto.rs crates/amlich-api/src/convert.rs crates/amlich-api/tests/weighted_guidance_contract.rs
git commit -m "feat(api): serve weighted do/avoid guidance"
```

### Task 5: Desktop Validation and UI Contract Check

**Files:**
- Modify: `apps/desktop/src/lib/insights/types/insight-dto.ts` (only if DTO changed)
- Modify: `apps/desktop/src/lib/components/DateInsightBox.svelte` (only if rendering behavior changes)

**Step 1: Write/adjust TS type expectations**

Run: keep `day_guidance.good_for` and `day_guidance.avoid_for` shape unchanged unless optional reason fields are added.

**Step 2: Verify render behavior with weighted output**

Run: ensure top 3 `good_for` and top 3 `avoid_for` still render and language toggle still works.

**Step 3: Run desktop type/build check**

Run: `cd apps/desktop && pnpm -s check`  
Expected: PASS.

**Step 4: Manual sanity check**

Run app and inspect 3 known dates (e.g. Tet, random weekday, major solar holiday) for plausible do/avoid differences.

**Step 5: Commit**

```bash
git add apps/desktop/src/lib/insights/types/insight-dto.ts apps/desktop/src/lib/components/DateInsightBox.svelte
git commit -m "chore(desktop): validate weighted guidance contract"
```

### Task 6: Regression Suite and Documentation

**Files:**
- Modify: `crates/amlich-api/tests/insight_parity.rs`
- Modify: `docs/plans/2026-02-24-weighted-guidance-implementation.md` (checklist updates only)
- Optional Create: `docs/weighted-guidance.md`

**Step 1: Add regression tests for edge dates**

Run: test at least:
- festival day
- non-festival weekday
- date with no holiday

**Step 2: Run targeted test matrix**

Run:
- `cargo test -p amlich-core -q`
- `cargo test -p amlich-api -q`

Expected: PASS.

**Step 3: Run workspace smoke test**

Run: `cargo test -q`  
Expected: PASS or document unrelated failures.

**Step 4: Document tuning knobs**

Run: describe how to edit `activity-rules.v1.json` (weights, thresholds, hard-blocks).

**Step 5: Commit**

```bash
git add crates/amlich-api/tests/insight_parity.rs docs/weighted-guidance.md crates/amlich-core/data/activity-rules.v1.json
git commit -m "test/docs: add weighted guidance regression coverage and tuning guide"
```

### Task 7: Rollout Guardrails

**Files:**
- Modify: `crates/amlich-api/src/lib.rs`
- Optional Create: `crates/amlich-api/tests/weighted_guidance_rollout.rs`

**Step 1: Add feature flag fallback**

Run: add env/config gate:
- default ON for local development
- can fallback to legacy static guidance if needed

**Step 2: Write failing fallback test**

```rust
#[test]
fn can_fallback_to_legacy_guidance() {
    // set flag off, assert legacy source path still returns guidance
}
```

**Step 3: Implement fallback path**

Run: if flag disabled, call legacy static guidance lookup.

**Step 4: Run rollout tests**

Run: `cargo test -p amlich-api weighted_guidance_rollout -q`  
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/amlich-api/src/lib.rs crates/amlich-api/tests/weighted_guidance_rollout.rs
git commit -m "feat(api): add weighted guidance rollout fallback"
```

## Final Verification Checklist

- `cargo test -p amlich-core -q`
- `cargo test -p amlich-api -q`
- `cargo test -q`
- `cd apps/desktop && pnpm -s check`

Expected outcome:
- `day_guidance` is generated by configurable weighted rules.
- Desktop UI renders without contract changes.
- Behavior is deterministic and test-covered.
