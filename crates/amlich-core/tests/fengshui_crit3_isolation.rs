//! CRIT-3 isolation grep guard - Phi Tinh types MUST NOT be imported into
//! `interaction/direction_merge.rs`. The `khcbppt` `direction_merge` module
//! handles DIRECTIONS (8-compass-point signals) for a query date; the
//! `huyen-khong` Phi Tinh module handles PALACE LAYOUTS (9-palace Lo Shu
//! arrangements). These are disjoint - the boundary discipline (DEC-0015/0016,
//! PITFALLS CRIT-3) forbids crossing them until Tier-3 spatial_compose lands.

use std::fs;
use std::path::Path;

/// Forbidden patterns: if ANY of these appear in `direction_merge.rs`,
/// CRIT-3 isolation is broken. Each pattern targets a different way the leak
/// could manifest (type import, module path, evidence method, function name).
const FORBIDDEN_TYPE_NAMES: &[&str] = &[
    "FlyingStar",
    "DailyFlyingStar",
    "DailyFlyingStarLayout",
    "almanac::fengshui",
    "phi_tinh",
    "compute_daily_flying_stars",
];

#[test]
fn direction_merge_does_not_import_flying_star_or_daily_flying_star() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/interaction/direction_merge.rs");
    let contents = fs::read_to_string(&path).expect("read direction_merge.rs");

    let mut violations: Vec<String> = Vec::new();
    for forbidden in FORBIDDEN_TYPE_NAMES {
        if contents.contains(forbidden) {
            violations.push(format!(
                "CRIT-3 violation: direction_merge.rs contains {:?} - \
                 Phi Tinh types must remain disjoint from interaction/direction_merge",
                forbidden
            ));
        }
    }
    assert!(
        violations.is_empty(),
        "CRIT-3 isolation broken (PITFALLS P-1):\n{}",
        violations.join("\n")
    );
}
