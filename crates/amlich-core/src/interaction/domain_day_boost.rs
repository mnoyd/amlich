use crate::almanac::types::{DayDeityClassification, DayFortune, RuleEvidence};
use crate::bazi::scoring::BaziDomainScores;

use super::types::{DomainDayBoostEntry, DomainDayBoostMatrix};

/// Compute the Domain-Day Boost Matrix.
///
/// For each of 5 life domains, applies day-level modifiers (stars, trực, thần)
/// and yearly hạn penalty to the person's base Bazi domain scores.
pub fn compute_domain_day_boost(
    day_fortune: &DayFortune,
    domain_scores: &BaziDomainScores,
    han_active_count: u8,
) -> DomainDayBoostMatrix {
    let star_mod = compute_star_modifier(day_fortune);
    let truc_mod = compute_truc_modifier(day_fortune);
    let than_mod = compute_than_modifier(day_fortune);
    let day_modifier = star_mod + truc_mod + than_mod;

    let han_penalty = match han_active_count {
        0 => 0.0,
        1 => -0.05,
        2 => -0.15,
        _ => -0.25,
    };

    let domains = [
        ("career", domain_scores.career.score),
        ("wealth", domain_scores.wealth.score),
        ("relationship", domain_scores.relationship.score),
        ("health", domain_scores.health.score),
        ("timing", domain_scores.timing.score),
    ];

    let entries = domains
        .iter()
        .map(|(name, base)| {
            let base_f = *base as f32;
            let boosted = (base_f * (1.0 + day_modifier + han_penalty)).clamp(0.0, 100.0);
            DomainDayBoostEntry {
                domain: name.to_string(),
                base_score: base_f,
                day_modifier,
                han_penalty,
                boosted_score: boosted,
            }
        })
        .collect();

    DomainDayBoostMatrix {
        day_canchi: format!(
            "{} {}",
            day_fortune.day_element.can_element, day_fortune.day_element.chi_element
        ),
        entries,
        evidence: RuleEvidence {
            source_id: "khcbppt".to_string(),
            method: "domain-day-boost-matrix".to_string(),
            profile: "baseline".to_string(),
        },
    }
}

/// Star modifier: positive for many good stars, negative for many bad stars.
fn compute_star_modifier(fortune: &DayFortune) -> f32 {
    let cat = fortune.stars.cat_tinh.len() as f32;
    let sat = fortune.stars.sat_tinh.len() as f32;
    (cat - sat) * 0.02 // each net good star adds +2%
}

/// Trực modifier based on quality.
fn compute_truc_modifier(fortune: &DayFortune) -> f32 {
    match fortune.truc.quality.as_str() {
        "cat" => 0.05,
        "hung" => -0.05,
        _ => 0.0, // "binh"
    }
}

/// Thần (deity) modifier: Hoàng Đạo positive, Hắc Đạo negative.
fn compute_than_modifier(fortune: &DayFortune) -> f32 {
    match &fortune.day_deity {
        Some(deity) => match deity.classification {
            DayDeityClassification::HoangDao => 0.05,
            DayDeityClassification::HacDao => -0.03,
        },
        None => 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::almanac::types::*;
    use crate::bazi::scoring::{BaziDomainScore, BaziDomainScores};

    fn make_domain_scores() -> BaziDomainScores {
        let score = |s: u8| BaziDomainScore {
            score: s,
            label: "moderate".to_string(),
            confidence: 0.7,
            evidence_level: "baseline".to_string(),
            contributors: vec![],
        };
        BaziDomainScores {
            career: score(60),
            wealth: score(50),
            relationship: score(70),
            health: score(55),
            timing: score(45),
        }
    }

    fn make_fortune(cat_count: usize, sat_count: usize, truc_quality: &str, hoang_dao: bool) -> DayFortune {
        DayFortune {
            ruleset_id: "test".to_string(),
            ruleset_version: "v1".to_string(),
            profile: "baseline".to_string(),
            day_element: DayElement {
                na_am: "test".to_string(),
                element: "Kim".to_string(),
                can_element: "Mộc".to_string(),
                chi_element: "Thổ".to_string(),
                evidence: None,
            },
            conflict: DayConflict {
                opposing_chi: "Tuất".to_string(),
                opposing_con_giap: "Tuất (Chó)".to_string(),
                tuoi_xung: vec![],
                sat_huong: "Nam".to_string(),
                evidence: None,
            },
            travel: TravelDirection {
                xuat_hanh_huong: "Đông".to_string(),
                tai_than: "Tây Nam".to_string(),
                hy_than: "Đông Bắc".to_string(),
                evidence: None,
            },
            stars: DayStars {
                cat_tinh: (0..cat_count).map(|i| format!("Star{i}")).collect(),
                sat_tinh: (0..sat_count).map(|i| format!("Sat{i}")).collect(),
                day_star: None,
                star_system: None,
                evidence: None,
                matched_rules: vec![],
            },
            day_deity: if hoang_dao {
                Some(DayDeity {
                    name: "Thanh Long".to_string(),
                    classification: DayDeityClassification::HoangDao,
                    evidence: None,
                })
            } else {
                Some(DayDeity {
                    name: "Bạch Hổ".to_string(),
                    classification: DayDeityClassification::HacDao,
                    evidence: None,
                })
            },
            taboos: vec![],
            xung_hop: XungHopResult {
                luc_xung: "Tuất".to_string(),
                tam_hop: vec![],
                tu_hanh_xung: vec![],
                liu_he: None,
                xiang_hai: None,
                xiang_xing: None,
            },
            truc: TrucInfo {
                index: 0,
                name: "Kiến".to_string(),
                quality: truc_quality.to_string(),
                evidence: None,
            },
            tang_can: None,
            ten_gods: None,
            tu_menh: None,
        }
    }

    #[test]
    fn matrix_has_5_domains() {
        let fortune = make_fortune(2, 1, "cat", true);
        let matrix = compute_domain_day_boost(&fortune, &make_domain_scores(), 0);
        assert_eq!(matrix.entries.len(), 5);
    }

    #[test]
    fn domain_names_are_correct() {
        let fortune = make_fortune(0, 0, "binh", true);
        let matrix = compute_domain_day_boost(&fortune, &make_domain_scores(), 0);
        let names: Vec<&str> = matrix.entries.iter().map(|e| e.domain.as_str()).collect();
        assert_eq!(names, ["career", "wealth", "relationship", "health", "timing"]);
    }

    #[test]
    fn good_day_boosts_scores() {
        // Many good stars, cat trực, hoàng đạo, no hạn
        let fortune = make_fortune(4, 0, "cat", true);
        let matrix = compute_domain_day_boost(&fortune, &make_domain_scores(), 0);
        for entry in &matrix.entries {
            assert!(
                entry.boosted_score >= entry.base_score,
                "{}: boosted {} should >= base {}",
                entry.domain,
                entry.boosted_score,
                entry.base_score
            );
        }
    }

    #[test]
    fn bad_day_reduces_scores() {
        // Many bad stars, hung trực, hắc đạo, 3 hạn
        let fortune = make_fortune(0, 4, "hung", false);
        let matrix = compute_domain_day_boost(&fortune, &make_domain_scores(), 3);
        for entry in &matrix.entries {
            assert!(
                entry.boosted_score <= entry.base_score,
                "{}: boosted {} should <= base {}",
                entry.domain,
                entry.boosted_score,
                entry.base_score
            );
        }
    }

    #[test]
    fn han_penalty_scales_with_count() {
        let fortune = make_fortune(0, 0, "binh", true);
        let scores = make_domain_scores();
        let m0 = compute_domain_day_boost(&fortune, &scores, 0);
        let m1 = compute_domain_day_boost(&fortune, &scores, 1);
        let m2 = compute_domain_day_boost(&fortune, &scores, 2);
        // More hạn → lower boosted scores
        assert!(m0.entries[0].boosted_score > m1.entries[0].boosted_score);
        assert!(m1.entries[0].boosted_score > m2.entries[0].boosted_score);
    }

    #[test]
    fn scores_are_clamped_0_100() {
        let fortune = make_fortune(0, 10, "hung", false);
        let matrix = compute_domain_day_boost(&fortune, &make_domain_scores(), 3);
        for entry in &matrix.entries {
            assert!(entry.boosted_score >= 0.0);
            assert!(entry.boosted_score <= 100.0);
        }
    }

    #[test]
    fn matrix_serializes_to_json() {
        let fortune = make_fortune(1, 1, "cat", true);
        let matrix = compute_domain_day_boost(&fortune, &make_domain_scores(), 0);
        let json = serde_json::to_string(&matrix).expect("should serialize");
        assert!(json.contains("\"domain\""));
        assert!(json.contains("\"boosted_score\""));
    }

    #[test]
    fn evidence_is_set() {
        let fortune = make_fortune(0, 0, "binh", true);
        let matrix = compute_domain_day_boost(&fortune, &make_domain_scores(), 0);
        assert_eq!(matrix.evidence.source_id, "khcbppt");
        assert_eq!(matrix.evidence.method, "domain-day-boost-matrix");
    }
}
