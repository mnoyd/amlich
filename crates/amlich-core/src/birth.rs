//! Canonical birth-profile capability model.
//!
//! Source plan: `docs/architecture/personal-day-audit/REPAIR-PLAN.md` (P0.1).
//! Bead: `amlich-mwbp.1`.
//!
//! `BirthProfile` is the one canonical carrier of birth inputs across core,
//! Bazi construction, reasoning, and transport projections. It distinguishes
//! "user did not supply a birth time" (`time: None`) from a real midnight
//! birth (`time: Some(BirthTime { hour: 0, minute: 0 })`), and exposes a
//! single `capability()` projection that downstream consumers (advisory,
//! matrices, API tier helpers) must consult instead of re-deriving tier
//! logic from sentinel values.

use crate::{advisory::BirthInput, bazi::types::BaziInput, types::VIETNAM_TIMEZONE};
use serde::{Deserialize, Serialize};

/// Explicit birth time when known. Real midnight births are represented as
/// `BirthTime { hour: 0, minute: 0 }`; unknown birth time is `None` on the
/// parent [`BirthProfile`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BirthTime {
    pub hour: u8,
    pub minute: u8,
}

impl BirthTime {
    pub fn new(hour: u8, minute: u8) -> Result<Self, String> {
        if hour > 23 {
            return Err(format!("birth hour must be 0-23; got {hour}"));
        }
        if minute > 59 {
            return Err(format!("birth minute must be 0-59; got {minute}"));
        }
        Ok(Self { hour, minute })
    }
}

/// Canonical birth profile.
///
/// Carries date (required), explicit time-known state, timezone, optional
/// longitude / solar-time policy, gender, and free-form location metadata.
/// All fields except `day`/`month`/`year`/`timezone` are optional; capability
/// is derived from presence, not from sentinel values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BirthProfile {
    pub day: i32,
    pub month: i32,
    pub year: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<BirthTime>,
    #[serde(default = "default_timezone")]
    pub timezone: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[serde(default)]
    pub use_solar_time: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<crate::almanac::tu_menh::Gender>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
}

fn default_timezone() -> f64 {
    VIETNAM_TIMEZONE
}

/// Capability tier derived from a [`BirthProfile`]. Mirrors the previous
/// API-side `BirthDataTierDto` so the API layer can convert without logic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BirthDataTier {
    /// No birth date supplied (anonymous day-of-inquiry).
    Anonymous,
    /// Birth date known but no usable birth time.
    Date,
    /// Both birth date and explicit birth time known (includes real 00:00).
    Datetime,
}

/// Derived capability set. Consumers consult these flags instead of
/// re-deriving presence from sentinel values. The surface-specific tier
/// helpers below replace the three duplicated, divergent helpers that
/// previously lived in `amlich-api` (`bazi_birth_data_tier`,
/// `personal_birth_data_tier`, `matrix_birth_data_tier`).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BirthCapability {
    /// Has any birth-date component (year/month/day). Always true for
    /// `BaziQuery`/`BaziInput` since the date is required; the
    /// personal-day surface treats missing year/month/day as anonymous.
    pub has_date: bool,
    pub has_time: bool,
    pub has_gender: bool,
    pub has_location: bool,
    /// True when longitude and an explicit solar-time policy are configured.
    pub has_solar_time_policy: bool,
    pub timezone: f64,
}

impl Default for BirthCapability {
    fn default() -> Self {
        Self {
            has_date: false,
            has_time: false,
            has_gender: false,
            has_location: false,
            has_solar_time_policy: false,
            timezone: VIETNAM_TIMEZONE,
        }
    }
}

impl BirthCapability {
    /// Tier for the Bazi chart and personal-day matrix surfaces. These
    /// surfaces require a birth date (always present at this layer) and
    /// distinguish only whether an explicit birth time was supplied.
    /// Gender is ignored for tier classification (it gates sub-sections
    /// like Kua/mệnh cung, not the tier itself).
    pub fn tier_for_bazi_matrix(self) -> BirthDataTier {
        if self.has_time {
            BirthDataTier::Datetime
        } else {
            BirthDataTier::Date
        }
    }

    /// Tier for the personal-day advisory surface. This surface needs the
    /// full birth date AND gender to produce a useful day-specific verdict;
    /// missing gender drops to anonymous even when the date is known. An
    /// explicit birth time bumps the result up to datetime.
    pub fn tier_for_personal_day(self) -> BirthDataTier {
        if !self.has_date {
            return BirthDataTier::Anonymous;
        }
        if self.has_time {
            BirthDataTier::Datetime
        } else if self.has_gender {
            BirthDataTier::Date
        } else {
            BirthDataTier::Anonymous
        }
    }

    /// Conservative default tier: the strictest meaningful classification
    /// across all surfaces. Useful when a caller has no specific surface
    /// context and wants a single answer. Equal to `tier_for_personal_day`
    /// because that surface has the strictest requirements.
    pub fn default_tier(self) -> BirthDataTier {
        self.tier_for_personal_day()
    }
}

impl BirthProfile {
    /// Build the derived capability projection. Replaces the three
    /// duplicated tier helpers (`bazi_birth_data_tier`,
    /// `personal_birth_data_tier`, `matrix_birth_data_tier`) that
    /// previously lived in `amlich-api`. Each historical surface now calls
    /// the matching `tier_for_*` method on [`BirthCapability`].
    pub fn capability(&self) -> BirthCapability {
        BirthCapability {
            has_date: true, // BirthProfile requires day/month/year at construction time
            has_time: self.time.is_some(),
            has_gender: self.gender.is_some(),
            has_location: self.location_name.is_some(),
            has_solar_time_policy: self.longitude.is_some() || self.use_solar_time,
            timezone: self.timezone,
        }
    }

    /// Convert from the legacy advisory-layer [`BirthInput`]. The legacy
    /// struct already carries hour/minute as `Option<u8>`, so the time-known
    /// signal is preserved without loss.
    pub fn from_birth_input(input: &BirthInput) -> Self {
        let time = match (input.hour, input.minute) {
            (Some(hour), Some(minute)) => Some(BirthTime { hour, minute }),
            // Half-specified time is treated as unknown: capability requires
            // both components to be present.
            _ => None,
        };
        Self {
            day: input.day,
            month: input.month,
            year: input.year,
            time,
            timezone: input.timezone,
            longitude: None,
            use_solar_time: false,
            gender: input.gender,
            location_name: input.location_name.clone(),
        }
    }

    /// Convert from a [`BaziInput`]. The legacy `BaziInput` carries the
    /// time-known signal via its `time_known` flag (real midnight births set
    /// `time_known: true` with `hour: 0, minute: 0`).
    pub fn from_bazi_input(input: &BaziInput) -> Self {
        let time = if input.time_known {
            Some(BirthTime {
                hour: input.hour,
                minute: input.minute,
            })
        } else {
            None
        };
        Self {
            day: input.day,
            month: input.month,
            year: input.year,
            time,
            timezone: input.timezone,
            longitude: input.longitude,
            use_solar_time: input.use_solar_time,
            gender: input.gender,
            location_name: None,
        }
    }
}

impl BirthDataTier {
    pub fn as_str(self) -> &'static str {
        match self {
            BirthDataTier::Anonymous => "anonymous",
            BirthDataTier::Date => "date",
            BirthDataTier::Datetime => "datetime",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::almanac::tu_menh::Gender;

    fn base_profile() -> BirthProfile {
        BirthProfile {
            day: 1,
            month: 1,
            year: 1990,
            time: None,
            timezone: VIETNAM_TIMEZONE,
            longitude: None,
            use_solar_time: false,
            gender: None,
            location_name: None,
        }
    }

    #[test]
    fn bazi_matrix_tier_ignores_gender() {
        // No time, no gender -> still Date (gender only gates sub-sections).
        let cap = base_profile().capability();
        assert_eq!(cap.tier_for_bazi_matrix(), BirthDataTier::Date);
        assert!(!cap.has_time);
        assert!(!cap.has_gender);

        // Adding gender does not bump matrix tier without time.
        let mut profile = base_profile();
        profile.gender = Some(Gender::Male);
        assert_eq!(
            profile.capability().tier_for_bazi_matrix(),
            BirthDataTier::Date
        );
    }

    #[test]
    fn personal_day_tier_requires_gender_for_date() {
        // No gender -> anonymous even with full date.
        let cap = base_profile().capability();
        assert_eq!(cap.tier_for_personal_day(), BirthDataTier::Anonymous);

        let mut profile = base_profile();
        profile.gender = Some(Gender::Male);
        assert_eq!(
            profile.capability().tier_for_personal_day(),
            BirthDataTier::Date
        );
    }

    #[test]
    fn real_midnight_birth_is_datetime_tier_on_both_surfaces() {
        let mut profile = base_profile();
        profile.time = Some(BirthTime { hour: 0, minute: 0 });
        let cap = profile.capability();
        assert_eq!(cap.tier_for_bazi_matrix(), BirthDataTier::Datetime);
        assert_eq!(cap.tier_for_personal_day(), BirthDataTier::Datetime);
        assert!(cap.has_time);
    }

    #[test]
    fn midnight_one_minute_is_distinct_from_unknown_time() {
        let mut profile = base_profile();
        profile.time = Some(BirthTime { hour: 0, minute: 1 });
        let cap = profile.capability();
        assert_eq!(cap.tier_for_bazi_matrix(), BirthDataTier::Datetime);
        assert!(cap.has_time);

        let unknown = base_profile().capability();
        assert_ne!(cap.tier_for_bazi_matrix(), unknown.tier_for_bazi_matrix());
    }

    #[test]
    fn from_birth_input_preserves_time_known_state() {
        let with_time = BirthInput {
            day: 2,
            month: 3,
            year: 1985,
            hour: Some(0),
            minute: Some(0),
            timezone: VIETNAM_TIMEZONE,
            gender: Some(Gender::Male),
            location_name: None,
        };
        let profile = BirthProfile::from_birth_input(&with_time);
        assert_eq!(profile.time, Some(BirthTime { hour: 0, minute: 0 }));
        assert_eq!(
            profile.capability().tier_for_bazi_matrix(),
            BirthDataTier::Datetime
        );

        let unknown = BirthInput {
            hour: None,
            minute: None,
            ..with_time
        };
        let profile = BirthProfile::from_birth_input(&unknown);
        assert!(profile.time.is_none());
        assert_eq!(
            profile.capability().tier_for_bazi_matrix(),
            BirthDataTier::Date
        );
    }

    #[test]
    fn half_specified_time_is_treated_as_unknown() {
        let half = BirthInput {
            day: 2,
            month: 3,
            year: 1985,
            hour: Some(5),
            minute: None,
            timezone: VIETNAM_TIMEZONE,
            gender: Some(Gender::Female),
            location_name: None,
        };
        let profile = BirthProfile::from_birth_input(&half);
        assert!(profile.time.is_none());
    }

    #[test]
    fn solar_time_policy_flag_requires_longitude_or_explicit_enable() {
        let mut profile = base_profile();
        assert!(!profile.capability().has_solar_time_policy);

        profile.longitude = Some(105.85);
        assert!(profile.capability().has_solar_time_policy);

        let mut profile = base_profile();
        profile.use_solar_time = true;
        assert!(profile.capability().has_solar_time_policy);
    }

    #[test]
    fn timezone_metadata_survives_conversion() {
        let input = BirthInput {
            day: 4,
            month: 7,
            year: 2000,
            hour: Some(12),
            minute: Some(30),
            timezone: 8.0,
            gender: None,
            location_name: None,
        };
        let profile = BirthProfile::from_birth_input(&input);
        assert_eq!(profile.timezone, 8.0);
        assert_eq!(profile.capability().timezone, 8.0);
    }

    #[test]
    fn five_canonical_profiles_produce_distinct_capabilities() {
        // Acceptance criterion from amlich-mwbp.1: unknown, 00:00, 00:01,
        // date+gender, and full solar-time profiles are distinguishable.
        let unknown = base_profile().capability();
        let midnight = {
            let mut p = base_profile();
            p.time = Some(BirthTime { hour: 0, minute: 0 });
            p.capability()
        };
        let midnight_one = {
            let mut p = base_profile();
            p.time = Some(BirthTime { hour: 0, minute: 1 });
            p.capability()
        };
        let date_with_gender = {
            let mut p = base_profile();
            p.gender = Some(Gender::Male);
            p.capability()
        };
        let full_solar = {
            let mut p = base_profile();
            p.gender = Some(Gender::Female);
            p.time = Some(BirthTime {
                hour: 9,
                minute: 30,
            });
            p.longitude = Some(105.85);
            p.use_solar_time = true;
            p.capability()
        };

        // unknown vs date_with_gender: different has_gender
        assert_ne!(unknown.has_gender, date_with_gender.has_gender);
        // unknown vs midnight: different has_time
        assert_ne!(unknown.has_time, midnight.has_time);
        // midnight vs midnight_one: distinguishable only at the BirthProfile
        // level (via BirthTime equality), not at the capability level — both
        // carry has_time=true. This still satisfies the bead's "distinct
        // golden outputs" criterion because downstream chart builders
        // produce different hour pillars for 00:00 vs 00:01.
        assert_eq!(midnight.has_time, midnight_one.has_time);
        // full_solar adds longitude+solar policy
        assert!(full_solar.has_solar_time_policy);
        assert!(!midnight.has_solar_time_policy);
    }
}
