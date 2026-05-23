# Feature Research — v1.5 Eastern Knowledge Expansion

**Domain:** Two NEW pillars on existing Vietnamese almanac engine
- **P1 Văn khấn cổ truyền** — `source_id: vn-folk-ritual`, Tier 0, content corpus + lookup
- **P4 Phi Tinh thời gian** — `source_id: huyen-khong`, Tier 0, algorithm-driven (Vận/Năm/Tháng)

**Researched:** 2026-05-23
**Confidence:** MEDIUM-HIGH

Scope boundary: This milestone adds Tier 0 surfaces only. Spatial Phi Tinh (Tier 3, requires `facing_direction`) is explicitly P5 and deferred per EXPANSION_FRAMEWORK §3.3 and PROJECT.md.

---

## Feature Landscape

Features are grouped by pillar and category. Each row maps cleanly to a single REQ-ID for downstream requirements.

### P1 Văn khấn — Table Stakes (Rituals Corpus)

These define the JSON data model and minimum content coverage that any Vietnamese almanac app shipping `văn khấn` is expected to provide.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| `Ritual` record schema with full prayer text | Every `lichviet`/`lichvansu` app surfaces the exact words to recite | LOW | Fields: `id`, `event_type`, `title_vi`, `title_en`, `body_vi` (multi-paragraph), `source_id: "vn-folk-ritual"`, `source_citation`. Multi-paragraph plain text; no markup beyond newlines. **Confidence: HIGH** |
| `event_type` taxonomy (enum) | App must trigger correct prayer per occasion; free-text would break lookups | LOW | Closed enum: `SocVong`, `TetNguyenDan`, `TetTrungThu`, `TetThanhMinh`, `TetDoanNgo`, `TetVuLan`, `TetTaoQuan`, `RamThangBay`, `DongTho`, `NhapTrach`, `KhaiTruong`, `Cuoi` (with sub: `DamNgo`/`AnHoi`/`DonDau`), `Gio`, `DayThang`, `ThuongTho`, `CaiTang`, `ThanTai`, `GiaTien`, `ThoCong`. Maps 1:1 to existing `holiday_data.category`. **Confidence: HIGH** |
| `lễ vật` (offerings) checklist per ritual | Users physically prepare offerings; the words alone are insufficient | LOW | Array of strings (e.g., `["hương", "hoa tươi", "trầu cau", "rượu trắng", "mâm ngũ quả"]`). Plain checklist; no quantities/prices. **Confidence: HIGH** |
| `trình tự` (procedure) steps | Ritual ordering matters religiously (e.g., light incense BEFORE reciting) | LOW | Ordered array of step strings. Optional but expected for major events (Động thổ, Nhập trạch, Cưới). **Confidence: HIGH** |
| Source attribution per record | Required by DEC-0015/0016 (source_id discipline) | LOW | Each JSON row carries `source_id`, `source_book`, `source_page` (when known). Validator rejects missing `source_id`. **Confidence: HIGH** |
| Coverage of Sóc/Vọng (Mùng 1, Rằm) for all 12 months | These trigger 24x/year — highest-frequency use case | LOW | Two ritual records (`soc-vong-mung-1`, `soc-vong-ram`) reused across all 12 months; no per-month variant needed. Generic `Cúng Gia Tiên` body. **Confidence: HIGH** |
| Coverage of 8 major lunar festivals | Already detected by `holidays.rs` `isMajor=true` set | LOW | One ritual per: Tết Nguyên Đán, Tết Khai Hạ (mùng 7), Rằm tháng Giêng (Thượng Nguyên), Thanh Minh, Đoan Ngọ, Vu Lan, Trung Thu, Ông Công Ông Táo (23/12 ÂL). **Confidence: HIGH** |
| Coverage of 6+ life-event rituals | Users search by life event, not date | MEDIUM | Động thổ, Nhập trạch, Khai trương, Cưới (3 sub-events), Giỗ (gia tiên), Đầy tháng. **Confidence: HIGH** |

### P1 Văn khấn — Table Stakes (Lookup API)

API surfaces that any consumer (desktop app, mobile, CLI) requires.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Lookup by `event_type` | Primary use case: "I need the Khai trương prayer" | LOW | `rituals::get_by_event(EventType::KhaiTruong) -> Vec<&Ritual>`. Returns all matching (e.g., Khai trương may have multiple variants). **Confidence: HIGH** |
| Lookup by lunar date (Mùng 1 / Rằm trigger) | Calendar UI shows "today's prayer" without knowing event semantics | LOW | `rituals::get_for_lunar_date(lunar_day, lunar_month) -> Vec<&Ritual>`. Returns Sóc/Vọng when day=1/15, lunar festivals when date matches `lunar-festivals.json` entry. **Confidence: HIGH** |
| Lookup by JD via `DaySnapshot` integration | Calendar drilldown surfaces from existing date pipeline | LOW | `rituals::get_for_day(snapshot: &DaySnapshot) -> Vec<&Ritual>`. Reuses lunar date + holiday detection already in `holidays.rs`. **Confidence: HIGH** |
| Category filter | Users browse by intent (worship vs. life event vs. seasonal) | LOW | Category coarser than `event_type`: `Worship`, `Seasonal`, `LifeEvent`, `Business`. `rituals::list_by_category(cat)`. **Confidence: HIGH** |
| Stable IDs | Cross-app deep links and bookmarks | LOW | kebab-case (`soc-vong-mung-1`, `dong-tho-lam-nha`). Documented in schema. **Confidence: HIGH** |
| List-all (corpus enumeration) | Test/audit and UI "browse all prayers" | LOW | `rituals::all() -> &[Ritual]`. **Confidence: HIGH** |

### P1 Văn khấn — Differentiators

Features competitive Vietnamese calendar apps differentiate on. Optional for MVP but high UX leverage.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| English/bilingual `body_en` translations | International diaspora users (US, AU Vietnamese communities) | MEDIUM | Mark prayer body as `body: Localized { vi, en: Option<String> }`. v1.5 ships `vi` only; schema reserves `en`. **Confidence: HIGH** |
| Variant rituals per event (e.g., Khai trương đơn giản vs. đầy đủ) | Different households have different ritual depth | LOW | Multiple `Ritual` records share an `event_type`, distinguished by `variant: "simple"\|"full"\|"buddhist"\|"folk"`. **Confidence: MEDIUM** |
| Cross-link to triggering holiday | "View prayer" button on holiday detail | LOW | Holiday JSON entries gain optional `ritual_ids: ["<id>", ...]`. **Confidence: HIGH** |
| Auspicious-hour pairing | Suggest hours from existing `hoang-dao` for ritual performance | LOW | Read-only join — no new compute. Belongs in reasoning layer, not rituals module. **Confidence: HIGH** |
| Search across prayer body | Full-text find ("tìm bài có chữ X") | MEDIUM | Out of scope for amlich-core; consumers (desktop UI) can index. Document as non-goal. **Confidence: HIGH** |

### P1 Văn khấn — Anti-Features

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| AI-generated / auto-personalized prayer text | "Make it match my family's name automatically" | Violates source provenance (DEC-0015); the corpus IS the truth, paraphrasing breaks canonicity; risk of doctrinal errors | Provide template placeholders (`{tín chủ họ tên}`) the consumer UI fills in; never alter text |
| Audio recordings of prayers | "Easier than reading" | Out of scope for amlich-core (text-only library); copyright on liturgical recordings is murky | Surface text + IPA-ish reading guide for diaspora; let consumer apps optionally bundle audio |
| Per-user prayer history / journaling | "Track what I prayed for" | User-state belongs in app layer, not engine; storage/sync explosion | Engine remains stateless; consumer apps own user data |
| Editable corpus from user input | "Add my family's variation" | Source corpus must remain authoritative and validatable; user-edits poison golden tests | Document a community contribution pathway via PR to `data/rituals/*.json` |
| Spatial direction recommendations in ritual API | "Which way should I face during cúng?" | Belongs to Bát Trạch / Phi Tinh, not văn khấn corpus; coupling them blurs source_id | Consumer composes — call `direction_merge` separately if user requests |

---

### P4 Phi Tinh thời gian — Table Stakes (Period Layer / Vận)

The fundamental data layer. No spatial input; pure time-based table lookup.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Period (Vận) determination from year | Every Phi Tinh consumer needs to know current Vận | LOW | Vận 8: 2004-2023; Vận 9: 2024-2043 (20-year period; 9 periods × 20yr = 180-year cycle). `flying_stars::period_for_year(year) -> Period`. **Confidence: HIGH** |
| Vận 8 + Vận 9 base charts | This milestone spans both — historical data (2004-2023) for backtest/UX continuity AND current period | LOW | Two static 9-cell grids. Vận 9 center = 9 (Cửu Tử/Fire). Vận 8 center = 8 (Bát Bạch/Earth). Each cell carries a star number 1-9. **Confidence: HIGH** |
| Period star metadata (element, polarity, auspice) | Downstream interpretations need element + polarity for resonance | LOW | Each of 9 stars: `number`, `name` (Nhất Bạch…Cửu Tử), `element` (Thủy/Mộc/Mộc/Mộc-Kim/Thổ/Kim/Kim/Thổ/Hỏa for 1..9), `auspice` (cát/hung/trung-tính), `palace_color`. Static reference table. **Confidence: HIGH** |

### P4 Phi Tinh thời gian — Table Stakes (Annual Layer / Lưu Niên)

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Annual center star (`lưu niên trung cung tinh`) | Headline output every Vietnamese app prints in January | LOW | Formula (sources cite as canonical): center = ((11 - digit_sum(year)) mod 9), with 0→9. Verify against known: 2024→3, 2025→2, 2026→1. **Confidence: HIGH** |
| Full 9-palace annual grid | Standard published output across all references | LOW | Once center is known, populate 8 remaining palaces by Yang sequence (Center→NW→W→NE→S→N→SW→E→SE) using ascending star numbers mod 9. Deterministic. **Confidence: HIGH** |
| Palace→Direction mapping | Reader needs "what's in the South this year?" | LOW | Fixed Lạc Thư bagua: N=1, NE=8, E=3, SE=4, S=9, SW=2, W=7, NW=6, Center=5. Static. **Confidence: HIGH** |
| Star+Palace auspice interpretation | Reader expects "cát/hung" annotation per palace | MEDIUM | Per-star inherent auspice (Nhất Bạch=cát, Nhị Hắc=hung, Tam Bích=hung, Tứ Lục=cát, Ngũ Hoàng=đại hung, Lục Bạch=cát, Thất Xích=hung, Bát Bạch=đại cát, Cửu Tử=cát in Vận 9). Surface as static field on star metadata, not derived. **Confidence: MEDIUM** |
| Year input via solar year (Gregorian) | Phi Tinh year starts at Lập Xuân in classical practice; users supply Gregorian | LOW | Accept `i32` solar year. Document: year boundary = Lập Xuân (~Feb 4). Edge dates in Jan/early Feb need solar-term check. Reuse existing `tietkhi` module. **Confidence: HIGH** |

### P4 Phi Tinh thời gian — Table Stakes (Monthly Layer / Lưu Nguyệt)

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Monthly center star (`lưu nguyệt trung cung tinh`) | Standard fine-grained output (every published almanac includes monthly) | MEDIUM | Rule: Year-branch group determines starting star in lunar month 1; subsequent months count DOWN (reverse). Three groups: {Tý, Ngọ, Mão, Dậu} start at Bát Bạch (8); {Thìn, Tuất, Sửu, Mùi} start at Ngũ Hoàng (5); {Dần, Thân, Tỵ, Hợi} start at Nhị Hắc (2). Month N center = ((start - (N-1) - 1) mod 9) + 1. **Confidence: HIGH (multiple sources concur)** |
| Full 9-palace monthly grid | Same expectation as annual | LOW | Same fill rule as annual once center is known. **Confidence: HIGH** |
| Month input via lunar-month boundaries | Phi Tinh month uses Tiết Khi boundaries (Tiết, not Nguyệt) | MEDIUM | Month boundary = mid-month solar terms (Lập Xuân, Kinh Trập…). Use existing tietkhi data; document explicitly. **Confidence: MEDIUM — boundary semantics vary by school; need ADR.** |

### P4 Phi Tinh thời gian — Differentiators

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Combined Annual + Monthly overlay grid | Power users want both stars per palace ("year over month") at once | MEDIUM | Output a struct with both star numbers per palace + aspect (e.g., `palace.year=1, palace.month=8 → 1-8 combo`). Major differentiator vs. apps that show only year. **Confidence: HIGH** |
| Combo aspect interpretations (2-star combinations) | Classical texts catalog 81 combinations (9×9); each has named significance | HIGH | Static lookup table (e.g., "1-6 → Văn Xương — học tập, thi cử"). 81 entries from Thẩm Thị. Maps `(year_star, month_star)` → `CombinationAspect`. **Confidence: MEDIUM — corpus exists, digitization effort is real** |
| Star avoidance flags ("kiêng kỵ") | Ngũ Hoàng / Nhị Hắc landing in important palaces drives user action | LOW | Surface `is_danger_palace: bool` and `recommended_cure: ElementHint` (e.g., Ngũ Hoàng → metal cure). Static rules from Thẩm Thị. **Confidence: HIGH** |
| `DaySnapshot` integration field | Calendar drill-down shows year+month Phi Tinh inline | LOW | Additive field `flying_stars: Option<FlyingStarsSummary>`. Backward-compatible per established v1.x pattern. **Confidence: HIGH** |
| Cross-link to Thái Tuế / Tam Sát directional warnings | Existing direction signals + Phi Tinh in same view = full directional picture | LOW | No new compute — reasoning layer joins. Document boundary: Thái Tuế stays in `almanac/thai_tue.rs` (`source_id: khcbppt`); Phi Tinh stays in `almanac/fengshui/flying_stars.rs` (`source_id: huyen-khong`). Distinct citations even when both touch "Đông" direction. **Confidence: HIGH** |

### P4 Phi Tinh thời gian — Anti-Features

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| House-facing input / Sơn-Hướng natal chart | "I want my actual house's stars, not just the year" | This is Tier 3 (spatial) per §3.3; requires `Direction24`, `SpatialInput`, room subdivision — entire Tier-3 design unfinished | Explicitly defer to P5; if API receives spatial fields in v1.5, return `Unsupported` per DEC-0022 |
| Daily / Hourly Phi Tinh (`Lưu Nhật`, `Lưu Thời`) | Some advanced practitioners want day-grain | Boundary semantics (solar-term vs. midnight) more ambiguous at day grain; corpus reliability lower; very few apps ship it | Document as future (post-v1.5); requirements may include it as a `MAY` not `SHALL` |
| Personalized Phi Tinh recommendations ("which palace is YOUR best") | "Tell me where to sit/sleep" | Requires user's Kua + house facing (Tier 2 + Tier 3); blurs into Bát Trạch territory | Keep generic — surface star-per-palace facts; let `interaction/spatial_compose.rs` (P5) handle personalization |
| Visual SVG/PNG grid rendering | "Show the chart" | Rendering is presentation; engine returns data | Return a stable `FlyingStarsGrid` struct; consumers (desktop app) render |
| "Cures" / remedy product suggestions | Mainstream feng-shui apps push commercial cures | Commercial / cultural risk; no canonical source justifies specific product recommendations | Surface only the classical element hint (e.g., "use Metal element"); never product names |
| Automatic Vận transition warnings around 2024 boundary | "Alert me when period changes" | Stateful, time-based — belongs in consumer app layer | Engine is pure function of year; caller computes diffs |

---

## Feature Dependencies

```
P1 Văn khấn
├── Ritual schema (data) ──> required by ──> All P1 lookup APIs
├── event_type enum ──> required by ──> get_by_event, get_for_lunar_date
├── holidays.rs (EXISTING) ──> triggers ──> get_for_lunar_date, get_for_day
├── DaySnapshot (EXISTING) ──> consumed by ──> get_for_day
└── Cross-link to holidays ──> ENHANCES ──> existing holiday_data.json (additive)

P4 Phi Tinh thời gian
├── Period (Vận) table (data) ──> required by ──> Annual + Monthly layers
├── 9-star metadata (data) ──> required by ──> All P4 output enrichment
├── Lạc Thư palace-direction map (static) ──> required by ──> All grid outputs
├── tietkhi module (EXISTING) ──> required by ──> year/month boundary semantics
├── Annual center formula ──> required by ──> Annual grid, Combined overlay
├── Monthly center formula ──> required by ──> Monthly grid, Combined overlay
├── Annual + Monthly grids ──> required by ──> Combined overlay (differentiator)
└── Combined overlay ──> ENHANCES ──> 2-star combination aspects (differentiator)

CROSS-PILLAR (no dependency between P1 and P4 — they can ship in parallel)
```

### Dependency Notes

- **P1 corpus precedes P1 lookup APIs:** Schema and JSON content must be in place before any `rituals::*` function can be tested.
- **P1 leans on existing `holidays.rs` / `lunar.rs`:** Zero changes required to existing code beyond an additive `ritual_ids: Option<Vec<String>>` field in holiday JSON entries.
- **P4 layers ordered:** Period → Annual → Monthly → Combined. Each subsequent layer reuses prior structs. Combined aspects (81-cell table) are last and OPTIONAL.
- **P4 borders existing direction modules:** `thai_tue.rs`, `sat_phuong.rs`, `than_huong.rs`, `phuc_than.rs` stay in `almanac/` under `source_id: khcbppt`. NEW module `almanac/fengshui/flying_stars.rs` under `source_id: huyen-khong`. Distinct citations even when describing the same compass direction. **Critical boundary** (per EXPANSION_FRAMEWORK §2.3).
- **No P1↔P4 dependency:** Văn khấn never reads Phi Tinh and vice versa. They are independent feature streams within the same milestone.

---

## MVP Definition

### Launch With (v1.5 must-ship)

P1 Văn khấn — **minimum viable corpus + lookup**:
- [ ] `Ritual` struct + JSON schema (frontmatter validated)
- [ ] `event_type` closed enum covering at least: SocVong, lunar festival set (8 entries), DongTho, NhapTrach, KhaiTruong, Cuoi, Gio, DayThang
- [ ] Corpus content: ≥ 20 ritual records (2 Sóc/Vọng + 8 festivals + 10 life events)
- [ ] `rituals::get_by_event`, `get_for_lunar_date`, `get_for_day`, `all`, `list_by_category`
- [ ] Source attribution per record validated by golden test
- [ ] Additive integration: holiday JSON entries gain optional `ritual_ids`

P4 Phi Tinh — **minimum viable time-based chart**:
- [ ] `Period` (Vận 8, Vận 9) determination from year
- [ ] 9-star metadata table (name, element, polarity, auspice)
- [ ] Annual center star + full 9-palace grid (formula-driven, golden-tested for 2020-2030)
- [ ] Monthly center star + full 9-palace grid (golden-tested for at least 24 month-points)
- [ ] Palace→Direction static mapping
- [ ] `DaySnapshot.flying_stars: Option<FlyingStarsSummary>` (year + month, no combined yet)
- [ ] Year/month boundary documented (Lập Xuân = year start; tiết = month start) and ADR'd

### Add After Validation (v1.5.x patches if scope permits)

- [ ] Combined annual+monthly overlay grid (differentiator) — DEFER if 2-star combo corpus not ready
- [ ] 81-cell combination aspect table (Văn Xương, Bát Bạch + Lục Bạch etc.) — DEFER pending Thẩm Thị digitization
- [ ] Ritual variants (simple/full/Buddhist/folk) — DEFER until user feedback requests
- [ ] Bilingual `body_en` translations — schema reserves field; content deferred

### Future Consideration (v1.6+ / later milestones)

- [ ] Spatial Phi Tinh (Tier 3) — explicit P5 in framework
- [ ] Daily / Hourly Phi Tinh (Lưu Nhật / Lưu Thời) — boundary semantics need ADR
- [ ] Star avoidance + cure recommendations — needs DEC for commercial/cultural posture
- [ ] Audio prayer recordings — consumer responsibility, not engine

---

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
|---------|------------|---------------------|----------|
| Ritual schema + 20-record corpus | HIGH | LOW | P1 |
| event_type enum + lookup APIs | HIGH | LOW | P1 |
| Sóc/Vọng coverage (24× per year hit rate) | HIGH | LOW | P1 |
| Major festival coverage (8 entries) | HIGH | LOW | P1 |
| Life-event rituals (Động thổ, Nhập trạch, etc.) | HIGH | MEDIUM | P1 |
| Holiday→ritual_id cross-link | MEDIUM | LOW | P1 |
| Period (Vận 8/9) + 9-star metadata | HIGH | LOW | P1 |
| Annual 9-palace grid | HIGH | LOW | P1 |
| Monthly 9-palace grid | HIGH | MEDIUM | P1 |
| Palace↔Direction mapping | HIGH | LOW | P1 |
| DaySnapshot.flying_stars integration | HIGH | LOW | P1 |
| Combined annual+monthly overlay | MEDIUM | MEDIUM | P2 |
| 81-cell combination aspects | MEDIUM | HIGH | P2 |
| Star avoidance flags + element cures | MEDIUM | LOW | P2 |
| Ritual variants (simple/full) | LOW | LOW | P3 |
| Bilingual prayer translations | LOW | MEDIUM | P3 |
| Spatial Phi Tinh (Tier 3) | HIGH | HIGH | DEFER (P5) |
| Daily/Hourly Phi Tinh | LOW | MEDIUM | DEFER |

**Priority key:**
- **P1:** Must have for v1.5 launch (MVP)
- **P2:** Should have, add if scope permits within v1.5 timebox
- **P3:** Nice to have, schedule into v1.5.x patches
- **DEFER:** Explicit non-goal; document in `out_of_scope.md`

---

## Competitor / Reference Feature Analysis

| Feature | Lịch Vạn Niên (mobile) | Lịch Ngày Tốt | phongthuy.com.vn | Our Approach (amlich v1.5) |
|---------|-----------------------|---------------|------------------|---------------------------|
| Văn khấn corpus | 100+ prayers categorized | Full year prayers | N/A | 20+ prayers, closed enum event_type, golden-tested source attribution |
| Văn khấn surfacing | Daily card + "thư viện" | By holiday detail page | N/A | API-driven (`get_for_day` + `get_by_event`), consumer renders |
| Annual Phi Tinh grid | Yes, static yearly card | Yes, with palace meanings | Yes, deep analysis | Algorithm-driven; verifiable against Thẩm Thị reference |
| Monthly Phi Tinh | Rare in mobile apps | Yes | Yes | Algorithm-driven; reuses tietkhi for boundaries |
| Combined annual+monthly | No | Sometimes | Yes (premium) | Differentiator — schedule for P2 |
| 81 star-combo interpretations | No | Limited | Yes (premium) | Differentiator — schedule for P2; corpus from Thẩm Thị |
| Spatial Phi Tinh (Sơn-Hướng) | No | No | Yes | OUT OF SCOPE v1.5 (Tier 3, deferred to P5) |
| Source attribution | None | Marketing copy | Vague | Per-record `source_id` + book + page (DEC-0015 discipline) |

**Strategic positioning:** amlich does not compete on UI polish or content volume. It competes on **verifiability** (golden tests against named sources) and **API cleanliness** (consumer-renderable structs, additive integration with existing pipeline). The differentiator vs. mobile apps is "engine that can be embedded in a verifiable agent / desktop app", not "prettiest card".

---

## Module / File Mapping (for downstream requirements)

| Feature cluster | Target location | Existing code touched |
|-----------------|-----------------|----------------------|
| P1 ritual schema + corpus | `data/rituals/*.json`, `data/schemas/ritual.schema.json` | none (additive) |
| P1 lookup APIs | `crates/amlich-core/src/rituals/{mod.rs, lookup.rs, types.rs}` | none — new sibling to `almanac/` |
| P1 ↔ holidays cross-link | `data/holidays/lunar-festivals.json` (add `ritualIds`), `data/holidays/solar-holidays.json` | additive field; no code change required |
| P1 DaySnapshot integration | `crates/amlich-core/src/almanac/recommendation/` or `lib.rs` aggregator | optional `rituals: Option<Vec<&Ritual>>` field |
| P4 Period + star metadata | `crates/amlich-core/src/almanac/fengshui/{mod.rs, period.rs, stars.rs}` | new submodule under `almanac/` |
| P4 Annual grid | `crates/amlich-core/src/almanac/fengshui/flying_stars.rs::annual_chart()` | reuses `tietkhi::Lập Xuân` for year boundary |
| P4 Monthly grid | same file, `monthly_chart()` | reuses `tietkhi` for month boundaries |
| P4 DaySnapshot integration | `lib.rs` aggregator | additive `flying_stars: Option<FlyingStarsSummary>` |
| P4 Data | `data/almanac/flying_stars.json` (Vận tables + star metadata) | new file |

---

## Confidence Assessment Per Feature Block

| Block | Confidence | Reason |
|-------|------------|--------|
| Ritual schema + lookup APIs | HIGH | Standard CRUD-style content corpus; many reference apps |
| Event type taxonomy | HIGH | Vietnamese ritual taxonomy is well-documented across folk sources |
| Phi Tinh Period + 9-star metadata | HIGH | Static, canonical Thẩm Thị table; widely published |
| Phi Tinh annual formula | HIGH | Multiple Vietnamese and English sources agree (digit-sum + reverse mod 9). Verified 2024→3, 2025→2, 2026→1 against published charts |
| Phi Tinh monthly formula | HIGH | Year-branch-group rule (8/5/2 starting stars + reverse count) attested in 3+ sources |
| Phi Tinh month boundary (tiết vs. trung khí) | MEDIUM | Schools differ; needs ADR. Recommend tiết-based (节, mid-month transitions like Lập Xuân) |
| Combined overlay + 81 combinations | MEDIUM | Differentiator only; corpus exists but digitization effort underestimated until samples surveyed |
| Cross-link discipline (P4 vs. existing direction modules) | HIGH | Already explicit in EXPANSION_FRAMEWORK §2.3; just enforcement |

---

## Sources

Vietnamese ritual corpus and event taxonomy:
- [Tổng hợp các bài văn khấn đầy đủ trong năm 2026 — chuabavang.com](https://chuabavang.com/tong-hop-van-khan-ca-nam-d3852.html)
- [Văn Khấn Cổ Truyền Việt Nam — SachHayOnline.com](https://www.sachhayonline.com/tua-sach/van-khan-co-truyen-viet-nam)
- [Bài văn khấn Động thổ làm nhà 2026 chuẩn Thọ Mai Gia Lễ — luatminhkhue.vn](https://luatminhkhue.vn/bai-van-khan-dong-tho-lam-nha-chuan-tho-mai-gia-le.aspx)
- [12 Bài Văn Khấn Thần Tài Thổ Địa — tuhuyen.com](https://tuhuyen.com/van-khan-than-tai-tho-dia/)
- [Văn khấn xin gia tiên cầu lộc, cầu con, cưới hỏi, nhập trạch — tuhuyen.com](https://tuhuyen.com/van-khan-xin-gia-tien/)
- [Lịch Vạn Niên 2026 & Lịch Việt (App Store)](https://apps.apple.com/us/app/l%E1%BB%8Bch-v%E1%BA%A1n-ni%C3%AAn-2026-l%E1%BB%8Bch-vi%E1%BB%87t/id1071624317)
- [Lịch Vạn Sự — Lich Ngay Tot (App Store)](https://apps.apple.com/vn/app/lich-ngay-tot-lich-van-su/id791061378)

Phi Tinh algorithms and reference charts:
- [Cửu cung phi tinh năm 2026 Bính Ngọ — phongthuydathanh.com](https://www.phongthuydathanh.com/tin-tuc/cuu-cung-phi-tinh-2026-nam-binh-ngo.html) — confirms 2026 center = Nhất Bạch
- [CỬU CUNG PHI TINH 2026 — lichngaytot.com](https://lichngaytot.com/phong-thuy/cuu-cung-phi-tinh-2026-284-231904.html)
- [Cách tính Cửu cung phi tinh theo năm, tháng, ngày, giờ — lykhi.com](https://lykhi.com/cach-tinh-cuu-cung-phi-tinh-theo-nam-thang-ngay-gio/)
- [Cách tính Cửu cung phi tinh theo năm, tháng, ngày, giờ — lichngaytot.com](https://lichngaytot.com/phong-thuy/cach-tinh-cuu-cung-phi-tinh-284-216988.html)
- [Cách tra Phi tinh Niên Nguyệt Nhật Thời — phongthuycaivan.org](https://phongthuycaivan.org/cach-tra-phi-tinh-nien-nguyet-nhat-thoi/)
- [Huyền không phi tinh vận 9 2024-2043 — phongthuycaivan.org](https://phongthuycaivan.org/huyen-khong-phi-tinh-van-9-2024-2043/)
- [Lưu Nguyệt Phi Tinh là gì — phongthuyvietnam.com](http://www.phongthuyvietnam.com/2017/04/02/luu-nguyet-phi-tinh-la-gi/)
- [Yearly and Monthly Flying Star Charts (Master Class Lesson 14) — Feng Shui DIY](https://fengshuidiy.com/yearly-and-monthly-flying-star-charts-flying-star-sequence-master-class-lesson-14/)
- [Flying Star Feng Shui System + 2026 Annual Chart Guide — uniquefengshui.com](https://uniquefengshui.com/understanding-flying-star-feng-shui/)

Internal references (already in repo):
- `.planning/research/EXPANSION_FRAMEWORK.md` §2.3 (Phi Tinh), §2.4 (Văn khấn), §3.1 (provenance), §3.3 (Tier 3 deferral)
- `crates/amlich-core/src/holidays.rs` (event detection — văn khấn trigger source)
- `crates/amlich-core/src/almanac/than_huong.rs`, `thai_tue.rs`, `sat_phuong.rs`, `phuc_than.rs` (existing direction modules — boundary with new Phi Tinh module)
- `data/holidays/lunar-festivals.json`, `solar-holidays.json` (event corpus that triggers văn khấn lookup)
- DEC-0015 / 0016 (source_id discipline), DEC-0022 (Tier model)

---
*Feature research for: v1.5 Eastern Knowledge Expansion (P1 Văn khấn + P4 Phi Tinh time-based)*
*Researched: 2026-05-23*
