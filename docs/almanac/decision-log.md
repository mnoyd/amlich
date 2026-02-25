# Almanac Decision Log

## Purpose

Track major design and ruleset decisions so implementation remains consistent and auditable.

## Format

- `ID`: short decision id
- `Status`: proposed / accepted / superseded
- `Date`: YYYY-MM-DD
- `Decision`: what we decided
- `Why`: rationale
- `Impact`: code/data/API/test implications
- `Follow-up`: next bead(s)

---

## DEC-0001

- Status: accepted
- Date: 2026-02-25
- Decision: Keep current deterministic calendar engine and extend with a rule-driven almanac layer.
- Why: Preserves validated solar/lunar conversion and reduces regression risk.
- Impact: Build new features in `crates/amlich-core/src/almanac/*`; avoid replacing lunar math.
- Follow-up: Phase 1 ruleset infrastructure beads.

## DEC-0002

- Status: accepted
- Date: 2026-02-25
- Decision: Separate deterministic math from cultural rules and person/event evaluation.
- Why: Rule variance and source differences require versioned data packs and explainability.
- Impact: Output contracts need ruleset/profile/evidence metadata.
- Follow-up: Phase 0 and Phase 1 schema work.

## DEC-0003

- Status: accepted
- Date: 2026-02-25
- Decision: Use a phased, bead-based delivery plan with research and implementation in parallel.
- Why: Scope is large and variant-heavy; phase gates reduce drift and make agentic coding safer.
- Impact: Each feature family gets dedicated research, implementation, tests, and docs beads.
- Follow-up: Execute Phase 0 and Phase 1 first.

## DEC-0004

- Status: proposed
- Date: 2026-02-25
- Decision: v1 ruleset identifier will be `vn_baseline_v1`.
- Why: Names the intended baseline clearly and leaves room for future variants.
- Impact: Ruleset registry, tests, and API outputs should include this id/version.
- Follow-up: Confirm in Phase 0 scope freeze.

## DEC-0005

- Status: proposed
- Date: 2026-02-25
- Decision: Treat variant-heavy rule families (Sat Chu, Tho Tu, Cuu Dieu, Hoang Oc, direction tables) as data packs requiring explicit source selection before implementation.
- Why: Avoid silent assumptions and cross-app mismatch.
- Impact: Add research beads before implementation beads for these families.
- Follow-up: Phase 3-5 research beads.

Policy reference:

- `docs/almanac/known-differences.md`

## DEC-0006

- Status: accepted
- Date: 2026-02-25
- Decision: Freeze day-level hoang dao/hac dao v1 mapping to the 12-deity cycle with month-branch group start offsets (`Dần/Thân` start at `Thanh Long`, advancing by day branch order).
- Why: Phase 2 resolver needs one canonical mapping to avoid drift across implementations.
- Impact: Add canonical table doc (`docs/almanac/day-deity-v1-table.md`), implement ruleset-backed resolver in `I-2002`, and pin golden tests to this mapping.
- Follow-up: `R-2001`, `I-2002`, `T-2005`, `D-2006`.

## DEC-0007

- Status: accepted
- Date: 2026-02-25
- Decision: Standardize v1 taboo output explanations as deterministic family-specific templates and treat `hard`/`soft` as policy hints (not final recommendation labels).
- Why: Client apps need consistent copy and stable semantics for testing, display, and future evaluation scoring.
- Impact: `taboos[]` docs and tests can assert stable reason strings; future scoring engines should use `severity` as input while keeping event policy separate.
- Follow-up: `D-3006`, `T-3005`, Phase 6 scoring beads.

## DEC-0008

- Status: accepted
- Date: 2026-02-25
- Decision: Freeze `vn_baseline_v1` taboo-family definitions as fixed lunar-day sets for `tam_nuong`/`nguyet_ky` and the current month->chi tables for `sat_chu`/`tho_tu`, with default severities `hard`, `hard`, `hard`, `soft` respectively.
- Why: Phase 3 implementation and docs need a stable v1 baseline despite known variant differences (especially for `sat_chu`/`tho_tu`).
- Impact: Ruleset data, resolver tests, and `taboo-rules.md` can rely on one canonical v1 mapping; alternate tables must be introduced as new ruleset versions/variants.
- Follow-up: `R-3001`, `I-3002`, `I-3003`, `T-3005`.

---

## Supersession Rules

- Do not edit prior accepted decisions to change meaning.
- Add a new decision entry with `superseded by DEC-xxxx` if a decision changes.
- Reference decision ids in PRs and bead notes when applicable.
