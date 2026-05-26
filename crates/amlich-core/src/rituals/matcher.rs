//! Lookup APIs for the ritual corpus. See ADR-0001 for the schema and
//! 11-RESEARCH.md §Architecture/Matcher for the design rationale.
//!
//! Four public functions:
//!   - `find_van_khan_for_snapshot` (RIT-01) — derives event keys from a DaySnapshot
//!     then filters the corpus.
//!   - `find_van_khan_for_event` (RIT-02) — direct lookup by RitualEventKey.
//!   - `find_van_khan_for_life_event` (RIT-03) — convenience wrapper over RIT-02.
//!   - `get_ritual_by_id` (RIT-04) — linear scan by stable id.
//!
//! Snapshot derivation does NOT emit `LifeEvent` keys; those are caller intents
//! (Động thổ / Cưới / etc.), not day properties (research Q4).

use crate::holidays::get_vietnamese_holidays;
use crate::rituals::corpus::all_rituals;
use crate::rituals::schema::{LeapPolicy, LifeEventKind, RitualEntry, RitualEventKey};
use crate::DaySnapshot;

/// RIT-01: returns every ritual whose `event_keys[]` overlaps the snapshot's
/// derived event-key set.
pub fn find_van_khan_for_snapshot(snapshot: &DaySnapshot) -> Vec<&'static RitualEntry> {
    let needles = derive_event_keys(snapshot);
    all_rituals()
        .iter()
        .filter(|entry| {
            entry
                .event_keys
                .iter()
                .any(|haystack| needles.iter().any(|needle| event_key_matches(haystack, needle)))
        })
        .collect()
}

/// RIT-02: direct lookup by a single event key.
pub fn find_van_khan_for_event(event: &RitualEventKey) -> Vec<&'static RitualEntry> {
    all_rituals()
        .iter()
        .filter(|e| e.event_keys.iter().any(|k| event_key_matches(k, event)))
        .collect()
}

/// RIT-03: convenience for life-event lookup. Wraps the kind as
/// `RitualEventKey::LifeEvent` and delegates.
pub fn find_van_khan_for_life_event(kind: LifeEventKind) -> Vec<&'static RitualEntry> {
    let needle = RitualEventKey::LifeEvent { event: kind };
    find_van_khan_for_event(&needle)
}

/// RIT-04: single-entry lookup by stable id.
pub fn get_ritual_by_id(ritual_id: &str) -> Option<&'static RitualEntry> {
    all_rituals().iter().find(|e| e.ritual_id == ritual_id)
}

/// Derive the set of event-key needles applicable to one day:
///   (a) one HolidayId per holiday landing on this solar date whose `Holiday.id` is Some
///   (b) one LunarDate with the snapshot's lunar month/day + leap policy
///   (c) one SolarTerm with the snapshot's Tiết Khí name
///   (d) Always (sentinel — every entry tagged Always matches)
///
/// Does NOT emit LifeEvent keys. Life events are caller intents, not day
/// properties (research Q4).
fn derive_event_keys(snapshot: &DaySnapshot) -> Vec<RitualEventKey> {
    let ctx = &snapshot.context;
    let mut keys: Vec<RitualEventKey> = Vec::new();

    // (a) Holiday ids — join via solar date; skip auto-gen entries with id=None.
    for h in get_vietnamese_holidays(ctx.solar.year) {
        if h.solar_day == ctx.solar.day && h.solar_month == ctx.solar.month {
            if let Some(id) = &h.id {
                keys.push(RitualEventKey::HolidayId { value: id.clone() });
            }
        }
    }

    // (b) Lunar month-day with leap policy reflecting snapshot reality.
    let lunar_month = ctx.lunar.month as u8;
    let lunar_day = ctx.lunar.day as u8;
    keys.push(RitualEventKey::LunarDate {
        month: lunar_month,
        day: lunar_day,
        leap_month_policy: if ctx.lunar.is_leap {
            LeapPolicy::LeapMonthOnly
        } else {
            LeapPolicy::CanonicalMonthOnly
        },
    });

    // (c) Tiết Khí anchor — exact name string match against entry SolarTerm keys.
    keys.push(RitualEventKey::SolarTerm {
        name: ctx.tiet_khi.name.clone(),
    });

    // (d) Always sentinel.
    keys.push(RitualEventKey::Always);

    keys
}

/// Symmetric matcher: returns true when the two keys denote the same event,
/// honouring leap-month policy semantics for LunarDate pairs.
///
/// Cross-variant non-matches collapse to a single `_ => false` arm. This is
/// safe today because the RitualEventKey variant set is locked by ADR-0001 and
/// further closed by `tests/source_id_guard.rs` discipline. A future ADR that
/// adds a 6th variant MUST update this match — see the must_haves note in
/// 11-03-PLAN.md and ADR-0001 §Schema Discipline.
fn event_key_matches(haystack: &RitualEventKey, needle: &RitualEventKey) -> bool {
    use RitualEventKey::*;
    match (haystack, needle) {
        (Always, _) | (_, Always) => true,
        (HolidayId { value: a }, HolidayId { value: b }) => a == b,
        (SolarTerm { name: a }, SolarTerm { name: b }) => a == b,
        (LifeEvent { event: a }, LifeEvent { event: b }) => a == b,
        (
            LunarDate { month: m1, day: d1, leap_month_policy: p },
            LunarDate { month: m2, day: d2, leap_month_policy: q },
        ) => {
            if m1 != m2 || d1 != d2 {
                return false;
            }
            // Leap-policy reconciliation: `Either` matches anything; otherwise
            // the two policies must be identical.
            matches!(
                (p, q),
                (LeapPolicy::Either, _)
                    | (_, LeapPolicy::Either)
                    | (LeapPolicy::CanonicalMonthOnly, LeapPolicy::CanonicalMonthOnly)
                    | (LeapPolicy::LeapMonthOnly, LeapPolicy::LeapMonthOnly)
            )
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calculate_day_snapshot;

    // RIT-01: Tết Nguyên Đán 2024 = solar 2024-02-10 (lunar 1/1).
    // The Tết simple-variant fixture has event_keys including HolidayId{"tet-nguyen-dan"}.
    #[test]
    fn tet_snapshot_returns_tet_rituals() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let hits = find_van_khan_for_snapshot(&snapshot);
        assert!(
            hits.iter().any(|r| r.ritual_id == "van-khan-tet-don-gian"),
            "expected 'van-khan-tet-don-gian' in Tết 2024 results, got: {:?}",
            hits.iter().map(|r| &r.ritual_id).collect::<Vec<_>>()
        );
    }

    // RIT-02: direct lookup by HolidayId returns ≥ 1 entry for Tết.
    #[test]
    fn find_van_khan_for_event_holiday_id_tet() {
        let needle = RitualEventKey::HolidayId { value: "tet-nguyen-dan".to_string() };
        let hits = find_van_khan_for_event(&needle);
        assert!(!hits.is_empty(), "expected ≥ 1 entry for tet-nguyen-dan");
    }

    // RIT-03: life-event lookup returns the Động thổ fixture.
    #[test]
    fn find_van_khan_for_life_event_dong_tho() {
        let hits = find_van_khan_for_life_event(LifeEventKind::DongTho);
        assert!(
            hits.iter().any(|r| r.ritual_id == "van-khan-dong-tho"),
            "expected 'van-khan-dong-tho' for LifeEventKind::DongTho"
        );
    }

    // RIT-04: id-based lookup.
    #[test]
    fn get_ritual_by_id_known() {
        let hit = get_ritual_by_id("van-khan-tet-don-gian");
        assert!(hit.is_some(), "expected Some for known ritual_id");
    }

    #[test]
    fn get_ritual_by_id_unknown_is_none() {
        assert!(get_ritual_by_id("does-not-exist").is_none());
    }

    // RIT-06: Always sentinel matches every needle, in both directions.
    #[test]
    fn always_sentinel_matches_anything() {
        assert!(event_key_matches(&RitualEventKey::Always, &RitualEventKey::HolidayId { value: "x".to_string() }));
        assert!(event_key_matches(&RitualEventKey::HolidayId { value: "x".to_string() }, &RitualEventKey::Always));
        // The "fall-through" Always-entry from fixtures.json:
        let hits = find_van_khan_for_event(&RitualEventKey::Always);
        assert!(hits.iter().any(|r| r.ritual_id == "van-khan-gia-tien-hang-ngay"));
    }

    // RIT-07: leap policy semantics — canonical entry MUST NOT match leap snapshot.
    #[test]
    fn leap_policy_canonical_does_not_match_leap() {
        let canonical = RitualEventKey::LunarDate {
            month: 5,
            day: 5,
            leap_month_policy: LeapPolicy::CanonicalMonthOnly,
        };
        let leap = RitualEventKey::LunarDate {
            month: 5,
            day: 5,
            leap_month_policy: LeapPolicy::LeapMonthOnly,
        };
        assert!(!event_key_matches(&canonical, &leap),
            "CanonicalMonthOnly must NOT match LeapMonthOnly");
    }

    // RIT-07: `Either` matches both leap and canonical sides.
    #[test]
    fn leap_policy_either_matches_both() {
        let either = RitualEventKey::LunarDate { month: 1, day: 1, leap_month_policy: LeapPolicy::Either };
        let canonical = RitualEventKey::LunarDate { month: 1, day: 1, leap_month_policy: LeapPolicy::CanonicalMonthOnly };
        let leap = RitualEventKey::LunarDate { month: 1, day: 1, leap_month_policy: LeapPolicy::LeapMonthOnly };
        assert!(event_key_matches(&either, &canonical));
        assert!(event_key_matches(&either, &leap));
        assert!(event_key_matches(&canonical, &either));
        assert!(event_key_matches(&leap, &either));
    }

    // Different month or day → no match regardless of policy.
    #[test]
    fn lunar_date_month_or_day_mismatch_never_matches() {
        let a = RitualEventKey::LunarDate { month: 1, day: 1, leap_month_policy: LeapPolicy::Either };
        let b = RitualEventKey::LunarDate { month: 1, day: 2, leap_month_policy: LeapPolicy::Either };
        let c = RitualEventKey::LunarDate { month: 2, day: 1, leap_month_policy: LeapPolicy::Either };
        assert!(!event_key_matches(&a, &b));
        assert!(!event_key_matches(&a, &c));
    }
}
