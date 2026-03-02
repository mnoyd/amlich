# 🌙 VIỆT NAM LUNAR CALENDAR ENGINE - BẢNG CÔNG THỨC

## Mục lục
1. [Thiên Can & Địa Chi](#1-thiên-can--địa-chi)
2. [Thiên Can - Ngũ Hành](#2-thiên-can---ngũ-hành)
3. [Địa Chi - Ngũ Hành](#3-địa-chi---ngũ-hành)
4. [60 Hoa Giáp (Lục Thập Hoa Giáp)](#4-60-hoa-giáp)
5. [Nạp Âm (Na Yin)](#5-nạp-âm-na-yin)
6. [Tàng Can (Hidden Stems)](#6-tàng-can-hidden-stems)
7. [Tính Năm Trụ (Year Pillar)](#7-tính-năm-trụ-year-pillar)
8. [Tính Tháng Trụ (Month Pillar)](#8-tính-tháng-trụ-month-pillar)
9. [Tính Nhật Trụ (Day Pillar)](#9-tính-nhật-trụ-day-pillar)
10. [Tính Thì Trụ (Hour Pillar) - Ngũ Thử Độn Nguyên](#10-tính-thì-trụ-hour-pillar)
11. [Thập Thần (Ten Gods)](#11-thập-thần-ten-gods)
12. [Lục Xung & Tam Hợp](#12-lục-xung--tam-hợp)
13. [12 Trực (Twelve Duty Officers)](#13-12-trực-twelve-duty-officers)
14. [Đông/Tây Tứ Mệnh](#14-đôngtây-tứ-mệnh)
15. [Đại Vận (Major Luck)](#15-đại-vận-major-luck)

---

## 1. Thiên Can & Địa Chi

### 1.1 Thiên Can (10 Stems)

| Index | Can | Pinyin | Âm/Dương | Ngũ Hành |
|-------|-----|--------|----------|----------|
| 0 | 甲 | Jiǎ | Dương | Mộc |
| 1 | 乙 | Yǐ | Âm | Mộc |
| 2 | 丙 | Bǐng | Dương | Hỏa |
| 3 | 丁 | Dīng | Âm | Hỏa |
| 4 | 戊 | Wù | Dương | Thổ |
| 5 | 己 | Jǐ | Âm | Thổ |
| 6 | 庚 | Gēng | Dương | Kim |
| 7 | 辛 | Xīn | Âm | Kim |
| 8 | 壬 | Rén | Dương | Thủy |
| 9 | 癸 | Guǐ | Âm | Thủy |

### 1.2 Địa Chi (12 Branches)

| Index | Chi | Pinyin | Âm/Dương | Ngũ Hành | Con Giáp |
|-------|-----|--------|----------|----------|----------|
| 0 | 子 | Zǐ | Dương | Thủy | Tý (Chuột) |
| 1 | 丑 | Chǒu | Âm | Thổ | Sửu (Trâu) |
| 2 | 寅 | Yín | Dương | Mộc | Dần (Hổ) |
| 3 | 卯 | Mǎo | Âm | Mộc | Mão (Mèo) |
| 4 | 辰 | Chén | Dương | Thổ | Thìn (Rồng) |
| 5 | 巳 | Sì | Âm | Hỏa | Tỵ (Rắn) |
| 6 | 午 | Wǔ | Dương | Hỏa | Ngọ (Ngựa) |
| 7 | 未 | Wèi | Âm | Thổ | Mùi (Dê) |
| 8 | 申 | Shēn | Dương | Kim | Thân (Khỉ) |
| 9 | 酉 | Yǒu | Âm | Kim | Dậu (Gà) |
| 10 | 戌 | Xū | Dương | Thổ | Tuất (Chó) |
| 11 | 亥 | Hài | Âm | Thủy | Hợi (Lợn) |

---

## 2. Thiên Can - Ngũ Hành

```
// Rust Implementation
pub const TIANGAN: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
pub const TIANGAN_WUXING: [&str; 10] = ["Mộc", "Mộc", "Hỏa", "Hỏa", "Thổ", "Thổ", "Kim", "Kim", "Thủy", "Thủy"];
pub const TIANGAN_YINYANG: [&str; 10] = ["Dương", "Âm", "Dương", "Âm", "Dương", "Âm", "Dương", "Âm", "Dương", "Âm"];
```

### Công thức:
```
Can Index = (Năm - 4) % 10
Ngũ Hành Index = Can Index / 2 (làm tròn xuống)
```

| Can | Ngũ Hành | Phương | Mùa |
|-----|----------|--------|-----|
| 甲, 乙 | Mộc | Đông | Xuân |
| 丙, 丁 | Hỏa | Nam | Hạ |
| 戊, 己 | Thổ | Trung | Trường Hạ |
| 庚, 辛 | Kim | Tây | Thu |
| 壬, 癸 | Thủy | Bắc | Đông |

---

## 3. Địa Chi - Ngũ Hành

```
// Rust Implementation
pub const DIZHI: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
pub const DIZHI_WUXING: [&str; 12] = ["Thủy", "Thổ", "Mộc", "Mộc", "Thổ", "Hỏa", "Hỏa", "Thổ", "Kim", "Kim", "Thổ", "Thủy"];
```

### Công thức:
```
Chi Index = (Năm - 4) % 12
```

| Chi | Ngũ Hành | Tạng Can chính |
|-----|----------|----------------|
| 子 | Thủy | 癸 |
| 丑 | Thổ | 己, 癸, 辛 |
| 寅 | Mộc | 甲, 丙, 戊 |
| 卯 | Mộc | 乙 |
| 辰 | Thổ | 戊, 乙, 癸 |
| 巳 | Hỏa | 丙, 庚, 戊 |
| 午 | Hỏa | 丁, 己 |
| 未 | Thổ | 己, 丁, 乙 |
| 申 | Kim | 庚, 壬, 戊 |
| 酉 | Kim | 辛 |
| 戌 | Thổ | 戊, 辛, 丁 |
| 亥 | Thủy | 壬, 甲 |

---

## 4. 60 Hoa Giáp (Lục Thập Hoa Giáp)

### Công thức:
```
Can Index = index % 10
Chi Index = index % 12
```

| Index | Can Chi | Index | Can Chi | Index | Can Chi |
|-------|---------|-------|---------|-------|---------|
| 0 | 甲子 | 20 | 甲申 | 40 | 甲辰 |
| 1 | 乙丑 | 21 | 乙酉 | 41 | 乙巳 |
| 2 | 丙寅 | 22 | 丙戌 | 42 | 丙午 |
| 3 | 丁卯 | 23 | 丁亥 | 43 | 丁未 |
| 4 | 戊辰 | 24 | 戊子 | 44 | 戊申 |
| 5 | 己巳 | 25 | 己丑 | 45 | 己酉 |
| 6 | 庚午 | 26 | 庚寅 | 46 | 庚戌 |
| 7 | 辛未 | 27 | 辛卯 | 47 | 辛亥 |
| 8 | 壬申 | 28 | 壬辰 | 48 | 壬子 |
| 9 | 癸酉 | 29 | 癸巳 | 49 | 癸丑 |
| 10 | 甲戌 | 30 | 甲午 | 50 | 甲寅 |
| 11 | 乙亥 | 31 | 乙未 | 51 | 乙卯 |
| 12 | 丙子 | 32 | 丙申 | 52 | 丙辰 |
| 13 | 丁丑 | 33 | 丁酉 | 53 | 丁巳 |
| 14 | 戊寅 | 34 | 戊戌 | 54 | 戊午 |
| 15 | 己卯 | 35 | 己亥 | 55 | 己未 |
| 16 | 庚辰 | 36 | 庚子 | 56 | 庚申 |
| 17 | 辛巳 | 37 | 辛丑 | 57 | 辛酉 |
| 18 | 壬午 | 38 | 壬寅 | 58 | 壬戌 |
| 19 | 癸未 | 39 | 癸卯 | 59 | 癸亥 |

---

## 5. Nạp Âm (Na Yin)

### Bảng đầy đủ 60 Nạp Âm:

```rust
pub const NAYIN: [(&str, &str); 60] = [
    // Index 0-9
    ("甲子", "Hải Trung Kim"),  // Kim trong biển
    ("乙丑", "Hải Trung Kim"),
    ("丙寅", "Lô Trung Hỏa"),   // Lửa trong lò
    ("丁卯", "Lô Trung Hỏa"),
    ("戊辰", "Đại Lâm Mộc"),    // Cây rừng lớn
    ("己巳", "Đại Lâm Mộc"),
    ("庚午", "Lộ Bàng Thổ"),    // Đất bên đường
    ("辛未", "Lộ Bàng Thổ"),
    ("壬申", "Kiếm Phong Kim"), // Kim mũi kiếm
    ("癸酉", "Kiếm Phong Kim"),
    // Index 10-19
    ("甲戌", "Sơn Đầu Hỏa"),    // Lửa trên núi
    ("乙亥", "Sơn Đầu Hỏa"),
    ("丙子", "Giản Hạ Thủy"),   // Nước dưới khe
    ("丁丑", "Giản Hạ Thủy"),
    ("戊寅", "Thành Đầu Thổ"),  // Đất trên thành
    ("己卯", "Thành Đầu Thổ"),
    ("庚辰", "Bạch Lạp Kim"),   // Kim sáp trắng
    ("辛巳", "Bạch Lạp Kim"),
    ("壬午", "Dương Liễu Mộc"), // Cây dương liễu
    ("癸未", "Dương Liễu Mộc"),
    // Index 20-29
    ("甲申", "Tỉnh Tuyền Thủy"), // Nước giếng suối
    ("乙酉", "Tỉnh Tuyền Thủy"),
    ("丙戌", "Ốc Thượng Thổ"),   // Đất trên mái nhà
    ("丁亥", "Ốc Thượng Thổ"),
    ("戊子", "Tích Lịch Hỏa"),   // Lửa sấm sét
    ("己丑", "Tích Lịch Hỏa"),
    ("庚寅", "Tùng Bách Mộc"),   // Cây tùng bách
    ("辛卯", "Tùng Bách Mộc"),
    ("壬辰", "Trường Lưu Thủy"), // Nước sông dài
    ("癸巳", "Trường Lưu Thủy"),
    // Index 30-39
    ("甲午", "Sa Trung Kim"),    // Kim trong cát
    ("乙未", "Sa Trung Kim"),
    ("丙申", "Sơn Hạ Hỏa"),      // Lửa dưới núi
    ("丁酉", "Sơn Hạ Hỏa"),
    ("戊戌", "Bình Địa Mộc"),    // Cây đồng bằng
    ("己亥", "Bình Địa Mộc"),
    ("庚子", "Bích Thượng Thổ"), // Đất trên vách
    ("辛丑", "Bích Thượng Thổ"),
    ("壬寅", "Kim Bạch Kim"),    // Kim mạ vàng
    ("癸卯", "Kim Bạch Kim"),
    // Index 40-49
    ("甲辰", "Phú Đăng Hỏa"),    // Lửa đèn phú
    ("乙巳", "Phú Đăng Hỏa"),
    ("丙午", "Thiên Hà Thủy"),   // Nước sông trời
    ("丁未", "Thiên Hà Thủy"),
    ("戊申", "Đại Trạch Thổ"),   // Đất đầm lớn
    ("己酉", "Đại Trạch Thổ"),
    ("庚戌", "Xuyến Xoa Kim"),   // Kim trâm cài
    ("辛亥", "Xuyến Xoa Kim"),
    ("壬子", "Tang Đồ Mộc"),     // Cây dâu tằm
    ("癸丑", "Tang Đồ Mộc"),
    // Index 50-59
    ("甲寅", "Đại Khê Thủy"),    // Nước suối lớn
    ("乙卯", "Đại Khê Thủy"),
    ("丙辰", "Sa Trung Thổ"),    // Đất trong cát
    ("丁巳", "Sa Trung Thổ"),
    ("戊午", "Thiên Thượng Hỏa"), // Lửa trên trời
    ("己未", "Thiên Thượng Hỏa"),
    ("庚申", "Thạch Lựu Mộc"),    // Cây thạch lựu
    ("辛酉", "Thạch Lựu Mộc"),
    ("壬戌", "Đại Hải Thủy"),     // Nước biển lớn
    ("癸亥", "Đại Hải Thủy"),
];
```

### Công thức tra Nạp Âm:
```rust
pub fn get_nayin(year: i32) -> &'static str {
    let index = ((year - 4) % 60 + 60) % 60;
    NAYIN[index as usize].1
}
```

---

## 6. Tàng Can (Hidden Stems)

### Bảng Tàng Can trong mỗi Địa Chi:

```rust
pub const CANGAN: [[&str; 3]; 12] = [
    // [Chính, Trung, Dư]
    ["癸", "", ""],           // 子 - Chỉ có Quý
    ["己", "癸", "辛"],       // 丑 - Kỷ chính, Quý trung, Tân dư
    ["甲", "丙", "戊"],       // 寅 - Giáp chính, Bính trung, Mậu dư
    ["乙", "", ""],           // 卯 - Chỉ có Ất
    ["戊", "乙", "癸"],       // 辰 - Mậu chính, Ất trung, Quý dư
    ["丙", "庚", "戊"],       // 巳 - Bính chính, Canh trung, Mậu dư
    ["丁", "己", ""],         // 午 - Đinh chính, Kỷ trung
    ["己", "丁", "乙"],       // 未 - Kỷ chính, Đinh trung, Ất dư
    ["庚", "壬", "戊"],       // 申 - Canh chính, Nhâm trung, Mậu dư
    ["辛", "", ""],           // 酉 - Chỉ có Tân
    ["戊", "辛", "丁"],       // 戌 - Mậu chính, Tân trung, Đinh dư
    ["壬", "甲", ""],         // 亥 - Nhâm chính, Giáp trung
];

// Độ mạnh của Tàng Can
pub const CANGAN_STRENGTH: [[u8; 3]; 12] = [
    [100, 0, 0],    // 子
    [60, 25, 15],   // 丑
    [60, 25, 15],   // 寅
    [100, 0, 0],    // 卯
    [60, 25, 15],   // 辰
    [60, 25, 15],   // 巳
    [70, 30, 0],    // 午
    [60, 25, 15],   // 未
    [60, 25, 15],   // 申
    [100, 0, 0],    // 酉
    [60, 25, 15],   // 戌
    [70, 30, 0],    // 亥
];
```

---

## 7. Tính Năm Trụ (Year Pillar)

### Công thức:
```rust
pub fn get_year_pillar(year: i32) -> (String, String) {
    let gan_index = ((year - 4) % 10 + 10) % 10;
    let zhi_index = ((year - 4) % 12 + 12) % 12;
    
    let gan = TIANGAN[gan_index as usize];
    let zhi = DIZHI[zhi_index as usize];
    
    (gan.to_string(), zhi.to_string())
}
```

### Ví dụ:
- Năm 2025: `(2025 - 4) % 10 = 1` → 乙, `(2025 - 4) % 12 = 5` → 巳 → **Ất Tỵ**
- Năm 2024: **Giáp Thìn**

---

## 8. Tính Tháng Trụ (Month Pillar)

### 24 Tiết Khí:

```rust
pub const JIEQI: [&str; 24] = [
    "Xuân Phân", "Thanh Minh", "Cốc Vũ", "Lập Hạ",
    "Tiểu Mãn", "Mang Chủng", "Hạ Chí", "Tiểu Thử",
    "Đại Thử", "Lập Thu", "Xử Thử", "Bạch Lộ",
    "Thu Phân", "Hàn Lộ", "Sương Giáng", "Lập Đông",
    "Tiểu Tuyết", "Đại Tuyết", "Đông Chí", "Tiểu Hàn",
    "Đại Hàn", "Lập Xuân", "Vũ Thủy", "Kinh Trập"
];
```

### Bảng Tháng Can Chi:

| Tháng âm | Địa Chi | Tiết Khí bắt đầu |
|----------|---------|------------------|
| 1 | 寅 | Lập Xuân |
| 2 | 卯 | Kinh Trập |
| 3 | 辰 | Thanh Minh |
| 4 | 巳 | Lập Hạ |
| 5 | 午 | Mang Chủng |
| 6 | 未 | Tiểu Thử |
| 7 | 申 | Lập Thu |
| 8 | 酉 | Bạch Lộ |
| 9 | 戌 | Hàn Lộ |
| 10 | 亥 | Lập Đông |
| 11 | 子 | Đại Tuyết |
| 12 | 丑 | Tiểu Hàn |

### Công thức tính Can tháng (Ngũ Hổ Độn):

```rust
// "甲己之年丙作首" - Năm Giáp/Kỷ, tháng 1 bắt đầu từ Bính
// "乙庚之岁戊为头" - Năm Ất/Canh, tháng 1 bắt đầu từ Mậu
// "丙辛之岁寻庚上" - Năm Bính/Tân, tháng 1 bắt đầu từ Canh
// "丁壬壬位顺行流" - Năm Đinh/Nhâm, tháng 1 bắt đầu từ Nhâm
// "戊癸之岁甲寅首" - Năm Mậu/Quý, tháng 1 bắt đầu từ Giáp

pub fn get_month_gan(year_gan_index: usize, month: usize) -> usize {
    let start_gan = match year_gan_index {
        0 | 5 => 2,  // 甲, 己 → 丙
        1 | 6 => 4,  // 乙, 庚 → 戊
        2 | 7 => 6,  // 丙, 辛 → 庚
        3 | 8 => 8,  // 丁, 壬 → 壬
        4 | 9 => 0,  // 戊, 癸 → 甲
        _ => 0,
    };
    (start_gan + month - 1) % 10
}
```

---

## 9. Tính Nhật Trụ (Day Pillar)

### Công thức Zeller (điều chỉnh):

```rust
pub fn get_day_pillar(year: i32, month: i32, day: i32) -> (String, String) {
    // Công thức tính ngày Julius
    let a = (14 - month) / 12;
    let y = year + 4800 - a;
    let m = month + 12 * a - 3;
    
    // Julian Day Number
    let jdn = day + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045;
    
    // Ngày Can Chi (tính từ ngày chuẩn 1/1/1900 = Giáp Tý)
    let base_jdn = 2415021; // 1/1/1900 Julian Day
    let diff = jdn - base_jdn;
    
    let gan_index = ((diff % 10) + 10) % 10;
    let zhi_index = ((diff % 12) + 12) % 12;
    
    (TIANGAN[gan_index as usize].to_string(), DIZHI[zhi_index as usize].to_string())
}
```

---

## 10. Tính Thì Trụ (Hour Pillar) - Ngũ Thử Độn Nguyên

### 12 Giờ Địa Chi:

| Giờ | Địa Chi | Thời gian |
|-----|---------|-----------|
| Tý | 子 | 23:00 - 01:00 |
| Sửu | 丑 | 01:00 - 03:00 |
| Dần | 寅 | 03:00 - 05:00 |
| Mão | 卯 | 05:00 - 07:00 |
| Thìn | 辰 | 07:00 - 09:00 |
| Tỵ | 巳 | 09:00 - 11:00 |
| Ngọ | 午 | 11:00 - 13:00 |
| Mùi | 未 | 13:00 - 15:00 |
| Thân | 申 | 15:00 - 17:00 |
| Dậu | 酉 | 17:00 - 19:00 |
| Tuất | 戌 | 19:00 - 21:00 |
| Hợi | 亥 | 21:00 - 23:00 |

### Công thức Ngũ Thử Độn (Five Rats Escape):

```rust
// Bài quyết:
// "甲己还加甲" - Ngày Giáp/Kỷ, giờ Tý = Giáp Tý
// "乙庚丙作初" - Ngày Ất/Canh, giờ Tý = Bính Tý
// "丙辛从戊起" - Ngày Bính/Tân, giờ Tý = Mậu Tý
// "丁壬庚子居" - Ngày Đinh/Nhâm, giờ Tý = Canh Tý
// "戊癸起壬子" - Ngày Mậu/Quý, giờ Tý = Nhâm Tý

pub fn get_hour_gan(day_gan_index: usize, hour: usize) -> usize {
    let start_gan = match day_gan_index {
        0 | 5 => 0,  // 甲, 己 → 甲
        1 | 6 => 2,  // 乙, 庚 → 丙
        2 | 7 => 4,  // 丙, 辛 → 戊
        3 | 8 => 6,  // 丁, 壬 → 庚
        4 | 9 => 8,  // 戊, 癸 → 壬
        _ => 0,
    };
    (start_gan + hour) % 10
}

pub fn get_hour_zhi(hour: i32) -> usize {
    // hour: 0-23
    let zhi = match hour {
        23 | 0 => 0,   // 子
        1..=2 => 1,    // 丑
        3..=4 => 2,    // 寅
        5..=6 => 3,    // 卯
        7..=8 => 4,    // 辰
        9..=10 => 5,   // 巳
        11..=12 => 6,  // 午
        13..=14 => 7,  // 未
        15..=16 => 8,  // 申
        17..=18 => 9,  // 酉
        19..=20 => 10, // 戌
        21..=22 => 11, // 亥
        _ => 0,
    };
    zhi
}
```

### Bảng tra Can Giờ:

| Ngày Can | Giờ Tý | Giờ Sửu | Giờ Dần | Giờ Mão | Giờ Thìn | Giờ Tỵ | Giờ Ngọ | Giờ Mùi | Giờ Thân | Giờ Dậu | Giờ Tuất | Giờ Hợi |
|----------|--------|---------|---------|---------|----------|--------|---------|---------|----------|---------|----------|---------|
| 甲, 己 | 甲子 | 乙丑 | 丙寅 | 丁卯 | 戊辰 | 己巳 | 庚午 | 辛未 | 壬申 | 癸酉 | 甲戌 | 乙亥 |
| 乙, 庚 | 丙子 | 丁丑 | 戊寅 | 己卯 | 庚辰 | 辛巳 | 壬午 | 癸未 | 甲申 | 乙酉 | 丙戌 | 丁亥 |
| 丙, 辛 | 戊子 | 己丑 | 庚寅 | 辛卯 | 壬辰 | 癸巳 | 甲午 | 乙未 | 丙申 | 丁酉 | 戊戌 | 己亥 |
| 丁, 壬 | 庚子 | 辛丑 | 壬寅 | 癸卯 | 甲辰 | 乙巳 | 丙午 | 丁未 | 戊申 | 己酉 | 庚戌 | 辛亥 |
| 戊, 癸 | 壬子 | 癸丑 | 甲寅 | 乙卯 | 丙辰 | 丁巳 | 戊午 | 己未 | 庚申 | 辛酉 | 壬戌 | 癸亥 |

---

## 11. Thập Thần (Ten Gods)

### Bảng Thập Thần (dựa trên Ngũ Hành):

```rust
pub fn get_ten_gods(day_gan_wuxing: &str, target_wuxing: &str, same_yinyang: bool) -> &'static str {
    // So sánh ngũ hành của Can ngày với ngũ hành mục tiêu
    // same_yinyang: cùng âm dương hay khác âm dương
    
    match (day_gan_wuxing, target_wuxing) {
        // Sinh ra tôi (sinh nhập) - Ấn
        ("Mộc", "Thủy") | ("Hỏa", "Mộc") | ("Thổ", "Hỏa") | 
        ("Kim", "Thổ") | ("Thủy", "Kim") => {
            if same_yinyang { "Thiên Ấn" } else { "Chính Ấn" }
        }
        // Tôi sinh ra (sinh xuất) - Thực Thương
        ("Thủy", "Mộc") | ("Mộc", "Hỏa") | ("Hỏa", "Thổ") | 
        ("Thổ", "Kim") | ("Kim", "Thủy") => {
            if same_yinyang { "Thực Thần" } else { "Thương Quan" }
        }
        // Tôi khắc (tài) - Tài
        ("Mộc", "Thổ") | ("Hỏa", "Kim") | ("Thổ", "Thủy") | 
        ("Kim", "Mộc") | ("Thủy", "Hỏa") => {
            if same_yinyang { "Thiên Tài" } else { "Chính Tài" }
        }
        // Khắc tôi (quan sát) - Quan Sát
        ("Thổ", "Mộc") | ("Kim", "Hỏa") | ("Thủy", "Thổ") | 
        ("Mộc", "Kim") | ("Hỏa", "Thủy") => {
            if same_yinyang { "Thất Sát" } else { "Chính Quan" }
        }
        // Cùng ngũ hành - Tỷ Kiếp
        _ => {
            if same_yinyang { "Tỷ Kiên" } else { "Kiếp Tài" }
        }
    }
}
```

### Bảng tóm tắt:

| Quan hệ | Cùng Âm Dương | Khác Âm Dương |
|---------|---------------|---------------|
| Cùng ngũ hành | Tỷ Kiên | Kiếp Tài |
| Tôi sinh | Thực Thần | Thương Quan |
| Sinh tôi | Thiên Ấn | Chính Ấn |
| Tôi khắc | Thiên Tài | Chính Tài |
| Khắc tôi | Thất Sát | Chính Quan |

### Viết tắt:

| Viết tắt | Thập Thần |
|----------|-----------|
| Tỷ | Tỷ Kiên |
| Kiếp | Kiếp Tài |
| Thực | Thực Thần |
| Thương | Thương Quan |
| Tài | Thiên Tài |
| Chính Tài | Chính Tài |
| Sát | Thất Sát |
| Quan | Chính Quan |
| Ấn | Thiên Ấn |
| Chính Ấn | Chính Ấn |

---

## 12. Lục Xung & Tam Hợp

### Lục Xung (6 Cặp Xung):

```rust
pub const LIUCHONG: [(usize, usize); 6] = [
    (0, 6),   // 子午 xung (Tý - Ngọ)
    (1, 7),   // 丑未 xung (Sửu - Mùi)
    (2, 8),   // 寅申 xung (Dần - Thân)
    (3, 9),   // 卯酉 xung (Mão - Dậu)
    (4, 10),  // 辰戌 xung (Thìn - Tuất)
    (5, 11),  // 巳亥 xung (Tỵ - Hợi)
];
```

### Tam Hợp (3 Nhóm Hợp):

```rust
pub const SANHE: [[usize; 3]; 4] = [
    [0, 4, 8],   // 申子辰 - Thân Tý Thìn = Thủy cục
    [2, 6, 10],  // 寅午戌 - Dần Ngọ Tuất = Hỏa cục
    [3, 5, 9],   // 巳酉丑 - Tỵ Dậu Sửu = Kim cục
    [1, 7, 11],  // 亥卯未 - Hợi Mão Mùi = Mộc cục
];
```

### Lục Hợp (6 Cặp Hợp):

```rust
pub const LIUHE: [(usize, usize); 6] = [
    (0, 1),   // 子丑 hợp - Thổ
    (2, 11),  // 寅亥 hợp - Mộc
    (3, 10),  // 卯戌 hợp - Hỏa
    (4, 9),   // 辰酉 hợp - Kim
    (5, 8),   // 巳申 hợp - Thủy
    (6, 7),   // 午未 hợp - Thổ/Hỏa
];
```

### Tương Hại (6 Cặp Hại):

```rust
pub const XIANGHAI: [(usize, usize); 6] = [
    (0, 7),   // 子未 hại
    (1, 6),   // 丑午 hại
    (2, 9),   // 寅酉 hại
    (3, 8),   // 卯申 hại
    (4, 11),  // 辰亥 hại
    (5, 10),  // 巳戌 hại
];
```

### Tương Hình (Hình):

```rust
pub const XIANGXING: [[usize; 3]; 4] = [
    [2, 3, 5],   // 寅卯巳 - Vô恩之刑
    [0, 1, 4],   // 子辰丑 - 恃势之刑
    [8, 9, 11],  // 申酉亥 - 无礼之刑
    [6, 6, 6],   // 午午 - 自刑
];
```

---

## 13. 12 Trực (Twelve Duty Officers)

### Bảng 12 Trực:

| Index | Trực | Ý nghĩa |
|-------|------|---------|
| 0 | 建 | Kiến - Xây dựng |
| 1 | 除 | Trừ - Loại bỏ |
| 2 | 满 | Mãn - Đầy đủ |
| 3 | 平 | Bình - Bình an |
| 4 | 定 | Định - Ổn định |
| 5 | 执 | Chấp - Nắm giữ |
| 6 | 破 | Phá - Phá vỡ |
| 7 | 危 | Nguy - Nguy hiểm |
| 8 | 成 | Thành - Thành công |
| 9 | 收 | Thu - Thu hoạch |
| 10 | 开 | Khai - Mở rộng |
| 11 | 闭 | Bế - Đóng lại |

### Công thức tính Trực:

```rust
pub fn get_truc(day_zhi: usize, month_zhi: usize) -> usize {
    // Trực đầu tháng = Địa chi tháng
    // Các ngày sau tính theo thứ tự
    (day_zhi + 12 - month_zhi) % 12
}
```

---

## 14. Đông/Tây Tứ Mệnh

### Công thức tính Cung Mệnh:

```rust
pub fn get_kua(year: i32, gender: char) -> usize {
    let year_last_two = year % 100;
    
    let remainder = match gender {
        'M' | 'm' => (100 - year_last_two) % 9,  // Nam
        'F' | 'f' => (year_last_two - 4) % 9,    // Nữ
        _ => 0,
    };
    
    // Xử lý trường hợp remainder = 0
    let kua = if remainder == 0 { 9 } else { remainder };
    
    kua
}
```

### Bảng Đông/Tây Tứ Mệnh:

| Cung | Kua | Phương | Loại |
|------|-----|--------|------|
| Khảm | 1 | Bắc | Đông Tứ |
| Khôn | 2 | Tây Nam | Tây Tứ |
| Chấn | 3 | Đông | Đông Tứ |
| Tốn | 4 | Đông Nam | Đông Tứ |
| (Trung) | 5 | Trung | Nam→Khôn, Nữ→Cấn |
| Càn | 6 | Tây Bắc | Tây Tứ |
| Đoài | 7 | Tây | Tây Tứ |
| Cấn | 8 | Đông Bắc | Tây Tứ |
| Ly | 9 | Nam | Đông Tứ |

### Phân loại:

```rust
pub fn is_east_kua(kua: usize) -> bool {
    matches!(kua, 1 | 3 | 4 | 9)  // Khảm, Chấn, Tốn, Ly
}

pub fn is_west_kua(kua: usize) -> bool {
    matches!(kua, 2 | 6 | 7 | 8)  // Khôn, Càn, Đoài, Cấn
}
```

---

## 15. Đại Vận (Major Luck)

### Công thức tính:

```rust
pub struct DaYun {
    pub order: i32,        // 1 = Thuận, -1 = Nghịch
    pub start_age: i32,    // Tuổi bắt đầu
    pub pillars: Vec<(String, String)>, // Các trụ đại vận
}

pub fn calculate_dayun(
    year_zhi: usize,       // Địa chi năm sinh
    gender: char,          // Giới tính
    month_pillar: (usize, usize), // Can Chi tháng
    birth_to_jieqi_days: i32, // Số ngày từ sinh đến tiết khí gần nhất
) -> DaYun {
    // Xác định Âm/Dương theo năm
    let is_yang_year = year_zhi % 2 == 0; // Tý, Dần, Thìn, Ngọ, Thân, Tuất = Dương
    
    // Xác định chiều Đại Vận
    let order = match (is_yang_year, gender) {
        (true, 'M') | (false, 'F') => 1,  // Thuận
        _ => -1,                          // Nghịch
    };
    
    // Tính tuổi bắt đầu Đại Vận
    // 3 ngày = 1 năm
    let start_age = birth_to_jieqi_days / 3;
    
    // Tính các trụ Đại Vận (10 năm mỗi trụ)
    let mut pillars = Vec::new();
    let (mut gan_idx, mut zhi_idx) = month_pillar;
    
    for _ in 0..8 {  // Thường có 8 Đại Vận
        gan_idx = (gan_idx as i32 + order + 10) as usize % 10;
        zhi_idx = (zhi_idx as i32 + order + 12) as usize % 12;
        pillars.push((
            TIANGAN[gan_idx].to_string(),
            DIZHI[zhi_idx].to_string()
        ));
    }
    
    DaYun {
        order,
        start_age,
        pillars,
    }
}
```

### Bảng tóm tắt quy tắc:

| Năm | Giới | Chiều Đại Vận |
|-----|------|---------------|
| Dương | Nam | Thuận (+) |
| Dương | Nữ | Nghịch (-) |
| Âm | Nam | Nghịch (-) |
| Âm | Nữ | Thuận (+) |

### Tính tuổi bắt đầu:

```
Tuổi bắt đầu = Số ngày từ ngày sinh đến tiết khí gần nhất / 3
(3 ngày = 1 năm, 1 ngày = 4 tháng, 1 giờ = 10 ngày)
```

---

## PHỤ LỤC: Code Rust Hoàn Chỉnh

### File: `src/constants.rs`

```rust
// Thiên Can
pub const TIANGAN: [&str; 10] = [
    "甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"
];

pub const TIANGAN_VN: [&str; 10] = [
    "Giáp", "Ất", "Bính", "Đinh", "Mậu", "Kỷ", "Canh", "Tân", "Nhâm", "Quý"
];

pub const TIANGAN_WUXING: [&str; 10] = [
    "Mộc", "Mộc", "Hỏa", "Hỏa", "Thổ", "Thổ", "Kim", "Kim", "Thủy", "Thủy"
];

// Địa Chi
pub const DIZHI: [&str; 12] = [
    "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"
];

pub const DIZHI_VN: [&str; 12] = [
    "Tý", "Sửu", "Dần", "Mão", "Thìn", "Tỵ", "Ngọ", "Mùi", "Thân", "Dậu", "Tuất", "Hợi"
];

pub const DIZHI_WUXING: [&str; 12] = [
    "Thủy", "Thổ", "Mộc", "Mộc", "Thổ", "Hỏa", "Hỏa", "Thổ", "Kim", "Kim", "Thổ", "Thủy"
];

// 12 Trực
pub const TRUC: [&str; 12] = [
    "Kiến", "Trừ", "Mãn", "Bình", "Định", "Chấp",
    "Phá", "Nguy", "Thành", "Thu", "Khai", "Bế"
];
```

---

*Tài liệu này được tổng hợp từ các nguồn:*
- *作业部落 (zybuluo.com) - 八字排盘算法列表*
- *腾讯云 (cloud.tencent.com) - 八字排盘JAVA实现*
- *百度百科 (baike.baidu.com)*
- *Wikipedia*
