---
name: tui-experience-worker
description: Build the explorer-first interactive amlich TUI and verify launch, selector, and inspector flows with test-first Rust changes plus bounded PTY smoke checks.
---

# TUI Experience Worker

NOTE: Startup and cleanup are handled by `worker-base`. This skill defines the WORK PROCEDURE.

## When to Use This Skill

Use this skill for features centered on:
- `crates/amlich-tui`
- interactive launch wiring in `crates/amlich`
- explorer dashboard state, selector interactions, loading/error states, and inspector panels
- TUI flows that must be verified through bounded PTY/script smoke checks

Do not use this skill for primarily machine-readable CLI/headless contract work unless the feature explicitly says the interactive surface is the main success criterion.

## Work Procedure

1. Read `mission.md`, `validation-contract.md`, mission `AGENTS.md`, and `.factory/library/user-testing.md` before touching code.
2. Trace the exact interactive path the feature touches: launcher entrypoint, TUI state/event flow, widget/layout rendering, and any `amlich-api` calls the TUI consumes.
3. Add failing tests first. Prefer focused tests in `crates/amlich-tui` for state, events, layout, or widgets. If the feature changes launch wiring, extend `crates/amlich/tests/cli_contract.rs` or nearby focused coverage before implementation.
4. Implement the smallest coherent interactive slice. Keep selector normalization and recommendation policy in `amlich-api` / existing contract code rather than inventing new local policy in widgets.
5. Run focused verification first using `.factory/services.yaml` commands when possible:
   - `cargo check --package amlich-tui --package amlich-cli`
   - `cargo test --package amlich-tui`
   - `cargo test --package amlich-cli --test cli_contract -- --nocapture` when launch wiring or CLI-facing interactive entrypoints change
6. Run bounded PTY smoke checks for every changed user-facing interactive flow:
   - `printf 'q' | timeout 15 script -qec "cargo run --package amlich-cli -- tui --date 2026-03-13" /dev/null`
   - also run `printf 'q' | timeout 15 script -qec "cargo run --package amlich-cli --" /dev/null` if the feature touches launch parity
7. If the feature changes selector or inspector semantics, compare at least one TUI scenario against a matching machine-readable `day` JSON command and record the specific matching fields.
8. Update `.factory/library/` only if you discover durable interactive-surface knowledge future workers or validators will need.
9. In the handoff, be explicit about which interactive flows were tested, which keys/actions were exercised, and which metadata fields were verified across surfaces.

## Example Handoff

```json
{
  "salientSummary": "Unified `amlich` TTY auto-mode with `amlich tui`, replaced the calendar-first landing with the new explorer shell, and added staged selector controls for ruleset, event kind, packs, and date. Focused TUI tests plus bounded PTY smoke runs now prove both launch paths reach the same explorer-first experience.",
  "whatWasImplemented": "Refactored `crates/amlich/src/main.rs` so both interactive entrypoints route through the same explorer-shell runtime in `crates/amlich-tui`, added selector-focused state and event handling for the landing dashboard, and updated widgets/layout so the shell shows catalog-backed configuration controls before deep inspection. Also added loading-state and preserved-selection coverage in `crates/amlich-tui` tests.",
  "whatWasLeftUndone": "Deep provenance panels and headless parity checks remain for later features; this slice only established the explorer shell and launch coherence.",
  "verification": {
    "commandsRun": [
      {
        "command": "cargo check --package amlich-tui --package amlich-cli",
        "exitCode": 0,
        "observation": "Interactive launcher and TUI code compile cleanly together."
      },
      {
        "command": "cargo test --package amlich-tui",
        "exitCode": 0,
        "observation": "State, event, and widget tests passed for the new explorer shell flow."
      },
      {
        "command": "cargo test --package amlich-cli --test cli_contract -- --nocapture",
        "exitCode": 0,
        "observation": "CLI contract coverage still passes after changing interactive launch routing."
      },
      {
        "command": "printf 'q' | timeout 15 script -qec \"cargo run --package amlich-cli -- tui --date 2026-03-13\" /dev/null",
        "exitCode": 0,
        "observation": "Explicit TUI launch reached the explorer shell and exited cleanly."
      },
      {
        "command": "printf 'q' | timeout 15 script -qec \"cargo run --package amlich-cli --\" /dev/null",
        "exitCode": 0,
        "observation": "TTY auto-mode reached the same explorer shell and exited cleanly."
      }
    ],
    "interactiveChecks": [
      {
        "action": "Opened the explorer shell, moved focus across ruleset, pack, event-kind, and date controls, then quit.",
        "observed": "All four controls were reachable from the landing shell without falling into the old calendar-first flow."
      },
      {
        "action": "Applied a selector change and compared the resulting inspector metadata to a matching `amlich day --format json --pretty` command.",
        "observed": "The TUI and JSON output agreed on canonical `ruleset_id`, `ruleset_version`, and `profile` for the selected date."
      }
    ]
  },
  "tests": {
    "added": [
      {
        "file": "crates/amlich-tui/src/events.rs",
        "cases": [
          {
            "name": "selector_edits_stage_until_apply",
            "verifies": "Editing explorer controls does not silently mutate the inspected result before the apply action."
          },
          {
            "name": "launch_parity_routes_auto_mode_into_explorer_shell",
            "verifies": "Bare `amlich` and `amlich tui` enter the same explorer-first runtime."
          }
        ]
      }
    ]
  },
  "discoveredIssues": [
    {
      "severity": "medium",
      "description": "The current PTY smoke path can prove launch and key flow, but it cannot capture full semantic terminal snapshots without additional tooling.",
      "suggestedFix": "Rely on focused widget/state tests plus PTY smoke for this mission, and only expand automation if the user later installs tuistory."
    }
  ]
}
```

## When to Return to Orchestrator

- The feature requires a new API/catalog field or selector validation seam that is not already available through `amlich-api`.
- Interactive acceptance depends on richer terminal automation than the agreed PTY/script approach can provide.
- The working tree contains unrelated interactive-launch edits that make it unsafe to isolate the feature.
- A requested behavior would force the worker to redesign headless machine-output contracts beyond the feature description.
