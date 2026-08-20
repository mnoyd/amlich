//! v1.10 `amlich-l2zc.3` (EXPLAIN-01) cross-surface contract test
//! for the unified Traditional Wellness Context.
//!
//! Locks:
//!
//! - The unified context produced by
//!   `amlich_core::traditional_wellness::resolve_traditional_wellness_context_unified`
//!   is the single source of truth for the bilingual explanation,
//!   disclaimer, review state, time basis, divergence details, and
//!   evidence envelopes.
//! - The same `(snapshot, local_hour, local_minute)` triple yields
//!   byte-equal JSON whether the consumer reads it from the core
//!   library or from the `enrich_day_snapshot_with_traditional_wellness`
//!   helper that the Tauri command invokes.
//! - The enriched `DaySnapshot.traditional_wellness` field is the
//!   additive container: absent in JSON when None, present when Some,
//!   and byte-equal on round-trip.

use amlich_core::calculate_day_snapshot;
use amlich_core::enrich_day_snapshot_with_traditional_wellness;
use amlich_core::sources::{SOURCE_HUANGDI_NEIJING_SUWEN, SOURCE_SHI_ER_JING_NA_DI_ZHI};
use amlich_core::traditional_wellness::{
    resolve_traditional_wellness_context_unified, COMPOSITE_SEASONAL_WELLNESS,
};
use amlich_core::VIETNAM_TIMEZONE;

const JIAQIAO_HOUR: u8 = 9;
const JIAQIAO_MINUTE: u8 = 30;

fn snapshot_for_2026_08_16() -> amlich_core::DaySnapshot {
    calculate_day_snapshot(16, 8, 2026)
}

/// 1. Core `resolve_traditional_wellness_context_unified` and the
///    `enrich_day_snapshot_with_traditional_wellness` helper produce
///    byte-equal serialized Traditional Wellness Contexts for the
///    same `(date, local_hour, local_minute)` triple. This is the
///    contract the desktop, TUI, and API consumers depend on.
#[test]
fn core_and_helper_yield_byte_equal_unified_context() {
    let snapshot = snapshot_for_2026_08_16();
    let core_ctx = resolve_traditional_wellness_context_unified(
        snapshot.context.jd,
        VIETNAM_TIMEZONE,
        JIAQIAO_HOUR,
        JIAQIAO_MINUTE,
    );
    let enriched = enrich_day_snapshot_with_traditional_wellness(
        &snapshot,
        snapshot.context.jd,
        VIETNAM_TIMEZONE,
        JIAQIAO_HOUR,
        JIAQIAO_MINUTE,
    )
    .expect("unified enrichment must succeed");
    let helper_ctx = enriched
        .traditional_wellness
        .as_ref()
        .expect("enriched snapshot must carry the unified context");

    let core_json = serde_json::to_string(&core_ctx).expect("serialise core context");
    let helper_json = serde_json::to_string(helper_ctx).expect("serialise helper context");
    assert_eq!(
        core_json, helper_json,
        "core resolve and helper enrichment must yield byte-equal Traditional Wellness Contexts"
    );
}

/// 2. The enriched snapshot's `traditional_wellness` field carries
///    both primitive source ids (branch + seasonal) plus exactly one
///    composite. SOURCE-01 contract.
#[test]
fn enriched_unified_context_carries_two_primitive_envelopes_plus_exactly_one_composite() {
    let snapshot = snapshot_for_2026_08_16();
    let enriched = enrich_day_snapshot_with_traditional_wellness(
        &snapshot,
        snapshot.context.jd,
        VIETNAM_TIMEZONE,
        JIAQIAO_HOUR,
        JIAQIAO_MINUTE,
    )
    .expect("unified enrichment must succeed");
    let ctx = enriched
        .traditional_wellness
        .as_ref()
        .expect("must populate unified context");

    // Both primitives are present.
    let has_branch = ctx
        .evidence
        .iter()
        .any(|e| e.source_id == SOURCE_SHI_ER_JING_NA_DI_ZHI);
    let has_solar_term = ctx
        .evidence
        .iter()
        .any(|e| e.source_id == "amlich-solar-term-engine");
    let has_suwen = ctx
        .evidence
        .iter()
        .any(|e| e.source_id == SOURCE_HUANGDI_NEIJING_SUWEN);
    assert!(
        has_branch,
        "branch-channel primitive envelope must be present"
    );
    assert!(
        has_solar_term,
        "solar-term primitive envelope must be present"
    );
    assert!(has_suwen, "Suwen primitive envelope must be present");

    // Exactly one composite envelope, never two or zero.
    let composite_count = ctx
        .evidence
        .iter()
        .filter(|e| e.source_id == COMPOSITE_SEASONAL_WELLNESS)
        .count();
    assert_eq!(
        composite_count, 1,
        "exactly one seasonal composite envelope must be emitted"
    );

    // Branch and seasonal sides are populated.
    assert!(ctx.hour_branch.is_some(), "hour_branch must resolve");
    assert!(
        ctx.seasonal_cultivation.is_some(),
        "seasonal_cultivation must resolve"
    );
}

/// 3. Additive DTO discipline: enriched snapshot serializes with the
///    field; ordinary snapshot omits it byte-for-byte. Round-trip
///    stays byte-equal for both shapes.
#[test]
fn additive_dto_discipline_round_trip_byte_equal() {
    let snapshot = snapshot_for_2026_08_16();
    let ordinary_json = serde_json::to_string(&snapshot).expect("serialise ordinary");
    assert!(
        !ordinary_json.contains("traditional_wellness"),
        "ordinary snapshot must omit traditional_wellness; got {ordinary_json}"
    );

    let enriched = enrich_day_snapshot_with_traditional_wellness(
        &snapshot,
        snapshot.context.jd,
        VIETNAM_TIMEZONE,
        JIAQIAO_HOUR,
        JIAQIAO_MINUTE,
    )
    .expect("unified enrichment must succeed");
    let enriched_json1 = serde_json::to_string(&enriched).expect("serialise enriched");
    let parsed: amlich_core::DaySnapshot =
        serde_json::from_str(&enriched_json1).expect("deserialise enriched");
    let enriched_json2 = serde_json::to_string(&parsed).expect("re-serialise");
    assert_eq!(
        enriched_json1, enriched_json2,
        "enriched snapshot must round-trip byte-equal"
    );
    assert!(
        enriched_json1.contains("traditional_wellness"),
        "enriched snapshot must include traditional_wellness; got {enriched_json1}"
    );
    assert!(
        parsed.traditional_wellness.is_some(),
        "round-trip must retain traditional_wellness"
    );
}

/// 4. The desktop `ClassicalSurfaceDto.traditional_wellness` field
///    type carries the same field set as the core
///    `TraditionalWellnessContext`. The keyof contract test
///    (`classical-surface.types.test.ts`) enforces the matching keys
///    on the TypeScript side; this test enforces the byte-equal
///    serde lock on the Rust side.
#[test]
fn traditional_wellness_context_serde_preserves_field_set() {
    let snapshot = snapshot_for_2026_08_16();
    let ctx = resolve_traditional_wellness_context_unified(
        snapshot.context.jd,
        VIETNAM_TIMEZONE,
        JIAQIAO_HOUR,
        JIAQIAO_MINUTE,
    );
    let value = serde_json::to_value(&ctx).expect("serialise to value");
    let obj = value.as_object().expect("context must be a JSON object");
    for required in [
        "hour_branch",
        "seasonal_cultivation",
        "disclaimer",
        "review_state",
        "time_basis",
        "evidence",
    ] {
        assert!(
            obj.contains_key(required),
            "TraditionalWellnessContext must carry {required} for cross-surface parity"
        );
    }
}

/// 5. The desktop / TUI / API surfaces see the SAME bilingual
///    explanation text for a given resolved hour and date. The
///    branch-channel wording is byte-equal between the core context
///    and the corpus JSON (already locked by `branch_channel_integration
///    .rs::twelve_row_goldens_each_branch_resolves_to_expected_channel`);
///    the seasonal wording is locked by `seasonal_cultivation_integration
///    .rs::four_corpus_goldens_identity_wording_and_citation`. This
///    test makes the cross-surface parity explicit by re-asserting
///    the wording fields on the unified context shape.
#[test]
fn unified_context_wording_matches_corpus_for_resolved_hour_and_term() {
    let snapshot = snapshot_for_2026_08_16();
    let ctx = resolve_traditional_wellness_context_unified(
        snapshot.context.jd,
        VIETNAM_TIMEZONE,
        JIAQIAO_HOUR,
        JIAQIAO_MINUTE,
    );
    let hb = ctx.hour_branch.as_ref().expect("hour_branch must resolve");
    // 09:30 falls in the Tỵ window (09:00–11:00), so the wording must
    // use the Tỵ branch and the Suwen seasonal wording for the
    // resolved term (mid-august 2026 — the resolved season depends on
    // the Lập Thu transition which is determined by the engine).
    assert_eq!(hb.branch_index, 5);
    assert_eq!(hb.branch_vi, "Tỵ");
    assert!(hb.wording_vi.contains("gắn với"));
    assert!(hb.wording_en.contains("historically associated"));

    let seasonal = ctx
        .seasonal_cultivation
        .as_ref()
        .expect("seasonal must resolve");
    assert!(seasonal.profile.wording_vi.contains("văn bản cổ mô tả"));
    assert!(seasonal
        .profile
        .wording_en
        .contains("the classical text describes"));
}

/// 6. The additive `DaySnapshot.traditional_wellness` field does NOT
///    appear in any pre-existing report DTO other than via
///    `ClassicalSurfaceDto`. We assert the field-name absence on
///    `PersonalDayReportDto`'s JSON — the report path does not
///    surface the Traditional Wellness Context, and the desktop reads
///    it through `get_classical_surface` only. This is the API-side
///    half of the additive DTO discipline; the TypeScript-side half
///    lives in `classical-surface.types.test.ts`.
#[test]
fn personal_day_report_dto_does_not_carry_traditional_wellness() {
    use amlich_api::dto::PersonalDayChartDto;
    let chart = PersonalDayChartDto {
        input: amlich_api::dto::PersonalDayQueryDto {
            date: amlich_api::DateQuery {
                day: 16,
                month: 8,
                year: 2026,
                timezone: Some(7.0),
                ruleset_id: None,
                event_kind: None,
                enabled_pack_ids: vec![],
            },
            birth_year: None,
            birth_month: None,
            birth_day: None,
            gender: None,
        },
        tier: amlich_api::dto::BirthDataTierDto::Anonymous,
        solar: amlich_api::dto::SolarDto {
            day: 16,
            month: 8,
            year: 2026,
            day_of_week: 0,
            day_of_week_name: "Sun".to_string(),
            date_string: "2026-08-16".to_string(),
        },
        lunar: amlich_api::dto::LunarDto {
            day: 4,
            month: 7,
            year: 2026,
            is_leap_month: false,
            date_string: "2026-07-04".to_string(),
        },
        canchi: None,
        tiet_khi: None,
    };
    let json = serde_json::to_string(&chart).expect("serialise PersonalDayChartDto");
    assert!(
        !json.contains("traditional_wellness"),
        "PersonalDayChartDto must not carry traditional_wellness (it lives on ClassicalSurfaceDto); got {json}"
    );
}
