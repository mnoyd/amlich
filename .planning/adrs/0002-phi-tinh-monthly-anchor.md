# ADR-0002: Phi Tinh Monthly Anchor Convention

Status: Accepted
Date: 2026-05-26
Deciders: Phase 10 Foundation

---

## Context

Monthly Phi Tinh (Nguyệt Tử Bạch) requires a month-boundary definition. Vietnamese feng shui practice splits across schools:

- **Lunar months** — boundaries shift Gregorian dates year-to-year; produce school-dependent results (PITFALLS MOD-2 warns this yields inconsistent outcomes between classical texts).
- **Civil calendar months** — January 1 based; ignores classical convention entirely.
- **Solar-term months (tháng tiết khí)** — defined by the sun's ecliptic longitude; aligns with *Thẩm Thị Huyền Không Học* (Shen Shi Xuan Kong Xue), the primary classical reference for Phi Tinh.

The codebase already ships a v1.1.2 Tiết Khí scanner that resolves solar-term instants precisely. Using a different boundary would require new scanning code with no classical justification.

---

## Decision

1. **Monthly Phi Tinh uses solar-term month boundaries** (tháng tiết khí) per *Thẩm Thị Huyền Không Học*. Lunar-month and civil-month conventions are rejected.

2. **Boundary resolver:** the v1.1.2 Tiết Khí scanner function  
   `get_all_tiet_khi_for_year(year: i32, time_zone: f64) -> Vec<SolarTermWithDate>`  
   located at `crates/amlich-core/src/tietkhi.rs:227`.  
   No new term-scanning code is added in v1.5.

3. **The 12 solar-month opening terms** (every other Tiết Khí term from the `TIET_KHI` const array, the odd-indexed ones that open each branch-month):

   | Lunar Branch Month | Tiết Khí (Opening Term) | Ecliptic Longitude |
   |--------------------|-------------------------|--------------------|
   | Dần (Month 1)      | Lập Xuân                | 315°               |
   | Mão (Month 2)      | Kinh Trập               | 345°               |
   | Thìn (Month 3)     | Thanh Minh              | 15°                |
   | Tỵ (Month 4)       | Lập Hạ                  | 45°                |
   | Ngọ (Month 5)      | Mang Chủng              | 75°                |
   | Mùi (Month 6)      | Tiểu Thử                | 105°               |
   | Thân (Month 7)     | Lập Thu                 | 135°               |
   | Dậu (Month 8)      | Bạch Lộ                 | 165°               |
   | Tuất (Month 9)     | Hàn Lộ                  | 195°               |
   | Hợi (Month 10)     | Lập Đông                | 225°               |
   | Tý (Month 11)      | Đại Tuyết               | 255°               |
   | Sửu (Month 12)     | Tiểu Hàn                | 285°               |

4. **Year-branch group rule** (groups start at 8/5/2, descend mod-9) is applied to these solar months, not lunar months. The month star for the group leader (Dần/Tỵ/Thân/Hợi years vs. Mão/Ngọ/Dậu/Tý years vs. Thìn/Mùi/Tuất/Sửu years) descends from the annual star at Lập Xuân.

---

## Consequences

- **Phase 13** wires `compute_monthly_flying_stars` against this resolver (`get_all_tiet_khi_for_year`). The function must be called to find which solar-term month a given Gregorian date falls in.
- **Annual Phi Tinh** (Niên Tử Bạch) also anchors year boundaries at Lập Xuân via the same scanner — consistent with ADR-0002 even though Niên is yearly.
- **Future revisions** to monthly anchor convention require a superseding ADR (ADR-0002a or later); this ADR may not be silently amended.
- **School divergence:** implementations that use lunar-month boundaries will produce different results from this implementation; divergences are logged as `KnownDivergence` per EXPANSION_FRAMEWORK §7.
