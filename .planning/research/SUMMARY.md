# Project Research Summary

**Project:** amlich — Vietnamese Almanac Correctness Validation
**Domain:** Classical Vietnamese almanac (am-lich) cross-referencing against authoritative printed source (KHCBPPT)
**Researched:** 2026-02-28
**Confidence:** HIGH (codebase analysis), MEDIUM (classical text interpretation)

## Executive Summary

The amlich project has a Rust workspace with a working almanac engine (amlich-core) that computes DayFortune values — star rules, taboo rules, day deity, trực, xung hợp, thần hướng, nạp âm, and 28-star cycle — for any Vietnamese solar/lunar date. The core problem is that the existing test suite verifies internal consistency only: expected values in golden tests were written by the implementer against their own output, not against the authoritative classical text (Khâm Định Cổ Bản Phong Phú Toàn, KHCBPPT). This project must close that gap by building a machine-readable reference dataset derived directly from KHCBPPT and then running automated comparison validators against the live implementation.

The recommended approach is a four-phase pipeline: (1) establish the authoritative source edition and manually extract reference tables per subsystem into a typed JSON golden dataset, (2) build a thin Rust loader and per-subsystem validator harness that surfaces all divergences, (3) correct baseline.json and hardcoded source arrays to eliminate divergences, and (4) run end-to-end verification to confirm zero divergences. The entire validation stack uses only tools already present in the workspace — `cargo test`, `serde_json`, and `include_str!` — so no new dependencies are introduced. The critical constraint is that Phase 1 is a manual research task that cannot be automated or shortcut.

The key risk is mistaking source fidelity for internal consistency. Several subsystems carry `source_id: "khcbppt"` metadata without any real cross-reference having occurred. A secondary structural risk is that star-rule contextual data in baseline.json appears to have only 1 entry per category, which may mean hundreds of rules are simply missing rather than incorrect. Both risks are mitigated by the same discipline: build the golden dataset from KHCBPPT first, never from implementation output, and perform a completeness audit (expected count vs. actual count) before any correction work.

## Key Findings

### Recommended Stack

The entire validation stack is already present in the workspace. No new dependencies are justified. Rust's `#[test]` framework with `cargo test` is the test harness; `serde`/`serde_json` handles golden dataset deserialization; `include_str!` embeds the JSON at compile time, matching the existing `baseline.json` pattern. The golden dataset uses JSON format (not CSV) for proper Unicode handling and structured multi-field entries.

**Core technologies:**
- `cargo test` / `#[test]` — test harness for parametric golden dataset validation — already in workspace
- `serde` + `serde_json` 1.0 — typed deserialization of golden JSON — already in workspace
- `include_str!` macro — compile-time embedding of golden dataset — standard library, no cost

What to avoid: Python scripts (adds language boundary), automated OCR (too error-prone on classical Vietnamese/Chinese text), external almanac apps as reference (not authoritative), `proptest` (generates random inputs, cannot validate KHCBPPT table values), CSV format (Unicode problems).

### Expected Features

The scope is bounded to cross-referencing eight subsystems against KHCBPPT and producing a corrected implementation. Nạp âm is a scope question: its `source_id` is "tam-menh-thong-hoi" not "khcbppt", so in-scope status must be decided before the golden dataset schema is finalized.

**Must have (table stakes — audit is incomplete without these):**
- Star rules (cat_tinh/sat_tinh) cross-reference — 5 category buckets; contextual buckets are suspiciously sparse
- Taboo rules cross-reference — 4 rules, ~20 values, LOW cost HIGH value
- Day deity cycle cross-reference — 12 names × classification + 12 month-start offsets = 36 values
- Trực quality assignments cross-reference — 12 values hardcoded in source, never verified against KHCBPPT
- Thần hướng directions cross-reference — 30 values; prior corrections already applied (commit 0f29f3f), suggesting this area is error-prone
- Golden reference dataset (~200 representative dates, 2020–2030, covering all pattern combinations)

**Should have (deeper validation):**
- Star rule completeness audit — determine if sparse contextual buckets are correct or massively incomplete
- 28-star JD epoch verification — `jd.rem_euclid(28)` offset correctness; existing test only checks bounds
- Sát Hướng directional verification
- Precedence algorithm textual verification (6-tier order)

**Defer (out of scope for this project):**
- Lunar/solar conversion (already well-tested separately)
- Giờ Hoàng Đạo auspicious hours (separate calculation chain)
- New almanac subsystems
- TUI/CLI/WASM display layer changes
- Performance optimization

### Architecture Approach

The architecture is strictly additive. Four new artifacts are introduced alongside the existing test suite without modifying any existing test files. The golden dataset (`khcbppt-golden.json`) is the authoritative judge; `baseline.json` is the defendant. A shared `golden_loader.rs` deserializes the dataset; per-subsystem `khcbppt_*.rs` test files each perform parametric comparisons with collect-all failure reporting (so the full divergence scope is visible in a single run). `cargo test --package amlich-core` discovers all files automatically; no `Cargo.toml` changes are needed.

**Major components:**
1. `khcbppt-golden.json` — machine-readable KHCBPPT reference data; located at `crates/amlich-core/data/almanac/`; every entry carries a `khcbppt_ref` citation
2. `golden_loader.rs` — test-only shared module; deserializes `GoldenEntry` structs for all validators
3. `khcbppt_stars.rs`, `khcbppt_taboos.rs`, `khcbppt_deity.rs`, `khcbppt_truc.rs`, `khcbppt_xung_hop.rs`, `khcbppt_than_huong.rs`, `khcbppt_na_am.rs` — per-subsystem validators at `crates/amlich-core/tests/`
4. Corrected `baseline.json` and source-code constants (e.g., `TRUC_QUALITY` in `truc.rs`) — produced by Phase 4

### Critical Pitfalls

1. **Tests ≠ KHCBPPT validation** — Existing tests were written by the implementer against their own output. `source_id: "khcbppt"` metadata is not cross-referencing. Build the golden dataset from the text first; never reverse-engineer expected values from current output.

2. **JD epoch offset for 28-star cycle** — `jd.rem_euclid(28)` in `calc.rs:46` is untested for actual star identity. If the offset is wrong by 1, every 28-star result is shifted. Verify with 3+ real KHCBPPT dated entries before any other star validation.

3. **Trực quality array is hardcoded in source, not data** — `TRUC_QUALITY` in `truc.rs` is a Rust const, not a JSON entry. Correction requires a code change and recompile. Popular almanacs disagree on whether Trừ and Nguy are cat/binh/hung.

4. **Star rule completeness vs. correctness** — baseline.json contextual star buckets have only 1 entry each. Cross-referencing may surface that values are correct but hundreds of rules are missing entirely. Establish expected entry counts from KHCBPPT before correction work.

5. **Wrong KHCBPPT edition** — Multiple editions/reprints exist; modern Vietnamese compilations citing KHCBPPT may derive from 20th-century adaptations. Pin the specific edition in golden dataset metadata at project kickoff or all subsequent work is on an unstable foundation.

## Implications for Roadmap

Based on research, suggested phase structure:

### Phase 1: Source Establishment and Reference Data Extraction
**Rationale:** Everything downstream depends on having authoritative KHCBPPT values. This phase cannot be automated. It unblocks all other work and determines the schema for the golden dataset. The KHCBPPT edition must be pinned here to prevent the wrong-edition pitfall.
**Delivers:** Specific KHCBPPT edition documented; raw reference tables per subsystem extracted; nạp âm scope decision made; representative ~200 date set selected covering all pattern combinations.
**Addresses:** Golden reference dataset (P1), nạp âm scope determination (P1 blocker), trực quality (12 values), taboo rules (4 rules), day deity (36 values), thần hướng (30 values)
**Avoids:** Pitfall 6 (wrong edition), Pitfall 1 (circular validation), Pitfall 5 (completeness audit precedes correction)

### Phase 2: Golden Dataset and Loader Infrastructure
**Rationale:** Once raw reference tables exist, they must be serialized into machine-readable form and the loader infrastructure built so validators can be written. Schema must accommodate multi-source entries (some subsystems cite non-KHCBPPT sources).
**Delivers:** `khcbppt-golden.json` with ~200 fully cited entries; `golden_loader.rs` with `GoldenEntry` structs; schema validated against all subsystem fields.
**Uses:** `serde`/`serde_json`, `include_str!` macro, JSON format
**Implements:** Golden Dataset and Golden Loader components
**Avoids:** Pitfall 8 (run `cargo test` after every JSON edit), Pitfall 1 (every entry has `khcbppt_ref`)

### Phase 3: Validator Harness and Divergence Inventory
**Rationale:** With the golden dataset in place, validators can be written per subsystem to surface all divergences before any corrections are made. Fixing before full inventory hides total scope.
**Delivers:** All `khcbppt_*.rs` validator files passing compilation; full divergence inventory per subsystem visible from a single `cargo test` run; no corrections made yet.
**Uses:** `cargo test` parametric test pattern, collect-all failure reporting
**Implements:** All subsystem validator components
**Avoids:** Pitfall 2 (JD offset verified first within stars validator), Pitfall 3 (all 12 months covered in deity validator), Pitfall 7 (intercalary month date included in dataset)

### Phase 4: Correction and Zero-Divergence Verification
**Rationale:** Only after the complete divergence inventory is known should fixes be applied. Corrections may require changes to `baseline.json` (data changes) or Rust source constants (code changes), or both.
**Delivers:** Corrected `baseline.json` and source constants (e.g., `TRUC_QUALITY`); `cargo test --package amlich-core` passes with zero divergences including all new `khcbppt_*.rs` validators; all existing regression tests still pass.
**Avoids:** Pitfall 8 (cargo test after every JSON edit), Pitfall 4 (trực quality table has direct KHCBPPT citation after correction)

### Phase Ordering Rationale

- Phase 1 precedes everything because the golden dataset is the foundation — no validator can be written without reference values, and no schema can be finalized until the nạp âm scope question is answered.
- Phase 2 before Phase 3 because validators depend on the loader and dataset existing and compiling.
- Phase 3 surfaces the full divergence inventory before Phase 4 applies any fixes — this ordering prevents partial corrections that obscure the total scope of errors.
- Thần hướng and taboo validation can proceed in parallel within Phase 3 since their data dependencies are independent.

### Research Flags

Phases likely needing deeper research during planning:
- **Phase 1:** Classical text extraction from KHCBPPT is a manual research task with no automated shortcuts. The specific edition, its table structure, and how it handles intercalary months are not fully documented in the codebase. Needs dedicated KHCBPPT text analysis before planning can scope the work precisely.
- **Phase 3 (stars subsystem):** Star rule completeness is the highest-complexity P1 item. The contextual bucket sparseness issue means the validator must be designed to detect missing rules, not just incorrect values — this requires upfront design decisions.

Phases with standard patterns (skip research-phase):
- **Phase 2:** Golden dataset schema and serde loader follow existing workspace patterns exactly (`baseline.json`, `day-info-golden.json`). No novel patterns.
- **Phase 4:** Corrections to JSON data and Rust const arrays are mechanical once the divergence inventory is known. Standard workflow.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | Derived from direct codebase analysis; all tools already present in workspace |
| Features | HIGH (scope), MEDIUM (completeness) | Subsystem boundaries are clear from code; KHCBPPT content per subsystem requires manual verification |
| Architecture | HIGH | Additive pattern follows existing workspace conventions exactly; no novel architectural decisions |
| Pitfalls | HIGH (codebase), MEDIUM (classical text) | Code pitfalls identified from direct inspection; classical text interpretation pitfalls inferred from domain knowledge |

**Overall confidence:** MEDIUM-HIGH

### Gaps to Address

- **KHCBPPT edition identification:** The specific edition and its structural differences from modern adaptations are unknown until Phase 1 manual research. This is a prerequisite blocker.
- **Nạp âm scope:** `na_am_meta.source_id` is "tam-menh-thong-hoi" not "khcbppt". The decision of whether to include or exclude nạp âm from the golden dataset schema must be made at Phase 1 kickoff — the schema cannot be finalized without it.
- **Star rule completeness baseline:** The expected entry count per star category from KHCBPPT is unknown. Without this count, a completeness audit cannot be performed — only a value correctness audit.
- **Intercalary month handling:** KHCBPPT's treatment of intercalary months for taboo and trực rules is not documented in the codebase. Phase 1 must explicitly extract this from the text.

## Sources

### Primary (HIGH confidence)
- `crates/amlich-core/` codebase — all subsystem implementations, data locations, existing test patterns
- `crates/amlich-core/data/almanac/baseline.json` — current rule data, source metadata, structural patterns
- Git history (commit 0f29f3f) — evidence of prior thần hướng corrections indicating error-prone area

### Secondary (MEDIUM confidence)
- Existing test files (`almanac_golden.rs`, `ruleset_determinism.rs`, `taboo_boundary.rs`) — what is and is not currently verified
- `baseline.json` source_id metadata — subsystem sourcing claims (unverified cross-references)

### Tertiary (LOW confidence — requires Phase 1 validation)
- KHCBPPT classical text content — table structure, intercalary month handling, edition differences
- Comparison between modern Vietnamese almanac apps and KHCBPPT — not used as reference (not authoritative)

---
*Research completed: 2026-02-28*
*Ready for roadmap: yes*
