//! Day View Model — the shared surface projection behind the replacement
//! desktop and TUI (`amlich-b14l.7`, `docs/surface-replacement-slices.md` §3).
//!
//! Pure projection over `get_day_info` outputs: grouping, labeling, and
//! navigation only. No assessment math, no re-evaluation, no new claims.
//!
//! v1.1 (`amlich-b14l.8`, S2 — Navigate & Hours) adds, additively: the
//! selected-hour detail (Hoàng Đạo classification, ruling star, hour-context
//! reasons; never a ranking or score) and the month-grid navigation
//! projection with leap-month labels. v1 fields are unchanged.

use serde::{Deserialize, Serialize};

use crate::dto::{CanChiInfoDto, DateQuery, DayInfoDto, LunarDto, SolarDto, TietKhiDto};
use crate::get_day_info;

pub const DAY_VIEW_SCHEMA_VERSION: &str = "day-view-v1.1";
pub const DAY_VIEW_MONTH_SCHEMA_VERSION: &str = "day-view-month-v1";

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
    /// S2 selected hour (`amlich-b14l.8`). The Selected Hour updates
    /// time-dependent context without moving the date anchor; it never
    /// requests an assessment and never ranks the window.
    pub selected_hour_index: Option<usize>,
    /// Drill-down detail for the selected hour. `None` until a surface
    /// selects a window; carries no score, rank, or verdict.
    pub detail: Option<DayViewHourDetailDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayViewHourDetailDto {
    pub hour_index: usize,
    pub chi: String,
    pub time_range: String,
    /// Ruling star of the window (Thập Nhị Kiến Trừ star transported
    /// from `gio_hoang_dao`).
    pub star: String,
    /// Hoàng Đạo / Hắc Đạo classification label for the window.
    pub classification: String,
    pub is_hoang_dao: bool,
    pub is_notable: bool,
    pub notable_reason: Option<String>,
    pub is_current: bool,
    /// Hour-context reasons (S2 evidence depth): why this window is
    /// classified as it is, in the context of the selected date.
    pub reasons: Vec<String>,
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

/// One solar day cell of the month-grid navigation projection (S2).
/// Pure labels over `get_day_info`; no pattern, no hours, no verdicts —
/// the grid navigates, the Day View explains.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayViewMonthCellDto {
    pub day: i32,
    pub lunar_day: i32,
    pub lunar_month: i32,
    /// Leap-month marker stays visible while browsing (`docs/
    /// surface-replacement-cutover.md` §2 W2).
    pub is_leap_lunar_month: bool,
    /// e.g. `10/6` or `10/6 (nhuận)`.
    pub lunar_label: String,
    /// Full Can Chi of the day pillar.
    pub can_chi_day: String,
    pub is_today: bool,
    pub is_selected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayViewMonthDto {
    pub schema_version: String,
    pub year: i32,
    pub month: i32,
    /// Weekday index (Sunday = 0) of day 1, for grid layout.
    pub first_weekday: usize,
    pub cells: Vec<DayViewMonthCellDto>,
}

pub fn current_chi_index_from_hour(hour: u32) -> usize {
    (((hour + 1) % 24) / 2) as usize
}

pub fn get_day_view(
    query: &DateQuery,
    current_chi_index: Option<usize>,
    selected_chi_index: Option<usize>,
) -> Result<DayViewDto, String> {
    let info = get_day_info(query)?;
    Ok(project_day_view(
        &info,
        current_chi_index,
        selected_chi_index,
    ))
}

pub fn get_day_view_for_date(
    day: i32,
    month: i32,
    year: i32,
    current_chi_index: Option<usize>,
    selected_chi_index: Option<usize>,
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
        selected_chi_index,
    )
}

/// Month-grid navigation projection (S2). `today` and `selected` are the
/// surface's own navigation state (solar `day/month/year`); the projection
/// only labels, never derives, them.
pub fn get_day_view_month(
    month: i32,
    year: i32,
    today: Option<(i32, i32, i32)>,
    selected: Option<(i32, i32, i32)>,
) -> Result<DayViewMonthDto, String> {
    if !(1..=12).contains(&month) {
        return Err(format!("month must be 1-12; got {month}"));
    }
    let mut first_weekday = 0;
    let mut cells = Vec::new();
    for day in 1..=31 {
        let info = match crate::get_day_info_for_date(day, month, year) {
            Ok(info) => info,
            Err(_) => break,
        };
        if day == 1 {
            first_weekday = info.solar.day_of_week;
        }
        let leap = info.lunar.is_leap_month;
        cells.push(DayViewMonthCellDto {
            day,
            lunar_day: info.lunar.day,
            lunar_month: info.lunar.month,
            is_leap_lunar_month: leap,
            lunar_label: if leap {
                format!("{}/{} (nhuận)", info.lunar.day, info.lunar.month)
            } else {
                format!("{}/{}", info.lunar.day, info.lunar.month)
            },
            can_chi_day: info.canchi.day.full.clone(),
            is_today: today == Some((day, month, year)),
            is_selected: selected == Some((day, month, year)),
        });
    }
    if cells.is_empty() {
        return Err(format!("month {month}/{year} has no days"));
    }
    Ok(DayViewMonthDto {
        schema_version: DAY_VIEW_MONTH_SCHEMA_VERSION.to_string(),
        year,
        month,
        first_weekday,
        cells,
    })
}

fn project_day_view(
    info: &DayInfoDto,
    current_chi_index: Option<usize>,
    selected_chi_index: Option<usize>,
) -> DayViewDto {
    DayViewDto {
        schema_version: DAY_VIEW_SCHEMA_VERSION.to_string(),
        solar: info.solar.clone(),
        lunar: info.lunar.clone(),
        canchi: info.canchi.clone(),
        tiet_khi: info.tiet_khi.clone(),
        pattern: project_pattern(info),
        hours: project_hours(info, current_chi_index, selected_chi_index),
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

fn project_hours(
    info: &DayInfoDto,
    current_chi_index: Option<usize>,
    selected_chi_index: Option<usize>,
) -> DayViewHourTimelineDto {
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
    let detail = selected_chi_index.and_then(|index| {
        hours
            .iter()
            .find(|hour| hour.hour_index == index)
            .map(|hour| project_hour_detail(info, hour))
    });
    DayViewHourTimelineDto {
        hours,
        current_hour_index: current_chi_index,
        notable_count,
        selected_hour_index: selected_chi_index,
        detail,
    }
}

/// Hour detail for the selected window: classification, ruling star, and
/// hour-context reasons. Labeling of `gio_hoang_dao` outputs only — no
/// score, no rank, no suitability claim.
fn project_hour_detail(info: &DayInfoDto, hour: &DayViewHourDto) -> DayViewHourDetailDto {
    let classification = if hour.is_hoang_dao {
        "Hoàng Đạo"
    } else {
        "Hắc Đạo"
    };
    let mut reasons = vec![format!(
        "Ngày chi {}: giờ {} ({}) thuộc nhóm {} — sao {} cai trị cửa sổ này theo Thập Nhị Kiến Trừ.",
        info.gio_hoang_dao.day_chi, hour.chi, hour.time_range, classification, hour.star
    )];
    if let Some(notable_reason) = &hour.notable_reason {
        reasons.push(notable_reason.clone());
    }
    if hour.is_current {
        reasons.push("Đây là cửa sổ giờ đang diễn ra tại thời điểm hiện tại.".to_string());
    }
    reasons.push(
        "Chọn giờ chỉ cập nhật bối cảnh phụ thuộc thời gian; ngày đang xem không đổi.".to_string(),
    );
    DayViewHourDetailDto {
        hour_index: hour.hour_index,
        chi: hour.chi.clone(),
        time_range: hour.time_range.clone(),
        star: hour.star.clone(),
        classification: classification.to_string(),
        is_hoang_dao: hour.is_hoang_dao,
        is_notable: hour.is_notable,
        notable_reason: hour.notable_reason.clone(),
        is_current: hour.is_current,
        reasons,
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
