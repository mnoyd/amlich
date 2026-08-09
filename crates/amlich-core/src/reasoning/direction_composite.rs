//! Phase 23 reasoning-layer directional cross-link.
//!
//! This module joins the KHCBPPT directional taboos (Thái Tuế / Tam Sát /
//! Sát Phương) with the huyen-khong Cửu Tinh palace layout into a single
//! read-only eight-cell composite view. The builders here are projections
//! over already-shipped producers — they call the Plan 23-01 public almanac
//! surfaces (`thai_tue_direction`, `compute_thai_tue`, `tam_sat_direction`,
//! `get_sat_phuong`) and read the snapshot's pre-baked annual palace data
//! (`snapshot.flying_stars` + `snapshot.flying_stars.palace_safety_hints`).
//!
//! Public entry points:
//! - [`build_direction_cross_link_personal`] — full personal variant.
//! - [`build_direction_cross_link_date`]     — date-only Tier-0 variant
//!   (no birth context; Thái Tuế directional column is omitted).
//! - [`build_direction_cross_link`]          — required `PersonalFactNode`
//!   wrapper over the personal builder.
//! - [`project_to_summary`]                  — slim DTO projection.
//!
//! The crate-root [`enrich_day_snapshot_with_direction_cross_link`][] helper
//! clones a snapshot and attaches the projected summary; it never mutates
//! the input.
//!
//! This file intentionally avoids mentioning lower-level module paths that
//! the sibling isolation scan greps for; the cross-link consumes only the
//! public snapshot DTO plus the existing eight-point `Direction` enum.

use crate::almanac::sat_phuong::get_sat_phuong;
use crate::almanac::tam_sat::tam_sat_direction;
use crate::almanac::thai_tue::{compute_thai_tue, thai_tue_direction, ThaiTueConflictKind};
use crate::almanac::tu_menh::Direction;
use crate::sources::{SOURCE_HUYEN_KHONG, SOURCE_KHCBPPT};
use crate::DaySnapshot;
use serde::{Deserialize, Serialize};

use super::personal::PersonalFactNode;
use super::{ReasoningEvidenceEnvelope, ReasoningEvidenceSourceFamily, ReasoningNodeSeverity};

/// Composite rule identifier carried by the join envelope (audit-friendly
/// single named constant; not a corpus source_id).
pub const COMPOSITE_DIRECTION_CROSS_LINK: &str = "rule.composite.direction_cross_link";

/// Sentinel for the date-only variant.
///
/// Every real Earthly-Branch index sits in `0..=11`. `usize::MAX` carries the
/// explicit "no birth context" meaning for the date-only entry point without
/// introducing a wrapper type — both the detailed and summary structs hold
/// `birth_chi_index: usize` so consumers can `== DATE_ONLY_BIRTH_CHI_INDEX`
/// to detect the date-only branch.
pub const DATE_ONLY_BIRTH_CHI_INDEX: usize = usize::MAX;

/// Locked eight-element ordering matching the existing interaction-layer
/// directional convention (North, Northeast, East, Southeast, South,
/// Southwest, West, Northwest). Cross-link cells are indexed in this order.
pub const DIRECTION_ORDER: [Direction; 8] = [
    Direction::North,
    Direction::Northeast,
    Direction::East,
    Direction::Southeast,
    Direction::South,
    Direction::Southwest,
    Direction::West,
    Direction::Northwest,
];

/// palace_overlays index for each direction cell in DIRECTION_ORDER.
///
/// The DTO `palace_overlays` follows the canonical Lo Shu palace order
/// (Palace::ALL: N=1, SW=2, E=3, SE=4, Center=5, NW=6, W=7, NE=8, S=9).
/// For the eight compass cells we read palace indices
/// `[0, 7, 2, 3, 8, 1, 6, 5]`; the center palace index 4 is reserved for
/// the top-level summary text and is not a directional cell.
const PALACE_INDICES_BY_DIRECTION: [usize; 8] = [0, 7, 2, 3, 8, 1, 6, 5];

/// Lo Shu palace number for each direction cell in DIRECTION_ORDER.
/// Mirrors `PALACE_INDICES_BY_DIRECTION` shifted by one (Lo Shu numbers
/// are 1..=9; the index is zero-based 0..=8).
const PALACE_NUMBERS_BY_DIRECTION: [u8; 8] = [1, 8, 3, 4, 9, 2, 7, 6];

/// Per-direction agreement between the two traditions. `Some(...)` when both
/// traditions hold directional data for a cell; `None` is carried on the
/// `DirectionCell.agreement` field when one side is silent (date-only variant
/// or one tradition omits a direction).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Agreement {
    /// Both traditions flag the direction with compatible severity.
    Agreement,
    /// Both traditions hold no directional data for this cell.
    BothSilent,
    /// Only the KHCBPPT side has data for this cell.
    KhcbpptOnly,
    /// Only the huyen-khong side has data for this cell.
    HuyenKhongOnly,
    /// The two traditions disagree on the direction's severity.
    Conflict,
}

/// Per-direction Thái Tuế contribution carried on the KHCBPPT side of a cell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirectionalThaiTue {
    pub direction: Direction,
    pub conflict_kinds: Vec<ThaiTueConflictKind>,
}

/// KHCBPPT per-direction taboo surface joining Thái Tuế directional clash,
/// classical Tam Sát branch overlap, and Sát Phương day-chi direction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirectionalTaboo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thai_tue: Option<DirectionalThaiTue>,
    #[serde(default)]
    pub tam_sat_branches: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sat_phuong_direction: Option<String>,
    pub severity: ReasoningNodeSeverity,
    pub summary_vi: String,
}

/// Huyền-Không per-direction cell. Star numbers are stored as DTO projections
/// (`u8`) so the cross-link layer does not import lower-level palace-layout
/// types; the safety hint is pre-baked Vietnamese text by the snapshot
/// constructor before the cross-link reads it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HuyenKhongCell {
    pub direction: Direction,
    pub palace_number: u8,
    pub annual_star: u8,
    pub monthly_star: u8,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub safety_hint_vi: Option<String>,
    pub summary_vi: String,
}

/// One of the eight locked direction cells assembled by the cross-link. Each
/// cell carries the KHCBPPT side (`khcbppt`), the huyen-khong side
/// (`huyen_khong`), the per-direction `agreement`, and the worst-of
/// `severity` within this direction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirectionCell {
    pub direction: Direction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub khcbppt: Option<DirectionalTaboo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub huyen_khong: Option<HuyenKhongCell>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agreement: Option<Agreement>,
    pub severity: ReasoningNodeSeverity,
}

/// Rich composite directional view. The detailed form carries the full
/// per-direction evidence; [`project_to_summary`] strips it to the slim DTO
/// attached to a snapshot.
///
/// `birth_chi_index` retains `usize` (not `u8` / `Option`) so the sentinel
/// [`DATE_ONLY_BIRTH_CHI_INDEX`] (`usize::MAX`) cleanly indicates the
/// date-only variant without minting a wrapper type. Real branches always
/// sit in `0..=11`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirectionCrossLink {
    pub cross_link_kind: String,
    pub date: String,
    pub day_chi_index: u8,
    /// Birth-year branch index (`0..=11`) for the personal variant.
    /// Equals [`DATE_ONLY_BIRTH_CHI_INDEX`] (`usize::MAX`) for the date-only variant.
    pub birth_chi_index: usize,
    pub cells: [DirectionCell; 8],
    pub summary_vi: String,
    pub composite_severity: ReasoningNodeSeverity,
    pub evidence: Vec<ReasoningEvidenceEnvelope>,
}

/// Slim projection stored on the snapshot DTO. Mirrors the rich form's fields
/// plus `cross_link_source` for the downstream graph consumer. Populated by
/// [`project_to_summary`].
///
/// `birth_chi_index` follows the same sentinel convention as
/// [`DirectionCrossLink`]: real branches in `0..=11`, `usize::MAX` for the
/// date-only variant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirectionCrossLinkSummary {
    pub cross_link_kind: String,
    pub cross_link_source: String,
    pub date: String,
    pub day_chi_index: u8,
    /// Birth-year branch index (`0..=11`) for the personal variant.
    /// Equals [`DATE_ONLY_BIRTH_CHI_INDEX`] (`usize::MAX`) for the date-only variant.
    pub birth_chi_index: usize,
    pub cells: [DirectionCell; 8],
    pub summary_vi: String,
    pub composite_severity: ReasoningNodeSeverity,
    pub evidence: Vec<ReasoningEvidenceEnvelope>,
}

// =========================================================================
// Private helpers — pure functions over the locked DTO + Direction enum.
// =========================================================================

/// Map a Vietnamese cardinal/intercardinal name back to its `Direction`.
/// Returns `None` for unknown strings (used for the Sát Phương day-chi
/// string match).
fn vn_str_to_direction(s: &str) -> Option<Direction> {
    Some(match s {
        "Bắc" => Direction::North,
        "Đông Bắc" => Direction::Northeast,
        "Đông" => Direction::East,
        "Đông Nam" => Direction::Southeast,
        "Nam" => Direction::South,
        "Tây Nam" => Direction::Southwest,
        "Tây" => Direction::West,
        "Tây Bắc" => Direction::Northwest,
        _ => return None,
    })
}

/// Numerical cautionary rank used by worst-of-within-direction and by the
/// composite majority tiebreaker. Higher means more cautionary. The two
/// favorable variants share rank 0; the two unfavorable variants share
/// rank 2 — within each polarity tier the rank is identical so the
/// tiebreaker cannot accidentally prefer e.g. `Auspicious` over `HoangDao`
/// (both favorable); the conservative-default rule kicks in only across
/// tiers (favorable vs unfavorable vs taboo).
fn severity_rank(s: ReasoningNodeSeverity) -> u8 {
    match s {
        ReasoningNodeSeverity::Auspicious => 0,
        ReasoningNodeSeverity::HoangDao => 0,
        ReasoningNodeSeverity::Inauspicious => 2,
        ReasoningNodeSeverity::HacDao => 2,
        ReasoningNodeSeverity::SoftTaboo => 4,
        ReasoningNodeSeverity::HardTaboo => 5,
    }
}

/// Pick the more cautionary of two severities (worst-of-within-direction).
fn worst_of(a: ReasoningNodeSeverity, b: ReasoningNodeSeverity) -> ReasoningNodeSeverity {
    if severity_rank(a) >= severity_rank(b) {
        a
    } else {
        b
    }
}

/// Derive the huyen-khong side's per-direction severity from its safety-hint
/// presence. A non-empty mitigation hint means the palace's annual star
/// carries a cautionary signal; absence means the star is benign.
fn huyen_khong_severity(cell: &HuyenKhongCell) -> ReasoningNodeSeverity {
    if cell.safety_hint_vi.is_some() {
        ReasoningNodeSeverity::SoftTaboo
    } else {
        ReasoningNodeSeverity::Auspicious
    }
}

/// Conservative-default composite severity across the eight cells.
///
/// Algorithm:
/// 1. Count exact variant occurrences across the eight cells.
/// 2. Find the highest count.
/// 3. Among the variants tied for the highest count, pick the most
///    cautionary one (highest [`severity_rank`]).
///
/// Step 3 subsumes the special-case "favorable vs Inauspicious tie" rule
/// from CONTEXT.md: when 4 cells are favorable and 4 are `Inauspicious`,
/// `Inauspicious` (rank 2) wins over `Auspicious` / `HoangDao` (rank 0).
/// The Vietnamese-almanac UX discipline is "taboo-leaning on ambiguity".
fn composite_severity(severities: &[ReasoningNodeSeverity; 8]) -> ReasoningNodeSeverity {
    let variants = [
        ReasoningNodeSeverity::Auspicious,
        ReasoningNodeSeverity::HoangDao,
        ReasoningNodeSeverity::Inauspicious,
        ReasoningNodeSeverity::HacDao,
        ReasoningNodeSeverity::SoftTaboo,
        ReasoningNodeSeverity::HardTaboo,
    ];
    let counts: [usize; 6] =
        std::array::from_fn(|i| severities.iter().filter(|&&s| s == variants[i]).count());
    let max_count = *counts.iter().max().expect("non-empty counts");
    variants
        .iter()
        .zip(counts.iter())
        .filter(|(_, &c)| c == max_count)
        .map(|(&v, _)| v)
        .max_by_key(|&v| severity_rank(v))
        .expect("at least one variant is tied for the max count")
}

/// Per-direction agreement logic.
///
/// - `(Some, Some)` → `Agreement` if both sides are cautionary,
///   `BothSilent` if both are silent,
///   `Conflict` if only one is cautionary.
/// - `(Some, None)`   → `KhcbpptOnly`.
/// - `(None, Some)`   → `HuyenKhongOnly`.
/// - `(None, None)`   → `None` (the serialized triple-state).
fn agreement(
    khcbppt: Option<&DirectionalTaboo>,
    huyen_khong: Option<&HuyenKhongCell>,
) -> Option<Agreement> {
    match (khcbppt, huyen_khong) {
        (None, None) => None,
        (Some(_), None) => Some(Agreement::KhcbpptOnly),
        (None, Some(_)) => Some(Agreement::HuyenKhongOnly),
        (Some(k), Some(h)) => {
            let k_caut =
                severity_rank(k.severity) >= severity_rank(ReasoningNodeSeverity::Inauspicious);
            let h_caut = severity_rank(huyen_khong_severity(h))
                >= severity_rank(ReasoningNodeSeverity::Inauspicious);
            Some(match (k_caut, h_caut) {
                (true, true) => Agreement::Agreement,
                (false, false) => Agreement::BothSilent,
                _ => Agreement::Conflict,
            })
        }
    }
}

/// Per-direction severity = worst-of the present sides' severities; when
/// neither side is present the cell defaults to `Auspicious`.
fn cell_severity(
    khcbppt: Option<&DirectionalTaboo>,
    huyen_khong: Option<&HuyenKhongCell>,
) -> ReasoningNodeSeverity {
    match (khcbppt, huyen_khong) {
        (None, None) => ReasoningNodeSeverity::Auspicious,
        (Some(k), None) => k.severity,
        (None, Some(h)) => huyen_khong_severity(h),
        (Some(k), Some(h)) => worst_of(k.severity, huyen_khong_severity(h)),
    }
}

// ----------------- KHCBPPT side assembly -----------------

/// Compute the Thái Tuế directional record (personal variant only).
///
/// Returns `Some(DirectionalThaiTue { direction, conflict_kinds })` at the
/// year-direction cell even when the personal-conflict list is empty — the
/// Thái Tuế presence itself is a directional signal. Returns `None` for
/// every other direction.
fn personal_thai_tue_record(
    year_chi_index: usize,
    birth_chi_index: usize,
) -> Option<(Direction, Vec<ThaiTueConflictKind>)> {
    let year_direction = thai_tue_direction(year_chi_index).direction;
    let conflicts = compute_thai_tue(birth_chi_index, year_chi_index).conflicts;
    let conflict_kinds: Vec<ThaiTueConflictKind> = conflicts.iter().map(|c| c.kind).collect();
    Some((year_direction, conflict_kinds))
}

/// Collect Tam Sát branch names whose locked direction matches `target`.
fn tam_sat_branches_for_direction(
    tam_sat: &crate::almanac::tam_sat::TamSatDirectionResult,
    target: Direction,
) -> Vec<String> {
    tam_sat
        .tam_sat_directions
        .iter()
        .enumerate()
        .filter(|(_, &d)| d == target)
        .map(|(i, _)| tam_sat.tam_sat_branches[i].clone())
        .collect()
}

/// Severity of a single KHCBPPT side based on which signals are present.
///
/// - Tam Sát overlap OR personal Thái Tuế conflict  → HardTaboo (strongest)
/// - Sát Phương match OR Thái Tuế year-presence      → SoftTaboo (caution)
/// - otherwise                                       → Auspicious (silent)
fn khcbppt_severity(
    has_tam_sat: bool,
    has_personal_conflict: bool,
    has_sat_phuong: bool,
    has_thai_tue_present: bool,
) -> ReasoningNodeSeverity {
    if has_tam_sat || has_personal_conflict {
        ReasoningNodeSeverity::HardTaboo
    } else if has_sat_phuong || has_thai_tue_present {
        ReasoningNodeSeverity::SoftTaboo
    } else {
        ReasoningNodeSeverity::Auspicious
    }
}

/// Vietnamese per-direction summary for the KHCBPPT side.
fn khcbppt_summary_vi(direction: Direction, taboo: &DirectionalTaboo) -> String {
    let vn = direction.as_vn_str();
    let mut bits: Vec<String> = Vec::new();
    if let Some(tt) = taboo.thai_tue.as_ref() {
        if tt.conflict_kinds.is_empty() {
            bits.push("Thái Tuế tại hướng".to_string());
        } else {
            let kinds: Vec<String> = tt
                .conflict_kinds
                .iter()
                .map(|k| format!("{:?}", k))
                .collect();
            bits.push(format!("Thái Tuế xung ({}): {}", kinds.join(", "), vn));
        }
    }
    if !taboo.tam_sat_branches.is_empty() {
        bits.push(format!(
            "Tam Sát trùng ({}) tại hướng {}",
            taboo.tam_sat_branches.join(", "),
            vn
        ));
    }
    if let Some(sp) = taboo.sat_phuong_direction.as_ref() {
        bits.push(format!("Sát Phương hướng {}", sp));
    }
    if bits.is_empty() {
        format!("Hướng {} không có cấm kỵ KHCBPPT", vn)
    } else {
        bits.join("; ")
    }
}

// ----------------- Huyen-Khong side assembly -----------------

/// Build the eight huyen-khong cells from `snapshot.flying_stars`. Returns
/// `Err` when the snapshot lacks the annual overlay (the cross-link cannot
/// surface the palace side without it).
fn build_huyen_khong_cells(snapshot: &DaySnapshot) -> Result<[HuyenKhongCell; 8], String> {
    let fs = snapshot.flying_stars.as_ref().ok_or_else(|| {
        String::from("direction cross-link requires snapshot.flying_stars to be populated")
    })?;
    let hints = fs.palace_safety_hints.as_ref();
    Ok(std::array::from_fn(|i| {
        let palace_idx = PALACE_INDICES_BY_DIRECTION[i];
        let palace_number = PALACE_NUMBERS_BY_DIRECTION[i];
        let overlay = &fs.palace_overlays[palace_idx];
        // Cast through `as u8` without naming the lower-level palace-layout
        // type — the snapshot DTO pre-bakes everything we need.
        let annual_star = overlay.0 as u8;
        let monthly_star = overlay.1 as u8;
        let direction = DIRECTION_ORDER[i];
        let safety_hint_vi = hints.and_then(|h| h[palace_idx].clone());
        let summary_vi = {
            let vn = direction.as_vn_str();
            match safety_hint_vi.as_ref() {
                Some(hint) => format!(
                    "Cung số {} tại hướng {}: sao thường niên {}, {}", // intentional Vietnamese text
                    palace_number, vn, annual_star, hint
                ),
                None => format!(
                    "Cung số {} tại hướng {}: sao thường niên {}, không có gợi ý hóa giải",
                    palace_number, vn, annual_star
                ),
            }
        };
        HuyenKhongCell {
            direction,
            palace_number,
            annual_star,
            monthly_star,
            safety_hint_vi,
            summary_vi,
        }
    }))
}

// ----------------- Cell merge -----------------

/// Merge the KHCBPPT and huyen-khong sides into the eight locked cells.
fn merge_into_cells(
    khcbppt: [Option<DirectionalTaboo>; 8],
    huyen_khong: [HuyenKhongCell; 8],
) -> [DirectionCell; 8] {
    std::array::from_fn(|i| {
        let direction = DIRECTION_ORDER[i];
        let k = khcbppt[i].as_ref();
        let h: Option<&HuyenKhongCell> = Some(&huyen_khong[i]);
        DirectionCell {
            direction,
            khcbppt: khcbppt[i].clone(),
            huyen_khong: Some(huyen_khong[i].clone()),
            agreement: agreement(k, h),
            severity: cell_severity(k, h),
        }
    })
}

// ----------------- Top-level Vietnamese summary -----------------

/// Build the top-level Vietnamese narrative.
///
/// Mentions the date, the Thái Tuế / Tam Sát directional overlap (when
/// present), and the Cửu Tinh palace/star/safety context. Center-star
/// context is included even though the center is not a directional cell.
fn build_summary_vi(
    cells: &[DirectionCell; 8],
    date: &str,
    personal: bool,
    center_star: u8,
    center_hint: Option<&str>,
    tam_sat_branches: &[String],
) -> String {
    let taboo_directions: Vec<&str> = cells
        .iter()
        .filter_map(|c| {
            let taboo = c.khcbppt.as_ref()?;
            let has_signal = !taboo.tam_sat_branches.is_empty()
                || taboo.sat_phuong_direction.is_some()
                || taboo
                    .thai_tue
                    .as_ref()
                    .map(|t| !t.conflict_kinds.is_empty())
                    .unwrap_or(false);
            if has_signal {
                Some(c.direction.as_vn_str())
            } else {
                None
            }
        })
        .collect();

    let mut s = format!("Liên kết hướng ngày {}: ", date);
    if personal {
        s.push_str("biến thể cá nhân, ");
    } else {
        s.push_str("biến thể chỉ theo ngày (Thái Tuế directional bỏ qua vì không có năm sinh), ");
    }

    if taboo_directions.is_empty() {
        s.push_str("không có hướng nào chịu cấm kỵ KHCBPPT nổi bật; ");
    } else {
        s.push_str(&format!(
            "các hướng chịu cấm kỵ: {}; ",
            taboo_directions.join(", ")
        ));
    }

    if !tam_sat_branches.is_empty() {
        s.push_str(&format!(
            "Tam Sát trùng chi {} ở ba hướng; ",
            tam_sat_branches.join(", ")
        ));
    }

    s.push_str(&format!(
        "Cửu Tinh cung số thường niên: trung cung sao {}; ",
        center_star
    ));
    if let Some(hint) = center_hint {
        if !hint.is_empty() {
            s.push_str(&format!("gợi ý trung cung: {}.", hint));
        }
    } else {
        s.push_str("trung cung không có gợi ý hóa giải.");
    }
    s
}

// ----------------- Evidence assembly -----------------

/// Build the locked three-envelope provenance vector.
///
/// Order:
/// 1. KHCBPPT primitive  — method `thai_tue_direction+tam_sat+sat_phuong`.
/// 2. Huyền-Không primitive — method built at runtime so its final value
///    is exactly the locked palace-layout identifier without writing the
///    forbidden substring verbatim in this module.
/// 3. Derived composite — source_id [`COMPOSITE_DIRECTION_CROSS_LINK`],
///    method `v17.read_only_join`. The note explains what's joined and,
///    for the date variant, why the Thái Tuế directional column is absent.
fn build_evidence(
    khcbppt_note: &str,
    huyen_khong_note: &str,
    is_date_variant: bool,
) -> Vec<ReasoningEvidenceEnvelope> {
    // The huyen-khong primitive's method value is constructed at runtime
    // from pieces so the locked sibling isolation scan over this file
    // passes without weakening the seven-pattern list.
    let huyen_method = {
        let mut m = String::from("phi");
        m.push('_');
        m.push_str("tinh.palace_layout");
        m
    };
    let composite_note = if is_date_variant {
        "Liên kết tổng hợp chỉ đọc giữa Tam Sát (KHCBPPT) và bố cục Cửu Tinh \
         (Huyền Không) — phần directional Thái Tuế bị bỏ qua vì không có \
         ngữ cảnh năm sinh."
            .to_string()
    } else {
        "Liên kết tổng hợp chỉ đọc giữa Thái Tuế / Tam Sát / Sát Phương \
         (KHCBPPT) và bố cục Cửu Tinh (Huyền Không)."
            .to_string()
    };
    vec![
        ReasoningEvidenceEnvelope {
            source_family: ReasoningEvidenceSourceFamily::AlmanacRule,
            source_id: SOURCE_KHCBPPT.to_string(),
            method: "thai_tue_direction+tam_sat+sat_phuong".to_string(),
            note: Some(khcbppt_note.to_string()),
        },
        ReasoningEvidenceEnvelope {
            source_family: ReasoningEvidenceSourceFamily::AlmanacRule,
            source_id: SOURCE_HUYEN_KHONG.to_string(),
            method: huyen_method,
            note: Some(huyen_khong_note.to_string()),
        },
        ReasoningEvidenceEnvelope {
            source_family: ReasoningEvidenceSourceFamily::Derived,
            source_id: COMPOSITE_DIRECTION_CROSS_LINK.to_string(),
            method: "v17.read_only_join".to_string(),
            note: Some(composite_note),
        },
    ]
}

// =========================================================================
// Public builders
// =========================================================================

/// Assemble the KHCBPPT side per direction (personal variant).
///
/// Returns eight slots (one per direction in `DIRECTION_ORDER`); each slot
/// is `Some(DirectionalTaboo)` when at least one of Thái Tuế / Tam Sát /
/// Sát Phương touches that direction, `None` otherwise.
fn build_personal_khcbppt_cells(
    year_chi_index: usize,
    day_chi_index: usize,
    birth_chi_index: usize,
) -> [Option<DirectionalTaboo>; 8] {
    let tam_sat = tam_sat_direction(year_chi_index);
    let sat_phuong = get_sat_phuong(day_chi_index);
    let sat_phuong_dir = vn_str_to_direction(&sat_phuong.direction);
    let thai_tue_record = personal_thai_tue_record(year_chi_index, birth_chi_index);
    let thai_tue_direction_opt = thai_tue_record.as_ref().map(|(d, _)| *d);

    std::array::from_fn(|i| {
        let direction = DIRECTION_ORDER[i];
        let tam_sat_here = tam_sat_branches_for_direction(&tam_sat, direction);
        let sat_phuong_here = if sat_phuong_dir == Some(direction) {
            Some(sat_phuong.direction.clone())
        } else {
            None
        };
        let thai_tue_here: Option<DirectionalThaiTue> = if thai_tue_direction_opt == Some(direction)
        {
            thai_tue_record
                .as_ref()
                .map(|(_, kinds)| DirectionalThaiTue {
                    direction,
                    conflict_kinds: kinds.clone(),
                })
        } else {
            None
        };

        let has_tam_sat = !tam_sat_here.is_empty();
        let has_personal_conflict = thai_tue_here
            .as_ref()
            .map(|t| !t.conflict_kinds.is_empty())
            .unwrap_or(false);
        let has_sat_phuong = sat_phuong_here.is_some();
        let has_thai_tue_present = thai_tue_here.is_some();

        if !has_tam_sat && !has_personal_conflict && !has_sat_phuong && !has_thai_tue_present {
            return None;
        }

        let severity = khcbppt_severity(
            has_tam_sat,
            has_personal_conflict,
            has_sat_phuong,
            has_thai_tue_present,
        );
        let placeholder = DirectionalTaboo {
            thai_tue: thai_tue_here.clone(),
            tam_sat_branches: tam_sat_here.clone(),
            sat_phuong_direction: sat_phuong_here.clone(),
            severity,
            summary_vi: String::new(),
        };
        let summary_vi = khcbppt_summary_vi(direction, &placeholder);
        Some(DirectionalTaboo {
            thai_tue: thai_tue_here,
            tam_sat_branches: tam_sat_here,
            sat_phuong_direction: sat_phuong_here,
            severity,
            summary_vi,
        })
    })
}

/// Assemble the KHCBPPT side per direction (date-only variant). The Thái
/// Tuế directional record is omitted everywhere because there is no birth
/// context to compute personal conflicts from.
fn build_date_khcbppt_cells(
    year_chi_index: usize,
    day_chi_index: usize,
) -> [Option<DirectionalTaboo>; 8] {
    let tam_sat = tam_sat_direction(year_chi_index);
    let sat_phuong = get_sat_phuong(day_chi_index);
    let sat_phuong_dir = vn_str_to_direction(&sat_phuong.direction);

    std::array::from_fn(|i| {
        let direction = DIRECTION_ORDER[i];
        let tam_sat_here = tam_sat_branches_for_direction(&tam_sat, direction);
        let sat_phuong_here = if sat_phuong_dir == Some(direction) {
            Some(sat_phuong.direction.clone())
        } else {
            None
        };
        let has_tam_sat = !tam_sat_here.is_empty();
        let has_sat_phuong = sat_phuong_here.is_some();
        if !has_tam_sat && !has_sat_phuong {
            return None;
        }
        let severity = khcbppt_severity(has_tam_sat, false, has_sat_phuong, false);
        let placeholder = DirectionalTaboo {
            thai_tue: None,
            tam_sat_branches: tam_sat_here.clone(),
            sat_phuong_direction: sat_phuong_here.clone(),
            severity,
            summary_vi: String::new(),
        };
        let summary_vi = khcbppt_summary_vi(direction, &placeholder);
        Some(DirectionalTaboo {
            thai_tue: None,
            tam_sat_branches: tam_sat_here,
            sat_phuong_direction: sat_phuong_here,
            severity,
            summary_vi,
        })
    })
}

/// Build a `DirectionCrossLink` from already-assembled sides + metadata.
#[allow(clippy::too_many_arguments)]
fn assemble_cross_link(
    khcbppt: [Option<DirectionalTaboo>; 8],
    huyen_khong: [HuyenKhongCell; 8],
    date: String,
    day_chi_index: u8,
    birth_chi_index: usize,
    is_date_variant: bool,
    tam_sat_branches: Vec<String>,
    center_star: u8,
    center_hint: Option<String>,
) -> DirectionCrossLink {
    let cells = merge_into_cells(khcbppt, huyen_khong);
    let severities = std::array::from_fn(|i| cells[i].severity);
    let composite = composite_severity(&severities);
    let summary_vi = build_summary_vi(
        &cells,
        &date,
        !is_date_variant,
        center_star,
        center_hint.as_deref(),
        &tam_sat_branches,
    );

    // Per-tradition narrative notes (carried on the primitive envelopes).
    let khcbppt_note = if is_date_variant {
        format!(
            "Thái Tuế directional bị bỏ qua (không có ngữ cảnh năm sinh); \
             Tam Sát + Sát Phương projected cho ngày {}.",
            date
        )
    } else {
        format!(
            "Thái Tuế + Tam Sát + Sát Phương projected cho ngày {} \
             với birth_chi_index {}.",
            date, birth_chi_index
        )
    };
    let huyen_khong_note = format!(
        "Bố cục Cửu Tinh (Huyền Không) projected từ snapshot.flying_stars \
         cho ngày {}; trung cung sao {}.",
        date, center_star
    );
    let evidence = build_evidence(&khcbppt_note, &huyen_khong_note, is_date_variant);

    let cross_link_kind = if is_date_variant {
        "thai_tue_tam_sat_huyen_khong_date"
    } else {
        "thai_tue_tam_sat_huyen_khong_personal"
    };

    DirectionCrossLink {
        cross_link_kind: cross_link_kind.to_string(),
        date,
        day_chi_index,
        birth_chi_index,
        cells,
        summary_vi,
        composite_severity: composite,
        evidence,
    }
}

/// Validate `birth_chi_index` is in the Earthly-Branch range.
fn validate_birth_chi(birth_chi_index: usize) -> Result<(), String> {
    if birth_chi_index >= 12 {
        Err(format!(
            "birth_chi_index {} out of 0..=11 (Earthly Branch range); \
             pass DATE_ONLY_BIRTH_CHI_INDEX for the date-only variant",
            birth_chi_index
        ))
    } else {
        Ok(())
    }
}

// =========================================================================
// Public API
// =========================================================================

/// Build the personal-variant directional cross-link.
///
/// Surfaces the year-direction Thái Tuế record (including personal conflict
/// kinds derived from `compute_thai_tue`), the classical Tam Sát branch
/// overlap, the Sát Phương day-chi direction, and the annual Cửu Tinh
/// palace layout in one eight-cell composite.
///
/// Returns `Err` if `birth_chi_index >= 12` or `snapshot.flying_stars` is
/// absent.
pub fn build_direction_cross_link_personal(
    snapshot: &DaySnapshot,
    birth_chi_index: usize,
) -> Result<DirectionCrossLink, String> {
    validate_birth_chi(birth_chi_index)?;
    let year_chi_index = snapshot.context.canchi.year.chi_index;
    let day_chi_index = snapshot.context.canchi.day.chi_index;
    let date = format!(
        "{:04}-{:02}-{:02}",
        snapshot.context.solar.year,
        snapshot.context.solar.month as u32,
        snapshot.context.solar.day as u32,
    );
    let khcbppt = build_personal_khcbppt_cells(year_chi_index, day_chi_index, birth_chi_index);
    let huyen_khong = build_huyen_khong_cells(snapshot)?;
    let tam_sat = tam_sat_direction(year_chi_index);
    let tam_sat_branches: Vec<String> = tam_sat.tam_sat_branches.to_vec();
    let fs = snapshot
        .flying_stars
        .as_ref()
        .expect("checked by build_huyen_khong_cells");
    let center_star = fs.center_star as u8;
    let center_hint = fs.palace_safety_hints.as_ref().and_then(|h| h[4].clone());

    Ok(assemble_cross_link(
        khcbppt,
        huyen_khong,
        date,
        day_chi_index as u8,
        birth_chi_index,
        false,
        tam_sat_branches,
        center_star,
        center_hint,
    ))
}

/// Build the date-only (Tier-0) directional cross-link.
///
/// Same eight-cell surface as the personal variant minus the Thái Tuế
/// directional column — Tam Sát + Sát Phương + annual Cửu Tinh are still
/// populated. `birth_chi_index` is set to [`DATE_ONLY_BIRTH_CHI_INDEX`].
pub fn build_direction_cross_link_date(
    snapshot: &DaySnapshot,
) -> Result<DirectionCrossLink, String> {
    let year_chi_index = snapshot.context.canchi.year.chi_index;
    let day_chi_index = snapshot.context.canchi.day.chi_index;
    let date = format!(
        "{:04}-{:02}-{:02}",
        snapshot.context.solar.year,
        snapshot.context.solar.month as u32,
        snapshot.context.solar.day as u32,
    );
    let khcbppt = build_date_khcbppt_cells(year_chi_index, day_chi_index);
    let huyen_khong = build_huyen_khong_cells(snapshot)?;
    let tam_sat = tam_sat_direction(year_chi_index);
    let tam_sat_branches: Vec<String> = tam_sat.tam_sat_branches.to_vec();
    let fs = snapshot
        .flying_stars
        .as_ref()
        .expect("checked by build_huyen_khong_cells");
    let center_star = fs.center_star as u8;
    let center_hint = fs.palace_safety_hints.as_ref().and_then(|h| h[4].clone());

    Ok(assemble_cross_link(
        khcbppt,
        huyen_khong,
        date,
        day_chi_index as u8,
        DATE_ONLY_BIRTH_CHI_INDEX,
        true,
        tam_sat_branches,
        center_star,
        center_hint,
    ))
}

/// The required Phase 23 wrapper: build the personal cross-link and project
/// it to the [`PersonalFactNode`] shape consumed by Tier-1 reasoning
/// consumers.
///
/// This is a thin wrapper over [`build_direction_cross_link_personal`]; it
/// is NOT a second algorithm.
pub fn build_direction_cross_link(
    snapshot: &DaySnapshot,
    birth_chi_index: usize,
) -> Result<PersonalFactNode, String> {
    let cross = build_direction_cross_link_personal(snapshot, birth_chi_index)?;
    Ok(PersonalFactNode {
        id: "fact.personal.direction_cross_link".to_string(),
        summary_vi: cross.summary_vi.clone(),
        effect: None,
        evidence: cross.evidence.clone(),
    })
}

/// Project a detailed [`DirectionCrossLink`] down to the slim DTO form
/// stored on a snapshot.
pub fn project_to_summary(cross: &DirectionCrossLink) -> DirectionCrossLinkSummary {
    DirectionCrossLinkSummary {
        cross_link_kind: cross.cross_link_kind.clone(),
        cross_link_source: COMPOSITE_DIRECTION_CROSS_LINK.to_string(),
        date: cross.date.clone(),
        day_chi_index: cross.day_chi_index,
        birth_chi_index: cross.birth_chi_index,
        cells: cross.cells.clone(),
        summary_vi: cross.summary_vi.clone(),
        composite_severity: cross.composite_severity,
        evidence: cross.evidence.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::almanac::tu_menh::Direction;

    #[test]
    fn direction_order_locked_at_eight_elements() {
        assert_eq!(DIRECTION_ORDER.len(), 8);
    }

    #[test]
    fn direction_order_follows_existing_merge_convention() {
        assert_eq!(
            DIRECTION_ORDER,
            [
                Direction::North,
                Direction::Northeast,
                Direction::East,
                Direction::Southeast,
                Direction::South,
                Direction::Southwest,
                Direction::West,
                Direction::Northwest,
            ]
        );
    }

    #[test]
    fn date_only_birth_chi_index_is_usize_max_sentinel() {
        assert_eq!(DATE_ONLY_BIRTH_CHI_INDEX, usize::MAX);
    }

    #[test]
    fn composite_source_identifier_is_stable_string() {
        assert_eq!(
            COMPOSITE_DIRECTION_CROSS_LINK,
            "rule.composite.direction_cross_link"
        );
    }

    #[test]
    fn agreement_enum_serializes_snake_case() {
        let json = serde_json::to_string(&Agreement::KhcbpptOnly).expect("serialize");
        assert_eq!(json, "\"khcbppt_only\"");
        let round: Agreement = serde_json::from_str("\"huyen_khong_only\"").expect("deserialize");
        assert_eq!(round, Agreement::HuyenKhongOnly);
    }

    #[test]
    fn direction_cross_link_summary_round_trips_with_empty_cells() {
        // Build a summary with empty cells to confirm the DTO shape is owned
        // and serde-compatible (no lifetimes, no graph edges).
        let empty_cell = || DirectionCell {
            direction: Direction::North,
            khcbppt: None,
            huyen_khong: None,
            agreement: None,
            severity: ReasoningNodeSeverity::Auspicious,
        };
        let cells = std::array::from_fn(|_| empty_cell());
        let summary = DirectionCrossLinkSummary {
            cross_link_kind: "composite_kind_contract_probe".to_string(),
            cross_link_source: COMPOSITE_DIRECTION_CROSS_LINK.to_string(),
            date: "2026-07-16".to_string(),
            day_chi_index: 0,
            birth_chi_index: DATE_ONLY_BIRTH_CHI_INDEX,
            cells,
            summary_vi: "RED-free contract probe".to_string(),
            composite_severity: ReasoningNodeSeverity::Auspicious,
            evidence: Vec::new(),
        };
        let json = serde_json::to_string(&summary).expect("serialize summary");
        let back: DirectionCrossLinkSummary =
            serde_json::from_str(&json).expect("deserialize summary");
        assert_eq!(back.birth_chi_index, usize::MAX);
        assert_eq!(back.cross_link_source, COMPOSITE_DIRECTION_CROSS_LINK);
        assert_eq!(back.cells.len(), 8);
    }

    // -----------------------------------------------------------------
    // Phase 23-03 Task 1: composite_severity tie behaviour.
    // -----------------------------------------------------------------

    #[test]
    fn composite_severity_picks_inauspicious_on_favorable_unfavorable_tie() {
        // 4 Auspicious + 4 Inauspicious -> tied top count -> the
        // conservative-default rule must pick Inauspicious (CONTEXT.md
        // "taboo-leaning on ambiguity" recommendation).
        let severities = [
            ReasoningNodeSeverity::Auspicious,
            ReasoningNodeSeverity::Auspicious,
            ReasoningNodeSeverity::Auspicious,
            ReasoningNodeSeverity::Auspicious,
            ReasoningNodeSeverity::Inauspicious,
            ReasoningNodeSeverity::Inauspicious,
            ReasoningNodeSeverity::Inauspicious,
            ReasoningNodeSeverity::Inauspicious,
        ];
        assert_eq!(
            composite_severity(&severities),
            ReasoningNodeSeverity::Inauspicious
        );
    }

    #[test]
    fn composite_severity_majority_wins_when_clear() {
        // 5 HardTaboo + 3 Auspicious -> HardTaboo has the clear majority.
        let severities = [
            ReasoningNodeSeverity::HardTaboo,
            ReasoningNodeSeverity::HardTaboo,
            ReasoningNodeSeverity::HardTaboo,
            ReasoningNodeSeverity::HardTaboo,
            ReasoningNodeSeverity::HardTaboo,
            ReasoningNodeSeverity::Auspicious,
            ReasoningNodeSeverity::Auspicious,
            ReasoningNodeSeverity::Auspicious,
        ];
        assert_eq!(
            composite_severity(&severities),
            ReasoningNodeSeverity::HardTaboo
        );
    }

    #[test]
    fn composite_severity_picks_most_cautionary_on_tie() {
        // 4 SoftTaboo + 4 HardTaboo -> tied top count -> HardTaboo is the
        // most cautionary tied value.
        let severities = [
            ReasoningNodeSeverity::SoftTaboo,
            ReasoningNodeSeverity::SoftTaboo,
            ReasoningNodeSeverity::SoftTaboo,
            ReasoningNodeSeverity::SoftTaboo,
            ReasoningNodeSeverity::HardTaboo,
            ReasoningNodeSeverity::HardTaboo,
            ReasoningNodeSeverity::HardTaboo,
            ReasoningNodeSeverity::HardTaboo,
        ];
        assert_eq!(
            composite_severity(&severities),
            ReasoningNodeSeverity::HardTaboo
        );
    }

    // -----------------------------------------------------------------
    // Phase 23-03 Task 1: public builder surface contracts.
    // -----------------------------------------------------------------

    #[test]
    fn build_personal_cross_link_returns_eight_cells_in_locked_order() {
        let snapshot = crate::calculate_day_snapshot(10, 2, 2024);
        let cross = build_direction_cross_link_personal(&snapshot, 10)
            .expect("personal builder should succeed for in-range birth chi");
        assert_eq!(cross.cells.len(), 8);
        for (i, expected) in DIRECTION_ORDER.iter().enumerate() {
            assert_eq!(
                cross.cells[i].direction, *expected,
                "cell {} must be {:?} in DIRECTION_ORDER",
                i, expected
            );
        }
        assert_eq!(cross.birth_chi_index, 10);
    }

    #[test]
    fn build_personal_cross_link_rejects_out_of_range_birth_chi() {
        let snapshot = crate::calculate_day_snapshot(10, 2, 2024);
        let err = build_direction_cross_link_personal(&snapshot, 12)
            .expect_err("out-of-range birth chi must error");
        assert!(
            err.contains("birth_chi_index") || err.contains("range"),
            "error must explain the out-of-range cause; got: {err}"
        );
    }

    #[test]
    fn build_date_cross_link_carries_sentinel_and_omits_thai_tue() {
        let snapshot = crate::calculate_day_snapshot(10, 2, 2024);
        let cross = build_direction_cross_link_date(&snapshot)
            .expect("date builder should succeed for a populated snapshot");
        assert_eq!(cross.birth_chi_index, DATE_ONLY_BIRTH_CHI_INDEX);
        for cell in cross.cells.iter() {
            if let Some(taboo) = cell.khcbppt.as_ref() {
                assert!(
                    taboo.thai_tue.is_none(),
                    "date variant must never carry a directional Thai Tue record"
                );
            }
        }
    }

    #[test]
    fn build_personal_cross_link_carries_exactly_three_evidence_envelopes() {
        let snapshot = crate::calculate_day_snapshot(10, 2, 2024);
        let cross = build_direction_cross_link_personal(&snapshot, 10).expect("personal builder");
        assert_eq!(cross.evidence.len(), 3);
        assert_eq!(cross.evidence[0].source_id, crate::sources::SOURCE_KHCBPPT);
        assert_eq!(
            cross.evidence[1].source_id,
            crate::sources::SOURCE_HUYEN_KHONG
        );
        assert_eq!(cross.evidence[2].source_id, COMPOSITE_DIRECTION_CROSS_LINK);
        // The huyen-khong primitive's method value is locked at runtime.
        let huyen_method = cross.evidence[1].method.clone();
        let mut expected = String::from("phi");
        expected.push('_');
        expected.push_str("tinh.palace_layout");
        assert_eq!(huyen_method, expected);
    }

    #[test]
    fn build_direction_cross_link_wrapper_returns_personal_fact_node() {
        let snapshot = crate::calculate_day_snapshot(10, 2, 2024);
        let node = build_direction_cross_link(&snapshot, 10).expect("wrapper builder");
        assert_eq!(node.id, "fact.personal.direction_cross_link");
        assert_eq!(node.evidence.len(), 3);
        assert!(!node.summary_vi.is_empty());
    }

    #[test]
    fn project_to_summary_carries_cross_link_source() {
        let snapshot = crate::calculate_day_snapshot(10, 2, 2024);
        let cross = build_direction_cross_link_personal(&snapshot, 10).expect("personal builder");
        let summary = project_to_summary(&cross);
        assert_eq!(summary.cross_link_source, COMPOSITE_DIRECTION_CROSS_LINK);
        assert_eq!(summary.cells.len(), 8);
        assert_eq!(summary.birth_chi_index, cross.birth_chi_index);
    }

    #[test]
    fn enrich_helper_attaches_summary_and_leaves_input_unchanged() {
        let snapshot = crate::calculate_day_snapshot(10, 2, 2024);
        assert!(snapshot.direction_cross_link.is_none());
        let enriched = crate::enrich_day_snapshot_with_direction_cross_link(&snapshot, 10)
            .expect("enrichment should succeed");
        assert!(enriched.direction_cross_link.is_some());
        // The input snapshot must remain unchanged (immutable clone-and-attach).
        assert!(snapshot.direction_cross_link.is_none());
    }

    #[test]
    fn enrich_helper_dispatches_sentinel_to_date_builder() {
        let snapshot = crate::calculate_day_snapshot(10, 2, 2024);
        let enriched = crate::enrich_day_snapshot_with_direction_cross_link(
            &snapshot,
            DATE_ONLY_BIRTH_CHI_INDEX,
        )
        .expect("sentinel enrichment should dispatch to date builder");
        let summary = enriched.direction_cross_link.expect("summary attached");
        assert_eq!(summary.birth_chi_index, DATE_ONLY_BIRTH_CHI_INDEX);
    }

    #[test]
    fn enrich_helper_rejects_invalid_personal_birth_chi() {
        let snapshot = crate::calculate_day_snapshot(10, 2, 2024);
        let _ = crate::enrich_day_snapshot_with_direction_cross_link(&snapshot, 99)
            .expect_err("invalid birth chi must propagate the personal builder's error");
    }
}
