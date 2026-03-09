# Engineer-Ready Framework for Converting Vietnamese–Chinese Almanac Signals into Daily Guidance

## Executive summary

A credible “nên làm / không nên làm / nên tránh / kiêng kỵ” engine is not produced by **one** label (e.g., hoàng đạo day) but by a **layered hemerology system** whose core artifacts are *thông thư / 通書* (“tongshu”-style almanacs) that map **signals → activities** and provide **precedence rules** for conflicts. Vietnamese practice includes both (a) Vietnamese-market “Thông thư / Hoàng lịch” compilations that name many *sao tốt/xấu* and taboo-day sets, and (b) Vietnam-held Hán-script tongshu texts (e.g., “Ngọc hạp / 玉匣” lineages) that show direct continuity with Chinese selection traditions preserved in Vietnam. citeturn18view0turn18view1turn17view0turn21view1turn21view2

For normative, rule-structured logic, the Qing court’s **欽定協紀辨方書** (Qinding Xieji Bianfang Shu, completed 1739) is the best backbone because it explicitly: (1) enumerates **signals (吉神/凶煞, 建除/2-road, 月建/月破, special taboo days)**, (2) defines a large **activity catalog (用事)**, and (3) provides explicit **annotation / conflict-resolution rules (鋪註條例)** such as “good outweighs bad → follow 宜,” while preserving non-overridable taboos (“德猶忌”). citeturn22search6turn8view0turn10view2turn7view0

**Key engineering implication:** your current engine computes many “surface” signals (Trực, Can–Chi, 28 mansions, tiết khí, hoàng đạo/hắc đạo). To match real tongshu decision-making, you will likely need to add a **minimal star-set layer** (a curated subset of 吉神/凶煞) and a **conflict engine** that follows Xieji’s precedence rules, plus clear labeling when output comes from “folk compilation layers” (e.g., 玉匣記’s Yang Gong taboo days, monthly 5/14/23 taboo). citeturn10view2turn12view3turn13view0turn7view0

## A. Source map

The table below is biased toward sources that (a) are historically grounded, (b) enumerate signals explicitly, and (c) can be converted into implementable rule tables.

| Source | What it is | Tradition type | Why it is influential / trustworthy | Signals it explicitly covers (examples) | Practical value for software rules |
|---|---|---|---|---|---|
| **“THÔNG THƯ” (Vietnamese)** (PDF scan) citeturn20view0turn21view1turn21view2turn21view3turn21view5 | Vietnamese-language “Thông thư” style manual with explicit lists of *các sao tốt* (what they support) and *các ngày xấu* (what they forbid), plus hoàng đạo/hắc đạo and taboo-day content | Modern popular compilation reflecting folk/commercial almanac usage | High UX relevance for Vietnam because it expresses “sao → việc” in plain Vietnamese (e.g., lists of “Nên…” and “Kiêng…”). Its hosting site is not a scholarly institution, so treat as a **practice snapshot**, not a normative authority | Sao tốt/xấu lists; hoàng đạo/hắc đạo by lunar month branches; “Nguyệt đức”; “Thập bát tú / 28 sao”; “Tam nương” taboo poem citeturn21view1turn21view2turn21view3turn21view4turn21view5 | Great for (1) building Vietnam-facing activity vocabulary and (2) confirming which dimensions users expect. Use cautiously for precedence; validate against Xieji/Yuxia when possible |
| **增補選擇通書廣玉匣記** (Tăng bổ tuyển trạch thông thư quảng ngọc hạp ký), NLV woodblock, **1920**, held by National Library of Vietnam citeturn18view0 | Vietnam-held Hán-script tongshu (“Ngọc hạp / 玉匣”) text, woodblock printed | Late-imperial / early-modern Vietnamese tongshu transmission (Hán script) | Digitized from the National Library of Vietnam; strong evidence of Vietnam’s direct adoption of tongshu selection logic (even when the text body is not OCR-extractable here) citeturn18view0turn17view0 | Likely includes core tongshu layers (stars, day selection for activities), but page content is image-based in this interface | Best used as provenance/trust anchor for “Vietnamese traditional sources” and terminology alignment; use Chinese OCR transcriptions (Xieji/Yuxia) for implementable tables unless you digitize this corpus |
| **玉匣攢要通用** (Ngọc hạp toản yếu thông dụng), NLV woodblock, **1926** citeturn18view2 | Vietnam-held Hán-script “Ngọc hạp” derivative | Late-imperial/early-modern Vietnamese tongshu transmission | National Library of Vietnam provenance via Nôm Preservation Foundation | Signals not OCR-visible here | Same role as above: provenance + mapping evidence |
| **玉匣纂要通用** (Ngọc hạp toản yếu thông dụng), Chùa Phổ Nhân collection citeturn18view1 | Vietnam-held Hán-script almanac-like manual described as covering “xem tuổi, xem ngày… ngày nào nên đi buôn… cưới xin…” | Vietnamese temple-held folk-practice text (Hán script) | Description explicitly states it covers day-selection for trade, marriage, etc.; referenced in the **Di sản Hán Nôm Việt Nam** bibliographic program | “Nên đi buôn / xuất quân / cưới xin” etc (in description) citeturn18view1turn17view0 | Confirms that “daily guidance” for practical activities is a core feature in Vietnamese Hán-script tradition |
| **Di sản Hán Nôm Việt Nam – Thư mục đề yếu** (Hán Nôm Institute / EFEO project) citeturn17view0turn18view1 | Scholarly bibliographic program cataloging Vietnam’s Hán–Nôm corpus | Academic reference infrastructure | Authoritative institutional framing: describes the project’s international scholarly value and large-scale cataloging of Hán–Nôm materials | Not a ruleset itself; helps validate provenance and classification of Vietnamese texts citeturn17view0 | Use as a trust layer to justify which Vietnam-held texts you treat as “traditional sources” |
| **欽定協紀辨方書** (Qinding Xieji Bianfang Shu, **1739**) citeturn22search6turn22search0turn8view0turn10view2 | Qing court-compilated, systematic date-selection compendium | Classical / state-standard doctrinal base for selection | Compiled under imperial order and explicitly aims to correct contradictions and weigh forces (力) rather than blanket-tabooing; widely treated as a standard reference, including cross-cultural adoption (Joseon exam textbook) citeturn22search6turn22search0turn8view0 | Enumerates “宜忌” signal sets, taboo-day families, and a “鋪註條例” conflict system; includes “用事” activity catalog with per-activity 宜/忌 lists citeturn10view2turn7view0 | **Primary normative backbone** for implementable rule tables and precedence rules |
| **玉匣記** (Yuxia Ji / 玉匣記通書) citeturn11view0turn12view0turn13view0turn12view3 | Widely circulating compilation tongshu with many folk layers | Mixed: compilation + folk accretion + practice mnemonics | Useful because it contains many explicit “if X then avoid Y” mnemonics and taboo-day sets that are widely recognized in Vietnam/East Asia, but it is explicitly described as a compilation with pseudo-attributions and many versions citeturn13view0 | 28 mansions poem mapping; Pengzu taboos; Yang Gong taboo days; monthly 5/14/23 taboo; hoàng đạo/hắc đạo rules and “remedy” lore; six clashes; “用日法” strength by month command citeturn12view0turn13view0turn12view3turn16view0turn15view0 | Best as an **optional ruleset pack** (“folk tongshu layer”) that you can enable with transparent labeling |
| Scholarly: Richard J. Smith on continuity from early **日書** (daybooks) to **通書/黃曆** almanacs citeturn23search0 | Academic history of hemerology and almanacs | Scholarship | Supports that “daily do/don’t” guidance is historically continuous and not merely modern SEO content | Conceptual framing; not a rule table citeturn23search0 | Use for documentation, not for rule extraction |
| Scholarly: “Time as Norm: The Ritual Dimension of the Calendar Book…” (JSTOR OA chapter) citeturn23search3 | Academic analysis of Qing calendar books as normative ritual/behavior manuals | Scholarship | Useful to justify a “normative manual” model and layered integration of astronomy + numerology | Conceptual framing | Good for product narrative and “uncertainty labeling” principles |

## B. Signal-by-signal analysis

The table focuses on signals you already compute plus the **minimum additional dimensions** that Xieji/Yuxia treat as essential for serious activity recommendations.

| Signal (VI) | Signal (ZH + characters) | English gloss | Meaning in tradition | How it drives actions | Example activities influenced | Typical decision weight | Caveats / variant traditions |
|---|---|---|---|---|---|---|---|
| **Trực** (Kiến/Trừ/Mãn/…/Bế) | 建除十二神 / 十二建星 | Twelve Day Officers | Recurring cycle used to judge what the day “is good for” | Used both as (a) direct “activity fit” and (b) part of hoàng/hắc classification in some tongshu traditions | Xieji explicitly assigns “入學 宜成日開日,” “冠帶 宜定日,” etc. citeturn7view0 | Strong | Different tongshu versions attach different mnemonics; use Xieji “用事” as canonical activity mapping and Yuxia as folk mnemonic pack citeturn7view0turn13view0 |
| **Hoàng đạo / Hắc đạo** | 黃道 / 黑道 | Yellow-road / black-road days (and sometimes hours) | Broad auspicious/inauspicious governance label, often derived from day officer sets | Commonly used as a day-level label; in Yuxia, explicit mapping “建滿平收黑…成開皆可用…” | Good for “general vibe” label; can gate low-stakes vs high-stakes actions | Medium | Yuxia includes mitigation lore: even if black-road, marriage can be “resolved” by wearing yellow shoes; treat this as optional folk practice, not a universal override citeturn16view0 |
| **Can–Chi (day/month/year)** | 天干地支 | Sexagenary cycle markers | Base indices that generate many derived rules (taboos, clashes, month build/break, etc.) | Drives table-based taboos like Pengzu, computes clashes and combinations | Contracts, travel, surgery/treatment, marriage, etc. | Strong | Many rules are explicit mnemonics (table-driven), not purely algorithmic; store as curated rule tables with source tags citeturn13view0turn10view2 |
| **Pengzu taboos** (*Bách kỵ* by stem/branch) | 彭祖百忌日 | “Pengzu hundred taboos” | Stem-based and branch-based “do not do X” mappings | Provides hard “avoid” constraints (e.g., 甲 day avoid opening storehouse; 亥 day avoid marriage) | Commerce (storehouse), agriculture (planting), grooming, medical (服藥), marriage, guests/banquets | Medium–Strong | Exists as a folk-compilation layer; Xieji also lists “百忌日” as a taboo family, but implementation should allow “strict vs soft” mode citeturn13view0turn10view2 |
| **Xung / clash** | 沖 / 六沖 | Clash (esp. six clashes) | Day clashes month or “year lord” → inauspicious | Used as a strong negative for major actions; Yuxia states day–month or day–year clash is凶 | Moving, marriage, construction, high-stakes travel | Strong | Needs clear scope: Yuxia frames as “日與月沖/與歲君沖” (date-wide constraint). Personalized “xung tuổi” is another layer you may add separately citeturn15view0 |
| **Hợp / combine** | 合 (三合/六合/五合) | Harmonious combinations | Structural “compatibility” positives | Used as positive modifiers for social/contract actions; Yuxia states “三合、六合皆吉” for marriage selection | Marriage, contracts, social meetings, starting business | Medium–Strong | Don’t let combines override Xieji “德猶忌” vetoes; treat as “upgrade” rather than “veto breaker” citeturn14view2turn10view2 |
| **Hình / hại / phá / tuyệt** (your relationship set) | 刑 / 害 / 破 / 絕 (varies) | Punish / harm / break / sever | Compatibility-negative families used variably across tongshu | Often used to downgrade or prohibit when tied to a specific activity/person profile | Marriage matching, litigation, travel | Medium | Yuxia explicitly covers “相穿” (穿 / “lục hại”–type) with marriage example; broader刑害破绝 need careful sourcing before treating as hard vetoes citeturn14view2 |
| **Tiết khí** | 節氣 / 二十四節氣 | Solar terms | Seasonal nodes; also define special taboo constructs (四離四絕) | Xieji uses节气-derived taboo days as near-global constraints; also flags solstices/equinoxes as days whose “major actions” are not annotated even if otherwise auspicious | Marriage/major state-like actions; also agriculture (seasonal tasks) | Strong when triggering special taboo days; otherwise low–medium | Keep two layers: (1) strict “special taboo days” (四離四絕, etc.), (2) seasonal task suggestions (agriculture) citeturn10view2turn15view0 |
| **Nhị thập bát tú** | 二十八宿 | 28 lunar mansions | Mansion “in charge” of the day; many mansions have strong pro/contra statements per activity | Yuxia provides explicit mansion-by-mansion poems tied to building, marriage, burial, litigation | Construction, burial, marriage, travel, legal disputes | Strong (activity-specific veto in some traditions) | High variance across versions; treat as versioned “tongshu pack.” Vietnamese “Thông thư” also includes “Thập bát tú / 28 sao dùng xem ngày” sections, showing local expectation citeturn12view0turn21view4 |
| **Hoàng đạo / hắc đạo as hours** | 黃道/黑道 + 六神 (青龍…司命) | Auspicious hours | Many almanacs apply “yellow/black road” at hour level too | Supports “time-of-day” recommendations (optional feature) | Departing travel hour, signing hour | Medium | Out of scope if your product is day-only; but Yuxia includes explicit hour mapping and “six auspicious hours” list citeturn16view2 |
| **Good stars** (*cát tinh / thần sát*) | 吉神 (天德/月德/天赦/天願/… ) | Auspicious “stars” (mantic indicators) | Core levers for most “宜” judgments in Xieji | Xieji’s “宜忌” section lists these as major positives; Xieji’s “用事” uses them as the primary “宜” sets for activities | Marriage, construction, contracts, travel, ritual, medical | Strong | Your engine may need to add this star layer for a serious system; Vietnamese “Thông thư” similarly lists “các sao tốt” and their recommended actions, indicating Vietnamese practice depends on these mappings citeturn8view0turn7view0turn21view1 |
| **Bad stars / taboo families** (*hung tinh / sát*) | 凶煞 / 忌日 families (月建/月破/四離四絕/土王用事/…) | Inauspicious indicators & taboo days | Often act as vetoes or heavy downgrades | Xieji lists many “所忌” families and defines non-overridable ones via “鋪註條例” and special-day rules | Construction, marriage, medical, commerce, funerals | Strong | Vietnamese “Thông thư” also lists “các ngày xấu” with explicit “kiêng…” scopes; treat these as hard/soft depending on Xieji vs folk layer citeturn10view2turn21view2 |

## C. Activity taxonomy

This taxonomy is designed for software: each category should have explicit (a) **primary selectors**, (b) **hard veto triggers**, and (c) **upgrade/downgrade modifiers**. Xieji explicitly frames selection as “take activities as the ‘warp’ and signals as the ‘weft’,” and provides a standardized activity catalog (御用/民用/通書用事). citeturn7view0

| Software category (VI / EN) | Closest Xieji “用事” anchors | Primary signals (highest priority) | Typical “kiêng kỵ / đại kỵ” vetoes | Downgrade (nên tránh) vs prohibit (kiêng kỵ) rule of thumb |
|---|---|---|---|---|
| **Cưới hỏi / Marriage** | 結婚姻 / 納采問名 / 嫁娶 citeturn7view0 | Xieji good-star set (天德/月德/天赦/天願 + 月恩/四相/時德), + 三合/六合/五合, + “天喜/不將” for weddings citeturn7view0turn8view0 | Xieji: month-break/build and heavy凶煞 lists for marriage; Yuxia: Pengzu “亥不嫁娶” and Yang Gong/Moon taboo sets (optional) citeturn7view0turn13view0turn12view3 | If an Xieji “德猶忌” veto day triggers, treat as **prohibit**. If only moderate negatives (e.g., hắc đạo alone), downgrade to “cân nhắc/avoid,” not absolute |
| **Động thổ / xây dựng / sửa nhà / Construction** | 興造動土 / 修造 / 豎柱上梁 / 營建宮室 citeturn7view0 | Xieji good-star set + “開日/成日” patterns; soil/earthwork-specific constraints (土府/土符/地囊/土王用事) citeturn7view0turn10view2 | Xieji: 土王用事 is explicit construction veto;四離四絕/上朔/晦 block most actions; Yuxia adds “戊己日” caution for earthworks in some months (treat as optional) citeturn10view2turn15view0 | Soil/earthwork taboo families → **prohibit**. Lack of strong positives but no veto → “cân nhắc,” especially for minor repairs |
| **Nhập trạch / chuyển nhà / Move-in** | 般移/移徙 / 入宅移居 citeturn7view0turn11view0 | Xieji: virtue stars + travel-horse signals (驛馬/天馬) + direct days (成日/開日) citeturn7view0 | Xieji: 往亡/歸忌 appear as strong avoiders for moving; month-break families; Yuxia “衝破” (day clashes month/year) as general negative citeturn7view0turn15view0 | If “veto” families (往亡/歸忌) appear, treat as **prohibit** for move-in; otherwise use score-based downgrade |
| **Khai trương / hợp đồng / Commerce & contract** | 開市 / 立券 / 交易 / 納財 citeturn7view0 | Xieji: 天願 + 民日 + “五富/母倉/天倉” (wealth storage) plus 三合/六合/五合 citeturn7view0turn8view0 | Xieji lists strong commerce vetoes: 月破 + 大耗/小耗/四耗 + 九空 etc; Pengzu “甲不開倉” is a strong folk taboo for inventory/warehouse actions citeturn7view0turn13view0 | The “loss cluster” (耗/空) → **prohibit** for opening/signing if you want a conservative product; otherwise classify as “không nên làm” (avoid) |
| **Xuất hành / Travel** | 出行 (民用) / 行幸遣使 (御用) citeturn7view0 | Xieji: virtue stars + 驛馬/天馬; Yuxia provides practical travel taboos and directions (optional extension) citeturn7view0turn11view0 | Xieji: 往亡 (and related) = strong; Yuxia: monthly taboo days and Yang Gong taboo (optional) citeturn7view0turn12view3 | For ordinary trips, downgrade. For long-distance/major departure, treat strong travel-vetoes as prohibit |
| **An táng / cải táng / Burial** | 破土 / 安葬 / 啟攢 citeturn7view0 | Xieji: burial-specific positives (鳴吠/鳴吠對, 六合) + core virtue stars; 28 mansions can act as strong veto/allowed depending on tradition pack citeturn7view0turn12view0 | Xieji: “復日/重日” and soil taboo families; Yuxia: some mansions categorically forbid burial; Yang Gong taboo explicitly warns burial leads to severe outcomes citeturn7view0turn12view0turn12view3 | Treat soil taboo families and “復日/重日” as **prohibit** (Xieji). Mansion-based vetoes should be optional/versioned but are often treated as prohibit |
| **Cầu cúng / lễ bái / Ritual–spiritual** | 祭祀 / 祈福 / 求嗣 citeturn7view0 | Xieji: virtue stars and “開日” etc; Xieji explicitly allows limited activities on special taboo days (上朔/四離四絕/晦) citeturn10view2turn7view0 | Xieji: 天狗寅日 bans祭祀 explicitly; special taboo days constrain to a short allowed list citeturn7view0turn10view2 | Ritual is often in the “exception list,” so classify as “allowed but with cautions” rather than blanket “kiêng” on special days |
| **Chữa bệnh / phẫu thuật / Medical** | 求醫療病 citeturn7view0 | Xieji: 天醫 + 解神/除神 + some direct days; treat as activity-specific model citeturn7view0turn8view0 | Xieji: 朔弦望 + monthly 15th explicitly forbid medical; Yuxia has similar “bad-day sets” (optional) citeturn10view2turn7view0 | Medical vetoes are strong; treat as **prohibit** because sources are explicit |
| **Gặp gỡ / đàm phán / kiện tụng** | 宴會 / 會親友 + (litigation appears indirectly via taboos) citeturn7view0 | Xieji:宴會 requires positive clusters; avoid “酉日” per annotation rules; Vietnamese “Thông thư” lists some bad days as “kiêng kiện tụng” citeturn10view2turn21view1turn21view2 | Xieji suppression rules (e.g.,酉日) are strong for meetings/banquets; other litigation aspects may be folk layer | Usually downgrade unless explicit “kiêng kiện tụng” day appears in your enabled dataset |
| **Học hành / thi cử** | 入學 / 應試赴舉 (Yuxia contains “應試赴舉吉日” section) citeturn11view0turn7view0 | Xieji: direct-day mapping (成日/開日) is explicit; Yuxia has dedicated exam day guidance (optional) citeturn7view0turn11view0 | Yuxia “先賢死葬日” forbids “入學求師” (folk layer) citeturn13view0 | Usually downgrade unless an explicit taboo day is active |
| **Planting / harvesting** | 栽種 / 取魚 / 畋獵 etc citeturn7view0 | Xieji has explicit “栽種 宜…” sets; Yuxia also contains agriculture-specific day sections citeturn7view0turn11view0 | Seasonal taboo families (土王用事) and “avoid harming 生氣” caution appears in Xieji’s discussion of 德日 and hunting/fishing citeturn8view0 | Generally use score + seasonal windows; reserve prohibit only for explicit soil/season taboos |
| **Ordinary daily restrictions** | Not a single “用事” category | Vietnamese practice often encodes through taboo-day sets (Tam nương, monthly taboo) and “sao xấu” lists | Tam nương poem in Vietnamese “Thông thư”; Yuxia monthly taboo (5/14/23) and Yang Gong taboo are explicit “百事忌” sets citeturn21view5turn12view3 | These should be labeled “folk taboo sets”; you can surface them as “nên tránh / kiêng kỵ” depending on strictness mode |

## D. Recommendation logic draft

**Core idea:** model recommendations as *activity-specific scoring with veto layers*, following Xieji’s principle that old calendars were wrong to treat “any凶煞 → everything忌,” and that one must compare “force” (力) and only preserve certain non-overridable taboos. citeturn8view0turn10view2

**Core inputs (what you have + minimal additions):**  
You already compute date structure + Trực + hoàng/hắc + tiết khí + 28 mansions + relationships. To implement Xieji-style *用事*, add (or derive) at least: **月建/月破**, **special taboo days** (上朔/四離四絕/晦/朔弦望/十五/土王用事/月忌日/百忌日), and a curated subset of **吉神/凶煞** that dominate Xieji “宜/忌” lists (e.g., 天德/月德/天赦/天願/月恩/四相/時德 vs 劫煞/災煞/月煞/月刑/月害/月厭/厭對/大時/天吏/四廢/五墓/九空, etc.). citeturn8view0turn10view2turn7view0

**Decision layers (software-friendly):**  
Layer 1 is **date-wide veto gating**, Layer 2 is **activity veto gating**, Layer 3 is **activity scoring**, Layer 4 is **conflict-resolution and suppression rules**, Layer 5 is **output labeling**.

```mermaid
flowchart TD
  A[Input: computed signals + optional star packs] --> B[Layer 1: Date-wide veto check]
  B -->|Global taboo triggers?| B1[Return: mostly KIENG KY\nwith allowed-exception list]
  B -->|No global veto| C[Layer 2: Activity-specific veto check]
  C -->|Veto hit| C1[Return: KIENG KY for that activity\nwith trace]
  C -->|No veto| D[Layer 3: Score positives vs negatives]
  D --> E[Layer 4: Apply Xieji suppression rules\n(e.g., 酉日 blocks banquet recommendations)]
  E --> F[Layer 5: Map score to verdict\n(nên làm / cân nhắc / không nên làm)]
  F --> G[Attach reason trace + confidence + tradition labels]
```

**Layer 1: date-wide veto rules (examples that map cleanly to code)**  
Xieji defines a class of special days where “only a small list of activities are not taboo; all others are taboo,” and explicitly says these taboos remain even if major virtues (德合/赦願) coincide. citeturn10view2  
Yuxia additionally defines wide “百事忌” sets (e.g., 楊公忌日; monthly 5/14/23). Treat these as optional “folk taboo packs,” not mandatory doctrine. citeturn12view3

**Layer 2: activity-specific veto rules**  
Example: medical is explicitly forbidden on 朔弦望 and the 15th; earthworks are forbidden under 土王用事; commerce is heavily vetoed by “耗/空” clusters. citeturn10view2turn7view0

**Layer 3: scoring / weighting**  
A practical scoring scheme that matches Xieji’s “compare force” principle:

- Assign each signal a **signed weight** (e.g., +3 strong positive; −3 strong negative; ±1 weak modifier).  
- Compute `score(activity) = Σ weights(signal_i if applicable to activity)`.  
- Maintain `veto(activity)` separate from score.  
- Maintain a `source_mode` list (Xieji-core vs Yuxia-pack vs Vietnamese-compiled pack).

Xieji itself conceptualizes force comparison and criticizes “blanket忌” when any凶煞 appears, implying a weighted approach is closer to its intent. citeturn8view0

**Layer 4: conflict resolution and suppression rules (use Xieji’s 鋪註條例 as algorithm spec)**  
Xieji provides implementable rules such as:  
- “Write recommendations by activity order; then compare 宜/忌 to determine what to keep.” citeturn10view2  
- “If good outweighs bad, follow 宜 not 忌 — **but** if it is a ‘德猶忌’ matter, still mark 忌.” citeturn10view2  
- “If good and bad offset, mark neither — again preserving ‘德猶忌’ prohibitions.” citeturn10view2  
- Multiple explicit suppression patterns: e.g., 酉日忌宴會 → do not label banquet/celebration as 宜 even if other positives exist; 卯日忌穿井 → suppress “open canal” pairing; 巳日忌出行 → suppress “travel” recommendations. citeturn10view2

In code, represent this as a list of **hard suppression constraints** keyed by `(trigger_signal, suppressed_activity_tags)`.

**Layer 5: output mapping + uncertainty labeling**  
Map to your desired UI states (four tiers + trace) and attach (a) the tradition label(s), (b) confidence, and (c) “why” explanations.

- **nên làm (Recommended):** no veto; score ≥ high threshold; at least one primary positive cluster for that activity (e.g., 天願 + 民日 for commerce; 天醫 for medical). citeturn7view0turn10view2  
- **cân nhắc (Consider):** no veto; score near neutral, or positives exist but lack top-tier enablers.  
- **không nên làm (Not recommended):** no veto; score clearly negative or explicit mild prohibitions.  
- **kiêng kỵ / đại kỵ (Taboo):** any veto triggers OR Xieji “德猶忌” cases OR enabled “百事忌” days. citeturn10view2turn12view3

For credibility: when your output depends on Yuxia-style folk sets (Yang Gong taboo days, monthly taboo days, some mansion verses), label as “Tongshu compilation tradition (玉匣記)” rather than “court-standard.” citeturn13view0turn12view3turn22search6

## E. Evidence table

This table provides “signal → activity → direction → strength → source” rows suitable to seed a first rule database. (It is illustrative, not exhaustive; Xieji alone contains dozens of activities and long signal lists.)

| Signal | Activity | Direction | Strength | Source | Source type | Notes / conflicts |
|---|---|---:|---:|---|---|---|
| 天德 / 月德 / 天赦 / 天願 | Many major activities (worship, marriage, construction, commerce, etc.) | Favor | Strong | Xieji “宜忌” lists these as major “所宜” and expands their “宜事” scope citeturn8view0turn7view0 | Court-standard | Use as primary positive cluster |
| 月建 / 月破 | 祈福 / 求嗣 / 結婚姻 etc. | Forbid | Strong | Xieji “用事” repeatedly lists 月建/月破 in 忌 sets citeturn7view0 | Court-standard | Treat as veto for many categories |
| 上朔 / 四離 / 四絕 / 晦日 | Most activities (allowed exceptions only) | Forbid | Strong | Xieji: only specific actions are not taboo; “with 德合/赦願 still taboo” citeturn10view2 | Court-standard | Implement as date-wide gating + exception list |
| 土王用事 | Construction / earthworks / planting / grave digging | Forbid | Strong | Xieji explicitly lists 土王用事 “忌” for a long list incl. 動土/栽種/破土 citeturn10view2 | Court-standard | Earthwork veto family |
| 朔弦望 / 十五日 | 求醫療病 | Forbid | Strong | Xieji explicitly: 朔弦望忌療病; 十五日義同朢日忌療病 citeturn10view2turn7view0 | Court-standard | Medical-specific veto |
| 天醫 | 求醫療病 | Favor | Strong | Xieji includes 天醫 in medical “宜” set via “成日(天喜天醫)” and explicit activity lists citeturn8view0turn7view0 | Court-standard | Model for activity-specific enabling stars |
| 鳴吠 / 鳴吠對 | 破土 / 安葬 / 啟攢 | Favor | Strong | Xieji: 破土宜鳴吠鳴吠對; 啟攢宜鳴吠對 citeturn7view0 | Court-standard | Funeral-specific enabling signals |
| 往亡 / 歸忌 | Travel / moving house | Forbid | Strong | Xieji includes 往亡 and 歸忌 in relevant “忌” lists citeturn7view0turn10view2 | Court-standard | Treat as veto for departure/moving |
| 鋪註條例: 吉足勝凶從宜不從忌 | All activities | Conflict rule | Core | Xieji “鋪註條例” provides this decision rule citeturn10view2 | Court-standard | Implement as scoring + thresholds with veto exceptions |
| 鋪註條例: 德猶忌仍註忌 | All activities | Override rule | Core | Xieji: even when good dominates, “德猶忌” must stay forbidden citeturn10view2 | Court-standard | Maintain a non-overridable veto list |
| 酉日忌宴會 (suppression) | Banquets/celebrations | Forbid | Strong | Xieji: 酉日忌宴會, suppress “宜慶賜賞賀” too citeturn10view2turn7view0 | Court-standard | Hard suppression mapping example |
| 卯日忌穿井; 壬日忌開渠 (suppression) | Digging well / canal | Forbid | Strong | Xieji provides explicit suppression pair rules for wells/canals citeturn10view2turn7view0 | Court-standard | Implement as explicit constraint rules |
| 角宿 (example mansion) | Marriage / building | Favor (but burial forbid) | Strong | Yuxia: 角宿 poem praises marriage/building but warns burial not allowed citeturn12view0 | Compilation/folk-layer | Mansion rules are high impact but versioned |
| 亢宿 / 氐宿 / 心宿 (examples) | Marriage / burial / building | Forbid | Strong | Yuxia: “凶” mansions warn marriage/burial/building lead to severe harm citeturn12view0 | Compilation/folk-layer | Use as optional “mansion pack” |
| 彭祖: 甲不開倉…亥不嫁娶 | Warehouse; marriage | Forbid | Medium–Strong | Yuxia Pengzu taboos are explicit and map to concrete activities citeturn13view0 | Compilation/folk-layer | Great for “reason trace”; consider strictness toggle |
| 地支六沖: day clashes month/year | Major activities | Negative | Strong | Yuxia: “日與月沖或與歲君沖皆凶” citeturn15view0 | Compilation/folk-layer | Clean to implement from existing xung logic |
| 地支相穿 (example) | Marriage matching | Forbid | Medium | Yuxia gives explicit example: 子年生不可用未日犯穿六害 citeturn14view2 | Compilation/folk-layer | Strongly suggests a personalization layer (birth-year) |
| 黃黑道: 建滿平收黑…閉破不相當 | General day-type | Mixed | Medium | Yuxia provides explicit hoàng/hắc mapping tied to Trực citeturn16view0 | Compilation/folk-layer | Useful shorthand; avoid treating as sole determinant |
| Vietnamese “CÁC SAO TỐT” list (Thiên hỷ, Thiên y, Nguyệt tài…) | Marriage, medical, commerce, moving | Favor | Medium | Vietnamese “Thông thư” explicitly maps “sao” to “Nên…” activities citeturn21view1 | Modern compilation (Vietnam practice) | Use for Vietnam terminology and additional stars (may not map 1:1 to Xieji naming) |
| Vietnamese “CÁC NGÀY XẤU” list (Thụ tử, Sát chủ, Đại hao…) | Many activities | Forbid | Medium–Strong | Vietnamese “Thông thư” explicitly states “Kiêng…” scopes, including “kiêng mọi việc” items citeturn21view2 | Modern compilation | Conflicts likely with Xieji; treat as “Vietnam practice ruleset” toggled mode |
| Vietnamese hoàng đạo/hắc đạo table by lunar months | General day-type | Mixed | Medium | Vietnamese “Thông thư” provides month→chi lists for hoàng đạo/hắc đạo citeturn21view3 | Modern compilation | Use as user-facing “calendar style” output; validate against your own hoàng/hắc computation |
| Vietnamese “Tam nương” poem | Marriage & house-building | Forbid | Medium | Vietnamese “Thông thư” includes “Tam-Nương nhật kỵ” and warns against building/marriage citeturn21view5 | Modern compilation / folk taboo layer | Treat as optional taboo pack unless you can anchor in stronger sources |
| 楊公忌日 (13 days) | Construction, marriage, burial, taking office | Forbid | Strong (folk) | Yuxia: explicit “百事忌” poem with listed dates citeturn12view3 | Compilation/folk-layer | Widely used; label as “Yuxia pack” |
| 月忌日 (5, 14, 23) | General; travel/household harm | Forbid | Strong (folk) | Yuxia: “百事忌” and explains harms citeturn12view3 | Compilation/folk-layer | Widely recognized in Vietnam; label and allow toggling |

## F. Conservative implementation proposal

**Goal:** ship a “minimal credible v1” that (a) matches widely accepted structure, (b) is explainable, and (c) avoids overclaiming in contested folk areas.

**Minimal credible v1 (recommended core mode = “Xieji-core”):**  
Implement Xieji’s *activity-first* model: use the **用事** catalog and **鋪註條例** as the algorithm spec, then compute a curated set of signals that Xieji repeatedly uses across activities. Xieji explicitly frames “choose dates by signals and activities” and provides a formal conflict system. citeturn7view0turn10view2

**Signals safe to include first (high consistency, easy to compute, high impact):**
1) **Special taboo days and exceptions**: 上朔/四離四絕/晦, 冬至/夏至/春分/秋分 constraints, 土王用事, 朔弦望, 十五日; these are explicitly described and often non-overridable. citeturn10view2  
2) **Trực (建除十二神)** as a day-type selector, tied directly to activities in Xieji “用事.” citeturn7view0turn8view0  
3) **Month build/break (月建/月破)** and a small “heavy negative” cluster (四廢/五墓/九空/大耗/小耗). citeturn10view2turn7view0  
4) **Primary positives**: 天德/月德(+合), 天赦, 天願, 月恩/四相/時德; these dominate “宜” sets. citeturn8view0turn7view0  
5) **A few activity-specific enabling stars**: 天醫 (medical), 天喜/不將 (marriage), 鳴吠/鳴吠對 (burial). citeturn8view0turn7view0  
6) **三合/六合/五合 and 六沖 (clash)** as upgrade/downgrade modifiers (you already compute relationship data; Yuxia provides explicit semantics for six clash). citeturn15view0turn10view2

**Signals to defer or ship as optional “packs” (higher inconsistency / versioning risk):**
- Full long-tail of 神煞 beyond the curated set; Xieji lists many, but implementing “all of them” without careful QA risks contradictions and opaque UX. citeturn7view0turn10view2  
- **28 mansions** as a veto engine unless you can guarantee the mansion computation matches the verse tradition you adopt; treat as a versioned pack with strong warnings. citeturn12view0  
- **Yang Gong taboo days / monthly 5-14-23 taboo / Tam nương**: widely used, but better shipped as explicitly labeled “folk taboo packs” with user-configurable strictness. citeturn12view3turn21view5  
- Personalized “xung tuổi / hợp tuổi / lục hại / tương xuyên” matching: valuable, but requires a user profile model and careful consent/UX framing. Yuxia’s marriage example indicates this is a distinct logic layer, not purely date-wide. citeturn14view2

**Defaults when sources disagree (responsible behavior):**
- Default to **Xieji-core** for “strong claims” (kiêng kỵ) and treat non-Xieji sources as *advisory* unless corroborated. Xieji self-identifies as correcting earlier mismatches and formalizing “force comparison,” which is a reasonable basis for conservative defaults. citeturn8view0turn22search6  
- When a Yuxia or Vietnam-compilation pack conflicts with Xieji, do **not** flatten into fake consensus. Present both as separate “tradition modes” and let product decide which is default. citeturn13view0turn10view2turn21view2

**Labeling / UX guidance to avoid false authority:**
- Every recommendation should carry a **trace**: `{signal, polarity, weight/class, source_mode, short explanation}`. Xieji’s own framing emphasizes explainability (“light/heavy, choose/reject can be distinguished”). citeturn7view0turn10view2  
- Display “confidence” based on (a) whether rule comes from Xieji-core vs a folk pack and (b) how many independent signals converge.  
- Avoid absolute language unless a rule is explicitly framed as “百事忌” or Xieji non-overridable veto. citeturn10view2turn12view3

## G. Open questions and final deliverables

**Open questions / unresolved inconsistencies (what you should treat as “research backlog”):**  
Vietnamese practice often merges multiple traditions (Vietnamese-market “Thông thư” star names, Yuxia taboo sets, Xieji court-standard rules). Without a curated set of Vietnamese print almanacs across regions/time, you may not know which “sao” names in Vietnamese correspond cleanly to Xieji’s canonical star taxonomy (e.g., Vietnam lists “Thiên phúc/Thiên hỷ/Nguyệt tài…” as action mappings). citeturn21view1turn8view0  
28 mansion rules can be highly categorical but are version-sensitive; you need to decide whether to ship (a) “mansion veto semantics” or (b) “mansion as soft modifier” in v1. citeturn12view0  
Personalized matching (xung tuổi, tương xuyên/lục hại in marriage) is likely expected by users but moves you from date-wide rules into user-profile constraints; you’ll need product policies and disclaimers. citeturn14view2  
Folk mitigation rituals (e.g., “black-road wedding → yellow shoes”) are culturally real but should be clearly labeled as folk remedies, not guaranteed fixes. citeturn16view0  
Scholarly framing suggests almanacs serve as normative “behavior manuals,” but scholarship generally won’t tell you *which exact stars override* others—Xieji does. Use scholarship to guide uncertainty labeling, not rules. citeturn23search3turn10view2

### Ranked list of most implementation-ready signals

| Rank | Signal / dimension | Why implementation-ready | Default mode |
|---:|---|---|---|
| 1 | **Xieji “鋪註條例” conflict engine** | Gives explicit resolution logic and suppression rules; directly implementable as constraints and thresholds citeturn10view2 | Xieji-core |
| 2 | **Special taboo days + exception list** (上朔/四離四絕/晦/土王用事/朔弦望/十五) | Clear veto semantics; explicit exception list for what remains allowed citeturn10view2 | Xieji-core |
| 3 | **Trực (建除十二神)** | Strong day-type selector; directly mapped to activities in Xieji and Vietnamese practice citeturn7view0turn16view0 | Xieji-core |
| 4 | **Month build/break (月建/月破)** | Repeated as strong negative across major activities in Xieji “用事” citeturn7view0turn10view2 | Xieji-core |
| 5 | **Core virtue stars** (天德/月德(+合)/天赦/天願/月恩/四相/時德) | Dominant positive backbone in Xieji for many activities citeturn8view0turn7view0 | Xieji-core |
| 6 | **Activity enablers** (天醫, 天喜, 不將, 鳴吠/鳴吠對) | High signal-to-action specificity citeturn7view0turn8view0 | Xieji-core |
| 7 | **Six clash + 合 sets** (六沖/三合/六合/五合) | Already computed by you; Yuxia gives explicit semantics for 六沖 and marriage “三合六合皆吉” citeturn15view0turn14view2 | Mixed (Xieji+Yuxia) |
| 8 | **Hoàng đạo/hắc đạo** | High user recognition; useful UI label; treat as modifier not sole determinant citeturn21view3turn16view0 | Optional |
| 9 | **Pengzu taboos** | Concrete mappings to daily actions (仓/planting/grooming/medicine/marriage/guests) citeturn13view0 | Optional pack |
| 10 | **Yang Gong, monthly taboo sets, Tam nương** | Strong user familiarity in many communities; but better shipped as labeled folk packs citeturn12view3turn21view5 | Optional pack |
| 11 | **28 lunar mansions** | High impact but version-sensitive; ship after validation citeturn12view0turn21view4 | Advanced pack |

### JSON-like recommendation schema

```js
{
  "date": {
    "gregorian": "YYYY-MM-DD",
    "lunar": { "day": 1, "month": 1, "year": 2026, "leapMonth": false },
    "canChi": {
      "day": { "stem": "甲", "branch": "子" },
      "month": { "stem": "丙", "branch": "寅" },
      "year": { "stem": "丙", "branch": "午" }
    },
    "tietKhi": { "vi": "Kinh Trập", "zh": "驚蟄", "isNodeDay": false },
    "computedSignals": {
      "truc": { "vi": "Khai", "zh": "開", "weightClass": "primary" },
      "hoangHac": { "type": "day", "isHoangDao": true, "ruleSet": "YuxiaPackV1" },
      "lunarMansion": { "vi": "Tâm", "zh": "心宿", "ruleSet": "YuxiaMansionV1" },
      "relations": {
        "sixClash": { "dayVsMonth": true, "pair": "子午冲" },
        "combos": ["三合", "六合"]
      },
      "tabooDays": {
        "xiejiCore": { "isSiLiSiJue": false, "isShangShuo": false, "isHui": false, "isTuWangYongShi": false },
        "folkPacks": { "isYangGongJi": false, "isYueJi_5_14_23": false, "isTamNuong": false }
      },
      "stars": {
        "good": ["天德", "月德", "天赦", "天願"],
        "bad": ["月破", "大耗", "四廢"]
      }
    }
  },
  "recommendations": {
    "marriage": {
      "verdict": "khongNenLam",
      "tier": "avoid",
      "confidence": "medium",
      "traditionModesUsed": ["XiejiCore"],
      "trace": [
        { "signal": "月破", "polarity": "negative", "severity": "veto", "source": "Xieji", "explain": "Month-break is listed among the main '忌' sets for marriage-family actions." },
        { "signal": "天德", "polarity": "positive", "severity": "primary", "source": "Xieji", "explain": "Virtue stars are major enablers but do not override non-overridable taboos." }
      ]
    }
  }
}
```

(Source rationale for schema fields: Xieji defines activity-first annotation (“以事為經以神為緯”) and provides explicit precedence and suppression rules, so a traceable object model aligns with the tradition and is engineer-testable. citeturn7view0turn10view2)

### Sample rule models for five activity categories

These examples are deliberately conservative: veto-first, then weighted scoring, then Xieji-style suppression.

```js
// Shared helpers
const VETO = "veto";
const POS = "pos";
const NEG = "neg";

ruleSet = {
  name: "XiejiCoreV1",
  precedence: {
    // Mirrors 鋪註條例: keep vetoes for 德猶忌; allow "good beats bad" elsewhere.
    vetoBeatsScore: true,
    goodBeatsBadThreshold: 2,
    tieBand: [-1, +1],
    suppressionRules: [
      { if: ["酉日忌宴會"], suppress: ["social.meetings", "celebrations"] },
      { if: ["卯日忌穿井"], suppress: ["construction.digWell"] },
      { if: ["巳日忌出行"], suppress: ["travel.departure"] }
    ]
  },

  activities: {
    "marriage": {
      vetoIf: ["上朔", "四離四絕", "晦日", "月破", "月建"],          // Xieji-core
      preferIf: ["天德", "月德", "天赦", "天願", "三合", "六合", "天喜", "不將"],
      avoidIf: ["劫煞", "災煞", "月刑", "月害", "月厭", "厭對", "四廢", "五墓"],
      scoring: { prefer: +2, avoid: -2, weakPrefer: +1, weakAvoid: -1 }
    },

    "construction": {
      vetoIf: ["土王用事", "上朔", "四離四絕", "晦日", "月破", "土府", "土符", "地囊"],
      preferIf: ["天德", "月德", "天赦", "天願", "月恩", "四相", "時德", "三合", "開日"],
      avoidIf: ["四廢", "五墓", "九空", "大時", "天吏"],
      scoring: { prefer: +2, avoid: -2, weakPrefer: +1, weakAvoid: -1 }
    },

    "businessOpening": {
      vetoIf: ["上朔", "四離四絕", "晦日", "月破", "大耗", "小耗", "四耗", "九空"],
      preferIf: ["天願", "民日", "五富", "天倉", "母倉", "三合", "六合", "滿日", "成日", "開日"],
      avoidIf: ["月厭", "月刑", "月害", "四廢", "五墓"],
      scoring: { prefer: +2, avoid: -2, weakPrefer: +1, weakAvoid: -1 }
    },

    "travel": {
      vetoIf: ["上朔", "四離四絕", "晦日", "往亡", "歸忌"],
      preferIf: ["天德", "月德", "天赦", "天願", "驛馬", "天馬", "開日"],
      avoidIf: ["月破", "四廢", "五墓"],
      scoring: { prefer: +2, avoid: -2, weakPrefer: +1, weakAvoid: -1 }
    },

    "medical": {
      vetoIf: ["朔弦望", "十五日", "上朔", "四離四絕"],             // explicit medical taboos
      preferIf: ["天醫", "解神", "除神", "天德", "月德"],
      avoidIf: ["月建", "收日", "閉日", "四廢", "五墓"],
      scoring: { prefer: +2, avoid: -2, weakPrefer: +1, weakAvoid: -1 }
    }
  }
};
```

(Why these rule ingredients: Xieji’s “用事” provides per-activity 宜/忌 sets; Xieji’s “鋪註條例” provides the veto/override and suppression logic; Xieji’s “special taboo day” section defines global gating with exceptions; Yuxia provides additional folk packs and explicit mnemonics such as hoàng/hắc mappings and taboo day sets. citeturn7view0turn10view2turn16view0turn12view3)

### Bibliography grouped by Vietnamese, Chinese, Korean sources

**Vietnamese sources (traditional holdings + modern practice compilations)**  
- Vietnam-held Hán-script tongshu: 增補選擇通書廣玉匣記 (1920, woodblock; National Library of Vietnam). citeturn18view0  
- Vietnam-held Hán-script tongshu: 玉匣攢要通用 (1926, woodblock; National Library of Vietnam). citeturn18view2  
- Vietnam-held Hán-script manual: 玉匣纂要通用 (Chùa Phổ Nhân; described as covering day-selection for trade/marriage/etc.). citeturn18view1  
- Institutional bibliographic framing: Viện Nghiên cứu Hán Nôm “Di sản Hán Nôm Việt Nam – Thư mục đề yếu” description of the project’s scholarly importance and corpus coverage. citeturn17view0  
- Practice-facing Vietnamese compilation: “THÔNG THƯ” PDF (lists “các sao tốt/xấu,” hoàng đạo/hắc đạo, nguyệt đức, 28 sao, tam nương). citeturn20view0turn21view1turn21view2turn21view3turn21view5  

**Chinese sources (normative base + widely used compilation)**  
- 欽定協紀辨方書 (四庫全書本), 卷十 “宜忌” and “鋪註條例”; 卷十一 “用事.” citeturn8view0turn10view2turn7view0  
- 玉匣記 (Wikisource edition): 二十八宿值日吉凶歌; 彭祖百忌日; 楊公忌日; 月忌日; 地支六沖凶日; 黃黑道用事吉日; 用日法. citeturn12view0turn13view0turn12view3turn15view0turn16view0  
- Reference overview: ChinaKnowledge entry on (Qinding) Xieji bianfang shu (compilation under Prince Yunlu, finished 1739). citeturn22search6  

**Korean sources (included only for added value)**  
- SillokWiki entry on 協紀辨方書: notes compilation to correct earlier date-selection errors and its adoption as a *Joseon* date-selection exam textbook (1791). citeturn22search0  

**Scholarship (context for documentation / uncertainty principles)**  
- Richard J. Smith (Brill): notes direct line of descent from early daybooks (日書) to later almanacs (通書/皇曆/黃曆). citeturn23search0  
- JSTOR OA chapter on Qing calendar books as normative ritual/behavior manuals (“Time as Norm…”). citeturn23search3