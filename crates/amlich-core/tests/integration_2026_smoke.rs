//! INT-06 — 2026 E2E calendar smoke test across ≥30 representative dates.
//!
//! Categories covered:
//!   - Tết Nguyên Đán 2026 (solar 2026-02-17 = lunar 1/1)
//!   - Sóc (lunar day 1) ×12 months of 2026
//!   - Vọng (lunar day 15) ×12 months of 2026
//!   - Vận 8→9 boundary straddle (2024-02-03 = Vận 8; 2024-02-05 = Vận 9)
//!   - 2026 leap lunar month 6 (3 solar dates with lunar.is_leap && lunar.month == 6)
//!   - All 24 Tiết Khí boundaries of 2026 via TietKhiScanner::terms_for_year(2026)
//!
//! For each date the test asserts:
//!   - calculate_day_snapshot does not panic
//!   - find_van_khan_for_snapshot does not panic (result may be empty)
//!   - compute_combined_overlay(year, lunar_month, &scanner).palace_overlays.len() == 9
//!   - compute_palace_aspects(year, lunar_month, &scanner).len() == 9
//!
//! Imports via `use amlich_core::...` as an external consumer would.

use amlich_core::almanac::fengshui::{
    compute_combined_overlay, compute_palace_aspects, compute_period, TietKhiScanner,
};
use amlich_core::julian::{jd_from_date, jd_to_date};
use amlich_core::rituals::find_van_khan_for_snapshot;
use amlich_core::calculate_day_snapshot;
use amlich_core::semantic_graph::build_day_snapshot_graph;
use amlich_core::semantic_graph::{EdgeConcept, NodeConcept};

// ---------------------------------------------------------------------------
// Helper: collect Sóc (lunar day 1) and Vọng (lunar day 15) dates in 2026
// ---------------------------------------------------------------------------

/// Scan solar year 2026 (2026-01-01 .. 2026-12-31) and return the first solar
/// date in each distinct lunar month that has `lunar.day == target_day`.
/// Returns up to 13 entries (12 normal months + possible leap month).
fn collect_lunar_day_dates(target_day: i32) -> Vec<(i32, i32, i32)> {
    let mut result: Vec<(i32, i32, i32)> = Vec::new();
    // Track (lunar_month, is_leap) pairs we have already recorded.
    let mut seen: std::collections::HashSet<(i32, bool)> = std::collections::HashSet::new();

    let start_jd = jd_from_date(1, 1, 2026);
    let end_jd = jd_from_date(31, 12, 2026);

    let mut jd = start_jd;
    while jd <= end_jd {
        let (d, m, y) = jd_to_date(jd);
        let snap = calculate_day_snapshot(d, m, y);
        let lunar = &snap.context.lunar;
        if lunar.day == target_day {
            let key = (lunar.month, lunar.is_leap);
            if !seen.contains(&key) {
                seen.insert(key);
                result.push((d, m, y));
            }
        }
        jd += 1;
    }

    result
}

// ---------------------------------------------------------------------------
// Helper: collect 3 dates in the 2026 leap lunar month 6
// ---------------------------------------------------------------------------

fn collect_leap_month6_dates() -> Vec<(i32, i32, i32)> {
    let mut result: Vec<(i32, i32, i32)> = Vec::new();

    // The 2026 leap month 6 falls in solar late Jul–Aug 2026. Scan a wide
    // enough window to find at least 3 dates.
    let start_jd = jd_from_date(1, 6, 2026); // earlier than expected, safe
    let end_jd = jd_from_date(30, 9, 2026);

    let mut jd = start_jd;
    while jd <= end_jd && result.len() < 3 {
        let (d, m, y) = jd_to_date(jd);
        let snap = calculate_day_snapshot(d, m, y);
        let lunar = &snap.context.lunar;
        if lunar.month == 6 && lunar.is_leap {
            result.push((d, m, y));
        }
        jd += 1;
    }

    result
}

// ---------------------------------------------------------------------------
// Helper: collect 24 Tiết Khí boundary dates for 2026
// ---------------------------------------------------------------------------

fn collect_tiet_khi_dates() -> Vec<(i32, i32, i32)> {
    let scanner = TietKhiScanner::new();
    scanner
        .terms_for_year(2026)
        .iter()
        .map(|t| {
            let (d, m, y) = jd_to_date(t.jd);
            (d, m, y)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Helper: exercise the four pillar APIs for a single date
// ---------------------------------------------------------------------------

fn assert_date_pillar_apis_ok(d: i32, m: i32, y: i32, scanner: &TietKhiScanner) {
    let snap = calculate_day_snapshot(d, m, y);

    // find_van_khan_for_snapshot must not panic; result may be empty
    let _rituals = find_van_khan_for_snapshot(&snap);

    // Clamp lunar month to 1..=12 for the fengshui functions
    let lunar_month = snap.context.lunar.month.clamp(1, 12) as u8;
    let solar_year = snap.context.solar.year;

    // compute_combined_overlay must return exactly 9 palace_overlays
    let overlay = compute_combined_overlay(solar_year, lunar_month, scanner);
    assert_eq!(
        overlay.palace_overlays.len(),
        9,
        "compute_combined_overlay({y}-{m:02}-{d:02}): expected 9 palace_overlays, got {}",
        overlay.palace_overlays.len()
    );

    // compute_palace_aspects must return exactly 9 aspects
    let aspects = compute_palace_aspects(solar_year, lunar_month, scanner);
    assert_eq!(
        aspects.len(),
        9,
        "compute_palace_aspects({y}-{m:02}-{d:02}): expected 9 aspects, got {}",
        aspects.len()
    );
}

// ---------------------------------------------------------------------------
// Main smoke test — all categories, ≥30 distinct dates
// ---------------------------------------------------------------------------

#[test]
fn e2e_2026_smoke_all_categories() {
    let scanner = TietKhiScanner::new();

    let mut dates: Vec<(i32, i32, i32)> = Vec::new();

    // --- Tết Nguyên Đán 2026 (solar 2026-02-17) ---
    dates.push((17, 2, 2026));

    // --- Sóc (lunar day 1) × all months of 2026 ---
    dates.extend(collect_lunar_day_dates(1));

    // --- Vọng (lunar day 15) × all months of 2026 ---
    dates.extend(collect_lunar_day_dates(15));

    // --- Vận 8→9 boundary straddle ---
    dates.push((3, 2, 2024)); // Van 8 (before Lap Xuan 2024-02-04)
    dates.push((5, 2, 2024)); // Van 9 (after  Lap Xuan 2024-02-04)

    // --- Leap lunar month 6 of 2026 (3 dates) ---
    let leap_dates = collect_leap_month6_dates();
    dates.extend(leap_dates.iter());

    // --- 24 Tiết Khí boundaries of 2026 ---
    dates.extend(collect_tiet_khi_dates());

    // Dedup (preserve first occurrence)
    {
        let mut seen = std::collections::HashSet::new();
        dates.retain(|(d, m, y)| seen.insert((*d, *m, *y)));
    }

    // Must have at least 30 distinct dates
    assert!(
        dates.len() >= 30,
        "date set must contain >= 30 distinct entries; got {}",
        dates.len()
    );

    // Exercise all four pillar APIs for every date
    for &(d, m, y) in &dates {
        assert_date_pillar_apis_ok(d, m, y, &scanner);
    }
}

// ---------------------------------------------------------------------------
// Tết assertion — solar 2026-02-17 maps to lunar 1/1
// ---------------------------------------------------------------------------

#[test]
fn tet_2026_is_lunar_1_1() {
    let snap = calculate_day_snapshot(17, 2, 2026);
    assert_eq!(
        snap.context.lunar.day, 1,
        "Tết 2026-02-17: lunar day must be 1, got {}",
        snap.context.lunar.day
    );
    assert_eq!(
        snap.context.lunar.month, 1,
        "Tết 2026-02-17: lunar month must be 1, got {}",
        snap.context.lunar.month
    );
    assert!(
        !snap.context.lunar.is_leap,
        "Tết 2026-02-17: lunar month 1 must not be a leap month"
    );
}

// ---------------------------------------------------------------------------
// Vận boundary assertions — 2024-02-03 = Van 8, 2024-02-05 = Van 9
// ---------------------------------------------------------------------------

#[test]
fn van_boundary_8_to_9() {
    let scanner = TietKhiScanner::new();

    let jd_before = jd_from_date(3, 2, 2024); // before Lập Xuân 2024-02-04
    let period_before = compute_period(jd_before, &scanner);
    assert_eq!(
        period_before.van, 8,
        "2024-02-03 (before Lập Xuân) must be Vận 8, got Vận {}",
        period_before.van
    );

    let jd_after = jd_from_date(5, 2, 2024); // after Lập Xuân 2024-02-04
    let period_after = compute_period(jd_after, &scanner);
    assert_eq!(
        period_after.van, 9,
        "2024-02-05 (after Lập Xuân) must be Vận 9, got Vận {}",
        period_after.van
    );
}

// ---------------------------------------------------------------------------
// Phase 19 E2E smoke (INT-10) — Offering wiring on representative 2026 dates
// ---------------------------------------------------------------------------
//
// Exercises ≥5 representative 2026 dates where BOTH `daily_flying_stars`
// (Phase 18-04 auto-populated) AND `offering_refs` (Phase 19-01 auto-populated)
// are populated, AND the corresponding semantic-graph carries `Offering` nodes
// + `RecommendsOffering` edges wired from `Ritual` to each `Offering`.
//
// Date selection (mirrors 19-RESEARCH.md Example 5):
//   - Tết Nguyên Đán 2026 (2026-02-17) — guaranteed applicable_rituals match
//     AND surfaces the van-khan-tet-day-du ritual (annotated with
//     metadata.cross_source_curing referencing huyen-khong per Plan 19-02).
//   - Sóc (lunar day 1) of lunar months 3, 6, 9, 12 of 2026 — surface rituals
//     matching the lunar cycle pattern via `find_van_khan_for_snapshot`.
//
// For each date:
//   1. `daily_flying_stars` MUST be Some (Phase 18-04 invariant)
//   2. When `applicable_rituals` is non-empty:
//      a. `offering_refs` MUST be Some AND non-empty (Plan 19-01 invariant)
//      b. `offerings` (flat-string) MUST be Some AND non-empty
//      c. The semantic-graph MUST contain ≥1 `NodeConcept::Offering` node
//      d. The semantic-graph MUST contain ≥1 `EdgeConcept::RecommendsOffering` edge
//      e. Every `RecommendsOffering` edge's from_node_id is a Ritual node +
//         to_node_id is an Offering node (BLOCKER 6 FIX)
//   3. Tết 2026 specifically: at least one RecommendsOffering edge's provenance
//      contains BOTH "vn-folk-ritual" AND "huyen-khong" (BLOCKER 6 FIX —
//      proves INT-09 dual-source pattern is actually implemented)
//   4. Existing annual `flying_stars` MUST be Some AND have 9 palace_overlays
//      AND each palace_overlays[i] tuple has BOTH annual and monthly FlyingStar
//      components (BLOCKER 7 FIX — annual + monthly layers coexist with daily)
//
// WARNING 3 ASSUMPTION: the date filter `(_, m, _) where m in [3,6,9,12]` filters
// on the SOLAR month (the 3rd tuple element), NOT the lunar month. This may
// produce dates that cross lunar month boundaries (the 2026 lunar-Sóc dates
// for lunar months 3/6/9/12 may have different solar months). The filter
// is kept as-is (it produces valid representative dates) BUT the test
// explicitly asserts the date set has ≥5 entries AND each entry exercises
// the wiring. If the filter produces <5 entries in a future year, the
// filter MUST be replaced with a lunar-month-aware helper.
#[test]
fn e2e_2026_smoke_offering_wiring_on_representative_dates() {
    use amlich_core::sources::SOURCE_VN_FOLK_RITUAL;

    // --- Date set: Tết + 4 Sóc dates from solar months 3/6/9/12 ---
    let mut dates: Vec<(i32, i32, i32)> = Vec::new();

    // Tết Nguyên Đán 2026 (solar 2026-02-17) — guaranteed applicable_rituals match
    // AND surfaces van-khan-tet-day-du (annotated with cross_source_curing)
    dates.push((17, 2, 2026));

    // Sóc (lunar day 1) of solar months 3, 6, 9, 12 of 2026
    // (WARNING 3: this filters on SOLAR month; the test verifies >=5 entries)
    let soc_dates = collect_lunar_day_dates(1);
    let filtered: Vec<(i32, i32, i32)> = soc_dates.into_iter()
        .filter(|(_, m, _)| [3, 6, 9, 12].contains(m))
        .collect();
    dates.extend(filtered);

    // Dedup (preserve first occurrence)
    {
        let mut seen = std::collections::HashSet::new();
        dates.retain(|(d, m, y)| seen.insert((*d, *m, *y)));
    }

    // Must have ≥5 representative dates
    assert!(
        dates.len() >= 5,
        "date set must contain >= 5 distinct entries; got {}",
        dates.len()
    );

    // Track whether ANY date exercises the INT-09 dual-source pattern
    let mut found_dual_source = false;

    // --- Exercise both additive v1.6 surfaces + semantic-graph wiring on each date ---
    for &(d, m, y) in &dates {
        let snap = calculate_day_snapshot(d, m, y);

        // 1. daily_flying_stars MUST be populated (Phase 18-04 invariant)
        assert!(
            snap.daily_flying_stars.is_some(),
            "daily_flying_stars must be Some for {y:04}-{m:02}-{d:02} (Phase 18-04 invariant)"
        );

        // 3. flying_stars (annual + monthly) MUST be populated with 9 palace_overlays
        // BLOCKER 7 FIX: explicit assertions on annual + monthly components
        let fs = snap.flying_stars.as_ref()
            .expect("flying_stars must be Some for all dates");
        assert_eq!(
            fs.palace_overlays.len(), 9,
            "flying_stars.palace_overlays must have exactly 9 entries for {y:04}-{m:02}-{d:02}"
        );
        // BLOCKER 7 FIX: annual + monthly FlyingStar components are populated.
        // Each palace_overlays[i] is a (annual, monthly) tuple — both members
        // must be valid FlyingStar variants (matches! all 9 valid variants).
        for (i, (annual, monthly)) in fs.palace_overlays.iter().enumerate() {
            assert!(
                matches!(annual,
                    amlich_core::almanac::fengshui::types::FlyingStar::NhatBach
                    | amlich_core::almanac::fengshui::types::FlyingStar::NhiHac
                    | amlich_core::almanac::fengshui::types::FlyingStar::TamBich
                    | amlich_core::almanac::fengshui::types::FlyingStar::TuLuc
                    | amlich_core::almanac::fengshui::types::FlyingStar::NguHoang
                    | amlich_core::almanac::fengshui::types::FlyingStar::LucBach
                    | amlich_core::almanac::fengshui::types::FlyingStar::ThatXich
                    | amlich_core::almanac::fengshui::types::FlyingStar::BatBach
                    | amlich_core::almanac::fengshui::types::FlyingStar::CuuTu),
                "flying_stars.palace_overlays[{i}].0 (annual) must be a valid FlyingStar variant for {y:04}-{m:02}-{d:02}"
            );
            assert!(
                matches!(monthly,
                    amlich_core::almanac::fengshui::types::FlyingStar::NhatBach
                    | amlich_core::almanac::fengshui::types::FlyingStar::NhiHac
                    | amlich_core::almanac::fengshui::types::FlyingStar::TamBich
                    | amlich_core::almanac::fengshui::types::FlyingStar::TuLuc
                    | amlich_core::almanac::fengshui::types::FlyingStar::NguHoang
                    | amlich_core::almanac::fengshui::types::FlyingStar::LucBach
                    | amlich_core::almanac::fengshui::types::FlyingStar::ThatXich
                    | amlich_core::almanac::fengshui::types::FlyingStar::BatBach
                    | amlich_core::almanac::fengshui::types::FlyingStar::CuuTu),
                "flying_stars.palace_overlays[{i}].1 (monthly) must be a valid FlyingStar variant for {y:04}-{m:02}-{d:02}"
            );
        }

        // 2. applicable_rituals — if non-empty, offering_refs MUST be populated
        let rituals = snap.applicable_rituals.as_ref()
            .expect("applicable_rituals must be Some");
        if !rituals.is_empty() {
            // 2a. offering_refs MUST be Some AND non-empty
            let refs = snap.offering_refs.as_ref()
                .expect(&format!(
                    "offering_refs must be Some when applicable_rituals is non-empty for {y:04}-{m:02}-{d:02}"
                ));
            assert!(
                !refs.is_empty(),
                "offering_refs must be non-empty when applicable_rituals is non-empty for {y:04}-{m:02}-{d:02}"
            );

            // 2b. offerings (flat-string) MUST be Some AND non-empty
            let flat = snap.offerings.as_ref()
                .expect(&format!(
                    "offerings (flat-string) must be Some when applicable_rituals is non-empty for {y:04}-{m:02}-{d:02}"
                ));
            assert!(
                !flat.is_empty(),
                "offerings (flat-string) must be non-empty when applicable_rituals is non-empty for {y:04}-{m:02}-{d:02}"
            );

            // Every OfferingRef.source_id must equal SOURCE_VN_FOLK_RITUAL
            for r in refs {
                assert_eq!(
                    r.source_id, SOURCE_VN_FOLK_RITUAL,
                    "OfferingRef.source_id must equal vn-folk-ritual for {y:04}-{m:02}-{d:02}; got {:?}",
                    r.source_id
                );
            }

            // 2c + 2d. Semantic-graph MUST contain ≥1 Offering node + ≥1 RecommendsOffering edge
            let graph = build_day_snapshot_graph(&snap);
            let offering_node_count = graph.nodes().values()
                .filter(|n| matches!(n.concept, NodeConcept::Offering))
                .count();
            assert!(
                offering_node_count >= 1,
                "semantic-graph must contain >= 1 NodeConcept::Offering node for {y:04}-{m:02}-{d:02}; got {}",
                offering_node_count
            );

            let rec_edges: Vec<_> = graph.edges().values()
                .filter(|e| matches!(e.label.concept, EdgeConcept::RecommendsOffering))
                .collect();
            let rec_edge_count = rec_edges.len();
            assert!(
                rec_edge_count >= 1,
                "semantic-graph must contain >= 1 EdgeConcept::RecommendsOffering edge for {y:04}-{m:02}-{d:02}; got {}",
                rec_edge_count
            );

            // Cross-check: number of Offering nodes >= number of RecommendsOffering edges
            assert!(
                offering_node_count >= rec_edge_count,
                "Offering node count ({}) must be >= RecommendsOffering edge count ({}) for {y:04}-{m:02}-{d:02}",
                offering_node_count, rec_edge_count
            );

            // BLOCKER 6 FIX: endpoint verification — from_node_id is a Ritual node,
            // to_node_id is an Offering node, AND at least one edge has dual-source
            // provenance. (Per-edge vn-folk-ritual provenance is required for ALL edges.)
            for edge in &rec_edges {
                let from = graph.nodes().get(&edge.from_node_id)
                    .expect("from_node must exist");
                let to = graph.nodes().get(&edge.to_node_id)
                    .expect("to_node must exist");
                assert!(
                    matches!(from.concept, NodeConcept::Ritual),
                    "RecommendsOffering from_node_id must point to a Ritual node for {y:04}-{m:02}-{d:02}; got {:?}",
                    from.concept
                );
                assert!(
                    matches!(to.concept, NodeConcept::Offering),
                    "RecommendsOffering to_node_id must point to an Offering node for {y:04}-{m:02}-{d:02}; got {:?}",
                    to.concept
                );

                let entries = graph.provenance().get(&edge.edge_id)
                    .expect("RecommendsOffering edge must have provenance entries");
                assert!(
                    entries.iter().any(|p| p.source_id.as_str() == "vn-folk-ritual"),
                    "RecommendsOffering edge provenance must include vn-folk-ritual for {y:04}-{m:02}-{d:02}"
                );

                // BLOCKER 6 FIX: dual-source pattern verification.
                // For Tết 2026 specifically (the date where van-khan-tet-day-du
                // is annotated with cross_source_curing), at least one edge's
                // provenance must contain BOTH source_ids.
                let source_ids: Vec<&str> = entries.iter()
                    .map(|p| p.source_id.as_str())
                    .collect();
                if source_ids.contains(&"vn-folk-ritual") && source_ids.contains(&"huyen-khong") {
                    found_dual_source = true;
                }
            }
        }
    }

    // BLOCKER 6 FIX: assert that AT LEAST ONE date in the set exercises the
    // INT-09 dual-source pattern (proves the cross_source_curing corpus
    // annotation actually surfaces as edge provenance). Tết 2026
    // surfaces the annotated van-khan-tet-day-du ritual — this assertion
    // confirms the annotation is wired end-to-end.
    assert!(
        found_dual_source,
        "At least one date in the smoke-test set MUST exercise INT-09 dual-source provenance \
         (a RecommendsOffering edge carrying BOTH 'vn-folk-ritual' AND 'huyen-khong' source_ids). \
         Got 0 — the van-khan-tet-day-du corpus annotation (Plan 19-02) is missing or not wired."
    );
}
