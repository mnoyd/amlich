#!/bin/bash
# Vietnamese Lunar Calendar Expert Engine - Demo Script

echo "🌙 Vietnamese Lunar Calendar Expert Engine Demo 🌙"
echo "=================================================="
echo ""

echo "📋 1. Testing Can Chi Calculations"
echo "-----------------------------------"
node engine/test.js
echo ""

echo "📅 2. Today's Date with Full Can Chi Info"
echo "-----------------------------------------"
node index.js today
echo ""

echo "🎊 3. Tết 2024 (Lunar New Year) - Detailed Info"
echo "-----------------------------------------------"
node index.js info 10 2 2024
echo ""

echo "🎊 4. Tết 2025 - Detailed Info"
echo "------------------------------"
node index.js info 29 1 2025
echo ""

echo "🔄 5. Date Conversion Examples"
echo "------------------------------"
echo "Lunar 1/1/2024 → Solar:"
node index.js convert 1 1 2024 lunar
echo ""
echo "Solar 10/2/2024 → Lunar:"
node index.js convert 10 2 2024 solar
echo ""

echo "✅ Demo Complete!"
echo ""
echo "Try more commands:"
echo "  node index.js show 2024          # Show all holidays"
echo "  node index.js info <d> <m> <y>   # Get Can Chi for any date"
echo "  node index.js export-ics 2024    # Export to calendar"
