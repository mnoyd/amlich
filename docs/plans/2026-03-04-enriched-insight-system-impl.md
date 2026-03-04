# Enriched Insight System Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Expand DayInsightDto to surface all computed almanac subsystems with bilingual interpretive text, driven by user profile for birth-dependent features.

**Architecture:** The insight builder (`get_day_insight`) gains access to computed `DayFortune` data and merges it with bilingual interpretation text from new JSON data files. A user profile config at `~/.config/amlich/profile.json` enables birth-dependent features (Kua, Dai Van). All new fields are `Option<T>` for backward compatibility.

**Tech Stack:** Rust (amlich-core, amlich-api, amlich CLI), serde/serde_json, OnceLock + include_str! pattern, ratatui for TUI rendering.

**Design doc:** `docs/plans/2026-03-04-enriched-insight-system-design.md`

---

## Task 1: User Profile Module

**Files:**
- Create: `crates/amlich/src/profile.rs`
- Modify: `crates/amlich/src/main.rs` (add `mod profile;`)

**Step 1: Write failing test for profile load/save**

In `crates/amlich/src/profile.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_empty_when_no_file() {
        let profile = load_profile_from_str("{}");
        assert!(profile.birth_year.is_none());
        assert!(profile.gender.is_none());
    }

    #[test]
    fn loads_full_profile() {
        let json = r#"{"birth_year":1990,"birth_month":5,"birth_day":15,"gender":"male"}"#;
        let profile = load_profile_from_str(json);
        assert_eq!(profile.birth_year, Some(1990));
        assert_eq!(profile.birth_month, Some(5));
        assert_eq!(profile.birth_day, Some(15));
        assert_eq!(profile.gender, Some(ProfileGender::Male));
    }

    #[test]
    fn partial_profile_ok() {
        let json = r#"{"birth_year":1990}"#;
        let profile = load_profile_from_str(json);
        assert_eq!(profile.birth_year, Some(1990));
        assert!(profile.gender.is_none());
    }

    #[test]
    fn has_birth_context_requires_year_and_gender() {
        let full = load_profile_from_str(
            r#"{"birth_year":1990,"gender":"female"}"#,
        );
        assert!(full.has_birth_context());

        let partial = load_profile_from_str(r#"{"birth_year":1990}"#);
        assert!(!partial.has_birth_context());
    }
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test -p amlich --lib profile`
Expected: FAIL (module doesn't exist)

**Step 3: Implement profile module**

```rust
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProfileGender {
    Male,
    Female,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_month: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_day: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<ProfileGender>,
}

impl UserProfile {
    pub fn has_birth_context(&self) -> bool {
        self.birth_year.is_some() && self.gender.is_some()
    }
}

fn profile_path() -> Option<PathBuf> {
    dirs::config_dir().map(|dir| dir.join("amlich").join("profile.json"))
}

pub fn load_profile_from_str(json: &str) -> UserProfile {
    serde_json::from_str(json).unwrap_or_default()
}

pub fn load_profile() -> UserProfile {
    let Some(path) = profile_path() else {
        return UserProfile::default();
    };
    match std::fs::read_to_string(path) {
        Ok(content) => load_profile_from_str(&content),
        Err(_) => UserProfile::default(),
    }
}

pub fn save_profile(profile: &UserProfile) -> Result<(), String> {
    let Some(path) = profile_path() else {
        return Err("could not determine config directory".to_string());
    };
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("failed to create config directory: {e}"))?;
    }
    let json = serde_json::to_string_pretty(profile)
        .map_err(|e| format!("failed to serialize profile: {e}"))?;
    std::fs::write(&path, json)
        .map_err(|e| format!("failed to write profile: {e}"))
}
```

Add `mod profile;` to `crates/amlich/src/main.rs` near the other mod declarations (around line 4).

**Step 4: Run tests to verify they pass**

Run: `cargo test -p amlich --lib profile`
Expected: PASS (4 tests)

**Step 5: Commit**

```bash
git add crates/amlich/src/profile.rs crates/amlich/src/main.rs
git commit -m "feat: add user profile module for birth context configuration"
```

---

## Task 2: Profile CLI Commands

**Files:**
- Modify: `crates/amlich/src/main.rs` (ConfigCommand enum, run_config fn)

**Step 1: Write CLI contract test**

In `crates/amlich/tests/cli_contract.rs`, add:

```rust
#[test]
fn config_profile_show_succeeds() {
    let output = Command::cargo_bin("amlich")
        .unwrap()
        .args(["config", "profile", "show"])
        .output()
        .unwrap();
    assert!(output.status.success());
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p amlich --test cli_contract config_profile_show`
Expected: FAIL (subcommand not recognized)

**Step 3: Add Profile subcommand to ConfigCommand**

In `crates/amlich/src/main.rs`, extend `ConfigCommand` enum (around line 413):

```rust
#[derive(Subcommand, Debug)]
enum ConfigCommand {
    Mode(ModeArgs),
    Profile(ProfileSubArgs),
}

#[derive(Args, Debug)]
struct ProfileSubArgs {
    #[command(subcommand)]
    command: ProfileCommand,
}

#[derive(Subcommand, Debug)]
enum ProfileCommand {
    Show,
    Set {
        #[arg(long)]
        birth_year: Option<i32>,
        #[arg(long)]
        birth_month: Option<i32>,
        #[arg(long)]
        birth_day: Option<i32>,
        #[arg(long)]
        gender: Option<String>,
    },
    Clear,
}
```

In `run_config`, add the `Profile` arm:

```rust
ConfigCommand::Profile(sub) => match sub.command {
    ProfileCommand::Show => {
        let profile = crate::profile::load_profile();
        let json = serde_json::to_string_pretty(&profile)
            .map_err(|e| format!("failed to serialize: {e}"))?;
        println!("{json}");
    }
    ProfileCommand::Set { birth_year, birth_month, birth_day, gender } => {
        let mut profile = crate::profile::load_profile();
        if let Some(y) = birth_year { profile.birth_year = Some(y); }
        if let Some(m) = birth_month { profile.birth_month = Some(m); }
        if let Some(d) = birth_day { profile.birth_day = Some(d); }
        if let Some(g) = &gender {
            profile.gender = Some(match g.to_lowercase().as_str() {
                "male" | "m" => crate::profile::ProfileGender::Male,
                "female" | "f" => crate::profile::ProfileGender::Female,
                _ => return Err(format!("invalid gender '{g}'; use male or female")),
            });
        }
        crate::profile::save_profile(&profile)?;
        println!("Profile updated.");
    }
    ProfileCommand::Clear => {
        crate::profile::save_profile(&crate::profile::UserProfile::default())?;
        println!("Profile cleared.");
    }
},
```

**Step 4: Run tests**

Run: `cargo test -p amlich --test cli_contract config_profile`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/amlich/src/main.rs
git commit -m "feat: add config profile CLI commands (show/set/clear)"
```

---

## Task 3: Truc Insight Data File

**Files:**
- Create: `crates/amlich-core/data/truc-insight.json`

**Step 1: Create the bilingual data file**

Create `crates/amlich-core/data/truc-insight.json` with entries for each of the 12 Truc officers. Follow existing bilingual pattern. Each entry needs: `id` (matching TRUC_NAMES), `meaning` (vi/en), `good_for` (vi/en arrays), `avoid_for` (vi/en arrays).

```json
{
  "truc": [
    {
      "id": "Kiến",
      "meaning": {
        "vi": "Kiến là ngày khởi đầu, thích hợp cho việc bắt đầu mới, khai trương, động thổ.",
        "en": "Kiến (Establish) is a day of new beginnings, suitable for starting businesses, groundbreaking, and launching ventures."
      },
      "good_for": {
        "vi": ["Khai trương", "Động thổ", "Nhập trạch", "Cầu tài", "Xuất hành"],
        "en": ["Grand opening", "Groundbreaking", "Moving in", "Seeking wealth", "Travel"]
      },
      "avoid_for": {
        "vi": ["Kiện tụng", "An táng"],
        "en": ["Litigation", "Burial"]
      }
    },
    {
      "id": "Trừ",
      "meaning": {
        "vi": "Trừ là ngày trừ bỏ, thích hợp cho việc dọn dẹp, chữa bệnh, giải quyết vấn đề cũ.",
        "en": "Trừ (Remove) is a day of elimination, suitable for cleaning, healing, and resolving old matters."
      },
      "good_for": {
        "vi": ["Chữa bệnh", "Dọn dẹp", "Tẩy uế", "Cắt may", "Giải trừ"],
        "en": ["Medical treatment", "Cleaning", "Purification", "Tailoring", "Resolution"]
      },
      "avoid_for": {
        "vi": ["Khai trương", "Cưới hỏi"],
        "en": ["Grand opening", "Wedding"]
      }
    },
    {
      "id": "Mãn",
      "meaning": {
        "vi": "Mãn là ngày đầy đủ nhưng dễ tràn, nên thận trọng trong mọi việc.",
        "en": "Mãn (Full) is a day of abundance but overflow risk, requiring caution in all matters."
      },
      "good_for": {
        "vi": ["Cầu tài", "Khai trương nhỏ"],
        "en": ["Seeking wealth", "Small openings"]
      },
      "avoid_for": {
        "vi": ["Động thổ", "Xuất hành xa", "Kiện tụng", "An táng"],
        "en": ["Groundbreaking", "Long travel", "Litigation", "Burial"]
      }
    },
    {
      "id": "Bình",
      "meaning": {
        "vi": "Bình là ngày bình ổn, thích hợp cho việc điều hòa, tu sửa, bảo trì.",
        "en": "Bình (Balance) is a day of stability, suitable for adjustments, repairs, and maintenance."
      },
      "good_for": {
        "vi": ["Tu sửa nhà", "Bảo trì", "Giải hòa", "Trồng cây"],
        "en": ["Home repairs", "Maintenance", "Reconciliation", "Planting"]
      },
      "avoid_for": {
        "vi": ["Khai trương", "Cưới hỏi", "Nhập trạch"],
        "en": ["Grand opening", "Wedding", "Moving in"]
      }
    },
    {
      "id": "Định",
      "meaning": {
        "vi": "Định là ngày ổn định, thích hợp cho những quyết định quan trọng và giao kết.",
        "en": "Định (Settle) is a day of determination, suitable for important decisions and agreements."
      },
      "good_for": {
        "vi": ["Cưới hỏi", "Ký hợp đồng", "Giao dịch", "Khai trương", "Nhập trạch"],
        "en": ["Wedding", "Contract signing", "Business deals", "Grand opening", "Moving in"]
      },
      "avoid_for": {
        "vi": ["Kiện tụng", "Xuất hành xa"],
        "en": ["Litigation", "Long travel"]
      }
    },
    {
      "id": "Chấp",
      "meaning": {
        "vi": "Chấp là ngày nắm giữ, thích hợp cho việc thu thập, bảo quản.",
        "en": "Chấp (Grasp) is a day of holding, suitable for collecting and preserving."
      },
      "good_for": {
        "vi": ["Thu hoạch", "Cất giữ", "Sửa chữa", "Bắt thú"],
        "en": ["Harvesting", "Storage", "Repairs", "Hunting"]
      },
      "avoid_for": {
        "vi": ["Khai trương", "Xuất hành", "Cưới hỏi"],
        "en": ["Grand opening", "Travel", "Wedding"]
      }
    },
    {
      "id": "Phá",
      "meaning": {
        "vi": "Phá là ngày phá vỡ, hung nhiều hơn cát, nên tránh việc quan trọng.",
        "en": "Phá (Break) is a day of disruption, more inauspicious than auspicious, avoid important matters."
      },
      "good_for": {
        "vi": ["Phá dỡ", "Tháo gỡ", "Dọn dẹp cũ"],
        "en": ["Demolition", "Dismantling", "Clearing old things"]
      },
      "avoid_for": {
        "vi": ["Cưới hỏi", "Khai trương", "Ký kết", "Nhập trạch", "Động thổ"],
        "en": ["Wedding", "Grand opening", "Signing contracts", "Moving in", "Groundbreaking"]
      }
    },
    {
      "id": "Nguy",
      "meaning": {
        "vi": "Nguy là ngày nguy hiểm, cần thận trọng đặc biệt trong mọi hoạt động.",
        "en": "Nguy (Danger) is a perilous day, requiring special caution in all activities."
      },
      "good_for": {
        "vi": ["Cầu nguyện", "Cúng tế"],
        "en": ["Prayer", "Rituals"]
      },
      "avoid_for": {
        "vi": ["Xuất hành", "Động thổ", "Khai trương", "Cưới hỏi", "Ký kết"],
        "en": ["Travel", "Groundbreaking", "Grand opening", "Wedding", "Signing"]
      }
    },
    {
      "id": "Thành",
      "meaning": {
        "vi": "Thành là ngày thành tựu, vạn sự hanh thông, thích hợp cho mọi việc lớn.",
        "en": "Thành (Succeed) is a day of achievement, everything goes smoothly, suitable for all major activities."
      },
      "good_for": {
        "vi": ["Cưới hỏi", "Khai trương", "Nhập trạch", "Động thổ", "Ký kết", "Xuất hành"],
        "en": ["Wedding", "Grand opening", "Moving in", "Groundbreaking", "Signing", "Travel"]
      },
      "avoid_for": {
        "vi": ["Kiện tụng"],
        "en": ["Litigation"]
      }
    },
    {
      "id": "Thu",
      "meaning": {
        "vi": "Thu là ngày thu lại, thích hợp cho việc kết thúc và thu hoạch.",
        "en": "Thu (Collect) is a day of gathering, suitable for closing matters and harvesting."
      },
      "good_for": {
        "vi": ["Thu hoạch", "Thu nợ", "Kết thúc công việc"],
        "en": ["Harvesting", "Collecting debts", "Closing projects"]
      },
      "avoid_for": {
        "vi": ["Khai trương", "Khởi công", "Cưới hỏi"],
        "en": ["Grand opening", "Starting construction", "Wedding"]
      }
    },
    {
      "id": "Khai",
      "meaning": {
        "vi": "Khai là ngày mở ra, vạn sự tốt lành, thích hợp cho mọi khởi đầu.",
        "en": "Khai (Open) is a day of opening, all things auspicious, suitable for any new beginning."
      },
      "good_for": {
        "vi": ["Khai trương", "Nhập trạch", "Cưới hỏi", "Xuất hành", "Động thổ", "Ký kết"],
        "en": ["Grand opening", "Moving in", "Wedding", "Travel", "Groundbreaking", "Signing"]
      },
      "avoid_for": {
        "vi": ["An táng"],
        "en": ["Burial"]
      }
    },
    {
      "id": "Bế",
      "meaning": {
        "vi": "Bế là ngày đóng lại, không thích hợp cho mọi khởi đầu hay khai mở.",
        "en": "Bế (Close) is a day of closure, not suitable for any new beginnings or openings."
      },
      "good_for": {
        "vi": ["An táng", "Bịt lỗ", "Tu sửa nội bộ"],
        "en": ["Burial", "Sealing", "Internal repairs"]
      },
      "avoid_for": {
        "vi": ["Khai trương", "Cưới hỏi", "Xuất hành", "Nhập trạch", "Động thổ"],
        "en": ["Grand opening", "Wedding", "Travel", "Moving in", "Groundbreaking"]
      }
    }
  ]
}
```

**Step 2: Validate JSON is well-formed**

Run: `python3 -c "import json; json.load(open('crates/amlich-core/data/truc-insight.json'))"`
Expected: No error

**Step 3: Commit**

```bash
git add crates/amlich-core/data/truc-insight.json
git commit -m "data: add bilingual Truc insight data (12 duty officers)"
```

---

## Task 4: Day Deity and Na Am Insight Data Files

**Files:**
- Create: `crates/amlich-core/data/day-deity-insight.json`
- Create: `crates/amlich-core/data/na-am-insight.json`

**Step 1: Create day-deity-insight.json**

Structure: Two entries for HoangDao and HacDao classifications.

```json
{
  "classifications": [
    {
      "id": "HoangDao",
      "name": { "vi": "Hoàng Đạo", "en": "Auspicious Day" },
      "meaning": {
        "vi": "Ngày Hoàng Đạo là ngày tốt lành, được các vị thần bảo hộ. Thích hợp cho mọi việc quan trọng như cưới hỏi, khai trương, xuất hành, ký kết.",
        "en": "Hoang Dao (Yellow Path) is an auspicious day protected by benevolent deities. Suitable for all important activities such as weddings, openings, travel, and contracts."
      }
    },
    {
      "id": "HacDao",
      "name": { "vi": "Hắc Đạo", "en": "Inauspicious Day" },
      "meaning": {
        "vi": "Ngày Hắc Đạo là ngày xấu, nên tránh các việc quan trọng. Chỉ thích hợp cho việc nhỏ hoặc nội bộ.",
        "en": "Hac Dao (Black Path) is an inauspicious day, avoid important activities. Only suitable for minor or internal matters."
      }
    }
  ],
  "deities": [
    {
      "name": "Thanh Long", "classification": "HoangDao",
      "meaning": { "vi": "Thanh Long chủ về phúc lộc và may mắn.", "en": "Azure Dragon governs fortune and good luck." }
    },
    {
      "name": "Minh Đường", "classification": "HoangDao",
      "meaning": { "vi": "Minh Đường chủ về sáng suốt và hanh thông.", "en": "Bright Hall governs clarity and smooth progress." }
    },
    {
      "name": "Kim Quỹ", "classification": "HoangDao",
      "meaning": { "vi": "Kim Quỹ chủ về tài lộc và bảo vệ.", "en": "Golden Vault governs wealth and protection." }
    },
    {
      "name": "Thiên Đức", "classification": "HoangDao",
      "meaning": { "vi": "Thiên Đức chủ về nhân đức và che chở.", "en": "Heavenly Virtue governs benevolence and shelter." }
    },
    {
      "name": "Ngọc Đường", "classification": "HoangDao",
      "meaning": { "vi": "Ngọc Đường chủ về quý hiển và vinh hoa.", "en": "Jade Hall governs nobility and glory." }
    },
    {
      "name": "Tư Mệnh", "classification": "HoangDao",
      "meaning": { "vi": "Tư Mệnh chủ về mệnh vận và phù hộ.", "en": "Fate Commander governs destiny and divine assistance." }
    },
    {
      "name": "Thiên Hình", "classification": "HacDao",
      "meaning": { "vi": "Thiên Hình chủ về hình phạt và tai họa.", "en": "Heavenly Punishment governs penalties and calamity." }
    },
    {
      "name": "Chu Tước", "classification": "HacDao",
      "meaning": { "vi": "Chu Tước chủ về thị phi và tranh cãi.", "en": "Vermilion Bird governs gossip and disputes." }
    },
    {
      "name": "Bạch Hổ", "classification": "HacDao",
      "meaning": { "vi": "Bạch Hổ chủ về tang thương và tổn hại.", "en": "White Tiger governs mourning and loss." }
    },
    {
      "name": "Thiên Lao", "classification": "HacDao",
      "meaning": { "vi": "Thiên Lao chủ về giam cầm và trở ngại.", "en": "Heavenly Prison governs confinement and obstacles." }
    },
    {
      "name": "Huyền Vũ", "classification": "HacDao",
      "meaning": { "vi": "Huyền Vũ chủ về mất mát và bất an.", "en": "Dark Warrior governs loss and insecurity." }
    },
    {
      "name": "Câu Trận", "classification": "HacDao",
      "meaning": { "vi": "Câu Trận chủ về tranh chấp và kiện tụng.", "en": "Hook Formation governs conflicts and litigation." }
    }
  ]
}
```

**Step 2: Create na-am-insight.json**

Structure: 30 Na Am entries (each pair covers 2 of the 60 cycle positions). Include element + nature interpretation.

```json
{
  "pairs": [
    { "na_am": "Hải Trung Kim", "element": "Kim",
      "meaning": { "vi": "Kim trong biển — vàng ẩn dưới biển sâu, quý báu nhưng khó khai thác. Người mệnh này kiên nhẫn và có tiềm năng lớn.", "en": "Gold in the Sea — precious metal hidden deep underwater. People of this destiny are patient with great hidden potential." } },
    { "na_am": "Lư Trung Hỏa", "element": "Hỏa",
      "meaning": { "vi": "Lửa trong lò — ngọn lửa được kiểm soát, ấm áp và hữu ích. Người mệnh này ổn định và đáng tin cậy.", "en": "Fire in the Furnace — controlled flame, warm and useful. People of this destiny are stable and reliable." } },
    { "na_am": "Đại Lâm Mộc", "element": "Mộc",
      "meaning": { "vi": "Cây rừng lớn — gỗ của rừng già, vững chãi và trường thọ. Người mệnh này bền bỉ và có nền tảng vững chắc.", "en": "Great Forest Wood — timber of ancient forests, strong and enduring. People of this destiny are persistent with solid foundations." } },
    { "na_am": "Lộ Bàng Thổ", "element": "Thổ",
      "meaning": { "vi": "Đất ven đường — đất phù sa ven đường, phì nhiêu và dễ tiếp cận. Người mệnh này hòa nhã và dễ gần.", "en": "Roadside Earth — fertile alluvial soil, rich and accessible. People of this destiny are approachable and gentle." } },
    { "na_am": "Kiếm Phong Kim", "element": "Kim",
      "meaning": { "vi": "Vàng mũi kiếm — kim loại được rèn luyện sắc bén. Người mệnh này quyết đoán và mạnh mẽ.", "en": "Sword Edge Metal — sharpened refined metal. People of this destiny are decisive and strong." } },
    { "na_am": "Sơn Đầu Hỏa", "element": "Hỏa",
      "meaning": { "vi": "Lửa đầu núi — ngọn lửa trên đỉnh núi, rực rỡ và nổi bật. Người mệnh này tự tin và có tầm nhìn cao.", "en": "Mountain Top Fire — blazing flame on the summit, brilliant and prominent. People of this destiny are confident with a broad vision." } },
    { "na_am": "Giản Hạ Thủy", "element": "Thủy",
      "meaning": { "vi": "Nước dưới suối — dòng nước trong vắt chảy qua khe đá. Người mệnh này trong sáng và linh hoạt.", "en": "Water Under the Stream — crystal clear water flowing through stone crevices. People of this destiny are pure and adaptable." } },
    { "na_am": "Thành Đầu Thổ", "element": "Thổ",
      "meaning": { "vi": "Đất đắp thành — nền đất vững chắc xây thành lũy. Người mệnh này đáng tin cậy và có khả năng bảo vệ.", "en": "City Wall Earth — solid foundation for fortress walls. People of this destiny are trustworthy and protective." } },
    { "na_am": "Bạch Lạp Kim", "element": "Kim",
      "meaning": { "vi": "Vàng nến trắng — kim loại tinh khiết, sáng và quý. Người mệnh này tinh tế và có giá trị nội tại.", "en": "White Wax Metal — pure refined metal, bright and precious. People of this destiny are refined with intrinsic value." } },
    { "na_am": "Dương Liễu Mộc", "element": "Mộc",
      "meaning": { "vi": "Gỗ cây liễu — mềm mại, uyển chuyển theo gió. Người mệnh này linh hoạt và dễ thích nghi.", "en": "Willow Wood — soft and flexible, swaying with the wind. People of this destiny are flexible and adaptable." } },
    { "na_am": "Tuyền Trung Thủy", "element": "Thủy",
      "meaning": { "vi": "Nước trong suối — nước ngầm tinh khiết, nguồn sống không bao giờ cạn. Người mệnh này sâu sắc và bền bỉ.", "en": "Spring Water — pure underground water, an inexhaustible life source. People of this destiny are profound and enduring." } },
    { "na_am": "Ốc Thượng Thổ", "element": "Thổ",
      "meaning": { "vi": "Đất trên nóc nhà — đất trên mái, cao và vững. Người mệnh này có vị trí cao và ổn định.", "en": "Rooftop Earth — soil atop the house, elevated and stable. People of this destiny hold high positions with stability." } },
    { "na_am": "Tích Lịch Hỏa", "element": "Hỏa",
      "meaning": { "vi": "Lửa sấm sét — ngọn lửa mãnh liệt từ sấm. Người mệnh này năng động và có sức ảnh hưởng lớn.", "en": "Thunderbolt Fire — fierce flame from lightning. People of this destiny are dynamic with great influence." } },
    { "na_am": "Tùng Bách Mộc", "element": "Mộc",
      "meaning": { "vi": "Gỗ tùng bách — loại gỗ quý, bền bỉ qua bốn mùa. Người mệnh này kiên cường và trung thành.", "en": "Pine and Cypress Wood — precious wood enduring through four seasons. People of this destiny are resilient and loyal." } },
    { "na_am": "Trường Lưu Thủy", "element": "Thủy",
      "meaning": { "vi": "Nước chảy dài — dòng sông lớn không ngừng chảy. Người mệnh này bền bỉ và luôn tiến về phía trước.", "en": "Long-flowing Water — a great river flowing ceaselessly. People of this destiny are persistent and always moving forward." } },
    { "na_am": "Sa Trung Kim", "element": "Kim",
      "meaning": { "vi": "Vàng trong cát — kim loại quý ẩn trong cát. Người mệnh này có tài năng tiềm ẩn chờ được phát hiện.", "en": "Gold in Sand — precious metal hidden in sand. People of this destiny have latent talents awaiting discovery." } },
    { "na_am": "Sơn Hạ Hỏa", "element": "Hỏa",
      "meaning": { "vi": "Lửa chân núi — ngọn lửa ấm áp dưới chân núi. Người mệnh này ấm áp và che chở.", "en": "Fire at Mountain Base — warm flame at the foot of the mountain. People of this destiny are warm and sheltering." } },
    { "na_am": "Bình Địa Mộc", "element": "Mộc",
      "meaning": { "vi": "Cây đồng bằng — cây mọc trên đất bằng, phát triển đều đặn. Người mệnh này ổn định và đáng tin cậy.", "en": "Flatland Wood — trees growing on plains, developing steadily. People of this destiny are stable and dependable." } },
    { "na_am": "Bích Thượng Thổ", "element": "Thổ",
      "meaning": { "vi": "Đất trên vách — đất bám trên tường, vững vàng ở vị trí cao. Người mệnh này có chí hướng và kiên định.", "en": "Wall Earth — soil on the wall, firm at elevated positions. People of this destiny are ambitious and steadfast." } },
    { "na_am": "Kim Bạch Kim", "element": "Kim",
      "meaning": { "vi": "Vàng bạc kim — kim loại thuần khiết nhất. Người mệnh này có phẩm chất cao quý và tinh khiết.", "en": "Pure Gold and Silver — the purest metals. People of this destiny have noble and pure qualities." } },
    { "na_am": "Phú Đăng Hỏa", "element": "Hỏa",
      "meaning": { "vi": "Lửa đèn lớn — ánh sáng chiếu xa, soi đường cho người khác. Người mệnh này là người dẫn đường và truyền cảm hứng.", "en": "Great Lamp Fire — light shining far, illuminating the path for others. People of this destiny are leaders and inspirers." } },
    { "na_am": "Thiên Hà Thủy", "element": "Thủy",
      "meaning": { "vi": "Nước trên trời — mưa từ bầu trời, ban phát cho vạn vật. Người mệnh này rộng lượng và có tầm nhìn rộng.", "en": "Heavenly River Water — rain from the sky, bestowing upon all things. People of this destiny are generous with a broad perspective." } },
    { "na_am": "Đại Trạch Thổ", "element": "Thổ",
      "meaning": { "vi": "Đất lớn rộng — vùng đất bao la, phì nhiêu. Người mệnh này bao dung và có khả năng chứa đựng lớn.", "en": "Great Marshland Earth — vast fertile land. People of this destiny are tolerant with great capacity." } },
    { "na_am": "Thoa Xuyến Kim", "element": "Kim",
      "meaning": { "vi": "Vàng trang sức — kim loại được chế tác tinh xảo. Người mệnh này khéo léo và có thẩm mỹ cao.", "en": "Ornamental Gold — finely crafted metal jewelry. People of this destiny are skillful with refined aesthetics." } },
    { "na_am": "Tang Đố Mộc", "element": "Mộc",
      "meaning": { "vi": "Gỗ cây dâu — cây dâu tằm, nuôi dưỡng và hữu ích. Người mệnh này chăm chỉ và đóng góp cho cộng đồng.", "en": "Mulberry Wood — silkworm tree, nurturing and useful. People of this destiny are hardworking and community-minded." } },
    { "na_am": "Đại Khê Thủy", "element": "Thủy",
      "meaning": { "vi": "Nước suối lớn — dòng suối rộng và sâu. Người mệnh này sâu sắc và có chiều sâu nội tâm.", "en": "Great Stream Water — wide and deep brook. People of this destiny are deep and introspective." } },
    { "na_am": "Sa Trung Thổ", "element": "Thổ",
      "meaning": { "vi": "Đất trong cát — đất trộn cát, linh hoạt nhưng cần nền tảng. Người mệnh này thích nghi tốt nhưng cần sự hỗ trợ.", "en": "Sand Earth — soil mixed with sand, flexible but needs foundation. People of this destiny adapt well but need support." } },
    { "na_am": "Thiên Thượng Hỏa", "element": "Hỏa",
      "meaning": { "vi": "Lửa trên trời — ánh sáng mặt trời, mãnh liệt và vinh quang. Người mệnh này có năng lượng mạnh mẽ và sự nổi bật.", "en": "Heavenly Fire — sunlight, fierce and glorious. People of this destiny have powerful energy and prominence." } },
    { "na_am": "Thạch Lựu Mộc", "element": "Mộc",
      "meaning": { "vi": "Gỗ cây lựu — cây lựu đầy quả, biểu tượng của sự sinh sôi. Người mệnh này sung túc và phồn thịnh.", "en": "Pomegranate Wood — fruit-bearing tree, symbol of abundance. People of this destiny are prosperous and thriving." } },
    { "na_am": "Đại Hải Thủy", "element": "Thủy",
      "meaning": { "vi": "Nước biển lớn — đại dương bao la, chứa đựng vạn vật. Người mệnh này có tấm lòng rộng lớn và bao dung.", "en": "Great Ocean Water — vast ocean containing all things. People of this destiny have great hearts and tolerance." } }
  ]
}
```

**Step 2: Validate JSON files**

Run: `python3 -c "import json; json.load(open('crates/amlich-core/data/day-deity-insight.json')); json.load(open('crates/amlich-core/data/na-am-insight.json')); print('OK')"`
Expected: `OK`

**Step 3: Commit**

```bash
git add crates/amlich-core/data/day-deity-insight.json crates/amlich-core/data/na-am-insight.json
git commit -m "data: add bilingual day deity and Na Am insight data"
```

---

## Task 5: Ten Gods Insight Data File

**Files:**
- Create: `crates/amlich-core/data/ten-gods-insight.json`

**Step 1: Create ten-gods-insight.json**

10 entries matching `ThapThanLabel` enum variants, with personality/energy descriptions.

```json
{
  "gods": [
    { "id": "TyKien", "name": { "vi": "Tỷ Kiên", "en": "Companion" },
      "meaning": { "vi": "Tỷ Kiên đại diện cho sự ngang hàng, bạn bè cùng chí hướng. Thể hiện tính tự lập, kiên định nhưng có thể cố chấp.", "en": "Companion represents peers and like-minded friends. Shows independence and determination but can be stubborn." } },
    { "id": "KiepTai", "name": { "vi": "Kiếp Tài", "en": "Rob Wealth" },
      "meaning": { "vi": "Kiếp Tài đại diện cho cạnh tranh và thử thách tài chính. Thể hiện tính cạnh tranh mạnh, năng động nhưng dễ hao tài.", "en": "Rob Wealth represents competition and financial challenges. Shows strong competitive drive, dynamism but prone to financial loss." } },
    { "id": "ThucThan", "name": { "vi": "Thực Thần", "en": "Eating God" },
      "meaning": { "vi": "Thực Thần đại diện cho sáng tạo, tài nghệ và hưởng thụ. Thể hiện tính lạc quan, nghệ thuật và khả năng biểu đạt.", "en": "Eating God represents creativity, talent and enjoyment. Shows optimism, artistic nature and expressive ability." } },
    { "id": "ThuongQuan", "name": { "vi": "Thương Quan", "en": "Hurting Officer" },
      "meaning": { "vi": "Thương Quan đại diện cho sự phá cách, nổi loạn và đổi mới. Thể hiện trí tuệ sắc bén, thách thức quyền uy.", "en": "Hurting Officer represents unconventionality, rebellion and innovation. Shows sharp intellect and challenges authority." } },
    { "id": "ChinhTai", "name": { "vi": "Chính Tài", "en": "Direct Wealth" },
      "meaning": { "vi": "Chính Tài đại diện cho thu nhập ổn định, tài chính bền vững. Thể hiện tính thực tế, cần cù và quản lý tài chính tốt.", "en": "Direct Wealth represents stable income and sustainable finances. Shows practicality, diligence and good financial management." } },
    { "id": "ThienTai", "name": { "vi": "Thiên Tài", "en": "Indirect Wealth" },
      "meaning": { "vi": "Thiên Tài đại diện cho cơ hội bất ngờ, đầu tư và kinh doanh. Thể hiện tính mạo hiểm, nhạy bén với cơ hội.", "en": "Indirect Wealth represents unexpected opportunities, investment and business. Shows risk-taking and sharp opportunity sense." } },
    { "id": "ChinhQuan", "name": { "vi": "Chính Quan", "en": "Direct Officer" },
      "meaning": { "vi": "Chính Quan đại diện cho kỷ luật, trách nhiệm và địa vị xã hội. Thể hiện tính chính trực, tuân thủ quy tắc.", "en": "Direct Officer represents discipline, responsibility and social status. Shows integrity and rule-following nature." } },
    { "id": "ThatSat", "name": { "vi": "Thất Sát", "en": "Seven Killings" },
      "meaning": { "vi": "Thất Sát đại diện cho quyền lực, sức mạnh và thách thức. Thể hiện tính quyết đoán, dũng cảm nhưng dễ gặp áp lực.", "en": "Seven Killings represents power, strength and challenge. Shows decisiveness, courage but prone to pressure." } },
    { "id": "ChinhAn", "name": { "vi": "Chính Ấn", "en": "Direct Seal" },
      "meaning": { "vi": "Chính Ấn đại diện cho học vấn, mẹ và sự bảo hộ. Thể hiện tính nhân hậu, ham học và được che chở.", "en": "Direct Seal represents education, mother and protection. Shows benevolence, love of learning and being sheltered." } },
    { "id": "ThienAn", "name": { "vi": "Thiên Ấn", "en": "Indirect Seal" },
      "meaning": { "vi": "Thiên Ấn đại diện cho trực giác, tâm linh và tư duy phi truyền thống. Thể hiện tính sáng tạo, nhạy cảm và hay suy tư.", "en": "Indirect Seal represents intuition, spirituality and unconventional thinking. Shows creativity, sensitivity and contemplation." } }
  ]
}
```

**Step 2: Validate**

Run: `python3 -c "import json; d=json.load(open('crates/amlich-core/data/ten-gods-insight.json')); assert len(d['gods'])==10; print('OK')"`
Expected: `OK`

**Step 3: Commit**

```bash
git add crates/amlich-core/data/ten-gods-insight.json
git commit -m "data: add bilingual Ten Gods insight data (10 entries)"
```

---

## Task 6: Tu Menh and Dai Van Insight Data Files

**Files:**
- Create: `crates/amlich-core/data/tu-menh-insight.json`
- Create: `crates/amlich-core/data/dai-van-insight.json`

**Step 1: Create tu-menh-insight.json**

8 Kua descriptions + East/West group meanings.

```json
{
  "groups": [
    { "id": "East", "name": { "vi": "Đông Tứ Mệnh", "en": "East Group" },
      "meaning": { "vi": "Nhóm Đông Tứ Mệnh gồm các Quái 1, 3, 4, 9. Hướng tốt: Đông, Đông Nam, Bắc, Nam.", "en": "East Group includes Kua 1, 3, 4, 9. Favorable directions: East, Southeast, North, South." } },
    { "id": "West", "name": { "vi": "Tây Tứ Mệnh", "en": "West Group" },
      "meaning": { "vi": "Nhóm Tây Tứ Mệnh gồm các Quái 2, 6, 7, 8. Hướng tốt: Tây, Tây Bắc, Tây Nam, Đông Bắc.", "en": "West Group includes Kua 2, 6, 7, 8. Favorable directions: West, Northwest, Southwest, Northeast." } }
  ],
  "kua": [
    { "number": 1, "trigram": { "vi": "Khảm", "en": "Kan" }, "direction": { "vi": "Bắc", "en": "North" },
      "meaning": { "vi": "Quái Khảm (Nước) — trí tuệ, linh hoạt, thích nghi tốt. Phương tốt nhất: Bắc.", "en": "Kan Trigram (Water) — wisdom, flexibility, adaptability. Best direction: North." } },
    { "number": 2, "trigram": { "vi": "Khôn", "en": "Kun" }, "direction": { "vi": "Tây Nam", "en": "Southwest" },
      "meaning": { "vi": "Quái Khôn (Đất) — nhân hậu, bao dung, chăm sóc. Phương tốt nhất: Tây Nam.", "en": "Kun Trigram (Earth) — benevolent, tolerant, nurturing. Best direction: Southwest." } },
    { "number": 3, "trigram": { "vi": "Chấn", "en": "Zhen" }, "direction": { "vi": "Đông", "en": "East" },
      "meaning": { "vi": "Quái Chấn (Sấm) — năng động, quyết đoán, khởi đầu mới. Phương tốt nhất: Đông.", "en": "Zhen Trigram (Thunder) — dynamic, decisive, new beginnings. Best direction: East." } },
    { "number": 4, "trigram": { "vi": "Tốn", "en": "Xun" }, "direction": { "vi": "Đông Nam", "en": "Southeast" },
      "meaning": { "vi": "Quái Tốn (Gió) — mềm mại, giao tiếp tốt, may mắn tài lộc. Phương tốt nhất: Đông Nam.", "en": "Xun Trigram (Wind) — gentle, good communication, financial fortune. Best direction: Southeast." } },
    { "number": 6, "trigram": { "vi": "Càn", "en": "Qian" }, "direction": { "vi": "Tây Bắc", "en": "Northwest" },
      "meaning": { "vi": "Quái Càn (Trời) — lãnh đạo, mạnh mẽ, có quyền lực. Phương tốt nhất: Tây Bắc.", "en": "Qian Trigram (Heaven) — leadership, strength, authority. Best direction: Northwest." } },
    { "number": 7, "trigram": { "vi": "Đoài", "en": "Dui" }, "direction": { "vi": "Tây", "en": "West" },
      "meaning": { "vi": "Quái Đoài (Đầm) — vui vẻ, giao tiếp, sáng tạo. Phương tốt nhất: Tây.", "en": "Dui Trigram (Lake) — joyful, communicative, creative. Best direction: West." } },
    { "number": 8, "trigram": { "vi": "Cấn", "en": "Gen" }, "direction": { "vi": "Đông Bắc", "en": "Northeast" },
      "meaning": { "vi": "Quái Cấn (Núi) — ổn định, kiên nhẫn, tĩnh lặng. Phương tốt nhất: Đông Bắc.", "en": "Gen Trigram (Mountain) — stable, patient, tranquil. Best direction: Northeast." } },
    { "number": 9, "trigram": { "vi": "Ly", "en": "Li" }, "direction": { "vi": "Nam", "en": "South" },
      "meaning": { "vi": "Quái Ly (Lửa) — sáng suốt, nhiệt huyết, nổi bật. Phương tốt nhất: Nam.", "en": "Li Trigram (Fire) — bright, passionate, prominent. Best direction: South." } }
  ]
}
```

**Step 2: Create dai-van-insight.json**

```json
{
  "directions": [
    { "id": "Thuan", "name": { "vi": "Thuận hành", "en": "Forward Progression" },
      "meaning": { "vi": "Đại vận thuận hành theo chiều tiến, vận mệnh phát triển tự nhiên theo tuổi.", "en": "Forward progression follows natural age development, destiny unfolds in ascending order." } },
    { "id": "Nghich", "name": { "vi": "Nghịch hành", "en": "Reverse Progression" },
      "meaning": { "vi": "Đại vận nghịch hành theo chiều lùi, vận mệnh phát triển ngược dòng thời gian.", "en": "Reverse progression goes backward, destiny unfolds against the flow of time." } }
  ],
  "phases": {
    "meaning": {
      "vi": "Mỗi trụ đại vận kéo dài 10 năm, thể hiện một giai đoạn vận mệnh với ngũ hành và can chi riêng. Ngũ hành của trụ tương tác với tứ trụ bản mệnh để xác định vận may hay xấu.",
      "en": "Each Dai Van pillar spans 10 years, representing a destiny phase with its own five-element and stem-branch pair. The pillar's element interacts with the natal chart to determine fortune."
    }
  },
  "elements": [
    { "element": "Kim", "meaning": { "vi": "Giai đoạn Kim — thời kỳ quyết đoán, sắc bén, thích hợp cho sự nghiệp và tài chính.", "en": "Metal phase — period of decisiveness and sharpness, favorable for career and finance." } },
    { "element": "Mộc", "meaning": { "vi": "Giai đoạn Mộc — thời kỳ phát triển, sáng tạo, thích hợp cho học tập và khởi nghiệp.", "en": "Wood phase — period of growth and creativity, favorable for learning and starting ventures." } },
    { "element": "Thủy", "meaning": { "vi": "Giai đoạn Thủy — thời kỳ trí tuệ, linh hoạt, thích hợp cho giao tiếp và du lịch.", "en": "Water phase — period of wisdom and flexibility, favorable for communication and travel." } },
    { "element": "Hỏa", "meaning": { "vi": "Giai đoạn Hỏa — thời kỳ nhiệt huyết, nổi bật, thích hợp cho danh tiếng và quan hệ.", "en": "Fire phase — period of passion and prominence, favorable for fame and relationships." } },
    { "element": "Thổ", "meaning": { "vi": "Giai đoạn Thổ — thời kỳ ổn định, bền vững, thích hợp cho bất động sản và gia đình.", "en": "Earth phase — period of stability and sustainability, favorable for property and family." } }
  ]
}
```

**Step 2: Validate**

Run: `python3 -c "import json; json.load(open('crates/amlich-core/data/tu-menh-insight.json')); json.load(open('crates/amlich-core/data/dai-van-insight.json')); print('OK')"`
Expected: `OK`

**Step 3: Commit**

```bash
git add crates/amlich-core/data/tu-menh-insight.json crates/amlich-core/data/dai-van-insight.json
git commit -m "data: add bilingual Tu Menh (Kua) and Dai Van insight data"
```

---

## Task 7: Insight Data Loading — Truc, Day Deity, Na Am, Ten Gods

**Files:**
- Modify: `crates/amlich-core/src/insight_data.rs`

**Step 1: Write failing tests for new loaders**

Add tests at the bottom of `insight_data.rs`:

```rust
#[test]
fn all_truc_insights_has_12_entries() {
    assert_eq!(all_truc_insights().len(), 12);
}

#[test]
fn find_truc_insight_returns_entry() {
    let truc = find_truc_insight("Kiến");
    assert!(truc.is_some());
    assert!(!truc.unwrap().meaning.vi.is_empty());
}

#[test]
fn all_na_am_insights_has_30_entries() {
    assert_eq!(all_na_am_insights().len(), 30);
}

#[test]
fn find_na_am_insight_returns_entry() {
    let na_am = find_na_am_insight("Hải Trung Kim");
    assert!(na_am.is_some());
}

#[test]
fn all_ten_gods_insights_has_10_entries() {
    assert_eq!(all_ten_gods_insights().len(), 10);
}

#[test]
fn find_deity_classification_returns_entry() {
    let cls = find_deity_classification_insight("HoangDao");
    assert!(cls.is_some());
}

#[test]
fn find_deity_by_name_returns_entry() {
    let deity = find_deity_insight("Thanh Long");
    assert!(deity.is_some());
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test -p amlich-core --lib insight_data`
Expected: FAIL (functions don't exist)

**Step 3: Add include_str!, structs, OnceLock, and lookup functions**

At the top of `insight_data.rs`, add new constants:

```rust
const TRUC_INSIGHT_JSON: &str = include_str!("../data/truc-insight.json");
const DAY_DEITY_INSIGHT_JSON: &str = include_str!("../data/day-deity-insight.json");
const NA_AM_INSIGHT_JSON: &str = include_str!("../data/na-am-insight.json");
const TEN_GODS_INSIGHT_JSON: &str = include_str!("../data/ten-gods-insight.json");
```

Add data structs (follow existing `BilingualText`/`BilingualList` types):

```rust
#[derive(Debug, Clone, Deserialize)]
pub struct TrucInsight {
    pub id: String,
    pub meaning: BilingualText,
    pub good_for: BilingualList,
    pub avoid_for: BilingualList,
}

#[derive(Debug, Clone, Deserialize)]
struct TrucInsightFile {
    truc: Vec<TrucInsight>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeityClassificationInsight {
    pub id: String,
    pub name: BilingualText,
    pub meaning: BilingualText,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeityInsight {
    pub name: String,
    pub classification: String,
    pub meaning: BilingualText,
}

#[derive(Debug, Clone, Deserialize)]
struct DayDeityInsightFile {
    classifications: Vec<DeityClassificationInsight>,
    deities: Vec<DeityInsight>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NaAmInsight {
    pub na_am: String,
    pub element: String,
    pub meaning: BilingualText,
}

#[derive(Debug, Clone, Deserialize)]
struct NaAmInsightFile {
    pairs: Vec<NaAmInsight>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TenGodsInsight {
    pub id: String,
    pub name: BilingualText,
    pub meaning: BilingualText,
}

#[derive(Debug, Clone, Deserialize)]
struct TenGodsInsightFile {
    gods: Vec<TenGodsInsight>,
}
```

Add statics and loaders:

```rust
static TRUC_INSIGHT_DATA: OnceLock<Vec<TrucInsight>> = OnceLock::new();
static DAY_DEITY_INSIGHT_DATA: OnceLock<DayDeityInsightFile> = OnceLock::new();
static NA_AM_INSIGHT_DATA: OnceLock<Vec<NaAmInsight>> = OnceLock::new();
static TEN_GODS_INSIGHT_DATA: OnceLock<Vec<TenGodsInsight>> = OnceLock::new();

pub fn all_truc_insights() -> &'static [TrucInsight] {
    TRUC_INSIGHT_DATA
        .get_or_init(|| {
            let parsed: TrucInsightFile = serde_json::from_str(TRUC_INSIGHT_JSON)
                .expect("Failed to parse data/truc-insight.json");
            parsed.truc
        })
        .as_slice()
}

pub fn find_truc_insight(name: &str) -> Option<&'static TrucInsight> {
    all_truc_insights().iter().find(|t| t.id == name)
}

fn day_deity_insight_data() -> &'static DayDeityInsightFile {
    DAY_DEITY_INSIGHT_DATA.get_or_init(|| {
        serde_json::from_str(DAY_DEITY_INSIGHT_JSON)
            .expect("Failed to parse data/day-deity-insight.json")
    })
}

pub fn find_deity_classification_insight(id: &str) -> Option<&'static DeityClassificationInsight> {
    day_deity_insight_data().classifications.iter().find(|c| c.id == id)
}

pub fn find_deity_insight(name: &str) -> Option<&'static DeityInsight> {
    day_deity_insight_data().deities.iter().find(|d| d.name == name)
}

pub fn all_na_am_insights() -> &'static [NaAmInsight] {
    NA_AM_INSIGHT_DATA
        .get_or_init(|| {
            let parsed: NaAmInsightFile = serde_json::from_str(NA_AM_INSIGHT_JSON)
                .expect("Failed to parse data/na-am-insight.json");
            parsed.pairs
        })
        .as_slice()
}

pub fn find_na_am_insight(na_am: &str) -> Option<&'static NaAmInsight> {
    all_na_am_insights().iter().find(|n| n.na_am == na_am)
}

pub fn all_ten_gods_insights() -> &'static [TenGodsInsight] {
    TEN_GODS_INSIGHT_DATA
        .get_or_init(|| {
            let parsed: TenGodsInsightFile = serde_json::from_str(TEN_GODS_INSIGHT_JSON)
                .expect("Failed to parse data/ten-gods-insight.json");
            parsed.gods
        })
        .as_slice()
}

pub fn find_ten_gods_insight(id: &str) -> Option<&'static TenGodsInsight> {
    all_ten_gods_insights().iter().find(|g| g.id == id)
}
```

**Step 4: Run tests to verify they pass**

Run: `cargo test -p amlich-core --lib insight_data`
Expected: PASS (all new + existing tests)

**Step 5: Commit**

```bash
git add crates/amlich-core/src/insight_data.rs
git commit -m "feat: add insight data loaders for Truc, Day Deity, Na Am, Ten Gods"
```

---

## Task 8: Insight Data Loading — Tu Menh and Dai Van

**Files:**
- Modify: `crates/amlich-core/src/insight_data.rs`

**Step 1: Write failing tests**

```rust
#[test]
fn tu_menh_kua_insights_has_8_entries() {
    assert_eq!(all_kua_insights().len(), 8);
}

#[test]
fn find_kua_insight_by_number() {
    let kua = find_kua_insight(1);
    assert!(kua.is_some());
}

#[test]
fn tu_menh_group_insights_has_2_entries() {
    assert_eq!(all_kua_group_insights().len(), 2);
}

#[test]
fn dai_van_element_insights_has_5_entries() {
    assert_eq!(all_dai_van_element_insights().len(), 5);
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test -p amlich-core --lib insight_data`
Expected: FAIL

**Step 3: Add loaders for Tu Menh and Dai Van data**

```rust
const TU_MENH_INSIGHT_JSON: &str = include_str!("../data/tu-menh-insight.json");
const DAI_VAN_INSIGHT_JSON: &str = include_str!("../data/dai-van-insight.json");

#[derive(Debug, Clone, Deserialize)]
pub struct KuaGroupInsight {
    pub id: String,
    pub name: BilingualText,
    pub meaning: BilingualText,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KuaInsight {
    pub number: u8,
    pub trigram: BilingualText,
    pub direction: BilingualText,
    pub meaning: BilingualText,
}

#[derive(Debug, Clone, Deserialize)]
struct TuMenhInsightFile {
    groups: Vec<KuaGroupInsight>,
    kua: Vec<KuaInsight>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DaiVanDirectionInsight {
    pub id: String,
    pub name: BilingualText,
    pub meaning: BilingualText,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DaiVanPhasesInsight {
    pub meaning: BilingualText,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DaiVanElementInsight {
    pub element: String,
    pub meaning: BilingualText,
}

#[derive(Debug, Clone, Deserialize)]
struct DaiVanInsightFile {
    directions: Vec<DaiVanDirectionInsight>,
    phases: DaiVanPhasesInsight,
    elements: Vec<DaiVanElementInsight>,
}

static TU_MENH_INSIGHT_DATA: OnceLock<TuMenhInsightFile> = OnceLock::new();
static DAI_VAN_INSIGHT_DATA: OnceLock<DaiVanInsightFile> = OnceLock::new();

fn tu_menh_insight_data() -> &'static TuMenhInsightFile {
    TU_MENH_INSIGHT_DATA.get_or_init(|| {
        serde_json::from_str(TU_MENH_INSIGHT_JSON)
            .expect("Failed to parse data/tu-menh-insight.json")
    })
}

pub fn all_kua_group_insights() -> &'static [KuaGroupInsight] {
    &tu_menh_insight_data().groups
}

pub fn all_kua_insights() -> &'static [KuaInsight] {
    &tu_menh_insight_data().kua
}

pub fn find_kua_insight(number: u8) -> Option<&'static KuaInsight> {
    all_kua_insights().iter().find(|k| k.number == number)
}

pub fn find_kua_group_insight(id: &str) -> Option<&'static KuaGroupInsight> {
    all_kua_group_insights().iter().find(|g| g.id == id)
}

fn dai_van_insight_data() -> &'static DaiVanInsightFile {
    DAI_VAN_INSIGHT_DATA.get_or_init(|| {
        serde_json::from_str(DAI_VAN_INSIGHT_JSON)
            .expect("Failed to parse data/dai-van-insight.json")
    })
}

pub fn all_dai_van_direction_insights() -> &'static [DaiVanDirectionInsight] {
    &dai_van_insight_data().directions
}

pub fn dai_van_phases_insight() -> &'static DaiVanPhasesInsight {
    &dai_van_insight_data().phases
}

pub fn all_dai_van_element_insights() -> &'static [DaiVanElementInsight] {
    &dai_van_insight_data().elements
}

pub fn find_dai_van_element_insight(element: &str) -> Option<&'static DaiVanElementInsight> {
    all_dai_van_element_insights().iter().find(|e| e.element == element)
}

pub fn find_dai_van_direction_insight(id: &str) -> Option<&'static DaiVanDirectionInsight> {
    all_dai_van_direction_insights().iter().find(|d| d.id == id)
}
```

**Step 4: Run tests**

Run: `cargo test -p amlich-core --lib insight_data`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/amlich-core/src/insight_data.rs
git commit -m "feat: add insight data loaders for Tu Menh (Kua) and Dai Van"
```

---

## Task 9: New Insight DTOs

**Files:**
- Modify: `crates/amlich-api/src/dto.rs`

**Step 1: Write failing DTO serialization test**

```rust
#[test]
fn enriched_day_insight_dto_serializes_with_new_fields() {
    let insight = DayInsightDto {
        solar: SolarDto { day: 1, month: 1, year: 2025, day_of_week: 3, day_of_week_name: "Wed".into(), date_string: "2025-01-01".into() },
        lunar: LunarDto { day: 1, month: 12, year: 2024, is_leap_month: false, date_string: "".into() },
        festival: None,
        holiday: None,
        canchi: None,
        day_guidance: None,
        tiet_khi: None,
        na_am: None,
        truc: None,
        day_deity: None,
        stars: None,
        taboos: None,
        travel: None,
        xung_hop: None,
        tang_can: None,
        ten_gods: None,
        hours: None,
        tu_menh: None,
        dai_van: None,
    };
    let json = serde_json::to_string(&insight).expect("serialize");
    // New optional fields should not appear when None (skip_serializing_if)
    assert!(!json.contains("na_am"));
    assert!(!json.contains("truc"));
    assert!(!json.contains("dai_van"));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p amlich-api --lib dto`
Expected: FAIL (fields don't exist)

**Step 3: Add new insight DTOs and expand DayInsightDto**

Add these new DTOs in `dto.rs`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NaAmInsightDto {
    pub na_am: String,
    pub element: String,
    pub meaning: LocalizedTextDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrucInsightDto {
    pub name: String,
    pub quality: String,
    pub meaning: LocalizedTextDto,
    pub good_for: LocalizedListDto,
    pub avoid_for: LocalizedListDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayDeityInsightDto {
    pub name: String,
    pub classification: String,
    pub classification_meaning: LocalizedTextDto,
    pub deity_meaning: Option<LocalizedTextDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarsInsightDto {
    pub cat_tinh: Vec<String>,
    pub sat_tinh: Vec<String>,
    pub day_star: Option<String>,
    pub day_star_quality: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabooInsightItemDto {
    pub name: String,
    pub severity: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TravelInsightDto {
    pub xuat_hanh_huong: String,
    pub tai_than: String,
    pub hy_than: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XungHopInsightDto {
    pub luc_xung: String,
    pub tam_hop: Vec<String>,
    pub liu_he: Option<String>,
    pub xiang_hai: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TangCanInsightDto {
    pub main: String,
    pub central: String,
    pub residual: String,
    pub strength: [u8; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenGodsInsightDto {
    pub to_year_stem: Option<TenGodsEntryInsightDto>,
    pub to_self: Option<TenGodsEntryInsightDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenGodsEntryInsightDto {
    pub label: String,
    pub name: LocalizedTextDto,
    pub meaning: LocalizedTextDto,
    pub relation: String,
    pub same_polarity: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoursInsightDto {
    pub good_hour_count: usize,
    pub good_hours: Vec<HourInsightEntryDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourInsightEntryDto {
    pub chi: String,
    pub time_range: String,
    pub star: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuMenhInsightDto {
    pub kua: u8,
    pub group: String,
    pub trigram: LocalizedTextDto,
    pub direction: LocalizedTextDto,
    pub meaning: LocalizedTextDto,
    pub group_meaning: LocalizedTextDto,
    pub favorable_directions: Vec<String>,
    pub unfavorable_directions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaiVanInsightDto {
    pub direction: String,
    pub direction_meaning: LocalizedTextDto,
    pub start_age: String,
    pub current_pillar: Option<DaiVanPillarInsightDto>,
    pub all_pillars: Vec<DaiVanPillarInsightDto>,
    pub phases_meaning: LocalizedTextDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaiVanPillarInsightDto {
    pub index: usize,
    pub can_chi: String,
    pub start_age: f64,
    pub end_age: f64,
    pub element: String,
    pub element_meaning: LocalizedTextDto,
}
```

Update `DayInsightDto` to include all new fields (all `Option<T>` with `skip_serializing_if`):

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayInsightDto {
    pub solar: SolarDto,
    pub lunar: LunarDto,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub festival: Option<FestivalInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holiday: Option<HolidayInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canchi: Option<CanChiInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_guidance: Option<DayGuidanceDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiet_khi: Option<TietKhiInsightDto>,
    // --- NEW FIELDS ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub na_am: Option<NaAmInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truc: Option<TrucInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_deity: Option<DayDeityInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stars: Option<StarsInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taboos: Option<Vec<TabooInsightItemDto>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub travel: Option<TravelInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xung_hop: Option<XungHopInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tang_can: Option<TangCanInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ten_gods: Option<TenGodsInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours: Option<HoursInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tu_menh: Option<TuMenhInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dai_van: Option<DaiVanInsightDto>,
}
```

**Step 4: Fix all existing code that constructs `DayInsightDto`** — add `None` for each new field in `crates/amlich-api/src/lib.rs` in the `get_day_insight` function.

**Step 5: Run tests**

Run: `cargo test -p amlich-api`
Expected: PASS

**Step 6: Commit**

```bash
git add crates/amlich-api/src/dto.rs crates/amlich-api/src/lib.rs
git commit -m "feat: add enriched insight DTOs with all almanac subsystem fields"
```

---

## Task 10: Enriched Insight Builder — Day-Only Subsystems

**Files:**
- Modify: `crates/amlich-api/src/lib.rs` (the `get_day_insight` function)

**Step 1: Write failing integration test**

In `crates/amlich-api/src/lib.rs` tests:

```rust
#[test]
fn get_day_insight_populates_truc_and_na_am() {
    let query = DateQuery { day: 10, month: 2, year: 2024, timezone: None };
    let insight = get_day_insight(&query).expect("insight");
    assert!(insight.truc.is_some(), "truc should be populated");
    assert!(insight.na_am.is_some(), "na_am should be populated");
    assert!(insight.day_deity.is_some(), "day_deity should be populated");
    assert!(insight.stars.is_some(), "stars should be populated");
    assert!(insight.travel.is_some(), "travel should be populated");
    assert!(insight.ten_gods.is_some(), "ten_gods should be populated");
    assert!(insight.hours.is_some(), "hours should be populated");
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p amlich-api --lib get_day_insight_populates`
Expected: FAIL (fields are all None)

**Step 3: Wire get_day_insight to populate enriched fields**

In `get_day_insight`, after computing `day_info`, access `day_info.day_fortune` and merge with insight data:

```rust
// After existing code that builds canchi_insight, guidance, tiet_khi_insight...

let fortune = day_info.day_fortune.as_ref();

// Na Am insight
let na_am_insight = fortune.and_then(|f| {
    amlich_core::insight_data::find_na_am_insight(&f.day_element.na_am)
        .map(|n| NaAmInsightDto {
            na_am: f.day_element.na_am.clone(),
            element: f.day_element.element.clone(),
            meaning: LocalizedTextDto::from(&n.meaning),
        })
});

// Truc insight
let truc_insight = fortune.and_then(|f| {
    amlich_core::insight_data::find_truc_insight(&f.truc.name)
        .map(|t| TrucInsightDto {
            name: f.truc.name.clone(),
            quality: f.truc.quality.clone(),
            meaning: LocalizedTextDto::from(&t.meaning),
            good_for: LocalizedListDto::from(&t.good_for),
            avoid_for: LocalizedListDto::from(&t.avoid_for),
        })
});

// Day Deity insight
let day_deity_insight = fortune.and_then(|f| {
    f.day_deity.as_ref().map(|deity| {
        let cls_id = match deity.classification {
            amlich_core::DayDeityClassification::HoangDao => "HoangDao",
            amlich_core::DayDeityClassification::HacDao => "HacDao",
        };
        let cls_meaning = amlich_core::insight_data::find_deity_classification_insight(cls_id)
            .map(|c| LocalizedTextDto::from(&c.meaning))
            .unwrap_or_else(|| LocalizedTextDto { vi: String::new(), en: String::new() });
        let deity_meaning = amlich_core::insight_data::find_deity_insight(&deity.name)
            .map(|d| LocalizedTextDto::from(&d.meaning));
        DayDeityInsightDto {
            name: deity.name.clone(),
            classification: cls_id.to_string(),
            classification_meaning: cls_meaning,
            deity_meaning,
        }
    })
});

// Stars insight
let stars_insight = fortune.map(|f| StarsInsightDto {
    cat_tinh: f.stars.cat_tinh.clone(),
    sat_tinh: f.stars.sat_tinh.clone(),
    day_star: f.stars.day_star.as_ref().map(|s| s.name.clone()),
    day_star_quality: f.stars.day_star.as_ref().map(|s| format!("{:?}", s.quality)),
});

// Taboos insight
let taboos_insight = fortune.map(|f| {
    f.taboos.iter().map(|t| TabooInsightItemDto {
        name: t.name.clone(),
        severity: t.severity.clone(),
        reason: t.reason.clone(),
    }).collect::<Vec<_>>()
}).filter(|v: &Vec<_>| !v.is_empty());

// Travel insight
let travel_insight = fortune.map(|f| TravelInsightDto {
    xuat_hanh_huong: f.travel.xuat_hanh_huong.clone(),
    tai_than: f.travel.tai_than.clone(),
    hy_than: f.travel.hy_than.clone(),
});

// Xung Hop insight
let xung_hop_insight = fortune.map(|f| XungHopInsightDto {
    luc_xung: f.xung_hop.luc_xung.clone(),
    tam_hop: f.xung_hop.tam_hop.clone(),
    liu_he: f.xung_hop.liu_he.clone(),
    xiang_hai: f.xung_hop.xiang_hai.clone(),
});

// Tang Can insight
let tang_can_insight = fortune.and_then(|f| {
    f.tang_can.as_ref().map(|tc| TangCanInsightDto {
        main: tc.main.clone(),
        central: tc.central.clone(),
        residual: tc.residual.clone(),
        strength: tc.strength,
    })
});

// Ten Gods insight
let ten_gods_insight = fortune.and_then(|f| {
    f.ten_gods.as_ref().map(|tg| {
        let map_entry = |r: &amlich_core::ThapThanResult| -> TenGodsEntryInsightDto {
            let label_id = format!("{:?}", r.label);
            let insight = amlich_core::insight_data::find_ten_gods_insight(&label_id);
            TenGodsEntryInsightDto {
                label: label_id,
                name: insight.map(|i| LocalizedTextDto::from(&i.name))
                    .unwrap_or_else(|| LocalizedTextDto { vi: String::new(), en: String::new() }),
                meaning: insight.map(|i| LocalizedTextDto::from(&i.meaning))
                    .unwrap_or_else(|| LocalizedTextDto { vi: String::new(), en: String::new() }),
                relation: format!("{:?}", r.relation),
                same_polarity: r.same_polarity,
            }
        };
        TenGodsInsightDto {
            to_year_stem: tg.to_year_stem.as_ref().map(map_entry),
            to_self: tg.to_self.as_ref().map(map_entry),
        }
    })
});

// Hours insight
let hours_insight = Some(HoursInsightDto {
    good_hour_count: day_info.gio_hoang_dao.good_hours.len(),
    good_hours: day_info.gio_hoang_dao.good_hours.iter().map(|h| HourInsightEntryDto {
        chi: h.hour_chi.clone(),
        time_range: h.time_range.clone(),
        star: h.star.clone(),
    }).collect(),
});
```

Then update the returned `DayInsightDto` to include all new fields.

**Step 4: Run tests**

Run: `cargo test -p amlich-api`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/amlich-api/src/lib.rs
git commit -m "feat: wire enriched insight builder with day-only subsystems"
```

---

## Task 11: Enriched Insight Builder — Birth-Dependent Subsystems

**Files:**
- Modify: `crates/amlich-api/src/lib.rs`
- Modify: `crates/amlich-api/src/dto.rs` (add ProfileContext param)

**Step 1: Add profile context to insight API**

Add a new function that accepts optional birth context:

```rust
pub fn get_day_insight_with_profile(
    query: &DateQuery,
    birth_year: Option<i32>,
    birth_month: Option<i32>,
    birth_day: Option<i32>,
    gender: Option<amlich_core::Gender>,
) -> Result<DayInsightDto, String>
```

This function calls the existing logic, then if `birth_year` and `gender` are both present:
- Compute Kua via `amlich_core::compute_kua(birth_year, gender)`
- Look up kua insight via `find_kua_insight(kua.kua)` and `find_kua_group_insight(group_id)`
- If birth_day + birth_month also present: compute Dai Van via `amlich_core::calculate_dai_van(birth_day, birth_month, birth_year, gender)`
- Look up Dai Van insight data

**Step 2: Write test**

```rust
#[test]
fn get_day_insight_with_profile_populates_tu_menh() {
    let query = DateQuery { day: 10, month: 2, year: 2024, timezone: None };
    let insight = get_day_insight_with_profile(
        &query,
        Some(1990), Some(5), Some(15),
        Some(amlich_core::Gender::Male),
    ).expect("insight");
    assert!(insight.tu_menh.is_some(), "tu_menh should be populated");
    assert!(insight.dai_van.is_some(), "dai_van should be populated");
}

#[test]
fn get_day_insight_with_profile_no_birth_skips_tu_menh() {
    let query = DateQuery { day: 10, month: 2, year: 2024, timezone: None };
    let insight = get_day_insight_with_profile(
        &query, None, None, None, None,
    ).expect("insight");
    assert!(insight.tu_menh.is_none());
    assert!(insight.dai_van.is_none());
}
```

**Step 3: Implement the function**

Follow the pattern from Task 10 but add Kua and Dai Van population when birth context is available.

**Step 4: Update existing `get_day_insight` to delegate**

```rust
pub fn get_day_insight(query: &DateQuery) -> Result<DayInsightDto, String> {
    get_day_insight_with_profile(query, None, None, None, None)
}
```

**Step 5: Run tests**

Run: `cargo test -p amlich-api`
Expected: PASS

**Step 6: Commit**

```bash
git add crates/amlich-api/src/lib.rs
git commit -m "feat: wire birth-dependent insight subsystems (Kua, Dai Van)"
```

---

## Task 12: CLI Insight with Profile Integration

**Files:**
- Modify: `crates/amlich/src/main.rs` (run_insight function)

**Step 1: Update run_insight to load user profile**

In `run_insight`, load the user profile and pass birth context to the insight API:

```rust
fn run_insight(args: InsightArgs) -> Result<(), String> {
    let date = parse_date_or_today(args.date.as_deref())?;
    let query = query_from_date(date, args.timezone);
    let profile = crate::profile::load_profile();
    let gender = profile.gender.map(|g| match g {
        crate::profile::ProfileGender::Male => amlich_core::Gender::Male,
        crate::profile::ProfileGender::Female => amlich_core::Gender::Female,
    });
    let insight = amlich_api::get_day_insight_with_profile(
        &query,
        profile.birth_year,
        profile.birth_month,
        profile.birth_day,
        gender,
    )?;
    match args.format {
        StructuredFormatArg::Json => print_json(&insight, args.pretty)?,
        StructuredFormatArg::Text => render_insight_text(args.lang, &insight),
    }
    Ok(())
}
```

**Step 2: Update render_insight_text to display new sections**

Add rendering for the new insight fields after the existing sections. For each new field, print a section header and the bilingual content based on the `lang` argument. Keep it simple — check if the field is `Some` and print.

Key new sections to render:
- Truc (name + quality + meaning + good_for/avoid_for)
- Day Deity (name + classification + meaning)
- Stars (cat_tinh/sat_tinh lists)
- Na Am (element + meaning)
- Taboos (list with severity + reason)
- Travel (directions)
- Ten Gods (label + meaning for each relation)
- Hours (good hour summary)
- Tu Menh (if present: kua number + group + trigram + directions)
- Dai Van (if present: direction + current pillar + element meaning)

**Step 3: Run CLI smoke test**

Run: `cargo run -p amlich -- insight 2024-02-10 --format text`
Expected: Output includes new sections (Truc, Stars, etc.)

Run: `cargo run -p amlich -- insight 2024-02-10 --format json --pretty`
Expected: JSON includes new fields

**Step 4: Commit**

```bash
git add crates/amlich/src/main.rs
git commit -m "feat: integrate user profile into CLI insight command"
```

---

## Task 13: TUI Insight Overlay Expansion

**Files:**
- Modify: `crates/amlich/src/widgets/insight_overlay.rs`
- Modify: `crates/amlich/src/app.rs` (add new InsightTab variants)
- Modify: `crates/amlich/src/event.rs` (update tab keybindings)

**Step 1: Add new InsightTab variants**

In `app.rs`, extend `InsightTab`:

```rust
pub enum InsightTab {
    Festival,
    Guidance,
    TietKhi,
    Almanac,   // NEW: Truc, Deity, Stars, Na Am, Taboos
    Advanced,  // NEW: Ten Gods, Travel, Xung Hop, Tang Can
    Personal,  // NEW: Tu Menh, Dai Van (only if profile configured)
}
```

**Step 2: Update event.rs tab keybindings**

Add keys `4`, `5`, `6` for the new tabs.

**Step 3: Implement new tab renderers in insight_overlay.rs**

- `render_almanac_tab`: Renders Truc, Day Deity, Stars, Na Am, Taboos from insight DTO
- `render_advanced_tab`: Renders Ten Gods, Travel, Xung Hop, Tang Can, Hours
- `render_personal_tab`: Renders Tu Menh and Dai Van (show "Configure profile with `amlich config profile set`" if absent)

Follow the existing pattern: use `pick_text`/`pick_items` for language selection, `push_bulleted` for lists.

**Step 4: Update tab_content dispatch and tab_indicator**

Add the new variants to both functions.

**Step 5: Run TUI**

Run: `cargo run -p amlich -- tui`
Expected: Press `i` to open insight overlay, tabs 4/5/6 show new content

**Step 6: Commit**

```bash
git add crates/amlich/src/widgets/insight_overlay.rs crates/amlich/src/app.rs crates/amlich/src/event.rs
git commit -m "feat: expand TUI insight overlay with almanac, advanced, and personal tabs"
```

---

## Task 14: Integration Tests

**Files:**
- Create: `crates/amlich-api/tests/insight_enrichment.rs`

**Step 1: Write comprehensive integration tests**

```rust
use amlich_api::{DateQuery, get_day_insight, get_day_insight_with_profile};

#[test]
fn enriched_insight_has_all_day_only_fields() {
    let query = DateQuery { day: 1, month: 1, year: 2025, timezone: None };
    let insight = get_day_insight(&query).unwrap();

    // Existing fields still work
    assert!(insight.canchi.is_some());
    assert!(insight.tiet_khi.is_some());

    // New day-only fields populated
    assert!(insight.truc.is_some());
    assert!(insight.na_am.is_some());
    assert!(insight.stars.is_some());
    assert!(insight.travel.is_some());
    assert!(insight.hours.is_some());

    // Birth-dependent fields absent without profile
    assert!(insight.tu_menh.is_none());
    assert!(insight.dai_van.is_none());
}

#[test]
fn enriched_insight_bilingual_non_empty() {
    let query = DateQuery { day: 15, month: 6, year: 2025, timezone: None };
    let insight = get_day_insight(&query).unwrap();

    if let Some(truc) = &insight.truc {
        assert!(!truc.meaning.vi.is_empty());
        assert!(!truc.meaning.en.is_empty());
    }
    if let Some(na_am) = &insight.na_am {
        assert!(!na_am.meaning.vi.is_empty());
        assert!(!na_am.meaning.en.is_empty());
    }
}

#[test]
fn enriched_insight_with_profile_populates_birth_fields() {
    let query = DateQuery { day: 1, month: 1, year: 2025, timezone: None };
    let insight = get_day_insight_with_profile(
        &query,
        Some(1990), Some(5), Some(15),
        Some(amlich_core::Gender::Male),
    ).unwrap();

    assert!(insight.tu_menh.is_some());
    let tu_menh = insight.tu_menh.unwrap();
    assert!(tu_menh.kua > 0);
    assert!(!tu_menh.meaning.vi.is_empty());

    assert!(insight.dai_van.is_some());
    let dai_van = insight.dai_van.unwrap();
    assert!(!dai_van.all_pillars.is_empty());
}

#[test]
fn enriched_insight_json_roundtrip() {
    let query = DateQuery { day: 10, month: 3, year: 2025, timezone: None };
    let insight = get_day_insight(&query).unwrap();
    let json = serde_json::to_string(&insight).unwrap();
    let parsed: amlich_api::DayInsightDto = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.solar.day, insight.solar.day);
}
```

**Step 2: Run tests**

Run: `cargo test -p amlich-api --test insight_enrichment`
Expected: PASS

**Step 3: Commit**

```bash
git add crates/amlich-api/tests/insight_enrichment.rs
git commit -m "test: add enriched insight integration tests"
```

---

## Task 15: Final Verification

**Step 1: Run full test suite**

Run: `cargo test --workspace`
Expected: All tests pass

**Step 2: Run CLI smoke tests**

```bash
cargo run -p amlich -- insight 2025-01-01 --format json --pretty
cargo run -p amlich -- insight 2025-06-15 --format text
cargo run -p amlich -- insight 2025-06-15 --format text --lang en
cargo run -p amlich -- config profile show
```

**Step 3: Verify backward compatibility**

Run: `cargo run -p amlich -- day 2025-01-01 --format json --pretty`
Expected: Existing output unchanged

**Step 4: Commit any fixes**

```bash
git add -A
git commit -m "fix: final adjustments for enriched insight system"
```
