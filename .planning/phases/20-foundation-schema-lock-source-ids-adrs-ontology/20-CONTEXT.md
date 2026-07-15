# Phase 20: Foundation — Schema Lock + Source IDs + ADRs + Ontology - Context

**Gathered:** 2026-07-16
**Status:** Ready for planning

<domain>
## Phase Boundary

Lock the v1.7 IChing + Thái Tuế / Tam Sát ⇄ Phi Tinh cross-link foundation **before** any of the 64 corpus entries are authored (CRIT-1 × 7 prevention per the v1.5 lesson: 64 hexagrams × ~7 text fields = 448 corpus fields). Phase 20 delivers:

1. Two new `pub const SOURCE_*` registrations in `sources.rs` + `tests/source_id_guard.rs` extension.
2. Three accepted ADRs: ADR-0005 (`HexagramEntry` schema v1), ADR-0006 (Mai Hoa casting convention), ADR-0007 (cross-link CRIT-3 carve-out).
3. Three trigram/hexagram newtypes — `TienThienTrigram(u8)` / `HauThienTrigram(u8)` / `KingWenHexagram(u8)` — with **NO `From` impl between them** (CRIT-3 prevention) + the 64-entry Tiên Thiên-pair → King Wen composition table.
4. A passing 1-entry `HexagramEntry` serde round-trip probe (CRIT-1 schema-lock-first).
5. 6-slice ontology extension: `NodeConcept::Hexagram`, `EdgeConcept::LocatedAt`, `EdgeConcept::Transforms`, `ReasoningEvidenceSourceFamily::IChing`, `ActionId::IChing`.

**Phase 20 produces NO corpus content, NO casting algorithm, NO evaluator, NO DaySnapshot changes.** Phase 21 authors the 64-hexagram corpus against this locked schema; Phase 22 implements `cast_mai_hoa`; Phase 23 builds the cross-link; Phase 24 wires the evaluator + extends `DaySnapshot`. Phase 20 is the gate they all wait on.

This is the **third** "Foundation — Schema Lock" phase (Phase 10 = v1.5 Rituals/Phi Tinh, Phase 16 = v1.6 ADR-0003 closure). Most infrastructure decisions are locked by precedent; only ADR-0006's citation discipline and ADR-0005's field-set/reservation policy needed fresh user input.

</domain>

<decisions>
## Implementation Decisions

### ADR-0006 — Mai Hoa casting convention

- **Two-source pin from day 1.** ADR-0006 cites BOTH:
  - **Thiều Khang Tiết** (Shao Yong, 1011–1077) as the classical authority for the Tiên Thiên arrangement — the *Mai Hoa Dịch Số* text attributed to him. Chapter/section citation.
  - **nhantu.net** as the modern Vietnamese practitioner reference (which Phase 22 already names as a golden cross-source). Concrete URL + accessed-date citation; demonstrates the `((n-1)%k)+1` remainder-zero convention with worked examples.
  - Mirrors v1.5/v1.6 dual-source discipline (AF-05) and satisfies Phase 22's `≥2 independent sources` cross-check requirement from the moment the ADR lands.
- **Lock lunar-only + boundary convention.** ADR-0006 locks:
  - Inputs are **LUNAR** (not solar) — `lunar_year_branch`, `lunar_month`, `lunar_day`, `chi_hour_index`.
  - The `((n-1)%k)+1` remainder-zero convention is canonical (CRIT-2 prevention — `n=8, k=8` resolves to 8, not 1).
  - **Defer exact parameter encoding** (e.g., chi as `u8` index vs typed enum) to Phase 22 schema research. ADR-0006 specifies the *convention*, not the Rust signature.
- **Worked boundary example in ADR body.** ADR-0006 includes the `month=8 / day=8 / hour=8 → Tiên Thiên 8 (Khôn), not 1 (Kiền)` derivation as a step-by-step worked example in the ADR body itself. Phase 22's contract test cites it. Self-contained proof the convention is unambiguous — a reader does not need to consult the external source to verify the boundary.
- **Best-effort page citation + deferral marker.** Cite chapter/section by name; if a page number is available for the chosen edition (e.g., a specific printed Vietnamese edition of *Mai Hoa Dịch Số*), include it; otherwise add an explicit `PendingExternalReview` page-deferral note. **Mirrors Phase 16 ADR-0004's "chapter + verse with explicit page-deferral" discipline** (the page-deferral note is documented in v1.6-MILESTONE-AUDIT.md). Algorithm is unaffected by the page-number gap.

### HexagramEntry schema (ADR-0005)

- **Reserve English `*_en` optional fields.** Lock the roadmap-mandated Vietnamese fields AND reserve English counterparts as `Option<T>` with `#[serde(default, skip_serializing_if = "Option::is_none")]`:
  - **Locked VN fields:** `king_wen_index`, `vi_name`, `upper_trigram`, `lower_trigram`, `thoai_tu` (judgment), `hao_tu` (6 line texts; **7 for hexagrams 1 & 2**), `cat_hung`.
  - **Reserved EN fields:** `vi_name_en: Option<String>`, `thoai_tu_en: Option<String>`, `hao_tu_en: Option<Vec<String>>` (same 6/7 length rule). v1.7 ships VN-only but the schema accepts English later without re-locking.
  - Mirrors Phase 10's RIT-13 `body_en: Option<String>` reservation for rituals.
- **Field naming convention — `vi_name` / `*_tu`.** Language marker `vi_` AT THE FRONT for content fields (`vi_name`); romanized Vietnamese technical terms unmarked (`thoai_tu`, `hao_tu`, `cat_hung`). Matches the roadmap's existing spelling verbatim; differs from the rituals `body / body_en` (suffix) pattern — **ADR-0005 must explicitly document this divergence** so future maintainers don't "fix" it.
- **Free-text `reviewer: String` ON each entry.** Reviewer lives **ON** each `HexagramEntry` as a free-text `String`, using the `ExternalReviewPending(reason="..."; expected_review_date="..."; assigned_to="...")` marker pattern from `data/rituals/provenance_audit.md` (Phase 17 closure). This honors the roadmap's literal "each entry carries a reviewer signature" phrasing; the separate `data/iching/provenance_audit.md` ledger (mentioned in Phase 21 success criterion 3) is the *aggregate* audit view, not the canonical record.
  - Rationale: the rituals precedent (separate ledger) was driven by Phase 12 authoring ~60 entries in parallel; Phase 21 authors 64 entries with the same parallelism but the roadmap explicitly chose entry-embedded reviewer for IChing. The planner should NOT silently revert to the rituals ledger-only pattern.
- **Reuse `DeferralMarker` for `PendingExternalReview`.** `pending_review: Option<DeferralMarker>` where `DeferralMarker` is the existing struct from `crates/amlich-core/src/almanac/fengshui/golden.rs:85-95` (`{ reason, expected_review_date, assigned_to: Option<String> }`). Zero new types; the v1.6 RIT-14 pattern is reused verbatim. `HexagramEntry` becomes:
  ```rust
  pub struct HexagramEntry {
      pub king_wen_index: KingWenHexagram,           // newtype, see below
      pub vi_name: String,
      pub vi_name_en: Option<String>,                // reserved
      pub upper_trigram: HauThienTrigram,            // Hậu Thiên display per King Wen
      pub lower_trigram: HauThienTrigram,
      pub thoai_tu: String,
      pub thoai_tu_en: Option<String>,               // reserved
      pub hao_tu: Vec<String>,                       // 6 entries; 7 for hexagrams 1 & 2
      pub hao_tu_en: Option<Vec<String>>,            // reserved
      pub cat_hung: String,
      pub reviewer: String,                          // ExternalReviewPending(...) marker
      #[serde(default, skip_serializing_if = "Option::is_none")]
      pub pending_review: Option<DeferralMarker>,
      // ... deny_unknown_fields on the struct
  }
  ```
  Exact newtype identities (`TienThienTrigram` vs `HauThienTrigram` for upper/lower_trigram) is **Claude's discretion** — see below.

### Claude's Discretion

The following were NOT user-selected for discussion. Planner has flexibility within research-recommended defaults (mirrors Phase 10's discretion block discipline):

- **Newtype internal encoding** — whether `TienThienTrigram(u8)` carries the classical Tiên Thiên position (1..=8, Kiền=1) or the 3-bit line pattern (0..=7). Both are viable; planner picks based on what makes the composition table readable. The locked constraint is: three distinct newtypes with **NO `From` impl between them** (CRIT-3 prevention), each carrying `Debug + Clone + Copy + PartialEq + Eq + Serialize + Deserialize`.
- **Composition table representation** — `const COMPOSITION_TABLE: [(TienThienTrigram, TienThienTrigram); 64]` indexed by King Wen index, or a `fn compose(upper: TienThienTrigram, lower: TienThienTrigram) -> KingWenHexagram` with match arms, or embedded `data/iching/composition_table.json`. Planner decides; WASM-safety (no `std::fs`) and the "validates at load" success criterion are the constraints.
- **"Validates at load" assertion semantics** — bijectivity (every King Wen 1..=64 ↔ exactly one Tiên Thiên pair and vice versa)? Exhaustive coverage? Both? Planner defines the contract test.
- **1-entry serde round-trip probe corpus content** — synthetic fixture, hexagram #1 (Kiền), or a deliberately tricky case (hexagram #2 Khôn with 7 hao_tu, or a hexagram with `cat_hung` containing NFC-sensitive Vietnamese diacritics). The locked constraint is that it passes BEFORE any of the 64 real entries are authored.
- **`upper_trigram` / `lower_trigram` newtype identity** — Phase 22's casting produces Tiên Thiên trigrams, but the corpus's `upper/lower_trigram` fields semantically describe the Hậu Thiên (King Wen) arrangement. Whether both newtypes appear on `HexagramEntry` or only `HauThienTrigram` does is a research-informed choice.
- **ADR-0007 cross-link CRIT-3 carve-out body** — the roadmap locks the conclusion (read-only `reasoning/direction_composite.rs` placement + composite `rule.composite.direction_cross_link` envelope + `tests/thai_tue_cross_link_crit3.rs` grep guard). ADR-0007's narrative depth is at planner discretion; mirror ADR-0002/0003/0004's Nygard short-form length.
- **Ontology variant label strings** — `NodeConcept::Hexagram.label() == "hexagram"` (snake-case English) follows the existing convention (`flying_star`, `offering`, `ritual`). Confirm during planning; no need for Vietnamese label strings since the Rust variants are already locked English by the roadmap.
- **`ReasoningEvidenceSourceFamily::IChing` variant shape** — sibling variant alongside `AlmanacRule`, `FolkTradition`, etc. (NOT reusing `AlmanacRule` — IChing is a distinct Tier-0 family per the roadmap). Confirmed by the success criterion's literal name.
- **ADR cross-reference in `.planning/MILESTONES.md`** — append three new DEC-NNNN rows for ADR-0005/0006/0007 in the Key Decisions table (Phase 10 precedent).

</decisions>

<specifics>
## Specific Ideas

- ADR-0006's worked `month=8 / day=8 / hour=8 → Khôn` example should be **literate** — show the actual `((8-1)%8)+1 = 8` arithmetic inline so a reader can verify the boundary by inspection, not just trust the citation. This is the single most important sentence in ADR-0006 (it's the CRIT-2 prevention proof).
- The `reviewer: String` field on `HexagramEntry` is deliberately NOT a typed struct — the rituals `ExternalReviewPending(reason="..."; expected_review_date="..."; assigned_to="...")` free-text shape was chosen because it survives a future reviewer-name change without schema migration. ADR-0005 should document this choice (and link to the rituals precedent).
- The `hao_tu: Vec<String>` length rule (6 for hexagrams 3..=64; 7 for hexagrams 1 & 2 — Kiền and Khôn have a "dụng cửu" / "dụng lục" seventh line text) must be enforced by a loader invariant, not just documented. Planner should include this in the Phase 20 probe test or defer explicitly to Phase 21's loader — but ADR-0005 must mention the rule.
- ADR-0007's grep guard (`tests/thai_tue_cross_link_crit3.rs`) is a **sibling** to `tests/fengshui_crit3_isolation.rs` — same pattern, different module. Planner should look at the existing guard for the exact grep discipline.
- ADR storage stays in `.planning/adrs/` (NOT `docs/adr/` — that path does not exist; Phase 10 created `.planning/adrs/` deliberately next to the rest of the GSD planning artifacts). ADRs 0005/0006/0007 land alongside 0001-0004.

</specifics>

<code_context>
## Existing Code Insights

### Reusable Assets

- **`crates/amlich-core/src/sources.rs:1-41`** — canonical home for every `SOURCE_*` constant. Phase 20 adds `SOURCE_KINH_DICH: &str = "kinh-dich"` and `SOURCE_MAI_HOA_DICH_SO: &str = "mai-hoa-dich-so"` as plain `pub const` (matches the existing 7 constants exactly — no enum machinery per Phase 10's locked decision).
- **`crates/amlich-core/tests/source_id_guard.rs:13-21`** — `FORBIDDEN_LITERALS` array. Phase 20 appends `"\"kinh-dich\""` and `"\"mai-hoa-dich-so\""` so bare literals at provenance call-sites fail CI. The guard already skips `sources.rs` itself + `#[cfg(test)]` blocks + `//`-comments, so no test-fixture breakage.
- **`crates/amlich-core/src/almanac/fengshui/types.rs:15-43, 70-91`** — `Palace` (`#[repr(u8)]` enum with explicit discriminants 1..=9) and `FlyingStar` (same pattern, 1..=9) are the **direct precedent** for the three trigram/hexagram newtypes. The planner may follow either the `enum + #[repr(u8)]` style OR a true `struct TienThienTrigram(pub u8)` newtype — both are acceptable; the locked constraint is "three distinct types, no `From` between them".
- **`crates/amlich-core/src/almanac/fengshui/types.rs:129-143`** — `DailyFlyingStarLayout` is the v1.6 **sibling-newtype precedent**. Phase 22's `IChingQuery` / Phase 24's `IChingEvaluator` will follow this exact "sibling-newtype over closed-enum extension" discipline (avoids ~25–43 call-site `Copy`-break churn).
- **`crates/amlich-core/src/almanac/fengshui/golden.rs:85-95, 107-125`** — `DeferralMarker` struct + its embed as `KnownDivergence.deferral: Option<DeferralMarker>`. **Reused verbatim** on `HexagramEntry.pending_review: Option<DeferralMarker>`. Import path: `crate::almanac::fengshui::golden::DeferralMarker` (or re-export if the planner prefers a more neutral location).
- **`crates/amlich-core/src/semantic_graph/ontology.rs:3-43, 89-121, 159-228, 230-301, 336-411`** — the **6-slice ontology**: every concept must appear in (1) `NodeConcept` enum, (2) `NodeConcept::label()` match, (3) `ConceptLabel` enum, (4) `ConceptLabel::as_str()` match, (5) `GraphOntology::node_concepts()` slice, (6) `GraphOntology::edge_concepts()` slice (for edges). Phase 20 adds `Hexagram` to nodes + `LocatedAt` + `Transforms` to edges, touching all 6 locations. The v1.5 / v1.6 concept-addition precedent (`Ritual`, `FlyingStar`, `Offering`, `RecommendsOffering`) is the template.
- **`.planning/adrs/0001-ritual-schema-v1.md`, `0002-phi-tinh-monthly-anchor.md`, `0003-nien-tu-bach-polarity.md`, `0003a-...`, `0004-daily-phi-tinh-starting-star-convention.md`** — ADRs 0001-0004. **Nygard short-form** template: `Title / Status / Context / Decision / Consequences`, ~1 page each. ADRs 0005/0006/0007 inherit this format exactly. ADR-0004 is the closest analog to ADR-0006 (both lock a *convention* with a citation deferral note).
- **`crates/amlich-core/data/rituals/provenance_audit.md`** — the rituals reviewer-audit ledger. The `ExternalReviewPending(reason="..."; expected_review_date="..."; assigned_to="...")` free-text marker format is defined here and is **reused verbatim** as the `reviewer: String` value shape on `HexagramEntry`.

### Established Patterns

- **Schema-lock before corpus/algorithm** — type stubs + ADR + 1-entry serde round-trip probe precede corpus authoring (v1.5 CRIT-1 lesson × 7 amplification for v1.7's 448 corpus text fields). The probe must pass in Phase 20 before Phase 21 can start.
- **`#[serde(deny_unknown_fields)]` at the corpus entry level** — used by rituals + every golden dataset; `HexagramEntry` inherits this discipline. Unknown fields fail at deserialization (catches typos in `vi_name` / `thoai_tu` etc. during Phase 21 corpus authoring).
- **`Option<T>` + `#[serde(default, skip_serializing_if = "Option::is_none")]`** for additive DTO fields (v1.2 Ten Gods / Kua precedent; re-validated in v1.5 INT-05 + v1.6 INT-10). The reserved `*_en` fields on `HexagramEntry` follow this exactly.
- **Sibling-newtype over closed-enum extension** — v1.6 `DailyFlyingStarLayout` sibling precedent (avoids `Copy`-break churn). Phase 24's `IChingQuery` newtype follows this; Phase 20 only needs to ensure the three trigram/hexagram newtypes don't accidentally grow a `From` impl between them (CRIT-3 prevention).
- **NFC normalization at load** — every Vietnamese text field is NFC-normalized (RIT-08 precedent). `HexagramEntry`'s `vi_name` / `thoai_tu` / `hao_tu` / `cat_hung` inherit this at Phase 21 loader time; Phase 20's probe should use NFC-sensitive diacritics to prove the round-trip is normalization-safe.
- **Source-id discipline** — every new module declares `pub const SOURCE_*`; provenance call-sites never use string literals (CI-enforced by `source_id_guard.rs`). Phase 20 extends the guard with `SOURCE_KINH_DICH` + `SOURCE_MAI_HOA_DICH_SO`.

### Integration Points

- **`crates/amlich-core/src/sources.rs`** — append two new constants after `SOURCE_HUYEN_KHONG` (line 26). Update the `all_constants_have_expected_values` test (lines 47-56) to assert the two new values.
- **`crates/amlich-core/tests/source_id_guard.rs:13-21`** — append `"\"kinh-dich\""` and `"\"mai-hoa-dich-so\""` to `FORBIDDEN_LITERALS`.
- **`crates/amlich-core/src/lib.rs`** — add `pub mod iching;` (or `pub mod reasoning::iching;` depending on planner's module-path choice — the roadmap places the cross-link in `reasoning/direction_composite.rs`, suggesting IChing code also lives under `reasoning/`). Reserve the location in Phase 20; populate in Phases 21-24.
- **`crates/amlich-core/src/semantic_graph/ontology.rs`** — 6-slice extension for `Hexagram` (node) + `LocatedAt` + `Transforms` (edges). Plus `ReasoningEvidenceSourceFamily::IChing` (likely in `crates/amlich-core/src/reasoning/types.rs` — planner locates the exact enum) and `ActionId::IChing` (likely in `crates/amlich-core/src/reasoning/action.rs`).
- **`.planning/adrs/`** — create `0005-hexagram-entry-schema-v1.md`, `0006-mai-hoa-casting-convention.md`, `0007-cross-link-crit3-carve-out.md`.
- **`.planning/MILESTONES.md` Key Decisions table** — append three new DEC-NNNN rows linking to ADRs 0005/0006/0007 (Phase 10 precedent).
- **`crates/amlich-core/data/iching/`** — directory reserved for Phase 21 corpus (`hexagrams.json`, `provenance_audit.md`, possibly `composition_table.json`). Phase 20 may create the directory + the 1-entry probe fixture but does NOT author the 64-entry corpus.
- **NOT touched in Phase 20:** `DaySnapshot` struct (`lib.rs:154-185`) — `iching_cast` and `direction_cross_link` fields are Phase 24's job. `cast_mai_hoa` function — Phase 22's job. `build_direction_cross_link` — Phase 23's job.

</code_context>

<deferred>
## Deferred Ideas

- **English `*_en` field POPULATION** — ADR-0005 *reserves* `vi_name_en` / `thoai_tu_en` / `hao_tu_en` in the schema; actually authoring English content is deferred indefinitely (mirrors RIT-13's `body_en` reservation — no milestone scheduled).
- **Mai Hoa casting algorithm** (`cast_mai_hoa` signature + body) — Phase 22. ADR-0006 only locks the convention.
- **biến quée derivation + Thể/Dụng classification** — Phase 22.
- **64-hexagram corpus authoring** — Phase 21 (Ngô Tất Tố translation).
- **`IChingQuery` sibling newtype + `IChingEvaluator`** — Phase 24.
- **`build_direction_cross_link` composite + Thái Tuế directional module + classical 3-direction Tam Sát** — Phase 23.
- **`DaySnapshot.iching_cast` + `DaySnapshot.direction_cross_link` additive fields** — Phase 24.
- **E2E validation + ≥10 cross-source golden casting cases** — Phase 25.
- **Image / unicode hexagram symbol fields on `HexagramEntry`** — NOT reserved in v1.7 (planner may add at its discretion if it makes the schema more robust, but the user did not select this option; default is to omit).
- **Custom clippy lint for source_id literals** — Phase 10 already deferred this; the grep test in `source_id_guard.rs` remains the canonical discipline.

</deferred>

---

*Phase: 20-foundation-schema-lock-source-ids-adrs-ontology*
*Context gathered: 2026-07-16*
