use crate::{
    almanac::types::FiveElement,
    bazi::{
        analysis::{
            BaziAnalysisReport, DayMasterStrengthLabel, ElementDistribution, analyze_bazi_chart,
        },
        timing::BaziTimingReport,
        types::BaziChart,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsefulGodAnalysis {
    pub favorable_elements: Vec<FiveElement>,
    pub unfavorable_elements: Vec<FiveElement>,
    pub tentative_yong_shen: Option<FiveElement>,
    pub tentative_xi_shen: Option<FiveElement>,
    pub confidence: String,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaziAdvisoryDomains {
    pub career: Vec<String>,
    pub wealth: Vec<String>,
    pub relationship: Vec<String>,
    pub health: Vec<String>,
    pub timing: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaziAdvisoryReport {
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
    let useful_god_analysis = infer_useful_gods(&analysis);
    let domains = derive_domain_advice(&analysis, timing, &useful_god_analysis);
    let warnings = build_warnings(chart, &analysis);
    let summary_vi = build_summary(&analysis, &useful_god_analysis, timing);

    BaziAdvisoryReport {
        useful_god_analysis,
        summary_vi,
        warnings,
        domains,
    }
}

pub fn infer_useful_gods(analysis: &BaziAnalysisReport) -> UsefulGodAnalysis {
    let ranked_low = least_present_elements(&analysis.element_distribution);
    let ranked_high = most_present_elements(&analysis.element_distribution);

    let (favorable_elements, unfavorable_elements, tentative_yong_shen, tentative_xi_shen, confidence, mut reasons) =
        match analysis.day_master_strength.label {
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
                let favorable = ranked_high
                    .iter()
                    .rev()
                    .copied()
                    .collect::<Vec<_>>();
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
                (
                    favorable,
                    unfavorable,
                    yong,
                    xi,
                    "low".to_string(),
                    reasons,
                )
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
    analysis: &BaziAnalysisReport,
    timing: Option<&BaziTimingReport>,
    useful_gods: &UsefulGodAnalysis,
) -> BaziAdvisoryDomains {
    let resource_count =
        analysis.ten_god_distribution.chinh_an + analysis.ten_god_distribution.thien_an;
    let output_count =
        analysis.ten_god_distribution.thuc_than + analysis.ten_god_distribution.thuong_quan;
    let wealth_count =
        analysis.ten_god_distribution.chinh_tai + analysis.ten_god_distribution.thien_tai;
    let authority_count =
        analysis.ten_god_distribution.chinh_quan + analysis.ten_god_distribution.that_sat;

    let mut career = Vec::new();
    if resource_count >= authority_count {
        career.push("Hợp môi trường học hỏi, chuẩn hoá, chuyên môn sâu.".to_string());
    }
    if output_count > 0 {
        career.push("Có thể phát huy qua diễn đạt, sản xuất ý tưởng, xây sản phẩm.".to_string());
    }

    let mut wealth = Vec::new();
    if wealth_count > 0 {
        wealth.push("Có tín hiệu kích hoạt nhóm Tài, hợp tối ưu vận hành và dòng tiền.".to_string());
    } else {
        wealth.push("Nhóm Tài chưa nổi bật; nên đi chậm và ưu tiên nền tảng dài hạn.".to_string());
    }

    let mut relationship = Vec::new();
    if !analysis.interactions.is_empty() {
        relationship.push("Mệnh cục có tương tác chi; nên chú ý nhịp hợp tác và va chạm.".to_string());
    } else {
        relationship.push("Tương tác chi chưa quá dày; hợp xây quan hệ ổn định, ít cực đoan.".to_string());
    }

    let mut health = Vec::new();
    if let Some(unfavorable) = useful_gods.unfavorable_elements.first() {
        health.push(format!(
            "Nên lưu ý các thói quen làm hành {:?} thêm quá vượng.",
            unfavorable
        ));
    }

    let mut timing_notes = Vec::new();
    if let Some(timing) = timing {
        if let Some(active) = &timing.active_dai_van {
            timing_notes.push(format!("Đang ở đại vận {}.", active.can_chi));
        }
        timing_notes.push(format!("Lưu niên hiện xét: {}.", timing.annual.can_chi));
    }

    BaziAdvisoryDomains {
        career,
        wealth,
        relationship,
        health,
        timing: timing_notes,
    }
}

fn build_warnings(chart: &BaziChart, analysis: &BaziAnalysisReport) -> Vec<String> {
    let mut warnings = Vec::new();

    if chart.input.use_solar_time {
        warnings.push("Đang bật solar-time path; cần kiểm chứng thêm với dữ liệu kinh độ thực.".to_string());
    }

    if analysis.interactions.len() >= 3 {
        warnings.push("Chart có nhiều tương tác chi; diễn giải nên đi kèm kiểm chứng thủ công.".to_string());
    }

    warnings.push("Dụng thần/hỷ thần hiện là heuristic giai đoạn đầu, chưa phải kết luận trường phái đầy đủ.".to_string());
    warnings
}

fn build_summary(
    analysis: &BaziAnalysisReport,
    useful_gods: &UsefulGodAnalysis,
    timing: Option<&BaziTimingReport>,
) -> String {
    let strength = match analysis.day_master_strength.label {
        DayMasterStrengthLabel::Strong => "thân vượng",
        DayMasterStrengthLabel::Balanced => "cân bằng",
        DayMasterStrengthLabel::Weak => "thân nhược",
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

fn least_present_elements(distribution: &ElementDistribution) -> Vec<FiveElement> {
    let mut pairs = element_pairs(distribution);
    pairs.sort_by_key(|(_, value)| *value);
    pairs.into_iter().map(|(element, _)| element).collect()
}

fn most_present_elements(distribution: &ElementDistribution) -> Vec<FiveElement> {
    let mut pairs = element_pairs(distribution);
    pairs.sort_by(|left, right| right.1.cmp(&left.1));
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
        bazi::{build_bazi_chart, build_bazi_timing_report},
        BaziInput,
        almanac::tu_menh::Gender,
        types::VIETNAM_TIMEZONE,
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
}
