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
//!
//! v1.2 (`amlich-b14l.9`, S3 — Assessment on demand) adds, additively: the
//! optional Personal Context (Intent [+ Birth Profile][+ Location]) and the
//! Day Assessment projection (Result + Reasons + Assessment Confidence,
//! missing inputs as Unknowns, per-layer enrichment diff). The projection
//! labels the canonical `PersonalDayAssessment` — it never re-evaluates and
//! never invents claims. The anonymous view is unchanged: `assessment` is
//! absent until an Intent is supplied (Progressive Enrichment,
//! `CONTEXT.md`).

use serde::{Deserialize, Serialize};

use crate::dto::{CanChiInfoDto, DateQuery, DayInfoDto, LunarDto, SolarDto, TietKhiDto};
use crate::get_day_info;

pub const DAY_VIEW_SCHEMA_VERSION: &str = "day-view-v1.2";
pub const DAY_VIEW_MONTH_SCHEMA_VERSION: &str = "day-view-month-v1";

/// Intent choices shared by both replacement surfaces. Keys are the
/// canonical `ConsultationIntent::event_kind` strings and are part of the
/// cross-surface contract; labels are Vietnamese and render everywhere
/// through [`DAY_VIEW_INTENT_LABELS`].
pub const DAY_VIEW_INTENTS: [(&str, &str); 9] = [
    ("wedding", "Cưới hỏi"),
    ("moving_house", "Chuyển nhà"),
    ("opening_business", "Khai trương"),
    ("contract_signing", "Ký kết hợp đồng"),
    ("travel", "Xuất hành"),
    ("burial", "An táng"),
    ("renovation", "Tu tạo"),
    ("medical_checkup", "Khám bệnh"),
    ("prayer", "Cầu nguyện"),
];

pub fn day_view_intent_label(event_kind: &str) -> String {
    DAY_VIEW_INTENTS
        .iter()
        .find(|(key, _)| *key == event_kind)
        .map(|(_, label)| *label)
        .unwrap_or(event_kind)
        .to_string()
}

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

/// Personal Context input for Progressive Enrichment (S3). Layers apply in
/// the fixed order Intent [+ Birth Profile][+ Location]; an Intent is
/// required before a Day Assessment exists. Every unknown string is
/// rejected loudly so a typo cannot silently downgrade a layer.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DayViewPersonalContextInput {
    /// Intent key — one of [`DAY_VIEW_INTENTS`] (`event_kind` strings).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub birth_year: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub birth_month: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub birth_day: Option<i32>,
    /// Birth hour and minute must be supplied together (e.g. `09:30`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub birth_hour: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub birth_minute: Option<u8>,
    /// Gender label (`nam`/`male` or `nữ`/`nu`/`female`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// Location layer — a free-form location name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
}

impl DayViewPersonalContextInput {
    /// Whether the Birth Profile layer is supplied (all three date parts).
    pub fn has_birth_profile(&self) -> bool {
        self.birth_year.is_some() && self.birth_month.is_some() && self.birth_day.is_some()
    }

    /// Whether the Location layer is supplied.
    pub fn has_location(&self) -> bool {
        self.location_name.is_some()
    }
}

/// Day Assessment Result — the policy verdict for the date under the
/// supplied Intent. Labels transport the shared Vietnamese rendering so
/// both surfaces show identical wording.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayViewAssessmentResultDto {
    /// `favorable` | `mixed` | `cautious` | `avoid`.
    pub bucket: String,
    pub bucket_label: String,
    pub semantic: String,
    pub primary_conclusion: String,
}

/// One assessment Reason, projected from a canonical decision
/// contribution: axis, polarity, and full source attribution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayViewAssessmentReasonDto {
    pub axis: String,
    pub axis_label: String,
    /// `favorable` | `avoid` | `neutral` | `info`.
    pub polarity: String,
    pub polarity_label: String,
    /// Full reason sentence, including provenance.
    pub text: String,
    pub source_family: String,
    pub source_id: String,
    pub method: String,
}

/// Assessment Confidence — input/evidence completeness for this
/// assessment, never predictive certainty (`CONTEXT.md`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayViewAssessmentConfidenceDto {
    /// `low` | `medium` | `high`.
    pub level: String,
    pub level_label: String,
    pub reasons: Vec<String>,
}

/// One Progressive Enrichment layer and the changes it made explicit.
/// All three layers are always listed, in the fixed order
/// intent → birth_profile → location, so a surface can render the whole
/// ladder; skipped layers carry `applied: false`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayViewEnrichmentLayerDto {
    /// `intent` | `birth_profile` | `location`.
    pub layer: String,
    pub layer_label: String,
    pub applied: bool,
    /// One-line summary of the layer state (e.g. the intent label or the
    /// birth date), rendered directly by both surfaces.
    pub summary: String,
    /// Explicit change statements unlocked by this layer.
    pub changes: Vec<String>,
}

/// Day Assessment projection (S3). Pure labeling of the canonical
/// `amlich_core::assessment::PersonalDayAssessment` — grouping and
/// translation only, no recomputation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayViewAssessmentDto {
    /// Intent key (canonical `event_kind`).
    pub intent: String,
    pub intent_label: String,
    /// Capability tier of the assessed profile: `anonymous` | `date` |
    /// `datetime`.
    pub profile_tier: String,
    pub result: DayViewAssessmentResultDto,
    pub reasons: Vec<DayViewAssessmentReasonDto>,
    pub confidence: DayViewAssessmentConfidenceDto,
    /// Personal inputs still missing; mirrors the Day Pattern unknowns.
    pub unknowns: Vec<DayViewPatternItemDto>,
    /// The enrichment ladder with per-layer diffs.
    pub layers: Vec<DayViewEnrichmentLayerDto>,
    /// Policy provenance (S4 will follow Reasons into full evidence).
    pub ruleset_id: String,
    pub ruleset_version: String,
    pub policy_id: String,
    pub policy_version: String,
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
    /// S3 Day Assessment (`amlich-b14l.9`). `None` while the Personal
    /// Context carries no Intent — the anonymous view stays non-verdict.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assessment: Option<DayViewAssessmentDto>,
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
        &personal_unknowns(false, false, false),
        (false, false, false),
        None,
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

/// S3 (`amlich-b14l.9`) enriched Day View: the same anonymous projection
/// plus the Day Assessment unlocked by the Personal Context. Progressive
/// Enrichment order is fixed — Intent [+ Birth Profile][+ Location]; a
/// context without an Intent yields the unchanged anonymous view (no
/// assessment, no layer claims). The anonymous sections (identity, Day
/// Pattern supports/constraints, Hour Timeline) are computed from the same
/// `get_day_info` output as the anonymous path, so calendar facts cannot
/// shift when a layer is added.
pub fn get_day_view_enriched_for_date(
    day: i32,
    month: i32,
    year: i32,
    current_chi_index: Option<usize>,
    selected_chi_index: Option<usize>,
    context: &DayViewPersonalContextInput,
) -> Result<DayViewDto, String> {
    let intent = match &context.intent {
        Some(key) => Some(parse_day_view_intent(key)?),
        None => None,
    };
    let birth = if context.has_birth_profile() {
        Some(validate_birth_profile(context)?)
    } else if context.birth_year.is_some()
        || context.birth_month.is_some()
        || context.birth_day.is_some()
    {
        return Err("birth_year, birth_month, and birth_day must be supplied together".to_string());
    } else {
        None
    };
    let gender = match &context.gender {
        Some(label) => Some(parse_day_view_gender(label)?),
        None => None,
    };

    let query = DateQuery {
        day,
        month,
        year,
        timezone: None,
        ruleset_id: None,
        event_kind: None,
        enabled_pack_ids: vec![],
    };
    let info = get_day_info(&query)?;

    let Some(intent) = intent else {
        // No Intent → no assessment (Progressive Enrichment: the Intent is
        // required). The view is byte-for-byte the anonymous projection.
        return Ok(project_day_view(
            &info,
            current_chi_index,
            selected_chi_index,
            &personal_unknowns(false, context.has_birth_profile(), context.has_location()),
            (false, context.has_birth_profile(), context.has_location()),
            None,
        ));
    };

    // Canonical assessment over the same seams the personal-day surfaces
    // use — projection only, never re-evaluation.
    let snapshot = crate::build_personal_day_snapshot(&query, intent)?;
    let timezone = query.timezone.unwrap_or(amlich_core::VIETNAM_TIMEZONE);
    let (birth_day, birth_month, birth_year) = match birth {
        Some((d, m, y)) => (d, m, y),
        None => (day, month, year),
    };
    let birth_time = match (context.birth_hour, context.birth_minute) {
        (Some(hour), Some(minute)) => Some(amlich_core::BirthTime::new(hour, minute)?),
        (None, None) => None,
        _ => return Err("birth_hour and birth_minute must be supplied together".to_string()),
    };
    let profile = amlich_core::BirthProfile {
        day: birth_day,
        month: birth_month,
        year: birth_year,
        time: birth_time,
        timezone,
        longitude: None,
        use_solar_time: false,
        gender,
        location_name: context.location_name.clone(),
    };
    let kua = match (profile.year, profile.gender) {
        (by, Some(g)) => Some(amlich_core::almanac::tu_menh::compute_kua(by, g)),
        _ => None,
    };
    let assessment = amlich_core::assessment::PersonalDayAssessment::assess_with_kua(
        snapshot.clone(),
        profile,
        intent,
        kua.as_ref(),
    );

    let unknowns = personal_unknowns(true, context.has_birth_profile(), context.has_location());
    let assessment = project_assessment(&assessment, context, intent.event_kind(), &unknowns);
    Ok(project_day_view(
        &info,
        current_chi_index,
        selected_chi_index,
        &unknowns,
        (true, context.has_birth_profile(), context.has_location()),
        Some(assessment),
    ))
}

fn parse_day_view_intent(key: &str) -> Result<amlich_core::ConsultationIntent, String> {
    use amlich_core::ConsultationIntent as Intent;
    let intent = match key {
        "wedding" => Intent::Wedding,
        "moving_house" => Intent::MovingHouse,
        "opening_business" => Intent::OpeningBusiness,
        "contract_signing" => Intent::ContractSigning,
        "travel" => Intent::Travel,
        "burial" => Intent::Burial,
        "renovation" => Intent::Renovation,
        "medical_checkup" => Intent::Medical,
        "prayer" => Intent::Prayer,
        other => {
            return Err(format!(
                "unknown intent `{other}`; expected one of {}",
                DAY_VIEW_INTENTS
                    .iter()
                    .map(|(key, _)| *key)
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
        }
    };
    Ok(intent)
}

fn parse_day_view_gender(label: &str) -> Result<amlich_core::almanac::tu_menh::Gender, String> {
    crate::parse_gender(label)
        .ok_or_else(|| format!("unknown gender `{label}`; expected nam/male or nữ/nu/female"))
}

/// Validate the birth date parts of the context; returns
/// `(day, month, year)` already range-checked.
fn validate_birth_profile(
    context: &DayViewPersonalContextInput,
) -> Result<(i32, i32, i32), String> {
    if !(1..=12).contains(&context.birth_month.unwrap()) {
        return Err(format!(
            "birth_month must be 1-12; got {}",
            context.birth_month.unwrap()
        ));
    }
    if !(1..=31).contains(&context.birth_day.unwrap()) {
        return Err(format!(
            "birth_day must be 1-31; got {}",
            context.birth_day.unwrap()
        ));
    }
    Ok((
        context.birth_day.unwrap(),
        context.birth_month.unwrap(),
        context.birth_year.unwrap(),
    ))
}

fn personal_unknowns(
    has_intent: bool,
    has_birth: bool,
    has_location: bool,
) -> Vec<DayViewPatternItemDto> {
    let mut unknowns = Vec::new();
    if !has_intent {
        unknowns.push(unknown(
            "Chưa có mục đích".to_string(),
            "Tổng quan ngày là ẩn danh; thêm mục đích để mở Đánh giá Ngày.".to_string(),
        ));
    }
    if !has_birth {
        unknowns.push(unknown(
            "Chưa có hồ sơ sinh".to_string(),
            "Tương thích cá nhân với ngày chưa được xét.".to_string(),
        ));
    }
    if !has_location {
        unknowns.push(unknown(
            "Chưa có vị trí".to_string(),
            "Chứng cứ phụ thuộc vị trí chưa khả dụng.".to_string(),
        ));
    }
    unknowns
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

fn project_day_view(
    info: &DayInfoDto,
    current_chi_index: Option<usize>,
    selected_chi_index: Option<usize>,
    personal_unknowns: &[DayViewPatternItemDto],
    personal_coverage: (bool, bool, bool),
    assessment: Option<DayViewAssessmentDto>,
) -> DayViewDto {
    DayViewDto {
        schema_version: DAY_VIEW_SCHEMA_VERSION.to_string(),
        solar: info.solar.clone(),
        lunar: info.lunar.clone(),
        canchi: info.canchi.clone(),
        tiet_khi: info.tiet_khi.clone(),
        pattern: project_pattern(info, personal_unknowns),
        hours: project_hours(info, current_chi_index, selected_chi_index),
        coverage: project_coverage(info, personal_coverage),
        assessment,
    }
}

fn project_pattern(
    info: &DayInfoDto,
    personal_unknowns: &[DayViewPatternItemDto],
) -> DayViewPatternDto {
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

    DayViewPatternDto {
        supports,
        constraints,
        unknowns: personal_unknowns.to_vec(),
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

fn project_coverage(info: &DayInfoDto, personal: (bool, bool, bool)) -> DayViewCoverageDto {
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
    push(DayViewCoverageFamilyDto::Intent, personal.0);
    push(DayViewCoverageFamilyDto::BirthProfile, personal.1);
    push(DayViewCoverageFamilyDto::Location, personal.2);

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

// -----------------------------------------------------------------------
// S3 (`amlich-b14l.9`): Day Assessment projection — pure labeling of the
// canonical PersonalDayAssessment (Result + Reasons + Assessment
// Confidence + Unknowns + per-layer enrichment diff).
// -----------------------------------------------------------------------

fn day_view_bucket_label(bucket: &str) -> String {
    match bucket {
        "favorable" => "Thuận lợi",
        "mixed" => "Pha trộn",
        "cautious" => "Thận trọng",
        "avoid" => "Nên tránh",
        other => other,
    }
    .to_string()
}

fn day_view_confidence_label(level: &str) -> String {
    match level {
        "high" => "Cao",
        "medium" => "Trung bình",
        "low" => "Thấp",
        other => other,
    }
    .to_string()
}

fn day_view_axis_label(axis: &str) -> String {
    match axis {
        "generic_day_quality" => "Chất lượng ngày chung",
        "intent_fit" => "Phù hợp mục đích",
        "personal_alignment" => "Tương thích cá nhân",
        "annual_pressure" => "Áp lực theo năm",
        "evidence_coverage" => "Phủ chứng cứ",
        other => other,
    }
    .to_string()
}

fn day_view_polarity_label(polarity: &str) -> String {
    match polarity {
        "favorable" => "thuận",
        "avoid" => "tránh",
        "neutral" => "trung tính",
        "info" => "thông tin",
        other => other,
    }
    .to_string()
}

fn day_view_tier_key(tier: &amlich_core::birth::BirthDataTier) -> String {
    match tier {
        amlich_core::birth::BirthDataTier::Anonymous => "anonymous".to_string(),
        amlich_core::birth::BirthDataTier::Date => "date".to_string(),
        amlich_core::birth::BirthDataTier::Datetime => "datetime".to_string(),
    }
}

fn day_view_tier_label(tier: &str) -> String {
    match tier {
        "anonymous" => "ẩn danh (chỉ mục đích)",
        "date" => "ngày sinh (không giờ sinh)",
        "datetime" => "ngày và giờ sinh",
        other => other,
    }
    .to_string()
}

fn gender_label_of(context: &DayViewPersonalContextInput) -> Option<&'static str> {
    let label = context.gender.as_deref()?.to_lowercase();
    match label.as_str() {
        "nam" | "male" => Some("Nam"),
        "nữ" | "nu" | "female" => Some("Nữ"),
        _ => Some("Nam"),
    }
}

/// Axis state sentence shared by the layer diffs: availability first,
/// verdict second — never a score.
fn axis_state_line(axis: &amlich_core::assessment::AxisOutcome) -> String {
    let label = day_view_axis_label(axis.axis.as_str());
    match &axis.unavailable_reason {
        Some(reason) => format!("Trục {label}: chưa khả dụng — {reason}."),
        None => format!("Trục {label}: {}.", axis.verdict),
    }
}

fn project_assessment(
    assessment: &amlich_core::assessment::PersonalDayAssessment,
    context: &DayViewPersonalContextInput,
    event_kind: &str,
    unknowns: &[DayViewPatternItemDto],
) -> DayViewAssessmentDto {
    use amlich_core::assessment::AssessmentAxis;

    // Result — the policy decision, labeled not recomputed. The intent
    // token inside the canonical conclusion is relabeled to the shared
    // Vietnamese intent label (exact-token substitution only — the
    // claim itself stays the policy's).
    let bucket = assessment.decision.bucket.as_str().to_string();
    let result = DayViewAssessmentResultDto {
        bucket_label: day_view_bucket_label(&bucket).to_string(),
        bucket,
        semantic: assessment.decision.semantic.clone(),
        primary_conclusion: assessment
            .decision
            .primary_conclusion
            .replace(event_kind, &day_view_intent_label(event_kind)),
    };

    // Reasons — one per decision contribution with full provenance.
    let reasons: Vec<DayViewAssessmentReasonDto> = assessment
        .contributions
        .iter()
        .map(|contribution| {
            let axis = contribution.axis.as_str().to_string();
            let polarity = contribution.polarity.as_str().to_string();
            let mut text = format!(
                "{}: đóng góp {} (độ mạnh {:.2}) — nguồn {}/{}, phương pháp {}.",
                day_view_axis_label(&axis),
                day_view_polarity_label(&polarity),
                contribution.strength,
                contribution.source_evidence.source_family,
                contribution.source_evidence.source_id,
                contribution.source_evidence.method,
            );
            if let Some(note) = &contribution.note {
                text.push_str(format!(" Ghi chú: {note}.").as_str());
            }
            DayViewAssessmentReasonDto {
                axis_label: day_view_axis_label(&axis).to_string(),
                axis,
                polarity_label: day_view_polarity_label(&polarity).to_string(),
                polarity,
                text,
                source_family: contribution.source_evidence.source_family.clone(),
                source_id: contribution.source_evidence.source_id.clone(),
                method: contribution.source_evidence.method.clone(),
            }
        })
        .collect();

    // Assessment Confidence — input and evidence completeness reasons.
    let level = format!("{:?}", assessment.decision.confidence).to_lowercase();
    let mut confidence_reasons = vec![format!(
        "Cấp dữ liệu hồ sơ: {}.",
        day_view_tier_label(&day_view_tier_key(&assessment.capability_tier))
    )];
    if assessment.unavailable_sections.is_empty() {
        confidence_reasons.push(
            "Mọi phần đánh giá đều có đủ dữ liệu đầu vào theo chính sách hiện hành.".to_string(),
        );
    }
    for section in &assessment.unavailable_sections {
        confidence_reasons.push(format!(
            "Thiếu `{}`: {} (cần: {}).",
            section.section,
            section.reason,
            section.required_fields.join(", ")
        ));
    }
    let confidence = DayViewAssessmentConfidenceDto {
        level_label: day_view_confidence_label(&level).to_string(),
        level,
        reasons: confidence_reasons,
    };

    // Layers — the fixed Progressive Enrichment ladder with explicit
    // per-layer diffs, so adding context never silently rewrites the day.
    let mut layers = Vec::new();

    let intent_contributions = assessment
        .contributions
        .iter()
        .filter(|contribution| contribution.axis == AssessmentAxis::IntentFit)
        .count();
    let mut intent_changes = vec![format!(
        "Đánh giá Ngày mở theo mục đích {}; tổng quan ẩn danh và các tín hiệu lịch không đổi.",
        day_view_intent_label(event_kind)
    )];
    intent_changes.push(axis_state_line(&assessment.axes.intent_fit));
    intent_changes.push(format!(
        "{intent_contributions} đóng góp quyết định đến từ trục Phù hợp mục đích."
    ));
    layers.push(DayViewEnrichmentLayerDto {
        layer: "intent".to_string(),
        layer_label: "Mục đích".to_string(),
        applied: true,
        summary: format!("Mục đích: {}.", day_view_intent_label(event_kind)),
        changes: intent_changes,
    });

    let birth_applied = context.has_birth_profile();
    let gender_label = match gender_label_of(context) {
        Some("Nữ") => " · Nữ",
        Some(_) => " · Nam",
        None => "",
    };
    let birth_summary = if birth_applied {
        let date = format!(
            "{:02}/{:02}/{}",
            context.birth_day.unwrap(),
            context.birth_month.unwrap(),
            context.birth_year.unwrap()
        );
        let time = match (context.birth_hour, context.birth_minute) {
            (Some(hour), Some(minute)) => format!(" {hour:02}:{minute:02}"),
            _ => String::new(),
        };
        format!("Hồ sơ sinh: {date}{time}{gender_label}.")
    } else {
        "Hồ sơ sinh: chưa cung cấp.".to_string()
    };
    let mut birth_changes = Vec::new();
    if birth_applied {
        birth_changes.push(format!(
            "Cấp dữ liệu hồ sơ: {}.",
            day_view_tier_label(&day_view_tier_key(&assessment.capability_tier))
        ));
        birth_changes.push(axis_state_line(&assessment.axes.personal_alignment));
        birth_changes.push(axis_state_line(&assessment.axes.annual_pressure));
        birth_changes.push(if assessment.evidence.has_kua {
            "Bối cảnh Kua được đưa vào đánh giá.".to_string()
        } else {
            "Bối cảnh Kua chưa khả dụng (cần năm sinh và giới tính).".to_string()
        });
    } else {
        birth_changes.push(
            "Chưa cung cấp hồ sơ sinh — trục Tương thích cá nhân và các phân tích cá nhân giữ trạng thái Chưa biết."

                .to_string(),
        );
    }
    layers.push(DayViewEnrichmentLayerDto {
        layer: "birth_profile".to_string(),
        layer_label: "Hồ sơ sinh".to_string(),
        applied: birth_applied,
        summary: birth_summary,
        changes: birth_changes,
    });

    let location_applied = context.has_location();
    let location_changes = if location_applied {
        vec!["Ghi nhận vị trí vào hồ sơ; vị trí được tính vào mức phủ chứng cứ.".to_string()]
    } else {
        vec![
            "Chưa cung cấp vị trí — chứng cứ phụ thuộc vị trí giữ trạng thái Chưa biết."
                .to_string(),
        ]
    };
    layers.push(DayViewEnrichmentLayerDto {
        layer: "location".to_string(),
        layer_label: "Vị trí".to_string(),
        applied: location_applied,
        summary: match &context.location_name {
            Some(name) => format!("Vị trí: {name}."),
            None => "Vị trí: chưa cung cấp.".to_string(),
        },
        changes: location_changes,
    });

    DayViewAssessmentDto {
        intent: event_kind.to_string(),
        intent_label: day_view_intent_label(event_kind).to_string(),
        profile_tier: day_view_tier_key(&assessment.capability_tier),
        result,
        reasons,
        confidence,
        unknowns: unknowns.to_vec(),
        layers,
        ruleset_id: assessment.ruleset_id.clone(),
        ruleset_version: assessment.ruleset_version.clone(),
        policy_id: assessment.policy_id.clone(),
        policy_version: assessment.policy_version.clone(),
    }
}
