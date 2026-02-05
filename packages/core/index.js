const fs = require('fs');
const { getVietnameseHolidays, getLunarDate, getSolarDate, exportToGoogleCalendarCSV, exportToICS } = require('./vietnamese-holidays.js');
const { getDayInfo, formatDayInfo } = require('./engine/index.js');

// Get command line arguments
const args = process.argv.slice(2);
const command = args[0] || 'show';
const year = parseInt(args[1]) || new Date().getFullYear();

console.log('\n🌙 Vietnamese Lunar Calendar (Âm Lịch Việt Nam) 🌙\n');

switch (command) {
    case 'show':
    case 'list':
        // Show all holidays for the year
        console.log(`📅 Vietnamese Holidays for ${year}:\n`);
        const holidays = getVietnameseHolidays(year);
        
        // Group by type
        const major = holidays.filter(h => !h.name.includes('Mùng 1 tháng') && !h.name.includes('Rằm tháng'));
        const monthly = holidays.filter(h => h.name.includes('Mùng 1 tháng') || h.name.includes('Rằm tháng'));
        
        console.log('🎉 Major Holidays & Festivals:');
        console.log('─'.repeat(80));
        major.forEach(h => {
            const lunarStr = h.lunarDate ? `(${h.lunarDate.day}/${h.lunarDate.month} AL)` : '(Solar)';
            console.log(`${h.dateString.padEnd(12)} ${lunarStr.padEnd(15)} ${h.name}`);
            if (h.description) {
                console.log(`${' '.repeat(29)}→ ${h.description}`);
            }
        });
        
        console.log('\n🌕 Monthly Full Moons (Rằm) & New Moons (Mùng 1):');
        console.log('─'.repeat(80));
        monthly.forEach(h => {
            console.log(`${h.dateString.padEnd(12)} ${h.name}`);
        });
        break;

    case 'export-csv':
        // Export to Google Calendar CSV
        const csv = exportToGoogleCalendarCSV(year);
        const csvFile = `vietnamese-calendar-${year}.csv`;
        fs.writeFileSync(csvFile, csv);
        console.log(`✅ Exported to: ${csvFile}`);
        console.log('📝 Import to Google Calendar:');
        console.log('   1. Go to Google Calendar (calendar.google.com)');
        console.log('   2. Settings → Import & Export → Import');
        console.log(`   3. Select file: ${csvFile}`);
        break;

    case 'export-ics':
        // Export to iCal format
        const ics = exportToICS(year);
        const icsFile = `vietnamese-calendar-${year}.ics`;
        fs.writeFileSync(icsFile, ics);
        console.log(`✅ Exported to: ${icsFile}`);
        console.log('📝 Import to Calendar:');
        console.log('   • Google Calendar: Settings → Import & Export → Import');
        console.log('   • Apple Calendar: File → Import');
        console.log('   • Outlook: File → Import');
        break;

    case 'today':
        // Show today's lunar date with full Can Chi info
        const today = new Date();
        const todayInfo = getDayInfo(today.getDate(), today.getMonth() + 1, today.getFullYear());
        
        console.log(formatDayInfo(todayInfo));
        
        // Check if today is a special day
        const todayHolidays = getVietnameseHolidays(today.getFullYear()).filter(h => {
            return h.dateString === today.toISOString().split('T')[0];
        });
        
        if (todayHolidays.length > 0) {
            console.log('\n🎊 Special days today:');
            todayHolidays.forEach(h => {
                console.log(`   • ${h.name}`);
                if (h.description) console.log(`     ${h.description}`);
            });
        }
        break;

    case 'info':
    case 'canchi':
        // Show detailed Can Chi info for a specific date
        if (args.length < 4) {
            console.log('Usage: node index.js info <day> <month> <year>');
            console.log('Example: node index.js info 10 2 2024');
            break;
        }
        
        const infoDay = parseInt(args[1]);
        const infoMonth = parseInt(args[2]);
        const infoYear = parseInt(args[3]);
        
        const dayInfo = getDayInfo(infoDay, infoMonth, infoYear);
        console.log(formatDayInfo(dayInfo));
        console.log('\n💡 Calculation methods:');
        console.log(`   • ${dayInfo._meta.methods.dayCanChi}`);
        console.log(`   • ${dayInfo._meta.methods.monthCanChi}`);
        console.log(`   • ${dayInfo._meta.methods.yearCanChi}`);
        break;

    
    case 'convert':
        // Convert a specific date
        if (args.length < 4) {
            console.log('Usage: node index.js convert <day> <month> <year> [solar|lunar]');
            console.log('Example: node index.js convert 1 1 2024 lunar');
            console.log('Example: node index.js convert 10 2 2024 solar');
            break;
        }
        
        const day = parseInt(args[1]);
        const month = parseInt(args[2]);
        const convertYear = parseInt(args[3]);
        const type = args[4] || 'solar';
        
        if (type === 'solar') {
            const lunarResult = getLunarDate(day, month, convertYear);
            console.log(`Solar: ${day}/${month}/${convertYear}`);
            console.log(`Lunar: ${lunarResult.day}/${lunarResult.month}/${lunarResult.year}${lunarResult.isLeapMonth ? ' (tháng nhuận)' : ''}`);
        } else {
            const solarResult = getSolarDate(day, month, convertYear);
            console.log(`Lunar: ${day}/${month}/${convertYear}`);
            console.log(`Solar: ${solarResult.day}/${solarResult.month}/${solarResult.year}`);
        }
        break;

    case 'help':
    default:
        console.log('Usage: node index.js <command> [year]\n');
        console.log('Commands:');
        console.log('  show [year]              Show all holidays for the year (default)');
        console.log('  today                    Show today\'s lunar date, Can Chi, and special events');
        console.log('  info <d> <m> <y>         Show detailed Can Chi info for a specific date');
        console.log('  export-csv [year]        Export to Google Calendar CSV format');
        console.log('  export-ics [year]        Export to iCal (.ics) format');
        console.log('  convert <d> <m> <y> [type]  Convert between solar and lunar dates');
        console.log('                           type: solar (default) or lunar');
        console.log('\nExamples:');
        console.log('  node index.js show 2024');
        console.log('  node index.js today');
        console.log('  node index.js info 10 2 2024');
        console.log('  node index.js export-ics 2024');
        console.log('  node index.js convert 1 1 2024 lunar');
        break;
}

console.log('');
