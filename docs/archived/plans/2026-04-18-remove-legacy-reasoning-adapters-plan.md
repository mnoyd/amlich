# Remove Legacy Reasoning Adapters Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Remove remaining legacy personal-day reasoning fields and fallbacks so current app surfaces depend only on canonical reasoning contracts.

**Architecture:** Tighten the desktop contract at the DTO/type boundary and remove presentation fallbacks that still tolerate legacy summary/advisory shapes. Keep the API/Tauri boundary aligned with the canonical contract already used by the renderer, then verify that desktop type checks and Rust tests pass without compatibility aliases.

**Tech Stack:** TypeScript, Svelte 5, Tauri desktop app, Rust workspace, `amlich-api`, `svelte-check`, `cargo test`.

---

### Task 1: Lock the desktop type surface to canonical personal-day fields

**Files:**
- Modify: `apps/desktop/src/lib/insights/types/personal-day-dto.ts`
- Modify: `apps/desktop/src/lib/insights/types/index.ts`
- Read for context: `crates/amlich-api/src/dto.rs`
- Read for context: `apps/desktop/src/lib/components/PersonalDayPanel.svelte`

**Step 1: Write the failing test**

Remove the legacy compatibility fields from the TypeScript DTO definitions first:
- `PersonalDayAdvisoryDto.highlights`
- `PersonalDayAdvisoryDto.cautions`
- `PersonalDayAdvisoryDto.reasoning_bucket`
- `PersonalDayAdvisoryDto.reasoning_confidence`
- any top-level legacy report fields that are no longer used by current surfaces and duplicate canonical meaning

Leave consumer code untouched for the moment so type-checking fails anywhere those fields are still assumed.

**Step 2: Run test to verify it fails**

Run: `pnpm --dir apps/desktop check`
Expected: FAIL if any desktop code still references removed legacy fields.

**Step 3: Write minimal implementation**

Update the type exports and any remaining desktop references so the app compiles against canonical fields only.

**Step 4: Run test to verify it passes**

Run: `pnpm --dir apps/desktop check`
Expected: PASS (existing unrelated warnings are acceptable if unchanged).

**Step 5: Commit**

```bash
git add apps/desktop/src/lib/insights/types/personal-day-dto.ts apps/desktop/src/lib/insights/types/index.ts apps/desktop/src/lib/components/PersonalDayPanel.svelte
git commit -m "refactor(desktop): remove legacy personal-day dto fields"
```

---

### Task 2: Remove legacy presentation fallbacks from the personal-day surface

**Files:**
- Modify: `apps/desktop/src/lib/components/PersonalDayPanel.svelte`
- Modify if needed: `apps/desktop/src-tauri/src/lib.rs`
- Read for context: `docs/almanac/contract-usage-examples.md`

**Step 1: Write the failing test**

In `PersonalDayPanel.svelte`, remove fallback presentation that prefers legacy fields when canonical fields are absent, such as:
- `report.decision_export?.primary_conclusion ?? report.summary`
- any remaining dependence on compatibility advisory summary/highlight/caution behavior

Keep the rest untouched so type or template checks fail if canonical assumptions are incomplete.

**Step 2: Run test to verify it fails**

Run: `pnpm --dir apps/desktop check`
Expected: FAIL if the component or command surface still depends on removed fallback behavior.

**Step 3: Write minimal implementation**

Adjust the panel and, only if necessary, the Tauri command/test surface so the personal-day UI renders exclusively from canonical fields plus matrix sections.

**Step 4: Run test to verify it passes**

Run:
- `pnpm --dir apps/desktop check`
- `cargo test -p am-lich`
Expected: PASS

**Step 5: Commit**

```bash
git add apps/desktop/src/lib/components/PersonalDayPanel.svelte apps/desktop/src-tauri/src/lib.rs
git commit -m "refactor(desktop): drop personal-day legacy fallbacks"
```

---

### Task 3: Verify no legacy adapter assumptions remain in current surfaces

**Files:**
- Modify only if needed: `apps/desktop/src/lib/insights/types/personal-day-dto.ts`
- Modify only if needed: `apps/desktop/src/lib/components/PersonalDayPanel.svelte`
- Modify only if needed: `docs/almanac/contract-usage-examples.md`

**Step 1: Write the failing test**

Add one narrow regression only if final verification finds a remaining compatibility assumption that is not already covered.

**Step 2: Run test to verify it fails**

Run the smallest relevant command for that regression.
Expected: FAIL for the uncovered assumption.

**Step 3: Write minimal implementation**

Fix only the remaining adapter/fallback assumption.

**Step 4: Run test to verify it passes**

Run:
- `pnpm --dir apps/desktop check`
- `cargo test -p am-lich`
- `cargo test -p amlich-cli insight_overlay`
Expected: PASS

**Step 5: Commit**

```bash
git add apps/desktop/src/lib/insights/types/personal-day-dto.ts apps/desktop/src/lib/components/PersonalDayPanel.svelte apps/desktop/src-tauri/src/lib.rs docs/almanac/contract-usage-examples.md
git commit -m "test: verify canonical-only personal-day surfaces"
```
