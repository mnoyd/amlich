/**
 * Test suite for Vietnamese Lunar Calendar Expert Engine
 * 
 * Verifies Can Chi calculations against known reference dates
 */

const { getDayInfo, formatDayInfo } = require('./index.js');

// Known reference dates with verified Can Chi
// Using standard formula: Can=(JD+9)%10, Chi=(JD+1)%12
const REFERENCE_DATES = [
    {
        solar: { day: 10, month: 2, year: 2024 },
        expected: {
            dayCanChi: 'Giáp Thìn',
            monthCanChi: 'Bính Dần',
            yearCanChi: 'Giáp Thìn',
            lunar: { day: 1, month: 1, year: 2024 },
            description: 'Tết Nguyên Đán 2024 (First day of Lunar New Year)'
        }
    },
    {
        solar: { day: 29, month: 1, year: 2025 },
        expected: {
            dayCanChi: 'Mậu Tuất',
            monthCanChi: 'Mậu Dần',
            yearCanChi: 'Ất Tỵ',
            lunar: { day: 1, month: 1, year: 2025 },
            description: 'Tết Nguyên Đán 2025'
        }
    },
    {
        solar: { day: 22, month: 1, year: 2023 },
        expected: {
            dayCanChi: 'Canh Thìn',
            monthCanChi: 'Giáp Dần',
            yearCanChi: 'Quý Mão',
            lunar: { day: 1, month: 1, year: 2023 },
            description: 'Tết Nguyên Đán 2023'
        }
    },
    {
        solar: { day: 1, month: 1, year: 2024 },
        expected: {
            dayCanChi: 'Giáp Tý',
            monthCanChi: 'Giáp Tý',
            yearCanChi: 'Quý Mão',
            lunar: { day: 20, month: 11, year: 2023 },
            description: 'New Year 2024 (solar)'
        }
    },
    {
        solar: { day: 1, month: 1, year: 2000 },
        expected: {
            dayCanChi: 'Mậu Ngọ',
            monthCanChi: 'Bính Tý',
            yearCanChi: 'Kỷ Mão',
            lunar: { day: 25, month: 11, year: 1999 },
            description: 'Y2K - Millennium reference date'
        }
    },
    {
        solar: { day: 5, month: 2, year: 2026 },
        expected: {
            description: 'Random future date test'
        }
    }
];

console.log('🧪 Vietnamese Lunar Calendar Expert Engine - Test Suite\n');
console.log('='.repeat(80));

let passCount = 0;
let failCount = 0;

REFERENCE_DATES.forEach((testCase, index) => {
    console.log(`\nTest ${index + 1}: ${testCase.expected.description}`);
    console.log('-'.repeat(80));
    
    const { day, month, year } = testCase.solar;
    const info = getDayInfo(day, month, year);
    
    console.log(formatDayInfo(info));
    
    // Validate expected values
    let testPassed = true;
    
    if (testCase.expected.dayCanChi) {
        if (info.canChi.day.full === testCase.expected.dayCanChi) {
            console.log(`✅ Day Can Chi: ${info.canChi.day.full} (PASS)`);
        } else {
            console.log(`❌ Day Can Chi: Expected ${testCase.expected.dayCanChi}, got ${info.canChi.day.full} (FAIL)`);
            testPassed = false;
        }
    }
    
    if (testCase.expected.monthCanChi) {
        if (info.canChi.month.full.startsWith(testCase.expected.monthCanChi)) {
            console.log(`✅ Month Can Chi: ${info.canChi.month.full} (PASS)`);
        } else {
            console.log(`❌ Month Can Chi: Expected ${testCase.expected.monthCanChi}, got ${info.canChi.month.full} (FAIL)`);
            testPassed = false;
        }
    }
    
    if (testCase.expected.yearCanChi) {
        if (info.canChi.year.full === testCase.expected.yearCanChi) {
            console.log(`✅ Year Can Chi: ${info.canChi.year.full} (PASS)`);
        } else {
            console.log(`❌ Year Can Chi: Expected ${testCase.expected.yearCanChi}, got ${info.canChi.year.full} (FAIL)`);
            testPassed = false;
        }
    }
    
    if (testCase.expected.lunar) {
        const lunarMatch = info.lunar.day === testCase.expected.lunar.day &&
                          info.lunar.month === testCase.expected.lunar.month &&
                          info.lunar.year === testCase.expected.lunar.year;
        
        if (lunarMatch) {
            console.log(`✅ Lunar date: ${info.lunar.dateString} (PASS)`);
        } else {
            console.log(`❌ Lunar date: Expected ${testCase.expected.lunar.day}/${testCase.expected.lunar.month}/${testCase.expected.lunar.year}, got ${info.lunar.dateString} (FAIL)`);
            testPassed = false;
        }
    }
    
    if (testPassed) {
        passCount++;
    } else {
        failCount++;
    }
});

console.log('\n' + '='.repeat(80));
console.log(`\n📊 Test Results: ${passCount} passed, ${failCount} failed out of ${REFERENCE_DATES.length} tests`);

if (failCount === 0) {
    console.log('✅ All tests passed!\n');
    process.exit(0);
} else {
    console.log('❌ Some tests failed. Please review formulas.\n');
    process.exit(1);
}
