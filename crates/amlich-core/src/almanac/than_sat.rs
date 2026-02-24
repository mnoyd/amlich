/// Cát/Sát Tinh Rule Engine
///
/// Builds the set of star rules applicable to a given day and resolves
/// them into final cat/hung lists via the precedence engine in `star.rs`.
///
/// Currently implemented categories:
/// - `FixedByChi` — from `conflict_by_chi` in baseline data (Batch 1 source tag: khcbppt).
///
/// Stub categories (type system in place, no data yet):
/// - `ByYear`, `ByMonth`, `ByTietKhi` — empty until corresponding data is added.

use super::data::baseline_data;
use super::star::{StarCategory, StarQualityTag, StarRule};

/// Build all star rules applicable to a day given its Earthly Branch (chi).
///
/// Returns rules from the `FixedByChi` category sourced from the baseline
/// conflict table. Additional categories will be appended here as data is added.
pub fn get_day_star_rules(day_chi: &str) -> Vec<StarRule> {
    let data = baseline_data();
    let mut rules = Vec::new();

    // --- Category: FixedByChi ---
    // Source: conflict_by_chi table, attributed to conflict_meta.source_id.
    if let Some(conflict) = data.conflict_by_chi.get(day_chi) {
        let source = data.conflict_meta.source_id.clone();
        for name in &conflict.cat_tinh {
            rules.push(StarRule {
                name: name.clone(),
                quality: StarQualityTag::Cat,
                category: StarCategory::FixedByChi,
                source_id: source.clone(),
            });
        }
        for name in &conflict.sat_tinh {
            rules.push(StarRule {
                name: name.clone(),
                quality: StarQualityTag::Hung,
                category: StarCategory::FixedByChi,
                source_id: source.clone(),
            });
        }
    }

    // --- Category stubs: ByYear, ByMonth, ByTietKhi ---
    // No data yet; the category variants are defined and the precedence
    // engine is in place. Rules will be appended here in future batches.

    rules
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::star::{category_priority, resolve_rules, StarCategory, StarQualityTag, StarRule};

    // --- FixedByChi category ---

    #[test]
    fn fixed_by_chi_rules_for_ty() {
        // Tý: cat=["Thiên Đức","Nguyệt Đức"], sat=["Thiên Hình","Chu Tước"]
        let rules = get_day_star_rules("Tý");
        let fixed: Vec<_> = rules
            .iter()
            .filter(|r| r.category == StarCategory::FixedByChi)
            .collect();

        assert!(
            fixed.iter().any(|r| r.name == "Thiên Đức" && r.quality == StarQualityTag::Cat),
            "Thiên Đức must be Cat for Tý"
        );
        assert!(
            fixed.iter().any(|r| r.name == "Nguyệt Đức" && r.quality == StarQualityTag::Cat),
            "Nguyệt Đức must be Cat for Tý"
        );
        assert!(
            fixed.iter().any(|r| r.name == "Thiên Hình" && r.quality == StarQualityTag::Hung),
            "Thiên Hình must be Hung for Tý"
        );
        assert!(
            fixed.iter().any(|r| r.name == "Chu Tước" && r.quality == StarQualityTag::Hung),
            "Chu Tước must be Hung for Tý"
        );
    }

    #[test]
    fn all_12_chi_produce_nonempty_rules() {
        let chi_list = [
            "Tý", "Sửu", "Dần", "Mão", "Thìn", "Tỵ",
            "Ngọ", "Mùi", "Thân", "Dậu", "Tuất", "Hợi",
        ];
        for chi in chi_list {
            let rules = get_day_star_rules(chi);
            assert!(!rules.is_empty(), "{chi}: must produce at least one rule");
            assert!(
                rules.iter().any(|r| r.quality == StarQualityTag::Cat),
                "{chi}: must have at least one Cat rule"
            );
            assert!(
                rules.iter().any(|r| r.quality == StarQualityTag::Hung),
                "{chi}: must have at least one Hung rule"
            );
        }
    }

    #[test]
    fn source_id_is_populated() {
        let rules = get_day_star_rules("Tý");
        for rule in &rules {
            assert!(!rule.source_id.is_empty(), "source_id must not be empty");
        }
    }

    // --- resolve_rules ---

    #[test]
    fn resolve_separates_cat_and_hung() {
        let rules = vec![
            StarRule {
                name: "Good".to_string(),
                quality: StarQualityTag::Cat,
                category: StarCategory::FixedByChi,
                source_id: "test".to_string(),
            },
            StarRule {
                name: "Bad".to_string(),
                quality: StarQualityTag::Hung,
                category: StarCategory::FixedByChi,
                source_id: "test".to_string(),
            },
        ];
        let (cat, hung) = resolve_rules(&rules);
        assert_eq!(cat, vec!["Good"]);
        assert_eq!(hung, vec!["Bad"]);
    }

    #[test]
    fn resolve_excludes_binh() {
        let rules = vec![StarRule {
            name: "Neutral".to_string(),
            quality: StarQualityTag::Binh,
            category: StarCategory::FixedByChi,
            source_id: "test".to_string(),
        }];
        let (cat, hung) = resolve_rules(&rules);
        assert!(cat.is_empty());
        assert!(hung.is_empty());
    }

    // --- Precedence conflict test ---

    #[test]
    fn by_month_overrides_fixed_by_chi() {
        // Same star name: FixedByChi says Cat, ByMonth says Hung.
        // ByMonth has lower priority number → wins.
        let rules = vec![
            StarRule {
                name: "Contested".to_string(),
                quality: StarQualityTag::Cat,
                category: StarCategory::FixedByChi,
                source_id: "base".to_string(),
            },
            StarRule {
                name: "Contested".to_string(),
                quality: StarQualityTag::Hung,
                category: StarCategory::ByMonth,
                source_id: "monthly".to_string(),
            },
        ];
        let (cat, hung) = resolve_rules(&rules);
        assert!(
            hung.contains(&"Contested".to_string()),
            "ByMonth (priority 1) must beat FixedByChi (priority 4)"
        );
        assert!(!cat.contains(&"Contested".to_string()));
    }

    #[test]
    fn by_tiet_khi_overrides_by_month() {
        let rules = vec![
            StarRule {
                name: "SolarStar".to_string(),
                quality: StarQualityTag::Cat,
                category: StarCategory::ByMonth,
                source_id: "m".to_string(),
            },
            StarRule {
                name: "SolarStar".to_string(),
                quality: StarQualityTag::Hung,
                category: StarCategory::ByTietKhi,
                source_id: "tk".to_string(),
            },
        ];
        let (cat, hung) = resolve_rules(&rules);
        assert!(hung.contains(&"SolarStar".to_string()));
        assert!(!cat.contains(&"SolarStar".to_string()));
    }

    // --- category_priority ordering ---

    #[test]
    fn category_priority_ordering() {
        assert!(category_priority(&StarCategory::ByTietKhi) < category_priority(&StarCategory::ByMonth));
        assert!(category_priority(&StarCategory::ByMonth) < category_priority(&StarCategory::ByYear));
        assert!(category_priority(&StarCategory::ByYear) < category_priority(&StarCategory::FixedByCanChi));
        assert!(category_priority(&StarCategory::FixedByCanChi) < category_priority(&StarCategory::FixedByChi));
        assert!(category_priority(&StarCategory::FixedByChi) < category_priority(&StarCategory::JdCycle));
    }

    // --- Integration: resolve full Tý day rules ---

    #[test]
    fn resolve_ty_day_rules() {
        let rules = get_day_star_rules("Tý");
        let (cat, hung) = resolve_rules(&rules);
        assert!(cat.contains(&"Thiên Đức".to_string()));
        assert!(cat.contains(&"Nguyệt Đức".to_string()));
        assert!(hung.contains(&"Thiên Hình".to_string()));
        assert!(hung.contains(&"Chu Tước".to_string()));
    }
}
