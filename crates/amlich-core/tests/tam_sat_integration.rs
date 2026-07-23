//! XLK-02 — Classical Tam Sát (三殺) directional module integration tests.
//!
//! Phase 23 Plan 23-01 Task 2 ships a new sibling `almanac::tam_sat` module
//! that returns the classical three-direction Tam Sát for every year-chi,
//! derived from the lục-xung opposite Tam Hợp triad (mirrors `tam_tai.rs`
//! `TAI_YEARS` precedent).
//!
//! This test module exercises the **public** black-box API via the external
//! crate path. It verifies:
//!
//!   1. The four locked year-triad → Tam Sát rows from CONTEXT.md
//!      §"Tam Sát triad → 3-direction mapping".
//!   2. All 12 year-chi indexes are covered (each maps to exactly one row).
//!   3. Exactly three directions + three opposite-triad branches are returned.
//!   4. The evidence carries `SOURCE_KHCBPPT` and a pending-review marker
//!      referencing the discoverable `data/almanac/tam_sat_provenance.md`
//!      artifact (PendingExternalReview for exact page citation).
//!   5. The existing `get_sat_phuong` day-chi API remains unchanged (1
//!      direction per chi, untouched values).
//!
//! Imports via `use amlich_core::...` as an external consumer would.

use amlich_core::almanac::sat_phuong::get_sat_phuong;
use amlich_core::almanac::tam_sat::{tam_sat_direction, TamSatDirectionResult};
use amlich_core::almanac::tu_menh::Direction;
use amlich_core::sources::SOURCE_KHCBPPT;

/// Tradition-ordered Tam Hợp triads as locked in CONTEXT.md.
/// Each tuple is (triad_branches, opposite_triad_branches, opposite_directions).
const EXPECTED_ROWS: &[(&[&str; 3], &[&str; 3], &[Direction; 3])] = &[
    // Water triad: Thân, Tý, Thìn → Tam Sát: Dần, Ngọ, Tuất
    (
        &["Thân", "Tý", "Thìn"],
        &["Dần", "Ngọ", "Tuất"],
        &[Direction::Northeast, Direction::South, Direction::Northwest],
    ),
    // Wood triad: Hợi, Mão, Mùi → Tam Sát: Tỵ, Dậu, Sửu
    (
        &["Hợi", "Mão", "Mùi"],
        &["Tỵ", "Dậu", "Sửu"],
        &[Direction::Southeast, Direction::West, Direction::Northeast],
    ),
    // Fire triad: Dần, Ngọ, Tuất → Tam Sát: Thân, Tý, Thìn
    (
        &["Dần", "Ngọ", "Tuất"],
        &["Thân", "Tý", "Thìn"],
        &[Direction::Southwest, Direction::North, Direction::Southeast],
    ),
    // Metal triad: Tỵ, Dậu, Sửu → Tam Sát: Hợi, Mão, Mùi
    (
        &["Tỵ", "Dậu", "Sửu"],
        &["Hợi", "Mão", "Mùi"],
        &[Direction::Northwest, Direction::East, Direction::Southwest],
    ),
];
/// Map a year-chi name to its index in the locked table.
fn chi_index(name: &str) -> usize {
    const CHI: [&str; 12] = [
        "Tý", "Sửu", "Dần", "Mão", "Thìn", "Tỵ", "Ngọ", "Mùi", "Thân", "Dậu", "Tuất", "Hợi",
    ];
    CHI.iter()
        .position(|c| *c == name)
        .unwrap_or_else(|| panic!("unknown chi: {name}"))
}

// ---------------------------------------------------------------------------
// Locked-row tests — the four tradition-ordered mappings from CONTEXT.md
// ---------------------------------------------------------------------------

#[test]
fn water_triad_year_returns_dan_ngo_tuat_directions() {
    // Year-chi in Water triad (Thân, Tý, Thìn) → Tam Sát branches Dần, Ngọ, Tuất
    // → directions Đông Bắc, Nam, Tây Bắc.
    for &year_chi_name in &["Thân", "Tý", "Thìn"] {
        let r = tam_sat_direction(chi_index(year_chi_name));
        assert_eq!(
            r.tam_hop_group.as_slice(),
            &EXPECTED_ROWS[0].0[..],
            "year {}: tam_hop_group must be tradition-ordered Water triad",
            year_chi_name,
        );
        assert_eq!(
            r.tam_sat_branches.as_slice(),
            &EXPECTED_ROWS[0].1[..],
            "year {}: tam_sat_branches must be the opposite (Fire) triad in tradition order",
            year_chi_name,
        );
        assert_eq!(
            r.tam_sat_directions, *EXPECTED_ROWS[0].2,
            "year {}: tam_sat_directions must be [NE, S, NW]",
            year_chi_name,
        );
    }
}

#[test]
fn wood_triad_year_returns_ty_dau_suu_directions() {
    for &year_chi_name in &["Hợi", "Mão", "Mùi"] {
        let r = tam_sat_direction(chi_index(year_chi_name));
        assert_eq!(r.tam_hop_group.as_slice(), &EXPECTED_ROWS[1].0[..]);
        assert_eq!(r.tam_sat_branches.as_slice(), &EXPECTED_ROWS[1].1[..]);
        assert_eq!(
            r.tam_sat_directions, *EXPECTED_ROWS[1].2,
            "year {}: tam_sat_directions must be [SE, W, NE]",
            year_chi_name,
        );
    }
}

#[test]
fn fire_triad_year_returns_than_ty_thin_directions() {
    for &year_chi_name in &["Dần", "Ngọ", "Tuất"] {
        let r = tam_sat_direction(chi_index(year_chi_name));
        assert_eq!(r.tam_hop_group.as_slice(), &EXPECTED_ROWS[2].0[..]);
        assert_eq!(r.tam_sat_branches.as_slice(), &EXPECTED_ROWS[2].1[..]);
        assert_eq!(
            r.tam_sat_directions, *EXPECTED_ROWS[2].2,
            "year {}: tam_sat_directions must be [SW, N, SE]",
            year_chi_name,
        );
    }
}

#[test]
fn metal_triad_year_returns_hoi_mao_mui_directions() {
    for &year_chi_name in &["Tỵ", "Dậu", "Sửu"] {
        let r = tam_sat_direction(chi_index(year_chi_name));
        assert_eq!(r.tam_hop_group.as_slice(), &EXPECTED_ROWS[3].0[..]);
        assert_eq!(r.tam_sat_branches.as_slice(), &EXPECTED_ROWS[3].1[..]);
        assert_eq!(
            r.tam_sat_directions, *EXPECTED_ROWS[3].2,
            "year {}: tam_sat_directions must be [NW, E, SW]",
            year_chi_name,
        );
    }
}

// ---------------------------------------------------------------------------
// All-12 coverage + exactly-three contract
// ---------------------------------------------------------------------------

#[test]
fn all_12_year_branches_covered_and_return_exactly_three_directions() {
    for i in 0..12usize {
        let r = tam_sat_direction(i);
        assert_eq!(r.year_chi_index, i, "year_chi_index round-trip");
        assert_eq!(
            r.tam_hop_group.len(),
            3,
            "Tam Hợp group must have exactly 3 branches"
        );
        assert_eq!(
            r.tam_sat_branches.len(),
            3,
            "Tam Sát branches must have exactly 3 entries"
        );
        assert_eq!(
            r.tam_sat_directions.len(),
            3,
            "Tam Sát directions must have exactly 3 entries (classical three-direction contract)"
        );
        // Tam Sát branches must NOT overlap with the Tam Hợp group (opposite triad).
        for sat_branch in r.tam_sat_branches.iter() {
            assert!(
                !r.tam_hop_group.contains(sat_branch),
                "Tam Sát branch {} must not overlap with Tam Hợp group {:?}",
                sat_branch,
                r.tam_hop_group,
            );
        }
    }
}

#[test]
fn same_triad_members_return_identical_tam_sat() {
    // All 3 branches of one Tam Hợp triad share the same Tam Sát row.
    let than_r = tam_sat_direction(chi_index("Thân"));
    let ty_r = tam_sat_direction(chi_index("Tý"));
    let thin_r = tam_sat_direction(chi_index("Thìn"));
    assert_eq!(than_r.tam_sat_branches, ty_r.tam_sat_branches);
    assert_eq!(ty_r.tam_sat_branches, thin_r.tam_sat_branches);
    assert_eq!(than_r.tam_sat_directions, ty_r.tam_sat_directions);
    assert_eq!(ty_r.tam_sat_directions, thin_r.tam_sat_directions);
}

// ---------------------------------------------------------------------------
// Evidence + provenance deferral
// ---------------------------------------------------------------------------

#[test]
fn evidence_carries_khcbppt_source_id_and_pending_review_marker() {
    let r = tam_sat_direction(chi_index("Tý"));
    assert_eq!(
        r.evidence.source_id, SOURCE_KHCBPPT,
        "Tam Sát evidence.source_id must equal SOURCE_KHCBPPT"
    );
    // The method/profile text must reference the discoverable provenance
    // artifact AND carry an explicit PendingExternalReview marker — we do
    // NOT invent an exact KHCBPPT page citation.
    let combined = format!("{} | {}", r.evidence.method, r.evidence.profile);
    let combined_lower = combined.to_lowercase();
    assert!(
        combined_lower.contains("tam_sat_provenance")
            || combined_lower.contains("data/almanac/tam_sat_provenance"),
        "evidence method/profile must reference the discoverable provenance artifact; got: {combined}",
    );
    assert!(
        combined_lower.contains("pendingexternalreview")
            || combined_lower.contains("pending"),
        "evidence method/profile must explicitly mark the exact page citation as pending review; got: {combined}",
    );
}

// ---------------------------------------------------------------------------
// sat_phuong.rs day-chi API remains unchanged (regression guard)
// ---------------------------------------------------------------------------

#[test]
fn get_sat_phuong_day_chi_values_remain_unchanged() {
    // The existing one-direction day-chi Sát Phương API must remain intact.
    // (XLK-01 backfilled its evidence but did not change the day-chi mapping.)
    assert_eq!(get_sat_phuong(0).direction, "Nam"); // Tý
    assert_eq!(get_sat_phuong(3).direction, "Tây"); // Mão
    assert_eq!(get_sat_phuong(6).direction, "Bắc"); // Ngọ
    assert_eq!(get_sat_phuong(9).direction, "Đông"); // Dậu
}

#[test]
fn tam_sat_result_is_distinct_module_from_sat_phuong() {
    // Sanity: Tam Sát is a 3-direction year-chi API; Sát Phương is a
    // 1-direction day-chi API. They share no struct shape. For year=Tý,
    // Tam Sát = [NE, S, NW] (opposite triad Dần/Ngọ/Tuất) while Sát Phương
    // for day=Tý is "Nam" — the two APIs return different cardinalities
    // and are invoked on different chi axes (year vs day).
    let tam_sat = tam_sat_direction(chi_index("Tý"));
    let sat_phuong = get_sat_phuong(chi_index("Tý"));
    assert_eq!(
        tam_sat.tam_sat_directions.len(),
        3,
        "Tam Sát returns exactly 3 directions (classical three-direction contract)"
    );
    assert_eq!(
        sat_phuong.direction.len(),
        3,
        "Sát Phương direction string for Tý is the 3-char VN word 'Nam'"
    );
    assert_eq!(sat_phuong.direction, "Nam");
    // Tam Sát for Tý year includes Ngọ→South; Sát Phương for Tý day also
    // returns South — but the two APIs operate on different chi axes
    // (year vs day) and return different shapes. They are sibling APIs,
    // not duplicates.
    assert!(
        tam_sat
            .tam_sat_directions
            .iter()
            .any(|d| matches!(d, Direction::South)),
        "Tam Sát for Tý year must include South (Ngọ → South) per the locked mapping"
    );
}

// ---------------------------------------------------------------------------
// TamSatDirectionResult struct shape (public API surface contract)
// ---------------------------------------------------------------------------

#[test]
fn tam_sat_result_round_trips_serde() {
    let r = tam_sat_direction(chi_index("Mão"));
    let json = serde_json::to_string(&r).expect("serialize TamSatDirectionResult");
    let recovered: TamSatDirectionResult =
        serde_json::from_str(&json).expect("deserialize TamSatDirectionResult");
    assert_eq!(
        recovered, r,
        "TamSatDirectionResult must round-trip byte-equal"
    );
    assert_eq!(recovered.year_chi, "Mão");
}
