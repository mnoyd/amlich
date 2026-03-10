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

## DEC-0009

- Status: accepted
- Date: 2026-03-09
- Decision: Freeze the recommendation v1 planning contract around a precedence-first engine, bounded day-deity modifiers, explicit source-family separation, and versioned optional packs for variant-sensitive rule families.
- Why: The recommendation research converges on explainability and explicit provenance, while the largest disagreements come from source mixing and fake precision.
- Impact: The planning artifacts in `recommendation-research-reconciliation.md`, `recommendation-conflict-triage.md`, `recommendation-rule-matrix.json`, and `recommendation-promotion-order.json` become the active v1 policy contract for follow-up implementation beads.
- Follow-up: `aml-bhy`, `aml-o99`, `aml-b0g`.

## DEC-0010

- Status: accepted
- Date: 2026-03-09
- Decision: Treat burial/funeral recommendation automation as a safety-sensitive domain that defaults to conservative wording and does not receive aggressive positive automation in default v1.
- Why: This domain is culturally sensitive, variant-heavy across traditions, and carries outsized product trust risk when the engine overclaims.
- Impact: Core recommendation logic and all presentation layers must avoid generic auspicious wording for burial/funeral output; future automation in this area requires explicit policy review and dedicated tests.
- Follow-up: `aml-6km`, `aml-o99`.

## DEC-0011

- Status: accepted
- Date: 2026-03-09
- Decision: Do not expose numeric confidence scores in default v1 recommendation APIs or UI; confidence may only be expressed through explicit structural semantics such as provenance, rule class, and advisory posture.
- Why: The current recommendation engine is deterministic and precedence-based, not probabilistic; numeric percentages would overstate certainty and obscure source variance.
- Impact: Recommendation DTO and UI work must avoid invented percentage confidence until a fully specified semantics and parity contract exists.
- Follow-up: `aml-6km`, `aml-o99`, future confidence-policy work.

## DEC-0012

- Status: accepted
- Date: 2026-03-09
- Decision: Limit strong recommendation wording to non-absolute, activity-specific language backed by deterministic baseline rules, and ban absolute safety or certainty phrasing.
- Why: Recommendation text is a product contract; overly strong wording creates user-trust failures even when the underlying rule firing is technically correct.
- Impact: Rendering layers should prefer phrases such as `Suitable`, `Generally favorable`, `Use caution`, or `Needs expert review`, and must avoid terms such as `Guaranteed`, `Perfect`, or `Risk-free`.
- Follow-up: `aml-6km`, UI/API wording reviews, future recommendation presentation tests.

## DEC-0013

- Status: accepted
- Date: 2026-03-10
- Decision: Freeze the v1 implementation target for recommendation alignment as follows: the public bucket taxonomy remains `Nên / Có thể / Tránh / Kỵ mạnh`; legacy `dayGuidance` remains informational-only and must not seed default recommendation synthesis; baseline recommendations remain date-only; and top-level day outputs are expected to expose `ruleset_id`, `ruleset_version`, and `profile` without requiring consumers to infer profile from nested fortune payloads.
- Why: Current code and UI drift comes from mixing an older 2-column guidance surface with the newer recommendation engine, plus ambiguity around where profile provenance lives in day-level contracts.
- Impact: Follow-up implementation beads should remove `dayGuidance` from the default merger, keep personalization separate, align DTO/meta contracts around top-level provenance, and keep presentation layers on the 4-bucket model.
- Follow-up: `aml-ig4.2`, `aml-ig4.3`, `aml-ig4.6`, `aml-ig4.7`.

## DEC-0014

- Status: accepted
- Date: 2026-03-10
- Decision: In default v1, hard-stop (`Kỵ mạnh`) authority is reserved for policy-approved rule families with explicit blocking semantics, with structured taboo severity as the only built-in hard-stop source until another family is explicitly promoted by decision.
- Why: The extension-layer API is intentionally flexible, but unrestricted hard-stop emission would let implementation details bypass the published safety policy and destabilize product meaning.
- Impact: Recommendation-layer plumbing must not allow arbitrary extension hits to escalate to `Kỵ mạnh` without an explicit policy gate, and tests must cover both allowed and disallowed hard-stop producers.
- Follow-up: `aml-ig4.5`, `aml-ig4.7`.

---

## Supersession Rules

- Do not edit prior accepted decisions to change meaning.
- Add a new decision entry with `superseded by DEC-xxxx` if a decision changes.
- Reference decision ids in PRs and bead notes when applicable.
