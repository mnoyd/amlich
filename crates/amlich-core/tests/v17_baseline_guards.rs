//! Phase 25 baseline guards: lock the v1.6 cargo dependency tree shape (SC4)
//! and the Mai Hoa golden dataset's cross-source discipline (SC1) as
//! runtime-invariant tests.
//!
//! These guards ensure the v1.7 milestone's locked contracts cannot silently
//! drift in v1.8+. The file is purely additive — it touches NO production
//! source code, adds NO new dependencies, and reads only from
//! `amlich_core`'s public API + the std library.
//!
//! ## Success criteria covered
//!
//! - **SC4** (`cargo_dependency_tree_unchanged_from_v16`): parses
//!   `crates/amlich-core/Cargo.toml` via `include_str!` and asserts the
//!   `[dependencies]` section contains EXACTLY four entries with the locked
//!   names `serde`, `serde_json`, `chrono`, `unicode-normalization`. Locks
//!   the SET of production deps (declaration order is not enforced).
//!
//! - **SC1** (`int13_golden_dataset_cross_source_discipline_holds`): loads
//!   `mai_hoa_golden.json` via `load_mai_hoa_golden()` and re-asserts
//!   INT-13's full cross-source discipline holistically (≥10 cases, every
//!   case has ≥2 sources with `nhantu.net` present in ≥1, ≥1
//!   KnownDivergence row with non-empty fields, schema pinned to
//!   `mai-hoa-golden-v1`). The Phase 22-02 loader already validates these
//!   invariants at load time; this test is the formal Phase 25 closure
//!   sentinel so a future weakening of the loader would still trip this
//!   test.

use amlich_core::iching::load_mai_hoa_golden;

// ---------------------------------------------------------------------------
// Test 1: SC4 — cargo dependency tree unchanged from v1.6 baseline
// ---------------------------------------------------------------------------

/// Lock the production `[dependencies]` section of
/// `crates/amlich-core/Cargo.toml` at exactly four entries: `serde`,
/// `serde_json`, `chrono`, `unicode-normalization`.
///
/// ## v1.6 baseline reference
///
/// The 922-test v1.6 milestone (shipped 2026-07-16) landed with this exact
/// 4-dep tree: `chrono` + `serde` + `serde_json` + `unicode-normalization`.
/// v1.7 added NO new production deps despite shipping:
/// - 448-text-field IChing corpus (`hexagrams.json`)
/// - Mai Hoa casting (`cast_mai_hoa`) + biến quẻ derivation + Thể/Dụng
///   classification (`classify_the_dung`)
/// - `IChingEvaluator` + `IChingQuery` + `IChingCastSummary` DTO
/// - Thái Tuế / Tam Sát ⇄ Phi Tinh read-only directional cross-link
/// - Semantic-graph wiring (`DaySnapshotGraphBuilder::add_iching_facts` +
///   `add_direction_composite_facts`)
/// - Additive `DaySnapshot.iching_cast` + `direction_cross_link` fields +
///   v1.6→v1.7 round-trip
///
/// This test pins the dep tree shape so a future v1.8+ regression that adds
/// a runtime dep will fail CI. The test does NOT invoke `cargo tree` at
/// test time (slow + brittle across cargo versions) — the Cargo.toml-parse
/// approach is faster and locks the same invariant.
#[test]
fn cargo_dependency_tree_unchanged_from_v16() {
    // Embed `crates/amlich-core/Cargo.toml` at compile time. The test file
    // lives at `crates/amlich-core/tests/`, so `../Cargo.toml` resolves to
    // `crates/amlich-core/Cargo.toml`.
    const CARGO_TOML: &str = include_str!("../Cargo.toml");

    // Locate the `[dependencies]` section. We look for the `\n[dependencies]`
    // marker (newline-prefixed to avoid matching nested-table names like
    // `[dev-dependencies]` or `package.dependencies`).
    let deps_marker = "\n[dependencies]";
    let deps_start = CARGO_TOML
        .find(deps_marker)
        .expect("Cargo.toml shape changed: [dependencies] section not found")
        + deps_marker.len();

    // Find the next `\n[` after the deps section start — that's the start
    // of the next section (e.g. `[dev-dependencies]`).
    let rest_after_deps_start = &CARGO_TOML[deps_start..];
    let next_section_offset = rest_after_deps_start
        .find("\n[")
        .expect("Cargo.toml shape changed: unterminated [dependencies] section");
    let deps_section = &rest_after_deps_start[..next_section_offset];

    // Parse the deps section: one entry per non-blank, non-comment, non-section
    // line. The dependency name is the substring before the first `=`.
    let mut names: Vec<&str> = Vec::new();
    for line in deps_section.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with('#') {
            continue;
        }
        if trimmed.starts_with('[') {
            // Defensive — should have been excluded by the section-end logic
            // above, but guard against nested-table entries just in case.
            continue;
        }
        let name = trimmed.split('=').next().unwrap_or(trimmed).trim();
        names.push(name);
    }

    // Assert EXACTLY 4 entries (v1.6 baseline shape).
    assert_eq!(
        names.len(),
        4,
        "SC4 violation: Cargo.toml [dependencies] must have exactly 4 entries \
         (v1.6 baseline); got {}: {:?}",
        names.len(),
        names
    );

    // Assert the names match the locked set (sort both sides so declaration
    // order in Cargo.toml does not affect the comparison — locks the SET
    // of deps, not the order).
    let mut names_sorted: Vec<&str> = names.clone();
    names_sorted.sort_unstable();
    let mut expected_sorted: Vec<&str> =
        ["chrono", "serde", "serde_json", "unicode-normalization"].to_vec();
    expected_sorted.sort_unstable();
    assert_eq!(
        names_sorted, expected_sorted,
        "SC4 violation: Cargo.toml [dependencies] names must equal the v1.6 \
         baseline set (chrono + serde + serde_json + unicode-normalization); \
         got {:?}",
        names
    );
}

// ---------------------------------------------------------------------------
// Test 2: SC1 — INT-13 golden dataset cross-source discipline sentinel
// ---------------------------------------------------------------------------

/// Re-assert INT-13's full cross-source discipline holistically at the
/// Phase 25 closure point.
///
/// ## INT-13 SC1 background
///
/// INT-13 requires the Mai Hoa casting surface be validated against ≥10
/// golden cases cross-checked against ≥2 independent sources per case.
/// Phase 22-02 shipped `mai_hoa_golden.json` with 12 cases × 2 sources each
/// + 2 KnownDivergence rows (SC1 already met at the data level). The
///   Phase 22-02 loader (`load_mai_hoa_golden`) already validates these
///   invariants at load time. This test re-asserts the same invariants
///   EXPLICITLY at the INT-13 level so a future weakening of the loader
///   would still trip this test — making this the formal Phase 25 closure
///   sentinel for SC1.
///
/// The test asserts:
/// - ≥10 cases (SC1 count).
/// - Every case has ≥2 sources (FS-10 dual-source discipline).
/// - Every case has at least one source whose `source` field contains
///   `nhantu.net` (the canonical first reference per INT-13's wording).
/// - ≥1 KnownDivergence row (divergences logged, NOT silently corrected
///   per AF-05).
/// - Every KnownDivergence row has non-empty `our_value`, `tiebreaker`,
///   and `note` fields (mirrors Phase 22-02's existing
///   `golden_known_divergences_are_logged_not_corrected` test, but with
///   INT-13-specific messages so a future regression points back to this
///   milestone closure).
/// - Schema pinned to `mai-hoa-golden-v1` (locks the schema pin so a
///   future v2 schema cannot silently invalidate the SC1 contract).
#[test]
fn int13_golden_dataset_cross_source_discipline_holds() {
    let ds = load_mai_hoa_golden();

    // SC1 count: ≥10 cases.
    assert!(
        ds.cases.len() >= 10,
        "INT-13 SC1 violation: need >= 10 golden cases, got {}",
        ds.cases.len()
    );

    // FS-10 dual-source + nhantu.net canonical first reference.
    for case in &ds.cases {
        assert!(
            case.sources.len() >= 2,
            "INT-13 SC1 violation: case '{}' must have >= 2 sources (FS-10 dual-source), got {}",
            case.id,
            case.sources.len()
        );
        let has_nhantu = case.sources.iter().any(|s| s.source.contains("nhantu.net"));
        assert!(
            has_nhantu,
            "INT-13 SC1 violation: case '{}' must have at least one nhantu.net source entry; \
             got sources: {:?}",
            case.id,
            case.sources
                .iter()
                .map(|s| s.source.as_str())
                .collect::<Vec<_>>()
        );
    }

    // AF-05: ≥1 KnownDivergence row (divergences logged, not silently corrected).
    assert!(
        !ds.known_divergences.is_empty(),
        "INT-13 SC1 violation: known_divergences must be non-empty (divergences logged, not \
         silently corrected per AF-05)"
    );

    // Every KnownDivergence row has non-empty our_value + tiebreaker + note.
    for div in &ds.known_divergences {
        assert!(
            !div.our_value.is_empty(),
            "INT-13 SC1 violation: KnownDivergence '{}' must have a non-empty our_value",
            div.case
        );
        assert!(
            !div.tiebreaker.is_empty(),
            "INT-13 SC1 violation: KnownDivergence '{}' must have a non-empty tiebreaker",
            div.case
        );
        assert!(
            !div.note.is_empty(),
            "INT-13 SC1 violation: KnownDivergence '{}' must have a non-empty note",
            div.case
        );
    }

    // Schema pin — locks the contract so a future v2 schema cannot silently
    // invalidate the SC1 invariant.
    assert_eq!(
        ds.schema_version, "mai-hoa-golden-v1",
        "INT-13 SC1 violation: schema_version must be pinned to 'mai-hoa-golden-v1'; got {:?}",
        ds.schema_version
    );
}

// Phase 25 (INT-13) baseline guards — closure of the v1.7 milestone.
// SC1 + SC4 locked as runtime invariants.
