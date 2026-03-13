#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecommendationProfile {
    pub profile_id: String,
    pub birth_year: Option<i32>,
    pub gender: Option<String>,
    pub event_kind: Option<String>,
    pub locale: Option<String>,
}

impl RecommendationProfile {
    pub fn inline_event(profile_id: &str, event_kind: &str) -> Self {
        Self {
            profile_id: profile_id.to_string(),
            birth_year: None,
            gender: None,
            event_kind: Some(event_kind.to_string()),
            locale: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_inline_event_profile() {
        let profile = RecommendationProfile::inline_event("session", "contract_signing");
        assert_eq!(profile.profile_id, "session");
        assert_eq!(profile.event_kind.as_deref(), Some("contract_signing"));
    }
}
