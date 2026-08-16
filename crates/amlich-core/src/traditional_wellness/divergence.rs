//! v1.10 Tier-0 Traditional Wellness Context — time basis, review state, and
//! known-divergence types.
//!
//! These three types are the building blocks of the
//! [`BranchChannelAssociation`](super::branch_channel::BranchChannelAssociation)
//! struct and the [`TraditionalWellnessContext`](super::branch_channel::TraditionalWellnessContext)
//! wrapper. The enum/variant serializations are the JSON contract — changing
//! them is a breaking change for the corpus.

use serde::{Deserialize, Serialize};

use crate::almanac::fengshui::golden::DeferralMarker;

// ---------------------------------------------------------------------------
// TimeBasis
// ---------------------------------------------------------------------------

/// Time-basis disclosure for every Traditional Wellness Context surface.
///
/// Every traditional source verse names double-hours but does not specify
/// modern civil time zones, daylight-saving rules, longitude correction,
/// or "true solar time" (see `LUNAR_HEALTH_RESEARCH.md:66`). The
/// `LocalCivilHourBranch` variant is the disclosure that the civil-time
/// windows are an Amlich convention reusing the existing local civil
/// hour-pillar contract — not a classical claim.
///
/// Single-variant today; future variants (e.g. `LunarLocalTime`) belong to
/// a separate milestone and require their own REVIEWER-PACK sign-off.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TimeBasis {
    /// Modern local civil two-hour windows per Amlich's existing
    /// hour-pillar contract (`Tý = 23:00–01:00`). Serializes as
    /// `"local_civil_hour_branch"`.
    #[default]
    #[serde(rename = "local_civil_hour_branch")]
    LocalCivilHourBranch,
}

impl TimeBasis {
    /// String label exposed to consumers; matches the serde rename.
    pub const fn as_str(self) -> &'static str {
        match self {
            TimeBasis::LocalCivilHourBranch => "local_civil_hour_branch",
        }
    }
}

// ---------------------------------------------------------------------------
// ExternalReviewState
// ---------------------------------------------------------------------------

/// Review state of a corpus row. The JSON wire format is the free-text
/// convention used across the amlich corpus (precedent:
/// `crates/amlich-core/src/iching/schema.rs:43, 313-315` for hexagrams,
/// `crates/amlich-core/src/rituals/schema.rs:114` for rituals).
///
/// Wire-format examples (round-trippable):
/// - `ExternalReviewPending(reason="..."; expected_review_date="YYYY-MM-DD"; assigned_to="...")`
/// - `Signed(reviewer="..."; signed_on="YYYY-MM-DD")`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExternalReviewState {
    ExternalReviewPending {
        reason: String,
        expected_review_date: String,
        assigned_to: String,
    },
    Signed {
        reviewer: String,
        signed_on: String,
    },
}

impl ExternalReviewState {
    /// Serialize to the canonical free-text form. Format mirrors the
    /// `ExternalReviewPending(...)` / `Signed(...)` strings emitted by the
    /// iching and ritual corpus loaders.
    pub fn to_marker(&self) -> String {
        match self {
            ExternalReviewState::ExternalReviewPending {
                reason,
                expected_review_date,
                assigned_to,
            } => format!(
                "ExternalReviewPending(reason=\"{reason}\"; expected_review_date=\"{expected_review_date}\"; assigned_to=\"{assigned_to}\")"
            ),
            ExternalReviewState::Signed { reviewer, signed_on } => {
                format!("Signed(reviewer=\"{reviewer}\"; signed_on=\"{signed_on}\")")
            }
        }
    }

    /// Parse the canonical free-text form. Returns `None` if the string is
    /// not in either recognised shape, **or** if the payload is missing
    /// a required key, has malformed `key=value` syntax, or has an
    /// unquoted value. The strict-mode parser mirrors the strict producer
    /// ([`to_marker`]) so round-trip is symmetric.
    pub fn from_marker(marker: &str) -> Option<Self> {
        let trimmed = marker.trim();
        if let Some(rest) = trimmed
            .strip_prefix("ExternalReviewPending(")
            .and_then(|s| s.strip_suffix(")"))
        {
            let fields =
                parse_kv_fields(rest, &["reason", "expected_review_date", "assigned_to"]).ok()?;
            return Some(ExternalReviewState::ExternalReviewPending {
                reason: fields.get("reason").cloned()?,
                expected_review_date: fields.get("expected_review_date").cloned()?,
                assigned_to: fields.get("assigned_to").cloned()?,
            });
        }
        if let Some(rest) = trimmed
            .strip_prefix("Signed(")
            .and_then(|s| s.strip_suffix(")"))
        {
            let fields = parse_kv_fields(rest, &["reviewer", "signed_on"]).ok()?;
            return Some(ExternalReviewState::Signed {
                reviewer: fields.get("reviewer").cloned()?,
                signed_on: fields.get("signed_on").cloned()?,
            });
        }
        None
    }

    /// Convenience: is this state a `Signed` (review-complete) variant?
    pub fn is_signed(&self) -> bool {
        matches!(self, ExternalReviewState::Signed { .. })
    }
}

impl Serialize for ExternalReviewState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_marker())
    }
}

impl<'de> Deserialize<'de> for ExternalReviewState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        ExternalReviewState::from_marker(&raw)
            .ok_or_else(|| serde::de::Error::custom(format!("unrecognized reviewer marker: {raw}")))
    }
}

/// Parse `key="value"; key="value"` pairs into a map. Used by
/// [`ExternalReviewState::from_marker`]. Simple split on `;` then on `=`,
/// stripping surrounding double quotes — sufficient for the canonical
/// free-text format. Returns `Err` if any required key is missing or if
/// the syntax is malformed, so the reviewer marker is strict on both
/// the producing side (`to_marker`) and the consuming side
/// (`from_marker`).
fn parse_kv_fields(
    input: &str,
    required_keys: &[&str],
) -> Result<std::collections::HashMap<String, String>, String> {
    let mut out = std::collections::HashMap::new();
    for part in input.split(';') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let (k, v) = part
            .split_once('=')
            .ok_or_else(|| format!("malformed kv pair (expected key=\"value\"): {part:?}"))?;
        let k = k.trim().to_string();
        let v_trim = v.trim();
        let v = v_trim
            .strip_prefix('"')
            .and_then(|s| s.strip_suffix('"'))
            .ok_or_else(|| format!("value for key {k:?} must be double-quoted: {v_trim:?}"))?
            .to_string();
        out.insert(k, v);
    }
    for required in required_keys {
        if !out.contains_key(*required) {
            return Err(format!(
                "missing required key {required:?} in marker payload {input:?}"
            ));
        }
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// TraditionalWellnessKnownDivergence
// ---------------------------------------------------------------------------

/// A single source's value for a given divergence case. Distinct from
/// `crate::almanac::fengshui::golden::SourceValue` because that type's
/// `value: u8` is Phi-Tinh-specific; the Traditional Wellness divergences
/// carry string values (e.g. `"amlich_branch_channel_v1"`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraditionalWellnessSourceValue {
    /// Identifier for the reference source (e.g. `"khcbppt"`,
    /// `"shi-er-jing-na-di-zhi"`).
    pub source: String,
    /// The value reported by this source.
    pub value: String,
}

/// A known source divergence for Traditional Wellness Context.
///
/// Logged, never silently corrected. The `our_value` is the tiebreaker
/// selection; the losing `source_values` are preserved for audit. The
/// optional `deferral` marker signals a `PendingExternalReview`
/// disposition — the divergence has NOT been silently resolved and the
/// reviewer gate must sign before the affected corpus records are marked
/// `Signed`.
///
/// Distinct from `crate::almanac::fengshui::golden::KnownDivergence`
/// (whose `our_value: u8` is Phi-Tinh-specific) and from
/// `crate::iching::golden::MaiHoaKnownDivergence` (which carries its own
/// casting-specific fields). The three divergence types are deliberately
/// not cross-cast (per `EXPANSION_FRAMEWORK.md` tradition isolation).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraditionalWellnessKnownDivergence {
    /// Stable divergence identifier (e.g. `"LH-DIV-02"`).
    pub id: String,
    /// Human-readable case identifier (e.g. `"fixed_cycle_contestation"`).
    pub case: String,
    /// The value we select after applying the tiebreaker
    /// (e.g. `"amlich_branch_channel_v1"`).
    pub our_value: String,
    /// All source values, including the losing ones.
    pub source_values: Vec<TraditionalWellnessSourceValue>,
    /// Which tiebreaker was applied and which source won.
    pub tiebreaker: String,
    /// Additional context on why this divergence exists.
    pub note: String,
    /// Optional typed `PendingExternalReview` deferral marker.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deferral: Option<DeferralMarker>,
}

/// The LH-DIV-02 fixed-cycle contestation divergence. Per
/// `LUNAR_HEALTH_RESEARCH.md:71-73`, *Zhenjiu Fengyuan* and *Maijue
/// Huibian* preserve the mnemonic while criticising the fixed
/// one-channel-per-double-hour allocation. The Amlich disposition is
/// "historical association only — no physiological claim", with the
/// classical-Chinese review gate as the deferred-resolution path.
pub fn fixed_cycle_contestation() -> TraditionalWellnessKnownDivergence {
    TraditionalWellnessKnownDivergence {
        id: "LH-DIV-02".to_string(),
        case: "fixed_cycle_contestation".to_string(),
        our_value: "amlich_branch_channel_v1".to_string(),
        source_values: vec![
            TraditionalWellnessSourceValue {
                source: "Zhenjiu Daquan".to_string(),
                value: "fixed_one_channel_per_double_hour".to_string(),
            },
            TraditionalWellnessSourceValue {
                source: "Zhenjiu Fengyuan".to_string(),
                value: "fixed_one_channel_per_double_hour_criticised".to_string(),
            },
            TraditionalWellnessSourceValue {
                source: "Maijue Huibian".to_string(),
                value: "fixed_one_channel_per_double_hour_preserved_pending_correction".to_string(),
            },
        ],
        tiebreaker: "historical_association_only_no_physiological_claim".to_string(),
        note: "Later classical authors preserve but explicitly criticise the fixed \
               one-channel-per-double-hour allocation. Amlich surfaces as historical \
               association with a mandatory divergence marker; no physiology claim."
            .to_string(),
        deferral: Some(DeferralMarker {
            reason: "classical_12_row_table_review_pending".to_string(),
            expected_review_date: "2026-12-31".to_string(),
            assigned_to: Some("classical_chinese_reviewer".to_string()),
        }),
    }
}

/// The LH-DIV-03 civil-time disclosure divergence. Per
/// `LUNAR_HEALTH_RESEARCH.md:66` and `LH-DIV-03` row at `:221`, classical
/// timekeeping does not define modern civil-zone, DST, or
/// longitude-correction behaviour. The Amlich disposition is to reuse
/// the existing local civil hour-branch contract and disclose the time
/// basis (`local_civil_hour_branch`) rather than claim classical
/// exactness.
pub fn civil_time_disclosure() -> TraditionalWellnessKnownDivergence {
    TraditionalWellnessKnownDivergence {
        id: "LH-DIV-03".to_string(),
        case: "civil_time_disclosure".to_string(),
        our_value: "amlich_local_civil_hour_branch_v1".to_string(),
        source_values: vec![
            TraditionalWellnessSourceValue {
                source: "classical_timekeeping".to_string(),
                value: "double_hour_no_modern_zone".to_string(),
            },
            TraditionalWellnessSourceValue {
                source: "amlich_existing_contract".to_string(),
                value: "local_civil_two_hour_window".to_string(),
            },
        ],
        tiebreaker: "disclose_time_basis_no_classical_exactness_claim".to_string(),
        note: "Classical verses name double-hours but do not specify modern time zones, \
               daylight-saving rules, longitude correction, or 'true solar time'. The \
               BranchChannelAssociation.time_basis field carries the disclosure."
            .to_string(),
        deferral: Some(DeferralMarker {
            reason: "classical_12_row_table_review_pending".to_string(),
            expected_review_date: "2026-12-31".to_string(),
            assigned_to: Some("classical_chinese_reviewer".to_string()),
        }),
    }
}

/// The LH-DIV-06 channel-not-organ divergence. Per
/// `LUNAR_HEALTH_RESEARCH.md:186` and the `LH-DIV-06` row at `:224`,
/// classical `臟腑` labels do not map one-to-one to modern
/// anatomy/physiology, and `心包` (Tâm bào / Pericardium) and `三焦`
/// (Tam tiêu / Triple Burner) are especially unsafe to biomedicalise.
/// The Amlich disposition is to preserve the traditional names and use
/// "channel" rather than "organ function".
pub fn channel_not_organ() -> TraditionalWellnessKnownDivergence {
    TraditionalWellnessKnownDivergence {
        id: "LH-DIV-06".to_string(),
        case: "channel_not_organ".to_string(),
        our_value: "channel_name_verbatim".to_string(),
        source_values: vec![
            TraditionalWellnessSourceValue {
                source: "classical_zangfu_taxonomy".to_string(),
                value: "traditional_channel_name".to_string(),
            },
            TraditionalWellnessSourceValue {
                source: "modern_anatomy".to_string(),
                value: "biomedical_organ_claim_explicitly_avoided".to_string(),
            },
        ],
        tiebreaker: "preserve_traditional_names_use_channel_not_organ".to_string(),
        note: "Channel names (especially 心包 / 三焦) must not be biomedicalized. The \
               BranchChannelAssociation.channel_zh / channel_vi / channel_en fields \
               carry the verbatim labels."
            .to_string(),
        deferral: Some(DeferralMarker {
            reason: "classical_12_row_table_review_pending".to_string(),
            expected_review_date: "2026-12-31".to_string(),
            assigned_to: Some("classical_chinese_reviewer".to_string()),
        }),
    }
}

/// All Traditional Wellness divergences relevant to the branch-channel
/// track. Three entries today; future additions belong to the same
/// registry and must reuse the [`TraditionalWellnessKnownDivergence`]
/// shape verbatim.
pub fn all_divergences_for_branch_channel() -> Vec<TraditionalWellnessKnownDivergence> {
    vec![
        fixed_cycle_contestation(),
        civil_time_disclosure(),
        channel_not_organ(),
    ]
}

/// The LH-DIV-04 four-profiles-vs-24-terms divergence (seasonal track).
/// Per `LUNAR_HEALTH_RESEARCH.md:222`, *Suwen* `四氣調神大論` supplies one
/// routine profile per three-month season — four profiles — while the
/// product key is one of 24 solar terms. The Amlich disposition is a
/// transparent deterministic composition (terms joined to seasons at the
/// four Lập boundaries), emitted as the composite
/// `rule.composite.seasonal_wellness` and never presented as a
/// term-specific classical prescription.
pub fn four_profiles_not_term_regimens() -> TraditionalWellnessKnownDivergence {
    TraditionalWellnessKnownDivergence {
        id: "LH-DIV-04".to_string(),
        case: "four_profiles_not_24_term_regimens".to_string(),
        our_value: "amlich_term_to_season_composition_v1".to_string(),
        source_values: vec![
            TraditionalWellnessSourceValue {
                source: "Huangdi Neijing Suwen".to_string(),
                value: "four_seasonal_profiles".to_string(),
            },
            TraditionalWellnessSourceValue {
                source: "product_ui_key".to_string(),
                value: "one_of_24_solar_terms".to_string(),
            },
        ],
        tiebreaker: "transparent_amlich_composition_never_source_claim".to_string(),
        note: "Suwen supplies four seasonal routine profiles, not a regimen per \
                solar term. The term-to-season join is an Amlich presentation \
                composition disclosed in every result; no term-specific \
                prescription is emitted."
            .to_string(),
        deferral: Some(DeferralMarker {
            reason: "suwen_four_season_paraphrase_review_pending".to_string(),
            expected_review_date: "2026-12-31".to_string(),
            assigned_to: Some("suwen_paraphrase_reviewer".to_string()),
        }),
    }
}

/// The LH-DIV-05 Yellow-River phenology divergence (seasonal track). Per
/// `LUNAR_HEALTH_RESEARCH.md:223`, solar-term phenology formed around the
/// Yellow River; actual weather seasons differ greatly by geography. The
/// Amlich disposition is to emit no local-weather or exposure advice —
/// the profiles describe the historical text only.
pub fn phenology_is_not_local_weather() -> TraditionalWellnessKnownDivergence {
    TraditionalWellnessKnownDivergence {
        id: "LH-DIV-05".to_string(),
        case: "yellow_river_phenology_not_local_weather".to_string(),
        our_value: "historical_text_description_only".to_string(),
        source_values: vec![
            TraditionalWellnessSourceValue {
                source: "China Meteorological Administration".to_string(),
                value: "terms_originated_in_yellow_river_phenology".to_string(),
            },
            TraditionalWellnessSourceValue {
                source: "regional_weather_reality".to_string(),
                value: "seasonal_weather_varies_by_geography".to_string(),
            },
        ],
        tiebreaker: "no_local_weather_or_exposure_advice".to_string(),
        note: "Solar-term phenology was formed around the Yellow River and \
                weather seasons differ by geography. The seasonal profiles \
                describe the classical text only; no local-weather or \
                exposure advice is emitted."
            .to_string(),
        deferral: Some(DeferralMarker {
            reason: "suwen_four_season_paraphrase_review_pending".to_string(),
            expected_review_date: "2026-12-31".to_string(),
            assigned_to: Some("suwen_paraphrase_reviewer".to_string()),
        }),
    }
}

/// The LH-DIV-07 omitted-consequences divergence (seasonal track). Per
/// `LUNAR_HEALTH_RESEARCH.md:225`, the classical seasonal chapter
/// attaches organ-injury and later-illness consequences to acting
/// contrary to each season. Those are claims inside the historical
/// text and are not evidence of modern clinical validity; the Amlich
/// disposition is to omit them from Tier-0 output entirely and retain
/// them only in the research audit notes.
pub fn organ_injury_clauses_omitted() -> TraditionalWellnessKnownDivergence {
    TraditionalWellnessKnownDivergence {
        id: "LH-DIV-07".to_string(),
        case: "organ_injury_and_disease_consequences_omitted".to_string(),
        our_value: "routine_themes_only".to_string(),
        source_values: vec![
            TraditionalWellnessSourceValue {
                source: "Huangdi Neijing Suwen".to_string(),
                value: "chapter_includes_organ_injury_and_illness_consequences".to_string(),
            },
            TraditionalWellnessSourceValue {
                source: "amlich_tier0_scope".to_string(),
                value: "consequences_omitted_from_output".to_string(),
            },
        ],
        tiebreaker: "omit_disease_clauses_keep_routine_themes".to_string(),
        note: "The classical chapter attaches organ-injury and later-illness \
                consequences to acting contrary to each season. Those claims \
                are omitted from Tier-0 output; only the routine themes are \
                paraphrased, framed as historical description."
            .to_string(),
        deferral: Some(DeferralMarker {
            reason: "suwen_four_season_paraphrase_review_pending".to_string(),
            expected_review_date: "2026-12-31".to_string(),
            assigned_to: Some("suwen_paraphrase_reviewer".to_string()),
        }),
    }
}

/// All Traditional Wellness divergences relevant to the seasonal
/// cultivation track. Three entries today; future additions belong to
/// the same registry and must reuse the
/// [`TraditionalWellnessKnownDivergence`] shape verbatim.
pub fn all_divergences_for_seasonal_cultivation() -> Vec<TraditionalWellnessKnownDivergence> {
    vec![
        four_profiles_not_term_regimens(),
        phenology_is_not_local_weather(),
        organ_injury_clauses_omitted(),
    ]
}

/// Look up a divergence by its `id` (e.g. `"LH-DIV-02"`). Returns `None`
/// for unknown ids — the corpus loaders use this to assert every row's
/// `known_divergence_ids` resolves to an entry in this registry (the
/// closed-world contract asserted by `tests/branch_channel_integration.rs`
/// and `tests/seasonal_cultivation_integration.rs`).
pub fn divergence_by_id(id: &str) -> Option<TraditionalWellnessKnownDivergence> {
    all_divergences_for_branch_channel()
        .into_iter()
        .chain(all_divergences_for_seasonal_cultivation())
        .find(|d| d.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_basis_serializes_as_canonical_string() {
        let tb = TimeBasis::LocalCivilHourBranch;
        assert_eq!(tb.as_str(), "local_civil_hour_branch");
        assert_eq!(
            serde_json::to_string(&tb).unwrap(),
            "\"local_civil_hour_branch\""
        );
        let roundtrip: TimeBasis = serde_json::from_str("\"local_civil_hour_branch\"").unwrap();
        assert_eq!(roundtrip, tb);
    }

    #[test]
    fn external_review_state_round_trips_via_free_text() {
        let pending = ExternalReviewState::ExternalReviewPending {
            reason: "classical_12_row_table_review_pending".to_string(),
            expected_review_date: "2026-12-31".to_string(),
            assigned_to: "classical_chinese_reviewer".to_string(),
        };
        let signed = ExternalReviewState::Signed {
            reviewer: "classical_chinese_reviewer".to_string(),
            signed_on: "2026-12-31".to_string(),
        };

        for state in [&pending, &signed] {
            let json = serde_json::to_string(state).unwrap();
            let recovered: ExternalReviewState = serde_json::from_str(&json).unwrap();
            assert_eq!(&recovered, state);
        }
    }

    #[test]
    fn external_review_state_marker_format_matches_iching_precedent() {
        let pending = ExternalReviewState::ExternalReviewPending {
            reason: "classical_12_row_table_review_pending".to_string(),
            expected_review_date: "2026-12-31".to_string(),
            assigned_to: "classical_chinese_reviewer".to_string(),
        };
        assert_eq!(
            pending.to_marker(),
            "ExternalReviewPending(reason=\"classical_12_row_table_review_pending\"; \
             expected_review_date=\"2026-12-31\"; assigned_to=\"classical_chinese_reviewer\")"
        );
        let parsed = ExternalReviewState::from_marker(&pending.to_marker()).unwrap();
        assert_eq!(parsed, pending);
    }

    #[test]
    fn external_review_state_signed_marker_round_trips() {
        let signed = ExternalReviewState::Signed {
            reviewer: "classical_chinese_reviewer".to_string(),
            signed_on: "2026-12-31".to_string(),
        };
        let marker = signed.to_marker();
        assert!(marker.starts_with("Signed("));
        let parsed = ExternalReviewState::from_marker(&marker).unwrap();
        assert_eq!(parsed, signed);
        assert!(parsed.is_signed());
    }

    #[test]
    fn unknown_marker_is_rejected_at_deserialize() {
        let bad = "\"RandomUnstructuredText()\"";
        let result: Result<ExternalReviewState, _> = serde_json::from_str(bad);
        assert!(
            result.is_err(),
            "unknown marker format must fail deserialization"
        );
    }

    #[test]
    fn fixed_cycle_contestation_carries_lh_div_02_with_deferral() {
        let d = fixed_cycle_contestation();
        assert_eq!(d.id, "LH-DIV-02");
        assert_eq!(d.our_value, "amlich_branch_channel_v1");
        assert_eq!(
            d.tiebreaker,
            "historical_association_only_no_physiological_claim"
        );
        let deferral = d
            .deferral
            .expect("fixed_cycle_contestation must carry a deferral");
        assert_eq!(deferral.reason, "classical_12_row_table_review_pending");
        assert_eq!(
            deferral.assigned_to.as_deref(),
            Some("classical_chinese_reviewer")
        );
        assert!(
            d.source_values.len() >= 2,
            "must cite at least two source values"
        );
    }

    #[test]
    fn divergence_by_id_resolves_lh_div_02() {
        let d = divergence_by_id("LH-DIV-02").expect("LH-DIV-02 must be registered");
        assert_eq!(d.id, "LH-DIV-02");
        assert!(divergence_by_id("LH-DIV-99").is_none());
    }

    #[test]
    fn divergence_registry_covers_every_id_used_by_corpus() {
        // Lockstep between the corpus JSON's `known_divergence_ids` and
        // the in-code registry. The corpus uses LH-DIV-02/03/06 on every
        // row; the registry must know all three.
        for id in ["LH-DIV-02", "LH-DIV-03", "LH-DIV-06"] {
            assert!(divergence_by_id(id).is_some(), "registry must resolve {id}");
        }
    }

    #[test]
    fn civil_time_disclosure_carries_deferral_to_classical_reviewer() {
        let d = civil_time_disclosure();
        assert_eq!(d.id, "LH-DIV-03");
        assert!(
            d.deferral.is_some(),
            "LH-DIV-03 must carry a deferral marker"
        );
    }

    #[test]
    fn channel_not_organ_carries_deferral_to_classical_reviewer() {
        let d = channel_not_organ();
        assert_eq!(d.id, "LH-DIV-06");
        assert!(
            d.deferral.is_some(),
            "LH-DIV-06 must carry a deferral marker"
        );
    }

    #[test]
    fn seasonal_divergences_carry_ids_and_deferrals() {
        let d4 = four_profiles_not_term_regimens();
        assert_eq!(d4.id, "LH-DIV-04");
        assert_eq!(d4.our_value, "amlich_term_to_season_composition_v1");
        assert!(
            d4.deferral.is_some(),
            "LH-DIV-04 must carry a deferral marker"
        );

        let d5 = phenology_is_not_local_weather();
        assert_eq!(d5.id, "LH-DIV-05");
        assert_eq!(d5.tiebreaker, "no_local_weather_or_exposure_advice");
        assert!(
            d5.deferral.is_some(),
            "LH-DIV-05 must carry a deferral marker"
        );

        let d7 = organ_injury_clauses_omitted();
        assert_eq!(d7.id, "LH-DIV-07");
        assert_eq!(d7.tiebreaker, "omit_disease_clauses_keep_routine_themes");
        assert!(
            d7.deferral.is_some(),
            "LH-DIV-07 must carry a deferral marker"
        );
    }

    #[test]
    fn divergence_registry_covers_every_seasonal_id_used_by_corpus() {
        // Lockstep between the seasonal corpus JSON's
        // `known_divergence_ids` and the in-code registry.
        for id in ["LH-DIV-04", "LH-DIV-05", "LH-DIV-07"] {
            assert!(divergence_by_id(id).is_some(), "registry must resolve {id}");
        }
        assert_eq!(
            all_divergences_for_seasonal_cultivation().len(),
            3,
            "seasonal registry must contain exactly three divergences"
        );
    }

    #[test]
    fn divergence_by_id_resolves_across_both_tracks() {
        assert!(divergence_by_id("LH-DIV-02").is_some());
        assert!(divergence_by_id("LH-DIV-04").is_some());
        assert!(divergence_by_id("LH-DIV-07").is_some());
        assert!(divergence_by_id("LH-DIV-99").is_none());
    }
}
