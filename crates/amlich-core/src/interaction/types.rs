use serde::{Deserialize, Serialize};

use crate::almanac::types::{
    FiveElement, FiveElementRelation, PunishmentKind, RuleEvidence, ThapThanResult, TriadElement,
};
use crate::bazi::types::PillarKind;

/// How today's Earthly Branch relates to a personal pillar's branch.
///
/// Source-cited taxonomy per
/// `docs/architecture/personal-day-audit/branch-relation-decision.md`:
/// membership in a Tam hợp triad is exposed as a typed `TriadElement`,
/// and Tương hình is exposed as the typed [`PunishmentKind`] so that
/// "same-branch is not automatically tam hợp" and "incomplete triads
/// are not promoted to a completed-group verdict".
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchRelation {
    /// `true` when the day's branch and the pillar's branch are equal.
    /// This is the only piece of state needed to disambiguate "tam hợp
    /// pair" from "tam hợp membership" — see
    /// [`BranchRelation::is_tam_hop_pair`].
    pub same_branch: bool,
    /// Lục Xung: true if the two branches directly clash (6 positions apart).
    pub luc_xung: bool,
    /// Lục Hợp: true if the two branches form a six-harmony pair.
    pub luc_hop: bool,
    /// Tam Hợp triad membership: `Some(element)` when both branches
    /// belong to the same canonical Tam hợp triad (Water / Wood / Fire
    /// / Metal), `None` when the two branches are in different triads.
    /// Same-branch is always `Some(element)` for the branch's own triad
    /// (because every branch is a member of its own triad). Use
    /// [`BranchRelation::is_tam_hop_pair`] to test the friendly
    /// "two distinct branches in the same triad" case.
    pub tam_hop_member: Option<TriadElement>,
    /// Tam Hợp completed group: true only when three distinct branches
    /// of one triad are simultaneously in scope. By definition this
    /// cannot be true at the pair level (the API only takes two
    /// branches), so this field is always `false` here. It exists so
    /// the contract is explicit and to make chart-level aggregation
    /// (e.g. a Bazi chart with three pillars in the same triad)
    /// straightforward to express without inventing a new field.
    pub tam_hop_completed: bool,
    /// Tương Hại: true if the two branches form a mutual-harm pair.
    pub tuong_hai: bool,
    /// Tương Hình: typed canonical punishment classification
    /// (`None` | `DirectedPair` | `CompletedTriad` |
    /// `SelfPunishment` | `Unavailable`). See [`PunishmentKind`].
    pub tuong_hinh: PunishmentKind,
}

impl BranchRelation {
    /// True when no clash/harm/punishment relations exist.
    pub fn is_neutral(&self) -> bool {
        !self.luc_xung
            && !self.tuong_hai
            && !self.tuong_hinh.is_punishment()
            && !self.is_tam_hop_pair()
    }

    /// True when at least one harmony relation exists.
    ///
    /// Tam hợp only counts as harmony when the two branches are
    /// **distinct** members of the same triad; same-branch is not a
    /// harmony pair (it is a self-punishment or simply neutral, see
    /// [`BranchRelation::tam_hop_member`]).
    pub fn has_harmony(&self) -> bool {
        self.luc_hop || self.is_tam_hop_pair()
    }

    /// True when at least one conflict relation exists.
    pub fn has_conflict(&self) -> bool {
        self.luc_xung
            || self.tuong_hai
            || matches!(
                self.tuong_hinh,
                PunishmentKind::DirectedPair { .. }
                    | PunishmentKind::CompletedTriad { .. }
                    | PunishmentKind::SelfPunishment { .. }
            )
    }

    /// True when both branches are **distinct** members of the same
    /// Tam hợp triad (the friendly "tam hợp" reading).
    pub fn is_tam_hop_pair(&self) -> bool {
        !self.same_branch && self.tam_hop_member.is_some() && !self.tam_hop_completed
    }
}

/// Element interaction between the day's stem element and a pillar's stem element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ElementInteraction {
    /// Same element (đồng).
    Same,
    /// Day generates pillar (sinh).
    DayGeneratesPillar,
    /// Pillar generates day (sinh).
    PillarGeneratesDay,
    /// Day controls pillar (khắc).
    DayControlsPillar,
    /// Pillar controls day (khắc).
    PillarControlsDay,
}

impl From<FiveElementRelation> for ElementInteraction {
    fn from(rel: FiveElementRelation) -> Self {
        match rel {
            FiveElementRelation::Same => ElementInteraction::Same,
            FiveElementRelation::DayGeneratesTarget => ElementInteraction::DayGeneratesPillar,
            FiveElementRelation::TargetGeneratesDay => ElementInteraction::PillarGeneratesDay,
            FiveElementRelation::DayControlsTarget => ElementInteraction::DayControlsPillar,
            FiveElementRelation::TargetControlsDay => ElementInteraction::PillarControlsDay,
        }
    }
}

/// One row of the Day-Person Interaction Matrix — how today interacts with one pillar.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PillarInteraction {
    /// Which pillar this row describes.
    pub pillar: PillarKind,
    /// The pillar's Can Chi label (e.g. "Giáp Tý").
    pub pillar_canchi: String,
    /// Thập Thần: relation from today's stem to this pillar's stem.
    pub thap_than: ThapThanResult,
    /// Branch relationships: Xung/Hợp between today's branch and this pillar's branch.
    pub branch_relation: BranchRelation,
    /// Element interaction between today's stem element and this pillar's stem element.
    pub element_interaction: ElementInteraction,
}

/// The complete Day-Person Interaction Matrix — 4 rows (one per pillar).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DayPersonMatrix {
    /// The day's Can Chi used for this computation.
    pub day_canchi: String,
    /// The person's Nhật Chủ (day master) Can Chi.
    pub day_master: String,
    /// Thập Thần from today's stem to the person's day master stem.
    pub day_to_day_master: ThapThanResult,
    /// Per-pillar interaction rows.
    pub pillars: Vec<PillarInteraction>,
    /// Source attribution.
    pub evidence: RuleEvidence,
}

// ── Matrix 3: Personal Hour Matrix ──────────────────────────────────────

/// How one of the 12 traditional hours interacts with the person on a given day.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonalHourEntry {
    /// Branch index 0-11 (Tý..Hợi).
    pub chi_index: usize,
    /// Branch name (e.g. "Tý").
    pub chi: String,
    /// Full Can Chi of this hour (derived via Ngũ Thử Độn Thời).
    pub canchi: String,
    /// Time range (e.g. "23:00-01:00").
    pub time_range: String,
    /// Generic Hoàng Đạo quality (true = auspicious star).
    pub is_hoang_dao: bool,
    /// Name of the 12-star governing this hour.
    pub star_name: String,
    /// Thập Thần: hour stem → person's Nhật Chủ.
    pub thap_than_to_day_master: ThapThanResult,
    /// Branch relation between this hour's chi and the person's birth hour chi.
    pub branch_relation_to_birth_hour: BranchRelation,
    /// Element interaction: hour stem element vs person's day master element.
    pub element_interaction: ElementInteraction,
    /// Whether the hour's element supports the person's weakest element.
    pub supports_weak_element: bool,
    /// Composite personal score (0-100).
    pub score: u8,
}

/// The complete Personal Hour Matrix — 12 rows (one per traditional hour).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonalHourMatrix {
    /// The day's Can Chi.
    pub day_canchi: String,
    /// The person's Nhật Chủ label.
    pub day_master: String,
    /// The person's birth hour chi (e.g. "Dần").
    pub birth_hour_chi: String,
    /// The person's weakest element (lowest score in distribution).
    pub weak_element: FiveElement,
    /// Per-hour rows sorted by chi_index 0..11.
    pub hours: Vec<PersonalHourEntry>,
    /// Source attribution.
    pub evidence: RuleEvidence,
}

// ── Matrix 2: Element Resonance Matrix ──────────────────────────────────

/// One row per element: how today's energy resonates with that element in the person's chart.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElementResonanceEntry {
    /// Which of the 5 elements this row describes.
    pub element: FiveElement,
    /// The person's score for this element (from ElementDistribution).
    pub personal_score: u16,
    /// Raw relation coefficient: day_element → this element (−1.0 to +1.0).
    pub relation_to_day: f32,
    /// Seasonal modifier for the day's element in the current month.
    pub season_factor: f32,
    /// Effective resonance = relation × season_factor.
    pub effective_resonance: f32,
    /// True when personal_score ≤ 15 (deficit threshold).
    pub is_deficit: bool,
    /// True when is_deficit AND effective_resonance > 0 (day helps fill the gap).
    pub day_helps_deficit: bool,
}

/// The complete Element Resonance Matrix — 5 rows (one per element).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElementResonanceMatrix {
    /// The day's Can Chi.
    pub day_canchi: String,
    /// The day's stem element.
    pub day_element: FiveElement,
    /// The current month's branch (for seasonal context).
    pub month_chi: String,
    /// Seasonal strength of the day's element in the current month.
    pub season_factor: f32,
    /// Per-element resonance rows.
    pub entries: Vec<ElementResonanceEntry>,
    /// Personal-score-weighted aggregate of `effective_resonance`. The
    /// previous unweighted sum was independent of the input
    /// `ElementDistribution`, so contrasting profiles produced identical
    /// values — see amlich-mwbp.5 / REPAIR-PLAN.md P0.2.
    pub net_resonance: f32,
    /// Policy version that produced `net_resonance`. Bumped whenever the
    /// weighting formula changes; consumers should not compare values
    /// across versions.
    #[serde(default)]
    pub resonance_policy_version: String,
    /// Source attribution.
    pub evidence: RuleEvidence,
}

// ── Matrix 4a: Direction Merge Matrix ───────────────────────────────────

/// A signal contributing to a direction's favorability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DirectionSignal {
    /// Kua (Bát Trạch) favorable direction.
    KuaFavorable,
    /// Kua (Bát Trạch) unfavorable direction.
    KuaUnfavorable,
    /// Tài Thần (Wealth God) direction today.
    TaiThan,
    /// Hỷ Thần (Joy God) direction today.
    HyThan,
    /// Phúc Thần (Fortune God) direction today.
    PhucThan,
    /// Sát Phương (Killing direction) today.
    SatPhuong,
}

impl DirectionSignal {
    pub fn is_favorable(self) -> bool {
        matches!(
            self,
            DirectionSignal::KuaFavorable
                | DirectionSignal::TaiThan
                | DirectionSignal::HyThan
                | DirectionSignal::PhucThan
        )
    }

    pub fn is_unfavorable(self) -> bool {
        matches!(
            self,
            DirectionSignal::KuaUnfavorable | DirectionSignal::SatPhuong
        )
    }
}

/// One row of the Direction Merge Matrix — one compass direction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirectionEntry {
    /// Vietnamese direction name (e.g. "Đông Nam").
    pub direction: String,
    /// All signals active for this direction.
    pub signals: Vec<DirectionSignal>,
    /// Count of favorable signals.
    pub favorable_count: i8,
    /// Count of unfavorable signals.
    pub unfavorable_count: i8,
    /// Net score = favorable − unfavorable.
    pub net_score: i8,
}

/// The complete Direction Merge Matrix — 8 rows (compass directions).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectionMergeMatrix {
    /// The day's Can Chi.
    pub day_canchi: String,
    /// The person's Kua number.
    pub kua_number: u8,
    /// Per-direction entries.
    pub entries: Vec<DirectionEntry>,
    /// Source attribution.
    pub evidence: RuleEvidence,
}

// ── Matrix 4b: Domain-Day Boost Matrix ──────────────────────────────────

/// One row of the Domain-Day Boost Matrix — one life domain.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomainDayBoostEntry {
    /// Domain name.
    pub domain: String,
    /// Base Bazi domain score (0-100).
    pub base_score: f32,
    /// Day modifier from stars/trực/thần.
    pub day_modifier: f32,
    /// Yearly Hạn penalty (0.0 if no hạn, negative if hạn active).
    pub han_penalty: f32,
    /// Boosted score = base × (1 + day_modifier + han_penalty), clamped 0-100.
    pub boosted_score: f32,
}

/// The complete Domain-Day Boost Matrix — 5 rows (life domains).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomainDayBoostMatrix {
    /// The day's Can Chi.
    pub day_canchi: String,
    /// Per-domain entries.
    pub entries: Vec<DomainDayBoostEntry>,
    /// Source attribution.
    pub evidence: RuleEvidence,
}
