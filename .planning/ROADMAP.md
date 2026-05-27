# Roadmap: Amlich Almanac Correctness Audit

## Milestones

- ✅ **v1.4 Lunar Engine Table Parity** - shipped 2026-03-04 (Phases 7-9, 6/6 plans), archived in `.planning/milestones/v1.4-ROADMAP.md`
- 🚧 **v1.5 Eastern Knowledge Expansion** - started 2026-05-23 (Phases 10-15)

## Current Focus

- Execute v1.5 phases: Văn khấn (`vn-folk-ritual`) + Phi Tinh thời gian (`huyen-khong`).
- Schema-lock first (Phase 10) gates all corpus authoring downstream.

---

# v1.5 Eastern Knowledge Expansion

**Milestone goal:** Add two Tier-0 pillars to `amlich-core` — P1 Văn khấn cổ truyền (`source_id: vn-folk-ritual`) and P4 Phi Tinh thời gian (`source_id: huyen-khong`) — coexisting as new code beside the entrenched `khcbppt` family per DEC-0015/0016.

**Phase numbering:** Continues from v1.4 (last phase 9 — `09-na-am-api-surfaces`). v1.5 starts at Phase 10.

**Depth:** quick (compressed; 6 phases for 40 requirements).

**Coverage:** 40 / 40 requirements mapped ✓

## Phases

- [x] **Phase 10: Foundation — Schema Lock + ADRs + Source-ID Registration** - Lock ritual + Flying Star schemas, register two new source_ids, write the three blocking ADRs.
- [x] **Phase 11: Văn khấn Module + Lookup APIs** - Ship the `rituals/` module with closed event enum, structured matchers, and the five public lookup APIs. (completed 2026-05-26)
- [x] **Phase 12: Văn khấn Corpus Authoring** - Author ≥60 entries across ≤14 per-event-category JSON files with full provenance audit and 4+ event-variant coverage. (completed 2026-05-27)
- [x] **Phase 13: Phi Tinh Primitives + Period + Annual/Monthly** - Ship `almanac/fengshui/` with Lo Shu validators, Vận 7-9 tables, and annual/monthly/combined layout APIs. (completed 2026-05-27)
- [x] **Phase 14: Phi Tinh 81-cell Aspects + Safety Hints** - Digitize the 81 star-pair aspects from *Thẩm Thị Huyền Không Học* and ship the advisory danger/element-hint APIs. (completed 2026-05-27)
- [x] **Phase 15: Semantic Graph Wiring + DTO Integration + E2E Validation** - Add `Ritual` and `FlyingStar` node concepts, wire additive `DaySnapshot` fields, run 2026 smoke + v1.4 round-trip. (completed 2026-05-27)

## Phase Details

### Phase 10: Foundation — Schema Lock + ADRs + Source-ID Registration

**Goal**: User-of-API can rely on frozen v1 schemas for both pillars and a documented source-taxonomy so corpus authoring and algorithm work can begin without churn risk.

**Depends on**: Nothing (first phase of milestone — hard gate per PITFALLS research).

**Requirements**: FND-01, FND-02, FND-03, FND-04, FND-05, FND-06

**Success Criteria** (what must be TRUE):
  1. A caller can deserialize a sample `RitualEntry` JSON with the v1 schema (typed `event_keys[]`, structured `offerings[]`, structured `preparation_steps[]`, required `source_id`, `original_citation`, `confidence`) and any extra field is rejected by `#[serde(deny_unknown_fields)]`.
  2. A caller can construct a `FlyingStarLayout { period, palaces: [FlyingStar; 9], center_star, evidence }` value and the shape is frozen by an ADR before any Vận table work begins.
  3. A code reader can find `pub const SOURCE_VN_FOLK_RITUAL: &str = "vn-folk-ritual"` and `pub const SOURCE_HUYEN_KHONG: &str = "huyen-khong"` at module level, with both ids also documented in the source-taxonomy memory.
  4. A reader can find three ADRs in `.planning/`: ritual JSON schema v1, monthly Phi Tinh anchor convention (solar-term per *Thẩm Thị Huyền Không Học*, reusing the v1.1.2 Tiết Khí scanner), and Niên Tử Bạch direction rule per Tam Nguyên × year polarity (with the polarity matrix).
  5. A v1.4 JSON fixture loads into the v1.5 `Holiday` struct (now carrying `id: Option<String>` with `#[serde(default)]`) and re-serializes round-trip without unexpected fields.

**Plans**: 5 plans across 3 waves
- [x] 10-01-PLAN.md — Wave 1 — sources.rs registry + migration sweep + CI grep guard + rituals/ stubs + lib.rs registration (FND-03)
- [x] 10-02-PLAN.md — Wave 1 — Holiday.id additive field + LunarFestivalData.id exposure (FND-06)
- [x] 10-03-PLAN.md — Wave 2 — ADR 0001 ritual schema v1 + rituals/schema.rs locked types (depends on 10-01 placeholders) (FND-01)
- [x] 10-04-PLAN.md — Wave 1 — ADR 0002 monthly anchor + ADR 0003 polarity matrix + almanac/fengshui/types.rs (FND-02, FND-04, FND-05)
- [x] 10-05-PLAN.md — Wave 3 — MILESTONES.md ADR Cross-References subsection (DEC-0023/0024/0025) (depends on all three ADRs)

### Phase 11: Văn khấn Module + Lookup APIs

**Goal**: User can call five public APIs from `crates/amlich-core/src/rituals/` to look up rituals by snapshot, event key, or life event — backed by an `OnceLock` corpus loader with NFC normalization and Hán-character guard.

**Depends on**: Phase 10 (ritual schema must be locked; `Holiday.id` must exist).

**Requirements**: RIT-01, RIT-02, RIT-03, RIT-04, RIT-05, RIT-06, RIT-07, RIT-08

**Success Criteria** (what must be TRUE):
  1. A caller can pass a `DaySnapshot` for the Tết Nguyên Đán date and `find_van_khan_for_snapshot(&snapshot)` returns the matching ritual entries (joined via `Holiday.id` + `RitualEntry.event_keys[]`).
  2. A caller can resolve `find_van_khan_for_event(&RitualEventKey)`, `find_van_khan_for_life_event(LifeEventKind)`, `get_ritual_by_id(&str)`, and `all_rituals()` against the loaded corpus.
  3. A code reader can find a closed `RitualEventKey` enum covering Sóc/Vọng, the 8 major lunar festivals, Tiết Khí anchors, life events, and `Always` — with the matcher's exhaustiveness enforced by the compiler.
  4. A caller can rely on `LunarDateMatch` having `MonthDay { month, day, leap_month_policy }`, `SolarTerm`, and `GregorianFixed` variants, with leap-month policy defaulting to `CanonicalMonthOnly`.
  5. CI rejects any ritual JSON whose body contains Hán characters above the configured threshold; loaded text is NFC-normalized and verifiable via a round-trip byte-equal test.

**Plans**: 4 plans across 4 waves
- [ ] 11-01-PLAN.md — Wave 1 — unicode-normalization dep + data/rituals/fixtures.json (5–8 stub entries) + tests/ritual_han_guard.rs CI guard (RIT-08)
- [ ] 11-02-PLAN.md — Wave 2 — rituals/corpus.rs OnceLock loader + NFC normalize-at-load + source_id discipline; registers `mod corpus;` in rituals/mod.rs (RIT-05, RIT-08)
- [ ] 11-03-PLAN.md — Wave 3 — rituals/matcher.rs four lookup APIs (find_van_khan_for_snapshot/event/life_event, get_ritual_by_id) + derive_event_keys + leap-aware event_key_matches; rituals/mod.rs re-exports (RIT-01, RIT-02, RIT-03, RIT-04, RIT-06, RIT-07)
- [ ] 11-04-PLAN.md — Wave 4 — tests/rituals_integration.rs 6 black-box tests (Tết snapshot, Vọng path, Thanh Minh via SolarTerm, HolidayId cross-ref, NFC byte-equal round-trip, leap-policy semantics) (RIT-01, RIT-07, RIT-08)

### Phase 12: Văn khấn Corpus Authoring

**Goal**: User can find at least 60 traceable, peer-reviewed ritual entries shipped under `data/rituals/` with full citation discipline and variant coverage for at least 4 events.

**Depends on**: Phases 10 (schema lock) and 11 (module + loader). Can parallelize with Phase 13.

**Requirements**: RIT-09, RIT-10, RIT-11, RIT-12, RIT-13

**Success Criteria** (what must be TRUE):
  1. A reader can find ≥ 60 entries under `data/rituals/` spread across ≤ 14 per-event-category files plus `manifest.json`.
  2. A reader can open any entry and find `source_id: "vn-folk-ritual"`, an `original_citation` (book + page), and a `confidence` tier of `primary` / `regional-variant` / `synthesized`.
  3. A reviewer can find a `provenance_audit.md` ledger in `data/rituals/` recording the classical reference and independent reviewer for every entry.
  4. A caller can iterate `all_rituals()` and find ≥ 4 events with multiple variants sharing the same `event_type` and discriminated by a `variant` field on `RitualEntry` (e.g., Tết: simple / full / Buddhist / folk).
  5. A code reader can find a reserved `body_en: Option<String>` field on `RitualEntry`, deserialized via `#[serde(default)]`, content authoring deferred.

**Plans**: 4 plans

Plans:
- [ ] 12-01-PLAN.md — Author spring/summer festival corpus batch A (Tết 4 variants, Nguyên Tiêu, Hàn Thực, Thanh Minh, Đoan Ngọ 3 variants, Phật Đản; ≥ 26 entries)
- [ ] 12-02-PLAN.md — Author autumn/winter + life-event + daily corpus batch B (Vu Lan 3v, Trung Thu, Trùng Cửu/Hạ Nguyên, Ông Táo 2v/Giao Thừa, life-events incl. Nhập trạch 2v, Sóc/Vọng, daily gia-tiên Always; ≥ 34 entries)
- [ ] 12-03-PLAN.md — Generalize corpus loader to multi-file include_str! + manifest.json + RIT-09/10/12/13 invariant tests
- [ ] 12-04-PLAN.md — Write provenance_audit.md ledger (RIT-11) — classical reference + reviewer per entry

### Phase 13: Phi Tinh Primitives + Period + Annual/Monthly

**Goal**: User can call `compute_period`, `compute_yearly_flying_stars`, `compute_monthly_flying_stars`, and `compute_combined_overlay` from `almanac/fengshui/`, with Vận 7-9 covered and per-sub-star evidence envelopes attached.

**Depends on**: Phase 10 (Flying Star schema + monthly anchor + Niên direction ADRs). Parallelizable with Phases 11-12 (zero shared code paths).

**Requirements**: FS-01, FS-02, FS-03, FS-04, FS-05, FS-06, FS-07, FS-08, FS-09, FS-10

**Success Criteria** (what must be TRUE):
  1. A caller can invoke `compute_period(2024, &term_scanner)` and receives Vận 8 for any instant before Lập Xuân 2024-02-04 16:27 ICT and Vận 9 thereafter (boundary scanned via the v1.1.2 Tiết Khí scanner, never via naïve `year >= 2024`).
  2. A caller can find Vận 7 (1984-2003), Vận 8 (2004-2023), and Vận 9 (2024-2043) all populated and golden-tested at boundary instants; every base palace passes Lo Shu invariant checks (sum=45, each 1-9 once, center=Vận).
  3. A caller can invoke `compute_yearly_flying_stars` and `compute_monthly_flying_stars` against ≥ 10 dates per Vận with multi-source verification (≥ 2 sources per case; *Thẩm Thị Huyền Không Học* as tiebreaker; divergences logged as `KnownDivergence`).
  4. A caller can invoke `compute_combined_overlay(year, month, &term_scanner)` and receives `[(annual_star, monthly_star); 9]` per palace.
  5. A reader inspecting any aggregate result finds separate `ReasoningEvidenceEnvelope` entries for Vận, Niên, and Nguyệt, plus a composite `rule.composite.flying_stars` envelope on the aggregate output.

**Plans**: 4 plans
- [ ] 13-01-PLAN.md — Primitives: TietKhiScanner wrapper, FlyingStar metadata loader, Vận base tables + Lo Shu validator, compute_period (FS-01..05)
- [ ] 13-02-PLAN.md — Annual (Niên polarity matrix) + Monthly (solar-term + 8/5/2 group rule) with per-layer evidence (FS-06, FS-07, FS-09)
- [ ] 13-03-PLAN.md — Combined overlay + composite rule.composite.flying_stars evidence (FS-08, FS-09)
- [ ] 13-04-PLAN.md — Golden dataset + KnownDivergence + black-box FS-04/05/10 invariant tests (FS-05, FS-10)

### Phase 14: Phi Tinh 81-cell Aspects + Safety Hints

**Goal**: User can look up the digitized 2-star aspect for any of the 81 ordered star pairs and receive citation-bearing advisory hints (danger predicate + Ngũ-Hành element hint) — never product names.

**Depends on**: Phase 13 (combined overlay must exist so `compute_palace_aspects` can derive per-palace pairs).

**Requirements**: FS-11, FS-12, FS-13, FS-14, FS-15

**Success Criteria** (what must be TRUE):
  1. A caller can invoke `lookup_star_pair_aspect(star_a, star_b)` for any of the 81 ordered pairs and receive a `StarPairAspect { name, ngu_hanh_relation, auspice, original_citation }`.
  2. A reader can open any `StarPairAspect` and find `source_id: "huyen-khong"` plus an `original_citation` pointing to a specific chapter of *Thẩm Thị Huyền Không Học*, with a `confidence` tier.
  3. A caller can invoke `compute_palace_aspects(year, month, &term_scanner)` and receive `[StarPairAspect; 9]` derived from the combined overlay.
  4. A caller can call `is_danger_palace(star)` on `FlyingStar` and receive `true` exactly for Ngũ Hoàng (5) and Nhị Hắc (2) per classical tradition.
  5. A caller can call `element_hint_for_palace(star)` and receive an `Option<RemedyHint>` referencing Ngũ-Hành (kim/mộc/thủy/hỏa/thổ) with a classical citation — and the test suite verifies no product names appear anywhere in the hint corpus.

**Plans**: 3 plans
- [ ] 14-01-PLAN.md — aspects.rs: 81-cell star-pair corpus + lookup_star_pair_aspect + compute_palace_aspects (FS-11/12/13)
- [ ] 14-02-PLAN.md — safety.rs: is_danger_palace + element_hint_for_palace + safety hints corpus (FS-14/15)
- [ ] 14-03-PLAN.md — black-box fengshui_aspects integration tests + no-product-names guard (FS-11..15)

### Phase 15: Semantic Graph Wiring + DTO Integration + E2E Validation

**Goal**: User of `DaySnapshot` can observe ritual + flying-star surfaces additively, and a 2026 smoke test confirms the milestone holds end-to-end across Tết, Sóc/Vọng, Vận transitions, leap months, and all 24 Tiết Khí boundaries.

**Depends on**: Phases 11, 12, 13, 14 (all pillar code must exist before graph/DTO wiring).

**Requirements**: INT-01, INT-02, INT-03, INT-04, INT-05, INT-06

**Success Criteria** (what must be TRUE):
  1. A caller can deserialize a v1.5 `DaySnapshot` and find new optional `flying_stars: Option<FlyingStarsSummary>` and ritual-surfacing fields (both `#[serde(default, skip_serializing_if = "Option::is_none")]`) — additive only.
  2. A semantic-graph reader can find new `NodeConcept::Ritual` and `NodeConcept::FlyingStar` variants and new `EdgeConcept::PrescribedFor`, `EdgeConcept::OccupiesPalace`, `EdgeConcept::CarriesElement` edges with exhaustive matches enforced by the compiler.
  3. A reader can inspect a `FlyingStar` node and find it carries only `source_id: "huyen-khong"`; a ritual node only `source_id: "vn-folk-ritual"`; and a shared `Direction` node carrying both `khcbppt` and `huyen-khong` provenance entries (multi-source dedup verified). Phi Tinh remains absent from `interaction/direction_merge.rs`.
  4. A caller can load any v1.4 JSON fixture into v1.5 structs and re-serialize without unexpected fields (backward-compat round-trip).
  5. The end-to-end 2026 calendar smoke test passes on ≥ 30 representative dates covering Tết Nguyên Đán, Sóc/Vọng × 12, Vận 8 → 9 transition dates, leap-month dates, and all 24 Tiết Khí boundaries.

**Plans**: 4 plans across 2 waves
- [ ] 15-01-PLAN.md — Wave 1 — serde-derive sweep on DaySnapshot chain + FlyingStarsSummary DTO + additive flying_stars/applicable_rituals fields, populated by default (INT-01, INT-02)
- [ ] 15-02-PLAN.md — Wave 1 — ontology: NodeConcept::Ritual/FlyingStar + EdgeConcept::PrescribedFor/OccupiesPalace/CarriesElement across all six locations + GraphOntology completeness test (INT-03)
- [ ] 15-03-PLAN.md — Wave 2 — builder wiring: add_flying_star_facts + add_ritual_facts (disjoint source_ids) + dual-provenance Direction node; direction_merge.rs untouched (INT-04) (depends on 15-01, 15-02)
- [ ] 15-04-PLAN.md — Wave 2 — INT-05 v1.4 backward-compat round-trip + INT-06 2026 E2E smoke (≥30 dates: Tết, Sóc/Vọng×12, Vận boundary, leap month, 24 Tiết Khí) (depends on 15-01)

## Progress Table

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 10. Foundation — Schema Lock + ADRs + Source-ID Registration | 5/5 | Complete    | 2026-05-26 |
| 11. Văn khấn Module + Lookup APIs | 4/4 | Complete    | 2026-05-26 |
| 12. Văn khấn Corpus Authoring | 4/4 | Complete    | 2026-05-27 |
| 13. Phi Tinh Primitives + Period + Annual/Monthly | 4/4 | Complete    | 2026-05-27 |
| 14. Phi Tinh 81-cell Aspects + Safety Hints | 3/3 | Complete    | 2026-05-27 |
| 15. Semantic Graph Wiring + DTO Integration + E2E Validation | 4/4 | Complete   | 2026-05-27 |

## Requirement Coverage

| Phase | Requirements | Count |
|-------|--------------|-------|
| 10 | FND-01, FND-02, FND-03, FND-04, FND-05, FND-06 | 6 |
| 11 | RIT-01, RIT-02, RIT-03, RIT-04, RIT-05, RIT-06, RIT-07, RIT-08 | 8 |
| 12 | RIT-09, RIT-10, RIT-11, RIT-12, RIT-13 | 5 |
| 13 | FS-01, FS-02, FS-03, FS-04, FS-05, FS-06, FS-07, FS-08, FS-09, FS-10 | 10 |
| 14 | FS-11, FS-12, FS-13, FS-14, FS-15 | 5 |
| 15 | INT-01, INT-02, INT-03, INT-04, INT-05, INT-06 | 6 |
| **Total** | | **40 / 40** ✓ |

## Cross-Cutting Constraints

- **Additive-only DTOs** — all new fields are `Option<T>` with `#[serde(default, skip_serializing_if = "Option::is_none")]` (v1.2 precedent).
- **Source-id discipline** — every new module declares `pub const SOURCE_*` for its tradition; provenance call-sites never use string literals.
- **No `direction_merge.rs` wiring** — Phi Tinh node kind stays disjoint from `sat_phuong` / `than_huong` / `thai_tue`; spatial composition is Tier-3 / P5 work, out of scope for v1.5.
- **Tiết Khí scanner reuse** — Vận and monthly anchors reuse the v1.1.2 real-Tiết-Khí boundary scanner; no naïve year arithmetic.

---
*Last updated: 2026-05-26 after Phase 11 planning (4 plans across 4 waves; RIT-01..08 distributed)*
