use crate::almanac::phuc_than::get_phuc_than;
use crate::almanac::sat_phuong::get_sat_phuong;
use crate::almanac::tu_menh::{Direction, KuaResult};
use crate::almanac::types::RuleEvidence;
use crate::types::CanChi;

use super::types::{DirectionEntry, DirectionMergeMatrix, DirectionSignal};

const ALL_DIRECTIONS: [&str; 8] = [
    "Bắc",
    "Đông Bắc",
    "Đông",
    "Đông Nam",
    "Nam",
    "Tây Nam",
    "Tây",
    "Tây Bắc",
];

/// Compute the Direction Merge Matrix.
///
/// For each of 8 compass directions, aggregates signals from:
/// - Kua (Bát Trạch): personal favorable/unfavorable (static)
/// - Tài Thần: day's Wealth God direction
/// - Hỷ Thần: day's Joy God direction
/// - Phúc Thần: day's Fortune God direction
/// - Sát Phương: day's Killing direction
pub fn compute_direction_merge(
    day_canchi: &CanChi,
    tai_than: &str,
    hy_than: &str,
    kua: &KuaResult,
) -> DirectionMergeMatrix {
    let phuc_than = get_phuc_than(day_canchi.can_index);
    let sat_phuong = get_sat_phuong(day_canchi.chi_index);

    let entries: Vec<DirectionEntry> = ALL_DIRECTIONS
        .iter()
        .map(|&dir| {
            let mut signals = Vec::new();

            // Kua signals
            if kua_has_direction(&kua.favorable_directions, dir) {
                signals.push(DirectionSignal::KuaFavorable);
            }
            if kua_has_direction(&kua.unfavorable_directions, dir) {
                signals.push(DirectionSignal::KuaUnfavorable);
            }

            // Day deity signals
            if direction_matches(tai_than, dir) {
                signals.push(DirectionSignal::TaiThan);
            }
            if direction_matches(hy_than, dir) {
                signals.push(DirectionSignal::HyThan);
            }
            if direction_matches(&phuc_than.direction, dir) {
                signals.push(DirectionSignal::PhucThan);
            }
            if direction_matches(&sat_phuong.direction, dir) {
                signals.push(DirectionSignal::SatPhuong);
            }

            let favorable_count = signals.iter().filter(|s| s.is_favorable()).count() as i8;
            let unfavorable_count = signals.iter().filter(|s| s.is_unfavorable()).count() as i8;

            DirectionEntry {
                direction: dir.to_string(),
                signals: signals.clone(),
                favorable_count,
                unfavorable_count,
                net_score: favorable_count - unfavorable_count,
            }
        })
        .collect();

    DirectionMergeMatrix {
        day_canchi: day_canchi.full.clone(),
        kua_number: kua.kua,
        entries,
        evidence: RuleEvidence {
            source_id: crate::sources::SOURCE_KHCBPPT.to_string(),
            method: "direction-merge-matrix".to_string(),
            profile: "baseline".to_string(),
        },
    }
}

/// Check if a Kua Direction array contains a direction matching the Vietnamese string.
fn kua_has_direction(dirs: &[Direction; 4], vn_dir: &str) -> bool {
    dirs.iter().any(|d| direction_to_vn(d) == vn_dir)
}

/// Convert Direction enum to Vietnamese direction string.
fn direction_to_vn(d: &Direction) -> &'static str {
    match d {
        Direction::North => "Bắc",
        Direction::Northeast => "Đông Bắc",
        Direction::East => "Đông",
        Direction::Southeast => "Đông Nam",
        Direction::South => "Nam",
        Direction::Southwest => "Tây Nam",
        Direction::West => "Tây",
        Direction::Northwest => "Tây Bắc",
    }
}

/// Check if two Vietnamese direction strings match.
/// Handles exact and cardinal-only matching (e.g. "Nam" matches "Nam" but not "Tây Nam").
fn direction_matches(source: &str, target: &str) -> bool {
    source == target
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::almanac::tu_menh::{ConventionMetadata, KuaGroup};

    fn make_kua() -> KuaResult {
        KuaResult::new(
            1,
            KuaGroup::East,
            [
                Direction::Southeast,
                Direction::East,
                Direction::South,
                Direction::North,
            ],
            [
                Direction::West,
                Direction::Northwest,
                Direction::Southwest,
                Direction::Northeast,
            ],
            ConventionMetadata {
                year_basis: "test".to_string(),
                kua5_resolution: "test".to_string(),
                gender_encoding: "test".to_string(),
            },
        )
    }

    #[test]
    fn matrix_has_8_directions() {
        let day = CanChi::new(0, 0); // Giáp Tý
        let kua = make_kua();
        let matrix = compute_direction_merge(&day, "Tây Nam", "Đông Bắc", &kua);
        assert_eq!(matrix.entries.len(), 8);
    }

    #[test]
    fn kua_favorable_is_detected() {
        let day = CanChi::new(0, 0);
        let kua = make_kua(); // favorable: SE, E, S, N
        let matrix = compute_direction_merge(&day, "Tây", "Tây", &kua);
        let south = matrix
            .entries
            .iter()
            .find(|e| e.direction == "Nam")
            .unwrap();
        assert!(south.signals.contains(&DirectionSignal::KuaFavorable));
    }

    #[test]
    fn kua_unfavorable_is_detected() {
        let day = CanChi::new(0, 0);
        let kua = make_kua(); // unfavorable: W, NW, SW, NE
        let matrix = compute_direction_merge(&day, "Bắc", "Bắc", &kua);
        let west = matrix
            .entries
            .iter()
            .find(|e| e.direction == "Tây")
            .unwrap();
        assert!(west.signals.contains(&DirectionSignal::KuaUnfavorable));
    }

    #[test]
    fn tai_than_signal_is_detected() {
        let day = CanChi::new(0, 0);
        let kua = make_kua();
        let matrix = compute_direction_merge(&day, "Đông Nam", "Bắc", &kua);
        let se = matrix
            .entries
            .iter()
            .find(|e| e.direction == "Đông Nam")
            .unwrap();
        assert!(se.signals.contains(&DirectionSignal::TaiThan));
    }

    #[test]
    fn sat_phuong_signal_is_detected() {
        // Tý(0) → Sát Phương = Nam
        let day = CanChi::new(0, 0);
        let kua = make_kua();
        let matrix = compute_direction_merge(&day, "Bắc", "Bắc", &kua);
        let south = matrix
            .entries
            .iter()
            .find(|e| e.direction == "Nam")
            .unwrap();
        assert!(south.signals.contains(&DirectionSignal::SatPhuong));
    }

    #[test]
    fn phuc_than_signal_is_detected() {
        // Giáp(0) → Phúc Thần = Bắc
        let day = CanChi::new(0, 0);
        let kua = make_kua();
        let matrix = compute_direction_merge(&day, "Tây", "Tây", &kua);
        let north = matrix
            .entries
            .iter()
            .find(|e| e.direction == "Bắc")
            .unwrap();
        assert!(north.signals.contains(&DirectionSignal::PhucThan));
    }

    #[test]
    fn net_score_is_favorable_minus_unfavorable() {
        let day = CanChi::new(0, 0);
        let kua = make_kua();
        let matrix = compute_direction_merge(&day, "Bắc", "Bắc", &kua);
        for entry in &matrix.entries {
            assert_eq!(
                entry.net_score,
                entry.favorable_count - entry.unfavorable_count
            );
        }
    }

    #[test]
    fn matrix_serializes_to_json() {
        let day = CanChi::new(0, 0);
        let kua = make_kua();
        let matrix = compute_direction_merge(&day, "Tây Nam", "Đông Bắc", &kua);
        let json = serde_json::to_string(&matrix).expect("should serialize");
        assert!(json.contains("\"direction\""));
        assert!(json.contains("\"signals\""));
        assert!(json.contains("\"net_score\""));
    }

    #[test]
    fn evidence_is_set() {
        let day = CanChi::new(0, 0);
        let kua = make_kua();
        let matrix = compute_direction_merge(&day, "Bắc", "Bắc", &kua);
        assert_eq!(matrix.evidence.source_id, "khcbppt");
        assert_eq!(matrix.evidence.method, "direction-merge-matrix");
    }
}
