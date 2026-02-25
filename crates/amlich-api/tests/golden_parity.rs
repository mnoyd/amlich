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
    xung_hop_luc_xung: String,
    truc_name: String,
    stars_contains_cat: Option<Vec<String>>,
    stars_contains_sat: Option<Vec<String>>,
    stars_excludes: Option<Vec<String>>,
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
        })
        .expect("query should be valid");

        assert_eq!(info.lunar.day, fixture.expected.lunar.day);
        assert_eq!(info.lunar.month, fixture.expected.lunar.month);
        assert_eq!(info.lunar.year, fixture.expected.lunar.year);
        assert_eq!(info.canchi.day.full, fixture.expected.canchi_day);
        assert_eq!(info.canchi.year.full, fixture.expected.canchi_year);

        let fortune = info.day_fortune.expect("day_fortune must be present");
        assert_eq!(
            fortune.xung_hop.luc_xung,
            fixture.expected.xung_hop_luc_xung
        );
        assert_eq!(fortune.truc.name, fixture.expected.truc_name);

        if let Some(expected) = &fixture.expected.stars_contains_cat {
            for name in expected {
                assert!(
                    fortune.stars.cat_tinh.contains(name),
                    "expected cat_tinh to contain '{name}'"
                );
            }
        }

        if let Some(expected) = &fixture.expected.stars_contains_sat {
            for name in expected {
                assert!(
                    fortune.stars.sat_tinh.contains(name),
                    "expected sat_tinh to contain '{name}'"
                );
            }
        }

        if let Some(excluded) = &fixture.expected.stars_excludes {
            for name in excluded {
                assert!(
                    !fortune.stars.cat_tinh.contains(name),
                    "expected cat_tinh to exclude '{name}'"
                );
                assert!(
                    !fortune.stars.sat_tinh.contains(name),
                    "expected sat_tinh to exclude '{name}'"
                );
            }
        }
    }
}
