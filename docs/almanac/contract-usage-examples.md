# Advisory and Matrix Contract Usage Examples

## Status

Developer documentation — amlich-1ek.

## Goal

Provide copyable examples for consumers of the canonical advisory and matrix surfaces:

- `personal-day` report payloads
- `hour-selection` report payloads
- `bazi` advisory payloads
- `personal-day-matrix` payloads

This document focuses on **how to consume** the payloads rather than re-defining every field in the schema. For field-by-field semantics, see `docs/almanac/reasoning-graph-schema.md`.

---

## Consumer rules of thumb

### 1. Prefer canonical fields over legacy aliases

When a surface exposes both a canonical export and a flattened legacy advisory, use the canonical export first.

- Prefer `decision_export.primary_conclusion` over legacy summary-like aliases.
- Prefer `decision_export.recommendation_bucket` and `decision_export.confidence` for headline state.
- Prefer `decision_export.strongest_supports`, `strongest_resistances`, `override_factors`, and `conflict_notes` for rationale/caution sections.
- Treat flattened `advisory.highlights` / `advisory.cautions` as compatibility output.

### 2. Treat matrix sections as optional

Matrix payloads are gated by input completeness.

- Birth date only may be enough for some personal-day reasoning.
- Birth hour/minute is required for `personal_hours`.
- Gender is required for direction merge / Kua-derived overlays.
- Consumers must gracefully handle `null` or omitted matrix sections.

### 3. Show the shallowest layer that satisfies the surface

Use the explanation hierarchy:

- **Headline**: `primary_conclusion`, bucket, confidence
- **Rationale**: strongest supports/resistances, axis scores
- **Cautions**: override factors, conflict notes, warnings
- **Drill-down**: graph nodes/edges, evidence envelopes, matrix rows

---

## Personal-day report

### Recommended consumption order

1. Use `decision_export.primary_conclusion` as the main sentence.
2. Use `decision_export.recommendation_bucket` for badge/color.
3. Use `decision_export.strongest_supports` and `strongest_resistances` for short bullets.
4. Use `decision_export.override_factors` and `conflict_notes` for caution treatment.
5. Use `graph` only for deep explanation affordances.
6. Use `advisory` only for compatibility with older surfaces.

### Example request

```bash
amlich lookup personal-day 2024-02-10 \
  --birth-year 1990 \
  --birth-month 1 \
  --birth-day 1 \
  --gender male \
  --surface report \
  --format json
```

### Example response excerpt

```json
{
  "summary": "Personal day view has 2 caution signal(s) and 3 highlight(s).",
  "severity": "medium",
  "top_signals": [
    "Bối cảnh khởi sự còn trái chiều nên cần giữ thế thận trọng: thuận Kim Quỹ nhưng vẫn có lực cản Kiêng/kỵ: Thọ Tử"
  ],
  "decision_export": {
    "primary_conclusion": "Bối cảnh khởi sự còn trái chiều nên cần giữ thế thận trọng: thuận Kim Quỹ nhưng vẫn có lực cản Kiêng/kỵ: Thọ Tử",
    "recommendation_bucket": "cautious",
    "confidence": "low",
    "context_is_clear": false,
    "semantic": "conflicted_cautious",
    "strongest_supports": [
      { "node_id": "fact.day.day_deity", "summary_vi": "Kim Quỹ", "tags": ["support"] }
    ],
    "strongest_resistances": [
      { "node_id": "fact.day.taboos", "summary_vi": "Kiêng/kỵ: Thọ Tử", "tags": ["resistance"] }
    ],
    "override_factors": [],
    "conflict_notes": [
      { "node_id": "fact.graph.mixed_day_signals", "summary_vi": "fact.graph.mixed_day_signals", "tags": ["conflict"] }
    ],
    "axis_scores": [
      { "axis": "support", "score": 3.0, "strongest_summary_vi": "Kim Quỹ" },
      { "axis": "resistance", "score": 1.0, "strongest_summary_vi": "Kiêng/kỵ: Thọ Tử" }
    ]
  },
  "graph": {
    "action_id": "initiation_opening",
    "nodes": [{ "id": "fact.day.day_deity", "summary_vi": "Kim Quỹ" }],
    "edges": [{ "from_node_id": "fact.day.day_deity", "to_node_id": "signal.support", "effect": "supports" }]
  },
  "advisory": {
    "highlights": ["Kim Quỹ", "kua 9 East", "dai_van Nghich"],
    "cautions": ["Kiêng/kỵ: Thọ Tử", "kim lau: suc"]
  }
}
```

### Example UI mapping

| UI element | Read from | Notes |
|------------|-----------|-------|
| Verdict sentence | `decision_export.primary_conclusion` | Canonical headline |
| Severity badge | `decision_export.recommendation_bucket` | Prefer bucket over legacy summary/severity |
| Confidence pill | `decision_export.confidence` | Secondary qualifier |
| Support bullets | `decision_export.strongest_supports[].summary_vi` | Canonical rationale |
| Resistance bullets | `decision_export.strongest_resistances[].summary_vi` | Canonical rationale |
| Hard caution block | `decision_export.override_factors`, `conflict_notes` | Use stronger styling |
| Explain more | `graph.nodes`, `graph.edges` | Drill-down only |

### Anti-pattern

Do **not** rebuild highlights/cautions from `advisory.highlights` / `advisory.cautions` if `decision_export` is available.

---

## Hour-selection report

### Recommended consumption order

1. Use `analysis.canonical` or `advisory.canonical` as the stable export.
2. Use `summary_vi` / `summary_en` for localized body copy.
3. Use `top_recommendation` for the single best hour.
4. Use `ranked_hours` to render sorted windows.
5. Keep `good_hours` / `bad_hours` for compatibility or low-detail displays.

### Example request

```bash
amlich lookup hour-selection 2024-02-10 --surface report --format json
```

### Example response excerpt

```json
{
  "analysis": {
    "intent": "travel",
    "summary_vi": "Ưu tiên giờ Dần (03:00-05:00) cho travel vì đứng đầu xếp hạng với 6 giờ hoàng đạo hỗ trợ.",
    "top_recommendation": {
      "hour_chi": "Dần",
      "time_range": "03:00-05:00",
      "is_good": true
    },
    "canonical": {
      "intent": "travel",
      "birth_data_tier": "anonymous",
      "summary_vi": "Ưu tiên giờ Dần (03:00-05:00) cho travel vì đứng đầu xếp hạng với 6 giờ hoàng đạo hỗ trợ.",
      "top_recommendation": {
        "chi_name": "Dần",
        "time_range": "03:00-05:00",
        "is_auspicious": true,
        "score": 70
      },
      "ranked_hours": [
        { "chi_name": "Dần", "time_range": "03:00-05:00", "is_auspicious": true, "score": 70 }
      ],
      "auspicious_count": 6,
      "total_hours": 12
    }
  },
  "advisory": {
    "best_windows": ["Dần 03:00-05:00", "Dậu 17:00-19:00"],
    "canonical": {
      "intent": "travel",
      "top_recommendation": {
        "chi_name": "Dần",
        "time_range": "03:00-05:00",
        "is_auspicious": true,
        "score": 70
      }
    }
  }
}
```

### Example UI mapping

| UI element | Read from | Notes |
|------------|-----------|-------|
| Header text | `analysis.canonical.summary_vi` | Canonical localized summary |
| Best hour card | `analysis.canonical.top_recommendation` | Stable top-pick source |
| Ranked list | `analysis.canonical.ranked_hours` | Preferred over unsorted compatibility lists |
| Count badge | `analysis.canonical.auspicious_count` | Useful for aggregate display |
| Evidence affordance | `analysis.canonical.evidence` | Optional provenance |

---

## Bazi advisory

### Recommended consumption order

1. Use `summary`, `severity`, and `top_signals` as the top-level advisory headline.
2. Use `useful_god_analysis` to show deeper chart-balance reasoning.
3. Use `warnings` as explicit caution treatment.
4. Use `domains` to populate concrete domain sections (career, wealth, relationship, health, timing).

### Example request

```bash
amlich lookup bazi 2024-02-10 \
  --hour 9 \
  --gender male \
  --target-year 2027 \
  --months 1,2 \
  --surface advisory \
  --format json
```

### Example response excerpt

```json
{
  "summary": "Bazi advisory includes 1 warning(s) with 3 top signal(s).",
  "severity": "medium",
  "top_signals": [
    "yong_shen Tho",
    "xi_shen Moc",
    "Dụng thần/hỷ thần hiện là heuristic giai đoạn đầu, chưa phải kết luận trường phái đầy đủ."
  ],
  "why_this_matters": [
    "Dụng thần points to the element most useful for restoring chart balance.",
    "Hỷ thần highlights secondary support, useful for timing and softer optimization."
  ],
  "recommended_actions": [
    "Treat warnings as constraints before optimizing around favorable signals."
  ],
  "warnings": [
    "Dụng thần/hỷ thần hiện là heuristic giai đoạn đầu, chưa phải kết luận trường phái đầy đủ."
  ],
  "useful_god_analysis": {
    "favorable_elements": ["tho", "moc"],
    "tentative_yong_shen": "tho",
    "tentative_xi_shen": "moc",
    "confidence": "medium"
  },
  "domains": {
    "career": ["Sự nghiệp đang ở pha phát triển; nên tích lũy track record và giữ nhịp bền."],
    "health": ["Có dấu hiệu mất cân bằng; cần theo dõi các thói quen tiêu hao kéo dài."]
  }
}
```

### Example UI mapping

| UI element | Read from | Notes |
|------------|-----------|-------|
| Advisory headline | `summary` | Canonical export headline |
| Severity tone | `severity` | `low`, `medium`, `high` |
| Signal chips | `top_signals` | Ordered signal list |
| Constraint banner | `warnings` | Always show prominently when non-empty |
| Useful god breakdown | `useful_god_analysis` | Better for advanced users |
| Domain cards | `domains.<domain>` | Concrete user-facing actions |

---

## Personal-day matrix report

### Recommended consumption order

1. Use `tier` to determine how much of the matrix can be shown.
2. Use `day_person` for pillar interaction explanation.
3. Use `element_resonance` for five-element balance framing.
4. Use `personal_hours`, `direction_merge`, and `domain_day_boost` when present.
5. Use `unavailable_sections` to explain absent matrix sections.

### Example request

```bash
amlich lookup personal-day-matrix 2024-02-10 \
  --birth-year 1990 \
  --birth-month 1 \
  --birth-day 1 \
  --hour 9 \
  --minute 30 \
  --gender male \
  --format json
```

### Example response excerpt

```json
{
  "tier": "datetime",
  "day_person": {
    "day_canchi": "Giáp Thìn",
    "day_master": "Bính Dần",
    "pillars": [
      { "pillar": "year", "pillar_canchi": "Kỷ Tỵ", "element_interaction": "day_controls_pillar" }
    ]
  },
  "element_resonance": {
    "day_element": "moc",
    "net_resonance": 0.40000004,
    "entries": [
      { "element": "hoa", "effective_resonance": 1.0, "day_helps_deficit": false }
    ]
  },
  "personal_hours": {
    "birth_hour_chi": "Tỵ",
    "hours": [
      { "chi": "Tý", "time_range": "23:00-01:00", "score": 95, "is_hoang_dao": true }
    ]
  },
  "direction_merge": {
    "kua_number": 9,
    "entries": [
      { "direction": "Bắc", "signals": ["kua_favorable", "phuc_than"], "net_score": 2 }
    ]
  },
  "domain_day_boost": {
    "entries": [
      { "domain": "career", "base_score": 80.0, "boosted_score": 76.0 }
    ]
  },
  "unavailable_sections": []
}
```

### Example UI mapping

| UI element | Read from | Notes |
|------------|-----------|-------|
| Matrix availability badge | `tier` | `date` vs `datetime` |
| Pillar interaction list | `day_person.pillars` | Good summary block |
| Element chart | `element_resonance.entries` / `net_resonance` | Supports chart/table UI |
| Best personal hours | `personal_hours.hours` sorted by `score` | Prefer top few rows |
| Best directions | `direction_merge.entries` sorted by `net_score` | Show signal breakdown |
| Domain optimization | `domain_day_boost.entries` sorted by `boosted_score` | Useful for recommendation ranking |
| Missing data note | `unavailable_sections` | Explain why a section is absent |

### Anti-pattern

Do **not** assume `personal_hours`, `direction_merge`, or `domain_day_boost` always exist. These sections depend on input completeness and rule availability.

---

## Desktop / TypeScript example

The desktop app now models these payloads in `apps/desktop/src/lib/insights/types/personal-day-dto.ts` and consumes them from `PersonalDayPanel.svelte`.

### Recommended TypeScript usage pattern

```ts
const report = await invoke<PersonalDayReportDto>("get_personal_day_report", {
  day: 10,
  month: 2,
  year: 2024,
  birthYear: 1990,
  birthMonth: 1,
  birthDay: 1,
  gender: "male",
});

const headline = report.decision_export?.primary_conclusion ?? report.summary;
const supports = report.decision_export?.strongest_supports.map((n) => n.summary_vi) ?? [];
const cautions = [
  ...(report.decision_export?.strongest_resistances ?? []),
  ...(report.decision_export?.override_factors ?? []),
].map((n) => n.summary_vi);
```

For matrix payloads:

```ts
const matrix = await invoke<PersonalDayMatrixReportDto>("get_personal_day_matrix_report", {
  day: 10,
  month: 2,
  year: 2024,
  birthYear: 1990,
  birthMonth: 1,
  birthDay: 1,
  birthHour: 9,
  birthMinute: 30,
  gender: "male",
});

const topDirections = [...(matrix.direction_merge?.entries ?? [])]
  .sort((a, b) => b.net_score - a.net_score)
  .slice(0, 4);
```

---

## Explanation UX validation cases

Use these representative cases as acceptance checks for the explanation hierarchy across desktop, TUI, and future surfaces.

| Case class | Parity case ID | Expected headline | What helps / rationale | What to watch / cautions | Proceed / details expectations |
|------------|----------------|-------------------|-------------------------|--------------------------|-------------------------------|
| Favorable baseline | `strong_favorable` | Clear positive verdict with `favorable` bucket and no caution-first framing | At least one support should be visible early | Override treatment should stay absent | Details can stay collapsed behind evidence / axis summaries |
| Cautious layered | `conflicting_layered` | Mixed or careful verdict with `cautious` bucket | Supports can be shown, but must not overpower caution framing | Conflict visibility and override treatment should both be visible | Details should explain why the day feels mixed rather than simply good/bad |
| Avoid baseline | `strong_avoid` | Strong stop / avoid headline with `avoid` bucket | Rationale may exist, but should remain secondary | Override treatment should be prominent and immediate | Proceed guidance, if any, must feel conditional rather than encouraging |
| Personal-profile refinement | `profile_directions_with_gendered_kua` | Personalized output can still land in a `cautious` headline rather than becoming fully favorable | Personalized direction/hour output should be reflected in visible rationale or next-step guidance | Cautions should remain honest when profile data sharpens tradeoffs instead of removing them | Surface should show that profile-specific advice changed the explanation, not just extra raw data |
| Boundary / timezone edge | `vn_midnight_conflict_window` | Headline should remain stable and understandable at local-day boundaries | Rationale should still include concrete reasons instead of exposing timezone mechanics first | Conflict treatment should remain visible when the boundary case is mixed | Detail affordances may mention deeper evidence, but the main surface should not require users to understand snapshot/timezone internals |

These cases are backed by the executable parity corpus in `crates/amlich-core/tests/reasoning_graph_parity.rs` and should be rechecked whenever headline wording, caution prominence, or personalized explanation treatment changes.

## Integration checklist

Before shipping a consumer of these contracts, verify:

- [ ] Canonical fields are used where available (`decision_export`, canonical hour-selection export, matrix rows)
- [ ] Legacy advisory aliases are treated as compatibility only
- [ ] Optional matrix sections are null-safe
- [ ] `warnings`, `override_factors`, and `conflict_notes` receive stronger visual treatment than generic rationale
- [ ] Deep graph/evidence data is kept behind a drill-down or detail affordance
- [ ] Localized fields (`summary_vi`, `summary_en`) are chosen appropriately for the surface

---

## Related docs

- `docs/almanac/reasoning-graph-schema.md`
- `docs/almanac/contract-evolution-guidelines.md`
- `docs/almanac/explanation-hierarchy.md` (see worktree branch until merged if absent on current branch)
- `docs/audit/legacy-advisory-fields-migration.md`
