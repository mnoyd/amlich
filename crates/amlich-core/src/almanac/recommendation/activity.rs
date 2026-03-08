use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityCategory {
    Social,
    Business,
    Construction,
    Relocation,
    Ritual,
    Health,
    Legal,
    Maintenance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityId {
    Travel,
    MeetingSocial,
    OpeningStart,
    ContractAgreement,
    BusinessTrade,
    FinanceInvestment,
    ConstructionGroundbreaking,
    RepairRenovation,
    MoveRelocation,
    WeddingEngagement,
    LawsuitDispute,
    PrayerOffering,
    MedicalTreatment,
    BurialMemorial,
    CleaningPurging,
}

impl ActivityId {
    pub const ALL: [ActivityId; 15] = [
        ActivityId::Travel,
        ActivityId::MeetingSocial,
        ActivityId::OpeningStart,
        ActivityId::ContractAgreement,
        ActivityId::BusinessTrade,
        ActivityId::FinanceInvestment,
        ActivityId::ConstructionGroundbreaking,
        ActivityId::RepairRenovation,
        ActivityId::MoveRelocation,
        ActivityId::WeddingEngagement,
        ActivityId::LawsuitDispute,
        ActivityId::PrayerOffering,
        ActivityId::MedicalTreatment,
        ActivityId::BurialMemorial,
        ActivityId::CleaningPurging,
    ];

    pub fn category(self) -> ActivityCategory {
        match self {
            ActivityId::Travel => ActivityCategory::Social,
            ActivityId::MeetingSocial => ActivityCategory::Social,
            ActivityId::OpeningStart => ActivityCategory::Business,
            ActivityId::ContractAgreement => ActivityCategory::Business,
            ActivityId::BusinessTrade => ActivityCategory::Business,
            ActivityId::FinanceInvestment => ActivityCategory::Business,
            ActivityId::ConstructionGroundbreaking => ActivityCategory::Construction,
            ActivityId::RepairRenovation => ActivityCategory::Maintenance,
            ActivityId::MoveRelocation => ActivityCategory::Relocation,
            ActivityId::WeddingEngagement => ActivityCategory::Ritual,
            ActivityId::LawsuitDispute => ActivityCategory::Legal,
            ActivityId::PrayerOffering => ActivityCategory::Ritual,
            ActivityId::MedicalTreatment => ActivityCategory::Health,
            ActivityId::BurialMemorial => ActivityCategory::Ritual,
            ActivityId::CleaningPurging => ActivityCategory::Maintenance,
        }
    }

    pub fn labels(self) -> ActivityLabel {
        match self {
            ActivityId::Travel => ActivityLabel::new("Xuất hành", "Travel"),
            ActivityId::MeetingSocial => ActivityLabel::new("Gặp gỡ", "Meetings and social visits"),
            ActivityId::OpeningStart => ActivityLabel::new("Khai mở", "Opening and launching"),
            ActivityId::ContractAgreement => {
                ActivityLabel::new("Ký kết", "Contracts and agreements")
            }
            ActivityId::BusinessTrade => ActivityLabel::new("Giao dịch", "Business and trade"),
            ActivityId::FinanceInvestment => {
                ActivityLabel::new("Tài chính lớn", "Finance and investment")
            }
            ActivityId::ConstructionGroundbreaking => {
                ActivityLabel::new("Động thổ", "Groundbreaking and construction")
            }
            ActivityId::RepairRenovation => ActivityLabel::new("Tu sửa", "Repair and renovation"),
            ActivityId::MoveRelocation => {
                ActivityLabel::new("Nhập trạch", "Relocation and move-in")
            }
            ActivityId::WeddingEngagement => {
                ActivityLabel::new("Cưới hỏi", "Wedding and engagement")
            }
            ActivityId::LawsuitDispute => {
                ActivityLabel::new("Kiện tụng", "Litigation and disputes")
            }
            ActivityId::PrayerOffering => ActivityLabel::new("Cầu cúng", "Prayer and offerings"),
            ActivityId::MedicalTreatment => ActivityLabel::new("Chữa bệnh", "Medical treatment"),
            ActivityId::BurialMemorial => {
                ActivityLabel::new("An táng", "Burial and memorial rites")
            }
            ActivityId::CleaningPurging => {
                ActivityLabel::new("Dọn dẹp", "Cleaning and purification")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityLabel {
    pub vi: String,
    pub en: String,
}

impl ActivityLabel {
    pub fn new(vi: &str, en: &str) -> Self {
        Self {
            vi: vi.to_string(),
            en: en.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedActivity {
    pub activity_id: ActivityId,
    pub label: ActivityLabel,
    pub matched_alias: String,
}

pub fn normalize_activity_alias(input: &str) -> Option<NormalizedActivity> {
    let normalized = canonicalize_alias(input);

    let activity_id = match normalized.as_str() {
        "xuat hanh" | "xuat hanh xa" | "di lai" | "di chuyen" | "di xa" | "travel" => {
            ActivityId::Travel
        }

        "gap go" | "giao tiep" | "hoi hop" | "giai hoa" | "reconciliation" => {
            ActivityId::MeetingSocial
        }

        "khai truong" | "khai truong nho" | "khai mo" | "khoi dau" | "ra mat" | "grand opening"
        | "small openings" => ActivityId::OpeningStart,

        "ky hop dong" | "ky ket" | "signing contracts" | "signing" | "contract signing"
        | "hop dong" => ActivityId::ContractAgreement,

        "giao dich" | "business deals" | "business" | "trade" | "giao thuong" => {
            ActivityId::BusinessTrade
        }

        "cau tai" | "thu no" | "seeking wealth" | "tai chinh" | "dau tu" | "investment" => {
            ActivityId::FinanceInvestment
        }

        "dong tho" | "khoi cong" | "xay dung" | "groundbreaking" | "starting construction" => {
            ActivityId::ConstructionGroundbreaking
        }

        "tu sua" | "tu sua nha" | "bao tri" | "sua chua" | "tu sua noi bo" | "repairs"
        | "maintenance" | "home repairs" | "internal repairs" => ActivityId::RepairRenovation,

        "nhap trach" | "move in" | "moving in" | "di doi" | "chuyen nha" => {
            ActivityId::MoveRelocation
        }

        "cuoi hoi" | "wedding" | "dam hoi" | "dinh hon" => ActivityId::WeddingEngagement,

        "kien tung" | "tranh chap" | "litigation" | "lawsuit" => ActivityId::LawsuitDispute,

        "cau nguyen" | "cung te" | "cau cung" | "prayer" | "rituals" | "offerings" => {
            ActivityId::PrayerOffering
        }

        "chua benh" | "medical treatment" | "healing" => ActivityId::MedicalTreatment,

        "an tang" | "burial" | "tang le" | "memorial" => ActivityId::BurialMemorial,

        "don dep" | "tay ue" | "giai tru" | "purification" | "cleaning" | "clearing old things" => {
            ActivityId::CleaningPurging
        }

        _ => return None,
    };

    Some(NormalizedActivity {
        activity_id,
        label: activity_id.labels(),
        matched_alias: normalized,
    })
}

fn canonicalize_alias(input: &str) -> String {
    let lowered = input.trim().to_lowercase();
    let mut out = String::with_capacity(lowered.len());

    for ch in lowered.chars() {
        let mapped = match ch {
            'à' | 'á' | 'ạ' | 'ả' | 'ã' | 'ă' | 'ằ' | 'ắ' | 'ặ' | 'ẳ' | 'ẵ' | 'â' | 'ầ' | 'ấ'
            | 'ậ' | 'ẩ' | 'ẫ' => 'a',
            'đ' => 'd',
            'è' | 'é' | 'ẹ' | 'ẻ' | 'ẽ' | 'ê' | 'ề' | 'ế' | 'ệ' | 'ể' | 'ễ' => {
                'e'
            }
            'ì' | 'í' | 'ị' | 'ỉ' | 'ĩ' => 'i',
            'ò' | 'ó' | 'ọ' | 'ỏ' | 'õ' | 'ô' | 'ồ' | 'ố' | 'ộ' | 'ổ' | 'ỗ' | 'ơ' | 'ờ' | 'ớ'
            | 'ợ' | 'ở' | 'ỡ' => 'o',
            'ù' | 'ú' | 'ụ' | 'ủ' | 'ũ' | 'ư' | 'ừ' | 'ứ' | 'ự' | 'ử' | 'ữ' => {
                'u'
            }
            'ỳ' | 'ý' | 'ỵ' | 'ỷ' | 'ỹ' => 'y',
            ch if ch.is_ascii_alphanumeric() => ch,
            ch if ch.is_whitespace() || ch == '-' || ch == '_' || ch == '/' => ' ',
            _ => ' ',
        };
        out.push(mapped);
    }

    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_common_vietnamese_aliases() {
        let activity = normalize_activity_alias("Động thổ").expect("known alias");
        assert_eq!(activity.activity_id, ActivityId::ConstructionGroundbreaking);

        let activity = normalize_activity_alias("Ký hợp đồng").expect("known alias");
        assert_eq!(activity.activity_id, ActivityId::ContractAgreement);

        let activity = normalize_activity_alias("Xuất hành xa").expect("known alias");
        assert_eq!(activity.activity_id, ActivityId::Travel);
    }

    #[test]
    fn normalizes_english_aliases() {
        let activity = normalize_activity_alias("Grand opening").expect("known alias");
        assert_eq!(activity.activity_id, ActivityId::OpeningStart);

        let activity = normalize_activity_alias("Medical treatment").expect("known alias");
        assert_eq!(activity.activity_id, ActivityId::MedicalTreatment);
    }

    #[test]
    fn unknown_alias_returns_none() {
        assert!(normalize_activity_alias("Đọc sách").is_none());
    }
}
