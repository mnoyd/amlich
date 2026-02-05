/**
 * Tiết Khí (24 Solar Terms) Calculations
 * 
 * Solar terms are based on the sun's ecliptic longitude:
 * - Each term corresponds to 15° (360° / 24)
 * - 0° = Xuân Phân (Spring Equinox)
 * - 15° = Thanh Minh
 * - ...and so on
 * 
 * References:
 * - Based on astronomical calculations from Jean Meeus
 * - Traditional Vietnamese naming
 */

const { SunLongitude, jdFromDate } = require('../amlich-core.js');

// 24 Solar Terms in Vietnamese (starting from 0° = Xuân Phân)
const TIET_KHI = [
    { name: 'Xuân Phân', description: 'Spring Equinox (Xuân Phân)', longitude: 0 },
    { name: 'Thanh Minh', description: 'Pure Brightness (Thanh Minh)', longitude: 15 },
    { name: 'Cốc Vũ', description: 'Grain Rain (Cốc Vũ)', longitude: 30 },
    { name: 'Lập Hạ', description: 'Start of Summer (Lập Hạ)', longitude: 45 },
    { name: 'Tiểu Mãn', description: 'Grain Buds (Tiểu Mãn)', longitude: 60 },
    { name: 'Mang Chủng', description: 'Grain in Ear (Mang Chủng)', longitude: 75 },
    { name: 'Hạ Chí', description: 'Summer Solstice (Hạ Chí)', longitude: 90 },
    { name: 'Tiểu Thử', description: 'Slight Heat (Tiểu Thử)', longitude: 105 },
    { name: 'Đại Thử', description: 'Great Heat (Đại Thử)', longitude: 120 },
    { name: 'Lập Thu', description: 'Start of Autumn (Lập Thu)', longitude: 135 },
    { name: 'Xử Thử', description: 'End of Heat (Xử Thử)', longitude: 150 },
    { name: 'Bạch Lộ', description: 'White Dew (Bạch Lộ)', longitude: 165 },
    { name: 'Thu Phân', description: 'Autumn Equinox (Thu Phân)', longitude: 180 },
    { name: 'Hàn Lộ', description: 'Cold Dew (Hàn Lộ)', longitude: 195 },
    { name: 'Sương Giáng', description: 'Frost Descent (Sương Giáng)', longitude: 210 },
    { name: 'Lập Đông', description: 'Start of Winter (Lập Đông)', longitude: 225 },
    { name: 'Tiểu Tuyết', description: 'Slight Snow (Tiểu Tuyết)', longitude: 240 },
    { name: 'Đại Tuyết', description: 'Great Snow (Đại Tuyết)', longitude: 255 },
    { name: 'Đông Chí', description: 'Winter Solstice (Đông Chí)', longitude: 270 },
    { name: 'Tiểu Hàn', description: 'Slight Cold (Tiểu Hàn)', longitude: 285 },
    { name: 'Đại Hàn', description: 'Great Cold (Đại Hàn)', longitude: 300 },
    { name: 'Lập Xuân', description: 'Start of Spring (Lập Xuân)', longitude: 315 },
    { name: 'Vũ Thủy', description: 'Rain Water (Vũ Thủy)', longitude: 330 },
    { name: 'Kinh Trập', description: 'Awakening of Insects (Kinh Trập)', longitude: 345 }
];

/**
 * Get Solar Term (Tiết Khí) for a given date
 * 
 * @param {number} jd - Julian Day Number
 * @param {number} timeZone - Timezone offset (default: 7 for Vietnam)
 * @returns {Object} Solar term information
 */
function getTietKhi(jd, timeZone = 7) {
    // Calculate sun longitude at local midnight
    const sunLongRad = SunLongitude(jd - 0.5 - timeZone / 24);
    
    // Convert radians to degrees
    const sunLongDeg = (sunLongRad * 180 / Math.PI) % 360;
    
    // Calculate term index (0-23)
    const termIndex = Math.floor(sunLongDeg / 15);
    
    const term = TIET_KHI[termIndex];
    
    return {
        index: termIndex,
        name: term.name,
        description: term.description,
        longitude: term.longitude,
        currentLongitude: Math.round(sunLongDeg * 100) / 100, // Round to 2 decimal places
        season: getSeason(termIndex)
    };
}

/**
 * Get season name from term index
 * 
 * @param {number} termIndex - Solar term index (0-23)
 * @returns {string} Season name
 */
function getSeason(termIndex) {
    if (termIndex >= 0 && termIndex <= 5) return 'Xuân (Spring)';
    if (termIndex >= 6 && termIndex <= 11) return 'Hạ (Summer)';
    if (termIndex >= 12 && termIndex <= 17) return 'Thu (Autumn)';
    return 'Đông (Winter)';
}

/**
 * Get all solar terms for a given year
 * Useful for displaying a full year calendar
 * 
 * @param {number} year - Solar year
 * @param {number} timeZone - Timezone offset
 * @returns {Array} Array of solar terms with dates
 */
function getAllTietKhiForYear(year, timeZone = 7) {
    const terms = [];
    const startJd = jdFromDate(1, 1, year);
    const endJd = jdFromDate(31, 12, year);
    
    let prevTermIndex = -1;
    
    for (let jd = startJd; jd <= endJd; jd++) {
        const tietKhi = getTietKhi(jd, timeZone);
        
        // Detect when we cross into a new term
        if (tietKhi.index !== prevTermIndex) {
            terms.push({
                jd,
                ...tietKhi
            });
            prevTermIndex = tietKhi.index;
        }
    }
    
    return terms;
}

module.exports = {
    getTietKhi,
    getAllTietKhiForYear,
    TIET_KHI
};
