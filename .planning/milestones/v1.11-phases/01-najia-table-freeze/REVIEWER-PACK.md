# v1.11 Tý Ngọ Lưu Chú Point-Opening Context — Reviewer Pack (Najia Table Freeze)

**Scope:** Child bead `amlich-xlag.2.1` — Xu-style Najia (納甲法) point-opening table freeze
**Milestone:** v1.11 Tý Ngọ Lưu Chú Point-Opening Context (Tier 1)
**Authoritative corpus:** `crates/amlich-core/data/ty-ngo-luu-chu/najia-open-points.json` (schema `najia_open_points_v1`)
**Plan:** `.planning/milestones/v1.11-phases/01-najia-table-freeze/01-01-PLAN.md`
**Research base:** `.planning/research/TNLC_POINT_OPENING_RESEARCH.md`
**Decisions in force:** `docs/adr/0003-separate-branch-channel-association-from-ty-ngo-luu-chu.md`, `docs/adr/0004-implement-full-ty-ngo-luu-chu-under-procedural-citation-scope.md`

This pack is the artifact that closes the four human review gates listed in `.planning/milestones/v1.11-REQUIREMENTS.md:91-102`:

- **Gate 1 — classical-Chinese:** §B (the entire frozen table set against the chosen facsimile).
- **Gate 2 — Vietnamese point nomenclature:** §C (huyệt danh set + alphanumeric code glosses).
- **Gate 3 — health-safety:** §D (schema, wording, closed-slot behavior, default-surface exposure).
- **Gate 4 — product/legal:** §E (disclaimer v2, distribution jurisdictions).

Until a gate signs, the affected corpus records stay `ExternalReviewPending` and surfaced content (when the engine bead lands) carries disclaimer v2.

---

## §A — Source-pinned Najia table set

### §A.1 Edition selection and stable URIs

**Primary source under review:** Yang Jizhou 楊繼洲, *Zhenjiu Dacheng* (《針灸大成》), the 徐氏子午流注 corpus: `徐氏子午流注逐日按時定穴歌`, `流注圖`, `十二經納天干歌`, `論子午流注法` (徐氏), `流注時日`, and the `井榮俞原經合橫圖`.

| # | Role | Edition / transcription | Stable URI | Notes |
|---:|---|---|---|---|
| 1 | **Freeze edition (chosen)** | Wikisource transcription of *Zhenjiu Dacheng* 卷七 (traditional-character) | https://zh.wikisource.org/zh-hant/%E9%87%9D%E7%81%B8%E5%A4%A7%E6%88%90/%E5%8D%B7%E4%B8%83 | Reachable, stable, section-anchored. **Volume note:** the research note's "vol. 5" follows the shidianguiji scan pagination (book `NA08718`, unreachable at freeze time; re-checked 2026-09-04). Wikisource arranges the identical 徐氏 corpus in 卷七. Gate 1 confirms or replaces this choice and records `edition_or_facsimile_uri`. |
| 2 | **Cross-reference (originating work)** | Xu Feng 徐鳳, *Zhenjiu Daquan* (《針灸大全》) 卷之五 `論子午流注之法` — CTEXT wiki transcription | https://ctext.org/wiki.pl?chapter=688012&if=en | The *Dacheng* corpus reproduces Xu Feng's tables; used for glyph collation and edition-variant recording. CTEXT facsimile catalog: https://ctext.org/library.pl?if=en&res=2772 |
| 3 | Candidate facsimile (reviewer may substitute) | shidianguiji scan of *Zhenjiu Dacheng* vol. 5 with linked page images | https://www.shidianguiji.com/book/NA08718/chapter/1l0gx6qnyiz2h | Cited by the research note; unreachable at freeze time. Any equivalent facsimile of comparable authority is acceptable with a one-line justification. |
| 4 | Candidate print (reviewer may substitute) | 人民卫生出版社 校释本 (1958/repr. 1987) or 四部叢刊 facsimile | Bibliographic: 徐鳳《針灸大全》人衛 1958; ctext library record 2772 | Standard modern reprints suitable for collation-level review. |

**Translation-kind metadata:** the day-table rows are `verbatim_classical_table_with_project_paraphrase_gloss`. The Chinese verse lines, hour pillars, and 穴名 are transcribed verbatim; only the Vietnamese/English presentation around them is project paraphrase. No modern copyrighted edition has been copied.

### §A.2 Collation record (Wikisource 卷七 freeze text vs CTEXT 針灸大全 卷之五)

All substantive variants the freeze resolved. Each resolution keeps the freeze edition's reading unless a dropped graph required collation (marked ➤):

1. 榮 (freeze ed.) vs 滎 (CTEXT) for the ying point class — edition orthography; both kept per source, corpus uses 榮.
2. 邱墟 vs 丘墟 — edition orthography; corpus uses 邱墟 (freeze ed.).
3. 沖/衝 graph variance (太沖/太衝, 中沖/中衝, 關沖, 衝陽) across sections and editions — corpus records the verse form per row; **Gate 1 must confirm the graph against the facsimile**.
4. 戊日 verse: Wikisource drops one graph before 「陽土穴必還原」; CTEXT prints 「**衝**陽土穴必還原」 ➤ restored 衝陽.
5. 丁日 verse: 「發丑復溜」 (WS) vs 「**癸**丑**複**溜」 (CTEXT) ➤ restored 癸丑復溜.
6. 辛日 verse: WS drops the second graph of 「太□原太淵」; CTEXT prints 「乙未太**衝**原太淵」 ➤ restored 太衝; the WS 流注圖 mis-transcribes the slot as 「己未時」 ➤ collated to 乙未.
7. 壬日 verse: WS 「大腸庚**戍**曲池真」 ➤ collated to 庚戌 from the 流注圖 「庚戌時 大腸合土」.
8. **壬日 table, substantive edition variant:** the *Zhenjiu Daquan* verse omits the 戊申(解谿)/庚戌(曲池)/壬子(關沖) rows; the *Dacheng* verse and 流注圖 include them. The freeze follows *Dacheng* (complete table); the omission is recorded in the corpus, not merged. **Gate 1 adjudicates.**
9. 癸日 己巳 slot: WS 「己**已**商邱」 ➤ collated to 己巳.
10. 乙日 乙未 line: *Dacheng* 「勞宮**火**穴榮」 with figure 「血納包絡之榮**火**」; CTEXT prints 「**水**穴滎」 ➤ freeze keeps 火 (figure + doctrine 木生火). **Gate 1 adjudicates.**
11. 癸日 closing figure prints 「癸木屬水，謂水生木也」 — quoted as printed (doctrine: 癸 water generating the 井木 point 中沖).
12. 丙日 figure 「並過**水**腸原」 — read as 小腸原 (transcription slip); verse confirms 本原腕骨.
13. 壬日 figure 「所過**未**原京骨」 — read as 本原 (transcription slip); verse confirms 返求京骨本原尋.

### §A.3 Resolution conventions pinned by the freeze (TNLC-DIV-03)

- **Hour-branch windows:** Amlich local civil slots, unchanged: 子 23:00–01:00 … 亥 21:00–23:00 (boundary tests exist at 22:59/23:00/00:59/01:00).
- **Day attribution of the 子 block:** the 23:00–01:00 block belongs to the civil date containing its 00:00–01:00 half (classical 子-opens-the-day attribution). For all other hours the cell day stem is the current civil date's day stem. This is the cross-day spillover pin; engine goldens freeze the 甲→乙 and one yin-pair 亥/子 boundary.
- **Hour pillars:** 五鼠遁 (甲己日起甲子), matching the existing Amlich hour-pillar seed mapping.
- **Open rule:** cell (day stem, hour branch) is open iff its hour pillar equals a row pillar of a day-table whose running window covers the cell. Twelve pillars occur in two tables (甲↔庚: 庚辰/壬午/甲申；乙↔辛: 辛卯/癸巳/乙未；丙↔壬: 壬寅/甲辰/丙午；己↔癸: 己巳/辛未/癸酉) and resolve uniquely by cell day stem — no interpolation, no later-school filling (TNLC-DIV-01).
- **Closed doctrine as printed:** 「得時為之開，失時為之闔」(論子午流注法)；「闔者閉也…陽日遇陰時，陰日遇陽時，則前穴已閉」(流注時日). Closed slots serialize an explicit unavailable-by-tradition state, never a fallback point.

### §A.4 Bilingual disclaimer v2 (draft for Gate 4; carried on every surfaced output)

**§A.4.1 Vietnamese:**
> Trích dẫn thuật ngữ y học cổ truyền từ văn bản Châm Cứu Đại Thành; chỉ mang tính văn hóa – lịch sử. Đây không phải hướng dẫn châm, bấm, cứu hay tự điều trị tại bất kỳ thời điểm nào. Không dùng để trì hoãn hoặc thay thế chăm sóc từ nhân viên y tế có chuyên môn.

**§A.4.2 English:**
> Citations of classical acupuncture terminology from Zhenjiu Dacheng; provided as historical and cultural information only. This is not instruction or encouragement to needle, press, moxibust, or self-treat at any time. Do not use it to delay or replace care from a qualified health professional.

**§A.4.3 Stable identifier (proposed):** `historical_procedural_citation_v1` — safety classification `historical_procedural_citation` (BOUND-02). The engine bead will byte-lock the strings; this pack is their source of truth until then.

### §A.5 The ten day-tables as frozen

Each row cites the verse line (歌訣行) and the 流注圖 fragment. 「解析格」 shows the civil day × hour-branch cell the row resolves to under §A.3 (spillover rows land on the next civil day).

<!-- DIGEST:BEGIN (generated from the frozen corpus; do not hand-edit) -->
#### 甲日 — 足少陽膽（甲主 與己合 膽引氣行）

| # | 時柱 | 五輸註（如刊） | 穴（角色） | 替代/納穴 | 歌訣行 | 解析格 |
|---:|---|---|---|---|---|---|
| 1 | 甲戌 | 井金 | 竅陰 | — | 甲日戌時膽竅陰 | 甲日戌時 |
| 2 | 丙子 | 榮水 | 前谷 | — | 丙子時中前谷榮 | 乙日子時（spillover） |
| 3 | 戊寅 | 俞木 | 陷谷、邱墟（並過原） | 返本還原 | 戊寅陷谷陽明俞，返本邱墟木在寅 | 乙日寅時（spillover） |
| 4 | 庚辰 | 經火 | 陽谿 | — | 庚辰經注陽谿穴 | 乙日辰時（spillover） |
| 5 | 壬午 | 合土 | 委中 | — | 壬午膀胱委中尋 | 乙日午時（spillover） |
| 6 | 甲申 | 榮水 | 液門 | 氣納三焦 | 甲申時納三焦水，榮合天干取液門 | 乙日申時（spillover） |

#### 乙日 — 足厥陰肝（乙主 與庚合 肝引血行）

| # | 時柱 | 五輸註（如刊） | 穴（角色） | 替代/納穴 | 歌訣行 | 解析格 |
|---:|---|---|---|---|---|---|
| 1 | 乙酉 | 井木 | 大敦 | — | 乙日酉時肝大敦 | 乙日酉時 |
| 2 | 丁亥 | 榮火 | 少府 | — | 丁亥時榮少府心 | 乙日亥時 |
| 3 | 己丑 | 俞土 | 太白、太衝（並過原） | 返本還原 | 己丑太白太衝穴 | 丙日丑時（spillover） |
| 4 | 辛卯 | 經金 | 經渠 | — | 辛卯經渠是肺經 | 丙日卯時（spillover） |
| 5 | 癸巳 | 合水 | 陰谷 | — | 癸巳腎宮陰谷合 | 丙日巳時（spillover） |
| 6 | 乙未 | 榮火 | 勞宮 | 血納包絡 | 乙未勞宮火穴榮 | 丙日未時（spillover） |

#### 丙日 — 手太陽小腸（丙主 與辛合 小腸引氣行）

| # | 時柱 | 五輸註（如刊） | 穴（角色） | 替代/納穴 | 歌訣行 | 解析格 |
|---:|---|---|---|---|---|---|
| 1 | 丙申 | 井金 | 少澤 | — | 丙日申時少澤當 | 丙日申時 |
| 2 | 戊戌 | 榮水 | 內庭 | — | 戊戌內庭治脹康 | 丙日戌時 |
| 3 | 庚子 | 俞木 | 三間、腕骨（並過原） | 返本還原 | 庚子時在三間俞，本原腕骨可祛黃 | 丁日子時（spillover） |
| 4 | 壬寅 | 經火 | 崑崙 | — | 壬寅經火崑崙上 | 丁日寅時（spillover） |
| 5 | 甲辰 | 合土 | 陽陵泉 | — | 甲辰陽陵泉合長 | 丁日辰時（spillover） |
| 6 | 丙午 | 俞木 | 中渚 | 氣納三焦 | 丙午時受三焦木，中渚之中仔細詳 | 丁日午時（spillover） |

#### 丁日 — 手少陰心（丁主 與壬合 心引血行）

| # | 時柱 | 五輸註（如刊） | 穴（角色） | 替代/納穴 | 歌訣行 | 解析格 |
|---:|---|---|---|---|---|---|
| 1 | 丁未 | 井木 | 少沖 | — | 丁日未時心少沖 | 丁日未時 |
| 2 | 己酉 | 榮火 | 大都 | — | 己酉大都脾土逢 | 丁日酉時 |
| 3 | 辛亥 | 俞土 | 太淵、神門（並過原） | 返本還原 | 辛亥太淵神門穴 | 丁日亥時 |
| 4 | 癸丑 | 經金 | 復溜 | — | 癸丑復溜腎水通（一作「發丑復溜」） | 戊日丑時（spillover） |
| 5 | 乙卯 | 合水 | 曲泉 | — | 乙卯肝經曲泉合 | 戊日卯時（spillover） |
| 6 | 丁巳 | 俞土 | 大陵 | 血納包絡 | 丁巳包絡大陵中（一作「丁已」） | 戊日巳時（spillover） |

#### 戊日 — 足陽明胃（戊主 與癸合 胃引氣行）

| # | 時柱 | 五輸註（如刊） | 穴（角色） | 替代/納穴 | 歌訣行 | 解析格 |
|---:|---|---|---|---|---|---|
| 1 | 戊午 | 井金 | 厲兌 | — | 戊日午時厲兌先 | 戊日午時 |
| 2 | 庚申 | 榮水 | 二間 | — | 庚申榮穴二間遷 | 戊日申時 |
| 3 | 壬戌 | 俞木 | 束骨、衝陽（並過原） | 返本還原 | 壬戌膀胱尋束骨，衝陽土穴必還原（「衝」字據針灸大全補） | 戊日戌時 |
| 4 | 甲子 | 經火 | 陽輔 | — | 甲子膽經陽輔是 | 己日子時（spillover） |
| 5 | 丙寅 | 合土 | 小海 | — | 丙寅小海穴安然 | 己日寅時（spillover） |
| 6 | 戊辰 | 經火 | 支溝 | 氣納三焦 | 戊辰氣納三焦脈，經穴支溝刺必痊 | 己日辰時（spillover） |

#### 己日 — 足太陰脾（己主 與甲合 脾引血行）

| # | 時柱 | 五輸註（如刊） | 穴（角色） | 替代/納穴 | 歌訣行 | 解析格 |
|---:|---|---|---|---|---|---|
| 1 | 己巳 | 井木 | 隱白 | — | 己日巳時隱白始 | 己日巳時 |
| 2 | 辛未 | 榮火 | 魚際 | — | 辛未時中魚際取 | 己日未時 |
| 3 | 癸酉 | 俞土 | 太谿、太白（並過原） | 返本還原 | 癸酉太谿太白原 | 己日酉時 |
| 4 | 乙亥 | 經金 | 中封 | — | 乙亥中封內踝比 | 己日亥時 |
| 5 | 丁丑 | 合水 | 少海 | — | 丁丑時合少海心 | 庚日丑時（spillover） |
| 6 | 己卯 | 經金 | 間使 | 血納包絡 | 己卯間使包絡止 | 庚日卯時（spillover） |

#### 庚日 — 手陽明大腸（庚主 與乙合 大腸引氣行）

| # | 時柱 | 五輸註（如刊） | 穴（角色） | 替代/納穴 | 歌訣行 | 解析格 |
|---:|---|---|---|---|---|---|
| 1 | 庚辰 | 井金 | 商陽 | — | 庚日辰時商陽居 | 庚日辰時 |
| 2 | 壬午 | 榮水 | 通谷 | — | 壬午膀胱通谷之 | 庚日午時 |
| 3 | 甲申 | 俞木 | 臨泣、合谷（並過原） | 返本還原 | 甲申臨泣為俞木，合谷金原返本歸 | 庚日申時 |
| 4 | 丙戌 | 經火 | 陽谷 | — | 丙戌小腸陽谷火 | 庚日戌時 |
| 5 | 戊子 | 合土 | 三里 | — | 戊子時居三里宜 | 辛日子時（spillover） |
| 6 | 庚寅 | 合土 | 天井 | 氣納三焦 | 庚寅氣納三焦合，天井之中不用疑 | 辛日寅時（spillover） |

#### 辛日 — 手太陰肺（辛主 與丙合 肺引血行）

| # | 時柱 | 五輸註（如刊） | 穴（角色） | 替代/納穴 | 歌訣行 | 解析格 |
|---:|---|---|---|---|---|---|
| 1 | 辛卯 | 井木 | 少商 | — | 辛日卯時少商木 | 辛日卯時 |
| 2 | 癸巳 | 榮火 | 然谷 | — | 癸巳然谷何須忖 | 辛日巳時 |
| 3 | 乙未 | 俞土 | 太衝、太淵（並過原） | 返本還原 | 乙未太衝原太淵（「衝」字據針灸大全補；流注圖誤作「己未時」） | 辛日未時 |
| 4 | 丁酉 | 經金 | 靈道 | — | 丁酉心經靈道引 | 辛日酉時 |
| 5 | 己亥 | 合水 | 陰陵泉 | — | 己亥脾合陰陵泉 | 辛日亥時 |
| 6 | 辛丑 | 合水 | 曲澤 | 血納包絡 | 辛丑曲澤包絡准 | 壬日丑時（spillover） |

#### 壬日 — 足太陽膀胱（壬主 與丁合 膀胱引氣行）

| # | 時柱 | 五輸註（如刊） | 穴（角色） | 替代/納穴 | 歌訣行 | 解析格 |
|---:|---|---|---|---|---|---|
| 1 | 壬寅 | 井金 | 至陰 | — | 壬日寅時起至陰 | 壬日寅時 |
| 2 | 甲辰 | 榮水 | 俠谿 | — | 甲辰膽脈俠谿榮 | 壬日辰時 |
| 3 | 丙午 | 俞木 | 後谿、京骨（並過原）、陽池（兼過三焦原） | 返本還原＋兼過三焦原 | 丙午小腸後谿俞，返求京骨本原尋，三焦寄有陽池穴，返本還原似的親 | 壬日午時 |
| 4 | 戊申 | 經火 | 解谿 | — | 戊申時註解谿胃 | 壬日申時 |
| 5 | 庚戌 | 合土 | 曲池 | — | 大腸庚戌曲池真（一作「庚戍」） | 壬日戌時 |
| 6 | 壬子 | 井金 | 關沖 | 氣納三焦 | 壬子氣納三焦寄，井穴關沖一片金，關沖屬金壬屬水，子母相生恩義深 | 癸日子時（spillover） |

#### 癸日 — 足少陰腎（癸主 與戊合 腎引血行）

| # | 時柱 | 五輸註（如刊） | 穴（角色） | 替代/納穴 | 歌訣行 | 解析格 |
|---:|---|---|---|---|---|---|
| 1 | 癸亥 | 井木 | 湧泉 | — | 癸日亥時井湧泉 | 癸日亥時 |
| 2 | 乙丑 | 榮火 | 行間 | — | 乙丑行間穴必然 | 甲日丑時（spillover） |
| 3 | 丁卯 | 俞土 | 神門、太谿（並過原）、大陵（又過包絡原） | 返本還原＋又過包絡原 | 丁卯俞穴神門是，本尋腎水太谿原，包絡大陵原井過 | 甲日卯時（spillover） |
| 4 | 己巳 | 經金 | 商邱 | — | 己巳商邱內踝邊（一作「己已」） | 甲日巳時（spillover） |
| 5 | 辛未 | 合水 | 尺澤 | — | 辛未肺經合尺澤 | 甲日未時（spillover） |
| 6 | 癸酉 | 井木 | 中沖 | 血納包絡 | 癸酉中沖包絡連 | 甲日酉時（spillover） |

Verse closing line as printed: 「子午截時安定穴，留傳後學莫忘言。」
<!-- DIGEST:END -->

### §A.6 Closed (閉穴) inventory — the explicit 120-slot grid

Each day stem × hour branch cell is either open (resolving to exactly one frozen row above) or explicitly closed. Closed cells serialize the doctrine quote and the two running tables, never a fallback point (TNLC-DIV-01).

- 甲日: 開 丑 卯 巳 未 酉 戌；閉 子 寅 辰 午 申 亥
- 乙日: 開 子 寅 辰 午 申 酉 亥；閉 丑 卯 巳 未 戌
- 丙日: 開 丑 卯 巳 未 申 戌；閉 子 寅 辰 午 酉 亥
- 丁日: 開 子 寅 辰 午 未 酉 亥；閉 丑 卯 巳 申 戌
- 戊日: 開 丑 卯 巳 午 申 戌；閉 子 寅 辰 未 酉 亥
- 己日: 開 子 寅 辰 巳 未 酉 亥；閉 丑 卯 午 申 戌
- 庚日: 開 丑 卯 辰 午 申 戌；閉 子 寅 巳 未 酉 亥
- 辛日: 開 子 寅 卯 巳 未 酉 亥；閉 丑 辰 午 申 戌
- 壬日: 開 丑 寅 辰 午 申 戌；閉 子 卯 巳 未 酉 亥
- 癸日: 開 子 亥；閉 丑 寅 卯 辰 巳 午 未 申 酉 戌

**Note for reviewers:** the 癸-day gap (only the spillover 壬子 and the opening 癸亥 are open) is the largest gap in the Xu tables as printed and is the slot later schools filled with 返本還原-type substitution rules. Those schools are recorded as divergence (TNLC-DIV-01) and are **never** merged into this corpus.

### §A.7 氣納三焦 / 血納包絡 rows (TNLC-DIV-02)

As printed: yang-stem tables close with 「氣納三焦」 rows (甲申液門榮水，丙午中渚俞木，戊辰支溝經火，庚寅天井合土，壬子關沖井金) and yin-stem tables close with 「血納包絡」 rows (乙未勞宮榮火，丁巳大陵俞土，己卯間使經金，辛丑曲澤合水，癸酉中沖井木), each with the printed generation rationale (子母相生). 三焦/心包 remain traditional identities (LH-DIV-06 alignment); no biomedical conversion.

### §A.8 Divergence markers attached to every row

- **TNLC-DIV-01** — closed slots stay closed; later filling schools are divergence, never merged.
- **TNLC-DIV-02** — 氣納三焦/血納包絡 encoded as printed; traditional identities preserved.
- **TNLC-DIV-03** — classical timekeeping vs modern civil time: pinned conventions in §A.3, disclosed `time_basis`.
- **TNLC-DIV-05** — the method is contested in the classical corpus itself (e.g. *Zhenjiu Fengyuan* vol. 4 critiques one-channel-per-block readings); a point-opening citation is never a physiological or efficacy claim.
- **TNLC-DIV-04** — applies to the nomenclature registry (§C), not the classical rows.

---

## §B — Classical-Chinese reviewer sign-off (Gate 1)

I, the undersigned, have reviewed the ten day-tables in §A.5, the closed-slot inventory in §A.6, the substitution rows in §A.7, and the collation record in §A.2 against the chosen facsimile of the 徐氏子午流注 corpus (*Zhenjiu Dacheng*; Wikisource 卷七 by default, or an alternative facsimile of equivalent authority), and I confirm:

- [ ] All 60 table rows (hour pillar, slot class, point identity 穴名 exactly as printed) are faithful to the chosen facsimile; no row is sourced from memory or a modern chart.
- [ ] The 沖/衝, 榮/滎, 邱/丘 graph choices recorded in §A.2 match the facsimile or are marked `corrected` with the canonical reading proposed.
- [ ] The collation restorations in §A.2 items 4–13 are correct (or marked `corrected`).
- [ ] The 壬日 edition variant (§A.2 item 8) is correctly frozen per *Zhenjiu Dacheng*.
- [ ] The closed-slot inventory and the 氣納三焦/血納包絡 rows match the facsimile's tables and figure.
- [ ] The resolution conventions in §A.3 (五鼠遁 pillars, 子-block day attribution, running-window rule) are a faithful presentation of the tables as printed.
- [ ] I have recorded the `edition_or_facsimile_uri` actually consulted in the corpus JSON (replacing `PENDING_CLASSICAL_REVIEW` on every row).

**Reviewer role:** classical_chinese_reviewer
**Name / signature:**
**Date (YYYY-MM-DD):**
**Edition consulted (URI or bibliographic reference):**

---

## §C — Vietnamese point-nomenclature reviewer sign-off (Gate 2)

The corpus `point_nomenclature_registry` proposes, for each of the 66 point identities used by the frozen tables, a triple: the classical Chinese 穴名 exactly as printed, a Vietnamese huyệt danh (Sino-Vietnamese reading), and a standard alphanumeric code as lookup gloss (WHO/WFAS-style). All Vietnamese names and codes are drafts (`*_draft_gate2_pending` fields) until this gate signs. The complete table is `crates/amlich-core/data/ty-ngo-luu-chu/najia-open-points.json` `point_nomenclature_registry`; sample rows:

| point_key | 穴名 (as printed) | huyệt danh (draft) | code gloss (draft) | kinh (zh) | kinh (vi) |
|---|---|---|---|---|---|
| qiao-yin | 竅陰 | Kiếu âm | GB44 | 足少陽膽 | Đởm |
| tai-chong | 太衝 | Thái xung | LR3 | 足厥陰肝 | Can |
| shang-khau | 商邱 | Thương khâu | SP5 | 足太陰脾 | Tỳ |
| tam-ly | 三里 | Tam lý | ST36 | 足陽明胃 | Vị |

I, the undersigned, confirm:

- [ ] The Vietnamese huyệt danh set is one consistent, school-appropriate nomenclature for the printed 穴名 forms (variants such as 商邱/商丘, 邱墟/丘墟 are recorded, with the printed form preserved as primary).
- [ ] The alphanumeric code glosses are correct for the printed point identities (note 三里 = 足三里 ST36; 商邱 = 商丘 SP5).
- [ ] Codes are presented as standard lookup glosses only — never as WHO endorsement of efficacy, and never annotated with function, effect, or biomedical claims (TNLC-DIV-04, per WHO ICD-11 TM FAQ positioning).
- [ ] The five-shu class Vietnamese terms used by the engine (Tỉnh/Vinh/Du/Nguyên/Kinh/Hợp) match the printed 井/榮(滎)/俞/原/經/合.

**Reviewer role:** vietnamese_nomenclature_reviewer
**Name / signature:**
**Date (YYYY-MM-DD):**
**Nomenclature school / reference consulted:**

---

## §D — Health-safety reviewer sign-off (Gate 3)

I, the undersigned, have reviewed the corpus schema, wording contracts, and the planned default-surface exposure (owner decision 2026-08-21), and I confirm:

- [ ] The bilingual disclaimer v2 (§A.4) cannot be read as instruction or encouragement to needle, press, moxibust, or self-treat, and instructs the reader not to delay or replace qualified professional care.
- [ ] The corpus contains no technique, depth, manipulation, dose, procedural, or efficacy field; the CI guard `najia_open_points_corpus_guard.rs::corpus_carries_no_clinical_or_technique_field_names` enforces the lexical boundary, and the future engine DTO guard (bead `.2.2`) extends it.
- [ ] Closed (閉穴) slots serialize an explicit unavailable-by-tradition state and are never converted to an adjacent point, a substitution from a later school, or a recommendation.
- [ ] An open-point citation is always framed as historical citation ("theo bài ca/bảng, giờ X ngày Y được ghi tương ứng với huyệt Z"), never as an action recommendation, "best time to treat", or "đang mở → hãy can thiệp".
- [ ] The default-surface exposure decision (TUI + desktop inspector, no opt-in) with disclaimer v2 and visible review state does not make the product a point-selection aid; if you judge it does, record the required mitigation here instead of signing.

**Reviewer role:** health_safety_reviewer
**Name / signature:**
**Date (YYYY-MM-DD):**
**Additional concerns (optional):**

---

## §E — Product/Legal reviewer sign-off (Gate 4)

I, the undersigned, have reviewed disclaimer v2 (§A.4) and the distribution framing for the intended jurisdictions, and I confirm:

- [ ] The Vietnamese text is appropriate for distribution to Vietnamese-speaking users.
- [ ] The English text is appropriate for distribution to English-speaking users.
- [ ] The stable identifier `historical_procedural_citation_v1` is the contract clients must honor when rendering (byte-locked strings once the engine lands).
- [ ] Point names, open/closed states, and citation framing do not constitute medical advice, and the disclaimer distinguishes historical-cultural citation from treatment instruction.
- [ ] No regulatory determination of Vietnamese law is implied; the FDA general-wellness boundary is a vocabulary model only, not legal advice.
- [ ] The intended distribution jurisdictions are recorded below and re-reviewed if they change.

**Reviewer role:** product_legal_reviewer
**Name / signature:**
**Date (YYYY-MM-DD):**
**Jurisdictions reviewed:**

---

## §F — Boundaries enforced by the implementation (read-only context for reviewers)

- **No birth / medical data.** The resolver (bead `.2.2`) will consume local civil date/time and existing day-pillar facts only — no `BirthInput`, sex/gender, symptom, location, or health history.
- **ADR-0003 source separation.** The v1.10 Tier-0 corpus never cites `ty-ngo-luu-chu` and this corpus never cites `shi-er-jing-na-di-zhi`; a CI guard keeps the id out of `crates/amlich-core/src/` until the v1.11 engine bead performs its first, policy-contracted emission.
- **Data-side guard now in force.** `crates/amlich-core/tests/najia_open_points_corpus_guard.rs` re-derives the 120-cell grid independently from the frozen tables (五鼠遁 + window rule), checks the 60/60 open/closed split, the row↔cell bijection, the spillover pins (乙日子 = 前谷, 癸日子 = 關沖, 癸日 gap), pending reviewer literals, and the clinical-lexeme boundary.
- **No coupling.** Day Assessment, Hour Ranking, Direction Assessment, and v1.10 Traditional Wellness Context are untouched by this freeze; the future DTO field is additive and separate.
- **Non-goals stay non-goals.** 靈龜八法/飛騰八法 and 養子時刻注穴法 appear only in divergence notes, never in tables.

---

## §G — Packet history (filled in by the bead owner, not the reviewer)

- **Packet version:** v1
- **Packet author:** implementation owner of `amlich-xlag.2.1`
- **Packet date:** 2026-09-04
- **Linked bead:** `amlich-xlag.2.1`
- **Linked plan:** `.planning/milestones/v1.11-phases/01-najia-table-freeze/01-01-PLAN.md`
- **Linked research:** `.planning/research/TNLC_POINT_OPENING_RESEARCH.md` (§2 divergence ledger, §3 disclaimer draft, §5 gates, §7 verification ledger)
- **Linked ADRs:** `docs/adr/0003-…`, `docs/adr/0004-…`
- **Linked external-review policy:** `docs/architecture/external-review-lifecycle.md` (Active Register rows added by this bead)
- **Resulting Active Register rows:** v1.11 Najia corpus freeze; disclaimer v2 draft.

The implementation owner will not flip any corpus record from `ExternalReviewPending` to `Signed` until §§B, C, D, and E are each signed by a named reviewer. Sign-off identity, date, and source are recorded as bead comments at close time.

---

## §H — Reviewer outreach (for the bead owner / coordinator)

Mirrors the v1.10 `REVIEWER-COORDINATION.md` protocol:

1. Send the full text of this pack, pointing the reviewer at §B / §C / §D / §E as appropriate, with the return format below and the deadline (2026-12-31) and compensation arrangement.
2. Suggested subject: `Amlich v1.11 Tý Ngọ Lưu Chú — external review request (Gate {1|2|3|4}, role: {classical_chinese_reviewer|vietnamese_nomenclature_reviewer|health_safety_reviewer|product_legal_reviewer})`.
3. The reviewer edits this pack in place (ticks boxes, fills name/date/edition) and returns it as a GitHub PR adding the signed pack under `docs/reviews/v1.11/` (preferred) or as an attachment the coordinator commits there.
4. After sign-off the implementation owner: replaces `PENDING_CLASSICAL_REVIEW` with the recorded `edition_or_facsimile_uri` on every row, replaces the `ExternalReviewPending(...)` literals with `Signed(reviewer=…, date=…, source_uri=…)`, updates the Active Register, re-runs `cargo test -p amlich-core --test najia_open_points_corpus_guard`, and comments on the gate bead (`amlich-xlag.2.5`–`.2.8`).
5. This bead (`amlich-xlag.2.1`) closes only when its own acceptance criteria hold; the freeze records themselves remain unsigned until Gate 1 signs, which is an expected, disclosed state — not a blocker for the engine bead to consume the frozen tables as `ExternalReviewPending` data.
