use crate::{
    almanac::tu_menh::compute_kua,
    bazi::{build_bazi_chart, compute_bazi_metrics, compute_element_distribution, BaziInput},
    interaction::{
        day_person::compute_day_person_matrix, direction_merge::compute_direction_merge,
        personal_hour::compute_personal_hour_matrix,
    },
    BirthInput, ConsultationIntent, DaySnapshot,
};

use super::{ReasoningEvidenceEnvelope, ReasoningEvidenceSourceFamily};

pub struct PersonalFactNode {
    pub id: String,
    pub summary_vi: String,
    pub severity: Option<String>,
    pub evidence: Vec<ReasoningEvidenceEnvelope>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PersonalReasoningInput {
    pub birth: BirthInput,
    pub intent: ConsultationIntent,
}

impl PersonalReasoningInput {
    pub fn from_birth(birth: BirthInput, intent: ConsultationIntent) -> Self {
        Self { birth, intent }
    }

    pub fn build_fact_nodes(
        &self,
        snapshot: &DaySnapshot,
    ) -> Result<Vec<PersonalFactNode>, String> {
        let chart = build_bazi_chart(self.to_bazi_input())?;
        let analysis = compute_element_distribution(&chart);
        let metrics = compute_bazi_metrics(&chart, None);
        let mut nodes = vec![PersonalFactNode {
            id: "fact.personal.day_person_matrix".to_string(),
            summary_vi: summarize_day_person_matrix(&compute_day_person_matrix(
                &snapshot.context.canchi.day,
                &chart,
            )),
            severity: None,
            evidence: vec![interaction_evidence(
                "interaction.day_person.compute_day_person_matrix",
                "day_person_matrix",
            )],
        }];

        if let Some(personal_hour) =
            compute_personal_hour_matrix(&snapshot.context.canchi.day, &chart, &analysis)
        {
            nodes.push(PersonalFactNode {
                id: "fact.personal.personal_hour_matrix".to_string(),
                summary_vi: summarize_personal_hour_matrix(&personal_hour),
                severity: Some(personal_hour.hours.len().to_string()),
                evidence: vec![interaction_evidence(
                    "interaction.personal_hour.compute_personal_hour_matrix",
                    "personal_hour_matrix",
                )],
            });
        }

        if let Some(gender) = self.birth.gender {
            let kua = compute_kua(self.birth.year, gender);
            let direction_merge = compute_direction_merge(
                &snapshot.context.canchi.day,
                &snapshot.day_fortune.travel.tai_than,
                &snapshot.day_fortune.travel.hy_than,
                &kua,
            );
            nodes.push(PersonalFactNode {
                id: "fact.personal.direction_merge".to_string(),
                summary_vi: summarize_direction_merge(&direction_merge),
                severity: Some(kua.kua.to_string()),
                evidence: vec![interaction_evidence(
                    "interaction.direction_merge.compute_direction_merge",
                    "direction_merge",
                )],
            });
        }

        nodes.push(PersonalFactNode {
            id: "fact.personal.bazi_profile".to_string(),
            summary_vi: format!(
                "Nhật chủ {}, mạnh yếu {}, hành trội {}",
                chart.day_master.full,
                metrics.core_metrics.day_master_strength_label,
                metrics
                    .structure_metrics
                    .dominant_elements
                    .iter()
                    .take(2)
                    .map(|element| format!("{element:?}").to_lowercase())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            severity: None,
            evidence: vec![bazi_evidence(
                "bazi.compute_bazi_metrics",
                "profile_analysis",
            )],
        });

        Ok(nodes)
    }

    pub fn suggested_hours(&self, snapshot: &DaySnapshot) -> Vec<String> {
        let Ok(chart) = build_bazi_chart(self.to_bazi_input()) else {
            return Vec::new();
        };
        let analysis = compute_element_distribution(&chart);

        compute_personal_hour_matrix(&snapshot.context.canchi.day, &chart, &analysis)
            .map(|matrix| {
                matrix
                    .hours
                    .iter()
                    .filter(|hour| hour.is_hoang_dao)
                    .filter(|hour| hour.score >= 70)
                    .take(3)
                    .map(|hour| {
                        format!(
                            "Nếu vẫn tiến hành, ưu tiên giờ {} ({}) hợp cá nhân hơn (điểm {})",
                            hour.chi, hour.time_range, hour.score
                        )
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn suggested_directions(&self, snapshot: &DaySnapshot) -> Vec<String> {
        let Some(gender) = self.birth.gender else {
            return Vec::new();
        };

        let kua = compute_kua(self.birth.year, gender);
        let matrix = compute_direction_merge(
            &snapshot.context.canchi.day,
            &snapshot.day_fortune.travel.tai_than,
            &snapshot.day_fortune.travel.hy_than,
            &kua,
        );

        matrix
            .entries
            .iter()
            .filter(|entry| entry.net_score > 0)
            .take(3)
            .map(|entry| {
                format!(
                    "Nếu vẫn tiến hành, ưu tiên hướng {} theo tổng hợp Kua/ngày (điểm ròng {})",
                    entry.direction, entry.net_score
                )
            })
            .collect()
    }

    pub(crate) fn to_bazi_input(&self) -> BaziInput {
        BaziInput {
            day: self.birth.day,
            month: self.birth.month,
            year: self.birth.year,
            hour: self.birth.hour.unwrap_or(0),
            minute: self.birth.minute.unwrap_or(0),
            timezone: self.birth.timezone,
            longitude: None,
            use_solar_time: false,
            gender: self.birth.gender,
        }
    }
}

fn interaction_evidence(source_id: &str, note: &str) -> ReasoningEvidenceEnvelope {
    ReasoningEvidenceEnvelope {
        source_family: ReasoningEvidenceSourceFamily::Interaction,
        source_id: source_id.to_string(),
        method: "computed_matrix".to_string(),
        note: Some(note.to_string()),
    }
}

fn bazi_evidence(source_id: &str, note: &str) -> ReasoningEvidenceEnvelope {
    ReasoningEvidenceEnvelope {
        source_family: ReasoningEvidenceSourceFamily::Bazi,
        source_id: source_id.to_string(),
        method: "profile_analysis".to_string(),
        note: Some(note.to_string()),
    }
}

fn summarize_day_person_matrix(matrix: &crate::interaction::types::DayPersonMatrix) -> String {
    let harmonious_pillars = matrix
        .pillars
        .iter()
        .filter(|pillar| pillar.branch_relation.has_harmony())
        .count();
    let conflicting_pillars = matrix
        .pillars
        .iter()
        .filter(|pillar| pillar.branch_relation.has_conflict())
        .count();

    format!(
        "Ngày {} so với nhật chủ {}: {} trụ hợp, {} trụ xung/khắc",
        matrix.day_canchi, matrix.day_master, harmonious_pillars, conflicting_pillars
    )
}

fn summarize_personal_hour_matrix(
    matrix: &crate::interaction::types::PersonalHourMatrix,
) -> String {
    let best_hour = matrix.hours.iter().max_by_key(|hour| hour.score);

    match best_hour {
        Some(hour) => format!(
            "Giờ hợp cá nhân nổi bật: {} ({}, điểm {})",
            hour.chi, hour.time_range, hour.score
        ),
        None => "Chưa có giờ hợp cá nhân nổi bật".to_string(),
    }
}

fn summarize_direction_merge(matrix: &crate::interaction::types::DirectionMergeMatrix) -> String {
    let best_direction = matrix.entries.iter().max_by_key(|entry| entry.net_score);

    match best_direction {
        Some(direction) => format!(
            "Hướng hợp theo Kua {}: {} (điểm ròng {})",
            matrix.kua_number, direction.direction, direction.net_score
        ),
        None => format!("Không có hướng nổi bật theo Kua {}", matrix.kua_number),
    }
}
