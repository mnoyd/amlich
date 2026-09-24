//! Day View Model v1 — the shared surface projection behind the replacement
//! desktop and TUI (`amlich-b14l.7`, `docs/surface-replacement-slices.md` §3 S1).
//!
//! Pure projection over `get_day_info` outputs: grouping, labeling, and
//! navigation only. No assessment math, no re-evaluation, no new claims.

use serde::{Deserialize, Serialize};

use crate::dto::{CanChiInfoDto, DateQuery, DayInfoDto, LunarDto, SolarDto, TietKhiDto};
use crate::get_day_info;

pub const DAY_VIEW_SCHEMA_VERSION: &str = "day-view-v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DayViewPatternItemKindDto {
    Support,
    Constraint,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DayViewSignalSourceDto {
    HoangDaoHours,
    Truc,
    CatTinh,
    SatTinh,
    DayDeity,
    DayConflict,
    Taboo,
    PersonalContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayViewPatternItemDto {
    pub kind: DayViewPatternItemKindDto,
    pub title: String,
    pub reason: String,
    pub source: DayViewSignalSourceDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayViewPatternDto {
    pub supports: Vec<DayViewPatternItemDto>,
    pub constraints: Vec<DayViewPatternItemDto>,
    pub unknowns: Vec<DayViewPatternItemDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayViewHourDto {
    pub hour_index: usize,
    pub chi: String,
    pub time_range: String,
    pub star: String,
    pub is_hoang_dao: bool,
    pub is_notable: bool,
    pub notable_reason: Option<String>,
    pub is_current: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayViewHourTimelineDto {
    pub hours: Vec<DayViewHourDto>,
    pub current_hour_index: Option<usize>,
    pub notable_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DayViewCoverageFamilyDto {
    Canchi,
    TietKhi,
    Truc,
    Stars,
    DayDeity,
    Taboos,
    DayConflict,
    HoangDaoHours,
    DayElement,
    Recommendations,
    Intent,
    BirthProfile,
    Location,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DayViewCoverageStateDto {
    Present,
    Unknown,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayViewCoverageEntryDto {
    pub family: DayViewCoverageFamilyDto,
    pub state: DayViewCoverageStateDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayViewCoverageDto {
    pub entries: Vec<DayViewCoverageEntryDto>,
    pub present_count: usize,
    pub unknown_count: usize,
    pub pending_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayViewDto {
    pub schema_version: String,
    pub solar: SolarDto,
    pub lunar: LunarDto,
    pub canchi: CanChiInfoDto,
    pub tiet_khi: TietKhiDto,
    pub pattern: DayViewPatternDto,
    pub hours: DayViewHourTimelineDto,
    pub coverage: DayViewCoverageDto,
}

pub fn current_chi_index_from_hour(hour: u32) -> usize {
    (((hour + 1) % 24) / 2) as usize
}

pub fn get_day_view(
    query: &DateQuery,
    current_chi_index: Option<usize>,
) -> Result<DayViewDto, String> {
    let info = get_day_info(query)?;
    Ok(project_day_view(&info, current_chi_index))
}

pub fn get_day_view_for_date(
    day: i32,
    month: i32,
    year: i32,
    current_chi_index: Option<usize>,
) -> Result<DayViewDto, String> {
    get_day_view(
        &DateQuery {
            day,
            month,
            year,
            timezone: None,
            ruleset_id: None,
            event_kind: None,
            enabled_pack_ids: vec![],
        },
        current_chi_index,
    )
}

fn project_day_view(info: &DayInfoDto, current_chi_index: Option<usize>) -> DayViewDto {
    DayViewDto {
        schema_version: DAY_VIEW_SCHEMA_VERSION.to_string(),
        solar: info.solar.clone(),
        lunar: info.lunar.clone(),
        canchi: info.canchi.clone(),
        tiet_khi: info.tiet_khi.clone(),
        pattern: project_pattern(info),
        hours: project_hours(info, current_chi_index),
        coverage: project_coverage(info),
    }
}

fn support(title: String, reason: String, source: DayViewSignalSourceDto) -> DayViewPatternItemDto {
    DayViewPatternItemDto {
        kind: DayViewPatternItemKindDto::Support,
        title,
        reason,
        source,
    }
}

fn constraint(
    title: String,
    reason: String,
    source: DayViewSignalSourceDto,
) -> DayViewPatternItemDto {
    DayViewPatternItemDto {
        kind: DayViewPatternItemKindDto::Constraint,
        title,
        reason,
        source,
    }
}

fn unknown(title: String, reason: String) -> DayViewPatternItemDto {
    DayViewPatternItemDto {
        kind: DayViewPatternItemKindDto::Unknown,
        title,
        reason,
        source: DayViewSignalSourceDto::PersonalContext,
    }
}

fn project_pattern(info: &DayInfoDto) -> DayViewPatternDto {
    let mut supports = Vec::new();
    let mut constraints = Vec::new();

    let hoang_dao = &info.gio_hoang_dao;
    let good_chis: Vec<String> = hoang_dao
        .good_hours
        .iter()
        .map(|hour| hour.hour_chi.clone())
        .collect();
    supports.push(support(
        "Giờ Hoàng Đạo".to_string(),
        format!(
            "Ngày chi {} có {} giờ Hoàng Đạo theo truyền thống: {}.",
            hoang_dao.day_chi,
            hoang_dao.good_hour_count,
            good_chis.join(", ")
        ),
        DayViewSignalSourceDto::HoangDaoHours,
    ));

    if let Some(fortune) = &info.day_fortune {
        let truc = &fortune.truc;
        let item_title = format!("Trực {}", truc.name);
        let item_reason = format!(
            "Trực {} được phân loại '{}' theo nguồn truyền thống.",
            truc.name, truc.quality
        );
        match truc.quality.as_str() {
            "cat" => supports.push(support(
                item_title,
                item_reason,
                DayViewSignalSourceDto::Truc,
            )),
            "hung" => constraints.push(constraint(
                item_title,
                item_reason,
                DayViewSignalSourceDto::Truc,
            )),
            _ => {}
        }

        if !fortune.stars.cat_tinh.is_empty() {
            supports.push(support(
                "Cát tinh".to_string(),
                format!(
                    "Các sao được ghi nhận là cát: {}.",
                    fortune.stars.cat_tinh.join(", ")
                ),
                DayViewSignalSourceDto::CatTinh,
            ));
        }
        if !fortune.stars.sat_tinh.is_empty() {
            constraints.push(constraint(
                "Sát tinh".to_string(),
                format!(
                    "Các sao được ghi nhận là hung: {}.",
                    fortune.stars.sat_tinh.join(", ")
                ),
                DayViewSignalSourceDto::SatTinh,
            ));
        }

        if let Some(deity) = &fortune.day_deity {
            let item_reason = format!(
                "Ngày {} được xếp vào nhóm {} theo truyền thống.",
                deity.name, deity.classification
            );
            match deity.classification.as_str() {
                "hoang_dao" => supports.push(support(
                    format!("Ngày {} (Hoàng Đạo)", deity.name),
                    item_reason,
                    DayViewSignalSourceDto::DayDeity,
                )),
                "hac_dao" => constraints.push(constraint(
                    format!("Ngày {} (Hắc Đạo)", deity.name),
                    item_reason,
                    DayViewSignalSourceDto::DayDeity,
                )),
                _ => {}
            }
        }

        let conflict = &fortune.conflict;
        let mut conflict_reason = format!(
            "Chi ngày xung với chi {} theo lục xung.",
            conflict.opposing_chi
        );
        if !conflict.tuoi_xung.is_empty() {
            conflict_reason = format!(
                "{conflict_reason} Tuổi xung: {}.",
                conflict.tuoi_xung.join(", ")
            );
        }
        constraints.push(constraint(
            format!("Xung chi {}", conflict.opposing_chi),
            conflict_reason,
            DayViewSignalSourceDto::DayConflict,
        ));

        for taboo in &fortune.taboos {
            constraints.push(constraint(
                taboo.name.clone(),
                taboo.reason.clone(),
                DayViewSignalSourceDto::Taboo,
            ));
        }
    }

    let unknowns = vec![
        unknown(
            "Chưa có mục đích".to_string(),
            "Tổng quan ngày là ẩn danh; thêm mục đích để mở Đánh giá Ngày.".to_string(),
        ),
        unknown(
            "Chưa có hồ sơ sinh".to_string(),
            "Tương thích cá nhân với ngày chưa được xét.".to_string(),
        ),
        unknown(
            "Chưa có vị trí".to_string(),
            "Chứng cứ phụ thuộc vị trí chưa khả dụng.".to_string(),
        ),
    ];

    DayViewPatternDto {
        supports,
        constraints,
        unknowns,
    }
}

fn project_hours(info: &DayInfoDto, current_chi_index: Option<usize>) -> DayViewHourTimelineDto {
    let hours: Vec<DayViewHourDto> = info
        .gio_hoang_dao
        .all_hours
        .iter()
        .map(|hour| {
            let notable_reason = if hour.is_good {
                Some(format!("Giờ {} (Hoàng Đạo).", hour.star))
            } else {
                None
            };
            DayViewHourDto {
                hour_index: hour.hour_index,
                chi: hour.hour_chi.clone(),
                time_range: hour.time_range.clone(),
                star: hour.star.clone(),
                is_hoang_dao: hour.is_good,
                is_notable: hour.is_good,
                notable_reason,
                is_current: current_chi_index == Some(hour.hour_index),
            }
        })
        .collect();
    let notable_count = hours.iter().filter(|hour| hour.is_notable).count();
    DayViewHourTimelineDto {
        hours,
        current_hour_index: current_chi_index,
        notable_count,
    }
}

fn project_coverage(info: &DayInfoDto) -> DayViewCoverageDto {
    let mut entries = Vec::new();
    let mut push = |family: DayViewCoverageFamilyDto, present: bool| {
        entries.push(DayViewCoverageEntryDto {
            family,
            state: if present {
                DayViewCoverageStateDto::Present
            } else {
                DayViewCoverageStateDto::Unknown
            },
        });
    };

    push(DayViewCoverageFamilyDto::Canchi, true);
    push(DayViewCoverageFamilyDto::TietKhi, true);
    if let Some(fortune) = &info.day_fortune {
        push(DayViewCoverageFamilyDto::Truc, true);
        push(
            DayViewCoverageFamilyDto::Stars,
            !fortune.stars.cat_tinh.is_empty() || !fortune.stars.sat_tinh.is_empty(),
        );
        push(
            DayViewCoverageFamilyDto::DayDeity,
            fortune.day_deity.is_some(),
        );
        push(DayViewCoverageFamilyDto::Taboos, !fortune.taboos.is_empty());
        push(DayViewCoverageFamilyDto::DayConflict, true);
        push(DayViewCoverageFamilyDto::DayElement, true);
    }
    push(
        DayViewCoverageFamilyDto::HoangDaoHours,
        !info.gio_hoang_dao.all_hours.is_empty(),
    );
    push(
        DayViewCoverageFamilyDto::Recommendations,
        !info.daily_recommendations.activities.is_empty(),
    );
    push(DayViewCoverageFamilyDto::Intent, false);
    push(DayViewCoverageFamilyDto::BirthProfile, false);
    push(DayViewCoverageFamilyDto::Location, false);

    let present_count = entries
        .iter()
        .filter(|entry| entry.state == DayViewCoverageStateDto::Present)
        .count();
    let unknown_count = entries
        .iter()
        .filter(|entry| entry.state == DayViewCoverageStateDto::Unknown)
        .count();
    let pending_count = entries
        .iter()
        .filter(|entry| entry.state == DayViewCoverageStateDto::Pending)
        .count();

    DayViewCoverageDto {
        entries,
        present_count,
        unknown_count,
        pending_count,
    }
}
