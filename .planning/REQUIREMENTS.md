# Requirements: Amlich v1.8 — Surface & Debt Closure

**Defined:** 2026-07-20
**Completed:** 2026-08-10
**Core Value:** Every almanac subsystem in amlich must produce output matching its canonical classical source for 2020-2030 with test-backed, traceable evidence.
**Milestone Goal:** Land v1.7's backend power (IChing pillar + Thái Tuế/Tam Sát ⇄ Phi Tinh cross-link) into the desktop + TUI user-facing surfaces, close the two in-flight P1/P2 UX epics (`amlich-00j` desktop observatory, `amlich-5no` TUI explanation views), and retire the engineering debt that has been carried unchanged across v1.5 → v1.6 → v1.7.

**Sequencing notes:** v1.7 RETROSPECTIVE lesson #9 ("Pre-existing engineering debt does not self-heal") and lesson #10 ("Run the audit BEFORE closing the milestone") are load-bearing for this milestone. v1.8 is intentionally NOT adding a new backend pillar (P3 Y học / P6 Tử Vi / P5 Spatial Phi Tinh all deferred) — it closes what is in flight before opening new pillars.

REQs start fresh categories for v1.8 (OBS / EXP / DEBT); prior categories FND / RIT / FS / INT / ICH / XLK are closed under v1.7 archive.

## v1.8 Requirements

### OBS — Desktop Observatory Closure (amlich-00j epic)

- [x] **OBS-01**: Desktop user can open the **Evidence Graph workspace** in the Amlich Observatory shell — a readable list/tree view of nodes, edges, evidence envelopes, axis scores, support/resistance/override factors, and source families — with raw debug graph data available behind a developer-oriented lens. *(Closed by `amlich-01mx`.)*
- [x] **OBS-02**: Desktop user can see v1.7 IChing + Thái Tuế / Tam Sát cross-link payloads surfaced through at least one Observatory workspace (Day Console, Almanac Inspector, or Evidence Graph) — no v1.7 backend field is left without a user-facing surface. *(Closed by `amlich-t757.3`.)*
- [x] **OBS-03**: Maintainer can run the desktop redesign quality-gate suite (lint, type-check, build, smoke) and it passes clean before the `amlich-00j` epic is closed. *(Closed by `amlich-2nqy`; re-verified by `amlich-t757.4`.)*

### EXP — TUI Explanation Views Closure (amlich-5no epic)

- [x] **EXP-01**: TUI user can press `Tab` to cycle the four explanation lenses — **Vì sao → Yếu tố → Hoạt động → Nguồn** — with the current Causality Map preserved as the user-facing "Yếu Tố" lens (Vietnamese labels: *Nhóm*, *Kiểu yếu tố*, *Liên kết*) and the raw Semantic Graph Inspector remaining available only behind debug mode. *(Closed by `amlich-0qv` and `amlich-5no`.)*
- [x] **EXP-02**: Maintainer can run focused TUI rendering + navigation tests covering: default "Vì Sao Kết Luận" view, lens cycling (Vì sao / Yếu tố / Hoạt động / Nguồn), debug-mode entry (`d` key), and small-layout rendering without panic — all passing. Existing debug-inspector tests continue to pass. *(Closed by `amlich-jet` and `amlich-5no`.)*

### DEBT — Engineering Debt Closure

- [x] **DEBT-01**: Maintainer can run `cargo clippy --workspace --all-targets -- -W clippy::all` and see **ZERO warnings** across `amlich-core` and `amlich-tui` (resolves the ~96-warning carry-forward from v1.5). *(Closed by `amlich-081`; strict `-D warnings` gate re-verified by `amlich-t757.4`.)*
- [x] **DEBT-02**: Maintainer can run `cargo fmt --check` clean across the workspace. *(Verified by `amlich-t757.4`.)*
- [x] **DEBT-03**: User-of-sources can find `SourceId` as a **true newtype wrapper** (`pub struct SourceId(String)` with explicit constructor + accessor + `Display`/`AsRef<str>` impls) rather than the v1.6 transparent `pub type SourceId = String` alias, with all existing call-sites updated and `tests/source_id_guard.rs` still passing. *(Closed by `amlich-t757.1`.)*
- [x] **DEBT-04**: Maintainer can read one canonical document describing the **`[PendingExternalReview]` / `ExternalReviewPending` / `DeferralMarker` lifecycle** — what triggers a deferral, where it is recorded (corpus ledger vs ADR vs typed schema field), what the due-date discipline is, and how a deferral is later resolved or escalated — covering the four active cases (64-hexagram Ngô Tất Tố text, Tam Sát KHCBPPT page citation, 1960 Trung Nguyên polarity split, ADR-0004 daily-page citation). *(Closed by `amlich-t757.2`.)*

## Out of Scope

| Feature | Reason |
|---------|--------|
| P3 Y học Tý Ngọ Lưu Chú | Next backend pillar per EXPANSION_FRAMEWORK §5; deferred to v1.9+ so v1.8 can close in-flight UX epics first. |
| P5 Spatial Phi Tinh (Tier-3 `spatial_compose`) | Requires new DEC for `SpatialInput` model; framework §5 says P1–P4 must stabilize first. CRIT-3 carve-out is load-bearing until a superseding ADR lands. |
| P6 Tử Vi Đẩu Số | XL scope; needs its own multi-phase milestone (An Sao + 12 cung + Tứ Hóa + ≥2 cross-check sources per §7). v2.0 candidate. |
| Hỗ Quả (nuclear hexagram) (DF-03) | Depth feature on a v1.7 surface not yet user-exposed; defer to v1.9+ after v1.8 surfaces land. |
| Tier-2 Bazi enrichment of hexagram reading (DF-01) | Depth on v1.7 surface; same rationale as DF-03. |
| LLM free-form interpretation (AF-02) | Violates source provenance (DEC-0015/0016); Ngô Tất Tố corpus IS the interpretation. |
| Domain-expert text resolution for 64-hexagram `[PendingExternalReview]` markers | External dependency — requires Ngô Tất Tố domain-expert verification. v1.8 only tightens the *workflow* around the markers (DEBT-04), not the markers themselves. |
| Tam Sát KHCBPPT page-level citation resolution | External dependency — `PendingExternalReview` per ADR-0006 §5. v1.8 only documents the workflow. |
| Coin / yarrow / RNG IChing casting (AF-01) | Different tradition; breaks determinism; would need a third `source_id`. v1.7 ships Mai Hoa time-numerology only. |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| OBS-01 | `amlich-01mx` | Complete — Evidence Graph workspace with user and developer lenses |
| OBS-02 | `amlich-t757.3` | Complete — Almanac Inspector surfaces I Ching and direction cross-link DTOs |
| OBS-03 | `amlich-2nqy`, `amlich-t757.4` | Complete — desktop Rust tests, type-check, and production build pass |
| EXP-01 | `amlich-0qv`, `amlich-5no` | Complete — four user lenses and debug-only raw inspector |
| EXP-02 | `amlich-jet`, `amlich-5no` | Complete — navigation, debug entry, and small-layout tests pass |
| DEBT-01 | `amlich-081` | Complete — strict workspace clippy passes with zero warnings |
| DEBT-02 | `amlich-t757.4` | Complete — `cargo fmt --check` passes |
| DEBT-03 | `amlich-t757.1` | Complete — serde-transparent `SourceId` newtype and guard tests |
| DEBT-04 | `amlich-t757.2` | Complete — canonical external-review lifecycle and active register |

**Coverage:**
- v1.8 requirements: 9 total
- Categories: OBS (3), EXP (2), DEBT (4)
- New categories vs v1.7: OBS, EXP, DEBT (all new; FND/RIT/FS/INT/ICH/XLK closed under v1.7 archive)

---
*Requirements defined: 2026-07-20; verified complete: 2026-08-10*
*Research basis: skipped per user decision — v1.8 is closure of in-flight epics + mechanical debt; no new domain to discover.*
