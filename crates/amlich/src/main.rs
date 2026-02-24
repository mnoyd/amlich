mod app;
mod bookmark_store;
mod dashboard;
mod date_jump;
mod event;
mod headless;
mod history;
mod search;
mod theme;
mod tui_runtime;
mod ui;
mod waybar;
mod widgets;

use std::ffi::OsString;
use std::io::{stdin, stdout, IsTerminal};

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::headless::{
    parse_date, query, read_mode, set_mode, toggle_mode, DisplayMode, QueryFormat,
};
use crate::tui_runtime::run_tui;

#[derive(Parser)]
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

#[derive(Subcommand)]
enum Command {
    /// Open the interactive terminal UI
    Tui(TuiArgs),
    /// Query date information without launching the TUI
    Query(QueryArgs),
    /// Manage persistent user settings
    Config(ConfigArgs),
}

#[derive(Args)]
struct TuiArgs {
    /// Start TUI focused on a specific date in YYYY-MM-DD format
    #[arg(long, value_name = "DATE")]
    date: Option<String>,
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

#[derive(Args)]
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

#[derive(Args)]
struct ConfigArgs {
    #[command(subcommand)]
    command: ConfigCommand,
}

#[derive(Subcommand)]
enum ConfigCommand {
    Mode(ModeArgs),
}

#[derive(Args)]
struct ModeArgs {
    #[command(subcommand)]
    command: ModeCommand,
}

#[derive(Subcommand)]
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
            run_tui(date).map_err(|e| format!("failed to run TUI: {e}"))?;
        }
        Some(Command::Query(args)) => run_query(args)?,
        Some(Command::Config(args)) => run_config(args)?,
        None => run_auto_mode()?,
    }

    Ok(())
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
    }

    Ok(())
}

fn run_auto_mode() -> Result<(), String> {
    if stdin().is_terminal() && stdout().is_terminal() {
        run_tui(None).map_err(|e| format!("failed to run TUI: {e}"))?;
        return Ok(());
    }

    let mode = read_mode();
    let result = query(None, QueryFormat::Waybar, Some(mode), false)?;
    println!("{}", result.output);
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
        if matches!(value.as_str(), "query" | "tui" | "config") {
            return Err(
                "--headless cannot be used with subcommands; use `amlich query ...`".into(),
            );
        }
    }

    let mut rewritten = filtered;
    rewritten.insert(1, OsString::from("query"));
    Ok(rewritten)
}
