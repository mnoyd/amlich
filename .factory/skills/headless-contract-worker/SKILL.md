---
name: headless-contract-worker
description: Redesign machine-readable amlich outputs and enforce selector/headless parity with focused CLI contract tests and explicit stdout/stderr verification.
---

# Headless Contract Worker

NOTE: Startup and cleanup are handled by `worker-base`. This skill defines the WORK PROCEDURE.

## When to Use This Skill

Use this skill for features centered on:
- non-TTY `amlich` behavior
- machine-readable `day`, `range`, and NDJSON output contracts in `crates/amlich`
- selector propagation, alias normalization, invalid-selector failures, and stdout/stderr hygiene
- cross-surface parity features where the main success criterion is coherence between interactive selection state and machine-readable outputs

Do not use this skill for predominantly interactive widget/layout work when the machine-output contract is not the main feature boundary.

## Work Procedure

1. Read `mission.md`, `validation-contract.md`, mission `AGENTS.md`, `.factory/library/api-contract.md`, and `.factory/library/user-testing.md` before editing.
2. Trace the exact output path the feature touches: default non-TTY auto-mode, explicit `day`/`range` format branches, CLI selector parsing, and any `amlich-api` calls that provide canonical metadata.
3. Add failing tests first. Prefer `crates/amlich/tests/cli_contract.rs` for CLI behavior and selector/output assertions; add or extend `amlich-api` tests only when the contract seam itself needs coverage.
4. Implement the smallest coherent contract change. Keep machine output parseable, preserve stderr/stdout separation, and do not silently coerce invalid selectors to defaults.
5. Run focused verification first:
   - `cargo check --package amlich-api --package amlich-cli`
   - `cargo test --package amlich-cli --test cli_contract -- --nocapture`
   - targeted `amlich-api` contract tests when selector normalization or pack metadata behavior changes
6. Run manual machine-output checks for every affected format. Parse and compare canonical fields explicitly: `schema_version`, `ruleset_id`, `ruleset_version`, `profile`, contextual activation, and active packs.
7. If the feature spans parity with the interactive surface, run one bounded PTY TUI scenario for the same query and record the exact fields that matched.
8. Update `.factory/library/` only when you discover durable contract or validation knowledge that future workers need.
9. In the handoff, record exact commands, exact parse observations, and whether stdout remained clean on both success and failure cases.

## Example Handoff

```json
{
  "salientSummary": "Redesigned default non-TTY output to expose canonical engine identity, tightened selector propagation across day/range/NDJSON outputs, and added explicit CLI failures for duplicate and empty pack ids without contaminating stdout. The final parity check also confirmed interactive and machine-readable surfaces agree on the same canonical metadata for a selector-enabled scenario.",
  "whatWasImplemented": "Updated `crates/amlich` headless branches so default non-TTY output emits a single parseable payload with `ruleset_id`, `ruleset_version`, and `profile`, extended selector-aware metadata propagation through day/range/NDJSON outputs, and hardened stdout/stderr hygiene for invalid selectors and deprecated-command warnings. Also expanded `crates/amlich/tests/cli_contract.rs` to cover alias normalization, NDJSON row identity, and no-stdout-on-error expectations.",
  "whatWasLeftUndone": "Interactive widget-level provenance presentation remains outside this feature; it only validated parity against already-implemented TUI metadata surfaces.",
  "verification": {
    "commandsRun": [
      {
        "command": "cargo check --package amlich-api --package amlich-cli",
        "exitCode": 0,
        "observation": "Headless CLI and API contract changes compile cleanly."
      },
      {
        "command": "cargo test --package amlich-cli --test cli_contract -- --nocapture",
        "exitCode": 0,
        "observation": "CLI contract tests passed, including selector propagation, NDJSON row identity, and no-stdout-on-error cases."
      },
      {
        "command": "cargo test --package amlich-api --test catalog_contract --test recommendation_contract -- --nocapture",
        "exitCode": 0,
        "observation": "API selector normalization and recommendation-pack metadata tests still pass."
      },
      {
        "command": "cargo run --package amlich-cli -- day 2026-03-13 --format json --pretty --ruleset baseline --event-kind travel --recommendation-packs pack.xuat_hanh.v1",
        "exitCode": 0,
        "observation": "Output normalized `baseline` to the canonical ruleset id and surfaced contextual activation plus active-pack metadata."
      },
      {
        "command": "cargo run --package amlich-cli -- range --start 2026-03-13 --end 2026-03-15 --format ndjson --ruleset baseline --event-kind travel --recommendation-packs pack.xuat_hanh.v1",
        "exitCode": 0,
        "observation": "Each NDJSON row parsed independently and retained canonical engine identity fields."
      }
    ],
    "interactiveChecks": [
      {
        "action": "Ran one selector-enabled explorer/inspector PTY flow and compared the resulting metadata to matching day JSON output.",
        "observed": "The TUI and machine-readable output agreed on canonical `ruleset_id`, `ruleset_version`, `profile`, and active-pack context for the same date."
      }
    ]
  },
  "tests": {
    "added": [
      {
        "file": "crates/amlich/tests/cli_contract.rs",
        "cases": [
          {
            "name": "selector_bearing_ndjson_rows_remain_self_describing",
            "verifies": "Each NDJSON row carries canonical engine identity for selector-enabled ranges."
          },
          {
            "name": "invalid_pack_inputs_write_no_stdout_bytes",
            "verifies": "Duplicate or empty pack ids fail with explicit stderr and pristine stdout."
          }
        ]
      }
    ]
  },
  "discoveredIssues": [
    {
      "severity": "low",
      "description": "Deprecated `query` behavior remains a separate warning path that could diverge again if not covered by future contract tests.",
      "suggestedFix": "Keep at least one stdout/stderr hygiene assertion for deprecated-command flows in `cli_contract` coverage."
    }
  ]
}
```

## When to Return to Orchestrator

- The feature requires a product decision about which headless formats remain supported or how much metadata to expose for machine consumers.
- The necessary interactive parity evidence cannot be gathered with the agreed PTY/script approach.
- A requested change would force broad API redesign beyond the accepted mission scope.
- Existing dirty-worktree changes in `crates/amlich` make it unsafe to isolate machine-output contract edits.
