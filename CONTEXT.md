# Amlich Domain Context

This context captures the project language for Vietnamese lunar-calendar assessment, advisory, and timing decisions.

## Language

**Traditional Wellness Context**:
A source-attributed cultural account of seasonal or time-of-day wellness traditions. It is informational and never diagnoses, treats, predicts illness, or changes a Day Assessment or Hour Ranking.
_Avoid_: health assessment, medical recommendation, treatment advice

**Twelve-Branch Channel Association**:
The historical fixed association between the twelve two-hour Earthly Branch windows and the twelve regular channels. It is a cultural table, not a claim about channel flow, acupuncture-point opening, or organ performance.
_Avoid_: organ clock, Tý Ngọ Lưu Chú, channel rhythm, peak organ function

**Traditional Channel**:
A named kinh mạch identity preserved from the source tradition without conversion into a modern anatomical organ or physiological claim.
_Avoid_: active organ, biomedical meridian, organ function

**Tý Ngọ Lưu Chú**:
The classical day-and-hour method that combines stems, branches, channel assignments, and five-shu/original points to determine point-opening sequences. It is distinct from the fixed Twelve-Branch Channel Association and remains outside the initial Traditional Wellness Context.
_Avoid_: twelve-hour meridian table, simple organ clock

**Seasonal Cultivation Context**:
A source-attributed description of traditional dưỡng sinh themes associated with the active solar term or season. It reports the tradition without converting it into clinical prevention, supplement, diet, or treatment instructions.
_Avoid_: seasonal prescription, disease prevention advice, health verdict

**Day Assessment**:
A suitability verdict for a solar date in the context of an intent and, when available, a personal birth profile. It decides whether the day should be used, approached cautiously, or avoided.
_Avoid_: day ranking, lucky score

**Hour Ranking**:
An ordering of the twelve traditional hour slots within an already-assessed day. It does not change the day assessment or decide whether the day itself is usable.
_Avoid_: hour verdict, hour assessment

**Direction Assessment**:
An intent-aware ordering of the eight compass directions for a selected activity. It is independent of Day Assessment and Hour Ranking, and exposes travel-deity, Kua compatibility, directional-constraint, and available flying-star-overlay axes with explicit unavailable and warning states.
_Avoid_: direction lucky score, day verdict modifier

**Directional Constraint**:
A source-attributed Thái Tuế, Tam Sát, or Sát Phương fact applied to one compass direction. Overlapping facts remain separate contributions within the same direction; each stable fact fires at most once.
_Avoid_: generic bad direction, duplicated taboo penalty

**Location-sensitive Flying-star Overlay**:
A Phi Tinh directional overlay considered only for intents that use directional optimization and only when location context is available. Missing location is unavailable, not neutral or adverse.
_Avoid_: global flying-star direction score

**Hour Ranking Vector**:
The semantic dimensions used to order hour slots: Hoàng Đạo quality, intent timing fit, personal hour alignment, and day-hour harmony.
_Avoid_: hour embedding, opaque vector

**Hoàng Đạo Quality**:
The hour-ranking dimension that represents whether a traditional hour slot is auspicious according to the day’s Hoàng Đạo/Hắc Đạo classification.
_Avoid_: lucky hour flag

**Intent Timing Fit**:
The hour-ranking dimension that represents how well an hour slot supports the requested activity, such as travel, contract signing, or opening. It is available only when source-backed hour-specific rules exist for the intent.
_Avoid_: activity bonus

**Personal Hour Alignment**:
The hour-ranking dimension that represents compatibility between the hour slot and the person’s birth-derived facts when those facts are available.
_Avoid_: user bonus

**Day-Hour Harmony**:
The hour-ranking dimension that represents compatibility between the chosen day and the hour slot itself.
_Avoid_: time bonus

**Assessment Factor**:
A source-attributed input considered while forming a Day Assessment, with an explicit role and availability state.
_Avoid_: lucky signal, raw score

**Context Fact**:
An Assessment Factor that establishes calendar or personal context without directly moving an assessment axis or overriding its verdict.
_Avoid_: neutral feature

**Scored Feature**:
An Assessment Factor whose normalized value directly contributes to a declared assessment axis.
_Avoid_: bonus, penalty

**Veto**:
A named, source-attributed constraint that takes precedence over weighted suitability for its declared scope.
_Avoid_: large negative weight, implicit hard rule

**Explanation-only Factor**:
An Assessment Factor retained to explain relevant almanac context but excluded from both weighted aggregation and veto precedence.
_Avoid_: ignored factor, unused data

**Bazi-to-day Observation**:
A typed, source-attributed projection of the target day into the user's birth chart. The target-day Ten God relation to the natal day master, the target-day branch's relation to natal pillars (lục xung / lục hợp / tam hợp), and the target-day element's resonance with the natal day master are projected as `scored_feature` Assessment Factors. Each relation kind is deduplicated, and a date-only chart (no hour pillar) still produces year/month/day observations. The observations feed the PersonalAlignment axis only and never affect the Bazi chart scoring surface.
_Avoid_: day ranking, lucky score, Bazi chart score

**Bazi Target-Day Ten God**:
The Thập Thần relation from the target-day stem to the birth day master. Resource / support labels (Tỷ Kiến, Kiếp Tài, Chính Ấn, Thiên Ấn) are Favorable; draining / opposition labels (Thực Thần, Thương Quan, Chính Tài, Thiên Tài, Chính Quan, Thất Sát) are Avoid.
_Avoid_: stem match, hidden stem only

**Bazi Target-Day Pillar Relation**:
The target-day branch's classical xung / hợp relation to one or more natal pillars. Each relation kind (clash, lục hợp, tam hợp) fires at most once per assessment, so a target day that clashes with both the year and month pillars emits a single Avoid contribution rather than two. The day pillar is excluded from the relation check to avoid double-counting the existing `personal_alignment` axis.
_Avoid_: pillar duel, xung hợp double-count

**Bazi Target-Day Element Resonance**:
The target-day element's Ngũ Hành sinh / khắc relation to the birth day-master element. Generation (sinh) is Favorable; control (khắc) is Avoid; same element is Neutral. Distinct from the v2.2 `BaziElementResonance` interaction, which only fires for weak day-master synergy; this is the explicit feature observation that enters the axis aggregation directly.
_Avoid_: element score, weak day-master only

**Hour Ranking Policy**:
The versioned policy that owns the canonical ordering of the twelve traditional hour slots within an already-assessed day. v1 (`amlich-rv13`) keeps the birth-year-chi semantics; v2.4 (`amlich-bz0f.4`) layers three typed, source-attributed full-profile observations on top so a full birth profile produces a richer Personal Hour Alignment axis. The legacy PersonalHourMatrix integer-score surface is preserved as a compatibility projection, not retired.
_Avoid_: hour verdict, hour assessment, lucky hour score

**Hour Pillar Ten God**:
The Thập Thần relation from the hour-pillar stem to the birth day master, projected by the v2.4 hour-ranking policy. Resource / support labels (Tỷ Kiến, Kiếp Tài, Chính Ấn, Thiên Ấn) are Favorable; draining / opposition labels are Avoid. Fires only when a full birth profile (date + time) lets the Bazi chart expose the hour pillar.
_Avoid_: stem match, hidden stem only

**Hour Chi Birth Hour Branch Relation**:
The classical xung / hợp / hình relation between the hour chi and the birth hour chi, projected by the v2.4 hour-ranking policy. Each relation kind (lục xung, tương hại, tương hình, lục hợp, tam hợp pair) fires at most once per hour slot, so a clash cannot double-count against the Personal Hour Alignment axis.
_Avoid_: pillar duel, xung hợp double-count

**Hour Stem Element Support**:
Whether the hour-pillar stem's element equals the birth chart's weakest element, projected by the v2.4 hour-ranking policy. Favorable when the hour stem supports the weak element; Neutral otherwise. Fires only when a full birth profile produces a Bazi chart with the hour pillar available.
_Avoid_: element score, weak day-master only

**Non-Bazi Annual Pressure**:
A typed, source-attributed projection of a user's active annual affliction systems into the `AnnualPressure` axis. Each system (Tam Tai, Kim Lâu, Hoàng Ốc, Thái Tuế, Cửu Diệu / sao hạn) fires its own `Avoid` scored feature observation with source provenance from its classical source (KHCBPPT, Ngọc Hạp Ký, vn-folk, cuu-dieu). Informational systems surface as weighted `Avoid` contributions rather than universal vetoes; the hard `AnnualPressure` veto still fires only when the combined `HanSeverity` reaches High or Critical. Replaces the v2.3 single-observation aggregation; parity holds because the v2 aggregation formula averages same-polarity observations.
_Avoid_: Hạn aggregate score, opaque lucky pressure

**Tam Tai Observation**:
The Tam Tai (Three Calamities) three-year affliction period, projected by the v2.4 non-Bazi policy. Year position 2 (Cư — residing) carries the heaviest strength; years 1 (Nhập — entering) and 3 (Xuất — exiting) carry lighter strengths. Each active year emits one `Avoid` contribution sourced from KHCBPPT.
_Avoid_: Tam Tai veto, lucky year flag

**Kim Lâu Observation**:
The Kim Lâu (Golden Tower) age taboo, projected by the v2.4 non-Bazi policy. The category determines whom the taboo harms: Thân (self, heaviest), Thê (spouse), Tử (children), Súc (livestock, lightest). Each active category emits one `Avoid` contribution sourced from Ngọc Hạp Ký.
_Avoid_: Kim Lâu veto, opaque remainder score

**Hoàng Ốc Observation**:
The Hoàng Ốc (Desolate House) six-position construction taboo, projected by the v2.4 non-Bazi policy. Surfaces as an informational `Avoid` contribution (lighter strength) sourced from Vietnamese folk tradition; it is never a universal veto on its own.
_Avoid_: Hoàng Ốc veto, hard house taboo

**Thái Tuế Observation**:
The Thái Tuế (Grand Duke) annual conflict between the birth-year chi and the current-year chi, projected by the v2.4 non-Bazi policy. Five conflict kinds (Trực, Xung, Hại, Hình, Phá) are observed as a single `Avoid` contribution regardless of how many kinds fire, sourced from KHCBPPT.
_Avoid_: pillar duel, Thái Tuế double-count

**Cửu Diệu Observation**:
The Cửu Diệu (Nine Star) personal fortune star, projected by the v2.4 non-Bazi policy. Only the three Hung stars (La Hầu, Kế Đô, Thái Bạch — collectively "sao hạn") emit an `Avoid` contribution sourced from the cuu-dieu tradition. Trung / Cát stars stay omitted (non-occurring, not missing evidence).
_Avoid_: star veto, opaque star score

**Explanation Projection**:
A pure, deterministic projection over the personal-day, hour-ranking, and direction assessment traces that consolidates the favorable and adverse factors that influenced the result, the deduplicated facts that were collapsed to prevent double-counting, the named vetoes that won precedence, the missing evidence, and a per-dimension confidence breakdown. Built by `explain_day_assessment`, `explain_hour_ranking`, and `explain_direction_assessment`; serialized identically through the API, terminal, and desktop surfaces under the `explanation-projection` v1 contract (`amlich-bz0f.6`).
_Avoid_: raw score, opaque score, lucky score

**Deduplicated Fact**:
A typed rule describing which inputs were collapsed into a single observation so the same underlying signal cannot inflate an axis twice. Examples: Bazi target-day pillar relations collapse to one observation per relation kind (clash / lục hợp / tam hợp); non-Bazi annual pressure observations collapse to one per declared system (Tam Tai / Kim Lâu / Hoàng Ốc / Thái Tuế / sao hạn); direction constraint facts collapse to one per (direction, fact-family) pair.
_Avoid_: double-count, pillar duel, sao hạn inflation

**Precedence Rule**:
The deterministic rule a consumer is reading when interpreting why a result looks the way it does. The current explanation projection always reports `veto_overrides_aggregation`: a named veto forces the decision to `Avoid` (or the veto's declared bucket) regardless of the weighted aggregation. The rule is reported on the `precedence_rule` field of every `*Explanation` so terminal, API, and desktop surfaces render the same precedence narrative.
_Avoid_: weighted only, veto equals weight

**Confidence Reason**:
A typed row in the explanation's confidence breakdown that says which evidence dimension is present (`date` / `time` / `gender` / `location` / `direction_overlay`) and what the user-visible consequence of the dimension's presence is. The decision confidence level is derived from the capability flags the policy already inspects; the reason list surfaces the underlying dimensions so a `Medium` confidence is not silently misread as a strong signal.
_Avoid_: confidence score, magic level
