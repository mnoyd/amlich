use crate::dto::{
    CanChiDto, CanChiInfoDto, DayInfoDto, GioHoangDaoDto, HolidayDto, HourInfoDto, LunarDto,
    NguHanhDto, SolarDto, TietKhiDto,
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
