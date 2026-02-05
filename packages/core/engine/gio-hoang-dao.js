/**
 * Giờ Hoàng Đạo (Auspicious Hours) Calculations
 * 
 * Based on the traditional 12-Star System (Thập Nhị Kiến Trừ)
 * Each day has 12 hours, and each hour is governed by one of 12 stars.
 * The cycle starts at different hours depending on the Day's Branch (Chi).
 * 
 * The 12 Stars:
 * - Good Stars (Hoàng Đạo - 6 stars): Thanh Long, Minh Đường, Kim Quỹ, Bảo Quang, Ngọc Đường, Tư Mệnh
 * - Bad Stars (Hắc Đạo - 6 stars): Thiên Hình, Chu Tước, Bạch Hổ, Thiên Lao, Nguyên Vũ, Câu Trận
 */

const { CHI } = require('./types.js');

// 12 Stars in order (Good/Bad alternating pattern with variations)
const TWELVE_STARS = [
    { name: 'Thanh Long', type: 'good', description: 'Azure Dragon - Very auspicious' },
    { name: 'Minh Đường', type: 'good', description: 'Bright Hall - Auspicious' },
    { name: 'Thiên Hình', type: 'bad', description: 'Heavenly Punishment - Ominous' },
    { name: 'Chu Tước', type: 'bad', description: 'Vermilion Bird - Ominous' },
    { name: 'Kim Quỹ', type: 'good', description: 'Golden Coffer - Auspicious' },
    { name: 'Bảo Quang', type: 'good', description: 'Precious Light - Auspicious' },
    { name: 'Bạch Hổ', type: 'bad', description: 'White Tiger - Very ominous' },
    { name: 'Ngọc Đường', type: 'good', description: 'Jade Hall - Auspicious' },
    { name: 'Thiên Lao', type: 'bad', description: 'Heavenly Prison - Ominous' },
    { name: 'Nguyên Vũ', type: 'bad', description: 'Black Tortoise - Ominous' },
    { name: 'Tư Mệnh', type: 'good', description: 'Life Star - Auspicious' },
    { name: 'Câu Trận', type: 'bad', description: 'Hook Array - Ominous' }
];

// Start hour offset for each Day Branch (where Thanh Long appears)
// Key: Day Branch index (0-11), Value: Hour Branch index where cycle starts
const DAY_TO_START_HOUR = {
    0: 0,   // Tý day → Thanh Long at Tý hour
    1: 10,  // Sửu day → Thanh Long at Tuất hour
    2: 8,   // Dần day → Thanh Long at Thân hour
    3: 6,   // Mão day → Thanh Long at Ngọ hour
    4: 4,   // Thìn day → Thanh Long at Thìn hour
    5: 2,   // Tỵ day → Thanh Long at Dần hour
    6: 0,   // Ngọ day → Thanh Long at Tý hour
    7: 10,  // Mùi day → Thanh Long at Tuất hour
    8: 8,   // Thân day → Thanh Long at Thân hour
    9: 6,   // Dậu day → Thanh Long at Ngọ hour
    10: 4,  // Tuất day → Thanh Long at Thìn hour
    11: 2   // Hợi day → Thanh Long at Dần hour
};

/**
 * Get hour time range from Chi index
 * Vietnamese traditional hours: Each Chi covers 2 modern hours
 * 
 * @param {number} chiIndex - Hour Branch index (0-11)
 * @returns {string} Time range (e.g., "23:00-01:00")
 */
function getHourTimeRange(chiIndex) {
    const ranges = [
        '23:00-01:00', // Tý
        '01:00-03:00', // Sửu
        '03:00-05:00', // Dần
        '05:00-07:00', // Mão
        '07:00-09:00', // Thìn
        '09:00-11:00', // Tỵ
        '11:00-13:00', // Ngọ
        '13:00-15:00', // Mùi
        '15:00-17:00', // Thân
        '17:00-19:00', // Dậu
        '19:00-21:00', // Tuất
        '21:00-23:00'  // Hợi
    ];
    return ranges[chiIndex];
}

/**
 * Get Auspicious Hours (Giờ Hoàng Đạo) for a given day
 * 
 * @param {number} dayChiIndex - Day's Branch index (0-11)
 * @returns {Object} Hour information with stars
 */
function getGioHoangDao(dayChiIndex) {
    const startHour = DAY_TO_START_HOUR[dayChiIndex];
    const hours = [];
    const goodHours = [];
    
    // Calculate star for each of the 12 hours
    for (let i = 0; i < 12; i++) {
        const hourChiIndex = i;
        const hourChi = CHI[hourChiIndex];
        
        // Calculate which star governs this hour
        // The star cycle starts at startHour with Thanh Long (index 0)
        const starIndex = (hourChiIndex - startHour + 12) % 12;
        const star = TWELVE_STARS[starIndex];
        
        const hourInfo = {
            hourIndex: hourChiIndex,
            hourChi: hourChi,
            timeRange: getHourTimeRange(hourChiIndex),
            star: star.name,
            starDescription: star.description,
            type: star.type,
            isGood: star.type === 'good'
        };
        
        hours.push(hourInfo);
        
        if (star.type === 'good') {
            goodHours.push(hourInfo);
        }
    }
    
    return {
        dayChiIndex,
        dayChi: CHI[dayChiIndex],
        allHours: hours,
        goodHours: goodHours,
        goodHourCount: goodHours.length,
        summary: goodHours.map(h => `${h.hourChi} (${h.timeRange})`).join(', ')
    };
}

/**
 * Check if a specific hour is auspicious
 * 
 * @param {number} dayChiIndex - Day's Branch index
 * @param {number} hourChiIndex - Hour's Branch index
 * @returns {Object} Hour details with star info
 */
function isHourAuspicious(dayChiIndex, hourChiIndex) {
    const result = getGioHoangDao(dayChiIndex);
    return result.allHours[hourChiIndex];
}

module.exports = {
    getGioHoangDao,
    isHourAuspicious,
    TWELVE_STARS,
    getHourTimeRange
};
