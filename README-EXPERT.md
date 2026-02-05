# Vietnamese Lunar Calendar Expert Engine (Âm Lịch Việt Nam) 🌙

A comprehensive Vietnamese Lunar Calendar library with **Expert Engine** support for traditional almanac features including Can Chi, Ngũ Hành, and more.

## Features

### ✅ Core Features
- Solar ↔ Lunar date conversion (accurate 1900-2199)
- Vietnamese holiday calculations (Tết, Vu Lan, Trung Thu, etc.)
- Export to Google Calendar (CSV/ICS format)
- Beautiful desktop web viewer
- Command-line interface (CLI)

### 🌟 Expert Engine Features

#### ✅ Implemented
- **Can Chi (干支)** - Heavenly Stems & Earthly Branches
  - Day Can Chi (JD-based: verified against Tết dates)
  - Month Can Chi (lunar month + year stem table)
  - Year Can Chi (lunar year formula)
- **Con Giáp** - Vietnamese Zodiac animals (12 animals)
- **Ngũ Hành** - Five Elements (Mộc, Hỏa, Thổ, Kim, Thủy)
- **24 Solar Terms (Tiết khí)** - Seasonal markers
  - Astronomical calculation based on sun longitude
  - All 24 terms with Vietnamese names
  - Season classification (Spring/Summer/Autumn/Winter)
- **Giờ Hoàng Đạo** - Auspicious Hours
  - 12-Star System (Thập Nhị Kiến Trừ)
  - 6 Good Stars: Thanh Long, Minh Đường, Kim Quỹ, Bảo Quang, Ngọc Đường, Tư Mệnh
  - 6 Bad Stars: Thiên Hình, Chu Tước, Bạch Hổ, Thiên Lao, Nguyên Vũ, Câu Trận
  - Hour-by-hour analysis with time ranges

#### 🔲 Coming Soon
- **12 Trực** - 12 day officers
- **Nạp Âm** - 60-cycle element mapping
- **Ngày Hoàng Đạo/Hắc Đạo** - Day classifications
- **Nhị thập bát tú** - 28 star mansions
- **Xung/Hợp** - Conflict/harmony relations

---

## Installation

```bash
# No dependencies needed! Pure Node.js
git clone <repo>
cd amlich-view
```

---

## CLI Usage

### Show Today with Can Chi Info

```bash
node index.js today
```

**Output:**
```
📅 Ngày 2024-02-10 (Thứ Bảy)
🌙 Âm lịch: 1/1/2024
📜 Can Chi:
   • Ngày: Giáp Thìn (Thìn (Rồng))
   • Tháng: Bính Dần
   • Năm: Giáp Thìn (Thìn (Rồng))
🌟 Ngũ hành:
   • Ngày: Mộc (Can) - Thổ (Chi)
🌤️  Tiết khí: Lập Xuân - Đông (Winter)
   • Start of Spring (Lập Xuân)
   • Kinh độ mặt trời: 320.44°
⏰ Giờ Hoàng Đạo (6 giờ tốt):
   • Dần (03:00-05:00) - Tư Mệnh
   • Thìn (07:00-09:00) - Thanh Long
   • Tỵ (09:00-11:00) - Minh Đường
   • Thân (15:00-17:00) - Kim Quỹ
   • Dậu (17:00-19:00) - Bảo Quang
   • Hợi (21:00-23:00) - Ngọc Đường
```

### Get Detailed Info for Any Date

```bash
node index.js info 10 2 2024    # Tết 2024
node index.js info 29 1 2025    # Tết 2025
```

### Show All Holidays

```bash
node index.js show 2024
```

### Convert Dates

```bash
# Lunar → Solar
node index.js convert 1 1 2024 lunar

# Solar → Lunar
node index.js convert 10 2 2024 solar
```

### Export to Calendar Apps

```bash
node index.js export-ics 2024    # iCal format (recommended)
node index.js export-csv 2024    # CSV format
```

---

## Library Usage

### Basic Date Conversion

```javascript
const { getLunarDate, getSolarDate } = require('./vietnamese-holidays.js');

// Solar → Lunar
const lunar = getLunarDate(10, 2, 2024);
console.log(lunar);
// { day: 1, month: 1, year: 2024, isLeapMonth: false }

// Lunar → Solar
const solar = getSolarDate(1, 1, 2024);
console.log(solar);
// { day: 10, month: 2, year: 2024 }
```

### Expert Engine - Can Chi & Almanac

```javascript
const { getDayInfo, formatDayInfo } = require('./engine/index.js');

// Get complete day information
const info = getDayInfo(10, 2, 2024);

// Access Can Chi data
console.log(info.canChi.day.full);      // "Giáp Thìn"
console.log(info.canChi.month.full);    // "Bính Dần"
console.log(info.canChi.year.full);     // "Giáp Thìn"

// Access zodiac and elements
console.log(info.canChi.day.conGiap);   // "Thìn (Rồng)"
console.log(info.canChi.day.nguHanh);   // { can: "Mộc", chi: "Thổ" }

// Access Solar Term
console.log(info.tietKhi.name);         // "Lập Xuân"
console.log(info.tietKhi.season);       // "Đông (Winter)"
console.log(info.tietKhi.currentLongitude); // 320.44

// Access Auspicious Hours
console.log(info.gioHoangDao.goodHourCount);  // 6
info.gioHoangDao.goodHours.forEach(h => {
    console.log(`${h.hourChi} (${h.timeRange}): ${h.star}`);
});

// Pretty print
console.log(formatDayInfo(info));
```

### Get Holidays

```javascript
const { getVietnameseHolidays } = require('./vietnamese-holidays.js');

const holidays = getVietnameseHolidays(2024);
holidays.forEach(h => {
    console.log(`${h.dateString}: ${h.name}`);
});
```

---

## Technical Details

### Can Chi Calculation Formulas

The Expert Engine uses verified formulas from traditional Vietnamese almanac systems:

#### Day Can Chi (based on Julian Day Number)
```
Can (Stem):   (JD + 9) % 10
Chi (Branch): (JD + 1) % 12
```
- Verified against Tết dates: 2023, 2024, 2025, 2026
- Test suite includes Y2K reference date

#### Month Can Chi
- **Chi**: Fixed by lunar month
  - Month 1 = Dần (index 2)
  - Month 2 = Mão (index 3)
  - etc.
- **Can**: Determined by year stem using traditional table
  ```
  Year Can    | Month 1 (Dần) starts with
  ------------|---------------------------
  Giáp/Kỷ     → Bính (index 2)
  Ất/Canh     → Mậu (index 4)
  Bính/Tân    → Canh (index 6)
  Đinh/Nhâm   → Nhâm (index 8)
  Mậu/Quý     → Giáp (index 0)
  ```

#### Year Can Chi
```
Can: (lunar_year + 6) % 10
Chi: (lunar_year + 8) % 12
```

### Architecture

```
amlich-view/
├── amlich-core.js           # Core lunar algorithm (Hồ Ngọc Đức)
├── vietnamese-holidays.js   # Holiday calculations
├── engine/                  # 🌟 Expert Engine
│   ├── index.js            # Main entry: getDayInfo()
│   ├── types.js            # Can/Chi constants & types
│   ├── canchi.js           # Can Chi calculations
│   ├── tietkhi.js          # Solar Terms (24 terms)
│   ├── gio-hoang-dao.js    # Auspicious Hours (12-Star System)
│   └── test.js             # Test suite (6 reference dates)
├── index.js                # CLI application
├── app.js                  # Web app logic
└── index.html              # Web viewer
```

### Core Algorithm Source

- **Lunar calculations**: Hồ Ngọc Đức's astronomical algorithm
- **Based on**: Jean Meeus' "Astronomical Algorithms" (1998)
- **Timezone**: UTC+7 (Vietnam)
- **Accuracy**: Years 1900-2199

---

## Vietnamese Holidays Included

**Major Festivals:**
- 🎊 Tết Nguyên Đán (Lunar New Year)
- 🏮 Tết Nguyên Tiêu (Lantern Festival)
- 🌸 Thanh Minh (Tomb Sweeping Day)
- 🙏 Phật Đản (Buddha's Birthday)
- 🐉 Tết Đoan Ngọ (Dragon Boat Festival)
- 👪 Vu Lan (Parents' Day / Wandering Souls)
- 🥮 Tết Trung Thu (Mid-Autumn Festival)
- 🏔️ Tết Trùng Cửu (Double Ninth)
- 🎋 Tết Hạ Nguyên (Lower Nguyên Festival)
- 🍲 Ông Táo chầu trời (Kitchen Gods' Day)

**Monthly Events:**
- 🌑 Mùng 1 (New Moon - 1st of each lunar month)
- 🌕 Rằm (Full Moon - 15th of each lunar month)

---

## Testing

Run the test suite to verify Can Chi calculations:

```bash
# Core engine tests
node engine/test.js

# Holiday calculations
node test.js
```

---

## Web Viewer

Open `index.html` in a browser for a beautiful desktop calendar viewer with:
- Month/year navigation
- Lunar date overlay
- Holiday highlighting
- Export functionality

---

## Import to Google Calendar

### Method 1: iCal (Recommended)
```bash
node index.js export-ics 2024
```
1. Go to [Google Calendar](https://calendar.google.com)
2. Settings ⚙️ → Import & Export
3. Select `vietnamese-calendar-2024.ics`
4. Import

### Method 2: CSV
```bash
node index.js export-csv 2024
```
Same import process as above.

---

## Development Roadmap

### Phase 1: Foundation ✅ COMPLETE
- [x] Can Chi for day/month/year
- [x] Con Giáp (zodiac animals)
- [x] Ngũ Hành (five elements)
- [x] Test suite with verified dates
- [x] CLI integration

### Phase 2: Solar Terms & Hours ✅ COMPLETE
- [x] 24 Tiết khí calculations
- [x] Solar term names in Vietnamese
- [x] Season classification
- [x] Giờ Hoàng Đạo (auspicious hours)
- [x] 12-Star System (Thập Nhị Kiến Trừ)
- [x] Integration with getDayInfo()

### Phase 3: Day Classifications 🔲 Planned
- [ ] Ngày Hoàng Đạo/Hắc Đạo (auspicious days)
- [ ] 12 Trực (day officers)

### Phase 4: Advanced Features 🔲 Planned
- [ ] Nạp Âm (60-cycle elements)
- [ ] Nhị thập bát tú (28 mansions)
- [ ] Xung/Hợp relations

---

## License

- Core algorithm: Copyright (c) 2006 Ho Ngoc Duc
- Expert Engine & additions: MIT License

## Credits

- **Astronomical algorithms**: Ho Ngoc Duc
- **Based on**: Jean Meeus' "Astronomical Algorithms" (1998)
- **Expert Engine**: Built with traditional Vietnamese almanac knowledge

---

## Contributing

Contributions welcome! Areas of interest:
- Verification of Can Chi formulas against historical almanacs
- Additional almanac features (Trực, Tiết khí, etc.)
- UI/UX improvements
- Documentation in Vietnamese

---

**Made with ❤️ for Vietnamese culture and traditions**
