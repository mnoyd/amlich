#[cfg(test)]
mod app;
#[cfg(test)]
mod bookmark_store;
#[cfg(test)]
mod date_jump;
#[cfg(test)]
mod event;
mod headless;
#[cfg(test)]
mod history;
mod profile;
#[cfg(test)]
mod search;
#[cfg(test)]
mod theme;
#[cfg(test)]
mod ui;
mod waybar;
#[cfg(test)]
mod widgets;

use std::ffi::OsString;
use std::io::{stdin, stdout, IsTerminal};

use amlich_api::v2::Include;
use amlich_api::{DateQuery, DayInsightDto, InsightSurface};
use chrono::{Datelike, Local, NaiveDate};
use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::headless::{
    parse_date, query, read_mode, set_mode, toggle_mode, DisplayMode, QueryFormat,
};
#[derive(Parser, Debug)]
#[command(
    name = "amlich",
    author = "Vietnamese Calendar Project",
    version,
    about = "Vietnamese Lunar Calendar with TUI and headless modes"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Open the interactive terminal UI
    Tui(TuiArgs),
    /// Query date information without launching the TUI
    Day(DayArgs),
    /// Query date range information
    Range(RangeArgs),
    /// Convert between solar and lunar dates
    Convert(ConvertArgs),
    /// Almanac-focused output for a day
    Almanac(AlmanacArgs),
    /// Cultural and interpretive day insight
    Insight(InsightArgs),
    /// List holidays in a year
    Holidays(HolidaysArgs),
    /// Solar term queries
    TietKhi(TietKhiArgs),
    /// Domain lookups
    Lookup(LookupArgs),
    /// Manage persistent user settings
    Config(ConfigArgs),
    /// DEPRECATED: use `day`
    Query(QueryArgs),
}

#[derive(Args, Debug)]
struct TuiArgs {
    /// Start TUI focused on a specific date in YYYY-MM-DD format
    #[arg(long, value_name = "DATE")]
    date: Option<String>,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum IncludeArg {
    Base,
    Canchi,
    #[value(name = "tiet-khi")]
    TietKhi,
    Hours,
    Fortune,
    Insight,
    Evidence,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum DayFormatArg {
    Json,
    Text,
    Waybar,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum StructuredFormatArg {
    Json,
    Text,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum RangeFormatArg {
    Json,
    Ndjson,
    Text,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum InsightLangArg {
    Vi,
    En,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum AlmanacTabArg {
    Overview,
    Taboos,
    Stars,
    Evidence,
    All,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum GenderArg {
    Male,
    Female,
}

#[derive(Args, Debug)]
struct DayArgs {
    /// Date in YYYY-MM-DD format (defaults to today)
    #[arg(value_name = "DATE")]
    date: Option<String>,

    /// Output format
    #[arg(long, value_enum, default_value_t = DayFormatArg::Json)]
    format: DayFormatArg,

    /// Include blocks in response (comma-separated)
    #[arg(
        long,
        value_enum,
        value_delimiter = ',',
        default_values_t = [
            IncludeArg::Base,
            IncludeArg::Canchi,
            IncludeArg::TietKhi,
            IncludeArg::Hours,
            IncludeArg::Fortune
        ]
    )]
    include: Vec<IncludeArg>,

    /// Field projection (comma-separated dot paths)
    #[arg(long, value_delimiter = ',', value_name = "FIELDS")]
    fields: Vec<String>,

    /// Pretty-print JSON formats
    #[arg(long)]
    pretty: bool,

    /// Timezone offset
    #[arg(long, value_name = "TZ")]
    timezone: Option<f64>,

    /// Almanac ruleset id (canonical id or alias)
    #[arg(long, value_name = "RULESET")]
    ruleset_id: Option<String>,

    /// Contextual event kind for recommendation synthesis
    #[arg(long, value_name = "EVENT_KIND")]
    event_kind: Option<String>,

    /// Enable recommendation packs (comma-separated pack ids)
    #[arg(long, value_delimiter = ',', value_name = "PACKS")]
    recommendation_packs: Vec<String>,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum OutputFormat {
    DayinfoJson,
    Waybar,
    Text,
}

impl From<OutputFormat> for QueryFormat {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::DayinfoJson => QueryFormat::DayInfoJson,
            OutputFormat::Waybar => QueryFormat::Waybar,
            OutputFormat::Text => QueryFormat::Text,
        }
    }
}

#[derive(Args, Debug)]
struct QueryArgs {
    /// Date in YYYY-MM-DD format (defaults to today)
    #[arg(value_name = "DATE")]
    date: Option<String>,

    /// Output format
    #[arg(long, value_enum, default_value_t = OutputFormat::DayinfoJson)]
    format: OutputFormat,

    /// Display mode used by waybar/text rendering
    #[arg(long, value_parser = parse_mode, value_name = "MODE")]
    mode: Option<DisplayMode>,

    /// Pretty-print JSON formats
    #[arg(long)]
    pretty: bool,
}

#[derive(Args, Debug)]
struct RangeArgs {
    #[arg(long, value_name = "DATE")]
    start: String,

    #[arg(long, value_name = "DATE")]
    end: String,

    #[arg(long, value_enum, default_value_t = RangeFormatArg::Json)]
    format: RangeFormatArg,

    #[arg(
        long,
        value_enum,
        value_delimiter = ',',
        default_values_t = [IncludeArg::Base, IncludeArg::Canchi, IncludeArg::TietKhi]
    )]
    include: Vec<IncludeArg>,

    #[arg(long)]
    pretty: bool,

    #[arg(long, value_name = "TZ")]
    timezone: Option<f64>,

    #[arg(long, value_name = "RULESET")]
    ruleset_id: Option<String>,

    #[arg(long, value_name = "EVENT_KIND")]
    event_kind: Option<String>,

    #[arg(long, value_delimiter = ',', value_name = "PACKS")]
    recommendation_packs: Vec<String>,
}

#[derive(Args, Debug)]
struct ConvertArgs {
    #[command(subcommand)]
    command: ConvertCommand,
}

#[derive(Subcommand, Debug)]
enum ConvertCommand {
    SolarToLunar(SolarToLunarArgs),
    LunarToSolar(LunarToSolarArgs),
}

#[derive(Args, Debug)]
struct SolarToLunarArgs {
    #[arg(value_name = "DATE")]
    date: String,

    #[arg(long, value_name = "TZ")]
    timezone: Option<f64>,

    #[arg(long, value_enum, default_value_t = StructuredFormatArg::Json)]
    format: StructuredFormatArg,

    #[arg(long)]
    pretty: bool,
}

#[derive(Args, Debug)]
struct LunarToSolarArgs {
    #[arg(long)]
    day: i32,
    #[arg(long)]
    month: i32,
    #[arg(long)]
    year: i32,
    #[arg(long, default_value_t = false)]
    leap: bool,

    #[arg(long, value_name = "TZ")]
    timezone: Option<f64>,

    #[arg(long, value_enum, default_value_t = StructuredFormatArg::Json)]
    format: StructuredFormatArg,

    #[arg(long)]
    pretty: bool,
}

#[derive(Args, Debug)]
struct AlmanacArgs {
    #[arg(value_name = "DATE")]
    date: Option<String>,

    #[arg(long, value_enum, default_value_t = AlmanacTabArg::All)]
    tab: AlmanacTabArg,

    #[arg(long, value_enum, default_value_t = StructuredFormatArg::Text)]
    format: StructuredFormatArg,

    #[arg(long)]
    pretty: bool,

    #[arg(long, value_name = "TZ")]
    timezone: Option<f64>,
}

#[derive(Args, Debug)]
struct InsightArgs {
    #[arg(value_name = "DATE")]
    date: Option<String>,

    #[arg(long, value_enum, default_value_t = InsightLangArg::Vi)]
    lang: InsightLangArg,

    #[arg(long, value_enum, default_value_t = StructuredFormatArg::Text)]
    format: StructuredFormatArg,

    #[arg(long)]
    pretty: bool,

    #[arg(long, value_name = "TZ")]
    timezone: Option<f64>,
}

#[derive(Args, Debug)]
struct HolidaysArgs {
    #[arg(value_name = "YEAR")]
    year: i32,

    #[arg(long)]
    major: bool,

    #[arg(long, value_name = "CATEGORY")]
    category: Vec<String>,

    #[arg(long, value_enum, default_value_t = StructuredFormatArg::Text)]
    format: StructuredFormatArg,

    #[arg(long)]
    pretty: bool,
}

#[derive(Args, Debug)]
struct TietKhiArgs {
    #[arg(value_name = "DATE", conflicts_with = "year")]
    date: Option<String>,

    #[arg(long, value_name = "YEAR", conflicts_with = "date")]
    year: Option<i32>,

    #[arg(long, value_enum, default_value_t = StructuredFormatArg::Text)]
    format: StructuredFormatArg,

    #[arg(long)]
    pretty: bool,

    #[arg(long, value_name = "TZ")]
    timezone: Option<f64>,
}

#[derive(Args, Debug)]
struct LookupArgs {
    #[command(subcommand)]
    command: LookupCommand,
}

#[derive(Subcommand, Debug)]
enum LookupCommand {
    NaAm(LookupNaAmArgs),
    TenGods(LookupTenGodsArgs),
    Kua(LookupKuaArgs),
    Bazi(BaziArgs),
    PersonalDay(PersonalDayArgs),
    PersonalDayMatrix(PersonalDayMatrixArgs),
    HourSelection(HourSelectionArgs),
    Rulesets(LookupCatalogArgs),
    RecommendationPacks(LookupCatalogArgs),
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum InsightSurfaceArg {
    Chart,
    Analysis,
    Timing,
    Advisory,
    Metrics,
    Report,
}

impl From<InsightSurfaceArg> for InsightSurface {
    fn from(value: InsightSurfaceArg) -> Self {
        match value {
            InsightSurfaceArg::Chart => InsightSurface::Chart,
            InsightSurfaceArg::Analysis => InsightSurface::Analysis,
            InsightSurfaceArg::Timing => InsightSurface::Timing,
            InsightSurfaceArg::Advisory => InsightSurface::Advisory,
            InsightSurfaceArg::Metrics => InsightSurface::Metrics,
            InsightSurfaceArg::Report => InsightSurface::Report,
        }
    }
}

#[derive(Args, Debug)]
struct BaziArgs {
    #[arg(value_name = "DATE")]
    date: String,

    #[arg(long, value_name = "HOUR")]
    hour: u8,

    #[arg(long, default_value_t = 0)]
    minute: u8,

    #[arg(long, value_name = "TZ")]
    timezone: Option<f64>,

    #[arg(long, value_name = "LONGITUDE")]
    longitude: Option<f64>,

    #[arg(long, default_value_t = false)]
    use_solar_time: bool,

    #[arg(long, value_enum)]
    gender: Option<GenderArg>,

    #[arg(long)]
    current_age: Option<f64>,

    #[arg(long)]
    target_year: Option<i32>,

    #[arg(long, value_delimiter = ',')]
    months: Vec<i32>,

    #[arg(long, value_enum, default_value_t = InsightSurfaceArg::Report)]
    surface: InsightSurfaceArg,

    #[arg(long, value_enum, default_value_t = StructuredFormatArg::Json)]
    format: StructuredFormatArg,

    #[arg(long)]
    pretty: bool,
}

#[derive(Args, Debug)]
struct PersonalDayArgs {
    #[arg(value_name = "DATE")]
    date: String,

    #[arg(long)]
    birth_year: Option<i32>,

    #[arg(long)]
    birth_month: Option<i32>,

    #[arg(long)]
    birth_day: Option<i32>,

    #[arg(long, value_enum)]
    gender: Option<GenderArg>,

    #[arg(long, value_enum, default_value_t = InsightSurfaceArg::Report)]
    surface: InsightSurfaceArg,

    #[arg(long, value_enum, default_value_t = StructuredFormatArg::Json)]
    format: StructuredFormatArg,

    #[arg(long)]
    pretty: bool,
}

#[derive(Args, Debug)]
struct PersonalDayMatrixArgs {
    #[arg(value_name = "DATE")]
    date: String,

    #[arg(long)]
    birth_year: Option<i32>,

    #[arg(long)]
    birth_month: Option<i32>,

    #[arg(long)]
    birth_day: Option<i32>,

    #[arg(long)]
    hour: Option<u8>,

    #[arg(long)]
    minute: Option<u8>,

    #[arg(long, value_enum)]
    gender: Option<GenderArg>,

    #[arg(long, value_name = "TZ")]
    timezone: Option<f64>,

    #[arg(long, value_name = "LONGITUDE")]
    longitude: Option<f64>,

    #[arg(long, default_value_t = false)]
    use_solar_time: bool,

    #[arg(long, value_enum, default_value_t = StructuredFormatArg::Json)]
    format: StructuredFormatArg,

    #[arg(long)]
    pretty: bool,
}

#[derive(Args, Debug)]
struct HourSelectionArgs {
    #[arg(value_name = "DATE")]
    date: String,

    #[arg(long, value_enum, default_value_t = InsightSurfaceArg::Report)]
    surface: InsightSurfaceArg,

    #[arg(long, value_enum, default_value_t = StructuredFormatArg::Json)]
    format: StructuredFormatArg,

    #[arg(long)]
    pretty: bool,
}

#[derive(Args, Debug)]
struct LookupNaAmArgs {
    #[arg(long, conflicts_with_all = ["can", "chi"])]
    index: Option<u8>,

    #[arg(long, requires = "chi", conflicts_with = "index")]
    can: Option<String>,

    #[arg(long, requires = "can", conflicts_with = "index")]
    chi: Option<String>,

    #[arg(long, value_enum, default_value_t = StructuredFormatArg::Json)]
    format: StructuredFormatArg,

    #[arg(long)]
    pretty: bool,
}

#[derive(Args, Debug)]
struct LookupTenGodsArgs {
    #[arg(long = "day-can")]
    day_can: String,

    #[arg(long = "target-can")]
    target_can: String,

    #[arg(long, value_enum, default_value_t = StructuredFormatArg::Json)]
    format: StructuredFormatArg,

    #[arg(long)]
    pretty: bool,
}

#[derive(Args, Debug)]
struct LookupKuaArgs {
    #[arg(long = "birth-year")]
    birth_year: i32,

    #[arg(long, value_enum)]
    gender: GenderArg,

    #[arg(long, value_enum, default_value_t = StructuredFormatArg::Json)]
    format: StructuredFormatArg,

    #[arg(long)]
    pretty: bool,
}

#[derive(Args, Debug)]
struct LookupCatalogArgs {
    #[arg(long, value_enum, default_value_t = StructuredFormatArg::Json)]
    format: StructuredFormatArg,

    #[arg(long)]
    pretty: bool,
}

#[derive(Args, Debug)]
struct ConfigArgs {
    #[command(subcommand)]
    command: ConfigCommand,
}

#[derive(Subcommand, Debug)]
enum ConfigCommand {
    Mode(ModeArgs),
    Profile(ProfileSubArgs),
}

#[derive(Args, Debug)]
struct ProfileSubArgs {
    #[command(subcommand)]
    command: ProfileCommand,
}

#[derive(Subcommand, Debug)]
enum ProfileCommand {
    Show,
    Set {
        #[arg(long)]
        birth_year: Option<i32>,
        #[arg(long)]
        birth_month: Option<i32>,
        #[arg(long)]
        birth_day: Option<i32>,
        #[arg(long)]
        birth_hour: Option<u8>,
        #[arg(long)]
        birth_minute: Option<u8>,
        #[arg(long)]
        gender: Option<String>,
    },
    Clear,
}

#[derive(Args, Debug)]
struct ModeArgs {
    #[command(subcommand)]
    command: ModeCommand,
}

#[derive(Subcommand, Debug)]
enum ModeCommand {
    Show,
    Set {
        #[arg(value_parser = parse_mode, value_name = "MODE")]
        mode: DisplayMode,
    },
    Toggle,
}

fn parse_mode(input: &str) -> Result<DisplayMode, String> {
    input.parse()
}

fn main() {
    let args = match rewrite_headless_alias(std::env::args_os().collect()) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("Error: {err}");
            std::process::exit(1);
        }
    };

    let cli = Cli::parse_from(args);

    if let Err(err) = run(cli) {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), String> {
    match cli.command {
        Some(Command::Tui(args)) => {
            let date = args.date.as_deref().map(parse_date).transpose()?;
            amlich_tui::run_tui(date).map_err(|e| format!("failed to run TUI: {e}"))?;
        }
        Some(Command::Day(args)) => run_day(args)?,
        Some(Command::Range(args)) => run_range(args)?,
        Some(Command::Convert(args)) => run_convert(args)?,
        Some(Command::Almanac(args)) => run_almanac(args)?,
        Some(Command::Insight(args)) => run_insight(args)?,
        Some(Command::Holidays(args)) => run_holidays(args)?,
        Some(Command::TietKhi(args)) => run_tiet_khi(args)?,
        Some(Command::Lookup(args)) => run_lookup(args)?,
        Some(Command::Config(args)) => run_config(args)?,
        Some(Command::Query(args)) => {
            eprintln!("Warning: `amlich query` is deprecated; use `amlich day`");
            run_query(args)?;
        }
        None => run_auto_mode()?,
    }

    Ok(())
}

fn parse_date_or_today(input: Option<&str>) -> Result<NaiveDate, String> {
    match input {
        Some(value) => parse_date(value),
        None => Ok(Local::now().date_naive()),
    }
}

fn include_args_to_api(include: &[IncludeArg]) -> Vec<Include> {
    include
        .iter()
        .map(|x| match x {
            IncludeArg::Base => Include::Base,
            IncludeArg::Canchi => Include::CanChi,
            IncludeArg::TietKhi => Include::TietKhi,
            IncludeArg::Hours => Include::Hours,
            IncludeArg::Fortune => Include::Fortune,
            IncludeArg::Insight => Include::Insight,
            IncludeArg::Evidence => Include::Evidence,
        })
        .collect()
}

fn print_json<T: serde::Serialize>(value: &T, pretty: bool) -> Result<(), String> {
    let output = if pretty {
        serde_json::to_string_pretty(value).map_err(|e| format!("failed to render json: {e}"))?
    } else {
        serde_json::to_string(value).map_err(|e| format!("failed to render json: {e}"))?
    };
    println!("{output}");
    Ok(())
}

fn query_from_date(date: NaiveDate, timezone: Option<f64>) -> DateQuery {
    DateQuery {
        day: date.day() as i32,
        month: date.month() as i32,
        year: date.year(),
        timezone,
        ruleset_id: None,
        event_kind: None,
        enabled_pack_ids: vec![],
    }
}

fn run_day(args: DayArgs) -> Result<(), String> {
    let date = parse_date_or_today(args.date.as_deref())?;
    let mut query = query_from_date(date, args.timezone);
    query.ruleset_id = args.ruleset_id.clone();
    query.event_kind = args.event_kind.clone();
    query.enabled_pack_ids = args.recommendation_packs.clone();
    let includes = include_args_to_api(&args.include);

    match args.format {
        DayFormatArg::Json => {
            if args.fields.is_empty() {
                let bundle = amlich_api::v2::get_day_bundle(&query, &includes)?;
                print_json(&bundle, args.pretty)?;
            } else {
                let projected =
                    amlich_api::v2::get_day_bundle_projected(&query, &includes, &args.fields)?;
                print_json(&projected, args.pretty)?;
            }
        }
        DayFormatArg::Text => {
            let bundle = amlich_api::v2::get_day_bundle(&query, &includes)?;
            println!(
                "{} | lunar {} | tiet-khi {}",
                bundle.solar.date_string,
                bundle.lunar.date_string,
                bundle
                    .tiet_khi
                    .as_ref()
                    .map(|t| t.name.as_str())
                    .unwrap_or("n/a")
            );
        }
        DayFormatArg::Waybar => {
            let info = amlich_api::get_day_info(&query)?;
            let payload = waybar::build_waybar_payload(&info, &read_mode());
            if args.pretty {
                print_json(&payload, true)?;
            } else {
                println!("{payload}");
            }
        }
    }

    Ok(())
}

fn run_range(args: RangeArgs) -> Result<(), String> {
    let start = parse_date(&args.start)?;
    let end = parse_date(&args.end)?;

    let mut start_query = query_from_date(start, args.timezone);
    let mut end_query = query_from_date(end, args.timezone);
    start_query.ruleset_id = args.ruleset_id.clone();
    start_query.event_kind = args.event_kind.clone();
    start_query.enabled_pack_ids = args.recommendation_packs.clone();
    end_query.ruleset_id = args.ruleset_id.clone();
    end_query.event_kind = args.event_kind.clone();
    end_query.enabled_pack_ids = args.recommendation_packs.clone();
    let includes = include_args_to_api(&args.include);
    let range = amlich_api::v2::get_day_range(start_query, end_query, &includes)?;

    match args.format {
        RangeFormatArg::Json => print_json(&range, args.pretty)?,
        RangeFormatArg::Ndjson => {
            for day in &range.days {
                println!(
                    "{}",
                    serde_json::to_string(day)
                        .map_err(|e| format!("failed to render ndjson: {e}"))?
                );
            }
        }
        RangeFormatArg::Text => {
            for day in &range.days {
                let tiet_khi = day
                    .tiet_khi
                    .as_ref()
                    .map(|t| t.name.as_str())
                    .unwrap_or("n/a");
                println!(
                    "{} | lunar {} | tiet-khi {}",
                    day.solar.date_string, day.lunar.date_string, tiet_khi
                );
            }
        }
    }

    Ok(())
}

fn run_convert(args: ConvertArgs) -> Result<(), String> {
    match args.command {
        ConvertCommand::SolarToLunar(a) => run_convert_solar_to_lunar(a),
        ConvertCommand::LunarToSolar(a) => run_convert_lunar_to_solar(a),
    }
}

fn run_convert_solar_to_lunar(args: SolarToLunarArgs) -> Result<(), String> {
    let date = parse_date(&args.date)?;
    let query = query_from_date(date, args.timezone);
    let result = amlich_api::v2::convert_solar_to_lunar(&query)?;
    match args.format {
        StructuredFormatArg::Json => print_json(&result, args.pretty)?,
        StructuredFormatArg::Text => println!("{} -> {}", date, result.date_string),
    }
    Ok(())
}

fn run_convert_lunar_to_solar(args: LunarToSolarArgs) -> Result<(), String> {
    let result = amlich_api::v2::convert_lunar_to_solar(
        args.day,
        args.month,
        args.year,
        args.leap,
        args.timezone,
    )?;
    match args.format {
        StructuredFormatArg::Json => print_json(&result, args.pretty)?,
        StructuredFormatArg::Text => println!("{}", result.date_string),
    }
    Ok(())
}

fn run_almanac(args: AlmanacArgs) -> Result<(), String> {
    let date = parse_date_or_today(args.date.as_deref())?;
    let query = query_from_date(date, args.timezone);
    let almanac = amlich_api::v2::get_almanac(&query)?;
    match args.format {
        StructuredFormatArg::Json => print_json(&almanac, args.pretty)?,
        StructuredFormatArg::Text => {
            if matches!(args.tab, AlmanacTabArg::Overview | AlmanacTabArg::All) {
                println!(
                    "Ruleset: {}@{} ({})",
                    almanac.ruleset_id, almanac.ruleset_version, almanac.profile
                );
                println!("Truc: {} [{}]", almanac.truc.name, almanac.truc.quality);
                println!(
                    "Day element: {} ({})",
                    almanac.day_element.na_am, almanac.day_element.element
                );
            }
            if matches!(args.tab, AlmanacTabArg::Taboos | AlmanacTabArg::All) {
                if almanac.taboos.is_empty() {
                    println!("Taboos: none");
                } else {
                    println!("Taboos:");
                    for taboo in &almanac.taboos {
                        println!("- [{}] {}: {}", taboo.severity, taboo.name, taboo.reason);
                    }
                }
            }
            if matches!(args.tab, AlmanacTabArg::Stars | AlmanacTabArg::All) {
                println!("Stars (cat): {}", almanac.stars.cat_tinh.join(", "));
                println!("Stars (sat): {}", almanac.stars.sat_tinh.join(", "));
            }
            if matches!(args.tab, AlmanacTabArg::Evidence | AlmanacTabArg::All) {
                if let Some(ev) = &almanac.day_element.evidence {
                    println!(
                        "Evidence day_element: {} · {} · {}",
                        ev.source_id, ev.method, ev.profile
                    );
                }
                if let Some(ev) = &almanac.truc.evidence {
                    println!(
                        "Evidence truc: {} · {} · {}",
                        ev.source_id, ev.method, ev.profile
                    );
                }
            }
        }
    }
    Ok(())
}

fn localized_text(lang: InsightLangArg, text: &amlich_api::LocalizedTextDto) -> String {
    match lang {
        InsightLangArg::Vi => text.vi.clone(),
        InsightLangArg::En => text.en.clone(),
    }
}

fn localized_list(lang: InsightLangArg, list: &amlich_api::LocalizedListDto) -> Vec<String> {
    match lang {
        InsightLangArg::Vi => list.vi.clone(),
        InsightLangArg::En => list.en.clone(),
    }
}

fn render_insight_text(lang: InsightLangArg, insight: &DayInsightDto) {
    println!(
        "Date: {} | Lunar: {}",
        insight.solar.date_string, insight.lunar.date_string
    );
    if let Some(festival) = &insight.festival {
        let names = match lang {
            InsightLangArg::Vi => festival.names.vi.clone(),
            InsightLangArg::En => festival.names.en.clone(),
        };
        println!("Festival: {}", names.join(", "));
    }
    if let Some(holiday) = &insight.holiday {
        let names = match lang {
            InsightLangArg::Vi => holiday.names.vi.clone(),
            InsightLangArg::En => holiday.names.en.clone(),
        };
        println!("Holiday: {}", names.join(", "));
    }
    if let Some(guidance) = &insight.day_guidance {
        println!(
            "Good for: {}",
            localized_list(lang, &guidance.good_for).join(", ")
        );
        println!(
            "Avoid: {}",
            localized_list(lang, &guidance.avoid_for).join(", ")
        );
    }
    if let Some(tiet_khi) = &insight.tiet_khi {
        println!("Tiet khi: {}", localized_text(lang, &tiet_khi.name));
        println!("Weather: {}", localized_text(lang, &tiet_khi.weather));
    }
    if let Some(truc) = &insight.truc {
        println!(
            "Truc: {} ({}) — {}",
            truc.name,
            truc.quality,
            localized_text(lang, &truc.meaning)
        );
        let good = localized_list(lang, &truc.good_for);
        if !good.is_empty() {
            println!("  Good for: {}", good.join(", "));
        }
        let avoid = localized_list(lang, &truc.avoid_for);
        if !avoid.is_empty() {
            println!("  Avoid: {}", avoid.join(", "));
        }
    }
    if let Some(deity) = &insight.day_deity {
        let class_text = localized_text(lang, &deity.classification_meaning);
        println!(
            "Day Deity: {} ({} — {})",
            deity.name, deity.classification, class_text
        );
        if let Some(meaning) = &deity.deity_meaning {
            println!("  {}", localized_text(lang, meaning));
        }
    }
    if let Some(stars) = &insight.stars {
        if !stars.cat_tinh.is_empty() {
            println!("Cat tinh: {}", stars.cat_tinh.join(", "));
        }
        if !stars.sat_tinh.is_empty() {
            println!("Sat tinh: {}", stars.sat_tinh.join(", "));
        }
    }
    if let Some(na_am) = &insight.na_am {
        println!(
            "Na Am: {} ({}) — {}",
            na_am.na_am,
            na_am.element,
            localized_text(lang, &na_am.meaning)
        );
    }
    if let Some(taboos) = &insight.taboos {
        if !taboos.is_empty() {
            println!("Taboos:");
            for t in taboos {
                println!("  [{}] {} — {}", t.severity, t.name, t.reason);
            }
        }
    }
    if let Some(travel) = &insight.travel {
        println!(
            "Travel: {} | Tai Than: {} | Hy Than: {}",
            travel.xuat_hanh_huong, travel.tai_than, travel.hy_than
        );
    }
    if let Some(ten_gods) = &insight.ten_gods {
        if let Some(entry) = &ten_gods.to_year_stem {
            println!(
                "Ten Gods (year): {} — {}",
                entry.label,
                localized_text(lang, &entry.meaning)
            );
        }
        if let Some(entry) = &ten_gods.to_self {
            println!(
                "Ten Gods (self): {} — {}",
                entry.label,
                localized_text(lang, &entry.meaning)
            );
        }
    }
    if let Some(hours) = &insight.hours {
        let hour_strs: Vec<String> = hours
            .good_hours
            .iter()
            .map(|h| format!("{} ({})", h.chi, h.time_range))
            .collect();
        println!(
            "Good hours ({}): {}",
            hours.good_hour_count,
            hour_strs.join(", ")
        );
    }
    if let Some(tu_menh) = &insight.tu_menh {
        println!(
            "Tu Menh: Kua {} ({}) — {}",
            tu_menh.kua,
            tu_menh.group,
            localized_text(lang, &tu_menh.meaning)
        );
        if !tu_menh.favorable_directions.is_empty() {
            println!("  Favorable: {}", tu_menh.favorable_directions.join(", "));
        }
        if !tu_menh.unfavorable_directions.is_empty() {
            println!(
                "  Unfavorable: {}",
                tu_menh.unfavorable_directions.join(", ")
            );
        }
    }
    if let Some(dai_van) = &insight.dai_van {
        println!(
            "Dai Van: {} — {}",
            dai_van.direction,
            localized_text(lang, &dai_van.direction_meaning)
        );
        if let Some(pillar) = &dai_van.current_pillar {
            println!(
                "  Current pillar: {} ({}-{} tuoi) — {}",
                pillar.can_chi,
                pillar.start_age,
                pillar.end_age,
                localized_text(lang, &pillar.element_meaning)
            );
        }
    }
}

fn run_insight(args: InsightArgs) -> Result<(), String> {
    let date = parse_date_or_today(args.date.as_deref())?;
    let query = query_from_date(date, args.timezone);
    let profile = crate::profile::load_profile();
    let gender = profile.gender.map(|g| match g {
        crate::profile::ProfileGender::Male => amlich_core::almanac::tu_menh::Gender::Male,
        crate::profile::ProfileGender::Female => amlich_core::almanac::tu_menh::Gender::Female,
    });
    let insight = amlich_api::v2::get_insight_with_profile(
        &query,
        profile.birth_year,
        profile.birth_month,
        profile.birth_day,
        gender,
    )?;
    match args.format {
        StructuredFormatArg::Json => print_json(&insight, args.pretty)?,
        StructuredFormatArg::Text => render_insight_text(args.lang, &insight),
    }
    Ok(())
}

fn run_holidays(args: HolidaysArgs) -> Result<(), String> {
    let mut holidays = amlich_api::get_holidays(args.year, args.major);
    if !args.category.is_empty() {
        holidays.retain(|h| {
            args.category
                .iter()
                .any(|c| c.eq_ignore_ascii_case(&h.category))
        });
    }
    match args.format {
        StructuredFormatArg::Json => print_json(&holidays, args.pretty)?,
        StructuredFormatArg::Text => {
            for h in holidays {
                println!(
                    "{}: {:04}-{:02}-{:02} [{}]",
                    h.name, h.solar_year, h.solar_month, h.solar_day, h.category
                );
            }
        }
    }
    Ok(())
}

fn run_tiet_khi(args: TietKhiArgs) -> Result<(), String> {
    if let Some(year) = args.year {
        let year_data = amlich_api::v2::get_tiet_khi_for_year(year, args.timezone)?;
        match args.format {
            StructuredFormatArg::Json => print_json(&year_data, args.pretty)?,
            StructuredFormatArg::Text => {
                for transition in year_data.transitions {
                    println!("{}: {}", transition.date, transition.term.name);
                }
            }
        }
        return Ok(());
    }

    let date = parse_date_or_today(args.date.as_deref())?;
    let query = query_from_date(date, args.timezone);
    let day = amlich_api::get_day_info(&query)?;
    match args.format {
        StructuredFormatArg::Json => print_json(&day.tiet_khi, args.pretty)?,
        StructuredFormatArg::Text => {
            println!(
                "{}: {} ({})",
                day.solar.date_string, day.tiet_khi.name, day.tiet_khi.season
            );
        }
    }
    Ok(())
}

fn run_lookup(args: LookupArgs) -> Result<(), String> {
    match args.command {
        LookupCommand::NaAm(a) => run_lookup_na_am(a),
        LookupCommand::TenGods(a) => run_lookup_ten_gods(a),
        LookupCommand::Kua(a) => run_lookup_kua(a),
        LookupCommand::Bazi(a) => run_lookup_bazi(a),
        LookupCommand::PersonalDay(a) => run_lookup_personal_day(a),
        LookupCommand::PersonalDayMatrix(a) => run_lookup_personal_day_matrix(a),
        LookupCommand::HourSelection(a) => run_lookup_hour_selection(a),
        LookupCommand::Rulesets(a) => run_lookup_rulesets(a),
        LookupCommand::RecommendationPacks(a) => run_lookup_recommendation_packs(a),
    }
}

fn run_lookup_rulesets(args: LookupCatalogArgs) -> Result<(), String> {
    let catalog = amlich_api::get_ruleset_catalog();
    match args.format {
        StructuredFormatArg::Json => print_json(&catalog, args.pretty)?,
        StructuredFormatArg::Text => {
            for entry in catalog {
                let aliases = if entry.aliases.is_empty() {
                    String::new()
                } else {
                    format!(" aliases={} ", entry.aliases.join(","))
                };
                println!(
                    "{}@{} [{}] profile={} default={}{}tz={} schema={}",
                    entry.id,
                    entry.version,
                    entry.region,
                    entry.profile,
                    entry.is_default,
                    aliases,
                    entry.defaults.tz_offset,
                    entry.schema_version
                );
            }
        }
    }
    Ok(())
}

fn run_lookup_recommendation_packs(args: LookupCatalogArgs) -> Result<(), String> {
    let catalog = amlich_api::get_recommendation_pack_catalog();
    match args.format {
        StructuredFormatArg::Json => print_json(&catalog, args.pretty)?,
        StructuredFormatArg::Text => {
            for entry in catalog {
                println!(
                    "{}@{} family={} mode={}",
                    entry.pack_id, entry.version, entry.source_family, entry.mode
                );
            }
        }
    }
    Ok(())
}

fn run_lookup_na_am(args: LookupNaAmArgs) -> Result<(), String> {
    let response = match (args.index, args.can.as_deref(), args.chi.as_deref()) {
        (Some(index), None, None) => amlich_api::v2::lookup_na_am_by_index(index),
        (None, Some(can), Some(chi)) => amlich_api::v2::lookup_na_am_by_pair(can, chi),
        _ => {
            return Err("provide either --index or --can+--chi".to_string());
        }
    };

    match args.format {
        StructuredFormatArg::Json => print_json(&response, args.pretty)?,
        StructuredFormatArg::Text => match response {
            amlich_api::NaAmResponseDto::Success(data) => {
                println!(
                    "{} {} (#{}): {} ({})",
                    data.can, data.chi, data.cycle_index, data.na_am, data.element
                )
            }
            amlich_api::NaAmResponseDto::Error(err) => {
                println!("{}: {}", err.error, err.message)
            }
        },
    }
    Ok(())
}

fn run_lookup_ten_gods(args: LookupTenGodsArgs) -> Result<(), String> {
    let result = amlich_api::v2::lookup_ten_gods(&args.day_can, &args.target_can)?;
    match args.format {
        StructuredFormatArg::Json => print_json(&result, args.pretty)?,
        StructuredFormatArg::Text => println!(
            "{} | {} | same_polarity={}",
            result.label, result.relation, result.same_polarity
        ),
    }
    Ok(())
}

fn run_lookup_kua(args: LookupKuaArgs) -> Result<(), String> {
    let gender = match args.gender {
        GenderArg::Male => amlich_core::Gender::Male,
        GenderArg::Female => amlich_core::Gender::Female,
    };
    let result = amlich_api::v2::lookup_kua(args.birth_year, gender);
    match args.format {
        StructuredFormatArg::Json => print_json(&result, args.pretty)?,
        StructuredFormatArg::Text => {
            println!("Kua {} ({})", result.kua, result.group);
            println!("Favorable: {}", result.favorable_directions.join(", "));
            println!("Unfavorable: {}", result.unfavorable_directions.join(", "));
        }
    }
    Ok(())
}

fn run_lookup_bazi(args: BaziArgs) -> Result<(), String> {
    let date = parse_date(&args.date)?;
    let profile = crate::profile::load_profile();
    let gender = args
        .gender
        .map(gender_arg_to_str)
        .or_else(|| profile.gender.map(profile_gender_to_str));

    let query = amlich_api::BaziQuery {
        day: date.day() as i32,
        month: date.month() as i32,
        year: date.year(),
        hour: args.hour,
        minute: args.minute,
        timezone: args.timezone,
        longitude: args.longitude,
        use_solar_time: args.use_solar_time,
        gender: gender.map(str::to_string),
    };

    let timing = match (
        args.current_age.or_else(|| profile.birth_year.map(|birth_year| {
            (date.year() - birth_year) as f64
        })),
        args.target_year,
    ) {
        (Some(current_age), Some(target_year)) => Some(amlich_api::BaziTimingQuery {
            current_age,
            target_year,
            months: args.months.clone(),
        }),
        (None, None) => None,
        (Some(_), None) | (None, Some(_))
            if matches!(InsightSurface::from(args.surface), InsightSurface::Timing) =>
        {
            return Err(
                "timing requires both --current-age and --target-year, or a profile birth year plus --target-year"
                    .to_string(),
            )
        }
        _ => None,
    };

    match InsightSurface::from(args.surface) {
        InsightSurface::Chart => render_structured(
            &amlich_api::get_bazi_chart(&query)?,
            args.format,
            args.pretty,
        ),
        InsightSurface::Analysis => render_structured(
            &amlich_api::get_bazi_analysis(&query)?,
            args.format,
            args.pretty,
        ),
        InsightSurface::Timing => {
            let timing = timing
                .as_ref()
                .ok_or_else(|| "timing surface requires timing inputs".to_string())?;
            render_structured(
                &amlich_api::get_bazi_timing(&query, timing)?,
                args.format,
                args.pretty,
            )
        }
        InsightSurface::Advisory => render_structured(
            &amlich_api::get_bazi_advisory(&query, timing.as_ref())?,
            args.format,
            args.pretty,
        ),
        InsightSurface::Metrics => render_structured(
            &amlich_api::get_bazi_metrics(&query, timing.as_ref())?,
            args.format,
            args.pretty,
        ),
        InsightSurface::Report => render_structured(
            &amlich_api::get_bazi_report(&query, timing.as_ref())?,
            args.format,
            args.pretty,
        ),
    }
}

fn run_lookup_personal_day(args: PersonalDayArgs) -> Result<(), String> {
    let date = parse_date(&args.date)?;
    let profile = crate::profile::load_profile();
    let birth_year = args.birth_year.or(profile.birth_year);
    let birth_month = args.birth_month.or(profile.birth_month);
    let birth_day = args.birth_day.or(profile.birth_day);
    let gender = args
        .gender
        .map(|g| match g {
            GenderArg::Male => amlich_core::almanac::tu_menh::Gender::Male,
            GenderArg::Female => amlich_core::almanac::tu_menh::Gender::Female,
        })
        .or_else(|| {
            profile.gender.map(|g| match g {
                crate::profile::ProfileGender::Male => amlich_core::almanac::tu_menh::Gender::Male,
                crate::profile::ProfileGender::Female => {
                    amlich_core::almanac::tu_menh::Gender::Female
                }
            })
        });

    let query = DateQuery {
        day: date.day() as i32,
        month: date.month() as i32,
        year: date.year(),
        timezone: None,
        ruleset_id: None,
        event_kind: None,
        enabled_pack_ids: vec![],
    };

    match InsightSurface::from(args.surface) {
        InsightSurface::Chart => render_structured(
            &amlich_api::get_personal_day_chart(
                &query,
                birth_year,
                birth_month,
                birth_day,
                gender,
            )?,
            args.format,
            args.pretty,
        ),
        InsightSurface::Analysis => render_structured(
            &amlich_api::get_personal_day_analysis(
                &query,
                birth_year,
                birth_month,
                birth_day,
                gender,
            )?,
            args.format,
            args.pretty,
        ),
        InsightSurface::Metrics => render_structured(
            &amlich_api::get_personal_day_metrics(
                &query,
                birth_year,
                birth_month,
                birth_day,
                gender,
            )?,
            args.format,
            args.pretty,
        ),
        InsightSurface::Advisory => render_structured(
            &amlich_api::get_personal_day_advisory(
                &query,
                birth_year,
                birth_month,
                birth_day,
                gender,
            )?,
            args.format,
            args.pretty,
        ),
        InsightSurface::Report => render_structured(
            &amlich_api::get_personal_day_report(
                &query,
                birth_year,
                birth_month,
                birth_day,
                gender,
            )?,
            args.format,
            args.pretty,
        ),
        InsightSurface::Timing => {
            Err("timing surface is not supported for personal-day yet".to_string())
        }
    }
}

fn run_lookup_personal_day_matrix(args: PersonalDayMatrixArgs) -> Result<(), String> {
    let date = parse_date(&args.date)?;
    let profile = crate::profile::load_profile();
    let birth_year = args
        .birth_year
        .or(profile.birth_year)
        .ok_or_else(|| "birth year is required (flag or saved profile)".to_string())?;
    let birth_month = args
        .birth_month
        .or(profile.birth_month)
        .ok_or_else(|| "birth month is required (flag or saved profile)".to_string())?;
    let birth_day = args
        .birth_day
        .or(profile.birth_day)
        .ok_or_else(|| "birth day is required (flag or saved profile)".to_string())?;
    let hour = args
        .hour
        .or(profile.birth_hour)
        .ok_or_else(|| "birth hour is required (flag or saved profile)".to_string())?;
    let minute = args.minute.or(profile.birth_minute).unwrap_or(0);
    let gender = args
        .gender
        .map(|g| match g {
            GenderArg::Male => amlich_core::almanac::tu_menh::Gender::Male,
            GenderArg::Female => amlich_core::almanac::tu_menh::Gender::Female,
        })
        .or_else(|| {
            profile.gender.map(|g| match g {
                crate::profile::ProfileGender::Male => amlich_core::almanac::tu_menh::Gender::Male,
                crate::profile::ProfileGender::Female => {
                    amlich_core::almanac::tu_menh::Gender::Female
                }
            })
        })
        .ok_or_else(|| "gender is required (flag or saved profile)".to_string())?;

    let birth = amlich_api::BaziQuery {
        day: birth_day,
        month: birth_month,
        year: birth_year,
        hour,
        minute,
        timezone: args.timezone,
        longitude: args.longitude,
        use_solar_time: args.use_solar_time,
        gender: Some(match gender {
            amlich_core::almanac::tu_menh::Gender::Male => "male".to_string(),
            amlich_core::almanac::tu_menh::Gender::Female => "female".to_string(),
        }),
    };
    let query = DateQuery {
        day: date.day() as i32,
        month: date.month() as i32,
        year: date.year(),
        timezone: args.timezone,
        ruleset_id: None,
        event_kind: None,
        enabled_pack_ids: vec![],
    };
    let report = amlich_api::get_personal_day_matrix_report(&birth, &query)?;

    match args.format {
        StructuredFormatArg::Json => print_json(&report, args.pretty)?,
        StructuredFormatArg::Text => {
            println!("tier: {:?}", report.tier);
            println!("day-person: {}", report.day_person.day_canchi);
            println!(
                "direction-merge: {}",
                if report.direction_merge.is_some() {
                    "available"
                } else {
                    "unavailable"
                }
            );
        }
    }

    Ok(())
}

fn run_lookup_hour_selection(args: HourSelectionArgs) -> Result<(), String> {
    let date = parse_date(&args.date)?;
    let query = DateQuery {
        day: date.day() as i32,
        month: date.month() as i32,
        year: date.year(),
        timezone: None,
        ruleset_id: None,
        event_kind: None,
        enabled_pack_ids: vec![],
    };

    match InsightSurface::from(args.surface) {
        InsightSurface::Chart => render_structured(
            &amlich_api::get_hour_selection_chart(&query)?,
            args.format,
            args.pretty,
        ),
        InsightSurface::Analysis => render_structured(
            &amlich_api::get_hour_selection_analysis(&query, None, None, None, None)?,
            args.format,
            args.pretty,
        ),
        InsightSurface::Metrics => render_structured(
            &amlich_api::get_hour_selection_metrics(&query)?,
            args.format,
            args.pretty,
        ),
        InsightSurface::Advisory => render_structured(
            &amlich_api::get_hour_selection_advisory(&query, None, None, None, None)?,
            args.format,
            args.pretty,
        ),
        InsightSurface::Report => render_structured(
            &amlich_api::get_hour_selection_report(&query, None, None, None, None)?,
            args.format,
            args.pretty,
        ),
        InsightSurface::Timing => {
            Err("timing surface is not supported for hour-selection yet".to_string())
        }
    }
}

fn render_structured<T: serde::Serialize>(
    value: &T,
    format: StructuredFormatArg,
    pretty: bool,
) -> Result<(), String> {
    match format {
        StructuredFormatArg::Json => print_json(value, pretty),
        StructuredFormatArg::Text => {
            let output = serde_json::to_string_pretty(value)
                .map_err(|e| format!("failed to render text output: {e}"))?;
            println!("{output}");
            if let Ok(json) = serde_json::to_value(value) {
                let tier = json
                    .get("tier")
                    .and_then(|value| value.as_str())
                    .or_else(|| {
                        json.get("chart")
                            .and_then(|chart| chart.get("tier"))
                            .and_then(|value| value.as_str())
                    });
                if let Some(tier) = tier {
                    eprintln!("Note: birth data tier = {tier}");
                }
                let unavailable_items = json
                    .get("unavailable_sections")
                    .and_then(|value| value.as_array())
                    .or_else(|| {
                        json.get("analysis")
                            .and_then(|analysis| analysis.get("unavailable_sections"))
                            .and_then(|value| value.as_array())
                    })
                    .or_else(|| {
                        json.get("computed_metrics")
                            .and_then(|metrics| metrics.get("unavailable_sections"))
                            .and_then(|value| value.as_array())
                    });
                if let Some(items) = unavailable_items {
                    if !items.is_empty() {
                        eprintln!("Unavailable sections:");
                        for item in items {
                            let section = item
                                .get("section")
                                .and_then(|value| value.as_str())
                                .unwrap_or("unknown");
                            let reason = item
                                .get("reason")
                                .and_then(|value| value.as_str())
                                .unwrap_or("missing context");
                            eprintln!("  - {section}: {reason}");
                        }
                    }
                }
            }
            Ok(())
        }
    }
}

fn gender_arg_to_str(value: GenderArg) -> &'static str {
    match value {
        GenderArg::Male => "male",
        GenderArg::Female => "female",
    }
}

fn profile_gender_to_str(value: crate::profile::ProfileGender) -> &'static str {
    match value {
        crate::profile::ProfileGender::Male => "male",
        crate::profile::ProfileGender::Female => "female",
    }
}

fn run_query(args: QueryArgs) -> Result<(), String> {
    let date = args.date.as_deref().map(parse_date).transpose()?;
    let result = query(date, args.format.into(), args.mode, args.pretty)?;
    if let Some(warning) = result.warning {
        eprintln!("Warning: {warning}");
    }
    println!("{}", result.output);
    Ok(())
}

fn run_config(args: ConfigArgs) -> Result<(), String> {
    match args.command {
        ConfigCommand::Mode(mode_args) => match mode_args.command {
            ModeCommand::Show => {
                println!("{}", read_mode());
            }
            ModeCommand::Set { mode } => {
                set_mode(mode)?;
                println!("Mode set to: {mode}");
            }
            ModeCommand::Toggle => {
                let mode = toggle_mode()?;
                println!("Mode set to: {mode}");
            }
        },
        ConfigCommand::Profile(sub) => match sub.command {
            ProfileCommand::Show => {
                let p = crate::profile::load_profile();
                let json = serde_json::to_string_pretty(&p)
                    .map_err(|e| format!("failed to serialize: {e}"))?;
                println!("{json}");
            }
            ProfileCommand::Set {
                birth_year,
                birth_month,
                birth_day,
                birth_hour,
                birth_minute,
                gender,
            } => {
                let mut p = crate::profile::load_profile();
                if let Some(y) = birth_year {
                    p.birth_year = Some(y);
                }
                if let Some(m) = birth_month {
                    p.birth_month = Some(m);
                }
                if let Some(d) = birth_day {
                    p.birth_day = Some(d);
                }
                if let Some(h) = birth_hour {
                    p.birth_hour = Some(h);
                }
                if let Some(m) = birth_minute {
                    p.birth_minute = Some(m);
                }
                if let Some(g) = &gender {
                    p.gender = Some(match g.to_lowercase().as_str() {
                        "male" | "m" => crate::profile::ProfileGender::Male,
                        "female" | "f" => crate::profile::ProfileGender::Female,
                        _ => return Err(format!("invalid gender '{g}'; use male or female")),
                    });
                }
                crate::profile::save_profile(&p)?;
                println!("Profile updated.");
            }
            ProfileCommand::Clear => {
                crate::profile::save_profile(&crate::profile::UserProfile::default())?;
                println!("Profile cleared.");
            }
        },
    }

    Ok(())
}

fn run_auto_mode() -> Result<(), String> {
    if stdin().is_terminal() && stdout().is_terminal() {
        amlich_tui::run_tui(None).map_err(|e| format!("failed to run TUI: {e}"))?;
        return Ok(());
    }

    let args = DayArgs {
        date: None,
        format: DayFormatArg::Json,
        include: vec![
            IncludeArg::Base,
            IncludeArg::Canchi,
            IncludeArg::TietKhi,
            IncludeArg::Hours,
            IncludeArg::Fortune,
        ],
        fields: vec![],
        pretty: false,
        timezone: None,
        ruleset_id: None,
        event_kind: None,
        recommendation_packs: vec![],
    };
    run_day(args)?;
    Ok(())
}

fn rewrite_headless_alias(args: Vec<OsString>) -> Result<Vec<OsString>, String> {
    let contains_headless = args.iter().any(|arg| arg == "--headless");
    if !contains_headless {
        return Ok(args);
    }

    let filtered: Vec<OsString> = args.into_iter().filter(|arg| arg != "--headless").collect();

    let next = filtered
        .get(1)
        .map(|value| value.to_string_lossy().to_string());
    if let Some(value) = next {
        if matches!(
            value.as_str(),
            "query"
                | "day"
                | "range"
                | "convert"
                | "almanac"
                | "insight"
                | "holidays"
                | "tiet-khi"
                | "lookup"
                | "tui"
                | "config"
        ) {
            return Err(
                "--headless cannot be used with subcommands; use `amlich day ...` or `amlich query ...`"
                    .into(),
            );
        }
    }

    let mut rewritten = filtered;
    rewritten.insert(1, OsString::from("query"));
    Ok(rewritten)
}
