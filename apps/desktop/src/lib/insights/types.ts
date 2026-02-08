/**
 * Type definitions for the Date Insight system
 * Provides rich cultural information about Vietnamese dates
 */

export type Lang = "vi" | "en";

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

export type NormalDayInsight = {
  mode: "normal";
  termName: string;
  termDescription: string;
  canchiDay: string;
  cards: InsightCard[];
};

export type DateInsight = FestivalInsight | NormalDayInsight;

// ============================================
// Input Day Type (from calendar)
// ============================================

export type DayForInsight = {
  day: number;
  month: number;
  year: number;
  lunar_day: number;
  lunar_month: number;
  lunar_year: number;
  lunar_leap: boolean;
  canchi_day: string;
  canchi_month: string;
  canchi_year: string;
  tiet_khi: string;
  tiet_khi_description: string;
  tiet_khi_season: string;
  holidays: {
    name: string;
    description: string;
    is_solar: boolean;
    lunar_day: number | null;
    lunar_month: number | null;
    is_major: boolean;
  }[];
};
