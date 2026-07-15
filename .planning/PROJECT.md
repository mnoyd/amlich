# Amlich Almanac Correctness Audit

## Current State

The project has shipped six milestones:

- `v1.0` KHCBPPT alignment complete (full validator + zero-divergence audit cycle).
- `v1.1` Foundation extensions complete and accepted (Xung Hop extensions, Tang Can, Tiet Khi regression fix).
- `v1.2` Ten Gods and Kua Foundation complete (deterministic calculators, typed API, DayFortune integration).
- `v1.3` Dai Van Core complete (core algorithm, helper contracts, Kua analysis, synchronized verification).
- `v1.4` Lunar Engine Table Parity complete (hour-pillar parity, full 60-cycle parity, Na Am API contracts).
- `v1.5` Eastern Knowledge Expansion complete (Văn khấn `vn-folk-ritual` corpus + lookup APIs; Phi Tinh `huyen-khong` Vận/Niên/Nguyệt overlays + 81-cell aspects + safety hints; additive `DaySnapshot` integration; 886 tests pass).

Canonical status and acceptance evidence are archived in milestone artifacts:

- `.planning/milestones/v1.1-ROADMAP.md`
- `.planning/milestones/v1.1-REQUIREMENTS.md`
- `.planning/milestones/v1.1-MILESTONE-AUDIT.md`
- `.planning/milestones/v1.2-ROADMAP.md`
- `.planning/milestones/v1.2-REQUIREMENTS.md`
- `.planning/milestones/v1.4-ROADMAP.md`
- `.planning/milestones/v1.4-REQUIREMENTS.md`
- `.planning/milestones/v1.4-MILESTONE-AUDIT.md`
- `.planning/milestones/v1.5-ROADMAP.md`
- `.planning/milestones/v1.5-REQUIREMENTS.md`
- `.planning/milestones/v1.5-MILESTONE-AUDIT.md`

## Core Value

Every almanac subsystem in amlich must produce output that matches its canonical classical source (KHCBPPT for the original engine; `vn-folk-ritual` for ritual text; *Thẩm Thị Huyền Không Học* for Phi Tinh) for the 2020-2030 date range, with test-backed and traceable evidence. v1.5 expanded "canonical source" from a single text to a registered taxonomy of source_ids, each enforced by module-level `pub const` and CI grep guards.

## Validated Capabilities (after v1.5)

> v1.6 Eastern Knowledge Completion initialised; capabilities below are the post-v1.5 baseline. v1.6 will add Daily Flying Star + `RecommendsOffering` node + close review/confidence debt.

- ✓ KHCBPPT-aligned core calendar (v1.0–v1.1)
- ✓ Ten Gods + Kua + Dai Van calculators (v1.2–v1.3)
- ✓ Hour pillar + 60-cycle + Na Am parity (v1.4)
- ✓ Văn khấn corpus + lookup APIs (v1.5, `vn-folk-ritual`)
- ✓ Phi Tinh Vận/Niên/Nguyệt overlays + 81-cell aspects + safety hints (v1.5, `huyen-khong`)
- ✓ Semantic graph wiring with dual-provenance Direction node and additive `DaySnapshot` integration (v1.5)

## Out of Scope (carry-forward)

- **P2 Kinh Dịch, P3 Y học, P6 Tử Vi** — deferred per Expansion Framework tiering.
- **P5 Spatial Phi Tinh / `spatial_compose`** — requires user spatial input (sit/face direction); explicit CRIT-3 isolation forbids wiring `FlyingStar` into `interaction/direction_merge.rs`.

## Current Milestone: v1.6 Eastern Knowledge Completion

**Goal:** Round out the Eastern Knowledge pillar by adding the deferred daily Phi Tinh layer, promoting `RecommendsOffering` to a first-class semantic-graph node, and closing the v1.5 review/confidence tech debt.

**Target features:**
- **Daily Flying Star (日紫白)** — per-day Phi Tinh overlay with 冬至/夏至 reversal, reusing v1.5 `huyen-khong` overlay + aspect machinery; new ADR for daily starting-star convention.
- **`RecommendsOffering` semantic-graph node** — promote offerings from flat string list inside `Ritual` payload to first-class node (per `research/ARCHITECTURE.md:263`).
- **RIT-11 reviewer field closure** — independent peer review for the 60 `reviewer: pending` ritual entries; resolution logged back into `provenance_audit.md`.
- **ADR-0003 pre-1984 confidence boost** — promote Thượng/Trung Nguyên polarity rows from MEDIUM to HIGH after external cross-check; resolve 1960 Trung Nguyén `KnownDivergence`.

## Current Focus

Milestone `v1.5` shipped 2026-05-28. Milestone `v1.6 Eastern Knowledge Completion` initialised 2026-07-15; goals above.

**Outstanding v1.5 tech debt (carry-forward → v1.6 closure):**
- RIT-11: `provenance_audit.md reviewer: pending` for all 60 ritual entries → v1.6 closure phase.
- ADR-0003: pre-1984 Thượng/Trung Nguyên polarity rows MEDIUM-confidence → v1.6 confidence-boost phase.

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
| Source-ID taxonomy as `pub const &str` (not enum) | New traditions register without enum churn; CI grep guard prevents bare-literal drift | ✓ Confirmed in v1.5 (DEC-0023) |
| Schema-lock before corpus authoring | Re-editing 60 corpus entries after a schema slip is prohibitively expensive (PITFALLS CRIT-1/5) | ✓ Confirmed in v1.5 (Phase 10 → 12 ordering) |
| ADR-0001: `RitualEntry` JSON schema v1 with `deny_unknown_fields` | Frozen 10-type schema gives corpus authors a stable target | ✓ Confirmed in v1.5 |
| ADR-0002: solar-term boundaries for monthly Phi Tinh | Reuses v1.1.2 Tiết Khí scanner per *Thẩm Thị Huyền Không Học* convention | ✓ Confirmed in v1.5 |
| ADR-0003: Niên Tử Bạch polarity matrix (Tam Nguyên × year polarity) | Explicit (yuan, polarity) → (start, direction) table over arithmetic; 1960 divergence resolved by tiebreak | ✓ §§1–5 authoritative; §6 superseded by ADR-0003a (v1.6 — pre-1984 rows HIGH after dual-source independent secondary modern verification; 1960 case-level center-value split PendingExternalReview) |
| Additive `Option<T>` `DaySnapshot` fields (no `deny_unknown_fields`) | v1.4 producer payloads still deserialize cleanly; v1.5 consumers see new fields when present | ✓ Confirmed in v1.5 (INT-05 round-trip) |
| CRIT-3 isolation: `FlyingStar` never wired into `direction_merge.rs` | Keeps `huyen-khong` palace layouts disjoint from `khcbppt` `sát_phương`/`thần_hướng` until Tier-3 `spatial_compose` lands | ✓ Confirmed in v1.5 (grep-verified by audit) |
| Center star carries Ngũ Hành on aggregate FlyingStar node | `CarriesElement` edge gives the FlyingStar node both spatial (palace) and elemental handles per *Thẩm Thị Huyền Không Học* | ✓ Confirmed in v1.5 post-audit (commit 3e6a148) |

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
*Last updated: 2026-07-15 — v1.6 Eastern Knowledge Completion initialised (4 target features).*
