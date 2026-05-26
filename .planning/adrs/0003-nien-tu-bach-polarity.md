# ADR-0003: Niên Tử Bạch Direction Polarity Matrix

Status: Accepted
Date: 2026-05-26
Deciders: Phase 10 Foundation

---

## Context

Annual flying stars (Niên Tử Bạch) have a starting-star and a flight direction (thuận hành = forward ascending, nghịch hành = retrograde descending). The direction varies by:

1. **Tam Nguyên yuan** — the 60-year super-cycle group the year belongs to.
2. **Year polarity** — whether the year's Heavenly Stem (thiên can) is dương (yang) or âm (yin).

PITFALLS MOD-3 warns that encoding this as a single `is_retrograde: bool` flag is incorrect — the rule is a matrix, not a single flag. A single flag cannot capture the Tam Nguyên dimension.

Single-source risk: the Thượng Nguyên and Trung Nguyên rows below come from a single Vietnamese-language source (phongthuycaivan.org). Cross-validation against *Thẩm Thị Huyền Không Học* is deferred to Phase 13 implementation.

---

## Decision

### 1. Direction is determined by `(Tam Nguyên yuan, year polarity) → (starting star, direction)`

This is a two-key lookup, not a boolean flag.

### 2. Tam Nguyên Structure (FS-05)

- **Thượng Nguyên:** Vận 1 (1864–1883), Vận 2 (1884–1903), Vận 3 (1904–1923) — base/starting star: **1 (Nhất Bạch)**
- **Trung Nguyên:** Vận 4 (1924–1943), Vận 5 (1944–1963), Vận 6 (1964–1983) — base/starting star: **4 (Tứ Lục)**
- **Hạ Nguyên:** Vận 7 (1984–2003), Vận 8 (2004–2023), Vận 9 (2024–2043) — base/starting star: **7 (Thất Xích)**
- **Yuan boundaries:** Lập Xuân (not January 1) — anchored by the v1.1.2 Tiết Khí scanner (`get_all_tiet_khi_for_year` at `crates/amlich-core/src/tietkhi.rs:227`), consistent with ADR-0002.

### 3. Year Polarity Rule (FS-05)

Year polarity is determined by the Heavenly Stem (thiên can) of the sexagenary year:

- **Dương (yang)** — stem index is **odd** in the 10-stem sequence: Giáp (1), Bính (3), Mậu (5), Canh (7), Nhâm (9)
- **Âm (yin)** — stem index is **even**: Ất (2), Đinh (4), Kỷ (6), Tân (8), Quý (10)

Stem polarity is already available via existing `canchi.rs` (no new code needed in Phase 10).

### 4. Niên Tử Bạch Direction Matrix

| Yuan | Vận Range | Starting Star | Dương Year Direction | Âm Year Direction |
|------|-----------|---------------|---------------------|-------------------|
| Thượng Nguyên | 1864–1923 (Vận 1–3) | 1 (Nhất Bạch) | Nghịch hành (retrograde) | Thuận hành (forward) |
| Trung Nguyên  | 1924–1983 (Vận 4–6) | 4 (Tứ Lục)    | Nghịch hành (retrograde) | Thuận hành (forward) |
| Hạ Nguyên     | 1984–2043 (Vận 7–9) | 7 (Thất Xích) | Nghịch hành (retrograde) | Thuận hành (forward) |

### 5. Worked Examples (year polarity verification)

- **2024 = Giáp Thìn** → Giáp is stem 1 (odd) → **dương**; Vận 9 → Hạ Nguyên → starting star 7, **nghịch hành** (retrograde)
- **2025 = Ất Tỵ** → Ất is stem 2 (even) → **âm**; Vận 9 → Hạ Nguyên → starting star 7, **thuận hành** (forward)

### 6. Confidence Acknowledgment

- **Hạ Nguyên row** — HIGH confidence. This is the practical contemporary era; the dương=nghịch / âm=thuận pattern for Hạ Nguyên is cross-confirmed by multiple sources.
- **Thượng Nguyên and Trung Nguyên rows** — **MEDIUM confidence**. These rows are sourced from phongthuycaivan.org (single Vietnamese-language source). Phase 13 is the designated cross-validation phase: during `compute_yearly_flying_stars` implementation, these rows must be cross-checked against *Thẩm Thị Huyền Không Học*. If divergence is found:
  - Log as `KnownDivergence` per EXPANSION_FRAMEWORK §7
  - Do NOT silently correct — issue ADR-0003a to supersede this document
  - Flag in the `evidence: ReasoningEvidenceEnvelope` of any `FlyingStarLayout` computed for pre-1984 years

---

## Consequences

- **Phase 13** implements a `(van_number, year_polarity) -> (starting_star, direction)` lookup function, never a `bool` flag.
- **Year polarity** computed from can-chi stem (already available via `canchi.rs` — no new code needed in Phase 10).
- **Year boundaries** anchor at Lập Xuân via `tietkhi.rs` (consistent with ADR-0002).
- **Pre-1984 year queries** must surface the MEDIUM-confidence annotation in the evidence envelope.
- **Future revisions** to Thượng/Trung Nguyên rows after Phase 13 cross-check will be captured in ADR-0003a (not an amendment to this document).
