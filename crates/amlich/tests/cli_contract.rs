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
        "day_fortune",
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
fn day_fortune_json_contains_xung_hop_and_truc() {
    let home = temp_home();
    let output = run(&home, &["query", "2024-02-10"]);
    assert!(
        output.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json: Value = serde_json::from_slice(&output.stdout).expect("stdout should be valid json");
    let fortune = json
        .get("day_fortune")
        .and_then(Value::as_object)
        .expect("day_fortune should be an object");

    assert!(
        fortune.contains_key("xung_hop"),
        "day_fortune missing xung_hop"
    );
    assert!(fortune.contains_key("truc"), "day_fortune missing truc");

    let xung_hop = fortune
        .get("xung_hop")
        .and_then(Value::as_object)
        .expect("xung_hop should be an object");
    assert!(
        xung_hop.contains_key("luc_xung"),
        "xung_hop missing luc_xung"
    );
    assert!(xung_hop.contains_key("tam_hop"), "xung_hop missing tam_hop");
    assert!(
        xung_hop.contains_key("tu_hanh_xung"),
        "xung_hop missing tu_hanh_xung"
    );

    let truc = fortune
        .get("truc")
        .and_then(Value::as_object)
        .expect("truc should be an object");
    assert!(truc.contains_key("name"), "truc missing name");
    assert!(truc.contains_key("index"), "truc missing index");
    assert!(truc.contains_key("quality"), "truc missing quality");
}

#[test]
fn day_fortune_json_applies_seeded_star_precedence() {
    let home = temp_home();
    let output = run(&home, &["query", "2024-02-10"]);
    assert!(
        output.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json: Value = serde_json::from_slice(&output.stdout).expect("stdout should be valid json");
    let fortune = json
        .get("day_fortune")
        .and_then(Value::as_object)
        .expect("day_fortune should be an object");
    let stars = fortune
        .get("stars")
        .and_then(Value::as_object)
        .expect("stars should be an object");

    let cat_tinh = stars
        .get("cat_tinh")
        .and_then(Value::as_array)
        .expect("cat_tinh should be an array");
    let sat_tinh = stars
        .get("sat_tinh")
        .and_then(Value::as_array)
        .expect("sat_tinh should be an array");

    let has_cat = |name: &str| cat_tinh.iter().any(|v| v.as_str() == Some(name));
    let has_sat = |name: &str| sat_tinh.iter().any(|v| v.as_str() == Some(name));

    assert!(has_cat("Bạch Hổ"));
    assert!(!has_sat("Bạch Hổ"));

    assert!(has_cat("Thiên Quý"));
    assert!(!has_sat("Thiên Quý"));

    assert!(has_sat("Phúc Sinh"));
    assert!(!has_cat("Phúc Sinh"));

    assert!(!has_cat("Nguyệt Không"));
    assert!(!has_sat("Nguyệt Không"));
}

#[test]
fn invalid_date_returns_error() {
    let home = temp_home();
    let output = run(&home, &["query", "2026-13-99"]);
    assert!(!output.status.success(), "command should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("invalid date"));
}

#[test]
fn day_command_returns_v2_bundle_json() {
    let home = temp_home();
    let output = run(&home, &["day", "2026-02-20", "--format", "json"]);
    assert!(
        output.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json: Value = serde_json::from_slice(&output.stdout).expect("stdout should be valid json");
    let obj = json.as_object().expect("top-level should be object");
    for key in ["meta", "solar", "lunar", "jd"] {
        assert!(obj.contains_key(key), "missing key: {key}");
    }
    assert_eq!(
        json["meta"]["schema_version"].as_str(),
        Some("amlich.api/v2")
    );
}

#[test]
fn day_projection_fields_filters_output() {
    let home = temp_home();
    let output = run(
        &home,
        &[
            "day",
            "2026-02-20",
            "--format",
            "json",
            "--fields",
            "solar.date_string,lunar.date_string",
        ],
    );
    assert!(
        output.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json: Value = serde_json::from_slice(&output.stdout).expect("stdout should be valid json");
    assert!(json.get("solar").is_some());
    assert!(json.get("lunar").is_some());
    assert!(json.get("meta").is_none());
    assert!(json["solar"].get("day").is_none());
    assert!(json["solar"].get("date_string").is_some());
}

#[test]
fn range_ndjson_emits_one_object_per_day() {
    let home = temp_home();
    let output = run(
        &home,
        &[
            "range",
            "--start",
            "2026-02-20",
            "--end",
            "2026-02-22",
            "--format",
            "ndjson",
        ],
    );
    assert!(
        output.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().filter(|l| !l.trim().is_empty()).collect();
    assert_eq!(
        lines.len(),
        3,
        "expected 3 ndjson rows, got {}",
        lines.len()
    );
    for line in lines {
        let row: Value = serde_json::from_str(line).expect("line should be json object");
        assert!(row.get("solar").is_some());
    }
}

#[test]
fn lookup_commands_return_expected_shapes() {
    let home = temp_home();

    let na_am = run(
        &home,
        &["lookup", "na-am", "--index", "1", "--format", "json"],
    );
    assert!(na_am.status.success());
    let na_am_json: Value = serde_json::from_slice(&na_am.stdout).expect("valid na-am json");
    assert!(na_am_json.get("Success").is_some());

    let ten_gods = run(
        &home,
        &[
            "lookup",
            "ten-gods",
            "--day-can",
            "Giáp",
            "--target-can",
            "Ất",
            "--format",
            "json",
        ],
    );
    assert!(ten_gods.status.success());
    let tg_json: Value = serde_json::from_slice(&ten_gods.stdout).expect("valid ten-gods json");
    assert!(tg_json.get("label").is_some());
    assert!(tg_json.get("relation").is_some());

    let kua = run(
        &home,
        &[
            "lookup",
            "kua",
            "--birth-year",
            "1990",
            "--gender",
            "male",
            "--format",
            "json",
        ],
    );
    assert!(kua.status.success());
    let kua_json: Value = serde_json::from_slice(&kua.stdout).expect("valid kua json");
    assert!(kua_json.get("kua").is_some());
    assert!(kua_json.get("group").is_some());
}

#[test]
fn config_profile_show_succeeds() {
    let home = temp_home();
    let output = run(&home, &["config", "profile", "show"]);
    assert!(
        output.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json: Value =
        serde_json::from_slice(&output.stdout).expect("profile show should output json");
    assert!(json.is_object());
}

#[test]
fn config_profile_set_and_clear_roundtrip() {
    let home = temp_home();

    let set = run(
        &home,
        &[
            "config", "profile", "set",
            "--birth-year", "1990",
            "--gender", "male",
        ],
    );
    assert!(set.status.success());

    let show = run(&home, &["config", "profile", "show"]);
    assert!(show.status.success());
    let json: Value = serde_json::from_slice(&show.stdout).expect("valid json");
    assert_eq!(json["birth_year"].as_i64(), Some(1990));
    assert_eq!(json["gender"].as_str(), Some("male"));

    let clear = run(&home, &["config", "profile", "clear"]);
    assert!(clear.status.success());

    let show2 = run(&home, &["config", "profile", "show"]);
    assert!(show2.status.success());
    let json2: Value = serde_json::from_slice(&show2.stdout).expect("valid json");
    assert!(json2.get("birth_year").is_none() || json2["birth_year"].is_null());
}

#[test]
fn query_command_prints_deprecation_warning() {
    let home = temp_home();
    let output = run(&home, &["query", "2026-02-20"]);
    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("deprecated"));
}
