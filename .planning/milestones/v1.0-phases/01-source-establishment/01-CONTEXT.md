# Phase 1: Source Establishment - Context

**Gathered:** 2026-02-28
**Status:** Ready for planning

<domain>
## Phase Boundary

Pin the KHCBPPT edition used by this project and extract raw reference tables for every almanac subsystem. This phase produces the authoritative reference data that Phase 2 will serialize into the golden dataset. No code changes — this is pure research and documentation.

Requirements: SRC-01 (edition pinning), SRC-02 (nạp âm scope), SRC-03 (intercalary month handling).

</domain>

<decisions>
## Implementation Decisions

### KHCBPPT edition & access
- A specific edition (publisher, year, translator) must be identified and recorded — different editions may have different table values
- Citation granularity: chapter + section level (e.g., "Quyển 4, Thiên Văn") — enough to locate data in the text without requiring page-level precision
- The current baseline.json data origin (direct transcription vs secondary sources) must be documented honestly — this affects confidence levels in Phase 3 validators

### Nạp Âm scope
- Nạp âm currently cites "tam-menh-thong-hoi" not "khcbppt" — Phase 1 must first check whether KHCBPPT contains nạp âm tables
- If KHCBPPT has nạp âm tables: re-source from KHCBPPT and update source_id
- If KHCBPPT does not: document the gap and validate against Tam Mệnh Thông Hội as a separate source
- Golden dataset should support per-subsystem source attribution — pragmatic and honest about mixed sources

### Reference table format
- Structured markdown tables, one file per subsystem (stars.md, taboos.md, day_deity.md, truc.md, xung_hop.md, than_huong.md, na_am.md)
- Include original Vietnamese text alongside extracted values where available — provides audit trail
- Location: `docs/reference/khcbppt/` — permanent project documentation, not ephemeral planning artifacts

### Star source clarification
- Stars cite "nhi-thap-bat-tu" with JD-cycle method — need to verify whether the 28-star system appears within KHCBPPT or is a separate tradition
- JD epoch origin must be traced (likely from Ho Ngoc Duc's implementation or similar reference code)
- Extract complete chi→star mappings for all fixed_by_chi stars (all 12 chi values) — thorough foundation for Phase 3
- Include star quality assignments (cát/hung/bình) in Phase 1 extraction — they're needed for Phase 3 validators anyway
- Star rule sparsity (1 entry per contextual bucket) flagged for investigation — determine if rules are missing or intentionally minimal

### Intercalary month handling
- KHCBPPT's treatment of intercalary months for taboo and trực rules must be documented from the text, not inferred from the implementation
- This is a known gap — the current code may handle intercalary months differently than KHCBPPT prescribes

### Claude's Discretion
- Exact file naming within `docs/reference/khcbppt/`
- Internal organization of each subsystem reference file (table headers, section ordering)
- Whether to create an index/overview file linking all subsystem references

</decisions>

<code_context>
## Existing Code Insights

### Reusable Assets
- `baseline.json` metadata section: Contains `source_id` and `method` per subsystem — provides a checklist of what needs verification
- `data.rs` (1199 lines): Loads all almanac data — shows the exact structure the golden dataset must match
- Existing test golden files in `crates/amlich-core/tests/`: Show the expected output format for cross-referencing

### Established Patterns
- Source attribution via `_meta` objects in baseline.json: `{ "source_id": "khcbppt", "method": "table-lookup" }`
- Subsystem boundaries match code modules: star.rs, taboo.rs, day_deity.rs, truc.rs, xung_hop.rs, than_huong.rs
- Constants like `TRUC_QUALITY` in truc.rs are hardcoded in Rust, not in baseline.json — reference tables must cover these too

### Integration Points
- Phase 1 output feeds directly into Phase 2 golden dataset serialization
- The `_meta.source_id` fields in baseline.json should be updated to reflect verified sources after Phase 1
- `star_meta.source_id: "nhi-thap-bat-tu"` and `na_am_meta.source_id: "tam-menh-thong-hoi"` are the two non-"khcbppt" sources that need resolution

</code_context>

<specifics>
## Specific Ideas

- Each reference table file should mirror the structure of the corresponding baseline.json section — makes Phase 2 transformation straightforward
- The edition pinning document should be a standalone file (`docs/reference/khcbppt/EDITION.md`) that all other files reference
- For subsystems where the source is unclear or mixed, document confidence levels (high/medium/low) per data point

</specifics>

<deferred>
## Deferred Ideas

- Full star rule completeness audit (all 60 sexagenary pairs, all 10 stems, etc.) — v2 scope (STR-V2-01 through STR-V2-05)
- Sát Hướng directional verification — v2 scope (EXT-V2-01)
- Extended date range beyond 2030 — v2 scope (EXT-V2-02)

</deferred>

---

*Phase: 01-source-establishment*
*Context gathered: 2026-02-28*
