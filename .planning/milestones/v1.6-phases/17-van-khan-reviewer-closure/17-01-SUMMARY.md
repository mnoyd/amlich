---
phase: 17-van-khan-reviewer-closure
plan: 01
subsystem: rituals
tags: [provenance, audit, deferral, vn-folk-ritual, rit-11, rit-14, rit-15, dec-0015, dec-0016, adr-0001]

# Dependency graph
requires:
  - phase: 12-van-khan-corpus-authoring
    provides: 60 ritual entries across 13 category sub-tables in provenance_audit.md (v1.5 baseline with bare `pending` reviewer cells)
  - phase: 16-foundation-adr-0003-confidence-closure
    provides: Typed DeferralMarker shape (Phase 16 golden.rs) — Phase 17 adapts the same deferral discipline to the 60-row ritual ledger
provides:
  - 60-row reviewer-audit ledger with 8-column header (ritual_id | classical_reference | page | confidence | reviewer | method_of_review | date_reviewed | outcome)
  - Explicit `ExternalReviewPending(...)` marker on every deferred row (60/60)
  - Closure-policy prose at top of file documenting 0/0/0/60 outcome breakdown
  - Deterministic structural shape ready for Plan 17-02 ledger-driven test parser
affects:
  - 17-02-PLAN.md: ledger-driven re-verification test (RIT-16); depends on 8-column stable shape
  - Phase 18/19: any future reviewer-record changes must preserve the 8-column contract

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Phase 16 DeferralMarker pattern adapted to Markdown ledger (ExternalReviewPending marker)"
    - "Editorial-only ledger pass preserves locked RitualEntry JSON schema (ADR-0001)"
    - "Source-provenance discipline: no fabricated reviewer identities (DEC-0015/0016)"

key-files:
  created: []
  modified:
    - crates/amlich-core/data/rituals/provenance_audit.md

key-decisions:
  - "All 60 rows dispositioned as ExternalReviewPending — no actual-name reviewer records (per source-provenance discipline DEC-0015/0016 + ADR-0001)"
  - "Method_of_review = desk-check for all 60 (audit-of-record against existing cited reference, no independent classical reviewer available)"
  - "Expected review date = 2026-12-31 across all 60 rows; assigned_to = external-vn-folk-ritual-reviewer"
  - "Outcome column = ExternalReviewPending for all 60 rows (0 confirmed / 0 corrected / 0 disputed / 60 ExternalReviewPending)"
  - "Phase 16 DeferralMarker shape adapted (Markdown marker analog) — no Rust struct change required, ledger remains the canonical record"

patterns-established:
  - "Ledger expansion pattern: when v1.5 carries a deferred field placeholder, Phase-N closure widens the ledger table with controlled-token columns and typed marker cells, without modifying the underlying JSON schema"
  - "Single-commit RED→GREEN-style discipline: editorial ledger passes use one commit per plan (no separate wip commit)"

requirements-completed: [RIT-14, RIT-15]

# Metrics
duration: 2 min
completed: 2026-07-15
---

# Phase 17 Plan 01: Reviewer-Audit Ledger Expansion Summary

**Editorial pass on `provenance_audit.md` closes RIT-14 + RIT-15 by widening every category sub-table from a 5-column to an 8-column review record and substituting the bare `pending` reviewer cell with the typed `ExternalReviewPending(...)` deferral marker on all 60 rows, with 0/0/0/60 outcome breakdown documented in the top-of-file prose.**

## Performance

- **Duration:** 2 min
- **Started:** 2026-07-15T11:27:21Z
- **Completed:** 2026-07-15T11:29:40Z
- **Tasks:** 2
- **Files modified:** 1

## Accomplishments

- All 13 category sub-tables in `provenance_audit.md` rewritten to the exact 8-column header `| ritual_id | classical_reference | page | confidence | reviewer | method_of_review | date_reviewed | outcome |` with matching 8-cell separator row.
- All 60 reviewer cells replaced with the typed `ExternalReviewPending(reason="..."; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer")` marker; zero bare `| pending |` cells remain.
- Every data row now carries `method_of_review=desk-check`, `date_reviewed=2026-07-15`, and `outcome=ExternalReviewPending` — controlled tokens stable for the Plan 17-02 test parser.
- Top-of-file prose updated to document the Phase 17 closure policy, the source-provenance rationale (DEC-0015/0016 + ADR-0001), and the 0/0/0/60 outcome breakdown.
- Bottom-of-file note dated 2026-07-15 records the closure: RIT-11 satisfied by replacing all 60 `pending` reviewer cells; no reviewer identities fabricated; no JSON schema changes.
- `cargo build -p amlich-core` and `cargo test -p amlich-core --test rituals_integration` (6/6) remain green — corpus and Rust code are untouched.

## Task Commits

Each task was committed atomically (single-commit editorial pass per Task 2's v1.5 discipline):

1. **Task 1 + 2 combined: Close RIT-14 + RIT-15 in `provenance_audit.md`** - `1777666` (docs)

**Plan metadata:** `1777666` (docs: close RIT-14 + RIT-15 — covers both Task 1 rewrite and Task 2 audit per the plan's single-commit discipline)

_Note: Task 1 and Task 2 are both satisfied by the single `1777666` commit. Task 1 produces the rewritten ledger; Task 2 audits it (Python LEDGER_AUDIT_OK 60 rows / 60 unique IDs) and commits the result. The plan's Task 2 commit instruction is "single commit" — this is intentional editorial-pass discipline._

## Files Created/Modified

- `crates/amlich-core/data/rituals/provenance_audit.md` — 60-row reviewer-audit ledger; 13 category sub-tables now 8-column; top-of-file prose documents the Phase 17 closure policy and 0/0/0/60 outcome breakdown.

## Decisions Made

- **No fabricated reviewer identities.** Per source-provenance discipline (DEC-0015/0016) and the locked `RitualEntry` JSON schema (ADR-0001), the project does not invent classical-Vietnamese reviewer names in this Claude execution. All 60 rows disposition as `ExternalReviewPending` with truthful reason and a concrete `expected_review_date=2026-12-31`.
- **`method_of_review=desk-check` is the audit-of-record convention.** Plan 17-02 (RIT-16) will introduce `independent-peer` / `cross-source` tokens for actual reviews; this ledger pass uses the controlled `desk-check` token uniformly because the deferral assessment is an audit of the existing cited reference, not an independent review.
- **Marker shape adapted from Phase 16 `DeferralMarker`.** Phase 16's Rust `DeferralMarker` carries `{ reason, expected_review_date, assigned_to }`; the ledger uses the Markdown analog `ExternalReviewPending(reason="..."; expected_review_date="..."; assigned_to="...")` so a downstream parser can read both fields without a code change.
- **Single-commit editorial pass.** v1.5 discipline (no separate wip commit); the plan's Task 2 commit instruction explicitly states "no separate wip commit allowed".

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] File-wide `ExternalReviewPending(` count is 62, not 60 — the plan's Task 1 grep `grep -c 'ExternalReviewPending(' ... | awk '{ exit ($1 == 60 ? 0 : 1) }'` will fail**
- **Found during:** Task 1 verification
- **Issue:** The plan's prose instructions replace the top-of-file paragraph with a body that itself references the marker by name once ("the `reviewer` cell (via the `ExternalReviewPending(...)` marker) and the `outcome` column") and the bottom-of-file note references it again once ("explicit `ExternalReviewPending(...)` deferral markers"). Total: 60 data-row markers + 2 prose references = 62 file-wide occurrences. The plan's own verification check `$1 == 60` would fail by design.
- **Fix:** Treat the plan's Python audit (Task 2) as the authoritative structural check. The Python script validates per-row markers (60/60), per-row cells (8/8), and per-row controlled tokens — none of which are affected by prose references. Documented here; the data invariant is correct.
- **Files modified:** none (no edit needed; the discrepancy is in the plan's verification command, not the ledger)
- **Verification:** `python3 -c "..."` reports `LEDGER_AUDIT_OK 60 rows, 60 unique IDs`; the `OK_60_METHOD_DATE_OUTCOME` grep (matching the full data-row trailing `| desk-check | 2026-07-15 | ExternalReviewPending |`) returns exactly 60. The other Task 1 grep checks (`OK_NO_BARE_PENDING` = 0 bare-pending cells; 13 header rows + 13 separator rows) all pass.
- **Committed in:** `1777666` (part of the single editorial commit)

---

**Total deviations:** 1 (informational; no data integrity issue, plan's own verification command has a 2-reference prose-side error)
**Impact on plan:** None — the ledger is in its intended shape (60 marker rows, 8 columns, controlled tokens, no bare `pending`). Plan 17-02's per-row parser invariants are met.

## Issues Encountered

- None. The Markdown rewrite is mechanical (all 60 rows get the same trailing cells); the corpus JSON and Rust schema are untouched; the build and 6-test integration suite remain green.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- The 8-column ledger with controlled tokens is in its final shape and ready for Plan 17-02's ledger-driven re-verification test (RIT-16).
- Plan 17-02 will add a Rust integration test that parses this Markdown ledger, asserts 1:1 coverage with `all_rituals()`, and asserts that any future `corrected` outcome row has its `invocation_text_vi` re-verified against the cited source.
- No blockers. Phase 17 plan 1 of 2 complete.

## Self-Check: PASSED

All claims verified:
- `.planning/phases/17-van-khan-reviewer-closure/17-01-SUMMARY.md` exists.
- `crates/amlich-core/data/rituals/provenance_audit.md` exists with 8-column shape (13 headers, 13 separators, 60 data rows).
- Commit `1777666` present in `git log` (single editorial commit per Task 2 v1.5 discipline).
- `cargo build -p amlich-core` clean; `cargo test -p amlich-core --test rituals_integration` 6/6 pass.
- `LEDGER_AUDIT_OK 60 rows, 60 unique IDs` from Python audit.

---

*Phase: 17-van-khan-reviewer-closure*
*Completed: 2026-07-15*
