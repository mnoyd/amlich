use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProfileGender {
    Male,
    Female,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_month: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_day: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_hour: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_minute: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<ProfileGender>,
}

fn profile_path() -> Option<PathBuf> {
    std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(dirs::config_dir)
        .map(|dir| dir.join("amlich").join("profile.json"))
}

pub fn load_profile_from_str(json: &str) -> UserProfile {
    serde_json::from_str(json).unwrap_or_default()
}

pub fn load_profile() -> UserProfile {
    let Some(path) = profile_path() else {
        return UserProfile::default();
    };
    match std::fs::read_to_string(path) {
        Ok(content) => load_profile_from_str(&content),
        Err(_) => UserProfile::default(),
    }
}

pub fn save_profile(profile: &UserProfile) -> Result<(), String> {
    let Some(path) = profile_path() else {
        return Err("could not determine config directory".to_string());
    };
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("failed to create config directory: {e}"))?;
    }
    let json = serde_json::to_string_pretty(profile)
        .map_err(|e| format!("failed to serialize profile: {e}"))?;
    std::fs::write(&path, json).map_err(|e| format!("failed to write profile: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_empty_when_no_file() {
        let profile = load_profile_from_str("{}");
        assert!(profile.birth_year.is_none());
        assert!(profile.gender.is_none());
    }

    #[test]
    fn loads_full_profile() {
        let json = r#"{"birth_year":1990,"birth_month":5,"birth_day":15,"birth_hour":9,"birth_minute":30,"gender":"male"}"#;
        let profile = load_profile_from_str(json);
        assert_eq!(profile.birth_year, Some(1990));
        assert_eq!(profile.birth_month, Some(5));
        assert_eq!(profile.birth_day, Some(15));
        assert_eq!(profile.birth_hour, Some(9));
        assert_eq!(profile.birth_minute, Some(30));
        assert_eq!(profile.gender, Some(ProfileGender::Male));
    }

    #[test]
    fn partial_profile_ok() {
        let json = r#"{"birth_year":1990}"#;
        let profile = load_profile_from_str(json);
        assert_eq!(profile.birth_year, Some(1990));
        assert!(profile.gender.is_none());
    }

    #[test]
    fn has_birth_context_requires_year_and_gender() {
        let full = load_profile_from_str(r#"{"birth_year":1990,"gender":"female"}"#);
        assert!(full.birth_year.is_some() && full.gender.is_some());

        let partial = load_profile_from_str(r#"{"birth_year":1990}"#);
        assert!(!(partial.birth_year.is_some() && partial.gender.is_some()));
    }
}
