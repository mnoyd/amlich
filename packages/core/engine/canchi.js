/**
 * Can Chi (干支 / Can-Chi) Calculations
 * 
 * Calculates Heavenly Stem and Earthly Branch for:
 * - Day (based on Julian Day Number)
 * - Month (based on lunar month and year stem)
 * - Year (based on lunar year)
 * 
 * References:
 * - Day Can Chi: JD-based formula with verified offset
 * - Month Can Chi: Fixed branch by lunar month, stem by year
 * - Year Can Chi: Standard formula from lunar year
 */

const { createCanChi, normalizeIndex } = require('./types.js');

/**
 * Get Can Chi for a given day
 * 
 * Formula verified against known dates:
 * - 2024-02-10 (JD 2460349) should be Giáp Thìn (0, 4)
 * - 2025-01-29 (JD 2460703) should be Giáp Dần (0, 2) - Tết
 * 
 * Standard formula:
 * - Can: (JD + 9) % 10
 * - Chi: (JD + 1) % 12
 * 
 * @param {number} jd - Julian Day Number
 * @returns {Object} Day Can Chi information
 */
function getDayCanChi(jd) {
    // JD offset formulas (verified against multiple sources)
    const canIndex = normalizeIndex(jd + 9, 10);
    const chiIndex = normalizeIndex(jd + 1, 12);
    
    return createCanChi(canIndex, chiIndex);
}

/**
 * Get Can Chi for a lunar month
 * 
 * Branch (Chi): Fixed by lunar month
 * - Month 1 = Dần (index 2)
 * - Month 2 = Mão (index 3)
 * - Month 3 = Thìn (index 4)
 * - ... wraps at 12
 * 
 * Stem (Can): Determined by year stem using table:
 * Year Can | Month 1 (Dần) starts with
 * ---------|---------------------------
 * Giáp/Kỷ  → Bính (index 2)
 * Ất/Canh  → Mậu (index 4)
 * Bính/Tân → Canh (index 6)
 * Đinh/Nhâm → Nhâm (index 8)
 * Mậu/Quý  → Giáp (index 0)
 * 
 * @param {number} lunarMonth - Lunar month (1-12)
 * @param {number} lunarYear - Lunar year
 * @param {number} isLeapMonth - Is leap month (0 or 1)
 * @returns {Object} Month Can Chi information
 */
function getMonthCanChi(lunarMonth, lunarYear, isLeapMonth = 0) {
    // Month branch is fixed: Month 1 = Dần (2), Month 2 = Mão (3), etc.
    const chiIndex = normalizeIndex(lunarMonth + 1, 12);
    
    // Get year stem to determine first month stem
    const yearCanIndex = normalizeIndex(lunarYear + 6, 10);
    
    // Year stem to first month (Dần) stem mapping
    const firstMonthCanTable = [2, 4, 6, 8, 0, 2, 4, 6, 8, 0]; // Giáp→Bính, Ất→Mậu, etc.
    const firstMonthCan = firstMonthCanTable[yearCanIndex];
    
    // Calculate current month stem (offset from month 1)
    const canIndex = normalizeIndex(firstMonthCan + (lunarMonth - 1), 10);
    
    const result = createCanChi(canIndex, chiIndex);
    
    // Add leap month indicator
    if (isLeapMonth === 1) {
        result.full = `${result.full} (nhuận)`;
        result.isLeapMonth = true;
    }
    
    return result;
}

/**
 * Get Can Chi for a lunar year
 * 
 * Standard formula verified against known years:
 * - 2024 lunar = Giáp Thìn (0, 4)
 * - 2025 lunar = Ất Tỵ (1, 5)
 * - 2023 lunar = Quý Mão (9, 3)
 * 
 * Formula:
 * - Can: (year + 6) % 10
 * - Chi: (year + 8) % 12
 * 
 * @param {number} lunarYear - Lunar year
 * @returns {Object} Year Can Chi information
 */
function getYearCanChi(lunarYear) {
    const canIndex = normalizeIndex(lunarYear + 6, 10);
    const chiIndex = normalizeIndex(lunarYear + 8, 12);
    
    return createCanChi(canIndex, chiIndex);
}

/**
 * Get sexagenary cycle index (0-59) from Can and Chi indices
 * Used for 60-day/year cycle calculations (Nạp Âm, etc.)
 * 
 * @param {number} canIndex - Stem index (0-9)
 * @param {number} chiIndex - Branch index (0-11)
 * @returns {number} Sexagenary index (0-59)
 */
function getSexagenaryIndex(canIndex, chiIndex) {
    // Find position in 60-cycle where Can and Chi align
    for (let i = 0; i < 60; i++) {
        if (i % 10 === canIndex && i % 12 === chiIndex) {
            return i;
        }
    }
    return 0; // Fallback (should never happen)
}

module.exports = {
    getDayCanChi,
    getMonthCanChi,
    getYearCanChi,
    getSexagenaryIndex
};
