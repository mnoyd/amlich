# v1.10 Traditional Wellness Context — External Review Coordination

**Audience:** Amlich maintainer / milestone coordinator (Hoang Manh and successors).
**Purpose:** One-page playbook that moves bead `amlich-l2zc.1` and `amlich-l2zc.2` from "implementation done, awaiting external sign-off" to "signed and ready to close the milestone."
**Last touched:** 2026-08-16 (by `amlich-l2zc.4`-prep session).

This document is the **coordinator's** artifact. It does not duplicate the
reviewer-facing packs; it points at them. The packs themselves are the
authoritative thing a reviewer signs.

---

## 1. The four gates, at a glance

| # | Gate | Role | Packet | Signs against | Required for |
|---:|---|---|---|---|---|
| 1 | classical-Chinese (Gate 1) | `classical_chinese_reviewer` | `.planning/milestones/v1.10-phases/01-hour-branch-channel-association/REVIEWER-PACK.md` §B | §A.4 12-row table vs. `Zhenjiu Daquan` v.5 `十二經納地支歌` (see §A.6.a) | `amlich-l2zc.1` |
| 2 | Suwen paraphrase (Gate 2) | `suwen_paraphrase_reviewer` | `.planning/milestones/v1.10-phases/02-seasonal-cultivation-context/REVIEWER-PACK.md` §B | §A.4 four paraphrases vs. Suwen ch. `四氣調神大論篇第二` (see §A.7.a) | `amlich-l2zc.2` |
| 3 | product/legal (Gate 3) | `product_legal_reviewer` | both packs §D | the bilingual disclaimer in §A.1/§A.2 | `amlich-l2zc.1` and `amlich-l2zc.2` (separate signatures, one per packet) |
| 4 | health-safety (Gate 4) | `health_safety_reviewer` | both packs §C | the wording + lexical/schema guards + no clinical fields | `amlich-l2zc.1` and `amlich-l2zc.2` (separate signatures, one per packet) |

Each gate corresponds to an engagement bead (`amlich-l2zc.5`–`.8`) so the
sign-off is trackable in `bd ready`.

## 2. The packets are ready to send

Both `REVIEWER-PACK.md` files are version-controlled, byte-locked against the
corpus, and contain:

- the bilingual disclaimer (byte-locked to `crates/amlich-core/src/traditional_wellness/disclaimer.rs`);
- the 12-row table (`.1`) or four paraphrases (`.2`);
- the divergence markers;
- the candidate edition URIs (so the reviewer does not have to hunt);
- a sign-off block per gate (B / C / D);
- the read-only implementation-boundary context (§E);
- a packet-history footer (§F);
- **§G — Reviewer outreach** (the how-to-send-this-pack checklist, added
  2026-08-16) so the coordinator does not have to re-derive the protocol.

Pre-flight before sending any pack:

```bash
# Confirm the pack matches the corpus (byte-locked contracts)
cargo test -p amlich-core --test prohibited_language_guard
cargo test -p amlich-core --test branch_channel_integration
cargo test -p amlich-core --test seasonal_cultivation_integration
# Confirm Phase 03 unified surface still passes
cargo test -p amlich-api --test traditional_wellness_cross_surface
cargo test -p amlich-core --test semantic_graph_traditional_wellness_integration
cargo test --workspace
```

All five suites must be green at the moment the pack is sent.

## 3. Recommended send order

Gates are not strictly order-dependent, but the recommended sequence minimizes
total wall-clock if any gate discovers a correction:

1. **Gate 1 (classical-Chinese) first.** A correction here would change the
   12-row table, which is the spine of the branch-channel surface. Catching it
   before Gates 3/4 saves review cycles. ~3–5 days expected return.
2. **Gate 2 (Suwen paraphrase) in parallel.** Different corpus, different
   reviewer, no shared text. Can run concurrently with Gate 1. ~3–5 days.
3. **Gate 3 (product/legal) after Gates 1 + 2.** The legal reviewer benefits
   from seeing the final, signed corpus before signing the disclaimer.
   ~2–3 days.
4. **Gate 4 (health-safety) last.** Same reason as Gate 3 — wait for any
   wording changes from Gates 1/2 to land before signing the safety check.
   ~3–5 days.

## 4. Per-gate send procedure

For each gate, the coordinator:

1. Identifies a candidate reviewer matching the role (see §6 below).
2. Confirms the packet version in `§F` matches the live corpus.
3. Confirms the candidate edition list in `§A.6.a` / `§A.7.a` has at least one
   URI the reviewer can actually consult.
4. Sends the full `REVIEWER-PACK.md` text plus the §G.3 return-format note.
5. Files the engagement bead (see §5) so the sign-off has a tracker.
6. Updates the engagement bead status to `in_progress` when sent.
7. On return: commits the signed pack to `docs/reviews/v1.10/<role>-<gate>-<YYYY-MM-DD>.md`,
   records the PR or file reference in the engagement bead, and closes the
   engagement bead per §G.5 of the relevant pack.

## 5. Engagement beads

These four beads are the per-gate tracker. They are filed by this session and
linked back to `amlich-l2zc.1` / `.2` via `discovered-from`.

- `amlich-l2zc.5` — Gate 1 classical-Chinese sign-off (blocks `amlich-l2zc.1`)
- `amlich-l2zc.6` — Gate 2 Suwen paraphrase sign-off (blocks `amlich-l2zc.2`)
- `amlich-l2zc.7` — Gate 3 product/legal sign-off, both packs (blocks both
  `.1` and `.2`; the bead has two AC items, one per pack)
- `amlich-l2zc.8` — Gate 4 health-safety sign-off, both packs (blocks both
  `.1` and `.2`; same pattern)

The `expected_review_date` of `2026-12-31` (per `docs/architecture/external-review-lifecycle.md`)
is the latest acceptable date; reviewer engagements should target an earlier
date where possible.

## 6. Candidate reviewer channels (suggested, not endorsed)

This project does not endorse specific individuals. The list below is the
**kind** of channel a maintainer might approach; the maintainer is responsible
for the final choice and any honorarium, NDA, or licensing arrangement.

### Gate 1 (classical-Chinese, `Zhenjiu Daquan` v.5)

- A faculty member in classical Chinese medical literature at a university
  TCM / acupuncture program (e.g. 广州中医药大学, 北京中医药大学, 上海中医药大学,
  南京中医药大学 Chinese-medicine literature departments).
- A practitioner-researcher who has published on `子午流注` / `納甲法`
  philology or on *Zhenjiu Daquan* / *Zhenjiu Dacheng* textual history.
- The ctext.org editorial network sometimes routes questions to volunteer
  editors; their consent is required before any pack text is shared.

### Gate 2 (Suwen paraphrase, `四氣調神大論`)

- A faculty member in *Neijing* studies or Suwen textual scholarship at a
  TCM university.
- A scholar-practitioner with peer-reviewed publications specifically on
  chapter 2 of Suwen or on the `四氣調神` interpretation tradition.
- The 人民卫生出版社 《黄帝内经素问》 校释本 editorial team, if accessible
  through a professional introduction.

### Gate 3 (product/legal, bilingual disclaimer)

- A Vietnamese-speaking product counsel familiar with the Ministry of Health
  classification of "general wellness" / "thông tin sức khỏe" content and
  FDA General Wellness policy as a comparative vocabulary reference.
- An English-speaking product counsel who can confirm the disclaimer wording
  for U.S. distribution. The disclaimer model is FDA General Wellness (see
  `LUNAR_HEALTH_RESEARCH.md:122, 231`) but only as a vocabulary boundary, not
  legal advice.

### Gate 4 (health-safety, lexical + schema scope)

- A clinician (physician or licensed TCM practitioner) who has *not* been a
  contributor to this codebase, to avoid reviewer-equals-author bias.
- A public-health reviewer with experience reviewing consumer health-content
  copy.
- A consumer-protection or health-content regulator who is willing to review
  in a non-binding capacity.

## 7. What changes after all four gates sign

Once all four engagement beads (`amlich-l2zc.5`–`.8`) are closed:

1. The corpus JSON files (`branch-channel.json`, `seasonal-cultivation.json`)
   have their `PENDING_CLASSICAL_REVIEW` / `PENDING_SUWEN_PARAPHRASE_REVIEW`
   placeholders replaced with the recorded `edition_or_facsimile_uri` and the
   `ExternalReviewPending(...)` literals replaced with `Signed(...)`.
2. `amlich-l2zc.1` and `amlich-l2zc.2` close with bead comments citing the
   signed packs in `docs/reviews/v1.10/`.
3. `amlich-l2zc.3` (unified explanation) closes; its implementation is
   already green and its only AC gate is the sign-off on `.1` + `.2`.
4. `amlich-l2zc.4` (audit/release) unblocks; it re-runs all eight v1.10
   requirements and the four gates and publishes the milestone audit.

The Active Register rows in `docs/architecture/external-review-lifecycle.md`
(rows 7–10 in the current register, covering the four v1.10 gates) move from
"open" to "resolved" with the sign-off date, URI, and reviewer identity.

## 8. What does NOT change after sign-off

The four out-of-scope items in `.planning/milestones/v1.10-REQUIREMENTS.md`
(§"Explicitly out of scope") stay out of scope. The sign-off does **not**
authorize any of:

- full `子午流注` / `納甲法` / timed point opening;
- disease / symptom / risk / prevention / treatment claims;
- food / herb / supplement / fasting / quantified sleep-exercise prescriptions;
- 24 solar-term regimens or local-weather claims;
- Tier-2 Bazi personalization.

If a future bead wants to ship any of these, it requires a new milestone, a
new REVIEWER-PACK, and a new round of gates — see `amlich-xlag` for the
deferred post-v1.10 portfolio.

## 9. Failure modes and fallbacks

- **Reviewer declines the engagement.** File a new bead discovered-from
  `amlich-l2zc.5`–`.8` with the decline reason, the next candidate reviewer,
  and a new target date. Do not silently roll the date forward (per the
  external-review lifecycle policy, §"Lifecycle" item 5).
- **Reviewer requests correction.** They mark the relevant row `corrected` in
  §B and propose the canonical reading. The implementation owner files a
  regression-fixture bead, updates the corpus JSON, re-runs the four
  byte-locked test suites, then re-issues the pack with the correction. The
  engagement bead stays open until the corrected pack is re-signed.
- **Reviewer returns a partial sign-off** (e.g. signs the disclaimer but
  flags a wording concern). The engagement bead stays open; the concern is
  filed as a discovered-from bead with its own priority.
- **All four gates signed but `amlich-l2zc.3` still fails a gate.** This is a
  Phase 03 bug, not a review issue. File it as a `bug` bead discovered-from
  `amlich-l2zc.3` and fix it before closing `.4`.

## 10. References

- `docs/architecture/external-review-lifecycle.md` — the canonical policy.
- `.planning/milestones/v1.10-REQUIREMENTS.md:94-105` — the four human review
  gates in the requirements doc.
- `.planning/milestones/v1.10-phases/01-hour-branch-channel-association/REVIEWER-PACK.md`
- `.planning/milestones/v1.10-phases/02-seasonal-cultivation-context/REVIEWER-PACK.md`
- `.planning/research/LUNAR_HEALTH_RESEARCH.md` — the research base.
- `docs/adr/0003-separate-branch-channel-association-from-ty-ngo-luu-chu.md`
  — the scope-split ADR.