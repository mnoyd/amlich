const { getVietnameseHolidays, getLunarDate, getSolarDate } = require('./vietnamese-holidays.js');

console.log('🧪 Testing Vietnamese Lunar Calendar...\n');

// Test 1: Convert today to lunar
console.log('Test 1: Today\'s date');
const today = new Date();
const lunar = getLunarDate(today.getDate(), today.getMonth() + 1, today.getFullYear());
console.log(`Solar: ${today.getDate()}/${today.getMonth() + 1}/${today.getFullYear()}`);
console.log(`Lunar: ${lunar.day}/${lunar.month}/${lunar.year}${lunar.isLeapMonth ? ' (leap month)' : ''}`);

// Test 2: Tết 2024 (Lunar New Year)
console.log('\nTest 2: Tết 2024 (1/1/2024 Lunar)');
const tet2024 = getSolarDate(1, 1, 2024);
console.log(`1/1/2024 Lunar = ${tet2024.day}/${tet2024.month}/${tet2024.year} Solar`);

// Test 3: Mid-Autumn Festival 2024 (15/8 Lunar)
console.log('\nTest 3: Trung Thu 2024 (15/8/2024 Lunar)');
const trungThu = getSolarDate(15, 8, 2024);
console.log(`15/8/2024 Lunar = ${trungThu.day}/${trungThu.month}/${trungThu.year} Solar`);

// Test 4: List major holidays for 2024
console.log('\nTest 4: Major holidays for 2024');
const holidays = getVietnameseHolidays(2024);
const major = holidays.filter(h => 
    h.name.includes('Tết') || 
    h.name.includes('Phật Đản') || 
    h.name.includes('Vu Lan')
);

major.forEach(h => {
    console.log(`${h.dateString} - ${h.name}`);
});

console.log('\n✅ All tests completed!');
