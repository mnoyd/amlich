# v1.10 Traditional Wellness Context — Seasonal Cultivation Reviewer Pack

**Scope:** Child bead `amlich-l2zc.2` — selected-date four-season cultivation context (Tứ khí điều thần / 四氣調神大論)
**Milestone:** v1.10 Traditional Wellness Context (Tier 0)
**Plan under review:** `.planning/milestones/v1.10-phases/02-seasonal-cultivation-context/02-01-PLAN.md`
**Research base:** `.planning/research/LUNAR_HEALTH_RESEARCH.md` §2 (Seasonal source boundary), §6 (LH-DIV-04/05/07)
**Decision in force:** `docs/adr/0003-separate-branch-channel-association-from-ty-ngo-luu-chu.md` (tradition-isolation consequences apply to this track too)

This pack closes the four human review gates listed in `.planning/milestones/v1.10-REQUIREMENTS.md:96-105` for the **seasonal** track. Gate 2 (Suwen paraphrases) is addressed **only** here; Gates 3 (product/legal) and 4 (health-safety) are addressed here **in their seasonal scope** and in the parallel pack for bead `amlich-l2zc.1` in their branch-channel scope. Gate 1 (classical-Chinese, 12-row table) is addressed by the `.1` pack.

---

## §A — Source-pinned four-profile seasonal corpus

**Primary source under review:** *Huangdi Neijing Suwen* (《黃帝內經 · 素問》), chapter `四氣調神大論篇第二` (*Great Discourse on Regulating the Spirit According to the Four Seasons*, Suwen chapter 2).
**Transcription / discovery URI:** https://ctext.org/huangdi-neijing/si-qi-diao-shen-da-lun/zh (Chinese Text Project)
**Facsimile record:** Sibu Congkan — https://ctext.org/library.pl?if=en&res=77415
**Translation-kind metadata:** `project_paraphrase` (per `LUNAR_HEALTH_RESEARCH.md:178`); never a copied modern edition.

**What the source supports:** one routine profile per three-month season — **four** profiles, not twenty-four solar-term regimens (`LUNAR_HEALTH_RESEARCH.md:80-93`). The chapter's organ-injury and later-illness consequences clauses are **deliberately omitted** from every paraphrase (LH-DIV-07); they remain only in the research audit notes.

### §A.0 Bilingual disclaimer (carried on every output of this corpus)

The exact strings the implementation emits (from `crates/amlich-core/src/traditional_wellness/disclaimer.rs` — byte-identical to the Phase 01 pack):

**§A.1 Vietnamese:**
> Thông tin văn hóa–lịch sử về quan niệm dưỡng sinh truyền thống; không phải tư vấn y khoa, chẩn đoán, phòng ngừa hay điều trị. Không dùng để trì hoãn hoặc thay thế chăm sóc từ nhân viên y tế có chuyên môn.

**§A.2 English:**
> Historical and cultural information about a traditional wellness system; not medical advice, diagnosis, prevention, or treatment. Do not use it to delay or replace care from a qualified health professional.

**§A.3 Stable identifier:** `cultural_information_v1` — clients are contractually required to render this string (or its localized variant) verbatim.

### §A.4 The four project paraphrases

Each paraphrase is a project paraphrase of the chapter's routine themes only, prefixed with the classical-text framing ("Văn bản cổ mô tả" / "the classical text describes"). Wording discipline: no organ-injury consequence, no disease/symptom claim, no food/herb/supplement/fasting instruction, no quantified sleep/exercise prescription, no local-weather statement.

| Season | Passage key | Vietnamese paraphrase (verbatim corpus `wording_vi`) | English paraphrase (verbatim corpus `wording_en`) |
|---|---|---|---|
| Xuân (春, spring) | `spring` | Trong chương "Tứ khí điều thần" của Hoàng Đế Nội Kinh – Tố Vấn, văn bản cổ mô tả việc dưỡng sinh mùa xuân: ngủ dậy sớm, đi dạo thong thả ngoài trời, buông lỏng thân thể, để ý định và hoạt động triển khai tự nhiên thay vì gò bó. | In the "Great Discourse on Regulating the Spirit According to the Four Seasons" of the Huangdi Neijing Suwen, the classical text describes the spring routine as rising early, taking unhurried outdoor walks, loosening the body, and letting intention and activity unfold rather than constraining them. |
| Hạ (夏, summer) | `summer` | Trong chương "Tứ khí điều thần" của Hoàng Đế Nội Kinh – Tố Vấn, văn bản cổ mô tả việc dưỡng sinh mùa hè: ngủ dậy sớm, tiếp xúc với ánh sáng ban ngày, tránh nóng giận, để hoạt động và sự chú ý hướng ra bên ngoài. | In the "Great Discourse on Regulating the Spirit According to the Four Seasons" of the Huangdi Neijing Suwen, the classical text describes the summer routine as rising early, engaging with daylight, avoiding anger, and allowing activity and attention to move outward. |
| Thu (秋, autumn) | `autumn` | Trong chương "Tứ khí điều thần" của Hoàng Đế Nội Kinh – Tố Vấn, văn bản cổ mô tả việc dưỡng sinh mùa thu: ngủ sớm và dậy sớm, lắng lại và thu giữ sự chú ý hướng vào trong, giữ tâm thế tĩnh tại. | In the "Great Discourse on Regulating the Spirit According to the Four Seasons" of the Huangdi Neijing Suwen, the classical text describes the autumn routine as sleeping and rising early, settling and gathering attention inward, and keeping a tranquil disposition. |
| Đông (冬, winter) | `winter` | Trong chương "Tứ khí điều thần" của Hoàng Đế Nội Kinh – Tố Vấn, văn bản cổ mô tả việc dưỡng sinh mùa đông: ngủ sớm, dậy muộn hơn đợi ánh ngày, giữ ý hướng vào trong, tìm chỗ ấm và tránh tiếp xúc không cần thiết với lạnh. | In the "Great Discourse on Regulating the Spirit According to the Four Seasons" of the Huangdi Neijing Suwen, the classical text describes the winter routine as sleeping early and rising later, waiting for daylight, keeping intention inward, and seeking warmth while avoiding unnecessary exposure to cold. |

### §A.5 The Amlich term-to-season composition (explicitly NOT a source claim)

The 24 solar terms are joined to the four profiles at the four seasonal-beginning terms (per `LUNAR_HEALTH_RESEARCH.md:101-110`, grounded in the China Meteorological Administration's canonical term sequence):

| Seasonal profile | Solar terms (six per season) |
|---|---|
| Spring | Lập Xuân, Vũ Thủy, Kinh Trập, Xuân Phân, Thanh Minh, Cốc Vũ |
| Summer | Lập Hạ, Tiểu Mãn, Mang Chủng, Hạ Chí, Tiểu Thử, Đại Thử |
| Autumn | Lập Thu, Xử Thử, Bạch Lộ, Thu Phân, Hàn Lộ, Sương Giáng |
| Winter | Lập Đông, Tiểu Tuyết, Đại Tuyết, Đông Chí, Tiểu Hàn, Đại Hàn |

This join is a deterministic **Amlich presentation composition** — the bilingual disclosure emitted with every result reads (verbatim from `seasonal.rs`):

- **vi:** Amlich ghép tiết khí hiện hành vào một trong bốn mùa theo bốn tiết mở đầu mùa: Lập Xuân, Lập Hạ, Lập Thu, Lập Đông (mỗi mùa sáu tiết). Văn bản cổ chỉ trình bày bốn đề cương theo mùa; phép ghép này là của Amlich, không phải hai mươi bốn chế độ riêng theo tiết, cũng không phải nhận định về thời tiết địa phương.
- **en:** Amlich joins the current solar term into one of four seasons at the four seasonal-beginning terms: Lập Xuân, Lập Hạ, Lập Thu, and Lập Đông (six terms per season). The classical text presents only four seasonal profiles; this join is an Amlich composition — not twenty-four term-specific regimens and not a statement about local weather.

The astronomical solar-term computation retains its existing source and provenance; it is never retagged as *Suwen* (LH-DIV-04). The composite provenance envelope carries the reserved identifier `rule.composite.seasonal_wellness` and nothing else.

### §A.6 Known divergence markers (per profile row)

- **LH-DIV-04** — *Suwen* supplies four seasonal profiles while the desired UI key is one of 24 solar terms. *Project decision:* treat term → season as a transparent Amlich composition with a mandatory divergence marker; never present the profile as a term-specific classical prescription.
- **LH-DIV-05** — Solar-term phenology formed around the Yellow River; weather seasons differ by geography. *Project decision:* no local-weather or exposure advice is emitted; the profiles describe the historical text only.
- **LH-DIV-07** — The classical seasonal chapter attaches organ-injury and later-illness consequences to acting contrary to each season. *Project decision:* those clauses are omitted from Tier-0 output entirely; they are retained only in the research audit notes.

### §A.7 Citation metadata payload (per row)

```json
"sources": [{
  "source_id": "huangdi-neijing-suwen",
  "work_title": "Huangdi Neijing Suwen",
  "volume_or_chapter": "素問 四氣調神大論篇第二",
  "passage_key": "spring",
  "edition_or_facsimile_uri": "<filled by the Suwen reviewer on sign-off>",
  "transcription_uri": "https://ctext.org/huangdi-neijing/si-qi-diao-shen-da-lun/zh",
  "translation_kind": "project_paraphrase"
}]
```

The `edition_or_facsimile_uri` is the artifact the corpus record will reference for every future citation dispute; the Sibu Congkan record above is the suggested candidate.

### §A.8 Displaced unsourced copy (what this corpus replaces)

The pre-v1.10 `data/tiet-khi.json` corpus carried an unsourced, per-term `health` list on each of the 24 terms (e.g. "Bổ gan, thanh nhiệt với các loại rau xanh" / "Nourish liver, clear heat with green vegetables"). That copy had no citation, no review state, and a clinical-sounding register. v1.10 Phase 02-01 **empties** those lists (schema retained for additive serialization compatibility) so that no surface — core, API, terminal, or desktop — can render unsourced seasonal health advice. The only seasonal wellness content shipped after this change is the four reviewed paraphrases in §A.4, each carrying the §A.0 disclaimer, review state, safety class, and §A.6 divergences.

---

## §B — Suwen paraphrase reviewer sign-off (Gate 2)

I, the undersigned, have reviewed the four paraphrases in §A.4 against `四氣調神大論篇第二` (using the ctext.org transcription and/or a facsimile edition of equivalent authority, e.g. the Sibu Congkan record above), and I confirm:

- [ ] Each paraphrase is faithful to the routine themes of its seasonal passage and asserts nothing the passage does not say.
- [ ] The paraphrases are project paraphrases, not translations copied from any modern copyrighted edition.
- [ ] The organ-injury and later-illness consequence clauses of the chapter are intentionally omitted (LH-DIV-07) and their omission does not distort the surviving routine themes.
- [ ] The seasonal boundaries in §A.5 are presented as an Amlich composition, never as a claim that *Suwen* prescribes per-solar-term regimens (LH-DIV-04).
- [ ] I have recorded the `edition_or_facsimile_uri` actually consulted (above) in the corpus JSON.

**Reviewer role:** suwen_paraphrase_reviewer
**Name / signature:**
**Date (YYYY-MM-DD):**
**Edition consulted (URI or bibliographic reference):**

The implementation will not mark these records `Signed` until this gate is signed. Until then, every result exposing this context emits the bilingual disclaimer and the corpus `reviewer` literal remains `ExternalReviewPending(...)`.

---

## §C — Health-safety reviewer sign-off (Gate 4, seasonal scope)

I, the undersigned, have reviewed the seasonal surface, and I confirm:

- [ ] The four paraphrases in §A.4 use "văn bản cổ mô tả" / "the classical text describes" framing exclusively — they never instruct, recommend, or prescribe, and never name a disease, symptom, food, herb, supplement, dose, or quantified sleep/exercise regimen.
- [ ] The composition disclosure in §A.5 cannot be read as local-weather or exposure advice (LH-DIV-05).
- [ ] The displaced `health` copy removal in §A.8 leaves no surface able to render unsourced seasonal health advice; the replacement content carries the §A.0 disclaimer and a safety classification on every record.
- [ ] The lexical-guard extension at `crates/amlich-core/tests/prohibited_language_guard.rs` now scans `seasonal-cultivation.json` with the same forbidden-lexeme and clinical-field lists signed in the Phase 01 pack §C.
- [ ] The schema field set on `SeasonalCultivationProfile` / `SeasonalCultivationContext` — season identities, paraphrase text, citation, reviewer state, safety class, divergence ids, composition disclosure, evidence envelopes — contains no field that can be interpreted as self-treatment guidance.

**Reviewer role:** health_safety_reviewer
**Name / signature:**
**Date (YYYY-MM-DD):**
**Additional concerns (optional):**

---

## §D — Product/Legal reviewer sign-off (Gate 3, seasonal scope)

I, the undersigned, have reviewed the bilingual disclaimer text in §A.1 and §A.2 and the seasonal copy in §A.4/§A.5 for the intended distribution jurisdictions, and I confirm:

- [ ] The Vietnamese and English paraphrases and the composition disclosure are appropriate for distribution to Vietnamese- and English-speaking users as historical/cultural information.
- [ ] The stable identifier `cultural_information_v1` remains the contract clients must honor when rendering.
- [ ] The content distinguishes "historical and cultural information" from medical advice, diagnosis, prevention, or treatment, and instructs the reader not to delay or replace qualified professional care.
- [ ] Removing the unsourced `health` lists (§A.8) is an acceptable product change for all distribution surfaces.
- [ ] No regulatory determination of Vietnamese law is implied by my signature; the FDA General Wellness wording model is referenced only as a vocabulary boundary, not as legal advice.

**Reviewer role:** product_legal_reviewer
**Name / signature:**
**Date (YYYY-MM-DD):**
**Jurisdictions reviewed:**

---

## §E — Boundaries enforced by the implementation (read-only context for the reviewers)

- **No birth / medical data required.** `resolve_seasonal_cultivation(jd, time_zone)` and `enrich_day_snapshot_with_seasonal_cultivation(snapshot, jd, time_zone)` succeed without any `BirthInput`, sex/gender, symptom, location, or health history field. Locked by `seasonal_cultivation_integration.rs::tier0_resolves_without_birth_or_medical_data`.
- **No 24-term regimen.** The corpus contains exactly 4 rows; the term→season mapping lives in code as a disclosed composition; the corpus structure test fails CI if a fifth row or a per-term paraphrase appears.
- **No local-weather claim.** The displaced `weather` field of `tiet-khi.json` remains an astronomical/calendar description and is not part of the wellness context; the wellness context emits no weather statement.
- **Provenance separation is test-locked.** Every resolved context carries exactly three evidence envelopes — solar-term primitive (Snapshot family, method `get_tiet_khi`), Suwen primitive (`huangdi-neijing-suwen`), and one Derived composite (`rule.composite.seasonal_wellness`). The solar-term envelope is never tagged `huangdi-neijing-suwen`; the Suwen envelope never claims the term computation.
- **No change to Day Assessment.** The seasonal module is additive; `calculate_day_snapshot` output is byte-identical before/after enrichment (locked by `enrichment_leaves_day_assessment_untouched`).
- **`ty-ngo-luu-chu` is never emitted.** The Phase 01 CI guard covers the whole `src/` tree including the new module.

---

## §F — Reviewer packet history (filled in by the bead owner, not the reviewer)

- **Packet version:** v1
- **Packet author:** implementation owner of `amlich-l2zc.2`
- **Packet date:** 2026-08-16
- **Linked bead:** `amlich-l2zc.2`
- **Linked plan:** `.planning/milestones/v1.10-phases/02-seasonal-cultivation-context/02-01-PLAN.md`
- **Linked research:** `.planning/research/LUNAR_HEALTH_RESEARCH.md` §2, §6
- **Linked external-review policy:** `docs/architecture/external-review-lifecycle.md`

The implementation owner will not flip any corpus record from `ExternalReviewPending` to `Signed` until §§B, C, and D are each signed by a named reviewer. The sign-off dates and reviewer identities will be recorded on the bead as comments at the moment of close.
