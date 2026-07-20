---
phase: 21-iching-corpus-loader
plan: 01
subsystem: database
tags: [iching, kinh-dich, ngo-tat-to, corpus, af-05, pending-external-review, king-wen, hau-thien, nfc, provenance]

# Dependency graph
requires:
  - phase: 20-foundation-schema-lock-source-ids-adrs-ontology
    provides: Locked HexagramEntry schema (deny_unknown_fields + additive Option<T>) + 64-entry bijective COMPOSITION_TABLE + DeferralMarker reuse (Plan 20-02)
  - phase: 17-rituals-crit3-source-id
    provides: ExternalReviewPending(reason=...; expected_review_date=...; assigned_to=...) free-text reviewer marker shape + Phase 17 closure template (provenance_audit.md)
provides:
  - "64-entry hexagrams.json corpus (envelope {$schema_version: iching-v1}) authored against the locked HexagramEntry schema — structural fields populated, interpretive text honestly deferred per AF-05"
  - "64-row provenance_audit.md ledger mirroring the Phase 17 closure template — all entries ExternalReviewPending"
  - "Reserved *_en fields OMITTED in every entry (additive Option<T> + skip_serializing_if discipline)"
affects:
  - 21-iching-corpus-loader (Plan 21-02 — OnceLock loader + get_hexagram/all_hexagrams API consuming this corpus)
  - 22-mai-hoa-casting-bien-que-the-dung (Mai Hoa casting -> King Wen index -> corpus lookup)
  - 24-iching-evaluator-semantic-graph-wiring-dto (semantic-graph Hexagram nodes sourced from this corpus)

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Data-only corpus authoring against a locked serde schema (deny_unknown_fields catches field-name typos at load, not authoring time)"
    - "Python generator script for deterministic corpus derivation from a const composition table (NFC-normalized, no manual transcription errors)"
    - "Byte-identical reviewer marker string shared between canonical per-entry record (hexagrams.json) and aggregate audit view (provenance_audit.md)"
    - "Honest deferral discipline: structural fields ARE populated (king_wen_index, vi_name, trigrams); interpretive text (thoai_tu, hao_tu, cat_hung) carries [PendingExternalReview — ...] placeholders, NOT fabricated from another translator (AF-05)"

key-files:
  created:
    - crates/amlich-core/data/iching/hexagrams.json
    - crates/amlich-core/data/iching/provenance_audit.md
  modified: []

key-decisions:
  - "Hexagram vi_name values come from the COMPOSITION_TABLE comments in schema.rs:182-247 (Hán-Việt classical names like 'Thuần Kiền', 'Truân', 'Thái') — these are standard Hán-Việt names, NOT Ngô Tất Tố's unique contribution, so they ARE safe to populate (per Plan design_decisions)"
  - "Trigram identities map by NAME between TienThienTrigram and HauThienTrigram (both #[serde(rename_all = \"snake_case\")]): COMPOSITION_TABLE[i].0 = TienThienTrigram::Kien -> upper_trigram: \"kien\" (deserialises to HauThienTrigram::Kien). Same identity, different discriminant — CRIT-3-safe."
  - "hao_tu placeholder strings carry the Vietnamese line position name (sơ hào / nhị hào / tam hào / tứ hào / ngũ hào / thượng hào, plus dụng cửu for #1 Kiền and dụng lục for #2 Khôn) so a future reviewer can fill each slot deterministically"
  - "Python generator script (/tmp/opencode/gen_hexagrams.py + gen_provenance.py) ensures byte-identical reviewer marker strings across hexagrams.json and provenance_audit.md — no copy-paste drift"
  - "Provenance ledger grouped under 8 octant sub-headings (#1-8 ... #57-64) mirroring the rituals ledger's per-category sub-headings for readability"

patterns-established:
  - "Generator-driven corpus authoring for any future large deterministic dataset derived from a const table (avoids 64× manual transcription; NFC + structural invariants verified by post-generation cross-checks)"
  - "Dual-surface reviewer record: canonical = per-entry reviewer: String field (survives reviewer-name change without schema migration per ADR-0005 §4); aggregate = provenance_audit.md ledger (human-readable audit). Both byte-identical."

requirements-completed: [ICH-01]

# Metrics
duration: 7 min
completed: 2026-07-16
---

# Phase 21 Plan 01: IChing Corpus Data Summary

**64-hexagram Ngô Tất Tố corpus JSON authored against the locked Phase 20 HexagramEntry schema — structural fields (vi_name, trigrams) deterministically derived from COMPOSITION_TABLE, interpretive text (thoai_tu, hao_tu, cat_hung) honestly deferred as [PendingExternalReview] placeholders per AF-05, paired with a 64-row provenance audit ledger mirroring the Phase 17 closure template**

## Performance

- **Duration:** 7 min (419 s)
- **Started:** 2026-07-16T01:56:18Z
- **Completed:** 2026-07-16T02:03:17Z
- **Tasks:** 2 (both `type="auto"`, no checkpoints — Pattern A autonomous)
- **Files created:** 2 (hexagrams.json, provenance_audit.md)

## Accomplishments

- **64-entry corpus JSON authored** against the locked `HexagramEntry` schema (ADR-0005) — envelope `{"$schema_version": "iching-v1", "entries": [...]}` mirrors the v1.5 rituals corpus discipline. Every King Wen index 1..=64 appears exactly once in ascending order.
- **Structural fields deterministically derived** from `COMPOSITION_TABLE` in `iching/schema.rs:182-247`: `vi_name` (Hán-Việt classical names — Thuần Kiền, Truân, Thái, ...), `upper_trigram`/`lower_trigram` (snake_case identity matching each `(TienThienTrigram, TienThienTrigram)` tuple). Cross-checked: all 64 entries' trigram identities match the table.
- **`hao_tu` length rule honored** (ADR-0005 §2): hexagrams #1 Kiền and #2 Khôn carry **7** entries (the seventh *dụng cửu* / *dụng lục* line); hexagrams #3..=64 carry **6** entries (sơ/nhị/tam/tứ/ngũ/thượng hào). Verified by the plan's automated invariant check.
- **AF-05 deferral discipline**: every entry's interpretive text (`thoai_tu`, `hao_tu`, `cat_hung`) is an honest `[PendingExternalReview — ...]` placeholder — no Ngô Tất Tố text fabricated, no silent fill from Richard Wilhelm / Gregory Whincup / another translator. Every entry carries `pending_review: {reason, expected_review_date: "2026-12-31", assigned_to: "external-kinh-dich-reviewer"}` + the `ExternalReviewPending(...)` reviewer marker.
- **Reserved `*_en` fields OMITTED** in every entry (additive `Option<T>` + `skip_serializing_if` — absent in JSON means `None` at deserialise time). Mirrors RIT-13 `body_en` reservation discipline.
- **64-row provenance audit ledger** mirrors the Phase 17 closure template — header block documenting the AF-05 closure policy, exactly 64 data rows grouped under 8 octant sub-headings (#1-8 ... #57-64), all dispositioned `ExternalReviewPending`, reviewer marker strings byte-identical to `hexagrams.json` (cross-file verified).
- **All Vietnamese text is NFC-normalized** (precomposed diacritics — "Thuần Kiền", "Thuần Khôn", "Đại Tráng", etc.) — verified across every text field in both files.
- **Zero Rust code touched** — pure data authoring. `cargo build -p amlich-core` stays green; the existing Phase 20 1-entry serde probe still passes.

## Task Commits

Each task was committed atomically:

1. **Task 1: Author the 64-hexagram corpus JSON** — `760b2d9` (feat)
   - `crates/amlich-core/data/iching/hexagrams.json` (1415 lines, 101 KB)
   - 64 `HexagramEntry` records + `{"$schema_version": "iching-v1"}` envelope
   - Plan's automated invariant check passes (64 entries, indices 1..=64, 7/6 hao_tu rule, all carry pending_review + reviewer marker, no reserved `*_en` fields)
   - Additional cross-checks: trigram identities match COMPOSITION_TABLE (64/64); all text NFC-normalized; reviewer marker byte-identical across all entries; #1/#2 carry dụng cửu/dụng lục seventh line
2. **Task 2: Author the 64-row provenance audit ledger** — `24e8fbc` (docs)
   - `crates/amlich-core/data/iching/provenance_audit.md` (142 lines)
   - Header block + AF-05 closure policy + 8-octant-grouped 64-row ledger + References section
   - Plan's automated row-count check passes (`rg -c "^\| [0-9]+ \|" == 64`)
   - Additional cross-checks: all 64 reviewer markers byte-identical to hexagrams.json; all vi_names match; all outcomes = ExternalReviewPending; file fully NFC-normalized; 8 octant sub-headings present

**Plan metadata:** (pending final commit below)

## Files Created/Modified

- `crates/amlich-core/data/iching/hexagrams.json` — 64-entry Ngô Tất Tố corpus JSON. Envelope `{"$schema_version": "iching-v1", "entries": [...]}`. Each entry: `king_wen_index` (1..=64), `vi_name` (Hán-Việt from COMPOSITION_TABLE comments), `upper_trigram`/`lower_trigram` (snake_case identity), deferred `thoai_tu`/`hao_tu`/`cat_hung`, `reviewer` free-text marker, `pending_review` DeferralMarker.
- `crates/amlich-core/data/iching/provenance_audit.md` — Aggregate provenance ledger. Header block with AF-05 closure policy + 64-row table grouped under 8 octant sub-headings + References section. Mirrors the Phase 17 rituals closure template.

## Decisions Made

- **vi_name values are safe to populate** because they come from the COMPOSITION_TABLE comments (Hán-Việt classical names like "Thuần Kiền", "Truân", "Thái") — these are standard Hán-Việt hexagram names, NOT Ngô Tất Tố's unique textual contribution. Per the plan's `<design_decisions>`: the corpus is a faithful skeleton, not hollow.
- **Trigram identity mapping is CRIT-3-safe.** `upper_trigram`/`lower_trigram` in JSON are the snake_case variant names (`"kien"`, `"khon"`, `"kham"`, etc.) which deserialise to `HauThienTrigram` variants. The IDENTITY matches `COMPOSITION_TABLE[i].0/.1` (a `TienThienTrigram`) by variant NAME — both enums carry `#[serde(rename_all = "snake_case")]`. The discriminants differ (Tiên Thiên Kiền=1 vs Hậu Thiên Kiền=6) but the JSON name is the same; CRIT-3 isolation is preserved at the type level (no `From` impl).
- **Generator-driven authoring (Python).** Both files are produced by deterministic Python scripts (`/tmp/opencode/gen_hexagrams.py`, `/tmp/opencode/gen_provenance.py`) that re-declare the 64-hexagram table. This eliminates 64× manual transcription risk and guarantees byte-identical reviewer marker strings across the two surfaces. NFC normalisation applied via `unicodedata.normalize("NFC", ...)` on every Vietnamese string.
- **`hao_tu` placeholders carry the Vietnamese line position name** (sơ hào / nhị hào / tam hào / tứ hào / ngũ hào / thượng hào, plus dụng cửu for #1 and dụng lục for #2) — so a future reviewer filling each slot has an unambiguous per-line target.

## Deviations from Plan

None - plan executed exactly as written.

Both tasks were `type="auto"` with no checkpoints; Pattern A (autonomous) execution. No deviation rules (1-4) triggered. No authentication gates. The plan's prescribed verbatim text for `thoai_tu`, `cat_hung`, `hao_tu` placeholders, the `reviewer` marker, and the `pending_review` DeferralMarker was followed exactly (cross-file byte-equality verified between `hexagrams.json` and `provenance_audit.md`).

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required. This plan is pure data authoring; no new dependencies, no environment variables, no dashboards.

## Next Phase Readiness

- **Corpus is ready for Plan 21-02** (the CODE half of ICH-01). The loader will `include_str!("data/iching/hexagrams.json")`, parse it once via `OnceLock`, NFC-normalise every text field at load (RIT-08 precedent), enforce the `hao_tu.len()` invariant (6 for #3..=64; 7 for #1 & #2 — ADR-0005 §2), and expose `get_hexagram(KingWenHexagram) -> Option<&'static HexagramEntry>` + `all_hexagrams() -> &'static [HexagramEntry]`. Mirrors the v1.5 `rituals/corpus.rs` pattern.
- **`$schema_version: "iching-v1"`** is the contract the loader will assert at load (mirrors `EXPECT_SCHEMA_VERSION = "rituals-v1"` in `rituals/corpus.rs:76`).
- **Schema is `deny_unknown_fields`** — any field-name typo in the corpus fails loudly at deserialise time, but the Python generator + automated invariant check have already verified every entry is well-formed.
- **ICH-01 closure is split across 21-01 (this plan, DATA) + 21-02 (CODE)**. This plan delivers the corpus; 21-02 delivers the loader + lookup API + integration tests. ICH-01 is fully closeable once 21-02 ships (its 4 success criteria span both plans).
- **No blockers.** Plan 21-02 can proceed immediately against this corpus.

---
*Phase: 21-iching-corpus-loader*
*Completed: 2026-07-16*

## Self-Check: PASSED

- All 3 created files exist on disk: `hexagrams.json` (1415 lines, 101 KB), `provenance_audit.md` (142 lines), `21-01-SUMMARY.md`.
- Both task commits exist: `760b2d9` (feat — corpus JSON), `24e8fbc` (docs — provenance ledger).
- `hexagrams.json`: 64 entries, `$schema_version == "iching-v1"`, all King Wen indices 1..=64 present once, #1/#2 carry 7 hao_tu (dụng cửu / dụng lục), #3..=64 carry 6 hao_tu, every entry carries `pending_review` + `ExternalReviewPending` reviewer marker, no reserved `*_en` fields present.
- `provenance_audit.md`: exactly 64 ledger rows, all dispositioned `ExternalReviewPending`, reviewer marker strings byte-identical to `hexagrams.json` (cross-file verified).
- Trigram identities verified to match `COMPOSITION_TABLE` in `iching/schema.rs:182-247` for all 64 entries.
- All Vietnamese text in both files is NFC-normalized.
- `cargo build -p amlich-core` stays green (data-only plan, zero Rust code touched).
- Existing Phase 20 1-entry serde probe (`tests/iching_schema_probe.rs`) still passes — no regression.
