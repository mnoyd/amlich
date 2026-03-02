//! Tu Mến (Kua) Calculator
//!
//! This module implements the Kua (Trường Sinh) number calculation based on Vietnamese/Asian
//! feng-shui conventions. The Kua number is used to determine favorable and unfavorable
//! directions for individuals based on their birth year and gender.
//!
//! # Project Convention
//!
//! This implementation follows a specific, frozen convention to ensure deterministic results:
//!
//! - **Year Basis**: Solar year (Gregorian calendar) - input year is used directly without
//!   lunar conversion. This is the most common convention in modern Vietnamese feng-shui practice.
//!
//! - **Kua 5 Resolution**: Kua 5 is a special case that doesn't exist in the East/West grouping.
//!   - Male: Kua 5 → Kua 8
//!   - Female: Kua 5 → Kua 2
//!
//! - **Gender Encoding**: Enum `Gender` with `Male` and `Female` variants.
//!
//! - **East/West Group Assignment**:
//!   - East Group: Kua 1, 3, 4, 9 (odd numbers except 5)
//!   - West Group: Kua 2, 5→6, 7, 8 (even numbers except 5 resolved to even)
//!
//! - **Favorable/Unfavorable Directions**:
//!   - East Group favorable: North, South, East, Southeast
//!   - East Group unfavorable: Northwest, Southwest, West, Northeast
//!   - West Group favorable: Northwest, Southwest, West, Northeast
//!   - West Group unfavorable: North, South, East, Southeast
//!
//! # Formula
//!
//! For 2000+ years:
//! - Sum the digits of the birth year
//! - If Male: Subtract from 9
//! - If Female: Add 5 to the sum
//!
//! For 1900-1999 years:
//! - Sum the digits of the birth year
//! - If Male: Subtract from 10
//! - If Female: Add 5 to the sum
//!
//! If result is > 9, sum digits again (reduce to single digit).
//! Then apply Kua 5 resolution if applicable.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Gender for Kua calculation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Gender {
    Male,
    Female,
}

/// East/West group based on Kua number
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KuaGroup {
    East,
    West,
}

impl fmt::Display for KuaGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KuaGroup::East => write!(f, "East"),
            KuaGroup::West => write!(f, "West"),
        }
    }
}

/// The eight cardinal and intercardinal directions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::North => write!(f, "North"),
            Direction::Northeast => write!(f, "Northeast"),
            Direction::East => write!(f, "East"),
            Direction::Southeast => write!(f, "Southeast"),
            Direction::South => write!(f, "South"),
            Direction::Southwest => write!(f, "Southwest"),
            Direction::West => write!(f, "West"),
            Direction::Northwest => write!(f, "Northwest"),
        }
    }
}

/// Metadata documenting the convention used for Kua calculation
///
/// This structure allows the API consumer to understand exactly which
/// conventions were applied, important for reproducibility and debugging.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConventionMetadata {
    /// Year basis convention used
    pub year_basis: String,
    /// How Kua 5 is resolved
    pub kua5_resolution: String,
    /// Gender encoding scheme
    pub gender_encoding: String,
}

impl ConventionMetadata {
    /// Create convention metadata with project defaults
    pub fn project_default() -> Self {
        Self {
            year_basis: "solar".to_string(),
            kua5_resolution: "male->8,female->2".to_string(),
            gender_encoding: "enum(Male,Female)".to_string(),
        }
    }
}

/// Result of Kua calculation
///
/// Contains the computed Kua number, East/West group, favorable/unfavorable
/// directions, and convention metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KuaResult {
    /// Kua number (1-9, excluding 5)
    pub kua: u8,
    /// East or West group
    pub group: KuaGroup,
    /// Four favorable directions for this group
    pub favorable_directions: [Direction; 4],
    /// Four unfavorable directions for this group
    pub unfavorable_directions: [Direction; 4],
    /// Convention metadata documenting how this was calculated
    pub convention: ConventionMetadata,
}

impl KuaResult {
    /// Create a new KuaResult
    pub fn new(
        kua: u8,
        group: KuaGroup,
        favorable_directions: [Direction; 4],
        unfavorable_directions: [Direction; 4],
        convention: ConventionMetadata,
    ) -> Self {
        Self {
            kua,
            group,
            favorable_directions,
            unfavorable_directions,
            convention,
        }
    }
}

/// Compute Kua number and related information from birth year and gender
///
/// # Arguments
/// * `birth_year` - Solar year (e.g., 1990, 2002)
/// * `gender` - Gender (Male or Female)
///
/// # Returns
/// Kua result with number, group, directions, and convention metadata
///
/// # Examples
/// ```
/// use amlich_core::almanac::tu_menh::{compute_kua, Gender};
///
/// let result = compute_kua(1990, Gender::Male);
/// assert_eq!(result.kua, 9);  // Kua 9, East group
/// assert_eq!(result.group, amlich_core::almanac::tu_menh::KuaGroup::East);
/// ```
pub fn compute_kua(birth_year: i32, gender: Gender) -> KuaResult {
    let convention = ConventionMetadata::project_default();

    // Sum the digits of the birth year to a single digit
    let year_str = birth_year.to_string();
    let mut digit_sum: u8 = year_str
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .sum();
    while digit_sum > 9 {
        let sum_str = digit_sum.to_string();
        digit_sum = sum_str.chars().map(|c| c.to_digit(10).unwrap() as u8).sum();
    }

    // Calculate Kua based on century and gender
    let kua_unresolved = if birth_year >= 2000 {
        // 2000+ years
        match gender {
            Gender::Male => {
                // Subtract from 9
                let result = 9 - digit_sum;
                if result == 0 {
                    9
                } else {
                    result
                }
            }
            Gender::Female => {
                // Add 5
                let result = digit_sum + 5;
                if result > 9 {
                    result - 9
                } else {
                    result
                }
            }
        }
    } else {
        // 1900-1999 years
        match gender {
            Gender::Male => {
                // Subtract from 10
                let result = 10 - digit_sum;
                if result == 10 {
                    1
                } else {
                    result
                }
            }
            Gender::Female => {
                // Add 5
                let result = digit_sum + 5;
                if result > 9 {
                    result - 9
                } else {
                    result
                }
            }
        }
    };

    // Apply Kua 5 resolution
    let kua = if kua_unresolved == 5 {
        match gender {
            Gender::Male => 8,
            Gender::Female => 2,
        }
    } else {
        kua_unresolved
    };

    // Determine East/West group
    let group = match kua {
        1 | 3 | 4 | 9 => KuaGroup::East,
        2 | 6 | 7 | 8 => KuaGroup::West,
        _ => panic!("Invalid Kua number: {}", kua),
    };

    // Determine favorable and unfavorable directions based on group
    let (favorable_directions, unfavorable_directions) = match group {
        KuaGroup::East => (
            [
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::Southeast,
            ],
            [
                Direction::Northwest,
                Direction::Southwest,
                Direction::West,
                Direction::Northeast,
            ],
        ),
        KuaGroup::West => (
            [
                Direction::Northwest,
                Direction::Southwest,
                Direction::West,
                Direction::Northeast,
            ],
            [
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::Southeast,
            ],
        ),
    };

    KuaResult::new(
        kua,
        group,
        favorable_directions,
        unfavorable_directions,
        convention,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kua_group_enum_has_east_and_west_variants() {
        let east = KuaGroup::East;
        let west = KuaGroup::West;
        assert_ne!(east, west);
        assert_eq!(east, KuaGroup::East);
        assert_eq!(west, KuaGroup::West);
    }

    #[test]
    fn kua_group_serializes_to_snake_case() {
        let json = serde_json::to_string(&KuaGroup::East).expect("serialize");
        assert_eq!(json, "\"east\"");

        let json = serde_json::to_string(&KuaGroup::West).expect("serialize");
        assert_eq!(json, "\"west\"");
    }

    #[test]
    fn kua_result_struct_has_required_fields() {
        let convention = ConventionMetadata::project_default();
        let result = KuaResult::new(
            1,
            KuaGroup::East,
            [
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::Southeast,
            ],
            [
                Direction::Northwest,
                Direction::Southwest,
                Direction::West,
                Direction::Northeast,
            ],
            convention,
        );

        assert_eq!(result.kua, 1);
        assert_eq!(result.group, KuaGroup::East);
        assert_eq!(result.favorable_directions.len(), 4);
        assert_eq!(result.unfavorable_directions.len(), 4);
    }

    #[test]
    fn convention_metadata_contains_required_fields() {
        let meta = ConventionMetadata::project_default();
        assert_eq!(meta.year_basis, "solar");
        assert_eq!(meta.kua5_resolution, "male->8,female->2");
        assert_eq!(meta.gender_encoding, "enum(Male,Female)");
    }

    #[test]
    fn convention_metadata_serializes() {
        let meta = ConventionMetadata::project_default();
        let json = serde_json::to_string(&meta).expect("serialize");
        assert!(json.contains("solar"));
        assert!(json.contains("male->8,female->2"));
        assert!(json.contains("enum(Male,Female)"));
    }

    #[test]
    fn gender_enum_has_male_and_female() {
        assert_eq!(Gender::Male, Gender::Male);
        assert_eq!(Gender::Female, Gender::Female);
        assert_ne!(Gender::Male, Gender::Female);
    }

    #[test]
    fn gender_serializes_to_snake_case() {
        let json = serde_json::to_string(&Gender::Male).expect("serialize");
        assert_eq!(json, "\"male\"");

        let json = serde_json::to_string(&Gender::Female).expect("serialize");
        assert_eq!(json, "\"female\"");
    }

    #[test]
    fn direction_enum_has_all_eight_directions() {
        use Direction::*;
        let directions = [
            North, Northeast, East, Southeast, South, Southwest, West, Northwest,
        ];
        assert_eq!(directions.len(), 8);
    }

    #[test]
    fn compute_kua_api_signature_exists() {
        // This test will fail until compute_kua is implemented
        // It verifies the API signature exists
        let _ = compute_kua(1990, Gender::Male);
    }
}

// Task 2 tests - RED phase
#[cfg(test)]
mod compute_tests {
    use super::*;

    #[test]
    fn standard_years_compute_correctly() {
        // Test representative years from both centuries
        // Values calculated using documented formula:
        // - Sum year digits to single digit
        // - 1900-1999 male: subtract from 10
        // - 1900-1999 female: add 5
        // - 2000+ male: subtract from 9
        // - 2000+ female: add 5
        // - Kua 5 resolution: male->8, female->2

        let tests = vec![
            // 1900s
            (1990, Gender::Male, 9),   // 1+9+9+0=19->1, 10-1=9
            (1990, Gender::Female, 6), // 1+5=6
            (1985, Gender::Male, 8),   // 1+9+8+5=23->5, 10-5=5->8 (Kua 5 resolution)
            (1985, Gender::Female, 1), // 5+5=10->1
            (1978, Gender::Male, 3),   // 1+9+7+8=25->7, 10-7=3
            (1978, Gender::Female, 3), // 7+5=12->3
            // 2000s
            (2002, Gender::Male, 8), // 2+0+0+2=4, 9-4=5->8 (Kua 5 resolution)
            (2002, Gender::Female, 9), // 4+5=9
            (2010, Gender::Male, 6), // 2+0+1+0=3, 9-3=6
            (2010, Gender::Female, 8), // 3+5=8
            (2024, Gender::Male, 1), // 2+0+2+4=8, 9-8=1
            (2024, Gender::Female, 4), // 8+5=13->4
        ];

        for (year, gender, expected_kua) in tests {
            let result = compute_kua(year, gender);
            assert_eq!(
                result.kua, expected_kua,
                "Year {} {:?} should have Kua {}, got {}",
                year, gender, expected_kua, result.kua
            );
        }
    }

    #[test]
    fn kua_5_resolves_correctly_by_gender() {
        // Test years that result in Kua 5 before resolution
        // 2002: 2+0+0+2 = 4
        // Male: 9-4 = 5 -> should become 8
        // Female: 4+5 = 9
        let male_result = compute_kua(2002, Gender::Male);
        assert_eq!(male_result.kua, 8, "Male Kua 5 should resolve to 8");

        let female_result = compute_kua(2002, Gender::Female);
        assert_ne!(female_result.kua, 5, "Female should not have Kua 5");
        assert_eq!(female_result.kua, 9, "Female 2002 should have Kua 9");

        // 2011: 2+0+1+1 = 4 -> same as 2002
        let male_2011 = compute_kua(2011, Gender::Male);
        assert_eq!(male_2011.kua, 8, "Male Kua 5 should resolve to 8");

        let female_2011 = compute_kua(2011, Gender::Female);
        assert_ne!(female_2011.kua, 5, "Female should not have Kua 5");
        assert_eq!(female_2011.kua, 9, "Female 2011 should have Kua 9");
    }

    #[test]
    fn east_west_group_derives_correctly() {
        // East group: Kua 1, 3, 4, 9
        let east_kuas = [1, 3, 4, 9];
        for kua in east_kuas {
            let result = create_test_result(kua);
            assert_eq!(
                result.group,
                KuaGroup::East,
                "Kua {} should be East group",
                kua
            );
        }

        // West group: Kua 2, 6, 7, 8
        let west_kuas = [2, 6, 7, 8];
        for kua in west_kuas {
            let result = create_test_result(kua);
            assert_eq!(
                result.group,
                KuaGroup::West,
                "Kua {} should be West group",
                kua
            );
        }
    }

    /// Helper to create a KuaResult with a specific Kua number for testing group assignment
    fn create_test_result(kua: u8) -> KuaResult {
        let group = match kua {
            1 | 3 | 4 | 9 => KuaGroup::East,
            2 | 6 | 7 | 8 => KuaGroup::West,
            _ => panic!("Invalid test Kua: {}", kua),
        };

        let (favorable, unfavorable) = match group {
            KuaGroup::East => (
                [
                    Direction::North,
                    Direction::South,
                    Direction::East,
                    Direction::Southeast,
                ],
                [
                    Direction::Northwest,
                    Direction::Southwest,
                    Direction::West,
                    Direction::Northeast,
                ],
            ),
            KuaGroup::West => (
                [
                    Direction::Northwest,
                    Direction::Southwest,
                    Direction::West,
                    Direction::Northeast,
                ],
                [
                    Direction::North,
                    Direction::South,
                    Direction::East,
                    Direction::Southeast,
                ],
            ),
        };

        KuaResult::new(
            kua,
            group,
            favorable,
            unfavorable,
            ConventionMetadata::project_default(),
        )
    }
}

// Task 3 tests - RED phase
#[cfg(test)]
mod direction_tests {
    use super::*;

    #[test]
    fn east_group_has_correct_favorable_directions() {
        // East group favorable: North, South, East, Southeast
        let east_kua_years = [(1978, Gender::Male), (2024, Gender::Female)];

        for (year, gender) in east_kua_years {
            let result = compute_kua(year, gender);
            assert_eq!(result.group, KuaGroup::East);

            let expected_favorable = [
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::Southeast,
            ];

            for (i, expected) in expected_favorable.iter().enumerate() {
                assert_eq!(
                    &result.favorable_directions[i], expected,
                    "East group direction {} should be {:?}",
                    i, expected
                );
            }
        }
    }

    #[test]
    fn east_group_has_correct_unfavorable_directions() {
        // East group unfavorable: Northwest, Southwest, West, Northeast
        let east_kua_years = [(1978, Gender::Male), (2024, Gender::Female)];

        for (year, gender) in east_kua_years {
            let result = compute_kua(year, gender);
            assert_eq!(result.group, KuaGroup::East);

            let expected_unfavorable = [
                Direction::Northwest,
                Direction::Southwest,
                Direction::West,
                Direction::Northeast,
            ];

            for (i, expected) in expected_unfavorable.iter().enumerate() {
                assert_eq!(
                    &result.unfavorable_directions[i], expected,
                    "East group unfavorable direction {} should be {:?}",
                    i, expected
                );
            }
        }
    }

    #[test]
    fn west_group_has_correct_favorable_directions() {
        // West group favorable: Northwest, Southwest, West, Northeast
        let west_kua_years = [(1990, Gender::Female), (1985, Gender::Male)];

        for (year, gender) in west_kua_years {
            let result = compute_kua(year, gender);
            assert_eq!(result.group, KuaGroup::West);

            let expected_favorable = [
                Direction::Northwest,
                Direction::Southwest,
                Direction::West,
                Direction::Northeast,
            ];

            for (i, expected) in expected_favorable.iter().enumerate() {
                assert_eq!(
                    &result.favorable_directions[i], expected,
                    "West group direction {} should be {:?}",
                    i, expected
                );
            }
        }
    }

    #[test]
    fn west_group_has_correct_unfavorable_directions() {
        // West group unfavorable: North, South, East, Southeast
        let west_kua_years = [(1990, Gender::Female), (1985, Gender::Male)];

        for (year, gender) in west_kua_years {
            let result = compute_kua(year, gender);
            assert_eq!(result.group, KuaGroup::West);

            let expected_unfavorable = [
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::Southeast,
            ];

            for (i, expected) in expected_unfavorable.iter().enumerate() {
                assert_eq!(
                    &result.unfavorable_directions[i], expected,
                    "West group unfavorable direction {} should be {:?}",
                    i, expected
                );
            }
        }
    }
}

// Task 3 fixture tests - RED phase
#[cfg(test)]
mod fixture_tests {
    use super::*;

    /// Fixture test data for Kua calculations
    /// Covers representative years from 1900-2099 with edge cases
    #[derive(Debug, Clone)]
    struct KuaFixture {
        year: i32,
        gender: Gender,
        expected_kua: u8,
        expected_group: KuaGroup,
    }

    #[test]
    fn representative_years_1900_2099() {
        let fixtures = vec![
            // Year boundary cases
            KuaFixture {
                year: 1899,
                gender: Gender::Male,
                expected_kua: 1,
                expected_group: KuaGroup::East,
            },
            KuaFixture {
                year: 1899,
                gender: Gender::Female,
                expected_kua: 2,
                expected_group: KuaGroup::West,
            },
            KuaFixture {
                year: 1900,
                gender: Gender::Male,
                expected_kua: 9,
                expected_group: KuaGroup::East,
            },
            KuaFixture {
                year: 1900,
                gender: Gender::Female,
                expected_kua: 6,
                expected_group: KuaGroup::West,
            },
            KuaFixture {
                year: 2099,
                gender: Gender::Male,
                expected_kua: 7,
                expected_group: KuaGroup::West,
            },
            KuaFixture {
                year: 2099,
                gender: Gender::Female,
                expected_kua: 7,
                expected_group: KuaGroup::West,
            },
            KuaFixture {
                year: 2100,
                gender: Gender::Male,
                expected_kua: 6,
                expected_group: KuaGroup::West,
            },
            KuaFixture {
                year: 2100,
                gender: Gender::Female,
                expected_kua: 8,
                expected_group: KuaGroup::West,
            },
            // Kua 5 years (before resolution)
            KuaFixture {
                year: 1994,
                gender: Gender::Male,
                expected_kua: 8,
                expected_group: KuaGroup::West,
            },
            KuaFixture {
                year: 1994,
                gender: Gender::Female,
                expected_kua: 1,
                expected_group: KuaGroup::East,
            },
            KuaFixture {
                year: 2002,
                gender: Gender::Male,
                expected_kua: 8,
                expected_group: KuaGroup::West,
            },
            KuaFixture {
                year: 2002,
                gender: Gender::Female,
                expected_kua: 9,
                expected_group: KuaGroup::East,
            },
            // Full Kua range coverage
            // Kua 1 (East)
            KuaFixture {
                year: 1994,
                gender: Gender::Female,
                expected_kua: 1,
                expected_group: KuaGroup::East,
            },
            // Kua 2 (West)
            KuaFixture {
                year: 1899,
                gender: Gender::Female,
                expected_kua: 2,
                expected_group: KuaGroup::West,
            },
            // Kua 3 (East)
            KuaFixture {
                year: 1978,
                gender: Gender::Male,
                expected_kua: 3,
                expected_group: KuaGroup::East,
            },
            // Kua 4 (East)
            KuaFixture {
                year: 1995,
                gender: Gender::Male,
                expected_kua: 4,
                expected_group: KuaGroup::East,
            },
            // Kua 6 (West)
            KuaFixture {
                year: 1993,
                gender: Gender::Male,
                expected_kua: 6,
                expected_group: KuaGroup::West,
            },
            // Kua 7 (West)
            KuaFixture {
                year: 2099,
                gender: Gender::Male,
                expected_kua: 7,
                expected_group: KuaGroup::West,
            },
            // Kua 8 (West)
            KuaFixture {
                year: 1985,
                gender: Gender::Male,
                expected_kua: 8,
                expected_group: KuaGroup::West,
            },
            // Kua 9 (East)
            KuaFixture {
                year: 1900,
                gender: Gender::Male,
                expected_kua: 9,
                expected_group: KuaGroup::East,
            },
            // Additional representative cases
            KuaFixture {
                year: 1950,
                gender: Gender::Male,
                expected_kua: 4,
                expected_group: KuaGroup::East,
            },
            KuaFixture {
                year: 1950,
                gender: Gender::Female,
                expected_kua: 2,
                expected_group: KuaGroup::West,
            },
            KuaFixture {
                year: 1975,
                gender: Gender::Male,
                expected_kua: 6,
                expected_group: KuaGroup::West,
            },
            KuaFixture {
                year: 1975,
                gender: Gender::Female,
                expected_kua: 9,
                expected_group: KuaGroup::East,
            },
            KuaFixture {
                year: 2000,
                gender: Gender::Male,
                expected_kua: 7,
                expected_group: KuaGroup::West,
            },
            KuaFixture {
                year: 2000,
                gender: Gender::Female,
                expected_kua: 7,
                expected_group: KuaGroup::West,
            },
            KuaFixture {
                year: 2020,
                gender: Gender::Male,
                expected_kua: 8,
                expected_group: KuaGroup::West,
            },
            KuaFixture {
                year: 2020,
                gender: Gender::Female,
                expected_kua: 9,
                expected_group: KuaGroup::East,
            },
            KuaFixture {
                year: 2050,
                gender: Gender::Male,
                expected_kua: 2,
                expected_group: KuaGroup::West,
            },
            KuaFixture {
                year: 2050,
                gender: Gender::Female,
                expected_kua: 3,
                expected_group: KuaGroup::East,
            },
        ];

        for fixture in fixtures {
            let result = compute_kua(fixture.year, fixture.gender);
            assert_eq!(
                result.kua, fixture.expected_kua,
                "Year {} {:?}: expected Kua {}, got {}",
                fixture.year, fixture.gender, fixture.expected_kua, result.kua
            );
            assert_eq!(
                result.group, fixture.expected_group,
                "Year {} {:?}: expected group {:?}, got {:?}",
                fixture.year, fixture.gender, fixture.expected_group, result.group
            );
        }
    }
}
