use super::data::{baseline_data, StarRuleBucket};
use super::star::{StarCategory, StarQualityTag, StarRule};

pub fn get_day_star_rules(
    day_chi: &str,
    day_canchi_full: &str,
    year_can: &str,
    lunar_month: i32,
    tiet_khi_name: &str,
) -> Vec<StarRule> {
    let data = baseline_data();
    let mut rules = Vec::new();

    if let Some(conflict) = data.conflict_by_chi.get(day_chi) {
        let source = data.star_rule_meta.fixed_by_chi.source_id.clone();
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

    if let Some(bucket) = data.star_rules_fixed_by_canchi.get(day_canchi_full) {
        append_bucket_rules(
            &mut rules,
            bucket,
            StarCategory::FixedByCanChi,
            &data.star_rule_meta.fixed_by_canchi.source_id,
        );
    }

    if let Some(bucket) = data.star_rules_by_year_can.get(year_can) {
        append_bucket_rules(
            &mut rules,
            bucket,
            StarCategory::ByYear,
            &data.star_rule_meta.by_year.source_id,
        );
    }

    if let Ok(month) = u8::try_from(lunar_month) {
        if let Some(bucket) = data.star_rules_by_lunar_month.get(&month) {
            append_bucket_rules(
                &mut rules,
                bucket,
                StarCategory::ByMonth,
                &data.star_rule_meta.by_month.source_id,
            );
        }
    }

    if let Some(bucket) = data.star_rules_by_tiet_khi.get(tiet_khi_name) {
        append_bucket_rules(
            &mut rules,
            bucket,
            StarCategory::ByTietKhi,
            &data.star_rule_meta.by_tiet_khi.source_id,
        );
    }

    rules
}

fn append_bucket_rules(
    out: &mut Vec<StarRule>,
    bucket: &StarRuleBucket,
    category: StarCategory,
    source_id: &str,
) {
    for name in &bucket.cat_tinh {
        out.push(StarRule {
            name: name.clone(),
            quality: StarQualityTag::Cat,
            category: category.clone(),
            source_id: source_id.to_string(),
        });
    }
    for name in &bucket.sat_tinh {
        out.push(StarRule {
            name: name.clone(),
            quality: StarQualityTag::Hung,
            category: category.clone(),
            source_id: source_id.to_string(),
        });
    }
    for name in &bucket.binh_tinh {
        out.push(StarRule {
            name: name.clone(),
            quality: StarQualityTag::Binh,
            category: category.clone(),
            source_id: source_id.to_string(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::super::star::{
        category_priority, resolve_rules, StarCategory, StarQualityTag, StarRule,
    };
    use super::*;

    #[test]
    fn fixed_by_chi_rules_for_ty() {
        let rules = get_day_star_rules("Tý", "Giáp Tý", "Giáp", 1, "Lập Xuân");
        let fixed: Vec<_> = rules
            .iter()
            .filter(|r| r.category == StarCategory::FixedByChi)
            .collect();

        assert!(fixed
            .iter()
            .any(|r| r.name == "Thiên Đức" && r.quality == StarQualityTag::Cat));
        assert!(fixed
            .iter()
            .any(|r| r.name == "Nguyệt Đức" && r.quality == StarQualityTag::Cat));
        assert!(fixed
            .iter()
            .any(|r| r.name == "Thiên Hình" && r.quality == StarQualityTag::Hung));
        assert!(fixed
            .iter()
            .any(|r| r.name == "Chu Tước" && r.quality == StarQualityTag::Hung));
    }

    #[test]
    fn all_12_chi_produce_nonempty_rules() {
        let chi_list = [
            "Tý", "Sửu", "Dần", "Mão", "Thìn", "Tỵ", "Ngọ", "Mùi", "Thân", "Dậu", "Tuất", "Hợi",
        ];
        for chi in chi_list {
            let rules = get_day_star_rules(chi, "Giáp Tý", "Giáp", 1, "Lập Xuân");
            assert!(!rules.is_empty(), "{chi}: must produce at least one rule");
            assert!(rules.iter().any(|r| r.quality == StarQualityTag::Cat));
            assert!(rules.iter().any(|r| r.quality == StarQualityTag::Hung));
        }
    }

    #[test]
    fn source_id_is_populated() {
        let rules = get_day_star_rules("Tý", "Giáp Tý", "Giáp", 1, "Lập Xuân");
        for rule in &rules {
            assert!(!rule.source_id.is_empty(), "source_id must not be empty");
        }
    }

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

    #[test]
    fn by_month_overrides_fixed_by_chi() {
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
        assert!(hung.contains(&"Contested".to_string()));
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

    #[test]
    fn category_priority_ordering() {
        assert!(
            category_priority(&StarCategory::ByTietKhi) < category_priority(&StarCategory::ByMonth)
        );
        assert!(
            category_priority(&StarCategory::ByMonth) < category_priority(&StarCategory::ByYear)
        );
        assert!(
            category_priority(&StarCategory::ByYear)
                < category_priority(&StarCategory::FixedByCanChi)
        );
        assert!(
            category_priority(&StarCategory::FixedByCanChi)
                < category_priority(&StarCategory::FixedByChi)
        );
        assert!(
            category_priority(&StarCategory::FixedByChi)
                < category_priority(&StarCategory::JdCycle)
        );
    }

    #[test]
    fn resolve_ty_day_rules() {
        let rules = get_day_star_rules("Tý", "Giáp Tý", "Giáp", 1, "Lập Xuân");
        let (cat, hung) = resolve_rules(&rules);
        assert!(cat.contains(&"Thiên Đức".to_string()));
        assert!(cat.contains(&"Nguyệt Đức".to_string()));
        assert!(hung.contains(&"Thiên Hình".to_string()));
        assert!(hung.contains(&"Chu Tước".to_string()));
    }

    #[test]
    fn unknown_context_keys_do_not_crash_or_remove_fixed_by_chi() {
        let rules = get_day_star_rules("Tý", "X Y", "Unknown", 99, "Unknown");
        assert!(!rules.is_empty());
        assert!(rules.iter().all(|r| r.category == StarCategory::FixedByChi));
    }

    #[test]
    fn emits_rules_from_all_context_categories_when_data_matches() {
        let rules = get_day_star_rules("Thìn", "Giáp Thìn", "Giáp", 1, "Lập Xuân");
        assert!(rules.iter().any(|r| r.category == StarCategory::FixedByChi));
        assert!(rules
            .iter()
            .any(|r| r.category == StarCategory::FixedByCanChi));
        assert!(rules.iter().any(|r| r.category == StarCategory::ByYear));
        assert!(rules.iter().any(|r| r.category == StarCategory::ByMonth));
        assert!(rules.iter().any(|r| r.category == StarCategory::ByTietKhi));
    }

    #[test]
    fn real_data_precedence_prefers_tiet_khi_then_month_then_year_then_canchi() {
        let rules = get_day_star_rules("Thìn", "Giáp Thìn", "Giáp", 1, "Lập Xuân");
        let (cat, hung) = resolve_rules(&rules);

        // Bạch Hổ appears as Hung in FixedByChi, Cat in ByYear, Hung in ByMonth, Cat in ByTietKhi.
        // Highest precedence ByTietKhi wins => Cat.
        assert!(cat.contains(&"Bạch Hổ".to_string()));
        assert!(!hung.contains(&"Bạch Hổ".to_string()));

        // Thiên Quý appears as Cat in FixedByChi, Hung in ByMonth, Cat in ByTietKhi.
        // ByTietKhi wins => Cat.
        assert!(cat.contains(&"Thiên Quý".to_string()));
        assert!(!hung.contains(&"Thiên Quý".to_string()));

        // Phúc Sinh appears as Cat in FixedByChi, Hung in FixedByCanChi only.
        // FixedByCanChi outranks FixedByChi => Hung.
        assert!(hung.contains(&"Phúc Sinh".to_string()));
        assert!(!cat.contains(&"Phúc Sinh".to_string()));

        // Nguyệt Không only exists as Binh in ByMonth => excluded.
        assert!(!cat.contains(&"Nguyệt Không".to_string()));
        assert!(!hung.contains(&"Nguyệt Không".to_string()));
    }
}
