//! Na Am API Contract Tests
//!
//! Contract tests verify schema stability, error handling, backward compatibility,
//! and roundtrip consistency for Na Am lookup APIs.

use amlich_api::{
    get_day_info, get_na_am_by_index, get_na_am_by_pair, DateQuery, NaAmErrorDto,
    NaAmResponseDto,
};
use amlich_core::almanac::sexagenary_cycle::{canchi_to_cycle_index, cycle_index_to_canchi};
use serde_json;

// ============================================================================
// Schema Stability Tests for Index Lookup
// ============================================================================

#[test]
fn test_index_lookup_schema() {
    // Test that index lookup returns Success response with all required fields
    let response = get_na_am_by_index(1);

    match response {
        NaAmResponseDto::Success(result) => {
            // Verify all required fields are present and non-empty
            assert!(result.cycle_index >= 1 && result.cycle_index <= 60);
            assert!(!result.can.is_empty(), "can field should not be empty");
            assert!(!result.chi.is_empty(), "chi field should not be empty");
            assert!(!result.na_am.is_empty(), "na_am field should not be empty");
            assert!(
                !result.element.is_empty(),
                "element field should not be empty"
            );
            assert!(
                !result.source_id.is_empty(),
                "source_id field should not be empty"
            );
            assert!(
                !result.method.is_empty(),
                "method field should not be empty"
            );
            assert!(
                !result.profile.is_empty(),
                "profile field should not be empty"
            );

            // Verify element is the last word of na_am
            let na_am_words: Vec<&str> = result.na_am.split_whitespace().collect();
            let last_word = na_am_words.last().expect("na_am should have words");
            assert_eq!(result.element, *last_word);
        }
        NaAmResponseDto::Error(err) => {
            panic!(
                "Expected Success for valid index 1, got Error: {}",
                err.message
            );
        }
    }
}

#[test]
fn test_index_lookup_serialization() {
    // Test that serialization is stable: serialize -> deserialize -> verify equality
    let response = get_na_am_by_index(31);

    // Serialize to JSON
    let json_string = serde_json::to_string(&response).expect("should serialize NaAmResponseDto");

    // Deserialize back
    let deserialized: NaAmResponseDto =
        serde_json::from_str(&json_string).expect("should deserialize JSON string");

    // Verify equality
    match (&response, &deserialized) {
        (NaAmResponseDto::Success(orig), NaAmResponseDto::Success(deser)) => {
            assert_eq!(orig.cycle_index, deser.cycle_index);
            assert_eq!(orig.can, deser.can);
            assert_eq!(orig.chi, deser.chi);
            assert_eq!(orig.na_am, deser.na_am);
            assert_eq!(orig.element, deser.element);
            assert_eq!(orig.source_id, deser.source_id);
            assert_eq!(orig.method, deser.method);
            assert_eq!(orig.profile, deser.profile);
        }
        _ => panic!("Both responses should be Success variant"),
    }
}

#[test]
fn test_index_lookup_all_positions() {
    // Test that all 60 valid indices return Success with valid cycle_index
    for index in 1..=60u8 {
        let response = get_na_am_by_index(index);

        match response {
            NaAmResponseDto::Success(result) => {
                assert_eq!(
                    result.cycle_index, index,
                    "cycle_index should match input index {}",
                    index
                );
                assert!(!result.can.is_empty());
                assert!(!result.chi.is_empty());
                assert!(!result.na_am.is_empty());
                assert!(!result.element.is_empty());

                // Verify element matches last word of na_am
                let na_am_words: Vec<&str> = result.na_am.split_whitespace().collect();
                let last_word = na_am_words.last().unwrap();
                assert_eq!(result.element, *last_word);
            }
            NaAmResponseDto::Error(err) => {
                panic!(
                    "Index {} should be valid, got Error: {}",
                    index, err.message
                );
            }
        }
    }
}

#[test]
fn test_index_lookup_evidence_metadata() {
    // Test that source_id, method, and profile are non-empty and stable
    let indices = [1, 7, 31, 60];

    let mut source_ids = Vec::new();
    let mut methods = Vec::new();
    let mut profiles = Vec::new();

    for &index in &indices {
        let response = get_na_am_by_index(index);

        match response {
            NaAmResponseDto::Success(result) => {
                assert!(
                    !result.source_id.is_empty(),
                    "source_id should not be empty"
                );
                assert!(!result.method.is_empty(), "method should not be empty");
                assert!(!result.profile.is_empty(), "profile should not be empty");

                source_ids.push(result.source_id.clone());
                methods.push(result.method.clone());
                profiles.push(result.profile.clone());
            }
            NaAmResponseDto::Error(_) => {
                panic!("Index {} should be valid", index);
            }
        }
    }

    // Verify all responses use the same metadata values (stable)
    assert!(
        source_ids.iter().all(|s| s == &source_ids[0]),
        "source_id should be consistent across lookups"
    );
    assert!(
        methods.iter().all(|m| m == &methods[0]),
        "method should be consistent across lookups"
    );
    assert!(
        profiles.iter().all(|p| p == &profiles[0]),
        "profile should be consistent across lookups"
    );

    // Verify expected values
    assert_eq!(
        source_ids[0], "tam-menh-thong-hoi",
        "Expected source_id to be tam-menh-thong-hoi"
    );
    assert_eq!(
        methods[0], "table-lookup",
        "Expected method to be table-lookup"
    );
}

// ============================================================================
// Schema Stability Tests for Pair Lookup
// ============================================================================

#[test]
fn test_pair_lookup_schema() {
    // Test that pair lookup returns Success response with all required fields
    let response = get_na_am_by_pair("Giáp", "Tý");

    match response {
        NaAmResponseDto::Success(result) => {
            // Verify all required fields are present and non-empty
            assert!(result.cycle_index >= 1 && result.cycle_index <= 60);
            assert_eq!(result.can, "Giáp");
            assert_eq!(result.chi, "Tý");
            assert!(!result.na_am.is_empty());
            assert!(!result.element.is_empty());
            assert!(!result.source_id.is_empty());
            assert!(!result.method.is_empty());
            assert!(!result.profile.is_empty());
        }
        NaAmResponseDto::Error(err) => {
            panic!(
                "Expected Success for valid pair Giáp Tý, got Error: {}",
                err.message
            );
        }
    }
}

#[test]
fn test_pair_lookup_serialization() {
    // Test that serialization is stable for pair lookup
    let response = get_na_am_by_pair("Ất", "Sửu");

    // Serialize to JSON
    let json_string = serde_json::to_string(&response).expect("should serialize NaAmResponseDto");

    // Deserialize back
    let deserialized: NaAmResponseDto =
        serde_json::from_str(&json_string).expect("should deserialize JSON string");

    // Verify equality
    match (&response, &deserialized) {
        (NaAmResponseDto::Success(orig), NaAmResponseDto::Success(deser)) => {
            assert_eq!(orig.cycle_index, deser.cycle_index);
            assert_eq!(orig.can, deser.can);
            assert_eq!(orig.chi, deser.chi);
            assert_eq!(orig.na_am, deser.na_am);
            assert_eq!(orig.element, deser.element);
            assert_eq!(orig.source_id, deser.source_id);
            assert_eq!(orig.method, deser.method);
            assert_eq!(orig.profile, deser.profile);
        }
        _ => panic!("Both responses should be Success variant"),
    }
}

#[test]
fn test_pair_lookup_all_canonical_pairs() {
    // Test canonical pairs from different stem groups
    let canonical_pairs = [
        ("Giáp", "Tý"),   // Index 1
        ("Ất", "Sửu"),    // Index 2
        ("Bính", "Dần"),  // Index 3
        ("Đinh", "Mão"),  // Index 4
        ("Canh", "Ngọ"),  // Index 7
        ("Tân", "Mùi"),   // Index 8
        ("Nhâm", "Thân"), // Index 9
        ("Quý", "Dậu"),   // Index 10
    ];

    for (can, chi) in canonical_pairs {
        let response = get_na_am_by_pair(can, chi);

        match response {
            NaAmResponseDto::Success(result) => {
                assert_eq!(result.can, can);
                assert_eq!(result.chi, chi);
                assert!(!result.na_am.is_empty());
                assert!(!result.element.is_empty());
            }
            NaAmResponseDto::Error(err) => {
                panic!(
                    "Pair {} {} should be valid, got Error: {}",
                    can, chi, err.message
                );
            }
        }
    }
}

#[test]
fn test_pair_lookup_roundtrip() {
    // Test that index -> CanChi -> pair lookup returns same result
    for index in 1..=60u8 {
        // Lookup by index
        let index_response = get_na_am_by_index(index);
        match index_response {
            NaAmResponseDto::Success(index_result) => {
                // Lookup by pair
                let pair_response = get_na_am_by_pair(&index_result.can, &index_result.chi);

                match pair_response {
                    NaAmResponseDto::Success(pair_result) => {
                        // Verify results match
                        assert_eq!(
                            index_result.cycle_index, pair_result.cycle_index,
                            "cycle_index mismatch for index {}",
                            index
                        );
                        assert_eq!(
                            index_result.can, pair_result.can,
                            "can mismatch for index {}",
                            index
                        );
                        assert_eq!(
                            index_result.chi, pair_result.chi,
                            "chi mismatch for index {}",
                            index
                        );
                        assert_eq!(
                            index_result.na_am, pair_result.na_am,
                            "na_am mismatch for index {}",
                            index
                        );
                        assert_eq!(
                            index_result.element, pair_result.element,
                            "element mismatch for index {}",
                            index
                        );
                    }
                    NaAmResponseDto::Error(err) => {
                        panic!(
                            "Pair lookup for {} {} should succeed, got Error: {}",
                            index_result.can, index_result.chi, err.message
                        );
                    }
                }
            }
            NaAmResponseDto::Error(_) => {
                panic!("Index {} should be valid", index);
            }
        }
    }
}

// ============================================================================
// Error Contract Tests
// ============================================================================

#[test]
fn test_index_lookup_bounds_error() {
    // Test that index 0 returns error with "invalid_cycle_index"
    let response = get_na_am_by_index(0);
    match response {
        NaAmResponseDto::Success(_) => {
            panic!("Index 0 should return Error");
        }
        NaAmResponseDto::Error(err) => {
            assert_eq!(err.error, "invalid_cycle_index");
            assert!(!err.message.is_empty());
        }
    }

    // Test that index 61 returns error with "invalid_cycle_index"
    let response = get_na_am_by_index(61);
    match response {
        NaAmResponseDto::Success(_) => {
            panic!("Index 61 should return Error");
        }
        NaAmResponseDto::Error(err) => {
            assert_eq!(err.error, "invalid_cycle_index");
            assert!(!err.message.is_empty());
        }
    }
}

#[test]
fn test_index_lookup_error_format() {
    // Verify Error response has correct error and message fields
    let response = get_na_am_by_index(0);

    match response {
        NaAmResponseDto::Success(_) => {
            panic!("Expected Error for invalid index");
        }
        NaAmResponseDto::Error(err) => {
            // Verify error field is a known error type
            let valid_errors = [
                "invalid_cycle_index",
                "invalid_stem_branch_pair",
                "unknown_stem",
                "unknown_branch",
            ];
            assert!(
                valid_errors.contains(&err.error.as_str()),
                "Unknown error type: {}",
                err.error
            );

            // Verify message field is present and non-empty
            assert!(!err.message.is_empty(), "Error message should not be empty");

            // Verify serialization works
            let json_string = serde_json::to_string(&err).expect("should serialize NaAmErrorDto");
            let deserialized: NaAmErrorDto =
                serde_json::from_str(&json_string).expect("should deserialize JSON");
            assert_eq!(err.error, deserialized.error);
            assert_eq!(err.message, deserialized.message);
        }
    }
}

#[test]
fn test_pair_lookup_non_canonical_error() {
    // Test odd/even mismatch pairs return "invalid_stem_branch_pair"
    // Giáp (even index 0) + Sửu (odd index 1) - non-canonical
    let response = get_na_am_by_pair("Giáp", "Sửu");
    match response {
        NaAmResponseDto::Success(_) => {
            panic!("Giáp Sửu should return Error (non-canonical)");
        }
        NaAmResponseDto::Error(err) => {
            assert_eq!(err.error, "invalid_stem_branch_pair");
        }
    }

    // Ất (odd index 1) + Tý (even index 0) - non-canonical
    let response = get_na_am_by_pair("Ất", "Tý");
    match response {
        NaAmResponseDto::Success(_) => {
            panic!("Ất Tý should return Error (non-canonical)");
        }
        NaAmResponseDto::Error(err) => {
            assert_eq!(err.error, "invalid_stem_branch_pair");
        }
    }
}

#[test]
fn test_pair_lookup_unknown_stem_error() {
    // Test invalid stem name returns "unknown_stem"
    let response = get_na_am_by_pair("Invalid", "Tý");
    match response {
        NaAmResponseDto::Success(_) => {
            panic!("Invalid stem should return Error");
        }
        NaAmResponseDto::Error(err) => {
            assert_eq!(err.error, "unknown_stem");
        }
    }
}

#[test]
fn test_pair_lookup_unknown_branch_error() {
    // Test invalid branch name returns "unknown_branch"
    let response = get_na_am_by_pair("Giáp", "Invalid");
    match response {
        NaAmResponseDto::Success(_) => {
            panic!("Invalid branch should return Error");
        }
        NaAmResponseDto::Error(err) => {
            assert_eq!(err.error, "unknown_branch");
        }
    }
}

#[test]
fn test_error_determinism() {
    // Test that calling the same error case multiple times returns same error type and message
    let test_cases = vec![
        || get_na_am_by_index(0),
        || get_na_am_by_index(61),
        || get_na_am_by_pair("Giáp", "Sửu"),
        || get_na_am_by_pair("Invalid", "Tý"),
        || get_na_am_by_pair("Giáp", "Invalid"),
    ];

    for test_case in test_cases {
        let first_error = test_case();
        let second_error = test_case();

        match (first_error, second_error) {
            (NaAmResponseDto::Error(err1), NaAmResponseDto::Error(err2)) => {
                assert_eq!(err1.error, err2.error, "Error type should be deterministic");
                assert_eq!(
                    err1.message, err2.message,
                    "Error message should be deterministic"
                );
            }
            _ => {
                panic!("Both calls should return Error");
            }
        }
    }
}

// ============================================================================
// Backward Compatibility Tests
// ============================================================================

#[test]
fn test_day_fortune_api_unchanged() {
    // Verify that get_day_info still returns DayInfoDto (Na Am API didn't break it)
    let query = DateQuery {
        day: 4,
        month: 3,
        year: 2026,
        timezone: None,
        ruleset_id: None,
        event_kind: None,
        enabled_pack_ids: vec![],
    };

    let result = get_day_info(&query);
    assert!(result.is_ok(), "get_day_info should still work");

    let day_info = result.unwrap();
    // Verify DayInfoDto structure is intact
    assert!(!day_info.solar.date_string.is_empty());
    assert!(!day_info.lunar.date_string.is_empty());
    assert!(!day_info.canchi.day.full.is_empty());
}

#[test]
fn test_day_fortune_na_am_preserved() {
    // Verify that DayFortune.day_element.na_am still works as before
    let query = DateQuery {
        day: 4,
        month: 3,
        year: 2026,
        timezone: None,
        ruleset_id: None,
        event_kind: None,
        enabled_pack_ids: vec![],
    };

    let result = get_day_info(&query);
    assert!(result.is_ok());

    let day_info = result.unwrap();
    let day_fortune = day_info.day_fortune.expect("day_fortune should be present");

    // Verify day_element.na_am field is preserved
    assert!(!day_fortune.day_element.na_am.is_empty());
    assert!(!day_fortune.day_element.element.is_empty());

    // Verify can_element and chi_element are still present
    assert!(!day_fortune.day_element.can_element.is_empty());
    assert!(!day_fortune.day_element.chi_element.is_empty());
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_index_pair_consistency() {
    // For multiple random indices, verify index and pair lookups return identical results
    let test_indices = [1, 7, 15, 23, 31, 38, 45, 52, 60];

    for &index in &test_indices {
        // Lookup by index
        let index_response = get_na_am_by_index(index);
        match index_response {
            NaAmResponseDto::Success(index_result) => {
                // Lookup by pair
                let pair_response = get_na_am_by_pair(&index_result.can, &index_result.chi);

                match pair_response {
                    NaAmResponseDto::Success(pair_result) => {
                        // Verify all fields match
                        assert_eq!(
                            index_result.cycle_index, pair_result.cycle_index,
                            "cycle_index mismatch"
                        );
                        assert_eq!(index_result.can, pair_result.can, "can mismatch");
                        assert_eq!(index_result.chi, pair_result.chi, "chi mismatch");
                        assert_eq!(index_result.na_am, pair_result.na_am, "na_am mismatch");
                        assert_eq!(
                            index_result.element, pair_result.element,
                            "element mismatch"
                        );
                    }
                    NaAmResponseDto::Error(_) => {
                        panic!("Pair lookup should succeed for valid pair");
                    }
                }
            }
            NaAmResponseDto::Error(_) => {
                panic!("Index lookup should succeed for valid index");
            }
        }
    }
}

#[test]
fn test_cycle_index_roundtrip() {
    // Test that index -> CanChi -> index returns same index
    for index in 1..=60u8 {
        // Get CanChi from index
        let canchi = cycle_index_to_canchi(index).expect("should convert index to CanChi");

        // Convert back to index
        let back_index = canchi_to_cycle_index(canchi.can_index, canchi.chi_index)
            .expect("should convert CanChi back to index");

        assert_eq!(
            back_index, index,
            "Roundtrip failed: {} -> ({}, {}) -> {}",
            index, canchi.can_index, canchi.chi_index, back_index
        );
    }
}
