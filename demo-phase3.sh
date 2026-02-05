#!/bin/bash
# Phase 3 CLI Demonstration
# Shows all features of the amlich CLI tool

set -e

CLI="./target/release/amlich"

# Build if needed
if [ ! -f "$CLI" ]; then
    echo "Building CLI..."
    cargo build --release --package amlich-cli
fi

echo "════════════════════════════════════════════════════════════"
echo "  Vietnamese Lunar Calendar CLI - Phase 3 Demo"
echo "════════════════════════════════════════════════════════════"
echo ""

# 1. Help
echo "📖 1. HELP & VERSION"
echo "────────────────────────────────────────────────────────────"
$CLI --version
echo ""
$CLI --help
echo ""

# 2. Today's information
echo "📅 2. TODAY'S INFORMATION"
echo "────────────────────────────────────────────────────────────"
echo "Command: amlich today"
echo ""
$CLI today | jq .
echo ""

# 3. Specific dates (Tết dates)
echo "🎊 3. TẾT DATES"
echo "────────────────────────────────────────────────────────────"

echo "Tết 2024 (Feb 10, 2024):"
$CLI date 2024-02-10 | jq -r '.text'
echo ""

echo "Tết 2025 (Jan 29, 2025):"
$CLI date 2025-01-29 | jq -r '.text'
echo ""

echo "Tết 2023 (Jan 22, 2023):"
$CLI date 2023-01-22 | jq -r '.text'
echo ""

# 4. Display modes
echo "🎨 4. DISPLAY MODES"
echo "────────────────────────────────────────────────────────────"

echo "Current mode:"
$CLI mode
echo ""

echo "Setting to FULL mode:"
$CLI set-mode full
echo ""

echo "Full mode output:"
$CLI today | jq -r '.text'
echo "Class: $($CLI today | jq -r '.class')"
echo ""

echo "Toggling to LUNAR mode:"
$CLI toggle > /dev/null
echo "Lunar mode output:"
$CLI today | jq -r '.text'
echo "Class: $($CLI today | jq -r '.class')"
echo ""

echo "Toggling to CANCHI mode:"
$CLI toggle > /dev/null
echo "CanChi mode output:"
$CLI today | jq -r '.text'
echo "Class: $($CLI today | jq -r '.class')"
echo ""

echo "Toggling to MINIMAL mode:"
$CLI toggle > /dev/null
echo "Minimal mode output:"
$CLI today | jq -r '.text'
echo "Class: $($CLI today | jq -r '.class')"
echo ""

# Reset to full
$CLI set-mode full > /dev/null

# 5. Tooltip
echo "💬 5. TOOLTIP (Multi-line)"
echo "────────────────────────────────────────────────────────────"
$CLI today | jq -r '.tooltip'
echo ""

# 6. JSON Output
echo "📦 6. JSON OUTPUT (Structured)"
echo "────────────────────────────────────────────────────────────"
echo "Command: amlich json 2024-02-10"
echo ""
$CLI json 2024-02-10 | head -40
echo "... (truncated)"
echo ""

# 7. Scripting Examples
echo "🔧 7. SCRIPTING EXAMPLES"
echo "────────────────────────────────────────────────────────────"

echo "Extract lunar date:"
echo "  $ amlich json | jq -r '.lunar.date_string'"
echo "  → $($CLI json | jq -r '.lunar.date_string')"
echo ""

echo "Extract Can Chi:"
echo "  $ amlich json | jq -r '.canchi.day'"
echo "  → $($CLI json | jq -r '.canchi.day')"
echo ""

echo "Extract solar term:"
echo "  $ amlich json | jq -r '.tiet_khi.name'"
echo "  → $($CLI json | jq -r '.tiet_khi.name')"
echo ""

echo "Count good hours:"
echo "  $ amlich json | jq '.gio_hoang_dao.good_hour_count'"
echo "  → $($CLI json | jq '.gio_hoang_dao.good_hour_count')"
echo ""

echo "Check if leap month:"
echo "  $ amlich json 2023-03-22 | jq '.lunar.is_leap_month'"
echo "  → $($CLI json 2023-03-22 | jq '.lunar.is_leap_month')"
echo ""

# 8. State Persistence
echo "💾 8. STATE PERSISTENCE"
echo "────────────────────────────────────────────────────────────"
echo "State file location: ~/.local/state/amlich/mode"
echo "Current state: $(cat ~/.local/state/amlich/mode)"
echo ""

echo "Toggling mode..."
$CLI toggle > /dev/null
echo "New state: $(cat ~/.local/state/amlich/mode)"
echo ""

echo "Toggling again..."
$CLI toggle > /dev/null
echo "New state: $(cat ~/.local/state/amlich/mode)"
echo ""

# Reset to full
$CLI set-mode full > /dev/null
echo "Reset to: $(cat ~/.local/state/amlich/mode)"
echo ""

# 9. Error Handling
echo "⚠️  9. ERROR HANDLING"
echo "────────────────────────────────────────────────────────────"

echo "Invalid date format:"
$CLI date 2024/02/10 2>&1 || echo "(Error caught successfully)"
echo ""

echo "Invalid month:"
$CLI date 2024-13-01 2>&1 || echo "(Error caught successfully)"
echo ""

echo "Invalid mode:"
$CLI set-mode invalid 2>&1 || echo "(Error caught successfully)"
echo ""

# 10. Waybar Integration
echo "🎯 10. WAYBAR INTEGRATION"
echo "────────────────────────────────────────────────────────────"
echo "Add to ~/.config/waybar/config:"
echo ""
cat <<'EOF'
"custom/amlich": {
    "exec": "amlich today",
    "interval": 60,
    "return-type": "json",
    "on-click": "amlich toggle",
    "tooltip": true
}
EOF
echo ""

echo "Add to ~/.config/waybar/style.css:"
echo ""
cat <<'EOF'
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
EOF
echo ""

echo "════════════════════════════════════════════════════════════"
echo "  ✅ Phase 3 Complete - CLI Tool Fully Functional!"
echo "════════════════════════════════════════════════════════════"
echo ""
echo "Installation:"
echo "  cargo install --path crates/amlich-cli"
echo ""
echo "Usage:"
echo "  amlich today         # Show today (default)"
echo "  amlich date <DATE>   # Show specific date"
echo "  amlich toggle        # Toggle display mode"
echo "  amlich json [DATE]   # JSON output"
echo "  amlich mode          # Show current mode"
echo "  amlich set-mode MODE # Set display mode"
echo ""
echo "Next Phase: WASM Package (Phase 4) 🚀"
echo ""
