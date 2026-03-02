# Amlich Almanac Correctness Audit

## Current State

The project has shipped three milestones:

- `v1.0` KHCBPPT alignment complete (full validator + zero-divergence audit cycle).
- `v1.1` Foundation extensions complete and accepted (Xung Hop extensions, Tang Can, Tiet Khi regression fix).
- `v1.2` Ten Gods and Kua Foundation complete (deterministic calculators, typed API, DayFortune integration).

Canonical status and acceptance evidence are archived in milestone artifacts:

- `.planning/milestones/v1.1-ROADMAP.md`
- `.planning/milestones/v1.1-REQUIREMENTS.md`
- `.planning/milestones/v1.1-MILESTONE-AUDIT.md`
- `.planning/milestones/v1.2-ROADMAP.md`
- `.planning/milestones/v1.2-REQUIREMENTS.md`

## Core Value

Every almanac subsystem in amlich must produce output that matches KHCBPPT for the 2020-2030 date range, with test-backed and traceable evidence.

## Current Milestone: None

**Last completed:** v1.2 Ten Gods and Kua Foundation (shipped 2026-03-02)

**Delivered features:**
- Deterministic Thap Than mapping engine with typed stems, 10x10 matrix validation, and evidence metadata.
- Typed Tu Menh/Kua calculator with solar year basis, East/West grouping, and comprehensive fixture coverage (1899-2100).
- DayFortune/API/serialization integration with optional Ten Gods and Kua fields and full backward compatibility.
- All 16 requirements satisfied (TT-01 through TT-05, TM-01 through TM-05, INT-01 through INT-06).

## Next Milestone Goals

Planning required for next milestone. Potential candidates:

1. `v1.3` Dai Van integration (period transitions, correlation with Ten Gods/Kua, API contracts).
2. Extended validation (different date ranges, edge cases).
3. Star rule completeness (fill contextual buckets in baseline.json).
4. Performance optimization.

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| KHCBPPT as sole reference | Most authoritative classical text for Vietnamese almanac | ✓ Confirmed in v1.0 |
| Golden dataset + validator harness | Enables repeatable, test-backed correctness verification | ✓ Confirmed in v1.0 |
| Verification artifact as status authority | Prevents roadmap/state drift from acceptance truth | ✓ Confirmed in v1.1.1 |
| Real term-boundary scan for nearest Tiet Khi | Removes synthetic approximation regressions and stabilizes signed distances | ✓ Confirmed in v1.1.2 |
| Ten Gods mapping via five-element + polarity | Explicit mapping table over arithmetic shortcuts for audit readability | ✓ Confirmed in v1.2 |
| Kua calculator solar year basis | Vietnamese feng-shui convention using Gregorian calendar | ✓ Confirmed in v1.2 |
| Kua 5 resolution (male→8, female→2) | Frozen project policy for consistent output | ✓ Confirmed in v1.2 |
| Additive-only integration changes | Preserve backward compatibility while extending outputs | ✓ Confirmed in v1.2 |

<details>
<summary>Archived initialization snapshot (pre-v1.1)</summary>

- Original scope centered on foundational KHCBPPT alignment and baseline correctness audit.
- Original active checklist has been superseded by shipped v1.0/v1.1 milestone artifacts.

</details>

---
*Last updated: 2026-03-03 after v1.2 milestone completion*
