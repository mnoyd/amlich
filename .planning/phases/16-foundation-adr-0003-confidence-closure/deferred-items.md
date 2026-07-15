# Phase 16 — Deferred Items

Pre-existing issues discovered during Phase 16 execution that are out of scope for this phase per the deviation rules (auto-fix only directly-caused issues; do not fix pre-existing tech debt).

## FMT/Clippy — Pre-existing on master (Phase 16 commit c76e741 + uncommitted changes)

**Verified scope:** `git stash && cargo clippy -p amlich-core --all-targets 2>&1 | grep -E "^error|^warning" | wc -l` returns **96** pre-existing errors+warnings on master without the Phase 16-01 working tree applied. Re-running the same count **with** the working tree returns **96** — no new clippy issues introduced by Phase 16.

**Pre-existing `cargo fmt --all --check` failures:** ~100+ across `crates/amlich-core/src/almanac/fengshui/{annual,aspects,combined,golden,monthly,period,safety,types}.rs`, `holiday_data.rs`, `holidays.rs`, `lib.rs`, `rituals/corpus.rs`, and CLI source. None introduced by Phase 16.

**Pre-existing `cargo clippy --all-targets -- -D warnings` failures:** 72 errors including `manual_range_contains`, `unnecessary_to_owned`, `too_many_arguments`, `collapsible_match`, etc. across `sun.rs`, `canchi.rs`, `tietkhi.rs`, `interaction/`, `rituals/`, `semantic_graph/`, etc. None introduced by Phase 16.

**Discipline:** Per `deviation_rules` SCOPE BOUNDARY: "Only auto-fix issues DIRECTLY caused by the current task's changes. Pre-existing warnings, linting errors, or failures in unrelated files are out of scope." Logged here for visibility.

**Recommendation for future Phase:** Add a dedicated tech-debt phase (or extend an existing phase) that runs `cargo fmt --all && cargo fix --clippy --allow-dirty --allow-staged` and commits the resulting mechanical cleanup. The v1.5 audit reported 886 tests passing without any clippy `-D warnings` gate, suggesting either an older clippy version was used, the gate was relaxed, or these issues accumulated post-v1.5.

---

*Recorded 2026-07-15 during execution of 16-01-PLAN.md.*
