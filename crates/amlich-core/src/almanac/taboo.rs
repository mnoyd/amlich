use super::data::baseline_data;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabooSeverity {
    Hard,
    Soft,
}

impl TabooSeverity {
    fn from_token(value: &str) -> Self {
        match value {
            "hard" => Self::Hard,
            "soft" => Self::Soft,
            _ => panic!("invalid taboo severity token: {value}"),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Hard => "hard",
            Self::Soft => "soft",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabooHit {
    pub rule_id: String,
    pub name: String,
    pub severity: TabooSeverity,
}

pub fn resolve_day_taboos(lunar_day: i32, lunar_month: i32, day_chi: &str) -> Vec<TabooHit> {
    let data = baseline_data();
    let mut hits = Vec::new();

    if let Ok(lunar_day_u8) = u8::try_from(lunar_day) {
        if data
            .taboo_rules
            .tam_nuong
            .lunar_days
            .contains(&lunar_day_u8)
        {
            hits.push(TabooHit {
                rule_id: data.taboo_rules.tam_nuong.rule_id.clone(),
                name: data.taboo_rules.tam_nuong.name.clone(),
                severity: TabooSeverity::from_token(&data.taboo_rules.tam_nuong.severity),
            });
        }

        if data
            .taboo_rules
            .nguyet_ky
            .lunar_days
            .contains(&lunar_day_u8)
        {
            hits.push(TabooHit {
                rule_id: data.taboo_rules.nguyet_ky.rule_id.clone(),
                name: data.taboo_rules.nguyet_ky.name.clone(),
                severity: TabooSeverity::from_token(&data.taboo_rules.nguyet_ky.severity),
            });
        }
    }

    if let Ok(lunar_month_u8) = u8::try_from(lunar_month) {
        if data
            .taboo_rules
            .sat_chu
            .by_lunar_month
            .get(&lunar_month_u8)
            .is_some_and(|chi| chi == day_chi)
        {
            hits.push(TabooHit {
                rule_id: data.taboo_rules.sat_chu.rule_id.clone(),
                name: data.taboo_rules.sat_chu.name.clone(),
                severity: TabooSeverity::from_token(&data.taboo_rules.sat_chu.severity),
            });
        }

        if data
            .taboo_rules
            .tho_tu
            .by_lunar_month
            .get(&lunar_month_u8)
            .is_some_and(|chi| chi == day_chi)
        {
            hits.push(TabooHit {
                rule_id: data.taboo_rules.tho_tu.rule_id.clone(),
                name: data.taboo_rules.tho_tu.name.clone(),
                severity: TabooSeverity::from_token(&data.taboo_rules.tho_tu.severity),
            });
        }
    }

    hits
}

#[cfg(test)]
mod tests {
    use super::{resolve_day_taboos, TabooSeverity};

    #[test]
    fn matches_tam_nuong_by_lunar_day() {
        let hits = resolve_day_taboos(3, 6, "Dậu");
        assert!(hits.iter().any(|hit| hit.rule_id == "tam_nuong"));
    }

    #[test]
    fn matches_nguyet_ky_by_lunar_day() {
        let hits = resolve_day_taboos(14, 4, "Tý");
        assert!(hits.iter().any(|hit| hit.rule_id == "nguyet_ky"));
    }

    #[test]
    fn matches_sat_chu_by_lunar_month_and_day_chi() {
        let hits = resolve_day_taboos(1, 1, "Tỵ");
        assert!(hits.iter().any(|hit| hit.rule_id == "sat_chu"));
    }

    #[test]
    fn matches_tho_tu_by_lunar_month_and_day_chi() {
        let hits = resolve_day_taboos(12, 12, "Mùi");
        assert!(hits.iter().any(|hit| hit.rule_id == "tho_tu"));
    }

    #[test]
    fn emits_expected_severity_per_rule() {
        let hits = resolve_day_taboos(5, 2, "Tý");
        assert!(hits
            .iter()
            .any(|hit| hit.rule_id == "nguyet_ky" && hit.severity == TabooSeverity::Hard));
        assert!(hits
            .iter()
            .any(|hit| hit.rule_id == "sat_chu" && hit.severity == TabooSeverity::Hard));

        let soft = resolve_day_taboos(1, 12, "Mùi");
        assert!(soft
            .iter()
            .any(|hit| hit.rule_id == "tho_tu" && hit.severity == TabooSeverity::Soft));
    }

    #[test]
    fn no_hits_for_non_matching_day_context() {
        let hits = resolve_day_taboos(2, 2, "Dần");
        assert!(hits.is_empty());
    }

    #[test]
    fn emits_deterministic_family_order() {
        let hits = resolve_day_taboos(5, 2, "Tý");
        let ids: Vec<_> = hits.iter().map(|hit| hit.rule_id.as_str()).collect();
        assert_eq!(ids, vec!["nguyet_ky", "sat_chu"]);
    }
}
