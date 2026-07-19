# Project Retrospective

*A living document updated after each milestone. Lessons feed forward into future planning.*

## Milestone: v1.1 - Foundation Extensions

**Shipped:** 2026-03-02
**Phases:** 3 | **Plans:** 9 | **Sessions:** 1

### What Was Built
- Added complete extended Xung Hop relationship coverage and integrated new fields into day-level outputs.
- Added Tang Can hidden-stem data model, lookup logic, baseline data, and serialization-ready DayFortune integration.
- Fixed Tiet Khi nearest-term regression and restored full-package acceptance gate with updated governance artifacts.

### What Worked
- Verification-first status reconciliation prevented roadmap/state drift after implementation work.
- Gap-closure phases (v1.1.1, v1.1.2) isolated documentation/governance debt from code fix execution.

### What Was Inefficient
- Initial Tiet Khi implementation used synthetic approximation and required a follow-up correction cycle.
- Milestone completion metadata needed multiple post-cycle passes before reaching stable machine-readable consistency.

### Patterns Established
- Treat canonical verification artifacts as status authority, then propagate to roadmap/state/requirements.
- Use scoped requirement IDs when identifiers overlap across milestone families.

### Key Lessons
1. Acceptance gates should validate both implementation behavior and governance artifacts in the same execution cycle.
2. Milestone-specific requirement files reduce ambiguity and keep master registries lightweight.

### Cost Observations
- Model mix: not measured in this repo snapshot
- Sessions: 1
- Notable: most effort in this milestone came from reconciliation and acceptance hardening, not net-new feature volume

---

## Milestone: v1.3 - Dai Van Core

**Shipped:** 2026-03-03
**Phases:** 3 completed / 3 planned | **Plans:** 5 completed | **Sessions:** 1

### What Was Built
- Delivered the core Dai Van engine with deterministic direction and start-age primitives plus auditable convention/evidence metadata.
- Implemented end-to-end Dai Van generation and helper APIs for age lookup, current pillar, and transition countdown semantics.
- Added lazy Ten Gods adapters for Dai Van pillars/ages with explicit Option handling for unknown birth-day stem and out-of-range paths.
- Implemented Kua-based per-pillar directional analysis with birth-Kua single-computation reuse and age/index lookup helpers.

### What Worked
- Plan-level decomposition (04-01, 04-02, 05-01, 05-02) kept implementation scope tight and verification fast.
- Contract-focused tests (half-open ranges, transition boundaries, lazy behavior) prevented silent semantic drift.
- Reusing existing helper boundary semantics for Kua lookup-by-age kept Phase 6 integration low-risk and deterministic.

### What Was Inefficient
- Strict float equality assumptions initially caused avoidable test churn before epsilon-based assertions were adopted.
- A patch-merge context mismatch temporarily removed helper functions and required blocking recovery during plan execution.
- Direction-order assumptions in Kua intersection tests needed one alignment pass to match deterministic source ordering.

### Patterns Established
- Keep Ten Gods out of base Dai Van payloads and expose correlation via lazy helper APIs only.
- Use deterministic in-memory DaiVanResult fixtures to lock helper contracts independent of upstream conversion variability.
- Compute birth Kua once per analysis and derive per-pillar direction results through pure intersections.

### Key Lessons
1. Deterministic domain contracts are easier to extend when core payloads remain minimal and optional features are derived lazily.
2. Boundary semantics (`[start_age, end_age)`) need dedicated contract tests, not just integration-path coverage.
3. For directional analysis, preserving canonical ordering from source sets reduces flaky expectation drift in tests.

### Cost Observations
- Model mix: not measured in this repo snapshot
- Sessions: 1
- Notable: execution remained efficient; rework was limited to correctness guardrails (float precision, patch-context recovery, direction-order alignment)

---

## Milestone: v1.5 — Eastern Knowledge Expansion

**Shipped:** 2026-05-28
**Phases:** 6 (10–15) | **Plans:** 24 | **Tests:** 886 passing | **Timeline:** 2026-05-26 → 2026-05-29 (3 active days)

### What Was Built

Two new Tier-0 pillars beside the entrenched `khcbppt` family:
- **P1 Văn khấn cổ truyền** (`vn-folk-ritual`): 60-entry corpus across 13 category JSON files; OnceLock + NFC-at-load loader; 4 lookup APIs (`find_van_khan_for_snapshot/event/life_event/by_id`); leap-aware `event_key_matches` with asymmetric `Always` semantics.
- **P4 Phi Tinh thời gian** (`huyen-khong`): Vận/Niên/Nguyệt layered overlays with Lo Shu invariants enforced at load; 81-pair star-pair aspect table (6 classical primaries + 75 Ngũ Hành derivations); danger-star override; 4-row safety-hint corpus with `FORBIDDEN_PRODUCT_TERMS` CI guard.
- **Semantic graph integration**: 5 new ontology variants (`Ritual`, `FlyingStar` nodes; `PrescribedFor`, `OccupiesPalace`, `CarriesElement` edges); dual-provenance Direction node (`khcbppt` + `huyen-khong`); additive `Option<T>` `DaySnapshot` fields with v1.4 round-trip preserved.

### What Worked

- **Schema-lock-before-corpus ordering (Phase 10 → 12).** Authoring 60 ritual entries against a frozen ADR-0001 schema produced zero re-edit churn — the PITFALLS CRIT-1/5 gate paid off exactly as scoped.
- **`pub const` source-ID taxonomy over enum.** Adding two new source-ids (`vn-folk-ritual`, `huyen-khong`) required only one-line module-level constants plus a CI grep guard (`tests/source_id_guard.rs`); no central enum churn.
- **Multi-source golden + `KnownDivergence` ledger.** Where authoritative texts disagreed (1960 Trung Nguyên: lasotuvi.com=6 vs phongthuycaivan.org=5), we logged the divergence and picked the *Thẩm Thị Huyền Không Học* tiebreak — divergences became evidence rather than silent corrections.
- **Single-commit RED→GREEN where the production test IS the falsifier.** Phase 11-04 and the post-audit CarriesElement fix both shipped the failing test + fix in one commit; the test file is the proof, not a separate ceremony commit.
- **Milestone audit caught a real runtime gap.** `EdgeConcept::CarriesElement` was declared in ontology, tested at type level, but unemitted by any builder. The audit's evidence-based integration check (886 tests + grep sweeps + builder-call traces) surfaced what plan verifications hadn't.

### What Was Inefficient

- **STATE.md drift across waves.** The frontmatter `milestone:` field remained `v1.4` even after v1.5 had shipped multiple phases; the per-plan log developed duplicate entries (e.g. `Phase 14 P03` listed twice). This is harness mechanics, not strategy — but downstream tools (`milestone complete`) treat it as authoritative.
- **REQUIREMENTS.md checkbox lag.** Phase 10 marked FND-01/02/04/05/06 as SATISFIED in its VERIFICATION.md on 2026-05-26, but the REQUIREMENTS.md rollup table still showed `Pending` / `[ ]` until the audit on 2026-05-28. A verification step should automatically flip the rollup.
- **`summary-extract` one-liner extraction returned "Dependency graph"** for many SUMMARY.md files because the heading-detection heuristic matched the wrong section. MILESTONES.md accomplishments had to be hand-composed.

### Patterns Established

- **External-crate black-box test convention.** `crates/<crate>/tests/<feature>_integration.rs` files using `use <crate>::...` exercise the public API surface as a consumer would; complements white-box `#[cfg(test)] mod tests` blocks. v1.5 added 4 such files: `rituals_integration.rs`, `fengshui_invariants.rs`, `fengshui_aspects.rs`, `day_snapshot_v14_compat.rs`, `integration_2026_smoke.rs`.
- **Dual-provenance nodes** for semantic edges that cross pillars (Direction node carries both `khcbppt`-family travel evidence and `huyen-khong` Phi Tinh provenance via `.with_provenance()` chaining; `add_node` would overwrite).
- **CRIT-3 isolation as a grep-verifiable invariant.** `FlyingStar` is forbidden in `interaction/direction_merge.rs`; the audit confirms by exact-zero grep hits rather than by code review.
- **Audit-as-decisive-source.** Plan VERIFICATION.md files verified phase goals individually; the milestone audit re-derived satisfaction from three independent sources (VERIFICATION + SUMMARY frontmatter + REQUIREMENTS rollup) and flipped checkboxes where they disagreed.

### Key Lessons

1. **Schema-lock gates are worth their cost.** Phase 10 took 1 day; the alternative was re-editing 60 corpus entries when an author hit a missing field. The gate's value scales with corpus size, so always pay it before authoring.
2. **Dead-code edges in ontology are silent integration bugs.** Type-level "reader can find X" is satisfied by declaration; runtime "X is emitted" needs an explicit builder test. The milestone audit should grep for ontology variants in builder code, not just in ontology declarations.
3. **Source-id discipline survives only with a CI guard.** Bare string literals creep in via copy-paste; the brace-depth grep in `source_id_guard.rs` proved necessary and sufficient.
4. **Tiered Ngũ Hành tables outperform per-pair authoring.** 81-pair aspect table was authored as 6 classical primaries + 75 element-relation derivations + override rule (danger stars 2/5); 100% manual would have been 81 entries with no consistency check.
5. **`Option<T>` + `#[serde(default, skip_serializing_if)]` is sufficient for additive DTO evolution.** v1.4 fixtures round-tripped through v1.5 `DaySnapshot` unmodified; no `deny_unknown_fields` was needed at the DTO level (only at corpus-JSON level).

### Cost Observations

- Model mix: not instrumented this milestone; per-plan elapsed times (in STATE.md per-plan log) range from 2 min to 18 min for normal plans, with a single 137-min outlier (Phase 15 P04, the 2026 E2E smoke build-out).
- Sessions: bursty across 4 days (2026-05-23 milestone-define, 2026-05-26 Phase 10, 2026-05-27 Phases 11–13, 2026-05-28 Phases 14–15 + audit, 2026-05-29 close-out).
- Notable: 84 commits / 9,476 LOC delta / 24 plans suggests ~390 LOC/plan and ~3.5 commits/plan — consistent with v1.4's pattern at higher absolute volume.

---

## Milestone: v1.6 - Eastern Knowledge Completion

**Shipped:** 2026-07-16
**Phases:** 4 (16-19) | **Plans:** 11 | **Sessions:** 2 (2026-07-15 define+execute, 2026-07-16 audit+close)

### What Was Built
- Landed the deferred daily Phi Tinh layer (日紫白): `compute_daily_flying_stars` with 冬至/夏至 reversal (Dương thuận / Âm nghịch) via the v1.1.2 real-boundary Tiết Khí scanner, Giáp-Tý-as-seed mechanic, and Pitfall P-7 prior-pivot fall-back guard; ADR-0004 locks the daily starting-star convention.
- Promoted `RecommendsOffering` to a first-class semantic-graph node — `NodeConcept::Offering` + `EdgeConcept::RecommendsOffering` (Ritual → Offering) across all 6 ontology slice locations, with dual-source edge provenance (`huyen-khong` + `vn-folk-ritual`) via `RitualMetadata`/`CrossSourceCure` reusing the v1.5 multi-source `track_provenance` pattern.
- Closed carry-forward v1.5 review/confidence tech debt: ADR-0003a superseded ADR-0003 §6 (pre-1984 Thượng/Trung Nguyên rows MEDIUM → HIGH after dual-source independent secondary modern verification); all 60 ritual entries carry `reviewer` field with method/date/outcome in `provenance_audit.md`.

### What Worked
- **Deferral-discipline schema fields.** By-design domain-expert deferrals (1960 Trung Nguyên split, 60 reviewer entries, ADR-0004 page citation, Hạ Chí 2025 daily divergence) are tracked via typed `DeferralMarker`/`ExternalReviewPending` schema fields with due dates, not silently corrected. This kept the audit clean (zero gaps) and made the deferrals machine-queryable.
- **Schema-lock-before-builder again paid off.** `DailyFlyingStarLayout` stub (18-01) and `OfferingRef` type (19-01) preceded any builder emission — no schema slip cascaded into corpus rework.
- **Algorithm-as-ground-truth + multi-source citation.** The daily golden dataset computes `expected_center` via the algorithm itself and cites external sources as verifications (not as the primary computation source), cleanly handling the Hạ Chí 2025 cross-source disagreement as a logged `KnownDivergence`.
- **Combined-strip round-trip discipline** (INT-10): stripping ALL v1.6-new additive fields together to simulate a v1.5 fixture, then asserting byte-equal re-serialization — a single canonical DTO-evolution verification pattern.
- **Audit re-derived satisfaction from 3 independent sources** (VERIFICATION + SUMMARY frontmatter + REQUIREMENTS rollup), catching zero orphans.

### What Was Inefficient
- **Milestone-complete CLI `--help` is treated as a version/name argument**, not a help request. Invoking `milestone complete --help` silently archived the milestone under the literal name `--help`, polluting MILESTONES.md + STATE.md and creating `--help-*.md` files. Required manual cleanup (revert + rename). The CLI lacks a real help/usage surface.
- **`summary-extract --fields one_liner` returns empty** for v1.6 SUMMARY.md files because the frontmatter uses a `provides:` YAML block instead of a `one_liner` field. Accomplishments had to be hand-composed from `provides:` lists — same issue noted in v1.5 retrospective (heading-detection heuristic).
- **2-day compressed timeline** meant the milestone audit ran immediately after the last plan; no cool-down buffer between feature-complete and audit, though it passed cleanly.

### Patterns Established
- **Typed deferral markers over silent correction.** When a source disagreement cannot be resolved without external domain-expert review, the code records a typed schema field (`DeferralMarker`, `ExternalReviewPending`) with a due date rather than picking a winner. This is now the canonical "honest about uncertainty" pattern.
- **Ontology exhaustiveness enforced by compiler.** Adding `NodeConcept::Offering` + `EdgeConcept::RecommendsOffering` required updating 6 slice locations; the compiler's exhaustive-match forced every consumer (`cluster_for_node_id`, `shape_hint_for_node_id`) to handle the new variant — no `#[allow(non_exhaustive)]` escape.
- **Additive `Option<serde_json::Value>` payload on `SemanticNode`.** Generic typed-payload-per-concept pattern (Option B) over a concept-specific enum; other concepts can reuse the same field.
- **Daily-boundary algorithm reuses the v1.1.2 Tiết Khí scanner.** No naïve year arithmetic; the daily Giáp-Tý seed lookup widens to `[year-1, year, year+1]` for robust boundary edge cases.

### Key Lessons
1. **Dual-source verification can promote confidence tiers.** Two independent secondary modern sources cross-checking each other was sufficient to promote pre-1984 polarity rows MEDIUM → HIGH, with the classical text retained only as tiebreaker — a reusable confidence-boost recipe.
2. **CRIT-3 isolation survives additive feature growth.** Adding daily Phi Tinh + Offering nodes touched neither `direction_merge.rs` nor leaked any `FlyingStar` type names into the interaction layer; the grep-guard test (`fengshui_crit3_isolation.rs`) is the cheap, durable enforcement.
3. **Pre-existing debt does not accrue.** The ~96 clippy/fmt warnings were 96 before v1.6 and 96 after — additive discipline prevented new debt, but the existing debt persists as a cleanup-phase candidate.
4. **Transparent type aliases satisfy literal SC text without runtime cost.** `pub type SourceId = String` met INT-07's "source_id: SourceId" success criterion without a true newtype; documented as future-tightenable.

### Cost Observations
- Model mix: not instrumented this milestone.
- Sessions: 2 (2026-07-15 define + all 4 phases executed; 2026-07-16 audit + milestone close).
- Notable: 50 commits / +12,704 LOC / 11 plans = ~1,155 LOC/plan and ~4.5 commits/plan — higher LOC/plan than v1.5 (~390), driven by the daily golden dataset + corpus annotations. Timeline compressed to ~2 days vs v1.5's ~4.

## Milestone: v1.7 — Kinh Dịch (I-Ching Divination)

**Shipped:** 2026-07-19
**Phases:** 6 (20-25) | **Plans:** 14 | **Sessions:** ~5 (2026-07-16 research/define/Phase 20-23, 2026-07-19 Phase 24-25 + close)

### What Was Built
- Shipped the P2 Kinh Dịch pillar: pure-deterministic Mai Hoa Dịch Số casting (`cast_mai_hoa` with the single named `mai_hoa_remainder` boundary-safe reduction helper) + `derive_bien_que` (384-case CRIT-4 contract) + `classify_the_dung` Ngũ Hành sinh/khắc + CatHung verdict.
- Authored the 64-hexagram Ngô Tất Tố corpus (`data/iching/hexagrams.json`) against the locked ADR-0005 schema — structural fields populated; interpretive text honestly `PendingExternalReview` per AF-05 (no fabrication from another translator); lazy `OnceLock` loader consuming `include_str!`.
- Built `IChingEvaluator` + `IChingQuery` SIBLING-NEWTYPE (not a `ConsultationIntent::IChing` variant) at Tier-0 (no birth data required, MOD-7) with a locked 4-envelope evidence vector (3 primitives carrying `SOURCE_MAI_HOA_DICH_SO` / `SOURCE_KINH_DICH` + 1 composite `rule.composite.iching_consultation` — CRIT-6).
- Shipped Thái Tuế / Tam Sát ⇄ Phi Tinh read-only directional cross-link: `thai_tue_direction` year-only sibling API + classical 3-direction `almanac::tam_sat` module + read-only `build_direction_cross_link(_personal/_date)` surfacing BOTH KHCBPPT directional taboos AND Huyền-Không palace layout with three-envelope provenance.
- Wired Hexagram semantic-graph nodes (chu + biến with role-bearing stable keys) via `LocatedAt` / `Transforms` edges + composite Direction fact node through additive `add_iching_facts()` + `add_direction_composite_facts()` builder methods.
- Added additive `DaySnapshot.iching_cast` + `DaySnapshot.direction_cross_link` fields with combined-strip v1.6→v1.7 backward-compat round-trip (Tests 10-12 in `day_snapshot_v14_compat.rs`).
- Phase 25 closure: unified v1.7 E2E smoke exercising all five surfaces on Tết 2026 + 4 Sóc dates; two NEW runtime-invariant baseline guards (`cargo_dependency_tree_unchanged_from_v16` + `int13_golden_dataset_cross_source_discipline_holds`) for defense-in-depth.

### What Worked
- **Schema-lock-before-corpus × 7 amplification paid off again.** ADR-0005 + 1-entry serde round-trip probe + three CRIT-3-isolating newtypes (`TienThienTrigram` / `HauThienTrigram` / `KingWenHexagram`) landed in Phase 20 BEFORE any of the 64 corpus entries were authored in Phase 21. Zero schema slips across 448 corpus text fields.
- **Sibling-newtype over closed-enum extension.** `IChingQuery` as a NEW newtype (not a `ConsultationIntent::IChing` variant) avoided ~25-43 call-site `Copy`-break churn — mirrors v1.6 `DailyFlyingStarLayout` sibling precedent and the v1.7 `DirectionCrossLinkSummary` precedent. Compiler enforced the discipline at the trait surface.
- **Single named helper as the CRIT-2 boundary gate.** `mai_hoa_remainder((sum, k))` is the ONE function in the codebase implementing the `((n-1)%k)+1` reduction. Boundary test guards against refactors that replace it with `sum%k` or `(sum%k)+1` — structural prevention of the silent ~1/8 casting corruption.
- **384-case exhaustive contract test (CRIT-4).** `derive_bien_que` validated across 64 chủ quée × 6 động hào combinations — exhaustive coverage rather than representative sampling. The CRIT-4 "line-flip ALWAYS changes the hexagram" runtime invariant is now also locked by Phase 25's E2E smoke.
- **Runtime-built needle patterns for grep guards.** Building the forbidden-pattern needles via `String::from("phi").push('_').push_str("tinh.palace_layout")` (final value byte-equal to `phi_tinh.palace_layout`) keeps the test's own source code clean — the same self-tripping trap noted in v1.5/v1.6 was avoided by construction. Now codified across corpus.rs / mai_hoa.rs / bien_que.rs / the_dung.rs / golden.rs / evaluator.rs.
- **Combined-strip round-trip discipline (INT-12).** Tests 10-12 strip BOTH v1.7-new fields together (`iching_cast` + `direction_cross_link`) from a fully populated v1.6 snapshot, then assert byte-equal round-trip with null-count parity (strictly stronger than v1.6 INT-10's literal-negation pattern — tolerates pre-existing nulls elsewhere in the JSON).
- **Two parallel tracks merging at the evaluator+wiring phase.** Cross-link (Phase 23) ran in parallel with the IChing pillar (Phases 21-22) without file conflicts; Phase 24 was the merge point. Critical path 20→21→22→24→25 (5 hops) vs parallel 20→23→24 — finished in ~5 days.
- **`include_str!`-parse approach for runtime Cargo.toml dep-tree guard** (Phase 25 SC4) — faster + cargo-version-robust vs invoking `cargo tree` at test time via `std::process::Command`. Locks the SET of 4 deps without enforcing declaration order.

### What Was Inefficient
- **No formal `v1.7-MILESTONE-AUDIT.md` was produced.** Milestone closed directly after Phase 25 with no separate audit pass. Phase 25's baseline guards + E2E smoke served as partial validation, but no comprehensive audit re-derived satisfaction from VERIFICATION.md / SUMMARY frontmatter / REQUIREMENTS rollup across all 6 phases the way v1.4/v1.5/v1.6 audits did. v1.5/v1.6 caught zero gaps in their audits, but the audit is also where cross-phase regression risk surfaces — skipping it loses that safety net.
- **`milestone complete --help` is still treated as a version/name argument** (carry-forward from v1.6 retrospective). I tripped this again — running `milestone complete --help` to learn the CLI syntax silently archived a milestone under the literal name `--help`, polluting MILESTONES.md + STATE.md and creating `--help-*.md` files. Required manual `git checkout` + `rm` cleanup. The CLI still lacks a real help/usage surface — second milestone in a row this trap has fired.
- **`summary-extract --fields one_liner` still returns empty** for the same reason as v1.6 — SUMMARY frontmatter uses a `provides:` YAML block instead of a `one_liner` field. Accomplishments had to be hand-composed from `provides:` lists across 14 SUMMARY files. Carry-forward from v1.5/v1.6.
- **5-day compressed timeline with no cool-down** between feature-complete (Phase 24-03, 2026-07-19) and milestone close (same day). Risk of cross-phase regression is highest right after the last merge — no buffer for surfacing late issues.
- **Phase 21-02 plan still shows `[ ]` unchecked in the archived v1.7-ROADMAP.md** even though the SUMMARY exists and the requirement is marked Complete. Carry-forward bookkeeping drift; does not affect correctness.

### Patterns Established
- **Runtime-built needle patterns for grep guards (canonical).** When a test's source code would otherwise contain the literal forbidden substrings, build the needles via `String::from(...).push_str(...)` so the test's own source code is clean. Established across 6 v1.7 modules and codified as a Cross-Cutting Constraint in ROADMAP.md for any future CRIT-3-surface grep guard.
- **Sibling-newtype over closed-enum extension (re-validated).** Adding a sibling newtype + sibling evaluator struct is cheaper than extending a closed enum that ~25-43 call-sites already match on. The third consecutive confirmation (v1.6 `DailyFlyingStarLayout`, v1.7 `IChingQuery`, v1.7 `DirectionCrossLinkSummary`) — now a documented carry-forward constraint.
- **Single named helper for boundary conventions.** Boundary conventions like `((n-1)%k)+1` live in ONE named function (`mai_hoa_remainder`) — the boundary test then guards against any refactor that bypasses it. Reusable recipe for any future "naive arithmetic vs classical convention" boundary.
- **Combined-strip DTO round-trip (extension of v1.6 INT-10).** Strip ALL milestone-new additive `Option<T>` fields together to simulate the previous-version fixture, then assert byte-equal re-serialization with null-count parity (not literal negation). Tolerates pre-existing nulls elsewhere; strictly stronger than v1.6's pattern.
- **Per-step evidence envelope + composite (CRIT-6).** Each step in a multi-source derivation gets its own primitive envelope (with distinct `source_id`); the composite envelope does NOT collapse the primitives — every step remains individually traceable. Generalizes the v1.5 INT-09 dual-source Direction-node pattern to N-primitive + 1-composite.
- **Role-bearing stable keys for graph nodes.** `SemanticId::iching_hexagram(role, king_wen, date, tz)` puts the role (`chu` vs `bien`) as the FIRST segment of the stable key so the primary + transformed hexagrams cannot collide in `graph.node_count()`. Reusable for any future "two-of-the-same-concept" graph wiring.
- **Tier-0 `ActionEvaluator` adapter pattern.** When a Tier-0 (no birth data) reasoning capability needs to plug into the generic `ActionEvaluator` trait surface, return `Ok(ActionEvaluation::empty(ActionId::X))` ignoring `personal_input`. The rich evaluation lives behind a sibling `evaluate_consultation` method. Mirrors `InitiationOpeningEvaluator`.

### Key Lessons
1. **A 5-day milestone with no audit is a calculated risk, not a free win.** Phase 25's baseline guards caught the regression-class risks (dep-tree drift, golden-discipline weakening), but the cross-phase integration audit is a different failure mode. v1.5/v1.6 audits passed clean, but the value of running the audit is partly the act of running it — skipping it loses the forcing function. Recommend either (a) always run `/gsd-audit-milestone` before closing, or (b) accept the trade-off explicitly in MILESTONES.md.
2. **CLI `--help` arguments are still positional.** The `milestone complete` CLI parses `args[2]` as the version with no flag validation; passing `--help` literal silently creates a `--help` milestone. This has now fired in two consecutive milestone closes (v1.6 + v1.7). Permanent mitigation: never invoke `milestone complete` without a literal version; do not try to introspect the CLI via `--help`. Real fix would be a `--help` short-circuit in the CLI dispatcher.
3. **Three newtypes + 64-entry composition table > runtime King Wen correctness assertions.** Compiler-enforced type disjointness between Tiên Thiên / Hậu Thiên / King Wen (CRIT-3 prevention) plus a bijective composition table validated at load eliminates whole classes of bugs that runtime assertions would only catch at execution time. The schema is the spec.
4. **Parallel tracks with explicit merge points compress timelines without coordination overhead.** Phase 23 (cross-link) running parallel to Phases 21-22 (IChing pillar) had zero file conflicts because the contracts were defined upfront in Phase 20. Phase 24 was the explicit merge point. Pattern is now repeatable for any future multi-pillar milestone.
5. **Honest page-citation deferral is a shipping enabler.** `data/almanac/tam_sat_provenance.md` ships the locked rule + mapping today with an explicit `PendingExternalReview` marker for the exact KHCBPPT edition/page pin — this unblocks XLK-02 without fabricating a citation, and the deferred item is machine-queryable. The fourth confirmation of the v1.6 typed-deferral pattern (joining ADR-0003a / ADR-0004 / RIT-14).
6. **Pre-existing engineering debt persists across milestones.** The ~96 clippy/fmt warnings were 96 before v1.7 and 96 after — additive discipline prevented new debt, but the existing debt does not self-heal. A dedicated cleanup phase remains the only path to actually reducing the count.

### Cost Observations
- Model mix: not instrumented this milestone.
- Sessions: ~5 (research/define + Phase 20 + Phase 21-23 + Phase 24 + Phase 25/close, compressed 2026-07-16 → 2026-07-20).
- Notable: 79 commits / +24,498 LOC / 14 plans = ~1,750 LOC/plan and ~5.6 commits/plan — highest LOC/plan of any milestone to date (v1.6 was ~1,155; v1.5 was ~390). Driven by the 64-hexagram corpus + 13-section Phase 20 ADR/ontology foundation + 384-case contract test + 22-test integration suite for the cross-link. +198 tests vs v1.6's 922-test baseline (v1.5 → v1.6 added 36 tests; v1.6 → v1.7 added 198 — the contract-test discipline is intensifying).

---

## Cross-Milestone Trends

### Process Evolution

| Milestone | Sessions | Phases | Key Change |
|-----------|----------|--------|------------|
| v1.0 | 4 | 4 | Built KHCBPPT baseline and validator harness workflow |
| v1.1 | 1 | 3 | Added verification-led governance closure as a first-class phase pattern |
| v1.3 | 1 | 3/3 completed | Introduced deterministic Dai Van core contracts, lazy Ten Gods helpers, and per-pillar Kua direction analysis |
| v1.5 | ~4 | 6 | First multi-pillar milestone: source-id taxonomy as `pub const`, schema-lock-before-corpus ordering, milestone audit re-deriving satisfaction from 3 independent sources |
| v1.6 | 2 | 4 | Deferral-discipline schema fields, daily-boundary scanner reuse, algorithm-as-ground-truth golden datasets, compiler-enforced ontology exhaustiveness |
| v1.7 | ~5 | 6 | Sibling-newtype over closed-enum extension, single-named-helper boundary gates (CRIT-2/4), per-step evidence envelope + composite (CRIT-6), runtime-built needle patterns for grep guards, parallel tracks with explicit merge points, **first milestone closed without a formal audit** |

### Cumulative Quality

| Milestone | Tests | Coverage | Zero-Dep Additions |
|-----------|-------|----------|-------------------|
| v1.0 | 184 passing | Core KHCBPPT validator coverage established | 0 |
| v1.1 | 188 passing | Extended subsystem and regression acceptance coverage | 0 |
| v1.3 | 242 passing (`amlich-core --lib`) | Dai Van core + helper contracts + Kua per-pillar analysis verified | 0 |
| v1.5 | 886 passing (`amlich-core`, 0 failures) | Văn khấn lookup + Phi Tinh overlays + 81-cell aspects + semantic graph wiring; CRIT-3 isolation grep-verified | 1 (`unicode-normalization` for NFC-at-load) |
| v1.6 | 922 passing (`amlich-core`, 0 failures) | Daily Phi Tinh + RecommendsOffering node + dual-source provenance + ADR-0003a confidence closure; CRIT-3 isolation preserved; 13/13 integration WIRED | 0 (pure Rust, reuses v1.5 deps) |
| v1.7 | 1120 passing (`amlich-core`, 0 failures) | Mai Hoa casting + 64-hexagram corpus + IChingEvaluator + Thái Tuế/Tam Sát cross-link + semantic-graph Hexagram wiring + combined-strip v1.6→v1.7 round-trip + runtime-invariant baseline guards; CRIT-3 + CRIT-6 + zero-dep-tree preserved | 0 (pure Rust, reuses v1.5 deps — locked by `cargo_dependency_tree_unchanged_from_v16` runtime guard) |

### Top Lessons (Verified Across Milestones)

1. Verification artifacts and executable tests must remain synchronized to avoid status drift.
2. Small focused phases with explicit acceptance criteria improve recovery when regressions appear.
3. Deterministic helper contracts should be asserted with fixture-based boundary tests to prevent edge-case regressions.
4. Reuse-first integration (new feature over existing deterministic helpers) lowers scope risk while keeping behavior auditable.
5. **Schema-lock before corpus authoring** (v1.5 → v1.6 → v1.7 × 7 amplification) — type stubs + ADR + 1-entry serde round-trip probe precede corpus authoring; compiler-enforced newtype disjointness eliminates whole bug classes that runtime assertions would only catch at execution time. The schema is the spec.
6. **Additive `Option<T>` + combined-strip round-trip** (v1.5 INT-05 → v1.6 INT-10 → v1.7 INT-12) — the discipline that keeps `DaySnapshot` backward-compatible across four consecutive milestones. Strip ALL milestone-new fields together to simulate the previous-version fixture; assert byte-equal re-serialization with null-count parity.
7. **Sibling-newtype over closed-enum extension** (v1.6 `DailyFlyingStarLayout` → v1.7 `IChingQuery` + `DirectionCrossLinkSummary`) — adding a sibling newtype + sibling evaluator struct is cheaper than extending a closed enum that ~25-43 call-sites already match on. Three consecutive confirmations; now a documented carry-forward constraint.
8. **CLI `--help` is positional** (v1.6 + v1.7 both fired) — never invoke `gsd-tools milestone complete` without a literal version; do not try to introspect via `--help`. Real fix would be a `--help` short-circuit in the CLI dispatcher.
9. **Pre-existing engineering debt does not self-heal** (v1.5 → v1.6 → v1.7) — additive discipline prevents new debt (96 → 96 → 96 clippy/fmt warnings) but the existing count persists. A dedicated cleanup phase remains the only path to actually reducing it.
