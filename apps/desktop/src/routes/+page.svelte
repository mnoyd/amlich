<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { getVersion } from "@tauri-apps/api/app";
  import DateInsightBox from "$lib/components/DateInsightBox.svelte";
  import { getDayDots, getDayEventCategories, classifyHoliday } from "$lib/insights/date-insight-engine";
  import type { DayCell, HolidayInfo, MonthData } from "$lib/insights/types";
  import { checkForAppUpdates } from "$lib/updater";

  const weekLabels = ["T2", "T3", "T4", "T5", "T6", "T7", "CN"];
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

  const dotLabels: Record<string, string> = {
    festival: "Lễ truyền thống",
    "public-holiday": "Nghỉ lễ",
    commemorative: "Kỷ niệm",
    professional: "Ngành nghề",
    social: "Xã hội",
    international: "Quốc tế",
    "solar-term": "Tiết khí",
    "lunar-cycle": "Sóc/Vọng",
  };

  const today = new Date();
  let viewYear = $state(today.getFullYear());
  let viewMonth = $state(today.getMonth() + 1);
  let monthData = $state<MonthData | null>(null);
  let selectedDay = $state<DayCell | null>(null);
  let preferredDayOfMonth: number | null = null;
  let isLoading = $state(false);
  let error = $state<string | null>(null);

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
      const previousDayMatch =
        preferredDayOfMonth == null
          ? null
          : data.days.find((d) => d.day === preferredDayOfMonth) ?? null;
      selectedDay = todayMatch ?? previousDayMatch ?? data.days[0] ?? null;
      preferredDayOfMonth = selectedDay?.day ?? null;
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
      preferredDayOfMonth = detail.day;
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
    }
  }

  function prevMonth() {
    preferredDayOfMonth = selectedDay?.day ?? preferredDayOfMonth;
    if (viewMonth === 1) {
      viewMonth = 12;
      viewYear -= 1;
    } else {
      viewMonth -= 1;
    }
  }

  function nextMonth() {
    preferredDayOfMonth = selectedDay?.day ?? preferredDayOfMonth;
    if (viewMonth === 12) {
      viewMonth = 1;
      viewYear += 1;
    } else {
      viewMonth += 1;
    }
  }

  function goToday() {
    preferredDayOfMonth = today.getDate();
    viewYear = today.getFullYear();
    viewMonth = today.getMonth() + 1;
  }

  const dayRows = () => {
    if (!monthData) return [] as (DayCell | null)[][];
    const rows: (DayCell | null)[][] = [];
    let currentRow: (DayCell | null)[] = [];
    const mondayFirstOffset = (monthData.first_weekday + 6) % 7;

    for (let i = 0; i < mondayFirstOffset; i += 1) {
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
  let currentTimeStr = $state("");

  // Cultural Detail Visibility
  let isInsightVisible = $state(false);

  function toggleInsight() {
    isInsightVisible = !isInsightVisible;
  }

  // Settings menu
  let showSettingsMenu = $state(false);
  let appVersion = $state("");

  $effect(() => {
    getVersion().then((v) => (appVersion = v));
  });

  function toggleSettings() {
    showSettingsMenu = !showSettingsMenu;
  }

  function handleCheckUpdate() {
    showSettingsMenu = false;
    checkForAppUpdates(true);
  }

  function handleClickOutside(event: MouseEvent) {
    const target = event.target as HTMLElement;
    if (showSettingsMenu && !target.closest(".settings-wrapper")) {
      showSettingsMenu = false;
    }
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
      currentTimeStr = now.toLocaleTimeString("vi-VN", {
        hour: "2-digit",
        minute: "2-digit",
      });
    };

    updateHand();
    const interval = setInterval(updateHand, 60000); // Update every minute
    return () => clearInterval(interval);
  });
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="app-container" onclick={handleClickOutside}>
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
      <div class="settings-wrapper">
        <button
          class="icon-btn settings-btn"
          onclick={toggleSettings}
          aria-label="Settings"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/>
            <circle cx="12" cy="12" r="3"/>
          </svg>
        </button>
        {#if showSettingsMenu}
          <div class="settings-dropdown">
            <button class="dropdown-item" onclick={handleCheckUpdate}>
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"/>
                <path d="M21 3v5h-5"/>
              </svg>
              Kiểm tra cập nhật
            </button>
            <div class="dropdown-version">v{appVersion}</div>
          </div>
        {/if}
      </div>
    </div>
  </header>

  <!-- Main Content Area -->
  <main class="main-layout" class:insight-mode={isInsightVisible}>
    <!-- Left: Calendar Grid -->
    <section class="calendar-section">
        <div class="weekday-header">
          {#each weekLabels as label, i}
          <div class="weekday-label" class:weekend={i === 5 || i === 6}>
            {label}
          </div>
        {/each}
      </div>

      <details class="legend-bar">
        <summary class="legend-summary" aria-label="Mở chú giải màu sự kiện">
          <span class="legend-dot" style="background: var(--cat-festival);"></span>
          <span class="legend-dot" style="background: var(--cat-public-holiday);"></span>
          <span class="legend-dot" style="background: var(--cat-commemorative);"></span>
          <span class="legend-dot" style="background: var(--cat-professional);"></span>
          <span class="legend-dot" style="background: var(--cat-social);"></span>
          <span class="legend-dot" style="background: var(--cat-international);"></span>
          <span class="legend-dot" style="background: var(--cat-lunar-cycle);"></span>
        </summary>
        <div class="legend-items">
          <span class="legend-item"><span class="legend-dot" style="background: var(--cat-festival);"></span>Lễ truyền thống</span>
          <span class="legend-item"><span class="legend-dot" style="background: var(--cat-public-holiday);"></span>Nghỉ lễ</span>
          <span class="legend-item"><span class="legend-dot" style="background: var(--cat-commemorative);"></span>Kỷ niệm</span>
          <span class="legend-item"><span class="legend-dot" style="background: var(--cat-professional);"></span>Ngành nghề</span>
          <span class="legend-item"><span class="legend-dot" style="background: var(--cat-social);"></span>Xã hội</span>
          <span class="legend-item"><span class="legend-dot" style="background: var(--cat-international);"></span>Quốc tế</span>
          <span class="legend-item"><span class="legend-dot" style="background: var(--cat-lunar-cycle);"></span>Sóc/Vọng</span>
        </div>
      </details>

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
                {@const categories = getDayEventCategories(day)}
                {@const dots = getDayDots(day)}
                {@const topCat = categories.length > 0 ? categories[0] : null}
                <button
                  type="button"
                  class="day-card"
                  class:selected={selectedDay?.solar_date === day.solar_date}
                  class:today={day.day === today.getDate() &&
                    day.month === today.getMonth() + 1 &&
                    day.year === today.getFullYear()}
                  class:has-holiday={day.holidays.length > 0}
                  class:has-events={categories.length > 0}
                  style={topCat ? `--accent-color: ${topCat.colorHex}; --accent-tint: ${topCat.colorHex}0D;` : ''}
                  onclick={() => selectDay(day)}
                >
                  <div class="day-header">
                    <span class="solar-date">{day.day}</span>
                    {#if dots.length > 0}
                      <span class="category-dots">
                        {#each dots as dot}
                          <span class="cat-dot" style="background: {dot.colorHex};" title={dotLabels[dot.type] ?? dot.type}></span>
                        {/each}
                      </span>
                    {/if}
                    <div class="lunar-stack">
                      <span class="lunar-date">{day.lunar_day}</span>
                      <span class="lunar-month">/{day.lunar_month}</span>
                    </div>
                  </div>

                  <div class="day-body">
                    {#if day.lunar_day === 1 || day.lunar_day === 15 || day.lunar_day === 8 || day.lunar_day === 23}
                      <div class="moon-phase" class:new-moon={day.lunar_day === 1} class:full-moon={day.lunar_day === 15} class:first-quarter={day.lunar_day === 8} class:last-quarter={day.lunar_day === 23}>
                        <span class="moon-icon"></span>
                        {#if day.lunar_day === 1}
                          Sóc
                        {:else if day.lunar_day === 15}
                          Vọng
                        {:else if day.lunar_day === 8}
                          Thượng huyền
                        {:else}
                          Hạ huyền
                        {/if}
                      </div>
                    {/if}

                    <div class="holiday-pills">
                      {#each categories.slice(0, 2) as cat}
                        <div
                          class="pill cat-pill"
                          style="background: {cat.colorHex}; color: {cat.type === 'lunar-cycle' ? '#6B5D4D' : 'white'}; {cat.type === 'lunar-cycle' ? 'background: ' + cat.colorHex + '26;' : ''}"
                        >
                          {cat.name}
                        </div>
                      {/each}
                      {#if categories.length > 2}
                        <div class="pill more">
                          +{categories.length - 2}
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
                        {currentTimeStr}
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
                        onfocus={() => (hoveredZodiac = zodiac)}
                        onblur={() => (hoveredZodiac = null)}
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

            {#if selectedDay.holidays.length}
              <div class="section-block">
                <h3 class="section-title">Sự Kiện & Lễ</h3>
                <div class="holiday-list">
                  {#each selectedDay.holidays as holiday}
                    {@const catType = classifyHoliday(holiday, selectedDay)}
                    {@const catColors: Record<string, { bg: string; border: string }> = {
                      'festival': { bg: '#FFF0F0', border: '#C62828' },
                      'public-holiday': { bg: '#FFF0F5', border: '#AD1457' },
                      'commemorative': { bg: '#F0F4FF', border: '#1565C0' },
                      'professional': { bg: '#F0FFF0', border: '#2E7D32' },
                      'social': { bg: '#FFF8F0', border: '#E65100' },
                      'international': { bg: '#F8F0FF', border: '#6A1B9A' },
                      'solar-term': { bg: '#F0FFFD', border: '#00796B' },
                      'lunar-cycle': { bg: '#FFFCF0', border: '#D4AF37' },
                    }}
                    {@const colors = catColors[catType] || catColors['lunar-cycle']}
                    <div class="holiday-item" style="background: {colors.bg}; border-left-color: {colors.border};">
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

<style>
  /* App Container */
  .app-container {
    max-width: 1400px;
    margin: 0 auto;
    padding: 16px 24px;
    display: flex;
    flex-direction: column;
    min-height: 100vh;
    gap: 16px;
    user-select: none;
    -webkit-user-select: none;
  }

  /* Header */
  .app-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px 16px;
    background:
      radial-gradient(
        120% 200% at 0% 0%,
        rgba(212, 175, 55, 0.13) 0%,
        rgba(212, 175, 55, 0) 52%
      ),
      linear-gradient(120deg, #fffefb 0%, #fef8f1 100%);
    border-radius: 16px;
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
    width: 32px;
    height: 32px;
    border-radius: 10px;
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
    gap: 24px;
    min-height: 0;
    transition: grid-template-columns 0.3s ease;
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
    margin-bottom: 6px;
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
    border-bottom: 1px solid rgba(217, 48, 37, 0.16);
  }

  /* Legend Bar */
  .legend-bar {
    margin-bottom: 6px;
    padding: 0 12px;
  }

  .legend-summary {
    display: flex;
    align-items: center;
    gap: 4px;
    list-style: none;
    cursor: pointer;
    padding: 4px 0;
    opacity: 0.55;
    transition: opacity 0.15s;
  }

  .legend-summary:hover {
    opacity: 0.85;
  }

  .legend-summary::-webkit-details-marker {
    display: none;
  }

  .legend-bar[open] .legend-summary {
    opacity: 0.35;
  }

  .legend-items {
    display: flex;
    flex-wrap: wrap;
    gap: 4px 10px;
    margin-top: 4px;
    padding-bottom: 2px;
  }

  .legend-item {
    display: flex;
    align-items: center;
    gap: 4px;
    font-family: var(--font-sans);
    font-size: 0.62rem;
    font-weight: 600;
    color: var(--text-tertiary);
    letter-spacing: 0.02em;
    white-space: nowrap;
  }

  .legend-dot {
    display: inline-block;
    width: 6px;
    height: 6px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .legend-items .legend-dot {
    width: 7px;
    height: 7px;
  }

  .main-layout.insight-mode .legend-bar {
    display: none;
  }

  .calendar-grid {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    grid-auto-rows: minmax(78px, auto);
    gap: 6px;
    height: auto;
    min-height: 0;
    padding: 2px;
  }

  /* Day Card */
  .day-card {
    background: var(--surface-white);
    border: 1px solid rgba(255, 255, 255, 0.5);
    border-radius: 10px;
    padding: 6px 8px;
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
    overflow: hidden;
    min-height: 0;
  }

  .day-card:hover {
    transform: translateY(-2px);
    background: var(--surface-hover);
    box-shadow: 0 6px 16px rgba(44, 36, 27, 0.08);
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

  /* Event accent strip — colored left border for days with events */
  .day-card.has-events {
    border-left: 3px solid var(--accent-color, transparent);
    background: var(--accent-tint, var(--surface-white));
  }

  .day-card.has-events.selected {
    border: 2px solid var(--accent-jade);
    border-left: 3px solid var(--accent-color, var(--accent-jade));
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
    border-radius: 10px;
    border: 2px solid var(--primary-red);
    pointer-events: none;
  }

  .day-header {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    width: 100%;
    margin-bottom: 2px;
  }

  .solar-date {
    font-size: 1.25rem;
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

  /* Category indicator dots — colored by event type */
  .category-dots {
    display: inline-flex;
    align-items: center;
    gap: 2px;
    margin-left: 3px;
    margin-bottom: 2px;
    flex-shrink: 0;
  }

  .cat-dot {
    display: inline-block;
    width: 6px;
    height: 6px;
    border-radius: 50%;
    flex-shrink: 0;
    opacity: 0.8;
    transition: opacity 0.2s, transform 0.2s;
  }

  .day-card:hover .cat-dot {
    opacity: 1;
    transform: scale(1.15);
  }

  .main-layout.insight-mode .category-dots {
    gap: 1px;
    margin-left: 2px;
    margin-bottom: 1px;
  }

  .main-layout.insight-mode .cat-dot {
    width: 4px;
    height: 4px;
  }

  .day-body {
    display: flex;
    flex-direction: column;
    gap: 3px;
    flex: 1;
    justify-content: flex-end;
  }

  .moon-phase {
    font-size: 0.62rem;
    color: var(--accent-jade);
    font-weight: 600;
    font-family: var(--font-sans);
    display: flex;
    align-items: center;
    gap: 3px;
  }

  .moon-icon {
    display: inline-block;
    width: 8px;
    height: 8px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  /* New moon (Sóc) — dark circle */
  .moon-phase.new-moon .moon-icon {
    background: var(--text-primary);
    box-shadow: 0 0 0 1px rgba(0, 0, 0, 0.1);
  }

  /* Full moon (Vọng) — bright gold circle */
  .moon-phase.full-moon .moon-icon {
    background: var(--accent-gold);
    box-shadow: 0 0 4px rgba(212, 175, 55, 0.5);
  }

  .moon-phase.full-moon {
    color: #8a6d1c;
  }

  /* First quarter (Thượng Huyền) — half light right */
  .moon-phase.first-quarter .moon-icon {
    background: linear-gradient(90deg, var(--text-primary) 50%, var(--accent-gold) 50%);
  }

  .moon-phase.first-quarter {
    color: var(--text-tertiary);
  }

  /* Last quarter (Hạ Huyền) — half light left */
  .moon-phase.last-quarter .moon-icon {
    background: linear-gradient(90deg, var(--accent-gold) 50%, var(--text-primary) 50%);
  }

  .moon-phase.last-quarter {
    color: var(--text-tertiary);
  }

  .holiday-pills {
    display: flex;
    flex-wrap: wrap;
    gap: 2px;
  }

  .pill {
    font-family: var(--font-sans);
    font-size: 0.6rem;
    padding: 1px 5px;
    border-radius: 4px;
    font-weight: 600;
    max-width: 100%;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .pill.cat-pill {
    /* Colors set via inline style from category */
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
    transition: opacity 0.2s;
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

  .main-layout.insight-mode .day-card.has-events {
    border-left: 2px solid var(--accent-color, transparent);
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
  .detail-panel {
    background: var(--surface-white);
    border-radius: 16px;
    padding: 14px;
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
    gap: 12px;
  }

  .focus-panel {
    background: var(--surface-white);
    border-radius: 16px;
    padding: 10px;
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
    gap: 8px;
    min-width: 0;
  }

  .focus-header-card {
    background: linear-gradient(135deg, rgba(255, 255, 255, 0.98) 0%, #fff7ed 100%);
    border: 1px solid rgba(212, 175, 55, 0.26);
    border-radius: 10px;
    padding: 8px 10px;
  }

  .focus-meta {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 6px;
  }

  .focus-meta-item {
    background: rgba(255, 255, 255, 0.86);
    border: 1px solid rgba(212, 175, 55, 0.22);
    border-radius: 8px;
    padding: 6px 8px;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .focus-meta-item .label {
    font-size: 0.62rem;
  }

  .focus-meta-item .value {
    font-size: 0.82rem;
    line-height: 1.3;
  }

  .focus-insight {
    min-width: 0;
  }

  .focus-insight :global(.insight-box) {
    border-radius: 10px;
    border: 1px solid rgba(212, 175, 55, 0.22);
    box-shadow: none;
  }

  .detail-header-card {
    background: rgba(0, 0, 0, 0.015);
    border: 1px solid var(--border-subtle);
    border-radius: 10px;
    padding: 10px 12px;
  }

  .header-main-row {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .detail-solar-large {
    font-size: 3.2rem;
    font-weight: 800;
    color: var(--primary-red);
    line-height: 1;
    opacity: 0.88;
  }

  .detail-right-col {
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: left;
  }

  .detail-weekday {
    font-size: 0.95rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.03em;
    line-height: 1.2;
    color: var(--text-primary);
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
    border-left: 3px solid var(--accent-gold);
    padding: 8px 10px;
    border-radius: 4px;
    margin-bottom: 6px;
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

    .main-layout.insight-mode {
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
      padding: 12px;
    }

    .app-header {
      padding: 8px 12px;
      flex-direction: column;
      gap: 10px;
      border-radius: 12px;
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
      justify-content: flex-end;
    }

    .calendar-grid {
      gap: 4px;
    }

    .day-card {
      min-height: 60px;
      padding: 4px 6px;
    }

    .solar-date {
      font-size: 1.1rem;
    }

    .pill {
      font-size: 0.55rem;
      padding: 1px 3px;
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

  /* Settings Menu */
  .settings-wrapper {
    position: relative;
  }

  .settings-btn {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(255, 255, 255, 0.85);
    border: 1px solid var(--border-subtle);
    color: var(--text-secondary);
    cursor: pointer;
    transition: all 0.2s;
  }

  .settings-btn:hover {
    color: var(--primary-red);
    transform: translateY(-1px);
    box-shadow: 0 4px 10px rgba(217, 48, 37, 0.14);
  }

  .settings-dropdown {
    position: absolute;
    top: calc(100% + 8px);
    right: 0;
    background: #fff;
    border: 1px solid var(--border-subtle);
    border-radius: 12px;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
    min-width: 200px;
    padding: 6px;
    z-index: 100;
  }

  .dropdown-item {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 10px 12px;
    border: none;
    background: none;
    border-radius: 8px;
    font-family: var(--font-sans);
    font-size: 0.85rem;
    font-weight: 600;
    color: var(--text-primary);
    cursor: pointer;
    transition: background 0.15s;
  }

  .dropdown-item:hover {
    background: rgba(42, 110, 100, 0.08);
  }

  .dropdown-version {
    padding: 8px 12px 6px;
    font-family: var(--font-sans);
    font-size: 0.72rem;
    color: var(--text-tertiary);
    border-top: 1px solid var(--border-subtle);
    margin-top: 4px;
  }

  @media (prefers-reduced-motion: reduce) {
    *, *::before, *::after {
      animation-duration: 0.01ms !important;
      animation-iteration-count: 1 !important;
      transition-duration: 0.01ms !important;
      scroll-behavior: auto !important;
    }
  }

</style>
