# Deferred Items (Out of Scope)

1. `cargo test --package amlich-core --lib` fails in pre-existing `tietkhi` tests:
   - `tietkhi::tests::test_after_returns_positive`
   - `tietkhi::tests::test_before_returns_negative`
   - `tietkhi::tests::test_equidistant_prefers_after`
   - `tietkhi::tests::test_exact_match_returns_zero`
   - `tietkhi::tests::test_term_positions_in_year`

These failures are unrelated to Ten Gods changes in `almanac::thap_than` and were not modified in this plan.
