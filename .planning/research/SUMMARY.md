# Project Research Summary

**Project:** amlich-core v1.7 — Kinh Dịch (I-Ching) pillar + Thái Tuế/Tam Sát ⇄ Phi Tinh cross-link
**Domain:** Vietnamese almanac engine (Rust library, multi-tradition reasoning system)
**Researched:** 2026-07-16
**Confidence:** HIGH overall — every recommendation is anchored in v1.5/v1.6 code paths with `file:line` references; no new dependencies, no new architectural layer, no new tier.

---

## Executive Summary

v1.7 is a **content-and-algorithm extension**, not an architectural one. It adds two new leaves into the existing layer cake — a Kinh Dịch divination pillar (`reasoning/iching/`) and a Thái Tuế ⇄ Phi Tinh directional cross-link (`reasoning/direction_composite.rs`) — and threads both through additive, schema-locked surfaces (`DaySnapshot`, semantic graph, reasoning envelope). The Mai Hoa Dịch Số casting algorithm is fully deterministic (≤30 lines of pure integer arithmetic on `chrono::Datelike` inputs); the 64-hexagram corpus is a JSON file embedded at compile time via `include_str!` and lazily parsed through `std::sync::OnceLock`, mirroring the v1.5 Văn khấn pattern verbatim. **No new crate dependencies are required** — the trio of `serde 1.0` + `serde_json 1.0` + `chrono 0.4` plus the already-direct `unicode-normalization 0.1.25` covers every new surface. Every upstream Rust I-Ching library considered (`xalen-iching`, `i-ching`, `iching`) is **rejected** because none carry Vietnamese text from Ngô Tất Tố / Thiệu Khang Tiết, which would violate the project's source-provenance discipline (DEC-0015/0016, ADR-0001, `tests/source_id_guard.rs`).

The recommended approach is **schema-lock-first** (the v1.5 CRIT-1/CRIT-5 lesson, amplified for v1.7 because 64 hexagrams × ~7 text fields = 448 corpus fields — re-editing after a slip is far more expensive than v1.5's 60 rituals). The Mai Hoa casting, biến quẻ derivation, Thể/Dụng classification, and the read-only directional cross-link are all pure functions over already-shipped producers (`lunar.rs`, `compute_thai_tue`, `get_sat_phuong`, the v1.5 FlyingStar palace layout). The cross-link preserves the project's most-explicitly-enforced boundary (CRIT-3: `FlyingStar` never wired into `interaction/direction_merge.rs`) by living in `reasoning/` and emitting two distinct primitive `source_id`s plus one `rule.composite.*` envelope.

The **top three risks** are: (1) the Mai Hoa `% 8 == 0` remainder-zero convention (CRIT-2) silently corrupts ~1/8 of castings if implemented naïvely; (2) Tiên Thiên trigram numbers vs King Wen hexagram numbers (CRIT-3) share the surface form "1..N" but have completely different mappings — easy to ship a system that returns wrong-hexagram texts that look correct; (3) the cross-link can quietly collapse CRIT-3 isolation (CRIT-5) if the two `source_id`s are merged or if `FlyingStar` references leak back into `interaction/`. All three are gated by ADRs (0005, 0006, 0007) landing in Phase 1, typed Rust boundaries (e.g. `TienThienTrigram(u8)` ≠ `KingWenHexagram(u8)` with no `From` impl between them), and dedicated CI grep guards.

### Reconciliation notes (cross-research conflicts resolved)

- **`ConsultationIntent::IChing { question }` vs `#[derive(Copy)]`** — STACK lists two options (drop Copy vs sibling-newtype); ARCHITECTURE firm-recommends the sibling-newtype. **Synthesis: take ARCHITECTURE's path.** Introduce `IChingQuery` newtype + `IChingEvaluator` (impl `ActionEvaluator`), mirroring the v1.6 `DailyFlyingStarLayout` sibling to `FlyingStarLayout` precedent. `ConsultationIntent` stays the closed 9-variant `Copy` enum it is. This avoids the ~25–43 call-site churn and keeps the framework's `{ question }` shape inside the sibling struct. EXPANSION_FRAMEWORK §2.2's wording is treated as aspirational pseudo-code, not a contract.
- **Directional Thái Tuế + classical 3-direction Tam Sát** — FEATURES finds the existing `thai_tue.rs` is personal-conflict-only (no directional aspect) and `sat_phuong.rs` returns ONE direction per chi (not classical Tam Sát's three). **Synthesis: surface as a Phase-1 decision point (DEC required), not a blocker.** Recommend implementing directional Thái Tuế as a new `pub fn` on `thai_tue.rs` and classical 3-direction Tam Sát as a new `almanac/tam_sat.rs` module (option b per FEATURES FS-10), keeping `sat_phuong.rs`'s single-direction day-chi feature intact.
- **Two 1-line evidence backfills** — ARCHITECTURE identified `thai_tue.rs:107-111` and `sat_phuong.rs:49-53` currently carry `evidence: None` and MUST be populated with `SOURCE_KHCBPPT` for the cross-link to cite them. This is a 2-line prerequisite on the cross-link phase.
- **Schema-lock before corpus** — unanimous across all four research files. ADR-0005 (HexagramRecord) and ADR-0006 (Mai Hoa casting conventions) MUST land before the 64-entry corpus is authored.

---

## Key Findings

### Recommended Stack

**No new dependencies.** The "no new deps" precedent (established v1.5, re-affirmed v1.6) holds verbatim for v1.7. `cargo tree -p amlich-core --depth 1` after v1.7 ships still shows only `serde`, `serde_json`, `chrono`, `unicode-normalization`.

**Core technologies (unchanged):**
- **`serde 1.0` / `serde_json 1.0`** (workspace pins) — derive for `HexagramEntry`, `MaiHoaCast`, `IChingCastSummary`, and parse the embedded 64-hexagram corpus at first call.
- **`chrono 0.4`** (workspace pin) — `Datelike` trait feeds the Mai Hoa time-number algorithm; consultation instant supplied by caller via `DaySnapshot.context` (project policy forbids `Utc::now()`).
- **`unicode-normalization 0.1.25`** (direct dep) — NFC-normalize every Vietnamese text field in the 64-hexagram corpus at load (RIT-08 precedent).
- **`std::sync::OnceLock<T>` + `include_str!`** — compile-time corpus embedding + lazy parse-and-cache; proven by `rituals/corpus.rs:17,27-56,85` and WASM-safe.
- **`u8` bit math** — biến quẻ is a single `^` op on a 6-bit line pattern; no `bitvec` needed.

**Integration points (all codebase-verified):**
1. New module `crates/amlich-core/src/reasoning/iching/` (sibling to `rituals/`, nested under `reasoning/` per EXPANSION_FRAMEWORK §2.2 because IChing spans corpus + algorithm + consultation semantics).
2. New data dir `crates/amlich-core/data/iching/` with `hexagrams.json` (64 entries, `"$schema_version": "iching-v1"`).
3. Two new `pub const` source_ids in `sources.rs`: `SOURCE_KINH_DICH` (`"kinh-dich"`), `SOURCE_MAI_HOA_DICH_SO` (`"mai-hoa-dich-so"`) + two new rows in `tests/source_id_guard.rs:FORBIDDEN_LITERALS`.
4. Sibling `IChingQuery` newtype + `IChingEvaluator` (reconciliation: NOT a new `ConsultationIntent::IChing` variant — see Anti-Pattern 1).
5. New `ReasoningEvidenceSourceFamily::IChing` + `ActionId::IChing` enum variants (closed-enum additive extension; exhaustive `match` sites updated mechanically).
6. New `reasoning/direction_composite.rs` for the cross-link; lives under `reasoning/` (outside the CRIT-3 quarantine zone) and uses **only `&` references** to the almanac layer.

### Expected Features

**Table stakes (must-have for v1.7):**
- **Mai Hoa time-based casting** (FS-01) — deterministic, no RNG; pinned algorithm below.
- **Tiên Thiên Bát Quái numerical map** (FS-02) — Càn=1..Khôn=8; static `const` table.
- **64-hexagram lookup corpus** (FS-03) — `HexagramRecord { king_wen_index, vi_name, chinese_name, upper/lower_trigram, thoai_tu, hao_tu[6], tuong_truyen?, cat_hung }`; biggest single deliverable; schema-locked first.
- **Biến quẻ derivation** (FS-04) — flip động hào bit, re-lookup; pure function.
- **Thể / Dụng classification** (FS-05) — the trigram NOT containing the động hào is Thể; Ngũ Hành sinh khắc drives cát/hung.
- **`IChingQuery` + `IChingEvaluator`** (FS-06) — sibling newtype (NOT a `ConsultationIntent` variant; reconciliation above); emits `source_id: kinh-dich` + `mai-hoa-dich-so` envelopes.
- **`source_id` registration** (FS-07) — `kinh-dich` + `mai-hoa-dich-so` as `pub const`.
- **`Hexagram` semantic-graph node + `LocatedAt`/`Transforms` edges** (FS-08) — 6-slice ontology extension.
- **Thái Tuế directional derivation** (FS-09) — new `pub fn thai_tue_direction(year_chi_index) -> Direction8` on existing `thai_tue.rs` (current module is personal-conflict-only).
- **Tam Sát directional** (FS-10) — **DECISION REQUIRED**: option (a) reuse single-direction `sat_phuong.rs` OR option (b) new `almanac/tam_sat.rs` with classical 3-direction rule. **Recommend (b)** for KHCBPPT correctness parity.
- **Thái Tuế / Tam Sát ⇄ Phi Tinh read-only cross-link** (FS-11) — composite directional picture with **dual source_ids** (`khcbppt` + `huyen-khong`); CRIT-3 isolation preserved.
- **Golden tests** (FS-12) — ≥10 casting cases × ≥2 independent sources (`nhantu.net` + Thiệu Khang Tiết / second Mai Hoa site); divergences logged as `KnownDivergence`.

**Differentiators (should-have, defer most to v1.8):**
- DF-01 Tier-2 Bazi enrichment of hexagram reading (Tier-0 baseline first; mirrors v1.5 Phi Tinh T0/T2 split).
- DF-02 Full Ngũ Hành sinh khắc matrix for Thể/Dụng (LOW complexity, high UX — strong v1.8 candidate).
- DF-03 Hỗ Quái (nuclear hexagram) — defer to v1.9+ depth milestone.
- DF-04 24-sơn directional resolution — co-design with future Tier-3 `SpatialInput`.
- DF-05 Pre-cast intent capture (question text in evidence).

**Anti-features (explicit exclusions):**
- AF-01 Coin/yarrow RNG casting — different tradition, breaks determinism, would need third `source_id`.
- AF-02 LLM-generated free-form interpretation — Ngô Tất Tố corpus IS the interpretation; surface verbatim.
- AF-03 Spatial feng-shui composition (wire `FlyingStar` into `interaction/direction_merge.rs`) — CRIT-3 violation; deferred to Tier-3 v1.9+.
- AF-04 Personal Thái Tuế rewrite based on directional cross-link — cross-link is **read-only by design**.
- AF-05 Mixing hexagram corpus translators — breaks single-`source_id` discipline; gaps logged as `PendingExternalReview` (v1.6 RIT-14 pattern).
- AF-06 User-selectable casting variants (số vật / âm thanh / chữ viết) — out of scope; ship time-numerology only.

#### Mai Hoa Casting Algorithm (pinned for FS-01, locked via ADR-0006)

Inputs: `lunar_year_branch_index (0..12)`, `lunar_month (1..13)`, `lunar_day (1..30)`, `chi_hour_index (0..12)`. **Lunar (not solar)** is the Mai Hoa tradition; the project's `lunar.rs` already does correct Vietnamese lunar conversion.

```
upper_trigram_idx = ((year + month + day - 1) % 8) + 1       // 1..=8
lower_trigram_idx = ((year + month + day + hour - 1) % 8) + 1 // 1..=8
moving_line       = ((year + month + day + hour - 1) % 6) + 1 // 1..=6 (1=bottom)
```

The `((n-1) % k) + 1` form achieves the classical "dư 0 thì lấy số cuối" rule (CRIT-2 prevention) without an `if`. The Tiên Thiên trigram pair is then composed into a King Wen hexagram via a 64-entry composition table (CRIT-3 prevention — see Pitfalls). Then biến quẻ = flip the động hào bit and re-compose.

### Architecture Approach

v1.7 introduces **no new layer**. Two new module-leaves slot into the existing layer cake: `reasoning/iching/` (corpus + algorithm + evaluator) and `reasoning/direction_composite.rs` (read-only cross-link). Everything else is **additive**: two `Option<T>` fields on `DaySnapshot`, three new variants in `semantic_graph/ontology.rs` (6-slice pattern), two new enum variants in `reasoning/types.rs`, two new `pub const` source_ids, two 1-line evidence backfills in `almanac/thai_tue.rs` + `almanac/sat_phuong.rs`.

**Major components:**
1. **`reasoning/iching/` (NEW)** — self-contained pillar: `schema.rs` (ADR-locked types), `corpus.rs` (`OnceLock` loader), `mai_hoa.rs` (casting), `bien_que.rs` (transformation), `evaluator.rs` (`impl ActionEvaluator for IChingEvaluator`), `golden.rs` (≥10 cross-source cases).
2. **`reasoning/direction_composite.rs` (NEW)** — read-only join of KHCBPPT Thái Tuế/Tam Sát + Huyền Không Phi Tinh; emits 3 envelopes (`khcbppt` + `huyen-khong` + `rule.composite.direction_cross_link`); takes only `&` references.
3. **`reasoning/personal.rs` (MODIFIED, additive)** — two new builder methods on `PersonalReasoningInput`; no signature changes to existing methods.
4. **`almanac/thai_tue.rs` + `almanac/sat_phuong.rs` (MODIFIED, 1 line each)** — populate `evidence.source_id = SOURCE_KHCBPPT.to_string()` (currently `None`).
5. **`semantic_graph/ontology.rs` (MODIFIED)** — 6-slice additions for `NodeConcept::Hexagram`, `EdgeConcept::LocatedAt`, `EdgeConcept::Transforms` (×3 concepts × 6 slices = ~18 mechanical edits).
6. **`sources.rs` + `tests/source_id_guard.rs` (MODIFIED)** — two new `pub const` + two new `FORBIDDEN_LITERALS` rows.

**Key pattern: schema-lock-first (CRIT-1 prevention).** Land `HexagramEntry` types with `#[serde(deny_unknown_fields)]` + ADR-0005 + serde round-trip tests on a 1-entry fixture BEFORE authoring any of the 64 entries. Re-editing 448 text fields after a schema slip is prohibitively expensive (v1.5 lesson × 7).

**Key pattern: composite-envelope multi-source provenance (CRIT-5 prevention).** Cross-link emits **multiple** `ReasoningEvidenceEnvelope` entries on the same `PersonalFactNode`, each with its distinct primitive `source_id`, plus ONE composite envelope with `source_id: "rule.composite.direction_cross_link"`. This is the only pattern compatible with the CRIT-3 grep guard.

### Critical Pitfalls

1. **CRIT-1 — Schema-slip after corpus authored.** Lock `HexagramEntry` types + ADR-0005 in Phase 1 before any of 64 entries lands. **Cannot be retrofitted.** (v1.5 CRIT-1/5 lesson × 7.)
2. **CRIT-2 — Mai Hoa `% 8 == 0` / `% 6 == 0` remainder-zero convention.** Naïve `sum % 8` produces Tiên Thiên 1 (Kiền) instead of the correct 8 (Khôn). Prevention: implement as named helpers returning typed `TienThienTrigram(1..=8)` / `MovingLine(1..=6)`; golden dataset MUST include `month=8/day=8/hour=8` boundary case.
3. **CRIT-3 — Tiên Thiên trigram numbers vs King Wen hexagram numbers.** Both use "1..N" but with completely different mappings (Tiên Thiên (3,3) = Ly over Ly = King Wen #30, NOT #3). Prevention: three distinct newtypes `TienThienTrigram(u8)` / `HauThienTrigram(u8)` / `KingWenHexagram(u8)` with NO `From` impls between them; composition is the only path `(TienThienTrigram, TienThienTrigram) → KingWenHexagram`; 64-entry composition table validated at load.
4. **CRIT-5 — Cross-link collapses CRIT-3 isolation.** Prevention: module lives in `reasoning/`, not `interaction/`; function takes only `&` references; emits distinct primitive `source_id`s + composite `rule.composite.*`; extend `tests/fengshui_crit3_isolation.rs` with sibling `tests/thai_tue_cross_link_crit3.rs`.
5. **CRIT-6 — `kinh-dich` vs `mai-hoa-dich-so` source-id cross-contamination.** A single consultation uses BOTH (casting step = `mai-hoa-dich-so`; text lookup step = `kinh-dich`). Prevention: register both `pub const` in Phase 1; emit per-step `ReasoningEvidenceEnvelope` instances; contract test asserts ≥2 distinct primitive source_ids + 1 composite per consultation.

(Moderate pitfalls MOD-1 through MOD-8 and MIN-1 through MIN-5 are catalogued in PITFALLS.md; the highest-impact ones — MOD-2 Thể/Dụng inversion, MOD-3 classical-vs-commentary text fidelity, MOD-5 additive `Option<T>` regression, MOD-7 Tier-0 contract, MOD-8 KHCBPPT subfamily tags — each map to a phase in the Pitfall-to-Phase Mapping table.)

---

## Implications for Roadmap

Based on combined research, v1.7 should follow an **8-phase structure** (mirroring v1.5's schema-lock-first precedent, with the Thái Tuế cross-link on a parallel track that merges at the semantic-graph wiring phase). The critical path is **Phase 1 → 3 → 4 → 5 → 7 → 8** (IChing pillar); the parallel track is **Phase 1 → 2 → 6** (Thái Tuế cross-link).

### Phase 1: Foundation — schemas, sources, ADRs, ontology (BLOCKING)
**Rationale:** Every downstream phase needs source_ids registered, the `IChing` source-family enum variant, the 6-slice ontology extended, and the schema locked. Doing any out-of-order triggers rework.
**Delivers:** ADR-0005 (IChing schema), ADR-0006 (Mai Hoa casting), ADR-0007 (cross-link CRIT-3 carve-out); `SOURCE_KINH_DICH` / `SOURCE_MAI_HOA_DICH_SO` constants; `FORBIDDEN_LITERALS` extended; `ReasoningEvidenceSourceFamily::IChing` + `ActionId::IChing`; `Hexagram` / `LocatedAt` / `Transforms` 6-slice ontology entries; `HexagramEntry` types with `deny_unknown_fields` + 1-entry serde round-trip probe; the 64-entry Tiên Thiên → King Wen composition table + typed `TienThienTrigram` / `KingWenHexagram` newtypes.
**Addresses:** FS-07 (source_id registration), FS-02 (trigram map), partial FS-03 (schema), partial FS-08 (ontology slots).
**Avoids:** CRIT-1 (schema-lock), CRIT-3 (typed trigram/hexagram boundary), CRIT-6 (source_id registration + guard), MIN-2 (Trigram vs Hexagram vocabulary).
**Research flag:** ⚠️ Needs **deeper research** during planning — the Tiên Thiên number assignment (1=Kiền..8=Khôn is the dominant convention; at least one Vietnamese sub-school differs) must be pinned to a page reference in ADR-0006 before the algorithm lands.

### Phase 2: Thái Tuế evidence backfill (PARALLEL with Phases 3–5)
**Rationale:** Two 1-line `evidence: None` → `Some(RuleEvidence { source_id: SOURCE_KHCBPPT, ... })` backfills in `almanac/thai_tue.rs:107-111` and `almanac/sat_phuong.rs:49-53`. Pure additive backfill; existing call-sites ignore the field; no corpus or algorithm work blocks it.
**Delivers:** Cross-link's prerequisites (KHCBPPT-side evidence becomes citable).
**Addresses:** Cross-link prerequisite for FS-11.
**Research flag:** Standard pattern (mirror v1.6 RIT-11 reviewer-field closure). Skip research-phase.

### Phase 3: IChing corpus + loader (DEPENDS ON Phase 1)
**Rationale:** Corpus authors need the locked schema from Phase 1 before they can write entries. The loader cannot exist before the schema. 64 entries × ~7 text fields = 448 text fields — long-pole task; file as its own epic in `bd`.
**Delivers:** `data/iching/hexagrams.json` (64 entries, NFC-normalized, reviewer-signed); `reasoning/iching/corpus.rs` (`OnceLock` loader mirroring `rituals/corpus.rs:85-117`); `data/iching/manifest.json` + `provenance_audit.md`.
**Uses:** `include_str!` + `OnceLock` + `unicode-normalization` + `serde_json::from_str`.
**Addresses:** FS-03 (64-hexagram corpus).
**Avoids:** CRIT-1 (schema-lock already landed), MOD-3 (classical vs commentary layer separation), MOD-4 (NFC), MIN-1 (file layout: recommend 8 files grouped by King Wen octant for parallel review), MIN-4 (Hán-Việt vs Chinese-character fields).
**Research flag:** ⚠️ Needs **deeper research** during planning — Ngô Tất Tố corpus completeness (does the source include all 64 hexagrams with both thoán từ AND all 6 hào từ? are hexagrams 1 & 2's 7th "dụng" hào included?) must be answered before authoring. Surface gaps as `PendingExternalReview` (v1.6 RIT-14 pattern) — never silently fill from another translator (AF-05).

### Phase 4: Mai Hoa casting algorithm + biến quẻ (DEPENDS ON Phase 3)
**Rationale:** Algorithm consumes the corpus loader (`get_hexagram_by_number`); cannot test without it.
**Delivers:** `reasoning/iching/mai_hoa.rs` (casting), `reasoning/iching/bien_que.rs` (transformation), `reasoning/iching/golden.rs` (≥10 cross-source cases). 384-case (64 chủ quẻ × 6 động hào) biến quẻ contract test. 4320-cast enumeration distribution test.
**Uses:** `chrono::Datelike` integer arithmetic; `u8` bit math for biến quẻ.
**Addresses:** FS-01 (casting), FS-04 (biến quẻ), FS-05 (Thể/Dụng), FS-12 (golden tests), DF-02 (Ngũ Hành matrix — optional enhancement).
**Avoids:** CRIT-2 (remainder-zero convention; named helpers), CRIT-3 (composition table already locked in Phase 1), CRIT-4 (biến quẻ bit-position; 384-case test), MOD-1 (hào từ vs thoán từ selection), MOD-2 (Thể/Dụng inversion), MIN-3 (year-parametrized casting tests).
**Research flag:** ⚠️ Needs **deeper research** during planning — edge-case tiebreaks (raw sum mod 8 == 0, hour chi indexing with DEC-0017) need at least one printed-table golden case per edge. Lunar-vs-solar input convention must be pinned (recommend lunar per Thiệu Khang Tiết tradition; existing `lunar.rs` does correct Vietnamese conversion).

### Phase 5: IChing evaluator + DaySnapshot integration (DEPENDS ON Phase 4)
**Rationale:** Evaluator needs the algorithm (Phase 4) to cast and the corpus (Phase 3) to look up text. DTO field needs the evaluator to populate it.
**Delivers:** `reasoning/iching/evaluator.rs` (`impl ActionEvaluator for IChingEvaluator`); sibling `IChingQuery` newtype (NOT a `ConsultationIntent::IChing` variant — reconciliation above); additive `DaySnapshot.iching_cast: Option<IChingCastSummary>` field with `#[serde(default, skip_serializing_if = "Option::is_none")]`; `reasoning/personal.rs::build_iching_fact_nodes()` method; `reasoning/mod.rs::pub use iching::*`.
**Addresses:** FS-06 (evaluator integration), partial FS-08 (reasoning graph integration).
**Avoids:** CRIT-6 (per-step evidence envelopes; contract test asserts ≥2 distinct primitive source_ids + 1 composite), MOD-5 (additive `Option<T>`; round-trip v1.6 fixtures), MOD-6 (composite envelope granularity ≥3 entries), MOD-7 (Tier-0 contract: `cast_iching` with `birth = None` must succeed).
**Research flag:** Standard pattern (mirror v1.5 Phi Tinh T0/T2 split). Skip research-phase.

### Phase 6: Thái Tuế ⇄ Phi Tinh cross-link (PARALLEL after Phase 2; INDEPENDENT of Phases 3–5)
**Rationale:** Cross-link consumes `compute_thai_tue` (Phase 2 backfill), `get_sat_phuong` (shipped v1.0), and `snapshot.flying_stars` (shipped v1.5). It does NOT touch anything in `reasoning/iching/`. CRIT-3 isolation is preserved by module placement (`reasoning/`, not `interaction/`) and read-only signatures.
**Delivers:** `reasoning/direction_composite.rs::build_direction_cross_link(snapshot, birth_chi_index) -> PersonalFactNode` with 3 evidence envelopes; `tests/thai_tue_cross_link_crit3.rs` (extended CRIT-3 grep guard); additive `DaySnapshot.direction_cross_link: Option<DirectionCrossLinkSummary>`; directional Thái Tuế `pub fn` on `thai_tue.rs` (FS-09); classical 3-direction Tam Sát module `almanac/tam_sat.rs` (FS-10, option b per FEATURES recommendation) — **DEC required first**.
**Addresses:** FS-09 (Thái Tuế directional), FS-10 (Tam Sát directional — decision-dependent), FS-11 (cross-link).
**Avoids:** CRIT-5 (CRIT-3 isolation; distinct source_ids; module placement), MOD-8 (KHCBPPT subfamily tags `SatPhuong` / `TamSat` / `ThaiTue`).
**Research flag:** ⚠️ Needs **deeper research** during planning — **FS-10 3-vs-1 direction decision** must be resolved (recommend option b: new `tam_sat.rs`); the KHCBPPT-pinned citation for classical 3-direction Tam Sát (Dần-Ngọ-Tuất → Sát Bắc at three contiguous sơn, etc.) must be located before implementation. **Decision point**, not a blocker.

### Phase 7: Semantic graph wiring (DEPENDS ON Phases 5 + 6)
**Rationale:** Builder consumes everything produced by Phases 5 and 6. Wiring earlier would compile against stubs and miss integration bugs.
**Delivers:** `semantic_graph/builders/day_snapshot.rs` — two new private methods `add_iching_facts()` + `add_direction_composite_facts()`; two new call lines in `new()`. Hexagram nodes + `Transforms`/`LocatedAt` edges for primary + biến quẻ; composite fact node for the cross-link.
**Addresses:** FS-08 (Hexagram node + edges).
**Research flag:** Standard pattern (mirror v1.5 FlyingStar/Offering builders). Skip research-phase.

### Phase 8: E2E validation (DEPENDS ON all above)
**Rationale:** Final smoke + backward-compat round-trip + golden cross-source validation.
**Delivers:** `tests/integration_2026_smoke.rs` extension; `tests/day_snapshot_v14_compat.rs` pattern (v1.6 producer JSON must deserialize cleanly into v1.7 `DaySnapshot`); `tests/iching_golden.rs` independent golden verification; ≥10 IChing golden cases × ≥2 sources per Expansion Framework §7.
**Avoids:** All "Looks Done But Isn't" checklist items in PITFALLS.md — verified end-to-end.
**Research flag:** Standard pattern. Skip research-phase.

### Phase Ordering Rationale

- **Schema-lock first (Phase 1) because:** 448 corpus fields × re-edit cost is non-negotiable (v1.5 CRIT-1/5 lesson × 7); the typed `TienThienTrigram` / `KingWenHexagram` boundary is the only CRIT-3 prevention that can't be retrofitted; source_id registration must precede any code that references them (CRIT-6).
- **Cross-link on a parallel track (Phase 2 → 6) because:** it depends only on Phase 1 (sources) + Phase 2 (evidence backfill) + already-shipped v1.5 FlyingStar — completely independent of the IChing pillar. Parallelising cuts ~2 phases off the critical path.
- **Algorithm AFTER corpus (Phase 4 after 3) because:** the casting function calls `get_hexagram_by_number`; without the loader, the algorithm has nothing to test against. Mirrors v1.5 Phase 10 → 12 ordering.
- **Evaluator AFTER algorithm (Phase 5 after 4) because:** the evaluator composes cast + lookup + biến quẻ; it IS the integration point.
- **Semantic graph LAST before validation (Phase 7) because:** the builder consumes everything; wiring earlier would compile against stubs.

### Research Flags

**Phases likely needing deeper research during planning:**
- **Phase 1:** Tiên Thiên number arrangement pinning (ADR-0006 must cite a page in Thiệu Khang Tiết's *Mai Hoa Dịch Số*); Hán-Việt orthography choice (modern `thuỷ` vs pre-1975 `thủy`).
- **Phase 3:** Ngô Tất Tố corpus completeness (all 64 hexagrams? hexagrams 1 & 2 7th "dụng" hào? sparse entries?); translation-layer separation (MOD-3).
- **Phase 4:** Mai Hoa edge-case tiebreaks (raw sum mod 8 == 0; hour chi indexing with DEC-0017); lunar-vs-solar input convention.
- **Phase 6:** **FS-10 3-vs-1 direction decision** (recommend option b: new `tam_sat.rs`); classical Tam Sát KHCBPPT citation.

**Phases with standard patterns (skip research-phase):**
- **Phase 2:** v1.6 RIT-11 reviewer-field closure precedent.
- **Phase 5:** v1.5 Phi Tinh T0/T2 evaluator split precedent.
- **Phase 7:** v1.5 FlyingStar/Offering semantic-graph builder precedent.
- **Phase 8:** v1.5/v1.6 milestone-audit + golden-case validation methodology.

---

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | **HIGH** | Zero new deps; every recommendation anchored in v1.5/v1.6 code paths with `file:line` refs. Crates.io survey rejected all upstream I-Ching libraries on source-provenance grounds (DEC-0015/0016 + ADR-0001). |
| Features | **HIGH** on Mai Hoa casting algorithm + 64-hexagram data shape; **MEDIUM-HIGH** on Tam Sát directional conventions (existing-module gap forces FS-10 decision). |
| Architecture | **HIGH** | Every component slot, integration point, and pattern grounded in v1.5 Văn khấn + v1.6 Phi Tinh precedents. Sibling-newtype resolution avoids the `Copy`-break blast radius. |
| Pitfalls | **HIGH** on integration pitfalls (CRIT-1/5/6 anchored in v1.5 lessons); **MEDIUM** on Mai Hoa casting-micro-rule claims (Tiên Thiên arrangement + remainder-zero convention must be re-verified against Thiệu Khang Tiết's text in P-KD-1). |

**Overall confidence:** **HIGH** — the v1.7 milestone is the third exercise of the v1.5 "no new deps + schema-lock-first + additive DTO + dual-source provenance" pattern. The new domain content (Kinh Dịch) is well-documented and the integration shape is established.

### Gaps to Address / Open Questions

- **FS-10 3-vs-1 direction decision** (FEATURES gap, surface as roadmap decision point in Phase 6): reuse existing single-direction `sat_phuong.rs` OR implement classical 3-direction Tam Sát as new `almanac/tam_sat.rs`. **Recommend option (b)** for KHCBPPT correctness parity; file a DEC + locate classical citation before Phase 6 lands.
- **Lunar-vs-solar Mai Hoa input convention**: Thiệu Khang Tiết's tradition uses lunar; the project's `lunar.rs` does correct Vietnamese conversion. Pin in ADR-0006 (Phase 1).
- **Ngô Tất Tố corpus completeness**: Does the source include all 64 hexagrams with both thoán từ AND all 6 hào từ? Are hexagrams 1 & 2's 7th "dụng" hào (per Wikipedia I Ching §Structure) included? Answer before Phase 3 authoring; design `hao_tu: Vec<HaoTu>` to allow 7 entries only for those two; surface gaps as `PendingExternalReview` (v1.6 RIT-14 pattern) — never silently fill from another translator (AF-05).
- **Tiên Thiên number arrangement**: 1=Kiền..8=Khôn is the dominant convention; at least one Vietnamese sub-school differs. Pin to a page reference in Thiệu Khang Tiết's text during Phase 1 ADR-0006.
- **Mai Hoa remainder-zero edge cases**: raw sum mod 8 == 0 boundary; hour chi indexing convention with DEC-0017 (early-Tý/late-Tý split). Each edge needs ≥1 printed-table golden case in Phase 4.
- **Tam Sát module existence check** (PITFALLS MOD-8): confirm `almanac/tam_sat.rs` does NOT exist (only `than_sat.rs`); verify whether `than_sat.rs` covers classical Tam Sát or whether it's net-new in v1.7.

---

## Sources

### Primary (HIGH confidence — in-repo anchors)
- `.planning/PROJECT.md` — v1.0–v1.6 milestone history; v1.7 milestone scope; Key Decisions table (DEC-0023 source_id discipline; schema-lock-before-corpus; additive `Option<T>`; CRIT-3 isolation; ADR-0001..0004 pattern).
- `.planning/research/EXPANSION_FRAMEWORK.md` — §2.2 (Kinh Dịch pillar spec, Tier-0), §3.1 (source provenance), §3.2 (semantic-graph extension + `rule.composite.*`), §7 (validation references).
- `.planning/research/{STACK,FEATURES,ARCHITECTURE,PITFALLS}.md` — the four v1.7 research files synthesised here.
- `crates/amlich-core/Cargo.toml` — confirmed `serde`/`serde_json`/`chrono` (workspace) + `unicode-normalization = "0.1.25"` only.
- `crates/amlich-core/src/rituals/corpus.rs:17,27-56,85-117` — `OnceLock + include_str!` corpus loader pattern (v1.5 precedent).
- `crates/amlich-core/src/rituals/schema.rs` — `#[serde(deny_unknown_fields)]` schema-lock pattern (ADR-0001 precedent).
- `crates/amlich-core/src/sources.rs:7-26,41,48-56` — `pub const SOURCE_*` taxonomy + CI test pattern (DEC-0023).
- `crates/amlich-core/tests/source_id_guard.rs:13-21,17-25` — `FORBIDDEN_LITERALS` list to extend.
- `crates/amlich-core/src/reasoning/personal.rs:31-107` — `PersonalReasoningInput::build_fact_nodes` integration point.
- `crates/amlich-core/src/reasoning/types.rs:144-151` — `ReasoningEvidenceEnvelope` shape.
- `crates/amlich-core/src/reasoning/action_evaluator.rs:51-67` — `ActionEvaluator` trait (IChingEvaluator target).
- `crates/amlich-core/src/advisory.rs:18-30` — `ConsultationIntent` enum (must NOT be extended; sibling-newtype instead).
- `crates/amlich-core/src/almanac/fengshui/types.rs:120-143` — sibling-not-extend pattern (`FlyingStarLayout` / `DailyFlyingStarLayout`).
- `crates/amlich-core/src/almanac/thai_tue.rs:14-41,107-111` — Thái Tuế types (personal-conflict-only); backfill target.
- `crates/amlich-core/src/almanac/sat_phuong.rs:23-43,49-53` — Sát Phương direction table; backfill target.
- `crates/amlich-core/src/almanac/fengshui/mod.rs:10-11` — explicit CRIT-3 isolation note.
- `crates/amlich-core/src/semantic_graph/ontology.rs:5-411` — 6-slice ontology discipline (full file).
- `crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs:476-747,697-744` — FlyingStar/Ritual/Offering builder precedent; INT-09 dual-source provenance precedent.
- `crates/amlich-core/tests/fengshui_crit3_isolation.rs:14-21` — CRIT-3 grep guard (`FORBIDDEN_TYPE_NAMES`).
- `.planning/milestones/v1.5-ROADMAP.md` — schema-lock-first phase ordering (Phase 10 → 11 → 12 → 13 → 14 → 15).
- `.planning/adrs/0001-ritual-schema-v1.md`, `0002`, `0003`, `0003a`, `0004` — ADR template + schema-lock-first precedent.

### Secondary (MEDIUM confidence — classical domain knowledge)
- Thiệu Khang Tiết (邵雍), *Mai Hoa Dịch Số* (梅花易數), NXB Văn Hoá Thông tin 2002 — classical casting-algorithm reference; Tiên Thiên trigram numbers; động hào derivation; Thể/Dụng rule.
- Ngô Tất Tố, *Kinh Dịch Trọn Bộ* — 64-hexagram text corpus reference; King Wen ordering; layered text + commentary.
- vi.wikipedia — Mai Hoa Dịch Số (casting algorithm, HIGH confidence on mod-8/mod-6/Tiên Thiên numbering; MEDIUM on edge-case tiebreaks).
- en.wikipedia — I Ching / I Ching divination (line number semantics 6/7/8/9; hexagram structure; King Wen sequence); Shao Yong (authorship attribution).
- EXPANSION_FRAMEWORK §7 named validation references: `nhantu.net` (Mai Hoa casting), `divination.com` (hexagram texts).

### Tertiary (LOW confidence — needs validation during P-KD-1)
- Specific Tiên Thiên number arrangement (1=Kiền..8=Khôn dominant, but at least one Vietnamese sub-school differs).
- Mai Hoa `% 8 == 0 → 8` boundary convention (classical and well-known in Vietnamese manuals, but the exact Thiệu Khang Tiết wording must be re-verified against the project's chosen reference text).
- Classical 3-direction Tam Sát KHCBPPT citation (needed before Phase 6 if option b chosen).
- Ngô Tất Tố corpus completeness for all 64 hexagrams (especially hexagrams 1 & 2 7th "dụng" hào).

---
*Research completed: 2026-07-16*
*Ready for roadmap: yes*
