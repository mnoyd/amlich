//! Finite-core golden suite for the v1.11 point-opening engine (bead
//! `amlich-xlag.2.2.7`): the machine-verifiable close-out of the
//! ADR-0004 core contract.
//!
//! The committed golden matrix
//! `data/ty-ngo-luu-chu/finite-core-golden.json` is derived
//! independently of the Rust loader by `scripts/gen-tnlc-finite-core-
//! golden.py` straight from the frozen corpus JSON (bead
//! `amlich-xlag.2.1`); this suite proves the engine agrees with it:
//!
//!   1. **120-slot coverage** — both resolver entry points (Vietnamese
//!      hour-branch labels and Chinese corpus keys) return exactly one
//!      frozen record for each of the 120 day-stem × hour-branch slots.
//!   2. **Golden matrix equality + digest** — the canonical projection
//!      of every engine record (hour pillar, spillover marker, open
//!      state with table evidence / slot class / substitution / full
//!      identity triples, or explicit closed evidence) matches the
//!      committed golden entry for entry, and the canonical matrix
//!      digest is pinned; the golden header pins the corpus fingerprint
//!      so corpus and golden can never drift silently.
//!   3. **Nomenclature round trips** — every point identity triple
//!      serde round trips byte-stable, and the point_key / 穴名 /
//!      huyệt danh / code-gloss legs are each unique across all 66
//!      distinct points (NAME-01: lookup glosses, never annotations).
//!   4. **Closed-state availability** — exactly 60 slots stay
//!      explicitly closed (閉穴) with no point, substitution, or row
//!      evidence; the per-stem open/closed distribution matches the
//!      frozen counts.
//!   5. **Spillover truthfulness** — exactly 30 open rows are
//!      cross-day spillovers, every one filled from the previous day
//!      stem's table, with the three REVIEWER-PACK §A.3 boundary rows
//!      pinned; non-spillover rows always come from the cell's own
//!      day-stem table.
//!   6. **Provenance separation summary** — all 120 records stay under
//!      the reserved `ty-ngo-luu-chu` primitive source and their method
//!      evidence never cites the v1.10 Tier-0 id or the calendar
//!      engine families (exhaustive lock in
//!      `tests/point_opening_provenance.rs`).
//!   7. **Additive compatibility** — an ordinary `DaySnapshot`
//!      serializes without a `point_opening` key; enrichment adds
//!      exactly that one key and leaves every other field (including
//!      v1.10 `traditional_wellness`) untouched.
//!   8. **Extended safety guard** — every serialized surface (all 120
//!      record contexts, all 24 hourly snapshot contexts of a fixture
//!      day, and the committed golden text itself) is free of clinical
//!      field names and action/efficacy phrasing (BOUND-02).
//!
//! Boundary and civil-time goldens live in
//! `tests/point_opening_civil_time.rs`; the deep semantic-graph and
//! byte-compatibility locks live in
//! `tests/semantic_graph_point_opening_integration.rs`.

use std::collections::HashMap;

use amlich_core::almanac::types::HeavenlyStem;
use amlich_core::point_opening::{
    all_frozen_point_opening_records, frozen_point_opening_record,
    resolve_day_point_opening_context, resolve_frozen_point_opening, FrozenPointOpeningRecord,
    PointOpeningIdentity, PointOpeningSlotState,
};
use amlich_core::sources;
use amlich_core::types::CHI;
use amlich_core::{calculate_day_snapshot, enrich_day_snapshot_with_point_opening};
use serde_json::{json, Value};

const GOLDEN_JSON: &str = include_str!("../data/ty-ngo-luu-chu/finite-core-golden.json");
const CORPUS_JSON: &str = include_str!("../data/ty-ngo-luu-chu/najia-open-points.json");

const STEMS_ZH: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
const BRANCHES_ZH: [&str; 12] = [
    "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥",
];

/// FNV-1a 64 over the canonical (sorted-key, compact) serialization of
/// the engine-built 120-slot matrix. Any corpus, loader, or resolver
/// change that alters any slot breaks this pin — regeneration is the
/// documented re-freeze procedure.
const MATRIX_DIGEST_FNV1A64: u64 = 0xcec5d7231b85765c;

/// The frozen per-stem open-cell distribution (60 open total).
const OPEN_BY_STEM: [usize; 10] = [6, 7, 6, 7, 6, 7, 6, 7, 6, 2];

/// 2024-02-10 — the verified Giáp-day Julian fixture shared with the
/// civil-time and provenance suites.
const SNAPSHOT_YEAR: i32 = 2024;
const SNAPSHOT_MONTH: i32 = 2;
const SNAPSHOT_DAY: i32 = 10;

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

fn stem_index(zh: &str) -> usize {
    STEMS_ZH
        .iter()
        .position(|stem| *stem == zh)
        .unwrap_or_else(|| panic!("unknown day stem {zh}"))
}

fn branch_index(zh: &str) -> usize {
    BRANCHES_ZH
        .iter()
        .position(|branch| *branch == zh)
        .unwrap_or_else(|| panic!("unknown hour branch {zh}"))
}

/// All 120 records sorted by (day stem, hour branch) — the same order
/// the committed golden matrix is generated in.
fn sorted_records() -> Vec<&'static FrozenPointOpeningRecord> {
    let mut records: Vec<&'static FrozenPointOpeningRecord> =
        all_frozen_point_opening_records().iter().collect();
    records.sort_by_key(|record| {
        (
            stem_index(&record.day_stem_zh),
            branch_index(&record.hour_branch_zh),
        )
    });
    records
}

/// The canonical golden projection of one engine record: exactly the
/// shape `scripts/gen-tnlc-finite-core-golden.py` writes.
fn canonical_record_value(record: &FrozenPointOpeningRecord) -> Value {
    let state = match &record.context.state {
        PointOpeningSlotState::Open {
            slot_class_zh_as_printed,
            phase_annotation_as_printed,
            points,
            substitution,
        } => {
            let table = record
                .provenance
                .table_evidence
                .as_ref()
                .expect("open records carry table evidence");
            json!({
                "kind": "open",
                "resolves_to": {
                    "table": table.table_id,
                    "row_index": table.row_index,
                },
                "slot_class_zh_as_printed": slot_class_zh_as_printed,
                "phase_annotation_as_printed": phase_annotation_as_printed,
                "substitution": substitution,
                "points": points,
            })
        }
        PointOpeningSlotState::Closed {
            running_tables,
            doctrine_zh,
            note,
        } => json!({
            "kind": "closed",
            "running_tables": running_tables,
            "doctrine_zh": doctrine_zh,
            "note": note,
        }),
    };
    json!({
        "day_stem_zh": record.day_stem_zh,
        "hour_branch_zh": record.hour_branch_zh,
        "hour_pillar_zh": record.hour_pillar_zh,
        "cross_day_spillover": record.cross_day_spillover,
        "state": state,
    })
}

fn golden() -> Value {
    serde_json::from_str(GOLDEN_JSON).expect("finite-core golden parses")
}

// ---------------------------------------------------------------------------
// 1. 120-slot coverage through both entry points
// ---------------------------------------------------------------------------

#[test]
fn resolver_covers_exactly_the_120_unique_slots_through_both_entry_points() {
    let mut seen: Vec<(String, String)> = Vec::new();

    // Vietnamese hour-branch labels (the existing CHI convention).
    for (stem_index, stem) in HeavenlyStem::ALL.iter().enumerate() {
        for branch_vi in CHI {
            let record = resolve_frozen_point_opening(*stem, branch_vi)
                .unwrap_or_else(|| panic!("{stem:?}/{branch_vi} must resolve"));
            assert_eq!(record.day_stem_zh, STEMS_ZH[stem_index]);
            assert_eq!(record.hour_branch_zh, BRANCHES_ZH[seen_branch(branch_vi)]);
            seen.push((record.day_stem_zh.clone(), record.hour_branch_zh.clone()));
        }
    }
    assert_eq!(seen.len(), 120, "10 day stems x 12 hour branches");
    seen.sort();
    seen.dedup();
    assert_eq!(seen.len(), 120, "every slot must be unique");

    // Chinese corpus keys agree with the Vietnamese-label path.
    for (stem_zh, branch_zh) in &seen {
        let by_zh = frozen_point_opening_record(stem_zh, branch_zh)
            .unwrap_or_else(|| panic!("{stem_zh}/{branch_zh} must resolve by corpus keys"));
        let by_vi = resolve_frozen_point_opening(
            HeavenlyStem::ALL[stem_index(stem_zh)],
            CHI[branch_index(branch_zh)],
        )
        .expect("Vietnamese-label path resolves");
        assert_eq!(by_zh.day_stem_zh, by_vi.day_stem_zh);
        assert_eq!(by_zh.hour_branch_zh, by_vi.hour_branch_zh);
        assert_eq!(by_zh.context, by_vi.context);
        assert_eq!(by_zh.provenance, by_vi.provenance);
        assert_eq!(by_zh.hour_pillar_zh, by_vi.hour_pillar_zh);
        assert_eq!(by_zh.cross_day_spillover, by_vi.cross_day_spillover);
    }
}

fn seen_branch(branch_vi: &str) -> usize {
    CHI.iter()
        .position(|candidate| *candidate == branch_vi)
        .expect("CHI branch label")
}

// ---------------------------------------------------------------------------
// 2. Golden matrix equality, digest, and corpus lockstep
// ---------------------------------------------------------------------------

#[test]
fn engine_matrix_matches_the_committed_finite_core_golden() {
    let golden = golden();
    let matrix = golden["matrix"].as_array().expect("golden matrix");

    // Header contract.
    assert_eq!(golden["schema_version"], "tnlc_finite_core_golden_v1");
    assert_eq!(golden["bead"], "amlich-xlag.2.2.7");

    let records = sorted_records();
    assert_eq!(
        matrix.len(),
        records.len(),
        "golden and engine both hold 120 slots"
    );

    for (entry, record) in matrix.iter().zip(&records) {
        let canonical = canonical_record_value(record);
        assert_eq!(
            &canonical, entry,
            "{}/{} differs from the committed golden",
            record.day_stem_zh, record.hour_branch_zh
        );
    }
}

#[test]
fn golden_header_pins_the_frozen_counts_and_corpus_fingerprint() {
    let golden = golden();

    // The golden never drifts from the corpus it was generated from.
    assert_eq!(
        golden["source_corpus"]["path"],
        "crates/amlich-core/data/ty-ngo-luu-chu/najia-open-points.json"
    );
    let declared = u64::from_str_radix(
        golden["source_corpus"]["fnv1a64"]
            .as_str()
            .expect("hex fingerprint"),
        16,
    )
    .expect("fingerprint parses as hex");
    assert_eq!(
        declared,
        fnv1a64(CORPUS_JSON.as_bytes()),
        "the frozen corpus changed without regenerating the golden (run scripts/gen-tnlc-finite-core-golden.py)"
    );

    // Frozen distribution pins.
    assert_eq!(golden["counts"]["slots"], 120);
    assert_eq!(golden["counts"]["open"], 60);
    assert_eq!(golden["counts"]["closed"], 60);
    assert_eq!(golden["counts"]["cross_day_spillovers"], 30);
    for (index, stem) in STEMS_ZH.iter().enumerate() {
        assert_eq!(
            golden["counts"]["open_by_day_stem"][stem],
            OPEN_BY_STEM[index]
        );
    }
}

#[test]
fn canonical_matrix_digest_is_pinned() {
    let values: Vec<Value> = sorted_records()
        .iter()
        .map(|record| canonical_record_value(record))
        .collect();
    let canonical = serde_json::to_string(&Value::Array(values)).expect("matrix serializes");
    assert_eq!(
        fnv1a64(canonical.as_bytes()),
        MATRIX_DIGEST_FNV1A64,
        "the resolved 120-slot matrix digest changed; if the freeze was deliberate, regenerate the golden and re-pin"
    );
}

// ---------------------------------------------------------------------------
// 3. Nomenclature round trips
// ---------------------------------------------------------------------------

#[test]
fn every_identity_triple_round_trips_and_the_nomenclature_legs_are_unique() {
    let mut by_key: HashMap<String, PointOpeningIdentity> = HashMap::new();
    for record in all_frozen_point_opening_records() {
        let PointOpeningSlotState::Open { points, .. } = &record.context.state else {
            continue;
        };
        for identity in points {
            // Serde round trip: field-preserving and byte-stable.
            let wire = serde_json::to_string(identity).expect("identity serializes");
            let recovered: PointOpeningIdentity =
                serde_json::from_str(&wire).expect("identity parses");
            assert_eq!(recovered, *identity);
            assert_eq!(
                serde_json::to_string(&recovered).unwrap(),
                wire,
                "identity wire shape must be stable"
            );
            by_key.insert(identity.point_key.clone(), identity.clone());
        }
    }
    assert_eq!(by_key.len(), 66, "exactly the 66 frozen registry points");

    // Each identity leg is a unique lookup key across the corpus.
    for name in [
        "point_key",
        "xue_ming_zh",
        "huyet_danh_vi",
        "standard_code_gloss",
    ] {
        let mut seen: Vec<String> = by_key
            .values()
            .map(|identity| match name {
                "point_key" => identity.point_key.clone(),
                "xue_ming_zh" => identity.xue_ming_zh.clone(),
                "huyet_danh_vi" => identity.huyet_danh_vi.clone(),
                _ => identity.standard_code_gloss.clone(),
            })
            .collect();
        seen.sort();
        seen.dedup();
        assert_eq!(
            seen.len(),
            by_key.len(),
            "nomenclature leg {name} must be unique across all {} points",
            by_key.len()
        );
    }
}

// ---------------------------------------------------------------------------
// 4. Closed-state availability
// ---------------------------------------------------------------------------

#[test]
fn exactly_sixty_slots_stay_explicitly_closed_and_unavailable() {
    let records = all_frozen_point_opening_records();
    let mut closed_by_stem: HashMap<&str, usize> = HashMap::new();

    for record in records {
        if let PointOpeningSlotState::Closed {
            running_tables,
            doctrine_zh,
            note,
        } = &record.context.state
        {
            *closed_by_stem.entry(&record.day_stem_zh).or_default() += 1;
            assert!(!record.cross_day_spillover, "closed slots never spill");
            assert!(
                record.provenance.table_evidence.is_none()
                    && record.provenance.work_evidence.is_empty(),
                "{}/{} closed records carry no row evidence",
                record.day_stem_zh,
                record.hour_branch_zh
            );
            assert_eq!(running_tables.len(), 2, "closed evidence names two tables");
            assert!(!doctrine_zh.trim().is_empty() && !note.trim().is_empty());

            // The serialized closed state never carries a point.
            let state = serde_json::to_value(&record.context.state).unwrap();
            assert_eq!(state["state"], "closed");
            assert!(state.get("points").is_none());
            assert!(state.get("substitution").is_none());
        }
    }

    assert_eq!(closed_by_stem.len(), 10);
    for (index, stem) in STEMS_ZH.iter().enumerate() {
        assert_eq!(
            closed_by_stem.get(stem).copied().unwrap_or(0),
            12 - OPEN_BY_STEM[index],
            "{stem} closed-cell count"
        );
    }
}

// ---------------------------------------------------------------------------
// 5. Spillover truthfulness
// ---------------------------------------------------------------------------

#[test]
fn every_spillover_row_comes_from_the_previous_day_stems_table() {
    let records = all_frozen_point_opening_records();

    // Derive the stem → own-table mapping from the non-spillover rows.
    let mut own_table: HashMap<&str, &str> = HashMap::new();
    for record in records.iter().filter(|record| {
        !record.cross_day_spillover
            && matches!(record.context.state, PointOpeningSlotState::Open { .. })
    }) {
        let table = record.provenance.table_evidence.as_ref().unwrap();
        own_table.insert(&record.day_stem_zh, &table.table_id);
    }
    assert_eq!(own_table.len(), 10, "each day stem owns exactly one table");

    let mut spillovers = 0;
    for record in records.iter().filter(|record| record.cross_day_spillover) {
        assert!(
            matches!(record.context.state, PointOpeningSlotState::Open { .. }),
            "only open rows can be spillovers"
        );
        spillovers += 1;
        let table = record.provenance.table_evidence.as_ref().unwrap();
        let previous_stem = STEMS_ZH[(stem_index(&record.day_stem_zh) + 9) % 10];
        assert_eq!(
            table.table_id, own_table[previous_stem],
            "{}/{} spillover must be filled by the previous day stem's table",
            record.day_stem_zh, record.hour_branch_zh
        );
    }
    assert_eq!(spillovers, 30, "exactly 30 cross-day spillover rows");

    // REVIEWER-PACK §A.3 boundary exemplars.
    let yi_zi = frozen_point_opening_record("乙", "子").unwrap();
    assert_eq!(
        serde_json::to_value(&yi_zi.context.state).unwrap()["state"],
        "open"
    );
    let yi_zi_table = yi_zi.provenance.table_evidence.as_ref().unwrap();
    assert_eq!(yi_zi_table.table_id, "jia");
    assert_eq!(yi_zi_table.row_index, 2);

    let gui_zi = frozen_point_opening_record("癸", "子").unwrap();
    let gui_zi_table = gui_zi.provenance.table_evidence.as_ref().unwrap();
    assert_eq!(gui_zi_table.table_id, "ren");
    assert_eq!(gui_zi_table.row_index, 6);

    let jia_chou = frozen_point_opening_record("甲", "丑").unwrap();
    let jia_chou_table = jia_chou.provenance.table_evidence.as_ref().unwrap();
    assert_eq!(jia_chou_table.table_id, "gui");
    assert_eq!(jia_chou_table.row_index, 2);
}

// ---------------------------------------------------------------------------
// 6. Provenance separation summary
// ---------------------------------------------------------------------------

#[test]
fn all_records_stay_under_the_reserved_source_with_no_cross_citation() {
    for record in all_frozen_point_opening_records() {
        let label = format!("{}/{}", record.day_stem_zh, record.hour_branch_zh);
        assert_eq!(record.provenance.source_id, sources::SOURCE_TY_NGO_LUU_CHU);
        for envelope in record.reasoning_evidence() {
            assert_eq!(envelope.source_id, sources::SOURCE_TY_NGO_LUU_CHU);
            assert_ne!(
                envelope.source_id,
                sources::SOURCE_SHI_ER_JING_NA_DI_ZHI,
                "{label}: never the v1.10 Tier-0 id"
            );
            assert_ne!(
                envelope.source_id,
                sources::SOURCE_KHCBPPT,
                "{label}: never the hour-pillar rule source"
            );
            assert_ne!(
                envelope.source_id, "amlich-calendar-engine",
                "{label}: never the calendar engine"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// 7. Additive DaySnapshot compatibility
// ---------------------------------------------------------------------------

#[test]
fn ordinary_snapshots_stay_byte_compatible_until_point_opening_is_enriched() {
    let plain = calculate_day_snapshot(SNAPSHOT_DAY, SNAPSHOT_MONTH, SNAPSHOT_YEAR);
    let plain_value = serde_json::to_value(&plain).expect("snapshot serializes");
    let plain_object = plain_value.as_object().expect("snapshot object");
    assert!(
        !plain_object.contains_key("point_opening"),
        "un-enriched snapshots must not carry the point_opening field"
    );

    let enriched =
        enrich_day_snapshot_with_point_opening(&plain, 12, 30).expect("valid moment enriches");
    let enriched_value = serde_json::to_value(&enriched).expect("enriched serializes");
    let enriched_object = enriched_value.as_object().expect("enriched object");
    assert!(enriched_object.contains_key("point_opening"));

    // Exactly one additive key; every other field (including v1.10
    // traditional_wellness) is untouched.
    for (key, value) in plain_object {
        assert_eq!(
            enriched_object.get(key),
            Some(value),
            "enrichment must not alter the `{key}` field"
        );
    }
    assert_eq!(
        enriched_object.len(),
        plain_object.len() + 1,
        "enrichment adds exactly the point_opening key"
    );
    assert_eq!(
        plain_object.get("traditional_wellness"),
        enriched_object.get("traditional_wellness"),
        "v1.10 traditional_wellness is untouched"
    );
}

// ---------------------------------------------------------------------------
// 8. Extended safety guard (BOUND-02)
// ---------------------------------------------------------------------------

/// Clinical / technique field names forbidden anywhere in the
/// serialized surfaces (extends the contract-guard list).
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
    "needle",
    "needles",
    "needling",
    "moxa",
    "moxibustion",
    "pressure_point",
    "therapeutic",
    "therapy",
    "physiology",
];

/// Action / efficacy phrasing forbidden in every serialized surface and
/// the committed golden text. The disclaimer v2 negation frames are
/// byte-locked separately and stripped before scanning.
const FORBIDDEN_PHRASES: &[&str] = &[
    "best time to treat",
    "best hour to treat",
    "best hour",
    "should be needled",
    "should be pressed",
    "should needle",
    "should stimulate",
    "recommended point",
    "optimal point",
    "nên châm",
    "nên bấm",
    "nên cứu",
    "nên kích thích",
    "hãy châm",
    "hãy bấm",
    "điểm nên ",
    "công dụng",
    "chữa",
    "điều trị",
    "thải độc",
    "liều lượng",
    "hoạt động mạnh nhất",
    "đạt đỉnh",
];

fn collect_keys(value: &Value, out: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            for (key, child) in map {
                out.push(key.clone());
                collect_keys(child, out);
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_keys(item, out);
            }
        }
        _ => {}
    }
}

/// Remove every `disclaimer` node: its bilingual negation frames are
/// the only permitted clinical-verb contexts and are byte-locked by
/// `tests/point_opening_contract_guard.rs`.
fn strip_disclaimers(value: &mut Value) {
    if let Value::Object(map) = value {
        map.remove("disclaimer");
        for child in map.values_mut() {
            strip_disclaimers(child);
        }
    } else if let Value::Array(items) = value {
        for item in items {
            strip_disclaimers(item);
        }
    }
}

fn assert_surface_is_safe(label: &str, mut payload: Value) {
    strip_disclaimers(&mut payload);
    let mut keys = Vec::new();
    collect_keys(&payload, &mut keys);
    for key in &keys {
        assert!(
            !FORBIDDEN_KEYS.contains(&key.as_str()),
            "{label}: prohibited clinical field `{key}`"
        );
    }
    let text = payload.to_string().to_lowercase();
    for phrase in FORBIDDEN_PHRASES {
        assert!(
            !text.contains(phrase),
            "{label}: prohibited action/efficacy phrasing `{phrase}`"
        );
    }
}

#[test]
fn every_serialized_surface_is_free_of_clinical_or_action_language() {
    // All 120 frozen record contexts with their provenance.
    for record in all_frozen_point_opening_records() {
        let label = format!("record {}/{}", record.day_stem_zh, record.hour_branch_zh);
        assert_surface_is_safe(&label, serde_json::to_value(&record.context).unwrap());
        assert_surface_is_safe(&label, serde_json::to_value(&record.provenance).unwrap());
    }

    // All 24 hourly snapshot contexts of the fixture day (open and
    // closed states, both evidence vectors included).
    for hour in 0u8..=23 {
        let context = resolve_day_point_opening_context(2_460_351, hour, 30).expect("valid moment");
        assert_surface_is_safe(
            &format!("snapshot context {hour}:30"),
            serde_json::to_value(&context).unwrap(),
        );
    }
}

#[test]
fn the_committed_golden_text_carries_no_clinical_or_action_language() {
    let lowered = GOLDEN_JSON.to_lowercase();
    for phrase in FORBIDDEN_PHRASES {
        assert!(
            !lowered.contains(phrase),
            "prohibited action/efficacy phrasing `{phrase}` found in the committed golden"
        );
    }
    for lexeme in FORBIDDEN_KEYS {
        assert!(
            !lowered.contains(&format!("\"{lexeme}\"")),
            "prohibited clinical field `{lexeme}` found in the committed golden"
        );
    }
}
