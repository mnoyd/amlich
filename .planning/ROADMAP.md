# Roadmap: Amlich Almanac Correctness Audit

## Milestones

- ✅ **v1.1 Foundation Extensions** — shipped 2026-01 (Phases 1-3 + v1.1.1/v1.1.2 hotfixes), archived in [`milestones/v1.1-ROADMAP.md`](milestones/v1.1-ROADMAP.md)
- ✅ **v1.2 Ten Gods + Kua Foundation** — shipped 2026-02 (Phases v1.2-01..03), archived in [`milestones/v1.2-ROADMAP.md`](milestones/v1.2-ROADMAP.md)
- ✅ **v1.4 Lunar Engine Table Parity** — shipped 2026-03-04 (Phases 7-9, 6/6 plans), archived in [`milestones/v1.4-ROADMAP.md`](milestones/v1.4-ROADMAP.md)
- ✅ **v1.5 Eastern Knowledge Expansion** — shipped 2026-05-28 (Phases 10-15, 24/24 plans), archived in [`milestones/v1.5-ROADMAP.md`](milestones/v1.5-ROADMAP.md)
- ✅ **v1.6 Eastern Knowledge Completion** — shipped 2026-07-16 (Phases 16-19, 11/11 plans), archived in [`milestones/v1.6-ROADMAP.md`](milestones/v1.6-ROADMAP.md)

## Current Focus

All five shipped milestones are archived. No active milestone — next milestone is TBD via `/gsd-new-milestone`.

---

## Phases

<details>
<summary>✅ v1.6 Eastern Knowledge Completion (Phases 16-19) — SHIPPED 2026-07-16</summary>

- [x] Phase 16: Foundation — ADR-0003 Confidence Closure (2/2 plans) — completed 2026-07-15
- [x] Phase 17: Văn khấn Reviewer Closure (2/2 plans) — completed 2026-07-15
- [x] Phase 18: Daily Phi Tinh 日紫白 (4/4 plans) — completed 2026-07-15
- [x] Phase 19: RecommendsOffering Semantic-Graph Node + v1.6 Integration (3/3 plans) — completed 2026-07-16

**Delivered:** Daily Phi Tinh layer + `RecommendsOffering` first-class node + v1.5 review/confidence debt closed. 12/12 requirements satisfied; 922 tests pass. Full details in [`milestones/v1.6-ROADMAP.md`](milestones/v1.6-ROADMAP.md).

</details>

<details>
<summary>✅ v1.5 Eastern Knowledge Expansion (Phases 10-15) — SHIPPED 2026-05-28</summary>

- [x] Phase 10: Foundation — Schema Lock + ADRs + Source-ID Registration (5/5 plans)
- [x] Phase 11: Văn khấn Module + Lookup APIs (3/3 plans)
- [x] Phase 12: Văn khấn Corpus Authoring (4/4 plans)
- [x] Phase 13: Phi Tinh Primitives + Period + Annual/Monthly (4/4 plans)
- [x] Phase 14: Phi Tinh 81-Cell Aspects + Safety Hints (4/4 plans)
- [x] Phase 15: Semantic Graph Wiring + DTO Integration + E2E Validation (4/4 plans)

Full details in [`milestones/v1.5-ROADMAP.md`](milestones/v1.5-ROADMAP.md).

</details>

## Progress Table

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 16. Foundation — ADR-0003 Confidence Closure | v1.6 | 2/2 | Complete | 2026-07-15 |
| 17. Văn khấn Reviewer Closure | v1.6 | 2/2 | Complete | 2026-07-15 |
| 18. Daily Phi Tinh 日紫白 | v1.6 | 4/4 | Complete | 2026-07-15 |
| 19. RecommendsOffering Semantic-Graph Node | v1.6 | 3/3 | Complete | 2026-07-16 |

## Cross-Cutting Constraints (carry-forward)

- **Additive-only DTOs** — all new fields `Option<T>` with `#[serde(default, skip_serializing_if = "Option::is_none")]` (v1.2 precedent; re-validated in v1.5 INT-05 + v1.6 INT-10).
- **Source-id discipline** — every new module declares `pub const SOURCE_*` for its tradition; provenance call-sites never use string literals (CI-enforced by `tests/source_id_guard.rs`).
- **CRIT-3 isolation** — `FlyingStar`/`DailyFlyingStarLayout` remain palace-layout descriptors; never wired into `interaction/direction_merge.rs`.
- **Tiết Khí scanner reuse** — boundary semantics reuse the v1.1.2 real-boundary scanner; no naïve year arithmetic.
- **Audit-as-decisive-source** — `provenance_audit.md` and ADR narrative are the canonical records for confidence + reviewer disposition.
- **Schema-lock before corpus/algorithm** — type stubs precede builder emission.

---

*Last updated: 2026-07-16 — v1.6 Eastern Knowledge Completion archived (Phases 16-19, 11 plans, 12/12 requirements). No active milestone; next milestone TBD via `/gsd-new-milestone`.*
