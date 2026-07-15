# ADR-0003a: Niên Tử Bạch Polarity — Pre-1984 Confidence Closure

Status: Accepted
Date: 2026-07-15
Deciders: Phase 16 Foundation
Supersedes: ADR-0003 §6 (Confidence Acknowledgment)

---

## Context

ADR-0003 (`nien-tu-bach-polarity`, 2026-05-26) locked the Thượng / Trung / Hạ Nguyên polarity matrix, the year-polarity rule (dương = nghịch, âm = thuận), and the Tam Nguyên base/starting-star assignments. §6 of that ADR explicitly left the Thượng Nguyên (Vận 1–3, 1864–1923) and Trung Nguyên (Vận 4–6, 1924–1983) rows at **MEDIUM confidence**, sourced from a single Vietnamese-language web reference (phongthuycaivan.org), with the explicit expectation that "Future revisions to Thượng/Trung Nguyên rows after Phase 13 cross-check will be captured in ADR-0003a (not an amendment to this document)."

Phase 13 (Phi Tinh primitives + period + annual/monthly, 2026-05-28) added the two pre-1984 cross-validation cases (`annual-thuong-nguyen-1920`, `annual-trung-nguyen-1960`) and the 1960 Trung Nguyên `KnownDivergence` (center 5 vs 6) into `data/almanac/flying_stars_golden.json`. v1.5 milestone audit (`v1.5-MILESTONE-AUDIT.md`, 2026-05-28) recorded the pre-1984 MEDIUM confidence as Phase 10/13 tech debt to close in v1.6.

FND-07 (Phase 16) requires that pre-1984 Thượng/Trung Nguyên polarity rows be promoted from MEDIUM to HIGH confidence **after an external cross-check**, with the cross-check citation trail visible to a future reader.

---

## Decision

### 1. Pre-1984 Thượng/Trung Nguyên polarity rows are HIGH confidence

The Thượng Nguyên (Vận 1–3, 1864–1923) and Trung Nguyên (Vận 4–6, 1924–1983) rows of the polarity matrix are reclassified from MEDIUM to **HIGH** confidence.

The audit trail (cross-check sources, agreement, tiebreaker) is recorded in the `tiebreaker` and `note` fields of the two pre-1984 cases in `crates/amlich-core/data/almanac/flying_stars_golden.json`. A typed `confidence: "high"` annotation is added on each case per Plan 16-01.

### 2. Cross-check sources are independent secondary modern sources, not additional classical authorities

The promotion rests on **dual-source independent secondary modern verification** against the dương = nghịch / âm = thuận polarity rule and the Tam Nguyên base/starting-star assignments of ADR-0003:

- `phongthuycaivan.org` — the original Vietnamese-language web reference already cited in every golden case.
- `lasotuvi.com` — an independent secondary modern Vietnamese-language web reference that independently publishes the same polarity matrix for Vận 3 (Thượng Nguyên) and Vận 6 (Trung Nguyên) cross-validation years.
- `phongthuyso.vn` — an independent secondary modern Vietnamese-language web reference additionally consulted for Vận 6 (Trung Nguyên) years where lasotuvi.com disagrees.

These three sites are independent secondary modern references. They are **not** classical texts. **No additional classical title, chapter, or page is cited by this ADR.** A future reader who seeks classical confirmation beyond *Thẩm Thị Huyền Không Học* will not find one in this ADR's provenance trail — that gap is acknowledged and logged in §4 below.

### 3. *Thẩm Thị Huyền Không Học* remains the classical tiebreaker

*Thẩm Thị Huyền Không Học* (`Shen Shi Xuan Kong Xue`) remains the canonical classical tiebreaker per ADR-0003 §6 and `EXPANSION_FRAMEWORK §7`. For the 1960 Trung Nguyên case (phongthuycaivan.org = 5, lasotuvi.com = 6), the *Thẩm Thị* polarity matrix selects center = 5 (Ngũ Hoàng); lasotuvi.com's value 6 is rejected as Vận-number confusion. This tiebreaker call is unchanged from ADR-0003.

### 4. The 1960 Trung Nguyên case-level center-value split is **not** thereby resolved — disposition is `PendingExternalReview`

The HIGH confidence upgrade applies to the **polarity-row** (dương = nghịch / âm = thuận across the matrix) and to the Tam Nguyên base/starting-star assignments, **not** to the case-level center-value split that appears in the 1960 Trung Nguyên cross-validation case (`annual-trung-nguyen-1960`: phongthuycaivan.org reports center = 5, lasotuvi.com reports center = 6).

The 1960 center-value split is recorded as `PendingExternalReview`:

- **Operational `our_value` remains 5** per the *Thẩm Thị Huyền Không Học* polarity-matrix tiebreaker (Plan 16-02 will add the structured `DeferralMarker` field to the `KnownDivergence` schema; this ADR only locks the narrative disposition).
- **The case-level `expected_center` stays 5** in `data/almanac/flying_stars_golden.json`. Do NOT silently correct to 6.
- **The 1960 `KnownDivergence` entry remains** in the golden dataset's `known_divergences` array — the divergence is logged per FS-10, not silently corrected.
- **A reader must not interpret the HIGH polarity-row confidence upgrade as resolution of the 1960 center-value split.** The two are separate findings.

### 5. Backward Compatibility

- **ADR-0003 §§1–5 remain authoritative** (matrix structure, Tam Nguyên ranges, year polarity rule, worked examples, anchoring at Lập Xuân).
- **Only §6 (Confidence Acknowledgment) of ADR-0003 is superseded by this document.**
- **The polarity algorithm (`compute_yearly_flying_stars`) is unchanged.** The matrix inputs are unchanged; only the confidence annotation changes.
- **The Hạ Nguyên row (Vận 7–9) remains HIGH** as it was in ADR-0003 §6; this ADR does not alter that classification.
- **The v1.5 milestone audit tech-debt item for Phase 10/13 ("ADR-0003 confidence") is closed by this ADR + the typed golden-dataset annotations of Plan 16-01.**

---

## Consequences

- **FND-07 satisfied.** A reader of this ADR can trace the MEDIUM → HIGH promotion back to dual-source independent secondary modern verification (phongthuycaivan.org + lasotuvi.com / phongthuyso.vn), with the *Thẩm Thị* classical tiebreaker retained for the divergent case.
- **Plan 16-01 (FND-07) lands** a typed `GoldenConfidence { High, Medium, Low }` enum on `PhiTinhGoldenCase`, explicit `"confidence": "high"` annotations on the two pre-1984 cross-validation cases, an updated `metadata.description`, and an updated runtime evidence-note that emits `confidence=high` for pre-1984 years (mirroring the dataset's HIGH annotation). A new external-crate test `test_f_golden_pre_1984_confidence_is_high` in `tests/fengshui_invariants.rs` gates the typed annotation.
- **Plan 16-02 (FND-08) lands** the structured `DeferralMarker` field on `KnownDivergence`, applied to the 1960 case to make the `PendingExternalReview` disposition machine-readable. The narrative disposition is fixed by this ADR; the schema work is Plan 16-02.
- **No future classical reference is promised by this ADR.** The cross-check trail is modern secondary sources only. Any future upgrade to a classical reference beyond *Thẩm Thị* requires a further superseding ADR (ADR-0003b or later) and a new cross-check citation.
- **The provenance language in ADR-0003a and downstream artifacts must say "independent secondary modern verification" (or equivalent), not "additional classical authority".** The upgrade rests on independent secondary modern sources, not on any additional classical authority. (See Pitfall 1 of `16-RESEARCH.md`.)

---

*Supersedes ADR-0003 §6 only. ADR-0003 §§1–5 remain in force.*
