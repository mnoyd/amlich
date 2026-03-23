use crate::state::{AppState, ProfileAvailabilityVm, SeasonalVerdictVm};

pub fn seasonal_verdict(app: &AppState) -> Option<SeasonalVerdictVm> {
    let bundle = app.bundle.as_ref()?;
    let tiet_khi = bundle.tiet_khi.as_ref()?;
    let insight = bundle
        .insight
        .as_ref()
        .and_then(|insight| insight.tiet_khi.as_ref());

    let headline = format!("{} · mùa {}", tiet_khi.name, tiet_khi.season);
    let implication = insight
        .map(|insight| take_first_sentence(&insight.meaning.vi))
        .filter(|value: &String| !value.is_empty())
        .unwrap_or_else(|| take_first_sentence(&tiet_khi.description));

    let mut application_lines = Vec::new();
    if let Some(insight) = insight {
        let weather = take_first_sentence(&insight.weather.vi);
        if !weather.is_empty() {
            push_unique(&mut application_lines, format!("Thời khí: {weather}"));
        }
        if let Some(item) = insight.agriculture.vi.first() {
            push_unique(&mut application_lines, format!("Nhịp việc mùa này: {item}"));
        }
        if let Some(item) = insight.health.vi.first() {
            push_unique(&mut application_lines, format!("Chăm sóc cơ thể: {item}"));
        }
    }

    Some(SeasonalVerdictVm {
        headline,
        implication,
        application_lines,
    })
}

pub fn profile_availability_summary(app: &AppState) -> Option<ProfileAvailabilityVm> {
    let bundle = app.bundle.as_ref()?;
    let has_personal_overlay = bundle
        .insight
        .as_ref()
        .map(|insight| insight.tu_menh.is_some() || insight.dai_van.is_some())
        .unwrap_or(false);

    let note = if has_personal_overlay {
        "Đã có lớp cá nhân hóa; tách riêng phần ngày chung và phần mệnh cá nhân.".to_string()
    } else {
        "Chưa có hồ sơ cá nhân; màn hình này chỉ nên đọc như hướng theo ngày, không phải phong thủy bản mệnh.".to_string()
    };

    Some(ProfileAvailabilityVm {
        has_personal_overlay,
        note,
    })
}

pub fn sensitive_domain_notice(app: &AppState) -> Option<String> {
    let recommendations = super::shared::selected_recommendations(app)?;
    let has_medical = recommendations
        .activities
        .iter()
        .any(|activity| activity.activity_id == "medical_treatment");
    let has_burial = recommendations
        .activities
        .iter()
        .any(|activity| activity.activity_id == "burial_memorial");

    let mut notes = Vec::new();
    if has_medical {
        notes.push(
            "Lưu ý: điều trị thực tế luôn ưu tiên đánh giá chuyên môn; lịch chỉ mang tính tham khảo."
                .to_string(),
        );
    }
    if has_burial {
        notes.push(
            "Lưu ý: an táng hoặc tưởng niệm cần thẩm định thêm theo tập tục và chuyên gia địa phương."
                .to_string(),
        );
    }

    if notes.is_empty() {
        None
    } else {
        Some(notes.join(" "))
    }
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
