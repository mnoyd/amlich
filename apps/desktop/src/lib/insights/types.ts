/**
 * Type definitions for the Date Insight system
 * Provides rich cultural information about Vietnamese dates
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
// Festival Data Types
// ============================================

export type FoodItem = {
  name: BilingualText;
  description: BilingualText;
};

export type TabooItem = {
  action: BilingualText;
  reason: BilingualText;
};

export type ProverbItem = {
  text: string; // Original Vietnamese
  meaning: BilingualText;
};

export type RegionalCustoms = {
  north: BilingualText;
  central: BilingualText;
  south: BilingualText;
};

export type FestivalData = {
  id: string;
  lunarDay: number;
  lunarMonth: number;
  isSolar?: boolean; // For Thanh Minh which is solar-based
  solarDay?: number;
  solarMonth?: number;
  names: BilingualList;
  origin: BilingualText;
  activities: BilingualList;
  food: FoodItem[];
  taboos: TabooItem[];
  proverbs: ProverbItem[];
  regions: RegionalCustoms;
};

// ============================================
// National Holiday Data Types
// ============================================

export type NationalHolidayData = {
  id: string;
  isSolar: true; // Always true for national holidays
  solarDay: number;
  solarMonth: number;
  category: "public-holiday" | "commemorative" | "professional" | "social" | "international";
  names: BilingualList;
  origin: BilingualText;
  significance?: BilingualText;
  activities?: BilingualList;
  traditions?: BilingualList;
  food?: FoodItem[];
  symbols?: string[];
  taboos?: TabooItem[];
  proverbs?: ProverbItem[];
  regions?: RegionalCustoms;
};

// ============================================
// Tiết Khí (Solar Term) Data Types
// ============================================

export type TietKhiData = {
  id: string;
  name: BilingualText;
  longitude: number; // Sun's ecliptic longitude
  meaning: BilingualText; // Etymology and cultural meaning
  astronomy: BilingualText; // Astronomical significance
  agriculture: BilingualList; // Farming advice
  health: BilingualList; // Traditional medicine / wellness
  weather: BilingualText; // Weather patterns
};

// ============================================
// Can Chi Data Types
// ============================================

export type ElementInfo = {
  name: BilingualText;
  nature: BilingualText;
};

export type CanInfo = {
  name: string;
  element: string; // Ngũ Hành
  meaning: BilingualText;
  nature: BilingualText;
};

export type ChiInfo = {
  name: string;
  animal: BilingualText;
  element: string;
  meaning: BilingualText;
  hours: string; // Time range
};

export type DayGuidance = {
  goodFor: BilingualList;
  avoidFor: BilingualList;
};

export type CanChiData = {
  can: CanInfo[];
  chi: ChiInfo[];
  elements: Record<string, ElementInfo>;
  dayGuidance: Record<string, DayGuidance>; // Keyed by chi name for simplicity
};

// ============================================
// Output Content Types (for rendering)
// ============================================

export type GuidanceSection = {
  title: string;
  items: string[];
};

export type RegionSection = {
  title: string;
  content: string;
};

export type ProverbDisplay = {
  text: string;
  meaning: string;
};

export type InsightCardExtra = {
  weather?: string;
  canNature?: string;
  chiHours?: string;
  goodFor?: GuidanceSection;
  avoidFor?: GuidanceSection;
  agriculture?: GuidanceSection;
  health?: GuidanceSection;
  proverbs?: ProverbDisplay[];
  north?: RegionSection;
  central?: RegionSection;
  south?: RegionSection;
};

export type InsightCard = {
  id: string;
  title: string;
  subtitle?: string;
  content: string | string[];
  type: "text" | "list" | "proverb" | "region-tabs";
  extra?: InsightCardExtra;
};

export type FestivalInsight = {
  mode: "festival";
  festivalId: string;
  title: string;
  subtitle: string;
  cards: InsightCard[];
};

export type NationalHolidayInsight = {
  mode: "national-holiday";
  holidayId: string;
  category: "public-holiday" | "commemorative" | "professional" | "social" | "international";
  title: string;
  subtitle: string;
  cards: InsightCard[];
};

export type NormalDayInsight = {
  mode: "normal";
  termName: string;
  termDescription: string;
  canchiDay: string;
  cards: InsightCard[];
};

export type DateInsight = FestivalInsight | NationalHolidayInsight | NormalDayInsight;

export type DateInsightMulti = {
  mode: "multi";
  title: string;
  subtitle: string;
  sections: (FestivalInsight | NationalHolidayInsight | NormalDayInsight)[];
};

export type AnyDateInsight = DateInsight | DateInsightMulti;

// ============================================
// Event Category Types (for visual indicators)
// ============================================

export type EventCategoryType =
  | "festival"
  | "public-holiday"
  | "commemorative"
  | "professional"
  | "social"
  | "international"
  | "solar-term"
  | "lunar-cycle";

export type EventCategory = {
  type: EventCategoryType;
  color: string;       // CSS variable name, e.g. "var(--cat-festival)"
  colorHex: string;    // Raw hex for inline styles
  tint: string;        // CSS variable name for tint
  label: { vi: string; en: string };
  priority: number;    // Lower = higher priority (1 = highest)
  name?: string;       // Event name for pill display
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
