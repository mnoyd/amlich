# Explanation UX Validation Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add representative, parity-backed explanation UX validation for favorable, cautious, avoid, personal-profile, and boundary cases, with both executable coverage and a developer-facing acceptance document.

**Architecture:** Reuse the existing parity corpus in `crates/amlich-core/tests/reasoning_graph_parity.rs` as the single source of representative scenarios. Add only narrow assertions that the corpus exposes the explanation traits the new UI hierarchy depends on, then document a small curated subset in `docs/almanac` as concrete acceptance examples for consumers and future UI work.

**Tech Stack:** Rust workspace tests, `amlich-core`, existing reasoning parity corpus, Markdown docs, `cargo test`.

---

### Task 1: Lock representative UX cases in executable coverage

**Files:**
- Modify: `crates/amlich-core/tests/reasoning_graph_parity.rs`
- Read for context: `crates/amlich-core/tests/reasoning_graph_canonical.rs`
- Read for context: `docs/almanac/contract-usage-examples.md`

**Step 1: Write the failing test**

Add a focused test in `crates/amlich-core/tests/reasoning_graph_parity.rs` that selects one case from each required track:
- favorable baseline
- cautious/conflicted baseline
- avoid baseline
- personal-profile case
- boundary/timezone case

For each selected case, assert the explanation traits required by the new hierarchy:
- headline bucket matches expectation
- at least one rationale item exists
- caution visibility is present when expected
- personal case changes profile-dependent output
- boundary case preserves declared visibility traits

**Step 2: Run test to verify it fails**

Run: `cargo test -p amlich-core reasoning_graph_parity`
Expected: FAIL because the new representative UX validation test does not exist yet.

**Step 3: Write minimal implementation**

Implement only the smallest additions needed:
- add a helper if necessary to retrieve a case by id without duplicating the corpus
- add the new representative UX validation test
- keep the corpus itself stable unless a missing scenario must be added

**Step 4: Run test to verify it passes**

Run: `cargo test -p amlich-core reasoning_graph_parity`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/amlich-core/tests/reasoning_graph_parity.rs
git commit -m "test(core): lock representative explanation ux cases"
```

---

### Task 2: Document the curated acceptance examples

**Files:**
- Modify: `docs/almanac/contract-usage-examples.md`
- Create only if truly needed: `docs/almanac/explanation-ux-validation.md`
- Read for context: `crates/amlich-core/tests/reasoning_graph_parity.rs`
- Read for context: `docs/almanac/reasoning-graph-schema.md`

**Step 1: Write the failing test**

Use the doc itself as the failing artifact: add a new section outline or TODO headings describing the five representative case classes, but leave the concrete mappings absent so the acceptance content is obviously incomplete.

**Step 2: Run test to verify it fails**

Run: `Read docs/almanac/contract-usage-examples.md`
Expected: The new section exists but lacks the concrete examples/acceptance mappings needed for `amlich-q5r`.

**Step 3: Write minimal implementation**

Add a concise validation section that maps one concrete parity case from each class to:
- why it belongs to that class
- expected headline tone
- expected support/watch/proceed/details treatment
- any special personal or boundary caveat

Prefer editing `docs/almanac/contract-usage-examples.md` unless a separate document is clearly cleaner and necessary.

**Step 4: Run test to verify it passes**

Run: `Read docs/almanac/contract-usage-examples.md`
Expected: The document now contains concrete, curated acceptance examples for all five case classes.

**Step 5: Commit**

```bash
git add docs/almanac/contract-usage-examples.md
git commit -m "docs: add explanation ux validation examples"
```

---

### Task 3: Verify the combined validation path

**Files:**
- Modify only if needed: `crates/amlich-core/tests/reasoning_graph_parity.rs`
- Modify only if needed: `docs/almanac/contract-usage-examples.md`

**Step 1: Write the failing test**

If final verification reveals a missing assumption in the representative cases or docs, add one narrow regression assertion for that exact gap before changing behavior.

**Step 2: Run test to verify it fails**

Run the smallest relevant command for the newly added assertion.
Expected: FAIL for the missing assumption.

**Step 3: Write minimal implementation**

Fix only that specific validation/documentation gap.

**Step 4: Run test to verify it passes**

Run:
- `cargo test -p amlich-core reasoning_graph_parity`
- `cargo test -p am-lich`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/amlich-core/tests/reasoning_graph_parity.rs docs/almanac/contract-usage-examples.md
git commit -m "test: verify explanation ux validation corpus"
```
