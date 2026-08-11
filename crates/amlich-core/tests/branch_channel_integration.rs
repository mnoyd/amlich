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

/// One representative (hour, minute) per branch slot, picked so it falls
/// strictly inside the canonical two-hour window for that slot. Each entry
/// also lists the expected channel identity (vi, en, zh) verbatim from
/// the corpus JSON.
const TWELVE_ROW_GOLDENS: &[(u8, u8, &str, &str, &str, &str)] = &[
    // (hour, minute, branch_vi, channel_vi, channel_en, channel_zh)
    (23, 30, "Tý", "Đởm", "Gallbladder", "足少陽膽"),
    (1, 30, "Sửu", "Can", "Liver", "足厥陰肝"),
    (3, 30, "Dần", "Phế", "Lung", "手太陰肺"),
    (5, 30, "Mão", "Đại trường", "Large Intestine", "手陽明大腸"),
    (7, 30, "Thìn", "Vị", "Stomach", "足陽明胃"),
    (9, 30, "Tỵ", "Tỳ", "Spleen", "足太陰脾"),
    (11, 30, "Ngọ", "Tâm", "Heart", "手少陰心"),
    (
        13,
        30,
        "Mùi",
        "Tiểu trường",
        "Small Intestine",
        "手太陽小腸",
    ),
    (15, 30, "Thân", "Bàng quang", "Bladder", "足太陽膀胱"),
    (17, 30, "Dậu", "Thận", "Kidney", "足少陰腎"),
    (19, 30, "Tuất", "Tâm bào", "Pericardium", "手厥陰心包"),
    (21, 30, "Hợi", "Tam tiêu", "Triple Burner", "手少陽三焦"),
];

#[test]
fn twelve_row_goldens_each_branch_resolves_to_expected_channel() {
    for (hour, minute, branch_vi, channel_vi, channel_en, channel_zh) in TWELVE_ROW_GOLDENS {
        let row = resolve_hour_branch_association(*hour, *minute)
            .unwrap_or_else(|| panic!("lookup at {hour}:{minute:02} must resolve"));
        assert_eq!(
            &row.branch_vi, branch_vi,
            "branch label at {hour}:{minute:02}"
        );
        assert_eq!(
            &row.channel_vi, channel_vi,
            "channel vi at {hour}:{minute:02}"
        );
        assert_eq!(
            &row.channel_en, channel_en,
            "channel en at {hour}:{minute:02}"
        );
        assert_eq!(
            &row.channel_zh, *channel_zh,
            "channel zh at {hour}:{minute:02}"
        );
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
