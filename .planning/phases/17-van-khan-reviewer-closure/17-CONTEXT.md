# Phase 17: Văn khấn Reviewer Closure — Context

**Gathered:** 2026-07-15
**Status:** Ready for planning
**Source:** Claude's Discretion (no discuss-phase run; automated orchestration flow)

<domain>
## Phase Boundary

Phase 17 closes the v1.5 deferred RIT-11 reviewer field. The 60 ritual entries in `crates/amlich-core/data/rituals/` currently have `reviewer: pending` in `provenance_audit.md`. Phase 17 must replace every `pending` placeholder with either a real reviewer record or an explicit `ExternalReviewPending` deferral marker (matching the Phase 16 `DeferralMarker` pattern adapted for a Markdown ledger), expand the ledger columns to carry reviewer + method + date + outcome, and extend `tests/rituals_integration.rs` with a corrected-entry regression guard driven by the canonical ledger.

No JSON schema changes (ADR-0001 is locked; `RitualEntry` has `#[serde(deny_unknown_fields)]` and the body field is `invocation_text_vi`, not `body_vi`). The ledger is the canonical reviewer record.

</domain>

<decisions>
## Implementation Decisions

### Reviewer policy (BLOCKING — locked by source-provenance discipline)

- **All 60 entries get `ExternalReviewPending` markers** with truthful reason and expected review date. No fabricated reviewer identities. Rationale: the project does not have an available independent classical-Vietnamese reviewer in this Claude execution; the project's source-provenance discipline (DEC-0015/0016, ADR-0001, v1.5 PITFALLS.md) forbids inventing reviewer names or claiming reviews that did not occur. Using the explicit deferral marker — exactly as Phase 16 used `PendingExternalReview` for the 1960 Trung Nguyên divergence — preserves integrity and makes the deferral auditable.
- Marker notation (per Phase 17 RESEARCH.md Pattern 2):
  - `ExternalReviewPending(reason="<truthful reason>"; expected_review_date="<YYYY-MM-DD>"; assigned_to="<optional assignee>")`
- `method_of_review` for deferred rows = `desk-check` (the audit itself is a desk-check of the existing cited record; not `independent-peer`).
- `date_reviewed` for deferred rows = 2026-07-15 (date of the audit/deferral assessment).
- `expected_review_date` for all 60 entries = 2026-12-31 (user-specified ritual-review window; project chooses a single uniform date so audit counts are deterministic).
- `assigned_to` is set to `external-vn-folk-ritual-reviewer` for all 60 (a role-style identifier chosen by the project to own the deferral, matching the Phase 16 `external-huyen-khong-reviewer` role pattern).
- Outcome column for all 60 rows = `ExternalReviewPending`.

### Ledger format (locked — Pattern 2 from RESEARCH.md)

- Eight-column table per category: `ritual_id | classical_reference | page | confidence | reviewer | method_of_review | date_reviewed | outcome`
- Top-of-file prose updated to describe the closure policy and the count breakdown (60 rows, all `ExternalReviewPending`).
- No legacy `pending` placeholder remains anywhere in the file.

### Corrected-entry test (locked — Pattern 3 from RESEARCH.md)

- Extend `tests/rituals_integration.rs` with a test-only ledger parser (`include_str!` of `provenance_audit.md`) that returns `corrected_ids` from `outcome=corrected` rows.
- Add `every_corrected_entry_passes_schema_and_nfc_round_trip` modeled on existing Test 5.
- Add `every_ledger_row_passes_invariants` as a sibling test that asserts: all 60 corpus IDs appear in ledger exactly once, every reviewer cell is the real-name notation or `ExternalReviewPending(...)` marker, every deferral marker has non-empty reason and expected_review_date, every method is one of {`independent-peer`, `cross-source`, `desk-check`}, every outcome is one of {`confirmed`, `corrected`, `disputed`, `ExternalReviewPending`}, and outcome counts sum to 60.
- Because all 60 rows are deferred (no real reviews occurred), the corrected-ID set is initially empty. The test must still parse the ledger successfully and assert `corrected_count == 0` rather than passing vacuously (Pitfall 7 from RESEARCH.md).

### Body field mapping (locked — RESEARCH.md Pitfall 5)

- `body_vi` in REQUIREMENTS.md and ROADMAP.md is the domain name for the locked Rust field `invocation_text_vi`. Plans and tests use `invocation_text_vi` in code; refer to it as "the ritual body" in editorial prose.
- No `body_vi` schema field is added.

### Backward compatibility (locked)

- `RitualEntry` JSON unchanged. No new fields, no `#[serde(...)]` attribute changes, no schema version bump.
- Existing v1.5 fixtures still load unchanged (they already have `invocation_text_vi` populated; no edits are made to any corpus JSON entry in Phase 17).

### Don't Hand-Roll (locked — RESEARCH.md "Don't Hand-Roll" table)

- No new `reviewer` or `outcome` field in JSON.
- No duplicated `CORRECTED_RITUAL_IDS` constant in Rust — parse from ledger.
- No direct filesystem reads in tests — use `all_rituals()` and the loader.
- No new external dependency.

## Claude's Discretion

The reviewer policy itself (defer-all) is Claude's Discretion under the automated orchestration flow. The user may override by editing this CONTEXT.md before executing plans, or by overriding during plan execution.

- Specific deferral date (2026-12-31) and assignee (`external-vn-folk-ritual-reviewer`) are Claude's Discretion chosen to mirror Phase 16's deferral pattern while remaining distinct from it.
- The Markdown table column order, exact marker syntax, and method/date values for deferred rows are Claude's Discretion, anchored on Phase 16 `DeferralMarker` shape and Phase 12 ledger format.

</decisions>

<specifics>
## Specific Ideas

- Use the exact marker name `ExternalReviewPending` (capitalized, no spaces). This is the stable fourth outcome token per ROADMAP.md success criterion #3.
- Marker syntax uses parentheses with `key="value"` pairs separated by `;` — matches the Rust `DeferralMarker` struct shape (`reason`, `expected_review_date`, `assigned_to`).
- Ledger prose should explicitly state the deferral policy so a reader can find it without parsing the tables.
- Test file extends `tests/rituals_integration.rs` (do not create a second integration target).
- Test parser must validate the table header before consuming rows; reject malformed rows; assert 60 unique IDs; assert ledger/corpus ID parity via `all_rituals()` IDs.
- The Phase 17 research note about the 1960 Phi Tinh divergence is in Phase 16 — Phase 17 does NOT reuse Phase 16's specific date `2026-12-31` and assignee `external-huyen-khong-reviewer`. Phase 17 uses ritual-specific values chosen by Claude's Discretion.

</specifics>

<deferred>
## Deferred Ideas

None — Phase 17 scope is bounded by RIT-14, RIT-15, RIT-16. No further deferrals in scope.

Out-of-scope items already declared (carry-forward from REQUIREMENTS.md):
- Real reviewer identification (requires user domain contact).
- Body text edits to entries not flagged `corrected` (no review evidence available).
- Migration of `RitualEntry.notes` into a status channel (kept editorial; not a status field).

</deferred>

---

*Phase: 17-van-khan-reviewer-closure*
*Context gathered: 2026-07-15 via Claude's Discretion (automated orchestration flow)*