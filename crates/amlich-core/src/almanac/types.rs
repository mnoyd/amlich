use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FiveElement {
    Moc,
    Hoa,
    Tho,
    Kim,
    Thuy,
}

impl FiveElement {
    pub const ALL: [FiveElement; 5] = [
        FiveElement::Moc,
        FiveElement::Hoa,
        FiveElement::Tho,
        FiveElement::Kim,
        FiveElement::Thuy,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Polarity {
    Duong,
    Am,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeavenlyStem {
    Giap,
    At,
    Binh,
    Dinh,
    Mau,
    Ky,
    Canh,
    Tan,
    Nham,
    Quy,
}

impl HeavenlyStem {
    pub const ALL: [HeavenlyStem; 10] = [
        HeavenlyStem::Giap,
        HeavenlyStem::At,
        HeavenlyStem::Binh,
        HeavenlyStem::Dinh,
        HeavenlyStem::Mau,
        HeavenlyStem::Ky,
        HeavenlyStem::Canh,
        HeavenlyStem::Tan,
        HeavenlyStem::Nham,
        HeavenlyStem::Quy,
    ];

    pub fn element(self) -> FiveElement {
        match self {
            HeavenlyStem::Giap | HeavenlyStem::At => FiveElement::Moc,
            HeavenlyStem::Binh | HeavenlyStem::Dinh => FiveElement::Hoa,
            HeavenlyStem::Mau | HeavenlyStem::Ky => FiveElement::Tho,
            HeavenlyStem::Canh | HeavenlyStem::Tan => FiveElement::Kim,
            HeavenlyStem::Nham | HeavenlyStem::Quy => FiveElement::Thuy,
        }
    }

    pub fn polarity(self) -> Polarity {
        match self {
            HeavenlyStem::Giap
            | HeavenlyStem::Binh
            | HeavenlyStem::Mau
            | HeavenlyStem::Canh
            | HeavenlyStem::Nham => Polarity::Duong,
            HeavenlyStem::At
            | HeavenlyStem::Dinh
            | HeavenlyStem::Ky
            | HeavenlyStem::Tan
            | HeavenlyStem::Quy => Polarity::Am,
        }
    }
}

impl TryFrom<&str> for HeavenlyStem {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim().to_lowercase().as_str() {
            "giáp" | "giap" => Ok(HeavenlyStem::Giap),
            "ất" | "at" => Ok(HeavenlyStem::At),
            "bính" | "binh" => Ok(HeavenlyStem::Binh),
            "đinh" | "dinh" => Ok(HeavenlyStem::Dinh),
            "mậu" | "mau" => Ok(HeavenlyStem::Mau),
            "kỷ" | "ky" => Ok(HeavenlyStem::Ky),
            "canh" => Ok(HeavenlyStem::Canh),
            "tân" | "tan" => Ok(HeavenlyStem::Tan),
            "nhâm" | "nham" => Ok(HeavenlyStem::Nham),
            "quý" | "quy" => Ok(HeavenlyStem::Quy),
            other => Err(format!("invalid heavenly stem: {other}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FiveElementRelation {
    Same,
    DayGeneratesTarget,
    TargetGeneratesDay,
    DayControlsTarget,
    TargetControlsDay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThapThanLabel {
    TyKien,
    KiepTai,
    ThucThan,
    ThuongQuan,
    ChinhTai,
    ThienTai,
    ChinhQuan,
    ThatSat,
    ChinhAn,
    ThienAn,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuleSetDefaults {
    pub tz_offset: f64,
    #[serde(default)]
    pub meridian: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleSetSourceNote {
    pub family: String,
    pub source_id: String,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuleSetDescriptor {
    pub id: String,
    pub version: String,
    pub region: String,
    pub profile: String,
    pub defaults: RuleSetDefaults,
    #[serde(default)]
    pub source_notes: Vec<RuleSetSourceNote>,
    pub schema_version: String,
}

/// Source attribution for a group of almanac rules.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleEvidence {
    pub source_id: String,
    pub method: String,
    pub profile: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThapThanResult {
    pub label: ThapThanLabel,
    pub relation: FiveElementRelation,
    pub same_polarity: bool,
    pub evidence: RuleEvidence,
}

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
    pub evidence: Option<RuleEvidence>,
}

/// Tàng Can (Hidden Stems) - hidden Heavenly Stems embedded in each Địa Chi
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TangCan {
    /// Main (chính) hidden stem - 100% strength
    pub main: String,
    /// Central (trung) hidden stem - variable strength
    pub central: String,
    /// Residual (dư) hidden stem - variable strength
    pub residual: String,
    /// Strength values for [main, central, residual]
    pub strength: [u8; 3],
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
    /// Lục hợp harmony partner (six harmonies).
    pub liu_he: Option<String>,
    /// Tương hại harm partner (mutual harms).
    pub xiang_hai: Option<String>,
    /// Tương hình punishment group members (mutual punishments).
    ///
    /// Day-level projection: the three members of the day chi's canonical
    /// punishment group (one of 寅巳申, 丑未戌), or `None` for branches
    /// that are not in any canonical 3-branch group. The pair-level,
    /// self-vs-directed, and self-punishment semantics live in
    /// `PunishmentKind` (see `interaction::types::BranchRelation`).
    pub xiang_xing: Option<Vec<String>>,
}

/// The five-element classification used for Tam hợp (三合) triads.
///
/// This is the canonical element for the three-harmony triad that a branch
/// belongs to. Branches that do not belong to a triad (none in the current
/// 12-branch system) would not produce a value; in practice every branch
/// belongs to exactly one triad, so this is total over `0..12`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TriadElement {
    /// Thủy — Water triad: {Thân(8), Tý(0), Thìn(4)}
    Thuy,
    /// Kim — Metal triad: {Tỵ(5), Dậu(9), Sửu(1)}
    Kim,
    /// Hỏa — Fire triad: {Dần(2), Ngọ(6), Tuất(10)}
    Hoa,
    /// Mộc — Wood triad: {Hợi(11), Mão(3), Mùi(7)}
    Moc,
}

impl TriadElement {
    /// Vietnamese name used in summaries and golden fixtures.
    pub const fn as_vietnamese(self) -> &'static str {
        match self {
            TriadElement::Thuy => "Thủy",
            TriadElement::Kim => "Kim",
            TriadElement::Hoa => "Hỏa",
            TriadElement::Moc => "Mộc",
        }
    }
}

/// Canonical Tương hình (相刑 / Mutual Punishment) classification for a pair
/// of Earthly Branches.
///
/// Source-cited taxonomy per `docs/architecture/personal-day-audit/branch-relation-decision.md`:
/// 1. **寅巳申** — Vô ân chi hình (mutual 3-branch Fire triad)
/// 2. **丑未戌** — Trì thế chi hình (mutual 3-branch Earth triad)
/// 3. **子卯** — Vô lễ chi hình (directed: Tý → Mão only)
/// 4. **自刑** — Tự hình (self-punishment for {Thìn, Ngọ, Dậu, Hợi})
/// 5. Everything else is **not** a punishment per the canonical Vietnamese /
///    Chinese primary tradition. Incomplete two-branch occurrences of
///    寅巳申 / 丑未戌 are **marked unavailable** rather than promoted.
///
/// Strict canonical only: no sub-school variants are shipped in v1.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PunishmentKind {
    /// No punishment relation between the two branches.
    None,
    /// Sub 卯 (Tý → Mão) directed pair. Only the direction Tý → Mão
    /// is canonical; the reverse Mão → Tý is also represented as this
    /// variant with `aggressor: Tý, victim: Mão` (the same direction) so
    /// callers don't have to special-case the input order.
    DirectedPair {
        /// The branch that "harms" or "punishes" (the aggressor).
        aggressor: BranchRef,
        /// The branch that is harmed or punished (the victim).
        victim: BranchRef,
    },
    /// Mutual 3-branch punishment group (寅巳申 = Fire, 丑未戌 = Earth).
    /// Only emitted when both branches are in the triad and are
    /// distinct (same-branch self-punishment is reported separately).
    CompletedTriad {
        /// The element (and therefore the canonical triad) the pair belongs to.
        triad: TriadElement,
    },
    /// Self-punishment (自刑). Only emitted when the two branches are equal
    /// and the branch is one of {Thìn(4), Ngọ(6), Dậu(9), Hợi(11)}.
    SelfPunishment {
        /// The branch that is in self-punishment.
        branch: BranchRef,
    },
    /// Disputed or incomplete case that the canonical tradition marks
    /// as unavailable rather than promoting it to a verdict.
    Unavailable {
        /// A short, stable human-readable reason (English / Vietnamese).
        reason: String,
    },
}

impl PunishmentKind {
    /// `true` when this kind encodes a real punishment relation
    /// (directed, completed triad, or self-punishment).
    pub fn is_punishment(&self) -> bool {
        matches!(
            self,
            PunishmentKind::DirectedPair { .. }
                | PunishmentKind::CompletedTriad { .. }
                | PunishmentKind::SelfPunishment { .. }
        )
    }
}

/// Stable, serializable reference to an Earthly Branch (Địa Chi).
///
/// Carries both the canonical index (0..12) and the Vietnamese name so
/// the typed `PunishmentKind` round-trips through JSON without losing
/// either piece of context. The name is owned (not `&'static str`) so
/// `BranchRef` is `Deserialize`-compatible and can be used inside
/// owning enums like [`PunishmentKind`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BranchRef {
    /// 0-based Earthly Branch index (0=Tý .. 11=Hợi).
    pub index: usize,
    /// Vietnamese name (e.g. "Tý").
    pub name: String,
}

impl BranchRef {
    /// Build a `BranchRef` from a 0-based index. Panics on out-of-range
    /// input — callers in this codebase always hold validated indices.
    pub fn new(index: usize) -> Self {
        debug_assert!(index < 12, "BranchRef index must be in 0..12");
        Self {
            index,
            name: crate::types::CHI[index].to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DayFortune {
    pub ruleset_id: String,
    pub ruleset_version: String,
    pub profile: String,
    pub day_element: DayElement,
    pub conflict: DayConflict,
    pub travel: TravelDirection,
    pub stars: DayStars,
    pub day_deity: Option<DayDeity>,
    pub taboos: Vec<DayTaboo>,
    pub xung_hop: XungHopResult,
    pub truc: TrucInfo,
    pub tang_can: Option<TangCan>,
    /// Ten Gods relations for predefined targets (populated when day stem available)
    pub ten_gods: Option<DayTenGods>,
    /// Kua (Tu Mến) result (populated only when birth year and gender provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tu_menh: Option<super::tu_menh::KuaResult>,
}

/// Day-level Ten Gods results for predefined targets
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DayTenGods {
    /// Ten Gods relation from day stem to year stem
    pub to_year_stem: Option<ThapThanResult>,
    /// Ten Gods relation from day stem to self (day stem to day stem)
    pub to_self: Option<ThapThanResult>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ruleset_descriptor_serializes() {
        let descriptor = RuleSetDescriptor {
            id: "vn_baseline_v1".to_string(),
            version: "v1".to_string(),
            region: "vn".to_string(),
            profile: "baseline".to_string(),
            defaults: RuleSetDefaults {
                tz_offset: 7.0,
                meridian: None,
            },
            source_notes: vec![RuleSetSourceNote {
                family: "taboo_rules".to_string(),
                source_id: "khcbppt".to_string(),
                note: "Baseline v1 frozen mapping".to_string(),
            }],
            schema_version: "ruleset-descriptor/v1".to_string(),
        };

        let encoded = serde_json::to_string(&descriptor).expect("serialize");
        let decoded: RuleSetDescriptor = serde_json::from_str(&encoded).expect("deserialize");
        assert_eq!(decoded.id, "vn_baseline_v1");
        assert_eq!(decoded.defaults.tz_offset, 7.0);
        assert_eq!(decoded.source_notes.len(), 1);
    }

    #[test]
    fn day_ten_gods_struct_exists() {
        // Test 1: DayTenGods structure exists
        let _ = std::mem::size_of::<DayTenGods>();
    }

    #[test]
    fn day_fortune_has_optional_ten_gods_field() {
        // Test 1: DayFortune includes optional ten_gods field
        let fortune = DayFortune {
            ruleset_id: "test".to_string(),
            ruleset_version: "v1".to_string(),
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
                tuoi_xung: vec![],
                sat_huong: "Nam".to_string(),
                evidence: None,
            },
            travel: TravelDirection {
                xuat_hanh_huong: "Đông Nam".to_string(),
                tai_than: "Tây Nam".to_string(),
                hy_than: "Đông Bắc".to_string(),
                evidence: None,
            },
            stars: DayStars {
                cat_tinh: vec![],
                sat_tinh: vec![],
                day_star: None,
                star_system: None,
                evidence: None,
                matched_rules: vec![],
            },
            day_deity: None,
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
                quality: "cat".to_string(),
                evidence: None,
            },
            tang_can: None,
            ten_gods: None,
            tu_menh: None,
        };

        // Verify ten_gods field exists and is optional (defaults to None)
        // This test will fail initially because we haven't added the field yet
        let _ = fortune.ten_gods;
    }

    #[test]
    fn day_fortune_serializes_with_snake_case_fields() {
        // Test 3: Serialization produces stable JSON field names (snake_case)
        let fortune = DayFortune {
            ruleset_id: "test".to_string(),
            ruleset_version: "v1".to_string(),
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
                tuoi_xung: vec![],
                sat_huong: "Nam".to_string(),
                evidence: None,
            },
            travel: TravelDirection {
                xuat_hanh_huong: "Đông Nam".to_string(),
                tai_than: "Tây Nam".to_string(),
                hy_than: "Đông Bắc".to_string(),
                evidence: None,
            },
            stars: DayStars {
                cat_tinh: vec![],
                sat_tinh: vec![],
                day_star: None,
                star_system: None,
                evidence: None,
                matched_rules: vec![],
            },
            day_deity: None,
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
                quality: "cat".to_string(),
                evidence: None,
            },
            tang_can: None,
            ten_gods: None,
            tu_menh: None,
        };

        let json = serde_json::to_string(&fortune).expect("serialize");
        // Verify new fields serialize with snake_case names
        // This test will fail initially because we haven't added the fields yet
        assert!(json.contains("\"ten_gods\"") || json.contains("\"tu_menh\""));
    }

    #[test]
    fn day_fortune_struct_exists() {
        let _ = std::mem::size_of::<DayFortune>();
    }

    #[test]
    fn tang_can_struct_exists() {
        let _ = std::mem::size_of::<TangCan>();
    }

    #[test]
    fn tang_can_has_4_fields() {
        let tc = TangCan {
            main: "甲".to_string(),
            central: "乙".to_string(),
            residual: "丙".to_string(),
            strength: [60, 25, 15],
        };
        assert_eq!(tc.main, "甲");
        assert_eq!(tc.central, "乙");
        assert_eq!(tc.residual, "丙");
        assert_eq!(tc.strength, [60, 25, 15]);
    }

    #[test]
    fn tang_can_serializes() {
        let tc = TangCan {
            main: "甲".to_string(),
            central: "乙".to_string(),
            residual: "丙".to_string(),
            strength: [60, 25, 15],
        };
        let json = serde_json::to_string(&tc).expect("serialize");
        assert!(json.contains("\"甲\""));
        assert!(json.contains("\"乙\""));
        assert!(json.contains("\"丙\""));
        assert!(json.contains("[60,25,15]"));
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
            ruleset_id: "vn_baseline_v1".to_string(),
            ruleset_version: "v1".to_string(),
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
                liu_he: Some("Mùi".to_string()),
                xiang_hai: Some("Tý".to_string()),
                xiang_xing: Some(vec!["Dần".to_string(), "Mão".to_string(), "Tỵ".to_string()]),
            },
            truc: TrucInfo {
                index: 2,
                name: "Mãn".to_string(),
                quality: "hung".to_string(),
                evidence: None,
            },
            tang_can: Some(TangCan {
                main: "甲".to_string(),
                central: "乙".to_string(),
                residual: "丙".to_string(),
                strength: [60, 25, 15],
            }),
            ten_gods: None,
            tu_menh: None,
        };

        let encoded = serde_json::to_string(&value).expect("serialize");
        let decoded: DayFortune = serde_json::from_str(&encoded).expect("deserialize");
        assert_eq!(decoded.profile, "baseline");
        assert_eq!(decoded.ruleset_id, "vn_baseline_v1");
        assert_eq!(decoded.day_element.element, "Kim");
    }
}
