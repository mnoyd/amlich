//! OnceLock-backed corpus loader for Văn khấn ritual entries.
//!
//! RED-phase stub: `all_rituals()` is `todo!()` so the inline tests fail.
//! GREEN phase replaces the stub with the full loader.

use crate::rituals::schema::RitualEntry;

#[allow(dead_code)]
pub fn all_rituals() -> &'static [RitualEntry] {
    todo!("implemented in GREEN phase")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sources::SOURCE_VN_FOLK_RITUAL;
    use unicode_normalization::is_nfc;

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
