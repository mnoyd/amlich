//! Locked v1 ritual entry schema. See ADR-0001.
//!
//! Every change to a public type in this file requires a superseding ADR
//! per the schema-lock discipline (PITFALLS CRIT-1, CRIT-5).

use serde::{Deserialize, Serialize};

/// Confidence tier for a ritual entry's provenance (per ADR-0001).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RitualConfidenceTier {
    Primary,
    RegionalVariant,
    Synthesized,
}

/// Leap-month inclusion policy for lunar date matching (RIT-07).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum LeapPolicy {
    #[default]
    CanonicalMonthOnly,
    LeapMonthOnly,
    Either,
}

/// Discriminated union for matching a day against a lunar date pattern (RIT-07).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum LunarDateMatch {
    MonthDay {
        month: u8,
        day: u8,
        #[serde(default)]
        leap_month_policy: LeapPolicy,
    },
    SolarTerm {
        name: String,
    },
    GregorianFixed {
        month: u8,
        day: u8,
    },
}

/// Life-event kinds (RIT-03).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifeEventKind {
    DongTho,
    NhapTrach,
    KhaiTruong,
    Cuoi,
    Gio,
    DayThang,
}

/// Event key — discriminated union for `event_keys[]` (RIT-06).
///
/// Note: `LunarDate` embeds month/day/leap_month_policy directly rather than wrapping
/// `LunarDateMatch` as a newtype. This avoids the serde internally-tagged enum conflict that
/// would arise from nesting two `#[serde(tag = "kind")]` enums (the outer `kind: "lunar_date"`
/// would be consumed before the inner `LunarDateMatch` could read its own `kind` field).
/// `LunarDateMatch` is kept as a standalone type for direct use cases (e.g., RIT-07 query API).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum RitualEventKey {
    HolidayId { value: String },
    LunarDate {
        month: u8,
        day: u8,
        #[serde(default)]
        leap_month_policy: LeapPolicy,
    },
    SolarTerm { name: String },
    LifeEvent { event: LifeEventKind },
    Always,
}

/// Variant discriminator for rituals sharing an event (RIT-12, CONTEXT.md locked).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RitualVariantTag {
    Simple,
    Full,
    Buddhist,
    Folk,
    Regional(String),
}

/// Citation pointing at the classical reference for an entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceCitation {
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
}

/// Structured offering (lễ vật).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Offering {
    pub name_vi: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_en: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Annotation indicating that a ritual offering reference ALSO originates in a
/// non-ritual classical tradition (Phase 19, INT-09). Example: a Huyền Không
/// Ngũ Hành element-cure surfaced inside a văn khấn ritual.
///
/// When `RitualEntry::metadata.cross_source_curing` contains a `CrossSourceCure`,
/// the `add_offering_facts` builder emits TWO `track_provenance` calls on the
/// `RecommendsOffering` edge — one for the ritual tradition (`vn-folk-ritual`)
/// and one for the annotated tradition (e.g., `huyen-khong`). This implements
/// INT-09's dual-source edge provenance on the existing `ProvenanceTracker::track()`
/// append-pattern (v1.5 multi-source dedup is implicit — NO parallel dedup helper).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CrossSourceCure {
    /// The Ngũ Hành element this cure addresses (e.g. "Kim", "Mộc", "Thủy", "Hỏa", "Thổ").
    /// Free-form Vietnamese string for now (the existing `Element` enum at
    /// `almanac/fengshui/types.rs` uses lowercase English: "metal", "wood", etc.).
    /// Phase 19 keeps the cross-source cure annotation human-readable; future
    /// phases MAY tighten into a typed enum.
    pub element_cure_for: String,
    /// MUST equal one of `crate::sources::SOURCE_*` (typically `SOURCE_HUYEN_KHONG`).
    /// Typed as `crate::sources::SourceId` per INT-07 discipline.
    pub source_id: crate::sources::SourceId,
    /// Vietnamese-language rationale explaining why this tradition curates
    /// this element (free-form, audited-by-author at corpus-load time).
    pub rationale_vi: String,
}

/// Optional extension metadata on a `RitualEntry` (Phase 19, INT-09 corpus
/// augmentation mechanism). Currently carries `cross_source_curing` annotations
/// — a list of non-ritual-tradition element cures surfaced inside the ritual.
/// Future fields MAY be added as the additive `Option<T>` discipline permits
/// (`metadata: Option<RitualMetadata>` stays compatible with new fields inside
/// the inner struct).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RitualMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cross_source_curing: Option<Vec<CrossSourceCure>>,
}

/// Identity handle for a semantic-graph Offering node.
///
/// Locked before any builder code emits Offering nodes (schema-lock
/// discipline per Phase 10 / Phase 18-01). Mirrors `RitualEntry::ritual_id`
/// as the stable join key for the semantic graph. The `offering_id`
/// is derived from the corpus position (e.g. `ritual.van-khan-tet-don-gian.offering.0`),
/// NOT hashed from `name_vi` — see Pitfall P-3 / Don't-Hand-Roll in 19-RESEARCH.md.
///
/// `source_id` is typed as `crate::sources::SourceId` (a `String` alias) per
/// INT-07's literal SC text "source_id: SourceId" (REQUIREMENTS.md:31). The
/// underlying value MUST equal one of `crate::sources::SOURCE_*` — enforced
/// by the constructor (`debug_assert!(!source_id.is_empty())`) + the
/// `tests/source_id_guard.rs` grep guard on bare-string literals.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OfferingRef {
    /// Stable id of the form "ritual.{ritual_id}.offering.{idx}".
    pub offering_id: String,
    pub name_vi: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_en: Option<String>,
    /// MUST equal one of `crate::sources::SOURCE_*`. Enforced by the
    /// constructor (debug_assert) + `tests/source_id_guard.rs`.
    pub source_id: crate::sources::SourceId,
}

impl OfferingRef {
    /// Ergonomic constructor accepting `String` (so call-sites can pass
    /// `SOURCE_*.to_string()` directly without conversion). Internally
    /// stored as `crate::sources::SourceId` (a `String` alias).
    pub fn new(
        offering_id: String,
        name_vi: String,
        name_en: Option<String>,
        source_id: String,
    ) -> Self {
        debug_assert!(!offering_id.is_empty(), "OfferingRef::offering_id must be non-empty");
        debug_assert!(!name_vi.is_empty(), "OfferingRef::name_vi must be non-empty");
        debug_assert!(!source_id.is_empty(), "OfferingRef::source_id must be non-empty");
        Self {
            offering_id,
            name_vi,
            name_en,
            source_id: crate::sources::SourceId::from(source_id),
        }
    }
}

/// Structured preparation step (trình tự).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PreparationStep {
    pub order: u8,
    pub description_vi: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description_en: Option<String>,
}

/// Locked v1 ritual entry. See ADR-0001.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RitualEntry {
    pub ritual_id: String,
    pub title_vi: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title_en: Option<String>,
    pub event_keys: Vec<RitualEventKey>,
    pub variant: RitualVariantTag,
    pub offerings: Vec<Offering>,
    pub preparation_steps: Vec<PreparationStep>,
    pub invocation_text_vi: String,
    /// Reserved per RIT-13. Always null in v1.5 corpus.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_en: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
    /// Must always equal `crate::sources::SOURCE_VN_FOLK_RITUAL` for ritual entries.
    /// Phase 11 corpus loader enforces this; Phase 10 type stub only declares the field.
    pub source_id: String,
    pub original_citation: SourceCitation,
    pub confidence: RitualConfidenceTier,
    /// Optional Phase 19 extension metadata (INT-09 corpus augmentation).
    /// Carries `cross_source_curing` annotations for non-ritual-tradition
    /// element cures surfaced inside this ritual. Additive `Option<T>` with
    /// `#[serde(default, skip_serializing_if = "Option::is_none")]` matches the
    /// established additive field discipline (see `body_en`, `notes`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<RitualMetadata>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test 1: A sample valid RitualEntry JSON (Tết simple-variant) deserializes
    // successfully and entry.source_id == "vn-folk-ritual".
    #[test]
    fn sample_ritual_entry_json_deserializes() {
        let json = r#"{
            "ritual_id": "van-khan-tet-don-gian",
            "title_vi": "Văn Khấn Tết Nguyên Đán (Đơn Giản)",
            "event_keys": [
                {"kind": "holiday_id", "value": "tet-nguyen-dan"},
                {"kind": "lunar_date", "month": 1, "day": 1}
            ],
            "variant": "simple",
            "offerings": [
                {"name_vi": "Hương", "quantity": "3 nén"},
                {"name_vi": "Hoa tươi", "quantity": "1 bình"}
            ],
            "preparation_steps": [
                {"order": 1, "description_vi": "Tắm rửa sạch sẽ, mặc quần áo chỉnh tề"},
                {"order": 2, "description_vi": "Bày lễ vật lên bàn thờ gia tiên"}
            ],
            "invocation_text_vi": "Nam mô a di đà phật! Con lạy chín phương trời...",
            "source_id": "vn-folk-ritual",
            "original_citation": {
                "title": "Văn Khấn Cổ Truyền Việt Nam",
                "publisher": "NXB Văn Hóa Thông Tin",
                "page": "12"
            },
            "confidence": "primary"
        }"#;

        let entry: RitualEntry = serde_json::from_str(json).expect("should deserialize");
        assert_eq!(entry.source_id, "vn-folk-ritual");
        assert_eq!(entry.ritual_id, "van-khan-tet-don-gian");
        assert_eq!(entry.variant, RitualVariantTag::Simple);
        assert_eq!(entry.offerings.len(), 2);
        assert_eq!(entry.preparation_steps.len(), 2);
        assert_eq!(entry.confidence, RitualConfidenceTier::Primary);
        // Verify event_keys decoded correctly
        assert_eq!(entry.event_keys.len(), 2);
        assert_eq!(entry.event_keys[1], RitualEventKey::LunarDate { month: 1, day: 1, leap_month_policy: LeapPolicy::CanonicalMonthOnly });
    }

    // Test 2: A JSON with an unknown field fails to deserialize.
    #[test]
    fn unknown_field_fails_deserialization() {
        let json = r#"{
            "ritual_id": "van-khan-tet-don-gian",
            "title_vi": "Văn Khấn Tết Nguyên Đán",
            "event_keys": [{"kind": "holiday_id", "value": "tet-nguyen-dan"}],
            "variant": "simple",
            "offerings": [],
            "preparation_steps": [],
            "invocation_text_vi": "Nam mô...",
            "source_id": "vn-folk-ritual",
            "original_citation": {"title": "Test"},
            "confidence": "primary",
            "unexpected_field": "x"
        }"#;

        let result: Result<RitualEntry, _> = serde_json::from_str(json);
        assert!(result.is_err(), "unknown field should fail deserialization");
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("unexpected_field") || err_msg.contains("unknown field"),
            "error should mention unknown field, got: {err_msg}"
        );
    }

    // Test 3: RitualVariantTag round-trips all five variants via serde JSON.
    #[test]
    fn variant_tag_roundtrip_all_five() {
        // Simple
        let simple: RitualVariantTag = serde_json::from_str(r#""simple""#).unwrap();
        assert_eq!(simple, RitualVariantTag::Simple);
        assert_eq!(serde_json::to_string(&simple).unwrap(), r#""simple""#);

        // Full
        let full: RitualVariantTag = serde_json::from_str(r#""full""#).unwrap();
        assert_eq!(full, RitualVariantTag::Full);
        assert_eq!(serde_json::to_string(&full).unwrap(), r#""full""#);

        // Buddhist
        let buddhist: RitualVariantTag = serde_json::from_str(r#""buddhist""#).unwrap();
        assert_eq!(buddhist, RitualVariantTag::Buddhist);
        assert_eq!(serde_json::to_string(&buddhist).unwrap(), r#""buddhist""#);

        // Folk
        let folk: RitualVariantTag = serde_json::from_str(r#""folk""#).unwrap();
        assert_eq!(folk, RitualVariantTag::Folk);
        assert_eq!(serde_json::to_string(&folk).unwrap(), r#""folk""#);

        // Regional
        let regional: RitualVariantTag =
            serde_json::from_str(r#"{"regional":"mien-bac"}"#).unwrap();
        assert_eq!(regional, RitualVariantTag::Regional("mien-bac".to_string()));
        assert_eq!(
            serde_json::to_string(&regional).unwrap(),
            r#"{"regional":"mien-bac"}"#
        );
    }

    // Test 4: An unknown variant tag fails to deserialize.
    #[test]
    fn unknown_variant_tag_fails() {
        let result: Result<RitualVariantTag, _> = serde_json::from_str(r#""unknown""#);
        assert!(result.is_err(), "unknown variant tag should fail deserialization");
    }

    // Test 5: LunarDateMatch::MonthDay defaults leap_month_policy to CanonicalMonthOnly when absent.
    #[test]
    fn lunar_date_month_day_defaults_leap_policy_to_canonical_month_only() {
        let json = r#"{"kind":"month_day","month":1,"day":1}"#;
        let result: LunarDateMatch = serde_json::from_str(json).expect("should deserialize");
        assert_eq!(
            result,
            LunarDateMatch::MonthDay {
                month: 1,
                day: 1,
                leap_month_policy: LeapPolicy::CanonicalMonthOnly,
            }
        );
    }

    // Test 6: OfferingRef serde round-trip — Phase 19-01 schema lock (INT-08)
    #[test]
    fn offering_ref_serde_round_trip_and_deny_unknown_fields() {
        use crate::sources::SOURCE_VN_FOLK_RITUAL;

        // Round-trip with all four fields populated
        let r = OfferingRef::new(
            "ritual.van-khan-tet-don-gian.offering.0".to_string(),
            "Hương".to_string(),
            Some("Incense".to_string()),
            SOURCE_VN_FOLK_RITUAL.to_string(),
        );
        let json = serde_json::to_string(&r).expect("serialize");
        let recovered: OfferingRef = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(recovered, r);
        assert_eq!(recovered.source_id, "vn-folk-ritual");

        // Round-trip with name_en absent (skipped via skip_serializing_if)
        let r2 = OfferingRef::new(
            "ritual.x.offering.1".to_string(),
            "Hoa tươi".to_string(),
            None,
            SOURCE_VN_FOLK_RITUAL.to_string(),
        );
        let json2 = serde_json::to_string(&r2).expect("serialize");
        assert!(!json2.contains("name_en"), "name_en must be absent in JSON when None");
        let recovered2: OfferingRef = serde_json::from_str(&json2).expect("deserialize");
        assert_eq!(recovered2, r2);

        // Unknown field rejected by deny_unknown_fields
        let bad_json = r#"{"offering_id":"x","name_vi":"y","source_id":"vn-folk-ritual","bogus":1}"#;
        let err: Result<OfferingRef, _> = serde_json::from_str(bad_json);
        assert!(err.is_err(), "deny_unknown_fields must reject unknown fields");

        // INT-07 typed-source_id discipline: source_id is `crate::sources::SourceId`
        // (a String alias). Confirm compile-time type identity.
        let _: &crate::sources::SourceId = &r.source_id;
        assert_eq!(r.source_id.as_str(), "vn-folk-ritual");
    }

    // Test 7: RitualMetadata + CrossSourceCure serde round-trip — Phase 19-02 INT-09 schema lock
    #[test]
    fn ritual_metadata_and_cross_source_cure_serde_round_trip() {
        use crate::sources::SOURCE_HUYEN_KHONG;

        // Round-trip RitualMetadata with cross_source_curing populated
        let metadata = RitualMetadata {
            cross_source_curing: Some(vec![CrossSourceCure {
                element_cure_for: "Kim".to_string(),
                source_id: SOURCE_HUYEN_KHONG.to_string(),
                rationale_vi: "Huyền Không Ngũ Hành tương sinh: Kim sinh Thủy".to_string(),
            }]),
        };
        let json = serde_json::to_string(&metadata).expect("serialize");
        let recovered: RitualMetadata = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(recovered, metadata);

        // Round-trip with cross_source_curing absent (skip_serializing_if honored)
        let empty = RitualMetadata { cross_source_curing: None };
        let json_empty = serde_json::to_string(&empty).expect("serialize");
        assert!(!json_empty.contains("cross_source_curing"),
                "cross_source_curing must be absent in JSON when None; got: {json_empty}");

        // deny_unknown_fields on RitualMetadata rejects unknown fields
        let bad = r#"{"cross_source_curing": [], "bogus": 1}"#;
        let err: Result<RitualMetadata, _> = serde_json::from_str(bad);
        assert!(err.is_err(), "deny_unknown_fields must reject unknown fields on RitualMetadata");

        // deny_unknown_fields on CrossSourceCure rejects unknown fields
        let bad_cure = r#"{"element_cure_for":"Kim","source_id":"huyen-khong","rationale_vi":"x","bogus":1}"#;
        let err_cure: Result<CrossSourceCure, _> = serde_json::from_str(bad_cure);
        assert!(err_cure.is_err(), "deny_unknown_fields must reject unknown fields on CrossSourceCure");
    }
}
