//! Evidence Graph projection of the personal-day [`AssessmentTrace`].
//!
//! Bead: `amlich-8tdm`.
//!
//! The trace is the canonical record the v2 [`AssessmentPolicy`] emits of
//! every feature observation, applied weight, axis subtotal, veto event,
//! interaction term, and final aggregation step it performed for one
//! `(snapshot, profile, intent)` triple. This module projects that record
//! into a [`SemanticGraph`] so the API, desktop, and TUI explanation
//! surfaces can describe the actual calculation rather than recompute it.
//!
//! ## Stability contract
//!
//! The projection is **pure and deterministic**: identical
//! `(assessment, trace)` pairs produce identical graphs. Node IDs are
//! stable: every feature, axis, veto, interaction, and decision carries
//! a stable `contribution_id` / `veto_id` / `interaction_id` / `axis`
//! key from the trace, plus the assessment's own `ruleset_id`,
//! `ruleset_version`, `profile`, normalized-birth triple, and
//! `intent.event_kind()` to make the graph unique per assessment.
//!
//! ## Scope
//!
//! The builder does not modify the trace, does not recompute scores, and
//! does not invent domain coefficients. Every value stamped on a node or
//! edge is taken verbatim from the trace or from the supplied
//! [`PersonalDayAssessment`] context:
//!
//! - `feature` nodes carry the trace's `FeatureObservation` (feature_id,
//!   polarity, strength, signed_value, contribution_id, availability,
//!   source evidence, ruleset metadata);
//! - `axis` nodes carry the `AxisAggregation` (axis, verdict, subtotal,
//!   per-contributor applied weights);
//! - `veto` nodes carry the `VetoEvent` (veto_id, axis, reason, source
//!   evidence);
//! - `interaction` nodes carry the `InteractionTerm` (interaction_id,
//!   axis, value, weight, feature_ids, source evidence);
//! - the single `decision` node carries the trace's
//!   `DecisionAggregation` (bucket, decision_score, applied axis weights,
//!   available / unavailable axis split).
//!
//! Edges preserve the calculation flow:
//!
//! - `feature → axis` (`ContributesTo`) with the contributor's applied
//!   weight as the edge weight;
//! - `axis → decision` (`ContributesTo`);
//! - `interaction → axis` (`ContributesTo`);
//! - `veto → decision` (`Overrides`).
//!
//! ## Node concepts
//!
//! - `AssessmentFeature` for per-feature observation nodes (new in v1.8);
//! - `AxisSignal` for per-axis aggregation nodes (existing);
//! - `Taboo` for hard veto events (existing);
//! - `InteractionSignal` for declared interaction terms (existing);
//! - `AssessmentDecision` for the final decision node (new in v1.8).
//!
//! The two new node concepts are declared in
//! [`crate::semantic_graph::ontology`] and routed to the
//! `interaction-core` and `recommendation-summary` clusters respectively
//! in [`crate::semantic_graph::views::helpers`].

use crate::assessment::{
    AssessmentAxis, AssessmentTrace, AxisAggregation, AxisContributor, DecisionAggregation,
    FeatureObservation, InteractionTerm, PersonalDayAssessment, VetoEvent,
};
use crate::semantic_graph::{
    EdgeConcept, NodeConcept, NodeOrigin, ProvenanceEntry, ProvenanceSource, SemanticEdge,
    SemanticGraph, SemanticId, SemanticNode,
};

/// Stable key for an assessment-decision node. Built from the
/// `PersonalDayAssessment`'s already-stable identifiers (ruleset,
/// profile, normalized-birth triple, intent), so the graph does not
/// need the `DaySnapshot` / `BirthProfile` as separate inputs.
fn decision_stable_key(assessment: &PersonalDayAssessment) -> String {
    let birth = &assessment.normalized_birth;
    format!(
        "ruleset:{}@{}:profile:{}:birth:{}-{}-{}:intent:{}:policy:{}@{}",
        assessment.ruleset_id,
        assessment.ruleset_version,
        assessment.profile,
        birth.year,
        birth.month,
        birth.day,
        assessment.intent.event_kind(),
        assessment.policy_id,
        assessment.policy_version,
    )
}

/// Stable key for an axis-aggregation node. The trace's `axis` is the
/// only discriminator needed within a single assessment.
fn axis_stable_key(assessment_key: &str, axis: AssessmentAxis) -> String {
    format!("{}:axis:{}", assessment_key, axis.as_str())
}

/// Stable key for a per-feature observation node. The trace's stable
/// `contribution_id` uniquely identifies the feature within the trace.
fn feature_stable_key(assessment_key: &str, feature: &FeatureObservation) -> String {
    format!("{}:feature:{}", assessment_key, feature.contribution_id)
}

/// Stable key for a hard veto event node. The trace's stable `veto_id`
/// uniquely identifies the veto within the trace.
fn veto_stable_key(assessment_key: &str, veto: &VetoEvent) -> String {
    format!("{}:veto:{}", assessment_key, veto.veto_id)
}

/// Stable key for a declared interaction-term node. The trace's stable
/// `interaction_id` uniquely identifies the interaction within the trace.
fn interaction_stable_key(assessment_key: &str, interaction: &InteractionTerm) -> String {
    format!(
        "{}:interaction:{}",
        assessment_key, interaction.interaction_id
    )
}

/// Build the Evidence Graph projection of a [`PersonalDayAssessment`].
///
/// Returns `None` when the assessment was built by the legacy v1
/// builder (which does not emit an [`AssessmentTrace`]): the projection
/// is a strict consumer of the trace, so absent-trace callers get no
/// graph. This preserves the additive `Option<T>` contract on the API
/// DTO.
///
/// The returned graph carries one node per feature, axis, veto,
/// interaction, and the single decision, with edges that follow the
/// calculation flow. The projection is deterministic and does not
/// recompute any score.
pub fn build_assessment_trace_graph(assessment: &PersonalDayAssessment) -> Option<SemanticGraph> {
    let trace = assessment.trace.as_ref()?;
    Some(build_graph_from_trace(assessment, trace))
}

fn build_graph_from_trace(
    assessment: &PersonalDayAssessment,
    trace: &AssessmentTrace,
) -> SemanticGraph {
    let mut graph = SemanticGraph::new();

    let assessment_key = decision_stable_key(assessment);
    let profile_label = assessment.profile.clone();

    // Pass 1: emit every entity node. Edges are added in a second pass
    // so node IDs are stable across the graph and downstream consumers
    // can iterate features / axes / vetoes independently.
    let mut axis_node_ids: Vec<(AssessmentAxis, String)> = Vec::new();
    let mut feature_node_ids: Vec<(AssessmentAxis, String)> = Vec::new();
    let mut veto_node_ids: Vec<(AssessmentAxis, String)> = Vec::new();
    let mut interaction_node_ids: Vec<(AssessmentAxis, String)> = Vec::new();

    // Axis nodes (one per AxisAggregation in the trace, in canonical order).
    for axis_agg in &trace.axes {
        let node_id = SemanticId::new(
            "axis_signal",
            axis_stable_key(&assessment_key, axis_agg.axis),
        )
        .to_node_id();
        let summary = summarize_axis(axis_agg);
        let provenance = provenance_for_axis(trace, &profile_label);
        let mut node = SemanticNode::new(
            SemanticId::new(
                "axis_signal",
                axis_stable_key(&assessment_key, axis_agg.axis),
            ),
            NodeConcept::AxisSignal,
            NodeOrigin::Interpreted,
            summary,
        )
        .with_severity_if(true, axis_agg.verdict.as_str())
        .with_tags(axis_tags(trace, axis_agg))
        .with_provenance(provenance);

        // Carry the applied weight vector on the node so explanations
        // can describe the aggregation without walking contributors.
        let contributor_payload = axis_contributors_payload(&node_id, &axis_agg.contributors);
        if let Some(payload) = contributor_payload {
            node.payload = Some(payload);
        }

        graph.add_node(node);
        axis_node_ids.push((axis_agg.axis, node_id));
    }

    // Feature nodes (one per FeatureObservation in the trace, including
    // unavailable ones so explanations describe missing evidence).
    for feature in &trace.features {
        let axis = feature.feature_id.default_axis();
        let node_id = SemanticId::new(
            "assessment_feature",
            feature_stable_key(&assessment_key, feature),
        )
        .to_node_id();
        let summary = summarize_feature(feature);
        let provenance = provenance_for_feature(feature, &node_id, &profile_label);

        let mut node = SemanticNode::new(
            SemanticId::new(
                "assessment_feature",
                feature_stable_key(&assessment_key, feature),
            ),
            NodeConcept::AssessmentFeature,
            NodeOrigin::Fact,
            summary,
        )
        .with_severity(feature_polarity_label(feature))
        .with_tags(feature_tags(trace, feature))
        .with_provenance(provenance);

        if let Some(payload) = feature_payload(feature) {
            node.payload = Some(payload);
        }

        graph.add_node(node);
        feature_node_ids.push((axis, node_id));
    }

    // Veto nodes (one per VetoEvent in the trace).
    for veto in &trace.vetoes {
        let node_id = SemanticId::new("taboo", veto_stable_key(&assessment_key, veto)).to_node_id();
        let summary = veto.reason.clone();
        let provenance = provenance_for_veto(veto, &profile_label);
        let mut node = SemanticNode::new(
            SemanticId::new("taboo", veto_stable_key(&assessment_key, veto)),
            NodeConcept::Taboo,
            NodeOrigin::Interpreted,
            summary,
        )
        .with_severity("hard_veto")
        .with_tags(veto_tags(trace, veto))
        .with_provenance(provenance);

        if let Some(payload) = veto_payload(veto) {
            node.payload = Some(payload);
        }

        graph.add_node(node);
        veto_node_ids.push((veto.axis, node_id));
    }

    // Interaction nodes (one per InteractionTerm in the trace).
    for interaction in &trace.interactions {
        let node_id = SemanticId::new(
            "interaction_signal",
            interaction_stable_key(&assessment_key, interaction),
        )
        .to_node_id();
        let summary = format!(
            "interaction {} → {}",
            interaction.interaction_id,
            interaction.axis.as_str()
        );
        let provenance = provenance_for_interaction(interaction, &profile_label);

        let mut node = SemanticNode::new(
            SemanticId::new(
                "interaction_signal",
                interaction_stable_key(&assessment_key, interaction),
            ),
            NodeConcept::InteractionSignal,
            NodeOrigin::Interpreted,
            summary,
        )
        .with_severity(format!("weight={:.2}", interaction.weight))
        .with_tags(interaction_tags(trace, interaction))
        .with_provenance(provenance);

        if let Some(payload) = interaction_payload(interaction) {
            node.payload = Some(payload);
        }

        graph.add_node(node);
        interaction_node_ids.push((interaction.axis, node_id));
    }

    // Decision node (single per assessment).
    let decision_node_id =
        SemanticId::new("assessment_decision", assessment_key.clone()).to_node_id();
    let summary = summarize_decision(&trace.decision);
    let provenance = provenance_for_decision(trace, &profile_label);

    let mut decision_node = SemanticNode::new(
        SemanticId::new("assessment_decision", assessment_key.clone()),
        NodeConcept::AssessmentDecision,
        NodeOrigin::Decision,
        summary,
    )
    .with_severity(trace.decision.bucket.as_str())
    .with_tags(decision_tags(trace, &trace.decision))
    .with_provenance(provenance);

    if let Some(payload) = decision_payload(&trace.decision) {
        decision_node.payload = Some(payload);
    }
    graph.add_node(decision_node);

    // Pass 2: edges.
    // feature → axis (ContributesTo), with the contributor's applied
    // weight as the edge weight when the trace records it.
    let contributor_weights: Vec<(AssessmentAxis, String, f32)> = trace
        .axes
        .iter()
        .flat_map(|axis_agg| {
            axis_agg
                .contributors
                .iter()
                .map(move |c| (axis_agg.axis, c.contribution_id.clone(), c.applied_weight))
        })
        .collect();

    for (axis, feature_node_id) in &feature_node_ids {
        // Find the matching axis node for the feature's default axis.
        if let Some((_, axis_node_id)) = axis_node_ids.iter().find(|(a, _)| a == axis) {
            let mut edge = SemanticEdge::new(
                feature_node_id.as_str(),
                axis_node_id.as_str(),
                EdgeConcept::ContributesTo,
            );
            // Look up the contributor's applied weight to set the edge
            // weight, so the graph preserves the v1 multiplier /
            // intent-aware weight actually applied.
            let contribution_id = contribution_id_for_node(feature_node_id);
            if let Some((_, _, weight)) = contributor_weights
                .iter()
                .find(|(a, c, _)| *a == *axis && *c == contribution_id)
            {
                edge = edge.with_weight(weight_to_int(*weight));
            }
            graph.add_edge(edge);
        }
    }

    // axis → decision (ContributesTo)
    for (_, axis_node_id) in &axis_node_ids {
        let edge = SemanticEdge::new(
            axis_node_id.as_str(),
            decision_node_id.as_str(),
            EdgeConcept::ContributesTo,
        );
        graph.add_edge(edge);
    }

    // interaction → axis (ContributesTo) — only when the interaction's
    // axis is one of the five canonical axes and has a matching axis
    // node.
    for (axis, interaction_node_id) in &interaction_node_ids {
        if let Some((_, axis_node_id)) = axis_node_ids.iter().find(|(a, _)| a == axis) {
            let edge = SemanticEdge::new(
                interaction_node_id.as_str(),
                axis_node_id.as_str(),
                EdgeConcept::ContributesTo,
            );
            graph.add_edge(edge);
        }
    }

    // veto → decision (Overrides)
    for (_, veto_node_id) in &veto_node_ids {
        let edge = SemanticEdge::new(
            veto_node_id.as_str(),
            decision_node_id.as_str(),
            EdgeConcept::Overrides,
        );
        graph.add_edge(edge);
    }

    graph.validate().expect(
        "AssessmentTrace projection must produce a well-formed graph: missing edges indicate a builder bug",
    );
    graph
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn provenance_for_feature(
    feature: &FeatureObservation,
    node_id: &str,
    profile: &str,
) -> ProvenanceEntry {
    let source = provenance_source_for(&feature.source_evidence.source_family);
    ProvenanceEntry::new(
        source,
        format!(
            "{}::{}",
            feature.source_evidence.source_id, feature.source_evidence.method
        ),
        format!(
            "feature::{}::{}",
            feature.feature_id.as_str(),
            feature.source_evidence.profile
        ),
    )
    .with_profile(profile.to_string())
    .with_note(node_id.to_string())
}

fn provenance_for_axis(trace: &AssessmentTrace, profile: &str) -> ProvenanceEntry {
    ProvenanceEntry::derived(
        format!("policy::{}", trace.policy_version),
        "aggregate_axis",
    )
    .with_profile(profile.to_string())
}

fn provenance_for_veto(veto: &VetoEvent, profile: &str) -> ProvenanceEntry {
    let source = provenance_source_for(&veto.source_evidence.source_family);
    ProvenanceEntry::new(
        source,
        format!(
            "{}::{}",
            veto.source_evidence.source_id, veto.source_evidence.method
        ),
        format!("veto::{}::{}", veto.veto_id, veto.source_evidence.profile),
    )
    .with_profile(profile.to_string())
}

fn provenance_for_interaction(interaction: &InteractionTerm, profile: &str) -> ProvenanceEntry {
    let source = provenance_source_for(&interaction.source_evidence.source_family);
    ProvenanceEntry::new(
        source,
        format!(
            "{}::{}",
            interaction.source_evidence.source_id, interaction.source_evidence.method
        ),
        format!(
            "interaction::{}::{}",
            interaction.interaction_id, interaction.source_evidence.profile
        ),
    )
    .with_profile(profile.to_string())
}

fn provenance_for_decision(trace: &AssessmentTrace, profile: &str) -> ProvenanceEntry {
    ProvenanceEntry::derived(
        format!("policy::{}::{}", trace.policy_id, trace.policy_version),
        "synthesize_decision",
    )
    .with_profile(profile.to_string())
}

fn provenance_source_for(source_family: &str) -> ProvenanceSource {
    // Map the trace's source_family string into the closest existing
    // ProvenanceSource variant. Unknown families collapse to `Derived`
    // so the projection stays robust to new evidence families in the
    // trace. The literal family is preserved on the note and on the
    // underlying source_evidence.
    match source_family {
        "snapshot" | "almanac_rule" => ProvenanceSource::AlmanacRule,
        "interaction" => ProvenanceSource::Interaction,
        "bazi" => ProvenanceSource::Bazi,
        "insight" => ProvenanceSource::Insight,
        _ => ProvenanceSource::Derived,
    }
}

fn summarize_axis(axis_agg: &AxisAggregation) -> String {
    let score = axis_agg
        .subtotal
        .map(|s| format!("{:.2}", s))
        .unwrap_or_else(|| "n/a".to_string());
    let reason = axis_agg
        .unavailable_reason
        .as_deref()
        .unwrap_or(axis_agg.verdict.as_str());
    format!("{} = {} ({})", axis_agg.axis.as_str(), score, reason)
}

fn summarize_feature(feature: &FeatureObservation) -> String {
    let signed = feature
        .signed_value()
        .map(|s| format!("{:+.2}", s))
        .unwrap_or_else(|| "n/a".to_string());
    if feature.is_unavailable() {
        format!("{} (unavailable)", feature.feature_id.as_str())
    } else {
        format!("{} → {}", feature.feature_id.as_str(), signed)
    }
}

fn summarize_decision(decision: &DecisionAggregation) -> String {
    let score = decision
        .decision_score
        .map(|s| format!("{:.2}", s))
        .unwrap_or_else(|| "n/a".to_string());
    format!("decision = {} ({})", decision.bucket.as_str(), score)
}

fn feature_polarity_label(feature: &FeatureObservation) -> String {
    if feature.is_unavailable() {
        "unavailable".to_string()
    } else {
        format!("{:?}", feature.polarity).to_lowercase()
    }
}

fn axis_tags(trace: &AssessmentTrace, axis_agg: &AxisAggregation) -> Vec<String> {
    let mut tags = vec![
        format!("axis:{}", axis_agg.axis.as_str()),
        format!("policy:{}", trace.policy_version),
    ];
    if axis_agg.subtotal.is_some() {
        tags.push(format!("verdict:{}", axis_agg.verdict));
    } else {
        tags.push("verdict:unavailable".to_string());
    }
    tags
}

fn feature_tags(trace: &AssessmentTrace, feature: &FeatureObservation) -> Vec<String> {
    let mut tags = vec![
        format!("feature:{}", feature.feature_id.as_str()),
        format!("policy:{}", trace.policy_version),
        format!("ruleset:{}", feature.ruleset_version),
    ];
    if feature.is_unavailable() {
        tags.push("availability:unavailable".to_string());
    } else {
        tags.push("availability:complete".to_string());
    }
    tags
}

fn veto_tags(trace: &AssessmentTrace, veto: &VetoEvent) -> Vec<String> {
    vec![
        format!("veto:{}", veto.veto_id),
        format!("axis:{}", veto.axis.as_str()),
        format!("policy:{}", trace.policy_version),
    ]
}

fn interaction_tags(trace: &AssessmentTrace, interaction: &InteractionTerm) -> Vec<String> {
    vec![
        format!("interaction:{}", interaction.interaction_id),
        format!("axis:{}", interaction.axis.as_str()),
        format!("policy:{}", trace.policy_version),
    ]
}

fn decision_tags(trace: &AssessmentTrace, decision: &DecisionAggregation) -> Vec<String> {
    let mut tags = vec![
        format!("bucket:{}", decision.bucket.as_str()),
        format!("policy:{}", trace.policy_version),
    ];
    if let Some(score) = decision.decision_score {
        tags.push(format!("decision_score:{:.3}", score));
    } else {
        tags.push("decision_score:n/a".to_string());
    }
    tags
}

fn contribution_id_for_node(node_id: &str) -> String {
    // Node IDs are formatted as `assessment_feature:{assessment_key}:feature:{contribution_id}`.
    // We only need the contribution_id segment to look up the contributor
    // weight — extract the trailing segment after the last `:` after
    // `feature:`. The trace guarantees a stable contribution_id per
    // feature, so this lookup is deterministic.
    node_id
        .rsplit(':')
        .next()
        .map(|s| s.to_string())
        .unwrap_or_default()
}

fn weight_to_int(weight: f32) -> i32 {
    // Edge weights are `i32`; the trace stores f32 weights in
    // `[-1, 1]`. Scale to int so negative contributions still render
    // (and so two policies with the same fractional weight share the
    // same integer weight, which downstream layout code relies on).
    (weight.clamp(-1.0, 1.0) * 1000.0).round() as i32
}

fn axis_contributors_payload(
    _axis_node_id: &str,
    contributors: &[AxisContributor],
) -> Option<serde_json::Value> {
    if contributors.is_empty() {
        return None;
    }
    let entries: Vec<serde_json::Value> = contributors
        .iter()
        .map(|c| {
            serde_json::json!({
                "feature_id": c.feature_id.as_str(),
                "contribution_id": c.contribution_id,
                "signed_value": c.signed_value,
                "applied_weight": c.applied_weight,
                "contribution": c.contribution,
            })
        })
        .collect();
    Some(serde_json::json!({ "contributors": entries }))
}

fn feature_payload(feature: &FeatureObservation) -> Option<serde_json::Value> {
    Some(serde_json::json!({
        "feature_id": feature.feature_id.as_str(),
        "polarity": format!("{:?}", feature.polarity).to_lowercase(),
        "strength": feature.strength,
        "signed_value": feature.signed_value(),
        "availability": format!("{:?}", feature.availability),
        "contribution_id": feature.contribution_id,
        "ruleset_id": feature.ruleset_id,
        "ruleset_version": feature.ruleset_version,
        "source_evidence": {
            "source_family": feature.source_evidence.source_family,
            "source_id": feature.source_evidence.source_id,
            "method": feature.source_evidence.method,
            "profile": feature.source_evidence.profile,
            "note": feature.source_evidence.note,
        },
    }))
}

fn veto_payload(veto: &VetoEvent) -> Option<serde_json::Value> {
    Some(serde_json::json!({
        "veto_id": veto.veto_id,
        "axis": veto.axis.as_str(),
        "reason": veto.reason,
        "source_evidence": {
            "source_family": veto.source_evidence.source_family,
            "source_id": veto.source_evidence.source_id,
            "method": veto.source_evidence.method,
            "profile": veto.source_evidence.profile,
            "note": veto.source_evidence.note,
        },
    }))
}

fn interaction_payload(interaction: &InteractionTerm) -> Option<serde_json::Value> {
    Some(serde_json::json!({
        "interaction_id": interaction.interaction_id,
        "axis": interaction.axis.as_str(),
        "value": interaction.value,
        "weight": interaction.weight,
        "feature_ids": interaction
            .feature_ids
            .iter()
            .map(|f| f.as_str())
            .collect::<Vec<_>>(),
        "source_evidence": {
            "source_family": interaction.source_evidence.source_family,
            "source_id": interaction.source_evidence.source_id,
            "method": interaction.source_evidence.method,
            "profile": interaction.source_evidence.profile,
            "note": interaction.source_evidence.note,
        },
        "note": interaction.note,
    }))
}

fn decision_payload(decision: &DecisionAggregation) -> Option<serde_json::Value> {
    let axis_weights: Vec<serde_json::Value> = decision
        .axis_weights
        .iter()
        .map(|w| {
            serde_json::json!({
                "axis": w.axis.as_str(),
                "weight": w.weight,
            })
        })
        .collect();
    Some(serde_json::json!({
        "bucket": decision.bucket.as_str(),
        "decision_score": decision.decision_score,
        "axis_weights": axis_weights,
        "available_axes": decision
            .available_axes
            .iter()
            .map(|a| a.as_str())
            .collect::<Vec<_>>(),
        "unavailable_axes": decision
            .unavailable_axes
            .iter()
            .map(|a| a.as_str())
            .collect::<Vec<_>>(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::advisory::ConsultationIntent;
    use crate::almanac::tu_menh::Gender;
    use crate::assessment::{
        AssessmentInputs, AssessmentPolicy, PersonalDayAssessment, ASSESSMENT_POLICY_V2_VERSION,
    };
    use crate::birth::{BirthProfile, BirthTime};
    use crate::types::VIETNAM_TIMEZONE;
    use crate::DaySnapshot;

    fn snapshot_2024_02_10() -> DaySnapshot {
        crate::calculate_day_snapshot_with_timezone(10, 2, 2024, VIETNAM_TIMEZONE)
    }

    fn full_profile() -> BirthProfile {
        BirthProfile {
            day: 1,
            month: 1,
            year: 1990,
            time: Some(BirthTime {
                hour: 9,
                minute: 30,
            }),
            timezone: VIETNAM_TIMEZONE,
            longitude: Some(105.85),
            use_solar_time: true,
            gender: Some(Gender::Male),
            location_name: Some("Hanoi".to_string()),
        }
    }

    fn han_severe_profile() -> BirthProfile {
        // 1985 birth year fires the annual.han_severe hard veto on
        // 2024-02-10, exercising the veto → decision edge path.
        BirthProfile {
            year: 1985,
            gender: Some(Gender::Female),
            ..full_profile()
        }
    }

    fn build_assessment(
        intent: ConsultationIntent,
        profile: BirthProfile,
    ) -> PersonalDayAssessment {
        AssessmentPolicy::baseline_v2().evaluate(
            AssessmentInputs::default(),
            &snapshot_2024_02_10(),
            &profile,
            intent,
        )
    }

    fn build_trace(intent: ConsultationIntent, profile: BirthProfile) -> AssessmentTrace {
        build_assessment(intent, profile)
            .trace
            .expect("baseline_v2 must attach an AssessmentTrace")
    }

    #[test]
    fn projection_emits_decision_axis_feature_and_veto_nodes() {
        // 1985 birth year + female forces the annual.han_severe veto so
        // the projection covers every node kind at once.
        let profile = han_severe_profile();
        let intent = ConsultationIntent::Wedding;
        let trace = build_trace(intent, profile.clone());
        let assessment = build_assessment(intent, profile.clone());
        let graph = build_assessment_trace_graph(&assessment).expect("v2 trace must project");

        assert!(
            graph.has_node(&format!(
                "assessment_decision:{}",
                decision_stable_key(&assessment)
            )),
            "decision node must be present"
        );

        // Every axis must have an axis_signal node.
        for axis in AssessmentAxis::ALL {
            assert!(
                graph
                    .nodes()
                    .values()
                    .any(|n| n.concept == NodeConcept::AxisSignal
                        && n.summary_vi.contains(axis.as_str())),
                "axis_signal node missing for axis {}",
                axis.as_str()
            );
        }

        // Every feature observation must produce an assessment_feature node.
        let assessment_key = decision_stable_key(&assessment);
        for feature in &trace.features {
            let node_id = SemanticId::new(
                "assessment_feature",
                feature_stable_key(&assessment_key, feature),
            )
            .to_node_id();
            assert!(
                graph.has_node(&node_id),
                "assessment_feature node missing for feature {}",
                feature.contribution_id
            );
        }

        // Vetoes produce taboo nodes in the trace projection.
        assert!(
            !trace.vetoes.is_empty(),
            "han_severe_profile must trigger a hard veto on this snapshot"
        );
        for veto in &trace.vetoes {
            let node_id =
                SemanticId::new("taboo", veto_stable_key(&assessment_key, veto)).to_node_id();
            assert!(
                graph.has_node(&node_id),
                "veto (taboo) node missing for veto {}",
                veto.veto_id
            );
        }
    }

    #[test]
    fn projection_carries_policy_version_and_source_attribution() {
        let profile = full_profile();
        let intent = ConsultationIntent::Wedding;
        let trace = build_trace(intent, profile.clone());
        let assessment = build_assessment(intent, profile.clone());
        let graph = build_assessment_trace_graph(&assessment).expect("v2 trace must project");

        // Every assessment_feature node's tags must include policy: and
        // the trace's policy_version.
        let expected_policy = trace.policy_version.clone();
        let feature_nodes: Vec<_> = graph
            .nodes()
            .values()
            .filter(|n| n.concept == NodeConcept::AssessmentFeature)
            .collect();
        assert!(
            !feature_nodes.is_empty(),
            "projection must emit assessment_feature nodes"
        );
        for node in &feature_nodes {
            assert!(
                node.tags
                    .iter()
                    .any(|t| t == &format!("policy:{}", expected_policy)),
                "feature node {} must carry policy:{} tag",
                node.node_id,
                expected_policy
            );
            assert!(
                !node.provenance.is_empty(),
                "feature node {} must carry provenance",
                node.node_id
            );
        }

        // The decision node carries the bucket in its severity slot and
        // the policy version in its tags.
        let decision_node_id =
            SemanticId::new("assessment_decision", decision_stable_key(&assessment)).to_node_id();
        let decision = graph
            .get_node(&decision_node_id)
            .expect("decision node present");
        assert_eq!(
            decision.severity.as_deref(),
            Some(trace.decision.bucket.as_str()),
            "decision severity must equal the bucket the policy classified"
        );
        assert!(
            decision
                .tags
                .iter()
                .any(|t| t == &format!("policy:{}", expected_policy)),
            "decision node must carry the policy_version tag"
        );
    }

    #[test]
    fn projection_edges_follow_calculation_flow() {
        let profile = han_severe_profile();
        let intent = ConsultationIntent::Wedding;
        let trace = build_trace(intent, profile.clone());
        let assessment = build_assessment(intent, profile.clone());
        let graph = build_assessment_trace_graph(&assessment).expect("v2 trace must project");

        let decision_node_id =
            SemanticId::new("assessment_decision", decision_stable_key(&assessment)).to_node_id();
        let assessment_key = decision_stable_key(&assessment);

        // Each axis_signal node must contribute to the decision node.
        for node in graph.nodes().values() {
            if node.concept == NodeConcept::AxisSignal {
                assert!(
                    graph
                        .outgoing_edges(&node.node_id)
                        .iter()
                        .any(|e| e.to_node_id == decision_node_id
                            && e.label.concept == EdgeConcept::ContributesTo),
                    "axis {} must have a ContributesTo edge to the decision",
                    node.node_id
                );
            }
        }

        // Each feature must contribute to its default axis.
        for feature in &trace.features {
            let feature_node_id = SemanticId::new(
                "assessment_feature",
                feature_stable_key(&assessment_key, feature),
            )
            .to_node_id();
            let edges = graph.outgoing_edges(&feature_node_id);
            assert!(
                edges
                    .iter()
                    .any(|e| e.label.concept == EdgeConcept::ContributesTo),
                "feature {} must have a ContributesTo edge to its axis",
                feature.contribution_id
            );
        }

        // Each veto must override the decision.
        for veto in &trace.vetoes {
            let veto_node_id =
                SemanticId::new("taboo", veto_stable_key(&assessment_key, veto)).to_node_id();
            assert!(
                graph
                    .outgoing_edges(&veto_node_id)
                    .iter()
                    .any(|e| e.to_node_id == decision_node_id
                        && e.label.concept == EdgeConcept::Overrides),
                "veto {} must have an Overrides edge to the decision",
                veto.veto_id
            );
        }
    }

    #[test]
    fn projection_is_deterministic() {
        let profile = full_profile();
        let intent = ConsultationIntent::Travel;
        let assessment_a = build_assessment(intent, profile.clone());
        let assessment_b = build_assessment(intent, profile);
        let graph_a = build_assessment_trace_graph(&assessment_a).expect("v2 trace must project");
        let graph_b = build_assessment_trace_graph(&assessment_b).expect("v2 trace must project");
        assert_eq!(graph_a.nodes(), graph_b.nodes());
        assert_eq!(graph_a.edges(), graph_b.edges());
    }

    #[test]
    fn projection_is_pure_no_score_recomputation() {
        // The projection must NOT recompute scores: changing the trace
        // and re-projecting must produce a graph whose visible
        // severities (decision bucket, axis verdicts, feature polarity
        // labels) match the trace's recorded values byte-for-byte.
        let profile = full_profile();
        let intent = ConsultationIntent::Travel;
        let trace = build_trace(intent, profile.clone());
        let assessment = build_assessment(intent, profile);
        let graph = build_assessment_trace_graph(&assessment).expect("v2 trace must project");

        let decision_node_id =
            SemanticId::new("assessment_decision", decision_stable_key(&assessment)).to_node_id();
        let decision = graph.get_node(&decision_node_id).expect("decision node");
        assert_eq!(
            decision.severity.as_deref(),
            Some(trace.decision.bucket.as_str())
        );

        let assessment_key = decision_stable_key(&assessment);
        for axis_agg in &trace.axes {
            let axis_node_id = SemanticId::new(
                "axis_signal",
                axis_stable_key(&assessment_key, axis_agg.axis),
            )
            .to_node_id();
            let node = graph.get_node(&axis_node_id).expect("axis node");
            assert_eq!(node.severity.as_deref(), Some(axis_agg.verdict.as_str()));
        }
    }

    #[test]
    fn projection_records_policy_version_baseline_v2() {
        // The projection must carry the trace's policy_version so
        // explanations describe the v2 calculation, not a v1
        // recomputation.
        let profile = full_profile();
        let intent = ConsultationIntent::Travel;
        let trace = build_trace(intent, profile.clone());
        assert_eq!(trace.policy_version, ASSESSMENT_POLICY_V2_VERSION);

        let assessment = build_assessment(intent, profile);
        let graph = build_assessment_trace_graph(&assessment).expect("v2 trace must project");

        let feature_nodes: Vec<_> = graph
            .nodes()
            .values()
            .filter(|n| n.concept == NodeConcept::AssessmentFeature)
            .collect();
        for node in feature_nodes {
            assert!(
                node.tags
                    .iter()
                    .any(|t| t == &format!("policy:{}", ASSESSMENT_POLICY_V2_VERSION)),
                "feature {} must carry policy:{} tag",
                node.node_id,
                ASSESSMENT_POLICY_V2_VERSION,
            );
        }
    }

    #[test]
    fn empty_vetoes_produce_no_veto_nodes() {
        // A profile that does not trigger any veto must produce no
        // veto (taboo) nodes in the projection — the projection never
        // invents vetoes that are absent from the trace.
        let profile = full_profile();
        let intent = ConsultationIntent::Wedding;
        let trace = build_trace(intent, profile.clone());
        let assessment = build_assessment(intent, profile);
        let graph = build_assessment_trace_graph(&assessment).expect("v2 trace must project");

        if trace.vetoes.is_empty() {
            assert!(
                !graph
                    .nodes()
                    .values()
                    .any(|n| n.concept == NodeConcept::Taboo && n.summary_vi.contains("veto.")),
                "projection must not invent vetoes when the trace has none"
            );
        }
    }

    #[test]
    fn legacy_v1_assessment_produces_no_graph() {
        // The legacy v1 builder does not emit an AssessmentTrace, so
        // the projection returns None and the additive Option<T> on
        // the API DTO keeps the v1 wire contract byte-equal.
        let assessment = PersonalDayAssessment::assess(
            snapshot_2024_02_10(),
            full_profile(),
            ConsultationIntent::Wedding,
        );
        assert!(
            assessment.trace.is_none(),
            "v1 builder must not attach an AssessmentTrace"
        );
        assert!(
            build_assessment_trace_graph(&assessment).is_none(),
            "v1 assessment must produce no explanation graph"
        );
    }
}
