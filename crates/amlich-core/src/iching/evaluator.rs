//! v1.7 IChing (Kinh Dịch) Tier-0 evaluator + slim DTO for DaySnapshot.
//!
//! Phase 24-01 (ICH-05 + partial INT-12). Composes the already-shipped
//! Phase 22 [`cast_mai_hoa`], [`derive_bien_que`], [`classify_the_dung`]
//! over the Phase 21 [`get_hexagram`] corpus lookup. Emits a compound
//! [`IChingEvaluation`] result with per-step [`ReasoningEvidenceEnvelope`]
//! provenance (distinct [`crate::sources::SOURCE_MAI_HOA_DICH_SO`] +
//! [`crate::sources::SOURCE_KINH_DICH`] primitive sources plus one
//! [`COMPOSITE_ICHING_CONSULTATION`] composite envelope), and projects it to
//! a slim owned [`IChingCastSummary`] DTO for the additive `DaySnapshot`
//! integration.
//!
//! CRIT-3 isolation: this module defines NO `impl From<...>` between the
//! three iching newtypes (`TienThienTrigram` / `HauThienTrigram` /
//! `KingWenHexagram`). The composition table + [`crate::iching::compose`] are
//! the only bridges.
//!
//! CRIT-6 source-id discipline: every evidence envelope constructed here uses
//! the [`crate::sources::SOURCE_MAI_HOA_DICH_SO`] /
//! [`crate::sources::SOURCE_KINH_DICH`] consts and the
//! [`COMPOSITE_ICHING_CONSULTATION`] named const — never bare literals.
//!
//! MOD-7 / Tier-0: the evaluator works with NO birth data. The
//! [`ActionEvaluator::evaluate`] adapter ignores the `personal_input`
//! parameter; the rich [`IChingEvaluator::evaluate_consultation`] path
//! reads only the lunar inputs from the [`IChingQuery`].
//!
//! WASM-safety: no filesystem access, no wall-clock, no RNG in this file.
//! Verified by an inline runtime-built-needle grep guard (mirrors the v1.6 /
//! v1.7 discipline codified across `corpus.rs`, `mai_hoa.rs`, `bien_que.rs`,
//! and `the_dung.rs`).

use serde::{Deserialize, Serialize};

use crate::iching::bien_que::BienQue;
use crate::iching::mai_hoa::MaiHoaCast;
use crate::iching::schema::KingWenHexagram;
use crate::iching::the_dung::{CatHung, TheDungClassification};
use crate::iching::{classify_the_dung, cast_mai_hoa, derive_bien_que, get_hexagram};
use crate::reasoning::{
    ActionEvaluation, ActionEvaluator, PersonalReasoningInput, ReasoningEvidenceEnvelope,
};
use crate::ActionId;
use crate::semantic_graph::SemanticGraph;
use crate::sources::{SOURCE_KINH_DICH, SOURCE_MAI_HOA_DICH_SO};
use crate::DaySnapshot;
use unicode_normalization::{is_nfc, UnicodeNormalization};

// ===========================================================================
// Composite rule identifier (named const, single audit point for CRIT-6)
// ===========================================================================

/// The composite source_id for a complete I Ching consultation — present in
/// exactly one envelope of every evaluation's `evidence` vector. Mirrors
/// `COMPOSITE_DIRECTION_CROSS_LINK`'s discipline (Phase 23-02). All
/// production call-sites use this constant, never the bare literal.
pub const COMPOSITE_ICHING_CONSULTATION: &str = "rule.composite.iching_consultation";

// ===========================================================================
// IChingQuery — sibling newtype (NOT a ConsultationIntent::IChing variant)
// ===========================================================================

/// Sibling I Ching consultation query (Phase 24-01). Constructed explicitly
/// from lunar inputs OR via [`IChingQuery::from_snapshot`]; carried by
/// [`IChingEvaluator`] on the Tier-0 path. Deliberately NOT a
/// `ConsultationIntent::IChing` variant (sibling-newtype decision per the
/// v1.7 plan — adding a variant would force ~25-43 call-site `Copy`-break
/// churn across the codebase).
///
/// The `question_vi` field is NFC-normalised and whitespace-stripped at
/// construction. `chi_hour_index` is validated to lie in `0..=11`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IChingQuery {
    /// Earthly Branch of the cast hour (Tý=0, ..., Hợi=11). Validated.
    pub chi_hour_index: u8,
    /// Optional free-form Vietnamese question. NFC-normalised; whitespace-only
    /// is normalised to `None`.
    pub question_vi: Option<String>,
    /// Lunar year branch (0..=11).
    pub lunar_year_branch: u8,
    /// Lunar month (1..=12).
    pub lunar_month: u8,
    /// Lunar day (1..=30).
    pub lunar_day: u8,
}

impl IChingQuery {
    /// Build a query from a [`DaySnapshot`] + user-supplied `chi_hour_index`
    /// + optional `question_vi`. Canonical public surface for Tier-0
    /// consultation.
    ///
    /// Validation:
    /// - `chi_hour_index` must be `0..=11` (returns `Err` otherwise).
    /// - `question_vi` is NFC-normalised.
    /// - Whitespace-only `question_vi` is normalised to `None`.
    ///
    /// The lunar fields are derived from the snapshot's
    /// `context.canchi.year.chi_index` + `context.lunar.{month,day}` —
    /// the caller does not need to know the lunar mapping.
    pub fn from_snapshot(
        snapshot: &DaySnapshot,
        question_vi: Option<String>,
        chi_hour_index: u8,
    ) -> Result<Self, String> {
        let _ = (snapshot, question_vi, chi_hour_index);
        unimplemented!("RED phase: IChingQuery::from_snapshot")
    }

    /// Direct constructor for golden tests / boundary checks. Validates all
    /// four input ranges:
    /// - `lunar_year_branch` ∈ 0..=11
    /// - `lunar_month` ∈ 1..=12
    /// - `lunar_day` ∈ 1..=30
    /// - `chi_hour_index` ∈ 0..=11
    /// Returns `Err` on any out-of-range value.
    pub fn from_lunar_inputs(
        lunar_year_branch: u8,
        lunar_month: u8,
        lunar_day: u8,
        chi_hour_index: u8,
        question_vi: Option<String>,
    ) -> Result<Self, String> {
        let _ = (
            lunar_year_branch,
            lunar_month,
            lunar_day,
            chi_hour_index,
            question_vi,
        );
        unimplemented!("RED phase: IChingQuery::from_lunar_inputs")
    }
}

// ===========================================================================
// HexagramEntryProjection — owned projection (no &'static corpus refs)
// ===========================================================================

/// Owned DTO projection of a corpus [`crate::iching::HexagramEntry`] (no
/// `&'static HexagramEntry` — the corpus is `OnceLock`-cached; attaching it
/// to a DaySnapshot would tie the snapshot's lifetime to the corpus cache).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HexagramEntryProjection {
    pub king_wen_index: KingWenHexagram,
    pub vi_name: String,
    pub thoai_tu: String,
    pub hao_tu: Vec<String>,
    pub cat_hung: String,
}

// ===========================================================================
// IChingEvaluation — compound rich result
// ===========================================================================

/// The compound rich result of an I Ching consultation. Carries every
/// intermediate so callers can introspect the full derivation. Distinct from
/// the slim [`IChingCastSummary`] DTO which is what rides on DaySnapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IChingEvaluation {
    pub query: IChingQuery,
    pub cast: MaiHoaCast,
    pub bien_que: BienQue,
    pub the_dung: TheDungClassification,
    pub chu_hexagram: HexagramEntryProjection,
    pub bien_hexagram: HexagramEntryProjection,
    pub evidence: Vec<ReasoningEvidenceEnvelope>,
}

// ===========================================================================
// IChingCastSummary — slim owned DTO for DaySnapshot.iching_cast
// ===========================================================================

/// Slim owned DTO summarising one I Ching consultation. The
/// `DaySnapshot.iching_cast: Option<IChingCastSummary>` field carries this
/// value; absent from JSON when None (additive-`Option<T>` + serde
/// `skip_serializing_if` discipline). Owned strings — no `&'static` corpus
/// references — so the snapshot stays self-contained.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IChingCastSummary {
    pub cast: MaiHoaCast,
    pub bien_que: BienQue,
    pub the_dung: TheDungClassification,
    pub chu_hexagram_vi_name: String,
    pub chu_hexagram_thoai_tu: String,
    pub bien_hexagram_vi_name: String,
    pub bien_hexagram_thoai_tu: String,
    /// Stable string projection of the TheDung verdict (`"cat"` /
    /// `"binh"` / `"hung"`).
    pub cat_hung_summary: String,
    /// Echo of the cast's động hào (1..=6).
    pub moving_line: u8,
    pub question_vi: Option<String>,
    pub evidence: Vec<ReasoningEvidenceEnvelope>,
}

// ===========================================================================
// IChingEvaluator — Tier-0 evaluator (no birth data)
// ===========================================================================

/// Tier-0 I Ching evaluator. Pure projection over the Phase 22
/// cast/biến-quẻ/thể-dụng surface + the Phase 21 corpus lookup. No Bazi,
/// no birth data, no RNG, no wall-clock, no filesystem.
///
/// Implements [`ActionEvaluator`] as a thin trait-shape adapter that returns
/// an empty [`ActionEvaluation::empty`] for `ActionId::IChing` — the rich
/// I Ching result lives behind [`IChingEvaluator::evaluate_consultation`],
/// NOT collapsed into the generic trait shape (per 24-CONTEXT.md Claude's
/// Discretion §1).
pub struct IChingEvaluator {
    query: IChingQuery,
}

impl IChingEvaluator {
    /// Construct from an [`IChingQuery`].
    pub fn new(query: IChingQuery) -> Self {
        Self { query }
    }

    /// Run the full I Ching consultation and return the rich
    /// [`IChingEvaluation`] result. Composes:
    ///
    /// 1. [`cast_mai_hoa`] over the query's lunar inputs.
    /// 2. [`derive_bien_que`] on the cast.
    /// 3. [`classify_the_dung`] on the cast.
    /// 4. [`get_hexagram`] for both chủ quẻ + biến quẻ.
    /// 5. Per-step [`ReasoningEvidenceEnvelope`] construction.
    pub fn evaluate_consultation(
        &self,
        _snapshot: &DaySnapshot,
    ) -> Result<IChingEvaluation, String> {
        unimplemented!("RED phase: IChingEvaluator::evaluate_consultation")
    }

    /// Project an [`IChingEvaluation`] into the slim owned
    /// [`IChingCastSummary`] DTO. Owned strings throughout — no `&'static`
    /// corpus references.
    pub fn to_summary(&self, _evaluation: &IChingEvaluation) -> IChingCastSummary {
        unimplemented!("RED phase: IChingEvaluator::to_summary")
    }

    /// Convenience: evaluate + project to summary. The `snapshot` is unused
    /// on the Tier-0 path; carried in the signature for the future
    /// personal-context-aware path.
    pub fn evaluate(&self, _snapshot: &DaySnapshot) -> Result<IChingCastSummary, String> {
        unimplemented!("RED phase: IChingEvaluator::evaluate")
    }
}

impl ActionEvaluator for IChingEvaluator {
    fn action_id(&self) -> ActionId {
        ActionId::IChing
    }

    fn select_subgraph(
        &self,
        graph: &SemanticGraph,
        _snapshot: &DaySnapshot,
        _personal_input: Option<&PersonalReasoningInput>,
    ) -> Result<SemanticGraph, String> {
        // IChing facts are added by DaySnapshotGraphBuilder and consumed
        // wholesale by the evaluator — no subgraph filtering. Mirrors
        // InitiationOpeningEvaluator::select_subgraph.
        Ok(graph.clone())
    }

    fn evaluate(
        &self,
        _graph: &SemanticGraph,
        _snapshot: &DaySnapshot,
        _personal_input: Option<&PersonalReasoningInput>,
    ) -> Result<ActionEvaluation, String> {
        // Trait-shape adapter: returns an empty ActionEvaluation. The rich
        // IChingEvaluation lives behind evaluate_consultation, NOT collapsed
        // into the generic ActionEvaluation shape.
        Ok(ActionEvaluation::empty(ActionId::IChing))
    }
}

// ===========================================================================
// nfc() helper — module-private, mirrors corpus.rs:163-169 + rituals/corpus.rs
// ===========================================================================

/// RIT-08 NFC normalisation helper. Module-private mirror of
/// `rituals::corpus::nfc()` and `iching::corpus::nfc()` — keeps this
/// module independent of the corpus loader while staying byte-identical to
/// the proven shape.
fn nfc(s: &str) -> String {
    if is_nfc(s) {
        s.to_string()
    } else {
        s.nfc().collect()
    }
}

/// Stable string projection of a [`CatHung`] verdict (used in
/// `IChingCastSummary.cat_hung_summary`).
pub(crate) fn cat_hung_str(v: CatHung) -> &'static str {
    match v {
        CatHung::Cat => "cat",
        CatHung::Binh => "binh",
        CatHung::Hung => "hung",
    }
}

// ===========================================================================
// Inline tests — RED phase must FAIL on every `unimplemented!("RED phase: ...")`
// call site; GREEN phase makes them PASS via real implementations.
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::iching::schema::{HauThienTrigram, KingWenHexagram, TienThienTrigram};
    use crate::reasoning::ReasoningEvidenceEnvelope;
    use crate::ReasoningEvidenceSourceFamily;
    use crate::semantic_graph::SemanticGraph;
    use crate::sources::{SOURCE_KINH_DICH, SOURCE_MAI_HOA_DICH_SO};
    use crate::DaySnapshot;
    use crate::advisory::{BirthInput, ConsultationIntent};
    use crate::almanac::tu_menh::Gender;
    use unicode_normalization::is_nfc;
    use crate::VIETNAM_TIMEZONE;

    /// Convenience: a populated snapshot for testing.
    fn sample_snapshot() -> DaySnapshot {
        crate::calculate_day_snapshot(10, 2, 2024)
    }

    /// ──────────────────────────────────────────────────────────────────
    /// IChingQuery construction tests (RED: from_snapshot /
    /// from_lunar_inputs panic with unimplemented!)
    /// ──────────────────────────────────────────────────────────────────

    #[test]
    fn iching_query_from_snapshot_derives_lunar_inputs() {
        let snap = sample_snapshot();
        let query = IChingQuery::from_snapshot(
            &snap,
            Some("việc làm".to_string()),
            9,
        )
        .expect("valid inputs");
        assert_eq!(query.chi_hour_index, 9);
        assert_eq!(query.question_vi.as_deref(), Some("việc làm"));
        // lunar_year_branch derives from snapshot's year chi_index.
        assert_eq!(
            query.lunar_year_branch as usize,
            snap.context.canchi.year.chi_index
        );
    }

    #[test]
    fn iching_query_rejects_invalid_hour_index() {
        let snap = sample_snapshot();
        let err = IChingQuery::from_snapshot(&snap, None, 12).expect_err("hour 12 must fail");
        assert!(
            err.contains("chi_hour_index"),
            "error must mention chi_hour_index; got: {err}"
        );
        let err_high = IChingQuery::from_snapshot(&snap, None, 255).expect_err("hour 255 must fail");
        assert!(err_high.contains("chi_hour_index"));
    }

    #[test]
    fn iching_query_nfc_normalises_question() {
        let snap = sample_snapshot();
        // Combining diacritics form: "vi\u{0300}e\u{0302}" should NFC-normalise
        // to "viê".
        let decomposed = String::from("vi\u{0300}e\u{0302} công vi\u{00ea}\u{0302}c");
        let query = IChingQuery::from_snapshot(&snap, Some(decomposed), 5)
            .expect("NFD input should be normalised, not rejected");
        let stored = query.question_vi.expect("non-whitespace input persists");
        // The exact form after NFC: combining marks recomposed into precomposed chars.
        assert!(is_nfc(&stored), "stored question_vi must be NFC; got decomposed form");
        // "vi\u{0300}e\u{0302}" → "viê" (U+1EC3).
        assert!(
            stored.contains("viê công viếc"),
            "NFC must recompose the combining marks; got: {stored}"
        );
    }

    #[test]
    fn iching_query_rejects_whitespace_only_question() {
        let snap = sample_snapshot();
        let query =
            IChingQuery::from_snapshot(&snap, Some("   \t  ".to_string()), 5).expect("ok");
        assert!(
            query.question_vi.is_none(),
            "whitespace-only question_vi must normalise to None; got: {:?}",
            query.question_vi
        );
    }

    #[test]
    fn iching_query_from_lunar_inputs_validates_all_ranges() {
        let q = IChingQuery::from_lunar_inputs(0, 1, 1, 0, None).expect("all minimal OK");
        assert_eq!(q.lunar_year_branch, 0);
        assert_eq!(q.lunar_month, 1);
        assert_eq!(q.lunar_day, 1);
        assert_eq!(q.chi_hour_index, 0);

        // Out-of-range branches.
        assert!(IChingQuery::from_lunar_inputs(12, 1, 1, 0, None).is_err());
        // Out-of-range month.
        assert!(IChingQuery::from_lunar_inputs(0, 0, 1, 0, None).is_err());
        assert!(IChingQuery::from_lunar_inputs(0, 13, 1, 0, None).is_err());
        // Out-of-range day.
        assert!(IChingQuery::from_lunar_inputs(0, 1, 0, 0, None).is_err());
        assert!(IChingQuery::from_lunar_inputs(0, 1, 31, 0, None).is_err());
        // Out-of-range hour.
        assert!(IChingQuery::from_lunar_inputs(0, 1, 1, 12, None).is_err());
    }

    /// ──────────────────────────────────────────────────────────────────
    /// IChingEvaluator rich-path tests (RED: evaluate_consultation panics)
    /// ──────────────────────────────────────────────────────────────────

    #[test]
    fn iching_evaluator_emits_at_least_two_primitive_source_ids_plus_one_composite() {
        let snap = sample_snapshot();
        let query = IChingQuery::from_snapshot(&snap, None, 9).expect("query");
        let evaluator = IChingEvaluator::new(query);
        let eval = evaluator
            .evaluate_consultation(&snap)
            .expect("evaluation succeeds");

        // CRIT-6: at least one primitive envelope with source_id = SOURCE_MAI_HOA_DICH_SO
        // AND at least one with source_id = SOURCE_KINH_DICH. We check
        // membership via Vec (the list is small — 4 envelopes — so a HashSet
        // is not necessary and avoids dragging a Hash derive into
        // ReasoningEvidenceSourceFamily just for this test).
        let has_mai_hoa = eval
            .evidence
            .iter()
            .any(|e| e.source_id == SOURCE_MAI_HOA_DICH_SO);
        let has_kinh_dich = eval
            .evidence
            .iter()
            .any(|e| e.source_id == SOURCE_KINH_DICH);
        assert!(
            has_mai_hoa,
            "evidence must contain a primitive envelope with source_id={SOURCE_MAI_HOA_DICH_SO}"
        );
        assert!(
            has_kinh_dich,
            "evidence must contain a primitive envelope with source_id={SOURCE_KINH_DICH}"
        );

        // Exactly ONE composite envelope with the locked source_id.
        let composite_count = eval
            .evidence
            .iter()
            .filter(|e| e.source_id == COMPOSITE_ICHING_CONSULTATION)
            .count();
        assert_eq!(
            composite_count, 1,
            "evidence must contain exactly 1 composite envelope with \
             source_id={COMPOSITE_ICHING_CONSULTATION}; got {composite_count}"
        );

        // The composite envelope's source_family is Derived.
        let composite = eval
            .evidence
            .iter()
            .find(|e| e.source_id == COMPOSITE_ICHING_CONSULTATION)
            .expect("composite must exist");
        assert_eq!(
            composite.source_family,
            ReasoningEvidenceSourceFamily::Derived,
            "composite envelope's source_family must be Derived"
        );

        // The primitive envelopes (those carrying
        // SOURCE_MAI_HOA_DICH_SO / SOURCE_KINH_DICH) carry source_family
        // = IChing. At least one exists with each source_id.
        let primitive_mai_hoa_family = eval
            .evidence
            .iter()
            .find(|e| e.source_id == SOURCE_MAI_HOA_DICH_SO)
            .map(|e| e.source_family);
        let primitive_kinh_dich_family = eval
            .evidence
            .iter()
            .find(|e| e.source_id == SOURCE_KINH_DICH)
            .map(|e| e.source_family);
        assert_eq!(
            primitive_mai_hoa_family,
            Some(ReasoningEvidenceSourceFamily::IChing),
            "primitive envelope with source_id={SOURCE_MAI_HOA_DICH_SO} must have source_family=IChing"
        );
        assert_eq!(
            primitive_kinh_dich_family,
            Some(ReasoningEvidenceSourceFamily::IChing),
            "primitive envelope with source_id={SOURCE_KINH_DICH} must have source_family=IChing"
        );
    }

    #[test]
    fn iching_evaluator_uses_phase_22_surface_no_reimplementation() {
        // Asserts the evaluator REUSES cast_mai_hoa + derive_bien_que +
        // classify_the_dung rather than re-implementing modulo arithmetic.
        let snap = sample_snapshot();
        let query = IChingQuery::from_snapshot(&snap, None, 7).expect("query");
        let evaluator = IChingEvaluator::new(query.clone());

        // Re-derive the expected values directly from the Phase 22 surface.
        let expected_cast = cast_mai_hoa(
            query.lunar_year_branch,
            query.lunar_month,
            query.lunar_day,
            query.chi_hour_index,
        );
        let expected_bien = derive_bien_que(&expected_cast);
        let expected_td = classify_the_dung(&expected_cast);

        let eval = evaluator
            .evaluate_consultation(&snap)
            .expect("evaluation succeeds");

        // Cast identities match.
        assert_eq!(eval.cast.upper_trigram, expected_cast.upper_trigram);
        assert_eq!(eval.cast.lower_trigram, expected_cast.lower_trigram);
        assert_eq!(eval.cast.dong_hao, expected_cast.dong_hao);
        assert_eq!(eval.cast.chu_que, expected_cast.chu_que);

        // Biến quẻ identity matches.
        assert_eq!(eval.bien_que.king_wen, expected_bien.king_wen);

        // Thể/Dụng verdict matches.
        assert_eq!(eval.the_dung.verdict, expected_td.verdict);
    }

    #[test]
    fn iching_evaluator_is_deterministic() {
        let snap = sample_snapshot();
        let query = IChingQuery::from_snapshot(&snap, None, 8).expect("query");
        let evaluator = IChingEvaluator::new(query);
        let a = evaluator.evaluate_consultation(&snap).expect("first eval");
        let b = evaluator.evaluate_consultation(&snap).expect("second eval");
        assert_eq!(
            a, b,
            "evaluate_consultation must be deterministic (no RNG, no wall-clock)"
        );
    }

    #[test]
    fn iching_evaluator_works_at_tier0_with_no_birth_data() {
        // No birth context — the Tier-0 path. ActionEvaluator::evaluate must
        // return Ok(empty ActionEvaluation) without consulting personal_input.
        let snap = sample_snapshot();
        let query = IChingQuery::from_snapshot(&snap, None, 9).expect("query");
        let evaluator = IChingEvaluator::new(query);
        // Empty graph is fine — the trait-shape adapter doesn't read it.
        let graph = SemanticGraph::new();
        let empty = <IChingEvaluator as ActionEvaluator>::evaluate(
            &evaluator,
            &graph,
            &snap,
            None,
        )
        .expect("Tier-0 evaluate with None personal_input");
        assert_eq!(empty.action_id, ActionId::IChing);

        // With personal_input too — the I Ching baseline ignores it.
        let personal = PersonalReasoningInput::from_birth(
            BirthInput {
                day: 10,
                month: 2,
                year: 1990,
                hour: Some(8),
                minute: Some(0),
                timezone: VIETNAM_TIMEZONE,
                gender: Some(Gender::Male),
                location_name: None,
            },
            ConsultationIntent::OpeningBusiness,
        );
        let with_personal = <IChingEvaluator as ActionEvaluator>::evaluate(
            &evaluator,
            &graph,
            &snap,
            Some(&personal),
        )
        .expect("Tier-0 evaluate with Some personal_input");
        // No-op invariant: both calls must produce the same action_id and
        // same primary_conclusion (the trait-shape adapter is a thin mapper
        // that ignores personal_input).
        assert_eq!(
            empty.action_id, with_personal.action_id,
            "action_id must match across None / Some personal_input"
        );
        assert_eq!(
            empty.primary_conclusion, with_personal.primary_conclusion,
            "primary_conclusion must match — personal_input must be a no-op"
        );
        assert_eq!(
            empty.bucket, with_personal.bucket,
            "bucket must match across None / Some personal_input"
        );
    }

    #[test]
    fn iching_evaluator_select_subgraph_returns_full_graph() {
        let snap = sample_snapshot();
        let query = IChingQuery::from_snapshot(&snap, None, 0).expect("query");
        let evaluator = IChingEvaluator::new(query);
        // DaySnapshotGraphBuilder adds Hexagram nodes; the IChing evaluator
        // doesn't filter. Just exercise the trait method.
        let graph = SemanticGraph::new();
        let selected = <IChingEvaluator as ActionEvaluator>::select_subgraph(
            &evaluator,
            &graph,
            &snap,
            None,
        )
        .expect("select_subgraph succeeds");
        assert_eq!(
            selected.nodes().len(),
            graph.nodes().len(),
            "select_subgraph must return the full graph unchanged"
        );
    }

    #[test]
    fn iching_evaluator_action_id_is_iching() {
        let snap = sample_snapshot();
        let query = IChingQuery::from_snapshot(&snap, None, 0).expect("query");
        let evaluator = IChingEvaluator::new(query);
        assert_eq!(evaluator.action_id(), ActionId::IChing);
    }

    #[test]
    fn iching_evaluator_to_summary_projects_owned_strings() {
        let snap = sample_snapshot();
        let query = IChingQuery::from_snapshot(&snap, Some("test".to_string()), 6).expect("query");
        let evaluator = IChingEvaluator::new(query);
        let eval = evaluator
            .evaluate_consultation(&snap)
            .expect("evaluation succeeds");
        let summary = evaluator.to_summary(&eval);

        // The summary is owned — no &'static refs to the OnceLock-cached corpus.
        assert_eq!(summary.cast.upper_trigram, eval.cast.upper_trigram);
        assert_eq!(summary.bien_que.king_wen, eval.bien_que.king_wen);
        assert_eq!(summary.the_dung.verdict, eval.the_dung.verdict);
        assert_eq!(summary.moving_line, eval.cast.dong_hao);
        assert_eq!(
            summary.cat_hung_summary,
            cat_hung_str(eval.the_dung.verdict)
        );
        assert_eq!(summary.question_vi.as_deref(), Some("test"));
        assert!(
            !summary.chu_hexagram_vi_name.is_empty(),
            "chu_hexagram_vi_name must be non-empty"
        );
        assert!(
            !summary.bien_hexagram_vi_name.is_empty(),
            "bien_hexagram_vi_name must be non-empty"
        );
        assert_eq!(
            summary.evidence.len(),
            eval.evidence.len(),
            "to_summary preserves the full evidence vector"
        );
    }

    /// ──────────────────────────────────────────────────────────────────
    /// cat_hung_str test (the stable string projection)
    /// ──────────────────────────────────────────────────────────────────

    #[test]
    fn cat_hung_str_returns_stable_lowercase_strings() {
        assert_eq!(cat_hung_str(CatHung::Cat), "cat");
        assert_eq!(cat_hung_str(CatHung::Binh), "binh");
        assert_eq!(cat_hung_str(CatHung::Hung), "hung");
    }

    /// ──────────────────────────────────────────────────────────────────
    /// CRIT-3 grep guard — module must NOT define cross-newtype From impls.
    /// Uses RUNTIME-BUILT needles (mirrors Phase 22-01 / 22-02 discipline)
    /// so the test's own doc-comments / source-text don't false-positive.
    /// ──────────────────────────────────────────────────────────────────

    #[test]
    fn crit3_isolation_no_cross_newtype_from_impls_inline() {
        const SRC: &str = include_str!("evaluator.rs");
        let needles: Vec<String> = [
            ("Tien", "ThienTrigram"),
            ("Hau", "ThienTrigram"),
            ("King", "WenHexagram"),
        ]
        .iter()
        .flat_map(|(a, b)| {
            [
                format!("impl From<{a}{b}"),
                format!("impl<{a}{b}> From"),
            ]
        })
        .collect();
        for needle in &needles {
            assert!(
                !SRC.contains(needle.as_str()),
                "CRIT-3 violation: `{needle}` found in evaluator.rs. \
                 The three iching newtypes must NOT have cross-type From impls."
            );
        }
    }

    /// ──────────────────────────────────────────────────────────────────
    /// WASM-safety grep guard — verifies that the module contains no
    /// filesystem/wall-clock/RNG usages (mirrors the 22-02 + corpus.rs
    /// discipline). Uses RUNTIME-BUILT needles so the test's own source
    /// text — which mentions the forbidden patterns by NAME in doc
    /// comments — does not self-trip.
    /// ──────────────────────────────────────────────────────────────────

    #[test]
    fn wasm_safety_no_fs_no_utc_no_rand() {
        const SRC: &str = include_str!("evaluator.rs");
        let mut fs = String::from("std::f");
        fs.push('s');
        let mut utc_now = String::from("Utc");
        utc_now.push_str("::now");
        let mut rand = String::from("rand");
        rand.push_str("::");
        for needle in &[fs.as_str(), utc_now.as_str(), rand.as_str()] {
            assert!(
                !SRC.contains(needle),
                "WASM-safety violation: `{needle}` found in evaluator.rs. \
                 WASM targets have no filesystem, no wall-clock, no RNG."
            );
        }
    }

    /// ──────────────────────────────────────────────────────────────────
    /// COMPOSITE_ICHING_CONSULTATION contract — single audit point.
    /// ──────────────────────────────────────────────────────────────────

    #[test]
    fn composite_iching_consultation_constant_has_expected_value() {
        assert_eq!(COMPOSITE_ICHING_CONSULTATION, "rule.composite.iching_consultation");
    }

    /// ──────────────────────────────────────────────────────────────────
    /// IChingCastSummary — owned DTO structure round-trip.
    /// ──────────────────────────────────────────────────────────────────

    #[test]
    fn iching_cast_summary_struct_has_expected_owned_fields() {
        // Build a sample IChingCastSummary via a snapshot and verify the
        // owned-string discipline + length-matters round-trip.
        let snap = sample_snapshot();
        let query = IChingQuery::from_snapshot(&snap, None, 4).expect("query");
        let evaluator = IChingEvaluator::new(query);
        let eval = evaluator
            .evaluate_consultation(&snap)
            .expect("evaluation succeeds");
        let summary = evaluator.to_summary(&eval);

        // Every field is owned (no &'static). Verify by trying to construct
        // a second summary by cloning.
        let cloned = summary.clone();
        assert_eq!(cloned, summary);
    }

    /// ──────────────────────────────────────────────────────────────────
    /// Trigram identity cross-check — TienThienTrigram vs HauThienTrigram.
    /// Spot-checks that no implicit conversion exists (CRIT-3 isolation).
    /// ──────────────────────────────────────────────────────────────────

    #[test]
    fn evaluator_uses_only_tien_thien_trigram_identities_no_hau_thien() {
        let snap = sample_snapshot();
        let query = IChingQuery::from_snapshot(&snap, None, 1).expect("query");
        let evaluator = IChingEvaluator::new(query);
        let eval = evaluator
            .evaluate_consultation(&snap)
            .expect("evaluation succeeds");

        // The cast carries TienThienTrigram identities (same as the Phase 22
        // surface). The biến quẻ also carries TienThienTrigram identities.
        // The corpus projection (HexagramEntryProjection) does NOT carry the
        // trigram itself — the projection's `king_wen_index` is the only
        // corpus-side handle. This is the CRIT-3 discipline: the evaluator
        // uses TienThien for casting (input), and only accesses the King Wen
        // corpus (output) without naming the HauThien arrangement internally.
        let _ = TienThienTrigram::Kien; // touches the type to keep the import live.
        let _ = KingWenHexagram(1);
        let _ = HauThienTrigram::Kien; // CRIT-3: TienThien ≠ HauThien — no implicit conv.
        let upper = eval.cast.upper_trigram;
        assert!(matches!(
            upper,
            TienThienTrigram::Kien
                | TienThienTrigram::Doai
                | TienThienTrigram::Ly
                | TienThienTrigram::Chan
                | TienThienTrigram::Ton
                | TienThienTrigram::Kham
                | TienThienTrigram::Can
                | TienThienTrigram::Khon
        ));
    }

    /// ──────────────────────────────────────────────────────────────────
    /// Tap the eval machinery with explicit reasoning envelope construction
    /// (simulating the per-step envelope shape the GREEN code must build).
    /// RED: every assertion on the per-step shape fails because
    /// evaluate_consultation panics first.
    /// ──────────────────────────────────────────────────────────────────

    #[test]
    fn iching_evaluator_per_step_evidence_envelope_methods() {
        // Locked per-step evidence construction shape — captured in a
        // standalone test so the GREEN code is forced to populate the
        // exact envelope ordering documented in the plan.
        let expected_methods = [
            "cast_mai_hoa", "derive_bien_que", "corpus_lookup", "iching_consultation",
        ];
        let snap = sample_snapshot();
        let query = IChingQuery::from_snapshot(&snap, None, 9).expect("query");
        let evaluator = IChingEvaluator::new(query);
        let eval = evaluator
            .evaluate_consultation(&snap)
            .expect("evaluation succeeds");

        // Every evidence envelope's method must be one of the four locked
        // per-step methods (cast_mai_hoa + derive_bien_que + corpus_lookup
        // + iching_consultation).
        for e in &eval.evidence {
            assert!(
                expected_methods.contains(&e.method.as_str()),
                "evidence envelope method must be one of {expected_methods:?}; got {}",
                e.method
            );
            // The composite envelope's method is "iching_consultation".
            if e.source_id == COMPOSITE_ICHING_CONSULTATION {
                assert_eq!(e.method, "iching_consultation");
            }
        }

        // Sanity: source_id values come from the SOURCE_* consts, not bare
        // literals (the actual call-sites in the GREEN code use the consts
        // directly — this is a downstream-consumer shape check, not a
        // grep guard).
        for e in &eval.evidence {
            let _ = ReasoningEvidenceEnvelope {
                source_family: e.source_family,
                source_id: if e.source_id == SOURCE_MAI_HOA_DICH_SO {
                    SOURCE_MAI_HOA_DICH_SO.to_string()
                } else if e.source_id == SOURCE_KINH_DICH {
                    SOURCE_KINH_DICH.to_string()
                } else if e.source_id == COMPOSITE_ICHING_CONSULTATION {
                    COMPOSITE_ICHING_CONSULTATION.to_string()
                } else {
                    panic!("unexpected source_id: {}", e.source_id);
                },
                method: e.method.clone(),
                note: e.note.clone(),
            };
        }
    }
}
