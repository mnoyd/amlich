# Roadmap: Amlich Almanac Correctness Audit

## Milestones

- ✅ **v1.1 Foundation Extensions** — shipped 2026-01 (Phases 1-3 + v1.1.1/v1.1.2 hotfixes), archived in [`milestones/v1.1-ROADMAP.md`](milestones/v1.1-ROADMAP.md)
- ✅ **v1.2 Ten Gods + Kua Foundation** — shipped 2026-02 (Phases v1.2-01..03), archived in [`milestones/v1.2-ROADMAP.md`](milestones/v1.2-ROADMAP.md)
- ✅ **v1.4 Lunar Engine Table Parity** — shipped 2026-03-04 (Phases 7-9, 6/6 plans), archived in [`milestones/v1.4-ROADMAP.md`](milestones/v1.4-ROADMAP.md)
- ✅ **v1.5 Eastern Knowledge Expansion** — shipped 2026-05-28 (Phases 10-15, 24/24 plans), archived in [`milestones/v1.5-ROADMAP.md`](milestones/v1.5-ROADMAP.md)
- ✅ **v1.6 Eastern Knowledge Completion** — shipped 2026-07-16 (Phases 16-19, 11/11 plans), archived in [`milestones/v1.6-ROADMAP.md`](milestones/v1.6-ROADMAP.md)
- ✅ **v1.7 Kinh Dịch (I-Ching Divination)** — shipped 2026-07-19 (Phases 20-25, 14/14 plans), archived in [`milestones/v1.7-ROADMAP.md`](milestones/v1.7-ROADMAP.md)

## Current Focus

- v1.7 shipped the P2 Kinh Dịch pillar (Mai Hoa Dịch Số casting + 64-hexagram Ngô Tất Tố corpus + `IChingEvaluator`) and the Thái Tuế / Tam Sát ⇄ Phi Tinh read-only directional cross-link. All 15 requirements satisfied; 1120 crate tests green; zero new crate dependencies.
- Next milestone is undefined — start with `/gsd-new-milestone` (questioning → research → requirements → roadmap). Carry-forward candidates from v1.7 retrospective + `EXPANSION_FRAMEWORK` include: P3 Y học Tý Ngọ Lưu Chú, P5 spatial Phi Tinh (requires user spatial input + Tier-3 `spatial_compose` landing), P6 Tử Vi Đẩu Số, Hỗ Quả (nuclear hexagram, v1.9+), Tier-2 Bazi enrichment of hexagram reading.

---

<details>
<summary>✅ v1.7 Kinh Dịch (I-Ching Divination) (Phases 20-25) — SHIPPED 2026-07-19</summary>

- [x] Phase 20: Foundation — Schema Lock + Source IDs + ADRs + Ontology (3/3 plans) — completed 2026-07-15
- [x] Phase 21: IChing Corpus + Loader (2/2 plans) — completed 2026-07-16
- [x] Phase 22: Mai Hoa Casting + Biến Quẻ + Thể/Dụng (2/2 plans) — completed 2026-07-16
- [x] Phase 23: Thái Tuế / Tam Sát ⇄ Phi Tinh Cross-Link (3/3 plans) — completed 2026-07-16
- [x] Phase 24: IChing Evaluator + Semantic-Graph Wiring + DTO Integration (3/3 plans) — completed 2026-07-19
- [x] Phase 25: E2E Validation + Golden Cross-Source Verification (1/1 plan) — completed 2026-07-19

**Delivered:** Mai Hoa Dịch Số casting + 64-hexagram Ngô Tất Tố corpus + `IChingEvaluator` (Tier-0) + Thái Tuế / Tam Sát ⇄ Phi Tinh read-only directional cross-link. All 15 requirements satisfied; 1120 tests pass; zero new deps. Full details in [`milestones/v1.7-ROADMAP.md`](milestones/v1.7-ROADMAP.md).

</details>

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
| 20. Foundation — Schema Lock + Source IDs + ADRs + Ontology | v1.7 | 3/3 | Complete | 2026-07-15 |
| 21. IChing Corpus + Loader | v1.7 | 2/2 | Complete | 2026-07-16 |
| 22. Mai Hoa Casting + Biến Quẻ + Thể/Dụng | v1.7 | 2/2 | Complete | 2026-07-16 |
| 23. Thái Tuế / Tam Sát ⇄ Phi Tinh Cross-Link | v1.7 | 3/3 | Complete | 2026-07-16 |
| 24. IChing Evaluator + Semantic-Graph Wiring + DTO Integration | v1.7 | 3/3 | Complete | 2026-07-19 |
| 25. E2E Validation + Golden Cross-Source Verification | v1.7 | 1/1 | Complete | 2026-07-19 |

## Cross-Cutting Constraints (carry-forward)

- **Additive-only DTOs** — all new fields `Option<T>` with `#[serde(default, skip_serializing_if = "Option::is_none")]` (v1.2 precedent; re-validated in v1.5 INT-05 + v1.6 INT-10 + v1.7 INT-12 combined-strip round-trip).
- **Source-id discipline** — every new module declares `pub const SOURCE_*` for its tradition; provenance call-sites never use string literals (CI-enforced by `tests/source_id_guard.rs`; v1.7 extended the guard with `SOURCE_KINH_DICH` + `SOURCE_MAI_HOA_DICH_SO`).
- **CRIT-3 isolation** — `FlyingStar` / `DailyFlyingStarLayout` remain palace-layout descriptors; never wired into `interaction/direction_merge.rs`. v1.7 extended the grep-guard surface with a sibling `tests/thai_tue_cross_link_crit3.rs` covering the new `reasoning/direction_composite.rs` read-only cross-link.
- **Schema-lock before corpus/algorithm** — type stubs + ADR + 1-entry serde round-trip probe precede corpus authoring (v1.5 CRIT-1 lesson × 7 amplification for v1.7's 448 corpus text fields).
- **Sibling-newtype over closed-enum extension** — `IChingQuery` newtype + `IChingEvaluator` rather than extending `ConsultationIntent` (v1.6 `DailyFlyingStarLayout` sibling precedent; avoids ~25–43 call-site `Copy`-break churn).
- **Composite-envelope multi-source provenance** — cross-link emits multiple `ReasoningEvidenceEnvelope` instances per `PersonalFactNode`, each with its distinct primitive `source_id`, plus ONE composite envelope with `source_id: "rule.composite.*"` (the only pattern compatible with the CRIT-3 grep guard; v1.5 INT-09 dual-source Direction-node precedent + v1.7 `rule.composite.iching_consultation` + `rule.composite.direction_cross_link`).
- **Runtime-built needle patterns for grep guards** — `tests/thai_tue_cross_link_crit3.rs` (v1.7) builds its forbidden-pattern needles at runtime (e.g. `String::from("phi").push('_').push_str("tinh.palace_layout")`) so the test's own source code does not self-trip the guard. Established as the canonical pattern for any future CRIT-3-surface grep guard.

---

*Last updated: 2026-07-19 — v1.7 Kinh Dịch (I-Ching Divination) milestone SHIPPED. 6/6 phases, 14/14 plans, 15/15 requirements complete; 1120 crate tests green; zero new deps. Next milestone undefined — start with `/gsd-new-milestone`.*
