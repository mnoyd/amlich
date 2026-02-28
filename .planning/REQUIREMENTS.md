# Requirements: Amlich Almanac Correctness Audit

**Defined:** 2026-02-28
**Core Value:** Every almanac subsystem must produce output that matches KHCBPPT for 2020–2030

## v1 Requirements

Requirements for initial audit. Each maps to roadmap phases.

### Source Establishment

- [ ] **SRC-01**: KHCBPPT edition identified and documented in golden dataset metadata
- [ ] **SRC-02**: Nạp Âm scope determined (KHCBPPT or "tam-menh-thong-hoi" — in or out of audit)
- [ ] **SRC-03**: Intercalary month handling researched and documented from KHCBPPT text

### Golden Dataset

- [ ] **DATA-01**: Golden reference dataset created with ~200 representative dates covering 2020–2030
- [ ] **DATA-02**: Dataset covers all 12 chi, 10 can, 12 lunar months, 28 JD-cycle positions
- [ ] **DATA-03**: Every golden entry includes KHCBPPT citation (`khcbppt_ref` field)
- [ ] **DATA-04**: Golden loader (`golden_loader.rs`) deserializes dataset into typed Rust structs

### Taboo Rules

- [ ] **TAB-01**: Tam Nương lunar day list cross-referenced against KHCBPPT
- [ ] **TAB-02**: Nguyệt Kỵ lunar day list cross-referenced against KHCBPPT
- [ ] **TAB-03**: Sát Chủ 12-month chi map cross-referenced against KHCBPPT
- [ ] **TAB-04**: Thọ Tử 12-month chi map cross-referenced against KHCBPPT
- [ ] **TAB-05**: All divergences fixed in baseline.json

### Day Deity

- [ ] **DEI-01**: 12-deity cycle order and classification (hoàng đạo/hắc đạo) cross-referenced
- [ ] **DEI-02**: 12 month-start offsets (`month_group_start_by_chi`) cross-referenced
- [ ] **DEI-03**: All divergences fixed in baseline.json

### Trực Quality

- [ ] **TRC-01**: All 12 trực quality assignments (cat/hung/binh) cross-referenced against KHCBPPT
- [ ] **TRC-02**: All divergences fixed in `TRUC_QUALITY` const in `truc.rs`

### Star Rules

- [ ] **STR-01**: FixedByChi star assignments (12 chi) cross-referenced against KHCBPPT
- [ ] **STR-02**: 28-star (Nhị Thập Bát Tú) JD epoch alignment verified (3+ dated entries)
- [ ] **STR-03**: 28-star quality assignments (cat/hung/binh) cross-referenced
- [ ] **STR-04**: All divergences fixed in baseline.json

### Thần Hướng

- [ ] **THH-01**: 10 stems × 3 directions (30 values) cross-referenced against KHCBPPT
- [ ] **THH-02**: All divergences fixed in baseline.json

### Xung Hợp

- [ ] **XH-01**: Lục Xung, Tam Hợp, Tứ Hành Xung formula basis verified in KHCBPPT text
- [ ] **XH-02**: All divergences fixed in `xung_hop.rs`

### Nạp Âm

- [ ] **NAM-01**: If in scope (per SRC-02), 30 nạp âm pairs cross-referenced against source
- [ ] **NAM-02**: All divergences fixed in baseline.json

## v2 Requirements

Deferred to future audit cycle.

### Star Rule Completeness

- **STR-V2-01**: Full FixedByCanChi audit (all 60 sexagenary pairs)
- **STR-V2-02**: Full ByYear audit (all 10 heavenly stems)
- **STR-V2-03**: Full ByMonth audit (all 12 lunar months)
- **STR-V2-04**: Full ByTietKhi audit (all 24 solar terms)
- **STR-V2-05**: Precedence algorithm verified against KHCBPPT text

### Extended Validation

- **EXT-V2-01**: Sát Hướng directional verification per chi
- **EXT-V2-02**: Golden dataset extended beyond 2030
- **EXT-V2-03**: Gap inventory of KHCBPPT subsystems not yet implemented

## Out of Scope

| Feature | Reason |
|---------|--------|
| Lunar/solar date conversion | Already well-tested; separate concern |
| Giờ Hoàng Đạo (auspicious hours) | Separate calculation chain; not in DayFortune |
| New almanac subsystems | Focus on getting existing rules right |
| TUI/CLI/WASM/desktop changes | Display layer; picks up corrected data automatically |
| Performance optimization | Correctness first |
| Dates outside 2020–2030 | Practical daily use priority; cyclical rules provide coverage |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| SRC-01 | Phase 1 | Pending |
| SRC-02 | Phase 1 | Pending |
| SRC-03 | Phase 1 | Pending |
| DATA-01 | Phase 2 | Pending |
| DATA-02 | Phase 2 | Pending |
| DATA-03 | Phase 2 | Pending |
| DATA-04 | Phase 2 | Pending |
| TAB-01 | Phase 3 | Pending |
| TAB-02 | Phase 3 | Pending |
| TAB-03 | Phase 3 | Pending |
| TAB-04 | Phase 3 | Pending |
| TAB-05 | Phase 4 | Pending |
| DEI-01 | Phase 3 | Pending |
| DEI-02 | Phase 3 | Pending |
| DEI-03 | Phase 4 | Pending |
| TRC-01 | Phase 3 | Pending |
| TRC-02 | Phase 4 | Pending |
| STR-01 | Phase 3 | Pending |
| STR-02 | Phase 3 | Pending |
| STR-03 | Phase 3 | Pending |
| STR-04 | Phase 4 | Pending |
| THH-01 | Phase 3 | Pending |
| THH-02 | Phase 4 | Pending |
| XH-01 | Phase 3 | Pending |
| XH-02 | Phase 4 | Pending |
| NAM-01 | Phase 3 | Pending |
| NAM-02 | Phase 4 | Pending |

**Coverage:**
- v1 requirements: 27 total
- Mapped to phases: 27
- Unmapped: 0 ✓

---
*Requirements defined: 2026-02-28*
*Last updated: 2026-02-28 after roadmap creation*
