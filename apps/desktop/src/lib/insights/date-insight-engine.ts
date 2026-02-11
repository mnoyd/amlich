/**
 * Date Insight Engine
 * Generates rich cultural content for Vietnamese dates
 */

import type {
  Lang,
  DayForInsight,
  DateInsight,
  FestivalInsight,
  NationalHolidayInsight,
  NormalDayInsight,
  InsightCard,
  FestivalData,
  NationalHolidayData,
  TietKhiData,
  CanInfo,
  ChiInfo,
  DayGuidance,
  EventCategory,
  EventCategoryType,
} from "./types";

import festivalsData from "./data/festivals.json";
import nationalHolidaysData from "./data/national-holidays.json";
import tietKhiData from "./data/tiet-khi.json";
import canchiData from "./data/canchi.json";

// Type assertions for imported JSON
const festivals = festivalsData.festivals as FestivalData[];
const nationalHolidays = nationalHolidaysData.holidays as NationalHolidayData[];
const tietKhiList = tietKhiData.tietKhi as TietKhiData[];
const canList = canchiData.can as CanInfo[];
const chiList = canchiData.chi as ChiInfo[];
const elements = canchiData.elements as Record<string, { name: { vi: string; en: string }; nature: { vi: string; en: string } }>;
const dayGuidanceMap = canchiData.dayGuidance as Record<string, DayGuidance>;

/**
 * Find a festival that matches the given day
 */
function findFestival(day: DayForInsight): FestivalData | null {
  return festivals.find((f) => {
    if (f.isSolar) {
      // Solar-based festival (like Thanh Minh)
      return f.solarDay === day.day && f.solarMonth === day.month;
    }
    // Lunar-based festival
    return f.lunarDay === day.lunar_day && f.lunarMonth === day.lunar_month;
  }) || null;
}

/**
 * Find a national holiday that matches the given day (always solar-based)
 */
function findNationalHoliday(day: DayForInsight): NationalHolidayData | null {
  return nationalHolidays.find((h) =>
    h.solarDay === day.day && h.solarMonth === day.month
  ) || null;
}

/**
 * Find the current tiết khí data
 */
function findTietKhi(termName: string): TietKhiData | null {
  // Normalize the term name for matching
  const normalized = termName.trim();
  return tietKhiList.find((t) => t.name.vi === normalized) || null;
}

/**
 * Extract Can and Chi from a canchi string like "Giáp Tý"
 */
function parseCanChi(canchiStr: string): { can: string; chi: string } {
  const parts = canchiStr.trim().split(/\s+/);
  return {
    can: parts[0] || "",
    chi: parts[1] || "",
  };
}

/**
 * Find Can info by name
 */
function findCan(name: string): CanInfo | null {
  return canList.find((c) => c.name === name) || null;
}

/**
 * Find Chi info by name
 */
function findChi(name: string): ChiInfo | null {
  return chiList.find((c) => c.name === name) || null;
}

/**
 * Build festival insight content
 */
function buildFestivalInsight(festival: FestivalData, day: DayForInsight, lang: Lang): FestivalInsight {
  const cards: InsightCard[] = [];

  // Origin card
  cards.push({
    id: "origin",
    title: lang === "vi" ? "Nguồn gốc" : "Historical Origin",
    content: festival.origin[lang],
    type: "text",
  });

  // Activities card
  cards.push({
    id: "activities",
    title: lang === "vi" ? "Hoạt động truyền thống" : "Traditional Activities",
    content: festival.activities[lang],
    type: "list",
  });

  // Food & Offerings card
  if (festival.food.length > 0) {
    const foodItems = festival.food.map((f) => 
      `${f.name[lang]}: ${f.description[lang]}`
    );
    cards.push({
      id: "food",
      title: lang === "vi" ? "Ẩm thực & Lễ vật" : "Food & Offerings",
      content: foodItems,
      type: "list",
    });
  }

  // Taboos card
  if (festival.taboos.length > 0) {
    const tabooItems = festival.taboos.map((t) =>
      `${t.action[lang]} — ${t.reason[lang]}`
    );
    cards.push({
      id: "taboos",
      title: lang === "vi" ? "Điều kiêng kỵ" : "Taboos to Avoid",
      content: tabooItems,
      type: "list",
    });
  }

  // Proverbs card
  if (festival.proverbs.length > 0) {
    cards.push({
      id: "proverbs",
      title: lang === "vi" ? "Ca dao - Tục ngữ" : "Proverbs & Sayings",
      content: festival.proverbs.map((p) => p.text),
      type: "proverb",
      extra: {
        proverbs: festival.proverbs.map((p) => ({
          text: p.text,
          meaning: p.meaning[lang],
        })),
      },
    });
  }

  // Regional customs card
  cards.push({
    id: "regions",
    title: lang === "vi" ? "Phong tục vùng miền" : "Regional Customs",
    content: "",
    type: "region-tabs",
    extra: {
      north: {
        title: lang === "vi" ? "Miền Bắc" : "North",
        content: festival.regions.north[lang],
      },
      central: {
        title: lang === "vi" ? "Miền Trung" : "Central",
        content: festival.regions.central[lang],
      },
      south: {
        title: lang === "vi" ? "Miền Nam" : "South",
        content: festival.regions.south[lang],
      },
    },
  });

  return {
    mode: "festival",
    festivalId: festival.id,
    title: festival.names[lang][0],
    subtitle: festival.names[lang].slice(1).join(" • ") || festival.names[lang === "vi" ? "en" : "vi"][0],
    cards,
  };
}

/**
 * Build national holiday insight content
 */
function buildNationalHolidayInsight(holiday: NationalHolidayData, day: DayForInsight, lang: Lang): NationalHolidayInsight {
  const cards: InsightCard[] = [];

  // Origin card
  cards.push({
    id: "origin",
    title: lang === "vi" ? "Lịch sử & Nguồn gốc" : "History & Origin",
    content: holiday.origin[lang],
    type: "text",
  });

  // Significance card
  if (holiday.significance) {
    cards.push({
      id: "significance",
      title: lang === "vi" ? "Ý nghĩa" : "Significance",
      content: holiday.significance[lang],
      type: "text",
    });
  }

  // Activities card
  if (holiday.activities && holiday.activities[lang].length > 0) {
    cards.push({
      id: "activities",
      title: lang === "vi" ? "Hoạt động" : "Activities",
      content: holiday.activities[lang],
      type: "list",
    });
  }

  // Traditions card
  if (holiday.traditions && holiday.traditions[lang].length > 0) {
    cards.push({
      id: "traditions",
      title: lang === "vi" ? "Phong tục" : "Traditions",
      content: holiday.traditions[lang],
      type: "list",
    });
  }

  // Food card
  if (holiday.food && holiday.food.length > 0) {
    const foodItems = holiday.food.map((f) =>
      `${f.name[lang]}: ${f.description[lang]}`
    );
    cards.push({
      id: "food",
      title: lang === "vi" ? "Ẩm thực" : "Food",
      content: foodItems,
      type: "list",
    });
  }

  // Taboos card
  if (holiday.taboos && holiday.taboos.length > 0) {
    const tabooItems = holiday.taboos.map((t) =>
      `${t.action[lang]} — ${t.reason[lang]}`
    );
    cards.push({
      id: "taboos",
      title: lang === "vi" ? "Điều kiêng kỵ" : "Taboos to Avoid",
      content: tabooItems,
      type: "list",
    });
  }

  // Proverbs card
  if (holiday.proverbs && holiday.proverbs.length > 0) {
    cards.push({
      id: "proverbs",
      title: lang === "vi" ? "Ca dao - Tục ngữ" : "Proverbs & Sayings",
      content: holiday.proverbs.map((p) => p.text),
      type: "proverb",
      extra: {
        proverbs: holiday.proverbs.map((p) => ({
          text: p.text,
          meaning: p.meaning[lang],
        })),
      },
    });
  }

  // Regional customs card
  if (holiday.regions) {
    cards.push({
      id: "regions",
      title: lang === "vi" ? "Phong tục vùng miền" : "Regional Customs",
      content: "",
      type: "region-tabs",
      extra: {
        north: {
          title: lang === "vi" ? "Miền Bắc" : "North",
          content: holiday.regions.north[lang],
        },
        central: {
          title: lang === "vi" ? "Miền Trung" : "Central",
          content: holiday.regions.central[lang],
        },
        south: {
          title: lang === "vi" ? "Miền Nam" : "South",
          content: holiday.regions.south[lang],
        },
      },
    });
  }

  return {
    mode: "national-holiday",
    holidayId: holiday.id,
    category: holiday.category,
    title: holiday.names[lang][0],
    subtitle: holiday.names[lang].slice(1).join(" • ") || holiday.names[lang === "vi" ? "en" : "vi"][0],
    cards,
  };
}

/**
 * Build normal day insight content (tiết khí + can chi focused)
 */
function buildNormalDayInsight(day: DayForInsight, lang: Lang): NormalDayInsight {
  const cards: InsightCard[] = [];
  
  // Find tiết khí data
  const tietKhi = findTietKhi(day.tiet_khi);
  
  // Parse can chi
  const { can: canName, chi: chiName } = parseCanChi(day.canchi_day);
  const canInfo = findCan(canName);
  const chiInfo = findChi(chiName);
  const chiGuidance = dayGuidanceMap[chiName];

  // Tiết Khí meaning card
  if (tietKhi) {
    cards.push({
      id: "term-meaning",
      title: lang === "vi" ? "Ý nghĩa tiết khí" : "Solar Term Meaning",
      subtitle: tietKhi.name[lang],
      content: tietKhi.meaning[lang],
      type: "text",
    });

    // Astronomy card
    cards.push({
      id: "astronomy",
      title: lang === "vi" ? "Thiên văn" : "Astronomy",
      content: tietKhi.astronomy[lang],
      type: "text",
      extra: {
        weather: tietKhi.weather[lang],
      },
    });
  }

  // Can Chi interpretation card
  if (canInfo && chiInfo) {
    const canElement = elements[canInfo.element];
    const chiElement = elements[chiInfo.element];
    
    const canChiContent = lang === "vi"
      ? [
          `${canName} (${canElement?.name.vi || canInfo.element}): ${canInfo.meaning.vi}`,
          `${chiName} - ${chiInfo.animal.vi} (${chiElement?.name.vi || chiInfo.element}): ${chiInfo.meaning.vi}`,
        ]
      : [
          `${canName} (${canElement?.name.en || canInfo.element}): ${canInfo.meaning.en}`,
          `${chiName} - ${chiInfo.animal.en} (${chiElement?.name.en || chiInfo.element}): ${chiInfo.meaning.en}`,
        ];

    cards.push({
      id: "canchi",
      title: lang === "vi" ? `Can Chi ngày: ${day.canchi_day}` : `Day's Can Chi: ${day.canchi_day}`,
      content: canChiContent,
      type: "list",
      extra: {
        canNature: canInfo.nature[lang],
        chiHours: chiInfo.hours,
      },
    });
  }

  // Day guidance card
  if (chiGuidance) {
    cards.push({
      id: "guidance",
      title: lang === "vi" ? "Hướng dẫn cho ngày" : "Day Guidance",
      content: "",
      type: "list",
      extra: {
        goodFor: {
          title: lang === "vi" ? "Nên làm" : "Good For",
          items: chiGuidance.goodFor[lang],
        },
        avoidFor: {
          title: lang === "vi" ? "Hạn chế" : "Avoid",
          items: chiGuidance.avoidFor[lang],
        },
      },
    });
  }

  // Agriculture & Health card (from tiết khí)
  if (tietKhi) {
    cards.push({
      id: "wellness",
      title: lang === "vi" ? "Nông nghiệp & Sức khỏe" : "Agriculture & Health",
      content: "",
      type: "list",
      extra: {
        agriculture: {
          title: lang === "vi" ? "Nông nghiệp" : "Agriculture",
          items: tietKhi.agriculture[lang],
        },
        health: {
          title: lang === "vi" ? "Sức khỏe" : "Health",
          items: tietKhi.health[lang],
        },
      },
    });
  }

  return {
    mode: "normal",
    termName: day.tiet_khi,
    termDescription: day.tiet_khi_description,
    canchiDay: day.canchi_day,
    cards,
  };
}

/**
 * Main function to build date insight
 */
export function buildDateInsight(day: DayForInsight, lang: Lang = "vi"): DateInsight {
  // Check if this day has a major festival (traditional lunar/solar festivals take priority)
  const festival = findFestival(day);
  
  if (festival) {
    return buildFestivalInsight(festival, day, lang);
  }

  // Check if this day has a national holiday
  const nationalHoliday = findNationalHoliday(day);

  if (nationalHoliday) {
    return buildNationalHolidayInsight(nationalHoliday, day, lang);
  }
  
  return buildNormalDayInsight(day, lang);
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

// ============================================
// Event Category Classification System
// ============================================

const CATEGORY_META: Record<EventCategoryType, {
  color: string;
  colorHex: string;
  tint: string;
  label: { vi: string; en: string };
  priority: number;
}> = {
  "festival":       { color: "var(--cat-festival)",       colorHex: "#C62828", tint: "var(--cat-festival-tint)",       label: { vi: "Lễ truyền thống", en: "Traditional Festival" }, priority: 1 },
  "public-holiday": { color: "var(--cat-public-holiday)", colorHex: "#AD1457", tint: "var(--cat-public-holiday-tint)", label: { vi: "Nghỉ lễ",          en: "Public Holiday" },        priority: 2 },
  "commemorative":  { color: "var(--cat-commemorative)",  colorHex: "#1565C0", tint: "var(--cat-commemorative-tint)",  label: { vi: "Kỷ niệm",         en: "Commemorative" },         priority: 3 },
  "professional":   { color: "var(--cat-professional)",   colorHex: "#2E7D32", tint: "var(--cat-professional-tint)",   label: { vi: "Ngành nghề",       en: "Professional Day" },      priority: 4 },
  "social":         { color: "var(--cat-social)",         colorHex: "#E65100", tint: "var(--cat-social-tint)",         label: { vi: "Xã hội",           en: "Social Day" },            priority: 5 },
  "international":  { color: "var(--cat-international)",  colorHex: "#6A1B9A", tint: "var(--cat-international-tint)",  label: { vi: "Quốc tế",          en: "International" },         priority: 6 },
  "solar-term":     { color: "var(--cat-solar-term)",     colorHex: "#00796B", tint: "var(--cat-solar-term-tint)",     label: { vi: "Tiết khí",         en: "Solar Term" },            priority: 7 },
  "lunar-cycle":    { color: "var(--cat-lunar-cycle)",    colorHex: "#D4AF37", tint: "var(--cat-lunar-cycle-tint)",    label: { vi: "Sóc/Vọng",         en: "Lunar 1st/15th" },        priority: 8 },
};

/**
 * Classify a single holiday by matching against known data sources.
 * Returns the EventCategoryType or null if unclassifiable.
 */
export function classifyHoliday(holiday: { name: string; is_solar: boolean; is_major: boolean; lunar_day: number | null; lunar_month: number | null }, day: DayForInsight): EventCategoryType {
  // 1. Check if it matches a traditional festival (lunar-based or Thanh Minh)
  const festival = findFestival(day);
  if (festival) {
    // If the holiday name overlaps with the festival, it's a festival
    const festivalNames = [...festival.names.vi, ...festival.names.en];
    for (const fname of festivalNames) {
      if (holiday.name.includes(fname) || fname.includes(holiday.name)) {
        return "festival";
      }
    }
    // If not solar and lunar dates match the festival, still a festival
    if (!holiday.is_solar && holiday.lunar_day === festival.lunarDay && holiday.lunar_month === festival.lunarMonth) {
      return "festival";
    }
  }

  // 2. Check if it matches a national holiday (solar-based)
  const natHoliday = findNationalHoliday(day);
  if (natHoliday && holiday.is_solar) {
    return natHoliday.category;
  }

  // 3. Check for lunar cycle (Mùng 1 / Rằm)
  if (holiday.name.startsWith("Mùng 1 tháng") || holiday.name.startsWith("Rằm tháng")) {
    return "lunar-cycle";
  }

  // 4. For remaining lunar holidays from the backend that aren't in festivals.json
  //    (e.g. Phật Đản, Trùng Cửu, etc.), treat as festival
  if (!holiday.is_solar && holiday.is_major) {
    return "festival";
  }

  // 5. Remaining solar holidays not in national-holidays.json — try to classify by name patterns
  if (holiday.is_solar) {
    if (holiday.is_major) return "public-holiday";
    return "commemorative";
  }

  return "lunar-cycle";
}

/**
 * Get all event categories present on a given day.
 * Returns categories sorted by priority (highest first).
 */
export function getDayEventCategories(day: DayForInsight, lang: Lang = "vi"): EventCategory[] {
  const categories: EventCategory[] = [];
  const seenTypes = new Set<EventCategoryType>();

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
    seenTypes.add(catType);
  }

  // Check if this is a solar term transition day (tiet_khi is meaningful)
  // We detect this by checking if the tiet_khi data exists in our dataset
  // and the day is the actual start date of that term
  // For simplicity, we check if the day has tiet_khi info and there's tiet_khi data
  const tietKhi = findTietKhi(day.tiet_khi);
  if (tietKhi && !seenTypes.has("solar-term")) {
    // Only show solar term dot on the approximate start day
    // The tiet_khi field is set for all days in that term range,
    // so we check if this is lunar day 1 of a month or a specific known start
    // Actually, tiet_khi changes happen roughly every 15 days
    // For now, we'll skip auto-adding solar term dots to keep the grid clean
    // Solar terms show up in the insight panel already
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
