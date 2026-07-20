# Phase 11: Văn khấn Module + Lookup APIs — Research

**Researched:** 2026-05-26
**Domain:** Rust corpus loader + closed-enum matcher (Vietnamese ritual lookup)
**Confidence:** HIGH (Phase 10 locked the schema, all integration points exist in-tree)

> No CONTEXT.md exists for Phase 11 yet. This research is constrained by ADR-0001 (locked
> 2026-05-26), the existing `rituals::schema` types in `crates/amlich-core/src/rituals/schema.rs`,
> Phase 10 decisions captured in `.planning/STATE.md` lines 99–117, and the v1.5
> ARCHITECTURE.md / PITFALLS.md research files. Phase 11 must NOT alter ADR-0001 types — those
> are frozen. Any schema change requires a superseding ADR.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| RIT-01 | `find_van_khan_for_snapshot(&DaySnapshot) -> Vec<&'static RitualEntry>` matches via `Holiday.id` + `event_keys[]` overlap | §Architecture/Matcher; §Code Examples (snapshot→event_keys derivation); confirmed `Holiday.id: Option<String>` lands in Phase 10 (FND-06 ✓) populated for Tết / Vu Lan / Trung Thu / ông Táo / Đoan Ngọ etc. from `lunar-festivals.json` |
| RIT-02 | `find_van_khan_for_event(&RitualEventKey) -> Vec<&'static RitualEntry>` | §Architecture/Matcher; trivial linear scan against `all_rituals()` once `RitualEventKey: Eq` is in place (schema.rs already derives PartialEq+Eq) |
| RIT-03 | `find_van_khan_for_life_event(LifeEventKind) -> Vec<&'static RitualEntry>` | §Architecture/Matcher; `LifeEventKind` enum already locked in schema.rs:49 (6 variants: DongTho, NhapTrach, KhaiTruong, Cuoi, Gio, DayThang) |
| RIT-04 | `get_ritual_by_id(&str) -> Option<&'static RitualEntry>` | §Architecture/Matcher; linear scan over `ritual_id` field — corpus is small (≤ 60 entries Phase 12) so no HashMap needed |
| RIT-05 | `all_rituals() -> &'static [RitualEntry]` | §Code Examples (OnceLock pattern); mirrors `holiday_data::lunar_festivals()` line-for-line |
| RIT-06 | Closed `RitualEventKey` enum: Sóc/Vọng, 8 major festivals, Tiết Khí, life events, Always | §Architecture; locked in schema.rs:67 — **5 variants** (`HolidayId`, `LunarDate { month, day, leap_month_policy }`, `SolarTerm { name }`, `LifeEvent { event }`, `Always`); the "8 festivals" are NOT separate variants — they're matched via `HolidayId { value: "tet-nguyen-dan" }` etc. drawing on `lunar-festivals.json` ids |
| RIT-07 | `LunarDateMatch` variants `MonthDay`/`SolarTerm`/`GregorianFixed`, leap policy default `CanonicalMonthOnly` | §Architecture; locked in schema.rs:30 with `#[derive(Default)]` on `LeapPolicy::CanonicalMonthOnly` already enforced |
| RIT-08 | NFC normalize at load + CI Hán-character guard above threshold | §Standard Stack (`unicode-normalization 0.1.25`); §Don't Hand-Roll; §Common Pitfalls/MOD-9 |
</phase_requirements>

## Summary

Phase 11 is **pure plumbing**: ADR-0001 + `rituals/schema.rs` (Phase 10 plan 10-03) already locked
every type signature this phase consumes. The remaining work is (a) ship an `OnceLock + include_str!`
corpus loader matching the proven `holiday_data.rs` / `golden_loader.rs` pattern, (b) write a
matcher that derives `RitualEventKey` values from a `DaySnapshot` and intersects them against each
entry's `event_keys[]`, (c) normalize-to-NFC at load with a round-trip byte-equal assertion, and
(d) ship a CI integration test that rejects ritual JSON containing Hán characters above a
configured per-file threshold.

**Zero new external dependencies are needed for matching/loading** — the existing
`serde + serde_json + OnceLock + include_str!` stack handles everything. **One new dependency is
needed for NFC**: `unicode-normalization = "0.1.25"`. The Hán-character guard is a single-purpose
integration test (no crate); the test scans `data/rituals/*.json` for code points in the
CJK Unified Ideographs blocks using stdlib `char` comparisons.

The schema's `RitualEventKey::LunarDate` is a **flat struct variant** (`{ month, day, leap_month_policy }`),
NOT a wrapper around `LunarDateMatch` — this was a deliberate Phase 10 plan 10-03 decision to
avoid serde internally-tagged enum nesting conflicts. `LunarDateMatch` is preserved as a separate
standalone type usable in query APIs but is **not** the body of `RitualEventKey::LunarDate`.
The planner must treat these as two parallel types, not a wrap/unwrap pair.

**Primary recommendation:** Mirror `crates/amlich-core/src/holiday_data.rs` lines 4–6 (const JSON +
OnceLock cache) and lines 117–138 (lazy getter) one-to-one. Author 5–8 stub ritual entries under
`crates/amlich-core/data/rituals/` solely to exercise the five APIs end-to-end; full ≥60-entry
corpus is Phase 12 work and out of scope here.

## Standard Stack

### Core (already in workspace — reuse as-is)

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `serde` | workspace `1.0` | Derive `Serialize`/`Deserialize` on schema types | Already used by `rituals/schema.rs`, golden_loader, holiday_data — no new dep |
| `serde_json` | workspace `1.0` | Parse `data/rituals/*.json` strings | Standard across crate (golden_loader, holiday_data, insight_data) |
| `std::sync::OnceLock` | stdlib (Rust ≥ 1.70) | Lazy static corpus cache | Used by holiday_data:117, golden_loader:6, insight_data:186–193 — the canonical lazy-init pattern in this crate |
| `include_str!` | stdlib macro | Compile-time embed of corpus JSON | Used everywhere `data/**` is referenced; the `Cargo.toml` `include` list already lists `"data/**"` |

### Supporting (new dependency required for RIT-08)

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `unicode-normalization` | `0.1.25` | NFC normalization of `invocation_text_vi`, `title_vi`, offering descriptions, preparation step text, notes | At parse time in the corpus loader; also at test time to assert files are already NFC on disk |

**Installation:**

Add to `crates/amlich-core/Cargo.toml` under `[dependencies]`:

```toml
unicode-normalization = "0.1.25"
```

(Workspace-level addition is optional — only `amlich-core` consumes it in this milestone.)

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `unicode-normalization` | Hand-rolled NFC via stdlib | NFC requires the full UCD canonical-decomposition + composition tables. Stdlib has no `nfc()`. Hand-rolling is a guaranteed bug factory. |
| `OnceLock` | `once_cell::Lazy` | `OnceLock` is stdlib-stable since 1.70 and is what this crate already uses (10+ call-sites). Adding `once_cell` would be a dep regression. |
| `HashMap<String, &RitualEntry>` for `get_ritual_by_id` | Linear scan | Corpus is bounded at ≤ 60 entries (Phase 12 RIT-09). Linear scan is O(60) ≈ a few µs; HashMap adds initialization cost and lifetime gymnastics with `&'static`. Skip the map. |
| `unicode-blocks` crate for Hán detection | Inline `u32` range check against CJK Unified Ideographs block | The check is 4 lines (`'\u{4E00}'..='\u{9FFF}'` plus Ext-A `'\u{3400}'..='\u{4DBF}'`). Adding a crate for this is overkill. |
| `build.rs` for Hán guard | Integration test under `tests/` | `tests/` is how this crate already enforces source_id discipline (`tests/source_id_guard.rs`). Same mechanism, same precedent, runs in `cargo test`. `build.rs` would bypass CI gating. |

## Architecture Patterns

### Recommended Project Structure

```
crates/amlich-core/
├── data/
│   └── rituals/                 # NEW — Phase 11 writes minimal fixtures; Phase 12 fills corpus
│       ├── manifest.json        # OPTIONAL in Phase 11 — a single manifest can replace per-event files
│       └── <fixtures>.json      # 5–8 stub entries exercising all 5 APIs
└── src/
    └── rituals/
        ├── mod.rs               # public API re-exports + module docs (Phase 10 already wrote stub)
        ├── schema.rs            # LOCKED by ADR-0001 — DO NOT EDIT in Phase 11 (Phase 10 owns)
        ├── corpus.rs            # NEW — OnceLock loader + NFC normalization + source_id validation
        └── matcher.rs           # NEW — DaySnapshot → Vec<RitualEventKey> → filter all_rituals()
tests/
└── ritual_han_guard.rs          # NEW — CI test rejecting Hán-char-polluted ritual JSON above threshold
```

### Pattern 1: OnceLock + include_str! Corpus Loader

**What:** Embed JSON at compile time, parse once on first access, return `&'static [T]`.

**When to use:** Any read-only reference corpus < 1 MB that should never hit the filesystem at
runtime (wasm-friendly).

**Example (verbatim pattern from `holiday_data.rs:4–138`):**

```rust
// crates/amlich-core/src/rituals/corpus.rs
use std::sync::OnceLock;
use serde::Deserialize;
use unicode_normalization::{is_nfc, UnicodeNormalization};

use crate::rituals::schema::RitualEntry;
use crate::sources::SOURCE_VN_FOLK_RITUAL;

// Embed each fixture file at compile time.
// Phase 11 ships ≤ 8 stub entries; Phase 12 expands to ≥ 60 across ≤ 14 files.
const RITUAL_FIXTURES_JSON: &str = include_str!("../../data/rituals/fixtures.json");

#[derive(Debug, Deserialize)]
struct RitualFile {
    #[serde(rename = "$schema_version")]
    schema_version: String,                  // must equal "rituals-v1"
    entries: Vec<RitualEntry>,
}

static RITUALS: OnceLock<Vec<RitualEntry>> = OnceLock::new();

pub fn all_rituals() -> &'static [RitualEntry] {
    RITUALS
        .get_or_init(|| {
            let file: RitualFile = serde_json::from_str(RITUAL_FIXTURES_JSON)
                .expect("Failed to parse data/rituals/fixtures.json");
            assert_eq!(
                file.schema_version, "rituals-v1",
                "ritual corpus schema_version must be \"rituals-v1\" (ADR-0001)"
            );
            file.entries.into_iter().map(normalize_and_validate).collect()
        })
        .as_slice()
}

fn normalize_and_validate(mut entry: RitualEntry) -> RitualEntry {
    // RIT-08: source_id discipline — every entry must equal the constant.
    assert_eq!(
        entry.source_id, SOURCE_VN_FOLK_RITUAL,
        "ritual {} has source_id {:?}, expected {:?}",
        entry.ritual_id, entry.source_id, SOURCE_VN_FOLK_RITUAL
    );
    // RIT-08: NFC normalize every text field. is_nfc() is a fast early-return.
    entry.title_vi = nfc(&entry.title_vi);
    entry.invocation_text_vi = nfc(&entry.invocation_text_vi);
    if let Some(t) = entry.title_en.as_deref() { entry.title_en = Some(nfc(t)); }
    if let Some(t) = entry.body_en.as_deref()  { entry.body_en  = Some(nfc(t)); }
    for off in entry.offerings.iter_mut() {
        off.name_vi = nfc(&off.name_vi);
        if let Some(q) = off.quantity.as_deref() { off.quantity = Some(nfc(q)); }
        if let Some(n) = off.notes.as_deref()    { off.notes    = Some(nfc(n)); }
    }
    for step in entry.preparation_steps.iter_mut() {
        step.description_vi = nfc(&step.description_vi);
    }
    for note in entry.notes.iter_mut() {
        *note = nfc(note);
    }
    entry
}

fn nfc(s: &str) -> String {
    if is_nfc(s) { s.to_string() } else { s.nfc().collect() }
}
```

### Pattern 2: Closed Matcher — Snapshot → Event Keys → Vec<&RitualEntry>

**What:** Derive a `HashSet<RitualEventKey>`-like set from one `DaySnapshot`, then linear-scan all
ritual entries returning those whose `event_keys[]` overlap.

**When to use:** RIT-01 only. Other lookup APIs (RIT-02/03/04) are direct attribute filters and
need no derivation step.

**Example:**

```rust
// crates/amlich-core/src/rituals/matcher.rs
use crate::rituals::corpus::all_rituals;
use crate::rituals::schema::{LeapPolicy, LifeEventKind, RitualEntry, RitualEventKey};
use crate::holidays::get_vietnamese_holidays;
use crate::DaySnapshot;

/// RIT-01: derive the set of applicable RitualEventKey values for a given day,
/// then filter the corpus.
pub fn find_van_khan_for_snapshot(snapshot: &DaySnapshot) -> Vec<&'static RitualEntry> {
    let keys = derive_event_keys(snapshot);
    all_rituals()
        .iter()
        .filter(|entry| entry.event_keys.iter().any(|k| keys.iter().any(|d| event_key_matches(k, d))))
        .collect()
}

/// RIT-02: direct lookup by event key.
pub fn find_van_khan_for_event(event: &RitualEventKey) -> Vec<&'static RitualEntry> {
    all_rituals()
        .iter()
        .filter(|e| e.event_keys.iter().any(|k| event_key_matches(k, event)))
        .collect()
}

/// RIT-03: direct lookup by life-event kind.
pub fn find_van_khan_for_life_event(kind: LifeEventKind) -> Vec<&'static RitualEntry> {
    let needle = RitualEventKey::LifeEvent { event: kind };
    find_van_khan_for_event(&needle)
}

/// RIT-04.
pub fn get_ritual_by_id(ritual_id: &str) -> Option<&'static RitualEntry> {
    all_rituals().iter().find(|e| e.ritual_id == ritual_id)
}

/// Derive every event key applicable to this snapshot:
/// (a) Holiday.id from any holiday landing on this solar date (via get_vietnamese_holidays).
/// (b) LunarDate { month, day, leap_month_policy } from snapshot.context.lunar.
/// (c) SolarTerm { name } from snapshot.context.tiet_khi.name.
/// (d) Always — every entry tagged Always matches every day.
/// LifeEvent variants are NEVER derived from a snapshot; they require an explicit caller intent.
fn derive_event_keys(snapshot: &DaySnapshot) -> Vec<RitualEventKey> {
    let mut keys: Vec<RitualEventKey> = Vec::new();
    let ctx = &snapshot.context;

    // (a) Holiday ids — read-only join via Holiday.id (FND-06)
    for h in get_vietnamese_holidays(ctx.solar.year) {
        if h.solar_day == ctx.solar.day && h.solar_month == ctx.solar.month {
            if let Some(id) = &h.id {
                keys.push(RitualEventKey::HolidayId { value: id.clone() });
            }
        }
    }

    // (b) Lunar month-day — RIT-07 default leap policy is CanonicalMonthOnly
    keys.push(RitualEventKey::LunarDate {
        month: ctx.lunar.month as u8,
        day:   ctx.lunar.day   as u8,
        leap_month_policy: if ctx.lunar.is_leap {
            LeapPolicy::LeapMonthOnly
        } else {
            LeapPolicy::CanonicalMonthOnly
        },
    });

    // (c) Tiết Khí anchor
    keys.push(RitualEventKey::SolarTerm { name: ctx.tiet_khi.name.clone() });

    // (d) Always sentinel — every entry whose event_keys[] contains Always matches every day.
    keys.push(RitualEventKey::Always);

    keys
}

/// Matcher with leap-month policy semantics applied to LunarDate.
fn event_key_matches(haystack: &RitualEventKey, needle: &RitualEventKey) -> bool {
    use RitualEventKey::*;
    match (haystack, needle) {
        (Always, _) | (_, Always) => true,
        (HolidayId { value: a }, HolidayId { value: b }) => a == b,
        (SolarTerm { name: a }, SolarTerm { name: b }) => a == b,
        (LifeEvent { event: a }, LifeEvent { event: b }) => a == b,
        // LunarDate match must honour leap_month_policy on the *entry* side
        (LunarDate { month: m1, day: d1, leap_month_policy: p }, LunarDate { month: m2, day: d2, leap_month_policy: q }) => {
            if m1 != m2 || d1 != d2 { return false; }
            match (p, q) {
                (LeapPolicy::Either, _) | (_, LeapPolicy::Either) => true,
                (LeapPolicy::CanonicalMonthOnly, LeapPolicy::CanonicalMonthOnly) => true,
                (LeapPolicy::LeapMonthOnly, LeapPolicy::LeapMonthOnly) => true,
                _ => false,
            }
        }
        _ => false,
    }
}
```

### Pattern 3: CI Hán-Character Guard (integration test)

**What:** Walk `data/rituals/*.json`, parse each file, scan every string for code points in
CJK Unified Ideographs blocks, fail the test if any *single* string field crosses the per-field
threshold OR if the corpus-wide count of Hán code points crosses the global threshold.

**When to use:** Always — runs in `cargo test` via Cargo's standard `tests/` directory and gates CI
the same way `tests/source_id_guard.rs` does for source_id literals.

**Recommended thresholds (low confidence — finalize during planning):**
- Per-string field threshold: **0 Hán chars allowed by default** in `invocation_text_vi`,
  `title_vi`, offering names, preparation step descriptions. Văn khấn corpus is Quốc-ngữ.
- Global corpus override: an opt-in **`hannom_text: Option<String>`** field on `RitualEntry` is
  reserved for entries that explicitly cite Sino-Vietnamese (Hán-Nôm) phrases — but ADR-0001 does
  NOT include this field, so adding it is out of scope. Default = strict 0.
- If a future entry requires Hán quotation, the resolution is a superseding ADR + schema field, not
  a per-file allow-list.

**Example:**

```rust
// crates/amlich-core/tests/ritual_han_guard.rs
use std::fs;
use std::path::Path;

fn is_han_char(c: char) -> bool {
    matches!(c,
        '\u{4E00}'..='\u{9FFF}'   // CJK Unified Ideographs
      | '\u{3400}'..='\u{4DBF}'   // CJK Ext-A
      | '\u{20000}'..='\u{2A6DF}' // CJK Ext-B
      | '\u{F900}'..='\u{FAFF}'   // CJK Compatibility Ideographs
    )
}

#[test]
fn ritual_corpus_rejects_han_characters() {
    let data_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("data/rituals");
    if !data_dir.exists() {
        // Phase 11 may ship before data/rituals/ exists; the test gracefully no-ops in that case.
        return;
    }
    let mut violations: Vec<String> = Vec::new();
    for entry in fs::read_dir(&data_dir).expect("read_dir") {
        let path = entry.expect("dir entry").path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") { continue; }
        let body = fs::read_to_string(&path).expect("read file");
        let han_count = body.chars().filter(|&c| is_han_char(c)).count();
        if han_count > 0 {
            violations.push(format!("{}: {} Hán code points found", path.display(), han_count));
        }
    }
    assert!(violations.is_empty(),
        "Hán characters detected in ritual corpus (threshold=0). \
         Văn khấn must be Quốc-ngữ. Violations:\n{}", violations.join("\n"));
}
```

### Anti-Patterns to Avoid

- **Re-defining schema types in `corpus.rs`.** All public types live in `rituals/schema.rs` per
  ADR-0001. `corpus.rs` only ships the loader + the private `RitualFile` wrapper.
- **Adding new variants to `RitualEventKey`.** Phase 10 plan 10-03 locked exactly five variants. The
  "8 major lunar festivals" mentioned in RIT-06 are **NOT** separate enum variants — they are
  `HolidayId { value }` instances with the canonical ids drawn from `lunar-festivals.json`
  (see Code Examples → "Holiday IDs available to ritual authors").
- **Building a `HashMap<String, &RitualEntry>` index.** With ≤ 60 entries the linear scan is faster
  than building the hashmap and dealing with `&'static` lifetime gymnastics.
- **Normalizing at every API call.** Normalize once at `OnceLock` init. APIs return `&'static`
  references; callers cannot mutate.
- **Reading the corpus from disk at runtime.** Use `include_str!`. wasm and embedded targets cannot
  do filesystem I/O.
- **Adding a `valid_year_range` field on `RitualEntry`.** ARCHITECTURE.md §1.2 explicitly forbids
  this — văn khấn is timeless; year-gating belongs in the holiday detector.
- **Coupling `holidays.rs` to `rituals/`.** The dependency arrow is one-way: `rituals` → `holidays`.
  Never the reverse (see Phase 10 plan 10-02 decisions; Holiday derives only Debug+Clone today).

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Unicode NFC normalization | Manual decomposition + recomposition tables | `unicode-normalization::UnicodeNormalization::nfc()` | NFC requires the full UCD canonical-decomposition matrix; hand-rolling is a multi-week project that *will* miscompose Vietnamese stacked diacritics (ế = e + ˆ + ´, with three legal decomposition orders) |
| Detecting non-NFC strings | Byte comparison | `unicode_normalization::is_nfc(s)` | Quick check that returns early for already-NFC text; the slow path uses authoritative recomposition |
| Lazy static cache | `lazy_static!`, custom Mutex, or raw `static mut` | `std::sync::OnceLock` (stdlib ≥ 1.70) | Already adopted by this crate at 10+ call-sites; thread-safe; no external dep |
| Compile-time JSON embedding | `build.rs` + `fs::read_to_string` | `include_str!` | Standard across this crate (holiday_data, golden_loader, insight_data) |
| Holiday→Ritual join key invention | Inventing a "category" or "name-substring-match" join | Use `Holiday.id` (Phase 10 FND-06 locked) + `RitualEventKey::HolidayId { value }` | `lunar-festivals.json` already carries stable kebab-case ids; matching on `name.contains("Tết")` is a localization landmine |
| CI Hán detection | `regex` crate | Inline char range check `'\u{4E00}'..='\u{9FFF}'` + Ext-A | Zero new dep; the test is 30 lines; regex is overkill |
| Schema versioning | Inventing a versioning scheme | Use `$schema_version: "rituals-v1"` per ADR-0001 §Serde Discipline | Loader asserts the value matches; future v2 corpus files declare `"rituals-v2"` and require a superseding ADR |

**Key insight:** Phase 11 is **80% wiring** — every type, every constant, every JSON shape decision
was already made in Phase 10 (ADR-0001) or earlier (FND-06 Holiday.id). The single technical
choice Phase 11 makes is "where to normalize" (answer: at OnceLock init, not at API call time) and
"how to detect Hán" (answer: inline char-range check, threshold = 0).

## Common Pitfalls

### Pitfall 1: NFC Drift — Same Text, Different Bytes (PITFALLS MOD-9)

**What goes wrong:** Văn khấn authors may save JSON from text editors that emit NFD (e.g. macOS
default) or mixed forms. Two ritual entries with visually identical titles compare unequal under
`==` because one is NFC, the other is NFD. Search/lookup silently returns nothing.

**Why it happens:** Vietnamese stacked tone marks (`ế`, `ậ`, `ự`) have multiple legal Unicode
sequences. NFC chooses the precomposed form; NFD decomposes to base letter + combining marks.

**How to avoid:**
1. **Normalize at load** (corpus.rs:normalize_and_validate above). Every string field is `.nfc()`-collected
   on first access; the OnceLock cache stores the canonical form.
2. **Round-trip byte-equal test.** A test in `rituals/corpus.rs` (`#[cfg(test)]` module) asserts
   `is_nfc(s) == true` for every string field on every loaded entry — proves nothing got missed.
3. **On-disk lint** (optional, recommend Phase 12): a separate test reads every
   `data/rituals/*.json` raw and asserts the file body satisfies `is_nfc()`. Catches drift before
   the loader can paper over it.

**Warning signs:** Lookup that returns 0 entries for a known-good ritual id; visual diff of two
strings that look identical in the editor but differ in `.len()`.

### Pitfall 2: Holiday → Ritual Join Breaks Silently (PITFALLS CRIT-1 family)

**What goes wrong:** Ritual author writes `event_keys: [{"kind": "holiday_id", "value": "tet"}]`
expecting Tết Nguyên Đán. The corpus loader accepts the entry; the matcher never fires because the
canonical Holiday id is `"tet-nguyen-dan"`, not `"tet"`. No error, no warning — just empty results.

**Why it happens:** `HolidayId.value` is a free-form `String`. The schema cannot constrain it to
"only ids actually in lunar-festivals.json".

**How to avoid:**
1. **Cross-reference test** in `tests/`: walk `all_rituals()`, collect every `HolidayId.value`,
   intersect with the set of ids returned by `get_vietnamese_holidays(2024)` (or any in-range year);
   assert each ritual's referenced id is actually realisable. Fails the build if a typo lands.
2. **Document the canonical id list** in `rituals/mod.rs` doc comment — point ritual authors at
   `data/holidays/lunar-festivals.json` as the source of truth.

**Warning signs:** A ritual entry with `event_keys: [HolidayId]` returning 0 from
`find_van_khan_for_snapshot` on a day that matches the holiday name.

### Pitfall 3: LunarDate Variant Confusion (Phase 10 plan 10-03 nuance)

**What goes wrong:** A planner reads ADR-0001 and assumes `RitualEventKey::LunarDate(LunarDateMatch)`.
The actual schema is `LunarDate { month, day, leap_month_policy }` — a *flat* struct variant. Code
written against the assumed wrapper fails to compile.

**Why it happens:** Plan 10-03 explicitly changed the shape (STATE.md line 100) to avoid serde
internally-tagged enum nesting conflicts. `LunarDateMatch` was kept as a *standalone* type for
query APIs (not embedded inside `RitualEventKey`).

**How to avoid:** Read `rituals/schema.rs:67–78` before writing any matcher code. The
`#[serde(tag = "kind")]` on the outer enum collides with a nested `#[serde(tag = "kind")]` on the
inner — that's why the variants are flat.

**Warning signs:** `error[E0532]: expected tuple struct or tuple variant, found struct variant`.

### Pitfall 4: Leap-Month Policy Edge Case

**What goes wrong:** Ritual entry says `event_keys: [{"kind": "lunar_date", "month": 5, "day": 5}]`
(Đoan Ngọ, omitting leap_month_policy — defaults to `CanonicalMonthOnly`). User queries a leap-month-5
snapshot; matcher *should not* fire because Tết Đoan Ngọ is observed in the canonical month 5, not
the leap month 5. Naive implementation matches by month+day only, fires incorrectly.

**How to avoid:** Honour `leap_month_policy` in the matcher (`event_key_matches` above). The
`derive_event_keys` function sets the snapshot side to `LeapMonthOnly` when `ctx.lunar.is_leap`,
which lets the matcher reject canonical-only entries cleanly.

**Warning signs:** A test with leap-month-5 2025 (or any leap fixture) returning Đoan Ngọ rituals
that should not apply.

### Pitfall 5: Including `corpus.rs` mod.rs Public API Drift

**What goes wrong:** Phase 10 wrote `rituals/mod.rs` with only `pub mod schema;`. Phase 11 must
register `corpus` and `matcher` submodules AND re-export the five public APIs at the module level
(RIT-01..05). Forgetting to re-export means `amlich_core::rituals::find_van_khan_for_snapshot`
doesn't resolve from external crates.

**How to avoid:** `rituals/mod.rs` Phase 11 update should be:

```rust
pub mod schema;
mod corpus;       // private — only the public API is exposed
mod matcher;      // private

pub use schema::*;                  // RitualEntry, RitualEventKey, LifeEventKind, etc.
pub use corpus::all_rituals;        // RIT-05
pub use matcher::{                   // RIT-01..04
    find_van_khan_for_snapshot,
    find_van_khan_for_event,
    find_van_khan_for_life_event,
    get_ritual_by_id,
};
```

**Warning signs:** Doc-test or external-crate consumer can't find `find_van_khan_*` despite the
internal tests passing.

### Pitfall 6: source_id Discipline — Bare String Literals in `corpus.rs`

**What goes wrong:** `corpus.rs` does an `assert_eq!(entry.source_id, "vn-folk-ritual")`.
`tests/source_id_guard.rs` finds the bare literal and fails the build.

**How to avoid:** Always compare against `crate::sources::SOURCE_VN_FOLK_RITUAL`. The
source_id_guard test's `FORBIDDEN_LITERALS` includes `"\"vn-folk-ritual\""` — every production
call-site MUST use the constant.

**Warning signs:** `cargo test --test source_id_guard` fails after Phase 11 lands; CI rejects.

## Code Examples

### Holiday IDs available to ritual authors (drawn from `data/holidays/lunar-festivals.json`)

The eight "major lunar festivals" called out by RIT-06 map to these canonical Holiday ids:

| Festival (Vietnamese) | `Holiday.id` |
|-----------------------|--------------|
| Tết Nguyên Đán | `tet-nguyen-dan` |
| Tết Nguyên Tiêu / Thượng Nguyên / Rằm tháng Giêng | `tet-nguyen-tieu` |
| Tết Hàn Thực | `tet-han-thuc` |
| Tết Thanh Minh (solar, computed) | `tet-thanh-minh` (note: `Holiday.id` is **None** in current `holidays.rs` because Thanh Minh is computed from the Tiết Khí scanner, not the corpus — ritual authors should use `{"kind": "tiet_khi", "name": "Thanh Minh"}` instead, or expose via FND-06 follow-up) |
| Tết Đoan Ngọ | `tet-doan-ngo` |
| Lễ Vu Lan / Rằm tháng Bảy | `vu-lan` |
| Tết Trung Thu | `tet-trung-thu` |
| Tết Trung Cửu | `tet-trung-cuu` |
| Tết Hạ Nguyên | `tet-ha-nguyen` |
| Ông Công Ông Táo / Ngày 23 tháng Chạp | `ong-tao` |
| Giao Thừa | `giao-thua` |
| Mùng 2 Tết / Mùng 3 Tết | `mung-2-tet` / `mung-3-tet` |
| Phật Đản | `phat-dan` |

Source: grep of `crates/amlich-core/data/holidays/lunar-festivals.json` (Phase 11 research scan).

> **Caveat — Thanh Minh:** `holidays.rs:177` assigns `id: None` to the computed Thanh Minh entry,
> because the value comes from the Tiết Khí scanner not the corpus. Ritual entries for Thanh Minh
> must use `{"kind": "tiet_khi", "name": "Thanh Minh"}` — NOT a HolidayId. This is the only
> "solar-via-tiet-khi" override in the corpus.

> **Caveat — Sóc/Vọng (Mùng 1, Rằm):** `holidays.rs:240–268` also assigns `id: None` to the 12×
> Mùng 1 and 12× Rằm entries (they're auto-generated, not from JSON). Ritual authors target Sóc/Vọng
> via `{"kind": "lunar_date", "month": <m>, "day": 1}` and `{"day": 15}` respectively. The Phase 11
> matcher's `derive_event_keys` must therefore emit a `LunarDate` key for every snapshot, which the
> implementation above already does.

### Sample stub corpus entry (Phase 11 fixture)

```json
{
  "$schema_version": "rituals-v1",
  "entries": [
    {
      "ritual_id": "van-khan-tet-don-gian",
      "title_vi": "Văn Khấn Tết Nguyên Đán (Đơn Giản)",
      "event_keys": [
        {"kind": "holiday_id", "value": "tet-nguyen-dan"},
        {"kind": "lunar_date", "month": 1, "day": 1}
      ],
      "variant": "simple",
      "offerings": [
        {"name_vi": "Hương", "quantity": "3 nén"},
        {"name_vi": "Hoa tươi", "quantity": "1 bình"}
      ],
      "preparation_steps": [
        {"order": 1, "description_vi": "Tắm rửa sạch sẽ, mặc quần áo chỉnh tề"},
        {"order": 2, "description_vi": "Bày lễ vật lên bàn thờ gia tiên"}
      ],
      "invocation_text_vi": "Nam mô a di đà phật! Con lạy chín phương trời, mười phương chư phật...",
      "source_id": "vn-folk-ritual",
      "original_citation": {
        "title": "Văn Khấn Cổ Truyền Việt Nam",
        "publisher": "NXB Văn Hóa Thông Tin",
        "edition": "2003",
        "page": "12"
      },
      "confidence": "primary"
    }
  ]
}
```

(Verbatim from ADR-0001 §Sample JSON Entry, wrapped in the `RitualFile` envelope.)

### Recommended stub corpus minimum (Phase 11 only — Phase 12 expands to ≥60)

To exercise all five APIs end-to-end, ship at least:

1. One entry with `event_keys: [HolidayId{tet-nguyen-dan}, LunarDate{m=1,d=1}]` — covers RIT-01 snapshot, RIT-02 event, RIT-04 by-id.
2. One entry with `event_keys: [LunarDate{m=*, d=1, CanonicalMonthOnly}]` — Mùng 1 generic, covers Sóc.
3. One entry with `event_keys: [LunarDate{m=*, d=15}]` — Rằm generic, covers Vọng.
4. One entry with `event_keys: [SolarTerm{Thanh Minh}]` — covers Tiết Khí path.
5. One entry with `event_keys: [LifeEvent{DongTho}]` — covers RIT-03.
6. One entry with `event_keys: [Always]` — covers fall-through.
7. (Optional) One entry with `variant: Regional("mien-bac")` — covers ADR-0001 variant tag round-trip.
8. (Optional) One entry with `event_keys: [LunarDate{m=5,d=5,LeapMonthOnly}]` — covers leap-policy semantics.

5 entries is the minimum; 8 covers all reasonable test paths.

## State of the Art

| Old Approach (pre-Phase 10) | Current Approach (Phase 10 locked) | When Changed | Impact |
|-----------------------------|------------------------------------|--------------|--------|
| Freeform `event_type: String` matching | Closed `RitualEventKey` enum with 5 variants | 2026-05-26 (ADR-0001) | Compiler enforces exhaustive match in `event_key_matches`; no typo-class bugs |
| Lễ vật / trình tự as freeform strings | Structured `Offering` / `PreparationStep` types | 2026-05-26 (ADR-0001) | UI can enumerate items, validate order, render structured components |
| Holiday name substring match | `Holiday.id` join (FND-06) | 2026-05-26 (Phase 10 plan 10-02) | Localization-stable; survives renames in `names.vi` |
| Hand-rolled lazy statics | `std::sync::OnceLock` | Rust 1.70 (2023) | Standard, thread-safe, no `lazy_static!` dep |

**Deprecated / outdated:**
- ARCHITECTURE.md §1.3 (research file, lines 172–197) shows an *earlier* draft of `RitualEntry` and
  `RitualEventKey` (e.g. `HolidayId(String)` tuple variant, `time_of_day: TimeOfDay` field,
  `invocation_text_en_summary: String`). **That draft was superseded by ADR-0001.** Use only the
  shapes in `rituals/schema.rs` and ADR-0001.
- ARCHITECTURE.md API draft uses `find_van_khan_for_snapshot(snapshot) -> Vec<&'static RitualEntry>`
  which matches the locked ADR-0001 expectation (return type is `&'static`, not owned). Phase 11
  preserves this.

## Open Questions

1. **Hán-character threshold: 0 or > 0?**
   - What we know: PITFALLS MOD-9 line 25 says `rg -l '[一-龥]' data/rituals/` should be empty
     "except for entries that explicitly declare bilingual Sino-Vietnamese (Hán-Việt) provenance".
   - What's unclear: ADR-0001 has NO `hannom_text` field, so there is no schema place to declare
     Hán-Việt provenance. The "explicit declaration" path doesn't exist.
   - Recommendation: **Threshold = 0** in Phase 11. If Phase 12 corpus authors need Hán quotation,
     they file an ADR superseding 0001 to add the field, and the test threshold becomes a per-file
     allow-list (e.g. only entries with `hannom_text != null` may contain Hán). Out of scope here.

2. **Manifest.json — required in Phase 11 or deferred to Phase 12?**
   - What we know: RIT-09 (Phase 12) requires `manifest.json` + ≤ 14 per-event-category files for
     the full corpus.
   - What's unclear: Phase 11 only needs a working loader. A single `fixtures.json` file with
     `$schema_version: "rituals-v1"` and an `entries: []` array is the minimal shape.
   - Recommendation: **Skip manifest.json in Phase 11**, ship one `fixtures.json`. Phase 12 adds
     the manifest + per-category split. The loader can detect single-file vs multi-file at
     `OnceLock` init.

3. **Error type for loader — `Result` or panic?**
   - What we know: `holiday_data.rs`, `golden_loader.rs`, `insight_data.rs` all `.expect()` on
     parse — corpus is compile-embedded; a parse failure is a build-time bug, not a runtime error.
   - What's unclear: Should `corpus.rs` return `Result<&'static [RitualEntry], CorpusError>` for
     future flexibility?
   - Recommendation: **Panic on init via `.expect()`** matching the crate-wide pattern. There's no
     legitimate runtime recovery — if the embedded JSON is malformed, the binary is broken. Phase 11
     is consistent with the crate; reconsider only if a use case for fallible loading appears.

4. **Should `derive_event_keys` emit a `LifeEvent` from snapshot?**
   - What we know: Roadmap RIT-01 says "matching the day's lunar date, Tiết Khí anchor, and active
     holidays" — no mention of life events.
   - What's unclear: A snapshot has no "life event" intent (Động thổ / Cưới are caller choices, not
     day properties).
   - Recommendation: **Do NOT emit `LifeEvent` keys from `derive_event_keys`.** Life events are
     surfaced only via the explicit `find_van_khan_for_life_event(kind)` API (RIT-03). Document
     this in the `mod.rs` doc comment.

5. **`derive_event_keys` Sóc/Vọng exhaustivity for tests**
   - What we know: Every day must emit a `LunarDate { month: ctx.lunar.month, day: ctx.lunar.day }`
     key. On Sóc (day 1) and Vọng (day 15), this matches month-agnostic Sóc/Vọng entries.
   - What's unclear: Should there be a separate sentinel `RitualEventKey::Soc` / `::Vong`?
     ADR-0001 says NO — only the five locked variants exist.
   - Recommendation: **Stay with `LunarDate{day=1}` / `LunarDate{day=15}` matching.** Ritual entries
     for Sóc/Vọng generically use those forms. Tests must verify the matcher fires on every Sóc
     and Vọng day across a year-long snapshot scan.

## Sources

### Primary (HIGH confidence)
- `.planning/adrs/0001-ritual-schema-v1.md` — locked v1 ritual schema, full field set, closed enums,
  serde discipline, sample JSON entry.
- `crates/amlich-core/src/rituals/schema.rs` — actual locked Rust types (10 types, 5 unit tests pass).
- `crates/amlich-core/src/holiday_data.rs:4–138` — proven `include_str! + OnceLock + Vec<T>` loader
  pattern; mirror exactly.
- `crates/amlich-core/src/almanac/golden_loader.rs:1–21` — alternative `OnceLock` example with
  validation-at-load.
- `crates/amlich-core/src/holidays.rs:14–26, 90–98, 142–166` — `Holiday.id: Option<String>` field
  shape (FND-06) and the population path from `lunar_festivals[].id`.
- `crates/amlich-core/src/sources.rs:22` — `pub const SOURCE_VN_FOLK_RITUAL: &str = "vn-folk-ritual"`.
- `crates/amlich-core/tests/source_id_guard.rs` — the CI source_id guard pattern; the Phase 11 Hán
  guard mirrors its file-scan + assert-violations-empty shape.
- `crates/amlich-core/src/lib.rs:133–142` — actual `DaySnapshot` shape; contains `context`,
  `day_fortune`, `daily_recommendations`, `contextual_recommendations`. `context: DayContext` carries
  `solar`, `lunar` (with `is_leap: bool`), `tiet_khi: SolarTerm { name: String }` — every field
  the matcher needs.
- `crates/amlich-core/data/holidays/lunar-festivals.json` — canonical Holiday id source (lines 1–700)
  for ritual `HolidayId.value` references.
- `.planning/research/ARCHITECTURE.md:30–225` — Phase 11 module layout, holiday integration recipe,
  one-way dependency rationale.
- `.planning/research/PITFALLS.md:25, 200–212, 256–292, 307–331` — MOD-9 (NFC drift), MIN-3 (title/body
  mixing), MIN-5 (Gregorian-without-lunar test antipattern).

### Secondary (MEDIUM confidence)
- [unicode-normalization 0.1.25 docs.rs](https://docs.rs/unicode-normalization/) — confirmed
  `nfc()`, `is_nfc()`, `is_nfc_quick()` APIs; `no_std + alloc` compatible; tinyvec dep.
- [unicode-normalization crates.io](https://crates.io/crates/unicode-normalization) — version,
  Rust 1.36+ MSRV.
- ARCHITECTURE.md draft API at lines 172–197 — superseded by ADR-0001 but useful for understanding
  the original intent and one-way dependency rationale.

### Tertiary (LOW confidence)
- Hán-character threshold of 0 — based on PITFALLS MOD-9 wording, not an explicit ADR. Confirm
  during planning.
- [CJK Unified Ideographs Wikipedia](https://en.wikipedia.org/wiki/CJK_Unified_Ideographs) —
  block ranges (4E00–9FFF base, 3400–4DBF Ext-A) widely documented but not authoritative for the
  exact threshold strategy.
- [unicode-blocks docs.rs](https://docs.rs/unicode-blocks) — alternative dep for Hán detection;
  REJECTED in favour of inline char range (zero new dep).

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — every type and pattern lives in-tree already; one trivial new dep
  (`unicode-normalization 0.1.25`) is well-established (since 2014, current docs verified).
- Architecture: HIGH — ARCHITECTURE.md §1.1–1.4 specifies the module layout, file naming, and
  dependency arrow; ADR-0001 freezes every schema decision; Phase 10 plan 10-03 already wrote
  `schema.rs` and registered the module in `lib.rs`.
- Pitfalls: HIGH for items 1–6 (every one is grounded in either ADR-0001 review, PITFALLS.md
  catalogue, or grep of `source_id_guard.rs`). MEDIUM only on the Hán-character threshold (=0 is
  research-team intent but not formally adopted).

**Research date:** 2026-05-26
**Valid until:** 2026-06-25 (30 days — schema is locked; ecosystem deps are stable)

---

*Sources cited:*

- [unicode-normalization on docs.rs](https://docs.rs/unicode-normalization/)
- [unicode-normalization on crates.io](https://crates.io/crates/unicode-normalization)
- [CJK Unified Ideographs (Wikipedia)](https://en.wikipedia.org/wiki/CJK_Unified_Ideographs)
- [CJK Unified Ideographs Block reference](https://symbl.cc/en/unicode/blocks/cjk-unified-ideographs/)
