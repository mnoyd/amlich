use crate::{
    almanac::tu_menh::{compute_kua, KuaResult},
    bazi::{
        analyze_bazi_chart, build_bazi_chart, BaziAnalysisReport, BaziChart, BaziComputedMetrics,
        BaziInput, ElementDistribution,
    },
    interaction::{
        day_person::compute_day_person_matrix,
        direction_merge::compute_direction_merge,
        personal_hour::compute_personal_hour_matrix,
        types::{DayPersonMatrix, DirectionMergeMatrix, PersonalHourMatrix},
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

/// Precomputed per-request personal facts (chart, distributions, gates,
/// matrices). Built once per `(snapshot, personal_input)` pair and reused by
/// the fact-node projection, the evaluator's suggestion lookups, and the
/// semantic-graph merge so the consuming paths do not each recompute the
/// Bazi chart, the element distribution, the Kua, or the day-person matrix.
/// See REPAIR-PLAN.md P2 (`amlich-mwbp.8` finding A-R11).
pub struct PersonalAssessmentFacts {
    pub chart: BaziChart,
    pub analysis: BaziAnalysisReport,
    pub element_distribution: ElementDistribution,
    pub metrics: BaziComputedMetrics,
    pub kua: Option<KuaResult>,
    pub day_person_matrix: DayPersonMatrix,
    pub personal_hour_matrix: Option<PersonalHourMatrix>,
    pub direction_merge_matrix: Option<DirectionMergeMatrix>,
}

impl PersonalAssessmentFacts {
    pub fn build(
        personal: &PersonalReasoningInput,
        snapshot: &DaySnapshot,
    ) -> Result<Self, String> {
        let chart = build_bazi_chart(personal.to_bazi_input())?;
        let analysis = analyze_bazi_chart(&chart);
        // Reuse the element distribution that `analyze_bazi_chart` already
        // computed so the per-request request path does not redundantly
        // re-walk the chart pillars (amlich-mwbp.8 P2 finding A-R11).
        let element_distribution = analysis.element_distribution.clone();
        let metrics = crate::bazi::build_metrics_from_analysis(
            &chart,
            &analysis,
            None,
            &crate::bazi::default_bazi_scoring_matrix_set(),
        );
        let kua = personal
            .birth
            .gender
            .map(|gender| compute_kua(personal.birth.year, gender));
        let day_person_matrix = compute_day_person_matrix(&snapshot.context.canchi.day, &chart);
        let personal_hour_matrix = compute_personal_hour_matrix(
            &snapshot.context.canchi.day,
            &chart,
            &element_distribution,
        );
        let direction_merge_matrix = kua.as_ref().map(|kua_result| {
            compute_direction_merge(
                &snapshot.context.canchi.day,
                &snapshot.day_fortune.travel.tai_than,
                &snapshot.day_fortune.travel.hy_than,
                kua_result,
            )
        });
        Ok(Self {
            chart,
            analysis,
            element_distribution,
            metrics,
            kua,
            day_person_matrix,
            personal_hour_matrix,
            direction_merge_matrix,
        })
    }
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

    /// Project precomputed [`PersonalAssessmentFacts`] into user-facing
    /// [`PersonalFactNode`]s. Preferred over the snapshot-based variant —
    /// call sites that already have facts (graph projection, evaluator) must
    /// use this method so each fact is built exactly once per request.
    pub fn build_fact_nodes_from_facts(
        &self,
        facts: &PersonalAssessmentFacts,
    ) -> Vec<PersonalFactNode> {
        let mut nodes = vec![PersonalFactNode {
            id: "fact.personal.day_person_matrix".to_string(),
            summary_vi: summarize_day_person_matrix(&facts.day_person_matrix),
            severity: None,
            evidence: vec![interaction_evidence(
                "interaction.day_person.compute_day_person_matrix",
                "day_person_matrix",
            )],
        }];

        if let Some(personal_hour) = &facts.personal_hour_matrix {
            nodes.push(PersonalFactNode {
                id: "fact.personal.personal_hour_matrix".to_string(),
                summary_vi: summarize_personal_hour_matrix(personal_hour),
                severity: Some(personal_hour.hours.len().to_string()),
                evidence: vec![interaction_evidence(
                    "interaction.personal_hour.compute_personal_hour_matrix",
                    "personal_hour_matrix",
                )],
            });
        }

        if let Some(direction_merge) = &facts.direction_merge_matrix {
            let kua = facts
                .kua
                .as_ref()
                .expect("direction_merge_matrix requires kua (gated in build)");
            nodes.push(PersonalFactNode {
                id: "fact.personal.direction_merge".to_string(),
                summary_vi: summarize_direction_merge(direction_merge),
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
                facts.chart.day_master.full,
                facts.metrics.core_metrics.day_master_strength_label,
                facts
                    .metrics
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

        nodes
    }

    /// Backward-compatible snapshot-based variant: builds facts on demand.
    /// Prefer [`PersonalReasoningInput::build_fact_nodes_from_facts`] when
    /// facts are already available so the chart/element-distribution/Kua are
    /// not recomputed.
    pub fn build_fact_nodes(
        &self,
        snapshot: &DaySnapshot,
    ) -> Result<Vec<PersonalFactNode>, String> {
        let facts = PersonalAssessmentFacts::build(self, snapshot)?;
        Ok(self.build_fact_nodes_from_facts(&facts))
    }

    pub fn suggested_hours_from_facts(&self, facts: &PersonalAssessmentFacts) -> Vec<String> {
        let Some(matrix) = &facts.personal_hour_matrix else {
            return Vec::new();
        };
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
    }

    pub fn suggested_hours(&self, snapshot: &DaySnapshot) -> Vec<String> {
        let Ok(facts) = PersonalAssessmentFacts::build(self, snapshot) else {
            return Vec::new();
        };
        self.suggested_hours_from_facts(&facts)
    }

    pub fn suggested_directions_from_facts(&self, facts: &PersonalAssessmentFacts) -> Vec<String> {
        let Some(matrix) = &facts.direction_merge_matrix else {
            return Vec::new();
        };
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

    pub fn suggested_directions(&self, snapshot: &DaySnapshot) -> Vec<String> {
        let Ok(facts) = PersonalAssessmentFacts::build(self, snapshot) else {
            return Vec::new();
        };
        self.suggested_directions_from_facts(&facts)
    }

    pub(crate) fn to_bazi_input(&self) -> BaziInput {
        // Time-known signal flows from BirthInput's `Option<u8>` directly into
        // BaziInput's `time_known` flag. Real midnight births (Some(0)) are
        // preserved as `time_known: true`; absent time (None) becomes `false`
        // and the hour/minute scalars hold the legacy 0/0 sentinel value for
        // compatibility only. See REPAIR-PLAN.md P0.1.
        let time_known = self.birth.hour.is_some() && self.birth.minute.is_some();
        BaziInput {
            day: self.birth.day,
            month: self.birth.month,
            year: self.birth.year,
            hour: self.birth.hour.unwrap_or(0),
            minute: self.birth.minute.unwrap_or(0),
            time_known,
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
