use serde::{Deserialize, Serialize};

use crate::{
    almanac::types::FiveElement,
    bazi::{
        analysis::{
            analyze_bazi_chart, BaziAnalysisReport, DayMasterStrengthLabel, ElementDistribution,
        },
        scoring::{compute_bazi_metrics, BaziComputedMetrics, BaziDomainScore},
        timing::BaziTimingReport,
        types::BaziChart,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UsefulGodAnalysis {
    pub favorable_elements: Vec<FiveElement>,
    pub unfavorable_elements: Vec<FiveElement>,
    pub tentative_yong_shen: Option<FiveElement>,
    pub tentative_xi_shen: Option<FiveElement>,
    pub confidence: String,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaziAdvisoryDomains {
    pub career: Vec<String>,
    pub wealth: Vec<String>,
    pub relationship: Vec<String>,
    pub health: Vec<String>,
    pub timing: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaziAdvisoryReport {
    pub useful_god_analysis: UsefulGodAnalysis,
    pub summary_vi: String,
    pub warnings: Vec<String>,
    pub domains: BaziAdvisoryDomains,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaziAdvisoryExport {
    pub summary: String,
    pub severity: String,
    pub top_signals: Vec<String>,
    pub why_this_matters: Vec<String>,
    pub recommended_actions: Vec<String>,
    pub priority_order: Vec<String>,
    pub useful_god_analysis: UsefulGodAnalysis,
    pub summary_vi: String,
    pub warnings: Vec<String>,
    pub domains: BaziAdvisoryDomains,
}

pub fn build_bazi_advisory(
    chart: &BaziChart,
    timing: Option<&BaziTimingReport>,
) -> BaziAdvisoryReport {
    let analysis = analyze_bazi_chart(chart);
    let metrics = compute_bazi_metrics(chart, timing);
    build_bazi_advisory_from_metrics(chart, &analysis, &metrics, timing)
}

pub fn build_bazi_advisory_from_metrics(
    chart: &BaziChart,
    analysis: &BaziAnalysisReport,
    metrics: &BaziComputedMetrics,
    timing: Option<&BaziTimingReport>,
) -> BaziAdvisoryReport {
    let useful_god_analysis = infer_useful_gods_from_metrics(analysis, metrics);
    let domains = derive_domain_advice(metrics, timing, &useful_god_analysis);
    let warnings = build_warnings(chart, analysis, metrics);
    let summary_vi = build_summary(metrics, &useful_god_analysis, timing);

    BaziAdvisoryReport {
        useful_god_analysis,
        summary_vi,
        warnings,
        domains,
    }
}

pub fn export_bazi_advisory(report: &BaziAdvisoryReport) -> BaziAdvisoryExport {
    let mut top_signals = Vec::new();
    let mut why_this_matters = Vec::new();
    let mut recommended_actions = Vec::new();

    if let Some(yong_shen) = report.useful_god_analysis.tentative_yong_shen {
        top_signals.push(format!("yong_shen {:?}", yong_shen));
        why_this_matters.push(
            "Dụng thần points to the element most useful for restoring chart balance.".to_string(),
        );
    }
    if let Some(xi_shen) = report.useful_god_analysis.tentative_xi_shen {
        top_signals.push(format!("xi_shen {:?}", xi_shen));
        why_this_matters.push(
            "Hỷ thần highlights secondary support, useful for timing and softer optimization."
                .to_string(),
        );
    }
    if let Some(first_warning) = report.warnings.first() {
        top_signals.push(first_warning.clone());
        recommended_actions.push(
            "Treat warnings as constraints before optimizing around favorable signals.".to_string(),
        );
    }

    let severity = if report.warnings.len() >= 2 {
        "high"
    } else if !report.warnings.is_empty() {
        "medium"
    } else {
        "low"
    }
    .to_string();

    let summary = if !report.warnings.is_empty() {
        format!(
            "Bazi advisory includes {} warning(s) with {} top signal(s).",
            report.warnings.len(),
            top_signals.len()
        )
    } else {
        format!(
            "Bazi advisory is primarily explanatory with {} top signal(s).",
            top_signals.len()
        )
    };

    if recommended_actions.is_empty() {
        recommended_actions.push(
            "Use the strongest supporting element signals first when choosing timing, workload, or emphasis."
                .to_string(),
        );
    }

    let priority_order = if !report.warnings.is_empty() {
        vec![
            "Review warnings first".to_string(),
            "Interpret top_signals in light of balance goals".to_string(),
            "Then use domain guidance for concrete decisions".to_string(),
        ]
    } else {
        vec![
            "Start from top_signals".to_string(),
            "Use useful-god guidance to shape choices".to_string(),
            "Apply domain guidance to execution details".to_string(),
        ]
    };

    BaziAdvisoryExport {
        summary,
        severity,
        top_signals,
        why_this_matters,
        recommended_actions,
        priority_order,
        useful_god_analysis: report.useful_god_analysis.clone(),
        summary_vi: report.summary_vi.clone(),
        warnings: report.warnings.clone(),
        domains: report.domains.clone(),
    }
}

pub fn infer_useful_gods(analysis: &BaziAnalysisReport) -> UsefulGodAnalysis {
    infer_useful_gods_from_metrics(
        analysis,
        &BaziComputedMetrics {
            core_metrics: crate::bazi::scoring::BaziCoreMetrics {
                day_master_strength_score: analysis.day_master_strength.score,
                day_master_strength_label: match analysis.day_master_strength.label {
                    DayMasterStrengthLabel::Strong => "strong".to_string(),
                    DayMasterStrengthLabel::Balanced => "balanced".to_string(),
                    DayMasterStrengthLabel::Weak => "weak".to_string(),
                },
                season_support_score: 0.0,
                same_element_score: 0,
                resource_support_score: 0,
                drain_pressure_score: 0,
                control_pressure_score: 0,
                element_balance_score: 0.0,
            },
            structure_metrics: crate::bazi::scoring::BaziStructureMetrics {
                dominant_elements: most_present_elements(&analysis.element_distribution)
                    .into_iter()
                    .take(2)
                    .collect(),
                weak_elements: least_present_elements(&analysis.element_distribution)
                    .into_iter()
                    .take(2)
                    .collect(),
                dominant_ten_gods: vec![],
                interaction_score: 0.0,
                notable_interactions: vec![],
                confidence: 0.5,
            },
            domain_scores: crate::bazi::scoring::BaziDomainScores {
                career: empty_domain_score(),
                wealth: empty_domain_score(),
                relationship: empty_domain_score(),
                health: empty_domain_score(),
                timing: empty_domain_score(),
            },
            timing_metrics: crate::bazi::scoring::BaziTimingMetrics {
                current_dai_van_alignment: None,
                annual_alignment: None,
                monthly_windows: vec![],
                activation_summary: vec![],
            },
        },
    )
}

pub fn infer_useful_gods_from_metrics(
    analysis: &BaziAnalysisReport,
    metrics: &BaziComputedMetrics,
) -> UsefulGodAnalysis {
    let ranked_low = metrics.structure_metrics.weak_elements.clone();
    let ranked_high = metrics.structure_metrics.dominant_elements.clone();

    let (
        favorable_elements,
        unfavorable_elements,
        tentative_yong_shen,
        tentative_xi_shen,
        confidence,
        mut reasons,
    ) = match analysis.day_master_strength.label {
        DayMasterStrengthLabel::Weak => {
            let favorable = ranked_low.clone();
            let unfavorable = ranked_high.clone();
            let yong = favorable.first().copied();
            let xi = favorable.get(1).copied().or(yong);
            let reasons = vec![
                "Day master đang nghiêng về nhược nên ưu tiên ngũ hành trợ thân.".to_string(),
                "Ngũ hành thiếu được xem như ứng viên dụng/hỷ thần giai đoạn đầu.".to_string(),
            ];
            (
                favorable,
                unfavorable,
                yong,
                xi,
                "medium".to_string(),
                reasons,
            )
        }
        DayMasterStrengthLabel::Strong => {
            let favorable = ranked_high.iter().rev().copied().collect::<Vec<_>>();
            let unfavorable = ranked_high.clone();
            let yong = favorable.first().copied();
            let xi = favorable.get(1).copied().or(yong);
            let reasons = vec![
                "Day master đang vượng nên ưu tiên hành giúp tiết hoặc cân bằng.".to_string(),
                "Ngũ hành đang trội bị xem là ứng viên kỵ thần tạm thời.".to_string(),
            ];
            (
                favorable,
                unfavorable,
                yong,
                xi,
                "medium".to_string(),
                reasons,
            )
        }
        DayMasterStrengthLabel::Balanced => {
            let favorable = ranked_low.clone();
            let unfavorable = ranked_high.clone();
            let yong = favorable.first().copied();
            let xi = favorable.get(1).copied().or(yong);
            let reasons = vec![
                "Mệnh cục khá cân bằng nên chỉ bổ trợ nhẹ vào hành còn thiếu.".to_string(),
                "Không nên kết luận dụng thần quá mạnh ở giai đoạn heuristic.".to_string(),
            ];
            (favorable, unfavorable, yong, xi, "low".to_string(), reasons)
        }
    };

    reasons.extend(analysis.day_master_strength.reasons.iter().cloned());

    UsefulGodAnalysis {
        favorable_elements,
        unfavorable_elements,
        tentative_yong_shen,
        tentative_xi_shen,
        confidence,
        reasons,
    }
}

fn derive_domain_advice(
    metrics: &BaziComputedMetrics,
    timing: Option<&BaziTimingReport>,
    useful_gods: &UsefulGodAnalysis,
) -> BaziAdvisoryDomains {
    let career = domain_advice_lines(
        &metrics.domain_scores.career,
        "Sự nghiệp có nền khá hỗ trợ; hợp tăng trách nhiệm và chuẩn hóa hướng phát triển.",
        "Sự nghiệp đang ở pha phát triển; nên tích lũy track record và giữ nhịp bền.",
        "Sự nghiệp có tín hiệu trái chiều; nên ưu tiên chọn môi trường ổn định.",
        "Sự nghiệp đang nhạy cảm; tránh ôm quá nhiều mặt trận cùng lúc.",
    );

    let wealth = domain_advice_lines(
        &metrics.domain_scores.wealth,
        "Tài vận có nhịp hỗ trợ; hợp tối ưu vận hành và kỷ luật dòng tiền.",
        "Tài vận đang mở dần; nên tích lũy đều và ưu tiên cấu trúc dài hạn.",
        "Tài vận lẫn lộn; cần tách rõ phần tăng trưởng và phần phòng thủ.",
        "Tài vận cần thận trọng; tránh đòn bẩy hoặc quyết định cảm tính.",
    );

    let relationship = domain_advice_lines(
        &metrics.domain_scores.relationship,
        "Quan hệ/hợp tác có nền khá hỗ trợ; hợp xây nhịp phối hợp rõ ràng.",
        "Quan hệ ở pha phát triển; nên giao tiếp sớm để tránh hiểu sai tích lũy.",
        "Quan hệ có tín hiệu pha trộn; nên giữ ranh giới và kỳ vọng thực tế.",
        "Quan hệ đang nhạy cảm; nên giảm va chạm trực diện và tăng độ mềm của nhịp trao đổi.",
    );

    let mut health = domain_advice_lines(
        &metrics.domain_scores.health,
        "Thể trạng nền tương đối ổn; nên giữ nhịp sinh hoạt đều để duy trì cân bằng.",
        "Sức khỏe cần được tối ưu dần; nên ưu tiên ngủ nghỉ và hồi phục nền.",
        "Có dấu hiệu mất cân bằng; cần theo dõi các thói quen tiêu hao kéo dài.",
        "Sức khỏe nằm trong vùng cảnh giác; nên giảm quá tải và ưu tiên kiểm chứng thực tế.",
    );
    if let Some(unfavorable) = useful_gods.unfavorable_elements.first() {
        health.push(format!(
            "Nên lưu ý các thói quen làm hành {:?} thêm quá vượng.",
            unfavorable
        ));
    }

    let mut timing_notes = Vec::new();
    timing_notes.extend(domain_advice_lines(
        &metrics.domain_scores.timing,
        "Nhịp vận hiện khá hỗ trợ cho việc chủ động triển khai kế hoạch.",
        "Nhịp vận đang mở; nên đi từng bước và ưu tiên điểm tựa chắc chắn.",
        "Nhịp vận đan xen; nên chọn cửa đánh có biên an toàn.",
        "Nhịp vận nhạy cảm; hợp quan sát thêm trước các quyết định lớn.",
    ));
    timing_notes.extend(metrics.timing_metrics.activation_summary.iter().cloned());
    if let Some(timing) = timing {
        if let Some(active) = &timing.active_dai_van {
            timing_notes.push(format!("Đang ở đại vận {}.", active.can_chi));
        }
        timing_notes.push(format!("Lưu niên hiện xét: {}.", timing.annual.can_chi));
        if !timing.annual.interactions.is_empty() {
            timing_notes.push("Lưu niên hiện có tương tác trực tiếp với mệnh cục.".to_string());
        }

        let active_months = timing
            .monthly
            .iter()
            .filter(|month| !month.interactions.is_empty())
            .map(|month| format!("tháng {} ({})", month.month, month.can_chi))
            .collect::<Vec<_>>();
        if !active_months.is_empty() {
            timing_notes.push(format!(
                "Các lưu nguyệt nổi bật: {}.",
                active_months.join(", ")
            ));
        }
    }

    BaziAdvisoryDomains {
        career,
        wealth,
        relationship,
        health,
        timing: timing_notes,
    }
}

fn build_warnings(
    chart: &BaziChart,
    analysis: &BaziAnalysisReport,
    metrics: &BaziComputedMetrics,
) -> Vec<String> {
    let mut warnings = Vec::new();

    if chart.input.use_solar_time {
        warnings.push(
            "Đang bật solar-time path; cần kiểm chứng thêm với dữ liệu kinh độ thực.".to_string(),
        );
    }

    if analysis.interactions.len() >= 3 {
        warnings.push(
            "Chart có nhiều tương tác chi; diễn giải nên đi kèm kiểm chứng thủ công.".to_string(),
        );
    }
    if metrics.structure_metrics.confidence < 0.45 {
        warnings.push(
            "Confidence của scoring hiện chưa cao; nên ưu tiên đọc như tín hiệu định hướng."
                .to_string(),
        );
    }

    warnings.push(
        "Dụng thần/hỷ thần hiện là heuristic giai đoạn đầu, chưa phải kết luận trường phái đầy đủ."
            .to_string(),
    );
    warnings
}

fn build_summary(
    metrics: &BaziComputedMetrics,
    useful_gods: &UsefulGodAnalysis,
    timing: Option<&BaziTimingReport>,
) -> String {
    let strength = match metrics.core_metrics.day_master_strength_label.as_str() {
        "strong" => "thân vượng",
        "balanced" => "cân bằng",
        _ => "thân nhược",
    };

    let yong = useful_gods
        .tentative_yong_shen
        .map(element_label_vi)
        .unwrap_or("chưa xác định");
    let timing_text = timing
        .and_then(|report| report.active_dai_van.as_ref())
        .map(|active| format!(" Đại vận hiện tại: {}.", active.can_chi))
        .unwrap_or_default();

    format!(
        "Bát tự hiện cho thấy day master ở trạng thái {}, dụng thần tạm nghiêng về {}.{}",
        strength, yong, timing_text
    )
}

fn domain_advice_lines(
    score: &BaziDomainScore,
    supportive: &str,
    developing: &str,
    mixed: &str,
    watchlist: &str,
) -> Vec<String> {
    let primary = match score
        .label
        .rsplit('_')
        .next()
        .unwrap_or(score.label.as_str())
    {
        "supportive" => supportive,
        "developing" => developing,
        "mixed" => mixed,
        _ => watchlist,
    };
    let mut lines = vec![primary.to_string()];
    if let Some(top) = score
        .contributors
        .iter()
        .max_by(|left, right| left.delta.abs().total_cmp(&right.delta.abs()))
    {
        lines.push(format!(
            "Tín hiệu nổi bật: {} ({:+.1}).",
            top.signal, top.delta
        ));
    }
    lines
}

fn empty_domain_score() -> BaziDomainScore {
    BaziDomainScore {
        score: 50,
        label: "mixed".to_string(),
        confidence: 0.5,
        evidence_level: "low".to_string(),
        contributors: vec![],
    }
}

fn least_present_elements(distribution: &ElementDistribution) -> Vec<FiveElement> {
    let mut pairs = element_pairs(distribution);
    pairs.sort_by_key(|(_, value)| *value);
    pairs.into_iter().map(|(element, _)| element).collect()
}

fn most_present_elements(distribution: &ElementDistribution) -> Vec<FiveElement> {
    let mut pairs = element_pairs(distribution);
    pairs.sort_by_key(|item| std::cmp::Reverse(item.1));
    pairs.into_iter().map(|(element, _)| element).collect()
}

fn element_pairs(distribution: &ElementDistribution) -> Vec<(FiveElement, u16)> {
    vec![
        (FiveElement::Moc, distribution.moc),
        (FiveElement::Hoa, distribution.hoa),
        (FiveElement::Tho, distribution.tho),
        (FiveElement::Kim, distribution.kim),
        (FiveElement::Thuy, distribution.thuy),
    ]
}

fn element_label_vi(element: FiveElement) -> &'static str {
    match element {
        FiveElement::Moc => "Mộc",
        FiveElement::Hoa => "Hỏa",
        FiveElement::Tho => "Thổ",
        FiveElement::Kim => "Kim",
        FiveElement::Thuy => "Thủy",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        almanac::tu_menh::Gender,
        bazi::{build_bazi_chart, build_bazi_timing_report},
        types::VIETNAM_TIMEZONE,
        BaziInput,
    };

    fn sample_chart() -> BaziChart {
        build_bazi_chart(BaziInput {
            day: 10,
            month: 2,
            year: 2024,
            hour: 9,
            minute: 30,
            time_known: true,
            timezone: VIETNAM_TIMEZONE,
            longitude: None,
            use_solar_time: false,
            gender: Some(Gender::Male),
        })
        .expect("chart")
    }

    #[test]
    fn infers_useful_gods_from_analysis() {
        let chart = sample_chart();
        let analysis = analyze_bazi_chart(&chart);
        let useful = infer_useful_gods(&analysis);

        assert!(!useful.favorable_elements.is_empty());
        assert!(!useful.reasons.is_empty());
    }

    #[test]
    fn builds_bazi_advisory_with_timing_context() {
        let chart = sample_chart();
        let timing =
            build_bazi_timing_report(&chart, Gender::Male, 15.0, 2027, &[1, 2]).expect("timing");
        let advisory = build_bazi_advisory(&chart, Some(&timing));

        assert!(!advisory.summary_vi.is_empty());
        assert!(!advisory.domains.timing.is_empty());
        assert!(!advisory.warnings.is_empty());
    }

    #[test]
    fn timing_domain_mentions_active_transits_when_present() {
        let chart = sample_chart();
        let timing =
            build_bazi_timing_report(&chart, Gender::Male, 15.0, 2027, &[1, 2, 3]).expect("timing");
        let advisory = build_bazi_advisory(&chart, Some(&timing));

        assert!(advisory
            .domains
            .timing
            .iter()
            .any(|note| note.contains("Đại vận")
                || note.contains("Lưu niên")
                || note.contains("lưu nguyệt")
                || note.contains("Lưu nguyệt")));
    }
}
