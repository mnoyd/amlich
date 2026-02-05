<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  type GoodHour = {
    hour_chi: string;
    time_range: string;
    star: string;
  };

  type DayCell = {
    day: number;
    month: number;
    year: number;
    day_of_week_index: number;
    day_of_week: string;
    solar_date: string;
    lunar_day: number;
    lunar_month: number;
    lunar_year: number;
    lunar_leap: boolean;
    lunar_date: string;
    canchi_day: string;
    canchi_month: string;
    canchi_year: string;
    tiet_khi: string;
    tiet_khi_description: string;
    tiet_khi_season: string;
    good_hours: GoodHour[];
    holidays: HolidayInfo[];
  };

  type HolidayInfo = {
    name: string;
    description: string;
    is_solar: boolean;
    lunar_day: number | null;
    lunar_month: number | null;
    is_major: boolean;
  };

  type MonthData = {
    month: number;
    year: number;
    first_weekday: number;
    days: DayCell[];
  };

  const weekLabels = ["CN", "T2", "T3", "T4", "T5", "T6", "T7"];
  const monthNames = [
    "Tháng 1",
    "Tháng 2",
    "Tháng 3",
    "Tháng 4",
    "Tháng 5",
    "Tháng 6",
    "Tháng 7",
    "Tháng 8",
    "Tháng 9",
    "Tháng 10",
    "Tháng 11",
    "Tháng 12",
  ];

  const today = new Date();
  let viewYear = $state(today.getFullYear());
  let viewMonth = $state(today.getMonth() + 1);
  let monthData = $state<MonthData | null>(null);
  let selectedDay = $state<DayCell | null>(null);
  let isLoading = $state(false);
  let error = $state<string | null>(null);
  let holidayFilter = $state<"all" | "major">("all");

  $effect(() => {
    loadMonth(viewMonth, viewYear);
  });

  async function loadMonth(month: number, year: number) {
    isLoading = true;
    error = null;
    try {
      const data = await invoke<MonthData>("get_month_data", { month, year });
      monthData = data;
      const todayMatch = data.days.find(
        (d) => d.day === today.getDate() && d.month === month && d.year === year,
      );
      selectedDay = todayMatch ?? data.days[0] ?? null;
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      monthData = null;
      selectedDay = null;
    } finally {
      isLoading = false;
    }
  }

  async function selectDay(day: DayCell) {
    try {
      const detail = await invoke<DayCell>("get_day_detail", {
        day: day.day,
        month: day.month,
        year: day.year,
      });
      selectedDay = detail;
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
    }
  }

  function filterHolidays(holidays: HolidayInfo[]) {
    return holidayFilter === "all"
      ? holidays
      : holidays.filter((holiday) => holiday.is_major);
  }

  function cycleHolidayFilter() {
    holidayFilter = holidayFilter === "all" ? "major" : "all";
  }

  function prevMonth() {
    if (viewMonth === 1) {
      viewMonth = 12;
      viewYear -= 1;
    } else {
      viewMonth -= 1;
    }
  }

  function nextMonth() {
    if (viewMonth === 12) {
      viewMonth = 1;
      viewYear += 1;
    } else {
      viewMonth += 1;
    }
  }

  function goToday() {
    viewYear = today.getFullYear();
    viewMonth = today.getMonth() + 1;
  }

  const dayRows = () => {
    if (!monthData) return [] as (DayCell | null)[][];
    const rows: (DayCell | null)[][] = [];
    let currentRow: (DayCell | null)[] = [];

    for (let i = 0; i < monthData.first_weekday; i += 1) {
      currentRow.push(null);
    }

    for (const day of monthData.days) {
      currentRow.push(day);
      if (currentRow.length === 7) {
        rows.push(currentRow);
        currentRow = [];
      }
    }

    if (currentRow.length) {
      while (currentRow.length < 7) {
        currentRow.push(null);
      }
      rows.push(currentRow);
    }

    return rows;
  };

  const monthTitle = () => `${monthNames[viewMonth - 1]} ${viewYear}`;
</script>

<div class="app-container">
  <!-- Top Navigation Bar -->
  <header class="app-header">
    <div class="brand">
      <div class="brand-mark">AL</div>
      <span class="brand-name">Âm Lịch</span>
    </div>

    <div class="month-navigator">
      <button class="icon-btn" onclick={prevMonth} aria-label="Previous Month">
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="m15 18-6-6 6-6"/></svg>
      </button>
      <span class="current-month">{monthTitle()}</span>
      <button class="icon-btn" onclick={nextMonth} aria-label="Next Month">
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="m9 18 6-6-6-6"/></svg>
      </button>
    </div>

    <div class="actions">
      <button class="action-btn secondary" onclick={goToday}>Hôm nay</button>
      <button class="action-btn {holidayFilter === 'major' ? 'active' : ''}" onclick={cycleHolidayFilter} title="Toggle Holiday View">
        {holidayFilter === "all" ? "Tất cả lễ" : "Lễ chính"}
      </button>
    </div>
  </header>

  <!-- Main Content Area -->
  <main class="main-layout">
    
    <!-- Left: Calendar Grid -->
    <section class="calendar-section">
      <div class="weekday-header">
        {#each weekLabels as label, i}
          <div class="weekday-label" class:weekend={i === 0 || i === 6}>{label}</div>
        {/each}
      </div>

      {#if isLoading}
        <div class="status-message loading">
          <div class="spinner"></div>
          <p>Đang tải dữ liệu...</p>
        </div>
      {:else if error}
        <div class="status-message error">{error}</div>
      {:else}
        <div class="calendar-grid">
          {#each dayRows() as row}
            {#each row as day}
              {#if day}
                {@const activeHolidays = filterHolidays(day.holidays)}
                <button
                  type="button"
                  class="day-card"
                  class:selected={selectedDay?.solar_date === day.solar_date}
                  class:today={
                    day.day === today.getDate() &&
                    day.month === today.getMonth() + 1 &&
                    day.year === today.getFullYear()
                  }
                  class:has-holiday={activeHolidays.length > 0}
                  onclick={() => selectDay(day)}
                >
                  <div class="day-header">
                    <span class="solar-date">{day.day}</span>
                    <div class="lunar-stack">
                      <span class="lunar-date">{day.lunar_day}</span>
                      <span class="lunar-month">/{day.lunar_month}</span>
                    </div>
                  </div>
                  
                  <div class="day-body">
                    {#if day.day === 1 || day.day === 15}
                      <div class="moon-phase">
                        {day.day === 1 ? "🌑 Sóc" : "🌕 Vọng"}
                      </div>
                    {/if}
                    
                    <div class="holiday-pills">
                      {#each activeHolidays.slice(0, 2) as holiday}
                        <div class="pill {holiday.is_major ? 'major' : 'minor'}">
                          {holiday.name}
                        </div>
                      {/each}
                      {#if activeHolidays.length > 2}
                        <div class="pill more">+{activeHolidays.length - 2}</div>
                      {/if}
                    </div>
                  </div>
                </button>
              {:else}
                <div class="day-card empty"></div>
              {/if}
            {/each}
          {/each}
        </div>
      {/if}
    </section>

    <!-- Right: Detail Sidebar -->
    <aside class="detail-panel">
      {#if selectedDay}
        <div class="detail-content">
          <!-- Header Card -->
          <div class="detail-header-card">
            <div class="detail-solar-large">{selectedDay.day}</div>
            <div class="detail-meta">
              <div class="detail-weekday">{selectedDay.day_of_week}</div>
              <div class="detail-full-date">Tháng {selectedDay.month}, {selectedDay.year}</div>
            </div>
            <div class="detail-lunar-box">
              <div class="label">Âm Lịch</div>
              <div class="value">{selectedDay.lunar_date}</div>
              <div class="year-canchi">Năm {selectedDay.canchi_year}</div>
            </div>
          </div>

          <!-- Can Chi & Tiet Khi -->
          <div class="info-group">
            <div class="info-row">
              <span class="label">Ngày</span>
              <span class="value strong">{selectedDay.canchi_day}</span>
            </div>
            <div class="info-row">
              <span class="label">Tháng</span>
              <span class="value">{selectedDay.canchi_month}</span>
            </div>
            <div class="info-row highlight-jade">
              <span class="label">Tiết Khí</span>
              <span class="value">{selectedDay.tiet_khi}</span>
            </div>
          </div>

          <!-- Holidays -->
          {#if filterHolidays(selectedDay.holidays).length}
            <div class="section-block">
              <h3 class="section-title">Sự Kiện & Lễ</h3>
              <div class="holiday-list">
                {#each filterHolidays(selectedDay.holidays) as holiday}
                  <div class="holiday-item {holiday.is_major ? 'major' : ''}">
                    <div class="h-name">{holiday.name}</div>
                    {#if holiday.description}
                      <div class="h-desc">{holiday.description}</div>
                    {/if}
                  </div>
                {/each}
              </div>
            </div>
          {/if}

          <!-- Good Hours -->
          <div class="section-block">
            <h3 class="section-title">Giờ Hoàng Đạo</h3>
            <div class="good-hours-grid">
              {#each selectedDay.good_hours as hour}
                <div class="good-hour-chip">
                  <span class="chi">{hour.hour_chi}</span>
                  <span class="range">{hour.time_range}</span>
                </div>
              {/each}
            </div>
          </div>
        </div>
      {:else}
        <div class="empty-state">
          <p>Chọn một ngày để xem chi tiết</p>
        </div>
      {/if}
    </aside>

  </main>
</div>

<style>
  @import url("https://fonts.googleapis.com/css2?family=Source+Serif+4:ital,opsz,wght@0,8..60,400;0,8..60,600;0,8..60,700;1,8..60,400&family=Inter:wght@400;500;600&display=swap");

  :global(:root) {
    /* Color Palette - Neo-Indochine Vibrant */
    --bg-base: #FDFBF7;
    --bg-gradient: radial-gradient(circle at 50% 0%, #FFF5E6 0%, #F5E6D3 100%);
    
    --primary-red: #D93025;
    --primary-red-hover: #B31F15;
    --accent-gold: #D4AF37;
    --accent-jade: #2A6E64;
    --accent-jade-light: #E0F2F1;
    
    --text-primary: #2C241B;
    --text-secondary: #6B5D4D;
    --text-tertiary: #9D8C78;
    
    --surface-white: rgba(255, 255, 255, 0.85);
    --surface-hover: rgba(255, 255, 255, 0.95);
    --border-subtle: rgba(107, 93, 77, 0.15);
    
    --shadow-soft: 0 8px 24px rgba(44, 36, 27, 0.06);
    --shadow-hover: 0 12px 32px rgba(44, 36, 27, 0.1);
    
    --font-serif: "Source Serif 4", serif;
    --font-sans: "Inter", sans-serif;
  }

  :global(body) {
    margin: 0;
    font-family: var(--font-serif);
    background: var(--bg-gradient);
    color: var(--text-primary);
    min-height: 100vh;
    overflow-x: hidden;
  }

  :global(*) {
    box-sizing: border-box;
  }

  /* App Container */
  .app-container {
    max-width: 1400px;
    margin: 0 auto;
    padding: 24px 32px;
    display: flex;
    flex-direction: column;
    height: 100vh;
    gap: 24px;
  }

  /* Header */
  .app-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 24px;
    background: var(--surface-white);
    backdrop-filter: blur(12px);
    border-radius: 100px; /* Pill shape header */
    box-shadow: var(--shadow-soft);
    border: 1px solid var(--border-subtle);
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .brand-mark {
    width: 36px;
    height: 36px;
    background: var(--primary-red);
    color: white;
    border-radius: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 700;
    font-size: 14px;
    box-shadow: 0 4px 12px rgba(217, 48, 37, 0.25);
  }

  .brand-name {
    font-size: 1.25rem;
    font-weight: 700;
    letter-spacing: -0.02em;
    color: var(--text-primary);
  }

  .month-navigator {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .current-month {
    font-size: 1.35rem;
    font-weight: 600;
    min-width: 140px;
    text-align: center;
    font-feature-settings: "tnum";
  }

  .icon-btn {
    background: transparent;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    padding: 8px;
    border-radius: 50%;
    transition: all 0.2s;
    display: flex;
  }

  .icon-btn:hover {
    background: rgba(0,0,0,0.05);
    color: var(--text-primary);
  }

  .actions {
    display: flex;
    gap: 12px;
  }

  .action-btn {
    padding: 8px 16px;
    border-radius: 99px;
    font-family: var(--font-sans);
    font-size: 0.85rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    border: 1px solid var(--border-subtle);
    background: transparent;
    color: var(--text-secondary);
  }

  .action-btn:hover {
    background: rgba(0,0,0,0.04);
    color: var(--text-primary);
  }

  .action-btn.secondary {
    background: rgba(42, 110, 100, 0.1);
    color: var(--accent-jade);
    border-color: transparent;
  }

  .action-btn.active {
    background: var(--primary-red);
    color: white;
    border-color: var(--primary-red);
    box-shadow: 0 4px 12px rgba(217, 48, 37, 0.2);
  }

  /* Main Layout */
  .main-layout {
    display: grid;
    grid-template-columns: 1fr 340px;
    gap: 32px;
    flex: 1;
    min-height: 0; /* Prevent overflow */
  }

  /* Calendar Section */
  .calendar-section {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .weekday-header {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    margin-bottom: 12px;
    padding: 0 12px;
  }

  .weekday-label {
    text-align: center;
    font-family: var(--font-sans);
    font-size: 0.8rem;
    font-weight: 600;
    color: var(--text-tertiary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .weekday-label.weekend {
    color: var(--primary-red);
    opacity: 0.7;
  }

  .calendar-grid {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    grid-template-rows: repeat(6, 1fr); /* Force exactly 6 equal rows */
    gap: 12px;
    height: 100%;
    min-height: 0; /* Important for grid nesting */
    padding: 4px; 
  }

  /* Day Card */
  .day-card {
    background: var(--surface-white);
    border: 1px solid rgba(255,255,255,0.5);
    border-radius: 16px;
    padding: 10px 12px;
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    text-align: left;
    cursor: pointer;
    transition: all 0.25s cubic-bezier(0.2, 0.8, 0.2, 1);
    position: relative;
    box-shadow: 0 2px 4px rgba(0,0,0,0.02);
    height: 100%;
    width: 100%;
    overflow: hidden; /* Clip content that overflows */
    min-height: 0; /* Allow shrinking below content size */
  }

  .day-card:hover {
    transform: translateY(-4px);
    background: var(--surface-hover);
    box-shadow: var(--shadow-hover);
    z-index: 2;
  }

  .day-card.selected {
    border: 2px solid var(--accent-jade);
    background: #fff;
  }

  .day-card.today {
    background: #FFF8F7;
  }
  
  .day-card.today::after {
    content: "";
    position: absolute;
    top: -1px; left: -1px; right: -1px; bottom: -1px;
    border-radius: 16px;
    border: 2px solid var(--primary-red);
    pointer-events: none;
  }

  .day-header {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    width: 100%;
    margin-bottom: 4px;
  }

  .solar-date {
    font-size: 1.6rem;
    font-weight: 700;
    color: var(--text-primary);
    line-height: 1;
  }

  .today .solar-date {
    color: var(--primary-red);
  }

  .lunar-stack {
    display: flex;
    align-items: baseline;
    font-family: var(--font-sans);
    font-size: 0.75rem;
    color: var(--text-tertiary);
    font-weight: 500;
  }

  .lunar-month {
    font-size: 0.65rem;
    opacity: 0.8;
  }

  .day-body {
    display: flex;
    flex-direction: column;
    gap: 6px;
    flex: 1;
    justify-content: flex-end;
  }

  .moon-phase {
    font-size: 0.7rem;
    color: var(--accent-jade);
    font-weight: 600;
    font-family: var(--font-sans);
  }

  .holiday-pills {
    display: flex;
    flex-wrap: wrap;
    gap: 4px;
  }

  .pill {
    font-family: var(--font-sans);
    font-size: 0.65rem;
    padding: 2px 6px;
    border-radius: 4px;
    font-weight: 600;
    max-width: 100%;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .pill.major {
    background: var(--primary-red);
    color: white;
  }

  .pill.minor {
    background: rgba(212, 175, 55, 0.15);
    color: #8A6D1C;
  }
  
  .pill.more {
    background: var(--text-tertiary);
    color: white;
  }

  .day-card.empty {
    background: transparent;
    border: none;
    cursor: default;
    box-shadow: none;
  }

  /* Detail Panel */
  .detail-panel {
    background: var(--surface-white);
    border-radius: 24px;
    padding: 24px;
    box-shadow: var(--shadow-soft);
    border: 1px solid var(--border-subtle);
    display: flex;
    flex-direction: column;
    overflow-y: auto;
    height: 100%;
  }

  .detail-content {
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .detail-header-card {
    text-align: center;
    border-bottom: 1px solid var(--border-subtle);
    padding-bottom: 20px;
  }

  .detail-solar-large {
    font-size: 4rem;
    font-weight: 700;
    color: var(--primary-red);
    line-height: 1;
    margin-bottom: 8px;
  }

  .detail-meta {
    font-family: var(--font-sans);
    color: var(--text-secondary);
    margin-bottom: 16px;
  }

  .detail-weekday {
    font-size: 1.1rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .detail-full-date {
    font-size: 0.9rem;
    opacity: 0.8;
  }

  .detail-lunar-box {
    background: rgba(44, 36, 27, 0.03);
    border-radius: 12px;
    padding: 12px;
  }

  .detail-lunar-box .label {
    font-size: 0.75rem;
    text-transform: uppercase;
    color: var(--text-tertiary);
    letter-spacing: 0.1em;
  }

  .detail-lunar-box .value {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--text-primary);
    margin: 4px 0;
  }
  
  .year-canchi {
    font-size: 0.85rem;
    color: var(--text-secondary);
    font-style: italic;
  }

  .info-group {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .info-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 0;
    border-bottom: 1px dashed var(--border-subtle);
  }

  .info-row .label {
    color: var(--text-secondary);
    font-size: 0.9rem;
  }

  .info-row .value {
    font-weight: 500;
    color: var(--text-primary);
  }

  .info-row .value.strong {
    font-weight: 700;
  }

  .info-row.highlight-jade .value {
    color: var(--accent-jade);
    font-weight: 700;
  }

  .section-block {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .section-title {
    font-size: 0.8rem;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--text-tertiary);
    margin: 0;
    font-weight: 700;
    border-left: 3px solid var(--accent-gold);
    padding-left: 8px;
  }

  .holiday-item {
    background: #FFF8F0;
    border-left: 3px solid var(--accent-gold);
    padding: 10px 12px;
    border-radius: 4px;
    margin-bottom: 8px;
  }

  .holiday-item.major {
    background: #FFF0F0;
    border-left-color: var(--primary-red);
  }

  .h-name {
    font-weight: 700;
    font-size: 0.95rem;
    color: var(--text-primary);
  }

  .h-desc {
    font-size: 0.8rem;
    color: var(--text-secondary);
    margin-top: 4px;
    line-height: 1.4;
  }

  .good-hours-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
  }

  .good-hour-chip {
    background: var(--accent-jade-light);
    color: var(--accent-jade);
    padding: 6px 10px;
    border-radius: 8px;
    font-size: 0.8rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
  }

  .good-hour-chip .chi {
    font-weight: 700;
  }
  
  .good-hour-chip .range {
    font-size: 0.7rem;
    opacity: 0.8;
  }

  .status-message {
    grid-column: span 7;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 300px;
    color: var(--text-tertiary);
  }

  .spinner {
    width: 40px; height: 40px;
    border: 3px solid rgba(0,0,0,0.1);
    border-top-color: var(--primary-red);
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin-bottom: 16px;
  }

  @keyframes spin { to { transform: rotate(360deg); } }

  /* Responsive Adjustments */
  @media (max-width: 1024px) {
    .main-layout {
      grid-template-columns: 1fr;
      grid-template-rows: auto auto;
    }
    
    .detail-panel {
      height: auto;
      max-height: 500px;
    }

    .calendar-grid {
      height: auto;
      overflow: visible;
    }
  }

  @media (max-width: 640px) {
    .app-container {
      padding: 16px;
    }

    .app-header {
      padding: 12px;
      flex-direction: column;
      gap: 12px;
      border-radius: 24px;
    }

    .actions {
      width: 100%;
      justify-content: space-between;
    }

    .calendar-grid {
      gap: 6px;
    }
    
    .day-card {
      min-height: 70px;
      padding: 6px;
    }

    .solar-date {
      font-size: 1.2rem;
    }
    
    .pill {
      font-size: 0.6rem;
      padding: 1px 4px;
    }
  }
</style>
