# ADR-0007: Cross-Link CRIT-3 Carve-Out

**Status:** Accepted
**Date:** 2026-07-16
**Deciders:** Phase 20 Foundation (v1.7 Kinh Dịch)

---

## Context

Phase 23 implements the read-only directional cross-link between the **Thái Tuế / Tam Sát** subsystem (KHCBPPT-source directional taboos in `almanac/thai_tue.rs` + the new `almanac/tam_sat.rs`) and the **Phi Tinh** subsystem (`huyen-khong`-source palace layout in `almanac/fengshui/`). The cross-link surfaces both sources' directional reasoning inside a single composite envelope so a DaySnapshot consumer can answer "what does each tradition say about direction X on date Y?" without consulting two separate fields.

The cross-link surface intersects the project's most expensive pitfall: **CRIT-3 (Tiên Thiên trigram numbers vs King Wen hexagram numbers vs Hậu Thiên palace numbers — different mappings, shared "1..N" form)**. v1.5 / v1.6隔离 discipline established that `crates/amlich-core/src/interaction/direction_merge.rs` MUST NOT reference Phi Tinh types — enforced by `tests/fengshui_crit3_isolation.rs` (a CI grep guard scanning for `FlyingStar|DailyFlyingStar|DailyFlyingStarLayout|almanac::fengshui` references in `direction_merge.rs` and asserting zero matches). The guard preserves the type-level firewall between directional-merge logic (which consumes KHCBPPT + tam_sat output) and Flying-Star layout (which is a separate read surface).

The cross-link, by design, crosses that firewall — but it must do so **without collapsing CRIT-3 isolation**. If `interaction/direction_merge.rs` started importing `FlyingStarLayout` directly to surface the cross-link, the existing CRIT-3 guard would fail (correctly), and the type-level firewall that protects the rest of the codebase from trigram/palace conflation would be punctured. The cross-link needs a **carve-out**: a separate placement + a composite-envelope pattern that lets the cross-link surface both sources WITHOUT importing either source's types into `direction_merge.rs`.

This ADR locks the **placement contract** for the cross-link (the *where*) and the **envelope pattern** (the *how*). Phase 23 populates the implementation; Phase 24 wires it into `DaySnapshot.direction_cross_link`. Phase 20 documents the contract so Plans 20-02 / 20-03 (the newtype + ontology foundations) reserve the right module path and the right ontology edge variants (`EdgeConcept::LocatedAt`, `EdgeConcept::Transforms` per FND-12) for the cross-link to populate later.

## Decision

### 1. Placement — `reasoning/direction_composite.rs::build_direction_cross_link`

The cross-link lives in a NEW module `crates/amlich-core/src/reasoning/direction_composite.rs` (Phase 23 authors the file). The single public entry point is:

```rust
/// Build the read-only directional cross-link between Thái Tuế / Tam Sát
/// (KHCBPPT-source) and Phi Tinh (huyen-khong-source) for a given snapshot.
///
/// Read-only by design: takes only `&` references, does NOT mutate either
/// source subsystem. Phase 23 populates; Phase 24 wires into DaySnapshot.
pub fn build_direction_cross_link(/* &DaySnapshot or sub-refs */) -> DirectionCrossLink {
    // ... Phase 23 implementation ...
}
```

The placement is **`reasoning/`, NOT `interaction/direction_merge.rs`**. Rationale:

- `interaction/direction_merge.rs` is the existing KHCBPPT + tam_sat merge surface, guarded by `tests/fengshui_crit3_isolation.rs` against Phi Tinh imports. Adding the cross-link there would either break the guard (regressing CRIT-3) OR require weakening the guard (also regressing CRIT-3).
- `reasoning/` is the layer ABOVE `interaction/` and `almanac/` — it consumes read-only references from lower layers and emits composite artifacts (semantic-graph edges, evidence envelopes). The v1.5 / v1.6 reasoning layer already follows this read-only composite pattern (e.g., `build_reasoning_input_graph`, `build_day_snapshot_graph`); `build_direction_cross_link` is a sibling.
- A future maintainer looking for "where does the cross-link live?" finds it under `reasoning/` next to the other composite builders — not buried inside `interaction/direction_merge.rs` behind a CRIT-3 carve-out exception.

### 2. Composite envelope pattern — distinct primitive envelopes + ONE composite

The cross-link emits **three** provenance envelopes per DaySnapshot:

1. A primitive `source_id: khcbppt` envelope for the Thái Tuế / Tam Sát directional content.
2. A primitive `source_id: huyen-khong` envelope for the Phi Tinh palace layout content.
3. **ONE composite `rule.composite.direction_cross_link` envelope** that references both primitive envelopes and carries the cross-link's combined rationale.

The composite envelope's `source_id` field carries the literal string `"rule.composite.direction_cross_link"` (NOT a `SOURCE_*` constant — composite envelopes are rule identifiers, not corpus source identifiers, and are exempt from the `source_id_guard.rs` discipline because they do not match any `FORBIDDEN_LITERALS` entry). This is the **only envelope pattern compatible with the CRIT-3 grep guard** (§3 below) because it lets the cross-link surface both sources' content through a single composite reference rather than by importing either source's types into the merge layer.

This pattern mirrors the v1.5 multi-source provenance append pattern (DEC-0019 / INT-09 dual-source `cross_source_curing`): each primitive envelope is a separate `track_provenance` call on the relevant source's content, and the composite envelope is an additional `track_provenance` call that names the composite rule. The dual-source discipline (AF-05) is preserved because each primitive envelope independently cites its source.

### 3. CRIT-3 grep guard sibling — `tests/thai_tue_cross_link_crit3.rs`

A NEW sibling CI grep guard lands in Phase 23 at `crates/amlich-core/tests/thai_tue_cross_link_crit3.rs`. The guard mirrors `tests/fengshui_crit3_isolation.rs:14-44` (the v1.6 precedent) but scans a DIFFERENT module:

- `tests/fengshui_crit3_isolation.rs` scans `interaction/direction_merge.rs` for `FlyingStar|DailyFlyingStar|DailyFlyingStarLayout|almanac::fengshui` references and asserts zero matches.
- `tests/thai_tue_cross_link_crit3.rs` (NEW) scans `interaction/direction_merge.rs` for the SAME forbidden type-name set (preserving the existing guard's contract) AND additionally asserts that the cross-link's composite envelope in `reasoning/direction_composite.rs` does NOT import Phi Tinh types directly — only the read-only `&DaySnapshot` (or sub-references) and the composite envelope types defined locally to `direction_composite.rs`.

The two guards are **complementary**: the existing guard preserves the `direction_merge.rs` firewall (unchanged); the new guard preserves the `direction_composite.rs` carve-out (new). Phase 23 authors the new guard; the existing guard is NOT modified.

### 4. Read-only by design — `&` references only

`build_direction_cross_link` takes only `&` references (immutable borrows). It does NOT mutate the Thái Tuế / Tam Sát / Phi Tinh / `direction_merge` state. The cross-link is a **projection**, not a transformation: it reads the post-merge directional taboos (KHCBPPT) and the post-layout palace assignments (huyen-khong) and emits a composite envelope that references both.

This is the same read-only discipline as the v1.5 / v1.6 reasoning builders (`build_reasoning_input_graph`, `build_day_snapshot_graph`). It preserves the layering invariant: lower layers (almanac, interaction) compute authoritative state; the reasoning layer composes read-only views. A future maintainer adding mutation to `build_direction_cross_link` regresses this ADR.

### 5. Ontology reservation (FND-12 cross-reference)

Plan 20-03's ontology extension reserves the edge variants the cross-link populates:

- `EdgeConcept::LocatedAt` — surfaces the directional palace/layout position each tradition assigns (the "where" of each directional claim).
- `EdgeConcept::Transforms` — surfaces the composite cross-link relationship itself (the "this tradition's directional claim transforms into a composite view when joined with the other tradition's claim" relationship).

These variants are added to the 6-slice ontology (per FND-12) in Plan 20-03. Phase 23's `build_direction_cross_link` populates the actual edge instances; Phase 24 wires them into the `DaySnapshot.direction_cross_link` additive field. This ADR does not author the variants (Plan 20-03 does); it documents the **placement contract** so Plan 20-03's variant additions are not "abstract" — they have a concrete consumer in Phase 23.

## Consequences

- **Phase 23** authors `crates/amlich-core/src/reasoning/direction_composite.rs` with the single `pub fn build_direction_cross_link` entry point per §1. The function signature's exact parameter shape (`&DaySnapshot` vs sub-references) is Phase 23's discretion; the locked constraint is **read-only `&` references only** (§4).
- **Phase 23** authors `crates/amlich-core/tests/thai_tue_cross_link_crit3.rs` per §3 — sibling to `tests/fengshui_crit3_isolation.rs`, scanning `interaction/direction_merge.rs` (preserving the existing guard) AND adding a new scan over `reasoning/direction_composite.rs` for forbidden Phi Tinh type imports.
- **Phase 23** populates the Thái Tuế directional primitive (`source_id: khcbppt`) AND the classical 3-direction Tam Sát primitive (`source_id: khcbppt` per 20-CONTEXT.md open research question — Phase 23 resolves the FS-10 3-vs-1 direction decision, recommended option b: new `almanac/tam_sat.rs`). The cross-link surfaces both KHCBPPT primitives + the `huyen-khong` primitive via the §2 composite envelope pattern.
- **Phase 24** wires `build_direction_cross_link`'s output into the additive `DaySnapshot.direction_cross_link: Option<DirectionCrossLink>` field. The additive `Option<T>` + `#[serde(default, skip_serializing_if = "Option::is_none")]` discipline (v1.6 INT-10) applies — v1.6→v1.7 round-trip test strips the new field and asserts byte-equal recovery.
- **CRIT-5 (cross-link collapses CRIT-3 isolation)** is gated by this ADR's §1 (placement in `reasoning/`, NOT `interaction/direction_merge.rs`) + §3 (sibling grep guard). A future maintainer who moves the cross-link into `direction_merge.rs` (or who weakens the existing guard to allow Phi Tinh imports there) regresses CRIT-5 and re-opens CRIT-3.
- **`tests/fengshui_crit3_isolation.rs` is unchanged** by this ADR. The v1.6 guard continues to enforce the v1.6 contract; this ADR adds a sibling, not an amendment.
- **`interaction/direction_merge.rs` is unchanged** by this ADR. The merge layer continues to consume KHCBPPT + tam_sat output; the cross-link is a separate read-only composite at the reasoning layer.

## References

- **In-repo cross-references:**
  - `crates/amlich-core/tests/fengshui_crit3_isolation.rs:14-44` — existing CRIT-3 grep guard template (Phase 18 / v1.6 precedent); `tests/thai_tue_cross_link_crit3.rs` mirrors this pattern.
  - `crates/amlich-core/src/interaction/direction_merge.rs` — existing merge layer; this ADR does NOT modify it (the cross-link lives at `reasoning/`, not here).
  - `crates/amlich-core/src/reasoning/` — composite-builder layer (existing `build_reasoning_input_graph`, `build_day_snapshot_graph`); `build_direction_cross_link` is a sibling.
  - `crates/amlich-core/src/sources.rs` — `SOURCE_KHCBPPT` + `SOURCE_HUYEN_KHONG` constants (the two primitive source_id values surfaced by the cross-link's §2 envelope pattern).
  - `.planning/adrs/0005-hexagram-entry-schema-v1.md` §5 — `HauThienTrigram` Lo Shu encoding pin (CRIT-3 prevention discipline companion).
  - `.planning/adrs/0006-mai-hoa-casting-convention.md` §1 — Tiên Thiên trigram pin (CRIT-3 prevention discipline companion).
  - `.planning/phases/20-foundation-schema-lock-source-ids-adrs-ontology/20-CONTEXT.md` §"Claude's Discretion > ADR-0007 cross-link CRIT-3 carve-out body" — locks the conclusion (read-only `reasoning/direction_composite.rs` placement + composite `rule.composite.direction_cross_link` envelope + `tests/thai_tue_cross_link_crit3.rs` grep guard).
  - `.planning/phases/20-foundation-schema-lock-source-ids-adrs-ontology/20-RESEARCH.md` §"Sources > Primary" — `tests/fengshui_crit3_isolation.rs:14-44` grep-guard pattern verification.
- **v1.5 / v1.6 precedents:**
  - DEC-0019 / INT-09 dual-source `cross_source_curing` — composite-envelope provenance append pattern (referenced in §2).
  - STATE.md "Key Decisions Added in 19-02" — payload-via-`nodes_mut()` post-population + edge-dedup-via-HashSet discipline (Phase 23's cross-link edge emission follows the same pattern).

---

*Adopted: 2026-07-16 (Phase 20-01)*
*No supersessions. Sibling to ADR-0005 (HexagramEntry schema) + ADR-0006 (Mai Hoa casting). CRIT-5 cross-link placement lock + CRIT-3 carve-out contract.*
