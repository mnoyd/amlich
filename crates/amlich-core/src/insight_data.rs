use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

const CANCHI_JSON: &str = include_str!("../data/canchi.json");
const TIET_KHI_JSON: &str = include_str!("../data/tiet-khi.json");
const TRUC_INSIGHT_JSON: &str = include_str!("../data/truc-insight.json");
const DAY_DEITY_INSIGHT_JSON: &str = include_str!("../data/day-deity-insight.json");
const NA_AM_INSIGHT_JSON: &str = include_str!("../data/na-am-insight.json");
const TEN_GODS_INSIGHT_JSON: &str = include_str!("../data/ten-gods-insight.json");
const TU_MENH_INSIGHT_JSON: &str = include_str!("../data/tu-menh-insight.json");
const DAI_VAN_INSIGHT_JSON: &str = include_str!("../data/dai-van-insight.json");

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

#[derive(Debug, Clone, Deserialize)]
pub struct TrucInsight {
    pub id: String,
    pub meaning: BilingualText,
    pub good_for: BilingualList,
    pub avoid_for: BilingualList,
}

#[derive(Debug, Deserialize)]
struct TrucInsightFile {
    truc: Vec<TrucInsight>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeityClassificationInsight {
    pub id: String,
    pub name: BilingualText,
    pub meaning: BilingualText,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeityInsight {
    pub name: String,
    pub classification: String,
    pub meaning: BilingualText,
}

#[derive(Debug, Deserialize)]
struct DayDeityInsightFile {
    classifications: Vec<DeityClassificationInsight>,
    deities: Vec<DeityInsight>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NaAmInsight {
    pub na_am: String,
    pub element: String,
    pub meaning: BilingualText,
}

#[derive(Debug, Deserialize)]
struct NaAmInsightFile {
    pairs: Vec<NaAmInsight>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TenGodsInsight {
    pub id: String,
    pub name: BilingualText,
    pub meaning: BilingualText,
}

#[derive(Debug, Deserialize)]
struct TenGodsInsightFile {
    gods: Vec<TenGodsInsight>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KuaGroupInsight {
    pub id: String,
    pub name: BilingualText,
    pub meaning: BilingualText,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KuaInsight {
    pub number: u8,
    pub trigram: BilingualText,
    pub direction: BilingualText,
    pub meaning: BilingualText,
}

#[derive(Debug, Deserialize)]
struct TuMenhInsightFile {
    groups: Vec<KuaGroupInsight>,
    kua: Vec<KuaInsight>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DaiVanDirectionInsight {
    pub id: String,
    pub name: BilingualText,
    pub meaning: BilingualText,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DaiVanPhasesInsight {
    pub meaning: BilingualText,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DaiVanElementInsight {
    pub element: String,
    pub meaning: BilingualText,
}

#[derive(Debug, Deserialize)]
struct DaiVanInsightFile {
    directions: Vec<DaiVanDirectionInsight>,
    phases: DaiVanPhasesInsight,
    elements: Vec<DaiVanElementInsight>,
}

static CANCHI_DATA: OnceLock<CanChiFile> = OnceLock::new();
static TIET_KHI_DATA: OnceLock<TietKhiFile> = OnceLock::new();
static TRUC_INSIGHT_DATA: OnceLock<Vec<TrucInsight>> = OnceLock::new();
static DAY_DEITY_INSIGHT_DATA: OnceLock<DayDeityInsightFile> = OnceLock::new();
static NA_AM_INSIGHT_DATA: OnceLock<Vec<NaAmInsight>> = OnceLock::new();
static TEN_GODS_INSIGHT_DATA: OnceLock<Vec<TenGodsInsight>> = OnceLock::new();
static TU_MENH_INSIGHT_DATA: OnceLock<TuMenhInsightFile> = OnceLock::new();
static DAI_VAN_INSIGHT_DATA: OnceLock<DaiVanInsightFile> = OnceLock::new();

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

pub fn all_truc_insights() -> &'static [TrucInsight] {
    TRUC_INSIGHT_DATA
        .get_or_init(|| {
            let parsed: TrucInsightFile = serde_json::from_str(TRUC_INSIGHT_JSON)
                .expect("Failed to parse data/truc-insight.json");
            parsed.truc
        })
        .as_slice()
}

pub fn find_truc_insight(name: &str) -> Option<&'static TrucInsight> {
    all_truc_insights().iter().find(|t| t.id == name)
}

fn day_deity_insight_data() -> &'static DayDeityInsightFile {
    DAY_DEITY_INSIGHT_DATA.get_or_init(|| {
        serde_json::from_str(DAY_DEITY_INSIGHT_JSON)
            .expect("Failed to parse data/day-deity-insight.json")
    })
}

pub fn find_deity_classification_insight(id: &str) -> Option<&'static DeityClassificationInsight> {
    day_deity_insight_data()
        .classifications
        .iter()
        .find(|c| c.id == id)
}

pub fn find_deity_insight(name: &str) -> Option<&'static DeityInsight> {
    day_deity_insight_data()
        .deities
        .iter()
        .find(|d| d.name == name)
}

pub fn all_na_am_insights() -> &'static [NaAmInsight] {
    NA_AM_INSIGHT_DATA
        .get_or_init(|| {
            let parsed: NaAmInsightFile = serde_json::from_str(NA_AM_INSIGHT_JSON)
                .expect("Failed to parse data/na-am-insight.json");
            parsed.pairs
        })
        .as_slice()
}

pub fn find_na_am_insight(na_am: &str) -> Option<&'static NaAmInsight> {
    all_na_am_insights().iter().find(|n| n.na_am == na_am)
}

pub fn all_ten_gods_insights() -> &'static [TenGodsInsight] {
    TEN_GODS_INSIGHT_DATA
        .get_or_init(|| {
            let parsed: TenGodsInsightFile = serde_json::from_str(TEN_GODS_INSIGHT_JSON)
                .expect("Failed to parse data/ten-gods-insight.json");
            parsed.gods
        })
        .as_slice()
}

pub fn find_ten_gods_insight(id: &str) -> Option<&'static TenGodsInsight> {
    all_ten_gods_insights().iter().find(|g| g.id == id)
}

fn tu_menh_insight_data() -> &'static TuMenhInsightFile {
    TU_MENH_INSIGHT_DATA.get_or_init(|| {
        serde_json::from_str(TU_MENH_INSIGHT_JSON)
            .expect("Failed to parse data/tu-menh-insight.json")
    })
}

pub fn all_kua_group_insights() -> &'static [KuaGroupInsight] {
    &tu_menh_insight_data().groups
}

pub fn all_kua_insights() -> &'static [KuaInsight] {
    &tu_menh_insight_data().kua
}

pub fn find_kua_insight(number: u8) -> Option<&'static KuaInsight> {
    all_kua_insights().iter().find(|k| k.number == number)
}

pub fn find_kua_group_insight(id: &str) -> Option<&'static KuaGroupInsight> {
    all_kua_group_insights().iter().find(|g| g.id == id)
}

fn dai_van_insight_data() -> &'static DaiVanInsightFile {
    DAI_VAN_INSIGHT_DATA.get_or_init(|| {
        serde_json::from_str(DAI_VAN_INSIGHT_JSON)
            .expect("Failed to parse data/dai-van-insight.json")
    })
}

pub fn all_dai_van_direction_insights() -> &'static [DaiVanDirectionInsight] {
    &dai_van_insight_data().directions
}

pub fn dai_van_phases_insight() -> &'static DaiVanPhasesInsight {
    &dai_van_insight_data().phases
}

pub fn all_dai_van_element_insights() -> &'static [DaiVanElementInsight] {
    &dai_van_insight_data().elements
}

pub fn find_dai_van_element_insight(element: &str) -> Option<&'static DaiVanElementInsight> {
    all_dai_van_element_insights()
        .iter()
        .find(|e| e.element == element)
}

pub fn find_dai_van_direction_insight(id: &str) -> Option<&'static DaiVanDirectionInsight> {
    all_dai_van_direction_insights().iter().find(|d| d.id == id)
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

    #[test]
    fn all_truc_insights_has_12_entries() {
        assert_eq!(super::all_truc_insights().len(), 12);
    }

    #[test]
    fn find_truc_insight_returns_entry() {
        let truc = super::find_truc_insight("Kiến");
        assert!(truc.is_some());
        assert!(!truc.unwrap().meaning.vi.is_empty());
    }

    #[test]
    fn all_na_am_insights_has_30_entries() {
        assert_eq!(super::all_na_am_insights().len(), 30);
    }

    #[test]
    fn find_na_am_insight_returns_entry() {
        let na_am = super::find_na_am_insight("Hải Trung Kim");
        assert!(na_am.is_some());
    }

    #[test]
    fn all_ten_gods_insights_has_10_entries() {
        assert_eq!(super::all_ten_gods_insights().len(), 10);
    }

    #[test]
    fn find_deity_classification_returns_entry() {
        let cls = super::find_deity_classification_insight("HoangDao");
        assert!(cls.is_some());
    }

    #[test]
    fn find_deity_by_name_returns_entry() {
        let deity = super::find_deity_insight("Thanh Long");
        assert!(deity.is_some());
    }

    #[test]
    fn tu_menh_kua_insights_has_8_entries() {
        assert_eq!(super::all_kua_insights().len(), 8);
    }

    #[test]
    fn find_kua_insight_by_number() {
        let kua = super::find_kua_insight(1);
        assert!(kua.is_some());
    }

    #[test]
    fn tu_menh_group_insights_has_2_entries() {
        assert_eq!(super::all_kua_group_insights().len(), 2);
    }

    #[test]
    fn dai_van_element_insights_has_5_entries() {
        assert_eq!(super::all_dai_van_element_insights().len(), 5);
    }
}
