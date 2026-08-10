//! Direction assessment is a separate ordering of the eight compass directions.
//!
//! It deliberately consumes day, personal, constraint, and overlay facts without
//! feeding them back into either `PersonalDayAssessment` or `HourRankingPolicy`.

use serde::{Deserialize, Serialize};

use crate::{
    advisory::ConsultationIntent,
    almanac::tu_menh::{compute_kua, Direction, KuaResult},
    assessment::{AvailabilityState, ContributionPolarity, SourceEvidence},
    canchi::get_year_canchi,
    lunar::convert_solar_to_lunar,
    reasoning::{
        direction_composite::{
            build_direction_cross_link_date, build_direction_cross_link_personal, DIRECTION_ORDER,
        },
        DecisionConfidence,
    },
    sources::{SOURCE_HUYEN_KHONG, SOURCE_KHCBPPT, SOURCE_VN_FOLK},
    BirthProfile, DaySnapshot,
};

pub const DIRECTION_ASSESSMENT_POLICY_ID: &str = "direction-assessment";
pub const DIRECTION_ASSESSMENT_POLICY_VERSION: &str = "v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DirectionAssessmentAxis {
    TravelDeities,
    KuaCompatibility,
    DirectionalConstraints,
    FlyingStarOverlay,
}

impl DirectionAssessmentAxis {
    pub const ALL: [Self; 4] = [
        Self::TravelDeities,
        Self::KuaCompatibility,
        Self::DirectionalConstraints,
        Self::FlyingStarOverlay,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TravelDeities => "travel_deities",
            Self::KuaCompatibility => "kua_compatibility",
            Self::DirectionalConstraints => "directional_constraints",
            Self::FlyingStarOverlay => "flying_star_overlay",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectionAssessmentAxisOutcome {
    pub axis: DirectionAssessmentAxis,
    pub score: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unavailable_reason: Option<String>,
}

impl DirectionAssessmentAxisOutcome {
    fn score(axis: DirectionAssessmentAxis, score: f32) -> Self {
        Self {
            axis,
            score: Some(score.clamp(0.0, 1.0)),
            unavailable_reason: None,
        }
    }

    fn unavailable(axis: DirectionAssessmentAxis, reason: impl Into<String>) -> Self {
        Self {
            axis,
            score: None,
            unavailable_reason: Some(reason.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectionAssessmentAxes {
    pub travel_deities: DirectionAssessmentAxisOutcome,
    pub kua_compatibility: DirectionAssessmentAxisOutcome,
    pub directional_constraints: DirectionAssessmentAxisOutcome,
    pub flying_star_overlay: DirectionAssessmentAxisOutcome,
}

impl DirectionAssessmentAxes {
    pub fn iter(&self) -> impl Iterator<Item = &DirectionAssessmentAxisOutcome> {
        [
            &self.travel_deities,
            &self.kua_compatibility,
            &self.directional_constraints,
            &self.flying_star_overlay,
        ]
        .into_iter()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectionAssessmentContribution {
    pub contribution_id: String,
    pub axis: DirectionAssessmentAxis,
    pub polarity: ContributionPolarity,
    pub strength: f32,
    pub availability: AvailabilityState,
    pub source_evidence: SourceEvidence,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirectionAssessmentWarning {
    pub code: String,
    pub message_vi: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectionAssessmentEntry {
    pub direction: Direction,
    pub rank_score: f32,
    pub axes: DirectionAssessmentAxes,
    pub contributions: Vec<DirectionAssessmentContribution>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<DirectionAssessmentWarning>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectionAssessment {
    pub policy_id: String,
    pub policy_version: String,
    pub ruleset_id: String,
    pub ruleset_version: String,
    pub intent: ConsultationIntent,
    pub confidence: DecisionConfidence,
    pub entries: Vec<DirectionAssessmentEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unavailable_sections: Vec<DirectionAssessmentWarning>,
}

#[derive(Debug, Clone, Default)]
pub struct DirectionAssessmentPolicy;

impl DirectionAssessmentPolicy {
    pub fn assess(
        snapshot: &DaySnapshot,
        profile: &BirthProfile,
        intent: ConsultationIntent,
    ) -> DirectionAssessment {
        let kua = profile
            .gender
            .map(|gender| compute_kua(profile.year, gender));
        Self::assess_with_kua(snapshot, profile, intent, kua.as_ref())
    }

    pub fn assess_with_kua(
        snapshot: &DaySnapshot,
        profile: &BirthProfile,
        intent: ConsultationIntent,
        kua: Option<&KuaResult>,
    ) -> DirectionAssessment {
        let uses_travel_deities = matches!(
            intent,
            ConsultationIntent::Travel | ConsultationIntent::MovingHouse
        );
        let uses_kua = !matches!(
            intent,
            ConsultationIntent::Burial | ConsultationIntent::Prayer | ConsultationIntent::Medical
        );
        let uses_flying_stars = matches!(
            intent,
            ConsultationIntent::Travel
                | ConsultationIntent::MovingHouse
                | ConsultationIntent::OpeningBusiness
                | ConsultationIntent::Renovation
        );
        let birth_lunar_year =
            convert_solar_to_lunar(profile.day, profile.month, profile.year, profile.timezone).year;
        let birth_chi = get_year_canchi(birth_lunar_year).chi_index;
        let cross_link = if kua.is_some() {
            build_direction_cross_link_personal(snapshot, birth_chi).ok()
        } else {
            build_direction_cross_link_date(snapshot).ok()
        };

        let mut unavailable_sections = Vec::new();
        if uses_kua && kua.is_none() {
            unavailable_sections.push(warning(
                "kua_unavailable",
                "Cần năm sinh và giới tính để xét hợp hướng Cung mệnh.",
            ));
        }
        if uses_flying_stars && profile.location_name.is_none() {
            unavailable_sections.push(warning(
                "location_unavailable",
                "Chưa có địa điểm; không áp dụng lớp Phi Tinh nhạy vị trí.",
            ));
        }
        if cross_link.is_none() {
            unavailable_sections.push(warning(
                "directional_overlay_unavailable",
                "Không có dữ liệu Tam Sát, Thái Tuế và Phi Tinh cho ngày này.",
            ));
        }

        let entries = DIRECTION_ORDER
            .into_iter()
            .map(|direction| {
                let mut contributions = Vec::new();
                let travel_deities = if uses_travel_deities {
                    let mut score = 0.5;
                    let direction_name = direction.as_vn_str();
                    for (id, deity, value) in [
                        (
                            "tai_than",
                            "Tài Thần",
                            &snapshot.day_fortune.travel.tai_than,
                        ),
                        ("hy_than", "Hỷ Thần", &snapshot.day_fortune.travel.hy_than),
                        (
                            "xuat_hanh",
                            "Xuất hành",
                            &snapshot.day_fortune.travel.xuat_hanh_huong,
                        ),
                    ] {
                        if value == direction_name {
                            score += 0.15;
                            contributions.push(contribution(
                                format!("direction.travel_deity.{id}.{}", direction_name),
                                DirectionAssessmentAxis::TravelDeities,
                                ContributionPolarity::Favorable,
                                0.6,
                                snapshot,
                                "travel_deity",
                                Some(deity.to_string()),
                            ));
                        }
                    }
                    DirectionAssessmentAxisOutcome::score(
                        DirectionAssessmentAxis::TravelDeities,
                        score,
                    )
                } else {
                    DirectionAssessmentAxisOutcome::unavailable(
                        DirectionAssessmentAxis::TravelDeities,
                        "Travel deities apply only to travel and moving-house intent",
                    )
                };

                let kua_compatibility = if uses_kua {
                    match kua {
                        Some(kua) if kua.favorable_directions.contains(&direction) => {
                            contributions.push(contribution(
                                format!("direction.kua.favorable.{}", direction.as_vn_str()),
                                DirectionAssessmentAxis::KuaCompatibility,
                                ContributionPolarity::Favorable,
                                0.8,
                                snapshot,
                                "kua_compatibility",
                                Some(format!("kua={}", kua.kua)),
                            ));
                            DirectionAssessmentAxisOutcome::score(
                                DirectionAssessmentAxis::KuaCompatibility,
                                1.0,
                            )
                        }
                        Some(kua) if kua.unfavorable_directions.contains(&direction) => {
                            contributions.push(contribution(
                                format!("direction.kua.unfavorable.{}", direction.as_vn_str()),
                                DirectionAssessmentAxis::KuaCompatibility,
                                ContributionPolarity::Avoid,
                                0.8,
                                snapshot,
                                "kua_compatibility",
                                Some(format!("kua={}", kua.kua)),
                            ));
                            DirectionAssessmentAxisOutcome::score(
                                DirectionAssessmentAxis::KuaCompatibility,
                                0.0,
                            )
                        }
                        Some(_) => DirectionAssessmentAxisOutcome::score(
                            DirectionAssessmentAxis::KuaCompatibility,
                            0.5,
                        ),
                        None => DirectionAssessmentAxisOutcome::unavailable(
                            DirectionAssessmentAxis::KuaCompatibility,
                            "Gender is required to calculate Kua compatibility",
                        ),
                    }
                } else {
                    DirectionAssessmentAxisOutcome::unavailable(
                        DirectionAssessmentAxis::KuaCompatibility,
                        "Kua compatibility does not apply to this intent",
                    )
                };

                let cell = cross_link
                    .as_ref()
                    .and_then(|cross| cross.cells.iter().find(|cell| cell.direction == direction));
                let directional_constraints = match cell {
                    Some(cell) => {
                        let taboo = cell.khcbppt.as_ref();
                        let mut severity = 0.5;
                        if let Some(taboo) = taboo {
                            if taboo.thai_tue.is_some() {
                                severity -= 0.25;
                                contributions.push(contribution(
                                    format!("direction.thai_tue.{}", direction.as_vn_str()),
                                    DirectionAssessmentAxis::DirectionalConstraints,
                                    ContributionPolarity::Avoid,
                                    0.8,
                                    snapshot,
                                    "thai_tue_direction",
                                    None,
                                ));
                            }
                            if !taboo.tam_sat_branches.is_empty() {
                                severity -= 0.25;
                                contributions.push(contribution(
                                    format!("direction.tam_sat.{}", direction.as_vn_str()),
                                    DirectionAssessmentAxis::DirectionalConstraints,
                                    ContributionPolarity::Avoid,
                                    0.8,
                                    snapshot,
                                    "tam_sat_direction",
                                    Some(taboo.tam_sat_branches.join(",")),
                                ));
                            }
                            if taboo.sat_phuong_direction.is_some() {
                                severity -= 0.15;
                                contributions.push(contribution(
                                    format!("direction.sat_phuong.{}", direction.as_vn_str()),
                                    DirectionAssessmentAxis::DirectionalConstraints,
                                    ContributionPolarity::Avoid,
                                    0.6,
                                    snapshot,
                                    "sat_phuong",
                                    None,
                                ));
                            }
                        }
                        DirectionAssessmentAxisOutcome::score(
                            DirectionAssessmentAxis::DirectionalConstraints,
                            severity,
                        )
                    }
                    None => DirectionAssessmentAxisOutcome::unavailable(
                        DirectionAssessmentAxis::DirectionalConstraints,
                        "Directional constraint overlay is unavailable",
                    ),
                };

                let flying_star_overlay = if uses_flying_stars && profile.location_name.is_some() {
                    match cell.and_then(|cell| cell.huyen_khong.as_ref()) {
                        Some(overlay) => {
                            let caution = overlay.safety_hint_vi.is_some();
                            contributions.push(contribution(
                                format!("direction.flying_star.{}", direction.as_vn_str()),
                                DirectionAssessmentAxis::FlyingStarOverlay,
                                if caution {
                                    ContributionPolarity::Avoid
                                } else {
                                    ContributionPolarity::Favorable
                                },
                                0.4,
                                snapshot,
                                "flying_star_overlay",
                                overlay.safety_hint_vi.clone(),
                            ));
                            DirectionAssessmentAxisOutcome::score(
                                DirectionAssessmentAxis::FlyingStarOverlay,
                                if caution { 0.25 } else { 0.75 },
                            )
                        }
                        None => DirectionAssessmentAxisOutcome::unavailable(
                            DirectionAssessmentAxis::FlyingStarOverlay,
                            "Flying-star overlay is unavailable",
                        ),
                    }
                } else if !uses_flying_stars {
                    DirectionAssessmentAxisOutcome::unavailable(
                        DirectionAssessmentAxis::FlyingStarOverlay,
                        "Flying-star overlay does not apply to this intent",
                    )
                } else {
                    DirectionAssessmentAxisOutcome::unavailable(
                        DirectionAssessmentAxis::FlyingStarOverlay,
                        "Location is required for the location-sensitive flying-star overlay",
                    )
                };

                let axes = DirectionAssessmentAxes {
                    travel_deities,
                    kua_compatibility,
                    directional_constraints,
                    flying_star_overlay,
                };
                let available: Vec<f32> = axes.iter().filter_map(|axis| axis.score).collect();
                let rank_score = if available.is_empty() {
                    0.5
                } else {
                    available.iter().sum::<f32>() / available.len() as f32
                };
                let mut warnings = Vec::new();
                if axes
                    .directional_constraints
                    .score
                    .is_some_and(|score| score < 0.5)
                {
                    warnings.push(warning(
                        "directional_constraint",
                        "Hướng này có ràng buộc Thái Tuế, Tam Sát hoặc Sát Phương.",
                    ));
                }
                DirectionAssessmentEntry {
                    direction,
                    rank_score,
                    axes,
                    contributions,
                    warnings,
                }
            })
            .collect::<Vec<_>>();

        let available_axis_count = entries
            .first()
            .map(|entry| {
                entry
                    .axes
                    .iter()
                    .filter(|axis| axis.score.is_some())
                    .count()
            })
            .unwrap_or(0);
        let confidence = match available_axis_count {
            0 | 1 => DecisionConfidence::Low,
            2 | 3 => DecisionConfidence::Medium,
            _ => DecisionConfidence::High,
        };
        DirectionAssessment {
            policy_id: DIRECTION_ASSESSMENT_POLICY_ID.to_string(),
            policy_version: DIRECTION_ASSESSMENT_POLICY_VERSION.to_string(),
            ruleset_id: snapshot.ruleset_id.clone(),
            ruleset_version: snapshot.ruleset_version.clone(),
            intent,
            confidence,
            entries,
            unavailable_sections,
        }
    }
}

fn contribution(
    contribution_id: String,
    axis: DirectionAssessmentAxis,
    polarity: ContributionPolarity,
    strength: f32,
    snapshot: &DaySnapshot,
    method: &str,
    note: Option<String>,
) -> DirectionAssessmentContribution {
    DirectionAssessmentContribution {
        contribution_id,
        axis,
        polarity,
        strength,
        availability: AvailabilityState::Complete,
        source_evidence: SourceEvidence {
            source_family: match axis {
                DirectionAssessmentAxis::KuaCompatibility => "interaction",
                _ => "almanac_rule",
            }
            .to_string(),
            source_id: match axis {
                DirectionAssessmentAxis::KuaCompatibility => SOURCE_VN_FOLK,
                DirectionAssessmentAxis::FlyingStarOverlay => SOURCE_HUYEN_KHONG,
                DirectionAssessmentAxis::TravelDeities
                | DirectionAssessmentAxis::DirectionalConstraints => SOURCE_KHCBPPT,
            }
            .to_string(),
            method: method.to_string(),
            profile: snapshot.profile.clone(),
            note: None,
        },
        note,
    }
}

fn warning(code: &str, message_vi: &str) -> DirectionAssessmentWarning {
    DirectionAssessmentWarning {
        code: code.to_string(),
        message_vi: message_vi.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{almanac::tu_menh::Gender, calculate_day_snapshot};

    fn profile(gender: Option<Gender>, location: Option<&str>) -> BirthProfile {
        BirthProfile {
            day: 1,
            month: 1,
            year: 1990,
            time: None,
            timezone: 7.0,
            longitude: None,
            use_solar_time: false,
            gender,
            location_name: location.map(str::to_string),
        }
    }

    #[test]
    fn direction_assessment_is_separate_and_deduplicates_constraint_facts() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let result = DirectionAssessmentPolicy::assess(
            &snapshot,
            &profile(Some(Gender::Male), Some("Hà Nội")),
            ConsultationIntent::Travel,
        );
        assert_eq!(result.entries.len(), 8);
        for entry in result.entries {
            let mut ids = entry
                .contributions
                .iter()
                .map(|item| &item.contribution_id)
                .collect::<Vec<_>>();
            ids.sort();
            ids.dedup();
            assert_eq!(ids.len(), entry.contributions.len());
        }
    }

    #[test]
    fn missing_gender_and_location_are_unavailable_not_neutral() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let result = DirectionAssessmentPolicy::assess(
            &snapshot,
            &profile(None, None),
            ConsultationIntent::Travel,
        );
        assert!(result
            .unavailable_sections
            .iter()
            .any(|item| item.code == "kua_unavailable"));
        assert!(result
            .unavailable_sections
            .iter()
            .any(|item| item.code == "location_unavailable"));
        assert!(result
            .entries
            .iter()
            .all(|entry| entry.axes.kua_compatibility.score.is_none()));
        assert!(result
            .entries
            .iter()
            .all(|entry| entry.axes.flying_star_overlay.score.is_none()));
    }

    #[test]
    fn non_directional_intent_does_not_apply_travel_deities() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let result = DirectionAssessmentPolicy::assess(
            &snapshot,
            &profile(Some(Gender::Female), Some("Hà Nội")),
            ConsultationIntent::Wedding,
        );
        assert!(result
            .entries
            .iter()
            .all(|entry| entry.axes.travel_deities.score.is_none()));
    }

    #[test]
    fn overlapping_tam_sat_and_sat_phuong_remain_separate_facts() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let result = DirectionAssessmentPolicy::assess(
            &snapshot,
            &profile(Some(Gender::Male), Some("Hà Nội")),
            ConsultationIntent::Travel,
        );
        let south = result
            .entries
            .iter()
            .find(|entry| entry.direction == Direction::South)
            .expect("south direction");
        assert!(south
            .contributions
            .iter()
            .any(|item| item.contribution_id == "direction.tam_sat.Nam"));
        assert!(south
            .contributions
            .iter()
            .any(|item| item.contribution_id == "direction.sat_phuong.Nam"));
        assert!(south
            .warnings
            .iter()
            .any(|item| item.code == "directional_constraint"));
    }
}
