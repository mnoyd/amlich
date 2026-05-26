# Requirements: Amlich v1.5 — Eastern Knowledge Expansion

**Defined:** 2026-05-25
**Core Value:** Every almanac subsystem in amlich must produce output matching its canonical classical source for 2020-2030 with test-backed, traceable evidence.
**Milestone Goal:** Add two new pillars (P1 Văn khấn cổ truyền under `source_id: vn-folk-ritual`, P4 Phi Tinh thời gian under `source_id: huyen-khong`) to amlich-core. Both Tier 0. No spatial input. First milestone where multiple source traditions coexist as new code.

## v1.5 Requirements

### Foundation (schema lock + cross-cutting ADRs)

- [ ] **FND-01**: User-of-API can rely on a frozen `RitualEntry` JSON schema v1 (typed `event_keys[]`, structured `offerings[]`, structured `preparation_steps[]`, required `source_id`, `original_citation`, `confidence`) — locked before corpus authoring begins.
- [ ] **FND-02**: User-of-API can rely on a frozen `FlyingStarLayout` API shape (`Period`, `[FlyingStar; 9]` palace array, `center_star`, `evidence`) — locked before algorithm work begins.
- [x] **FND-03**: User-of-source-taxonomy can find `vn-folk-ritual` and `huyen-khong` registered as distinct source IDs alongside existing `khcbppt` / `ngoc-hap-ky` / `vn-folk` / `cuu-dieu` / `tam-menh-thong-hoi`, with module-level `pub const SOURCE_*` constants preventing typos.
- [ ] **FND-04**: User-of-Phi-Tinh can rely on a documented decision for monthly anchor convention (solar-term boundaries per *Thẩm Thị Huyền Không Học*, reusing the v1.1.2 Tiết Khí scanner) captured as an ADR.
- [ ] **FND-05**: User-of-Phi-Tinh can rely on a documented decision for Niên Tử Bạch direction rule per Tam Nguyên × year polarity, captured as an ADR with a polarity matrix.
- [ ] **FND-06**: User-of-holidays can rely on `Holiday.id: Option<String>` (additive, `#[serde(default)]`) populated from `lunar_festivals[].id` — round-trip compatible with v1.4 JSON fixtures.

### P1 Văn khấn — Module & Lookup APIs

- [ ] **RIT-01**: User can resolve `find_van_khan_for_snapshot(&DaySnapshot) -> Vec<RitualEntry>` to discover all rituals matching the day's lunar date, Tiết Khí anchor, and active holidays.
- [ ] **RIT-02**: User can resolve `find_van_khan_for_event(&RitualEventKey) -> Vec<RitualEntry>` for direct event-based lookup (Sóc/Vọng, 8 lunar festivals, life events).
- [ ] **RIT-03**: User can resolve `find_van_khan_for_life_event(LifeEventKind) -> Vec<RitualEntry>` for life-cycle events (Động thổ, Nhập trạch, Khai trương, Cưới, Giỗ, Đầy tháng).
- [ ] **RIT-04**: User can call `get_ritual_by_id(&str) -> Option<RitualEntry>` to fetch a single ritual by stable id.
- [ ] **RIT-05**: User can call `all_rituals() -> &'static [RitualEntry]` to iterate the entire corpus.
- [ ] **RIT-06**: User can rely on a closed `RitualEventKey` enum covering Sóc/Vọng (Mùng 1, Rằm), the 8 major lunar festivals (Tết Nguyên Đán, Khai Hạ, Thượng Nguyên, Thanh Minh, Đoan Ngọ, Vu Lan, Trung Thu, Ông Công Ông Táo), Tiết Khí anchors, life events, and `Always`.
- [ ] **RIT-07**: User can rely on `LunarDateMatch` variants (`MonthDay { month, day, leap_month_policy }`, `SolarTerm`, `GregorianFixed`) — leap-month policy defaults to `CanonicalMonthOnly`.
- [x] **RIT-08**: User-of-corpus can be confident every entry's text is NFC-normalized at load and that the loader rejects Hán-character pollution above a CI-enforced threshold.

### P1 Văn khấn — Corpus Authoring (Editorial)

- [ ] **RIT-09**: User can find at least **60 ritual entries** in the shipped corpus, spread across ≤ 14 per-event-category JSON files plus a `manifest.json`.
- [ ] **RIT-10**: User can verify every ritual entry carries `source_id: "vn-folk-ritual"`, an `original_citation` (book + page), and a `confidence` tier (`primary` / `regional-variant` / `synthesized`).
- [ ] **RIT-11**: User can find a per-entry `provenance_audit.md` ledger documenting which classical reference each entry was drawn from and which independent reviewer confirmed the citation.
- [ ] **RIT-12**: User can find ritual **variants** for at least 4 events (e.g., Tết Nguyên Đán: simple / full / Buddhist / folk) — all sharing the same `event_type` with a discriminating `variant` field on `RitualEntry`.
- [ ] **RIT-13**: User-of-schema can rely on a reserved `body_en: Option<String>` field on `RitualEntry` (English content authoring deferred to a future milestone — schema reservation only).

### P4 Phi Tinh — Primitives & Period

- [ ] **FS-01**: User can call `compute_period(year: i32, term_scanner: &TietKhiScanner) -> Period` returning the active Vận accounting for Lập Xuân, not Jan 1.
- [ ] **FS-02**: User can rely on a `Palace` enum with canonical Lo Shu numbering (N=1, NE=8, E=3, SE=4, S=9, SW=2, W=7, NW=6, Center=5) and a static `palace_to_direction()` mapping.
- [ ] **FS-03**: User can rely on a `FlyingStar` enum (NhatBach=1 … CuuTu=9) with associated element + polarity + auspice metadata loaded from `data/almanac/flying_stars.json`.
- [ ] **FS-04**: User-of-Phi-Tinh can be confident every base palace table (Vận 1-9) is validated at load by Lo Shu invariants (sum=45, each 1-9 once, center = Vận number).
- [ ] **FS-05**: User can find Vận 7 (1984–2003), Vận 8 (2004–2023), and Vận 9 (2024–2043) all populated and golden-tested at boundary instants.

### P4 Phi Tinh — Annual & Monthly Computation

- [ ] **FS-06**: User can call `compute_yearly_flying_stars(year, term_scanner) -> FlyingStarLayout` returning the 9-palace annual grid. Verified against multiple sources for ≥ 10 dates per Vận.
- [ ] **FS-07**: User can call `compute_monthly_flying_stars(year, month, term_scanner) -> FlyingStarLayout` returning the 9-palace monthly grid using the year-branch-group rule (groups start at 8/5/2, descend mod-9), with solar-term month boundaries per FND-04.
- [ ] **FS-08**: User can call `compute_combined_overlay(year, month, term_scanner) -> CombinedFlyingStarLayout` returning `[(annual_star, monthly_star); 9]` per palace.
- [ ] **FS-09**: User-of-evidence can find per-sub-star `ReasoningEvidenceEnvelope` entries (separate envelopes for Vận, Niên, Nguyệt) plus a composite `rule.composite.flying_stars` envelope on aggregate outputs.
- [ ] **FS-10**: User-of-validation can find a Phi Tinh golden dataset with ≥ 10 dates per Vận, ≥ 2 reference sources per case, *Thẩm Thị Huyền Không Học* as tiebreaker, and `KnownDivergence` entries logged (not silently corrected).

### P4 Phi Tinh — 2-Star Aspects (81-cell)

- [ ] **FS-11**: User can call `lookup_star_pair_aspect(star_a: FlyingStar, star_b: FlyingStar) -> StarPairAspect` returning the digitized interpretation (name, ngũ-hành relation, auspice, classical citation) for all 81 ordered pairs.
- [ ] **FS-12**: User can rely on `StarPairAspect` carrying `source_id: "huyen-khong"`, an `original_citation` pointing to a specific chapter of *Thẩm Thị Huyền Không Học*, and a `confidence` tier.
- [ ] **FS-13**: User-of-Phi-Tinh can call `compute_palace_aspects(year, month, term_scanner) -> [StarPairAspect; 9]` returning the per-palace aspect derived from the combined overlay (FS-08).

### P4 Phi Tinh — Safety / Cures (advisory only)

- [ ] **FS-14**: User can find an `is_danger_palace()` predicate on `FlyingStar` (true for Ngũ Hoàng and Nhị Hắc per classical tradition).
- [ ] **FS-15**: User can call `element_hint_for_palace(star: FlyingStar) -> Option<RemedyHint>` returning a Ngũ-Hành mitigation hint (kim/mộc/thủy/hỏa/thổ) with classical citation — **never** product names.

### Integration

- [ ] **INT-01**: User-of-`DaySnapshot` can find a new `flying_stars: Option<FlyingStarsSummary>` field (additive, `#[serde(default, skip_serializing_if = "Option::is_none")]`).
- [ ] **INT-02**: User-of-`DaySnapshot` can find a new ritual-surfacing field (additive, optional) exposing rituals applicable to the day.
- [ ] **INT-03**: User-of-semantic-graph can find new `NodeConcept::Ritual` and `NodeConcept::FlyingStar` variants with `EdgeConcept::PrescribedFor`, `EdgeConcept::OccupiesPalace`, and `EdgeConcept::CarriesElement`.
- [ ] **INT-04**: User-of-`semantic_graph` can be confident that `FlyingStar` nodes carry **only** `source_id: "huyen-khong"`, ritual nodes carry **only** `source_id: "vn-folk-ritual"`, and a `Direction` node shared between KHCBPPT direction modules and Huyền Không Phi Tinh carries **both** provenance entries (multi-source dedup verified).
- [ ] **INT-05**: User-of-`DaySnapshot` can load a v1.4 JSON fixture into v1.5 structs and re-serialize without unexpected fields (backward-compat round-trip).
- [ ] **INT-06**: User can run an end-to-end 2026 calendar smoke test on ≥ 30 representative dates covering Tết Nguyên Đán, Sóc/Vọng × 12, Vận 8→9 transition dates, leap-month dates, and all 24 Tiết Khí boundaries.

## Out of Scope

| Feature | Reason |
|---------|--------|
| Spatial Phi Tinh (Tier 3, Sơn-Hướng, `Direction24` input) | Deferred to P5 per EXPANSION_FRAMEWORK §3.3; requires new birth-data tier. |
| Daily / Hourly Phi Tinh (Lưu Nhật, Lưu Thời) | Boundary semantics need separate ADR; corpus reliability lower. |
| AI-generated / auto-personalized prayer text | Violates source provenance discipline (DEC-0015/0016). |
| Audio prayer recordings, full-text search across `khan_text`, per-user prayer history, user-editable corpus | UI/app concerns, not engine. |
| "Cure" product recommendations, Vận-transition alerts | Commercial/stateful; out of engine scope. |
| Holiday-side `ritual_ids[]` cross-link field | User opted out — matcher works via `Holiday.id` + `RitualEntry.event_keys[]` already (FND-06). |
| Holiday-driven auto-recommendation in `holidays.rs` | One-way dependency (`rituals` → `holidays`) preserved; recommendation lives in `rituals` module. |
| Tử Vi Đẩu Số (P6), Kinh Dịch (P2), Y học (P3) | Deferred to later milestones per EXPANSION_FRAMEWORK §5 sequencing. |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| FND-01 | Phase 10 | Pending |
| FND-02 | Phase 10 | Pending |
| FND-03 | Phase 10 | Complete (10-01) |
| FND-04 | Phase 10 | Pending |
| FND-05 | Phase 10 | Pending |
| FND-06 | Phase 10 | Pending |
| RIT-01 | Phase 11 | Pending |
| RIT-02 | Phase 11 | Pending |
| RIT-03 | Phase 11 | Pending |
| RIT-04 | Phase 11 | Pending |
| RIT-05 | Phase 11 | Pending |
| RIT-06 | Phase 11 | Pending |
| RIT-07 | Phase 11 | Pending |
| RIT-08 | Phase 11 | Complete |
| RIT-09 | Phase 12 | Pending |
| RIT-10 | Phase 12 | Pending |
| RIT-11 | Phase 12 | Pending |
| RIT-12 | Phase 12 | Pending |
| RIT-13 | Phase 12 | Pending |
| FS-01 | Phase 13 | Pending |
| FS-02 | Phase 13 | Pending |
| FS-03 | Phase 13 | Pending |
| FS-04 | Phase 13 | Pending |
| FS-05 | Phase 13 | Pending |
| FS-06 | Phase 13 | Pending |
| FS-07 | Phase 13 | Pending |
| FS-08 | Phase 13 | Pending |
| FS-09 | Phase 13 | Pending |
| FS-10 | Phase 13 | Pending |
| FS-11 | Phase 14 | Pending |
| FS-12 | Phase 14 | Pending |
| FS-13 | Phase 14 | Pending |
| FS-14 | Phase 14 | Pending |
| FS-15 | Phase 14 | Pending |
| INT-01 | Phase 15 | Pending |
| INT-02 | Phase 15 | Pending |
| INT-03 | Phase 15 | Pending |
| INT-04 | Phase 15 | Pending |
| INT-05 | Phase 15 | Pending |
| INT-06 | Phase 15 | Pending |

**Coverage:**
- v1.5 requirements: 40 total
- Mapped to phases: 40 ✓
- Unmapped: 0 ✓

---
*Requirements defined: 2026-05-25*
*Last updated: 2026-05-25 after v1.5 roadmap creation (traceability filled with phase mappings)*
