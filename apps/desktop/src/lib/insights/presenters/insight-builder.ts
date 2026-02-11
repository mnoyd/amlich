/**
 * Insight card builders.
 * Transforms domain data into UI-ready InsightCard structures.
 */

import type {
  Lang,
  DayForInsight,
  FestivalData,
  NationalHolidayData,
} from "../types/domain";

import type {
  InsightCard,
  FestivalInsight,
  NationalHolidayInsight,
  NormalDayInsight,
  DateInsight,
  DateInsightMulti,
  AnyDateInsight,
} from "../types/view";

import {
  elements,
  dayGuidanceMap,
} from "../shared-data";

import {
  findFestival,
  findNationalHoliday,
  findTietKhi,
  parseCanChi,
  findCan,
  findChi,
} from "../domain/selectors";

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
  if (festival.food && festival.food.length > 0) {
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
  if (festival.taboos && festival.taboos.length > 0) {
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
  if (festival.proverbs && festival.proverbs.length > 0) {
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
  if (festival.regions) {
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
  }

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
export function buildDateInsight(day: DayForInsight, lang: Lang = "vi"): AnyDateInsight {
  const festival = findFestival(day);
  const nationalHoliday = findNationalHoliday(day);

  const sections: DateInsight[] = [];

  if (festival) {
    sections.push(buildFestivalInsight(festival, day, lang));
  }

  if (nationalHoliday) {
    sections.push(buildNationalHolidayInsight(nationalHoliday, day, lang));
  }

  const normalInsight = buildNormalDayInsight(day, lang);

  if (sections.length === 0) {
    return normalInsight;
  }

  if (sections.length === 1) {
    sections.push(normalInsight);
  }

  if (sections.length > 1) {
    const title =
      lang === "vi"
        ? `Nhiều sự kiện trong ngày (${sections.length})`
        : `Multiple events today (${sections.length})`;
    const subtitle =
      lang === "vi"
        ? "Lễ/ngày kỷ niệm và bối cảnh tiết khí được hiển thị cùng nhau"
        : "Festival/holiday context and solar-term context shown together";

    const multi: DateInsightMulti = {
      mode: "multi",
      title,
      subtitle,
      sections,
    };

    return multi;
  }

  return sections[0];
}
