import type {
  FestivalData,
  NationalHolidayData,
  TietKhiData,
  CanInfo,
  ChiInfo,
  DayGuidance,
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

function festivalKey(month: number, day: number): string {
  return `${month}-${day}`;
}

const lunarFestivalIndex = new Map<string, FestivalData>();
const solarFestivalIndex = new Map<string, FestivalData>();
for (const f of festivals) {
  if (f.isSolar && f.solarMonth != null && f.solarDay != null) {
    solarFestivalIndex.set(festivalKey(f.solarMonth, f.solarDay), f);
  } else {
    lunarFestivalIndex.set(festivalKey(f.lunarMonth, f.lunarDay), f);
  }
}
export { lunarFestivalIndex, solarFestivalIndex };

const nationalHolidayIndex = new Map<string, NationalHolidayData>();
for (const h of nationalHolidays) {
  nationalHolidayIndex.set(festivalKey(h.solarMonth, h.solarDay), h);
}
export { nationalHolidayIndex };

const tietKhiIndex = new Map<string, TietKhiData>();
for (const t of tietKhiList) {
  tietKhiIndex.set(t.name.vi, t);
}
export { tietKhiIndex };

const canIndex = new Map<string, CanInfo>();
for (const c of canList) {
  canIndex.set(c.name, c);
}
export { canIndex };

const chiIndex = new Map<string, ChiInfo>();
for (const c of chiList) {
  chiIndex.set(c.name, c);
}
export { chiIndex };
