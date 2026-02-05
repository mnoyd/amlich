// Main app logic

let currentYear = new Date().getFullYear();

// Initialize on page load
window.addEventListener('load', () => {
    updateTodayDisplay();
    loadHolidays();
    document.getElementById('year-input').value = currentYear;
});

function updateTodayDisplay() {
    const today = new Date();
    const lunar = getLunarDate(today.getDate(), today.getMonth() + 1, today.getFullYear());
    
    const solarStr = `${today.getDate()}/${today.getMonth() + 1}/${today.getFullYear()}`;
    const lunarStr = `${lunar.day}/${lunar.month}/${lunar.year}${lunar.isLeapMonth ? ' (nhuận)' : ''}`;
    
    document.getElementById('solar-today').textContent = solarStr;
    document.getElementById('lunar-today').textContent = lunarStr;
}

function loadHolidays() {
    const year = parseInt(document.getElementById('year-input').value);
    currentYear = year;
    
    const holidays = getVietnameseHolidays(year);
    const holidayList = document.getElementById('holiday-list');
    holidayList.innerHTML = '';
    
    // Filter out monthly events for main display
    const majorHolidays = holidays.filter(h => 
        !h.name.includes('Mùng 1 tháng') || 
        h.name.includes('Mùng 1 Tết') ||
        !h.name.includes('Rằm tháng') || 
        h.name.includes('Rằm tháng Giêng') ||
        h.name.includes('Rằm tháng Tư') ||
        h.name.includes('Rằm tháng Bảy') ||
        h.name.includes('Rằm tháng Tám') ||
        h.name.includes('Rằm tháng Mười')
    );
    
    majorHolidays.forEach(holiday => {
        const card = document.createElement('div');
        card.className = 'holiday-card';
        
        const lunarInfo = holiday.lunarDate 
            ? `${holiday.lunarDate.day}/${holiday.lunarDate.month} Âm Lịch`
            : 'Dương lịch';
        
        card.innerHTML = `
            <div class="date">${holiday.solarDate.day}/${holiday.solarDate.month}/${holiday.solarDate.year}</div>
            <div class="name">${getEmoji(holiday.name)} ${holiday.name}</div>
            <div class="lunar">${lunarInfo}</div>
            ${holiday.description ? `<div class="desc">${holiday.description}</div>` : ''}
        `;
        
        holidayList.appendChild(card);
    });
}

function getEmoji(name) {
    if (name.includes('Tết Nguyên Đán')) return '🎊';
    if (name.includes('Nguyên Tiêu')) return '🏮';
    if (name.includes('Thanh Minh')) return '🌸';
    if (name.includes('Phật Đản')) return '🙏';
    if (name.includes('Đoan Ngọ')) return '🐉';
    if (name.includes('Vu Lan')) return '👪';
    if (name.includes('Trung Thu')) return '🥮';
    if (name.includes('Ông Táo')) return '🍲';
    if (name.includes('Giao Thừa')) return '🎆';
    if (name.includes('Rằm')) return '🌕';
    if (name.includes('Mùng 1')) return '🌑';
    return '📅';
}

function switchTab(tabName) {
    // Hide all tabs
    document.querySelectorAll('.tab-content').forEach(tab => {
        tab.classList.remove('active');
    });
    
    // Deactivate all buttons
    document.querySelectorAll('.tab-btn').forEach(btn => {
        btn.classList.remove('active');
    });
    
    // Show selected tab
    document.getElementById(tabName).classList.add('active');
    
    // Activate selected button
    event.target.classList.add('active');
}

function convertDate() {
    const day = parseInt(document.getElementById('conv-day').value);
    const month = parseInt(document.getElementById('conv-month').value);
    const year = parseInt(document.getElementById('conv-year').value);
    const type = document.getElementById('conv-type').value;
    
    const result = document.getElementById('convert-result');
    
    if (type === 'solar') {
        const lunar = getLunarDate(day, month, year);
        result.innerHTML = `
            <strong>Dương lịch:</strong> ${day}/${month}/${year}<br>
            <strong>Âm lịch:</strong> ${lunar.day}/${lunar.month}/${lunar.year}${lunar.isLeapMonth ? ' (tháng nhuận)' : ''}
        `;
    } else {
        const solar = getSolarDate(day, month, year);
        result.innerHTML = `
            <strong>Âm lịch:</strong> ${day}/${month}/${year}<br>
            <strong>Dương lịch:</strong> ${solar.day}/${solar.month}/${solar.year}
        `;
    }
}

function downloadFile(content, filename, contentType) {
    const blob = new Blob([content], { type: contentType });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
}

function exportICS() {
    const year = currentYear;
    const ics = exportToICS(year);
    downloadFile(ics, `vietnamese-calendar-${year}.ics`, 'text/calendar');
}

function exportCSV() {
    const year = currentYear;
    const csv = exportToCSV(year);
    downloadFile(csv, `vietnamese-calendar-${year}.csv`, 'text/csv');
}
