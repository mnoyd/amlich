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

export interface CanChiItemDto {
    can: string;
    chi: string;
    full: string;
}

export interface CanChiInfoDto {
    day: CanChiItemDto;
    month: CanChiItemDto;
    year: CanChiItemDto;
    hour: CanChiItemDto;
}

export interface NguHanhDto {
    element: string;
    color: string;
    direction: string;
}

export interface DayInfoDto {
    solar: SolarDateDto;
    lunar: LunarDateDto;
    canchi: CanChiInfoDto;
    tiet_khi: any;
    gio_hoang_dao: any;
}

export interface TietKhiDto {
    name: string;
    season: string;
    description: string;
    jd: number;
    date: string;
}

export interface GioHoangDaoDto {
    good_hours: any[];
}

export type RecommendationBucketDto = 'Nen' | 'CoThe' | 'Tranh' | 'KyManh';
export type RecommendationSeverityDto = 'Primary' | 'Secondary' | 'Tertiary';

export interface ActivityLabelDto {
    vi: string;
    en: string;
}

export interface RecommendationEvidenceSourceDto {
    source_family: string;
    source_id: string;
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

export interface DailyRecommendationsDto {
    ruleset_id: string;
    ruleset_version: string;
    profile: string;
    version: string;
    summary_vi: string;
    summary_en: string;
    activities: SynthesizedRecommendationDto[];
}

export interface DayBundleDto {
    schema_version: string;
    ruleset_id: string;
    ruleset_version: string;
    profile: string;
    generated_at: string;
    solar: SolarDateDto;
    lunar: LunarDateDto;
    jd: number;
    canchi?: CanChiInfoDto;
    tiet_khi?: TietKhiDto;
    gio_hoang_dao?: GioHoangDaoDto;
    day_fortune?: DayFortuneDto;
    daily_recommendations?: DailyRecommendationsDto;
    contextual_recommendations?: DailyRecommendationsDto;
    insight?: DayInsightDto;
    upcoming_events: UpcomingEventDto[];
}

export interface DayRangeDto {
    schema_version: string;
    ruleset_id: string;
    ruleset_version: string;
    profile: string;
    generated_at: string;
    start: string;
    end: string;
    days: DayBundleDto[];
}

export interface UpcomingEventDto {
    name: string;
    days_left: number;
    is_lunar: boolean;
}

export interface DayInsightDto {
    [key: string]: any;
}

export interface DayFortuneDto {
    [key: string]: any;
}

export interface BaziReportDto {
    [key: string]: any;
}

export interface BaziDerivedReportDto {
    [key: string]: any;
}

export interface HourSelectionReportDto {
    [key: string]: any;
}

export interface RulesetCatalogEntryDto {
    [key: string]: any;
}

export interface RecommendationPackCatalogEntryDto {
    [key: string]: any;
}

export interface HolidayDto {
    name: string;
    description: string;
    is_solar: boolean;
    lunar_day?: number;
    lunar_month?: number;
    solar_day: number;
    solar_month: number;
    solar_year: number;
    category: string;
    is_major: boolean;
}

export interface PersonalDayReportDto {
    [key: string]: any;
}

export interface PersonalDayMatrixReportDto {
    [key: string]: any;
}

export interface TietKhiYearDto {
    year: number;
    transitions: TietKhiTransitionDto[];
}

export interface TietKhiTransitionDto {
    date: string;
    term: TietKhiDto;
}

export type ApiInclude =
    | 'base'
    | 'can_chi'
    | 'tiet_khi'
    | 'hours'
    | 'fortune'
    | 'insight'
    | 'evidence';

export interface DateQuery {
    day: number;
    month: number;
    year: number;
    timezone?: number | null;
    ruleset_id?: string | null;
    event_kind?: string | null;
    enabled_pack_ids?: string[];
}
