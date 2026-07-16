---
phase: 22-mai-hoa-casting-bien-que-the-dung
plan: 01
subsystem: reasoning
tags: [iching, kinh-dich, mai-hoa-dich-so, casting, bien-que, deterministic, crit-2, crit-4, crdt-3, wasm-safe, adr-0006]

# Dependency graph
requires:
  - phase: 21-iching-corpus-loader
    provides: "Locked HexagramEntry schema (ADR-0005), three CRIT-3-isolating newtypes (TienThienTrigram, HauThienTrigram, KingWenHexagram), bijective COMPOSITION_TABLE + compose() (the only bridge), get_hexagram/all_hexagrams lookup API"
  - phase: 20-foundation-schema-lock-source-ids-adrs-ontology
    provides: "ADR-0006 (Mai Hoa casting convention + worked boundary example), TienThienTrigram encoding (Kiền=1..Khôn=8), Compose table"
provides:
  - "MaiHoaCast struct (lunar inputs + Tiên Thiên pair + động hào + chủ quẻ King Wen index) — pure-data traceability record for a single casting"
  - "cast_mai_hoa(lunar_year_branch, lunar_month, lunar_day, chi_hour_index) -> MaiHoaCast — deterministic Mai Hoa Dịch Số casting per ADR-0006 §3-§4"
  - "mai_hoa_remainder((sum, k)) -> i32 — the SINGLE named helper implementing the CRIT-2 boundary-safe ((sum-1)%k)+1 reduction; the structural gate that prevents the naive `sum % k` convention from regressing"
  - "BienQue struct (new Tiên Thiên pair + King Wen index + flipped_dong_hao echo) — pure-data traceability record for a biến quẻ derivation"
  - "derive_bien_que(&MaiHoaCast) -> BienQue — flip động hào line + re-compose via COMPOSITION_TABLE"
  - "trigram_lines(TienThienTrigram) -> [u8; 3] + lines_to_trigram([u8; 3]) -> TienThienTrigram — bijective 8-pattern Bā Guà ↔ 3-line bit mapping (bottom-to-top, yang=1, yin=0)"
  - "9 inline lib tests + 6 black-box integration tests covering ICH-02 (CRIT-2 boundary, determinism, 51,840-cast range sweep) and ICH-03 (CRIT-4 384-case exhaustive contract + worked #7 Sư derivation + trigram-order inversion guard)"
affects:
  - 22-02-mai-hoa-casting-the-dung (next plan in phase — Thể/Dụng classification + Ngũ Hành sinh/khắc + ≥10 cross-source golden cases; consumes MaiHoaCast + derive_bien_que)
  - 24-iching-evaluator-semantic-graph-wiring-dto (IChingEvaluator + Hexagram semantic-graph nodes will be sourced from cast_mai_hoa + derive_bien_que + get_hexagram)
  - 25-e2e-validation-golden-cross-source (golden cross-source verification will exercise cast_mai_hoa on the cross-source dataset)

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "CRIT-2 lock via single named reduction helper (mai_hoa_remainder) — the boundary test guards against refactors that replace ((n-1)%k)+1 with sum%k or (sum%k)+1"
    - "TDD RED→GREEN two-commit discipline for algorithm + invariants — RED commit's tests fail with 'not implemented: RED phase', GREEN commit's implementation passes them"
    - "Inline grep-guard for CRIT-3 isolation with runtime-built needle patterns — avoids the self-tripping trap where the test's own source code contains the literal forbidden strings"
    - "Black-box integration tests via external crate path (use amlich_core::iching::{...}) — mirrors Phase 21-02 iching_corpus_integration.rs discipline"
    - "Synthetic MaiHoaCast construction in tests (fields are pub) — enables 384-case contract test by directly specifying (upper, lower, dong_hao) without round-tripping through cast_mai_hoa"

key-files:
  created:
    - crates/amlich-core/src/iching/mai_hoa.rs
    - crates/amlich-core/src/iching/bien_que.rs
    - crates/amlich-core/tests/mai_hoa_casting_integration.rs
  modified:
    - crates/amlich-core/src/iching/mod.rs

key-decisions:
  - "MaiHoaCast retains all four lunar inputs on the struct (not just the derived pair) — preserves traceability / recasting; field ranges documented in doc-comments"
  - "mai_hoa_remainder is the SINGLE named CRIT-2 reduction helper — the structural gate; replacing it with `sum % k` or `(sum % k) + 1` regresses CRIT-2 (boundary test would fail at the all-eights case)"
  - "trigram_lines + lines_to_trigram use the canonical 8 classical Bā Guà patterns (Kiền ☰ = [1,1,1] ... Khôn ☷ = [0,0,0]); lines indexed bottom-to-top (index 0 = line 1 = bottom hào) matching Vietnamese Mai Hoa convention"
  - "BienQue.flipped_dong_hao echoes the input cast's dong_hao (no new info) — convenience for downstream consumers that want a self-contained biến quẻ record without cross-referencing the original cast"
  - "TDD RED → GREEN → integration suite = three commits, in order. RED commit ships stub + failing tests; GREEN commit ships implementation; integration suite commit ships the black-box discipline"
  - "CRIT-3 grep guards use RUNTIME-BUILT needles (format! at test runtime) — the test's own source contains doc-comment mentions of `impl From<TienThienTrigram>` etc., so a literal-needle check would self-trip"
  - "384-case biến quẻ contract test uses synthetic MaiHoaCast construction (fields are pub) — directly specifies (upper, lower, dong_hao) without round-tripping through cast_mai_hoa. This decouples CRIT-4 verification from CRIT-2 correctness (any CRIT-2 bug would only affect specific input tuples; the 384-case sweep is independent)"

patterns-established:
  - "Named-helper CRIT lock: when an invariant has a single subtle convention (e.g. ((n-1)%k)+1, off-by-one, boundary-encoding), extract it into a single named helper whose name documents the invariant. The test that guards the invariant then references the helper by name, making the contract explicit."
  - "Synthetic struct construction for exhaustive property tests: when a struct's fields are pub and the test wants to enumerate the input space (vs round-tripping through the public API), construct directly. This decouples property-test correctness from upstream-API correctness — different invariants under test."
  - "Runtime-built needle strings in self-referential grep guards: when the test guards a forbidden-string literal that the test itself must mention in doc-comments, build the needle string at runtime via format!() so the literal doesn't appear in the source."

requirements-completed: [ICH-02, ICH-03]

# Metrics
duration: 13 min
completed: 2026-07-16
---

# Phase 22 Plan 01: Mai Hoa Casting + Biến Quẻ Summary

**Pure-deterministic Mai Hoa Dịch Số casting (`cast_mai_hoa` + `MaiHoaCast` struct) and biến quẻ derivation (`derive_bien_que` + `BienQue` struct) closing ICH-02 and ICH-03 with the CRIT-2 boundary-safe `((n-1)%k)+1` convention and the CRIT-4 384-case (64 chủ quẻ × 6 động hào) exhaustive contract test**

## Performance

- **Duration:** 13 min (798 s)
- **Started:** 2026-07-16T03:38:35Z
- **Completed:** 2026-07-16T03:51:53Z
- **Tasks:** 2 (Task 1 = TDD red→green; Task 2 = black-box integration suite)
- **Task commits:** 3 (RED, GREEN, integration suite)
- **Files created:** 3 (`mai_hoa.rs`, `bien_que.rs`, `mai_hoa_casting_integration.rs`)
- **Files modified:** 1 (`iching/mod.rs`)
- **Total tests added:** 15 (9 inline + 6 integration); all passing
- **Crate test suite:** 962 tests, 0 failures, 0 regressions vs Phase 21-02 baseline

## Accomplishments

- **`crates/amlich-core/src/iching/mai_hoa.rs` (~250 lines)** implements:
  - `MaiHoaCast` struct — 4 lunar inputs preserved for traceability + Tiên Thiên pair + động hào + chủ quẻ King Wen index; derives `Debug/Clone/PartialEq/Eq/Serialize/Deserialize`
  - `mai_hoa_remainder(sum: i32, k: i32) -> i32` — the SINGLE named CRIT-2 helper implementing `((sum - 1) % k) + 1`; doc-comment explicitly warns that replacing it with `sum % k` or `(sum % k) + 1` regresses CRIT-2
  - `cast_mai_hoa(lunar_year_branch, lunar_month, lunar_day, chi_hour_index) -> MaiHoaCast` — pure integer arithmetic per ADR-0006 §3-§4; no RNG, no wall-clock, no filesystem
  - 5 inline tests including the CRIT-2 HEADLINE `crit2_all_eights_yields_khon` (cites ADR-0006 §4 verbatim, explicitly rejects the naïve Kiền regression), determinism (`casting_is_deterministic`), non-boundary (1,1,1,1)→Ly/Chan/#21/4, full 51,840-cast range sweep, CRIT-3 isolation grep guard
- **`crates/amlich-core/src/iching/bien_que.rs` (~250 lines)** implements:
  - `BienQue` struct — new Tiên Thiên pair + King Wen index + `flipped_dong_hao` echo; derives `Debug/Clone/PartialEq/Eq/Serialize/Deserialize`
  - `trigram_lines(TienThienTrigram) -> [u8; 3]` — 8 classical Bā Guà patterns (Kiền ☰ = [1,1,1] .. Khôn ☷ = [0,0,0]); lines indexed bottom-to-top
  - `lines_to_trigram([u8; 3]) -> TienThienTrigram` — reverse lookup via linear scan over `TienThienTrigram::ALL`
  - `derive_bien_que(&MaiHoaCast) -> BienQue` — flip động hào line + re-compose via `COMPOSITION_TABLE`
  - 4 inline tests including `crit4_all_eights_bien_que_is_kw7_su` (worked (8,8,8,8)→#7 Sư case per COMPOSITION_TABLE line 189; explicitly rejects #8 Tỷ to guard against trigram-order inversion), bijectivity round-trip, 384-case CRIT-4 HEADLINE, CRIT-3 isolation grep guard
- **`crates/amlich-core/src/iching/mod.rs`** registers `pub mod mai_hoa;` + `pub mod bien_que;` + re-exports `cast_mai_hoa`/`MaiHoaCast`/`derive_bien_que`/`BienQue` alongside the existing schema + corpus re-exports
- **`crates/amlich-core/tests/mai_hoa_casting_integration.rs` (296 lines)** ships 6 black-box integration tests from the external crate path:
  1. **`crit2_all_eights_boundary_yields_khon_not_kien`** — CRIT-2 HEADLINE; explicit assertion of #2 + explicit rejection of #1 regression
  2. **`casting_is_deterministic_and_rng_free`** — two equal calls + 20-tuple × 3-repeated sweep
  3. **`remainder_indices_always_in_range`** — full 51,840-cast sweep asserting `dong_hao ∈ 1..=6`, `chu_que ∈ 1..=64`, every Tiên Thiên trigram visited at least once on both sides
  4. **`crit4_bien_que_384_case_exhaustive_contract`** — CRIT-4 HEADLINE; 64 × 6 = 384 cases; every biến quẻ is valid, differs from chủ quẻ, flips EXACTLY ONE trigram
  5. **`bien_que_known_case_all_eights`** — worked #7 (Sư) derivation; explicit rejection of #8 (Tỷ) trigram-order inversion trap
  6. **`crit3_isolation_no_cross_newtype_from_impls`** — runtime-built needle patterns guard against any cross-newtype From impl + WASM-safety (no `std::fs::`, no `Utc::now`, no `rand::`)
- **TDD discipline observed**: RED commit (`fb13272` — 9 inline tests fail with "not implemented: RED phase"), GREEN commit (`5d61b7d` — implementation passes all 9 + CRIT-3 grep test), integration suite commit (`e077210` — 6 black-box tests from external path)
- **Zero regressions** across the crate: 738 lib tests + 7 doc tests + all integration suites pass
- **CRIT-2 + CRIT-4 both gated**: the all-eights boundary (8,8,8,8)→Khôn/Khôn/#2/dong=2 is the worked proof; (8,8,8,8)→biến quẻ=#7 Sư (compose(Khôn, Khảm)) per COMPOSITION_TABLE line 189
- **CRIT-3 isolation preserved**: `rg "impl From"` on both new files returns only doc-comment mentions + runtime-built test needles (zero actual cross-newtype From impls)

## Task Commits

Each task was committed atomically (TDD on Task 1 produced the conventional RED → GREEN pair):

1. **Task 1 RED: failing tests for Mai Hoa cast + biến quẻ** — `fb13272` (test)
   - `crates/amlich-core/src/iching/mai_hoa.rs` (created, 246 lines) — `MaiHoaCast` struct + `cast_mai_hoa` stub (`unimplemented!`) + 5 inline tests
   - `crates/amlich-core/src/iching/bien_que.rs` (created, 254 lines) — `BienQue` struct + `derive_bien_que` stub + 4 inline tests
   - `crates/amlich-core/src/iching/mod.rs` — register modules + re-exports
   - 8 of 9 tests fail with "not implemented: RED phase"; the CRIT-3 grep test correctly passes (no actual cross-newtype From impl exists yet)
2. **Task 1 GREEN: implement Mai Hoa cast + biến quẻ** — `5d61b7d` (feat)
   - `crates/amlich-core/src/iching/mai_hoa.rs` — `mai_hoa_remainder` CRIT-2 helper + `cast_mai_hoa` implementation per ADR-0006 §3-§4
   - `crates/amlich-core/src/iching/bien_que.rs` — `trigram_lines` + `lines_to_trigram` (8 Bā Guà patterns) + `derive_bien_que` (flip + re-compose)
   - All 9 inline tests pass
3. **Task 2: black-box integration suite for ICH-02 + ICH-03** — `e077210` (test)
   - `crates/amlich-core/tests/mai_hoa_casting_integration.rs` (created, 296 lines) — 6 black-box tests from external crate path
   - `crates/amlich-core/src/iching/mai_hoa.rs` — doc-comment algorithm pseudocode block changed from `rust` to `text` fence (was tripping `cargo test --doc`)
   - All 6 integration tests pass; full crate suite green (962 tests, 0 failures)

**Plan metadata:** `docs(22-01): complete Mai Hoa casting + biến quẻ plan` (commit pending below)

## Files Created/Modified

- `crates/amlich-core/src/iching/mai_hoa.rs` (created, ~250 lines) — `MaiHoaCast` struct + `cast_mai_hoa` deterministic casting + `mai_hoa_remainder` CRIT-2 helper + 5 inline tests (CRIT-2 boundary, determinism, non-boundary (1,1,1,1)→#21, 51,840-cast range sweep, CRIT-3 isolation grep guard)
- `crates/amlich-core/src/iching/bien_que.rs` (created, ~250 lines) — `BienQue` struct + `derive_bien_que` + `trigram_lines` + `lines_to_trigram` (8 Bā Guà patterns) + 4 inline tests (trigram-lines bijectivity, CRIT-4 worked (8,8,8,8)→#7 Sư, 384-case contract, CRIT-3 isolation grep guard)
- `crates/amlich-core/tests/mai_hoa_casting_integration.rs` (created, 296 lines) — 6 black-box integration tests for ICH-02 + ICH-03 from external crate path
- `crates/amlich-core/src/iching/mod.rs` (modified, 19 → 22 lines) — registers `pub mod mai_hoa;` + `pub mod bien_que;` + re-exports `cast_mai_hoa`/`MaiHoaCast`/`derive_bien_que`/`BienQue`

## Decisions Made

- **MaiHoaCast retains all four lunar inputs on the struct** (not just the derived pair) — preserves traceability / recasting. Field ranges (year_branch 0..=11, month 1..=12, day 1..=30, hour 0..=11, dong_hao 1..=6) documented in doc-comments; out-of-range is a caller contract violation consistent with the rest of the crate.
- **`mai_hoa_remainder` is the SINGLE named CRIT-2 reduction helper** — the structural gate. The doc-comment explicitly warns: "Replacing this helper with `sum % k` or `(sum % k) + 1` regresses CRIT-2." Per the research SUMMARY's pitfall 2 ("implement as named helpers"), this concentrates the convention into one auditable location.
- **`trigram_lines` + `lines_to_trigram` use the canonical 8 classical Bā Guà patterns** (Kiền ☰ = [1,1,1] ... Khôn ☷ = [0,0,0]); lines indexed bottom-to-top (index 0 = line 1 = bottom hào). The reverse lookup uses a linear scan over `TienThienTrigram::ALL` (8 entries — a pre-computed reverse map is premature).
- **`BienQue.flipped_dong_hao` echoes the input cast's `dong_hao`** — convenience for downstream consumers that want a self-contained biến quẻ record without cross-referencing the original cast. No new information.
- **TDD RED → GREEN → integration suite = three commits, in order** — `fb13272` (RED stubs + tests), `5d61b7d` (GREEN implementation), `e077210` (black-box integration suite from external crate path). The integration suite is a separate commit because it exercises the public API only (caller perspective) — distinct invariant class from the inline tests (module-internal correctness).
- **CRIT-3 grep guards use RUNTIME-BUILT needles** — the test's own doc-comments mention `impl From<TienThienTrigram>` etc. as legitimate documentation. A literal-needle grep would self-trip. The fix (per `format!("impl From<{a}{b}")` at runtime) was a Rule 1 bug encountered during RED-phase compilation; documented as a deviation below.
- **384-case biến quẻ contract test uses synthetic `MaiHoaCast` construction** (fields are pub) — directly specifies (upper, lower, dong_hao) without round-tripping through `cast_mai_hoa`. This DECOUPLES CRIT-4 verification from CRIT-2 correctness: a CRIT-2 bug would only affect specific input tuples, while the 384-case sweep exercises every (upper, lower, dong_hao) triple independently.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] CRIT-3 grep guard initially self-tripped on doc-comment text**
- **Found during:** Task 1 RED-phase compilation (`cargo test -p amlich-core --lib iching::mai_hoa`)
- **Issue:** The first iteration of `crit3_isolation_no_cross_newtype_from_impls_inline` checked `!SRC.contains("impl From")` and `!SRC.contains("impl From<TienThienTrigram")` etc. with literal needles. The test's own doc-comment legitimately contained the strings `impl From<TienThienTrigram>`, `impl From<HauThienTrigram>`, `impl From<KingWenHexagram>` (explaining what the test was guarding against) — so the test failed on its own documentation. A grep guard that fires on its own rationale text is broken.
- **Fix:** Rewrote the test to BUILD the needle patterns at RUNTIME via `format!("impl From<{a}{b}")` where `(a, b)` is `("Tien", "ThienTrigram")` etc. The runtime-built string never appears as a literal in the source, so the test doesn't self-trip.
- **Files modified:** `crates/amlich-core/src/iching/mai_hoa.rs`, `crates/amlich-core/src/iching/bien_que.rs`
- **Verification:** RED phase: `crit3_isolation_no_cross_newtype_from_impls_inline` PASSES (correctly — no actual cross-newtype From impl exists in the stub); GREEN phase: same test still passes (correctly — no actual cross-newtype From impl in the implementation). Both phases behave as expected.
- **Committed in:** `fb13272` (RED commit) — the bug was fixed BEFORE the RED commit shipped, so the commit landed with the correct (runtime-built) guard pattern.

**2. [Rule 1 - Bug] Doc-test failure: algorithm pseudocode block was a `rust` fence but not valid Rust**
- **Found during:** Task 2 (`cargo test -p amlich-core` full crate suite)
- **Issue:** The module-level doc-comment in `mai_hoa.rs` contained a fenced code block showing the ADR-0006 algorithm as `/// ``` ` ... ` ``` ` (rust fence by default). The pseudocode (`sum_base = lunar_year_branch + lunar_month + lunar_day`) is not valid Rust syntax — `cargo test --doc` attempted to compile it as a doctest and failed at the first `=` token.
- **Fix:** Changed the fence type from ` ``` ` to ` ```text ` — the pseudocode is descriptive, not executable; `text` prevents cargo from trying to compile it as a Rust doctest.
- **Files modified:** `crates/amlich-core/src/iching/mai_hoa.rs`
- **Verification:** `cargo test -p amlich-core --doc` passes; `cargo test -p amlich-core` (full crate) passes with 0 failures.
- **Committed in:** `e077210` (Task 2 commit) — bundled with the integration suite because both are "make the doc/integration test surface clean" changes.

---

**Total deviations:** 2 auto-fixed (2 bugs — false-positive grep guard + false-positive doc-test fence).
**Impact on plan:** Both auto-fixes necessary for the plan's own verification gates (CRIT-3 grep test + `cargo test --doc`) to pass. No scope creep; no behavior change to the casting algorithm itself. Both fixes document reusable patterns for future grep-guard / doctest authors (the established patterns in the frontmatter above).

## Issues Encountered

None beyond the Rule 1 deviations above.

## Authentication Gates

None — no external services, no credentials, no CLI deployments. Pure Rust algorithm + tests against already-shipped Phase 20/21 types (`TienThienTrigram`, `HauThienTrigram`, `KingWenHexagram`, `COMPOSITION_TABLE`, `compose`).

## User Setup Required

None — no external service configuration required. This plan is pure Rust algorithm + tests against an already-shipped CRIT-3-isolated schema (Phase 20) + corpus (Phase 21). No new dependencies, no environment variables, no dashboards.

## Next Phase Readiness

- **ICH-02 is fully closed.** `cast_mai_hoa(lunar_year_branch, lunar_month, lunar_day, chi_hour_index) -> MaiHoaCast` exists, is deterministic (no RNG), honours the `((n-1) % k) + 1` remainder-zero convention (CRIT-2 boundary test passes — (8,8,8,8) → Khôn/#2, not Kiền/#1). 6 black-box integration tests from external crate path + 5 inline tests in the module.
- **ICH-03 is fully closed.** `derive_bien_que(&MaiHoaCast) -> BienQue` exists, flips exactly the động hào line, and the 384-case (64 chủ quẻ × 6 động hào) exhaustive contract test passes (CRIT-4 — every biến quẻ is valid, differs from its chủ quẻ, flips exactly one trigram). 6 black-box integration tests + 4 inline tests.
- **CRIT-3 isolation preserved.** `rg "impl From<(TienThienTrigram|HauThienTrigram|KingWenHexagram)> for "` returns zero matches; runtime-built needle grep guards in both modules + the integration test.
- **Ready for Plan 22-02** (Thể/Dụng classification + Ngũ Hành sinh/khắc + ≥10 cross-source golden cases; will consume `MaiHoaCast` + `derive_bien_que` + `TienThienTrigram::ALL`). **ICH-04 stays Pending.**
- **Phase 22 is 1/2 plans complete.** Ready for `/gsd-execute-phase 22-02`.
- **No blockers.**

---

*Phase: 22-mai-hoa-casting-bien-que-the-dung*
*Completed: 2026-07-16*

## Self-Check: PASSED

- All 4 created/modified files exist on disk: `mai_hoa.rs`, `bien_que.rs`, `mai_hoa_casting_integration.rs`, `iching/mod.rs`.
- All 3 task commits exist: `fb13272` (test RED), `5d61b7d` (feat GREEN), `e077210` (test integration suite).
- `mai_hoa.rs` contains the required patterns: `pub struct MaiHoaCast`, `pub fn cast_mai_hoa`, `fn mai_hoa_remainder` (CRIT-2 helper), inline tests including `crit2_all_eights_yields_khon`.
- `bien_que.rs` contains the required patterns: `pub struct BienQue`, `pub fn derive_bien_que`, `pub(crate) fn trigram_lines` + `lines_to_trigram` (8 Bā Guà patterns), inline tests including `crit4_bien_que_384_case_exhaustive_contract_inline` + `crit4_all_eights_bien_que_is_kw7_su`.
- `mod.rs` registers `pub mod mai_hoa;` + `pub mod bien_que;` + re-exports `cast_mai_hoa`, `MaiHoaCast`, `derive_bien_que`, `BienQue`.
- `cargo test -p amlich-core --lib iching::mai_hoa` → 5/5 inline tests pass.
- `cargo test -p amlich-core --lib iching::bien_que` → 4/4 inline tests pass.
- `cargo test -p amlich-core --test mai_hoa_casting_integration` → 6/6 black-box tests pass.
- `cargo test -p amlich-core` (full crate) → 962 tests across all suites, 0 failures, 0 regressions.
- `rg "impl From<(TienThienTrigram|HauThienTrigram|KingWenHexagram)> for " crates/amlich-core/src/iching/` returns ZERO matches — CRIT-3 isolation preserved.
- `rg "rand::|Utc::now|std::fs::" crates/amlich-core/src/iching/mai_hoa.rs crates/amlich-core/src/iching/bien_que.rs` returns ZERO matches — WASM-safety + determinism discipline preserved.
- ICH-02 + ICH-03 ready to be marked Complete in REQUIREMENTS.md.