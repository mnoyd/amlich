# 🌙 Vietnamese Lunar Calendar - Quick Start Guide

## What You Got

A complete Vietnamese lunar calendar system with:

✅ **Core Algorithm** - Hồ Ngọc Đức's astronomical calculations  
✅ **All Vietnamese Holidays** - Tết, Rằm, Mùng 1, and traditional festivals  
✅ **Google Calendar Export** - CSV and iCal formats  
✅ **Desktop App** - No browser needed  
✅ **Web Viewer** - Beautiful HTML interface  

---

## 📁 Files Overview

```
amlich-view/
├── amlich-core.js              # Core lunar calendar algorithm
├── vietnamese-holidays.js      # Holiday definitions (Node.js)
├── vietnamese-holidays-browser.js  # Holiday definitions (Browser)
├── index.js                    # CLI app
├── index.html                  # Web viewer
├── app.js                      # Web app logic
├── test.js                     # Test suite
└── README.md                   # Full documentation
```

---

## 🚀 Quick Start

### Option 1: Command Line (No Browser)

```bash
# Show today's lunar date
node index.js today

# Show all holidays for 2024
node index.js show 2024

# Export to Google Calendar
node index.js export-ics 2024

# Convert dates
node index.js convert 10 2 2024 solar
```

### Option 2: Desktop App (HTML Viewer)

```bash
# Just open in your browser:
open index.html
# or
firefox index.html
# or
chrome index.html
```

**Works offline!** No internet needed after opening.

---

## 📅 Add to Google Calendar

### Method 1: iCal File (Recommended)

```bash
node index.js export-ics 2024
```

Then:
1. Go to [Google Calendar](https://calendar.google.com)
2. Click ⚙️ Settings → **Import & Export**
3. Click **Select file from your computer**
4. Choose `vietnamese-calendar-2024.ics`
5. Click **Import**

### Method 2: CSV File

```bash
node index.js export-csv 2024
```

Same import steps as above, but choose the `.csv` file.

---

## 🎯 Vietnamese Holidays Included

### Major Festivals:
- 🎊 **Tết Nguyên Đán** - Lunar New Year (1/1 AL)
- 🏮 **Tết Nguyên Tiêu** - Lantern Festival (15/1 AL)
- 🌸 **Thanh Minh** - Tomb Sweeping Day (~5/4 Solar)
- 🙏 **Phật Đản** - Buddha's Birthday (15/4 AL)
- 🐉 **Tết Đoan Ngọ** - Dragon Boat Festival (5/5 AL)
- 👪 **Vu Lan** - Parents' Day (15/7 AL)
- 🥮 **Tết Trung Thu** - Mid-Autumn Festival (15/8 AL)
- 🏔️ **Tết Trùng Cửu** - Double Ninth (9/9 AL)
- 🎋 **Tết Hạ Nguyên** - Lower Nguyên (15/10 AL)
- 🍲 **Ông Táo** - Kitchen Gods' Day (23/12 AL)

### Monthly Events:
- 🌑 **Mùng 1** - New Moon (1st of each lunar month)
- 🌕 **Rằm** - Full Moon (15th of each lunar month)

---

## 💡 Usage Examples

### CLI Examples

```bash
# Check if today is Rằm or special day
node index.js today

# Get all holidays for multiple years
node index.js export-ics 2024
node index.js export-ics 2025
node index.js export-ics 2026

# Find when Tết 2025 is
node index.js convert 1 1 2025 lunar
# Output: 1/1/2025 Lunar = 29/1/2025 Solar

# Convert today to lunar
node index.js convert 5 2 2026 solar
# Output: 5/2/2026 Solar = 18/12/2025 Lunar
```

### Using in Your Code

```javascript
const { getLunarDate, getVietnameseHolidays } = require('./vietnamese-holidays.js');

// Get lunar date for any day
const lunar = getLunarDate(10, 2, 2024);
console.log(`${lunar.day}/${lunar.month}/${lunar.year}`);
// Output: 1/1/2024 (Tết!)

// Get all holidays
const holidays = getVietnameseHolidays(2024);
holidays.forEach(h => {
    console.log(`${h.name} - ${h.dateString}`);
});
```

---

## 🖥️ Desktop App Features

Open `index.html` in your browser to get:

1. **Today's Date** - See current solar & lunar dates
2. **Holiday Calendar** - Beautiful cards for all holidays
3. **Date Converter** - Convert any date between solar/lunar
4. **Export** - Download .ics or .csv files directly

**Screenshot Preview:**
```
┌─────────────────────────────────────┐
│   🌙 Âm Lịch Việt Nam               │
│   Vietnamese Lunar Calendar         │
├─────────────────────────────────────┤
│   Hôm Nay - Today                   │
│   ┌──────────┐  ┌──────────┐       │
│   │ 5/2/2026 │  │18/12/2025│       │
│   │  Solar   │  │  Lunar   │       │
│   └──────────┘  └──────────┘       │
├─────────────────────────────────────┤
│ [Ngày Lễ] [Chuyển Đổi] [Xuất File]│
└─────────────────────────────────────┘
```

---

## ⚙️ Technical Details

- **Algorithm**: Ho Ngoc Duc's astronomical calculations
- **Based on**: Jean Meeus' "Astronomical Algorithms" (1998)
- **Accuracy**: Years 1900-2199
- **Timezone**: Vietnam (UTC+7)
- **Dependencies**: None! Pure JavaScript

---

## 📱 Works Everywhere

✅ Linux  
✅ macOS  
✅ Windows  
✅ Any browser (Chrome, Firefox, Safari, Edge)  
✅ Offline (after first load)  

---

## 🎁 What You Can Do

1. **Personal Use**: Always know lunar dates
2. **Google Calendar**: Never miss Rằm or Mùng 1
3. **Family**: Share holiday calendar with relatives
4. **Reminders**: Set up alerts for traditional events
5. **Planning**: Know lunar dates for weddings, ceremonies
6. **Custom**: Modify code to add your own events

---

## 📝 Quick Reference

| Command | What It Does |
|---------|-------------|
| `node index.js today` | Show today's lunar date |
| `node index.js show 2024` | List all holidays |
| `node index.js export-ics 2024` | Export to calendar |
| `open index.html` | Open desktop app |

---

## 🆘 Need Help?

1. **Test the system**: `node test.js`
2. **See all commands**: `node index.js help`
3. **Check README**: More detailed docs

---

## 🎉 You're All Set!

Everything is ready to use. Just run:

```bash
node index.js today
```

Or open `index.html` in your browser!

Enjoy your Vietnamese lunar calendar! 🌙
