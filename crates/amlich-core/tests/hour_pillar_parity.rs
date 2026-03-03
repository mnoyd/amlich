use amlich_core::almanac::hour_pillar::{compute_hour_pillar, resolve_hour_branch_slot};
use amlich_core::almanac::types::HeavenlyStem;

fn assert_stable_metadata(source_id: &str, method: &str, profile: &str) {
    assert!(!source_id.trim().is_empty());
    assert!(!method.trim().is_empty());
    assert!(!profile.trim().is_empty());
    assert_eq!(source_id, "khcbppt");
    assert_eq!(method, "hour-pillar-seed-table");
    assert_eq!(profile, "baseline");
}

#[test]
fn parity_fixture_matrix() {
    let fixtures = [
        (HeavenlyStem::Giap, 23, 0, 0, "Tý", "Giáp Tý"),
        (HeavenlyStem::At, 1, 0, 1, "Sửu", "Đinh Sửu"),
        (HeavenlyStem::Binh, 5, 30, 3, "Mão", "Tân Mão"),
        (HeavenlyStem::Dinh, 11, 45, 6, "Ngọ", "Bính Ngọ"),
        (HeavenlyStem::Mau, 17, 0, 9, "Dậu", "Tân Dậu"),
        (HeavenlyStem::Giap, 21, 15, 11, "Hợi", "Ất Hợi"),
        (HeavenlyStem::At, 23, 59, 0, "Tý", "Bính Tý"),
        (HeavenlyStem::At, 0, 0, 0, "Tý", "Bính Tý"),
        (HeavenlyStem::At, 0, 59, 0, "Tý", "Bính Tý"),
        (HeavenlyStem::At, 1, 0, 1, "Sửu", "Đinh Sửu"),
    ];

    for (day_stem, hour, minute, expected_slot, expected_branch, expected_full) in fixtures {
        let result = compute_hour_pillar(day_stem, hour, minute).expect("fixture should resolve");
        assert_eq!(result.slot.slot_index, expected_slot);
        assert_eq!(result.slot.branch, expected_branch);
        assert_eq!(result.can_chi.full, expected_full);
        assert_stable_metadata(
            &result.evidence.source_id,
            &result.evidence.method,
            &result.evidence.profile,
        );
    }

    assert!(resolve_hour_branch_slot(24, 0).is_none());
    assert!(resolve_hour_branch_slot(23, 60).is_none());
}

#[test]
fn validate_hour_slot_boundaries_all_transitions() {
    let transitions = [
        (0, 59, 1, 0),
        (2, 59, 3, 0),
        (4, 59, 5, 0),
        (6, 59, 7, 0),
        (8, 59, 9, 0),
        (10, 59, 11, 0),
        (12, 59, 13, 0),
        (14, 59, 15, 0),
        (16, 59, 17, 0),
        (18, 59, 19, 0),
        (20, 59, 21, 0),
        (22, 59, 23, 0),
    ];

    for (left_hour, left_minute, right_hour, right_minute) in transitions {
        let left =
            resolve_hour_branch_slot(left_hour, left_minute).expect("left boundary should resolve");
        let right = resolve_hour_branch_slot(right_hour, right_minute)
            .expect("right boundary should resolve");

        let expected_next = (left.slot_index + 1) % 12;
        assert_eq!(right.slot_index, expected_next);
    }
}

#[test]
fn validate_hour_pillar_parity_matrix() {
    let expected = [
        "Giáp Tý",
        "Ất Sửu",
        "Bính Dần",
        "Đinh Mão",
        "Mậu Thìn",
        "Kỷ Tỵ",
        "Canh Ngọ",
        "Tân Mùi",
        "Nhâm Thân",
        "Quý Dậu",
        "Giáp Tuất",
        "Ất Hợi",
    ];

    for (slot_index, expected_full) in expected.iter().enumerate() {
        let (hour, minute) = if slot_index == 0 {
            (23, 0)
        } else {
            (((slot_index * 2) - 1) as u8, 0)
        };

        let result = compute_hour_pillar(HeavenlyStem::Giap, hour, minute)
            .expect("parity slot should resolve");
        assert_eq!(result.slot.slot_index, slot_index);
        assert_eq!(result.can_chi.full, *expected_full);
        assert_stable_metadata(
            &result.evidence.source_id,
            &result.evidence.method,
            &result.evidence.profile,
        );
    }
}
