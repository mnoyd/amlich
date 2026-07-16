# Phase 23: Thái Tuế / Tam Sát ⇄ Phi Tinh Cross-Link - Context

**Gathered:** 2026-07-16
**Status:** Ready for planning

<domain>

## Phase Boundary

A directional Thái Tuế `pub fn` (year-chi → `Direction`, `source_id: khcbppt`) + a classical 3-direction Tam Sát module (`almanac/tam_sat.rs`, also `khcbppt`) + a read-only `build_direction_cross_link` at `reasoning/direction_composite.rs` that surfaces BOTH the KHCBPPT taboos AND the `huyen-khong` Phi Tinh palace layout as one composite `PersonalFactNode`, emitting 2 primitive envelopes + 1 `rule.composite.direction_cross_link` composite envelope. The cross-link result projects to a slim `DirectionCrossLinkSummary` DTO attached to a new additive `DaySnapshot.direction_cross_link: Option<DirectionCrossLinkSummary>` field via an explicit immutable enrichment helper. Closes XLK-01, XLK-02, XLK-03.

The directional composite cross-link (Phase 23) feeds the IChing evaluator + semantic-graph wiring (Phase 24), which depends on Phase 23 shipping `DirectionCrossLinkSummary` so Plan 24-02 doesn't need a placeholder type. The directional cross-link lives at `reasoning/direction_composite.rs`, NOT in `interaction/direction_merge.rs` (CRIT-3 isolation per ADR-0007 §1).

</domain>

<decisions>

## Implementation Decisions

### Composite view shape

- **Rich per-direction cells** — each cell carries the full reasoning: KHCBPPT side = Thái Tuế directional year-clash (Direction + conflict kind Truc/Xung/Hai/Hinh/Pha) + Tam Sát 3-direction overlap + Sát Phương day-chi; huyen-khong side = FlyingStar at that palace + palace number + safety hint.
- **Per-direction `agreement` field** — `Some(Agreement | BothSilent | KhcbpptOnly | HuyenKhongOnly | Conflict)` when both traditions have data; `None` when one side is empty (date variant or one tradition silently omits a direction).
- **Vietnamese narrative `summary_vi`** at top of fact node — e.g. "Hôm nay, Thái Tuế xung hướng Bắc và Tam Sát trùng Đông Bắc; Phi Tinh Nhất Bạch tại Trung Cung kỵ khai trương."
- **Composite severity = majority vote** across the 8 directions' per-direction severities (with worst-of within a single direction). `ReasoningNodeSeverity` enum (existing) is the type.
- **8-point direction order** matches existing `direction_merge.rs:9-18` ALL_DIRECTIONS order.

### No-birth-context behavior

- **Two entry-point functions** in `reasoning/direction_composite.rs`:
  - `build_direction_cross_link_personal(snapshot: &DaySnapshot, birth_chi_index: usize) -> Result<DirectionCrossLink, String>` — full surface, all 3 columns populated.
  - `build_direction_cross_link_date(snapshot: &DaySnapshot) -> Result<DirectionCrossLink, String>` — date-only Tier-0 path. Thái Tuế column empty per direction, Tam Sát + Phi Tinh still populated.
- **`birth_chi_index: usize` (required field)** in both `DirectionCrossLink` and `DirectionCrossLinkSummary`. Date-variant uses a documented sentinel `usize::MAX` (out of 0..=11 branch range). Field carries a doc comment explaining the convention.
- **Composite envelope `note` explains partial data** in the date variant — e.g. "Cross-link surfaces Phi Tinh + Tam Sát only — Thái Tuế directional omitted (no birth context)." Personal variant's note is a fixed description of the cross-link.
- **Agreement = null** (`#[serde(skip_serializing_if = "Option::is_none")]`) when one tradition has no data for a direction. Triple-state in JSON.

### Directional granularity

- **Reuse existing `Direction` 8-point enum** (`almanac/tu_menh.rs:76-85`: `North, Northeast, East, Southeast, South, Southwest, West, Northwest`). Do NOT mint a new `Direction8` or `Direction4` type.
- **No 24-sơn scope creep** — that's a v1.8 differentiator per `.planning/research/FEATURES.md:42` (DF-04).
- **`Direction::as_vn_str()` refactor is OUT OF SCOPE for Phase 23.** `direction_composite.rs` carries its own private `direction_to_vn()` copy mirroring `direction_merge.rs:94-106`. Refactor the duplicate into a public `tu_menh.rs` method in a follow-up phase.

### Node vs edge surface scope

- **Phase 23 emits ONLY the `PersonalFactNode` + `DirectionCrossLinkSummary` types.** No semantic-graph edge emission.
- **Phase 24 authors the `LocatedAt` / `Transforms` edge wiring** (per FND-12 ontology reservation in `.planning/adrs/0007-cross-link-crit3-carve-out.md` §5) inside `build_day_snapshot_graph`'s `add_direction_composite_facts` step.
- Cleaner separation: Phase 23 = reasoning fact + DTO; Phase 24 = semantic-graph wiring into DaySnapshot.

### CRIT-3 sibling guard scope

- **NEW file: `crates/amlich-core/tests/thai_tue_cross_link_crit3.rs`** — sibling to `tests/fengshui_crit3_isolation.rs`.
- **Scans TWO modules**: `src/interaction/direction_merge.rs` (preserves v1.6 contract) AND `src/reasoning/direction_composite.rs` (new Phase 23 carve-out).
- **`FORBIDDEN_TYPE_NAMES` list (Phase 23)**: `["almanac::fengshui", "phi_tinh", "compute_daily_flying_stars", "compute_combined_overlay", "compute_palace_aspects", "TietKhiScanner", "FlyingStarPeriod"]`.
- **DROPPED from new guard**: `FlyingStar`, `DailyFlyingStar`, `DailyFlyingStarLayout` — would false-positive on legitimate `snapshot.flying_stars` (the `FlyingStarsSummary` DTO at `lib.rs:140-152`) and on the `palace_overlays` field's `FlyingStar` field type.
- **Existing `fengshui_crit3_isolation.rs` UNCHANGED** — it continues to scan `direction_merge.rs` with the original 6-pattern list.

### Tam Sát triad → 3-direction mapping

- **Classical lục-xung opposite triad** — Tam Sát for a year-chi triad is the 3 branches of the OPPOSITE triad (each branch +6 mod 12). Mirrors the existing `tam_tai.rs:58-63` `TAI_YEARS` precedent exactly.
- **Concrete mapping** (already validated against `tam_tai.rs:58-63` + `xung_hop.rs:28-34`):

  | Tam Hợp triad | Element | Tam Sát branches (opposite) | Tam Sát directions (8-point) |
  |---|---|---|---|
  | Thân·Tý·Thìn | Thủy | Dần·Ngọ·Tuất | Đông Bắc, Nam, Tây Bắc |
  | Hợi·Mão·Mùi | Mộc | Tỵ·Dậu·Sửu | Đông Nam, Tây, Đông Bắc |
  | Dần·Ngọ·Tuất | Hỏa | Thân·Tý·Thìn | Tây Nam, Bắc, Đông Nam |
  | Tỵ·Dậu·Sửu | Kim | Hợi·Mão·Mùi | Tây Bắc, Đông, Tây Nam |

- **Branch-to-direction mapping**: 4 cardinals map uniquely (Tý→Bắc, Mão→Đông, Ngọ→Nam, Dậu→Tây); 8 intercardinal branches collapse in pairs (Sửu+Dần→Đông Bắc, Thìn+Tỵ→Đông Nam, Mùi+Thân→Tây Nam, Tuất+Hợi→Tây Bắc).
- **Authoritative KHCBPPT citation** is NOT in the codebase yet. Plan 23-02 authors `data/almanac/tam_sat_provenance.md` (1-page discoverable artifact, not runtime).

### Evidence envelope pattern

- **2 primitive envelopes + 1 composite envelope per `DirectionCrossLink`**:
  - Primitive 1: `source_id = SOURCE_KHCBPPT`, `method = "thai_tue_direction+tam_sat+sat_phuong"`, `note` carries the per-tradition summary.
  - Primitive 2: `source_id = SOURCE_HUYEN_KHONG`, `method = "phi_tinh.palace_layout"`, `note` carries the palace-overlay summary.
  - Composite: `source_id = COMPOSITE_DIRECTION_CROSS_LINK` (named const), `method = "v17.read_only_join"`, `note` explains what's joined and (for date variant) why one side is missing.
- **Composite envelope's literal source_id held in ONE named const**: `pub const COMPOSITE_DIRECTION_CROSS_LINK: &str = "rule.composite.direction_cross_link";` in `direction_composite.rs`. Mirrors `COMPOSITE_ICHING_CONSULTATION` discipline from `24-01-PLAN.md:303`.
- **`source_id_guard.rs::FORBIDDEN_LITERALS` does NOT include `"rule.composite.*"` strings** — confirmed safe (the array only guards the 9 corpus source_ids).

### Evidence backfill on existing almanac functions

- **`ThaiTueResult.evidence: None` → `evidence: Some(RuleEvidence { source_id: SOURCE_KHCBPPT, ... })`** at `thai_tue.rs:107-111` (within `compute_thai_tue`'s return).
- **`SatPhuongResult.evidence: None` → `evidence: Some(RuleEvidence { source_id: SOURCE_KHCBPPT, ... })`** at `sat_phuong.rs:49-53` (within `get_sat_phuong`'s return).
- **Round-trip tests FIRST** in `crates/amlich-core/tests/almanac_backfill_compat.rs` — mirrors `tests/day_snapshot_v14_compat.rs` pattern. Verify v1.6 JSON without `evidence` deserialises correctly + populated v1.7 JSON round-trips byte-equal BEFORE the backfill lands.

### Claude's Discretion

- Exact `khcbppt_summary_vi` / `huyen_khong_summary_vi` Vietnamese wording per direction (templates vs hand-written).
- Exact `most_frequent_severity` tiebreaker rule (favor Auspicious on tie vs favor Inauspicious).
- Whether `vn_cardinal_to_direction("Nam")` is a private helper in `direction_composite.rs` or a shared helper in `tu_menh.rs` (recommend private for Phase 23; refactor in follow-up).
- Exact wording of `data/almanac/tam_sat_provenance.md` (1-page artifact, not runtime).
- Whether `enrich_day_snapshot_with_direction_cross_link` takes `birth_chi_index: usize` (required) or `Option<usize>` (recommended: required, matches Phase 24's contract).

</decisions>

<specifics>

## Specific Ideas

- "I want the cross-link to feel like looking at a Vietnamese-physics version of a weather radar — two traditions, one picture, with the disagreements highlighted."
- "Per-direction agreement should be visible — when both traditions flag the same direction as taboo, that's a strong signal worth surfacing in the UI."
- "Date-only consumers (Tier-0) should still get the Phi Tinh + Tam Sát surface — Thái Tuế is the only piece that needs birth context."
- "Reuse the existing Direction 8-point enum — don't proliferate types."
- "The composite envelope's source_id is a rule identifier (`rule.composite.direction_cross_link`), NOT a corpus source — it's the only envelope pattern compatible with the CRIT-3 grep guard."

</specifics>

<code_context>

## Existing Code Insights

### Reusable Assets

- **`Direction` enum** (`almanac/tu_menh.rs:76-85`, 8-point, English names + `serde(rename_all = "snake_case")`) — reused for Thái Tuế directional + Tam Sát 3-direction outputs. No new type.
- **`tam_hop(chi_index) -> [&str; 3]`** (`almanac/xung_hop.rs:28-34`) — Tam Hợp triad lookup, used by Tam Sát's group computation (`chi_index % 4`).
- **`TAI_YEARS` constant** (`almanac/tam_tai.rs:58-63`) — exact precedent for the lục-xung opposite triad pattern. Tam Sát's triad → branches mapping mirrors this verbatim.
- **`PersonalFactNode { id, summary_vi, severity, evidence: Vec<ReasoningEvidenceEnvelope> }`** (`reasoning/personal.rs:13-18`) — the reasoning-layer fact-node shape that `build_direction_cross_link` returns.
- **`ReasoningEvidenceEnvelope { source_family, source_id, method, note }`** (`reasoning/types.rs:147-153`) — composite envelope + 2 primitive envelopes use this shape.
- **`ReasoningNodeSeverity` enum** (`reasoning/types.rs:155-170`: Auspicious, Inauspicious, HardTaboo, SoftTaboo, HoangDao, HacDao) — composite severity uses this.
- **`SOURCE_KHCBPPT` + `SOURCE_HUYEN_KHONG` constants** (`sources.rs:8, 26`) — primitive envelope source_ids.
- **`FlyingStarsSummary` DTO** (`lib.rs:140-152`) — `snapshot.flying_stars: Option<FlyingStarsSummary>`; cross-link reads this, not the underlying `almanac::fengshui` types.
- **`DailyFlyingStarLayout` DTO** (`lib.rs:173`) — `snapshot.daily_flying_stars: Option<DailyFlyingStarLayout>`; cross-link reads this for daily layer (Phase 23 reads ONLY `FlyingStarsSummary` for annual layer; daily layer is a follow-up consideration).
- **`compute_thai_tue(birth_chi, current_year_chi) -> ThaiTueResult`** (`almanac/thai_tue.rs:53-112`) — existing personal-conflict-only Thái Tuế. Phase 23 ADDS a sibling `thai_tue_direction(year_chi) -> Direction` without modifying the personal-conflict API.
- **`get_sat_phuong(chi_index) -> SatPhuongResult`** (`almanac/sat_phuong.rs:49-54`) — existing 1-direction day-chi Sát Phương. Phase 23 ADDS a new `almanac/tam_sat.rs` module without modifying `sat_phuong.rs`'s day-chi feature.

### Established Patterns

- **CRIT-3 grep-guard pattern** (`tests/fengshui_crit3_isolation.rs:1-44`) — `fs::read_to_string` + `FORBIDDEN_TYPE_NAMES` array + assert-zero-matches. Sibling guard at `tests/thai_tue_cross_link_crit3.rs` mirrors this verbatim.
- **Composite-envelope provenance pattern** (v1.5 `cross_source_curing` precedent, `semantic_graph/builders/day_snapshot.rs:627-730`) — each primitive envelope via separate `track_provenance` call + composite envelope as additional call. Phase 23 follows this discipline.
- **Read-only `&` references only** at the reasoning layer — `build_reasoning_input_graph` (`semantic_graph/builders/merge.rs:115`) and `build_day_snapshot_graph` (`semantic_graph/builders/day_snapshot.rs:754`) are read-only. `build_direction_cross_link` is a sibling.
- **Additive `Option<T>` DaySnapshot field** (`#[serde(default, skip_serializing_if = "Option::is_none")]` at `lib.rs:165, 168, 173, 179, 185`) — `direction_cross_link` follows this exact discipline, init to `None` in `calculate_day_snapshot_internal`, populated only via explicit enrichment.
- **Named const for composite rule identifiers** (e.g., `COMPOSITE_ICHING_CONSULTATION` per `24-01-PLAN.md:303`) — `COMPOSITE_DIRECTION_CROSS_LINK` follows this pattern.
- **Round-trip BC tests as first-class artifacts** (`tests/day_snapshot_v14_compat.rs`) — `tests/almanac_backfill_compat.rs` mirrors this for the two `evidence: None` backfills.

### Integration Points

- **`reasoning/mod.rs`** (`reasoning/mod.rs:1-26`) — Phase 23 adds `pub use direction_composite::{...}` selective re-export. The existing `pub use personal::{PersonalFactNode, PersonalReasoningInput}` is the precedent.
- **`lib.rs::calculate_day_snapshot_internal`** (`lib.rs:325-380`) — Phase 23 adds `direction_cross_link: None` to the `DaySnapshot` literal initialiser. Does NOT populate the field; explicit enrichment populates it.
- **`lib.rs::enrich_day_snapshot_with_iching`** (per `24-01-PLAN.md:343-355`) — Phase 23 authors `enrich_day_snapshot_with_direction_cross_link(snapshot: &DaySnapshot, birth_chi_index: usize) -> Result<DaySnapshot, String>` mirroring the IChing enrichment helper.
- **`semantic_graph/builders/day_snapshot.rs::DaySnapshotGraphBuilder`** (per `24-02-PLAN.md:36, 53-61`) — Phase 24 wires `add_direction_composite_facts` here. Phase 23 ONLY provides the `DirectionCrossLinkSummary` input; Phase 24 consumes it.
- **`sources.rs::SOURCE_KHCBPPT` + `SOURCE_HUYEN_KHONG`** — already-defined constants; Phase 23 imports them.

</code_context>

<deferred>

## Deferred Ideas

- **`Direction::as_vn_str()` refactor** — DRY consolidation of `direction_merge.rs:94-106`'s private `direction_to_vn` and `direction_composite.rs`'s private copy into a public `tu_menh.rs` method. Follow-up phase (avoids merge churn with `direction_merge.rs` in Phase 23).
- **24-sơn directional resolution** — v1.8 differentiator per `.planning/research/FEATURES.md:42` (DF-04). Out of scope for Phase 23; flag in roadmap backlog.
- **Daily Phi Tinh layer consumption** — `snapshot.daily_flying_stars: Option<DailyFlyingStarLayout>` is NOT consumed by Phase 23's cross-link (annual layer only via `snapshot.flying_stars`). Follow-up phase can extend `DirectionCell.huyen_khong_summary_vi` with the daily layer.
- **`vn_cardinal_to_direction` helper shared between `direction_composite.rs` + `sat_phuong.rs` callers** — currently private in `direction_composite.rs`. Refactor when a second caller appears.
- **`build_reasoning_input_graph` integration** — Phase 23's `build_direction_cross_link` is wired by `personal.rs::build_fact_nodes` (Tier-1 personal path) AND by `enrich_day_snapshot_with_direction_cross_link` (additive DTO path). A future phase may unify both paths or expose `build_direction_cross_link_date` to a Tier-0 reasoning builder.
- **Phase 18 daily Phi Tinh corpus integration** — `snapshot.daily_flying_stars` is a separate source of palace-layout data; the cross-link currently consumes annual `snapshot.flying_stars` only.

</deferred>

---

*Phase: 23-thai-tue-tam-sat-phi-tinh-cross-link*
*Context gathered: 2026-07-16*