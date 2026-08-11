//! v1.10 Phase 01-01 — Twelve-Branch Channel Association integration tests.
//!
//! Covers ASSOC-01 / SOURCE-01 / SOURCE-02 / BOUND-01 / BOUND-02 for the
//! branch-channel track. The companion module-level unit tests live in
//! `src/traditional_wellness/{disclaimer,divergence,branch_channel}.rs`.

use amlich_core::almanac::hour_pillar::resolve_hour_branch_slot;
use amlich_core::sources::SOURCE_SHI_ER_JING_NA_DI_ZHI;
use amlich_core::traditional_wellness::{
    load_corpus, resolve_hour_branch_association, resolve_traditional_wellness_context,
    ExternalReviewState, TraditionalWellnessContext,
};

// ---------------------------------------------------------------------------
// 12 table goldens — one assertion per branch
// ---------------------------------------------------------------------------

/// One representative (hour, minute) per branch slot, picked so it
/// falls strictly inside the canonical two-hour window for that slot.
/// Each entry also lists the expected branch/channel identity verbatim
/// from the corpus JSON.
struct RowGolden {
    hour: u8,
    minute: u8,
    branch_vi: &'static str,
    branch_zh: &'static str,
    channel_vi: &'static str,
    channel_en: &'static str,
    channel_zh: &'static str,
    time_range: &'static str,
}

const TWELVE_ROW_GOLDENS: &[RowGolden] = &[
    RowGolden {
        hour: 23,
        minute: 30,
        branch_vi: "Tý",
        branch_zh: "子",
        channel_vi: "Đởm",
        channel_en: "Gallbladder",
        channel_zh: "足少陽膽",
        time_range: "23:00-01:00",
    },
    RowGolden {
        hour: 1,
        minute: 30,
        branch_vi: "Sửu",
        branch_zh: "丑",
        channel_vi: "Can",
        channel_en: "Liver",
        channel_zh: "足厥陰肝",
        time_range: "01:00-03:00",
    },
    RowGolden {
        hour: 3,
        minute: 30,
        branch_vi: "Dần",
        branch_zh: "寅",
        channel_vi: "Phế",
        channel_en: "Lung",
        channel_zh: "手太陰肺",
        time_range: "03:00-05:00",
    },
    RowGolden {
        hour: 5,
        minute: 30,
        branch_vi: "Mão",
        branch_zh: "卯",
        channel_vi: "Đại trường",
        channel_en: "Large Intestine",
        channel_zh: "手陽明大腸",
        time_range: "05:00-07:00",
    },
    RowGolden {
        hour: 7,
        minute: 30,
        branch_vi: "Thìn",
        branch_zh: "辰",
        channel_vi: "Vị",
        channel_en: "Stomach",
        channel_zh: "足陽明胃",
        time_range: "07:00-09:00",
    },
    RowGolden {
        hour: 9,
        minute: 30,
        branch_vi: "Tỵ",
        branch_zh: "巳",
        channel_vi: "Tỳ",
        channel_en: "Spleen",
        channel_zh: "足太陰脾",
        time_range: "09:00-11:00",
    },
    RowGolden {
        hour: 11,
        minute: 30,
        branch_vi: "Ngọ",
        branch_zh: "午",
        channel_vi: "Tâm",
        channel_en: "Heart",
        channel_zh: "手少陰心",
        time_range: "11:00-13:00",
    },
    RowGolden {
        hour: 13,
        minute: 30,
        branch_vi: "Mùi",
        branch_zh: "未",
        channel_vi: "Tiểu trường",
        channel_en: "Small Intestine",
        channel_zh: "手太陽小腸",
        time_range: "13:00-15:00",
    },
    RowGolden {
        hour: 15,
        minute: 30,
        branch_vi: "Thân",
        branch_zh: "申",
        channel_vi: "Bàng quang",
        channel_en: "Bladder",
        channel_zh: "足太陽膀胱",
        time_range: "15:00-17:00",
    },
    RowGolden {
        hour: 17,
        minute: 30,
        branch_vi: "Dậu",
        branch_zh: "酉",
        channel_vi: "Thận",
        channel_en: "Kidney",
        channel_zh: "足少陰腎",
        time_range: "17:00-19:00",
    },
    RowGolden {
        hour: 19,
        minute: 30,
        branch_vi: "Tuất",
        branch_zh: "戌",
        channel_vi: "Tâm bào",
        channel_en: "Pericardium",
        channel_zh: "手厥陰心包",
        time_range: "19:00-21:00",
    },
    RowGolden {
        hour: 21,
        minute: 30,
        branch_vi: "Hợi",
        branch_zh: "亥",
        channel_vi: "Tam tiêu",
        channel_en: "Triple Burner",
        channel_zh: "手少陽三焦",
        time_range: "21:00-23:00",
    },
];

#[test]
fn twelve_row_goldens_each_branch_resolves_to_expected_channel() {
    use amlich_core::traditional_wellness::ExternalReviewState;

    for golden in TWELVE_ROW_GOLDENS {
        let row =
            resolve_hour_branch_association(golden.hour, golden.minute).unwrap_or_else(|| {
                panic!(
                    "lookup at {}:{:02} must resolve",
                    golden.hour, golden.minute
                )
            });

        // Identity (ASSOC-01 contract)
        assert_eq!(
            &row.branch_vi, golden.branch_vi,
            "branch label at {}:{:02}",
            golden.hour, golden.minute
        );
        assert_eq!(
            &row.branch_zh, golden.branch_zh,
            "branch zh at {}:{:02}",
            golden.hour, golden.minute
        );
        assert_eq!(
            &row.time_range, golden.time_range,
            "time_range at {}:{:02}",
            golden.hour, golden.minute
        );
        assert_eq!(
            &row.channel_vi, golden.channel_vi,
            "channel vi at {}:{:02}",
            golden.hour, golden.minute
        );
        assert_eq!(
            &row.channel_en, golden.channel_en,
            "channel en at {}:{:02}",
            golden.hour, golden.minute
        );
        assert_eq!(
            &row.channel_zh, golden.channel_zh,
            "channel zh at {}:{:02}",
            golden.hour, golden.minute
        );

        // Wording uses the neutral historical-association language
        // (LUNAR_HEALTH_RESEARCH.md:134-141).
        assert!(
            row.wording_vi.contains("gắn với"),
            "wording_vi must use 'gắn với' wording at branch_index {}; got {:?}",
            row.branch_index,
            row.wording_vi
        );
        assert!(
            row.wording_en.contains("historically associated"),
            "wording_en must use 'historically associated' wording at branch_index {}; got {:?}",
            row.branch_index,
            row.wording_en
        );

        // Safety classification (BOUND-02 + LUNAR_HEALTH_RESEARCH.md:182).
        assert_eq!(row.safety_class, "historical_cultural_non_clinical");

        // Time-basis disclosure (LUNAR_HEALTH_RESEARCH.md:66).
        assert_eq!(row.time_basis.as_str(), "local_civil_hour_branch");

        // Review state (SOURCE-02): every shipped row is ExternalReviewPending.
        match &row.reviewer {
            ExternalReviewState::ExternalReviewPending {
                reason,
                expected_review_date,
                assigned_to,
            } => {
                assert_eq!(reason, "classical_12_row_table_review_pending");
                assert_eq!(assigned_to, "classical_chinese_reviewer");
                assert_eq!(
                    expected_review_date, "2026-12-31",
                    "expected_review_date must mirror the Active Register date"
                );
            }
            other => panic!(
                "row {} must be ExternalReviewPending at Phase 01-01; got {other:?}",
                row.branch_index
            ),
        }

        // Known divergence IDs (LH-DIV-02/03/06 per LUNAR_HEALTH_RESEARCH.md §6).
        assert!(
            row.known_divergence_ids.iter().any(|id| id == "LH-DIV-02"),
            "row {} must reference LH-DIV-02",
            row.branch_index
        );
        assert!(
            row.known_divergence_ids.iter().any(|id| id == "LH-DIV-03"),
            "row {} must reference LH-DIV-03",
            row.branch_index
        );
        assert!(
            row.known_divergence_ids.iter().any(|id| id == "LH-DIV-06"),
            "row {} must reference LH-DIV-06",
            row.branch_index
        );

        // Source citation uses the canonical source_id (SOURCE-01).
        assert_eq!(row.sources.len(), 1);
        assert_eq!(row.sources[0].source_id, SOURCE_SHI_ER_JING_NA_DI_ZHI);
        assert_eq!(row.sources[0].work_title, "Zhenjiu Daquan");
        assert_eq!(row.sources[0].passage_key, "十二經納地支歌");
        assert_eq!(row.sources[0].translation_kind, "project_paraphrase");
    }
}

// ---------------------------------------------------------------------------
// Four boundary cases (per LUNAR_HEALTH_RESEARCH.md:195 and ASSOC-01)
// ---------------------------------------------------------------------------

#[test]
fn four_boundary_cases_reuse_existing_hour_branch_contract() {
    let cases = [
        // (h, m, expected_branch_vi, expected_channel_vi)
        (22, 59, "Hợi", "Tam tiêu"),
        (23, 0, "Tý", "Đởm"),
        (0, 59, "Tý", "Đởm"),
        (1, 0, "Sửu", "Can"),
    ];

    for (hour, minute, branch_vi, channel_vi) in cases {
        let row = resolve_hour_branch_association(hour, minute)
            .unwrap_or_else(|| panic!("boundary {hour}:{minute:02} must resolve"));
        assert_eq!(row.branch_vi, branch_vi, "branch at {hour}:{minute:02}");
        assert_eq!(row.channel_vi, channel_vi, "channel at {hour}:{minute:02}");
        // The slot index must agree with the existing hour-pillar contract.
        let slot = resolve_hour_branch_slot(hour, minute)
            .expect("resolve_hour_branch_slot must also resolve");
        assert_eq!(
            row.branch_index as usize, slot.branch_index,
            "branch_index must equal hour-pillar slot branch_index at {hour}:{minute:02}"
        );
    }
}

// ---------------------------------------------------------------------------
// Round-trip serialization
// ---------------------------------------------------------------------------

#[test]
fn branch_channel_round_trip_byte_equal() {
    let corpus = load_corpus();
    for row in corpus.iter() {
        let json = serde_json::to_string(row).expect("serialize row");
        let recovered: amlich_core::traditional_wellness::BranchChannelAssociation =
            serde_json::from_str(&json).expect("deserialize row");
        let json2 = serde_json::to_string(&recovered).expect("re-serialize");
        assert_eq!(
            json, json2,
            "round-trip serialization must be byte-equal for branch_index {}",
            row.branch_index
        );
        assert_eq!(recovered, *row);
    }
}

#[test]
fn traditional_wellness_context_round_trip_byte_equal() {
    let ctx = resolve_traditional_wellness_context(23, 30);
    let json = serde_json::to_string(&ctx).expect("serialize context");
    let recovered: TraditionalWellnessContext =
        serde_json::from_str(&json).expect("deserialize context");
    let json2 = serde_json::to_string(&recovered).expect("re-serialize");
    assert_eq!(json, json2);
}

// ---------------------------------------------------------------------------
// Provenance contract
// ---------------------------------------------------------------------------

#[test]
fn provenance_uses_only_shi_er_jing_na_di_zhi() {
    // The reserved-but-never-emitted id from ADR-0003 is enforced by the
    // CI guard `ty_ngo_luu_chu_substring_never_appears_in_production_source`
    // in `tests/source_id_guard.rs`; this test asserts the affirmative
    // half of that contract — every loaded row cites only the registered
    // branch-channel source id.
    let corpus = load_corpus();
    for row in corpus {
        assert!(
            !row.sources.is_empty(),
            "row {} has no sources",
            row.branch_index
        );
        for src in &row.sources {
            assert_eq!(
                src.source_id, SOURCE_SHI_ER_JING_NA_DI_ZHI,
                "row {} source_id must be shi-er-jing-na-di-zhi",
                row.branch_index
            );
        }
        let entries = row.provenance_entries();
        assert_eq!(entries.len(), row.sources.len());
        for entry in &entries {
            assert_eq!(entry.source_id, SOURCE_SHI_ER_JING_NA_DI_ZHI);
        }
    }
}

// ---------------------------------------------------------------------------
// Divergence contract
// ---------------------------------------------------------------------------

#[test]
fn divergence_contract_every_row_carries_lh_div_02_and_resolves() {
    let corpus = load_corpus();
    for row in corpus {
        assert!(
            !row.known_divergence_ids.is_empty(),
            "row {} carries no divergence ids",
            row.branch_index
        );
        assert!(
            row.known_divergence_ids.iter().any(|id| id == "LH-DIV-02"),
            "row {} must carry LH-DIV-02",
            row.branch_index
        );
        for id in &row.known_divergence_ids {
            let d = amlich_core::traditional_wellness::divergence_by_id(id).unwrap_or_else(|| {
                panic!(
                    "row {} references unregistered divergence id {id}",
                    row.branch_index
                )
            });
            assert_eq!(d.id, *id);
        }
    }
}

// ---------------------------------------------------------------------------
// Tier-0 availability — succeeds without birth / medical data
// ---------------------------------------------------------------------------

#[test]
fn tier0_succeeds_without_birth_or_medical_data() {
    // The lookup takes only (local_hour, local_minute); no birth_chi_index,
    // no sex/gender, no symptom, no location, no health history.
    let ctx = resolve_traditional_wellness_context(7, 15);
    let hb = ctx.hour_branch.expect("must resolve");
    assert_eq!(hb.branch_index, 4);
    assert_eq!(hb.branch_vi, "Thìn");
    // No BirthInput-shaped fields are populated — this is BOUND-01.
    // The compile-time signature itself enforces this; the runtime
    // assertion is the absence of any panic on the call.
}

// ---------------------------------------------------------------------------
// Lockstep with the existing hour-pillar contract
// ---------------------------------------------------------------------------

#[test]
fn branch_channel_slot_matches_existing_hour_pillar_contract() {
    // Sweep all (hour, minute) combinations inside the valid range
    // and confirm `resolve_hour_branch_association` returns the same
    // branch_index as `resolve_hour_branch_slot`. This is the lockstep
    // contract that guarantees we never redefine the boundary.
    for hour in 0u8..=23 {
        for minute in (0u8..=59).step_by(7) {
            let assoc = resolve_hour_branch_association(hour, minute)
                .unwrap_or_else(|| panic!("lookup at {hour}:{minute:02} must resolve"));
            let slot = resolve_hour_branch_slot(hour, minute)
                .unwrap_or_else(|| panic!("slot at {hour}:{minute:02} must resolve"));
            assert_eq!(
                assoc.branch_index as usize, slot.branch_index,
                "branch_index mismatch at {hour}:{minute:02}"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Review state contract
// ---------------------------------------------------------------------------

#[test]
fn review_state_pending_marker_round_trips_through_corpus_loader() {
    // Every row ships ExternalReviewPending until §B of REVIEWER-PACK.md
    // is signed. The corpus loader must surface this faithfully — the
    // marker round-trips byte-for-byte through serde.
    let corpus = load_corpus();
    for row in corpus {
        assert!(
            !row.reviewer.is_signed(),
            "row {} must be Pending",
            row.branch_index
        );
        match &row.reviewer {
            ExternalReviewState::ExternalReviewPending {
                reason,
                expected_review_date,
                assigned_to,
            } => {
                assert_eq!(reason, "classical_12_row_table_review_pending");
                assert_eq!(assigned_to, "classical_chinese_reviewer");
                assert!(!expected_review_date.is_empty());
            }
            other => panic!(
                "row {} must be ExternalReviewPending; got {other:?}",
                row.branch_index
            ),
        }
    }
}

// ---------------------------------------------------------------------------
// Disclaimer / time-basis contract
// ---------------------------------------------------------------------------

#[test]
fn context_carries_stable_disclaimer_id_and_time_basis() {
    let ctx = resolve_traditional_wellness_context(2, 0);
    assert_eq!(ctx.disclaimer.id.as_str(), "cultural_information_v1");
    assert!(!ctx.disclaimer.vi.is_empty());
    assert!(!ctx.disclaimer.en.is_empty());
    assert_eq!(ctx.time_basis.as_str(), "local_civil_hour_branch");
}
