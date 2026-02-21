pub mod waybar;

use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use amlich_api::{get_day_info_for_date, DayInfoDto};
use chrono::{Datelike, Local, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum DisplayMode {
    Full,
    Lunar,
    CanChi,
    Minimal,
}

impl DisplayMode {
    pub fn from_str_value(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "full" => Some(DisplayMode::Full),
            "lunar" => Some(DisplayMode::Lunar),
            "canchi" => Some(DisplayMode::CanChi),
            "minimal" => Some(DisplayMode::Minimal),
            _ => None,
        }
    }

    pub fn next(self) -> Self {
        match self {
            DisplayMode::Full => DisplayMode::Lunar,
            DisplayMode::Lunar => DisplayMode::CanChi,
            DisplayMode::CanChi => DisplayMode::Minimal,
            DisplayMode::Minimal => DisplayMode::Full,
        }
    }
}

impl FromStr for DisplayMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_value(s).ok_or_else(|| {
            format!("invalid mode '{s}'; valid modes are: full, lunar, canchi, minimal")
        })
    }
}

impl fmt::Display for DisplayMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            DisplayMode::Full => "full",
            DisplayMode::Lunar => "lunar",
            DisplayMode::CanChi => "canchi",
            DisplayMode::Minimal => "minimal",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryFormat {
    DayInfoJson,
    Waybar,
    Text,
}

pub struct QueryResult {
    pub output: String,
    pub warning: Option<String>,
}

pub fn parse_date(date_str: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|_| format!("invalid date '{date_str}', expected format YYYY-MM-DD"))
}

pub fn read_mode() -> DisplayMode {
    match fs::read_to_string(get_mode_file()) {
        Ok(content) => DisplayMode::from_str_value(content.trim()).unwrap_or(DisplayMode::Full),
        Err(_) => DisplayMode::Full,
    }
}

pub fn set_mode(mode: DisplayMode) -> Result<(), String> {
    ensure_state_dir().map_err(|e| format!("failed to create state directory: {e}"))?;
    fs::write(get_mode_file(), mode.to_string())
        .map_err(|e| format!("failed to save mode to state file: {e}"))
}

pub fn toggle_mode() -> Result<DisplayMode, String> {
    let next = read_mode().next();
    set_mode(next)?;
    Ok(next)
}

pub fn query(
    date: Option<NaiveDate>,
    format: QueryFormat,
    mode: Option<DisplayMode>,
    pretty: bool,
) -> Result<QueryResult, String> {
    let date = date.unwrap_or_else(|| Local::now().date_naive());
    let info = get_day_info(date)?;

    let result = match format {
        QueryFormat::DayInfoJson => {
            let output = if pretty {
                serde_json::to_string_pretty(&info)
                    .map_err(|e| format!("failed to render json: {e}"))?
            } else {
                serde_json::to_string(&info).map_err(|e| format!("failed to render json: {e}"))?
            };

            QueryResult {
                output,
                warning: mode.map(|_| "--mode is ignored when --format dayinfo-json".to_string()),
            }
        }
        QueryFormat::Waybar => {
            let effective_mode = mode.unwrap_or_else(read_mode);
            let payload = waybar::build_waybar_payload(&info, &effective_mode);
            let output = if pretty {
                serde_json::to_string_pretty(&payload)
                    .map_err(|e| format!("failed to render waybar json: {e}"))?
            } else {
                payload.to_string()
            };
            QueryResult {
                output,
                warning: None,
            }
        }
        QueryFormat::Text => {
            let effective_mode = mode.unwrap_or_else(read_mode);
            QueryResult {
                output: format_text(&info, effective_mode),
                warning: None,
            }
        }
    };

    Ok(result)
}

fn format_text(info: &DayInfoDto, mode: DisplayMode) -> String {
    let lunar_suffix = if info.lunar.is_leap_month {
        " leap"
    } else {
        ""
    };
    let canchi = match mode {
        DisplayMode::Full => format!(
            "{}/{}/{}",
            info.canchi.day.full, info.canchi.month.full, info.canchi.year.full
        ),
        DisplayMode::Lunar => "-".to_string(),
        DisplayMode::CanChi => info.canchi.day.full.clone(),
        DisplayMode::Minimal => "-".to_string(),
    };

    format!(
        "{} | lunar {}{} | canchi {} | tiet-khi {}",
        info.solar.date_string, info.lunar.date_string, lunar_suffix, canchi, info.tiet_khi.name
    )
}

fn get_day_info(date: NaiveDate) -> Result<DayInfoDto, String> {
    get_day_info_for_date(date.day() as i32, date.month() as i32, date.year())
}

fn get_state_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".local/state/amlich")
}

fn get_mode_file() -> PathBuf {
    get_state_dir().join("mode")
}

fn ensure_state_dir() -> std::io::Result<()> {
    let state_dir = get_state_dir();
    if !state_dir.exists() {
        fs::create_dir_all(&state_dir)?;
    }
    Ok(())
}
