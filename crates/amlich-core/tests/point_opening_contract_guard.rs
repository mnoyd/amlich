//! CI guard for the v1.11 point-opening domain contract (bead
//! `amlich-xlag.2.2.1`): `crates/amlich-core/src/point_opening/`.
//!
//! Freezes the contract side of ADR-0004:
//!
//!   1. the `TY_NGO_LUU_CHU_POLICY_V1` contract is stable, serializable,
//!      and pins the reserved source id plus its separation from the
//!      Tier-0 `shi-er-jing-na-di-zhi` corpus;
//!   2. disclaimer v2 stays byte-identical to the v1.11 REVIEWER-PACK
//!      §A.4 and to the frozen corpus `disclaimer_v2_draft` block
//!      (drift in either direction fails CI);
//!   3. the typed open / explicit-closed states and the
//!      `PointOpeningContext` carrier serde round-trip with every field
//!      preserved;
//!   4. the serialized schema carries no technique, depth, indication,
//!      efficacy, or recommendation field, and no action-recommendation
//!      phrasing (BOUND-02 lexical boundary);
//!   5. every `TNLC-DIV-*` id used by the frozen corpus resolves in the
//!      in-code registry (closed-world divergence vocabulary).

use std::fs;
use std::path::{Path, PathBuf};

use amlich_core::point_opening::{
    policy_contract, tnlc_divergence_by_id, PointOpeningContext, PointOpeningIdentity,
    PointOpeningSlotState, DISCLAIMER_HISTORICAL_PROCEDURAL_CITATION_EN,
    DISCLAIMER_HISTORICAL_PROCEDURAL_CITATION_VN, SAFETY_CLASS_HISTORICAL_PROCEDURAL_CITATION,
    TY_NGO_LUU_CHU_POLICY_ID,
};
use amlich_core::sources;
use amlich_core::traditional_wellness::divergence::ExternalReviewState;

const CORPUS_JSON: &str = include_str!("../data/ty-ngo-luu-chu/najia-open-points.json");

fn reviewer_pack_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(".planning")
        .join("milestones")
        .join("v1.11-phases")
        .join("01-najia-table-freeze")
        .join("REVIEWER-PACK.md")
}

fn row_review_state() -> ExternalReviewState {
    ExternalReviewState::ExternalReviewPending {
        reason: "najia_xu_style_table_row_review_pending".to_string(),
        expected_review_date: "2026-12-31".to_string(),
        assigned_to: "classical_chinese_reviewer".to_string(),
    }
}

fn nomenclature_review_state() -> ExternalReviewState {
    ExternalReviewState::ExternalReviewPending {
        reason: "vietnamese_nomenclature_and_code_gloss_pending".to_string(),
        expected_review_date: "2026-12-31".to_string(),
        assigned_to: "vietnamese_nomenclature_reviewer".to_string(),
    }
}

fn sample_identity() -> PointOpeningIdentity {
    PointOpeningIdentity {
        point_key: "qiao-yin".to_string(),
        xue_ming_zh: "竅陰".to_string(),
        huyet_danh_vi: "Kiếu âm".to_string(),
        standard_code_gloss: "GB44".to_string(),
        channel_zh: "足少陽膽".to_string(),
        channel_vi: "Đởm".to_string(),
        channel_en: "Gallbladder".to_string(),
        role: "primary".to_string(),
    }
}

fn sample_open_context() -> PointOpeningContext {
    PointOpeningContext::new(
        PointOpeningSlotState::Open {
            slot_class_zh_as_printed: "井".to_string(),
            phase_annotation_as_printed: "井金".to_string(),
            points: vec![sample_identity()],
            substitution: Some("qi_na_san_jiao".to_string()),
        },
        row_review_state(),
        nomenclature_review_state(),
        vec![
            "TNLC-DIV-01".to_string(),
            "TNLC-DIV-02".to_string(),
            "TNLC-DIV-03".to_string(),
            "TNLC-DIV-05".to_string(),
        ],
    )
}

fn sample_closed_context() -> PointOpeningContext {
    PointOpeningContext::new(
        PointOpeningSlotState::Closed {
            running_tables: vec!["gui".to_string(), "jia".to_string()],
            doctrine_zh: "得時為之開，失時為之闔（論子午流注法）".to_string(),
            note: "the Xu-style tables as printed leave it without an assigned point (閉穴)"
                .to_string(),
        },
        row_review_state(),
        nomenclature_review_state(),
        vec![
            "TNLC-DIV-01".to_string(),
            "TNLC-DIV-02".to_string(),
            "TNLC-DIV-03".to_string(),
            "TNLC-DIV-05".to_string(),
        ],
    )
}

// ---------------------------------------------------------------------------
// 1. Policy contract stability + separation
// ---------------------------------------------------------------------------

#[test]
fn policy_contract_is_stable_serializable_and_separated() {
    let contract = policy_contract();

    assert_eq!(contract.policy_id, "TY_NGO_LUU_CHU_POLICY_V1");
    assert_eq!(TY_NGO_LUU_CHU_POLICY_ID, contract.policy_id);
    assert_eq!(contract.source_id, sources::SOURCE_TY_NGO_LUU_CHU);
    assert_eq!(contract.safety_class, "historical_procedural_citation");
    assert_eq!(
        contract.safety_class,
        SAFETY_CLASS_HISTORICAL_PROCEDURAL_CITATION
    );

    // Separation from the Tier-0 v1.10 corpus: the two source ids are
    // distinct and the contract pins the never-cross-cite rule.
    assert_eq!(
        contract.never_cross_cites,
        sources::SOURCE_SHI_ER_JING_NA_DI_ZHI
    );
    assert_ne!(contract.source_id, contract.never_cross_cites);

    // ADR-0004 consequences are pinned true.
    assert!(contract.closed_slots_stay_closed);
    assert!(contract.citation_framing_only);
    assert!(contract.disclaimer_required_until_gates_sign);

    // Stable and serializable: round trip preserves every field.
    let json = serde_json::to_string(&contract).expect("contract serializes");
    let recovered = serde_json::from_str::<amlich_core::point_opening::PolicyContract>(&json)
        .expect("contract parses");
    assert_eq!(recovered, contract);
    assert_eq!(
        serde_json::to_string(&recovered).unwrap(),
        json,
        "wire shape must be stable"
    );
}

// ---------------------------------------------------------------------------
// 2. Disclaimer v2 byte locks (REVIEWER-PACK §A.4 + frozen corpus)
// ---------------------------------------------------------------------------

#[test]
fn disclaimer_v2_is_byte_identical_to_reviewer_pack_and_corpus() {
    // REVIEWER-PACK §A.4 is the contract surface — same pattern as the
    // v1.10 prohibited-language guard, extended to the v1.11 pack.
    let pack_path = reviewer_pack_path();
    let pack = fs::read_to_string(&pack_path)
        .unwrap_or_else(|e| panic!("could not read REVIEWER-PACK {}: {e}", pack_path.display()));

    let extract_blockquote = |marker: &str| -> String {
        let (_, after) = pack.split_once(marker).unwrap_or_else(|| {
            panic!("REVIEWER-PACK must contain {marker}");
        });
        let line = after
            .lines()
            .map(str::trim)
            .find(|l| l.starts_with('>'))
            .unwrap_or_else(|| panic!("REVIEWER-PACK {marker} must have a blockquote"));
        line.trim_start_matches("> ").trim().to_string()
    };

    let pack_vn = extract_blockquote("**§A.4.1 Vietnamese:**");
    let pack_en = extract_blockquote("**§A.4.2 English:**");
    assert_eq!(
        DISCLAIMER_HISTORICAL_PROCEDURAL_CITATION_VN, pack_vn,
        "Vietnamese disclaimer v2 must be byte-identical to the v1.11 REVIEWER-PACK §A.4.1"
    );
    assert_eq!(
        DISCLAIMER_HISTORICAL_PROCEDURAL_CITATION_EN, pack_en,
        "English disclaimer v2 must be byte-identical to the v1.11 REVIEWER-PACK §A.4.2"
    );

    // The frozen corpus carries the same draft block — lock both sides.
    let corpus: serde_json::Value = serde_json::from_str(CORPUS_JSON).expect("corpus parses");
    assert_eq!(
        corpus["disclaimer_v2_draft"]["vi"].as_str().unwrap(),
        DISCLAIMER_HISTORICAL_PROCEDURAL_CITATION_VN,
        "disclaimer v2 must match the frozen corpus disclaimer_v2_draft.vi"
    );
    assert_eq!(
        corpus["disclaimer_v2_draft"]["en"].as_str().unwrap(),
        DISCLAIMER_HISTORICAL_PROCEDURAL_CITATION_EN,
        "disclaimer v2 must match the frozen corpus disclaimer_v2_draft.en"
    );
    assert_eq!(
        corpus["metadata"]["disclaimer_id_draft"].as_str().unwrap(),
        "historical_procedural_citation_v1",
        "the stable disclaimer id must match the corpus metadata"
    );
}

// ---------------------------------------------------------------------------
// 3. Typed open / explicit-closed round trips
// ---------------------------------------------------------------------------

#[test]
fn open_and_closed_contexts_round_trip_preserving_every_field() {
    for original in [sample_open_context(), sample_closed_context()] {
        let json = serde_json::to_string(&original).expect("context serializes");
        let recovered: PointOpeningContext = serde_json::from_str(&json).expect("context parses");
        assert_eq!(recovered, original, "round trip must preserve every field");
        assert_eq!(
            serde_json::to_string(&recovered).unwrap(),
            json,
            "wire shape must be stable"
        );
    }
}

#[test]
fn closed_state_is_explicit_and_never_carries_points() {
    let json = serde_json::to_value(sample_closed_context()).unwrap();
    assert_eq!(json["state"]["state"], "closed");
    assert!(
        json["state"].get("points").is_none() && json["state"].get("substitution").is_none(),
        "closed slots serialize the explicit unavailable state, never a point"
    );
    assert!(
        json["state"]["running_tables"].as_array().unwrap().len() == 2,
        "closed state names the running tables"
    );

    let open = serde_json::to_value(sample_open_context()).unwrap();
    assert_eq!(open["state"]["state"], "open");
    assert!(open["state"]["points"].as_array().unwrap().len() >= 1);
    // Exactly one typed state: the closed payload has no open keys and
    // vice versa.
    for open_only in ["points", "substitution", "slot_class_zh_as_printed"] {
        assert!(json["state"].get(open_only).is_none());
    }
    for closed_only in ["running_tables", "doctrine_zh", "note"] {
        assert!(open["state"].get(closed_only).is_none());
    }
}

// ---------------------------------------------------------------------------
// 4. Prohibited-field / action-phrasing schema guard
// ---------------------------------------------------------------------------

/// Recursively collect every object key in a JSON value.
fn collect_keys(value: &serde_json::Value, out: &mut Vec<String>) {
    match value {
        serde_json::Value::Object(map) => {
            for (k, v) in map {
                out.push(k.clone());
                collect_keys(v, out);
            }
        }
        serde_json::Value::Array(items) => {
            for item in items {
                collect_keys(item, out);
            }
        }
        _ => {}
    }
}

#[test]
fn serialized_schema_has_no_clinical_or_action_fields() {
    // BOUND-02: citation framing only. The bead acceptance criterion:
    // "schema guard proves no technique, depth, indication, efficacy,
    // or recommendation fields."
    const FORBIDDEN_KEYS: &[&str] = &[
        "technique",
        "techniques",
        "depth",
        "needle_depth",
        "depth_cun",
        "manipulation",
        "indication",
        "indications",
        "contraindication",
        "contraindications",
        "efficacy",
        "effect",
        "effects",
        "recommendation",
        "recommendations",
        "recommended_point",
        "best_time",
        "best_time_to_treat",
        "point_to_press",
        "treats",
        "cures",
        "heals",
        "diagnosis",
        "prescription",
        "dosage",
        "moxa_protocol",
        "physiological_flow",
        "stimulation",
        "needle_retention",
    ];

    let mut payloads = Vec::new();
    for ctx in [sample_open_context(), sample_closed_context()] {
        payloads.push(serde_json::to_value(&ctx).unwrap());
    }
    payloads.push(serde_json::to_value(policy_contract()).unwrap());

    let mut keys = Vec::new();
    for payload in &payloads {
        collect_keys(payload, &mut keys);
    }
    for key in &keys {
        assert!(
            !FORBIDDEN_KEYS.contains(&key.as_str()),
            "prohibited clinical/action field `{key}` found in the serialized point-opening schema"
        );
    }
}

#[test]
fn serialized_contexts_carry_no_action_recommendation_phrasing() {
    // The open state must read as a citation, never an instruction.
    // (The disclaimer's negation frames — "not instruction or
    // encouragement to needle" — are the only permitted clinical-verb
    // contexts and are byte-locked separately above.)
    const FORBIDDEN_PHRASES: &[&str] = &[
        "best time to treat",
        "best hour to treat",
        "should be needled",
        "should be pressed",
        "should needle",
        "nên châm",
        "nên bấm",
        "nên cứu",
        "hãy châm",
        "hãy bấm",
        "recommended point",
        "điểm nên ",
    ];

    for ctx in [sample_open_context(), sample_closed_context()] {
        let json = serde_json::to_string(&ctx).unwrap();
        for phrase in FORBIDDEN_PHRASES {
            assert!(
                !json.to_lowercase().contains(phrase),
                "action-recommendation phrasing `{phrase}` found in serialized context"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// 5. Divergence registry lockstep with the frozen corpus
// ---------------------------------------------------------------------------

#[test]
fn every_corpus_divergence_id_resolves_in_the_registry() {
    let corpus: serde_json::Value = serde_json::from_str(CORPUS_JSON).expect("corpus parses");

    // Metadata-declared vocabularies.
    for key in ["divergence_ids_in_use", "nomenclature_divergence_ids"] {
        for id in corpus["metadata"][key].as_array().unwrap() {
            let id = id.as_str().unwrap();
            assert!(
                tnlc_divergence_by_id(id).is_some(),
                "metadata {key} id {id} must resolve in the TNLC-DIV registry"
            );
        }
    }

    // Every id actually used by rows, grid cells, and registry entries.
    let mut used: Vec<String> = Vec::new();
    for t in corpus["day_tables"].as_array().unwrap() {
        for r in t["rows"].as_array().unwrap() {
            for id in r["known_divergence_ids"].as_array().unwrap() {
                used.push(id.as_str().unwrap().to_string());
            }
        }
    }
    for cell in corpus["grid"].as_array().unwrap() {
        for id in cell["known_divergence_ids"].as_array().unwrap() {
            used.push(id.as_str().unwrap().to_string());
        }
    }
    for p in corpus["point_nomenclature_registry"].as_array().unwrap() {
        for id in p["known_divergence_ids"].as_array().unwrap() {
            used.push(id.as_str().unwrap().to_string());
        }
    }
    assert!(!used.is_empty(), "corpus must use some divergence ids");
    for id in &used {
        assert!(
            tnlc_divergence_by_id(id).is_some(),
            "corpus divergence id {id} must resolve in the in-code registry"
        );
    }
}

// ---------------------------------------------------------------------------
// Separation from v1.10 Traditional Wellness Context
// ---------------------------------------------------------------------------

#[test]
fn point_opening_contract_stays_separate_from_v1_10_traditional_wellness() {
    use amlich_core::point_opening::disclaimer_id_historical_procedural_citation;
    use amlich_core::traditional_wellness::{
        cultural_information_disclaimer, DISCLAIMER_ID_CULTURAL_INFORMATION_STR,
    };

    // Distinct disclaimer ids and payloads — a client can never render
    // the weaker v1.10 text for point-opening output.
    let v2 = disclaimer_id_historical_procedural_citation();
    assert_ne!(v2.as_str(), DISCLAIMER_ID_CULTURAL_INFORMATION_STR);
    assert_ne!(
        amlich_core::point_opening::historical_procedural_citation_disclaimer(),
        cultural_information_disclaimer()
    );

    // Distinct source ids with the never-cross-cite rule pinned.
    let contract = policy_contract();
    assert_ne!(contract.source_id, sources::SOURCE_SHI_ER_JING_NA_DI_ZHI);

    // The serialized context carries no v1.10-only context fields —
    // the additive DaySnapshot projection (bead amlich-xlag.2.2.6)
    // keeps separate field names for the two contexts.
    let json = serde_json::to_value(sample_open_context()).unwrap();
    for v1_10_key in [
        "branch_channel",
        "branch_channel_associations",
        "seasonal_cultivation",
        "seasonal_profile",
        "traditional_wellness",
    ] {
        let mut keys = Vec::new();
        collect_keys(&json, &mut keys);
        assert!(
            !keys.iter().any(|k| k == v1_10_key),
            "point-opening schema must not embed the v1.10 field `{v1_10_key}`"
        );
    }
}
