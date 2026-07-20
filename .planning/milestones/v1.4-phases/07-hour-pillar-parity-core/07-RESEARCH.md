# Phase 7: Hour Pillar Parity Core - Research

**Researched:** 2026-03-03
**Domain:** Deterministic hour-pillar (gio tru) mapping from day stem + local hour with boundary-safe windows
**Confidence:** HIGH

## Summary

Phase 7 should introduce a dedicated hour-pillar core module in `amlich-core` that computes Can Chi hour deterministically from two inputs: day heavenly stem and local clock hour. The codebase already has the canonical stem/branch primitives (`HeavenlyStem`, `CanChi`, `normalize_index`) and a fixed 12-window hour table in `gio_hoang_dao.rs` (`23:00-01:00` through `21:00-23:00`), so implementation should reuse these conventions rather than create alternate encodings.

The key correctness risk is boundary handling around the Tý slot split (23:00-00:59) and transitions at odd/even clock edges. The second key risk is the day-stem seed grouping rule:
- Giáp/Kỷ -> Tý hour stem starts at Giáp
- Ất/Canh -> starts at Bính
- Bính/Tân -> starts at Mậu
- Đinh/Nhâm -> starts at Canh
- Mậu/Quý -> starts at Nhâm

From that seed, stems advance by +1 for each two-hour slot while branches advance in fixed order Tý..Hợi. This yields a pure table-driven algorithm with no external IO, no timezone conversions, and no floating-point behavior.

## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| HP-01 | Compute hour pillar from day stem + local hour via 12 fixed windows | Add deterministic `resolve_hour_branch_slot(hour, minute)` + `compute_hour_pillar(day_stem, hour, minute)` APIs in core. |
| HP-02 | Apply 5 day-stem seed groups correctly | Encode seed-group mapping as explicit match table; verify all 10 stems across 12 slots. |
| HP-03 | Handle all 12 window boundaries without overlap/gap | Add tests for every boundary minute around each transition (e.g., xx:59 -> next xx+1:00). |
| HP-04 | Include RuleEvidence-aligned metadata | Return result struct with `evidence: RuleEvidence { source_id, method, profile }`. |
| HP-05 | Fixtures cover all stem groups + rollover cases | Add fixture matrix test covering all 5 groups and Tý rollover at 23:xx/00:xx. |
| PAR-02 | Golden fixtures include representative + boundary cases | Create dedicated fixture set in tests mirroring parity expectations and boundary cases. |

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust workspace | edition 2021 | Deterministic hour-pillar algorithm | Existing core baseline and test tooling. |
| `amlich_core::almanac::types::HeavenlyStem` | in-repo | Typed day stem input and element/polarity utilities | Avoids stringly-typed stem logic. |
| `amlich_core::types::{CanChi, normalize_index}` | in-repo | Canonical stem/branch index arithmetic | Already used for other deterministic sexagenary logic. |
| `RuleEvidence` | in-repo | Metadata contract alignment | Matches existing evidence conventions in day fortune features. |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `cargo test` | workspace | Boundary and parity verification | Primary validation path for this phase. |
| Existing `gio_hoang_dao` hour windows | in-repo | Canonical two-hour time ranges | Reuse as reference for slot boundaries. |

## Architecture Pattern

### Pattern: Pure Mapping Module

Add a focused module (recommended: `crates/amlich-core/src/almanac/hour_pillar.rs`) with:
- input contract: `day_stem`, `local_hour`, `local_minute`
- internal helpers:
  - hour->branch-slot mapping (12 fixed windows)
  - day-stem->seed mapping (5 groups)
  - stem progression by slot index
- output contract containing:
  - hour can, hour chi, full canchi
  - slot metadata
  - RuleEvidence

No calendar conversion or timezone logic should exist here. Callers must provide local hour values explicitly.

## Common Pitfalls

1. **Tý boundary split bug**
   - 23:00-23:59 and 00:00-00:59 are both Tý hour; implementations that split by date may misclassify 00:xx.
2. **Seed offset off-by-one**
   - Using slot index with wrong base causes all stems after first window to drift.
3. **Boundary overlap/gap**
   - Inclusive upper bounds (`<=`) on adjacent windows produce duplicate membership at boundary minutes.
4. **String-based stem math**
   - Manual string lookups are fragile; prefer index-based progression via existing stem enum/index mapping.
5. **Metadata drift**
   - Returning bare `CanChi` without evidence violates HP-04 and parity-traceability goals.

## Recommended Validation Matrix

- **Seed-group parity:** 5 cases (one for each group) x key slots (Tý, Mão, Ngọ, Dậu, Hợi)
- **Full progression:** at least one day stem validates all 12 slots in sequence
- **Boundary transitions:** each transition minute pair (`xx:59`, `xx+1:00`) including 22:59->23:00 and 00:59->01:00
- **Input guards:** invalid hour/minute rejected deterministically
- **Evidence assertions:** every result includes non-empty `source_id`, `method`, `profile`

## Sources

- `crates/amlich-core/src/almanac/types.rs`
- `crates/amlich-core/src/types.rs`
- `crates/amlich-core/src/gio_hoang_dao.rs`
- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`

---

*Phase: 07-hour-pillar-parity-core*
*Research generated: 2026-03-03*
