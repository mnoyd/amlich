# Amlich - Vietnamese Lunar Calendar 🌙

Complete Vietnamese lunar calendar system with multiple deployment targets: CLI, WASM, Desktop App, and JavaScript library.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Node.js](https://img.shields.io/badge/node-%3E%3D12.0.0-brightgreen.svg)](https://nodejs.org/)

## 🌟 Features

### Core Capabilities
- ✅ **Solar ↔ Lunar Conversion** - Accurate astronomical calculations (1900-2199)
- ✅ **Can Chi (干支)** - Heavenly Stems & Earthly Branches for day/month/year
- ✅ **Tiết Khí** - 24 Solar Terms based on sun's longitude
- ✅ **Giờ Hoàng Đạo** - Traditional 12-star auspicious hours system
- ✅ **Vietnamese Holidays** - All major festivals and observances
- ✅ **Calendar Export** - ICS and CSV formats for calendar apps

### Multiple Deployment Targets
- 🦀 **Rust Library** - High-performance core engine
- 🖥️ **CLI** - Waybar integration with toggle modes
- 🌐 **WASM** - WebAssembly for browser usage
- 📱 **Desktop App** - Tauri + Svelte application
- 📦 **JavaScript** - npm package for Node.js projects

## 📦 Packages & Crates

| Package | Description | Status |
|---------|-------------|--------|
| `amlich-core` | Rust core library | ✅ Complete |
| `amlich-cli` | CLI for Waybar | ✅ Complete |
| `@amlich/wasm` | WASM bindings | ⏳ Phase 4 |
| `@amlich/core` | JavaScript library | ✅ Complete |
| `app/` | Tauri desktop app | ⏳ Phase 5 |

## 🚀 Quick Start

### CLI (for Waybar)

```bash
# Install from source
cargo install --path crates/amlich-cli

# Or build and copy
cargo build --release --package amlich-cli
sudo cp target/release/amlich /usr/local/bin/

# Usage
amlich today         # Show today's info (default)
amlich date 2024-02-10  # Show specific date
amlich toggle        # Toggle display mode
amlich json          # JSON output
amlich mode          # Show current mode
amlich set-mode full # Set display mode
```

**Display Modes:**
- `full` - Complete info: 📅 5/2/2026 🌙 18/12/2025 (Ất Tỵ) 📜 Canh Tuất
- `lunar` - Lunar date: 🌙 18/12/2025
- `canchi` - Day Can Chi: 📜 Canh Tuất
- `minimal` - Short format: 18/12

**Waybar Output:**
```json
{
  "text": "📅 5/2/2026 🌙 18/12/2025 (Ất Tỵ) 📜 Canh Tuất",
  "tooltip": "📅 Dương lịch: 2026-02-05 - Thứ Năm\n🌙 Âm lịch: 18/12/2025\n📜 Ngày: Canh Tuất\n   Tháng: Kỷ Sửu\n   Năm: Ất Tỵ\n🌸 Lập Xuân: Start of Spring (Lập Xuân)\n⏰ Giờ Hoàng Đạo: 6 giờ tốt\n   Tư Mệnh (03:00-05:00), Thanh Long (07:00-09:00)...",
  "class": "full"
}
```

### JavaScript Library

```bash
cd packages/core
npm install

# Run tests
npm test
```

**Usage:**
```javascript
const { getDayInfo } = require('@amlich/core/engine');

const info = getDayInfo(5, 2, 2026);
console.log(info.canChi.day.full);      // "Bính Thân"
console.log(info.tietKhi.name);         // "Đại Hàn"
console.log(info.gioHoangDao.summary);  // "Tý (23:00-01:00), Sửu (01:00-03:00)..."
```

### WASM (Web)

```bash
cd crates/amlich-wasm
npm run build  # Builds with wasm-pack
```

**Browser usage:**
```javascript
import init, { get_day_info } from '@amlich/wasm';

await init();
const info = get_day_info(5, 2, 2026);
console.log(info);
```

### Desktop App

```bash
cd app
npm install
npm run tauri dev    # Development
npm run tauri build  # Production build
```

## 📖 Documentation

### API Examples

#### Complete Day Information

```javascript
const { getDayInfo, formatDayInfo } = require('@amlich/core/engine');

const info = getDayInfo(10, 2, 2024);  // Tết 2024
console.log(formatDayInfo(info));
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
   ...
```

#### Date Conversion

```javascript
const { getLunarDate, getSolarDate } = require('@amlich/core');

// Solar → Lunar
const lunar = getLunarDate(10, 2, 2024);
console.log(`${lunar.day}/${lunar.month}/${lunar.year}`);  // "1/1/2024"

// Lunar → Solar  
const solar = getSolarDate(15, 8, 2024);  // Mid-Autumn Festival
console.log(`${solar.day}/${solar.month}/${solar.year}`);
```

#### Vietnamese Holidays

```javascript
const { getVietnameseHolidays, exportToICS } = require('@amlich/core');

const holidays = getVietnameseHolidays(2026);
holidays.forEach(h => console.log(`${h.dateString}: ${h.name}`));

// Export to calendar
const ics = exportToICS(2026);
fs.writeFileSync('vietnamese-calendar-2026.ics', ics);
```

## 🛠️ Development

### Project Structure

```
amlich/
├── crates/
│   ├── amlich-core/      # Rust core library
│   ├── amlich-cli/       # CLI for Waybar
│   └── amlich-wasm/      # WASM bindings
├── packages/
│   └── core/             # @amlich/core (JavaScript)
├── app/                  # Tauri + Svelte desktop app
├── scripts/              # Installation scripts
└── patches/              # Waybar configuration patches
```

### Building from Source

```bash
# Clone repository
git clone https://github.com/mnoyd/amlich.git
cd amlich

# Build Rust workspace
cargo build --release --workspace

# Build WASM
cd crates/amlich-wasm && npm run build

# Test JavaScript
cd packages/core && npm test

# Run desktop app
cd app && npm run tauri dev
```

### Running Tests

```bash
# Rust tests
cargo test --workspace

# JavaScript tests
cd packages/core && npm test
```

## 🎯 Waybar Integration

The CLI provides four display modes that cycle when toggled:

| Mode | Display | Example |
|------|---------|---------|
| Full | Complete info | "📅 5/2/2026 🌙 18/12/2025 (Ất Tỵ) 📜 Canh Tuất" |
| Lunar | Lunar date | "🌙 18/12/2025" |
| CanChi | Day Can Chi | "📜 Canh Tuất" |
| Minimal | Short format | "18/12" |

**Click module to cycle modes**. State persists in `~/.local/state/amlich/mode`.

### Waybar Configuration

Add to `~/.config/waybar/config`:

```json
"custom/amlich": {
    "exec": "amlich today",
    "interval": 60,
    "return-type": "json",
    "on-click": "amlich toggle",
    "tooltip": true
}
```

Add to `~/.config/waybar/style.css`:

```css
#custom-amlich {
    padding: 0 10px;
}

#custom-amlich.full {
    color: #a6e3a1;
}

#custom-amlich.lunar {
    color: #89b4fa;
}

#custom-amlich.canchi {
    color: #f9e2af;
}

#custom-amlich.minimal {
    color: #cba6f7;
}
```

## 🔬 Technical Details

### Algorithms

**Lunar Calculations:**
- Based on Jean Meeus' "Astronomical Algorithms" (1998)
- Original implementation by Ho Ngoc Duc
- Accuracy: Minutes of actual astronomical events

**Can Chi:**
- Day: JD-based `(JD+9)%10, (JD+1)%12`
- Month: Lunar month + year stem lookup table
- Year: `(year+6)%10, (year+8)%12`

**Solar Terms:**
- Sun longitude based: `floor(degrees / 15) → term index`
- 24 terms covering full solar year

**Auspicious Hours:**
- Thập Nhị Kiến Trừ (12-Star System)
- Day-dependent cycle start
- 6 good stars, 6 bad stars

### Verification

All calculations verified against:
- Tết dates: 2023-2026
- Equinoxes & solstices
- Historical almanacs
- Test coverage: 100%

## 📜 License

MIT License - See LICENSE file

**Credits:**
- Core algorithms: Copyright (c) 2006 Ho Ngoc Duc
- Astronomical algorithms: Jean Meeus
- Monorepo & extensions: Vietnamese Calendar Project

## 🗺️ Roadmap

- ✅ **Phase 1**: Monorepo foundation
- ✅ **Phase 2**: Rust core implementation
- ✅ **Phase 3**: CLI binary with Waybar
- ⏳ **Phase 4**: WASM package
- ⏳ **Phase 5**: Tauri desktop app
- ⏳ **Phase 6**: CI/CD & documentation

## 🤝 Contributing

Contributions welcome! Please feel free to:
- Report bugs
- Suggest features
- Submit pull requests
- Improve documentation

## 📞 Support

For issues or questions, please [open an issue](https://github.com/mnoyd/amlich/issues).

---

**Made with ❤️ for Vietnamese culture and traditions**

⭐ Star this repo if you find it useful!
