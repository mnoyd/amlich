---
gsd_state_version: 1.0
milestone: v1.6
milestone_name: Integration
status: in_progress
last_updated: "2026-07-15T17:05:20.000Z"
progress:
  total_phases: 23
  completed_phases: 23
  total_plans: 63
  completed_plans: 65
---

# Project State
## Project Reference

See: .planning/PROJECT.md (updated 2026-07-15)

**Core value:** Every almanac subsystem in amlich must produce output matching its canonical classical source (KHCBPPT / `vn-folk-ritual` / *Thẩm Thị Huyền Không Học*) for the 2020-2030 date range, with test-backed and traceable evidence.

**Current focus:** v1.6 Eastern Knowledge Completion — Phase 18 Daily Phi Tinh COMPLETE (all 4 plans + FS-16/17/18/19 closed). Next: Phase 19 plan 19-03 (external-crate black-box tests for Offering pipeline).

## Current Position

Milestone: v1.6 Eastern Knowledge Completion.
Phase: 19 of 4 planned (Phase 19: Recommends Offering Semantic Graph Node, 3 plans) — IN PROGRESS.
Plan: 19-02 COMPLETE (Offering node + RecommendsOffering edge across all 6 ontology slice locations + payload field + INT-09 dual-source provenance). Next: Phase 19 plan 19-03 (external-crate black-box tests).
Status: `OfferingRef` struct locked in `crates/amlich-core/src/rituals/schema.rs` with the exact 4-field identity tuple `{ offering_id, name_vi, name_en, source_id }`, `#[serde(deny_unknown_fields)]` (ADR-0001 discipline), and `OfferingRef::new(...)` constructor with `debug_assert!` non-empty enforcement. `source_id` field is typed as `crate::sources::SourceId` (new zero-cost newtype over String alias introduced in 19-01). `DaySnapshot` extended with two additive `Option<T>` fields: `offering_refs: Option<Vec<crate::rituals::OfferingRef>>` (structured preferred) + `offerings: Option<Vec<String>>` (legacy flat-string summary, deduped by `name_vi`) — both carrying `#[serde(default, skip_serializing_if = "Option::is_none")]` matching the EXACT serde pattern as `flying_stars` / `applicable_rituals` / `daily_flying_stars`. `calculate_day_snapshot_internal` populates both fields from `applicable_rituals` via `get_ritual_by_id` with `SOURCE_VN_FOLK_RITUAL` const import (no bare string literal — `source_id_guard` compliant). Schema-lock-before-builder discipline preserved: NO builder code emits `Offering` nodes yet (Plan 19-02's domain).
Last activity: 2026-07-15 — 19-01-PLAN.md executed: OfferingRef schema lock + SourceId alias + DaySnapshot additive fields (commits eddc51d + 6508f79). `cargo build -p amlich-core` clean; `cargo test -p amlich-core --lib rituals::schema::tests::offering_ref_serde_round_trip_and_deny_unknown_fields sources::tests::source_id_alias_is_string day_snapshot_offering_refs_populated_and_deduped day_snapshot_populates_additive_surfaces` 4/4 pass; `cargo test -p amlich-core --test day_snapshot_v14_compat` 6/6 pass (zero regressions); `cargo test -p amlich-core --test source_id_guard` 1/1 pass; `cargo test -p amlich-core` 712 lib tests + all integration tests pass (zero regressions vs Phase 18-04 baseline of 709 lib tests). INT-08 closed; Phase 19-01 complete (1/3 plans).

Progress: [▓▓▓▓▓▓▓▓▓░] 91% (v1.6: 3 of 4 phases complete; 10 of 11 plans complete; Phase 19 is 2/3).

## Key Decisions Added in 18-01 + 18-02 + 18-03 + 18-04

- ADR-0004 locks daily Phi Tinh to 6 Trung Khí pivots with Dương→thuận and Âm→nghịch direction; this is intentionally opposite ADR-0003's annual polarity rule.
- Daily pivot seeds take effect at the first Giáp Tý with JD >= pivot_jd, not at the pivot instant itself; pre-Giáp-Tý days remain under the prior pivot (Pitfall P-7 fall-back).
- The frozen v1 `FlyingStarLayout` remains unchanged; daily schema uses the additive `FlyingStarPeriod::Daily { date: (i32, u32, u32) }` variant plus sibling `DailyFlyingStarLayout`.
- The `daily_pivots_for_year` scanner bracket spans `[year-1, year, year+1]` (widened from the plan's `[year, year+1]`) for robust boundary lookup on late-December dates.
- Pivot matchers accept both "Vũ Thuỷ" (NFD/legacy) and "Vũ Thủy" (NFC/preferred) as the same pivot — Unicode NFC/NFD unification mirrors v1.5 source-corpus normalization discipline.
- Daily golden dataset uses one-file-per-concern split (`flying_stars_daily_golden.json` separate from `flying_stars_golden.json`) per 18-RESEARCH.md Q3 Option B.
- Daily dataset's `expected_center` values are algorithm-computed via `compute_daily_flying_stars` (algorithm-as-ground-truth); external sources are cited as verifications, not as the primary computation source.
- Validator's annual-coverage gate is now kind-aware (conditional on `has_annual`) so daily-only datasets pass validation without panic.
- `DaySnapshot.daily_flying_stars` uses the EXACT serde additive pattern as `flying_stars` / `applicable_rituals`; populate block sits BETWEEN the two existing blocks for readability; solar Y/M/D extracted from `snap.context.solar` to match the snapshot's own context.
- `tests/fengshui_crit3_isolation.rs` is semantically distinct from `tests/source_id_guard.rs` — the former forbids Phi Tinh TYPE NAMES leaking into `direction_merge.rs`; the latter forbids bare source_id STRING LITERALS. Both guards are complementary.

## Key Decisions Added in 19-01

- `pub type SourceId = String;` is a zero-cost newtype over String (NOT a true newtype enforcing SOURCE_* membership) — preserves DEC-0023's `pub const SOURCE_*: &str` discipline (all 7 consts unchanged) while satisfying INT-07's literal "source_id: SourceId" SC text. The alias is a transparent type marker; future phases MAY tighten into a true newtype that enforces SOURCE_* membership at construction.
- `OfferingRef::new(...)` accepts `String` source_id for call-site ergonomics — internally stored as `SourceId`; `debug_assert!` enforces non-empty on `offering_id`, `name_vi`, `source_id`. Avoids forcing call-sites to write `SourceId::from(SOURCE_X.to_string())`.
- `offering_id` is corpus-position-based (`format!("ritual.{ritual_id}.offering.{idx}")`), NOT hashed from `name_vi` — per 19-RESEARCH.md Pitfall P-3 / Don't-Hand-Roll (hashing name_vi would break stable join keys if the corpus is reordered or renamed).
- Both `offering_refs` and `offerings` are derived from the SAME source — `applicable_rituals` via `get_ritual_by_id`; `offering_refs` is the structured preferred path, `offerings` is the legacy flat-string BC summary. `offerings` is deduped by `name_vi` and preserves insertion order (Q4 interpretation i from 19-RESEARCH.md).
- `is_empty() → None` conversion in the populate block preserves the additive contract — a day with no matching rituals (no `offering_refs`) MUST NOT serialize the `offering_refs` key into JSON (skip_serializing_if honored).
- Schema-lock-before-builder discipline preserved: NO builder code emits `Offering` semantic-graph nodes in Plan 19-01; this is reserved for Plan 19-02 (Q4 dual-surface decision: fields on DaySnapshot PLUS additive `payload: Option<serde_json::Value>` on `SemanticNode`).

## Key Decisions Added in 19-02

- INT-07 closed: `NodeConcept::Offering` + `EdgeConcept::RecommendsOffering` added to all 6 ontology slice locations in `ontology.rs` (enum + label() match + ConceptLabel enum + as_str() match + node_concepts()/edge_concepts() static slices + extended locked test). Compiler-enforced exhaustiveness forced updates to `views/helpers.rs::cluster_for_node_id` + `views/visualization.rs::shape_hint_for_node` — both updated for `Offering` variant (no `#[allow(non_exhaustive)]` escape).
- INT-08 SC#2 literal interpretation (Blocker 2 fix): SemanticNode payload uses generic `serde_json::Value` (Option B from 19-RESEARCH.md) — NOT a typed `RitualNodePayload` enum. Matches v1.5 additive `Option<T>` discipline; other concepts can use the same field for concept-specific structured data.
- INT-09 closed (Blocker 1 fix — supersedes Q2 Option C deferral): `RitualMetadata { cross_source_curing: Option<Vec<CrossSourceCure>> }` + `CrossSourceCure { element_cure_for: String, source_id: SourceId, rationale_vi: String }` structs added to `rituals/schema.rs`; additive `metadata: Option<RitualMetadata>` field on `RitualEntry`. The `van-khan-tet-day-du` corpus entry (5 offerings) annotated with one `cross_source_curing` entry whose `source_id = "huyen-khong"`. The `add_offering_facts` builder emits 1 `track_provenance` call for `vn-folk-ritual` (always) + 1 extra call per `cross_source_curing` annotation — dual-source pattern reuses v1.5 multi-source append-pattern (NO parallel dedup helper).
- Payload post-population via new `pub fn nodes_mut(&mut self) -> &mut HashMap<String, SemanticNode>` accessor on `SemanticGraph` — additive companion to `nodes()`. The Ritual node is constructed first (without payload) via `add_node()`, then mutated via `nodes_mut()` after `offering_refs` is known.
- Rationale carried on the EDGE provenance note (Blocker 4 fix) — not just on the Offering node. The dual-source rationale `"lễ vật của nghi lễ, hỗ trợ chữa trị ngũ hành tương ứng"` is embedded in the vn-folk-ritual entry's note via `rationale=...` substring, ensuring any consumer querying the edge can recover the rationale without a node lookup.
- Edge dedup via `HashSet<(ritual_node_id, offering_node_id)>` (NOT provenance dedup) — keys on edge endpoints, not provenance entries. The v1.5 multi-source append-pattern remains the single source of truth for provenance.

## v1.6 Target Features

1. **Daily Flying Star (日紫白)** — ✅ ADR/schema lock done (FS-17); ✅ algorithm + 11 tests green (FS-16); ✅ golden dataset + 4 integration tests green (FS-18); ✅ DaySnapshot field + CRIT-3 grep guard green (FS-19). Phase 18 complete.
2. **`RecommendsOffering` semantic-graph node** — ✅ INT-08 + INT-07 schema lock done (19-01: OfferingRef struct + SourceId alias + DaySnapshot additive fields). ✅ 19-02 complete (Offering + RecommendsOffering ontology + payload field + INT-09 dual-source provenance). Next: 19-03 external-crate black-box tests.
3. **RIT-11 reviewer field closure** — ✅ RIT-14 + RIT-15 closed in 17-01; ✅ RIT-16 closed in 17-02. Phase 17 complete.
4. **ADR-0003 pre-1984 confidence boost** — ✅ FND-07 closed in 16-01; ✅ FND-08 closed in 16-02. Phase 16 complete.

## Resources

- `.planning/PROJECT.md` — project trajectory + Key Decisions table (updated 2026-07-15).
- `.planning/MILESTONES.md` — shipped-milestone log with stats + accomplishments.
- `.planning/ROADMAP.md` — v1.6 roadmap (Phases 16-19, 11 plans, 12/12 requirements mapped). Phase 18 marked Complete (4/4).
- `.planning/REQUIREMENTS.md` — v1.6 requirements + traceability (FS-19 marked Complete post-18-04).
- `.planning/research/SUMMARY.md` — v1.5 research (HIGH confidence on P1/P4; v1.6 daily layer extends the validated patterns; no refresh flagged).
- `.planning/milestones/v1.5-{ROADMAP,REQUIREMENTS,MILESTONE-AUDIT}.md` — v1.5 archive (reuse patterns).
- `.planning/RETROSPECTIVE.md` — cross-milestone learnings (v1.5 patterns carry forward: schema-lock-before-corpus, single-commit RED→GREEN, audit-as-decisive-source, external-crate black-box tests).
- `.planning/adrs/0001-ritual-schema-v1.md` — ADR-0001 (locked).
- `.planning/adrs/0002-phi-tinh-monthly-anchor.md` — ADR-0002 (locked).
- `.planning/adrs/0003-nien-tu-bach-polarity.md` — ADR-0003 (matrix authoritative; §6 superseded by ADR-0003a).
- `.planning/adrs/0003a-nien-tu-bach-polarity-confidence-closure.md` — ADR-0003a (Accepted 2026-07-15; FND-07 + FND-08 source of truth).
- `.planning/adrs/0004-daily-phi-tinh-starting-star-convention.md` — ADR-0004 (Accepted 2026-07-15; FS-17 daily starting-star convention source of truth).
- `.planning/phases/16-foundation-adr-0003-confidence-closure/16-01-SUMMARY.md` — Plan 16-01 execution record (FND-07).
- `.planning/phases/16-foundation-adr-0003-confidence-closure/16-02-SUMMARY.md` — Plan 16-02 execution record (FND-08).
- `.planning/phases/17-van-khan-reviewer-closure/17-01-SUMMARY.md` — Plan 17-01 execution record (RIT-14 + RIT-15 ledger expansion).
- `.planning/phases/17-van-khan-reviewer-closure/17-02-SUMMARY.md` — Plan 17-02 execution record (RIT-16 corrected-entry gate).
- `.planning/phases/18-daily-phi-tinh/18-01-SUMMARY.md` — Plan 18-01 execution record (FS-17 ADR + schema lock).
- `.planning/phases/18-daily-phi-tinh/18-02-SUMMARY.md` — Plan 18-02 execution record (FS-16 algorithm + 11 tests).
- `.planning/phases/18-daily-phi-tinh/18-03-SUMMARY.md` — Plan 18-03 execution record (FS-18 golden dataset + loader + integration tests).
- `.planning/phases/18-daily-phi-tinh/18-04-SUMMARY.md` — Plan 18-04 execution record (FS-19 DaySnapshot additive field + CRIT-3 grep guard).
- `.planning/phases/19-recommends-offering-semantic-graph-node/19-01-SUMMARY.md` — Plan 19-01 execution record (INT-08 OfferingRef schema lock + INT-07 SourceId alias + DaySnapshot additive fields).
- `.planning/phases/19-recommends-offering-semantic-graph-node/19-02-SUMMARY.md` — Plan 19-02 execution record (Offering + RecommendsOffering ontology + INT-08 payload field + INT-09 dual-source provenance).

## Session Continuity

Last session: 2026-07-15T17:05:20Z
Stopped at: Completed 19-02-PLAN.md. Phase 19 plan 2 of 3 executed (INT-07 + INT-08 + INT-09 closed; Offering + RecommendsOffering ontology across 6 slices + payload field + dual-source provenance). Phase 19-03 (external-crate black-box tests) next.
Resume file: None.

### Next Step

Start Phase 19 plan 19-03: external-crate black-box tests for the full RecommendsOffering pipeline. Primary test surface: load `tet-nguyen-dan.json`, build a Tết 2026 `DaySnapshot`, assert the graph contains `RecommendsOffering` edges with dual-source provenance (vn-folk-ritual + huyen-khong for the van-khan-tet-day-du entry's offerings).

---
*State updated: 2026-07-15 after 19-02-PLAN.md executed (INT-07 + INT-08 + INT-09 closed; Phase 19 2/3 complete; build clean; 716 lib tests pass (+4 from 712 baseline); day_snapshot_v14_compat 6/6 pass; source_id_guard 1/1 pass). Phase 19-03 next.*
