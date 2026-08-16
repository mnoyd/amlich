# External Review Deferral Lifecycle

Status: Canonical maintainer policy

Owner: Amlich maintainers

Last reviewed: 2026-08-10

This document is the source of truth for domain claims that cannot be closed
honestly without a qualified external reviewer or a specific physical source.
It governs the labels `PendingExternalReview`, `ExternalReviewPending`, and the
typed `DeferralMarker` representation.

## When to defer

Create an external-review deferral only when the implementation or corpus has
enough evidence to preserve a useful provisional result, but one of these
conditions remains true:

- the canonical edition or exact page is unavailable;
- qualified domain review is unavailable;
- primary and secondary sources disagree in a way maintainers cannot settle;
- filling a gap would mix translators, traditions, or rulesets.

A deferral must not hide a failing test, missing ordinary engineering work, or
an unsupported guess. If the provisional result could cause an unsafe or
materially misleading conclusion, keep that result unavailable instead.

## Required record

Every deferral records the following information in its authoritative artifact:

- a stable subject or case identifier;
- the literal disposition `PendingExternalReview` or
  `ExternalReviewPending`;
- a specific reason and the evidence already checked;
- the provisional operational behavior, if any;
- `expected_review_date` in `YYYY-MM-DD` form;
- `assigned_to`, naming a reviewer role rather than inventing a person;
- the artifact that must change when review completes.

Use a typed `DeferralMarker` when the owning data schema already has or can add
one compatibly. Use `ExternalReviewPending(...)` in a frozen free-text corpus
field when changing its schema would create unnecessary migration work. Use a
Markdown ledger or ADR for edition/page gaps that do not belong in runtime
data. Mirrored records must agree with their authoritative artifact.

## Lifecycle

1. **Record** — add the marker and its reason, owner, due date, provisional
   behavior, and evidence location in the same change that introduces or
   discovers the uncertainty.
2. **Gate** — add a test when machine-readable state or runtime behavior could
   drift. Documentation-only page gaps must be linked from the implementing
   module or its provenance ledger.
3. **Review** — the assigned reviewer checks the named edition/source and
   records reviewer identity, review date, source citation, and outcome:
   `confirmed`, `corrected`, or `disputed`.
4. **Resolve** — update the authoritative artifact, all mirrors, tests, and any
   affected ADR. Remove the pending marker only when the evidence is committed.
   Corrections require a regression fixture and compatibility assessment.
5. **Escalate or re-defer** — before the due date, unresolved items must receive
   a new bead linked to the originating milestone. Record the attempted review,
   explain why it remains blocked, and set a new owner and date. Never silently
   roll a date forward.

Milestone audits list unresolved external deferrals separately from code gaps.
They may ship when the provisional behavior is explicitly bounded and tested;
they do not count as resolved requirements.

## Active register

| Case | Authoritative artifact | Representation | Provisional behavior | Owner | Review date |
|---|---|---|---|---|---|
| 64-hexagram Ngô Tất Tố interpretive text | `crates/amlich-core/data/iching/hexagrams.json` and `provenance_audit.md` | `ExternalReviewPending(...)` per corpus row | Do not fill from another translator | `external-kinh-dich-reviewer` | 2026-12-31 |
| Tam Sát KHCBPPT edition/page pin | `crates/amlich-core/data/almanac/tam_sat_provenance.md` | Markdown `PendingExternalReview` ledger | Keep the tested opposite-triad mapping; label its page citation pending | `external-khcbppt-reviewer` | 2026-12-31 |
| 1960 Trung Nguyên center-star split | `crates/amlich-core/data/almanac/flying_stars_golden.json` | typed `DeferralMarker` plus ADR-0003a | Keep center 5 as the explicit provisional tiebreaker | `external-huyen-khong-reviewer` | 2026-12-31 |
| ADR-0004 daily Phi Tinh page pin | `.planning/adrs/0004-daily-phi-tinh-starting-star-convention.md` | ADR `PendingExternalReview` note | Keep the chapter/verse-backed and golden-tested convention | `external-huyen-khong-reviewer` | 2026-12-31 |
| Văn khấn corpus independent review | `crates/amlich-core/data/rituals/provenance_audit.md` and corpus reviewer fields | `ExternalReviewPending(...)` per corpus row | Preserve cited text; do not fabricate reviewer identities | `external-vn-folk-ritual-reviewer` | 2026-12-31 |
| v1.10 Twelve-Branch Channel Association (Thập nhị kinh nạp địa chi) — 12-row corpus | `crates/amlich-core/data/traditional-wellness/branch-channel.json` and `src/traditional_wellness/branch_channel.rs` | `ExternalReviewPending(reason="classical_12_row_table_review_pending"; expected_review_date="YYYY-MM-DD"; assigned_to="classical_chinese_reviewer")` per row | Do not flip rows to `Signed` until the classical-Chinese reviewer signs the chosen facsimile; every snapshot carrying this context emits the bilingual disclaimer | `classical-chinese-reviewer` | 2026-12-31 |
| v1.10 Bilingual cultural-information disclaimer | `crates/amlich-core/src/traditional_wellness/disclaimer.rs` and REVIEWER-PACK §A.1/§A.2 (Phase 01 + Phase 02 packs) | `DisclaimerId("cultural_information_v1")` with byte-locked Vietnamese + English strings | Do not edit the disclaimer text without product/legal re-sign-off; `tests/prohibited_language_guard.rs::bilingual_disclaimer_is_byte_identical_to_reviewer_pack` enforces the lock against both packs | `product-legal-reviewer` | 2026-12-31 |
| v1.10 Four-season Suwen cultivation corpus (Tứ khí điều thần) | `crates/amlich-core/data/traditional-wellness/seasonal-cultivation.json` and `src/traditional_wellness/seasonal.rs` | `ExternalReviewPending(reason="suwen_four_season_paraphrase_review_pending"; expected_review_date="2026-12-31"; assigned_to="suwen_paraphrase_reviewer")` per profile | Do not flip profiles to `Signed` until the Suwen paraphrase reviewer signs against `四氣調神大論`; every result exposes the bilingual disclaimer and the term-to-season composition disclosure (LH-DIV-04) | `suwen-paraphrase-reviewer` | 2026-12-31 |
| v1.10 Seasonal wellness copy replacement (displaced Tiết khí health lists) | `crates/amlich-core/data/tiet-khi.json` + root `data/tiet-khi.json` (emptied `health` lists) and Phase 02 REVIEWER-PACK §A.8 | Empty `health.vi`/`health.en` arrays with an in-file do-not-refill note | The unsourced lists stay empty (locked by `insight_parity.rs` and `insight_data.rs` tests); refill only from the reviewed seasonal corpus after Gates 2–4 sign | `health-safety-reviewer` | 2026-12-31 |

## Resolution checklist

- The authoritative source or qualified reviewer is identified.
- Exact citation and review outcome are recorded.
- Corpus, ledger, typed marker, and ADR mirrors agree.
- Provisional values are confirmed or corrected with regression coverage.
- Public contract impact is assessed when serialized data changes.
- The linked bead is closed with the evidence and commit reference.
