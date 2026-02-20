use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

const CANCHI_JSON: &str = include_str!("../../../data/canchi.json");
const TIET_KHI_JSON: &str = include_str!("../../../data/tiet-khi.json");

#[derive(Debug, Deserialize, Clone)]
pub struct BilingualText {
    pub vi: String,
    pub en: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BilingualList {
    pub vi: Vec<String>,
    pub en: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CanInfo {
    pub name: String,
    pub element: String,
    pub meaning: BilingualText,
    pub nature: BilingualText,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChiInfo {
    pub name: String,
    pub animal: BilingualText,
    pub element: String,
    pub meaning: BilingualText,
    pub hours: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ElementInfo {
    pub name: BilingualText,
    pub nature: BilingualText,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DayGuidance {
    pub good_for: BilingualList,
    pub avoid_for: BilingualList,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CanChiFile {
    can: Vec<CanInfo>,
    chi: Vec<ChiInfo>,
    elements: HashMap<String, ElementInfo>,
    day_guidance: HashMap<String, DayGuidance>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TietKhiInsight {
    pub id: String,
    pub name: BilingualText,
    pub longitude: i32,
    pub meaning: BilingualText,
    pub astronomy: BilingualText,
    pub agriculture: BilingualList,
    pub health: BilingualList,
    pub weather: BilingualText,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TietKhiFile {
    tiet_khi: Vec<TietKhiInsight>,
}

static CANCHI_DATA: OnceLock<CanChiFile> = OnceLock::new();
static TIET_KHI_DATA: OnceLock<TietKhiFile> = OnceLock::new();

fn canchi_data() -> &'static CanChiFile {
    CANCHI_DATA.get_or_init(|| {
        serde_json::from_str(CANCHI_JSON).expect("Failed to parse data/canchi.json")
    })
}

fn tiet_khi_data() -> &'static TietKhiFile {
    TIET_KHI_DATA.get_or_init(|| {
        serde_json::from_str(TIET_KHI_JSON).expect("Failed to parse data/tiet-khi.json")
    })
}

pub fn all_can() -> &'static [CanInfo] {
    &canchi_data().can
}

pub fn all_chi() -> &'static [ChiInfo] {
    &canchi_data().chi
}

pub fn all_elements() -> &'static HashMap<String, ElementInfo> {
    &canchi_data().elements
}

pub fn all_day_guidance() -> &'static HashMap<String, DayGuidance> {
    &canchi_data().day_guidance
}

pub fn all_tiet_khi_insights() -> &'static [TietKhiInsight] {
    &tiet_khi_data().tiet_khi
}

pub fn find_can(name: &str) -> Option<&'static CanInfo> {
    all_can().iter().find(|item| item.name == name)
}

pub fn find_chi(name: &str) -> Option<&'static ChiInfo> {
    all_chi().iter().find(|item| item.name == name)
}

pub fn get_day_guidance(chi_name: &str) -> Option<&'static DayGuidance> {
    all_day_guidance().get(chi_name)
}

pub fn find_tiet_khi_insight(term_name: &str) -> Option<&'static TietKhiInsight> {
    all_tiet_khi_insights()
        .iter()
        .find(|item| item.name.vi == term_name || item.name.en == term_name)
}

#[cfg(test)]
mod tests {
    use super::{
        all_can, all_chi, all_day_guidance, all_elements, all_tiet_khi_insights,
        find_tiet_khi_insight,
    };

    #[test]
    fn parses_canchi_collections() {
        assert_eq!(all_can().len(), 10);
        assert_eq!(all_chi().len(), 12);
        assert_eq!(all_elements().len(), 5);
        assert_eq!(all_day_guidance().len(), 12);
    }

    #[test]
    fn parses_all_tiet_khi() {
        assert_eq!(all_tiet_khi_insights().len(), 24);
    }

    #[test]
    fn lookup_tiet_khi_by_vi_name() {
        let term = find_tiet_khi_insight("Xuân Phân").expect("Xuân Phân should exist");
        assert_eq!(term.longitude, 0);
        assert!(!term.health.vi.is_empty());
    }
}
