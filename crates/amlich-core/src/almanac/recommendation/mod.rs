pub mod activity;
pub mod evidence;
pub mod rules;
pub mod synthesize;
pub mod types;

pub use activity::{
    normalize_activity_alias, ActivityCategory, ActivityId, ActivityLabel, NormalizedActivity,
};
pub use evidence::{BaseDirection, BaseEvidenceHit};
pub use synthesize::synthesize_base_daily_recommendations;
pub use types::{
    DailyRecommendations, RecommendationBucket, RecommendationEvidence,
    RecommendationEvidenceSource, RecommendationReason, RecommendationScope,
    RecommendationSeverity, SynthesizedRecommendation,
};
