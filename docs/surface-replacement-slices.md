# Surface Replacement — First Vertical Slices

Decision record for `amlich-b14l.6`. Defines the first end-to-end tracer
bullets that implement the replacement, their order, the smallest useful user
loop each proves, its evidence depth, and the staged roadmap that follows.

Vocabulary follows `CONTEXT.md` (Daily Workspace, Day View, Day Pattern, Hour
Timeline, Notable Hour, Evidence Coverage, Progressive Enrichment, Evidence
Depth, Vertical Slice, Day View Model). Predecessors: shared model
(`amlich-b14l.1`), default outputs (`amlich-b14l.3`), desktop structure
(`amlich-b14l.2`), TUI adaptation (`amlich-b14l.4`), acceptance and cutover
(`amlich-b14l.5`, `docs/surface-replacement-cutover.md`).

---

## 1. Slicing principles

- **End-to-end, thin, and closable.** Each slice is one tracer bullet:
  a shared-model projection in `amlich-api` (additive, versioned), rendered by
  the desktop replacement route and the TUI replacement entry, locked by
  contract tests with golden fixtures. A slice closes only when its workflow is
  demonstrated on **both** surfaces with every compatibility gate
  (cutover §3) green.
- **Fixed landing order inside a slice.** Shared model → desktop → TUI,
  matching the agreed sequence (`amlich-b14l`). The next slice's model work may
  start once the previous slice's model and its contract tests are merged, but
  no slice closes unevenly: desktop-only progress never closes a slice.
- **Projections, never re-evaluation.** Slice surfaces consume existing API
  outputs (DaySnapshot, insight, explanation projection, assessment trace) and
  may only group, label, and navigate them (cutover §4, §5.1).
- **Flags from slice one.** All slice code ships behind stable dev flags —
  desktop `?surface=next` (dev-guarded route, like the existing `prototype`
  param), TUI `--next` entry — leaving legacy defaults and rollbacks untouched
  through Stage 2 (cutover §6).

## 2. Smallest useful user loop (Slice 1 acceptance)

> Open the app with zero setup. Land on **today, now**: calendar identity
> (solar/lunar with leap-month marker, Can Chi, Tiết Khí), the anonymous Day
> Pattern (Supports / Constraints / Unknowns), the current hour with Notable
> Hours, and Evidence Coverage. Step to the previous or next day and back to
> today. Every visible Result carries a Reason; nothing recomputes; no
> universal verdict.

This is W1 plus single-day stepping — the thinnest loop that is genuinely
useful to a daily user. Full navigation (jump, month browsing) waits for
Slice 2.

## 3. The slices

| Slice | Workflow | Ships the user loop of… | New evidence depth | Exit criteria |
|-------|----------|------------------------|--------------------|---------------|
| **S1 — Anonymous Today** | W1 | open → today identity + Day Pattern + current hour + Evidence Coverage → step a day → return | Result + Reason (no drill) | Day View Model v0 in `amlich-api` with golden fixtures (normal, leap-month, boundary dates; anonymous); both dev-flag surfaces render it; gates green |
| **S2 — Navigate & Hours** | W2, W3 | prev/next/today/jump; month grid (desktop) and search (TUI); leap months labeled; selection state survives; select an hour → time-dependent context updates without moving the date anchor; Notable Hours ≠ ranking; hour detail shows Hoàng Đạo classification and ruling star | + hour-context reasons | Navigation and hour interaction demonstrated on both surfaces; hour-selection state contract-tested |
| **S3 — Assessment on demand** | W4 | Personal Context drill-down: Intent [+ Birth Profile][+ Location] progressively enriches the same Day View; Day Assessment renders Result + Reasons + Assessment Confidence; missing inputs surface as Unknowns; each layer makes its changes explicit | + assessment verdict, confidence reasons, enrichment diff | Enrichment layers and Unknown rendering contract-tested (anonymous + sample profile `1990-01-01 09:30 Nam`); non-verdict baseline unchanged |
| **S4 — Explain & evidence** | W5 | follow any Reason → Evidence: provenance, review state, divergences; Influence Explorer (desktop: graph / contribution matrix / evidence path) and collapsible causal tree (TUI) as read-only projections of the core-owned assessment trace | + full evidence provenance and expert projections | Explanation Projection v1 renders identically on both new surfaces (extends the cross-surface contract); pending-review renders pending/unavailable; disclosure presence verified |
| **S5 — Expert drill-downs** | W6 | Bát Tự report, hour selection, Tiết Khí/season timeline, personal-day matrix, and classical surfaces reachable as drill-downs from the Day View — never as top-level tabs; GraphRAG stays an evidence adapter | + expert surfaces at full depth | **Stage 1 feature-parity candidate**: W1–W6 demonstrated on both surfaces; cutover §5 parity evidence complete |

Depth is strictly additive down the table; each slice keeps the previous
depths working. No depth introduces re-evaluation, new claims, or un-gated
content.

## 4. Contract-test strategy

- New `day_view_projection` suite in `crates/amlich-api/tests`: golden
  fixtures over the representative corpus from cutover §5.1 (normal,
  leap-month, boundary dates; anonymous and the sample profile). Each slice
  extends the fixtures with its new fields — additive only.
- Cross-surface parity (cutover §5.2) accumulates per slice: S3 adds
  assessment verdict/confidence rows, S4 extends `explanation_cross_surface`
  to the new surfaces, S5 completes the expert set.
- Each slice appends its workflow rows to the per-surface manual checklists
  (conventions of `docs/desktop-verification.md`), so Stage 1 acceptance is
  assembled incrementally instead of re-run from scratch.

## 5. Roadmap after the slices

1. **S1 is the go/no-go checkpoint for the Day View Model.** If the projection
   feels forced against real API outputs, the model is corrected here — before
   S3 builds enrichment on it. A wrong-feeling model at S1 is a redesign; at
   S3 it is a migration.
2. **S2–S4 complete the daily-user product** (anonymous loop, navigation,
   hours, on-demand assessment, full evidence depth) — the replacement becomes
   the better daily surface for the primary loop.
3. **S5 completes the parity candidate** and exits cutover Stage 0 with
   Stage 1 evidence in hand.
4. **Stage 1 → 2 → 3 then follow `docs/surface-replacement-cutover.md` §7**
   unchanged: feature-parity candidate → default flip with one-release soak
   (rollback = flag flip) → legacy retirement, desktop and TUI advancing
   independently.

## 6. Out of scope (unchanged)

New domain engines, core enrichment, mobile, accounts, cloud sync, public web
service, and any change to the core/API compatibility boundaries
(`amlich-b14l`).
