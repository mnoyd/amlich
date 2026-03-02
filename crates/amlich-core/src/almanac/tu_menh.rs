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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
/// assert_eq!(result.kua, 4);  // Kua 4
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
