import type {
  FestivalData,
  NationalHolidayData,
  TietKhiData,
  CanInfo,
  ChiInfo,
  DayGuidance,
  EventCategoryType,
} from "./types";

import festivalsData from "../../../../../data/holidays/lunar-festivals.json";
import nationalHolidaysData from "../../../../../data/holidays/solar-holidays.json";
import tietKhiData from "../../../../../data/tiet-khi.json";
import canchiData from "../../../../../data/canchi.json";

export const festivals = festivalsData.festivals as FestivalData[];
export const nationalHolidays = nationalHolidaysData.holidays as NationalHolidayData[];
export const tietKhiList = tietKhiData.tietKhi as TietKhiData[];
export const canList = canchiData.can as CanInfo[];
export const chiList = canchiData.chi as ChiInfo[];
export const elements = canchiData.elements as Record<
  string,
  { name: { vi: string; en: string }; nature: { vi: string; en: string } }
>;
export const dayGuidanceMap = canchiData.dayGuidance as Record<string, DayGuidance>;

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
