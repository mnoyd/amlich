# Requirements: Amlich v1.6 — Eastern Knowledge Completion

**Defined:** 2026-07-15
**Core Value:** Every almanac subsystem in amlich must produce output matching its canonical classical source for 2020-2030 with test-backed, traceable evidence.
**Milestone Goal:** Round out the Eastern Knowledge pillar — add the deferred daily Phi Tinh layer (日紫白), promote `RecommendsOffering` to a first-class semantic-graph node, and close the v1.5 review/confidence tech debt (RIT-11 reviewer field + ADR-0003 pre-1984 confidence).

REQs continue numbering from v1.5 archive (FND-01..06, RIT-01..13, FS-01..15, INT-01..06). v1.6 adds FND-07..08, RIT-14..16, FS-16..19, INT-07..09.

## v1.6 Requirements

### Foundation — ADR-0003 confidence closure

- [x] **FND-07**: User-of-ADR-0003 can find pre-1984 Thượng/Trung Nguyên polarity rows promoted from MEDIUM to HIGH confidence after an external cross-check (independent classical reference beyond *Thẩm Thị Huyền Không Học*).
- [x] **FND-08**: User-of-divergence-ledger can find the 1960 Trung Nguyên `KnownDivergence` either resolved with source attribution or explicitly logged as a deferred `PendingExternalReview` with reason + tiebreaker decision; ADR-0003 narrative updated to record the disposition.

### P1 Văn khấn — RIT-11 reviewer field closure

- [x] **RIT-14**: User-of-corpus can find every ritual entry carries a `reviewer` field with one of: an actual reviewer identity (name + date + outcome), or an explicit `ExternalReviewPending` deferral marker with documented reason and expected review date.
- [x] **RIT-15**: User-of-audit can find `provenance_audit.md` updated with a per-entry review record: reviewer identity, method-of-review (`independent-peer` / `cross-source` / `desk-check`), date reviewed, and outcome (`confirmed` / `corrected` / `disputed`).
- [ ] **RIT-16**: User-of-corpus can be confident that any entry whose review outcome was `corrected` has its `body_vi` re-verified against the cited source and that the corrected entry passes the existing `RitualEntry` JSON-schema + NFC-at-load guards.

### P4 Phi Tinh — Daily layer (Lưu Nhật / 日紫白)

- [ ] **FS-16**: User can call `compute_daily_flying_stars(date: NaiveDate, term_scanner: &TietKhiScanner) -> DailyFlyingStarLayout` returning the 9-palace daily grid, honouring 冬至/夏至 reversal semantics per a new daily-boundary ADR (FS-17).
- [ ] **FS-17**: User-of-Phi-Tinh can rely on a documented ADR capturing the daily starting-star convention (which year's annual chart seeds the daily count, and how 冬至/夏至 pivot reverses the forward sequence). The ADR cites chapter + page in *Thẩm Thị Huyền Không Học* and lists the alternative conventions considered.
- [ ] **FS-18**: User-of-validation can find a daily-chart golden dataset with ≥ 10 reference dates per Vận, ≥ 2 independent classical sources per case, *Thẩm Thị Huyền Không Học* as tiebreaker, and any source disagreements logged as `KnownDivergence` (not silently corrected).
- [ ] **FS-19**: User-of-`DaySnapshot` can find a new additive `daily_flying_stars: Option<DailyFlyingStarLayout>` field (`#[serde(default, skip_serializing_if = "Option::is_none")]`) with v1.5 fixtures round-tripping cleanly through the new field absent.

### Integration — `RecommendsOffering` semantic-graph node + daily wiring

- [ ] **INT-07**: User-of-semantic-graph can find a new `NodeConcept::Offering` variant with associated identity (`offering_id: String`, `name_vi: String`, `name_en: Option<String>`, `source_id: SourceId`), plus `EdgeConcept::RecommendsOffering` connecting `Ritual` → `Offering` carrying rationale + source provenance.
- [ ] **INT-08**: User-of-API can rely on `Ritual` node payload exposing `offering_refs: Option<Vec<OfferingRef>>` additively (preferred path) while keeping the legacy `offerings: Option<Vec<String>>` flat-string field present for backward compatibility — both fields `#[serde(default, skip_serializing_if = "Option::is_none")]`.
- [ ] **INT-09**: User-of-semantic-graph can be confident that `RecommendsOffering` edges carry dual-source provenance where the offering reference originates in a non-ritual tradition (e.g., a Huyền Không element cure surfaced inside a ritual carries both `huyen-khong` and `vn-folk-ritual` provenance on the edge), reusing the existing multi-source Direction-node dedup logic from v1.5.
- [ ] **INT-10**: User-of-`DaySnapshot` can find a v1.5→v1.6 backward-compat round-trip test that loads a v1.5 JSON fixture (with `flying_stars` but no `daily_flying_stars`) into v1.6 structs and re-serializes without unexpected fields, plus an end-to-end 2026 calendar smoke test on ≥ 5 representative dates that exercise both the existing annual/monthly fields and the new daily field.

## Out of Scope

| Feature | Reason |
|---------|--------|
| Hourly Phi Tinh (Lưu Thời) | Boundary semantics even more ambiguous than daily; corpus reliability lower; explicitly deferred per v1.5 EXPANSION_FRAMEWORK §2.3. |
| Spatial Phi Tinh (Tier 3, Sơn-Hướng, `Direction24`) | Deferred to P5 per EXPANSION_FRAMEWORK §3.3; requires new birth-data tier. |
| AI-generated / auto-personalized prayer text or offerings | Violates source provenance discipline (DEC-0015/0016). |
| Migrating legacy `Ritual.offerings: Vec<String>` to a deprecation warning | INT-08 keeps both fields live; deprecation window begins in a later milestone. |
| Per-user ritual history / journaling, audio prayer recordings | UI/app concerns, not engine. |
| Kinh Dịch (P2), Y học (P3), Tử Vi (P6) | Deferred per EXPANSION_FRAMEWORK §5 sequencing. |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| FND-07 | Phase 16 | **Complete** (2026-07-15, 16-01) |
| FND-08 | Phase 16 | **Complete** (2026-07-15, 16-02) |
| RIT-14 | Phase 17 | **Complete** (2026-07-15, 17-01) |
| RIT-15 | Phase 17 | **Complete** (2026-07-15, 17-01) |
| RIT-16 | Phase 17 | Pending |
| FS-16 | Phase 18 | Pending |
| FS-17 | Phase 18 | Pending |
| FS-18 | Phase 18 | Pending |
| FS-19 | Phase 18 | Pending |
| INT-07 | Phase 19 | Pending |
| INT-08 | Phase 19 | Pending |
| INT-09 | Phase 19 | Pending |
| INT-10 | Phase 19 | Pending |

**Coverage:**
- v1.6 requirements: 12 total
- Mapped to phases: 12 ✓
- Unmapped: 0 ✓
- Phases: 4 (16 Foundation, 17 Văn khấn Reviewer Closure, 18 Daily Phi Tinh, 19 RecommendsOffering + Integration)

---
*Requirements defined: 2026-07-15*
*Last updated: 2026-07-15 — RIT-14 + RIT-15 marked Complete after 17-01 execution (provenance_audit.md expanded to 8-column review record; 60/60 ExternalReviewPending markers; 0/0/0/60 outcome breakdown; ledger remains canonical record; no JSON schema changes; 888/888 tests still pass — 6/6 rituals_integration confirmed).*
