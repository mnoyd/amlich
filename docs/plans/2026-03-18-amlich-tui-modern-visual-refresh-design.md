# Amlich-TUI Modern Visual Refresh Design

**Date:** 2026-03-18
**Status:** Approved

## Problem

`crates/amlich-tui` already surfaces substantial domain depth, but the visual layer still feels uneven across screens and overlays. The main issue is not missing data; it is inconsistent presentation hierarchy, overloaded shortcut surfaces, and widget-level style drift.

The user goal is to modernize the TUI visual language and interaction polish while the legacy TUI (`crates/amlich`) is being phased out.

## Scope

- Target only `crates/amlich-tui`
- Do not spend design effort on `crates/amlich` visual parity
- Preserve existing domain logic and recommendation policy boundaries

## User Intent

- The next polish pass should feel modern and intentional, not decorative.
- Improvements should prioritize readability, scannability, and interaction clarity.
- The redesign should keep current feature depth while making navigation feel cleaner.

## Approaches Considered

### Option A — Surface-Only Polish

Refresh colors, borders, and spacing without changing shell behavior or widget composition.

**Pros:** Fastest path, very low behavior risk.

**Cons:** Limited impact on discoverability and perceived quality.

### Option B — Design-System-First Refresh (recommended)

Define a coherent visual system (tokens + hierarchy + state styling) and apply it screen-by-screen.

**Pros:** Strong visual cohesion, durable foundation for later interaction work, moderate risk.

**Cons:** Slightly slower than cosmetic-only pass.

### Option C — Full Visual + Interaction Reboot

Redesign shell, navigation model, and widgets in one major rewrite.

**Pros:** Highest upside if fully executed.

**Cons:** Highest regression risk and stabilization cost.

## Recommended Direction

Adopt **Option B**: design-system-first refresh with phased rollout.

This maximizes visible modernization while preserving current behavior and avoiding a destabilizing rewrite.

## Section 1: Visual Language Spec

### Color System

Adopt semantic tokens in `theme.rs`:

- neutrals: `fg`, `muted`, `dim`, `surface`, `line`
- accents: `info` (cyan), `good` (green), `warn` (amber), `bad` (red)

Guideline: keep saturation moderate to support long reading sessions.

### Hierarchy Rules

- One primary focal point per screen (title + key verdict)
- Secondary cards use lower-contrast framing
- Metadata and provenance are dim by default
- Bright accents appear only on high-signal actions/states

### Border and Panel Language

Standardize panel frames into three levels:

- `primary` for active/selected blocks
- `secondary` for normal content cards
- `ghost` for internal grouping and separators

### Density and Spacing

Use fixed spacing tokens and avoid ad-hoc blank lines. Apply deterministic density behavior by layout mode (small/medium/large).

### Text Styling

- Bold for headings only
- Body text in regular weight
- Meta/provenance in dim style
- ALL-CAPS only for compact chips

### State Styling

Define explicit visuals for `focused`, `selected`, `disabled`, `loading`, and `error` so state transitions are predictable and legible.

## Section 2: Widget-Level Modernization

### Ribbon Redesign

Rework bottom ribbon into a cleaner, mode-aware bar:

- left: active screen identity
- right: context-relevant shortcuts only

### Header Strip

Add a compact top strip with chips for date context and active query settings (`ruleset`, `event kind`, `packs`).

### Card Consistency

Normalize cards to one anatomy:

1. title
2. key metric / verdict line
3. details body
4. optional evidence/footer line

### Focus and Selection

Use border emphasis + marker (not color alone) for focus/selection across keyboard navigation.

### Overlay Unification

Apply one visual pattern to Search, Context, and Help overlays:

- consistent width and title treatment
- denser grouping
- aligned spacing and borders

### Recommendation Card Polish

Keep existing recommendation ordering policy, but improve scanning with compact severity chips and clearer positive/negative bucket separation.

### Error and Loading Surfaces

Render loading/error as first-class status cards with retry hints instead of plain text fragments.

## Section 3: Phased Rollout

### Phase 1 (1-2 days): Foundation Tokens

Define semantic style tokens and helper constructors in `theme.rs`.

### Phase 2 (2-3 days): Shell Polish

Update `layout.rs` and `widgets/ribbon.rs` with modern shell framing and mode-aware shortcut surfaces.

### Phase 3 (3-4 days): High-Traffic Screens

Apply card and focus consistency to Dashboard, Scholar, and Recommendations.

### Phase 4 (2-3 days): Overlay Unification

Refine Search, Context, and Help overlays to shared visual patterns and state cards.

### Phase 5 (2-3 days): Remaining Screens

Bring Hours, Elements, FengShui, SolarTerms, and Calendar to full visual parity.

### Phase 6 (1 day): Final Quality Sweep

Run width-based layout checks, keyboard-flow smoke tests, and visual consistency review.

## Success Criteria

- Every screen and overlay uses the same semantic tokens and card anatomy.
- Recommendation and risk sections are immediately scannable on medium terminals.
- Focus/selection state is always obvious in keyboard flows.
- Small terminals avoid clipping/noise; large terminals avoid underfilled or uneven composition.
- Overall perception shifts from “assembled widgets” to “cohesive modern terminal product.”

## Non-Goals

- No redesign effort for `crates/amlich` (legacy TUI)
- No domain-rule changes in `amlich-core`
- No recommendation-policy changes in `amlich-api`
- No forced migration to entirely new navigation architecture in this phase
