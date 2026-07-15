---
phase: 20-foundation-schema-lock-source-ids-adrs-ontology
plan: 01
subsystem: infra
tags: [source-id, adr, ci-guard, kinh-dich, mai-hoa-dich-so, provenance, schema-lock]

# Dependency graph
requires:
  - phase: 19-01 (Phase 19 Offering/SourceId alias foundation)
    provides: "pub const SOURCE_* discipline + tests/source_id_guard.rs grep pattern + DeferralMarker struct location"
provides:
  - "SOURCE_KINH_DICH = \"kinh-dich\" pub const in sources.rs (Ngô Tất Tố hexagram corpus source_id)"
  - "SOURCE_MAI_HOA_DICH_SO = \"mai-hoa-dich-so\" pub const in sources.rs (Thiệu Khang Tiết casting algorithm source_id)"
  - "FORBIDDEN_LITERALS in source_id_guard.rs extended to 9 entries (CRIT-6 cross-contamination prevention from day 1)"
  - "ADR-0005: HexagramEntry schema v1 locked (CRIT-1 × 7 schema-lock-first gate for Phase 21 corpus authoring)"
  - "ADR-0006: Mai Hoa casting convention locked (CRIT-2 remainder-zero boundary proof + CRIT-3 Tiên Thiên pin)"
  - "ADR-0007: Cross-link CRIT-3 carve-out placement contract (reasoning/direction_composite.rs + sibling grep guard)"
  - "DEC-0026 / DEC-0027 / DEC-0028 MILESTONES.md cross-references for the three ADRs"
affects:
  - 20-02 (HexagramEntry schema + three newtypes + composition table — implements ADR-0005 field set + ADR-0006 Tiên Thiên pin)
  - 20-03 (ontology extension — reserves EdgeConcept::LocatedAt + Transforms for ADR-0007 cross-link)
  - 21 (IChing corpus loader — implements ADR-0005 hao_tu length rule + deny_unknown_fields)
  - 22 (cast_mai_hoa — implements ADR-0006 convention; cites §4 worked boundary in contract test)
  - 23 (build_direction_cross_link — implements ADR-0007 placement + composite envelope)
  - 24 (DaySnapshot wiring — consumes ADR-0005 HexagramEntry + ADR-0007 cross-link via additive Option<T>)

# Tech tracking
tech-stack:
  added: []  # no new crates (v1.5/v1.6 "no new deps" precedent holds)
  patterns:
    - "pub const SOURCE_* + tests/source_id_guard.rs CI guard (extended for v1.7 — same discipline as v1.5 Phase 10)"
    - "Nygard short-form ADR (Title/Status/Context/Decision/Consequences) — same template as ADR-0001 + ADR-0004"
    - "Two-source pin from day 1 (AF-05) — ADR-0006 cites classical (Thiều Khang Tiết) + modern (nhantu.net)"
    - "Page-citation deferral with explicit PendingExternalReview marker (mirrors ADR-0004 §5)"
    - "Worked boundary example in ADR body (CRIT-2 prevention proof self-contained for contract-test citation)"

key-files:
  created:
    - ".planning/adrs/0005-hexagram-entry-schema-v1.md (HexagramEntry schema v1 — locks field set + naming divergence + hao_tu length rule + HauThienTrigram Lo Shu pin)"
    - ".planning/adrs/0006-mai-hoa-casting-convention.md (Mai Hoa casting — Tiên Thiên pin + lunar inputs + ((n-1)%k)+1 + worked Khôn boundary example)"
    - ".planning/adrs/0007-cross-link-crit3-carve-out.md (Cross-link placement contract — reasoning/direction_composite.rs + composite envelope + sibling grep guard)"
  modified:
    - "crates/amlich-core/src/sources.rs (+2 pub const SOURCE_KINH_DICH/SOURCE_MAI_HOA_DICH_SO + 2 assert_eq! in all_constants_have_expected_values test)"
    - "crates/amlich-core/tests/source_id_guard.rs (+2 entries in FORBIDDEN_LITERALS; total 9)"
    - ".planning/MILESTONES.md (+3 rows DEC-0026/0027/0028 in ADR Cross-References table)"
    - ".planning/phases/20-foundation-schema-lock-source-ids-adrs-ontology/deferred-items.md (+ item #2: parallel-execution in-flight state observation)"

key-decisions:
  - "HexagramEntry schema v1 locked BEFORE Phase 21 corpus authoring begins (CRIT-1 × 7 prevention: 64 hexagrams × ~7 text fields = 448 corpus fields × re-edit cost)"
  - "Naming-convention divergence from rituals locked: vi_name (language marker prefix for content) vs thoai_tu/hao_tu/cat_hung (romanized VN technical terms unmarked) — DIFFERS from rituals' body/body_en suffix pattern; ADR-0005 §3 documents the divergence so a future maintainer does not 'fix' it"
  - "hao_tu length rule: 6 entries for hexagrams #3..64; 7 entries for #1 Kiền (dụng cửu) + #2 Khôn (dụng lục) — loader invariant (Phase 21 enforces), NOT a serde constraint"
  - "reviewer: String free-text ExternalReviewPending marker (NOT typed struct) — survives reviewer-name change without schema migration; DeferralMarker reused verbatim from golden.rs:85-95 for pending_review"
  - "HauThienTrigram Lo Shu encoding pin (Khảm=1..Ly=9, skipping 5/center) — same numbers as existing Palace enum but distinct type (re-aliasing would re-open CRIT-3 from a different angle)"
  - "Mai Hoa casting inputs are LUNAR (not solar) + ((n-1)%k)+1 remainder-zero convention (CRIT-2 prevention: n=8,k=8 resolves to 8 Khôn, NOT 1 Kiền)"
  - "ADR-0006 carries the worked all-eights boundary example in the body itself — Phase 22 contract test cites it; CRIT-2 prevention proof is self-contained"
  - "Cross-link lives in read-only reasoning/direction_composite.rs (NOT interaction/direction_merge.rs) — preserves CRIT-3 isolation; sibling tests/thai_tue_cross_link_crit3.rs guard added in Phase 23"
  - "Composite envelope pattern: distinct primitive source_id envelopes (khcbppt + huyen-khong) PLUS one composite rule.composite.direction_cross_link envelope — only pattern compatible with the CRIT-3 grep guard"

patterns-established:
  - "Decision registration BEFORE implementation: source IDs + ADRs + DEC cross-refs land in Plan 01; Plans 02/03 implement the schemas/code; corpus/algorithm plans (21+) consume them. The Phase 10 (v1.5) + Phase 16 (v1.6) foundation-phase precedent extended cleanly to v1.7."
  - "Two-source pin from day 1 (AF-05): every algorithmic ADR cites ≥2 independent sources (classical + modern practitioner). ADR-0006 mirrors ADR-0004's discipline (classical authority + open Vietnamese modern reference)."
  - "Page-citation deferral with explicit PendingExternalReview marker: mirrors ADR-0004 §5; algorithm unaffected by page-number gap; upgrade lands in a superseding ADR (e.g., ADR-0006a), never as amendment."
  - "Worked boundary example in ADR body: future contract tests cite the ADR derivation directly. A reader does not need to consult the external source to verify the boundary (CRIT-2 self-contained proof)."

requirements-completed: [FND-09, FND-10]

# Metrics
duration: 8 min
completed: 2026-07-16
---

# Phase 20 Plan 01: Foundation — Source IDs + ADRs Summary

**Two v1.7 source IDs (`SOURCE_KINH_DICH` + `SOURCE_MAI_HOA_DICH_SO`) registered with CI-guard extension, plus three accepted Nygard-form ADRs (HexagramEntry schema, Mai Hoa casting convention, cross-link CRIT-3 carve-out) cross-referenced as DEC-0026/0027/0028 — locks the v1.7 foundation before any corpus or algorithm code lands.**

## Performance

- **Duration:** 8 min
- **Started:** 2026-07-15T19:40:44Z
- **Completed:** 2026-07-15T19:49:03Z
- **Tasks:** 2
- **Files modified:** 5 (2 source + 3 ADR created + 1 milestones + 1 deferred-items)

## Accomplishments

- **FND-09 closed.** Two new `pub const` (`SOURCE_KINH_DICH = "kinh-dich"`, `SOURCE_MAI_HOA_DICH_SO = "mai-hoa-dich-so"`) registered in `crates/amlich-core/src/sources.rs` after `SOURCE_HUYEN_KHONG`, following the exact pattern of the existing 7 consts. `tests/source_id_guard.rs::FORBIDDEN_LITERALS` extended to 9 entries — bare `"kinh-dich"` / `"mai-hoa-dich-so"` literals at provenance call-sites now fail CI from day one (CRIT-6 cross-contamination prevention). The guard test passes (verified).
- **FND-10 closed.** Three ADRs authored in Nygard short-form (Title/Status/Context/Decision/Consequences), mirroring the length + tone of ADR-0001 (ritual schema) and ADR-0004 (daily Phi Tinh convention). All three: **Status: Accepted, Date: 2026-07-16.**
- **CRIT-2 prevention proof self-contained in ADR-0006 §4.** The all-eights boundary derivation (`month=8/day=8/hour=8 → Tiên Thiên 8 Khôn, NOT 1 Kiền`) with the explicit `((24-1) % 8) + 1 = 7 + 1 = 8` arithmetic lives in the ADR body itself. Phase 22's contract test cites this exact derivation.
- **CRIT-3 prevention discipline companion.** ADR-0005 §5 pins `HauThienTrigram` to Lo Shu palace numbers (Khảm=1..Ly=9); ADR-0006 §1 pins `TienThienTrigram` to Tiên Thiên numbers (Kiền=1..Khôn=8); ADR-0007 §1 places the cross-link in `reasoning/direction_composite.rs` (NOT `interaction/direction_merge.rs`). Together they pre-empt the trigram-arrangement conflation across the corpus, casting, and cross-link surfaces.
- **MILESTONES.md ADR Cross-References table extended** with DEC-0026 (→ ADR-0005), DEC-0027 (→ ADR-0006), DEC-0028 (→ ADR-0007). No DEC collision (DEC-0025 was the highest formally registered; the v1.6 ADR-0003a/0004 unregistered gap is left as a separate cleanup per 20-RESEARCH.md §Open Questions #3).

## Task Commits

Each task was committed atomically. Task 1 used TDD discipline (RED → GREEN).

1. **Task 1 RED: add failing test for v1.7 source IDs** — `4eff1d4` (test) — extends `all_constants_have_expected_values` with 2 new `assert_eq!` lines referencing not-yet-defined `SOURCE_KINH_DICH` / `SOURCE_MAI_HOA_DICH_SO`. Test fails to compile (RED state confirmed).
2. **Task 1 GREEN: register v1.7 source IDs + extend CI guard** — `cbfbcdb` (feat) — appends the 2 `pub const` definitions to `sources.rs` + 2 entries to `FORBIDDEN_LITERALS` in `source_id_guard.rs`. `source_id_guard` test passes (verified); `all_constants_have_expected_values` test now covers 9 consts.
3. **Task 2: author ADR-0005/0006/0007 + DEC-0026/0027/0028 cross-refs** — `370a486` (docs) — three new ADR files + MILESTONES.md update.

**Plan metadata:** will be added in the final docs commit after STATE.md / ROADMAP.md updates.

## Files Created/Modified

- `crates/amlich-core/src/sources.rs` — added 2 `pub const` after `SOURCE_HUYEN_KHONG`; extended `all_constants_have_expected_values` test with 2 new `assert_eq!` (now 9 consts total).
- `crates/amlich-core/tests/source_id_guard.rs` — appended 2 escaped-quote entries to `FORBIDDEN_LITERALS` (now 9 entries total); guard test passes.
- `.planning/adrs/0005-hexagram-entry-schema-v1.md` — **NEW** — HexagramEntry schema v1 (field set + naming divergence + hao_tu length rule + HauThienTrigram Lo Shu pin + reviewer free-text + DeferralMarker reuse + sample JSON).
- `.planning/adrs/0006-mai-hoa-casting-convention.md` — **NEW** — Mai Hoa casting convention (Tiên Thiên pin + lunar inputs + `((n-1)%k)+1` + worked all-eights boundary example + two-source pin + page-deferral marker + 3 rejected alternatives).
- `.planning/adrs/0007-cross-link-crit3-carve-out.md` — **NEW** — Cross-link CRIT-3 carve-out (placement in `reasoning/direction_composite.rs` + composite envelope pattern + sibling grep guard + read-only `&` discipline + FND-12 ontology reservation cross-reference).
- `.planning/MILESTONES.md` — added 3 rows to ADR Cross-References table (DEC-0026/0027/0028).
- `.planning/phases/20-foundation-schema-lock-source-ids-adrs-ontology/deferred-items.md` — appended item #2 (parallel-execution in-flight state observation).

## Decisions Made

The plan was exceptionally prescriptive (20-CONTEXT.md locks the ADR bodies' substance, the field naming divergence, the reviewer free-text shape, the DeferralMarker reuse, the HauThienTrigram Lo Shu pin, the worked boundary example arithmetic, the cross-link placement, and the composite envelope pattern). Execution followed the plan as written; no fresh design decisions were required beyond prose organisation.

Three low-stakes authoring choices made during ADR authoring (all within planner discretion per 20-CONTEXT.md):

1. **ADR length ~1.5–2× the 20-CONTEXT.md "mirror ADR-0001/0004 length" guidance.** The 20-CONTEXT.md locked ADR-0005 to document ~7 distinct decisions (field set, naming divergence, hao_tu length rule, reviewer free-text, DeferralMarker reuse, HauThienTrigram Lo Shu pin, deny_unknown_fields) and ADR-0006 to carry the full worked boundary example + 3 rejected alternatives + two-source citation + page-deferral note + lunar-input lock + remainder-zero convention lock. Each subsection in the ADRs maps 1-to-1 to a CONTEXT.md lock; the length is the natural consequence of the locked substance, not a stylistic choice. A shorter ADR would have omitted one of the locked decisions.
2. **DEC numbering kept at DEC-0026/0027/0028 (no v1.6 backfill).** 20-RESEARCH.md §Open Questions #3 explicitly recommends "use DEC-0026/0027/0028 for ADR-0005/0006/0007 (do NOT backfill v1.6 — that's a separate cleanup)". Followed as recommended.
3. **Worked boundary example used the 20-RESEARCH.md §"Code Examples > ADR-0006 worked boundary example" arithmetic verbatim** (`(23 % 8) + 1 = 7 + 1 = 8` for upper/lower; `(31 % 6) + 1 = 1 + 1 = 2` for moving line). No re-derivation; the research output is the canonical arithmetic.

## Deviations from Plan

### Auto-fixed Issues

None. The plan was followed exactly as written — both tasks (TDD source IDs + ADR authoring) executed per their `<action>` and `<behavior>` blocks.

### Out-of-Scope Discoveries (logged to deferred-items.md, NOT auto-fixed)

**1. [SCOPE BOUNDARY] Parallel-execution in-flight state on `crates/amlich-core/src/{semantic_graph,reasoning,iching}/`**
- **Found during:** Task 1 GREEN-phase verification
- **Issue:** Config has `parallelization: true`; Plans 20-02 and 20-03 are executing concurrently in the same working tree. Observed real-time modifications to `ontology.rs`, `views/{helpers,visualization}.rs`, `reasoning/types.rs`, plus untracked `iching/` + `data/iching/` directories — all belonging to Plan 20-02 / 20-03 in-flight work. The lib crate was non-compiling during Plan 20-01 verification (e.g., `NodeConcept::Hexagram` enum variant added by Plan 20-03 but match arms in `views/` not yet updated — Pitfall 3 from 20-RESEARCH.md).
- **Why NOT auto-fixed:** Per deviation rules SCOPE BOUNDARY — "Pre-existing warnings, linting errors, or failures in unrelated files are out of scope." The breakage traces entirely to Plan 20-02/20-03 incomplete state, NOT Plan 20-01 changes. Plan 20-01 only touches `sources.rs` + `source_id_guard.rs` (both independent of the broken modules).
- **Verification substitute:** `cargo test -p amlich-core --test source_id_guard` PASSES (1 test, verified — the guard is a standalone test target that does not require the full lib to compile). The `sources::tests::all_constants_have_expected_values` test is a pure `assert_eq!` on locally-defined `pub const` values — its pass/fail depends ONLY on the new constants matching the new asserts, which they do by construction.
- **Action:** Logged to `.planning/phases/20-foundation-schema-lock-source-ids-adrs-ontology/deferred-items.md` item #2. Plan 20-02 / 20-03 GREEN phases will resolve the lib-crate compile state.

---

**Total deviations:** 0 auto-fixed (the parallel-execution observation is a SCOPE BOUNDARY out-of-scope discovery, not a deviation that was auto-fixed).
**Impact on plan:** None. Plan executed exactly as written.

## Issues Encountered

None specific to Plan 20-01's scope. The parallel-execution observation (above) is a workspace-state note, not a Plan 20-01 issue.

## User Setup Required

None — no external service configuration required. This plan is pure decision registration (Rust const + Markdown ADRs + MILESTONES table rows).

## Next Phase Readiness

- **FND-09 + FND-10 closed.** Plan 20-02 (HexagramEntry schema + three newtypes + composition table → FND-11) and Plan 20-03 (ontology 6-slice extension → FND-12) can now consume ADR-0005 (field set + naming + Lo Shu pin) + ADR-0006 (Tiên Thiên pin) + ADR-0007 (placement contract for the `EdgeConcept::LocatedAt` + `EdgeConcept::Transforms` reservation).
- **No blockers.** The parallel-execution in-flight state is expected to resolve as Plans 20-02 / 20-03 complete their GREEN phases.
- **Forward citations:**
  - Phase 21 corpus loader implements ADR-0005 §2 `hao_tu.len()` invariant.
  - Phase 22 `cast_mai_hoa` implements ADR-0006 §1–§4; the contract test cites §4's worked example.
  - Phase 23 `build_direction_cross_link` implements ADR-0007 §1 (placement) + §2 (envelope pattern) + §3 (sibling grep guard).
  - Phase 24 DaySnapshot wiring consumes ADR-0005 HexagramEntry + ADR-0007 cross-link via additive `Option<T>`.

---
*Phase: 20-foundation-schema-lock-source-ids-adrs-ontology*
*Completed: 2026-07-16*

## Self-Check: PASSED

All 7 created/modified files exist on disk; all 3 task commits (4eff1d4, cbfbcdb, 370a486) verified in git log. ADR grep checks (Status, deny_unknown_fields, Thiệu Khang Tiết, direction_cross_link, DEC-0026/0027/0028) all pass.

**Verification suite (run after parallel Plan 20-02/20-03 agents completed their GREEN phase):**
- `cargo test -p amlich-core sources` — sources module tests pass (incl. new `all_constants_have_expected_values` with 9 consts).
- `cargo test -p amlich-core --test source_id_guard` — guard test passes (1 test, 9 FORBIDDEN_LITERALS entries).
- `cargo test -p amlich-core --lib` — 722/722 lib tests pass, 0 failures. Plan 20-01's contributions verified clean against the full crate.
