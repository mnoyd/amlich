#[derive(Debug, Clone, PartialEq)]
pub struct AnalysisEnvelope<Facts, Metrics, Advisory> {
    pub facts: Facts,
    pub metrics: Metrics,
    pub advisory: Advisory,
    pub confidence: f32,
    pub warnings: Vec<String>,
}

impl<Facts, Metrics, Advisory> AnalysisEnvelope<Facts, Metrics, Advisory> {
    pub fn new(
        facts: Facts,
        metrics: Metrics,
        advisory: Advisory,
        confidence: f32,
        warnings: Vec<String>,
    ) -> Self {
        Self {
            facts,
            metrics,
            advisory,
            confidence,
            warnings,
        }
    }
}
