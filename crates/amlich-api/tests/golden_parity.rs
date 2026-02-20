use amlich_api::{get_day_info, DateQuery};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    query: Query,
    expected: Expected,
}

#[derive(Debug, Deserialize)]
struct Query {
    day: i32,
    month: i32,
    year: i32,
    timezone: f64,
}

#[derive(Debug, Deserialize)]
struct Expected {
    lunar: ExpectedLunar,
    canchi_day: String,
    canchi_year: String,
}

#[derive(Debug, Deserialize)]
struct ExpectedLunar {
    day: i32,
    month: i32,
    year: i32,
}

#[test]
fn golden_day_info_parity() {
    let fixture_str = include_str!("fixtures/day-info-golden.json");
    let fixtures: Vec<Fixture> = serde_json::from_str(fixture_str).expect("valid fixture json");

    for fixture in fixtures {
        let info = get_day_info(&DateQuery {
            day: fixture.query.day,
            month: fixture.query.month,
            year: fixture.query.year,
            timezone: Some(fixture.query.timezone),
            locale: Some("vi-VN".to_string()),
            verbosity: None,
        })
        .expect("query should be valid");

        assert_eq!(info.lunar.day, fixture.expected.lunar.day);
        assert_eq!(info.lunar.month, fixture.expected.lunar.month);
        assert_eq!(info.lunar.year, fixture.expected.lunar.year);
        assert_eq!(info.canchi.day.full, fixture.expected.canchi_day);
        assert_eq!(info.canchi.year.full, fixture.expected.canchi_year);
    }
}
