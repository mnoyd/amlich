---
name: consumer-integration-worker
description: Migrate CLI/TUI consumers onto the redesigned engine contract and verify user-facing CLI behavior stays coherent.
---

# Consumer Integration Worker

NOTE: Startup and cleanup are handled by `worker-base`. This skill defines the WORK PROCEDURE.

## When to Use This Skill

Use this skill for features centered on:
- `crates/amlich`
- `crates/amlich-tui`
- CLI discovery commands, range/text/waybar behavior, and legacy entrypoint boundaries
- RFC or migration guidance only when it directly depends on consumer behavior or CLI boundaries

Do not use this skill for deep `amlich-core` or `amlich-api` contract design unless the feature explicitly includes consumer wiring.

## Work Procedure

1. Read `mission.md`, `validation-contract.md`, mission `AGENTS.md`, and `.factory/library/user-testing.md` before editing.
2. Inspect the current CLI/TUI consumer path that the feature touches. Trace which API call powers the surface today.
3. Add or update failing tests first. Prefer `crates/amlich/tests/cli_contract.rs` for CLI behavior and focused compile/test coverage for TUI.
4. Implement the migration with the smallest coherent surface area. Keep CLI JSON as the source of truth for overlapping facts, and avoid inventing consumer-only policy when the API can provide it.
5. Run focused Rust checks:
   - `cargo check --package amlich-cli --package amlich-tui`
   - `cargo test --package amlich-cli --test cli_contract -- --nocapture`
   - any extra targeted test scope for touched crates
6. Run manual CLI checks for every changed user-facing flow and record the exact commands and observations.
7. If TUI wiring changed, run only the agreed startup smoke test; do not expand validation scope beyond the mission guidance.
8. Update `.factory/library/` only when you discover durable CLI/testing knowledge future workers will need.
9. In the handoff, be explicit about what still uses legacy pathways, what was migrated, and any remaining migration boundary.

## Example Handoff

```json
{
  "salientSummary": "Migrated `amlich day` and `amlich range` onto the new engine contract, added CLI discovery commands for rulesets and recommendation packs, and kept text/Waybar outputs aligned on overlapping facts. CLI contract tests now cover the new selector flags and invalid range/field failures.",
  "whatWasImplemented": "Updated `crates/amlich` to build engine requests directly from CLI flags, wired lookup commands for rulesets and recommendation packs, tightened range envelope handling, and aligned legacy query/headless messaging with the documented migration boundary. Also updated TUI loading paths where the new contract changed required fields.",
  "whatWasLeftUndone": "Desktop and WASM consumers still target legacy DTOs and were intentionally left out of scope for this mission.",
  "verification": {
    "commandsRun": [
      {
        "command": "cargo check --package amlich-cli --package amlich-tui",
        "exitCode": 0,
        "observation": "CLI and TUI compile against the new contract."
      },
      {
        "command": "cargo test --package amlich-cli --test cli_contract -- --nocapture",
        "exitCode": 0,
        "observation": "CLI contract coverage passed, including the new engine-flag and discovery-command cases."
      },
      {
        "command": "cargo run --package amlich-cli -- lookup rulesets --format json",
        "exitCode": 0,
        "observation": "Ruleset discovery returns canonical ids and default-selection metadata."
      },
      {
        "command": "cargo run --package amlich-cli -- day 2026-02-20 --format json --pretty --ruleset vn_baseline_v1 --recommendation-packs pack.nhi_thap_bat_tu.v1",
        "exitCode": 0,
        "observation": "Day JSON shows canonical ruleset metadata and contextual recommendation activation matching the discovered pack."
      }
    ],
    "interactiveChecks": [
      {
        "action": "Compared `amlich day` JSON, text, and waybar output for the same date after migration.",
        "observed": "Overlapping facts stayed aligned and Waybar remained parseable with `text`, `tooltip`, and `class` fields."
      },
      {
        "action": "Ran the agreed TUI startup smoke test after updating TUI wiring.",
        "observed": "The TUI started successfully for the target date without expanding acceptance beyond CLI surfaces."
      }
    ]
  },
  "tests": {
    "added": [
      {
        "file": "crates/amlich/tests/cli_contract.rs",
        "cases": [
          {
            "name": "day_json_exposes_engine_metadata_and_selector_effects",
            "verifies": "CLI JSON matches the canonical engine contract and selector-enabled output changes predictably."
          },
          {
            "name": "range_rows_match_day_results_for_shared_fields",
            "verifies": "Range output stays aligned with per-day output on overlapping metadata and sections."
          }
        ]
      }
    ]
  },
  "discoveredIssues": [
    {
      "severity": "low",
      "description": "Desktop and WASM consumers still use legacy DTO entrypoints and will need a follow-up migration if they must adopt engine selectors.",
      "suggestedFix": "Track a later consumer migration mission after CLI/TUI contract stabilization."
    }
  ]
}
```

## When to Return to Orchestrator

- The feature requires changing desktop, WASM, or JS consumers that are currently out of mission scope.
- A CLI migration decision would materially change the accepted user-facing behavior and the feature description does not resolve it.
- TUI migration exposes a missing API contract seam that must be solved in `amlich-api` first.
- Existing dirty-worktree changes make it unsafe to distinguish consumer edits from unrelated local work.
