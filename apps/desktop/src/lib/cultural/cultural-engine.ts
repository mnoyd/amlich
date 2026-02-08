export type Lang = "vi" | "en";

type HolidayInfo = {
  name: string;
  description: string;
  is_solar: boolean;
  lunar_day: number | null;
  lunar_month: number | null;
  is_major: boolean;
};

export type DayForCultural = {
  day: number;
  month: number;
  year: number;
  lunar_day: number;
  lunar_month: number;
  lunar_date: string;
  canchi_day: string;
  tiet_khi: string;
  tiet_khi_description: string;
  tiet_khi_season: string;
  holidays: HolidayInfo[];
};

type EventCard = {
  title: string;
  whatIsIt: string;
  origin: string;
  dos: string[];
  donts: string[];
  traditionalLife: string;
};

export type CulturalContent =
  | {
      mode: "event";
      title: string;
      subtitle: string;
      events: EventCard[];
    }
  | {
      mode: "solar-term";
      title: string;
      subtitle: string;
      termName: string;
      termMeaning: string;
      whyItExists: string;
      seasonalContext: string;
      dos: string[];
      donts: string[];
      sourceNote: string;
    };

const TERM_HINTS: Record<string, { vi: string; en: string }> = {
  "Lập Xuân": {
    vi: "Mốc mở đầu mùa xuân theo hệ 24 tiết khí.",
    en: "Marks the beginning of spring in the 24-solar-term system.",
  },
  "Vũ Thủy": {
    vi: "Thời kỳ mưa ẩm tăng, đất bắt đầu mềm và giữ nước tốt hơn.",
    en: "A wetter phase when humidity rises and soil retains more water.",
  },
  "Kinh Trập": {
    vi: "Khí trời ấm dần, biểu tượng cho sự thức dậy của côn trùng và sinh giới.",
    en: "Warming weather symbolically marks the awakening of insects and life.",
  },
  "Xuân Phân": {
    vi: "Điểm cân bằng ngày đêm, giữa mùa xuân.",
    en: "An equinox point with near-equal day and night in spring.",
  },
  "Thanh Minh": {
    vi: "Giai đoạn trời trong sáng, thường gắn với nếp tảo mộ và dọn dẹp.",
    en: "A clearer-weather phase, often linked to grave-visiting and home reset.",
  },
  "Cốc Vũ": {
    vi: "Mưa thuận cho cây cối và nhịp canh tác đầu vụ.",
    en: "Rain-supportive period for crops and early-season cultivation.",
  },
  "Lập Hạ": {
    vi: "Bắt đầu mùa hạ trong hệ tiết khí.",
    en: "Marks the beginning of summer in the solar-term cycle.",
  },
  "Tiểu Mãn": {
    vi: "Thời kỳ ngũ cốc bắt đầu đầy đặn nhưng chưa chín hẳn.",
    en: "Grains begin to fill out but are not fully ripe yet.",
  },
  "Mang Chủng": {
    vi: "Mốc gợi nhịp gieo trồng và chăm cây theo mùa.",
    en: "A term traditionally guiding sowing and crop-care timing.",
  },
  "Hạ Chí": {
    vi: "Điểm chí hạ - thời gian chiếu sáng dài trong năm.",
    en: "Summer solstice point with one of the longest daylight spans.",
  },
  "Tiểu Thử": {
    vi: "Nóng tăng dần, phù hợp điều chỉnh nhịp sinh hoạt mùa hè.",
    en: "Heat builds up; a cue to adjust summer routines.",
  },
  "Đại Thử": {
    vi: "Giai đoạn nóng đỉnh điểm, cần ưu tiên phục hồi thể lực.",
    en: "Peak-heat period where recovery and hydration matter most.",
  },
  "Lập Thu": {
    vi: "Mốc bắt đầu mùa thu theo lịch tiết khí.",
    en: "Marks the beginning of autumn in the solar-term calendar.",
  },
  "Xử Thử": {
    vi: "Nhiệt oi giảm dần, chuyển nhịp sang tiết trời mát hơn.",
    en: "Lingering heat recedes as weather shifts cooler.",
  },
  "Bạch Lộ": {
    vi: "Sáng sớm có độ ẩm cao hơn, dễ xuất hiện sương nhẹ.",
    en: "Morning humidity increases, often linked to dew formation.",
  },
  "Thu Phân": {
    vi: "Điểm phân thu - ngày đêm gần cân bằng.",
    en: "Autumn equinox point with near-balanced day and night.",
  },
  "Hàn Lộ": {
    vi: "Tiết trời lạnh rõ hơn, chuyển dần sang cuối thu.",
    en: "Coolness becomes more apparent as late autumn arrives.",
  },
  "Sương Giáng": {
    vi: "Nhiệt độ hạ, khả năng xuất hiện sương muối tăng ở vùng lạnh.",
    en: "Lower temperatures raise frost likelihood in colder areas.",
  },
  "Lập Đông": {
    vi: "Mốc mở đầu mùa đông trong hệ 24 tiết khí.",
    en: "Marks the beginning of winter in the 24-term system.",
  },
  "Tiểu Tuyết": {
    vi: "Tên tiết khí phản ánh vùng khí hậu lạnh; ở Việt Nam đa phần không có tuyết.",
    en: "Named for colder-climate patterns; most of Vietnam sees little to no snow.",
  },
  "Đại Tuyết": {
    vi: "Biểu tượng giai đoạn rét tăng mạnh ở khí hậu ôn đới.",
    en: "Represents deeper winter cold in temperate climates.",
  },
  "Đông Chí": {
    vi: "Điểm chí đông - mốc thiên văn quan trọng của mùa lạnh.",
    en: "Winter solstice, a key astronomical marker of the cold season.",
  },
  "Tiểu Hàn": {
    vi: "Rét rõ nhưng chưa cực điểm.",
    en: "Noticeably cold, but not yet the peak.",
  },
  "Đại Hàn": {
    vi: "Giai đoạn lạnh sâu trong chu kỳ năm.",
    en: "One of the coldest phases in the annual cycle.",
  },
};

const SEASONAL_DOS: Record<string, { vi: string[]; en: string[] }> = {
  xuân: {
    vi: [
      "Sắp xếp lại lịch làm việc theo nhịp mới đầu mùa",
      "Tăng vận động nhẹ ngoài trời",
      "Ưu tiên bữa ăn thanh và đủ rau",
    ],
    en: [
      "Reset your schedule for the new seasonal rhythm",
      "Increase light outdoor movement",
      "Favor lighter, vegetable-rich meals",
    ],
  },
  hạ: {
    vi: [
      "Bổ sung nước đều trong ngày",
      "Dời việc nặng sang sáng sớm hoặc chiều muộn",
      "Ngủ đủ để phục hồi sau nóng",
    ],
    en: [
      "Hydrate consistently through the day",
      "Move heavy tasks to cooler hours",
      "Sleep enough for heat recovery",
    ],
  },
  thu: {
    vi: [
      "Ổn định nhịp học tập và công việc sau mùa hè",
      "Theo dõi sức khỏe hô hấp khi thời tiết khô hơn",
      "Dành thời gian sinh hoạt gia đình buổi tối",
    ],
    en: [
      "Stabilize work and study rhythm after summer",
      "Watch respiratory health in drier weather",
      "Make time for evening family routines",
    ],
  },
  đông: {
    vi: [
      "Giữ ấm, ngủ đủ và ăn uống điều độ",
      "Giảm cường độ công việc quá sức",
      "Tăng thời gian sinh hoạt trong nhà với gia đình",
    ],
    en: [
      "Keep warm, sleep enough, and eat regularly",
      "Reduce overstraining workloads",
      "Spend more quality family time indoors",
    ],
  },
};

const SEASONAL_DONTS: Record<string, { vi: string[]; en: string[] }> = {
  xuân: {
    vi: ["Ôm quá nhiều việc mới cùng lúc", "Bỏ qua nề nếp ngủ nghỉ"],
    en: ["Taking on too many new tasks at once", "Ignoring sleep discipline"],
  },
  hạ: {
    vi: ["Làm việc nặng dưới nắng gắt kéo dài", "Bỏ qua dấu hiệu mất nước"],
    en: ["Sustained heavy work under harsh sun", "Ignoring dehydration signs"],
  },
  thu: {
    vi: ["Thức khuya liên tục", "Để lịch sinh hoạt thiếu ổn định"],
    en: ["Frequent late nights", "Keeping an unstable routine"],
  },
  đông: {
    vi: ["Chủ quan với rét sớm", "Ít vận động trong thời gian dài"],
    en: ["Underestimating early cold", "Prolonged inactivity"],
  },
};

function seasonKey(input: string): "xuân" | "hạ" | "thu" | "đông" {
  const lowered = input.toLowerCase();
  if (lowered.includes("xuân") || lowered.includes("spring")) return "xuân";
  if (lowered.includes("hạ") || lowered.includes("summer")) return "hạ";
  if (lowered.includes("thu") || lowered.includes("autumn")) return "thu";
  return "đông";
}

function holidayWhatIsIt(h: HolidayInfo, day: DayForCultural, lang: Lang): string {
  if (lang === "vi") {
    if (h.description) {
      return `${h.description}. Sự kiện này diễn ra vào âm lịch ${day.lunar_date}.`;
    }
    return `Đây là mốc sinh hoạt văn hóa theo lịch ${h.is_solar ? "dương" : "âm"}, gắn với ngày ${day.lunar_date}.`;
  }

  if (h.description) {
    return `${h.description}. On this date, it aligns with lunar ${day.lunar_date}.`;
  }
  return `This is a cultural observance tied to the ${h.is_solar ? "solar" : "lunar"} calendar and today's lunar date ${day.lunar_date}.`;
}

function holidayOrigin(h: HolidayInfo, lang: Lang): string {
  if (lang === "vi") {
    if (h.is_major) {
      return "Ngày này thường có chiều sâu lịch sử - tín ngưỡng, phản ánh nếp sống gia đình, cộng đồng và truyền thống thờ kính tổ tiên trong văn hóa Việt.";
    }
    return "Ngày này thường bắt nguồn từ nếp lịch tháng âm và kinh nghiệm dân gian để giữ nhịp sống điều độ.";
  }

  if (h.is_major) {
    return "This observance usually carries deeper historical and spiritual layers, reflecting family bonds, community memory, and ancestor-centered Vietnamese traditions.";
  }
  return "This date commonly comes from monthly lunar rhythm and practical folk habits that regulate daily life.";
}

function holidayDos(h: HolidayInfo, lang: Lang): string[] {
  if (h.is_major) {
    return lang === "vi"
      ? [
          "Dành thời gian cho gia đình và người lớn tuổi",
          "Giữ không khí hòa nhã khi gặp gỡ họ hàng",
          "Sắp xếp lễ nghi vừa phải, tránh hình thức",
        ]
      : [
          "Make time for family and elders",
          "Keep a calm, respectful atmosphere in gatherings",
          "Keep rituals meaningful and moderate",
        ];
  }

  return lang === "vi"
    ? [
        "Dùng ngày này để rà soát nề nếp sinh hoạt",
        "Ưu tiên các việc gia đình đang dang dở",
        "Giữ lịch ăn ngủ ổn định",
      ]
    : [
        "Use the day to reset household routines",
        "Prioritize pending family tasks",
        "Keep sleep and meal timing stable",
      ];
}

function holidayDonts(h: HolidayInfo, lang: Lang): string[] {
  if (h.is_major) {
    return lang === "vi"
      ? [
          "Tranh cãi lớn tiếng trong dịp sum họp",
          "Chi tiêu phô trương quá mức",
          "Biến nghi lễ thành áp lực",
        ]
      : [
          "Avoid heated arguments during gatherings",
          "Avoid excessive showy spending",
          "Do not turn rituals into stress",
        ];
  }

  return lang === "vi"
    ? ["Bỏ qua sức khỏe cá nhân", "Để lịch sinh hoạt kéo dài mất cân bằng"]
    : ["Ignoring personal health", "Letting routines remain imbalanced for too long"];
}

function holidayTraditionalLife(day: DayForCultural, lang: Lang): string {
  if (lang === "vi") {
    if (day.lunar_day === 1 || day.lunar_day === 15) {
      return "Với mùng 1 hoặc rằm, nhiều gia đình Việt ưu tiên bữa cơm thanh, thắp hương gia tiên và nhắc nhau giữ lời nói ôn hòa.";
    }
    return "Trong nếp sống truyền thống, các ngày có sự kiện là dịp nối lại quan hệ gia đình và giữ nhịp sinh hoạt theo âm lịch.";
  }

  if (day.lunar_day === 1 || day.lunar_day === 15) {
    return "On lunar day 1 or 15, many Vietnamese families keep simple meals, offer incense to ancestors, and emphasize gentle speech.";
  }
  return "In traditional practice, event days help renew family ties and keep daily life aligned with lunar rhythm.";
}

function buildEventContent(day: DayForCultural, lang: Lang): CulturalContent {
  const events: EventCard[] = day.holidays.map((h) => ({
    title: h.name,
    whatIsIt: holidayWhatIsIt(h, day, lang),
    origin: holidayOrigin(h, lang),
    dos: holidayDos(h, lang),
    donts: holidayDonts(h, lang),
    traditionalLife: holidayTraditionalLife(day, lang),
  }));

  return {
    mode: "event",
    title: lang === "vi" ? "Sự kiện trong ngày" : "Events of the Day",
    subtitle:
      lang === "vi"
        ? `Hôm nay có ${events.length} sự kiện theo lịch Việt. Nội dung bên dưới được tạo theo từng sự kiện của ngày đã chọn.`
        : `Today has ${events.length} Vietnamese calendar observances. The content below is generated from each event on this selected date.`,
    events,
  };
}

function buildSolarTermContent(day: DayForCultural, lang: Lang): CulturalContent {
  const key = seasonKey(day.tiet_khi_season);
  const hint = TERM_HINTS[day.tiet_khi];
  const dos = SEASONAL_DOS[key][lang];
  const donts = SEASONAL_DONTS[key][lang];

  const termMeaning =
    lang === "vi"
      ? hint?.vi ?? `${day.tiet_khi} là một mốc trong hệ 24 tiết khí, thường kéo dài khoảng 15 ngày trước khi chuyển tiết kế tiếp.`
      : hint?.en ?? `${day.tiet_khi} is one marker in the 24-solar-term system, typically lasting around 15 days before the next transition.`;

  const whyItExists =
    lang === "vi"
      ? "Tiết khí tồn tại để đồng bộ lịch đời sống với mùa. Hệ này chia đường đi biểu kiến của Mặt Trời thành 24 mốc, mỗi mốc cách nhau 15° kinh độ hoàng đạo. Vì quỹ đạo Trái Đất là elip, khoảng cách thời gian giữa các tiết thường dao động khoảng 14-16 ngày."
      : "Solar terms exist to align daily calendars with seasonal change. The system divides the Sun's apparent path into 24 points, each 15° apart in ecliptic longitude. Because Earth's orbit is elliptical, spacing between terms is usually about 14-16 days.";

  const seasonalContext =
    lang === "vi"
      ? `Tiết hiện tại: ${day.tiet_khi} (${day.tiet_khi_season}). Trong nếp sống Việt, tiết khí giúp định nhịp việc nhà, chăm sức khỏe và các quyết định mùa vụ.`
      : `Current term: ${day.tiet_khi} (${day.tiet_khi_season}). In Vietnamese practice, solar terms guide household rhythm, health habits, and seasonal planning.`;

  const sourceNote =
    lang === "vi"
      ? "Tổng hợp kiến thức: Solar term (Wikipedia EN), Tiết khí (Wikipedia VI), đối chiếu với dữ liệu tiết khí trong ứng dụng."
      : "Knowledge sources: Solar term (Wikipedia EN), Tiết khí (Wikipedia VI), aligned with this app's solar-term data.";

  return {
    mode: "solar-term",
    title: lang === "vi" ? "Đọc ngày theo Tiết Khí" : "Read the Day by Solar Term",
    subtitle:
      lang === "vi"
        ? "Ngày thường không có lễ lớn sẽ được diễn giải theo tiết khí hiện tại để giữ đúng nhịp âm lịch - mùa vụ."
        : "On non-event days, the panel is generated from the active solar term to keep lunar-season context meaningful.",
    termName: day.tiet_khi,
    termMeaning,
    whyItExists,
    seasonalContext,
    dos,
    donts,
    sourceNote,
  };
}

export function buildCulturalContent(day: DayForCultural, lang: Lang): CulturalContent {
  if (day.holidays.length > 0) {
    return buildEventContent(day, lang);
  }
  return buildSolarTermContent(day, lang);
}
