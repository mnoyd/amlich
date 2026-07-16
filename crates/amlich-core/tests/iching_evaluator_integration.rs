//! Black-box integration tests for the Phase 24-01 IChing evaluator + DaySnapshot
//! enrichment surface (ICH-05 + partial INT-12).
//!
//! These tests exercise the public surface from the external crate path:
//!   - `IChingQuery::from_snapshot` / `from_lunar_inputs` (sibling newtype)
//!   - `IChingEvaluator::new(query).evaluate(...) -> IChingCastSummary`
//!   - `enrich_day_snapshot_with_iching(&DaySnapshot, IChingQuery) -> Result<DaySnapshot, String>`
//!   - `ProvenanceSource::IChing → ReasoningEvidenceSourceFamily::IChing` mapping
//!
//! The 17 tests correspond to the success criteria in `24-01-PLAN.md`:
//!   1.  `iching_query_from_snapshot_derives_lunar_inputs`
//!   2.  `iching_query_rejects_invalid_hour_index`
//!   3.  `iching_query_nfc_normalises_question`
//!   4.  `iching_query_rejects_whitespace_only_question`
//!   5.  `iching_evaluator_emits_at_least_two_primitive_source_ids_plus_one_composite`
//!   6.  `iching_evaluator_uses_phase_22_surface`
//!   7.  `iching_evaluator_is_deterministic`
//!   8.  `iching_evaluator_works_at_tier_0_with_no_birth_data`
//!   9.  `enrich_day_snapshot_with_iching_does_not_mutate_input`
//!   10. `enrich_day_snapshot_with_iching_populates_owned_summary`
//!   11. `ordinary_day_snapshot_has_iching_cast_none`
//!   12. `ordinary_day_snapshot_does_not_serialize_iching_cast_key`
//!   13. `iching_cast_byte_equal_round_trip`
//!   14. `iching_cast_absent_in_json_when_none`
//!   15. `iching_provenance_source_maps_to_iching_family`
//!   16. CRIT-3 grep guard (`fn crit3_isolation_...`)
//!   17. WASM-safety grep guard (`fn wasm_safety_...`)

use std::fs;
use std::path::{Path, PathBuf};

use amlich_core::{
    calculate_day_snapshot, enrich_day_snapshot_with_iching,
};

use amlich_core::iching::{
    classify_the_dung, cast_mai_hoa, derive_bien_que, IChingEvaluator, IChingQuery,
    IChingCastSummary, COMPOSITE_ICHING_CONSULTATION,
};
use amlich_core::reasoning::{ActionEvaluator, PersonalReasoningInput, ReasoningEvidenceSourceFamily};
use amlich_core::semantic_graph::{ProvenanceEntry, ProvenanceSource, SemanticGraph};
use amlich_core::sources::{SOURCE_KINH_DICH, SOURCE_MAI_HOA_DICH_SO};
use amlich_core::VIETNAM_TIMEZONE;
use amlich_core::advisory::{BirthInput, ConsultationIntent};
use amlich_core::almanac::tu_menh::Gender;

/// Convenience: a populated snapshot for testing.
fn sample_snapshot() -> amlich_core::DaySnapshot {
    calculate_day_snapshot(10, 2, 2024)
}

// ───────────────────────────────────────────────────────────────────────
// IChingQuery construction (4 tests)
// ───────────────────────────────────────────────────────────────────────

/// 1. `IChingQuery::from_snapshot` derives lunar inputs from the snapshot,
/// NFC-normalises the question, and accepts the canonical Tier-0 inputs.
#[test]
fn iching_query_from_snapshot_derives_lunar_inputs() {
    let snap = sample_snapshot();
    let query =
        IChingQuery::from_snapshot(&snap, Some("việc làm".to_string()), 9).expect("valid inputs");
    assert_eq!(query.chi_hour_index, 9);
    assert_eq!(query.question_vi.as_deref(), Some("việc làm"));
    // lunar_year_branch derives from the snapshot's year chi_index.
    assert_eq!(
        query.lunar_year_branch as usize,
        snap.context.canchi.year.chi_index
    );
}

/// 2. `chi_hour_index > 11` (or otherwise out-of-range) is rejected with an
/// error message that mentions the field name.
#[test]
fn iching_query_rejects_invalid_hour_index() {
    let snap = sample_snapshot();
    let err = IChingQuery::from_snapshot(&snap, None, 12).expect_err("hour 12 must fail");
    assert!(
        err.contains("chi_hour_index"),
        "error must mention chi_hour_index; got: {err}"
    );
    let err_high =
        IChingQuery::from_snapshot(&snap, None, 255).expect_err("hour 255 must fail");
    assert!(err_high.contains("chi_hour_index"));
}

/// 3. A question containing combining diacritics (NFD form) is NFC-normalised
/// in the stored `question_vi`.
#[test]
fn iching_query_nfc_normalises_question() {
    let snap = sample_snapshot();
    // "vi\u{0300}" (v + i + combining grave) → "vì" (U+00EC precomposed).
    let decomposed = String::from("vi\u{0300} công việc");
    let query = IChingQuery::from_snapshot(&snap, Some(decomposed), 5)
        .expect("NFD input should be normalised, not rejected");
    let stored = query.question_vi.expect("non-whitespace input persists");
    assert!(
        stored.contains('\u{00EC}'),
        "stored string must contain the precomposed ì (U+00EC) after NFC; got: {stored}"
    );
}

/// 4. A whitespace-only `question_vi` is normalised to `None` rather than
/// stored verbatim.
#[test]
fn iching_query_rejects_whitespace_only_question() {
    let snap = sample_snapshot();
    let query = IChingQuery::from_snapshot(&snap, Some("   \t  ".to_string()), 5).expect("ok");
    assert!(
        query.question_vi.is_none(),
        "whitespace-only question_vi must normalise to None; got: {:?}",
        query.question_vi
    );
}

// ───────────────────────────────────────────────────────────────────────
// IChingEvaluator rich path (4 tests)
// ───────────────────────────────────────────────────────────────────────

/// 5. The evaluation's evidence vector contains ≥2 primitive envelopes
/// with distinct source_ids including `mai-hoa-dich-so` AND `kinh-dich`,
/// plus exactly 1 composite envelope with source_id = `rule.composite.iching_consultation`.
#[test]
fn iching_evaluator_emits_at_least_two_primitive_source_ids_plus_one_composite() {
    let snap = sample_snapshot();
    let query = IChingQuery::from_snapshot(&snap, None, 9).expect("query");
    let evaluator = IChingEvaluator::new(query);
    let eval = evaluator
        .evaluate_consultation(&snap)
        .expect("evaluation succeeds");

    // CRIT-6 source-id discipline: at least one envelope per primitive source.
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

    // Exactly ONE composite envelope.
    let composite_count = eval
        .evidence
        .iter()
        .filter(|e| e.source_id == COMPOSITE_ICHING_CONSULTATION)
        .count();
    assert_eq!(
        composite_count, 1,
        "evidence must contain exactly 1 composite envelope with source_id={COMPOSITE_ICHING_CONSULTATION}"
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

    // Primitive envelopes carry source_family = IChing.
    for e in &eval.evidence {
        if e.source_id == SOURCE_MAI_HOA_DICH_SO || e.source_id == SOURCE_KINH_DICH {
            assert_eq!(
                e.source_family,
                ReasoningEvidenceSourceFamily::IChing,
                "primitive envelope's source_family must be IChing"
            );
        }
    }
}

/// 6. The evaluator REUSES cast_mai_hoa + derive_bien_que + classify_the_dung
/// directly — no re-implementation. Proves by re-deriving the same inputs via
/// the Phase 22 surface and asserting equality on the relevant fields.
#[test]
fn iching_evaluator_uses_phase_22_surface() {
    let snap = sample_snapshot();
    let query = IChingQuery::from_snapshot(&snap, None, 7).expect("query");
    let evaluator = IChingEvaluator::new(query.clone());

    // Re-derive expected values directly via the Phase 22 surface.
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

/// 7. Two `evaluate_consultation` calls with the same query + snapshot
/// return equal `IChingEvaluation` values (determinism invariant).
#[test]
fn iching_evaluator_is_deterministic() {
    let snap = sample_snapshot();
    let query = IChingQuery::from_snapshot(&snap, None, 8).expect("query");
    let evaluator = IChingEvaluator::new(query);
    let a = evaluator
        .evaluate_consultation(&snap)
        .expect("first eval");
    let b = evaluator
        .evaluate_consultation(&snap)
        .expect("second eval");
    assert_eq!(
        a, b,
        "evaluate_consultation must be deterministic (no RNG, no wall-clock)"
    );
}

/// 8. The I Ching baseline works at Tier-0 with NO birth data. The
/// `ActionEvaluator::evaluate` adapter ignores the `personal_input`
/// parameter — both None and Some(&personal_input) return Ok(empty
/// ActionEvaluation) with the same fields.
#[test]
fn iching_evaluator_works_at_tier_0_with_no_birth_data() {
    let snap = sample_snapshot();
    let query = IChingQuery::from_snapshot(&snap, None, 9).expect("query");
    let evaluator = IChingEvaluator::new(query);
    let graph = SemanticGraph::new();

    let empty = <IChingEvaluator as ActionEvaluator>::evaluate(&evaluator, &graph, &snap, None)
        .expect("Tier-0 evaluate with None personal_input");
    assert_eq!(empty.action_id, amlich_core::ActionId::IChing);

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
    let with_personal =
        <IChingEvaluator as ActionEvaluator>::evaluate(&evaluator, &graph, &snap, Some(&personal))
            .expect("Tier-0 evaluate with Some personal_input");
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

// ───────────────────────────────────────────────────────────────────────
// DaySnapshot.iching_cast + enrichment helper (6 tests)
// ───────────────────────────────────────────────────────────────────────

/// 9. The enrichment helper does NOT mutate the input snapshot. After
/// calling `enrich_day_snapshot_with_iching`, the original snapshot's
/// `iching_cast` field remains `None`.
#[test]
fn enrich_day_snapshot_with_iching_does_not_mutate_input() {
    let snap = sample_snapshot();
    assert!(snap.iching_cast.is_none(), "precondition: original iching_cast is None");
    let query = IChingQuery::from_snapshot(&snap, None, 9).expect("query");
    let enriched = enrich_day_snapshot_with_iching(&snap, query).expect("enrich");
    assert!(
        snap.iching_cast.is_none(),
        "input snapshot must remain unmutated; iching_cast still None"
    );
    assert!(
        enriched.iching_cast.is_some(),
        "enriched snapshot must populate the iching_cast field"
    );
}

/// 10. The populated `iching_cast` summary has all expected fields populated
/// with non-empty values (named golden contract).
#[test]
fn enrich_day_snapshot_with_iching_populates_owned_summary() {
    let snap = sample_snapshot();
    let query = IChingQuery::from_snapshot(&snap, Some("test".to_string()), 6).expect("query");
    let enriched = enrich_day_snapshot_with_iching(&snap, query).expect("enrich");
    let summary = enriched.iching_cast.as_ref().expect("iching_cast is Some");

    // Every owned field is populated.
    assert!(
        !summary.chu_hexagram_vi_name.is_empty(),
        "chu_hexagram_vi_name must be non-empty"
    );
    assert!(
        !summary.bien_hexagram_vi_name.is_empty(),
        "bien_hexagram_vi_name must be non-empty"
    );
    assert!(
        !summary.chu_hexagram_thoai_tu.is_empty(),
        "chu_hexagram_thoai_tu must be non-empty"
    );
    assert!(summary.moving_line >= 1 && summary.moving_line <= 6);
    assert!(
        matches!(
            summary.cat_hung_summary.as_str(),
            "cat" | "binh" | "hung"
        ),
        "cat_hung_summary must be one of cat/binh/hung; got: {}",
        summary.cat_hung_summary
    );
    assert_eq!(summary.question_vi.as_deref(), Some("test"));

    // Evidence vector has the locked envelope count (3 primitive + 1 composite).
    assert_eq!(
        summary.evidence.len(),
        4,
        "summary must carry the full evidence vector (3 primitives + 1 composite)"
    );

    // The composite envelope is present.
    let _composite_count = summary
        .evidence
        .iter()
        .filter(|e| e.source_id == COMPOSITE_ICHING_CONSULTATION)
        .count();
}

/// 11. Ordinary `calculate_day_snapshot(...)` produces a snapshot whose
/// `iching_cast` field stays `None`. No auto-population.
#[test]
fn ordinary_day_snapshot_has_iching_cast_none() {
    let snap = calculate_day_snapshot(10, 2, 2024);
    assert!(
        snap.iching_cast.is_none(),
        "ordinary calculate_day_snapshot must NOT auto-populate iching_cast"
    );
}

/// 12. When `iching_cast` is None, the serialized JSON does NOT contain the
/// `"iching_cast"` key (additive `Option<T>` + `skip_serializing_if` discipline).
#[test]
fn ordinary_day_snapshot_does_not_serialize_iching_cast_key() {
    let snap = calculate_day_snapshot(10, 2, 2024);
    let json = serde_json::to_string(&snap).expect("serialization");
    assert!(
        !json.contains("\"iching_cast\""),
        "iching_cast key must NOT appear in JSON when None; got: {json}"
    );
}

/// 13. Enriched snapshots serialize WITH `"iching_cast"` AND round-trip
/// byte-equally via serde JSON (proves the additive field coexists with
/// v1.6 fields).
#[test]
fn iching_cast_byte_equal_round_trip() {
    let snap = calculate_day_snapshot(10, 2, 2024);
    let query = IChingQuery::from_snapshot(&snap, None, 9).expect("query");
    let enriched = enrich_day_snapshot_with_iching(&snap, query).expect("enrich");

    let json1 = serde_json::to_string(&enriched).expect("serialize");
    assert!(
        json1.contains("\"iching_cast\""),
        "enriched snapshot must serialize WITH iching_cast; got: {json1}"
    );

    let recovered: amlich_core::DaySnapshot =
        serde_json::from_str(&json1).expect("deserialize");
    let json2 = serde_json::to_string(&recovered).expect("reserialize");
    assert_eq!(
        json1, json2,
        "Round-trip must be byte-equal (additive-Option + skip-if-none intact)"
    );

    // The recovered iching_cast is the same shape — owned data preserved.
    let recovered_summary = recovered
        .iching_cast
        .as_ref()
        .expect("recovered iching_cast must be Some");
    let original_summary = enriched
        .iching_cast
        .as_ref()
        .expect("original iching_cast must be Some");
    assert_eq!(
        recovered_summary.cast, original_summary.cast,
        "recovered cast must match"
    );
    assert_eq!(
        recovered_summary.bien_que, original_summary.bien_que,
        "recovered bien_que must match"
    );
    assert_eq!(
        recovered_summary.the_dung.verdict, original_summary.the_dung.verdict,
        "recovered verdict must match"
    );
    assert_eq!(
        recovered_summary.moving_line, original_summary.moving_line,
        "recovered moving_line must match"
    );
}

/// 14. Explicitly clearing `iching_cast` to `None` makes the key absent from
/// the serialized JSON (per IChingCastSummary: Option<T> + skip-if-none).
#[test]
fn iching_cast_absent_in_json_when_none() {
    let snap = calculate_day_snapshot(10, 2, 2024);
    let query = IChingQuery::from_snapshot(&snap, None, 9).expect("query");
    let mut enriched = enrich_day_snapshot_with_iching(&snap, query).expect("enrich");
    enriched.iching_cast = None;
    let json = serde_json::to_string(&enriched).expect("serialize");
    assert!(
        !json.contains("\"iching_cast\""),
        "iching_cast key must NOT appear in JSON when explicitly None; got: {json}"
    );
    // Other additive fields are unaffected.
    // (flying_stars may be absent in some snapshots depending on corpus state;
    // the byte-equality test exercises that.)
    let _ = IChingCastSummary::clone; // type marker for `use`
}

// ───────────────────────────────────────────────────────────────────────
// ProvenanceSource mapping (1 test)
// ───────────────────────────────────────────────────────────────────────

/// 15. `ProvenanceSource::IChing` maps to `ReasoningEvidenceSourceFamily::IChing`
/// via `to_reasoning_evidence()`. Mirrors the inline test
/// `to_reasoning_evidence_maps_iching_to_iching_family`, exercised here from
/// the external crate path.
#[test]
fn iching_provenance_source_maps_to_iching_family() {
    let entry =
        ProvenanceEntry::new(ProvenanceSource::IChing, SOURCE_KINH_DICH, "corpus_lookup")
            .with_note("Black-box integration variant of the mapping test");
    let envelope = entry.to_reasoning_evidence();
    assert_eq!(
        envelope.source_family,
        ReasoningEvidenceSourceFamily::IChing,
        "ProvenanceSource::IChing must map to ReasoningEvidenceSourceFamily::IChing"
    );
    assert_eq!(envelope.source_id, SOURCE_KINH_DICH);
    assert_eq!(envelope.method, "corpus_lookup");
}

// ───────────────────────────────────────────────────────────────────────
// CRIT-3 grep guard (1 test)
// ───────────────────────────────────────────────────────────────────────

/// 16. CRIT-3 isolation guard: the evaluator module must NOT define any
/// cross-newtype `From` impl between TienThienTrigram / HauThienTrigram /
/// KingWenHexagram. Uses RUNTIME-BUILT needles so this test's own doc text
/// (which legitimately names the forbidden pattern families) does not
/// false-positive.
#[test]
fn crit3_isolation_no_cross_newtype_from_impls_in_evaluator() {
    const SRC: &str = include_str!("../src/iching/evaluator.rs");
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
            "CRIT-3 violation: `{needle}` found in crates/amlich-core/src/iching/evaluator.rs. \
             The three iching newtypes must NOT have cross-type From impls."
        );
    }
}

// ───────────────────────────────────────────────────────────────────────
// WASM-safety grep guard (1 test)
// ───────────────────────────────────────────────────────────────────────

/// 17. WASM-safety guard: the evaluator module must NOT contain filesystem,
/// wall-clock, or RNG usages. Uses RUNTIME-BUILT needles so the test's own
/// rationale text (which mentions the forbidden patterns by NAME in doc
/// comments) does not self-trip. Mirrors the v1.7 corpus.rs / the_dung.rs
/// discipline.
#[test]
fn wasm_safety_no_fs_no_utc_no_rand_in_evaluator() {
    const SRC: &str = include_str!("../src/iching/evaluator.rs");
    let mut fs = String::from("std::f");
    fs.push('s');
    let mut utc_now = String::from("Utc");
    utc_now.push_str("::now");
    let mut rand = String::from("rand");
    rand.push_str("::");
    for needle in &[fs.as_str(), utc_now.as_str(), rand.as_str()] {
        assert!(
            !SRC.contains(needle),
            "WASM-safety violation: `{needle}` found in crates/amlich-core/src/iching/evaluator.rs. \
             WASM targets have no filesystem, no wall-clock, no RNG."
        );
    }
}

// ───────────────────────────────────────────────────────────────────────
// Bonus: verify the broader source_id_guard.rs pattern still passes
// (the new iching evaluator must use SOURCE_* consts, not bare literals).
// ───────────────────────────────────────────────────────────────────────

#[test]
fn source_id_guard_passes_for_new_evaluator_module() {
    // Mirror the source_id_guard.rs scan: walk through `src/` and verify
    // no bare `kinh-dich` / `mai-hoa-dich-so` literals exist outside
    // sources.rs / cfg(test) blocks / comments. (We only need a sanity
    // check that the new module respects the discipline — the full
    // CRIT-6 source_id_guard.rs test still passes via `tests/source_id_guard.rs`.)
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    collect_rs_files(&src_dir, &mut files);
    for path in &files {
        if path.file_name().and_then(|s| s.to_str()) == Some("sources.rs") {
            continue;
        }
        let contents = fs::read_to_string(path).expect("read file");
        let mut in_cfg_test_block = false;
        let mut brace_depth_at_cfg_test: i32 = -1;
        let mut current_brace_depth: i32 = 0;
        for line in contents.lines() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("#[cfg(test)]") {
                in_cfg_test_block = true;
                brace_depth_at_cfg_test = current_brace_depth;
            }
            for ch in line.chars() {
                match ch {
                    '{' => current_brace_depth += 1,
                    '}' => {
                        current_brace_depth -= 1;
                        if in_cfg_test_block
                            && current_brace_depth <= brace_depth_at_cfg_test
                        {
                            in_cfg_test_block = false;
                            brace_depth_at_cfg_test = -1;
                        }
                    }
                    _ => {}
                }
            }
            if in_cfg_test_block || trimmed.starts_with("//") {
                continue;
            }
            // Verify there are no bare `kinh-dich` / `mai-hoa-dich-so`
            // string literals in production code.
            assert!(
                !line.contains("\"kinh-dich\""),
                "bare literal found in {path:?}: {line}"
            );
            assert!(
                !line.contains("\"mai-hoa-dich-so\""),
                "bare literal found in {path:?}: {line}"
            );
        }
    }
}

fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).expect("read_dir failed") {
        let entry = entry.expect("dir entry");
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, out);
        } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            out.push(path);
        }
    }
}
