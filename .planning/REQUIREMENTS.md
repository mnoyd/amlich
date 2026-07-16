# Requirements: Amlich v1.7 — Kinh Dịch (I-Ching Divination)

**Defined:** 2026-07-16
**Core Value:** Every almanac subsystem in amlich must produce output matching its canonical classical source for 2020-2030 with test-backed, traceable evidence.
**Milestone Goal:** Add the P2 Kinh Dịch pillar — Mai Hoa Dịch Số casting + 64-hexagram lookup — as a new Tier-0 reasoning capability, plus the Thái Tuế/Tam Sát ⇄ Phi Tinh directional cross-link (a carry-forward "should-have" from v1.5 research). Both ship as additive, schema-locked surfaces with no new crate dependencies and CRIT-3 isolation preserved.

REQs continue numbering from v1.6 archive (FND-01..08, RIT-01..16, FS-01..19, INT-01..10). v1.7 adds FND-09..12, ICH-01..05 (new category), XLK-01..03 (new category), INT-11..13.

## v1.7 Requirements

### Foundation — Schema Lock + Sources + ADRs + Ontology

- [x] **FND-09**: User-of-sources can find two new `pub const` source_ids registered — `SOURCE_KINH_DICH` (`"kinh-dich"`, for hexagram text corpus) and `SOURCE_MAI_HOA_DICH_SO` (`"mai-hoa-dich-so"`, for the casting algorithm) — with the CI `source_id_guard.rs` extended so bare literals at provenance call-sites are forbidden (DEC-0023 discipline).
- [x] **FND-10**: User-of-ADRs can find three accepted decisions — ADR-0005 (IChing `HexagramEntry` schema v1 with `deny_unknown_fields`), ADR-0006 (Mai Hoa casting convention: Tiên Thiên arrangement pinned to a page reference, lunar input, `((n-1)%k)+1` remainder-zero convention), and ADR-0007 (cross-link CRIT-3 carve-out: read-only `reasoning/` placement + composite `rule.composite.direction_cross_link` envelope).
- [x] **FND-11**: User-of-schema can find a locked `HexagramEntry` type with `#[serde(deny_unknown_fields)]` + a passing 1-entry serde round-trip probe BEFORE any of the 64 corpus entries are authored (CRIT-1 schema-lock-first), plus three distinct newtypes `TienThienTrigram(u8)` / `HauThienTrigram(u8)` / `KingWenHexagram(u8)` with NO `From` impl between them (CRIT-3 Mai Hoa vs King Wen prevention) and a 64-entry Tiên Thiên-pair → King Wen composition table validated at load.
- [x] **FND-12**: User-of-ontology can find the 6-slice ontology extended with `NodeConcept::Hexagram`, `EdgeConcept::LocatedAt`, and `EdgeConcept::Transforms` (compiler-enforced exhaustive match across all slice locations), plus `ReasoningEvidenceSourceFamily::IChing` and `ActionId::IChing` enum variants.

### P2 Kinh Dịch — Corpus + Casting + Evaluator

- [x] **ICH-01**: User-of-corpus can find a 64-hexagram lookup (`data/iching/hexagrams.json`, NFC-normalized at load, reviewer-signed) where each `HexagramEntry` carries `king_wen_index`, `vi_name`, `upper/lower_trigram`, `thoai_tu` (judgment), `hao_tu` (6 line texts, 7 for hexagrams 1 & 2), and `cat_hung` verdict — loaded via `include_str!` + `OnceLock`; gaps in the Ngô Tất Tố source are logged as `PendingExternalReview`, never silently filled from another translator.
- [x] **ICH-02**: A caller can invoke `cast_mai_hoa(lunar_year_branch, lunar_month, lunar_day, chi_hour_index) -> MaiHoaCast` returning the upper/lower Tiên Thiên trigram pair + động hào (moving line), deterministic with no RNG, honouring the `((n-1)%k)+1` remainder-zero convention (CRIT-2).
- [x] **ICH-03**: A caller can derive the biến quẻ (transforming hexagram) from a `MaiHoaCast` by flipping the động hào bit and re-composing, verifiable by a 384-case (64 chủ quẻ × 6 động hào) exhaustive contract test (CRIT-4).
- [x] **ICH-04**: A reader of a cast result can find the Thể (the trigram NOT containing the động hào) and Dụng (the trigram containing it) classification plus the Ngũ Hành sinh/khắc relationship driving the cát/hùng reading.
- [ ] **ICH-05**: A caller can construct an `IChingQuery` (sibling newtype, NOT a `ConsultationIntent` variant) and run it through an `IChingEvaluator` that emits per-step `ReasoningEvidenceEnvelope` instances with distinct source_ids (`mai-hoa-dich-so` for casting + `kinh-dich` for text lookup) plus one composite envelope (CRIT-6) — works fully at Tier 0 (no birth data required).

### Thái Tuế / Tam Sát ⇄ Phi Tinh Cross-Link

- [x] **XLK-01**: User-of-directions can find a Thái Tuế directional derivation (`pub fn` on `thai_tue.rs` mapping year-chi → `Direction8`) distinct from the existing personal-conflict-only Thái Tuế, carrying `source_id: khcbppt` evidence (the two 1-line `evidence: None` backfills on `thai_tue.rs` + `sat_phuong.rs` are populated).
- [x] **XLK-02**: User-of-directions can find a classical Tam Sát directional module (`almanac/tam_sat.rs`) returning the THREE contiguous sơn/directions per year from the Tam Hợp triad opposition (per a KHCBPPT-pinned citation), carrying `source_id: khcbppt`; the existing single-direction `sat_phuong.rs` day-chi feature stays intact. *(Decision-dependent — DEC required first; recommended option b per research.)*
- [x] **XLK-03**: User-of-reasoning can find a read-only `build_direction_cross_link(snapshot, birth_chi_index)` in `reasoning/direction_composite.rs` that surfaces BOTH the KHCBPPT Thái Tuế/Tam Sát directional taboos AND the `huyen-khong` Phi Tinh palace layout in one composite fact node — emitting distinct primitive `source_id` envelopes (`khcbppt` + `huyen-khong`) plus one `rule.composite.direction_cross_link` envelope, with CRIT-3 isolation preserved (no `FlyingStar` reference in `interaction/direction_merge.rs`; grep-guarded by a sibling `tests/thai_tue_cross_link_crit3.rs`).

### Integration — Semantic Graph + DTO + E2E

- [ ] **INT-11**: User-of-semantic-graph can find Hexagram nodes (chủ quẻ + biến quẻ) wired via `LocatedAt`/`Transforms` edges and a composite cross-link fact node, emitted by additive `add_iching_facts()` + `add_direction_composite_facts()` builder methods (v1.5 FlyingStar/Offering precedent).
- [ ] **INT-12**: User-of-`DaySnapshot` can find additive `iching_cast: Option<IChingCastSummary>` and `direction_cross_link: Option<DirectionCrossLinkSummary>` fields (`#[serde(default, skip_serializing_if = "Option::is_none")]`), with a v1.6→v1.7 backward-compat round-trip test proving a v1.6 producer JSON deserializes cleanly and re-serializes without unexpected fields.
- [ ] **INT-13**: User-of-validation can find ≥10 IChing golden casting cases cross-checked against ≥2 independent sources (divergences logged as `KnownDivergence`, not silently corrected) plus a 2026 E2E smoke extending `integration_2026_smoke.rs`, and the full crate test suite green with zero regressions.

## Out of Scope

| Feature | Reason |
|---------|--------|
| Coin / yarrow / RNG casting (AF-01) | Different tradition; breaks determinism; would need a third `source_id`. v1.7 ships time-numerology (Mai Hoa) only. |
| LLM-generated free-form interpretation (AF-02) | Ngô Tất Tố corpus IS the interpretation; violates source provenance (DEC-0015/0016). Surface verbatim. |
| Spatial feng-shui composition / `FlyingStar` into `direction_merge.rs` (AF-03) | CRIT-3 violation; deferred to Tier-3 (P5) per EXPANSION_FRAMEWORK §3.3. |
| Personal Thái Tuế rewrite from cross-link (AF-04) | Cross-link is read-only by design; directional taboos stay `source_id: khcbppt`. |
| Hỗ Quả (nuclear hexagram) (DF-03) | Depth feature; defer to v1.9+. |
| Tier-2 Bazi enrichment of hexagram reading (DF-01) | Tier-0 baseline first this milestone; mirrors v1.5 Phi Tinh T0/T2 split. |
| User-selectable casting variants (số vật / âm thanh) (AF-06) | Out of scope; ship Mai Hoa time-numerology only. |
| Tử Vi Đẩu Số (P6), Y học Tý Ngọ Lưu Chú (P3) | Deferred per EXPANSION_FRAMEWORK §5 sequencing. |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| FND-09 | 20 | Complete |
| FND-10 | 20 | Complete |
| FND-11 | 20 | Complete |
| FND-12 | 20 | Complete |
| ICH-01 | 21 | Complete (21-01 DATA + 21-02 CODE both shipped; 8 black-box integration tests) |
| ICH-02 | 22 | Complete (22-01 PLAN: cast_mai_hoa + MaiHoaCast + CRIT-2 boundary test + 51,840-cast range sweep; 5 inline + 6 black-box integration tests passing) |
| ICH-03 | 22 | Complete (22-01 PLAN: derive_bien_que + BienQue + CRIT-4 384-case contract test + worked (8,8,8,8)→#7 Sư; 4 inline + 6 black-box integration tests passing) |
| ICH-04 | 22 | Complete (22-02 PLAN: classify_the_dung + TheDungClassification struct + TheDungRelation enum (5-way sinh/khắc) + CatHung verdict + trigram_element; 7 inline + 11 black-box integration tests passing; ≥10-case cross-source golden dataset at crates/amlich-core/data/iching/mai_hoa_golden.json with FS-10 dual-source discipline + 2 KnownDivergence rows) |
| ICH-05 | 24 | Pending |
| XLK-01 | 23 | Complete (23-01 PLAN: thai_tue_direction(year_chi_index) sibling API + 8-point Direction mapping for all 12 branches + compute_thai_tue + get_sat_phuong KHCBPPT evidence backfills + almanac_backfill_compat BC round-trip gate; 13 thai_tue lib tests + 7 sat_phuong lib tests + 6 compat tests green) |
| XLK-02 | 23 | Complete (23-01 PLAN: almanac::tam_sat::tam_sat_direction classical three-direction module with tradition-ordered TAM_SAT_ROWS table (Water/Wood/Fire/Metal) + lục-xung opposite-triad mapping per CONTEXT.md + discoverable data/almanac/tam_sat_provenance.md PendingExternalReview ledger; 5 tam_sat lib tests + 10 tam_sat_integration tests green) |
| XLK-03 | 23 | Complete (23-03 PLAN: read-only `build_direction_cross_link_personal`/`_date` + `build_direction_cross_link` PersonalFactNode wrapper + `project_to_summary` + immutable `enrich_day_snapshot_with_direction_cross_link` helper at crate root + sibling `tests/thai_tue_cross_link_crit3.rs` seven-pattern two-target CRIT-3 grep guard + 22-test `tests/direction_cross_link_integration.rs` black-box gate; conservative-default majority-vote severity with three inline unit tests; runtime-built huyen-khong method string == `phi_tinh.palace_layout`; 18 lib + 22 integration + 3 crit3-guard tests green) |
| INT-11 | 24 | Pending |
| INT-12 | 24 | Pending |
| INT-13 | 25 | Pending |

**Coverage:**
- v1.7 requirements: 15 total
- Mapped to phases: 15 / 15 ✓ (no orphans, no duplicates)
- Unmapped: 0
- New categories: ICH (I-Ching), XLK (cross-link)

**Phase groupings:**
- Phase 20 (Foundation, BLOCKING): FND-09, FND-10, FND-11, FND-12
- Phase 21 (IChing Corpus + Loader): ICH-01
- Phase 22 (Mai Hoa Casting + Biến Quẻ + Thể/Dụng): ICH-02, ICH-03, ICH-04
- Phase 23 (Thái Tuế / Tam Sát ⇄ Phi Tinh Cross-Link, PARALLEL): XLK-01, XLK-02, XLK-03
- Phase 24 (IChing Evaluator + Semantic-Graph Wiring + DTO): ICH-05, INT-11, INT-12
- Phase 25 (E2E Validation): INT-13

---
*Requirements defined: 2026-07-16*
*Research basis: `.planning/research/SUMMARY.md` (HIGH confidence, 8-phase structure suggested)*
