# Phase 4: Core Dai Van Module - Context

**Gathered:** 2026-03-03
**Status:** Ready for planning

<domain>
## Phase Boundary

Deliver a deterministic Dai Van calculation core that generates 8 contiguous 10-year pillars from birth date and gender, determines Chieuthu direction (Thuận/Nghịch), computes Dai Van start age from Tiết Khí distance using the 3-days-equals-1-year convention, and returns convention/evidence metadata for traceability. This phase does not add Ten Gods helpers or Kua analysis behavior.

</domain>

<decisions>
## Implementation Decisions

### Source Policy and Evaluation Basis
- Phase 4 implementation baseline uses `vietnamese_lunar_engine_tables.md` section 15 (Đại Vận) for concrete Dai Van formulas and rule matrix.
- `source_id` traceability remains explicitly marked as KHCBPPT placeholder pending manual chapter-level verification, consistent with requirements.
- Correctness evaluation for Phase 4 should prioritize deterministic fixtures and edge-case coverage (especially Tiết Khí boundaries and transition boundaries), with source provenance carried in metadata.

### Transition Boundary Semantics
- Pillar transition is boundary-inclusive for the incoming pillar (at exact transition age, use the new pillar).
- Age ranges are modeled as `[start_age, end_age)` (start inclusive, end exclusive) to avoid overlap.
- Ages before the first Dai Van start age return no pillar (`None`/empty).
- Out-of-range helper lookups (for this phase boundary discussion) should return no pillar (`None`) rather than clamping.

### Start-Age Conversion
- Canonical `start_age` should be stored as a precise numeric value (decimal years), not integer-only.
- If Tiết Khí distance is exactly zero, start age is `0`.
- Partial-day precision contributes to the conversion (not whole-day truncation only).
- Outputs should include both machine-usable raw value and human-facing display representation.

### Chieuthu Contract
- Output shape should carry both canonical direction enum semantics (forward/backward) and Vietnamese display labels (`Thuận`/`Nghịch`).
- Polarity source for the direction matrix is Year Heavenly Stem polarity.
- Missing/invalid gender does not silently default; direction should be absent/error for that calculation path.
- Include matrix convention metadata (explicit `year_polarity_x_gender`-style method note) for auditability.

### Claude's Discretion
- Exact field names and DTO shape for the dual representation (enum + Vietnamese label).
- Exact formatting of display string for start-age human-readable output.
- Exact error surface type (`Option` vs `Result`) per layer as long as behavior remains explicit and non-silent.

</decisions>

<specifics>
## Specific Ideas

- User asked to make source provenance explicit: which source, which book, and how calculation is evaluated.
- User accepted a pragmatic policy: implement from in-repo Dai Van table specification now, keep KHCBPPT citation verification as explicit follow-up traceability work.

</specifics>

<code_context>
## Existing Code Insights

### Reusable Assets
- `crates/amlich-core/src/almanac/types.rs`: `HeavenlyStem` and `polarity()` are already present and match Chieuthu polarity needs.
- `crates/amlich-core/src/almanac/tu_menh.rs`: convention-metadata pattern (`year_basis`, resolution, encoding) is established and reusable for Dai Van convention fields.
- `crates/amlich-core/src/almanac/types.rs`: `RuleEvidence { source_id, method, profile }` pattern already exists for evidence traceability.
- `crates/amlich-api/src/dto.rs`: optional/computed DTO field patterns and convention/evidence DTO style are already established.

### Established Patterns
- Core computation modules are deterministic and side-effect free; metadata/evidence is included directly in result payloads.
- Project favors explicit provenance fields (`source_id`, `method`) and convention descriptors over implicit assumptions.
- Optional absence for non-applicable computed values is used broadly rather than silent fallback defaults.

### Integration Points
- New Dai Van core module should live in `crates/amlich-core/src/almanac/` and align with existing module export patterns.
- Public exposure can follow existing re-export style from `crates/amlich-core/src/lib.rs`.
- API/DTO mapping should align with existing conversion conventions in `crates/amlich-api/src/dto.rs` and `crates/amlich-api/src/convert.rs`.

</code_context>

<deferred>
## Deferred Ideas

None - discussion stayed within phase scope.

</deferred>

---

*Phase: 04-core-dai-van-module*
*Context gathered: 2026-03-03*
