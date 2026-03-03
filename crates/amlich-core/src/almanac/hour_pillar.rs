use crate::gio_hoang_dao::get_hour_time_range;
use crate::types::{normalize_index, CanChi, CHI};

use super::types::{HeavenlyStem, RuleEvidence};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HourBranchSlot {
    pub slot_index: usize,
    pub branch_index: usize,
    pub branch: String,
    pub time_range: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HourPillarResult {
    pub can_chi: CanChi,
    pub slot: HourBranchSlot,
    pub evidence: RuleEvidence,
}

fn ty_hour_seed_stem_index(day_stem: HeavenlyStem) -> usize {
    match day_stem {
        HeavenlyStem::Giap | HeavenlyStem::Ky => 0,
        HeavenlyStem::At | HeavenlyStem::Canh => 2,
        HeavenlyStem::Binh | HeavenlyStem::Tan => 4,
        HeavenlyStem::Dinh | HeavenlyStem::Nham => 6,
        HeavenlyStem::Mau | HeavenlyStem::Quy => 8,
    }
}

fn derive_hour_stem_index(seed_index: usize, slot_index: usize) -> usize {
    normalize_index((seed_index + slot_index) as i32, 10)
}

pub fn resolve_hour_branch_slot(local_hour: u8, local_minute: u8) -> Option<HourBranchSlot> {
    if local_hour > 23 || local_minute > 59 {
        return None;
    }

    let slot_index = if local_hour == 23 || local_hour == 0 {
        0
    } else {
        (local_hour as usize + 1) / 2
    };

    Some(HourBranchSlot {
        slot_index,
        branch_index: slot_index,
        branch: CHI[slot_index].to_string(),
        time_range: get_hour_time_range(slot_index).to_string(),
    })
}

pub fn compute_hour_pillar(
    day_stem: HeavenlyStem,
    local_hour: u8,
    local_minute: u8,
) -> Option<HourPillarResult> {
    let slot = resolve_hour_branch_slot(local_hour, local_minute)?;
    let seed_index = ty_hour_seed_stem_index(day_stem);
    let stem_index = derive_hour_stem_index(seed_index, slot.slot_index);

    Some(HourPillarResult {
        can_chi: CanChi::new(stem_index, slot.branch_index),
        slot,
        evidence: RuleEvidence {
            source_id: "khcbppt".to_string(),
            method: "hour-pillar-seed-table".to_string(),
            profile: "baseline".to_string(),
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_mapping() {
        let fixtures = [
            (HeavenlyStem::Giap, 0),
            (HeavenlyStem::Ky, 0),
            (HeavenlyStem::At, 2),
            (HeavenlyStem::Canh, 2),
            (HeavenlyStem::Binh, 4),
            (HeavenlyStem::Tan, 4),
            (HeavenlyStem::Dinh, 6),
            (HeavenlyStem::Nham, 6),
            (HeavenlyStem::Mau, 8),
            (HeavenlyStem::Quy, 8),
        ];

        for (day_stem, expected_seed) in fixtures {
            assert_eq!(ty_hour_seed_stem_index(day_stem), expected_seed);
        }
    }

    #[test]
    fn stem_progression_rolls_modulo_ten() {
        let expected = [4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5];
        for (slot, expected_stem_index) in expected.iter().enumerate() {
            assert_eq!(derive_hour_stem_index(4, slot), *expected_stem_index);
        }
    }

    #[test]
    fn slot_boundaries_have_no_overlap_or_gap() {
        let transitions = [
            (0, 59, 1, 0, 0, 1),
            (2, 59, 3, 0, 1, 2),
            (4, 59, 5, 0, 2, 3),
            (6, 59, 7, 0, 3, 4),
            (8, 59, 9, 0, 4, 5),
            (10, 59, 11, 0, 5, 6),
            (12, 59, 13, 0, 6, 7),
            (14, 59, 15, 0, 7, 8),
            (16, 59, 17, 0, 8, 9),
            (18, 59, 19, 0, 9, 10),
            (20, 59, 21, 0, 10, 11),
            (22, 59, 23, 0, 11, 0),
        ];

        for (h1, m1, h2, m2, s1, s2) in transitions {
            let left = resolve_hour_branch_slot(h1, m1).expect("left slot must resolve");
            let right = resolve_hour_branch_slot(h2, m2).expect("right slot must resolve");
            assert_eq!(left.slot_index, s1);
            assert_eq!(right.slot_index, s2);
        }

        assert_eq!(
            resolve_hour_branch_slot(0, 59)
                .expect("00:59 must resolve")
                .slot_index,
            0
        );
        assert_eq!(
            resolve_hour_branch_slot(1, 0)
                .expect("01:00 must resolve")
                .slot_index,
            1
        );
    }

    #[test]
    fn compute_hour_pillar_provides_stable_evidence() {
        let result =
            compute_hour_pillar(HeavenlyStem::Giap, 23, 0).expect("valid hour must resolve");
        assert_eq!(result.can_chi.full, "Giáp Tý");
        assert_eq!(result.evidence.source_id, "khcbppt");
        assert_eq!(result.evidence.method, "hour-pillar-seed-table");
        assert_eq!(result.evidence.profile, "baseline");
    }

    #[test]
    fn module_exports_are_accessible_via_almanac_namespace() {
        let slot = crate::almanac::hour_pillar::resolve_hour_branch_slot(23, 30);
        assert!(slot.is_some());

        let pillar = crate::almanac::hour_pillar::compute_hour_pillar(HeavenlyStem::At, 1, 0);
        assert_eq!(
            pillar.expect("pillar should compute").can_chi.full,
            "Đinh Sửu"
        );
    }
}
