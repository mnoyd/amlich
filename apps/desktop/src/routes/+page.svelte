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
        (d) =>
          d.day === today.getDate() && d.month === month && d.year === year,
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

  function toggleSettings() {
    isSettingsOpen = !isSettingsOpen;
    if (isSettingsOpen) {
      jumpMonth = viewMonth;
      jumpYear = viewYear;
    }
  }

  function applyJump() {
    viewMonth = jumpMonth;
    viewYear = jumpYear;
    isSettingsOpen = false;
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

  const ZODIAC_HOURS = [
    { name: "Tý", time: "23:00-01:00" },
    { name: "Sửu", time: "01:00-03:00" },
    { name: "Dần", time: "03:00-05:00" },
    { name: "Mão", time: "05:00-07:00" },
    { name: "Thìn", time: "07:00-09:00" },
    { name: "Tỵ", time: "09:00-11:00" },
    { name: "Ngọ", time: "11:00-13:00" },
    { name: "Mùi", time: "13:00-15:00" },
    { name: "Thân", time: "15:00-17:00" },
    { name: "Dậu", time: "17:00-19:00" },
    { name: "Tuất", time: "19:00-21:00" },
    { name: "Hợi", time: "21:00-23:00" },
  ];

  let hoveredZodiac = $state<{ name: string; time: string } | null>(null);
  let currentHandRotation = $state(0);

  // Settings & Menu State
  let isSettingsOpen = $state(false);
  let jumpMonth = $state(viewMonth);
  let jumpYear = $state(viewYear);

  $effect(() => {
    const updateHand = () => {
      const now = new Date();
      const hours = now.getHours();
      const minutes = now.getMinutes();
      // 00:00 (Midnight) is aligned with Tý (Top/0deg)
      // Map 24h to 360deg
      const totalMinutes = hours * 60 + minutes;
      currentHandRotation = (totalMinutes / 1440) * 360;
    };

    updateHand();
    const interval = setInterval(updateHand, 60000); // Update every minute
    return () => clearInterval(interval);
  });
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
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="20"
          height="20"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2.5"
          stroke-linecap="round"
          stroke-linejoin="round"><path d="m15 18-6-6 6-6" /></svg
        >
      </button>
      <span class="current-month">{monthTitle()}</span>
      <button class="icon-btn" onclick={nextMonth} aria-label="Next Month">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="20"
          height="20"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2.5"
          stroke-linecap="round"
          stroke-linejoin="round"><path d="m9 18 6-6-6-6" /></svg
        >
      </button>
    </div>

    <div class="actions">
      <button class="action-btn secondary" onclick={goToday}>Hôm nay</button>

      <!-- Settings Toggle -->
      <div class="settings-wrapper">
        <button
          class="icon-btn settings-btn {isSettingsOpen ? 'active' : ''}"
          onclick={toggleSettings}
          aria-label="Settings"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="20"
            height="20"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            ><path
              d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.38a2 2 0 0 0-.73-2.73l-.15-.1a2 2 0 0 1-1-1.72v-.51a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"
            /><circle cx="12" cy="12" r="3" /></svg
          >
        </button>

        <!-- Popover Menu -->
        {#if isSettingsOpen}
          <div class="settings-popover">
            <div class="popover-header">Cài đặt</div>

            <div class="popover-row">
              <div class="popover-label">
                <span>Lễ hội chính</span>
                <span class="sub-label">Chỉ hiện ngày lễ lớn</span>
              </div>
              <button
                class="toggle-switch {holidayFilter === 'major'
                  ? 'checked'
                  : ''}"
                onclick={cycleHolidayFilter}
                aria-label="Toggle Holiday Filter"
              >
                <div class="toggle-thumb"></div>
              </button>
            </div>

            <div class="popover-divider"></div>

            <div class="popover-section">
              <div class="popover-label">Đi tới ngày</div>
              <div class="jump-inputs">
                <div class="input-group">
                  <label for="jump-month">Tháng</label>
                  <input
                    id="jump-month"
                    type="number"
                    min="1"
                    max="12"
                    bind:value={jumpMonth}
                    class="jump-input"
                  />
                </div>
                <div class="input-group">
                  <label for="jump-year">Năm</label>
                  <input
                    id="jump-year"
                    type="number"
                    min="1900"
                    max="2100"
                    bind:value={jumpYear}
                    class="jump-input"
                  />
                </div>
              </div>
              <button class="apply-btn" onclick={applyJump}>Xác nhận</button>
            </div>
          </div>
        {/if}
      </div>
    </div>
  </header>

  <!-- Main Content Area -->
  <main class="main-layout">
    <!-- Left: Calendar Grid -->
    <section class="calendar-section">
      <div class="weekday-header">
        {#each weekLabels as label, i}
          <div class="weekday-label" class:weekend={i === 0 || i === 6}>
            {label}
          </div>
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
                  class:today={day.day === today.getDate() &&
                    day.month === today.getMonth() + 1 &&
                    day.year === today.getFullYear()}
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
                        <div
                          class="pill {holiday.is_major ? 'major' : 'minor'}"
                        >
                          {holiday.name}
                        </div>
                      {/each}
                      {#if activeHolidays.length > 2}
                        <div class="pill more">
                          +{activeHolidays.length - 2}
                        </div>
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
          <!-- Header Card -->
          <div class="detail-header-card">
            <div class="header-main-row">
              <div class="detail-solar-large">{selectedDay.day}</div>
              <div class="detail-right-col">
                <div class="detail-weekday">{selectedDay.day_of_week}</div>
                <div class="detail-full-date">
                  Tháng {selectedDay.month}, {selectedDay.year}
                </div>
                <div class="detail-lunar-line">
                  <span class="lunar-tag">Âm:</span>
                  <span class="lunar-val">{selectedDay.lunar_date}</span>
                  <span class="lunar-year">{selectedDay.canchi_year}</span>
                </div>
              </div>
            </div>
          </div>

          <!-- Good Hours (Living Compass) - Moved Up -->
          <div class="section-block">
            <div class="zodiac-clock-container">
              <div class="zodiac-clock">
                <!-- Living Hand (Current Time) -->
                <div
                  class="clock-hand"
                  style="transform: rotate({currentHandRotation}deg);"
                ></div>

                <!-- Center Info -->
                <div class="clock-center">
                  {#if hoveredZodiac}
                    <div class="center-label">{hoveredZodiac.name}</div>
                    <div class="center-time">{hoveredZodiac.time}</div>
                  {:else}
                    <div class="center-label">Hiện tại</div>
                    <div class="center-time">
                      {new Date().toLocaleTimeString("vi-VN", {
                        hour: "2-digit",
                        minute: "2-digit",
                      })}
                    </div>
                  {/if}
                </div>

                <!-- Clock Segments -->
                {#each ZODIAC_HOURS as zodiac, i}
                  {@const isGood = selectedDay.good_hours.some((h) =>
                    h.hour_chi.includes(zodiac.name),
                  )}
                  {@const rotation = i * 30}
                  <div
                    class="clock-segment-wrapper"
                    style="transform: rotate({rotation}deg);"
                  >
                    <button
                      class="clock-segment {isGood ? 'good' : ''}"
                      onmouseenter={() => (hoveredZodiac = zodiac)}
                      onmouseleave={() => (hoveredZodiac = null)}
                    >
                      <!-- Wedge Shape for Good Hours -->
                      <div class="segment-shape"></div>

                      <!-- Text: Counter-rotated to stay upright -->
                      <span
                        class="segment-text"
                        style="transform: rotate({-rotation}deg)"
                      >
                        {zodiac.name}
                      </span>
                    </button>
                  </div>
                {/each}
              </div>
            </div>
          </div>

          <!-- Can Chi & Tiet Khi (Compact 3 Cols) -->
          <div class="info-group-compact">
            <div class="info-col">
              <span class="label">Ngày</span>
              <span class="value strong">{selectedDay.canchi_day}</span>
            </div>
            <div class="divider"></div>
            <div class="info-col">
              <span class="label">Tháng</span>
              <span class="value">{selectedDay.canchi_month}</span>
            </div>
            <div class="divider"></div>
            <div class="info-col highlight-jade">
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
    font-size: 1.5rem;
    font-weight: 700;
    min-width: 140px;
    text-align: center;
    font-feature-settings: "tnum";
    color: var(--primary-red);
    letter-spacing: -0.01em;
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
    background: rgba(0, 0, 0, 0.05);
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
    background: rgba(0, 0, 0, 0.04);
    color: var(--text-primary);
  }

  .action-btn.secondary {
    background: rgba(42, 110, 100, 0.1);
    color: var(--accent-jade);
    border-color: transparent;
  }

  /* Settings Menu */
  .settings-wrapper {
    position: relative;
  }

  .settings-btn.active {
    background: var(--surface-hover);
    color: var(--primary-red);
  }

  .settings-popover {
    position: absolute;
    top: 140%;
    right: 0;
    width: 280px;
    background: #fff;
    border: 1px solid var(--border-subtle);
    border-radius: 16px;
    box-shadow: 0 16px 48px rgba(0, 0, 0, 0.12);
    padding: 16px;
    z-index: 100;
    animation: popIn 0.2s cubic-bezier(0.2, 0.8, 0.2, 1);
  }

  @keyframes popIn {
    from {
      opacity: 0;
      transform: translateY(-8px) scale(0.96);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  .popover-header {
    font-size: 0.95rem;
    font-weight: 700;
    color: var(--text-primary);
    margin-bottom: 16px;
    padding-bottom: 8px;
    border-bottom: 1px solid var(--border-subtle);
  }

  .popover-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 4px 0;
  }

  .popover-label {
    display: flex;
    flex-direction: column;
    font-size: 0.9rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .sub-label {
    font-size: 0.75rem;
    color: var(--text-tertiary);
    font-weight: 400;
    margin-top: 2px;
  }

  /* Toggle Switch */
  .toggle-switch {
    width: 44px;
    height: 24px;
    background: #e0e0e0;
    border-radius: 99px;
    position: relative;
    cursor: pointer;
    border: none;
    padding: 2px;
    transition: all 0.3s;
  }

  .toggle-switch.checked {
    background: var(--primary-red);
  }

  .toggle-thumb {
    width: 20px;
    height: 20px;
    background: white;
    border-radius: 50%;
    transition: transform 0.3s cubic-bezier(0.2, 0.8, 0.2, 1);
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }

  .toggle-switch.checked .toggle-thumb {
    transform: translateX(20px);
  }

  .popover-divider {
    height: 1px;
    background: var(--border-subtle);
    margin: 16px 0;
  }

  .popover-section {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .jump-inputs {
    display: flex;
    gap: 12px;
  }

  .input-group {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .input-group label {
    font-size: 0.75rem;
    color: var(--text-tertiary);
    font-weight: 500;
  }

  .jump-input {
    width: 100%;
    padding: 10px;
    border: 1px solid var(--border-subtle);
    border-radius: 8px;
    font-family: var(--font-sans);
    text-align: center;
    background: #fafafa;
    font-size: 0.95rem;
    font-weight: 500;
    color: var(--text-primary);
    transition: all 0.2s;
  }

  .jump-input:focus {
    outline: none;
    border-color: var(--accent-jade);
    background: white;
    box-shadow: 0 0 0 3px var(--accent-jade-light);
  }

  .apply-btn {
    width: 100%;
    padding: 10px;
    background: var(--accent-jade);
    color: white;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    font-family: var(--font-sans);
    font-weight: 600;
    font-size: 0.9rem;
    transition: all 0.2s;
    margin-top: 4px;
  }

  .apply-btn:hover {
    background: #235c53;
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(42, 110, 100, 0.2);
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
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--text-tertiary);
    text-transform: uppercase;
    letter-spacing: 0.1em;
    padding-bottom: 4px;
    border-bottom: 1px solid transparent;
  }

  .weekday-label.weekend {
    color: var(--primary-red);
    opacity: 0.8;
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
    border: 1px solid rgba(255, 255, 255, 0.5);
    border-radius: 16px;
    padding: 10px 12px;
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    text-align: left;
    cursor: pointer;
    transition: all 0.25s cubic-bezier(0.2, 0.8, 0.2, 1);
    position: relative;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.02);
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

  .day-card:active {
    transform: scale(0.98);
  }

  .day-card.selected {
    border: 2px solid var(--accent-jade);
    background: #fff;
    box-shadow: 0 4px 12px rgba(42, 110, 100, 0.15);
  }

  .day-card.today {
    background: #fff8f7;
  }

  .day-card.today::after {
    content: "";
    position: absolute;
    top: -1px;
    left: -1px;
    right: -1px;
    bottom: -1px;
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
    color: #8a6d1c;
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
  /* Detail Panel */
  .detail-panel {
    background: var(--surface-white);
    border-radius: 24px;
    padding: 16px;
    box-shadow: var(--shadow-soft);
    border: 1px solid var(--border-subtle);
    display: flex;
    flex-direction: column;
    overflow-y: auto;
    height: auto;
    max-height: 100%;
    align-self: start;
  }

  .detail-content {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .detail-header-card {
    border-bottom: 1px solid var(--border-subtle);
    padding-bottom: 12px;
  }

  .header-main-row {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .detail-solar-large {
    font-size: 3.5rem;
    font-weight: 700;
    color: var(--primary-red);
    line-height: 1;
  }

  .detail-right-col {
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: left;
  }

  .detail-weekday {
    font-size: 1rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    line-height: 1.2;
  }

  .detail-full-date {
    font-size: 0.85rem;
    opacity: 0.8;
    margin-bottom: 4px;
  }

  .detail-lunar-line {
    display: flex;
    align-items: baseline;
    gap: 6px;
    font-size: 0.85rem;
    color: var(--text-secondary);
  }

  .lunar-tag {
    font-size: 0.7rem;
    text-transform: uppercase;
    color: var(--text-tertiary);
    font-weight: 600;
  }

  .lunar-val {
    font-weight: 600;
    color: var(--text-primary);
  }

  .lunar-year {
    font-style: italic;
    opacity: 0.8;
    font-size: 0.8rem;
  }

  .info-group-compact {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    background: rgba(0, 0, 0, 0.02);
    border-radius: 12px;
    padding: 10px 12px;
    border: 1px solid var(--border-subtle);
  }

  .info-col {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    flex: 1;
    gap: 4px;
  }

  .divider {
    width: 1px;
    height: 24px;
    background: var(--border-subtle);
    margin: 4px 8px;
  }

  .info-col .label {
    font-size: 0.7rem;
    color: var(--text-tertiary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    font-weight: 600;
  }

  .info-col .value {
    font-size: 0.9rem;
    font-weight: 600;
    color: var(--text-primary);
    line-height: 1.2;
  }

  .info-col.highlight-jade .value {
    color: var(--accent-jade);
    font-weight: 700;
  }

  .section-block {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .section-title {
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.15em;
    color: var(--text-tertiary);
    margin: 0;
    font-weight: 700;
    border-left: 3px solid var(--accent-gold);
    padding-left: 10px;
    font-family: var(--font-sans);
  }

  .holiday-item {
    background: #fff8f0;
    border-left: 3px solid var(--accent-gold);
    padding: 8px 10px;
    border-radius: 4px;
    margin-bottom: 6px;
  }

  .holiday-item.major {
    background: #fff0f0;
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

  /* Zodiac Clock */
  .zodiac-clock-container {
    display: flex;
    justify-content: center;
    padding: 0;
    margin-bottom: 8px;
  }

  .zodiac-clock {
    position: relative;
    width: 160px;
    height: 160px;
    border-radius: 50%;
    border: 1px solid rgba(212, 175, 55, 0.3);
    background: radial-gradient(
      circle,
      #fff 50%,
      rgba(255, 248, 240, 0.8) 100%
    );
    box-shadow:
      inset 0 0 20px rgba(212, 175, 55, 0.1),
      0 8px 24px rgba(0, 0, 0, 0.08);
  }

  .clock-center {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    text-align: center;
    width: 60px;
    height: 60px;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.9);
    backdrop-filter: blur(4px);
    z-index: 10;
    pointer-events: none;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.5);
  }

  .center-label {
    font-size: 1.1rem;
    font-weight: 700;
    color: var(--primary-red);
    font-family: var(--font-serif);
    line-height: 1.1;
  }

  .center-time {
    font-size: 0.75rem;
    color: var(--text-secondary);
    font-family: var(--font-sans);
    margin-top: 4px;
    font-variant-numeric: tabular-nums;
  }

  /* Clock Hand */
  .clock-hand {
    position: absolute;
    top: 0;
    left: 50%;
    width: 2px;
    height: 50%;
    background: linear-gradient(
      to top,
      var(--primary-red) 50%,
      transparent 100%
    ); /* Fade tip */
    transform-origin: bottom center;
    z-index: 5;
    pointer-events: none;
  }

  .clock-hand::after {
    content: "";
    position: absolute;
    top: 0;
    left: -3px;
    width: 8px;
    height: 8px;
    background: var(--primary-red);
    border-radius: 50%;
    box-shadow: 0 0 4px rgba(217, 48, 37, 0.4);
  }

  /* Segments */
  .clock-segment-wrapper {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
  }

  .clock-segment {
    position: absolute;
    top: 0;
    left: 50%;
    transform: translateX(-50%);
    width: 40px; /* Wider hit area */
    height: 50%;
    background: transparent;
    border: none;
    cursor: pointer;
    pointer-events: auto;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding-top: 8px;
    transition: all 0.2s;
  }

  /* The Wedge Shape */
  .segment-shape {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: rgba(0, 0, 0, 0.1);
    margin-bottom: 4px;
    transition: all 0.3s;
  }

  .segment-text {
    font-size: 0.7rem;
    font-weight: 600;
    color: var(--text-tertiary);
    transition: all 0.3s;
    /* Important: Text is upright due to inline style, just need positioning */
    margin-top: 2px;
  }

  /* Active/Good State */
  .clock-segment.good .segment-shape {
    background: var(--accent-jade);
    box-shadow: 0 0 8px rgba(42, 110, 100, 0.4);
    transform: scale(1.4);
  }

  .clock-segment.good .segment-text {
    color: var(--accent-jade);
    font-weight: 700;
  }

  /* Hover State */
  .clock-segment:hover .segment-shape {
    background: var(--primary-red);
    transform: scale(1.2);
  }

  .clock-segment:hover .segment-text {
    color: var(--primary-red);
    font-weight: 700;
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
    width: 40px;
    height: 40px;
    border: 3px solid rgba(0, 0, 0, 0.1);
    border-top-color: var(--primary-red);
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin-bottom: 16px;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

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
