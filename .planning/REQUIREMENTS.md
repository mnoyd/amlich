# Requirements: Amlich Almanac Correctness Audit

**Defined:** 2026-02-28
**Core Value:** Every almanac subsystem must produce output that matches KHCBPPT for 2020–2030

## v1 Requirements

Requirements for initial audit. Each maps to roadmap phases.

### Source Establishment

- [x] **SRC-01**: KHCBPPT edition identified and documented in golden dataset metadata
- [x] **SRC-02**: Nạp Âm scope determined (KHCBPPT or "tam-menh-thong-hoi" — in or out of audit)
- [x] **SRC-03**: Intercalary month handling researched and documented from KHCBPPT text

### Golden Dataset

- [x] **DATA-01**: Golden reference dataset created with ~200 representative dates covering 2020–2030
- [x] **DATA-02**: Dataset covers all 12 chi, 10 can, 12 lunar months, 28 JD-cycle positions
- [x] **DATA-03**: Every golden entry includes KHCBPPT citation (`khcbppt_ref` field)
- [x] **DATA-04**: Golden loader (`golden_loader.rs`) deserializes dataset into typed Rust structs

### Taboo Rules

- [x] **TAB-01**: Tam Nương lunar day list cross-referenced against KHCBPPT
- [x] **TAB-02**: Nguyệt Kỵ lunar day list cross-referenced against KHCBPPT
- [x] **TAB-03**: Sát Chủ 12-month chi map cross-referenced against KHCBPPT
- [x] **TAB-04**: Thọ Tử 12-month chi map cross-referenced against KHCBPPT
- [ ] **TAB-05**: All divergences fixed in baseline.json

### Day Deity

- [x] **DEI-01**: 12-deity cycle order and classification (hoàng đạo/hắc đạo) cross-referenced
- [x] **DEI-02**: 12 month-start offsets (`month_group_start_by_chi`) cross-referenced
- [ ] **DEI-03**: All divergences fixed in baseline.json

### Trực Quality

- [x] **TRC-01**: All 12 trực quality assignments (cat/hung/binh) cross-referenced against KHCBPPT
- [ ] **TRC-02**: All divergences fixed in `TRUC_QUALITY` const in `truc.rs`

### Star Rules

- [x] **STR-01**: FixedByChi star assignments (12 chi) cross-referenced against KHCBPPT
- [x] **STR-02**: 28-star (Nhị Thập Bát Tú) JD epoch alignment verified (3+ dated entries)
- [x] **STR-03**: 28-star quality assignments (cat/hung/binh) cross-referenced
- [ ] **STR-04**: All divergences fixed in baseline.json

### Thần Hướng

- [x] **THH-01**: 10 stems × 3 directions (30 values) cross-referenced against KHCBPPT
- [ ] **THH-02**: All divergences fixed in baseline.json

### Xung Hợp

- [x] **XH-01**: Lục Xung, Tam Hợp, Tứ Hành Xung formula basis verified in KHCBPPT text
- [ ] **XH-02**: All divergences fixed in `xung_hop.rs`

### Nạp Âm

- [x] **NAM-01**: If in scope (per SRC-02), 30 nạp âm pairs cross-referenced against source
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

## v1.1 Delegated Requirement Accounting (Master Registry)

Machine-readable accounting rows for all v1.1 IDs. Canonical requirement definitions remain in `.planning/REQUIREMENTS-v1.1.md`; canonical verification evidence remains in `.planning/phases/v1.1-foundation-extensions/v1.1-VERIFICATION.md`.

| Requirement Set | Requirement ID | Scoped ID | Delegated Definition | Evidence Authority | Accounting Status |
|-----------------|----------------|-----------|----------------------|--------------------|-------------------|
| v1.1 | XH-01 | v1.1::XH-01 | .planning/REQUIREMENTS-v1.1.md#category-1-enhanced-xung-hợp-relationships | .planning/phases/v1.1-foundation-extensions/v1.1-VERIFICATION.md | Accounted |
| v1.1 | XH-02 | v1.1::XH-02 | .planning/REQUIREMENTS-v1.1.md#category-1-enhanced-xung-hợp-relationships | .planning/phases/v1.1-foundation-extensions/v1.1-VERIFICATION.md | Accounted |
| v1.1 | XH-03 | v1.1::XH-03 | .planning/REQUIREMENTS-v1.1.md#category-1-enhanced-xung-hợp-relationships | .planning/phases/v1.1-foundation-extensions/v1.1-VERIFICATION.md | Accounted |
| v1.1 | XH-04 | v1.1::XH-04 | .planning/REQUIREMENTS-v1.1.md#category-1-enhanced-xung-hợp-relationships | .planning/phases/v1.1-foundation-extensions/v1.1-VERIFICATION.md | Accounted |
| v1.1 | XH-05 | v1.1::XH-05 | .planning/REQUIREMENTS-v1.1.md#category-1-enhanced-xung-hợp-relationships | .planning/phases/v1.1-foundation-extensions/v1.1-VERIFICATION.md | Accounted |
| v1.1 | XH-06 | v1.1::XH-06 | .planning/REQUIREMENTS-v1.1.md#category-1-enhanced-xung-hợp-relationships | .planning/phases/v1.1-foundation-extensions/v1.1-VERIFICATION.md | Accounted |
| v1.1 | XH-07 | v1.1::XH-07 | .planning/REQUIREMENTS-v1.1.md#category-1-enhanced-xung-hợp-relationships | .planning/phases/v1.1-foundation-extensions/v1.1-VERIFICATION.md | Accounted |
| v1.1 | XH-08 | v1.1::XH-08 | .planning/REQUIREMENTS-v1.1.md#category-1-enhanced-xung-hợp-relationships | .planning/phases/v1.1-foundation-extensions/v1.1-VERIFICATION.md | Accounted |
| v1.1 | TC-01 | v1.1::TC-01 | .planning/REQUIREMENTS-v1.1.md#category-2-tàng-can-hidden-stems | .planning/phases/v1.1-foundation-extensions/v1.1-VERIFICATION.md | Accounted |
| v1.1 | TC-02 | v1.1::TC-02 | .planning/REQUIREMENTS-v1.1.md#category-2-tàng-can-hidden-stems | .planning/phases/v1.1-foundation-extensions/v1.1-VERIFICATION.md | Accounted |
| v1.1 | TC-03 | v1.1::TC-03 | .planning/REQUIREMENTS-v1.1.md#category-2-tàng-can-hidden-stems | .planning/phases/v1.1-foundation-extensions/v1.1-VERIFICATION.md | Accounted |
| v1.1 | TC-04 | v1.1::TC-04 | .planning/REQUIREMENTS-v1.1.md#category-2-tàng-can-hidden-stems | .planning/phases/v1.1-foundation-extensions/v1.1-VERIFICATION.md | Accounted |
| v1.1 | TC-05 | v1.1::TC-05 | .planning/REQUIREMENTS-v1.1.md#category-2-tàng-can-hidden-stems | .planning/phases/v1.1-foundation-extensions/v1.1-VERIFICATION.md | Accounted |
| v1.1 | TC-06 | v1.1::TC-06 | .planning/REQUIREMENTS-v1.1.md#category-2-tàng-can-hidden-stems | .planning/phases/v1.1-foundation-extensions/v1.1-VERIFICATION.md | Accounted |
| v1.1 | TC-07 | v1.1::TC-07 | .planning/REQUIREMENTS-v1.1.md#category-2-tàng-can-hidden-stems | .planning/phases/v1.1-foundation-extensions/v1.1-VERIFICATION.md | Accounted |
| v1.1 | TC-08 | v1.1::TC-08 | .planning/REQUIREMENTS-v1.1.md#category-2-tàng-can-hidden-stems | .planning/phases/v1.1-foundation-extensions/v1.1-VERIFICATION.md | Accounted |
| v1.1 | TC-09 | v1.1::TC-09 | .planning/REQUIREMENTS-v1.1.md#category-2-tàng-can-hidden-stems | .planning/phases/v1.1-foundation-extensions/v1.1-VERIFICATION.md | Accounted |
| v1.1 | TK-01 | v1.1::TK-01 | .planning/REQUIREMENTS-v1.1.md#category-3-tiết-khí-helper-functions | .planning/phases/v1.1-foundation-extensions/v1.1-VERIFICATION.md | Accounted |
| v1.1 | TK-02 | v1.1::TK-02 | .planning/REQUIREMENTS-v1.1.md#category-3-tiết-khí-helper-functions | .planning/phases/v1.1-foundation-extensions/v1.1-VERIFICATION.md | Accounted |
| v1.1 | TK-03 | v1.1::TK-03 | .planning/REQUIREMENTS-v1.1.md#category-3-tiết-khí-helper-functions | .planning/phases/v1.1-foundation-extensions/v1.1-VERIFICATION.md | Accounted |
| v1.1 | TK-04 | v1.1::TK-04 | .planning/REQUIREMENTS-v1.1.md#category-3-tiết-khí-helper-functions | .planning/phases/v1.1-foundation-extensions/v1.1-VERIFICATION.md | Accounted |
| v1.1 | TK-05 | v1.1::TK-05 | .planning/REQUIREMENTS-v1.1.md#category-3-tiết-khí-helper-functions | .planning/phases/v1.1-foundation-extensions/v1.1-VERIFICATION.md | Accounted |
| v1.1 | TK-06 | v1.1::TK-06 | .planning/REQUIREMENTS-v1.1.md#category-3-tiết-khí-helper-functions | .planning/phases/v1.1-foundation-extensions/v1.1-VERIFICATION.md | Accounted |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| SRC-01 | Phase 1 | Complete |
| SRC-02 | Phase 1 | Complete |
| SRC-03 | Phase 1 | Complete |
| DATA-01 | Phase 2 | Complete |
| DATA-02 | Phase 2 | Complete |
| DATA-03 | Phase 2 | Complete |
| DATA-04 | Phase 2 | Complete |
| TAB-01 | Phase 3 | Complete (03-03) |
| TAB-02 | Phase 3 | Complete (03-03) |
| TAB-03 | Phase 3 | Complete (03-03) |
| TAB-04 | Phase 3 | Complete (03-03) |
| TAB-05 | Phase 4 | Complete (04-01) |
| DEI-03 | Phase 4 | Complete (04-01) |
| TRC-02 | Phase 4 | Complete (04-01) |
| STR-04 | Phase 4 | Complete (04-01) |
| THH-02 | Phase 4 | Complete (04-01) |
| XH-02 | Phase 4 | Complete (04-01) |
| NAM-02 | Phase 4 | Complete (04-01) |

**Coverage:**
- v1 requirements: 27 total
- Mapped to phases: 27
- Unmapped: 0 ✓

---
*Requirements defined: 2026-02-28*
*Last updated: 2026-03-02 after 04-01 completion (Phase 4 complete)*
