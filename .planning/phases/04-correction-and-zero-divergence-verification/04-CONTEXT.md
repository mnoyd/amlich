# Phase 4: Correction and Zero-Divergence Verification - Context

**Gathered:** 2026-03-02
**Status:** Ready for planning

<domain>
## Phase Boundary

Fix every divergence surfaced in Phase 3 by correcting `baseline.json` values and/or almanac source constants/logic so all `khcbppt_*.rs` validators pass with zero mismatches, while preserving existing regression behavior and keeping every correction traceable to KHCBPPT citations already present in the golden dataset.

</domain>

<decisions>
## Implementation Decisions

### Correction authority
- Golden dataset is canonical: when validator output and implementation disagree, implementation must be corrected to match KHCBPPT-cited golden entries.
- If any golden entry looks suspicious, do not force-match blindly; block and resolve the source evidence first.
- API/output contract shape must remain unchanged in this phase (behavior corrections only).
- Broad refactoring is allowed to eliminate divergence clusters, but acceptance requires a full green run: all KHCBPPT validators plus existing regression suites.
- Prefer KHCBPPT conceptual alignment over preserving legacy quirks when they conflict.
- Execute as a single coordinated correction batch (not piecemeal subsystem landings).
- Provide explicit correction notes grouped by subsystem for downstream planner/researcher use.

### Mismatch handling policy
- Phase completion requires strict zero divergence; no residual mismatches are allowed.
- Uncertain interpretation mismatches must be resolved before merge; do not defer within Phase 4 completion criteria.
- No temporary or persistent test suppression (no ignore/allowlist strategy).
- Maintain a per-mismatch correction ledger including: date, subsystem, affected entry/date, KHCBPPT citation reference, and what changed.

### Claude's Discretion
- Exact refactor structure and sequencing inside the single correction batch.
- Exact artifact format/location for the per-mismatch ledger and grouped subsystem notes.
- Exact wording style for correction notes, as long as required audit fields are present.

</decisions>

<specifics>
## Specific Ideas

- Keep correction acceptance objective and test-driven: only complete when all validators and regressions pass together.
- Treat suspect golden entries as blockers to resolve, not as exceptions to hide.

</specifics>

<code_context>
## Existing Code Insights

### Reusable Assets
- `crates/amlich-core/src/almanac/golden_loader.rs`: canonical typed loader with per-entry `khcbppt_ref` citation fields for traceability.
- `crates/amlich-core/src/almanac/data.rs`: baseline ruleset ingestion/validation path (`include_str!` + normalization + invariants) where many corrections will land.
- `crates/amlich-core/tests/khcbppt_*.rs`: subsystem validators already emit complete divergence reports across all entries.

### Established Patterns
- Golden-oracle validation pattern is already in place: validators compare implementation output from `get_day_info` against golden expected values.
- Data integrity uses assertive load-time validation (panic on invalid ruleset schema/data), so corrections must preserve these invariants.
- Test feedback is mismatch-accumulating and report-oriented (`Vec<String>` reports), enabling full-inventory verification after refactors.

### Integration Points
- Primary correction data source: `crates/amlich-core/data/almanac/baseline.json`.
- Primary correction logic modules: `crates/amlich-core/src/almanac/*.rs` (notably star/taboo/deity/truc/xung_hop/than_huong related code paths).
- Verification gate: `cargo test --package amlich-core`, including `khcbppt_*.rs` validators and pre-existing regression tests (`almanac_golden.rs`, `ruleset_determinism.rs`, `taboo_boundary.rs`).

</code_context>

<deferred>
## Deferred Ideas

None - discussion stayed within phase scope.

</deferred>

---

*Phase: 04-correction-and-zero-divergence-verification*
*Context gathered: 2026-03-02*
