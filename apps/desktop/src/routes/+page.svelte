<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import DateInsightBox from "$lib/components/DateInsightBox.svelte";
  import SettingsPanel from "$lib/components/SettingsPanel.svelte";

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
  let isSettingsOpen = $state(false);

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
    return holidays;
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

  function handleKeydown(e: KeyboardEvent) {
    // Don't handle shortcuts when typing in inputs
    if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) {
      return;
    }
    
    switch (e.key) {
      case "ArrowLeft":
        prevMonth();
        break;
      case "ArrowRight":
        nextMonth();
        break;
      case "t":
      case "T":
        goToday();
        break;
      case "?":
        isSettingsOpen = true;
        break;
    }
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

  // Cultural Detail Visibility
  let isInsightVisible = $state(false);

  function toggleInsight() {
    isInsightVisible = !isInsightVisible;
  }

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

<svelte:window onkeydown={handleKeydown} />

<div class="app-container">
  <!-- Top Navigation Bar -->
  <header class="app-header">
    <div class="brand">
      <div class="brand-mark">
        <img class="brand-logo" src="/amlich-logo.svg" alt="Amlich logo" />
      </div>
      <span class="brand-name">Âm Lịch</span>
    </div>

    <div class="month-navigator">
      <button
        class="icon-btn nav-arrow"
        onclick={prevMonth}
        aria-label="Previous Month"
      >
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
      <div class="month-pill">
        <span class="month-caption">Lịch tháng</span>
        <span class="current-month">{monthTitle()}</span>
      </div>
      <button
        class="icon-btn nav-arrow"
        onclick={nextMonth}
        aria-label="Next Month"
      >
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
      <button
        class="action-btn insight-toggle {isInsightVisible ? 'active' : ''}"
        onclick={toggleInsight}
        disabled={!selectedDay}
        aria-pressed={isInsightVisible}
      >
        {isInsightVisible ? "Ẩn văn hóa" : "Xem văn hóa"}
      </button>
      <button
        class="icon-btn settings-btn"
        onclick={() => (isSettingsOpen = true)}
        aria-label="Cài đặt"
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
        >
          <path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" />
          <circle cx="12" cy="12" r="3" />
        </svg>
      </button>
    </div>
  </header>

  <!-- Main Content Area -->
  <main class="main-layout" class:insight-mode={isInsightVisible}>
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

    <!-- Right: Detail / Cultural Panel -->
    <aside class={isInsightVisible ? "focus-panel" : "detail-panel"}>
      {#if selectedDay}
        {#if isInsightVisible}
          <div class="focus-content">
            <div class="focus-header-card">
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

            <div class="focus-meta">
              <div class="focus-meta-item">
                <span class="label">Can Chi ngày</span>
                <span class="value strong">{selectedDay.canchi_day}</span>
              </div>
              <div class="focus-meta-item">
                <span class="label">Can Chi tháng</span>
                <span class="value">{selectedDay.canchi_month}</span>
              </div>
              <div class="focus-meta-item">
                <span class="label">Tiết Khí</span>
                <span class="value">{selectedDay.tiet_khi}</span>
              </div>
            </div>

            <div class="focus-insight">
              <DateInsightBox day={selectedDay} />
            </div>
          </div>
        {:else}
          <div class="detail-content">
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

            <div class="section-block">
              <div class="zodiac-clock-container">
                <div class="zodiac-clock">
                  <div
                    class="clock-hand"
                    style="transform: rotate({currentHandRotation}deg);"
                  ></div>

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
                        <div class="segment-shape"></div>
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
        {/if}
      {:else}
        <div class="empty-state">
          <p>Chọn một ngày để xem chi tiết</p>
        </div>
      {/if}
    </aside>
  </main>
</div>

<!-- Settings Panel -->
<SettingsPanel bind:open={isSettingsOpen} />

<style>
  /* App Container */
  .app-container {
    max-width: 1400px;
    margin: 0 auto;
    padding: 24px 32px;
    display: flex;
    flex-direction: column;
    min-height: 100vh;
    gap: 24px;
    user-select: none;
    -webkit-user-select: none;
  }

  /* Header */
  .app-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 14px 20px;
    background:
      radial-gradient(
        120% 200% at 0% 0%,
        rgba(212, 175, 55, 0.13) 0%,
        rgba(212, 175, 55, 0) 52%
      ),
      linear-gradient(120deg, #fffefb 0%, #fef8f1 100%);
    border-radius: 24px;
    box-shadow: 0 12px 28px rgba(64, 43, 14, 0.1);
    border: 1px solid rgba(212, 175, 55, 0.34);
    position: relative;
    overflow: visible;
    gap: 16px;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .brand-mark {
    width: 40px;
    height: 40px;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    overflow: hidden;
  }

  .brand-logo {
    width: 100%;
    height: 100%;
    display: block;
  }

  .brand-name {
    font-size: 1.18rem;
    font-weight: 700;
    letter-spacing: -0.01em;
    color: var(--text-primary);
  }

  .month-navigator {
    display: flex;
    align-items: center;
    gap: 10px;
    flex: 1;
    justify-content: center;
    min-width: 0;
  }

  .month-pill {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-width: 220px;
    padding: 7px 16px 8px;
    border-radius: 14px;
    border: 1px solid rgba(212, 175, 55, 0.46);
    background:
      linear-gradient(180deg, rgba(255, 255, 255, 0.95) 0%, #fff8ef 100%);
    box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.9);
  }

  .month-caption {
    font-size: 0.62rem;
    font-weight: 700;
    letter-spacing: 0.12em;
    text-transform: uppercase;
    color: #a28043;
    margin-bottom: 3px;
  }

  .current-month {
    font-size: 1.28rem;
    font-weight: 700;
    min-width: 0;
    text-align: center;
    font-feature-settings: "tnum";
    color: #8d251a;
    letter-spacing: -0.01em;
    line-height: 1.15;
  }

  .icon-btn {
    background: transparent;
    border: 1px solid transparent;
    color: var(--text-secondary);
    cursor: pointer;
    width: 36px;
    height: 36px;
    padding: 0;
    border-radius: 10px;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .icon-btn:hover {
    background: rgba(250, 240, 226, 0.95);
    border-color: rgba(212, 175, 55, 0.35);
    color: var(--text-primary);
  }

  .nav-arrow {
    background: rgba(255, 255, 255, 0.85);
    border-color: rgba(212, 175, 55, 0.3);
    color: #8f4e1f;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
  }

  .nav-arrow:hover {
    color: var(--primary-red);
    transform: translateY(-1px);
    box-shadow: 0 4px 10px rgba(217, 48, 37, 0.14);
  }

  .actions {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-left: auto;
  }

  .action-btn {
    padding: 8px 12px;
    border-radius: 99px;
    font-family: var(--font-sans);
    font-size: 0.8rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    cursor: pointer;
    transition: all 0.2s;
    border: 1px solid var(--border-subtle);
    background: rgba(255, 255, 255, 0.9);
    color: var(--text-secondary);
  }

  .action-btn:hover {
    background: #fff;
    color: var(--text-primary);
    transform: translateY(-1px);
  }

  .action-btn.secondary {
    background: linear-gradient(
      140deg,
      rgba(42, 110, 100, 0.14) 0%,
      rgba(42, 110, 100, 0.23) 100%
    );
    color: #1f5f55;
    border-color: rgba(42, 110, 100, 0.24);
    box-shadow: 0 5px 12px rgba(42, 110, 100, 0.16);
  }

  .settings-btn {
    background: rgba(255, 255, 255, 0.85);
    border-color: rgba(212, 175, 55, 0.3);
    color: var(--text-secondary);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
  }

  .settings-btn:hover {
    color: var(--accent-jade);
    background: rgba(255, 255, 255, 0.95);
    border-color: rgba(42, 110, 100, 0.3);
    transform: translateY(-1px);
    box-shadow: 0 4px 10px rgba(42, 110, 100, 0.14);
  }

  .action-btn.insight-toggle.active {
    background: linear-gradient(
      140deg,
      rgba(212, 175, 55, 0.16) 0%,
      rgba(212, 175, 55, 0.26) 100%
    );
    border-color: rgba(160, 126, 52, 0.28);
    color: #7a5120;
    box-shadow: 0 5px 12px rgba(160, 126, 52, 0.14);
  }

  .action-btn.insight-toggle:disabled {
    opacity: 0.45;
    cursor: not-allowed;
    transform: none;
    box-shadow: none;
  }

  /* Main Layout */
  .main-layout {
    display: grid;
    grid-template-columns: 1fr 340px;
    gap: 32px;
    min-height: 0; /* Prevent overflow */
  }

  .main-layout.insight-mode {
    grid-template-columns: 210px minmax(0, 1fr);
    gap: 16px;
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
    grid-auto-rows: minmax(110px, auto);
    gap: 12px;
    height: auto;
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
    height: auto;
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

  .main-layout.insight-mode .weekday-header {
    margin-bottom: 6px;
    padding: 0 2px;
  }

  .main-layout.insight-mode .weekday-label {
    font-size: 0.56rem;
    letter-spacing: 0.02em;
    font-weight: 700;
    padding-bottom: 2px;
  }

  .main-layout.insight-mode .calendar-grid {
    grid-auto-rows: 34px;
    gap: 3px;
    padding: 0;
    height: auto;
  }

  .main-layout.insight-mode .day-card {
    min-height: 34px;
    border-radius: 8px;
    padding: 3px 4px;
    box-shadow: none;
    border: 1px solid rgba(255, 255, 255, 0.6);
    transition: border-color 0.15s ease;
  }

  .main-layout.insight-mode .day-header {
    margin-bottom: 0;
    align-items: center;
    justify-content: flex-start;
    gap: 2px;
  }

  .main-layout.insight-mode .solar-date {
    font-size: 0.94rem;
    line-height: 1;
  }

  .main-layout.insight-mode .lunar-stack {
    display: none;
  }

  .main-layout.insight-mode .day-body {
    display: none;
  }

  .main-layout.insight-mode .day-card:hover {
    transform: none;
    box-shadow: none;
    background: var(--surface-white);
    border-color: rgba(212, 175, 55, 0.5);
  }

  .main-layout.insight-mode .day-card.selected {
    border-width: 2px;
    box-shadow: none;
  }

  .main-layout.insight-mode .day-card.today::after {
    border-width: 1.5px;
    border-radius: 8px;
  }

  .main-layout.insight-mode .calendar-section {
    background: rgba(255, 255, 255, 0.5);
    border: 1px solid rgba(212, 175, 55, 0.2);
    border-radius: 14px;
    padding: 8px;
    align-self: start;
    height: auto;
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

  .focus-panel {
    background: var(--surface-white);
    border-radius: 24px;
    padding: 14px;
    box-shadow: var(--shadow-soft);
    border: 1px solid var(--border-subtle);
    display: flex;
    flex-direction: column;
    overflow-y: auto;
    height: auto;
    max-height: 100%;
    align-self: start;
    min-width: 0;
  }

  .focus-content {
    display: flex;
    flex-direction: column;
    gap: 12px;
    min-width: 0;
  }

  .focus-header-card {
    background: linear-gradient(135deg, rgba(255, 255, 255, 0.98) 0%, #fff7ed 100%);
    border: 1px solid rgba(212, 175, 55, 0.26);
    border-radius: 16px;
    padding: 10px 12px;
  }

  .focus-meta {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 10px;
  }

  .focus-meta-item {
    background: rgba(255, 255, 255, 0.86);
    border: 1px solid rgba(212, 175, 55, 0.22);
    border-radius: 12px;
    padding: 10px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .focus-meta-item .label {
    font-size: 0.68rem;
  }

  .focus-meta-item .value {
    font-size: 0.88rem;
    line-height: 1.35;
  }

  .focus-insight {
    min-width: 0;
  }

  .focus-insight :global(.insight-box) {
    border-radius: 16px;
    border: 1px solid rgba(212, 175, 55, 0.22);
    box-shadow: none;
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
    padding: 4px 0;
    margin-bottom: 10px;
  }

  .zodiac-clock {
    position: relative;
    width: 220px;
    height: 220px;
    border-radius: 50%;
    border: 1px solid rgba(212, 175, 55, 0.45);
    background: radial-gradient(
      circle,
      #ffffff 0%,
      #fff9f1 62%,
      #f8ede1 100%
    );
    box-shadow:
      inset 0 2px 18px rgba(255, 255, 255, 0.95),
      inset 0 -12px 26px rgba(212, 175, 55, 0.16),
      0 16px 30px rgba(0, 0, 0, 0.12);
  }

  .zodiac-clock::before {
    content: "";
    position: absolute;
    inset: 10px;
    border-radius: 50%;
    border: 1px dashed rgba(160, 126, 52, 0.3);
    pointer-events: none;
  }

  .clock-center {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    text-align: center;
    width: 86px;
    height: 86px;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    border-radius: 50%;
    background: linear-gradient(160deg, #ffffff 0%, #f9f1e6 100%);
    z-index: 10;
    pointer-events: none;
    box-shadow:
      0 8px 16px rgba(0, 0, 0, 0.1),
      inset 0 1px 0 rgba(255, 255, 255, 0.9);
    border: 1px solid rgba(212, 175, 55, 0.4);
  }

  .center-label {
    font-size: 1.2rem;
    font-weight: 700;
    color: var(--primary-red);
    font-family: var(--font-serif);
    line-height: 1;
  }

  .center-time {
    font-size: 0.82rem;
    color: var(--text-secondary);
    font-family: var(--font-sans);
    margin-top: 5px;
    font-weight: 600;
    letter-spacing: 0.01em;
    font-variant-numeric: tabular-nums;
  }

  /* Clock Hand */
  .clock-hand {
    position: absolute;
    top: 0;
    left: 50%;
    width: 3px;
    height: 50%;
    background: linear-gradient(
      to top,
      rgba(217, 48, 37, 0.92) 55%,
      transparent 100%
    );
    transform-origin: bottom center;
    z-index: 5;
    pointer-events: none;
  }

  .clock-hand::after {
    content: "";
    position: absolute;
    top: 0;
    left: -3.5px;
    width: 10px;
    height: 10px;
    background: var(--primary-red);
    border-radius: 50%;
    box-shadow:
      0 0 8px rgba(217, 48, 37, 0.45),
      0 0 0 2px rgba(255, 255, 255, 0.8);
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
    width: 50px;
    height: 50%;
    background: transparent;
    border: none;
    cursor: pointer;
    pointer-events: auto;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding-top: 10px;
    transition: all 0.2s;
  }

  .segment-shape {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: rgba(83, 70, 50, 0.24);
    margin-bottom: 6px;
    transition: all 0.3s;
  }

  .segment-text {
    font-size: 0.74rem;
    font-weight: 700;
    color: #7c6849;
    transition: all 0.3s;
    margin-top: 2px;
    text-shadow: 0 1px 0 rgba(255, 255, 255, 0.7);
  }

  /* Active/Good State */
  .clock-segment.good .segment-shape {
    background: var(--accent-jade);
    box-shadow: 0 0 10px rgba(42, 110, 100, 0.42);
    transform: scale(1.45);
  }

  .clock-segment.good .segment-text {
    color: var(--accent-jade);
    font-weight: 700;
  }

  /* Hover State */
  .clock-segment:hover .segment-shape {
    background: var(--primary-red);
    transform: scale(1.25);
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

    .month-navigator {
      width: 100%;
    }

    .month-pill {
      min-width: 0;
      flex: 1;
    }

    .actions {
      width: 100%;
      justify-content: center;
      flex-wrap: wrap;
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

    .zodiac-clock {
      width: 188px;
      height: 188px;
    }

    .clock-center {
      width: 74px;
      height: 74px;
    }

    .center-label {
      font-size: 1.05rem;
    }

    .center-time {
      font-size: 0.76rem;
    }

    .clock-segment {
      width: 44px;
      padding-top: 8px;
    }

    .segment-text {
      font-size: 0.68rem;
    }
  }

</style>
