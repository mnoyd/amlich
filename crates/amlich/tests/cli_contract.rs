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
fn auto_mode_without_tty_outputs_default_bundle_json() {
    let home = temp_home();
    let output = run(&home, &[]);
    assert!(
        output.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json: Value = serde_json::from_slice(&output.stdout).expect("stdout should be valid json");
    let obj = json.as_object().expect("top-level should be object");
    for key in [
        "schema_version",
        "ruleset_id",
        "ruleset_version",
        "profile",
        "generated_at",
        "solar",
        "lunar",
        "jd",
    ] {
        assert!(obj.contains_key(key), "missing key: {key}");
    }
    assert!(
        output.stderr.is_empty(),
        "stderr should be empty for machine output"
    );
}

#[test]
fn auto_mode_without_tty_matches_explicit_day_json_identity() {
    let home = temp_home();

    let auto = run(&home, &[]);
    assert!(
        auto.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&auto.stderr)
    );
    let auto_json: Value =
        serde_json::from_slice(&auto.stdout).expect("stdout should be valid json");

    let date = auto_json["solar"]["date_string"]
        .as_str()
        .expect("auto payload should expose solar.date_string")
        .to_string();
    let explicit = run(&home, &["day", &date, "--format", "json"]);
    assert!(
        explicit.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&explicit.stderr)
    );
    let explicit_json: Value =
        serde_json::from_slice(&explicit.stdout).expect("stdout should be valid json");

    for key in ["schema_version", "ruleset_id", "ruleset_version", "profile"] {
        assert_eq!(
            auto_json[key], explicit_json[key],
            "default non-tty payload mismatch for key: {key}"
        );
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
    for key in [
        "schema_version",
        "ruleset_id",
        "ruleset_version",
        "profile",
        "generated_at",
        "solar",
        "lunar",
        "jd",
    ] {
        assert!(obj.contains_key(key), "missing key: {key}");
    }
    assert_eq!(json["schema_version"].as_str(), Some("amlich.engine/v1"));
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
    assert!(json.get("schema_version").is_none());
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
fn selector_machine_formats_preserve_context_metadata_and_ndjson_rows_are_self_describing() {
    let home = temp_home();

    let day_output = run(
        &home,
        &[
            "day",
            "2026-02-20",
            "--format",
            "json",
            "--ruleset-id",
            "baseline",
            "--event-kind",
            "contract_signing",
            "--recommendation-packs",
            "pack.nhi_thap_bat_tu.v1",
        ],
    );
    assert!(day_output.status.success());
    let day_json: Value = serde_json::from_slice(&day_output.stdout).expect("valid day json");

    let range_output = run(
        &home,
        &[
            "range",
            "--start",
            "2026-02-20",
            "--end",
            "2026-02-21",
            "--format",
            "json",
            "--include",
            "base,canchi,tiet-khi,hours,fortune",
            "--ruleset-id",
            "baseline",
            "--event-kind",
            "contract_signing",
            "--recommendation-packs",
            "pack.nhi_thap_bat_tu.v1",
        ],
    );
    assert!(range_output.status.success());
    let range_json: Value = serde_json::from_slice(&range_output.stdout).expect("valid range json");

    for key in ["schema_version", "ruleset_id", "ruleset_version", "profile"] {
        assert_eq!(
            range_json[key], day_json[key],
            "range metadata mismatch for key: {key}"
        );
    }

    let day_context = day_json["contextual_recommendations"]
        .as_object()
        .expect("day contextual recommendations");
    let day_active_packs = day_context["active_packs"].clone();
    let range_days = range_json["days"].as_array().expect("range days");
    assert_eq!(range_days.len(), 2);
    for row in range_days {
        for key in ["schema_version", "ruleset_id", "ruleset_version", "profile"] {
            assert_eq!(
                row[key], day_json[key],
                "range row metadata mismatch for key: {key}"
            );
        }
        assert_eq!(
            row["contextual_recommendations"]["ruleset_id"],
            day_json["ruleset_id"]
        );
        assert_eq!(
            row["contextual_recommendations"]["ruleset_version"],
            day_json["ruleset_version"]
        );
        assert_eq!(
            row["contextual_recommendations"]["profile"],
            day_json["profile"]
        );
        assert_eq!(
            row["contextual_recommendations"]["active_packs"], day_active_packs,
            "range row should preserve active pack context"
        );
    }

    let ndjson_output = run(
        &home,
        &[
            "range",
            "--start",
            "2026-02-20",
            "--end",
            "2026-02-21",
            "--format",
            "ndjson",
            "--include",
            "base,canchi,tiet-khi,hours,fortune",
            "--ruleset-id",
            "baseline",
            "--event-kind",
            "contract_signing",
            "--recommendation-packs",
            "pack.nhi_thap_bat_tu.v1",
        ],
    );
    assert!(ndjson_output.status.success());

    let stdout = String::from_utf8_lossy(&ndjson_output.stdout);
    let lines: Vec<&str> = stdout.lines().filter(|l| !l.trim().is_empty()).collect();
    assert_eq!(lines.len(), 2);

    for (index, line) in lines.iter().enumerate() {
        let row: Value = serde_json::from_str(line).expect("line should be valid json");
        for key in [
            "schema_version",
            "ruleset_id",
            "ruleset_version",
            "profile",
            "solar",
        ] {
            assert!(row.get(key).is_some(), "ndjson row missing key: {key}");
        }
        assert_eq!(row["ruleset_id"], day_json["ruleset_id"]);
        assert_eq!(
            row["contextual_recommendations"]["ruleset_id"],
            day_json["ruleset_id"]
        );
        assert_eq!(
            row["contextual_recommendations"]["active_packs"],
            day_active_packs
        );
        assert_eq!(
            row["solar"]["date_string"], range_days[index]["solar"]["date_string"],
            "ndjson row should remain self-describing per date"
        );
    }
}

#[test]
fn day_command_accepts_engine_selectors() {
    let home = temp_home();
    let output = run(
        &home,
        &[
            "day",
            "2026-02-20",
            "--format",
            "json",
            "--ruleset-id",
            "baseline",
            "--event-kind",
            "contract_signing",
            "--recommendation-packs",
            "pack.nhi_thap_bat_tu.v1",
        ],
    );
    assert!(
        output.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json: Value = serde_json::from_slice(&output.stdout).expect("stdout should be valid json");
    assert_eq!(json["ruleset_id"].as_str(), Some("vn_baseline_v1"));
    assert!(json.get("contextual_recommendations").is_some());
    let active_packs = json["contextual_recommendations"]["active_packs"]
        .as_array()
        .expect("active_packs should be an array");
    assert_eq!(active_packs.len(), 1);
    assert_eq!(
        active_packs[0]["pack_id"].as_str(),
        Some("pack.nhi_thap_bat_tu.v1")
    );
}

#[test]
fn alias_backed_selector_identity_stays_canonical_across_day_and_range_outputs() {
    let home = temp_home();

    let day_output = run(
        &home,
        &[
            "day",
            "2026-02-20",
            "--format",
            "json",
            "--ruleset-id",
            "baseline",
            "--event-kind",
            "contract_signing",
            "--recommendation-packs",
            "pack.nhi_thap_bat_tu.v1",
        ],
    );
    assert!(day_output.status.success());
    let day_json: Value = serde_json::from_slice(&day_output.stdout).expect("valid day json");

    let range_output = run(
        &home,
        &[
            "range",
            "--start",
            "2026-02-20",
            "--end",
            "2026-02-20",
            "--format",
            "json",
            "--include",
            "base,canchi,tiet-khi,hours,fortune",
            "--ruleset-id",
            "baseline",
            "--event-kind",
            "contract_signing",
            "--recommendation-packs",
            "pack.nhi_thap_bat_tu.v1",
        ],
    );
    assert!(range_output.status.success());
    let range_json: Value = serde_json::from_slice(&range_output.stdout).expect("valid range json");
    let range_day = range_json["days"].as_array().expect("range days")[0].clone();

    for payload in [&day_json, &range_json, &range_day] {
        assert_eq!(payload["ruleset_id"].as_str(), Some("vn_baseline_v1"));
        assert_eq!(payload["ruleset_version"], day_json["ruleset_version"]);
        assert_eq!(payload["profile"], day_json["profile"]);
    }

    for payload in [&day_json, &range_day] {
        let contextual = payload["contextual_recommendations"]
            .as_object()
            .expect("contextual recommendations");
        assert_eq!(contextual["ruleset_id"].as_str(), Some("vn_baseline_v1"));
        assert_eq!(contextual["profile"], day_json["profile"]);
        let packs = contextual["active_packs"].as_array().expect("active packs");
        assert_eq!(packs.len(), 1);
        assert_eq!(
            packs[0]["pack_id"].as_str(),
            Some("pack.nhi_thap_bat_tu.v1")
        );
    }
}

#[test]
fn range_command_accepts_engine_selectors() {
    let home = temp_home();
    let output = run(
        &home,
        &[
            "range",
            "--start",
            "2026-02-20",
            "--end",
            "2026-02-21",
            "--format",
            "json",
            "--include",
            "base,canchi,tiet-khi,hours,fortune",
            "--ruleset-id",
            "baseline",
            "--event-kind",
            "travel",
        ],
    );
    assert!(
        output.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json: Value = serde_json::from_slice(&output.stdout).expect("stdout should be valid json");
    assert_eq!(json["ruleset_id"].as_str(), Some("vn_baseline_v1"));
    let days = json["days"].as_array().expect("days should be an array");
    assert_eq!(days.len(), 2);
    assert!(days[0].get("contextual_recommendations").is_some());
}

#[test]
fn range_json_is_inclusive_and_matches_day_metadata() {
    let home = temp_home();

    let day_output = run(&home, &["day", "2026-02-20", "--format", "json"]);
    assert!(
        day_output.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&day_output.stderr)
    );
    let day_json: Value =
        serde_json::from_slice(&day_output.stdout).expect("stdout should be valid json");

    let range_output = run(
        &home,
        &[
            "range",
            "--start",
            "2026-02-20",
            "--end",
            "2026-02-22",
            "--format",
            "json",
        ],
    );
    assert!(
        range_output.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&range_output.stderr)
    );

    let range_json: Value =
        serde_json::from_slice(&range_output.stdout).expect("stdout should be valid json");
    assert_eq!(range_json["start"].as_str(), Some("2026-02-20"));
    assert_eq!(range_json["end"].as_str(), Some("2026-02-22"));

    let days = range_json["days"]
        .as_array()
        .expect("days should be an array");
    assert_eq!(days.len(), 3);
    assert_eq!(days[0]["solar"]["date_string"].as_str(), Some("2026-02-20"));
    assert_eq!(days[1]["solar"]["date_string"].as_str(), Some("2026-02-21"));
    assert_eq!(days[2]["solar"]["date_string"].as_str(), Some("2026-02-22"));

    for key in ["schema_version", "ruleset_id", "ruleset_version", "profile"] {
        assert_eq!(
            range_json[key], day_json[key],
            "metadata mismatch for key: {key}"
        );
        assert_eq!(
            days[0][key], day_json[key],
            "day row mismatch for key: {key}"
        );
    }
    assert!(range_json["generated_at"].as_str().is_some());
    assert!(days[0]["generated_at"].as_str().is_some());
}

#[test]
fn single_day_range_output_matches_day_output_for_same_selector_context() {
    let home = temp_home();

    let day_output = run(
        &home,
        &[
            "day",
            "2026-02-20",
            "--format",
            "json",
            "--ruleset-id",
            "baseline",
            "--event-kind",
            "travel",
        ],
    );
    assert!(day_output.status.success());
    let day_json: Value = serde_json::from_slice(&day_output.stdout).expect("valid day json");

    let range_output = run(
        &home,
        &[
            "range",
            "--start",
            "2026-02-20",
            "--end",
            "2026-02-20",
            "--format",
            "json",
            "--include",
            "base,canchi,tiet-khi,hours,fortune",
            "--ruleset-id",
            "baseline",
            "--event-kind",
            "travel",
        ],
    );
    assert!(range_output.status.success());
    let range_json: Value = serde_json::from_slice(&range_output.stdout).expect("valid range json");
    let range_day = range_json["days"].as_array().expect("range days")[0].clone();

    for key in [
        "schema_version",
        "ruleset_id",
        "ruleset_version",
        "profile",
        "solar",
        "lunar",
        "canchi",
        "tiet_khi",
        "gio_hoang_dao",
        "day_fortune",
        "daily_recommendations",
        "contextual_recommendations",
    ] {
        assert_eq!(
            range_day[key], day_json[key],
            "single-day range mismatch for key: {key}"
        );
    }
}

#[test]
fn range_invalid_bounds_fail_explicitly() {
    let home = temp_home();
    let output = run(
        &home,
        &[
            "range",
            "--start",
            "2026-02-22",
            "--end",
            "2026-02-20",
            "--format",
            "json",
        ],
    );

    assert!(!output.status.success(), "command should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("end date must be greater than or equal to start date"));
}

#[test]
fn day_rejects_invalid_include_dependency() {
    let home = temp_home();
    let output = run(
        &home,
        &[
            "day",
            "2026-02-20",
            "--format",
            "json",
            "--include",
            "base,evidence",
        ],
    );

    assert!(!output.status.success(), "command should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("include=evidence requires include=fortune"));
}

#[test]
fn day_rejects_invalid_selector_values() {
    let home = temp_home();

    let bad_ruleset = run(
        &home,
        &[
            "day",
            "2026-02-20",
            "--format",
            "json",
            "--ruleset-id",
            "nope",
        ],
    );
    assert!(!bad_ruleset.status.success(), "command should fail");
    assert!(String::from_utf8_lossy(&bad_ruleset.stderr).contains("unknown almanac ruleset id"));

    let bad_event = run(
        &home,
        &[
            "day",
            "2026-02-20",
            "--format",
            "json",
            "--event-kind",
            "party",
        ],
    );
    assert!(!bad_event.status.success(), "command should fail");
    assert!(String::from_utf8_lossy(&bad_event.stderr)
        .contains("unsupported recommendation event_kind"));
}

#[test]
fn invalid_selector_failures_keep_machine_stdout_clean() {
    let home = temp_home();

    let bad_ruleset = run(
        &home,
        &[
            "day",
            "2026-02-20",
            "--format",
            "json",
            "--ruleset-id",
            "nope",
        ],
    );
    assert!(!bad_ruleset.status.success());
    assert!(
        bad_ruleset.stdout.is_empty(),
        "stdout should be empty on selector failure"
    );
    assert!(String::from_utf8_lossy(&bad_ruleset.stderr).contains("unknown almanac ruleset id"));

    let bad_range = run(
        &home,
        &[
            "range",
            "--start",
            "2026-02-20",
            "--end",
            "2026-02-21",
            "--format",
            "ndjson",
            "--event-kind",
            "party",
        ],
    );
    assert!(!bad_range.status.success());
    assert!(
        bad_range.stdout.is_empty(),
        "ndjson stdout should be empty on selector failure"
    );
    assert!(String::from_utf8_lossy(&bad_range.stderr)
        .contains("unsupported recommendation event_kind"));
}

#[test]
fn machine_output_warnings_stay_on_stderr() {
    let home = temp_home();
    let output = run(
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
    assert!(output.status.success());
    let _: Value =
        serde_json::from_slice(&output.stdout).expect("stdout should remain valid machine json");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("Warning:") && !stdout.contains("deprecated"),
        "machine stdout should not include warnings"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("deprecated"));
    assert!(stderr.contains("--mode is ignored"));
}

#[test]
fn day_text_and_waybar_overlap_with_json_output() {
    let home = temp_home();

    let json_output = run(&home, &["day", "2026-02-20", "--format", "json"]);
    assert!(json_output.status.success());
    let json: Value = serde_json::from_slice(&json_output.stdout).expect("valid json");

    let text_output = run(&home, &["day", "2026-02-20", "--format", "text"]);
    assert!(text_output.status.success());
    let text = String::from_utf8_lossy(&text_output.stdout);
    assert!(text.contains(json["solar"]["date_string"].as_str().expect("date string")));
    assert!(text.contains(json["lunar"]["date_string"].as_str().expect("lunar date")));
    assert!(text.contains(json["tiet_khi"]["name"].as_str().expect("tiet khi name")));

    let waybar_output = run(&home, &["day", "2026-02-20", "--format", "waybar"]);
    assert!(waybar_output.status.success());
    let waybar_json: Value = serde_json::from_slice(&waybar_output.stdout).expect("valid json");
    let text_field = waybar_json["text"].as_str().expect("waybar text field");
    let tooltip = waybar_json["tooltip"]
        .as_str()
        .expect("waybar tooltip field");
    assert!(text_field.contains(
        json["lunar"]["day"]
            .as_i64()
            .expect("lunar day")
            .to_string()
            .as_str()
    ));
    assert!(tooltip.contains(json["solar"]["date_string"].as_str().expect("date string")));
    assert!(tooltip.contains(json["canchi"]["day"]["full"].as_str().expect("canchi day")));
}

#[test]
fn lookup_catalog_commands_return_expected_shapes() {
    let home = temp_home();

    let rulesets = run(&home, &["lookup", "rulesets", "--format", "json"]);
    assert!(rulesets.status.success());
    let rulesets_json: Value =
        serde_json::from_slice(&rulesets.stdout).expect("valid rulesets json");
    let rulesets = rulesets_json
        .as_array()
        .expect("rulesets should be an array");
    assert!(!rulesets.is_empty());
    assert_eq!(rulesets[0]["id"].as_str(), Some("vn_baseline_v1"));
    assert!(rulesets[0]["aliases"].is_array());
    assert_eq!(rulesets[0]["is_default"].as_bool(), Some(true));

    let packs = run(
        &home,
        &["lookup", "recommendation-packs", "--format", "json"],
    );
    assert!(packs.status.success());
    let packs_json: Value = serde_json::from_slice(&packs.stdout).expect("valid packs json");
    let packs = packs_json.as_array().expect("packs should be an array");
    assert!(!packs.is_empty());
    assert_eq!(
        packs[0]["pack_id"].as_str(),
        Some("pack.nhi_thap_bat_tu.v1")
    );
    assert!(packs[0].get("mode").is_some());
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
fn lookup_bazi_report_outputs_machine_readable_payload() {
    let home = temp_home();
    let output = run(
        &home,
        &[
            "lookup",
            "bazi",
            "2024-02-10",
            "--hour",
            "9",
            "--gender",
            "male",
            "--surface",
            "report",
            "--format",
            "json",
        ],
    );
    assert!(
        output.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json: Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert!(json.get("chart").is_some());
    assert!(json.get("analysis").is_some());
    assert!(json.get("computed_metrics").is_some());
    assert!(json.get("advisory").is_some());
}

#[test]
fn lookup_bazi_timing_can_use_profile_birth_year_and_gender() {
    let home = temp_home();
    let set = run(
        &home,
        &[
            "config",
            "profile",
            "set",
            "--birth-year",
            "1990",
            "--gender",
            "male",
        ],
    );
    assert!(set.status.success());

    let output = run(
        &home,
        &[
            "lookup",
            "bazi",
            "2024-02-10",
            "--hour",
            "9",
            "--target-year",
            "2027",
            "--months",
            "1,2",
            "--surface",
            "timing",
            "--format",
            "json",
        ],
    );
    assert!(
        output.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json: Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert!(json.get("annual").is_some());
    assert_eq!(json["monthly"].as_array().map(Vec::len), Some(2));
}

#[test]
fn lookup_personal_day_report_outputs_machine_readable_payload() {
    let home = temp_home();
    let output = run(
        &home,
        &[
            "lookup",
            "personal-day",
            "2024-02-10",
            "--birth-year",
            "1990",
            "--gender",
            "male",
            "--surface",
            "report",
            "--format",
            "json",
        ],
    );
    assert!(
        output.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json: Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert!(json.get("chart").is_some());
    assert!(json.get("analysis").is_some());
    assert!(json.get("computed_metrics").is_some());
    assert!(json.get("advisory").is_some());
}

#[test]
fn lookup_hour_selection_report_outputs_machine_readable_payload() {
    let home = temp_home();
    let output = run(
        &home,
        &[
            "lookup",
            "hour-selection",
            "2024-02-10",
            "--surface",
            "report",
            "--format",
            "json",
        ],
    );
    assert!(
        output.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json: Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert!(json.get("chart").is_some());
    assert!(json.get("analysis").is_some());
    assert!(json.get("computed_metrics").is_some());
    assert!(json.get("advisory").is_some());
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
            "config",
            "profile",
            "set",
            "--birth-year",
            "1990",
            "--gender",
            "male",
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
