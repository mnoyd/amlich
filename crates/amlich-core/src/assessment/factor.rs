//! Canonical factor classification for personal-day assessment.
//!
//! A factor role describes how an input participates in the current
//! assessment policy, not whether the underlying tradition considers the
//! input important. Raw calendar context is a `fact`; observations included
//! in weighted aggregation are `scored_feature`; named hard constraints are
//! `veto`; and useful almanac context that does not directly move the current
//! verdict is `explanation_only`.
//!
//! Keeping this classification inside the assessment module prevents API,
//! terminal, and desktop consumers from guessing roles from rule names.

use serde::{Deserialize, Serialize};

use crate::{
    almanac::types::RuleEvidence,
    assessment::{
        AssessmentAxis, AssessmentTrace, AvailabilityState, DecisionContribution, SourceEvidence,
        UnavailableSection,
    },
    DaySnapshot,
};

/// Stable semantic role of one input in the current assessment policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssessmentFactorRole {
    Fact,
    ScoredFeature,
    Veto,
    ExplanationOnly,
}

impl AssessmentFactorRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fact => "fact",
            Self::ScoredFeature => "scored_feature",
            Self::Veto => "veto",
            Self::ExplanationOnly => "explanation_only",
        }
    }
}

/// One classified assessment input exposed to explanation consumers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssessmentFactor {
    /// Stable identifier within the assessment policy. Scored features use
    /// their stable contribution id so a consumer can join them to the
    /// contribution and calculation trace without fuzzy matching.
    pub factor_id: String,
    pub role: AssessmentFactorRole,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub axis: Option<AssessmentAxis>,
    pub availability: AvailabilityState,
    pub source_evidence: SourceEvidence,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// Classify the day inputs after assessment calculation.
///
/// This function is deliberately pure and has no influence on score or
/// verdict. V2 assessments consume the exact feature/veto trace. Legacy v1
/// assessments project their contribution set and explicit unavailable
/// sections, preserving the observable policy while the default remains v1.
pub(super) fn classify_day_factors(
    snapshot: &DaySnapshot,
    contributions: &[DecisionContribution],
    unavailable_sections: &[UnavailableSection],
    trace: Option<&AssessmentTrace>,
) -> Vec<AssessmentFactor> {
    let mut factors = context_facts(snapshot);

    match trace {
        Some(trace) => {
            factors.extend(trace.features.iter().map(|feature| AssessmentFactor {
                factor_id: feature.contribution_id.clone(),
                role: AssessmentFactorRole::ScoredFeature,
                axis: Some(feature.feature_id.default_axis()),
                availability: feature.availability.clone(),
                source_evidence: feature.source_evidence.clone(),
                note: feature.note.clone(),
            }));
            factors.extend(trace.vetoes.iter().map(|veto| AssessmentFactor {
                factor_id: veto.veto_id.clone(),
                role: AssessmentFactorRole::Veto,
                axis: Some(veto.axis),
                availability: AvailabilityState::Complete,
                source_evidence: veto.source_evidence.clone(),
                note: Some(veto.reason.clone()),
            }));
        }
        None => {
            factors.extend(contributions.iter().map(|contribution| AssessmentFactor {
                factor_id: contribution.contribution_id.clone(),
                role: AssessmentFactorRole::ScoredFeature,
                axis: Some(contribution.axis),
                availability: contribution.availability.clone(),
                source_evidence: contribution.source_evidence.clone(),
                note: contribution.note.clone(),
            }));

            // V1 applies this threshold as its legacy hard-override rule.
            // The classification exposes that existing behavior; it does not
            // introduce or evaluate a new veto.
            factors.extend(
                contributions
                    .iter()
                    .filter(|contribution| {
                        matches!(contribution.polarity, super::ContributionPolarity::Avoid)
                            && contribution.strength >= 0.8
                    })
                    .map(|contribution| AssessmentFactor {
                        factor_id: format!("veto.legacy.{}", contribution.contribution_id),
                        role: AssessmentFactorRole::Veto,
                        axis: Some(contribution.axis),
                        availability: contribution.availability.clone(),
                        source_evidence: contribution.source_evidence.clone(),
                        note: contribution.note.clone().or_else(|| {
                            Some("Legacy v1 hard-override threshold applied".to_string())
                        }),
                    }),
            );

            factors.extend(unavailable_sections.iter().map(|section| AssessmentFactor {
                factor_id: format!("unavailable.{}", section.section),
                role: AssessmentFactorRole::ScoredFeature,
                axis: axis_for_unavailable_section(&section.section),
                availability: AvailabilityState::Unavailable {
                    reason: section.reason.clone(),
                },
                source_evidence: snapshot_evidence(snapshot, "capability_check", None),
                note: Some(format!(
                    "required_fields={}",
                    section.required_fields.join(",")
                )),
            }));
        }
    }

    factors.extend(explanation_only_factors(snapshot));
    factors
}

fn context_facts(snapshot: &DaySnapshot) -> Vec<AssessmentFactor> {
    [
        (
            "fact.day.calendar",
            "calendar_context",
            format!(
                "solar={:02}/{:02}/{} lunar={:02}/{:02}/{}",
                snapshot.context.solar.day,
                snapshot.context.solar.month,
                snapshot.context.solar.year,
                snapshot.context.lunar.day,
                snapshot.context.lunar.month,
                snapshot.context.lunar.year
            ),
        ),
        (
            "fact.day.canchi",
            "can_chi_context",
            format!(
                "day={} month={} year={}",
                snapshot.context.canchi.day.full,
                snapshot.context.canchi.month.full,
                snapshot.context.canchi.year.full
            ),
        ),
        (
            "fact.day.tiet_khi",
            "tiet_khi_context",
            snapshot.context.tiet_khi.name.clone(),
        ),
        (
            "fact.day.hour_table",
            "gio_hoang_dao_context",
            format!(
                "good={} total={}",
                snapshot.context.gio_hoang_dao.good_hour_count,
                snapshot.context.gio_hoang_dao.all_hours.len()
            ),
        ),
    ]
    .into_iter()
    .map(|(factor_id, method, note)| AssessmentFactor {
        factor_id: factor_id.to_string(),
        role: AssessmentFactorRole::Fact,
        axis: None,
        availability: AvailabilityState::Complete,
        source_evidence: snapshot_evidence(snapshot, method, None),
        note: Some(note),
    })
    .collect()
}

fn explanation_only_factors(snapshot: &DaySnapshot) -> Vec<AssessmentFactor> {
    let fortune = &snapshot.day_fortune;
    let mut factors = vec![
        explanation_factor(
            snapshot,
            "explanation.day_element",
            "day_element",
            fortune.day_element.evidence.as_ref(),
            Some(format!(
                "na_am={} element={}",
                fortune.day_element.na_am, fortune.day_element.element
            )),
        ),
        explanation_factor(
            snapshot,
            "explanation.day_conflict",
            "day_conflict",
            fortune.conflict.evidence.as_ref(),
            Some(format!("opposing_chi={}", fortune.conflict.opposing_chi)),
        ),
        explanation_factor(
            snapshot,
            "explanation.travel_direction",
            "travel_direction",
            fortune.travel.evidence.as_ref(),
            Some(format!(
                "xuat_hanh={} tai_than={} hy_than={}",
                fortune.travel.xuat_hanh_huong, fortune.travel.tai_than, fortune.travel.hy_than
            )),
        ),
        explanation_factor(
            snapshot,
            "explanation.day_stars",
            "day_stars",
            fortune.stars.evidence.as_ref(),
            Some(format!(
                "cat={} sat={}",
                fortune.stars.cat_tinh.len(),
                fortune.stars.sat_tinh.len()
            )),
        ),
        explanation_factor(
            snapshot,
            "explanation.day_taboos",
            "day_taboos",
            fortune
                .taboos
                .first()
                .and_then(|taboo| taboo.evidence.as_ref()),
            Some(format!("count={}", fortune.taboos.len())),
        ),
        explanation_factor(
            snapshot,
            "explanation.xung_hop",
            "xung_hop",
            None,
            Some(format!("luc_xung={}", fortune.xung_hop.luc_xung)),
        ),
        explanation_factor(
            snapshot,
            "explanation.truc",
            "truc",
            fortune.truc.evidence.as_ref(),
            Some(format!(
                "name={} quality={}",
                fortune.truc.name, fortune.truc.quality
            )),
        ),
    ];

    if let Some(day_deity) = fortune.day_deity.as_ref() {
        factors.push(explanation_factor(
            snapshot,
            "explanation.day_deity",
            "day_deity",
            day_deity.evidence.as_ref(),
            Some(day_deity.name.clone()),
        ));
    }
    if fortune.tang_can.is_some() {
        factors.push(explanation_factor(
            snapshot,
            "explanation.tang_can",
            "tang_can",
            None,
            None,
        ));
    }
    if fortune.ten_gods.is_some() {
        factors.push(explanation_factor(
            snapshot,
            "explanation.ten_gods",
            "ten_gods",
            None,
            None,
        ));
    }
    if snapshot.flying_stars.is_some() || snapshot.daily_flying_stars.is_some() {
        factors.push(explanation_factor(
            snapshot,
            "explanation.flying_stars",
            "flying_stars",
            None,
            None,
        ));
    }
    if snapshot
        .applicable_rituals
        .as_ref()
        .is_some_and(|rituals| !rituals.is_empty())
    {
        factors.push(explanation_factor(
            snapshot,
            "explanation.rituals",
            "ritual_match",
            None,
            None,
        ));
    }
    if snapshot.iching_cast.is_some() {
        factors.push(explanation_factor(
            snapshot,
            "explanation.iching",
            "explicit_iching_enrichment",
            None,
            Some("Explicit consultation enrichment; does not change day score".to_string()),
        ));
    }

    factors
}

fn explanation_factor(
    snapshot: &DaySnapshot,
    factor_id: &str,
    method: &str,
    evidence: Option<&RuleEvidence>,
    note: Option<String>,
) -> AssessmentFactor {
    AssessmentFactor {
        factor_id: factor_id.to_string(),
        role: AssessmentFactorRole::ExplanationOnly,
        axis: None,
        availability: AvailabilityState::Complete,
        source_evidence: evidence
            .map(|evidence| SourceEvidence {
                source_family: "almanac_rule".to_string(),
                source_id: evidence.source_id.clone(),
                method: evidence.method.clone(),
                profile: evidence.profile.clone(),
                note: None,
            })
            .unwrap_or_else(|| snapshot_evidence(snapshot, method, None)),
        note,
    }
}

fn snapshot_evidence(snapshot: &DaySnapshot, method: &str, note: Option<String>) -> SourceEvidence {
    SourceEvidence {
        source_family: "snapshot".to_string(),
        source_id: snapshot.ruleset_id.clone(),
        method: method.to_string(),
        profile: snapshot.profile.clone(),
        note,
    }
}

fn axis_for_unavailable_section(section: &str) -> Option<AssessmentAxis> {
    match section {
        "personal_alignment" | "personal_kua" => Some(AssessmentAxis::PersonalAlignment),
        "annual_pressure" | "annual_han" => Some(AssessmentAxis::AnnualPressure),
        "personal_hours" => Some(AssessmentAxis::IntentFit),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roles_have_stable_wire_names() {
        assert_eq!(AssessmentFactorRole::Fact.as_str(), "fact");
        assert_eq!(
            AssessmentFactorRole::ScoredFeature.as_str(),
            "scored_feature"
        );
        assert_eq!(AssessmentFactorRole::Veto.as_str(), "veto");
        assert_eq!(
            AssessmentFactorRole::ExplanationOnly.as_str(),
            "explanation_only"
        );
    }
}
