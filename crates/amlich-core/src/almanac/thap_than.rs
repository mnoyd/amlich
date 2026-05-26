use crate::almanac::types::{
    FiveElement, FiveElementRelation, HeavenlyStem, RuleEvidence, ThapThanLabel, ThapThanResult,
};

pub fn get_thap_than(day_can: HeavenlyStem, target_can: HeavenlyStem) -> ThapThanResult {
    let relation = relation_between(day_can, target_can);
    let same_polarity = day_can.polarity() == target_can.polarity();
    let label = map_label(relation, same_polarity);

    ThapThanResult {
        label,
        relation,
        same_polarity,
        evidence: RuleEvidence {
            source_id: crate::sources::SOURCE_KHCBPPT.to_string(),
            method: "five-element-polarity-matrix".to_string(),
            profile: "baseline".to_string(),
        },
    }
}

fn relation_between(day_can: HeavenlyStem, target_can: HeavenlyStem) -> FiveElementRelation {
    let day_element = day_can.element();
    let target_element = target_can.element();

    if day_element == target_element {
        return FiveElementRelation::Same;
    }

    if generates(day_element) == target_element {
        return FiveElementRelation::DayGeneratesTarget;
    }

    if generates(target_element) == day_element {
        return FiveElementRelation::TargetGeneratesDay;
    }

    if controls(day_element) == target_element {
        return FiveElementRelation::DayControlsTarget;
    }

    FiveElementRelation::TargetControlsDay
}

fn generates(element: FiveElement) -> FiveElement {
    match element {
        FiveElement::Moc => FiveElement::Hoa,
        FiveElement::Hoa => FiveElement::Tho,
        FiveElement::Tho => FiveElement::Kim,
        FiveElement::Kim => FiveElement::Thuy,
        FiveElement::Thuy => FiveElement::Moc,
    }
}

fn controls(element: FiveElement) -> FiveElement {
    match element {
        FiveElement::Moc => FiveElement::Tho,
        FiveElement::Hoa => FiveElement::Kim,
        FiveElement::Tho => FiveElement::Thuy,
        FiveElement::Kim => FiveElement::Moc,
        FiveElement::Thuy => FiveElement::Hoa,
    }
}

fn map_label(relation: FiveElementRelation, same_polarity: bool) -> ThapThanLabel {
    match (relation, same_polarity) {
        (FiveElementRelation::Same, true) => ThapThanLabel::TyKien,
        (FiveElementRelation::Same, false) => ThapThanLabel::KiepTai,
        (FiveElementRelation::DayGeneratesTarget, true) => ThapThanLabel::ThucThan,
        (FiveElementRelation::DayGeneratesTarget, false) => ThapThanLabel::ThuongQuan,
        (FiveElementRelation::DayControlsTarget, true) => ThapThanLabel::ThienTai,
        (FiveElementRelation::DayControlsTarget, false) => ThapThanLabel::ChinhTai,
        (FiveElementRelation::TargetControlsDay, true) => ThapThanLabel::ThatSat,
        (FiveElementRelation::TargetControlsDay, false) => ThapThanLabel::ChinhQuan,
        (FiveElementRelation::TargetGeneratesDay, true) => ThapThanLabel::ThienAn,
        (FiveElementRelation::TargetGeneratesDay, false) => ThapThanLabel::ChinhAn,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod api_contract {
        use super::*;

        #[test]
        fn ten_gods_label_has_10_variants() {
            let all = [
                ThapThanLabel::TyKien,
                ThapThanLabel::KiepTai,
                ThapThanLabel::ThucThan,
                ThapThanLabel::ThuongQuan,
                ThapThanLabel::ChinhTai,
                ThapThanLabel::ThienTai,
                ThapThanLabel::ChinhQuan,
                ThapThanLabel::ThatSat,
                ThapThanLabel::ChinhAn,
                ThapThanLabel::ThienAn,
            ];
            assert_eq!(all.len(), 10);
        }

        #[test]
        fn api_returns_typed_serializable_result() {
            let result = get_thap_than(HeavenlyStem::Giap, HeavenlyStem::At);
            let json = serde_json::to_string(&result).expect("serialize");
            assert!(json.contains("\"label\""));
            assert!(json.contains("\"evidence\""));
            assert!(json.contains("\"source_id\":\"khcbppt\""));
        }

        #[test]
        fn invalid_stem_input_is_explicit_error() {
            let parsed = HeavenlyStem::try_from("INVALID");
            assert!(parsed.is_err());
            assert_eq!(
                parsed.expect_err("should fail"),
                "invalid heavenly stem: invalid"
            );
        }
    }

    mod mapping {
        use super::*;

        #[test]
        fn same_element_same_polarity_maps_to_ty_kien() {
            let result = get_thap_than(HeavenlyStem::Giap, HeavenlyStem::Giap);
            assert_eq!(result.label, ThapThanLabel::TyKien);
            assert_eq!(result.relation, FiveElementRelation::Same);
            assert!(result.same_polarity);
        }

        #[test]
        fn same_element_opposite_polarity_maps_to_kiep_tai() {
            let result = get_thap_than(HeavenlyStem::Giap, HeavenlyStem::At);
            assert_eq!(result.label, ThapThanLabel::KiepTai);
            assert_eq!(result.relation, FiveElementRelation::Same);
            assert!(!result.same_polarity);
        }

        #[test]
        fn five_element_relationships_map_correctly_by_polarity() {
            assert_eq!(
                get_thap_than(HeavenlyStem::Giap, HeavenlyStem::Binh).label,
                ThapThanLabel::ThucThan
            );
            assert_eq!(
                get_thap_than(HeavenlyStem::At, HeavenlyStem::Binh).label,
                ThapThanLabel::ThuongQuan
            );

            assert_eq!(
                get_thap_than(HeavenlyStem::Giap, HeavenlyStem::Mau).label,
                ThapThanLabel::ThienTai
            );
            assert_eq!(
                get_thap_than(HeavenlyStem::Giap, HeavenlyStem::Ky).label,
                ThapThanLabel::ChinhTai
            );

            assert_eq!(
                get_thap_than(HeavenlyStem::Giap, HeavenlyStem::Canh).label,
                ThapThanLabel::ThatSat
            );
            assert_eq!(
                get_thap_than(HeavenlyStem::Giap, HeavenlyStem::Tan).label,
                ThapThanLabel::ChinhQuan
            );

            assert_eq!(
                get_thap_than(HeavenlyStem::Giap, HeavenlyStem::Nham).label,
                ThapThanLabel::ThienAn
            );
            assert_eq!(
                get_thap_than(HeavenlyStem::Giap, HeavenlyStem::Quy).label,
                ThapThanLabel::ChinhAn
            );
        }

        #[test]
        fn full_10x10_matrix_matches_expected_labels() {
            let expected = [
                [
                    ThapThanLabel::TyKien,
                    ThapThanLabel::KiepTai,
                    ThapThanLabel::ThucThan,
                    ThapThanLabel::ThuongQuan,
                    ThapThanLabel::ThienTai,
                    ThapThanLabel::ChinhTai,
                    ThapThanLabel::ThatSat,
                    ThapThanLabel::ChinhQuan,
                    ThapThanLabel::ThienAn,
                    ThapThanLabel::ChinhAn,
                ],
                [
                    ThapThanLabel::KiepTai,
                    ThapThanLabel::TyKien,
                    ThapThanLabel::ThuongQuan,
                    ThapThanLabel::ThucThan,
                    ThapThanLabel::ChinhTai,
                    ThapThanLabel::ThienTai,
                    ThapThanLabel::ChinhQuan,
                    ThapThanLabel::ThatSat,
                    ThapThanLabel::ChinhAn,
                    ThapThanLabel::ThienAn,
                ],
                [
                    ThapThanLabel::ThienAn,
                    ThapThanLabel::ChinhAn,
                    ThapThanLabel::TyKien,
                    ThapThanLabel::KiepTai,
                    ThapThanLabel::ThucThan,
                    ThapThanLabel::ThuongQuan,
                    ThapThanLabel::ThienTai,
                    ThapThanLabel::ChinhTai,
                    ThapThanLabel::ThatSat,
                    ThapThanLabel::ChinhQuan,
                ],
                [
                    ThapThanLabel::ChinhAn,
                    ThapThanLabel::ThienAn,
                    ThapThanLabel::KiepTai,
                    ThapThanLabel::TyKien,
                    ThapThanLabel::ThuongQuan,
                    ThapThanLabel::ThucThan,
                    ThapThanLabel::ChinhTai,
                    ThapThanLabel::ThienTai,
                    ThapThanLabel::ChinhQuan,
                    ThapThanLabel::ThatSat,
                ],
                [
                    ThapThanLabel::ThatSat,
                    ThapThanLabel::ChinhQuan,
                    ThapThanLabel::ThienAn,
                    ThapThanLabel::ChinhAn,
                    ThapThanLabel::TyKien,
                    ThapThanLabel::KiepTai,
                    ThapThanLabel::ThucThan,
                    ThapThanLabel::ThuongQuan,
                    ThapThanLabel::ThienTai,
                    ThapThanLabel::ChinhTai,
                ],
                [
                    ThapThanLabel::ChinhQuan,
                    ThapThanLabel::ThatSat,
                    ThapThanLabel::ChinhAn,
                    ThapThanLabel::ThienAn,
                    ThapThanLabel::KiepTai,
                    ThapThanLabel::TyKien,
                    ThapThanLabel::ThuongQuan,
                    ThapThanLabel::ThucThan,
                    ThapThanLabel::ChinhTai,
                    ThapThanLabel::ThienTai,
                ],
                [
                    ThapThanLabel::ThienTai,
                    ThapThanLabel::ChinhTai,
                    ThapThanLabel::ThatSat,
                    ThapThanLabel::ChinhQuan,
                    ThapThanLabel::ThienAn,
                    ThapThanLabel::ChinhAn,
                    ThapThanLabel::TyKien,
                    ThapThanLabel::KiepTai,
                    ThapThanLabel::ThucThan,
                    ThapThanLabel::ThuongQuan,
                ],
                [
                    ThapThanLabel::ChinhTai,
                    ThapThanLabel::ThienTai,
                    ThapThanLabel::ChinhQuan,
                    ThapThanLabel::ThatSat,
                    ThapThanLabel::ChinhAn,
                    ThapThanLabel::ThienAn,
                    ThapThanLabel::KiepTai,
                    ThapThanLabel::TyKien,
                    ThapThanLabel::ThuongQuan,
                    ThapThanLabel::ThucThan,
                ],
                [
                    ThapThanLabel::ThucThan,
                    ThapThanLabel::ThuongQuan,
                    ThapThanLabel::ThienTai,
                    ThapThanLabel::ChinhTai,
                    ThapThanLabel::ThatSat,
                    ThapThanLabel::ChinhQuan,
                    ThapThanLabel::ThienAn,
                    ThapThanLabel::ChinhAn,
                    ThapThanLabel::TyKien,
                    ThapThanLabel::KiepTai,
                ],
                [
                    ThapThanLabel::ThuongQuan,
                    ThapThanLabel::ThucThan,
                    ThapThanLabel::ChinhTai,
                    ThapThanLabel::ThienTai,
                    ThapThanLabel::ChinhQuan,
                    ThapThanLabel::ThatSat,
                    ThapThanLabel::ChinhAn,
                    ThapThanLabel::ThienAn,
                    ThapThanLabel::KiepTai,
                    ThapThanLabel::TyKien,
                ],
            ];

            for (day_idx, day_can) in HeavenlyStem::ALL.into_iter().enumerate() {
                for (target_idx, target_can) in HeavenlyStem::ALL.into_iter().enumerate() {
                    let result = get_thap_than(day_can, target_can);
                    assert_eq!(result.label, expected[day_idx][target_idx]);
                }
            }
        }

        #[test]
        fn matrix_results_are_deterministic_across_repeated_calls() {
            for day_can in HeavenlyStem::ALL {
                for target_can in HeavenlyStem::ALL {
                    let first = get_thap_than(day_can, target_can);
                    let second = get_thap_than(day_can, target_can);
                    assert_eq!(first, second);
                }
            }
        }

        #[test]
        fn evidence_metadata_is_consistent_for_all_outputs() {
            for day_can in HeavenlyStem::ALL {
                for target_can in HeavenlyStem::ALL {
                    let result = get_thap_than(day_can, target_can);
                    assert_eq!(result.evidence.source_id, "khcbppt");
                    assert_eq!(result.evidence.method, "five-element-polarity-matrix");
                    assert_eq!(result.evidence.profile, "baseline");
                }
            }
        }

        #[test]
        fn json_label_representation_is_stable_snake_case() {
            let result = get_thap_than(HeavenlyStem::Giap, HeavenlyStem::Canh);
            let json = serde_json::to_string(&result).expect("serialize");
            assert!(json.contains("\"label\":\"that_sat\""));
            assert!(json.contains("\"relation\":\"target_controls_day\""));
        }
    }
}
