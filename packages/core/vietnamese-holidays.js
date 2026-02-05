const amlich = require('./amlich-core.js');

/**
 * Vietnamese Lunar Calendar Events
 * Returns all important Vietnamese lunar holidays for a given solar year
 */
function getVietnameseHolidays(solarYear) {
    const timeZone = 7; // Vietnam is UTC+7
    const holidays = [];

    // Function to convert lunar date to solar and add to holidays array
    function addLunarHoliday(name, lunarDay, lunarMonth, lunarYear, description = '') {
        const solar = amlich.convertLunar2Solar(lunarDay, lunarMonth, lunarYear, 0, timeZone);
        if (solar[0] > 0) { // Valid date
            holidays.push({
                name,
                description,
                lunarDate: { day: lunarDay, month: lunarMonth, year: lunarYear },
                solarDate: { day: solar[0], month: solar[1], year: solar[2] },
                dateString: `${solar[2]}-${String(solar[1]).padStart(2, '0')}-${String(solar[0]).padStart(2, '0')}`
            });
        }
    }

    // Tết Nguyên Đán (Lunar New Year) - 1/1 Lunar
    addLunarHoliday('Tết Nguyên Đán (Mùng 1 Tết)', 1, 1, solarYear, 'Vietnamese New Year - First day');
    addLunarHoliday('Mùng 2 Tết', 2, 1, solarYear, 'Second day of Tết');
    addLunarHoliday('Mùng 3 Tết', 3, 1, solarYear, 'Third day of Tết');

    // Tết Nguyên Tiêu - 15/1 Lunar (First full moon of the year)
    addLunarHoliday('Tết Nguyên Tiêu (Rằm tháng Giêng)', 15, 1, solarYear, 'Lantern Festival');

    // Tết Hàn Thực - 3/3 Lunar
    addLunarHoliday('Tết Hàn Thực', 3, 3, solarYear, 'Cold Food Festival');

    // Thanh Minh - Usually around 5/4 solar (Qingming Festival)
    // Note: Thanh Minh is solar-based, typically April 4-5
    // We'll add it as solar date
    const thanhMinh = new Date(solarYear, 3, 5); // April 5th
    holidays.push({
        name: 'Tết Thanh Minh',
        description: 'Tomb Sweeping Day (Solar calendar)',
        solarDate: { day: 5, month: 4, year: solarYear },
        dateString: `${solarYear}-04-05`,
        isSolar: true
    });

    // Phật Đản (Buddha's Birthday) - 15/4 Lunar
    addLunarHoliday('Phật Đản (Rằm tháng Tư)', 15, 4, solarYear, "Buddha's Birthday");

    // Tết Đoan Ngọ (Dragon Boat Festival) - 5/5 Lunar
    addLunarHoliday('Tết Đoan Ngọ', 5, 5, solarYear, 'Dragon Boat Festival');

    // Vu Lan (Wandering Souls Day) - 15/7 Lunar
    addLunarHoliday('Vu Lan (Rằm tháng Bảy)', 15, 7, solarYear, "Parents' Day / Wandering Souls");

    // Tết Trung Thu (Mid-Autumn Festival) - 15/8 Lunar
    addLunarHoliday('Tết Trung Thu (Rằm tháng Tám)', 15, 8, solarYear, "Mid-Autumn Festival / Children's Festival");

    // Tết Trùng Cửu - 9/9 Lunar
    addLunarHoliday('Tết Trùng Cửu', 9, 9, solarYear, 'Double Ninth Festival');

    // Tết Hạ Nguyên - 15/10 Lunar
    addLunarHoliday('Tết Hạ Nguyên (Rằm tháng Mười)', 15, 10, solarYear, 'Lower Nguyên Festival');

    // Ông Táo chầu trời (Kitchen Gods' Day) - 23/12 Lunar
    addLunarHoliday('Ông Táo chầu trời', 23, 12, solarYear - 1, 'Kitchen Gods go to Heaven');

    // Giao Thừa (New Year's Eve) - 30/12 Lunar
    addLunarHoliday('Giao Thừa (Đêm giao thừa)', 30, 12, solarYear - 1, "New Year's Eve");

    // Add all Rằm (15th) and Mùng 1 (1st) of each lunar month
    for (let month = 1; month <= 12; month++) {
        // Mùng 1 (New Moon - First day)
        addLunarHoliday(`Mùng 1 tháng ${month}`, 1, month, solarYear, 'First day of lunar month');
        
        // Rằm (Full Moon - 15th day)
        addLunarHoliday(`Rằm tháng ${month}`, 15, month, solarYear, 'Full moon day');
    }

    // Sort by date
    holidays.sort((a, b) => new Date(a.dateString) - new Date(b.dateString));

    return holidays;
}

/**
 * Get lunar date for any solar date
 */
function getLunarDate(day, month, year) {
    const timeZone = 7;
    const lunar = amlich.convertSolar2Lunar(day, month, year, timeZone);
    return {
        day: lunar[0],
        month: lunar[1],
        year: lunar[2],
        isLeapMonth: lunar[3] === 1
    };
}

/**
 * Get solar date from lunar date
 */
function getSolarDate(lunarDay, lunarMonth, lunarYear, isLeapMonth = false) {
    const timeZone = 7;
    const solar = amlich.convertLunar2Solar(lunarDay, lunarMonth, lunarYear, isLeapMonth ? 1 : 0, timeZone);
    return {
        day: solar[0],
        month: solar[1],
        year: solar[2]
    };
}

/**
 * Format for Google Calendar CSV import
 * CSV format: Subject,Start Date,Start Time,End Date,End Time,All Day Event,Description,Location
 */
function exportToGoogleCalendarCSV(year) {
    const holidays = getVietnameseHolidays(year);
    let csv = 'Subject,Start Date,Start Time,End Date,End Time,All Day Event,Description,Location\n';
    
    holidays.forEach(holiday => {
        const date = holiday.dateString.split('-').join('/'); // YYYY/MM/DD format
        const lunarInfo = holiday.lunarDate 
            ? `${holiday.lunarDate.day}/${holiday.lunarDate.month} Âm Lịch`
            : 'Solar date';
        
        csv += `"${holiday.name}","${date}","","${date}","",True,"${holiday.description} (${lunarInfo})","Vietnam"\n`;
    });
    
    return csv;
}

/**
 * Format for iCal (.ics) format - works with Google Calendar, Apple Calendar, Outlook
 */
function exportToICS(year) {
    const holidays = getVietnameseHolidays(year);
    let ics = 'BEGIN:VCALENDAR\n';
    ics += 'VERSION:2.0\n';
    ics += 'PRODID:-//Vietnamese Lunar Calendar//EN\n';
    ics += 'CALSCALE:GREGORIAN\n';
    ics += 'METHOD:PUBLISH\n';
    ics += 'X-WR-CALNAME:Vietnamese Lunar Calendar ' + year + '\n';
    ics += 'X-WR-TIMEZONE:Asia/Ho_Chi_Minh\n';
    
    holidays.forEach(holiday => {
        const dateStr = holiday.dateString.replace(/-/g, '');
        const uid = `${dateStr}-${holiday.name.replace(/\s/g, '-')}@amlich-view`;
        const lunarInfo = holiday.lunarDate 
            ? `${holiday.lunarDate.day}/${holiday.lunarDate.month} Âm Lịch`
            : 'Solar date';
        
        ics += 'BEGIN:VEVENT\n';
        ics += `UID:${uid}\n`;
        ics += `DTSTAMP:${new Date().toISOString().replace(/[-:]/g, '').split('.')[0]}Z\n`;
        ics += `DTSTART;VALUE=DATE:${dateStr}\n`;
        ics += `DTEND;VALUE=DATE:${dateStr}\n`;
        ics += `SUMMARY:${holiday.name}\n`;
        ics += `DESCRIPTION:${holiday.description}\\n${lunarInfo}\n`;
        ics += 'STATUS:CONFIRMED\n';
        ics += 'TRANSP:TRANSPARENT\n';
        ics += 'END:VEVENT\n';
    });
    
    ics += 'END:VCALENDAR\n';
    return ics;
}

module.exports = {
    getVietnameseHolidays,
    getLunarDate,
    getSolarDate,
    exportToGoogleCalendarCSV,
    exportToICS
};
