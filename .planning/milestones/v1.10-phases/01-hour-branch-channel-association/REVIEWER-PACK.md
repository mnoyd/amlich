# v1.10 Traditional Wellness Context — Reviewer Pack

**Scope:** Child bead `amlich-l2zc.1` — selected-hour Twelve-Branch Channel Association (Thập nhị kinh nạp địa chi / 十二經納地支)
**Milestone:** v1.10 Traditional Wellness Context (Tier 0)
**Plan under review:** `.planning/milestones/v1.10-phases/01-hour-branch-channel-association/01-01-PLAN.md`
**Research base:** `.planning/research/LUNAR_HEALTH_RESEARCH.md`
**Decision in force:** `docs/adr/0003-separate-branch-channel-association-from-ty-ngo-luu-chu.md`

This pack is the artifact that closes the four human review gates listed in `.planning/milestones/v1.10-REQUIREMENTS.md:96-105` for the branch-channel track. Gates 1 (classical-Chinese), 3 (product/legal), and 4 (health-safety) are addressed here; gate 2 (Suwen paraphrases) is addressed by the parallel pack for bead `amlich-l2zc.2`.

---

## §A — Source-pinned 12-row branch-channel table

**Primary source under review:** Xu Feng, *Zhenjiu Daquan* (《針灸大全》), volume 5, section `論子午流注之法` (lines 3–9) — the `十二經納地支歌` ("Song of the Twelve Channels Assigned to the Earthly Branches").
**Facsimile / discovery URI:** https://ctext.org/wiki.pl?chapter=688012&if=en (Chinese Text Project)
**Cross-reference:** Yang Jizhou, *Zhenjiu Dacheng* (《針灸大成》), volume 7 — https://zh.wikisource.org/zh-hant/%E9%87%9D%E7%81%B8%E5%A4%A7%E6%88%90/%E5%8D%B7%E4%B8%83 (Lung-at-Dần circulation statement).
**Translation-kind metadata:** `project_paraphrase` (per `LUNAR_HEALTH_RESEARCH.md:178`); never a copied modern edition.

**Modern civil-time convention:** Amlich's existing local civil hour-branch contract. **Not** classical timekeeping. **Disclosed** as `time_basis = "local_civil_hour_branch"` (per `LUNAR_HEALTH_RESEARCH.md:66`).

### §A.0 Bilingual disclaimer (carried on every output of this table)

The exact strings the implementation emits (from `crates/amlich-core/src/traditional_wellness/disclaimer.rs`):

**§A.1 Vietnamese:**
> Thông tin văn hóa–lịch sử về quan niệm dưỡng sinh truyền thống; không phải tư vấn y khoa, chẩn đoán, phòng ngừa hay điều trị. Không dùng để trì hoãn hoặc thay thế chăm sóc từ nhân viên y tế có chuyên môn.

**§A.2 English:**
> Historical and cultural information about a traditional wellness system; not medical advice, diagnosis, prevention, or treatment. Do not use it to delay or replace care from a qualified health professional.

**§A.3 Stable identifier:** `cultural_information_v1` — clients are contractually required to render this string (or its localized variant) verbatim. The identifier is referenced from the `docs/architecture/external-review-lifecycle.md` Active Register.

### §A.4 The 12 rows

For every row, the project paraphrase is **neutral historical association** wording. The Chinese channel name is preserved verbatim. `心包` and `三焦` are **not** converted into modern anatomical equivalents (per `LUNAR_HEALTH_RESEARCH.md:186`).

| # | Earthly Branch | Civil window | Channel (vi) | Channel (en) | Channel (zh) | Vietnamese wording | English wording | `known_divergence_ids` |
|---:|---|---|---|---|---|---|---|---|
| 0 | Tý / 子 | 23:00–01:00 | Đởm | Gallbladder | 足少陽膽 | Theo bảng nạp địa chi trong *Zhenjiu Daquan*, kinh Đởm được gắn với giờ Tý trong truyền thống. | According to the Earthly Branch allocation table in *Zhenjiu Daquan*, the Gallbladder channel is historically associated with the Tý hour. | LH-DIV-02, LH-DIV-03, LH-DIV-06 |
| 1 | Sửu / 丑 | 01:00–03:00 | Can | Liver | 足厥陰肝 | Theo bảng nạp địa chi trong *Zhenjiu Daquan*, kinh Can được gắn với giờ Sửu trong truyền thống. | According to the Earthly Branch allocation table in *Zhenjiu Daquan*, the Liver channel is historically associated with the Sửu hour. | LH-DIV-02, LH-DIV-03, LH-DIV-06 |
| 2 | Dần / 寅 | 03:00–05:00 | Phế | Lung | 手太陰肺 | Theo bảng nạp địa chi trong *Zhenjiu Daquan*, kinh Phế được gắn với giờ Dần trong truyền thống. | According to the Earthly Branch allocation table in *Zhenjiu Daquan*, the Lung channel is historically associated with the Dần hour. | LH-DIV-02, LH-DIV-03, LH-DIV-06 |
| 3 | Mão / 卯 | 05:00–07:00 | Đại trường | Large Intestine | 手陽明大腸 | Theo bảng nạp địa chi trong *Zhenjiu Daquan*, kinh Đại trường được gắn với giờ Mão trong truyền thống. | According to the Earthly Branch allocation table in *Zhenjiu Daquan*, the Large Intestine channel is historically associated with the Mão hour. | LH-DIV-02, LH-DIV-03, LH-DIV-06 |
| 4 | Thìn / 辰 | 07:00–09:00 | Vị | Stomach | 足陽明胃 | Theo bảng nạp địa chi trong *Zhenjiu Daquan*, kinh Vị được gắn với giờ Thìn trong truyền thống. | According to the Earthly Branch allocation table in *Zhenjiu Daquan*, the Stomach channel is historically associated with the Thìn hour. | LH-DIV-02, LH-DIV-03, LH-DIV-06 |
| 5 | Tỵ / 巳 | 09:00–11:00 | Tỳ | Spleen | 足太陰脾 | Theo bảng nạp địa chi trong *Zhenjiu Daquan*, kinh Tỳ được gắn với giờ Tỵ trong truyền thống. | According to the Earthly Branch allocation table in *Zhenjiu Daquan*, the Spleen channel is historically associated with the Tỵ hour. | LH-DIV-02, LH-DIV-03, LH-DIV-06 |
| 6 | Ngọ / 午 | 11:00–13:00 | Tâm | Heart | 手少陰心 | Theo bảng nạp địa chi trong *Zhenjiu Daquan*, kinh Tâm được gắn với giờ Ngọ trong truyền thống. | According to the Earthly Branch allocation table in *Zhenjiu Daquan*, the Heart channel is historically associated with the Ngọ hour. | LH-DIV-02, LH-DIV-03, LH-DIV-06 |
| 7 | Mùi / 未 | 13:00–15:00 | Tiểu trường | Small Intestine | 手太陽小腸 | Theo bảng nạp địa chi trong *Zhenjiu Daquan*, kinh Tiểu trường được gắn với giờ Mùi trong truyền thống. | According to the Earthly Branch allocation table in *Zhenjiu Daquan*, the Small Intestine channel is historically associated with the Mùi hour. | LH-DIV-02, LH-DIV-03, LH-DIV-06 |
| 8 | Thân / 申 | 15:00–17:00 | Bàng quang | Bladder | 足太陽膀胱 | Theo bảng nạp địa chi trong *Zhenjiu Daquan*, kinh Bàng quang được gắn với giờ Thân trong truyền thống. | According to the Earthly Branch allocation table in *Zhenjiu Daquan*, the Bladder channel is historically associated with the Thân hour. | LH-DIV-02, LH-DIV-03, LH-DIV-06 |
| 9 | Dậu / 酉 | 17:00–19:00 | Thận | Kidney | 足少陰腎 | Theo bảng nạp địa chi trong *Zhenjiu Daquan*, kinh Thận được gắn với giờ Dậu trong truyền thống. | According to the Earthly Branch allocation table in *Zhenjiu Daquan*, the Kidney channel is historically associated with the Dậu hour. | LH-DIV-02, LH-DIV-03, LH-DIV-06 |
| 10 | Tuất / 戌 | 19:00–21:00 | Tâm bào | Pericardium | 手厥陰心包 | Theo bảng nạp địa chi trong *Zhenjiu Daquan*, kinh Tâm bào được gắn với giờ Tuất trong truyền thống. | According to the Earthly Branch allocation table in *Zhenjiu Daquan*, the Pericardium channel is historically associated with the Tuất hour. | LH-DIV-02, LH-DIV-03, LH-DIV-06 |
| 11 | Hợi / 亥 | 21:00–23:00 | Tam tiêu | Triple Burner | 手少陽三焦 | Theo bảng nạp địa chi trong *Zhenjiu Daquan*, kinh Tam tiêu được gắn với giờ Hợi trong truyền thống. | According to the Earthly Branch allocation table in *Zhenjiu Daquan*, the Triple Burner channel is historically associated with the Hợi hour. | LH-DIV-02, LH-DIV-03, LH-DIV-06 |

**Wording discipline (per `LUNAR_HEALTH_RESEARCH.md:134-141`):** every row uses "gắn với" / "associated with" — never "hoạt động," "at peak," "active," "detox," "best time to treat," or any diagnostic/therapeutic verb.

### §A.5 Known divergence markers (per row)

The following three divergence IDs are attached to **every** row in the corpus. They are the historical-context disclosures — they are not errors to be resolved.

- **LH-DIV-02** — Later classical authors preserve but explicitly criticize the fixed one-channel-per-double-hour allocation. *Source:* Li Xuechuan, *Zhenjiu Fengyuan* (《針灸逢源》), volume 4, `經穴考正` — https://ctext.org/wiki.pl?chapter=414318&if=en. *Project decision:* surface as a historical association with a mandatory divergence marker; do not assert physiology.
- **LH-DIV-03** — Classical timekeeping does not define modern civil-zone, DST, or longitude-correction behavior. *Project decision:* reuse Amlich local civil hour branches and disclose `time_basis`; do not claim classical exactness.
- **LH-DIV-06** — Classical `臟腑` labels do not map one-to-one to modern anatomy/physiology; `心包` and `三焦` are especially unsafe to biomedicalize. *Project decision:* preserve traditional names and use "channel" rather than "organ function."

A fourth divergence (LH-DIV-01 — "Full Tý Ngọ Lưu Chú vs simplified cycle") is captured in the ADR and is the reason the milestone scope uses the `十二經納地支` title, not `子午流注`.

### §A.6 Citation metadata payload (per row)

The corpus JSON carries the following structured citation field on each row, in addition to the human-readable wording above:

```json
"sources": [{
  "source_id": "shi-er-jing-na-di-zhi",
  "work_title": "Zhenjiu Daquan",
  "volume_or_chapter": "卷之五 論子午流注之法",
  "passage_key": "十二經納地支歌",
  "edition_or_facsimile_uri": "<see candidate list below; reviewer confirms or supplies alternative>",
  "transcription_uri": "https://ctext.org/wiki.pl?chapter=688012&if=en",
  "translation_kind": "project_paraphrase"
}]
```

The reviewer filling in the `edition_or_facsimile_uri` is the single most important field on this page — it is the artifact the corpus record will reference for every future citation dispute.

#### §A.6.a Candidate editions (reviewer confirms one, or supplies an equivalent)

The reviewer is asked to **consult one of the following candidate editions** of *Zhenjiu Daquan* (《針灸大全》) volume 5, `論子午流注之法` lines 3–9 (`十二經納地支歌`), and to record **which one** they actually consulted in the `edition_or_facsimile_uri` field. Any equivalent facsimile edition of comparable textual authority is acceptable; the reviewer should justify the choice in a one-line note on §B.

| # | Edition / facsimile | URI / locator | Notes |
|---:|---|---|---|
| 1 | Chinese Text Project transcription (text + linked page-image plates) | https://ctext.org/wiki.pl?chapter=688012&if=en | Default candidate; clean UTF-8 transcription with linked page images. Suitable for paraphrase-level review. |
| 2 | Sibu Congkan 四部叢刊 reprint (Shanghai Commercial Press 商務印書館, 1936) — *Zhenjiu Daquan* juan 5 | ctext.org library record: https://ctext.org/library.pl?if=en&res=2772 (record); facsimile scan via 中国哲学书电子化计划/国立图书馆 合作项目 | Photographic facsimile of the Ming woodblock; classical verse legible without modern normalization. |
| 3 | Ming dynasty Xu Feng original woodblock (reprinted in 1958 人民卫生出版社 校释本, pp. 75–76) | Bibliographic: 徐鳳《針灸大全》, 人民卫生出版社, 1958 (reprinted 1987) | Standard modern Chinese reprint with punctuation; widely cited in TCM pedagogy. |
| 4 | *Zhenjiu Dacheng* (《針灸大成》) volume 7 cross-reference — Lung-at-Dần circulation statement | https://zh.wikisource.org/zh-hant/%E9%87%9D%E7%81%B8%E5%A4%A7%E6%88%90/%E5%8D%B7%E4%B8%83 | Cross-reference edition to confirm the fixed Lung-at-Dần pairing independently of *Zhenjiu Daquan*. |

If the reviewer consults more than one edition and finds a substantive variant in any of the 12 pairings, they MUST mark that row `corrected` in §B and propose the canonical reading; this triggers the regression-fixture procedure in the external-review lifecycle policy.

### §A.7 Time-basis disclosure

`time_basis` on every returned row is the string `"local_civil_hour_branch"`. This label is the disclosure that the modern two-hour windows are an Amlich convention, not a classical claim (per `LUNAR_HEALTH_RESEARCH.md:66`). The four boundary cases (22:59 → Hợi, 23:00 → Tý, 00:59 → Tý, 01:00 → Sửu) reuse Amlich's existing hour-pillar contract verbatim.

---

## §B — Classical-Chinese reviewer sign-off (Gate 1)

I, the undersigned, have reviewed the 12-row table in §A.4 against the chosen facsimile of *Zhenjiu Daquan* (volume 5, `論子午流注之法` lines 3–9, and/or an alternative facsimile edition of equivalent authority), and I confirm:

- [ ] The 12 Earthly Branch → channel pairings are faithful to the classical verse and its table.
- [ ] The Chinese channel names (足少陽膽 through 手少陽三焦) match the source exactly and are not modernized.
- [ ] The Vietnamese and English wording is paraphrase, not translation; no modern copyrighted edition has been copied.
- [ ] The discrepancy between this fixed allocation and the *Zhenjiu Dacheng* / *Zhenjiu Fengyuan* criticisms is acknowledged in §A.5 and is the correct surface-level disclosure for Tier 0.
- [ ] I have recorded the `edition_or_facsimile_uri` actually consulted (above) in the corpus JSON.

**Reviewer role:** classical_chinese_reviewer
**Name / signature:**
**Date (YYYY-MM-DD):**
**Edition consulted (URI or bibliographic reference):**

The implementation will not mark these records `Signed` until this gate is signed. Until then, every snapshot exposing this context emits the bilingual disclaimer and the corpus `reviewer` literal remains `ExternalReviewPending(...)`.

---

## §C — Health-safety reviewer sign-off (Gate 4)

I, the undersigned, have reviewed:

- [ ] The bilingual disclaimer text in §A.1 and §A.2 — confirm it cannot be read as a substitute for medical advice, diagnosis, prevention, or treatment; confirm it instructs the reader not to delay or replace qualified professional care.
- [ ] The 12 row wordings in §A.4 — confirm each uses neutral historical-association language ("gắn với" / "associated with"), never active/peak/detox/best-treatment-time language, and never names an acupuncture point, needle depth, dose, or procedure.
- [ ] The lexical-guard scope at `crates/amlich-core/tests/prohibited_language_guard.rs` — confirm the forbidden lexemes (Vietnamese: `"hoạt động mạnh nhất"`, `"thải độc"`, `"đạt đỉnh"`; English: `"best time to treat"`, `"active organ"`, `"peak"` as channel adjective, `"detox"`, `"prevents"`, `"treats"`, `"cures"`, `"diagnoses"`, `"reduces risk"`, `"balances hormones"`) and the clinical field names (`indication`, `contraindication`, `diagnosis`, `treatment`, `dose`, `needle_depth`, `point_to_press`, `efficacy`) are exhaustive for the Tier-0 surface.
- [ ] The schema field set on `BranchChannelAssociation` — confirm no field can be interpreted as point selection or self-treatment guidance. (See plan §interfaces for the field list.)
- [ ] The disclosure that `心包` (Pericardium) and `三焦` (Triple Burner) are preserved as traditional names and not converted into modern anatomical equivalents (per `LUNAR_HEALTH_RESEARCH.md:186` and LH-DIV-06).

**Reviewer role:** health_safety_reviewer
**Name / signature:**
**Date (YYYY-MM-DD):**
**Additional concerns (optional):**

---

## §D — Product/Legal reviewer sign-off (Gate 3)

I, the undersigned, have reviewed the bilingual disclaimer text in §A.1 and §A.2 for the intended distribution jurisdictions, and I confirm:

- [ ] The Vietnamese text is appropriate for distribution to Vietnamese-speaking users.
- [ ] The English text is appropriate for distribution to English-speaking users.
- [ ] The stable identifier `cultural_information_v1` is the contract clients must honor when rendering.
- [ ] The disclaimer distinguishes "historical and cultural information" from medical advice, diagnosis, prevention, or treatment, and instructs the reader not to delay or replace qualified professional care.
- [ ] No regulatory determination of Vietnamese law is implied by my signature; the FDA General Wellness wording model is referenced only as a vocabulary boundary, not as legal advice (per `LUNAR_HEALTH_RESEARCH.md:122, 231`).

**Reviewer role:** product_legal_reviewer
**Name / signature:**
**Date (YYYY-MM-DD):**
**Jurisdictions reviewed:**

---

## §E — Boundaries enforced by the implementation (read-only context for the reviewers)

These are *not* items the reviewer signs — they are guarantees the implementation makes so the reviewer can sign the gates above with confidence.

- **No birth / medical data required.** `enrich_day_snapshot_with_branch_channel_association(snapshot, local_hour, local_minute)` succeeds without any `BirthInput`, sex/gender, symptom, location, or health history field. BOUND-01 contract is locked by `branch_channel_integration.rs::tier0_succeeds_without_birth_or_medical_data`.
- **No claim of physiological activity.** The wording in §A.4 uses "gắn với" / "associated with" exclusively; the lexical guard fails CI on any active/peak/detox phrase.
- **No acupuncture point, procedure, or prescription field.** `BranchChannelAssociation` carries only historical-association and disclosure fields. The clinical-field lexeme guard fails CI on `point_to_press`, `needle_depth`, `dose`, `indication`, `contraindication`, `diagnosis`, `treatment`, `efficacy` anywhere under the `traditional_wellness/` module.
- **`ty-ngo-luu-chu` is never emitted.** A separate CI guard (`crates/amlich-core/tests/source_id_guard.rs::ty_ngo_luu_chu_substring_never_appears_in_production_source`) fails the build if the string `"ty-ngo-luu-chu"` appears anywhere under `crates/amlich-core/src/`. This enforces the ADR-0003 scope split.
- **No change to Day Assessment, Hour Ranking, or Direction Assessment.** The `traditional_wellness` module is a sibling of `reasoning/`, not nested under `almanac/`. It does not contribute to assessment axes.
- **Civil-time boundaries are inherited, not redefined.** The Tý = 23:00–01:00 split is whatever `resolve_hour_branch_slot` returns; the boundary tests at `(22, 59)`, `(23, 0)`, `(0, 59)`, `(1, 0)` lock that contract against future drift.

---

## §F — Reviewer packet history (filled in by the bead owner, not the reviewer)

- **Packet version:** v1
- **Packet author:** implementation owner of `amlich-l2zc.1`
- **Packet date:** 2026-08-11
- **Linked bead:** `amlich-l2zc.1`
- **Linked plan:** `.planning/milestones/v1.10-phases/01-hour-branch-channel-association/01-01-PLAN.md`
- **Linked research:** `.planning/research/LUNAR_HEALTH_RESEARCH.md`
- **Linked ADR:** `docs/adr/0003-separate-branch-channel-association-from-ty-ngo-luu-chu.md`
- **Linked external-review policy:** `docs/architecture/external-review-lifecycle.md`
- **Resulting Active Register rows:** `docs/architecture/external-review-lifecycle.md:72-79` (entries added by plan T8).

The implementation owner will not flip any corpus record from `ExternalReviewPending` to `Signed` until §§B, C, and D are each signed by a named reviewer. The sign-off dates and reviewer identities will be recorded on the bead as comments at the moment of close.

---

## §G — Reviewer outreach (for the bead owner / coordinator)

This section is **not** reviewed; it is a how-to-send-this-pack checklist for the human coordinator (the bead owner) so the review engagement can move forward without re-deriving the protocol.

### §G.1 What the coordinator sends the reviewer

A single email / message containing:

1. The full text of this `REVIEWER-PACK.md` (the markdown is the canonical send).
2. A one-line pointer: *"Please review §A.4 against the chosen facsimile listed in §A.6.a, sign §B, and return the pack with your name, date, and `edition_or_facsimile_uri` recorded."*
3. The expected return format (see §G.3 below).
4. The deadline and the reviewer honorarium / compensation arrangement (project-specific; not codified here).

For Gates 3 (product/legal) and 4 (health-safety), the same packet is sent but the reviewer is pointed at §C or §D specifically; §B is informational context, not a gate they sign.

### §G.2 Subject line (suggested)

> `Amlich v1.10 Traditional Wellness Context — external review request (Gate {1|2|3|4}, role: {classical_chinese_reviewer|suwen_paraphrase_reviewer|product_legal_reviewer|health_safety_reviewer})`

### §G.3 Expected return format

The reviewer edits this `REVIEWER-PACK.md` in place: ticks the relevant boxes on §B / §C / §D, fills in `Name / signature`, `Date (YYYY-MM-DD)`, `Edition consulted`, and (for §B) `Jurisdictions reviewed` (legal only). The edited pack is returned **either**:

- as a GitHub PR against `master` adding the signed pack under `docs/reviews/v1.10/` (preferred — keeps the signing artifact in the repo and links the bead via PR reference); **or**
- as an email attachment (PDF or signed markdown), which the coordinator then commits to `docs/reviews/v1.10/` and links from the bead.

A signed pack in `docs/reviews/v1.10/<role>-<gate>-<YYYY-MM-DD>.md` is the authoritative artifact; the implementation owner updates the bead's notes with the PR / file reference, replaces `PENDING_CLASSICAL_REVIEW` / `PENDING_SUWEN_PARAPHRASE_REVIEW` placeholders in the corpus JSON with the recorded `edition_or_facsimile_uri`, and adds a comment on the bead recording the sign-off identity, date, and source.

### §G.4 Coordinator pre-flight checklist (before sending)

- [ ] The pack version (see §F) is current and matches the corpus `schema_version` in `crates/amlich-core/data/traditional-wellness/branch-channel.json`.
- [ ] The bilingual disclaimer in §A.1/§A.2 still byte-matches `crates/amlich-core/src/traditional_wellness/disclaimer.rs` (the prohibited-language guard `bilingual_disclaimer_is_byte_identical_to_reviewer_pack` will catch drift; rerun `cargo test -p amlich-core --test prohibited_language_guard` to verify).
- [ ] The candidate edition list in §A.6.a has at least one URI the reviewer can actually consult (no orphan / broken links).
- [ ] The reviewer role matches one of: `classical_chinese_reviewer`, `suwen_paraphrase_reviewer`, `product_legal_reviewer`, `health_safety_reviewer` (these are the four role identifiers in `docs/architecture/external-review-lifecycle.md` and the corpus JSON `assigned_to` fields).
- [ ] A bead exists for this engagement (one of `amlich-l2zc.5`, `.6`, `.7`, `.8`) so the sign-off can be filed against a trackable item.

### §G.5 What happens after sign-off

The implementation owner:

1. Updates the corpus JSON: replaces `PENDING_CLASSICAL_REVIEW` (and analogously `PENDING_SUWEN_PARAPHRASE_REVIEW`) with the recorded URI on every row, and replaces the `ExternalReviewPending(...)` reviewer literal with `Signed(reviewer=<identity>, date=<YYYY-MM-DD>, source_uri=<URI>)`.
2. Adds a comment on the bead: `Signed by <identity> on <YYYY-MM-DD> against <URI>; see PR <#>`.
3. Updates the Active Register row in `docs/architecture/external-review-lifecycle.md` to set the new `Review date` and remove the row's "open" status.
4. Re-runs `cargo test -p amlich-core --test prohibited_language_guard` and `cargo test -p amlich-core --test branch_channel_integration` to confirm the corpus change did not regress byte-equal contracts.
5. Closes the engagement bead.

The originating bead (`amlich-l2zc.1` or `.2`) closes only when **all** of its required gates have been signed (`.1` requires §B + §C + §D; `.2` requires §B + §C + §D). Once both are closed, `amlich-l2zc.3` (unified explanation) can be closed and `amlich-l2zc.4` (audit/release) is unblocked.