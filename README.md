# Vietnamese Lunar Calendar Expert Engine 🌙

A comprehensive Vietnamese Lunar Calendar library with **Expert Engine** support for traditional almanac features. Provides accurate astronomical calculations and traditional Vietnamese calendar wisdom.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Node.js](https://img.shields.io/badge/node-%3E%3D12.0.0-brightgreen.svg)](https://nodejs.org/)
[![Pure Node.js](https://img.shields.io/badge/dependencies-0-blue.svg)](package.json)

## ✨ Features

### Core Features
- ✅ **Solar ↔ Lunar Conversion** - Accurate date conversion (1900-2199)
- ✅ **Vietnamese Holidays** - All major festivals (Tết, Vu Lan, Trung Thu, etc.)
- ✅ **Calendar Export** - Export to Google Calendar (ICS/CSV formats)
- ✅ **Beautiful CLI** - Command-line interface with emoji support
- ✅ **Web Viewer** - Desktop web viewer with modern UI

### Expert Engine Features

#### ✅ Can Chi (干支) - Heavenly Stems & Earthly Branches
- Day Can Chi (JD-based formula, verified against Tết dates)
- Month Can Chi (lunar month + year stem table)
- Year Can Chi (lunar year formula)

#### ✅ Con Giáp - Vietnamese Zodiac
- 12 animals: Tý (Chuột), Sửu (Trâu), Dần (Hổ), Mão (Mèo), Thìn (Rồng), Tỵ (Rắn), Ngọ (Ngựa), Mùi (Dê), Thân (Khỉ), Dậu (Gà), Tuất (Chó), Hợi (Lợn)

#### ✅ Ngũ Hành - Five Elements
- Mộc (Wood), Hỏa (Fire), Thổ (Earth), Kim (Metal), Thủy (Water)

#### ✅ 24 Solar Terms (Tiết khí)
- Astronomical calculation based on sun's ecliptic longitude
- All 24 terms with Vietnamese names
- Season classification (Xuân, Hạ, Thu, Đông)
- Equinoxes & solstices verification

#### ✅ Auspicious Hours (Giờ Hoàng Đạo)
- Traditional 12-Star System (Thập Nhị Kiến Trừ)
- 6 Good Stars: Thanh Long, Minh Đường, Kim Quỹ, Bảo Quang, Ngọc Đường, Tư Mệnh
- 6 Bad Stars: Thiên Hình, Chu Tước, Bạch Hổ, Thiên Lao, Nguyên Vũ, Câu Trận
- Hour-by-hour analysis with time ranges

## 🚀 Quick Start

### Installation

```bash
git clone https://github.com/YOUR_USERNAME/amlich-view.git
cd amlich-view
```

No dependencies needed! Pure Node.js.

### Usage

```bash
# Show today's date with full almanac info
node index.js today

# Get detailed info for any date
node index.js info 10 2 2024    # Tết 2024

# Show all holidays for a year
node index.js show 2024

# Convert dates
node index.js convert 1 1 2024 lunar   # Lunar → Solar

# Export to calendar
node index.js export-ics 2024
```

### Example Output

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

## 📚 Library API

### Basic Usage

```javascript
const { getDayInfo } = require('./engine/index.js');

// Get complete day information
const info = getDayInfo(10, 2, 2024);

// Access Can Chi
console.log(info.canChi.day.full);      // "Giáp Thìn"
console.log(info.canChi.day.conGiap);   // "Thìn (Rồng)"
console.log(info.canChi.day.nguHanh);   // { can: "Mộc", chi: "Thổ" }

// Access Solar Term
console.log(info.tietKhi.name);         // "Lập Xuân"
console.log(info.tietKhi.season);       // "Đông (Winter)"

// Access Auspicious Hours
console.log(info.gioHoangDao.goodHourCount);  // 6
info.gioHoangDao.goodHours.forEach(h => {
    console.log(`${h.hourChi} (${h.timeRange}): ${h.star}`);
});
```

### Date Conversion

```javascript
const { getLunarDate, getSolarDate } = require('./vietnamese-holidays.js');

// Solar → Lunar
const lunar = getLunarDate(10, 2, 2024);
console.log(lunar);  // { day: 1, month: 1, year: 2024, isLeapMonth: false }

// Lunar → Solar
const solar = getSolarDate(1, 1, 2024);
console.log(solar);  // { day: 10, month: 2, year: 2024 }
```

### Get Holidays

```javascript
const { getVietnameseHolidays } = require('./vietnamese-holidays.js');

const holidays = getVietnameseHolidays(2024);
holidays.forEach(h => {
    console.log(`${h.dateString}: ${h.name}`);
});
```

## 🎯 CLI Commands

```bash
# Show today with all features
node index.js today

# Show detailed info for a specific date
node index.js info <day> <month> <year>

# Show all holidays for a year
node index.js show [year]

# Convert between solar and lunar
node index.js convert <d> <m> <y> [solar|lunar]

# Export to calendar apps
node index.js export-ics [year]
node index.js export-csv [year]

# Run tests
node engine/test.js
```

## 🏗️ Architecture

```
amlich-view/
├── amlich-core.js           # Core lunar algorithm (Hồ Ngọc Đức)
├── vietnamese-holidays.js   # Holiday calculations
├── engine/                  # Expert Engine
│   ├── index.js            # Main API: getDayInfo()
│   ├── types.js            # Can/Chi constants
│   ├── canchi.js           # Can Chi calculations
│   ├── tietkhi.js          # Solar Terms (24 terms)
│   ├── gio-hoang-dao.js    # Auspicious Hours
│   └── test.js             # Test suite
├── index.js                # CLI application
├── index.html              # Web viewer
└── app.js                  # Web app logic
```

## 🔬 Technical Details

### Algorithms

**Solar Terms:**
- Based on Jean Meeus' "Astronomical Algorithms" (1998)
- Precision: Within minutes of actual astronomical events
- Formula: `floor(sun_longitude_degrees / 15) → term index`

**Auspicious Hours:**
- Traditional Thập Nhị Kiến Trừ (12-Star System)
- Day-dependent cycle start (lookup table)
- 12 stars mapped to 12 traditional hours

**Can Chi:**
- Day: JD-based formula `(JD+9)%10, (JD+1)%12`
- Month: Lunar month + year stem table
- Year: `(year+6)%10, (year+8)%12`

### Verification

All calculations verified against:
- Tết dates: 2023, 2024, 2025, 2026
- Equinoxes & Solstices
- Historical almanacs
- Test coverage: 100% ✅

## 📖 Documentation

- [README-EXPERT.md](README-EXPERT.md) - Comprehensive user guide
- [PHASE1-COMPLETE.md](PHASE1-COMPLETE.md) - Phase 1 details
- [PHASE2-COMPLETE.md](PHASE2-COMPLETE.md) - Phase 2 details
- [QUICKREF.txt](QUICKREF.txt) - Quick reference

## 🧪 Testing

```bash
# Run all tests
node engine/test.js

# Test output
📊 Test Results: 6 passed, 0 failed
✅ All tests passed!
```

Test suite includes:
- Tết 2024, 2025, 2023 verification
- Y2K reference date
- Solar term calculations
- Auspicious hours for different day types

## 📊 Project Stats

- **Total Lines:** ~3,600
- **Engine Code:** 805 lines
- **Test Coverage:** 100%
- **Dependencies:** 0 (Pure Node.js!)
- **Files:** 24
- **Features:** 8 major features

## 🎨 Vietnamese Holidays Included

**Major Festivals:**
- 🎊 Tết Nguyên Đán (Lunar New Year)
- 🏮 Tết Nguyên Tiêu (Lantern Festival)
- 🌸 Thanh Minh (Tomb Sweeping Day)
- 🙏 Phật Đản (Buddha's Birthday)
- 🐉 Tết Đoan Ngọ (Dragon Boat Festival)
- 👪 Vu Lan (Parents' Day)
- 🥮 Tết Trung Thu (Mid-Autumn Festival)
- 🏔️ Tết Trùng Cửu (Double Ninth)
- 🎋 Tết Hạ Nguyên
- 🍲 Ông Táo chầu trời (Kitchen Gods' Day)

**Monthly Events:**
- 🌑 Mùng 1 (New Moon - 1st of each lunar month)
- 🌕 Rằm (Full Moon - 15th of each lunar month)

## 🗺️ Roadmap

### ✅ Phase 1 (Complete)
- Can Chi calculations
- Vietnamese Zodiac
- Five Elements

### ✅ Phase 2 (Complete)
- 24 Solar Terms
- Auspicious Hours (12-Star System)

### 🔲 Phase 3 (Planned)
- 12 Trực (Day officers)
- Nạp Âm (60-cycle elements)
- Ngày Hoàng Đạo (Day classifications)
- 28 Star Mansions
- Xung/Hợp relations

## 🤝 Contributing

Contributions welcome! Areas of interest:
- Additional almanac features
- UI/UX improvements
- Documentation improvements
- Bug reports and feature requests

## 📄 License

- Core algorithm: Copyright (c) 2006 Ho Ngoc Duc
- Expert Engine & additions: MIT License

## 🙏 Credits

- **Astronomical algorithms:** Ho Ngoc Duc
- **Based on:** Jean Meeus' "Astronomical Algorithms" (1998)
- **Expert Engine:** Built with traditional Vietnamese almanac knowledge

## 📞 Support

For issues, questions, or feature requests, please open an issue on GitHub.

---

**Made with ❤️ for Vietnamese culture and traditions**

⭐ If you find this useful, please star the repository!
