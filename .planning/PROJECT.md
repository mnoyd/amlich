# Amlich Almanac Correctness Audit

## What This Is

A systematic correctness audit of the amlich almanac ruleset against the Khâm Định Hiệp Kỷ Biện Phương Thư (KHCBPPT), the authoritative classical text for Vietnamese calendar divination. The project builds a golden reference dataset of KHCBPPT-verified almanac data for the 2020–2030 date range and fixes any divergences found in the existing implementation.

## Core Value

Every almanac subsystem in amlich must produce output that matches KHCBPPT for the 2020–2030 date range — stars, taboos, day deity, trực, xung hợp, thần hướng, and nạp âm/ngũ hành.

## Requirements

### Validated

- ✓ Star rule engine with cat_tinh/sat_tinh and precedence resolution — existing
- ✓ Taboo rules: Tam Nương, Nguyệt Kỵ, Sát Chủ, Thọ Tử — existing
- ✓ Day deity cycle (Hoàng Đạo/Hắc Đạo) — existing
- ✓ Thập nhị trực formula — existing
- ✓ Xung hợp (Lục Xung, Tam Hợp, Tứ Hành Xung) — existing
- ✓ Thần hướng (travel direction by Thiên Can) — existing
- ✓ Nạp Âm / Ngũ Hành element lookup — existing
- ✓ Evidence tracking with RuleEvidence per component — existing
- ✓ Data-driven ruleset via baseline.json — existing

### Active

- [ ] KHCBPPT reference data compiled for each almanac subsystem
- [ ] Golden reference dataset covering representative dates in 2020–2030
- [ ] Star rules (cat_tinh/sat_tinh) cross-referenced and corrected against KHCBPPT
- [ ] Taboo rules cross-referenced and corrected against KHCBPPT
- [ ] Day deity mapping cross-referenced and corrected against KHCBPPT
- [ ] Trực quality assignments cross-referenced and corrected against KHCBPPT
- [ ] Xung hợp relationships cross-referenced and corrected against KHCBPPT
- [ ] Thần hướng directions cross-referenced and corrected against KHCBPPT
- [ ] Nạp Âm / Ngũ Hành tables cross-referenced and corrected against KHCBPPT
- [ ] All divergences fixed in baseline.json and/or code

### Out of Scope

- Lunar/solar date conversion algorithm — already well-tested, separate concern
- TUI, CLI, WASM, or desktop UI changes — this is a core correctness project
- Adding entirely new almanac subsystems not in KHCBPPT — focus on getting existing rules right
- Date ranges outside 2020–2030 — practical daily use is the priority
- Performance optimization — correctness first

## Context

The amlich almanac ruleset was built with KHCBPPT as the stated source (evidence metadata shows `source_id: "khcbppt"`), but the implementation has not been systematically cross-referenced against the original text. The ruleset data lives in `crates/amlich-core/data/almanac/baseline.json` and is accessed via `crates/amlich-core/src/almanac/data.rs`.

Current test coverage includes golden tests for trực, xung hợp, and day deity cycles, plus unit tests per module. However, these tests verify internal consistency, not KHCBPPT accuracy.

The almanac calculation chain (`calc.rs` → `than_sat.rs` → `star.rs` → `taboo.rs` → `day_deity.rs` → `truc.rs` → `xung_hop.rs` → `than_huong.rs`) processes multiple data lookups per date. Changes to `baseline.json` schema can cause runtime panics in several modules.

## Constraints

- **Source authority**: KHCBPPT is the single source of truth for all rule validation
- **Date range**: 2020–2030 for practical coverage; rules themselves are cyclical so this covers all pattern combinations
- **Data format**: Golden dataset must be machine-readable for automated validation
- **Backward compatibility**: Fixes to baseline.json must not break the existing data schema or API contract

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| KHCBPPT as sole reference | Most authoritative classical text for Vietnamese almanac | — Pending |
| 2020–2030 date range focus | Covers practical daily use; cyclical rules mean full pattern coverage | — Pending |
| Golden dataset approach | Enables automated regression testing, not just one-time audit | — Pending |

---
*Last updated: 2026-02-28 after initialization*
