#[derive(Debug, Clone, PartialEq)]
pub struct InitiationOpeningVector {
    pub support: f32,
    pub resistance: f32,
    pub stability: f32,
    pub personal_alignment: f32,
    pub timing_fit: f32,
    pub context_clarity: f32,
    pub strongest_support_id: Option<String>,
    pub strongest_support_note: Option<String>,
    pub strongest_resistance_id: Option<String>,
    pub strongest_resistance_note: Option<String>,
}
