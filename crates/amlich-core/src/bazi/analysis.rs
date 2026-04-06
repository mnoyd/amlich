use crate::{
    almanac::types::{FiveElement, ThapThanLabel},
    bazi::types::{BaziChart, HiddenStemEntry},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementDistribution {
    pub moc: u16,
    pub hoa: u16,
    pub tho: u16,
    pub kim: u16,
    pub thuy: u16,
}

impl ElementDistribution {
    fn add(&mut self, element: FiveElement, weight: u16) {
        match element {
            FiveElement::Moc => self.moc += weight,
            FiveElement::Hoa => self.hoa += weight,
            FiveElement::Tho => self.tho += weight,
            FiveElement::Kim => self.kim += weight,
            FiveElement::Thuy => self.thuy += weight,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DayMasterStrengthLabel {
    Strong,
    Balanced,
    Weak,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DayMasterStrength {
    pub score: i32,
    pub label: DayMasterStrengthLabel,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChartInteractionKind {
    BranchClash,
    BranchHarmony,
    BranchHarm,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChartInteraction {
    pub kind: ChartInteractionKind,
    pub participants: Vec<String>,
    pub summary_vi: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TenGodDistribution {
    pub ty_kien: u8,
    pub kiep_tai: u8,
    pub thuc_than: u8,
    pub thuong_quan: u8,
    pub chinh_tai: u8,
    pub thien_tai: u8,
    pub chinh_quan: u8,
    pub that_sat: u8,
    pub chinh_an: u8,
    pub thien_an: u8,
}

impl TenGodDistribution {
    fn add(&mut self, label: ThapThanLabel) {
        match label {
            ThapThanLabel::TyKien => self.ty_kien += 1,
            ThapThanLabel::KiepTai => self.kiep_tai += 1,
            ThapThanLabel::ThucThan => self.thuc_than += 1,
            ThapThanLabel::ThuongQuan => self.thuong_quan += 1,
            ThapThanLabel::ChinhTai => self.chinh_tai += 1,
            ThapThanLabel::ThienTai => self.thien_tai += 1,
            ThapThanLabel::ChinhQuan => self.chinh_quan += 1,
            ThapThanLabel::ThatSat => self.that_sat += 1,
            ThapThanLabel::ChinhAn => self.chinh_an += 1,
            ThapThanLabel::ThienAn => self.thien_an += 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaziAnalysisReport {
    pub element_distribution: ElementDistribution,
    pub day_master_strength: DayMasterStrength,
    pub interactions: Vec<ChartInteraction>,
    pub ten_god_distribution: TenGodDistribution,
}

pub fn analyze_bazi_chart(chart: &BaziChart) -> BaziAnalysisReport {
    let element_distribution = compute_element_distribution(chart);
    let ten_god_distribution = compute_ten_god_distribution(chart);
    let interactions = detect_chart_interactions(chart);
    let day_master_strength =
        evaluate_day_master_strength(chart, &element_distribution, &ten_god_distribution);

    BaziAnalysisReport {
        element_distribution,
        day_master_strength,
        interactions,
        ten_god_distribution,
    }
}

pub fn compute_element_distribution(chart: &BaziChart) -> ElementDistribution {
    let mut distribution = ElementDistribution {
        moc: 0,
        hoa: 0,
        tho: 0,
        kim: 0,
        thuy: 0,
    };

    for pillar in &chart.pillars {
        if let Ok(stem) = crate::almanac::types::HeavenlyStem::try_from(pillar.can_chi.can.as_str()) {
            distribution.add(stem.element(), 100);
        }

        for hidden in &pillar.hidden_stems {
            if let Some(element) = hidden_element(hidden) {
                distribution.add(element, hidden.strength as u16);
            }
        }
    }

    distribution
}

pub fn compute_ten_god_distribution(chart: &BaziChart) -> TenGodDistribution {
    let mut distribution = TenGodDistribution {
        ty_kien: 0,
        kiep_tai: 0,
        thuc_than: 0,
        thuong_quan: 0,
        chinh_tai: 0,
        thien_tai: 0,
        chinh_quan: 0,
        that_sat: 0,
        chinh_an: 0,
        thien_an: 0,
    };

    for pillar in &chart.pillars {
        if let Some(relation) = &pillar.stem_relation_to_day_master {
            distribution.add(relation.label);
        }

        for hidden in &pillar.hidden_stems {
            if let Some(relation) = &hidden.ten_god_to_day_master {
                distribution.add(relation.label);
            }
        }
    }

    distribution
}

pub fn detect_chart_interactions(chart: &BaziChart) -> Vec<ChartInteraction> {
    let mut interactions = Vec::new();
    let pillars = &chart.pillars;

    for left_idx in 0..pillars.len() {
        for right_idx in (left_idx + 1)..pillars.len() {
            let left = &pillars[left_idx].can_chi.chi;
            let right = &pillars[right_idx].can_chi.chi;

            if let Some(kind) = detect_branch_relationship(left, right) {
                let summary_vi = match kind {
                    ChartInteractionKind::BranchClash => {
                        format!("{} và {} tạo thế xung.", left, right)
                    }
                    ChartInteractionKind::BranchHarmony => {
                        format!("{} và {} có quan hệ hợp.", left, right)
                    }
                    ChartInteractionKind::BranchHarm => {
                        format!("{} và {} có quan hệ hại.", left, right)
                    }
                };

                interactions.push(ChartInteraction {
                    kind,
                    participants: vec![left.clone(), right.clone()],
                    summary_vi,
                });
            }
        }
    }

    interactions
}

pub fn evaluate_day_master_strength(
    chart: &BaziChart,
    elements: &ElementDistribution,
    ten_gods: &TenGodDistribution,
) -> DayMasterStrength {
    let day_master_stem =
        crate::almanac::types::HeavenlyStem::try_from(chart.day_master.can.as_str()).expect("day master stem");
    let same_element = element_total(elements, day_master_stem.element()) as i32;
    let supportive = match day_master_stem.element() {
        FiveElement::Moc => elements.thuy as i32,
        FiveElement::Hoa => elements.moc as i32,
        FiveElement::Tho => elements.hoa as i32,
        FiveElement::Kim => elements.tho as i32,
        FiveElement::Thuy => elements.kim as i32,
    };
    let draining = match day_master_stem.element() {
        FiveElement::Moc => elements.hoa as i32 + elements.kim as i32,
        FiveElement::Hoa => elements.tho as i32 + elements.thuy as i32,
        FiveElement::Tho => elements.kim as i32 + elements.moc as i32,
        FiveElement::Kim => elements.thuy as i32 + elements.hoa as i32,
        FiveElement::Thuy => elements.moc as i32 + elements.tho as i32,
    };

    let peer_support = ten_gods.ty_kien as i32 + ten_gods.kiep_tai as i32;
    let resource_support = ten_gods.chinh_an as i32 + ten_gods.thien_an as i32;

    let score = same_element + supportive + (peer_support + resource_support) * 10 - draining;
    let label = if score >= 180 {
        DayMasterStrengthLabel::Strong
    } else if score >= 120 {
        DayMasterStrengthLabel::Balanced
    } else {
        DayMasterStrengthLabel::Weak
    };

    let reasons = vec![
        format!("Ngũ hành đồng hành đạt {} điểm.", same_element),
        format!("Nguồn sinh trợ đạt {} điểm.", supportive),
        format!("Áp lực tiết/khắc đạt {} điểm.", draining),
        format!(
            "Tỷ kiếp + Ấn xuất hiện {} lần.",
            peer_support + resource_support
        ),
    ];

    DayMasterStrength {
        score,
        label,
        reasons,
    }
}

fn hidden_element(hidden: &HiddenStemEntry) -> Option<FiveElement> {
    hidden
        .stem_name
        .as_deref()
        .and_then(|name| crate::almanac::types::HeavenlyStem::try_from(name).ok())
        .map(|stem| stem.element())
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

fn detect_branch_relationship(left: &str, right: &str) -> Option<ChartInteractionKind> {
    let left_idx = crate::types::CHI.iter().position(|chi| chi == &left)?;
    let right_idx = crate::types::CHI.iter().position(|chi| chi == &right)?;

    if crate::almanac::xung_hop::luc_xung(left_idx) == right {
        return Some(ChartInteractionKind::BranchClash);
    }

    if crate::almanac::xung_hop::get_liu_he(left_idx) == right
        || crate::almanac::xung_hop::tam_hop(left_idx).contains(&right)
    {
        return Some(ChartInteractionKind::BranchHarmony);
    }

    if crate::almanac::xung_hop::get_xiang_hai(left_idx) == right {
        return Some(ChartInteractionKind::BranchHarm);
    }

    let _ = right_idx;
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{bazi::build_bazi_chart, BaziInput, types::VIETNAM_TIMEZONE};

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
            gender: None,
        })
        .expect("chart")
    }

    #[test]
    fn computes_element_distribution_from_visible_and_hidden_stems() {
        let chart = sample_chart();
        let distribution = compute_element_distribution(&chart);

        assert!(distribution.moc > 0);
        assert!(distribution.hoa > 0 || distribution.thuy > 0 || distribution.kim > 0);
    }

    #[test]
    fn computes_ten_god_distribution_for_chart() {
        let chart = sample_chart();
        let distribution = compute_ten_god_distribution(&chart);

        let total = distribution.ty_kien
            + distribution.kiep_tai
            + distribution.thuc_than
            + distribution.thuong_quan
            + distribution.chinh_tai
            + distribution.thien_tai
            + distribution.chinh_quan
            + distribution.that_sat
            + distribution.chinh_an
            + distribution.thien_an;

        assert!(total > 0);
    }

    #[test]
    fn evaluates_day_master_strength_with_reasoning() {
        let chart = sample_chart();
        let report = analyze_bazi_chart(&chart);

        assert!(!report.day_master_strength.reasons.is_empty());
        assert!(report.day_master_strength.score > 0);
    }

    #[test]
    fn detects_basic_chart_interactions() {
        let chart = sample_chart();
        let interactions = detect_chart_interactions(&chart);

        assert!(interactions.iter().all(|interaction| !interaction.summary_vi.is_empty()));
    }
}
