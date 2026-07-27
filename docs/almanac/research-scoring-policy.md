# Research: Explainable scoring policy for `amlich-core`

**Date:** 2026-07-27  
**Scope:** Personal-day assessment (the five existing axes), not replacement of the upstream Bazi/calendar rules.

## Executive conclusion

Use a **policy-versioned additive MCDA model** over a typed, normalized feature vector, with explicit interaction features and a separate constraint/veto layer. Preserve each feature's source, rule, policy version, and derivation in the calculation trace. Do not use a neural network or a general tensor for the first implementation: the repository needs deterministic behavior, reviewable rules, and stable explanations, while there is no reviewed outcome dataset from which to learn parameters.

This is a design recommendation synthesized from the sources below; the sources do **not** provide authoritative numeric weights for Vietnamese calendrical practice. Numeric weights must be an explicitly labeled product/domain policy and tested for sensitivity.

## Findings from high-trust sources

### 1. Weighted additive aggregation is an appropriate baseline

The weighted-sum model (WSM/SAW) is a standard multi-attribute decision method: normalize criterion values, multiply by criterion weights, and aggregate. A recent reference chapter describes WSM as useful for structured problems with a manageable number of criteria, emphasizing simplicity and transparency, while also warning that more complex settings can produce ranking inconsistencies. It explicitly recommends sensitivity analysis of criterion weights. [WSM and WPM: Weighted sum model and weighted product model for multi-attribute decision-making (2026, DOI)](https://doi.org/10.1016/B978-0-443-33275-3.00091-9)

For `amlich-core`, this supports:

```text
normalized feature vector x
  -> sparse feature-to-axis weights W(policy, intent)
  -> axis vector a = W x
  -> intent-specific aggregation of available axes
```

Keep the five axis scores before producing a single recommendation bucket; that makes the output inspectable and allows intent-specific priorities.

### 2. Weights are judgments, not discovered facts

Sensitivity analysis research treats the output as dependent on uncertain criterion weights and identifies the criteria whose perturbation changes rankings. One study evaluates WSM and other MCDA methods, finding that robustness should be checked by varying weights and identifying tolerable change intervals. [Dispersion of relative importance values contributes to ranking uncertainty (2018, DOI)](https://doi.org/10.1016/j.eswa.2018.05.048)

Therefore:

- store `policy_id` and `policy_version` with every assessment;
- run deterministic perturbation tests (for example, each weight ±10% and normalized again) and Monte Carlo draws over an allowed interval;
- report whether the recommendation bucket is stable, borderline, or weight-sensitive;
- never describe initial weights as empirically validated until reviewed examples exist.

### 3. Hard constraints should not be allowed to cancel through addition

Outranking methods such as ELECTRE model a **veto threshold** separately from criterion importance. The literature describes veto-related parameters as a distinct part of the outranking relation, and threshold selection as an elicitation problem rather than an ordinary score weight. [Inferring ELECTRE’s veto-related parameters from outranking examples (2006, DOI)](https://doi.org/10.1016/j.ejor.2004.09.019); [Choosing realistic indifference, preference and veto thresholds for ELECTRE (1997, DOI)](https://doi.org/10.1016/S0377-2217(96)00160-0)

For this project, implement:

```text
evaluate_constraints(facts, intent) -> zero or more explicit veto/caution events
if hard veto exists: return Avoid/Blocked with evidence
else: aggregate ordinary contributions with WSM
```

An arbitrary strength cutoff (for example `strength >= 0.8`) is not itself a domain-justified veto. Veto status should be a named rule with its own source and policy version.

### 4. Provenance should be a first-class calculation output

The W3C PROV recommendation models provenance using entities, activities, and agents, and supports links such as `wasGeneratedBy`, `wasDerivedFrom`, and `hadPrimarySource`; it explicitly includes versioning, reproducibility, and derivation as provenance goals. [PROV-O: The PROV Ontology (W3C Recommendation)](https://www.w3.org/TR/prov-o/); [PROV Overview](https://www.w3.org/TR/prov-overview/)

The European Commission’s trustworthy-AI guidance likewise treats traceability, explainability, and auditability as linked requirements: document data, processes, algorithms, and decisions so errors can be understood and prevented. [Requirements of Trustworthy AI — Transparency](https://ec.europa.eu/futurium/en/ai-alliance-consultation/guidelines/1.html)

The current `DecisionContribution` shape is aligned with this direction. The v2 scorer should additionally emit a calculation trace containing, at minimum:

- stable feature ID and normalized value;
- contribution polarity and strength;
- feature-to-axis weight and interaction term (if any);
- rule/source identifier and derivation method;
- policy ID/version and input availability;
- axis subtotal and final aggregation step;
- veto/constraint events separately from additive contributions.

The semantic graph should consume this trace; it should not silently recompute scores.

### 5. Explainability is a system property, not a post-hoc story

NIST identifies explainability, interpretability, accountability, and transparency as distinct but mutually supporting trustworthiness characteristics. Its AI RMF materials state that explanations should communicate why a system made a prediction or recommendation and that provenance contributes to resilience and accountability. [NIST AI Research — Explainability](https://www.nist.gov/artificial-intelligence/ai-research-explainability); [NIST AI RMF — Trustworthy and Responsible AI characteristics](https://airc.nist.gov/airmf-resources/airmf/3-sec-characteristics/)

For deterministic rules, this means exposing the actual additive terms and constraints, not generating a natural-language explanation that can diverge from the calculation.

## Recommended v2 shape

```rust
pub struct AssessmentPolicy {
    pub id: String,
    pub version: String,
    pub feature_axis_weights: SparseWeightTable,
    pub intent_axis_weights: IntentWeightTable,
    pub interactions: Vec<InteractionRule>,
    pub constraints: Vec<ConstraintRule>,
}

pub struct AssessmentTrace {
    pub features: Vec<FeatureObservation>,
    pub contributions: Vec<ContributionTrace>,
    pub constraints: Vec<ConstraintEvent>,
    pub axis_scores: AssessmentAxes,
    pub sensitivity: SensitivitySummary,
}
```

The policy should normalize weights per aggregation stage, mask unavailable axes rather than treating missing data as neutral evidence, and retain the uncollapsed axis vector. A general multidimensional tensor is not justified until there is evidence of high-order interactions and a reviewed dataset; explicit, sparse interaction rules are easier to calibrate and explain.

## What online research can and cannot decide

Research can validate MCDA mechanics, provenance vocabulary, and the distinction between ordinary preference and veto/constraint semantics. It cannot settle which traditional rule is authoritative or assign objectively correct numeric weights. Those require the repository’s source audit, explicit domain-policy decisions, and sensitivity/parity tests. The implementation should label the first policy `experimental` (or equivalent) until those reviews occur.

## Sources

- [WSM and WPM (2026)](https://doi.org/10.1016/B978-0-443-33275-3.00091-9)
- [Weight sensitivity and ranking uncertainty (2018)](https://doi.org/10.1016/j.eswa.2018.05.048)
- [ELECTRE veto-parameter inference (2006)](https://doi.org/10.1016/j.ejor.2004.09.019)
- [ELECTRE threshold selection (1997)](https://doi.org/10.1016/S0377-2217(96)00160-0)
- [W3C PROV-O Recommendation](https://www.w3.org/TR/prov-o/)
- [W3C PROV Overview](https://www.w3.org/TR/prov-overview/)
- [European Commission trustworthy-AI requirements](https://ec.europa.eu/futurium/en/ai-alliance-consultation/guidelines/1.html)
- [NIST Explainability research](https://www.nist.gov/artificial-intelligence/ai-research-explainability)
- [NIST AI RMF characteristics](https://airc.nist.gov/airmf-resources/airmf/3-sec-characteristics/)
