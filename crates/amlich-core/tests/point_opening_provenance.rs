//! CI guard for the v1.11 point-opening provenance emission (bead
//! `amlich-xlag.2.2.5`): `crates/amlich-core/src/point_opening/provenance.rs`.
//!
//! Freezes the provenance side of ADR-0004:
//!
//!   1. **Source separation** — method evidence always cites the
//!      reserved `ty-ngo-luu-chu` primitive source and never the
//!      v1.10 Tier-0 `shi-er-jing-na-di-zhi` id; calendar and pillar
//!      evidence keeps its existing engine source families and never
//!      cites the TNLC primitive source (both directions locked).
//!   2. **Pending-state propagation** — until Gates 1–2 sign, every
//!      emitted record carries the ExternalReviewPending row and
//!      nomenclature review states, disclaimer v2, the
//!      `historical_procedural_citation` safety class, the disclosed
//!      time basis, and applicable `TNLC-DIV-*` divergences.
//!   3. **Disclaimer identity** — the context disclaimer is
//!      byte-identical to disclaimer v2 for every record.
//!   4. **Evidence round trips** — provenance blocks, work
//!      citations, and real corpus contexts round trip with every
//!      field preserved and a stable wire shape.
//!   5. **Per-row work and table evidence** — every open record
//!      carries the referenced row's complete `sources` citations and
//!      table identity, re-derived independently from the frozen
//!      corpus JSON.

use amlich_core::point_opening::{
    all_frozen_point_opening_records, resolve_frozen_point_opening_at_local_civil_time,
    tnlc_divergence_by_id, PointOpeningContext, PointOpeningProvenance,
    DISCLAIMER_HISTORICAL_PROCEDURAL_CITATION_EN, DISCLAIMER_HISTORICAL_PROCEDURAL_CITATION_VN,
    SAFETY_CLASS_HISTORICAL_PROCEDURAL_CITATION,
};
use amlich_core::sources;
use amlich_core::traditional_wellness::divergence::{ExternalReviewState, TimeBasis};

const CORPUS_JSON: &str = include_str!("../data/ty-ngo-luu-chu/najia-open-points.json");

/// 2024-02-10, the verified Giáp-day Julian fixture from `canchi.rs`.
const JD_GIAP_DAY: i32 = 2460351;

// ---------------------------------------------------------------------------
// 1. Source separation
// ---------------------------------------------------------------------------

#[test]
fn method_evidence_cites_the_primitive_source_for_all_120_slots() {
    let records = all_frozen_point_opening_records();
    assert_eq!(records.len(), 120);
    for record in records {
        let entries = record.provenance_entries();
        let envelopes = record.reasoning_evidence();
        assert_eq!(entries.len(), envelopes.len());
        assert!(
            !entries.is_empty(),
            "{}/{} must emit method evidence",
            record.day_stem_zh,
            record.hour_branch_zh
        );
        for entry in &entries {
            assert_eq!(
                entry.source_id,
                sources::SOURCE_TY_NGO_LUU_CHU,
                "{}/{} method evidence must cite the reserved primitive source",
                record.day_stem_zh,
                record.hour_branch_zh
            );
            assert_ne!(
                entry.source_id,
                sources::SOURCE_SHI_ER_JING_NA_DI_ZHI,
                "{}/{} method evidence must never cite the v1.10 Tier-0 id",
                record.day_stem_zh,
                record.hour_branch_zh
            );
            assert_eq!(
                entry.method,
                format!(
                    "point_opening_lookup:{}/{}",
                    record.day_stem_zh, record.hour_branch_zh
                )
            );
        }
    }
}

#[test]
fn calendar_evidence_keeps_existing_engine_source_families() {
    for (hour, minute) in [(0u8, 30u8), (9, 0), (12, 30), (22, 59), (23, 30)] {
        let result = resolve_frozen_point_opening_at_local_civil_time(JD_GIAP_DAY, hour, minute)
            .expect("valid civil moment must resolve");
        let evidence = result.calendar_evidence();
        assert_eq!(evidence.len(), 2, "{hour}:{minute:02}");

        // Day pillar: the existing calendar engine, Snapshot family.
        assert_eq!(evidence[0].source_id, "amlich-calendar-engine");
        assert_eq!(evidence[0].method, "get_day_canchi");
        assert_eq!(
            evidence[0].source_family,
            amlich_core::reasoning::ReasoningEvidenceSourceFamily::Snapshot
        );

        // Hour branch: the existing khcbppt hour-pillar seed-table rule.
        assert_eq!(evidence[1].source_id, sources::SOURCE_KHCBPPT);
        assert_eq!(evidence[1].method, "hour-pillar-seed-table");
        assert_eq!(
            evidence[1].source_family,
            amlich_core::reasoning::ReasoningEvidenceSourceFamily::AlmanacRule
        );

        // Separation, both directions: calendar evidence never cites
        // the TNLC primitive source; method evidence never cites the
        // calendar-engine sources.
        for envelope in &evidence {
            assert_ne!(
                envelope.source_id,
                sources::SOURCE_TY_NGO_LUU_CHU,
                "calendar evidence must never cite the TNLC primitive source"
            );
        }
        for envelope in result.record.reasoning_evidence() {
            assert_ne!(
                envelope.source_id,
                sources::SOURCE_KHCBPPT,
                "method evidence must never cite the hour-pillar rule source"
            );
            assert_ne!(
                envelope.source_id, "amlich-calendar-engine",
                "method evidence must never cite the calendar engine"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// 2. Per-row work and table evidence, independently re-derived
// ---------------------------------------------------------------------------

#[test]
fn open_records_carry_per_row_work_and_table_evidence_matching_the_corpus() {
    use std::collections::HashMap;

    let corpus: serde_json::Value = serde_json::from_str(CORPUS_JSON).expect("corpus parses");

    // Re-derive the expected per-slot evidence from the raw JSON: each
    // open grid cell's resolves_to names the owning table and row.
    let mut tables: HashMap<String, Vec<serde_json::Value>> = HashMap::new();
    for table in corpus["day_tables"].as_array().unwrap() {
        tables.insert(
            table["table_id"].as_str().unwrap().to_string(),
            table["rows"].as_array().unwrap().clone(),
        );
    }
    let mut expected: HashMap<(String, String), (String, serde_json::Value)> = HashMap::new();
    for cell in corpus["grid"].as_array().unwrap() {
        if cell["state"] != "open" {
            continue;
        }
        let reference = &cell["resolves_to"];
        let table_id = reference["table"].as_str().unwrap().to_string();
        let row = &tables[&table_id][(reference["row_index"].as_u64().unwrap() - 1) as usize];
        expected.insert(
            (
                cell["day_stem_zh"].as_str().unwrap().to_string(),
                cell["hour_branch_zh"].as_str().unwrap().to_string(),
            ),
            (table_id, row.clone()),
        );
    }

    let records = all_frozen_point_opening_records();
    let open_records = records.iter().filter(|r| {
        matches!(
            r.context.state,
            amlich_core::point_opening::PointOpeningSlotState::Open { .. }
        )
    });
    let mut open_count = 0;
    for record in open_records {
        open_count += 1;
        let key = (record.day_stem_zh.clone(), record.hour_branch_zh.clone());
        let (expected_table_id, row) = &expected[&key];

        // Table evidence matches the corpus resolves_to.
        let table = record
            .provenance
            .table_evidence
            .as_ref()
            .expect("open records carry table evidence");
        assert_eq!(&table.table_id, expected_table_id, "{key:?}");
        assert_eq!(
            table.row_index,
            row_index_of(&tables[expected_table_id], row),
            "{key:?}"
        );
        assert!(!record.provenance.work_evidence.is_empty(), "{key:?}");

        // Work evidence matches the row's validated sources verbatim.
        let sources = row["sources"].as_array().unwrap();
        assert_eq!(
            record.provenance.work_evidence.len(),
            sources.len(),
            "{key:?}"
        );
        for (citation, source) in record.provenance.work_evidence.iter().zip(sources) {
            assert_eq!(citation.source_id, source["source_id"].as_str().unwrap());
            assert_eq!(citation.work_title, source["work_title"].as_str().unwrap());
            assert_eq!(
                citation.volume_or_chapter,
                source["volume_or_chapter"].as_str().unwrap()
            );
            assert_eq!(
                citation.passage_key,
                source["passage_key"].as_str().unwrap()
            );
            assert_eq!(
                citation.edition_or_facsimile_uri,
                source["edition_or_facsimile_uri"].as_str().unwrap()
            );
            assert_eq!(
                citation.transcription_uri,
                source["transcription_uri"].as_str().unwrap()
            );
            assert_eq!(
                citation.cross_reference_uri,
                source["cross_reference_uri"].as_str().unwrap()
            );
            assert_eq!(
                citation.translation_kind,
                source["translation_kind"].as_str().unwrap()
            );
        }
    }
    assert_eq!(open_count, 60, "exactly 60 open records carry row evidence");

    // Closed records keep explicit unavailability without row evidence.
    for record in records.iter().filter(|r| {
        matches!(
            r.context.state,
            amlich_core::point_opening::PointOpeningSlotState::Closed { .. }
        )
    }) {
        assert!(
            record.provenance.table_evidence.is_none(),
            "{}/{} closed records carry no table evidence",
            record.day_stem_zh,
            record.hour_branch_zh
        );
        assert!(record.provenance.work_evidence.is_empty());
        assert_eq!(
            record.provenance_entries().len(),
            1,
            "closed records emit exactly one corpus-level method entry"
        );
    }
}

/// Recover the 1-based row index of `row` within its table rows.
fn row_index_of(rows: &[serde_json::Value], row: &serde_json::Value) -> usize {
    rows.iter()
        .position(|candidate| candidate == row)
        .map(|index| index + 1)
        .expect("row must belong to its table")
}

// ---------------------------------------------------------------------------
// 3. Pending-state propagation + disclaimer identity
// ---------------------------------------------------------------------------

#[test]
fn every_record_stays_pending_with_disclaimer_v2_safety_class_and_divergences() {
    for record in all_frozen_point_opening_records() {
        let label = format!("{}/{}", record.day_stem_zh, record.hour_branch_zh);
        let context = &record.context;

        // Gate 1 (rows) and Gate 2 (nomenclature) both unsigned.
        assert!(
            matches!(
                context.review_state,
                ExternalReviewState::ExternalReviewPending { .. }
            ),
            "{label}: row review state must stay pending until Gate 1 signs"
        );
        assert!(
            matches!(
                context.nomenclature_review_state,
                ExternalReviewState::ExternalReviewPending { .. }
            ),
            "{label}: nomenclature review state must stay pending until Gate 2 signs"
        );

        // Disclaimer identity: byte-identical to disclaimer v2.
        assert_eq!(
            context.disclaimer.vi, DISCLAIMER_HISTORICAL_PROCEDURAL_CITATION_VN,
            "{label}: disclaimer v2 Vietnamese payload must be byte-identical"
        );
        assert_eq!(
            context.disclaimer.en, DISCLAIMER_HISTORICAL_PROCEDURAL_CITATION_EN,
            "{label}: disclaimer v2 English payload must be byte-identical"
        );
        assert_eq!(
            context.disclaimer.id.as_str(),
            "historical_procedural_citation_v1"
        );

        // Safety class, time basis, and applicable divergences ride along.
        assert_eq!(
            context.safety_class,
            SAFETY_CLASS_HISTORICAL_PROCEDURAL_CITATION
        );
        assert_eq!(context.time_basis, TimeBasis::LocalCivilHourBranch);
        assert!(!context.known_divergence_ids.is_empty());
        assert!(
            context
                .known_divergence_ids
                .contains(&"TNLC-DIV-03".to_string()),
            "{label}: grid cells always disclose the time-basis divergence"
        );
        for id in &context.known_divergence_ids {
            assert!(
                tnlc_divergence_by_id(id).is_some(),
                "{label}: divergence {id} must resolve in the registry"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// 4. Evidence round trips
// ---------------------------------------------------------------------------

#[test]
fn provenance_blocks_and_contexts_round_trip_from_real_corpus_records() {
    let records = all_frozen_point_opening_records();
    let open = records
        .iter()
        .find(|r| {
            matches!(
                r.context.state,
                amlich_core::point_opening::PointOpeningSlotState::Open { .. }
            )
        })
        .expect("an open record exists");
    let closed = records
        .iter()
        .find(|r| {
            matches!(
                r.context.state,
                amlich_core::point_opening::PointOpeningSlotState::Closed { .. }
            )
        })
        .expect("a closed record exists");

    for record in [open, closed] {
        // Provenance block round trip with a stable wire shape.
        let original: PointOpeningProvenance = record.provenance.clone();
        let json = serde_json::to_string(&original).expect("provenance serializes");
        let recovered: PointOpeningProvenance =
            serde_json::from_str(&json).expect("provenance parses");
        assert_eq!(recovered, original);
        assert_eq!(
            serde_json::to_string(&recovered).unwrap(),
            json,
            "provenance wire shape must be stable"
        );

        // Disclosure-tuple round trip: review state, disclaimer, time
        // basis, safety class, and divergences.
        let original_context: PointOpeningContext = record.context.clone();
        let json = serde_json::to_string(&original_context).expect("context serializes");
        let recovered: PointOpeningContext = serde_json::from_str(&json).expect("context parses");
        assert_eq!(recovered, original_context);
        assert_eq!(
            serde_json::to_string(&recovered).unwrap(),
            json,
            "context wire shape must be stable"
        );
        assert_eq!(recovered.disclaimer.vi, original_context.disclaimer.vi);
        assert_eq!(recovered.safety_class, original_context.safety_class);
        assert_eq!(recovered.time_basis, original_context.time_basis);
        assert_eq!(
            recovered.known_divergence_ids,
            original_context.known_divergence_ids
        );
    }
}
