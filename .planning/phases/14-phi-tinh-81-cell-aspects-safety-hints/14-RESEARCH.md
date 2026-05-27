# Phase 14: Phi Tinh 81-cell Aspects + Safety Hints — Research

**Researched:** 2026-05-28
**Domain:** Huyền Không Phi Tinh 2-star aspects (81-cell table), danger predicates, and Ngũ-Hành element hint API in Rust
**Confidence:** HIGH for architecture and patterns; MEDIUM for classical content (81-cell aspect authoring requires domain judgment)

---

## Summary

Phase 14 ships three new artifacts in `almanac/fengshui/`: a `aspects.rs` module carrying the 81-cell star-pair aspect table, a `safety.rs` module with `is_danger_palace` and `element_hint_for_palace`, and a `compute_palace_aspects` function that threads `compute_combined_overlay`'s output into `[StarPairAspect; 9]`. The phase has zero new crate dependencies, follows the identical `OnceLock + include_str! + serde` pattern used by `golden.rs` / `stars.rs`, and reuses `SOURCE_HUYEN_KHONG` / `ReasoningEvidenceEnvelope` from the existing infrastructure.

The biggest planning risk is content — the 81 ordered pairs require digitization from *Thẩm Thị Huyền Không Học* with per-cell citation discipline. The classical source covers two-star interactions in a chapter on 飛星相遇 (flying-star encounters). Each cell needs a `name`, `ngu_hanh_relation`, `auspice`, and `original_citation` pointing to a specific chapter. The "no product names" constraint needs a corpus-level test that scans all `RemedyHint.hint_text_vi` strings against a forbidden-term list, mirroring `source_id_guard.rs`'s pattern.

The architecture is straightforward: `compute_palace_aspects(year, month, scanner)` calls `compute_combined_overlay`, then for each palace index i, looks up `lookup_star_pair_aspect(overlay.palace_overlays[i].0, overlay.palace_overlays[i].1)`. The safety predicates (`is_danger_palace`, `element_hint_for_palace`) are pure functions on `FlyingStar` with no additional runtime state.

**Primary recommendation:** Follow the `stars.rs` OnceLock + include_str! + serde pattern for the aspects JSON corpus; keep `aspects.rs` and `safety.rs` as two separate files (aspects = 81 lookup cells, safety = 9 danger/hint rows). The no-product-names test should be a CI integration test mirroring `source_id_guard.rs`.

---

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| FS-11 | `lookup_star_pair_aspect(star_a, star_b) -> StarPairAspect` for all 81 ordered pairs | `aspects.rs` module with OnceLock-loaded JSON corpus; 81 cells indexed by `(star_a as u8, star_b as u8)` ordered pair key; returns `StarPairAspect { name, ngu_hanh_relation, auspice, source_id, original_citation, confidence }` |
| FS-12 | Every `StarPairAspect` carries `source_id: "huyen-khong"`, chapter-specific `original_citation`, and `confidence` tier | `original_citation` field reuses `SourceCitation` struct from `rituals/schema.rs`; `source_id` = `SOURCE_HUYEN_KHONG` constant enforced at load; `confidence` uses a new `FsConfidenceTier` enum |
| FS-13 | `compute_palace_aspects(year, month, term_scanner) -> [StarPairAspect; 9]` from combined overlay | Calls `compute_combined_overlay(year, month, scanner)` then maps `palace_overlays[i]` tuples through `lookup_star_pair_aspect`; pure composition, no star arithmetic |
| FS-14 | `is_danger_palace(star: FlyingStar) -> bool` — true exactly for NguHoang (5) and NhiHac (2) | Simple match on `FlyingStar` discriminant; inline test; pure function on `safety.rs` |
| FS-15 | `element_hint_for_palace(star) -> Option<RemedyHint>` — Ngũ-Hành mitigation hint with classical citation; no product names | `RemedyHint { element, hint_text_vi, source_id, original_citation }` loaded from `flying_stars_safety.json`; integration test scans all `hint_text_vi` strings for forbidden product terms |
</phase_requirements>

---

## Standard Stack

### Core
| Library/Module | Version/Location | Purpose | Why Standard |
|---|---|---|---|
| `serde` + `serde_json` | workspace dependency | JSON deserialization for 81-cell aspects corpus + safety hints | Project-wide pattern; identical to `stars.rs` and `golden.rs` |
| `std::sync::OnceLock` | std | Lazy static loading of aspects and safety JSON | Project canonical pattern — every data corpus uses this |
| `include_str!` | std | Embed JSON at compile time | Same as `golden_loader.rs`, `stars.rs`; data is static at build |
| `crate::sources::SOURCE_HUYEN_KHONG` | `src/sources.rs:26` | `"huyen-khong"` constant for all `source_id` assignments | `source_id_guard.rs` CI test enforces no bare literals |
| `crate::almanac::fengshui::combined::{compute_combined_overlay, CombinedFlyingStarLayout}` | `src/almanac/fengshui/combined.rs` | Feed palace_overlays into aspects | Phase 13 deliverable — stable public API |
| `crate::almanac::fengshui::types::FlyingStar` | `src/almanac/fengshui/types.rs:72-91` | Enum for star_a / star_b parameters | Phase 10 frozen type |
| `crate::almanac::fengshui::scanner::TietKhiScanner` | `src/almanac/fengshui/scanner.rs` | Thread-through for `compute_combined_overlay` | Phase 13 deliverable |

### No New Dependencies
Phase 14 requires zero new crate additions. All needed types (`FlyingStar`, `TietKhiScanner`, `CombinedFlyingStarLayout`, `ReasoningEvidenceEnvelope`, `SOURCE_HUYEN_KHONG`) are already in the codebase from Phases 10 and 13.

---

## Architecture Patterns

### Recommended Module Structure

`mod.rs` currently states: "Phase 14 will add `aspects.rs` (81-cell star-pair aspects) and `safety.rs`."

```
crates/amlich-core/src/almanac/fengshui/
├── mod.rs          # existing — add pub mod aspects; pub mod safety; re-exports
├── types.rs        # existing — FROZEN (no changes needed)
├── annual.rs       # existing
├── monthly.rs      # existing
├── combined.rs     # existing
├── golden.rs       # existing
├── period.rs       # existing
├── scanner.rs      # existing
├── stars.rs        # existing
├── aspects.rs      # NEW — StarPairAspect type + 81-cell corpus loader + lookup + compute_palace_aspects
└── safety.rs       # NEW — is_danger_palace + RemedyHint + element_hint_for_palace
```

Data files:
```
crates/amlich-core/data/almanac/
├── flying_stars.json            # existing
├── flying_stars_base.json       # existing
├── flying_stars_golden.json     # existing
├── flying_star_aspects.json     # NEW — 81-cell aspect corpus (see Q1 below)
└── flying_stars_safety.json     # NEW — 9-row safety hints (NguHoang, NhiHac danger + Ngu-Hanh hints)
```

Integration test:
```
crates/amlich-core/tests/
└── fengshui_aspects.rs          # NEW — black-box tests for FS-11..FS-15
```

### Pattern 1: OnceLock JSON Loader (from `stars.rs` and `golden.rs`)

All corpus loading follows the established project pattern — verified from `stars.rs:15-77`:

```rust
// Source: crates/amlich-core/src/almanac/fengshui/stars.rs:15-77
const ASPECTS_JSON: &str =
    include_str!("../../../data/almanac/flying_star_aspects.json");
static ASPECTS_CORPUS: OnceLock<StarPairAspectsCorpus> = OnceLock::new();

fn load_aspects_inner() -> StarPairAspectsCorpus {
    let corpus: StarPairAspectsCorpus =
        serde_json::from_str(ASPECTS_JSON).expect("Failed to parse flying_star_aspects.json");
    validate_aspects_corpus(&corpus);
    corpus
}

pub fn aspects_corpus() -> &'static StarPairAspectsCorpus {
    ASPECTS_CORPUS.get_or_init(load_aspects_inner)
}
```

Validation must check: exactly 81 entries, each `(star_a, star_b)` pair (both 1..=9) appears exactly once, every entry has a non-empty `original_citation.title`, every entry's `source_id == SOURCE_HUYEN_KHONG`.

### Pattern 2: StarPairAspect Struct Shape

Based on the success criteria and schema-lock conventions (`deny_unknown_fields`, `Option<T>` for optional fields, `pub const SOURCE_*`):

```rust
// aspects.rs — new type (FS-11, FS-12)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StarPairAspect {
    /// Star number 1..=9 (annual star, first in ordered pair).
    pub star_a: u8,
    /// Star number 1..=9 (monthly star, second in ordered pair).
    pub star_b: u8,
    /// Classical name for this encounter (e.g., "Thủy Hỏa tương chiến").
    pub name: String,
    /// Ngũ-Hành relationship label (e.g., "khắc", "sinh", "tỳ hòa").
    pub ngu_hanh_relation: String,
    /// Whether this pair is auspicious, inauspicious, or neutral.
    pub auspice: String,
    /// Must always equal SOURCE_HUYEN_KHONG = "huyen-khong" (FS-12).
    pub source_id: String,
    /// Points to a specific chapter of Thẩm Thị Huyền Không Học (FS-12).
    pub original_citation: SourceCitation,
    /// Confidence tier for this cell's provenance (FS-12).
    pub confidence: FsConfidenceTier,
}
```

`SourceCitation` is reused from `rituals/schema.rs` (or re-declared identically in `aspects.rs`). Given the schema-lock discipline, the cleanest option is to declare a local `pub struct SourceCitation` in `aspects.rs` if `rituals/schema.rs`'s version is not pub-re-exported at crate level. The planner should verify whether `rituals::schema::SourceCitation` is accessible — if so, use `use crate::rituals::schema::SourceCitation;`. If not, declare an identical local version.

Checking `rituals/schema.rs`: `SourceCitation` is declared `pub struct SourceCitation` with `#[serde(deny_unknown_fields)]`, fields `title: String`, `publisher: Option<String>`, `edition: Option<String>`, `page: Option<String>`. It is used inside the `rituals` module. Whether it is re-exported at crate root depends on `rituals/mod.rs` and `lib.rs`. The planner must check; if not re-exported, either re-export it or duplicate it as `FsCitation` to avoid cross-module coupling.

**Recommendation:** Declare a separate `FsCitation` struct in `aspects.rs` (or a shared `fengshui_citation.rs`) rather than coupling `aspects.rs` to the rituals module. The shape is identical; the duplication is trivial.

### Pattern 3: FsConfidenceTier Enum

Analogous to `RitualConfidenceTier` in `rituals/schema.rs`:

```rust
// aspects.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FsConfidenceTier {
    Primary,         // directly from Thẩm Thị chapter
    RegionalVariant, // alternate reading from secondary source
    Synthesized,     // reconstructed from multiple partial sources
}
```

### Pattern 4: RemedyHint Struct Shape

```rust
// safety.rs — new type (FS-15)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RemedyHint {
    /// Ngũ-Hành element: "kim" | "mộc" | "thủy" | "hỏa" | "thổ"
    pub element: String,
    /// Classical advisory text in Vietnamese — NEVER product names.
    pub hint_text_vi: String,
    /// Must equal SOURCE_HUYEN_KHONG.
    pub source_id: String,
    /// Classical citation for this hint.
    pub original_citation: FsCitation,
}
```

### Pattern 5: lookup_star_pair_aspect (FS-11)

The lookup is an array scan over the 81-entry `OnceLock` corpus, keyed by `(star_a as u8, star_b as u8)`:

```rust
pub fn lookup_star_pair_aspect(star_a: FlyingStar, star_b: FlyingStar) -> StarPairAspect {
    let a = star_a as u8;
    let b = star_b as u8;
    aspects_corpus()
        .aspects
        .iter()
        .find(|asp| asp.star_a == a && asp.star_b == b)
        .cloned()
        .unwrap_or_else(|| panic!(
            "lookup_star_pair_aspect: ({a},{b}) not found — corpus invariant broken"
        ))
}
```

At load, the validator guarantees all 81 pairs exist, so the `unwrap_or_else` is a pure safety net (never fires in production). Returning owned `StarPairAspect` (cloned from static) avoids lifetime complexity; the struct is small enough that clone is negligible.

An alternative is to return `&'static StarPairAspect` — which is also valid since the corpus is `'static`. Recommendation: return `StarPairAspect` (owned) for ergonomics consistency with how `RitualEntry` refs are returned via `Vec<&'static ...>`. The planner can decide; both work.

### Pattern 6: compute_palace_aspects (FS-13)

Pure composition — calls `compute_combined_overlay` then maps each of the 9 palaces:

```rust
// aspects.rs
pub fn compute_palace_aspects(
    year: i32,
    month: u8,
    scanner: &TietKhiScanner,
) -> [StarPairAspect; 9] {
    let overlay = compute_combined_overlay(year, month, scanner);
    std::array::from_fn(|i| {
        let (annual, monthly) = overlay.palace_overlays[i];
        lookup_star_pair_aspect(annual, monthly)
    })
}
```

`std::array::from_fn` is stable since Rust 1.63. The project uses Rust 2021 edition; this is available.

### Pattern 7: is_danger_palace (FS-14)

Simple predicate, no data loading, no JSON:

```rust
// safety.rs — on FlyingStar or as a free function
pub fn is_danger_palace(star: FlyingStar) -> bool {
    matches!(star, FlyingStar::NguHoang | FlyingStar::NhiHac)
}
```

Classical tradition: star 5 (Ngũ Hoàng) and star 2 (Nhị Hắc) are the two principal danger stars. No other stars are included. This is HIGH confidence — consistent across all major Huyền Không references.

### Pattern 8: element_hint_for_palace (FS-15)

Load from `flying_stars_safety.json` via OnceLock. Returns `None` for stars with no classical mitigation hint (some stars are auspicious and have no remedy needed):

```rust
// safety.rs
pub fn element_hint_for_palace(star: FlyingStar) -> Option<RemedyHint> {
    let n = star as u8;
    safety_corpus()
        .hints
        .iter()
        .find(|h| h.star == n)
        .cloned()
}
```

### Pattern 9: No-Product-Names Test

The forbidden-term lint is an integration test mirroring `source_id_guard.rs`. It reads the safety JSON corpus and asserts no `hint_text_vi` contains product/brand terms:

```rust
// tests/fengshui_aspects.rs
#[test]
fn no_product_names_in_hint_corpus() {
    // All RemedyHint.hint_text_vi strings must not contain product brand terms.
    // Forbidden: brand names, "sản phẩm", "mua", etc.
    // Classical hint style: "Dùng vật liệu kim loại", "Đặt bát nước", not brand X.
    const FORBIDDEN_TERMS: &[&str] = &[
        "chuông gió", // too specific — only if a brand uses this exact phrase
        // ... curated list of forbidden commercial terms
    ];
    // Walk all hints, check hint_text_vi for forbidden terms.
    // Also walk aspects corpus for any advisory text fields.
}
```

**Critical design note:** The forbidden-term list must be curated at authoring time. The test structure should scan all `RemedyHint.hint_text_vi` strings and all `StarPairAspect.name` strings. The forbidden terms are not pre-defined in the codebase yet — the planner must author them. The minimal viable list is any commercial/product-name pattern that could slip into Ngũ-Hành advisory text (e.g., product brand names, commercial calls-to-action like "đặt mua", "click", etc.). Vietnamese classical text would never contain these; the test is a regression guard against accidental corpus pollution.

### Pattern 10: mod.rs Extension

Add two new `pub mod` declarations and re-exports:

```rust
// mod.rs additions (after existing lines)
pub mod aspects;
pub mod safety;

// Re-exports
pub use aspects::{compute_palace_aspects, lookup_star_pair_aspect, StarPairAspect, FsConfidenceTier};
pub use safety::{is_danger_palace, element_hint_for_palace, RemedyHint};
```

### Anti-Patterns to Avoid

- **Bare `"huyen-khong"` string at call sites** — use `SOURCE_HUYEN_KHONG`; `source_id_guard.rs` will fail otherwise.
- **Coupling `aspects.rs` to `rituals::schema::SourceCitation`** — if the schema is not re-exported at crate level, the coupling creates a brittle import path. Use a local `FsCitation` alias or re-export the type.
- **Product names in `hint_text_vi`** — classical Ngũ-Hành mitigations reference elements and object categories (kim loại, nước, cây xanh) not brand names. The no-product-names test is the guard.
- **Adding `is_danger_palace` as a method on `FlyingStar`** — `FlyingStar` is in `types.rs` (Phase 10 FROZEN). Adding a method would require `types.rs` modification. Implement as a free function in `safety.rs` instead.
- **Wiring aspects into `direction_merge.rs`** — PITFALLS CRIT-3; same boundary as Phase 13. Phase 15 handles graph wiring.
- **Returning `Option<StarPairAspect>` from `lookup_star_pair_aspect`** — the corpus validator guarantees all 81 pairs exist at load, so `Option` is wrong; panic on corpus failure (test oracle discipline, same as `star_metadata()`).
- **Non-ordered pair semantics** — the 81-cell table is ORDERED: (star_a=annual, star_b=monthly). `lookup_star_pair_aspect(1, 9)` ≠ `lookup_star_pair_aspect(9, 1)`. The planner must verify the JSON corpus keys are consistently ordered `(annual, monthly)`.

---

## Key Question Answers

### Q1: Where does the 81-cell aspect data live?

The data should live as a JSON file at `crates/amlich-core/data/almanac/flying_star_aspects.json`, loaded by `aspects.rs` via `OnceLock + include_str!`. This is the identical pattern used by `flying_stars.json` (9 star metadata rows) and `flying_stars_base.json` (9 Van table rows).

JSON structure:
```json
{
  "schema_version": "aspects-v1",
  "source": "Thẩm Thị Huyền Không Học",
  "aspects": [
    {
      "star_a": 1,
      "star_b": 1,
      "name": "Nhất Bạch gặp Nhất Bạch — Thủy thủy tỳ hòa",
      "ngu_hanh_relation": "tỳ hòa",
      "auspice": "auspicious",
      "source_id": "huyen-khong",
      "original_citation": {
        "title": "Thẩm Thị Huyền Không Học",
        "page": "Chương X — Phi Tinh tương ngộ"
      },
      "confidence": "primary"
    }
    // ... 80 more ordered pairs
  ]
}
```

The corpus is authored by digitizing the star-pair chapter(s) of *Thẩm Thị Huyền Không Học*. Each of the 81 ordered pairs (star_a ∈ 1..=9, star_b ∈ 1..=9) needs a row.

### Q2: Classical content for 81 pairs — digitization approach

The classical source *Thẩm Thị Huyền Không Học* (深氏玄空學 by 談養吾 / Đàm Dưỡng Ngô, republished in Vietnamese-accessible editions) has a chapter on 飛星相遇 (phi tinh tương ngộ — flying star encounters). Classical Huyền Không tradition categorizes each pair by:

1. **Ngũ-Hành relation** between the two stars' elements:
   - Sinh (生) — generating cycle: water→wood, wood→fire, fire→earth, earth→metal, metal→water
   - Khắc (剋) — overcoming cycle: water→fire, fire→metal, metal→wood, wood→earth, earth→water
   - Tỳ hòa (比和) — same element: both water, both wood, etc.
   - (Some frameworks distinguish: bị sinh = being generated; bị khắc = being overcome)

2. **Auspice** — whether the encounter is auspicious (吉, cát), inauspicious (凶, hung), or conditional.

Star elements from `flying_stars.json` (already in codebase):
- 1 (Nhất Bạch): water (thủy)
- 2 (Nhị Hắc): earth (thổ)
- 3 (Tam Bích): wood (mộc)
- 4 (Tứ Lục): wood (mộc)
- 5 (Ngũ Hoàng): earth (thổ)
- 6 (Lục Bạch): metal (kim)
- 7 (Thất Xích): metal (kim)
- 8 (Bát Bạch): earth (thổ)
- 9 (Cửu Tử): fire (hỏa)

A basic mechanical derivation of the 81 pairs by element relationship (usable as a first-pass digitization skeleton):

| star_a element → star_b element | relation |
|---|---|
| same element | tỳ hòa |
| star_a generates star_b | sinh |
| star_b generates star_a | bị sinh |
| star_a overcomes star_b | khắc |
| star_b overcomes star_a | bị khắc |

However, the classical source assigns specific named aspects that go beyond pure element arithmetic — star numbers carry additional meaning (e.g., 1+6 or 1+8 are especially auspicious in Huyền Không because of their number resonance even beyond element logic). The planner should use element arithmetic as a skeleton and cite specific chapter locations for classical overrides.

**Confidence note:** The element-relationship skeleton (81 cells × ngu_hanh_relation) is HIGH confidence from `flying_stars.json` element data. The classical name and specific auspice call for each cell requires the classical text — this is MEDIUM confidence without the physical book. For Phase 14, the planner should author the corpus with `confidence: "primary"` only for pairs where the classical text is directly cited, and `confidence: "synthesized"` for element-formula-derived cells without chapter-level citation.

### Q3: StarPairAspect and RemedyHint struct shapes

Confirmed from the analysis above:

**StarPairAspect:**
```rust
pub struct StarPairAspect {
    pub star_a: u8,            // 1..=9
    pub star_b: u8,            // 1..=9
    pub name: String,          // classical encounter name
    pub ngu_hanh_relation: String, // "tỳ hòa" | "sinh" | "bị sinh" | "khắc" | "bị khắc"
    pub auspice: String,       // "auspicious" | "inauspicious" | "conditional"
    pub source_id: String,     // always SOURCE_HUYEN_KHONG
    pub original_citation: FsCitation, // specific chapter
    pub confidence: FsConfidenceTier,
}
```

**RemedyHint:**
```rust
pub struct RemedyHint {
    pub element: String,       // "kim" | "mộc" | "thủy" | "hỏa" | "thổ"
    pub hint_text_vi: String,  // classical advisory — no product names
    pub source_id: String,     // SOURCE_HUYEN_KHONG
    pub original_citation: FsCitation,
}
```

Both use `#[serde(deny_unknown_fields)]` per project schema-lock discipline.

### Q4: How compute_palace_aspects threads the combined overlay

`CombinedFlyingStarLayout.palace_overlays` is `[(FlyingStar, FlyingStar); 9]` where `palace_overlays[i] == (annual_layout.palaces[i], monthly_layout.palaces[i])`.

`compute_palace_aspects` maps each index:
- Palace 0 (N, Lo Shu 1): annual=`overlays[0].0`, monthly=`overlays[0].1` → aspect lookup
- Palace 1 (SW, Lo Shu 2): annual=`overlays[1].0`, monthly=`overlays[1].1` → aspect lookup
- ... for all 9 palaces

The returned `[StarPairAspect; 9]` is indexed in `Palace::ALL` order (same as `FlyingStarLayout.palaces`). The planner should document this indexing in the `compute_palace_aspects` doc comment.

### Q5: No-product-names constraint — implementation

The constraint is tested by a dedicated integration test in `tests/fengshui_aspects.rs`. The pattern mirrors `source_id_guard.rs` (file-scan approach) or `recommendation_safety_policy.rs` (runtime-corpus scan approach). 

**Recommended approach:** Runtime corpus scan (like `recommendation_safety_policy.rs`), not file scan. The test loads all `RemedyHint` entries via `element_hint_for_palace` for all 9 stars, then asserts `hint_text_vi` contains none of a curated `FORBIDDEN_PRODUCT_TERMS` list. The list is empty to start but the test structure is the guard. Example:

```rust
const FORBIDDEN_PRODUCT_TERMS: &[&str] = &[
    "đặt mua",
    "click",
    "sản phẩm",
    // add specific brand names if they appear in the corpus during authoring
];
```

This is future-proof: if a corpus edit accidentally introduces a product reference, the test turns RED.

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---|---|---|---|
| OnceLock corpus loading | Custom static initialization | `OnceLock::get_or_init` + `include_str!` + `serde_json::from_str` | Project canonical pattern (stars.rs, golden.rs, corpus.rs) |
| source_id enforcement | Runtime string equality checks | Compile-time `SOURCE_HUYEN_KHONG` constant at every `source_id` assignment | `source_id_guard.rs` CI test fails on bare literals |
| 81-pair validation | Manual spot-checks | Load-time validator: 81 entries, each pair appears once, all star numbers 1..=9 | Mirrors `validate_van_table()` pattern from period.rs |
| Star element lookup | Re-derive element per star | `star_metadata(star).element` from existing `stars.rs` loader | Already present, test-covered, and authoritative |
| No-product-names guard | Code review only | Integration test scanning `hint_text_vi` with `FORBIDDEN_PRODUCT_TERMS` list | Regression guard survives corpus edits; mirrors source_id_guard pattern |

---

## Common Pitfalls

### Pitfall 1: Modifying types.rs to add FlyingStar methods
**What goes wrong:** Adding `impl FlyingStar { pub fn is_danger() -> bool }` requires touching `types.rs` (Phase 10 FROZEN).
**Why it happens:** Natural object-oriented instinct — add method to the type.
**How to avoid:** Implement `is_danger_palace` as a free function in `safety.rs`. `types.rs` is schema-locked; the `mod.rs` comment says "FIELD SET LOCKED — any changes require a superseding ADR."
**Warning signs:** Any edit to `almanac/fengshui/types.rs` is a red flag for Phase 14.

### Pitfall 2: Returning Option from lookup_star_pair_aspect
**What goes wrong:** If the corpus has a gap (pair not found), `Option::None` silently propagates and `compute_palace_aspects` panics downstream on unwrap.
**Why it happens:** Defensive coding instinct.
**How to avoid:** Panic at lookup time with a clear message ("corpus invariant broken"). The load-time validator guarantees all 81 pairs exist; `Option` implies uncertain coverage that doesn't exist here (same pattern as `star_metadata()`).
**Warning signs:** An `Option<StarPairAspect>` signature on `lookup_star_pair_aspect`.

### Pitfall 3: Unordered pair semantics (star_a ↔ star_b swap)
**What goes wrong:** (annual=1, monthly=9) produces a different classical interpretation than (annual=9, monthly=1), but the corpus is authored with reversed order or the lookup swaps the arguments.
**Why it happens:** 81-cell tables are sometimes presented in matrix form where rows=host star, columns=guest star — order convention varies by source.
**How to avoid:** Canonicalize in `compute_palace_aspects`: `star_a = annual_star` (host/宫主), `star_b = monthly_star` (visiting/客). Document this in the `StarPairAspect` doc comment and the JSON schema. The corpus validator should assert `star_a != star_b` is NOT required (same-star diagonal is valid: 1+1, 2+2, etc.) but each ordered pair appears exactly once.
**Warning signs:** Only 45 entries in the corpus (treating the table as symmetric).

### Pitfall 4: Coupling aspects.rs to rituals::schema::SourceCitation
**What goes wrong:** Import of `crate::rituals::schema::SourceCitation` creates a cross-module dependency between the fengshui and rituals pillars, which "share no code paths" per SUMMARY.md.
**Why it happens:** Convenient reuse of an identical struct.
**How to avoid:** Declare `pub struct FsCitation` locally in `aspects.rs` (or a shared `fengshui/citation.rs`). The two pillars reconverge only at Phase 15 (semantic graph wiring).
**Warning signs:** `use crate::rituals::schema::SourceCitation` in any fengshui module.

### Pitfall 5: Product names entering the hint corpus
**What goes wrong:** A classical advisory like "đặt vật kim loại" gets paraphrased to "đặt chuông gió phong thủy [BrandX]" during content authoring.
**Why it happens:** Content authors familiar with commercial feng shui products may unconsciously use product language.
**How to avoid:** The integration test with `FORBIDDEN_PRODUCT_TERMS` is the CI gate. Additionally, the provenance authoring guideline should state: hints must reference element categories and object classes only (kim loại, nước, thực vật, màu sắc) — never specific products, brands, or purchase recommendations.
**Warning signs:** Any `hint_text_vi` containing specific product names, URLs, or commercial calls-to-action.

### Pitfall 6: Bare "huyen-khong" string in aspects.rs or safety.rs production code
**What goes wrong:** `source_id_guard.rs` integration test fails on next CI run.
**Why it happens:** Forgetting the SOURCE_HUYEN_KHONG constant, or writing `"huyen-khong"` directly in a struct constructor.
**How to avoid:** Always use `crate::sources::SOURCE_HUYEN_KHONG.to_string()` at struct construction sites. The JSON corpus files use `"huyen-khong"` as a value (that's the literal in the data file), which is allowed; the guard only scans `.rs` source files.
**Warning signs:** `source_id_guard.rs` test failure on first run after adding aspects.rs/safety.rs.

### Pitfall 7: Missing PITFALLS CRIT-3 boundary (direction_merge.rs)
**What goes wrong:** A Phase 15 reviewer finds `aspects.rs` imports from `crate::interaction`.
**Why it happens:** Phase 15 wires `FlyingStar` into the semantic graph — if Phase 14 anticipates this, it may add pre-wiring imports.
**How to avoid:** `aspects.rs` and `safety.rs` must follow the same boundary as all other fengshui modules — zero imports from `crate::interaction`. Phase 15 is the join point.

---

## 81-Cell Corpus Authoring Strategy

The 81-cell aspect table is the highest-effort authoring task in Phase 14. The recommended authoring approach:

**Step 1 — Element skeleton (HIGH confidence):**
Using `flying_stars.json` element data (water=1, earth=2,5,8, wood=3,4, metal=6,7, fire=9), derive the Ngũ-Hành relation for all 81 pairs mechanically:
- Same element → `tỳ hòa`
- star_a generates star_b → `sinh` (auspice: auspicious in most cases)
- star_b generates star_a → `bị sinh` (auspice: context-dependent)
- star_a overcomes star_b → `khắc` (auspice: inauspicious)
- star_b overcomes star_a → `bị khắc` (auspice: inauspicious for star_a's palace)

**Step 2 — Classical overrides (MEDIUM confidence, cite chapter):**
Specific star pairs have classical names and exceptional auspice that override the element formula:
- 1+6, 6+1 (Thủy+Kim): highly auspicious — metal generates water, Thiên Môn khai (天門開)
- 1+8, 8+1 (Thủy+Thổ): mixed — earth overcomes water but 1+8 has classical positive reading
- 8+9, 9+8 (Thổ+Hỏa): highly auspicious in Vận 9 — fire generates earth
- 2+5, 5+2 (Thổ+Thổ): highly inauspicious — double earth danger
- 2+9, 9+2 (Thổ+Hỏa): fire generates earth but with 2 = NhiHac danger
- Any pair involving 5 or 2: elevated danger flag per FS-14

**Step 3 — Citation discipline:**
All cells cite *Thẩm Thị Huyền Không Học*, chapter on 飛星相遇. Cells derived from element formula (no specific chapter quotation) use `confidence: "synthesized"`. Cells from a specific chapter passage use `confidence: "primary"`.

**Minimum viable corpus for Phase 14:**
- All 81 cells populated with at least name + ngu_hanh_relation + auspice
- At least 9 cells (one per annual star) have `confidence: "primary"` with specific chapter citation
- Remaining 72 cells may be `confidence: "synthesized"` (element-derived)
- Load-time validator confirms completeness

---

## Validation Architecture

`workflow.nyquist_validation` is not present in `.planning/config.json` (config has `workflow.research`, `workflow.plan_check`, `workflow.verifier` but not `nyquist_validation`). Therefore, the Validation Architecture section format is: describe the test approach but omit the formal "nyquist_validation" table.

### Test Framework
| Property | Value |
|---|---|
| Framework | Built-in Rust `cargo test` |
| Config file | None (default harness) |
| Quick run command | `cargo test -p amlich-core --test fengshui_aspects` |
| Full suite command | `cargo test -p amlich-core` |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command |
|--------|----------|-----------|-------------------|
| FS-11 | `lookup_star_pair_aspect(1,9)` returns valid `StarPairAspect` | unit (white-box in aspects.rs) | `cargo test -p amlich-core --lib fengshui::aspects` |
| FS-11 | All 81 pairs are accessible | unit | `cargo test -p amlich-core --lib fengshui::aspects::test_all_81_pairs_accessible` |
| FS-12 | Every aspect has `source_id == "huyen-khong"` and non-empty citation | unit | `cargo test -p amlich-core --lib fengshui::aspects::test_source_id_discipline` |
| FS-13 | `compute_palace_aspects(2024, 1, &scanner)` returns `[StarPairAspect; 9]` with correct star pairs | integration (black-box) | `cargo test -p amlich-core --test fengshui_aspects` |
| FS-14 | `is_danger_palace(NguHoang)` = true, `is_danger_palace(NhiHac)` = true, all others = false | unit | `cargo test -p amlich-core --lib fengshui::safety` |
| FS-15 | `element_hint_for_palace(NguHoang)` returns `Some(RemedyHint)` with non-empty hint | unit | `cargo test -p amlich-core --lib fengshui::safety` |
| FS-15 (no product names) | All `hint_text_vi` strings pass forbidden-term scan | integration | `cargo test -p amlich-core --test fengshui_aspects::no_product_names_in_hint_corpus` |

### Wave 0 Gaps (files that do not yet exist)

- [ ] `crates/amlich-core/src/almanac/fengshui/aspects.rs` — covers FS-11, FS-12, FS-13
- [ ] `crates/amlich-core/src/almanac/fengshui/safety.rs` — covers FS-14, FS-15
- [ ] `crates/amlich-core/data/almanac/flying_star_aspects.json` — 81-cell corpus
- [ ] `crates/amlich-core/data/almanac/flying_stars_safety.json` — 9-row safety hints
- [ ] `crates/amlich-core/tests/fengshui_aspects.rs` — black-box integration tests

---

## Open Questions

1. **SourceCitation sharing between fengshui and rituals modules**
   - What we know: `rituals/schema.rs` declares `pub struct SourceCitation` with `deny_unknown_fields`. It is used inside the rituals module. `aspects.rs` needs an identical struct.
   - What's unclear: Whether `rituals::schema::SourceCitation` is re-exported at crate root level (checking `rituals/mod.rs` and `lib.rs` is required before planning).
   - Recommendation: If not re-exported, declare `pub struct FsCitation` in `aspects.rs` (identical shape, different name to avoid confusion). If the planner finds it re-exported, they can use it directly, but document the cross-pillar dependency.

2. **Classical content completeness for all 81 pairs**
   - What we know: The element-relationship matrix gives a mechanical skeleton for all 81 cells. Specific classical names and auspice overrides require the physical text of *Thẩm Thị Huyền Không Học*.
   - What's unclear: Whether the full 81-pair chapter is accessible enough to author `confidence: "primary"` citations for all cells, or whether most cells will need `confidence: "synthesized"`.
   - Recommendation: Plan for a mixed corpus — skeletal `synthesized` entries for element-derived cells, `primary` citations for the ~20 classically named encounters. This is compatible with FS-12 (which only requires that *an* `original_citation` exists, not that all cells are `primary`).

3. **RemedyHint coverage — which stars get hints**
   - What we know: FS-15 returns `Option<RemedyHint>`, so not every star needs a hint. Stars 2 and 5 (danger stars) definitely need mitigation hints. Auspicious stars (1, 4, 8, 9) may not need mitigations.
   - What's unclear: Classical treatment — does the tradition prescribe element hints for all 9 stars, or only for inauspicious/danger ones?
   - Recommendation: Provide hints for at least stars 2, 3, 5, 7 (inauspicious per `flying_stars.json`'s `auspice` field) and return `None` for stars 1, 4, 6, 8, 9 (auspicious). This can be extended post-v1.5.

4. **`std::array::from_fn` availability**
   - What we know: `std::array::from_fn` is stable from Rust 1.63 (released 2022-08-11). The project uses Rust 2021 edition; Rust stable toolchain should be well past 1.63.
   - What's unclear: The exact Rust toolchain version pinned in `rust-toolchain.toml` or similar.
   - Recommendation: Check for `rust-toolchain.toml` in the repo root. If the MSRV is 1.63+, use `std::array::from_fn`. If uncertain, use a manual `for` loop to fill a `[MaybeUninit<StarPairAspect>; 9]` — but `from_fn` is almost certainly available.

---

## Sources

### Primary (HIGH confidence)
- `crates/amlich-core/src/almanac/fengshui/mod.rs` — explicitly states "Phase 14 will add aspects.rs (81-cell star-pair aspects) and safety.rs"
- `crates/amlich-core/src/almanac/fengshui/combined.rs` — `CombinedFlyingStarLayout.palace_overlays: [(FlyingStar, FlyingStar); 9]` — Phase 13 deliverable, verified current
- `crates/amlich-core/src/almanac/fengshui/types.rs` — `FlyingStar` enum variants and their `as u8` values confirmed; Phase 10 FROZEN
- `crates/amlich-core/src/almanac/fengshui/stars.rs` — OnceLock + include_str! + validate pattern for star metadata; canonical model for aspects.rs
- `crates/amlich-core/src/almanac/fengshui/golden.rs` — KnownDivergence + OnceLock + validation pattern; canonical model for aspects corpus loader
- `crates/amlich-core/data/almanac/flying_stars.json` — star element/polarity/auspice data; used to derive Ngũ-Hành relations for 81-cell skeleton
- `crates/amlich-core/src/sources.rs` — `SOURCE_HUYEN_KHONG = "huyen-khong"` confirmed
- `crates/amlich-core/src/rituals/schema.rs` — `SourceCitation` struct shape; `RitualConfidenceTier` pattern for `FsConfidenceTier`
- `crates/amlich-core/src/reasoning/types.rs:144-151` — `ReasoningEvidenceEnvelope` struct (used in aspects evidence if needed)
- `crates/amlich-core/tests/source_id_guard.rs` — forbidden literal guard pattern; model for no-product-names test
- `.planning/REQUIREMENTS.md` — FS-11 through FS-15 formal definitions
- `.planning/ROADMAP.md` — Phase 14 success criteria (5 items)
- `.planning/STATE.md` — Phase 13 complete, decisions accumulated; fengshui module architecture confirmed

### Secondary (MEDIUM confidence)
- `.planning/research/SUMMARY.md` — "no new crate dependencies" policy; pillar separation (fengshui ↔ rituals share no code paths)
- `.planning/research/PITFALLS.md` — CRIT-3 (direction_merge.rs boundary), MOD-5 (evidence envelopes), CRIT-1 (schema-lock)
- `.planning/adrs/0001-ritual-schema-v1.md` — `deny_unknown_fields` discipline; `SourceCitation` field shape reference
- Classical tradition notes on star elements and Ngũ-Hành relations: derived from `flying_stars.json` element data + standard Huyền Không framework (element generating/overcoming cycles); HIGH confidence for element arithmetic, MEDIUM for specific classical named encounters without book access

### Tertiary (LOW confidence — flagged for validation)
- Classical 81-pair aspect content (names, specific auspice per pair beyond element mechanics): requires verification against the physical *Thẩm Thị Huyền Không Học* text; the element-derived skeleton is a placeholder until chapter-level citations are authored.

---

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — zero new deps; all integration points verified against current codebase
- Architecture (module structure, API signatures, loader pattern): HIGH — directly derived from Phase 13 deliverables and mod.rs comment
- Classical content (81-cell aspect values, RemedyHint text): MEDIUM — element skeleton HIGH confidence; classical named encounters and specific auspice calls require book-level verification
- Pitfalls: HIGH — all derived from existing project pitfall catalogue (PITFALLS.md) and observed patterns in Phase 10/13 implementations

**Research date:** 2026-05-28
**Valid until:** 2026-06-28 (stable domain; Phase 13 API is locked)
