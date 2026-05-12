// Re-export or define DTOs here
// These mirror the amlich-api Rust DTOs

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

export interface CanChiDto {
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
    canchi: CanChiDto;
    tiet_khi: any;
    gio_hoang_dao: any;
}

export interface BaziReportDto {
    // Fill in as needed
    [key: string]: any;
}

export interface BaziDerivedReportDto {
    [key: string]: any;
}

export interface HourSelectionReportDto {
    [key: string]: any;
}

export interface DayBundleDto {
    info: DayInfoDto;
    insight?: any;
    fortune?: any;
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
    tiet_khi_list: any[];
}

export interface DayInsightDto {
    [key: string]: any;
}

export interface DayFortuneDto {
    [key: string]: any;
}
