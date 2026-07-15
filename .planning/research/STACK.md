# Technology Stack — v1.7 Kinh Dịch (I-Ching) + Thái Tuế Cross-link

**Project:** amlich-core v1.7 (P2 Kinh Dịch pillar — Mai Hoa Dịch Số casting + 64-hexagram lookup + biến quẻ; Thái Tuế/Tam Sát ⇄ Phi Tinh reasoning cross-link)
**Researched:** 2026-07-16
**Scope:** Stack delta for the NEW v1.7 features only. Existing validated capabilities (v1.0–v1.6: KHCBPPT core, Ten Gods/Kua/Dai Van, Hour pillar/60-cycle/Na Am, Văn khấn corpus, Phi Tinh overlays, semantic graph, ADRs 0001–0004) are assumed shipped and out of scope.
**Overall confidence:** HIGH — no new crate dependencies required; the v1.5 "no new deps" precedent holds verbatim.

---

## TL;DR — Recommended Stack

**No new crate dependencies are needed for v1.7.** The existing trio (`serde` 1.0, `serde_json` 1.0, `chrono` 0.4) plus `unicode-normalization` 0.1.25 plus the in-tree `include_str!` + `std::sync::OnceLock` + `serde_json::from_str` pattern (already proven by `rituals/corpus.rs`, `almanac/fengshui/period.rs`, `almanac/golden_loader.rs`) covers all four new v1.7 surfaces completely:

1. **Mai Hoa Dịch Số casting** — pure integer arithmetic on `chrono::Datelike` year/month/day/hour (mod 8 → upper trigram; running-sum mod 8 → lower trigram; total-sum mod 6 → động hào). No parser, no RNG, no upstream algorithm library.
2. **64-hexagram lookup table** — ≤64 entries of Vietnamese thoán từ / hào từ from Ngô Tất Tố's *Kinh Dịch Trọn Bộ*; serialized as `data/kinh_dich/hexagrams-v1.json` and embedded via one `include_str!` constant per file (mirrors `rituals/corpus.rs:27-56` exactly).
3. **Biến quẽ (transforming hexagram)** — flip the bit at the động hào index of the 6-line primary hexagram. One `^` op on a `u8`; no deps.
4. **Thái Tuế / Tam Sát ⇄ Phi Tinh cross-link** — pure read-only join inside `reasoning/personal.rs`. `compute_thai_tue` (`almanac/thai_tue.rs:53`) and `get_sat_phuong` (`almanac/sat_phuong.rs:49`) already exist; the FlyingStar palace layout is already produced by `almanac/fengshui/`. The cross-link emits **two** `ReasoningEvidenceEnvelope` rows (one `source_id: khcbppt`, one `source_id: huyen-khong`) inside the existing envelope — CRIT-3 isolation is preserved because nothing is wired into `interaction/direction_merge.rs`.

Any candidate library considered (`xalen-iching`, `i-ching`, `iching`, schema validators, RNG crates) is rejected — see **What NOT to Use**. The project's source-provenance discipline (DEC-0015/0016, ADR-0001, the `tests/source_id_guard.rs` CI grep) requires the *actual Vietnamese text* from *named Vietnamese books* (Ngô Tất Tố / Thiệu Khang Tiết), which no upstream Rust crate carries.

---

## Recommended Stack

### Core (already in `crates/amlich-core/Cargo.toml` — unchanged)

| Technology | Version | Purpose for v1.7 | Why |
|------------|---------|------------------|-----|
| `serde` | `1.0` (workspace pin, `features = ["derive"]`) | Derive `Serialize`/`Deserialize` for `Hexagram`, `Trigram`, `HexagramLine`, `MaiHoaCast`, `IChingReading`, and the `HexagramFile` loader struct | Already the project-wide serialization contract. Mirrors `RitualFile` (`rituals/corpus.rs:78-83`) shape exactly: one outer struct with `$schema_version: String` + `entries: Vec<…>`. |
| `serde_json` | `1.0` (workspace pin) | Parse the embedded 64-hexagram corpus at first call via `serde_json::from_str(json)` | Exact pattern proven by `rituals/corpus.rs:99-104` and `almanac/golden_loader.rs`. Same `unwrap_or_else(|e| panic!(…))` policy — corpus is compile-embedded so a parse failure is a build-time bug, not a runtime condition. |
| `chrono` | `0.4` (workspace pin) | `chrono::Datelike` trait methods (`year()`, `month()`, `day()`) to feed the Mai Hoa time-number algorithm; `NaiveDateTime` for the consultation instant | Already used in `advisory.rs:1` (`use chrono::Datelike;`) and across `lunar.rs`, `julian.rs`. **Forbids `Utc::now()`** — project policy unchanged; the consultation time is supplied by the caller via `DaySnapshot.context`. |
| `unicode-normalization` | `0.1.25` (already direct dep, non-workspace) | NFC-normalize every Vietnamese text field in the 64-hexagram corpus (thoán từ, hào từ, cát/hùng labels) at first load | RIT-08 precedent — Vietnamese text in corpus files is not trustable to be pre-NFC; `is_nfc` guard + `UnicodeNormalization::nfc` re-write at load time. Mirrors `rituals/corpus.rs:18,119+`. |

### Standard Library (no Cargo entry; load-bearing for both pillars)

| Facility | Purpose for v1.7 | Why |
|----------|------------------|-----|
| `std::sync::OnceLock<T>` | Lazy parse-and-cache of the 64-hexagram corpus (one global `OnceLock<Vec<Hexagram>>`); same trick for `TRIGRAMS` if split into its own file | Project precedent (`rituals/corpus.rs:85`, `almanac/data.rs`, `almanac/fengshui/period.rs`). WASM-compatible, no `lazy_static` / `once_cell` needed (the latter is in std prelude since 1.70 and the workspace already uses it directly). |
| `include_str!("../../data/kinh_dich/*.json")` | Compile-time corpus embedding — zero runtime file IO, zero asset-management machinery | Project precedent (`rituals/corpus.rs:27-56`). Critical for the `amlich-wasm` target: no `fs::read`, no path resolution, the binary stays self-contained. |
| `u8` bit math (XOR / shift / AND) | Biến quẻ: flip the động hào bit on the 6-bit line pattern; trigram indices via integer division/modulo | The I-Ching line model is a 6-bit binary number by definition. No bitvec/arrayvec needed for 6 bits — `[bool; 6]` or `u8` is the obvious encoding. |
| `chrono::Datelike` integer arithmetic | Mai Hoa time-number: `(year_n + month_n + day_n) % 8` → upper trigram; `(year_n + month_n + day_n + hour_n) % 8` → lower trigram; `(sum) % 6` → động hào (1-indexed). Hour maps via existing `hour_pillar.rs` 12-earthly-branch slot. | Standard classical algorithm (Thiệu Khang Tiết, *Mai Hoa Dịch Số*). The mapping from Gregorian year/month/day/hour to the numerical inputs uses the lunar-vs-solar choice the framework specifies (lunar for Mai Hoa tradition). The `chrono` types already in the codebase suffice; no extra calendar math needed because lunar conversion is already in `lunar.rs`. |

### Development Tools (already in the workspace — unchanged)

| Tool | Purpose | Notes |
|------|---------|-------|
| `tests/source_id_guard.rs` (existing CI grep) | Enforces that the two new constants `SOURCE_KINH_DICH` (`"kinh-dich"`) and `SOURCE_MAI_HOA_DICH_SO` (`"mai-hoa-dich-so"`) are added to `crates/amlich-core/src/sources.rs` and used by name at every call-site; bare literals in `src/` outside `sources.rs` continue to fail CI | Already extended in v1.5 (`SOURCE_VN_FOLK_RITUAL`, `SOURCE_HUYEN_KHONG`). The test row `all_constants_have_expected_values` (`sources.rs:48-56`) needs two new `assert_eq!` lines. |
| `bd` (beads) issue tracker | Track v1.7 implementation work — schema-lock phase, corpus authoring phase, evaluator wiring phase | Per `AGENTS.md`. The `data/kinh_dich/hexagrams-v1.json` corpus (60+ entries with hand-transcribed Vietnamese text) is the long-pole task and should be filed as its own epic. |
| Golden-test harness pattern (`almanac/fengshui/golden.rs`, `rituals/corpus.rs` tests) | Cross-check casting results against §7 references (nhantu.net for Mai Hoa; divination.com / printed Ngô Tất Tố for hexagram texts). ≥10 cases × ≥2 independent sources; divergences logged as `KnownDivergence`, never silently reconciled. | Per `EXPANSION_FRAMEWORK.md §6` + `§7`. Project policy — non-negotiable. |

---

## Integration Points (concrete, codebase-verified)

### 1. New module: `crates/amlich-core/src/kinh_dich/`

Sibling to `rituals/` (not nested under `reasoning/`) — matches the v1.5 precedent where the corpus layer (`rituals/`) is decoupled from the reasoning layer (`reasoning/personal.rs`). Suggested file layout:

```
crates/amlich-core/src/kinh_dich/
├── mod.rs              // pub re-exports
├── schema.rs           // Hexagram, Trigram, HexagramLine, MaiHoaCast, IChingReading + deny_unknown_fields
├── corpus.rs           // include_str! + OnceLock<Vec<Hexagram>> loader (mirrors rituals/corpus.rs)
├── mai_hoa.rs          // cast_mai_hoa(year, month, day, hour_chi_index) -> MaiHoaCast
└── bien_qua.rs         // transform(hexagram, dong_hao_index) -> Hexagram
```

> Note: `EXPANSION_FRAMEWORK.md §2.2` mentions `reasoning/iching/` as the module path, but the v1.5–v1.6 precedent (`rituals/` sibling, not `reasoning/rituals/`) is stronger. Pick the sibling layout for corpus cohesion; the `reasoning/personal.rs` evaluator branch is where the "reasoning" actually happens.

### 2. New data files: `crates/amlich-core/data/kinh_dich/`

```
data/kinh_dich/
├── hexagrams-v1.json   // 64 entries: {number, chinese_char, vietnamese_name, thuan_tu, hao_tu[6], cat_hung, source_id}
├── trigrams-v1.json    // 8 entries: {index, vietnamese_name, element, attribute}
└── manifest.json       // documentation-only; NOT parsed at runtime (per rituals/ pattern)
```

Both JSON files MUST start with `"$schema_version": "kinh-dich-v1"` and be checked by an ADR-0001-style version assertion in the loader (mirror `rituals/corpus.rs:105-109`). **Schema MUST be locked (ADR) before corpus authoring begins** — this is exactly the v1.5 PITFALL CRIT-1/5 lesson ("re-editing 60 corpus entries after a schema slip is prohibitively expensive"). For 64 hexagrams × 6 hào từ each = 384 line texts + 64 judgments, a schema slip is even more expensive than v1.5's 60 rituals.

### 3. New source constants: `crates/amlich-core/src/sources.rs`

```rust
/// Kinh Dịch — 64 hexagram texts and judgments (Ngô Tất Tố, *Kinh Dịch Trọn Bộ*).
pub const SOURCE_KINH_DICH: &str = "kinh-dich";

/// Mai Hoa Dịch Số — time-number hexagram casting (Thiệu Khang Tiết).
pub const SOURCE_MAI_HOA_DICH_SO: &str = "mai-hoa-dich-so";
```

Plus two new `assert_eq!` rows in `all_constants_have_expected_values` (`sources.rs:48-56`). The CI guard picks them up automatically — no test-harness change needed beyond the constants existing.

### 4. `ConsultationIntent` extension (in `advisory.rs:18-30`)

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]  // NOTE: drop `Copy`
#[serde(rename_all = "snake_case")]
pub enum ConsultationIntent {
    // ...existing 9 variants...
    IChing { question: Option<String> },   // EXPANSION_FRAMEWORK §2.2
}
```

⚠️ **API surface flag (not a dep, but a stack-adjacent breakage):** the current `ConsultationIntent` is `#[derive(Copy)]`. Adding `IChing { question: Option<String> }` (per framework §2.2) forces removal of `Copy` from the enum, which ripples through every call-site in `advisory.rs` (43 matches found), `reasoning/personal.rs`, and `PersonalReasoningInput::from_birth`. Two options for the v1.7 roadmap:

- **(A) Preferred:** Drop `Copy` from `ConsultationIntent`; add `.clone()` at the handful of re-use sites. Matches the "additive-only integration changes" decision in PROJECT.md and keeps the framework's `{ question }` shape.
- **(B) Avoid:** Keep `Copy` by introducing a sibling struct `IChingQuery { intent: ConsultationIntent::IChing, question: Option<String> }` passed alongside `PersonalReasoningInput`. Diverges from the framework's stated shape.

Recommend (A); file a bd issue to enumerate the call-site churn before implementation.

### 5. `reasoning/personal.rs` evaluator branch

Add a new arm in `build_fact_nodes` that fires when `self.intent == ConsultationIntent::IChing { .. }`:

```rust
// Pseudo-code following the established pattern at personal.rs:38-104
if matches!(self.intent, ConsultationIntent::IChing { .. }) {
    let cast = kinh_dich::mai_hoa::cast_from_snapshot(snapshot);
    let primary = kinh_dich::corpus::hexagram(cast.primary_index);
    let bien = kinh_dich::bien_qua::transform(primary, cast.dong_hao);
    nodes.push(PersonalFactNode {
        id: "fact.personal.iching_reading".to_string(),
        summary_vi: format!("Quẻ {} → biến quẻ {} ({})", primary.name, bien.name, primary.cat_hung),
        severity: Some(primary.cat_hung.clone()),
        evidence: vec![
            ReasoningEvidenceEnvelope {
                source_family: ReasoningEvidenceSourceFamily::IChing,  // NEW enum variant in ReasoningEvidenceSourceFamily
                source_id: SOURCE_MAI_HOA_DICH_SO.to_string(),
                method: "mai_hoa_time_number".to_string(),
                note: Some(format!("upper={} lower={} dong_hao={}", cast.upper, cast.lower, cast.dong_hao)),
            },
            ReasoningEvidenceEnvelope {
                source_family: ReasoningEvidenceSourceFamily::IChing,
                source_id: SOURCE_KINH_DICH.to_string(),
                method: "hexagram_lookup".to_string(),
                note: Some(format!("primary={} bien={}", primary.number, bien.number)),
            },
        ],
    });
}
```

`ReasoningEvidenceSourceFamily` needs a new `IChing` variant (currently has `Interaction`, `Bazi`, and likely others — verify before implementing). This is the only non-additive structural change required.

### 6. Thái Tuế / Tam Sát ⇄ Phi Tinh cross-link (CRIT-3 isolation preserved)

The cross-link is **read-only and lives entirely in `reasoning/personal.rs`** (or a new helper `reasoning/directional_composite.rs`). It composes three existing producers:

- `compute_thai_tue(birth_chi_index, year_chi_index)` → `ThaiTueResult` (`almanac/thai_tue.rs:53`) — already exported from `lib.rs:45`
- `get_sat_phuong(chi_index)` → `SatPhuongResult` (`almanac/sat_phuong.rs:49`) — already exported from `lib.rs:43`
- `snapshot.day_fortune.flying_star` (or whichever surface holds the palace layout from `almanac/fengshui/`)

The composite emits a single `PersonalFactNode` carrying **two evidence envelopes** — one with `source_id: SOURCE_KHCBPPT` (for Thái Tuế / Tam Sát), one with `source_id: SOURCE_HUYEN_KHONG` (for the Phi Tinh palace layout). CRIT-3 isolation is preserved because:

- ✅ `FlyingStar` is still NOT wired into `interaction/direction_merge.rs` (the v1.5 grep-verified boundary stands).
- ✅ The two source_ids remain distinct; the cross-link only co-locates them inside the *reasoning envelope*, which is the only place DEC-0015/0016 permits source-mixing.
- ✅ No new compute happens — the cross-link consumes already-computed `ThaiTueResult`, `SatPhuongResult`, and palace-layout values.

A `rule.composite.directional_picture` synthetic source_id (per framework §3.2 last paragraph) should be used if the node itself needs a single owner source_id, with the two raw envelopes preserved as evidence rows.

---

## Alternatives Considered

| Recommended | Alternative | When Alternative Wins | Verdict Here |
|-------------|-------------|------------------------|--------------|
| Hand-coded Mai Hoa casting + JSON corpus (this proposal) | `xalen-iching` 0.6.0 (crates.io) | Only if (a) English Legge translations were acceptable, (b) Chinese `乾`/`坤` characters were the target text, (c) source-provenance discipline did not apply | ❌ **Reject.** Inspection of `xalen-iching` 0.6.0 `src/lib.rs:293-352` confirms it ships English Legge translations ("The Creative", "The Receptive") with Chinese characters and Wade-Guess romanization. The project requires Vietnamese thoán từ / hào từ from Ngô Tất Tố — a source the crate does not carry and cannot carry without violating its own upstream text choices. Pulling it in would also inject an unverifiable `source_id` into a codebase policed by `tests/source_id_guard.rs`. |
| Hand-coded Mai Hoa casting | `i-ching` 1.0.0 (crates.io) | Same conditions as above, plus it is a CLI app with a Goose extension — wrong shape entirely | ❌ **Reject.** Description "I Ching divination readings for CLI and Goose extension" — it's a binary, not a library-first dependency. Wrong text source, wrong shape. |
| Hand-coded Mai Hoa casting | `iching` 0.5.0 (crates.io) | — | ❌ **Reject.** Marked as an app (`"An app for divination with the I-Ching"`), not a library. |
| `include_str!` + `OnceLock` (this proposal) | `phf` (perfect hash maps) for O(1) hexagram-by-number lookup | If the corpus grew to thousands of entries AND lookup-by-string-key dominated access patterns | ❌ **Not worth it.** 64 hexagrams indexed by `number: u8` is a `Vec<Hexagram>` with direct indexing — `hexagrams[number - 1]` is already O(1) and trivially cache-friendly. `phf` adds a proc-macro dep for zero measurable win. |
| `include_str!` (this proposal) | `rust-embed` / `include_dir` | If the corpus were hundreds of files or needed path-based lazy loading | ❌ **Not worth it.** Two JSON files (`hexagrams-v1.json`, `trigrams-v1.json`) — exactly the v1.5 rituals pattern (13 files, one `include_str!` each). Adding an embed crate would break the "deterministic and library-flat" posture without earning its keep. |
| Plain `serde_json::from_str` (this proposal) | `serde_json` + `jsonschema` validation crate | If runtime schema drift were a risk | ❌ **Not worth it.** Schema discipline is enforced at *load* time via the ADR-0001-style `assert_eq!(file.schema_version, EXPECTED_SCHEMA_VERSION)` pattern (`rituals/corpus.rs:105-109`) plus `#[serde(deny_unknown_fields)]` on the loader struct. A `jsonschema` validator would add ~12 transitive crates and a runtime cost for an invariant the build already enforces. |
| `u8` bit math for biến quẻ (this proposal) | `bitvec` crate | If we needed word-level bitwise ops across >64 bits | ❌ **Not worth it.** A hexagram is 6 bits. `bitvec` is overkill by orders of magnitude. |
| `chrono::Datelike` + manual lunar conversion (this proposal) | `lunar_rust` 1.0.1 (crates.io) | — | ❌ **Reject.** The crate description is in Chinese and integrates Chinese-calendar + 节气 + 彭祖百忌 logic — it would conflict with the project's KHCBPPT-aligned core calendar (v1.0–v1.1) and re-introduce a parallel computation path that bypasses the audit trail. The project's `lunar.rs` already does correct Vietnamese lunar conversion; reusing it preserves the source-of-truth invariant. |
| `std::sync::OnceLock` (this proposal) | `once_cell` crate | If MSRV were < 1.70 | ❌ **Already settled in v1.5.** `OnceLock` is in std since Rust 1.70 (June 2023); workspace `Cargo.toml` does not pin MSRV below that and the existing `rituals/corpus.rs:17` already uses `std::sync::OnceLock`. |

---

## What NOT to Use

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| Any upstream Rust I-Ching library (`xalen-iching`, `i-ching`, `iching`) | (1) **Provenance violation** — DEC-0015/0016, ADR-0001, and the `tests/source_id_guard.rs` discipline require every text to trace to a named source book. None of these crates carry Vietnamese text from Ngô Tất Tố or Thiệu Khang Tiết; they ship English (Legge) or Chinese (Wilhelm-derived) translations. (2) **Golden-test discipline** — framework §7 mandates cross-checking against `nhantu.net` (Mai Hoa) and Vietnamese reference texts; an upstream English crate produces outputs that cannot be diffed against those references. (3) **Algorithm opacity** — the casting logic must be auditable line-by-line in our own code with citations to specific pages of *Mai Hoa Dịch Số*; an opaque upstream function breaks the audit trail. | Hand-code Mai Hoa casting (≤30 lines of arithmetic) + ship the 64-hexagram table as a JSON corpus authored from *Kinh Dịch Trọn Bộ* (Ngô Tất Tố) and *Mai Hoa Dịch Số* (Thiệu Khang Tiết). |
| `rand` / `getrandom` (for "yarrow stalk" or "three-coin" casting methods) | (1) The v1.7 milestone scope is **Mai Hoa time-number casting only** — it is fully deterministic from the query instant, no RNG. (2) `rand`/`getrandom` is forbidden by the project's WASM posture (`amlich-wasm` target) without `js` feature hacks. (3) Project policy forbids `Utc::now()` — a RNG-based cast would violate the same "no nondeterminism" stance. | If a future milestone adds coin/yarrow casting: take an explicit `&mut impl Rng` (or `seed: u64`) parameter from the caller; never reach into the environment. (Out of scope for v1.7.) |
| `lunar_rust` or any other Chinese-calendar crate | Duplicates the project's KHCBPPT-aligned lunar conversion (`lunar.rs`, v1.0–v1.1) with a different canonical source; would create two parallel truth paths and break the "one source-of-truth per domain" invariant. | Reuse `crate::lunar::convert_solar_to_lunar` for any lunar-date input Mai Hoa requires. |
| `serde_yaml` / `toml` for the hexagram corpus | The v1.5 precedent is JSON-only (`data/rituals/*.json`, `data/almanac/*.json`). Mixing serializers adds a workspace dep for zero gain; JSON round-trips through `DaySnapshot` and the API DTOs unchanged. | JSON, schema-locked via `"$schema_version": "kinh-dich-v1"` + `#[serde(deny_unknown_fields)]`. |
| New semantic-graph node crates / graph DB libraries | The semantic graph is hand-rolled (`semantic_graph/`) with no external graph library, by design. Adding `petgraph` or similar to model `Hexagram` + `LocatedAt` / `Transforms` edges would diverge from the established pattern. | Add `Hexagram` as a new node variant in the existing `semantic_graph/builders/` pattern; add `LocatedAt` and `Transforms` as new edge variants. No new crate. |
| RNG-based or wall-clock-driven "cast at consultation time" patterns | Same nondeterminism concern as above; also breaks test reproducibility (golden tests require deterministic casting from a fixed input instant). | Casting takes the consultation instant from `DaySnapshot.context` (already supplied by the caller). |

---

## Stack Patterns by Variant

**If the Mai Hoa algorithm is implemented per Thiệu Khang Tiết's *Mai Hoa Dịch Số* (the project's chosen source):**
- Use lunar year/month/day numbers + the 12 earthly branches for the hour.
- Upper trigram index = `(year + month + day) % 8` with index 0 reserved (treat as 8) per classical convention.
- Lower trigram index = `(year + month + day + hour) % 8`, same 0→8 convention.
- Động hào = `(year + month + day + hour) % 6`, 1-indexed (1 = bottom line).
- Because these formulas appear in the source text and are testable against `nhantu.net` reference casts, **no library is needed** — hand-code them with cited page references in doc comments.

**If the Thái Tuế / Tam Sát cross-link ever needs to *write back* into `direction_merge.rs` (e.g. surfacing composite directional advice as a first-class Direction entry):**
- ⚠️ **This would VIOLATE CRIT-3 isolation** (v1.5 audit, PROJECT.md key decision row "CRIT-3 isolation: `FlyingStar` never wired into `direction_merge.rs`"). The v1.7 milestone scope explicitly forbids this.
- Use instead: keep the cross-link read-only in `reasoning/personal.rs` (or a new `reasoning/directional_composite.rs` helper) and emit two-source evidence envelopes. A Tier-3 `spatial_compose` milestone (P5 in the framework) is the proper place to revisit the boundary.

**If the 64-hexagram corpus outgrows a single file (it will not — 64 entries × ~6 lines of Vietnamese text each ≈ 40–60 KB, smaller than `data/rituals/` at 168 KB):**
- Same multi-file pattern as `rituals/corpus.rs:27-74` — one `include_str!` constant per category file (e.g. `hexagrams-1-32.json`, `hexagrams-33-64.json`) merged into a single `Vec<Hexagram>` by the loader. No new dep.

**If a future milestone (P3+) needs a *non-deterministic* I-Ching method (three-coin, yarrow):**
- Take `seed: u64` or `&mut impl Rng` from the caller — do NOT pull `rand` into `amlich-core`. Push the RNG boundary up to the API/TUI layer (`amlich-api`, `amlich-tui`) where environment access is acceptable. (Out of scope for v1.7.)

---

## Version Compatibility

| Existing package | v1.7 impact | Notes |
|------------------|-------------|-------|
| `serde` 1.0 (workspace) | None | Continues to drive all derive macros. No MSRV concern. |
| `serde_json` 1.0 (workspace) | None | Continues to parse `data/kinh_dich/*.json` via `from_str`. |
| `chrono` 0.4 (workspace) | None | `Datelike` trait methods are stable since 0.4.x — no version bump needed. |
| `unicode-normalization` 0.1.25 (direct dep) | None | Latest 0.1.x series as of research date (verified via `cargo search`). API surface used (`is_nfc`, `UnicodeNormalization::nfc`) is unchanged since v1.5 adoption. |
| Rust toolchain (no explicit MSRV pin) | None new | `std::sync::OnceLock` (stable 1.70) already in use. No edition bump required; `edition.workspace = true` resolves to "2021". |
| `amlich-wasm` target | None | `include_str!` + `OnceLock` is WASM-safe (no `fs`, no threads required beyond the once-init contract). Already proven by v1.5 rituals corpus. |

No version pins in `Cargo.toml` or `Cargo.lock` need to change for v1.7.

---

## Installation

```bash
# Core (already installed — nothing to do)
# crates/amlich-core/Cargo.toml [dependencies] block stays at:
#   serde, serde_json, chrono (workspace), unicode-normalization = "0.1.25"

# No `cargo add` commands are needed for v1.7.
# Verify with:
cargo tree -p amlich-core --depth 1
# Should still show only: serde, serde_json, chrono, unicode-normalization
# (plus dev-dependencies from [dev-dependencies], unchanged)
```

The only "installation" work for v1.7 is:
1. Add two `pub const SOURCE_*: &str` lines to `crates/amlich-core/src/sources.rs`.
2. Add two `assert_eq!` rows to `all_constants_have_expected_values` in the same file.
3. Create `crates/amlich-core/src/kinh_dich/` module + `crates/amlich-core/data/kinh_dich/` directory.
4. Drop `Copy` from `ConsultationIntent` (additive-only refactor; enumerate call-sites via `rg "ConsultationIntent" --type rust`).
5. Add `IChing` variant to `ReasoningEvidenceSourceFamily`.

Zero `cargo add` invocations.

---

## Sources

- **Codebase inspection (HIGH confidence):**
  - `crates/amlich-core/Cargo.toml` — confirmed: `serde`/`serde_json`/`chrono` (workspace) + `unicode-normalization = "0.1.25"` only. No other direct deps.
  - `Cargo.toml` (workspace root) — confirmed workspace pins: `serde 1.0`, `serde_json 1.0`, `chrono 0.4`.
  - `crates/amlich-core/src/rituals/corpus.rs` — confirmed `include_str!` + `OnceLock<Vec<RitualEntry>>` + `serde_json::from_str` + `assert_eq!(schema_version, …)` pattern (lines 17, 27-56, 85, 99-109). v1.5 precedent for v1.7 to mirror.
  - `crates/amlich-core/src/sources.rs` — confirmed `pub const SOURCE_*: &str` taxonomy + `SourceId = String` alias + CI test row pattern (lines 7-26, 41, 48-56).
  - `crates/amlich-core/src/reasoning/personal.rs` — confirmed `PersonalFactNode` / `ReasoningEvidenceEnvelope` integration point (lines 13-18, 31-107).
  - `crates/amlich-core/src/advisory.rs` — confirmed `ConsultationIntent` is `Copy` (line 18) and `Option<String>` payload would break the derive.
  - `crates/amlich-core/src/almanac/thai_tue.rs:53` — confirmed `compute_thai_tue` already exists; no new compute needed.
  - `crates/amlich-core/src/almanac/sat_phuong.rs:49` — confirmed `get_sat_phuong` (Tam Sát direction) already exists.
  - `.planning/PROJECT.md` — confirmed CRIT-3 isolation rule and source-provenance decisions.
  - `.planning/research/EXPANSION_FRAMEWORK.md` — confirmed §2.2 Kinh Dịch scope, §3.1 source provenance, §4 source book table, §7 validation references.

- **Crates.io survey (HIGH confidence on the rejection):**
  - `cargo search "i-ching"` / `"iching"` / `"hexagram"` (executed 2026-07-16) — surfaced 4 candidates: `i-ching 1.0.0` (CLI app), `xalen-iching 0.6.0` (English Legge translations, verified by inspecting `src/lib.rs:293-352`), `iching 0.5.0` (app), `lunar_rust 1.0.1` (Chinese-calendar, conflicts with KHCBPPT core).
  - `cargo search "unicode-normalization"` — confirmed `0.1.25` is current in the 0.1.x series.

- **Context7 / official docs:**
  - No Context7 queries issued — no library is being recommended for adoption, so version verification of a recommended library is moot. The only recommended "library" is `std::sync::OnceLock` (stable Rust std since 1.70, June 2023), which is already in use and verified by the v1.5 build.

- **Project precedent (HIGH confidence):**
  - v1.5 STACK.md (this directory) — established the "no new deps" precedent for Văn khấn + Phi Tinh; v1.7 is the third exercise of the same pattern.
  - ADR-0001 (`RitualEntry` JSON schema with `deny_unknown_fields`) — the template for the v1.7 Kinh Dịch schema ADR.
  - ADR-0003a / ADR-0004 — recent examples of the ADR discipline the v1.7 Kinh Dịch schema lock must follow.

---
*Stack research for: amlich-core v1.7 — Kinh Dịch (Mai Hoa Dịch Số) + Thái Tuế / Tam Sát ⇄ Phi Tinh reasoning cross-link.*
*Researched: 2026-07-16.*
