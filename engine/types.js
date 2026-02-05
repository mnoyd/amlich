/**
 * Core Type Definitions and Constants for Vietnamese Lunar Calendar Expert Engine
 * 
 * Conventions:
 * - Indices are 0-based
 * - Timezone: UTC+7 (Vietnam)
 * - Lunar month 1 branch = Dần (index 2)
 */

// Thiên Can (Heavenly Stems) - 10 elements
const CAN = ['Giáp', 'Ất', 'Bính', 'Đinh', 'Mậu', 'Kỷ', 'Canh', 'Tân', 'Nhâm', 'Quý'];

// Địa Chi (Earthly Branches) - 12 elements
const CHI = ['Tý', 'Sửu', 'Dần', 'Mão', 'Thìn', 'Tỵ', 'Ngọ', 'Mùi', 'Thân', 'Dậu', 'Tuất', 'Hợi'];

// Vietnamese zodiac animals (Con Giáp) - aligned with CHI
const CON_GIAP = ['Tý (Chuột)', 'Sửu (Trâu)', 'Dần (Hổ)', 'Mão (Mèo)', 'Thìn (Rồng)', 'Tỵ (Rắn)', 
                  'Ngọ (Ngựa)', 'Mùi (Dê)', 'Thân (Khỉ)', 'Dậu (Gà)', 'Tuất (Chó)', 'Hợi (Lợn)'];

// Ngũ Hành (Five Elements) - aligned with CAN (2 stems per element)
const NGU_HANH_CAN = ['Mộc', 'Mộc', 'Hỏa', 'Hỏa', 'Thổ', 'Thổ', 'Kim', 'Kim', 'Thủy', 'Thủy'];

// Ngũ Hành for CHI
const NGU_HANH_CHI = ['Thủy', 'Thổ', 'Mộc', 'Mộc', 'Thổ', 'Hỏa', 'Hỏa', 'Thổ', 'Kim', 'Kim', 'Thổ', 'Thủy'];

// Days of week (Vietnamese)
const THU = ['Chủ Nhật', 'Thứ Hai', 'Thứ Ba', 'Thứ Tư', 'Thứ Năm', 'Thứ Sáu', 'Thứ Bảy'];

/**
 * Create a Can Chi object with full information
 * @param {number} canIndex - Stem index (0-9)
 * @param {number} chiIndex - Branch index (0-11)
 * @returns {Object} Can Chi information
 */
function createCanChi(canIndex, chiIndex) {
    const sexagenaryIndex = ((canIndex % 10) * 6 + Math.floor((chiIndex % 12) / 2)) % 60;
    
    return {
        canIndex,
        chiIndex,
        can: CAN[canIndex],
        chi: CHI[chiIndex],
        full: `${CAN[canIndex]} ${CHI[chiIndex]}`,
        conGiap: CON_GIAP[chiIndex],
        nguHanh: {
            can: NGU_HANH_CAN[canIndex],
            chi: NGU_HANH_CHI[chiIndex]
        },
        sexagenaryIndex
    };
}

/**
 * Normalize an index to 0-based range
 * @param {number} value - Value to normalize
 * @param {number} modulo - Modulo value (10 for Can, 12 for Chi)
 * @returns {number} Normalized index
 */
function normalizeIndex(value, modulo) {
    return ((value % modulo) + modulo) % modulo;
}

module.exports = {
    CAN,
    CHI,
    CON_GIAP,
    NGU_HANH_CAN,
    NGU_HANH_CHI,
    THU,
    createCanChi,
    normalizeIndex
};
