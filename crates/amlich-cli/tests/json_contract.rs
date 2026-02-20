use serde_json::Value;
use std::process::Command;

fn run_json(date: &str) -> Value {
    let output = Command::new(env!("CARGO_BIN_EXE_amlich"))
        .arg("json")
        .arg(date)
        .output()
        .expect("should run amlich binary");

    assert!(
        output.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    serde_json::from_slice(&output.stdout).expect("output should be valid json")
}

#[test]
fn json_contract_matches_day_info_dto_shape() {
    let value = run_json("2024-02-10");
    let obj = value.as_object().expect("top-level should be an object");

    for key in [
        "solar",
        "lunar",
        "jd",
        "canchi",
        "tiet_khi",
        "gio_hoang_dao",
    ] {
        assert!(obj.contains_key(key), "missing top-level key: {key}");
    }

    let canchi = obj
        .get("canchi")
        .and_then(Value::as_object)
        .expect("canchi should be object");

    assert!(canchi.contains_key("day"));
    assert!(canchi.contains_key("month"));
    assert!(canchi.contains_key("year"));
    assert!(
        canchi.get("day_can").is_none(),
        "legacy key day_can should not exist"
    );
    assert!(
        canchi.get("year_can").is_none(),
        "legacy key year_can should not exist"
    );

    let gio_hoang_dao = obj
        .get("gio_hoang_dao")
        .and_then(Value::as_object)
        .expect("gio_hoang_dao should be object");
    assert!(gio_hoang_dao.contains_key("good_hours"));
    assert!(gio_hoang_dao.contains_key("all_hours"));

    assert!(obj.get("_meta").is_none(), "legacy _meta should not exist");
}
