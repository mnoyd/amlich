# Framework for Deriving Daily Guidance from Lunar Calendar Signals

## Part 1: Research Finding Framework

## A. Source Map

### 1. Vietnamese Sources (Primary)

- **Thong Thu / Van Su (Almanacs):**
  - **Nature:** Commercial/Folk compilation. Annual publications (e.g., Xuan Thuy, various Buddhist temple publications).
  - **Trustworthiness:** High influence in daily practice. While modern editions vary in quality, they are the direct practical application of the tradition.
  - **Signals:** `Truc` (12 Officers), `Nhi thap bat tu` (28 Mansions), `Sao tot/xau` (Good/Bad Stars - minor gods), `Hoang dao/Hac dao`.
- **Dong Cong Tuyen Trach (Dong Gong Selection):**
  - **Nature:** Classical compilation, highly respected in Vietnamese Feng Shui circles.
  - **Trustworthiness:** Authoritative for construction and burial.
  - **Signals:** Heavily emphasizes `Chinh Tinh` (Major Stars) and `Truc`.
- **Vietnamese Scholarly Works (e.g., Phan Ke Binh - "Viet Nam Phong Tuc"):**
  - **Nature:** Ethnographic/Descriptive.
  - **Usage:** Explains the "why" behind folk taboos rather than generating new dates.

### 2. Chinese Sources (Secondary)

- **Tongshu (通书 - The Chinese Almanac):**
  - **Nature:** Commercial/Folk. The standard reference for date selection.
  - **Usage:** The structural ancestor of Vietnamese almanacs.
- **Xieji Bianfang Shu (协纪辨方书 - Imperial Encyclopedia of Time Selection):**
  - **Nature:** Classical/Imperial Compilation (Qing Dynasty).
  - **Trustworthiness:** The "Gold Standard" for conflicting rules. It resolves disputes between different schools of thought.
  - **Signals:** Systematized `Than Sat` (Gods and Spirits), heavy emphasis on `Truc` and relationships between Year/Month/Day branches.

### 3. Korean Sources (Conditional)

- **Daejeon (Great Almanac):**
  - **Value:** Uses the same 28 Mansions and 12 Officers. Generally confirms the East Asian consensus.
  - **Verdict:** Not strictly necessary for a Vietnamese engine as Chinese sources cover the root logic, but useful for verifying specific obscure star calculations.

## B. Signal-by-Signal Analysis

### 1. Truc (Thap nhi truc / The 12 Day Officers)

- **Meaning:** The energy phase of the day relative to the month.
- **Decision Weight:** **STRONG**. This is the most critical filter for general activities.
- **Recommendation Logic:**
  - **Kien (Establish):** Good for starting new, construction. Bad for burial (creates conflict).
  - **Tru (Remove):** Good for cleaning, medical treatment, ending relationships. Bad for starting new business or marriage.
  - **Man (Full):** Good for commerce, warehouses. Bad for burials.
  - **Binh (Level):** Neutral. Good for mundane tasks.
  - **Dinh (Stabilize):** Good for signing contracts, acceptance. Bad for litigation/travel (stuck).
  - **Chap (Hold):** Good for stocking, planning. Bad for starting medical treatment.
  - **Pha (Break):** Generally BAD (Destruction). Exception: Good for demolition, breaking ground, pest control.
  - **Nguy (Danger):** Generally BAD. High risk. Good for climbing heights (ritualistically) but generally avoided for travel.
  - **Thanh (Success):** Very GOOD. Good for marriage, business, construction.
  - **Thu (Receive):** Good for receiving awards, acquiring assets. Bad for funerals (cannot "receive" death).
  - **Khai (Open):** Very GOOD. Good for opening shops, business, marriage.
  - **Be (Close):** Generally BAD. Closed doors. Good for closing accounts, burying the dead (closing the cycle).

### 2. Nhi thap bat tu (28 Mansions)

- **Meaning:** The constellation the moon transits that day.
- **Decision Weight:** **STRONG**. Determines the "nature" of the day.
- **Recommendation Logic:**
  - **Classification:** 13 Good (Cat), 13 Bad (Hung), 2 Neutral.
  - **Example - Giac (Horn):** Good for marriage, construction, opening business. Bad for burial.
  - **Example - Cang (Neck):** Bad for marriage (conflict), but good for some construction if combined with lucky stars.
  - **Interaction:** Often overrides general day quality. A "Yellow Dao" day with a "Hung" Mansion is often downgraded to "Can nhac" (Consider).

### 3. Hoang dao / Hac dao (Yellow/Black Dao)

- **Meaning:** Duty Gods governing the day (Thanh Long, Kim Duong, etc.).
- **Decision Weight:** **MEDIUM**. A general filter.
- **Logic:**
  - **Hoang dao (6 days):** Good for major events.
  - **Hac dao (6 days):** Avoid major events, especially marriages and construction.

### 4. Tiet Khi (Solar Terms)

- **Meaning:** Seasonal transitions (e.g., Lap Xuan, Dong Chi).
- **Decision Weight:** **SPECIAL (Contextual)**.
- **Logic:**
  - Days exactly *on* the transition (Giao tiet) are often considered unstable. **Avoid:** Major construction, marriage, long travel.
  - Seasons dictate elemental strength (e.g., avoid Earth days in Wood season if weak).

## C. Activity Taxonomy

| Category | Vietnamese | Key Signals | Typical "Taboo" (Kieng ky) |
| :--- | :--- | :--- | :--- |
| **Marriage** | Cuoi hoi | Truc (Thanh, Khai, Dinh), Tu (Good), User Year | Truc Pha, Truc Tru (divorce risk), Day clashing with Bride/Groom year. |
| **Construction** | Dong tho, Xay dung | Truc (Kien, Thanh), Tu (Good), Direction | Truc Pha (unless demolishing), Tam Sat at direction. |
| **Moving House** | Nhap trach | Truc (Thanh, Khai, Man), Hoang dao | Truc Be (stuck), Truc Nguy (risk). |
| **Funeral** | An tang | Truc (Be, Thu - specific schools), Tu (specific) | Truc Kien (disturbs peace), Truc Man. |
| **Business Open** | Khai truong | Truc (Khai, Man, Thanh), Tu (Good wealth) | Truc Be, Hac dao. |
| **Travel** | Xuat hanh | Truc (Kien, Thanh), Hoang dao | Truc Nguy (Danger), Truc Dinh (Stuck). |

## D. Recommendation Logic Draft

**Software Logic Flow:**

1. **Input:** Day Info (`Truc`, `Tu`, `Dao`) + Activity Category.
2. **Layer 1: Hard Filters (Kieng ky / Taboo).**
   - Is the `Truc` compatible with Activity? (e.g., `Activity == Marriage && Truc == Tru` => **TABOO**).
   - Is the `Tu` explicitly bad for Activity? (e.g., `Activity == Burial && Tu == Giac` => **TABOO**).
3. **Layer 2: Quality Grading (Nen lam / Can nhac).**
   - If not Taboo, calculate Score.
   - `Score = (Truc Weight) + (Tu Weight) + (Dao Weight)`.
   - High Score => **Nen lam**.
   - Medium Score => **Can nhac**.
4. **Layer 3: Specific Advice.**
   - If `Truc == Tru` and `Activity == Medical` => **Good for curing/ending illness**.
   - If `Truc == Pha` and `Activity == Construction` => **Good for demolition only**.

## E. Evidence Table (Sample)

| Signal | Activity | Direction | Strength | Source Basis | Notes |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Truc Tru** | Marriage | **Avoid** | Strong | Tongshu/Van Su | Meaning "Removal/Divorce". Bad for starting union. |
| **Truc Pha** | Construction | **Avoid** (Start) | Strong | Classical | "Break". Only good for demolition/renovation. |
| **Truc Thanh** | Business | **Do** | Strong | Classical | "Success". Best for opening, signing. |
| **Tu: Giac** | Burial | **Avoid** | Medium | 28 Mansions | Good for living, bad for dead. |
| **Tu: Thi** | Marriage | **Do** | Medium | 28 Mansions | "Roof/House". Good for family/marriage stability. |

## F. Conservative Implementation Proposal (v1)

**Safe Signals to Include:**

1. **Truc (12 Officers):** Clear, binary rules available.
2. **Nhi thap bat tu (28 Mansions):** Standard lists of Good/Bad available.
3. **Hoang/Hac Dao:** Simple boolean.

**Signals to Defer:**

1. **Complex Than Sat (Gods/Spirits):** There are hundreds. Only implement major ones (Thien Duc, Nguyet Duc) in v1.
2. **Personalization (User Age):** Requires user birth year input and complex clash logic (Xung/Nhi Hop). Keep v1 date-centric.

**Labeling Strategy:**

- Do not use absolute terms like "You will fail."
- Use:
  - **Dai cat (Great Luck):** Strong signal match.
  - **Tot (Good):** Positive signal match.
  - **Can nhac (Consider):** Mixed signals (Good Truc, Bad Tu).
  - **Khong nen (Avoid):** Strong negative signal.
  - **Kieng ky (Taboo):** Classical prohibition.

## G. Open Questions

1. Regional Variations: Vietnamese Northern vs. Southern almanacs sometimes differ on minor stars.
2. Truc Calculation: Standard calculation uses the Month Branch, but some modern apps use Solar terms strictly. Need to stick to the engine's core calculation method.
3. Burial Logic: Highly complex and sensitive. v1 might just label "Consult Expert" rather than auto-recommending burial dates.

## H. Final Deliverables

### 1. Ranked Implementation Signals

1. **Truc (Day Officer)** - Highest impact on specific actions.
2. **Nhi thap bat tu (28 Mansions)** - Specific activity taboos.
3. **Hoang dao / Hac dao** - General day quality baseline.

### 2. Recommendation Schema (JSON)

```json
{
  "date": "2024-03-15",
  "quality_score": 8,
  "general_verdict": "HOANG_DAO",
  "signals": {
    "truc": "THANH",
    "tinh_tu": "Giac",
    "dao": "THANH_LONG"
  },
  "guidance": [
    {
      "activity_category": "MARRIAGE",
      "verdict": "NEN_LAM",
      "reason": "Truc Thanh (Success) is excellent for marriage.",
      "strength": "HIGH"
    },
    {
      "activity_category": "CONSTRUCTION",
      "verdict": "NEN_LAM",
      "reason": "Truc Thanh and Tu Giac support construction.",
      "strength": "HIGH"
    },
    {
      "activity_category": "FUNERAL",
      "verdict": "KIENG_KY",
      "reason": "Tu Giac is traditionally taboo for burial.",
      "strength": "MEDIUM"
    }
  ]
}
```

### 3. Bibliography

- *Van Su KIM CO*, Annual Almanac.
- *Xieji Bianfang Shu* (协纪辨方书), Imperial Compilation.
- *Tu Vi Dau So* logic for Star interactions (reference only).

## Part 2: Recommendation Engine in Rust

This Rust code implements the logic defined in the research above. It focuses on a rule-based engine where `Truc` and `Nhi thap bat tu` are the primary decision factors.

### Rust Implementation Strategy

1. **Enums**: Define the domain types (`Truc`, `Tu`, `Activity`, `Verdict`).
2. **Knowledge Base**: Store the traditional rules in structs or lazy static maps (simulating a database).
3. **Engine**: A function that takes the day's signals and requested activity, checks the rules, and returns a recommendation.

### Cargo.toml

(No external dependencies required for this core logic, but `serde` would be useful for JSON output in a real app).

### main.rs

```rust
use std::collections::HashMap;

// --- 1. Domain Models ---

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Truc {
    Kien,  // Establish
    Tru,   // Remove
    Man,   // Full
    Binh,  // Level
    Dinh,  // Stabilize
    Chap,  // Hold
    Pha,   // Break
    Nguy,  // Danger
    Thanh, // Success
    Thu,   // Receive
    Khai,  // Open
    Be,    // Close
}

impl Truc {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "kiến" | "kien" => Some(Truc::Kien),
            "trừ"  | "tru"  => Some(Truc::Tru),
            "mãn"  | "man"  => Some(Truc::Man),
            "bình" | "binh" => Some(Truc::Binh),
            "định" | "dinh" => Some(Truc::Dinh),
            "chấp" | "chap" => Some(Truc::Chap),
            "phá"  | "pha"  => Some(Truc::Pha),
            "nguy" | "nguy" => Some(Truc::Nguy),
            "thành"| "thanh"=> Some(Truc::Thanh),
            "thu"  | "thu"  => Some(Truc::Thu),
            "khai" | "khai" => Some(Truc::Khai),
            "bế"   | "be"   => Some(Truc::Be),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Activity {
    Marriage,      // Cuoi hoi
    Construction,  // Dong tho
    MovingHouse,   // Nhap trach
    Funeral,       // An tang
    BusinessOpen,  // Khai truong
    Travel,        // Xuat hanh
}

// Weighted score for an activity
struct ActivityRule {
    // Score adjustment. Positive = Good, Negative = Bad.
    // Range roughly -100 to 100
    score: i32,
    // Specific note if needed
    note: &'static str,
}

// Final Verdict
#[derive(Debug)]
enum Verdict {
    GreatLuck,   // Dai cat
    Good,        // Nen lam
    Consider,    // Can nhac
    Avoid,       // Khong nen
    Taboo,       // Kieng ky
}

// --- 2. The Knowledge Base (Simulated) ---

// Rules for Truc (Simplified for demonstration)
// In production, this would be loaded from a JSON/Database.
fn get_truc_rules() -> HashMap<(Truc, Activity), ActivityRule> {
    let mut map = HashMap::new();

    // Marriage Rules
    map.insert((Truc::Thanh, Activity::Marriage), ActivityRule { score: 100, note: "Truc Thanh: Marriage leads to success." });
    map.insert((Truc::Khai, Activity::Marriage), ActivityRule { score: 80, note: "Truc Khai: Open to new beginnings." });
    map.insert((Truc::Tru, Activity::Marriage), ActivityRule { score: -80, note: "Truc Tru: Risk of separation/divorce." });
    map.insert((Truc::Pha, Activity::Marriage), ActivityRule { score: -100, note: "Truc Pha: Destruction. Strictly taboo." });

    // Construction Rules
    map.insert((Truc::Kien, Activity::Construction), ActivityRule { score: 90, note: "Truc Kien: Good for establishing structure." });
    map.insert((Truc::Thanh, Activity::Construction), ActivityRule { score: 90, note: "Truc Thanh: Successful completion." });
    map.insert((Truc::Pha, Activity::Construction), ActivityRule { score: -50, note: "Truc Pha: Good only for demolition, bad for starting." });
    map.insert((Truc::Nguy, Activity::Construction), ActivityRule { score: -70, note: "Truc Nguy: Danger. Avoid construction." });

    // Travel Rules
    map.insert((Truc::Kien, Activity::Travel), ActivityRule { score: 80, note: "Truc Kien: Smooth journey." });
    map.insert((Truc::Dinh, Activity::Travel), ActivityRule { score: -40, note: "Truc Dinh: Stuck. Movement is difficult." });
    map.insert((Truc::Nguy, Activity::Travel), ActivityRule { score: -90, note: "Truc Nguy: Danger. Avoid travel." });

    // Business Rules
    map.insert((Truc::Khai, Activity::BusinessOpen), ActivityRule { score: 100, note: "Truc Khai: Best for opening business." });
    map.insert((Truc::Be, Activity::BusinessOpen), ActivityRule { score: -90, note: "Truc Be: Closed doors. Bad for opening." });

    map
}

// --- 3. Engine Logic ---

struct DayContext {
    truc: Truc,
    // Add other signals here (Tu, Dao, etc.)
}

struct RecommendationEngine {
    truc_rules: HashMap<(Truc, Activity), ActivityRule>,
}

impl RecommendationEngine {
    fn new() -> Self {
        Self {
            truc_rules: get_truc_rules(),
        }
    }

    fn evaluate(&self, ctx: &DayContext, activity: Activity) -> (Verdict, String) {
        // Step 1: Check Truc Rules
        if let Some(rule) = self.truc_rules.get(&(ctx.truc, activity)) {
            // Determine verdict based on score
            let verdict = if rule.score >= 70 {
                Verdict::GreatLuck
            } else if rule.score >= 20 {
                Verdict::Good
            } else if rule.score <= -70 {
                Verdict::Taboo
            } else if rule.score <= -20 {
                Verdict::Avoid
            } else {
                Verdict::Consider
            };

            return (verdict, rule.note.to_string());
        }

        // Step 2: Default / Fallback logic
        // If no specific rule found, we might check general luck or return neutral
        (Verdict::Consider, "No specific strong signal for this activity.".to_string())
    }
}

// --- 4. Execution Example ---

fn main() {
    let engine = RecommendationEngine::new();

    // Test Case 1: A day with Truc Thanh (Success)
    let day_success = DayContext { truc: Truc::Thanh };

    println!("--- Day: Truc Thanh ---");
    check_activity(&engine, &day_success, Activity::Marriage);
    check_activity(&engine, &day_success, Activity::Construction);

    // Test Case 2: A day with Truc Pha (Break)
    let day_break = DayContext { truc: Truc::Pha };

    println!("\n--- Day: Truc Pha ---");
    check_activity(&engine, &day_break, Activity::Marriage);
    check_activity(&engine, &day_break, Activity::Construction); // Note: Logic shows negative for start
}

fn check_activity(engine: &RecommendationEngine, ctx: &DayContext, activity: Activity) {
    let (verdict, reason) = engine.evaluate(ctx, activity);
    println!("Activity: {:?} -> Verdict: {:?} (Reason: {})", activity, verdict, reason);
}
```
