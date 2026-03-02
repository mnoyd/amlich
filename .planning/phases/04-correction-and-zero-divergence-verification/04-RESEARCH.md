# Phase 4: Correction and Zero-Divergence Verification - Research

**Researched:** 2026-03-02
**Domain:** Rust almanac correction workflow, data/code reconciliation, zero-divergence verification gates
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

### Correction authority
- Golden dataset is canonical: when validator output and implementation disagree, implementation must be corrected to match KHCBPPT-cited golden entries.
- If any golden entry looks suspicious, do not force-match blindly; block and resolve the source evidence first.
- API/output contract shape must remain unchanged in this phase (behavior corrections only).
- Broad refactoring is allowed to eliminate divergence clusters, but acceptance requires a full green run: all KHCBPPT validators plus existing regression suites.
- Prefer KHCBPPT conceptual alignment over preserving legacy quirks when they conflict.
- Execute as a single coordinated correction batch (not piecemeal subsystem landings).
- Provide explicit correction notes grouped by subsystem for downstream planner/researcher use.

### Mismatch handling policy
- Phase completion requires strict zero divergence; no residual mismatches are allowed.
- Uncertain interpretation mismatches must be resolved before merge; do not defer within Phase 4 completion criteria.
- No temporary or persistent test suppression (no ignore/allowlist strategy).
- Maintain a per-mismatch correction ledger including: date, subsystem, affected entry/date, KHCBPPT citation reference, and what changed.

### Claude's Discretion
- Exact refactor structure and sequencing inside the single correction batch.
- Exact artifact format/location for the per-mismatch ledger and grouped subsystem notes.
- Exact wording style for correction notes, as long as required audit fields are present.

### Deferred Ideas (OUT OF SCOPE)

None - discussion stayed within phase scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| TAB-05 | All divergences fixed in baseline.json | Plan a taboo-specific correction pass in `crates/amlich-core/data/almanac/baseline.json`, then run `khcbppt_taboos.rs` and full package tests as hard gates |
| DEI-03 | All divergences fixed in baseline.json | Correct `day_deity_rule_set` values in `baseline.json` (cycle + month offsets) with citation-linked ledger entries |
| TRC-02 | All divergences fixed in `TRUC_QUALITY` const in `truc.rs` | Treat `crates/amlich-core/src/almanac/truc.rs` as the authoritative fix point; requires code edit + compile + full regressions |
| STR-04 | All divergences fixed in baseline.json | Correct `nhi_thap_bat_tu` and/or `star_rule_sets` + metadata in `baseline.json`, with special handling for JD epoch and contextual sparsity findings |
| THH-02 | All divergences fixed in baseline.json | Correct `travel_by_can` in `baseline.json` using KHCBPPT `than_huong.md` references and verify via `khcbppt_than_huong.rs` |
| XH-02 | All divergences fixed in `xung_hop.rs` | Apply formula/constant corrections in `crates/amlich-core/src/almanac/xung_hop.rs`; verify sorted-set semantics remain intact |
| NAM-02 | All divergences fixed in baseline.json | Correct `na_am_pairs` and related source attribution in `baseline.json`, keeping per-entry citation traceability |
</phase_requirements>

## Summary

Phase 4 is a correction-and-proof phase, not a harness-building phase. The core implementation pattern is to treat each mismatch as an auditable record, fix at the true source of behavior (`baseline.json` or specific Rust constants/modules), and repeatedly re-run `cargo test --package amlich-core` until every `khcbppt_*.rs` validator and all pre-existing regressions pass together. The current suite already passes because the golden dataset was generated from implementation output; Phase 4 planning must assume golden values are updated/validated against KHCBPPT citations and then drive implementation into convergence.

There are two distinct correction surfaces. Data-driven subsystems (taboo, deity, stars, than_huong, na_am) mostly land in `crates/amlich-core/data/almanac/baseline.json` and are guarded by strict loader invariants in `data.rs` (coverage, token validity, map completeness). Code-driven subsystems (`TRUC_QUALITY` in `truc.rs`, xung_hop logic in `xung_hop.rs`) require Rust source edits and recompilation. Planning should separate these pathways while keeping one coordinated landing batch, as required by context.

Traceability is the non-negotiable acceptance mechanism. `khcbppt-golden.json` already carries per-entry `khcbppt_ref` fields, but Phase 4 also needs an explicit mismatch ledger artifact that links each correction to date/subsystem/citation/change. Without that ledger, requirement-level completion can appear green in tests but fail auditability criterion #3.

**Primary recommendation:** Plan one coordinated correction batch with a mandatory mismatch ledger, subsystem-grouped fixes at true behavior sources (`baseline.json`, `truc.rs`, `xung_hop.rs`), and repeated full-suite gating (`cargo test --package amlich-core`) until strict zero divergence is achieved.

## Standard Stack

### Core
| Library/Tool | Version | Purpose | Why Standard |
|--------------|---------|---------|--------------|
| Rust built-in test runner (`cargo test`) | stable | Unified verification gate for validators + regressions | Already authoritative in project; success criteria explicitly require this command |
| `amlich-core` golden loader (`load_golden_dataset`) | workspace | Canonical expected values with per-entry citations | Existing typed oracle with invariants and cache behavior already proven |
| `baseline.json` + `data.rs` loader invariants | workspace | Data-driven rule corrections with schema/invariant validation | Ensures malformed corrections fail fast during load |

### Supporting
| Library/Tool | Version | Purpose | When to Use |
|--------------|---------|---------|-------------|
| `khcbppt_*.rs` validator tests | workspace | Subsystem divergence detection and report format | After every correction slice and at final gate |
| Existing regression suites (`almanac_golden.rs`, `ruleset_determinism.rs`, `taboo_boundary.rs`) | workspace | Detect behavior regressions outside target mismatch | Run in every full-gate cycle |
| KHCBPPT reference docs in `docs/reference/khcbppt/*.md` | workspace docs | Citation source for deciding/justifying corrections | During mismatch triage and ledger population |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Single coordinated correction batch | Incremental per-subsystem merges | Conflicts with locked decision requiring single coordinated landing |
| Strict fail-on-mismatch | Ignore lists / temporary suppression | Explicitly disallowed by mismatch handling policy |
| Source-of-truth corrections | Patching tests only | Violates correction authority and breaks audit integrity |

**Installation:**
```bash
# No new dependencies required
cargo test --package amlich-core
```

## Architecture Patterns

### Recommended Project Structure
```
crates/amlich-core/
├── data/almanac/baseline.json              # data corrections (TAB/DEI/STR/THH/NAM)
├── src/almanac/truc.rs                     # TRC-02 corrections
├── src/almanac/xung_hop.rs                 # XH-02 corrections
├── tests/khcbppt_*.rs                      # divergence validators
└── tests/{almanac_golden,ruleset_determinism,taboo_boundary}.rs

.planning/phases/04-correction-and-zero-divergence-verification/
└── 04-correction-ledger.md                 # per-mismatch audit ledger (recommended)
```

### Pattern 1: Mismatch Ledger-First Correction
**What:** Every mismatch gets a ledger row before/with code change: date, subsystem, affected entry/date, KHCBPPT citation, correction summary.
**When to use:** For all corrected divergences; mandatory for acceptance criterion #3.
**Example:**
```markdown
| Date | Subsystem | Affected Entry | KHCBPPT Citation | Change |
|------|-----------|----------------|------------------|--------|
| 2026-03-02 | STR-04 | 2024-02-10 | KHCBPPT, Quyển 13, Công Quy | baseline.json `nhi_thap_bat_tu[4].quality` hung -> cat |
```

### Pattern 2: Fix at True Behavior Source
**What:** Correct where behavior is produced, not where it is observed.
**When to use:** Always; prevents test-only or duplicated fixes.
**Example:**
```text
TAB/DEI/STR/THH/NAM mismatches -> crates/amlich-core/data/almanac/baseline.json
TRC mismatches -> crates/amlich-core/src/almanac/truc.rs::TRUC_QUALITY
XH mismatches -> crates/amlich-core/src/almanac/xung_hop.rs formulas/mappings
```

### Pattern 3: Two-Level Verification Loop
**What:** Run targeted validator during active correction, then always run full package suite for acceptance.
**When to use:** Every correction cycle in Phase 4.
**Example:**
```bash
# fast local feedback during a specific subsystem fix
cargo test --package amlich-core --test khcbppt_taboos

# mandatory acceptance gate after each correction batch
cargo test --package amlich-core
```

### Anti-Patterns to Avoid
- **Test suppression:** any ignore/allowlist strategy is explicitly prohibited.
- **Golden mutation without evidence:** changing expected values without KHCBPPT citation-backed rationale breaks auditability.
- **Data/code mismatch fixes in wrong layer:** e.g., patching `calc.rs` for a bad baseline table entry.
- **Partial green acceptance:** passing validators but skipping legacy regressions is not acceptable.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Divergence triage framework | Custom parser for test logs | Existing `khcbppt_*.rs` mismatch reports + subsystem grouping | Already emits actionable mismatch strings by date/field |
| Citation traceability schema | New DB/tooling | Lightweight markdown ledger in phase folder | Meets audit need with minimal operational complexity |
| Rule loading/validation | Ad-hoc JSON readers | Existing `baseline_data()` and invariants in `data.rs` | Prevents silent schema drift and token errors |
| Regression coverage recreation | New parallel test suite | Existing `almanac_golden.rs`, `ruleset_determinism.rs`, `taboo_boundary.rs` | Required by success criteria and already stable |

**Key insight:** Phase 4 risk is governance (wrong-source fixes, untraceable changes), not missing infrastructure. Existing validators/loaders already provide the enforcement surface.

## Common Pitfalls

### Pitfall 1: Treating Current Zero Divergence as Completion
**What goes wrong:** Team assumes Phase 4 is already done because tests are green now.
**Why it happens:** Golden dataset currently mirrors implementation output.
**How to avoid:** Plan explicitly for KHCBPPT-grounded correction input and require updated ledger-backed evidence.
**Warning signs:** No `baseline.json`/source changes but declaring TAB-05..NAM-02 complete.

### Pitfall 2: Breaking Loader Invariants During Data Fixes
**What goes wrong:** Quick JSON edits fail with panics (invalid keys/tokens/missing months).
**Why it happens:** `data.rs` enforces strict can/chi/month/tiet-khi/schema constraints.
**How to avoid:** Validate each edited section against existing map cardinality and token rules before full run.
**Warning signs:** Panics from `validate_*` functions on first test load.

### Pitfall 3: Ignoring Cross-Subsystem Coupling
**What goes wrong:** A correction fixes one validator but regresses `almanac_golden` or determinism behavior.
**Why it happens:** Shared calculation path in `calculate_day_fortune()` fans out evidence + values to multiple consumers.
**How to avoid:** Enforce full-suite gate after each correction batch, not just target validator.
**Warning signs:** Single validator green while package suite fails elsewhere.

### Pitfall 4: Citation Drift and Untraceable Edits
**What goes wrong:** Values are corrected but no clear KHCBPPT reference link is retained.
**Why it happens:** Ad-hoc edits during rapid mismatch resolution.
**How to avoid:** Ledger-first discipline and explicit mapping to `khcbppt_ref` fields.
**Warning signs:** Commit diffs change constants/data without subsystem note + citation.

### Pitfall 5: JD Epoch Ambiguity Mishandled
**What goes wrong:** Star mismatches are force-fit without resolving whether issue is epoch alignment vs value table.
**Why it happens:** `jd.rem_euclid(28)` is implementation-derived and only medium confidence versus KHCBPPT docs.
**How to avoid:** Separate epoch-alignment decisions from quality/name table corrections in the plan and ledger.
**Warning signs:** Broad star edits with no mention of epoch rationale.

## Code Examples

Verified project patterns to reuse in Phase 4 execution notes:

### Correction Surface Boundaries
```rust
// Source: crates/amlich-core/src/almanac/truc.rs
pub const TRUC_QUALITY: [&str; 12] = [
    "cat", "cat", "hung", "binh", "cat", "binh",
    "hung", "hung", "cat", "hung", "cat", "hung",
];
// TRC-02 fixes land here (code change), not in baseline.json.
```

### Data-Driven Star Epoch Hook
```rust
// Source: crates/amlich-core/src/almanac/calc.rs
let day_star_index = jd.rem_euclid(28) as usize;
let day_star_rule = &data.nhi_thap_bat_tu[day_star_index];
// STR-04 planning must distinguish epoch logic vs table content corrections.
```

### Full Acceptance Gate
```bash
# Source: phase success criteria and existing workflow
cargo test --package amlich-core
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Spot checks + implicit correctness | 7 subsystem validators + typed golden loader | Phase 3 | Full-surface divergence detection available |
| Unstructured corrections | Citation-backed golden references + required ledger policy | Phase 4 context decision | Audit-ready correction trail |
| Subsystem-local confidence | Package-wide green gate including regressions | Existing tests + Phase 4 goal | Prevents local fixes from creating collateral regressions |

**Deprecated/outdated for this phase:**
- Treating generated-golden self-consistency as KHCBPPT correctness proof.
- Shipping with known/uncertain mismatches.

## Open Questions

1. **Where should the required mismatch ledger live and in what schema?**
   - What we know: Context mandates required fields; format/location is discretionary.
   - What's unclear: Whether phase docs should host final ledger or repo docs should.
   - Recommendation: Use `.planning/phases/04-correction-and-zero-divergence-verification/04-correction-ledger.md` with stable table columns.

2. **How should suspect golden entries be operationally blocked?**
   - What we know: Policy says block and resolve source evidence first.
   - What's unclear: Exact workflow marker (failing test annotation vs ledger status marker).
   - Recommendation: Add a `Status` column in ledger (`open-blocked`, `resolved`) and prohibit merge until no blocked rows remain.

3. **Should `star_meta.source_id` update to `khcbppt` be bundled in this phase?**
   - What we know: Phase 1/STATE note flagged this for Phase 4; current value is `nhi-thap-bat-tu`.
   - What's unclear: Whether team considers this required to satisfy STR-04 traceability in same batch.
   - Recommendation: Include it in Phase 4 plan as a traceability-alignment task if KHCBPPT citations are now authoritative.

## Sources

### Primary (HIGH confidence)
- `/.planning/phases/04-correction-and-zero-divergence-verification/04-CONTEXT.md` - locked decisions, mismatch policy, acceptance constraints
- `/.planning/REQUIREMENTS.md` - phase requirement IDs and exact completion semantics
- `/.planning/STATE.md` - Phase 3 outcomes and known caveats (self-consistent golden baseline)
- `crates/amlich-core/src/almanac/data.rs` - correction surfaces, invariants, metadata constraints
- `crates/amlich-core/src/almanac/truc.rs` - TRUC_QUALITY correction point
- `crates/amlich-core/src/almanac/xung_hop.rs` - formula-based correction point
- `crates/amlich-core/src/almanac/calc.rs` - JD epoch hook and shared day_fortune assembly
- `crates/amlich-core/src/almanac/golden_loader.rs` - typed citation fields and validation guarantees
- `crates/amlich-core/tests/khcbppt_*.rs` - divergence reporting behavior and subsystem granularity
- `crates/amlich-core/tests/almanac_golden.rs`, `crates/amlich-core/tests/ruleset_determinism.rs`, `crates/amlich-core/tests/taboo_boundary.rs` - required regression guardrails
- `docs/reference/khcbppt/*.md` - citation references for correction justification

### Secondary (MEDIUM confidence)
- `crates/amlich-core/data/almanac/khcbppt-golden.json` - current entry-level citation payload and metadata context

### Tertiary (LOW confidence)
- None

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - all required tooling/patterns are internal and already exercised in test runs
- Architecture: HIGH - correction surfaces and gate commands are explicit in code/tests/context
- Pitfalls: HIGH - directly derived from existing validation flow and locked phase policy

**Research date:** 2026-03-02
**Valid until:** 2026-04-02 (stable internal architecture; revisit if phase policy changes)
