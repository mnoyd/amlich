# ADR-0006: Mai Hoa Dịch Số Casting Convention

**Status:** Accepted
**Date:** 2026-07-16
**Deciders:** Phase 20 Foundation (v1.7 Kinh Dịch)

---

## Context

Phase 22 will implement `cast_mai_hoa(lunar_year_branch, lunar_month, lunar_day, chi_hour_index) -> MaiHoaCast` — the deterministic Mai Hoa Dịch Số (梅花易數, "Plum Blossom Numerology") casting algorithm attributed to **Thiệu Khang Tiết** (邵康節, Shao Yong, 1011–1077 CE). The algorithm takes four integer inputs and produces a Tiên Thiên (先天, "Earlier Heaven") upper trigram, lower trigram, and moving-line index that together identify one of the 64 King Wen hexagrams plus a động hào (moving line) for biến quẻ (transformed hexagram) derivation.

The Mai Hoa casting surface contains TWO failure modes that this ADR pre-empts:

- **CRIT-2 (boundary convention):** the naïve `sum % k` form silently corrupts ~1/8 of castings at the `n % k == 0` boundary. For `n=8, k=8`: naïve `8 % 8 = 0`, and a reader might coerce `0` to position 1 (Kiền) — but the **correct** Tiên Thiên position at the boundary is **8 (Khôn)**, NOT 1 (Kiền). This is a silent data-corruption risk that contract-testing catches only if the convention is locked with a worked boundary example. v1.7 PITFALLS.md gates CRIT-2 at Phase 22 boundary golden case; this ADR front-loads the proof so Phase 22 implements against an unambiguous spec.
- **CRIT-3 (trigram-arrangement conflation):** Tiên Thiên trigram numbers (Kiền=1..Khôn=8, used by Mai Hoa casting) and Hậu Thiên / Lo Shu numbers (Khảm=1..Ly=9, used by King Wen display — see ADR-0005 §5) share the "1..N" form but are DIFFERENT mappings. The two arrangements describe the same 8 trigrams but assign different numbers. Conflating them produces a wrong hexagram. Plan 20-02's three-distinct-newtypes discipline (NO `From` between `TienThienTrigram`, `HauThienTrigram`, `KingWenHexagram`) is the type-level gate; this ADR locks the Tiên Thiên pin (§1) that the type encoding follows.

The v1.5 / v1.6 milestones established the **dual-source pin** (AF-05: every algorithmic convention cites ≥2 independent sources from day one). Mai Hoa casting requires the same discipline: a single source is not enough to settle the boundary convention with audit-trail authority. This ADR names **two** sources — Thiệu Khang Tiết (classical authority for the Tiên Thiên arrangement) and nhantu.net (modern Vietnamese practitioner reference with worked examples) — satisfying AF-05 from the moment the ADR lands.

This ADR locks the **convention**, not the Rust signature. Exact parameter encoding (e.g., `chi_hour_index` as `u8` index vs typed `Chi` enum, lunar-year-branch as branch index vs sexagenary pair) is **deferred to Phase 22 schema research** per 20-CONTEXT.md "Claude's Discretion". ADR-0006 specifies WHAT the algorithm computes, not HOW the parameters are typed.

## Decision

### 1. Tiên Thiên trigram arrangement (classical pin)

The Tiên Thiên (先天, "Earlier Heaven") trigram arrangement is **pinned** to the canonical Phục Hy / Thiệu Khang Tiết numbering, verified verbatim against vi.wikipedia's Mai Hoa Dịch Số article §"Lập quẻ đơn: Quẻ trừ 8":

| Tiên Thiên # | Trigram | Vietnamese | Symbol |
|---------------|---------|------------|--------|
| 1 | Càn / Kiền | Kiền | ☰ |
| 2 | Đoài | Đoài | ☱ |
| 3 | Ly | Ly | ☲ |
| 4 | Chấn | Chấn | ☳ |
| 5 | Tốn | Tốn | ☴ |
| 6 | Khảm | Khảm | ☵ |
| 7 | Cấn | Cấn | ☶ |
| 8 | Khôn | Khôn | ☷ |

This is the **dominant convention** in both the classical text and the modern Vietnamese practitioner references. The encoding choice (`TienThienTrigram` enum vs `struct(pub u8)`) is Plan 20-02's discretion; the locked constraint is that this 1..=8 mapping is the source-of-truth that Plan 20-02's encoding follows.

### 2. Inputs are LUNAR (not solar)

`cast_mai_hoa` takes **lunar** calendar inputs:

- `lunar_year_branch` — the Earthly Branch (Địa Chi) of the lunar year, as the index 0..=11 (Tý=0, Sửu=1, …, Hợi=11). The project's existing `crates/amlich-core/src/almanac/lunar.rs` does correct Vietnamese lunar conversion; `lunar_year_branch` is the **branch index**, not the full sexagenary pair.
- `lunar_month` — the lunar month number 1..=12. Leap-month (nhuận tháng) indexing is a Phase 22 schema-research question; the convention here is "canonical month only" (mirrors ADR-0001 `LeapPolicy::canonical_month_only`).
- `lunar_day` — the lunar day-of-month 1..=30.
- `chi_hour_index` — the Earthly Branch of the hour, as index 0..=11 (Tý=0, …, Hợi=11). The project's existing `Chi` enum is the canonical mapping; whether `chi_hour_index` is `u8` or `Chi` is Plan 22's discretion.

The classical Mai Hoa convention uses lunar inputs (the Thiệu Khang Tiết text operates in the traditional Chinese calendar, which is lunar-solar). Using solar (Gregorian) inputs would require a solar→lunar pre-conversion and would diverge from the classical source; the project's `lunar.rs` already exposes the correct lunar conversion so callers pass lunar values directly.

### 3. The `((n - 1) % k) + 1` remainder-zero convention (CRIT-2 prevention)

The Mai Hoa casting reduces each of (upper trigram, lower trigram, moving line) to a `sum % k` operation, where `k = 8` for trigrams and `k = 6` for the moving line. The **canonical convention** is:

```
result = ((sum - 1) % k) + 1
```

NOT the naïve `sum % k`. The `((n-1) % k) + 1` form maps the `sum % k == 0` boundary to position `k` (the **last** position), not position 0 (which would then be coerced to 1 = Kiền). A reader encountering the convention for the first time MUST verify this with the worked boundary example in §4 below — the convention is unambiguous by inspection once the example is read.

This convention is **locked** at the ADR level (not just the algorithm level) because:

- It is the **CRIT-2 prevention proof**. The naïve form silently corrupts ~1/8 of castings; the locked form is correct by construction.
- Phase 22's contract test cites this ADR's §4 worked example as the ground-truth boundary assertion.
- A future maintainer refactoring the casting code who replaces `((n-1) % k) + 1` with `sum % k` (or `(sum % k) + 1`, or any other variant) MUST be blocked by the contract test, which in turn cites this ADR.

### 4. Worked boundary example (CRIT-2 prevention proof — self-contained)

For inputs `lunar_year_branch=8, lunar_month=8, lunar_day=8, chi_hour_index=8` (the all-eights boundary, chosen because every sum is divisible by 8 and the naïve convention would coerce 0 → 1 = Kiền):

**Upper trigram** (sum of year-branch + month + day):

```
sum_upper = 8 + 8 + 8 = 24
((sum_upper - 1) % 8) + 1 = ((24 - 1) % 8) + 1 = (23 % 8) + 1 = 7 + 1 = 8
→ Tiên Thiên #8 = Khôn ☷
```

**Lower trigram** (sum of year-branch + month + day + hour):

```
sum_lower = 8 + 8 + 8 + 8 = 32
((sum_lower - 1) % 8) + 1 = ((32 - 1) % 8) + 1 = (31 % 8) + 1 = 7 + 1 = 8
→ Tiên Thiên #8 = Khôn ☷
```

**Moving line** (sum of year-branch + month + day + hour, modulo 6):

```
sum_line = 8 + 8 + 8 + 8 = 32
((sum_line - 1) % 6) + 1 = ((32 - 1) % 6) + 1 = (31 % 6) + 1 = 1 + 1 = 2
→ Moving line = 2 (nhị hào động)
```

**Result:** upper Khôn ☷ + lower Khôn ☷ = King Wen hexagram #2 (Thuần Khôn / Pure Earth), with the second hào động. Composed via Plan 20-02's `COMPOSITION_TABLE[(TienThienTrigram::Khon, TienThienTrigram::Khon)]` lookup.

**The CRIT-2 trap (explicit):** the naïve `sum % 8` for `sum_upper = 24` yields `24 % 8 = 0`. A reader who coerces `0 → 1` (because Tiên Thiên positions are 1..=8 and 0 is "out of range") gets **Tiên Thiên #1 = Kiền ☰**, NOT #8 = Khôn ☷. The composed hexagram would then be upper Kiền + lower Kiền = King Wen #1 (Thuần Kiền / Pure Heaven) — **a completely different hexagram**. The `((n-1) % k) + 1` form resolves this boundary WITHOUT an `if` statement: it produces `8 = Khôn` directly, matching the classical convention. Phase 22's contract test asserts this exact derivation (input tuple → hexagram #2, NOT hexagram #1).

### 5. Citation discipline (two-source pin + page-deferral marker)

The Mai Hoa casting convention is **pinned to two independent sources** from day one (AF-05 dual-source discipline):

**Classical authority — Thiệu Khang Tiết (邵康節):**

> Thiệu Khang Tiết (Shao Yong, 1011–1077 CE), *Mai Hoa Dịch Số* (梅花易數, "Plum Blossom Numerology"). Vietnamese edition: *Mai Hoa Dịch số*, dịch giả Văn Tùng (translator), NXB Văn Hoá Thông tin, Hà Nội, 2002.

Thiều Khang Tiết is the classical authority for the Tiên Thiên arrangement used in Mai Hoa casting. The text operates in lunar inputs (§2) and uses the `((n-1) % k) + 1` convention (§3) — the article "Lập quẻ đơn: Quẻ trừ 8" in vi.wikipedia's Mai Hoa Dịch Số entry summarises the convention with worked examples matching §4. The Tiên Thiên numbering in §1 is verbatim from this tradition.

**Modern Vietnamese practitioner reference — nhantu.net:**

> nhantu.net, *"Mai Hoa Dịch Số — Phần II: Cách lập quẻ Mai Hoa"*, https://www.nhantu.net/... (accessed 2026-07-16). [Exact URL: to be pinned in Phase 22 contract test.]

nhantu.net is the modern Vietnamese practitioner reference that Phase 22 names as a golden cross-source. It demonstrates the `((n-1) % k) + 1` convention with worked examples that match §4's all-eights derivation.

**Page-deferral note (mirrors ADR-0004 §5):** the Thiệu Khang Tiết Vietnamese edition (Văn Tùng translator, NXB Văn Hoá Thông tin, 2002) is cited by title + publisher + year + translator; the **exact page number** for the Tiên Thiên arrangement is **not located** in the open Vietnamese-language references reviewed for Phase 20 research. The algorithm is unaffected by the page-number gap — the convention is consistent across both sources — but the audit trail acknowledges that page-level authority awaits a numbered-edition physical or digital lookup. This deferral is recorded as an explicit `PendingExternalReview` marker:

```
ExternalReviewPending(
    reason="Exact page number for Tiên Thiên arrangement in *Mai Hoa Dịch số* (Văn Tùng translator, NXB Văn Hoá Thông tin, 2002) not located in open references";
    expected_review_date="2026-12-31";
    assigned_to="external-mai-hoa-reviewer"
)
```

This mirrors ADR-0004's "chapter + verse with explicit page-deferral" discipline (the page-deferral note is documented in v1.6-MILESTONE-AUDIT.md). An upgrade to page-level citation lands in ADR-0006a, not as an amendment to this document.

### 6. Alternative conventions considered — explicitly REJECTED

#### 6.1 — Solar (Gregorian) inputs — **REJECTED**

Using solar (Gregorian) calendar inputs for year/month/day/hour would require a solar→lunar pre-conversion at every call-site OR a silent convention shift away from the classical text. The classical Thiệu Khang Tiết text operates in the traditional Chinese lunar-solar calendar; the project's `crates/amlich-core/src/almanac/lunar.rs` already exposes the correct Vietnamese lunar conversion, so callers pass lunar values directly. A solar-input convention would diverge from the classical source AND introduce a redundant conversion layer. **REJECTED.**

#### 6.2 — Naïve `sum % k` form — **REJECTED**

The naïve `sum % k` form (without the `((n-1) % k) + 1` shift) silently corrupts every boundary case: `sum % k == 0` yields 0, which a reader coerces to 1 (Kiền) — producing a wrong hexagram ~1/8 of the time. This is CRIT-2 exactly. The worked example in §4 demonstrates the failure mode (would produce hexagram #1 Kiền instead of #2 Khôn at the all-eights boundary). **REJECTED.**

#### 6.3 — Hậu Thiên (Lo Shu) trigram arrangement for casting — **REJECTED**

Using the Hậu Thiên (Lo Shu) trigram numbers (Khảm=1, Khôn=2, …, Ly=9, skipping 5/center — the same numbers ADR-0005 §5 pins for `HauThienTrigram` corpus display) for the Mai Hoa casting sums would produce a different upper/lower trigram pair than the classical convention. The classical Thiệu Khang Tiết text uses the Tiên Thiên arrangement (§1) — encoding the casting in Hậu Thiên would invert the convention. The two arrangements describe the same 8 trigrams with different number assignments; the CRIT-3 prevention discipline (Plan 20-02's three distinct newtypes) is precisely what keeps the casting's Tiên Thiên and the corpus's Hậu Thiên from being conflated. **REJECTED.**

## Consequences

- **Phase 22** implements `cast_mai_hoa(lunar_year_branch, lunar_month, lunar_day, chi_hour_index) -> MaiHoaCast` per the convention in §1–§4. The exact Rust parameter encoding (chi as `u8` vs typed `Chi` enum; lunar-year-branch as branch index vs sexagenary pair) is Plan 22's schema-research discretion — this ADR specifies WHAT, not HOW the parameters are typed.
- **Phase 22 contract test** asserts the §4 all-eights derivation verbatim: input `(8, 8, 8, 8)` → upper Khôn ☷ + lower Khôn ☷ + moving line 2 → King Wen hexagram #2 (Thuần Khôn). Any refactor that breaks this assertion regresses CRIT-2.
- **Plan 20-02's** `TienThienTrigram` newtype follows the §1 numbering (Kiền=1..Khôn=8); `HauThienTrigram` follows ADR-0005 §5's Lo Shu numbering. NO `From` impl between them (CRIT-3).
- **biến quẻ derivation** (Phase 22) — flip the động hào bit on the cast hexagram's Tiên Thiên pair, re-compose via Plan 20-02's `COMPOSITION_TABLE`, derive the transformed King Wen hexagram. The 384-case (64 chủ quẻ × 6 động hào) contract test (CRIT-4) depends on this ADR's Tiên Thiên pin + Plan 20-02's bijective composition table.
- **English translation of `MaiHoaCast` debug output** is deferred; the cast result surfaces Tiên Thiên trigram identities by Vietnamese name (Kiền, Khôn, Chấn, …), matching the corpus's romanized-VN technical-term convention (ADR-0005 §3).
- **Page-citation upgrade** for the Thiệu Khang Tiết edition awaits a numbered-edition lookup; the upgrade lands in ADR-0006a, not as an amendment here. The `PendingExternalReview` marker in §5 is the audit-trail record.

## References

- **Classical (title + publisher + year + translator; page-deferral per §5):**
  - Thiệu Khang Tiết (邵康節, Shao Yong, 1011–1077 CE), *Mai Hoa Dịch Số* (梅花易數). Vietnamese edition: *Mai Hoa Dịch số*, dịch giả Văn Tùng, NXB Văn Hoá Thông tin, Hà Nội, 2002.
- **Modern Vietnamese practitioner reference (independent):**
  - nhantu.net, *"Mai Hoa Dịch Số — Phần II: Cách lập quẻ Mai Hoa"*, https://www.nhantu.net/... (accessed 2026-07-16; exact URL to be pinned in Phase 22 contract test).
- **Open encyclopaedic reference (verification):**
  - vi.wikipedia.org/wiki/Mai_Hoa_Dịch_số (accessed 2026-07-16) — confirms verbatim the Tiên Thiên numbering (§1), the "trừ 8 / trừ 6" casting convention (matches `((n-1)%k)+1`), the lunar-input convention (§2), and names the exact Vietnamese edition the project cites.
- **In-repo cross-references:**
  - `.planning/adrs/0004-daily-phi-tinh-starting-star-convention.md` §5 — page-deferral discipline precedent (mirrored in §5 of this ADR).
  - `.planning/adrs/0005-hexagram-entry-schema-v1.md` §5 — `HauThienTrigram` Lo Shu encoding pin (the DIFFERENT arrangement from §1 here; CRIT-3 prevention discipline).
  - `.planning/phases/20-foundation-schema-lock-source-ids-adrs-ontology/20-CONTEXT.md` §"ADR-0006" — locks the two-source pin, lunar-only inputs, `((n-1)%k)+1` convention, worked boundary example, and page-deferral discipline.
  - `.planning/phases/20-foundation-schema-lock-source-ids-adrs-ontology/20-RESEARCH.md` §"Sources > Secondary" — vi.wikipedia verification of the Tiên Thiên numbering.
  - `crates/amlich-core/src/almanac/lunar.rs` — existing Vietnamese lunar-conversion module; Phase 22 uses its output as `lunar_*` inputs to `cast_mai_hoa`.

---

*Adopted: 2026-07-16 (Phase 20-01)*
*No supersessions. Sibling to ADR-0005 (HexagramEntry schema). CRIT-2 boundary-convention lock + CRIT-3 trigram-arrangement pin.*
