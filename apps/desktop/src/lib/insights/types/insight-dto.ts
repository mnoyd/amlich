import type { BilingualList, BilingualText } from "./domain";

export type FestivalInsightDto = {
  names: BilingualList;
  origin?: BilingualText | null;
  activities?: BilingualList | null;
};

export type HolidayInsightDto = {
  names: BilingualList;
  origin?: BilingualText | null;
  significance?: BilingualText | null;
  traditions?: BilingualList | null;
};

export type CanChiInsightDto = {
  can: { name: string; meaning: BilingualText };
  chi: { name: string; animal: BilingualText; meaning: BilingualText };
};

export type DayGuidanceInsightDto = {
  good_for: BilingualList;
  avoid_for: BilingualList;
};

export type TietKhiInsightDto = {
  name: BilingualText;
  weather: BilingualText;
  agriculture: BilingualList;
  health: BilingualList;
};

export type DayInsightDto = {
  festival?: FestivalInsightDto | null;
  holiday?: HolidayInsightDto | null;
  canchi?: CanChiInsightDto | null;
  day_guidance?: DayGuidanceInsightDto | null;
  tiet_khi?: TietKhiInsightDto | null;
};
