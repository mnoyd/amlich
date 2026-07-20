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

## 2. Parallel-execution in-flight state on `crates/amlich-core/src/{semantic_graph,reasoning,iching}/` during Plan 20-01

- **Discovered during:** Plan 20-01 Task 1 GREEN-phase verification (`cargo test -p amlich-core sources`)
- **Source:** Config has `parallelization: true`; Plans 20-02 and 20-03 are executing concurrently in the same working tree. Observed real-time modifications to `crates/amlich-core/src/semantic_graph/ontology.rs`, `crates/amlich-core/src/semantic_graph/views/{helpers,visualization}.rs`, `crates/amlich-core/src/reasoning/types.rs`, plus untracked `crates/amlich-core/src/iching/` and `crates/amlich-core/data/iching/` directories — all belonging to Plan 20-02 / 20-03 in-flight work.
- **Symptom:** `cargo test -p amlich-core --lib sources::` cannot compile because the parallel agents' work-in-progress leaves the lib crate in a non-compiling state (e.g., `NodeConcept::Hexagram` enum variant added but match arms in `views/helpers.rs` / `visualization.rs` not yet updated — Pitfall 3 from 20-RESEARCH.md).
- **Scope:** NOT caused by Plan 20-01 changes. Plan 20-01 only touches `crates/amlich-core/src/sources.rs` (2 new `pub const`) + `crates/amlich-core/tests/source_id_guard.rs` (2 new `FORBIDDEN_LITERALS` entries). Both are mechanically trivial and follow the existing 7-const pattern exactly.
- **Verification substitute:** `cargo test -p amlich-core --test source_id_guard` PASSES (1 test, verified — source_id_guard.rs is a standalone test target that does not require the lib crate to compile fully). The `sources::tests::all_constants_have_expected_values` test is a pure `assert_eq!` on locally-defined `pub const` values — its pass/fail depends ONLY on the new constants matching the new asserts, which they do by construction.
- **Action required:** None from Plan 20-01. Plan 20-02 / 20-03 GREEN phases will resolve the lib-crate compile state. Once those plans complete, `cargo test -p amlich-core sources` will pass (the test references only `sources.rs`-local constants).
