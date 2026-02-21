use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

fn temp_home() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time should be monotonic")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("amlich-test-home-{nanos}"));
    fs::create_dir_all(&dir).expect("temp home should be created");
    dir
}

fn run(home: &PathBuf, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_amlich"))
        .args(args)
        .env("HOME", home)
        .output()
        .expect("command should execute")
}

#[test]
fn auto_mode_without_tty_outputs_waybar_json() {
    let home = temp_home();
    let output = run(&home, &[]);
    assert!(
        output.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json: Value =
        serde_json::from_slice(&output.stdout).expect("stdout should be valid waybar json");
    let obj = json.as_object().expect("top-level should be object");
    for key in ["text", "tooltip", "class"] {
        assert!(obj.contains_key(key), "missing key: {key}");
    }
}

#[test]
fn query_default_returns_dayinfo_json() {
    let home = temp_home();
    let output = run(&home, &["query"]);
    assert!(
        output.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json: Value = serde_json::from_slice(&output.stdout).expect("stdout should be valid json");
    let obj = json.as_object().expect("top-level should be object");
    for key in [
        "solar",
        "lunar",
        "jd",
        "canchi",
        "tiet_khi",
        "gio_hoang_dao",
    ] {
        assert!(obj.contains_key(key), "missing key: {key}");
    }
}

#[test]
fn query_specific_date_uses_requested_date() {
    let home = temp_home();
    let output = run(&home, &["query", "2026-02-20"]);
    assert!(
        output.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json: Value = serde_json::from_slice(&output.stdout).expect("stdout should be valid json");
    let solar = json
        .get("solar")
        .and_then(Value::as_object)
        .expect("solar should be an object");

    assert_eq!(solar.get("year").and_then(Value::as_i64), Some(2026));
    assert_eq!(solar.get("month").and_then(Value::as_i64), Some(2));
    assert_eq!(solar.get("day").and_then(Value::as_i64), Some(20));
}

#[test]
fn query_formats_work_and_mode_warning_for_dayinfo_json() {
    let home = temp_home();

    let dayinfo = run(
        &home,
        &[
            "query",
            "2026-02-20",
            "--format",
            "dayinfo-json",
            "--mode",
            "full",
        ],
    );
    assert!(
        dayinfo.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&dayinfo.stderr)
    );
    let stderr = String::from_utf8_lossy(&dayinfo.stderr);
    assert!(
        stderr.contains("--mode is ignored"),
        "expected warning in stderr, got: {stderr}"
    );

    let waybar = run(&home, &["query", "2026-02-20", "--format", "waybar"]);
    assert!(
        waybar.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&waybar.stderr)
    );
    let waybar_json: Value =
        serde_json::from_slice(&waybar.stdout).expect("waybar output should be json");
    let waybar_obj = waybar_json
        .as_object()
        .expect("waybar top-level should be object");
    assert!(waybar_obj.contains_key("text"));
    assert!(waybar_obj.contains_key("tooltip"));
    assert!(waybar_obj.contains_key("class"));

    let text = run(&home, &["query", "2026-02-20", "--format", "text"]);
    assert!(
        text.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&text.stderr)
    );
    let text_line = String::from_utf8_lossy(&text.stdout);
    assert!(text_line.contains("lunar"));
    assert!(text_line.contains("tiet-khi"));
}

#[test]
fn headless_alias_maps_to_query() {
    let home = temp_home();
    let output = run(
        &home,
        &[
            "--headless",
            "2026-02-20",
            "--format",
            "dayinfo-json",
            "--pretty",
        ],
    );
    assert!(
        output.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json: Value = serde_json::from_slice(&output.stdout).expect("stdout should be valid json");
    let solar = json
        .get("solar")
        .and_then(Value::as_object)
        .expect("solar should be object");
    assert_eq!(solar.get("year").and_then(Value::as_i64), Some(2026));
}

#[test]
fn config_mode_show_set_toggle_persists() {
    let home = temp_home();

    let show = run(&home, &["config", "mode", "show"]);
    assert!(show.status.success());
    assert_eq!(String::from_utf8_lossy(&show.stdout).trim(), "full");

    let set = run(&home, &["config", "mode", "set", "minimal"]);
    assert!(set.status.success());

    let show_after_set = run(&home, &["config", "mode", "show"]);
    assert!(show_after_set.status.success());
    assert_eq!(
        String::from_utf8_lossy(&show_after_set.stdout).trim(),
        "minimal"
    );

    let toggle = run(&home, &["config", "mode", "toggle"]);
    assert!(toggle.status.success());

    let show_after_toggle = run(&home, &["config", "mode", "show"]);
    assert!(show_after_toggle.status.success());
    assert_eq!(
        String::from_utf8_lossy(&show_after_toggle.stdout).trim(),
        "full"
    );
}

#[test]
fn invalid_date_returns_error() {
    let home = temp_home();
    let output = run(&home, &["query", "2026-13-99"]);
    assert!(!output.status.success(), "command should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("invalid date"));
}
