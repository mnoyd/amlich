use crate::dto::{
    ActiveRecommendationPackDto,
    ActivityLabelDto,
    AnnualPillarDto,
    BaziAdvisoryDomainsDto,
    BaziAdvisoryDto,
    BaziAnalysisDto,
    BaziCanChiDto,
    BaziChartDto,
    BaziChartMetadataDto,
    BaziComputedMetricsDto,
    BaziCoreMetricsDto,
    BaziDomainScoreDto,
    BaziDomainScoresDto,
    BaziInteractionMetricDto,
    BaziLuckPillarDto,
    BaziLunarDateDto,
    BaziPillarDto,
    BaziQuery,
    BaziReportDto,
    BaziScoreContributorDto,
    BaziStructureMetricsDto,
    BaziTimingDto,
    BaziTimingMetricsDto,
    BaziTimingWindowScoreDto,
    BirthDataTierDto,
    CanChiDto,
    CanChiInfoDto,
    CanInsightDto,
    ChartInteractionDto,
    ChiInsightDto,
    ConventionMetadataDto,
    DailyRecommendationsDto,
    DayConflictDto,
    DayDeityDto,
    DayElementDto,
    DayFortuneDto,
    DayGuidanceDto,
    DayInfoDto,
    DayMasterStrengthDto,
    DayStarDto,
    DayStarsDto,
    DayTabooDto,
    DayTenGodsDto,
    ElementDistributionDto,
    ElementInsightDto,
    FestivalInsightDto,
    FoodInsightDto,
    GioHoangDaoDto,
    HiddenStemEntryDto,
    HolidayDto,
    HolidayInsightDto,
    HourInfoDto,
    // Bazi Derived Report DTOs
    KhongVongAnalysisDto,
    KhongVongPairDto,
    KhongVongPillarEntryDto,
    KuaResultDto,
    LocalizedListDto,
    LocalizedTextDto,
    LunarDto,
    MenhCungDto,
    MonthlyPillarDto,
    NaAmErrorDto,
    NaAmLookupResultDto,
    NguHanhDto,
    ProverbInsightDto,
    RecommendationBucketDto,
    RecommendationEvidenceDto,
    RecommendationEvidenceSourceDto,
    RecommendationPackCatalogEntryDto,
    RecommendationReasonDto,
    RecommendationScopeDto,
    RecommendationSeverityDto,
    RegionsInsightDto,
    RuleEvidenceDto,
    RulesetCatalogEntryDto,
    RulesetDefaultsDto,
    RulesetSourceNoteDto,
    SolarDto,
    StarRuleEvidenceDto,
    SynthesizedRecommendationDto,
    TabooInsightDto,
    TangCanDto,
    TenGodDistributionDto,
    ThaiNguyenDto,
    ThanSatEntryDto,
    ThanSatResultDto,
    ThapThanResultDto,
    TietKhiDto,
    TietKhiInsightDto,
    TravelDirectionDto,
    TrucDto,
    UnavailableSectionDto,
    UsefulGodDto,
    XungHopDto,
};

impl From<&amlich_core::NguHanh> for NguHanhDto {
    fn from(value: &amlich_core::NguHanh) -> Self {
        Self {
            can: value.can.clone(),
            chi: value.chi.clone(),
        }
    }
}

impl From<&amlich_core::BaziCanChiResponse> for BaziCanChiDto {
    fn from(value: &amlich_core::BaziCanChiResponse) -> Self {
        Self {
            can: value.can.clone(),
            chi: value.chi.clone(),
            full: value.full.clone(),
            can_index: value.can_index,
            chi_index: value.chi_index,
        }
    }
}

impl From<&amlich_core::BaziLunarDateResponse> for BaziLunarDateDto {
    fn from(value: &amlich_core::BaziLunarDateResponse) -> Self {
        Self {
            day: value.day,
            month: value.month,
            year: value.year,
            is_leap: value.is_leap,
        }
    }
}

impl From<&amlich_core::HiddenStemEntry> for HiddenStemEntryDto {
    fn from(value: &amlich_core::HiddenStemEntry) -> Self {
        Self {
            stem_symbol: value.stem_symbol.clone(),
            stem_name: value.stem_name.clone(),
            strength: value.strength,
            ten_god_to_day_master: value
                .ten_god_to_day_master
                .as_ref()
                .map(ThapThanResultDto::from),
        }
    }
}

impl From<&amlich_core::BaziPillarResponse> for BaziPillarDto {
    fn from(value: &amlich_core::BaziPillarResponse) -> Self {
        Self {
            kind: match value.kind {
                amlich_core::PillarKind::Year => "year",
                amlich_core::PillarKind::Month => "month",
                amlich_core::PillarKind::Day => "day",
                amlich_core::PillarKind::Hour => "hour",
            }
            .to_string(),
            can_chi: BaziCanChiDto::from(&value.can_chi),
            hidden_stems: value
                .hidden_stems
                .iter()
                .map(HiddenStemEntryDto::from)
                .collect(),
            na_am: value.na_am.clone(),
            stem_relation_to_day_master: value
                .stem_relation_to_day_master
                .as_ref()
                .map(ThapThanResultDto::from),
        }
    }
}

impl From<&amlich_core::BaziChartMetadataResponse> for BaziChartMetadataDto {
    fn from(value: &amlich_core::BaziChartMetadataResponse) -> Self {
        Self {
            timezone: value.timezone,
            use_solar_time: value.use_solar_time,
            year_basis: value.year_basis.clone(),
            month_basis: value.month_basis.clone(),
            day_basis: value.day_basis.clone(),
            hour_basis: value.hour_basis.clone(),
            hour_evidence: value.hour_evidence.as_ref().map(RuleEvidenceDto::from),
        }
    }
}

impl
    From<(
        &BaziQuery,
        &amlich_core::BaziChartResponse,
        BirthDataTierDto,
    )> for BaziChartDto
{
    fn from(
        (query, value, tier): (
            &BaziQuery,
            &amlich_core::BaziChartResponse,
            BirthDataTierDto,
        ),
    ) -> Self {
        Self {
            input: query.clone(),
            tier,
            lunar_date: BaziLunarDateDto::from(&value.lunar_date),
            day_master: BaziCanChiDto::from(&value.day_master),
            pillars: value.pillars.iter().map(BaziPillarDto::from).collect(),
            metadata: BaziChartMetadataDto::from(&value.metadata),
        }
    }
}

impl From<&amlich_core::bazi::contracts::ElementDistributionResponse> for ElementDistributionDto {
    fn from(value: &amlich_core::bazi::contracts::ElementDistributionResponse) -> Self {
        Self {
            moc: value.moc,
            hoa: value.hoa,
            tho: value.tho,
            kim: value.kim,
            thuy: value.thuy,
        }
    }
}

impl From<&amlich_core::DayMasterStrengthResponse> for DayMasterStrengthDto {
    fn from(value: &amlich_core::DayMasterStrengthResponse) -> Self {
        Self {
            score: value.score,
            label: value.label.clone(),
            reasons: value.reasons.clone(),
        }
    }
}

impl From<&amlich_core::ChartInteractionResponse> for ChartInteractionDto {
    fn from(value: &amlich_core::ChartInteractionResponse) -> Self {
        Self {
            kind: value.kind.clone(),
            participants: value.participants.clone(),
            summary_vi: value.summary_vi.clone(),
        }
    }
}

impl From<&amlich_core::bazi::contracts::TenGodDistributionResponse> for TenGodDistributionDto {
    fn from(value: &amlich_core::bazi::contracts::TenGodDistributionResponse) -> Self {
        Self {
            ty_kien: value.ty_kien,
            kiep_tai: value.kiep_tai,
            thuc_than: value.thuc_than,
            thuong_quan: value.thuong_quan,
            chinh_tai: value.chinh_tai,
            thien_tai: value.thien_tai,
            chinh_quan: value.chinh_quan,
            that_sat: value.that_sat,
            chinh_an: value.chinh_an,
            thien_an: value.thien_an,
        }
    }
}

impl
    From<(
        &amlich_core::BaziAnalysisResponse,
        BirthDataTierDto,
        Vec<UnavailableSectionDto>,
    )> for BaziAnalysisDto
{
    fn from(
        (value, tier, unavailable_sections): (
            &amlich_core::BaziAnalysisResponse,
            BirthDataTierDto,
            Vec<UnavailableSectionDto>,
        ),
    ) -> Self {
        Self {
            tier,
            element_distribution: ElementDistributionDto::from(&value.element_distribution),
            day_master_strength: DayMasterStrengthDto::from(&value.day_master_strength),
            interactions: value
                .interactions
                .iter()
                .map(ChartInteractionDto::from)
                .collect(),
            ten_god_distribution: TenGodDistributionDto::from(&value.ten_god_distribution),
            unavailable_sections,
        }
    }
}

impl From<&amlich_core::BaziLuckPillarResponse> for BaziLuckPillarDto {
    fn from(value: &amlich_core::BaziLuckPillarResponse) -> Self {
        Self {
            index: value.index,
            can_chi: value.can_chi.clone(),
            start_age: value.start_age,
            end_age: value.end_age,
            ten_god_to_day_master: value
                .ten_god_to_day_master
                .as_ref()
                .map(ThapThanResultDto::from),
        }
    }
}

impl From<&amlich_core::AnnualPillarResponse> for AnnualPillarDto {
    fn from(value: &amlich_core::AnnualPillarResponse) -> Self {
        Self {
            year: value.year,
            can_chi: value.can_chi.clone(),
            branch: value.branch.clone(),
            ten_god_to_day_master: value
                .ten_god_to_day_master
                .as_ref()
                .map(ThapThanResultDto::from),
            interactions: value.interactions.clone(),
        }
    }
}

impl From<&amlich_core::MonthlyPillarResponse> for MonthlyPillarDto {
    fn from(value: &amlich_core::MonthlyPillarResponse) -> Self {
        Self {
            year: value.year,
            month: value.month,
            can_chi: value.can_chi.clone(),
            branch: value.branch.clone(),
            ten_god_to_day_master: value
                .ten_god_to_day_master
                .as_ref()
                .map(ThapThanResultDto::from),
            interactions: value.interactions.clone(),
        }
    }
}

impl From<&amlich_core::BaziTimingResponse> for BaziTimingDto {
    fn from(value: &amlich_core::BaziTimingResponse) -> Self {
        Self {
            dai_van: value.dai_van.iter().map(BaziLuckPillarDto::from).collect(),
            active_dai_van: value.active_dai_van.as_ref().map(BaziLuckPillarDto::from),
            annual: AnnualPillarDto::from(&value.annual),
            monthly: value.monthly.iter().map(MonthlyPillarDto::from).collect(),
        }
    }
}

impl From<&amlich_core::UsefulGodResponse> for UsefulGodDto {
    fn from(value: &amlich_core::UsefulGodResponse) -> Self {
        Self {
            favorable_elements: value
                .favorable_elements
                .iter()
                .map(|element| format!("{:?}", element))
                .collect(),
            unfavorable_elements: value
                .unfavorable_elements
                .iter()
                .map(|element| format!("{:?}", element))
                .collect(),
            tentative_yong_shen: value
                .tentative_yong_shen
                .map(|element| format!("{:?}", element)),
            tentative_xi_shen: value
                .tentative_xi_shen
                .map(|element| format!("{:?}", element)),
            confidence: value.confidence.clone(),
            reasons: value.reasons.clone(),
        }
    }
}

impl From<&amlich_core::UsefulGodAnalysis> for UsefulGodDto {
    fn from(value: &amlich_core::UsefulGodAnalysis) -> Self {
        Self {
            favorable_elements: value
                .favorable_elements
                .iter()
                .map(|element| format!("{element:?}").to_lowercase())
                .collect(),
            unfavorable_elements: value
                .unfavorable_elements
                .iter()
                .map(|element| format!("{element:?}").to_lowercase())
                .collect(),
            tentative_yong_shen: value
                .tentative_yong_shen
                .map(|element| format!("{element:?}").to_lowercase()),
            tentative_xi_shen: value
                .tentative_xi_shen
                .map(|element| format!("{element:?}").to_lowercase()),
            confidence: value.confidence.clone(),
            reasons: value.reasons.clone(),
        }
    }
}

impl From<&amlich_core::bazi::contracts::BaziAdvisoryDomainsResponse> for BaziAdvisoryDomainsDto {
    fn from(value: &amlich_core::bazi::contracts::BaziAdvisoryDomainsResponse) -> Self {
        Self {
            career: value.career.clone(),
            wealth: value.wealth.clone(),
            relationship: value.relationship.clone(),
            health: value.health.clone(),
            timing: value.timing.clone(),
        }
    }
}

impl From<&amlich_core::BaziAdvisoryDomains> for BaziAdvisoryDomainsDto {
    fn from(value: &amlich_core::BaziAdvisoryDomains) -> Self {
        Self {
            career: value.career.clone(),
            wealth: value.wealth.clone(),
            relationship: value.relationship.clone(),
            health: value.health.clone(),
            timing: value.timing.clone(),
        }
    }
}

impl From<&amlich_core::BaziAdvisoryExport> for BaziAdvisoryDto {
    fn from(value: &amlich_core::BaziAdvisoryExport) -> Self {
        Self {
            summary: value.summary.clone(),
            severity: value.severity.clone(),
            top_signals: value.top_signals.clone(),
            why_this_matters: value.why_this_matters.clone(),
            recommended_actions: value.recommended_actions.clone(),
            priority_order: value.priority_order.clone(),
            useful_god_analysis: UsefulGodDto::from(&value.useful_god_analysis),
            summary_vi: value.summary_vi.clone(),
            warnings: value.warnings.clone(),
            domains: BaziAdvisoryDomainsDto::from(&value.domains),
        }
    }
}

impl From<&amlich_core::BaziInteractionMetric> for BaziInteractionMetricDto {
    fn from(value: &amlich_core::BaziInteractionMetric) -> Self {
        Self {
            kind: value.kind.clone(),
            participants: value.participants.clone(),
            impact: value.impact,
        }
    }
}

impl From<&amlich_core::BaziScoreContributor> for BaziScoreContributorDto {
    fn from(value: &amlich_core::BaziScoreContributor) -> Self {
        Self {
            signal: value.signal.clone(),
            delta: value.delta,
        }
    }
}

impl From<&amlich_core::BaziDomainScore> for BaziDomainScoreDto {
    fn from(value: &amlich_core::BaziDomainScore) -> Self {
        Self {
            score: value.score,
            label: value.label.clone(),
            confidence: value.confidence,
            contributors: value
                .contributors
                .iter()
                .map(BaziScoreContributorDto::from)
                .collect(),
        }
    }
}

impl From<&amlich_core::BaziDomainScores> for BaziDomainScoresDto {
    fn from(value: &amlich_core::BaziDomainScores) -> Self {
        Self {
            career: BaziDomainScoreDto::from(&value.career),
            wealth: BaziDomainScoreDto::from(&value.wealth),
            relationship: BaziDomainScoreDto::from(&value.relationship),
            health: BaziDomainScoreDto::from(&value.health),
            timing: BaziDomainScoreDto::from(&value.timing),
        }
    }
}

impl From<&amlich_core::BaziTimingWindowScore> for BaziTimingWindowScoreDto {
    fn from(value: &amlich_core::BaziTimingWindowScore) -> Self {
        Self {
            month: value.month,
            score: value.score,
            label: value.label.clone(),
        }
    }
}

impl From<&amlich_core::BaziTimingMetrics> for BaziTimingMetricsDto {
    fn from(value: &amlich_core::BaziTimingMetrics) -> Self {
        Self {
            current_dai_van_alignment: value.current_dai_van_alignment,
            annual_alignment: value.annual_alignment,
            monthly_windows: value
                .monthly_windows
                .iter()
                .map(BaziTimingWindowScoreDto::from)
                .collect(),
            activation_summary: value.activation_summary.clone(),
        }
    }
}

impl From<&amlich_core::BaziCoreMetrics> for BaziCoreMetricsDto {
    fn from(value: &amlich_core::BaziCoreMetrics) -> Self {
        Self {
            day_master_strength_score: value.day_master_strength_score,
            day_master_strength_label: value.day_master_strength_label.clone(),
            season_support_score: value.season_support_score,
            same_element_score: value.same_element_score,
            resource_support_score: value.resource_support_score,
            drain_pressure_score: value.drain_pressure_score,
            control_pressure_score: value.control_pressure_score,
            element_balance_score: value.element_balance_score,
        }
    }
}

impl From<&amlich_core::BaziStructureMetrics> for BaziStructureMetricsDto {
    fn from(value: &amlich_core::BaziStructureMetrics) -> Self {
        Self {
            dominant_elements: value
                .dominant_elements
                .iter()
                .map(|element| format!("{:?}", element))
                .collect(),
            weak_elements: value
                .weak_elements
                .iter()
                .map(|element| format!("{:?}", element))
                .collect(),
            dominant_ten_gods: value.dominant_ten_gods.clone(),
            interaction_score: value.interaction_score,
            notable_interactions: value
                .notable_interactions
                .iter()
                .map(BaziInteractionMetricDto::from)
                .collect(),
            confidence: value.confidence,
        }
    }
}

impl
    From<(
        &amlich_core::BaziComputedMetrics,
        BirthDataTierDto,
        Vec<UnavailableSectionDto>,
    )> for BaziComputedMetricsDto
{
    fn from(
        (value, tier, unavailable_sections): (
            &amlich_core::BaziComputedMetrics,
            BirthDataTierDto,
            Vec<UnavailableSectionDto>,
        ),
    ) -> Self {
        Self {
            tier,
            core_metrics: BaziCoreMetricsDto::from(&value.core_metrics),
            structure_metrics: BaziStructureMetricsDto::from(&value.structure_metrics),
            domain_scores: BaziDomainScoresDto::from(&value.domain_scores),
            timing_metrics: BaziTimingMetricsDto::from(&value.timing_metrics),
            unavailable_sections,
        }
    }
}

impl From<(&BaziQuery, &amlich_core::BaziReport, BirthDataTierDto)> for BaziReportDto {
    fn from(
        (query, value, tier): (&BaziQuery, &amlich_core::BaziReport, BirthDataTierDto),
    ) -> Self {
        let advisory = BaziAdvisoryDto::from(&amlich_core::export_bazi_advisory(&value.advisory));
        Self {
            summary: advisory.summary.clone(),
            severity: advisory.severity.clone(),
            top_signals: advisory.top_signals.clone(),
            why_this_matters: advisory.why_this_matters.clone(),
            recommended_actions: advisory.recommended_actions.clone(),
            priority_order: advisory.priority_order.clone(),
            chart: BaziChartDto::from((
                query,
                value
                    .chart_response
                    .as_ref()
                    .expect("chart response present"),
                tier.clone(),
            )),
            analysis: BaziAnalysisDto::from((
                value
                    .analysis_response
                    .as_ref()
                    .expect("analysis response present"),
                tier.clone(),
                if tier == BirthDataTierDto::Date {
                    vec![UnavailableSectionDto {
                        section: "hour_pillar".to_string(),
                        reason: "requires birth hour and minute".to_string(),
                        required_fields: vec!["hour".to_string(), "minute".to_string()],
                    }]
                } else {
                    Vec::new()
                },
            )),
            timing: value.timing_response.as_ref().map(BaziTimingDto::from),
            computed_metrics: BaziComputedMetricsDto::from((
                &value.computed_metrics,
                tier.clone(),
                if tier == BirthDataTierDto::Date {
                    vec![UnavailableSectionDto {
                        section: "hour_pillar".to_string(),
                        reason: "requires birth hour and minute".to_string(),
                        required_fields: vec!["hour".to_string(), "minute".to_string()],
                    }]
                } else {
                    Vec::new()
                },
            )),
            advisory,
        }
    }
}

impl From<&amlich_core::CanChi> for CanChiDto {
    fn from(value: &amlich_core::CanChi) -> Self {
        Self {
            can_index: value.can_index,
            chi_index: value.chi_index,
            can: value.can.clone(),
            chi: value.chi.clone(),
            full: value.full.clone(),
            con_giap: value.con_giap.clone(),
            ngu_hanh: NguHanhDto::from(&value.ngu_hanh),
        }
    }
}

impl From<&amlich_core::CanChiSet> for CanChiInfoDto {
    fn from(value: &amlich_core::CanChiSet) -> Self {
        Self {
            day: CanChiDto::from(&value.day),
            month: CanChiDto::from(&value.month),
            year: CanChiDto::from(&value.year),
            full: format!(
                "{}, tháng {}, năm {}",
                value.day.full, value.month.full, value.year.full
            ),
        }
    }
}

impl From<&amlich_core::SolarDate> for SolarDto {
    fn from(value: &amlich_core::SolarDate) -> Self {
        Self {
            day: value.day,
            month: value.month,
            year: value.year,
            day_of_week: value.day_of_week,
            day_of_week_name: amlich_core::THU[value.day_of_week].to_string(),
            date_string: format!("{}-{:02}-{:02}", value.year, value.month, value.day),
        }
    }
}

impl From<&amlich_core::lunar::LunarDate> for LunarDto {
    fn from(value: &amlich_core::lunar::LunarDate) -> Self {
        Self {
            day: value.day,
            month: value.month,
            year: value.year,
            is_leap_month: value.is_leap,
            date_string: format!(
                "{}/{}/{}{}",
                value.day,
                value.month,
                value.year,
                if value.is_leap { " (nhuận)" } else { "" }
            ),
        }
    }
}

impl From<&amlich_core::tietkhi::SolarTerm> for TietKhiDto {
    fn from(value: &amlich_core::tietkhi::SolarTerm) -> Self {
        Self {
            index: value.index,
            name: value.name.clone(),
            description: value.description.clone(),
            longitude: value.longitude,
            current_longitude: value.current_longitude,
            season: value.season.clone(),
        }
    }
}

impl From<&amlich_core::gio_hoang_dao::HourInfo> for HourInfoDto {
    fn from(value: &amlich_core::gio_hoang_dao::HourInfo) -> Self {
        Self {
            hour_index: value.hour_index,
            hour_chi: value.hour_chi.clone(),
            time_range: value.time_range.clone(),
            star: value.star.clone(),
            is_good: value.is_good,
        }
    }
}

impl From<&amlich_core::gio_hoang_dao::GioHoangDao> for GioHoangDaoDto {
    fn from(value: &amlich_core::gio_hoang_dao::GioHoangDao) -> Self {
        Self {
            day_chi: value.day_chi.clone(),
            good_hour_count: value.good_hour_count,
            good_hours: value.good_hours.iter().map(HourInfoDto::from).collect(),
            all_hours: value.all_hours.iter().map(HourInfoDto::from).collect(),
            summary: value.summary.clone(),
        }
    }
}

impl From<&amlich_core::almanac::types::DayElement> for DayElementDto {
    fn from(value: &amlich_core::almanac::types::DayElement) -> Self {
        Self {
            na_am: value.na_am.clone(),
            element: value.element.clone(),
            can_element: value.can_element.clone(),
            chi_element: value.chi_element.clone(),
            evidence: value.evidence.as_ref().map(RuleEvidenceDto::from),
        }
    }
}

impl From<&amlich_core::almanac::types::RuleEvidence> for RuleEvidenceDto {
    fn from(value: &amlich_core::almanac::types::RuleEvidence) -> Self {
        Self {
            source_id: value.source_id.clone(),
            method: value.method.clone(),
            profile: value.profile.clone(),
        }
    }
}

impl From<&amlich_core::almanac::types::DayConflict> for DayConflictDto {
    fn from(value: &amlich_core::almanac::types::DayConflict) -> Self {
        Self {
            opposing_chi: value.opposing_chi.clone(),
            opposing_con_giap: value.opposing_con_giap.clone(),
            tuoi_xung: value.tuoi_xung.clone(),
            sat_huong: value.sat_huong.clone(),
            evidence: value.evidence.as_ref().map(RuleEvidenceDto::from),
        }
    }
}

impl From<&amlich_core::almanac::types::TravelDirection> for TravelDirectionDto {
    fn from(value: &amlich_core::almanac::types::TravelDirection) -> Self {
        Self {
            xuat_hanh_huong: value.xuat_hanh_huong.clone(),
            tai_than: value.tai_than.clone(),
            hy_than: value.hy_than.clone(),
            evidence: value.evidence.as_ref().map(RuleEvidenceDto::from),
        }
    }
}

impl From<&amlich_core::almanac::types::DayStar> for DayStarDto {
    fn from(value: &amlich_core::almanac::types::DayStar) -> Self {
        let system = match value.system {
            amlich_core::almanac::types::StarSystem::NhiThapBatTu => "nhi-thap-bat-tu",
        }
        .to_string();
        let quality = match value.quality {
            amlich_core::almanac::types::StarQuality::Cat => "cat",
            amlich_core::almanac::types::StarQuality::Hung => "hung",
            amlich_core::almanac::types::StarQuality::Binh => "binh",
        }
        .to_string();
        Self {
            system,
            index: value.index,
            name: value.name.clone(),
            quality,
            evidence: value.evidence.as_ref().map(RuleEvidenceDto::from),
        }
    }
}

impl From<&amlich_core::almanac::types::StarRuleEvidence> for StarRuleEvidenceDto {
    fn from(value: &amlich_core::almanac::types::StarRuleEvidence) -> Self {
        let quality = match value.quality {
            amlich_core::almanac::types::StarQuality::Cat => "cat",
            amlich_core::almanac::types::StarQuality::Hung => "hung",
            amlich_core::almanac::types::StarQuality::Binh => "binh",
        }
        .to_string();
        Self {
            name: value.name.clone(),
            quality,
            category: value.category.clone(),
            source_id: value.source_id.clone(),
            method: value.method.clone(),
            profile: value.profile.clone(),
        }
    }
}

impl From<&amlich_core::almanac::types::DayStars> for DayStarsDto {
    fn from(value: &amlich_core::almanac::types::DayStars) -> Self {
        let star_system = value.star_system.as_ref().map(|system| match system {
            amlich_core::almanac::types::StarSystem::NhiThapBatTu => "nhi-thap-bat-tu",
        });

        Self {
            cat_tinh: value.cat_tinh.clone(),
            sat_tinh: value.sat_tinh.clone(),
            day_star: value.day_star.as_ref().map(DayStarDto::from),
            star_system: star_system.map(str::to_string),
            evidence: value.evidence.as_ref().map(RuleEvidenceDto::from),
            matched_rules: value
                .matched_rules
                .iter()
                .map(StarRuleEvidenceDto::from)
                .collect(),
        }
    }
}

impl From<&amlich_core::almanac::types::XungHopResult> for XungHopDto {
    fn from(value: &amlich_core::almanac::types::XungHopResult) -> Self {
        Self {
            luc_xung: value.luc_xung.clone(),
            tam_hop: value.tam_hop.clone(),
            tu_hanh_xung: value.tu_hanh_xung.clone(),
            liu_he: value.liu_he.clone(),
            xiang_hai: value.xiang_hai.clone(),
            xiang_xing: value.xiang_xing.clone(),
        }
    }
}

impl From<&amlich_core::almanac::types::TangCan> for TangCanDto {
    fn from(value: &amlich_core::almanac::types::TangCan) -> Self {
        Self {
            main: value.main.clone(),
            central: value.central.clone(),
            residual: value.residual.clone(),
            strength: value.strength,
        }
    }
}

impl From<&amlich_core::almanac::types::TrucInfo> for TrucDto {
    fn from(value: &amlich_core::almanac::types::TrucInfo) -> Self {
        Self {
            index: value.index,
            name: value.name.clone(),
            quality: value.quality.clone(),
            evidence: value.evidence.as_ref().map(RuleEvidenceDto::from),
        }
    }
}

impl From<&amlich_core::almanac::types::DayTaboo> for DayTabooDto {
    fn from(value: &amlich_core::almanac::types::DayTaboo) -> Self {
        Self {
            rule_id: value.rule_id.clone(),
            name: value.name.clone(),
            severity: value.severity.clone(),
            reason: value.reason.clone(),
            evidence: value.evidence.as_ref().map(RuleEvidenceDto::from),
        }
    }
}

impl From<&amlich_core::almanac::types::DayDeity> for DayDeityDto {
    fn from(value: &amlich_core::almanac::types::DayDeity) -> Self {
        let classification = match value.classification {
            amlich_core::almanac::types::DayDeityClassification::HoangDao => "hoang_dao",
            amlich_core::almanac::types::DayDeityClassification::HacDao => "hac_dao",
        }
        .to_string();

        Self {
            name: value.name.clone(),
            classification,
            evidence: value.evidence.as_ref().map(RuleEvidenceDto::from),
        }
    }
}

impl From<&amlich_core::almanac::types::DayTenGods> for DayTenGodsDto {
    fn from(value: &amlich_core::almanac::types::DayTenGods) -> Self {
        Self {
            to_year_stem: value.to_year_stem.as_ref().map(ThapThanResultDto::from),
            to_self: value.to_self.as_ref().map(ThapThanResultDto::from),
        }
    }
}

impl From<&amlich_core::almanac::types::ThapThanResult> for ThapThanResultDto {
    fn from(value: &amlich_core::almanac::types::ThapThanResult) -> Self {
        let relation = match value.relation {
            amlich_core::almanac::types::FiveElementRelation::Same => "same".to_string(),
            amlich_core::almanac::types::FiveElementRelation::DayGeneratesTarget => {
                "day_generates_target".to_string()
            }
            amlich_core::almanac::types::FiveElementRelation::TargetGeneratesDay => {
                "target_generates_day".to_string()
            }
            amlich_core::almanac::types::FiveElementRelation::DayControlsTarget => {
                "day_controls_target".to_string()
            }
            amlich_core::almanac::types::FiveElementRelation::TargetControlsDay => {
                "target_controls_day".to_string()
            }
        };

        let label = match value.label {
            amlich_core::almanac::types::ThapThanLabel::TyKien => "ty_kien".to_string(),
            amlich_core::almanac::types::ThapThanLabel::KiepTai => "kiep_tai".to_string(),
            amlich_core::almanac::types::ThapThanLabel::ThucThan => "thuc_than".to_string(),
            amlich_core::almanac::types::ThapThanLabel::ThuongQuan => "thuong_quan".to_string(),
            amlich_core::almanac::types::ThapThanLabel::ChinhTai => "chinh_tai".to_string(),
            amlich_core::almanac::types::ThapThanLabel::ThienTai => "thien_tai".to_string(),
            amlich_core::almanac::types::ThapThanLabel::ChinhQuan => "chinh_quan".to_string(),
            amlich_core::almanac::types::ThapThanLabel::ThatSat => "that_sat".to_string(),
            amlich_core::almanac::types::ThapThanLabel::ChinhAn => "chinh_an".to_string(),
            amlich_core::almanac::types::ThapThanLabel::ThienAn => "thien_an".to_string(),
        };

        Self {
            label,
            relation,
            same_polarity: value.same_polarity,
            evidence: RuleEvidenceDto::from(&value.evidence),
        }
    }
}

impl From<&amlich_core::almanac::tu_menh::KuaResult> for KuaResultDto {
    fn from(value: &amlich_core::almanac::tu_menh::KuaResult) -> Self {
        let group = match value.group {
            amlich_core::almanac::tu_menh::KuaGroup::East => "east".to_string(),
            amlich_core::almanac::tu_menh::KuaGroup::West => "west".to_string(),
        };

        let favorable_directions = value
            .favorable_directions
            .iter()
            .map(|d| d.to_string())
            .collect();

        let unfavorable_directions = value
            .unfavorable_directions
            .iter()
            .map(|d| d.to_string())
            .collect();

        let convention = ConventionMetadataDto {
            year_basis: value.convention.year_basis.clone(),
            kua_five_resolution: value.convention.kua5_resolution.clone(),
            gender_encoding: value.convention.gender_encoding.clone(),
        };

        Self {
            kua: value.kua,
            group,
            favorable_directions,
            unfavorable_directions,
            convention,
        }
    }
}

impl From<&amlich_core::almanac::types::DayFortune> for DayFortuneDto {
    fn from(value: &amlich_core::almanac::types::DayFortune) -> Self {
        Self {
            ruleset_id: value.ruleset_id.clone(),
            ruleset_version: value.ruleset_version.clone(),
            profile: value.profile.clone(),
            day_element: DayElementDto::from(&value.day_element),
            conflict: DayConflictDto::from(&value.conflict),
            travel: TravelDirectionDto::from(&value.travel),
            stars: DayStarsDto::from(&value.stars),
            day_deity: value.day_deity.as_ref().map(DayDeityDto::from),
            taboos: value.taboos.iter().map(DayTabooDto::from).collect(),
            xung_hop: XungHopDto::from(&value.xung_hop),
            truc: TrucDto::from(&value.truc),
            tang_can: value.tang_can.as_ref().map(TangCanDto::from),
            ten_gods: value.ten_gods.as_ref().map(DayTenGodsDto::from),
            tu_menh: value.tu_menh.as_ref().map(KuaResultDto::from),
        }
    }
}

fn activity_id_to_snake_case(
    activity_id: amlich_core::almanac::recommendation::ActivityId,
) -> String {
    match activity_id {
        amlich_core::almanac::recommendation::ActivityId::Travel => "travel",
        amlich_core::almanac::recommendation::ActivityId::MeetingSocial => "meeting_social",
        amlich_core::almanac::recommendation::ActivityId::OpeningStart => "opening_start",
        amlich_core::almanac::recommendation::ActivityId::ContractAgreement => "contract_agreement",
        amlich_core::almanac::recommendation::ActivityId::BusinessTrade => "business_trade",
        amlich_core::almanac::recommendation::ActivityId::FinanceInvestment => "finance_investment",
        amlich_core::almanac::recommendation::ActivityId::ConstructionGroundbreaking => {
            "construction_groundbreaking"
        }
        amlich_core::almanac::recommendation::ActivityId::RepairRenovation => "repair_renovation",
        amlich_core::almanac::recommendation::ActivityId::MoveRelocation => "move_relocation",
        amlich_core::almanac::recommendation::ActivityId::WeddingEngagement => "wedding_engagement",
        amlich_core::almanac::recommendation::ActivityId::LawsuitDispute => "lawsuit_dispute",
        amlich_core::almanac::recommendation::ActivityId::PrayerOffering => "prayer_offering",
        amlich_core::almanac::recommendation::ActivityId::MedicalTreatment => "medical_treatment",
        amlich_core::almanac::recommendation::ActivityId::BurialMemorial => "burial_memorial",
        amlich_core::almanac::recommendation::ActivityId::CleaningPurging => "cleaning_purging",
    }
    .to_string()
}

impl From<&amlich_core::almanac::recommendation::ActivityLabel> for ActivityLabelDto {
    fn from(value: &amlich_core::almanac::recommendation::ActivityLabel) -> Self {
        Self {
            vi: value.vi.clone(),
            en: value.en.clone(),
        }
    }
}

impl From<amlich_core::almanac::recommendation::RecommendationScope> for RecommendationScopeDto {
    fn from(value: amlich_core::almanac::recommendation::RecommendationScope) -> Self {
        match value {
            amlich_core::almanac::recommendation::RecommendationScope::GeneralDay => {
                RecommendationScopeDto::GeneralDay
            }
        }
    }
}

impl From<amlich_core::almanac::recommendation::RecommendationBucket> for RecommendationBucketDto {
    fn from(value: amlich_core::almanac::recommendation::RecommendationBucket) -> Self {
        match value {
            amlich_core::almanac::recommendation::RecommendationBucket::Nen => {
                RecommendationBucketDto::Nen
            }
            amlich_core::almanac::recommendation::RecommendationBucket::CoThe => {
                RecommendationBucketDto::CoThe
            }
            amlich_core::almanac::recommendation::RecommendationBucket::Tranh => {
                RecommendationBucketDto::Tranh
            }
            amlich_core::almanac::recommendation::RecommendationBucket::KyManh => {
                RecommendationBucketDto::KyManh
            }
        }
    }
}

impl From<amlich_core::almanac::recommendation::RecommendationSeverity>
    for RecommendationSeverityDto
{
    fn from(value: amlich_core::almanac::recommendation::RecommendationSeverity) -> Self {
        match value {
            amlich_core::almanac::recommendation::RecommendationSeverity::Primary => {
                RecommendationSeverityDto::Primary
            }
            amlich_core::almanac::recommendation::RecommendationSeverity::Supporting => {
                RecommendationSeverityDto::Supporting
            }
            amlich_core::almanac::recommendation::RecommendationSeverity::Override => {
                RecommendationSeverityDto::Override
            }
        }
    }
}

impl From<amlich_core::almanac::recommendation::RecommendationEvidenceSource>
    for RecommendationEvidenceSourceDto
{
    fn from(value: amlich_core::almanac::recommendation::RecommendationEvidenceSource) -> Self {
        match value {
            amlich_core::almanac::recommendation::RecommendationEvidenceSource::DayGuidance => {
                RecommendationEvidenceSourceDto::DayGuidance
            }
            amlich_core::almanac::recommendation::RecommendationEvidenceSource::Truc => {
                RecommendationEvidenceSourceDto::Truc
            }
            amlich_core::almanac::recommendation::RecommendationEvidenceSource::Stars => {
                RecommendationEvidenceSourceDto::Stars
            }
            amlich_core::almanac::recommendation::RecommendationEvidenceSource::DayDeity => {
                RecommendationEvidenceSourceDto::DayDeity
            }
            amlich_core::almanac::recommendation::RecommendationEvidenceSource::Taboo => {
                RecommendationEvidenceSourceDto::Taboo
            }
            amlich_core::almanac::recommendation::RecommendationEvidenceSource::XungHop => {
                RecommendationEvidenceSourceDto::XungHop
            }
            amlich_core::almanac::recommendation::RecommendationEvidenceSource::TietKhi => {
                RecommendationEvidenceSourceDto::TietKhi
            }
            amlich_core::almanac::recommendation::RecommendationEvidenceSource::GioHoangDao => {
                RecommendationEvidenceSourceDto::GioHoangDao
            }
            amlich_core::almanac::recommendation::RecommendationEvidenceSource::Travel => {
                RecommendationEvidenceSourceDto::Travel
            }
            amlich_core::almanac::recommendation::RecommendationEvidenceSource::ProductRule => {
                RecommendationEvidenceSourceDto::ProductRule
            }
        }
    }
}

impl From<&amlich_core::almanac::recommendation::RecommendationEvidence>
    for RecommendationEvidenceDto
{
    fn from(value: &amlich_core::almanac::recommendation::RecommendationEvidence) -> Self {
        Self {
            source: RecommendationEvidenceSourceDto::from(value.source),
            code: value.code.clone(),
            note: value.note.clone(),
        }
    }
}

impl From<&amlich_core::almanac::recommendation::RecommendationReason> for RecommendationReasonDto {
    fn from(value: &amlich_core::almanac::recommendation::RecommendationReason) -> Self {
        Self {
            rule_id: value.rule_id.clone(),
            severity: RecommendationSeverityDto::from(value.severity),
            summary_vi: value.summary_vi.clone(),
            summary_en: value.summary_en.clone(),
            evidence: RecommendationEvidenceDto::from(&value.evidence),
        }
    }
}

impl From<&amlich_core::almanac::recommendation::ActiveRecommendationPack>
    for ActiveRecommendationPackDto
{
    fn from(value: &amlich_core::almanac::recommendation::ActiveRecommendationPack) -> Self {
        Self {
            pack_id: value.pack_id.clone(),
            version: value.version.clone(),
            source_family: value.source_family.clone(),
            mode: match value.mode {
                amlich_core::almanac::recommendation::RecommendationPackMode::Advisory => {
                    "advisory"
                }
                amlich_core::almanac::recommendation::RecommendationPackMode::TraditionVariant => {
                    "tradition_variant"
                }
                amlich_core::almanac::recommendation::RecommendationPackMode::Experimental => {
                    "experimental"
                }
            }
            .to_string(),
        }
    }
}

impl From<&amlich_core::almanac::types::RuleSetDefaults> for RulesetDefaultsDto {
    fn from(value: &amlich_core::almanac::types::RuleSetDefaults) -> Self {
        Self {
            tz_offset: value.tz_offset,
            meridian: value.meridian.clone(),
        }
    }
}

impl From<&amlich_core::almanac::types::RuleSetSourceNote> for RulesetSourceNoteDto {
    fn from(value: &amlich_core::almanac::types::RuleSetSourceNote) -> Self {
        Self {
            family: value.family.clone(),
            source_id: value.source_id.clone(),
            note: value.note.clone(),
        }
    }
}

impl From<&amlich_core::almanac::data::RulesetRegistryEntry> for RulesetCatalogEntryDto {
    fn from(value: &amlich_core::almanac::data::RulesetRegistryEntry) -> Self {
        let descriptor = value.descriptor.to_document_descriptor();

        Self {
            id: descriptor.id,
            canonical_id: value.descriptor.id.to_string(),
            version: descriptor.version,
            region: descriptor.region,
            profile: descriptor.profile,
            schema_version: descriptor.schema_version,
            is_default: value.descriptor.id == amlich_core::almanac::data::DEFAULT_RULESET_ID,
            aliases: value
                .aliases
                .iter()
                .map(|alias| (*alias).to_string())
                .collect(),
            defaults: RulesetDefaultsDto::from(&descriptor.defaults),
            source_notes: descriptor
                .source_notes
                .iter()
                .map(RulesetSourceNoteDto::from)
                .collect(),
        }
    }
}

impl From<&amlich_core::almanac::recommendation::RecommendationPackDescriptor>
    for RecommendationPackCatalogEntryDto
{
    fn from(value: &amlich_core::almanac::recommendation::RecommendationPackDescriptor) -> Self {
        Self {
            pack_id: value.pack_id.to_string(),
            request_field: "enabled_pack_ids".to_string(),
            version: value.version.to_string(),
            source_family: value.source_family.to_string(),
            mode: match value.mode {
                amlich_core::almanac::recommendation::RecommendationPackMode::Advisory => {
                    "advisory"
                }
                amlich_core::almanac::recommendation::RecommendationPackMode::TraditionVariant => {
                    "tradition_variant"
                }
                amlich_core::almanac::recommendation::RecommendationPackMode::Experimental => {
                    "experimental"
                }
            }
            .to_string(),
        }
    }
}

impl From<&amlich_core::almanac::recommendation::SynthesizedRecommendation>
    for SynthesizedRecommendationDto
{
    fn from(value: &amlich_core::almanac::recommendation::SynthesizedRecommendation) -> Self {
        Self {
            activity_id: activity_id_to_snake_case(value.activity_id),
            label: ActivityLabelDto::from(&value.label),
            bucket: RecommendationBucketDto::from(value.bucket),
            reasons: value
                .reasons
                .iter()
                .map(RecommendationReasonDto::from)
                .collect(),
        }
    }
}

impl From<&amlich_core::almanac::recommendation::DailyRecommendations> for DailyRecommendationsDto {
    fn from(value: &amlich_core::almanac::recommendation::DailyRecommendations) -> Self {
        Self {
            ruleset_id: value.ruleset_id.clone(),
            ruleset_version: value.ruleset_version.clone(),
            profile: value.profile.clone(),
            scope: RecommendationScopeDto::from(value.scope),
            version: value.version.clone(),
            summary_vi: value.summary_vi.clone(),
            summary_en: value.summary_en.clone(),
            active_packs: value
                .active_packs
                .iter()
                .map(ActiveRecommendationPackDto::from)
                .collect(),
            activities: value
                .activities
                .iter()
                .map(SynthesizedRecommendationDto::from)
                .collect(),
        }
    }
}

impl From<&amlich_core::DaySnapshot> for DayInfoDto {
    fn from(value: &amlich_core::DaySnapshot) -> Self {
        Self {
            ruleset_id: value.ruleset_id.clone(),
            ruleset_version: value.ruleset_version.clone(),
            profile: value.profile.clone(),
            solar: SolarDto::from(&value.context.solar),
            lunar: LunarDto::from(&value.context.lunar),
            jd: value.context.jd,
            canchi: CanChiInfoDto::from(&value.context.canchi),
            tiet_khi: TietKhiDto::from(&value.context.tiet_khi),
            gio_hoang_dao: GioHoangDaoDto::from(&value.context.gio_hoang_dao),
            day_fortune: Some(DayFortuneDto::from(&value.day_fortune)),
            daily_recommendations: DailyRecommendationsDto::from(&value.daily_recommendations),
            contextual_recommendations: value
                .contextual_recommendations
                .as_ref()
                .map(DailyRecommendationsDto::from),
        }
    }
}

impl From<&amlich_core::holidays::Holiday> for HolidayDto {
    fn from(value: &amlich_core::holidays::Holiday) -> Self {
        Self {
            name: value.name.clone(),
            description: value.description.clone(),
            solar_day: value.solar_day,
            solar_month: value.solar_month,
            solar_year: value.solar_year,
            lunar_day: value.lunar_date.as_ref().map(|d| d.day),
            lunar_month: value.lunar_date.as_ref().map(|d| d.month),
            lunar_year: value.lunar_date.as_ref().map(|d| d.year),
            is_solar: value.is_solar,
            category: value.category.clone(),
            is_major: value.is_major,
        }
    }
}

impl From<&amlich_core::holiday_data::BilingualText> for LocalizedTextDto {
    fn from(value: &amlich_core::holiday_data::BilingualText) -> Self {
        Self {
            vi: value.vi.clone(),
            en: value.en.clone(),
        }
    }
}

impl From<&amlich_core::insight_data::BilingualText> for LocalizedTextDto {
    fn from(value: &amlich_core::insight_data::BilingualText) -> Self {
        Self {
            vi: value.vi.clone(),
            en: value.en.clone(),
        }
    }
}

impl From<&amlich_core::insight_data::BilingualList> for LocalizedListDto {
    fn from(value: &amlich_core::insight_data::BilingualList) -> Self {
        Self {
            vi: value.vi.clone(),
            en: value.en.clone(),
        }
    }
}

impl From<&amlich_core::holiday_data::BilingualList> for LocalizedListDto {
    fn from(value: &amlich_core::holiday_data::BilingualList) -> Self {
        Self {
            vi: value.vi.clone(),
            en: value.en.clone(),
        }
    }
}

impl From<&amlich_core::holiday_data::FoodItem> for FoodInsightDto {
    fn from(value: &amlich_core::holiday_data::FoodItem) -> Self {
        Self {
            name: LocalizedTextDto::from(&value.name),
            description: LocalizedTextDto::from(&value.description),
        }
    }
}

impl From<&amlich_core::holiday_data::TabooItem> for TabooInsightDto {
    fn from(value: &amlich_core::holiday_data::TabooItem) -> Self {
        Self {
            action: LocalizedTextDto::from(&value.action),
            reason: LocalizedTextDto::from(&value.reason),
        }
    }
}

impl From<&amlich_core::holiday_data::ProverbItem> for ProverbInsightDto {
    fn from(value: &amlich_core::holiday_data::ProverbItem) -> Self {
        Self {
            text: value.text.clone(),
            meaning: LocalizedTextDto::from(&value.meaning),
        }
    }
}

impl From<&amlich_core::holiday_data::Regions> for RegionsInsightDto {
    fn from(value: &amlich_core::holiday_data::Regions) -> Self {
        Self {
            north: LocalizedTextDto::from(&value.north),
            central: LocalizedTextDto::from(&value.central),
            south: LocalizedTextDto::from(&value.south),
        }
    }
}

impl From<&amlich_core::holiday_data::LunarFestivalData> for FestivalInsightDto {
    fn from(value: &amlich_core::holiday_data::LunarFestivalData) -> Self {
        Self {
            names: LocalizedListDto {
                vi: value.names.vi.clone(),
                en: value.names.en.clone(),
            },
            origin: value.origin.as_ref().map(LocalizedTextDto::from),
            activities: value.activities.as_ref().map(LocalizedListDto::from),
            food: value.food.iter().map(FoodInsightDto::from).collect(),
            taboos: value.taboos.iter().map(TabooInsightDto::from).collect(),
            proverbs: value.proverbs.iter().map(ProverbInsightDto::from).collect(),
            regions: value.regions.as_ref().map(RegionsInsightDto::from),
            category: value.category.clone(),
            is_major: value.is_major,
        }
    }
}

impl From<&amlich_core::holiday_data::SolarHolidayData> for HolidayInsightDto {
    fn from(value: &amlich_core::holiday_data::SolarHolidayData) -> Self {
        Self {
            names: LocalizedListDto {
                vi: value.names.vi.clone(),
                en: value.names.en.clone(),
            },
            origin: value.origin.as_ref().map(LocalizedTextDto::from),
            significance: value.significance.as_ref().map(LocalizedTextDto::from),
            activities: value.activities.as_ref().map(LocalizedListDto::from),
            traditions: value.traditions.as_ref().map(LocalizedListDto::from),
            food: value.food.iter().map(FoodInsightDto::from).collect(),
            taboos: value.taboos.iter().map(TabooInsightDto::from).collect(),
            proverbs: value.proverbs.iter().map(ProverbInsightDto::from).collect(),
            regions: value.regions.as_ref().map(RegionsInsightDto::from),
            category: value.category.clone(),
            is_major: value.is_major,
        }
    }
}

impl From<(&String, &amlich_core::insight_data::ElementInfo)> for ElementInsightDto {
    fn from((key, value): (&String, &amlich_core::insight_data::ElementInfo)) -> Self {
        Self {
            key: key.clone(),
            name: LocalizedTextDto::from(&value.name),
            nature: LocalizedTextDto::from(&value.nature),
        }
    }
}

impl From<&amlich_core::insight_data::CanInfo> for CanInsightDto {
    fn from(value: &amlich_core::insight_data::CanInfo) -> Self {
        Self {
            name: value.name.clone(),
            element: value.element.clone(),
            meaning: LocalizedTextDto::from(&value.meaning),
            nature: LocalizedTextDto::from(&value.nature),
        }
    }
}

impl From<&amlich_core::insight_data::ChiInfo> for ChiInsightDto {
    fn from(value: &amlich_core::insight_data::ChiInfo) -> Self {
        Self {
            name: value.name.clone(),
            animal: LocalizedTextDto::from(&value.animal),
            element: value.element.clone(),
            meaning: LocalizedTextDto::from(&value.meaning),
            hours: value.hours.clone(),
        }
    }
}

impl From<&amlich_core::insight_data::DayGuidance> for DayGuidanceDto {
    fn from(value: &amlich_core::insight_data::DayGuidance) -> Self {
        Self {
            good_for: LocalizedListDto::from(&value.good_for),
            avoid_for: LocalizedListDto::from(&value.avoid_for),
        }
    }
}

impl From<&amlich_core::insight_data::TietKhiInsight> for TietKhiInsightDto {
    fn from(value: &amlich_core::insight_data::TietKhiInsight) -> Self {
        Self {
            id: value.id.clone(),
            name: LocalizedTextDto::from(&value.name),
            longitude: value.longitude,
            meaning: LocalizedTextDto::from(&value.meaning),
            astronomy: LocalizedTextDto::from(&value.astronomy),
            agriculture: LocalizedListDto::from(&value.agriculture),
            health: LocalizedListDto::from(&value.health),
            weather: LocalizedTextDto::from(&value.weather),
        }
    }
}

// Na Am conversion implementations

impl From<amlich_core::almanac::na_am::NaAmError> for NaAmErrorDto {
    fn from(error: amlich_core::almanac::na_am::NaAmError) -> Self {
        let (error_type, message) = match error {
            amlich_core::almanac::na_am::NaAmError::InvalidCycleIndex => (
                "invalid_cycle_index".to_string(),
                "Cycle index must be between 1 and 60".to_string(),
            ),
            amlich_core::almanac::na_am::NaAmError::InvalidStemBranchPair => (
                "invalid_stem_branch_pair".to_string(),
                "Stem and branch must have matching parity (both odd or both even)".to_string(),
            ),
            amlich_core::almanac::na_am::NaAmError::UnknownStem => (
                "unknown_stem".to_string(),
                "Unknown heavenly stem name".to_string(),
            ),
            amlich_core::almanac::na_am::NaAmError::UnknownBranch => (
                "unknown_branch".to_string(),
                "Unknown earthly branch name".to_string(),
            ),
        };

        Self {
            error: error_type,
            message,
        }
    }
}

impl From<&amlich_core::almanac::data::NaAmEntry> for NaAmLookupResultDto {
    fn from(entry: &amlich_core::almanac::data::NaAmEntry) -> Self {
        // Find indices for can and chi
        use amlich_core::types::{CAN, CHI};
        let can_idx = CAN.iter().position(|&c| c == entry.can).unwrap_or(0);
        let chi_idx = CHI.iter().position(|&c| c == entry.chi).unwrap_or(0);

        // Convert to cycle index using sexagenary cycle utilities
        use amlich_core::almanac::sexagenary_cycle::canchi_to_cycle_index;
        let cycle_index = canchi_to_cycle_index(can_idx, chi_idx).unwrap_or(1);

        // Get metadata and profile from ruleset data
        use amlich_core::almanac::data::get_ruleset_data;
        let ruleset =
            get_ruleset_data("vn_baseline_v1").expect("default ruleset should be available");
        let meta = &ruleset.na_am_meta;

        Self {
            cycle_index,
            can: entry.can.clone(),
            chi: entry.chi.clone(),
            na_am: entry.na_am.clone(),
            element: entry.element.clone(),
            source_id: meta.source_id.clone(),
            method: meta.method.clone(),
            profile: ruleset.profile.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// Bazi Derived Report conversions
// ---------------------------------------------------------------------------

impl From<&amlich_core::types::CanChi> for BaziCanChiDto {
    fn from(value: &amlich_core::types::CanChi) -> Self {
        Self {
            can: value.can.clone(),
            chi: value.chi.clone(),
            full: value.full.clone(),
            can_index: value.can_index,
            chi_index: value.chi_index,
        }
    }
}

impl From<&amlich_core::bazi::types::ThaiNguyenResult> for ThaiNguyenDto {
    fn from(value: &amlich_core::bazi::types::ThaiNguyenResult) -> Self {
        Self {
            can_chi: BaziCanChiDto::from(&value.can_chi),
            evidence: RuleEvidenceDto::from(&value.evidence),
        }
    }
}

impl From<&amlich_core::bazi::types::MenhCungResult> for MenhCungDto {
    fn from(value: &amlich_core::bazi::types::MenhCungResult) -> Self {
        Self {
            menh_cung: BaziCanChiDto::from(&value.menh_cung),
            than_cung: BaziCanChiDto::from(&value.than_cung),
            evidence: RuleEvidenceDto::from(&value.evidence),
        }
    }
}

fn pillar_kind_to_string(kind: amlich_core::bazi::PillarKind) -> String {
    match kind {
        amlich_core::bazi::PillarKind::Year => "year".to_string(),
        amlich_core::bazi::PillarKind::Month => "month".to_string(),
        amlich_core::bazi::PillarKind::Day => "day".to_string(),
        amlich_core::bazi::PillarKind::Hour => "hour".to_string(),
    }
}

impl From<&amlich_core::bazi::types::KhongVongAnalysis> for KhongVongAnalysisDto {
    fn from(value: &amlich_core::bazi::types::KhongVongAnalysis) -> Self {
        Self {
            entries: value
                .entries
                .iter()
                .map(|e| KhongVongPillarEntryDto {
                    pillar: pillar_kind_to_string(e.pillar),
                    void_pair: KhongVongPairDto {
                        branch_indices: e.void_pair.branch_indices,
                        branch_names: e.void_pair.branch_names.clone(),
                    },
                    hits: e.hits.iter().map(|k| pillar_kind_to_string(*k)).collect(),
                })
                .collect(),
            evidence: RuleEvidenceDto::from(&value.evidence),
        }
    }
}

fn than_sat_source_to_string(source: &amlich_core::bazi::ThanSatSource) -> String {
    match source {
        amlich_core::bazi::ThanSatSource::DayStem => "day_stem".to_string(),
        amlich_core::bazi::ThanSatSource::YearBranch => "year_branch".to_string(),
        amlich_core::bazi::ThanSatSource::DayBranch => "day_branch".to_string(),
        amlich_core::bazi::ThanSatSource::MonthBranch => "month_branch".to_string(),
    }
}

impl From<&amlich_core::bazi::types::ThanSatResult> for ThanSatResultDto {
    fn from(value: &amlich_core::bazi::types::ThanSatResult) -> Self {
        Self {
            stars: value
                .stars
                .iter()
                .map(|s| ThanSatEntryDto {
                    name: s.name.clone(),
                    source: than_sat_source_to_string(&s.source),
                    target_branch: s.target_branch,
                    target_branch_name: s.target_branch_name.clone(),
                    present_in: s
                        .present_in
                        .iter()
                        .map(|k| pillar_kind_to_string(*k))
                        .collect(),
                })
                .collect(),
            evidence: RuleEvidenceDto::from(&value.evidence),
        }
    }
}

// ---------------------------------------------------------------------------
// amlich-mwbp.6: Canonical PersonalDayAssessment DTO projection.
// ---------------------------------------------------------------------------

fn capability_tier_to_dto(tier: amlich_core::BirthDataTier) -> BirthDataTierDto {
    match tier {
        amlich_core::BirthDataTier::Anonymous => BirthDataTierDto::Anonymous,
        amlich_core::BirthDataTier::Date => BirthDataTierDto::Date,
        amlich_core::BirthDataTier::Datetime => BirthDataTierDto::Datetime,
    }
}

fn polarity_to_string(polarity: amlich_core::assessment::ContributionPolarity) -> String {
    match polarity {
        amlich_core::assessment::ContributionPolarity::Favorable => "favorable".to_string(),
        amlich_core::assessment::ContributionPolarity::Avoid => "avoid".to_string(),
        amlich_core::assessment::ContributionPolarity::Neutral => "neutral".to_string(),
        amlich_core::assessment::ContributionPolarity::Info => "info".to_string(),
    }
}

fn axis_outcome_to_dto(
    outcome: &amlich_core::assessment::AxisOutcome,
) -> crate::dto::PersonalDayAxisOutcomeDto {
    crate::dto::PersonalDayAxisOutcomeDto {
        axis: outcome.axis.as_str().to_string(),
        score: outcome.score,
        verdict: outcome.verdict.clone(),
        unavailable_reason: outcome.unavailable_reason.clone(),
    }
}

impl From<&amlich_core::assessment::AxisOutcome> for crate::dto::PersonalDayAxisOutcomeDto {
    fn from(outcome: &amlich_core::assessment::AxisOutcome) -> Self {
        axis_outcome_to_dto(outcome)
    }
}

impl From<&amlich_core::assessment::PersonalDayAssessment>
    for crate::dto::PersonalDayAssessmentDto
{
    fn from(value: &amlich_core::assessment::PersonalDayAssessment) -> Self {
        let sections: Vec<crate::dto::UnavailableSectionDto> = value
            .unavailable_sections
            .iter()
            .map(|s| crate::dto::UnavailableSectionDto {
                section: s.section.clone(),
                reason: s.reason.clone(),
                required_fields: s.required_fields.clone(),
            })
            .collect();

        let axes = crate::dto::PersonalDayAxesDto {
            generic_day_quality: axis_outcome_to_dto(&value.axes.generic_day_quality),
            intent_fit: axis_outcome_to_dto(&value.axes.intent_fit),
            personal_alignment: axis_outcome_to_dto(&value.axes.personal_alignment),
            annual_pressure: axis_outcome_to_dto(&value.axes.annual_pressure),
            evidence_coverage: axis_outcome_to_dto(&value.axes.evidence_coverage),
        };

        let decision = crate::dto::PersonalDayDecisionDto {
            bucket: value.decision.bucket.as_str().to_string(),
            confidence: format!("{:?}", value.decision.confidence).to_lowercase(),
            semantic: value.decision.semantic.clone(),
            primary_conclusion: value.decision.primary_conclusion.clone(),
            decision_score: value.decision.decision_score,
            context_is_clear: value.decision.context_is_clear,
        };

        let contributions: Vec<crate::dto::PersonalDayContributionDto> = value
            .contributions
            .iter()
            .map(|c| crate::dto::PersonalDayContributionDto {
                contribution_id: c.contribution_id.clone(),
                axis: c.axis.as_str().to_string(),
                polarity: polarity_to_string(c.polarity),
                strength: c.strength,
                policy_id: c.policy_id.clone(),
                policy_version: c.policy_version.clone(),
                ruleset_id: c.ruleset_id.clone(),
                ruleset_version: c.ruleset_version.clone(),
                source_family: c.source_evidence.source_family.clone(),
                source_id: c.source_evidence.source_id.clone(),
                method: c.source_evidence.method.clone(),
                note: c.note.clone(),
            })
            .collect();

        let normalized = crate::dto::PersonalDayNormalizedBirthDto {
            day: value.normalized_birth.day,
            month: value.normalized_birth.month,
            year: value.normalized_birth.year,
            has_time: value.normalized_birth.has_time,
            has_gender: value.normalized_birth.has_gender,
            has_location: value.normalized_birth.has_location,
            has_solar_time_policy: value.normalized_birth.has_solar_time_policy,
        };

        let evidence = crate::dto::PersonalDayEvidenceDto {
            has_chart: value.evidence.has_chart,
            has_analysis: value.evidence.has_analysis,
            has_yearly_han: value.evidence.has_yearly_han,
            has_kua: value.evidence.has_kua,
            recommendation_count: value.evidence.recommendation_count,
        };

        Self {
            ruleset_id: value.ruleset_id.clone(),
            ruleset_version: value.ruleset_version.clone(),
            policy_id: value.policy_id.clone(),
            policy_version: value.policy_version.clone(),
            profile: value.profile.clone(),
            intent: value.intent.event_kind().to_string(),
            capability_tier: capability_tier_to_dto(value.capability_tier),
            normalized_birth: normalized,
            axes,
            decision,
            contributions,
            unavailable_sections: sections,
            evidence,
        }
    }
}
