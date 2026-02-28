# Stack Research

**Domain:** Vietnamese almanac correctness validation against KHCBPPT
**Researched:** 2026-02-28
**Confidence:** HIGH (codebase-derived), MEDIUM (external tooling)

## Core Stack (No New Dependencies)

| Tool | Version | Purpose | Confidence |
|------|---------|---------|------------|
| Rust `#[test]` + `cargo test` | 1.93.0 | Test harness for golden dataset validation | HIGH |
| `serde_json` + `serde` | 1.0 | JSON golden dataset deserialization | HIGH |
| `include_str!` macro | std | Embed golden dataset at compile time | HIGH |

**Rationale:** The existing workspace already has everything needed. Adding dependencies would be unjustified complexity.

## Golden Dataset Format

**Format:** JSON (matches existing `baseline.json` and `day-info-golden.json` patterns)

**Schema:**
```json
{
  "schema_version": "khcbppt-golden/v1",
  "source_edition": "KHCBPPT [specific edition details]",
  "entries": [{
    "solar_date": { "day": 10, "month": 2, "year": 2024 },
    "lunar_date": { "day": 1, "month": 1, "year": 2024 },
    "canchi_day": "Giáp Thìn",
    "truc": { "name": "Mãn", "quality": "hung" },
    "day_deity": { "name": "Thiên Hình", "classification": "hac_dao" },
    "taboos": ["tam_nuong"],
    "xung_hop": { "luc_xung": "Tuất" },
    "than_huong": { "xuat_hanh_huong": "Đông Nam" },
    "na_am": "Hải Trung Kim",
    "stars": { "cat_tinh": ["Thiên Quý"], "sat_tinh": ["Phúc Sinh"] },
    "day_star_28": { "index": 5, "name": "Vĩ", "quality": "cat" },
    "khcbppt_ref": "vol. X, p. YY"
  }]
}
```

**File location:** `crates/amlich-core/data/almanac/khcbppt-golden.json`

**Test file:** `crates/amlich-core/tests/khcbppt_validation.rs`

## Testing Approach

- One parametric test that loads golden JSON and iterates all entries
- Each subsystem compared field-by-field against golden entry
- Collect-all failure reporting (not early exit) — shows full divergence scope
- Follows existing pattern from `almanac_golden.rs`

## What NOT to Use

| Tool | Why Avoid |
|------|-----------|
| Python scripts for data extraction | Adds language boundary; ground truth must stay in one place |
| Automated OCR on classical text | Error rate on classical Vietnamese/Chinese is too high |
| External almanac apps as reference | Not authoritative; may share the same errors |
| `proptest` / property testing | Generates random inputs; doesn't validate KHCBPPT table values |
| CSV format | Unicode handling and structured-field limitations |

## Source Authority Note

Not all subsystems cite KHCBPPT as source:
- `na_am_meta.source_id` = "tam-menh-thong-hoi" (not KHCBPPT)
- `star_meta.source_id` = "nhi-thap-bat-tu"

Golden dataset schema must accommodate multi-source validation.

---
*Stack research: 2026-02-28*
