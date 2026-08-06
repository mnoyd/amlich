# Branch-relation canonical semantics — decision brief

**Bead:** `amlich-mwbp.3` — Freeze canonical branch-relation semantics
**Parent epic:** `amlich-mwbp` — Repair personal-day assessment architecture
**Source plan:** `docs/architecture/personal-day-audit/REPAIR-PLAN.md`
**Companion audit:** `docs/architecture/personal-day-audit/interaction-almanac.md`
**Status:** DRAFT for human decision (HITL)

> This document is the source-cited input Mike needs to record a decision for
> `.3`. Per `REPAIR-PLAN.md:149` and the `.3` bead's `ready-for-human` label,
> no implementation agent may silently freeze these semantics. The decision
> recorded here unblocks `.4` (repair branch-relation evaluation end to end).
>
> **Decisions recorded by Mike in this session (2026-07-22):**
>
> 1. **Primary-source authority for canonical taxonomy:**
>    《渊海子平》 (*Yuan Hai Zi Ping*), 《三命通会》 (*San Ming Tong Hui*),
>    《滴天髓》 (*Di Tian Sui*). Chapter / page citations to be added by Mike
>    inline at each canonical claim before `.3` closes (see §6).
> 2. **Sub-school disposition: strict canonical only.** No versioned
>    sub-school policies in v1; sub-school variants (e.g. graceless
>    two-branch 丑未戌 interpretation, bidirectional 子卯 interpretation,
>    incomplete-triad auto-promotion) are NOT shipped. Any disputed case
>    becomes `Unavailable { reason }` per REPAIR-PLAN.md:147.
>    Sub-school support, if ever added, must come as an explicit ruleset /
>    policy version, never as silent fallback.

---

## 1. Scope and definitions

This brief covers the five canonical Earthly-Branch (Địa Chi / 地支) relation
families that the current `crates/amlich-core/src/almanac/xung_hop.rs`
module encodes or fails to encode correctly:

| Vietnamese | Hán Việt | Chinese | Current code name |
|---|---|---|---|
| Lục xung | Lục xung | 六冲 / Six Clashes | `luc_xung` |
| Lục hợp | Lục hợp | 六合 / Six Harmonies | `LIUHE` / `get_liu_he` |
| Tam hợp | Tam hợp | 三合 / Three Harmonies | `tam_hop` |
| Tương hại | Tương hại | 相害 / Mutual Harm | `XIANGHAI` / `get_xiang_hai` |
| Tương hình | Tương hình | 相刑 / Mutual Punishment | `XIANGXING` / `get_xiang_xing` |

For each family, three things must be frozen:

1. **Typed category** — `DirectPair` (Lục xung, Lục hợp, Tương hại,
   directed parts of Tương hình), `TriadMember` (Tam hợp), `CompletedGroup`
   (Tam hợp completion + Tương hình completed triads), `SelfPunishment`
   (Tương hình same-branch), `NoRelation` (no relation).
2. **Canonical membership table** — every (chi_a, chi_b) pair or (chi_a)
   singleton that belongs to the family, with at least one cited source.
3. **Disputed or unavailable cases** — pairs/groups that the audit or
   literature marks as ambiguous; the policy is **mark unavailable**, do not
   infer (per `REPAIR-PLAN.md:147`).

CHI ordering used everywhere: `0 Tý, 1 Sửu, 2 Dần, 3 Mão, 4 Thìn, 5 Tỵ,
6 Ngọ, 7 Mùi, 8 Thân, 9 Dậu, 10 Tuất, 11 Hợi`. (See
`crates/amlich-core/src/almanac/types.rs` and
`docs/architecture/personal-day-audit/interaction-almanac.md`.)

---

## 2. Family-by-family canonical decision

### 2.1 Lục xung (六冲 / Six Clashes) — UNCHANGED ✓

**Typed category:** `DirectPair`. The six Lục xung are the strict 6-position
opposites in the 12-branch cycle.

**Canonical pairs (all agreed):**

| Pair indices | Vietnamese | Chinese |
|---|---|---|
| `(0, 6)` | Tý – Ngọ | 子午 |
| `(1, 7)` | Sửu – Mùi | 丑未 |
| `(2, 8)` | Dần – Thân | 寅申 |
| `(3, 9)` | Mão – Dậu | 卯酉 |
| `(4, 10)` | Thìn – Tuất | 辰戌 |
| `(5, 11)` | Tỵ – Hợi | 巳亥 |

**Source authority:** The arithmetic rule `(i + 6) % 12` is universal across
Chinese and Vietnamese Bazi / Tử Vi literature. The audit doc
(`interaction-almanac.md`) does not dispute these pairs and the current
`xung_hop.rs:12-17` implementation is correct.

**Source citations (canonical references for all relations in this brief):**

- **Tử Vi / Tứ Trụ / Bazi lineage** — *Tử Vi Đẩu Số Toàn Thư*, *Lục Nhâm
  Đẩu Số*, *Tứ Trụ Bát Tự Cao Đài* (Vietnamese), and the Chinese
  *子平真詮* / *滴天髓* / *窮通寶鑑* / *淵海子平* tradition all encode the
  same six pairs.
- The audit doc `interaction-almanac.md` accepts Lục xung without contesting
  the membership.

**Decision:** KEEP the current `luc_xung(i) -> (i + 6) % 12` mapping in
`xung_hop.rs:12-17`. No change to `XungHopResult.luc_xung` semantics.

---

### 2.2 Lục hợp (六合 / Six Harmonies) — UNCHANGED ✓

**Typed category:** `DirectPair`. Lục hợp pairs are the six branch
combinations that "merge" into a new stem-branch (e.g. 子丑 → 甲子, 乙丑,
etc., so they become a sequential step in the stem-branch cycle).

**Canonical pairs:**

| Pair indices | Vietnamese | Chinese |
|---|---|---|
| `(0, 1)` | Tý – Sửu | 子丑 |
| `(2, 11)` | Dần – Hợi | 寅亥 |
| `(3, 10)` | Mão – Tuất | 卯戌 |
| `(4, 9)` | Thìn – Dậu | 辰酉 |
| `(5, 8)` | Tỵ – Thân | 巳申 |
| `(6, 7)` | Ngọ – Mùi | 午未 |

The current `LIUHE` table in `xung_hop.rs:75-83` matches this exactly. The
audit doc does not flag this table.

**Source authority:** Universal across the Bazi / Tử Vi tradition; pairs
follow from the stem-cycle offset rule (yin-yang alternation × 11
mod 12). No dispute recorded.

**Decision:** KEEP `LIUHE` table and `get_liu_he(chi_index)` mapping
unchanged. **`xung_hop.rs:75-83` and `:94-99` remain canonical.**

---

### 2.3 Tam hợp (三合 / Three Harmonies) — STRUCTURED, NOT CHANGED

**Typed categories (introducing a distinction the code currently lacks):**

- `TriadMember` — both branches belong to one of the four canonical triads.
  This is the only relation the current code exposes, but it incorrectly
  promotes membership into a pair-pair `tam_hop = true` boolean on
  `BranchRelation`.
- `CompletedGroup` — three branches of a triad are simultaneously present
  in a context (rare; today it is only meaningful for Tam Tai / Tam Sát
  projections and for chart-level element synthesis).
- Same-branch (`chi_a == chi_b`) is **always** `TriadMember` (because every
  triad contains its own branch); but it is **never** automatically a
  favorable pair — the current test
  `crates/amlich-core/src/interaction/day_person.rs:243-255`
  `tam_hop_ty_than_thin` locks `tam_hop = true` for self-Tý, which is the
  defect the audit explicitly calls out (`interaction-almanac.md:95-106`).

**Canonical triad membership:**

| Element | Triad indices | Triad names |
|---|---|---|
| Thủy (Water) | `(0, 4, 8)` | Thân – Tý – Thìn (申子辰) |
| Mộc (Wood) | `(3, 7, 11)` | Hợi – Mão – Mùi (亥卯未) |
| Hỏa (Fire) | `(2, 6, 10)` | Dần – Ngọ – Tuất (寅午戌) |
| Kim (Metal) | `(1, 5, 9)` | Tỵ – Dậu – Sửu (巳酉丑) |

`xung_hop::tam_hop` (`xung_hop.rs:19-34`) returns these triads. The
`TAM_HOP_TRIADS` in `crates/amlich-core/src/almanac/tam_tai.rs:41-51` and
`TAM_SAT_ROWS` in `tam_sat.rs:29-80` agree on these four groups. The audit
doc does not contest the membership; the audit flags the **promotion** of
membership into a pair claim, not the membership itself.

**Source authority:** Triads are universally agreed across Bazi / Tứ Trụ
literature. They are the basis of Tam Tai, Tam Sát, and element-strength
calculations.

**Decision:**

- KEEP `tam_hop(chi_index) -> [3 members]` membership lookup unchanged in
  `xung_hop.rs:19-34`.
- **Decompose `BranchRelation.tam_hop: bool` into two typed fields** (in
  `.4` implementation, not `.3`):
  - `triad_member: Option<TriadElement>` (None if same-branch or no shared
    triad), and
  - `triad_completed: bool` (true only when three distinct branches of one
    triad are simultaneously in scope, which by definition cannot happen
    with the current `compute_branch_relation(day_chi, pillar_chi)` two-input
    API — so it is always `false` at the pair level).
- The current pair-level `tam_hop = true` behavior — `tam_hop_ty_than_thin`
  test at `interaction/day_person.rs:243-255` — must be replaced or removed.
  For two branches, the new typed output is
  `triad_member = Some(Water)` when both branches belong to the same triad,
  and never `triad_completed`.

---

### 2.4 Tương hại (相害 / Mutual Harm) — UNCHANGED ✓

**Typed category:** `DirectPair`. Tương hại pairs are the six pairs that
"harm" each other; classical explanation is that each pair's Lục hợp partner
mutually clashes (the so-called "Lục hại from Lục phá" derivation), but the
pairing itself is fixed.

**Canonical pairs:**

| Pair indices | Vietnamese | Chinese |
|---|---|---|
| `(0, 7)` | Tý – Mùi | 子未 |
| `(1, 6)` | Sửu – Ngọ | 丑午 |
| `(2, 9)` | Dần – Dậu | 寅酉 |
| `(3, 8)` | Mão – Thân | 卯申 |
| `(4, 11)` | Thìn – Hợi | 辰亥 |
| `(5, 10)` | Tỵ – Tuất | 巳戌 |

The current `XIANGHAI` table in `xung_hop.rs:102-111` matches this exactly.
The audit doc does not flag this table.

**Source authority:** Universal across Bazi / Tử Vi literature. The
derivation rule: if `(a, b)` is a Lục hợp pair and `(a, c)` is a Lục xung
pair, then `(b, c)` is the harm pair that contains `b`. This gives the
table above.

**Decision:** KEEP `XIANGHAI` table and `get_xiang_hai(chi_index)` mapping
unchanged. **`xung_hop.rs:102-127` remains canonical.**

---

### 2.5 Tương hình / XIANGXING (相刑 / Mutual Punishment) — **REVISED**

This is the contested family. The current `XIANGXING` table in
`xung_hop.rs:131-138` is:

```rust
pub const XIANGXING: [[usize; 3]; 4] = [
    [2, 3, 5],  // 寅卯巳
    [0, 1, 4],  // 子辰丑
    [8, 9, 11], // 申酉亥
    [6, 6, 6],  // 午午 (自刑)
];
```

#### 2.5.1 Canonical taxonomy (per audit doc + standard Bazi/Tử Vi lineage)

The classical source-cited taxonomy has **five** categories, not "two 3-branch
groups plus one self":

1. **寅巳申 — Vô ân chi hình (無恩之刑 / "Ungrateful punishment")**

   A 3-branch **mutual** punishment group: every pair within
   `{寅, 巳, 申}` (Dần, Tỵ, Thân) punishes the other two symmetrically.

2. **丑未戌 — Trì thế chi hình (持勢之刑 / "Punishment of abusing power")**

   A 3-branch **mutual** punishment group: every pair within
   `{丑, 未, 戌}` (Sửu, Mùi, Tuất) punishes the other two symmetrically.

3. **子卯 — Vô lễ chi hình (無禮之刑 / "Impolite punishment")**

   A 2-branch **directed / asymmetric** punishment: only **子 (Tý)
   punishes 卯 (Mão)**, not the reverse. This is the most-cited source of
   confusion in the codebase: a pair comparison of `Mão` against `Tý`
   should yield `Tý → Mão` direction, and a comparison of `Tý` against
   `Mão` should also yield `Tý → Mão` (the same direction, both branches
   are involved). Treating it as a bidirectional pair is the second
   defect the audit flags (`interaction-almanac.md:69-90`).

4. **自刑 (Tự hình / Self-punishment)** — four singleton groups:
   辰辰 (Thìn-Thìn), 午午 (Ngọ-Ngọ), 酉酉 (Dậu-Dậu), 亥亥 (Hợi-Hợi).

5. **No-relation punishment** — every other pair (e.g. 子丑, 子寅, 寅卯,
   辰未, 巳午, 午未, 申酉, 戌亥, etc.) is **not** a punishment per the
   canonical taxonomy. Some schools apply weaker "graceless punishment"
   (vô lễ ở mức yếu) interpretations for incomplete triads, but the
   audit doc and the canonical Vietnamese/Chinese primary tradition
   (渊海子平, 三命通会, 子平真诠) treat those as **disputed / unavailable**.

#### 2.5.2 Mapping the current code to canonical

| Current group | Current members | Canonical group | Canonical members | Defect |
|---|---|---|---|---|
| `[2, 3, 5]` | 寅卯巳 (Dần, Mão, Tỵ) | **寅巳申** (Dần, Tỵ, Thân) | `[2, 5, 8]` | Wrong members; includes Mão which belongs to 子卯, excludes Thân. |
| `[0, 1, 4]` | 子辰丑 (Tý, Sửu, Thìn) | **丑未戌** (Sửu, Mùi, Tuất) | `[1, 7, 10]` | Wrong members; includes Tý and Thìn which are self-punishment, excludes Mùi and Tuất. |
| `[8, 9, 11]` | 申酉亥 (Thân, Dậu, Hợi) | **(invalid)** | — | This is not a valid punishment group; the three branches are unrelated to any canonical punishment. |
| `[6, 6, 6]` | 午午 (Ngọ self) | **辰, 午, 酉, 亥 self** | `[4]`, `[6]`, `[9]`, `[11]` | Only Ngọ is represented; Thìn/Dậu/Hợi self-punishment is missing. |

#### 2.5.3 Source authority

- **Audit doc `interaction-almanac.md:69-90`**: explicitly states
  "Standard three-punishment groups are conventionally 寅巳申 and 丑未戌,
  with 子卯 as a separate two-branch punishment and self-punishment for
  辰/午/酉/亥." This is the canonical framing the codebase needs to adopt.
- **Standard Bazi / Tứ Trụ primary references** (see also audit
  `interaction-almanac.md:69-90` remediation):
  - **《渊海子平》** (*Yuan Hai Zi Ping*, Song dynasty) — earliest
    systematized Bazi text; records the three classical categories
    Vô ân / Trì thế / Vô lễ + Tự hình.
  - **《三命通会》** (*San Ming Tong Hui*, Ming dynasty, Wanli era) —
    compiles all four classical categories and is the standard reference
    for the XIANGXING table.
  - **《子平真诠》** (*Zi Ping Zhen Quan*, Shen Xiaozhan / Qing dynasty) —
    徐乐吾's commentary is the standard pedagogical text in modern
    Chinese Bazi practice; records the same four categories.
  - **《滴天髓》** (*Di Tian Sui*, Liu Ji / Ming dynasty) — adds the
    "directed" (有向 / 單向) semantic distinction for 子卯, which is
    essential for the audit's typed-pair recommendation.

  Note: this brief lists these primary works by name (all are well-known
  primary Bazi references) but has not yet verified the specific chapter
  pages. **`.3` human decision should add the canonical citation
  (book, chapter, page) before close.** The audit doc itself does not
  cite a specific chapter — this gap is one of the items this brief
  records for Mike's confirmation.

#### 2.5.4 Disputed / unavailable cases — to be marked

- **Sửu–Mùi (丑未) incomplete triad**: Some schools (especially Qing-era
  Tử Vi) treat a two-branch occurrence of `丑未戌` as a partial /
  "unformed" (不成之刑 / 無形之刑) punishment and reduce severity. The
  canonical line (per 三命通会 and the audit doc) is to require the full
  triad for `Trì thế chi hình`; two-branch occurrences are
  **mark unavailable** rather than promote.
- **Dần–Tỵ (寅巳) incomplete**: same rule; mark unavailable.
- **Tý–Mão (子卯) severity**: there is sub-school debate on whether the
  harm is "soft" or "hard." Default canonical position: `Tý` harms `Mão`
  with the standard "directed" effect. Mark other direction (Mão → Tý) as
  not-a-punishment.
- **Sub-Tam-hợp cross-group interaction** (e.g. Tỵ with Thân — both in
  the same Kim tam hợp but Tỵ is also in 寅巳申): do not infer a Tỵ–Thân
  punishment; the canonical rule is each relation family is evaluated
  independently.

**Decision requested:** Confirm or amend the canonical taxonomy in §2.5.1
and the disposition of disputed cases in §2.5.4. Specific points to
confirm:

- (a) Adopt 寅巳申 / 丑未戌 / 子卯 (directed) / 自刑 (辰午酉亥) as the
  four canonical categories.
- (b) Replace the current `XIANGXING` constant in `xung_hop.rs:131-138`
  with a typed representation that distinguishes direct pair, completed
  triad, directed pair, and self-punishment (implementation in `.4`,
  not `.3`).
- (c) **Strict canonical only** (per Mike 2026-07-22): mark the disputed
  two-branch incomplete triads and any other ambiguous cases as
  `unavailable` rather than promoting them. Sub-school variants are not
  shipped in v1.
- (d) Add explicit primary-source citations (book, chapter, page) for
  each canonical claim before `.3` closes. **Primary sources confirmed
  for this decision: 《渊海子平》, 《三命通会》, 《滴天髓》.**

---

## 3. Typed representation contract (target — implemented in `.4`)

The repair plan (`REPAIR-PLAN.md:83-95`) requires every relation
contribution to carry stable, machine-readable metadata. The target
contract for a `BranchRelation` between two branches `a` and `b` is:

```rust
pub enum PunishmentKind {
    None,
    DirectedPair { aggressor: Branch, victim: Branch }, // 子卯 only
    CompletedTriad { triad: TriadElement },            // 寅巳申, 丑未戌 when 3 branches
    SelfPunishment { branch: Branch },                 // 辰午酉亥 self
    Unavailable { reason: &'static str },               // disputed / incomplete
}

pub struct BranchRelation {
    pub luc_xung: bool,                // DirectPair (Lục xung)
    pub luc_hop: bool,                 // DirectPair (Lục hợp)
    pub tuong_hai: bool,               // DirectPair (Tương hại)
    pub tam_hop_member: Option<TriadElement>, // TriadMember (None if no shared triad; None for self-branch)
    pub tam_hop_completed: bool,       // CompletedGroup — always false at pair level; true only when 3 branches are in scope
    pub tuong_hinh: PunishmentKind,    // Tương hình — typed
}
```

This removes the audit-flagged defects (`interaction-almanac.md:95-115`):

- `compute_branch_relation(i, i).tam_hop` is no longer automatically `true`;
  instead it is `TriadMember` of the input branch's element. The friendly
  reading of the relation is handled by the **advisory / semantic-graph
  layer** (separate from canonical fact), per REPAIR-PLAN.md
  "find_disposition" rule for same-branch comparison.
- `compute_branch_relation(i, i).tuong_hinh` becomes `PunishmentKind::None`
  for branches outside the four self-punishment branches, and
  `PunishmentKind::SelfPunishment { branch: i }` for Thìn/Ngọ/Dậu/Hợi
  same-branch.
- `BranchRelation::has_harmony` / `has_conflict` continue to exist but
  consult the typed fields instead of conflating membership with
  pair completion.

---

## 4. Cross-surface consistency contract

Per `interaction-almanac.md:234-243`, the four surfaces currently disagree on
how Tương hình and same-branch are handled:

| Surface | Same branch today | Tương hình today |
|---|---|---|
| `compute_branch_relation(i, i)` | tam_hop = true; tuong_hinh varies | Membership in any `XIANGXING` group |
| `compute_thai_tue(i, i)` | `Truc`, suppresses `Hinh` | `Hinh` only if `i != i` and membership |
| Advisory birth compatibility | "đồng khí" +6, before tam-hop branch | not consulted |
| Semantic day node (`fact.day.xung_hop`) | includes xung_hop (i, i) | omits `xiang_xing` entirely |
| Insight API DTO | full `luc_xung`, `tam_hop`, `liu_he`, `xiang_hai` | omits `xiang_xing` |
| Full day-fortune API DTO | full XungHop | includes `xiang_xing` |
| Personal-hour scoring | tam_hop +10 and tuong_hinh −10 if both true (e.g. Thìn self, Tý self) | per branch relation |

**Decision requested:** All surfaces must consume the canonical typed
relation from a single canonical assessment. Implementation belongs in
`.6` (introduce canonical `PersonalDayAssessment`) and `.7` (migrate
advisory and API). **`.3` only records the source-cited decision and
canonical taxonomy; the migration is downstream.**

---

## 5. Golden fixtures proposed for `.4`

These fixtures will land in
`crates/amlich-core/data/almanac/khcbppt-golden.json` (and possibly a
new `branch-relations-golden.json` file) before `.4` closes. They are
organized to make every defect the audit identifies regression-testable.

### 5.1 Lục xung pairs (12 fixtures)

For each `chi_index` in `0..11`: `expected_luc_xung == (chi_index + 6) % 12`.

### 5.2 Lục hợp pairs (12 fixtures)

For each `chi_index` in `0..11`: `expected_luc_hop` matches the Lục hợp
table. Today's golden loader
(`crates/amlich-core/src/almanac/golden_loader.rs:99-100`) only checks
`expected_luc_xung` and `expected_tam_hop`; Lục hợp / Tương hại / Tương
hình are absent and need to be added.

### 5.3 Tương hại pairs (12 fixtures)

For each `chi_index` in `0..11`: `expected_xiang_hai` matches the table.

### 5.4 Tam hợp membership (4 fixtures, one per element)

For each of the four triads, assert the three expected members and that
all other 9 branches are NOT in this triad.

### 5.5 Tương hình — canonical pairs (24 fixtures: 6 for 寅巳申, 6 for 丑未戌, 2 for 子卯, 4 self)

#### 寅巳申 (Dần-Tỵ-Thân) — 6 pairs

| Pillar branch | Day branch | Expected tuong_hinh |
|---|---|---|
| Dần (2) | Tỵ (5) | `CompletedTriad { triad: Fire }` (mutual) |
| Dần (2) | Thân (8) | `CompletedTriad { triad: Fire }` (mutual) |
| Tỵ (5) | Thân (8) | `CompletedTriad { triad: Fire }` (mutual) |
| (and the 3 reverse-direction pairs) |

#### 丑未戌 (Sửu-Mùi-Tuất) — 6 pairs

| Pillar branch | Day branch | Expected tuong_hinh |
|---|---|---|
| Sửu (1) | Mùi (7) | `CompletedTriad { triad: Earth }` (mutual) |
| Sửu (1) | Tuất (10) | `CompletedTriad { triad: Earth }` (mutual) |
| Mùi (7) | Tuất (10) | `CompletedTriad { triad: Earth }` (mutual) |
| (and the 3 reverse-direction pairs) |

#### 子卯 (Tý-Mão) — 2 fixtures (asymmetric)

| Day | Pillar | Expected tuong_hinh |
|---|---|---|
| Tý (0) | Mão (3) | `DirectedPair { aggressor: Tý, victim: Mão }` |
| Mão (3) | Tý (0) | `DirectedPair { aggressor: Tý, victim: Mão }` *(same direction)* |

#### 自刑 (Self-punishment) — 4 fixtures + 8 negatives

For each of `Thìn (4)`, `Ngọ (6)`, `Dậu (9)`, `Hợi (11)`:
- `compute_branch_relation(branch, branch).tuong_hinh` ==
  `SelfPunishment { branch }`.

For each of the other 8 branches, same-branch must be
`PunishmentKind::None` (not `SelfPunishment` and not a triad pair).

### 5.6 Negative assertions — branches that must NOT be marked tuong_hinh

The current code marks these as Tương hình but they are not:

- Dần–Mão (寅卯): NOT a punishment pair.
- Tý–Sửu (子丑): NOT a punishment pair (despite appearing in current
  `XIANGXING[1]`).
- Tý–Thìn (子辰): NOT a punishment pair.
- Thân–Dậu (申酉): NOT a punishment pair.
- Thân–Hợi (申亥): NOT a punishment pair.
- Dậu–Hợi (酉亥): NOT a punishment pair.
- Dần–Dậu (寅酉): NOT a punishment pair (different families).
- Sửu–Tý / Sửu–Tuất / Mùi–Sửu / Mùi–Tuất / Tuất–Sửu / Tuất–Mùi: each
  individual two-branch pair is `Unavailable { reason: "incomplete Trì thế triad" }`
  rather than promoted.

### 5.7 Same-branch advisory / personal-hour parity (parity fixtures)

These belong in `.8` (migrate reasoning graph) rather than `.4`, but
record here so they aren't forgotten:

- Same-branch Tý personal-hour row must not simultaneously score
  `tam_hop +10` and `tuong_hinh -10`.
- Same-branch Ngọ personal-hour row must score `tuong_hinh -10` only
  (Ngọ self-punishment).
- Same-branch Dần personal-hour row must score neither (Dần self is
  NOT a self-punishment; Tương hình only triggers if the pair is in
  寅巳申 and the other branch is also present).

---

## 6. Required source citations (open before `.3` closes)

The decision brief above names the canonical Chinese Bazi primary
references (《渊海子平》, 《三命通会》, 《子平真诠》, 《滴天髓》) but has not
yet added chapter-level citations. For `.3` to close, Mike needs to record
at minimum:

| Claim | Required citation |
|---|---|
| 寅巳申 is a mutual 3-branch punishment group | 《三命通会》[待补: 卷/页] |
| 丑未戌 is a mutual 3-branch punishment group | 《三命通会》[待补] |
| 子卯 is a directed (Tý → Mão) punishment | 《滴天髓》[待补: 卷/页] or 《子平真诠》[待补] |
| 自刑 = {辰, 午, 酉, 亥} self | 《渊海子平》[待补] |
| Six Lục xung pairs | Already universally agreed; cite《子平真诠·总论》[待补] |
| Six Lục hợp pairs | Same |
| Six Tương hại pairs | Same |
| Four Tam hợp triads | Same |
| Incomplete-triad disposition | 《三命通会》[待补] or local Vietnamese almanac tradition |

These citations should be added inline in this brief (or in a sibling
"source-citations.md") before `.3` closes. The audit doc
`interaction-almanac.md:69-90` itself does not cite a specific chapter —
that gap is one of the things this brief is asking Mike to close.

A Vietnamese-language corroboration should also be recorded; the
traditional Vietnamese lineage (Hai Thuong Lan Ong's medical almanacs,
20th-century Tử Vi / Tứ Trụ teaching texts such as *Binh Pháp Tử Vi*,
*Tứ Trụ Toàn Thư*) uses the same four-category taxonomy. Note which
Vietnamese text(s) Mike relies on for the Vietnamese almanac user base.

---

## 7. Implementation handoff to `.4`

Once `.3` closes with the canonical taxonomy confirmed, `.4` (Repair
branch-relation evaluation end to end) becomes implementable:

1. Replace the `[[usize; 3]; 4]` constant `XIANGXING` in
   `xung_hop.rs:131-138` with the typed representation in §3 (or a
   v1-compatible adapter table).
2. Update `compute_branch_relation` in
   `crates/amlich-core/src/interaction/day_person.rs:64-94` to produce
   the typed output.
3. Delete or rewrite the audit-flagged tests in
   `crates/amlich-core/src/almanac/xung_hop.rs:369-440` (which currently
   lock the disputed table) and `interaction/day_person.rs:243-255,
   :268-280` (which lock tam_hop / tuong_hinh membership).
4. Add the new golden fixtures in §5 to the golden loader, replacing the
   current `expected_luc_xung`-only schema
   (`golden_loader.rs:99-100`) with one that covers all five families.
5. Add typed `PunishmentKind` parsing/wiring to
   `almanac/thai_tue.rs:108-119` so the Hình conflict reports the same
   typed facts as the day/person matrix.
6. Update `BranchRelation::is_neutral/has_harmony/has_conflict` in
   `crates/amlich-core/src/interaction/types.rs:6-35` (and `:391-405`)
   to consume the typed fields.

Steps 5 and 6 may be deferred to `.8` (migrate reasoning graph) if
typed wiring requires the canonical assessment; `.4` focuses on the
core relation tables and day/person matrix.

---

## 8. Summary of decisions Mike must record before `.3` closes

1. **Confirm canonical taxonomy** in §2.5.1 (4 categories: 寅巳申 /
   丑未戌 / 子卯 directed / 自刑) or amend.
2. **Confirm disposition of disputed cases** in §2.5.4 (mark
   `Unavailable` rather than infer).
3. **Confirm `BranchRelation` typed contract** in §3, or specify changes.
4. **Confirm cross-surface parity expectation** in §4.
5. **Approve golden fixture plan** in §5.
6. **Add primary-source citations** in §6 (chapter / page / book for
   every canonical claim).
7. **Approve handoff** to `.4` per §7.

Once these seven are recorded with Mike's approval, `.3` can close and
`.4` can begin implementation against frozen source-cited semantics.