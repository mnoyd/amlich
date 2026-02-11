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

    function getNthWeekdayOfMonth(year, month, weekday, nth) {
        const first = new Date(year, month - 1, 1);
        const firstWeekday = first.getDay();
        const offset = (7 + weekday - firstWeekday) % 7;
        return 1 + offset + 7 * (nth - 1);
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

    // National / social / international solar observances
    const solarHolidays = [
        ['Tết Dương Lịch', 1, 1, "New Year's Day"],
        ['Ngày Truyền Thống Học Sinh - Sinh Viên Việt Nam', 9, 1, "Vietnamese Students' Day"],
        ['Ngày Thành Lập Đảng Cộng Sản Việt Nam', 3, 2, 'Founding Anniversary of the Communist Party of Vietnam'],
        ['Lễ Tình Nhân (Valentine)', 14, 2, "Valentine's Day"],
        ['Ngày Quyền Của Người Tiêu Dùng Việt Nam', 15, 3, 'Vietnam Consumer Rights Day'],
        ['Ngày Thầy Thuốc Việt Nam', 27, 2, "Vietnamese Doctors' Day"],
        ['Ngày Quốc Tế Phụ Nữ', 8, 3, "International Women's Day"],
        ['Ngày Nước Thế Giới', 22, 3, 'World Water Day'],
        ['Ngày Thành Lập Đoàn TNCS Hồ Chí Minh', 26, 3, "Founding of Ho Chi Minh Communist Youth Union"],
        ['Ngày Quốc Tế Hạnh Phúc', 20, 3, 'International Day of Happiness'],
        ['Ngày Cá Tháng Tư', 1, 4, "April Fools' Day"],
        ['Ngày Sức Khỏe Thế Giới', 7, 4, 'World Health Day'],
        ['Ngày Sách Việt Nam', 21, 4, 'Vietnamese Book and Reading Culture Day'],
        ['Ngày Trái Đất', 22, 4, 'Earth Day'],
        ['Ngày Kiến Trúc Sư Việt Nam', 27, 4, "Vietnamese Architects' Day"],
        ['Ngày Giải Phóng Miền Nam', 30, 4, 'Reunification Day'],
        ['Ngày Quốc Tế Lao Động', 1, 5, "International Workers' Day"],
        ['Ngày Chiến Thắng Điện Biên Phủ', 7, 5, 'Dien Bien Phu Victory Day'],
        ['Ngày Chiến Thắng Phát Xít', 9, 5, 'Victory Day over Fascism'],
        ['Ngày Quốc Tế Gia Đình', 15, 5, 'International Day of Families'],
        ['Ngày Sinh Chủ Tịch Hồ Chí Minh', 19, 5, "President Ho Chi Minh's Birthday"],
        ['Ngày Khoa Học và Công Nghệ Việt Nam', 18, 5, 'Vietnam Science and Technology Day'],
        ['Ngày Quốc Tế Thiếu Nhi', 1, 6, "International Children's Day"],
        ['Ngày Môi Trường Thế Giới', 5, 6, 'World Environment Day'],
        ['Ngày Gia Đình Việt Nam', 28, 6, 'Vietnamese Family Day'],
        ['Ngày Dân Số Thế Giới', 11, 7, 'World Population Day'],
        ['Ngày Thương Binh - Liệt Sĩ', 27, 7, 'War Invalids and Martyrs Day'],
        ['Ngày Quốc Tế Thanh Niên', 12, 8, 'International Youth Day'],
        ['Ngày Cách Mạng Tháng Tám', 19, 8, 'August Revolution Day'],
        ['Ngày Truyền Thống Công An Nhân Dân', 19, 8, "People's Public Security Traditional Day"],
        ['Ngày Quốc Tế Hòa Bình', 21, 9, 'International Day of Peace'],
        ['Ngày Quốc Khánh', 2, 9, 'National Day / Independence Day'],
        ['Ngày Xóa Mù Chữ Quốc Tế', 8, 9, 'International Literacy Day'],
        ['Ngày Quốc Tế Người Cao Tuổi', 1, 10, 'International Day of Older Persons'],
        ['Ngày Giải Phóng Thủ Đô', 10, 10, 'Liberation Day of the Capital'],
        ['Ngày Chuyển Đổi Số Quốc Gia', 10, 10, 'National Digital Transformation Day'],
        ['Ngày Doanh Nhân Việt Nam', 13, 10, "Vietnamese Entrepreneurs' Day"],
        ['Ngày Phụ Nữ Việt Nam', 20, 10, "Vietnamese Women's Day"],
        ['Halloween', 31, 10, 'Halloween'],
        ['Ngày Pháp Luật Việt Nam', 9, 11, 'Vietnam Law Day'],
        ['Ngày Quốc Tế Nam Giới', 19, 11, "International Men's Day"],
        ['Ngày Nhà Giáo Việt Nam', 20, 11, "Vietnamese Teachers' Day"],
        ['Ngày Thành Lập Hội Chữ Thập Đỏ Việt Nam', 23, 11, 'Vietnam Red Cross Society Founding Day'],
        ['Ngày Di Sản Văn Hóa Việt Nam', 23, 11, 'Vietnamese Cultural Heritage Day'],
        ['Ngày Thế Giới Phòng Chống AIDS', 1, 12, 'World AIDS Day'],
        ['Ngày Quốc Tế Người Khuyết Tật', 3, 12, 'International Day of Persons with Disabilities'],
        ['Ngày Toàn Quốc Kháng Chiến', 19, 12, 'National Resistance Day'],
        ['Ngày Thành Lập Quân Đội Nhân Dân Việt Nam', 22, 12, "Vietnam People's Army Founding Day"],
        ['Lễ Giáng Sinh', 25, 12, 'Christmas Day']
    ];

    solarHolidays.forEach(([name, day, month, description]) => {
        holidays.push({
            name,
            description,
            solarDate: { day, month, year: solarYear },
            dateString: `${solarYear}-${String(month).padStart(2, '0')}-${String(day).padStart(2, '0')}`,
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
