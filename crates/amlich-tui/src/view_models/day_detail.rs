use crate::state::{
    AppState, DirectionVerdictVm, DayDetailRiskBoardVm, DayDetailVerdictSupportVm,
};

use super::{
    today::{hero_verdict, risk_summary},
    shared::recommendation_layers,
};

pub fn day_detail_risk_board(app: &AppState) -> DayDetailRiskBoardVm {
    let flat_summary = risk_summary(app);
    let mut critical_items = Vec::new();
    let mut caution_items = Vec::new();
    let mut conflict_items = Vec::new();

    for row in app.top_recommendation_rows() {
        match row.bucket {
            amlich_api::RecommendationBucketDto::KyManh => {
                push_unique(&mut critical_items, format!("Kỵ mạnh: {}", row.label));
            }
            amlich_api::RecommendationBucketDto::Tranh => {
                let label = row
                    .reason_chip
                    .as_ref()
                    .map(|chip| format!("Tránh: {} · {}", row.label, chip))
                    .unwrap_or_else(|| format!("Tránh: {}", row.label));
                push_unique(&mut caution_items, label);
            }
            _ => {}
        }
    }

    if let Some(fortune) = app
        .bundle
        .as_ref()
        .and_then(|bundle| bundle.day_fortune.as_ref())
    {
        for taboo in &fortune.taboos {
            let item = if taboo.reason.trim().is_empty() {
                format!("Kiêng kỵ: {}", taboo.name)
            } else {
                format!("Kiêng kỵ: {} · {}", taboo.name, taboo.reason)
            };

            if taboo.severity.eq_ignore_ascii_case("high") {
                push_unique(&mut critical_items, item);
            } else {
                push_unique(&mut caution_items, item);
            }
        }

        if !fortune.conflict.tuoi_xung.is_empty() {
            push_unique(
                &mut conflict_items,
                format!("Tuổi xung: {}", fortune.conflict.tuoi_xung.join(", ")),
            );
        }
        if !fortune.xung_hop.luc_xung.trim().is_empty() {
            push_unique(
                &mut conflict_items,
                format!("Lục xung: {}", fortune.xung_hop.luc_xung),
            );
        }
        if !fortune.conflict.sat_huong.trim().is_empty() {
            push_unique(
                &mut conflict_items,
                format!("Sát hướng: {}", fortune.conflict.sat_huong),
            );
        }
    }

    let headline = critical_items
        .first()
        .cloned()
        .or_else(|| caution_items.first().cloned())
        .or_else(|| flat_summary.items.first().cloned())
        .or_else(|| conflict_items.first().cloned());

    DayDetailRiskBoardVm {
        headline,
        critical_items,
        caution_items,
        conflict_items,
        notice: app.sensitive_domain_notice(),
    }
}

pub fn day_detail_verdict_support(app: &AppState) -> Option<DayDetailVerdictSupportVm> {
    let bundle = app.bundle.as_ref()?;
    let mut segments = Vec::new();

    if let Some(canchi) = bundle
        .canchi
        .as_ref()
        .map(|canchi| canchi.full.trim())
        .filter(|full: &&str| !full.is_empty())
    {
        segments.push(canchi.to_string());
    }

    if let Some(truc) = bundle
        .day_fortune
        .as_ref()
        .map(|fortune| fortune.truc.name.trim())
    {
        if !truc.is_empty() {
            segments.push(format!("Trực {truc}"));
        }
    }

    if let Some(star_summary) = bundle.day_fortune.as_ref().and_then(primary_star_summary) {
        segments.push(star_summary);
    }

    if let Some(primary_risk) = risk_summary(app)
        .items
        .into_iter()
        .find(|item: &String| item.starts_with("Kiêng kỵ:") || item.starts_with("Kỵ mạnh:"))
    {
        segments.push(primary_risk);
    }

    if segments.is_empty() {
        let verdict = hero_verdict(app)?;
        if let Some(positive) = verdict.strongest_positive {
            segments.push(format!("Nên: {positive}"));
        }
        if let Some(negative) = verdict.strongest_negative {
            segments.push(format!("Tránh: {negative}"));
        }
    }

    if segments.is_empty() {
        return None;
    }

    let layer_note = recommendation_layers(app)
        .first()
        .filter(|layer| layer.kind == crate::state::RecommendationLayerKind::Contextual)
        .map(|layer| format!("Ngữ cảnh ưu tiên: {}", layer.summary));

    Some(DayDetailVerdictSupportVm {
        support_line: segments.join(" · "),
        layer_note,
    })
}

pub fn direction_verdict(app: &AppState) -> Option<DirectionVerdictVm> {
    let bundle = app.bundle.as_ref()?;

    let xuat_hanh = bundle
        .insight
        .as_ref()
        .and_then(|insight| insight.travel.as_ref())
        .map(|travel| travel.xuat_hanh_huong.as_str())
        .or_else(|| {
            bundle
                .day_fortune
                .as_ref()
                .map(|fortune| fortune.travel.xuat_hanh_huong.as_str())
        });
    let hy_than = bundle
        .insight
        .as_ref()
        .and_then(|insight| insight.travel.as_ref())
        .map(|travel| travel.hy_than.as_str())
        .or_else(|| {
            bundle
                .day_fortune
                .as_ref()
                .map(|fortune| fortune.travel.hy_than.as_str())
        });
    let tai_than = bundle
        .insight
        .as_ref()
        .and_then(|insight| insight.travel.as_ref())
        .map(|travel| travel.tai_than.as_str())
        .or_else(|| {
            bundle
                .day_fortune
                .as_ref()
                .map(|fortune| fortune.travel.tai_than.as_str())
        });

    if xuat_hanh.is_none() && hy_than.is_none() && tai_than.is_none() {
        return None;
    }

    let summary = match xuat_hanh {
        Some(direction) if !direction.trim().is_empty() => {
            format!("Nếu cần hành sự, ưu tiên dịch chuyển về {direction}.")
        }
        _ => "Nên lấy hướng và thần vị làm điểm neo khi xuất hành.".to_string(),
    };

    let mut directions = Vec::new();
    if let Some(direction) = xuat_hanh.filter(|value: &&str| !value.trim().is_empty()) {
        directions.push(format!("Xuất hành: {direction}"));
    }
    if let Some(direction) = hy_than.filter(|value: &&str| !value.trim().is_empty()) {
        directions.push(format!("Hỷ Thần: {direction}"));
    }
    if let Some(direction) = tai_than.filter(|value: &&str| !value.trim().is_empty()) {
        directions.push(format!("Tài Thần: {direction}"));
    }

    let deity_context = bundle
        .insight
        .as_ref()
        .and_then(|insight| insight.day_deity.as_ref())
        .map(|deity| {
            let mut segments = vec![format!(
                "{} · {}",
                deity.name, deity.classification_meaning.vi
            )];
            if let Some(meaning) = deity
                .deity_meaning
                .as_ref()
                .map(|meaning| take_first_sentence(&meaning.vi))
                .filter(|meaning: &String| !meaning.is_empty())
            {
                segments.push(meaning);
            }
            segments.join(" · ")
        });

    let note = recommendation_layers(app)
        .first()
        .filter(|layer| layer.kind == crate::state::RecommendationLayerKind::Contextual)
        .map(|layer| format!("Ngữ cảnh đang ưu tiên: {}", layer.profile));

    Some(DirectionVerdictVm {
        summary,
        directions,
        deity_context,
        note,
    })
}

fn primary_star_summary(fortune: &amlich_api::DayFortuneDto) -> Option<String> {
    fortune
        .stars
        .day_star
        .as_ref()
        .map(|star| format!("Sao ngày {}", star.name))
        .or_else(|| {
            fortune
                .stars
                .cat_tinh
                .first()
                .map(|star| format!("Cát tinh {star}"))
        })
        .or_else(|| {
            fortune
                .stars
                .sat_tinh
                .first()
                .map(|star| format!("Hung tinh {star}"))
        })
        .or_else(|| {
            fortune
                .stars
                .matched_rules
        .first()
                .map(|rule| format!("{} · {}", rule.name, rule.quality))
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
