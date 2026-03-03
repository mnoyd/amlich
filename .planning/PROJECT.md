# Amlich Almanac Correctness Audit

## Current State

The project has shipped five milestones:

- `v1.0` KHCBPPT alignment complete (full validator + zero-divergence audit cycle).
- `v1.1` Foundation extensions complete and accepted (Xung Hop extensions, Tang Can, Tiet Khi regression fix).
- `v1.2` Ten Gods and Kua Foundation complete (deterministic calculators, typed API, DayFortune integration).
- `v1.3` Dai Van Core complete (core algorithm, helper contracts, Kua analysis, synchronized verification).
- `v1.4` Lunar Engine Table Parity complete (hour-pillar parity, full 60-cycle parity, Na Am API contracts).

Canonical status and acceptance evidence are archived in milestone artifacts:

- `.planning/milestones/v1.1-ROADMAP.md`
- `.planning/milestones/v1.1-REQUIREMENTS.md`
- `.planning/milestones/v1.1-MILESTONE-AUDIT.md`
- `.planning/milestones/v1.2-ROADMAP.md`
- `.planning/milestones/v1.2-REQUIREMENTS.md`
- `.planning/milestones/v1.4-ROADMAP.md`
- `.planning/milestones/v1.4-REQUIREMENTS.md`
- `.planning/milestones/v1.4-MILESTONE-AUDIT.md`

## Core Value

Every almanac subsystem in amlich must produce output that matches KHCBPPT for the 2020-2030 date range, with test-backed and traceable evidence.

## Next Milestone Goals

- Define fresh milestone-scoped requirements in a new `.planning/REQUIREMENTS.md`.
- Prioritize next subsystem parity targets and dependency order in a new `.planning/ROADMAP.md`.
- Preserve deterministic behavior, evidence metadata coverage, and contract-level regression testing.
- Resolve any lingering documentation/traceability drift during next milestone setup.

## Current Focus

Milestone `v1.4` is archived and tagged. The project is ready to start next-milestone definition (`/gsd-new-milestone`).

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
| Hour pillar parity via fixed slot + seed group model | Stable deterministic mapping across 12 windows and day-stem groups | ✓ Confirmed in v1.4 |
| Sexagenary inversion via CRT-based formula | Correct roundtrip mapping between cycle index and stem-branch pairs | ✓ Confirmed in v1.4 |
| Na Am API contracts with typed deterministic errors | Stable schema and explicit invalid-input handling for pair/index lookups | ✓ Confirmed in v1.4 |

<details>
<summary>Archived initialization snapshot (pre-v1.1)</summary>

- Original scope centered on foundational KHCBPPT alignment and baseline correctness audit.
- Original active checklist has been superseded by shipped v1.0/v1.1 milestone artifacts.

</details>

<details>
<summary>Archived v1.4 milestone scope</summary>

- Goal: reach deterministic table-level parity for hour pillar and 60-cycle calculations, then expose Na Am API surfaces with evidence-backed outputs.
- Delivered features: hour pillar parity, full sexagenary 60-cycle parity, Na Am pair/index APIs, and contract validators.

</details>

---
*Last updated: 2026-03-04 after v1.4 milestone completion*
