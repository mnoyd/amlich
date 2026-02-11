/**
 * View types for the Vietnamese calendar insight system.
 * These types represent UI-specific structures used for
 * rendering insight cards, sections, and categories.
 */

import type { EventCategoryType } from "./domain";

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
// Event Category (view model with CSS metadata)
// ============================================

export type EventCategory = {
  type: EventCategoryType;
  color: string;       // CSS variable name, e.g. "var(--cat-festival)"
  colorHex: string;    // Raw hex for inline styles
  tint: string;        // CSS variable name for tint
  label: { vi: string; en: string };
  priority: number;    // Lower = higher priority (1 = highest)
  name?: string;       // Event name for pill display
};
