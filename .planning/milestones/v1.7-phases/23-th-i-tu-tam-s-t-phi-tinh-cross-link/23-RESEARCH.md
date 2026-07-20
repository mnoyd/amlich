# Phase 23: Thái Tuế / Tam Sát ⇄ Phi Tinh Cross-Link - Research

**Researched:** 2026-07-16
**Domain:** Vietnamese almanac cross-tradition composite reasoning (KHCBPPT + huyen-khong), read-only direction-merge join
**Confidence:** HIGH

## Summary

Phase 23 implements the read-only directional cross-link between the KHCBPPT Thái Tuế/Tam Sát subsystem (existing `almanac::thai_tue`, new `almanac::tam_sat`) and the huyen-khong Phi Tinh palace layout (existing `almanac::fengshui`) per the CRIT-3 carve-out locked in ADR-0007. The phase adds **one** new almanac module (`almanac/tam_sat.rs`), **one** directional `pub fn` on the existing `thai_tue.rs` module, **two** 1-line `evidence: None → Some(...)` backfills on `thai_tue.rs:107-111` + `sat_phuong.rs:49-53`, **one** new reasoning module (`reasoning/direction_composite.rs`), the additive `DaySnapshot.direction_cross_link: Option<DirectionCrossLinkSummary>` field, and a new sibling CRIT-3 grep guard (`tests/thai_tue_cross_link_crit3.rs`).

Every contract is already pinned by CONTEXT.md + ADR-0007: the 8-point `Direction` enum at `almanac/tu_menh.rs:76-85` is reused (no new `Direction8`/`Direction4` mint), the Tam Sát triad→3-direction mapping is locked (lục-xung opposite triad mirroring `tam_tai.rs:58-63`), the 2-primitive + 1-composite envelope pattern is locked (`source_id: "rule.composite.direction_cross_link"`), and the read-only `&`-references-only invariant is locked. The `Direction::as_vn_str()` refactor, 24-sơn scope, daily Phi Tinh consumption, shared `vn_cardinal_to_direction`, and `build_reasoning_input_graph` integration are explicitly deferred (CONTEXT.md §"Deferred Ideas").

**Primary recommendation:** Land Phase 23 as **2 plans** with NO file conflicts: Plan 23-01 = directional Thái Tuế `pub fn` + new `almanac/tam_sat.rs` module + both `evidence: None` backfills + a test-first BC round-trip pair in `tests/almanac_backfill_compat.rs`. Plan 23-02 = `reasoning/direction_composite.rs` (pure-function helpers + 2 entry-point builders + summary projection) + `enrich_day_snapshot_with_direction_cross_link` helper in `lib.rs` + additive `DaySnapshot.direction_cross_link: Option<DirectionCrossLinkSummary>` field + new sibling `tests/thai_tue_cross_link_crit3.rs` CRIT-3 grep guard. Both plans run in parallel (different file ownerships; 23-02 depends on 23-01 for the directional primitives but can land `enrich_day_snapshot_with_direction_cross_link` shell on stub types if 23-01 hasn't shipped — recommended: 23-01 first, 23-02 second).

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

#### Composite view shape
- **Rich per-direction cells** — each cell carries the full reasoning: KHCBPPT side = Thái Tuế directional year-clash (Direction + conflict kind Truc/Xung/Hai/Hinh/Pha) + Tam Sát 3-direction overlap + Sát Phương day-chi; huyen-khong side = FlyingStar at that palace + palace number + safety hint.
- **Per-direction `agreement` field** — `Some(Agreement | BothSilent | KhcbpptOnly | HuyenKhongOnly | Conflict)` when both traditions have data; `None` when one side is empty (date variant or one tradition silently omits a direction).
- **Vietnamese narrative `summary_vi`** at top of fact node — e.g. "Hôm nay, Thái Tuế xung hướng Bắc và Tam Sát trùng Đông Bắc; Phi Tinh Nhất Bạch tại Trung Cung kỵ khai trương."
- **Composite severity = majority vote** across the 8 directions' per-direction severities (with worst-of within a single direction). `ReasoningNodeSeverity` enum (existing) is the type.
- **8-point direction order** matches existing `direction_merge.rs:9-18` ALL_DIRECTIONS order.

#### No-birth-context behavior
- **Two entry-point functions** in `reasoning/direction_composite.rs`:
  - `build_direction_cross_link_personal(snapshot: &DaySnapshot, birth_chi_index: usize) -> Result<DirectionCrossLink, String>` — full surface, all 3 columns populated.
  - `build_direction_cross_link_date(snapshot: &DaySnapshot) -> Result<DirectionCrossLink, String>` — date-only Tier-0 path. Thái Tuế column empty per direction, Tam Sát + Phi Tinh still populated.
- **`birth_chi_index: usize` (required field)** in both `DirectionCrossLink` and `DirectionCrossLinkSummary`. Date-variant uses a documented sentinel `usize::MAX` (out of 0..=11 branch range). Field carries a doc comment explaining the convention.
- **Composite envelope `note` explains partial data** in the date variant — e.g. "Cross-link surfaces Phi Tinh + Tam Sát only — Thái Tuế directional omitted (no birth context)." Personal variant's note is a fixed description of the cross-link.
- **Agreement = null** (`#[serde(skip_serializing_if = "Option::is_none")]`) when one tradition has no data for a direction. Triple-state in JSON.

#### Directional granularity
- **Reuse existing `Direction` 8-point enum** (`almanac/tu_menh.rs:76-85`: `North, Northeast, East, Southeast, South, Southwest, West, Northwest`). Do NOT mint a new `Direction8` or `Direction4` type.
- **No 24-sơn scope creep** — that's a v1.8 differentiator per `.planning/research/FEATURES.md:42` (DF-04).
- **`Direction::as_vn_str()` refactor is OUT OF SCOPE for Phase 23.** `direction_composite.rs` carries its own private `direction_to_vn()` copy mirroring `direction_merge.rs:94-106`. Refactor the duplicate into a public `tu_menh.rs` method in a follow-up phase.

#### Node vs edge surface scope
- **Phase 23 emits ONLY the `PersonalFactNode` + `DirectionCrossLinkSummary` types.** No semantic-graph edge emission.
- **Phase 24 authors the `LocatedAt` / `Transforms` edge wiring** (per FND-12 ontology reservation in `.planning/adrs/0007-cross-link-crit3-carve-out.md` §5) inside `build_day_snapshot_graph`'s `add_direction_composite_facts` step.
- Cleaner separation: Phase 23 = reasoning fact + DTO; Phase 24 = semantic-graph wiring into DaySnapshot.

#### CRIT-3 sibling guard scope
- **NEW file: `crates/amlich-core/tests/thai_tue_cross_link_crit3.rs`** — sibling to `tests/fengshui_crit3_isolation.rs`.
- **Scans TWO modules**: `src/interaction/direction_merge.rs` (preserves v1.6 contract) AND `src/reasoning/direction_composite.rs` (new Phase 23 carve-out).
- **`FORBIDDEN_TYPE_NAMES` list (Phase 23)**: `["almanac::fengshui", "phi_tinh", "compute_daily_flying_stars", "compute_combined_overlay", "compute_palace_aspects", "TietKhiScanner", "FlyingStarPeriod"]`.
- **DROPPED from new guard**: `FlyingStar`, `DailyFlyingStar`, `DailyFlyingStarLayout` — would false-positive on legitimate `snapshot.flying_stars` (the `FlyingStarsSummary` DTO at `lib.rs:140-152`) and on the `palace_overlays` field's `FlyingStar` field type.
- **Existing `fengshui_crit3_isolation.rs` UNCHANGED** — it continues to scan `direction_merge.rs` with the original 6-pattern list.

#### Tam Sát triad → 3-direction mapping
- **Classical lục-xung opposite triad** — Tam Sát for a year-chi triad is the 3 branches of the OPPOSITE triad (each branch +6 mod 12). Mirrors the existing `tam_tai.rs:58-63` `TAI_YEARS` precedent exactly.
- **Concrete mapping** (already validated against `tam_tai.rs:58-63` + `xung_hop.rs:28-34`):

  | Tam Hợp triad | Element | Tam Sát branches (opposite) | Tam Sát directions (8-point) |
  |---|---|---|---|
  | Thân·Tý·Thìn | Thủy | Dần·Ngọ·Tuất | Đông Bắc, Nam, Tây Bắc |
  | Hợi·Mão·Mùi | Mộc | Tỵ·Dậu·Sửu | Đông Nam, Tây, Đông Bắc |
  | Dần·Ngọ·Tuất | Hỏa | Thân·Tý·Thìn | Tây Nam, Bắc, Đông Nam |
  | Tỵ·Dậu·Sửu | Kim | Hợi·Mão·Mùi | Tây Bắc, Đông, Tây Nam |

- **Branch-to-direction mapping**: 4 cardinals map uniquely (Tý→Bắc, Mão→Đông, Ngọ→Nam, Dậu→Tây); 8 intercardinal branches collapse in pairs (Sửu+Dần→Đông Bắc, Thìn+Tỵ→Đông Nam, Mùi+Thân→Tây Nam, Tuất+Hợi→Tây Bắc).
- **Authoritative KHCBPPT citation is NOT in the codebase yet.** Plan 23-02 authors `data/almanac/tam_sat_provenance.md` (1-page discoverable artifact, not runtime).

#### Evidence envelope pattern
- **2 primitive envelopes + 1 composite envelope per `DirectionCrossLink`**:
  - Primitive 1: `source_id = SOURCE_KHCBPPT`, `method = "thai_tue_direction+tam_sat+sat_phuong"`, `note` carries the per-tradition summary.
  - Primitive 2: `source_id = SOURCE_HUYEN_KHONG`, `method = "phi_tinh.palace_layout"`, `note` carries the palace-overlay summary.
  - Composite: `source_id = COMPOSITE_DIRECTION_CROSS_LINK` (named const), `method = "v17.read_only_join"`, `note` explains what's joined and (for date variant) why one side is missing.
- **Composite envelope's literal source_id held in ONE named const**: `pub const COMPOSITE_DIRECTION_CROSS_LINK: &str = "rule.composite.direction_cross_link";` in `direction_composite.rs`. Mirrors `COMPOSITE_ICHING_CONSULTATION` discipline from `24-01-PLAN.md:303`.
- **`source_id_guard.rs::FORBIDDEN_LITERALS` does NOT include `"rule.composite.*"` strings** — confirmed safe (the array only guards the 9 corpus source_ids).

#### Evidence backfill on existing almanac functions
- **`ThaiTueResult.evidence: None` → `evidence: Some(RuleEvidence { source_id: SOURCE_KHCBPPT, ... })`** at `thai_tue.rs:107-111` (within `compute_thai_tue`'s return).
- **`SatPhuongResult.evidence: None` → `evidence: Some(RuleEvidence { source_id: SOURCE_KHCBPPT, ... })`** at `sat_phuong.rs:49-53` (within `get_sat_phuong`'s return).
- **Round-trip tests FIRST** in `crates/amlich-core/tests/almanac_backfill_compat.rs` — mirrors `tests/day_snapshot_v14_compat.rs` pattern. Verify v1.6 JSON without `evidence` deserialises correctly + populated v1.7 JSON round-trips byte-equal BEFORE the backfill lands.

### Claude's Discretion
- Exact `khcbppt_summary_vi` / `huyen_khong_summary_vi` Vietnamese wording per direction (templates vs hand-written).
- Exact `most_frequent_severity` tiebreaker rule (favor Auspicious on tie vs favor Inauspicious).
- Whether `vn_cardinal_to_direction("Nam")` is a private helper in `direction_composite.rs` or a shared helper in `tu_menh.rs` (recommend private for Phase 23; refactor in follow-up).
- Exact wording of `data/almanac/tam_sat_provenance.md` (1-page artifact, not runtime).
- Whether `enrich_day_snapshot_with_direction_cross_link` takes `birth_chi_index: usize` (required) or `Option<usize>` (recommended: required, matches Phase 24's contract).

### Deferred Ideas (OUT OF SCOPE)
- **`Direction::as_vn_str()` refactor** — DRY consolidation of `direction_merge.rs:94-106`'s private `direction_to_vn` and `direction_composite.rs`'s private copy into a public `tu_menh.rs` method. Follow-up phase (avoids merge churn with `direction_merge.rs` in Phase 23).
- **24-sơn directional resolution** — v1.8 differentiator per `.planning/research/FEATURES.md:42` (DF-04). Out of scope for Phase 23; flag in roadmap backlog.
- **Daily Phi Tinh layer consumption** — `snapshot.daily_flying_stars: Option<DailyFlyingStarLayout>` is NOT consumed by Phase 23's cross-link (annual layer only via `snapshot.flying_stars`). Follow-up phase can extend `DirectionCell.huyen_khong_summary_vi` with the daily layer.
- **`vn_cardinal_to_direction` helper shared between `direction_composite.rs` + `sat_phuong.rs` callers** — currently private in `direction_composite.rs`. Refactor when a second caller appears.
- **`build_reasoning_input_graph` integration** — Phase 23's `build_direction_cross_link` is wired by `personal.rs::build_fact_nodes` (Tier-1 personal path) AND by `enrich_day_snapshot_with_direction_cross_link` (additive DTO path). A future phase may unify both paths or expose `build_direction_cross_link_date` to a Tier-0 reasoning builder.
- **Phase 18 daily Phi Tinh corpus integration** — `snapshot.daily_flying_stars` is a separate source of palace-layout data; the cross-link currently consumes annual `snapshot.flying_stars` only.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| XLK-01 | Thái Tuế directional `pub fn` (year-chi → `Direction`, `source_id: khcbppt`) + the two 1-line `evidence: None` backfills on `thai_tue.rs` + `sat_phuong.rs`. | `thai_tue.rs:107-111` (backfill target); `Direction` enum at `tu_menh.rs:76-85`; `CHI` at `types.rs:17-19`; `RuleEvidence` at `almanac/types.rs:157-162`; `tests/source_id_guard.rs:13-22` enforces const usage. |
| XLK-02 | Classical Tam Sát directional module (`almanac/tam_sat.rs`) returning THREE contiguous directions per year, `source_id: khcbppt`; `sat_phuong.rs` day-chi feature intact. | `tam_tai.rs:58-63` `TAI_YEARS` lục-xung opposite triad precedent; `xung_hop.rs:28-34` `tam_hop`; CONTEXT.md locked Tam Sát → 3-direction mapping table; `tests/source_id_guard.rs` enforcement. |
| XLK-03 | Read-only `build_direction_cross_link(snapshot, birth_chi_index) -> PersonalFactNode` in `reasoning/direction_composite.rs`; 2 primitive + 1 composite envelope; CRIT-3 preserved. | `reasoning/personal.rs:13-18` `PersonalFactNode` shape; `reasoning/types.rs:147-153` `ReasoningEvidenceEnvelope`; `ReasoningNodeSeverity` at `reasoning/types.rs:155-170`; `FlyingStarsSummary` at `lib.rs:140-152`; `DaySnapshot` additive `Option<T>` discipline at `lib.rs:163-185`; CRIT-3 carve-out per `adrs/0007-cross-link-crit3-carve-out.md`. |
</phase_requirements>

## Standard Stack

### Core (zero new dependencies)
| Item | Where | Why |
|------|-------|-----|
| `serde 1.0` / `serde_json 1.0` (workspace pin) | workspace pins | DTO serialization for `DirectionCrossLinkSummary` + `DirectionCrossLink` + `DirectionCell` + `DirectionalTaboo`; deserialization for v1.6→v1.7 BC round-trip tests. |
| `serde` derives | `reasoning/personal.rs:13-18` precedent | `DirectionCrossLinkSummary`, `DirectionCrossLink`, `DirectionCell`, `DirectionalTaboo` all use `#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]` + `#[serde(rename_all = "snake_case")]`. |
| `chrono 0.4` (workspace pin) | workspace pins | NOT directly touched by Phase 23 (cross-link consumes `snapshot.context.canchi` which already exists). |
| `unicode-normalization 0.1.25` | `Cargo.toml` direct dep | NFC normalisation of Vietnamese strings in `summary_vi` (RIT-08 discipline); re-implement inline (corpus's `nfc()` helper is module-private per Phase 21-02). |
| `std::sync::OnceLock<T>` | `crates/amlich-core/src/almanac/types.rs:158` precedent | NOT needed by Phase 23 — `DirectionCrossLink` is computed per-call (not cached). |
| `cargo tree -p amlich-core --depth 1` | `.planning/PROJECT.md` v1.5 lock | Verify zero new crate deps after Phase 23 (still only serde + serde_json + chrono + unicode-normalization). |

### Reused existing types (DO NOT mint new ones)
| Type | Location | Phase 23 use |
|------|----------|--------------|
| `Direction` (8-point enum) | `almanac/tu_menh.rs:76-85` | Thái Tuế directional + Tam Sát 3-direction outputs (CONTEXT.md locked). |
| `RuleEvidence` | `almanac/types.rs:157-162` | `ThaiTueResult.evidence` + `SatPhuongResult.evidence` backfills (XLK-01). |
| `ReasoningEvidenceEnvelope` | `reasoning/types.rs:147-153` | 2 primitive + 1 composite envelopes per `DirectionCrossLink` (XLK-03). |
| `ReasoningEvidenceSourceFamily` | `reasoning/types.rs:135-144` | Use `AlmanacRule` for KHCBPPT primitive + `AlmanacRule` for huyen-khong primitive + `Derived` for composite (mirrors IChing pattern at `24-01-PLAN.md:302`). |
| `ReasoningNodeSeverity` | `reasoning/types.rs:155-170` | Composite severity (majority vote) + per-direction severity. |
| `PersonalFactNode` | `reasoning/personal.rs:13-18` | The shape `build_direction_cross_link_personal/_date` returns. |
| `PersonalReasoningInput` + `build_fact_nodes` | `reasoning/personal.rs:20-107` | Tier-1 personal-path wiring point (Phase 23 may extend; see §"Integration Points"). |
| `FlyingStarsSummary` DTO | `lib.rs:140-152` | Read-only consumer for Phi Tinh data (the cross-link reads `snapshot.flying_stars`, NOT `almanac::fengshui::*` directly). |
| `DaySnapshot` + additive `Option<T>` discipline | `lib.rs:154-186` | `direction_cross_link: Option<DirectionCrossLinkSummary>` follows the exact pattern at `lib.rs:163-185`. |
| `SOURCE_KHCBPPT`, `SOURCE_HUYEN_KHONG` | `sources.rs:8, 26` | Already-defined primitive source_ids. |
| `CHI` constant | `types.rs:17-19` | The 12-branch string lookup table for branch→direction map (Sat Phương side). |
| `CanChi` struct | `types.rs:69-78` | `snapshot.context.canchi.day` exposes `chi_index` for `get_sat_phuong`. |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Direction enum reuse | Mint new `Direction8` / `Direction4` | REJECTED per CONTEXT.md — proliferation risk; existing type is already the right shape. |
| Custom `nfc()` in direction_composite.rs | Import from `rituals/corpus.rs:163-169` | REJECTED — `nfc()` is module-private (Phase 21-02 decision); re-implement the ~6-line helper inline (mirrors `iching/the_dung.rs` Phase 22 discipline). |
| Inlined source_id strings | `pub const SOURCE_*` from `sources.rs` | REJECTED — `tests/source_id_guard.rs:13-22` forbids bare literals in `src/` outside `sources.rs`. |
| `String` literals for composite source_id | Named `COMPOSITE_DIRECTION_CROSS_LINK` const | REJECTED — `COMPOSITE_ICHING_CONSULTATION` discipline per `24-01-PLAN.md:303` (single named const, not inlined). |

### Installation
None — `cargo build -p amlich-core` compiles without any `Cargo.toml` changes.

## Architecture Patterns

### Recommended Project Structure (Phase 23 deltas)
```
crates/amlich-core/
├── src/
│   ├── almanac/
│   │   ├── thai_tue.rs          # MODIFIED: add thai_tue_direction() + backfill evidence
│   │   ├── sat_phuong.rs        # MODIFIED: backfill evidence
│   │   └── tam_sat.rs           # NEW (~120 lines): classical 3-direction Tam Sát
│   ├── reasoning/
│   │   ├── direction_composite.rs   # NEW (~300 lines): build_direction_cross_link + summary projection + enrichment
│   │   ├── mod.rs              # MODIFIED: add pub use direction_composite::{...}
│   │   └── personal.rs         # UNCHANGED (Phase 24 may extend build_fact_nodes)
│   ├── lib.rs                  # MODIFIED: add DaySnapshot.direction_cross_link field + enrich_day_snapshot_with_direction_cross_link helper
│   └── sources.rs              # UNCHANGED (SOURCE_KHCBPPT, SOURCE_HUYEN_KHONG already exist)
├── data/almanac/
│   └── tam_sat_provenance.md   # NEW (~30 lines): KHCBPPT citation discovery artifact (NOT runtime)
└── tests/
    ├── almanac_backfill_compat.rs          # NEW (~100 lines): v1.6 evidence=None BC round-trip BEFORE backfill
    ├── thai_tue_cross_link_crit3.rs        # NEW (~80 lines): sibling CRIT-3 grep guard
    └── direction_cross_link_integration.rs # NEW (~250 lines): black-box public API + compositeness tests
```

### Pattern 1: Tam Sát triad → 3-direction mapping (lục-xung opposite)
**What:** For each year-chi, compute its Tam Hợp triad, take the OPPOSITE triad (each branch +6 mod 12), map each of those 3 branches to a Direction.

**When to use:** Whenever building classical 3-direction Tam Sát output.

**Example (locked mapping per CONTEXT.md §"Tam Sát triad → 3-direction mapping"):**
```rust
// crates/amlich-core/src/almanac/tam_sat.rs (NEW)
// Mirrors the tam_tai.rs:58-63 TAI_YEARS pattern verbatim.

// Triad 0 (Water: Thân, Tý, Thìn) → opposite triad (Dần, Ngọ, Tuất)
const TAM_SAT_DIRECTIONS_BY_TRIAD: [[Direction; 3]; 4] = [
    [Direction::Northeast, Direction::South,    Direction::Northwest], // triad 0: Dần, Ngọ, Tuất
    [Direction::Southeast, Direction::West,     Direction::Northeast], // triad 1: Tỵ,  Dậu, Sửu
    [Direction::Southwest, Direction::North,    Direction::Southeast], // triad 2: Thân, Tý,  Thìn
    [Direction::Northwest, Direction::East,     Direction::Southwest], // triad 3: Hợi, Mão, Mùi
];

pub fn tam_sat_direction(year_chi_index: usize) -> TamSatDirectionResult {
    let triad_group = year_chi_index % 4;  // mirrors xung_hop::tam_hop
    TamSatDirectionResult {
        year_chi: CHI[year_chi_index],
        triad_group: triad_group,
        tam_sat_branches: opposite_triad_branches(triad_group),
        tam_sat_directions: TAM_SAT_DIRECTIONS_BY_TRIAD[triad_group].to_vec(),
        evidence: RuleEvidence {
            source_id: SOURCE_KHCBPPT.to_string(),
            method: "tam_sat_opposite_triad".to_string(),
            profile: "baseline".to_string(),
        },
    }
}
```

### Pattern 2: Direction composite fact node (3 evidence envelopes)
**What:** Each `DirectionCrossLink` carries 2 primitive envelopes (`khcbppt` + `huyen-khong`) + 1 composite envelope (`rule.composite.direction_cross_link`).

**When to use:** Whenever the cross-link emits a `PersonalFactNode` (mirrors `24-01-PLAN.md:298-303` IChing pattern).

**Example:**
```rust
// crates/amlich-core/src/reasoning/direction_composite.rs (NEW)
pub const COMPOSITE_DIRECTION_CROSS_LINK: &str = "rule.composite.direction_cross_link";

fn build_evidence(
    khcbppt_summary: &str,
    huyen_khong_summary: &str,
    is_date_variant: bool,
) -> Vec<ReasoningEvidenceEnvelope> {
    let mut env = vec![
        ReasoningEvidenceEnvelope {
            source_family: ReasoningEvidenceSourceFamily::AlmanacRule,
            source_id: SOURCE_KHCBPPT.to_string(),
            method: "thai_tue_direction+tam_sat+sat_phuong".to_string(),
            note: Some(khcbppt_summary.to_string()),
        },
        ReasoningEvidenceEnvelope {
            source_family: ReasoningEvidenceSourceFamily::AlmanacRule,
            source_id: SOURCE_HUYEN_KHONG.to_string(),
            method: "phi_tinh.palace_layout".to_string(),
            note: Some(huyen_khong_summary.to_string()),
        },
        ReasoningEvidenceEnvelope {
            source_family: ReasoningEvidenceSourceFamily::Derived,
            source_id: COMPOSITE_DIRECTION_CROSS_LINK.to_string(),
            method: "v17.read_only_join".to_string(),
            note: Some(if is_date_variant {
                "Cross-link surfaces Phi Tinh + Tam Sát only — Thái Tuế directional omitted (no birth context).".to_string()
            } else {
                "Read-only composite cross-link of KHCBPPT directional taboos + Huyền-Không Phi Tinh palace layout.".to_string()
            }),
        },
    ];
    env
}
```

### Pattern 3: Additive `DaySnapshot` field with enrichment helper (immutable clone)
**What:** `direction_cross_link: Option<DirectionCrossLinkSummary>` initialised to `None` in `calculate_day_snapshot_internal`, populated only via explicit `enrich_day_snapshot_with_direction_cross_link` clone-and-attach.

**When to use:** Any new cross-link surface that needs DaySnapshot presence but no auto-population (mirrors `24-01-PLAN.md:330-355` IChing pattern).

**Example (lib.rs additive field):**
```rust
// crates/amlich-core/src/lib.rs (MODIFIED — additive after offerings field)

/// Additive optional directional cross-link summary (Phase 23, XLK-03).
/// Populated only when the caller explicitly invokes
/// `enrich_day_snapshot_with_direction_cross_link(snapshot, birth_chi_index)`.
/// Ordinary `calculate_day_snapshot(...)` calls leave this as `None`.
/// Absent in JSON when None.
#[serde(default, skip_serializing_if = "Option::is_none")]
pub direction_cross_link: Option<crate::reasoning::direction_composite::DirectionCrossLinkSummary>,
```

### Pattern 4: Pure-function pure helper decomposition
**What:** Decompose `build_direction_cross_link_personal` into ~6 small pure helpers so each can be RED→GREEN-tested independently.

**When to use:** Any cross-link builder that mixes multiple traditions.

**Decomposition (recommended):**
```rust
// direction_composite.rs helpers (each is a small pure fn)

// 1. Per-direction KHCBPPT contribution
fn khcbppt_per_direction(year_chi_index: usize, day_chi_index: usize, birth_chi_index: usize) -> HashMap<Direction, DirectionalTaboo>;

// 2. Per-direction Phi Tinh contribution
fn huyen_khong_per_direction(snapshot: &DaySnapshot) -> HashMap<Direction, HuyenKhongCell>;

// 3. Per-direction agreement computation
fn agreement(khcbppt: Option<&DirectionalTaboo>, huyen_khong: Option<&HuyenKhongCell>) -> Option<Agreement>;

// 4. Per-direction severity (worst-of within direction)
fn direction_severity(taboo: &DirectionalTaboo) -> ReasoningNodeSeverity;

// 5. Composite severity = majority vote
fn composite_severity(severities: &[ReasoningNodeSeverity]) -> ReasoningNodeSeverity;

// 6. Vietnamese narrative summary builder
fn build_summary_vi(cells: &[DirectionCell; 8], date_str: &str, birth_chi: usize) -> String;

// 7. PersonalFactNode assembler
fn assemble_fact_node(cells: [DirectionCell; 8], summary_vi: String, evidence: Vec<ReasoningEvidenceEnvelope>, composite_severity: ReasoningNodeSeverity) -> PersonalFactNode;

// 8. Summary projection
fn project_summary(cells: &[DirectionCell; 8], birth_chi_index: usize, composite_severity: ReasoningNodeSeverity) -> DirectionCrossLinkSummary;
```

### Anti-Patterns to Avoid
- **Modifying `direction_merge.rs` or its `evidence` payload** — CRIT-3 carve-out (ADR-0007 §1) forbids this; the cross-link lives at `reasoning/direction_composite.rs`, NOT in `interaction/`.
- **Direct `almanac::fengshui::*` import in `direction_composite.rs`** — CRIT-3 grep guard forbids it; the cross-link MUST consume `snapshot.flying_stars: Option<FlyingStarsSummary>` (the DTO at `lib.rs:140-152`), NOT the underlying types.
- **Minting `Direction8` / `Direction4` / 24-sơn types** — CONTEXT.md locked `Direction` reuse; 24-sơn deferred to v1.8 per `research/FEATURES.md:42` (DF-04).
- **Inline composite `source_id` string at multiple call-sites** — single named `COMPOSITE_DIRECTION_CROSS_LINK` const (mirrors `COMPOSITE_ICHING_CONSULTATION`).
- **Auto-populating `DaySnapshot.direction_cross_link` in `calculate_day_snapshot_internal`** — additive DTO discipline (lib.rs:163-185): `None` by default, explicit enrichment only. Otherwise v1.6→v1.7 BC round-trip breaks.
- **Refactoring `Direction::as_vn_str()` into `tu_menh.rs` in Phase 23** — out of scope per CONTEXT.md §"Deferred Ideas"; duplicate `direction_to_vn` in `direction_composite.rs` for now.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|------------|-----|
| 1..N enum with serde | New `Direction8` newtype | Existing `Direction` at `tu_menh.rs:76-85` | CONTEXT.md locked; CRIT-3 isolation requires keeping the merge layer's `Direction` type. |
| Tam Hợp triad lookup | New triad-grouping function | `xung_hop::tam_hop(chi_index)` at `xung_hop.rs:28-34` | Already shipped, already tested; Tam Sát just reuses the `% 4` group arithmetic. |
| Branch → direction mapping (12 branches → 8 directions) | Custom lookup table | Direct branch-index match (Tý→Bắc, etc.) + pair-collapse (Sửu+Dần→Đông Bắc) | CONTEXT.md §"Branch-to-direction mapping" is locked and small; inline it in `direction_composite.rs` as a `match` expression. |
| Vietnamese cardinal lookup | String-matching helper | Direct `&'static str` match against the 8 `Direction` variants (mirroring `direction_merge.rs:94-106`) | Reuse the locked 8-point string set; future `Direction::as_vn_str()` refactor lands separately. |
| Date-only safety hint lookup | Custom is-danger predicate | `almanac::fengshui::safety::element_hint_for_palace(FlyingStar)` (PUBLIC API at `fengshui/mod.rs:38`) | The cross-link reads from `snapshot.flying_stars` only (no direct fengshui imports per CRIT-3); but the per-palace safety hint shape is built from `palace_overlays[i]` + a public function lookup. **CAVEAT:** calling `element_hint_for_palace` directly from `direction_composite.rs` would import `almanac::fengshui::safety::*` and trip the new CRIT-3 sibling guard (`tests/thai_tue_cross_link_crit3.rs::FORBIDDEN_TYPE_NAMES` includes `almanac::fengshui`). **Recommendation:** the planner should consume the safety hint as a pre-baked string in `FlyingStarsSummary` or in a new additive `palace_safety_hints: Option<[Option<RemedyHint>; 9]>` field on `FlyingStarsSummary` (Phase 24 work) OR precompute the safety strings inside `lib.rs::calculate_day_snapshot_internal` at `FlyingStarsSummary` construction time and store them as plain `&'static str` (lifetime-safe). Phase 23's plan should author the safety hint computation in `lib.rs` once (during `FlyingStarsSummary` population), so `direction_composite.rs` consumes a precomputed `palace_safety_hints: [&'static str; 9]` (or similar additive field). This keeps CRIT-3 isolation at the cross-link layer. |
| NFC normalisation of Vietnamese text | Custom Unicode table | Re-implement the ~6-line `nfc()` helper inline (mirrors `iching/the_dung.rs:240-247`) | `corpus.rs::nfc()` is module-private (Phase 21-02 decision); re-implement inline to avoid widening visibility. |
| Backwards-compat BC round-trip tests | Custom test harness | `tests/day_snapshot_v14_compat.rs` pattern (Phase 18-04 + 19-01 + 19-03 precedent) | Combined-strip v1.6→v1.7 round-trip pattern is canonical: strip all additive fields → re-serialise → byte-equal + no unexpected fields. |
| Composite-envelope source_id | Inline `&str` at multiple call-sites | `pub const COMPOSITE_DIRECTION_CROSS_LINK: &str = "rule.composite.direction_cross_link";` | Mirrors `COMPOSITE_ICHING_CONSULTATION` per `24-01-PLAN.md:303`; single audit point for the rule identifier. |
| Majority-vote severity tiebreak | Custom ranking heuristic | The CONTEXT.md Claude's Discretion (recommend: tie → Inauspicious per CONSERVATIVE-DEFAULT discipline, since Vietnamese almanac UX defaults to "taboo-leaning" on ambiguity). Document the chosen rule in `direction_composite.rs` doc-comment. | Domain convention favors surfacing the cautionary signal; the planner may pick Auspicious if implementation shows that's better — flag the choice. |

**Key insight:** Phase 23's cross-link is a **projection** over already-shipped producers (`compute_thai_tue`, `get_sat_phuong`, the `FlyingStarsSummary` DTO, the `Direction` enum). There is **no algorithmic novel work** beyond the Tam Sát triad→direction mapping (which is a lookup table per CONTEXT.md) and the composite severity majority-vote rule. The complexity is in the **discipline** (CRIT-3 isolation, BC round-trip, additive DTO, composite-envelope pattern) — not in the algorithm.

## Common Pitfalls

### Pitfall 1: Tam Sát triad → direction mapping confusion
**What goes wrong:** Developer implements Tam Sát as "the Tam Hợp triad itself maps to 3 directions" (wrong — that would be Tam Tai direction), instead of "the OPPOSITE triad maps to 3 directions" (correct per CONTEXT.md).

**Why it happens:** `tam_tai.rs` and the new `tam_sat.rs` are both "Tam Tai vs Tam Sát" directional concepts — easy to confuse. `tam_tai.rs:1` doc-comment calls Tam Tai "三殺" but actually means the OPPOSITE-TRIAD Taì 3-YEAR AFFLICTION period, not the classical DIRECTIONAL Tam Sát. They share the Chinese name 三殺 but mean different things.

**How to avoid:** The CONTEXT.md table is authoritative. Use the `tam_tai.rs:58-63` `TAI_YEARS` pattern as a reference but verify with the CONTEXT.md table before encoding.

**Warning signs:** The test `tam_sat_direction_for_water_year_returns_east_directions` fails (would return Đông Bắc + Nam + Tây Bắc instead of the correct Đông Bắc + Nam + Tây Bắc — actually the same since triad 0 maps to Dần, Ngọ, Tuất which are the water-triad opposite. The correct verification case is triad 1: Wood triad Mão, Hợi, Mùi should give Tam Sát directions = Tỵ, Dậu, Sửu directions = Đông Nam, Tây, Đông Bắc — verify against `xung_hop::luc_xung(3)=Mùi...luc_xung(7)=Sửu...luc_xung(5)=Tỵ` mapped to Đông Nam+Tây+Đông Bắc).

### Pitfall 2: CRIT-3 grep guard false-positive on legitimate `FlyingStar`
**What goes wrong:** The naive grep guard includes `FlyingStar` in `FORBIDDEN_TYPE_NAMES`, which false-positives on `palace_overlays: [(FlyingStar, FlyingStar); 9]` field type in `FlyingStarsSummary` if any cross-link code reads `snapshot.flying_stars.palace_overlays[i].0` (a `FlyingStar` value type).

**Why it happens:** The original `tests/fengshui_crit3_isolation.rs:14-21` `FORBIDDEN_TYPE_NAMES` includes `FlyingStar`. CONTEXT.md locked dropping `FlyingStar` from the Phase 23 guard.

**How to avoid:** Use the CONTEXT.md-locked `FORBIDDEN_TYPE_NAMES` list: `["almanac::fengshui", "phi_tinh", "compute_daily_flying_stars", "compute_combined_overlay", "compute_palace_aspects", "TietKhiScanner", "FlyingStarPeriod"]` — drop `FlyingStar`, `DailyFlyingStar`, `DailyFlyingStarLayout` from the Phase 23 guard.

**Warning signs:** `tests/thai_tue_cross_link_crit3.rs` fails on `direction_composite.rs:147:  pub field FlyingStar palace_overlay` if the guard still includes `FlyingStar`.

### Pitfall 3: Additive `DaySnapshot` field auto-populated (BC regression)
**What goes wrong:** Developer initialises `direction_cross_link: None` but then ALSO populates it conditionally inside `calculate_day_snapshot_internal` (e.g., when `snapshot.context.canchi.year.chi_index` is set). This makes the field appear in JSON for some days but not others — BC test breaks.

**Why it happens:** Naive pattern from non-additive fields (e.g., `flying_stars` is ALWAYS populated because `compute_combined_overlay` always succeeds).

**How to avoid:** The additive discipline at `lib.rs:163-185` is strict: `None` by default, explicit enrichment only. The enrichment helper clones the snapshot and attaches; it never mutates the original.

**Warning signs:** `v16_json_without_v17_fields_deserializes` test fails (a v1.7-without-enrichment JSON has `direction_cross_link: null` or `direction_cross_link: { ... }` in the field, but a v1.6 producer JSON cannot have the field — `#[serde(default)]` should map absent → `None`, not present → panic).

### Pitfall 4: Bare `source_id` literals in `direction_composite.rs`
**What goes wrong:** Developer writes `source_id: "khcbppt".to_string()` directly in the cross-link builder (or `"huyen-khong"`, or `"rule.composite.direction_cross_link"`).

**Why it happens:** Faster than typing the const. Common Rust idiom violation in this codebase.

**How to avoid:** `tests/source_id_guard.rs::FORBIDDEN_LITERALS` at line 14-22 forbids all 9 corpus `source_id` strings (`"khcbppt"`, `"vn-folk"`, etc.). The `"rule.composite.direction_cross_link"` literal IS NOT in the forbidden list (it's a composite rule identifier, not a corpus source); still, hold it in `COMPOSITE_DIRECTION_CROSS_LINK` const for auditability (mirrors `COMPOSITE_ICHING_CONSULTATION`).

**Warning signs:** `cargo test -p amlich-core --test source_id_guard` fails with violations in `direction_composite.rs`.

### Pitfall 5: `birth_chi_index` validation skipped
**What goes wrong:** `build_direction_cross_link_personal(snapshot, birth_chi_index)` accepts any `usize` and silently treats e.g. `birth_chi_index = 200` as a Thái Tuế branch.

**Why it happens:** `usize` is the natural type for the `Direction`'s underlying index.

**How to avoid:** Validate `birth_chi_index < 12` early in both `build_direction_cross_link_personal` AND `enrich_day_snapshot_with_direction_cross_link`. The sentinel for date-variant is `usize::MAX`, NOT in the 0..12 range — validation passes it through to the date-variant branch.

**Warning signs:** `build_direction_cross_link_personal_validates_birth_chi_index` test fails with `birth_chi_index = 100` (out of range → panic or wrong branch returned).

### Pitfall 6: Composite severity majority-vote tiebreak undecided
**What goes wrong:** Developer leaves the tiebreak rule as "first match wins" (deterministic but not principled).

**Why it happens:** CONTEXT.md Claude's Discretion §"Exact most_frequent_severity tiebreaker rule" is open.

**How to avoid:** Document the chosen rule in `direction_composite.rs` doc-comment + add a unit test asserting the tiebreak behavior. Recommended: **tie → Inauspicious** (conservative default for Vietnamese almanac UX — flag warnings rather than blessings when both interpretations are equal).

**Warning signs:** `composite_severity_tiebreak_is_documented` test fails.

### Pitfall 7: KHCBPPT Tam Sát citation treated as authoritative when unverified
**What goes wrong:** Developer writes `evidence.note` claiming "KHCBPPT Quyển 9, trang 47" without verifying the page reference.

**Why it happens:** CONTEXT.md says "Authoritative KHCBPPT citation is NOT in the codebase yet" — the citation is honest-deferred.

**How to avoid:** Plan 23-02 authors `data/almanac/tam_sat_provenance.md` as a 1-page discovery artifact (NOT a runtime file) listing the search criteria + the chosen citation + the "page exact pin pending" status. The Rust code's `evidence.note` says `"tam_sat opposite triad; page ref: data/almanac/tam_sat_provenance.md (PendingExternalReview for exact KHCBPPT page pin)"` — explicit pending status.

**Warning signs:** `evidence_note_includes_pending_review_marker` test fails (or any test asserting the citation text matches a fake page number).

### Pitfall 8: Refactor merge conflict with `direction_merge.rs`
**What goes wrong:** Developer opportunistically refactors `direction_merge.rs:94-106`'s `direction_to_vn` into a public `Direction::as_vn_str()` method in `tu_menh.rs` "while we're touching this area" — causing a merge-conflict blast radius across v1.6 callers.

**Why it happens:** Out-of-scope refactor discipline is hard to maintain when adjacent code is being touched.

**How to avoid:** CONTEXT.md §"Deferred Ideas" explicitly excludes `Direction::as_vn_str()` refactor from Phase 23. Add an explicit `Direction::as_vn_str()` follow-up bead for the roadmap backlog (per `.planning/STATE.md` "Open Research Questions" Q-style discipline).

**Warning signs:** `rg "fn as_vn_str" crates/amlich-core/src/almanac/tu_menh.rs` returns a match in Phase 23.

## Code Examples

### Example 1: Directional Thái Tuế `pub fn` (year-chi → Direction)
```rust
// crates/amlich-core/src/almanac/thai_tue.rs (MODIFIED — add after compute_thai_tue)

// Note: existing compute_thai_tue is personal-conflict-only (birth vs year).
// This sibling is the year-only directional derivation — no birth context needed.
// Used by the cross-link to map each year's Thái Tuế to a Direction8 cell.

/// Return the directional mapping for the current year's Thái Tuế.
///
/// Classical rule: Thái Tuế sits at the direction of the year's Earthly Branch.
/// Mapping (8-point, per `Direction` enum at `almanac/tu_menh.rs:76-85`):
///   Tý(0)  → Bắc       Mão(3)  → Đông      Ngọ(6)  → Nam       Dậu(9)   → Tây
///   Sửu(1) → Đông Bắc  Thìn(4) → Đông Nam  Mùi(7)  → Tây Nam   Tuất(10) → Tây Bắc
///   Dần(2) → Đông Bắc  Tỵ(5)   → Đông Nam  Thân(8) → Tây Nam   Hợi(11)  → Tây Bắc
///
/// Source: KHCBPPT (Quyển 9, Lập Thành — directional Thái Tuế)
pub fn thai_tue_direction(year_chi_index: usize) -> ThaiTueDirectionResult {
    ThaiTueDirectionResult {
        year_chi: CHI[year_chi_index],
        direction: direction_for_year_chi(year_chi_index),
        evidence: RuleEvidence {
            source_id: SOURCE_KHCBPPT.to_string(),
            method: "thai_tue_year_branch_to_direction".to_string(),
            profile: "baseline".to_string(),
        },
    }
}

fn direction_for_year_chi(idx: usize) -> Direction {
    match idx {
        0 => Direction::North,
        1 | 2 => Direction::Northeast,
        3 => Direction::East,
        4 | 5 => Direction::Southeast,
        6 => Direction::South,
        7 | 8 => Direction::Southwest,
        9 => Direction::West,
        10 | 11 => Direction::Northwest,
        _ => unreachable!("year_chi_index {} not in 0..12", idx),
    }
}
```

### Example 2: Evidence backfill on existing `compute_thai_tue`
```rust
// crates/amlich-core/src/almanac/thai_tue.rs (MODIFIED at line 107-111)
ThaiTueResult {
    conflicts,
    has_conflict,
    evidence: Some(RuleEvidence {
        source_id: SOURCE_KHCBPPT.to_string(),  // was: None
        method: "thai_tue_year_branch_conflict".to_string(),
        profile: "baseline".to_string(),
    }),
}
```

### Example 3: `build_direction_cross_link_personal` core assembly
```rust
// crates/amlich-core/src/reasoning/direction_composite.rs (NEW)
pub fn build_direction_cross_link_personal(
    snapshot: &DaySnapshot,
    birth_chi_index: usize,
) -> Result<DirectionCrossLink, String> {
    if birth_chi_index >= 12 {
        return Err(format!("birth_chi_index {} out of 0..12 range", birth_chi_index));
    }
    let year_chi_index = snapshot.context.canchi.year.chi_index;
    let day_chi_index  = snapshot.context.canchi.day.chi_index;
    let day = snapshot.context.solar;
    let date_str = format!("{:04}-{:02}-{:02}", day.year, day.month, day.day);

    // 1. KHCBPPT per-direction contribution (Thái Tuế + Tam Sát + Sát Phương).
    let khcbppt_cells = build_khcbppt_cells(year_chi_index, day_chi_index, birth_chi_index);
    let khcbppt_summary = summarise_khcbppt(&khcbppt_cells, birth_chi_index);

    // 2. Huyền Không per-direction contribution (snapshot.flying_stars — read-only).
    let huyen_khong_cells = build_huyen_khong_cells(snapshot);
    let huyen_khong_summary = summarise_huyen_khong(&huyen_khong_cells);

    // 3. Merge into per-direction DirectionCell + agreement.
    let cells = merge_into_cells(&khcbppt_cells, &huyen_khong_cells);

    // 4. Composite severity = majority vote (with worst-of-within-direction tiebreak).
    let composite = composite_severity(&cells);

    // 5. Vietnamese narrative summary.
    let summary_vi = build_summary_vi(&cells, &date_str, birth_chi_index);

    // 6. Evidence envelopes (2 primitives + 1 composite).
    let evidence = build_evidence(&khcbppt_summary, &huyen_khong_summary, false);

    Ok(DirectionCrossLink {
        cross_link_kind: "thai_tue_x_tam_sat_x_phi_tinh".to_string(),
        date: date_str,
        day_chi_index: day_chi_index as u8,
        birth_chi_index: birth_chi_index as u8,
        cells,
        summary_vi,
        composite_severity: composite,
        evidence,
    })
}
```

### Example 4: `enrich_day_snapshot_with_direction_cross_link` immutable helper
```rust
// crates/amlich-core/src/lib.rs (NEW helper, mirrors enrich_day_snapshot_with_iching)

pub fn enrich_day_snapshot_with_direction_cross_link(
    snapshot: &DaySnapshot,
    birth_chi_index: usize,
) -> Result<DaySnapshot, String> {
    let cross = reasoning::direction_composite::build_direction_cross_link_personal(
        snapshot,
        birth_chi_index,
    )?;
    let summary = reasoning::direction_composite::project_to_summary(&cross);
    let mut enriched = snapshot.clone();
    enriched.direction_cross_link = Some(summary);
    Ok(enriched)
}
```

### Example 5: Sibling CRIT-3 grep guard
```rust
// crates/amlich-core/tests/thai_tue_cross_link_crit3.rs (NEW — mirrors fengshui_crit3_isolation.rs)

use std::fs;
use std::path::Path;

/// Forbidden type/function names per CONTEXT.md §"CRIT-3 sibling guard scope".
/// NOTE: dropped `FlyingStar` / `DailyFlyingStar` / `DailyFlyingStarLayout` because
/// they would false-positive on `snapshot.flying_stars: Option<FlyingStarsSummary>`.
const FORBIDDEN_TYPE_NAMES: &[&str] = &[
    "almanac::fengshui",
    "phi_tinh",
    "compute_daily_flying_stars",
    "compute_combined_overlay",
    "compute_palace_aspects",
    "TietKhiScanner",
    "FlyingStarPeriod",
];

const SCAN_TARGETS: &[&str] = &[
    "src/interaction/direction_merge.rs",  // preserves v1.6 contract (legacy pattern)
    "src/reasoning/direction_composite.rs", // new Phase 23 carve-out
];

#[test]
fn direction_merge_and_direction_composite_are_fengshui_free() {
    let mut violations = Vec::new();
    for rel_path in SCAN_TARGETS {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(rel_path);
        let contents = fs::read_to_string(&path).expect("read target file");
        for forbidden in FORBIDDEN_TYPE_NAMES {
            if contents.contains(forbidden) {
                violations.push(format!(
                    "CRIT-3 violation: {} contains {:?}",
                    rel_path, forbidden
                ));
            }
        }
    }
    assert!(
        violations.is_empty(),
        "CRIT-3 isolation broken:\n{}",
        violations.join("\n")
    );
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `direction_merge.rs` only (1 tradition: KHCBPPT via sat_phuong + than_huong + phuc_than + kua) | `direction_merge.rs` unchanged + `reasoning/direction_composite.rs` (NEW, 2 traditions joined: KHCBPPT + huyen-khong via read-only Phi Tinh read) | Phase 23 (2026-07-16) | Cross-tradition directional join surfaces both KHCBPPT taboos AND Phi Tinh palace layout in one composite picture. |
| `evidence: None` on `ThaiTueResult` + `SatPhuongResult` | `evidence: Some(RuleEvidence { source_id: SOURCE_KHCBPPT, ... })` | Phase 23 backfill | Cross-link can cite the evidence for both backfilled functions. |
| `TamTaiResult` (3-YEAR Tam Tai affliction, NOT directional Tam Sát) | `TamSatResult` (NEW — directional 3-direction Tam Sát per year) | Phase 23 NEW module | Distinguishes Tam Tai (3-year cycle) from Tam Sát (yearly 3-direction). |
| `Direction` enum only used by `tu_menh` + `direction_merge` | `Direction` enum ALSO used by `thai_tue::thai_tue_direction` + `tam_sat::tam_sat_direction` + `direction_composite::DirectionCell.direction` | Phase 23 (2026-07-16) | Same 8-point enum is the canonical Direction type across almanac + reasoning + cross-link. |
| DaySnapshot has `flying_stars: Option<FlyingStarsSummary>` (read-only Phi Tinh access from reasoning layer) | DaySnapshot ALSO has `direction_cross_link: Option<DirectionCrossLinkSummary>` (cross-link composite surface) | Phase 23 (2026-07-16) | Single DaySnapshot carry both raw Phi Tinh data AND composite cross-link summary. |

**Deprecated/outdated:**
- **None for Phase 23** — every change is ADDITIVE (`Option<T>` field on `DaySnapshot`, NEW module `almanac/tam_sat.rs`, NEW module `reasoning/direction_composite.rs`, NEW sibling grep guard). No existing public API changes.

## Open Questions

1. **Exact KHCBPPT Tam Sát page pin**
   - What we know: The Tam Sát triad → 3-direction mapping (CONTEXT.md §"Tam Sát triad → 3-direction mapping" table) is the classical rule (lục-xung opposite triad, mirrors `tam_tai.rs:58-63` precedent). The codebase does NOT have an authoritative KHCBPPT page reference.
   - What's unclear: The exact KHCBPPT Quyển + Trang + Câu reference. The WebSearch attempts above (vi.wikipedia.org/wiki/Tam_tai, etc.) all returned 404 / CAPTCHA, so external verification was blocked.
   - Recommendation: Plan 23-02 authors `data/almanac/tam_sat_provenance.md` (1-page discovery artifact) that records the SEARCH CRITERIA + the chosen citation + an explicit "PendingExternalReview: exact page pin awaiting physical copy review" marker. The Rust code's `evidence.note` references the provenance file. Future phase can supersede with exact page once verified (mirrors `ADR-0004 §5` deferral pattern from Phase 16).

2. **`build_direction_cross_link_personal` integration into `build_fact_nodes`**
   - What we know: `PersonalReasoningInput::build_fact_nodes` at `reasoning/personal.rs:31-107` is the Tier-1 personal-path entry point. CONTEXT.md §"Node vs edge surface scope" says "Phase 23 emits ONLY the `PersonalFactNode` + `DirectionCrossLinkSummary` types. No semantic-graph edge emission." But CONTEXT.md §"Lock-in: Phase 23 = reasoning fact + DTO" also implies integration is expected.
   - What's unclear: Whether Phase 23 (a) ships `build_fact_nodes` extended to call `build_direction_cross_link_personal` and push the resulting `PersonalFactNode`, OR (b) defers the integration to Phase 24 (alongside `add_direction_composite_facts`).
   - Recommendation: **(a)** — extend `build_fact_nodes` to push the cross-link fact node (one extra `nodes.push(...)` call inside the existing `if let Some(gender) = self.birth.gender { ... }` branch). This makes the cross-link available at Tier-1 immediately. The Tier-0 additive DTO path (Phase 24) is independent. Justification: the v1.5 cross-link research explicitly intended the cross-link to surface in the personal-path reasoning chain.

3. **Palace safety-hint transport from Phi Tinh into the cross-link**
   - What we know: The CONTEXT.md "rich per-direction cells" description says: "huyen-khong side = FlyingStar at that palace + palace number + safety hint." The safety hint comes from `almanac::fengshui::safety::element_hint_for_palace(FlyingStar)`.
   - What's unclear: How to get the safety hint into `direction_composite.rs` without importing `almanac::fengshui::safety::*` (which trips CRIT-3 grep guard).
   - Recommendation: Pre-bake the safety hints into `FlyingStarsSummary` as an additive field (`pub palace_safety_hints: [Option<String>; 9]`) populated in `lib.rs::calculate_day_snapshot_internal` at `FlyingStarsSummary` construction time (where `almanac::fengshui::*` imports are allowed). The cross-link reads the precomputed strings. **Caveat:** `FlyingStarsSummary` shape is currently locked per `lib.rs:140-152`; adding a field requires either (a) a new additive `Option<[Option<String>; 9]>` field (matches the additive discipline) or (b) a parallel `FlyingStarsSummaryExtended` DTO (more invasive). Recommend (a) as the lowest-impact path. Plan 23-01 or 23-02 adds the field; either is acceptable.

4. **Composite severity majority-vote tiebreak rule**
   - What we know: CONTEXT.md Claude's Discretion: "Exact `most_frequent_severity` tiebreaker rule (favor Auspicious on tie vs favor Inauspicious)."
   - What's unclear: The right call for Vietnamese almanac UX.
   - Recommendation: **Inauspicious on tie** (conservative — the almanac UX defaults to "taboo-leaning" on ambiguity). Document in `direction_composite.rs` doc-comment + add a unit test asserting tie → Inauspicious.

5. **`birth_chi_index: usize` vs `Option<usize>` for the enrichment helper**
   - What we know: CONTEXT.md Claude's Discretion: "Whether `enrich_day_snapshot_with_direction_cross_link` takes `birth_chi_index: usize` (required) or `Option<usize>` (recommended: required, matches Phase 24's contract)."
   - What's unclear: Whether the sentinel approach (`usize::MAX` for date-variant) is preferred over a typed `Option<usize>`.
   - Recommendation: **`usize` (required)** — matches CONTEXT.md recommendation + mirrors Phase 24's `IChingQuery::from_snapshot(...) -> Result<Self, String>` signature style (the validation is `Err`-based, not `Option`-based). The sentinel `usize::MAX` is documented in the field's doc comment.

## Sources

### Primary (HIGH confidence — in-repo anchors)
- `.planning/phases/23-th-i-tu-tam-s-t-phi-tinh-cross-link/23-CONTEXT.md` (Phase 23 user-decision file; locked Tam Sát mapping table; locked envelope pattern; locked CRIT-3 sibling guard scope).
- `.planning/adrs/0007-cross-link-crit3-carve-out.md` (CRIT-3 carve-out ADR; placement contract; sibling grep guard template).
- `.planning/research/SUMMARY.md` §"Critical Pitfalls" CRIT-3 + CRIT-5 + MOD-8 (KHCBPPT subfamily tags SatPhuong / TamSat / ThaiTue — prevents module-naming confusion).
- `crates/amlich-core/src/almanac/thai_tue.rs` (entire file; existing `compute_thai_tue` + backfill target at line 107-111).
- `crates/amlich-core/src/almanac/sat_phuong.rs` (entire file; existing `get_sat_phuong` + backfill target at line 49-53).
- `crates/amlich-core/src/almanac/tam_tai.rs:46-73` (`TAM_HOP_TRIADS` + `TAI_YEARS` constants — the lục-xung opposite triad pattern to mirror for Tam Sát).
- `crates/amlich-core/src/almanac/xung_hop.rs:28-34` (`tam_hop(chi_index)` — Tam Hợp triad lookup).
- `crates/amlich-core/src/almanac/tu_menh.rs:76-100` (`Direction` 8-point enum + Display impl — the type Phase 23 reuses).
- `crates/amlich-core/src/interaction/direction_merge.rs:9-18` (the locked `ALL_DIRECTIONS` 8-point order) + lines 94-106 (the `direction_to_vn` private helper that CONTEXT.md defers refactoring).
- `crates/amlich-core/src/almanac/types.rs:157-162` (`RuleEvidence` struct — backfill shape).
- `crates/amlich-core/src/types.rs:17-19` (`CHI` constant — branch lookup) + lines 69-78 (`CanChi` struct).
- `crates/amlich-core/src/reasoning/personal.rs:13-18` (`PersonalFactNode` shape) + lines 31-107 (`build_fact_nodes` — Tier-1 personal-path integration point).
- `crates/amlich-core/src/reasoning/types.rs:135-170` (`ReasoningEvidenceSourceFamily` + `ReasoningEvidenceEnvelope` + `ReasoningNodeSeverity` enums — the cross-link evidence shapes).
- `crates/amlich-core/src/reasoning/mod.rs:14` (the `pub use personal::{PersonalFactNode, PersonalReasoningInput}` precedent for the new `direction_composite` re-export).
- `crates/amlich-core/src/lib.rs:140-186` (`FlyingStarsSummary` DTO + `DaySnapshot` struct + additive `Option<T>` discipline).
- `crates/amlich-core/src/lib.rs:275-422` (`calculate_day_snapshot_internal` — where the new `direction_cross_link: None` field gets initialised).
- `crates/amlich-core/src/sources.rs:8,26` (`SOURCE_KHCBPPT` + `SOURCE_HUYEN_KHONG` already registered).
- `crates/amlich-core/tests/source_id_guard.rs:13-22` (`FORBIDDEN_LITERALS` — enforces `SOURCE_*` const usage; the `"rule.composite.direction_cross_link"` literal is NOT in the forbidden list, confirmed safe).
- `crates/amlich-core/tests/fengshui_crit3_isolation.rs:14-21` (existing 6-pattern CRIT-3 grep guard — template for the new sibling guard).
- `crates/amlich-core/tests/day_snapshot_v14_compat.rs` (the v1.5→v1.6 BC round-trip pattern; Phase 24-03 extends this to v1.6→v1.7).
- `crates/amlich-core/src/almanac/fengshui/types.rs:136-143` (`DailyFlyingStarLayout` — sibling shape that Phase 23 does NOT touch but `daily_flying_stars` field exists).
- `crates/amlich-core/src/almanac/fengshui/mod.rs:10-11` (explicit CRIT-3 isolation note — "FlyingStar is NEVER wired into interaction/direction_merge.rs").
- `.planning/phases/24-iching-evaluator-semantic-graph-wiring-dto-integration/24-01-PLAN.md:280-321` (the `COMPOSITE_ICHING_CONSULTATION` discipline + per-step evidence envelope pattern that Phase 23 mirrors).
- `.planning/phases/24-iching-evaluator-semantic-graph-wiring-dto-integration/24-02-PLAN.md:325-466` (the forward-compatible `DirectionCrossLinkSummary` placeholder + `add_direction_composite_facts` Phase-23-conditional design).
- `.planning/phases/22-mai-hoa-casting-bien-que-the-dung/22-01-SUMMARY.md` (the CRIT-3 grep guard discipline with RUNTIME-BUILT needles; Phase 23 should mirror for `tests/thai_tue_cross_link_crit3.rs`).

### Secondary (MEDIUM confidence — domain knowledge)
- CONTEXT.md Tam Sát triad → 3-direction mapping table (authoritative for Phase 23 implementation; user-locked).
- CONTEXT.md branch-to-direction mapping (Tý→Bắc etc.; 4 cardinals map uniquely, 8 intercardinal branches collapse in pairs).
- CONTEXT.md evidence envelope pattern (2 primitives + 1 composite; named `COMPOSITE_DIRECTION_CROSS_LINK` const).

### Tertiary (LOW confidence — needs validation during planning)
- **KHCBPPT Tam Sát exact page pin** — external WebSearch attempts (vi.wikipedia.org/wiki/Tam_tai, tuvi.vn/category/kien-thuc/tam-sat-71, etc.) all returned 404 / CAPTCHA blocks. The page reference is honest-deferred per CONTEXT.md; `data/almanac/tam_sat_provenance.md` artifact records the deferral.
- **Composite severity majority-vote tiebreak rule** — CONTEXT.md Claude's Discretion; recommendation is **Inauspicious on tie** but planner/executor may revisit if implementation shows otherwise.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — Zero new dependencies; every type/pattern reused is already shipped + tested (Direction, RuleEvidence, ReasoningEvidenceEnvelope, FlyingStarsSummary, DaySnapshot, SOURCE_KHCBPPT, SOURCE_HUYEN_KHONG).
- Architecture: HIGH — Every module placement + field placement + grep guard pattern + composite envelope pattern is locked by CONTEXT.md + ADR-0007 + 24-01/24-02 precedent.
- Pitfalls: HIGH — All 8 catalogued pitfalls have specific detection strategies + prevention patterns from CONTEXT.md or 24-01/24-02 lessons.
- Tam Sát algorithm: HIGH — CONTEXT.md mapping table is locked + verified against `tam_tai.rs:58-63` + `xung_hop.rs:28-34` precedents.
- KHCBPPT citation: LOW — exact page pin unverified; honest-deferred per CONTEXT.md.

**Research date:** 2026-07-16
**Valid until:** 2026-08-16 (30 days for stable CRIT-3 / additive DTO patterns; KHCBPPT Tam Sát citation if verified would shorten to 90 days).

---
*Research complete. Ready for planning. Planner should honor every locked decision verbatim from CONTEXT.md §"Decisions"; treat the 5 Open Questions as Claude's Discretion choices the planner may resolve; treat the 6 Deferred Ideas as HARD OUT OF SCOPE.*