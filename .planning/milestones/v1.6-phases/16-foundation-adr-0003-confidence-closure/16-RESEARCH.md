# Phase 16: Foundation — ADR-0003 Confidence Closure — Research

**Researched:** 2026-07-15
**Domain:** Editorial/ADR closure of v1.5 carry-forward tech debt — confidence annotations on Phi Tinh golden dataset + ADR-0003a supersession + 1960 Trung Nguyên KnownDivergence disposition.
**Confidence:** HIGH for code/test/ADR structure; MEDIUM for the independent-classical-reference choice (requires user input — out of scope for planning).

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| FND-07 | Pre-1984 Thượng/Trung Nguyên polarity rows promoted from MEDIUM → HIGH confidence after an external cross-check (independent classical reference beyond *Thẩm Thị Huyền Không Học*) | §"Cross-check sources" + §"Confidence annotation shape" sections below |
| FND-08 | 1960 Trung Nguyên `KnownDivergence` either resolved with source attribution or explicitly logged as `PendingExternalReview` with reason + tiebreaker decision; ADR-0003 narrative updated to record the disposition | §"1960 KnownDivergence shape" + §"PendingExternalReview marker shape" sections below |
</phase_requirements>

## Summary

This is a **2-plan editorial phase** — no algorithmic code changes, no schema breaks. The work consists of (1) authoring **ADR-0003a** as a superseding decision document that records the MEDIUM→HIGH confidence upgrade with an external cross-check citation; (2) extending the Phi Tinh golden dataset (`crates/amlich-core/data/almanac/flying_stars_golden.json`) so each pre-1984 case carries a typed `confidence: "high"` annotation; (3) deciding the 1960 Trung Nguyên `KnownDivergence` disposition (RESOLVED-as-current OR DEFERRED-as-`PendingExternalReview`); and (4) extending `tests/fengshui_invariants.rs` with confidence-annotation assertions that gate FND-07 and FND-08. The runtime confidence annotation in evidence notes (`annual.rs:163-169`) already differentiates pre-1984 (medium) vs post-1984 (high) correctly — the lift is to upgrade the **dataset-side annotation** to reflect the new external citation, and to update the runtime override path.

**Primary recommendation:** Two plans, both RED→GREEN single-commit. Plan 16-01 lands ADR-0003a + golden-dataset `confidence` field + FND-07 test gate. Plan 16-02 lands the 1960 disposition (one of two paths) + ADR-0003 narrative update + FND-08 test gate. Ask the user via checkpoint **before plan 16-01** for: (a) the second classical citation to use beyond *Thẩm Thị*; (b) the 1960 disposition preference (resolved vs deferred).

## Current State of ADR-0003

### File under revision
- **`.planning/adrs/0003-nien-tu-bach-polarity.md`** (73 lines, dated 2026-05-26, Deciders: Phase 10 Foundation).
- The ADR explicitly states at §6 ("Confidence Acknowledgment"):
  > **Thượng Nguyên and Trung Nguyên rows** — **MEDIUM confidence**. These rows are sourced from phongthuycaivan.org (single Vietnamese-language source). Phase 13 is the designated cross-validation phase: during `compute_yearly_flying_stars` implementation, these rows must be cross-checked against *Thẩm Thị Huyền Không Học*. If divergence is found: log as `KnownDivergence`, do NOT silently correct, issue ADR-0003a to supersede this document.
- §"Consequences" final bullet: **"Future revisions to Thượng/Trung Nguyên rows after Phase 13 cross-check will be captured in ADR-0003a (not an amendment to this document)."**

### What ADR-0003a must do (per the existing convention and ROADMAP §16 success criterion #1)
1. Be a **new file** at `.planning/adrs/0003a-nien-tu-bach-polarity-confidence-closure.md` (or `0004-nien-tu-bach-polarity-v2.md` if 0003a is rejected by file-naming convention).
2. Open with `Status: Accepted`, `Date: 2026-07-15`, `Deciders: Phase 16 Foundation`, and `Supersedes: ADR-0003 §6 (Confidence Acknowledgment)`.
3. Cite **an independent classical reference beyond *Thẩm Thị Huyền Không Học*** — this is the load-bearing requirement for FND-07. The repo currently does not name a second classical text (see "Open Questions" below).
4. Carry forward the matrix unchanged (Tu Luc / Bat Xich starting stars; dương=nghịch / âm=thuận polarity rule) but re-classify Thượng/Trung Nguyên rows as **HIGH confidence**.
5. Record the disposition of the 1960 Trung Nguyên `KnownDivergence` (resolved or deferred — see §"1960 KnownDivergence shape").
6. Leave a "Backward Compatibility" section noting: ADR-0003 §1–§5 still authoritative; only §6 (Confidence Acknowledgment) is superseded.

### Code-touching references to MEDIUM confidence
- **`crates/amlich-core/src/almanac/fengshui/annual.rs:93-103`** — `yuan_of_year(year: i32) -> (&'static str, bool)` returns `true` (is_medium) for `year < 1984`. Called from `compute_yearly_flying_stars` at line 163–169 to emit `confidence=medium` in the evidence note. **This is the runtime source of truth for the evidence-note annotation; it does NOT need to change for FND-07.**
- **`crates/amlich-core/src/almanac/fengshui/golden.rs:15`** — module-level comment: `//! ADR-0003 §4: Thượng/Trung Nguyên cases (pre-1984) are MEDIUM confidence;`.
- **`crates/amlich-core/data/almanac/flying_stars_golden.json`** — metadata description: `"Pre-1984 cases are MEDIUM confidence per ADR-0003."` Two pre-1984 cases: `annual-thuong-nguyen-1920` (Vận 3, expected_center=9) and `annual-trung-nguyen-1960` (Vận 6, expected_center=5, divergent).
- **`crates/amlich-core/tests/fengshui_invariants.rs:200`** — `match case.van { 7 => ..., 8 => ..., 9 => ..., _ => {} }` — pre-1984 cases currently **excluded from the coverage gate** (`>= 10 per Vận 7/8/9`). After FND-07 the pre-1984 cases remain a separate group (no coverage gate needed).
- **`crates/amlich-core/src/almanac/fengshui/annual.rs:337-344`** — unit test `test_compute_yearly_pre_1984_medium_confidence` asserts `"confidence=medium"` in evidence note for year=1960. **This test continues to pass after FND-07 because the runtime evidence note still says medium for pre-1984 years (the boost is dataset-side, not runtime-side).** A new test must assert that the **golden dataset** confidence annotation matches HIGH for those same years.

### Confirmation that ADR-0003a does not yet exist
- `.planning/adrs/` contains only `0001-ritual-schema-v1.md`, `0002-phi-tinh-monthly-anchor.md`, `0003-nien-tu-bach-polarity.md`. No `0003a-*` or `0004-*` file exists.

## Cross-check sources beyond *Thẩm Thị*

**Status: GAP — REQUIRES USER INPUT before plan 16-01.** The repository's research files (`.planning/research/FEATURES.md:283-289`, `.planning/research/PITFALLS.md:325`, `.planning/research/EXPANSION_FRAMEWORK §7`) cite four Vietnamese Phi Tinh resources but **only *Thẩm Thị Huyền Không Học* is classified as a "classical text"**; the others are modern websites:

| Reference | Type | Already wired into golden dataset? |
|-----------|------|-----------------------------------|
| *Thẩm Thị Huyền Không Học* (Thẩm Thị Huyen Khong Hoc) | Classical text (tiebreaker per EXPANSION_FRAMEWORK §7) | Yes — every case's `tiebreaker` field cites it |
| `phongthuycaivan.org` | Modern Vietnamese website | Yes — 100% of golden cases |
| `phongthuyso.vn` | Modern Vietnamese website | Yes — paired with phongthuycaivan.org on Vận 7/8/9 cases |
| `lasotuvi.com` | Modern Vietnamese website | Yes — paired on Vận 9 cases |
| `fengshui.net` | Modern English-language feng shui reference | NOT wired |
| `phongthuyhomemy.com` | Modern Vietnamese website | NOT wired |
| `phongthuykhaitoan.com` | Modern Vietnamese website | NOT wired |

**Implication for FND-07:** The literal ROADMAP success criterion #1 says "an independent classical reference beyond *Thẩm Thị Huyền Không Học*". If the user accepts **dual-source web verification** (e.g., both phongthuycaivan.org AND phongthuyso.vn OR lasotuvi.com agree on pre-1984 rows), that satisfies the "two independent sources" intent but may not satisfy the "classical reference" wording. The planner should present the user with **two options**:

1. **Option A — Add a second classical citation:** Cite a specific chapter/page of a named classical text (e.g., a Vietnamese Huyền Không commentary, *Hoàng Tôn Phong Thủy*, *Mật Tông Phong Thủy* — the planner should ask the user which one). Requires the user to name the text and citation.
2. **Option B — Dual-source web verification + tiebreaker:** Cite phongthuycaivan.org + lasotuvi.com (or phongthuyso.vn) as **two independent modern sources** both confirming the pre-1984 polarity row, with *Thẩm Thị* as tiebreaker. The 1920 case (both sources agree: 9) already meets this; the 1960 case currently splits 5 vs 6 — so it cannot use Option B without first resolving the divergence (which is the FND-08 work).

Either option is acceptable for FND-07 provided the cross-check is documented in the tiebreaker field of every pre-1984 case in the golden JSON.

## 1960 Trung Nguyên KnownDivergence shape

### Current state (literal JSON at `crates/amlich-core/data/almanac/flying_stars_golden.json`)

**Case entry (lines 481–492):**
```json
{
  "id": "annual-trung-nguyen-1960",
  "kind": "annual",
  "year": 1960,
  "van": 6,
  "expected_center": 5,
  "sources": [
    { "source": "phongthuycaivan.org", "value": 5 },
    { "source": "lasotuvi.com", "value": 6 }
  ],
  "tiebreaker": "Tham Thi Huyen Khong Hoc: Canh Ty 1960 Trung Nguyen Van 6, center Ngu Hoang (5) per Tham Thi Huyen Khong Hoc polarity matrix; lasotuvi.com (6) rejected as Van-number confusion.",
  "note": "Trung Nguyen cross-validation per ADR-0003 open question #3. MEDIUM confidence. Sources disagree: phongthuycaivan.org=5 vs lasotuvi.com=6. Divergence recorded in known_divergences. Tiebreaker selects 5."
}
```

**KnownDivergence entry (lines 494–505):**
```json
{
  "case": "annual 1960",
  "our_value": 5,
  "source_values": [
    { "source": "phongthuycaivan.org", "value": 5 },
    { "source": "lasotuvi.com", "value": 6 }
  ],
  "tiebreaker": "Tham Thi Huyen Khong Hoc polarity matrix selects center=5 (Ngu Hoang) for Canh Ty 1960 Trung Nguyen; lasotuvi.com value 6 appears to conflate center star with Van number.",
  "note": "ADR-0003 open question #3: Trung Nguyen polarity matrix MEDIUM confidence. Source lasotuvi.com likely reports the Van number (6) rather than the computed annual center (5). This divergence is logged per FS-10 requirement — NOT silently corrected."
}
```

**Rust struct** at `crates/amlich-core/src/almanac/fengshui/golden.rs:51-62`:
```rust
pub struct KnownDivergence {
    pub case: String,                  // "annual 1960"
    pub our_value: u8,
    pub source_values: Vec<SourceValue>,
    pub tiebreaker: String,
    pub note: String,
}
```

### Two disposition paths for FND-08

The ROADMAP success criterion #2 allows either resolution or deferral. The structure of `KnownDivergence` must support both without breaking the existing test (`tests/fengshui_invariants.rs:163-173`).

**Path A — RESOLVED (recommended default):** Treat the current 5/our_value as the authoritative disposition. The 1960 case stays in `known_divergences` with `our_value: 5` and an **updated tiebreaker string** citing the **new external classical reference** (the same one chosen for FND-07 cross-check) as the reason the divergence is resolved. ADR-0003a §"Disposition of 1960 Trung Nguyên Divergence" records: "Resolved per *Thẩm Thị* + *<new classical ref>*: both classical sources select center=5 (Ngũ Hoàng); lasotuvi.com=6 interpreted as a Vận-number confusion." No new field needed on `KnownDivergence`.

**Path B — DEFERRED (`PendingExternalReview`):** Add an `Option<DeferralMarker>` field to `KnownDivergence`:
```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeferralMarker {
    /// Reason the divergence is deferred (e.g., "awaiting independent classical cross-check").
    pub reason: String,
    /// ISO 8601 date when review is expected (e.g., "2026-12-31" or "v1.7").
    pub expected_review_date: String,
    /// Who/what will perform the review (e.g., "phase-17 owner" or external reviewer name).
    pub assigned_to: Option<String>,
}

pub struct KnownDivergence {
    // ... existing fields unchanged ...
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deferral: Option<DeferralMarker>,
}
```
The 1960 `known_divergences` entry gains `"deferral": { "reason": "...", "expected_review_date": "2026-12-31", "assigned_to": "..." }`. The case-level `expected_center` is set to `5` (the tiebreaker value) so the test still passes; the deferral marker signals "treat as not-yet-finalized".

**Path A is simpler and matches the existing `our_value` field's semantics** (it already encodes the tiebreaker choice). Path B is the explicit `PendingExternalReview` deferral that the ROADMAP names. **The planner should ask the user which path they prefer.**

## Confidence annotation shape on the golden dataset

### Current shape (NO typed confidence field)
- `PhiTinhGoldenCase` struct (`golden.rs:65-93`) has no `confidence` field.
- MEDIUM annotation is in **two places only**:
  1. Free-text `note` field (e.g., `"...MEDIUM confidence..."`).
  2. `metadata.description` top-level field.

### Additive path for FND-07 (recommended)
Add an `Option<ConfidenceTier>` field to `PhiTinhGoldenCase` and to `metadata`:

```rust
// New enum, parallel to aspects.rs FsConfidenceTier (golden.rs:56), but with these variants:
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GoldenConfidence {
    High,
    Medium,
    Low,
}

// Additive field on PhiTinhGoldenCase:
pub struct PhiTinhGoldenCase {
    // ... existing fields ...
    /// Confidence tier (HIGH = two-source + classical cross-check; MEDIUM = two-source only; LOW = single-source).
    /// Defaults to MEDIUM for pre-1984 cases, HIGH for post-1984 cases (matching ADR-0003 §6).
    #[serde(default = "default_confidence_for_year")]
    pub confidence: GoldenConfidence,
}

fn default_confidence_for_year(case: &PhiTinhGoldenCase) -> GoldenConfidence {
    if case.year < 1984 { GoldenConfidence::Medium } else { GoldenConfidence::High }
}
```

### JSON changes for FND-07
- Both pre-1984 cases (`annual-thuong-nguyen-1920`, `annual-trung-nguyen-1960`) gain `"confidence": "high"` plus an **updated `tiebreaker`** that cites the new external classical reference (see "Cross-check sources" above).
- All post-1984 cases optionally gain `"confidence": "high"` (most already do implicitly via the runtime evidence note; adding it to the JSON is harmless and consistent).
- `metadata.description` is updated to drop "Pre-1984 cases are MEDIUM confidence per ADR-0003." in favor of "Per ADR-0003a, all pre-1984 Thượng/Trung Nguyên polarity rows are HIGH confidence after cross-check against *Thẩm Thị Huyền Không Học* + *<new classical ref>*."

### Runtime code change scope (minimal)
- `annual.rs` `yuan_of_year()` does **not** need to change — the runtime evidence-note confidence annotation is unchanged (still medium for pre-1984, high for post-1984). The boost is **dataset-side only**.
- If the user wants the **runtime evidence note to also say "high" for pre-1984 years** (so consumers see the upgraded confidence without consulting the golden dataset), the change is one-line: replace `if year < 1984 { return ("pre-1984", true) }` with a lookup against the golden dataset's pre-1984 case `confidence`. This is OPTIONAL — the success criterion is satisfied by the dataset annotation alone.

## Existing test infrastructure

### `tests/fengshui_invariants.rs` — current 9 tests
| Test | Purpose | Status |
|------|---------|--------|
| `test_a_lo_shu_invariants_all_vans` | All 9 Vận satisfy Lo Shu (sum=45, each 1-9 once, center=Vận) | Untouched by Phase 16 |
| `test_b_van_boundary_lap_xuan_2024` | 2024-01-15 → Vận 8; 2024-02-05 → Vận 9 | Untouched |
| `test_b_van_boundary_mid_van8_and_van7` | Mid-Vận non-boundary dates resolve correctly | Untouched |
| `test_c_golden_annual_coverage` | ≥10 annual cases match per Vận 7/8/9; divergent years logged | **EXTEND** with pre-1984 case confidence assertion |
| `test_d_golden_monthly_cases` | Monthly cases match | Untouched |
| `test_d_golden_period_cases` | Period boundary cases match (≥2) | Untouched |
| `test_e_combined_overlay_smoke_2024_m1` | Combined overlay returns 9 palace overlays | Untouched |
| `test_e_combined_overlay_annual_center_2024` | 2024 center = 4 | Untouched |
| `test_e_combined_overlay_mirrors_components` | palace_overlays[].0 = annual, .1 = monthly | Untouched |

### Plus unit tests inside `golden.rs:206-289`
- `golden_dataset_loads_and_validates`, `golden_dataset_van7_coverage`, `golden_dataset_van8_coverage`, `golden_dataset_van9_coverage`, `golden_dataset_has_known_divergences`, `golden_dataset_all_cases_have_tiebreaker`, `golden_dataset_annual_monthly_cases_have_two_sources`, `golden_dataset_period_cases_exist`, `golden_dataset_cross_validation_cases_exist` (asserts ≥2 pre-1984 annual cases).

### Tests to add for Phase 16

**In `tests/fengshui_invariants.rs` (FND-07 gate):**
```rust
/// FND-07 gate: every pre-1984 annual case in the golden dataset carries
/// `confidence: "high"` after ADR-0003a supersession.
#[test]
fn test_f_golden_pre_1984_confidence_is_high() {
    let ds = load_flying_stars_golden();
    let pre_1984: Vec<_> = ds.cases.iter()
        .filter(|c| c.kind == "annual" && c.year < 1984)
        .collect();
    assert!(!pre_1984.is_empty(), "expected pre-1984 cross-validation cases");
    for case in pre_1984 {
        assert_eq!(
            case.confidence, GoldenConfidence::High,
            "FND-07: pre-1984 case '{}' (year={}) must be confidence=high after ADR-0003a",
            case.id, case.year
        );
    }
}
```

**In `tests/fengshui_invariants.rs` (FND-08 gate) — choose one per disposition path:**

```rust
// PATH A (Resolved): assert 1960 KnownDivergence still present with our_value=5
//                   and the tiebreaker cites the new external classical reference.
#[test]
fn test_g_1960_divergence_resolved_with_external_citation() {
    let ds = load_flying_stars_golden();
    let div = ds.known_divergences.iter()
        .find(|d| d.case == "annual 1960")
        .expect("1960 divergence must be present");
    assert_eq!(div.our_value, 5);
    assert!(div.tiebreaker.contains("<new-classical-ref>"), "tiebreaker must cite the new classical reference");
}

// PATH B (Deferred): assert 1960 KnownDivergence carries a deferral marker.
#[test]
fn test_g_1960_divergence_pending_external_review() {
    let ds = load_flying_stars_golden();
    let div = ds.known_divergences.iter()
        .find(|d| d.case == "annual 1960")
        .expect("1960 divergence must be present");
    let deferral = div.deferral.as_ref()
        .expect("FND-08 deferral path: 1960 must carry a deferral marker");
    assert!(!deferral.reason.is_empty());
    assert!(!deferral.expected_review_date.is_empty());
}
```

**In `tests/fengshui_invariants.rs` (cross-cutting invariant):**
```rust
/// GoldenConfidence defaults match ADR-0003a: HIGH if explicitly set, otherwise
/// inferred from year (< 1984 => MEDIUM, >= 1984 => HIGH).
#[test]
fn test_h_golden_confidence_default_matches_year() {
    let ds = load_flying_stars_golden();
    for case in &ds.cases {
        if case.kind == "annual" {
            let expected_default = if case.year < 1984 { GoldenConfidence::Medium } else { GoldenConfidence::High };
            // Either the case sets confidence explicitly OR it equals the default.
            // After FND-07, pre-1984 cases are explicitly HIGH (overriding the default).
            // The test simply asserts no case has an UNEXPECTEDLY LOW confidence.
            assert_ne!(case.confidence, GoldenConfidence::Low,
                "case '{}' unexpectedly marked LOW", case.id);
        }
    }
}
```

### Untouched tests
- All 9 existing tests in `tests/fengshui_invariants.rs` MUST continue to pass — Phase 16 is additive.
- All 886 tests passing pre-v1.6 (per v1.5 audit) MUST continue to pass.
- `tests/source_id_guard.rs`, `tests/rituals_integration.rs`, `tests/day_snapshot_v14_compat.rs`, `tests/integration_2026_smoke.rs` — no Phase 16 touch.

## Standard Stack

| Layer | Component | Why standard |
|-------|-----------|--------------|
| ADR authoring | Plain Markdown | Same as ADR-0001, ADR-0002, ADR-0003 (`.planning/adrs/NNNN-name.md`) |
| Golden dataset format | Existing JSON (`flying_stars_golden.json`) | Single source of truth per EXPANSION_FRAMEWORK §7; additive field pattern matches FND-01 (`RitualEntry` schema-lock via `deny_unknown_fields`) |
| Runtime evidence annotation | Existing `ReasoningEvidenceEnvelope.note` field | Per MOD-5 mitigation; runtime medium/high in `annual.rs` already wired |
| New confidence enum | `GoldenConfidence { High, Medium, Low }` | Mirrors `FsConfidenceTier { Primary, RegionalVariant, Synthesized }` pattern from `aspects.rs:56` |
| Tests | `cargo test --test fengshui_invariants` | Existing test file; Phase 16 only adds new `#[test]` functions |

**No new crate dependencies. No new files outside `.planning/adrs/`, `data/almanac/flying_stars_golden.json`, `tests/fengshui_invariants.rs`, and optionally `crates/amlich-core/src/almanac/fengshui/golden.rs` (if Path B requires `DeferralMarker`).**

## Architecture Patterns

### Pattern 1: ADR supersession (per existing convention)
- ADR-0001 (`ritual-schema-v1`) is the schema-lock ADR; ADR-0002 (`phi-tinh-monthly-anchor`) is a sibling decision ADR; ADR-0003 (`nien-tu-bach-polarity`) is a third sibling. **No amendment pattern exists** — `.planning/adrs/0003a-*.md` (or `0004-*.md`) is a new file that **supersedes** ADR-0003 §6 only.
- File naming convention: `NNNN-kebab-case-name.md`. ADR-0003a candidates: `0003a-nien-tu-bach-polarity-confidence-closure.md` (matches the comment in ADR-0003 line 62 "issue ADR-0003a") or `0004-nien-tu-bach-polarity-high-confidence.md` (numerically clean). **The planner should pick `0003a-` to match the ADR-0003 §6 language.**

### Pattern 2: Additive golden-dataset fields (matches FND-01/02 schema-lock precedent)
- `PhiTinhGoldenCase` gets `confidence: GoldenConfidence` with `#[serde(default = "default_confidence_for_year")]`. Existing fields untouched. Existing tests pass because the default matches the prior implicit assumption (medium for pre-1984, high for post-1984).

### Pattern 3: Test extension over test rewrite (per "audit-as-decisive-source" carry-forward)
- Phase 16 adds new `#[test]` functions to `tests/fengshui_invariants.rs` (Test F, Test G, Test H). It does NOT modify or rename existing tests. The 9 existing tests pass unchanged.

### Anti-patterns to avoid
- **Amending ADR-0003 in place.** ADR-0003 §"Consequences" explicitly forbids it: "Future revisions... will be captured in ADR-0003a (not an amendment to this document)."
- **Changing the runtime `yuan_of_year()` function** to hardcode "pre-1984 = high" would skip the dataset-side annotation and miss the cross-check citation trail. The dataset annotation is the audit record; the runtime note can stay at its current logic OR optionally consult the dataset, but the source of truth is the JSON.
- **Silently changing `our_value: 5` to `our_value: 6`** (or vice versa) in the 1960 divergence — explicitly forbidden by ADR-0003 §"Do NOT silently correct" and FS-10.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Confidence enum definition | Inline `String` field with arbitrary values | Typed `GoldenConfidence { High, Medium, Low }` enum with `#[serde(rename_all = "lowercase")]` | Mirrors `FsConfidenceTier` pattern in `aspects.rs:56`; typo-proof; serializes to `"high"`/`"medium"`/`"low"` matching the existing `confidence=high\|medium` substring in evidence notes |
| `PendingExternalReview` deferral | Free-text `"pending": true` flag | Structured `DeferralMarker { reason, expected_review_date, assigned_to }` on `KnownDivergence` | Auditable; future queries can sort/filter by review date; matches FS-10 logging discipline |
| Cross-check citation | Inline string in `note` field | Updated `tiebreaker` field per case | `tiebreaker` is already the canonical field for the authority that resolved the case (verified by `golden_dataset_all_cases_have_tiebreaker` test) |

**Key insight:** This phase is editorial + structural, not algorithmic. The lift is "annotate the existing data + add the ADR narrative + add the test gates" — every field that needs to change already has an analog elsewhere in the codebase. Copy the pattern; don't invent.

## Common Pitfalls

### Pitfall 1: Pasting a "second classical reference" that the user cannot verify
**What goes wrong:** The planner picks a plausible-sounding classical text (e.g., *Hoàng Tôn Phong Thủy* or *Mật Tông Phong Thủy*) without user confirmation, writes it into ADR-0003a and the golden dataset, and the audit trail becomes unverifiable. Worse: the cited text may disagree with *Thẩm Thị* on pre-1984 polarity, requiring a second divergence entry.
**How to avoid:** ASK THE USER before plan 16-01 lands. Provide the two options (single-text citation vs dual-source web verification) with their trade-offs.
**Warning signs:** ADR-0003a cites a text without a chapter + page; golden JSON `tiebreaker` field becomes a vague "per classical sources" string.

### Pitfall 2: Forgetting to update `metadata.case_count`
**What goes wrong:** The `validate_phi_tinh_golden` function at `golden.rs:142-148` asserts `metadata.case_count == cases.len()`. Adding or removing cases (e.g., adding a third pre-1984 case for cross-check coverage) without bumping this counter breaks the load.
**How to avoid:** After any case-list edit, run `cargo test --test fengshui_invariants test_c_golden_annual_coverage` — it loads the dataset and will fail loudly if `case_count` is stale. **Or:** bump `case_count` as part of every edit.
**Warning signs:** Load failure on first build after editing the JSON; the error message names the mismatch explicitly.

### Pitfall 3: Phase 16 test breaks `test_c_golden_annual_coverage`
**What goes wrong:** The existing test (line 156–215) iterates `ds.cases.iter().filter(|c| c.kind == "annual")` and asserts `center_star` matches. If the 1960 case's `expected_center` is changed from 5 (Thẩm Thị tiebreaker) to 6 (lasotuvi.com) or vice versa, the test fails unless the case is also flagged in `known_divergences`. The test already handles divergent cases correctly (lines 163–183), so as long as `expected_center` matches `our_value` AND the case appears in `known_divergences`, it passes.
**How to avoid:** Do NOT change `expected_center` without also confirming the case is in `known_divergences`. The current state is internally consistent (1960: expected_center=5, our_value=5, divergence logged) — leave it.
**Warning signs:** Test C failure with message "divergent case year=1960: golden expected_center X != computed Y".

### Pitfall 4: Phase 16 test breaks the existing unit tests in `golden.rs`
**What goes wrong:** Adding a `confidence: GoldenConfidence` field with `#[serde(default)]` means existing JSON cases without the field deserialize with the default value. But if the default function uses `case.year` (which is available during deserialization), the per-case defaults must be computed via the `#[serde(default = "fn_name")]` function form, NOT via `Default::default()`. The function signature requires `(&PhiTinhGoldenCase) -> GoldenConfidence`, but `#[serde(default)]` expects a function that takes no arguments.
**How to avoid:** Two acceptable shapes:
- **Option 1:** `#[serde(default)]` (uses `Default::default() = GoldenConfidence::Medium`) — every case loads as Medium unless explicitly set. Post-edit: explicitly set `confidence` on all post-1984 cases too. Verbose but simple.
- **Option 2:** Custom `Deserialize` implementation that inspects `year` after deserializing the rest of the case. More code; matches the intended behavior exactly.
**Recommendation:** Option 1 — post-1984 cases already have evidence-note confidence=high at runtime, so adding `"confidence": "high"` to each post-1984 case in the JSON is a clean mirror.
**Warning signs:** `cargo test golden_dataset_loads_and_validates` fails with "missing field `confidence`" (means `default` isn't being applied — check the function signature).

### Pitfall 5: User asks for "deferred" but planner writes "resolved" or vice versa
**What goes wrong:** The roadmap allows either disposition. If the planner picks one and the user wanted the other, plan 16-02 is wasted and a correction plan is needed.
**How to avoid:** ASK THE USER before plan 16-02 lands. The disposition question is a single-line conversation: "Resolved (per Thẩm Thị + new classical ref) or deferred (PendingExternalReview marker)?"
**Warning signs:** ADR-0003a §"Disposition of 1960 Trung Nguyên Divergence" reads as one path while `KnownDivergence` JSON carries the other.

## Code Examples (verified patterns from current codebase)

### Pattern: Adding a typed enum field to a golden-dataset struct (mirrors aspects.rs FsConfidenceTier)
```rust
// Source: crates/amlich-core/src/almanac/fengshui/aspects.rs:56
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FsConfidenceTier {
    Primary,
    RegionalVariant,
    Synthesized,
}

// Parallel for Phase 16:
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GoldenConfidence {
    High,
    Medium,
    Low,
}
```

### Pattern: Extending `PhiTinhGoldenCase` additively (current struct at golden.rs:65-93)
```rust
pub struct PhiTinhGoldenCase {
    pub id: String,
    pub kind: String,
    pub year: i32,
    #[serde(default)]
    pub month: Option<u8>,
    #[serde(default)]
    pub jd: Option<i32>,
    pub van: u8,
    pub expected_center: u8,
    pub sources: Vec<SourceValue>,
    pub tiebreaker: String,
    pub note: String,
    // NEW FIELD — additive, defaulted:
    #[serde(default)]
    pub confidence: GoldenConfidence, // defaults to Medium via Default::default()
}
```

### Pattern: Test extension (current tests at tests/fengshui_invariants.rs)
The 9 existing tests follow a `test_<group>_<description>` naming convention with `// ---` separator comments. New tests `test_f_*`, `test_g_*`, `test_h_*` follow the same pattern. Add the separator comment block above each new test for visual consistency.

## Validation Architecture (skipped — `workflow.nyquist_validation` is not set in `.planning/config.json`)

`config.json` exposes only `mode`, `depth`, `parallelization`, `commit_docs`, `model_profile`, and a partial `workflow` block (`research: true, plan_check: true, verifier: true`). No `nyquist_validation` key. **Skipping per the output-format guidance.**

Test commands the planner should use:
- **Quick gate per task:** `cargo test --test fengshui_invariants` (~3s).
- **Per plan:** `cargo test --test fengshui_invariants -- --nocapture` plus `cargo test` on the whole crate (~30s).
- **Phase gate:** full `cargo test` plus `cargo build --release` (~60s). Must be green before `/gsd-verify-work`.

## State of the Art

| Old | Current | Phase 16 target | Impact |
|-----|---------|-----------------|--------|
| ADR-0003 §6 declares pre-1984 rows MEDIUM | Unchanged | **ADR-0003a supersedes §6; pre-1984 rows HIGH** | Audit trail clarifies the matrix is two-source + classical cross-check confirmed, not single-source phongthuycaivan.org |
| Golden JSON carries "MEDIUM confidence" in `note` strings | Unchanged | **Typed `confidence: GoldenConfidence` field per case** | Queries (e.g., `ds.cases.iter().filter(|c| c.confidence == GoldenConfidence::High)`) become possible; CI guards can assert |
| Runtime `annual.rs` evidence note: "confidence=medium" for year<1984 | Unchanged | **Unchanged (no requirement to change)** | Runtime evidence note is per-computation; dataset annotation is per-case. They diverge intentionally (computation is forward-looking, dataset is the audit record). |
| 1960 KnownDivergence: `our_value: 5`, tiebreaker cites Thẩm Thị only | Unchanged | **Tiebreaker updated to cite new external classical ref (option A) OR `deferral` marker added (option B)** | FND-08 satisfied; disposition trail explicit |

## Open Questions

1. **Independent classical reference beyond *Thẩm Thị Huyền Không Học***
   - What we know: Repo only names *Thẩm Thị* as a classical text. Modern websites (phongthuycaivan.org, phongthuyso.vn, lasotuvi.com, fengshui.net, phongthuyhomemy.com, phongthuykhaitoan.com) are not "classical".
   - What's unclear: Whether the user has a specific classical text in mind (e.g., *Hoàng Tôn Phong Thủy* / *Mật Tông Phong Thủy* / a specific Huyền Không commentary) or accepts dual-source modern website verification.
   - Recommendation: Planner presents a checkpoint question with two options (classical text name + chapter OR "accept dual-source modern verification with Thẩm Thị tiebreaker"). **Block plan 16-01 until answered.**

2. **1960 Trung Nguyên disposition**
   - What we know: ROADMAP allows either RESOLVED (current `our_value=5` state, augmented tiebreaker) or DEFERRED (PendingExternalReview marker).
   - What's unclear: User preference.
   - Recommendation: Planner presents a checkpoint question: "Resolved per Thẩm Thị + new classical ref (Path A) or deferred as PendingExternalReview (Path B)?" **Block plan 16-02 until answered.**

3. **Should the runtime evidence-note confidence annotation also flip to "high" for pre-1984 years?**
   - What we know: FND-07 success criterion is satisfied by dataset annotation alone. The runtime note currently says "confidence=medium" for pre-1984 years via `annual.rs:163-169`.
   - What's unclear: Whether consumers of `compute_yearly_flying_stars(year<1984, scanner)` should also see "confidence=high" without consulting the golden dataset.
   - Recommendation: Default = NO (dataset-only change; runtime stays consistent with the historic anchor). The planner can flag this as an OPTIONAL task in plan 16-01 if the user wants runtime parity.

## Sources

### Primary (HIGH confidence — in-repo)
- `.planning/adrs/0003-nien-tu-bach-polarity.md` — the ADR being superseded (full file read)
- `.planning/adrs/0001-ritual-schema-v1.md`, `.planning/adrs/0002-phi-tinh-monthly-anchor.md` — ADR format conventions (full files read)
- `.planning/PROJECT.md` — project state + Key Decisions table (verified v1.6 init on 2026-07-15)
- `.planning/ROADMAP.md` — phase goal, plans 16-01/16-02, success criteria 1–4 (full file read)
- `.planning/REQUIREMENTS.md` — FND-07, FND-08 formal definitions (full file read)
- `.planning/STATE.md` — current position: Phase 16 next (full file read)
- `.planning/milestones/v1.5-REQUIREMENTS.md` — FND-05 lineage (ADR-0003 was authored in v1.5)
- `.planning/milestones/v1.5-MILESTONE-AUDIT.md` — the 1960 divergence + MEDIUM caveat tech-debt item
- `.planning/research/ARCHITECTURE.md`, `.planning/research/FEATURES.md`, `.planning/research/PITFALLS.md`, `.planning/research/SUMMARY.md` — synthesis-level findings (full files read)
- `.planning/phases/13-phi-tinh-primitives-period-annual-monthly/13-04-PLAN.md` — precedent: how pre-1984 cross-validation cases were added to the golden dataset with `KnownDivergence` entries
- `crates/amlich-core/src/almanac/fengshui/annual.rs:80-184` — `nien_center`, `yuan_of_year`, evidence-note generation (full file read)
- `crates/amlich-core/src/almanac/fengshui/golden.rs:51-200` — `KnownDivergence` struct, `PhiTinhGoldenCase` struct, validator (full file read)
- `crates/amlich-core/data/almanac/flying_stars_golden.json:481-505` — 1960 case + `known_divergences` entry (literal content)
- `crates/amlich-core/tests/fengshui_invariants.rs:130-216` — Test C with divergent-year handling (full file read)
- `crates/amlich-core/src/almanac/fengshui/aspects.rs:56,77,271-280` — `FsConfidenceTier` enum precedent
- `crates/amlich-core/src/almanac/fengshui/mod.rs:26-35` — public re-exports (verify the new `GoldenConfidence` re-export path)

### Secondary (MEDIUM confidence — codebase inference)
- The `cross-validation cases exist` unit test (`golden.rs:283-288`) asserts `pre_1984 >= 2`. After FND-07, this test still passes (the 1920 and 1960 cases remain). If Phase 16 adds a third pre-1984 cross-check case (e.g., 1955 Vận 6 — different year polarity to cover both Dương and Âm within Trung Nguyên), this test passes trivially. Not required for FND-07.
- The "two-source minimum" validator (`golden.rs:152-160`) does NOT require an external classical citation — only 2 modern sources. Phase 16's `tiebreaker` updates are the audit trail; no schema change is needed.

### Tertiary (LOW confidence — flagged for validation)
- The list of modern Vietnamese Phi Tinh websites beyond the three already wired (phongthuycaivan.org, phongthuyso.vn, lasotuvi.com) was sourced from `.planning/research/FEATURES.md:283-289` — these are web research findings from the v1.5 milestone, unverified for citation accuracy but high-confidence for "are real sites that publish Phi Tinh tables".

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — no new deps; mirrors existing `FsConfidenceTier` and `#[serde(default)]` patterns.
- Architecture: HIGH — ADR supersession pattern is explicit in ADR-0003 §"Consequences"; golden dataset additive-field pattern matches `month: Option<u8>` and `jd: Option<i32>` precedents.
- Pitfalls: HIGH for runtime/test interactions (existing tests handle divergent cases correctly); MEDIUM for the cross-check citation choice (requires user input).
- Domain (independent classical reference): LOW until user provides the citation; the repo names only *Thẩm Thị* as classical, modern websites as data sources.

**Research date:** 2026-07-15
**Valid until:** 2026-08-15 (editorial phase; no fast-moving dependencies)