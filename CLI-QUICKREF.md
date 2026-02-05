# Amlich CLI - Quick Reference

## Installation

```bash
# Install from source
cargo install --path crates/amlich-cli

# Or build and copy
cargo build --release --package amlich-cli
sudo cp target/release/amlich /usr/local/bin/
```

## Commands

| Command | Description | Example |
|---------|-------------|---------|
| `amlich` | Show today (default) | `amlich` |
| `amlich today` | Show today's info | `amlich today` |
| `amlich date <DATE>` | Show specific date | `amlich date 2024-02-10` |
| `amlich toggle` | Cycle display modes | `amlich toggle` |
| `amlich json [DATE]` | JSON output | `amlich json` |
| `amlich mode` | Show current mode | `amlich mode` |
| `amlich set-mode <MODE>` | Set mode directly | `amlich set-mode lunar` |
| `amlich --help` | Show help | `amlich --help` |
| `amlich --version` | Show version | `amlich --version` |

## Display Modes

| Mode | Output | Example |
|------|--------|---------|
| `full` | Complete info | `📅 5/2/2026 🌙 18/12/2025 (Ất Tỵ) 📜 Canh Tuất` |
| `lunar` | Lunar date | `🌙 18/12/2025` |
| `canchi` | Can Chi | `📜 Canh Tuất` |
| `minimal` | Short format | `18/12` |

**Toggle cycle**: full → lunar → canchi → minimal → full

## Date Format

- **Input**: `YYYY-MM-DD` (ISO 8601)
- **Examples**: `2024-02-10`, `2025-01-29`, `2023-01-22`

## Output Formats

### Waybar JSON
```json
{
  "text": "📅 5/2/2026 🌙 18/12/2025 (Ất Tỵ) 📜 Canh Tuất",
  "tooltip": "📅 Dương lịch: 2026-02-05 - Thứ Năm\n🌙 Âm lịch: 18/12/2025\n...",
  "class": "full"
}
```

### Structured JSON
```json
{
  "solar": { "day": 5, "month": 2, "year": 2026, ... },
  "lunar": { "day": 18, "month": 12, "year": 2025, ... },
  "canchi": { "day": "Canh Tuất", ... },
  "tiet_khi": { "name": "Lập Xuân", ... },
  "gio_hoang_dao": { "good_hour_count": 6, ... }
}
```

## Common Use Cases

### Show Today
```bash
amlich
amlich today
```

### Show Tết Dates
```bash
amlich date 2024-02-10  # Tết 2024
amlich date 2025-01-29  # Tết 2025
amlich date 2023-01-22  # Tết 2023
```

### Cycle Display Modes
```bash
amlich mode              # Check current mode
amlich toggle           # Switch to next mode
amlich set-mode lunar   # Set to lunar mode
```

### Scripting with JSON
```bash
# Extract lunar date
amlich json | jq -r '.lunar.date_string'

# Extract Can Chi
amlich json | jq -r '.canchi.day'

# Get solar term
amlich json | jq -r '.tiet_khi.name'

# Count good hours
amlich json | jq '.gio_hoang_dao.good_hour_count'

# Check if leap month
amlich json 2023-03-22 | jq '.lunar.is_leap_month'
```

### Integration Examples
```bash
# Check if today is Tết
if [ "$(amlich json | jq -r '.lunar | "\(.day)/\(.month)"')" = "1/1" ]; then
    notify-send "Chúc Mừng Năm Mới!"
fi

# Display in terminal
watch -n 60 'amlich today | jq -r .text'

# Log daily information
amlich json >> ~/lunar-calendar.log
```

## Waybar Configuration

### Config (~/.config/waybar/config)
```json
"custom/amlich": {
    "exec": "amlich today",
    "interval": 60,
    "return-type": "json",
    "on-click": "amlich toggle",
    "tooltip": true
}
```

### Style (~/.config/waybar/style.css)
```css
#custom-amlich {
    padding: 0 10px;
}

#custom-amlich.full {
    color: #a6e3a1;  /* Green */
}

#custom-amlich.lunar {
    color: #89b4fa;  /* Blue */
}

#custom-amlich.canchi {
    color: #f9e2af;  /* Yellow */
}

#custom-amlich.minimal {
    color: #cba6f7;  /* Purple */
}
```

## State File

- **Location**: `~/.local/state/amlich/mode`
- **Content**: Current mode (`full`, `lunar`, `canchi`, `minimal`)
- **Persistence**: Survives system restarts

## Error Handling

### Invalid Date Format
```bash
$ amlich date 2024/02/10
Error: Date must be in YYYY-MM-DD format
```

### Invalid Month/Day
```bash
$ amlich date 2024-13-01
Error: Month must be between 1 and 12
```

### Invalid Mode
```bash
$ amlich set-mode invalid
Error: Invalid mode: invalid
Valid modes: full, lunar, canchi, minimal
```

## JSON Fields Reference

### Solar
- `day`, `month`, `year` - Solar date
- `day_of_week` - Day name (Vietnamese)
- `date_string` - ISO format

### Lunar
- `day`, `month`, `year` - Lunar date
- `is_leap_month` - Boolean
- `date_string` - Vietnamese format

### Can Chi
- `day`, `month`, `year` - Full Can Chi
- `day_can`, `day_chi` - Separate components
- `month_can`, `month_chi`
- `year_can`, `year_chi`

### Tiết Khí (Solar Term)
- `name` - Vietnamese name
- `description` - English description
- `season` - Season name

### Giờ Hoàng Đạo (Auspicious Hours)
- `good_hour_count` - Number of good hours
- `hours[]` - Array of 12 hours
  - `hour` - Index (0-11)
  - `name` - Chi name
  - `time_range` - e.g., "09:00-11:00"
  - `star` - Star name
  - `is_good` - Boolean

## Tips

1. **Waybar Updates**: Set `interval: 60` for once-per-minute updates
2. **Click Events**: Use `on-click: "amlich toggle"` to cycle modes
3. **Scripting**: Pipe to `jq` for JSON processing
4. **Aliases**: Add to `.bashrc`:
   ```bash
   alias tet='amlich json | jq -r ".lunar.date_string"'
   alias canchi='amlich json | jq -r ".canchi.day"'
   ```

## Performance

- **Execution**: <10ms per command
- **Memory**: <2MB
- **Binary**: 860KB (release)
- **CPU**: Minimal (<1% on updates)

## Support

- **Documentation**: `amlich --help`
- **Issues**: https://github.com/mnoyd/amlich/issues
- **Source**: https://github.com/mnoyd/amlich

---

**Version**: 1.0.0  
**License**: MIT  
**Made with ❤️ for Vietnamese culture**
