use super::{
    render::{Frame, BOLD, CYAN, DIM, RED, RESET, YELLOW},
    state::AppState,
};
use amlich_api::RecommendationBucketDto;
use chrono::Datelike;
use serde_json::Value;
use std::fmt::Write;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Today,
    Personal,
    Hours,
    EventDetail,
    Elements,
    FengShui,
    Insight,
    GraphInspector,
    Help,
}
impl Screen {
    pub fn next(self) -> Self {
        match self {
            Self::Today => Self::Personal,
            Self::Personal => Self::Hours,
            Self::Hours => Self::EventDetail,
            Self::EventDetail => Self::Elements,
            Self::Elements => Self::FengShui,
            Self::FengShui => Self::Insight,
            Self::Insight => Self::GraphInspector,
            Self::GraphInspector | Self::Help => Self::Today,
        }
    }
    pub fn previous(self) -> Self {
        match self {
            Self::Today | Self::Help => Self::GraphInspector,
            Self::Personal => Self::Today,
            Self::Hours => Self::Personal,
            Self::EventDetail => Self::Hours,
            Self::Elements => Self::EventDetail,
            Self::FengShui => Self::Elements,
            Self::Insight => Self::FengShui,
            Self::GraphInspector => Self::Insight,
        }
    }
    fn label(self) -> &'static str {
        match self {
            Self::Today => "Today",
            Self::Personal => "Personal",
            Self::Hours => "Hours",
            Self::EventDetail => "Event",
            Self::Elements => "Elements",
            Self::FengShui => "Feng Shui",
            Self::Insight => "Insight",
            Self::GraphInspector => "Graph",
            Self::Help => "Help",
        }
    }
}

pub fn render(app: &AppState) -> String {
    let mut f = Frame::default();
    tabs(&mut f, app.screen);
    match app.screen {
        Screen::Today => today(&mut f, app),
        Screen::Personal => personal(&mut f, app),
        Screen::Hours => hours(&mut f, app),
        Screen::EventDetail => event(&mut f, app),
        Screen::Elements => elements(&mut f, app),
        Screen::FengShui => feng_shui(&mut f, app),
        Screen::Insight => insight(&mut f, app),
        Screen::GraphInspector => graph(&mut f, app),
        Screen::Help => help(&mut f),
    };
    f.blank();
    f.line(format!(
        "{DIM}[Tab/←/→] screen  [h/l] day  [j/k] scroll  [1-8] jump  [?] help  [q] quit{RESET}"
    ));
    f.finish(app.scroll)
}
fn tabs(f: &mut Frame, current: Screen) {
    let mut line = String::new();
    for (n, screen) in [
        (1, Screen::Today),
        (2, Screen::Personal),
        (3, Screen::Hours),
        (4, Screen::EventDetail),
        (5, Screen::Elements),
        (6, Screen::FengShui),
        (7, Screen::Insight),
        (8, Screen::GraphInspector),
    ] {
        let _ = write!(
            line,
            " {}{} {}:{} {}",
            if screen == current { BOLD } else { DIM },
            n,
            screen.label(),
            if screen == current { "●" } else { "" },
            RESET
        );
    }
    f.line(line);
}
fn foundation(f: &mut Frame, app: &AppState) {
    let b = &app.bundle;
    f.line(format!(
        "{BOLD}{}  | Lunar {} | JD {}{RESET}",
        b.solar.date_string, b.lunar.date_string, b.jd
    ));
    if let Some(c) = &b.canchi {
        f.line(format!(
            "Can Chi: {} · {} · {}",
            c.day.full, c.month.full, c.year.full
        ));
    }
    if let Some(x) = &b.tiet_khi {
        f.line(format!("Tiết khí: {} · mùa {}", x.name, x.season));
    }
}
fn today(f: &mut Frame, app: &AppState) {
    foundation(f, app);
    f.blank();
    f.section("Verdict");
    let rec = app
        .bundle
        .contextual_recommendations
        .as_ref()
        .or(app.bundle.daily_recommendations.as_ref());
    if let Some(r) = rec {
        f.line(format!("{YELLOW}{}{RESET}", r.summary_vi));
        for a in r.activities.iter().take(8) {
            f.line(format!("  {} {}", bucket(a.bucket), a.label.vi));
        }
    } else {
        f.line("No recommendations available.");
    }
    almanac(f, app);
}
fn personal(f: &mut Frame, app: &AppState) {
    foundation(f, app);
    f.blank();
    f.section("Personal assessment");
    match app.bundle.insight.as_ref().and_then(|x|x.tu_menh.as_ref()) { Some(x)=>f.line(format!("Kua {} · {}",x.kua,x.trigram.vi)), None=>f.line("No personal profile is configured. Set one with `amlich config profile set …` to enable personalized guidance."), };
    f.section("Scope");
    f.line("This screen keeps the general-day guidance separate from birth-profile advice.");
}
fn hours(f: &mut Frame, app: &AppState) {
    foundation(f, app);
    f.blank();
    f.section("Auspicious hours");
    if let Some(h) = &app.bundle.gio_hoang_dao {
        f.line(&h.summary);
        for x in &h.good_hours {
            f.line(format!(
                "  {CYAN}●{RESET} {} {} · {}",
                x.hour_chi, x.time_range, x.star
            ));
        }
        f.section("Caution hours");
        for x in h.all_hours.iter().filter(|x| !x.is_good).take(6) {
            f.line(format!(
                "  {RED}●{RESET} {} {} · {}",
                x.hour_chi, x.time_range, x.star
            ));
        }
    }
}
fn event(f: &mut Frame, app: &AppState) {
    foundation(f, app);
    f.blank();
    f.section("Recommendations by activity");
    if let Some(r) = app
        .bundle
        .contextual_recommendations
        .as_ref()
        .or(app.bundle.daily_recommendations.as_ref())
    {
        for a in &r.activities {
            f.line(format!("{} {}", bucket(a.bucket), a.label.vi));
            for reason in a.reasons.iter().take(2) {
                f.line(format!("    {}", reason.summary_vi));
            }
        }
    }
}
fn elements(f: &mut Frame, app: &AppState) {
    foundation(f, app);
    f.blank();
    f.section("Five elements");
    if let Some(x) = &app.bundle.day_fortune {
        f.line(format!(
            "{} · nạp âm {}",
            x.day_element.element, x.day_element.na_am
        ));
        f.line(format!(
            "Can {} / Chi {}",
            x.day_element.can_element, x.day_element.chi_element
        ));
        f.line(format!("Trực {} ({})", x.truc.name, x.truc.quality));
    }
    if let Some(c) = &app.bundle.canchi {
        f.section("Can Chi pillars");
        for (label, x) in [("Day", &c.day), ("Month", &c.month), ("Year", &c.year)] {
            f.line(format!(
                "{label}: {} · {} / {}",
                x.full, x.ngu_hanh.can, x.ngu_hanh.chi
            ));
        }
    }
}
fn feng_shui(f: &mut Frame, app: &AppState) {
    foundation(f, app);
    f.blank();
    f.section("Feng Shui & travel");
    if let Some(x) = &app.bundle.day_fortune {
        f.line(format!("Hỷ thần: {}", x.travel.hy_than));
        f.line(format!("Tài thần: {}", x.travel.tai_than));
        f.line(format!("Xuất hành: {}", x.travel.xuat_hanh_huong));
        f.line(format!("Sát hướng: {}", x.conflict.sat_huong));
    }
}
fn insight(f: &mut Frame, app: &AppState) {
    foundation(f, app);
    f.blank();
    f.section("Cultural insight");
    if let Some(value) = app
        .bundle
        .insight
        .as_ref()
        .and_then(|x| serde_json::to_value(x).ok())
    {
        text_values(f, &value, 0, 36);
    } else {
        f.line("No insight data available.");
    }
}
fn graph(f: &mut Frame, app: &AppState) {
    foundation(f, app);
    f.blank();
    f.section("Semantic reasoning graph");
    let i = amlich_core::debug_inspect_semantic_graph(
        app.date.day() as i32,
        app.date.month() as i32,
        app.date.year(),
        true,
    );
    f.line(format!(
        "Nodes: {} · Edges: {}",
        i.visualization.nodes.len(),
        i.visualization.edges.len()
    ));
    for node in i.visualization.nodes.iter().take(16) {
        f.line(format!("  {} [{}]", node.label, node.semantic_kind));
    }
}
fn help(f: &mut Frame) {
    f.section("Keyboard help");
    f.line("Tab / Shift-Tab / arrows: switch screens");
    f.line("1..8: jump to a screen");
    f.line("h / l: previous / next day");
    f.line("j / k: scroll output");
    f.line("q or Esc: exit");
}
fn almanac(f: &mut Frame, app: &AppState) {
    if let Some(x) = &app.bundle.day_fortune {
        f.blank();
        f.section("Almanac");
        f.line(format!("Trực {} · {}", x.truc.name, x.truc.quality));
        f.line(format!(
            "Lục xung: {} · Sát hướng: {}",
            x.xung_hop.luc_xung, x.conflict.sat_huong
        ));
    }
}
fn bucket(b: RecommendationBucketDto) -> &'static str {
    match b {
        RecommendationBucketDto::Nen => "✓ Nên",
        RecommendationBucketDto::CoThe => "~ Có thể",
        RecommendationBucketDto::Tranh => "! Tránh",
        RecommendationBucketDto::KyManh => "✕ Kỵ mạnh",
    }
}
fn text_values(f: &mut Frame, v: &Value, depth: usize, limit: usize) {
    if depth > 3 || limit == 0 {
        return;
    };
    match v {
        Value::String(s) if !s.trim().is_empty() => f.line(format!("{}{}", "  ".repeat(depth), s)),
        Value::Array(xs) => {
            for x in xs.iter().take(limit) {
                text_values(f, x, depth + 1, limit)
            }
        }
        Value::Object(xs) => {
            for (k, x) in xs
                .iter()
                .filter(|(_, x)| x.is_string() || x.is_array())
                .take(limit)
            {
                f.line(format!("{}{}:", "  ".repeat(depth), k));
                text_values(f, x, depth + 1, limit)
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plain::state::AppState;
    #[test]
    fn every_screen_has_a_title() {
        let mut app =
            AppState::new(Some(chrono::NaiveDate::from_ymd_opt(2026, 8, 10).unwrap())).unwrap();
        for screen in [
            Screen::Today,
            Screen::Personal,
            Screen::Hours,
            Screen::EventDetail,
            Screen::Elements,
            Screen::FengShui,
            Screen::Insight,
            Screen::GraphInspector,
            Screen::Help,
        ] {
            app.screen = screen;
            assert!(!render(&app).is_empty());
        }
    }
}
