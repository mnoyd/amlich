pub mod activity;
pub mod evidence;
pub mod rules;
pub mod synthesize;
pub mod types;

pub use activity::{
    normalize_activity_alias, ActivityCategory, ActivityId, ActivityLabel, NormalizedActivity,
};
pub use evidence::{BaseDirection, BaseEvidenceHit};
pub use synthesize::{
    synthesize_base_daily_recommendations, synthesize_daily_recommendations,
    synthesize_daily_recommendations_with_layers, RecommendationLayer, RecommendationLayerHit,
    RecommendationSynthesisContext,
};
pub use types::{
    DailyRecommendations, RecommendationBucket, RecommendationEvidence,
    RecommendationEvidenceSource, RecommendationReason, RecommendationScope,
    RecommendationSeverity, SynthesizedRecommendation,
};
