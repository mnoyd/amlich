use crate::almanac::types::{FiveElement, HeavenlyStem, RuleEvidence};
use crate::bazi::analysis::ElementDistribution;
use crate::types::CanChi;

use super::types::ElementResonanceMatrix;

/// Compute the Element Resonance Matrix.
///
/// Determines whether today's elemental energy supports or depletes the person's
/// element distribution, factoring in the seasonal context (month branch).
pub fn compute_element_resonance(
    day_canchi: &CanChi,
    month_chi: &str,
    element_dist: &ElementDistribution,
) -> ElementResonanceMatrix {
    let day_stem = HeavenlyStem::ALL[day_canchi.can_index];
    let day_element = day_stem.element();

    let season_factor = season_strength(day_element, month_chi);

    let entries: Vec<_> = FiveElement::ALL
        .iter()
        .map(|&personal_element| {
            let personal_score = element_score(element_dist, personal_element);
            let relation = element_relation(day_element, personal_element);
            let effective = relation * season_factor;
            let is_deficit = personal_score <= 15;

            super::types::ElementResonanceEntry {
                element: personal_element,
                personal_score,
                relation_to_day: relation,
                season_factor,
                effective_resonance: effective,
                is_deficit,
                day_helps_deficit: is_deficit && effective > 0.0,
            }
        })
        .collect();

    // Personal-score-weighted aggregate of effective_resonance. Contrasting
    // element distributions now produce different values, satisfying the
    // amlich-mwbp.5 acceptance criterion. Previous behavior summed
    // effective_resonance independently of personal_score, so two profiles
    // with the same day/month but different distributions collided.
    let total_personal_score: u16 = entries.iter().map(|e| e.personal_score).sum();
    let net_resonance: f32 = if total_personal_score == 0 {
        0.0
    } else {
        entries
            .iter()
            .map(|e| {
                e.effective_resonance * (e.personal_score as f32 / total_personal_score as f32)
            })
            .sum()
    };

    ElementResonanceMatrix {
        day_canchi: day_canchi.full.clone(),
        day_element,
        month_chi: month_chi.to_string(),
        season_factor,
        entries,
        net_resonance,
        resonance_policy_version: "v1-personal-weighted".to_string(),
        evidence: RuleEvidence {
            source_id: crate::sources::SOURCE_KHCBPPT.to_string(),
            method: "element-resonance-matrix".to_string(),
            profile: "baseline".to_string(),
        },
    }
}

// ── Element relation values (from BaziScoringMatrixSet::default()) ──────

/// Relation coefficient: how `day_element` affects `target_element`.
///  +1.0 = day generates target (sinh)
///  +0.8 = target generates day
///  +0.2 = same element
///  -0.6 = day controls target (khắc, mild)
///  -1.0 = target controls day (khắc, strong)
fn element_relation(day: FiveElement, target: FiveElement) -> f32 {
    use FiveElement::*;
    match (day, target) {
        (a, b) if a == b => 0.2,
        // Day generates target
        (Moc, Hoa) | (Hoa, Tho) | (Tho, Kim) | (Kim, Thuy) | (Thuy, Moc) => 1.0,
        // Target generates day
        (Hoa, Moc) | (Tho, Hoa) | (Kim, Tho) | (Thuy, Kim) | (Moc, Thuy) => 0.8,
        // Day controls target
        (Moc, Tho) | (Hoa, Kim) | (Tho, Thuy) | (Kim, Moc) | (Thuy, Hoa) => -0.6,
        // Target controls day
        (Tho, Moc) | (Kim, Hoa) | (Thuy, Tho) | (Moc, Kim) | (Hoa, Thuy) => -1.0,
        _ => 0.0,
    }
}

// ── Season strength (from BaziScoringMatrixSet::default()) ──────────────

/// How strong `element` is during the month indicated by `month_chi`.
fn season_strength(element: FiveElement, month_chi: &str) -> f32 {
    use FiveElement::*;
    match (element, month_chi) {
        (Moc, "Dần") | (Moc, "Mão") => 1.0,
        (Moc, "Thìn") => 0.6,
        (Moc, "Hợi") => 0.5,
        (Moc, "Tỵ") => 0.3,
        (Moc, "Tý") | (Moc, "Sửu") | (Moc, "Ngọ") | (Moc, "Mùi") => 0.2,
        (Moc, "Thân") | (Moc, "Dậu") | (Moc, "Tuất") => 0.1,

        (Hoa, "Tỵ") | (Hoa, "Ngọ") => 1.0,
        (Hoa, "Mùi") => 0.6,
        (Hoa, "Mão") => 0.5,
        (Hoa, "Dần") => 0.4,
        (Hoa, "Thìn") => 0.3,
        (Hoa, "Thân") | (Hoa, "Tuất") => 0.2,
        (Hoa, "Tý") | (Hoa, "Sửu") | (Hoa, "Dậu") | (Hoa, "Hợi") => 0.1,

        (Tho, "Sửu") | (Tho, "Thìn") | (Tho, "Mùi") | (Tho, "Tuất") => 0.8,
        (Tho, "Tỵ") | (Tho, "Ngọ") => 0.3,
        (Tho, "Tý") | (Tho, "Dần") | (Tho, "Mão") | (Tho, "Thân") | (Tho, "Dậu") | (Tho, "Hợi") => {
            0.2
        }

        (Kim, "Thân") | (Kim, "Dậu") => 1.0,
        (Kim, "Tuất") => 0.5,
        (Kim, "Sửu") => 0.3,
        (Kim, "Tý") | (Kim, "Thìn") | (Kim, "Mùi") | (Kim, "Hợi") => 0.2,
        (Kim, "Dần") | (Kim, "Mão") | (Kim, "Tỵ") | (Kim, "Ngọ") => 0.1,

        (Thuy, "Tý") | (Thuy, "Hợi") => 1.0,
        (Thuy, "Sửu") | (Thuy, "Thân") | (Thuy, "Dậu") => 0.5,
        (Thuy, "Thìn") => 0.3,
        (Thuy, "Dần") | (Thuy, "Mão") | (Thuy, "Mùi") | (Thuy, "Tuất") => 0.2,
        (Thuy, "Tỵ") | (Thuy, "Ngọ") => 0.1,

        _ => 0.2, // fallback
    }
}

fn element_score(dist: &ElementDistribution, element: FiveElement) -> u16 {
    match element {
        FiveElement::Moc => dist.moc,
        FiveElement::Hoa => dist.hoa,
        FiveElement::Tho => dist.tho,
        FiveElement::Kim => dist.kim,
        FiveElement::Thuy => dist.thuy,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn balanced_dist() -> ElementDistribution {
        ElementDistribution {
            moc: 20,
            hoa: 20,
            tho: 20,
            kim: 20,
            thuy: 20,
        }
    }

    #[test]
    fn matrix_has_5_entries() {
        let day = CanChi::new(0, 2); // Giáp Dần
        let matrix = compute_element_resonance(&day, "Dần", &balanced_dist());
        assert_eq!(matrix.entries.len(), 5);
    }

    #[test]
    fn day_element_is_stem_element() {
        // Giáp = Mộc
        let day = CanChi::new(0, 0);
        let matrix = compute_element_resonance(&day, "Tý", &balanced_dist());
        assert_eq!(matrix.day_element, FiveElement::Moc);

        // Bính = Hỏa
        let day2 = CanChi::new(2, 0);
        let matrix2 = compute_element_resonance(&day2, "Tý", &balanced_dist());
        assert_eq!(matrix2.day_element, FiveElement::Hoa);
    }

    #[test]
    fn season_factor_moc_in_spring_is_peak() {
        let day = CanChi::new(0, 2); // Giáp Dần (Mộc)
        let matrix = compute_element_resonance(&day, "Dần", &balanced_dist());
        assert_eq!(matrix.season_factor, 1.0);
    }

    #[test]
    fn season_factor_moc_in_autumn_is_low() {
        let day = CanChi::new(0, 0); // Giáp (Mộc)
        let matrix = compute_element_resonance(&day, "Dậu", &balanced_dist());
        assert_eq!(matrix.season_factor, 0.1);
    }

    #[test]
    fn same_element_relation_is_positive() {
        assert!(element_relation(FiveElement::Moc, FiveElement::Moc) > 0.0);
    }

    #[test]
    fn generation_relation_is_strongly_positive() {
        // Mộc sinh Hỏa
        assert_eq!(element_relation(FiveElement::Moc, FiveElement::Hoa), 1.0);
    }

    #[test]
    fn control_relation_is_negative() {
        // Mộc khắc Thổ
        assert!(element_relation(FiveElement::Moc, FiveElement::Tho) < 0.0);
    }

    #[test]
    fn deficit_element_is_flagged() {
        let dist = ElementDistribution {
            moc: 5,
            hoa: 30,
            tho: 25,
            kim: 20,
            thuy: 20,
        };
        let day = CanChi::new(8, 0); // Nhâm (Thủy), Thủy sinh Mộc
        let matrix = compute_element_resonance(&day, "Tý", &dist);
        let moc_entry = matrix
            .entries
            .iter()
            .find(|e| e.element == FiveElement::Moc)
            .unwrap();
        assert!(moc_entry.is_deficit);
        // Thủy → Mộc = generation (1.0), so day_helps_deficit should be true
        assert!(moc_entry.day_helps_deficit);
    }

    #[test]
    fn no_deficit_when_score_above_threshold() {
        let dist = balanced_dist();
        let day = CanChi::new(0, 0);
        let matrix = compute_element_resonance(&day, "Tý", &dist);
        for entry in &matrix.entries {
            assert!(!entry.is_deficit);
        }
    }

    #[test]
    fn net_resonance_is_personal_weighted_aggregate() {
        // amlich-mwbp.5 regression: net_resonance must be weighted by
        // personal_score so contrasting distributions produce different
        // values. The previous implementation was a flat sum of
        // effective_resonance, which was independent of the personal
        // distribution.
        let day = CanChi::new(0, 0); // Giáp Tý
        let dist_moc_heavy = ElementDistribution {
            moc: 50,
            hoa: 10,
            tho: 10,
            kim: 10,
            thuy: 20,
        };
        let dist_kim_heavy = ElementDistribution {
            moc: 10,
            hoa: 10,
            tho: 10,
            kim: 50,
            thuy: 20,
        };

        let m_moc = compute_element_resonance(&day, "Dần", &dist_moc_heavy);
        let m_kim = compute_element_resonance(&day, "Dần", &dist_kim_heavy);

        assert_ne!(
            m_moc.net_resonance, m_kim.net_resonance,
            "contrasting distributions must produce different net_resonance"
        );

        // Sanity: weighted average matches the formula
        let total: u16 = m_moc.entries.iter().map(|e| e.personal_score).sum();
        let expected: f32 = m_moc
            .entries
            .iter()
            .map(|e| e.effective_resonance * (e.personal_score as f32 / total as f32))
            .sum();
        assert!(
            (m_moc.net_resonance - expected).abs() < 0.001,
            "net_resonance {} must equal weighted aggregate {}",
            m_moc.net_resonance,
            expected
        );
    }

    #[test]
    fn net_resonance_policy_version_is_set() {
        let day = CanChi::new(0, 0);
        let matrix = compute_element_resonance(&day, "Dần", &balanced_dist());
        assert!(
            !matrix.resonance_policy_version.is_empty(),
            "resonance_policy_version must be documented"
        );
        assert!(
            matrix.resonance_policy_version.contains("personal"),
            "resonance_policy_version must indicate personal weighting; got {}",
            matrix.resonance_policy_version
        );
    }

    #[test]
    fn matrix_serializes_to_json() {
        let day = CanChi::new(0, 0);
        let matrix = compute_element_resonance(&day, "Tý", &balanced_dist());
        let json = serde_json::to_string(&matrix).expect("should serialize");
        assert!(json.contains("\"day_element\""));
        assert!(json.contains("\"entries\""));
        assert!(json.contains("\"net_resonance\""));
    }

    #[test]
    fn evidence_is_set() {
        let day = CanChi::new(0, 0);
        let matrix = compute_element_resonance(&day, "Tý", &balanced_dist());
        assert_eq!(matrix.evidence.source_id, "khcbppt");
        assert_eq!(matrix.evidence.method, "element-resonance-matrix");
    }
}
