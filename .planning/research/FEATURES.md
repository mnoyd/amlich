# Feature Research

**Domain:** Ten Gods (Thập Thần) + Tử Mệnh/Kua capabilities for Vietnamese almanac engine
**Researched:** 2026-03-02
**Confidence:** MEDIUM (high for Ten Gods engine patterns; medium/low for Kua convention standardization)

## Feature Landscape

### Table Stakes (Users Expect These)

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Deterministic Ten Gods API from day stem to target stem | Any production BaZi-style engine exposes stable Thập Thần labels from can-to-can relation | LOW | Already aligns with existing `get_thap_than(day_can, target_can)` pattern and 10x10 matrix tests. Keep output purely deterministic and typed. **Confidence: HIGH** |
| Complete Ten Gods label set and stable serialization | Integrators expect canonical 10 labels and machine-readable output | LOW | Table-stakes shape: label + relation + polarity + evidence. Keep enum serialization stable (snake_case) for API compatibility. **Confidence: HIGH** |
| Kua number computation with explicit gender handling | Kua/Tử Mệnh features are unusable without gender-aware formula path | MEDIUM | Provide strict input contract (`birth_year`, `gender`, `convention`). Ambiguity is in convention choice, not compute complexity. **Confidence: MEDIUM** |
| East/West group classification + 8-direction output | Production feng-shui calculators typically provide group and 4 favorable/4 unfavorable directions, not just raw number | MEDIUM | Include direction sets in typed structure to avoid downstream recomputation drift. **Confidence: MEDIUM** |
| Edge-case policy for year boundary | Production tools differ at year change; engines must be explicit to avoid silent disagreement | HIGH | Must define whether Kua uses Gregorian year vs lunar-year/Lập Xuân boundary. Reuse existing `tiet_khi` helper for boundary-aware mode. **Confidence: MEDIUM** |
| Backward-compatible DayFortune/API integration | Milestone goal is additive integration without regression | MEDIUM | New fields must be optional/default-safe; existing callers should parse unchanged payloads. **Confidence: HIGH** |

### Differentiators (Competitive Advantage)

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Convention-tagged Kua output (`convention`, `boundary_rule`) | Turns a historically ambiguous feature into auditable, reproducible engine behavior | MEDIUM | Return metadata alongside result so disagreements are traceable, not hidden. |
| Evidence-first outputs for both Ten Gods and Kua | Matches amlich’s KHCBPPT correctness posture and reduces future rework | LOW | Follow existing `RuleEvidence` shape; include source/method/profile for all new outputs. |
| Cross-check fixtures spanning 1900–2099 with boundary-heavy cases | Prevents false confidence from happy-path tests | HIGH | Include years near century transitions and dates near lunar/Lập Xuân boundaries. |
| Two-mode Kua support (single selected default + optional alternate mode) | Practical interoperability with external tools while preserving one canonical project default | HIGH | Only if needed by requirements; keep one default for DayFortune/API to avoid fragmentation. |

### Anti-Features (Commonly Requested, Often Problematic)

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| Human-language fate interpretation paragraphs | Users ask for “full reading” once Ten Gods/Kua exist | Non-deterministic, unverifiable, and outside v1.2 correctness scope | Return structured factors only; interpretation belongs to later separate milestone |
| Composite “fortune score” combining Ten Gods + Kua | Seems user-friendly for UI ranking | Pseudo-precision and heavy source ambiguity; likely causes trust regressions | Keep atomic outputs; let clients build scoring with explicit disclaimers |
| Silent auto-correction for missing/invalid gender or boundary context | Convenience | Hides assumptions and creates inconsistent results between clients | Fail fast with typed validation errors and required context |
| Expanding into Đại Vận in v1.2 | Natural next request | Explicitly deferred; introduces major scope and dependency expansion | Keep strict v1.2 boundary; plan Đại Vận in v1.3 |

## Feature Dependencies

```
[Existing HeavenlyStem + FiveElement + polarity model]
    └──requires──> [Deterministic Ten Gods mapping]
                        └──requires──> [Stable enum/JSON contract]
                                             └──requires──> [DayFortune/API additive fields]

[Birth year + gender inputs]
    └──requires──> [Kua number formula]
                        └──requires──> [Boundary convention decision]
                                             └──requires──> [Direction-set mapping + group]
                                                  └──requires──> [Integration/API exposure]

[Existing tiet_khi helper]
    └──enhances──> [Boundary-aware Kua mode]

[Existing KHCBPPT validator/evidence patterns]
    └──enhances──> [Ten Gods/Kua traceability + regression confidence]
```

### Dependency Notes

- **Ten Gods integration requires stable type contracts first:** integration before schema lock creates avoidable breaking changes.
- **Kua depends on convention decision before fixtures:** test fixtures are invalid until boundary/convention is frozen.
- **`tiet_khi` helper should be reused, not reimplemented:** avoids duplicate boundary logic and drift.
- **DayFortune integration should be last in this milestone chain:** keeps compute-core validation separate from API-shape churn.

## MVP Definition (for this milestone)

### Launch With (v1.2)

- [x] Deterministic Ten Gods core API + full 10x10 validation (TT-01..TT-05)
- [ ] Kua typed compute result (number, group, directions) with fixed documented convention (TM-01..TM-03, TM-05)
- [ ] Representative fixtures and regression-safe integration into DayFortune/API/serialization (TM-04, INT-01..INT-06)

### Add After Validation (v1.2.x)

- [ ] Optional alternate Kua convention mode (only if real interoperability need appears)
- [ ] Expanded fixture corpus for cross-engine compatibility audits

### Future Consideration (v1.3+)

- [ ] Đại Vận and cycle projections using Ten Gods/Kua context
- [ ] Interpretation/reporting layer (kept strictly separate from deterministic core)

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
|---------|------------|---------------------|----------|
| Ten Gods deterministic compute + stable schema | HIGH | LOW | P1 |
| Kua compute + explicit convention policy | HIGH | MEDIUM | P1 |
| Kua direction/group mapping | HIGH | MEDIUM | P1 |
| DayFortune/API backward-compatible integration | HIGH | MEDIUM | P1 |
| Boundary-heavy fixture suite (1900–2099 representative) | HIGH | HIGH | P1 |
| Alternate convention mode | MEDIUM | HIGH | P2 |
| Interpretive output/report text | LOW (for correctness milestone) | HIGH | P3 |

## Recommended Milestone Boundaries

1. **Boundary A — Core deterministic engines (compute only):**
   - Include: Ten Gods (done), Kua formula + convention lock
   - Exclude: API wiring, presentation concerns
2. **Boundary B — Typed contracts and fixtures:**
   - Include: output structs/enums, evidence metadata, representative fixtures
   - Gate: all deterministic tests green before integration
3. **Boundary C — Additive integration:**
   - Include: DayFortune/API/serialization optional fields, backward-compat tests
   - Exclude: new interpretive/business logic

This boundarying minimizes regression risk and maps directly to existing v1.2 phase split (P1 Ten Gods, P2 Kua, P3 integration).

## Sources

- Internal milestone scope and requirements: `.planning/PROJECT.md`, `.planning/REQUIREMENTS-v1.2.md` (**HIGH**)
- Existing Ten Gods implementation and test contract: `crates/amlich-core/src/almanac/thap_than.rs` (**HIGH**)
- Existing DayFortune/public type constraints: `crates/amlich-core/src/almanac/types.rs` (**HIGH**)
- Ecosystem reference (production-style open-source engine exposing 十神 in EightChar APIs):
  - https://raw.githubusercontent.com/6tail/lunar-python/master/README_EN.md (**MEDIUM**)
  - https://raw.githubusercontent.com/6tail/lunar-python/master/lunar_python/EightChar.py (**MEDIUM**)
- Kua formula/direction ecosystem behavior (non-official practitioner source; use with caution):
  - https://www.fengshuied.com/kua-number (**LOW**)

---
*Feature research for: Ten Gods and Tu Menh/Kua milestone (v1.2)*
*Researched: 2026-03-02*
