//! OnceLock-backed corpus loader for Văn khấn ritual entries.
//!
//! The corpus is embedded at compile time via `include_str!` and parsed once on
//! first access. Every text field is NFC-normalized at load (RIT-08); every
//! entry's `source_id` is validated against `crate::sources::SOURCE_VN_FOLK_RITUAL`.
//!
//! Schema is frozen by ADR-0001. Any change to the loaded shape requires a
//! superseding ADR and a bump from `"rituals-v1"` to `"rituals-vN"` in
//! `$schema_version` on the corpus files.

use serde::Deserialize;
use std::sync::OnceLock;
use unicode_normalization::{is_nfc, UnicodeNormalization};

use crate::rituals::schema::RitualEntry;
use crate::sources::SOURCE_VN_FOLK_RITUAL;

const RITUAL_FIXTURES_JSON: &str =
    include_str!("../../data/rituals/fixtures.json");

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
/// First call parses the embedded `data/rituals/fixtures.json`, asserts the
/// schema version, and NFC-normalizes every text field. Panics on any error —
/// corpus is compile-embedded so a parse failure is a build-time bug, not a
/// runtime condition (mirrors `holiday_data.rs:117` and `golden_loader.rs:6`).
pub fn all_rituals() -> &'static [RitualEntry] {
    RITUALS
        .get_or_init(|| {
            let file: RitualFile = serde_json::from_str(RITUAL_FIXTURES_JSON)
                .expect("Failed to parse data/rituals/fixtures.json");
            assert_eq!(
                file.schema_version, EXPECTED_SCHEMA_VERSION,
                "ritual corpus schema_version must equal {:?} (ADR-0001); found {:?}",
                EXPECTED_SCHEMA_VERSION, file.schema_version
            );
            file.entries
                .into_iter()
                .map(normalize_and_validate)
                .collect()
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

    #[test]
    fn corpus_loads_with_at_least_five_entries() {
        let rituals = all_rituals();
        assert!(
            rituals.len() >= 5,
            "expected at least 5 stub entries from fixtures.json, got {}",
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
            assert!(is_nfc(&entry.title_vi), "title_vi not NFC for {}", entry.ritual_id);
            assert!(is_nfc(&entry.invocation_text_vi), "invocation_text_vi not NFC for {}", entry.ritual_id);
            if let Some(t) = &entry.title_en { assert!(is_nfc(t), "title_en not NFC"); }
            if let Some(b) = &entry.body_en { assert!(is_nfc(b), "body_en not NFC"); }
            for off in &entry.offerings {
                assert!(is_nfc(&off.name_vi), "offering name_vi not NFC for {}", entry.ritual_id);
                if let Some(s) = &off.name_en { assert!(is_nfc(s), "offering name_en not NFC"); }
                if let Some(s) = &off.quantity { assert!(is_nfc(s), "offering quantity not NFC"); }
                if let Some(s) = &off.notes { assert!(is_nfc(s), "offering notes not NFC"); }
            }
            for step in &entry.preparation_steps {
                assert!(is_nfc(&step.description_vi), "step description_vi not NFC for {}", entry.ritual_id);
                if let Some(s) = &step.description_en { assert!(is_nfc(s), "step description_en not NFC"); }
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
        assert_eq!(a.as_ptr(), b.as_ptr(), "OnceLock should return the same slice on subsequent calls");
        assert_eq!(a.len(), b.len());
    }

    #[test]
    fn known_ritual_id_is_present() {
        let rituals = all_rituals();
        assert!(
            rituals.iter().any(|r| r.ritual_id == "van-khan-tet-don-gian"),
            "expected ritual_id 'van-khan-tet-don-gian' to be present (fixture entry 1)"
        );
    }
}
