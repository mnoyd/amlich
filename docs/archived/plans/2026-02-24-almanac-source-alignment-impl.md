# Almanac Source Alignment Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Align day-level almanac outputs with a source-backed Vietnamese workflow (mệnh ngày, xung/hợp, trực, sao/cát-sát, thần hướng, xuất hành) while preserving existing stable calendar math.

**Architecture:** Keep current `amlich-core` date engine (solar/lunar, Can Chi, Tiết Khí, Giờ Hoàng Đạo) unchanged. Refactor almanac into domain engines with explicit source-tagged rule packs and deterministic precedence, then expose through `amlich-api` and render in TUI.

**Tech Stack:** Rust (`amlich-core`, `amlich-api`, `amlich-cli`), serde/serde_json, Ratatui, cargo test, golden fixtures.

---

## Scope Baseline

Already implemented (keep):
- `day_fortune` skeleton + DTO plumbing
- baseline data loader + validations
- TUI sections for mệnh/xung/sao/thần hướng

Still needed (this plan):
- domain separation and source-backed rule packs
- full xung/hợp outputs
- trực 12 calculation
- stronger sao/cát-sát rule engine
- source metadata and reproducible golden tests

## Batch 0: Spec Freeze (No Code)

### Task 0.1: Lock glossary
**Files:**
- Modify: `docs/plans/2026-02-24-amlich-core-almanac-expansion-impl.md`

**Step 1:** Define exact meanings for:
- `mệnh ngày` = nạp âm của Can Chi ngày
- `sao` = nhị thập bát tú (or chosen alternative)
- `kỷ thần` term mapping (or leave explicitly nullable)

**Step 2:** Define month basis per domain:
- `lunar-month` vs `tiet-khi-month`

**Step 3:** Add a “Terminology Decision” section with chosen source references.

**Step 4:** Commit
```bash
git add docs/plans/2026-02-24-amlich-core-almanac-expansion-impl.md
git commit -m "docs: freeze almanac glossary and month-basis decisions"
```

## Batch 1: Data Contract Hardening

### Task 1.1: Add source metadata fields
**Files:**
- Modify: `crates/amlich-core/src/almanac/types.rs`
- Modify: `crates/amlich-core/src/almanac/data.rs`
- Modify: `crates/amlich-core/data/almanac/baseline.json`

**Step 1: Write failing tests**
- Add tests asserting every major rule includes `source_id`, `method`, and `profile`.

**Step 2: Run failing test**
Run: `cargo test -p amlich-core almanac::data --offline`
Expected: FAIL (missing metadata).

**Step 3: Implement minimal metadata fields**
- Extend data structs and JSON keys.
- Keep backward compatibility where possible.

**Step 4: Re-run test**
Run: `cargo test -p amlich-core almanac::data --offline`
Expected: PASS.

**Step 5: Commit**
```bash
git add crates/amlich-core/src/almanac/types.rs crates/amlich-core/src/almanac/data.rs crates/amlich-core/data/almanac/baseline.json
git commit -m "feat(core): add source metadata to almanac rule data"
```

### Task 1.2: Add canonical direction/method validators
**Files:**
- Modify: `crates/amlich-core/src/almanac/data.rs`

**Step 1:** Add failing tests for invalid `method`/direction token rejection.
**Step 2:** Run test and verify fail.
**Step 3:** Implement validators.
**Step 4:** Re-run and verify pass.
**Step 5:** Commit.

## Batch 2: Xung/Hợp Domain

### Task 2.1: Extract xung/hợp engine
**Files:**
- Create: `crates/amlich-core/src/almanac/xung_hop.rs`
- Modify: `crates/amlich-core/src/almanac/mod.rs`
- Modify: `crates/amlich-core/src/almanac/types.rs`
- Modify: `crates/amlich-core/src/almanac/calc.rs`

**Step 1: Write failing tests**
- `luc_xung` by day chi
- `tam_hop` group
- `tu_hanh_xung` group

**Step 2:** Run `cargo test -p amlich-core xung_hop --offline` and confirm FAIL.

**Step 3:** Implement minimal deterministic engine.

**Step 4:** Re-run targeted test and confirm PASS.

**Step 5:** Commit
```bash
git add crates/amlich-core/src/almanac/xung_hop.rs crates/amlich-core/src/almanac/mod.rs crates/amlich-core/src/almanac/types.rs crates/amlich-core/src/almanac/calc.rs
git commit -m "feat(core): implement xung/hop domain engine"
```

## Batch 3: Trực 12 Domain

### Task 3.1: Add `Truc` enum and calculator
**Files:**
- Create: `crates/amlich-core/src/almanac/truc.rs`
- Modify: `crates/amlich-core/src/almanac/types.rs`
- Modify: `crates/amlich-core/src/almanac/calc.rs`
- Modify: `crates/amlich-core/src/lib.rs` (if extra inputs needed)

**Step 1: Write failing tests**
- Known-date assertions for `Kiến/Trừ/.../Bế` under chosen month basis.

**Step 2:** Run failing test and confirm FAIL.

**Step 3:** Implement calculator.

**Step 4:** Re-run and confirm PASS.

**Step 5:** Commit
```bash
git add crates/amlich-core/src/almanac/truc.rs crates/amlich-core/src/almanac/types.rs crates/amlich-core/src/almanac/calc.rs crates/amlich-core/src/lib.rs
git commit -m "feat(core): add truc-12 calculation domain"
```

## Batch 4: Thần Hướng Domain

### Task 4.1: Separate thần hướng from generic map
**Files:**
- Create: `crates/amlich-core/src/almanac/than_huong.rs`
- Modify: `crates/amlich-core/src/almanac/data.rs`
- Modify: `crates/amlich-core/src/almanac/calc.rs`

**Step 1:** Add failing tests by day stem (`Giáp..Quý`) for Tài/Hỷ/Xuất hành.
**Step 2:** Run and confirm FAIL.
**Step 3:** Implement engine + source-tagged mapping.
**Step 4:** Run and confirm PASS.
**Step 5:** Commit.

## Batch 5: Sao + Cát/Sát Domain

### Task 5.1: Introduce rule categories
**Files:**
- Create: `crates/amlich-core/src/almanac/than_sat.rs`
- Create: `crates/amlich-core/src/almanac/star.rs`
- Modify: `crates/amlich-core/src/almanac/data.rs`
- Modify: `crates/amlich-core/src/almanac/calc.rs`

**Step 1: Write failing tests**
- Rule categories:
  - fixed by can-chi
  - by year
  - by month/day
  - by tiết khí window
- precedence conflict test

**Step 2:** Run and confirm FAIL.

**Step 3:** Implement first minimal category + precedence skeleton.

**Step 4:** Run and confirm partial PASS.

**Step 5:** Expand to all categories and finalize tests.

**Step 6:** Commit
```bash
git add crates/amlich-core/src/almanac/than_sat.rs crates/amlich-core/src/almanac/star.rs crates/amlich-core/src/almanac/data.rs crates/amlich-core/src/almanac/calc.rs
git commit -m "feat(core): implement source-backed sao/cat-sat rule engine"
```

## Batch 6: API Contract Expansion

### Task 6.1: Add domain DTOs
**Files:**
- Modify: `crates/amlich-api/src/dto.rs`
- Modify: `crates/amlich-api/src/convert.rs`
- Modify: `crates/amlich-api/tests/almanac_contract.rs`

**Step 1:** Write failing API contract tests for new fields (`xung_hop`, `truc`, domain-level `star` evidence).
**Step 2:** Run and confirm FAIL.
**Step 3:** Implement DTO/convert updates.
**Step 4:** Run and confirm PASS.
**Step 5:** Commit.

## Batch 7: TUI Rendering Refinement

### Task 7.1: Render new domain outputs cleanly
**Files:**
- Modify: `crates/amlich/src/widgets/info_panel.rs`
- Modify: `crates/amlich/src/widgets/detail.rs` (if needed)

**Step 1:** Add failing render test for required labels/sections.
**Step 2:** Run and confirm FAIL.
**Step 3:** Implement compact line layout with truncation strategy.
**Step 4:** Run and confirm PASS.
**Step 5:** Commit.

## Batch 8: Golden Fixtures and End-to-End Verification

### Task 8.1: Add canonical fixture set
**Files:**
- Create: `crates/amlich-core/tests/almanac_golden.rs`
- Modify: `crates/amlich-api/tests/golden_parity.rs`
- Modify: `crates/amlich/tests/cli_contract.rs`

**Step 1:** Add 12 canonical dates across seasons and edge boundaries.
**Step 2:** Add expected outputs for all almanac fields.
**Step 3:** Run full failing tests and capture deltas.
**Step 4:** Fix mismatches.
**Step 5:** Re-run full suite:
```bash
cargo test -p amlich-core --offline
cargo test -p amlich-api --offline
cargo test -p amlich-cli --offline
```
Expected: all PASS.

**Step 6:** Commit
```bash
git add crates/amlich-core/tests/almanac_golden.rs crates/amlich-api/tests/golden_parity.rs crates/amlich/tests/cli_contract.rs
git commit -m "test: add golden almanac parity coverage across core api and cli"
```

## Execution Checkpoints

- Checkpoint A: end of Batch 2 (xung/hop done)
- Checkpoint B: end of Batch 4 (trực + thần hướng done)
- Checkpoint C: end of Batch 6 (API contract stable)
- Checkpoint D: end of Batch 8 (golden verified)

## Risk Controls

- Keep old fields intact; only additive contract changes until final freeze.
- Source-tag every non-astronomical output.
- Never merge new rule category without failing test first.
- Lock precedence rules in tests to avoid silent drift.
