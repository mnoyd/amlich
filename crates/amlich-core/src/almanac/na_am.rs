//! Na Am (Nạp Âm) lookup module
//!
//! Provides deterministic Na Am lookups through cycle index (1-60) or
//! stem-branch pair with evidence metadata.
//!
//! # Public Contract
//! - Cycle indices are 1-based (1-60) to match Vietnamese convention
//! - Invalid inputs return NaAmError (not panic)
//! - All operations are deterministic and side-effect-free

use crate::almanac::data::{get_ruleset_data, NaAmEntry};
use crate::almanac::sexagenary_cycle::cycle_index_to_canchi;
use crate::almanac::types::SourceMeta;
use crate::types::{CAN, CHI};

/// Na Am lookup error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NaAmError {
    /// Invalid cycle index (outside [1, 60])
    InvalidCycleIndex,
    /// Non-canonical stem-branch pair (odd/even mismatch)
    InvalidStemBranchPair,
    /// Unknown stem name (not in CAN array)
    UnknownStem,
    /// Unknown branch name (not in CHI array)
    UnknownBranch,
}

/// Lookup Na Am by 1-based cycle index (1-60)
///
/// # Arguments
/// * `index` - 1-based cycle index in range [1, 60]
///
/// # Returns
/// * `Ok(NaAmEntry)` if index is valid
/// * `Err(NaAmError::InvalidCycleIndex)` if index is outside [1, 60]
///
/// # Examples
/// ```ignore
/// let entry = get_na_am_by_index(1).unwrap();
/// assert_eq!(entry.can, "Giáp");
/// assert_eq!(entry.chi, "Tý");
/// assert_eq!(entry.na_am, "Hải Trung Kim");
/// ```
pub fn get_na_am_by_index(index: u8) -> Result<NaAmEntry, NaAmError> {
    // Validate 1-based bounds
    if !(1..=60).contains(&index) {
        return Err(NaAmError::InvalidCycleIndex);
    }

    // Convert index to stem-branch pair
    let canchi = cycle_index_to_canchi(index).ok_or(NaAmError::InvalidCycleIndex)?;

    // Retrieve Na Am entry from ruleset data
    let ruleset = get_ruleset_data("vn_baseline_v1").expect("default ruleset should be available");

    ruleset
        .sexagenary_na_am
        .get(&canchi.full)
        .cloned()
        .ok_or(NaAmError::InvalidCycleIndex)
}

/// Lookup Na Am by stem-branch pair (Vietnamese names)
///
/// # Arguments
/// * `can` - Vietnamese stem name (e.g., "Giáp", "Ất")
/// * `chi` - Vietnamese branch name (e.g., "Tý", "Sửu")
///
/// # Returns
/// * `Ok(NaAmEntry)` if pair is valid and canonical
/// * `Err(NaAmError::UnknownStem)` if can is not valid
/// * `Err(NaAmError::UnknownBranch)` if chi is not valid
/// * `Err(NaAmError::InvalidStemBranchPair)` if combination is non-canonical
///
/// # Canonical Validation
/// Only 60 of 120 possible stem/branch combinations are valid in the
/// sexagenary cycle. Stems and branches must share polarity (both odd or both even).
///
/// # Examples
/// ```ignore
/// let entry = get_na_am_by_pair("Giáp", "Tý").unwrap();
/// assert_eq!(entry.na_am, "Hải Trung Kim");
///
/// // Non-canonical combination returns error
/// assert!(get_na_am_by_pair("Giáp", "Sửu").is_err());
/// ```
pub fn get_na_am_by_pair(can: &str, chi: &str) -> Result<NaAmEntry, NaAmError> {
    // Validate stem name
    let can_idx = CAN
        .iter()
        .position(|&c| c == can)
        .ok_or(NaAmError::UnknownStem)?;

    // Validate branch name
    let chi_idx = CHI
        .iter()
        .position(|&c| c == chi)
        .ok_or(NaAmError::UnknownBranch)?;

    // Validate canonical combination: same polarity (odd/even)
    if can_idx % 2 != chi_idx % 2 {
        return Err(NaAmError::InvalidStemBranchPair);
    }

    // Retrieve Na Am entry from ruleset data
    let ruleset = get_ruleset_data("vn_baseline_v1").expect("default ruleset should be available");

    let key = format!("{can} {chi}");
    ruleset
        .sexagenary_na_am
        .get(&key)
        .cloned()
        .ok_or(NaAmError::InvalidStemBranchPair)
}

/// Get Na Am metadata (source attribution)
///
/// # Returns
/// Static reference to SourceMeta containing source_id, method, and profile
///
/// # Examples
/// ```ignore
/// let meta = get_na_am_metadata();
/// assert!(!meta.source_id.is_empty());
/// assert!(!meta.method.is_empty());
/// ```
pub fn get_na_am_metadata() -> &'static SourceMeta {
    let ruleset = get_ruleset_data("vn_baseline_v1").expect("default ruleset should be available");
    &ruleset.na_am_meta
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test 1: get_na_am_by_index returns NaAmEntry with all fields for valid index 1-60
    #[test]
    fn test_get_na_am_by_index_valid_range() {
        // Test lower bound: index 1 (Giáp Tý -> Hải Trung Kim)
        let result = get_na_am_by_index(1);
        assert!(result.is_ok(), "index 1 should be valid");
        let entry = result.unwrap();
        assert_eq!(entry.can, "Giáp");
        assert_eq!(entry.chi, "Tý");
        assert_eq!(entry.na_am, "Hải Trung Kim");
        assert_eq!(entry.element, "Kim");

        // Test upper bound: index 60 (Quý Hợi -> Đại Khê Thủy)
        let result = get_na_am_by_index(60);
        assert!(result.is_ok(), "index 60 should be valid");
        let entry = result.unwrap();
        assert_eq!(entry.can, "Quý");
        assert_eq!(entry.chi, "Hợi");
        assert!(!entry.na_am.is_empty());
        assert!(!entry.element.is_empty());

        // Test intermediate: index 31
        let result = get_na_am_by_index(31);
        assert!(result.is_ok(), "index 31 should be valid");
        let entry = result.unwrap();
        assert_eq!(entry.can, "Giáp");
        assert_eq!(entry.chi, "Ngọ");
        assert!(!entry.na_am.is_empty());
        assert!(!entry.element.is_empty());
    }

    // Test 2: get_na_am_by_index returns Err("invalid_cycle_index") for index outside [1, 60]
    #[test]
    fn test_get_na_am_by_index_invalid_index() {
        // Test below lower bound
        let result = get_na_am_by_index(0);
        assert!(result.is_err(), "index 0 should be invalid");
        assert_eq!(result.unwrap_err(), NaAmError::InvalidCycleIndex);

        // Test above upper bound
        let result = get_na_am_by_index(61);
        assert!(result.is_err(), "index 61 should be invalid");
        assert_eq!(result.unwrap_err(), NaAmError::InvalidCycleIndex);

        // Test far out of range
        let result = get_na_am_by_index(100);
        assert!(result.is_err(), "index 100 should be invalid");
        assert_eq!(result.unwrap_err(), NaAmError::InvalidCycleIndex);
    }

    // Test 3: get_na_am_by_pair returns NaAmEntry with all fields for valid stem-branch pair
    #[test]
    fn test_get_na_am_by_pair_valid_pairs() {
        // Test Giáp Tý (canonical pair)
        let result = get_na_am_by_pair("Giáp", "Tý");
        assert!(result.is_ok(), "Giáp Tý should be a valid pair");
        let entry = result.unwrap();
        assert_eq!(entry.can, "Giáp");
        assert_eq!(entry.chi, "Tý");
        assert_eq!(entry.na_am, "Hải Trung Kim");
        assert_eq!(entry.element, "Kim");

        // Test Ất Sửu (canonical pair)
        let result = get_na_am_by_pair("Ất", "Sửu");
        assert!(result.is_ok(), "Ất Sửu should be a valid pair");
        let entry = result.unwrap();
        assert_eq!(entry.can, "Ất");
        assert_eq!(entry.chi, "Sửu");
        assert!(!entry.na_am.is_empty());
        assert!(!entry.element.is_empty());

        // Test Quý Hợi (canonical pair)
        let result = get_na_am_by_pair("Quý", "Hợi");
        assert!(result.is_ok(), "Quý Hợi should be a valid pair");
        let entry = result.unwrap();
        assert_eq!(entry.can, "Quý");
        assert_eq!(entry.chi, "Hợi");
        assert!(!entry.na_am.is_empty());
        assert!(!entry.element.is_empty());
    }

    // Test 4: get_na_am_by_pair returns Err("invalid_stem_branch_pair") for non-canonical combination
    #[test]
    fn test_get_na_am_by_pair_non_canonical() {
        // Odd/even mismatch: Giáp (even index 0) + Sửu (odd index 1)
        let result = get_na_am_by_pair("Giáp", "Sửu");
        assert!(result.is_err(), "Giáp Sửu should be non-canonical");
        assert_eq!(result.unwrap_err(), NaAmError::InvalidStemBranchPair);

        // Odd/even mismatch: Ất (odd index 1) + Tý (even index 0)
        let result = get_na_am_by_pair("Ất", "Tý");
        assert!(result.is_err(), "Ất Tý should be non-canonical");
        assert_eq!(result.unwrap_err(), NaAmError::InvalidStemBranchPair);

        // Odd/even mismatch: Quý (odd index 9) + Tuất (even index 10)
        let result = get_na_am_by_pair("Quý", "Tuất");
        assert!(result.is_err(), "Quý Tuất should be non-canonical");
        assert_eq!(result.unwrap_err(), NaAmError::InvalidStemBranchPair);
    }

    // Test 5: get_na_am_by_pair returns Err("unknown_stem") or Err("unknown_branch") for invalid stem/branch names
    #[test]
    fn test_get_na_am_by_pair_invalid_names() {
        // Invalid stem name
        let result = get_na_am_by_pair("Invalid", "Tý");
        assert!(result.is_err(), "Invalid stem should return error");
        assert_eq!(result.unwrap_err(), NaAmError::UnknownStem);

        // Invalid branch name
        let result = get_na_am_by_pair("Giáp", "Invalid");
        assert!(result.is_err(), "Invalid branch should return error");
        assert_eq!(result.unwrap_err(), NaAmError::UnknownBranch);

        // Both invalid (should report stem error first)
        let result = get_na_am_by_pair("Invalid", "Invalid");
        assert!(result.is_err(), "Both invalid should return error");
        assert_eq!(result.unwrap_err(), NaAmError::UnknownStem);
    }

    // Additional test: Verify metadata helper returns correct SourceMeta
    #[test]
    fn test_get_na_am_metadata() {
        let meta = get_na_am_metadata();
        assert!(!meta.source_id.is_empty(), "source_id should not be empty");
        assert!(!meta.method.is_empty(), "method should not be empty");
    }

    // Additional test: Verify roundtrip between index and pair lookup
    #[test]
    fn test_index_pair_roundtrip() {
        for index in 1..=60u8 {
            // Lookup by index
            let index_result = get_na_am_by_index(index).expect("valid index should succeed");
            let can = index_result.can.clone();
            let chi = index_result.chi.clone();

            // Lookup by pair
            let pair_result = get_na_am_by_pair(&can, &chi).expect("valid pair should succeed");

            // Results should match
            assert_eq!(
                index_result.na_am, pair_result.na_am,
                "na_am should match for index {index}"
            );
            assert_eq!(
                index_result.element, pair_result.element,
                "element should match for index {index}"
            );
        }
    }
}
