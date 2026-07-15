# Domain Pitfalls — v1.7 Kinh Dịch (Mai Hoa Dịch Số) + Thái Tuế/Tam Sát ⇄ Phi Tinh Cross-link

**Domain:** Vietnamese almanac engine adding (a) a P2 Kinh Dịch pillar — Mai Hoa Dịch Số casting + 64-hexagram lookup + biến quẻ derivation — to `reasoning/personal.rs` via a new `ConsultationIntent::IChing` branch, and (b) a read-only reasoning-layer join between the existing `khcbppt` Thái Tuế/Tam Sát directional warnings and the `huyen-khong` Phi Tinh palace layout.
**Researched:** 2026-07-16
**Overall confidence:** HIGH for integration pitfalls (anchored in this codebase and carry-forward of v1.5/v1.6 lessons). MEDIUM for Mai Hoa casting-micro-rule claims (classical and well-known, but the exact Vietnamese-language convention used by Thiệu Khang Tiết's *Mai Hoa Dịch Số* — especially remainder-zero handling — must be re-verified against the project's chosen reference text during P-KD-1 schema-lock; flagged inline).

Scope note: This document captures pitfalls **specific to adding Kinh Dịch / Mai Hoa Dịch Số and the Thái Tuế ⇄ Phi Tinh cross-link to this specific system**. Carry-forwards from v1.5 (`PITFALLS.md` prior version, now archived in `PROJECT.md` Key Decisions) are explicitly cross-referenced as `(carry: v1.5 CRIT-N / MOD-N)` so the reader can see which lessons are being re-applied and which are net-new to the divination domain.

Project conventions re-applied here without further justification (see `PROJECT.md` Key Decisions):
- `pub const SOURCE_*: &str` in `sources.rs` + `tests/source_id_guard.rs` literal guard (DEC-0023).
- Schema-lock phase precedes corpus authoring phase (v1.5 CRIT-1, confirmed via Phase 10 → 12 ordering).
- Additive `Option<T>` + `skip_serializing_if` on shared DTOs (`DaySnapshot`, reasoning envelopes).
- CRIT-3 isolation: `FlyingStar` is never wired into `interaction/direction_merge.rs` until a future Tier-3 `spatial_compose` module lands.
- NFC normalization at corpus load; `deny_unknown_fields` on corpus JSON schemas.
- Golden methodology: ≥2 independent sources, classical tiebreaker, divergence logged as `KnownDivergence` (NOT silently corrected) per Expansion Framework §7 and v1.6 `DeferralMarker` schema.

---

## Critical Pitfalls

Mistakes here cause (a) every divination output to be subtly wrong (domain), (b) source-graph corruption that violates the Tier boundaries v1.5/v1.6 enforced (integration), or (c) corpus re-authoring at 64× the cost of v1.5's 60 văn khấn entries.

### CRIT-1 — Schema-Lock the I-Ching Types BEFORE Authoring the 64-Hexagram Corpus (carry: v1.5 CRIT-1, CRIT-5)

**What goes wrong:** Implementers start authoring the 64-hexagram text corpus (`thoán từ` + 6×`hào từ` per hexagram = 7 fields × 64 = **448 text fields**, plus metadata) before the Rust types are frozen. A late-discovered schema need (e.g., wanting to separate "Tượng Truyện" 象傳 commentary from the main text, or adding a per-hào cát/hung tag) forces re-editing all 64 entries. This is the same trap as v1.5's văn khấn corpus, but at ~7× the entry count.
**Why it happens:** Domain pressure to "show a working demo of one hexagram" tempts contributors to land a thin schema + one entry, then bulk-author later. By the time the schema gap surfaces, the corpus is too large to migrate cheaply. Project history (`PROJECT.md` Key Decision: "Schema-lock before corpus authoring") already calls this out — but it must be re-asserted per milestone because every new pillar rediscovers the temptation.
**Consequences:**
1. Re-editing 448 text fields across 64 JSON files is a multi-day mechanical slog with high regression risk (NFC, Hán-Việt phrasing, source citation).
2. If the corpus ships under a thin schema and the schema is later tightened with `deny_unknown_fields`, every old entry fails CI simultaneously.
3. Source citations get lost in the migration — a CRIT-6 (source-id conflation) accelerant.
**Prevention:**
1. Land all I-Ching Rust types FIRST in a dedicated schema-lock phase (suggested: `P-KD-1`), with `#[derive(Deserialize)]` + `#[serde(deny_unknown_fields)]` on every corpus struct. Land ONE entry only as the schema probe.
2. ADR-equivalent required: lock the 64-hexagram JSON schema version (mirror ADR-0001 for `RitualEntry`) and the Mai Hoa casting convention (mirror ADR-0002 for monthly anchor). Without these, the corpus target moves under authors.
3. Required fields per hexagram entry: `id` (King Wen number 1-64), `name_vi` (e.g., "Thuần Kiền"), `name_hantu` (乾), `thoan_tu` (hexagram statement), `hao_tu` (array of exactly 6 line statements), `source_id` (must resolve to `SOURCE_KINH_DICH`), `original_citation` (Ngô Tất Tố book + page), `reviewer` (mirror v1.6 RIT-11 discipline). All present, all typed.
4. Loader-side assertions: hexagram count == 64, every King Wen number 1..=64 appears exactly once, every `hao_tu` has length 6. Reject on violation.
**Detection:** Loader test `iching_invariants.rs` (mirror `tests/fengshui_invariants.rs`) gating corpus load. CI fails if any of the 64 entries is malformed or absent. Periodic diff against the canonical King Wen hexagram index (1=Thuần Kiền … 64=Hỏa Thủy Vị Tế).
**Phase to address:** `P-KD-1` — Schema-lock phase, before any corpus file lands. **Cannot be retrofitted.**
**Confidence:** HIGH (anchored in v1.5 CRIT-1 lesson + PROJECT.md "Schema-lock before corpus authoring" Key Decision).

---

### CRIT-2 — Mai Hoa Casting `% 8` / `% 6` Remainder-Zero Convention (Domain-Specific Off-By-One)

**What goes wrong:** The Mai Hoa time-number method computes:
- Lower trigram (hạ quái): `(lunar_month + lunar_day) % 8`
- Upper trigram (thượng quái): `(lunar_month + lunar_day + lunar_hour) % 8`
- Động hào (moving line): `(lunar_month + lunar_day + lunar_hour) % 6`

The classical convention is that **remainder 0 maps to the highest value** (8 for trigram, 6 for động hào), NOT to 0. A naïve Rust implementation using `let trigram_idx = sum % 8;` produces index 0 for the boundary case, which maps to Tiên Thiên number 1 (Kiền) instead of the correct Tiên Thiên number 8 (Khôn). Every "all-eights" casting (e.g., month 8 + day 8 + hour 8 = 24; 24 % 8 == 0) is silently wrong.
**Why it happens:** The rule "dư 0 thì lấy số 8" is a classical Mai Hoa convention documented in Vietnamese-language divination manuals but rarely stated in English references. Programmers reaching for `% 8` instinctively produce a 0-indexed result. Compounding this: lunar hour (giờ Tý=1 .. giờ Hợi=12) is 1-indexed in classical texts, while Rust arrays are 0-indexed — a second silent off-by-one source.
**Consequences:** Every casting that lands exactly on a remainder-zero boundary yields a wrong trigram. With 8 trigrams × 8 trigrams = 64 possible castings, statistically 1/8 of all castings hit at least one remainder-zero boundary in the trigram step; 1/6 hit it in the động hào step. The bug surfaces in production with non-trivial frequency but is invisible to unit tests that don't include boundary cases.
**Prevention:**
1. Lock the convention in an ADR (suggested: ADR-0005) BEFORE writing the casting function: *"Mai Hoa Dịch Số casting: `% 8` remainder 0 → Tiên Thiên number 8 (Khôn). `% 6` remainder 0 → động hào 6 (top line). Inputs: lunar month 1-12, lunar day 1-30, lunar hour 1-12 (giờ Tý=1, giờ Hợi=12). Reference: Thiệu Khang Tiết, Mai Hoa Dịch Số, [exact page]."*
2. Implement as a named helper that returns a typed `TienThienTrigram` (1..=8) and a typed `MovingLine` (1..=6) — never a raw `u8`. The helper is the only place the modulo lives.
   ```rust
   pub fn tien_thien_trigram(sum: u32) -> TienThienTrigram {
       // Classical Mai Hoa convention: remainder 0 → 8
       let n = if sum % 8 == 0 { 8 } else { sum % 8 };
       TienThienTrigram::from_tien_thien_number(n) // 1=Kiền .. 8=Khôn
   }
   ```
3. Golden dataset MUST include the boundary cases: month=8/day=8/hour=8 (lower=8=Khôn, upper=8=Khôn, động hào=6), month=2/day=6/hour=10 (24, all-boundary), month=1/day=1/hour=1 (lower=2, upper=3, động hào=3 — non-boundary sanity).
4. Two-source golden per Expansion Framework §7 (nhantu.net for Mai Hoa casting reference; cross-check with a second Vietnamese Mai Hoa site). Any divergence logged as `KnownDivergence` with classical tiebreaker (Thiệu Khang Tiết).
**Detection:** Boundary unit tests are the only reliable detector. CI contract test: enumerate all 12×30×12 = 4320 casting inputs over a representative year and assert the result distribution has no anomalous clusters at boundary inputs.
**Phase to address:** `P-KD-1` (ADR) + `P-KD-2` (casting algorithm) — the ADR must land before the function; the boundary tests must land in the same plan as the function.
**Confidence:** HIGH on the failure-mode existence and the `% == 0 → 8` classical rule (well-documented in Mai Hoa manuals); MEDIUM on the specific input convention (1-indexed lunar hour from Tý) — re-verify against the project's chosen reference text during P-KD-1.

---

### CRIT-3 — Conflating Tiên Thiên Trigram Numbers (Casting Output) With King Wen Hexagram Numbers (Text Lookup) — THE Mai Hoa vs King Wen Trap

**What goes wrong:** The project uses **two distinct numerical systems** that share the surface form "1-8" / "1-64" but have completely different mappings:

| System | Range | Used For | Example: number 1 |
|--------|-------|----------|-------------------|
| **Tiên Thiên (Phục Hy / Earlier Heaven) trigram numbers** | 1-8 | Mai Hoa casting — maps the modulo result to a trigram | 1 = Kiền (乾) |
| **King Wen (Văn Vương / Wen Wang) hexagram sequence** | 1-64 | Hexagram text lookup in Ngô Tất Tố's *Kinh Dịch Trọn Bộ* | 1 = Thuần Kiền (乾) — coincidentally same name, different concept |

A casting produces a (lower-trigram, upper-trigram) pair, each identified by a Tiên Thiên number 1-8. The implementer must **compose the two trigrams into a hexagram** (upper over lower) and then look up that hexagram in the 64-entry King Wen table. The trap: a developer looks at the casting output, sees "upper=1, lower=1," and naively looks up King Wen hexagram #1, getting lucky once (Thuần Kiền is both Tiên Thiên 1/1 AND King Wen #1). But Tiên Thiên (3,3) (Ly over Ly) is King Wen #30 (Ly/Vision), not King Wen #3. Every non-trivial casting silently returns the WRONG hexagram text while looking correct in a "did the math, returned a number, found an entry" sense.

**Why it happens:**
1. The classical literature casually calls both "quẻ số N" — "quẻ 1" might mean Tiên Thiên trigram 1 in a casting chapter, or King Wen hexagram 1 in a text chapter. The ambiguity is invisible until you implement it.
2. Mai Hoa tutorials often present the casting result as a single composed hexagram with a name ("Thuần Kiền") without showing the intermediate trigram-composition step. Implementers skip the composition and cache a Tiên Thiên-pair → King Wen-number lookup that they hand-build from a single example.
3. There is a third system lurking: **Hậu Thiên (Lo Shu / Later Heaven) trigram arrangement**, used for some Mai Hoa derivatives and for Phi Tinh palace numbering. Three overlapping "1-8" systems in one codebase = guaranteed confusion.
**Consequences:** Every casting returns a plausible hexagram name + a plausible text, but for the majority of inputs the text is for the WRONG hexagram. Users receive "cát/hung" judgments that have no relationship to their actual casting. The bug is invisible to "did it return a string?" tests; only golden-dataset cross-check against a known casting (e.g., the famous Thiệu Khang Tiết "two sparrows" 梅花數 example cast) reveals it.
**Prevention:**
1. **Explicitly flag in PROJECT.md which system the project uses for each step** (the question requests this):
   - **Casting algorithm**: Mai Hoa Dịch Số per Thiệu Khang Tiết, using **Tiên Thiên (Phục Hy) trigram numbers** (1=Kiền, 2=Đoài, 3=Ly, 4=Chấn, 5=Tốn, 6=Khảm, 7=Cấn, 8=Khôn).
   - **Hexagram text corpus**: Ngô Tất Tố translation, ordered by **King Wen (Văn Vương) sequence** (1=Thuần Kiền … 64=Hỏa Thủy Vị Tế).
   - **Composition function**: a typed `compose_hexagram(upper: TienThienTrigram, lower: TienThienTrigram) -> KingWenHexagram` that lives in ONE place and is exhaustively tested against a 64-entry lookup table.
2. The 64-entry composition table MUST be its own constant (not derived), validated at load: every King Wen number 1..=64 is produced by exactly one (upper, lower) pair; the table has 64 entries; pairs (Kiền/Kiền) → King Wen #1, (Khôn/Khôn) → King Wen #2, etc. — match the classical 八卦 相盪 table.
3. Encode the three systems as **three distinct types** that do NOT implicitly convert:
   ```rust
   pub struct TienThienTrigram(u8);   // 1..=8
   pub struct HauThienTrigram(u8);    // 1..=8, used by Phi Tinh palaces
   pub struct KingWenHexagram(u8);    // 1..=64
   ```
   No `From` impls between them. Composition is the ONLY path from `(TienThienTrigram, TienThienTrigram) → KingWenHexagram`.
4. The Phi Tinh module already uses `Palace` for the Lo Shu grid concept (v1.5 MIN-2 lesson). Re-apply: do NOT reuse `Palace` for the trigram-position concept in IChing; use `Trigram` or `TienThienTrigram`.
5. Golden test: the canonical Thiệu Khang Tiết "two sparrows / 麻雀" cast (a known historical Mai Hoa reading with a published result) must produce the documented hexagram pair (chủ quẻ + biến quẻ).
**Detection:** Snapshot a casting → assert the King Wen number via the composition table. Cross-check at least 10 historical Mai Hoa casts from nhantu.net against the function output. ANY mismatch is a CRIT-3 bug.
**Phase to address:** `P-KD-1` (types + composition table + ADR) → `P-KD-2` (casting algorithm). **Cannot be retrofitted** — the type boundary must exist before corpus authoring references King Wen numbers.
**Confidence:** HIGH on the failure-mode existence and the system distinction (canonical I-Ching fact). MEDIUM on the exact Tiên Thiên number assignment (1=Kiền…8=Khôn) — this is the dominant convention but at least one Vietnamese sub-school uses a different arrangement; re-verify against the project's chosen reference.

---

### CRIT-4 — Biến Quẻ (Transforming Hexagram) Derivation Bugs From Wrong Động Hào Position

**What goes wrong:** Mai Hoa produces exactly ONE động hào (moving line, 1-6, bottom to top). The biến quẻ (之卦, transformed hexagram) is the original hexagram with that single line FLIPPED (yin↔yang). Common bugs:
1. **Hào numbering inverted**: classical hào are numbered 1 (bottom/初) to 6 (top/上). A developer using `lines[0]` for "hào 1" is correct, but a developer reading "hào 6" as the 6th-from-bottom (i.e., `lines[5]`) vs 6th-from-top (`lines[0]`) inverts the flip position.
2. **Wrong trigram flipped**: a different (NON-Mai-Hoa) tradition flips multiple lines and produces both a biến quẻ AND a hỗ quẻ (互卦, nuclear hexagram). Mai Hoa proper does NOT use biến quẻ from multiple moving lines for its primary reading — but it DOES derive the hỗ quẻ from lines 2-3-4 (lower) and 3-4-5 (upper) of the original. Conflating biến quẻ and hỗ quẻ derivation corrupts the consultation.
3. **Bit-position bug**: hexagrams are 6 bits; flipping "line 6" should XOR the high bit, not the low. Implementers who pack the hexagram as `u8` with bits[0]=bottom vs bits[5]=bottom produce mirrored biến quẻ.
**Why it happens:** Three competing line-ordering conventions exist in classical and modern sources (bottom-up classical, top-down diagram-reading, left-to-right binary-packing). Vietnamese tutorials rarely state which they use. The biến quẻ derivation looks like "trivial bit flip" so it gets the least review attention despite being the second-most-critical output (it determines the cát/hung-over-time reading).
**Consequences:** The biến quẻ is wrong but the chủ quẻ (original) is right — every "static" consultation looks correct, every "what happens next" answer is wrong. The bug only manifests when the user reads the biến quẻ text, which is often the most actionable part of the consultation.
**Prevention:**
1. Encode hào position as a typed enum, not a raw index:
   ```rust
   #[repr(u8)]
   pub enum HaoPosition { Chu = 1, Er = 2, San = 3, Si = 4, Wu = 5, Shang = 6 }
   // lines[HaoPosition::Chu as usize - 1] is hào 1 (bottom)
   ```
2. The biến quẻ derivation function is the ONLY place a line is flipped. Its signature takes `(original: Hexagram, dong_hao: HaoPosition) -> Hexagram` and lives in one file.
3. Golden test: all 6 động hào positions for a known chủ quẻ (e.g., Thuần Kiền → 6 different biến quẻ: Kiền→Khôn if hào 1 moves is wrong; the correct 6 biến quẻ of Kiền are Cấu, Độn, Phợ, Quan, Bạt, Kiền-by-no-move; verify against 八卦變 chart).
4. Lock the hỗ quẻ derivation rule in the same ADR as biến quẻ (CRIT-2's suggested ADR-0005): *"hỗ quẻ derived from original hexagram lines 2-3-4 (lower trigram) + 3-4-5 (upper trigram), per Mai Hoa standard."* Document whether the project surfaces hỗ quẻ at all in v1.7 (default: NOT — defer to a future "deep Mai Hoa interpretation" milestone; surface only chủ + biến).
**Detection:** Exhaustive contract test: for each of 64 chủ quẻ × 6 động hào positions = 384 cases, assert the biến quẻ matches the published 八卦變 table. This is a finite, enumerable, golden-able space — there is no excuse for shipping without 384 cases.
**Phase to address:** `P-KD-2` (casting algorithm phase; biến quẻ derivation lands with casting, not separately).
**Confidence:** HIGH on the failure-mode existence; HIGH on the 384-case enumeration being the correct gate; MEDIUM on the hỗ quẻ line-range convention (2-3-4 / 3-4-5 is the dominant convention but verify against Thiệu Khang Tiết's text).

---

### CRIT-5 — Thái Tuế / Tam Sát ⇄ Phi Tinh Cross-link Collapses CRIT-3 Isolation (carry: v1.5 CRIT-3)

**What goes wrong:** The cross-link joins two directional traditions into one "directional picture":
- **KHCBPPT Thái Tuế / Tam Sát** (Earthly-Branch-derived annual directional taboos; `source_id: khcbppt`; lives in `almanac/thai_tue.rs:36-41` and `almanac/sat_phuong.rs:38-43`).
- **Huyền Không Phi Tinh** (palace-layout spatial Feng Shui; `source_id: huyen-khong`; lives in `almanac/fengshui/` per v1.5 mod.rs header note).

The trap: a future composer reads the cross-linked envelope and treats a KHCBPPT Tam Sát direction (e.g., "hướng Dần sat phương Bắc") the same as a Huyền Không annual 5-Yellow palace direction (e.g., "Ngũ Hoàng tại cung Trung"). Both feel like "avoid this direction" — but they are **different rule families with different temporal/spatial scopes**, and merging them into a unified `Direction` node violates the v1.5 CRIT-3 isolation that has been the project's most-repeated integration rule ("`FlyingStar` is never wired into `interaction/direction_merge.rs`" — `PROJECT.md` Out of Scope, `fengshui/mod.rs:10-11`).
**Why it happens:** Three forces, identical to v1.5 CRIT-3 but now with a new "join" feature explicitly inviting the merge:
1. Both subsystems already emit direction-typed outputs (`sat_phuong.rs:38-43`'s `direction: String`, `than_huong.rs` similarly; Phi Tinh emits `Palace` per v1.5 MIN-2 discipline). String-typed directions cannot be distinguished at use site.
2. The cross-link's stated purpose is to "surface both in one directional picture" — the easiest implementation is a `Vec<DirectionAdvice>` that aggregates across `source_id` boundaries. This is exactly the failure mode v1.5 CRIT-3 warned against.
3. Phi Tinh terminology (Ngũ Hoàng = 5-Yellow) overlaps semantically with "killing direction" — making a tempting unification.
**Consequences:**
1. The reasoning envelope reports "avoid the South" because two unrelated subsystems happened to agree — the agreement is coincidence, not signal.
2. CRIT-3 isolation (v1.5 audit-verified by grep) is silently violated. The next audit must re-verify, and the violation may have spread by then.
3. DEC-0022 tier discipline degrades: Thái Tuế/Tam Sát is Tier-0 (date-derived); Phi Tinh annual overlay is Tier-0 in v1.5 scope but Tier-3 (`spatial_compose`) when merged with personal facing. The cross-link is permissibly Tier-0 ONLY because it's read-only; any derivation that combines them promotes the result to Tier-3 with no `SpatialInput`.
**Prevention:**
1. **The cross-link is READ-ONLY and LIVES ONLY in `reasoning/`** (suggested: `reasoning/directional_composite.rs` or extend `reasoning/personal.rs`). The existing `almanac/thai_tue.rs`, `almanac/sat_phuong.rs`, and `almanac/fengshui/` source files MUST NOT be modified to reference each other. Add a CI grep guard mirroring `tests/source_id_guard.rs` that asserts `almanac/fengshui/` files do NOT import `almanac::thai_tue` / `almanac::sat_phuong`, and vice versa.
2. The composite envelope MUST emit `source_id: "rule.composite.directional_cross_link"` (per Expansion Framework §3.2 convention for multi-pillar rules) AND list each contributing primitive evidence separately. Never collapse the two source_ids into one. Each primitive retains its original `source_id` (`khcbppt` or `huyen-khong`).
3. Node-kind discipline: introduce (or reuse) distinct node kinds. Do NOT emit a unified `DirectionAdvice { direction: String, why: String }`. Emit `DirectionalCrossLinkNode { khcbppt_signals: Vec<KhcbpptDirectionSignal>, huyen_khong_signals: Vec<FlyingStarSignal>, summary: String }` — keeping the two families structurally disjoint at the type level.
4. The composite envelope explicitly does NOT derive a "net recommend/avoid" — it SURFACES both traditions' outputs. Any "merged recommendation" requires explicit DEC and a Tier-3 `SpatialInput` (out of scope for v1.7).
5. Add to `tests/source_id_guard.rs` (or a sibling guard file) an assertion that the cross-link module's evidence envelope carries EXACTLY TWO primitive source_ids (`khcbppt` and `huyen-khong`) plus one composite (`rule.composite.directional_cross_link`) — no other source_id may appear.
**Detection:** Periodic grep audit (mirror v1.5 audit's CRIT-3 grep check): `rg 'fengshui|FlyingStar' crates/amlich-core/src/almanac/thai_tue.rs crates/amlich-core/src/almanac/sat_phuong.rs crates/amlich-core/src/interaction/direction_merge.rs` returns zero hits. Snapshot test of the cross-link envelope for a sample date: assert both source_ids appear, neither is collapsed.
**Phase to address:** `P-TT-1` (Thái Tuế cross-link phase) — module-boundary + envelope-shape decisions land first, algorithm second. **Cannot be retrofitted.**
**Confidence:** HIGH (anchored in `almanac/sat_phuong.rs:38-43`, `almanac/thai_tue.rs:36-41`, `almanac/fengshui/mod.rs:10-11`, and v1.5 CRIT-3 / PROJECT.md "Out of Scope" entry on `spatial_compose`).

---

### CRIT-6 — Source-ID Cross-Contamination Between `kinh-dich` and `mai-hoa-dich-so` (carry: v1.5 CRIT-1)

**What goes wrong:** The v1.7 milestone introduces TWO new source_ids (per Expansion Framework §2.2 and PROJECT.md v1.7 target):
- `kinh-dich` — Ngô Tất Tố, *Kinh Dịch Trọn Bộ* — for the 64-hexagram text corpus (thoán từ, hào từ).
- `mai-hoa-dich-so` — Thiệu Khang Tiết — for the casting algorithm (Tiên Thiên trigram assignment, động hào derivation).

A single `ConsultationIntent::IChing` consultation uses BOTH: the casting step is `mai-hoa-dich-so`; the text lookup step is `kinh-dich`. The trap: an implementer tags the whole consultation envelope with whichever source_id they thought of first, usually `kinh-dich` because that's the book they transliterated. Result: every casting-step audit trail is mis-attributed to the wrong tradition. The casting rule's provenance ("is this Thiệu Khang Tiết's exact modulo rule?") becomes unanswerable.
**Why it happens:**
1. The two source_ids share the conceptual umbrella "Kinh Dịch" in Vietnamese casual speech — "thuật Kinh Dịch" can refer to either. The semantic overlap is greater than between, say, `vn-folk-ritual` and `khcbppt`, which feels obviously distinct.
2. `tests/source_id_guard.rs:13-21` currently has 7 entries; adding `"kinh-dich"` and `"mai-hoa-dich-so"` brings it to 9. The guard prevents bare literals in `src/`, but the contamination often happens in the corpus JSON itself (data-side), which the guard does not cover. v1.5 CRIT-1 made the same observation about văn khấn JSON.
3. Composite reasoning envelopes (§3.2) tend to default to a single dominant source_id; without explicit per-step attribution the casting/text split collapses.
**Consequences:** Same as v1.5 CRIT-1: graph cross-references degrade, future Tier-aware rules that filter by `source_id == "mai-hoa-dich-so"` will silently miss all casting evidence; the `kinh-dich` corpus audit trail becomes contaminated with algorithmic claims it never made. Untangling is an irreversible audit cost.
**Prevention:**
1. Register both constants in `crates/amlich-core/src/sources.rs` before any casting code lands:
   ```rust
   /// Kinh Dịch Trọn Bộ — Ngô Tất Tố, 64-hexagram text corpus (new in v1.7).
   pub const SOURCE_KINH_DICH: &str = "kinh-dich";
   /// Mai Hoa Dịch Số — Thiệu Khang Tiết, casting algorithm (new in v1.7).
   pub const SOURCE_MAI_HOA_DICH_SO: &str = "mai-hoa-dich-so";
   ```
2. Extend `tests/source_id_guard.rs:13-21` `FORBIDDEN_LITERALS` with `"kinh-dich"` and `"mai-hoa-dich-so"`. CI fails on any bare literal outside `sources.rs`.
3. The casting envelope and the text envelope MUST be separate `ReasoningEvidenceEnvelope` instances per `reasoning/types.rs:144-151`. Pattern:
   ```rust
   let casting_evidence = ReasoningEvidenceEnvelope {
       source_family: ReasoningEvidenceSourceFamily::AlmanacRule,
       source_id: SOURCE_MAI_HOA_DICH_SO.to_string(),
       method: "tien_thien_modulo_cast".to_string(),
       note: Some(format!("month={} day={} hour={} → ({},{}) động {}",
                          month, day, hour, upper, lower, dong_hao)),
   };
   let text_evidence = ReasoningEvidenceEnvelope {
       source_family: ReasoningEvidenceSourceFamily::AlmanacRule,
       source_id: SOURCE_KINH_DICH.to_string(),
       method: "ngo_tat_so_lookup".to_string(),
       note: Some(format!("KingWen#{} → {}", king_wen_no, hexagram.name_vi)),
   };
   // Composite (per Expansion Framework §3.2):
   let composite = ReasoningEvidenceEnvelope {
       source_family: ReasoningEvidenceSourceFamily::Derived,
       source_id: "rule.composite.iching_consultation".to_string(),
       method: "v1.compose".to_string(),
       note: None,
   };
   ```
4. Corpus loader requires `source_id: kinh-dich` on EVERY hexagram entry (CRIT-1's schema). The casting module requires `source_id: mai-hoa-dich-so` on its ONE rule-evidence emission. No file emits both.
5. Contract test: serialize a sample `ConsultationIntent::IChing` consultation and assert the evidence list contains AT LEAST one entry per source_id, and that the composite `rule.composite.iching_consultation` is distinct from both.
**Detection:** Snapshot test on the evidence chain for a representative consultation. CI grep: `rg '"kinh-dich"' crates/amlich-core/src/reasoning/iching/` should appear ONLY in the corpus-loader and text-lookup code, never in casting code.
**Phase to address:** `P-KD-1` (register constants + extend guard) → `P-KD-4` (integration phase, separate envelopes).
**Confidence:** HIGH (anchored in v1.5 CRIT-1, `tests/source_id_guard.rs`, Expansion Framework §3.1).

---

## Moderate Pitfalls

These cause reliability or audit-trail issues but are recoverable without a rewrite.

### MOD-1 — Hào Từ vs Thoán Từ Selection Rule Mishandled

**What goes wrong:** Mai Hoa castings produce a chủ quẻ (original) + a biến quẻ (transformed). The classical reading rule selects WHICH text to surface based on the động hào:
- **No động hào** (impossible in pure Mai Hoa time-number method, but possible in coin/yarrow casts the project may later support): read the **thoán từ** of the chủ quẻ.
- **One động hào** (the Mai Hoa case): read the **hào từ** of the động hào line in the chủ quẻ, plus the thoán từ of the biến quẻ.
- **All six hào move** (rare): read the thoán từ of the biến quẻ.

Implementers collapse this to "always read the thoán từ of the chủ quẻ" because it's the simplest case, losing the động hào's interpretive primacy.
**Why it happens:** The selection rule is conditional and classical; few Vietnamese Mai Hoa tutorials state it explicitly. The hào từ of the biến quẻ is often omitted from popular summaries entirely.
**Consequences:** Every Mai Hoa reading surfaces the wrong text layer — the động hào's specific hào từ IS the primary reading; without it the consultation loses its time-dynamic ("what happens next") dimension.
**Prevention:**
1. Encode the selection rule as a typed enum + a `select_reading_text(cast: &MaiHoaCast) -> ReadingTextSelection` function in ONE place.
2. Test every branch of the selection function with at least one golden case.
3. Document the rule in the ADR (CRIT-2's ADR-0005) and in the module header comment.
**Detection:** Snapshot test: same casting input produces a `ReadingTextSelection` that names exactly which text fields to surface.
**Phase to address:** `P-KD-2` (algorithm phase; selection rule lands with casting).
**Confidence:** HIGH on the rule's existence; MEDIUM on which exact text layers the project surfaces (decision-bearing — make explicit in ADR).

---

### MOD-2 — Thể / Dụng (Body / Use) Determination Inverted

**What goes wrong:** Mai Hoa's interpretation hinges on the **Thể** (body = the trigram WITHOUT the động hào) and **Dụng** (use = the trigram WITH the động hào). The ngũ hành relationship between Thể and Dụng (sinh khắc / mutual generation or overcoming) determines the cát/hung judgment. Two bugs:
1. **Inverted assignment**: implementer hard-codes "lower trigram = Thể" — wrong when the động hào is in the lower trigram (then lower = Dụng).
2. **Ngũ hành direction wrong**: "Dụng sinh Thể" is auspicious; "Thể sinh Dụng" is inauspicious (draining). Inverting the direction flips every judgment.
**Why it happens:** The Thể/Dụng split is conditional on động hào position (which trigram it falls in: hào 1-3 = lower/Dụng-side, hào 4-6 = upper/Dụng-side). The conditional is easy to forget. The sinh direction is classical but Vietnamese tutorials sometimes paraphrase without specifying the Thể→Dụng direction.
**Consequences:** Every cát/hung judgment is the inverse of the classical rule for a subset of castings. The bug is statistically significant but not universal — a perfect "looks correct in demos, wrong in production" pattern.
**Prevention:**
1. Encode the Thể/Dụng assignment as a function returning `TheDung { the: Trigram, dung: Trigram }`, conditional on `dong_hao` position (1-3 → lower=Dụng; 4-6 → upper=Dụng).
2. Encode the ngũ hành sinh/khắc relationship as a typed enum with explicit auspiciousness per direction: `DungSinhThe` (cát), `TheSinhDung` (hung — drain), `TheKhacDung` (cát), `DungKhacThe` (hung), `BinhHo` (neutral — same element).
3. Lock the direction conventions in the ADR (CRIT-2's ADR-0005).
4. Golden: ≥10 castings covering all 5 Thể/Dụng relationship outcomes.
**Detection:** Unit test: for each of the 5 outcomes, golden the cát/hung label. Cross-reference with nhantu.net's published Mai Hoa Thể/Dụng examples.
**Phase to address:** `P-KD-2`.
**Confidence:** MEDIUM (the Thể/Dụng rule is classical Mai Hoa standard but the project must verify its chosen reference uses the same direction conventions).

---

### MOD-3 — Hexagram Text Fidelity: Footnotes / Commentary / Modern Paraphrase Leaking Into Classical Text

**What goes wrong:** Ngô Tất Tố's *Kinh Dịch Trọn Bộ* interleaves (a) the classical **thoán từ / hào từ** text, (b) **Tượng Truyện** 象傳 commentary, (c) **Văn Ngôn / Đại Truyện** traditional commentary, and (d) Ngô Tất Tố's own modern Vietnamese commentary. An author transcribing the entry conflates these layers into a single `thoan_tu` string. The corpus loses the ability to distinguish "what the oracle says" from "what the translator thinks it means."
**Why it happens:** The four layers appear on consecutive pages of the book. Without explicit schema fields per layer, the transcriber pastes whatever block they happen to be reading. The classical text is short (often 1-3 sentences for thoán từ); the commentary is long; the boundary requires careful reading.
**Consequences:**
1. Audit cannot answer "is this the classical text or Ngô Tất Tố's commentary?" — the cát/hung judgment derived from the text becomes unmappable to source.
2. Future ADR work (e.g., comparing with another Vietnamese Kinh Dịch translation) becomes impossible because the layers can't be separated post-hoc.
3. NFC normalization and Hán-Việt phrasing checks get muddied by mixed-register text (MOD-4 below accelerates).
**Prevention:**
1. Schema (CRIT-1's schema-lock) defines SEPARATE optional fields per layer:
   ```rust
   pub struct HexagramEntry {
       pub king_wen_no: u8,             // 1..=64
       pub name_vi: String,             // "Thuần Kiền"
       pub name_hantu: String,          // "乾"
       pub thoan_tu: String,            // REQUIRED — classical hexagram statement
       pub hao_tu: [String; 6],         // REQUIRED — 6 line statements
       #[serde(default, skip_serializing_if = "Option::is_none")]
       pub tuong_truyen: Option<String>,    // OPTIONAL — 象傳 commentary
       #[serde(default, skip_serializing_if = "Option::is_none")]
       pub modern_commentary: Option<String>, // OPTIONAL — Ngô Tất Tố's commentary
       pub source_id: String,           // = SOURCE_KINH_DICH
       pub original_citation: String,   // "Ngô Tất Tố, Kinh Dịch Trọn Bộ, Tập I, tr. 23"
       pub reviewer: String,            // mirror v1.6 RIT-11
   }
   ```
2. Loader lint: warn (or fail CI) if `thoan_tu` length exceeds a reasonable threshold (e.g., 500 chars) — signals that commentary leaked in.
3. Manual review per entry: the reviewer field must verify that `thoan_tu` is the classical text only, not commentary.
**Detection:** Periodic audit: randomly sample 5 entries per quarter, compare `thoan_tu` against the cited page of Ngô Tất Tố's book. Any drift is a MOD-3 bug.
**Phase to address:** `P-KD-1` (schema-lock) + `P-KD-3` (corpus authoring with reviewer sign-off per entry).
**Confidence:** HIGH (well-known translation-fidelity issue in classical-text digitization projects).

---

### MOD-4 — Vietnamese NFC Normalization Drift in 64-Hexagram Text (carry: v1.5 MOD-4)

**What goes wrong:** The 64-hexagram corpus uses extensive Vietnamese diacritics (in modern commentary) and Hán-Việt (Sino-Vietnamese) forms throughout. Same trap as v1.5 MOD-4: NFC vs NFD byte differences, pre-1975 orthography (`nầy`/`này`, `hoà`/`hòa`), and Hán-Việt character inconsistencies break search, snapshot tests, and equality checks.
**Why it happens:** Authoring across multiple editors/OSes. Ngô Tất Tố's text was originally set in pre-1975 orthography; modern re-publications may have updated to modern orthography inconsistently. The classical text is Hán-Việt-heavy (`nguyên hanh lợi trinh`, `vô cữu`); the commentary is modern Vietnamese.
**Consequences:** Search "Thuần Kiền" returns 0 hits when text contains the same string in a different normalization form. Snapshot tests flake across machines.
**Prevention:** Re-apply v1.5 MOD-4 prevention verbatim:
1. NFC-normalize every text field at corpus load (single helper in the loader).
2. CI lint: `unicode-normalization` crate check across `data/iching/*.json` files.
3. Pick one tone-position convention (modern: `hòa`, `thuỷ` vs `thủy`) and document in `data/iching/README.md`.
4. For classical Hán-Việt phrases (`nguyên hanh lợi trinh`, etc.) declare them in a separate `hanviet_text` field if inline modern Vietnamese translation is also present.
**Detection:** Round-trip test: load JSON → normalize → re-serialize → assert byte-equal to canonical fixture.
**Phase to address:** `P-KD-3` (loader phase, lands with corpus authoring).
**Confidence:** HIGH (carry-forward of v1.5 MOD-4 — exact same pattern, different corpus).

---

### MOD-5 — Additive `Option<T>` Regression on `DaySnapshot` / Reasoning Envelopes (carry: v1.5 MOD-6)

**What goes wrong:** The new IChing reasoning branch in `reasoning/personal.rs` adds new fields to shared DTOs (`DaySnapshot`, `PersonalRecommendation`, or `PersonalReasoningInput`/`PersonalFactNode`). Any non-`Option<T>` new field breaks v1.6 serialized fixtures and any external workspace (the Personal Lab, Season Timeline, Almanac Inspector apps per recent commits).
**Why it happens:** Project policy is additive-only (`PROJECT.md` Key Decision: "Additive-only integration changes — confirmed in v1.2"). New contributors forget; the convention is not enforced by the type system.
**Consequences:** Downstream consumers fail to load older saved data; snapshot tests across workspaces flake.
**Prevention:** Re-apply v1.5 MOD-6:
1. ALL new fields on existing public DTOs MUST be `Option<T>` with `#[serde(default, skip_serializing_if = "Option::is_none")]`.
2. Round-trip contract test: deserialize all `tests/fixtures/v1.6-*.json` (or current v1.6 fixtures) against the v1.7 struct; re-serialize; assert no new fields appear in the output.
3. Prefer **adding a new evaluator branch in `reasoning/personal.rs::build_fact_nodes`** that emits new `PersonalFactNode`s (each already an additive `Vec` entry — see `personal.rs:38-104`) OVER mutating `PersonalReasoningInput`. The existing pattern of pushing a new node into the `nodes` vec is the cleanest additive path — mirror it.
4. The `ConsultationIntent::IChing` enum variant addition (extending `advisory.rs:20-30`) is a non-additive change to the enum — this requires special handling. Per serde conventions, adding a new enum variant is backward-compatible ONLY if old serialized data doesn't deserialize via `#[serde(deny_unknown_fields)]`-equivalent on the enum (it doesn't, by default). Document this in the v1.7 milestone audit; assert all existing `ConsultationIntent` consumers continue to function. If any persisted data stores the enum as a string-keyed map, the new `"i_ching"` value will be invisible to old consumers (graceful ignore) but not a hard break.
**Detection:** CI test: deserialize all `tests/fixtures/v1.6-*.json` against the v1.7 struct without error.
**Phase to address:** `P-KD-4` (integration phase).
**Confidence:** HIGH (anchored in v1.2 additive precedent + v1.5 MOD-6 carry-forward).

---

### MOD-6 — Evidence Granularity Loss on Composite I-Ching Envelope (carry: v1.5 MOD-5)

**What goes wrong:** An I-Ching consultation emits ONE aggregate evidence envelope covering casting + text lookup + biến quẻ derivation. Per-step provenance is flattened; downstream audit cannot answer "which classical page sourced this hào từ?" — the answer is hidden behind the aggregate.
**Why it happens:** `ReasoningEvidenceEnvelope` (per `reasoning/types.rs:144-151`) is a single struct; the easy implementation attaches one envelope per consultation.
**Consequences:** Audit trail is incomplete. The semantic graph's value as an explainability tool degrades.
**Prevention:** Re-apply v1.5 MOD-5:
1. Per step: attach `ReasoningEvidenceEnvelope { source_id: SOURCE_MAI_HOA_DICH_SO, method: "tien_thien_modulo_cast", ... }`, `... SOURCE_KINH_DICH, method: "ngo_tat_so_lookup" ...`, `... SOURCE_MAI_HOA_DICH_SO, method: "bien_que_derivation" ...`. Aggregate node holds a separate `rule.composite.iching_consultation` envelope per §3.2.
2. Contract test: serialize a consultation result and assert `evidence.len() >= 3` (casting + text + composite, plus biến quẻ text = 4), with distinct `source_id` per primitive entry.
**Detection:** Snapshot test on the evidence chain for a representative consultation.
**Phase to address:** `P-KD-4` (integration phase).
**Confidence:** HIGH (anchored in Expansion Framework §3.2 + `reasoning/types.rs:144-151`).

---

### MOD-7 — Tier-0 vs Tier-2 Confusion: I-Ching is Tier-0 but Enrichment Tempts Tier-2 Drift

**What goes wrong:** Per Expansion Framework §2.2, I-Ching is **Tier-0** — only the query time is required; Bazi (Tier-2) enrichment is OPTIONAL. An implementer wires `PersonalReasoningInput::birth` into the casting step (e.g., uses the birth-year chi to derive a "personal động hào"), promoting IChing to Tier-2. Then a Tier-0 caller (no birth data) gets `Unsupported` where the framework mandates they get a valid casting.
**Why it happens:** `PersonalReasoningInput::from_birth(birth, intent)` (per `reasoning/personal.rs:27`) requires a `BirthInput`. The IChing evaluator is being added to a Tier-2-flavored module; the temptation to "enrich with personal data" is structural.
**Consequences:** The framework's Tier-0 promise for I-Ching is silently violated. Users without a configured birth time get no consultation result. The IChing source-id (`mai-hoa-dich-so`) gets entangled with Bazi source-ids, muddying the provenance.
**Prevention:**
1. The IChing evaluator branch MUST accept a query-time-only entrypoint: `cast_iching(snapshot: &DaySnapshot, question: &str) -> IChingConsultation`. `birth: &BirthInput` is an OPTIONAL enrichment parameter, NOT required.
2. If enrichment is present, emit a SEPARATE composite envelope (`rule.composite.iching_with_bazi_enrichment`) — distinct from the Tier-0 baseline envelope (`rule.composite.iching_consultation`).
3. The `ConsultationIntent::IChing` evaluator must return the Tier-0 result even when birth data is absent (mirror the framework's "Tier 0 đủ" declaration for Kinh Dịch).
4. Contract test: `cast_iching` with NO `birth` returns a valid consultation; with `birth` returns the same consultation PLUS an enrichment composite.
**Detection:** Unit test: build `PersonalReasoningInput` with `birth.year = 0` / `birth = None` (if API allows) and assert the IChing branch still produces a result.
**Phase to address:** `P-KD-4` (integration phase — evaluator branch lands with the Tier-0 contract).
**Confidence:** HIGH (anchored in Expansion Framework §2.2 "Tier 0 đủ").

---

### MOD-8 — Tam Sát vs Sát Phương: Both `khcbppt` but Distinct Concepts

**What goes wrong:** The cross-link joins KHCBPPT-derived directional warnings. Within KHCBPPT there are at least three directional-signal families:
- **Sát Phương** (`almanac/sat_phuong.rs:23-36`) — killing direction by Tam Hợp triad → opposite cardinal. ONE direction per day.
- **Tam Sát** (三殯, "three killings") — THREE adjacent directions derived from the year/day branch's triad (e.g., Dần-Ngọ-Tuất → Sát Bắc at three contiguous sơn).
- **Thái Tuế** (`almanac/thai_tue.rs:14-27`) — annual conflict relative to the USER's birth branch, NOT a directional warning per se (it's a branch-relationship check, despite its frequent grouping with "directional taboos").

The cross-link collapses these into "KHCBPPT directional signals" without distinguishing which sub-family produced each entry. A user is told "avoid hướng X" without knowing whether it's a one-direction sát phương or a three-direction Tam Sát. The two have different remedial actions in classical practice.
**Why it happens:** `sat_phuong.rs:38-43` emits a single `direction: String`. Tam Sát (when implemented) will likely emit `Vec<String>` of three directions. The type system doesn't differentiate at the cross-link boundary. Thái Tuế (`thai_tue.rs:36-41`) returns `ThaiTueConflict` not a direction at all — but popular usage groups it with "directional taboos."
**Consequences:** Remediation advice is wrong (different taboos have different remedies — Hóa Giải rituals differ). Audit cannot answer "which KHCBPPT sub-family produced this signal?"
**Prevention:**
1. In the cross-link envelope, emit each KHCBPPT signal with a typed `khcbppt_subfamily` tag (`SatPhuong`, `TamSat`, `ThaiTue`) — distinct from `source_id` (which stays `khcbppt`).
2. The cross-link's `khcbppt_signals: Vec<KhcbpptDirectionSignal>` (CRIT-5 prevention) is typed with the subfamily discriminator; not a flat `Vec<String>`.
3. Document in the module header: Thái Tuế is included in the cross-link as a directional-relevant signal but is NOT itself a direction (it's a branch relationship projected to direction space).
4. Verify Tam Sát is implemented as a distinct module (likely `almanac/tam_sat.rs` — does NOT currently exist; check whether PROJECT.md's reference to "KHCBPPT directional warnings (`thai_tue`/`tam_sat`)" implies Tam Sát must be implemented in v1.7 OR is already available via `sat_phuong`/`than_sat.rs`). **Open question for the roadmapper**: is Tam Sát net-new in v1.7 or pre-existing?
**Detection:** Snapshot test: cross-link envelope for a sample date distinguishes subfamily tags.
**Phase to address:** `P-TT-1` (cross-link phase — typed subfamily lands with the module).
**Confidence:** HIGH on the subfamily distinction; MEDIUM on whether Tam Sát exists in the codebase today (TODO: verify `almanac/tam_sat.rs` or `almanac/than_sat.rs` during P-TT-1 schema-lock).

---

## Minor Pitfalls

### MIN-1 — 64-Hexagram Corpus File Layout: One File vs 64 Files

**What goes wrong:** Decision needed: one `data/iching/hexagrams.json` containing all 64 entries, OR 64 separate files. v1.5 văn khấn chose one-file-per-ritual (`data/rituals/<event>.json` + manifest) for reviewer-atomicity. For 64 hexagrams, one-file-per-hexagram is overkill (no individual reviewer wants a one-entry PR), but one-monolithic-file risks merge conflicts when multiple reviewers transcribe in parallel.
**Prevention:** Recommend: 8 files, grouped by Cung (Upper/Lower × 4 each) or by King Wen octant, with a `manifest.json` mirroring `data/rituals/manifest.json:1-18`. Each file ~8 entries is reviewable and merge-conflict-safe.
**Detection:** N/A — design decision; lock in `P-KD-1` schema-lock phase.
**Phase to address:** `P-KD-1`.
**Confidence:** MEDIUM (project-layout style decision; no canonical "right" answer).

---

### MIN-2 — Vocabulary Collision: "Quẻ" Means Both Trigram and Hexagram in Vietnamese

**What goes wrong:** Vietnamese uses "quẻ" for both **quẻ đơn** (trigram / 3-line ba quái) and **quẻ kép** (hexagram / 6-line quẻ). A `struct Que { ... }` in code is ambiguous — does it carry 3 lines or 6?
**Prevention:** Use `Trigram` for the 3-line concept and `Hexagram` for the 6-line concept in Rust identifiers. Reserve `quẻ` for Vietnamese display strings only. Mirror v1.5 MIN-2's `Palace` discipline.
**Detection:** Code-review check.
**Phase to address:** `P-KD-1` (types phase).
**Confidence:** HIGH.

---

### MIN-3 — Hard-Coded Gregorian Casting Test Dates Without Lunar Context (carry: v1.5 MIN-5)

**What goes wrong:** A test asserts "on 2026-07-16, Mai Hoa cast returns hexagram X" with hard-coded Gregorian. The intent was "lunar 6/2 giờ Tý," which shifts year to year. Test passes the year it was written, becomes silently wrong in maintenance years later.
**Prevention:** Write casting tests as `for each year in [2024..2030]: compute lunar month/day/hour from Gregorian test-date → assert cast returns hexagram X`. Hard-coded Gregorian only for cross-validation pinning, never as the assertion.
**Detection:** Year-parametrized test runner.
**Phase to address:** `P-KD-2` (test phase).
**Confidence:** HIGH.

---

### MIN-4 — Hán-Việt / Nôm / Original Chinese Phrases in Classical Text

**What goes wrong:** Some classical thoán từ / hào từ contain short Chinese phrases (e.g., 「元亨利貞」) that appear in the Vietnamese translation as `nguyên hanh lợi trinh`. The transcriber may paste either form. NFC + character-set CI guards (CRIT-1 prevention) flag Han characters but the corpus is intended to have them in dedicated Hán-Việt fields.
**Prevention:** Schema separates `thoan_tu` (Vietnamese translation only) from `thoan_tu_hanviet` (Hán-Việt transliteration) and optionally `thoan_tu_original` (Chinese characters). CI guard: Han characters MUST appear only in the `_original` field.
**Detection:** CI lint: `rg '[一-龥]' data/iching/*.json` returns only `_original` field values.
**Phase to address:** `P-KD-1` (schema) + `P-KD-3` (loader lint).
**Confidence:** MEDIUM.

---

### MIN-5 — Cát / Hung Interpretation Boundary Creep

**What goes wrong:** Ngô Tất Tố's commentary (and modern Vietnamese Mai Hoa schools) attaches cát/hung labels to each hexagram and hào. The corpus author interpolates modern "self-help" interpretations alongside classical ones. The boundary between classical text and modern interpretation blurs.
**Prevention:**
1. Schema field `interpretation: Option<CatHungLabel>` carries an enum value (`Cat`, `Hung`, `BánCatBanHung`, `TùySự`) — not a freeform string.
2. `interpretation_source` field records whose interpretation it is (`kinh-dich-classical` vs `ngo-tat-so-modern` vs `mai-hoa-school`). The label is data, not derived at the UI.
3. Documentation: classical cát/hung from the hào từ takes precedence; modern commentary is supplementary and MUST be marked.
**Detection:** Schema lint: every `interpretation` entry has a non-empty `interpretation_source`.
**Phase to address:** `P-KD-1` (schema) + `P-KD-3` (loader lint).
**Confidence:** MEDIUM.

---

## Integration Gotchas (against the existing architecture)

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| `reasoning/personal.rs::build_fact_nodes` | Adding IChing as a `birth`-required node | Push a new `PersonalFactNode` into the `nodes` vec (mirror `personal.rs:38-104`) — additive, Tier-0-friendly. `birth` is optional enrichment. |
| `advisory.rs::ConsultationIntent` enum | Adding `IChing` variant breaks serialized fixtures | New variant is backward-compatible IF old consumers ignore unknown variants. Add round-trip test with v1.6 fixtures (MOD-5). Document in milestone audit. |
| `sources.rs` constants | Forgetting to register `SOURCE_KINH_DICH` / `SOURCE_MAI_HOA_DICH_SO` before code references them | Register constants FIRST in `P-KD-1`; extend `tests/source_id_guard.rs:13-21` `FORBIDDEN_LITERALS` in the same plan (CRIT-6). |
| `tests/source_id_guard.rs` | Adding new literals but not extending the forbidden list | Extend `FORBIDDEN_LITERALS` in the same commit that adds the constant to `sources.rs`. The guard's allow-list for `sources.rs` itself is automatic (file-name skip, see `source_id_guard.rs:43-46`). |
| `interaction/direction_merge.rs` | Wiring the Thái Tuế ⇄ Phi Tinh cross-link INTO `direction_merge` | KEEP the cross-link in a new `reasoning/directional_cross_link.rs` (or extend `reasoning/personal.rs`). `direction_merge.rs` stays Tier-1 Bát Trạch only (CRIT-5 / v1.5 CRIT-3). |
| Corpus loader pattern (v1.5) | Skipping NFC normalization, schema `deny_unknown_fields`, manifest-based file enumeration | Re-apply v1.5 loader verbatim. Manifest at `data/iching/manifest.json` mirroring `data/rituals/manifest.json:1-18`. |
| `KnownDivergence` / `DeferralMarker` schema (v1.6) | Inventing a new "divergence" type for IChing golden | Reuse the v1.6 typed schema directly (`fengshui/golden.rs::KnownDivergence` / `DeferralMarker`). Same pattern, different data. |
| ADR authoring (ADR-0001..0004 pattern) | Skipping the ADR for Mai Hoa casting convention | ADR-0005 (Mai Hoa casting) is a CRIT-1/CRIT-2 blocker; ADR-0006 (Thái Tuế ⇄ Phi Tinh read-only join shape) is a CRIT-5 blocker. Both land in `P-KD-1` / `P-TT-1`. |

---

## Performance Traps

Not a primary concern for this milestone (no large-data computation; 64-entry lookup table; single casting per consultation), but noted:

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| 64-hexagram table loaded from disk per call | Slow consultation; disk I/O | Use `OnceLock` + `include_str!` (mirror `fengshui/golden.rs:30-39`) — table is compile-time-embedded | N/A — breaks immediately at >1 req/sec |
| Casting recomputed on every `DaySnapshot` field access | Repeated modulo work | Cache the casting on the `DaySnapshot` for the queried date | N/A — modulo is cheap, but no reason to recompute |
| Cross-link recomputes Thái Tuế + Phi Tinh per query | Repeated branch-relation work | The cross-link READS existing outputs from `almanac/thai_tue.rs` and `almanac/fengshui/` — it MUST NOT recompute them | N/A — design dictates this |

---

## Security / Safety Mistakes

Domain-specific safety concerns beyond general Rust safety:

| Mistake | Risk | Prevention |
|---------|------|------------|
| Presenting Mai Hoa cát/hung as deterministic fortune-telling | Cultural / ethical: users make major life decisions (marriage, business) on a "cát" reading | Add `confidence: DecisionConfidence::Low` (or new `Divinatory` tier) on IChing reasoning nodes; surface "tham khảo" framing in Vietnamese summary. Do NOT include in `InitiationOpeningDecision`'s `favorable`/`avoid` buckets with the same weight as KHCBPPT taboos. |
| Caching a casting result keyed only by date | Two different `question` strings on the same date return the same casting | Include `question` (or its hash) in the cache key, OR document that Mai Hoa time-number casting is by-design question-independent (classical position: the casting reflects the moment, not the question — make this explicit). |
| Cross-link surfacing Thái Tuế conflicts as "directions to avoid" | Misleading: Thái Tuế is a branch-relationship signal, not a direction | Tag with `khcbppt_subfamily: ThaiTue` (MOD-8); summary text MUST distinguish "phạm Thái Tuế" from "sat phương." |

---

## UX Pitfalls

| Pitfall | User Impact | Better Approach |
|---------|-------------|-----------------|
| Showing only the biến quẻ text (the dynamic reading) without the chủ quẻ | User lacks context for how the situation transforms | Always surface BOTH chủ quẻ (current state) and biến quẻ (trajectory), labeled clearly in Vietnamese |
| Showing the hexagram name without the King Wen number | User cannot look up the hexagram in their own reference book | Include `Quẻ số N (King Wen)` in the summary |
| Collapsing Thái Tuế + Phi Tinh into one "Hướng Xấu Hôm Nay" | User cannot act on the appropriate remedy | Surface both traditions separately with their distinct remedy families (CRIT-5 / MOD-8) |
| Hard-coded modern cát/hung label without classical text | User cannot audit the judgment | Show classical hào từ + cát/hung label side-by-side (MOD-3, MIN-5) |

---

## "Looks Done But Isn't" Checklist

Things that appear complete during a demo but are missing critical pieces:

- [ ] **Mai Hoa casting:** Often missing the `% == 0 → 8` convention (CRIT-2) — verify boundary cases (8/8/8 input).
- [ ] **64-hexagram lookup:** Often missing the Tiên Thiên → King Wen composition table (CRIT-3) — verify Tiên Thiên (3,3) returns King Wen #30, not #3.
- [ ] **Biến quẻ derivation:** Often missing the động hào position convention (CRIT-4) — verify all 384 (64×6) cases match the 八卦變 chart.
- [ ] **Source-id discipline:** Often missing the `kinh-dich` vs `mai-hoa-dich-so` split (CRIT-6) — verify two distinct evidence envelopes per consultation.
- [ ] **Cross-link:** Often missing CRIT-3 isolation preservation (CRIT-5) — grep-verify `fengshui` and `thai_tue`/`sat_phuong` remain unimported across each other.
- [ ] **Additive DTO:** Often missing `Option<T>` on new fields (MOD-5) — round-trip a v1.6 fixture through v1.7 struct.
- [ ] **Tier-0 contract:** Often missing the no-birth-data path (MOD-7) — call `cast_iching` with `birth = None`, must succeed.
- [ ] **Hào từ vs thoán từ selection:** Often missing the động hào conditional (MOD-1) — verify same casting with different động hào positions yields different reading-text selections.
- [ ] **Thể/Dụng direction:** Often missing the inversion check (MOD-2) — verify Dụng sinh Thể = cát; Thể sinh Dụng = hung.
- [ ] **NFC normalization:** Often missing the loader-side normalize (MOD-4) — verify round-trip byte-equal.
- [ ] **Golden methodology:** Often missing the ≥2-source-per-case requirement — verify every IChing golden case has ≥2 sources + classical tiebreaker (per Expansion Framework §7).

---

## Recovery Strategies

When pitfalls occur despite prevention:

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| CRIT-1 (schema slip after corpus authored) | HIGH (448-field re-edit) | Author a migration script; re-run reviewer sign-off; bump schema version; re-verify NFC. Avoid by landing schema-lock first. |
| CRIT-2 (remainder-zero bug shipped) | LOW (one-line fix + boundary tests) | Fix the modulo helper; add the 8/8/8 golden case; re-issue affected castings. |
| CRIT-3 (Tiên Thiên/King Wen conflation shipped) | MEDIUM (restructure types) | Introduce the typed `TienThienTrigram` / `KingWenHexagram` boundary; re-validate the composition table exhaustively; re-run all 4320-cast enumeration. |
| CRIT-4 (biến quẻ bug) | LOW (384-case test reveals exact failure) | Run the 384-case contract test; fix the bit-position; re-verify. |
| CRIT-5 (CRIT-3 isolation broken) | HIGH (audit-graph repair) | Grep audit; move cross-link code back to `reasoning/`; restore source-file isolation; add the CI grep guard retroactively. |
| CRIT-6 (source-id conflation) | MEDIUM (re-tag corpus + evidence) | Re-tag every hexagram entry's `source_id`; split composite envelopes; re-run contract test. |
| MOD-3 (commentary leaked into classical text) | HIGH (manual re-review of all 64 entries) | Re-open each entry against the cited page; separate the layers into the schema fields; reviewer re-signs. |

---

## Pitfall-to-Phase Mapping

How the v1.7 roadmap phases (suggested labels — final numbering per ROADMAP phase) should address these pitfalls:

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| CRIT-1 (schema-lock) | `P-KD-1` Schema-lock (types + ADR-0005) | Loader test asserts 64 entries, all required fields, all King Wen numbers 1..=64 unique |
| CRIT-2 (modulo convention) | `P-KD-1` (ADR) + `P-KD-2` (algorithm) | Boundary golden cases (8/8/8, etc.); 4320-cast enumeration |
| CRIT-3 (Tiên Thiên/King Wen) | `P-KD-1` (types + composition table) + `P-KD-2` | Exhaustive 64-entry composition-table test; Thiệu Khang Tiết "two sparrows" historical golden |
| CRIT-4 (biến quẻ) | `P-KD-2` | 384-case (64×6) biến quẻ contract test |
| CRIT-5 (CRIT-3 isolation in cross-link) | `P-TT-1` (module boundary + envelope shape) | Grep audit: `fengshui` not imported by `thai_tue`/`sat_phuong`; cross-link envelope carries both source_ids distinctly |
| CRIT-6 (source-id split) | `P-KD-1` (register constants + extend guard) + `P-KD-4` (envelopes) | Contract test: each consultation has ≥2 distinct primitive source_ids + 1 composite |
| MOD-1 (hào from vs thoán từ selection) | `P-KD-2` | Snapshot test per selection branch |
| MOD-2 (Thể/Dụng inversion) | `P-KD-2` | Golden per each of 5 ngũ hành outcomes |
| MOD-3 (text fidelity) | `P-KD-1` (schema with layer fields) + `P-KD-3` (reviewer sign-off) | Periodic random-sample audit against cited pages |
| MOD-4 (NFC) | `P-KD-3` (loader phase) | Round-trip byte-equal test; CI unicode-normalization check |
| MOD-5 (additive DTO) | `P-KD-4` (integration) | Round-trip v1.6 fixtures through v1.7 struct |
| MOD-6 (composite envelope granularity) | `P-KD-4` (integration) | Snapshot test asserts ≥3 evidence entries per consultation |
| MOD-7 (Tier-0 contract) | `P-KD-4` (integration) | `cast_iching` with no `birth` succeeds |
| MOD-8 (KHCBPPT subfamily tags) | `P-TT-1` (cross-link module) | Snapshot test distinguishes `SatPhuong` / `TamSat` / `ThaiTue` tags |
| MIN-1..5 | Various | See per-pitfall detection above |

---

## Validation Strategy Summary

There is no single canonical Mai Hoa implementation in Vietnamese. Per Expansion Framework §7, the project's reference set is `nhantu.net (Mai Hoa), divination.com (hexagram texts)`. Strategy mirroring v1.5/v1.6:

**For Mai Hoa casting + biến quẻ:**
1. Build an IChing golden dataset (mirror `fengshui/golden.rs::PhiTinhGoldenDataset`): ≥10 castings, cross-checked against ≥2 of: nhantu.net, a second Vietnamese Mai Hoa site, the canonical Thiệu Khang Tiết text.
2. Treat the **classical Thiệu Khang Tiết text** as the tiebreaker when modern sites disagree; log every disagreement as `KnownDivergence` per §7 — do not silently pick a winner.
3. Algorithmic invariants gate the casting module regardless of source choice: every casting produces a valid `(TienThienTrigram, TienThienTrigram, MovingLine)` triple; every triple maps to exactly one King Wen hexagram; every (chủ, động hào) maps to exactly one biến quẻ.
4. Cover at least: the Thiệu Khang Tiết "two sparrows" historical cast, all 8 remainder-zero boundary cases for trigram, all 6 for động hào, one casting per King Wen hexagram class (8 representatives).

**For 64-hexagram text fidelity:**
1. Per-entry citation to Ngô Tất Tố, *Kinh Dịch Trọn Bộ* (book + page). Entries without citation fail CI.
2. Loader contract tests: schema enforcement (`deny_unknown_fields`), NFC normalization, Han-character guard (only in `_original` fields), `hao_tu` length == 6, all 64 King Wen numbers present and unique.
3. Independent reviewer audit per batch of entries (mirror v1.6 RIT-11 / Phase 17 work).
4. Classical-layer vs commentary-layer separation verified by sampling (MOD-3).

**For Thái Tuế ⇄ Phi Tinh cross-link:**
1. Validation = **structural**, not computational: assert both source_ids present in envelope, neither collapsed, subfamily tags distinct (MOD-8).
2. CRIT-3 isolation grep audit (mirror v1.5 milestone-audit methodology): `rg 'fengshui|FlyingStar'` in `almanac/thai_tue.rs`, `almanac/sat_phuong.rs`, `interaction/direction_merge.rs` returns zero.
3. Round-trip with v1.6 fixtures (MOD-5 gate) confirms no backward-compat regression.

**Cross-cutting:** All three new source_ids (`kinh-dich`, `mai-hoa-dich-so`, `rule.composite.iching_consultation`, plus `rule.composite.directional_cross_link`) appear in the evidence chain with distinct `source_id` values; no envelope collapses them.

---

## Sources

**In-repo (HIGH confidence anchors):**
- `.planning/PROJECT.md` — v1.7 milestone scope; Key Decisions table (DEC-0023 source_id discipline, schema-lock-before-corpus, additive Option<T>, CRIT-3 isolation, ADR-0001..0004 pattern).
- `.planning/research/EXPANSION_FRAMEWORK.md` — §2.2 (Kinh Dịch pillar definition, Tier-0), §2.3 (Phi Tinh/sat_phuong file boundaries), §3.1 (source provenance), §3.2 (composite envelopes), §7 (validation references).
- `.planning/research/PITFALLS.md` (prior v1.5 version, archived in PROJECT.md) — CRIT-1/3/4/5, MOD-1/4/5/6, MIN-1/2/5 lessons carried forward.
- `crates/amlich-core/src/sources.rs` — `pub const SOURCE_*` pattern + `SourceId = String` transparent alias (DEC-0023).
- `crates/amlich-core/tests/source_id_guard.rs:13-21` — `FORBIDDEN_LITERALS` list to extend.
- `crates/amlich-core/src/reasoning/personal.rs:38-104` — existing `build_fact_nodes` pattern to mirror (additive `PersonalFactNode` push).
- `crates/amlich-core/src/reasoning/types.rs:144-151` — `ReasoningEvidenceEnvelope` shape for per-step attribution.
- `crates/amlich-core/src/advisory.rs:20-30` — `ConsultationIntent` enum to extend with `IChing` variant.
- `crates/amlich-core/src/almanac/thai_tue.rs:14-41` — Thái Tuế types (KHCBPPT-tagged; `ThaiTueConflictKind` enum; evidence field).
- `crates/amlich-core/src/almanac/sat_phuong.rs:23-43` — Sát Phương direction table + `SatPhuongResult` shape.
- `crates/amlich-core/src/almanac/fengshui/mod.rs:10-11` — explicit CRIT-3 isolation note; module structure for the fengshui family.
- `crates/amlich-core/src/almanac/fengshui/golden.rs:30-39, 60-72` — `OnceLock + include_str!` loader pattern; `GoldenConfidence` tier; `KnownDivergence` schema to reuse.
- `crates/amlich-core/data/rituals/manifest.json:1-18` — manifest-based corpus pattern to mirror for `data/iching/`.
- `crates/amlich-core/data/almanac/flying_stars_golden.json:1-60` — golden case structure (`sources`, `tiebreaker`, `confidence`) to mirror for IChing golden.

**External (MEDIUM confidence — classical domain knowledge from training data, to be re-verified against the project's chosen reference text during P-KD-1):**
- Thiệu Khang Tiết (邵雍), *Mai Hoa Dịch Số* (梅花易數) — classical casting-algorithm reference; Tiên Thiên trigram numbers; động hào derivation; Thể/Dụng rule.
- Ngô Tất Tố, *Kinh Dịch Trọn Bộ* — 64-hexagram text corpus reference; King Wen ordering; layered text + commentary.
- Expansion Framework §7 named validation references: `nhantu.net` (Mai Hoa casting), `divination.com` (hexagram texts).
- Classical I-Ching: King Wen (文王) sequence vs Fuxi/Pre-Heaven (先天) trigram arrangement vs Lo Shu/Hậu Thiên (後天) palace arrangement — canonical distinction (CRIT-3).

**Open questions for the roadmapper / P-KD-1 schema-lock phase:**
- Does `almanac/tam_sat.rs` exist or must Tam Sát be implemented in v1.7? (MOD-8 dependency.) `ls crates/amlich-core/src/almanac/` shows `than_sat.rs` exists but no `tam_sat.rs` — verify whether `than_sat.rs` covers Tam Sát or whether it is net-new.
- Which Tiên Thiên number arrangement does the project use (1=Kiền…8=Khôn is the dominant convention; at least one Vietnamese sub-school differs)? Lock in ADR-0005.
- Which Hán-Việt orthography for hexagram names (modern `thuỷ` vs pre-1975 `thủy`)? Lock in `data/iching/README.md`.
- Does the project surface the hỗ quẻ (nuclear hexagram) at all in v1.7, or defer to a future "deep Mai Hoa interpretation" milestone? Default recommendation: defer.

---

*Pitfalls research for: v1.7 Kinh Dịch (Mai Hoa Dịch Số) + Thái Tuế/Tam Sát ⇄ Phi Tinh cross-link*
*Researched: 2026-07-16*
