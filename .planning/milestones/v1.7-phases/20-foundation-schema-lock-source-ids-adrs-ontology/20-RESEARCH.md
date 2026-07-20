# Phase 20: Foundation — Schema Lock + Source IDs + ADRs + Ontology - Research

**Researched:** 2026-07-16
**Domain:** Rust type-system boundary enforcement + classical I-Ching (Mai Hoa) domain conventions + additive ontology/source-id/ADR scaffolding
**Confidence:** HIGH — every integration point opened and confirmed against the v1.5/v1.6 codebase; the single external domain claim (Tiên Thiên trigram numbering) verified against vi.wikipedia Mai Hoa Dịch Số + matches CONTEXT.md verbatim.

## Summary

Phase 20 is the third "Foundation — Schema Lock" exercise (Phase 10 = v1.5, Phase 16 = v1.6 ADR-0003a, this = v1.7). The 20-CONTEXT.md is exceptionally detailed: it locks the struct shape, file paths, ADR bodies' substance, and the reviewer/DeferralMarker reuse. **This research validates that those locked decisions fit the codebase as described and fills the narrow gaps the planner needs** (newtype encoding choice, composition-table representation, ontology slice mechanics, DEC-NNNN numbering). It does NOT re-derive locked decisions.

Three confirmations matter most to the planner:
1. **The Tiên Thiên numbering (Kiền=1, Đoài=2, Ly=3, Chấn=4, Tốn=5, Khảm=6, Cấn=7, Khôn=8) is the canonical Phục Hy / Thiệu Khang Tiết arrangement** — verified verbatim against vi.wikipedia's Mai Hoa Dịch Số article §"Lập quẻ đơn: Quẻ trừ 8", which also names the exact Vietnamese edition the project already cites (*Mai Hoa Dịch số*, Thiệu Khang Tiết, dịch giả Văn Tùng, NXB Văn Hoá Thông tin, Hà Nội, 2002). ADR-0006's two-source pin is achievable from open references.
2. **The Hậu Thiên (King Wen / Lo Shu) trigram numbering is DIFFERENT and has a documented sub-school variance** — the same Wikipedia article shows Khảm=1, Khôn=2, Chấn=3, Tốn=4, Kiền=6, Đoài=7, Cấn=8, Ly=9 (the Lo Shu palace assignment the project's `Palace` enum already uses). This validates the CRIT-3 "three distinct newtypes, no From between them" discipline and means ADR-0005 must pin the exact `HauThienTrigram(u8)` encoding.
3. **`ActionId` and `ReasoningEvidenceSourceFamily` live in the SAME file (`reasoning/types.rs`), are matched nowhere exhaustively, and adding variants is purely additive** — no call-site churn, no `#[non_exhaustive]` escape needed. The 6-slice ontology extension has a fresh precedent (Phase 19's Offering/RecommendsOffering) sitting at exactly the lines CONTEXT.md cites.

**Primary recommendation:** Follow the `Palace` / `FlyingStar` `#[repr(u8)]` enum + explicit-discriminant style for the three trigram/hexagram newtypes (more self-documenting than a bare `struct(u8)`, and `HauThienTrigram` can reuse `Palace`'s exact Lo Shu numbers). Land the composition table as a `const ... [(_,_); 64]` indexed by King Wen + a `fn compose()` + a load-time bijectivity test. Assign DEC-0026/0027/0028 to ADR-0005/0006/0007 (DEC-0025 is the highest registered ID; the v1.6 ADR-0003a/0004 were not table-registered — leave that gap or backfill at planner discretion).

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions (research THESE, not alternatives)

**ADR-0006 — Mai Hoa casting convention:**
- Two-source pin from day 1: Thiệu Khang Tiết (classical authority for Tiên Thiên) + nhantu.net (modern Vietnamese practitioner reference).
- Lock lunar-only input + `((n-1)%k)+1` remainder-zero convention (CRIT-2 prevention: `n=8, k=8 → 8`, not 1).
- Defer exact parameter encoding (chi as `u8` vs typed enum) to Phase 22.
- Worked `month=8 / day=8 / hour=8 → Khôn, not Kiền` derivation in ADR body itself (CRIT-2 proof).
- Best-effort page citation + `PendingExternalReview` page-deferral marker (mirrors ADR-0004).

**HexagramEntry schema (ADR-0005):**
- Reserve English `*_en` optional fields (`vi_name_en`, `thoai_tu_en`, `hao_tu_en`) as `Option<T>` with `#[serde(default, skip_serializing_if = "Option::is_none")]` (mirrors RIT-13 `body_en`).
- Field naming: `vi_name` (language marker at front for content) vs `thoai_tu`/`hao_tu`/`cat_hung` (romanized VN technical terms unmarked). ADR-0005 must document this divergence from rituals' `body`/`body_en` suffix pattern.
- `reviewer: String` ON each entry (free-text `ExternalReviewPending(reason="..."; expected_review_date="..."; assigned_to="...")` marker). NOT a typed struct; survives reviewer-name change without schema migration.
- Reuse `DeferralMarker` from `crates/amlich-core/src/almanac/fengshui/golden.rs:85-95` for `pending_review: Option<DeferralMarker>`.
- `hao_tu: Vec<String>` length rule: 6 for hexagrams 3..=64; **7 for hexagrams 1 & 2** (dụng cửu / dụng lục). Enforced by loader invariant (Phase 21); ADR-0005 must mention the rule.
- ADR storage stays in `.planning/adrs/` (NOT `docs/adr/` — does not exist).
- ADR-0007 grep guard (`tests/thai_tue_cross_link_crit3.rs`) is a sibling to `tests/fengshui_crit3_isolation.rs` (same pattern, different module).

### Claude's Discretion (research options, recommend)

- **Newtype internal encoding** — `TienThienTrigram(u8)` carries classical Tiên Thiên position 1..=8 (Kiền=1) vs 3-bit line pattern 0..=7. Locked constraint: three distinct newtypes, NO `From` between them, each `Debug + Clone + Copy + PartialEq + Eq + Serialize + Deserialize`. **→ See Architecture Patterns §"Newtype encoding recommendation".**
- **Composition table representation** — `const COMPOSITION_TABLE: [(TienThienTrigram, TienThienTrigram); 64]` vs `fn compose(...)` vs `data/iching/composition_table.json`. Constraints: WASM-safe (no `std::fs`), "validates at load" bijectivity check. **→ See Architecture Patterns §"Composition table representation".**
- **"Validates at load" assertion semantics** — bijectivity (every King Wen 1..=64 ↔ exactly one Tiên Thiên pair) vs exhaustive coverage vs both. **→ See Architecture Patterns §"Bijectivity contract test".**
- **1-entry serde round-trip probe corpus content** — synthetic / hexagram #1 (Kiền) / tricky case (hexagram #2 Khôn with 7 hao_tu, or NFC-sensitive diacritics). **→ See Common Pitfalls §"Probe fixture choice".**
- **`upper_trigram`/`lower_trigram` newtype identity** — Tiên Thiên (casting output) vs Hậu Thiên (King Wen display). **→ See Architecture Patterns §"Hậu Thiên vs Tiên Thiên on HexagramEntry".**
- **ADR-0007 body depth** — mirror ADR-0002/0003/0004 Nygard short-form length.
- **Ontology variant label strings** — `"hexagram"` / `"located_at"` / `"transforms"` (snake_case English, following `flying_star`/`offering`/`recommends_offering`).
- **`ReasoningEvidenceSourceFamily::IChing` shape** — sibling variant alongside `AlmanacRule`, `FolkTradition`, etc. (NOT reusing `AlmanacRule`).
- **`.planning/MILESTONES.md` DEC-NNNN rows** — three new rows for ADR-0005/0006/0007.

### Deferred Ideas (OUT OF SCOPE)

- English `*_en` field POPULATION (reserved only; no milestone scheduled).
- `cast_mai_hoa` algorithm + biến quẻ + Thể/Dụng → Phase 22.
- 64-hexagram corpus authoring → Phase 21.
- `IChingQuery` sibling newtype + `IChingEvaluator` → Phase 24.
- `build_direction_cross_link` + Thái Tuế directional + classical 3-direction Tam Sát → Phase 23.
- `DaySnapshot.iching_cast` + `direction_cross_link` additive fields → Phase 24.
- E2E validation + ≥10 golden cross-source cases → Phase 25.
- Image / unicode hexagram symbol fields on `HexagramEntry` — NOT reserved in v1.7.
- Custom clippy lint for source_id literals (Phase 10 deferral stands; grep test remains canonical).
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| FND-09 | Register `SOURCE_KINH_DICH` ("kinh-dich") + `SOURCE_MAI_HOA_DICH_SO` ("mai-hoa-dich-so") as `pub const`; extend `source_id_guard.rs::FORBIDDEN_LITERALS`. | "source_id_guard.rs extension" finding confirms the escaped-quote format (`"\"kinh-dich\""`); "sources.rs append point" finding confirms the exact insertion location + the `all_constants_have_expected_values` test to extend. |
| FND-10 | Accept ADR-0005 (`HexagramEntry` schema v1, `deny_unknown_fields`), ADR-0006 (Mai Hoa convention: Tiên Thiên pin, lunar, `((n-1)%k)+1`), ADR-0007 (cross-link CRIT-3 carve-out). | "ADR template" finding confirms Nygard short-form (Title/Status/Context/Decision/Consequences) via 0001-ritual-schema-v1.md + 0004-daily-phi-tinh-starting-star-convention.md; "Tiên Thiên numbering" finding provides the authoritative vi.wikipedia citation for ADR-0006; "DEC-NNNN numbering" finding gives the next safe IDs. |
| FND-11 | Lock `HexagramEntry` with `deny_unknown_fields` + passing 1-entry serde round-trip probe BEFORE corpus authoring; three distinct newtypes `TienThienTrigram(u8)` / `HauThienTrigram(u8)` / `KingWenHexagram(u8)` with NO `From` between them; 64-entry Tiên Thiên-pair → King Wen composition table validated at load. | "Newtype encoding recommendation" + "Composition table representation" + "Bijectivity contract test" findings give the planner prescriptive shapes; "DeferralMarker reuse" finding confirms the struct is reusable verbatim at golden.rs:85-95. |
| FND-12 | Extend 6-slice ontology with `NodeConcept::Hexagram`, `EdgeConcept::LocatedAt`, `EdgeConcept::Transforms`; add `ReasoningEvidenceSourceFamily::IChing` + `ActionId::IChing` variants. | "Ontology 6-slice locations" finding pins the exact line ranges + the test-extension precedent; "Enum variant additive-safety" finding confirms NO exhaustive match blocks exist for the two reasoning enums. |
</phase_requirements>

## Standard Stack

### Core (no new dependencies — v1.5/v1.6 "no new deps" precedent holds)

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `serde` / `serde_json` | workspace pin | Derive `Serialize`/`Deserialize` for `HexagramEntry` + the three newtypes; parse the 1-entry probe JSON | Already the corpus-entry discipline (ADR-0001 Ritual, golden datasets) |
| `unicode-normalization` | 0.1.25 (direct) | NFC-normalize Vietnamese text fields at load (RIT-08 precedent) | Already direct dep; probe should exercise NFC-sensitive diacritics to prove normalization-safety |
| `std::sync::OnceLock<T>` + `include_str!` | std | Compile-time corpus embedding + lazy parse | Phase 21 loader pattern (NOT Phase 20 — probe uses inline literal JSON) |

**`Cargo.toml` is unchanged.** Phase 20 adds zero `[dependencies]` entries. The success criterion "no new crates" is satisfied by construction.

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `struct TienThienTrigram(pub u8)` newtype | `#[repr(u8)] enum TienThienTrigram { Kien = 1, ... }` | **Enum is more self-documenting** (variant names = trigram names) and matches the `Palace`/`FlyingStar` precedent at `fengshui/types.rs:15-43,70-91` exactly. Recommend enum. See Architecture Patterns. |
| `data/iching/composition_table.json` + `include_str!` | `const COMPOSITION_TABLE: [...; 64]` Rust array | **Const array is WASM-safe by construction** (no parse step, no `serde_json` at load), validates at compile, mirrors `Palace::ALL`. JSON file would also work but adds a parse + a second `OnceLock`. Recommend const array. |
| Hand-authored 64-entry composition table | Upstream Rust I-Ching crate (`xalen-iching`, `i-ching`, `iching`) | **REJECTED** per SUMMARY.md — none carry Vietnamese text; provenance violation (DEC-0015/0016). Hand-author the 64-tuple from the classical King Wen sequence. |

## Architecture Patterns

### Recommended File Layout (Phase 20 only — reserves locations for Phases 21-24)

```
crates/amlich-core/src/
├── sources.rs                          # +2 pub const (after line 26)
├── lib.rs                              # +pub mod iching; (reserve only — Phase 21+ populates)
├── reasoning/
│   └── types.rs                        # +ActionId::IChing, +ReasoningEvidenceSourceFamily::IChing
├── semantic_graph/
│   └── ontology.rs                     # 6-slice extension (3 new concepts × 6 locations + test)
└── iching/                             # NEW module dir (Phase 20 reserves; Phase 21+ fills)
    ├── mod.rs                          # pub use schema::*; (re-export surface)
    └── schema.rs                       # HexagramEntry + 3 newtypes + COMPOSITION_TABLE + compose()

crates/amlich-core/tests/
├── source_id_guard.rs                  # +2 FORBIDDEN_LITERALS entries
└── iching_schema_probe.rs              # NEW — 1-entry serde round-trip + bijectivity test

.planning/adrs/
├── 0005-hexagram-entry-schema-v1.md    # NEW
├── 0006-mai-hoa-casting-convention.md  # NEW
└── 0007-cross-link-crit3-carve-out.md  # NEW

crates/amlich-core/data/iching/         # NEW dir (reserve; Phase 21 authors hexagrams.json)
```

**Module-path choice (planner discretion, locked consequence):** CONTEXT.md notes the roadmap places the cross-link in `reasoning/direction_composite.rs`, suggesting IChing code also lives under `reasoning/`. Two viable layouts:
- `pub mod iching;` at crate root (sibling to `reasoning`, `rituals`, `almanac`) — **cleaner**, IChing is a Tier-0 pillar not a reasoning concern; matches `rituals` (also a corpus pillar).
- `pub mod reasoning::iching;` — matches EXPANSION_FRAMEWORK §2.2 wording.

**Recommendation:** `pub mod iching;` at crate root (mirrors `rituals/`). The evaluator (Phase 24) implements `ActionEvaluator` so it crosses into `reasoning/` anyway; the corpus + schema + casting stay under `iching/`. The planner may choose either — both compile; the difference is import-path aesthetics.

### Pattern 1: Newtype encoding recommendation (CONTEXT.md discretion #1)

**What:** Three distinct types with NO `From` between them (CRIT-3 prevention).
**When to use:** Always — the locked constraint is "three distinct types, no From".
**Recommendation: follow the `Palace` / `FlyingStar` enum + `#[repr(u8)]` + explicit-discriminant style** (`fengshui/types.rs:15-43, 70-91`), NOT a bare `struct(pub u8)`.

```rust
// Source: crates/amlich-core/src/almanac/fengshui/types.rs:15-43 (Palace precedent)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum TienThienTrigram {
    Kien = 1,   // Càn / Kiền ☰ — Tiên Thiên position 1 (verified vi.wikipedia)
    Doai = 2,   // Đoài ☱
    Ly   = 3,   // Ly ☲
    Chan = 4,   // Chấn ☳
    Ton  = 5,   // Tốn ☴
    Kham = 6,   // Khảm ☵
    Can  = 7,   // Cấn ☶
    Khon = 8,   // Khôn ☷
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum HauThienTrigram {
    // Hậu Thiên / Lo Shu palace numbers (verified vi.wikipedia; matches Palace enum)
    Kham = 1, Khon = 2, Chan = 3, Ton = 4,
    // 5 = Center (no trigram in Lo Shu) — skipped
    Kien = 6, Doai = 7, Can = 8, Ly = 9,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum KingWenHexagram {
    // 1..=64 — King Wen sequence index; variants named after the hexagram
    // (full 64-variant enum is verbose but maximally self-documenting;
    //  planner may instead use `pub struct KingWenHexagram(pub u8)` with a
    //  1..=64 construction check — both satisfy the "no From between them" lock.)
    ...
}
```

**Why enum over struct newtype:**
- Variant names ARE the trigram names — a reader sees `TienThienTrigram::Kien` not `TienThienTrigram(1)` and does not need to decode.
- `#[repr(u8)]` preserves the `as u8` arithmetic needed for the composition table + casting (Phase 22).
- Serde `#[serde(rename_all = "snake_case")]` gives stable JSON `"kien"`/`"khon"` strings (the corpus JSON is human-readable).
- Matches the established `Palace` + `FlyingStar` style exactly — no new pattern introduced.

**Why NOT to add `impl From<TienThienTrigram> for HauThienTrigram`:** the locked CRIT-3 constraint. The composition table is the ONLY bridge: `(TienThienTrigram, TienThienTrigram) -> KingWenHexagram`. A `TienThienTrigram -> HauThienTrigram` mapping exists mathematically (same 8 trigrams, different numbering) but encoding it as `From` would re-open CRIT-3.

**KingWenHexagram full-enum-vs-struct tradeoff:** the 64-variant enum is verbose (~70 lines) but turns every composition-table entry into compile-checked named variants. A `pub struct KingWenHexagram(pub u8)` with a `const fn new(n: u8) -> Option<Self>` is lighter. **Recommendation: struct newtype for KingWenHexagram** (64 variants is too many to name ergonomically; the composition table provides the named mapping); **enum for the two trigram types** (8 variants is the sweet spot).

### Pattern 2: Composition table representation (CONTEXT.md discretion #2)

**What:** The 64-entry bijection between Tiên Thiên trigram pairs and King Wen hexagram indices.
**Recommendation:** `const COMPOSITION_TABLE: [(TienThienTrigram, TienThienTrigram); 64]` indexed by `KingWenHexagram` (index 0 = King Wen #1) + `fn compose(upper, lower) -> KingWenHexagram` + a bijectivity test.

```rust
// Source: mirrors Palace::ALL static-array precedent at fengshui/types.rs:32-42
/// The 64 King Wen hexagrams indexed by King Wen number (index 0 = King Wen #1 = Thuần Kiền).
/// Each entry is (upper_trigram, lower_trigram) in the Tiên Thiên arrangement.
/// Validated bijective at load (see tests::composition_table_is_bijective).
pub const COMPOSITION_TABLE: [(TienThienTrigram, TienThienTrigram); 64] = [
    (TienThienTrigram::Kien, TienThienTrigram::Kien),   // #1 Thuần Kiền (Càn)
    (TienThienTrigram::Khon, TienThienTrigram::Khon),   // #2 Thuần Khôn
    // ... 62 more — hand-authored from the classical King Wen sequence ...
    (TienThienTrigram::Chan, TienThienTrigram::Chan),   // #51 Thuần Chấn
    // ...
];

/// Compose a Tiên Thiên upper+lower pair into the King Wen hexagram index.
/// Used by the Phase 22 casting algorithm. Linear scan over COMPOSITION_TABLE
/// (64 iterations — negligible; a pre-computed reverse table is premature optimisation).
pub fn compose(upper: TienThienTrigram, lower: TienThienTrigram) -> KingWenHexagram {
    for (i, &(u, l)) in COMPOSITION_TABLE.iter().enumerate() {
        if u == upper && l == lower {
            return KingWenHexagram((i + 1) as u8);
        }
    }
    // Unreachable: every pair is present (bijectivity test guarantees this).
    panic!("composition table missing pair ({upper:?}, {lower:?})")
}
```

**Why const array (not JSON file):**
- WASM-safe by construction — no `std::fs`, no `serde_json::from_str` at load, no `OnceLock`.
- Compile-time checked — a typo in a tuple is a compile error, not a runtime test failure.
- Mirrors `Palace::ALL` (the existing precedent for "static collection of typed enum values").
- The "validates at load" success criterion becomes a `#[test] fn composition_table_is_bijective` — asserts (a) 64 distinct pairs, (b) every `TienThienTrigram` × `TienThienTrigram` combination (8×8=64) appears exactly once. This is the bijectivity proof; it runs in `cargo test`, not at runtime.

**Bijectivity contract test (CONTEXT.md discretion #3):**

```rust
#[test]
fn composition_table_is_bijective() {
    assert_eq!(COMPOSITION_TABLE.len(), 64);
    let mut seen = std::collections::HashSet::new();
    for (i, &(upper, lower)) in COMPOSITION_TABLE.iter().enumerate() {
        let king_wen = i as u8 + 1;
        assert!((1..=64).contains(&king_wen));
        let pair = (upper as u8, lower as u8);
        assert!(seen.insert(pair), "duplicate pair at King Wen #{king_wen}: {pair:?}");
    }
    // Exhaustive coverage: every Tiên Thiên pair composes to exactly one King Wen hexagram
    for u in TienThienTrigram::ALL {
        for l in TienThienTrigram::ALL {
            let _ = compose(u, l); // panics if missing — proves surjective
        }
    }
}
```

### Pattern 3: Hậu Thiên vs Tiên Thiên on HexagramEntry (CONTEXT.md discretion #5)

**What:** Which newtype(s) appear on the corpus's `upper_trigram` / `lower_trigram` fields?
**Recommendation: ONLY `HauThienTrigram`** (as CONTEXT.md locks at line 54-55). `TienThienTrigram` does NOT appear on `HexagramEntry`.

**Rationale:**
- The corpus is authored in the King Wen / Hậu Thiên text tradition (Ngô Tất Tố *Kinh Dịch Trọn Bộ* follows the King Wen sequence). Displaying trigram numbers in the Hậu Thiên (Lo Shu) numbering is consistent with that tradition.
- Mai Hoa casting (Phase 22) produces `TienThienTrigram` pair → composes via the table → `KingWenHexagram` → looks up `HexagramEntry` by `king_wen_index`. The corpus's `upper_trigram`/`lower_trigram` are **descriptive display metadata**, never re-composed.
- Putting `TienThienTrigram` on `HexagramEntry` would invite a future maintainer to "round-trip" cast → corpus → re-compose, which is a CRIT-3 trap. Keeping the corpus purely Hậu Thiên closes that door.

**Consequence for ADR-0005:** the ADR must pin the exact `HauThienTrigram(u8)` encoding (Lo Shu numbers: Khảm=1, Khôn=2, Chấn=3, Tốn=4, Kiền=6, Đoài=7, Cấn=8, Ly=9 — the same assignment the `Palace` enum already uses). This pins the sub-school variance flagged in vi.wikipedia.

### Pattern 4: Ontology 6-slice extension (confirmed locations)

**What:** Each new concept must appear in 6 places; the compiler enforces exhaustiveness via `NodeConcept::label()` and `ConceptLabel::as_str()` match arms.

**Confirmed slice locations in `crates/amlich-core/src/semantic_graph/ontology.rs`:**

| Slice # | Location | Lines | What to add |
|---------|----------|-------|-------------|
| 1 | `enum NodeConcept` | 5-43 | `Hexagram,` variant (after `Offering,` at line 42) |
| 2 | `NodeConcept::label()` match | 47-85 | `Self::Hexagram => ConceptLabel::Hexagram,` arm |
| 3 | `enum ConceptLabel` | 161-228 | `Hexagram,` variant (after `Offering,` at line 223) |
| 4 | `ConceptLabel::as_str()` match | 232-299 | `Self::Hexagram => "hexagram",` arm |
| 5 | `GraphOntology::node_concepts()` slice | 338-376 | `NodeConcept::Hexagram,` entry (after `Offering,` at line 375) |
| 6 (edges) | `GraphOntology::edge_concepts()` slice | 380-409 | `EdgeConcept::LocatedAt,` + `EdgeConcept::Transforms,` entries |

**PLUS the symmetric `EdgeConcept` slices (LocatedAt + Transforms are EDGES):**
- `enum EdgeConcept` (lines 91-121): add `LocatedAt,` + `Transforms,` after `RecommendsOffering,` (line 120).
- `EdgeConcept::label()` match (lines 124-156): add two arms.
- `enum ConceptLabel` (lines 161-228): add `LocatedAt,` + `Transforms,`.
- `ConceptLabel::as_str()` match (lines 232-299): add `Self::LocatedAt => "located_at",` + `Self::Transforms => "transforms",`.

**PLUS a test extension** (the v1.6 precedent at lines 324-333): add a `v17_concepts_present_in_ontology_slices()` test asserting `Hexagram`/`LocatedAt`/`Transforms` are present + label round-trips. The v1.5 test (lines 309-321) + v1.6 test (lines 324-333) are the templates.

**Most-recent addition precedent:** Phase 19's `Offering` (node) + `RecommendsOffering` (edge) — the exact template. STATE.md "Key Decisions Added in 19-02" notes the compiler enforced updates to `views/helpers.rs::cluster_for_node_id` + `views/visualization.rs::shape_hint_for_node` for the `Offering` variant. **The planner MUST grep for exhaustive match arms on `NodeConcept`/`EdgeConcept`/`ConceptLabel`** outside `ontology.rs` (e.g. in `semantic_graph/views/`) and extend them; the compiler will flag them but the planner should budget for it.

### Pattern 5: Enum variant additive-safety (confirmed for ActionId + ReasoningEvidenceSourceFamily)

**What:** Adding `ActionId::IChing` + `ReasoningEvidenceSourceFamily::IChing` variants.
**Confirmed:** Both enums live in `crates/amlich-core/src/reasoning/types.rs` (NOT separate files — CONTEXT.md speculated "likely in action.rs"; actual location is `types.rs:3-7` for `ActionId` and `types.rs:132-142` for `ReasoningEvidenceSourceFamily`).
**Additive-safe:** `rg "match\s+\w+\s*\{"` across `src/` for these two enum names returns NO exhaustive match blocks. The variants are only constructed (e.g. `ActionId::InitiationOpening` at `graph_projection.rs:98`, `initiation_opening_evaluator.rs:579,674`) — never matched. **Adding `IChing` to either is a single-line append with zero call-site churn.**

`ReasoningEvidenceSourceFamily::IChing` should sit alongside the 7 existing variants (`Snapshot`, `Interaction`, `Bazi`, `Axis`, `AlmanacRule`, `Insight`, `Derived`). IChing is a distinct Tier-0 family — NOT a reuse of `AlmanacRule` (CONTEXT.md locks this).

### Anti-Patterns to Avoid

- **Adding `impl From<TienThienTrigram> for HauThienTrigram`** (or any cross-newtype From) — re-opens CRIT-3. The composition table is the ONLY bridge.
- **Encoding `HauThienTrigram` as 1..=8** (Tiên Thiên range) — conflates the two arrangements; the Lo Shu numbers skip 5 and reach 9. ADR-0005 must pin the exact encoding.
- **Using `data/iching/composition_table.json` + runtime parse** — adds a `OnceLock` + `serde_json` dependency at load; the const array is WASM-safer and compile-checked.
- **Placing ADRs in `docs/adr/`** — that path does not exist; `.planning/adrs/` is the locked location (Phase 10 precedent).
- **Forgetting the test extension in ontology.rs** — the v1.5/v1.6 tests at lines 309-333 are the audit trail; a v1.7 test asserting `Hexagram`/`LocatedAt`/`Transforms` presence is part of FND-12, not optional.
- **Grepping only `ontology.rs` for exhaustive matches** — Phase 19 found matches in `views/helpers.rs` + `views/visualization.rs`. Run a crate-wide grep for `match.*NodeConcept` / `match.*EdgeConcept` / `match.*ConceptLabel` and budget for ~2-4 mechanical arm additions.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| `PendingExternalReview` marker | New `ReviewState` enum / `Reviewer` struct | `DeferralMarker` from `almanac/fengshui/golden.rs:85-95` (reused verbatim as `pending_review: Option<DeferralMarker>`) | CONTEXT.md locks this; zero new types; v1.6 RIT-14 pattern. |
| `reviewer` free-text format | New structured type | The rituals `ExternalReviewPending(reason="..."; expected_review_date="..."; assigned_to="...")` free-text marker from `data/rituals/provenance_audit.md` | Survives reviewer-name change without schema migration; CONTEXT.md locks this. |
| Composition table reverse lookup | Hash map / `BTreeMap` | Linear scan in `fn compose()` (64 iterations) | Premature optimisation; the const array is the source of truth; scan is branch-predictor-friendly and < 1µs. |
| `Palace`-like trigram enum | New spatial enum | Reuse the `#[repr(u8)] enum + explicit discriminants` PATTERN from `Palace` (do NOT reuse the `Palace` type itself — that would re-open CRIT-3 by making `HauThienTrigram` interchangeable with a palace-layout descriptor) | Pattern reuse, not type reuse. |

**Key insight:** Phase 20 creates **zero new framework types** beyond the three trigram/hexagram newtypes + `HexagramEntry` + the three ontology/enum variants. Everything else is reuse (DeferralMarker, ExternalReviewPending marker shape, Palace enum style, source_id const pattern, ADR Nygard template).

## Common Pitfalls

### Pitfall 1: Hậu Thiên trigram numbering sub-school variance
**What goes wrong:** `HauThienTrigram(u8)` is encoded with the wrong number assignment, making the corpus's display fields semantically meaningless or cross-contaminating with Tiên Thiên.
**Why it happens:** vi.wikipedia's Mai Hoa article shows TWO numberings side-by-side: the Tiên Thiên (Kiền=1..Khôn=8, unambiguous) and the Hậu Thiên (Lo Shu: Khảm=1, Khôn=2, Chấn=3, Tốn=4, Kiền=6, Đoài=7, Cấn=8, Ly=9 — the article also mentions a variant placing Ly at 5). A naive reader may assign Kiền=1 in both.
**How to avoid:** ADR-0005 pins the `HauThienTrigram` encoding to the **Lo Shu palace numbers** (Khảm=1...Ly=9, skipping 5/center) — exactly the assignment the project's `Palace` enum already uses at `fengshui/types.rs:15-43`. Document this pin in the ADR body + the enum's doc-comment.
**Warning signs:** A reviewer asks "why does `HauThienTrigram::Ly` serialize to 9, not 5?" — the ADR body should pre-empt this.

### Pitfall 2: Composition table bijectivity silent break
**What goes wrong:** A tuple is duplicated or missing; the `compose()` function panics on a valid cast result (Phase 22) or returns the wrong hexagram.
**Why it happens:** Hand-authoring 64 tuples is error-prone; the King Wen sequence is not alphabetically ordered.
**How to avoid:** The `composition_table_is_bijective` test (Architecture Patterns §2) asserts (a) 64 distinct pairs, (b) every Tiên Thiên pair appears exactly once (surjective via the `compose()` loop). **This test MUST be part of Phase 20**, not deferred — it is the "validates at load" success criterion.
**Warning signs:** `compose()` panics during Phase 22 casting tests — means a tuple is missing; run the bijectivity test first.

### Pitfall 3: Ontology exhaustive-match sites outside ontology.rs
**What goes wrong:** Adding `NodeConcept::Hexagram` compiles in `ontology.rs` but breaks the build in `semantic_graph/views/helpers.rs` or `views/visualization.rs` (the Phase 19 precedent).
**Why it happens:** `NodeConcept`/`EdgeConcept`/`ConceptLabel` are matched exhaustively in visualization helpers that assign shapes/colors/clusters per concept.
**How to avoid:** After editing `ontology.rs`, run `cargo build` immediately — the compiler lists every non-exhaustive match site. Budget ~30 min for ~2-4 mechanical arm additions in `views/`. Do NOT add `#[non_exhaustive]` to escape (CONTEXT.md + FND-12 lock "compiler-enforced exhaustive match with no `#[non_exhaustive]` escape").
**Warning signs:** Compile errors mentioning `NodeConcept::Hexagram` not covered — expected; fix the arm.

### Pitfall 4: source_id_guard test-fixture false positive
**What goes wrong:** Appending `"\"kinh-dich\""` / `"\"mai-hoa-dich-so\""` to `FORBIDDEN_LITERALS` triggers violations in existing test fixtures or the new schema/probe code.
**Why it happens:** The guard skips `sources.rs` + `#[cfg(test)]` blocks + `//`-comments, but a doc-comment example showing `"kinh-dich"` as a literal value, or a non-`#[cfg(test)]` const in `iching/schema.rs`, would trip it.
**How to avoid:** The new literals appear ONLY in `sources.rs` (the definitions) + the `all_constants_have_expected_values` test (already `#[cfg(test)]`). The `HexagramEntry` probe fixture's JSON uses `serde_json::json!({...})` which contains the string value — but the probe test is `#[cfg(test)]` so it's skipped. Verify by running `cargo test --test source_id_guard` after the append.
**Warning signs:** `source_id_guard.rs` test failure listing a `iching/schema.rs:NN` line — wrap the fixture in `#[cfg(test)]` or use the `SOURCE_*` const.

### Pitfall 5: Probe fixture choice (CONTEXT.md discretion #4)
**What goes wrong:** The 1-entry serde round-trip probe uses a trivial fixture (hexagram #1 Kiền, all-ASCII) that passes but does NOT exercise the schema's edge cases (7-length `hao_tu` for hexagrams 1 & 2, NFC-sensitive Vietnamese diacritics in `cat_hung`, `pending_review: Some(DeferralMarker {...})`).
**Why it happens:** Hexagram #1 Kiền is the "simplest" entry; a reader assumes "if #1 round-trips, all do."
**How to avoid:** **Use hexagram #2 Khôn for the probe** — it has 7 `hao_tu` (dụng lục seventh line), exercises the `hao_tu: Vec<String>` length rule, and its `cat_hung` can contain NFC-sensitive diacritics (e.g. `"khôn / đất — thuận phục, hanh thông, lợi gà desublimate"`). Also include one entry with `pending_review: Some(DeferralMarker { reason: "test deferral", expected_review_date: "2026-12-31", assigned_to: None })` to prove the `Option<DeferralMarker>` round-trips.
**Warning signs:** The probe test is < 20 lines and uses only ASCII — expand it.

### Pitfall 6: DEC-NNNN collision / gap
**What goes wrong:** Phase 20 assigns DEC-0026 to ADR-0005, unaware that an intermediate planning doc (or a backfilled v1.6 ADR-0003a/0004) already used DEC-0026.
**Why it happens:** The MILESTONES.md ADR Cross-References table (lines 275-279) registers only DEC-0023/0024/0025 (v1.5 ADRs 0001/0002/0003). ADR-0003a (Phase 16) and ADR-0004 (Phase 18) were authored but NOT table-registered — a known v1.6 gap.
**How to avoid:** The next safe IDs are **DEC-0026 / DEC-0027 / DEC-0028** for ADR-0005/0006/0007 (DEC-0025 is the highest formally registered). Optionally backfill DEC-0029/0030 for ADR-0003a/0004 (or assign DEC-0026/0027 to them and use DEC-0028/0029/0030 for the new ADRs). Confirm with `rg "DEC-002[6789]"` before writing.
**Warning signs:** A reviewer finds two ADRs claiming the same DEC-NNNN — the planner's grep pre-check prevents this.

## Code Examples

### sources.rs append (FND-09)

```rust
// Source: existing pattern at crates/amlich-core/src/sources.rs:7-26
/// Kinh Dịch (I-Ching) — Ngô Tất Tố hexagram text corpus (new in v1.7).
pub const SOURCE_KINH_DICH: &str = "kinh-dich";

/// Mai Hoa Dịch Số — Thiệu Khang Tiết casting algorithm (new in v1.7).
pub const SOURCE_MAI_HOA_DICH_SO: &str = "mai-hoa-dich-so";
```

The `all_constants_have_expected_values` test (lines 47-56) needs two new `assert_eq!` lines.

### source_id_guard.rs append (FND-09)

```rust
// Source: existing pattern at crates/amlich-core/tests/source_id_guard.rs:13-21
const FORBIDDEN_LITERALS: &[&str] = &[
    "\"khcbppt\"",
    "\"vn-folk\"",
    "\"ngoc-hap-ky\"",
    "\"cuu-dieu\"",
    "\"tam-menh-thong-hoi\"",
    "\"vn-folk-ritual\"",
    "\"huyen-khong\"",
    "\"kinh-dich\"",           // NEW v1.7
    "\"mai-hoa-dich-so\"",     // NEW v1.7
];
```

### HexagramEntry (FND-11) — full shape per CONTEXT.md line 49-66

```rust
// Source: CONTEXT.md locks this shape verbatim; DeferralMarker reused from golden.rs:85-95
use serde::{Deserialize, Serialize};
use crate::almanac::fengshui::golden::DeferralMarker;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HexagramEntry {
    pub king_wen_index: KingWenHexagram,
    pub vi_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vi_name_en: Option<String>,
    pub upper_trigram: HauThienTrigram,   // Hậu Thiên display per King Wen (NOT Tiên Thiên)
    pub lower_trigram: HauThienTrigram,
    pub thoai_tu: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thoai_tu_en: Option<String>,
    pub hao_tu: Vec<String>,              // 6 entries; 7 for hexagrams 1 & 2 (loader-enforced)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hao_tu_en: Option<Vec<String>>,
    pub cat_hung: String,
    pub reviewer: String,                 // ExternalReviewPending(reason="...";...) free-text marker
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending_review: Option<DeferralMarker>,
}
```

### 1-entry serde round-trip probe (FND-11)

```rust
// Source: mirrors rituals serde-round-trip pattern + Phase 19 Offering test
#[test]
fn hexagram_entry_one_entry_serde_round_trip() {
    let entry = HexagramEntry {
        king_wen_index: KingWenHexagram::new(2).unwrap(),  // #2 Khôn — exercises 7-hao_tu rule
        vi_name: "Khôn / Địa".to_string(),                  // NFC diacritics
        vi_name_en: None,
        upper_trigram: HauThienTrigram::Khon,               // Lo Shu 2
        lower_trigram: HauThienTrigram::Khon,
        thoai_tu: "Nguyên hanh, lợi mã chi trinh".to_string(),
        thoai_tu_en: None,
        hao_tu: vec![
            "Lý sương, kiên băng chí".to_string(),
            "Trực phương, đại, bất tập vô bất lợi".to_string(),
            "Hàm chương, khả trinh".to_string(),
            "Quát nang, vô cữu vô dự".to_string(),
            "Hoàng thường, nguyên cát".to_string(),
            "Long chiến dã, kỳ huyết huyền hoàng".to_string(),
            "Lợi vĩnh trinh".to_string(),  // 7th "dụng lục" — proves length rule
        ],
        hao_tu_en: None,
        cat_hung: "thuận phục, hanh thông, lợi ào thuyết".to_string(),  // NFC diacritics
        reviewer: r#"ExternalReviewPending(reason="Ngô Tất Tố source gap for #2 Khôn dụng lục; pending corpus authoring"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer")"#.to_string(),
        pending_review: Some(DeferralMarker {
            reason: "probe fixture — Phase 21 corpus will populate".to_string(),
            expected_review_date: "2026-12-31".to_string(),
            assigned_to: None,
        }),
    };
    let json = serde_json::to_string(&entry).expect("serialize");
    let roundtripped: HexagramEntry = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(entry.vi_name, roundtripped.vi_name);
    assert_eq!(roundtripped.hao_tu.len(), 7, "7-hao_tu rule for hexagrams 1 & 2");
    assert_eq!(roundtripped.pending_review.as_ref().unwrap().expected_review_date, "2026-12-31");
}
```

### ADR-0006 worked boundary example (FND-10) — the CRIT-2 prevention proof

```markdown
### Worked boundary example (CRIT-2 prevention)

For inputs `lunar_year_branch=8, lunar_month=8, lunar_day=8, chi_hour_index=8`
(all-eights boundary):

- Upper trigram: `((year + month + day - 1) % 8) + 1 = ((8 + 8 + 8 - 1) % 8) + 1 = (23 % 8) + 1 = 7 + 1 = 8` → Tiên Thiên 8 = Khôn ☷
- Lower trigram: `((year + month + day + hour - 1) % 8) + 1 = ((8 + 8 + 8 + 8 - 1) % 8) + 1 = (31 % 8) + 1 = 7 + 1 = 8` → Tiên Thiên 8 = Khôn ☷
- Moving line: `((year + month + day + hour - 1) % 6) + 1 = ((31) % 6) + 1 = 1 + 1 = 2`

The `((n-1) % k) + 1` form resolves the `n % k == 0` boundary WITHOUT an `if`:
the naïve `sum % 8` would yield `0` (and a reader might coerce to 1 = Kiền),
but the correct Tiên Thiên value at the boundary is **8 = Khôn, NOT 1 = Kiền**.
Phase 22's contract test cites this exact derivation.
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `struct Trigram(pub u8)` bare newtype | `#[repr(u8)] enum` with named variants + explicit discriminants | v1.5 Phase 10 (`Palace`/`FlyingStar`) | Self-documenting; serde-stable; `as u8` arithmetic preserved |
| Free-text `source_id` | `pub const SOURCE_*: &str` + CI grep guard | v1.5 Phase 10 (DEC-0023) | Provenance audit-enforced; extended in v1.7 |
| Separate reviewer ledger | Reviewer ON each entry (free-text marker) + aggregate ledger | v1.6 Phase 17 + v1.7 ADR-0005 | Per-entry reviewer survives corpus reorder; ledger is aggregate view |
| `Option<T>` additive DTO fields | Same (unchanged) | v1.2 → v1.7 | Reserved `*_en` fields follow RIT-13 `body_en` exactly |

**Not deprecated, but pinned by ADR-0005:** the `vi_name` / `*_tu` naming (language marker `vi_` at front for content; romanized VN technical terms unmarked) DIVERGES from rituals' `body` / `body_en` (suffix) pattern. ADR-0005 must document this divergence so future maintainers don't "fix" it.

## Open Questions

1. **KingWenHexagram full-enum vs struct newtype** — CONTEXT.md discretion. Recommendation is struct newtype (`pub struct KingWenHexagram(pub u8)` + `const fn new(n: u8) -> Option<Self>`); the planner may choose the 64-variant enum if it prefers compile-checked named variants in the composition table. Both satisfy "no From between them".
   - What we know: both compile; both serde-serialize.
   - What's unclear: whether the composition-table readability gain from 64 named variants justifies ~70 lines of enum.
   - Recommendation: struct newtype (lighter); the composition table's `(TienThienTrigram, TienThienTrigram)` tuples already carry the readable names.

2. **Module path: `pub mod iching;` vs `pub mod reasoning::iching;`** — CONTEXT.md discretion. Recommendation is crate-root `pub mod iching;` (mirrors `rituals/`); the planner may nest under `reasoning/` to match EXPANSION_FRAMEWORK §2.2 wording.
   - What we know: both compile; the evaluator (Phase 24) implements `ActionEvaluator` regardless.
   - Recommendation: crate-root for the corpus+schema+casting; let Phase 24's evaluator live under `reasoning/`.

3. **DEC-NNNN numbering for the three new ADRs** — DEC-0026/0027/0028 (next safe) vs DEC-0028/0029/0030 (if backfilling v1.6 ADR-0003a/0004 as DEC-0026/0027).
   - What we know: MILESTONES.md table registers only DEC-0023/24/25; v1.6 ADRs are unregistered.
   - Recommendation: use DEC-0026/0027/0028 for ADR-0005/0006/0007 (do NOT backfill v1.6 — that's a separate cleanup); note the v1.6 gap in the ADR-0005 body or leave it.

4. **`HexagramEntry` upper/lower_trigram: `HauThienTrigram` enum vs `Palace` reuse** — confirmed distinct types (reusing `Palace` would re-open CRIT-3 by making trigram interchangeable with palace-layout). New `HauThienTrigram` enum with the SAME numbers as `Palace`.
   - What we know: `Palace` has 9 variants (includes `Center = 5`); `HauThienTrigram` has 8 (no center). They overlap on 8 of 9 numbers but are semantically distinct (trigram identity vs palace position).
   - Recommendation: new enum (do NOT alias to `Palace`).

## Sources

### Primary (HIGH confidence — in-repo anchors, opened and verified)

- `crates/amlich-core/src/semantic_graph/ontology.rs:3-43, 89-121, 159-228, 230-301, 336-411` — confirmed the 6-slice locations + the Phase 19 Offering/RecommendsOffering precedent + the v1.5/v1.6 test templates at lines 309-333.
- `crates/amlich-core/src/reasoning/types.rs:3-7, 132-142` — confirmed `ActionId` (1 variant) + `ReasoningEvidenceSourceFamily` (7 variants) live here, NOT in separate files; `rg` confirmed NO exhaustive match blocks on either.
- `crates/amlich-core/src/sources.rs:7-26, 41-56` — confirmed 7 existing `pub const SOURCE_*` + the `all_constants_have_expected_values` test to extend.
- `crates/amlich-core/tests/source_id_guard.rs:13-21` — confirmed `FORBIDDEN_LITERALS` escaped-quote format + the `#[cfg(test)]` / `//`-comment skip logic.
- `crates/amlich-core/src/almanac/fengshui/types.rs:15-43, 70-91` — confirmed `Palace` + `FlyingStar` `#[repr(u8)] enum + explicit discriminants` precedent for the three newtypes.
- `crates/amlich-core/src/almanac/fengshui/golden.rs:85-95` — confirmed `DeferralMarker { reason, expected_review_date, assigned_to: Option<String> }` reusable verbatim.
- `crates/amlich-core/tests/fengshui_crit3_isolation.rs:14-44` — confirmed `FORBIDDEN_TYPE_NAMES` grep-guard pattern (template for ADR-0007's sibling `tests/thai_tue_cross_link_crit3.rs` in Phase 23).
- `.planning/adrs/0001-ritual-schema-v1.md` + `.planning/adrs/0004-daily-phi-tinh-starting-star-convention.md` — confirmed Nygard short-form ADR template (Title/Status/Context/Decision/Consequences); ADR-0004 is the closest analog to ADR-0006 (both lock a *convention* with a citation-deferral note).
- `.planning/MILESTONES.md:275-279` — confirmed ADR Cross-References table format + DEC-0023/24/25 as highest registered IDs.

### Secondary (MEDIUM confidence — external classical reference)

- `vi.wikipedia.org/wiki/Mai_Hoa_Dịch_số` (accessed 2026-07-16) — confirms verbatim the Tiên Thiên numbering ("Càn = 1; Đoài = 2; Ly = 3; Chấn = 4; Tốn = 5; Khảm = 6; Cấn = 7; Khôn = 8"), the "trừ 8" / "trừ 6" casting convention (matches `((n-1)%k)+1`), the lunar-input convention, the Hậu Thiên Lo Shu numbering (different from Tiên Thiên — validates CRIT-3), AND names the exact Vietnamese edition the project cites (*Mai Hoa Dịch số*, Thiệu Khang Tiết, dịch giả Văn Tùng, NXB Văn Hoá Thông tin, Hà Nội, 2002). HIGH confidence on the numbering (matches CONTEXT.md + SUMMARY.md); MEDIUM on the "sub-school divergence" (article does not name a divergent school — STATE.md's open question stands but the dominant convention is unambiguous).

### Tertiary (LOW confidence — needs validation during ADR-0006 authoring)

- Exact page number in the Thiệu Khang Tiết edition for the Tiên Thiên arrangement — vi.wikipedia cites the edition by title + publisher + year + translator but not page; ADR-0006's `PendingExternalReview` page-deferral note (mirroring ADR-0004) covers this gap. The algorithm is unaffected.
- The 64-tuple composition table entries — hand-authored from the classical King Wen sequence; the bijectivity test is the correctness proof. Not a research question (data-entry task for the planner).

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — zero new deps; every integration point opened in the v1.5/v1.6 codebase and confirmed.
- Architecture (newtype encoding, composition table, ontology slices): HIGH — all three patterns have direct in-repo precedents (`Palace`, `Palace::ALL`, Phase 19 Offering).
- Domain (Tiên Thiên / Hậu Thiên numbering): HIGH on Tiên Thiên (vi.wikipedia verbatim match); MEDIUM on Hậu Thiên sub-school variance (the Lo Shu assignment is pinned; a variant exists but ADR-0005's encoding pin handles it).
- Pitfalls: HIGH — all six pitfalls are mechanical (test extension, bijectivity, exhaustive match, guard fixture, probe choice, DEC numbering) with concrete pre-emptive checks.

**Research date:** 2026-07-16
**Valid until:** 2026-08-16 (30 days — stable; the classical conventions and the codebase precedents do not move fast)

---

*Phase: 20-foundation-schema-lock-source-ids-adrs-ontology*
*Research completed: 2026-07-16 — ready for `/gsd-plan-phase 20`*
