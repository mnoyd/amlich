# Surface Replacement — Acceptance & Cutover

Decision record for `amlich-b14l.5`. Defines what the replacement desktop and TUI
must demonstrate before the legacy surfaces can be retired (`amlich-b14l`).

Vocabulary follows `CONTEXT.md` (Daily Workspace, Day View, Day Pattern, Hour
Timeline, Personal Context, Evidence Depth, Review State, Disclosure, Surface
Replacement). Surface decisions: default outputs in `amlich-b14l.3`, desktop
structure in `amlich-b14l.2`, TUI adaptation in `amlich-b14l.4`.

---

## 1. Principles

- The replacement changes presentation only: desktop UI, TUI screens, navigation,
  and rendering. `amlich-core` and `amlich-api` are untouched compatibility
  boundaries (§3).
- One shared product model; desktop and TUI render it with purpose-built
  interactions. Neither surface recomputes assessment math (§4).
- Legacy surfaces remain in-tree, CI-tested, and invocable behind stable opt-in
  flags until Stage 3 retirement (§6, §7).

## 2. Essential workflows

Acceptance scope. Each workflow must be demonstrated on **both** surfaces
(keyboard-first on TUI per `amlich-b14l.4`; pointer-first on desktop per
`amlich-b14l.2`).

| # | Workflow | Must demonstrate |
|---|----------|------------------|
| W1 | Daily check-in | Opens on today and now. Anonymous Day Overview: calendar identity (solar/lunar with leap-month marker, Can Chi, Tiết Khí), Day Pattern (Supports / Constraints / Unknowns), current hour and Notable Hours, Evidence Coverage. Useful with zero setup; no universal verdict. |
| W2 | Date navigation | Previous/next/today/jump-to-date; month browsing (desktop grid, TUI search). Selection state survives navigation; leap months stay labeled. |
| W3 | Hour exploration | Twelve-window Hour Timeline; selecting an hour updates time-dependent context without changing the date anchor; Notable Hours distinguished from any ranking; hour detail shows Hoàng Đạo classification and ruling star. |
| W4 | Assessment on demand | Personal Context (Intent [+ Birth Profile][+ Location]) enriches the same Day View. Day Assessment renders Result + Reasons + Assessment Confidence. Missing inputs surface as Unknowns — never neutral, never adverse. Each enrichment layer makes its changes explicit. |
| W5 | Explain & evidence | Every default-surface Result reaches Reason → Evidence (provenance, review state, divergences). Influence Explorer projects the core-owned assessment trace (graph, contribution matrix, evidence path) without recalculation. Explanation Projection v1 renders identically across surfaces. |
| W6 | Expert drill-downs | Bát Tự report, hour selection, Tiết Khí/season timeline, personal-day matrix, and classical surfaces remain reachable as drill-downs from the Day View — not as top-level module tabs. GraphRAG stays an evidence adapter. |

## 3. Compatibility boundaries

The replacement must not change any of these. Each has a blocking gate that
stays green throughout migration.

| Boundary | Contract | Gate |
|----------|----------|------|
| `amlich-core` / `amlich-api` | No semantic changes to engines or DTOs; additive, versioned DTO additions only, landed in `amlich-api` so CLI, desktop, and WASM share them | `just test`; the `*_contract.rs` suites in `crates/amlich-api` |
| CLI headless surface | `day`, `range`, `convert`, `almanac`, `insight`, `holidays`, `tiet-khi`, `lookup`, `config`, and deprecated `query --format dayinfo-json` stay byte-stable — `packages/core` bridges through `query`, and the JS fallback parity depends on it | `crates/amlich/tests/cli_contract.rs`; dayinfo-json golden; `pnpm test` in `packages/core` (rust and fallback modes) |
| Waybar output | `crates/amlich/src/waybar.rs` JSON unchanged | CLI contract tests |
| WASM | `amlich-wasm` builds from `amlich-api` unchanged | `just build-wasm` |
| Cross-surface parity | Explanation projection, point-opening (byte-for-byte goldens), traditional wellness, insight, day-info golden, personal-day assessment/projection, recommendation corpus | parity suites in `crates/amlich-api/tests` run in CI |
| Persisted user state | Display-mode config and saved profile schemas are shared by legacy and replacement; additive keys only — no schema fork, no migration step to roll back | config/contract tests on both surfaces |

## 4. Safety & review behavior

- **Review State**: content whose required human review is pending renders as
  pending or unavailable — never as normal content with a badge. The four v1.11
  gates (Najia open-point tables, Vietnamese point nomenclature, health-safety
  scope, disclaimer v2) are not unblocked by the surface swap; gated
  point-opening and wellness content stays behind core policy flags in the new
  surfaces exactly as in legacy.
- **Disclosure**: disclaimers and cultural-boundary disclosures render wherever
  the affected content appears in the new surfaces. The replacement introduces no
  new medical, treatment, or universal-verdict claims; Day Overview stays
  non-verdict.
- **Non-reevaluation**: surfaces never recompute weights, interactions, vetoes,
  precedence, or results. All assessment math stays in the core; the Influence
  Explorer is a read-only projection.
- **Verification**: contract tests on both new surfaces assert (a) pending-state
  rendering for gated content and (b) disclosure presence wherever gated content
  can appear.

## 5. Parity checks

1. **Data parity (automated, blocking).** The existing cross-surface and golden
   suites (§3) pass unchanged. New surface-layer projections — Day Pattern
   grouping, Evidence Coverage, Notable Hours — are pure projections of existing
   API outputs and get their own contract tests with golden fixtures over a
   representative corpus (normal, leap-month, and boundary dates; anonymous and
   the sample profile `1990-01-01 09:30 Nam`).
2. **Surface parity (cutover evidence).** For a fixed date/profile corpus,
   identity, pattern items, notable hours, assessment verdict and confidence,
   explanation rows, and coverage counts must be equal between legacy and
   replacement on the same surface. The Explanation Projection v1 contract is
   the comparison substrate.
3. **Manual acceptance walkthroughs.** W1–W6 checklists per surface, following
   the conventions of `docs/desktop-verification.md`. The TUI checklist covers
   the `amlich-b14l.4` keyboard map and the 40–59 / 60–99 / 100+ column density
   bands; the desktop checklist covers the Day View and Influence Explorer.

## 6. Rollback

- **Mechanism.** Legacy stays in-tree and CI-tested behind a stable opt-in
  (desktop: settings toggle/route; TUI: `--legacy` flag once the new TUI is the
  default) through Stage 2.
- **Action.** Rollback is flipping the default back in the next release. Because
  config/profile schemas are shared and un-migrated (§3), there is no data
  reversal step.
- **Triggers.** Any blocking parity suite red on a cutover candidate, an
  essential workflow blocked on the replacement, or a safety/disclosure
  regression.

## 7. Cutover sequence

| Stage | State | Exit criteria |
|-------|-------|---------------|
| 0 — Parallel build | Replacement behind dev flags (prototype routes / opt-in); legacy is the default | First vertical slices chosen (`amlich-b14l.6`) and landable |
| 1 — Feature-parity candidate | Both surfaces ship in one build; replacement opt-in via stable flag | W1–W6 demonstrated on both surfaces; §5 evidence complete |
| 2 — Default flip & soak | Replacement is the default; legacy opt-in; rollback armed | Soak across at least one release cycle with no rollback trigger (§6) |
| 3 — Legacy retirement | Legacy UI code removed from default builds; legacy checklists archived; `docs/desktop-verification.md` superseded by the replacement checklist | — |

Desktop and TUI advance through the stages independently — each has its own
checklist and flips its own default. A shared-model, core, or API regression
blocks both surfaces at any stage.

## 8. Ready-to-retire definition

A legacy surface may be deleted only when, for that surface: all essential
workflows (W1–W6) pass on the replacement, every compatibility gate (§3) is
green, safety and review behavior (§4) is verified, parity evidence (§5) is
complete, and the Stage 2 soak elapsed without a rollback.
