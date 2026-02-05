# Vietnamese Lunar Calendar (Âm Lịch Việt Nam)

A complete Vietnamese lunar calendar implementation with support for:
- ✅ Converting between Solar (Dương lịch) and Lunar (Âm lịch) dates
- ✅ All major Vietnamese holidays and festivals
- ✅ Monthly full moons (Rằm) and new moons (Mùng 1)
- ✅ Export to Google Calendar (CSV and iCal formats)
- ✅ Desktop CLI app (no browser needed!)

## Features

### Vietnamese Holidays Included:

**Major Festivals:**
- 🎊 Tết Nguyên Đán (Lunar New Year)
- 🏮 Tết Nguyên Tiêu (Lantern Festival - Rằm tháng Giêng)
- 🌸 Thanh Minh (Tomb Sweeping Day)
- 🙏 Phật Đản (Buddha's Birthday - Rằm tháng Tư)
- 🐉 Tết Đoan Ngọ (Dragon Boat Festival)
- 👪 Vu Lan (Parents' Day - Rằm tháng Bảy)
- 🥮 Tết Trung Thu (Mid-Autumn Festival - Rằm tháng Tám)
- 🏔️ Tết Trùng Cửu (Double Ninth)
- 🎋 Tết Hạ Nguyên (Rằm tháng Mười)
- 🍲 Ông Táo chầu trời (Kitchen Gods' Day)

**Monthly Events:**
- 🌑 Mùng 1 (New Moon - 1st day of each lunar month)
- 🌕 Rằm (Full Moon - 15th day of each lunar month)

## Installation

```bash
# No dependencies needed! Pure Node.js
cd amlich-view
```

## CLI Usage

```bash
# Show today's date with full Can Chi info
node index.js today

# Show detailed Can Chi for any date
node index.js info 10 2 2024    # Tết 2024

# Show all holidays for a year
node index.js show 2024

# Convert dates
node index.js convert 1 1 2024 lunar   # Lunar → Solar
node index.js convert 10 2 2024 solar  # Solar → Lunar

# Export to calendar apps
node index.js export-ics 2024
node index.js export-csv 2024
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
```

### 2. Check today's lunar date
```bash
node index.js today
```

### 3. Export to Google Calendar (CSV format)
```bash
node index.js export-csv 2024
# Creates: vietnamese-calendar-2024.csv
```

### 4. Export to iCal format (works with Google/Apple/Outlook)
```bash
node index.js export-ics 2024
# Creates: vietnamese-calendar-2024.ics
```

### 5. Convert dates
```bash
# Solar to Lunar
node index.js convert 10 2 2024 solar

# Lunar to Solar
node index.js convert 1 1 2024 lunar
```

## Import to Google Calendar

### Method 1: CSV Import
1. Run: `node index.js export-csv 2024`
2. Go to [Google Calendar](https://calendar.google.com)
3. Click Settings (⚙️) → Import & Export
4. Click "Select file from your computer"
5. Choose `vietnamese-calendar-2024.csv`
6. Select which calendar to add events to
7. Click Import

### Method 2: iCal Import (Recommended)
1. Run: `node index.js export-ics 2024`
2. Go to [Google Calendar](https://calendar.google.com)
3. Click Settings (⚙️) → Import & Export
4. Click "Select file from your computer"
5. Choose `vietnamese-calendar-2024.ics`
6. Select calendar and Import

## Examples

```bash
# Show all holidays for 2024
node index.js show 2024

# Check if today is a special day
node index.js today

# Find when Tết 2025 is
node index.js convert 1 1 2025 lunar

# Export next 3 years to calendar
node index.js export-ics 2024
node index.js export-ics 2025
node index.js export-ics 2026
```

## Test

```bash
node test.js
```

## Algorithm

This implementation uses **Hồ Ngọc Đức's astronomical algorithm** based on:
- "Astronomical Algorithms" by Jean Meeus (1998)
- Julian Day Number calculations
- New Moon and Sun Longitude calculations
- Accurate for years 1900-2199

## Files

- `amlich-core.js` - Core lunar calendar algorithm
- `vietnamese-holidays.js` - Vietnamese holiday definitions
- `index.js` - CLI application
- `test.js` - Test suite

## Time Zone

All calculations use **Vietnam timezone (UTC+7)**.

## License

- Core algorithm: Copyright (c) 2006 Ho Ngoc Duc
- Holiday definitions: MIT License

## Credits

- Astronomical algorithms: Ho Ngoc Duc (www.informatik.uni-leipzig.de/~duc/amlich/)
- Based on: Jean Meeus' "Astronomical Algorithms" (1998)
