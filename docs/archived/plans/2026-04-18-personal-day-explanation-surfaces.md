# Personal Day Explanation Surfaces Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Rework the personal-day explanation surfaces so desktop and TUI present canonical reasoning as layered end-user guidance instead of raw contract/debug output.

**Architecture:** Keep the existing contract boundary intact and only change presentation. First lock the desired desktop explanation structure with a failing UI/type check, then implement the layered desktop rendering helpers, and finally align the legacy TUI overlay wording and emphasis to the same hierarchy without exposing raw graph internals as the primary content.

**Tech Stack:** Svelte 5, TypeScript, Tauri desktop app, Rust workspace, legacy `crates/amlich` TUI, `svelte-check`, `cargo test`.

---

### Task 1: Lock the desktop explanation hierarchy

**Files:**
- Modify: `apps/desktop/src/lib/components/PersonalDayPanel.svelte`
- Read for context: `apps/desktop/src/lib/insights/types/personal-day-dto.ts`
- Read for context: `docs/almanac/contract-usage-examples.md`

**Step 1: Write the failing test**

Because this app currently has no component test harness, add a deliberately type-checked render structure in `PersonalDayPanel.svelte` that references new helper functions / sections that do not exist yet:
- verdict summary block
- what helps list
- what to watch list
- next-step guidance list
- compact evidence/details block

**Step 2: Run test to verify it fails**

Run: `pnpm --dir apps/desktop check`
Expected: FAIL because the new helpers / template references are not implemented.

**Step 3: Write minimal implementation**

Add the smallest helper functions and template updates needed so the panel:
- promotes `decision_export.primary_conclusion`
- maps supports/resistances/overrides/conflicts into human phrasing
- keeps semantic/confidence/details in a lower-prominence block
- demotes raw graph node counts into a small evidence summary

**Step 4: Run test to verify it passes**

Run: `pnpm --dir apps/desktop check`
Expected: PASS

**Step 5: Commit**

```bash
git add apps/desktop/src/lib/components/PersonalDayPanel.svelte
git commit -m "feat(desktop): layer personal-day explanations"
```

---

### Task 2: Align the legacy TUI explanation hierarchy

**Files:**
- Modify: `crates/amlich/src/widgets/insight_overlay.rs`
- Test: `crates/amlich/src/widgets/insight_overlay.rs` (existing or new focused tests if needed)

**Step 1: Write the failing test**

Add or update a focused widget test that expects the rendered personal explanation copy to include:
- explicit verdict label
- support section label
- caution/watch section label
- proceed/next-step section label when suggestions exist

**Step 2: Run test to verify it fails**

Run: `cargo test -p amlich-cli insight_overlay`
Expected: FAIL because the old wording/ordering does not match the new hierarchy.

**Step 3: Write minimal implementation**

Adjust the overlay text and ordering only as needed to match the approved layered explanation pattern. Keep canonical fields, but avoid expanding scope into unrelated overlay redesign work.

**Step 4: Run test to verify it passes**

Run: `cargo test -p amlich-cli insight_overlay`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/amlich/src/widgets/insight_overlay.rs
git commit -m "feat(tui): clarify personal-day explanation hierarchy"
```

---

### Task 3: Verify the combined surface behavior

**Files:**
- Modify if needed: `apps/desktop/src-tauri/src/lib.rs`
- Read for context: `docs/almanac/contract-usage-examples.md`

**Step 1: Write the failing test**

If verification reveals a missing contract assumption, add a targeted regression test in the narrowest existing test file covering the missing assumption.

**Step 2: Run test to verify it fails**

Run only the narrow command for the added regression.
Expected: FAIL for the missing assumption.

**Step 3: Write minimal implementation**

Fix only the contract/presentation boundary needed for the regression.

**Step 4: Run test to verify it passes**

Run:
- `pnpm --dir apps/desktop check`
- `cargo test -p amlich-cli insight_overlay`
- `cargo test -p am-lich`
Expected: PASS

**Step 5: Commit**

```bash
git add apps/desktop/src-tauri/src/lib.rs apps/desktop/src/lib/components/PersonalDayPanel.svelte crates/amlich/src/widgets/insight_overlay.rs
git commit -m "test: verify personal-day explanation surfaces"
```
