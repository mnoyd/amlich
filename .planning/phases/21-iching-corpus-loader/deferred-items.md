# Deferred Items — Phase 21

Out-of-scope discoveries logged per GSD deviation rules (do NOT fix during this phase's execution; surface for future planning).

## Item 1: Pre-existing unused-import warnings (out of scope)

**Discovered during:** Plan 21-02 final `cargo test -p amlich-core` regression check.

**Symptom:**
```
warning: unused import: `ProvenanceSource`
warning: unused import: `ReasoningNodeSeverity`
warning: unused import: `GraphValidationError`
```

**Location:** `crates/amlich-core/src/semantic_graph/views/helpers.rs:115` and adjacent files in `semantic_graph/views/`.

**Why deferred:** These warnings exist in files UNRELATED to Plan 21-02's scope (the iching corpus loader). Per the deviation rule scope boundary ("Only auto-fix issues DIRECTLY caused by the current task's changes"), pre-existing warnings in unrelated files are out of scope. Touching them would conflate this plan's commit history with unrelated cleanup.

**Recommended action:** Address in a separate `chore: clean up unused semantic_graph imports` commit during a future maintenance/cleanup phase. Trivial mechanical fix (`cargo fix` would handle most of it).
