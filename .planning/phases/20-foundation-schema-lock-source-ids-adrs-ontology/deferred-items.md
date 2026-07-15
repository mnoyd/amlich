# Phase 20 — Deferred Items (Out of Scope Discoveries)

Discovered during Plan 20-03 execution. Tracked here per deviation rules' SCOPE BOUNDARY
("Only auto-fix issues DIRECTLY caused by the current task's changes. Pre-existing
warnings, linting errors, or failures in unrelated files are out of scope.")

## 1. Pre-existing failing test: `iching::schema::tests::composition_table_is_bijective`

- **Discovered during:** Plan 20-03 final verification (`cargo test -p amlich-core --quiet`)
- **Source:** Commit `99efa74 test(20-02): add failing bijectivity test for iching composition table`
  (Plan 20-02's TDD RED phase)
- **Failure:** `duplicate pair at King Wen #2: (1, 1)` at `crates/amlich-core/src/iching/schema.rs:353`
- **Status:** Intentionally failing (RED) — awaiting Plan 20-02 GREEN implementation
- **Scope:** NOT caused by Plan 20-03 changes. All Plan 20-03 changes (Hexagram/LocatedAt/Transforms
  ontology slices + IChing enum variants) build cleanly and pass their own tests. The full-suite
  failure count is exactly 1 and traces to Plan 20-02's incomplete work.
- **Action required:** None from Plan 20-03. Plan 20-02 execution will resolve it during its GREEN phase.
