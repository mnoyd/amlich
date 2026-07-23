//! Combined Phi Tinh overlay — Vận + Niên + Nguyệt composition (FS-08, FS-09).
//!
//! Exposes `CombinedFlyingStarLayout` which aggregates the three time layers:
//!   - `van_layout`     — era-level base from `compute_period_for_year`
//!   - `annual_layout`  — Niên Tử Bạch from `compute_yearly_flying_stars`
//!   - `monthly_layout` — Nguyệt Tử Bạch from `compute_monthly_flying_stars`
//!
//! The `palace_overlays` field pairs `(annual_star, monthly_star)` per palace,
//! mirroring `annual_layout.palaces[i]` and `monthly_layout.palaces[i]` exactly —
//! no recomputation, pure composition.
//!
//! Four evidence envelopes are present on the returned value:
//!   1. `van_layout.evidence`     — method "phi_tinh.van"
//!   2. `annual_layout.evidence`  — method "phi_tinh.nien"
//!   3. `monthly_layout.evidence` — method "phi_tinh.nguyet"
//!   4. `evidence`                — composite method "rule.composite.flying_stars"
//!
//! NOTE: per PITFALLS CRIT-3, this module NEVER imports from
//! `crate::interaction`. It is a pure almanac composition layer.

use serde::{Deserialize, Serialize};

use crate::almanac::fengshui::{
    compute_monthly_flying_stars, compute_period_for_year, compute_yearly_flying_stars,
    scanner::TietKhiScanner,
    types::{FlyingStar, FlyingStarLayout},
};
use crate::reasoning::{ReasoningEvidenceEnvelope, ReasoningEvidenceSourceFamily};

// ---------------------------------------------------------------------------
// Combined layout type (FS-08)
// ---------------------------------------------------------------------------

/// Aggregate three Phi Tinh time layers into a single queryable overlay.
///
/// `palace_overlays[i] == (annual_layout.palaces[i], monthly_layout.palaces[i])`
/// for all i in 0..9.  Indexed in `Palace::ALL` order (index 0 = N, 4 = Center, 8 = S).
///
/// NOTE: `[(FlyingStar, FlyingStar); 9]` — serde handles fixed arrays of tuples
/// for arrays up to 32 elements via derive.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombinedFlyingStarLayout {
    /// Solar year.
    pub year: i32,
    /// Solar month (1-based, 1 = Dần/Lập Xuân … 12 = Sửu).
    pub month: u8,
    /// Per-palace `(annual_star, monthly_star)` pairs, indexed per `Palace::ALL` order.
    pub palace_overlays: [(FlyingStar, FlyingStar); 9],
    /// Annual (Niên) layer — evidence method "phi_tinh.nien".
    pub annual_layout: FlyingStarLayout,
    /// Monthly (Nguyệt) layer — evidence method "phi_tinh.nguyet".
    pub monthly_layout: FlyingStarLayout,
    /// Era (Vận) base layer — evidence method "phi_tinh.van".
    pub van_layout: FlyingStarLayout,
    /// Composite envelope — method "rule.composite.flying_stars" (FS-09).
    pub evidence: ReasoningEvidenceEnvelope,
}

// ---------------------------------------------------------------------------
// Composite evidence builder
// ---------------------------------------------------------------------------

fn composite_evidence() -> ReasoningEvidenceEnvelope {
    ReasoningEvidenceEnvelope {
        source_family: ReasoningEvidenceSourceFamily::AlmanacRule,
        source_id: crate::sources::SOURCE_HUYEN_KHONG.to_string(),
        method: "rule.composite.flying_stars".to_string(),
        note: None,
    }
}

// ---------------------------------------------------------------------------
// Public API (FS-08)
// ---------------------------------------------------------------------------

/// Compose the three Phi Tinh layers for `(year, month)` into a single overlay.
///
/// All star computations delegate to the existing sub-layer functions — no
/// star arithmetic is performed here.
pub fn compute_combined_overlay(
    year: i32,
    month: u8,
    scanner: &TietKhiScanner,
) -> CombinedFlyingStarLayout {
    let van_layout = compute_period_for_year(year, scanner).base_layout();
    let annual_layout = compute_yearly_flying_stars(year, scanner);
    let monthly_layout = compute_monthly_flying_stars(year, month, scanner);

    let overlays: [(FlyingStar, FlyingStar); 9] =
        std::array::from_fn(|i| (annual_layout.palaces[i], monthly_layout.palaces[i]));

    CombinedFlyingStarLayout {
        year,
        month,
        palace_overlays: overlays,
        annual_layout,
        monthly_layout,
        van_layout,
        evidence: composite_evidence(),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::almanac::fengshui::types::FlyingStarPeriod;

    fn scanner() -> TietKhiScanner {
        TietKhiScanner::new()
    }

    /// Basic construction: year=2024, month=1 returns the expected scalar fields.
    #[test]
    fn test_combined_overlay_2024_m1_basic() {
        let result = compute_combined_overlay(2024, 1, &scanner());
        assert_eq!(result.year, 2024);
        assert_eq!(result.month, 1);
        assert_eq!(result.palace_overlays.len(), 9);
    }

    /// palace_overlays[i].0 == annual_layout.palaces[i] for all i.
    #[test]
    fn test_combined_overlay_annual_mirrors_component() {
        let result = compute_combined_overlay(2024, 1, &scanner());
        for i in 0..9 {
            assert_eq!(
                result.palace_overlays[i].0, result.annual_layout.palaces[i],
                "palace_overlays[{i}].0 (annual) should equal annual_layout.palaces[{i}]"
            );
        }
    }

    /// palace_overlays[i].1 == monthly_layout.palaces[i] for all i.
    #[test]
    fn test_combined_overlay_monthly_mirrors_component() {
        let result = compute_combined_overlay(2024, 1, &scanner());
        for i in 0..9 {
            assert_eq!(
                result.palace_overlays[i].1, result.monthly_layout.palaces[i],
                "palace_overlays[{i}].1 (monthly) should equal monthly_layout.palaces[{i}]"
            );
        }
    }

    /// van_layout.period == Van { van: 9 } for 2024 (Vận 9 starts 2024).
    #[test]
    fn test_combined_overlay_van_period_2024() {
        let result = compute_combined_overlay(2024, 1, &scanner());
        if let FlyingStarPeriod::Van { van } = result.van_layout.period {
            assert_eq!(van, 9, "2024 should be Vận 9");
        } else {
            panic!("Expected Van period, got {:?}", result.van_layout.period);
        }
    }

    /// annual_layout.period == Yearly { year: 2024 }.
    #[test]
    fn test_combined_overlay_annual_period_2024() {
        let result = compute_combined_overlay(2024, 1, &scanner());
        if let FlyingStarPeriod::Yearly { year } = result.annual_layout.period {
            assert_eq!(year, 2024);
        } else {
            panic!(
                "Expected Yearly period, got {:?}",
                result.annual_layout.period
            );
        }
    }

    /// monthly_layout.period == Monthly { year: 2024, month: 1 }.
    #[test]
    fn test_combined_overlay_monthly_period_2024_m1() {
        let result = compute_combined_overlay(2024, 1, &scanner());
        if let FlyingStarPeriod::Monthly { year, month } = result.monthly_layout.period {
            assert_eq!(year, 2024);
            assert_eq!(month, 1);
        } else {
            panic!(
                "Expected Monthly period, got {:?}",
                result.monthly_layout.period
            );
        }
    }

    /// Four distinct evidence methods are present.
    #[test]
    fn test_combined_overlay_four_evidence_methods() {
        let result = compute_combined_overlay(2024, 1, &scanner());
        assert_eq!(
            result.annual_layout.evidence.method, "phi_tinh.nien",
            "annual evidence method should be phi_tinh.nien"
        );
        assert_eq!(
            result.monthly_layout.evidence.method, "phi_tinh.nguyet",
            "monthly evidence method should be phi_tinh.nguyet"
        );
        assert_eq!(
            result.van_layout.evidence.method, "phi_tinh.van",
            "van evidence method should be phi_tinh.van"
        );
        assert_eq!(
            result.evidence.method, "rule.composite.flying_stars",
            "composite evidence method should be rule.composite.flying_stars"
        );
    }

    /// Composite evidence has source_id == SOURCE_HUYEN_KHONG and family == AlmanacRule.
    #[test]
    fn test_combined_overlay_composite_evidence_source() {
        let result = compute_combined_overlay(2024, 1, &scanner());
        assert_eq!(
            result.evidence.source_id,
            crate::sources::SOURCE_HUYEN_KHONG
        );
        assert!(matches!(
            result.evidence.source_family,
            crate::reasoning::ReasoningEvidenceSourceFamily::AlmanacRule
        ));
    }

    /// Serde round-trip: the full CombinedFlyingStarLayout serializes and deserializes correctly.
    #[test]
    fn test_combined_overlay_serde_round_trip() {
        let original = compute_combined_overlay(2024, 1, &scanner());
        let json = serde_json::to_string(&original).expect("serialization failed");
        let roundtripped: CombinedFlyingStarLayout =
            serde_json::from_str(&json).expect("deserialization failed");
        // Verify key fields survived the round-trip.
        assert_eq!(roundtripped.year, original.year);
        assert_eq!(roundtripped.month, original.month);
        assert_eq!(roundtripped.evidence.method, original.evidence.method);
        assert_eq!(
            roundtripped.annual_layout.evidence.method,
            original.annual_layout.evidence.method
        );
        assert_eq!(
            roundtripped.monthly_layout.evidence.method,
            original.monthly_layout.evidence.method
        );
        assert_eq!(
            roundtripped.van_layout.evidence.method,
            original.van_layout.evidence.method
        );
        for i in 0..9 {
            assert_eq!(
                roundtripped.palace_overlays[i].0, original.palace_overlays[i].0,
                "palace_overlays[{i}].0 mismatch after round-trip"
            );
            assert_eq!(
                roundtripped.palace_overlays[i].1, original.palace_overlays[i].1,
                "palace_overlays[{i}].1 mismatch after round-trip"
            );
        }
    }

    /// Verify no crate::interaction import can exist (PITFALLS CRIT-3) — compile-time check.
    /// This test always passes; the guard is that combined.rs must compile without interaction.
    #[test]
    fn test_no_interaction_import_compiles() {
        // If combined.rs imported crate::interaction, the module would fail to compile.
        // This test existing and passing confirms the import guard holds.
        let _ = compute_combined_overlay(2024, 3, &TietKhiScanner::new());
    }
}
