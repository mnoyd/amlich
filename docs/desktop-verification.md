# Desktop App Verification — Quality Gates & Acceptance Checklist

Verification plan for the redesigned **Amlich Observatory** desktop app (Tauri + SvelteKit).

Covers: automated quality gates (Rust + frontend) and a manual acceptance checklist for
all eight workspaces.

---

## 1. Automated quality gates

These run in CI (`.github/workflows/ci.yml`, `desktop` job) and locally via `just`.

### 1.1 Rust — Tauri command layer (`am-lich` crate)

| Gate | Command | What it checks |
|------|---------|----------------|
| Unit tests | `cargo test --package am-lich` | All `#[tauri::command]` wrappers return well-formed DTOs for representative inputs; validators reject out-of-range date parts; gender aliases resolve; install context reports platform/arch/version. 18 tests. |
| Clippy | `cargo clippy --package am-lich --no-deps -- -D warnings` | The desktop crate is lint-clean. `--no-deps` isolates from pre-existing `amlich-core` lints tracked by `amlich-081`. |

> **Why `--no-deps`?** `amlich-core` carries clippy warnings from newer toolchain lints
> (`manual_is_multiple_of`, `unnecessary_cast`, …). Those are owned by `amlich-081`.
> The desktop crate must not add new warnings of its own.

#### Commands covered by tests (`apps/desktop/src-tauri/src/lib.rs`)

- `get_month_data` — calendar grid construction + month-range validation
- `get_day_detail` — decorated day cell + out-of-range rejection
- `get_day_bundle` — full v2 bundle (canchi, tiet_khi, gio_hoang_dao)
- `get_day_info` / `get_day_insight` — consistent solar/lunar anchor
- `get_day_range` — multi-day bundle with include flags
- `get_bazi_report` — summary, signals, actions; gender pass-through
- `get_bazi_derived_report` — thai nguyen, menh cung, tier
- `get_hour_selection_report` — chart + metrics
- `get_tiet_khi_for_year` — ≥24 transitions with names
- `get_ruleset_catalog` / `get_recommendation_pack_catalog` — non-empty entries
- `get_holidays_list` — major-only filter
- `get_personal_day_report` / `get_personal_day_matrix_report` — reasoning bundle + matrix
- `get_install_context` — platform/arch/version metadata
- `parse_gender` — alias acceptance and rejection

### 1.2 Frontend — SvelteKit layer (`apps/desktop`)

| Gate | Command | What it checks |
|------|---------|----------------|
| Type check | `cd apps/desktop && pnpm check` (`svelte-check`) | TypeScript + Svelte template type errors across all routes/components. Must report `0 errors, 0 warnings`. |
| Frontend build | `cd apps/desktop && pnpm build` (`vite build`) | SPA bundle compiles and is written to `build/` via `@sveltejs/adapter-static`. |

> **Lint gap.** No ESLint/Prettier configuration ships with the desktop frontend yet.
> Until one is added, `svelte-check` + `vite build` are the authoritative frontend gates.
> Filed as follow-up.

### 1.3 Tauri build smoke

A full `pnpm tauri build` requires platform webview dependencies
(`libwebkit2gtk-4.1-dev` on Linux, Cocoa/WebKit on macOS, WebView2 on Windows) and is
slow. It is intentionally **not** part of the per-push smoke gate.

| Gate | Command | When |
|------|---------|------|
| Combined smoke | `just smoke-desktop` | Pre-push / per-PR. Runs Rust tests + clippy + frontend type-check + frontend build. |
| Full bundle | `just build-app` (`pnpm tauri build`) | Release only. Produces installers (`AppImage`/`deb`/`dmg`). Run via the `Release` workflow on tag push. |

`just smoke-desktop` is the CI-equivalent gate and is what the `desktop` CI job runs.

### 1.4 pnpm configuration note

`apps/desktop/pnpm-workspace.yaml` declares `allowBuilds: { esbuild: true }`. This is the
pnpm 11 format that permits esbuild's postinstall to run. The older
`pnpm.onlyBuiltDependencies` field has been removed from `package.json` (pnpm 11 ignores
it and warns).

---

## 2. Manual acceptance checklist

Run the app with `just dev` (or `cd apps/desktop && pnpm tauri dev`). Today's date is
pre-selected. Walk each workspace via the left rail.

**Global setup:** before testing Personal / Bazi / Evidence, have a test birth profile
ready, e.g. `1990-01-01 09:30 Nam`.

### 2.1 Day Console (`day_console`)

- [ ] Landing view opens by default; today's solar date is shown in the header.
- [ ] Lunar date is correct; `(Nhuận)` marker appears only on leap months.
- [ ] Tiết Khí badge shows the current solar term; Can-Chi triad (Năm/Tháng/Ngày) is populated.
- [ ] Vietnamese summary line is non-empty.
- [ ] **Activity Board** renders four columns: Nên / Có thể / Tránh / Kỵ Mạnh.
- [ ] Each activity card shows a per-bucket count and at least one reason.
- [ ] Changing the selected date re-fetches the bundle (loading pulse, no stale data).

### 2.2 Activity Board (`activity_board`)

> **Known issue.** The left-rail entry for Activity currently renders the
> "under construction" fallback. The Activity Board content lives inside Day Console (§2.1).
> Either remove the nav entry or wire `+page.svelte` to a dedicated route. Tracked separately.

- [ ] Confirm the dead-link behavior: clicking "Activity" in the rail shows the fallback panel.
- [ ] Confirm the Activity Board itself (inside Day Console) is functional per §2.1.

### 2.3 Hour Studio (`hour_studio`)

- [ ] All 12 chi-hours render in the two-column timeline.
- [ ] Each tile shows `hour_chi`, `time_range`, and ruling star.
- [ ] Hoàng Đạo (good) hours are badged; the rest are neutral.
- [ ] Right rail lists `best_windows` (Nên) and `caution_windows` (Tránh).
- [ ] Advisory summary (`advisory.summary_vi`) is present and in Vietnamese.
- [ ] Changing the date refreshes the hour roster.

### 2.4 Almanac Inspector (`almanac_inspector`)

- [ ] Header shows solar date, Can Chi, `profile`, `ruleset_id`, `ruleset_version`, Trực quality.
- [ ] Nap Âm / day element section is populated.
- [ ] Conflict section lists opposing chi, tuổi xung, sát hướng.
- [ ] Travel directions (xuất hành, tài thân, hỷ thần) are present.
- [ ] Stars section shows cat/sat tinh with matched rules.
- [ ] Day Deity & Trực insight meanings render.
- [ ] Xung Hợp/Hại/Hình and Tàng Can / Ten Gods sections render.
- [ ] Taboos are merged from fortune + insight with severity tags.
- [ ] Right aside shows Explanatory Insight (good_for / avoid_for) and the Rule Evidence provenance table.
- [ ] Severity color-coding (cát vs. sát; hard vs. soft) is visually distinguishable.

### 2.5 Season Timeline (`season_timeline`)

- [ ] Year header shows the selected year; four stat badges (Festivals / Holidays / Cycle / Tiết khí) render counts.
- [ ] Event list combines holidays, festivals, lunar-cycle days (Mùng 1 / Rằm), and Tiết Khí transitions.
- [ ] Filter chips work: `All / Festival / Holiday / Lunar_cycle / Tiết_khi`.
- [ ] "Major only" toggle filters the list.
- [ ] Clicking an event selects it; right pane shows full detail (Origin, Significance, Activities, Traditions, Food, Taboos, Proverbs, regional notes — or Meaning/Weather/Agriculture/Health for Tiết Khí).
- [ ] Changing the year (via `selectedDate`) re-fetches the full year range.

### 2.6 Personal Lab (`personal_lab`)

- [ ] Form accepts Birth Year / Month / Day / Hour / Minute / Gender.
- [ ] With date-only (no time), the personal day report renders; matrix sections show an "unavailable until time provided" explanation.
- [ ] With full date + time, the Personal Day Matrix unlocks: personal hours, direction merge, domain boost.
- [ ] Tier badge reflects birth-data completeness (x/4).
- [ ] Decision card shows bucket, confidence, semantic, context clarity, primary conclusion, supports vs. resistance.
- [ ] Axis Scores render normalized bars with the ≥0.35 / ≤−0.35 color thresholds.
- [ ] Personal Hours (top 5), Directions (top 5), and Domain Boost (top 4) lists populate.
- [ ] Profile is pushed to the shared `userProfile` store (verify by switching to Evidence — profile carries over).

### 2.7 Bazi Lab (`bazi_lab`)

- [ ] Form defaults: `1990-01-01`, `12:00`, gender Nam.
- [ ] Empty state prompts "Enter birth data and click Generate Chart."
- [ ] Clicking **Generate Chart** fetches both `bazi_report` and `bazi_derived_report`.
- [ ] Day Master headline + advisory summary render.
- [ ] Four Pillars row (Năm / Tháng / Ngày / Giờ) shows Can, Chi, Nap Âm, Tàng Can with strength %.
- [ ] Dụng Thần (favorable / unfavorable elements + reasons) renders.
- [ ] Hợp Hoá / Xung Khắc interactions render.
- [ ] Cường Nhược (day-master strength) renders.
- [ ] Ngũ Hành element distribution bars render.
- [ ] Lưu Ý warnings render when applicable.
- [ ] Changing gender and regenerating produces a different advisory.

### 2.8 Evidence Graph (`evidence_graph`)

- [ ] Form accepts Birth Year / Month / Day / Gender (no time field by design).
- [ ] Empty state prompts for a birth date.
- [ ] **Vì Sao lens** — Decision verdict card + four role groups (Ghi đè / Xung đột / Kháng cự / Hỗ trợ) with source-family badges and provenance envelopes.
- [ ] **Yếu Tố lens** — Nodes grouped by kind (Decision / Signal / Fact); clicking a node opens a detail pane with severity, axis, tags, evidence, and incoming/outgoing edges.
- [ ] **Trục lens** — Five canonical axis bars + six reasoning-axis bars render with verdicts.
- [ ] **Nguồn lens** — Source-family breakdown table with totals, percentage bars, per-family counts.
- [ ] **Dev lens** — Raw graph dump (node/edge totals, severity/effect counts, full lists). Acceptable to be dense; verify it renders without error.
- [ ] Profile is pushed to the shared `userProfile` store.
- [ ] Switching lenses preserves the selected node where applicable.

---

## 3. Known follow-ups (not blockers for this gate)

| Issue | Note |
|-------|------|
| `activity_board` dead nav link | Content lives in Day Console; either remove entry or add a route. |
| `RightRail` is a stub | Placeholder "Context" panel only. |
| `BottomStrip` is a stub | Placeholder "Hour Strip" only; no date scrubber. |
| Profile forms not unified | BaziLab does not sync to `userProfile`; EvidenceGraph lacks birth time. |
| `loadToken` cancellation missing on DayConsole / HourStudio | Rapid date changes may cause flicker. |
| No ESLint/Prettier on frontend | `svelte-check` is the only static analysis. |
| `amlich-core` clippy warnings | Tracked by `amlich-081`; isolated via `--no-deps` here. |
