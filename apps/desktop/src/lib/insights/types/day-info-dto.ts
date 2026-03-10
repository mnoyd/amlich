export type RecommendationBucketDto = "nen" | "co_the" | "tranh" | "ky_manh";
export type RecommendationSeverityDto = "primary" | "supporting" | "override";
export type RecommendationEvidenceSourceDto =
  | "day_guidance"
  | "truc"
  | "stars"
  | "day_deity"
  | "taboo"
  | "xung_hop"
  | "tiet_khi"
  | "gio_hoang_dao"
  | "travel"
  | "product_rule";

export type ActivityLabelDto = {
  vi: string;
  en: string;
};

export type RecommendationEvidenceDto = {
  source: RecommendationEvidenceSourceDto;
  code: string;
  note: string;
};

export type RecommendationReasonDto = {
  rule_id: string;
  severity: RecommendationSeverityDto;
  summary_vi: string;
  summary_en: string;
  evidence: RecommendationEvidenceDto;
};

export type SynthesizedRecommendationDto = {
  activity_id: string;
  label: ActivityLabelDto;
  bucket: RecommendationBucketDto;
  reasons: RecommendationReasonDto[];
};

export type DailyRecommendationsDto = {
  ruleset_id: string;
  ruleset_version: string;
  profile: string;
  scope: "general_day";
  version: string;
  summary_vi: string;
  summary_en: string;
  activities: SynthesizedRecommendationDto[];
};

export type DayInfoDto = {
  ruleset_id: string;
  ruleset_version: string;
  profile: string;
  daily_recommendations: DailyRecommendationsDto;
};
