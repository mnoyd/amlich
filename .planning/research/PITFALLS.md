# Pitfalls Research

**Domain:** Ten Gods + Tu Menh/Kua integration into KHCBPPT-verified deterministic almanac engine
**Researched:** 2026-03-02
**Confidence:** HIGH (integration/codebase pitfalls), MEDIUM (Kua convention/source ambiguity)

## Critical Pitfalls

### Pitfall 1: Correct Ten Gods core logic, wrong integration target in DayFortune

**What goes wrong:**
`get_thap_than(day_can, target_can)` is correct in isolation (already matrix-tested), but integration computes Ten Gods against the wrong target stem (e.g., year/month/day branch-derived stem confusion), so output is deterministic but semantically wrong.

**Why it happens:**
v1.2-01 delivered a standalone engine. v1.2-03 introduces orchestration decisions not yet encoded in type contracts (which stem(s) are authoritative for day-level output).

**How to avoid (concrete checks):**
- Introduce explicit field names in core/API types (`ten_gods_vs_year_stem`, `ten_gods_vs_month_stem`, etc.) instead of one ambiguous `ten_gods` blob.
- Add integration fixtures asserting exact input pair used for each Ten Gods field.
- Add one test that fails if day stem is accidentally compared to itself (common false-positive path yielding TyKien/KiepTai-heavy output).

**Warning signs:**
- Ten Gods distribution is implausibly concentrated in `ty_kien`/`kiep_tai` across unrelated dates.
- API payload contains Ten Gods values but no clear provenance of compared stem.

**Phase to address:**
**v1.2-03 (INT-01, INT-03, INT-05)**

---

### Pitfall 2: Convention drift for Kua algorithm (year boundary + Kua 5 handling)

**What goes wrong:**
Kua results differ by source because convention choices are not frozen (solar year vs lunar year cutover, handling remainder=0, male/female handling of Kua 5).

**Why it happens:**
Requirements call for “established Vietnamese/Asian conventions,” but multiple conventions exist and current project docs include formula examples that are not yet codified as milestone-level source-of-truth.

**How to avoid (concrete checks):**
- Add a `KuaConvention` metadata block in result/evidence (`year_basis`, `gender_encoding`, `kua5_resolution`, `source_id`).
- Freeze one convention in TM-05 docs before implementation completion.
- Add edge fixtures specifically for: year ending 00, remainder 0, and dates near lunar new year boundary.
- Reject silent fallback for unsupported gender input; return typed error.

**Warning signs:**
- Same birth year returns different Kua between unit tests and API fixtures.
- Kua 5 appears directly in output (instead of resolved rule) or panic-based handling appears.

**Phase to address:**
**v1.2-02 (TM-01, TM-02, TM-05)**

---

### Pitfall 3: Birth-level Kua forced into day-level pipeline without required inputs

**What goes wrong:**
`calculate_day_fortune()` currently has only day/date context; Kua needs birth year + gender. If forced into day-level path, developers may inject placeholders, derive from query date incorrectly, or make fields always-null without contract clarity.

**Why it happens:**
INT requirements ask Kua exposure in API while existing `DateQuery` and `DayInfo` are date-centric, not person-centric.

**How to avoid (concrete checks):**
- Keep Kua calculation as separate typed API (e.g., `get_kua_info(birth_year, gender, optional_birth_date_context)`) and reference from day response only when explicit person input exists.
- If embedding in `DayFortune`, make field `Option` plus a `not_computed_reason` contract.
- Add contract tests for both paths: without person input (field absent/None) and with person input (field present + validated).

**Warning signs:**
- Kua appears in day responses for anonymous date-only requests.
- Integration code uses query year as birth year.

**Phase to address:**
**v1.2-03 (INT-02, INT-03, INT-05)**

---

### Pitfall 4: Schema mismatch between core and API DTO layers

**What goes wrong:**
Core `DayFortune` adds fields, but `amlich-api` DTO/convert layer is not updated consistently (`dto.rs` + `convert.rs`), causing silent field drops, breaking clients, or inconsistent JSON shape.

**Why it happens:**
Current architecture duplicates type surfaces (core structs + API DTO structs + conversion impls). Every new field requires 3 synchronized edits plus tests.

**How to avoid (concrete checks):**
- Add a compile-failing test or lint-like assertion pattern around DTO conversion completeness (snapshot JSON contract test minimum).
- Extend `almanac_contract.rs` with explicit assertions for new Ten Gods and Kua fields.
- Keep new fields additive and optional first; no renames/removals in v1.2.

**Warning signs:**
- Field exists in `amlich-core` serialized JSON but missing from API response.
- PR updates `types.rs` without matching `dto.rs` + `convert.rs` changes.

**Phase to address:**
**v1.2-03 (INT-01, INT-02, INT-04, INT-05)**

---

### Pitfall 5: Evidence metadata inconsistency breaks auditability standard

**What goes wrong:**
New Ten Gods/Kua outputs return values without consistent `RuleEvidence` conventions (source/method/profile), weakening the project’s core “traceable correctness” contract.

**Why it happens:**
Ten Gods engine currently uses hardcoded evidence (`khcbppt`, `five-element-polarity-matrix`), while Kua may be algorithmic and from non-KHCBPPT sources; without explicit policy, metadata becomes ad-hoc.

**How to avoid (concrete checks):**
- Define v1.2 evidence policy document: when to use `khcbppt` vs algorithmic source IDs.
- Add tests requiring non-empty and expected evidence fields for every new output path.
- Prohibit placeholder evidence (`source_id: "unknown"`, empty method).

**Warning signs:**
- Mixed or contradictory source IDs for same feature across tests.
- Evidence exists in core type but is dropped in DTO.

**Phase to address:**
**v1.2-02 (TM-05) and v1.2-03 (INT-04, INT-05)**

---

### Pitfall 6: Test coverage illusion (unit-green, integration-red after merge)

**What goes wrong:**
Feature modules pass local unit tests, but cross-crate behavior regresses (core/API contract, golden parity, existing KHCBPPT validators).

**Why it happens:**
This codebase uses layered tests; adding fields can pass module tests while failing API and golden parity paths.

**How to avoid (concrete checks):**
- Mandatory gate for v1.2 plans: run
  - `cargo test --package amlich-core`
  - `cargo test --package amlich-api`
  - KHCBPPT-focused suites
- Add representative 1900–2099 Kua fixtures and verify serialization round-trip.
- Add regression test that existing JSON consumers still parse when new optional fields are absent.

**Warning signs:**
- Only new module tests updated in PR.
- Existing parity/contract tests are ignored or quarantined.

**Phase to address:**
**v1.2-02 (TM-04) and v1.2-03 (INT-05, INT-06)**

---

## Technical Debt Patterns

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Put Kua inside `DayFortune` without person-input contract | Faster integration | Semantically wrong API and future breaking change | Never |
| Encode gender as loose `String` | Quick parsing | Inconsistent values across layers | Never (use enum + parser) |
| Omit convention metadata for Kua | Smaller payload | Cannot explain divergences later | Never |
| Add one generic `ten_gods` field with unclear target | Less schema work | Ambiguous semantics and client misuse | Never |

## Integration Gotchas

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| Core → API DTO | Add core fields only | Update `types.rs` + `dto.rs` + `convert.rs` + contract tests in one change |
| Day query → Kua calc | Reuse query year as birth year | Require explicit birth inputs for Kua endpoint/field |
| Evidence propagation | Keep evidence in core only | Assert evidence appears in API JSON for new fields |

## Performance Traps

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Recomputing Ten Gods repeatedly in serialization path | Unnecessary allocations/CPU in API layer | Compute once in core and pass typed result through | High-volume API/date-range batch queries |
| Kua fixtures generated ad hoc per test | Slow/flaky tests | Use fixed fixture set with deterministic expected outputs | CI parallel test load |

## Security Mistakes

| Mistake | Risk | Prevention |
|---------|------|------------|
| Accepting arbitrary gender string and defaulting silently | Data integrity errors in personal astrology output | Strict enum parsing + explicit error |
| Panic on invalid Kua edge case | Service instability if exposed through API | Return typed error/result, no panic paths |

## UX Pitfalls

| Pitfall | User Impact | Better Approach |
|---------|-------------|-----------------|
| Showing Kua in date-only response without user profile | Users trust incorrect personalized result | Separate “day data” from “person profile” outputs |
| Exposing Ten Gods labels without compared-target context | Misinterpretation by downstream consumers | Include `against` metadata (year/month/day stem target) |

## "Looks Done But Isn't" Checklist

- [ ] **Ten Gods integration:** Labels present but target-stem provenance missing — verify explicit field naming/tests.
- [ ] **Kua algorithm:** Formula implemented but convention metadata absent — verify TM-05 documentation and evidence fields.
- [ ] **API schema:** Core types updated but DTO conversion incomplete — verify `almanac_contract` assertions for new fields.
- [ ] **Regression safety:** New tests pass but KHCBPPT suites not re-run — verify INT-06 full regression gate.

## Recovery Strategies

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| Wrong Ten Gods integration target | MEDIUM | Freeze schema, add provenance fields, regenerate fixtures, backfill contract tests |
| Kua convention mismatch after release | HIGH | Version convention metadata, add compatibility mode, publish migration notes |
| Core/API schema drift | MEDIUM | Patch DTO/convert parity, add snapshot tests to prevent recurrence |

## Pitfall-to-Phase Mapping

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| Wrong Ten Gods target in integration | v1.2-03 | Contract tests assert exact day/target stem pairing for each output field |
| Kua convention drift | v1.2-02 | Edge fixture suite (1900–2099 + boundary cases) passes with documented convention metadata |
| Birth-level Kua miswired into day-level flow | v1.2-03 | Date-only requests return no Kua; person-context requests return validated Kua |
| Core/API schema mismatch | v1.2-03 | JSON contract tests include new fields and remain backward-compatible |
| Evidence metadata inconsistency | v1.2-02 + v1.2-03 | Tests enforce non-empty, policy-aligned evidence in both core and API |
| Coverage illusion / regression leakage | v1.2-02 + v1.2-03 | Full `amlich-core` + `amlich-api` suites green, including KHCBPPT validators |

## Sources

- `.planning/PROJECT.md` (v1.2 goals and constraints)
- `.planning/STATE.md` (integration risk note and current milestone context)
- `.planning/REQUIREMENTS-v1.2.md` (TT/TM/INT requirement boundaries)
- `crates/amlich-core/src/almanac/thap_than.rs` (existing deterministic Ten Gods engine + tests)
- `crates/amlich-core/src/almanac/types.rs` (DayFortune shape and evidence patterns)
- `crates/amlich-core/src/almanac/calc.rs` (current orchestration integration point)
- `crates/amlich-api/src/dto.rs` and `crates/amlich-api/src/convert.rs` (DTO duplication risk)
- `crates/amlich-api/tests/almanac_contract.rs` and `crates/amlich-api/tests/golden_parity.rs` (contract/regression gates)
- `.planning/phases/v1.2-ten-gods-and-kua-foundation/v1.2-RESEARCH.md` (phase assumptions and risks)
- `vietnamese_lunar_engine_tables.md` and `.planning/ENGINE_EXPANSION_ANALYSIS.md` (LOWER-authority formula references; used to identify convention ambiguity)

---
*Pitfalls research: Ten Gods + Tu Menh/Kua integration (v1.2)*
