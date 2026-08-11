# Amlich Almanac Correctness Audit

## Current State

The project has shipped nine milestones:

- `v1.0` KHCBPPT alignment complete (full validator + zero-divergence audit cycle).
- `v1.1` Foundation extensions complete and accepted (Xung Hop extensions, Tang Can, Tiet Khi regression fix).
- `v1.2` Ten Gods and Kua Foundation complete (deterministic calculators, typed API, DayFortune integration).
- `v1.3` Dai Van Core complete (core algorithm, helper contracts, Kua analysis, synchronized verification).
- `v1.4` Lunar Engine Table Parity complete (hour-pillar parity, full 60-cycle parity, Na Am API contracts).
- `v1.5` Eastern Knowledge Expansion complete (Văn khấn `vn-folk-ritual` corpus + lookup APIs; Phi Tinh `huyen-khong` Vận/Niên/Nguyệt overlays + 81-cell aspects + safety hints; additive `DaySnapshot` integration; 886 tests pass).
- `v1.6` Eastern Knowledge Completion complete (daily Phi Tinh 日紫白 layer; `RecommendsOffering` first-class semantic-graph node; v1.5 review/confidence tech debt closed — RIT-11 reviewer field across 60 entries + ADR-0003 pre-1984 confidence boost; 922 tests pass).
- `v1.7` Kinh Dịch (I-Ching Divination) complete (P2 Kinh Dịch pillar: Mai Hoa Dịch Số casting + Biến Quẻ + Thể/Dụng + 64-hexagram Ngô Tất Tố corpus + `IChingEvaluator` Tier-0 reasoning; Thái Tuế / Tam Sát ⇄ Phi Tinh read-only directional cross-link; 1120 tests pass; zero new crate dependencies).
- `v1.8` Surface & Debt Closure complete (desktop/TUI explanation surfaces; user-facing I Ching and direction cross-link; warning-free workspace; true `SourceId` newtype; external-review lifecycle; 9/9 requirements).

Canonical status and acceptance evidence are archived in milestone artifacts:

- `.planning/milestones/v1.1-ROADMAP.md` / `v1.1-REQUIREMENTS.md` / `v1.1-MILESTONE-AUDIT.md`
- `.planning/milestones/v1.2-ROADMAP.md` / `v1.2-REQUIREMENTS.md`
- `.planning/milestones/v1.4-ROADMAP.md` / `v1.4-REQUIREMENTS.md` / `v1.4-MILESTONE-AUDIT.md`
- `.planning/milestones/v1.5-ROADMAP.md` / `v1.5-REQUIREMENTS.md` / `v1.5-MILESTONE-AUDIT.md`
- `.planning/milestones/v1.6-ROADMAP.md` / `v1.6-REQUIREMENTS.md` / `v1.6-MILESTONE-AUDIT.md`
- `.planning/milestones/v1.7-ROADMAP.md` / `v1.7-REQUIREMENTS.md` / `v1.7-MILESTONE-AUDIT.md` (audit produced 2026-07-20 as retrospective backfill — status `tech_debt`; 15/15 satisfied, 0 gaps)
- `.planning/milestones/v1.8-ROADMAP.md` / `v1.8-REQUIREMENTS.md` / `v1.8-MILESTONE-AUDIT.md` (9/9 requirements, 6/6 release gates)

## Core Value

Every almanac subsystem in amlich must produce output that matches its canonical classical source (KHCBPPT for the original engine; `vn-folk-ritual` for ritual text; *Thẩm Thị Huyền Không Học* for Phi Tinh; *Kinh Dịch Trọn Bộ* (Ngô Tất Tố) for hexagram text; *Mai Hoa Dịch Số* (Thiệu Khang Tiết) for Mai Hoa casting) for the 2020-2030 date range, with test-backed and traceable evidence. v1.5 expanded "canonical source" from a single text to a registered taxonomy of source_ids, each enforced by module-level `pub const` and CI grep guards; v1.7 added the Kinh Dịch / Mai Hoa Dịch Số pair as the first non-almanac-classical-text sources.

## Validated Capabilities (after v1.8)

- ✓ KHCBPPT-aligned core calendar (v1.0–v1.1)
- ✓ Ten Gods + Kua + Dai Van calculators (v1.2–v1.3)
- ✓ Hour pillar + 60-cycle + Na Am parity (v1.4)
- ✓ Văn khấn corpus + lookup APIs (v1.5, `vn-folk-ritual`)
- ✓ Phi Tinh Vận/Niên/Nguyệt overlays + 81-cell aspects + safety hints (v1.5, `huyen-khong`)
- ✓ Semantic graph wiring with dual-provenance Direction node and additive `DaySnapshot` integration (v1.5)
- ✓ Daily Phi Tinh (日紫白) layer — `compute_daily_flying_stars` with 冬至/夏至 reversal (v1.6, `huyen-khong`)
- ✓ `RecommendsOffering` first-class semantic-graph node with dual-source edge provenance (v1.6)
- ✓ ADR-0003 pre-1984 confidence closure + 60-entry Văn khấn reviewer field closure (v1.6)
- ✓ Mai Hoa Dịch Số casting — `cast_mai_hoa` (pure deterministic; CRIT-2 boundary-safe `((n-1)%k)+1`) + `derive_bien_que` (CRIT-4 384-case contract) + `classify_the_dung` Ngũ Hành sinh/khắc + CatHung verdict (v1.7, `mai-hoa-dich-so`)
- ✓ 64-hexagram Ngô Tất Tố corpus — `data/iching/hexagrams.json` NFC-normalised + reviewer-signed + `PendingExternalReview` for source gaps; lazy `OnceLock` loader (v1.7, `kinh-dich`)
- ✓ `IChingEvaluator` + `IChingQuery` sibling-newtype Tier-0 reasoning (no birth data required, MOD-7) with 4-envelope evidence vector (3 primitives + 1 composite `rule.composite.iching_consultation`) — CRIT-6 (v1.7)
- ✓ Thái Tuế directional (`thai_tue_direction` year-only sibling) + classical 3-direction Tam Sát module (`almanac/tam_sat.rs`) + read-only `build_direction_cross_link` composite surfacing KHCBPPT + Huyền-Không in one picture (v1.7, `khcbppt` + `huyen-khong` + `rule.composite.direction_cross_link`)
- ✓ Semantic-graph Hexagram nodes (chu + biến) wired via `LocatedAt` / `Transforms` edges + composite Direction fact node (v1.7, INT-11)
- ✓ Additive `DaySnapshot.iching_cast` + `DaySnapshot.direction_cross_link` with combined-strip v1.6→v1.7 backward-compat round-trip (v1.7, INT-12)
- ✓ Runtime-invariant baseline guards: cargo dep-tree shape locked (`cargo_dependency_tree_unchanged_from_v16`) + INT-13 cross-source discipline locked (`int13_golden_dataset_cross_source_discipline_holds`) (v1.7 Phase 25)
- ✓ Desktop/TUI explanation surfaces, desktop I Ching + directional cross-link projection, typed `SourceId`, and external-review lifecycle (v1.8)

## Active Milestone: v1.10 Traditional Wellness Context (Tier 0)

The next milestone is now defined from primary-source research. It delivers a
source-attributed, non-clinical context for a selected local date and time:

- the historical **Twelve-Branch Channel Association** (`十二經納地支`), using
  neutral association language and a disclosed local-civil-hour basis; and
- four *Huangdi Neijing Suwen* seasonal cultivation profiles joined
  transparently to the 24 solar terms through the existing calendar engine.

The fixed association is not full **Tý Ngọ Lưu Chú**. The day/hour
acupuncture-point opening method, all points and procedures, clinical claims,
and Bazi personalization remain explicitly deferred. See the active
[`v1.10 requirements`](milestones/v1.10-REQUIREMENTS.md),
[`research note`](research/LUNAR_HEALTH_RESEARCH.md), and
[`scope ADR`](../docs/adr/0003-separate-branch-channel-association-from-ty-ngo-luu-chu.md).

## Out of Scope (carry-forward)

- **Full Tý Ngọ Lưu Chú / `納甲法`** — requires a separate source, point-opening policy, safety review, and clinical/procedural boundary.
- **P6 Tử Vi** — deferred per Expansion Framework tiering; candidate for a future milestone.
- **P5 Spatial Phi Tinh / `spatial_compose`** — requires user spatial input (sit/face direction); explicit CRIT-3 isolation forbids wiring `FlyingStar` into `interaction/direction_merge.rs`. Deferred until Tier-3 landing.
- **Hỗ Quả (nuclear hexagram)** — depth feature; defer to v1.10+.
- **Tier-2 Bazi enrichment of hexagram reading** — v1.7 ships Tier-0 baseline only; mirrors v1.5 Phi Tinh T0/T2 split.
- **User-selectable casting variants (số vật / âm thanh)** — out of scope; v1.7 ships Mai Hoa time-numerology only.
- **Coin / yarrow / RNG casting** — different tradition; breaks determinism; would need a third `source_id`.

## Completed Milestone: v1.8 Surface & Debt Closure

**Goal:** Land v1.7's backend power (IChing pillar + Thái Tuế/Tam Sát cross-link) into the desktop + TUI surfaces, close the two in-flight P1/P2 UX epics (`amlich-00j` desktop observatory, `amlich-5no` TUI explanation views), and retire the engineering debt carried since v1.5.

**Theme:** Surface what v1.7 built. Close epics that are 80%+ done. Pay down pre-existing engineering debt before adding new backend pillars.

**Result:** Shipped 2026-08-10. All three tracks, all nine requirements, and
all six release gates passed. See
`.planning/milestones/v1.8-MILESTONE-AUDIT.md`.

**Target tracks:**

- **Desktop Observatory closure** — finish `amlich-00j` epic (Evidence Graph workspace `amlich-01mx` + quality gates `amlich-2nqy`); get v1.7 IChing + cross-link payloads user-visible in Observatory workspaces.
- **TUI Explanation Views closure** — finish `amlich-5no` epic (Yếu Tố lens `amlich-0qv` + rendering/nav tests `amlich-jet`); default non-dev TUI shows decision-first "Vì Sao Kết Luận" with four Vietnamese-labelled lenses.
- **Engineering debt phase** — `amlich-081` (~96 clippy warnings) + `SourceId = String` → true newtype + documented `[PendingExternalReview]` workflow for the four carry-forward domain-expert deferrals.

**Out of scope (locked, deferred to v1.9+):**

- P3 Y học Tý Ngọ Lưu Chú (next backend pillar candidate per framework §5)
- P6 Tử Vi Đẩu Số (XL-scope; v2.0 candidate)
- P5 Spatial Phi Tinh (Tier-3 `spatial_compose` blocked on new DEC)
- Hỗ Quả / nuclear hexagram depth
- Tier-2 Bazi enrichment of hexagram reading
- LLM free-form interpretation
- Domain-expert text resolution for 64-hexagram `[PendingExternalReview]` markers (external dependency — v1.8 only tightens the workflow, does not resolve the deferrals)

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| KHCBPPT as sole reference | Most authoritative classical text for Vietnamese almanac | ✓ Confirmed in v1.0 |
| Golden dataset + validator harness | Enables repeatable, test-backed correctness verification | ✓ Confirmed in v1.0 |
| Verification artifact as status authority | Prevents roadmap/state drift from acceptance truth | ✓ Confirmed in v1.1.1 |
| Real term-boundary scan for nearest Tiet Khi | Removes synthetic approximation regressions and stabilizes signed distances | ✓ Confirmed in v1.1.2 |
| Ten Gods mapping via five-element + polarity | Explicit mapping table over arithmetic shortcuts for audit readability | ✓ Confirmed in v1.2 |
| Kua calculator solar year basis | Vietnamese feng-shui convention using Gregorian calendar | ✓ Confirmed in v1.2 |
| Kua 5 resolution (male→8, female→2) | Frozen project policy for consistent output | ✓ Confirmed in v1.2 |
| Additive-only integration changes | Preserve backward compatibility while extending outputs | ✓ Confirmed in v1.2 (re-validated v1.5/v1.6/v1.7) |
| Hour pillar parity via fixed slot + seed group model | Stable deterministic mapping across 12 windows and day-stem groups | ✓ Confirmed in v1.4 |
| Sexagenary inversion via CRT-based formula | Correct roundtrip mapping between cycle index and stem-branch pairs | ✓ Confirmed in v1.4 |
| Na Am API contracts with typed deterministic errors | Stable schema and explicit invalid-input handling for pair/index lookups | ✓ Confirmed in v1.4 |
| Source-ID taxonomy as `pub const &str` (not enum) | New traditions register without enum churn; CI grep guard prevents bare-literal drift | ✓ Confirmed in v1.5 (DEC-0023) |
| Schema-lock before corpus authoring | Re-editing corpus entries after a schema slip is prohibitively expensive (CRIT-1/5) | ✓ Confirmed in v1.5 (Phase 10 → 12); re-applied v1.7 Phase 20 → 21 (CRIT-1 × 7 amplification — 448 corpus fields) |
| ADR-0001: `RitualEntry` JSON schema v1 with `deny_unknown_fields` | Frozen 10-type schema gives corpus authors a stable target | ✓ Confirmed in v1.5 |
| ADR-0002: solar-term boundaries for monthly Phi Tinh | Reuses v1.1.2 Tiết Khí scanner per *Thẩm Thị Huyền Không Học* convention | ✓ Confirmed in v1.5 |
| ADR-0003: Niên Tử Bạch polarity matrix (Tam Nguyên × year polarity) | Explicit (yuan, polarity) → (start, direction) table over arithmetic; 1960 divergence resolved by tiebreak | ✓ §§1–5 authoritative; §6 superseded by ADR-0003a (v1.6) |
| Additive `Option<T>` `DaySnapshot` fields (no `deny_unknown_fields`) | v1.4 producer payloads still deserialize cleanly; later consumers see new fields when present | ✓ Confirmed in v1.5 (INT-05 round-trip); re-validated v1.6 INT-10 + v1.7 INT-12 (combined-strip round-trip) |
| CRIT-3 isolation: `FlyingStar` never wired into `direction_merge.rs` | Keeps `huyen-khong` palace layouts disjoint from `khcbppt` `sát_phương`/`thần_hướng` until Tier-3 `spatial_compose` lands | ✓ Confirmed in v1.5; v1.7 extended with sibling `tests/thai_tue_cross_link_crit3.rs` covering `reasoning/direction_composite.rs` |
| Center star carries Ngũ Hành on aggregate FlyingStar node | `CarriesElement` edge gives the FlyingStar node both spatial (palace) and elemental handles per *Thẩm Thị Huyền Không Học* | ✓ Confirmed in v1.5 post-audit (commit 3e6e148) |
| ADR-0003a: pre-1984 confidence boost via dual-source verification | Thượng/Trung Nguyên polarity rows MEDIUM → HIGH after independent secondary modern cross-check; 1960 divergence `PendingExternalReview` | ✓ Confirmed in v1.6 (FND-07/08) |
| ADR-0004: daily Phi Tinh starting-star convention | 6 Trung Khí pivots with Dương thuận / Âm nghịch (intentionally opposite annual polarity); Giáp-Tý-as-seed with prior-pivot fall-back | ✓ Confirmed in v1.6 (FS-16/17) |
| Typed `DeferralMarker` / `ExternalReviewPending` schema fields | By-design domain-expert deferrals tracked in-code, not silently corrected; due 2026-12-31 | ✓ Confirmed in v1.6 (FND-08, RIT-14); reused v1.7 (AF-05 hexagram corpus + Tam Sát page citation) |
| `Offering` first-class node + `RecommendsOffering` edge | Promotes offerings from flat string list to graph-native with dual-source provenance reusing v1.5 dedup logic | ✓ Confirmed in v1.6 (INT-07/08/09) |
| `SourceId` true newtype with transparent serde | Preserve the wire string while preventing unrelated Rust strings from crossing source boundaries | ✓ Confirmed in v1.8 (`amlich-t757.1`) |
| ADR-0005: `HexagramEntry` JSON schema v1 with `deny_unknown_fields` | CRIT-1 schema-lock-first gate for 64-hexagram corpus authoring (× 7 amplification — 448 text fields) | ✓ Confirmed in v1.7 (FND-11; Phase 20 → 21) |
| ADR-0006: Mai Hoa casting convention (Tiên Thiên arrangement + lunar input + `((n-1)%k)+1`) | Pins Thiệu Khang Tiết arrangement + CRIT-2 remainder-zero boundary-safe reduction; classical + modern (nhantu.net) two-source pin | ✓ Confirmed in v1.7 (FND-10; ICH-02 boundary test) |
| ADR-0007: cross-link CRIT-3 carve-out (`reasoning/direction_composite.rs` + composite `rule.composite.direction_cross_link` envelope) | Read-only placement preserves CRIT-3 isolation; composite envelope is the only pattern compatible with the grep guard | ✓ Confirmed in v1.7 (FND-10; XLK-03 closure) |
| Three CRIT-3-isolating newtypes (`TienThienTrigram` / `HauThienTrigram` / `KingWenHexagram`) with NO cross-`From` impls | Compiler-enforced boundary between Mai Hoa Tiên Thiên numbers and King Wen hexagram numbers (different mappings, shared "1..N" form) | ✓ Confirmed in v1.7 (FND-11; CRIT-3 prevention at type level) |
| Sibling-newtype query (`IChingQuery`) + evaluator (`IChingEvaluator`) over closed-enum extension | Adding `ConsultationIntent::IChing` variant would force ~25-43 call-site `Copy`-break churn; mirrors v1.6 `DailyFlyingStarLayout` precedent | ✓ Confirmed in v1.7 (ICH-05; Phase 24-01) |
| Per-step evidence envelope + composite (CRIT-6) | Each step in IChing derivation remains individually traceable; composite does NOT collapse primitives; locked 4-envelope vector | ✓ Confirmed in v1.7 (ICH-05; contract test pins `mai-hoa-dich-so` + `kinh-dich` + `rule.composite.iching_consultation`) |
| Runtime-built needle patterns for grep guards | Test source code containing the forbidden literal would self-trip the guard; needles built via `String::from(...).push(...)` so the test's own source code is clean | ✓ Confirmed in v1.7 (Phase 22-02 / 23-03 / 24-01 / 24-02) — established as canonical pattern |
| Tam Sát page-citation deferral (`PendingExternalReview` in `data/almanac/tam_sat_provenance.md`) | Honesty over fabrication — locked rule + mapping shipped, exact KHCBPPT edition/page pin deferred rather than invented | ✓ Confirmed in v1.7 (XLK-02; mirrors ADR-0006 §5 page-citation deferral pattern) |
| Tier-0 `ActionEvaluator` adapter returns empty evaluation (MOD-7) | Rich `IChingEvaluation` lives behind `IChingEvaluator::evaluate_consultation`; generic trait surface stays minimal | ✓ Confirmed in v1.7 (ICH-05; mirrors `InitiationOpeningEvaluator`) |
| Runtime-invariant baseline guards (`cargo_dependency_tree_unchanged_from_v16` + `int13_golden_dataset_cross_source_discipline_holds`) | Defense-in-depth — even if a future PR weakens the loader's own assertions or adds a new dep, these guards trip | ✓ Confirmed in v1.7 (Phase 25 SC1 + SC4) |

<details>
<summary>Archived initialization snapshot (pre-v1.1)</summary>

- Original scope centered on foundational KHCBPPT alignment and baseline correctness audit.
- Original active checklist has been superseded by shipped v1.0/v1.1 milestone artifacts.

</details>

<details>
<summary>Archived v1.4 milestone scope</summary>

- Goal: reach deterministic table-level parity for hour pillar and 60-cycle calculations, then expose Na Am API surfaces with evidence-backed outputs.
- Delivered features: hour pillar parity, full sexagenary 60-cycle parity, Na Am pair/index APIs, and contract validators.

</details>

## Known Gaps (after v1.9; v1.10 external gates are separate)

External-review gaps follow the canonical lifecycle in
[`docs/architecture/external-review-lifecycle.md`](../docs/architecture/external-review-lifecycle.md).

- **64-hexagram Ngô Tất Tố interpretive text** (AF-05) — structural fields populated; `thoai_tu` / `hao_tu` / `cat_hung` remain pending domain-expert verification in `crates/amlich-core/data/iching/provenance_audit.md`.
- **Tam Sát KHCBPPT page-level citation** — the locked rule and mapping remain operational while the exact edition/page pin awaits external review.
- **1960 Trung Nguyên and ADR-0004 page-pin reviews** — provisional behavior remains explicitly bounded and registered in the lifecycle document.

---

*Last updated: 2026-08-11 — v1.10 Traditional Wellness Context defined; v1.9 milestone audit complete*
