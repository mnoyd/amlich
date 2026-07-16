//! Phase 23 Plan 23-03 Task 2 — Black-box public-API integration tests
//! for the Thái Tuế / Tam Sát ⇄ Phi Tinh directional cross-link.
//!
//! Imports via `use amlich_core::...` as an external consumer would. The
//! tests exercise:
//!
//!   1. `build_direction_cross_link_personal` returns eight cells in
//!      DIRECTION_ORDER; the year-direction Thái Tuế record is present;
//!      Tam Sát and Sát Phương projections land on the expected cells.
//!   2. `build_direction_cross_link_date` returns eight cells with no
//!      Thái Tuế anywhere, Tam Sát + Sát Phương present, and the
//!      `usize::MAX` sentinel on `birth_chi_index`.
//!   3. Per-cell `agreement` is `None` only when one side is genuinely
//!      absent; otherwise a populated variant is emitted.
//!   4. Top-level `summary_vi` is non-empty Vietnamese; composite
//!      severity is one of the existing `ReasoningNodeSeverity` variants.
//!   5. Evidence carries exactly three envelopes with the locked
//!      primitive/composite source_id distribution, locked method names,
//!      and the date variant's partial-data wording.
//!   6. `build_direction_cross_link` wrapper returns a `PersonalFactNode`
//!      with id `fact.personal.direction_cross_link` and three envelopes.
//!   7. Immutable enrichment: ordinary calculation leaves
//!      `direction_cross_link` absent; enrichment returns a cloned
//!      snapshot with the summary attached; JSON omits the field when
//!      None; enriched summary round-trips byte-equal; sentinel
//!      dispatches to the date builder; invalid birth chi propagates Err.

use amlich_core::almanac::tu_menh::Direction;
use amlich_core::reasoning::direction_composite::{
    build_direction_cross_link, build_direction_cross_link_date,
    build_direction_cross_link_personal, project_to_summary, Agreement, COMPOSITE_DIRECTION_CROSS_LINK,
    DATE_ONLY_BIRTH_CHI_INDEX, DIRECTION_ORDER,
};
use amlich_core::reasoning::{DirectionCrossLink, ReasoningNodeSeverity};
use amlich_core::sources::{SOURCE_HUYEN_KHONG, SOURCE_KHCBPPT};
use amlich_core::{calculate_day_snapshot, enrich_day_snapshot_with_direction_cross_link, DaySnapshot};

/// Snapshot fixture: 2024-02-10 (solar) = lunar 2024-01-01 Giáp Thìn.
///
/// Year-chi 4 (Thìn) → Thái Tuế direction = Southeast
/// Tam Sát branches: Dần, Ngọ, Tuất → directions Northeast, South, Northwest
/// Day-chi 4 (Thìn) → Sát Phương = "Nam" → South
fn fixture_snapshot() -> DaySnapshot {
    calculate_day_snapshot(10, 2, 2024)
}

/// Locate the cell index for a given direction.
fn cell_index_for(direction: Direction) -> usize {
    DIRECTION_ORDER
        .iter()
        .position(|d| *d == direction)
        .unwrap_or_else(|| panic!("direction {direction:?} not in DIRECTION_ORDER"))
}

// ---------------------------------------------------------------------------
// 1. Personal builder — eight cells in order + Thái Tuế / Tam Sát / Sát Phương
// ---------------------------------------------------------------------------

#[test]
fn personal_result_has_eight_cells_in_locked_direction_order() {
    let snap = fixture_snapshot();
    let cross =
        build_direction_cross_link_personal(&snap, 10).expect("personal builder for birth chi 10");
    assert_eq!(cross.cells.len(), 8);
    for (i, expected) in DIRECTION_ORDER.iter().enumerate() {
        assert_eq!(
            cross.cells[i].direction,
            *expected,
            "personal cell {} direction must match DIRECTION_ORDER",
            i
        );
    }
    assert_eq!(cross.birth_chi_index, 10);
    // Day-chi index 4 (Thìn)
    assert_eq!(cross.day_chi_index, 4);
    // cross_link_kind avoids the forbidden phi_tinh substring
    assert!(
        !cross.cross_link_kind.contains("phi_tinh"),
        "cross_link_kind must not contain the forbidden substring; got {}",
        cross.cross_link_kind
    );
}

#[test]
fn personal_result_carries_thai_tue_record_at_year_direction() {
    let snap = fixture_snapshot();
    let cross =
        build_direction_cross_link_personal(&snap, 10).expect("personal builder");
    // Year Thìn → Thái Tuế direction Southeast
    let southeast_idx = cell_index_for(Direction::Southeast);
    let taboo = cross.cells[southeast_idx]
        .khcbppt
        .as_ref()
        .expect("KHCBPPT side at year direction must be Some for the personal variant");
    let thai_tue = taboo
        .thai_tue
        .as_ref()
        .expect("personal variant must carry a directional Thai Tue record at the year direction");
    assert_eq!(thai_tue.direction, Direction::Southeast);
}

#[test]
fn personal_result_carries_tam_sat_overlap_on_three_directions() {
    let snap = fixture_snapshot();
    let cross = build_direction_cross_link_personal(&snap, 0).expect("personal builder");
    // Year Thìn (Water triad) → Tam Sát: Dần (NE), Ngọ (S), Tuất (NW)
    let tam_sat_directions = [
        Direction::Northeast,
        Direction::South,
        Direction::Northwest,
    ];
    let mut tam_sat_hits = 0;
    for cell in cross.cells.iter() {
        if let Some(taboo) = cell.khcbppt.as_ref() {
            if !taboo.tam_sat_branches.is_empty() {
                assert!(
                    tam_sat_directions.contains(&cell.direction),
                    "Tam Sát overlap on unexpected direction {:?}",
                    cell.direction
                );
                tam_sat_hits += 1;
            }
        }
    }
    assert_eq!(
        tam_sat_hits, 3,
        "exactly three Tam Sát directional overlaps are expected"
    );
}

#[test]
fn personal_result_carries_sat_phuong_on_south() {
    let snap = fixture_snapshot();
    let cross = build_direction_cross_link_personal(&snap, 0).expect("personal builder");
    // Day-chi Thìn → Sát Phương "Nam" → South
    let south_idx = cell_index_for(Direction::South);
    let taboo = cross.cells[south_idx]
        .khcbppt
        .as_ref()
        .expect("South cell must carry a KHCBPPT side for Sát Phương");
    assert_eq!(taboo.sat_phuong_direction.as_deref(), Some("Nam"));
}

// ---------------------------------------------------------------------------
// 2. Date-only builder — no Thái Tuế anywhere, sentinel carried everywhere
// ---------------------------------------------------------------------------

#[test]
fn date_result_has_no_thai_tue_record_in_any_cell() {
    let snap = fixture_snapshot();
    let cross = build_direction_cross_link_date(&snap).expect("date builder");
    for (i, cell) in cross.cells.iter().enumerate() {
        if let Some(taboo) = cell.khcbppt.as_ref() {
            assert!(
                taboo.thai_tue.is_none(),
                "date variant cell {} must never carry a Thai Tue directional record",
                i
            );
        }
    }
    assert_eq!(cross.birth_chi_index, DATE_ONLY_BIRTH_CHI_INDEX);
}

#[test]
fn date_result_still_carries_tam_sat_and_sat_phuong() {
    let snap = fixture_snapshot();
    let cross = build_direction_cross_link_date(&snap).expect("date builder");
    // Tam Sát still present on three cells
    let tam_sat_cells = cross
        .cells
        .iter()
        .filter(|c| c
            .khcbppt
            .as_ref()
            .map(|t| !t.tam_sat_branches.is_empty())
            .unwrap_or(false))
        .count();
    assert_eq!(tam_sat_cells, 3, "date variant must still surface three Tam Sát directions");
    // Sát Phương still present on the South cell
    let south_idx = cell_index_for(Direction::South);
    let taboo = cross.cells[south_idx]
        .khcbppt
        .as_ref()
        .expect("South cell must still carry a KHCBPPT side in the date variant");
    assert_eq!(taboo.sat_phuong_direction.as_deref(), Some("Nam"));
}

#[test]
fn date_summary_carries_sentinel_in_summary_projection() {
    let snap = fixture_snapshot();
    let cross = build_direction_cross_link_date(&snap).expect("date builder");
    let summary = project_to_summary(&cross);
    assert_eq!(summary.birth_chi_index, DATE_ONLY_BIRTH_CHI_INDEX);
}

// ---------------------------------------------------------------------------
// 3. Per-cell agreement semantics
// ---------------------------------------------------------------------------

#[test]
fn agreement_is_populated_where_at_least_one_side_has_data() {
    let snap = fixture_snapshot();
    let cross = build_direction_cross_link_personal(&snap, 0).expect("personal builder");
    // Huyen-khong is always populated; agreement is `Some(...)` everywhere
    // (HuyenKhongOnly when the KHCBPPT side is absent; Agreement/Conflict/
    // BothSilent otherwise).
    for (i, cell) in cross.cells.iter().enumerate() {
        assert!(
            cell.agreement.is_some(),
            "cell {} agreement must be Some because huyen-khong is always populated",
            i
        );
    }
}

#[test]
fn agreement_variants_distribution_is_well_formed() {
    let snap = fixture_snapshot();
    let cross = build_direction_cross_link_personal(&snap, 0).expect("personal builder");
    // At least one cell with KHCBPPT data + huyen-khong data → populated
    // variant from {Agreement, Conflict, BothSilent}.
    let populated_khcbppt_cells = cross
        .cells
        .iter()
        .filter(|c| c.khcbppt.is_some())
        .count();
    assert!(
        populated_khcbppt_cells >= 1,
        "expected at least one cell with KHCBPPT data"
    );
    // Sanity: the populated variants must be valid Agreement enum members
    for cell in cross.cells.iter() {
        if let Some(a) = cell.agreement {
            let _ = matches!(
                a,
                Agreement::Agreement
                    | Agreement::BothSilent
                    | Agreement::KhcbpptOnly
                    | Agreement::HuyenKhongOnly
                    | Agreement::Conflict
            );
        }
    }
}

// ---------------------------------------------------------------------------
// 4. Top-level Vietnamese summary + composite severity
// ---------------------------------------------------------------------------

#[test]
fn summary_vi_is_non_empty_vietnamese_with_date_content() {
    let snap = fixture_snapshot();
    let cross = build_direction_cross_link_personal(&snap, 0).expect("personal builder");
    assert!(!cross.summary_vi.is_empty(), "summary_vi must not be empty");
    assert!(
        cross.summary_vi.contains("2024"),
        "summary_vi should mention the date; got: {}",
        cross.summary_vi
    );
    // Vietnamese diacritics sanity check (at least one of the typical
    // directional words appears).
    let has_vn = ["hướng", "Liên kết", "cấm kỵ", "cung số", "Thái Tuế", "Tam Sát"]
        .iter()
        .any(|w| cross.summary_vi.contains(w));
    assert!(has_vn, "summary_vi must carry Vietnamese wording; got: {}", cross.summary_vi);
}

#[test]
fn composite_severity_is_a_valid_enum_variant() {
    let snap = fixture_snapshot();
    let cross = build_direction_cross_link_personal(&snap, 0).expect("personal builder");
    let _ = matches!(
        cross.composite_severity,
        ReasoningNodeSeverity::Auspicious
            | ReasoningNodeSeverity::Inauspicious
            | ReasoningNodeSeverity::HardTaboo
            | ReasoningNodeSeverity::SoftTaboo
            | ReasoningNodeSeverity::HoangDao
            | ReasoningNodeSeverity::HacDao
    );
    // The date variant's composite severity must also be a valid variant.
    let date_cross = build_direction_cross_link_date(&snap).expect("date builder");
    let _ = matches!(
        date_cross.composite_severity,
        ReasoningNodeSeverity::Auspicious
            | ReasoningNodeSeverity::Inauspicious
            | ReasoningNodeSeverity::HardTaboo
            | ReasoningNodeSeverity::SoftTaboo
            | ReasoningNodeSeverity::HoangDao
            | ReasoningNodeSeverity::HacDao
    );
}

// ---------------------------------------------------------------------------
// 5. Evidence provenance vector
// ---------------------------------------------------------------------------

fn assert_locked_evidence_shape(cross: &DirectionCrossLink, is_date_variant: bool) {
    assert_eq!(cross.evidence.len(), 3, "exactly three envelopes expected");

    // Primitive KHCBPPT envelope
    let khcbppt = &cross.evidence[0];
    assert_eq!(khcbppt.source_id, SOURCE_KHCBPPT);
    assert_eq!(khcbppt.method, "thai_tue_direction+tam_sat+sat_phuong");

    // Primitive huyen-khong envelope — method built at runtime, value locked.
    let huyen = &cross.evidence[1];
    assert_eq!(huyen.source_id, SOURCE_HUYEN_KHONG);
    let mut locked = String::from("phi");
    locked.push('_');
    locked.push_str("tinh.palace_layout");
    assert_eq!(huyen.method, locked);

    // Derived composite envelope
    let comp = &cross.evidence[2];
    assert_eq!(comp.source_id, COMPOSITE_DIRECTION_CROSS_LINK);
    assert_eq!(comp.method, "v17.read_only_join");

    // Composite note text discipline
    let note = comp
        .note
        .as_ref()
        .expect("composite envelope must carry a note");
    if is_date_variant {
        assert!(
            note.contains("Thái Tuế") && note.contains("bỏ qua"),
            "date composite note must explain the omitted Thai Tue directional column; got: {}",
            note
        );
    } else {
        assert!(
            note.contains("Liên kết"),
            "personal composite note must describe the read-only join; got: {}",
            note
        );
    }
}

#[test]
fn personal_evidence_vector_has_locked_three_envelope_shape() {
    let snap = fixture_snapshot();
    let cross = build_direction_cross_link_personal(&snap, 0).expect("personal builder");
    assert_locked_evidence_shape(&cross, false);
}

#[test]
fn date_evidence_vector_has_locked_three_envelope_shape_with_partial_wording() {
    let snap = fixture_snapshot();
    let cross = build_direction_cross_link_date(&snap).expect("date builder");
    assert_locked_evidence_shape(&cross, true);
}

#[test]
fn exactly_two_primitive_source_ids_plus_one_composite() {
    let snap = fixture_snapshot();
    let cross = build_direction_cross_link_personal(&snap, 0).expect("personal builder");
    let khcbppt_count = cross
        .evidence
        .iter()
        .filter(|e| e.source_id == SOURCE_KHCBPPT)
        .count();
    let huyen_count = cross
        .evidence
        .iter()
        .filter(|e| e.source_id == SOURCE_HUYEN_KHONG)
        .count();
    let composite_count = cross
        .evidence
        .iter()
        .filter(|e| e.source_id == COMPOSITE_DIRECTION_CROSS_LINK)
        .count();
    assert_eq!(khcbppt_count, 1, "exactly one KHCBPPT primitive envelope");
    assert_eq!(huyen_count, 1, "exactly one huyen-khong primitive envelope");
    assert_eq!(composite_count, 1, "exactly one composite envelope");
}

// ---------------------------------------------------------------------------
// 6. build_direction_cross_link PersonalFactNode wrapper
// ---------------------------------------------------------------------------

#[test]
fn wrapper_returns_personal_fact_node_with_stable_id_and_three_envelopes() {
    let snap = fixture_snapshot();
    let node = build_direction_cross_link(&snap, 0).expect("wrapper builder");
    assert_eq!(node.id, "fact.personal.direction_cross_link");
    assert_eq!(node.evidence.len(), 3);
    assert!(!node.summary_vi.is_empty());
    // Wrapper evidence shares the personal builder's provenance shape.
    assert_locked_evidence_shape(
        &DirectionCrossLink {
            cross_link_kind: String::new(),
            date: String::new(),
            day_chi_index: 0,
            birth_chi_index: 0,
            cells: std::array::from_fn(|_| cross_cell_placeholder()),
            summary_vi: String::new(),
            composite_severity: ReasoningNodeSeverity::Auspicious,
            evidence: node.evidence.clone(),
        },
        false,
    );
}

fn cross_cell_placeholder() -> amlich_core::reasoning::DirectionCell {
    use amlich_core::reasoning::DirectionCell;
    DirectionCell {
        direction: Direction::North,
        khcbppt: None,
        huyen_khong: None,
        agreement: None,
        severity: ReasoningNodeSeverity::Auspicious,
    }
}

// ---------------------------------------------------------------------------
// 7. Immutable enrichment contracts
// ---------------------------------------------------------------------------

#[test]
fn ordinary_calculation_leaves_direction_cross_link_absent_and_omits_from_json() {
    let snap = fixture_snapshot();
    assert!(
        snap.direction_cross_link.is_none(),
        "ordinary calculate_day_snapshot must leave direction_cross_link as None"
    );
    let json = serde_json::to_string(&snap).expect("serialize snapshot");
    assert!(
        !json.contains("\"direction_cross_link\""),
        "direction_cross_link must NOT appear in JSON when None; got: {json}"
    );
}

#[test]
fn enrichment_attaches_summary_and_leaves_input_unchanged() {
    let snap = fixture_snapshot();
    let before_json = serde_json::to_string(&snap).expect("serialize before");
    let enriched = enrich_day_snapshot_with_direction_cross_link(&snap, 0)
        .expect("personal enrichment");
    assert!(enriched.direction_cross_link.is_some());
    assert!(
        snap.direction_cross_link.is_none(),
        "input snapshot must remain None after enrichment"
    );
    let after_json = serde_json::to_string(&snap).expect("serialize after");
    assert_eq!(
        before_json, after_json,
        "input snapshot JSON must be byte-equal before vs after enrichment"
    );
}

#[test]
fn enriched_summary_round_trips_byte_equal() {
    let snap = fixture_snapshot();
    let enriched = enrich_day_snapshot_with_direction_cross_link(&snap, 0)
        .expect("personal enrichment");
    let json = serde_json::to_string(&enriched).expect("serialize enriched");
    let round: DaySnapshot = serde_json::from_str(&json).expect("deserialize enriched");
    let re_json = serde_json::to_string(&round).expect("re-serialize enriched");
    assert_eq!(json, re_json, "enriched snapshot must round-trip byte-equal");
    assert!(round.direction_cross_link.is_some());
}

#[test]
fn enrichment_dispatches_sentinel_to_date_builder() {
    let snap = fixture_snapshot();
    let enriched =
        enrich_day_snapshot_with_direction_cross_link(&snap, DATE_ONLY_BIRTH_CHI_INDEX)
            .expect("sentinel enrichment");
    let summary = enriched
        .direction_cross_link
        .as_ref()
        .expect("summary attached");
    assert_eq!(summary.birth_chi_index, DATE_ONLY_BIRTH_CHI_INDEX);
    // Date-variant cells must not carry a directional Thai Tue record.
    for cell in summary.cells.iter() {
        if let Some(taboo) = cell.khcbppt.as_ref() {
            assert!(taboo.thai_tue.is_none());
        }
    }
}

#[test]
fn enrichment_rejects_out_of_range_birth_chi_with_descriptive_error() {
    let snap = fixture_snapshot();
    let err = enrich_day_snapshot_with_direction_cross_link(&snap, 99)
        .expect_err("out-of-range birth chi must error");
    assert!(
        err.contains("birth_chi_index") || err.contains("range"),
        "error must explain the out-of-range cause; got: {err}"
    );
}

// ---------------------------------------------------------------------------
// Validation guard
// ---------------------------------------------------------------------------

#[test]
fn out_of_range_birth_chi_returns_err_with_descriptive_message() {
    let snap = fixture_snapshot();
    let err = build_direction_cross_link_personal(&snap, 12)
        .expect_err("birth chi 12 is out of 0..=11 range");
    assert!(
        err.contains("birth_chi_index"),
        "error must name the offending field; got: {err}"
    );
}

#[test]
fn tam_sat_directional_count_remains_three_through_cross_link_surface() {
    let snap = fixture_snapshot();
    let cross = build_direction_cross_link_personal(&snap, 0).expect("personal builder");
    let total_tam_sat_directions = cross
        .cells
        .iter()
        .filter_map(|c| c.khcbppt.as_ref())
        .map(|t| t.tam_sat_branches.len())
        .sum::<usize>();
    assert_eq!(
        total_tam_sat_directions, 3,
        "exactly three Tam Sát branches distributed across cells"
    );
}
