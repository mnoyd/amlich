# Feature Research

**Domain:** Vietnamese almanac correctness validation against KHCBPPT
**Researched:** 2026-02-28
**Confidence:** HIGH (codebase analysis), MEDIUM (KHCBPPT content inference)

## Table Stakes (Audit Is Incomplete Without These)

| Feature | Complexity | Notes |
|---------|------------|-------|
| Star rules (cat_tinh/sat_tinh) cross-reference | HIGH | 5 category buckets (FixedByChi ×12, FixedByCanChi ×60, ByYear ×10, ByMonth ×12, ByTietKhi ×24); contextual buckets appear sparse — only 1 entry each |
| Taboo rules cross-reference (4 rules) | MEDIUM | Tam Nương days `[3,7,13,18,22,27]`, Nguyệt Kỵ `[5,14,23]`, Sát Chủ/Thọ Tử 12-month chi maps |
| Day deity cycle cross-reference | MEDIUM | 12 deity names × hoàng/hắc classification + 12 month-start offsets = 36 values |
| Trực quality assignments cross-reference | LOW | Formula is proven by structural tests; quality array `[cat,cat,hung,binh,cat,binh,hung,hung,cat,hung,cat,hung]` needs KHCBPPT confirmation |
| Xung hợp relationships verification | LOW | Algebraically derived (`+6%12`, `%4`, `%3`); needs textual confirmation |
| Thần hướng directions cross-reference | MEDIUM | 10 stems × 3 directions = 30 values; prior errors existed (commit 0f29f3f) |
| Nạp Âm scope determination | MEDIUM | `na_am_meta.source_id` is "tam-menh-thong-hoi" NOT "khcbppt" — determine if in scope |
| Golden reference dataset for 2020–2030 | HIGH | ~200 representative dates covering all pattern combinations |

## Differentiators (Deeper Validation)

| Feature | Complexity | Notes |
|---------|------------|-------|
| Star rule completeness audit (all 60 CanChi, all tiết khí) | HIGH | Determine whether sparse contextual buckets are correct or incomplete |
| 28-star JD epoch verification | MEDIUM | `jd.rem_euclid(28)` offset correctness; only test checks `index < 28`, not actual star name |
| Sát Hướng directional verification | MEDIUM | `conflict_by_chi` encodes `sat_huong` per chi |
| Precedence algorithm textual verification | MEDIUM | 6-tier precedence order needs KHCBPPT text confirmation |

## Anti-Features (Do NOT Validate)

| Feature | Reason |
|---------|--------|
| Lunar/solar conversion | Scoped out; separate concern, already well-tested |
| Giờ Hoàng Đạo (auspicious hours) | Separate calculation chain; not in DayFortune struct |
| New almanac subsystems | Focus on existing rules; document gaps but don't implement |
| TUI/CLI/WASM changes | Display layer; picks up corrected data automatically |
| Performance optimization | Correctness first |

## Critical Implementation Notes

- **Trực quality is hardcoded** in `truc.rs` as a Rust const array, NOT in `baseline.json`. Correction requires code change + recompile.
- **Current tests verify internal consistency only** — golden tests were written by the implementer against their own output, not KHCBPPT reference values.
- **Star rule contextual buckets are extremely sparse** — 1 entry each in CanChi, year, month, tiết-khí. Either KHCBPPT prescribes sparse rules or data is massively incomplete.

## Subsystem-to-Code Mapping

| Subsystem | Implementation | Data Location |
|-----------|---------------|---------------|
| Star rules (FixedByChi) | `than_sat.rs`, `star.rs` | `baseline.json: conflict_by_chi[*].cat_tinh/sat_tinh` |
| Star rules (contextual) | `than_sat.rs` | `baseline.json: star_rule_sets.*` |
| Taboo rules | `taboo.rs` | `baseline.json: taboo_rule_sets.*` |
| Day deity | `day_deity.rs` | `baseline.json: day_deity_rule_set.*` |
| Trực quality | `truc.rs` (TRUC_QUALITY const) | **Hardcoded in source, NOT baseline.json** |
| Xung hợp | `xung_hop.rs` | All logic in code |
| Thần hướng | `than_huong.rs` | `baseline.json: travel_by_can.*` |
| Nạp Âm | `calc.rs`, `data.rs` | `baseline.json: na_am_pairs` |
| 28 Stars | `calc.rs` | `baseline.json: nhi_thap_bat_tu` |

## Feature Dependencies

```
[KHCBPPT text extraction per subsystem]
    └──> [Golden Reference Dataset]
         └──> [Automated comparison harness]
              └──> [Divergence identification]
                   └──> [baseline.json / code corrections]

[Nạp Âm scope determination] ──blocker──> [Golden dataset schema]
[Trực + Day deity] ──coupled──> (both use month-chi relationship)
[Thần hướng] ──independent──> [Star rules] (can validate in parallel)
```

## Priority Matrix

| Feature | Audit Value | Cost | Priority |
|---------|------------|------|----------|
| Taboo rules (4 rules, ~20 values) | HIGH | LOW | P1 |
| Day deity cycle (36 values) | HIGH | LOW | P1 |
| Thần hướng (30 values) | HIGH | LOW | P1 |
| Trực quality (12 values) | HIGH | LOW | P1 |
| Nạp Âm scope determination | HIGH (blocker) | LOW | P1 |
| Golden reference dataset | HIGH | MEDIUM | P1 |
| Star rules completeness | HIGH | HIGH | P1 |
| Xung hợp formula verification | MEDIUM | LOW | P2 |
| 28-star JD epoch verification | MEDIUM | LOW | P2 |

---
*Feature research: 2026-02-28*
