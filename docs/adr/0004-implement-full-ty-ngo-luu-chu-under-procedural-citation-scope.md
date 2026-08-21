---
status: accepted
---

# Implement full Tý Ngọ Lưu Chú under an explicit procedural-citation safety scope

Amlich will implement the true Tý Ngọ Lưu Chú (子午流注, Xu-style 納甲法 per
*Zhenjiu Dacheng* vol. 5) as the v1.11 milestone: for an anonymous user's
selected local date and time, resolve the day-stem/hour-branch driven
open-channel, five-shu/original point, and open/closed (閉穴) state, each row
cited to the classical verse and tables. The owner selected the full
point-opening computation (over a stem-channel-only layer or a text-viewer
surface) and the default consumer surface (over an opt-in practitioner view or
engine-only delivery), both conditioned on a stronger bilingual disclaimer v2
and four human review gates (classical-Chinese table sign-off, Vietnamese
point-nomenclature sign-off, health-safety sign-off including the
default-surface exposure decision, and product/legal sign-off on disclaimer
v2). Research and the divergence ledger live in
`.planning/research/TNLC_POINT_OPENING_RESEARCH.md`. This satisfies ADR 0003's
requirement that any later Tý Ngọ Lưu Chú implementation carry its own policy
contract, source ID, golden dataset, and explicit safety review.

## Consequences

- v1.11 performs the first emission of the reserved `ty-ngo-luu-chu`
  `source_id`; the fixed Tier-0 association keeps emitting
  `shi-er-jing-na-di-zhi` and the two corpora never cross-cite.
- Every table row must be frozen from a chosen facsimile before the engine
  freezes; rows transcribed from memory or modern charts are not acceptable
  corpus inputs.
- Closed (閉穴) slots serialize an explicit unavailable-by-tradition state and
  are never filled by later-school rules (recorded as `TNLC-DIV-01`) or
  converted into recommendations.
- Point output uses citation framing only: point identity (Chinese 穴名, signed
  Vietnamese huyệt danh, standard alphanumeric code as gloss), point class,
  and review state. No technique, depth, manipulation, indication,
  contraindication, or efficacy content exists in any schema or surface.
- An open state is never phrased as an action recommendation ("best time to
  treat", "hãy châm/bấm/cứu"); the extended prohibited-language guard enforces
  this.
- Tý Ngọ Lưu Chú context never changes Day Assessment, Hour Ranking,
  Direction Assessment, or the v1.10 Traditional Wellness Context; its DTO
  field is additive and separate.
- The semantic graph represents point-opening rows with citation semantics
  (e.g. `ClassicallyCitedOpenAt`), never physiological flow or performance
  claims.
- 靈龜八法 / 飛騰八法 and 養子時刻注穴法 remain out of scope and recorded only
  as divergence notes.
- Until the four gates sign, corpus rows remain `ExternalReviewPending` and
  surfaced content carries disclaimer v2; per ADR 0003, point/procedure
  outputs may be held unavailable rather than exposed unsigned.
