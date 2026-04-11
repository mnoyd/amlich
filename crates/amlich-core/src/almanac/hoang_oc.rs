/// Hoàng Ốc (荒屋) — Desolate House construction age taboo
///
/// A 6-position cycle determining whether a person's current lunar age is
/// auspicious or inauspicious for house construction.
/// Formula: digit_sum(tuổi mụ) mod 6.
///
/// **Source:** Vietnamese folk tradition (dân gian), NOT in KHCBPPT
/// **Decision:** DEC-0015

use serde::{Deserialize, Serialize};

use super::types::RuleEvidence;

pub const HOANG_OC_NAMES: [&str; 6] = [
    "Lục Hoàng Ốc", // position 0 (mod 6 = 0) → BAD
    "Nhất Cát",      // position 1 → GOOD
    "Nhị Nghi",      // position 2 → GOOD
    "Tam Địa Sát",   // position 3 → BAD
    "Tứ Tấn Tài",    // position 4 → GOOD
    "Ngũ Thọ Tử",    // position 5 → BAD
];

pub const HOANG_OC_GOOD: [bool; 6] = [
    false, // 0: Lục Hoàng Ốc
    true,  // 1: Nhất Cát
    true,  // 2: Nhị Nghi
    false, // 3: Tam Địa Sát
    true,  // 4: Tứ Tấn Tài
    false, // 5: Ngũ Thọ Tử
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HoangOcResult {
    /// 0-based position in the 6-position cycle
    pub position: u8,
    pub position_name: String,
    pub is_good: bool,
    pub tuoi_mu: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<RuleEvidence>,
}

/// Sum the digits of a number until a single digit remains.
fn digit_sum(mut n: i32) -> i32 {
    n = n.abs();
    while n >= 10 {
        let mut sum = 0;
        let mut v = n;
        while v > 0 {
            sum += v % 10;
            v /= 10;
        }
        n = sum;
    }
    n
}

/// Compute Hoàng Ốc status for a person.
///
/// # Arguments
/// * `birth_lunar_year` — Lunar birth year
/// * `current_lunar_year` — Current lunar year
pub fn compute_hoang_oc(birth_lunar_year: i32, current_lunar_year: i32) -> HoangOcResult {
    let tuoi_mu = current_lunar_year - birth_lunar_year + 1;
    let ds = digit_sum(tuoi_mu);
    let position = (ds % 6) as usize;

    HoangOcResult {
        position: position as u8,
        position_name: HOANG_OC_NAMES[position].to_string(),
        is_good: HOANG_OC_GOOD[position],
        tuoi_mu,
        evidence: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digit_sum_single() {
        assert_eq!(digit_sum(5), 5);
    }

    #[test]
    fn digit_sum_multi() {
        assert_eq!(digit_sum(10), 1); // 1+0=1
        assert_eq!(digit_sum(29), 2); // 2+9=11 → 1+1=2
        assert_eq!(digit_sum(99), 9); // 9+9=18 → 1+8=9
    }

    #[test]
    fn anchor_age_10_is_nhat_cat() {
        // tuoi_mu=10, digit_sum=1, 1%6=1 → Nhất Cát (GOOD)
        let r = compute_hoang_oc(1990, 1999);
        assert_eq!(r.tuoi_mu, 10);
        assert_eq!(r.position, 1);
        assert_eq!(r.position_name, "Nhất Cát");
        assert!(r.is_good);
    }

    #[test]
    fn anchor_age_30_is_tam_dia_sat() {
        // tuoi_mu=30, digit_sum=3, 3%6=3 → Tam Địa Sát (BAD)
        let r = compute_hoang_oc(1990, 2019);
        assert_eq!(r.tuoi_mu, 30);
        assert_eq!(r.position, 3);
        assert!(!r.is_good);
    }

    #[test]
    fn anchor_age_60_is_hoang_oc() {
        // tuoi_mu=60, digit_sum=6, 6%6=0 → Lục Hoàng Ốc (BAD)
        let r = compute_hoang_oc(1966, 2025);
        assert_eq!(r.tuoi_mu, 60);
        assert_eq!(r.position, 0);
        assert_eq!(r.position_name, "Lục Hoàng Ốc");
        assert!(!r.is_good);
    }

    #[test]
    fn anchor_age_40_is_tu_tan_tai() {
        // tuoi_mu=40, digit_sum=4, 4%6=4 → Tứ Tấn Tài (GOOD)
        let r = compute_hoang_oc(1986, 2025);
        assert_eq!(r.tuoi_mu, 40);
        assert_eq!(r.position, 4);
        assert!(r.is_good);
    }

    #[test]
    fn anchor_age_50_is_ngu_tho_tu() {
        // tuoi_mu=50, digit_sum=5, 5%6=5 → Ngũ Thọ Tử (BAD)
        let r = compute_hoang_oc(1976, 2025);
        assert_eq!(r.tuoi_mu, 50);
        assert_eq!(r.position, 5);
        assert!(!r.is_good);
    }

    #[test]
    fn evidence_defaults_to_none() {
        let r = compute_hoang_oc(2000, 2025);
        assert!(r.evidence.is_none());
    }
}
