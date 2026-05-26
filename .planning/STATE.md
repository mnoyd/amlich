---
gsd_state_version: 1.0
milestone: v1.5
milestone_name: Eastern Knowledge Expansion
status: in_progress
last_updated: "2026-05-26T15:07:00.000Z"
progress:
  total_phases: 6
  completed_phases: 0
  total_plans: 5
  completed_plans: 4
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-05-23)

**Core value:** Every almanac subsystem in amlich must produce output matching its canonical classical source for 2020-2030 with test-backed, traceable evidence.
**Current focus:** v1.5 Eastern Knowledge Expansion — Phase 10 in progress (4/5 plans complete).

## Current Position

Milestone: v1.5 Eastern Knowledge Expansion
Phase: 10 (Foundation — Schema Lock + ADRs + Source-ID Registration) — in progress
Plan: 10-01, 10-02, 10-04, 10-03 complete; next: 10-05 (MILESTONES.md ADR cross-references)
Status: 10-03 complete (FND-01 satisfied — ADR-0001 ritual schema v1, RitualEntry type stubs)
Last activity: 2026-05-26 — 10-03 complete: ADR-0001 locked, 10 Rust ritual schema types, 5 behavioral tests

Progress: [░░░░░░░░░░] 0% (0/6 phases complete; 3/5 Phase 10 plans done)

### Milestone Status: v1.5 Roadmap Complete

**Goal:** Ship the first two pillars from `.planning/research/EXPANSION_FRAMEWORK.md` — ritual content lookup (P1 Văn khấn, `source_id: vn-folk-ritual`) and time-based Flying Stars (P4 Phi Tinh, `source_id: huyen-khong`). Both Tier 0; no spatial input this milestone.

**Phases (6 total, numbered 10-15, continuing from v1.4's last phase 9):**

| Phase | Name | Requirements | Status |
|-------|------|--------------|--------|
| 10 | Foundation — Schema Lock + ADRs + Source-ID Registration | FND-01..06 | Not started |
| 11 | Văn khấn Module + Lookup APIs | RIT-01..08 | Not started |
| 12 | Văn khấn Corpus Authoring | RIT-09..13 | Not started |
| 13 | Phi Tinh Primitives + Period + Annual/Monthly | FS-01..10 | Not started |
| 14 | Phi Tinh 81-cell Aspects + Safety Hints | FS-11..15 | Not started |
| 15 | Semantic Graph Wiring + DTO Integration + E2E Validation | INT-01..06 | Not started |

**Parallelization:** Phase 11+12 (Văn khấn) and Phase 13+14 (Phi Tinh) share no code paths and may execute concurrently after Phase 10 lands. Phase 15 is the join point.

**Hard gate:** Phase 10 must complete before any corpus authoring (Phase 12) or algorithm work (Phase 13) begins — re-editing 60 entries after a schema slip is prohibitively expensive (PITFALLS CRIT-1, CRIT-5).

## Performance Metrics

**Velocity:**
- v1.4 plans completed: 6/6
- v1.5 plans completed: 0/? (planning has not begun)

**By Milestone:**

| Milestone | Plans | Total | Avg/Plan |
|-----------|-------|-------|----------|
| v1.2 | 3/3 | 29 min | 9.7 min |
| v1.3 | 5/5 | n/a | n/a |
| v1.4 | 6/6 | 24 min | 4.0 min |
| v1.5 | 1/? | — | — |

**Recent Trend:**
- v1.4 closed clean; v1.5 milestone defined 2026-05-23.
- Research synthesis complete: ARCHITECTURE.md, PITFALLS.md, SUMMARY.md, EXPANSION_FRAMEWORK.md.
- Requirements defined 2026-05-25 (40 v1.5 requirements across FND / RIT / FS / INT categories).
- Roadmap written 2026-05-25 (6 phases, 100% coverage validated).

## Accumulated Context

### Decisions

Project-wide decisions live in PROJECT.md Key Decisions table.

**v1.5 Phase 10 plan 10-01 decisions (2026-05-26):**

- **No SourceId enum — pure pub const &str** — `pub const SOURCE_*: &str` matches CONVENTIONS.md `SCREAMING_SNAKE_CASE` pattern; enum explicitly rejected in CONTEXT.md. New source_ids added to sources.rs, never as bare literals.
- **CI guard uses brace-depth heuristic** — Integration test walks src/, excludes `sources.rs` by name, tracks `#[cfg(test)]` block depth to skip test assertions; no external AST parser needed for amlich-core's consistent layout.
- **Stub rituals/ files in 10-01** — rituals/mod.rs + schema.rs placeholder created by 10-01 to avoid transient lib.rs compile break in Wave 1 parallel execution; plan 10-03 overwrites with real content without touching lib.rs.

**v1.5 Phase 10 plan 10-04 decisions (2026-05-26):**

- **Single parameterized FlyingStarLayout struct** — one struct with FlyingStarPeriod discriminator (Van/Yearly/Monthly) chosen over three distinct types; simpler API, single Phase 15 DTO path.
- **ReasoningEvidenceEnvelope imported via crate::reasoning** — `reasoning::types` is private; types re-exported from `reasoning/mod.rs`; correct import is `use crate::reasoning::{ReasoningEvidenceEnvelope, ReasoningEvidenceSourceFamily}`.
- **ADR-0002 locked: solar-term month boundaries** — monthly Phi Tinh uses tháng tiết khí per Tham Thi Huyen Khong Hoc; get_all_tiet_khi_for_year is the boundary resolver.
- **ADR-0003 locked: polarity matrix not bool flag** — Nien Tu Bach direction is (Tam Nguyen yuan, year polarity) -> (starting star, direction); Thuong/Trung Nguyen rows MEDIUM confidence pending Phase 13 cross-check.

**v1.5 Phase 10 plan 10-03 decisions (2026-05-26):**

- **RitualEventKey::LunarDate as struct variant** — Changed from plan's `LunarDate(LunarDateMatch)` newtype to `LunarDate { month, day, leap_month_policy }` to avoid serde internally-tagged enum nesting conflict; `LunarDateMatch` preserved standalone for Phase 11 RIT-07 API.
- **English fields Option<String> with skip_serializing** — `title_en`, `name_en`, `description_en` are optional and serialization-skipped when None; v1.5 corpus leaves unpopulated.
- **ADR-0001 locked: RitualEntry v1 schema** — 10 types, deny_unknown_fields, closed enums, source_id always "vn-folk-ritual"; Phase 12 corpus changes require superseding ADR.

**v1.5 Phase 10 plan 10-02 decisions (2026-05-26):**

- **SolarHolidayData gets id: String** — solar-holidays.json has id on every entry; exposing it costs nothing and provides symmetry for Phase 15 if needed (additive, no scope creep).
- **Serde derive on Holiday deferred to Phase 15** — Holiday derives only Debug, Clone today; adding serde would reach DTO conversion code outside Phase 10 scope.
- **Thanh Minh id: None** — code path reads from Tiet Khi scanner, not corpus; no corpus id reachable at construction time.

**v1.5-scoped decisions baked into the roadmap (to be ADR'd in Phase 10):**

- **Schema-lock-before-corpus-authoring** — Hard ordering: Phase 10 ADRs precede Phases 12 and 13 (PITFALLS CRIT-1, CRIT-5).
- **Phi Tinh node kind disjoint from KHCBPPT direction modules** — `FlyingStar` is a palace-layout descriptor with `pub const SOURCE_HUYEN_KHONG`, never a bare direction string; NOT wired into `interaction/direction_merge.rs` this milestone (PITFALLS CRIT-3).
- **Vận boundary via Tiết Khí scanner** — Reuse v1.1.2 real Tiết Khí boundary scanner for Lập Xuân instants; naïve `year >= 2024` rejected (PITFALLS CRIT-2).
- **Lo Shu invariants enforced at load** — Vận tables validated for sum=45, each 1-9 once, center=Vận (PITFALLS CRIT-4).
- **Additive-only DTO modifications** — all new `DaySnapshot`/`DayFortune` fields are `Option<T>` with `#[serde(default, skip_serializing_if = "Option::is_none")]` (v1.2 precedent; PITFALLS MOD-6).
- **Two new source_ids registered** — `vn-folk-ritual` and `huyen-khong`, each with module-level `pub const SOURCE_*` to prevent typo-minted fake sources (DEC-0015/0016).

### Research Insights (from research/SUMMARY.md)

**Recommended stack:** No new crate dependencies. Existing `serde` + `serde_json` + `chrono` + `OnceLock` + `include_str!` pattern from `golden_loader.rs` suffices for both pillars.

**Pillar-shared architecture:**
- **P1 Văn khấn** → new top-level `crates/amlich-core/src/rituals/` module + JSON corpus under `data/rituals/`.
- **P4 Phi Tinh** → new sub-folder `crates/amlich-core/src/almanac/fengshui/` (folder, not file — Tier-3 `spatial_compose` will join it later).
- Zero shared code paths between P1 and P4; they reconverge only at semantic-graph wiring (Phase 15).

**Critical pitfalls anchored in PITFALLS.md:**
1. Source-ID cross-contamination between `vn-folk-ritual` / `vn-folk` / `khcbppt` (Phase 10/12).
2. Vận 8 → Vận 9 boundary off-by-one (Lập Xuân 2024-02-04 16:27 ICT, not Jan 1) (Phase 13).
3. Phi Tinh vs KHCBPPT directional conflation (Phase 13/15).
4. Phi Tinh base palace table typos catastrophic + silent — Lo Shu invariants at load (Phase 13).
5. Lễ vật / trình tự stored as freeform strings — schema-first (Phase 10).
6. Lunar/solar date matching ambiguity — typed `LunarDateMatch` (Phase 10/11).
7. Monthly Phi Tinh anchor convention — ADR in Phase 10.
8. Niên Tử Bạch direction inverted by Yuan — polarity matrix ADR in Phase 10.
9. Vietnamese diacritic NFC drift — normalize-on-load (Phase 11).
10. Evidence metadata holes — per-sub-star envelopes (Phase 13).
11. Backward-compat break — `Option<T>` only (Phase 15).

### Known Gaps

- **Phi Tinh has no canonical software cross-check** — mitigated by multi-source golden (≥ 2 references per case) with *Thẩm Thị Huyền Không Học* as tiebreaker; divergences logged as `KnownDivergence` not silently corrected.
- **Monthly anchor convention school-dependent** — mitigated by ADR-0002 (Accepted 2026-05-26): solar-term month boundaries, get_all_tiet_khi_for_year resolver.
- **Niên direction across Tam Nguyên needs polarity matrix** — mitigated by ADR-0003 (Accepted 2026-05-26): (Tam Nguyen, year_polarity) -> (starting_star, direction) matrix; Thuong/Trung Nguyen MEDIUM confidence pending Phase 13 cross-check.
- **Văn khấn single-author risk** — mitigated by per-entry citation + audit ledger (Phase 12).
- **Daily/Hourly Phi Tinh deferral** — explicit OUT-OF-SCOPE in REQUIREMENTS.md.

### Pending Todos

- Execute 10-05-PLAN.md (MILESTONES.md ADR cross-references — all three ADRs now landed: ADR-0001, ADR-0002, ADR-0003).

### Blockers/Concerns

None active. All prior blockers (Kua convention, person-context input, backward compat, Na Am source_id) resolved in v1.2-v1.4.

## Session Continuity

Last session: 2026-05-26T15:07:00Z
Stopped at: Completed 10-03-PLAN.md (FND-01 — ADR-0001 ritual schema v1, RitualEntry locked types).
Resume file: None

### Active TODOs

- Execute 10-05-PLAN.md (MILESTONES.md ADR cross-references — all three ADRs now landed).
- Confirm Phase 10 ADRs land before Phase 12 / Phase 13 execution.

### Context Handoff

**Focus Area:** Phase 10 Foundation — the hard gate for the rest of v1.5.

**Key Constraints:**
- All new types `Option<T>`, `#[serde(deny_unknown_fields)]` on JSON entries.
- Reuse v1.1.2 Tiết Khí scanner for any Lập Xuân instant resolution.
- Phi Tinh node kind never wired into `direction_merge.rs` in v1.5.
- Source-id constants are compile-time `pub const`, not free strings at call-sites.

**Resources:**
- .planning/research/SUMMARY.md — architectural narrative and 6-phase recommendation.
- .planning/research/ARCHITECTURE.md — file:line integration points and module layout.
- .planning/research/PITFALLS.md — CRIT/MOD/MIN catalogue mapped to phases.
- .planning/research/EXPANSION_FRAMEWORK.md — pillar source-of-truth, source_id taxonomy.
- .planning/REQUIREMENTS.md — 40 v1.5 requirements with traceability table.
- .planning/ROADMAP.md — 6-phase plan with success criteria per phase.

---
*State updated: 2026-05-25 after v1.5 roadmap creation*
