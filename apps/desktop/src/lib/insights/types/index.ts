/**
 * Barrel re-export for all insight types.
 * Maintains backwards compatibility — consumers can import from "./types"
 * or choose "./types/domain" / "./types/view" for explicit separation.
 */

export type {
  // Domain types
  Lang,
  HolidayInfo,
  GoodHour,
  DayCell,
  MonthData,
  BilingualText,
  BilingualList,
  FoodItem,
  TabooItem,
  ProverbItem,
  RegionalCustoms,
  FestivalData,
  NationalHolidayData,
  TietKhiData,
  ElementInfo,
  CanInfo,
  ChiInfo,
  DayGuidance,
  CanChiData,
  EventCategoryType,
  DayForInsight,
} from "./domain";

export type {
  // View types
  GuidanceSection,
  RegionSection,
  ProverbDisplay,
  InsightCardExtra,
  InsightCard,
  FestivalInsight,
  NationalHolidayInsight,
  NormalDayInsight,
  DateInsight,
  DateInsightMulti,
  AnyDateInsight,
  EventCategory,
} from "./view";
