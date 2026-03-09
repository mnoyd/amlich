# Framework for Deriving Daily Guidance from Vietnamese-Chinese Almanac Signals

## Scope and method

This research focuses on translating computed calendrical signals into actionable “should do / avoid / taboo” guidance, in a way that can be implemented as software logic rather than a cultural essay. It prioritizes (a) Vietnamese **practice-facing** sources (Vietnamese-language almanacs and “thông thư / tongshu”-derived traditions commonly used in Vietnam), then (b) the most **normative** and **explicitly rule-based** classical Chinese sources that underpin those practices, and includes Korean material only where it clarifies authority and transmission.

A key limitation of Vietnamese “traditional” material is that much of the operative rule-set is historically imported and standardized through Chinese date-selection (擇日 / 選擇) literature; Vietnamese usage often appears as Vietnamese-language editions/compilations of those rule systems (e.g., *Ngọc hạp thông thư* as a vernacularized/market form of *Yuxia tongshu* traditions), plus modern Vietnam market almanacs (lịch vạn niên / hoàng lịch) that summarize the classical rule layers. Therefore, the most reliable way to build a rigorous rules model is to anchor the knowledge model in (1) court-standard and heavily structured selection manuals (especially **欽定協紀辨方書**), then map the signals and terminology onto Vietnamese conventions. The **Qing court “Qinding Xieji Bianfang Shu”** (欽定協紀辨方書) is particularly important because it explicitly aims to correct earlier contradictions and provides both (i) a signal glossary and (ii) activity-by-activity 宜/忌 lists, plus a conflict-resolution approach for annotated almanacs. citeturn27view0turn26view0turn34view0

## Source map

### Vietnamese source base

**Vietnamese “Lịch vạn niên / Hoàng lịch” commercial-almanac tradition (modern compilation / folk-practice facing).** Modern “hoàng lịch” publications explicitly present “xem ngày tốt xấu theo lịch dân gian” and function as a practical interface layer over older selection rules (often: trực, hoàng đạo/hắc đạo, xung tuổi, sao tốt/xấu, etc.). A representative cataloged example is *Hoàng lịch năm 2009–2011: xem ngày tốt xấu theo lịch dân gian* (Nguyễn Bích Hằng). citeturn6search3  
Trust/influence: high in *actual consumer usage* in Vietnam (these are the artifacts people consult), but variable in rule purity (rules can be simplified, merged, or commercialized).

**Vietnamese-language “choose-a-day” manuals (modern interpretive / sometimes corrective).** A notable example is *Nguyên lý chọn ngày theo lịch Can chi*, which emphasizes reducing “kiêng kỵ vô lý” in popular habit and has multiple reprints/editions, suggesting sustained readership. citeturn6search4  
Trust/influence: medium. These are interpretive and may deviate from classical layers, but they are valuable for understanding **what modern Vietnamese users expect** a “serious” system to cover or reject.

**Vietnamese-language “Thông thư vạn sự” style books (modern compilation).** Example listing: *Thông Thư Vạn Sự* (Quảng Tuệ), a modern compilation marketed as covering “cát tinh / hung tinh” and selecting good/bad days. citeturn25search1  
Trust/influence: medium-to-low intellectually (varies by author), but high as a signal of **modern “folk-almanac” feature scope**.

Practical note for implementation: Vietnamese sources above are best used to (a) build your product’s **terminology/UX** and expected activity categories, and (b) flag which rule families users assume exist, while the core rule logic should be anchored in more systematic selection texts below to avoid a “random SEO consensus.”

### Chinese source base

**欽定協紀辨方書 (Qinding Xieji Bianfang Shu, 36 juan).** A Qing-court–endorsed, standardized date-selection compendium compiled under imperial order, completed in 1739, intended to correct errors and inconsistencies in earlier selection traditions. citeturn39view0turn7search4  
Why it is key: it provides (1) a structured **signal taxonomy**, (2) an explicit **activity list** (御用/民用/通書用事) and for each activity the **宜/忌 signal sets**, and (3) explicit guidance on **how to annotate** or resolve conflicts when producing a “万年书 / tongshu”-style calendar. citeturn34view0turn26view0  
Signals it discusses that matter to your engine: 建除十二神 (12 day officers = trực), month build/break (月建/月破), many “神煞” (good/bad stars), and explicit precedence rules. citeturn27view0turn28view2turn33view0

**玉匣記 / 玉匣記通書 (Yuxia Ji / Yuxia Tongshu tradition).** A widely circulating “tongshu” compilation with many versions; Wikisource notes it is a representative collection of diverse divination techniques and later accretions, traditionally attributed to Xu Xun (許遜) but commonly pseudepigraphic and versional. citeturn11view0turn35search1  
Why it is useful: it contains very explicit, practice-facing mapping tables and verses, including (a) 二十八宿值日吉凶 (28 lunar mansions day-by-day auspice), (b) 彭祖百忌 (Pengzu “hundred taboos”), (c) 楊公忌日 / 月忌日 (widely used taboo-day sets), and (d) “yellow/black road” usage notes—high value for building a rules engine, but must be tagged as “compilation/folk-layer” rather than “court-standard.” citeturn12view1turn37view3turn38view1turn14view0

**Scholarly context on continuity from early daybooks to late almanacs.** For historical grounding, scholarship notes a direct lineage from late Zhou/Qin/Han “daybooks” (日書) to late imperial almanacs (通書/曆書/黃曆). citeturn24search2  
Implementation relevance: reinforces that many “daily guidance” dimensions are not modern inventions, even if later layers (folk accretions) exist.

### Korean sources with real added value

**SillokWiki entry on 協紀辨方書 (협기변방서).** This Korean scholarly encyclopedia entry states the book was compiled in 1739 (36 volumes) by Qing astronomical officials and was adopted as a **textbook for date-selection (택일) examinations** in Joseon (1791), indicating cross-cultural institutional authority and preservation. citeturn39view0  
Value: it supports treating Xieji Bianfang Shu as a credible “standard reference layer” for East Asian almanac logic, not merely a later commercial artifact.

## Signal-by-signal analysis

This section is organized around signals you already compute, plus the additional dimensions that the classical “selection” tradition treats as essential for producing **activity-specific** recommendations. The main technical takeaway is that a serious recommendation system in this tradition is not driven by a single signal (e.g., “hoàng đạo day”) but by a **layered set of rule families**, with explicit “宜/忌” lists per activity and explicit conflict-handling heuristics. citeturn26view0turn27view0turn34view0

### Trực

**Vietnamese name:** *Trực* (Kiến/Trừ/Mãn/Bình/Định/Chấp/Phá/Nguy/Thành/Thu/Khải/Bế)  
**Chinese name:** 建除十二神; also discussed as 建除 (or “十二日”) and related aliases (同位異名). citeturn27view0turn28view2  
**English gloss:** “Twelve Day Officers” / “12 day-operator cycle”

**Meaning / use:** A monthly-cycling 12-state classifier used to “determine auspice by what the day ‘is’” (i.e., which officer governs it). Xieji explicitly frames this as a recurring 12-day cycle “to determine good/ill” via the officer. citeturn27view0

**Activity linkage (evidence-grounded examples):**  
In the *Xieji* “用事” section (卷十一), many activities explicitly require certain officers, e.g. “入學” is associated with 成日 and 開日, and “冠帶” with 定日. citeturn34view0  
The same text uses officers as decisive components inside “宜/忌” sets across domains (marriage, movement, commerce, medical, construction). citeturn33view3turn33view4turn33view0  
*Yuxia Ji* includes a compact “Pengzu” block that maps officers to actions (e.g., 建→travel; 收→taking money but avoid burial; etc.), showing how this logic is surfaced in practice-oriented almanacs. citeturn38view1turn13view0

**Relative weight:** **Strong**, because Xieji treats the officer system as one of the core “纲” layers and repeatedly uses it inside “宜/忌” criteria for major activities. citeturn27view0turn28view2turn33view4  
Caveat: the Xieji compiler notes that multiple traditions disagree on which officers are “yellow/black” or how to assign some meanings, and warns that “many auspicious gods → auspicious; many inauspicious gods → inauspicious,” so direct alone is not absolute. citeturn27view0turn26view0

### Hoàng đạo / hắc đạo

**Vietnamese name:** *Hoàng đạo* / *Hắc đạo*  
**Chinese names in selection texts:** 黃道 / 黑道; also “six yellow-road gods” listed as 青龍、明堂、金匱、寶光、玉堂、司命 (often the canonical “hoàng đạo” set). citeturn28view2turn14view0  
**English gloss:** “Yellow Road / Black Road days” (auspicious vs inauspicious day rulers)

**Meaning / use:**  
Xieji explicitly treats the six “Yellow Road” day-gods as generally auspicious due to proper placement/order, but also states they do **not** have “exclusive” dedicated activities; rather, when combined with other gods you follow those gods’ “宜/忌,” i.e., hoàng đạo is a **contextual amplifier** not a full rule by itself. citeturn28view2  
Yuxia Ji includes a pragmatic folk mitigation: for marriage on a black-road day, “wear yellow shoes” and treat as resolved, illustrating that in some practice layers, black-road is not always an absolute blocker. citeturn14view0

**Relative weight:** **Medium** in a conservative implementation. It is widely used as a user-facing label, but Xieji’s treatment implies it should not override stronger “activity-specific” or “hard taboo” constraints. citeturn28view2turn26view0

### Nhị thập bát tú

**Vietnamese name:** *Nhị thập bát tú*  
**Chinese name:** 二十八宿 (lunar mansions)  
**English gloss:** “28 lunar mansions”

**Meaning / use:**  
Yuxia Ji provides “二十八宿值日吉凶歌,” a verse-based mapping from each mansion to typical outcomes for construction, marriage, burial, travel, legal trouble, etc. For example, some mansions explicitly say “good for building/marriage; burial forbidden,” while others warn marriage or burial leads to serious misfortune. citeturn12view1  
This is a high-surface-area signal: it can generate strong “avoid” outputs for certain categories, but it is also tradition-variant (different systems and mnemonics exist across lineages), so your system should treat mansion rules as “strong but versioned.” citeturn11view0turn12view1

**Relative weight:** **Strong for activity-specific prohibitions**, especially burial and construction, because the mansion verses often express categorical “不可用” (“must not use”). citeturn12view1  
Caveat: many mansion rules are poetic and may encode symbolic rather than systematically justified reasoning, making conflicts likely unless you implement precedence rules (see below). citeturn26view0turn27view0

### Can chi of day/month/year

**Vietnamese name:** *Can Chi* (Thiên Can / Địa Chi)  
**Chinese name:** 天干地支  
**English gloss:** “Heavenly Stems & Earthly Branches (sexagenary cycle)”

**Meaning / use in recommendation:**  
In the selection tradition, stems/branches are not merely labels; they generate multiple derived “rule triggers,” including:  
(1) **hard taboos keyed to stem/branch**, e.g., “Pengzu hundred taboos” (彭祖百忌) assigns specific prohibitions by stem and by branch (e.g., “甲不開倉…”, “亥不嫁娶…”). citeturn38view1turn26view0  
(2) **month build/break logic** (月建/月破) and many “month-sha” families referenced heavily in Xieji’s “用事.” citeturn28view2turn33view4turn34view0  
(3) **day strength vs month seasonality** (旺相 / 休囚) as a method for judging day quality, explicitly discussed in Yuxia Ji’s “用日法,” where day auspice depends strongly on 月令 (seasonal command) and the day stem. citeturn31view0

**Relative weight:** **Strong**, because many “core” taboos and exemptions are can-chi–driven in both Xieji and Yuxia. citeturn26view0turn38view1turn33view4  
Engineering caveat: some can-chi derived systems are “hard-coded mnemonic lists” (e.g., Pengzu) rather than algorithmic derivations; treat them as explicit rule tables.

### Xung / hợp / hình / hại / phá / tuyệt style relationships

**Vietnamese names:**  
*xung* (沖), *hợp* (合: 六合/三合/五合), *hình* (刑), *hại* (害), *phá* (破), *tuyệt* (絕), etc.  
**English gloss:** “Clash / Combine / Punish / Harm / Break / Sever relations”

**How tradition uses them in recommendations:**  
A conservative, evidence-backed subset you can implement directly:

**Day–month or day–year clashes (六沖 as ‘xung’):** Yuxia Ji defines “地支六沖凶日,” stating that a “clash break” occurs when day clashes the month or year-lord, and lists the six opposing pairs (子午, 丑未, 寅申, 卯酉, 辰戌, 巳亥). citeturn31view0  
Software implication: if your engine already computes xung relations, you can detect “day clashes month” and “day clashes year” as strong negative triggers.

**Combinational positives (三合/六合/五合):** Xieji (卷十) assigns explicit “宜” lists for 六合 (e.g., suitable for banquets, marriage, contracts, wealth intake, livestock, and even burial) and similarly lists 三合 as broadly auspicious and structurally foundational (“日之吉者莫如三合”). citeturn28view2turn28view1  
Software implication: treat 三合/六合/五合 as *positive modifiers* that can upgrade “consider” to “recommended,” unless a hard taboo applies.

**Relative weight:** **Medium to Strong**, depending on subtype: “day clashes month/year” is typically stronger (often treated as a reject condition), while “六合/三合” are strong positive but still subordinate to certain hard taboos (see precedence). citeturn31view0turn28view1turn26view0  
Caveat:刑/害/破/绝 are widely used in folk practice but are less explicitly enumerated in the excerpts above; if you implement them, label them as “traditional compatibility constraints” rather than universally agreed “taboos,” unless you have direct textual anchors per activity.

### Tiết khí

**Vietnamese name:** *Tiết khí* (24 solar terms)  
**Chinese name:** 二十四節氣; and derivative taboo patterns keyed to solar terms  
**English gloss:** “Solar terms / seasonal nodes”

**How it is used (in date-selection logic rather than astronomy):**  
In Xieji’s “宜忌” (卷十), “四離四絕” are described as days where “two qi and five phases separate/decide” and are treated as days when most activities are taboo (with narrow exceptions like祭祀/解除). Importantly, Xieji states these remain taboo even if they coincide with high-grade good-day markers like 德合/赦願 (“與徳合赦願併猶忌”). citeturn26view0  
Xieji also treats the solstices and equinoxes (冬至、夏至、春分、秋分) as days where certain major activities are not even annotated as suitable (“雖吉日亦不註此數事”), implying a downgrading/avoidance rule for high-stakes actions on these nodes. citeturn26view0  
Yuxia Ji further shows “seasonal” solar-term usage for agricultural forecasting and practice-oriented guidance, indicating that solar terms naturally support agriculture/seasonal recommendations (though these are often separate from “major event selection”). citeturn31view0

**Relative weight:** **Strong when it triggers hard-taboo constructs** (四離四絕, some solstice/equinox restrictions), **weak-to-medium** for general daily advice outside agriculture unless you intentionally support seasonal tasks. citeturn26view0turn31view0

### Good stars / bad stars and “thần sát” families

**Vietnamese umbrella terms:** *cát tinh / hung tinh*, *thần sát*, *sao tốt / sao xấu*  
**Chinese terms:** 吉神 / 凶煞 / 神煞  
**English gloss:** “Auspicious / inauspicious deities (‘stars’) used in selection”

**Why they are essential for a serious recommendation system:**  
Xieji’s “用事” (卷十一) is explicitly organized as: “for each activity, list what is 宜 and what is 忌,” and its lists are dominated by these good/bad star families (天德/月德/天赦/天願/月恩/四相/時德/劫煞/災煞/月煞/月刑/月害/月厭/大時/天吏/四廢/五墓… etc.). citeturn34view0turn33view0turn33view4  
This means: if your engine does **not** compute at least a minimal subset of these stars, you can only produce a “partial almanac,” not a faithful “thông thư” recommendation engine.

**Weight:** Many of these are treated as **primary** signals for specific activities. For example, construction (興造動土) explicitly requires multiple “virtue” stars and prohibits a long list of soil/season taboo stars. citeturn33view0

**Practical “minimal set” suggested by Xieji tables:**  
A defensible v1 set is: Tian De / Yue De (天德/月德) and their “合,” Tian She (天赦), Tian Yuan (天願), Yue En (月恩), Si Xiang (四相), Shi De (時德), plus the strongest negatives: month build/break (月建/月破), soil/ground taboos (土府/土符/地囊/土王用事), “big time officials” (大時/天吏), seasonal dead days (四廢/五墓), and a small group of month-sha (劫煞/災煞/月煞/月刑/月害/月厭). citeturn28view2turn33view0turn33view3turn26view0

## Activity taxonomy

A practical, engineering-friendly taxonomy should be built from **documented “用事” lists** rather than ad-hoc modern categories, then mapped to Vietnamese UX labels. Xieji explicitly states it combines “御用六十七事,” “民用三十七事,” and “通書選擇六十事” into one ordered list, then assigns 宜/忌 under each activity so that “weight and choice can be distinguished.” citeturn34view0

Below is a software taxonomy aligned to typical Vietnamese product categories, mapped to classical “用事” anchors (Chinese), with notes on which signals dominate.

### Marriage and family formation

Vietnamese UX cluster: **cưới hỏi / hôn nhân / ăn hỏi / đăng ký kết hôn / lễ cưới**  
Primary classical anchors: 結婚姻、納采問名、嫁娶. citeturn34view0  

Signals that matter most (evidence-based):  
Xieji links marriage-family activities to core “virtue” stars and combinational positives (三合/六合/五合), and forbids strong negatives such as 月建/月破 and specific officer-days (e.g., 平日/收日/滿日/閉日 in the marriage-family set). citeturn34view0  
Special-case flag: “不將” is explicitly listed as 宜 for “嫁娶” (a strong positive used specifically for marriage). citeturn34view0turn28view2  
Hard-taboo examples: Yuxia’s “楊公忌日” poem marks “百事忌” and explicitly calls out marriage as “also not appropriate.” citeturn37view3

### Construction and property works

Vietnamese UX cluster: **động thổ / xây dựng / sửa nhà / làm nhà / đổ mái / đặt móng / lợp mái**  
Primary anchors: 興造動土、修造、豎柱上梁、營建宮室、修宮室、繕城郭、築隄防. citeturn33view0turn34view0  

Signals that matter most:  
Xieji makes construction heavily dependent on virtue-stars and explicitly prohibits soil/ground taboo star families and “土王用事.” citeturn33view0turn26view0  
If your system supports “trực,” note that construction is repeatedly linked to 開日 in the Xieji “興造動土” and “豎柱上梁” entries. citeturn33view0  
Yuxia adds practice-facing detail: “動土開基” must avoid multiple soil-sha families and certain “direct” days (建/破/平/收) in that tradition. citeturn14view1

### Moving house and entering residence

Vietnamese UX cluster: **nhập trạch / chuyển nhà / dọn nhà /搬家 / an cư**  
Primary anchors: 般移/移徙, 入宅移居 (present in Yuxia), 安牀. citeturn34view0turn13view3  

Signals that matter most:  
Xieji ties moving (般移/移徙) to virtue-stars plus travel-horse signals (驛馬/天馬), and prefers 成日/開日; it forbids a cluster including 月破 and other traveling-death star families such as 歸忌/往亡. citeturn34view0  
Bed placement (安牀) is tied to 危日 and forbids specific branch-days in addition to the common “month break” cluster. citeturn33view3

### Commerce and legal-economic actions

Vietnamese UX cluster: **khai trương / mở cửa hàng / mở bán / ký hợp đồng / lập giấy tờ / đặt cọc / giao dịch / mua bán**  
Primary anchors: 開市、立券、交易、納財、開倉庫出貨財. citeturn33view4turn34view0  

Signals that matter most:  
Xieji explicitly associates opening-market (開市) with 天願, 民日, certain officers (滿日/成日/開日) and “五富,” while forbidding the month-break cluster and “空”/loss signals (大耗/小耗/九空 etc.). citeturn33view4  
Contracts/trading (立券交易) emphasize 三合/六合/五合 plus wealth stars, with explicit avoidance of “五離” and other loss indicators. citeturn33view4turn26view0  
Pengzu taboos add stem/branch “hard avoid” flags (e.g., 甲 day avoid opening storehouse; 亥 day avoid marriage; 酉 day avoid meeting guests), which can be surfaced as “kiêng kỵ” warnings for commerce and social events. citeturn38view1turn26view0

### Travel and going out

Vietnamese UX cluster: **xuất hành / đi xa / công tác / khởi hành**  
Primary anchors: 出行, 行幸遣使 (travel in official sense), plus Yuxia’s “出行” sections. citeturn34view0turn11view0  

Signals that matter most:  
In Xieji, travel is tied to virtue-stars plus 驛馬/天馬 and also 建日 in some official contexts, while avoiding certain taboo day clusters (往亡, 天賊, etc.). citeturn34view0turn28view1  
Yuxia includes a “黄黑道日当用之事” mnemonic that maps certain officers to travel or cautions, illustrating how travel is “activity-typed” rather than simply “good day/bad day.” citeturn14view0

### Funerary and burial

Vietnamese UX cluster: **an táng / cải táng / tang lễ / bốc mộ**  
Primary anchors: 破土、安葬、啓攢. citeturn33view2turn34view0  

Signals that matter most:  
Xieji treats burial with explicit “鳴吠/鳴吠對” signals as positive (for 破土/啟攢) and forbids month build/break plus repeated “month-sha” clusters; burial also includes special taboo families (四忌/四窮/復日/重日). citeturn33view2turn28view2  
Yuxia’s 28-mansion verses are often extremely categorical for burial (“不可用” for certain mansions), making mansions a strong activity-specific veto if you adopt that layer. citeturn12view1

### Religion, prayer, and ritual

Vietnamese UX cluster: **cầu cúng / lễ bái / cúng gia tiên / cầu an / cầu phúc**  
Primary anchors: 祭祀、祈福、求嗣. citeturn34view0  

Signals that matter most:  
Xieji lists “virtue” stars (天德/月德 etc) plus “普護/福生/聖心/益後/續世” as positive for ritual/request actions, while forbidding month build/break and certain official/avoidance days for “祈福/求嗣.” citeturn34view0turn28view2  
Hard-taboo exception handling: Xieji explicitly states that on 上朔/四離/四絕/晦 days, only a narrow set of activities (including 祭祀 and 解除) are not taboo; others are. citeturn26view0

### Medical treatment

Vietnamese UX cluster: **khám bệnh / chữa bệnh / phẫu thuật / uống thuốc**  
Primary anchors: 求醫療病, plus Yuxia’s “求醫治病吉日 / 合藥服藥吉日.” citeturn33view3turn13view3  

Signals that matter most:  
Xieji explicitly lists positive signals for treatment (including 天醫 and certain “解除” family signals like 解神/除神), and forbids multiple hard triggers including “朔弦望日” and monthly fifteenth days, as well as certain branches (未日). citeturn33view3turn26view0

## Recommendation logic draft

This section proposes a software-friendly model that mirrors how the normative sources organize information: **activity-by-activity “宜/忌” signal matrices**, plus explicit precedence and exception logic.

### Core inputs

Minimum (already in your engine):  
lunar day/month/year; day/month/year can-chi; trực (12 day officers); relationships (xung/hợp/hình/hại…); tiết khí; 28 lunar mansion; hoàng đạo/hắc đạo.

Additional inputs required for a serious “tongshu-style” engine (explicitly required by Xieji’s tables):  
a selected set of 吉神/凶煞 (“stars”), including at least “virtue days” and high-impact taboo families used across many activities (e.g., 天德/月德/天赦/天願/月恩/四相/時德 vs 月建/月破/土府/土王用事/大時/天吏/四廢/五墓 plus a small set of month-sha). citeturn28view2turn33view0turn34view0turn26view0

### Decision layers and precedence

A conservative, evidence-aligned precedence stack:

**Hard global taboos (date-wide) that override most “should do” outputs:**  
Xieji explicitly marks 上朔、四離、四絕、晦 as days where only a small subset of actions are permitted; “all other actions are taboo,” and these remain taboo even if 德合/赦願 coincide. citeturn26view0  
Yuxia also provides “百事忌” sets (楊公忌日, 月忌日) that are treated as universal taboos in that tradition layer. citeturn37view3  

Recommended implementation: classify these as **“đại kỵ / absolute avoid”** unless the activity is in the explicit exception list.

**Activity-specific hard taboos:**  
Example: medical treatment is explicitly taboo on “朔弦望日” (new moon/quarters/full) and on monthly 15th, plus other triggers; treat these as hard negatives for medical. citeturn33view3turn26view0  
Example: construction is hard-blocked by 土王用事 and related soil taboo families in Xieji. citeturn26view0turn33view0

**Primary activity selectors:**  
In Xieji, the highest-coverage “primary positive” family for most major actions is the virtue set (天德/月德/天赦/天願 plus 月恩/四相/時德), augmented by combination days (三合/六合/五合) and specific-purpose stars (天喜 for marriage-like joy, 天醫 for treatment). citeturn34view0turn33view3turn33view4

**Secondary/modifier signals:**  
Hoàng đạo/hắc đạo is treated as a context marker; Xieji indicates it is not “exclusive-action binding,” and recommends following other gods’ rules when combined. citeturn28view2  
Likewise, 28 mansions can be treated as strong modifiers or vetoes per activity, but because they vary by tradition, you should version and label them. citeturn12view1turn11view0

### Conflict handling

Xieji provides explicit “annotator rules” (鋪註條例) for combining signals into calendar annotations:

* If “good is sufficient to overcome bad,” annotate as **宜** rather than **忌**, except for certain “德猶忌” matters which must still be marked as forbidden. citeturn26view0  
* If good and bad offset, annotate neither (or downgrade), but still mark “德猶忌” taboos. citeturn26view0  
* For specific common conflicts (e.g., 酉 day is taboo for banquets so do not annotate banquet-related goods), Xieji gives concrete suppression rules. citeturn26view0  

Software translation:  
1. Compute a per-activity **veto set** (hard prohibitions).  
2. Compute a per-activity **score** from positive/negative signals.  
3. Apply Xieji-style suppression/override: veto beats score; “德猶忌” beats score; otherwise score can upgrade/downgrade.

### Output mapping

A minimal four-level output that matches your product requirements:

* **nên làm (Recommended):** score high AND no veto triggers; ideally includes one or more primary positives (virtue set) plus an activity-specific enabling signal (e.g., 開日 for opening markets/construction). citeturn33view4turn33view0  
* **cân nhắc (Consider):** no veto triggers, but positives are moderate or there is mixed evidence. Use especially when hoàng đạo/hắc đạo is favorable but primary virtue stars are absent (to avoid overclaiming). citeturn28view2turn27view0  
* **không nên làm (Not recommended):** negative score (multiple medium negatives) but not a hard taboo.  
* **kiêng kỵ / đại kỵ (Taboo):** any hard taboo trigger (global or activity-specific) OR “百事忌” day sets (if you adopt that layer) OR explicit “忌” that Xieji treats as non-overridable for that activity. citeturn26view0turn37view3turn33view3

## Evidence table

The table below provides exemplars rather than an exhaustive cross-product of all signals × all activities. It is structured so an engineer can map each row to (signal → affects activity → direction → strength), and then extend.

| signal | activity | recommendation direction | strength | source | source type | notes / conflicts |
|---|---|---|---|---|---|---|
| Trực “成日 / 開日” | 入學 (học hành/nhập học) | favor | strong | Xieji 卷十一 lists 入學 “宜成日開日”. citeturn34view0 | court-standard selection manual | Good example of “direct → specific activity.” |
| Trực “定日” | 冠帶 (formal attire/ritual milestones) | favor | strong | Xieji 卷十一: 冠帶 “宜定日”. citeturn34view0 | court-standard | Useful proxy for “milestone/ceremony” category. |
| Hoàng đạo six gods (青龍…司命) | general | amplify only | medium | Xieji says six hoàng đạo should follow what other gods indicate; no exclusive actions. citeturn28view2 | court-standard | Treat as context, not a full ruleset. |
| 月建 (Month Build) | 祈福/求嗣 | forbid | strong | Xieji 卷十一 lists month-build among 祈福/求嗣 “忌”. citeturn34view0 | court-standard | Veto-like for many major actions. |
| 月破 (Month Break) | 開市 (opening business) | forbid | strong | Xieji 卷十一: 開市 “忌月破…大耗…” etc. citeturn33view4 | court-standard | Treat month-break as high severity for commerce. |
| 土王用事 | construction (動土/修造) | forbid | strong | Xieji 卷十: 土王用事 “忌” for construction/earthworks and more. citeturn26view0 | court-standard | “Seasonal soil king” periods are hard-blockers. |
| 四離四絕 | most activities | forbid | strong | Xieji 卷十: 上朔/四離/四絕/晦 → most activities taboo; exceptions listed. citeturn26view0 | court-standard | Non-overridable even with 德合/赦願. |
| 朔弦望日 | 求醫療病 | forbid | strong | Xieji 卷十一: 求醫療病 “忌…朔弦望日”. citeturn33view3 | court-standard | Activity-specific hard taboo. |
| 天醫 | 求醫療病 | favor | strong | Xieji 卷十一 includes 天醫 among “宜” for 求醫療病. citeturn33view3 | court-standard | Implement as “medical-positive star.” |
| 天喜 | marriage cluster | favor | strong | Xieji uses 天喜 frequently in marriage-family and celebratory clusters. citeturn34view0turn28view1 | court-standard | “Joy” star: strong positive for weddings/celebrations. |
| 不將 | 嫁娶 | favor | strong | Xieji: 嫁娶 “宜…不將”. citeturn34view0turn28view2 | court-standard | Special-purpose positive for marriage. |
| 三合 | marriage, commerce, construction | favor | strong | Xieji lists broad “宜” sets for 三合. citeturn28view1 | court-standard | Consider as major positive modifier. |
| 六合 | marriage, contracts, burial | favor | medium-strong | Xieji lists 六合 “宜宴會…嫁娶…立券交易…安葬”. citeturn28view2 | court-standard | Strong positive but not a veto-breaker. |
| 鳴吠 / 鳴吠對 | 破土/安葬/啟攢 | favor | strong | Xieji 卷十一: 破土 “宜鳴吠鳴吠對”; 安葬 includes 鳴吠. citeturn33view2 | court-standard | Funeral-specific enabling signals. |
| 彭祖百忌 (stem/branch taboos) | e.g., 甲日開倉, 亥日嫁娶 | forbid | medium-strong | Yuxia gives explicit “甲不開倉…亥不嫁娶…”. citeturn38view1 | compilation/folk-layer | Implement as explicit table; treat as warnings or veto depending on product stance. |
| 楊公忌日 | marriage, construction, burial | forbid | strong | Yuxia: “百事忌” + explicit warnings for building/marriage/burial. citeturn37view3 | compilation/folk-layer | Widely used taboo set; not clearly shown in Xieji excerpt—tag as “folk-layer.” |
| 月忌日 (5, 14, 23) | general | forbid | strong | Yuxia: “百事忌” and warns outcomes. citeturn37view3 | compilation/folk-layer | Very common in Vietnam (mùng 5, 14, 23). |
| 地支六沖 (day clashes month/year) | major actions | forbid / downgrade | medium-strong | Yuxia defines “clash with month or year-lord = inauspicious,” lists pairs. citeturn31view0 | compilation/folk-layer | Implement as general negative; consider stronger when day clashes month/year in your engine. |
| 28 lunar mansion (e.g., explicit “burial not allowed”) | burial/construction | forbid | strong | Yuxia “二十八宿值日吉凶歌” includes categorical prohibitions. citeturn12view1 | compilation/folk-layer | Versioned rules recommended due to tradition variance. |
| Conflict rule: “good outweighs bad → follow 宜” except “德猶忌” | all | resolution rule | core | Xieji “鋪註條例” describes how to annotate when signals conflict. citeturn26view0turn28view2 | court-standard | Implement as algorithmic precedence. |

## Conservative implementation proposal

This proposal assumes you want a “minimal credible v1” that is defensible, consistent, and avoids false authority.

### Minimal credible v1 signals

**Highly implementation-ready (clear, explicit, repeatedly used; low ambiguity):**  
Trực (建除十二神) as a primary selector for activity categories; Xieji and Yuxia both provide direct→activity clues and Xieji uses direct explicitly in “用事.” citeturn27view0turn34view0turn38view1  
Month build/break (月建/月破) as hard negatives across many major activities (marriage, commerce, construction). citeturn28view2turn33view4turn33view0  
Hard-taboo sets from Xieji: 上朔/晦 and 四離四絕, plus 土王用事 for earthworks. citeturn26view0  
Core “virtue” positives: 天德/月德 (+合), 天赦, 天願, 月恩/四相/時德 as the base of “recommended day” logic. citeturn34view0turn33view0turn33view4  
A small set of activity-specific “must-have” stars: 天醫 for medical, 天喜/不將 for wedding, 鳴吠/鳴吠對 for burial/repair of graves. citeturn33view3turn33view2turn34view0

**Useful but should be treated as optional/secondary in v1:**  
Hoàng đạo/hắc đạo as a UI layer and mild amplifier, not a sole decision. citeturn28view2turn14view0  
28 lunar mansions as optional “traditional layer” with versioning and user-visible “source tradition” labels. citeturn12view1turn11view0  
Pengzu hundred taboos (彭祖百忌) as warnings that can optionally hard-block in strict mode. citeturn38view1turn26view0

**Signals to delay unless you can source and test them carefully:**  
Broad, long-tail “month sha” families beyond the core set (because there are many, naming overlaps, and different tongshu editions differ). Xieji uses a lot of them, but implementing them all without robust validation risks contradiction. citeturn33view0turn33view4  
Highly localized folk taboos not clearly present in your chosen “normative base” (e.g., if you can’t anchor them in Xieji or a clearly identified Vietnamese tradition source, treat them as “folk add-on packs” rather than core). Xieji explicitly critiques older calendars for mismatching gods’ names and meanings and tries to correct them; that implies you should be careful about uncontrolled accretions. citeturn28view2turn26view0

### Defaults when sources disagree

Use **explicit versioning**:

* **Core (Xieji-based)** mode: treat Xieji’s “用事/宜忌/鋪註條例” as the default logic backbone. citeturn34view0turn26view0  
* **Folk-almanac add-ons**: Yuxia-based taboo day packs (楊公忌日, 月忌日, Pengzu, mansion verses) can be enabled as optional layers, clearly labeled “folk tongshu tradition.” citeturn37view3turn38view1turn12view1  

### Responsible labeling

To avoid false authority, every recommendation should carry:

* **Scope label:** “General day-selection tradition” vs “activity-specific rule.” Xieji itself is activity-specific (用事). citeturn34view0  
* **Tradition label:** “Court-standard selection manual (Xieji)” vs “Tongshu compilation (Yuxia).” citeturn39view0turn35search1  
* **Strength label:** “taboo / avoid / neutral / favorable” (not “guaranteed luck”).  
* **Conflict note:** if a positive is present but a small negative also exists, present as “cân nhắc” with the negative shown.

## Open questions

These are high-impact unknowns or inconsistencies you should resolve before claiming a “complete” Vietnamese recommendation engine.

**Vietnam-local canonicalization:** Modern Vietnamese almanac products vary in which Chinese-derived layers they include and how they rename or simplify them. Without a curated set of Vietnamese print almanacs to compare, you risk shipping rules that feel “Chinese” but not “Vietnamese-as-used.” (Your system can mitigate by making your rule-base transparent and configurable.) citeturn6search3turn6search4turn25search1  

**Which folk taboos are truly “Vietnam mainstream” vs internet amplification:** Some taboo-day sets are extremely popular (e.g., monthly 5/14/23), but the degree to which they are treated as absolute varies by community and product. Yuxia contains these as “百事忌,” but Xieji’s stance on them (accept/reject) needs deeper full-text study if you want to claim “court-standard” endorsement. citeturn37view3turn26view0  

**28 mansion rule variants:** Mansion verses are high-signal but notoriously variant across lineages (and some calendars use different computational anchoring). If your computed mansion system differs from the verse’s assumed mapping, results will look “wrong” to users. citeturn12view1turn27view0  

**Personalized vs date-wide guidance:** Vietnamese users often expect “xung tuổi” output (day clashes the user’s age, bride/groom/housing owner) as decisive. The classical texts distinguish many date-wide rules from person-specific matching. You will need a product decision: do you ship “day-only guidance,” or require user birth-year (or full bazi) for high-stakes categories? citeturn31view0turn34view0turn6search0  

## Final deliverables

### Ranked implementation-ready signals

Ranked for (a) textual clarity, (b) practical usage, (c) engineering feasibility, and (d) conflict-resolvability:

1. **Trực (建除十二神 / 12 day officers)** as the primary day-type selector (strong, explicit for many activities). citeturn34view0turn27view0turn38view1  
2. **Month build/break (月建 / 月破)** as high-severity avoid conditions across major actions. citeturn33view4turn34view0turn28view2  
3. **Hard taboo framework from Xieji**: 上朔/晦 + 四離四絕 + 土王用事 (strong veto rules with explicit exceptions). citeturn26view0  
4. **Core virtue stars**: 天德/月德 (+合), 天赦, 天願, 月恩/四相/時德 (broad positive backbone). citeturn34view0turn33view0turn28view2  
5. **Activity-specific flagship stars**: 天喜/不將 (marriage), 天醫 (medical), 鳴吠/鳴吠對 (burial). citeturn34view0turn33view3turn33view2  
6. **三合/六合/五合** as strong positive modifiers. citeturn28view1turn28view2  
7. **Hoàng đạo/hắc đạo day gods** as an explanatory UI layer (medium; avoid using alone). citeturn28view2turn14view0  
8. **Pengzu hundred taboos + monthly “百事忌” sets** as optional “folk layer packs” (high user recognition, but should be labeled as such). citeturn38view1turn37view3  
9. **28 lunar mansions** as optional/advanced with versioning and careful validation. citeturn12view1turn11view0  

### Proposed recommendation schema in JSON-like form

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
    "tietKhi": { "name_vi": "Kinh Trập", "name_zh": "驚蟄", "isNodeDay": false }
  },
  "signals": {
    "truc": { "vi": "Khai", "zh": "開", "rank": "high" },
    "hoangDao": { "isHoangDao": true, "gods": ["青龍", "明堂"] },
    "lunarMansion": { "vi": "Tâm", "zh": "心宿" },
    "relations": {
      "dayVsMonth": { "xung": false, "hop": "六合", "details": ["子丑合?"] },
      "dayVsYear": { "xung": true, "pair": "子午冲" }
    },
    "taboos": {
      "xieji": {
        "isSiLiSiJue": false,
        "isShangShuo": false,
        "isHui": false,
        "isTuWangYongShi": false,
        "isYueJian": false,
        "isYuePo": true
      },
      "folkPacks": {
        "isYangGongJi": false,
        "isYueJi_5_14_23": false,
        "pengzu": [{ "key": "甲", "rule": "不開倉" }]
      }
    },
    "stars": {
      "good": ["天德", "月德", "天赦", "天願", "天喜"],
      "bad": ["劫煞", "月厭", "大時", "天吏", "四廢"]
    }
  },
  "activities": {
    "marriage": {
      "verdict": "avoid",
      "levels": { "should": false, "consider": false, "avoid": true, "taboo": false },
      "reasons": [
        { "type": "veto", "signal": "月破", "source": "Xieji卷11" },
        { "type": "positive", "signal": "天喜", "source": "Xieji卷11" }
      ],
      "confidence": "medium",
      "traditionLabels": ["Xieji-core"]
    }
    // ...
  },
  "globalSummary": {
    "nenLam": ["low-stakes maintenance", "ritual/cleaning if allowed"],
    "canNhac": ["social/learning tasks"],
    "khongNenLam": ["market opening", "contracts"],
    "kiengKy": ["construction/earthworks if 土王用事", "medical if 朔弦望"]
  }
}
```

This schema mirrors the source structure: (a) compute signals, (b) evaluate per activity using explicit “宜/忌,” and (c) output a verdict and traceable reasons. The “trace” field is essential for responsible UX, reflecting Xieji’s emphasis on distinguishing “light/heavy” and making “choice discernible.” citeturn34view0turn26view0

### Sample rule model for five activity categories

These samples illustrate how to translate the classical “宜/忌” into code-friendly rules, using **Xieji core** as default and optionally adding **Yuxia folk packs**.

#### Marriage

Rule skeleton (Xieji-core):
* Require at least one of: 天德/月德/天赦/天願 (or equivalents)  
* Prefer: 三合 + 六合/五合; include 不將  
* Veto: 月建, 月破, 平日, 收日, 閉日, plus “major month-sha cluster” (劫煞/災煞/月厭…) citeturn34view0turn26view0turn28view2  
Optional folk-pack veto: 楊公忌日, 月忌日(5/14/23). citeturn37view3

#### Construction

Rule skeleton (Xieji-core):
* Prefer: 天德/月德/天赦/天願 + 月恩/四相/時德 + 三合 + 開日  
* Veto: 土王用事, 月建/土府, 月破, 平/收/閉, 土符/地囊, 四廢/五墓. citeturn33view0turn26view0turn28view2  
Optional: if 28-mansion layer enabled, apply mansion-specific “build/construct” veto list. citeturn12view1

#### Moving house

Rule skeleton (Xieji-core):
* Prefer: virtue set + 民日 + 驛馬/天馬 + 成日/開日  
* Avoid/Veto: 月破, 平/收/閉, 歸忌/往亡. citeturn34view0  
If user profile available, also veto day that xung with household head’s year-branch.

#### Opening business / signing contracts

Rule skeleton (Xieji-core):
* Opening market (開市) prefer: 天願 + 民日 + 滿/成/開 + 五富  
* Veto: 月破 + 大耗/小耗 + 平/收/閉 + “空” (九空) and month-sha cluster  
* Contracts (立券交易) prefer: 天願 + 民日 + 三合 + 六合 + 五合  
* Note: Xieji suppresses some “宜” outputs when a known hard conflict exists (caller should implement suppression patterns). citeturn33view4turn26view0turn28view2

#### Medical treatment

Rule skeleton (Xieji-core):
* Prefer: virtue set + 天醫 + (除日/破日/開日) + 解神/除神  
* Veto: 朔弦望日, monthly 15th, 未日 (and the common month-sha cluster) citeturn33view3turn26view0  
Optional: Yuxia “服藥/合藥” lists can be added as a specialized sub-module, but should be clearly labeled as a different tradition layer. citeturn13view3

### Bibliography

Vietnamese sources (practice-facing, modern compilation)
* *Hoàng lịch năm 2009–2011: xem ngày tốt xấu theo lịch dân gian* (catalog record). citeturn6search3  
* *Nguyên lý chọn ngày theo lịch Can chi* (multi-edition Vietnamese manual; listing). citeturn6search4  
* *Thông Thư Vạn Sự – Quảng Tuệ* (Vietnamese compilation; listing). citeturn25search1  
* *Trạch Nhật – Tự Học Chọn Ngày Giờ Cưới Hỏi* (modern Vietnamese-focused selection manual; Google Books record). citeturn6search0  

Chinese sources (primary / high-authority)
* 欽定協紀辨方書 (四庫全書本), especially:  
  * 卷四: 建除十二神 and commentary on variation and non-absolutism. citeturn27view0  
  * 卷十: 宜忌 + 鋪註條例 (conflict/precedence rules; hard taboo constructs). citeturn26view0turn28view2  
  * 卷十一: 用事 (activity list + per-activity 宜/忌). citeturn34view0turn33view0turn33view4turn33view3turn33view2  
* 玉匣記 / 玉匣記通書 (Wikisource edition), particularly:  
  * 二十八宿值日吉凶歌 (mansion-to-activity mapping). citeturn12view1  
  * 黃黑道用事吉日 (practical hoàng/hắc mnemonic + mitigation lore). citeturn14view0  
  * 彭祖百忌日, 楊公忌日, 月忌日 (widely used taboo sets). citeturn38view1turn37view3  
  * 地支六沖凶日 (xung pairs; clash with month/year). citeturn31view0  
* Chinaknowledge.de encyclopedia entry on 協紀辨方書 (overview and structure). citeturn7search4  
* Richard Smith, “The Legacy of Daybooks in Late Imperial and Modern China” (Brill) (historical continuity of rishu → tongshu/huangli). citeturn24search2  

Korean sources (comparative authority/preservation)
* SillokWiki: 협기변방서(協紀辨方書) (bibliographic and institutional usage as exam text; compilation context). citeturn39view0