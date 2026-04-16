/// Cửu Diệu (九曜) — Nine Star personal fortune cycle
///
/// A 9-star cycle determining personal fortune by year. Each star governs a year
/// based on the person's lunar age and gender. Male and female use different
/// lookup tables.
///
/// **Source:** Buddhist/Indian astronomical tradition (宿曜道 Sukuyōdō), NOT in KHCBPPT
/// **Decision:** DEC-0016
use serde::{Deserialize, Serialize};

use super::tu_menh::Gender;
use super::types::RuleEvidence;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CuuDieuQuality {
    /// Auspicious (Thái Dương, Thái Âm, Mộc Đức)
    Cat,
    /// Neutral (Thổ Tú, Thủy Diệu, Vân Hớn)
    Trung,
    /// Inauspicious (La Hầu, Thái Bạch, Kế Đô)
    Hung,
}

#[derive(Debug, Clone, Copy)]
struct StarInfo {
    name: &'static str,
    quality: CuuDieuQuality,
    element: &'static str,
}

/// Male lookup table (thuận — forward through base cycle)
/// Index 0 = remainder 1, index 8 = remainder 9
const MALE_STARS: [StarInfo; 9] = [
    StarInfo {
        name: "La Hầu",
        quality: CuuDieuQuality::Hung,
        element: "Kim",
    },
    StarInfo {
        name: "Thổ Tú",
        quality: CuuDieuQuality::Trung,
        element: "Thổ",
    },
    StarInfo {
        name: "Thủy Diệu",
        quality: CuuDieuQuality::Trung,
        element: "Thủy",
    },
    StarInfo {
        name: "Thái Bạch",
        quality: CuuDieuQuality::Hung,
        element: "Kim",
    },
    StarInfo {
        name: "Thái Dương",
        quality: CuuDieuQuality::Cat,
        element: "Hỏa",
    },
    StarInfo {
        name: "Vân Hớn",
        quality: CuuDieuQuality::Trung,
        element: "Hỏa",
    },
    StarInfo {
        name: "Kế Đô",
        quality: CuuDieuQuality::Hung,
        element: "Thổ",
    },
    StarInfo {
        name: "Thái Âm",
        quality: CuuDieuQuality::Cat,
        element: "Thủy",
    },
    StarInfo {
        name: "Mộc Đức",
        quality: CuuDieuQuality::Cat,
        element: "Mộc",
    },
];

/// Female lookup table (specific mapping, not simple reverse)
/// Index 0 = remainder 1, index 8 = remainder 9
const FEMALE_STARS: [StarInfo; 9] = [
    StarInfo {
        name: "Kế Đô",
        quality: CuuDieuQuality::Hung,
        element: "Thổ",
    },
    StarInfo {
        name: "Vân Hớn",
        quality: CuuDieuQuality::Trung,
        element: "Hỏa",
    },
    StarInfo {
        name: "Mộc Đức",
        quality: CuuDieuQuality::Cat,
        element: "Mộc",
    },
    StarInfo {
        name: "Thái Âm",
        quality: CuuDieuQuality::Cat,
        element: "Thủy",
    },
    StarInfo {
        name: "Thổ Tú",
        quality: CuuDieuQuality::Trung,
        element: "Thổ",
    },
    StarInfo {
        name: "La Hầu",
        quality: CuuDieuQuality::Hung,
        element: "Kim",
    },
    StarInfo {
        name: "Thái Dương",
        quality: CuuDieuQuality::Cat,
        element: "Hỏa",
    },
    StarInfo {
        name: "Thái Bạch",
        quality: CuuDieuQuality::Hung,
        element: "Kim",
    },
    StarInfo {
        name: "Thủy Diệu",
        quality: CuuDieuQuality::Trung,
        element: "Thủy",
    },
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CuuDieuResult {
    /// 1-based star index (1-9)
    pub star_index: u8,
    pub star_name: String,
    pub quality: CuuDieuQuality,
    /// True when star is La Hầu, Kế Đô, or Thái Bạch (the 3 "sao hạn")
    pub is_han: bool,
    pub element: String,
    pub tuoi_mu: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<RuleEvidence>,
}

/// Compute Cửu Diệu star for a person in a given year.
///
/// # Arguments
/// * `birth_lunar_year` — Lunar birth year
/// * `current_lunar_year` — Current lunar year
/// * `gender` — Male or Female (determines lookup table)
pub fn compute_cuu_dieu(
    birth_lunar_year: i32,
    current_lunar_year: i32,
    gender: Gender,
) -> CuuDieuResult {
    let tuoi_mu = current_lunar_year - birth_lunar_year + 1;
    let mut remainder = (tuoi_mu.rem_euclid(9)) as usize;
    if remainder == 0 {
        remainder = 9;
    }

    let stars = match gender {
        Gender::Male => &MALE_STARS,
        Gender::Female => &FEMALE_STARS,
    };

    let star = &stars[remainder - 1]; // 1-based → 0-based

    let is_han = matches!(star.name, "La Hầu" | "Kế Đô" | "Thái Bạch");

    CuuDieuResult {
        star_index: remainder as u8,
        star_name: star.name.to_string(),
        quality: star.quality,
        is_han,
        element: star.element.to_string(),
        tuoi_mu,
        evidence: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn male_age_1_is_la_hau() {
        let r = compute_cuu_dieu(2000, 2000, Gender::Male);
        assert_eq!(r.tuoi_mu, 1);
        assert_eq!(r.star_index, 1);
        assert_eq!(r.star_name, "La Hầu");
        assert_eq!(r.quality, CuuDieuQuality::Hung);
        assert!(r.is_han);
    }

    #[test]
    fn male_age_5_is_thai_duong() {
        let r = compute_cuu_dieu(2000, 2004, Gender::Male);
        assert_eq!(r.star_index, 5);
        assert_eq!(r.star_name, "Thái Dương");
        assert_eq!(r.quality, CuuDieuQuality::Cat);
        assert!(!r.is_han);
    }

    #[test]
    fn male_age_9_is_moc_duc() {
        let r = compute_cuu_dieu(2000, 2008, Gender::Male);
        assert_eq!(r.star_index, 9);
        assert_eq!(r.star_name, "Mộc Đức");
        assert_eq!(r.quality, CuuDieuQuality::Cat);
        assert!(!r.is_han);
    }

    #[test]
    fn female_age_1_is_ke_do() {
        let r = compute_cuu_dieu(2000, 2000, Gender::Female);
        assert_eq!(r.star_index, 1);
        assert_eq!(r.star_name, "Kế Đô");
        assert_eq!(r.quality, CuuDieuQuality::Hung);
        assert!(r.is_han);
    }

    #[test]
    fn female_age_7_is_thai_duong() {
        let r = compute_cuu_dieu(2000, 2006, Gender::Female);
        assert_eq!(r.star_index, 7);
        assert_eq!(r.star_name, "Thái Dương");
        assert_eq!(r.quality, CuuDieuQuality::Cat);
        assert!(!r.is_han);
    }

    #[test]
    fn cycle_repeats_every_9_years() {
        let r1 = compute_cuu_dieu(1990, 1990, Gender::Male); // age 1
        let r2 = compute_cuu_dieu(1990, 1999, Gender::Male); // age 10, 10%9=1
        assert_eq!(r1.star_name, r2.star_name);
    }

    #[test]
    fn male_thai_bach_is_han() {
        let r = compute_cuu_dieu(2000, 2003, Gender::Male); // age 4 → Thái Bạch
        assert_eq!(r.star_name, "Thái Bạch");
        assert!(r.is_han);
    }

    #[test]
    fn evidence_defaults_to_none() {
        let r = compute_cuu_dieu(2000, 2000, Gender::Male);
        assert!(r.evidence.is_none());
    }
}
