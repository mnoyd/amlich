# Feature Research

**Domain:** Vietnamese almanac — Kinh Dịch (I-Ching divination) pillar + Thái Tuế/Tam Sát ⇄ Phi Tinh directional cross-link
**Researched:** 2026-07-16
**Confidence:** HIGH for Mai Hoa casting algorithm & 64-hexagram data shape; MEDIUM-HIGH for Tam Sát directional conventions (existing-module gap)
**Milestone:** v1.7 (P2 pillar per `EXPANSION_FRAMEWORK.md` §2.2 + §5)

> Scope of this file: **only the NEW features** in v1.7. Core calendar, Ten Gods/Kua/Dai Van, hour pillar/60-cycle/Na Am, Văn khấn, Phi Tinh overlays, semantic-graph provenance — all already shipped v1.0–v1.6, **deliberately not re-researched here**.

---

## Feature Landscape

### Table Stakes (Users Expect These)

A v1.7 Kinh Dịch pillar without these feels incomplete or non-canonical. Every row maps to a downstream REQ-ID.

| ID | Feature | Why Expected | Complexity | Tier | Dependencies | Notes |
|----|---------|--------------|------------|------|--------------|-------|
| FS-01 | **Mai Hoa time-based casting** (`cast_hexagram_mai_hoa(lunar_year, lunar_month, lunar_day, chi_hour_index) -> CastHexagram`) | Tier-0 entry point; the whole point of "ask a question, get a quẻ". Vietnamese users expect deterministic time-numerology casting, not coin-toss. | **HIGH** | T0 | existing `convert_solar_to_lunar`, `get_day_canchi`, `CHI[12]` for chi-hour index | Algorithm pinned below in §"Mai Hoa Casting Algorithm". No RNG — deterministic. |
| FS-02 | **Tiên Thiên Bát Quái numerical map** (Càn=1, Đoài=2, Ly=3, Chấn=4, Tốn=5, Khảm=6, Cấn=7, Khôn=8) | Foundational lookup; required by FS-01 upper/lower trigram derivation. | **LOW** | T0 | none | Pure static `const` table. |
| FS-03 | **64-hexagram lookup corpus** (`HexagramRecord { king_wen_index, vi_name, chinese_name, upper_trigram, lower_trigram, thoai_tu (quái từ), hao_tu[6] (hào từ), tuong_truyen (optional), cat_hung_verdict }`) | Without this, casting produces an empty pointer. `source_id: kinh-dich` (Ngô Tất Tố *Kinh Dịch Trọn Bộ*). | **HIGH** | T0 | none (data-only); frozen via `deny_unknown_fields` schema (mirrors v1.5 ADR-0001) | 64 fixed records; biggest single deliverable. **Schema must be locked before corpus authoring** (per PITFALLS CRIT-1/5 discipline from v1.5). |
| FS-04 | **Biến quẻ derivation** (`derive_bien_que(primary_index, moving_line_position) -> HexagramRecord`) | Mai Hoa always produces a biến quẻ; users expect "chủ quẻ → biến quẻ" pair. Cát-hùng-over-time reading. | **LOW** | T0 | FS-03 (lookup) | Pure function: flip the moving line (6↔9, yin↔yang), recompute King Wen index. |
| FS-05 | **Thể / Dụng classification** (`classify_the_dung(upper_trigram, lower_trigram, moving_line_position) -> { body_trigram, application_trigram }`) | Required for Ngũ Hành sinh khắc reading — central to Mai Hoa interpretation per Thiệu Khang Tiết. | **LOW** | T0 | FS-02 (trigram → element) | Rule: the trigram **not containing** the moving line is Thể (body/self); the one containing it is Dụng (application/affair). |
| FS-06 | **`ConsultationIntent::IChing { question: String }` evaluator branch** in `reasoning/personal.rs` | Provides the API surface for engine consumers; returns `ReasoningEvidenceEnvelope { source_id: "kinh-dich" / "mai-hoa-dich-so", ... }`. | **MED** | T0 | FS-01, FS-03, FS-04, FS-05; existing `ConsultationIntent` enum (`advisory.rs:20`) | Extends `ConsultationIntent` enum (currently 9 activity intents — all activity-based; `IChing` is **query-based**, the first non-activity intent). |
| FS-07 | **`source_id` registration**: `kinh-dich` (Ngô Tất Tố) + `mai-hoa-dich-so` (Thiệu Khang Tiết) | DEC-0023 discipline (`sources.rs` `pub const`) + CI grep guard forbids bare literals. | **LOW** | T0 | existing `sources.rs` pattern | Two new `pub const SOURCE_KINH_DICH` / `SOURCE_MAI_HOA_DICH_SO`. |
| FS-08 | **`Hexagram` semantic-graph node + edges** (`Transforms` chủ→biến, `LocatedAt` for moving-line position) | Graph-native provenance per framework §3.2; matches v1.5 `Hexagram` slot already declared in framework. | **MED** | T0 | existing `ReasoningGraphExport`, `ReasoningNodeExport`, `ReasoningEdgeExport` (`reasoning/types.rs`) | Reuses existing edge types; adds new `EdgeJustification::HexagramTransform` variant. |
| FS-09 | **Thái Tuế directional derivation** (`thai_tue_direction(year_chi_index) -> Direction8`) | The directional aspect of Thái Tuế (year chi position = Thái Tuế direction; "phạm Thái Tuế phương" = sit/facing it). | **LOW** | T0 | `CHI[12]`, existing 8-direction vocabulary | **GAP**: existing `thai_tue.rs` is **personal-conflict only** (5 kinds: Trực/Xung/Hại/Hình/Phá Thái Tuế between birth chi and year chi). The directional aspect is NOT yet computed anywhere — must be added as a new function or module. |
| FS-10 | **Tam Sát / Sát Phương directional** — **decision required**: (a) reuse existing `sat_phuong.rs` (single direction per chi, `chi % 4` grouping) OR (b) implement full classical Tam Sát (3 directions per year chi from Tam Hợp triad opposition) | The directional cross-link needs a Sát indicator per direction; classical VN almanacs surface all 3 Sát positions. | **MED** | T0 | existing `sat_phuong.rs` OR new `almanac/tam_sat.rs` | **GAP / DECISION**: existing `sat_phuong.rs` returns ONE cardinal direction per chi via a Tam Hợp-simplified mapping (`chi % 4`). Classical Tam Sát (三煞) is **THREE** branches/directions (opposite Tam Hợp triad). Roadmap must pick: (a) document simplification + reuse, or (b) new `tam_sat.rs` with 3-direction classical rule. Recommend (b) for correctness parity with KHCBPPT; keep `sat_phuong.rs` as a separate "Sát Phương by day chi" feature. |
| FS-11 | **Thái Tuế / Tam Sát ⇄ Phi Tinh read-only cross-link** — directional composite view | Carries "should-have" forward from v1.5 research. Engine consumers want one picture: "this direction has Thái Tuế, Sát at huyền-không palace X with star Y". | **MED-HIGH** | T0 (Tier-0 join: only calendar + palace) | FS-09, FS-10, existing `huyen-khong` palace layout, existing `ReasoningEvidenceEnvelope` | **CRITICAL CONSTRAINT**: CRIT-3 isolation (PROJECT.md, v1.5 audit) — join happens ONLY at reasoning-envelope layer with **distinct source_ids** (`khcbppt` + `huyen-khong`); NEVER wire `FlyingStar` into `interaction/direction_merge.rs`. Composite `source_id` follows `rule.composite.*` pattern per framework §3.2. |
| FS-12 | **Golden tests** (≥10 casting cases cross-checked against ≥2 independent sources — vi.wikipedia Mai Hoa algorithm + nhantu.net or *Mai Hoa Dịch Số* printed tables) | Framework §6 mandates golden test per pillar with divergence logging as `KnownDivergence`, never silent fix. | **MED** | T0 | FS-01 | Algorithm is deterministic; fixture = `(lunar_ymdh, chi_hour) → expected (upper, lower, moving_line)` triples. |

### Differentiators (Competitive Advantage)

Optional high-UX features. v1.7 ships at most one or two; defer the rest to v1.8+.

| ID | Feature | Value Proposition | Complexity | Tier | Dependencies | Notes |
|----|---------|-------------------|------------|------|--------------|-------|
| DF-01 | **Tier-2 Bazi enrichment of hexagram reading** — overlay Nhật Chủ element on Thể/Dụng trigram-element analysis ("your day-master is Kim, Thể is Kim → vượng; Dụng is Mộc → khắc lợi cho bạn") | Personalizes the otherwise-generic hexagram reading. Closes the gap between Tier-0 divination and Tier-2 Bazi personality. | **MED** | T0 base + **T2 enrich** (T0 path returns `enrichment: None`) | FS-05, existing `bazi::compute_bazi_metrics`, existing `compute_element_distribution` | Pattern mirrors v1.5 Phi Tinh: Tier-0 always works; Tier-2 adds a section. |
| DF-02 | **Full Ngũ Hành sinh khắc matrix for Thể/Dụng** (Thể sinh Dụng / Dụng sinh Thể / Thể khắc Dụng / Dụng khắc Thể / tỷ hòa) — table-driven verdicts | Saves the consumer from re-deriving five-phase rules; "Thể khắc Dụng = cát" is the kind of one-line verdict users want. | **LOW** | T0 | FS-05, FS-02 | Pure lookup table; 5 × relationship → verdict text. |
| DF-03 | **Hỗ Quái (nuclear hexagram)** — derive from lines 2-3-4 (lower) + 3-4-5 (upper) of the chủ quẻ | Surfaces the "hidden middle" of the reading; standard Mai Hoa depth technique. | **MED** | T0 | FS-03 | Mentioned in vi.wikipedia Mai Hoa §"Thành quẻ". |
| DF-04 | **24-sơn directional resolution** for Thái Tuế / Tam Sát (instead of 8-direction) | Matches classical KHCBPPT precision (24 sơn = 15° each). Most consumer apps stop at 8; doing 24 is a differentiator. | **MED-HIGH** | T0 | FS-09, FS-10, new 24-mountain table | Likely defer to v1.8; flag for `Tier 3 SpatialInput` co-design (framework §3.3). |
| DF-05 | **Pre-cast intent capture** (question text + deterministic time-seed log) | Auditability / "why this quẻ" traceability; lets the user verify the cast was at the right time. | **LOW** | T0 | FS-01 | Store `question` on `ConsultationIntent::IChing` + include in evidence note. Already half-required by FS-06. |

### Anti-Features (Commonly Requested, Often Problematic)

Features to **explicitly exclude** from v1.7. Each row prevents scope creep.

| ID | Feature | Why Requested | Why Problematic | Alternative |
|----|---------|---------------|-----------------|-------------|
| AF-01 | **Stalk / coin / yarrow random casting** (Wen Wang Gua / Lục Dao) | "Real I-Ching users want to toss coins." | (a) Different tradition from Mai Hoa; (b) requires RNG source — violates amlich's deterministic-correctness stance; (c) dilutes `source_id` discipline (would need `wen-wang-gua` source). | Mai Hoa time-numerology only for v1.7. If coin-cast is ever added, it lands as a **separate** milestone with its own `source_id` and an explicit RNG-injection interface. |
| AF-02 | **LLM-generated free-form interpretation** of hexagrams | "Modern UX — let AI explain the quẻ." | (a) Breaks canonical-source correctness stance (PROJECT.md Core Value); (b) no audit trail; (c) non-deterministic. | The **Ngô Tất Tố corpus IS the interpretation** (thoán từ + hào từ + cát-hùng verdict). Surface verbatim. No prose generation. |
| AF-03 | **Spatial feng-shui composition** (wire `FlyingStar` into `interaction/direction_merge.rs` to compute per-room layouts) | "If I have palace stars AND Thái Tuế direction, why not merge them per room?" | CRIT-3 isolation (PROJECT.md, v1.5 audit). Merging distinct `source_id` families at the interaction layer destroys provenance. | Cross-link stays at **reasoning-envelope layer only** (FS-11). True spatial composition is Tier-3 `spatial_compose` (framework §3.3), deferred to v1.9+. |
| AF-04 | **Personalized Thái Tuế conflict rewriting** — modifying the personal Thái Tuế result based on directional cross-link | "If Thái Tuế direction is bad, maybe override the personal verdict." | Cross-link is **read-only** by design; writing back creates cyclic provenance and double-counting. | Keep FS-11 strictly read-only. Personal Thái Tuế (`thai_tue.rs`) and directional Thái Tuế (FS-09) are **independent computations** with independent source_ids; both surface but neither rewrites the other. |
| AF-05 | **Mixing hexagram corpus sources** (e.g., pull hào từ from a different translator to fill gaps) | "Ngô Tất Tố is sparse in places; can we augment?" | Breaks single-`source_id` discipline (DEC-0015/0016/0023). Mixing translators yields inconsistent terminology/numbering. | Use **only** `kinh-dich` (Ngô Tất Tố) for v1.7. If gaps exist, log as `PendingExternalReview` (mirrors v1.6 RIT-14 pattern) — do not silently fill from another source. |
| AF-06 | **User-selectable casting variants** (Mai Hoa time vs Mai Hoa số vật vs Mai Hoa âm thanh vs Mai Hoa chữ viết) | "Wikipedia lists 10+ Mai Hoa casting methods." | (a) Time-numerology is the only Tier-0 deterministic method; (b) other methods require user free-form input (counted objects, heard sounds, written words) — Tier-incompatible and untestable. | Ship time-numerology only (FS-01). Other variants documented in research notes as out-of-scope; revisit if/when a `MaiHoaVariant` enum is justified by user demand. |

---

## Mai Hoa Casting Algorithm (Concrete Spec for FS-01)

Verified from vi.wikipedia Mai Hoa Dịch Số (citing Thiệu Khang Tiết, *Mai Hoa Dịch Số*, NXB Văn Hoá Thông tin 2002; cross-checked against Thiệu Vĩ Hoa *Chu Dịch với dự đoán học*). **HIGH confidence on the algorithm; MEDIUM on edge-case tiebreaks (golden test required).**

### Inputs (Tier-0 only)
```
lunar_year_branch_index  ∈ 0..12  (Tý=0 .. Hợi=11)   — "số chi năm"
lunar_month              ∈ 1..13                       — "số tháng" (âm lịch, 13 = nhuận)
lunar_day                ∈ 1..30                       — "số ngày"
chi_hour_index           ∈ 0..12  (Tý=0 .. Hợi=11)    — "số chi giờ" (12 chi giờ)
```

> **Note on hour index:** the project's `get_gio_hoang_dao` and `hour_pillar` already use a 12-slot chi-hour index; reuse it. The "13th slot" early-Tý/late-Tý split (DEC-0017) is **not** relevant here — Mai Hoa uses the chi identity, not the stem.

### Step 1 — Thượng quái (upper / outer trigram)
```
let raw_upper = year + month + day;                       // single sum, all four are chi/calendar numbers
let upper_idx = ((raw_upper - 1) % 8) + 1;                // 1..=8, NOT raw % 8 (classical "trừ 8": subtract 8 until ≤ 8)
let upper_trigram = TIEN_THIEN_BAT_QUAI[upper_idx];       // Càn=1..Khôn=8
```
**Edge case (golden-test):** if `raw_upper % 8 == 0`, classical "trừ 8" leaves remainder 8 → Khôn. The `((n-1) % 8) + 1` form achieves this without an `if`. **Verify against a printed table** — some variants use `raw % 8` with 0→8 substitution.

### Step 2 — Hạ quái (lower / inner trigram)
```
let raw_lower = year + month + day + chi_hour_index;
let lower_idx = ((raw_lower - 1) % 8) + 1;
let lower_trigram = TIEN_THIEN_BAT_QUAI[lower_idx];
```

### Step 3 — Hào động (moving line position)
```
let raw_moving = year + month + day + chi_hour_index;
let moving_line = ((raw_moving - 1) % 6) + 1;             // 1..=6 (1 = bottom/initial hào, 6 = top hào)
```
**Convention:** hào counted from the **bottom** (sơ hào = 1, thượng hào = 6). Same classical "trừ 6" rule.

### Step 4 — Thành quẻ (compose hexagram from upper + lower trigram)
King Wen index = lookup `(upper_trigram, lower_trigram) → king_wen_index 1..=64` via the standard 8×8 King Wen table. This is the **chủ quẻ / bản quẻ** (primary).

### Step 5 — Biến quẻ (transforming hexagram)
```
// The moving line lives in either the upper trigram (hào 4/5/6) or lower trigram (hào 1/2/3).
// Flip that line's polarity (yin↔yang) in the appropriate trigram.
let bien_que = flip_line(chu_que, moving_line);
```
The result is a NEW hexagram with its own King Wen index → its own `HexagramRecord` from FS-03.

### Step 6 — Thể / Dụng (FS-05)
- The trigram that **contains** the moving line = **Dụng** (application — the affair/other).
- The trigram that does **not** contain the moving line = **Thể** (body — self).
- Reading then proceeds via Ngũ Hành sinh khắc between Thể-element and Dụng-element (DF-02):
  - Thể sinh Dụng → hao tổn (draining)
  - Dụng sinh Thể → được trợ (supported) — cát
  - Thể khắc Dụng → được lợi (profitable) — cát
  - Dụng khắc Thể → bị khắc (suppressed) — hung
  - Thể/Dụng tỷ hòa (same element) → bình hòa (stable)

### Trigram → Element (Hậu Thiên attribution, for Thể/Dụng analysis)
| Trigram | Tiên Thiên# | Element | Direction (Hậu Thiên) |
|---------|-------------|---------|------------------------|
| Càn (乾) | 1 | Kim | Tây Bắc (NW) |
| Đoài (兌) | 2 | Kim | Tây (W) |
| Ly (離)  | 3 | Hỏa | Nam (S) |
| Chấn (震) | 4 | Mộc | Đông (E) |
| Tốn (巽) | 5 | Mộc | Đông Nam (SE) |
| Khảm (坎) | 6 | Thủy | Bắc (N) |
| Cấn (艮) | 7 | Thổ | Đông Bắc (NE) |
| Khôn (坤) | 8 | Thổ | Tây Nam (SW) |

> **Critical distinction (per vi.wikipedia):** the **Tiên Thiên#** (1..8) is used **only** for the casting step (FS-01). The **Hậu Thiên direction/element** is used for Thể/Dụng analysis (FS-05, DF-02) and for the directional cross-link (FS-11). Mixing them is a classic implementation bug.

---

## 64-Hexagram Data Shape (Concrete Spec for FS-03)

```rust
// crates/amlich-core/src/reasoning/iching/types.rs (sketch)

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Trigram {
    Qian = 1,   // Càn
    Dui = 2,    // Đoài
    Li = 3,     // Ly
    Zhen = 4,   // Chấn
    Xun = 5,    // Tốn
    Kan = 6,    // Khảm
    Gen = 7,    // Cấn
    Kun = 8,    // Khôn
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CatHungVerdict {
    Cat,       // cát (auspicious)
    Hung,      // hung (inauspicious)
    CatHungBan, // bán cát bán hung (mixed)
    TieuCat,   // tiểu cát (slightly auspicious)
    TieuHung,  // tiểu hung (slightly inauspicious)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HaoTu {
    pub position: u8,           // 1..=6 (bottom-to-top), plus 7 for the "dụng" hào of hexagrams 1 & 2 (Wikipedia I Ching §Structure note)
    pub is_yang: bool,          // true = 9, false = 6
    pub text_vi: String,        // hào từ (Ngô Tất Tố translation)
    pub note: Option<String>,   // optional commentary gloss
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HexagramRecord {
    pub king_wen_index: u8,     // 1..=64
    pub vi_name: String,        // "Thiên Trùng Càn" / "Địa Thái" / etc.
    pub chinese_name: String,   // "乾為天" / "地天泰"
    pub upper_trigram: Trigram,
    pub lower_trigram: Trigram,
    pub thoai_tu: String,       // 彖辭 / 卦辭 — hexagram statement (Ngô Tất Tố)
    pub tuong_truyen: Option<String>,  //大象傳 — image commentary (optional — Ngô Tất Tố may or may not include)
    pub hao_tu: Vec<HaoTu>,     // 6 entries (7 for hexagrams 1 & 2 per Wikipedia note)
    pub cat_hung: CatHungVerdict,
    pub source_id: String,      // "kinh-dich" (Ngô Tất Tố)
}
```

**Schema-lock discipline (carries v1.5 ADR-0001 pattern):** freeze this struct with `#[serde(deny_unknown_fields)]` BEFORE authoring the 64 records. Re-editing 64 corpus entries after a schema slip is the v1.5 PITFALLS CRIT-1/5 failure mode — do not repeat it.

---

## Cross-Link Join Shape (Concrete Spec for FS-11)

Read-only composite; no write-back to either source. Per CRIT-3 isolation (PROJECT.md, v1.5 audit) and `rule.composite.*` discipline (framework §3.2).

```rust
// crates/amlich-core/src/reasoning/iching/cross_link.rs (sketch)

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectionalCrossLinkEntry {
    pub direction: Direction8,                          // 8-palace directional vocabulary (matches huyen-khong Palace)
    pub thai_tue_here: bool,                            // from FS-09 (year chi == this direction's branch group)
    pub sat_phuong_here: bool,                          // from existing sat_phuong.rs (single direction)
    pub tam_sat_here: Option<bool>,                     // from FS-10 (3 directions) if implemented
    pub palace: Palace,                                 // from existing huyen-khong FlyingStarLayout
    pub palace_star: FlyingStar,                        // the annual star at this palace
    pub safety_hint: Option<&'static str>,              // from existing huyen-khong safety.rs
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectionalCrossLink {
    pub year: i32,
    pub entries: Vec<DirectionalCrossLinkEntry>,        // 8 entries (one per direction)
    pub evidence: Vec<ReasoningEvidenceEnvelope>,       // dual source_ids: khcbppt + huyen-khong
    // NEVER: a "merged_score" or "net_recommendation" — that would re-introduce CRIT-3 violation.
}
```

**Provenance envelope:**
- `khcbppt` evidence: covers `thai_tue_here` + `sat_phuong_here` (+ `tam_sat_here` if FS-10b chosen).
- `huyen-khong` evidence: covers `palace` + `palace_star` + `safety_hint`.
- Composite source_id: `rule.composite.directional_cross_link` (per framework §3.2).

**CRITICAL — what NOT to add:** a single merged recommendation score, a "best direction" ranking, or a write-back into `direction_merge.rs`. All of those would collapse the two source_ids and re-introduce the exact CRIT-3 violation v1.5 explicitly forbade. The cross-link is a **view**, not a **reducer**.

---

## Feature Dependencies

```
[FS-02 Tiên Thiên Bát Quái map] ────required──> [FS-01 Mai Hoa casting] ─┐
                                                                          │
[existing lunar + CanChi compute] ──required──> [FS-01] ──────────────────┤
                                                                          │
                                                       [FS-03 64-hexagram corpus] ─┐
                                                                          │           │
                                          [FS-04 Biến quẻ] <──requires─────┘           │
                                                                          │           │
                                          [FS-05 Thể/Dụng] <──requires─[FS-02 element]│
                                                                          │           │
                       [FS-06 ConsultationIntent::IChing] <──requires─────┴───┘
                                                                          │
                       [FS-07 source_id registration] <──requires──[FS-06 source_id use]
                                                                          │
                       [FS-08 Hexagram graph node] <──requires──[FS-06, FS-04]
                                                                          │
                       [FS-12 Golden tests] <──requires──[FS-01]

[FS-09 Thái Tuế directional] ──┐
                                ├──requires──> [FS-11 Cross-link] ──requires──> [existing huyen-khong palace layout]
[FS-10 Tam Sát / Sát Phương] ──┘

[DF-01 Bazi enrichment] ──enhances──> [FS-06]   (Tier-2 only; T0 path returns enrichment: None)
[DF-02 Ngũ Hành matrix] ──enhances──> [FS-05]
[DF-03 Hỗ Quái]        ──enhances──> [FS-04]

[AF-01 stalk/coin cast] ──conflicts──> [FS-01]   (different tradition; would need different source_id)
[AF-03 spatial compose] ──conflicts──> [FS-11]   (CRIT-3 violation; deferred to Tier-3 v1.9+)
[AF-04 personal rewrite] ──conflicts──> [FS-09, thai_tue.rs]   (cross-link is read-only)
```

### Dependency Notes

- **FS-01 requires FS-02 + existing lunar/CanChi compute**: the casting algorithm sums year+month+day+hour numbers and mods by 8 / 6 — all inputs come from already-shipped modules (`convert_solar_to_lunar`, `get_day_canchi`, `CHI[12]`).
- **FS-03 (corpus) has no code dependency on FS-01/02**: it is a static lookup table. **Recommended phase order**: lock the `HexagramRecord` schema (ADR-style), then author the 64 records in parallel with FS-01 implementation. This mirrors the v1.5 "schema-lock before corpus" decision (DEC-0023 / ADR-0001).
- **FS-06 requires FS-01 + FS-03 + FS-04 + FS-05**: it is the integration point — the `ConsultationIntent::IChing` branch composes a cast + lookup + biến quẻ + thể/dụng into one envelope.
- **FS-09 (Thái Tuế directional) is NOT a modification of `thai_tue.rs`**: the existing module computes personal conflict (birth-year-chi vs current-year-chi). Directional Thái Tuế is a new function — extend `thai_tue.rs` with `pub fn thai_tue_direction(year_chi_index) -> Direction8` (year chi IS the Thái Tuế direction), keep personal-conflict logic untouched.
- **FS-10 decision blocks FS-11**: until the team picks (a) reuse `sat_phuong.rs` or (b) new `tam_sat.rs`, FS-11's `entries[*].tam_sat_here` field shape is ambiguous. Recommend (b) and a new DEC entry.
- **DF-01 enhances FS-06**: the Tier-0 path of `ConsultationIntent::IChing` returns `enrichment: None` when no birth data; the Tier-2 path overlays Bazi Nhật Chủ element. Mirrors v1.5 Phi Tinh T0/T2 split exactly.
- **AF-01 conflicts with FS-01**: stalk/coin casting (Wen Wang Gua / Lục Dao) is a different tradition with a different `source_id`. Allowing it in v1.7 would either pollute `mai-hoa-dich-so` or require a third source_id. Defer.

---

## MVP Definition

### Launch With (v1.7)

Minimum viable v1.7 Kinh Dịch pillar + directional cross-link:

- [ ] **FS-02** Tiên Thiên Bát Quái map — pure static; first commit.
- [ ] **FS-07** `source_id` registration (`kinh-dich`, `mai-hoa-dich-so`) — small but blocks all downstream evidence envelopes.
- [ ] **FS-03** 64-hexagram corpus — schema-locked first (ADR-0005), then 64 records authored; biggest single deliverable.
- [ ] **FS-01** Mai Hoa casting — algorithm per §"Mai Hoa Casting Algorithm"; deterministic, no RNG.
- [ ] **FS-04** Biến quẻ derivation — pure function of primary + moving line.
- [ ] **FS-05** Thể / Dụng classification — pure function of trigrams + moving line.
- [ ] **FS-06** `ConsultationIntent::IChing` evaluator branch — integration point.
- [ ] **FS-08** `Hexagram` graph node + edges — provenance wiring.
- [ ] **FS-09** Thái Tuế directional — extends existing `thai_tue.rs`.
- [ ] **FS-10** Tam Sát directional — DECISION REQUIRED first (recommend option b: new `tam_sat.rs`).
- [ ] **FS-11** Cross-link composite view — read-only, dual source_id.
- [ ] **FS-12** Golden tests (≥10 cases, ≥2 independent sources) — framework §6 mandate.

### Add After Validation (v1.8)

- [ ] **DF-02** Full Ngũ Hành sinh khắc matrix for Thể/Dụng — LOW complexity, high UX.
- [ ] **DF-01** Tier-2 Bazi enrichment — depends on validated Tier-0 path.
- [ ] **DF-05** Pre-cast intent capture (question text in evidence) — auditability polish.

### Future Consideration (v1.9+)

- [ ] **DF-03** Hỗ Quái (nuclear hexagram) — depth technique.
- [ ] **DF-04** 24-sơn directional resolution — co-design with Tier-3 `SpatialInput` (framework §3.3).
- [ ] **AF-01** reconsidered as a separate milestone with its own `source_id` and RNG-injection interface — only if user demand materialises.

---

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority | Phase Order Hint |
|---------|------------|---------------------|----------|------------------|
| FS-07 source_id registration | MED | LOW | P1 | 1 (unblocks all evidence) |
| FS-02 trigram map | LOW (internal) | LOW | P1 | 1 |
| FS-03 64-hexagram corpus | HIGH | HIGH | P1 | 2 (schema-lock) → 3 (records) |
| FS-01 Mai Hoa casting | HIGH | HIGH | P1 | 4 |
| FS-04 biến quẻ | MED | LOW | P1 | 5 |
| FS-05 thể/dụng | MED | LOW | P1 | 5 |
| FS-06 ConsultationIntent::IChing | HIGH | MED | P1 | 6 (integration) |
| FS-08 Hexagram graph node | MED | MED | P1 | 7 |
| FS-09 Thái Tuế directional | MED | LOW | P1 | 2 (independent of FS-01) |
| FS-10 Tam Sát directional | MED | MED | P1 | 2 (after DEC) |
| FS-11 cross-link | HIGH | MED-HIGH | P1 | 8 (last; depends on FS-09 + FS-10 + huyen-khong) |
| FS-12 golden tests | MED | MED | P1 | interleaved (start after FS-01) |
| DF-02 Ngũ Hành matrix | MED | LOW | P2 | after v1.7 |
| DF-01 Bazi enrichment | MED | MED | P2 | after v1.7 |
| DF-05 intent capture | LOW | LOW | P2 | after v1.7 |
| DF-03 Hỗ Quái | LOW | MED | P3 | v1.9+ |
| DF-04 24-sơn | MED | MED-HIGH | P3 | co-design with Tier-3 |

**Priority key:**
- P1: Must have for v1.7 launch.
- P2: Should have, add in v1.8 once v1.7 is validated.
- P3: Nice to have, future consideration v1.9+.

---

## Competitor Feature Analysis

Based on framework §7 validation references + manual review of common Vietnamese almanac apps.

| Feature | Print almanacs (Cửu Tu, Hằng Phúc) | Apps (lichviet, amlich.vn, tuvi.vn) | nhantu.net (Mai Hoa reference) | Our Approach (v1.7) |
|---------|------------------------------------|------------------------------------|-------------------------------|---------------------|
| Mai Hoa casting | Static tables only | Time-of-click RNG casting (non-deterministic) | Manual walkthrough examples | **Deterministic** time-numerology per FS-01 algorithm; no RNG. |
| 64-hexagram corpus | Sparse (8–16 popular quẻ only) | Often only name + 1-line verdict | Full hào từ + thoán từ | Full 64 with Ngô Tất Tố corpus (FS-03) + cát-hùng verdict. |
| Biến quẻ | Usually omitted | Sometimes (app-dependent) | Yes, with examples | First-class (FS-04), graph-linked to chủ quẻ (FS-08). |
| Thể/Dụng analysis | Rarely | Rarely | Yes, classical | Yes (FS-05), table-driven. |
| Thái Tuế directional | Yearly static table | Year-summary screen | Mentioned | Per-year (FS-09), joined with palace (FS-11). |
| Tam Sát directional | Yearly static table (3 directions) | Year-summary, often simplified to 1 | Mentioned | New `tam_sat.rs` with classical 3-direction rule (FS-10, decision pending). |
| Cross-link with Phi Tinh | Never | Sometimes (separate screens, no join) | Never | **Differentiator**: one directional picture joining KHCBPPT warnings + huyen-khong palace layout (FS-11), read-only, dual source_id. |
| Source provenance | Implicit (book title) | Implicit / absent | Implicit | **Differentiator**: per-record `source_id` + dual-source evidence envelope (DEC-0023 discipline). |
| Bazi enrichment | Never | Sometimes (separate Tử Vi screen) | Mentioned | DF-01 if v1.8 (Tier-2 path). |

---

## Tier-0 vs Tier-2 Distinction (Carry-Forward Summary)

Per framework §2.2 and DEC-0022:

| Feature | Tier-0 (anonymous, query time only) | Tier-2 (full Bazi) |
|---------|-------------------------------------|--------------------|
| FS-01 casting | ✓ works fully | (n/a — uses query time, not birth time) |
| FS-03 64-hexagram lookup | ✓ works fully | (n/a) |
| FS-04 biến quẻ | ✓ works fully | (n/a) |
| FS-05 Thể/Dụng (trigram-level) | ✓ works fully | (n/a) |
| FS-06 `ConsultationIntent::IChing` | ✓ returns core envelope | ✓ + DF-01 enrichment overlay |
| FS-09/10/11 directional cross-link | ✓ works fully (year + palace only) | (n/a — directional is calendar-driven) |
| DF-01 Bazi enrichment | returns `enrichment: None` | ✓ overlays Nhật Chủ element on Thể/Dụng reading |

**Key invariant:** Tier-0 MUST always produce a complete, sensible answer. Tier-2 only adds an optional `enrichment` section. This mirrors v1.5 Phi Tinh discipline exactly and must be golden-tested.

---

## Sources

- **vi.wikipedia — Mai Hoa Dịch Số** (https://vi.wikipedia.org/wiki/Mai_Hoa_D%E1%BB%8Bch_s%E1%BB%91) — citing Thiệu Khang Tiết *Mai Hoa Dịch Số* (NXB Văn Hoá Thông tin 2002, trans. Văn Tùng), Thiệu Vĩ Hoa *Chu Dịch với dự đoán học* (NXB Văn Hoá 1997, trans. Mạnh Hà). **Confidence: HIGH** on casting algorithm (mod-8 / mod-6 / Tiên Thiên numbering); **MEDIUM** on edge-case tiebreaks (raw_upper % 8 == 0 handling).
- **en.wikipedia — I Ching / I Ching divination** — line number semantics (6=old yin moving, 7=young yang, 8=young yin, 9=old yang moving), hexagram structure (彖 tuàn / 爻辭 yáocí / King Wen sequence). **Confidence: HIGH** (multiple inline citations).
- **en.wikipedia — Shao Yong** — confirms authorship attribution of Mei Hua Yi numerology to Shao Yong (1011–1077,邵雍 / Thiệu Ung). **Confidence: HIGH**.
- **Project: `EXPANSION_FRAMEWORK.md` §2.2** — pillar definition, source_id assignment (`kinh-dich`, `mai-hoa-dich-so`), tier-0 baseline.
- **Project: `EXPANSION_FRAMEWORK.md` §3.2 / §3.3** — `Hexagram` graph-node slot, `rule.composite.*` discipline, CRIT-3 isolation context.
- **Project: `docs/almanac/decision-log.md` DEC-0018 / DEC-0021 / DEC-0022 / DEC-0023** — direction-family source_id discipline, Tier-0/1/2 model, `pub const` source_id rule.
- **Project: existing `crates/amlich-core/src/almanac/{thai_tue,sat_phuong,than_huong}.rs`** — confirmed: `thai_tue.rs` is personal-conflict-only (5 kinds), `sat_phuong.rs` returns single direction per chi (NOT classical 3-direction Tam Sát), `than_huong.rs` is per-Can directional deity. No `tam_sat.rs` module exists.
- **Project: existing `crates/amlich-core/src/almanac/fengshui/types.rs`** — confirmed `Palace`/`FlyingStar`/`FlyingStarLayout`/`DailyFlyingStarLayout` shapes for the cross-link join target.

### Gaps to Address in Phase-Level Research

- **Tam Sát 3-direction classical rule** needs a KHCBPPT-pinned citation before FS-10 implementation (DEC required: option a vs option b).
- **Edge-case casting tiebreaks** (raw sum mod 8 == 0, hour chi indexing convention with DEC-0017) need at least one printed-table golden case per edge.
- **Ngô Tất Tố corpus completeness** — does the source include all 64 hexagrams with both thoán từ AND all 6 hào từ, or are some sparse? This affects `HexagramRecord.tuong_truyen: Option<...>` field cardinality and may require `PendingExternalReview` markers (mirrors v1.6 RIT-14 pattern).
- **Hexagrams 1 & 2 "dụng" hào** (Wikipedia I Ching §Structure: "Hexagrams 1 and 2 have an extra line statement, named yong") — confirm whether Ngô Tất Tố includes this 7th entry; design `hao_tu: Vec<HaoTu>` to allow 7 entries only for those two.

---
*Feature research for: Vietnamese almanac — Kinh Dịch (P2) + Thái Tuế/Tam Sát ⇄ Phi Tinh cross-link (v1.7 milestone)*
*Researched: 2026-07-16*
