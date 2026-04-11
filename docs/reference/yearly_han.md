# Yearly Hạn — Composite Assessment

**Source ID:** composite (aggregates multiple source_ids)
**Citation:** Vietnamese folk practice; individual components have own citations
**Confidence:** HIGH (structure), individual component confidence varies
**Decision:** DEC-0021

---

## Definition

"Hạn" (限) is a Vietnamese umbrella term for annual afflictions/obstacles. It is NOT a single unified system but a **composite of 5 independent checks**. A person can be affected by multiple hạn simultaneously ("hạn chồng hạn" — stacking afflictions).

## Components

| # | Component | Source | Calculation | Affects |
|---|---|---|---|---|
| 1 | **Sao Hạn** (Cửu Diệu) | `cuu-dieu` | tuổi_mụ mod 9, gender lookup | All activities |
| 2 | **Tam Tai** (三災) | `khcbppt` + VN adapt | Birth chi → Tam Hợp → 3-year cycle | All activities |
| 3 | **Kim Lâu** (金樓) | `ngoc-hap-ky` | tuổi_mụ mod 9, dư 1/3/6/8 | Marriage, construction |
| 4 | **Hoàng Ốc** (荒屋) | `vn-folk` | digit sum mod 6, positions 3/5/6 | Construction |
| 5 | **Thái Tuế** (太歲) | Both traditions | Branch conflicts with current year | All activities |

## Thái Tuế (太歲 / Grand Duke) — Component Detail

| Conflict Type | Vietnamese | Check |
|---|---|---|
| Trực Thái Tuế | Phạm Thái Tuế | Same branch as current year |
| Xung Thái Tuế | Xung Thái Tuế | Lục Xung (opposite branch) |
| Hại Thái Tuế | Hại Thái Tuế | Tương Hại relationship |
| Hình Thái Tuế | Hình Thái Tuế | Tương Hình relationship |
| Phá Thái Tuế | Phá Thái Tuế | Phá relationship |

**Note:** Xung Hợp module (`xung_hop.rs`) already computes all these relationships. Thái Tuế check = apply existing functions with current year branch as target.

## Composite Output Structure

```
YearlyHanAssessment {
  year: u16,
  person: { birth_year, gender },

  sao_han: Option<CuuDieuResult>,     // star affliction
  tam_tai: Option<TamTaiResult>,       // 3-calamity year
  kim_lau: Option<KimLauResult>,       // golden tower
  hoang_oc: Option<HoangOcResult>,     // desolate house
  thai_tue: Option<ThaiTueResult>,     // grand duke conflicts

  han_count: u8,                        // number of active hạn
  is_chong_han: bool,                   // true if han_count >= 2
  severity: low | medium | high | critical,
  summary_vi: String,
}
```

## Severity Heuristic

| Active Hạn Count | Severity |
|---|---|
| 0 | low (no affliction) |
| 1 | medium |
| 2 | high (hạn chồng hạn) |
| 3+ | critical |

## Implementation Notes

- Each component is independently calculated and sourced
- Composite aggregator calls each component, collects results
- Thái Tuế reuses existing `xung_hop` module functions
- Output preserves individual component details for transparency
- Gender required for Cửu Diệu and Kim Lâu gender-specific logic
