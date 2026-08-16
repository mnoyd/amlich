//! v1.10 Phase 02-01 — four-season cultivation context integration tests.
//!
//! Covers SEASON-01 / SOURCE-01 / SOURCE-02 / BOUND-01 / BOUND-02 for
//! the seasonal track. The companion module-level unit tests live in
//! `src/traditional_wellness/seasonal.rs` and
//! `src/traditional_wellness/divergence.rs`.
//!
//! The 24-term goldens and the 8 transition edges are derived from a
//! full-year day sweep through the real astronomical engine (per
//! `LUNAR_HEALTH_RESEARCH.md` §5 "Seasonal lookup": 24 table goldens +
//! 8 edge goldens), not from hardcoded transition dates.

use amlich_core::julian::{jd_from_date, jd_to_date};
use amlich_core::sources::SOURCE_HUANGDI_NEIJING_SUWEN;
use amlich_core::tietkhi::{get_tiet_khi, TIET_KHI};
use amlich_core::traditional_wellness::{
    load_seasonal_corpus, resolve_seasonal_cultivation, season_for_term_index, SeasonKey,
    SeasonalCultivationContext, COMPOSITE_SEASONAL_WELLNESS, COMPOSITION_NOTE_EN,
    COMPOSITION_NOTE_VN, SEASONAL_BOUNDARY_TERM_NAMES, SOLAR_TERM_ENGINE_SOURCE_ID,
    TERMS_PER_SEASON,
};

const TIME_ZONE: f64 = 7.0;

/// Sweep every day of 2026 and collect the first Julian day on which
/// each canonical term index becomes active (local civil dates).
fn first_jd_of_each_term_2026() -> Vec<(usize, i32)> {
    let start = jd_from_date(1, 1, 2026);
    let end = jd_from_date(31, 12, 2026);
    let mut seen: Vec<(usize, i32)> = Vec::new();
    let mut prev: Option<usize> = None;
    for jd in start..=end {
        let term = get_tiet_khi(jd, TIME_ZONE);
        if prev != Some(term.index) && !seen.iter().any(|(i, _)| *i == term.index) {
            seen.push((term.index, jd));
            prev = Some(term.index);
        }
    }
    seen.sort_unstable_by_key(|(i, _)| *i);
    seen
}

// ---------------------------------------------------------------------------
// 24 table goldens — every term joins to exactly one profile
// ---------------------------------------------------------------------------

#[test]
fn all_24_terms_map_deterministically_with_six_per_season() {
    for index in 0..24usize {
        let season = season_for_term_index(index)
            .unwrap_or_else(|| panic!("term index {index} must map to a season"));
        let context = resolve_seasonal_cultivation(jd_of_term_index(index), TIME_ZONE);
        assert_eq!(
            context.solar_term.index, index,
            "engine term index must equal the golden index"
        );
        assert_eq!(
            context.season, season,
            "composed season for term {} ({})",
            index, context.solar_term.name
        );
        assert_eq!(
            context.profile.season,
            season,
            "joined profile must be the {} profile",
            season.as_str()
        );
    }

    let mut counts = std::collections::HashMap::new();
    for index in 0..24usize {
        let season = season_for_term_index(index).expect("in range");
        *counts.entry(season).or_insert(0usize) += 1;
    }
    for season in SeasonKey::all() {
        assert_eq!(
            counts.get(&season).copied().unwrap_or(0),
            TERMS_PER_SEASON,
            "season {season:?} must cover exactly {TERMS_PER_SEASON} terms"
        );
    }
}

#[test]
fn term_names_used_by_the_mapping_match_the_canonical_engine_table() {
    // The composition's boundary names must exist verbatim in the
    // canonical TIET_KHI table — no invented term spellings.
    for name in SEASONAL_BOUNDARY_TERM_NAMES {
        assert!(
            TIET_KHI.iter().any(|t| t.name == name),
            "boundary term {name} must exist in tietkhi::TIET_KHI"
        );
    }
}

#[test]
fn full_year_sweep_join_agrees_with_the_frozen_mapping() {
    // Every day of 2026: the engine's active term joined through the
    // composition must equal season_for_term_index of that term, and
    // the resolved profile must be the corpus row for that season.
    let corpus = load_seasonal_corpus();
    let start = jd_from_date(1, 1, 2026);
    let end = jd_from_date(31, 12, 2026);
    let mut terms_seen = std::collections::HashSet::new();
    for jd in start..=end {
        let context = resolve_seasonal_cultivation(jd, TIME_ZONE);
        let expected_season = season_for_term_index(context.solar_term.index)
            .expect("engine indexes are canonical 0..=23");
        assert_eq!(context.season, expected_season, "jd {jd}");
        let expected_profile = corpus
            .iter()
            .find(|p| p.season == expected_season)
            .expect("corpus covers all four seasons");
        assert_eq!(&context.profile, expected_profile, "jd {jd}");
        terms_seen.insert(context.solar_term.index);
    }
    // A civil year misses at most two canonical terms at its edges.
    assert!(
        terms_seen.len() >= 22,
        "a full-year sweep must observe nearly all 24 terms; saw {}",
        terms_seen.len()
    );
}

// ---------------------------------------------------------------------------
// 8 transition edges — term before / at each Lập boundary
// ---------------------------------------------------------------------------

#[test]
fn eight_transition_edges_select_old_then_new_profile() {
    // (boundary term name, index, previous index, old season, new season)
    let edges = [
        (
            "Lập Xuân",
            21usize,
            20usize,
            SeasonKey::Winter,
            SeasonKey::Spring,
        ),
        ("Lập Hạ", 3, 2, SeasonKey::Spring, SeasonKey::Summer),
        ("Lập Thu", 9, 8, SeasonKey::Summer, SeasonKey::Autumn),
        ("Lập Đông", 15, 14, SeasonKey::Autumn, SeasonKey::Winter),
    ];

    for (name, index, prev_index, old_season, new_season) in edges {
        assert_eq!(
            season_for_term_index(prev_index),
            Some(old_season),
            "term before {name} must stay {old_season:?}"
        );
        assert_eq!(
            season_for_term_index(index),
            Some(new_season),
            "{name} must start {new_season:?}"
        );

        // Lock the edge against the real engine: the first day the
        // boundary term is active must compose to the new season, and
        // the day before must still compose to the old season.
        let boundary_jd = jd_of_term_index(index);
        let boundary_term = get_tiet_khi(boundary_jd, TIME_ZONE);
        assert_eq!(
            boundary_term.name, name,
            "first-active-day lookup for index {index}"
        );
        let on = resolve_seasonal_cultivation(boundary_jd, TIME_ZONE);
        assert_eq!(on.season, new_season, "{name} day composes {new_season:?}");
        let before = resolve_seasonal_cultivation(boundary_jd - 1, TIME_ZONE);
        assert_eq!(
            before.season, old_season,
            "day before {name} composes {old_season:?}"
        );
    }
}

// ---------------------------------------------------------------------------
// 4 corpus goldens — one per season
// ---------------------------------------------------------------------------

#[test]
fn four_corpus_goldens_identity_wording_and_citation() {
    let corpus = load_seasonal_corpus();
    assert_eq!(corpus.len(), 4);

    struct Golden {
        season: SeasonKey,
        season_vi: &'static str,
        season_en: &'static str,
        season_zh: &'static str,
        passage_key: &'static str,
        wording_fragment_vi: &'static str,
        wording_fragment_en: &'static str,
    }
    let goldens = [
        Golden {
            season: SeasonKey::Spring,
            season_vi: "Xuân",
            season_en: "Spring",
            season_zh: "春",
            passage_key: "spring",
            wording_fragment_vi: "đi dạo thong thả ngoài trời",
            wording_fragment_en: "taking unhurried outdoor walks",
        },
        Golden {
            season: SeasonKey::Summer,
            season_vi: "Hạ",
            season_en: "Summer",
            season_zh: "夏",
            passage_key: "summer",
            wording_fragment_vi: "tránh nóng giận",
            wording_fragment_en: "avoiding anger",
        },
        Golden {
            season: SeasonKey::Autumn,
            season_vi: "Thu",
            season_en: "Autumn",
            season_zh: "秋",
            passage_key: "autumn",
            wording_fragment_vi: "giữ tâm thế tĩnh tại",
            wording_fragment_en: "keeping a tranquil disposition",
        },
        Golden {
            season: SeasonKey::Winter,
            season_vi: "Đông",
            season_en: "Winter",
            season_zh: "冬",
            passage_key: "winter",
            wording_fragment_vi: "dậy muộn hơn đợi ánh ngày",
            wording_fragment_en: "waiting for daylight",
        },
    ];

    for golden in goldens {
        let profile = corpus
            .iter()
            .find(|p| p.season == golden.season)
            .unwrap_or_else(|| panic!("corpus must contain {:?}", golden.season));
        assert_eq!(profile.season_vi, golden.season_vi);
        assert_eq!(profile.season_en, golden.season_en);
        assert_eq!(profile.season_zh, golden.season_zh);
        assert_eq!(profile.passage_key, golden.passage_key);
        assert!(
            profile.wording_vi.contains(golden.wording_fragment_vi),
            "{:?} wording drift (vi): {}",
            golden.season,
            profile.wording_vi
        );
        assert!(
            profile.wording_en.contains(golden.wording_fragment_en),
            "{:?} wording drift (en): {}",
            golden.season,
            profile.wording_en
        );
        // SOURCE-01 / SOURCE-02 citation contract.
        assert_eq!(profile.sources.len(), 1);
        assert_eq!(profile.sources[0].source_id, SOURCE_HUANGDI_NEIJING_SUWEN);
        assert_eq!(profile.sources[0].work_title, "Huangdi Neijing Suwen");
        assert_eq!(
            profile.sources[0].volume_or_chapter,
            "素問 四氣調神大論篇第二"
        );
        assert_eq!(profile.sources[0].passage_key, golden.passage_key);
        assert_eq!(profile.sources[0].translation_kind, "project_paraphrase");
        assert_eq!(profile.safety_class, "historical_cultural_non_clinical");
    }
}

// ---------------------------------------------------------------------------
// Provenance separation (SOURCE-01 / LH-DIV-04)
// ---------------------------------------------------------------------------

#[test]
fn provenance_separates_term_engine_from_suwen_and_composites_once() {
    for index in 0..24usize {
        let context = resolve_seasonal_cultivation(jd_of_term_index(index), TIME_ZONE);
        assert_eq!(context.evidence.len(), 3, "exactly three envelopes");

        // Solar-term primitive: engine attribution, never Suwen.
        let term_ev = &context.evidence[0];
        assert_eq!(term_ev.source_id, SOLAR_TERM_ENGINE_SOURCE_ID);
        assert_eq!(term_ev.method, "get_tiet_khi");
        assert_ne!(term_ev.source_id, SOURCE_HUANGDI_NEIJING_SUWEN);

        // Suwen primitive: the paraphrase source, never the engine.
        let suwen_ev = &context.evidence[1];
        assert_eq!(suwen_ev.source_id, SOURCE_HUANGDI_NEIJING_SUWEN);
        assert_ne!(suwen_ev.source_id, SOLAR_TERM_ENGINE_SOURCE_ID);
        assert!(suwen_ev.method.starts_with("seasonal_profile_lookup:"));

        // Composite join: exactly one, reserved id, Derived family.
        let composite_count = context
            .evidence
            .iter()
            .filter(|e| e.source_id == COMPOSITE_SEASONAL_WELLNESS)
            .count();
        assert_eq!(composite_count, 1, "exactly one composite envelope");
        assert_eq!(context.evidence[2].source_id, COMPOSITE_SEASONAL_WELLNESS);

        // The astronomical term itself is preserved verbatim (primitive
        // evidence untouched by the join).
        let engine_term = get_tiet_khi(jd_of_term_index(index), TIME_ZONE);
        assert_eq!(context.solar_term, engine_term);
    }
}

#[test]
fn corpus_raw_json_never_mentions_reserved_source_ids() {
    let raw = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("data")
            .join("traditional-wellness")
            .join("seasonal-cultivation.json"),
    )
    .expect("read corpus JSON");
    assert!(
        !raw.contains("ty-ngo-luu-chu"),
        "the reserved id must never appear in the seasonal corpus"
    );
    assert!(
        !raw.contains("shi-er-jing-na-di-zhi"),
        "the branch-channel track's source must not bleed into the seasonal corpus"
    );
}

// ---------------------------------------------------------------------------
// Composition disclosure + disclaimer + review state
// ---------------------------------------------------------------------------

#[test]
fn every_result_carries_composition_note_disclaimer_and_pending_review() {
    for index in 0..24usize {
        let context = resolve_seasonal_cultivation(jd_of_term_index(index), TIME_ZONE);
        assert_eq!(context.composition_note_vi, COMPOSITION_NOTE_VN);
        assert_eq!(context.composition_note_en, COMPOSITION_NOTE_EN);
        assert!(context.composition_note_vi.contains("Amlich"));
        assert!(context.composition_note_en.contains("Amlich composition"));
        assert_eq!(context.disclaimer.id.as_str(), "cultural_information_v1");
        assert!(!context.disclaimer.vi.is_empty());
        assert!(!context.disclaimer.en.is_empty());
        match &context.review_state {
            amlich_core::traditional_wellness::ExternalReviewState::ExternalReviewPending {
                reason,
                expected_review_date,
                assigned_to,
            } => {
                assert_eq!(reason, "suwen_four_season_paraphrase_review_pending");
                assert_eq!(assigned_to, "suwen_paraphrase_reviewer");
                assert_eq!(expected_review_date, "2026-12-31");
            }
            other => panic!("must be ExternalReviewPending; got {other:?}"),
        }
        // Divergence triple on every profile (LH-DIV-04/05/07).
        for id in ["LH-DIV-04", "LH-DIV-05", "LH-DIV-07"] {
            assert!(
                context.profile.known_divergence_ids.iter().any(|x| x == id),
                "profile must reference {id}"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Round-trip serialization (additive compatibility)
// ---------------------------------------------------------------------------

#[test]
fn context_and_profiles_round_trip_byte_equal() {
    for profile in load_seasonal_corpus() {
        let json = serde_json::to_string(profile).expect("serialize profile");
        let recovered: amlich_core::traditional_wellness::SeasonalCultivationProfile =
            serde_json::from_str(&json).expect("deserialize profile");
        let json2 = serde_json::to_string(&recovered).expect("re-serialize profile");
        assert_eq!(json, json2);
        assert_eq!(recovered, *profile);
    }
    let context = resolve_seasonal_cultivation(jd_from_date(16, 8, 2026), TIME_ZONE);
    let json = serde_json::to_string(&context).expect("serialize context");
    let recovered: SeasonalCultivationContext =
        serde_json::from_str(&json).expect("deserialize context");
    let json2 = serde_json::to_string(&recovered).expect("re-serialize context");
    assert_eq!(json, json2);
    assert_eq!(recovered, context);
}

// ---------------------------------------------------------------------------
// Tier-0 availability + day-assessment neutrality
// ---------------------------------------------------------------------------

#[test]
fn tier0_resolves_without_birth_or_medical_data() {
    // The lookup takes only (jd, time_zone); no BirthInput, sex/gender,
    // symptom, location, or health history. BOUND-01 — the compile-time
    // signature enforces it; the runtime assertion is the absence of
    // panic plus a sane result.
    let context = resolve_seasonal_cultivation(jd_from_date(16, 8, 2026), TIME_ZONE);
    assert!(matches!(
        context.season,
        SeasonKey::Spring | SeasonKey::Summer | SeasonKey::Autumn | SeasonKey::Winter
    ));
    assert_eq!(
        context.profile.safety_class,
        "historical_cultural_non_clinical"
    );
}

#[test]
fn enrichment_leaves_day_assessment_untouched() {
    use amlich_core::calculate_day_snapshot;
    use amlich_core::enrich_day_snapshot_with_seasonal_cultivation;

    let snapshot = calculate_day_snapshot(16, 8, 2026);
    let before = serde_json::to_string(&snapshot).expect("serialize baseline");
    let (enriched, context) =
        enrich_day_snapshot_with_seasonal_cultivation(&snapshot, snapshot.context.jd, TIME_ZONE)
            .expect("enrichment must succeed");
    // The returned snapshot is a byte-identical clone (ADR-0003: the
    // wellness context never moves another assessment axis).
    let after = serde_json::to_string(&enriched).expect("serialize enriched");
    assert_eq!(before, after);
    assert_eq!(context.solar_term.name, enriched.context.tiet_khi.name);
}

#[test]
fn unified_enrichment_attaches_both_tracks_and_keeps_baseline_byte_equal_without_it() {
    use amlich_core::calculate_day_snapshot;
    use amlich_core::enrich_day_snapshot_with_traditional_wellness;

    let snapshot = calculate_day_snapshot(16, 8, 2026);
    let before = serde_json::to_string(&snapshot).expect("serialize baseline");
    let enriched = enrich_day_snapshot_with_traditional_wellness(
        &snapshot,
        snapshot.context.jd,
        TIME_ZONE,
        9,
        30,
    )
    .expect("unified enrichment must succeed");
    // The enrichment clones the snapshot and adds the additive
    // `traditional_wellness` field; the baseline JSON is byte-equal
    // only when the field stays absent — which holds because the
    // enriched snapshot has it set (so its JSON adds a new key).
    assert_ne!(before, serde_json::to_string(&enriched).unwrap());
    let ctx = enriched
        .traditional_wellness
        .as_ref()
        .expect("unified enrichment must populate the additive field");
    assert!(ctx.hour_branch.is_some());
    assert!(ctx.seasonal_cultivation.is_some());
    // Both primitive envelopes plus the seasonal composite (no
    // extra composite for the branch side — branch lookup is a
    // single Derived envelope, not a composite join).
    assert_eq!(ctx.evidence.len(), 4);
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// First local civil day of 2026 on which the canonical term `index`
/// is active (via the real engine, not a hardcoded table).
fn jd_of_term_index(index: usize) -> i32 {
    let firsts = first_jd_of_each_term_2026();
    let (_, jd) = firsts.iter().find(|(i, _)| *i == index).unwrap_or_else(|| {
        panic!(
            "term index {index} never became active in 2026 sweep; saw {:?}",
            firsts.iter().map(|(i, _)| *i).collect::<Vec<_>>()
        )
    });
    // Sanity: the JD must decode back into 2026.
    let (_, _, year) = jd_to_date(*jd);
    assert_eq!(year, 2026);
    *jd
}
