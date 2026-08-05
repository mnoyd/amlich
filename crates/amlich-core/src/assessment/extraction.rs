//! Shared input resolution and v2 feature extraction for the personal-day
//! assessment pipeline.
//!
//! Two responsibilities live here:
//!
//! 1. [`resolve_assessment_inputs`] — pure resolution of the optional
//!    upstream signals (Bazi chart, analysis, yearly Hạn, Kua, daily
//!    recommendations) from a `(snapshot, profile, capability, inputs)`
//!    quadruple. Both the legacy v1 builder and the v2 policy go through
//!    this seam so standalone and aggregate calls share identical
//!    normalized inputs (the amlich-mwbp.6 parity contract).
//!
//! 2. [`extract_features`] — typed projection of the resolved signals into
//!    stable [`FeatureObservation`] values consumed by the v2
//!    [`crate::assessment::AssessmentPolicy`]. Extraction is deterministic
//!    and never invents domain coefficients: every observation carries the
//!    legacy strength so `baseline_v2` reproduces v1 axis scores exactly.
//!    Capability-gated features that the current profile cannot support
//!    are emitted as explicit *unavailable* observations so the trace can
//!    explain what evidence was missing (amlich-l0wu).
//!
//! 3. [`extract_vetoes`] — named, source-attributed hard veto events
//!    declared from the same source facts. Vetoes are separate from
//!    weighted contributions: a veto forces the `Avoid` bucket with
//!    deterministic precedence regardless of how favorable the weighted
//!    axes are (amlich-l0wu).

use crate::{
    advisory::ConsultationIntent,
    almanac::{
        recommendation::{DailyRecommendations, RecommendationBucket},
        tu_menh::{compute_kua, KuaResult},
        yearly_han::{compute_yearly_han, HanSeverity, YearlyHanAssessment, YearlyHanInput},
    },
    assessment::{
        feature::{AssessmentFeatureId, FeatureObservation},
        trace::VetoEvent,
        AssessmentAxis, AssessmentInputs, ContributionPolarity, SourceEvidence,
    },
    bazi::{
        analysis::{analyze_bazi_chart, BaziAnalysisReport},
        chart::build_bazi_chart,
        types::BaziChart,
    },
    birth::{BirthCapability, BirthProfile},
    canchi::get_year_canchi,
    lunar::convert_solar_to_lunar,
    sources::{SOURCE_KHCBPPT, SOURCE_VN_FOLK},
    types::VIETNAM_TIMEZONE,
    DaySnapshot,
};

/// Resolved upstream signals shared by the v1 builder and the v2 policy.
///
/// Construction is pure: identical `(snapshot, profile, inputs)` triples
/// produce identical resolved inputs. This is the amlich-mwbp.6 parity
/// prerequisite — standalone and aggregate paths feed byte-identical
/// resolved inputs into the assessment.
#[derive(Debug, Clone)]
pub(super) struct ResolvedAssessmentInputs {
    pub chart: Option<BaziChart>,
    pub analysis: Option<BaziAnalysisReport>,
    pub yearly_han: Option<YearlyHanAssessment>,
    pub kua: Option<KuaResult>,
    pub recommendations: Option<DailyRecommendations>,
}

/// Pure input resolution shared by v1 and v2. Mirrors the legacy
/// `PersonalDayAssessmentBuilder::build` resolution block exactly so v1
/// behavior is unchanged; lifting it here lets the v2 policy reuse the
/// same seam without duplicating capability gates or Hạn/Kua computation.
pub(super) fn resolve_assessment_inputs(
    snapshot: &DaySnapshot,
    profile: &BirthProfile,
    capability: BirthCapability,
    inputs: AssessmentInputs,
) -> ResolvedAssessmentInputs {
    let chart: Option<BaziChart> = match inputs.chart {
        Some(Ok(c)) => Some(c),
        Some(Err(_)) => None,
        None => {
            if capability.has_time {
                build_bazi_chart(bazi_input_from_profile(profile)).ok()
            } else {
                None
            }
        }
    };
    let chart = chart.filter(|c| c.input.time_known);

    let analysis: Option<BaziAnalysisReport> = match inputs.analysis {
        Some(Ok(a)) => Some(a),
        Some(Err(_)) => None,
        None => chart.as_ref().map(analyze_bazi_chart),
    };

    let yearly_han: Option<YearlyHanAssessment> = match inputs.yearly_han {
        Some(Ok(h)) => Some(h),
        Some(Err(_)) => None,
        None => profile.gender.map(|gender| {
            let birth_lunar_year =
                convert_solar_to_lunar(profile.day, profile.month, profile.year, profile.timezone)
                    .year;
            let current_lunar_year = convert_solar_to_lunar(
                snapshot.context.solar.day,
                snapshot.context.solar.month,
                snapshot.context.solar.year,
                VIETNAM_TIMEZONE,
            )
            .year;
            let birth_year_chi = get_year_canchi(birth_lunar_year).chi_index;
            let current_year_chi = snapshot.context.canchi.year.chi_index;
            compute_yearly_han(
                &YearlyHanInput {
                    birth_lunar_year,
                    current_lunar_year,
                    gender,
                },
                birth_year_chi,
                current_year_chi,
            )
        }),
    };

    let kua: Option<KuaResult> = match inputs.kua {
        Some(Ok(k)) => Some(k),
        Some(Err(_)) => None,
        None => profile
            .gender
            .map(|gender| compute_kua(profile.year, gender)),
    };

    let recommendations: Option<DailyRecommendations> = match inputs.recommendations {
        Some(Ok(r)) => Some(r),
        Some(Err(_)) => None,
        None => snapshot
            .contextual_recommendations
            .clone()
            .or_else(|| Some(snapshot.daily_recommendations.clone())),
    };

    ResolvedAssessmentInputs {
        chart,
        analysis,
        yearly_han,
        kua,
        recommendations,
    }
}

pub(super) fn bazi_input_from_profile(profile: &BirthProfile) -> crate::bazi::types::BaziInput {
    let (hour, minute) = profile.time.map(|t| (t.hour, t.minute)).unwrap_or((0, 0));
    crate::bazi::types::BaziInput {
        day: profile.day,
        month: profile.month,
        year: profile.year,
        hour,
        minute,
        time_known: profile.time.is_some(),
        timezone: profile.timezone,
        longitude: profile.longitude,
        use_solar_time: profile.use_solar_time,
        gender: profile.gender,
    }
}

/// Extract typed, source-attributed feature observations from resolved
/// inputs.
///
/// The extraction is the v2 replacement for the legacy v1 contribution
/// builder. It emits one [`FeatureObservation`] per stable feature
/// identifier that the resolved inputs actually warrant, plus explicit
/// *unavailable* observations for features the current profile cannot
/// support. Unavailable observations are projected to `signed_value ==
/// None` so the policy aggregation can exclude them from the denominator
/// (the amlich-7bm4 "unavailable is distinct from zero" contract).
///
/// Strengths here mirror the legacy v1 strengths exactly so `baseline_v2`
/// reproduces v1 axis scores; the v2 policy versions them via
/// [`crate::assessment::ASSESSMENT_POLICY_V2_VERSION`] and future issues
/// (`amlich-lxu3`, `amlich-47wn`) layer new weights and interactions on
/// top without rewriting extraction.
pub(super) fn extract_features(
    snapshot: &DaySnapshot,
    profile: &BirthProfile,
    intent: ConsultationIntent,
    capability: BirthCapability,
    resolved: &ResolvedAssessmentInputs,
) -> Vec<FeatureObservation> {
    let ruleset_id = snapshot.ruleset_id.clone();
    let ruleset_version = snapshot.ruleset_version.clone();
    let profile_id = snapshot.profile.clone();

    let almanac_evidence = |method: &'static str, note: Option<String>| SourceEvidence {
        source_family: "almanac_rule".to_string(),
        source_id: SOURCE_KHCBPPT.to_string(),
        method: method.to_string(),
        profile: profile_id.clone(),
        note,
    };
    let interaction_evidence = |method: &'static str, note: Option<String>| SourceEvidence {
        source_family: "interaction".to_string(),
        source_id: SOURCE_KHCBPPT.to_string(),
        method: method.to_string(),
        profile: profile_id.clone(),
        note,
    };

    let mut features: Vec<FeatureObservation> = Vec::new();

    // --- Generic day quality + intent fit (recommendation-driven) -------
    if let Some(rec) = resolved.recommendations.as_ref() {
        for activity in &rec.activities {
            let (polarity, strength) = recommendation_polarity_strength(activity.bucket, false);
            features.push(
                FeatureObservation::observed(
                    // Each recommendation contributes to generic day quality.
                    // The primary activity's bucket is also reflected in
                    // IntentFit below; non-primary activities stay here.
                    AssessmentFeatureId::GenericDayQuality,
                    polarity,
                    strength,
                    format!(
                        "rec.{}.{}",
                        activity.activity_id.as_str(),
                        snapshot.context.solar.day
                    ),
                    almanac_evidence("recommendation_synthesis", None),
                    rec.ruleset_id.clone(),
                    rec.ruleset_version.clone(),
                )
                .with_note(activity.label.vi.clone()),
            );
        }

        if let Some(primary) = rec
            .activities
            .iter()
            .find(|a| a.activity_id == intent.primary_activity())
        {
            let (polarity, strength) = recommendation_polarity_strength(primary.bucket, true);
            features.push(
                FeatureObservation::observed(
                    AssessmentFeatureId::IntentFit,
                    polarity,
                    strength,
                    format!("intent.{}", intent.event_kind()),
                    almanac_evidence(
                        "intent_fit_lookup",
                        Some(format!("intent={}", intent.event_kind())),
                    ),
                    rec.ruleset_id.clone(),
                    rec.ruleset_version.clone(),
                )
                .with_note(primary.label.vi.clone()),
            );
        }
    }

    // --- Generic day quality: day-fortune taboos -----------------------
    if !snapshot.day_fortune.taboos.is_empty() {
        let taboo_count = snapshot.day_fortune.taboos.len();
        let strength = (taboo_count.min(3) as f32) / 3.0 * 0.6 + 0.2;
        features.push(FeatureObservation::observed(
            AssessmentFeatureId::GenericDayQuality,
            ContributionPolarity::Avoid,
            strength,
            format!("day_fortune.taboo.{}", snapshot.context.solar.day),
            almanac_evidence("day_fortune.taboos", Some(format!("count={taboo_count}"))),
            ruleset_id.clone(),
            ruleset_version.clone(),
        ));
    }

    // --- Personal alignment (requires gender) --------------------------
    if capability.has_gender {
        let birth_year = get_year_canchi(
            convert_solar_to_lunar(profile.day, profile.month, profile.year, profile.timezone).year,
        );
        let day_chi = snapshot.context.canchi.day.chi.as_str();
        let xung_hop = &snapshot.day_fortune.xung_hop;

        if birth_year.chi == day_chi {
            features.push(FeatureObservation::observed(
                AssessmentFeatureId::PersonalSameChi,
                ContributionPolarity::Neutral,
                0.3,
                "personal.same_chi",
                interaction_evidence("day_chi_eq_year_chi", None),
                ruleset_id.clone(),
                ruleset_version.clone(),
            ));
        } else if xung_hop.luc_xung == birth_year.chi {
            features.push(FeatureObservation::observed(
                AssessmentFeatureId::PersonalLucXung,
                ContributionPolarity::Avoid,
                0.8,
                "personal.luc_xung",
                interaction_evidence("luc_xung_lookup", None),
                ruleset_id.clone(),
                ruleset_version.clone(),
            ));
        } else if xung_hop.tam_hop.iter().any(|c| c == &birth_year.chi) {
            features.push(FeatureObservation::observed(
                AssessmentFeatureId::PersonalTamHop,
                ContributionPolarity::Favorable,
                0.4,
                "personal.tam_hop",
                interaction_evidence("tam_hop_lookup", None),
                ruleset_id.clone(),
                ruleset_version.clone(),
            ));
        } else if xung_hop.liu_he.as_deref() == Some(birth_year.chi.as_str()) {
            features.push(FeatureObservation::observed(
                AssessmentFeatureId::PersonalLiuHe,
                ContributionPolarity::Favorable,
                0.3,
                "personal.liu_he",
                interaction_evidence("liu_he_lookup", None),
                ruleset_id.clone(),
                ruleset_version.clone(),
            ));
        }
    }

    // --- Kua direction match -------------------------------------------
    if let Some(kua_result) = resolved.kua.as_ref() {
        let xuat_hanh = &snapshot.day_fortune.travel.xuat_hanh_huong;
        if kua_result
            .favorable_directions
            .iter()
            .any(|d| d.as_vn_str() == xuat_hanh.as_str())
        {
            features.push(FeatureObservation::observed(
                AssessmentFeatureId::KuaDirectionMatch,
                ContributionPolarity::Favorable,
                0.4,
                "personal.kua_favorable",
                SourceEvidence {
                    source_family: "interaction".to_string(),
                    source_id: SOURCE_VN_FOLK.to_string(),
                    method: "kua_favorable_match".to_string(),
                    profile: profile_id.clone(),
                    note: Some(format!("kua={} direction={}", kua_result.kua, xuat_hanh)),
                },
                ruleset_id.clone(),
                ruleset_version.clone(),
            ));
        } else if kua_result
            .unfavorable_directions
            .iter()
            .any(|d| d.as_vn_str() == xuat_hanh.as_str())
        {
            features.push(FeatureObservation::observed(
                AssessmentFeatureId::KuaDirectionMatch,
                ContributionPolarity::Avoid,
                0.4,
                "personal.kua_unfavorable",
                SourceEvidence {
                    source_family: "interaction".to_string(),
                    source_id: SOURCE_VN_FOLK.to_string(),
                    method: "kua_unfavorable_match".to_string(),
                    profile: profile_id.clone(),
                    note: Some(format!("kua={} direction={}", kua_result.kua, xuat_hanh)),
                },
                ruleset_id.clone(),
                ruleset_version.clone(),
            ));
        }
    }

    // --- Timing (Hoang Dao hour ratio; requires known birth time) ------
    if capability.has_time {
        let good = snapshot.context.gio_hoang_dao.good_hour_count as f32;
        let total = snapshot.context.gio_hoang_dao.all_hours.len() as f32;
        let ratio = if total > 0.0 { good / total } else { 0.0 };
        let polarity = if ratio >= 0.4 {
            ContributionPolarity::Favorable
        } else {
            ContributionPolarity::Neutral
        };
        features.push(FeatureObservation::observed(
            AssessmentFeatureId::TimingHoangDaoRatio,
            polarity,
            ratio,
            "timing.hoang_dao_ratio",
            almanac_evidence(
                "gio_hoang_dao_ratio",
                Some(format!("good={} total={}", good as u32, total as u32)),
            ),
            ruleset_id.clone(),
            ruleset_version.clone(),
        ));
    }

    // --- Annual pressure (yearly Hạn; requires gender) -----------------
    if let Some(han) = resolved.yearly_han.as_ref() {
        if han.han_count > 0 {
            let severity_strength = match han.severity {
                HanSeverity::Low => 0.3,
                HanSeverity::Medium => 0.55,
                HanSeverity::High => 0.85,
                HanSeverity::Critical => 1.0,
            };
            // The legacy v1 builder emitted a single AnnualPressure
            // contribution per Hạn assessment. baseline_v2 preserves that
            // shape via the Thai Tue feature identifier (the Hạn envelope
            // aggregates Tam Tai / Kim Lau / Hoang Oc / Thai Tue / Sao Hạn).
            features.push(FeatureObservation::observed(
                AssessmentFeatureId::AnnualThaiTue,
                ContributionPolarity::Avoid,
                severity_strength,
                format!("annual.han.{}", snapshot.context.solar.day),
                almanac_evidence(
                    "yearly_han",
                    Some(format!(
                        "count={} severity={:?}",
                        han.han_count, han.severity
                    )),
                ),
                ruleset_id.clone(),
                ruleset_version.clone(),
            ));
        }
    }

    // --- Explicit unavailable observations for capability gaps ----------
    // (amlich-l0wu) Feature identifiers that the current profile cannot
    // support are emitted as explicit `Unavailable` observations rather
    // than silently omitted, so the trace's feature list self-describes
    // what evidence was missing. The policy aggregation excludes them
    // from the denominator (unavailable != zero, per amlich-7bm4), and
    // explanations can surface them distinctly from neutral evidence.
    //
    // Only capability-gated feature families are emitted as unavailable.
    // Features that are simply "not triggered today" (e.g. no Tam Hop
    // match, no favorable Kua direction) stay omitted — they are not
    // missing evidence, just non-occurring signals.
    if !capability.has_gender {
        features.push(FeatureObservation::unavailable(
            AssessmentFeatureId::PersonalLucXung,
            "personal.luc_xung.unavailable",
            "requires gender for personal day-branch interaction",
            interaction_evidence("luc_xung_lookup", None),
            ruleset_id.clone(),
            ruleset_version.clone(),
        ));
        features.push(FeatureObservation::unavailable(
            AssessmentFeatureId::KuaDirectionMatch,
            "personal.kua.unavailable",
            "requires gender for Kua direction matching",
            SourceEvidence {
                source_family: "interaction".to_string(),
                source_id: SOURCE_VN_FOLK.to_string(),
                method: "kua_match".to_string(),
                profile: profile_id.clone(),
                note: None,
            },
            ruleset_id.clone(),
            ruleset_version.clone(),
        ));
        features.push(FeatureObservation::unavailable(
            AssessmentFeatureId::AnnualThaiTue,
            "annual.han.unavailable",
            "requires gender for yearly Hạn assessment",
            almanac_evidence("yearly_han", None),
            ruleset_id.clone(),
            ruleset_version.clone(),
        ));
    }
    if !capability.has_time {
        features.push(FeatureObservation::unavailable(
            AssessmentFeatureId::TimingHoangDaoRatio,
            "timing.hoang_dao_ratio.unavailable",
            "requires explicit birth time for personal timing context",
            almanac_evidence("gio_hoang_dao_ratio", None),
            ruleset_id.clone(),
            ruleset_version.clone(),
        ));
    }

    let _ = profile_id; // already captured by closure clones where needed
    features
}

/// Extract named, source-attributed hard veto events from the resolved
/// personal-day facts.
///
/// Bead: `amlich-l0wu`. Replaces the legacy v1 `polarity == Avoid &&
/// strength >= 0.8` implicit threshold with explicit, semantically
/// declared constraint events. Each veto carries a stable `veto_id`, the
/// axis the constraint originates from, a human-readable reason, and full
/// source evidence — so an explanation can name *why* a day was vetoed
/// rather than pointing at a numeric threshold.
///
/// ## Parity contract
///
/// For `baseline_v2`, the veto conditions are calibrated to fire on
/// exactly the same source-data states that produced an `Avoid` feature
/// at strength `>= 0.8` under v1, so user-visible decision buckets and
/// scores remain byte-identical to v1. The mechanism change is
/// intentional: an ordinary negative contribution can no longer become a
/// veto merely by crossing a strength threshold. A future policy version
/// may emit an `Avoid` observation at strength `0.9` that is *not* a veto
/// (because it is not declared here), which was impossible under v1.
///
/// ## Precedence
///
/// Vetoes are emitted in a stable, deterministic order
/// (`personal.luc_xung`, `annual.han_severe`,
/// `recommendation.ky_manh`, `day_fortune.taboos`). The decision
/// synthesizer applies them before any weighted suitability aggregation:
/// any veto present forces the `Avoid` bucket regardless of how
/// favorable the weighted axes are.
pub(super) fn extract_vetoes(
    snapshot: &DaySnapshot,
    profile: &BirthProfile,
    intent: ConsultationIntent,
    capability: BirthCapability,
    resolved: &ResolvedAssessmentInputs,
) -> Vec<VetoEvent> {
    let profile_id = snapshot.profile.clone();
    let ruleset_id = snapshot.ruleset_id.clone();
    let ruleset_version = snapshot.ruleset_version.clone();

    let interaction_evidence = |method: &'static str, note: Option<String>| SourceEvidence {
        source_family: "interaction".to_string(),
        source_id: SOURCE_KHCBPPT.to_string(),
        method: method.to_string(),
        profile: profile_id.clone(),
        note,
    };
    let almanac_evidence = |method: &'static str, note: Option<String>| SourceEvidence {
        source_family: "almanac_rule".to_string(),
        source_id: SOURCE_KHCBPPT.to_string(),
        method: method.to_string(),
        profile: profile_id.clone(),
        note,
    };

    let mut vetoes: Vec<VetoEvent> = Vec::new();

    // --- Personal Lục xung (requires gender) ---------------------------
    // The day's Lục xung branch matches the birth-year branch: a hard
    // personal-day clash that vetoes regardless of intent.
    if capability.has_gender {
        let birth_year = get_year_canchi(
            convert_solar_to_lunar(profile.day, profile.month, profile.year, profile.timezone).year,
        );
        let xung_hop = &snapshot.day_fortune.xung_hop;
        if xung_hop.luc_xung == birth_year.chi {
            vetoes.push(VetoEvent {
                veto_id: "veto.personal.luc_xung".to_string(),
                axis: AssessmentAxis::PersonalAlignment,
                reason: "Day branch clashes with birth-year branch (Lục xung)".to_string(),
                source_evidence: interaction_evidence(
                    "luc_xung_lookup",
                    Some(format!(
                        "day_luc_xung={} birth_year_chi={}",
                        xung_hop.luc_xung, birth_year.chi
                    )),
                ),
            });
        }
    }

    // --- Severe yearly Hạn (High / Critical severity) ------------------
    // The yearly Hạn envelope aggregates Tam Tai / Kim Lau / Hoang Oc /
    // Thai Tue / Sao Hạn. At High or Critical severity the combined
    // annual pressure is a hard constraint.
    if let Some(han) = resolved.yearly_han.as_ref() {
        if matches!(han.severity, HanSeverity::High | HanSeverity::Critical) {
            vetoes.push(VetoEvent {
                veto_id: "veto.annual.han_severe".to_string(),
                axis: AssessmentAxis::AnnualPressure,
                reason: format!("Yearly Hạn at {:?} severity", han.severity),
                source_evidence: almanac_evidence(
                    "yearly_han",
                    Some(format!(
                        "count={} severity={:?}",
                        han.han_count, han.severity
                    )),
                ),
            });
        }
    }

    // --- KyManh (forbidden) recommendation -----------------------------
    // A forbidden recommendation for any activity is a hard veto. When
    // the forbidden activity is the intent's primary, the veto
    // originates from the IntentFit axis; otherwise from
    // GenericDayQuality (where non-primary recommendations land).
    if let Some(rec) = resolved.recommendations.as_ref() {
        if let Some(activity) = rec
            .activities
            .iter()
            .find(|a| matches!(a.bucket, RecommendationBucket::KyManh))
        {
            let is_primary = activity.activity_id == intent.primary_activity();
            vetoes.push(VetoEvent {
                veto_id: "veto.recommendation.ky_manh".to_string(),
                axis: if is_primary {
                    AssessmentAxis::IntentFit
                } else {
                    AssessmentAxis::GenericDayQuality
                },
                reason: format!("KyManh (forbidden) recommendation: {}", activity.label.vi),
                source_evidence: SourceEvidence {
                    source_family: "almanac_rule".to_string(),
                    source_id: SOURCE_KHCBPPT.to_string(),
                    method: "recommendation_synthesis".to_string(),
                    profile: profile_id.clone(),
                    note: Some(format!(
                        "activity={} bucket=KyManh primary={}",
                        activity.activity_id.as_str(),
                        is_primary
                    )),
                },
            });
        }
    }

    // --- Stacked day-fortune taboos (3 or more) ------------------------
    // Three or more day-fortune taboos stacked on the same day indicate a
    // structurally conflicted day that vetoes regardless of personal
    // alignment.
    let taboo_count = snapshot.day_fortune.taboos.len();
    if taboo_count >= 3 {
        vetoes.push(VetoEvent {
            veto_id: "veto.day_fortune.taboos".to_string(),
            axis: AssessmentAxis::GenericDayQuality,
            reason: format!("{taboo_count} day-fortune taboos stacked"),
            source_evidence: almanac_evidence(
                "day_fortune.taboos",
                Some(format!("count={taboo_count}")),
            ),
        });
    }

    let _ = (ruleset_id, ruleset_version); // ruleset provenance flows through the features
    vetoes
}

/// Map a recommendation bucket to its v1 parity `(polarity, strength)`.
/// `intent_primary` selects the slightly-elevated weights the legacy v1
/// builder applied to the intent's primary activity.
fn recommendation_polarity_strength(
    bucket: RecommendationBucket,
    intent_primary: bool,
) -> (ContributionPolarity, f32) {
    let polarity = match bucket {
        RecommendationBucket::Nen => ContributionPolarity::Favorable,
        RecommendationBucket::CoThe => ContributionPolarity::Neutral,
        RecommendationBucket::Tranh => ContributionPolarity::Avoid,
        RecommendationBucket::KyManh => ContributionPolarity::Avoid,
    };
    let strength = match (bucket, intent_primary) {
        (RecommendationBucket::Nen, false) => 0.7,
        (RecommendationBucket::Nen, true) => 0.8,
        (RecommendationBucket::CoThe, false) => 0.4,
        (RecommendationBucket::CoThe, true) => 0.5,
        (RecommendationBucket::Tranh, false) => 0.6,
        (RecommendationBucket::Tranh, true) => 0.7,
        (RecommendationBucket::KyManh, false) => 0.9,
        (RecommendationBucket::KyManh, true) => 1.0,
    };
    (polarity, strength)
}
