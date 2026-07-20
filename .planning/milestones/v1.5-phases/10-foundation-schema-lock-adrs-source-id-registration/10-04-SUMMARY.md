---
phase: 10-foundation-schema-lock-adrs-source-id-registration
plan: "04"
subsystem: almanac
tags: [fengshui, phi-tinh, flying-stars, adr, lo-shu, huyen-khong, serde, rust]

requires:
  - phase: 10-foundation-schema-lock-adrs-source-id-registration/10-01
    provides: sources.rs with SOURCE_HUYEN_KHONG constant (used in minimal_evidence() in types.rs)

provides:
  - ADR-0002: monthly Phi Tinh solar-term month boundary convention (locked)
  - ADR-0003: Nien Tu Bach direction polarity matrix with Tam Nguyen x year-polarity (locked)
  - FlyingStarLayout frozen type stub (period, palaces: [FlyingStar; 9], center_star, evidence)
  - Palace enum with canonical Lo Shu numbering (N=1..S=9) and palace_to_direction() stub
  - FlyingStar enum (9 variants NhatBach=1..CuuTu=9, no metadata)
  - FlyingStarPeriod discriminator (Van/Yearly/Monthly) with serde support
  - crates/amlich-core/src/almanac/fengshui/ module registered under almanac/mod.rs

affects:
  - Phase 13 (Phi Tinh algorithms implement against these frozen types and ADRs)
  - Phase 14 (star-pair aspects extend this module)
  - Phase 15 (DTO surfacing uses FlyingStarLayout as aggregate)

tech-stack:
  added: []
  patterns:
    - "FlyingStarLayout uses single parameterized struct with FlyingStarPeriod discriminator (not three distinct types)"
    - "Palace/FlyingStar enums use #[repr(u8)] for O(1) Lo Shu number access"
    - "FlyingStarPeriod uses #[serde(tag = 'kind', rename_all = 'snake_case')] for tagged union JSON"
    - "minimal_evidence() helper in types.rs for test/stub envelope construction"

key-files:
  created:
    - .planning/adrs/0002-phi-tinh-monthly-anchor.md
    - .planning/adrs/0003-nien-tu-bach-polarity.md
    - crates/amlich-core/src/almanac/fengshui/mod.rs
    - crates/amlich-core/src/almanac/fengshui/types.rs
  modified:
    - crates/amlich-core/src/almanac/mod.rs

key-decisions:
  - "Single parameterized FlyingStarLayout struct chosen over three distinct types — simpler API surface, single Phase 15 DTO conversion path"
  - "ReasoningEvidenceEnvelope path resolved: crates/amlich-core/src/reasoning/types.rs:145, imported via crate::reasoning::{ReasoningEvidenceEnvelope, ReasoningEvidenceSourceFamily}"
  - "Monthly Phi Tinh uses solar-term month boundaries per Tham Thi Huyen Khong Hoc (ADR-0002 locked)"
  - "Nien Tu Bach direction is (Tam Nguyen yuan, year polarity) -> (starting star, direction) matrix — not a bool flag (ADR-0003 locked)"
  - "Thuong Nguyen and Trung Nguyen rows marked MEDIUM confidence — Phase 13 cross-validates against Tham Thi Huyen Khong Hoc"

patterns-established:
  - "ADR Nygard short-form: Title / Status / Context / Decision / Consequences"
  - "Phi Tinh types deliberately isolated from interaction/ module per PITFALLS CRIT-3"
  - "fengshui/ is a folder (not file) to allow Phase 13 to add period.rs, annual.rs, monthly.rs, combined.rs alongside types.rs"

requirements-completed: [FND-02, FND-04, FND-05]

duration: 15min
completed: 2026-05-26
---

# Phase 10 Plan 04: Phi Tinh ADRs and FlyingStarLayout Type Stubs Summary

**FlyingStarLayout frozen type stubs (Palace, FlyingStar, FlyingStarPeriod, FlyingStarLayout) landed in almanac/fengshui/types.rs with Lo Shu-numbered enums, serde support, and two accepted ADRs locking monthly anchor convention and Nien Tu Bach polarity matrix.**

## Performance

- **Duration:** ~15 min
- **Started:** 2026-05-26T14:35:59Z
- **Completed:** 2026-05-26T14:51:00Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- ADR-0002 written and accepted: monthly Phi Tinh uses solar-term month boundaries per Tham Thi Huyen Khong Hoc; names `get_all_tiet_khi_for_year` at `crates/amlich-core/src/tietkhi.rs:227` as boundary resolver; lists all 12 solar-month opening terms (Lap Xuan 315 deg through Tieu Han 285 deg)
- ADR-0003 written and accepted: Nien Tu Bach direction polarity matrix (Tam Nguyen x year polarity), worked 2024/2025 examples, MEDIUM-confidence acknowledgment for Thuong/Trung Nguyen rows naming Phase 13 as validation phase
- `crates/amlich-core/src/almanac/fengshui/types.rs` created with locked FlyingStarLayout, FlyingStar, Palace, FlyingStarPeriod type stubs; all 5 TDD tests pass
- `crates/amlich-core/src/almanac/fengshui/mod.rs` created declaring `pub mod types`
- `crates/amlich-core/src/almanac/mod.rs` updated with `pub mod fengshui;` (alphabetically positioned)
- PITFALLS CRIT-3 verified: `grep -rn 'fengshui|FlyingStar' crates/amlich-core/src/interaction/` returns zero matches

## Task Commits

Each task was committed atomically:

1. **Task 1: Write ADR 0002 and ADR 0003** - `348f33d` (docs)
2. **Task 2: Land almanac/fengshui/types.rs stubs and register module** - `eca1402` (feat)

Note: types.rs was first created in `be084e2` (plan 10-01 source_id guard fix); plan 10-04 commit `eca1402` adds mod.rs and almanac/mod.rs registration.

## Files Created/Modified

- `.planning/adrs/0002-phi-tinh-monthly-anchor.md` — Accepted ADR: monthly Phi Tinh uses solar-term boundaries, names get_all_tiet_khi_for_year, lists 12 opening terms
- `.planning/adrs/0003-nien-tu-bach-polarity.md` — Accepted ADR: Nien Tu Bach polarity matrix, Tam Nguyen structure, year polarity rule, worked examples, MEDIUM-confidence acknowledgment
- `crates/amlich-core/src/almanac/fengshui/types.rs` — Locked type stubs: Palace (Lo Shu 1-9), FlyingStar (NhatBach=1..CuuTu=9), FlyingStarPeriod (Van/Yearly/Monthly), FlyingStarLayout (frozen field set), palace_to_direction() stub, 5 tests
- `crates/amlich-core/src/almanac/fengshui/mod.rs` — Module entry declaring pub mod types; Phase 13 docstring reserved
- `crates/amlich-core/src/almanac/mod.rs` — Added pub mod fengshui at alphabetical position (line 6, between day_deity and golden_loader)

## Decisions Made

**FlyingStarLayout shape:** Single parameterized struct with FlyingStarPeriod discriminator, not three distinct types. Rationale: simpler API, single Phase 15 DTO conversion path (one FlyingStarsSummary aggregate), matches the plan recommendation.

**ReasoningEvidenceEnvelope path resolved:** Located at `crates/amlich-core/src/reasoning/types.rs:145`. Imported via `crate::reasoning::{ReasoningEvidenceEnvelope, ReasoningEvidenceSourceFamily}` (the `reasoning::types` module is private; types are re-exported from `reasoning/mod.rs`). This is the correct import path for all future Phi Tinh algorithm files in Phase 13.

**evidence field:** ReasoningEvidenceEnvelope already derives Serialize/Deserialize, so FlyingStarLayout can also derive Serialize/Deserialize. No fallback needed.

## Deviations from Plan

**1. [Rule 1 - Bug] Fixed private module import path for ReasoningEvidenceEnvelope**
- **Found during:** Task 2 (compile error: `module types is private`)
- **Issue:** Plan instructed `use crate::reasoning::types::{...}` but `reasoning/types.rs` is a private sub-module; types are re-exported from `crate::reasoning`
- **Fix:** Changed import to `use crate::reasoning::{ReasoningEvidenceEnvelope, ReasoningEvidenceSourceFamily}`
- **Files modified:** `crates/amlich-core/src/almanac/fengshui/types.rs`
- **Verification:** Compilation succeeded; all 5 tests passed
- **Committed in:** eca1402 (Task 2 commit — types.rs was already in be084e2 from plan 10-01)

---

**Total deviations:** 1 auto-fixed (Rule 1 - private module path correction)
**Impact on plan:** Necessary correction. No scope changes.

## Issues Encountered

- `cargo clippy -- -D warnings` showed 23 pre-existing errors in `semantic_graph/views/visualization.rs`, `reasoning/`, and other unrelated files. These are out-of-scope per deviation rules (not caused by current task). Deferred to `deferred-items.md`.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- FND-02, FND-04, FND-05 satisfied — Phase 13 has unambiguous types and ADRs to implement against
- `ReasoningEvidenceEnvelope` import path confirmed as `crate::reasoning::{...}` (not `crate::reasoning::types::{...}`)
- PITFALLS CRIT-3 confirmed clean — Phase 13/15 must maintain this isolation
- Phase 13 cross-validation checklist: Thuong Nguyen and Trung Nguyen rows in ADR-0003 are MEDIUM confidence and require verification against Tham Thi Huyen Khong Hoc

---
*Phase: 10-foundation-schema-lock-adrs-source-id-registration*
*Completed: 2026-05-26*
