//! Integration tests for Phase 11 ritual lookup APIs.
//!
//! These tests treat amlich-core as an EXTERNAL crate (via `amlich_core::...`)
//! to confirm the Phase 11 module's public re-exports work end-to-end. Inline
//! `#[cfg(test)] mod tests` in `matcher.rs` and `corpus.rs` cover white-box
//! invariants; this file covers black-box behavior matching ROADMAP §Phase 11
//! Success Criteria #1, #4, #5.

use amlich_core::holidays::get_vietnamese_holidays;
use amlich_core::rituals::{
    all_rituals, find_van_khan_for_event, find_van_khan_for_snapshot, LeapPolicy,
    RitualEventKey,
};
use amlich_core::{calculate_day_snapshot, DaySnapshot};

// Canonical provenance ledger embedded at compile time (RIT-14 + RIT-15 closure
// evidence; see crates/amlich-core/data/rituals/provenance_audit.md). The
// relative path resolves because tests/ lives under crates/amlich-core/tests/
// and the ledger lives under crates/amlich-core/data/rituals/ — same shape as
// the existing JSON `include_str!` paths in corpus.rs.
const PROVENANCE_AUDIT_MD: &str =
    include_str!("../data/rituals/provenance_audit.md");

// ─── Test 1: RIT-01 Tết snapshot wiring ──────────────────────────────────────
#[test]
fn tet_nguyen_dan_2024_snapshot_returns_tet_ritual() {
    // Tết 2024 = solar 2024-02-10, lunar 1/1.
    let snapshot: DaySnapshot = calculate_day_snapshot(10, 2, 2024);
    let hits = find_van_khan_for_snapshot(&snapshot);
    assert!(!hits.is_empty(), "Tết 2024 should return ≥ 1 ritual");

    let has_tet_holiday_key = hits.iter().any(|entry| {
        entry.event_keys.iter().any(|k| matches!(
            k,
            RitualEventKey::HolidayId { value } if value == "tet-nguyen-dan"
        ))
    });
    assert!(
        has_tet_holiday_key,
        "expected at least one returned ritual to carry HolidayId{{\"tet-nguyen-dan\"}}; got: {:?}",
        hits.iter().map(|r| &r.ritual_id).collect::<Vec<_>>()
    );
}

// ─── Test 2: Sóc/Vọng snapshot path — falsifiable via van-khan-ram-thang-gieng ─
#[test]
fn vong_snapshot_returns_ram_thang_gieng_via_snapshot_path() {
    // 2024-02-24 = lunar 1/15 = Tết Nguyên Tiêu (Vọng of month 1).
    let snapshot = calculate_day_snapshot(24, 2, 2024);

    // Anchor sanity: the conversion really did land on lunar day 15. If this
    // assertion fires, the lunar-conversion contract drifted and the rest of
    // the test is meaningless — surface that as the root cause.
    assert_eq!(
        snapshot.context.lunar.day, 15,
        "anchor date 2024-02-24 must be lunar day 15; lunar conversion drifted"
    );

    let hits = find_van_khan_for_snapshot(&snapshot);

    // FALSIFIABLE: the 11-01 fixture `van-khan-ram-thang-gieng` carries
    //   event_keys: [HolidayId{tet-nguyen-tieu}, LunarDate{m=1,d=15}]
    // Either of those needles must fire from `derive_event_keys` on this
    // snapshot. If hits is empty (or missing the fixture), the snapshot-path
    // Sóc/Vọng plumbing is broken — fail loudly.
    assert!(
        hits.iter().any(|r| r.ritual_id == "van-khan-ram-thang-gieng"),
        "Vọng 2024-02-24 must return 'van-khan-ram-thang-gieng' via the snapshot path; \
         got: {:?}",
        hits.iter().map(|r| &r.ritual_id).collect::<Vec<_>>()
    );

    // Honesty check: every hit must trace to a snapshot-derivable key.
    // `derive_event_keys` emits: HolidayId (per matching real holiday),
    // LunarDate (current month/day + policy), SolarTerm (current tiết khí),
    // Always. A hit with none of those is a matcher bug.
    for entry in &hits {
        let via_day_15 = entry.event_keys.iter().any(|k| matches!(
            k,
            RitualEventKey::LunarDate { day: 15, .. }
        ));
        let via_holiday = entry.event_keys.iter().any(|k| matches!(
            k,
            RitualEventKey::HolidayId { .. }
        ));
        let via_solar_term = entry.event_keys.iter().any(|k| matches!(
            k,
            RitualEventKey::SolarTerm { .. }
        ));
        let via_always = entry.event_keys.iter().any(|k| matches!(k, RitualEventKey::Always));
        assert!(
            via_day_15 || via_holiday || via_solar_term || via_always,
            "ritual {} fired on Vọng 2024-02-24 but has no day-15/holiday/solar-term/always event key",
            entry.ritual_id
        );
    }
}

// ─── Test 3: Thanh Minh via SolarTerm path (Holiday.id=None — only SolarTerm fires) ──
#[test]
fn thanh_minh_snapshot_returns_thanh_minh_ritual_via_solar_term_path() {
    let mut observed_tiet_khi: Vec<String> = Vec::new();
    let mut thanh_minh_day: Option<i32> = None;
    for day in 1..=10 {
        let snap = calculate_day_snapshot(day, 4, 2024);
        observed_tiet_khi.push(format!("{}/04: {}", day, snap.context.tiet_khi.name));
        if snap.context.tiet_khi.name == "Thanh Minh" {
            thanh_minh_day = Some(day);
            break;
        }
    }
    let day = thanh_minh_day.unwrap_or_else(|| {
        panic!(
            "no day in 2024-04-01..2024-04-10 has tiet_khi == \"Thanh Minh\". Observed:\n{}",
            observed_tiet_khi.join("\n")
        )
    });

    let snapshot = calculate_day_snapshot(day, 4, 2024);
    let hits = find_van_khan_for_snapshot(&snapshot);
    assert!(
        hits.iter().any(|r| r.ritual_id == "van-khan-thanh-minh"),
        "Thanh Minh 2024-04-{:02} should return 'van-khan-thanh-minh'; got: {:?}",
        day,
        hits.iter().map(|r| &r.ritual_id).collect::<Vec<_>>()
    );
}

// ─── Test 4: HolidayId cross-reference (no typos in fixtures) ────────────────
#[test]
fn every_holiday_id_in_fixtures_resolves_to_a_real_holiday() {
    use std::collections::HashSet;

    // Collect every HolidayId.value from the entire corpus.
    let mut fixture_ids: HashSet<String> = HashSet::new();
    for entry in all_rituals() {
        for key in &entry.event_keys {
            if let RitualEventKey::HolidayId { value } = key {
                fixture_ids.insert(value.clone());
            }
        }
    }

    // Collect every real Holiday.id across multiple years to cover any year-offset
    // edge cases (some holidays have year_offset = -1 / +1).
    let mut real_ids: HashSet<String> = HashSet::new();
    for year in 2020..=2030 {
        for h in get_vietnamese_holidays(year) {
            if let Some(id) = h.id {
                real_ids.insert(id);
            }
        }
    }

    let missing: Vec<&String> = fixture_ids.difference(&real_ids).collect();
    assert!(
        missing.is_empty(),
        "fixture HolidayId values not found in any 2020-2030 holiday corpus: {:?}",
        missing
    );
}

// ─── Test 5: NFC round-trip byte-equal serialization ─────────────────────────
#[test]
fn every_entry_round_trips_byte_equal_through_serde_json() {
    for entry in all_rituals() {
        let first = serde_json::to_string(entry).expect("serialize");
        let parsed: amlich_core::rituals::RitualEntry =
            serde_json::from_str(&first).expect("deserialize round-trip");
        let second = serde_json::to_string(&parsed).expect("re-serialize");
        assert_eq!(
            first, second,
            "ritual {} did not round-trip byte-equal (NFC drift in serde path?)",
            entry.ritual_id
        );
    }
}

// ─── Test 6: leap-policy semantics at the public API surface ─────────────────
#[test]
fn leap_month_only_needle_does_not_match_canonical_only_entry() {
    // The Đoan Ngọ fixture (plan 11-01) has:
    //   event_keys: [{kind:holiday_id, value:tet-doan-ngo},
    //                {kind:lunar_date, month:5, day:5, leap_month_policy:canonical_month_only}]
    let leap_needle = RitualEventKey::LunarDate {
        month: 5,
        day: 5,
        leap_month_policy: LeapPolicy::LeapMonthOnly,
    };
    let leap_hits = find_van_khan_for_event(&leap_needle);
    assert!(
        !leap_hits.iter().any(|r| r.ritual_id == "van-khan-doan-ngo"),
        "Đoan Ngọ (CanonicalMonthOnly) MUST NOT match a LeapMonthOnly needle"
    );

    let canonical_needle = RitualEventKey::LunarDate {
        month: 5,
        day: 5,
        leap_month_policy: LeapPolicy::CanonicalMonthOnly,
    };
    let canonical_hits = find_van_khan_for_event(&canonical_needle);
    assert!(
        canonical_hits.iter().any(|r| r.ritual_id == "van-khan-doan-ngo"),
        "Đoan Ngọ (CanonicalMonthOnly) MUST match a CanonicalMonthOnly needle"
    );
}

// ─── Test 7 (Plan 17-02 / RIT-14 + RIT-15 closure check) ─────────────────────
// Black-box ledger invariants test. Parses the canonical reviewer-audit ledger
// at test time and asserts: (a) ledger has 60 rows, (b) ledger IDs == corpus
// IDs (no orphans either direction), (c) every row uses a controlled method /
// outcome token, (d) outcome counts sum to 60, (e) every reviewer cell is a
// valid ExternalReviewPending(...) marker, (f) no bare `pending` cell remains.
// The ledger is the canonical reader-of-record; if this test fails the ledger
// is broken, NOT the test.
#[test]
fn every_ledger_row_passes_invariants() {
    use std::collections::HashSet;
    let rows = ledger::parse_ledger(PROVENANCE_AUDIT_MD);
    assert_eq!(rows.len(), 60, "RIT-14/RIT-15: ledger must have 60 rows");

    // 1:1 ledger <-> corpus ID parity
    let ledger_ids: HashSet<&str> = rows.iter().map(|r| r.ritual_id.as_str()).collect();
    let corpus_ids: HashSet<&str> = all_rituals().iter().map(|r| r.ritual_id.as_str()).collect();
    assert_eq!(
        ledger_ids, corpus_ids,
        "RIT-14: ledger IDs and corpus IDs must match exactly (no orphans)"
    );

    // Outcome counts sum to 60
    let mut total = 0usize;
    for outcome in ledger::OUTCOMES {
        total += ledger::count_outcome(&rows, outcome);
    }
    assert_eq!(total, 60, "RIT-15: outcome counts must sum to 60");

    // Per-row invariants
    for r in &rows {
        assert!(
            ledger::METHODS.contains(&r.method.as_str()),
            "row {}: method_of_review {:?} not in controlled tokens",
            r.ritual_id,
            r.method
        );
        assert!(
            ledger::OUTCOMES.contains(&r.outcome.as_str()),
            "row {}: outcome {:?} not in controlled tokens",
            r.ritual_id,
            r.outcome
        );
        assert!(
            !r.date_reviewed.trim().is_empty(),
            "row {}: date_reviewed is empty",
            r.ritual_id
        );
        ledger::validate_marker(&r.reviewer).unwrap_or_else(|e| {
            panic!("row {}: invalid reviewer marker: {}", r.ritual_id, e)
        });
    }

    // Legacy `pending` placeholder must be gone
    ledger::assert_no_bare_pending(PROVENANCE_AUDIT_MD);
}

// ─── Test 8 (Plan 17-02 / RIT-16 corrected-entry gate) ────────────────────────
// For every ledger row whose outcome == "corrected", re-verify that:
//   1. The corrected ID resolves to exactly one corpus entry (via all_rituals()).
//   2. The entry's `invocation_text_vi` (LOCKED body field per ADR-0001 —
//      NEVER `body_vi`) is non-empty (source content present).
//   3. The entry round-trips byte-equal through serde_json after the locked
//      schema parse + NFC-at-load guards (mirrors Test 5 at lines 155-169).
//
// Phase 17 closure state: 0 corrected rows. The Pitfall-7 guard (parser must
// successfully read 60 rows BEFORE the corrected_count == 0 assertion) prevents
// a vacuous pass when the ledger parser silently drops rows. The forward-
// compatible loop body runs once per future corrected row.
#[test]
fn every_corrected_entry_passes_schema_and_nfc_round_trip() {
    let rows = ledger::parse_ledger(PROVENANCE_AUDIT_MD);
    assert_eq!(
        rows.len(),
        60,
        "vacuous corrected-entry test: parser read {} rows from provenance_audit.md (expected 60)",
        rows.len()
    );

    let corrected_ids = ledger::find_corrected_ids(&rows);
    let corrected_count = corrected_ids.len();
    assert_eq!(
        corrected_count, 0,
        "Phase 17 closure state: 60 entries deferred as ExternalReviewPending, 0 corrected. \
         When a future phase marks an entry 'corrected' after source re-verification, \
         this test will round-trip that entry through schema + NFC + serde. \
         Got corrected_count = {corrected_count}."
    );

    // Loop body is forward-compatible: runs once per corrected ledger row.
    for ritual_id in &corrected_ids {
        let entry = all_rituals()
            .iter()
            .find(|e| e.ritual_id == *ritual_id)
            .unwrap_or_else(|| {
                panic!("corrected ledger ID {ritual_id} is absent from the loaded corpus")
            });
        assert!(
            !entry.invocation_text_vi.trim().is_empty(),
            "corrected ritual {ritual_id} has empty invocation_text_vi (source content missing)"
        );

        let first = serde_json::to_string(entry).expect("serialize corrected entry");
        let parsed: amlich_core::rituals::RitualEntry =
            serde_json::from_str(&first).expect("deserialize corrected entry");
        let second = serde_json::to_string(&parsed).expect("re-serialize corrected entry");
        assert_eq!(
            first, second,
            "corrected ritual {ritual_id} did not round-trip byte-equal through serde_json"
        );
    }
}

// ─── Ledger parser (test-only, Markdown pipe-table) ───────────────────────────
//
// This is test scaffolding: it parses the canonical reviewer-audit ledger
// (`provenance_audit.md`) at test time so the invariants + corrected-entry
// tests cannot drift from the audit. The parser is intentionally minimal —
// it does NOT cover general Markdown. If the ledger format changes, update
// the parser here. The ledger is the canonical record; the parser follows.
mod ledger {
    /// Controlled method_of_review tokens (Plan 17-02 locked contract).
    pub const METHODS: &[&str] = &["independent-peer", "cross-source", "desk-check"];

    /// Controlled outcome tokens (Plan 17-02 locked contract).
    pub const OUTCOMES: &[&str] = &[
        "confirmed",
        "corrected",
        "disputed",
        "ExternalReviewPending",
    ];

    /// Exact 8-column header expected in every category sub-table.
    const HEADER: &str =
        "| ritual_id | classical_reference | page | confidence | reviewer | method_of_review | date_reviewed | outcome |";
    /// Exact separator row matching the 8-column header.
    const SEPARATOR: &str = "|---|---|---|---|---|---|---|---|";

    /// Parsed row of the reviewer-audit ledger (8 cells, in header order).
    ///
    /// All 8 cells are extracted for completeness even when invariants only
    /// inspect a subset — the parser follows the locked 8-column header
    /// contract; the invariants test is a separate layer.
    #[allow(dead_code)]
    pub struct LedgerRow {
        pub ritual_id: String,
        pub classical_reference: String,
        pub page: String,
        pub confidence: String,
        pub reviewer: String,
        pub method: String,
        pub date_reviewed: String,
        pub outcome: String,
    }

    /// Parse the canonical reviewer-audit ledger Markdown into `LedgerRow`s.
    ///
    /// Walks the file line-by-line, opening a section when an `### ` category
    /// sub-heading arrives, locating the 8-column header + separator under it,
    /// then accumulating data rows (lines starting with `| van-khan-`) until
    /// the next section or EOF. Hard-errors on any structural deviation.
    pub fn parse_ledger(text: &str) -> Vec<LedgerRow> {
        let mut rows: Vec<LedgerRow> = Vec::new();
        let mut in_section = false;
        let mut header_seen = false;

        for (idx, raw_line) in text.lines().enumerate() {
            let line = raw_line;
            let line_num = idx + 1;

            // New category sub-heading: close the previous section (if any)
            // AND open the new one (the heading marks the start of a new
            // section that contains its own header + separator + data rows).
            if line.starts_with("### ") {
                in_section = true;
                header_seen = false;
                continue;
            }

            // Outside a section: ignore everything (prose, blank lines, etc.).
            if !in_section {
                continue;
            }

            // Inside a section: locate the 8-column header.
            if line == HEADER {
                if header_seen {
                    panic!(
                        "provenance_audit.md line {line_num}: duplicate header row in the same section"
                    );
                }
                header_seen = true;
                continue;
            }

            // Match the separator row immediately under the header.
            if line == SEPARATOR {
                if !header_seen {
                    panic!(
                        "provenance_audit.md line {line_num}: separator row before any header"
                    );
                }
                continue;
            }

            // Non-data, non-header, non-separator lines inside a section are
            // ignored (e.g., blank lines, "Source file: ..." comments).
            if !line.starts_with("| van-khan-") {
                continue;
            }

            // Data row: split on `|`, trim each cell, assert exactly 8 cells.
            let cells: Vec<&str> = line.split('|').collect();
            // split on `|` yields N+1 cells where the first and last are empty
            // (before the leading `|` and after the trailing `|`); 8 data
            // columns therefore produce 10 entries.
            assert_eq!(
                cells.len(),
                10,
                "provenance_audit.md line {line_num}: data row has {} cells (expected 8): {line:?}",
                cells.len() - 2
            );
            let ritual_id = cells[1].trim().to_string();
            let classical_reference = cells[2].trim().to_string();
            let page = cells[3].trim().to_string();
            let confidence = cells[4].trim().to_string();
            let reviewer = cells[5].trim().to_string();
            let method = cells[6].trim().to_string();
            let date_reviewed = cells[7].trim().to_string();
            let outcome = cells[8].trim().to_string();

            rows.push(LedgerRow {
                ritual_id,
                classical_reference,
                page,
                confidence,
                reviewer,
                method,
                date_reviewed,
                outcome,
            });
        }

        rows
    }

    /// Count ledger rows whose `outcome` equals the supplied token (exact match
    /// after trimming; controlled-token comparison).
    pub fn count_outcome(rows: &[LedgerRow], outcome: &str) -> usize {
        rows.iter().filter(|r| r.outcome == outcome).count()
    }

    /// Return the ritual_ids of every ledger row whose `outcome` is `corrected`.
    /// Drives the forward-compatible RIT-16 round-trip loop.
    #[allow(dead_code)] // Used by every_corrected_entry_passes_schema_and_nfc_round_trip in Task 2.
    pub fn find_corrected_ids(rows: &[LedgerRow]) -> Vec<String> {
        rows.iter()
            .filter(|r| r.outcome == "corrected")
            .map(|r| r.ritual_id.clone())
            .collect()
    }

    /// Validate that `reviewer` is a well-formed
    /// `ExternalReviewPending(reason="..."; expected_review_date="YYYY-MM-DD"; assigned_to="...")`
    /// marker. Returns Err with a debuggable message on any structural defect.
    ///
    /// Required substrings:
    ///   - Opens with `ExternalReviewPending(` and closes with `)`.
    ///   - `expected_review_date="<non-empty>"`
    ///   - `reason="<non-empty>"`
    /// Optional substring:
    ///   - `assigned_to="<non-empty>"` (absent is allowed; if present, value
    ///     must be non-empty).
    pub fn validate_marker(reviewer: &str) -> Result<(), String> {
        if !(reviewer.starts_with("ExternalReviewPending(") && reviewer.ends_with(')')) {
            return Err(format!(
                "marker must open with ExternalReviewPending( and close with ); got: {reviewer:?}"
            ));
        }

        let inner = &reviewer["ExternalReviewPending(".len()..reviewer.len() - 1];

        // expected_review_date="<value>" — value must be non-empty.
        let date_value = extract_named_value(inner, "expected_review_date").ok_or_else(|| {
            format!("marker missing expected_review_date=\"...\"; inner={inner:?}")
        })?;
        if date_value.is_empty() {
            return Err("expected_review_date value is empty".into());
        }

        // reason="<value>" — value must be non-empty.
        let reason_value = extract_named_value(inner, "reason").ok_or_else(|| {
            format!("marker missing reason=\"...\"; inner={inner:?}")
        })?;
        if reason_value.is_empty() {
            return Err("reason value is empty".into());
        }

        // assigned_to is optional, but if present its value must be non-empty.
        if let Some(assigned_value) = extract_named_value(inner, "assigned_to") {
            if assigned_value.is_empty() {
                return Err("assigned_to value is empty".into());
            }
        }

        Ok(())
    }

    /// Extract `name="<value>"` from the marker inner text. Returns `None` if
    /// `name="` is absent. Trims surrounding whitespace around the value.
    fn extract_named_value(inner: &str, name: &str) -> Option<String> {
        let needle = format!("{name}=\"");
        let start = inner.find(&needle)? + needle.len();
        let rest = &inner[start..];
        let end = rest.find('"')?;
        Some(rest[..end].trim().to_string())
    }

    /// Assert that no pipe-delimited cell in the ledger text equals `pending`
    /// after trimming. The literal substring `pending` may appear elsewhere in
    /// prose (e.g., "deferred", "ExternalReviewPending"), but no data-row cell
    /// may be the bare legacy placeholder.
    pub fn assert_no_bare_pending(text: &str) {
        for (idx, line) in text.lines().enumerate() {
            // Only inspect lines that look like table rows (have leading `|`).
            if !line.contains('|') {
                continue;
            }
            for (col, cell) in line.split('|').enumerate() {
                if cell.trim() == "pending" {
                    panic!(
                        "provenance_audit.md line {} col {}: bare `| pending |` cell is forbidden (Phase 17 closure: use ExternalReviewPending(...) marker)",
                        idx + 1,
                        col
                    );
                }
            }
        }
    }
}
