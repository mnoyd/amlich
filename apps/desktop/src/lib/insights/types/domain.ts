/**
 * Shared domain types used by desktop UI and insight DTO bindings.
 */

export type Lang = "vi" | "en";

export type HolidayInfo = {
  name: string;
  description: string;
  is_solar: boolean;
  lunar_day: number | null;
  lunar_month: number | null;
  category:
    | "festival"
    | "public-holiday"
    | "commemorative"
    | "professional"
    | "social"
    | "international"
    | "lunar-cycle";
  is_major: boolean;
};

export type GoodHour = {
  hour_chi: string;
  time_range: string;
  star: string;
};

export type DayCell = {
  day: number;
  month: number;
  year: number;
  day_of_week_index: number;
  day_of_week: string;
  solar_date: string;
  lunar_day: number;
  lunar_month: number;
  lunar_year: number;
  lunar_leap: boolean;
  lunar_date: string;
  canchi_day: string;
  canchi_month: string;
  canchi_year: string;
  tiet_khi: string;
  tiet_khi_description: string;
  tiet_khi_season: string;
  good_hours: GoodHour[];
  holidays: HolidayInfo[];
};

export type MonthData = {
  month: number;
  year: number;
  first_weekday: number;
  days: DayCell[];
};

// Bilingual text helper
export type BilingualText = {
  vi: string;
  en: string;
};

export type BilingualList = {
  vi: string[];
  en: string[];
};

// ============================================
// Input Day Type (from calendar)
// ============================================

export type DayForInsight = Pick<
  DayCell,
  | "day"
  | "month"
  | "year"
  | "lunar_day"
  | "lunar_month"
  | "lunar_year"
  | "lunar_leap"
  | "canchi_day"
  | "canchi_month"
  | "canchi_year"
  | "tiet_khi"
  | "tiet_khi_description"
  | "tiet_khi_season"
  | "holidays"
>;
