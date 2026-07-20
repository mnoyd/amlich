---
phase: 16-foundation-adr-0003-confidence-closure
plan: 02
subsystem: foundation
tags: [phi-tinh, adr, golden-dataset, deferral, fnd-08, schema-additive]

# Dependency graph
requires:
  - phase: 16-foundation-adr-0003-confidence-closure
    plans: ["16-01"]
    provides: "ADR-0003a accepted supersession + typed GoldenConfidence + Test F + 1960 PendingExternalReview narrative locked in §4"
provides:
  - "Typed pub struct DeferralMarker { reason, expected_review_date, assigned_to: Option<String> } re-exported from almanac::fengshui"
  - "Additive pub deferral: Option<DeferralMarker> field on KnownDivergence with #[serde(default, skip_serializing_if = \"Option::is_none\")] (backward compatible — legacy payloads without deferral deserialize unchanged)"
  - "1960 known_divergences row populated with deferral { reason, expected_review_date: 2026-12-31, assigned_to: external-huyen-khong-reviewer }; literal PendingExternalReview in note; tiebreaker rewritten to call center 5 the provisional *Thẩm Thị Huyền Không Học* operational value pending review"
  - "Test G FND-08 external-crate gate: test_g_1960_divergence_deferred in tests/fengshui_invariants.rs — asserts deferral presence, populated fields, our_value==5, note contains literal PendingExternalReview"
  - "ADR-0003a Consequences restructured into FND-07 (closed 16-01) / FND-08 (closed 16-02) / Backward Compatibility subsections with explicit review date, assignee, provisional tiebreaker decision, and matrix-vs-case confidence distinction"
affects:
  - phase: 17-van-khan-reviewer-closure
  - phase: 18-daily-phi-tinh
  - phase: 19-recommends-offering-integration

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Additive Option<T> marker field with #[serde(default, skip_serializing_if = \"Option::is_none\")] for typed disposition (parallels v1.5 DaySnapshot additive-only DTO convention)"
    - "DeferralMarker as typed counterpart to a free-text status flag (audit-trail over implicit-state pattern, mirrors FS-10 logging discipline)"
    - "Single-commit RED→GREEN with intermediate RED verification (v1.5 retrospective pattern, repeated from Plan 16-01)"
    - "External-crate black-box test gating a typed additive schema field (Test G, mirrors Test F from Plan 16-01)"

key-files:
  created: []
  modified:
    - "crates/amlich-core/src/almanac/fengshui/golden.rs"
    - "crates/amlich-core/src/almanac/fengshui/mod.rs"
    - "crates/amlich-core/data/almanac/flying_stars_golden.json"
    - "crates/amlich-core/tests/fengshui_invariants.rs"
    - ".planning/adrs/0003a-nien-tu-bach-polarity-confidence-closure.md"
    - ".planning/phases/16-foundation-adr-0003-confidence-closure/deferred-items.md"

key-decisions:
  - "Typed DeferralMarker struct with reason + expected_review_date + Option<assigned_to> — preserves the literal PendingExternalReview name in the 1960 row's note (human-readable) while making the disposition machine-queryable; no parallel status enum or boolean introduced"
  - "Additive Option<DeferralMarker> with #[serde(default, skip_serializing_if = \"Option::is_none\")] — backward compatible with any legacy JSON that omitted the deferral object (structurally verified)"
  - "Provisional tiebreaker values UNCHANGED: our_value=5 and expected_center=5 retained verbatim per *Thẩm Thị* matrix tiebreaker — this is a deferral, not a correction; HIGH polarity-row confidence does NOT resolve the case-level 5-vs-6 split"
  - "Single-commit RED→GREEN with intermediate RED verification (Test G confirmed RED before JSON/struct changes, GREEN after) — v1.5 retrospective pattern, same as Plan 16-01"
  - "ADR-0003a Consequences restructured into FND-07 / FND-08 / Backward Compatibility subsections so the deferred disposition, review timing, and assignee are visible at the same level as the matrix-confidence upgrade — preserves the matrix-vs-case confidence distinction"

patterns-established:
  - "Pattern: typed deferral marker on a divergence row (DeferralMarker { reason, expected_review_date, assigned_to }) as the machine-readable counterpart to a literal disposition name in the note — queryable, auditable, additive"
  - "Pattern: plan 16-N delivers the schema + populated ledger entry + test gate + ADR Consequences update as one logical unit — no separate plan needed for ADR finalization"
  - "Pattern: ADR Consequences restructured into per-requirement subsections (FND-XX) plus Backward Compatibility so each closed requirement has a co-located narrative record"

requirements-completed: [FND-08]

# Metrics
duration: 9 min
completed: 2026-07-15
---

# Phase 16 Plan 02: 1960 KnownDivergence DeferralMarker Summary

**Typed `DeferralMarker` on `KnownDivergence` + populated 1960 PendingExternalReview audit payload + Test G external-crate gate + ADR-0003a Consequences restructured for FND-08 deferred disposition (provisional tiebreaker `our_value=5` / `expected_center=5` UNCHANGED).**

## Performance

- **Duration:** 9 min 11 s
- **Started:** 2026-07-15T08:30:24Z
- **Completed:** 2026-07-15T08:39:35Z
- **Tasks:** 2 of 2 complete
- **Files modified:** 6 (4 source/test/JSON, 1 ADR, 1 deferred-items log)

## Accomplishments

- **Typed `DeferralMarker` schema field added.** `pub struct DeferralMarker { reason: String, expected_review_date: String, assigned_to: Option<String> }` is added to `crates/amlich-core/src/almanac/fengshui/golden.rs` and re-exported from `almanac::fengshui` via `mod.rs`. The `KnownDivergence` struct gains an additive `pub deferral: Option<DeferralMarker>` field with `#[serde(default, skip_serializing_if = "Option::is_none")]` — backward compatible: legacy payloads without a `deferral` object deserialize cleanly to `None` (verified by `cargo test` + structural JSON smoke test).
- **1960 PendingExternalReview audit payload populated.** The 1960 row in `crates/amlich-core/data/almanac/flying_stars_golden.json`'s `known_divergences` array now carries a fully populated `deferral` object — `reason: "phongthuycaivan.org=5 and lasotuvi.com=6 remain unresolved; independent secondary verification does not settle the center-star encoding"`, `expected_review_date: "2026-12-31"`, `assigned_to: "external-huyen-khong-reviewer"`. The row's `tiebreaker` is rewritten to explicitly call center 5 the provisional *Thẩm Thị Huyền Không Học* operational value pending review, and the `note` retains the literal disposition name `PendingExternalReview` so human readers can find it.
- **Provisional tiebreaker values UNCHANGED.** `our_value=5` in the divergence row and `expected_center=5` in the `annual-trung-nguyen-1960` case are retained verbatim from the v1.5 baseline (and from Plan 16-01's HIGH-confidence upgrade). The deferral marker is a deferral, not a correction — both source values (`phongthuycaivan.org=5` and `lasotuvi.com=6`) are preserved; the divergence is logged per FS-10, not silently corrected.
- **Test G external-crate gate added.** `test_g_1960_divergence_deferred` in `tests/fengshui_invariants.rs` imports the public types from `almanac::fengshui`, locates `KnownDivergence.case == "annual 1960"`, asserts `deferral.as_ref()` is present, asserts `reason.trim()` non-empty, asserts `expected_review_date == "2026-12-31"`, asserts `assigned_to` is `Some` with non-empty content, asserts the note contains the literal `PendingExternalReview`, and asserts `our_value == 5`. The integration test target now reports **11/11 passing tests** (9 v1.5 baseline + Test F from Plan 16-01 + Test G from Plan 16-02).
- **ADR-0003a Consequences restructured.** The single `## Consequences` section is split into three subsections: `FND-07 (closed in Plan 16-01)`, `FND-08 (closed in Plan 16-02)`, and `Backward Compatibility`. The FND-08 subsection records the PendingExternalReview disposition, the explicit `phongthuycaivan.org=5 and lasotuvi.com=6` source disagreement, the `2026-12-31` review date, the `external-huyen-khong-reviewer` assignee, the provisional *Thẩm Thị* operational value for `our_value=5` and `expected_center=5`, and the matrix-vs-case confidence distinction (HIGH polarity-row confidence does NOT resolve the 1960 center-value split). The Backward Compatibility subsection retains the explicit statement that ADR-0003 §§1–5 remain authoritative and only §6 is superseded.

## Task Commits

Each task was committed atomically (v1.5 single-commit RED→GREEN pattern):

1. **Task 1: Gate the 1960 PendingExternalReview marker RED→GREEN** - `e504fe4` (feat)
2. **Task 2: Finalize ADR-0003a Consequences for the deferred disposition** - `424010c` (docs)

## Files Created/Modified

- `crates/amlich-core/src/almanac/fengshui/golden.rs` — added `pub struct DeferralMarker { reason, expected_review_date, assigned_to }` + additive `pub deferral: Option<DeferralMarker>` field on `KnownDivergence` with `#[serde(default, skip_serializing_if = "Option::is_none")]`
- `crates/amlich-core/src/almanac/fengshui/mod.rs` — re-exported `DeferralMarker` alongside `KnownDivergence`, `GoldenConfidence`, etc. for external-crate test access
- `crates/amlich-core/data/almanac/flying_stars_golden.json` — 1960 `known_divergences` row updated with populated `deferral` object; tiebreaker rewritten to call center 5 the provisional *Thẩm Thị* operational value pending review; note retains literal `PendingExternalReview`
- `crates/amlich-core/tests/fengshui_invariants.rs` — added `Test G: test_g_1960_divergence_deferred` (imports public `KnownDivergence`, asserts deferral presence + populated fields + our_value==5 + note contains literal `PendingExternalReview`)
- `.planning/adrs/0003a-nien-tu-bach-polarity-confidence-closure.md` — Consequences section restructured into FND-07 / FND-08 / Backward Compatibility subsections; FND-08 records review date, assignee, provisional tiebreaker decision, and matrix-vs-case confidence distinction
- `.planning/phases/16-foundation-adr-0003-confidence-closure/deferred-items.md` — appended Phase 16-02 entry confirming no new clippy/fmt issues introduced (96 pre-existing → 96 after)

## Decisions Made

- **Typed `DeferralMarker` struct over a `status: String` enum or boolean.** The plan explicitly says: "Keep the literal disposition name in the 1960 JSON note and ADR so human readers can find it; do not add a parallel status enum or boolean." Structuring the marker as `{ reason, expected_review_date, assigned_to }` (rather than a status flag) gives the disposition a queryable audit trail without competing with the literal name in the note.
- **Optional `assigned_to`.** The plan's `Option<String>` shape is preserved — canonical current deferrals set it, but the field accepts `None` for purely external (unassigned) deferrals. The 1960 row sets it to `"external-huyen-khong-reviewer"`.
- **Plan-16-01 runtime-evidence-parity pattern retained.** Plan 16-02 does not need a runtime parity step because the deferral marker is a dataset-side disposition record (the runtime evidence note for `compute_yearly_flying_stars(1960, ...)` is unaffected — it already emits `confidence=high` post-Plan-16-01, which is correct: HIGH polarity-row confidence, with the case-level split logged via the deferral marker).
- **No `deny_unknown_fields` on `KnownDivergence`.** The additive `Option<DeferralMarker>` field relies on `#[serde(default)]` for backward compatibility. A `deny_unknown_fields` directive would have broken any payload that omits `deferral`, so it is intentionally absent (consistent with v1.5 additive-only DTO convention).
- **Single-commit RED→GREEN.** Following the v1.5 retrospective pattern and Plan 16-01's discipline: the failing test was written and verified RED before the struct/JSON changes were applied; both changes landed in one commit. No intermediate state was committed.

## Deviations from Plan

None — plan executed exactly as written. All must_haves truths, artifacts, and key_links satisfied:

- Truths: deferred disposition explicit ✓, our_value=5 / expected_center=5 preserved ✓, backward compat via `Option` + `#[serde(default)]` ✓, ADR Consequences records disposition + review + matrix-vs-case distinction ✓, all 9 pre-existing tests + Test F + Test G pass ✓
- Artifacts: `pub struct DeferralMarker` in golden.rs ✓, `PendingExternalReview` in JSON ✓, `test_g_1960_divergence_deferred` in fengshui_invariants.rs ✓, `PendingExternalReview` in ADR-0003a ✓
- Key links: Test G ↔ JSON via `load_flying_stars_golden` + `case == "annual 1960"` ✓; golden.rs ↔ JSON via serde `Option<DeferralMarker>` ✓; ADR-0003a ↔ JSON via matching marker name + date + assignee ✓

## Issues Encountered

None.

## Authentication Gates

None — no external service interactions required.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- **FND-08 closed.** The 1960 Trung Nguyên `KnownDivergence` is now traceable from the typed `Option<DeferralMarker>` schema to the populated 1960 row, to Test G, and to ADR-0003a's restructured Consequences — the full closed-loop audit trail that FND-08 requires. The disposition is visibly deferred (`PendingExternalReview` literal in note + JSON deferral object + ADR Consequences), not silently corrected.
- **Phase 16 complete.** Both plans (16-01 FND-07 closed, 16-02 FND-08 closed) executed; Phase 16 is ready for milestone bookkeeping (STATE.md / ROADMAP.md / REQUIREMENTS.md updates mark Phase 16 as complete, FND-07 + FND-08 as Complete).
- **v1.6 Eastern Knowledge Completion milestone** is on track: 4 phases planned (16 Foundation done; 17 Reviewer Closure, 18 Daily Phi Tinh, 19 RecommendsOffering pending). FND-07 and FND-08 were the v1.5 carry-forward tech debt items from `v1.5-MILESTONE-AUDIT.md`; both now closed.
- **Pre-existing fmt/clippy tech debt remains** (logged in `deferred-items.md`, 96 issues both before and after Phase 16-02). Recommended for a dedicated cleanup phase before v1.6 milestone close-out so future verification gates can run cleanly.

---
*Phase: 16-foundation-adr-0003-confidence-closure*
*Completed: 2026-07-15*