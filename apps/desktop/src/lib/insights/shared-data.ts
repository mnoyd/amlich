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
