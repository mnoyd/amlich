//! OnceLock-backed corpus loader for Văn khấn ritual entries.
//!
//! The corpus is embedded at compile time via `include_str!` and parsed once on
//! first access. Every text field is NFC-normalized at load (RIT-08); every
//! entry's `source_id` is validated against `crate::sources::SOURCE_VN_FOLK_RITUAL`.
//!
//! Schema is frozen by ADR-0001. Any change to the loaded shape requires a
//! superseding ADR and a bump from `"rituals-v1"` to `"rituals-vN"` in
//! `$schema_version` on the corpus files.
//!
//! Multi-file layout: one `include_str!` constant per category file (plan 12-03).
//! All files are merged into a single `Vec<RitualEntry>` in `ALL_CORPUS_JSONS`
//! order by the `all_rituals()` initializer. The manifest.json artifact documents
//! the file list for tooling but is NOT parsed at runtime (see plan 12-03 §Pattern 1).

use serde::Deserialize;
use std::sync::OnceLock;
use unicode_normalization::{is_nfc, UnicodeNormalization};

use crate::rituals::schema::RitualEntry;
use crate::sources::SOURCE_VN_FOLK_RITUAL;

// ---------------------------------------------------------------------------
// Corpus file constants (one include_str! per category file)
// Plan 12-01 batch — spring/summer festivals (26 entries)
// ---------------------------------------------------------------------------
const TET_NGUYEN_DAN_JSON: &str = include_str!("../../data/rituals/tet-nguyen-dan.json");
const NGUYEN_TIEU_JSON: &str = include_str!("../../data/rituals/nguyen-tieu.json");
const HAN_THUC_JSON: &str = include_str!("../../data/rituals/han-thuc.json");
const THANH_MINH_JSON: &str = include_str!("../../data/rituals/thanh-minh.json");
const DOAN_NGO_JSON: &str = include_str!("../../data/rituals/doan-ngo.json");
const PHAT_DAN_JSON: &str = include_str!("../../data/rituals/phat-dan.json");

// ---------------------------------------------------------------------------
// Plan 12-02 batch — autumn/winter festivals + life events + daily (34 entries)
// ---------------------------------------------------------------------------
const VU_LAN_JSON: &str = include_str!("../../data/rituals/vu-lan.json");
const TRUNG_THU_JSON: &str = include_str!("../../data/rituals/trung-thu.json");
const TRUNG_CUU_HA_NGUYEN_JSON: &str = include_str!("../../data/rituals/trung-cuu-ha-nguyen.json");
const ONG_TAO_JSON: &str = include_str!("../../data/rituals/ong-tao.json");
const LIFE_EVENTS_JSON: &str = include_str!("../../data/rituals/life-events.json");
const SOC_VONG_JSON: &str = include_str!("../../data/rituals/soc-vong.json");
const GIA_TIEN_THUONG_NHAT_JSON: &str =
    include_str!("../../data/rituals/gia-tien-thuong-nhat.json");

/// All corpus files in merge order. Every entry in every file is pushed into
/// the single `Vec<RitualEntry>` returned by `all_rituals()`.
const ALL_CORPUS_JSONS: &[&str] = &[
    TET_NGUYEN_DAN_JSON,
    NGUYEN_TIEU_JSON,
    HAN_THUC_JSON,
    THANH_MINH_JSON,
    DOAN_NGO_JSON,
    PHAT_DAN_JSON,
    VU_LAN_JSON,
    TRUNG_THU_JSON,
    TRUNG_CUU_HA_NGUYEN_JSON,
    ONG_TAO_JSON,
    LIFE_EVENTS_JSON,
    SOC_VONG_JSON,
    GIA_TIEN_THUONG_NHAT_JSON,
];

const EXPECTED_SCHEMA_VERSION: &str = "rituals-v1";

#[derive(Debug, Deserialize)]
struct RitualFile {
    #[serde(rename = "$schema_version")]
    schema_version: String,
    entries: Vec<RitualEntry>,
}

static RITUALS: OnceLock<Vec<RitualEntry>> = OnceLock::new();

/// RIT-05: returns the full ritual corpus as a static slice.
///
/// First call parses all embedded corpus files from `ALL_CORPUS_JSONS`, asserts
/// the schema version on each, and NFC-normalizes every text field. Panics on
/// any error — corpus is compile-embedded so a parse failure is a build-time
/// bug, not a runtime condition (mirrors `holiday_data.rs:117` and
/// `golden_loader.rs:6`).
pub fn all_rituals() -> &'static [RitualEntry] {
    RITUALS
        .get_or_init(|| {
            let mut all: Vec<RitualEntry> = Vec::new();
            for (i, json) in ALL_CORPUS_JSONS.iter().enumerate() {
                let file: RitualFile = serde_json::from_str(json)
                    .unwrap_or_else(|e| panic!("Failed to parse corpus file index {i}: {e}"));
                assert_eq!(
                    file.schema_version, EXPECTED_SCHEMA_VERSION,
                    "corpus file index {i} schema_version must equal {:?} (ADR-0001); found {:?}",
                    EXPECTED_SCHEMA_VERSION, file.schema_version
                );
                for entry in file.entries {
                    all.push(normalize_and_validate(entry));
                }
            }
            all
        })
        .as_slice()
}

fn normalize_and_validate(mut entry: RitualEntry) -> RitualEntry {
    // RIT-08 source_id discipline: every entry MUST equal the constant.
    // We compare against the constant (not the bare literal) so the
    // source_id_guard CI test stays green.
    assert_eq!(
        entry.source_id, SOURCE_VN_FOLK_RITUAL,
        "ritual {:?} has source_id {:?}, expected {:?}",
        entry.ritual_id, entry.source_id, SOURCE_VN_FOLK_RITUAL
    );

    // RIT-08 NFC normalization: every text field gets passed through `nfc()`.
    // `is_nfc()` returns true for already-canonical text -> fast early-out.
    entry.title_vi = nfc(&entry.title_vi);
    entry.invocation_text_vi = nfc(&entry.invocation_text_vi);
    if let Some(t) = entry.title_en.as_deref() {
        entry.title_en = Some(nfc(t));
    }
    if let Some(b) = entry.body_en.as_deref() {
        entry.body_en = Some(nfc(b));
    }
    for off in entry.offerings.iter_mut() {
        off.name_vi = nfc(&off.name_vi);
        if let Some(s) = off.name_en.as_deref() {
            off.name_en = Some(nfc(s));
        }
        if let Some(s) = off.quantity.as_deref() {
            off.quantity = Some(nfc(s));
        }
        if let Some(s) = off.notes.as_deref() {
            off.notes = Some(nfc(s));
        }
    }
    for step in entry.preparation_steps.iter_mut() {
        step.description_vi = nfc(&step.description_vi);
        if let Some(s) = step.description_en.as_deref() {
            step.description_en = Some(nfc(s));
        }
    }
    for note in entry.notes.iter_mut() {
        *note = nfc(note);
    }
    entry
}

fn nfc(s: &str) -> String {
    if is_nfc(s) {
        s.to_string()
    } else {
        s.nfc().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    use crate::rituals::schema::RitualEventKey;

    // ---------------------------------------------------------------------------
    // Legacy tests (preserved from Phase 11 — updated thresholds where needed)
    // ---------------------------------------------------------------------------

    #[test]
    fn corpus_loads_with_at_least_five_entries() {
        let rituals = all_rituals();
        assert!(
            rituals.len() >= 5,
            "expected at least 5 entries, got {}",
            rituals.len()
        );
    }

    #[test]
    fn every_entry_has_correct_source_id() {
        for entry in all_rituals() {
            assert_eq!(
                entry.source_id, SOURCE_VN_FOLK_RITUAL,
                "ritual {} has wrong source_id",
                entry.ritual_id
            );
        }
    }

    #[test]
    fn every_text_field_is_nfc_normalized() {
        for entry in all_rituals() {
            assert!(
                is_nfc(&entry.title_vi),
                "title_vi not NFC for {}",
                entry.ritual_id
            );
            assert!(
                is_nfc(&entry.invocation_text_vi),
                "invocation_text_vi not NFC for {}",
                entry.ritual_id
            );
            if let Some(t) = &entry.title_en {
                assert!(is_nfc(t), "title_en not NFC");
            }
            if let Some(b) = &entry.body_en {
                assert!(is_nfc(b), "body_en not NFC");
            }
            for off in &entry.offerings {
                assert!(
                    is_nfc(&off.name_vi),
                    "offering name_vi not NFC for {}",
                    entry.ritual_id
                );
                if let Some(s) = &off.name_en {
                    assert!(is_nfc(s), "offering name_en not NFC");
                }
                if let Some(s) = &off.quantity {
                    assert!(is_nfc(s), "offering quantity not NFC");
                }
                if let Some(s) = &off.notes {
                    assert!(is_nfc(s), "offering notes not NFC");
                }
            }
            for step in &entry.preparation_steps {
                assert!(
                    is_nfc(&step.description_vi),
                    "step description_vi not NFC for {}",
                    entry.ritual_id
                );
                if let Some(s) = &step.description_en {
                    assert!(is_nfc(s), "step description_en not NFC");
                }
            }
            for note in &entry.notes {
                assert!(is_nfc(note), "note not NFC for {}", entry.ritual_id);
            }
        }
    }

    #[test]
    fn get_or_init_is_idempotent() {
        let a = all_rituals();
        let b = all_rituals();
        assert_eq!(
            a.as_ptr(),
            b.as_ptr(),
            "OnceLock should return the same slice on subsequent calls"
        );
        assert_eq!(a.len(), b.len());
    }

    #[test]
    fn known_ritual_id_is_present() {
        let rituals = all_rituals();
        assert!(
            rituals
                .iter()
                .any(|r| r.ritual_id == "van-khan-tet-don-gian"),
            "expected ritual_id 'van-khan-tet-don-gian' to be present (fixture entry 1)"
        );
    }

    // ---------------------------------------------------------------------------
    // RIT-09: corpus has at least 60 entries
    // ---------------------------------------------------------------------------

    #[test]
    fn corpus_has_at_least_sixty_entries() {
        let count = all_rituals().len();
        assert!(
            count >= 60,
            "RIT-09: expected at least 60 entries from merged corpus, got {}",
            count
        );
    }

    // ---------------------------------------------------------------------------
    // RIT-10: every entry has a citation with a page number
    // ---------------------------------------------------------------------------

    #[test]
    fn every_entry_has_citation_with_page() {
        for entry in all_rituals() {
            assert!(
                entry.original_citation.page.is_some(),
                "RIT-10: ritual {:?} is missing original_citation.page",
                entry.ritual_id
            );
        }
    }

    // ---------------------------------------------------------------------------
    // RIT-12: at least 4 distinct events each have >= 2 variant values
    // ---------------------------------------------------------------------------

    #[test]
    fn at_least_four_events_have_multiple_variants() {
        // Group entries by their first "discriminator" key:
        //   HolidayId  -> use value string
        //   SolarTerm  -> "solar:{name}"
        //   LifeEvent  -> "life:{event:?}"
        //   LunarDate / Always -> skipped for grouping purposes
        let mut event_variant_count: HashMap<String, HashSet<String>> = HashMap::new();

        for entry in all_rituals() {
            let discriminator = entry.event_keys.iter().find_map(|k| match k {
                RitualEventKey::HolidayId { value } => Some(value.clone()),
                RitualEventKey::SolarTerm { name } => Some(format!("solar:{name}")),
                RitualEventKey::LifeEvent { event } => Some(format!("life:{event:?}")),
                RitualEventKey::LunarDate { .. } | RitualEventKey::Always => None,
            });

            if let Some(disc) = discriminator {
                event_variant_count
                    .entry(disc)
                    .or_default()
                    .insert(format!("{:?}", entry.variant));
            }
        }

        let multi_variant_count = event_variant_count
            .values()
            .filter(|v| v.len() >= 2)
            .count();

        assert!(
            multi_variant_count >= 4,
            "RIT-12: expected >= 4 events with multiple variants, got {}. Map: {:?}",
            multi_variant_count,
            event_variant_count
        );
    }

    // ---------------------------------------------------------------------------
    // RIT-13: body_en is reserved and never populated in v1.5 corpus
    // ---------------------------------------------------------------------------

    #[test]
    fn body_en_is_reserved_and_unset() {
        for entry in all_rituals() {
            assert!(
                entry.body_en.is_none(),
                "RIT-13: ritual {:?} has body_en set (reserved field, must be null in v1.5 corpus)",
                entry.ritual_id
            );
        }
    }
}
