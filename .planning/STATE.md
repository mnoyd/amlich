---
gsd_state_version: 1.0
milestone: v1.4
milestone_name: milestone
status: unknown
last_updated: "2026-05-27T17:05:26.769Z"
progress:
  total_phases: 18
  completed_phases: 17
  total_plans: 48
  completed_plans: 45
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-05-23)

**Core value:** Every almanac subsystem in amlich must produce output matching its canonical classical source for 2020-2030 with test-backed, traceable evidence.
**Current focus:** v1.5 Eastern Knowledge Expansion — Phase 11 (Văn khấn Module + Lookup APIs) COMPLETE (4/4 plans). Phase 12 (Văn khấn Corpus Authoring) or Phase 13 (Phi Tinh) next.

## Current Position

Milestone: v1.5 Eastern Knowledge Expansion
Phase: 12 (Văn khấn Corpus Authoring) — In Progress (3/4 plans complete)
Plan: 12-04 complete (RIT-11 provenance audit ledger — 60 unique ritual_ids across 13 event categories)
Status: 12-04 complete (provenance_audit.md: classical reference + page + confidence + reviewer per entry; 4 classical works enumerated; 1:1 coverage verified; plan 12-03 loader wiring remaining)
Last activity: 2026-05-27 — 12-04 complete: RIT-11 provenance ledger shipped; all 60 unique ritual_ids covered from 13 event category files; reviewer pending per research Q4; plan 12-03 (loader wiring) is the only remaining phase-12 plan

Progress: [███░░░░░░░] 33% (2/6 phases complete; Phase 11 done)

### Milestone Status: v1.5 Roadmap Complete

**Goal:** Ship the first two pillars from `.planning/research/EXPANSION_FRAMEWORK.md` — ritual content lookup (P1 Văn khấn, `source_id: vn-folk-ritual`) and time-based Flying Stars (P4 Phi Tinh, `source_id: huyen-khong`). Both Tier 0; no spatial input this milestone.

**Phases (6 total, numbered 10-15, continuing from v1.4's last phase 9):**

| Phase | Name | Requirements | Status |
|-------|------|--------------|--------|
| 10 | Foundation — Schema Lock + ADRs + Source-ID Registration | FND-01..06 | Complete |
| 11 | Văn khấn Module + Lookup APIs | RIT-01..08 | Complete (4/4 plans) |
| 12 | Văn khấn Corpus Authoring | RIT-09..13 | Not started |
| 13 | Phi Tinh Primitives + Period + Annual/Monthly | FS-01..10 | Not started |
| 14 | Phi Tinh 81-cell Aspects + Safety Hints | FS-11..15 | Not started |
| 15 | Semantic Graph Wiring + DTO Integration + E2E Validation | INT-01..06 | Not started |

**Parallelization:** Phase 11+12 (Văn khấn) and Phase 13+14 (Phi Tinh) share no code paths and may execute concurrently after Phase 10 lands. Phase 15 is the join point.

**Hard gate:** Phase 10 must complete before any corpus authoring (Phase 12) or algorithm work (Phase 13) begins — re-editing 60 entries after a schema slip is prohibitively expensive (PITFALLS CRIT-1, CRIT-5).

## Performance Metrics

**Velocity:**
- v1.4 plans completed: 6/6
- v1.5 plans completed: Phase 10 (5/5) + Phase 11 (4/4 — 11-01..11-04) = 9 plans

**By Milestone:**

| Milestone | Plans | Total | Avg/Plan |
|-----------|-------|-------|----------|
| v1.2 | 3/3 | 29 min | 9.7 min |
| v1.3 | 5/5 | n/a | n/a |
| v1.4 | 6/6 | 24 min | 4.0 min |
| v1.5 | 9/? | — | — |

**Recent Trend:**
- v1.4 closed clean; v1.5 milestone defined 2026-05-23.
- Research synthesis complete: ARCHITECTURE.md, PITFALLS.md, SUMMARY.md, EXPANSION_FRAMEWORK.md.
- Requirements defined 2026-05-25 (40 v1.5 requirements across FND / RIT / FS / INT categories).
- Roadmap written 2026-05-25 (6 phases, 100% coverage validated).
- Phase 10 (Foundation) closed clean 2026-05-26; Phase 11 wave 1 (plan 11-01) landed 2026-05-26 — fixtures + Hán guard + unicode-normalization dep.
- Phase 11 wave 2 (plan 11-02) landed 2026-05-26 — corpus.rs OnceLock loader; RIT-05 complete; NFC-at-load and source_id discipline both enforced at first access.
- Phase 11 wave 3 (plan 11-03) landed 2026-05-26 — matcher.rs with 4 lookup APIs (RIT-01..04) + leap-aware event_key_matches (RIT-06, RIT-07); rituals/mod.rs re-exports full API surface; 597 lib tests pass.
- Phase 11 wave 4 (plan 11-04) landed 2026-05-26 — 6 black-box integration tests at tests/rituals_integration.rs; Rule-1 auto-fix on matcher Always-needle symmetry; Phase 11 closes (RIT-01, RIT-07, RIT-08 complete).

**Per-plan log:**
| Phase 11 P01 | 18min | 3 tasks | 3 files |
| Phase 11 P02 | 3min | 2 tasks (RED+GREEN) | 2 files |
| Phase 11 P03 | 2min | 2 tasks | 2 files |
| Phase 11 P04 | 2min | 1 task | 2 files |
| Phase 12 P01 | 9 | 2 tasks | 6 files |
| Phase 12-van-khan-corpus-authoring P02 | 10 | 2 tasks | 7 files |
| Phase 12-van-khan-corpus-authoring P04 | 5 | 1 task | 1 file |
| Phase 12 P04 | 5 | 1 tasks | 1 files |
| Phase 12 P03 | 3 | 2 tasks | 3 files |
| Phase 13 P01 | 4 | 2 tasks | 6 files |

## Accumulated Context

### Decisions

Project-wide decisions live in PROJECT.md Key Decisions table.

**v1.5 Phase 11 plan 11-04 decisions (2026-05-26):**

- **Matcher Always semantic switched from symmetric to asymmetric** — Removed `(_, Always) => true` from `event_key_matches`; kept `(Always, _) => true`. The previous symmetric arm caused every LifeEvent-only entry (e.g. `van-khan-dong-tho`) to fire on every snapshot because `derive_event_keys` emits an `Always` needle. New semantic: haystack-side Always matches any needle (daily-fire entry); needle-side Always only matches an Always haystack (query for daily entries). Inline `always_sentinel_matches_anything` test updated to encode the asymmetry.
- **Single-commit RED→GREEN for 11-04** — Test file + matcher fix shipped in one commit (`e0cb5b4`). The integration test file IS the falsifier — Test 2's per-hit honesty check turned RED on the symmetric matcher and turned GREEN after the arm change. No artificial test-only RED commit needed.
- **HolidayId cross-reference sweep range 2020-2030** — Matches the project Core Value statement and catches any year-offset edge cases (some holidays have year_offset ±1).
- **External-crate black-box convention established** — `crates/amlich-core/tests/<feature>_integration.rs` files import via `use amlich_core::...` to verify the public API surface as an external consumer would; complements white-box `#[cfg(test)] mod tests` blocks in `src/`.

**v1.5 Phase 11 plan 11-03 decisions (2026-05-26):**

- **Closed-enum + `_ => false` collapse arm preserved** — `event_key_matches` covers all same-variant pairs across the locked 5-variant RitualEventKey plus the Always-sentinel cross-variant case; cross-variant non-matches collapse via a single `_ => false` arm. Doc-comment on the function flags the superseding-ADR requirement should a 6th variant ever land (ADR-0001 §Schema Discipline).
- **`derive_event_keys` does NOT emit LifeEvent needles** — Resolved at research Q4: life events are caller intent (Động thổ, Cưới, Khai Trương, etc.), not day properties. Only `find_van_khan_for_life_event(kind)` wraps a LifeEvent needle before delegating to `find_van_khan_for_event`.
- **Vec<&'static RitualEntry> return type** — Corpus is OnceLock-backed so static refs cost nothing; callers hold results indefinitely without cloning. Avoids the API ergonomics tax of `Vec<RitualEntry>` (forced clone) or `impl Iterator<...>` (lifetime gymnastics for callers).
- **Holiday id needles via `h.id.clone()`** — `get_vietnamese_holidays` returns owned `Vec<Holiday>`; no `&Holiday` reference held across the loop. Clone cost negligible (≤6 holidays/day, short ids).
- **Plain `as u8` casts for lunar month/day** — Domain invariants (1..=12 / 1..=30) guarantee safety; matches existing project cast discipline at lib.rs callsites. No `try_into()` ceremony.

**v1.5 Phase 11 plan 11-02 decisions (2026-05-26):**

- **TDD round-trip in 2 commits (RED + GREEN)** — RED commit shipped corpus.rs with `todo!()` stub + 5 inline tests + the `mod corpus;` registration (registration required for tests to compile, so Task 2's mod.rs edit landed in the RED commit). GREEN commit narrowed to swapping corpus.rs body with the full loader; mod.rs untouched in the GREEN phase.
- **RitualFile envelope kept private** — Loader-internal struct; only `pub fn all_rituals()` crosses module boundary. Helpers (`normalize_and_validate`, `nfc`, JSON/version consts, OnceLock) all stay module-private per plan constraint #5.
- **Constant-only source_id comparison** — `assert_eq!(entry.source_id, SOURCE_VN_FOLK_RITUAL, ...)` imported via `use crate::sources::SOURCE_VN_FOLK_RITUAL;`. Zero bare `"vn-folk-ritual"` literals in corpus.rs verified by grep; source_id_guard.rs CI green post-merge.
- **NFC short-circuit via `is_nfc()`** — `nfc(s) { if is_nfc(s) { s.to_string() } else { s.nfc().collect() } }` avoids the decomposition pipeline for canonical input (the common case since fixtures.json was pre-normalized by plan 11-01).
- **Dead-code warnings deferred to plan 11-03** — `all_rituals`, `normalize_and_validate`, `nfc` are flagged unused at `cargo build -p amlich-core` because corpus is private and not yet re-exported. Resolves automatically when 11-03 lands `pub use corpus::all_rituals;`.

**v1.5 Phase 11 plan 11-01 decisions (2026-05-26):**

- **Inline char-range Hán detection over external crate** — `matches!(c, '\u{4E00}'..='\u{9FFF}' | ...)` covers 4 CJK blocks (base + Ext-A + Ext-B + Compatibility); zero new transitive deps per 11-RESEARCH.md §Don't Hand-Roll.
- **No manifest.json this wave** — research Q2 defers manifest to Phase 12; single fixtures.json suffices for stub corpus.
- **unicode-normalization 0.1.25 landed before loader** — Cargo accepts unused deps; declaring early prevents transient build break when 11-02 lands in parallel wave.
- **Hán guard no-ops on missing data/rituals/ dir** — defensive ordering pattern preserved for Phase 12 file additions even though wave-1 plan ships fixtures + guard atomically.
- **TDD validated via ephemeral RED check** — injected `{"han_test":"中文"}` ad-hoc file, confirmed `2 Hán code points found` panic, removed before commit.

**v1.5 Phase 10 plan 10-01 decisions (2026-05-26):**

- **No SourceId enum — pure pub const &str** — `pub const SOURCE_*: &str` matches CONVENTIONS.md `SCREAMING_SNAKE_CASE` pattern; enum explicitly rejected in CONTEXT.md. New source_ids added to sources.rs, never as bare literals.
- **CI guard uses brace-depth heuristic** — Integration test walks src/, excludes `sources.rs` by name, tracks `#[cfg(test)]` block depth to skip test assertions; no external AST parser needed for amlich-core's consistent layout.
- **Stub rituals/ files in 10-01** — rituals/mod.rs + schema.rs placeholder created by 10-01 to avoid transient lib.rs compile break in Wave 1 parallel execution; plan 10-03 overwrites with real content without touching lib.rs.

**v1.5 Phase 10 plan 10-04 decisions (2026-05-26):**

- **Single parameterized FlyingStarLayout struct** — one struct with FlyingStarPeriod discriminator (Van/Yearly/Monthly) chosen over three distinct types; simpler API, single Phase 15 DTO path.
- **ReasoningEvidenceEnvelope imported via crate::reasoning** — `reasoning::types` is private; types re-exported from `reasoning/mod.rs`; correct import is `use crate::reasoning::{ReasoningEvidenceEnvelope, ReasoningEvidenceSourceFamily}`.
- **ADR-0002 locked: solar-term month boundaries** — monthly Phi Tinh uses tháng tiết khí per Tham Thi Huyen Khong Hoc; get_all_tiet_khi_for_year is the boundary resolver.
- **ADR-0003 locked: polarity matrix not bool flag** — Nien Tu Bach direction is (Tam Nguyen yuan, year polarity) -> (starting star, direction); Thuong/Trung Nguyen rows MEDIUM confidence pending Phase 13 cross-check.

**v1.5 Phase 10 plan 10-05 decisions (2026-05-26):**

- **TABLE subsection for ADR registry** — `### ADR Cross-References` uses markdown TABLE format (not narrative list) appended after Key Decisions; keeps v1.0-v1.4 history intact, gives Phase 10+ decisions their own structured shape per CONTEXT.md §specifics.
- **DEC-0023 is next safe id** — DEC-0015, 0016, 0022 referenced in planning docs; DEC-0017-0021 unreferenced; 0023 confirmed safe starting point for Phase 10 ADRs.
- **Relative links from .planning/MILESTONES.md** — `adrs/000X-name.md` resolves to `.planning/adrs/`; no absolute paths needed.

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
- [Phase 12]: thanh-minh entries use solar_term key exclusively — no holiday_id consistent with holidays.rs:177 None assignment
- [Phase 12]: Corpus batch 1 expanded entries to 5 per file for Nguyên Tiêu, Hàn Thực, Thanh Minh to reach >=26 total; RIT-12 coverage broadened to 4 multi-variant events
- [Phase 12-02]: Vu Lan 4 entries: added folk cung-co-hon alongside simple/full/buddhist RIT-12 variants to cover Ghost Festival dual purpose
- [Phase 12-02]: van-khan-dong-tho (full) ritual_id preserved exactly to match fixtures.json reference and test suite
- [Phase 12-02]: Giao Thua: 2 entries (indoor simple + outdoor full) capturing both le-cung sub-types with identical event_keys
- [Phase 12]: Fixtures.json duplicates excluded from provenance ledger: 6 ritual_ids appear in both fixtures.json and canonical category files; ledger uses canonical source for 60 unique rows
- [Phase 12]: Provenance reviewer field set to pending for all 60 entries per research Q4; peer review deferred post-v1.5 but citation coordinates enable future independent review
- [Phase 12]: fixtures.json absorbed (deleted): all 6 ritual_ids confirmed migrated to category files in 12-01/12-02; no 14th file needed
- [Phase 12]: Multi-file include_str! loader pattern: one const per file, ALL_CORPUS_JSONS array, loop-merge in OnceLock initializer; manifest.json is documentation-only artifact
- [Phase 13]: Lập Xuân CRIT-2 fix: compute_period uses jd<lap_xuan_jd to determine effective_year (never year>=2024 naive check)
- [Phase 13]: Lo Shu invariants at load: validate_van_table() panics on sum≠45, duplicates, or center≠van — catches JSON typos at startup (CRIT-4)
- [Phase 13]: van_for_solar_year formula: ((y-1864)/20)+1 clamped 1..=9; Vận 8=2004-2023, Vận 9=2024-2043
- [Phase 13]: Base palace table: Lo Shu thuận path Center->NW->W->NE->S->N->SW->E->SE; palaces[4]=van, palaces[0]=van+5, etc.

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

- Phase 11 closed (4/4 plans). Next: Phase 12 (Văn khấn Corpus Authoring, RIT-09..13) or Phase 13 (Phi Tinh Primitives, FS-01..10). The two share no code paths and may execute concurrently.

### Blockers/Concerns

None active. All prior blockers (Kua convention, person-context input, backward compat, Na Am source_id) resolved in v1.2-v1.4.

## Session Continuity

Last session: 2026-05-27T16:35:00Z
Stopped at: Completed 12-04-PLAN.md (RIT-11 provenance audit ledger: 60 unique ritual_ids across 13 event categories; classical reference + page + confidence + reviewer (pending) per entry; 4 classical works enumerated; 1:1 coverage verified).
Resume file: None

### Active TODOs

- Phase 12 plan 12-03 (wave 2) — wire loader to read per-category files via include_str! constants (only remaining Phase 12 plan).
- Phase 13 (Phi Tinh Primitives) may run concurrently — no shared code paths with Phase 12.

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
