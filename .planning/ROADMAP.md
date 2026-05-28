# Roadmap: Amlich Almanac Correctness Audit

## Milestones

- ✅ **v1.1 Foundation Extensions** — shipped 2026-01 (Phases 1-3 + v1.1.1/v1.1.2 hotfixes), archived in [`milestones/v1.1-ROADMAP.md`](milestones/v1.1-ROADMAP.md)
- ✅ **v1.2 Ten Gods + Kua Foundation** — shipped 2026-02 (Phases v1.2-01..03), archived in [`milestones/v1.2-ROADMAP.md`](milestones/v1.2-ROADMAP.md)
- ✅ **v1.4 Lunar Engine Table Parity** — shipped 2026-03-04 (Phases 7-9, 6/6 plans), archived in [`milestones/v1.4-ROADMAP.md`](milestones/v1.4-ROADMAP.md)
- ✅ **v1.5 Eastern Knowledge Expansion** — shipped 2026-05-28 (Phases 10-15, 24/24 plans), archived in [`milestones/v1.5-ROADMAP.md`](milestones/v1.5-ROADMAP.md)
- 📋 **v1.6 (next)** — to be scoped via `/gsd:new-milestone`

## Phases

<details>
<summary>✅ v1.5 Eastern Knowledge Expansion (Phases 10-15) — SHIPPED 2026-05-28</summary>

- [x] Phase 10: Foundation — Schema Lock + ADRs + Source-ID Registration (5/5 plans) — completed 2026-05-26
- [x] Phase 11: Văn khấn Module + Lookup APIs (4/4 plans) — completed 2026-05-26
- [x] Phase 12: Văn khấn Corpus Authoring (4/4 plans) — completed 2026-05-27
- [x] Phase 13: Phi Tinh Primitives + Period + Annual/Monthly (4/4 plans) — completed 2026-05-27
- [x] Phase 14: Phi Tinh 81-cell Aspects + Safety Hints (3/3 plans) — completed 2026-05-27
- [x] Phase 15: Semantic Graph Wiring + DTO Integration + E2E Validation (4/4 plans) — completed 2026-05-28

Full phase details, success criteria, and plan breakdown: [`milestones/v1.5-ROADMAP.md`](milestones/v1.5-ROADMAP.md). Milestone audit: [`milestones/v1.5-MILESTONE-AUDIT.md`](milestones/v1.5-MILESTONE-AUDIT.md). 40/40 requirements satisfied; 2 by-design tech-debt deferrals carried forward (RIT-11 reviewer field; ADR-0003 pre-1984 confidence).

</details>

<details>
<summary>✅ v1.4 Lunar Engine Table Parity (Phases 7-9) — SHIPPED 2026-03-04</summary>

- [x] Phase 7: Hour-Pillar Parity Core
- [x] Phase 8: Sexagenary-Cycle Parity + Validators
- [x] Phase 9: Na Am API Surfaces

Full details: [`milestones/v1.4-ROADMAP.md`](milestones/v1.4-ROADMAP.md).

</details>

<details>
<summary>✅ v1.2 Ten Gods + Kua Foundation — SHIPPED 2026-02</summary>

Phases v1.2-01..03. Full details: [`milestones/v1.2-ROADMAP.md`](milestones/v1.2-ROADMAP.md).

</details>

<details>
<summary>✅ v1.1 Foundation Extensions — SHIPPED 2026-01</summary>

Phases 1-3 + v1.1.1 verification closure + v1.1.2 Tiết Khí regression fix. Full details: [`milestones/v1.1-ROADMAP.md`](milestones/v1.1-ROADMAP.md).

</details>

### 📋 v1.6 (Planning)

No phases yet. Run `/gsd:new-milestone` to scope the next cycle.

## Progress Table

| Milestone | Phases | Plans | Status | Shipped |
|-----------|--------|-------|--------|---------|
| v1.1 Foundation Extensions | 1-3 + hotfixes | n/a | Shipped | 2026-01 |
| v1.2 Ten Gods + Kua Foundation | v1.2-01..03 | 3/3 | Shipped | 2026-02 |
| v1.4 Lunar Engine Table Parity | 7-9 | 6/6 | Shipped | 2026-03-04 |
| v1.5 Eastern Knowledge Expansion | 10-15 | 24/24 | Shipped | 2026-05-28 |
| v1.6 | — | — | Planning | — |

## Cross-Cutting Constraints (carry-forward)

- **Additive-only DTOs** — all new fields `Option<T>` with `#[serde(default, skip_serializing_if = "Option::is_none")]` (v1.2 precedent, re-validated in v1.5 INT-05).
- **Source-id discipline** — every module declares `pub const SOURCE_*` for its tradition; provenance call-sites never use string literals (CI-enforced by `tests/source_id_guard.rs`).
- **CRIT-3 isolation** — Phi Tinh / `FlyingStar` is a palace-layout descriptor; never wired into `interaction/direction_merge.rs` (`khcbppt`-family directional logic stays disjoint from `huyen-khong`).
- **Tiết Khí scanner reuse** — solar-term anchors reuse the v1.1.2 real-boundary scanner; no naïve year arithmetic.

---
*Last reorganized: 2026-05-29 after v1.5 close-out.*
