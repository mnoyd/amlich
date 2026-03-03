//! Na Am lookup tests
//!
//! Tests for Na Am (Nạp Âm) lookup functionality supporting both
//! cycle index (1-60) and stem-branch pair lookup modes.

use super::*;
use crate::almanac::data::get_ruleset_data;
use crate::almanac::sexagenary_cycle::{canchi_to_cycle_index, cycle_index_to_canchi};
use crate::types::CAN;

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
