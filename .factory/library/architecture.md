# Architecture

Architectural decisions, domain seams, and contract boundaries discovered during the mission.

**What belongs here:** core-to-API seams, consumer boundaries, migration notes, stable domain concepts.

---

- `amlich-core` is the domain source of truth. Key seams:
  - calendar/day computation
  - almanac/day-fortune enrichment
  - recommendation synthesis with contextual controls
  - ruleset registry and descriptor metadata
  - extension-pack descriptors and activation metadata
- This redesign mission treats `amlich-api` as the canonical engine contract and keeps consumer logic on top of that seam.
- Primary consumers for this mission are `crates/amlich` and `crates/amlich-tui`.
- Desktop, WASM, and JS package consumers are explicitly out of scope unless a follow-up feature is added.
- Current risk to fix first: `amlich` TTY auto-mode and `amlich tui` do not yet land in the same interactive experience.
- Target architecture for this mission:
  1. unify interactive launch paths onto one explorer-first shell
  2. add selector-first TUI state for ruleset, packs, event kind, and date
  3. route explorer selections into a deep inspection workspace without hidden context drift
  4. keep headless/machine-readable outputs aligned on canonical metadata and contextual activation
- The TUI should consume `amlich-api` bundle and catalog metadata directly; it should not invent new recommendation policy or selector normalization rules locally.
- Treat broader calendar/range browsing as secondary support flows, not the primary information architecture.
