//! v1.11 Point-Opening Context — the stable domain contract types
//! (bead `amlich-xlag.2.2.1`).
//!
//! The three types here are the *contract* the later engine beads fill
//! in: the corpus loader (`amlich-xlag.2.2.2`) validates frozen rows
//! into [`PointOpeningIdentity`] triples and closed evidence, the
//! resolver (`amlich-xlag.2.2.3` / `.2.2.4`) picks exactly one
//! [`PointOpeningSlotState`] per (day stem, hour branch) cell, and the
//! provenance bead (`amlich-xlag.2.2.5`) attaches per-row source
//! citations. This module defines only the typed shapes — no loading,
//! no resolution, no calendar math.
//!
//! Reuses the v1.10 `ExternalReviewState` / `TimeBasis` /
//! `LocalizedDisclaimer` *primitive building blocks* so the wire
//! vocabulary stays uniform; it never embeds a v1.10 Traditional
//! Wellness Context value (ADR-0004: separate contexts, separate
//! DaySnapshot fields, no cross-citation).

use serde::{Deserialize, Serialize};

use crate::traditional_wellness::disclaimer::LocalizedDisclaimer;
use crate::traditional_wellness::divergence::{ExternalReviewState, TimeBasis};

use super::disclaimer::historical_procedural_citation_disclaimer;
use super::divergence::tnlc_divergence_by_id;
use super::policy::{SAFETY_CLASS_HISTORICAL_PROCEDURAL_CITATION, TY_NGO_LUU_CHU_POLICY_ID};

/// The nomenclature triple every open point carries (NAME-01): the
/// classical Chinese 穴名 exactly as printed, the Vietnamese huyệt
/// danh, and a standard alphanumeric code as lookup gloss. The
/// Vietnamese and code legs stay review-pending until Gate 2 signs —
/// the pending state travels on the context
/// ([`PointOpeningContext::nomenclature_review_state`]), not on the
/// strings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PointOpeningIdentity {
    /// Registry key of the point (e.g. `qiao-yin`).
    pub point_key: String,
    /// Classical Chinese 穴名 exactly as printed (e.g. 竅陰).
    pub xue_ming_zh: String,
    /// Vietnamese huyệt danh (Gate-2-pending draft).
    pub huyet_danh_vi: String,
    /// Standard alphanumeric code as lookup gloss (Gate-2-pending
    /// draft; never an efficacy endorsement — TNLC-DIV-04).
    pub standard_code_gloss: String,
    /// Owning channel, classical Chinese (e.g. 足少陽膽).
    pub channel_zh: String,
    /// Owning channel, Vietnamese (e.g. Đởm).
    pub channel_vi: String,
    /// Owning channel, English gloss (e.g. Gallbladder).
    pub channel_en: String,
    /// `primary`, `yuan_guo` (並過原), `jian_guo_san_jiao_yuan`
    /// (兼過三焦原), or `you_guo_bao_luo_yuan` (又過包絡原).
    pub role: String,
}

/// The resolved state of one (day stem × hour branch) slot: exactly one
/// typed open state or the explicit closed (閉穴) state — never both,
/// never neither (TNLC-DIV-01: closed slots are never filled by
/// later-school rules).
///
/// Serializes internally tagged on `"state"` (`"open"` / `"closed"`),
/// mirroring the frozen corpus grid-cell convention.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum PointOpeningSlotState {
    /// The slot resolves to exactly one frozen table row: its five-shu
    /// class as printed and the complete point identity triple(s).
    Open {
        /// Five-shu/original class as printed (井/榮/俞/經/合 or
        /// 原/納 rows' annotation).
        slot_class_zh_as_printed: String,
        /// Phase annotation as printed (e.g. 井金).
        phase_annotation_as_printed: String,
        /// Point identity triples; the first entry is the primary
        /// point, later entries the 並過/兼過/又過 companions.
        points: Vec<PointOpeningIdentity>,
        /// Substitution marker (返本還原 / 氣納三焦 / 血納包絡 …) as
        /// frozen, if the row carries one.
        substitution: Option<String>,
    },
    /// The Xu-style tables as printed leave this slot without an
    /// assigned point (閉穴). Never filled, never converted to a
    /// recommendation (TNLC-DIV-01).
    Closed {
        /// The day-tables whose running windows cover the slot (e.g.
        /// `["gui", "jia"]`).
        running_tables: Vec<String>,
        /// The classical open/closed doctrine lines as cited by the
        /// frozen corpus (「得時為之開，失時為之闔」…).
        doctrine_zh: String,
        /// Explicit unavailable-by-tradition note.
        note: String,
    },
}

/// The Point-Opening Context carrier: one typed open or explicit closed
/// result plus the policy, safety, disclaimer, review-state, time-basis,
/// and divergence disclosures that must travel with it (ADR-0004).
///
/// Separation from v1.10: this type never embeds a
/// `TraditionalWellnessContext`; it is attached to `DaySnapshot` as its
/// own additive field (bead `amlich-xlag.2.2.6`) and never feeds Day
/// Assessment, Hour Ranking, or Direction Assessment.
///
/// Round-trip serde discipline: every field serializes — `Option`-free
/// by construction (the only `Option` lives inside the `Open` variant
/// where `null` is the frozen-row truth) so the wire shape is stable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointOpeningContext {
    /// Policy contract id (`TY_NGO_LUU_CHU_POLICY_V1`).
    pub policy_id: String,
    /// The resolved slot state — exactly one open or explicit closed.
    pub state: PointOpeningSlotState,
    /// Bilingual disclaimer v2; byte-locked to the REVIEWER-PACK §A.4.
    pub disclaimer: LocalizedDisclaimer,
    /// Row review state (Gate 1 owns signing).
    pub review_state: ExternalReviewState,
    /// Nomenclature review state (Gate 2 owns signing the Vietnamese
    /// huyệt danh and code-gloss legs of every identity).
    pub nomenclature_review_state: ExternalReviewState,
    /// `historical_procedural_citation` (BOUND-02).
    pub safety_class: String,
    /// Disclosed time basis (TNLC-DIV-03).
    pub time_basis: TimeBasis,
    /// Applicable `TNLC-DIV-*` divergence references; every id must
    /// resolve in the [`super::divergence`] registry.
    pub known_divergence_ids: Vec<String>,
}

impl PointOpeningContext {
    /// Wrap a resolved slot state in the v1.11 contract carrier with
    /// the pinned policy id, canonical safety class, disclaimer v2, and
    /// disclosed time basis. Every divergence id must resolve in the
    /// `TNLC-DIV-*` registry — the closed-world contract the corpus
    /// loader and CI guards also assert.
    pub fn new(
        state: PointOpeningSlotState,
        review_state: ExternalReviewState,
        nomenclature_review_state: ExternalReviewState,
        known_divergence_ids: Vec<String>,
    ) -> Self {
        for id in &known_divergence_ids {
            assert!(
                tnlc_divergence_by_id(id).is_some(),
                "unknown point-opening divergence id {id:?} — must be a registered TNLC-DIV-* id"
            );
        }
        Self {
            policy_id: TY_NGO_LUU_CHU_POLICY_ID.to_string(),
            state,
            disclaimer: historical_procedural_citation_disclaimer(),
            review_state,
            nomenclature_review_state,
            safety_class: SAFETY_CLASS_HISTORICAL_PROCEDURAL_CITATION.to_string(),
            time_basis: TimeBasis::LocalCivilHourBranch,
            known_divergence_ids,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn sample_review_state() -> ExternalReviewState {
        ExternalReviewState::ExternalReviewPending {
            reason: "najia_xu_style_table_row_review_pending".to_string(),
            expected_review_date: "2026-12-31".to_string(),
            assigned_to: "classical_chinese_reviewer".to_string(),
        }
    }

    fn sample_open_state() -> PointOpeningSlotState {
        PointOpeningSlotState::Open {
            slot_class_zh_as_printed: "井".to_string(),
            phase_annotation_as_printed: "井金".to_string(),
            points: vec![sample_identity()],
            substitution: None,
        }
    }

    fn sample_closed_state() -> PointOpeningSlotState {
        PointOpeningSlotState::Closed {
            running_tables: vec!["gui".to_string(), "jia".to_string()],
            doctrine_zh: "得時為之開，失時為之闔".to_string(),
            note: "the Xu-style tables as printed leave it without an assigned point (閉穴)"
                .to_string(),
        }
    }

    fn sample_divergences() -> Vec<String> {
        vec![
            "TNLC-DIV-01".to_string(),
            "TNLC-DIV-02".to_string(),
            "TNLC-DIV-03".to_string(),
            "TNLC-DIV-05".to_string(),
        ]
    }

    #[test]
    fn identity_round_trips_preserving_every_field() {
        let original = sample_identity();
        let json = serde_json::to_string(&original).unwrap();
        let recovered: PointOpeningIdentity = serde_json::from_str(&json).unwrap();
        assert_eq!(recovered, original);
    }

    #[test]
    fn open_state_serializes_internally_tagged_on_state() {
        let json = serde_json::to_value(sample_open_state()).unwrap();
        assert_eq!(json["state"], "open");
        assert!(json["points"].is_array());
        assert_eq!(json["substitution"], serde_json::Value::Null);
        assert!(json.get("running_tables").is_none());
    }

    #[test]
    fn closed_state_serializes_explicit_and_never_carries_points() {
        let json = serde_json::to_value(sample_closed_state()).unwrap();
        assert_eq!(json["state"], "closed");
        assert!(json["running_tables"].is_array());
        assert!(json["doctrine_zh"].as_str().unwrap().contains("失時為之闔"));
        assert!(
            json.get("points").is_none() && json.get("substitution").is_none(),
            "closed slots never carry points or substitution markers"
        );
    }

    #[test]
    fn slot_state_round_trips_both_variants() {
        for state in [sample_open_state(), sample_closed_state()] {
            let json = serde_json::to_string(&state).unwrap();
            let recovered: PointOpeningSlotState = serde_json::from_str(&json).unwrap();
            assert_eq!(recovered, state);
            let json2 = serde_json::to_string(&recovered).unwrap();
            assert_eq!(json, json2);
        }
    }

    #[test]
    fn unknown_state_tag_is_rejected() {
        let bad = r#"{"state":"maybe","points":[]}"#;
        assert!(serde_json::from_str::<PointOpeningSlotState>(bad).is_err());
    }

    #[test]
    fn constructor_pins_the_contract_fields() {
        let ctx = PointOpeningContext::new(
            sample_open_state(),
            sample_review_state(),
            ExternalReviewState::ExternalReviewPending {
                reason: "vietnamese_nomenclature_and_code_gloss_pending".to_string(),
                expected_review_date: "2026-12-31".to_string(),
                assigned_to: "vietnamese_nomenclature_reviewer".to_string(),
            },
            sample_divergences(),
        );
        assert_eq!(ctx.policy_id, TY_NGO_LUU_CHU_POLICY_ID);
        assert_eq!(
            ctx.safety_class,
            SAFETY_CLASS_HISTORICAL_PROCEDURAL_CITATION
        );
        assert_eq!(
            ctx.disclaimer.id.as_str(),
            super::super::disclaimer::DISCLAIMER_ID_HISTORICAL_PROCEDURAL_CITATION_STR
        );
        assert_eq!(ctx.time_basis, TimeBasis::LocalCivilHourBranch);
        assert_eq!(ctx.known_divergence_ids, sample_divergences());
    }

    #[test]
    fn constructor_rejects_unregistered_divergence_ids() {
        let result = std::panic::catch_unwind(|| {
            PointOpeningContext::new(
                sample_closed_state(),
                sample_review_state(),
                sample_review_state(),
                vec!["TNLC-DIV-99".to_string()],
            )
        });
        assert!(
            result.is_err(),
            "unknown TNLC-DIV ids must be rejected at construction"
        );
    }

    #[test]
    fn context_round_trips_open_and_closed_preserving_every_field() {
        let nomenclature = ExternalReviewState::ExternalReviewPending {
            reason: "vietnamese_nomenclature_and_code_gloss_pending".to_string(),
            expected_review_date: "2026-12-31".to_string(),
            assigned_to: "vietnamese_nomenclature_reviewer".to_string(),
        };
        for state in [sample_open_state(), sample_closed_state()] {
            let original = PointOpeningContext::new(
                state,
                sample_review_state(),
                nomenclature.clone(),
                sample_divergences(),
            );
            let json = serde_json::to_string(&original).unwrap();
            let recovered: PointOpeningContext = serde_json::from_str(&json).unwrap();
            assert_eq!(recovered, original);
            let json2 = serde_json::to_string(&recovered).unwrap();
            assert_eq!(json, json2, "wire shape must be stable");
        }
    }
}
