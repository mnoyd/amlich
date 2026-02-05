// Browser-compatible version of vietnamese-holidays.js

const PI = Math.PI;

function getVietnameseHolidays(solarYear) {
    const timeZone = 7;
    const holidays = [];

    function addLunarHoliday(name, lunarDay, lunarMonth, lunarYear, description = '') {
        const solar = convertLunar2Solar(lunarDay, lunarMonth, lunarYear, 0, timeZone);
        if (solar[0] > 0) {
            holidays.push({
                name,
                description,
                lunarDate: { day: lunarDay, month: lunarMonth, year: lunarYear },
                solarDate: { day: solar[0], month: solar[1], year: solar[2] },
                dateString: `${solar[2]}-${String(solar[1]).padStart(2, '0')}-${String(solar[0]).padStart(2, '0')}`
            });
        }
    }

    // Major holidays
    addLunarHoliday('Tết Nguyên Đán (Mùng 1 Tết)', 1, 1, solarYear, 'Vietnamese New Year - First day');
    addLunarHoliday('Mùng 2 Tết', 2, 1, solarYear, 'Second day of Tết');
    addLunarHoliday('Mùng 3 Tết', 3, 1, solarYear, 'Third day of Tết');
    addLunarHoliday('Tết Nguyên Tiêu (Rằm tháng Giêng)', 15, 1, solarYear, 'Lantern Festival');
    addLunarHoliday('Tết Hàn Thực', 3, 3, solarYear, 'Cold Food Festival');
    
    holidays.push({
        name: 'Tết Thanh Minh',
        description: 'Tomb Sweeping Day (Solar calendar)',
        solarDate: { day: 5, month: 4, year: solarYear },
        dateString: `${solarYear}-04-05`,
        isSolar: true
    });
    
    addLunarHoliday('Phật Đản (Rằm tháng Tư)', 15, 4, solarYear, "Buddha's Birthday");
    addLunarHoliday('Tết Đoan Ngọ', 5, 5, solarYear, 'Dragon Boat Festival');
    addLunarHoliday('Vu Lan (Rằm tháng Bảy)', 15, 7, solarYear, "Parents' Day / Wandering Souls");
    addLunarHoliday('Tết Trung Thu (Rằm tháng Tám)', 15, 8, solarYear, "Mid-Autumn Festival");
    addLunarHoliday('Tết Trùng Cửu', 9, 9, solarYear, 'Double Ninth Festival');
    addLunarHoliday('Tết Hạ Nguyên (Rằm tháng Mười)', 15, 10, solarYear, 'Lower Nguyên Festival');
    addLunarHoliday('Ông Táo chầu trời', 23, 12, solarYear - 1, 'Kitchen Gods go to Heaven');
    addLunarHoliday('Giao Thừa (Đêm giao thừa)', 30, 12, solarYear - 1, "New Year's Eve");

    // Monthly events
    for (let month = 1; month <= 12; month++) {
        addLunarHoliday(`Mùng 1 tháng ${month}`, 1, month, solarYear, 'First day of lunar month');
        addLunarHoliday(`Rằm tháng ${month}`, 15, month, solarYear, 'Full moon day');
    }

    holidays.sort((a, b) => new Date(a.dateString) - new Date(b.dateString));
    return holidays;
}

function getLunarDate(day, month, year) {
    const timeZone = 7;
    const lunar = convertSolar2Lunar(day, month, year, timeZone);
    return {
        day: lunar[0],
        month: lunar[1],
        year: lunar[2],
        isLeapMonth: lunar[3] === 1
    };
}

function getSolarDate(lunarDay, lunarMonth, lunarYear, isLeapMonth = false) {
    const timeZone = 7;
    const solar = convertLunar2Solar(lunarDay, lunarMonth, lunarYear, isLeapMonth ? 1 : 0, timeZone);
    return {
        day: solar[0],
        month: solar[1],
        year: solar[2]
    };
}

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

function exportToCSV(year) {
    const holidays = getVietnameseHolidays(year);
    let csv = 'Subject,Start Date,Start Time,End Date,End Time,All Day Event,Description,Location\n';
    
    holidays.forEach(holiday => {
        const date = holiday.dateString.split('-').join('/');
        const lunarInfo = holiday.lunarDate 
            ? `${holiday.lunarDate.day}/${holiday.lunarDate.month} Âm Lịch`
            : 'Solar date';
        
        csv += `"${holiday.name}","${date}","","${date}","",True,"${holiday.description} (${lunarInfo})","Vietnam"\n`;
    });
    
    return csv;
}
