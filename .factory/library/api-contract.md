# API Contract

Worker-facing notes about the engine contract and migration targets.

**What belongs here:** canonical request/response expectations, catalog/discovery expectations, migration invariants.

---

- The target contract is an engine-style request model, not another fixed DTO bundle retrofit.
- Always-on base fields and optional sections should be distinguished explicitly.
- Baseline and contextual recommendation outputs must remain separate.
- Catalog/discovery surfaces should expose canonical ids and enough metadata to explain runtime behavior.
- TUI selectors should reuse catalog/discovery surfaces rather than inventing parallel local option lists.
- Canonical identity fields that matter across surfaces in this mission are `schema_version`, `ruleset_id`, `ruleset_version`, and `profile`.
- When contextual recommendations are active, workers must preserve the distinction between bundle-level identity and recommendation-layer identity.
- Active pack runtime provenance should remain visible through `pack_id`, `version`, `source_family`, and `mode`.
- CLI-first discovery surfaces available for this mission:
  - `lookup rulesets`
  - `lookup recommendation-packs`
- The planned architecture RFC path is `docs/api/engine-contract-rfc.md`.
- Legacy CLI entrypoints may remain temporarily, but their migration boundary must be explicit rather than accidental.
- Headless machine outputs must stay parseable and keep stdout/stderr hygiene even when warnings or deprecations are emitted.
