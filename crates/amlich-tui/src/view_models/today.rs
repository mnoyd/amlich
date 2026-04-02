use crate::state::{
    AppState, DayIdentitySummaryVm, HeroVerdictVm, RecommendationRowVm,
    RiskSummaryVm, TraditionalEvidenceSummaryVm,
};

use super::shared::{selected_recommendations, top_recommendation_rows};

pub fn top_rows(app: &AppState) -> Vec<RecommendationRowVm> {
    top_recommendation_rows(app)
}

pub fn hero_verdict(app: &AppState) -> Option<HeroVerdictVm> {
    let recommendations = selected_recommendations(app)?;
    let rows = top_rows(app);
    let strongest_positive = rows
        .iter()
        .find(|row| {
            matches!(
                row.bucket,
                amlich_api::RecommendationBucketDto::Nen
                    | amlich_api::RecommendationBucketDto::CoThe
            )
        })
        .map(|row| row.label.clone());
    let strongest_negative = rows
        .iter()
        .find(|row| row.bucket == amlich_api::RecommendationBucketDto::KyManh)
        .or_else(|| {
            rows.iter()
                .find(|row| row.bucket == amlich_api::RecommendationBucketDto::Tranh)
        })
        .map(|row| row.label.clone());
    let summary = if recommendations.summary_vi.trim().is_empty() {
        strongest_positive
            .clone()
            .or_else(|| strongest_negative.clone())
            .unwrap_or_default()
    } else {
        recommendations.summary_vi.clone()
    };

    Some(HeroVerdictVm {
        summary,
        strongest_positive,
        strongest_negative,
    })
}

pub fn risk_summary(app: &AppState) -> RiskSummaryVm {
    let mut items = Vec::new();
    for row in top_rows(app) {
        if row.bucket == amlich_api::RecommendationBucketDto::KyManh {
            items.push(format!("Kỵ mạnh: {}", row.label));
        }
    }

    if let Some(fortune) = app
        .bundle
        .as_ref()
        .and_then(|bundle| bundle.day_fortune.as_ref())
    {
        for taboo in &fortune.taboos {
            items.push(format!("Kiêng kỵ: {}", taboo.name));
        }
        items.push(format!("Lục xung: {}", fortune.xung_hop.luc_xung));
        items.push(format!("Sát hướng: {}", fortune.conflict.sat_huong));
    }

    RiskSummaryVm { items }
}

pub fn day_identity_summary(app: &AppState) -> Option<DayIdentitySummaryVm> {
    let bundle = app.bundle.as_ref()?;
    let canchi = bundle.canchi.as_ref();
    let fortune = bundle.day_fortune.as_ref();
    let insight = bundle.insight.as_ref();

    if canchi.is_none() && fortune.is_none() && insight.is_none() {
        return None;
    }

    let mut headline_parts = Vec::new();
    if let Some(canchi) = canchi {
        headline_parts.push(canchi.day.full.clone());
    }
    if let Some(fortune) = fortune {
        headline_parts.push(format!(
            "{} · {}",
            fortune.day_element.element, fortune.day_element.na_am
        ));
    }
    let headline = if headline_parts.is_empty() {
        "Khí ngày chưa đủ dữ liệu để luận".to_string()
    } else {
        headline_parts.join(" · ")
    };

    let mut detail_lines = Vec::new();
    if let Some(canchi) = canchi {
        push_unique(
            &mut detail_lines,
            format!(
                "Can chi ngày: {} {} · con giáp {}",
                canchi.day.can, canchi.day.chi, canchi.day.con_giap
            ),
        );
    }
    if let Some(fortune) = fortune {
        push_unique(
            &mut detail_lines,
            format!(
                "Ngũ hành ngày: {} · can {} / chi {}",
                fortune.day_element.element,
                fortune.day_element.can_element,
                fortune.day_element.chi_element
            ),
        );
    }
    if let Some(can_chi_insight) = insight.and_then(|insight| insight.canchi.as_ref()) {
        let element_tone = can_chi_insight
            .element
            .as_ref()
            .map(|element| take_first_sentence(&element.nature.vi))
            .filter(|value: &String| !value.is_empty());
        let can_tone = take_first_sentence(&can_chi_insight.can.nature.vi);
        let chi_tone = take_first_sentence(&can_chi_insight.chi.meaning.vi);
        let mut parts = vec![
            format!("Can {}: {}", can_chi_insight.can.name, can_tone),
            format!("Chi {}: {}", can_chi_insight.chi.name, chi_tone),
        ];
        if let Some(element_tone) = element_tone {
            parts.push(format!("Khí hành: {element_tone}"));
        }
        push_unique(&mut detail_lines, parts.join(" · "));
    }
    if let Some(na_am) = insight.and_then(|insight| insight.na_am.as_ref()) {
        push_unique(
            &mut detail_lines,
            format!(
                "Nạp âm {}: {}",
                na_am.na_am,
                take_first_sentence(&na_am.meaning.vi)
            ),
        );
    }

    let application_note = insight
        .and_then(|insight| insight.day_guidance.as_ref())
        .and_then(|guidance| guidance.good_for.vi.first())
        .map(|value| format!("Ứng dụng: hợp để {value}."))
        .or_else(|| {
            insight
                .and_then(|insight| insight.truc.as_ref())
                .and_then(|truc| truc.good_for.vi.first())
                .map(|value| format!("Ứng dụng: trực này thuận cho {value}."))
        });

    Some(DayIdentitySummaryVm {
        headline,
        detail_lines,
        application_note,
    })
}

pub fn traditional_evidence_summary(app: &AppState) -> Option<TraditionalEvidenceSummaryVm> {
    let bundle = app.bundle.as_ref()?;
    let mut headline_parts = Vec::new();
    let mut positive_signals = Vec::new();
    let mut caution_signals = Vec::new();
    let mut provenance = Vec::new();
    let mut source_notes = Vec::new();

    if let Some(truc) = bundle
        .insight
        .as_ref()
        .and_then(|insight| insight.truc.as_ref())
    {
        headline_parts.push(format!("Trực {} ({})", truc.name, truc.quality));
        let meaning = take_first_sentence(&truc.meaning.vi);
        if !meaning.is_empty() {
            push_unique(&mut positive_signals, format!("Luận trực: {meaning}"));
        }
    } else if let Some(fortune) = &bundle.day_fortune {
        headline_parts.push(format!(
            "Trực {} ({})",
            fortune.truc.name, fortune.truc.quality
        ));
    }

    if let Some(stars) = bundle
        .insight
        .as_ref()
        .and_then(|insight| insight.stars.as_ref())
    {
        if let Some(day_star) = &stars.day_star {
            let quality = stars.day_star_quality.as_deref().unwrap_or("không rõ");
            headline_parts.push(format!("Sao ngày {day_star} ({quality})"));
        }

        for star in stars.cat_tinh.iter().take(3) {
            push_unique(&mut positive_signals, format!("Cát tinh: {star}"));
        }
        for star in stars.sat_tinh.iter().take(3) {
            push_unique(&mut caution_signals, format!("Hung tinh: {star}"));
        }
    }

    if let Some(fortune) = &bundle.day_fortune {
        for rule in fortune.stars.matched_rules.iter().take(4) {
            push_unique(
                &mut provenance,
                format!("{} · {} · {}", rule.name, rule.quality, rule.category),
            );
        }
    }

    if let Some(ruleset) = app.ruleset_catalog.iter().find(|ruleset| {
        bundle.ruleset_id == ruleset.id || bundle.ruleset_id == ruleset.canonical_id
    }) {
        for note in &ruleset.source_notes {
            push_unique(
                &mut source_notes,
                format!("{} · {} · {}", note.family, note.source_id, note.note),
            );
        }
    }

    if headline_parts.is_empty()
        && positive_signals.is_empty()
        && caution_signals.is_empty()
        && provenance.is_empty()
        && source_notes.is_empty()
    {
        return None;
    }

    Some(TraditionalEvidenceSummaryVm {
        headline: (!headline_parts.is_empty()).then_some(headline_parts.join(" · ")),
        positive_signals,
        caution_signals,
        provenance,
        source_notes,
    })
}

fn take_first_sentence(text: &str) -> String {
    text.split(['.', '!', '?'])
        .next()
        .unwrap_or("")
        .trim()
        .to_string()
}

fn push_unique(items: &mut Vec<String>, value: String) {
    if !value.trim().is_empty() && !items.iter().any(|existing| existing == &value) {
        items.push(value);
    }
}
