/// Star rule type definitions and precedence resolution engine.
///
/// Stars (tinh) in the Vietnamese almanac can come from several sources
/// with different specificity. This module defines the category hierarchy
/// and a deterministic resolver that picks the highest-priority rule
/// when two sources assign conflicting qualities to the same star name.
///
/// **Category precedence (lower index = higher priority):**
/// 0. `ByTietKhi`   — solar-term window override
/// 1. `ByMonth`     — lunar-month keyed
/// 2. `ByYear`      — year-stem keyed
/// 3. `FixedByCanChi` — full sexagenary pair keyed
/// 4. `FixedByChi`  — day's earthly-branch keyed (base rules)
/// 5. `JdCycle`     — Julian Day modular cycle (nhị thập bát tú)

use std::collections::HashMap;

/// Determines which source a star rule comes from.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StarCategory {
    /// Active during a specific solar-term (tiết khí) window.
    ByTietKhi,
    /// Keyed by lunar month number.
    ByMonth,
    /// Keyed by year's Heavenly Stem.
    ByYear,
    /// Keyed by the full sexagenary Can Chi pair.
    FixedByCanChi,
    /// Keyed by the day's Earthly Branch (chi) only — base table.
    FixedByChi,
    /// Derived from the Julian Day modular cycle (nhị thập bát tú).
    JdCycle,
}

/// Auspicious quality tag for a star rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StarQualityTag {
    Cat,
    Hung,
    Binh,
}

/// A single resolved star assignment from one source.
#[derive(Debug, Clone)]
pub struct StarRule {
    pub name: String,
    pub quality: StarQualityTag,
    pub category: StarCategory,
    /// Source identifier, matching `SourceMeta.source_id`.
    pub source_id: String,
}

/// Return the precedence priority for a category.
/// Lower number = higher priority = wins in conflicts.
pub fn category_priority(cat: &StarCategory) -> u8 {
    match cat {
        StarCategory::ByTietKhi => 0,
        StarCategory::ByMonth => 1,
        StarCategory::ByYear => 2,
        StarCategory::FixedByCanChi => 3,
        StarCategory::FixedByChi => 4,
        StarCategory::JdCycle => 5,
    }
}

/// Resolve a list of `StarRule`s into `(cat_list, sat_list)`.
///
/// When two rules name the same star:
/// - The rule with the **lower** `category_priority()` wins.
/// - `Binh` stars are excluded from both output lists.
///
/// Both output lists are sorted for deterministic output.
pub fn resolve_rules(rules: &[StarRule]) -> (Vec<String>, Vec<String>) {
    // For each star name, keep the rule with the lowest priority number.
    let mut by_name: HashMap<&str, &StarRule> = HashMap::new();
    for rule in rules {
        let entry = by_name.entry(rule.name.as_str()).or_insert(rule);
        if category_priority(&rule.category) < category_priority(&entry.category) {
            *entry = rule;
        }
    }

    let mut cat: Vec<String> = Vec::new();
    let mut hung: Vec<String> = Vec::new();
    for rule in by_name.values() {
        match rule.quality {
            StarQualityTag::Cat => cat.push(rule.name.clone()),
            StarQualityTag::Hung => hung.push(rule.name.clone()),
            StarQualityTag::Binh => {}
        }
    }
    cat.sort();
    hung.sort();
    (cat, hung)
}
