# Project Research Summary

**Project:** amlich v1.3 - Dai Van Core
**Domain:** Vietnamese Almanac (Đại Vân/10-Year Major Luck Cycles)
**Researched:** 2026-03-03
**Confidence:** HIGH

## Executive Summary

Dai Van (Đại Vân/大運) is a deterministic 10-year luck cycle calculation system in Vietnamese Bazi astrology that projects life phases from birth date and gender. The integration follows the same additive-only pattern used successfully for Ten Gods and Kua in v1.2: create a pure calculation module (`dai_van.rs`), expose results through an optional `dai_van` field in `DayFortune`, and reuse existing modules (canchi, tietkhi, thap_than, tu_menh) without modification. No new dependencies are needed—all required capabilities exist in the workspace (Rust stdlib, serde, chrono).

The recommended approach uses standard Bazi formulas: 8-pillar generation from month Can Chi with chieuthu direction (forward/backward based on year polarity × gender), start age calculated as |days_to_nearest_tiết_khi| / 3, and optional Ten Gods correlation per pillar (pillar Can → birth day Can). Key risks include off-by-one errors in age ranges, incorrect Ten Gods target stem (must use birth day stem, not pillar Can), and potential KHCBPPT source verification gaps. Mitigation: comprehensive golden fixtures with edge cases (Tiết Khí boundaries, leap months, Kua 5), property-based tests for invariants (contiguous age ranges), and transparent documentation of source uncertainty.

## Key Findings

### Recommended Stack

No new dependencies required. All capabilities exist in the existing Rust workspace.

**Core technologies:**
- **Rust workspace (edition 2021)** — Deterministic Dai Van calculation engine; no FFI/language boundary needed for correctness-critical milestone
- **serde (workspace)** — Serialize/deserialize Dai Van types and evidence metadata; follows existing DTO/JSON contract patterns
- **chrono (workspace)** — Birth date handling and Tiết Khí distance calculation; date conversion and day difference logic needed for start age
- **Existing modules** — canchi (year/month Can Chi), tietkhi (days to nearest solar term), thap_than (Ten Gods), tu_menh (Kua calculator)

**Why no new dependencies:** All needed calculation logic is pure deterministic algorithms (stem/branch progression, chieuthu matrix, age range calculations). Database not needed—Dai Van computed on-demand from birth inputs. Infrastructure unchanged—use existing module structure.

### Expected Features

**Must have (table stakes):**
- 8-pillar Dai Van calculation with Chieuthu (Thuận/Nghịch) direction determination
- Start age calculation from Tiết Khí distance (3 days = 1 year conversion)
- Pillar generation with contiguous 10-year age ranges
- Optional Ten Gods correlation per pillar (lazy, on-demand)
- Backward-compatible DayFortune/API integration (optional field with skip_serializing_if)
- Convention metadata with evidence traceability (source_id, method, year_basis, etc.)

**Should have (differentiators):**
- Kua-based directional analysis per pillar (links 10-year cycles with feng-shui directions)
- Helper functions: get_current_pillar(), years_to_next_transition(), get_pillar_at_age()
- Current pillar identification for any age
- Convention-tagged metadata (auditable calculation assumptions)

**Defer (v2+):**
- Tiểu Vận (yearly/decadal luck) — different calculation rules, unclear KHCBPPT coverage
- Human-language fortune interpretation paragraphs — non-deterministic, outside correctness scope
- Separate public API for birth fortune — optional field approach keeps API minimal
- Composite "fortune score" — pseudo-precision, heavy source ambiguity

### Architecture Approach

Dai Van introduces a new computation pathway for birth-based context that integrates cleanly into the existing DayFortune architecture using the established optional field pattern from v1.2. The architecture follows four key patterns: (1) Optional field additive integration (no breaking changes to DayFortune consumers), (2) Pure calculation modules (deterministic, testable, no side effects), (3) Evidence metadata for traceability (source_id, method, convention fields), and (4) Module-level reuse without modification (dai_van calls thap_than::get_thap_than() and tu_menh::compute_kua() directly).

**Major components:**
1. **dai_van.rs (NEW)** — Core Dai Van calculation logic (6-step: lunar conversion → year/month Can Chi → chieuthu → start age → 8 pillars → Ten Gods correlation). 400-600 LOC estimated.
2. **calc.rs (MODIFIED)** — Orchestrate all calculations including Dai Van; add optional birth_date, birth_year, gender parameters; conditionally call dai_van::calculate when inputs provided.
3. **types.rs (MODIFIED)** — Add dai_van optional field to DayFortune with #[serde(skip_serializing_if = "Option::is_none")]; export new types.
4. **API Layer (MODIFIED)** — Add DaYunResultDto and DaYunPillarDto in dto.rs; implement From<> conversion traits; update exports and contract tests.

**Key architectural insight:** Dai Van requires birth date + gender (semantically distinct from day-based almanac). Create separate calculation pathway that optionally populates DayFortune when birth context provided, rather than modifying core day-fortune calculation logic.

### Critical Pitfalls

**1. Period transition boundary errors** — Off-by-one errors in age range calculations cause wrong pillar assignments. Avoid: use start_age inclusive, end_age exclusive bounds; correct modulo arithmetic for Nghich direction (-1); verify age ranges contiguous with property-based tests. (Phase 1)

**2. Ten Gods correlation uses wrong stem** — Must use birth day stem → pillar Can, not pillar Can → pillar Can (always ty_kien) or query day stem → pillar Can (dynamic, not birth-based). Avoid: explicit field naming (`ten_gods_vs_day_stem`); integration fixtures assert correct target stem. (Phase 2)

**3. KHCBPPT source verification gap** — Dai Van KHCBPPT coverage uncertain (no explicit section found in online search). Avoid: use standard Bazi formulas from vietnamese_lunar_engine_tables.md Section 15 as primary source; document source_id as "khcbppt" placeholder with TODO comment; create tracking issue for manual KHCBPPT verification. (Phase 1)

**4. Start age calculation uses wrong Tiết Khí** — Must use nearest (previous or next), signed distance, lunar month/year for lookup, correct 3-days-per-year conversion. Avoid: edge case fixtures for births on/within ±1, ±2, ±5, ±10, ±30 days of Tiết Khí; document rounding convention explicitly. (Phase 1)

**5. Chiều rule matrix errors** — (Year Yang/Âm × Gender) → Thuận/Nghịch counterintuitive matrix. Avoid: unit tests for all 4 combinations; document Yang Chi indices (Tý, Dần, Thìn, Ngọ, Thân, Tuất = even indices 0,2,4,6,8,10). (Phase 1)

**6. Backward compatibility broken** — Adding birth inputs as required fields breaks existing calculate_day_fortune() callers. Avoid: keep existing function unchanged OR add optional parameters with backward-compatible defaults; contract tests assert old API still works. (Phase 3)

**7. Determinism violations** — Using Utc::now() as default or floating-point without rounding causes non-deterministic results. Avoid: require explicit reference date; integer arithmetic with documented rounding convention; determinism tests (run 1000x with same inputs). (Phase 1)

**8. Core/API schema mismatch** — Core type updated but DTO/convert layer missing fields. Avoid: update core+DTO+convert in one PR; add conversion completeness test; warning: field exists in core JSON but missing from API JSON. (Phase 3)

## Implications for Roadmap

Based on combined research, suggested 3-phase implementation matching FEATURES.md recommendation:

### Phase 1: Core Dai Van Module (Weeks 1-2)
**Rationale:** Implements deterministic calculation logic in isolation before connecting to other systems. This is the foundation that all other phases depend on. Avoids period transition boundary errors, chieuthu matrix errors, start age Tiết Khí errors, and determinism violations.

**Delivers:** dai_van.rs module (400-600 LOC) with core types (Gender, ChieuThu, DaYunPillar, DaYunResult), 6-step calculation algorithm, chieuthu rule matrix, start age from Tiết Khí, 8-pillar generation, unit tests for all calculation steps.

**Addresses:** 8-pillar Dai Van calculation (P1), Chieuthu direction determination (P1), Start age calculation (P1), Pillar generation (P1), Convention metadata (P1).

**Avoids:** Period transition boundary errors (Pitfall 1), Chiều rule matrix errors (Pitfall 6), Start age Tiết Khí errors (Pitfall 5), Determinism violations (Pitfall 8).

**Stack elements:** Rust stdlib, chrono (dates), serde (serialization), existing modules (canchi, tietkhi, lunar, julian).

**Architecture components:** dai_van.rs (pure calculation module).

**Research flags:** LOW research risk — standard Bazi formulas, well-documented algorithms, existing v1.2 integration patterns to follow. No `/gsd-research-phase` needed.

### Phase 2: Ten Gods Integration and Helpers (Weeks 3-4)
**Rationale:** Connects Dai Van with existing Ten Gods engine once core calculation is verified correct. Leverages proven v1.2 thap_than module without modification. Avoids Ten Gods wrong target stem errors.

**Delivers:** Lazy Ten Gods correlation per pillar (`get_ten_gods_for_pillar(thu_tu, day_stem)`), helper functions (`get_current_pillar`, `years_to_next_transition`, `get_pillar_at_age`), integration tests between dai_van and thap_than modules, comprehensive golden fixtures (15+ edge cases), property-based tests for invariants.

**Addresses:** Ten Gods correlation per pillar (P2), Helper functions (P2), Testing coverage gaps (Pitfall 11).

**Uses:** thap_than::get_thap_than() (existing, no modification), dai_van.rs core calculation.

**Implements:** Architecture pattern "Module-level reuse without modification" — dai_van calls thap_than directly.

**Avoids:** Ten Gods wrong target stem errors (Pitfall 2), Testing gaps (Pitfall 11).

**Research flags:** LOW research risk — existing ThapThan module proven in v1.2, integration pattern clear. No `/gsd-research-phase` needed.

### Phase 3: API Integration and Kua Analysis (Weeks 5-6)
**Rationale:** Last to integrate to ensure no breaking changes until everything works internally. Adds public API exposure and optional Kua directional analysis. Ensures zero regressions in existing v1.2 features.

**Delivers:** Backward-compatible DayFortune integration (optional dai_van field with skip_serializing_if), calculate_day_fortune() signature update (add birth_date, birth_year, gender as optional), DaYunResultDto and DaYunPillarDto in API layer, From<> conversion implementations, Kua-based directional analysis per pillar (optional), convention metadata documentation, full regression suite including all subsystems, backward compatibility contract tests.

**Addresses:** Backward-compatible DayFortune/API integration (P1), Kua-based directional analysis per pillar (P3), Convention metadata (P1).

**Uses:** tu_menh::compute_kua() (existing, no modification), calc.rs orchestrator, API layer types.

**Implements:** Architecture pattern "Optional field additive integration" and "Evidence metadata for traceability".

**Avoids:** Backward compatibility breaks (Pitfall 7), Core/API schema mismatch (Pitfall 9), Kua integration mismatched (Pitfall 3).

**Research flags:** MEDIUM research risk — API design decision (optional field vs. separate API), breaking change to calculate_day_fortune() signature requires coordinated update of ~5-10 call sites. Consider `/gsd-research-phase` for API design validation.

### Phase Ordering Rationale

- **Core calculation first → ensures algorithm is correct** before connecting to other systems. Errors in chieuthu, age ranges, or start age propagate to all integration points.
- **Ten Gods second → builds on working core**, adds deterministic integration with proven v1.2 module. Most users only need current pillar; lazy calculation is more efficient.
- **API/Kua third → last to integrate**, ensures no breaking changes until everything works. Kua integration is birth-level, not pillar-level (computed once per person, applied to all pillars).

**Why this grouping based on architecture patterns:**
- Phase 1 implements pure calculation module pattern (dai_van.rs isolated)
- Phase 2 implements module-level reuse pattern (calls thap_than without modification)
- Phase 3 implements optional field additive integration pattern (DayFortune optional field)

**How this avoids pitfalls:**
- Phase 1: Adds age range validation, chieuthu matrix tests, determinism tests, edge case fixtures
- Phase 2: Adds explicit Ten Gods field naming, integration fixtures asserting correct target stem
- Phase 3: Adds backward compatibility tests, DTO conversion completeness test, Kua convention compliance

### Research Flags

**Phases likely needing deeper research during planning:**
- **Phase 3:** API design decision (optional field vs. separate API) — MEDIUM risk. Breaking change to calculate_day_fortune() signature requires coordinated update. Consider running `/gsd-research-phase` to validate API design and backward compatibility strategy.

**Phases with standard patterns (skip research-phase):**
- **Phase 1:** Core Dai Van calculation — standard Bazi formulas, well-documented algorithms, existing v1.2 integration patterns to follow. LOW research risk.
- **Phase 2:** Ten Gods integration — existing ThapThan module proven in v1.2, integration pattern clear. LOW research risk.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | All dependencies exist in workspace; no new infrastructure needed; pure deterministic calculation well-understood. |
| Features | HIGH | Core features (8-pillar, chieuthu, start age, pillar generation) clearly defined with high confidence from DAI_VAN_RESEARCH.md. KHCBPPT verification gap noted but mitigated with placeholder source_id and tracking issue. |
| Architecture | HIGH | Clear analysis of existing DayFortune patterns from v1.2; detailed component specification from ARCHITECTURE.md; proven additive integration approach. Breaking change to calculate_day_fortune() identified and mitigated. |
| Pitfalls | HIGH | 11 detailed pitfalls with concrete examples, prevention strategies, and phase assignments. Integration gotchas, performance traps, and anti-patterns documented. KHCBPPT uncertainty acknowledged. |

**Overall confidence:** HIGH

### Gaps to Address

- **KHCBPPT source verification gap:** Dai Van KHCBPPT coverage uncertain (no explicit section found in online search). Mitigation: use standard Bazi formulas from vietnamese_lunar_engine_tables.md Section 15 as primary source; document source_id as "khcbppt" placeholder with TODO comment; create tracking issue for manual KHCBPPT verification during or after v1.3 implementation.

- **Start age rounding convention:** Different schools may round differently (truncate vs. nearest integer) for days-to-years conversion. Mitigation: pick one convention (truncate/floor) and document explicitly in ConventionMetadata and code comments; add edge case fixtures showing chosen convention.

- **Ten Gods birth hour dependency:** Ten Gods correlation requires birth hour for complete day stem extraction. Mitigation: support unknown birth hour gracefully (ten_gods = None or day_fortune-based targets); document limitation in API docs.

These gaps are manageable and do not block v1.3 implementation. Mitigation strategies are clear and documented.

## Sources

### Primary (HIGH confidence)
- **.planning/research/DAI_VAN_RESEARCH.md** — Comprehensive Dai Van calculation formulas, 6-step algorithm, types, and integration approach
- **.planning/research/STACK.md** — Detailed Rust code templates and data structures for dai_van.rs, integration points with existing systems
- **.planning/research/FEATURES.md** — Table stakes, differentiators, anti-features, MVP definition, recommended 3-phase structure
- **.planning/research/ARCHITECTURE.md** — Component responsibilities, data flow changes, integration patterns, build order
- **.planning/research/PITFALLS.md** — 11 detailed pitfalls with concrete examples and prevention strategies
- **crates/amlich-core/src/almanac/** — Existing implementation analysis (types.rs, calc.rs, thap_than.rs, tu_menh.rs, canchi.rs, tietkhi.rs)
- **v1.2 integration patterns** — Ten Gods and Kua additive integration proven in v1.2
- **Cargo.toml (workspace)** — Existing dependencies: serde 1.0, serde_json 1.0, chrono 0.4

### Secondary (MEDIUM confidence)
- **vietnamese_lunar_engine_tables.md Section 15** — Dai Van calculation formulas and code templates (used as primary source until KHCBPPT verified)
- **Wikipedia "Four Pillars of Destiny"** — Mentions 10-year luck cycle and shows example with 8 pillars
- **Wikipedia "Sexagenary cycle"** — Explains Can Chi system of 10 Heavenly Stems + 12 Earthly Branches

### Tertiary (LOW confidence)
- **KHCBPPT classical reference: 卷六 (Volume 6) or Quyển 12-13** — Cited in DAI_VAN_RESEARCH.md but requires manual lookup to verify exact calculation rules. Placeholder source_id used with TODO comment.
- **Modern Vietnamese numerology sites** — Often have simplified explanations but should be cross-checked against classical sources

---
*Research completed: 2026-03-03*
*Ready for roadmap: yes*
