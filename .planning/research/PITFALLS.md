# Domain Pitfalls — v1.5 Eastern Knowledge Expansion (Văn khấn + Phi Tinh)

**Domain:** Vietnamese almanac engine adding (a) Văn khấn ritual content corpus and (b) time-based Huyền Không Phi Tinh to a system already enforcing KHCBPPT parity, source_id discipline, deterministic algorithms, and additive-only DTO extension.
**Researched:** 2026-05-23
**Overall confidence:** HIGH for system-integration pitfalls (anchored in this codebase). MEDIUM for some domain claims about Phi Tinh palace tables (cross-source verification still needed during P4 implementation).

Scope note: This document captures pitfalls **specific to adding these two pillars to this specific system**. Generic Rust/JSON advice has been omitted. Where a claim is grounded in a file or decision already in the repo, the reference is given. Where a claim is sourced from external Phi Tinh literature, confidence is downgraded and a validation hook is named.

---

## Critical Pitfalls

Mistakes here cause rewrites, evidence-graph corruption, or KHCBPPT-style "wrong answer for years" outcomes.

### CRIT-1 — Source-ID Cross-Contamination Between `vn-folk-ritual` and `vn-folk` / `khcbppt`

**What goes wrong:** A văn khấn entry (e.g., văn khấn Thổ Công) copied from a KHCBPPT-derived calendar app gets tagged `source_id: vn-folk-ritual` even though its provenance is actually KHCBPPT taboo commentary. Or a văn khấn entry translated from a Chinese ceremonial corpus gets tagged `vn-folk-ritual` when the text is not Vietnamese folk tradition at all (e.g., a literal translation of a 《禮記》 fragment).
**Why it happens:** DEC-0015/0016 mandates separate source_ids per tradition, but the văn khấn corpus has no automated validator. Authors copy text from books with mixed provenance and apply the new tag uniformly. Also: the existing `vn-folk` tag (used for Hoàng Ốc) is one character away from `vn-folk-ritual` and is easy to typo-degrade in JSON.
**Consequences:** Semantic graph cross-references degrade. Future Tier-aware reasoning rules that filter by `source_id == "vn-folk-ritual"` will leak KHCBPPT taboo text into ritual recommendations and vice versa. Once mixed, untangling requires re-reading every entry against its original source — an irreversible audit cost.
**Prevention:**
1. Lock `RitualSourceId` as a Rust enum (not free string) at corpus load time: `enum RitualSourceId { VnFolkRitual, /* future: chinese-classical, buddhist-canonical */ }` deserialized via `#[serde(rename = "vn-folk-ritual")]`. Any JSON with an unknown source_id fails deserialization — corpus cannot ship.
2. Required `original_citation` field on every văn khấn entry (book name + page or oral-tradition region). Reject entries without it at the loader (compile-time fixture test).
3. Maintain a per-entry `provenance_audit.md` ledger in `data/rituals/`. Any entry whose citation cannot be traced to a Vietnamese folk source must be quarantined under a different source_id, not silently included.
4. Add a contract test that scans `data/rituals/*.json` and asserts every entry's text contains no traditional/simplified Chinese characters above a configurable threshold (signal of Chinese-corpus origin masquerading as Vietnamese).
**Detection:** Grep audit — `rg -l '[一-龥]' data/rituals/` should return only entries that explicitly declare bilingual Sino-Vietnamese (Hán-Việt) provenance. Periodic provenance audit comparing entries against *Văn khấn cổ truyền Việt Nam* (NXB Văn Hóa Dân Tộc) reference index.
**Owning phase:** P1 Văn khấn — corpus schema phase, before any data files land.
**Confidence:** HIGH (anchored in DEC-0015/0016 + `semantic_graph/provenance.rs` design).

---

### CRIT-2 — Phi Tinh Period Boundary Off-By-One at Vận 8 → Vận 9 Transition

**What goes wrong:** Phi Tinh Vận (20-year period) boundaries align with **Lập Xuân**, not civil January 1. Vận 8 ends and Vận 9 begins at **2024-02-04 16:27 ICT** (the precise Lập Xuân instant for that year), not 2024-01-01 and not 2024-02-04 00:00. A date like 2024-01-15 belongs to **Vận 8**, not Vận 9.
**Why it happens:** Naïve implementation uses `year >= 2024 → Vận 9`. The mistake hides behind the fact that "Vận 9 = 2024-2043" is correct as a slogan but wrong as an algorithm. Same trap exists for the yearly star (Niên Tử Bạch) — the "year" is solar (Lập Xuân anchor), not Gregorian.
**Consequences:** Every January/early-February output for any year is wrong. Because the boundary is annual, ~1 in 30 days is in the danger window — wide enough to escape lightweight sampling tests but visible to any user who queries a Tết-period date.
**Prevention:**
1. Reuse the existing solar-term scanner already proven in `v1.1.2` (real Tiết Khi boundary scan that fixed nearest-term regression — DEC noted in `.planning/PROJECT.md` "Real term-boundary scan for nearest Tiet Khi"). The Lập Xuân instant for every year 2020-2043 is already deterministically resolvable. Do not re-derive it.
2. Express Vận boundaries as a sealed Rust table `(vận_number, start_lap_xuan_year)` and resolve the active Vận via a function that takes the **DaySnapshot's solar-term context**, not the Gregorian year.
3. Golden dataset must include explicit boundary cases: 2024-01-31 (Vận 8), 2024-02-04 06:00 (Vận 8 — before instant), 2024-02-04 16:27 (Vận 9 transition), 2024-02-05 (Vận 9). The instant itself is a UTC question — record the convention (Vietnam local time = UTC+7) in the data file and lock it.
4. Same rule applies to the yearly star ("Niên Tử Bạch"): the "year" for star purposes starts at Lập Xuân. Document this in the module header and assert it in tests.
**Detection:** Any test passing for "2024-01" but checking against a Vận-9 expected value is wrong. Cross-check with reference site (phongthuy.com.vn) for several Tết-period dates.
**Owning phase:** P4 Phi Tinh — algorithm phase. Must be locked before any palace table embedding.
**Confidence:** HIGH (boundary instant 2024-02-04 16:27 verified against Vietnamese Phi Tinh references; aligns with documented Lập Xuân Giáp Thìn 2024).

Sources:
- [Huyền không phi tinh vận 9 | Khải Toàn](https://phongthuykhaitoan.com/huyen-khong-phi-tinh-van-9/)
- [Cách tra Phi tinh Niên Nguyệt Nhật Thời](https://phongthuycaivan.org/cach-tra-phi-tinh-nien-nguyet-nhat-thoi/)

---

### CRIT-3 — Conflating Phi Tinh `huyen-khong` Outputs With Existing `sat_phuong.rs` / `than_huong.rs` Directional Outputs

**What goes wrong:** Both Phi Tinh and the existing direction subsystems produce direction-typed outputs. A future composer reads the merged graph and treats a Phi Tinh annual-5-Yellow direction the same as a `khcbppt` Tam Sát direction, or a Hỷ Thần direction. The user is told "avoid the South" because two unrelated subsystems agreed by accident.
**Why it happens:** Three forces conspire:
1. `sat_phuong.rs` already emits a `direction: String` (see file lines 38-43 — `SatPhuongResult { direction: "Nam" | "Đông" | ... }`). `than_huong.rs` does the same (`xuat_hanh_huong`, `tai_than`, `hy_than` as strings). String-typed directions cannot be distinguished by kind at use site.
2. The existing source_ids `khcbppt` (sat_phuong, than_huong, thai_tue) and the new `huyen-khong` (Phi Tinh) both live in the almanac evidence tree. Without explicit node-kind discrimination in the semantic graph, the only differentiator is the `source_id` string — easy to drop in downstream code.
3. Phi Tinh terminology (e.g., "Ngũ Hoàng" = 5-Yellow center star, projected to a palace direction) overlaps semantically with "killing direction" — both feel like "avoid this direction" — making a tempting (and wrong) unification target.
**Consequences:** Recommendations contradict reality. Audit becomes impossible because two completely different rule families show up under the same direction label with no kind tag. Worst case: composite rules in `interaction/direction_merge.rs` start treating Phi Tinh stars as Bát Trạch sectors, breaking DEC-0022 tier discipline (Phi Tinh stays Tier 0 in this milestone; direction_merge is Tier 1 Bát Trạch).
**Prevention:**
1. Introduce a distinct node kind: `FlyingStar { palace: Palace1to9, star_number: 1..=9, polarity: YinYang, period: Vận }` — never a bare `direction` string. The direction-from-palace projection is a separate downstream step, not the primary output.
2. Keep the file boundary explicit per Expansion Framework §2.3: `almanac/sat_phuong.rs`, `almanac/than_huong.rs`, `almanac/thai_tue.rs` stay tagged `khcbppt`. New `almanac/fengshui/flying_stars.rs` ONLY emits `source_id: "huyen-khong"`. No file may emit both.
3. Add a compile-time assertion (or contract test) that the new module's `source_id` constant is `"huyen-khong"` and that no other almanac module references that constant.
4. In `semantic_graph/provenance.rs`, the existing `ProvenanceSource::AlmanacRule` is broad enough — DO NOT subdivide it. Differentiation lives in `source_id`. But emit a `note` on Phi Tinh evidence indicating subsystem (`note: "phi_tinh.annual"` vs absent for sat_phuong) to make audit grep'able.
5. Defer any merge of Phi Tinh + sat_phuong recommendations to a future milestone with explicit DEC. Do NOT wire Phi Tinh into `direction_merge.rs` in v1.5 (this is a Tier 3 problem per §3.3, out of scope per `PROJECT.md`).
**Detection:** Code review checklist: any function returning `Vec<DirectionAdvice>` that aggregates across `source_id` boundaries needs explicit sign-off. Contract test: serialize the semantic graph for a sample date and assert that `sat_phuong` and `flying_stars` results have non-overlapping node IDs and distinct `source_id` values.
**Owning phase:** P4 Phi Tinh — module boundary phase. Cannot be retrofitted.
**Confidence:** HIGH (anchored in actual code in `sat_phuong.rs:38-43`, `than_huong.rs:20-32`, and Expansion Framework §2.3).

---

### CRIT-4 — Phi Tinh Base Palace Table Typos Are Catastrophic and Silent

**What goes wrong:** The Vận 8 and Vận 9 base palace tables (3x3 Lo Shu arrangements with the Vận star at center) are embedded as constants. A single transposition (e.g., NW palace shows 7 instead of 5 for Vận 8) silently corrupts every monthly and yearly star derived from it.
**Why it happens:**
1. There's no algorithmic generation of the base table — it's literally Lo Shu rotated by Vận. Authors transcribe by hand.
2. No KHCBPPT parity check exists (Phi Tinh is `huyen-khong`, separate source). The existing golden-dataset/validator infrastructure (KHCBPPT validators) does NOT cover this — there is no parallel reference dataset yet.
3. Unit tests authored alongside the table will test self-consistency (table → table), not external truth — same trap noted in v1.0 Phase 3 lessons learned ("Generating golden from implementation creates tautological validation").
**Consequences:** Every Phi Tinh output for the entire 20-year period is silently wrong. Users get plausible-looking palace assignments; only a domain expert cross-checking against published Phi Tinh charts will notice.
**Prevention:**
1. Treat the base palace tables for Vận 1-9 as a **separate JSON data file** (`data/almanac/flying_stars_base.json`), not Rust source constants. Validate at load time:
   - Sum of all nine palaces in a Vận = 1+2+…+9 = 45 (Lo Shu invariant). Reject any Vận whose sum is not 45.
   - Each palace 1..=9 appears exactly once per Vận.
   - The center palace equals the Vận number.
2. Establish a **Phi Tinh golden dataset** (mirror the KHCBPPT golden pattern from v1.0). Minimum 10 cross-checked dates per Vận spanning ≥2 of: phongthuy.com.vn, *Thẩm Thị Huyền Không Học* tables, fengshui.net Vận charts. Per Expansion Framework §7, divergences logged as `KnownDivergence`, not silently fixed.
3. Both `huyen-khong` and `khcbppt` golden validators run in CI. Treat Phi Tinh golden as a hard gate on P4.
4. Document the Lo Shu invariants as comments at the top of the base table file so the next maintainer cannot "fix" them.
**Detection:** Lo Shu sum check at load. Diff against published charts for at least one date in each of the 12 months of 2024 and 2025 before declaring P4 done.
**Owning phase:** P4 Phi Tinh — data phase (before algorithm), and validation phase (mirror of KHCBPPT v1.0 work).
**Confidence:** HIGH on the failure mode; MEDIUM on the recommended reference set (validation references in Expansion Framework §7 are listed but not yet exercised — confidence will rise once goldens land).

---

### CRIT-5 — Lễ Vật (Offerings) and Trình Tự (Procedure) Stored as Freeform Strings

**What goes wrong:** Văn khấn JSON entries declare `lễ_vật: "Hương, hoa, quả, oản, xôi, gà luộc"` as a single string. Downstream consumers (interaction layer, UI) cannot:
- Filter by offering type (vegetarian vs meat — important for Buddhist contexts).
- Surface "you need to prepare: X, Y, Z" as a checklist.
- De-duplicate across rituals on the same day.

**Why it happens:** Văn khấn corpus is text-heavy; structuring offerings feels like over-engineering during initial digitization. Once the freeform JSON ships, breaking the schema becomes expensive (every entry needs re-editing).
**Consequences:** The "Bản chất: Content corpus + rule mapping, KHÔNG phải reasoning thuần. Phải có metadata rõ ràng để semantic graph trích xuất" requirement (Expansion Framework §2.4) is unmet. The corpus becomes opaque to the semantic graph — search/filter only via substring grep, never semantic.
**Prevention:**
1. Schema-first: define the Rust struct before the corpus exists.
   ```rust
   pub struct Ritual {
       pub id: String,                          // e.g., "le-tao-quan-23-thang-chap"
       pub event_type: RitualEventType,         // enum: Tet, SocVong, DongTho, CuoiHoi, ...
       pub lunar_date: Option<LunarDateMatch>,  // structured, see MOD-1
       pub offerings: Vec<Offering>,            // structured, not String
       pub procedure: Vec<ProcedureStep>,       // ordered, structured
       pub khan_text: KhanText,                 // the prayer body
       pub source_id: RitualSourceId,           // enum, see CRIT-1
       pub original_citation: String,           // required
       pub vegetarian_compatible: bool,
   }
   pub struct Offering {
       pub category: OfferingCategory,          // Incense, Flower, Fruit, Cake, Meat, Liquor, ...
       pub item: String,                        // "gà luộc" / "xôi gấc"
       pub optional: bool,
   }
   ```
2. Provide a `display_string()` method on `Vec<Offering>` for backward-compat text rendering. Storage stays structured; the human string is derived.
3. JSON schema lockdown: any entry failing deserialization fails CI. No "we'll fix it later" loose entries.
**Detection:** Code review — reject PRs whose `Ritual` struct adds `String` fields where an enum or `Vec<T>` is structurally correct.
**Owning phase:** P1 Văn khấn — schema phase, before any content authoring.
**Confidence:** HIGH (anchored in Expansion Framework §2.4 metadata requirement + DEC-0015/0016 source discipline).

---

## Moderate Pitfalls

These cause reliability issues but are recoverable without a rewrite.

### MOD-1 — Lunar/Solar Date Matching Ambiguity in Văn Khấn Event Lookup

**What goes wrong:** A văn khấn declares `lunar_date: "23/12"` (lễ Táo Quân). Code does `if lunar.month == 12 && lunar.day == 23` — wrong in **leap years where tháng nhuận 12** exists (rare but real). Also fails the Tết question: is the new year prayer keyed on lunar 1/1 or solar 2/4 (Lập Xuân)?
**Why it happens:** Vietnamese ritual calendar mixes lunar dates (Sóc Vọng = 1 and 15 lunar), solar terms (Tết Đoan Ngọ = 5/5 lunar but conceptually tied to summer solstice region), and Gregorian holidays (modern hybrid). A single `lunar_date: "23/12"` string cannot express which rule applies.
**Consequences:** Lookups miss the actual ritual day, or fire on the wrong day (e.g., recommending Táo Quân prayer on the second tháng-12-nhuận instead of the canonical one).
**Prevention:**
1. Structured match type:
   ```rust
   pub enum LunarDateMatch {
       MonthDay { month: u8, day: u8, leap_month_policy: LeapPolicy },
       SolarTerm(SolarTerm),  // e.g., Lập Xuân, Đông Chí
       GregorianFixed { month: u8, day: u8 },  // modern observances only
   }
   pub enum LeapPolicy {
       CanonicalMonthOnly,   // ignore tháng nhuận
       BothMonths,           // observe in both
       LeapMonthOnly,        // rare
   }
   ```
2. Default `LeapPolicy::CanonicalMonthOnly` and require the corpus author to make the policy explicit only when known to differ.
3. Test fixture: a leap year covering tháng nhuận must be in the matching test suite.
**Detection:** Golden lookup test: assert lễ Táo Quân fires exactly once per civil year, even in lunar leap years.
**Owning phase:** P1 Văn khấn — schema phase.
**Confidence:** MEDIUM (the failure mode is real but corpus-author-dependent — depends on how many leap-month-sensitive rituals exist in the chosen reference book).

---

### MOD-2 — Monthly Phi Tinh Start Month Convention (Lập Xuân vs Civil January vs Lunar Tháng Giêng)

**What goes wrong:** Phi Tinh Nguyệt (monthly star) "starts" at one of three plausibly-correct anchors:
- Lập Xuân (early February, solar term — matches the yearly anchor)
- Lunar tháng Giêng (varies Jan-Feb Gregorian)
- Civil January

Picking the wrong anchor offsets every monthly star result by 1.
**Why it happens:** Vietnamese Phi Tinh sources are not unanimous. The classical *Thẩm Thị Huyền Không Học* uses solar-term anchoring (months = solar months bounded by 節氣, not lunar months and not civil months). Modern popular sources sometimes use lunar months for accessibility. Without explicit declaration, the implementer guesses.
**Consequences:** Monthly stars off-by-one for any date near month boundaries (every January-February stretch hits this).
**Prevention:**
1. Pick a convention explicitly, write a DEC: "v1.5 uses solar-term monthly boundaries for Phi Tinh Nguyệt, per *Thẩm Thị Huyền Không Học* §[ref]." Reuse the existing Tiết Khí scanner from v1.1.2.
2. Document the convention in `flying_stars.rs` header and in every test that depends on monthly boundaries.
3. Cross-check golden dates against ≥2 sources using the **same convention**. If two sources disagree, prefer the classical text and log a `KnownDivergence` per Expansion Framework §7.
**Detection:** Boundary golden tests at the start and end of each Tiết Khí month-pair for 2024-2025.
**Owning phase:** P4 Phi Tinh — algorithm phase, decision-bearing.
**Confidence:** HIGH on the failure-mode existence; MEDIUM on which convention is "correct" (this is itself a source choice — make the DEC explicit).

---

### MOD-3 — Niên Tử Bạch Direction (Thuận Hành vs Nghịch Hành) Inverted by Yuan

**What goes wrong:** The yearly star descends through the palaces, but the **direction of descent depends on the 三元 (Tam Nguyên) period the year belongs to**: Upper Yuan / Middle Yuan / Lower Yuan, AND on the year's polarity (dương/âm). A common naïve implementation hard-codes "descend reverse" everywhere — correct for some Yuans, wrong for others.
**Why it happens:** Vietnamese-language summaries of the rule often paraphrase "đi nghịch" without qualifying which yuan. The actual rule (from `phongthuycaivan.org/cach-tra-phi-tinh-nien-nguyet-nhat-thoi`): Thượng Nguyên starts at 1-White and descends in reverse palace order; Trung Nguyên starts at 4-Green; Hạ Nguyên starts at 7-Red. Implementers reading only one source skip the Yuan-conditional.
**Consequences:** Wrong yearly star for entire Yuan periods. Since 2024 is the start of Vận 9 within Hạ Nguyên, current-era outputs may look right while past/future predictions are wrong.
**Prevention:**
1. Encode the rule as a table keyed by **Yuan + year-polarity**, not as a single sign flag.
2. Golden dataset must span multiple Yuans (at minimum: one date in Vận 7 = Hạ Nguyên early, one in Vận 8, one in Vận 9, and ideally a year you cross-checked against a Trung Nguyên reference if you can find one — but be honest: practical use is current-era, so most goldens are Hạ Nguyên. Log this as a known coverage gap rather than pretending older periods are tested).
3. Test for both `niên dương` and `niên âm` cases within each Yuan.
**Detection:** Cross-check 2024 (Giáp Thìn — dương) and 2025 (Ất Tỵ — âm) yearly stars against published references; both must agree.
**Owning phase:** P4 Phi Tinh — algorithm phase.
**Confidence:** MEDIUM (verified that the Yuan-conditional rule exists; the exact polarity rule across all Yuans needs locking against the chosen classical reference during implementation).

Sources:
- [Cách tra Phi tinh Niên Nguyệt Nhật Thời — Khải Toàn](https://phongthuycaivan.org/cach-tra-phi-tinh-nien-nguyet-nhat-thoi/)

---

### MOD-4 — Vietnamese Diacritic Encoding Drift in Văn Khấn JSON

**What goes wrong:** Văn khấn text uses extensive Vietnamese diacritics (ă, â, ơ, ư, đ, all six tones × many vowels). Common pitfalls:
- Two Unicode normalization forms (NFC vs NFD) — same visible text, different byte sequences, breaks string equality and search.
- Editor auto-correct mangles composed-vs-decomposed characters silently.
- Pre-1975 South Vietnamese orthography for older prayers (e.g., `nầy` vs `này`, `hoà` vs `hòa` tone-position differences) leaks in inconsistently.
**Why it happens:** Authoring across multiple editors / OSes / contributors. The corpus is the only place in this codebase with large free-text Vietnamese; existing code is short identifier-strings (e.g., "Đông Nam") that don't surface this.
**Consequences:** Search "Táo Quân" returns 0 hits when text contains the same string in a different normalization form. Snapshot tests flake on machines with different default normalizations.
**Prevention:**
1. Normalize-on-load: deserialize văn khấn JSON, then NFC-normalize every text field before constructing the Rust value. Make this a single helper at the corpus loader.
2. CI lint: run `unicode-normalization` crate check across `data/rituals/*.json` files; fail if any file contains non-NFC sequences.
3. Pick one tone-position convention (modern: `hòa`, `tòa`) and document in `data/rituals/README.md`. Migrate old-orthography entries explicitly.
4. For prayers containing Hán-Việt or Chinese characters (some classical văn khấn cite 偈 / phrases), declare them in a separate `hannom_text` field, never inline in the main `khan_text` body unless it's the intended display.
**Detection:** Round-trip test: load JSON → normalize → re-serialize → assert byte-equal to a re-canonicalized fixture.
**Owning phase:** P1 Văn khấn — loader phase.
**Confidence:** HIGH (Unicode normalization is a well-known issue for any large Vietnamese text corpus).

---

### MOD-5 — Evidence Metadata Holes on Phi Tinh Aggregate Outputs

**What goes wrong:** A Phi Tinh result for date D includes:
- Vận star (period)
- Niên star (yearly)
- Nguyệt star (monthly)
- (later: Nhật star, Thời star)

The aggregate carries one evidence envelope tagged `huyen-khong`. The individual sub-stars are not separately attributed. Downstream graph traversal cannot distinguish "yearly star contributed by X" vs "monthly star contributed by Y" — both are flattened into one envelope.
**Why it happens:** `ReasoningEvidenceEnvelope` is a single struct; the easy implementation attaches one envelope per aggregate. The richer per-sub-star evidence requires either a `Vec<Envelope>` or a structured `composite` envelope per the §3.2 `rule.composite.*` convention.
**Consequences:** Audit trail is incomplete. Cannot answer "which classical table sourced this yearly star?" — the answer is hidden behind the aggregate. Reduces the value of the semantic graph as an explainability tool.
**Prevention:**
1. Per sub-star: attach `Provenance::almanac_rule("huyen-khong", "vận")`, `..."niên"`, `..."nguyệt"` with distinct `method` strings. The aggregate node holds a separate `rule.composite.flying_stars` envelope per §3.2.
2. Mirror the existing pattern in `provenance.rs::from_rule_evidence` — `RuleEvidence` already supports per-rule attribution; replicate that granularity for Phi Tinh.
3. Contract test: serialize a Phi Tinh result and assert `evidence.notes.len() >= n_sub_stars` and that `source_id` is consistently `huyen-khong` on primitive entries, `rule.composite.*` on aggregates.
**Detection:** Snapshot test on the evidence chain for a representative date — manually inspect that every sub-claim is independently attributed.
**Owning phase:** P4 Phi Tinh — integration phase.
**Confidence:** HIGH (anchored in §3.2 of Expansion Framework + actual `provenance.rs:65,77` patterns).

---

### MOD-6 — Backward-Compat Break: New Fields on Shared DTOs Not Marked `Option<T>`

**What goes wrong:** Adding `flying_stars: FlyingStarsResult` (non-optional) to `DaySnapshot` or `DayFortune`. Any existing JSON consumer that deserializes the old schema breaks; any serialized snapshot fixture stored from v1.4 fails to round-trip.
**Why it happens:** Project policy is "additive-only" (`PROJECT.md` Key Decision: "Additive-only integration changes — confirmed in v1.2"). v1.2 set the precedent with Ten Gods/Kua as `Option<T>`. New contributors may forget the convention.
**Consequences:** Downstream consumers (the desktop app workspaces — e.g., `Personal Lab`, `Season Timeline`, `Almanac Inspector` per recent commits) fail to load older saved data or break their snapshot tests.
**Prevention:**
1. All new fields added to existing public DTOs in v1.5 must be `Option<T>` with `#[serde(default, skip_serializing_if = "Option::is_none")]`.
2. Round-trip contract test: load a v1.4 serialized fixture into the v1.5 struct, then re-serialize. Assert no unexpected fields appear in the output for the old data.
3. Code-review checklist item: any modification of `DayFortune`, `DaySnapshot`, `PersonalRecommendation` etc. requires `Option<T>` justification or DEC.
**Detection:** CI test: deserialize all `tests/fixtures/v1.4-*.json` against the v1.5 struct.
**Owning phase:** P1 and P4 — integration phases.
**Confidence:** HIGH (policy precedent from v1.2; risk is forgetting, not the policy itself).

---

## Minor Pitfalls

### MIN-1 — JSON Schema Drift Across Văn Khấn Files

**What goes wrong:** Two files in `data/rituals/` use slightly different field names (e.g., one has `event_type`, another has `eventType` or `loai_su_kien`). Deserialization works for one, fails for the other, or silently uses default values.
**Prevention:** Single `Ritual` struct deserialized by `serde` with `#[serde(deny_unknown_fields)]`. CI loads every file at startup and fails on any deserialization error. Forbids drift at the file-system boundary.
**Owning phase:** P1 — loader phase.
**Confidence:** HIGH.

---

### MIN-2 — Confusion of Phi Tinh "Palace" (Cung) With Bazi "Cung" (Mệnh Cung, Etc.)

**What goes wrong:** Both Phi Tinh and Bazi/Tử Vi use the word "Cung." A reader of the codebase grepping for "cung" finds matches across unrelated subsystems. A struct field `cung: u8` in Phi Tinh code might be conflated with a `cung` in Bazi code by a future contributor.
**Prevention:** Use `Palace` (English) or `LoShuPalace` for the Phi Tinh 3x3 grid concept. Reserve `cung` for the Vietnamese display strings; never for internal field names that span subsystems. Same discipline that gave us `chi_index: usize` instead of bare `chi`.
**Owning phase:** P4 — naming, design phase.
**Confidence:** HIGH (style consistency issue).

---

### MIN-3 — Văn Khấn Title vs Body Mixing Causes Display Bugs

**What goes wrong:** Some văn khấn entries put the title (e.g., "VĂN KHẤN TÁO QUÂN") as the first line of the `khan_text`. Downstream UI renders it twice (once as header, once in body).
**Prevention:** Schema separates `title` from `khan_text` body. Loader-time lint: reject entries where the first line of `khan_text` matches the `title` (case-insensitive, normalized).
**Owning phase:** P1 — loader phase.
**Confidence:** MEDIUM (depends on corpus authoring habits).

---

### MIN-4 — Phi Tinh Polarity (Star Yin/Yang) Dropped in Display, Lost in Reasoning

**What goes wrong:** Each flying star has a polarity (5-Yellow is famously inauspicious; 8-White famously auspicious; some stars are conditionally so). Implementer stores just the number and drops the auspiciousness signal. Downstream cannot answer "is this an avoid-day?"
**Prevention:** Star struct carries `polarity: StarPolarity { Auspicious, Inauspicious, Mixed { conditions: String } }` alongside the number. Auspiciousness is data, not derived at the UI.
**Owning phase:** P4 — data design phase.
**Confidence:** MEDIUM.

---

### MIN-5 — Tests for Văn Khấn Lookup Use Hard-Coded Gregorian Dates Without Lunar Context

**What goes wrong:** A test asserts "on 2026-02-09, recommend lễ Táo Quân" with hard-coded Gregorian. The intent was "23 tháng Chạp," which shifts year to year. Test passes for the year it was written, becomes silently wrong in maintenance review years later.
**Prevention:** Write lookup tests as `for each year in [2024..2030]: compute lunar 23/12 → assert lookup returns lễ Táo Quân`. Hard-coded Gregorian only for cross-validation pinning, never as the assertion.
**Owning phase:** P1 — test phase.
**Confidence:** HIGH.

---

## Phase-Specific Warnings

These map pitfalls to roadmap phases for the roadmapper.

| Phase Topic | Likely Pitfalls | Mitigation Owners |
|---|---|---|
| P1.1 Văn khấn schema design | CRIT-1, CRIT-5, MOD-1, MIN-1, MIN-3 | Lock schema + enums before any data. Loader uses `deny_unknown_fields`. Required `original_citation`. |
| P1.2 Văn khấn corpus authoring | CRIT-1, MOD-4 | Provenance audit per entry. NFC normalize at load. CI grep guard for Han characters. |
| P1.3 Văn khấn lookup API | MOD-1, MIN-5 | Structured `LunarDateMatch`. Leap-month fixture. Year-parametrized tests. |
| P1.4 Văn khấn DTO integration | MOD-5, MOD-6 | All new fields `Option<T>`. Round-trip with v1.4 fixtures. Per-entry evidence envelopes. |
| P4.1 Phi Tinh base tables + data | CRIT-4, MIN-4 | JSON-not-Rust for tables. Lo Shu invariant load-check. Polarity field present. |
| P4.2 Phi Tinh boundary scanner | CRIT-2, MOD-2 | Reuse v1.1.2 Tiết Khi scanner. DEC for monthly anchor convention. Golden cases at Lập Xuân instants. |
| P4.3 Phi Tinh algorithm (Vận/Niên/Nguyệt) | CRIT-2, CRIT-4, MOD-3 | Yuan + polarity table for niên descent direction. Golden against ≥2 references per Expansion Framework §7. |
| P4.4 Phi Tinh graph/DTO integration | CRIT-3, MOD-5, MOD-6 | Distinct `FlyingStar` node kind. No bare-direction outputs. `huyen-khong` source_id strictly siloed from `khcbppt`. NOT wired into `direction_merge.rs` this milestone. |
| Validation gate (parallels v1.0 work) | CRIT-4, MOD-3 | Phi Tinh golden dataset created mirroring KHCBPPT golden methodology. Văn khấn provenance audit ledger. |

---

## Validation Strategy Summary (Gap Note from Question)

The question flagged "no canonical software to cross-check against — what's the validation strategy?" This is the most important system-level pitfall and deserves its own treatment.

**For Phi Tinh:** There is no single canonical implementation. The framework's §7 names three reference points: `fengshui.net`, `phongthuyhomemy.com`, and *Thẩm Thị Huyền Không Học* book tables. Strategy:
1. Build a Phi Tinh golden dataset (mirror v1.0 KHCBPPT pattern): 10+ dates per Vận, cross-checked against ≥2 of the §7 references.
2. Treat the **classical text** (*Thẩm Thị Huyền Không Học*) as the tiebreaker when modern sites disagree, and log every disagreement as a `KnownDivergence` per Expansion Framework §7 — do not silently pick a winner.
3. Algorithmic correctness gates: Lo Shu invariants (sum=45, each 1-9 once, center=Vận). These hold regardless of source choice — make them load-time assertions.
4. Cover at least one date per month of 2024 and 2025 (active Vận 9 period — practical use range, mirrors the v1.0 2020-2030 scope).

**For Văn khấn:** There is no algorithm to validate — the validation is **provenance + corpus integrity**, not computation. Strategy:
1. Per-entry citation to *Văn khấn cổ truyền Việt Nam* (NXB Văn Hóa Dân Tộc) or other named Vietnamese folk reference. Entries without citation fail CI.
2. Loader contract tests: schema enforcement, NFC normalization, Han-character guard, leap-month policy presence.
3. Lookup contract tests: parametrized over multiple years, structured `LunarDateMatch`, no hard-coded Gregorian assertions.
4. Independent reviewer audit before each batch of entries lands — single-author corpora are higher risk than algorithmic outputs because there's no math to catch errors.

**Cross-cutting:** Both pillars produce outputs that flow into the same `DayFortune` / `DaySnapshot` DTOs. The round-trip test with v1.4 fixtures (per MOD-6) is the gate that catches backward-compat regressions for both pillars simultaneously.

---

## Sources

- `.planning/PROJECT.md` — current state, decision history, additive-only policy.
- `.planning/research/EXPANSION_FRAMEWORK.md` — §2.3 (Phi Tinh boundary with existing modules), §2.4 (Văn khấn nature), §3.1 (source provenance), §3.2 (composite envelopes), §5 (sequencing), §7 (validation references).
- `crates/amlich-core/src/almanac/sat_phuong.rs` — existing direction subsystem (`khcbppt` source_id, bare-string direction output — anti-pattern to avoid replicating in Phi Tinh).
- `crates/amlich-core/src/almanac/than_huong.rs` — second existing direction subsystem to differentiate from.
- `crates/amlich-core/src/semantic_graph/provenance.rs` — `ProvenanceEntry` constructors, `from_rule_evidence` pattern for granular attribution.
- `.planning/MILESTONES.md` — v1.0 lessons learned ("Self-consistent golden dataset" tautology trap; "Set-based comparison"); v1.1.2 real Tiết Khi boundary scan precedent; v1.2 additive Option<T> pattern.
- [Huyền không phi tinh vận 9 — Khải Toàn](https://phongthuykhaitoan.com/huyen-khong-phi-tinh-van-9/) — Vận 8→9 boundary at 2024-02-04 16:27 ICT.
- [Cách tra Phi tinh Niên Nguyệt Nhật Thời](https://phongthuycaivan.org/cach-tra-phi-tinh-nien-nguyet-nhat-thoi/) — Yuan-conditional direction rule for Niên Tử Bạch.
- [HUYỀN KHÔNG PHI TINH NĂM GIÁP THÌN 2024](https://vacationtravel.com.vn/tin-chi-tiet/phi-tinh-nam-giap-thin-2024) — 2024 reference chart.
