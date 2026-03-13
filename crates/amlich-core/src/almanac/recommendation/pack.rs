use super::types::{ActiveRecommendationPack, RecommendationPackMode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecommendationPackDescriptor {
    pub pack_id: &'static str,
    pub version: &'static str,
    pub source_family: &'static str,
    pub mode: RecommendationPackMode,
}

impl RecommendationPackDescriptor {
    pub fn to_active(&self) -> ActiveRecommendationPack {
        ActiveRecommendationPack {
            pack_id: self.pack_id.to_string(),
            version: self.version.to_string(),
            source_family: self.source_family.to_string(),
            mode: self.mode.clone(),
        }
    }
}

pub const fn recommendation_pack_descriptors() -> &'static [RecommendationPackDescriptor] {
    &[super::packs::nhi_thap_bat_tu::NHI_THAP_BAT_TU_PACK]
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecommendationPackLookupError {
    UnknownPackId(String),
    DuplicatePackId(String),
    UnsupportedPackId(String),
}

impl std::fmt::Display for RecommendationPackLookupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownPackId(id) => write!(f, "unknown recommendation pack id: {id}"),
            Self::DuplicatePackId(id) => {
                write!(f, "duplicate recommendation pack id: {id}")
            }
            Self::UnsupportedPackId(id) => {
                write!(f, "unsupported recommendation pack id: {id}")
            }
        }
    }
}

impl std::error::Error for RecommendationPackLookupError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptor_converts_to_active_pack() {
        let descriptor = RecommendationPackDescriptor {
            pack_id: "pack.nhi_thap_bat_tu.v1",
            version: "v1",
            source_family: "nhi_thap_bat_tu",
            mode: RecommendationPackMode::Advisory,
        };

        let active = descriptor.to_active();
        assert_eq!(active.pack_id, "pack.nhi_thap_bat_tu.v1");
        assert_eq!(active.source_family, "nhi_thap_bat_tu");
    }
}
