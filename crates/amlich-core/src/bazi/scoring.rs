use serde::{Deserialize, Serialize};

use crate::{
    almanac::types::{FiveElement, ThapThanLabel},
    bazi::{
        analysis::{
            analyze_bazi_chart, BaziAnalysisReport, ChartInteractionKind, DayMasterStrengthLabel,
            ElementDistribution, TenGodDistribution,
        },
        timing::BaziTimingReport,
        types::BaziChart,
    },
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElementRelationMatrix {
    pub moc: ElementRelationVector,
    pub hoa: ElementRelationVector,
    pub tho: ElementRelationVector,
    pub kim: ElementRelationVector,
    pub thuy: ElementRelationVector,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElementRelationVector {
    pub moc: f32,
    pub hoa: f32,
    pub tho: f32,
    pub kim: f32,
    pub thuy: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeasonStrengthMatrix {
    pub moc: BranchStrengthProfile,
    pub hoa: BranchStrengthProfile,
    pub tho: BranchStrengthProfile,
    pub kim: BranchStrengthProfile,
    pub thuy: BranchStrengthProfile,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchStrengthProfile {
    pub ty: f32,
    pub suu: f32,
    pub dan: f32,
    pub mao: f32,
    pub thin: f32,
    pub ty2: f32,
    pub ngo: f32,
    pub mui: f32,
    pub than: f32,
    pub dau: f32,
    pub tuat: f32,
    pub hoi: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VisibilityWeightMatrix {
    pub visible_stem: f32,
    pub branch_main_hidden: f32,
    pub branch_middle_hidden: f32,
    pub branch_residual_hidden: f32,
    pub month_branch_bonus: f32,
    pub day_branch_bonus: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InteractionImpactMatrix {
    pub branch_clash: f32,
    pub branch_harmony: f32,
    pub branch_harm: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TenGodContextMatrix {
    pub weak_dm: TenGodWeightProfile,
    pub balanced_dm: TenGodWeightProfile,
    pub strong_dm: TenGodWeightProfile,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TenGodWeightProfile {
    pub ty_kien: f32,
    pub kiep_tai: f32,
    pub thuc_than: f32,
    pub thuong_quan: f32,
    pub chinh_tai: f32,
    pub thien_tai: f32,
    pub chinh_quan: f32,
    pub that_sat: f32,
    pub chinh_an: f32,
    pub thien_an: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomainMappingMatrix {
    pub career: DomainWeightProfile,
    pub wealth: DomainWeightProfile,
    pub relationship: DomainWeightProfile,
    pub health: DomainWeightProfile,
    pub timing: DomainWeightProfile,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomainWeightProfile {
    pub ty_kien: f32,
    pub kiep_tai: f32,
    pub thuc_than: f32,
    pub thuong_quan: f32,
    pub chinh_tai: f32,
    pub thien_tai: f32,
    pub chinh_quan: f32,
    pub that_sat: f32,
    pub chinh_an: f32,
    pub thien_an: f32,
    pub branch_clash: f32,
    pub branch_harmony: f32,
    pub branch_harm: f32,
    pub element_imbalance: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaziScoringMatrixSet {
    pub element_relations: ElementRelationMatrix,
    pub season_strength: SeasonStrengthMatrix,
    pub visibility_weights: VisibilityWeightMatrix,
    pub interaction_impacts: InteractionImpactMatrix,
    pub ten_god_context: TenGodContextMatrix,
    pub domain_mapping: DomainMappingMatrix,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaziComputedMetrics {
    pub core_metrics: BaziCoreMetrics,
    pub structure_metrics: BaziStructureMetrics,
    pub domain_scores: BaziDomainScores,
    pub timing_metrics: BaziTimingMetrics,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaziCoreMetrics {
    pub day_master_strength_score: i32,
    pub day_master_strength_label: String,
    pub season_support_score: f32,
    pub same_element_score: u16,
    pub resource_support_score: u16,
    pub drain_pressure_score: u16,
    pub control_pressure_score: u16,
    pub element_balance_score: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaziStructureMetrics {
    pub dominant_elements: Vec<FiveElement>,
    pub weak_elements: Vec<FiveElement>,
    pub dominant_ten_gods: Vec<String>,
    pub interaction_score: f32,
    #[serde(default)]
    pub notable_interactions: Vec<BaziInteractionMetric>,
    pub confidence: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaziInteractionMetric {
    pub kind: String,
    pub participants: Vec<String>,
    pub impact: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaziDomainScores {
    pub career: BaziDomainScore,
    pub wealth: BaziDomainScore,
    pub relationship: BaziDomainScore,
    pub health: BaziDomainScore,
    pub timing: BaziDomainScore,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaziDomainScore {
    pub score: u8,
    pub label: String,
    pub confidence: f32,
    pub evidence_level: String,
    #[serde(default)]
    pub contributors: Vec<BaziScoreContributor>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaziScoreContributor {
    pub signal: String,
    pub delta: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaziTimingMetrics {
    pub current_dai_van_alignment: Option<f32>,
    pub annual_alignment: Option<f32>,
    #[serde(default)]
    pub monthly_windows: Vec<BaziTimingWindowScore>,
    #[serde(default)]
    pub activation_summary: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaziTimingWindowScore {
    pub month: i32,
    pub score: f32,
    pub label: String,
    pub confidence: f32,
}

impl Default for BaziScoringMatrixSet {
    fn default() -> Self {
        Self {
            element_relations: ElementRelationMatrix {
                moc: ElementRelationVector {
                    moc: 0.2,
                    hoa: 1.0,
                    tho: -0.6,
                    kim: -1.0,
                    thuy: 0.8,
                },
                hoa: ElementRelationVector {
                    moc: 0.8,
                    hoa: 0.2,
                    tho: 1.0,
                    kim: -0.6,
                    thuy: -1.0,
                },
                tho: ElementRelationVector {
                    moc: -1.0,
                    hoa: 0.8,
                    tho: 0.2,
                    kim: 1.0,
                    thuy: -0.6,
                },
                kim: ElementRelationVector {
                    moc: -0.6,
                    hoa: -1.0,
                    tho: 0.8,
                    kim: 0.2,
                    thuy: 1.0,
                },
                thuy: ElementRelationVector {
                    moc: 1.0,
                    hoa: -0.6,
                    tho: -1.0,
                    kim: 0.8,
                    thuy: 0.2,
                },
            },
            season_strength: SeasonStrengthMatrix {
                moc: BranchStrengthProfile {
                    ty: 0.2,
                    suu: 0.2,
                    dan: 1.0,
                    mao: 1.0,
                    thin: 0.6,
                    ty2: 0.3,
                    ngo: 0.2,
                    mui: 0.2,
                    than: 0.1,
                    dau: 0.1,
                    tuat: 0.1,
                    hoi: 0.5,
                },
                hoa: BranchStrengthProfile {
                    ty: 0.1,
                    suu: 0.1,
                    dan: 0.4,
                    mao: 0.5,
                    thin: 0.3,
                    ty2: 1.0,
                    ngo: 1.0,
                    mui: 0.6,
                    than: 0.2,
                    dau: 0.1,
                    tuat: 0.2,
                    hoi: 0.1,
                },
                tho: BranchStrengthProfile {
                    ty: 0.2,
                    suu: 0.8,
                    dan: 0.2,
                    mao: 0.2,
                    thin: 0.8,
                    ty2: 0.3,
                    ngo: 0.3,
                    mui: 0.8,
                    than: 0.2,
                    dau: 0.2,
                    tuat: 0.8,
                    hoi: 0.2,
                },
                kim: BranchStrengthProfile {
                    ty: 0.2,
                    suu: 0.3,
                    dan: 0.1,
                    mao: 0.1,
                    thin: 0.2,
                    ty2: 0.1,
                    ngo: 0.1,
                    mui: 0.2,
                    than: 1.0,
                    dau: 1.0,
                    tuat: 0.5,
                    hoi: 0.2,
                },
                thuy: BranchStrengthProfile {
                    ty: 1.0,
                    suu: 0.5,
                    dan: 0.2,
                    mao: 0.2,
                    thin: 0.3,
                    ty2: 0.1,
                    ngo: 0.1,
                    mui: 0.2,
                    than: 0.5,
                    dau: 0.5,
                    tuat: 0.2,
                    hoi: 1.0,
                },
            },
            visibility_weights: VisibilityWeightMatrix {
                visible_stem: 1.0,
                branch_main_hidden: 0.7,
                branch_middle_hidden: 0.4,
                branch_residual_hidden: 0.2,
                month_branch_bonus: 0.25,
                day_branch_bonus: 0.1,
            },
            interaction_impacts: InteractionImpactMatrix {
                branch_clash: -0.9,
                branch_harmony: 0.6,
                branch_harm: -0.5,
            },
            ten_god_context: TenGodContextMatrix {
                weak_dm: TenGodWeightProfile {
                    ty_kien: 0.8,
                    kiep_tai: 0.7,
                    thuc_than: -0.2,
                    thuong_quan: -0.4,
                    chinh_tai: -0.5,
                    thien_tai: -0.5,
                    chinh_quan: -0.6,
                    that_sat: -0.8,
                    chinh_an: 1.0,
                    thien_an: 0.9,
                },
                balanced_dm: TenGodWeightProfile {
                    ty_kien: 0.2,
                    kiep_tai: 0.1,
                    thuc_than: 0.5,
                    thuong_quan: 0.3,
                    chinh_tai: 0.5,
                    thien_tai: 0.4,
                    chinh_quan: 0.5,
                    that_sat: 0.2,
                    chinh_an: 0.4,
                    thien_an: 0.4,
                },
                strong_dm: TenGodWeightProfile {
                    ty_kien: -0.5,
                    kiep_tai: -0.6,
                    thuc_than: 0.8,
                    thuong_quan: 0.7,
                    chinh_tai: 0.8,
                    thien_tai: 0.7,
                    chinh_quan: 0.6,
                    that_sat: 0.3,
                    chinh_an: 0.1,
                    thien_an: 0.1,
                },
            },
            domain_mapping: DomainMappingMatrix {
                career: DomainWeightProfile {
                    ty_kien: 0.1,
                    kiep_tai: -0.1,
                    thuc_than: 0.5,
                    thuong_quan: 0.2,
                    chinh_tai: 0.3,
                    thien_tai: 0.2,
                    chinh_quan: 0.9,
                    that_sat: 0.4,
                    chinh_an: 0.8,
                    thien_an: 0.6,
                    branch_clash: -0.3,
                    branch_harmony: 0.2,
                    branch_harm: -0.2,
                    element_imbalance: -0.2,
                },
                wealth: DomainWeightProfile {
                    ty_kien: -0.2,
                    kiep_tai: -0.6,
                    thuc_than: 0.4,
                    thuong_quan: 0.5,
                    chinh_tai: 1.0,
                    thien_tai: 0.9,
                    chinh_quan: 0.2,
                    that_sat: 0.1,
                    chinh_an: 0.2,
                    thien_an: 0.1,
                    branch_clash: -0.2,
                    branch_harmony: 0.2,
                    branch_harm: -0.2,
                    element_imbalance: -0.1,
                },
                relationship: DomainWeightProfile {
                    ty_kien: 0.1,
                    kiep_tai: -0.2,
                    thuc_than: 0.1,
                    thuong_quan: -0.1,
                    chinh_tai: 0.4,
                    thien_tai: 0.2,
                    chinh_quan: 0.4,
                    that_sat: -0.1,
                    chinh_an: 0.1,
                    thien_an: 0.1,
                    branch_clash: -0.8,
                    branch_harmony: 0.6,
                    branch_harm: -0.4,
                    element_imbalance: -0.2,
                },
                health: DomainWeightProfile {
                    ty_kien: 0.0,
                    kiep_tai: -0.1,
                    thuc_than: 0.1,
                    thuong_quan: -0.1,
                    chinh_tai: 0.0,
                    thien_tai: 0.0,
                    chinh_quan: 0.1,
                    that_sat: -0.2,
                    chinh_an: 0.2,
                    thien_an: 0.2,
                    branch_clash: -0.3,
                    branch_harmony: 0.1,
                    branch_harm: -0.5,
                    element_imbalance: -0.9,
                },
                timing: DomainWeightProfile {
                    ty_kien: 0.1,
                    kiep_tai: -0.1,
                    thuc_than: 0.2,
                    thuong_quan: 0.1,
                    chinh_tai: 0.2,
                    thien_tai: 0.2,
                    chinh_quan: 0.2,
                    that_sat: -0.1,
                    chinh_an: 0.2,
                    thien_an: 0.2,
                    branch_clash: -0.5,
                    branch_harmony: 0.4,
                    branch_harm: -0.3,
                    element_imbalance: -0.2,
                },
            },
        }
    }
}

pub fn default_bazi_scoring_matrix_set() -> BaziScoringMatrixSet {
    BaziScoringMatrixSet::default()
}

pub fn compute_bazi_metrics(
    chart: &BaziChart,
    timing: Option<&BaziTimingReport>,
) -> BaziComputedMetrics {
    compute_bazi_metrics_with_matrix(chart, timing, &BaziScoringMatrixSet::default())
}

pub fn compute_bazi_metrics_with_matrix(
    chart: &BaziChart,
    timing: Option<&BaziTimingReport>,
    matrix: &BaziScoringMatrixSet,
) -> BaziComputedMetrics {
    let analysis = analyze_bazi_chart(chart);
    build_metrics_from_analysis(chart, &analysis, timing, matrix)
}

pub fn build_metrics_from_analysis(
    chart: &BaziChart,
    analysis: &BaziAnalysisReport,
    timing: Option<&BaziTimingReport>,
    matrix: &BaziScoringMatrixSet,
) -> BaziComputedMetrics {
    let core_metrics = compute_core_metrics(chart, analysis, matrix);
    let structure_metrics =
        compute_structure_metrics(analysis, matrix, core_metrics.element_balance_score);
    let domain_scores = compute_domain_scores(analysis, matrix, structure_metrics.confidence);
    let timing_metrics = compute_timing_metrics(analysis, timing, matrix);

    BaziComputedMetrics {
        core_metrics,
        structure_metrics,
        domain_scores,
        timing_metrics,
    }
}

fn compute_core_metrics(
    chart: &BaziChart,
    analysis: &BaziAnalysisReport,
    matrix: &BaziScoringMatrixSet,
) -> BaziCoreMetrics {
    let day_element = stem_element_from_chart(chart);
    let month_branch = chart.month_pillar.can_chi.chi.as_str();
    let season_support_score =
        season_strength_value(&matrix.season_strength, day_element, month_branch);
    let same_element_score = element_total(&analysis.element_distribution, day_element);
    let resource_element = resource_element_for(day_element);
    let resource_support_score = element_total(&analysis.element_distribution, resource_element);
    let drain_pressure_score = drain_pressure(day_element, &analysis.element_distribution);
    let control_pressure_score = control_pressure(day_element, &analysis.element_distribution);
    let element_balance_score = compute_element_balance_score(&analysis.element_distribution);

    BaziCoreMetrics {
        day_master_strength_score: analysis.day_master_strength.score,
        day_master_strength_label: strength_label_name(analysis.day_master_strength.label.clone())
            .to_string(),
        season_support_score,
        same_element_score,
        resource_support_score,
        drain_pressure_score,
        control_pressure_score,
        element_balance_score,
    }
}

fn compute_structure_metrics(
    analysis: &BaziAnalysisReport,
    matrix: &BaziScoringMatrixSet,
    element_balance_score: f32,
) -> BaziStructureMetrics {
    let ordered_elements = ordered_elements(&analysis.element_distribution);
    let dominant_elements = ordered_elements.iter().take(2).map(|(e, _)| *e).collect();
    let weak_elements = ordered_elements
        .iter()
        .rev()
        .take(2)
        .map(|(e, _)| *e)
        .collect();
    let dominant_ten_gods = ordered_ten_gods(&analysis.ten_god_distribution)
        .into_iter()
        .take(3)
        .map(|(label, _)| ten_god_name(label).to_string())
        .collect();
    let notable_interactions = analysis
        .interactions
        .iter()
        .map(|interaction| BaziInteractionMetric {
            kind: interaction_kind_name(&interaction.kind).to_string(),
            participants: interaction.participants.clone(),
            impact: interaction_impact(&interaction.kind, &matrix.interaction_impacts),
        })
        .collect::<Vec<_>>();
    let interaction_score = notable_interactions
        .iter()
        .map(|item| item.impact)
        .sum::<f32>();
    let confidence = structure_confidence(analysis, element_balance_score);

    BaziStructureMetrics {
        dominant_elements,
        weak_elements,
        dominant_ten_gods,
        interaction_score,
        notable_interactions,
        confidence,
    }
}

fn compute_domain_scores(
    analysis: &BaziAnalysisReport,
    matrix: &BaziScoringMatrixSet,
    base_confidence: f32,
) -> BaziDomainScores {
    BaziDomainScores {
        career: compute_domain_score(
            analysis,
            &matrix.domain_mapping.career,
            &matrix.ten_god_context,
            domain_confidence(base_confidence, analysis, "career"),
            "career",
        ),
        wealth: compute_domain_score(
            analysis,
            &matrix.domain_mapping.wealth,
            &matrix.ten_god_context,
            domain_confidence(base_confidence, analysis, "wealth"),
            "wealth",
        ),
        relationship: compute_domain_score(
            analysis,
            &matrix.domain_mapping.relationship,
            &matrix.ten_god_context,
            domain_confidence(base_confidence, analysis, "relationship"),
            "relationship",
        ),
        health: compute_domain_score(
            analysis,
            &matrix.domain_mapping.health,
            &matrix.ten_god_context,
            domain_confidence(base_confidence, analysis, "health"),
            "health",
        ),
        timing: compute_domain_score(
            analysis,
            &matrix.domain_mapping.timing,
            &matrix.ten_god_context,
            domain_confidence(base_confidence, analysis, "timing"),
            "timing",
        ),
    }
}

fn compute_timing_metrics(
    analysis: &BaziAnalysisReport,
    timing: Option<&BaziTimingReport>,
    matrix: &BaziScoringMatrixSet,
) -> BaziTimingMetrics {
    let Some(timing) = timing else {
        return BaziTimingMetrics {
            current_dai_van_alignment: None,
            annual_alignment: None,
            monthly_windows: vec![],
            activation_summary: vec![],
        };
    };

    let current_dai_van_alignment = timing.active_dai_van.as_ref().map(|active| {
        transient_alignment_score(
            active.can_chi.as_str(),
            active.ten_god_to_day_master.as_ref(),
            analysis,
            matrix,
        )
    });
    let annual_alignment = Some(transient_alignment_score(
        timing.annual.can_chi.as_str(),
        timing.annual.ten_god_to_day_master.as_ref(),
        analysis,
        matrix,
    ));
    let monthly_windows = timing
        .monthly
        .iter()
        .map(|month| {
            let score = transient_alignment_score(
                month.can_chi.as_str(),
                month.ten_god_to_day_master.as_ref(),
                analysis,
                matrix,
            );
            BaziTimingWindowScore {
                month: month.month,
                score,
                label: normalized_label(score),
                confidence: timing_confidence(score, analysis),
            }
        })
        .collect::<Vec<_>>();

    let mut activation_summary = Vec::new();
    if let Some(active) = &timing.active_dai_van {
        activation_summary.push(format!("Đại vận hiện hành: {}.", active.can_chi));
    }
    if !timing.annual.interactions.is_empty() {
        activation_summary.push("Lưu niên có tương tác trực tiếp với mệnh cục.".to_string());
    }
    if analysis.interactions.len() >= 2 {
        activation_summary.push(
            "Mệnh cục nền đã có nhiều tương tác, nên ưu tiên đọc vận theo ngữ cảnh.".to_string(),
        );
    }

    BaziTimingMetrics {
        current_dai_van_alignment,
        annual_alignment,
        monthly_windows,
        activation_summary,
    }
}

fn compute_domain_score(
    analysis: &BaziAnalysisReport,
    mapping: &DomainWeightProfile,
    context_matrix: &TenGodContextMatrix,
    confidence: f32,
    label_namespace: &str,
) -> BaziDomainScore {
    let context_weights = match analysis.day_master_strength.label {
        DayMasterStrengthLabel::Weak => &context_matrix.weak_dm,
        DayMasterStrengthLabel::Balanced => &context_matrix.balanced_dm,
        DayMasterStrengthLabel::Strong => &context_matrix.strong_dm,
    };

    let interaction_counts = interaction_counts(&analysis.interactions);
    let imbalance_penalty =
        (100.0 - compute_element_balance_score(&analysis.element_distribution)) / 100.0;

    let mut contributors = vec![
        contributor(
            "ty_kien",
            analysis.ten_god_distribution.ty_kien as f32,
            mapping.ty_kien,
            context_weights.ty_kien,
        ),
        contributor(
            "kiep_tai",
            analysis.ten_god_distribution.kiep_tai as f32,
            mapping.kiep_tai,
            context_weights.kiep_tai,
        ),
        contributor(
            "thuc_than",
            analysis.ten_god_distribution.thuc_than as f32,
            mapping.thuc_than,
            context_weights.thuc_than,
        ),
        contributor(
            "thuong_quan",
            analysis.ten_god_distribution.thuong_quan as f32,
            mapping.thuong_quan,
            context_weights.thuong_quan,
        ),
        contributor(
            "chinh_tai",
            analysis.ten_god_distribution.chinh_tai as f32,
            mapping.chinh_tai,
            context_weights.chinh_tai,
        ),
        contributor(
            "thien_tai",
            analysis.ten_god_distribution.thien_tai as f32,
            mapping.thien_tai,
            context_weights.thien_tai,
        ),
        contributor(
            "chinh_quan",
            analysis.ten_god_distribution.chinh_quan as f32,
            mapping.chinh_quan,
            context_weights.chinh_quan,
        ),
        contributor(
            "that_sat",
            analysis.ten_god_distribution.that_sat as f32,
            mapping.that_sat,
            context_weights.that_sat,
        ),
        contributor(
            "chinh_an",
            analysis.ten_god_distribution.chinh_an as f32,
            mapping.chinh_an,
            context_weights.chinh_an,
        ),
        contributor(
            "thien_an",
            analysis.ten_god_distribution.thien_an as f32,
            mapping.thien_an,
            context_weights.thien_an,
        ),
        BaziScoreContributor {
            signal: "branch_clash".to_string(),
            delta: interaction_counts.0 as f32 * mapping.branch_clash * 10.0,
        },
        BaziScoreContributor {
            signal: "branch_harmony".to_string(),
            delta: interaction_counts.1 as f32 * mapping.branch_harmony * 10.0,
        },
        BaziScoreContributor {
            signal: "branch_harm".to_string(),
            delta: interaction_counts.2 as f32 * mapping.branch_harm * 10.0,
        },
        BaziScoreContributor {
            signal: "element_imbalance".to_string(),
            delta: imbalance_penalty * mapping.element_imbalance * 20.0,
        },
    ]
    .into_iter()
    .filter(|item| item.delta.abs() > 0.01)
    .collect::<Vec<_>>();
    contributors.sort_by(|left, right| right.delta.abs().total_cmp(&left.delta.abs()));
    let top_signal_count = contributors.iter().take(3).count();
    contributors.push(BaziScoreContributor {
        signal: "domain_signal_count".to_string(),
        delta: top_signal_count as f32,
    });

    let raw = 50.0 + contributors.iter().map(|item| item.delta).sum::<f32>();
    let score = raw.clamp(0.0, 100.0) as u8;
    let label_key = normalized_label(score as f32 / 100.0);

    BaziDomainScore {
        score,
        label: format!("{}_{}", label_namespace, label_key),
        confidence,
        evidence_level: evidence_level(confidence, contributors.len()),
        contributors,
    }
}

fn contributor(
    signal: &str,
    count: f32,
    mapping_weight: f32,
    context_weight: f32,
) -> BaziScoreContributor {
    BaziScoreContributor {
        signal: signal.to_string(),
        delta: count * mapping_weight * context_weight * 10.0,
    }
}

fn transient_alignment_score(
    _can_chi: &str,
    ten_god: Option<&crate::almanac::types::ThapThanResult>,
    analysis: &BaziAnalysisReport,
    matrix: &BaziScoringMatrixSet,
) -> f32 {
    let mut score = 0.5;
    if let Some(ten_god) = ten_god {
        let context = match analysis.day_master_strength.label {
            DayMasterStrengthLabel::Weak => &matrix.ten_god_context.weak_dm,
            DayMasterStrengthLabel::Balanced => &matrix.ten_god_context.balanced_dm,
            DayMasterStrengthLabel::Strong => &matrix.ten_god_context.strong_dm,
        };
        score += ten_god_profile_weight(context, ten_god.label) * 0.2;
    }
    let (clash_count, harmony_count, harm_count) = interaction_counts(&analysis.interactions);
    score += harmony_count as f32 * 0.05;
    score -= clash_count as f32 * 0.06;
    score -= harm_count as f32 * 0.04;
    score.clamp(0.0, 1.0)
}

fn interaction_counts(
    interactions: &[crate::bazi::analysis::ChartInteraction],
) -> (usize, usize, usize) {
    let mut clash = 0;
    let mut harmony = 0;
    let mut harm = 0;
    for interaction in interactions {
        match interaction.kind {
            ChartInteractionKind::BranchClash => clash += 1,
            ChartInteractionKind::BranchHarmony => harmony += 1,
            ChartInteractionKind::BranchHarm => harm += 1,
        }
    }
    (clash, harmony, harm)
}

fn compute_element_balance_score(distribution: &ElementDistribution) -> f32 {
    let values = [
        distribution.moc as f32,
        distribution.hoa as f32,
        distribution.tho as f32,
        distribution.kim as f32,
        distribution.thuy as f32,
    ];
    let total = values.iter().sum::<f32>();
    if total <= 0.0 {
        return 0.0;
    }
    let mean = total / values.len() as f32;
    let variance = values
        .iter()
        .map(|value| (value - mean).powi(2))
        .sum::<f32>()
        / values.len() as f32;
    let normalized = (1.0 - (variance.sqrt() / (mean + 1.0))).clamp(0.0, 1.0);
    normalized * 100.0
}

fn ordered_elements(distribution: &ElementDistribution) -> Vec<(FiveElement, u16)> {
    let mut values = vec![
        (FiveElement::Moc, distribution.moc),
        (FiveElement::Hoa, distribution.hoa),
        (FiveElement::Tho, distribution.tho),
        (FiveElement::Kim, distribution.kim),
        (FiveElement::Thuy, distribution.thuy),
    ];
    values.sort_by(|left, right| right.1.cmp(&left.1));
    values
}

fn ordered_ten_gods(distribution: &TenGodDistribution) -> Vec<(ThapThanLabel, u8)> {
    let mut values = vec![
        (ThapThanLabel::TyKien, distribution.ty_kien),
        (ThapThanLabel::KiepTai, distribution.kiep_tai),
        (ThapThanLabel::ThucThan, distribution.thuc_than),
        (ThapThanLabel::ThuongQuan, distribution.thuong_quan),
        (ThapThanLabel::ChinhTai, distribution.chinh_tai),
        (ThapThanLabel::ThienTai, distribution.thien_tai),
        (ThapThanLabel::ChinhQuan, distribution.chinh_quan),
        (ThapThanLabel::ThatSat, distribution.that_sat),
        (ThapThanLabel::ChinhAn, distribution.chinh_an),
        (ThapThanLabel::ThienAn, distribution.thien_an),
    ];
    values.sort_by(|left, right| right.1.cmp(&left.1));
    values
}

fn stem_element_from_chart(chart: &BaziChart) -> FiveElement {
    crate::almanac::types::HeavenlyStem::try_from(chart.day_master.can.as_str())
        .expect("day master stem")
        .element()
}

fn resource_element_for(element: FiveElement) -> FiveElement {
    match element {
        FiveElement::Moc => FiveElement::Thuy,
        FiveElement::Hoa => FiveElement::Moc,
        FiveElement::Tho => FiveElement::Hoa,
        FiveElement::Kim => FiveElement::Tho,
        FiveElement::Thuy => FiveElement::Kim,
    }
}

fn drain_pressure(element: FiveElement, distribution: &ElementDistribution) -> u16 {
    match element {
        FiveElement::Moc => distribution.hoa,
        FiveElement::Hoa => distribution.tho,
        FiveElement::Tho => distribution.kim,
        FiveElement::Kim => distribution.thuy,
        FiveElement::Thuy => distribution.moc,
    }
}

fn control_pressure(element: FiveElement, distribution: &ElementDistribution) -> u16 {
    match element {
        FiveElement::Moc => distribution.kim,
        FiveElement::Hoa => distribution.thuy,
        FiveElement::Tho => distribution.moc,
        FiveElement::Kim => distribution.hoa,
        FiveElement::Thuy => distribution.tho,
    }
}

fn element_total(distribution: &ElementDistribution, element: FiveElement) -> u16 {
    match element {
        FiveElement::Moc => distribution.moc,
        FiveElement::Hoa => distribution.hoa,
        FiveElement::Tho => distribution.tho,
        FiveElement::Kim => distribution.kim,
        FiveElement::Thuy => distribution.thuy,
    }
}

fn season_strength_value(matrix: &SeasonStrengthMatrix, element: FiveElement, branch: &str) -> f32 {
    let profile = match element {
        FiveElement::Moc => &matrix.moc,
        FiveElement::Hoa => &matrix.hoa,
        FiveElement::Tho => &matrix.tho,
        FiveElement::Kim => &matrix.kim,
        FiveElement::Thuy => &matrix.thuy,
    };
    match branch {
        "Tý" => profile.ty,
        "Sửu" => profile.suu,
        "Dần" => profile.dan,
        "Mão" => profile.mao,
        "Thìn" => profile.thin,
        "Tỵ" => profile.ty2,
        "Ngọ" => profile.ngo,
        "Mùi" => profile.mui,
        "Thân" => profile.than,
        "Dậu" => profile.dau,
        "Tuất" => profile.tuat,
        "Hợi" => profile.hoi,
        _ => 0.0,
    }
}

fn ten_god_profile_weight(profile: &TenGodWeightProfile, label: ThapThanLabel) -> f32 {
    match label {
        ThapThanLabel::TyKien => profile.ty_kien,
        ThapThanLabel::KiepTai => profile.kiep_tai,
        ThapThanLabel::ThucThan => profile.thuc_than,
        ThapThanLabel::ThuongQuan => profile.thuong_quan,
        ThapThanLabel::ChinhTai => profile.chinh_tai,
        ThapThanLabel::ThienTai => profile.thien_tai,
        ThapThanLabel::ChinhQuan => profile.chinh_quan,
        ThapThanLabel::ThatSat => profile.that_sat,
        ThapThanLabel::ChinhAn => profile.chinh_an,
        ThapThanLabel::ThienAn => profile.thien_an,
    }
}

fn strength_label_name(label: DayMasterStrengthLabel) -> &'static str {
    match label {
        DayMasterStrengthLabel::Strong => "strong",
        DayMasterStrengthLabel::Balanced => "balanced",
        DayMasterStrengthLabel::Weak => "weak",
    }
}

fn interaction_kind_name(kind: &ChartInteractionKind) -> &'static str {
    match kind {
        ChartInteractionKind::BranchClash => "branch_clash",
        ChartInteractionKind::BranchHarmony => "branch_harmony",
        ChartInteractionKind::BranchHarm => "branch_harm",
    }
}

fn interaction_impact(kind: &ChartInteractionKind, matrix: &InteractionImpactMatrix) -> f32 {
    match kind {
        ChartInteractionKind::BranchClash => matrix.branch_clash,
        ChartInteractionKind::BranchHarmony => matrix.branch_harmony,
        ChartInteractionKind::BranchHarm => matrix.branch_harm,
    }
}

fn ten_god_name(label: ThapThanLabel) -> &'static str {
    match label {
        ThapThanLabel::TyKien => "ty_kien",
        ThapThanLabel::KiepTai => "kiep_tai",
        ThapThanLabel::ThucThan => "thuc_than",
        ThapThanLabel::ThuongQuan => "thuong_quan",
        ThapThanLabel::ChinhTai => "chinh_tai",
        ThapThanLabel::ThienTai => "thien_tai",
        ThapThanLabel::ChinhQuan => "chinh_quan",
        ThapThanLabel::ThatSat => "that_sat",
        ThapThanLabel::ChinhAn => "chinh_an",
        ThapThanLabel::ThienAn => "thien_an",
    }
}

fn normalized_label(score: f32) -> String {
    if score >= 0.78 {
        "supportive".to_string()
    } else if score >= 0.58 {
        "developing".to_string()
    } else if score >= 0.38 {
        "mixed".to_string()
    } else {
        "watchlist".to_string()
    }
}

fn structure_confidence(analysis: &BaziAnalysisReport, element_balance_score: f32) -> f32 {
    ((element_balance_score / 100.0) * 0.45
        + (1.0 - (analysis.interactions.len() as f32 * 0.08)).clamp(0.25, 1.0) * 0.35
        + if analysis.day_master_strength.reasons.len() >= 4 {
            0.2
        } else {
            0.1
        })
    .clamp(0.0, 1.0)
}

fn domain_confidence(base_confidence: f32, analysis: &BaziAnalysisReport, domain: &str) -> f32 {
    let interaction_penalty = match domain {
        "relationship" => analysis.interactions.len() as f32 * 0.04,
        "timing" => analysis.interactions.len() as f32 * 0.03,
        _ => analysis.interactions.len() as f32 * 0.02,
    };
    let ten_god_signal_bonus = (ordered_ten_gods(&analysis.ten_god_distribution)
        .iter()
        .take(3)
        .filter(|(_, count)| *count > 0)
        .count() as f32)
        * 0.03;
    (base_confidence + ten_god_signal_bonus - interaction_penalty).clamp(0.0, 1.0)
}

fn timing_confidence(score: f32, analysis: &BaziAnalysisReport) -> f32 {
    (0.45 + score * 0.35 - analysis.interactions.len() as f32 * 0.03).clamp(0.0, 1.0)
}

fn evidence_level(confidence: f32, contributor_count: usize) -> String {
    if confidence >= 0.75 && contributor_count >= 4 {
        "high".to_string()
    } else if confidence >= 0.5 && contributor_count >= 2 {
        "medium".to_string()
    } else {
        "low".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        bazi::{build_bazi_chart, build_bazi_timing_report},
        types::VIETNAM_TIMEZONE,
        BaziInput, Gender,
    };

    fn sample_chart() -> BaziChart {
        build_bazi_chart(BaziInput {
            day: 10,
            month: 2,
            year: 2024,
            hour: 9,
            minute: 30,
            timezone: VIETNAM_TIMEZONE,
            longitude: None,
            use_solar_time: false,
            gender: Some(Gender::Male),
        })
        .expect("chart")
    }

    #[test]
    fn default_matrix_set_has_expected_weights() {
        let matrix = default_bazi_scoring_matrix_set();
        assert_eq!(matrix.visibility_weights.visible_stem, 1.0);
        assert!(matrix.interaction_impacts.branch_clash < 0.0);
        assert!(matrix.domain_mapping.wealth.chinh_tai > 0.0);
    }

    #[test]
    fn computes_metrics_without_timing() {
        let chart = sample_chart();
        let metrics = compute_bazi_metrics(&chart, None);

        assert!(!metrics.structure_metrics.dominant_elements.is_empty());
        assert!(metrics.core_metrics.day_master_strength_score > 0);
        assert!(metrics.domain_scores.career.score <= 100);
    }

    #[test]
    fn computes_metrics_with_timing() {
        let chart = sample_chart();
        let timing =
            build_bazi_timing_report(&chart, Gender::Male, 15.0, 2027, &[1, 2]).expect("timing");
        let metrics = compute_bazi_metrics(&chart, Some(&timing));

        assert!(metrics.timing_metrics.current_dai_van_alignment.is_some());
        assert_eq!(metrics.timing_metrics.monthly_windows.len(), 2);
        assert!(!metrics.timing_metrics.activation_summary.is_empty());
    }
}
