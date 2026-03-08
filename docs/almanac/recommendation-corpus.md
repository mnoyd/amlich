# Recommendation Validation Corpus (v1)

Canonical fixture: `crates/amlich-core/data/almanac/recommendation-corpus-v1.json`.

## Scenarios

- `strong_favorable` (2024-05-13)
- `strong_avoid` (2024-01-03)
- `conflicting_layered` (2024-02-14)
- `sparse_relative` (2024-05-22)
- `edge_case_leap_day` (2024-02-29)
- `hard_stop_dense` (2024-12-22)

Each case includes:

- rationale
- expected bucket profile (`nen`, `co_the`, `tranh`, `ky_manh`)
- summary substring guard
- required activity IDs

## Enforcement

- Core regression: `crates/amlich-core/tests/recommendation_corpus.rs`
- API parity regression: `crates/amlich-api/tests/recommendation_corpus_parity.rs`
