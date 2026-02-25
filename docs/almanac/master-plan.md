# AmLich Almanac Expansion - Master Plan

## Purpose

Build a reliable, explainable, versioned Vietnamese almanac engine on top of `amlich-core`.

The architecture is split into 3 layers:

1. Deterministic calendar and astronomy core
2. Rule-based almanac engine
3. Person and event evaluation engine

## Guiding Principles

- Keep existing lunar conversion behavior stable unless explicitly versioned.
- Keep deterministic math separate from cultural rule tables.
- Encode cultural rules in data packs (JSON), not hard-coded constants.
- Include evidence metadata in outputs (`source_id`, `method`, `profile`, `ruleset_version`).
- Add features in vertical slices with tests and docs per slice.

## Delivery Phases

### Phase 0 - Scope Freeze and Contracts

Goals:
- Freeze v1 boundaries.
- Freeze terminology and output contracts.

Beads:
- R-0001: Glossary normalization
- R-0002: v1 ruleset scope (`vn_baseline_v1`)
- I-0003: Define `RuleSetDescriptor` schema
- D-0004: Document known differences policy

Exit criteria:
- Team agrees on v1 includes/excludes.
- Ruleset contract is stable.

### Phase 1 - Ruleset Infrastructure

Goals:
- Move from baseline-only loading to multi-ruleset loading.
- Make ruleset and version explicit in compute context.

Beads:
- R-1001: Catalog current almanac dataset fields
- I-1002: Implement ruleset registry and loader
- I-1003: Add schema and token validators
- I-1004: Wire `ruleset_id` into day computation
- T-1005: Determinism tests across rulesets
- D-1006: Loader and fallback documentation

Exit criteria:
- Core computes with explicit `ruleset_id`.
- Output includes ruleset profile/version provenance.

### Phase 2 - Day Hoang Dao and Hac Dao

Goals:
- Add day-level hoang dao/hac dao (not only hour-level).

Beads:
- R-2001: Finalize day deity mapping table for v1
- I-2002: Implement day deity resolver
- I-2003: Add day-level output fields
- I-2004: Integrate evidence metadata
- T-2005: Golden date tests by lunar month and day branch
- D-2006: Formula and variant documentation

Exit criteria:
- `DayFortune` includes day-level classification and explanation.

### Phase 3 - Core Taboo Rules

Goals:
- Ship common VN taboo indicators users expect.

Beads:
- R-3001: Normalize tables for Tam Nuong, Nguyet Ky, Sat Chu, Tho Tu
- I-3002: Add taboo rule categories in ruleset schema
- I-3003: Implement taboo resolver with severity (`hard`/`soft`)
- I-3004: Emit structured `taboos[]` with reason/evidence
- T-3005: Boundary tests around lunar month transitions
- D-3006: Explanation conventions

Exit criteria:
- Day output includes standardized taboo flags.

### Phase 4 - Person Rule Foundation

Goals:
- Add person-aware computation model and core personal rules.

Beads:
- R-4001: Normalize age policy (`tuoi_mu`, Tet boundary)
- I-4002: Add `PersonProfile` schema
- I-4003: Implement tuoi xung, tam tai, kim lau, hoang oc
- T-4004: Birth-near-Tet test cases
- D-4005: Age policy documentation

Exit criteria:
- Personal warnings computed deterministically.

### Phase 5 - Cuu Dieu and Yearly Han

Goals:
- Add yearly star and yearly han outputs by age and gender.

Beads:
- R-5001: Freeze cửu diệu and han mapping tables for v1
- I-5002: Add star and han tables to ruleset schema
- I-5003: Implement yearly resolver
- T-5004: Cross-check known examples
- D-5005: Variant caveat documentation

Exit criteria:
- Person yearly output includes `cuu_dieu` and `han` with evidence metadata.

### Phase 6 - Event Evaluation Engine

Goals:
- Evaluate and rank dates for event types with explainable scoring.

Beads:
- R-6001: Define event taxonomy (`cuoi_hoi`, `dong_tho`, `xuat_hanh`, ...)
- R-6002: Define hard filters and scoring weights
- I-6003: Implement evaluation pipeline (hard -> soft -> score)
- I-6004: Add date-range evaluation API surface
- T-6005: Deterministic ranking tests
- D-6006: Scoring transparency documentation

Exit criteria:
- Engine returns ranked days with reasons and violated rules.

### Phase 7 - Solar Term Accuracy Track (Optional)

Goals:
- Keep current fast mode and add optional high-accuracy occurrences.

Beads:
- R-7001: Choose reference strategy for exact term instants
- I-7002: Add occurrence model (`instant_utc`, `instant_local`, `type`)
- I-7003: Add precompute/cache by year and tz
- T-7004: Compare with published term tables
- D-7005: Fast vs accurate mode documentation

Exit criteria:
- Accurate mode available and validated without regressing fast mode.

### Phase 8 - Locale and Variant Presentation

Goals:
- Decouple symbolic display choices from core branch identity.

Beads:
- R-8001: Define locale symbol policy (e.g. Mao -> Meo/Tho)
- I-8002: Move animal labels to locale layer
- I-8003: Add direction variant tables (VN/CN)
- T-8004: Verify locale-only changes do not alter core results
- D-8005: Locale behavior guarantees

Exit criteria:
- Same computed branch can render locale-specific labels safely.

### Phase 9 - OSS Hardening and Release

Goals:
- Stabilize contracts, fixtures, and contributor workflow.

Beads:
- R-9001: Build canonical golden corpus
- I-9002: Finalize API contract versioning
- T-9003: Full parity suite (core/api/ui)
- D-9004: Contributor guide for adding rulesets
- D-9005: Release notes with known differences

Exit criteria:
- Behavior is reproducible and documented by ruleset version.

## Bead Status Model

- TODO
- RESEARCHING
- READY_FOR_IMPL
- IMPLEMENTING
- VERIFYING
- DONE
- BLOCKED

## Definition of Done (Per Bead)

A bead is done only when:

1. Code/data change is complete.
2. Tests pass, including regression scope.
3. Evidence metadata is present in output.
4. Docs/changelog note are updated.
5. No unversioned contract break is introduced.
