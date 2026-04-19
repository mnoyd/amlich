# Contract Evolution Guidelines Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Create a full policy document for evolving canonical reasoning and personalization contracts without fragmenting shapes, semantics, or migration expectations across surfaces.

**Architecture:** Add one policy-level document under `docs/almanac` that sits above the schema and usage-example docs. It should define evolution rules, breaking/non-breaking classifications, required tests and docs, migration expectations, and worked examples for reasoning nodes, headline semantics, and matrix-section growth, while referencing existing schema/versioning policy where relevant instead of duplicating it blindly.

**Tech Stack:** Markdown docs, existing almanac schema/contract docs, existing ruleset/versioning policy.

---

### Task 1: Lock the policy shape with a failing doc skeleton

**Files:**
- Create: `docs/almanac/contract-evolution-guidelines.md`
- Read for context: `docs/almanac/reasoning-graph-schema.md`
- Read for context: `docs/almanac/contract-usage-examples.md`
- Read for context: `docs/almanac/known-differences.md`
- Read for context: `docs/almanac/ruleset-loader.md`

**Step 1: Write the failing test**

Create `docs/almanac/contract-evolution-guidelines.md` with the final top-level section structure only, but leave the substantive rules absent. Include headings for:
- goals / scope
- invariants
- allowed changes
- breaking changes
- migration rules
- required tests and docs
- worked examples

This is the failing artifact because the guidance exists structurally but does not yet satisfy `amlich-606`.

**Step 2: Run test to verify it fails**

Run: `Read docs/almanac/contract-evolution-guidelines.md`
Expected: The file exists but lacks the concrete policy content required by the task.

**Step 3: Write minimal implementation**

Fill in the smallest complete policy that makes the document actually useful for future work.

**Step 4: Run test to verify it passes**

Run: `Read docs/almanac/contract-evolution-guidelines.md`
Expected: The document now contains actionable policy, not just headings.

**Step 5: Commit**

```bash
git add docs/almanac/contract-evolution-guidelines.md
git commit -m "docs: add contract evolution guidelines"
```

---

### Task 2: Integrate the policy with existing contract docs

**Files:**
- Modify: `docs/almanac/reasoning-graph-schema.md`
- Modify: `docs/almanac/contract-usage-examples.md`
- Read for context: `docs/almanac/contract-evolution-guidelines.md`

**Step 1: Write the failing test**

Add a placeholder related-doc reference in the existing docs pointing to the new policy file, but do not yet align the wording around evolution/migration requirements.

**Step 2: Run test to verify it fails**

Run: `Read docs/almanac/reasoning-graph-schema.md` and `Read docs/almanac/contract-usage-examples.md`
Expected: References exist, but the guidance is not yet clearly integrated into the surrounding contract story.

**Step 3: Write minimal implementation**

Update existing docs so they point readers to the policy for future changes, versioning, and migration expectations without duplicating the full rules.

**Step 4: Run test to verify it passes**

Run: `Read docs/almanac/reasoning-graph-schema.md` and `Read docs/almanac/contract-usage-examples.md`
Expected: The new policy is clearly discoverable from the schema and usage docs.

**Step 5: Commit**

```bash
git add docs/almanac/reasoning-graph-schema.md docs/almanac/contract-usage-examples.md docs/almanac/contract-evolution-guidelines.md
git commit -m "docs: link contract evolution policy into reasoning docs"
```

---

### Task 3: Verify the guidance is complete enough to gate future changes

**Files:**
- Modify only if needed: `docs/almanac/contract-evolution-guidelines.md`
- Modify only if needed: `docs/almanac/reasoning-graph-schema.md`
- Modify only if needed: `docs/almanac/contract-usage-examples.md`

**Step 1: Write the failing test**

If review reveals a missing policy area (for example semantic changes, optional matrix sections, or migration procedure), add one narrow missing subsection first so the gap is explicit.

**Step 2: Run test to verify it fails**

Run: `Read docs/almanac/contract-evolution-guidelines.md`
Expected: The gap is visible and unresolved.

**Step 3: Write minimal implementation**

Fill only that missing gap.

**Step 4: Run test to verify it passes**

Run: `Read docs/almanac/contract-evolution-guidelines.md`
Expected: The policy now covers contract invariants, migration rules, test/doc requirements, and worked examples well enough to guide future contributors.

**Step 5: Commit**

```bash
git add docs/almanac/contract-evolution-guidelines.md docs/almanac/reasoning-graph-schema.md docs/almanac/contract-usage-examples.md
git commit -m "docs: finalize reasoning contract evolution policy"
```
