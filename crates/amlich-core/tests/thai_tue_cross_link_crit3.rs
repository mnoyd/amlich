//! Phase 23 (XLK-03) sibling CRIT-3 isolation guard.
//!
//! Mirrors the existing `tests/fengshui_crit3_isolation.rs` pattern: read
//! the target file(s) at runtime, scan for forbidden lower-level patterns,
//! assert zero matches. The forbidden-pattern list is shorter than the
//! fengshui guard's because `FlyingStar` / `DailyFlyingStar` /
//! `DailyFlyingStarLayout` are intentionally DROPPED here — they would
//! false-positive on the legitimate `snapshot.flying_stars` DTO field
//! access and the `FlyingStarsSummary.palace_overlays` array element
//! type at `crates/amlich-core/src/lib.rs:151`.
//!
//! Scan targets (exactly two):
//!   * `src/interaction/direction_merge.rs` — preserves the v1.6 firewall
//!     (no Phi Tinh / lower-level palace-layout type may appear).
//!   * `src/reasoning/direction_composite.rs` — Phase 23 carve-out
//!     (the cross-link consumes only the snapshot DTO + the existing
//!     eight-point `Direction` enum).
//!
//! The existing `tests/fengshui_crit3_isolation.rs` guard is UNCHANGED
//! and continues to scan `direction_merge.rs` with its original
//! six-pattern list.

use std::fs;
use std::path::Path;

/// Forbidden patterns: if ANY of these appear in either scan target,
/// CRIT-3 isolation is broken. The list and order are LOCKED by the
/// Phase 23 CONTEXT.md §"CRIT-3 sibling guard scope" decision.
const FORBIDDEN_TYPE_NAMES: &[&str] = &[
    "almanac::fengshui",
    "phi_tinh",
    "compute_daily_flying_stars",
    "compute_combined_overlay",
    "compute_palace_aspects",
    "TietKhiScanner",
    "FlyingStarPeriod",
];

/// The exact two scan targets (LOCKED).
const SCAN_TARGETS: &[&str] = &[
    "src/interaction/direction_merge.rs",
    "src/reasoning/direction_composite.rs",
];

#[test]
fn direction_merge_and_direction_composite_are_fengshui_free() {
    let mut violations = Vec::new();
    for rel_path in SCAN_TARGETS {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(rel_path);
        let contents = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read target file {rel_path}: {e}"));
        for forbidden in FORBIDDEN_TYPE_NAMES {
            if contents.contains(forbidden) {
                violations.push(format!(
                    "CRIT-3 violation: {} contains {:?}",
                    rel_path, forbidden
                ));
            }
        }
    }
    assert!(
        violations.is_empty(),
        "CRIT-3 isolation broken (Phase 23 sibling guard):\n{}",
        violations.join("\n")
    );
}

/// Defensive contract test: the guard list MUST stay at exactly seven
/// entries. A future maintainer adding or removing a pattern would
/// silently weaken or over-scan the guard; this test makes the change
/// explicit.
#[test]
fn forbidden_pattern_list_locked_at_seven_entries_in_order() {
    assert_eq!(
        FORBIDDEN_TYPE_NAMES.len(),
        7,
        "FORBIDDEN_TYPE_NAMES must hold exactly seven entries (CONTEXT.md locked list)"
    );
    // Order-sensitive equality so a future commit shuffling the entries
    // surfaces in the diff.
    assert_eq!(
        FORBIDDEN_TYPE_NAMES,
        &[
            "almanac::fengshui",
            "phi_tinh",
            "compute_daily_flying_stars",
            "compute_combined_overlay",
            "compute_palace_aspects",
            "TietKhiScanner",
            "FlyingStarPeriod",
        ]
    );
}

/// Defensive contract test: the scan target list MUST stay at exactly
/// two entries — the v1.6 firewall file and the Phase 23 carve-out.
#[test]
fn scan_target_list_locked_at_two_entries() {
    assert_eq!(
        SCAN_TARGETS.len(),
        2,
        "SCAN_TARGETS must hold exactly two entries (direction_merge + direction_composite)"
    );
    assert_eq!(
        SCAN_TARGETS,
        &[
            "src/interaction/direction_merge.rs",
            "src/reasoning/direction_composite.rs",
        ]
    );
}
