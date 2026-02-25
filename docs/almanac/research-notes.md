# Almanac Research Notes (Working Draft)

## Context

This document captures the current research understanding used to design the
AmLich expansion roadmap. It is a working synthesis and not yet a frozen source
of truth for every rule family.

## Core Research Conclusions

1. Vietnamese lunar calendar behavior should be modeled as two layers:
   - Deterministic astronomy/calendar calculations
   - Rule-based cultural and divination logic
2. Technical difficulty is mostly in rule normalization and variant management,
   not in UI rendering.
3. Ruleset versioning and explainable outputs are mandatory for OSS reliability.

## Layer A: Deterministic Calendar and Astronomy

### A.1 Solar <-> Lunar conversion

- Month start is anchored to the new moon day.
- Leap month behavior follows major-term (trung khi) logic.
- Month 11 is the month containing winter solstice.
- Timezone choice can shift local day boundaries and output dates.

Implementation implication:

- Keep timezone-parametric conversion and test boundary cases around midnight.

### A.2 Can Chi and cycle outputs

- Year/month/day can-chi are deterministic from lunar year/month and JDN.
- These are stable and suitable for O(1) formulas or table lookups.

Implementation implication:

- Preserve current formulas and expand regression fixtures.

### A.3 Tiet khi

- 24 terms are tied to sun ecliptic longitude in 15-degree increments.
- For app-level usage, active-term classification by date is sufficient.
- Exact instant mode is possible but requires stronger astronomical references.

Implementation implication:

- Keep fast active-term mode for v1.
- Treat exact-occurrence mode as optional advanced track.

## Layer B: Rule-Based Almanac

### B.1 Hoang dao/hac dao and gio hoang dao

- Hour-level behavior is straightforward and already available in many systems.
- Day-level behavior depends on month/day branch mapping tables and source choice.

Implementation implication:

- Add day-level resolver as a dedicated ruleset family.

### B.2 12 truc and 28 stars

- 12 truc is formulaic with month-branch/day-branch relationship.
- 28-star implementation can be practical cycle-based in v1; exact astronomical
  interpretation can be deferred.

Implementation implication:

- Keep cycle model with explicit anchor documentation in ruleset metadata.

### B.3 Common taboo families

- High-usage practical rules include:
  - Tam Nuong
  - Nguyet Ky
  - Sat Chu
  - Tho Tu
- Some are simple day-number checks; some require month-to-branch tables with variants.

Implementation implication:

- Implement taboo engine first.
- Freeze variant-heavy tables by explicit source for `vn_baseline_v1`.

## Layer C: Person and Event Rules

### C.1 Person-focused outputs

- Common expectations include:
  - Tuoi xung
  - Tam Tai
  - Kim Lau
  - Hoang Oc
  - Cuu Dieu and yearly Han
- These depend on age policy and variant-specific lookup tables.

Implementation implication:

- Freeze `tuoi_mu` policy first.
- Implement all personal outputs as versioned lookup/table rules.

### C.2 Event evaluation

- Users need recommendation quality per event type (wedding, construction, travel).
- This requires hard filters, soft penalties, and transparent scoring.

Implementation implication:

- Build explainable evaluation pipeline, not opaque "good/bad" labels.

## Variant and Localization Findings

- VN/CN differences may come from timezone conventions, source tables, and display culture.
- Symbolic labels (e.g. animal display for branch) should be locale-driven presentation,
  not core identity.

Implementation implication:

- Decouple branch identity from locale labels.
- Support ruleset variants and version bumps for behavior differences.

## Data and Contract Design Findings

Recommended entities:

- Reference tables: can/chi, stem-branch cycle, nap-am, tiet-khi definitions
- Ruleset metadata: id, version, region, source notes, defaults
- Rule families: day rules, taboo rules, personal rules, direction rules, event scoring
- Fact outputs: day facts, hour facts, person facts, evaluation explanations

Recommended API capabilities:

- conversion endpoints
- rich day-info endpoint
- person compute endpoint
- date-range evaluate endpoint

## Quality and Testing Findings

Must-have validation tracks:

1. New moon boundary cases near local midnight
2. Leap month tricky years
3. Tiet-khi transitions and seasonal checks
4. Day rules and taboo rules by golden fixtures
5. Person-rule boundary cases around Tet and age-cycle transitions

## Open Questions (Research Backlog)

1. Which canonical source tables to freeze for v1:
   - Sat Chu / Tho Tu
   - Cuu Dieu / Han
   - Hoang Oc variants
   - Direction variants
2. Whether v1 should expose only fast tiet-khi mode or both fast and accurate modes.
3. Whether official/historic mode is in scope for first public release.

## Decision Guidance

- If formula is deterministic and timezone-parameterized: implement now.
- If behavior is variant-heavy and table-based: research-source freeze first, then implement.
- Never silently replace a rule table; introduce a new ruleset version.
