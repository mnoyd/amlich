/**
 * Domain selectors for calendar data.
 * Pure functions that perform data lookups and classification.
 * No UI concepts — no cards, CSS variables, or display formatting.
 */

import type {
  Lang,
  DayForInsight,
  FestivalData,
  NationalHolidayData,
  TietKhiData,
  CanInfo,
  ChiInfo,
  HolidayInfo,
  EventCategoryType,
} from "../types/domain";

import {
  lunarFestivalIndex,
  solarFestivalIndex,
  nationalHolidayIndex,
  tietKhiIndex,
  canIndex,
  chiIndex,
} from "../shared-data";

function dateKey(month: number, day: number): string {
  return `${month}-${day}`;
}

/**
 * Find a festival that matches the given day
 */
export function findFestival(day: DayForInsight): FestivalData | null {
  return solarFestivalIndex.get(dateKey(day.month, day.day))
    ?? lunarFestivalIndex.get(dateKey(day.lunar_month, day.lunar_day))
    ?? null;
}

/**
 * Find a national holiday that matches the given day (always solar-based)
 */
export function findNationalHoliday(day: DayForInsight): NationalHolidayData | null {
  return nationalHolidayIndex.get(dateKey(day.month, day.day)) ?? null;
}

/**
 * Find the current tiết khí data
 */
export function findTietKhi(termName: string): TietKhiData | null {
  return tietKhiIndex.get(termName.trim()) ?? null;
}

/**
 * Extract Can and Chi from a canchi string like "Giáp Tý"
 */
export function parseCanChi(canchiStr: string): { can: string; chi: string } {
  const parts = canchiStr.trim().split(/\s+/);
  return {
    can: parts[0] || "",
    chi: parts[1] || "",
  };
}

/**
 * Find Can info by name
 */
export function findCan(name: string): CanInfo | null {
  return canIndex.get(name) ?? null;
}

/**
 * Find Chi info by name
 */
export function findChi(name: string): ChiInfo | null {
  return chiIndex.get(name) ?? null;
}

/**
 * Check if a day has festival or national holiday data available
 */
export function hasFestivalData(day: DayForInsight): boolean {
  return findFestival(day) !== null || findNationalHoliday(day) !== null;
}

/**
 * Get festival or national holiday names for a day (if any)
 */
export function getFestivalNames(day: DayForInsight, lang: Lang = "vi"): string[] {
  const festival = findFestival(day);
  if (festival) return festival.names[lang];

  const nationalHoliday = findNationalHoliday(day);
  if (nationalHoliday) return nationalHoliday.names[lang];

  return [];
}

/**
 * Classify a single holiday by matching against known data sources.
 * Returns the EventCategoryType string.
 */
export function classifyHoliday(holiday: HolidayInfo, day: DayForInsight): EventCategoryType {
  // Primary path: category is provided by backend data
  if (holiday.category) {
    return holiday.category;
  }

  // Fallback path for legacy payloads without category
  const festival = findFestival(day);
  if (festival && !holiday.is_solar && holiday.lunar_day === festival.lunarDay && holiday.lunar_month === festival.lunarMonth) {
    return "festival";
  }

  const natHoliday = findNationalHoliday(day);
  if (natHoliday && holiday.is_solar) {
    return natHoliday.category;
  }

  if (holiday.name.startsWith("Mùng 1 tháng") || holiday.name.startsWith("Rằm tháng")) {
    return "lunar-cycle";
  }

  if (!holiday.is_solar && holiday.is_major) {
    return "festival";
  }

  if (holiday.is_solar) {
    if (holiday.is_major) return "public-holiday";
    return "commemorative";
  }

  return "lunar-cycle";
}
