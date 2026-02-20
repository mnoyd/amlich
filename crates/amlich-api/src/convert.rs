use crate::dto::{
    CanChiDto, CanChiInfoDto, CanInsightDto, ChiInsightDto, DayGuidanceDto, DayInfoDto,
    ElementInsightDto, FestivalInsightDto, FoodInsightDto, GioHoangDaoDto, HolidayDto,
    HolidayInsightDto, HourInfoDto, LocalizedListDto, LocalizedTextDto, LunarDto, NguHanhDto,
    ProverbInsightDto, RegionsInsightDto, SolarDto, TabooInsightDto, TietKhiDto, TietKhiInsightDto,
};

impl From<&amlich_core::NguHanh> for NguHanhDto {
    fn from(value: &amlich_core::NguHanh) -> Self {
        Self {
            can: value.can.clone(),
            chi: value.chi.clone(),
        }
    }
}

impl From<&amlich_core::CanChi> for CanChiDto {
    fn from(value: &amlich_core::CanChi) -> Self {
        Self {
            can_index: value.can_index,
            chi_index: value.chi_index,
            can: value.can.clone(),
            chi: value.chi.clone(),
            full: value.full.clone(),
            con_giap: value.con_giap.clone(),
            ngu_hanh: NguHanhDto::from(&value.ngu_hanh),
        }
    }
}

impl From<&amlich_core::CanChiInfo> for CanChiInfoDto {
    fn from(value: &amlich_core::CanChiInfo) -> Self {
        Self {
            day: CanChiDto::from(&value.day),
            month: CanChiDto::from(&value.month),
            year: CanChiDto::from(&value.year),
            full: value.full.clone(),
        }
    }
}

impl From<&amlich_core::SolarInfo> for SolarDto {
    fn from(value: &amlich_core::SolarInfo) -> Self {
        Self {
            day: value.day,
            month: value.month,
            year: value.year,
            day_of_week: value.day_of_week,
            day_of_week_name: value.day_of_week_name.clone(),
            date_string: value.date_string.clone(),
        }
    }
}

impl From<&amlich_core::LunarInfo> for LunarDto {
    fn from(value: &amlich_core::LunarInfo) -> Self {
        Self {
            day: value.day,
            month: value.month,
            year: value.year,
            is_leap_month: value.is_leap_month,
            date_string: value.date_string.clone(),
        }
    }
}

impl From<&amlich_core::tietkhi::SolarTerm> for TietKhiDto {
    fn from(value: &amlich_core::tietkhi::SolarTerm) -> Self {
        Self {
            index: value.index,
            name: value.name.clone(),
            description: value.description.clone(),
            longitude: value.longitude,
            current_longitude: value.current_longitude,
            season: value.season.clone(),
        }
    }
}

impl From<&amlich_core::gio_hoang_dao::HourInfo> for HourInfoDto {
    fn from(value: &amlich_core::gio_hoang_dao::HourInfo) -> Self {
        Self {
            hour_index: value.hour_index,
            hour_chi: value.hour_chi.clone(),
            time_range: value.time_range.clone(),
            star: value.star.clone(),
            is_good: value.is_good,
        }
    }
}

impl From<&amlich_core::gio_hoang_dao::GioHoangDao> for GioHoangDaoDto {
    fn from(value: &amlich_core::gio_hoang_dao::GioHoangDao) -> Self {
        Self {
            day_chi: value.day_chi.clone(),
            good_hour_count: value.good_hour_count,
            good_hours: value.good_hours.iter().map(HourInfoDto::from).collect(),
            all_hours: value.all_hours.iter().map(HourInfoDto::from).collect(),
            summary: value.summary.clone(),
        }
    }
}

impl From<&amlich_core::DayInfo> for DayInfoDto {
    fn from(value: &amlich_core::DayInfo) -> Self {
        Self {
            solar: SolarDto::from(&value.solar),
            lunar: LunarDto::from(&value.lunar),
            jd: value.jd,
            canchi: CanChiInfoDto::from(&value.canchi),
            tiet_khi: TietKhiDto::from(&value.tiet_khi),
            gio_hoang_dao: GioHoangDaoDto::from(&value.gio_hoang_dao),
        }
    }
}

impl From<&amlich_core::holidays::Holiday> for HolidayDto {
    fn from(value: &amlich_core::holidays::Holiday) -> Self {
        Self {
            name: value.name.clone(),
            description: value.description.clone(),
            solar_day: value.solar_day,
            solar_month: value.solar_month,
            solar_year: value.solar_year,
            lunar_day: value.lunar_date.as_ref().map(|d| d.day),
            lunar_month: value.lunar_date.as_ref().map(|d| d.month),
            lunar_year: value.lunar_date.as_ref().map(|d| d.year),
            is_solar: value.is_solar,
            category: value.category.clone(),
            is_major: value.is_major,
        }
    }
}

impl From<&amlich_core::holiday_data::BilingualText> for LocalizedTextDto {
    fn from(value: &amlich_core::holiday_data::BilingualText) -> Self {
        Self {
            vi: value.vi.clone(),
            en: value.en.clone(),
        }
    }
}

impl From<&amlich_core::insight_data::BilingualText> for LocalizedTextDto {
    fn from(value: &amlich_core::insight_data::BilingualText) -> Self {
        Self {
            vi: value.vi.clone(),
            en: value.en.clone(),
        }
    }
}

impl From<&amlich_core::insight_data::BilingualList> for LocalizedListDto {
    fn from(value: &amlich_core::insight_data::BilingualList) -> Self {
        Self {
            vi: value.vi.clone(),
            en: value.en.clone(),
        }
    }
}

impl From<&amlich_core::holiday_data::BilingualList> for LocalizedListDto {
    fn from(value: &amlich_core::holiday_data::BilingualList) -> Self {
        Self {
            vi: value.vi.clone(),
            en: value.en.clone(),
        }
    }
}

impl From<&amlich_core::holiday_data::FoodItem> for FoodInsightDto {
    fn from(value: &amlich_core::holiday_data::FoodItem) -> Self {
        Self {
            name: LocalizedTextDto::from(&value.name),
            description: LocalizedTextDto::from(&value.description),
        }
    }
}

impl From<&amlich_core::holiday_data::TabooItem> for TabooInsightDto {
    fn from(value: &amlich_core::holiday_data::TabooItem) -> Self {
        Self {
            action: LocalizedTextDto::from(&value.action),
            reason: LocalizedTextDto::from(&value.reason),
        }
    }
}

impl From<&amlich_core::holiday_data::ProverbItem> for ProverbInsightDto {
    fn from(value: &amlich_core::holiday_data::ProverbItem) -> Self {
        Self {
            text: value.text.clone(),
            meaning: LocalizedTextDto::from(&value.meaning),
        }
    }
}

impl From<&amlich_core::holiday_data::Regions> for RegionsInsightDto {
    fn from(value: &amlich_core::holiday_data::Regions) -> Self {
        Self {
            north: LocalizedTextDto::from(&value.north),
            central: LocalizedTextDto::from(&value.central),
            south: LocalizedTextDto::from(&value.south),
        }
    }
}

impl From<&amlich_core::holiday_data::LunarFestivalData> for FestivalInsightDto {
    fn from(value: &amlich_core::holiday_data::LunarFestivalData) -> Self {
        Self {
            names: LocalizedListDto {
                vi: value.names.vi.clone(),
                en: value.names.en.clone(),
            },
            origin: value.origin.as_ref().map(LocalizedTextDto::from),
            activities: value.activities.as_ref().map(LocalizedListDto::from),
            food: value.food.iter().map(FoodInsightDto::from).collect(),
            taboos: value.taboos.iter().map(TabooInsightDto::from).collect(),
            proverbs: value.proverbs.iter().map(ProverbInsightDto::from).collect(),
            regions: value.regions.as_ref().map(RegionsInsightDto::from),
            category: value.category.clone(),
            is_major: value.is_major,
        }
    }
}

impl From<&amlich_core::holiday_data::SolarHolidayData> for HolidayInsightDto {
    fn from(value: &amlich_core::holiday_data::SolarHolidayData) -> Self {
        Self {
            names: LocalizedListDto {
                vi: value.names.vi.clone(),
                en: value.names.en.clone(),
            },
            origin: value.origin.as_ref().map(LocalizedTextDto::from),
            significance: value.significance.as_ref().map(LocalizedTextDto::from),
            activities: value.activities.as_ref().map(LocalizedListDto::from),
            traditions: value.traditions.as_ref().map(LocalizedListDto::from),
            food: value.food.iter().map(FoodInsightDto::from).collect(),
            taboos: value.taboos.iter().map(TabooInsightDto::from).collect(),
            proverbs: value.proverbs.iter().map(ProverbInsightDto::from).collect(),
            regions: value.regions.as_ref().map(RegionsInsightDto::from),
            category: value.category.clone(),
            is_major: value.is_major,
        }
    }
}

impl From<(&String, &amlich_core::insight_data::ElementInfo)> for ElementInsightDto {
    fn from((key, value): (&String, &amlich_core::insight_data::ElementInfo)) -> Self {
        Self {
            key: key.clone(),
            name: LocalizedTextDto::from(&value.name),
            nature: LocalizedTextDto::from(&value.nature),
        }
    }
}

impl From<&amlich_core::insight_data::CanInfo> for CanInsightDto {
    fn from(value: &amlich_core::insight_data::CanInfo) -> Self {
        Self {
            name: value.name.clone(),
            element: value.element.clone(),
            meaning: LocalizedTextDto::from(&value.meaning),
            nature: LocalizedTextDto::from(&value.nature),
        }
    }
}

impl From<&amlich_core::insight_data::ChiInfo> for ChiInsightDto {
    fn from(value: &amlich_core::insight_data::ChiInfo) -> Self {
        Self {
            name: value.name.clone(),
            animal: LocalizedTextDto::from(&value.animal),
            element: value.element.clone(),
            meaning: LocalizedTextDto::from(&value.meaning),
            hours: value.hours.clone(),
        }
    }
}

impl From<&amlich_core::insight_data::DayGuidance> for DayGuidanceDto {
    fn from(value: &amlich_core::insight_data::DayGuidance) -> Self {
        Self {
            good_for: LocalizedListDto::from(&value.good_for),
            avoid_for: LocalizedListDto::from(&value.avoid_for),
        }
    }
}

impl From<&amlich_core::insight_data::TietKhiInsight> for TietKhiInsightDto {
    fn from(value: &amlich_core::insight_data::TietKhiInsight) -> Self {
        Self {
            id: value.id.clone(),
            name: LocalizedTextDto::from(&value.name),
            longitude: value.longitude,
            meaning: LocalizedTextDto::from(&value.meaning),
            astronomy: LocalizedTextDto::from(&value.astronomy),
            agriculture: LocalizedListDto::from(&value.agriculture),
            health: LocalizedListDto::from(&value.health),
            weather: LocalizedTextDto::from(&value.weather),
        }
    }
}
