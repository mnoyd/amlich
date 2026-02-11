/**
 * Event category presentation layer.
 * Maps domain category types to visual metadata (colors, labels, priority).
 */

import type {
  Lang,
  DayForInsight,
  EventCategoryType,
} from "../types/domain";

import type { EventCategory } from "../types/view";

import { classifyHoliday } from "../domain/selectors";

export const CATEGORY_META: Record<
  EventCategoryType,
  {
    color: string;
    colorHex: string;
    tint: string;
    label: { vi: string; en: string };
    priority: number;
  }
> = {
  festival: {
    color: "var(--cat-festival)",
    colorHex: "#C62828",
    tint: "var(--cat-festival-tint)",
    label: { vi: "Lễ truyền thống", en: "Traditional Festival" },
    priority: 1,
  },
  "public-holiday": {
    color: "var(--cat-public-holiday)",
    colorHex: "#AD1457",
    tint: "var(--cat-public-holiday-tint)",
    label: { vi: "Nghỉ lễ", en: "Public Holiday" },
    priority: 2,
  },
  commemorative: {
    color: "var(--cat-commemorative)",
    colorHex: "#1565C0",
    tint: "var(--cat-commemorative-tint)",
    label: { vi: "Kỷ niệm", en: "Commemorative" },
    priority: 3,
  },
  professional: {
    color: "var(--cat-professional)",
    colorHex: "#2E7D32",
    tint: "var(--cat-professional-tint)",
    label: { vi: "Ngành nghề", en: "Professional Day" },
    priority: 4,
  },
  social: {
    color: "var(--cat-social)",
    colorHex: "#E65100",
    tint: "var(--cat-social-tint)",
    label: { vi: "Xã hội", en: "Social Day" },
    priority: 5,
  },
  international: {
    color: "var(--cat-international)",
    colorHex: "#6A1B9A",
    tint: "var(--cat-international-tint)",
    label: { vi: "Quốc tế", en: "International" },
    priority: 6,
  },
  "solar-term": {
    color: "var(--cat-solar-term)",
    colorHex: "#00796B",
    tint: "var(--cat-solar-term-tint)",
    label: { vi: "Tiết khí", en: "Solar Term" },
    priority: 7,
  },
  "lunar-cycle": {
    color: "var(--cat-lunar-cycle)",
    colorHex: "#D4AF37",
    tint: "var(--cat-lunar-cycle-tint)",
    label: { vi: "Sóc/Vọng", en: "Lunar 1st/15th" },
    priority: 8,
  },
};

/**
 * Get all event categories present on a given day.
 * Returns categories sorted by priority (highest first).
 */
export function getDayEventCategories(day: DayForInsight, lang: Lang = "vi"): EventCategory[] {
  const categories: EventCategory[] = [];

  // Classify each holiday on this day
  for (const holiday of day.holidays) {
    const catType = classifyHoliday(holiday, day);
    const meta = CATEGORY_META[catType];

    categories.push({
      type: catType,
      color: meta.color,
      colorHex: meta.colorHex,
      tint: meta.tint,
      label: meta.label,
      priority: meta.priority,
      name: holiday.name,
    });
  }

  // Sort by priority (lower number = higher priority)
  categories.sort((a, b) => a.priority - b.priority);

  return categories;
}

/**
 * Get the category metadata for a given category type.
 */
export function getCategoryMeta(type: EventCategoryType) {
  return CATEGORY_META[type];
}

/**
 * Get unique category dots for a day (deduplicated by type, max 3).
 * Returns the distinct category types present, sorted by priority.
 */
export function getDayDots(day: DayForInsight): { type: EventCategoryType; colorHex: string }[] {
  const categories = getDayEventCategories(day);
  const seen = new Set<EventCategoryType>();
  const dots: { type: EventCategoryType; colorHex: string }[] = [];

  for (const cat of categories) {
    if (!seen.has(cat.type) && dots.length < 3) {
      seen.add(cat.type);
      dots.push({ type: cat.type, colorHex: cat.colorHex });
    }
  }

  return dots;
}
