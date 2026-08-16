use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeConcept {
    DayCanchi,
    MonthCanchi,
    YearCanchi,
    SolarTerm,
    HourCanchi,
    Truc,
    DayDeity,
    NaAm,
    Star,
    Element,
    Direction,
    PersonalAlignment,
    InteractionSignal,
    Recommendation,
    Taboo,
    ChartPillar,
    AxisSignal,
    XungHop,
    HoangDaoHour,
    DayPersonMatrix,
    PersonalHourMatrix,
    ElementResonanceMatrix,
    DirectionMergeMatrix,
    DomainDayBoostMatrix,
    InteractionRow,
    TenGodRelation,
    BranchRelationNode,
    ElementRelationNode,
    DirectionSignalNode,
    HourSlot,
    Activity,
    RecommendationHit,
    RecommendationLayer,
    RecommendationSummary,
    Ritual,
    FlyingStar,
    Offering,
    Hexagram,
    // amlich-8tdm: per-feature observation node in the personal-day
    // assessment trace graph. Carries polarity, strength, contribution_id,
    // and source evidence so explanations describe the actual feature
    // projection (and not a parallel recomputation).
    AssessmentFeature,
    // amlich-8tdm: aggregate decision node in the personal-day
    // assessment trace graph. Carries bucket, decision_score, policy_id,
    // policy_version, axis weights, and provenance so explanations can
    // name the verdict the policy actually produced.
    AssessmentDecision,
    // v1.10 (amlich-l2zc.3, EXPLAIN-01) — Traditional Channel node from
    // `十二經納地支` (shi-er-jing-na-di-zhi). Carries the verbatim
    // Chinese channel identity and the bilingual vi/en labels; never
    // carries a physiological-flow or organ-performance claim (per
    // LH-DIV-06). Distinct from `Direction` / `Element` so the schema
    // rejects accidental biomedicalization.
    TraditionalChannel,
    // v1.10 (amlich-l2zc.3, EXPLAIN-01) — Seasonal Profile node from
    // `四氣調神大論` (huangdi-neijing-suwen). Carries one of four
    // seasonal cultivation profiles (spring/summer/autumn/winter);
    // the term-to-season join is the `JoinedByTermToSeason` composite
    // edge, never asserted as a 24-term regimen (LH-DIV-04).
    SeasonalProfile,
}

impl NodeConcept {
    pub fn label(&self) -> ConceptLabel {
        match self {
            Self::DayCanchi => ConceptLabel::DayCanchi,
            Self::MonthCanchi => ConceptLabel::MonthCanchi,
            Self::YearCanchi => ConceptLabel::YearCanchi,
            Self::SolarTerm => ConceptLabel::SolarTerm,
            Self::HourCanchi => ConceptLabel::HourCanchi,
            Self::Truc => ConceptLabel::Truc,
            Self::DayDeity => ConceptLabel::DayDeity,
            Self::NaAm => ConceptLabel::NaAm,
            Self::Star => ConceptLabel::Star,
            Self::Element => ConceptLabel::Element,
            Self::Direction => ConceptLabel::Direction,
            Self::PersonalAlignment => ConceptLabel::PersonalAlignment,
            Self::InteractionSignal => ConceptLabel::InteractionSignal,
            Self::Recommendation => ConceptLabel::Recommendation,
            Self::Taboo => ConceptLabel::Taboo,
            Self::ChartPillar => ConceptLabel::ChartPillar,
            Self::AxisSignal => ConceptLabel::AxisSignal,
            Self::XungHop => ConceptLabel::XungHop,
            Self::HoangDaoHour => ConceptLabel::HoangDaoHour,
            Self::DayPersonMatrix => ConceptLabel::DayPersonMatrix,
            Self::PersonalHourMatrix => ConceptLabel::PersonalHourMatrix,
            Self::ElementResonanceMatrix => ConceptLabel::ElementResonanceMatrix,
            Self::DirectionMergeMatrix => ConceptLabel::DirectionMergeMatrix,
            Self::DomainDayBoostMatrix => ConceptLabel::DomainDayBoostMatrix,
            Self::InteractionRow => ConceptLabel::InteractionRow,
            Self::TenGodRelation => ConceptLabel::TenGodRelation,
            Self::BranchRelationNode => ConceptLabel::BranchRelationNode,
            Self::ElementRelationNode => ConceptLabel::ElementRelationNode,
            Self::DirectionSignalNode => ConceptLabel::DirectionSignalNode,
            Self::HourSlot => ConceptLabel::HourSlot,
            Self::Activity => ConceptLabel::Activity,
            Self::RecommendationHit => ConceptLabel::RecommendationHit,
            Self::RecommendationLayer => ConceptLabel::RecommendationLayer,
            Self::RecommendationSummary => ConceptLabel::RecommendationSummary,
            Self::Ritual => ConceptLabel::Ritual,
            Self::FlyingStar => ConceptLabel::FlyingStar,
            Self::Offering => ConceptLabel::Offering,
            Self::Hexagram => ConceptLabel::Hexagram,
            Self::AssessmentFeature => ConceptLabel::AssessmentFeature,
            Self::AssessmentDecision => ConceptLabel::AssessmentDecision,
            Self::TraditionalChannel => ConceptLabel::TraditionalChannel,
            Self::SeasonalProfile => ConceptLabel::SeasonalProfile,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EdgeConcept {
    Resonates,
    Conflicts,
    Conditions,
    Supports,
    Weakens,
    Overrides,
    Composes,
    Projects,
    Derives,
    HasMatrix,
    HasRow,
    RelatesTo,
    Evaluates,
    InteractsWith,
    HasTenGodRelation,
    HasBranchRelation,
    HasElementRelation,
    BestFor,
    Recommends,
    AdvisesAgainst,
    ContributesTo,
    OriginatesFrom,
    TargetsActivity,
    ProducedByLayer,
    Aggregates,
    PrescribedFor,
    OccupiesPalace,
    CarriesElement,
    RecommendsOffering,
    LocatedAt,
    Transforms,
    // v1.10 (amlich-l2zc.3, EXPLAIN-01) — association edge between a
    // `TraditionalChannel` and the `HourCanchi` it is historically
    // associated with (per the `shi-er-jing-na-di-zhi` corpus). The
    // edge label is `associated_with_hour_branch` (neutral historical
    // association wording) — never `flow_through`, `peak_at`, or any
    // physiological-claim concept (LH-DIV-02/06).
    AssociatedWithHourBranch,
    // v1.10 (amlich-l2zc.3, EXPLAIN-01) — composite edge from a
    // `SeasonalProfile` back to the day root, representing the Amlich
    // term-to-season join (`rule.composite.seasonal_wellness`,
    // LH-DIV-04). Carries the SourceFamily::Derived family on the
    // composite envelope.
    JoinedByTermToSeason,
}

impl EdgeConcept {
    pub fn label(&self) -> ConceptLabel {
        match self {
            Self::Resonates => ConceptLabel::Resonates,
            Self::Conflicts => ConceptLabel::Conflicts,
            Self::Conditions => ConceptLabel::Conditions,
            Self::Supports => ConceptLabel::Supports,
            Self::Weakens => ConceptLabel::Weakens,
            Self::Overrides => ConceptLabel::Overrides,
            Self::Composes => ConceptLabel::Composes,
            Self::Projects => ConceptLabel::Projects,
            Self::Derives => ConceptLabel::Derives,
            Self::HasMatrix => ConceptLabel::HasMatrix,
            Self::HasRow => ConceptLabel::HasRow,
            Self::RelatesTo => ConceptLabel::RelatesTo,
            Self::Evaluates => ConceptLabel::Evaluates,
            Self::InteractsWith => ConceptLabel::InteractsWith,
            Self::HasTenGodRelation => ConceptLabel::HasTenGodRelation,
            Self::HasBranchRelation => ConceptLabel::HasBranchRelation,
            Self::HasElementRelation => ConceptLabel::HasElementRelation,
            Self::BestFor => ConceptLabel::BestFor,
            Self::Recommends => ConceptLabel::Recommends,
            Self::AdvisesAgainst => ConceptLabel::AdvisesAgainst,
            Self::ContributesTo => ConceptLabel::ContributesTo,
            Self::OriginatesFrom => ConceptLabel::OriginatesFrom,
            Self::TargetsActivity => ConceptLabel::TargetsActivity,
            Self::ProducedByLayer => ConceptLabel::ProducedByLayer,
            Self::Aggregates => ConceptLabel::Aggregates,
            Self::PrescribedFor => ConceptLabel::PrescribedFor,
            Self::OccupiesPalace => ConceptLabel::OccupiesPalace,
            Self::CarriesElement => ConceptLabel::CarriesElement,
            Self::RecommendsOffering => ConceptLabel::RecommendsOffering,
            Self::LocatedAt => ConceptLabel::LocatedAt,
            Self::Transforms => ConceptLabel::Transforms,
            Self::AssociatedWithHourBranch => ConceptLabel::AssociatedWithHourBranch,
            Self::JoinedByTermToSeason => ConceptLabel::JoinedByTermToSeason,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConceptLabel {
    DayCanchi,
    MonthCanchi,
    YearCanchi,
    SolarTerm,
    HourCanchi,
    Truc,
    DayDeity,
    NaAm,
    Star,
    Element,
    Direction,
    PersonalAlignment,
    InteractionSignal,
    Recommendation,
    Taboo,
    ChartPillar,
    AxisSignal,
    XungHop,
    HoangDaoHour,
    DayPersonMatrix,
    PersonalHourMatrix,
    ElementResonanceMatrix,
    DirectionMergeMatrix,
    DomainDayBoostMatrix,
    InteractionRow,
    TenGodRelation,
    BranchRelationNode,
    ElementRelationNode,
    DirectionSignalNode,
    HourSlot,
    Activity,
    RecommendationHit,
    RecommendationLayer,
    RecommendationSummary,
    Resonates,
    Conflicts,
    Conditions,
    Supports,
    Weakens,
    Overrides,
    Composes,
    Projects,
    Derives,
    HasMatrix,
    HasRow,
    RelatesTo,
    Evaluates,
    InteractsWith,
    HasTenGodRelation,
    HasBranchRelation,
    HasElementRelation,
    BestFor,
    Recommends,
    AdvisesAgainst,
    ContributesTo,
    OriginatesFrom,
    TargetsActivity,
    ProducedByLayer,
    Aggregates,
    Ritual,
    FlyingStar,
    Offering,
    PrescribedFor,
    OccupiesPalace,
    CarriesElement,
    RecommendsOffering,
    Hexagram,
    LocatedAt,
    Transforms,
    AssessmentFeature,
    AssessmentDecision,
    TraditionalChannel,
    SeasonalProfile,
    AssociatedWithHourBranch,
    JoinedByTermToSeason,
}

impl ConceptLabel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DayCanchi => "day_canchi",
            Self::MonthCanchi => "month_canchi",
            Self::YearCanchi => "year_canchi",
            Self::SolarTerm => "solar_term",
            Self::HourCanchi => "hour_canchi",
            Self::Truc => "truc",
            Self::DayDeity => "day_deity",
            Self::NaAm => "na_am",
            Self::Star => "star",
            Self::Element => "element",
            Self::Direction => "direction",
            Self::PersonalAlignment => "personal_alignment",
            Self::InteractionSignal => "interaction_signal",
            Self::Recommendation => "recommendation",
            Self::Taboo => "taboo",
            Self::ChartPillar => "chart_pillar",
            Self::AxisSignal => "axis_signal",
            Self::XungHop => "xung_hop",
            Self::HoangDaoHour => "hoang_dao_hour",
            Self::DayPersonMatrix => "day_person_matrix",
            Self::PersonalHourMatrix => "personal_hour_matrix",
            Self::ElementResonanceMatrix => "element_resonance_matrix",
            Self::DirectionMergeMatrix => "direction_merge_matrix",
            Self::DomainDayBoostMatrix => "domain_day_boost_matrix",
            Self::InteractionRow => "interaction_row",
            Self::TenGodRelation => "ten_god_relation",
            Self::BranchRelationNode => "branch_relation_node",
            Self::ElementRelationNode => "element_relation_node",
            Self::DirectionSignalNode => "direction_signal_node",
            Self::HourSlot => "hour_slot",
            Self::Activity => "activity",
            Self::RecommendationHit => "recommendation_hit",
            Self::RecommendationLayer => "recommendation_layer",
            Self::RecommendationSummary => "recommendation_summary",
            Self::Resonates => "resonates",
            Self::Conflicts => "conflicts",
            Self::Conditions => "conditions",
            Self::Supports => "supports",
            Self::Weakens => "weakens",
            Self::Overrides => "overrides",
            Self::Composes => "composes",
            Self::Projects => "projects",
            Self::Derives => "derives",
            Self::HasMatrix => "has_matrix",
            Self::HasRow => "has_row",
            Self::RelatesTo => "relates_to",
            Self::Evaluates => "evaluates",
            Self::InteractsWith => "interacts_with",
            Self::HasTenGodRelation => "has_ten_god_relation",
            Self::HasBranchRelation => "has_branch_relation",
            Self::HasElementRelation => "has_element_relation",
            Self::BestFor => "best_for",
            Self::Recommends => "recommends",
            Self::AdvisesAgainst => "advises_against",
            Self::ContributesTo => "contributes_to",
            Self::OriginatesFrom => "originates_from",
            Self::TargetsActivity => "targets_activity",
            Self::ProducedByLayer => "produced_by_layer",
            Self::Aggregates => "aggregates",
            Self::Ritual => "ritual",
            Self::FlyingStar => "flying_star",
            Self::Offering => "offering",
            Self::PrescribedFor => "prescribed_for",
            Self::OccupiesPalace => "occupies_palace",
            Self::CarriesElement => "carries_element",
            Self::RecommendsOffering => "recommends_offering",
            Self::Hexagram => "hexagram",
            Self::LocatedAt => "located_at",
            Self::Transforms => "transforms",
            Self::AssessmentFeature => "assessment_feature",
            Self::AssessmentDecision => "assessment_decision",
            Self::TraditionalChannel => "traditional_channel",
            Self::SeasonalProfile => "seasonal_profile",
            Self::AssociatedWithHourBranch => "associated_with_hour_branch",
            Self::JoinedByTermToSeason => "joined_by_term_to_season",
        }
    }
}

pub struct GraphOntology;

impl GraphOntology {
    pub fn node_concepts() -> &'static [NodeConcept] {
        &[
            NodeConcept::DayCanchi,
            NodeConcept::MonthCanchi,
            NodeConcept::YearCanchi,
            NodeConcept::SolarTerm,
            NodeConcept::HourCanchi,
            NodeConcept::Truc,
            NodeConcept::DayDeity,
            NodeConcept::NaAm,
            NodeConcept::Star,
            NodeConcept::Element,
            NodeConcept::Direction,
            NodeConcept::PersonalAlignment,
            NodeConcept::InteractionSignal,
            NodeConcept::Recommendation,
            NodeConcept::Taboo,
            NodeConcept::ChartPillar,
            NodeConcept::AxisSignal,
            NodeConcept::XungHop,
            NodeConcept::HoangDaoHour,
            NodeConcept::DayPersonMatrix,
            NodeConcept::PersonalHourMatrix,
            NodeConcept::ElementResonanceMatrix,
            NodeConcept::DirectionMergeMatrix,
            NodeConcept::DomainDayBoostMatrix,
            NodeConcept::InteractionRow,
            NodeConcept::TenGodRelation,
            NodeConcept::BranchRelationNode,
            NodeConcept::ElementRelationNode,
            NodeConcept::DirectionSignalNode,
            NodeConcept::HourSlot,
            NodeConcept::Activity,
            NodeConcept::RecommendationHit,
            NodeConcept::RecommendationLayer,
            NodeConcept::RecommendationSummary,
            NodeConcept::Ritual,
            NodeConcept::FlyingStar,
            NodeConcept::Offering,
            NodeConcept::Hexagram,
            NodeConcept::AssessmentFeature,
            NodeConcept::AssessmentDecision,
            NodeConcept::TraditionalChannel,
            NodeConcept::SeasonalProfile,
        ]
    }

    pub fn edge_concepts() -> &'static [EdgeConcept] {
        &[
            EdgeConcept::Resonates,
            EdgeConcept::Conflicts,
            EdgeConcept::Conditions,
            EdgeConcept::Supports,
            EdgeConcept::Weakens,
            EdgeConcept::Overrides,
            EdgeConcept::Composes,
            EdgeConcept::Projects,
            EdgeConcept::Derives,
            EdgeConcept::HasMatrix,
            EdgeConcept::HasRow,
            EdgeConcept::RelatesTo,
            EdgeConcept::Evaluates,
            EdgeConcept::InteractsWith,
            EdgeConcept::HasTenGodRelation,
            EdgeConcept::HasBranchRelation,
            EdgeConcept::HasElementRelation,
            EdgeConcept::BestFor,
            EdgeConcept::Recommends,
            EdgeConcept::AdvisesAgainst,
            EdgeConcept::ContributesTo,
            EdgeConcept::OriginatesFrom,
            EdgeConcept::TargetsActivity,
            EdgeConcept::ProducedByLayer,
            EdgeConcept::Aggregates,
            EdgeConcept::PrescribedFor,
            EdgeConcept::OccupiesPalace,
            EdgeConcept::CarriesElement,
            EdgeConcept::RecommendsOffering,
            EdgeConcept::LocatedAt,
            EdgeConcept::Transforms,
            EdgeConcept::AssociatedWithHourBranch,
            EdgeConcept::JoinedByTermToSeason,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v15_concepts_present_in_ontology_slices() {
        let nodes = GraphOntology::node_concepts();
        assert!(
            nodes.contains(&NodeConcept::Ritual),
            "Ritual missing from node_concepts()"
        );
        assert!(
            nodes.contains(&NodeConcept::FlyingStar),
            "FlyingStar missing from node_concepts()"
        );
        let edges = GraphOntology::edge_concepts();
        assert!(
            edges.contains(&EdgeConcept::PrescribedFor),
            "PrescribedFor missing from edge_concepts()"
        );
        assert!(
            edges.contains(&EdgeConcept::OccupiesPalace),
            "OccupiesPalace missing from edge_concepts()"
        );
        assert!(
            edges.contains(&EdgeConcept::CarriesElement),
            "CarriesElement missing from edge_concepts()"
        );
        // Label round-trip sanity:
        assert_eq!(NodeConcept::FlyingStar.label().as_str(), "flying_star");
        assert_eq!(
            EdgeConcept::OccupiesPalace.label().as_str(),
            "occupies_palace"
        );
    }

    // Phase 19 (INT-07): v1.6 Offering + RecommendsOffering concepts present in ontology slices
    #[test]
    fn v16_concepts_present_in_ontology_slices() {
        let nodes = GraphOntology::node_concepts();
        assert!(
            nodes.contains(&NodeConcept::Offering),
            "Offering missing from node_concepts()"
        );
        let edges = GraphOntology::edge_concepts();
        assert!(
            edges.contains(&EdgeConcept::RecommendsOffering),
            "RecommendsOffering missing from edge_concepts()"
        );
        // Label round-trip:
        assert_eq!(NodeConcept::Offering.label().as_str(), "offering");
        assert_eq!(
            EdgeConcept::RecommendsOffering.label().as_str(),
            "recommends_offering"
        );
    }

    // Phase 20 (FND-12): v1.7 Hexagram node + LocatedAt/Transforms edges present in ontology slices
    #[test]
    fn v17_concepts_present_in_ontology_slices() {
        let nodes = GraphOntology::node_concepts();
        assert!(
            nodes.contains(&NodeConcept::Hexagram),
            "Hexagram missing from node_concepts()"
        );
        let edges = GraphOntology::edge_concepts();
        assert!(
            edges.contains(&EdgeConcept::LocatedAt),
            "LocatedAt missing from edge_concepts()"
        );
        assert!(
            edges.contains(&EdgeConcept::Transforms),
            "Transforms missing from edge_concepts()"
        );
        // Label round-trip sanity:
        assert_eq!(NodeConcept::Hexagram.label().as_str(), "hexagram");
        assert_eq!(EdgeConcept::LocatedAt.label().as_str(), "located_at");
        assert_eq!(EdgeConcept::Transforms.label().as_str(), "transforms");
    }

    // amlich-8tdm: v1.8 AssessmentFeature + AssessmentDecision node
    // concepts present in ontology slices. These power the Evidence
    // Graph projection of the personal-day scoring trace.
    #[test]
    fn v18_assessment_trace_concepts_present_in_ontology_slices() {
        let nodes = GraphOntology::node_concepts();
        assert!(
            nodes.contains(&NodeConcept::AssessmentFeature),
            "AssessmentFeature missing from node_concepts()"
        );
        assert!(
            nodes.contains(&NodeConcept::AssessmentDecision),
            "AssessmentDecision missing from node_concepts()"
        );
        // Label round-trip sanity:
        assert_eq!(
            NodeConcept::AssessmentFeature.label().as_str(),
            "assessment_feature"
        );
        assert_eq!(
            NodeConcept::AssessmentDecision.label().as_str(),
            "assessment_decision"
        );
    }

    // amlich-l2zc.3 (v1.10 EXPLAIN-01): TraditionalChannel +
    // SeasonalProfile node concepts and AssociatedWithHourBranch +
    // JoinedByTermToSeason edge concepts present in the ontology slices.
    // These power the semantic-graph projection of the Traditional
    // Wellness Context per ADR-0003 and the bead's "no physiological
    // flow or organ-performance claim" guarantee.
    #[test]
    fn v110_traditional_wellness_concepts_present_in_ontology_slices() {
        let nodes = GraphOntology::node_concepts();
        assert!(
            nodes.contains(&NodeConcept::TraditionalChannel),
            "TraditionalChannel missing from node_concepts()"
        );
        assert!(
            nodes.contains(&NodeConcept::SeasonalProfile),
            "SeasonalProfile missing from node_concepts()"
        );
        let edges = GraphOntology::edge_concepts();
        assert!(
            edges.contains(&EdgeConcept::AssociatedWithHourBranch),
            "AssociatedWithHourBranch missing from edge_concepts()"
        );
        assert!(
            edges.contains(&EdgeConcept::JoinedByTermToSeason),
            "JoinedByTermToSeason missing from edge_concepts()"
        );
        // Label round-trip sanity — schema contract for the public graph
        // surface; consumers depend on these exact snake_case strings.
        assert_eq!(
            NodeConcept::TraditionalChannel.label().as_str(),
            "traditional_channel"
        );
        assert_eq!(
            NodeConcept::SeasonalProfile.label().as_str(),
            "seasonal_profile"
        );
        assert_eq!(
            EdgeConcept::AssociatedWithHourBranch.label().as_str(),
            "associated_with_hour_branch"
        );
        assert_eq!(
            EdgeConcept::JoinedByTermToSeason.label().as_str(),
            "joined_by_term_to_season"
        );
    }
}
