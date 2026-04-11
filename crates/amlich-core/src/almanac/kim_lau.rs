/// Kim Lâu (金樓) — Golden Tower age taboo
///
/// An age-based taboo system marking certain ages as inauspicious for marriage
/// and house construction. Formula: tuổi mụ mod 9, remainders 1/3/6/8 = Kim Lâu.
///
/// **Source:** Ngọc Hạp Ký (玉匣記), NOT in KHCBPPT
/// **Decision:** DEC-0015

use serde::{Deserialize, Serialize};

use super::types::RuleEvidence;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KimLauCategory {
    /// Remainder 1 — harms the person themselves (most severe)
    Than,
    /// Remainder 3 — harms the spouse
    The,
    /// Remainder 6 — harms children
    Tu,
    /// Remainder 8 — harms livestock/property (lightest)
    Suc,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KimLauResult {
    pub in_kim_lau: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<KimLauCategory>,
    pub remainder: u8,
    pub tuoi_mu: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<RuleEvidence>,
}

/// Compute Kim Lâu status for a person.
///
/// # Arguments
/// * `birth_lunar_year` — Lunar birth year
/// * `current_lunar_year` — Current lunar year
pub fn compute_kim_lau(birth_lunar_year: i32, current_lunar_year: i32) -> KimLauResult {
    let tuoi_mu = current_lunar_year - birth_lunar_year + 1;
    let remainder = (tuoi_mu.rem_euclid(9)) as u8;
    // rem_euclid(9) returns 0 for multiples of 9; we treat 0 as 9
    let remainder = if remainder == 0 { 9 } else { remainder };

    let category = match remainder {
        1 => Some(KimLauCategory::Than),
        3 => Some(KimLauCategory::The),
        6 => Some(KimLauCategory::Tu),
        8 => Some(KimLauCategory::Suc),
        _ => None,
    };

    KimLauResult {
        in_kim_lau: category.is_some(),
        category,
        remainder,
        tuoi_mu,
        evidence: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kim_lau_than_at_remainder_1() {
        // tuoi_mu = 1 → remainder 1 → Than
        let r = compute_kim_lau(2000, 2000);
        assert_eq!(r.tuoi_mu, 1);
        assert_eq!(r.remainder, 1);
        assert!(r.in_kim_lau);
        assert_eq!(r.category, Some(KimLauCategory::Than));
    }

    #[test]
    fn kim_lau_the_at_remainder_3() {
        // tuoi_mu = 3 → remainder 3 → The
        let r = compute_kim_lau(2000, 2002);
        assert_eq!(r.tuoi_mu, 3);
        assert_eq!(r.remainder, 3);
        assert!(r.in_kim_lau);
        assert_eq!(r.category, Some(KimLauCategory::The));
    }

    #[test]
    fn kim_lau_tu_at_remainder_6() {
        // tuoi_mu = 6 → remainder 6 → Tu
        let r = compute_kim_lau(2000, 2005);
        assert_eq!(r.tuoi_mu, 6);
        assert!(r.in_kim_lau);
        assert_eq!(r.category, Some(KimLauCategory::Tu));
    }

    #[test]
    fn kim_lau_suc_at_remainder_8() {
        // tuoi_mu = 8 → remainder 8 → Suc
        let r = compute_kim_lau(2000, 2007);
        assert_eq!(r.tuoi_mu, 8);
        assert!(r.in_kim_lau);
        assert_eq!(r.category, Some(KimLauCategory::Suc));
    }

    #[test]
    fn safe_remainders() {
        // remainder 2 → safe
        let r = compute_kim_lau(2000, 2001);
        assert_eq!(r.remainder, 2);
        assert!(!r.in_kim_lau);
        assert_eq!(r.category, None);

        // remainder 9 (tuoi_mu=9) → safe
        let r = compute_kim_lau(2000, 2008);
        assert_eq!(r.tuoi_mu, 9);
        assert_eq!(r.remainder, 9);
        assert!(!r.in_kim_lau);
    }

    #[test]
    fn cycle_repeats_every_9_years() {
        let r1 = compute_kim_lau(1990, 2000); // tuoi_mu = 11, 11%9=2 safe
        let r2 = compute_kim_lau(1990, 2009); // tuoi_mu = 20, 20%9=2 safe
        assert_eq!(r1.remainder, r2.remainder);
    }

    #[test]
    fn evidence_defaults_to_none() {
        let r = compute_kim_lau(2000, 2000);
        assert!(r.evidence.is_none());
    }
}
