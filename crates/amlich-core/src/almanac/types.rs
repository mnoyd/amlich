use serde::{Deserialize, Serialize};

/// Source attribution for a group of almanac rules.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceMeta {
    /// Classical source identifier (e.g. "khcbppt", "tam-menh-thong-hoi").
    pub source_id: String,
    /// Derivation method (e.g. "table-lookup", "bai-quyet", "jd-cycle").
    pub method: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StarSystem {
    NhiThapBatTu,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StarQuality {
    Cat,
    Hung,
    Binh,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DayElement {
    pub na_am: String,
    pub element: String,
    pub can_element: String,
    pub chi_element: String,
    pub evidence: Option<RuleEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DayConflict {
    pub opposing_chi: String,
    pub opposing_con_giap: String,
    pub tuoi_xung: Vec<String>,
    pub sat_huong: String,
    pub evidence: Option<RuleEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TravelDirection {
    pub xuat_hanh_huong: String,
    pub tai_than: String,
    pub hy_than: String,
    pub ky_than: Option<String>,
    pub evidence: Option<RuleEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleEvidence {
    pub source_id: String,
    pub method: String,
    pub profile: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DayDeityClassification {
    HoangDao,
    HacDao,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DayDeity {
    pub name: String,
    pub classification: DayDeityClassification,
    pub evidence: Option<RuleEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DayTaboo {
    pub rule_id: String,
    pub name: String,
    pub severity: String,
    pub reason: String,
    pub evidence: Option<RuleEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DayStar {
    pub system: StarSystem,
    pub index: usize,
    pub name: String,
    pub quality: StarQuality,
    pub evidence: Option<RuleEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StarRuleEvidence {
    pub name: String,
    pub quality: StarQuality,
    pub category: String,
    pub source_id: String,
    pub method: String,
    pub profile: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DayStars {
    pub cat_tinh: Vec<String>,
    pub sat_tinh: Vec<String>,
    pub day_star: Option<DayStar>,
    pub star_system: Option<StarSystem>,
    pub evidence: Option<RuleEvidence>,
    pub matched_rules: Vec<StarRuleEvidence>,
}

/// Thập nhị trực duty-star for the day.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrucInfo {
    /// Index 0–11 (0=Kiến, 1=Trừ, …, 11=Bế).
    pub index: usize,
    /// Vietnamese name (e.g. "Kiến", "Trừ").
    pub name: String,
    /// Auspicious quality: "cat" | "hung" | "binh".
    pub quality: String,
    pub evidence: Option<RuleEvidence>,
}

/// Xung/hợp relationships for the day branch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct XungHopResult {
    /// Directly opposing branch (lục xung).
    pub luc_xung: String,
    /// Three-harmony triad (tam hợp).
    pub tam_hop: Vec<String>,
    /// Four-clash square (tứ hành xung).
    pub tu_hanh_xung: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DayFortune {
    pub profile: String,
    pub day_element: DayElement,
    pub conflict: DayConflict,
    pub travel: TravelDirection,
    pub stars: DayStars,
    pub day_deity: Option<DayDeity>,
    pub taboos: Vec<DayTaboo>,
    pub xung_hop: XungHopResult,
    pub truc: TrucInfo,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_fortune_struct_exists() {
        let _ = std::mem::size_of::<DayFortune>();
    }

    #[test]
    fn core_sub_types_exist() {
        let _ = std::mem::size_of::<DayConflict>();
        let _ = std::mem::size_of::<TravelDirection>();
        let _ = std::mem::size_of::<DayStars>();
        let _ = std::mem::size_of::<DayElement>();
    }

    #[test]
    fn day_fortune_serializes() {
        let value = DayFortune {
            profile: "baseline".to_string(),
            day_element: DayElement {
                na_am: "Hải Trung Kim".to_string(),
                element: "Kim".to_string(),
                can_element: "Mộc".to_string(),
                chi_element: "Thổ".to_string(),
                evidence: None,
            },
            conflict: DayConflict {
                opposing_chi: "Tuất".to_string(),
                opposing_con_giap: "Tuất (Chó)".to_string(),
                tuoi_xung: vec!["Nhâm Tuất".to_string()],
                sat_huong: "Nam".to_string(),
                evidence: None,
            },
            travel: TravelDirection {
                xuat_hanh_huong: "Đông Nam".to_string(),
                tai_than: "Tây Nam".to_string(),
                hy_than: "Đông Bắc".to_string(),
                ky_than: None,
                evidence: None,
            },
            stars: DayStars {
                cat_tinh: vec!["Thiên Đức".to_string()],
                sat_tinh: vec!["Thiên Hình".to_string()],
                day_star: Some(DayStar {
                    system: StarSystem::NhiThapBatTu,
                    index: 0,
                    name: "Giác".to_string(),
                    quality: StarQuality::Cat,
                    evidence: None,
                }),
                star_system: Some(StarSystem::NhiThapBatTu),
                evidence: None,
                matched_rules: Vec::new(),
            },
            day_deity: Some(DayDeity {
                name: "Thanh Long".to_string(),
                classification: DayDeityClassification::HoangDao,
                evidence: None,
            }),
            taboos: vec![DayTaboo {
                rule_id: "tam_nuong".to_string(),
                name: "Tam Nương".to_string(),
                severity: "hard".to_string(),
                reason: "Ngày âm lịch 3 thuộc Tam Nương".to_string(),
                evidence: None,
            }],
            xung_hop: XungHopResult {
                luc_xung: "Tuất".to_string(),
                tam_hop: vec!["Dần".to_string(), "Ngọ".to_string(), "Tuất".to_string()],
                tu_hanh_xung: vec![
                    "Tý".to_string(),
                    "Mão".to_string(),
                    "Ngọ".to_string(),
                    "Dậu".to_string(),
                ],
            },
            truc: TrucInfo {
                index: 2,
                name: "Mãn".to_string(),
                quality: "hung".to_string(),
                evidence: None,
            },
        };

        let encoded = serde_json::to_string(&value).expect("serialize");
        let decoded: DayFortune = serde_json::from_str(&encoded).expect("deserialize");
        assert_eq!(decoded.profile, "baseline");
        assert_eq!(decoded.day_element.element, "Kim");
    }
}
