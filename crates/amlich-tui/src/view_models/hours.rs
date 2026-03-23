use std::collections::BTreeSet;

use crate::state::{AppState, HoursVerdictVm, ScholarTimingSummaryVm};

use super::{dashboard::hero_verdict, shared::format_good_hour_count_summary, shared::format_hour_window};

pub fn scholar_timing_summary(app: &AppState) -> Option<ScholarTimingSummaryVm> {
    let bundle = app.bundle.as_ref()?;
    let gio = bundle.gio_hoang_dao.as_ref();
    let insight_hours = bundle
        .insight
        .as_ref()
        .and_then(|insight| insight.hours.as_ref());

    if gio.is_none() && insight_hours.is_none() {
        return None;
    }

    let summary = gio
        .and_then(|hours| {
            let summary = hours.summary.trim();
            (!summary.is_empty()).then_some(summary.to_string())
        })
        .or_else(|| {
            insight_hours.map(|hours| format_good_hour_count_summary(hours.good_hour_count))
        })
        .or_else(|| gio.map(|hours| format_good_hour_count_summary(hours.good_hour_count)))?;

    let mut windows = Vec::new();
    let mut seen = BTreeSet::new();

    if let Some(hours) = insight_hours {
        for hour in hours.good_hours.iter().take(3) {
            let dedupe_key = format!("{}|{}", hour.chi, hour.time_range);
            if seen.insert(dedupe_key) {
                windows.push(format_hour_window(
                    &hour.chi,
                    &hour.time_range,
                    Some(hour.star.as_str()),
                ));
            }
        }
    }

    if let Some(hours) = gio {
        for hour in &hours.good_hours {
            if windows.len() >= 3 {
                break;
            }

            let dedupe_key = format!("{}|{}", hour.hour_chi, hour.time_range);
            if seen.insert(dedupe_key) {
                windows.push(format_hour_window(
                    &hour.hour_chi,
                    &hour.time_range,
                    Some(hour.star.as_str()),
                ));
            }
        }
    }

    Some(ScholarTimingSummaryVm { summary, windows })
}

pub fn hours_verdict(app: &AppState) -> Option<HoursVerdictVm> {
    let bundle = app.bundle.as_ref()?;
    let timing = scholar_timing_summary(app)?;
    let mut bad_windows = Vec::new();

    if let Some(gio) = &bundle.gio_hoang_dao {
        for hour in gio.all_hours.iter().filter(|hour| !hour.is_good).take(3) {
            bad_windows.push(format_hour_window(
                &hour.hour_chi,
                &hour.time_range,
                Some(hour.star.as_str()),
            ));
        }
    }

    let caution = hero_verdict(app).and_then(|verdict| {
        if timing.windows.is_empty() {
            return None;
        }

        verdict.strongest_negative.map(|negative| {
            format!("Có giờ đẹp để xoay xở, nhưng tổng thể ngày vẫn cần dè chừng: {negative}.")
        })
    });

    Some(HoursVerdictVm {
        summary: timing.summary,
        top_windows: timing.windows,
        caution,
        bad_windows,
    })
}
