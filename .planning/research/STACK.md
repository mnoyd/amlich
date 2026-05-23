# Technology Stack — v1.5 Eastern Knowledge Expansion

**Project:** amlich-core v1.5 (P1 Văn khấn cổ truyền + P4 Phi Tinh thời gian)
**Researched:** 2026-05-23
**Scope:** Stack delta for the two NEW pillars only. Existing workspace deps and patterns from v1.0–v1.4 are assumed.
**Overall confidence:** HIGH (no new crates required; recommendation is to reuse the established pattern verbatim)

---

## TL;DR — Recommended Stack

**No new crate dependencies are needed for v1.5.** The existing trio (`serde` 1.0, `serde_json` 1.0, `chrono` 0.4) plus the in-tree `include_str!` + `OnceLock` pattern already used by `almanac/golden_loader.rs` covers both P1 (ritual JSON corpus) and P4 (Flying Stars time-based tables) completely.

Adding any of the candidates considered (schema validators, markdown parsers, embed/asset helpers, `once_cell`, `phf`, `lazy_static`) would violate the project's "deterministic and library-flat" posture without delivering load-bearing value at v1.5 scope.

---

## Recommended Stack

### Core (already in `crates/amlich-core/Cargo.toml`)

| Technology   | Version (workspace pin) | Purpose for v1.5                                                    | Why |
|--------------|-------------------------|---------------------------------------------------------------------|-----|
| `serde`      | `1.0` (derive)          | Derive `Serialize`/`Deserialize` for `Ritual`, `RitualCorpus`, `FlyingStarChart`, `PalaceLayout` structs | Already the project-wide serialization contract; matches `GoldenDataset`, `GoldenEntry` patterns. |
| `serde_json` | `1.0`                   | Parse `data/rituals/*.json` and `data/almanac/flying_stars.json` at first-call | Exact pattern already used by `golden_loader::load_golden_dataset` (`serde_json::from_str(GOLDEN_JSON)`). Keeps loader code mechanically identical across modules — important for reviewability. |
| `chrono`     | `0.4`                   | `NaiveDate` inputs for ritual event lookups and Vận/Năm/Tháng resolution for Flying Stars | Already used across `lunar.rs`, `julian.rs`. Deterministic — `Utc::now()` is forbidden by project policy and remains forbidden in v1.5. |

### Standard Library (no Cargo entry; load-bearing for both pillars)

| Facility | Purpose for v1.5 | Why |
|---|---|---|
| `include_str!` | Embed `rituals/*.json` and `flying_stars.json` into the compiled binary at build time | Matches `golden_loader.rs:5` exactly (`const GOLDEN_JSON: &str = include_str!("../../data/almanac/khcbppt-golden.json");`). Zero-runtime-IO, zero filesystem dependency — critical for WASM target (`crates/amlich-wasm`) and deterministic tests. |
| `std::sync::OnceLock` | One-time parse + validate cache for corpus and Flying Stars tables | Matches `golden_loader.rs:6` exactly. Stable since Rust 1.70 (April 2023); no need for `once_cell` or `lazy_static`. |
| `std::collections::HashMap` / `BTreeMap` | Index rituals by `event_type` / `lunar_date`; index flying-star tables by `(vận, năm)` / `(vận, năm, tháng)` | Already used throughout `almanac/`. For small finite domains (24 Tiết khí, 9 Vận, 12 tháng, ~50–200 rituals), `BTreeMap` gives deterministic iteration order — preferred for any output that touches golden tests. |

### Data Layout (new files, no new dependencies)

| Path | Format | Purpose |
|------|--------|---------|
| `crates/amlich-core/data/rituals/<event>.json` (or single `rituals.json`) | JSON | Văn khấn corpus entries, each carrying `event_type`, `season`, `lunar_date`, `source`, `body` fields, plus `source_id: "vn-folk-ritual"`. |
| `crates/amlich-core/data/almanac/flying_stars.json` | JSON | Phi Tinh tables: Vận 1–9 base palace charts, year-star offsets, month-star offsets. Bounded, fully enumerable (≤ a few KB). |

Both files must be reachable via `include_str!` and listed under the package `include` array (already covers `"data/**"` in current `Cargo.toml:13`).

---

## Alternatives Considered (and rejected)

| Capability | Recommended | Alternative Considered | Why Not (for v1.5) |
|---|---|---|---|
| JSON schema validation | Hand-rolled `validate_*` functions in module (mirrors `golden_loader::validate_golden_dataset`, lines 153–237) | `jsonschema` crate | Adds a transitive dep tree (`fancy-regex`, `url`, `ahash`). The golden dataset's hand-rolled validator catches richer invariants (e.g., "must cover all 24 Tiết khí", "must cover all 9 Vận") than a JSON Schema document can express tersely. Project pattern is to assert invariants in Rust, not JSON Schema. |
| Markdown rendering for Văn khấn prose | Store Vietnamese prayer text as plain UTF-8 strings (with `\n` separators) inside the JSON `body` field; render at the UI layer (`amlich-tui`, `apps/desktop`) if needed | `pulldown-cmark`, `comrak` | Văn khấn texts are line-oriented prayer scripts, not Markdown documents. They have no headings, links, or code spans — only line breaks. Rendering belongs at the UI boundary, not in `amlich-core`. `amlich-core` must remain a calculation/lookup library; introducing a Markdown parser would leak presentation concerns into the kernel and violate the WASM-friendly posture. |
| Asset embedding helper | `include_str!` (std macro) | `rust-embed`, `include_dir` | A single macro per file is fine for the small, finite corpus envisioned (P1: tens to low hundreds of prayers; P4: ≤ ~30 numeric tables). `rust-embed` shines for hundreds of files of varying types — we have neither volume nor heterogeneity. |
| Static lookup tables | `OnceLock<HashMap<…>>` or `BTreeMap` constructed from parsed JSON | `phf` (perfect hash function) crate | `phf` is a compile-time optimization; the JSON corpus is parsed once on first call and cached. Lookup cost is irrelevant compared to library-flatness. Also: `phf` requires `phf_codegen` in `build.rs`, which we have deliberately avoided. |
| Lazy statics | `std::sync::OnceLock` (stable since Rust 1.70) | `once_cell`, `lazy_static` | `OnceLock` is std; no extra dep. Already adopted in `golden_loader.rs`. |
| Pre-compiled binary tables for Flying Stars | JSON parsed via `serde_json` | `bincode`, `postcard`, `rkyv` | Flying-star tables are tiny (Vận 8 + Vận 9 base charts = 2 × 9 cells; year/month offsets are simple modular formulas). Human-readable JSON is invaluable for KHCBPPT-style cross-check audits; binary formats hide a content corpus that classical-text reviewers must be able to read. |
| Internationalization / Vietnamese diacritics handling | UTF-8 strings + serde defaults | `unicode-normalization`, `icu` | Văn khấn text is stored verbatim. Normalization (NFC vs NFD) should be enforced at corpus-author time via a fixture-builder script (out of crate scope), not at parse time. |
| Async file IO for corpus loading | None — synchronous `include_str!` at build time | `tokio::fs` | Project policy: no async runtime in `amlich-core`. WASM and TUI consumers expect a sync API surface. |
| Float math for any angular/period calculation | Integer modular arithmetic (Vận = `((year - 1864) / 20) + 1` style; month offsets via mod-9 wheel) | `num` / `bigdecimal` | Project policy: no floating point. All Flying Stars period math is integer-pure. |

---

## Installation

**No `cargo add` invocations are required.** The existing `crates/amlich-core/Cargo.toml` already declares the three workspace deps (lines 17–19):

```toml
[dependencies]
serde = { workspace = true }
serde_json = { workspace = true }
chrono = { workspace = true }
```

Confirm the package `include` array still picks up the new data directories (it does — `"data/**"` on line 13 already covers `data/rituals/` and any new files under `data/almanac/`).

---

## Integration Points with Existing Modules

| New module | Mirrors pattern from | Touchpoints |
|---|---|---|
| `crates/amlich-core/src/rituals/mod.rs` + `loader.rs` | `almanac/golden_loader.rs` (line-for-line: `const X: &str = include_str!(…); static X: OnceLock<…>; load_x() -> &'static X`) | Read `DaySnapshot.event_type` (existing); emit lookup results tagged `source_id: "vn-folk-ritual"` via `semantic_graph::provenance::Provenance::almanac_rule` (existing constructor). |
| `crates/amlich-core/src/almanac/fengshui/mod.rs` + `flying_stars.rs` | `almanac/cuu_dieu.rs` and `almanac/thai_tue.rs` (Vận/year-based tables, integer arithmetic) | Pure time inputs (`year: i32`, `lunar_month: i32`); returns `FlyingStarChart { period: u8, year_star: u8, month_star: u8, base_palaces: [[u8; 3]; 3] }`. Source-tag every emission with `source_id: "huyen-khong"`. Do NOT touch `sat_phuong.rs`, `than_huong.rs`, `thai_tue.rs` (those stay on `khcbppt`). |
| `interaction/` matrices | n/a for v1.5 | Phi Tinh tables remain Tier 0; no spatial composition until P5. Ritual lookup feeds the Domain-Day Boost narrative layer only if downstream chooses to wire it — not required by v1.5 scope. |
| `semantic_graph/provenance.rs` | Existing source-id discipline | Register two new `source_id` constants if a registry exists; otherwise inline-literal them at emission sites and add cases to whatever exhaustive match exists today. DEC-0015/0016 forbids ad-hoc `source_id` creation outside this registry. |

---

## Explicit Non-Goals — Do NOT Add to v1.5

The following are explicitly out of scope for the v1.5 stack to prevent bloat and preserve the deterministic / library-flat invariants:

- **No async runtime** (`tokio`, `async-std`, `smol`). `amlich-core` is sync; WASM target requires it.
- **No floating-point math libraries** (`num-traits`, `bigdecimal`, `rust_decimal`). All Flying Stars math is integer-modular.
- **No web/HTTP framework** (`axum`, `actix`, `reqwest`). Corpus is embedded; no fetch.
- **No filesystem IO at runtime** (`std::fs::read_to_string`, `walkdir`). Use `include_str!`.
- **No Markdown / templating engine** (`pulldown-cmark`, `tera`, `handlebars`). Văn khấn text is plain UTF-8.
- **No schema-validation crate** (`jsonschema`, `valico`). Hand-rolled validators match `golden_loader.rs` precedent.
- **No build-script codegen** (`build.rs`, `phf_codegen`, `prost-build`). Keep build flat.
- **No `Utc::now()` or any wall-clock read** anywhere in v1.5 code paths. All inputs flow from `BirthInput`, `DaySnapshot`, or explicit caller-supplied `NaiveDate`.
- **No spatial / `Direction24` types yet** — Tier 3 model is deferred to P5 per EXPANSION_FRAMEWORK §3.3.
- **No Tử Vi (P6), Kinh Dịch (P2), Y học (P3)** — out of scope per PROJECT.md current milestone.

---

## Confidence Notes

| Decision | Confidence | Basis |
|---|---|---|
| `serde` / `serde_json` / `chrono` sufficient | HIGH | Direct inspection of `golden_loader.rs` confirms the same input shape (date-keyed JSON corpus + structured records + invariant validation) works end-to-end today across all v1.0–v1.4 subsystems. |
| `include_str!` + `OnceLock` is the right embedding pattern | HIGH | Confirmed in-repo (`almanac/golden_loader.rs:5–6, 14–21`). Stable since Rust 1.70 (April 2023). |
| No Markdown renderer needed in core | HIGH | Văn khấn entries are line-broken prayer scripts; presentation belongs at UI boundary (`amlich-tui`, `apps/desktop`). |
| No schema validator needed | HIGH | Hand-rolled validators already exceed schema-doc expressivity (see `validate_coverage`, lines 188–237 of `golden_loader.rs`). |
| Flying Stars period math fits in integer modular arithmetic | MEDIUM | Classical formulas for Vận (20-year periods), year-star (descending mod-9), and month-star (mod-9 with seasonal offset) are integer-pure in every reference reviewed. To be re-confirmed against *Thẩm Thị Huyền Không Học* during the architecture pass — but does NOT change the stack recommendation either way. |
| Workspace dep versions (`serde 1.0`, `serde_json 1.0`, `chrono 0.4`) remain current as of 2026-05-23 | MEDIUM | These are major-version-stable release lines maintained for years; the workspace already pins them. Network verification was not available during this research pass, but the pins are consistent with the pre-v1.5 milestone deliveries and unchanged risk profile. |

---

## Sources

- `crates/amlich-core/Cargo.toml` (lines 16–19) — current deps
- `crates/amlich-core/src/almanac/golden_loader.rs` (lines 1–50, 153–237) — embedding + validation pattern to mirror
- `Cargo.toml` workspace root (lines 20–24) — pinned versions for `serde`, `serde_json`, `chrono`
- `.planning/research/EXPANSION_FRAMEWORK.md` §2.3 (Phi Tinh), §2.4 (Văn khấn), §3.1 (provenance), §5 (P1/P4 sequencing)
- `.planning/PROJECT.md` (v1.5 milestone scope, DEC-0015/0016 source-id discipline)
- Rust stdlib `std::sync::OnceLock` — stabilized in Rust 1.70 (April 2023)
