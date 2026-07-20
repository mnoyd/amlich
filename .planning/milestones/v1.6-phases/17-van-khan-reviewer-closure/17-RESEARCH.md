# Phase 17: Văn khấn Reviewer Closure — Research

**Researched:** 2026-07-15
**Domain:** Editorial review closure for the Vietnamese ritual corpus, Markdown provenance ledger, and black-box corpus regression tests
**Confidence:** HIGH for repository structure and test mechanics; MEDIUM for the ledger policy; LOW for reviewer identity and source-verification outcomes until the user supplies the decision and available evidence

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| RIT-14 | Every ritual entry has a reviewer value: an actual reviewer identity with name, date, and outcome, or an explicit `ExternalReviewPending` marker with reason and expected review date. | The 60-row `provenance_audit.md` ledger is the canonical reviewer record. Keep reviewer data out of `RitualEntry` JSON because ADR-0001 locks the schema and `RitualEntry` rejects unknown fields. Use a documented reviewer/deferral notation and replace every `pending` cell. |
| RIT-15 | The audit ledger records reviewer identity, method (`independent-peer` / `cross-source` / `desk-check`), review date, and outcome (`confirmed` / `corrected` / `disputed`). | Extend the existing five-column ledger table with `method_of_review`, `date_reviewed`, and `outcome`; retain `reviewer` as the identity-or-marker column. Use exact controlled tokens so tests and future tooling can count outcomes without interpreting prose. |
| RIT-16 | Any `corrected` entry has its ritual body re-verified against `original_citation` and passes the existing schema and NFC-at-load guards. | Treat the ledger's `outcome=corrected` rows as the source of the corrected-ID set. A test-only `include_str!` reader can enumerate those rows, locate each ID through `all_rituals()`, and apply the existing `RitualEntry` serde round-trip plus loaded-text/NFC assertions. The locked Rust body field is `invocation_text_vi`, not `body_vi`; this is the field that must be re-verified against the cited source. |
</phase_requirements>

## Summary

Phase 17 is primarily a 60-row editorial closure, not a Rust feature. The corpus already contains exactly 60 entries across 13 JSON files, and `provenance_audit.md` already has a one-to-one ledger with the corpus. Every current reviewer cell is `pending`, so the phase must replace that placeholder with either a real, named review record or an explicit `ExternalReviewPending` disposition. The reviewer record belongs in the Markdown ledger: ADR-0001 locks the `RitualEntry` shape, `RitualEntry` has `#[serde(deny_unknown_fields)]`, and the roadmap explicitly says no JSON schema change is expected.

The central planning decision is whether the project can name actual reviewers. Do not invent a scholar, reviewer name, review date, or source-verification result. If no external classical-Vietnamese reviewer is available, the planner must route the user to choose a deferral policy, an author-attributed `desk-check` policy, or a hybrid. A `desk-check` by a named project author may document an actual editorial pass, but it must not be represented as an independent peer review. Rows that cannot honestly claim a completed review should use `ExternalReviewPending` with a reason and an expected date.

RIT-16 needs one source of truth for corrected entries. The recommended design is to parse the canonical Markdown ledger in the integration test, rather than add a `reviewer`, `outcome`, or `corrected` field to JSON and rather than duplicate corrected IDs in Rust. The test can then enumerate exactly the rows marked `corrected`, load those entries through the existing `OnceLock` corpus path, and assert the same schema/NFC round-trip behavior already established by Test 5.

**Primary recommendation:** First obtain the reviewer-policy decision; then execute two plans: 17-01 updates all 60 ledger rows and closes RIT-14/RIT-15, while 17-02 re-verifies any corrected `invocation_text_vi` values, updates JSON only when evidence requires a correction, and adds a ledger-driven corrected-entry regression guard for RIT-16.

## User Decision Required Before Planning

### Reviewer identity and review policy

This is the phase's blocking context decision. The repository does not contain an available classical-Vietnamese reviewer identity. The planner must ask the user to choose one of these policies:

1. **Named external reviewers:** provide real reviewer names and actual review dates for entries reviewed by an independent classical-Vietnamese reviewer. Use `independent-peer` only when that review actually occurred.
2. **Explicit deferral:** use `ExternalReviewPending` for entries lacking a real independent reviewer. The marker must include a truthful reason and an expected review date; do not copy Phase 16's `2026-12-31` date unless the user chooses that date for this review.
3. **Hybrid:** use named project-author `desk-check` records for entries the author actually inspected against the cited source, and `ExternalReviewPending` for entries that still require independent external review. This closes the audit field without mislabeling a desk-check as peer review.

A role, handle, or invented placeholder such as `project-author`, `reviewer-1`, or `pending` is not an actual reviewer identity for RIT-14. The actual value must identify a real person by name. The user also needs to choose who is assigned to deferred reviews, if anyone, and the expected review date. `assigned_to` is recommended in the marker because it follows the Phase 16 pattern, although RIT-14 explicitly requires only reason and expected date.

### Outcome policy

Use these exact outcome tokens in the ledger:

- `confirmed`: review found no required correction.
- `corrected`: review found a source-backed textual correction, and the corrected corpus entry was re-verified against its cited source.
- `disputed`: a completed review found an unresolved disagreement; preserve the disagreement and cite it rather than silently selecting a value.
- `ExternalReviewPending`: no completed independent review can be claimed; the reviewer column carries the deferral marker.

The first three tokens are the RIT-15 review outcomes. `ExternalReviewPending` is additionally required by the roadmap's stable outcome-count criterion. It should be treated as the fourth ledger disposition, not hidden in free text.

For a deferred row, `date_reviewed` should mean the date the current audit/deferral assessment was recorded, not a falsely implied date of external review. `expected_review_date` belongs inside the marker. `method_of_review` should be `desk-check` only if a real desk-check occurred; if no review action occurred at all, the planner must resolve how the project wants to satisfy the required method column rather than inventing one.

## Current State

### Corpus and schema

- `crates/amlich-core/data/rituals/` contains 13 category JSON files plus `manifest.json`.
- `all_rituals()` merges the 13 embedded files through `OnceLock` and `include_str!`, validates `$schema_version`, validates the source ID, and normalizes text to NFC at load (`crates/amlich-core/src/rituals/corpus.rs:85-169`).
- The loaded corpus has exactly 60 entries, and Phase 12 verification confirmed one ledger row for every corpus ID (`12-VERIFICATION.md:20-26`, `12-VERIFICATION.md:47-49`).
- `RitualEntry` is locked by ADR-0001 and has `#[serde(deny_unknown_fields)]` (`crates/amlich-core/src/rituals/schema.rs:127-150`). Adding `reviewer`, `review_method`, `review_date`, `outcome`, or a corrected marker to a JSON entry would be a schema change and would reject the current corpus unless a superseding ADR and migration were introduced.
- The schema's Vietnamese prayer-body field is `invocation_text_vi` (`schema.rs:139`); there is no `body_vi` field. Treat `body_vi` in RIT-16 and the phase description as the domain name for `invocation_text_vi`.
- `body_en` is a reserved optional field and must remain unrelated to this phase (`schema.rs:140-142`).

### Audit ledger

The current ledger has these columns:

```markdown
| ritual_id | classical_reference | page | confidence | reviewer |
|---|---|---|---|---|
```

It already has all 60 IDs exactly once, but every reviewer value is `pending` (`crates/amlich-core/data/rituals/provenance_audit.md:1-9`, rows through line 164). The ledger is explicitly described as the canonical record for the deferred peer-review work, so Phase 17 should update it in place rather than introduce a parallel reviewer database or JSON file.

### Existing integration guard

`crates/amlich-core/tests/rituals_integration.rs:155-169` is Test 5, `every_entry_round_trips_byte_equal_through_serde_json`. It serializes every loaded `RitualEntry`, deserializes it through the public schema, serializes again, and requires byte equality. The corpus loader's `normalize_and_validate` path and the inline corpus test `every_text_field_is_nfc_normalized` provide the NFC-at-load behavior (`corpus.rs:119-169`, `corpus.rs:203-224`). Phase 17 should extend this test file rather than create a second ritual integration target.

## Standard Stack

### Existing components

| Component | Version / location | Purpose | Recommendation |
|-----------|--------------------|---------|----------------|
| Markdown ledger | `data/rituals/provenance_audit.md` | Canonical reviewer and disposition record | Keep as the only source of reviewer status; extend its table columns. |
| Locked ritual schema | ADR-0001, `src/rituals/schema.rs` | Validates corpus JSON | Do not modify for reviewer metadata. |
| `OnceLock` + `include_str!` | `src/rituals/corpus.rs:85-116` | Compile-time corpus loading | Reuse through `all_rituals()`; no runtime file discovery. |
| `serde` / `serde_json` | Workspace `1.0` | `RitualEntry` schema deserialization and round-trip | Use the existing public test pattern. |
| `unicode-normalization` | `0.1.25` | NFC normalization in the loader | Rely on the loader's existing normalization and, if the new test needs explicit assertions, use the already-declared crate dependency. |
| External black-box integration test | `tests/rituals_integration.rs` | Consumer-facing corpus and schema guard | Add the corrected-entry test here, matching the v1.5 convention. |

No new Cargo dependency, production type, loader path, or public API is needed. The ledger parser is test-only and should consume a compile-time `include_str!` of the existing Markdown file.

### Validation commands

- Quick phase test: `cargo test -p amlich-core --test rituals_integration`
- Full crate gate: `cargo test -p amlich-core`
- Build gate: `cargo build -p amlich-core`

The project configuration does not enable `workflow.nyquist_validation`, so the formal Nyquist section is skipped. The Phase 12 and Phase 16 verification patterns still support the commands above as the practical gates.

## Architecture Patterns

### Pattern 1: Ledger-only reviewer metadata

**What:** Keep reviewer identity, method, date, outcome, and deferral details in `provenance_audit.md`; leave all corpus JSON entries and `RitualEntry` fields unchanged.

**Why:** The roadmap explicitly makes the ledger canonical for reviewer information. The schema is locked, and `deny_unknown_fields` makes an unplanned JSON field an immediate load failure. The Phase 12 precedent already treated the ledger as the provenance record rather than a runtime data structure.

**When to use:** Always for RIT-14 and RIT-15 in this phase.

### Pattern 2: Explicit table columns with controlled tokens

Replace each category table header with:

```markdown
| ritual_id | classical_reference | page | confidence | reviewer | method_of_review | date_reviewed | outcome |
|---|---|---|---|---|---|---|---|
```

Recommended actual reviewer notation in the `reviewer` column:

```text
<Full reviewer name> — <YYYY-MM-DD> — <confirmed|corrected|disputed>
```

The separate `method_of_review`, `date_reviewed`, and `outcome` columns are intentionally redundant with the reviewer value's identity/date/outcome. The redundancy satisfies RIT-14's human-readable reviewer field while providing stable machine-readable columns for RIT-15 and outcome counts.

Recommended deferred notation in the `reviewer` column:

```text
ExternalReviewPending(reason="<truthful reason>"; expected_review_date="<YYYY-MM-DD>"; assigned_to="<optional assignee>")
```

For an unassigned deferral, omit `assigned_to` rather than writing a placeholder. The marker name must remain exactly `ExternalReviewPending`; its reason and expected date must be non-empty. The ledger prose should define that the marker is a disposition, not a reviewer identity, and that the `outcome` column repeats `ExternalReviewPending` for stable counting.

### Pattern 3: Ledger-driven corrected-entry test

**What:** Embed the Markdown ledger in the integration test and derive corrected IDs from rows whose `outcome` cell equals `corrected`.

**Why:** The roadmap says the audit ledger is canonical, and a duplicated Rust list would create a second source of truth. Adding a `corrected` marker to `RitualEntry` would violate the no-schema-change constraint. A small test-only parser is sufficient because the ledger format is deliberately constrained to pipe-delimited tables with a fixed header.

**Suggested test-only shape:**

```rust
const PROVENANCE_AUDIT: &str = include_str!("../data/rituals/provenance_audit.md");

fn corrected_ritual_ids() -> Vec<&str> {
    let mut ids = Vec::new();
    for line in PROVENANCE_AUDIT.lines() {
        let cells: Vec<_> = line.split('|').map(str::trim).collect();
        if cells.len() >= 9 && cells[1] != "ritual_id" && cells[8] == "corrected" {
            ids.push(cells[1]);
        }
    }
    ids
}
```

The exact implementation should also validate the header, skip separator rows, reject malformed rows, reject duplicate IDs, and assert that the ledger ID set equals the `all_rituals()` ID set. The snippet only demonstrates the source-of-truth relationship; it is not a complete parser.

The corrected-entry guard should then follow Test 5:

```rust
#[test]
fn every_corrected_entry_passes_schema_and_nfc_round_trip() {
    let rituals = all_rituals();
    let corrected = corrected_ritual_ids();
    for ritual_id in corrected {
        let entry = rituals
            .iter()
            .find(|entry| entry.ritual_id == ritual_id)
            .unwrap_or_else(|| panic!("corrected ledger ID {ritual_id} is absent from the corpus"));
        let first = serde_json::to_string(entry).expect("serialize corrected entry");
        let parsed: amlich_core::rituals::RitualEntry =
            serde_json::from_str(&first).expect("deserialize corrected entry");
        let second = serde_json::to_string(&parsed).expect("re-serialize corrected entry");
        assert_eq!(first, second, "corrected ritual {ritual_id} did not round-trip");
    }
}
```

The final test should not silently pass because the parser found no rows due to a malformed table. Assert that every table has the expected header and that all 60 corpus IDs appear once. If the user chooses no corrections, an explicit `corrected_count == 0` result is acceptable; the parser must still prove the ledger was read and validated.

`all_rituals()` loads the complete corpus before the corrected entry is selected, so the existing schema and NFC-at-load guards run for all entries. The new test adds corrected-row traceability and the existing public serde round-trip contract. If a stronger direct assertion is desired, check `is_nfc` on `title_vi`, `invocation_text_vi`, offerings, preparation steps, and notes for the selected entries using the already-declared `unicode-normalization` dependency.

### Pattern 4: Correction traceability before JSON edits

For every row assigned `corrected`:

1. Read the cited `original_citation` title, edition, and page.
2. Compare the source passage to `invocation_text_vi` and record what changed in the ledger's surrounding documentation or the entry's existing `notes` without adding a new schema field.
3. Update the JSON body only when the source-backed difference is established.
4. Preserve `ritual_id`, event keys, variant, source ID, citation, and confidence unless the review evidence specifically requires a corresponding editorial update.
5. Run the corpus loader and corrected-entry test after the edit.

If the cited source cannot be accessed or the page is insufficient to establish the correction, do not mark the row `corrected`; use `disputed` when a completed reviewer found an unresolved conflict, or `ExternalReviewPending` when no valid review can be claimed.

## Audit-Ledger Format

### Required column headers

Every category table should use the same eight columns, in the same order:

```markdown
| ritual_id | classical_reference | page | confidence | reviewer | method_of_review | date_reviewed | outcome |
|---|---|---|---|---|---|---|---|
```

The existing category headings and `Source file` lines can remain. The top-of-file summary should be updated from the v1.5 statement that all review is pending to describe the closure policy and the counts of `confirmed`, `corrected`, `disputed`, and `ExternalReviewPending` rows.

### Field semantics

| Field | Required value |
|-------|----------------|
| `ritual_id` | Exact ID from the corpus JSON; unique once across the complete ledger. |
| `classical_reference` | Existing source description; retain it unless the review discovers a citation error. |
| `page` | Existing cited page; use the same value as `original_citation.page` unless corrected by source evidence. |
| `confidence` | Existing `primary`, `regional-variant`, or `synthesized` tier; not the review outcome. |
| `reviewer` | Either `<real name> — <date> — <outcome>` or the exact `ExternalReviewPending(...)` marker. No bare `pending`. |
| `method_of_review` | Exactly `independent-peer`, `cross-source`, or `desk-check`. Do not call a self-review `independent-peer`. |
| `date_reviewed` | ISO date of the completed review or current audit/deferral assessment. |
| `outcome` | Exactly `confirmed`, `corrected`, `disputed`, or `ExternalReviewPending`. |

### Deferral rules

- The marker reason must explain why an actual reviewer outcome is unavailable, for example the absence of an independent classical-Vietnamese reviewer or inaccessible source evidence.
- `expected_review_date` must be a real future date selected by the user, not a vague placeholder such as `TBD`.
- `assigned_to` is optional, but if present it must identify a real owner or an explicit external-review role selected by the user.
- The marker must appear in the `reviewer` cell and `ExternalReviewPending` must also appear in `outcome`.
- The method and date columns must describe the work actually done. A deferral should not claim `independent-peer`; use `desk-check` only when a desk-check was performed.
- The prose below the tables should define the marker and state that no pending placeholder remains.

## Requirement-Specific Research Support

### RIT-14: reviewer field closure

The Phase 12 ledger already establishes the one-row-per-ID structure and the Phase 12 verification proves 60/60 coverage. The Phase 17 change is editorial replacement of the reviewer cell, not a corpus-schema change. The completion check should assert:

- all 60 corpus IDs are present exactly once;
- no reviewer cell equals `pending` or is blank;
- each reviewer cell matches either the actual-name notation or `ExternalReviewPending` notation;
- every deferred marker has a non-empty reason and expected review date;
- the outcome column agrees with the reviewer cell's disposition.

The user decision about real reviewer identities is part of satisfying this requirement. Automated checks can validate shape and coverage, but they cannot establish that a name is a real scholar or that a review actually occurred.

### RIT-15: complete per-row audit record

The existing ledger already contains citation and confidence information. Add the three review columns and use controlled values. The review method captures the evidence path:

- `independent-peer`: a real independent reviewer assessed the entry;
- `cross-source`: the reviewer compared the entry against more than one cited or authoritative source;
- `desk-check`: a named author/editor checked the entry against the cited record without claiming independent peer status.

`date_reviewed` and the date inside an actual reviewer string must match. For a deferral, use the date of the deferral assessment in the table and the separate expected future date in the marker. A test can count each outcome from the ledger and verify that the counts sum to 60.

### RIT-16: corrected body verification and regression guard

The requirement refers to `body_vi`, but the locked `RitualEntry` field is `invocation_text_vi`. The planner should use that exact field in source-comparison instructions and test failure messages. The original citation is already structured as `SourceCitation { title, publisher, edition, page }`, with the page optional in the Rust type but populated in the current corpus and required by the v1.5 provenance discipline.

The test cannot prove that a human compared text to a physical or external source. It can prove the traceable mechanical half: every ledger-marked corrected ID resolves to exactly one loaded entry, the entry passes the locked serde schema, the loader has run its NFC normalization, and the entry round-trips byte-equal through serde JSON. The human source comparison must be documented in the ledger review record and performed before the row is marked `corrected`.

## Dependencies on Prior Phases

| Prior phase | Provides | Phase 17 use |
|-------------|----------|--------------|
| Phase 10 — foundation/schema lock | ADR-0001; `RitualEntry` locked field set; source/citation types; `deny_unknown_fields` | Prohibits reviewer fields in JSON and defines `invocation_text_vi`, `original_citation`, and schema-round-trip expectations. |
| Phase 11 — module and lookup APIs | Public `all_rituals()`; `RitualEntry` re-exports; external-crate test convention | Supplies the consumer-facing integration-test surface and the existing Test 5 pattern. |
| Phase 12 — corpus authoring | 60 entries; 13 category files; ledger; citations/pages; NFC-at-load loader behavior | Supplies the 60 IDs, current table layout, and audit rows to update. |
| Phase 16 — ADR-0003 confidence closure | `DeferralMarker { reason, expected_review_date, assigned_to }`; explicit `PendingExternalReview` human-readable marker; no-silent-correction discipline | Provides the reference notation and disposition vocabulary. Apply the pattern in Markdown only; do not add a ritual JSON schema field. |

## Don't Hand-Roll

| Problem | Don't build | Use instead | Why |
|---------|-------------|-------------|-----|
| Reviewer metadata storage | A new `reviewer` or `outcome` field in `RitualEntry` JSON | The existing Markdown ledger | ADR-0001 locks the schema and JSON unknown fields fail loading. |
| Deferred status | A bare `pending` string, boolean, or ambiguous prose | `ExternalReviewPending(reason=...; expected_review_date=...; assigned_to=...)` | Phase 16 shows that an explicit disposition with reason/date/owner is auditable and queryable. |
| Corrected-entry selection | A hand-maintained `CORRECTED_RITUAL_IDS` constant | Parse `outcome=corrected` from the embedded canonical ledger | Avoids drift between ledger and test. |
| Corpus loading | Direct filesystem reads from the test or production code | `all_rituals()` and the existing `OnceLock` + `include_str!` loader | Preserves compile-time embedding and exercises the actual schema/NFC path. |
| Source comparison result | A new status enum in the locked corpus schema | Ledger outcome plus citation and review method | This phase is editorial and explicitly excludes schema changes. |
| Markdown infrastructure | A new database or general-purpose audit service | Fixed pipe-table format and a small test-only parser | The ledger is intentionally simple; adding infrastructure would create a second source of truth and unnecessary dependency risk. |

**Key insight:** The Phase 16 deferral pattern is useful as an audit notation and policy pattern, not as a reason to expand the ritual schema. The canonical source for ritual reviewer disposition is the ledger; the canonical source for ritual content remains the JSON corpus.

## Common Pitfalls

### Pitfall 1: Claiming an unavailable reviewer

**What goes wrong:** The ledger uses a plausible scholar name, a generic role, or the author as an independent peer without a real review event.

**How to avoid:** Require the user to supply actual identities and choose the desk-check/deferral policy. Use `ExternalReviewPending` where evidence is unavailable.

**Warning sign:** A reviewer cell contains `project-author`, `external-reviewer`, or another role rather than a person's name.

### Pitfall 2: Leaving `pending` hidden in prose

**What goes wrong:** A row's reviewer cell is updated, but the top-level ledger note or a table still contains the v1.5 `pending` placeholder, making automated closure ambiguous.

**How to avoid:** Replace all 60 cells, update the header prose, and test that the old standalone placeholder is absent. The only pending token should be the explicit `ExternalReviewPending` disposition.

### Pitfall 3: Mixing corpus confidence with review outcome

**What goes wrong:** `primary`, `regional-variant`, and `synthesized` are overwritten with `confirmed` or `corrected`, losing the original provenance tier.

**How to avoid:** Keep `confidence` unchanged as the corpus provenance tier and add a separate `outcome` column for review disposition.

### Pitfall 4: Marking an entry corrected without source traceability

**What goes wrong:** `invocation_text_vi` is edited based on intuition or a secondary paraphrase, then marked `corrected` without checking the exact cited title/edition/page.

**How to avoid:** Require a source comparison before the outcome changes to `corrected`; otherwise use `disputed` or `ExternalReviewPending`.

### Pitfall 5: Treating `body_vi` as a Rust field

**What goes wrong:** A plan or test searches for a nonexistent `body_vi` field or adds one to the locked schema.

**How to avoid:** Use `invocation_text_vi` in code and refer to it as the ritual body in editorial instructions.

### Pitfall 6: Duplicating corrected IDs in the test

**What goes wrong:** A Rust constant lists corrected IDs, but a later ledger edit changes the set and the test still passes against the old list.

**How to avoid:** Embed and parse the ledger; assert exact ledger/corpus ID parity and validate all outcome tokens.

### Pitfall 7: A vacuous corrected-entry test

**What goes wrong:** The parser fails to recognize the repeated Markdown tables, returns zero IDs, and the test passes without checking any corrected entry.

**How to avoid:** Validate every table header, reject malformed data rows, assert 60 unique IDs, and report the corrected count. A zero corrected count is valid only after the ledger was successfully parsed and counted.

### Pitfall 8: Assigning a future date copied from Phase 16

**What goes wrong:** Ritual review deferrals inherit `2026-12-31` and `external-huyen-khong-reviewer` even though those values belong to the 1960 Phi Tinh divergence.

**How to avoid:** Ask the user for a ritual-specific expected date and assignee. Use Phase 16 only as a shape reference.

### Pitfall 9: Markdown table drift across categories

**What goes wrong:** One category retains the old five-column header or changes column order, so a simple test parser reads incorrect cells.

**How to avoid:** Rewrite all repeated table headers to the exact eight-column format and validate headers before consuming rows.

## Plan Decomposition Recommendation

### Plan 17-01 — Audit-of-record pass and ledger closure

**Scope:** editorial ledger work for RIT-14 and RIT-15.

1. Obtain the reviewer-policy decision and expected deferral date/assignee.
2. Inventory all 60 corpus IDs against the current ledger before editing.
3. For each row, assign a truthful named reviewer record or `ExternalReviewPending` marker.
4. Add `method_of_review`, `date_reviewed`, and `outcome` columns to every category table.
5. Preserve existing citation, page, and confidence values unless a review finds a documented citation error.
6. Update the ledger summary, definitions, and references so `pending` is no longer presented as the accepted state.
7. Run a deterministic 60-row coverage/outcome audit before handing off to Plan 17-02.

**Plan boundary:** Do not add reviewer fields to JSON or `schema.rs`. Do not mark a row `corrected` unless Plan 17-02 can re-verify the body against the cited source.

### Plan 17-02 — Corrected-entry source re-verification and test guard

**Scope:** RIT-16 and the mechanical closure of ledger/corpus consistency.

1. Enumerate ledger rows with `outcome=corrected` from the canonical Markdown ledger.
2. For each corrected row, compare `invocation_text_vi` with the exact `original_citation` and record the review outcome using the chosen ledger notation.
3. Apply only source-backed JSON corrections; preserve the locked entry shape and NFC-compatible text.
4. Extend `tests/rituals_integration.rs` with a test-only ledger parser and a corrected-entry test modeled on Test 5.
5. Assert exact corpus/ledger ID parity, valid methods/outcomes, non-empty deferral fields, and no legacy `pending` placeholder.
6. For each corrected ID, load via `all_rituals()`, assert the entry exists, deserialize/serialize byte-equal, and rely on or explicitly check the NFC-at-load invariants.
7. Run the ritual integration target, the full `amlich-core` suite, and the build gate.

**Why two plans:** The reviewer-policy decision and 60-row ledger assignment are independent editorial work. Corrected content changes and the test require the final outcome set, so they should follow the ledger pass rather than be interleaved with it.

## State of the Art

| v1.5 state | Phase 17 target | Impact |
|------------|-----------------|--------|
| Every ledger reviewer cell is `pending` | Every row has a real reviewer record or explicit `ExternalReviewPending` | No ambiguous placeholder remains; deferrals are intentional and dated. |
| Reviewer metadata exists only as a single `reviewer` column | Fixed eight-column audit row with reviewer, method, date, and outcome | Human-readable identity remains available while outcome counts become stable. |
| Test 5 covers every entry's generic JSON round-trip | Corrected-entry test selects IDs from the ledger and applies the same guard | RIT-16 is tied to the audit disposition rather than a duplicated test list. |
| `RitualEntry` contains no review state | It remains unchanged | Backward compatibility and ADR-0001 schema lock are preserved. |
| Body terminology in the requirement says `body_vi` | Code uses `invocation_text_vi` | The plan must map editorial body review to the actual locked field. |

## Open Questions

1. **Which reviewer policy does the user authorize?**
   - Known: no in-house classical-Vietnamese scholar is available in this session.
   - Unclear: named external review, author `desk-check`, deferral, or hybrid.
   - Recommendation: block Plan 17-01 until answered; never fabricate identity or review evidence.

2. **What expected review date and assignee should deferrals carry?**
   - Known: Phase 16 uses `reason`, `expected_review_date`, and optional `assigned_to`.
   - Unclear: a ritual-specific date and owner.
   - Recommendation: user chooses them; do not reuse Phase 16 values automatically.

3. **How should a deferred row satisfy method/date columns?**
   - Known: RIT-15 requires a method and date on each row, while RIT-14 allows a deferral marker.
   - Unclear: whether the project considers the ledger audit itself a `desk-check`, or wants a separate policy for rows with no review activity.
   - Recommendation: use `desk-check` only for a real editorial inspection and record the assessment date; otherwise obtain a user decision before assigning the token.

4. **Are all cited original sources and pages accessible for correction review?**
   - Known: every current JSON entry has a citation page and the ledger mirrors it.
   - Unclear: whether the reviewer can inspect the cited editions during this session.
   - Recommendation: only mark `corrected` after the exact source passage is available; use `disputed` or `ExternalReviewPending` when traceability cannot be established.

5. **Should the integration test validate every ledger row or only corrected rows?**
   - Known: RIT-16 explicitly needs corrected-row enumeration, and the roadmap requires stable outcome counts.
   - Recommendation: implement one shared test-only parser that validates all 60 rows and returns corrected IDs; add the corrected-entry guard on top of it. This avoids drift without exposing a production audit API.

6. **Should a correction be recorded in an existing `notes` value?**
   - Known: `RitualEntry.notes` is an existing editorial field, but the ledger is the canonical review record.
   - Recommendation: do not add a new schema marker. Keep the required review outcome and source trace in the ledger; change `notes` only if the source-backed editorial note is useful to corpus consumers and does not turn notes into a status channel.

## Validation Architecture

`workflow.nyquist_validation` is not present in `.planning/config.json`, so the formal Nyquist validation section is not enabled. The phase should nevertheless use these gates:

| Requirement | Behavior | Test type | Automated command | File |
|-------------|----------|-----------|-------------------|------|
| RIT-14 | Ledger covers all 60 IDs; every reviewer cell is an actual-name record or a populated deferral marker; no legacy `pending` placeholder | Integration / ledger invariant | `cargo test -p amlich-core --test rituals_integration` | `tests/rituals_integration.rs` plus `provenance_audit.md` |
| RIT-15 | Every row has valid method, date, and outcome; outcome counts sum to the 60 ledger rows | Integration / ledger invariant | `cargo test -p amlich-core --test rituals_integration` | `tests/rituals_integration.rs` plus `provenance_audit.md` |
| RIT-16 | Every `outcome=corrected` ID exists in the corpus and passes schema/NFC load plus byte-equal serde round-trip | Integration / regression | `cargo test -p amlich-core --test rituals_integration every_corrected_entry_passes_schema_and_nfc_round_trip` | `tests/rituals_integration.rs` |

### Sampling rate

- Per ledger edit: run the ledger-driven integration test.
- Per plan: run `cargo test -p amlich-core --test rituals_integration` and `cargo test -p amlich-core`.
- Phase gate: run the full crate test suite and `cargo build -p amlich-core` before verification.

### Wave 0 gaps

- No production schema gap: ADR-0001 already locks the required shape.
- No corpus loader gap: `all_rituals()` already performs schema parsing, source validation, and NFC normalization.
- Test gap: the audit-ledger parser and corrected-entry guard do not yet exist and belong in Plan 17-02.

## Sources

### Primary (HIGH confidence — repository evidence)

- `.planning/ROADMAP.md:55-71` — Phase 17 goal, no-schema-change constraint, success criteria, and two-plan decomposition.
- `.planning/REQUIREMENTS.md:16-20` — formal RIT-14, RIT-15, and RIT-16 requirements.
- `.planning/PROJECT.md:48-64, 81-85` — v1.6 trajectory, reviewer tech-debt item, and schema-lock/audit-as-decisive-source decisions.
- `.planning/adrs/0001-ritual-schema-v1.md:26-52, 104-109, 147-162` — locked field set, optional fields, serde discipline, and schema consequences.
- `crates/amlich-core/src/rituals/schema.rs:127-150` — actual `RitualEntry` definition, `deny_unknown_fields`, and `invocation_text_vi` location.
- `crates/amlich-core/src/rituals/corpus.rs:85-169, 203-224` — `OnceLock` loader, source validation, NFC normalization, and text-field guard.
- `crates/amlich-core/data/rituals/provenance_audit.md:1-9, 13-164` — current 60-row ledger and all-`pending` reviewer state.
- `crates/amlich-core/tests/rituals_integration.rs:155-169` — existing Test 5 schema/NFC round-trip pattern.
- `.planning/phases/12-van-khan-corpus-authoring/12-VERIFICATION.md:20-26, 47-50, 80-95` — verified corpus/ledger one-to-one coverage and explicitly deferred reviewer state.
- `.planning/phases/16-foundation-adr-0003-confidence-closure/16-02-SUMMARY.md:13-17, 42-52, 73-103` — `DeferralMarker` shape, explicit marker naming, and no-silent-correction pattern.
- `.planning/adrs/0003a-nien-tu-bach-polarity-confidence-closure.md:42-51, 70-78` — accepted `PendingExternalReview` narrative and reason/date/assignee expectations.

### Secondary (MEDIUM confidence — project precedent and design inference)

- `.planning/phases/12-van-khan-corpus-authoring/12-RESEARCH.md:266-295, 324-332, 655-661` — original audit-ledger format, no-new-schema guidance, and why `pending` was accepted only for v1.5.
- `.planning/RETROSPECTIVE.md:88-115, 145-150` — audit-as-decisive-source, external-crate black-box tests, schema-lock-before-corpus, and verification synchronization lessons.
- `.planning/phases/16-foundation-adr-0003-confidence-closure/16-VERIFICATION.md:17-28, 76-86` — typed deferral verification and boundary between code-verifiable state and human review claims.
- `crates/amlich-core/src/rituals/mod.rs:22-33` — public re-export surface used by external integration tests.

### Tertiary (LOW confidence — requires user validation)

- Actual reviewer names, review dates, source-access availability, and the expected review date/assignee for ritual deferrals. These are editorial facts not present in the repository and must not be inferred.
- Whether the project considers a particular author desk-check sufficient for the intended closure; this is a policy decision, not a codebase fact.

## Metadata

**Confidence breakdown:**

- Repository/schema/test mechanics: HIGH — directly confirmed in the locked schema, loader, ledger, integration test, ADR, and prior verification artifacts.
- Ledger format and plan decomposition: HIGH — directly prescribed by the roadmap and established Phase 12/16 patterns.
- Reviewer policy and classical content correctness: LOW until the user supplies identities, access, and disposition policy.
- Corrected-entry test source-of-truth design: HIGH — follows the roadmap's ledger-as-canonical constraint and existing Test 5 surface; the small Markdown parser is a recommended test-only implementation detail.

**Research date:** 2026-07-15
**Valid until:** 2026-08-15 (repository structure is stable; reviewer availability and editorial decisions may change sooner)
