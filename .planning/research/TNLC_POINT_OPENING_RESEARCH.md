# Tý Ngọ Lưu Chú (子午流注) Point-Opening Research

**Domain:** Tier-1 classical day/hour acupuncture-point opening method (Xu-style 納甲法)
**Researched:** 2026-08-21
**Confidence:** HIGH on the method's structure, lineage, and separation from the Tier-0 fixed association; MEDIUM on the table inventory (which sections of *Zhenjiu Dacheng* vol. 5 hold which rows); **TABLE-LEVEL ROWS ARE NOT FROZEN** — every per-day/per-hour point assignment must be verified against a chosen facsimile before the engine corpus freezes (Track 1).
**Candidate milestone:** v1.11 — full point-opening computation, default consumer surface (owner decision 2026-08-21)
**Prior research:** [`LUNAR_HEALTH_RESEARCH.md`](LUNAR_HEALTH_RESEARCH.md) §1.1 established the Tier-0/Tier-1 separation; this note extends it to the full method.

Claims follow the repo convention:

- **[SOURCE]** — stated by a classical text, facsimile/transcription, or official institution.
- **[DESIGN]** — an Amlich product/engineering inference; must never be presented as classical or medical fact.

## Executive decision (owner, 2026-08-21)

1. Implement the **full point-opening computation**: day-stem → channel assignment, five-shu/original point resolution, and open/closed (閉穴) hour state per the Xu-style 納甲法.
2. Surface it on the **default consumer surface** (TUI + desktop inspector), not an opt-in practitioner view and not engine-only.
3. Both decisions are conditioned on a **stronger bilingual disclaimer v2** and **four human review gates** (§Review gates). Until gates sign, all point rows ship `ExternalReviewPending` and surfaced content carries disclaimer v2.
4. ADR 0003's reservation is satisfied: v1.11 performs the first emission of `source_id: ty-ngo-luu-chu`, under its own policy contract (`TY_NGO_LUU_CHU_POLICY_V1`), golden dataset, and safety review.

## 1. What the classical method actually is

### 1.1 Structure [SOURCE]

- *Zhenjiu Dacheng* (*Châm Cứu Đại Thành*) volume 5 contains the `徐氏子午流注逐日按時定穴歌` (Xu-style song for selecting points by day and hour), day/hour tables, assignments of channels to the ten Heavenly Stems, five-shu points, original (原) points, generating/combining (生/合) rules, and rules for points being open or closed. Examples change the selected channel and point according to **both the day's stem and the hour's stem/branch**; the Jiǎ-day sequence begins at the Gallbladder well point at Jiǎ-Xū (甲戌) hour and proceeds through other channels and point classes. See the [volume 5 transcription with linked page images](https://www.shidianguiji.com/book/NA08718/chapter/1l0gx6qnyiz2h), sections `徐氏子午流注逐日按時定穴歌`, `流注圖`, `論子午流注法`; the [CTEXT facsimile catalog](https://ctext.org/library.pl?if=en&res=2772) identifies the scanned work as Yang Jizhou's *Zhenjiu Dacheng*.
- A 2025 WFCMS training notice describes the `納甲法` as a **ten-day timed structure** combining the twelve regular channels, ten Heavenly Stems, five-shu points, five-phase generation, and original points, tracing the lineage through Yan Mingguang and Xu Feng into *Zhenjiu Dacheng*. See the [WFCMS course description](https://wfcms.org/show/11/7609.html). This first-party description agrees with the structure visible in the classical text.
- The text's own explanation defines `子午流注` via yin-yang pairing, cyclical qi/blood, and timed opening/closing of points (CTEXT *Zhenjiu Daquan*, [`卷之五 論子午流注之法`](https://ctext.org/wiki.pl?chapter=688012&if=en)).

### 1.2 What v1.10 shipped, and why this is different [SOURCE]

v1.10's fixed Earthly-Branch → channel lookup is separately titled `十二經納地支` (in *Zhenjiu Daquan* vol. 5, printed beside — not inside — the point-opening material). The full method adds: the **day stem** as a first-class input, the **five-shu/original point** as output, **open vs closed** hour states, phase-generation sequencing across a 10-day cycle, and cross-day spillover. A function mapping only hour-branch → channel is not this method (research §1.1 of the prior note, LH-DIV-01).

## 2. Canonical-variant and divergence ledger

The school landscape must be recorded, not silently resolved — same policy as LH-DIV-02.

| ID | Divergence | Current decision |
|---|---|---|
| `TNLC-DIV-01` | The Xu-style 納甲法 leaves certain day/hour slots **closed (閉穴)**; later schools fill them (e.g. with 本穴/原穴 or 返本還原-type rules). | Freeze the **徐氏 tables as printed in *Zhenjiu Dacheng***; closed slots serialize an explicit closed state; filling schools are recorded as divergence, never merged in. |
| `TNLC-DIV-02` | Yang-stem days route surplus hours to the Triple Burner (氣納三焦) and yin-stem days to the Pericardium (血歸包絡); editions differ in which rows encode this. | Encode the *Zhenjiu Dacheng* rows; keep 三焦/心包 as traditional identities (LH-DIV-06); no biomedical conversion. |
| `TNLC-DIV-03` | Classical timekeeping defines no modern civil-zone, DST, or longitude behavior, and the day boundary (which day a 亥→子 transition belongs to) is an edition/presentation choice. | Reuse Amlich's existing day-pillar and `local_civil_hour_branch` conventions; disclose `time_basis`; pin cross-day spillover goldens to the frozen convention. |
| `TNLC-DIV-04` | Vietnamese point nomenclature (huyệt danh) varies across modern schools; WHO/WFAS standard alphanumeric codes exist as a lookup convention. | Gate 2 signs one Vietnamese nomenclature set + code set; codes are presented as standard lookup glosses, never as WHO endorsement of efficacy (per WHO ICD-11 TM FAQ). |
| `TNLC-DIV-05` | The method is contested in the classical corpus itself (e.g. *Zhenjiu Fengyuan* vol. 4 critiques one-channel-per-block readings). | Carry the v1.10 historical-contestation marker; a point-opening citation is never a physiological or efficacy claim. |

## 3. Safety and wording contract

Anchors unchanged from Tier 0 (WHO ICD-11 TM non-endorsement FAQ; WHO acupuncture training benchmarks; NCCIH serious-adverse-effect notice; FDA general-wellness boundary). **The NCCIH anchor is load-bearing here**: point names are the closest this product has come to procedural guidance, which is why owner decision (2) is conditioned on Gates 3–4.

**[DESIGN] Disclaimer v2 (draft for Gate 4, both languages must be reviewed):**

> Trích dẫn thuật ngữ y học cổ truyền từ văn bản Châm Cứu Đại Thành; chỉ mang tính văn hóa – lịch sử. Đây không phải hướng dẫn châm, bấm, cứu hay tự điều trị tại bất kỳ thời điểm nào. Không dùng để trì hoãn hoặc thay thế chăm sóc từ nhân viên y tế có chuyên môn.
>
> Citations of classical acupuncture terminology from Zhenjiu Dacheng; provided as historical and cultural information only. This is not instruction or encouragement to needle, press, moxibust, or self-treat at any time. Do not use it to delay or replace care from a qualified health professional.

**[DESIGN] Allowed framing:** "theo bài ca/bảng, giờ X ngày Giáp được ghi tương ứng với huyệt Y" — citation framing; review state, time basis, and divergences always visible.

**[DESIGN] Prohibited (extends the v1.10 guard):** "giờ tốt để châm/bấm/cứu", "best time to treat", needle depth, technique, manipulation (補瀉 as instruction), indication/contraindication, efficacy, " đang mở → hãy can thiệp", or any imperative derived from an open state. An open-point citation must never be phrased as an action recommendation.

## 4. Minimum golden and contract cases (finite)

1. **120 slot goldens:** every day-stem × hour-branch combination (10 × 12) resolves to the frozen row — open (channel, point class, point identity) or explicit closed — with zero computation from unpinned sources.
2. **Cross-day spillover goldens:** the 甲→乙 (and one yin-pair) 亥/子 boundaries resolve per the frozen convention.
3. **Civil-time boundaries:** reuse 22:59 / 23:00 / 00:59 / 01:00 hour-branch goldens.
4. **Provenance contract:** every row cites only `ty-ngo-luu-chu` for the method plus the existing engine source for pillar/time facts; v1.10's `shi-er-jing-na-di-zhi` is never emitted for point rows and vice versa.
5. **Nomenclature contract:** every point row carries the triple identity (Chinese 穴名, signed Vietnamese huyệt danh, standard alphanumeric code as gloss) with round-trip serialization.
6. **Closed-hour contract:** closed slots serialize a distinct unavailable-by-tradition state and are never converted to an adjacent point or a recommendation.
7. **Safety/schema guard:** extended prohibited-language guard; DTOs contain no technique/depth/indication/efficacy fields.
8. **Cross-surface parity:** core/API/TUI/desktop render byte-identical citation wording; `traditional_wellness` (v1.10) and the v1.11 field remain separate additive DTO fields.

## 5. Review gates before surfaced availability

1. **Gate 1 — classical-Chinese:** a qualified reviewer verifies the entire frozen table set (all 120 slots + spillover + closed rows) against a chosen facsimile of *Zhenjiu Dacheng* vol. 5 and signs every record.
2. **Gate 2 — Vietnamese point nomenclature:** a qualified reviewer signs the huyệt danh set and code glosses.
3. **Gate 3 — health-safety:** a qualified reviewer confirms no schema, wording, or surface behavior can be read as point-selection instruction or self-treatment encouragement, including the default-surface exposure decision.
4. **Gate 4 — product/legal:** approves disclaimer v2 and distribution jurisdictions (FDA general-wellness boundary is a model, not Vietnamese legal advice).

Until gates sign: corpus rows stay `ExternalReviewPending`, surfaced rows carry disclaimer v2, and (per ADR 0003) point/procedure outputs may be held unavailable rather than exposed unsigned.

## 6. Explicit non-goals

- 靈龜八法 (Linggui Bafa) and 飛騰八法 (Feiteng Bafa) eight-method open points.
- 養子時刻注穴法 (hour-stem variant) — record existence in divergence notes only.
- Needling/moxibustion/acupressure technique, depth, manipulation, contraindication, or first-aid guidance.
- Efficacy, prevention, treatment, diagnosis claims of any kind; Tier-2 Bazi personalization.
- Any coupling into Day Assessment, Hour Ranking, Direction Assessment, or v1.10 Traditional Wellness Context.

## 7. Track-1 verification ledger (what must be frozen from facsimile)

- [ ] Facsimile edition selection + stable page/chapter URIs for every cited table.
- [ ] Per-day verse/table transcription: 甲日 through 癸日, all hour rows.
- [ ] Closed-slot inventory and the exact wording that marks them.
- [ ] 氣納三焦 / 血歸包絡 rows and their placement.
- [ ] Cross-day spillover convention as printed.
- [ ] Point-identity strings exactly as printed (穴名), before any Vietnamese glossing.
- [ ] REVIEWER-PACK with per-row evidence links for Gates 1–4.

Links above were checked 2026-08-11 (in the prior note); re-verify at Track-1 start. Online transcriptions are discovery aids; a chosen facsimile plus signed human review remains the corpus acceptance gate.
