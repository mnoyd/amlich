# Architecture Research

**Domain:** Rust Vietnamese-almanac engine (`amlich-core`) — extending an existing multi-tradition reasoning system with the P2 Kinh Dịch (I-Ching) pillar and a Thái Tuế/Tam Sát ⇄ Phi Tinh read-only cross-link.
**Researched:** 2026-07-16
**Confidence:** HIGH (every recommendation is grounded in existing v1.5/v1.6 code paths with file:line refs; the v1.5 Văn khấn and Phi Tinh pillars provide direct precedent for both the corpus-driven and algorithm-driven integration patterns this milestone needs).

---

## Standard Architecture

### System Overview

v1.7 does **not** introduce a new layer. It adds two new leaves into the existing layer cake, then threads them through the established additive surfaces (`DaySnapshot`, semantic graph, reasoning envelope). The diagram below shows where the new code lands — every box marked **NEW** slots into an existing layer without modifying layer boundaries.

```
┌──────────────────────────────────────────────────────────────────────────┐
│  PUBLIC API (lib.rs ~ lines 224-272)                                     │
│   calculate_day_snapshot  /  build_initiation_opening_reasoning_bundle   │
│   + NEW: cast_iching(snapshot, IChingQuery)  +  build_direction_cross_link│
├──────────────────────────────────────────────────────────────────────────┤
│  REASONING LAYER  (crates/amlich-core/src/reasoning/)                    │
│   ┌────────────────┐ ┌──────────────────┐ ┌──────────────────────────┐   │
│   │ personal.rs    │ │ initiation_      │ │ NEW: direction_composite │   │
│   │ (build_fact_   │ │ opening_         │ │ .rs  (Thai Tue ⇄ Phi     │   │
│   │  nodes)        │ │ evaluator.rs     │ │  Tinh cross-link,        │   │
│   │                │ │ (ActionEvaluator │ │  read-only)              │   │
│   │                │ │  precedent)      │ │                          │   │
│   └────────────────┘ └──────────────────┘ └──────────────────────────┘   │
│   ┌──────────────────────────────────────────────────────────────────┐   │
│   │ NEW: iching/  (mod, schema, corpus, mai_hoa, bien_que, evaluator)│   │
│   │  impl ActionEvaluator for IChingEvaluator                        │   │
│   └──────────────────────────────────────────────────────────────────┘   │
├──────────────────────────────────────────────────────────────────────────┤
│  ALMANAC LAYER  (crates/amlich-core/src/almanac/)                        │
│   ┌─────────────────┐ ┌──────────────────┐ ┌─────────────────────────┐   │
│   │ thai_tue.rs     │ │ sat_phuong.rs    │ │ fengshui/  (Phi Tinh)   │   │
│   │ (KHCBPPT,       │ │ (KHCBPPT,        │ │ (huyen-khong, CRIT-3    │   │
│   │  DEC-0021)      │ │  DEC-0018)       │ │  isolated from          │   │
│   │ *evidence =     │ │ *evidence =      │ │  direction_merge.rs)    │   │
│   │  None ← BACKFILL│ │  None ← BACKFILL │ │                         │   │
│   └─────────────────┘ └──────────────────┘ └─────────────────────────┘   │
│   *MODIFIED (1-line backfill): populate RuleEvidence with SOURCE_KHCBPPT  │
├──────────────────────────────────────────────────────────────────────────┤
│  SEMANTIC GRAPH  (crates/amlich-core/src/semantic_graph/)                │
│   ontology.rs (NodeConcept/EdgeConcept 6-slice pattern)                  │
│   builders/day_snapshot.rs (+ NEW add_iching_facts builder method)       │
│   + NEW: Hexagram node concept, LocatedAt + Transforms edge concepts     │
├──────────────────────────────────────────────────────────────────────────┤
│  DTO  (crates/amlich-core/src/lib.rs:154 DaySnapshot)                    │
│   + NEW additive Option<IChingCastSummary> field                        │
│   + NEW additive Option<DirectionCrossLinkSummary> field                │
├──────────────────────────────────────────────────────────────────────────┤
│  CORPUS  (crates/amlich-core/data/)                                      │
│   data/iching/hexagrams.json   (64 entries, $schema_version "iching-v1") │
│   data/schemas/iching-schema.json                                        │
└──────────────────────────────────────────────────────────────────────────┘
```

### Component Responsibilities

| Component | Responsibility | Typical Implementation |
|-----------|----------------|------------------------|
| `reasoning/iching/` (NEW) | Cast hexagram from query time, resolve 64-quẻ table, derive biến quẻ, surface interpretation. Self-contained pillar. | Sibling of `rituals/` (corpus+lookup precedent) and `almanac/fengshui/` (algorithm precedent). |
| `reasoning/direction_composite.rs` (NEW) | Read-only join of `khcbppt` Thai-Tuế/Sát-Phương with `huyen-khong` palace layout into one directional picture. | Single-purpose reasoning module; emits composite `rule.composite.*` evidence; never mutates either source. |
| `reasoning/personal.rs` (MODIFIED, additive) | Branch dispatch for IChing intent + new direction-cross-link fact node. | New methods on existing `impl PersonalReasoningInput`; no signature changes to existing methods. |
| `almanac/thai_tue.rs`, `almanac/sat_phuong.rs` (MODIFIED, 1-line each) | Backfill `RuleEvidence { source_id: SOURCE_KHCBPPT, ... }` so the cross-link can cite them. | Currently `evidence: None` — populate via `.to_string()` from the `SOURCE_*` constant per CI guard. |
| `semantic_graph/ontology.rs` (MODIFIED) | Add `Hexagram` node + `LocatedAt`, `Transforms` edges per the 6-slice pattern. | Six mechanical edits (see Pattern 1 below). |
| `sources.rs` (MODIFIED) | Register `SOURCE_KINH_DICH`, `SOURCE_MAI_HOA_DICH_SO`. | Two new `pub const` lines; CI guard FORBIDDEN_LITERALS extended. |
| `reasoning/types.rs` (MODIFIED) | Add `ReasoningEvidenceSourceFamily::IChing` + `ActionId::IChing`. | Closed-enum additive extension; exhaustive match sites in `initiation_opening_evaluator.rs` updated. |

---

## Recommended Project Structure

```
crates/amlich-core/
├── data/
│   └── iching/                          # NEW — 64-hexagram corpus (mirrors data/rituals/)
│       ├── hexagrams.json               # 64 entries, $schema_version "iching-v1"
│       ├── manifest.json                # file list for tooling (NOT parsed at runtime, per 12-03)
│       └── provenance_audit.md          # classical reference + reviewer ledger (RIT-11 precedent)
└── src/
    ├── sources.rs                       # MODIFIED — +SOURCE_KINH_DICH, +SOURCE_MAI_HOA_DICH_SO
    ├── lib.rs                           # MODIFIED — +additive DaySnapshot fields, +public API fns
    ├── advisory.rs                      # UNCHANGED — ConsultationIntent stays Copy (see Pattern 3)
    ├── almanac/
    │   ├── thai_tue.rs                  # MODIFIED (1 line) — populate evidence.source_id
    │   └── sat_phuong.rs                # MODIFIED (1 line) — populate evidence.source_id
    ├── reasoning/
    │   ├── mod.rs                       # MODIFIED — pub use iching::* ; pub use direction_composite
    │   ├── personal.rs                  # MODIFIED — new build_iching_fact_nodes + cross-link fact
    │   ├── types.rs                     # MODIFIED — +IChing source family, +ActionId::IChing
    │   ├── iching/                      # NEW — Tier-0 divination pillar
    │   │   ├── mod.rs                   # public API: cast_iching, all_hexagrams, lookup_hexagram
    │   │   ├── schema.rs                # ADR-locked HexagramEntry + HexagramLine + IChingCastSummary
    │   │   ├── corpus.rs                # OnceLock<Vec<HexagramEntry>> loader + NFC + source_id check
    │   │   ├── mai_hoa.rs               # cast_hexagram_mai_hoa(query_time) -> HexagramCast
    │   │   ├── bien_que.rs              # derive_transformed_hexagram(cast) -> Hexagram (biến quẻ)
    │   │   ├── evaluator.rs             # impl ActionEvaluator for IChingEvaluator
    │   │   └── golden.rs                # 10+ golden cases vs nhantu.net/divination.com (§7)
    │   └── direction_composite.rs       # NEW — Thai-Tuế ⇄ Phi Tinh read-only join
    └── semantic_graph/
        ├── ontology.rs                  # MODIFIED — 6-slice Hexagram + LocatedAt + Transforms
        └── builders/
            └── day_snapshot.rs          # MODIFIED — +add_iching_facts(), +add_direction_composite_facts()

.planning/
├── adrs/
│   ├── 0005-iching-schema-v1.md         # NEW — locks HexagramEntry field set
│   ├── 0006-mai-hoa-casting-algorithm.md # NEW — time-number method, động hào derivation
│   └── 0007-thai-tue-phi-tinh-cross-link.md # NEW — formalizes CRIT-3 carve-out for reasoning layer
└── research/
    └── ARCHITECTURE.md                  # THIS FILE
```

### Structure Rationale

- **`reasoning/iching/` (not top-level `iching/`):** EXPANSION_FRAMEWORK §2.2 explicitly directs placement under `reasoning/`. Unlike `rituals/` (pure content+lookup, no inference) and `almanac/fengshui/` (algorithm-only, no consultation semantics), IChing spans both: it has a 64-row corpus AND a casting algorithm AND a consultation evaluator. The reasoning layer is the only layer that owns all three concerns. The sub-module pattern keeps the public API surface narrow (`pub use iching::cast_iching`, etc.) and isolates corpus-loading cost from cold-start of other pillars.
- **`direction_composite.rs` (not `interaction/`):** CRIT-3 isolation (`tests/fengshui_crit3_isolation.rs`) explicitly forbids FlyingStar references inside `interaction/`. The cross-link needs to *read* both `almanac::fengshui` and `almanac::thai_tue`, so it cannot live under `interaction/`. The reasoning layer is the only layer that already imports both families (`personal.rs:1-9` already pulls `almanac::tu_menh`, `bazi`, `interaction::*`). Placing it under `reasoning/` keeps the CRIT-3 boundary at the `interaction/` ↔ `almanac/fengshui/` interface, where the grep guard already enforces it.
- **No new top-level crate:** All new code lives inside `amlich-core`. The boundary discipline is module-level, not crate-level — consistent with v1.5/v1.6 precedent.

---

## Architectural Patterns

### Pattern 1: 6-Slice Ontology Extension (mandatory for semantic-graph additions)

**What:** Every new `NodeConcept` or `EdgeConcept` requires coordinated edits in **six** locations inside `semantic_graph/ontology.rs`. The compiler enforces three of them via `match` exhaustiveness; the runtime enforces the other three via slice-membership tests.

**When to use:** Whenever EXPANSION_FRAMEWORK §3.2 mandates a new node/edge (Hexagram, LocatedAt, Transforms are all in scope for v1.7).

**The six slices (file:line refs):**
1. `enum NodeConcept { ... }` (`ontology.rs:5-43`) — add variant `Hexagram`.
2. `impl NodeConcept { pub fn label(&self) -> ConceptLabel { match self { ... } } }` (`ontology.rs:45-87`) — add `Self::Hexagram => ConceptLabel::Hexagram` arm. **Compiler-enforced.**
3. `enum ConceptLabel { ... }` (`ontology.rs:159-228`) — add variant `Hexagram`.
4. `impl ConceptLabel { pub fn as_str(&self) -> &'static str { match self { ... } } }` (`ontology.rs:230-301`) — add `Self::Hexagram => "hexagram"` arm. **Compiler-enforced.**
5. `impl GraphOntology { pub fn node_concepts() -> &'static [NodeConcept] { &[ ... ] } }` (`ontology.rs:336-377`) — add `NodeConcept::Hexagram` to the slice. **Runtime test enforces** (`v15_concepts_present_in_ontology_slices` at `ontology.rs:310`).
6. (Mirror 1-5 for each new edge concept — `LocatedAt`, `Transforms` — across `EdgeConcept`, `EdgeConcept::label`, `ConceptLabel`, `ConceptLabel::as_str`, `edge_concepts()` slice at `ontology.rs:379-411`.)

**Trade-offs:** Tedious but bullet-proof. Once added, the concept participates in every graph-projection, selector, and reasoning-evaluator automatically (no other code needs to learn about it — it surfaces via `find_node_by_concept(graph, NodeConcept::Hexagram)` like the existing `find_node_by_concept(graph, NodeConcept::Truc)` at `initiation_opening_evaluator.rs:32`).

**Example** (the only idiomatic shape — anything else fails CI):
```rust
// ontology.rs — six edits per concept, no shortcuts
pub enum NodeConcept {
    // ... existing 38 variants ...
    Hexagram,         // ← slice 1
}

impl NodeConcept {
    pub fn label(&self) -> ConceptLabel {
        match self {
            // ...
            Self::Hexagram => ConceptLabel::Hexagram,   // ← slice 2
        }
    }
}

pub enum ConceptLabel {
    // ...
    Hexagram,         // ← slice 3
}

impl ConceptLabel {
    pub fn as_str(&self) -> &'static str {
        match self {
            // ...
            Self::Hexagram => "hexagram",               // ← slice 4
        }
    }
}

impl GraphOntology {
    pub fn node_concepts() -> &'static [NodeConcept] {
        &[
            // ... existing 38 entries ...
            NodeConcept::Hexagram,                       // ← slice 5
        ]
    }
}
// Slice 6: a new test `v17_concepts_present_in_ontology_slices` mirroring
// `v16_concepts_present_in_ontology_slices` at ontology.rs:324.
```

### Pattern 2: Schema-Lock-Before-Corpus (v1.5 Phase 10 precedent, mandatory)

**What:** Lock the corpus JSON schema (`#[serde(deny_unknown_fields)]` types + ADR) **before** authoring any data file. Re-editing N corpus entries after a schema slip costs O(N); locking first costs O(1).

**When to use:** ANY time the milestone introduces a JSON corpus with ≥10 entries. v1.7 has 64 hexagrams — non-negotiable.

**Trade-offs:** Slows Phase 1 by ~2 days (ADR writing + type stubs + serde round-trip tests). Saves weeks of corpus re-authoring and prevents the CRIT-1/CRIT-5 failures documented in `pitfalls` research.

**Example** (the exact v1.5 sequence, applied to IChing):
```rust
// reasoning/iching/schema.rs — locked FIRST, before hexagrams.json exists
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]                 // ← ADR-0005 will reference this attribute
pub struct HexagramEntry {
    pub number: u8,                            // 1..=64 (King Wen sequence)
    pub name_vi: String,                       // NFC-normalized at load
    pub name_pinyin: String,
    pub binary: [u8; 6],                       // bottom line first; 0=yin 1=yang
    pub truc_from: u8,                         // kinh-dich tradition: Ngô Tất Tố thoán từ
    pub thoan_tu_vi: String,                   // 列傳 — Judgment text
    pub hao_tu_vi: Vec<HaoTu>,                 // 6 line texts (or fewer for symmetrical quẻ)
    pub cat_hung: CatHungTier,                 // Closed enum
    pub source_id: String,                     // Always "kinh-dich" (loader-validated)
    pub original_citation: SourceCitation,     // re-export from rituals::schema
    pub confidence: RitualConfidenceTier,      // re-export (closed enum)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CatHungTier { Catt, Hung, TrungBinh } // closed enum — unknown values fail
```

Then the loader (Phase 3) follows the `OnceLock + include_str!` pattern verbatim from `rituals/corpus.rs:85-117`, asserting `$schema_version == "iching-v1"` and `entry.source_id == SOURCE_KINH_DICH` per the source_id discipline.

### Pattern 3: Sibling-Newtype for Payload-Carrying Intents (avoiding Copy-enum breakage)

**What:** `ConsultationIntent` (`advisory.rs:18-30`) derives `Copy + Eq`. EXPANSION_FRAMEWORK §2.2 specifies `ConsultationIntent::IChing { question }`, but `String` is not `Copy`. **Naively following the framework would require removing `Copy` from `ConsultationIntent` — a cascading break across ~25 call-sites** (every `match intent { ... }` would need `&intent` or `intent.clone()`).

**Recommendation: do NOT extend `ConsultationIntent`.** Instead, introduce a sibling newtype that lives alongside it, mirroring how `DailyFlyingStarLayout` was added as a sibling to `FlyingStarLayout` in v1.6 (`almanac/fengshui/types.rs:136-143`) rather than mutating the frozen v1 layout.

**When to use:** Any new "consultation" surface that carries payload (question text, spatial input, etc.). Existing `ConsultationIntent` stays the closed 9-variant lookup table it is.

**Trade-offs:** Two parallel dispatch surfaces (`ConsultationIntent` for activity-mapping, `IChingQuery` for divination). Cleaner than the alternative because:
- `ConsultationIntent::primary_activity()` returns `ActivityId` — IChing has no `ActivityId` mapping (divination is not an "activity" in the recommendation engine).
- `ConsultationIntent::event_kind()` returns a slug consumed by `data/holidays/*.json` join — IChing has no event-key join.
- The ActionEvaluator trait (`reasoning/action_evaluator.rs:51`) already supports multiple evaluators side-by-side via `InitiationOpeningEvaluator`; an `IChingEvaluator` is the natural second instance.

**Example:**
```rust
// reasoning/iching/evaluator.rs
#[derive(Debug, Clone, PartialEq, Eq)]   // NO Copy — carries String payload
pub struct IChingQuery {
    pub question_vi: String,              // NFC-normalized at construction
    pub query_time: SolarDate,            // sets the hexagram via Mai Hoa time-number method
}

pub struct IChingEvaluator;
impl ActionEvaluator for IChingEvaluator {
    fn action_id(&self) -> ActionId { ActionId::IChing }            // NEW variant
    fn select_subgraph(&self, graph, snapshot, _personal) -> Result<_, String> {
        // select Hexagram nodes + Truc/DayDeity support nodes
    }
    fn evaluate(&self, graph, snapshot, personal) -> Result<ActionEvaluation, String> {
        // returns ActionEvaluation with bucket = Auspicious/Inauspicious based on cat_hung
    }
}
```

### Pattern 4: Composite-Envelope Multi-Source Provenance (for the Thái Tuế ⇄ Phi Tinh cross-link)

**What:** When a reasoning-layer fact joins two traditions, emit **multiple** `ReasoningEvidenceEnvelope` entries on the same `PersonalFactNode`, each carrying the **distinct** `source_id` of its contributing tradition, plus ONE composite envelope with `source_id: "rule.composite.<topic>"` per EXPANSION_FRAMEWORK §3.2. This is exactly the pattern already used by `add_offering_facts` at `semantic_graph/builders/day_snapshot.rs:697-744` (the INT-09 dual-source `RecommendsOffering` edge).

**When to use:** The Thái Tuế ⇄ Phi Tinh cross-link. CRIT-3 isolation **forbids** merging the traditions at the data layer (no shared `source_id`); the join must be visible only at the reasoning-envelope layer, with each tradition's contribution independently citable.

**Trade-offs:** Slightly more verbose than a single merged envelope. Buys auditability: a downstream consumer can ask "what did KHCBPPT contribute vs. what did Huyền Không contribute?" and get two distinct answers. This is the **only** pattern compatible with the CRIT-3 grep guard.

**Example:**
```rust
// reasoning/direction_composite.rs
use crate::sources::{SOURCE_HUYEN_KHONG, SOURCE_KHCBPPT};

pub fn build_direction_cross_link(
    snapshot: &DaySnapshot,
    birth_chi_index: usize,
) -> PersonalFactNode {
    // READ huyen-khong (FlyingStar) — never mutate
    let fs = snapshot.flying_stars.as_ref();
    // READ khcbppt (Thai Tue + Sat Phuong) — never mutate
    let thai_tue = crate::almanac::thai_tue::compute_thai_tue(
        birth_chi_index,
        snapshot.context.canchi.year.chi_index,
    );
    let sat_phuong = crate::almanac::sat_phuong::get_sat_phuong(
        snapshot.context.canchi.day.chi_index,
    );

    PersonalFactNode {
        id: "fact.composite.direction_cross_link".to_string(),
        summary_vi: format!(
            "Thái Tuế={:?} | Sát Phương={} | Phi Tinh trung cung={:?}",
            thai_tue.conflicts.iter().map(|c| c.kind).collect::<Vec<_>>(),
            sat_phuong.direction,
            fs.map(|f| f.center_star),
        ),
        severity: None,
        evidence: vec![
            // Distinct KHCBPPT evidence (Thai Tue + Sat Phuong)
            ReasoningEvidenceEnvelope {
                source_family: ReasoningEvidenceSourceFamily::AlmanacRule,
                source_id: SOURCE_KHCBPPT.to_string(),
                method: "thai_tue+sat_phuong".to_string(),
                note: Some(format!("thai_tue_conflicts={};sat_phuong={}",
                    thai_tue.conflicts.len(), sat_phuong.direction)),
            },
            // Distinct huyen-khong evidence (Phi Tinh)
            ReasoningEvidenceEnvelope {
                source_family: ReasoningEvidenceSourceFamily::AlmanacRule,
                source_id: SOURCE_HUYEN_KHONG.to_string(),
                method: "phi_tinh.palace_layout".to_string(),
                note: fs.map(|f| format!("van={};center={:?}", f.van, f.center_star)),
            },
            // Composite envelope — joins both, prefixed per §3.2
            ReasoningEvidenceEnvelope {
                source_family: ReasoningEvidenceSourceFamily::Derived,
                source_id: "rule.composite.direction_cross_link".to_string(),
                method: "v17.read_only_join".to_string(),
                note: Some("read-only join of khcbppt directionals + huyen-khong palace".into()),
            },
        ],
    }
}
```

The function signature uses **only `&` references** to the almanac layer — this is what makes CRIT-3 preservation mechanically checkable (see Anti-Pattern 2 below).

---

## Data Flow

### Request Flow — IChing cast (the 64-hexagram pipeline)

```
[User: cast_iching(date, question)]
    ↓
lib.rs::cast_iching()
    ↓
IChingQuery::new(question, snapshot.context.solar)
    ↓
 ┌───────────────────────────────────────────────────────────────────────┐
 │ reasoning/iching/mai_hoa.rs::cast_hexagram_mai_hoa(query_time)        │
 │   1. Extract lunar year/month/day + chi-tý hour from SolarDate        │
 │   2. Upper trigram = (year_branch + month + day) % 8                  │
 │   3. Lower trigram = (above + hour_chi) % 8                           │
 │   4. Động hào = (year+month+day+hour) % 6                            │
 │   Returns: HexagramCast { primary_number, moving_line, time_meta }    │
 └───────────────────────────────────────────────────────────────────────┘
    ↓
 ┌───────────────────────────────────────────────────────────────────────┐
 │ reasoning/iching/corpus.rs::get_hexagram_by_number(primary_number)    │
 │   Looks up OnceLock<Vec<HexagramEntry>> (loaded from                   │
 │   data/iching/hexagrams.json via include_str!)                         │
 │   Returns: &'static HexagramEntry (thoán từ, hào từ, cat_hung)        │
 └───────────────────────────────────────────────────────────────────────┘
    ↓
 ┌───────────────────────────────────────────────────────────────────────┐
 │ reasoning/iching/bien_que.rs::derive_transformed_hexagram(cast)       │
 │   Flip the động hào in primary.binary, re-lookup → biến quẻ            │
 │   Returns: Hexagram { entry: &'static HexagramEntry, relation: ... }   │
 └───────────────────────────────────────────────────────────────────────┘
    ↓
 ┌───────────────────────────────────────────────────────────────────────┐
 │ reasoning/iching/evaluator.rs::evaluate()                              │
 │   impl ActionEvaluator for IChingEvaluator                             │
 │   Emits ActionEvaluation { bucket: Favorable|Avoid|... based on        │
 │   primary.cat_hung + biến quẻ cat_hung delta }                         │
 │   Evidence: ReasoningEvidenceEnvelope {                                 │
 │     source_family: IChing,                                             │
 │     source_id: "mai-hoa-dich-so" (casting) + "kinh-dich" (text),      │
 │     method: "mai_hoa_time_number+bien_que",                            │
 │   }                                                                    │
 └───────────────────────────────────────────────────────────────────────┘
    ↓
 ┌───────────────────────────────────────────────────────────────────────┐
 │ lib.rs::calculate_day_snapshot_internal                                │
 │   Populates additive DaySnapshot.iching_cast:                          │
 │     Option<IChingCastSummary>                                          │
 │   ONLY when query is provided — absent in JSON when None               │
 └───────────────────────────────────────────────────────────────────────┘
    ↓
 ┌───────────────────────────────────────────────────────────────────────┐
 │ semantic_graph/builders/day_snapshot.rs::add_iching_facts()            │
 │   - Hexagram node for primary quẻ (NodeConcept::Hexagram)              │
 │   - Hexagram node for biến quẻ (if động hào)                           │
 │   - EdgeConcept::Transforms: primary → biến                            │
 │   - EdgeConcept::LocatedAt: primary hexagram → day_root                │
 │   - EdgeConcept::Composes: day_root → primary hexagram                 │
 │   Provenance: SOURCE_MAI_HOA_DICH_SO (cast) + SOURCE_KINH_DICH (text)  │
 └───────────────────────────────────────────────────────────────────────┘
    ↓
[Response: DaySnapshot + reasoning graph with Hexagram nodes]
```

### Request Flow — Thái Tuế ⇄ Phi Tinh cross-link

```
[User: build_direction_cross_link(snapshot, birth_year_chi)]
    ↓
reasoning/direction_composite.rs::build_direction_cross_link()
    ↓
 ┌───────────────────────────────────────────────────────────────────────┐
 │ READ-ONLY: snapshot.flying_stars (huyen-khong provenance already set)  │
 │ READ-ONLY: compute_thai_tue(birth_chi, year_chi)  [khcbppt]            │
 │ READ-ONLY: get_sat_phuong(day_chi)                [khcbppt]            │
 │                                                                        │
 │ NONE of these calls mutate. All inputs borrowed by shared reference.   │
 └───────────────────────────────────────────────────────────────────────┘
    ↓
[Composite PersonalFactNode with 3 evidence envelopes (see Pattern 4)]
    ↓
[Optional: DaySnapshot.direction_cross_link: Option<DirectionCrossLinkSummary>]
    ↓
[semantic_graph/builders/day_snapshot.rs::add_direction_composite_facts()]
    ↓
[Response: same DaySnapshot, same graph, with one new composite fact node]
```

### Key Data Flows

1. **Backward-compat round-trip:** v1.6 producers of `DaySnapshot` JSON must deserialize cleanly into v1.7. The additive `Option<T>` fields with `#[serde(default, skip_serializing_if = "Option::is_none")]` (pattern at `lib.rs:163-184`) guarantee this — `serde` ignores missing fields on read and omits them on write when `None`. New v1.7 consumers see the new fields when present.
2. **Source-ID provenance flows end-to-end:** Every fact in the semantic graph carries `ProvenanceEntry` with a `source_id`. The CI guard (`tests/source_id_guard.rs`) ensures no bare string literals appear in `src/` outside `sources.rs`. Adding `SOURCE_KINH_DICH` / `SOURCE_MAI_HOA_DICH_SO` requires extending the `FORBIDDEN_LITERALS` array at `tests/source_id_guard.rs:17-25` — otherwise the new constants would be the only legal form and bare `"kinh-dich"` literals could leak undetected.
3. **CRIT-3 preservation flow:** The cross-link READS from both `almanac/thai_tue` and `almanac/fengshui`, but does so **only** from inside `reasoning/direction_composite.rs`. The CRIT-3 grep guard (`tests/fengshui_crit3_isolation.rs:14-21`) forbids `FlyingStar` references inside `src/interaction/direction_merge.rs` — it does **not** forbid them inside `reasoning/`. The new module lives outside the CRIT-3 quarantine zone while still preserving the boundary at its original location.

---

## Scaling Considerations

| Scale | Architecture Adjustments |
|-------|--------------------------|
| Per-day query (1 snapshot) | All v1.7 work fits in-memory. `OnceLock<Vec<HexagramEntry>>` (64 entries, ~few hundred KB) loads once per process. Mai Hoa casting is O(1) arithmetic. Cross-link is O(1) reads + O(1) envelope construction. |
| Bulk historical scan (2020-2030 = ~4k days) | No architecture change. The 64-hexagram corpus is shared across all days via `OnceLock`. The cross-link is invoked per-day on demand. The only N-scaling concern is the semantic-graph node count per snapshot, which already doubled in v1.5 with FlyingStar + Ritual and stayed well under noisy thresholds. |
| Future: 1M+ queries/day | Out of scope for v1.7. The next scaling concern is caching `compute_combined_overlay` (annual+monthly Phi Tinh) across same-month queries — that is a v1.8+ concern, not v1.7. |

### Scaling Priorities

1. **First bottleneck (not in v1.7 but worth flagging):** `calculate_day_snapshot_internal` (lib.rs:274-360) recomputes the Phi Tinh overlay on every call even for repeat months. The v1.7 cross-link reads this overlay; if v1.8 adds cross-link-per-call auditing, consider memoizing the monthly overlay. No action needed now.
2. **Second bottleneck:** The 64-hexagram corpus is small enough that lazy-loading via `OnceLock` is the right choice forever. Do NOT pre-load at module-init time — that would slow cold-start for callers who never cast a hexagram. The current pattern (`rituals/corpus.rs:94-117` — first-call init) is correct and should be mirrored exactly.

---

## Anti-Patterns

### Anti-Pattern 1: Extending `ConsultationIntent` with a payload-carrying variant

**What people do:** Follow EXPANSION_FRAMEWORK §2.2 literally and add `ConsultationIntent::IChing { question: String }`.
**Why it's wrong:** `ConsultationIntent` derives `Copy` (`advisory.rs:18`). Adding a `String` payload removes `Copy`, which breaks ~25 call-sites: every `match intent { ... }` consumes the value, every function taking `intent: ConsultationIntent` by value now requires `clone()`, every `#[derive(Clone, Copy, PartialEq, Eq)]` on types embedding it (e.g. `advisory.rs:484, 501, 510, 576, 630`) loses `Copy`. The blast radius is large and silent (compile errors mask runtime correctness issues until resolved).
**Do this instead:** Use Pattern 3 above — sibling `IChingQuery` newtype + separate `IChingEvaluator`. The framework's wording was aspirational pseudo-code; the v1.5/v1.6 codebase already establishes the sibling-not-extend pattern (`DailyFlyingStarLayout` sibling to `FlyingStarLayout`).

### Anti-Pattern 2: Crossing CRIT-3 inside `interaction/direction_merge.rs`

**What people do:** Add Thai Tue or Flying Star consumption to the existing `compute_direction_merge()` function (`interaction/direction_merge.rs:28-87`), since that's "where direction logic lives."
**Why it's wrong:** The CI guard at `tests/fengshui_crit3_isolation.rs:14-21` will fail the build. It greps for `FlyingStar`, `DailyFlyingStar`, `DailyFlyingStarLayout`, `almanac::fengshui`, `phi_tinh`, `compute_daily_flying_stars` in `src/interaction/direction_merge.rs`. Even a comment containing one of those tokens fails it. This boundary is the project's most-explicitly-enforced discipline.
**Do this instead:** Put the cross-link in `reasoning/direction_composite.rs` (outside the CRIT-3 quarantine zone). If a future milestone wants to relax CRIT-3 (e.g. v2.0 `spatial_compose` Tier-3 pillar), that requires a new ADR explicitly superseding the CRIT-3 carve-out — never a silent code change.

### Anti-Pattern 3: Skipping the schema-lock phase and authoring corpus directly

**What people do:** Eager to show progress, write `data/iching/hexagrams.json` first using a draft schema, then discover that adding a missing field requires re-editing all 64 entries.
**Why it's wrong:** This is exactly the CRIT-1/CRIT-5 failure mode documented for v1.5 and codified in the "Schema-lock before corpus authoring" decision (`PROJECT.md:79`). 64 entries is 64× the rework cost of v1.5's 60 ritual entries. The schema MUST be locked by an ADR (ADR-0005) and the Rust types MUST be frozen (`#[serde(deny_unknown_fields)]`) before any data file is created.
**Do this instead:** Follow the v1.5 Phase 10 → Phase 12 sequence exactly. Phase 1 = sources + ADRs + locked types + serde round-trip tests on a 1-entry fixture. Phase 3 = 64-entry corpus authoring against the now-immutable schema.

### Anti-Pattern 4: Using a single merged `source_id` for the cross-link

**What people do:** Mint a new `source_id: "thai-tue-phi-tinh"` to label cross-link facts.
**Why it's wrong:** DEC-0015/0016 (`EXPANSION_FRAMEWORK §1`) require per-tradition `source_id` discipline. Minting a hybrid id destroys audit provenance — readers can no longer tell which claim came from which classical text. It also violates the CRIT-3 spirit (the isolation exists precisely so the two traditions' contributions remain distinguishable).
**Do this instead:** Use Pattern 4 above — emit **two** tradition-specific envelopes (`SOURCE_KHCBPPT` + `SOURCE_HUYEN_KHONG`) plus **one** composite envelope with `source_id: "rule.composite.direction_cross_link"` (the `rule.composite.*` prefix is mandated by EXPANSION_FRAMEWORK §3.2 and is what distinguishes derived joins from primitive traditions).

### Anti-Pattern 5: Authoring hexagram text in English first

**What people do:** Populate `thoan_tu_en` eagerly because it's "easier to review."
**Why it's wrong:** RIT-13 precedent (rituals `body_en` reserved, always null in v1.5 corpus, `rituals/schema.rs:228-230`) establishes that English translations for cultural text are deferred indefinitely. The discipline protects against half-translated corpora that look "complete" but aren't.
**Do this instead:** Vietnamese `thoan_tu_vi` + `hao_tu_vi` mandatory; English fields reserved (declared `Option<String>`, default `None`, never populated in v1.7 corpus). Future translation milestone can populate them via additive corpus augmentation.

---

## Integration Points

### External Services

| Service | Integration Pattern | Notes |
|---------|---------------------|-------|
| `nhantu.net` (Mai Hoa reference) | Manual cross-check during golden-case authoring (`reasoning/iching/golden.rs`); not a runtime dependency. | Per EXPANSION_FRAMEWORK §7, ≥2 independent sources per case. Divergences logged as `KnownDivergence` (pattern at `almanac/fengshui/golden.rs`). |
| `divination.com` (hexagram texts) | Same — golden-case cross-check only. | Used to verify `thoán từ` English-translation parity if/when `title_en` is populated (it is NOT in v1.7). |

### Internal Boundaries

| Boundary | Communication | Notes |
|----------|---------------|-------|
| `reasoning/iching/` ↔ `rituals/` (schema reuse) | Direct type re-export: `use crate::rituals::schema::{SourceCitation, RitualConfidenceTier};` | Both pillars share citation/confidence shapes. Re-export avoids duplicate schema drift. |
| `reasoning/iching/` ↔ `almanac/` (lunar lookup) | Read-only call: `use crate::lunar::convert_solar_to_lunar;` | IChing casting needs lunar year/month/day + chi-hour from the query time. Reuses existing v1.0 lunar engine. |
| `reasoning/iching/` ↔ `reasoning/` (evaluator trait) | `impl ActionEvaluator for IChingEvaluator` | Trait defined at `reasoning/action_evaluator.rs:51`. |
| `reasoning/direction_composite.rs` ↔ `almanac/thai_tue.rs` | Read-only call: `compute_thai_tue(birth_chi_index, current_year_chi_index)` | Returns owned `ThaiTueResult` — no mutation possible. CRIT-3 safe. |
| `reasoning/direction_composite.rs` ↔ `almanac/sat_phuong.rs` | Read-only call: `get_sat_phuong(chi_index)` | Returns owned `SatPhuongResult` — no mutation possible. CRIT-3 safe. |
| `reasoning/direction_composite.rs` ↔ `almanac/fengshui/` | Read-only field access: `snapshot.flying_stars.as_ref()` | The cross-link NEVER calls `compute_*` directly — it reads the already-populated DaySnapshot field. This means FlyingStar is computed once per snapshot, not recomputed per cross-link call. CRIT-3 safe (no new `compute_daily_flying_stars` invocation). |
| `reasoning/direction_composite.rs` ↔ `interaction/direction_merge.rs` | **NO communication.** | The cross-link is a sibling to direction_merge, not a consumer of it. direction_merge stays CRIT-3-quarantined. |

---

## CRIT-3 Isolation Preservation — Explicit Treatment

The brief asks specifically how the Thái Tuế/Tam Sát ⇄ Phi Tinh cross-link preserves CRIT-3. The answer has three mechanical guarantees:

### Guarantee 1: Module placement
The cross-link lives at `crates/amlich-core/src/reasoning/direction_composite.rs`. The CRIT-3 CI guard (`tests/fengshui_crit3_isolation.rs`) reads only one file: `src/interaction/direction_merge.rs`. Files under `reasoning/` are outside its scope. This is not a loophole — it is the project's intended carve-out. `reasoning/personal.rs:1-9` already imports from both `almanac::tu_menh` and `interaction::*` without violating CRIT-3, establishing that the reasoning layer is the designated join layer.

### Guarantee 2: Read-only signatures
The cross-link function takes `&DaySnapshot` and `birth_chi_index: usize`, returning an owned `PersonalFactNode`. It calls `compute_thai_tue(...)` and `get_sat_phuong(...)` which themselves return owned values (`ThaiTueResult`, `SatPhuongResult`). It accesses `snapshot.flying_stars.as_ref()` — a shared reference. At no point does it mutate either source's outputs. This makes the read-only-ness mechanically verifiable via grep:
```bash
# tests/thai_tue_cross_link_crit3.rs (NEW) — grep guard
# Asserts direction_composite.rs contains ONLY &-borrow patterns of the
# CRIT-3-isolated symbols, never &mut or owned mutable handles.
```

### Guarantee 3: Distinct source_ids on every emitted envelope
Per Pattern 4, the cross-link emits three envelopes with three distinct source_ids: `SOURCE_KHCBPPT`, `SOURCE_HUYEN_KHONG`, and `"rule.composite.direction_cross_link"`. The first two are unchanged from their home modules — the cross-link does NOT mint a merged tradition id (Anti-Pattern 4). This means a downstream audit can always answer "is this claim from KHCBPPT or Huyền Không?" — the provenance is never lost.

### Required CI Guard Extension
The existing `tests/fengshui_crit3_isolation.rs` should be **augmented** (not replaced) with a sibling test `tests/thai_tue_cross_link_crit3.rs` that asserts:
1. `direction_composite.rs` exists under `src/reasoning/` (NOT under `src/interaction/`).
2. `direction_composite.rs` does not contain `&mut` references to `FlyingStarLayout`, `ThaiTueResult`, or `SatPhuongResult`.
3. `direction_composite.rs` imports both `crate::almanac::thai_tue` AND `crate::almanac::fengshui` (proving the cross-link is real, not a no-op).
4. `interaction/direction_merge.rs` still does NOT import `crate::almanac::fengshui` (the original CRIT-3 boundary is intact).

This mirrors the discipline that v1.5 used to extend `source_id_guard.rs` whenever new `SOURCE_*` constants were registered.

---

## Build Order (Dependency-Driven)

The v1.5 milestone established the schema-lock-first precedent (Phase 10 → 11/12/13/14/15). v1.7 follows the same shape with one wrinkle: the Thái Tuế cross-link is **independent** of the IChing pillar and can be parallelised after Phase 1.

### Phase 1 — Foundation: schemas, sources, ADRs, ontology (BLOCKING; everything else depends on this)
- **Files created:** `.planning/adrs/0005-iching-schema-v1.md`, `.planning/adrs/0006-mai-hoa-casting-algorithm.md`, `.planning/adrs/0007-thai-tue-phi-tinh-cross-link.md`.
- **Files modified (additive):**
  - `sources.rs` — `+SOURCE_KINH_DICH`, `+SOURCE_MAI_HOA_DICH_SO`
  - `tests/source_id_guard.rs` — extend `FORBIDDEN_LITERALS` with `"kinh-dich"`, `"mai-hoa-dich-so"`
  - `reasoning/types.rs` — `+ReasoningEvidenceSourceFamily::IChing`, `+ActionId::IChing`
  - `semantic_graph/ontology.rs` — 6-slice additions for `NodeConcept::Hexagram`, `EdgeConcept::LocatedAt`, `EdgeConcept::Transforms`
  - `crates/amlich-core/data/schemas/iching-schema.json` (JSON Schema mirror of ADR-0005 for tooling)
- **Files created (stubs only):**
  - `reasoning/iching/mod.rs`, `reasoning/iching/schema.rs` — locked types, no logic, serde round-trip tests
- **Why first:** Every downstream phase needs the source_ids registered, the source_family enum extended, and the ontology extended. Doing any of these out-of-order triggers rework.

### Phase 2 — Thái Tuế evidence backfill (CAN PARALLELISE with Phases 3-5)
- **Files modified (1 line each):**
  - `almanac/thai_tue.rs:107-111` — populate `evidence: Some(RuleEvidence { source_id: SOURCE_KHCBPPT.to_string(), method: "thai_tue_5_relationships", profile: "baseline" })` instead of `None`.
  - `almanac/sat_phuong.rs:49-53` — same pattern for `get_sat_phuong`.
- **Why parallelisable:** Pure additive backfill; existing call-sites ignore the field. No corpus or algorithm work blocks it.

### Phase 3 — IChing corpus + loader (DEPENDS ON Phase 1)
- **Files created:**
  - `data/iching/hexagrams.json` — 64 entries
  - `data/iching/manifest.json`, `data/iching/provenance_audit.md`
  - `reasoning/iching/corpus.rs` — `OnceLock` loader, mirrors `rituals/corpus.rs:85-117`
- **Why this order:** Corpus authors need the locked schema from Phase 1 before they can write entries. The loader cannot exist before the schema.

### Phase 4 — Mai Hoa casting algorithm + biến quẻ (DEPENDS ON Phase 3)
- **Files created:**
  - `reasoning/iching/mai_hoa.rs` — `cast_hexagram_mai_hoa(query_time) -> HexagramCast`
  - `reasoning/iching/bien_que.rs` — `derive_transformed_hexagram(cast) -> Hexagram`
  - `reasoning/iching/golden.rs` — 10+ golden cases vs nhantu.net + divination.com
- **Why this order:** Algorithm consumes the corpus loader from Phase 3 (`get_hexagram_by_number`). Cannot test without it.

### Phase 5 — IChing evaluator + DaySnapshot integration (DEPENDS ON Phase 4)
- **Files created:**
  - `reasoning/iching/evaluator.rs` — `impl ActionEvaluator for IChingEvaluator`
- **Files modified (additive):**
  - `lib.rs:154-185` — `+pub iching_cast: Option<IChingCastSummary>` with `#[serde(default, skip_serializing_if = "Option::is_none")]`
  - `lib.rs:274-360` — populate `iching_cast` when an IChing query is supplied
  - `reasoning/personal.rs` — `+build_iching_fact_nodes()` method on `PersonalReasoningInput`
  - `reasoning/mod.rs` — `pub use iching::*` re-exports
- **Why this order:** The evaluator needs the algorithm (Phase 4) to cast and the corpus (Phase 3) to look up text. The DTO field needs the evaluator to populate it.

### Phase 6 — Thái Tuế ⇄ Phi Tinh cross-link (CAN START after Phase 2; INDEPENDENT of Phases 3-5)
- **Files created:**
  - `reasoning/direction_composite.rs` — `build_direction_cross_link(snapshot, birth_chi_index) -> PersonalFactNode`
  - `tests/thai_tue_cross_link_crit3.rs` — extended CRIT-3 grep guard
- **Files modified (additive):**
  - `lib.rs` — `+pub direction_cross_link: Option<DirectionCrossLinkSummary>` on DaySnapshot
  - `reasoning/personal.rs` — wire cross-link fact into `build_fact_nodes()` as an additive branch
- **Why independent:** The cross-link consumes `compute_thai_tue`, `get_sat_phuong` (Phase 2 backfill), and `snapshot.flying_stars` (shipped v1.5). It does NOT touch anything in `reasoning/iching/`.

### Phase 7 — Semantic graph wiring (DEPENDS ON Phase 5 and Phase 6)
- **Files modified:**
  - `semantic_graph/builders/day_snapshot.rs:42-44` — `+builder.add_iching_facts(snapshot); +builder.add_direction_composite_facts(snapshot);`
  - New private methods on `DaySnapshotGraphBuilder` emitting Hexagram nodes + Transforms/LocatedAt edges + composite fact node
- **Why last:** The builder consumes everything produced by Phases 5 and 6. Wiring earlier would compile against stubs and miss integration bugs.

### Phase 8 — E2E validation (DEPENDS ON all above)
- 2026 smoke test extension (`tests/integration_2026_smoke.rs`)
- v1.6 backward-compat round-trip (`tests/day_snapshot_v14_compat.rs` pattern) — v1.6 producer JSON must deserialize into v1.7 DaySnapshot
- 10+ golden IChing cases cross-checked against ≥2 sources per EXPANSION_FRAMEWORK §7

### Dependency Graph Summary

```
                 ┌──────────────────────────┐
                 │ Phase 1: Foundation      │
                 │ (ADRs + sources + enum)  │
                 └────────┬─────────────────┘
                          │
              ┌───────────┴────────────┐
              ▼                        ▼
   ┌────────────────────┐    ┌────────────────────┐
   │ Phase 2: Thai Tue  │    │ Phase 3: Corpus    │
   │ evidence backfill  │    │ + loader           │
   └────────┬───────────┘    └────────┬───────────┘
            │                         ▼
            │              ┌────────────────────┐
            │              │ Phase 4: Mai Hoa   │
            │              │ casting algorithm  │
            │              └────────┬───────────┘
            │                       ▼
            │              ┌────────────────────┐
            │              │ Phase 5: Evaluator │
            │              │ + DTO integration  │
            │              └────────┬───────────┘
            │                       │
            ▼                       ▼
   ┌────────────────────┐          │
   │ Phase 6: Thai Tuế  │          │
   │ cross-link module  │          │
   └────────┬───────────┘          │
            │                       │
            └───────────┬───────────┘
                        ▼
              ┌────────────────────┐
              │ Phase 7: Semantic  │
              │ graph wiring       │
              └────────┬───────────┘
                       ▼
              ┌────────────────────┐
              │ Phase 8: E2E       │
              │ validation         │
              └────────────────────┘
```

**Critical path:** Phase 1 → 3 → 4 → 5 → 7 → 8 (IChing pillar).
**Parallel track:** Phase 1 → 2 → 6 (Thái Tuế cross-link), merges at Phase 7.

---

## Summary of New vs Modified Files (Audit Table)

| File | Status | Lines changed (est.) | Notes |
|------|--------|----------------------|-------|
| `.planning/adrs/0005-iching-schema-v1.md` | NEW | ~160 | Mirrors ADR-0001 structure exactly. |
| `.planning/adrs/0006-mai-hoa-casting-algorithm.md` | NEW | ~80 | Locks the time-number formulas + động hào derivation. |
| `.planning/adrs/0007-thai-tue-phi-tinh-cross-link.md` | NEW | ~60 | Formalises CRIT-3 carve-out for reasoning layer. |
| `crates/amlich-core/src/sources.rs` | MODIFIED | +6 | Two `pub const` lines + two test assertions. |
| `crates/amlich-core/tests/source_id_guard.rs` | MODIFIED | +2 | Two new entries in `FORBIDDEN_LITERALS`. |
| `crates/amlich-core/src/reasoning/types.rs` | MODIFIED | +4 | New `IChing` source family variant + `ActionId::IChing` variant. |
| `crates/amlich-core/src/semantic_graph/ontology.rs` | MODIFIED | +18 (6-slice) | Three concepts × six slices. |
| `crates/amlich-core/src/almanac/thai_tue.rs` | MODIFIED | +4 (1 line populated) | Populate `evidence.source_id`. |
| `crates/amlich-core/src/almanac/sat_phuong.rs` | MODIFIED | +4 (1 line populated) | Populate `evidence.source_id`. |
| `crates/amlich-core/src/lib.rs` | MODIFIED | +20 | Two additive `Option<T>` DaySnapshot fields + populate blocks + two new public API fns. |
| `crates/amlich-core/src/reasoning/mod.rs` | MODIFIED | +3 | `pub use iching::*; pub use direction_composite::*;` |
| `crates/amlich-core/src/reasoning/personal.rs` | MODIFIED | +60 | Two new builder methods on `PersonalReasoningInput`; no signature changes to existing. |
| `crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs` | MODIFIED | +120 | Two new private builder methods; two new call lines in `new()`. |
| `crates/amlich-core/src/reasoning/iching/mod.rs` | NEW | ~30 | Public API re-exports. |
| `crates/amlich-core/src/reasoning/iching/schema.rs` | NEW | ~200 | Locked HexagramEntry + sub-types + serde tests. |
| `crates/amlich-core/src/reasoning/iching/corpus.rs` | NEW | ~170 | `OnceLock` loader, NFC, source_id discipline. |
| `crates/amlich-core/src/reasoning/iching/mai_hoa.rs` | NEW | ~150 | Time-number casting algorithm. |
| `crates/amlich-core/src/reasoning/iching/bien_que.rs` | NEW | ~80 | Biến quẻ derivation. |
| `crates/amlich-core/src/reasoning/iching/evaluator.rs` | NEW | ~200 | `impl ActionEvaluator for IChingEvaluator`. |
| `crates/amlich-core/src/reasoning/iching/golden.rs` | NEW | ~150 | 10+ golden cases + `KnownDivergence` support. |
| `crates/amlich-core/src/reasoning/direction_composite.rs` | NEW | ~180 | Read-only cross-link + composite envelope. |
| `crates/amlich-core/data/iching/hexagrams.json` | NEW | ~3-5k lines | 64 hexagram entries. |
| `crates/amlich-core/data/iching/manifest.json` | NEW | ~20 | Tooling-only file list. |
| `crates/amlich-core/data/iching/provenance_audit.md` | NEW | ~200 | Per-entry citation + reviewer ledger. |
| `crates/amlich-core/tests/thai_tue_cross_link_crit3.rs` | NEW | ~80 | Extended CRIT-3 grep guard for the new module. |
| `crates/amlich-core/tests/iching_integration.rs` | NEW | ~250 | 10+ black-box tests for cast + corpus lookup + biến quẻ. |
| `crates/amlich-core/tests/iching_golden.rs` | NEW | ~150 | Independent golden-case verification (cross-source). |

**Backward-compat invariant:** Every MODIFIED file change is additive. The only enum extensions (`ReasoningEvidenceSourceFamily::IChing`, `ActionId::IChing`, `NodeConcept::Hexagram`, `EdgeConcept::{LocatedAt,Transforms}`) require updating exhaustive `match` sites — those are mechanical and CI-enforced. No existing public function signature changes. No existing JSON output shape changes (additive `Option<T>` with `skip_serializing_if = Option::is_none`).

---

## Sources

### In-repo authoritative references (HIGH confidence)
- `.planning/PROJECT.md` — v1.0–v1.6 milestone history, validated capabilities, key decisions table.
- `.planning/research/EXPANSION_FRAMEWORK.md` §2.2 (Kinh Dịch pillar spec), §3.1 (source provenance), §3.2 (semantic graph extension patterns).
- `.planning/adrs/0001-ritual-schema-v1.md` — ADR template + schema-lock-first precedent.
- `crates/amlich-core/src/rituals/corpus.rs:85-117` — `OnceLock + include_str!` corpus loader pattern.
- `crates/amlich-core/src/rituals/schema.rs` — `#[serde(deny_unknown_fields)]` schema-lock pattern + `RitualMetadata` additive extension.
- `crates/amlich-core/src/almanac/fengshui/types.rs:120-143` — sibling-not-extend pattern for `FlyingStarLayout` / `DailyFlyingStarLayout`.
- `crates/amlich-core/src/semantic_graph/ontology.rs` — 6-slice ontology discipline (full file).
- `crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs:476-747` — FlyingStar/Ritual/Offering builder precedent; `add_offering_facts:697-744` INT-09 dual-source provenance precedent.
- `crates/amlich-core/src/reasoning/types.rs:144-151` — `ReasoningEvidenceEnvelope` shape.
- `crates/amlich-core/src/reasoning/types.rs:5-7, 11-15` — `ActionId`, `NodeKind` closed enums (extension surface).
- `crates/amlich-core/src/reasoning/action_evaluator.rs:51-67` — `ActionEvaluator` trait (IChing evaluator target).
- `crates/amlich-core/src/reasoning/personal.rs:31-107` — `PersonalReasoningInput::build_fact_nodes` integration point.
- `crates/amlich-core/src/advisory.rs:18-30` — `ConsultationIntent` enum (must NOT be extended; see Anti-Pattern 1).
- `crates/amlich-core/src/lib.rs:154-185` — `DaySnapshot` additive `Option<T>` discipline.
- `crates/amlich-core/src/almanac/thai_tue.rs:36-112` — `ThaiTueResult.evidence: Option<RuleEvidence>` (currently `None`, backfill target).
- `crates/amlich-core/src/almanac/sat_phuong.rs:38-54` — `SatPhuongResult.evidence` (same backfill target).
- `crates/amlich-core/src/interaction/direction_merge.rs:1-87` — CRIT-3-isolated direction-merge module (NOT to be modified for cross-link).
- `crates/amlich-core/tests/fengshui_crit3_isolation.rs:14-21` — CRIT-3 grep guard (`FORBIDDEN_TYPE_NAMES`).
- `crates/amlich-core/tests/source_id_guard.rs:17-25` — source_id literal CI guard (`FORBIDDEN_LITERALS`).
- `.planning/milestones/v1.5-ROADMAP.md` — schema-lock-first phase ordering (Phase 10 → 11 → 12 → 13 → 14 → 15).

### External references (MEDIUM confidence — used only for golden-case cross-check planning, not for architecture)
- EXPANSION_FRAMEWORK §7 — `nhantu.net` (Mai Hoa), `divination.com` (hexagram texts) for golden-case validation per the project's ≥2-sources-per-case discipline.

---
*Architecture research for: amlich-core v1.7 Kinh Dịch pillar + Thái Tuế ⇄ Phi Tinh cross-link*
*Researched: 2026-07-16*
