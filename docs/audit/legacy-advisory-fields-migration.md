# Legacy Advisory Fields Audit (amlich-5a2)

## Purpose
Audit remaining desktop/web/app consumers for legacy advisory/reasoning fields instead of canonical `decision_export`, `graph`, and `personal-day-matrix` outputs.

## Legacy Advisory Fields

`PersonalDayAdvisoryDto` (crates/amlich-api/src/dto.rs:1127-1140):
- `summary: String`
- `severity: String`
- `top_signals: Vec<String>`
- `why_this_matters: Vec<String>`
- `recommended_actions: Vec<String>`
- `priority_order: Vec<String>`
- `highlights: Vec<String>`
- `cautions: Vec<String>`
- `reasoning_bucket: Option<String>`
- `reasoning_confidence: Option<String>`

## Canonical Outputs

### InitiationOpeningDecisionExport (crates/amlich-core/src/reasoning/types.rs:281-301)
- `primary_conclusion: String`
- `recommendation_bucket: RecommendationBucket`
- `confidence: DecisionConfidence`
- `context_is_clear: bool`
- `semantic: ReasoningConclusionSemantic`
- `strongest_supports: Vec<ReasoningNote>` - each note has `summary_vi`, `tags`, `node_id`
- `strongest_resistances: Vec<ReasoningNote>`
- `override_factors: Vec<ReasoningNote>`
- `conflict_notes: Vec<ReasoningNote>`
- `suggested_hours: Vec<String>`
- `suggested_directions: Vec<String>`
- `axis_scores: Vec<ReasoningAxisScore>`

### ReasoningGraphExport (crates/amlich-core/src/reasoning/types.rs:244-248)
- `action_id: ActionId`
- `nodes: Vec<ReasoningNodeExport>` - each has `id`, `kind`, `axis`, `severity`, `tags`, `summary_vi`, `evidence`
- `edges: Vec<ReasoningEdgeExport>` - each has `from_node_id`, `to_node_id`, `effect`, `weight`, `justification`, `evidence`, `tags`

### PersonalDayMatrixReportDto (crates/amlich-api/src/dto.rs:1174-1192)
- `day_person: DayPersonMatrix`
- `element_resonance: ElementResonanceMatrix`
- `personal_hours: Option<PersonalHourMatrix>`
- `direction_merge: Option<DirectionMergeMatrix>`
- `domain_day_boost: Option<DomainDayBoostMatrix>`

## Consumer Audit

| Consumer | Location | Legacy Usage | Canonical Usage |
|----------|----------|--------------|-----------------|
| **CLI** | `crates/amlich/src/main.rs:1413-1422` | `InsightSurface::Advisory` → `get_personal_day_advisory()` returns pure `PersonalDayAdvisoryDto` | `InsightSurface::Analysis` → `get_personal_day_analysis()` has `decision_export`/`graph`; `InsightSurface::Report` → `get_personal_day_report()` has both |
| **TUI** | `crates/amlich/src/widgets/insight_overlay.rs:855-870` | `render_personal_tab()` reads `report.advisory.highlights` and `report.advisory.cautions` | Uses `get_personal_day_report()` which contains both legacy `advisory` AND canonical `decision_export`/`graph` |
| **Desktop App** | `apps/desktop/` | Does NOT use personal day reasoning at all | Uses `get_day_info`/`get_day_insight` for general day recommendations only |

## Migration Map

### 1. TUI `render_personal_tab` Migration

**Current (legacy)** - `insight_overlay.rs:855-870`:
```rust
if !report.advisory.highlights.is_empty() {
    push_bulleted(&mut lines, &report.advisory.highlights, "•", 4);
}
if !report.advisory.cautions.is_empty() {
    push_bulleted(&mut lines, &report.advisory.cautions, "•", 4);
}
```

**Target (canonical)**:
```rust
if let Some(export) = &report.decision_export {
    // highlights → strongest_supports
    let highlights: Vec<String> = export.strongest_supports
        .iter()
        .map(|n| n.summary_vi.clone())
        .collect();
    if !highlights.is_empty() {
        push_bulleted(&mut lines, &highlights, "•", 4);
    }
    // cautions → strongest_resistances + override_factors
    let cautions: Vec<String> = export.strongest_resistances.iter()
        .chain(export.override_factors.iter())
        .map(|n| n.summary_vi.clone())
        .collect();
    if !cautions.is_empty() {
        push_bulleted(&mut lines, &cautions, "•", 4);
    }
}
```

### 2. CLI `InsightSurface::Advisory` Migration

**Current**: `main.rs:1413-1422` calls `get_personal_day_advisory()` which returns `PersonalDayAdvisoryDto` only.

**Options**:
- **Option A**: Change `InsightSurface::Advisory` to call `get_personal_day_report()` instead and render the canonical `decision_export` fields
- **Option B**: Keep the flattened advisory format but populate it from canonical sources in `get_personal_day_advisory()` (already partially done - it uses reasoning bundle to build highlights/cautions)

Note: `get_personal_day_report()` already contains both the legacy `advisory` AND canonical exports. The CLI's `--format advisory` could be deprecated in favor of `--format report`.

### 3. Desktop App

**Current**: Does NOT use personal day reasoning at all.

**Implication for amlich-7dd**: The desktop app needs *new* personal day reasoning functionality, not migration of existing functionality. The desktop app would need:
1. A new Tauri command to get personal day report/analysis
2. UI surfaces to display `decision_export` and `graph` data
3. Personal day matrix display (for amlich-6xm)

## Verification

Canonical outputs are available via:
- `get_personal_day_analysis()` → returns `PersonalDayAnalysisDto` with `decision`, `decision_export`, `graph`
- `get_personal_day_report()` → returns `PersonalDayReportDto` with `decision`, `decision_export`, `graph`, `advisory`, `chart`, `analysis`, `computed_metrics`
- `get_personal_day_matrix_report()` → returns `PersonalDayMatrixReportDto` with matrix outputs

Tests confirming canonical contracts:
- `crates/amlich-core/tests/reasoning_graph_canonical.rs`
- `crates/amlich-core/tests/reasoning_graph_contract.rs`
- `crates/amlich-api/tests/personal_day_contract.rs`
- `crates/amlich-api/tests/personal_day_matrix_contract.rs`
