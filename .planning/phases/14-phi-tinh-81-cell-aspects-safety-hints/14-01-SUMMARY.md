---
phase: 14-phi-tinh-81-cell-aspects-safety-hints
plan: 01
subsystem: almanac
tags: [fengshui, phi-tinh, flying-stars, aspects, serde, json, onceLock]

# Dependency graph
requires:
  - phase: 13-phi-tinh-primitives-period-annual-monthly
    provides: compute_combined_overlay, CombinedFlyingStarLayout, TietKhiScanner, FlyingStar
  - phase: 10-foundation-schema-lock-adrs-source-id-registration
    provides: SOURCE_HUYEN_KHONG constant, source_id_guard test

provides:
  - StarPairAspect, FsCitation, FsConfidenceTier types with serde support
  - lookup_star_pair_aspect(star_a, star_b) — order-sensitive, panics on missing
  - compute_palace_aspects(year, month, scanner) -> [StarPairAspect; 9]
  - flying_star_aspects.json — 81 ordered pair corpus with citations + confidence tiers
  - aspects module registered in fengshui/mod.rs with full re-exports

affects:
  - 14-02 (safety hints will use lookup_star_pair_aspect)
  - 15-semantic-graph-wiring (DTO integration will consume StarPairAspect)

# Tech tracking
tech-stack:
  added: []
  patterns:
    - OnceLock+include_str! corpus loader with seen-grid validator (81-cell variant)
    - order-sensitive lookup for asymmetric pair tables
    - local FsCitation struct (not importing from rituals pillar — decoupled per PITFALLS Pitfall 4)
    - std::array::from_fn delegation pattern for palace-array construction

key-files:
  created:
    - crates/amlich-core/src/almanac/fengshui/aspects.rs
    - crates/amlich-core/data/almanac/flying_star_aspects.json
  modified:
    - crates/amlich-core/src/almanac/fengshui/mod.rs

key-decisions:
  - "Local FsCitation struct declared in aspects.rs — do NOT import from crate::rituals to keep fengshui/rituals pillars decoupled (PITFALLS Pitfall 4)"
  - "lookup_star_pair_aspect returns owned StarPairAspect (not Option) — corpus validator guarantees all 81 pairs exist; mirrors star_metadata panic discipline"
  - "6 primary classical overrides: (1,6)/(6,1) Kim-Thuy auspicious, (8,9)/(9,8) Tho-Hoa Van 9 auspicious, (2,5)/(5,2) double-earth danger inauspicious — confidence=primary; remainder=synthesized"
  - "Auspice rule: star 2 or 5 in any pair -> inauspicious regardless of element relation (classical override)"
  - "ngu_hanh_relation is order-sensitive: sinh means star_a generates star_b; bi_sinh means star_b generates star_a"
  - "TDD RED commit shipped stub JSON (empty aspects array) so include_str! compiles; GREEN overwrote with full 81-row corpus"

patterns-established:
  - "seen[a][b] grid for duplicate/completeness validation of ordered pair tables"
  - "std::array::from_fn(|i| lookup(overlay.palace_overlays[i])) for palace array construction"

requirements-completed: [FS-11, FS-12, FS-13]

# Metrics
duration: 4min
completed: 2026-05-28
---

# Phase 14 Plan 01: 81-Cell Star-Pair Aspect Corpus Summary

**81-cell Huyền Không star-pair aspects with order-sensitive lookup, citation-bearing types, and palace-composition API bridging the Phase 13 combined overlay**

## Performance

- **Duration:** 4 min
- **Started:** 2026-05-27T18:26:18Z
- **Completed:** 2026-05-27T18:30:28Z
- **Tasks:** 2 (Task 1 TDD RED + Task 2 GREEN)
- **Files modified:** 3

## Accomplishments

- Authored all 81 ordered (star_a × star_b) pairs with Vietnamese classical names, ngũ hành relations, auspice ratings, and citations in `flying_star_aspects.json`
- Implemented `lookup_star_pair_aspect` (order-sensitive, panic discipline matching `star_metadata`) and `compute_palace_aspects` (delegates to `compute_combined_overlay` + array construction via `std::array::from_fn`)
- Registered `pub mod aspects;` in `mod.rs` with full re-exports; source_id_guard CI green (no bare "huyen-khong" literals in .rs)

## Task Commits

Each task was committed atomically:

1. **Task 1: Define aspect types + corpus loader + validator (TDD RED)** - `3fff229` (test)
2. **Task 2: Author 81-cell corpus + lookup + compute_palace_aspects (TDD GREEN)** - `3e426dd` (feat)

## Files Created/Modified

- `crates/amlich-core/src/almanac/fengshui/aspects.rs` - StarPairAspect/FsCitation/FsConfidenceTier types; OnceLock corpus loader + seen-grid validator; lookup_star_pair_aspect; compute_palace_aspects; 8 unit tests
- `crates/amlich-core/data/almanac/flying_star_aspects.json` - 81 ordered-pair aspect corpus (schema-v1, source: Thẩm Thị Huyền Không Học, 6 primary + 75 synthesized entries)
- `crates/amlich-core/src/almanac/fengshui/mod.rs` - Added `pub mod aspects;` and re-exports

## Decisions Made

- **Local FsCitation** declared in `aspects.rs` rather than importing `crate::rituals::schema::SourceCitation` — keeps fengshui/rituals pillars decoupled per PITFALLS Pitfall 4.
- **Panic discipline on lookup** — `lookup_star_pair_aspect` returns owned `StarPairAspect` (not `Option`); validator guarantees all 81 pairs exist at load time; mirrors `star_metadata` panic approach.
- **6 primary classical overrides**: (1,6)/(6,1) = Kim generates Thủy (highly auspicious), (8,9)/(9,8) = Hỏa generates Thổ (auspicious in Vận 9), (2,5)/(5,2) = double-earth danger (inauspicious) — `confidence: "primary"`; remaining 75 rows = `confidence: "synthesized"`.
- **Danger-star override** — any pair containing star 2 (Nhị Hắc) or star 5 (Ngũ Hoàng) is inauspicious regardless of element relation (classical teaching).
- **TDD stub JSON** — created empty corpus stub first so `include_str!` compiles for RED phase; replaced with full 81-row corpus in GREEN commit.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Removed bare "huyen-khong" literal from test block**
- **Found during:** Task 1 (aspects.rs creation)
- **Issue:** Initial `test_star_pair_aspect_deny_unknown_fields` test had a raw JSON string containing the literal `"huyen-khong"` which source_id_guard.rs would flag
- **Fix:** Replaced the bare literal with a `format!()` using `crate::sources::SOURCE_HUYEN_KHONG`; source_id_guard CI confirmed green
- **Files modified:** crates/amlich-core/src/almanac/fengshui/aspects.rs
- **Verification:** `cargo test -p amlich-core --test source_id_guard` passes
- **Committed in:** 3fff229 (Task 1 commit, before Task 2 JSON was written)

---

**Total deviations:** 1 auto-fixed (Rule 2 — missing source_id discipline in test)
**Impact on plan:** Required for source_id_guard compliance; no scope creep.

## Issues Encountered

None — plan executed cleanly after the single deviation fix.

## Next Phase Readiness

- FS-11: `lookup_star_pair_aspect` resolves all 81 ordered pairs — complete.
- FS-12: every `StarPairAspect` carries `source_id "huyen-khong"`, non-empty `original_citation.title`, and a `FsConfidenceTier` — complete.
- FS-13: `compute_palace_aspects(year, month, &scanner)` returns `[StarPairAspect; 9]` derived from `compute_combined_overlay` — complete.
- Phase 14-02 (safety hints) can now import `lookup_star_pair_aspect` and `StarPairAspect` from `crate::almanac::fengshui`.

## Self-Check: PASSED

All created files exist on disk and all task commits are present in git log.

---
*Phase: 14-phi-tinh-81-cell-aspects-safety-hints*
*Completed: 2026-05-28*
