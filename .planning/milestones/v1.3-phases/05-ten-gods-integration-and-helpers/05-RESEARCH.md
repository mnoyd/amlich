# Phase 5: Ten Gods Integration and Helpers - Research

**Researched:** 2026-03-03
**Domain:** Dai Van pillar enrichment (Ten Gods) and age-query helper contracts in Rust core
**Confidence:** HIGH

## Summary

Phase 5 should be planned as integration-only work on top of the already-stable `dai_van.rs` foundation from Phase 4. The core helper functions required by DV-HELP-01..04 already exist in `crates/amlich-core/src/almanac/dai_van.rs` and already implement the required Option-based out-of-range behavior. Planning should therefore treat helper work as verification/hardening/refinement, not net-new architecture.

The true implementation focus is DV-TG-01..03: add lazy Ten Gods correlation per pillar by reusing `thap_than::get_thap_than()` without modifying the Ten Gods engine. The critical correctness rule is stem direction: relation must be `birth_day_stem -> pillar_stem` (not pillar->day, and never pillar->pillar). Existing project decisions and prior pitfalls already identify this as the main integration risk.

Unknown birth hour handling (DV-TG-03) must be explicit at API boundaries. The phase can satisfy this either by accepting an optional day stem and returning `None` when absent, or by documented fallback to day_fortune-derived targets where available. For clean planning and deterministic behavior, prefer an explicit `Option<HeavenlyStem>` path in Dai Van Ten Gods helpers.

**Primary recommendation:** Plan Phase 5 as two tracks: (1) add lazy Ten Gods-per-pillar APIs and tests; (2) formally validate/helper-contract lock-in for existing age helper behavior.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| DV-TG-01 | System can correlate each pillar's Heavenly Stem with birth day stem via Thap Than | Reuse `get_thap_than(day_stem, pillar_stem)` from `crates/amlich-core/src/almanac/thap_than.rs`; add pillar-level adapter in `dai_van.rs` that converts pillar can string/index to `HeavenlyStem`. |
| DV-TG-02 | Ten Gods calculation is lazy/on-demand (not pre-computed for all pillars) | Implement query methods that compute only when requested (e.g. `get_ten_gods_for_pillar(...)`), keep `DaiVanResult` free of eagerly populated Ten Gods arrays. |
| DV-TG-03 | System supports unknown birth hour gracefully (Ten Gods = None or day_fortune-based targets) | Accept optional day stem input and return `None` when unavailable; document optional fallback policy and keep behavior deterministic. |
| DV-HELP-01 | System can find current pillar for given age | Already implemented as `get_current_pillar(result, age)` in `crates/amlich-core/src/almanac/dai_van.rs`. |
| DV-HELP-02 | System can calculate years until next transition | Already implemented as `years_to_next_transition(result, age)` in `crates/amlich-core/src/almanac/dai_van.rs`. |
| DV-HELP-03 | System can find pillar at specific age (range lookup) | Already implemented as `get_pillar_at_age(result, age)` in `crates/amlich-core/src/almanac/dai_van.rs`. |
| DV-HELP-04 | Helper functions return Option to handle out-of-range ages gracefully | Existing helpers already return `Option`; phase should lock this contract with focused integration tests and avoid any clamping behavior. |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust (workspace) | edition 2021 | Deterministic core implementation | Existing project baseline; all required primitives already in place. |
| `serde` | 1.0 (workspace) | Stable serialization for structs/enums | Project-wide contract pattern for API-safe types. |
| `amlich-core::almanac::thap_than` | in-repo module | Ten Gods label/relation computation | Already validated with full 10x10 matrix tests in module; avoid duplicate logic. |
| `amlich-core::almanac::dai_van` | in-repo module | Pillar lifecycle + age helper foundation | Phase 4 shipped this as stable upstream dependency for Phase 5. |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `amlich-core::almanac::types::HeavenlyStem` | in-repo type | Typed stem parsing and polarity/element metadata | Use as input/output boundary for Ten Gods helper APIs. |
| Existing test harness (`cargo test`) | workspace | Contract and integration verification | Use for helper boundary tests and lazy-computation behavior checks. |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Reusing `get_thap_than` | New Dai Van-local Ten Gods mapping table | High duplication risk and drift from v1.2-validated mapping. |
| Lazy query API | Eager Ten Gods for all 8 pillars in `calculate_dai_van` | Simpler read path but unnecessary compute and violates DV-TG-02. |
| `Option<HeavenlyStem>` unknown-hour contract | Silent default stem | Hidden assumptions, non-traceable behavior, and correctness regressions. |

**Installation:**
```bash
cargo test --package amlich-core --lib
```

## Architecture Patterns

### Recommended Project Structure
```
crates/amlich-core/src/almanac/
├── dai_van.rs      # Dai Van result/pillar types + helper lookups + new lazy Ten Gods adapter(s)
├── thap_than.rs    # Existing Ten Gods engine (reuse only; no behavior change expected)
└── types.rs        # Shared HeavenlyStem/ThapThanResult types used by adapter signatures
```

### Pattern 1: Module-Level Reuse Without Modification
**What:** Keep Ten Gods formula logic centralized in `thap_than.rs` and call it from Dai Van helper adapters.
**When to use:** Any pillar-level Ten Gods request in Phase 5.
**Example:**
```rust
use crate::almanac::thap_than::get_thap_than;
use crate::almanac::types::HeavenlyStem;

pub fn ten_god_for_pillar(day_stem: HeavenlyStem, pillar_stem: HeavenlyStem) -> ThapThanResult {
    get_thap_than(day_stem, pillar_stem)
}
```
Source: `crates/amlich-core/src/almanac/thap_than.rs`

### Pattern 2: Lazy Query API for Pillar Enrichment
**What:** Compute Ten Gods only for requested pillar/age, not all pillars at Dai Van calculation time.
**When to use:** Any user query like "current pillar Ten God" or "pillar at age X".
**Example:**
```rust
pub fn get_ten_gods_for_age(
    result: &DaiVanResult,
    age: f64,
    birth_day_stem: Option<HeavenlyStem>,
) -> Option<ThapThanResult> {
    let day_stem = birth_day_stem?;
    let pillar = get_pillar_at_age(result, age)?;
    let pillar_stem = HeavenlyStem::try_from(pillar.can_chi.can.as_str()).ok()?;
    Some(get_thap_than(day_stem, pillar_stem))
}
```
Source pattern: `crates/amlich-core/src/almanac/dai_van.rs` helper style + `crates/amlich-core/src/almanac/thap_than.rs`

### Anti-Patterns to Avoid
- **Duplicate Ten Gods matrix in Dai Van:** creates maintenance drift and bypasses proven tests.
- **Eagerly attaching Ten Gods to every pillar in `calculate_dai_van`:** violates lazy requirement and adds unnecessary work.
- **Incorrect stem direction:** using `pillar->day` or `pillar->pillar` silently returns wrong semantics.
- **Silent default for missing day stem/birth hour:** breaks explicitness required by DV-TG-03.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Ten Gods relation mapping | New custom lookup/matrix in `dai_van.rs` | `thap_than::get_thap_than()` | Existing engine already has complete mapping tests and evidence metadata pattern. |
| Age range search logic | New separate range-index utility | Existing `get_pillar_at_age()`/`get_current_pillar()` | Phase 4 helpers already encode half-open boundary semantics correctly. |
| Unknown hour fallback inference | Implicit guessed day stem | Explicit `Option` path + documented fallback | Keeps deterministic and auditable behavior. |

**Key insight:** Phase 5 is an integration phase; new correctness comes from composition and contracts, not from new core algorithms.

## Common Pitfalls

### Pitfall 1: Wrong Stem Orientation in Ten Gods Correlation
**What goes wrong:** Correlation uses the wrong source/target stem order.
**Why it happens:** `get_thap_than(a, b)` is easy to call with reversed arguments when wiring from pillar models.
**How to avoid:** Name parameters explicitly (`birth_day_stem`, `pillar_stem`) and enforce with targeted tests.
**Warning signs:** Excessive `ty_kien` outputs or mismatch with known fixture expectations.

### Pitfall 2: Violating Lazy Requirement
**What goes wrong:** Ten Gods computed for all 8 pillars during `calculate_dai_van`.
**Why it happens:** Convenience-driven design to "store everything upfront".
**How to avoid:** Keep Ten Gods out of base `DaiVanResult`; expose query functions only.
**Warning signs:** New loops in Dai Van calculation path invoking `get_thap_than` for every pillar.

### Pitfall 3: Unknown Birth Hour Not Handled Explicitly
**What goes wrong:** Missing day stem gets silently defaulted or panics.
**Why it happens:** Incomplete API contract for optional birth context.
**How to avoid:** `Option` in function signature and early-return `None` semantics.
**Warning signs:** `unwrap()` on day stem parsing or inferred fallback values without metadata.

### Pitfall 4: Helper Contract Drift
**What goes wrong:** Helper functions start clamping ages instead of returning `None`.
**Why it happens:** UI convenience changes leak into core.
**How to avoid:** Keep helper tests asserting out-of-range -> `None` at both ends.
**Warning signs:** Ages before first pillar or at/after final end age returning a pillar.

## Code Examples

Verified existing patterns in this repo:

### Existing Option-based Helper Contract
```rust
pub fn get_pillar_at_age(result: &DaiVanResult, age: f64) -> Option<&DaiVanPillar> {
    result
        .pillars
        .iter()
        .find(|pillar| age >= pillar.start_age && age < pillar.end_age)
}
```
Source: `crates/amlich-core/src/almanac/dai_van.rs:188`

### Existing Ten Gods Engine Entry Point
```rust
pub fn get_thap_than(day_can: HeavenlyStem, target_can: HeavenlyStem) -> ThapThanResult
```
Source: `crates/amlich-core/src/almanac/thap_than.rs:5`

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Ten Gods only at day-fortune level (`to_self`, `to_year_stem`) | Add pillar-level lazy adapters in Dai Van while reusing same engine | v1.3 Phase 5 (planned) | Extends capability without rewriting Ten Gods core. |
| Helper semantics could be implicit in design docs | Concrete Option-returning helper APIs already in core (`dai_van.rs`) | v1.3 Phase 4 completed 2026-03-03 | Phase 5 can focus on contract locking and integration tests. |

**Deprecated/outdated:**
- Re-implementing Ten Gods logic outside `thap_than.rs` is outdated for this codebase and should not be planned.

## Open Questions

1. **Unknown birth hour policy variant for Phase 5 output**
   - What we know: Requirement allows either `None` or day_fortune-based targets.
   - What's unclear: Which fallback should be canonical for this milestone.
   - Recommendation: Plan default as `None` when birth day stem unavailable; treat day_fortune fallback as explicit optional extension if product needs it.

2. **Public exposure scope for new Ten Gods helpers**
   - What we know: Core module integration is required; API-layer exposure is deferred in requirements.
   - What's unclear: Whether to re-export new helpers from `amlich-core/src/lib.rs` in this phase.
   - Recommendation: Keep phase scope core-internal unless a plan explicitly maps to an API requirement (none in Phase 5).

## Sources

### Primary (HIGH confidence)
- `crates/amlich-core/src/almanac/dai_van.rs` - existing helper contracts and phase-4 baseline
- `crates/amlich-core/src/almanac/thap_than.rs` - Ten Gods engine and deterministic mapping behavior
- `crates/amlich-core/src/almanac/types.rs` - HeavenlyStem, ThapThanResult, and Option-bearing model patterns
- `.planning/REQUIREMENTS.md` - authoritative Phase 5 requirement definitions (DV-TG-01..03, DV-HELP-01..04)
- `.planning/ROADMAP.md` - phase dependency and success criteria
- `.planning/STATE.md` - prior decisions and known pitfalls to carry into planning

### Secondary (MEDIUM confidence)
- `.planning/research/SUMMARY.md` - prior project-wide recommendations for phase ordering and risk controls
- `.planning/research/FEATURES.md` - feature prioritization and anti-feature guidance

### Tertiary (LOW confidence)
- None needed for this phase; domain behavior is fully covered by in-repo artifacts.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - all implementation dependencies already exist and are in active use.
- Architecture: HIGH - Phase 4 established stable integration boundaries and helper contracts.
- Pitfalls: HIGH - pitfalls are directly evidenced by existing state/research documents and code-level interfaces.

**Research date:** 2026-03-03
**Valid until:** 2026-04-02
