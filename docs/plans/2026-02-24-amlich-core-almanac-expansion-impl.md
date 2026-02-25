# Amlich Core Almanac Expansion Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Expand `amlich-core` and TUI day view with traditional almanac metadata (mệnh ngày, tuổi xung, giờ hoàng đạo detail, thần hướng, cát tinh/sát tinh, sao, xuất hành hướng) without regressing current lunar calendar correctness.

**Architecture:** Keep the current Rust calendar engine for solar/lunar, Can Chi, Tiết Khí, and Giờ Hoàng Đạo. Add a new `almanac` domain module and data pack for rule-driven fields, then expose it through `amlich-api` DTOs and render in TUI info panel. Use a profile-driven approach so we can support baseline rule sets now and Vietnamese overrides later, instead of replacing the whole engine.

**Tech Stack:** Rust (`amlich-core`, `amlich-api`, `amlich`), JSON data tables, Ratatui TUI, serde/serde_json, existing API parity tests.

---

## Coverage Audit (Current State)

Already covered and should be preserved:
- Solar/Lunar conversion and JD flow: `crates/amlich-core/src/lunar.rs`, `crates/amlich-core/src/julian.rs`
- Can Chi + con giáp + ngũ hành: `crates/amlich-core/src/canchi.rs`, `crates/amlich-core/src/types.rs`
- Tiết Khí: `crates/amlich-core/src/tietkhi.rs`
- Giờ Hoàng Đạo and giờ-star mapping: `crates/amlich-core/src/gio_hoang_dao.rs`

Not covered yet:
- Tuổi xung (day conflict zodiacs)
- Xuất hành hướng + thần hướng (Tài thần/Hỷ thần/etc.)
- Cát tinh/sát tinh (day-level stars)
- Sao ngày (must specify exact star system)
- Mệnh ngày (must specify whether this means Nạp âm)

Conclusion:
- **Do not switch core engine** for date math.
- **Merge/extend** with a new almanac module + rule tables.

## Decision: Switch vs Merge

Option A: Full switch to external engine logic
- Pros: Faster initial feature coverage if wrapped directly
- Cons: High regression risk for existing contracts; harder Rust-first maintenance; more moving parts

Option B: Merge rule packs into current Rust core (recommended)
- Pros: Preserves validated existing behavior; incremental rollout; testable per field; no runtime dependency change
- Cons: Requires careful modeling and migration of rule tables

Option C: Dual-engine with runtime fallback
- Pros: Safe migration path
- Cons: Long-term complexity and drift risk; expensive to maintain parity

Recommended decision:
- **Choose Option B**.
- Add optional compile-time profile support for future variants (baseline + VN overrides), without dual-runtime execution.

### Task 1: Define Canonical Almanac Domain and Glossary

**Files:**
- Create: `crates/amlich-core/src/almanac/types.rs`
- Create: `crates/amlich-core/src/almanac/profile.rs`
- Modify: `crates/amlich-core/src/lib.rs`
- Test: `crates/amlich-core/src/almanac/types.rs` (unit tests)

**Step 1: Write failing type-level tests for serialization and field presence**

Add tests that fail until all required structs exist:
- `DayFortune`
- `DayConflict` (tuổi xung, sát hướng)
- `TravelDirection` (xuất hành hướng, thần hướng)
- `DayStars` (cát tinh, sát tinh, sao)
- `DayElement` (mệnh ngày, define as `na_am` if accepted)

**Step 2: Run test to verify it fails**

Run: `cargo test -p amlich-core almanac::types`
Expected: FAIL due to missing module/types.

**Step 3: Implement minimal domain structs and profile enum**

Create typed structures with explicit optional fields for unresolved terms:
- `star_system: Option<StarSystem>`

**Step 4: Run test to verify it passes**

Run: `cargo test -p amlich-core almanac::types`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/amlich-core/src/almanac/types.rs crates/amlich-core/src/almanac/profile.rs crates/amlich-core/src/lib.rs
git commit -m "feat(core): add almanac domain model and profile scaffolding"
```

### Task 2: Add Almanac Data Pack and Loader

**Files:**
- Create: `crates/amlich-core/data/almanac/baseline.json`
- Create: `crates/amlich-core/src/almanac/data.rs`
- Modify: `crates/amlich-core/src/almanac/mod.rs`
- Test: `crates/amlich-core/src/almanac/data.rs`

**Step 1: Write failing tests for data integrity**

Test that loader validates:
- 10 Can keys
- 12 Chi keys
- 60 CanChi cycle entries where applicable
- direction values constrained to known compass tokens

**Step 2: Run test to verify it fails**

Run: `cargo test -p amlich-core almanac::data`
Expected: FAIL until file/loader exists.

**Step 3: Implement loader and schema checks**

Load JSON with `include_str!`, validate at startup via `OnceLock`, return typed maps.

**Step 4: Run test to verify it passes**

Run: `cargo test -p amlich-core almanac::data`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/amlich-core/data/almanac/baseline.json crates/amlich-core/src/almanac/data.rs crates/amlich-core/src/almanac/mod.rs
git commit -m "feat(core): add almanac baseline dataset and validated loader"
```

### Task 3: Implement Day Fortune Calculator (Rule-Driven)

**Files:**
- Create: `crates/amlich-core/src/almanac/calc.rs`
- Modify: `crates/amlich-core/src/lib.rs`
- Test: `crates/amlich-core/src/almanac/calc.rs`

**Step 1: Write failing behavior tests from fixed known dates**

Test outputs for selected dates:
- tuổi xung
- xuất hành hướng
- thần hướng (at least tài thần/hỷ thần)
- cát tinh/sát tinh lists non-empty for supported profile

**Step 2: Run test to verify it fails**

Run: `cargo test -p amlich-core almanac::calc`
Expected: FAIL.

**Step 3: Implement minimal calculator**

Inputs:
- `DayInfo.canchi.day` (can/chi index)
- profile data maps

Output:
- `DayFortune` with deterministic rule lookup.

**Step 4: Run test to verify it passes**

Run: `cargo test -p amlich-core almanac::calc`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/amlich-core/src/almanac/calc.rs crates/amlich-core/src/lib.rs
git commit -m "feat(core): compute day fortune metadata from almanac profile"
```

### Task 4: Extend DayInfo API/DTO Contracts

**Files:**
- Modify: `crates/amlich-api/src/dto.rs`
- Modify: `crates/amlich-api/src/convert.rs`
- Modify: `crates/amlich-api/src/lib.rs`
- Modify: `apps/desktop/src-tauri/src/lib.rs` (if command payload reused by TUI/desktop)
- Test: `crates/amlich-api/tests/golden_parity.rs`
- Test: `crates/amlich-api/tests/insight_parity.rs`
- Create: `crates/amlich-api/tests/almanac_contract.rs`

**Step 1: Write failing contract tests for new fields**

Add tests asserting presence/shape of:
- `day_fortune`

And verify existing fields are unchanged.

**Step 2: Run test to verify it fails**

Run: `cargo test -p amlich-api`
Expected: FAIL on missing DTO fields.

**Step 3: Implement DTO additions and converters**

Keep new fields optional at first to allow phased UI rollout.

**Step 4: Run test to verify it passes**

Run: `cargo test -p amlich-api`
Expected: PASS including existing golden fixtures.

**Step 5: Commit**

```bash
git add crates/amlich-api/src/dto.rs crates/amlich-api/src/convert.rs crates/amlich-api/src/lib.rs crates/amlich-api/tests
git commit -m "feat(api): expose day fortune in day info contracts"
```

### Task 5: Render Expanded Day View in TUI

**Files:**
- Modify: `crates/amlich/src/widgets/info_panel.rs`
- Modify: `crates/amlich/src/widgets/detail.rs` (if still used)
- Modify: `crates/amlich/src/app.rs` (state wiring if needed)
- Test: manual + snapshot-like checks via `cargo test -p amlich`

**Step 1: Write failing UI-level tests or render assertions where feasible**

At minimum, add formatter tests to verify line-building includes:
- `ngày <canchi> (<con giáp>)`
- `tháng <canchi>`
- `mệnh ngày`
- `tuổi xung`
- `tiết khí`
- `sao`
- `xuất hành hướng`
- `kỷ/hỷ/tài thần` (or agreed glossary)
- `cát tinh/sát tinh`

**Step 2: Run test to verify it fails**

Run: `cargo test -p amlich`
Expected: FAIL on missing render output.

**Step 3: Implement compact sectioned rendering**

Order sections by priority:
1. Can Chi + mệnh
2. Giờ hoàng đạo + tuổi xung
3. Thần hướng + xuất hành
4. Cát tinh/sát tinh + sao
5. Tiết khí

**Step 4: Run test to verify it passes**

Run: `cargo test -p amlich`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/amlich/src/widgets/info_panel.rs crates/amlich/src/widgets/detail.rs crates/amlich/src/app.rs
git commit -m "feat(tui): enrich day view with full almanac metadata"
```

### Task 6: Verification, Compatibility, and Documentation

**Files:**
- Modify: `README.md`
- Modify: `apps/desktop/README.md` (if API contract shared)
- Create: `docs/plans/2026-02-24-amlich-core-almanac-expansion-design.md` (if final design freeze needed)

**Step 1: Run full verification suite**

Run:
- `cargo test -p amlich-core`
- `cargo test -p amlich-api`
- `cargo test -p amlich`

Expected: all PASS.

**Step 2: Add regression notes**

Document:
- glossary decisions (`mệnh ngày`, `sao`, `kỷ thần`)
- profile source and versioning strategy
- known limitations

**Step 3: Commit**

```bash
git add README.md apps/desktop/README.md docs/plans
git commit -m "docs: document almanac model, glossary, and verification baseline"
```

## Risk Controls

- Keep existing `DayInfo` fields stable; add new optional sections first.
- Use profile ID in outputs to prevent silent rule-set drift.
- Require golden fixtures for at least 12 canonical dates across seasons.
- Guard unresolved terminology with explicit optional fields instead of implicit assumptions.

## Recommendation Summary

- Keep current core calendar math.
- Merge a new almanac rule profile into Rust core.
- Expose everything through API with optional fields, then harden once glossary is finalized.
