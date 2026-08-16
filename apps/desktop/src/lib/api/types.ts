export interface DateQuery {
    day: number;
    month: number;
    year: number;
    timezone?: number | null;
    ruleset_id?: string | null;
    event_kind?: string | null;
    enabled_pack_ids?: string[];
}

export interface BaziQuery {
    day: number;
    month: number;
    year: number;
    hour: number;
    minute: number;
    timezone?: number | null;
    longitude?: number | null;
    use_solar_time?: boolean;
    gender?: string | null;
}

export interface SolarDateDto {
    day: number;
    month: number;
    year: number;
    date_string: string;
    day_of_week: number;
    day_of_week_name: string;
}

export interface LunarDateDto {
    day: number;
    month: number;
    year: number;
    is_leap_month: boolean;
    date_string: string;
}

export interface NguHanhDto {
    can: string;
    chi: string;
}

export interface CanChiItemDto {
    can_index: number;
    chi_index: number;
    can: string;
    chi: string;
    full: string;
    con_giap: string;
    ngu_hanh: NguHanhDto;
}

export interface CanChiInfoDto {
    day: CanChiItemDto;
    month: CanChiItemDto;
    year: CanChiItemDto;
    full: string;
}

export interface TietKhiDto {
    index: number;
    name: string;
    description: string;
    longitude: number;
    current_longitude: number;
    season: string;
}

export interface HourInfoDto {
    hour_index: number;
    hour_chi: string;
    time_range: string;
    star: string;
    is_good: boolean;
}

export interface GioHoangDaoDto {
    day_chi: string;
    good_hour_count: number;
    good_hours: HourInfoDto[];
    all_hours: HourInfoDto[];
    summary: string;
}

export interface RuleEvidenceDto {
    source_id: string;
    method: string;
    profile: string;
}

export interface DayElementDto {
    na_am: string;
    element: string;
    can_element: string;
    chi_element: string;
    evidence?: RuleEvidenceDto | null;
}

export interface DayConflictDto {
    opposing_chi: string;
    opposing_con_giap: string;
    tuoi_xung: string[];
    sat_huong: string;
    evidence?: RuleEvidenceDto | null;
}

export interface TravelDirectionDto {
    xuat_hanh_huong: string;
    tai_than: string;
    hy_than: string;
    evidence?: RuleEvidenceDto | null;
}

export interface DayStarDto {
    system: string;
    index: number;
    name: string;
    quality: string;
    evidence?: RuleEvidenceDto | null;
}

export interface StarRuleEvidenceDto {
    name: string;
    quality: string;
    category: string;
    source_id: string;
    method: string;
    profile: string;
}

export interface DayStarsDto {
    cat_tinh: string[];
    sat_tinh: string[];
    day_star?: DayStarDto | null;
    star_system?: string | null;
    evidence?: RuleEvidenceDto | null;
    matched_rules: StarRuleEvidenceDto[];
}

export interface DayDeityDto {
    name: string;
    classification: string;
    evidence?: RuleEvidenceDto | null;
}

export interface XungHopDto {
    luc_xung: string;
    tam_hop: string[];
    tu_hanh_xung: string[];
    liu_he?: string | null;
    xiang_hai?: string | null;
    xiang_xing?: string[] | null;
}

export interface TangCanDto {
    main: string;
    central: string;
    residual: string;
    strength: [number, number, number];
}

export interface TrucDto {
    index: number;
    name: string;
    quality: string;
    evidence?: RuleEvidenceDto | null;
}

export interface ThapThanResultDto {
    label: string;
    relation: string;
    same_polarity: boolean;
    evidence: RuleEvidenceDto;
}

export interface DayTenGodsDto {
    to_year_stem?: ThapThanResultDto | null;
    to_self?: ThapThanResultDto | null;
}

export interface DayTabooDto {
    rule_id: string;
    name: string;
    severity: string;
    reason: string;
    evidence?: RuleEvidenceDto | null;
}

export interface KuaResultDto {
    kua: number;
    group: string;
    favorable_directions: string[];
    unfavorable_directions: string[];
    convention: ConventionMetadataDto;
}

export interface ConventionMetadataDto {
    year_basis: string;
    kua_five_resolution: string;
    gender_encoding: string;
}

export interface DayFortuneDto {
    ruleset_id: string;
    ruleset_version: string;
    profile: string;
    day_element: DayElementDto;
    conflict: DayConflictDto;
    travel: TravelDirectionDto;
    stars: DayStarsDto;
    day_deity?: DayDeityDto | null;
    taboos: DayTabooDto[];
    xung_hop: XungHopDto;
    truc: TrucDto;
    tang_can?: TangCanDto | null;
    ten_gods?: DayTenGodsDto | null;
    tu_menh?: KuaResultDto | null;
}

export type RecommendationScopeDto = 'general_day';
export type RecommendationBucketDto = 'nen' | 'co_the' | 'tranh' | 'ky_manh';
export type RecommendationSeverityDto = 'primary' | 'supporting' | 'override';
export type RecommendationEvidenceSourceDto =
    | 'day_guidance'
    | 'truc'
    | 'stars'
    | 'day_deity'
    | 'taboo'
    | 'xung_hop'
    | 'tiet_khi'
    | 'gio_hoang_dao'
    | 'travel'
    | 'product_rule';

export interface ActivityLabelDto {
    vi: string;
    en: string;
}

export interface RecommendationEvidenceDto {
    source: RecommendationEvidenceSourceDto;
    code: string;
    note: string;
}

export interface RecommendationReasonDto {
    rule_id: string;
    severity: RecommendationSeverityDto;
    summary_vi: string;
    summary_en: string;
    evidence: RecommendationEvidenceDto;
}

export interface SynthesizedRecommendationDto {
    activity_id: string;
    label: ActivityLabelDto;
    bucket: RecommendationBucketDto;
    reasons: RecommendationReasonDto[];
}

export interface ActiveRecommendationPackDto {
    pack_id: string;
    version: string;
    source_family: string;
    mode: string;
}

export interface DailyRecommendationsDto {
    ruleset_id: string;
    ruleset_version: string;
    profile: string;
    scope: RecommendationScopeDto;
    version: string;
    summary_vi: string;
    summary_en: string;
    active_packs: ActiveRecommendationPackDto[];
    activities: SynthesizedRecommendationDto[];
}

export interface DayInfoDto {
    ruleset_id: string;
    ruleset_version: string;
    profile: string;
    solar: SolarDateDto;
    lunar: LunarDateDto;
    jd: number;
    canchi: CanChiInfoDto;
    tiet_khi: TietKhiDto;
    gio_hoang_dao: GioHoangDaoDto;
    day_fortune?: DayFortuneDto | null;
    daily_recommendations: DailyRecommendationsDto;
    contextual_recommendations?: DailyRecommendationsDto | null;
}

export interface LocalizedTextDto {
    vi: string;
    en: string;
}

export interface LocalizedListDto {
    vi: string[];
    en: string[];
}

export interface FoodInsightDto {
    name: LocalizedTextDto;
    description: LocalizedTextDto;
}

export interface TabooInsightDto {
    action: LocalizedTextDto;
    reason: LocalizedTextDto;
}

export interface ProverbInsightDto {
    text: string;
    meaning: LocalizedTextDto;
}

export interface RegionsInsightDto {
    north: LocalizedTextDto;
    central: LocalizedTextDto;
    south: LocalizedTextDto;
}

export interface FestivalInsightDto {
    names: LocalizedListDto;
    origin?: LocalizedTextDto | null;
    activities?: LocalizedListDto | null;
    food: FoodInsightDto[];
    taboos: TabooInsightDto[];
    proverbs: ProverbInsightDto[];
    regions?: RegionsInsightDto | null;
    category: string;
    is_major: boolean;
}

export interface HolidayInsightDto {
    names: LocalizedListDto;
    origin?: LocalizedTextDto | null;
    significance?: LocalizedTextDto | null;
    activities?: LocalizedListDto | null;
    traditions?: LocalizedListDto | null;
    food: FoodInsightDto[];
    taboos: TabooInsightDto[];
    proverbs: ProverbInsightDto[];
    regions?: RegionsInsightDto | null;
    category: string;
    is_major: boolean;
}

export interface UpcomingEventDto {
    name: string;
    days_left: number;
    is_lunar: boolean;
}

export interface ElementInsightDto {
    key: string;
    name: LocalizedTextDto;
    nature: LocalizedTextDto;
}

export interface CanInsightDto {
    name: string;
    element: string;
    meaning: LocalizedTextDto;
    nature: LocalizedTextDto;
}

export interface ChiInsightDto {
    name: string;
    animal: LocalizedTextDto;
    element: string;
    meaning: LocalizedTextDto;
    hours: string;
}

export interface CanChiInsightDto {
    can: CanInsightDto;
    chi: ChiInsightDto;
    element?: ElementInsightDto | null;
}

export interface DayGuidanceDto {
    good_for: LocalizedListDto;
    avoid_for: LocalizedListDto;
}

export interface TietKhiInsightDto {
    id: string;
    name: LocalizedTextDto;
    longitude: number;
    meaning: LocalizedTextDto;
    astronomy: LocalizedTextDto;
    agriculture: LocalizedListDto;
    health: LocalizedListDto;
    weather: LocalizedTextDto;
}

export interface NaAmInsightDto {
    na_am: string;
    element: string;
    meaning: LocalizedTextDto;
}

export interface TrucInsightDto {
    name: string;
    quality: string;
    meaning: LocalizedTextDto;
    good_for: LocalizedListDto;
    avoid_for: LocalizedListDto;
}

export interface DayDeityInsightDto {
    name: string;
    classification: string;
    classification_meaning: LocalizedTextDto;
    deity_meaning?: LocalizedTextDto | null;
}

export interface StarsInsightDto {
    cat_tinh: string[];
    sat_tinh: string[];
    day_star?: string | null;
    day_star_quality?: string | null;
}

export interface TabooInsightItemDto {
    name: string;
    severity: string;
    reason: string;
}

export interface TravelInsightDto {
    xuat_hanh_huong: string;
    tai_than: string;
    hy_than: string;
}

export interface XungHopInsightDto {
    luc_xung: string;
    tam_hop: string[];
    liu_he?: string | null;
    xiang_hai?: string | null;
}

export interface TangCanInsightDto {
    main: string;
    central: string;
    residual: string;
    strength: [number, number, number];
}

export interface TenGodsEntryInsightDto {
    label: string;
    name: LocalizedTextDto;
    meaning: LocalizedTextDto;
    relation: string;
    same_polarity: boolean;
}

export interface TenGodsInsightDto {
    to_year_stem?: TenGodsEntryInsightDto | null;
    to_self?: TenGodsEntryInsightDto | null;
}

export interface HourInsightEntryDto {
    chi: string;
    time_range: string;
    star: string;
}

export interface HoursInsightDto {
    good_hour_count: number;
    good_hours: HourInsightEntryDto[];
}

export interface TuMenhInsightDto {
    kua: number;
    group: string;
    trigram: LocalizedTextDto;
    direction: LocalizedTextDto;
    meaning: LocalizedTextDto;
    group_meaning: LocalizedTextDto;
    favorable_directions: string[];
    unfavorable_directions: string[];
}

export interface DaiVanPillarInsightDto {
    index: number;
    can_chi: string;
    start_age: number;
    end_age: number;
    element: string;
    element_meaning: LocalizedTextDto;
}

export interface DaiVanInsightDto {
    direction: string;
    direction_meaning: LocalizedTextDto;
    start_age: string;
    current_pillar?: DaiVanPillarInsightDto | null;
    all_pillars: DaiVanPillarInsightDto[];
    phases_meaning: LocalizedTextDto;
}

export interface CuuDieuInsightDto {
    star_index: number;
    star_name: string;
    quality: string;
    is_han: boolean;
    element: string;
}

export interface TamTaiInsightDto {
    in_tam_tai: boolean;
    year_position?: number | null;
    severity?: string | null;
    tam_hop_group: string[];
    tai_years: string[];
}

export interface KimLauInsightDto {
    in_kim_lau: boolean;
    category?: string | null;
    remainder: number;
    tuoi_mu: number;
}

export interface HoangOcInsightDto {
    position: number;
    position_name: string;
    is_good: boolean;
    tuoi_mu: number;
}

export interface ThaiTueConflictDto {
    kind: string;
    description: string;
}

export interface ThaiTueInsightDto {
    conflicts: ThaiTueConflictDto[];
    has_conflict: boolean;
}

export interface YearlyHanInsightDto {
    sao_han: CuuDieuInsightDto;
    tam_tai: TamTaiInsightDto;
    kim_lau: KimLauInsightDto;
    hoang_oc: HoangOcInsightDto;
    thai_tue: ThaiTueInsightDto;
    han_count: number;
    is_chong_han: boolean;
    severity: string;
}

export interface DayInsightDto {
    solar: SolarDateDto;
    lunar: LunarDateDto;
    festival?: FestivalInsightDto | null;
    holiday?: HolidayInsightDto | null;
    canchi?: CanChiInsightDto | null;
    day_guidance?: DayGuidanceDto | null;
    tiet_khi?: TietKhiInsightDto | null;
    na_am?: NaAmInsightDto | null;
    truc?: TrucInsightDto | null;
    day_deity?: DayDeityInsightDto | null;
    stars?: StarsInsightDto | null;
    taboos?: TabooInsightItemDto[] | null;
    travel?: TravelInsightDto | null;
    xung_hop?: XungHopInsightDto | null;
    tang_can?: TangCanInsightDto | null;
    ten_gods?: TenGodsInsightDto | null;
    hours?: HoursInsightDto | null;
    tu_menh?: TuMenhInsightDto | null;
    dai_van?: DaiVanInsightDto | null;
    yearly_han?: YearlyHanInsightDto | null;
}

export interface BaziCanChiDto {
    can: string;
    chi: string;
    full: string;
    can_index: number;
    chi_index: number;
}

export interface BaziLunarDateDto {
    day: number;
    month: number;
    year: number;
    is_leap: boolean;
}

export interface HiddenStemEntryDto {
    stem_symbol: string;
    stem_name?: string | null;
    strength: number;
    ten_god_to_day_master?: ThapThanResultDto | null;
}

export interface BaziPillarDto {
    kind: string;
    can_chi: BaziCanChiDto;
    hidden_stems: HiddenStemEntryDto[];
    na_am?: string | null;
    stem_relation_to_day_master?: ThapThanResultDto | null;
}

export interface BaziChartMetadataDto {
    timezone: number;
    use_solar_time: boolean;
    year_basis: string;
    month_basis: string;
    day_basis: string;
    hour_basis: string;
    hour_evidence?: RuleEvidenceDto | null;
}

export type BirthDataTierDto = 'anonymous' | 'date' | 'datetime';

export interface BaziChartDto {
    input: BaziQuery;
    tier: BirthDataTierDto;
    lunar_date: BaziLunarDateDto;
    day_master: BaziCanChiDto;
    pillars: BaziPillarDto[];
    metadata: BaziChartMetadataDto;
}

export interface ElementDistributionDto {
    moc: number;
    hoa: number;
    tho: number;
    kim: number;
    thuy: number;
}

export interface DayMasterStrengthDto {
    score: number;
    label: string;
    reasons: string[];
}

export interface ChartInteractionDto {
    kind: string;
    participants: string[];
    summary_vi: string;
}

export interface TenGodDistributionDto {
    ty_kien: number;
    kiep_tai: number;
    thuc_than: number;
    thuong_quan: number;
    chinh_tai: number;
    thien_tai: number;
    chinh_quan: number;
    that_sat: number;
    chinh_an: number;
    thien_an: number;
}

export interface UnavailableSectionDto {
    section: string;
    reason: string;
    required_fields: string[];
}

export interface BaziAnalysisDto {
    tier: BirthDataTierDto;
    element_distribution: ElementDistributionDto;
    day_master_strength: DayMasterStrengthDto;
    interactions: ChartInteractionDto[];
    ten_god_distribution: TenGodDistributionDto;
    unavailable_sections: UnavailableSectionDto[];
}

export interface BaziLuckPillarDto {
    index: number;
    can_chi: string;
    start_age: number;
    end_age: number;
    ten_god_to_day_master?: ThapThanResultDto | null;
}

export interface AnnualPillarDto {
    year: number;
    can_chi: string;
    branch: string;
    ten_god_to_day_master?: ThapThanResultDto | null;
    interactions: string[];
}

export interface MonthlyPillarDto {
    year: number;
    month: number;
    can_chi: string;
    branch: string;
    ten_god_to_day_master?: ThapThanResultDto | null;
    interactions: string[];
}

export interface BaziTimingDto {
    dai_van: BaziLuckPillarDto[];
    active_dai_van?: BaziLuckPillarDto | null;
    annual: AnnualPillarDto;
    monthly: MonthlyPillarDto[];
}

export interface UsefulGodDto {
    favorable_elements: string[];
    unfavorable_elements: string[];
    tentative_yong_shen?: string | null;
    tentative_xi_shen?: string | null;
    confidence: string;
    reasons: string[];
}

export interface BaziAdvisoryDomainsDto {
    career: string[];
    wealth: string[];
    relationship: string[];
    health: string[];
    timing: string[];
}

export interface BaziAdvisoryDto {
    summary: string;
    severity: string;
    top_signals: string[];
    why_this_matters: string[];
    recommended_actions: string[];
    priority_order: string[];
    useful_god_analysis: UsefulGodDto;
    summary_vi: string;
    warnings: string[];
    domains: BaziAdvisoryDomainsDto;
}

export interface BaziInteractionMetricDto {
    kind: string;
    participants: string[];
    impact: number;
}

export interface BaziScoreContributorDto {
    signal: string;
    delta: number;
}

export interface BaziDomainScoreDto {
    score: number;
    label: string;
    confidence: number;
    contributors: BaziScoreContributorDto[];
}

export interface BaziDomainScoresDto {
    career: BaziDomainScoreDto;
    wealth: BaziDomainScoreDto;
    relationship: BaziDomainScoreDto;
    health: BaziDomainScoreDto;
    timing: BaziDomainScoreDto;
}

export interface BaziTimingWindowScoreDto {
    month: number;
    score: number;
    label: string;
}

export interface BaziTimingMetricsDto {
    current_dai_van_alignment?: number | null;
    annual_alignment?: number | null;
    monthly_windows: BaziTimingWindowScoreDto[];
    activation_summary: string[];
}

export interface BaziCoreMetricsDto {
    day_master_strength_score: number;
    day_master_strength_label: string;
    season_support_score: number;
    same_element_score: number;
    resource_support_score: number;
    drain_pressure_score: number;
    control_pressure_score: number;
    element_balance_score: number;
}

export interface BaziStructureMetricsDto {
    dominant_elements: string[];
    weak_elements: string[];
    dominant_ten_gods: string[];
    interaction_score: number;
    notable_interactions: BaziInteractionMetricDto[];
    confidence: number;
}

export interface BaziComputedMetricsDto {
    tier: BirthDataTierDto;
    core_metrics: BaziCoreMetricsDto;
    structure_metrics: BaziStructureMetricsDto;
    domain_scores: BaziDomainScoresDto;
    timing_metrics: BaziTimingMetricsDto;
    unavailable_sections: UnavailableSectionDto[];
}

export interface BaziReportDto {
    summary: string;
    severity: string;
    top_signals: string[];
    why_this_matters: string[];
    recommended_actions: string[];
    priority_order: string[];
    chart: BaziChartDto;
    analysis: BaziAnalysisDto;
    timing?: BaziTimingDto | null;
    computed_metrics: BaziComputedMetricsDto;
    advisory: BaziAdvisoryDto;
}

export interface RulesetDefaultsDto {
    tz_offset: number;
    meridian?: string | null;
}

export interface RulesetSourceNoteDto {
    family: string;
    source_id: string;
    note: string;
}

export interface RulesetCatalogEntryDto {
    id: string;
    canonical_id: string;
    version: string;
    region: string;
    profile: string;
    schema_version: string;
    is_default: boolean;
    aliases: string[];
    defaults: RulesetDefaultsDto;
    source_notes: RulesetSourceNoteDto[];
}

export interface RecommendationPackCatalogEntryDto {
    pack_id: string;
    request_field: string;
    version: string;
    source_family: string;
    mode: string;
}

export interface HolidayDto {
    name: string;
    description: string;
    is_solar: boolean;
    lunar_day?: number | null;
    lunar_month?: number | null;
    lunar_year?: number | null;
    solar_day: number;
    solar_month: number;
    solar_year: number;
    category: string;
    is_major: boolean;
}

export type ActionIdDto = 'initiation_opening';
export type NodeKindDto = 'fact' | 'interpreted_signal' | 'decision_target';
export type InterpretedAxisDto =
    | 'support'
    | 'resistance'
    | 'stability'
    | 'personal_alignment'
    | 'timing_fit'
    | 'context_clarity';
export type EdgeEffectDto = 'supports' | 'weakens' | 'overrides' | 'conflicts_with' | 'conditions';
export type DecisionConfidenceDto = 'low' | 'medium' | 'high';
export type InitiationRecommendationBucketDto = 'avoid' | 'cautious' | 'mixed' | 'favorable';
export type ReasoningConclusionSemanticDto =
    | 'override_avoid'
    | 'override_cautious'
    | 'conflicted_cautious'
    | 'resistance_led_cautious'
    | 'favorable_clear'
    | 'favorable_contextual';
export type ReasoningEvidenceSourceFamilyDto =
    | 'snapshot'
    | 'interaction'
    | 'bazi'
    | 'bazi_observation'
    | 'personal_hour_matrix'
    | 'axis'
    | 'almanac_rule'
    | 'insight'
    | 'derived'
    | 'iching';
export type ReasoningNodeSeverityDto =
    | 'auspicious'
    | 'inauspicious'
    | 'hard_taboo'
    | 'soft_taboo'
    | 'hoang_dao'
    | 'hac_dao';
export type ReasoningEdgeJustificationDto =
    | 'favorable_day_signal'
    | 'truc_activity_support'
    | 'truc_activity_conflict'
    | 'day_deity_support'
    | 'star_support'
    | 'taboo_pressure'
    | 'taboo_stability_penalty'
    | 'taboo_context_penalty'
    | 'clash_pressure'
    | 'clash_stability_penalty'
    | 'hoang_dao_hour_support'
    | 'personal_day_alignment'
    | 'personal_hour_alignment'
    | 'mixed_signal_conflict'
    | 'available_context_support';

export interface InitiationOpeningDecisionDto {
    primary_conclusion: string;
    recommendation_bucket: InitiationRecommendationBucketDto;
    strongest_supports: string[];
    strongest_resistances: string[];
    override_factors: string[];
    conflict_notes: string[];
    confidence: DecisionConfidenceDto;
    context_is_clear: boolean;
    suggested_hours: string[];
    suggested_directions: string[];
}

export interface ReasoningEvidenceEnvelopeDto {
    source_family: ReasoningEvidenceSourceFamilyDto;
    source_id: string;
    method: string;
    note?: string | null;
}

export interface ReasoningNoteDto {
    node_id?: string | null;
    summary_vi: string;
    tags: string[];
    provenance: ReasoningEvidenceEnvelopeDto[];
}

export interface ReasoningNodeExportDto {
    id: string;
    kind: NodeKindDto;
    axis?: InterpretedAxisDto | null;
    severity?: ReasoningNodeSeverityDto | null;
    tags: string[];
    summary_vi: string;
    evidence: ReasoningEvidenceEnvelopeDto[];
}

export interface ReasoningEdgeExportDto {
    from_node_id: string;
    to_node_id: string;
    effect: EdgeEffectDto;
    weight: number;
    justification: ReasoningEdgeJustificationDto;
    evidence: ReasoningEvidenceEnvelopeDto[];
    tags: string[];
}

export interface ReasoningGraphExportDto {
    action_id: ActionIdDto;
    nodes: ReasoningNodeExportDto[];
    edges: ReasoningEdgeExportDto[];
}

export interface ReasoningAxisScoreDto {
    axis: InterpretedAxisDto;
    score: number;
    strongest_node_id?: string | null;
    strongest_summary_vi?: string | null;
}

export interface InitiationOpeningDecisionExportDto {
    primary_conclusion: string;
    recommendation_bucket: InitiationRecommendationBucketDto;
    confidence: DecisionConfidenceDto;
    context_is_clear: boolean;
    semantic: ReasoningConclusionSemanticDto;
    strongest_supports: ReasoningNoteDto[];
    strongest_resistances: ReasoningNoteDto[];
    override_factors: ReasoningNoteDto[];
    conflict_notes: ReasoningNoteDto[];
    suggested_hours: string[];
    suggested_directions: string[];
    axis_scores: ReasoningAxisScoreDto[];
}

export interface PersonalDayQueryDto {
    date: DateQuery;
    birth_year?: number | null;
    birth_month?: number | null;
    birth_day?: number | null;
    gender?: string | null;
}

export interface PersonalDayChartDto {
    input: PersonalDayQueryDto;
    tier: BirthDataTierDto;
    solar: SolarDateDto;
    lunar: LunarDateDto;
    canchi?: CanChiInsightDto | null;
    tiet_khi?: TietKhiInsightDto | null;
}

export interface PersonalDayAnalysisDto {
    tier: BirthDataTierDto;
    decision?: InitiationOpeningDecisionDto | null;
    decision_export?: InitiationOpeningDecisionExportDto | null;
    graph?: ReasoningGraphExportDto | null;
    ten_gods?: TenGodsInsightDto | null;
    xung_hop?: XungHopInsightDto | null;
    tang_can?: TangCanInsightDto | null;
    tu_menh?: TuMenhInsightDto | null;
    dai_van?: DaiVanInsightDto | null;
    yearly_han?: YearlyHanInsightDto | null;
    unavailable_sections: UnavailableSectionDto[];
}

export interface PersonalDayMetricsDto {
    tier: BirthDataTierDto;
    profile_completeness: number;
    available_sections: string[];
    unavailable_sections: UnavailableSectionDto[];
    has_personal_recommendations: boolean;
}

export interface PersonalDayNormalizedBirthDto {
    day: number;
    month: number;
    year: number;
    has_time: boolean;
    has_gender: boolean;
    has_location: boolean;
    has_solar_time_policy: boolean;
}

export interface PersonalDayAxisOutcomeDto {
    axis: string;
    score?: number | null;
    verdict: string;
    unavailable_reason?: string | null;
}

export interface PersonalDayAxesDto {
    generic_day_quality: PersonalDayAxisOutcomeDto;
    intent_fit: PersonalDayAxisOutcomeDto;
    personal_alignment: PersonalDayAxisOutcomeDto;
    annual_pressure: PersonalDayAxisOutcomeDto;
    evidence_coverage: PersonalDayAxisOutcomeDto;
}

export interface PersonalDayDecisionDto {
    bucket: string;
    confidence: string;
    semantic: string;
    primary_conclusion: string;
    decision_score?: number | null;
    context_is_clear: boolean;
}

export interface PersonalDayContributionDto {
    contribution_id: string;
    axis: string;
    polarity: string;
    strength: number;
    policy_id: string;
    policy_version: string;
    ruleset_id: string;
    ruleset_version: string;
    source_family: string;
    source_id: string;
    method: string;
    note?: string | null;
}

export interface PersonalDayFactorDto {
    factor_id: string;
    role: 'fact' | 'scored_feature' | 'veto' | 'explanation_only' | string;
    axis?: string | null;
    availability: 'complete' | 'unavailable' | string;
    unavailable_reason?: string | null;
    source_family: string;
    source_id: string;
    method: string;
    note?: string | null;
}

export interface PersonalDayEvidenceDto {
    has_chart: boolean;
    has_analysis: boolean;
    has_yearly_han: boolean;
    has_kua: boolean;
    recommendation_count: number;
}

// amlich-8tdm: Evidence Graph projection of the personal-day
// scoring trace. Optional on the wire (Rust serializes with
// skip_serializing_if = "Option::is_none"). When present, the
// graph carries the actual feature / weight / source / policy /
// veto state the policy computed.
export interface AssessmentTraceGraphNodeDto {
    node_id: string;
    concept: string;
    origin: 'fact' | 'interpreted' | 'decision' | string;
    cluster: string;
    label: string;
    severity?: string | null;
    tags?: string[];
    stable_key: string;
    policy_version: string;
    payload?: unknown;
}

export interface AssessmentTraceGraphEdgeDto {
    edge_id: string;
    from_node_id: string;
    to_node_id: string;
    concept: string;
    weight: number;
    veto_overrides_decision: boolean;
}

export interface AssessmentTraceContributorDto {
    feature_id: string;
    contribution_id: string;
    signed_value: number;
    applied_weight: number;
    contribution: number;
}

export interface AssessmentTraceAxisSummaryDto {
    axis: string;
    verdict: string;
    subtotal?: number | null;
    unavailable_reason?: string | null;
    contributors: AssessmentTraceContributorDto[];
}

export interface AssessmentTraceAxisWeightDto {
    axis: string;
    weight: number;
}

export interface AssessmentTraceDecisionSummaryDto {
    bucket: string;
    decision_score?: number | null;
    axis_weights: AssessmentTraceAxisWeightDto[];
    available_axes: string[];
    unavailable_axes: string[];
}

export interface AssessmentTraceVetoSummaryDto {
    veto_id: string;
    axis: string;
    reason: string;
    source_family: string;
    source_id: string;
    method: string;
    profile: string;
}

export interface AssessmentTraceInteractionSummaryDto {
    interaction_id: string;
    axis: string;
    value: number;
    weight: number;
    feature_ids: string[];
}

export interface AssessmentTraceGraphDto {
    policy_id: string;
    policy_version: string;
    ruleset_id: string;
    ruleset_version: string;
    node_count: number;
    edge_count: number;
    nodes: AssessmentTraceGraphNodeDto[];
    edges: AssessmentTraceGraphEdgeDto[];
    axes: AssessmentTraceAxisSummaryDto[];
    decision: AssessmentTraceDecisionSummaryDto;
    vetoes?: AssessmentTraceVetoSummaryDto[];
    interactions?: AssessmentTraceInteractionSummaryDto[];
}

export interface PersonalDayAssessmentDto {
    ruleset_id: string;
    ruleset_version: string;
    policy_id: string;
    policy_version: string;
    profile: string;
    intent: string;
    capability_tier: BirthDataTierDto;
    normalized_birth: PersonalDayNormalizedBirthDto;
    axes: PersonalDayAxesDto;
    decision: PersonalDayDecisionDto;
    factors: PersonalDayFactorDto[];
    contributions: PersonalDayContributionDto[];
    unavailable_sections: UnavailableSectionDto[];
    evidence: PersonalDayEvidenceDto;
    explanation_graph?: AssessmentTraceGraphDto | null;
    explanation?: AssessmentExplanationDto | null;
}

// ---------------------------------------------------------------------------
// Explanation projection DTO (amlich-bz0f.6).
// The single source of truth for "which factors influenced the result,
// which facts were deduplicated, which vetoes won, which evidence was
// unavailable, and why confidence is at its level".
// ---------------------------------------------------------------------------

export type PrecedenceRuleDto =
    | "veto_overrides_aggregation"
    | (string & {});

export type DeduplicationFamilyDto =
    | "bazi_target_day_pillar_relation"
    | "non_bazi_annual_pressure"
    | "direction_constraint_fact"
    | "hour_pillar_relation"
    | (string & {});

export type ConfidenceDimensionDto =
    | "date"
    | "time"
    | "gender"
    | "location"
    | "direction_overlay"
    | (string & {});

export interface ExplainedFactorDto {
    contribution_id: string;
    axis: string;
    polarity: string;
    strength: number;
    source_family: string;
    source_id: string;
    method: string;
    note?: string | null;
}

export interface ExplainedVetoDto {
    veto_id: string;
    axis: string;
    reason: string;
    source_family: string;
    source_id: string;
    method: string;
}

export interface DeduplicatedFactDto {
    family: DeduplicationFamilyDto;
    rule: string;
    observed_count: number;
    note?: string | null;
}

export interface UnavailableEvidenceDto {
    section: string;
    axis?: string | null;
    reason: string;
    required_fields: string[];
}

export interface ConfidenceReasonDto {
    dimension: ConfidenceDimensionDto;
    present: boolean;
    impact: string;
}

export interface ExplainedConfidenceDto {
    level: string;
    reasons: ConfidenceReasonDto[];
    present_count: number;
    total_count: number;
}

export interface AssessmentExplanationDto {
    projection_id: string;
    projection_version: string;
    policy_id: string;
    policy_version: string;
    intent_kind: string;
    precedence_rule: PrecedenceRuleDto;
    favorable_factors: ExplainedFactorDto[];
    adverse_factors: ExplainedFactorDto[];
    vetoes_applied: ExplainedVetoDto[];
    deduplicated_facts: DeduplicatedFactDto[];
    unavailable_evidence: UnavailableEvidenceDto[];
    confidence: ExplainedConfidenceDto;
}

export interface DirectionConstraintFactSummaryDto {
    direction: string;
    facts: ExplainedFactorDto[];
    rule: string;
}

export interface DirectionExplanationDto {
    projection_id: string;
    projection_version: string;
    policy_id: string;
    policy_version: string;
    intent_kind: string;
    precedence_rule: PrecedenceRuleDto;
    unavailable_evidence: UnavailableEvidenceDto[];
    confidence: ExplainedConfidenceDto;
    constraint_facts: DirectionConstraintFactSummaryDto[];
    deduplicated_facts?: DeduplicatedFactDto[];
}

export interface HourEntryExplanationDto {
    chi_index: number;
    chi_name: string;
    time_range: string;
    is_auspicious: boolean;
    rank_score: number;
    factors: ExplainedFactorDto[];
    unavailable_evidence: UnavailableEvidenceDto[];
    policy_version: string;
}

export interface HourExplanationDto {
    projection_id: string;
    projection_version: string;
    policy_id: string;
    policy_version: string;
    precedence_rule: PrecedenceRuleDto;
    hours: HourEntryExplanationDto[];
    deduplicated_facts: DeduplicatedFactDto[];
    confidence: ExplainedConfidenceDto;
}

export interface PersonalDayAdvisoryDto {
    summary: string;
    severity: string;
    top_signals: string[];
    why_this_matters: string[];
    recommended_actions: string[];
    priority_order: string[];
    highlights: string[];
    cautions: string[];
    /**
     * Missing-profile / missing-context messages that the Rust side separates
     * from `cautions` (amlich-mwbp.5) so `severity` reflects only genuine
     * adverse day signals. Omitted on the wire when empty
     * (`#[serde(skip_serializing_if = "Vec::is_empty")]`).
     */
    unavailable_context?: string[];
    reasoning_bucket?: string | null;
    reasoning_confidence?: string | null;
    canonical_assessment?: PersonalDayAssessmentDto | null;
}

export interface PersonalDayReportDto {
    summary: string;
    severity: string;
    top_signals: string[];
    chart: PersonalDayChartDto;
    decision?: InitiationOpeningDecisionDto | null;
    decision_export?: InitiationOpeningDecisionExportDto | null;
    graph?: ReasoningGraphExportDto | null;
    analysis: PersonalDayAnalysisDto;
    computed_metrics: PersonalDayMetricsDto;
    advisory: PersonalDayAdvisoryDto;
    canonical_assessment?: PersonalDayAssessmentDto | null;
}

export type PillarKindDto = 'year' | 'month' | 'day' | 'hour';
export type ElementInteractionDto =
    | 'same'
    | 'day_generates_pillar'
    | 'pillar_generates_day'
    | 'day_controls_pillar'
    | 'pillar_controls_day';
export type DirectionSignalDto =
    | 'kua_favorable'
    | 'kua_unfavorable'
    | 'tai_than'
    | 'hy_than'
    | 'phuc_than'
    | 'sat_phuong';

export interface BranchRelationDto {
    luc_xung: boolean;
    luc_hop: boolean;
    tam_hop: boolean;
    tuong_hai: boolean;
    tuong_hinh: boolean;
}

export interface PillarInteractionDto {
    pillar: PillarKindDto;
    pillar_canchi: string;
    thap_than: ThapThanResultDto;
    branch_relation: BranchRelationDto;
    element_interaction: ElementInteractionDto;
}

export interface DayPersonMatrixDto {
    day_canchi: string;
    day_master: string;
    day_to_day_master: ThapThanResultDto;
    pillars: PillarInteractionDto[];
    evidence: RuleEvidenceDto;
}

export interface ElementResonanceEntryDto {
    element: string;
    personal_score: number;
    relation_to_day: number;
    season_factor: number;
    effective_resonance: number;
    is_deficit: boolean;
    day_helps_deficit: boolean;
}

export interface ElementResonanceMatrixDto {
    day_canchi: string;
    day_element: string;
    month_chi: string;
    season_factor: number;
    entries: ElementResonanceEntryDto[];
    net_resonance: number;
    evidence: RuleEvidenceDto;
}

export interface PersonalHourEntryDto {
    chi_index: number;
    chi: string;
    canchi: string;
    time_range: string;
    is_hoang_dao: boolean;
    star_name: string;
    thap_than_to_day_master: ThapThanResultDto;
    branch_relation_to_birth_hour: BranchRelationDto;
    element_interaction: ElementInteractionDto;
    supports_weak_element: boolean;
    score: number;
}

export interface PersonalHourMatrixDto {
    day_canchi: string;
    day_master: string;
    birth_hour_chi: string;
    weak_element: string;
    hours: PersonalHourEntryDto[];
    evidence: RuleEvidenceDto;
}

export interface DirectionEntryDto {
    direction: string;
    signals: DirectionSignalDto[];
    favorable_count: number;
    unfavorable_count: number;
    net_score: number;
}

export interface DirectionMergeMatrixDto {
    day_canchi: string;
    kua_number: number;
    entries: DirectionEntryDto[];
    evidence: RuleEvidenceDto;
}

export interface DirectionAssessmentAxisOutcomeDto {
    axis: string;
    score?: number | null;
    unavailable_reason?: string | null;
}

export interface DirectionAssessmentWarningDto {
    code: string;
    message_vi: string;
}

export interface DirectionAssessmentEntryDto {
    direction: string;
    rank_score: number;
    axes: {
        travel_deities: DirectionAssessmentAxisOutcomeDto;
        kua_compatibility: DirectionAssessmentAxisOutcomeDto;
        directional_constraints: DirectionAssessmentAxisOutcomeDto;
        flying_star_overlay: DirectionAssessmentAxisOutcomeDto;
    };
    warnings?: DirectionAssessmentWarningDto[];
}

export interface DirectionAssessmentDto {
    policy_id: string;
    policy_version: string;
    intent: string;
    confidence: string;
    entries: DirectionAssessmentEntryDto[];
    unavailable_sections?: DirectionAssessmentWarningDto[];
}

export interface DomainDayBoostEntryDto {
    domain: string;
    base_score: number;
    day_modifier: number;
    han_penalty: number;
    boosted_score: number;
}

export interface DomainDayBoostMatrixDto {
    day_canchi: string;
    entries: DomainDayBoostEntryDto[];
    evidence: RuleEvidenceDto;
}

export interface PersonalDayMatrixQueryDto {
    birth: BaziQuery;
    date: DateQuery;
}

export interface PersonalDayMatrixReportDto {
    input: PersonalDayMatrixQueryDto;
    tier: BirthDataTierDto;
    day_person: DayPersonMatrixDto;
    element_resonance: ElementResonanceMatrixDto;
    personal_hours?: PersonalHourMatrixDto | null;
    direction_merge?: DirectionMergeMatrixDto | null;
    direction_assessment?: DirectionAssessmentDto | null;
    domain_day_boost?: DomainDayBoostMatrixDto | null;
    unavailable_sections: UnavailableSectionDto[];
    canonical_assessment?: PersonalDayAssessmentDto | null;
}

export interface HourSelectionQueryDto {
    date: DateQuery;
}

export interface HourSelectionChartDto {
    input: HourSelectionQueryDto;
    solar: SolarDateDto;
    lunar: LunarDateDto;
    gio_hoang_dao: GioHoangDaoDto;
}

export interface RankedHourCandidateDto {
    chi_name: string;
    time_range: string;
    is_auspicious: boolean;
    score: number;
    note_vi: string;
}

export interface HourSelectionEvidenceDto {
    source_family: string;
    source_id: string;
    method: string;
    note?: string | null;
}

export interface HourSelectionReasoningExportDto {
    intent: string;
    birth_data_tier: BirthDataTierDto;
    summary_vi: string;
    summary_en: string;
    top_recommendation?: RankedHourCandidateDto | null;
    ranked_hours: RankedHourCandidateDto[];
    auspicious_count: number;
    total_hours: number;
    evidence: HourSelectionEvidenceDto[];
    /**
     * Versioned hour-ranking policy that produced this reasoning
     * (`amlich-bz0f.4`). `v1` keeps the legacy birth-year-chi
     * semantics; `v2.4` layers three typed, source-attributed
     * full-profile observations on top so a full birth profile
     * (date + time) produces a richer `PersonalHourAlignment` axis.
     * Absent for reasoning produced by pre-v1.9 wrappers.
     */
    policy_version?: string | null;
}

export interface HourSelectionAnalysisDto {
    intent: string;
    summary_vi: string;
    summary_en: string;
    good_hours: HourInfoDto[];
    bad_hours: HourInfoDto[];
    top_recommendation?: HourInfoDto | null;
    canonical?: HourSelectionReasoningExportDto | null;
    canonical_assessment?: PersonalDayAssessmentDto | null;
}

export interface HourSelectionMetricsDto {
    good_hour_count: number;
    bad_hour_count: number;
    good_hour_ratio: number;
}

export interface HourSelectionAdvisoryDto {
    intent: string;
    summary_vi: string;
    summary_en: string;
    best_windows: string[];
    caution_windows: string[];
    canonical?: HourSelectionReasoningExportDto | null;
    canonical_assessment?: PersonalDayAssessmentDto | null;
}

export interface HourSelectionReportDto {
    chart: HourSelectionChartDto;
    analysis: HourSelectionAnalysisDto;
    computed_metrics: HourSelectionMetricsDto;
    advisory: HourSelectionAdvisoryDto;
}

export interface ThaiNguyenDto {
    can_chi: BaziCanChiDto;
    evidence: RuleEvidenceDto;
}

export interface MenhCungDto {
    menh_cung: BaziCanChiDto;
    than_cung: BaziCanChiDto;
    evidence: RuleEvidenceDto;
}

export interface KhongVongPairDto {
    branch_indices: [number, number];
    branch_names: [string, string];
}

export interface KhongVongPillarEntryDto {
    pillar: string;
    void_pair: KhongVongPairDto;
    hits: string[];
}

export interface KhongVongAnalysisDto {
    entries: KhongVongPillarEntryDto[];
    evidence: RuleEvidenceDto;
}

export interface ThanSatEntryDto {
    name: string;
    source: string;
    target_branch: number;
    target_branch_name: string;
    present_in: string[];
}

export interface ThanSatResultDto {
    stars: ThanSatEntryDto[];
    evidence: RuleEvidenceDto;
}

export interface BaziDerivedReportDto {
    input: BaziQuery;
    tier: BirthDataTierDto;
    thai_nguyen: ThaiNguyenDto;
    menh_cung?: MenhCungDto | null;
    khong_vong: KhongVongAnalysisDto;
    than_sat: ThanSatResultDto;
    unavailable_sections: UnavailableSectionDto[];
}

export interface ApiMetaDto {
    schema_version: string;
    ruleset_id: string;
    ruleset_version: string;
    profile: string;
    generated_at: string;
}

export interface DayBundleDto extends ApiMetaDto {
    solar: SolarDateDto;
    lunar: LunarDateDto;
    jd: number;
    canchi?: CanChiInfoDto | null;
    tiet_khi?: TietKhiDto | null;
    gio_hoang_dao?: GioHoangDaoDto | null;
    day_fortune?: DayFortuneDto | null;
    daily_recommendations?: DailyRecommendationsDto | null;
    contextual_recommendations?: DailyRecommendationsDto | null;
    insight?: DayInsightDto | null;
    upcoming_events: UpcomingEventDto[];
}

// Canonical v1.7 snapshot surfaces exposed by the desktop command. These
// mirror amlich-core's IChingCastSummary and DirectionCrossLinkSummary without
// recomputing or renaming evidence.
export type TienThienTrigramDto =
    | 'kien'
    | 'doai'
    | 'ly'
    | 'chan'
    | 'ton'
    | 'kham'
    | 'can'
    | 'khon';
export type FiveElementDto = 'moc' | 'hoa' | 'tho' | 'kim' | 'thuy';
export type TheDungRelationDto =
    | 'dung_sinh_the'
    | 'the_khac_dung'
    | 'dong'
    | 'the_sinh_dung'
    | 'dung_khac_the';
export type CatHungDto = 'cat' | 'binh' | 'hung';

export interface MaiHoaCastDto {
    lunar_year_branch: number;
    lunar_month: number;
    lunar_day: number;
    chi_hour_index: number;
    upper_trigram: TienThienTrigramDto;
    lower_trigram: TienThienTrigramDto;
    dong_hao: number;
    chu_que: number;
}

export interface BienQueDto {
    upper_trigram: TienThienTrigramDto;
    lower_trigram: TienThienTrigramDto;
    king_wen: number;
    flipped_dong_hao: number;
}

export interface TheDungClassificationDto {
    the_trigram: TienThienTrigramDto;
    dung_trigram: TienThienTrigramDto;
    dong_hao: number;
    the_element: FiveElementDto;
    dung_element: FiveElementDto;
    relation: TheDungRelationDto;
    verdict: CatHungDto;
}

export interface IChingCastSummaryDto {
    cast: MaiHoaCastDto;
    bien_que: BienQueDto;
    the_dung: TheDungClassificationDto;
    chu_hexagram_vi_name: string;
    chu_hexagram_thoai_tu: string;
    bien_hexagram_vi_name: string;
    bien_hexagram_thoai_tu: string;
    cat_hung_summary: CatHungDto;
    moving_line: number;
    question_vi?: string | null;
    evidence: ReasoningEvidenceEnvelopeDto[];
}

export type CompassDirectionDto =
    | 'north'
    | 'northeast'
    | 'east'
    | 'southeast'
    | 'south'
    | 'southwest'
    | 'west'
    | 'northwest';
export type DirectionAgreementDto =
    | 'agreement'
    | 'both_silent'
    | 'khcbppt_only'
    | 'huyen_khong_only'
    | 'conflict';

export interface DirectionalThaiTueDto {
    direction: CompassDirectionDto;
    conflict_kinds: string[];
}

export interface DirectionalTabooDto {
    thai_tue?: DirectionalThaiTueDto | null;
    tam_sat_branches: string[];
    sat_phuong_direction?: string | null;
    severity: ReasoningNodeSeverityDto;
    summary_vi: string;
}

export interface HuyenKhongCellDto {
    direction: CompassDirectionDto;
    palace_number: number;
    annual_star: number;
    monthly_star: number;
    safety_hint_vi?: string | null;
    summary_vi: string;
}

export interface DirectionCellDto {
    direction: CompassDirectionDto;
    khcbppt?: DirectionalTabooDto | null;
    huyen_khong?: HuyenKhongCellDto | null;
    agreement?: DirectionAgreementDto | null;
    severity: ReasoningNodeSeverityDto;
}

export interface DirectionCrossLinkSummaryDto {
    cross_link_kind: string;
    cross_link_source: string;
    date: string;
    day_chi_index: number;
    birth_chi_index: number;
    cells: DirectionCellDto[];
    summary_vi: string;
    composite_severity: ReasoningNodeSeverityDto;
    evidence: ReasoningEvidenceEnvelopeDto[];
}

export interface ClassicalSurfaceDto {
    iching_cast?: IChingCastSummaryDto | null;
    direction_cross_link: DirectionCrossLinkSummaryDto;
    // v1.10 `amlich-l2zc.3` (EXPLAIN-01) unified Traditional Wellness
    // Context. Additive: absent from JSON when the snapshot has not
    // been enriched with `enrich_day_snapshot_with_traditional_wellness`.
    traditional_wellness?: TraditionalWellnessContextDto | null;
}

// ---------------------------------------------------------------------------
// v1.10 Traditional Wellness Context DTO surface (amlich-l2zc.3,
// EXPLAIN-01). Mirrors `amlich_core::traditional_wellness::
// TraditionalWellnessContext` byte-for-byte so the desktop / TUI /
// API render the same bilingual explanation, disclaimer, review
// state, time basis, and KnownDivergence details.
// ---------------------------------------------------------------------------

export interface TraditionalWellnessContextDto {
    hour_branch?: BranchChannelAssociationDto | null;
    seasonal_cultivation?: SeasonalCultivationContextDto | null;
    disclaimer: LocalizedDisclaimerDto;
    review_state: string;
    time_basis: string;
    evidence: ReasoningEvidenceEnvelopeDto[];
}

export interface BranchChannelAssociationDto {
    branch_index: number;
    branch_vi: string;
    branch_zh: string;
    time_range: string;
    channel_vi: string;
    channel_en: string;
    channel_zh: string;
    wording_vi: string;
    wording_en: string;
    sources: TraditionalWellnessSourceCitationDto[];
    reviewer: string;
    safety_class: string;
    known_divergence_ids: string[];
    time_basis: string;
}

export interface TraditionalWellnessSourceCitationDto {
    source_id: string;
    work_title: string;
    volume_or_chapter: string;
    passage_key: string;
    edition_or_facsimile_uri: string;
    transcription_uri: string;
    translation_kind: string;
}

export interface SeasonalCultivationContextDto {
    solar_term: SolarTermDto;
    season: string;
    profile: SeasonalCultivationProfileDto;
    disclaimer: LocalizedDisclaimerDto;
    review_state: string;
    composition_note_vi: string;
    composition_note_en: string;
    evidence: ReasoningEvidenceEnvelopeDto[];
}

export interface SeasonalCultivationProfileDto {
    season: string;
    season_vi: string;
    season_en: string;
    season_zh: string;
    passage_key: string;
    wording_vi: string;
    wording_en: string;
    sources: TraditionalWellnessSourceCitationDto[];
    reviewer: string;
    safety_class: string;
    known_divergence_ids: string[];
}

export interface SolarTermDto {
    index: number;
    name: string;
    description: string;
    longitude: number;
    current_longitude: number;
    season: string;
}

export interface LocalizedDisclaimerDto {
    // `DisclaimerId` is a serde-transparent newtype — it serializes as
    // the inner string. The Rust `cultural_information_v1` constant
    // is the only stable id today.
    id: string;
    vi: string;
    en: string;
}

export interface DayRangeDto extends ApiMetaDto {
    start: string;
    end: string;
    days: DayBundleDto[];
}

export interface TietKhiTransitionDto {
    date: string;
    term: TietKhiDto;
}

export interface TietKhiYearDto {
    year: number;
    transitions: TietKhiTransitionDto[];
}

export type ApiInclude =
    | 'base'
    | 'can_chi'
    | 'tiet_khi'
    | 'hours'
    | 'fortune'
    | 'recommendations'
    | 'insight'
    | 'evidence';

// ---------------------------------------------------------------------------
// Debug semantic graph inspection (amlich-4gef).
// Source of truth: crates/amlich-api/src/dto.rs (DebugSemanticGraph* DTOs).
// Surface: amlich_api::get_debug_semantic_graph_inspection, wrapped by the
// `get_debug_semantic_graph_inspection` Tauri command. Richer than the
// reasoning graph on PersonalDayReportDto — clusters nodes, adds shape_hint,
// and includes recommendation evidence.
//
// Note: Rust DTOs do NOT use `skip_serializing_if = "Option::is_none"` for
// DebugVisualizationNodeDto, so `severity` and `shape_hint` are emitted as
// `null` (not omitted) when absent — mirrored here as `string | null`.
// ---------------------------------------------------------------------------

export interface DebugSemanticGraphQueryDto {
    day: number;
    month: number;
    year: number;
    include_recommendations?: boolean;
}

export interface DebugInspectionDateDto {
    year: number;
    month: number;
    day: number;
}

export interface DebugInspectionSummaryDto {
    total_nodes: number;
    total_edges: number;
    clusters: string[];
    semantic_kinds: string[];
    has_recommendation_evidence: boolean;
}

export interface DebugVisualizationNodeDto {
    node_id: string;
    label: string;
    cluster: string;
    semantic_kind: string;
    severity: string | null;
    shape_hint: string | null;
}

export interface DebugVisualizationEdgeDto {
    edge_id: string;
    from_id: string;
    to_id: string;
    label: string;
    semantic_kind: string;
    weight: number;
}

export interface DebugVisualizationDto {
    nodes: DebugVisualizationNodeDto[];
    edges: DebugVisualizationEdgeDto[];
}

export interface DebugSemanticGraphResponseDto {
    surface: string;
    date: DebugInspectionDateDto;
    visualization: DebugVisualizationDto;
    summary: DebugInspectionSummaryDto;
    cluster_counts: Record<string, number>;
    semantic_kind_counts: Record<string, number>;
    severity_counts: Record<string, number>;
}
