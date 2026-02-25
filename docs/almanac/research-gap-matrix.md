# AmLich Almanac Research Gap Matrix

## Purpose

Track which topics are already strong enough to implement, which need one more source,
and which need a validation corpus before release.

Status labels:

- `READY` - enough to implement now
- `NEED_SOURCE` - need at least one canonical source/table selection
- `NEED_VALIDATION` - implementable, but must build parity/golden checks before release

## Matrix by Domain

| Domain | Status | Why | Next Action |
|---|---|---|---|
| Solar <-> Lunar conversion (VN UTC+7) | READY | Core formulas and behavior are well-defined and already implemented | Preserve behavior, add boundary tests |
| Leap month and month 11 logic | READY | Rule and algorithm are explicit | Add tricky-year golden cases |
| Can Chi (year/month/day) | READY | Formulas are stable and standard | Add more parity fixtures |
| Can Chi hour | NEED_SOURCE | Formula family is known but implementation variant/presentation details need explicit source choice | Select v1 rule and examples |
| Tiet khi (active term by date) | READY | Current longitude-based day classification is enough for app baseline | Keep fast mode |
| Tiet khi exact instants | NEED_VALIDATION | Concept is clear, but needs ephemeris/reference source and comparison data | Choose source + build validation corpus |
| Day hoang dao/hac dao | READY | v1 canonical table frozen in `docs/almanac/day-deity-v1-table.md` (`DEC-0006`) | Implement resolver + golden tests |
| Gio hoang dao | READY | Already implemented and test-covered | Integrate under ruleset versioning |
| 12 truc | READY | Formula is stable and implemented | Add evidence metadata and variant notes |
| Nhi thap bat tu (JD cycle mode) | NEED_SOURCE | Cycle approach is acceptable, but anchor/ruleset must be explicit | Choose anchor and document |
| Nhi thap bat tu (astronomical mode) | NEED_SOURCE | Complex and not needed for v1 | Defer to advanced track |
| Ngu hanh can/chi mapping | READY | Common mappings stable enough for v1 | Version if alternate schools added |
| Nap am (60 pair lookup) | READY | Table-driven and already partially present | Expand docs and tests |
| Tuoi xung / luc xung / tam hop / tu hanh xung | READY | Base branch relation graph is stable and already implemented in part | Extend to person-aware outputs |
| Tam Nuong / Nguyet Ky | READY | Widely used simple lunar-day rules | Implement as taboo ruleset entries |
| Sat Chu / Tho Tu | READY | v1 month->chi tables frozen in `docs/almanac/taboo-v1-table-freeze.md` with caveat policy | Keep as baseline mapping and add versioned variants later |
| Tam Tai | NEED_SOURCE | Common rule exists but table must be explicitly versioned | Choose v1 mapping source and examples |
| Kim Lau | NEED_SOURCE | Common modern rule exists but conventions differ | Document age convention and v1 formula |
| Hoang Oc | NEED_SOURCE | Multiple teaching variants exist | Freeze one v1 mapping table |
| Cuu Dieu | NEED_SOURCE | Requires gender + age-cycle table by source | Freeze v1 table and examples |
| Yearly Han | NEED_SOURCE | Same issue as Cuu Dieu, source-specific | Freeze v1 table and examples |
| Than huong / hy than / tai than / hac than | NEED_SOURCE | Existing data exists, but variants differ by source | Separate direction rule variants |
| Event scoring (cuoi hoi, dong tho, ...) | NEED_SOURCE | Depends on policy, not only source | Define scoring policy + hard filters |
| Locale animal labels (Meo/Tho for Mao) | READY | Display-layer decoupling is straightforward | Implement locale display mapping |
| VN/CN ruleset variants | NEED_VALIDATION | Architecture is clear; correctness needs fixture comparisons | Add ruleset + parity corpus |
| Official vs astronomical/historic modes | NEED_SOURCE | Scope and references need explicit decision | Defer unless required |

## Research Backlog by Phase

### Phase 1 (Ruleset Infrastructure)
- No heavy new research required.
- Need to define schema extension points for future rule families.

### Phase 2 (Day Hoang Dao/Hac Dao)
- Pick one canonical v1 day-deity mapping table.
- Capture at least 12 worked examples (one per lunar month branch group).

### Phase 3 (Taboo Rules)
- Freeze Sat Chu and Tho Tu tables and identify variant name/lineage.
- Decide severity (`hard`/`soft`) by event type or global default.

### Phase 4-5 (Person Rules)
- Freeze age policy around Tet (tuoi mu).
- Freeze Tam Tai/Kim Lau/Hoang Oc/Cuu Dieu/Han v1 tables.
- Gather examples for both genders and ages around cycle boundaries.

### Phase 6 (Event Evaluation)
- Research product policy more than historical sources.
- Define explainable scoring and hard-filter behavior.

### Phase 7 (Tiet Khi Accuracy)
- Choose ephemeris/reference source and compare against published tables.

### Phase 8 (Locale/Variant Presentation)
- Define display-only vs logic-affecting differences.

## Minimum Research Package for v1 Implementation Start

Enough to start coding now:

- Ruleset infrastructure
- Day hoang dao/hac dao framework (pending chosen table)
- Taboo engine framework
- Tam Nuong and Nguyet Ky rules

Need before implementing person rules:

- Frozen age policy
- Frozen v1 tables for Tam Tai/Kim Lau/Hoang Oc/Cuu Dieu/Han

Need before claiming cross-app parity:

- Golden corpus and side-by-side comparisons by ruleset/version
