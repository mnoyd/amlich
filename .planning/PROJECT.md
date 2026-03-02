# Amlich Almanac Correctness Audit

## Current State

The project has shipped two milestones:

- `v1.0` KHCBPPT alignment complete (full validator + zero-divergence audit cycle).
- `v1.1` Foundation extensions complete and accepted (Xung Hop extensions, Tang Can, Tiet Khi regression fix).

Canonical status and acceptance evidence are archived in milestone artifacts:

- `.planning/milestones/v1.1-ROADMAP.md`
- `.planning/milestones/v1.1-REQUIREMENTS.md`
- `.planning/milestones/v1.1-MILESTONE-AUDIT.md`

## Core Value

Every almanac subsystem in amlich must produce output that matches KHCBPPT for the 2020-2030 date range, with test-backed and traceable evidence.

## Next Milestone Goals

Current target is `v1.2` (Ten Gods and Kua Foundation):

1. Implement deterministic Thap Than mapping engine with complete matrix validation.
2. Implement Tu Menh/Kua typed calculations with representative fixture coverage.
3. Integrate new outputs into `DayFortune`/API/serialization without regressions.

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| KHCBPPT as sole reference | Most authoritative classical text for Vietnamese almanac | ✓ Confirmed in v1.0 |
| Golden dataset + validator harness | Enables repeatable, test-backed correctness verification | ✓ Confirmed in v1.0 |
| Verification artifact as status authority | Prevents roadmap/state drift from acceptance truth | ✓ Confirmed in v1.1.1 |
| Real term-boundary scan for nearest Tiet Khi | Removes synthetic approximation regressions and stabilizes signed distances | ✓ Confirmed in v1.1.2 |

<details>
<summary>Archived initialization snapshot (pre-v1.1)</summary>

- Original scope centered on foundational KHCBPPT alignment and baseline correctness audit.
- Original active checklist has been superseded by shipped v1.0/v1.1 milestone artifacts.

</details>

---
*Last updated: 2026-03-02 after v1.1 milestone completion*
