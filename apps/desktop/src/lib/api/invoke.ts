import { invoke } from '@tauri-apps/api/core';
import type {
    ApiInclude, DateQuery, DayInfoDto, DayInsightDto, DayBundleDto, DayRangeDto,
    BaziReportDto, BaziDerivedReportDto, HourSelectionReportDto,
    RulesetCatalogEntryDto, RecommendationPackCatalogEntryDto,
    HolidayDto, PersonalDayReportDto, PersonalDayMatrixReportDto,
    TietKhiYearDto
} from './types';

export type DayRangeRequest = {
    start: DateQuery;
    end: DateQuery;
    includes?: ApiInclude[];
};

export async function fetchDayBundle(day: number, month: number, year: number): Promise<DayBundleDto> {
    return invoke('get_day_bundle', { day, month, year });
}

export async function fetchDayRange(request: DayRangeRequest): Promise<DayRangeDto> {
    return invoke('get_day_range', request);
}

export async function fetchDayInfo(day: number, month: number, year: number): Promise<DayInfoDto> {
    return invoke('get_day_info', { day, month, year });
}

export async function fetchDayInsight(day: number, month: number, year: number): Promise<DayInsightDto> {
    return invoke('get_day_insight', { day, month, year });
}

export async function fetchBaziReport(year: number, month: number, day: number, hour: number, minute: number, gender?: string): Promise<BaziReportDto> {
    return invoke('get_bazi_report', { year, month, day, hour, minute, gender });
}

export async function fetchBaziDerivedReport(year: number, month: number, day: number, hour: number, minute: number, gender?: string): Promise<BaziDerivedReportDto> {
    return invoke('get_bazi_derived_report', { year, month, day, hour, minute, gender });
}

export async function fetchHourSelectionReport(day: number, month: number, year: number): Promise<HourSelectionReportDto> {
    return invoke('get_hour_selection_report', { day, month, year });
}

export async function fetchTietKhiForYear(year: number): Promise<TietKhiYearDto> {
    return invoke('get_tiet_khi_for_year', { year });
}

export async function fetchRulesetCatalog(): Promise<RulesetCatalogEntryDto[]> {
    return invoke('get_ruleset_catalog');
}

export async function fetchRecommendationPackCatalog(): Promise<RecommendationPackCatalogEntryDto[]> {
    return invoke('get_recommendation_pack_catalog');
}

export async function fetchHolidaysList(year: number, majorOnly: boolean = false): Promise<HolidayDto[]> {
    return invoke('get_holidays_list', { year, majorOnly });
}

export async function fetchPersonalDayReport(day: number, month: number, year: number, birthYear?: number, birthMonth?: number, birthDay?: number, gender?: string): Promise<PersonalDayReportDto> {
    return invoke('get_personal_day_report', { day, month, year, birthYear, birthMonth, birthDay, gender });
}

export async function fetchPersonalDayMatrixReport(day: number, month: number, year: number, birthYear: number, birthMonth: number, birthDay: number, birthHour: number, birthMinute: number, gender?: string): Promise<PersonalDayMatrixReportDto> {
    return invoke('get_personal_day_matrix_report', { day, month, year, birthYear, birthMonth, birthDay, birthHour, birthMinute, gender });
}
