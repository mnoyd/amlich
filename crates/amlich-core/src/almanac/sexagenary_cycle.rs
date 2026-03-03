//! Sexagenary 60-cycle utilities
//!
//! Provides deterministic conversion and progression helpers for the Vietnamese
//! sexagenary cycle (60-year cycle of 10 heavenly stems and 12 earthly branches).
//!
//! # Public Contract
//! - Cycle indices are 1-based (1-60) to match Vietnamese convention
//! - Invalid inputs return None (not panic)
//! - All operations are deterministic and side-effect-free

use crate::types::CanChi;

/// Convert 1-based cycle index (1-60) to stem-branch pair
///
/// # Arguments
/// * `index` - 1-based cycle index in range [1, 60]
///
/// # Returns
/// * `Some(CanChi)` if index is valid
/// * `None` if index is outside [1, 60]
///
/// # Examples
/// ```ignore
/// let cc = cycle_index_to_canchi(1).unwrap();
/// assert_eq!(cc.can, "Giáp");
/// assert_eq!(cc.chi, "Tý");
/// ```
pub fn cycle_index_to_canchi(index: u8) -> Option<CanChi> {
    // Validate 1-based bounds
    if !(1..=60).contains(&index) {
        return None;
    }

    // Convert to 0-based for internal arithmetic
    let zero_based = (index - 1) as usize;

    // Compute stem and branch indices
    let can_idx = zero_based % 10;
    let chi_idx = zero_based % 12;

    Some(CanChi::new(can_idx, chi_idx))
}

/// Convert stem-branch pair to 1-based cycle index (1-60)
///
/// # Arguments
/// * `can_index` - Stem index (0-9)
/// * `chi_index` - Branch index (0-11)
///
/// # Returns
/// * `Some(u8)` with 1-based cycle index if pair is canonical
/// * `None` if pair is non-canonical (odd/even mismatch)
///
/// # Canonical Validation
/// Only 60 of 120 possible stem/branch combinations are valid in the
/// sexagenary cycle. Stems and branches must share polarity (both odd or both even).
///
/// # Examples
/// ```ignore
/// let idx = canchi_to_cycle_index(0, 0).unwrap(); // Giáp Tý
/// assert_eq!(idx, 1);
/// ```
pub fn canchi_to_cycle_index(can_index: usize, chi_index: usize) -> Option<u8> {
    // Validate indices are in bounds
    if can_index >= 10 || chi_index >= 12 {
        return None;
    }

    // Validate canonical combination: same polarity (odd/even)
    if can_index % 2 != chi_index % 2 {
        return None;
    }

    // Compute 60-cycle position (0-based)
    // Formula: solve for i where i % 10 = can_index and i % 12 = chi_index
    // Using Chinese Remainder Theorem for moduli 10 and 12 (gcd=2, so parity must match)
    let k = ((can_index as i32 - chi_index as i32) / 2).rem_euclid(6) as usize;
    let zero_based = (can_index + 10 * k) % 60;

    // Convert to 1-based
    Some((zero_based + 1) as u8)
}

/// Progress cycle index forward or backward with modular rollover
///
/// # Arguments
/// * `index` - Current 1-based cycle index in range [1, 60]
/// * `delta` - Number of steps to progress (positive = forward, negative = backward)
///
/// # Returns
/// * `Some(u8)` with new 1-based cycle index if input is valid
/// * `None` if input index is outside [1, 60]
///
/// # Rollover Behavior
/// - Forward from 60 wraps to 1: `progress_cycle_index(60, 1) == Some(1)`
/// - Backward from 1 wraps to 60: `progress_cycle_index(1, -1) == Some(60)`
/// - Large deltas wrap correctly: `progress_cycle_index(1, 125) == Some(6)`
///
/// # Examples
/// ```ignore
/// let idx = progress_cycle_index(1, -1).unwrap();
/// assert_eq!(idx, 60); // Wraps backward
/// ```
pub fn progress_cycle_index(index: u8, delta: i32) -> Option<u8> {
    // Validate 1-based bounds
    if !(1..=60).contains(&index) {
        return None;
    }

    // Convert to signed 0-based for arithmetic
    let zero_based = (index - 1) as i32;

    // Use rem_euclid for correct signed modular arithmetic
    let progressed = (zero_based + delta).rem_euclid(60) as u8;

    // Convert back to 1-based
    Some(progressed + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Task 1 Tests: cycle_index_to_canchi

    #[test]
    fn test_cycle_index_to_canchi_bounds() {
        // Valid lower bound: index 1 -> Giáp Tý
        let cc = cycle_index_to_canchi(1).expect("index 1 should be valid");
        assert_eq!(cc.full, "Giáp Tý");
        assert_eq!(cc.can_index, 0);
        assert_eq!(cc.chi_index, 0);

        // Valid upper bound: index 60 -> Quý Hợi
        let cc = cycle_index_to_canchi(60).expect("index 60 should be valid");
        assert_eq!(cc.full, "Quý Hợi");
        assert_eq!(cc.can_index, 9);
        assert_eq!(cc.chi_index, 11);

        // Invalid lower bound
        assert!(
            cycle_index_to_canchi(0).is_none(),
            "index 0 should be invalid"
        );

        // Invalid upper bound
        assert!(
            cycle_index_to_canchi(61).is_none(),
            "index 61 should be invalid"
        );
    }

    #[test]
    fn test_cycle_index_to_canchi_intermediate_values() {
        // Test index 7 -> Canh Ngọ
        let cc = cycle_index_to_canchi(7).expect("index 7 should be valid");
        assert_eq!(cc.full, "Canh Ngọ");
        assert_eq!(cc.can_index, 6);
        assert_eq!(cc.chi_index, 6);

        // Test index 31 -> Ất Mùi (let me verify: (31-1) % 10 = 0? No, 30 % 10 = 0 -> Giáp
        // Wait, let me recalculate: index 31 -> zero_based = 30
        // can_idx = 30 % 10 = 0 (Giáp)
        // chi_idx = 30 % 12 = 6 (Ngọ)
        // So index 31 should be Giáp Ngọ, not Ất Mùi
        let cc = cycle_index_to_canchi(31).expect("index 31 should be valid");
        assert_eq!(cc.full, "Giáp Ngọ");
        assert_eq!(cc.can_index, 0);
        assert_eq!(cc.chi_index, 6);

        // Test index 41 -> Giáp Thìn
        // zero_based = 40
        // can_idx = 40 % 10 = 0 (Giáp)
        // chi_idx = 40 % 12 = 4 (Thìn)
        let cc = cycle_index_to_canchi(41).expect("index 41 should be valid");
        assert_eq!(cc.full, "Giáp Thìn");
        assert_eq!(cc.can_index, 0);
        assert_eq!(cc.chi_index, 4);
    }

    #[test]
    fn test_cycle_index_to_canchi_all_canonical() {
        // Test that all 60 positions produce valid canonical pairs
        for i in 1..=60u8 {
            let cc = cycle_index_to_canchi(i).expect("should convert all valid indices");
            assert!(cc.can_index < 10, "can_index should be in range [0, 9]");
            assert!(cc.chi_index < 12, "chi_index should be in range [0, 11]");
            // Verify parity is consistent (canonical combination)
            assert_eq!(
                cc.can_index % 2,
                cc.chi_index % 2,
                "canonical pairs must have matching parity at index {}",
                i
            );
        }
    }

    // Task 2 Tests: canchi_to_cycle_index

    #[test]
    fn test_canchi_to_cycle_index_edge_cases() {
        // Giáp Tý (0, 0) -> 1
        assert_eq!(
            canchi_to_cycle_index(0, 0),
            Some(1),
            "Giáp Tý should map to index 1"
        );

        // Quý Hợi (9, 11) -> 60
        assert_eq!(
            canchi_to_cycle_index(9, 11),
            Some(60),
            "Quý Hợi should map to index 60"
        );
    }

    #[test]
    fn test_canchi_to_cycle_index_non_canonical() {
        // Odd/even mismatch - should return None
        assert!(
            canchi_to_cycle_index(0, 1).is_none(),
            "Giáp Sửu (0, 1) should be non-canonical (odd/even mismatch)"
        );
        assert!(
            canchi_to_cycle_index(1, 0).is_none(),
            "Ất Tý (1, 0) should be non-canonical (odd/even mismatch)"
        );
        assert!(
            canchi_to_cycle_index(9, 10).is_none(),
            "Quý Tuất (9, 10) should be non-canonical (odd/even mismatch)"
        );
        assert!(
            canchi_to_cycle_index(8, 11).is_none(),
            "Nhâm Hợi (8, 11) should be non-canonical (odd/even mismatch)"
        );
    }

    #[test]
    fn test_canchi_to_cycle_index_parity_validation() {
        // All canonical pairs have matching parity (both odd or both even)
        // Test a sampling of canonical pairs
        let canonical_pairs = [
            (0, 0),  // Giáp Tý (even, even)
            (1, 1),  // Ất Sửu (odd, odd)
            (2, 2),  // Bính Dần (even, even)
            (3, 3),  // Đinh Mão (odd, odd)
            (9, 11), // Quý Hợi (odd, odd)
            (0, 10), // Giáp Tuất (even, even)
            (2, 0),  // Bính Tý (even, even)
        ];

        for (can_idx, chi_idx) in canonical_pairs {
            assert_eq!(
                can_idx % 2,
                chi_idx % 2,
                "test pair ({}, {}) should have matching parity",
                can_idx,
                chi_idx
            );
            assert!(
                canchi_to_cycle_index(can_idx, chi_idx).is_some(),
                "canonical pair ({}, {}) should return Some(index)",
                can_idx,
                chi_idx
            );
        }
    }

    #[test]
    fn test_canchi_to_cycle_index_roundtrip() {
        // Test that conversion is invertible for all 60 cycle positions
        for i in 1..=60u8 {
            let cc = cycle_index_to_canchi(i).expect("should convert all valid indices");
            let back = canchi_to_cycle_index(cc.can_index, cc.chi_index)
                .expect("should convert back to cycle index");
            assert_eq!(
                back, i,
                "roundtrip failed: index {} -> ({}, {}) -> {}",
                i, cc.can_index, cc.chi_index, back
            );
        }
    }

    // Task 3 Tests: progress_cycle_index

    #[test]
    fn test_progress_cycle_index_rollover() {
        // Forward rollover: 60 + 1 = 1
        assert_eq!(
            progress_cycle_index(60, 1),
            Some(1),
            "forward from 60 by +1 should wrap to 1"
        );

        // Backward rollover: 1 - 1 = 60
        assert_eq!(
            progress_cycle_index(1, -1),
            Some(60),
            "backward from 1 by -1 should wrap to 60"
        );
    }

    #[test]
    fn test_progress_cycle_index_large_deltas() {
        // Test large positive delta
        // 1 + 125: zero_based = 0, 0 + 125 = 125, 125 % 60 = 5, result = 6
        assert_eq!(
            progress_cycle_index(1, 125),
            Some(6),
            "1 + 125 should wrap to 6"
        );

        // Test large negative delta
        // 30 - 125: zero_based = 29, 29 - 125 = -96, (-96).rem_euclid(60) = 24, result = 25
        assert_eq!(
            progress_cycle_index(30, -125),
            Some(25),
            "30 - 125 should wrap to 25"
        );

        // Test delta equal to cycle length
        // 50 + 60: should return same value
        assert_eq!(
            progress_cycle_index(50, 60),
            Some(50),
            "50 + 60 should return 50 (full cycle)"
        );

        // Test delta equal to negative cycle length
        // 25 - 60: should return same value
        assert_eq!(
            progress_cycle_index(25, -60),
            Some(25),
            "25 - 60 should return 25 (full backward cycle)"
        );
    }

    #[test]
    fn test_progress_cycle_index_zero_delta() {
        // Zero delta should return same index for all valid positions
        for i in 1..=60u8 {
            assert_eq!(
                progress_cycle_index(i, 0),
                Some(i),
                "zero delta should preserve index {}",
                i
            );
        }
    }

    #[test]
    fn test_progress_cycle_index_invalid_start() {
        // Invalid starting indices
        assert!(
            progress_cycle_index(0, 1).is_none(),
            "index 0 should be invalid starting position"
        );
        assert!(
            progress_cycle_index(61, -1).is_none(),
            "index 61 should be invalid starting position"
        );
    }

    #[test]
    fn test_progress_cycle_index_composition() {
        // Test that progression is composable: f(f(x, a), b) = f(x, a+b)
        for start in 1..=60u8 {
            for delta1 in [-125, -60, -1, 0, 1, 60, 125] {
                for delta2 in [-2, -1, 0, 1, 2] {
                    let result1 =
                        progress_cycle_index(progress_cycle_index(start, delta1).unwrap(), delta2);
                    let result2 = progress_cycle_index(start, delta1 + delta2);
                    assert_eq!(
                        result1, result2,
                        "progression composition failed: start={}, delta1={}, delta2={}",
                        start, delta1, delta2
                    );
                }
            }
        }
    }
}
