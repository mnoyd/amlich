/**
 * Vietnamese Lunar Calendar Expert Engine
 * 
 * Main entry point for almanac calculations.
 * Provides comprehensive information about any date including:
 * - Solar and Lunar dates
 * - Can Chi (Heavenly Stems & Earthly Branches) for day/month/year
 * - Traditional almanac data (to be added in future phases)
 * 
 * Usage:
 *   const { getDayInfo } = require('./engine');
 *   const info = getDayInfo(10, 2, 2024);  // Feb 10, 2024
 *   console.log(info.canChi.day.full);     // "Giáp Thìn"
 */

const { jdFromDate, convertSolar2Lunar } = require('../amlich-core.js');
const { getDayCanChi, getMonthCanChi, getYearCanChi } = require('./canchi.js');
const { THU } = require('./types.js');

/**
 * Get comprehensive information for a given solar date
 * 
 * @param {number} dd - Day (1-31)
 * @param {number} mm - Month (1-12)
 * @param {number} yyyy - Year
 * @param {Object} options - Optional settings
 * @param {number} options.timezone - Timezone offset (default: 7 for Vietnam)
 * @returns {Object} Complete day information
 */
function getDayInfo(dd, mm, yyyy, options = {}) {
    const timeZone = options.timezone || 7;
    
    // Calculate Julian Day Number
    const jd = jdFromDate(dd, mm, yyyy);
    
    // Convert to lunar date
    const lunar = convertSolar2Lunar(dd, mm, yyyy, timeZone);
    const lunarDay = lunar[0];
    const lunarMonth = lunar[1];
    const lunarYear = lunar[2];
    const isLeapMonth = lunar[3];
    
    // Calculate day of week (JD + 1 because JD 0 was Monday)
    const dayOfWeekIndex = (jd + 1) % 7;
    
    // Calculate Can Chi for day, month, year
    const dayCanChi = getDayCanChi(jd);
    const monthCanChi = getMonthCanChi(lunarMonth, lunarYear, isLeapMonth);
    const yearCanChi = getYearCanChi(lunarYear);
    
    return {
        // Solar date information
        solar: {
            day: dd,
            month: mm,
            year: yyyy,
            dayOfWeek: dayOfWeekIndex,
            dayOfWeekName: THU[dayOfWeekIndex],
            dateString: `${yyyy}-${String(mm).padStart(2, '0')}-${String(dd).padStart(2, '0')}`
        },
        
        // Lunar date information
        lunar: {
            day: lunarDay,
            month: lunarMonth,
            year: lunarYear,
            isLeapMonth: isLeapMonth === 1,
            dateString: `${lunarDay}/${lunarMonth}/${lunarYear}${isLeapMonth === 1 ? ' (nhuận)' : ''}`
        },
        
        // Julian Day Number (for reference)
        jd,
        
        // Can Chi information
        canChi: {
            day: dayCanChi,
            month: monthCanChi,
            year: yearCanChi,
            
            // Full date in Can Chi format
            full: `${dayCanChi.full}, tháng ${monthCanChi.full}, năm ${yearCanChi.full}`
        },
        
        // Metadata about calculation methods
        _meta: {
            version: '1.0.0',
            timezone: timeZone,
            methods: {
                dayCanChi: 'jd-based: (jd+9)%10, (jd+1)%12',
                monthCanChi: 'lunar-month-based with year-stem table',
                yearCanChi: 'lunar-year-based: (year+6)%10, (year+8)%12'
            },
            conventions: {
                lunarMonth1Branch: 'Dần (index 2)',
                timezone: 'UTC+7 (Vietnam)',
                dayBoundary: 'local midnight'
            }
        }
    };
}

/**
 * Get Can Chi information only (lighter than full getDayInfo)
 * 
 * @param {number} dd - Day
 * @param {number} mm - Month
 * @param {number} yyyy - Year
 * @param {Object} options - Optional settings
 * @returns {Object} Can Chi information
 */
function getCanChi(dd, mm, yyyy, options = {}) {
    const info = getDayInfo(dd, mm, yyyy, options);
    return info.canChi;
}

/**
 * Format day info as a readable string
 * 
 * @param {Object} dayInfo - Result from getDayInfo()
 * @returns {string} Formatted string
 */
function formatDayInfo(dayInfo) {
    const lines = [];
    
    lines.push(`📅 Ngày ${dayInfo.solar.dateString} (${dayInfo.solar.dayOfWeekName})`);
    lines.push(`🌙 Âm lịch: ${dayInfo.lunar.dateString}`);
    lines.push(`📜 Can Chi:`);
    lines.push(`   • Ngày: ${dayInfo.canChi.day.full} (${dayInfo.canChi.day.conGiap})`);
    lines.push(`   • Tháng: ${dayInfo.canChi.month.full}`);
    lines.push(`   • Năm: ${dayInfo.canChi.year.full} (${dayInfo.canChi.year.conGiap})`);
    lines.push(`🌟 Ngũ hành:`);
    lines.push(`   • Ngày: ${dayInfo.canChi.day.nguHanh.can} (Can) - ${dayInfo.canChi.day.nguHanh.chi} (Chi)`);
    
    return lines.join('\n');
}

module.exports = {
    getDayInfo,
    getCanChi,
    formatDayInfo
};
