//! v1.10 Phase 02-01 Tier-0 Traditional Wellness Context — four-season
//! cultivation profiles (Tứ khí điều thần / 四氣調神大論).
//!
//! This module is a sibling of `branch_channel` — it does not contribute
//! to Day Assessment, Hour Ranking, or Direction Assessment per
//! ADR-0003. It delivers:
//!
//! - the four source-grounded seasonal routine profiles paraphrased from
//!   *Huangdi Neijing Suwen*, chapter `四氣調神大論篇第二` (corpus at
//!   `crates/amlich-core/data/traditional-wellness/seasonal-cultivation.json`),
//! - the frozen Amlich term-to-season composition that joins all 24
//!   solar terms into those four profiles at the four Lập boundaries
//!   (LH-DIV-04: a transparent presentation join, **never** presented as
//!   a term-specific classical prescription),
//! - strict primitive/composite provenance separation: the astronomical
//!   solar-term evidence keeps its existing engine provenance, the
//!   paraphrase carries `huangdi-neijing-suwen`, and exactly one Derived
//!   composite envelope (`rule.composite.seasonal_wellness`) represents
//!   the join.
//!
//! Plan 02-01 ships the standalone context + enrich helper; bead
//! `amlich-l2zc.3` (Phase 03) projects this through API, terminal,
//! desktop, and semantic-graph surfaces together with the branch-channel
//! track.

use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use crate::reasoning::{ReasoningEvidenceEnvelope, ReasoningEvidenceSourceFamily};
use crate::sources::SOURCE_HUANGDI_NEIJING_SUWEN;
use crate::tietkhi::{get_tiet_khi, SolarTerm};
use crate::ProvenanceEntry;

use super::branch_channel::SourceCitation;
use super::disclaimer::{cultural_information_disclaimer, LocalizedDisclaimer};
use super::divergence::{divergence_by_id, ExternalReviewState};

// ---------------------------------------------------------------------------
// Composite rule id + engine source id
// ---------------------------------------------------------------------------

/// Composite rule identifier carried by the term-to-season join envelope
/// (audit-friendly single named constant; not a corpus source_id — same
/// discipline as `crate::reasoning::direction_composite::
/// COMPOSITE_DIRECTION_CROSS_LINK`). Emitted exactly once per resolved
/// context, with the `Derived` evidence family, and never as a
/// primitive source.
pub const COMPOSITE_SEASONAL_WELLNESS: &str = "rule.composite.seasonal_wellness";

/// Descriptive source id for the existing astronomical solar-term
/// engine (`tietkhi::get_tiet_khi`). This is an engine attribution, not
/// a classical source: the solar-term primitive envelope exists so the
/// astronomical evidence stays clearly separate from
/// `huangdi-neijing-suwen` (SOURCE-01 / LH-DIV-04).
pub const SOLAR_TERM_ENGINE_SOURCE_ID: &str = "amlich-solar-term-engine";

/// The four seasonal-boundary term names, in canonical
/// `tietkhi::TIET_KHI` index order of the year (Lập Xuân = 21,
/// Lập Hạ = 3, Lập Thu = 9, Lập Đông = 15). The corpus metadata must
/// agree with this array verbatim.
pub const SEASONAL_BOUNDARY_TERM_NAMES: [&str; 4] = ["Lập Xuân", "Lập Hạ", "Lập Thu", "Lập Đông"];

/// Number of solar terms joined into each seasonal profile.
pub const TERMS_PER_SEASON: usize = 6;

/// Bilingual composition disclosure (Vietnamese). Byte-identical to the
/// seasonal REVIEWER-PACK §A.5; `tests/prohibited_language_guard.rs`
/// enforces the pack lock.
pub const COMPOSITION_NOTE_VN: &str = "Amlich ghép tiết khí hiện hành vào một trong bốn mùa theo bốn tiết mở đầu mùa: Lập Xuân, Lập Hạ, Lập Thu, Lập Đông (mỗi mùa sáu tiết). Văn bản cổ chỉ trình bày bốn đề cương theo mùa; phép ghép này là của Amlich, không phải hai mươi bốn chế độ riêng theo tiết, cũng không phải nhận định về thời tiết địa phương.";

/// Bilingual composition disclosure (English). Byte-identical to the
/// seasonal REVIEWER-PACK §A.5.
pub const COMPOSITION_NOTE_EN: &str = "Amlich joins the current solar term into one of four seasons at the four seasonal-beginning terms: Lập Xuân, Lập Hạ, Lập Thu, and Lập Đông (six terms per season). The classical text presents only four seasonal profiles; this join is an Amlich composition — not twenty-four term-specific regimens and not a statement about local weather.";

// ---------------------------------------------------------------------------
// SeasonKey
// ---------------------------------------------------------------------------

/// One of the four seasonal profiles. Serde form is the lowercase
/// season name (`"spring"` … `"winter"`) — the corpus `season` field and
/// the `passage_key` share this spelling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SeasonKey {
    #[serde(rename = "spring")]
    Spring,
    #[serde(rename = "summer")]
    Summer,
    #[serde(rename = "autumn")]
    Autumn,
    #[serde(rename = "winter")]
    Winter,
}

impl SeasonKey {
    /// String label exposed to consumers; matches the serde rename.
    pub const fn as_str(self) -> &'static str {
        match self {
            SeasonKey::Spring => "spring",
            SeasonKey::Summer => "summer",
            SeasonKey::Autumn => "autumn",
            SeasonKey::Winter => "winter",
        }
    }

    /// All four seasons in corpus order.
    pub const fn all() -> [SeasonKey; 4] {
        [
            SeasonKey::Spring,
            SeasonKey::Summer,
            SeasonKey::Autumn,
            SeasonKey::Winter,
        ]
    }
}

// ---------------------------------------------------------------------------
// Frozen term-to-season composition
// ---------------------------------------------------------------------------

/// The frozen Amlich term-to-season composition (LH-DIV-04). All 24
/// solar terms map deterministically to exactly one of four profiles,
/// six terms per season, with transitions at the four Lập boundaries —
/// **not** the equinox/solstice quarters used by
/// `tietkhi::get_season`:
///
/// - Spring: Lập Xuân (21), Vũ Thủy (22), Kinh Trập (23), Xuân Phân
///   (0), Thanh Minh (1), Cốc Vũ (2)
/// - Summer: Lập Hạ (3) … Đại Thử (8)
/// - Autumn: Lập Thu (9) … Sương Giáng (14)
/// - Winter: Lập Đông (15) … Đại Hàn (20)
///
/// Term indexes follow the canonical `tietkhi::TIET_KHI` array (0 =
/// Xuân Phân at ecliptic longitude 0°). Returns `None` for indexes
/// outside `0..=23`.
///
/// This mapping is a design composition disclosed via
/// [`COMPOSITION_NOTE_VN`]/[`COMPOSITION_NOTE_EN`] — it must never be
/// presented as a claim of *Suwen*.
pub fn season_for_term_index(term_index: usize) -> Option<SeasonKey> {
    match term_index {
        0..=2 | 21..=23 => Some(SeasonKey::Spring),
        3..=8 => Some(SeasonKey::Summer),
        9..=14 => Some(SeasonKey::Autumn),
        15..=20 => Some(SeasonKey::Winter),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// SeasonalCultivationProfile
// ---------------------------------------------------------------------------

/// One row of the four-season cultivation corpus. The four rows
/// together live in
/// `crates/amlich-core/data/traditional-wellness/seasonal-cultivation.json`
/// and are `ExternalReviewPending` until the Suwen paraphrase reviewer
/// gate signs (REVIEWER-PACK §B, seasonal track).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeasonalCultivationProfile {
    /// Which of the four seasons this profile describes.
    pub season: SeasonKey,
    /// Vietnamese season label (e.g. `"Xuân"`).
    pub season_vi: String,
    /// English season label (e.g. `"Spring"`).
    pub season_en: String,
    /// Chinese season label (e.g. `"春"`).
    pub season_zh: String,
    /// Passage key within `四氣調神大論篇第二` (`spring`/`summer`/
    /// `autumn`/`winter`); equals `season.as_str()` on every valid row.
    pub passage_key: String,
    /// Vietnamese project paraphrase of the seasonal routine themes.
    pub wording_vi: String,
    /// English project paraphrase of the seasonal routine themes.
    pub wording_en: String,
    /// Citation entries; every row carries exactly one (the Suwen
    /// chapter, with the per-season passage key).
    pub sources: Vec<SourceCitation>,
    /// Review state; `ExternalReviewPending` until the Suwen gate signs.
    pub reviewer: ExternalReviewState,
    /// Safety classification. Canonical value
    /// `"historical_cultural_non_clinical"`.
    pub safety_class: String,
    /// Divergence IDs applicable to this profile
    /// (`["LH-DIV-04", "LH-DIV-05", "LH-DIV-07"]`).
    pub known_divergence_ids: Vec<String>,
}

impl SeasonalCultivationProfile {
    /// Stable semantic-graph node id for this profile (used by bead
    /// `.3`'s graph projection; mirrors the
    /// `BranchChannelAssociation::channel_semantic_id` convention).
    pub fn season_semantic_id(&self) -> String {
        format!(
            "seasonal_profile:huangdi-neijing-suwen:{}",
            self.season.as_str()
        )
    }

    /// Convert each [`SourceCitation`] into a [`ProvenanceEntry`] for
    /// the reasoning-evidence emission path. Uses the canonical
    /// [`SOURCE_HUANGDI_NEIJING_SUWEN`] constant (static source-id
    /// discipline; the loader has already asserted equality).
    pub fn provenance_entries(&self) -> Vec<ProvenanceEntry> {
        self.sources
            .iter()
            .map(|c| {
                ProvenanceEntry::almanac_rule(
                    SOURCE_HUANGDI_NEIJING_SUWEN,
                    format!("seasonal_profile_lookup:{}", self.season.as_str()),
                )
                .with_note(format!(
                    "{} — {} (passage: {})",
                    c.work_title, c.volume_or_chapter, c.passage_key
                ))
            })
            .collect()
    }

    /// Convert each [`SourceCitation`] into a
    /// [`ReasoningEvidenceEnvelope`] for the high-level reasoning
    /// surface (the Suwen **primitive** envelope; the composite lives on
    /// the context, not the profile).
    pub fn reasoning_evidence(&self) -> Vec<ReasoningEvidenceEnvelope> {
        self.sources
            .iter()
            .map(|c| ReasoningEvidenceEnvelope {
                source_family: ReasoningEvidenceSourceFamily::AlmanacRule,
                source_id: SOURCE_HUANGDI_NEIJING_SUWEN.to_string(),
                method: format!("seasonal_profile_lookup:{}", self.season.as_str()),
                note: Some(format!(
                    "{} — {} (passage: {}; {})",
                    c.work_title, c.volume_or_chapter, c.passage_key, c.translation_kind
                )),
            })
            .collect()
    }
}

// ---------------------------------------------------------------------------
// SeasonalCultivationContext
// ---------------------------------------------------------------------------

/// The selected-date Traditional Wellness Context: the active solar
/// term joined to one of four seasonal cultivation profiles.
///
/// Provenance discipline (SOURCE-01): `evidence` always carries exactly
/// three envelopes — the solar-term **primitive** (Snapshot family,
/// method `get_tiet_khi`, engine attribution), the Suwen **primitive**
/// (`huangdi-neijing-suwen`), and one **composite**
/// (`rule.composite.seasonal_wellness`, Derived family) representing
/// the Amlich join. The solar-term evidence is never retagged as Suwen
/// and the Suwen evidence never claims the term computation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeasonalCultivationContext {
    /// The active solar term from the existing astronomical engine,
    /// preserved verbatim (primitive evidence, unchanged provenance).
    pub solar_term: SolarTerm,
    /// The season the current term composes into.
    pub season: SeasonKey,
    /// The source-grounded seasonal cultivation profile.
    pub profile: SeasonalCultivationProfile,
    /// Stable bilingual cultural-information disclaimer.
    pub disclaimer: LocalizedDisclaimer,
    /// Per-profile review state (aggregate with the disclaimer +
    /// provenance audit for the full picture).
    pub review_state: ExternalReviewState,
    /// Bilingual composition disclosure (LH-DIV-04): the term-to-season
    /// join is an Amlich composition, not 24 term regimens, not local
    /// weather. Byte-identical to [`COMPOSITION_NOTE_VN`].
    pub composition_note_vi: String,
    /// English composition disclosure; byte-identical to
    /// [`COMPOSITION_NOTE_EN`].
    pub composition_note_en: String,
    /// Exactly three envelopes: solar-term primitive, Suwen primitive,
    /// and the composite join.
    pub evidence: Vec<ReasoningEvidenceEnvelope>,
}

// ---------------------------------------------------------------------------
// Corpus loader
// ---------------------------------------------------------------------------

const SEASONAL_CULTIVATION_JSON: &str =
    include_str!("../../data/traditional-wellness/seasonal-cultivation.json");

#[derive(Debug, Deserialize)]
struct SeasonalCultivationFile {
    metadata: SeasonalCultivationFileMetadata,
    profiles: Vec<SeasonalCultivationProfileRow>,
}

#[derive(Debug, Deserialize)]
struct SeasonalCultivationFileMetadata {
    primary_source: SeasonalCultivationPrimarySource,
    composition: SeasonalCultivationCompositionMetadata,
    divergence_ids_in_use: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct SeasonalCultivationPrimarySource {
    source_id: String,
}

#[derive(Debug, Deserialize)]
struct SeasonalCultivationCompositionMetadata {
    boundaries: Vec<String>,
    terms_per_season: usize,
    composite_rule_id: String,
}

#[derive(Debug, Deserialize)]
struct SeasonalCultivationProfileRow {
    season: SeasonKey,
    season_vi: String,
    season_en: String,
    season_zh: String,
    passage_key: String,
    wording_vi: String,
    wording_en: String,
    sources: Vec<SourceCitation>,
    reviewer: ExternalReviewState,
    safety_class: String,
    known_divergence_ids: Vec<String>,
}

impl From<SeasonalCultivationProfileRow> for SeasonalCultivationProfile {
    fn from(row: SeasonalCultivationProfileRow) -> Self {
        SeasonalCultivationProfile {
            season: row.season,
            season_vi: row.season_vi,
            season_en: row.season_en,
            season_zh: row.season_zh,
            passage_key: row.passage_key,
            wording_vi: row.wording_vi,
            wording_en: row.wording_en,
            sources: row.sources,
            reviewer: row.reviewer,
            safety_class: row.safety_class,
            known_divergence_ids: row.known_divergence_ids,
        }
    }
}

static SEASONAL_CULTIVATION_CORPUS: OnceLock<Vec<SeasonalCultivationProfile>> = OnceLock::new();

/// Load and validate the 4-profile seasonal cultivation corpus. Returns
/// a `&'static` slice; panics on any invariant violation (test oracle /
/// authoritative corpus, not user-facing data).
///
/// Validation rules:
/// - exactly 4 profiles — one per [`SeasonKey`], no duplicates (a fifth
///   row or a per-term paraphrase would break the "four profiles, not
///   24 regimens" contract, LH-DIV-04);
/// - every profile's `passage_key` equals its `season.as_str()`;
/// - every profile's `sources` is non-empty and every `source_id`
///   equals [`SOURCE_HUANGDI_NEIJING_SUWEN`];
/// - every profile's `known_divergence_ids` resolves via
///   [`divergence_by_id`];
/// - metadata `primary_source.source_id` equals
///   [`SOURCE_HUANGDI_NEIJING_SUWEN`], `composition.composite_rule_id`
///   equals [`COMPOSITE_SEASONAL_WELLNESS`], `terms_per_season` equals
///   [`TERMS_PER_SEASON`], and `composition.boundaries` equals
///   [`SEASONAL_BOUNDARY_TERM_NAMES`] element-for-element.
pub fn load_seasonal_corpus() -> &'static [SeasonalCultivationProfile] {
    SEASONAL_CULTIVATION_CORPUS.get_or_init(|| {
        let file: SeasonalCultivationFile = serde_json::from_str(SEASONAL_CULTIVATION_JSON)
            .expect("seasonal-cultivation.json must parse cleanly");
        assert_eq!(
            file.metadata.primary_source.source_id,
            SOURCE_HUANGDI_NEIJING_SUWEN,
            "corpus metadata.primary_source.source_id must equal SOURCE_HUANGDI_NEIJING_SUWEN"
        );
        assert_eq!(
            file.metadata.composition.composite_rule_id,
            COMPOSITE_SEASONAL_WELLNESS,
            "corpus composition.composite_rule_id must equal COMPOSITE_SEASONAL_WELLNESS"
        );
        assert_eq!(
            file.metadata.composition.terms_per_season, TERMS_PER_SEASON,
            "corpus composition.terms_per_season must equal TERMS_PER_SEASON"
        );
        assert_eq!(
            file.metadata.composition.boundaries,
            SEASONAL_BOUNDARY_TERM_NAMES,
            "corpus composition.boundaries must equal the frozen Lập boundary names"
        );
        let mut registered_seasonal_ids: Vec<String> =
            super::divergence::all_divergences_for_seasonal_cultivation()
                .into_iter()
                .map(|d| d.id)
                .collect();
        registered_seasonal_ids.sort_unstable();
        let mut declared_ids = file.metadata.divergence_ids_in_use.clone();
        declared_ids.sort_unstable();
        assert_eq!(
            declared_ids, registered_seasonal_ids,
            "corpus metadata.divergence_ids_in_use must match the in-code seasonal divergence registry"
        );
        assert_eq!(
            file.profiles.len(),
            4,
            "seasonal cultivation corpus must contain exactly 4 profiles"
        );
        let mut seen_seasons = Vec::with_capacity(4);
        for row in &file.profiles {
            assert!(
                !seen_seasons.contains(&row.season),
                "duplicate season profile: {:?}",
                row.season
            );
            seen_seasons.push(row.season);
            assert_eq!(
                row.passage_key,
                row.season.as_str(),
                "profile {:?} passage_key must equal its season key",
                row.season
            );
            assert!(
                !row.sources.is_empty(),
                "profile {:?} carries no sources",
                row.season
            );
            for src in &row.sources {
                assert_eq!(
                    src.source_id, SOURCE_HUANGDI_NEIJING_SUWEN,
                    "every seasonal source_id must equal SOURCE_HUANGDI_NEIJING_SUWEN; got {}",
                    src.source_id
                );
            }
            for id in &row.known_divergence_ids {
                assert!(
                    divergence_by_id(id).is_some(),
                    "profile {:?} references unregistered divergence id {id}",
                    row.season
                );
            }
        }
        for season in SeasonKey::all() {
            assert!(
                seen_seasons.contains(&season),
                "corpus missing profile for season {:?}",
                season
            );
        }
        file.profiles.into_iter().map(Into::into).collect()
    })
}

// ---------------------------------------------------------------------------
// Resolution + evidence
// ---------------------------------------------------------------------------

/// Build the three-envelope evidence triple (SOURCE-01 provenance
/// separation). Order is stable: solar-term primitive, Suwen primitive,
/// composite join.
fn build_evidence(
    solar_term: &SolarTerm,
    profile: &SeasonalCultivationProfile,
) -> Vec<ReasoningEvidenceEnvelope> {
    let mut evidence = Vec::with_capacity(3);

    // 1. Solar-term primitive — the existing astronomical engine keeps
    //    its own provenance and is never retagged as Suwen (LH-DIV-04).
    evidence.push(ReasoningEvidenceEnvelope {
        source_family: ReasoningEvidenceSourceFamily::Snapshot,
        source_id: SOLAR_TERM_ENGINE_SOURCE_ID.to_string(),
        method: "get_tiet_khi".to_string(),
        note: Some(format!(
            "solar term {} (index {}; sun longitude {}°) computed by the existing amlich astronomical engine",
            solar_term.name, solar_term.index, solar_term.current_longitude
        )),
    });

    // 2. Suwen primitive — the seasonal paraphrase, its own source.
    evidence.extend(profile.reasoning_evidence());

    // 3. Derived composite — the Amlich term-to-season join, exactly one.
    evidence.push(ReasoningEvidenceEnvelope {
        source_family: ReasoningEvidenceSourceFamily::Derived,
        source_id: COMPOSITE_SEASONAL_WELLNESS.to_string(),
        method: "v110.term_to_season_join".to_string(),
        note: Some(format!(
            "joins solar term {} into the {} seasonal profile at the Lập seasonal boundaries; Amlich composition — not a Suwen claim about solar terms",
            solar_term.name,
            profile.season.as_str()
        )),
    });

    evidence
}

/// Build a [`SeasonalCultivationContext`] for the given local date
/// (Julian day + timezone offset). Total from valid inputs: the
/// existing `get_tiet_khi` engine resolves the active term (indexes
/// always in `0..=23`), and the frozen composition joins it to exactly
/// one of the four corpus profiles.
///
/// Tier 0 (BOUND-01): the input is `(jd, time_zone)` alone; no
/// `BirthInput`, sex/gender, symptom, location, or health history is
/// consulted.
pub fn resolve_seasonal_cultivation(jd: i32, time_zone: f64) -> SeasonalCultivationContext {
    let solar_term = get_tiet_khi(jd, time_zone);
    let season = season_for_term_index(solar_term.index)
        .expect("get_tiet_khi returns a canonical term index in 0..=23");
    let profile = load_seasonal_corpus()
        .iter()
        .find(|p| p.season == season)
        .expect("corpus validated to contain all four seasons")
        .clone();
    let evidence = build_evidence(&solar_term, &profile);
    SeasonalCultivationContext {
        solar_term,
        season,
        review_state: profile.reviewer.clone(),
        disclaimer: cultural_information_disclaimer(),
        composition_note_vi: COMPOSITION_NOTE_VN.to_string(),
        composition_note_en: COMPOSITION_NOTE_EN.to_string(),
        profile,
        evidence,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::julian::jd_from_date;

    #[test]
    fn season_for_term_index_maps_all_24_terms_six_per_season() {
        let mut counts = std::collections::HashMap::new();
        for index in 0..24 {
            let season = season_for_term_index(index).expect("every term index 0..=23 must map");
            *counts.entry(season).or_insert(0usize) += 1;
        }
        for season in SeasonKey::all() {
            assert_eq!(
                counts.get(&season).copied().unwrap_or(0),
                TERMS_PER_SEASON,
                "season {season:?} must cover exactly {TERMS_PER_SEASON} terms"
            );
        }
    }

    #[test]
    fn season_for_term_index_rejects_out_of_range() {
        assert!(season_for_term_index(24).is_none());
        assert!(season_for_term_index(999).is_none());
    }

    #[test]
    fn transitions_anchor_at_the_four_lap_boundaries() {
        // The 8 transition edges: term before each Lập keeps the old
        // season; the Lập term itself starts the new season.
        assert_eq!(season_for_term_index(20), Some(SeasonKey::Winter)); // Đại Hàn
        assert_eq!(season_for_term_index(21), Some(SeasonKey::Spring)); // Lập Xuân
        assert_eq!(season_for_term_index(2), Some(SeasonKey::Spring)); // Cốc Vũ
        assert_eq!(season_for_term_index(3), Some(SeasonKey::Summer)); // Lập Hạ
        assert_eq!(season_for_term_index(8), Some(SeasonKey::Summer)); // Đại Thử
        assert_eq!(season_for_term_index(9), Some(SeasonKey::Autumn)); // Lập Thu
        assert_eq!(season_for_term_index(14), Some(SeasonKey::Autumn)); // Sương Giáng
        assert_eq!(season_for_term_index(15), Some(SeasonKey::Winter)); // Lập Đông
    }

    #[test]
    fn seasonal_mapping_is_not_the_equinox_quarter_mapping() {
        // LH-DIV-04 lock: the composition must NOT reuse the
        // equinox/solstice quarters of tietkhi::get_season. Under
        // get_season's quarter arithmetic, Lập Xuân (21), Vũ Thủy (22),
        // and Kinh Trập (23) are still "Đông (Winter)" (winter runs
        // 18..=23); the frozen composition starts spring at Lập Xuân.
        assert!(crate::tietkhi::get_season(21).starts_with("Đông"));
        assert_eq!(season_for_term_index(21), Some(SeasonKey::Spring));
        // Likewise Lập Hạ (3) is "Xuân" under the quarters but summer in
        // the composition.
        assert!(crate::tietkhi::get_season(3).starts_with("Xuân"));
        assert_eq!(season_for_term_index(3), Some(SeasonKey::Summer));
    }

    #[test]
    fn load_seasonal_corpus_returns_exactly_four_distinct_seasons() {
        let corpus = load_seasonal_corpus();
        assert_eq!(corpus.len(), 4);
        for season in SeasonKey::all() {
            assert!(
                corpus.iter().any(|p| p.season == season),
                "corpus must contain the {season:?} profile"
            );
        }
    }

    #[test]
    fn every_profile_uses_only_huangdi_neijing_suwen() {
        for profile in load_seasonal_corpus() {
            assert!(!profile.sources.is_empty());
            for src in &profile.sources {
                assert_eq!(src.source_id, SOURCE_HUANGDI_NEIJING_SUWEN);
            }
            assert_eq!(profile.passage_key, profile.season.as_str());
        }
    }

    #[test]
    fn every_profile_references_the_seasonal_divergence_triple() {
        for profile in load_seasonal_corpus() {
            for id in ["LH-DIV-04", "LH-DIV-05", "LH-DIV-07"] {
                assert!(
                    profile.known_divergence_ids.iter().any(|x| x == id),
                    "profile {:?} must reference {id}",
                    profile.season
                );
            }
        }
    }

    #[test]
    fn every_profile_wording_uses_classical_text_framing() {
        // Wording discipline: paraphrases are framed as classical-text
        // description ("văn bản cổ mô tả" / "the classical text
        // describes"), never as instruction or advice.
        for profile in load_seasonal_corpus() {
            assert!(
                profile.wording_vi.contains("văn bản cổ mô tả"),
                "wording must use classical-text framing (vi): {}",
                profile.wording_vi
            );
            assert!(
                profile.wording_en.contains("the classical text describes"),
                "wording must use classical-text framing (en): {}",
                profile.wording_en
            );
        }
    }

    #[test]
    fn resolve_carries_three_envelopes_with_separated_provenance() {
        let jd = jd_from_date(16, 8, 2026);
        let ctx = resolve_seasonal_cultivation(jd, 7.0);
        assert_eq!(ctx.evidence.len(), 3);

        // 1. Solar-term primitive: Snapshot family, engine attribution,
        //    and never tagged as Suwen.
        assert_eq!(
            ctx.evidence[0].source_family,
            ReasoningEvidenceSourceFamily::Snapshot
        );
        assert_eq!(ctx.evidence[0].source_id, SOLAR_TERM_ENGINE_SOURCE_ID);
        assert_eq!(ctx.evidence[0].method, "get_tiet_khi");
        assert_ne!(ctx.evidence[0].source_id, SOURCE_HUANGDI_NEIJING_SUWEN);

        // 2. Suwen primitive: the paraphrase source, never the engine.
        assert_eq!(
            ctx.evidence[1].source_family,
            ReasoningEvidenceSourceFamily::AlmanacRule
        );
        assert_eq!(ctx.evidence[1].source_id, SOURCE_HUANGDI_NEIJING_SUWEN);
        assert!(ctx.evidence[1]
            .method
            .starts_with("seasonal_profile_lookup:"));

        // 3. Exactly one composite join envelope.
        assert_eq!(
            ctx.evidence[2].source_family,
            ReasoningEvidenceSourceFamily::Derived
        );
        assert_eq!(ctx.evidence[2].source_id, COMPOSITE_SEASONAL_WELLNESS);
        let composite_count = ctx
            .evidence
            .iter()
            .filter(|e| e.source_id == COMPOSITE_SEASONAL_WELLNESS)
            .count();
        assert_eq!(composite_count, 1);
    }

    #[test]
    fn resolve_carries_disclaimer_pending_review_and_composition_note() {
        let ctx = resolve_seasonal_cultivation(jd_from_date(16, 8, 2026), 7.0);
        assert_eq!(ctx.disclaimer.id.as_str(), "cultural_information_v1");
        assert!(!ctx.disclaimer.vi.is_empty());
        assert!(!ctx.disclaimer.en.is_empty());
        assert!(matches!(
            ctx.review_state,
            ExternalReviewState::ExternalReviewPending { .. }
        ));
        assert_eq!(ctx.composition_note_vi, COMPOSITION_NOTE_VN);
        assert_eq!(ctx.composition_note_en, COMPOSITION_NOTE_EN);
        assert_eq!(ctx.season, ctx.profile.season);
    }

    #[test]
    fn context_round_trips_byte_equal() {
        let ctx = resolve_seasonal_cultivation(jd_from_date(16, 8, 2026), 7.0);
        let json = serde_json::to_string(&ctx).expect("serialize");
        let recovered: SeasonalCultivationContext =
            serde_json::from_str(&json).expect("deserialize");
        let json2 = serde_json::to_string(&recovered).expect("re-serialize");
        assert_eq!(json, json2);
        assert_eq!(recovered, ctx);
    }

    #[test]
    fn composition_notes_are_byte_identical_to_reviewer_pack() {
        let pack = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("..")
                .join(".planning")
                .join("milestones")
                .join("v1.10-phases")
                .join("02-seasonal-cultivation-context")
                .join("REVIEWER-PACK.md"),
        )
        .expect("seasonal REVIEWER-PACK must exist");
        assert!(
            pack.contains(COMPOSITION_NOTE_VN),
            "Vietnamese composition note must appear verbatim in REVIEWER-PACK §A.5"
        );
        assert!(
            pack.contains(COMPOSITION_NOTE_EN),
            "English composition note must appear verbatim in REVIEWER-PACK §A.5"
        );
    }
}
