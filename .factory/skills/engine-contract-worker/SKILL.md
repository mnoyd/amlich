---
name: engine-contract-worker
description: Implement canonical engine contract, selector validation, discovery metadata, and contract-facing architecture artifacts in amlich-core/amlich-api.
---

# Engine Contract Worker

NOTE: Startup and cleanup are handled by `worker-base`. This skill defines the WORK PROCEDURE.

## When to Use This Skill

Use this skill for features centered on:
- `crates/amlich-core`
- `crates/amlich-api`
- catalog/discovery surfaces for rulesets and recommendation packs
- contract-level RFC or architecture artifacts that explain the redesigned engine API

Do not use this skill for primarily CLI/TUI integration work inside `crates/amlich` or `crates/amlich-tui` unless the feature description explicitly says the contract worker owns that boundary.

## Work Procedure

1. Read `mission.md`, `validation-contract.md`, mission `AGENTS.md`, and relevant files in `.factory/library/` before touching code.
2. Inspect the current contract and implementation seams in the touched crates. Identify the smallest test surface that proves the feature.
3. Write or update failing tests first. Prefer focused contract tests in `crates/amlich-api/tests/` or targeted unit tests in `amlich-core`.
4. Implement the minimum coherent contract change across `amlich-core` and `amlich-api`. Keep domain logic in core and transport/validation behavior in API.
5. If the feature introduces discovery metadata, make sure runtime activation can be compared back to catalog output.
6. Run focused verification first:
   - `cargo check --package amlich-core --package amlich-api`
   - the targeted `cargo test` scope for touched tests
7. If the feature changes behavior visible through the CLI JSON surface, run one representative CLI smoke command and record the output observation.
8. Update `.factory/library/` only when you learn durable information that future workers need.
9. Leave the handoff with exact commands, exact failing/passing observations, and any remaining ambiguity. If selector semantics or discovery behavior are still unclear, return to the orchestrator instead of guessing.

## Example Handoff

```json
{
  "salientSummary": "Introduced the new engine request envelope in amlich-api, added explicit ruleset/pack validation, and wired contextual recommendation activation metadata through the canonical JSON response. Targeted API tests now cover invalid selector failures and baseline-vs-context separation.",
  "whatWasImplemented": "Added canonical engine request/response types in `crates/amlich-api`, tightened selector validation for rulesets and recommendation packs, and updated recommendation transport so contextual output is absent on baseline requests and attributable when controls are enabled. Also added catalog-facing metadata needed for later CLI discovery wiring.",
  "whatWasLeftUndone": "CLI discovery commands are not part of this feature and still need consumer-side integration in a later feature.",
  "verification": {
    "commandsRun": [
      {
        "command": "cargo check --package amlich-core --package amlich-api",
        "exitCode": 0,
        "observation": "Core and API compile cleanly after the contract changes."
      },
      {
        "command": "cargo test --package amlich-api recommendation_contract -- --nocapture",
        "exitCode": 0,
        "observation": "Recommendation contract tests passed, including the new baseline-vs-context assertions."
      },
      {
        "command": "cargo test --package amlich-api ruleset_catalog -- --nocapture",
        "exitCode": 0,
        "observation": "Catalog tests passed and unknown selector paths now fail explicitly instead of falling back."
      },
      {
        "command": "cargo run --package amlich-cli -- day 2026-02-20 --format json --pretty",
        "exitCode": 0,
        "observation": "CLI JSON exposes the new metadata envelope and contextual output only appears when controls are passed."
      }
    ],
    "interactiveChecks": [
      {
        "action": "Compared baseline and selector-enabled day JSON output for the same date.",
        "observed": "Baseline recommendations stayed stable while contextual recommendations gained active-pack metadata only when the selector was enabled."
      }
    ]
  },
  "tests": {
    "added": [
      {
        "file": "crates/amlich-api/tests/engine_request_contract.rs",
        "cases": [
          {
            "name": "engine_request_emits_canonical_metadata_and_gated_sections",
            "verifies": "Always-on fields remain present and optional sections obey include controls."
          },
          {
            "name": "unknown_ruleset_and_pack_ids_fail_explicitly",
            "verifies": "Bad selectors return deterministic actionable errors."
          }
        ]
      }
    ]
  },
  "discoveredIssues": [
    {
      "severity": "medium",
      "description": "Legacy CLI entrypoints still bypass the new catalog/discovery surfaces, so migration boundary work remains for a consumer feature.",
      "suggestedFix": "Handle in the later CLI discovery and legacy-boundary feature rather than patching it ad hoc here."
    }
  ]
}
```

## When to Return to Orchestrator

- The feature requires a CLI shape or migration decision that is not spelled out in the feature description.
- `amlich-core` lacks a stable seam for the requested selector or catalog behavior and multiple architectural directions are plausible.
- Existing dirty-worktree changes make it impossible to isolate the contract edit safely.
- A required assertion in `validation-contract.md` cannot be made testable from the engine/API layer alone.
