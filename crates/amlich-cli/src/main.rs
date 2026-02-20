// amlich-cli - Vietnamese Lunar Calendar CLI for Waybar Integration
//
// Features:
// - Display current lunar date and Can Chi
// - Multiple display modes (full, lunar, canchi, minimal)
// - Toggle between modes with persistent state
// - JSON output for scripting
// - Waybar integration with tooltips

mod waybar;

use amlich_api::{get_day_info_for_date, DayInfoDto};
use chrono::{Datelike, Local};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "amlich",
    author = "Noy",
    version = "1.0.0",
    about = "Vietnamese Lunar Calendar CLI",
    long_about = "Vietnamese Lunar Calendar CLI tool with Waybar integration.\n\
                  Displays lunar dates, Can Chi, solar terms, and auspicious hours."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Show today's information (default)
    Today,

    /// Show information for a specific date
    Date {
        /// Date in YYYY-MM-DD format
        #[arg(value_name = "DATE")]
        date: String,
    },

    /// Toggle display mode (cycles through: full → lunar → canchi → minimal → full)
    Toggle,

    /// Output in JSON format
    Json {
        /// Date in YYYY-MM-DD format (optional, defaults to today)
        #[arg(value_name = "DATE")]
        date: Option<String>,
    },

    /// Show current display mode
    Mode,

    /// Set display mode explicitly
    SetMode {
        /// Mode: full, lunar, canchi, or minimal
        #[arg(value_name = "MODE")]
        mode: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum DisplayMode {
    Full,
    Lunar,
    CanChi,
    Minimal,
}

impl DisplayMode {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "full" => Some(DisplayMode::Full),
            "lunar" => Some(DisplayMode::Lunar),
            "canchi" => Some(DisplayMode::CanChi),
            "minimal" => Some(DisplayMode::Minimal),
            _ => None,
        }
    }

    fn next(&self) -> Self {
        match self {
            DisplayMode::Full => DisplayMode::Lunar,
            DisplayMode::Lunar => DisplayMode::CanChi,
            DisplayMode::CanChi => DisplayMode::Minimal,
            DisplayMode::Minimal => DisplayMode::Full,
        }
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
        write!(f, "{}", s)
    }
}

#[derive(Debug, Serialize)]
struct JsonOutput {
    solar: JsonSolar,
    lunar: JsonLunar,
    canchi: JsonCanChi,
    tiet_khi: JsonTietKhi,
    gio_hoang_dao: JsonGioHoangDao,
}

#[derive(Debug, Serialize)]
struct JsonSolar {
    day: i32,
    month: i32,
    year: i32,
    day_of_week: String,
    date_string: String,
}

#[derive(Debug, Serialize)]
struct JsonLunar {
    day: i32,
    month: i32,
    year: i32,
    is_leap_month: bool,
    date_string: String,
}

#[derive(Debug, Serialize)]
struct JsonCanChi {
    day: String,
    month: String,
    year: String,
    day_can: String,
    day_chi: String,
    month_can: String,
    month_chi: String,
    year_can: String,
    year_chi: String,
}

#[derive(Debug, Serialize)]
struct JsonTietKhi {
    name: String,
    description: String,
    season: String,
}

#[derive(Debug, Serialize)]
struct JsonGioHoangDao {
    good_hour_count: usize,
    hours: Vec<JsonHourInfo>,
}

#[derive(Debug, Serialize)]
struct JsonHourInfo {
    hour: usize,
    name: String,
    time_range: String,
    star: String,
    is_good: bool,
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

fn read_mode() -> DisplayMode {
    match fs::read_to_string(get_mode_file()) {
        Ok(content) => DisplayMode::from_str(content.trim()).unwrap_or(DisplayMode::Full),
        Err(_) => DisplayMode::Full,
    }
}

fn write_mode(mode: &DisplayMode) -> std::io::Result<()> {
    ensure_state_dir()?;
    fs::write(get_mode_file(), mode.to_string())
}

fn parse_date(date_str: &str) -> Result<(i32, i32, i32), String> {
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() != 3 {
        return Err("Date must be in YYYY-MM-DD format".to_string());
    }

    let year = parts[0]
        .parse::<i32>()
        .map_err(|_| "Invalid year".to_string())?;
    let month = parts[1]
        .parse::<i32>()
        .map_err(|_| "Invalid month".to_string())?;
    let day = parts[2]
        .parse::<i32>()
        .map_err(|_| "Invalid day".to_string())?;

    if !(1..=12).contains(&month) {
        return Err("Month must be between 1 and 12".to_string());
    }
    if !(1..=31).contains(&day) {
        return Err("Day must be between 1 and 31".to_string());
    }

    Ok((day, month, year))
}

fn convert_to_json(info: &DayInfoDto) -> JsonOutput {
    JsonOutput {
        solar: JsonSolar {
            day: info.solar.day,
            month: info.solar.month,
            year: info.solar.year,
            day_of_week: info.solar.day_of_week_name.clone(),
            date_string: info.solar.date_string.clone(),
        },
        lunar: JsonLunar {
            day: info.lunar.day,
            month: info.lunar.month,
            year: info.lunar.year,
            is_leap_month: info.lunar.is_leap_month,
            date_string: info.lunar.date_string.clone(),
        },
        canchi: JsonCanChi {
            day: info.canchi.day.full.clone(),
            month: info.canchi.month.full.clone(),
            year: info.canchi.year.full.clone(),
            day_can: info.canchi.day.can.clone(),
            day_chi: info.canchi.day.chi.clone(),
            month_can: info.canchi.month.can.clone(),
            month_chi: info.canchi.month.chi.clone(),
            year_can: info.canchi.year.can.clone(),
            year_chi: info.canchi.year.chi.clone(),
        },
        tiet_khi: JsonTietKhi {
            name: info.tiet_khi.name.clone(),
            description: info.tiet_khi.description.clone(),
            season: info.tiet_khi.season.clone(),
        },
        gio_hoang_dao: JsonGioHoangDao {
            good_hour_count: info.gio_hoang_dao.good_hour_count,
            hours: info
                .gio_hoang_dao
                .all_hours
                .iter()
                .map(|h| JsonHourInfo {
                    hour: h.hour_index,
                    name: h.hour_chi.clone(),
                    time_range: h.time_range.clone(),
                    star: h.star.clone(),
                    is_good: h.is_good,
                })
                .collect(),
        },
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Today) | None => {
            let now = Local::now();
            let info = get_day_info_for_date(now.day() as i32, now.month() as i32, now.year())
                .expect("today should always produce a valid date");
            let mode = read_mode();
            println!("{}", waybar::format_waybar_json(&info, &mode));
        }

        Some(Commands::Date { date }) => match parse_date(&date) {
            Ok((day, month, year)) => {
                let info = get_day_info_for_date(day, month, year)
                    .expect("validated date should produce day info");
                let mode = read_mode();
                println!("{}", waybar::format_waybar_json(&info, &mode));
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },

        Some(Commands::Toggle) => {
            let current_mode = read_mode();
            let new_mode = current_mode.next();

            if let Err(e) = write_mode(&new_mode) {
                eprintln!("Error saving mode: {}", e);
                std::process::exit(1);
            }

            // Output current state for Waybar
            let now = Local::now();
            let info = get_day_info_for_date(now.day() as i32, now.month() as i32, now.year())
                .expect("today should always produce a valid date");
            println!("{}", waybar::format_waybar_json(&info, &new_mode));
        }

        Some(Commands::Json { date }) => {
            let (day, month, year) = if let Some(date_str) = date {
                match parse_date(&date_str) {
                    Ok(parsed) => parsed,
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        std::process::exit(1);
                    }
                }
            } else {
                let now = Local::now();
                (now.day() as i32, now.month() as i32, now.year())
            };

            let info = get_day_info_for_date(day, month, year)
                .expect("validated date should produce day info");
            let json = convert_to_json(&info);
            println!("{}", serde_json::to_string_pretty(&json).unwrap());
        }

        Some(Commands::Mode) => {
            let mode = read_mode();
            println!("{mode}");
        }

        Some(Commands::SetMode { mode }) => match DisplayMode::from_str(&mode) {
            Some(new_mode) => {
                if let Err(e) = write_mode(&new_mode) {
                    eprintln!("Error saving mode: {}", e);
                    std::process::exit(1);
                }
                println!("Mode set to: {new_mode}");
            }
            None => {
                eprintln!("Invalid mode: {}", mode);
                eprintln!("Valid modes: full, lunar, canchi, minimal");
                std::process::exit(1);
            }
        },
    }
}
