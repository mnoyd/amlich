pub mod activity;
pub mod event_kind;
pub mod evidence;
pub mod matrix;
pub mod pack;
pub mod packs;
pub mod policy;
pub mod profile;
pub mod rules;
pub mod synthesize;
pub mod types;

pub use activity::{
    normalize_activity_alias, ActivityCategory, ActivityId, ActivityLabel, NormalizedActivity,
};
pub use evidence::{BaseDirection, BaseEvidenceHit};
pub use pack::{RecommendationPackDescriptor, RecommendationPackLookupError};
pub use synthesize::{
    collect_recommendation_hits, synthesize_base_daily_recommendations,
    synthesize_daily_recommendations, synthesize_daily_recommendations_with_layers,
    RecommendationHit, RecommendationLayer, RecommendationLayerHit, RecommendationSynthesisContext,
};
pub use types::{
    ActiveRecommendationPack, DailyRecommendations, RecommendationBucket, RecommendationEvidence,
    RecommendationEvidenceSource, RecommendationPackMode, RecommendationReason,
    RecommendationScope, RecommendationSeverity, SynthesizedRecommendation,
};
