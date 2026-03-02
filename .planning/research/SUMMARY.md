# Project Research Summary

**Project:** Amlich Almanac Correctness Audit (v1.2 Ten Gods and Kua Foundation)
**Domain:** Deterministic Vietnamese almanac engine extension (Rust core + API/DTO integration)
**Researched:** 2026-03-02
**Confidence:** MEDIUM-HIGH

## Executive Summary

This milestone is a **correctness-first engine extension**, not a net-new product surface: Ten Gods (Thập Thần) and Tử Mệnh/Kua must be added as deterministic, typed outputs in `amlich-core`, then propagated additively through API/serialization without breaking existing consumers. The research strongly converges on an implementation style already proven in the codebase: pure rule modules, orchestrator-only wiring in `calc.rs`, optional nested contract fields, and fixture-backed regression gates.

The recommended approach is to keep the current Rust workspace stack (no new dependencies), freeze Kua convention decisions early (year boundary, Kua 5 resolution, gender handling), then implement through dependency-safe layering: type scaffolding → pure Kua engine + fixtures → orchestrator wiring → DTO conversion → compatibility/contract regression. This preserves KHCBPPT auditability while minimizing schema churn and integration drift.

The highest risks are semantic integration errors (correct formula but wrong target context), Kua convention ambiguity, and core/API schema drift. Mitigation is explicit provenance in outputs, convention metadata embedded in results/evidence, and mandatory full-suite cross-crate tests (`amlich-core` + `amlich-api`) with boundary-heavy fixture coverage (1900–2099 and year-cutover cases).

## Key Findings

### Recommended Stack

v1.2 should ship on the **existing workspace stack only**. The research found no capability gap requiring new crates; introducing extra dependencies would increase maintenance and regression risk for a correctness milestone.

**Core technologies:**
- **Rust workspace (`edition = 2021`)**: deterministic Ten Gods + Kua rule implementation — keeps logic in the established correctness-critical layer.
- **`serde`/`serde_json` (1.0)**: typed output and fixture/contract testing — aligns with existing DTO and golden-fixture patterns.
- **`chrono` (0.4, existing)**: boundary/date normalization support where needed — avoids adding alternative date stacks.

Critical version note: remain on current workspace baselines; prioritize compatibility between `amlich-core`, `amlich-api`, and `amlich-wasm` path dependencies via additive fields.

### Expected Features

**Must have (table stakes):**
- Deterministic Ten Gods API with full canonical labels and stable serialization.
- Kua compute with explicit gender handling and documented boundary convention.
- East/West grouping plus 8-direction favorable/unfavorable outputs.
- Backward-compatible DayFortune/API integration (optional/additive fields only).

**Should have (competitive):**
- Convention-tagged Kua outputs (`convention`, `boundary_rule`, evidence metadata) for auditability.
- Evidence-first payloads for both Ten Gods and Kua.
- Boundary-heavy fixture corpus (1900–2099, especially year-transition cases).

**Defer (v2+ / later):**
- Interpretive narrative/fate text and composite “fortune scores”.
- Đại Vận expansion (explicitly v1.3+ scope).
- Optional alternate Kua convention mode unless interoperability pressure appears.

### Architecture Approach

Architecture research is explicit and high-confidence: add a new pure `tu_menh.rs` module in `amlich-core`, extend `types.rs` with optional nested structures (`ten_gods`, `tu_menh`), wire computation only in `calc.rs`, and mirror fields in API `dto.rs` + `convert.rs` without recomputation. UI/CLI layers must treat new fields as optional and tolerate absence.

**Major components:**
1. **Core rule modules (`thap_than.rs`, `tu_menh.rs`)** — deterministic, independently testable computation.
2. **Orchestrator + types (`calc.rs`, `types.rs`)** — canonical composition boundary and additive schema evolution.
3. **API mapping (`dto.rs`, `convert.rs`) + contract tests** — stable external JSON surface and compatibility enforcement.

### Critical Pitfalls

1. **Wrong Ten Gods integration target despite correct core logic** — prevent via explicit provenance fields and integration fixtures asserting exact stem pairs.
2. **Kua convention drift (boundary/Kua-5/gender semantics)** — freeze one convention in docs/tests and include convention metadata in outputs.
3. **Birth-level Kua forced into date-only pipeline** — compute Kua only with explicit person inputs; otherwise keep field absent with clear reasoning.
4. **Core/API schema mismatch** — always update `types.rs` + `dto.rs` + `convert.rs` + contract tests in the same change.
5. **Coverage illusion (unit-green, integration-red)** — enforce full cross-crate regression gates, not module-only tests.

## Implications for Roadmap

Based on combined research, suggested phase structure:

### Phase 1: Contract & Convention Lock
**Rationale:** All downstream implementation depends on frozen semantics and stable types. Without this, fixtures and integration are unstable.
**Delivers:**
- Finalized type scaffolding (`DayTenGods`, `KuaResult`, optional fields in `DayFortune`/DTO).
- Explicit Ten Gods provenance model (what each relation is “against”).
- Frozen Kua convention policy (`year_basis`, `kua5_resolution`, `gender_encoding`).
**Addresses:** Table-stake schema stability, Kua edge-case policy, backward compatibility.
**Avoids:** Pitfalls 1, 2, and early schema drift.

### Phase 2: Deterministic Engines & Fixture Validation
**Rationale:** Pure compute and fixture correctness should be proven before touching integration surfaces.
**Delivers:**
- Kua engine implementation in `tu_menh.rs`.
- Representative 1900–2099 fixture suite including boundary-heavy cases.
- Evidence metadata compliance tests for Ten Gods/Kua outputs.
**Addresses:** P1 feature set for Kua compute/group/directions + evidence-first outputs.
**Avoids:** Pitfalls 2, 5, and 6.

### Phase 3: Additive Integration & Regression Hardening
**Rationale:** Integration last minimizes blast radius and keeps regressions observable.
**Delivers:**
- `calc.rs` conditional wiring for Ten Gods/Kua.
- API DTO/convert propagation and backward-compatible JSON behavior.
- End-to-end contract + golden parity + KHCBPPT non-regression gate.
**Addresses:** DayFortune/API integration and release-readiness.
**Avoids:** Pitfalls 3, 4, and 6.

### Phase Ordering Rationale

- Dependencies are strict: convention/type lock must precede fixtures; fixtures must precede integration.
- Architecture favors module purity and orchestrator-only composition, so compute-first then wiring is the lowest-risk path.
- This ordering directly neutralizes the highest-risk pitfalls (semantic drift, schema mismatch, and false test confidence).

### Research Flags

Phases likely needing deeper research during planning:
- **Phase 1:** Kua convention finalization (source ambiguity remains; must validate chosen convention and migration impact).
- **Phase 3:** Person-context API shape for Kua exposure (day-level vs dedicated profile input path needs explicit UX/API decision).

Phases with standard patterns (can usually skip extra research):
- **Phase 2:** Deterministic Rust rule implementation + fixture testing follows established project patterns.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | Directly validated against current workspace dependencies and existing test/tooling patterns. |
| Features | MEDIUM | Ten Gods expectations are clear; Kua standardization has real convention variance across sources. |
| Architecture | HIGH | Recommendations map cleanly to existing code boundaries and proven integration patterns. |
| Pitfalls | HIGH (integration), MEDIUM (domain convention) | Integration risks are concrete and codebase-specific; Kua convention risk depends on external authority choice. |

**Overall confidence:** MEDIUM-HIGH

### Gaps to Address

- **Authoritative Kua convention source-of-truth:** finalize and document one canonical convention before implementation freeze.
- **Kua exposure contract in date-centric API:** confirm whether Kua is returned only with explicit person input or via separate endpoint.
- **Ten Gods output granularity:** settle whether one aggregate field is sufficient or explicit per-target fields are required to prevent misuse.

## Sources

### Primary (HIGH confidence)
- `.planning/research/STACK.md` — workspace stack, dependency and test strategy.
- `.planning/research/ARCHITECTURE.md` — component boundaries, wiring patterns, build order.
- `.planning/research/PITFALLS.md` — integration failure modes and prevention controls.
- `.planning/PROJECT.md`, `.planning/REQUIREMENTS-v1.2.md`, `.planning/ROADMAP.md` — milestone scope and constraints.
- Code evidence referenced by research: `crates/amlich-core/src/almanac/*`, `crates/amlich-api/src/{dto.rs,convert.rs}`, contract/golden tests.

### Secondary (MEDIUM confidence)
- `.planning/research/FEATURES.md` internal feature synthesis with ecosystem comparisons.
- `lunar-python` EightChar references for production-style Ten Gods exposure patterns.

### Tertiary (LOW confidence)
- Practitioner Kua references (e.g., fengshui calculators) used only to surface convention ambiguity, not as normative authority.

---
*Research completed: 2026-03-02*
*Ready for roadmap: yes*
