# Project Research Summary

**Project:** amlich v1.5 — Eastern Knowledge Expansion (P1 Văn khấn + P4 Phi Tinh thời gian)
**Domain:** Vietnamese Almanac — ritual content corpus + time-based Huyền Không Flying Stars
**Researched:** 2026-05-23
**Confidence:** HIGH (system integration); MEDIUM (a few domain conventions need ADR-locking during execution)

## Executive Summary

v1.5 adds two **Tier 0** pillars to the existing deterministic `amlich-core` Rust workspace shipped through v1.0–v1.4: **P1 Văn khấn cổ truyền** (`source_id: vn-folk-ritual`) is a content corpus + lookup pillar that surfaces Vietnamese ritual prayers, offerings, and procedure by event/date; **P4 Phi Tinh thời gian** (`source_id: huyen-khong`) is a finite-table pillar that emits 9-palace Lo Shu star layouts for the queried Vận/Năm/Tháng with **no spatial input**. Neither pillar requires algorithmic novelty and neither requires new crate dependencies — the existing `serde` / `serde_json` / `chrono` trio plus the `include_str!` + `OnceLock` pattern from `almanac/golden_loader.rs` covers both pillars end-to-end. P1 and P4 share no code paths and can ship in parallel within the milestone; they reconverge only at semantic-graph wiring and integration tests.

The recommended approach mirrors v1.2's proven additive-only playbook: new top-level module `crates/amlich-core/src/rituals/` for P1, new sub-folder `crates/amlich-core/src/almanac/fengshui/` for P4 (folder, not file, to leave room for the Tier-3 `spatial_compose` work explicitly deferred to P5). All new fields on shared DTOs (`DaySnapshot`, `DayFortune`) must be `Option<T>` per v1.2 precedent. The single load-bearing **boundary rule** is that Phi Tinh outputs under `huyen-khong` must NEVER merge with the existing KHCBPPT direction modules (`sat_phuong.rs`, `than_huong.rs`, `thai_tue.rs`) — they answer different questions (palace layout vs. single auspicious/inauspicious direction) and carry disjoint source provenance per DEC-0015/0016. Three risks drive sequencing: (a) **schema-lock for the ritual JSON must precede corpus authoring** because corpus production (~60 entries) is the longest-pole work and re-editing entries after a schema slip is prohibitively expensive; (b) **Vận 8 → Vận 9 transition aligns with Lập Xuân 2024-02-04 16:27 ICT, not calendar 2024-01-01** — naïve `year >= 2024` checks corrupt every January/early-February output and must reuse the v1.1.2 real-Tiết-Khi boundary scanner; (c) **Phi Tinh base palace tables are validated by Lo Shu invariants (sum=45, each 1-9 once, center=Vận)** plus a golden dataset mirroring the v1.0 KHCBPPT methodology — there is no canonical software to cross-check against, so the validation strategy is multi-source-with-tiebreaker (classical *Thẩm Thị Huyền Không Học*) per EXPANSION_FRAMEWORK §7.

## Key Findings

### Recommended Stack

**No new crate dependencies required.** Existing workspace pins suffice. Stack delta for v1.5 is the *embedding pattern*, not the dependency list.

**Core technologies (already in `crates/amlich-core/Cargo.toml`):**
- **`serde` 1.0 (derive)** — Derive `Serialize`/`Deserialize` for `RitualEntry`, `RitualCorpus`, `FlyingStarLayout`, `Palace`, `FlyingStar`. Matches existing `GoldenDataset` / `GoldenEntry` patterns.
- **`serde_json` 1.0** — Parse `data/rituals/*.json` and `data/almanac/flying_stars.json` at first-call. Exact pattern used by `golden_loader::load_golden_dataset`.
- **`chrono` 0.4** — `NaiveDate` inputs for ritual lookups and Vận/Năm/Tháng resolution. Deterministic — `Utc::now()` remains forbidden by project policy in v1.5.

**Standard library (load-bearing for both pillars):**
- **`include_str!`** — Embed all corpus JSON into the compiled binary; matches `golden_loader.rs:5` and the WASM target's zero-runtime-IO constraint.
- **`std::sync::OnceLock`** — One-time parse + validate cache; matches `golden_loader.rs:6`; stable since Rust 1.70.
- **`BTreeMap` / `HashMap`** — Index rituals by event key; index Phi Tinh tables by `(vận, năm)` and `(vận, năm, tháng)`. `BTreeMap` preferred where iteration order touches golden tests.

**Why no new dependencies:** Hand-rolled validators (mirror `golden_loader::validate_*`) catch richer invariants than `jsonschema` documents; văn khấn text is plain UTF-8 prayer scripts (no Markdown rendering needed in core — UI's job); `OnceLock` removes the `once_cell`/`lazy_static` case; `phf` would force a `build.rs` we have deliberately avoided; Flying Stars math is integer-pure (no float libs); async runtimes, web frameworks, runtime file IO, and Markdown engines are all explicit non-goals for the v1.5 stack.

### Expected Features

**Must have (table stakes — v1.5 MVP gate):**

*P1 Văn khấn — corpus + lookup:*
- `Ritual` (a.k.a. `RitualEntry`) struct + JSON schema with `ritual_id`, `event_keys[]`, `category`, `offerings`, `preparation_steps`, `invocation_text_vi`, `source_id: "vn-folk-ritual"`, `original_citation`, `confidence`.
- Closed `event_type` / `RitualEventKey` enum covering at minimum: Sóc/Vọng (Mùng 1, Rằm), 8 major lunar festivals (Tết Nguyên Đán, Khai Hạ, Thượng Nguyên, Thanh Minh, Đoan Ngọ, Vu Lan, Trung Thu, Ông Công Ông Táo), life events (Động thổ, Nhập trạch, Khai trương, Cưới, Giỗ, Đầy tháng).
- Corpus content: ≥ 20 entries minimum, target ~60 for first release.
- Lookup APIs: `find_van_khan_for_snapshot(&DaySnapshot)`, `find_van_khan_for_event(&RitualEventKey)`, `find_van_khan_for_life_event(LifeEventKind)`, `get_ritual_by_id(&str)`, `all_rituals()`.
- Per-record source citation validated by golden test; loader rejects entries missing `source_id` or `original_citation`.
- Additive integration: holiday JSON entries optionally gain `ritual_ids: ["<id>", ...]`; `Holiday` struct gains `id: Option<String>` so the matcher can join by stable id (one tiny existing-file modification — the only one in v1.5).

*P4 Phi Tinh — time-based 9-palace layout:*
- `Period` determination from year (Vận 8: 2004–2023, Vận 9: 2024–2043) — boundary at **Lập Xuân**, not Jan 1.
- 9-star metadata table (name Nhất Bạch…Cửu Tử, element, polarity, auspice, palace_color).
- Annual center star + full 9-palace grid via formula (`center = ((11 - digit_sum(year)) mod 9)`, 0→9), golden-tested for 2020–2030.
- Monthly center star + full 9-palace grid via year-branch-group rule (groups start at 8/5/2, descend mod-9), golden-tested for ≥ 24 month-points.
- Static `Palace ↔ Direction` mapping (Lạc Thư canonical: N=1, NE=8, E=3, SE=4, S=9, SW=2, W=7, NW=6, Center=5).
- `DaySnapshot.flying_stars: Option<FlyingStarsSummary>` additive field (year + month layers; combined overlay deferred).
- Year boundary = Lập Xuân (~Feb 4); month boundary = solar terms (tiết) — **both decisions ADR'd** during execution.

**Should have (differentiators, schedule if scope permits):**
- Combined annual+monthly overlay grid (`year_star + month_star` per palace).
- 81-cell 2-star combination aspect table (e.g., 1-6 → Văn Xương) from *Thẩm Thị Huyền Không Học* — DEFER if digitization effort underestimated.
- Star avoidance flags (`is_danger_palace`) + element-hint cures (Ngũ Hoàng → metal) — never product names.
- Cross-link of Phi Tinh to existing Thái Tuế / Tam Sát directional warnings (read-only join in reasoning layer; no boundary merge).
- Ritual variants per event (simple/full/Buddhist/folk) via shared `event_type` + `variant` field.
- Bilingual `body_en` — schema reserves field; content authoring deferred.

**Defer (v1.6+ / future milestones):**
- Spatial Phi Tinh (Tier 3, Sơn-Hướng, `Direction24` input) — explicit P5 deliverable, OUT of scope this milestone.
- Daily / Hourly Phi Tinh (Lưu Nhật, Lưu Thời) — boundary semantics need ADR; corpus reliability lower.
- AI-generated / auto-personalized prayer text — violates source provenance.
- Audio prayer recordings, full-text search across `khan_text`, per-user prayer history, user-editable corpus — all UI/app concerns, not engine.
- "Cure" product recommendations, Vận-transition alerts — out of scope (commercial/stateful).

### Architecture Approach

v1.5 introduces two computation pathways into the existing additive-only architecture (same playbook as v1.2 Ten Gods + Kua) with one critical refinement: this is the first milestone where **two distinct source_id traditions** (`vn-folk-ritual` and `huyen-khong`) coexist as new code alongside the entrenched `khcbppt` family. The architecture follows five patterns: (1) **Additive top-level module for P1** (`rituals/`) and **additive sub-folder for P4** (`almanac/fengshui/`) — folder for P4 because future Tier-3 work (`spatial_compose`) will join it; (2) **Pure lookup / pure computation** — neither pillar mutates state, both load-once via `OnceLock`; (3) **Hybrid data form for Phi Tinh** — Vận base palace tables as `const` Rust arrays (mathematically determined Lo Shu permutations), star metadata as JSON (human-edited, citation-bearing); (4) **One-way dependency** — `rituals` reads `holidays`, never the reverse; `holidays.rs` frozen apart from one new `id: Option<String>` field; (5) **Distinct provenance per pillar** — `ProvenanceEntry::almanac_rule("vn-folk-ritual", ...)` and `ProvenanceEntry::almanac_rule("huyen-khong", ...)` with module-level `pub const SOURCE_* : &str` to prevent typo-minted fake sources.

**Major components:**
1. **`crates/amlich-core/src/rituals/{mod,corpus,event_match,types,tests}.rs` (NEW)** — Public ritual API; OnceLock-backed corpus loader; `DaySnapshot` → `Vec<RitualEventKey>` resolver joining `holidays.rs`; types (`RitualEntry`, `RitualEventKey`, `LifeEventKind`, `LunarDateMatch`, `RitualConfidence`); golden coverage tests.
2. **`crates/amlich-core/src/almanac/fengshui/{mod,lo_shu,flying_stars,star_meta}.rs` (NEW)** — Sub-folder root with **boundary docstring** (verbatim in `mod.rs`) explicitly disjoining Phi Tinh from `sat_phuong` / `than_huong` / `thai_tue`; `Palace` enum + Lo Shu canonical ordering + direction mapping; Vận/Năm/Tháng layouts; star-metadata loader.
3. **`crates/amlich-core/data/rituals/*.json` (NEW)** — `manifest.json` + ~14 per-event-category files (`tet_nguyen_dan.json`, `soc_vong_monthly.json`, `life_events_dong_tho.json`, …). One-file-per-event-category trades giant-file merge-conflicts for review-sized diffs.
4. **`crates/amlich-core/data/almanac/flying_stars.json` (NEW)** — Star metadata (name, element, polarity, default interpretation); bilingual; citation-bearing. NOT the Vận tables themselves — those stay as `const` Rust arrays validated by Lo Shu invariants at load.
5. **`semantic_graph/ontology.rs` (MODIFIED, additive)** — Add `NodeConcept::Ritual`, `NodeConcept::FlyingStar`; add `EdgeConcept::PrescribedFor`, `EdgeConcept::OccupiesPalace`, `EdgeConcept::CarriesElement`; matching `label()` arms + slice entries; exhaustive matches enforced by compiler.
6. **`holidays.rs` (MODIFIED, one additive field)** — `Holiday { ..., id: Option<String> }` with `#[serde(default)]`; populated from existing `lunar_festivals[].id` so the ritual matcher can join by stable id rather than fragile display name. Default `None` for generated Mùng 1/Rằm entries.
7. **Day snapshot builder (MODIFIED, additive)** — Materializes ritual + flying-star nodes; first time non-`khcbppt` nodes co-exist in the day graph; provenance separation tests required.

**Key architectural insight:** This milestone is the codebase's first stress-test of the source_id discipline established in DEC-0015/0016. The two new pillars exist precisely *because* they have different traditions than KHCBPPT, and they must demonstrate clean coexistence — separate `source_id`s, separate node kinds (`FlyingStar` is a palace-layout descriptor, NOT a bare direction string), and separate evidence envelopes. The boundary docstring in `almanac/fengshui/mod.rs` is the operational definition of "no overlap, no duplication" that future contributors will use as precedent.

### Critical Pitfalls

**1. Source-ID cross-contamination between `vn-folk-ritual` / `vn-folk` / `khcbppt`** — Văn khấn entry copied from a KHCBPPT-derived calendar or Chinese ceremonial corpus gets uniformly tagged `vn-folk-ritual`, leaking foreign provenance. Avoid: Rust enum `RitualSourceId` (not free string); required `original_citation` field; per-entry `provenance_audit.md` ledger; CI grep guard for traditional/simplified Chinese characters. (Phase 1)

**2. Phi Tinh Vận 8 → Vận 9 boundary off-by-one** — Naïve `year >= 2024 → Vận 9` is wrong; boundary is Lập Xuân 2024-02-04 16:27 ICT. Avoid: reuse v1.1.2 real-Tiết-Khi boundary scanner; golden cases at 2024-01-31 / 2024-02-04 06:00 / 2024-02-04 16:27 / 2024-02-05. (Phase 4)

**3. Phi Tinh / KHCBPPT directional output conflation** — `sat_phuong.rs`, `than_huong.rs`, `thai_tue.rs` emit `direction: String`; Phi Tinh emits palace layouts. Avoid: `FlyingStar { palace, star_number, polarity, period }` node kind, NEVER bare direction string; `pub const SOURCE_HUYEN_KHONG`; contract test asserts node IDs disjoint; **do NOT** wire Phi Tinh into `direction_merge.rs` this milestone (Tier-3 work for P5). (Phase 4)

**4. Phi Tinh base palace table typos catastrophic and silent** — Single transposition in a 9-cell Vận grid corrupts every derived star. Avoid: Lo Shu invariants enforced at load (sum=45, each 1-9 once, center=Vận); Phi Tinh golden dataset mirroring v1.0 KHCBPPT methodology (≥ 10 cross-checked dates per Vận); divergences logged as `KnownDivergence`. (Phase 4)

**5. Lễ vật / trình tự stored as freeform strings** — Blocks vegetarian filtering, checklist UI, semantic-graph extraction. Avoid: schema-first — `offerings: Vec<Offering { category, item, optional }>`, `preparation_steps: Vec<ProcedureStep>`; `#[serde(deny_unknown_fields)]`. (Phase 1)

**6. Lunar/solar date matching ambiguity** — `lunar_date: "23/12"` misses tháng-12-nhuận years. Avoid: structured `LunarDateMatch { MonthDay { month, day, leap_month_policy }, SolarTerm(...), GregorianFixed { month, day } }`; default `LeapPolicy::CanonicalMonthOnly`. (Phase 1)

**7. Monthly Phi Tinh anchor convention** — Lập Xuân vs lunar Giêng vs civil January — wrong anchor offsets every monthly star by 1. Avoid: explicit DEC picking solar-term boundaries per *Thẩm Thị Huyền Không Học*; reuse v1.1.2 Tiết Khí scanner. (Phase 4; ADR in Phase 1)

**8. Niên Tử Bạch direction (Thuận/Nghịch) inverted by Yuan** — Depends on Tam Nguyên + year polarity; 2024 (Hạ Nguyên) hides bug because current outputs look right. Avoid: encode as table keyed by (Yuan × polarity); golden dataset spans multiple Vận. (Phase 4; ADR in Phase 1)

**9. Vietnamese diacritic encoding drift in văn khấn JSON** — NFC vs NFD, pre-1975 South Vietnamese orthography. Avoid: NFC-normalize-on-load; CI lint via `unicode-normalization`; pick one tone-position convention. (Phase 1/2)

**10. Evidence metadata holes on Phi Tinh aggregate outputs** — Sub-stars (Vận / Niên / Nguyệt) not separately attributed. Avoid: per-sub-star `Provenance::almanac_rule("huyen-khong", "vận")`, `..."niên"`, `..."nguyệt"`; aggregate carries separate `rule.composite.flying_stars` envelope. (Phase 4)

**11. Backward-compat break: new DTO fields not `Option<T>`** — Breaks desktop workspaces (Personal Lab, Season Timeline, Almanac Inspector). Avoid: every new field is `Option<T>` with `#[serde(default, skip_serializing_if = "Option::is_none")]`; round-trip contract test loads `tests/fixtures/v1.4-*.json` into v1.5 struct. (Phase 5)

## Implications for Roadmap

**6 phases recommended.** Phase 1 is a hard gate (schema-lock for both pillars + 2 ADRs). Phases 2-3 (P1) and Phase 4 (P4) parallelize. Phase 5 unifies the ontology. Phase 6 is the integration + validation gate.

### Phase 1: Schema Lock + Source-ID Registration (Foundation — gates everything)

**Delivers:** ADR ritual JSON schema v1; ADR monthly Phi Tinh anchor convention; ADR Niên Tử Bạch direction rule per Yuan; source-taxonomy memory doc updated with `vn-folk-ritual` and `huyen-khong`; `Holiday.id: Option<String>` field; compile-time source-id constants.

**Addresses:** Schema-lock gating; CRIT-1, CRIT-3, MOD-2, MOD-3, MOD-6.

**Research flags:** LOW.

### Phase 2: P1 Văn khấn Module + Lookup APIs (parallelizable with Phase 4)

**Delivers:** `crates/amlich-core/src/rituals/` full public API; OnceLock-backed loader; NFC normalization; `RitualEventKey` enum; loader lints.

**Addresses:** P1 table-stakes; CRIT-5, MOD-1, MOD-4, MIN-1, MIN-3, MIN-5.

**Research flags:** LOW.

### Phase 3: P1 Corpus Authoring (longest-pole content work)

**Delivers:** `data/rituals/manifest.json` + ~14 per-event-category JSON files; ≥ 20 (target ~60) entries; per-entry `source_id` + `original_citation` + `confidence`; `provenance_audit.md` ledger.

**Addresses:** P1 corpus content; CRIT-1, MOD-4.

**Research flags:** MEDIUM — editorial domain-expert work.

### Phase 4: P4 Phi Tinh Primitives + Algorithm (parallelizable with Phases 2-3)

**Delivers:** `crates/amlich-core/src/almanac/fengshui/` with boundary docstring; `Palace` + `FlyingStar` enums; `FlyingStarLayout`; Vận 1-9 base palace `const` tables validated by Lo Shu invariants; `data/almanac/flying_stars.json` star metadata; Vận 8 → 9 boundary via v1.1.2 Tiết Khí scanner; per-sub-star evidence envelopes; Phi Tinh golden dataset with ≥ 10 dates per Vận and `KnownDivergence` log.

**Addresses:** P4 table-stakes; CRIT-2, CRIT-3, CRIT-4, MOD-2, MOD-3, MOD-5, MIN-2, MIN-4.

**Research flags:** MEDIUM-HIGH — Phi Tinh has no canonical software cross-check.

### Phase 5: Semantic Graph Wiring (Both Pillars)

**Delivers:** `NodeConcept::Ritual`, `NodeConcept::FlyingStar`; `EdgeConcept::PrescribedFor`, `EdgeConcept::OccupiesPalace`, `EdgeConcept::CarriesElement`; builder additions; provenance verification tests (multi-source `Direction` nodes).

**Addresses:** Semantic-graph extension; MOD-5, CRIT-3.

**Research flags:** LOW.

### Phase 6: DTO Integration + End-to-End Validation

**Delivers:** `DaySnapshot.flying_stars: Option<FlyingStarsSummary>` additive field; optional ritual surfacing in `DaySnapshot`; round-trip test with v1.4 fixtures; 2026 smoke test on ≥ 30 representative dates.

**Addresses:** MOD-6; P1 + P4 cross-pillar integration.

**Research flags:** LOW.

### Phase Ordering Rationale

- **Phase 1 first** — schema lock + ADRs gate everything.
- **Phases 2-3 (P1) and Phase 4 (P4) parallelize** — zero shared code paths.
- **Phase 5 unifies the ontology** — exhaustive-match boundaries best landed together.
- **Phase 6 last** — backward-compat verification meaningful only once both pillars wired.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | No new deps; pattern directly confirmed in `golden_loader.rs:5-21`. |
| Features | HIGH (P1) / MEDIUM-HIGH (P4) | P1 taxonomy well-documented; P4 formulas cross-verified; monthly anchor + Niên direction need ADR-lock. |
| Architecture | HIGH | All file:line refs verified; boundary discipline encoded as docstring + module-level `const`. |
| Pitfalls | HIGH | 11 pitfalls anchored in concrete code refs and v1.0/v1.1.2/v1.2 lessons. |

**Overall confidence:** HIGH

### Gaps to Address

- Phi Tinh validation has no canonical reference (mitigated by multi-source golden + classical tiebreaker)
- Monthly anchor convention school-dependent (mitigated by Phase 1 ADR)
- Niên direction across Tam Nguyên needs polarity matrix (mitigated by Phase 1 ADR + table encoding)
- Văn khấn single-author-risk (mitigated by per-entry citation + audit ledger)
- Daily/Hourly Phi Tinh deferral needs explicit DEC
- Builder file size budget — orchestrator decides

## Sources

### Primary (HIGH confidence, in-repo)
- `.planning/research/STACK.md`
- `.planning/research/FEATURES.md`
- `.planning/research/ARCHITECTURE.md`
- `.planning/research/PITFALLS.md`
- `.planning/research/EXPANSION_FRAMEWORK.md` (pillar source-of-truth)
- `.planning/PROJECT.md` (DEC-0015/0016/0022, additive-only policy from v1.2)
- `crates/amlich-core/src/almanac/golden_loader.rs:5-21, 153-237` (embedding + validation pattern)
- `crates/amlich-core/src/almanac/sat_phuong.rs:38-43`, `than_huong.rs:20-32`, `thai_tue.rs:53-112` (boundary disjoint)
- `crates/amlich-core/src/semantic_graph/provenance.rs:65-67, 130-135`
- `crates/amlich-core/src/semantic_graph/ontology.rs:5-40, 85-111`
- `crates/amlich-core/src/holidays.rs:14-25, 148-198, 228-260`
- `crates/amlich-core/Cargo.toml:13, 16-19`

### Secondary (MEDIUM confidence, external)
- Vietnamese ritual corpus references: chuabavang.com, sachhayonline.com, luatminhkhue.vn, tuhuyen.com, Lịch Vạn Niên 2026, Lịch Ngày Tốt.
- Phi Tinh algorithm references: phongthuydathanh.com, lichngaytot.com, lykhi.com, phongthuycaivan.org, phongthuyvietnam.com, fengshuidiy.com, uniquefengshui.com.
- Vận 8→9 boundary: phongthuykhaitoan.com (2024-02-04 16:27 ICT).
- *Thẩm Thị Huyền Không Học* — classical text, designated tiebreaker per EXPANSION_FRAMEWORK §7.

### Tertiary (LOW confidence, validation hooks)
- 81-cell 2-star combination corpus from *Thẩm Thị Huyền Không Học* — scheduled as P2 differentiator, deferrable.
- Pre-1975 South Vietnamese orthography in older văn khấn — mitigated via NFC + tone-position convention.

---
*Research completed: 2026-05-23*
*Ready for roadmap: yes*
