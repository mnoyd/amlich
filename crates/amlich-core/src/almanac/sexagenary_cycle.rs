//! Sexagenary 60-cycle utilities
//!
//! Provides deterministic conversion and progression helpers for the Vietnamese
//! sexagenary cycle (60-year cycle of 10 heavenly stems and 12 earthly branches).
//!
//! # Public Contract
//! - Cycle indices are 1-based (1-60) to match Vietnamese convention
//! - Invalid inputs return None (not panic)
//! - All operations are deterministic and side-effect-free

use crate::types::{CanChi, CAN, CHI};

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
    // Formula: (can_index * 6) + (chi_index / 2) modulo 60
    let zero_based = ((can_index * 6) + (chi_index / 2)) % 60;

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

    #[test]
    fn test_placeholder() {
        // Placeholder to ensure module compiles during RED phase
        assert!(true);
    }
}
