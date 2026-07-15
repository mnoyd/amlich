# Amlich Almanac Correctness Audit

## Current State

The project has shipped seven milestones:

- `v1.0` KHCBPPT alignment complete (full validator + zero-divergence audit cycle).
- `v1.1` Foundation extensions complete and accepted (Xung Hop extensions, Tang Can, Tiet Khi regression fix).
- `v1.2` Ten Gods and Kua Foundation complete (deterministic calculators, typed API, DayFortune integration).
- `v1.3` Dai Van Core complete (core algorithm, helper contracts, Kua analysis, synchronized verification).
- `v1.4` Lunar Engine Table Parity complete (hour-pillar parity, full 60-cycle parity, Na Am API contracts).
- `v1.5` Eastern Knowledge Expansion complete (Văn khấn `vn-folk-ritual` corpus + lookup APIs; Phi Tinh `huyen-khong` Vận/Niên/Nguyệt overlays + 81-cell aspects + safety hints; additive `DaySnapshot` integration; 886 tests pass).
- `v1.6` Eastern Knowledge Completion complete (daily Phi Tinh 日紫白 layer; `RecommendsOffering` first-class semantic-graph node; v1.5 review/confidence tech debt closed — RIT-11 reviewer field across 60 entries + ADR-0003 pre-1984 confidence boost; 922 tests pass).

Canonical status and acceptance evidence are archived in milestone artifacts:

- `.planning/milestones/v1.1-ROADMAP.md`
- `.planning/milestones/v1.1-REQUIREMENTS.md`
- `.planning/milestones/v1.1-MILESTONE-AUDIT.md`
- `.planning/milestones/v1.2-ROADMAP.md`
- `.planning/milestones/v1.2-REQUIREMENTS.md`
- `.planning/milestones/v1.4-ROADMAP.md`
- `.planning/milestones/v1.4-REQUIREMENTS.md`
- `.planning/milestones/v1.4-MILESTONE-AUDIT.md`
- `.planning/milestones/v1.5-ROADMAP.md`
- `.planning/milestones/v1.5-REQUIREMENTS.md`
- `.planning/milestones/v1.5-MILESTONE-AUDIT.md`
- `.planning/milestones/v1.6-ROADMAP.md`
- `.planning/milestones/v1.6-REQUIREMENTS.md`
- `.planning/milestones/v1.6-MILESTONE-AUDIT.md`

## Core Value

Every almanac subsystem in amlich must produce output that matches its canonical classical source (KHCBPPT for the original engine; `vn-folk-ritual` for ritual text; *Thẩm Thị Huyền Không Học* for Phi Tinh) for the 2020-2030 date range, with test-backed and traceable evidence. v1.5 expanded "canonical source" from a single text to a registered taxonomy of source_ids, each enforced by module-level `pub const` and CI grep guards.

## Validated Capabilities (after v1.6)

- ✓ KHCBPPT-aligned core calendar (v1.0–v1.1)
- ✓ Ten Gods + Kua + Dai Van calculators (v1.2–v1.3)
- ✓ Hour pillar + 60-cycle + Na Am parity (v1.4)
- ✓ Văn khấn corpus + lookup APIs (v1.5, `vn-folk-ritual`)
- ✓ Phi Tinh Vận/Niên/Nguyệt overlays + 81-cell aspects + safety hints (v1.5, `huyen-khong`)
- ✓ Semantic graph wiring with dual-provenance Direction node and additive `DaySnapshot` integration (v1.5)
- ✓ Daily Phi Tinh (日紫白) layer — `compute_daily_flying_stars` with 冬至/夏至 reversal (v1.6, `huyen-khong`)
- ✓ `RecommendsOffering` first-class semantic-graph node with dual-source edge provenance (v1.6)
- ✓ ADR-0003 pre-1984 confidence closure + 60-entry Văn khấn reviewer field closure (v1.6)

## Out of Scope (carry-forward)

- **P3 Y học, P6 Tử Vi** — deferred per Expansion Framework tiering (P2 Kinh Dịch is the current milestone).
- **P5 Spatial Phi Tinh / `spatial_compose`** — requires user spatial input (sit/face direction); explicit CRIT-3 isolation forbids wiring `FlyingStar` into `interaction/direction_merge.rs`.

## Current Milestone: v1.7 Kinh Dịch (I-Ching Divination)

**Goal:** Add the P2 Kinh Dịch pillar — Mai Hoa Dịch Số casting + 64-hexagram lookup — as a new Tier-0 reasoning capability, plus the Thái Tuế/Tam Sát directional cross-link (read-only reasoning join, a carry-forward "should-have" from v1.5 research).

**Target features:**
- **P2 Kinh Dịch (Mai Hoa Dịch Số)** — Tier-0 divination: cast a hexagram (quẻ) from the query time via Mai Hoa time-number method; resolve the 64-hexagram table (thoán từ / hào từ) with cát/hùng interpretation; integrate as a `ConsultationIntent::IChing` evaluator branch in `reasoning/personal.rs`. New `source_id: kinh-dich` (Ngô Tất Tố) + `mai-hoa-dich-so` (Thiệu Khang Tiết).
- **Biến Quẻ (transforming hexagram)** — derive the biến quẻ from động hào (moving line) for the cát-hùng-over-time reading.
- **Thái Tuế / Tam Sát ⇄ Phi Tinh cross-link** — read-only reasoning-layer join surfacing both the KHCBPPT directional warnings (`thai_tue`/`tam_sat`) and the `huyen-khong` palace layout in one directional picture (no CRIT-3 boundary merge — distinct source_ids, joined only in the reasoning envelope).
- **Semantic-graph + DTO integration** — `Hexagram` node + `LocatedAt`/`Transforms` edges; additive `DaySnapshot` / reasoning surfaces; backward-compat round-trip preserved.

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
| Additive-only integration changes | Preserve backward compatibility while extending outputs | ✓ Confirmed in v1.2 |
| Hour pillar parity via fixed slot + seed group model | Stable deterministic mapping across 12 windows and day-stem groups | ✓ Confirmed in v1.4 |
| Sexagenary inversion via CRT-based formula | Correct roundtrip mapping between cycle index and stem-branch pairs | ✓ Confirmed in v1.4 |
| Na Am API contracts with typed deterministic errors | Stable schema and explicit invalid-input handling for pair/index lookups | ✓ Confirmed in v1.4 |
| Source-ID taxonomy as `pub const &str` (not enum) | New traditions register without enum churn; CI grep guard prevents bare-literal drift | ✓ Confirmed in v1.5 (DEC-0023) |
| Schema-lock before corpus authoring | Re-editing 60 corpus entries after a schema slip is prohibitively expensive (PITFALLS CRIT-1/5) | ✓ Confirmed in v1.5 (Phase 10 → 12 ordering) |
| ADR-0001: `RitualEntry` JSON schema v1 with `deny_unknown_fields` | Frozen 10-type schema gives corpus authors a stable target | ✓ Confirmed in v1.5 |
| ADR-0002: solar-term boundaries for monthly Phi Tinh | Reuses v1.1.2 Tiết Khí scanner per *Thẩm Thị Huyền Không Học* convention | ✓ Confirmed in v1.5 |
| ADR-0003: Niên Tử Bạch polarity matrix (Tam Nguyên × year polarity) | Explicit (yuan, polarity) → (start, direction) table over arithmetic; 1960 divergence resolved by tiebreak | ✓ §§1–5 authoritative; §6 superseded by ADR-0003a (v1.6 — pre-1984 rows HIGH after dual-source independent secondary modern verification; 1960 case-level center-value split PendingExternalReview) |
| Additive `Option<T>` `DaySnapshot` fields (no `deny_unknown_fields`) | v1.4 producer payloads still deserialize cleanly; v1.5 consumers see new fields when present | ✓ Confirmed in v1.5 (INT-05 round-trip) |
| CRIT-3 isolation: `FlyingStar` never wired into `direction_merge.rs` | Keeps `huyen-khong` palace layouts disjoint from `khcbppt` `sát_phương`/`thần_hướng` until Tier-3 `spatial_compose` lands | ✓ Confirmed in v1.5 (grep-verified by audit) |
| Center star carries Ngũ Hành on aggregate FlyingStar node | `CarriesElement` edge gives the FlyingStar node both spatial (palace) and elemental handles per *Thẩm Thị Huyền Không Học* | ✓ Confirmed in v1.5 post-audit (commit 3e6a148) |
| ADR-0003a: pre-1984 confidence boost via dual-source verification | Thượng/Trung Nguyên polarity rows MEDIUM → HIGH after independent secondary modern cross-check; 1960 divergence `PendingExternalReview` | ✓ Confirmed in v1.6 (FND-07/08) |
| ADR-0004: daily Phi Tinh starting-star convention | 6 Trung Khí pivots with Dương thuận / Âm nghịch (intentionally opposite annual polarity); Giáp-Tý-as-seed with prior-pivot fall-back | ✓ Confirmed in v1.6 (FS-16/17) |
| Typed `DeferralMarker` / `ExternalReviewPending` schema fields | By-design domain-expert deferrals tracked in-code, not silently corrected; due 2026-12-31 | ✓ Confirmed in v1.6 (FND-08, RIT-14) |
| `Offering` first-class node + `RecommendsOffering` edge | Promotes offerings from flat string list to graph-native with dual-source provenance reusing v1.5 dedup logic | ✓ Confirmed in v1.6 (INT-07/08/09) |
| `SourceId = String` transparent alias (not true newtype) | Preserves DEC-0023 const discipline while satisfying INT-07 literal text; future phases may tighten | — Pending (documented decision, future-tightenable) |

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

---
*Last updated: 2026-07-16 — v1.7 Kinh Dịch milestone initialised (P2 pillar + Thái Tuế cross-link).*
