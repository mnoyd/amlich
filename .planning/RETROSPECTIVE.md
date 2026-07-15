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

### Cumulative Quality

| Milestone | Tests | Coverage | Zero-Dep Additions |
|-----------|-------|----------|-------------------|
| v1.0 | 184 passing | Core KHCBPPT validator coverage established | 0 |
| v1.1 | 188 passing | Extended subsystem and regression acceptance coverage | 0 |
| v1.3 | 242 passing (`amlich-core --lib`) | Dai Van core + helper contracts + Kua per-pillar analysis verified | 0 |
| v1.5 | 886 passing (`amlich-core`, 0 failures) | Văn khấn lookup + Phi Tinh overlays + 81-cell aspects + semantic graph wiring; CRIT-3 isolation grep-verified | 1 (`unicode-normalization` for NFC-at-load) |
| v1.6 | 922 passing (`amlich-core`, 0 failures) | Daily Phi Tinh + RecommendsOffering node + dual-source provenance + ADR-0003a confidence closure; CRIT-3 isolation preserved; 13/13 integration WIRED | 0 (pure Rust, reuses v1.5 deps) |

### Top Lessons (Verified Across Milestones)

1. Verification artifacts and executable tests must remain synchronized to avoid status drift.
2. Small focused phases with explicit acceptance criteria improve recovery when regressions appear.
3. Deterministic helper contracts should be asserted with fixture-based boundary tests to prevent edge-case regressions.
4. Reuse-first integration (new feature over existing deterministic helpers) lowers scope risk while keeping behavior auditable.
