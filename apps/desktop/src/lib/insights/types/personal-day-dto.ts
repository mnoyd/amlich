export type DateQuery = {
  day: number;
  month: number;
  year: number;
  timezone?: number | null;
  ruleset_id?: string | null;
  event_kind?: string | null;
  enabled_pack_ids?: string[];
};

export type BaziQuery = {
  day: number;
  month: number;
  year: number;
  hour: number;
  minute: number;
  timezone?: number | null;
  longitude?: number | null;
  use_solar_time?: boolean;
  gender?: string | null;
};

export type BirthDataTierDto = "anonymous" | "date" | "datetime";

export type UnavailableSectionDto = {
  section: string;
  reason: string;
  required_fields: string[];
};

export type ReasoningNoteDto = {
  node_id?: string | null;
  summary_vi: string;
  tags: string[];
};

export type ReasoningAxisScoreDto = {
  axis: string;
  score: number;
  strongest_node_id?: string | null;
  strongest_summary_vi?: string | null;
};

export type ReasoningEvidenceEnvelopeDto = {
  source_family: string;
  source_id: string;
  method: string;
  note?: string | null;
};

export type ReasoningNodeExportDto = {
  id: string;
  kind: string;
  axis?: string | null;
  severity?: string | null;
  tags: string[];
  summary_vi: string;
  evidence: ReasoningEvidenceEnvelopeDto[];
};

export type ReasoningEdgeExportDto = {
  from_node_id: string;
  to_node_id: string;
  effect: string;
  weight: number;
  justification: string;
  evidence: ReasoningEvidenceEnvelopeDto[];
  tags: string[];
};

export type ReasoningGraphExportDto = {
  action_id: string;
  nodes: ReasoningNodeExportDto[];
  edges: ReasoningEdgeExportDto[];
};

export type InitiationOpeningDecisionExportDto = {
  primary_conclusion: string;
  recommendation_bucket: string;
  confidence: string;
  context_is_clear: boolean;
  semantic: string;
  strongest_supports: ReasoningNoteDto[];
  strongest_resistances: ReasoningNoteDto[];
  override_factors: ReasoningNoteDto[];
  conflict_notes: ReasoningNoteDto[];
  suggested_hours: string[];
  suggested_directions: string[];
  axis_scores: ReasoningAxisScoreDto[];
};

export type PersonalDayQueryDto = {
  date: DateQuery;
  birth_year?: number | null;
  birth_month?: number | null;
  birth_day?: number | null;
  gender?: string | null;
};

export type PersonalDayChartDto = {
  input: PersonalDayQueryDto;
  tier: BirthDataTierDto;
};

export type PersonalDayMetricsDto = {
  tier: BirthDataTierDto;
  profile_completeness: number;
  available_sections: string[];
  unavailable_sections: UnavailableSectionDto[];
  has_personal_recommendations: boolean;
};

export type PersonalDayAdvisoryDto = {
  summary: string;
  severity: string;
  top_signals: string[];
  why_this_matters: string[];
  recommended_actions: string[];
  priority_order: string[];
  highlights: string[];
  cautions: string[];
  reasoning_bucket?: string | null;
  reasoning_confidence?: string | null;
};

export type PersonalDayAnalysisDto = {
  tier: BirthDataTierDto;
  decision_export?: InitiationOpeningDecisionExportDto | null;
  graph?: ReasoningGraphExportDto | null;
  unavailable_sections: UnavailableSectionDto[];
};

export type PersonalDayReportDto = {
  summary: string;
  severity: string;
  top_signals: string[];
  chart: PersonalDayChartDto;
  decision_export?: InitiationOpeningDecisionExportDto | null;
  graph?: ReasoningGraphExportDto | null;
  analysis: PersonalDayAnalysisDto;
  computed_metrics: PersonalDayMetricsDto;
  advisory: PersonalDayAdvisoryDto;
};

export type BranchRelationDto = {
  luc_xung: boolean;
  luc_hop: boolean;
  tam_hop: boolean;
  tuong_hai: boolean;
  tuong_hinh: boolean;
};

export type PillarInteractionDto = {
  pillar: string;
  pillar_canchi: string;
  thap_than: string;
  branch_relation: BranchRelationDto;
  element_interaction: string;
};

export type DayPersonMatrixDto = {
  day_canchi: string;
  day_master: string;
  day_to_day_master: string;
  pillars: PillarInteractionDto[];
};

export type ElementResonanceEntryDto = {
  element: string;
  personal_score: number;
  relation_to_day: number;
  season_factor: number;
  effective_resonance: number;
  is_deficit: boolean;
  day_helps_deficit: boolean;
};

export type ElementResonanceMatrixDto = {
  day_canchi: string;
  day_element: string;
  month_chi: string;
  season_factor: number;
  entries: ElementResonanceEntryDto[];
  net_resonance: number;
};

export type PersonalHourEntryDto = {
  chi_index: number;
  chi: string;
  canchi: string;
  time_range: string;
  is_hoang_dao: boolean;
  star_name: string;
  thap_than_to_day_master: string;
  branch_relation_to_birth_hour: BranchRelationDto;
  element_interaction: string;
  supports_weak_element: boolean;
  score: number;
};

export type PersonalHourMatrixDto = {
  day_canchi: string;
  day_master: string;
  birth_hour_chi: string;
  weak_element: string;
  hours: PersonalHourEntryDto[];
};

export type DirectionEntryDto = {
  direction: string;
  signals: string[];
  favorable_count: number;
  unfavorable_count: number;
  net_score: number;
};

export type DirectionMergeMatrixDto = {
  day_canchi: string;
  kua_number: number;
  entries: DirectionEntryDto[];
};

export type DomainDayBoostEntryDto = {
  domain: string;
  base_score: number;
  day_modifier: number;
  han_penalty: number;
  boosted_score: number;
};

export type DomainDayBoostMatrixDto = {
  day_canchi: string;
  entries: DomainDayBoostEntryDto[];
};

export type PersonalDayMatrixReportDto = {
  input: {
    birth: BaziQuery;
    date: DateQuery;
  };
  tier: BirthDataTierDto;
  day_person: DayPersonMatrixDto;
  element_resonance: ElementResonanceMatrixDto;
  personal_hours?: PersonalHourMatrixDto | null;
  direction_merge?: DirectionMergeMatrixDto | null;
  domain_day_boost?: DomainDayBoostMatrixDto | null;
  unavailable_sections: UnavailableSectionDto[];
};
