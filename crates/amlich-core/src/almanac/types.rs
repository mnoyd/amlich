use serde::{Deserialize, Serialize};

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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DayConflict {
    pub opposing_chi: String,
    pub opposing_con_giap: String,
    pub tuoi_xung: Vec<String>,
    pub sat_huong: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TravelDirection {
    pub xuat_hanh_huong: String,
    pub tai_than: String,
    pub hy_than: String,
    pub ky_than: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DayStar {
    pub system: StarSystem,
    pub index: usize,
    pub name: String,
    pub quality: StarQuality,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DayStars {
    pub cat_tinh: Vec<String>,
    pub sat_tinh: Vec<String>,
    pub day_star: Option<DayStar>,
    pub star_system: Option<StarSystem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DayFortune {
    pub profile: String,
    pub day_element: DayElement,
    pub conflict: DayConflict,
    pub travel: TravelDirection,
    pub stars: DayStars,
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
            },
            conflict: DayConflict {
                opposing_chi: "Tuất".to_string(),
                opposing_con_giap: "Tuất (Chó)".to_string(),
                tuoi_xung: vec!["Nhâm Tuất".to_string()],
                sat_huong: "Nam".to_string(),
            },
            travel: TravelDirection {
                xuat_hanh_huong: "Đông Nam".to_string(),
                tai_than: "Tây Nam".to_string(),
                hy_than: "Đông Bắc".to_string(),
                ky_than: None,
            },
            stars: DayStars {
                cat_tinh: vec!["Thiên Đức".to_string()],
                sat_tinh: vec!["Thiên Hình".to_string()],
                day_star: Some(DayStar {
                    system: StarSystem::NhiThapBatTu,
                    index: 0,
                    name: "Giác".to_string(),
                    quality: StarQuality::Cat,
                }),
                star_system: Some(StarSystem::NhiThapBatTu),
            },
        };

        let encoded = serde_json::to_string(&value).expect("serialize");
        let decoded: DayFortune = serde_json::from_str(&encoded).expect("deserialize");
        assert_eq!(decoded.profile, "baseline");
        assert_eq!(decoded.day_element.element, "Kim");
    }
}
