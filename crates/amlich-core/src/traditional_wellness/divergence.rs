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
    /// not in either recognised shape — the loader falls back to treating
    /// the raw string as opaque text.
    pub fn from_marker(marker: &str) -> Option<Self> {
        let trimmed = marker.trim();
        if let Some(rest) = trimmed
            .strip_prefix("ExternalReviewPending(")
            .and_then(|s| s.strip_suffix(")"))
        {
            let fields = parse_kv_fields(rest);
            return Some(ExternalReviewState::ExternalReviewPending {
                reason: fields.get("reason").cloned().unwrap_or_default(),
                expected_review_date: fields
                    .get("expected_review_date")
                    .cloned()
                    .unwrap_or_default(),
                assigned_to: fields.get("assigned_to").cloned().unwrap_or_default(),
            });
        }
        if let Some(rest) = trimmed
            .strip_prefix("Signed(")
            .and_then(|s| s.strip_suffix(")"))
        {
            let fields = parse_kv_fields(rest);
            return Some(ExternalReviewState::Signed {
                reviewer: fields.get("reviewer").cloned().unwrap_or_default(),
                signed_on: fields.get("signed_on").cloned().unwrap_or_default(),
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
/// free-text format.
fn parse_kv_fields(input: &str) -> std::collections::HashMap<String, String> {
    let mut out = std::collections::HashMap::new();
    for part in input.split(';') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        if let Some((k, v)) = part.split_once('=') {
            let k = k.trim().to_string();
            let v = v.trim();
            let v = v
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .unwrap_or(v)
                .to_string();
            out.insert(k, v);
        }
    }
    out
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
                value: "fixed_one_channel_per_double_hour_preserved_pending_correction"
                    .to_string(),
            },
        ],
        tiebreaker: "historical_association_only_no_physiological_claim".to_string(),
        note: "Later classical authors preserve but explicitly criticise the fixed \
               one-channel-per-double-hour allocation. Amlich surfaces as historical \
               association with a mandatory divergence marker; no physiology claim."
            .to_string(),
        deferral: Some(DeferralMarker {
            reason: "classical_12_row_table_review_pending".to_string(),
            expected_review_date: "YYYY-MM-DD".to_string(),
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
            expected_review_date: "YYYY-MM-DD".to_string(),
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
            expected_review_date: "YYYY-MM-DD".to_string(),
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

/// Look up a divergence by its `id` (e.g. `"LH-DIV-02"`). Returns `None`
/// for unknown ids — the corpus loader uses this to assert every row's
/// `known_divergence_ids` resolves to an entry in this registry (the
/// closed-world contract asserted by `tests/branch_channel_integration.rs`).
pub fn divergence_by_id(id: &str) -> Option<TraditionalWellnessKnownDivergence> {
    all_divergences_for_branch_channel()
        .into_iter()
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
            expected_review_date: "YYYY-MM-DD".to_string(),
            assigned_to: "classical_chinese_reviewer".to_string(),
        };
        assert_eq!(
            pending.to_marker(),
            "ExternalReviewPending(reason=\"classical_12_row_table_review_pending\"; \
             expected_review_date=\"YYYY-MM-DD\"; assigned_to=\"classical_chinese_reviewer\")"
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
            assert!(
                divergence_by_id(id).is_some(),
                "registry must resolve {id}"
            );
        }
    }

    #[test]
    fn civil_time_disclosure_carries_deferral_to_classical_reviewer() {
        let d = civil_time_disclosure();
        assert_eq!(d.id, "LH-DIV-03");
        assert!(d.deferral.is_some(), "LH-DIV-03 must carry a deferral marker");
    }

    #[test]
    fn channel_not_organ_carries_deferral_to_classical_reviewer() {
        let d = channel_not_organ();
        assert_eq!(d.id, "LH-DIV-06");
        assert!(d.deferral.is_some(), "LH-DIV-06 must carry a deferral marker");
    }
}
