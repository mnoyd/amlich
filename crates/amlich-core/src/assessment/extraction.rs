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
        thap_than::get_thap_than,
        tu_menh::{compute_kua, KuaResult},
        types::HeavenlyStem,
        xung_hop,
        yearly_han::{compute_yearly_han, HanSeverity, YearlyHanAssessment, YearlyHanInput},
    },
    assessment::{
        feature::{AssessmentFeatureId, FeatureObservation},
        strongest_taboo, taboo_contribution_strength, taboo_evidence_quality,
        trace::VetoEvent,
        AssessmentAxis, AssessmentInputs, ContributionPolarity, SourceEvidence,
    },
    bazi::{
        analysis::{analyze_bazi_chart, BaziAnalysisReport},
        chart::build_bazi_chart,
        types::{BaziChart, PillarKind},
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
    if let Some(taboo) = strongest_taboo(&snapshot.day_fortune.taboos) {
        let taboo_count = snapshot.day_fortune.taboos.len();
        let strength = taboo_contribution_strength(taboo);
        let quality = taboo_evidence_quality(taboo);
        let evidence = match taboo.evidence.as_ref() {
            Some(evidence) => SourceEvidence {
                source_family: "almanac_rule".to_string(),
                source_id: evidence.source_id.clone(),
                method: evidence.method.clone(),
                profile: evidence.profile.clone(),
                note: Some(format!(
                    "rule_id={} severity={} evidence_quality={quality} count={taboo_count}",
                    taboo.rule_id, taboo.severity
                )),
            },
            None => SourceEvidence {
                source_family: "unqualified_rule".to_string(),
                source_id: ruleset_id.clone(),
                method: "missing_rule_evidence".to_string(),
                profile: profile_id.clone(),
                note: Some(format!(
                    "rule_id={} severity={} evidence_quality={quality} count={taboo_count}",
                    taboo.rule_id, taboo.severity
                )),
            },
        };
        features.push(FeatureObservation::observed(
            AssessmentFeatureId::GenericDayQuality,
            ContributionPolarity::Avoid,
            strength,
            format!("day_fortune.taboo.{}", snapshot.context.solar.day),
            evidence,
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

    // --- Qualified hard day-fortune taboo -------------------------------
    // A hard rule only becomes a veto when its source evidence is complete.
    // Soft rules and unqualified hard strings remain weighted resistance.
    if let Some(taboo) = strongest_taboo(&snapshot.day_fortune.taboos)
        .filter(|taboo| taboo.severity == "hard" && taboo_contribution_strength(taboo) >= 0.8)
    {
        vetoes.push(VetoEvent {
            veto_id: "veto.day_fortune.taboos".to_string(),
            axis: AssessmentAxis::GenericDayQuality,
            reason: format!("Qualified hard day-fortune taboo: {}", taboo.rule_id),
            source_evidence: SourceEvidence {
                source_family: "almanac_rule".to_string(),
                source_id: taboo
                    .evidence
                    .as_ref()
                    .map(|evidence| evidence.source_id.clone())
                    .unwrap_or_else(|| ruleset_id.clone()),
                method: taboo
                    .evidence
                    .as_ref()
                    .map(|evidence| evidence.method.clone())
                    .unwrap_or_else(|| "missing_rule_evidence".to_string()),
                profile: taboo
                    .evidence
                    .as_ref()
                    .map(|evidence| evidence.profile.clone())
                    .unwrap_or_else(|| profile_id.clone()),
                note: Some(format!(
                    "rule_id={} severity={} evidence_quality={}",
                    taboo.rule_id,
                    taboo.severity,
                    taboo_evidence_quality(taboo)
                )),
            },
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

// ---------------------------------------------------------------------------
// Bazi-to-day observations (amlich-bz0f.2)
//
// These feature observations project the *target day* (the day being
// assessed) into the user's birth chart. They are distinct from the
// existing intra-chart Bazi observations (ten god distribution, element
// distribution, day-master strength) which describe the chart itself.
// The projection stays inside the assessment pipeline; Bazi chart
// scoring is computed elsewhere and never reused as a Day Assessment
// verdict input.
//
// The observations layer on top of the v2.2 policy (`amlich-47wn`) and
// are surfaced through the v2.3 `bazi_projection_v2_3` policy. They
// feed the `PersonalAlignment` axis so a user's Bazi context can
// explain personal-day suitability without affecting other axes.
//
// Each Bazi-to-day observation is fully deduplicated: a clash with the
// year pillar and a clash with the month pillar emit *one* Avoid
// contribution, not two. The dedup is keyed by relation kind, so a
// single chart can produce at most one Avoid and one Favorable branch
// observation per assessment.
// ---------------------------------------------------------------------------

/// Bazi-to-day pillar relation kinds recognised by the projection. Each
/// variant maps to a stable string identifier and a contribution
/// polarity. Adding a new relation kind requires a policy version
/// bump (`amlich-bz0f.2`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum BaziDayPillarRelation {
    /// Target day branch clashes (lục xung) with one or more natal
    /// pillars. Avoid polarity: the user-facing day is in direct
    /// conflict with an important natal pillar.
    Clash,
    /// Target day branch is in lục hợp with one or more natal
    /// pillars. Favorable polarity.
    LiuHe,
    /// Target day branch shares a tam hợp triad with one or more natal
    /// pillars. Favorable polarity.
    TamHop,
}

impl BaziDayPillarRelation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Clash => "clash",
            Self::LiuHe => "liu_he",
            Self::TamHop => "tam_hop",
        }
    }

    fn polarity(self) -> ContributionPolarity {
        match self {
            Self::Clash => ContributionPolarity::Avoid,
            Self::LiuHe | Self::TamHop => ContributionPolarity::Favorable,
        }
    }

    fn strength(self) -> f32 {
        match self {
            Self::Clash => 0.6,
            Self::LiuHe => 0.4,
            Self::TamHop => 0.3,
        }
    }
}

/// Project typed, source-attributed Bazi-to-day feature observations
/// from a resolved birth chart (`amlich-bz0f.2`).
///
/// Returns a `Vec<FeatureObservation>` containing:
/// - one [`AssessmentFeatureId::BaziTargetDayTenGod`] observation when
///   the target-day stem can be related to the natal day master;
/// - one [`AssessmentFeatureId::BaziTargetDayPillarRelation`]
///   observation per *unique* branch relation kind that fires against
///   any natal pillar (clash, lục hợp, tam hợp), so a chart that
///   clashes with both year and month pillars emits a single Avoid
///   contribution rather than two;
/// - one [`AssessmentFeatureId::BaziTargetDayElementResonance`]
///   observation when the target-day element can be compared to the
///   natal day-master element.
///
/// When the resolved inputs are missing the chart, the day master, or
/// the target-day Can Chi (e.g., the snapshot was built without a
/// target day), the function emits one explicit `Unavailable`
/// observation per Bazi-to-day feature so the trace self-describes
/// what evidence was missing (the `unavailable != zero` contract from
/// `amlich-7bm4`).
///
/// For date-only profiles (no birth time), `resolve_assessment_inputs`
/// drops the chart because the hour pillar cannot be derived. The
/// Bazi-to-day extraction rebuilds a date-only chart locally so the
/// year/month/day pillars stay available for the observations; the
/// hour pillar is still excluded from the branch-relation check via
/// the `capability.has_time` gate.
///
/// `capability.has_time` controls which natal pillars are eligible for
/// the branch-relation check: a time-known chart includes the hour
/// pillar; a date-only chart compares against year/month/day only.
/// The branch dedup is unaffected — it is per relation kind, not per
/// pillar.
pub(super) fn extract_bazi_target_day_observations(
    snapshot: &DaySnapshot,
    profile: &BirthProfile,
    capability: BirthCapability,
    resolved: &ResolvedAssessmentInputs,
) -> Vec<FeatureObservation> {
    let ruleset_id = snapshot.ruleset_id.clone();
    let ruleset_version = snapshot.ruleset_version.clone();
    let profile_id = snapshot.profile.clone();

    let bazi_evidence = |method: &'static str, note: Option<String>| SourceEvidence {
        source_family: "bazi_observation".to_string(),
        source_id: SOURCE_KHCBPPT.to_string(),
        method: method.to_string(),
        profile: profile_id.clone(),
        note,
    };

    let mut features: Vec<FeatureObservation> = Vec::new();

    // The Bazi-to-day observations project the target day into the
    // user's birth chart. For users with a known date (but no time),
    // `resolve_assessment_inputs` drops the chart because the hour
    // pillar cannot be derived. We rebuild a date-only chart locally
    // so the year/month/day pillars stay available for the Bazi-to-day
    // extraction (the hour pillar is still excluded from the
    // branch-relation check via the `capability.has_time` gate).
    let chart_ref: Option<BaziChart> = match resolved.chart.as_ref() {
        Some(chart) => Some(chart.clone()),
        None if capability.has_date => build_bazi_chart(bazi_input_from_profile(profile)).ok(),
        None => None,
    };

    let Some(chart) = chart_ref.as_ref() else {
        // Birth chart is unavailable: every Bazi-to-day observation is
        // unavailable. Emit one explicit unavailable observation per
        // declared feature identifier so the trace can list what was
        // missing (the amlich-7bm4 contract).
        features.push(FeatureObservation::unavailable(
            AssessmentFeatureId::BaziTargetDayTenGod,
            "bazi.target_day.ten_god.unavailable",
            "requires Bazi chart for target-day Ten God relation",
            bazi_evidence("target_day_ten_god", None),
            ruleset_id.clone(),
            ruleset_version.clone(),
        ));
        features.push(FeatureObservation::unavailable(
            AssessmentFeatureId::BaziTargetDayPillarRelation,
            "bazi.target_day.pillar_relation.unavailable",
            "requires Bazi chart for target-day branch relation",
            bazi_evidence("target_day_pillar_relation", None),
            ruleset_id.clone(),
            ruleset_version.clone(),
        ));
        features.push(FeatureObservation::unavailable(
            AssessmentFeatureId::BaziTargetDayElementResonance,
            "bazi.target_day.element_resonance.unavailable",
            "requires Bazi chart for target-day element resonance",
            bazi_evidence("target_day_element_resonance", None),
            ruleset_id.clone(),
            ruleset_version.clone(),
        ));
        return features;
    };

    let target_day = &snapshot.context.canchi.day;
    let target_day_stem_name = target_day.can.as_str();
    let natal_day_master_name = chart.day_master.can.as_str();

    // --- 1. Target-day Ten God relation to the natal day master ------
    let ten_god_feature = match (
        HeavenlyStem::try_from(target_day_stem_name),
        HeavenlyStem::try_from(natal_day_master_name),
    ) {
        (Ok(target_stem), Ok(master_stem)) => {
            let result = get_thap_than(master_stem, target_stem);
            let (polarity, strength) = ten_god_polarity_strength(result.label);
            Some(
                FeatureObservation::observed(
                    AssessmentFeatureId::BaziTargetDayTenGod,
                    polarity,
                    strength,
                    "bazi.target_day.ten_god",
                    bazi_evidence(
                        "target_day_ten_god",
                        Some(format!(
                            "target_stem={} day_master={} label={:?} relation={:?}",
                            target_day_stem_name,
                            natal_day_master_name,
                            result.label,
                            result.relation
                        )),
                    ),
                    ruleset_id.clone(),
                    ruleset_version.clone(),
                )
                .with_note(format!(
                    "{} → {}",
                    label_vi(result.label),
                    natal_day_master_name
                )),
            )
        }
        _ => Some(FeatureObservation::unavailable(
            AssessmentFeatureId::BaziTargetDayTenGod,
            "bazi.target_day.ten_god.unavailable",
            "could not parse target-day or natal day-master stem",
            bazi_evidence("target_day_ten_god", None),
            ruleset_id.clone(),
            ruleset_version.clone(),
        )),
    };
    if let Some(obs) = ten_god_feature {
        features.push(obs);
    }

    // --- 2. Target-day branch relation to natal pillars -------------
    //
    // The target-day branch (`snapshot.context.canchi.day.chi`) is
    // compared against the eligible natal pillar branches (year,
    // month, day, and hour when the birth time is known). Each
    // relation kind fires at most once per assessment.
    let target_chi = target_day.chi.as_str();
    let mut observed_relations: Vec<(BaziDayPillarRelation, Vec<PillarKind>)> = Vec::new();
    let eligible_pillars = eligible_natal_pillars(chart, capability.has_time);

    for (relation, pillars) in
        detect_pillar_relations(target_chi, &eligible_pillars, &chart.day_pillar)
    {
        observed_relations.push((relation, pillars));
    }

    if observed_relations.is_empty() {
        // No branch relation fires. Emit an Info observation so the
        // trace records that the relation was evaluated against the
        // full eligible pillar set but found no classical xung / hợp
        // / tam hợp pattern. Info polarity projects a 0.0 signed
        // value, so the axis aggregation is unaffected.
        let evaluated = eligible_pillars
            .iter()
            .map(|p| p.kind.as_str())
            .collect::<Vec<_>>()
            .join(",");
        features.push(
            FeatureObservation::observed(
                AssessmentFeatureId::BaziTargetDayPillarRelation,
                ContributionPolarity::Info,
                0.0,
                "bazi.target_day.pillar_relation",
                bazi_evidence(
                    "target_day_pillar_relation",
                    Some(format!(
                        "target_chi={} evaluated_pillars=[{}] matched=none",
                        target_chi, evaluated
                    )),
                ),
                ruleset_id.clone(),
                ruleset_version.clone(),
            )
            .with_note("Không có quan hệ xung/hợp giữa ngày và các trụ")
            .clone(),
        );
    } else {
        for (relation, pillars) in observed_relations {
            let pillar_names = pillars
                .iter()
                .map(|kind| kind.as_str())
                .collect::<Vec<_>>()
                .join(",");
            let polarity = relation.polarity();
            let strength = relation.strength();
            features.push(
                FeatureObservation::observed(
                    AssessmentFeatureId::BaziTargetDayPillarRelation,
                    polarity,
                    strength,
                    format!("bazi.target_day.pillar_relation.{}", relation.as_str()),
                    bazi_evidence(
                        "target_day_pillar_relation",
                        Some(format!(
                            "relation={} target_chi={} matched_pillars=[{}]",
                            relation.as_str(),
                            target_chi,
                            pillar_names
                        )),
                    ),
                    ruleset_id.clone(),
                    ruleset_version.clone(),
                )
                .with_note(format!(
                    "{} với trụ {}",
                    relation_vi(relation),
                    pillar_names
                )),
            );
        }
    }

    // --- 3. Target-day element resonance with the natal day master --
    let day_element = target_day.ngu_hanh.can.as_str();
    let day_master_element = chart.day_master.ngu_hanh.can.as_str();
    if let Some((polarity, strength)) = element_resonance(day_element, day_master_element) {
        features.push(
            FeatureObservation::observed(
                AssessmentFeatureId::BaziTargetDayElementResonance,
                polarity,
                strength,
                "bazi.target_day.element_resonance",
                bazi_evidence(
                    "target_day_element_resonance",
                    Some(format!(
                        "day_element={} day_master_element={} relation={:?}",
                        day_element,
                        day_master_element,
                        element_relation(day_element, day_master_element)
                    )),
                ),
                ruleset_id.clone(),
                ruleset_version.clone(),
            )
            .with_note(format!(
                "{} sinh/khắc với {}",
                day_element, day_master_element
            )),
        );
    } else {
        features.push(FeatureObservation::unavailable(
            AssessmentFeatureId::BaziTargetDayElementResonance,
            "bazi.target_day.element_resonance.unavailable",
            "could not parse day or day-master Ngũ Hành element",
            bazi_evidence("target_day_element_resonance", None),
            ruleset_id.clone(),
            ruleset_version.clone(),
        ));
    }

    let _ = (ruleset_id, ruleset_version, profile_id);
    features
}

/// Map a Thập Thần label to a `(polarity, strength)` pair for the
/// target-day Ten God observation. Resource / support labels
/// (Tỷ Kiến, Kiếp Tài, Chính Ấn, Thiên Ấn) are favorable; draining
/// / opposition labels (Thực Thần, Thương Quan, Chính Tài, Thiên
/// Tài, Chính Quan, Thất Sát) are avoided. Strengths are conservative
/// so the new feature does not dominate the PersonalAlignment axis.
fn ten_god_polarity_strength(
    label: crate::almanac::types::ThapThanLabel,
) -> (ContributionPolarity, f32) {
    use crate::almanac::types::ThapThanLabel::*;
    match label {
        TyKien | KiepTai | ChinhAn | ThienAn => (ContributionPolarity::Favorable, 0.4),
        ThucThan | ThuongQuan | ChinhTai | ThienTai | ChinhQuan | ThatSat => {
            (ContributionPolarity::Avoid, 0.4)
        }
    }
}

fn label_vi(label: crate::almanac::types::ThapThanLabel) -> &'static str {
    use crate::almanac::types::ThapThanLabel::*;
    match label {
        TyKien => "Tỷ Kiến",
        KiepTai => "Kiếp Tài",
        ThucThan => "Thực Thần",
        ThuongQuan => "Thương Quan",
        ChinhTai => "Chính Tài",
        ThienTai => "Thiên Tài",
        ChinhQuan => "Chính Quan",
        ThatSat => "Thất Sát",
        ChinhAn => "Chính Ấn",
        ThienAn => "Thiên Ấn",
    }
}

fn relation_vi(relation: BaziDayPillarRelation) -> &'static str {
    match relation {
        BaziDayPillarRelation::Clash => "Lục xung",
        BaziDayPillarRelation::LiuHe => "Lục hợp",
        BaziDayPillarRelation::TamHop => "Tam hợp",
    }
}

/// Return the natal pillars eligible for the Bazi-to-day branch
/// relation check, gated on whether the birth time is known. The
/// natal day pillar is always present; the hour pillar is only
/// present when `has_time` is true.
fn eligible_natal_pillars(
    chart: &BaziChart,
    has_time: bool,
) -> Vec<&crate::bazi::types::BaziPillar> {
    let mut pillars = vec![&chart.year_pillar, &chart.month_pillar, &chart.day_pillar];
    if has_time {
        if let Some(hour) = chart.hour_pillar.as_ref() {
            pillars.push(hour);
        }
    }
    pillars
}

/// Detect which [`BaziDayPillarRelation`] kinds fire between the
/// target-day branch and the eligible natal pillars. Returns one
/// `(relation, matched_pillar_kinds)` pair per *relation kind* — if
/// both the year pillar and the month pillar clash with the target
/// day, the `Clash` kind appears once with `matched_pillar_kinds`
/// listing both. This is the dedup that prevents the same underlying
/// signal from inflating the PersonalAlignment axis.
///
/// The natal day pillar's branch is treated specially: matching the
/// target-day branch against the day pillar is a "self-meeting"
/// signal. The function records it as an `Info` branch relation (no
/// Avoid / Favorable) so the chart can still surface the meeting in
/// the trace without doubling the day-pillar's own contribution.
fn detect_pillar_relations(
    target_chi: &str,
    pillars: &[&crate::bazi::types::BaziPillar],
    day_pillar: &crate::bazi::types::BaziPillar,
) -> Vec<(BaziDayPillarRelation, Vec<PillarKind>)> {
    let mut clash_pillars: Vec<PillarKind> = Vec::new();
    let mut liu_he_pillars: Vec<PillarKind> = Vec::new();
    let mut tam_hop_pillars: Vec<PillarKind> = Vec::new();

    for pillar in pillars {
        // Self-meeting on the day pillar is recorded as `Info` and
        // contributes no Avoid / Favorable weight (the existing
        // day-pillar clash on the personal_alignment axis covers
        // it; the `birth_pillar_clash` path is for the *other*
        // natal pillars).
        if std::ptr::eq(*pillar, day_pillar) {
            continue;
        }
        let pillar_chi = pillar.can_chi.chi.as_str();
        if pillar_chi == target_chi {
            continue;
        }
        let target_chi_idx = chi_index(target_chi);
        let pillar_chi_idx = chi_index(pillar_chi);

        if let (Some(t_idx), Some(p_idx)) = (target_chi_idx, pillar_chi_idx) {
            if xung_hop::luc_xung(t_idx) == pillar_chi {
                clash_pillars.push(pillar.kind);
            } else if xung_hop::get_liu_he(t_idx) == pillar_chi {
                liu_he_pillars.push(pillar.kind);
            } else if xung_hop::tam_hop(t_idx).contains(&pillar_chi) {
                tam_hop_pillars.push(pillar.kind);
            }
            let _ = p_idx;
        }
    }

    let mut out: Vec<(BaziDayPillarRelation, Vec<PillarKind>)> = Vec::new();
    if !clash_pillars.is_empty() {
        sort_and_dedup_pillars(&mut clash_pillars);
        out.push((BaziDayPillarRelation::Clash, clash_pillars));
    }
    if !liu_he_pillars.is_empty() {
        sort_and_dedup_pillars(&mut liu_he_pillars);
        out.push((BaziDayPillarRelation::LiuHe, liu_he_pillars));
    }
    if !tam_hop_pillars.is_empty() {
        sort_and_dedup_pillars(&mut tam_hop_pillars);
        out.push((BaziDayPillarRelation::TamHop, tam_hop_pillars));
    }
    // Sorted for deterministic trace output.
    out.sort_by_key(|(relation, _)| relation_order(*relation));
    out
}

fn sort_and_dedup_pillars(pillars: &mut Vec<PillarKind>) {
    pillars.sort_by_key(|k| pillar_kind_order(*k));
    pillars.dedup();
}

fn pillar_kind_order(kind: PillarKind) -> u8 {
    match kind {
        PillarKind::Year => 0,
        PillarKind::Month => 1,
        PillarKind::Day => 2,
        PillarKind::Hour => 3,
    }
}

fn relation_order(relation: BaziDayPillarRelation) -> u8 {
    match relation {
        BaziDayPillarRelation::Clash => 0,
        BaziDayPillarRelation::LiuHe => 1,
        BaziDayPillarRelation::TamHop => 2,
    }
}

fn chi_index(name: &str) -> Option<usize> {
    crate::types::CHI.iter().position(|c| *c == name)
}

/// Look up the Ngũ Hành relation between two element names and map it
/// to a `(polarity, strength)` pair for the
/// `BaziTargetDayElementResonance` observation.
///
/// The mapping is intentionally simple: any sinh (generation) relation
/// is favorable (the day nourishes or is nourished by the natal day
/// master); any khắc (control) relation is avoided. Same element is
/// treated as neutral — the day-master and the day share the same
/// element, which is neither inherently favorable nor inherently
/// adverse. Unknown element names degrade to `None` so the caller can
/// emit an explicit `Unavailable` observation.
fn element_resonance(day: &str, master: &str) -> Option<(ContributionPolarity, f32)> {
    match element_relation(day, master) {
        Some(crate::almanac::types::FiveElementRelation::Same) => {
            Some((ContributionPolarity::Neutral, 0.2))
        }
        Some(
            crate::almanac::types::FiveElementRelation::DayGeneratesTarget
            | crate::almanac::types::FiveElementRelation::TargetGeneratesDay,
        ) => Some((ContributionPolarity::Favorable, 0.4)),
        Some(
            crate::almanac::types::FiveElementRelation::DayControlsTarget
            | crate::almanac::types::FiveElementRelation::TargetControlsDay,
        ) => Some((ContributionPolarity::Avoid, 0.4)),
        None => None,
    }
}

/// Compute the [`FiveElementRelation`] between two element names using
/// the Ngũ Hành sinh / khắc cycles. Returns `None` for unknown element
/// names so the caller can degrade to an `Unavailable` observation.
fn element_relation(day: &str, master: &str) -> Option<crate::almanac::types::FiveElementRelation> {
    use crate::almanac::types::FiveElementRelation;
    if day == master {
        return Some(FiveElementRelation::Same);
    }
    if !is_known_element(day) || !is_known_element(master) {
        return None;
    }
    if generates(day) == Some(master) {
        return Some(FiveElementRelation::DayGeneratesTarget);
    }
    if generates(master) == Some(day) {
        return Some(FiveElementRelation::TargetGeneratesDay);
    }
    if controls(day, master) {
        return Some(FiveElementRelation::DayControlsTarget);
    }
    if controls(master, day) {
        return Some(FiveElementRelation::TargetControlsDay);
    }
    None
}

fn is_known_element(name: &str) -> bool {
    matches!(name, "Mộc" | "Hỏa" | "Thổ" | "Kim" | "Thủy")
}

/// Ngũ Hành sinh (generation) cycle. `generates("Mộc")` returns
/// `Some("Hỏa")` because Mộc generates Hỏa.
fn generates(element: &str) -> Option<&'static str> {
    match element {
        "Mộc" => Some("Hỏa"),
        "Hỏa" => Some("Thổ"),
        "Thổ" => Some("Kim"),
        "Kim" => Some("Thủy"),
        "Thủy" => Some("Mộc"),
        _ => None,
    }
}

/// Ngũ Hành khắc (control) cycle. `controls("Mộc", "Thổ")` is `true`
/// because Mộc controls Thổ.
fn controls(a: &str, b: &str) -> bool {
    matches!(
        (a, b),
        ("Mộc", "Thổ") | ("Hỏa", "Kim") | ("Thổ", "Thủy") | ("Kim", "Mộc") | ("Thủy", "Hỏa")
    )
}
