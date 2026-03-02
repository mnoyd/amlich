use crate::almanac::types::{
    FiveElementRelation, HeavenlyStem, RuleEvidence, ThapThanLabel, ThapThanResult,
};

pub fn get_thap_than(day_can: HeavenlyStem, target_can: HeavenlyStem) -> ThapThanResult {
    let _ = (day_can, target_can);
    ThapThanResult {
        label: ThapThanLabel::TyKien,
        relation: FiveElementRelation::Same,
        same_polarity: true,
        evidence: RuleEvidence {
            source_id: "pending".to_string(),
            method: "pending".to_string(),
            profile: "baseline".to_string(),
        },
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
}
