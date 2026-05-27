# Phase 15: Semantic Graph Wiring + DTO Integration + E2E Validation — Research

**Researched:** 2026-05-28
**Domain:** Rust integration wiring — semantic graph ontology extension, DTO additive fields, provenance dedup, E2E smoke testing
**Confidence:** HIGH

## Summary

Phase 15 is the join point for the entire v1.5 milestone. Phases 11–14 have already delivered all pillar code (Văn khấn + Phi Tinh): the `rituals/` module, 60+ corpus entries, all 10 FS primitives + period + annual/monthly functions, the 81-cell aspect corpus, and the safety-hints API. Phase 15 must wire them additively into the two public surfaces: the `DaySnapshot` DTO and the semantic graph ontology, then validate the whole milestone with a 2026 E2E smoke test.

The codebase patterns are highly consistent. Every prior integration extension (Holiday.id, tu_menh on DayFortune, tang_can) follows the same additive `Option<T>` + `#[serde(default, skip_serializing_if = "Option::is_none")]` discipline. The semantic graph has a clear two-file ontology (`ontology.rs` declares `NodeConcept` and `EdgeConcept` enums plus `ConceptLabel`, `GraphOntology`; `builders/day_snapshot.rs` consumes them). The provenance system uses `ProvenanceEntry` with a `source_id: String` field, and the `SemanticNode.provenance: Vec<ProvenanceEntry>` allows multi-source dedup by accumulating entries. The Direction node already uses `NodeConcept::Direction` — a shared node carrying both KHCBPPT and Huyền Không provenance is achievable by calling `.with_provenance(entry)` twice.

The E2E smoke test follows the established `tests/<feature>_integration.rs` pattern (external crate consumer via `use amlich_core::`). The `TietKhiScanner::terms_for_year(year)` function makes it straightforward to enumerate all 24 Tiết Khí boundaries for 2026. The `find_van_khan_for_snapshot`, `compute_combined_overlay`, and `compute_palace_aspects` APIs are all public and callable without internal access.

**Primary recommendation:** Wire additive fields onto `DaySnapshot` (and a new `FlyingStarsSummary` DTO type) first. Then extend `ontology.rs` with two `NodeConcept` variants and three `EdgeConcept` variants (plus `ConceptLabel` entries and `GraphOntology` arrays). Then add a ritual+flying-star builder sub-function to `builders/day_snapshot.rs`. Finally write the 2026 E2E smoke test and a v1.4 backward-compat round-trip test.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| INT-01 | `DaySnapshot` gains `flying_stars: Option<FlyingStarsSummary>` — additive, `#[serde(default, skip_serializing_if = "Option::is_none")]` | `DaySnapshot` in `src/lib.rs:133` currently lacks serde derives — adding serde here (and defining `FlyingStarsSummary` as a DTO wrapper over `CombinedFlyingStarLayout`) enables round-trip JSON tests |
| INT-02 | `DaySnapshot` gains a ritual-surfacing field — additive, optional | Same additive field discipline; field type could be `rituals: Option<Vec<RitualEntrySummary>>` (a lightweight DTO) or simply `applicable_rituals: Option<Vec<String>>` (ritual_ids only); shape to be decided in planning |
| INT-03 | `NodeConcept::Ritual` + `NodeConcept::FlyingStar` added; `EdgeConcept::PrescribedFor`, `EdgeConcept::OccupiesPalace`, `EdgeConcept::CarriesElement` added | Both enums in `src/semantic_graph/ontology.rs`; `NodeConcept::label()`, `EdgeConcept::label()`, `ConceptLabel::as_str()`, and `GraphOntology::node_concepts()/edge_concepts()` ALL need updating for exhaustive match coverage |
| INT-04 | `FlyingStar` node carries only `source_id: "huyen-khong"`; ritual node only `source_id: "vn-folk-ritual"`; shared `Direction` node carries both provenance entries | `ProvenanceEntry.source_id` is a `String`; `SemanticNode.provenance: Vec<ProvenanceEntry>` allows multi-entry. Direction node is built in `add_travel_direction_fact` in `builders/day_snapshot.rs` — extend to append a second `ProvenanceEntry` with `SOURCE_HUYEN_KHONG` |
| INT-05 | v1.4 JSON fixture loads into v1.5 structs and re-serializes without unexpected fields | `DaySnapshot` currently derives only `Debug, Clone` — adding `Serialize, Deserialize` is required; all new fields must be `#[serde(default, skip_serializing_if = "Option::is_none")]`; existing `DayFortune` already derives serde |
| INT-06 | 2026 E2E smoke test on ≥30 representative dates: Tết Nguyên Đán, Sóc/Vọng×12, Vận 8→9 transition, leap months, 24 Tiết Khí boundaries | `TietKhiScanner::terms_for_year(2026)` yields all boundary dates; `calculate_day_snapshot(d,m,2026)` + `compute_combined_overlay(2026, m, &scanner)` + `find_van_khan_for_snapshot(&snap)` are all public |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `serde` + `serde_json` | existing in Cargo.toml | JSON round-trip for DaySnapshot DTO + backward-compat test | Already in project; no new dep needed |
| `amlich_core` (internal) | current | Public API surface for E2E test | The integration test file is an external-consumer black-box test |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `OnceLock` (std) | stable | Lazy static initialization | Not needed for Phase 15; pillar corpora already use OnceLock in Phases 13–14 |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `Option<Vec<String>>` for rituals field | `Option<Vec<RitualEntrySummary>>` | Full summary is richer but adds a new DTO type; id-only is simpler for INT-01/02 initial pass |
| Direct `CombinedFlyingStarLayout` as DTO | Wrapper `FlyingStarsSummary` | Wrapper allows slimming the JSON payload; direct embed exposes full nested struct |

**Installation:** No new crate dependencies required. All needed crates are already in `Cargo.toml`.

## Architecture Patterns

### Recommended Project Structure

New/modified files for Phase 15:
```
crates/amlich-core/src/
├── lib.rs                          ← Add serde derives + new Option fields to DaySnapshot
├── semantic_graph/
│   ├── ontology.rs                 ← Add Ritual/FlyingStar NodeConcept; PrescribedFor/OccupiesPalace/CarriesElement EdgeConcept
│   └── builders/
│       └── day_snapshot.rs         ← Add add_ritual_facts() + add_flying_star_facts() + multi-source Direction provenance
crates/amlich-core/tests/
├── integration_2026_smoke.rs       ← INT-06: 2026 E2E calendar smoke test (new file)
└── day_snapshot_v14_compat.rs      ← INT-05: backward-compat round-trip test (new file, or inline in smoke test)
```

### Pattern 1: Additive Optional DTO Field (v1.2 precedent)
**What:** All new fields on `DaySnapshot` must be `Option<T>` with `#[serde(default, skip_serializing_if = "Option::is_none")]`.
**When to use:** Every new field on any versioned DTO.
**Example (from `DayFortune` in types.rs:328):**
```rust
#[serde(skip_serializing_if = "Option::is_none")]
pub tu_menh: Option<super::tu_menh::KuaResult>,
```
For INT-01 the pattern will be:
```rust
// On DaySnapshot:
#[serde(default, skip_serializing_if = "Option::is_none")]
pub flying_stars: Option<FlyingStarsSummary>,
#[serde(default, skip_serializing_if = "Option::is_none")]
pub applicable_rituals: Option<Vec<String>>,  // or Vec<RitualEntrySummary>
```

### Pattern 2: NodeConcept / EdgeConcept Extension
**What:** Ontology enums are closed (exhaustive `match` arms). Adding new variants requires touching four locations in `ontology.rs`: the enum body, the `label()` method, the `ConceptLabel` enum, the `ConceptLabel::as_str()` method, and the `GraphOntology` static arrays.
**When to use:** Any new semantic concept.
**Verified from:** `src/semantic_graph/ontology.rs` — `NodeConcept` has 33 variants; `EdgeConcept` has 25 variants; both have parallel `label()` match arms; `ConceptLabel` mirrors all variants; `GraphOntology::node_concepts()` / `GraphOntology::edge_concepts()` return static slices.

For INT-03, add:
```rust
// NodeConcept additions:
Ritual,
FlyingStar,

// EdgeConcept additions:
PrescribedFor,
OccupiesPalace,
CarriesElement,
```
All six locations in `ontology.rs` must be updated atomically to keep Rust exhaustive match enforcement.

### Pattern 3: Multi-Source Direction Node Provenance (INT-04)
**What:** The existing `add_travel_direction_fact` in `builders/day_snapshot.rs` creates a `Direction` node with a single KHCBPPT provenance. To satisfy INT-04, append a second `ProvenanceEntry` with `SOURCE_HUYEN_KHONG`.
**When to use:** When Phi Tinh palace directions overlap with KHCBPPT travel direction.
**Pattern from existing code:**
```rust
let node = SemanticNode::new(...)
    .with_provenance(khcbppt_entry)
    .with_provenance(huyen_khong_entry);  // ← add second entry
```
The `with_provenance` method uses `push` so order is preserved. Dedup verification: the test asserts both `source_id` values appear in `node.provenance`.

### Pattern 4: E2E Smoke Test Structure (INT-06)
**What:** A single `integration_2026_smoke.rs` test file that enumerates ≥30 representative 2026 dates and asserts all pillar functions return non-error results for each.
**When to use:** Milestone-close validation.
**Pattern from:** `tests/rituals_integration.rs` + `tests/fengshui_invariants.rs`.

Date selection strategy for ≥30 dates covering all required categories:
- Tết 2026: 2026-02-17 (solar) = lunar 1/1 2026 [1 date]
- Sóc (lunar 1st) × 12: scan `calculate_day_snapshot` for each month's first day [12 dates]
- Vọng (lunar 15th) × 12: scan for each month's 15th day [12 dates]
- Vận 8→9 transition: 2026-02-04 (Lập Xuân 2026) and 1-2 days before/after [3 dates]
- 2026 leap month (lunar month 6 per generate_golden.rs): 2026-07-26..2026-07-28 [3 dates]
- 24 Tiết Khí boundaries: use `TietKhiScanner::terms_for_year(2026)` to get boundary JDs [24 dates]

Total distinct dates: well above 30. The test calls each date through:
1. `calculate_day_snapshot(d, m, y)` — succeeds without panic
2. `find_van_khan_for_snapshot(&snap)` — returns a slice (may be empty but must not panic)
3. `compute_combined_overlay(y, m as u8, &scanner)` — returns a valid layout
4. `compute_palace_aspects(y, m as u8, &scanner)` — returns 9 aspects

### Anti-Patterns to Avoid
- **Adding serde to `DaySnapshot` without `#[serde(default)]` on new fields:** This breaks backward compat deserialization of v1.4 JSON (field will be required).
- **String literals instead of SOURCE_* constants in new builder code:** Violates source-id discipline enforced by `tests/source_id_guard.rs` CI guard.
- **Wiring Phi Tinh into `direction_merge.rs`:** Explicitly out of scope per PITFALLS CRIT-3 and ROADMAP constraints. The Direction node provenance is added to the snapshot graph only — not to the interaction computation.
- **Using a `ConceptLabel` variant without adding it to `as_str()` match:** The compiler catches missing enum arms but `as_str()` is an infallible method — omitting the new variant will cause a compile error, so this is automatically caught.
- **Forgetting `GraphOntology::node_concepts()` / `edge_concepts()` static slice updates:** These slices are not enforced by the compiler. They must be manually updated to keep `GraphOntology` complete. Missing variants here will silently produce an incomplete graph when consumers enumerate all concepts.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Tiết Khí boundary enumeration for 2026 | Custom date arithmetic | `TietKhiScanner::new().terms_for_year(2026)` | Already returns `Vec<SolarTermWithDate>` with JD per boundary |
| Flying star computation | Re-implement | `compute_combined_overlay(year, month, &scanner)` | Phases 13–14 built and tested this |
| Ritual lookup for smoke test | Manual corpus search | `find_van_khan_for_snapshot(&snap)` | Phase 11 public API, integration-tested |
| Provenance tracking | Custom HashMap | `SemanticNode::with_provenance(entry)` | Existing builder method, push-based |
| Serde round-trip test | Custom JSON comparison | `serde_json::to_string` → `from_str` → `to_string` + `assert_eq!` | Byte-equal round-trip pattern from `tests/rituals_integration.rs:Test 5` |

**Key insight:** Every building block for Phase 15 already exists and is tested. Phase 15 is pure wiring and integration — zero new algorithmic logic needed.

## Common Pitfalls

### Pitfall 1: `DaySnapshot` Lacks Serde Derives
**What goes wrong:** `DaySnapshot` currently derives only `Debug, Clone` (confirmed at `src/lib.rs:133`). Adding `flying_stars: Option<FlyingStarsSummary>` requires serde to compile the DTO test.
**Why it happens:** DaySnapshot was internal-only; its constituent types (`DayFortune`, etc.) have serde but the top-level struct doesn't.
**How to avoid:** Add `#[derive(Serialize, Deserialize)]` to `DaySnapshot`, `DayContext`, `SolarDate`, `CanChiSet` in `src/lib.rs`. All nested types already have serde derives.
**Warning signs:** Compile error on `serde_json::to_string(&snapshot)` in the backward-compat test.

### Pitfall 2: Ontology Exhaustiveness — Six Locations
**What goes wrong:** Adding `NodeConcept::Ritual` to the enum body but forgetting `ConceptLabel::Ritual` in the `ConceptLabel` enum, or forgetting the `Self::Ritual => ConceptLabel::Ritual` arm in `NodeConcept::label()`, causes a compile error. However, `GraphOntology::node_concepts()` is a static slice that the compiler does NOT check for completeness — a missing variant there is a silent bug.
**Why it happens:** `GraphOntology` returns a hand-maintained `&'static [NodeConcept]` slice; there is no compile-time check that all variants are listed.
**How to avoid:** After adding variants to the enum and match arms, explicitly grep-verify `GraphOntology::node_concepts()` and `GraphOntology::edge_concepts()` to confirm every new variant is present.
**Warning signs:** Integration test that checks `GraphOntology::node_concepts().contains(&NodeConcept::Ritual)` is the only automated check — add this assertion to the test.

### Pitfall 3: Direction Node Provenance Dedup vs. Creation
**What goes wrong:** INT-04 requires the shared `Direction` node to carry BOTH `khcbppt` and `huyen-khong` provenance. But the node is only created once (by `add_travel_direction_fact`). If the Phi Tinh builder tries to `add_node` a new Direction node with the same ID, `SemanticGraph::add_node` will overwrite the existing node (HashMap insert), losing the KHCBPPT provenance.
**Why it happens:** `SemanticGraph::add_node` uses `HashMap::insert` which replaces on collision.
**How to avoid:** Extend the existing `add_travel_direction_fact` to append the `SOURCE_HUYEN_KHONG` provenance entry to the same node — do NOT create a separate node. Alternatively, use `graph.get_node_mut()` if available, or restructure to pass both entries at construction time.
**Warning signs:** Test asserting `node.provenance.len() == 2` and both source_ids present fails.

### Pitfall 4: Backward Compat — `DayContext` Also Needs Serde
**What goes wrong:** `DaySnapshot` contains `DayContext` which contains `CanChiSet` and `SolarDate` — none of these derive serde currently. The whole chain must be serde-able for INT-05.
**Why it happens:** `LunarDate` and `SolarTerm` (from `tietkhi.rs`) and `GioHoangDao` (from `gio_hoang_dao.rs`) — need to verify all nested types in `DayContext` already have serde derives.
**How to avoid:** Audit the full type tree: `DaySnapshot` → `DayContext` → (`SolarDate`, `LunarDate`, `CanChiSet` → `CanChi`, `SolarTerm`, `GioHoangDao`). Add serde derives to any that lack them. `DayFortune` already has serde.
**Warning signs:** `error[E0277]: ... does not implement Serialize` at compile time.

### Pitfall 5: Vận 8→9 Boundary in 2026 Smoke Test
**What goes wrong:** 2026 Lập Xuân is ~Feb 4 — this boundary marks Vận 8→9 within the calendar year 2026. Pre-Lập-Xuân dates in Jan/early-Feb 2026 are technically still Vận 8 scope. The smoke test should include at least 2 dates straddling this boundary.
**Why it happens:** `compute_period(jd, &scanner)` uses Lập Xuân-based boundary, not Jan 1.
**How to avoid:** Include 2026-01-15 (pre-Lập-Xuân 2026 → Vận 8) and 2026-02-10 (post-Lập-Xuân 2026 → Vận 9) as explicit test cases. Assert their period numbers.
**Warning signs:** Smoke test asserting `period.van == 9` on a Jan 2026 date will fail.

## Code Examples

Verified patterns from existing sources:

### Adding Serde to DaySnapshot (INT-01/05)
```rust
// Source: src/lib.rs — add to existing struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaySnapshot {
    pub ruleset_id: String,
    // ... existing fields ...
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flying_stars: Option<FlyingStarsSummary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applicable_rituals: Option<Vec<String>>,
}
```

### NodeConcept Extension (INT-03) — ontology.rs
```rust
// Source: src/semantic_graph/ontology.rs — extend all four locations
pub enum NodeConcept {
    // ... existing variants ...
    Ritual,      // ← new
    FlyingStar,  // ← new
}

// label() match arm additions:
Self::Ritual => ConceptLabel::Ritual,
Self::FlyingStar => ConceptLabel::FlyingStar,

// ConceptLabel enum additions:
Ritual,
FlyingStar,
PrescribedFor,
OccupiesPalace,
CarriesElement,

// ConceptLabel::as_str() additions:
Self::Ritual => "ritual",
Self::FlyingStar => "flying_star",
Self::PrescribedFor => "prescribed_for",
Self::OccupiesPalace => "occupies_palace",
Self::CarriesElement => "carries_element",

// GraphOntology::node_concepts() slice — add Ritual, FlyingStar
// GraphOntology::edge_concepts() slice — add PrescribedFor, OccupiesPalace, CarriesElement
```

### Multi-Source Direction Provenance (INT-04)
```rust
// Source: src/semantic_graph/builders/day_snapshot.rs — extend add_travel_direction_fact
let huyen_khong_prov = ProvenanceEntry::almanac_rule(
    SOURCE_HUYEN_KHONG,
    "phi_tinh.direction_overlap",
);

let node = SemanticNode::new(
    SemanticId::new("direction", format!("travel:day:{}:all", self.tz_suffix)),
    NodeConcept::Direction,
    NodeOrigin::Fact,
    summary,
)
.with_tags(vec![...])
.with_provenance(khcbppt_provenance)    // existing
.with_provenance(huyen_khong_prov);     // new — satisfies INT-04
```

### E2E Smoke Test Date Enumeration (INT-06)
```rust
// Source: pattern from tests/fengshui_invariants.rs + tests/rituals_integration.rs
use amlich_core::almanac::fengshui::{compute_combined_overlay, compute_palace_aspects, TietKhiScanner};
use amlich_core::rituals::find_van_khan_for_snapshot;
use amlich_core::calculate_day_snapshot;
use amlich_core::tietkhi::get_all_tiet_khi_for_year;  // or via TietKhiScanner

fn tiet_khi_dates_2026() -> Vec<(i32, i32, i32)> {
    let scanner = TietKhiScanner::new();
    scanner.terms_for_year(2026)
        .iter()
        .map(|t| { let (d,m,y) = amlich_core::julian::jd_to_date(t.jd); (d,m,y) })
        .collect()
}
```

### Backward Compat Round-Trip (INT-05)
```rust
// Pattern from tests/rituals_integration.rs Test 5
#[test]
fn v14_day_snapshot_round_trips_without_unexpected_fields() {
    let snap = calculate_day_snapshot(10, 2, 2024);  // v1.4-equivalent (no new fields populated)
    let json = serde_json::to_string(&snap).expect("serialize");
    // New optional fields must NOT appear in JSON when None
    assert!(!json.contains("flying_stars"), "flying_stars must not appear when None");
    assert!(!json.contains("applicable_rituals"), "applicable_rituals must not appear when None");
    // Round-trip must be byte-equal
    let snap2: DaySnapshot = serde_json::from_str(&json).expect("deserialize");
    let json2 = serde_json::to_string(&snap2).expect("re-serialize");
    assert_eq!(json, json2, "DaySnapshot round-trip not byte-equal");
}
```

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust built-in `#[test]` via `cargo test` |
| Config file | `Cargo.toml` (no separate test config) |
| Quick run command | `cargo test -p amlich-core 2>&1 \| tail -5` |
| Full suite command | `cargo test -p amlich-core -- --include-ignored 2>&1 \| tail -20` |

### Phase Requirements → Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| INT-01 | `DaySnapshot.flying_stars: Option<FlyingStarsSummary>` present and serializes | black-box integration | `cargo test -p amlich-core --test day_snapshot_v14_compat` | ❌ Wave 0 |
| INT-02 | `DaySnapshot.applicable_rituals: Option<...>` present and serializes | black-box integration | same test file | ❌ Wave 0 |
| INT-03 | `NodeConcept::Ritual` + `NodeConcept::FlyingStar` + 3 edge concepts present | unit (compile + graph builder test) | `cargo test -p amlich-core 2>&1 \| grep ontology` | ❌ Wave 0 |
| INT-04 | Direction node has both khcbppt + huyen-khong provenance | black-box integration | new test in graph builder or smoke test file | ❌ Wave 0 |
| INT-05 | v1.4 fixture round-trip byte-equal, no unexpected fields | black-box integration | `cargo test -p amlich-core --test day_snapshot_v14_compat` | ❌ Wave 0 |
| INT-06 | 2026 E2E smoke on ≥30 dates: Tết, Sóc/Vọng×12, Vận boundary, leap month, 24 Tiết Khí | E2E smoke | `cargo test -p amlich-core --test integration_2026_smoke` | ❌ Wave 0 |

### Sampling Rate
- **Per task commit:** `cargo test -p amlich-core 2>&1 | tail -5`
- **Per wave merge:** `cargo test -p amlich-core 2>&1 | tail -20`
- **Phase gate:** Full suite green before `/gsd:verify-work`

### Wave 0 Gaps
- [ ] `crates/amlich-core/tests/day_snapshot_v14_compat.rs` — covers INT-01, INT-02, INT-05
- [ ] `crates/amlich-core/tests/integration_2026_smoke.rs` — covers INT-06 (and partial INT-04)
- [ ] Serde derives on `DaySnapshot`, `DayContext`, `SolarDate`, `CanChiSet` in `src/lib.rs` (prerequisite for all DTO tests)

## Key Technical Facts

### DaySnapshot serde gap
`DaySnapshot` at `src/lib.rs:133` currently derives only `Debug, Clone`. To add serde fields for INT-01/02/05, the full type chain must gain serde:
- `DaySnapshot` → needs `Serialize, Deserialize`
- `DayContext` at `src/lib.rs:122` → needs `Serialize, Deserialize`
- `SolarDate` at `src/lib.rs:107` → needs `Serialize, Deserialize`
- `CanChiSet` at `src/lib.rs:115` → needs `Serialize, Deserialize`
- `CanChi` in `src/types.rs` — already derives `Debug, Clone, PartialEq`; needs serde
- `LunarDate` in `src/lunar.rs` — need to verify
- `SolarTerm` in `src/tietkhi.rs` — need to verify
- `GioHoangDao` in `src/gio_hoang_dao.rs` — need to verify
- `DayFortune` in `src/almanac/types.rs:311` — **already derives serde**

The planner should include a serde-derive sweep of the DayContext chain as Wave 0.

### SemanticGraph::add_node collision behavior
`src/semantic_graph/graph.rs:19`: `self.nodes.insert(node.node_id.clone(), node)` — HashMap insert replaces on collision. The Direction node provenance strategy must append both source entries in a single construction, not via a second `add_node` call.

### GraphOntology static slices — not compiler-checked
`GraphOntology::node_concepts()` and `GraphOntology::edge_concepts()` in `ontology.rs:277-345` return hand-maintained `&'static [NodeConcept]` and `&'static [EdgeConcept]` slices. New variants added to the enums are NOT automatically included. The planner must add a test asserting `GraphOntology::node_concepts().contains(&NodeConcept::Ritual)` etc.

### Source-ID guard is active
`tests/source_id_guard.rs` walks `src/` and rejects bare string literals matching source IDs outside `sources.rs`. Any new builder code for Phase 15 must use `SOURCE_HUYEN_KHONG` and `SOURCE_VN_FOLK_RITUAL` constants from `crate::sources`.

### Phi Tinh wiring constraint
Per ROADMAP Cross-Cutting Constraints and PITFALLS CRIT-3: Phase 15 must NOT wire `FlyingStar` nodes into `interaction/direction_merge.rs`. The only permitted Phi Tinh wiring is:
1. The new `flying_stars` field on `DaySnapshot`
2. `NodeConcept::FlyingStar` + three new edge concepts in `ontology.rs`
3. A new builder sub-function `add_flying_star_facts()` in `builders/day_snapshot.rs`
4. Multi-source provenance on the existing `Direction` node

`direction_merge.rs` must remain untouched.

### 2026 Tết date
Tết Nguyên Đán 2026 = 2026-02-17 (solar), lunar 1/1 Bính Ngọ year. Confirm via `calculate_day_snapshot(17, 2, 2026)` → `snapshot.context.lunar.day == 1 && snapshot.context.lunar.month == 1`.

### 2026 Leap Month
Per `generate_golden.rs` comments and the corpus: 2026 has leap lunar month 6 (solar: late July–late August 2026). The `generate_golden.rs` file shows `add(day, 6, 2026)` for day 1..28 around June-July solar — confirm the actual leap month via `convert_solar_to_lunar` and `lunar.is_leap`.

### Vận 8→9 boundary in 2026
2026 Lập Xuân ≈ 2026-02-04. Dates in January 2026 are in solar year 2026 but before Lập Xuân → `compute_period` returns Vận 8 (since effective year = 2025 < 2024... wait: Vận 9 starts at Lập Xuân 2024, so 2025 and 2026 are both Vận 9). **Correction:** Vận 8 ended at Lập Xuân 2024. By 2026 the transition is fully in Vận 9. The "Vận 8→9 transition dates" referenced in INT-06 most likely means: include dates around the original 2024 transition (e.g., 2024-02-03 and 2024-02-05), which the smoke test can include alongside 2026 dates, OR include dates from 2023-2024 boundary. The ROADMAP says "2026 smoke test" but also "Vận 8→9 transition dates" — the test must include dates from both sides of Lập Xuân 2024 to exercise the boundary, even if the bulk of dates are in 2026.

## Open Questions

1. **`FlyingStarsSummary` DTO shape for INT-01**
   - What we know: it must be `Option<T>` on `DaySnapshot`; `CombinedFlyingStarLayout` exists but is large (nested layouts)
   - What's unclear: Should the planner define a slim summary DTO or embed `CombinedFlyingStarLayout` directly?
   - Recommendation: Define a new `FlyingStarsSummary` struct that contains only `period: Period`, `center_star: FlyingStar`, and `palace_overlays: [(FlyingStar, FlyingStar); 9]` — enough for inspection without the full nested layout. Full layout accessible via `compute_combined_overlay` directly.

2. **`applicable_rituals` DTO shape for INT-02**
   - What we know: `find_van_khan_for_snapshot(&snap)` returns `Vec<&'static RitualEntry>` — cannot embed statics in the DTO
   - What's unclear: Should the field be `Option<Vec<String>>` (ritual_ids only) or `Option<Vec<RitualEntrySummary>>` (clone of key fields)?
   - Recommendation: `Option<Vec<String>>` (ritual_ids) for INT-02 — minimal footprint, callers can look up full entries via `get_ritual_by_id`. If the planner wants richer data, a `RitualSummary { ritual_id: String, event_keys: Vec<RitualEventKey> }` is reasonable.

3. **`DayContext` serde chain — which types already have serde?**
   - What we know: `DayFortune` (in `almanac/types.rs`) has serde; `DaySnapshot`/`DayContext`/`SolarDate`/`CanChiSet` do not.
   - What's unclear: `LunarDate`, `SolarTerm`, `GioHoangDao` — likely have serde (they are returned by APIs that serialize), but not confirmed by reading.
   - Recommendation: The planner should audit these three types' derive macros before writing the serde-sweep task.

4. **How to populate `flying_stars` in `calculate_day_snapshot`**
   - What we know: `calculate_day_snapshot_internal` in `src/lib.rs` constructs `DaySnapshot`; `TietKhiScanner::new()` is cheap to construct.
   - What's unclear: Should `flying_stars` be populated by default (always compute) or only on request?
   - Recommendation: Populate by default — `compute_combined_overlay(year, month, &TietKhiScanner::new())` is deterministic and fast; no new function parameter needed. Consistent with how `daily_recommendations` is always computed.

## Sources

### Primary (HIGH confidence)
- Direct code inspection — `src/semantic_graph/ontology.rs` — NodeConcept, EdgeConcept, ConceptLabel, GraphOntology
- Direct code inspection — `src/semantic_graph/builders/day_snapshot.rs` — builder pattern, provenance usage
- Direct code inspection — `src/semantic_graph/node.rs`, `edge.rs`, `provenance.rs` — data model
- Direct code inspection — `src/lib.rs` — DaySnapshot struct (lines 133–142)
- Direct code inspection — `src/almanac/fengshui/mod.rs`, `combined.rs`, `types.rs` — public API surface
- Direct code inspection — `src/rituals/mod.rs`, `schema.rs` — ritual public API
- Direct code inspection — `src/sources.rs` — SOURCE_* constants
- Direct code inspection — `tests/rituals_integration.rs` — external-consumer integration test pattern
- Direct code inspection — `tests/fengshui_invariants.rs`, `tests/fengshui_aspects.rs` — smoke/invariant test patterns

### Secondary (MEDIUM confidence)
- `.planning/STATE.md` + `.planning/ROADMAP.md` — cross-cutting constraints and wiring rules
- `.planning/v1.5-MILESTONE-AUDIT.md` — INT-01..INT-06 current gap status

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — no new dependencies; all patterns verified from existing code
- Architecture: HIGH — ontology extension pattern is unambiguous; DTO additive pattern verified from multiple precedents
- Pitfalls: HIGH — Pitfalls 1–4 are directly observed from code structure; Pitfall 5 is verified from period.rs boundary logic

**Research date:** 2026-05-28
**Valid until:** 2026-06-28 (30 days — codebase is stable; fengshui and ritual APIs are locked)
