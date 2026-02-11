const amlich = require('./amlich-core.js');
const { getLunarFestivalRows, getSolarHolidayRows } = require('./holiday-data.js');

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

    function getNthWeekdayOfMonth(year, month, weekday, nth) {
        const first = new Date(year, month - 1, 1);
        const firstWeekday = first.getDay();
        const offset = (7 + weekday - firstWeekday) % 7;
        return 1 + offset + 7 * (nth - 1);
    }

    // Lunar festivals from shared JSON data
    getLunarFestivalRows().forEach(festival => {
        const lunarYear = solarYear + festival.yearOffset;
        addLunarHoliday(festival.name, festival.lunarDay, festival.lunarMonth, lunarYear, festival.description);
    });

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

    // National / social / international solar observances from shared JSON data
    getSolarHolidayRows().forEach(holiday => {
        holidays.push({
            name: holiday.name,
            description: holiday.description,
            solarDate: { day: holiday.solarDay, month: holiday.solarMonth, year: solarYear },
            dateString: `${solarYear}-${String(holiday.solarMonth).padStart(2, '0')}-${String(holiday.solarDay).padStart(2, '0')}`,
            isSolar: true
        });
    });

    const motherDay = getNthWeekdayOfMonth(solarYear, 5, 0, 2);
    holidays.push({
        name: 'Ngày của Mẹ',
        description: "Mother's Day (2nd Sunday of May)",
        solarDate: { day: motherDay, month: 5, year: solarYear },
        dateString: `${solarYear}-05-${String(motherDay).padStart(2, '0')}`,
        isSolar: true
    });

    const fatherDay = getNthWeekdayOfMonth(solarYear, 6, 0, 3);
    holidays.push({
        name: 'Ngày của Cha',
        description: "Father's Day (3rd Sunday of June)",
        solarDate: { day: fatherDay, month: 6, year: solarYear },
        dateString: `${solarYear}-06-${String(fatherDay).padStart(2, '0')}`,
        isSolar: true
    });

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
