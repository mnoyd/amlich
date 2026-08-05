//! Declared, source-attributed interaction features for the v2.2
//! personal-day assessment policy.
//!
//! Source spec: `docs/architecture/personal-day-audit/SCORING-POLICY-V2-SPEC.md`
//! Bead: `amlich-47wn`.
//!
//! Interactions are explicit, typed synergies between two or more feature
//! observations. The linear axis aggregation in `baseline_v2` /
//! `intent_weighted_v2` treats each feature independently; interactions
//! capture effects that only emerge when specific conditions co-occur —
//! e.g., a hard taboo is costlier for a ceremonial intent than for a
//! casual one, and a favorable Kua direction matters more for travel than
//! for medical consultations.
//!
//! ## Design rules (from the spec)
//!
//! - No interaction is inferred merely because two source facts coexist.
//!   Each term fires only on a declared, tested condition.
//! - Each interaction has a stable [`InteractionKind`] identifier, a
//!   policy weight (from [`INTERACTION_WEIGHTS_V2_2`]), source evidence,
//!   and a test fixture.
//! - Each interaction can fire at most once per assessment, so duplicate
//!   inputs cannot inflate the score.
//! - Interactions layer on top of the v2.1 intent-aware policy: axis
//!   subtotals still match v2/v2.1 from the feature aggregation, then
//!   interaction deltas are applied as a post-processing step.
//!
//! ## Experimental status
//!
//! The weights here are an explicitly experimental product policy, NOT a
//! claim of canonical traditional truth. Every weight is a multiple of
//! 0.05 for reviewability. The stability gate (`amlich-31oa`)
//! sensitivity-tests each entry at ±10% and ±20% before v2.x is promoted
//! to default.

use crate::{
    advisory::ConsultationIntent,
    assessment::{
        extraction::ResolvedAssessmentInputs,
        feature::{AssessmentFeatureId, FeatureObservation},
        trace::InteractionTerm,
        AssessmentAxis, ContributionPolarity, SourceEvidence,
    },
    bazi::analysis::DayMasterStrengthLabel,
    birth::{BirthCapability, BirthProfile},
    sources::{SOURCE_KHCBPPT, SOURCE_VN_FOLK},
    DaySnapshot,
};

use super::policy::ASSESSMENT_POLICY_V2_2_VERSION;

// ---------------------------------------------------------------------------
// Stable interaction identifiers
// ---------------------------------------------------------------------------

/// Declared interaction kinds for the v2.2 policy. Each variant maps to a
/// stable string identifier and a fixed target axis. Adding a new
/// interaction requires a policy version bump.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InteractionKind {
    /// `hard_taboo × requested_activity` — day-fortune taboos are
    /// costlier for ceremonial intents (Wedding, MovingHouse, Burial,
    /// Renovation) than for casual ones.
    HardTabooActivity,
    /// `personal_relation × important_birth_pillar` — a favorable
    /// day-person relation (Tam Hop / Liu He) reinforces personal
    /// alignment beyond the linear feature contribution.
    PersonalRelationPillar,
    /// `weak_element × day_generates_element` — the day's heavenly-stem
    /// element generates the birth chart's weak day-master element,
    /// providing supportive nourishment.
    WeakElementDayGeneration,
    /// `kua_direction × travel_intent` — a favorable Kua direction is
    /// amplified specifically for travel.
    KuaDirectionTravel,
    /// `annual_pressure × requested_activity` — yearly Hạn pressure is
    /// amplified for major life events (Wedding, MovingHouse, Burial).
    AnnualPressureActivity,
}

impl InteractionKind {
    /// All declared interaction kinds in canonical order. Used by the
    /// weight table's coverage test.
    pub const ALL: [Self; 5] = [
        Self::HardTabooActivity,
        Self::PersonalRelationPillar,
        Self::WeakElementDayGeneration,
        Self::KuaDirectionTravel,
        Self::AnnualPressureActivity,
    ];

    /// Stable, policy-versioned interaction identifier string. Never
    /// reused; renaming requires a policy version bump.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HardTabooActivity => "interaction.hard_taboo_activity",
            Self::PersonalRelationPillar => "interaction.personal_relation_pillar",
            Self::WeakElementDayGeneration => "interaction.weak_element_day_generation",
            Self::KuaDirectionTravel => "interaction.kua_direction_travel",
            Self::AnnualPressureActivity => "interaction.annual_pressure_activity",
        }
    }

    /// Axis the interaction contributes its delta to. Fixed per kind;
    /// changing the target axis requires a policy version bump.
    pub fn target_axis(self) -> AssessmentAxis {
        match self {
            Self::HardTabooActivity | Self::KuaDirectionTravel => AssessmentAxis::IntentFit,
            Self::PersonalRelationPillar | Self::WeakElementDayGeneration => {
                AssessmentAxis::PersonalAlignment
            }
            Self::AnnualPressureActivity => AssessmentAxis::AnnualPressure,
        }
    }
}

// ---------------------------------------------------------------------------
// Interaction weight table
// ---------------------------------------------------------------------------

/// Per-interaction policy weights. Each weight is a multiple of 0.05 and
/// produces the axis delta via `delta = weight × value`. Weights are
/// reviewed product policy, not canonical truth — see the experimental
/// status note on the module docs.
#[derive(Debug, Clone, Copy)]
pub struct InteractionWeight {
    pub kind: InteractionKind,
    pub weight: f32,
    /// Short rationale so reviewers can challenge the number without
    /// grepping for magic constants.
    pub rationale: &'static str,
}

/// Sparse, policy-versioned table of interaction weights. Construct via
/// [`INTERACTION_WEIGHTS_V2_2`] for the v2.2 policy.
#[derive(Debug, Clone, Copy)]
pub struct InteractionWeightTable {
    pub policy_version: &'static str,
    pub entries: &'static [InteractionWeight],
}

impl InteractionWeightTable {
    /// Look up the weight for a given interaction kind. Returns 0.0 if
    /// the kind is absent (defensive — the coverage test ensures every
    /// declared kind has an entry).
    pub fn weight_for(self, kind: InteractionKind) -> f32 {
        for entry in self.entries {
            if entry.kind == kind {
                return entry.weight;
            }
        }
        0.0
    }
}

/// The v2.2 interaction weight table.
///
/// Rationale per interaction (multiple of 0.05):
///
/// - **HardTabooActivity (0.10)** — taboos already penalize
///   `GenericDayQuality`; this interaction adds a modest extra penalty on
///   `IntentFit` only for ceremonial intents where taboos are
///   traditionally most consequential.
/// - **PersonalRelationPillar (0.10)** — Tam Hop / Liu He already
///   contribute to `PersonalAlignment` linearly; this interaction captures
///   the reinforcing synergy at a conservative magnitude.
/// - **WeakElementDayGeneration (0.15)** — a day that nourishes a weak
///   day-master is a meaningful Bazi support signal; slightly higher than
///   the relation interactions because it requires chart-level evidence.
/// - **KuaDirectionTravel (0.20)** — direction is the dominant factor for
///   travel specifically; this is the largest weight because the Kua
///   direction feature's linear contribution under-weights its importance
///   for travel relative to other intents.
/// - **AnnualPressureActivity (0.15)** — Hạn already penalizes
///   `AnnualPressure` linearly; this interaction adds a targeted extra
///   burden for major life events where Hạn is traditionally most feared.
pub const INTERACTION_WEIGHTS_V2_2: InteractionWeightTable = InteractionWeightTable {
    policy_version: ASSESSMENT_POLICY_V2_2_VERSION,
    entries: &[
        InteractionWeight {
            kind: InteractionKind::HardTabooActivity,
            weight: 0.10,
            rationale: "ceremonial intents amplify taboo penalty on intent fit",
        },
        InteractionWeight {
            kind: InteractionKind::PersonalRelationPillar,
            weight: 0.10,
            rationale: "favorable relation synergy beyond linear contribution",
        },
        InteractionWeight {
            kind: InteractionKind::WeakElementDayGeneration,
            weight: 0.15,
            rationale: "day nourishing weak day-master is meaningful Bazi support",
        },
        InteractionWeight {
            kind: InteractionKind::KuaDirectionTravel,
            weight: 0.20,
            rationale: "direction is the dominant factor for travel specifically",
        },
        InteractionWeight {
            kind: InteractionKind::AnnualPressureActivity,
            weight: 0.15,
            rationale: "major life events amplify Hạn burden on annual pressure",
        },
    ],
};

// ---------------------------------------------------------------------------
// Element generation cycle helper
// ---------------------------------------------------------------------------

/// Returns the element that `element` generates in the Ngũ Hành sinh
/// cycle. Element names are Vietnamese (`"Mộc"`, `"Hỏa"`, `"Thổ"`,
/// `"Kim"`, `"Thủy"`), matching the `NguHanh.can` / `NguHanh.chi` string
/// representation used throughout the codebase.
fn generates(element: &str) -> Option<&'static str> {
    match element {
        "Mộc" => Some("Hỏa"),
        "Hỏa" => Some("Thổ"),
        "Thổ" => Some("Kim"),
        "Kim" => Some("Thủy"),
        "Thủy" => Some("Mộc"),
        _ => None,
    }
}

/// True if the day's element generates the target element (sinh) in the
/// Ngũ Hành cycle.
fn day_generates_target(day_element: &str, target_element: &str) -> bool {
    generates(day_element) == Some(target_element)
}

// ---------------------------------------------------------------------------
// Interaction extraction
// ---------------------------------------------------------------------------

/// Extract declared, source-attributed interaction terms from the
/// resolved personal-day facts.
///
/// Each interaction fires only when its declared condition is met —
/// co-occurrence of source facts alone is never sufficient (spec: "No
/// interaction is inferred merely because two source facts coexist"). At
/// most one term per [`InteractionKind`] is emitted, so duplicate inputs
/// cannot inflate results.
///
/// Called by [`crate::assessment::AssessmentPolicy::evaluate`] only under
/// the v2.2 `interaction_aware_v2` policy; v2 / v2.1 produce no
/// interactions.
pub(super) fn extract_interactions(
    features: &[FeatureObservation],
    snapshot: &DaySnapshot,
    _profile: &BirthProfile,
    intent: ConsultationIntent,
    capability: &BirthCapability,
    resolved: &ResolvedAssessmentInputs,
    weight_table: &InteractionWeightTable,
) -> Vec<InteractionTerm> {
    let profile_id = snapshot.profile.clone();

    let interaction_evidence = |method: &'static str, note: Option<String>| SourceEvidence {
        source_family: "interaction".to_string(),
        source_id: SOURCE_KHCBPPT.to_string(),
        method: method.to_string(),
        profile: profile_id.clone(),
        note,
    };

    let mut terms: Vec<InteractionTerm> = Vec::new();

    // 1. hard_taboo × requested_activity
    //    Fires when the day has taboos AND the intent is ceremonial.
    if is_ceremonial_intent(intent) {
        let taboo_count = snapshot.day_fortune.taboos.len();
        if taboo_count > 0 {
            let taboo_strength = (taboo_count.min(3) as f32) / 3.0;
            let kind = InteractionKind::HardTabooActivity;
            terms.push(InteractionTerm {
                interaction_id: kind.as_str().to_string(),
                feature_ids: vec![AssessmentFeatureId::GenericDayQuality, AssessmentFeatureId::IntentFit],
                axis: kind.target_axis(),
                value: -taboo_strength,
                weight: weight_table.weight_for(kind),
                source_evidence: interaction_evidence(
                    "hard_taboo_activity",
                    Some(format!(
                        "taboo_count={} intent={:?} ceremonial=true",
                        taboo_count, intent
                    )),
                ),
                note: None,
            });
        }
    }

    // 2. personal_relation × important_birth_pillar
    //    Fires when a favorable personal relation (Tam Hop or Liu He) is
    //    present in the feature vector. The interaction captures the
    //    synergistic reinforcement beyond the linear contribution.
    if capability.has_gender {
        if let Some(relation) = find_favorable_relation(features) {
            let kind = InteractionKind::PersonalRelationPillar;
            terms.push(InteractionTerm {
                interaction_id: kind.as_str().to_string(),
                feature_ids: vec![relation.feature_id],
                axis: kind.target_axis(),
                value: relation.strength,
                weight: weight_table.weight_for(kind),
                source_evidence: interaction_evidence(
                    "personal_relation_pillar",
                    Some(format!(
                        "relation={} strength={:.2}",
                        relation.feature_id.as_str(),
                        relation.strength
                    )),
                ),
                note: None,
            });
        }
    }

    // 3. weak_element × day_generates_element
    //    Fires when the birth chart's day-master is weak AND the day's
    //    heavenly-stem element generates the day-master's element.
    if let (Some(chart), Some(analysis)) = (resolved.chart.as_ref(), resolved.analysis.as_ref()) {
        if analysis.day_master_strength.label == DayMasterStrengthLabel::Weak {
            let day_master_element = &chart.day_master.ngu_hanh.can;
            let day_element = &snapshot.context.canchi.day.ngu_hanh.can;
            if day_generates_target(day_element, day_master_element) {
                let kind = InteractionKind::WeakElementDayGeneration;
                terms.push(InteractionTerm {
                    interaction_id: kind.as_str().to_string(),
                    feature_ids: vec![AssessmentFeatureId::BaziElementResonance],
                    axis: kind.target_axis(),
                    value: 0.5,
                    weight: weight_table.weight_for(kind),
                    source_evidence: interaction_evidence(
                        "weak_element_day_generation",
                        Some(format!(
                            "day_element={} generates day_master_element={}",
                            day_element, day_master_element
                        )),
                    ),
                    note: None,
                });
            }
        }
    }

    // 4. kua_direction × travel_intent
    //    Fires when a favorable Kua direction matches the day's xuất
    //    hành direction AND the intent is Travel. The interaction does
    //    its own direction check against raw Kua data (rather than
    //    gating on the KuaDirectionMatch feature observation) so that
    //    the Travel-specific evidence method is attributed correctly.
    if intent == ConsultationIntent::Travel {
        if let Some(kua_result) = resolved.kua.as_ref() {
            let xuat_hanh = &snapshot.day_fortune.travel.xuat_hanh_huong;
            if kua_direction_is_favorable(kua_result, xuat_hanh) {
                let kind = InteractionKind::KuaDirectionTravel;
                terms.push(InteractionTerm {
                    interaction_id: kind.as_str().to_string(),
                    feature_ids: vec![
                        AssessmentFeatureId::KuaDirectionMatch,
                        AssessmentFeatureId::IntentFit,
                    ],
                    axis: kind.target_axis(),
                    value: 0.4,
                    weight: weight_table.weight_for(kind),
                    source_evidence: SourceEvidence {
                        source_family: "interaction".to_string(),
                        source_id: SOURCE_VN_FOLK.to_string(),
                        method: "kua_direction_travel".to_string(),
                        profile: profile_id.clone(),
                        note: Some(format!(
                            "kua={} direction={} favorable=true intent=Travel",
                            kua_result.kua, xuat_hanh
                        )),
                    },
                    note: None,
                });
            }
        }
    }

    // 5. annual_pressure × requested_activity
    //    Fires when yearly Hạn is active AND the intent is a major life
    //    event where Hạn is traditionally most consequential.
    if is_major_life_event(intent) {
        if let Some(han) = features.iter().find(|f| {
            f.feature_id == AssessmentFeatureId::AnnualThaiTue && !f.is_unavailable()
        }) {
            let kind = InteractionKind::AnnualPressureActivity;
            terms.push(InteractionTerm {
                interaction_id: kind.as_str().to_string(),
                feature_ids: vec![AssessmentFeatureId::AnnualThaiTue],
                axis: kind.target_axis(),
                value: -han.strength,
                weight: weight_table.weight_for(kind),
                source_evidence: interaction_evidence(
                    "annual_pressure_activity",
                    Some(format!(
                        "han_strength={:.2} intent={:?} major_life_event=true",
                        han.strength, intent
                    )),
                ),
                note: None,
            });
        }
    }

    terms
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

struct FavorableRelation {
    feature_id: AssessmentFeatureId,
    strength: f32,
}

/// Find the first favorable personal-relation feature (Tam Hop or Liu He)
/// in the feature vector. Returns `None` if neither is present.
fn find_favorable_relation(features: &[FeatureObservation]) -> Option<FavorableRelation> {
    for f in features {
        if f.is_unavailable() {
            continue;
        }
        if f.polarity != ContributionPolarity::Favorable {
            continue;
        }
        match f.feature_id {
            AssessmentFeatureId::PersonalTamHop => {
                return Some(FavorableRelation {
                    feature_id: f.feature_id,
                    strength: f.strength,
                });
            }
            AssessmentFeatureId::PersonalLiuHe => {
                return Some(FavorableRelation {
                    feature_id: f.feature_id,
                    strength: f.strength,
                });
            }
            _ => {}
        }
    }
    None
}

/// Intents where day-fortune taboos are traditionally most consequential.
/// These are ceremonial / life-event intents where a conflicted day is
/// especially problematic.
fn is_ceremonial_intent(intent: ConsultationIntent) -> bool {
    matches!(
        intent,
        ConsultationIntent::Wedding
            | ConsultationIntent::MovingHouse
            | ConsultationIntent::Burial
            | ConsultationIntent::Renovation
    )
}

/// Intents where yearly Hạn is traditionally most feared. These are the
/// major life events where families consult the almanac specifically to
/// avoid Hạn years.
fn is_major_life_event(intent: ConsultationIntent) -> bool {
    matches!(
        intent,
        ConsultationIntent::Wedding
            | ConsultationIntent::MovingHouse
            | ConsultationIntent::Burial
    )
}

/// Check whether the day's xuất hành direction is among the Kua group's
/// favorable directions. Compares the Vietnamese `xuat_hanh_huong` string
/// against each favorable [`Direction`] via [`Direction::as_vn_str`].
fn kua_direction_is_favorable(
    kua_result: &crate::almanac::tu_menh::KuaResult,
    xuat_hanh: &str,
) -> bool {
    let day_dir = xuat_hanh.trim();
    kua_result
        .favorable_directions
        .iter()
        .any(|d| d.as_vn_str() == day_dir)
}

// ---------------------------------------------------------------------------
// Axis delta application
// ---------------------------------------------------------------------------

/// Apply interaction deltas to the axis subtotals. Each interaction
/// contributes `weight × value` to its target axis subtotal; the updated
/// subtotal is clamped to `[0, 1]` and the verdict label is refreshed.
///
/// Called by [`crate::assessment::AssessmentPolicy::evaluate`] after the
/// feature-based axis aggregation, only when interactions are non-empty.
pub(super) fn apply_interaction_deltas(
    axes: &mut crate::assessment::AssessmentAxes,
    axis_traces: &mut [crate::assessment::trace::AxisAggregation],
    interactions: &[InteractionTerm],
) {
    for interaction in interactions {
        let delta = interaction.weight * interaction.value;

        // Update the typed AssessmentAxes outcome.
        let outcome = match interaction.axis {
            AssessmentAxis::GenericDayQuality => &mut axes.generic_day_quality,
            AssessmentAxis::IntentFit => &mut axes.intent_fit,
            AssessmentAxis::PersonalAlignment => &mut axes.personal_alignment,
            AssessmentAxis::AnnualPressure => &mut axes.annual_pressure,
            AssessmentAxis::EvidenceCoverage => &mut axes.evidence_coverage,
        };

        if let Some(score) = outcome.score {
            let new_score = (score + delta).clamp(0.0, 1.0);
            outcome.score = Some(new_score);
            outcome.verdict = super::classify_score(new_score);
        }

        // Update the trace's AxisAggregation subtotal to match.
        for trace in axis_traces.iter_mut() {
            if trace.axis == interaction.axis {
                if let Some(subtotal) = trace.subtotal {
                    let new_subtotal = (subtotal + delta).clamp(0.0, 1.0);
                    trace.subtotal = Some(new_subtotal);
                    trace.verdict = super::classify_score(new_subtotal);
                }
                break;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interaction_kinds_have_stable_strings_and_axes() {
        for kind in InteractionKind::ALL {
            assert!(!kind.as_str().is_empty());
            assert!(kind.as_str().starts_with("interaction."));
            // Every interaction must target a scored axis (not EvidenceCoverage).
            assert_ne!(
                kind.target_axis(),
                AssessmentAxis::EvidenceCoverage,
                "interactions must not target EvidenceCoverage"
            );
        }
        assert_eq!(InteractionKind::ALL.len(), 5);
    }

    #[test]
    fn v2_2_table_covers_every_kind() {
        for kind in InteractionKind::ALL {
            let weight = INTERACTION_WEIGHTS_V2_2.weight_for(kind);
            assert!(
                weight > 0.0,
                "v2.2 table is missing a weight for {:?}",
                kind
            );
        }
        assert_eq!(
            INTERACTION_WEIGHTS_V2_2.entries.len(),
            InteractionKind::ALL.len(),
            "v2.2 table must carry one entry per InteractionKind"
        );
    }

    #[test]
    fn v2_2_weights_are_reviewable_multiples_of_0_05() {
        let step = 0.05_f32;
        for entry in INTERACTION_WEIGHTS_V2_2.entries {
            let units = (entry.weight / step).round();
            assert!(
                (entry.weight - units * step).abs() < 1e-6,
                "{:?} weight is {}, not a multiple of 0.05",
                entry.kind,
                entry.weight
            );
        }
    }

    #[test]
    fn v2_2_table_is_versioned() {
        assert_eq!(INTERACTION_WEIGHTS_V2_2.policy_version, "v2.2");
    }

    #[test]
    fn element_generation_cycle_is_correct() {
        assert_eq!(generates("Mộc"), Some("Hỏa"));
        assert_eq!(generates("Hỏa"), Some("Thổ"));
        assert_eq!(generates("Thổ"), Some("Kim"));
        assert_eq!(generates("Kim"), Some("Thủy"));
        assert_eq!(generates("Thủy"), Some("Mộc"));
        assert_eq!(generates("???"), None);

        assert!(day_generates_target("Mộc", "Hỏa"));
        assert!(day_generates_target("Thủy", "Mộc"));
        assert!(!day_generates_target("Mộc", "Kim"));
        assert!(!day_generates_target("Mộc", "Mộc"));
    }

    #[test]
    fn ceremonial_intents_are_correctly_classified() {
        assert!(is_ceremonial_intent(ConsultationIntent::Wedding));
        assert!(is_ceremonial_intent(ConsultationIntent::MovingHouse));
        assert!(is_ceremonial_intent(ConsultationIntent::Burial));
        assert!(is_ceremonial_intent(ConsultationIntent::Renovation));
        assert!(!is_ceremonial_intent(ConsultationIntent::Medical));
        assert!(!is_ceremonial_intent(ConsultationIntent::Prayer));
        assert!(!is_ceremonial_intent(ConsultationIntent::Travel));
    }

    #[test]
    fn major_life_events_are_correctly_classified() {
        assert!(is_major_life_event(ConsultationIntent::Wedding));
        assert!(is_major_life_event(ConsultationIntent::MovingHouse));
        assert!(is_major_life_event(ConsultationIntent::Burial));
        assert!(!is_major_life_event(ConsultationIntent::Renovation));
        assert!(!is_major_life_event(ConsultationIntent::Medical));
    }
}
