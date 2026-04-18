/**
 * Barrel re-export for insight-related types used by desktop UI.
 */

export type {
  // Domain types
  Lang,
  HolidayInfo,
  GoodHour,
  DayCell,
  MonthData,
  BilingualText,
  BilingualList,
  DayForInsight,
} from "./domain";

export type {
  FestivalInsightDto,
  HolidayInsightDto,
  CanChiInsightDto,
  DayGuidanceInsightDto,
  TietKhiInsightDto,
  DayInsightDto,
} from "./insight-dto";

export type {
  RecommendationBucketDto,
  RecommendationSeverityDto,
  RecommendationEvidenceSourceDto,
  ActivityLabelDto,
  RecommendationEvidenceDto,
  RecommendationReasonDto,
  SynthesizedRecommendationDto,
  DailyRecommendationsDto,
  DayInfoDto,
} from "./day-info-dto";

export type {
  DateQuery,
  BaziQuery,
  BirthDataTierDto,
  UnavailableSectionDto,
  ReasoningNoteDto,
  ReasoningAxisScoreDto,
  ReasoningEvidenceEnvelopeDto,
  ReasoningNodeExportDto,
  ReasoningEdgeExportDto,
  ReasoningGraphExportDto,
  InitiationOpeningDecisionExportDto,
  PersonalDayQueryDto,
  PersonalDayChartDto,
  PersonalDayMetricsDto,
  PersonalDayAnalysisDto,
  PersonalDayReportDto,
  BranchRelationDto,
  PillarInteractionDto,
  DayPersonMatrixDto,
  ElementResonanceEntryDto,
  ElementResonanceMatrixDto,
  PersonalHourEntryDto,
  PersonalHourMatrixDto,
  DirectionEntryDto,
  DirectionMergeMatrixDto,
  DomainDayBoostEntryDto,
  DomainDayBoostMatrixDto,
  PersonalDayMatrixReportDto,
} from "./personal-day-dto";
