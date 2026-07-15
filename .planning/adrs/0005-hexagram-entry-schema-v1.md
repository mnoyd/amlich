# ADR-0005: HexagramEntry Schema v1

**Status:** Accepted
**Date:** 2026-07-16
**Deciders:** Phase 20 Foundation (v1.7 Kinh Dịch)

---

## Context

Phase 21 will author the 64-hexagram corpus (`source_id: kinh-dich`) from *Kinh Dịch Trọn Bộ* (Ngô Tất Tố translation). Per v1.7 PITFALLS.md **CRIT-1** (× 7 amplification): 64 hexagrams × ~7 text fields = **448 corpus text fields** — a schema slip AFTER corpus authoring triggers a re-edit cost roughly 7× the v1.5 rituals lesson (which itself cost ~60 entries × re-edit). Schema-lock-first is the gate that every subsequent Phase 21-25 plan waits on.

This ADR is the **third** "Foundation — Schema Lock" exercise (Phase 10 = v1.5 Ritual, Phase 16 = v1.6 ADR-0003 closure, this = v1.7 IChing). The serde discipline, additive `Option<T>` pattern, `deny_unknown_fields` corpus-entry contract, and reviewer/DeferralMarker convention are all locked by precedent (ADR-0001, RIT-13 `body_en`, RIT-14 `DeferralMarker`). What is fresh in this ADR is:

- The **field set** for `HexagramEntry` (king_wen_index, vi_name, trigrams, thoai_tu, hao_tu, cat_hung, reviewer, pending_review).
- The **`hao_tu: Vec<String>` length rule** (6 entries for hexagrams #3..=64; **7 entries for hexagrams #1 Kiền and #2 Khôn** — the *dụng cửu* / *dụng lục* seventh line). This is a loader invariant enforced in Phase 21, not a serde constraint (Rust's `Vec<String>` cannot encode "6 or 7 depending on enum value"); ADR-0005 names the rule so the loader implements it.
- The **naming-convention divergence** from the rituals schema (see Decision §3).
- The **`reviewer: String` free-text marker** shape (NOT a typed struct) and the verbatim reuse of `DeferralMarker` from `almanac/fengshui/golden.rs:85-95` for `pending_review`.
- The **`HauThienTrigram` Lo Shu encoding pin** (CRIT-3 sub-school pre-emption, see Decision §5).

The corpus loader (Phase 21) will use the `OnceLock + include_str!` pattern from `golden_loader.rs:5-21` (same as rituals). Phase 20 only locks the schema and ships a 1-entry serde round-trip probe — Phase 21 cannot begin until this ADR is Accepted AND the probe passes.

## Decision

### 1. Field set on `HexagramEntry`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HexagramEntry {
    pub king_wen_index: KingWenHexagram,            // newtype, see Plan 20-02
    pub vi_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vi_name_en: Option<String>,                 // reserved; v1.7 unpopulated
    pub upper_trigram: HauThienTrigram,             // Hậu Thiên (King Wen) display
    pub lower_trigram: HauThienTrigram,             // NOT Tiên Thiên (CRIT-3 prevention)
    pub thoai_tu: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thoai_tu_en: Option<String>,                // reserved
    pub hao_tu: Vec<String>,                        // 6 entries; 7 for #1 & #2 (loader-enforced)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hao_tu_en: Option<Vec<String>>,             // reserved; same 6/7 length rule
    pub cat_hung: String,
    pub reviewer: String,                           // ExternalReviewPending(...) free-text marker
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending_review: Option<DeferralMarker>,     // reused verbatim from golden.rs:85-95
}
```

**Required VN fields (locked v1.7 corpus content):** `king_wen_index`, `vi_name`, `upper_trigram`, `lower_trigram`, `thoai_tu`, `hao_tu`, `cat_hung`, `reviewer`.

**Reserved EN fields (schema real estate only; unpopulated in v1.7):** `vi_name_en`, `thoai_tu_en`, `hao_tu_en`. All carry `#[serde(default, skip_serializing_if = "Option::is_none")]` — this is the **RIT-13 `body_en` reservation pattern** verbatim (Phase 10). v1.7 ships Vietnamese-only; a future English-translation milestone populates these without re-locking the schema.

**Reserved deferral field:** `pending_review: Option<DeferralMarker>` — reused **verbatim** (zero new types) from `crates/amlich-core/src/almanac/fengshui/golden.rs:85-95`. The struct is `{ reason: String, expected_review_date: String, assigned_to: Option<String> }`. This is the **RIT-14 (v1.6 Phase 17) reviewer-field pattern** — the v1.6 audit established `DeferralMarker` as the canonical "this entry has a known source gap awaiting external review" marker; reusing it for IChing means the audit trail reads identically across corpora.

### 2. The `hao_tu: Vec<String>` length rule (loader invariant)

Each `HexagramEntry.hao_tu` carries **six** line-text entries for hexagrams #3..=64 (the six hào từ — *sơ hào, nhị hào, tam hào, tứ hào, ngũ hào, thượng hào*). **Hexagrams #1 (Thuần Kiền) and #2 (Thuần Khôn) carry SEVEN entries** because classical I-Ching attaches a seventh *dụng cửu* (用九, "application of nine") line to Kiền and a seventh *dụng lục* (用六, "application of six") line to Khôn.

This 6-vs-7 rule **cannot be encoded as a serde constraint** (Rust's `Vec<String>` has no length-dependent-on-other-field derive). It is therefore a **loader invariant**: the Phase 21 corpus loader asserts `hao_tu.len() == 6` for `king_wen_index ∈ 3..=64` and `hao_tu.len() == 7` for `king_wen_index ∈ {1, 2}` at load time, failing fast on violation. Phase 20's 1-entry probe exercises the 7-entry case (hexagram #2 Khôn) to prove the schema accepts it.

### 3. Naming-convention divergence from rituals (DO NOT "FIX")

The `HexagramEntry` field names **DIVERGE** from the rituals (`RitualEntry`) schema locked in ADR-0001:

| Convention | Rituals (ADR-0001) | IChing (this ADR) |
|------------|--------------------|--------------------|
| Content language marker | Suffix: `body` / `body_en` | **Prefix:** `vi_name` / `vi_name_en` |
| Romanized VN technical terms | (none) | **Unmarked:** `thoai_tu`, `hao_tu`, `cat_hung` |

This divergence is **intentional** and **locked**: `vi_name` places the language marker AT THE FRONT for content fields (the field that varies most — name, judgment, line-texts), matching the v1.7 roadmap's literal spelling. `thoai_tu` (toán từ / judgment), `hao_tu` (hào từ / line texts), and `cat_hung` (cát hung / auspicious-inauspicious) are **romanized Vietnamese technical terms left unmarked** because they are domain vocabulary, not translatable content — an English edition would still call them `thoai_tu` (or retain the VN term with an English gloss), not rename to `judgment_text`.

**A future maintainer MUST NOT normalise these to `body`/`body_en` or strip the `vi_` prefix.** Doing so would silently break the Phase 21 corpus JSON files, every consumer indexing by field name, and the v1.7 ROADMAP's literal field spelling. This divergence is documented here in the ADR so the audit trail pre-empts the "consistency" refactor.

### 4. `reviewer: String` free-text marker (NOT a typed struct)

The `reviewer` field on each `HexagramEntry` is a **free-text `String`** carrying the `ExternalReviewPending(reason="..."; expected_review_date="..."; assigned_to="...")` marker shape from `data/rituals/provenance_audit.md` (Phase 17 closure). It is **NOT** a typed struct.

**Rationale:** a free-text marker survives a reviewer-name change without schema migration. If the external `kinh-dich` reviewer changes (e.g., a different translator is engaged for an entry), the corpus editor updates the `assigned_to="..."` substring in place — no Rust type change, no serde migration, no re-lock ADR. A typed struct would force a schema-level migration for what is fundamentally editorial metadata.

The `data/iching/provenance_audit.md` aggregate ledger (referenced in Phase 21 success criterion 3) is the **aggregate audit view** across all 64 entries — it is NOT the canonical record. The canonical reviewer record is the per-entry `reviewer: String` field on `HexagramEntry` itself.

This honours the roadmap's literal "each entry carries a reviewer signature" phrasing and **intentionally diverges** from the rituals ledger-only pattern (the rituals precedent was driven by Phase 12 authoring ~60 entries in parallel; Phase 21 authors 64 with the same parallelism but the v1.7 roadmap explicitly chose entry-embedded reviewer for IChing).

### 5. `HauThienTrigram` Lo Shu encoding pin (CRIT-3 sub-school pre-emption)

`upper_trigram` and `lower_trigram` are typed `HauThienTrigram` — the **Hậu Thiên (後天, "Later Heaven" / King Wen display) trigram arrangement**, NOT the Tiên Thiên arrangement used by Mai Hoa casting (Phase 22). This is a CRIT-3 prevention measure: Tiên Thiên and Hậu Thiên share the same 8 trigram identities but use **DIFFERENT number assignments** (Tiên Thiên: Kiền=1..Khôn=8; Hậu Thiên: Lo Shu numbers Khảm=1, Khôn=2, Chấn=3, Tốn=4, Kiền=6, Đoài=7, Cấn=8, Ly=9 — **skipping 5/center**). Encoding both arrangements in one type would re-open CRIT-3; Plan 20-02 introduces three distinct newtypes (`TienThienTrigram`, `HauThienTrigram`, `KingWenHexagram`) with **NO `From` impl between them**.

The `HauThienTrigram(u8)` encoding is **pinned to the Lo Shu palace numbers** (Khảm=1, Khôn=2, Chấn=3, Tốn=4, Kiền=6, Đoài=7, Cấn=8, Ly=9 — the same assignment the existing `Palace` enum in `almanac/fengshui/types.rs:15-43` already uses). This pre-empts a documented sub-school variance: vi.wikipedia's Mai Hoa Dịch Số article shows the Lo Shu assignment as canonical but mentions a divergent placement (Ly at 5/center). Pinning to the Lo Shu palace numbers — the same numbers `Palace` already uses — closes that ambiguity. A reader encountering `HauThienTrigram::Ly` serializing to `9` (not `5`) finds the rationale in this ADR body.

### 6. `#[serde(deny_unknown_fields)]` (corpus-entry contract)

`HexagramEntry` carries `#[serde(deny_unknown_fields)]` at the struct level — typos in field names (e.g., `viName` camelCase, `hao_tu_en` missing underscore, `king_wen_idx` abbreviation) **fail at deserialization**, not silently discarded. This is the v1.5 rituals + golden-dataset contract (ADR-0001 §"Serde Discipline").

### 7. Sample JSON entry (Phase 20 1-entry probe shape)

```json
{
  "king_wen_index": 2,
  "vi_name": "Khôn / Địa",
  "upper_trigram": "khon",
  "lower_trigram": "khon",
  "thoai_tu": "Nguyên hanh, lợi mã chi trinh",
  "hao_tu": [
    "Lý sương, kiên băng chí",
    "Trực phương, đại, bất tập vô bất lợi",
    "Hàm chương, khả trinh",
    "Quát nang, vô cữu vô dự",
    "Hoàng thường, nguyên cát",
    "Long chiến dã, kỳ huyết huyền hoàng",
    "Lợi vĩnh trinh"
  ],
  "cat_hung": "thuận phục, hanh thông",
  "reviewer": "ExternalReviewPending(reason=\"Ngô Tất Tố source gap for #2 Khôn dụng lục; pending corpus authoring\"; expected_review_date=\"2026-12-31\"; assigned_to=\"external-kinh-dich-reviewer\")"
}
```

The 7-entry `hao_tu` exercises the loader-invariant rule for hexagram #2 Khôn. Reserved `*_en` and `pending_review` fields are omitted (additive `Option<T>` with `skip_serializing_if`).

## Consequences

- **Phase 20 Plan 20-02** ships the three trigram/hexagram newtypes (`TienThienTrigram`, `HauThienTrigram`, `KingWenHexagram`) per the encoding pin in §5. `HauThienTrigram` reuses the Lo Shu numbers but is a **distinct type** from `Palace` (re-aliasing to `Palace` would make trigram identity interchangeable with palace-layout position, re-opening CRIT-3 from a different angle).
- **Phase 20 Plan 20-02** ships `HexagramEntry` with `#[serde(deny_unknown_fields)]` + a passing 1-entry serde round-trip probe at hexagram #2 Khôn (exercises the 7-entry `hao_tu` rule + NFC-sensitive diacritics + `Option<DeferralMarker>`). The probe must pass BEFORE Phase 21 corpus authoring starts.
- **Phase 21** corpus authors target this locked schema. Any field addition or type change after Phase 21 corpus authoring has begun requires a superseding ADR (e.g., ADR-0005a) and a full corpus migration — expected cost: re-editing 64 entries × affected fields.
- **Phase 21 loader** implements the `hao_tu.len()` invariant from §2 (fail-fast at load; the corpus JSON itself cannot encode the rule).
- **English `*_en` field POPULATION is deferred** indefinitely; the reservation is schema real estate only (mirrors RIT-13 `body_en`).
- **The naming divergence from rituals (§3) is locked.** A future "consistency" refactor that normalises `vi_name`→`body` or strips `vi_` prefix is a regression, not a fix — the ADR body pre-empts it.
- **`reviewer` stays free-text** (§4). A typed struct migration would require a superseding ADR.
- **CRIT-6 (kinh-dich vs mai-hoa-dich-so source-id cross-contamination)** is gated by Plan 20-01's dual `pub const` registration + Plan 20-02's per-type newtype discipline; ADR-0005's contribution is the `HauThienTrigram` pin (§5) which keeps the corpus's display metadata from being round-tripped through Mai Hoa's Tiên Thiên arrangement.

## References

- `.planning/adrs/0001-ritual-schema-v1.md` — Nygard short-form template; serde discipline precedent (`deny_unknown_fields`, `Option<T>` + `skip_serializing_if`); `body_en` reservation (RIT-13).
- `crates/amlich-core/src/almanac/fengshui/types.rs:15-43` — `Palace` enum (Lo Shu numbers; direct precedent for `HauThienTrigram` encoding).
- `crates/amlich-core/src/almanac/fengshui/golden.rs:85-95` — `DeferralMarker` struct (reused verbatim for `pending_review`).
- `data/rituals/provenance_audit.md` — `ExternalReviewPending(reason="..."; expected_review_date="..."; assigned_to="...")` free-text marker shape (reused for `reviewer: String`).
- `.planning/phases/20-foundation-schema-lock-source-ids-adrs-ontology/20-CONTEXT.md` §"HexagramEntry schema" — locks the field set + the naming divergence + the reviewer free-text choice.
- `.planning/phases/20-foundation-schema-lock-source-ids-adrs-ontology/20-RESEARCH.md` §"Pitfall 1" (Hậu Thiên sub-school variance) + §"Pattern 3" (Hậu Thiên vs Tiên Thiên on HexagramEntry).

---

*Adopted: 2026-07-16 (Phase 20-01)*
*No supersessions. Sibling to ADR-0001 (ritual schema). CRIT-1 × 7 schema-lock-first gate.*
