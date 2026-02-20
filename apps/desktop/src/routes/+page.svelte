<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { getVersion } from "@tauri-apps/api/app";
  import DateInsightBox from "$lib/components/DateInsightBox.svelte";
  import ZodiacClock from "$lib/components/ZodiacClock.svelte";
  import type { DayCell, MonthData } from "$lib/insights/types";
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

  const CAT_COLORS: Record<string, { bg: string; border: string }> = {
    'festival': { bg: '#FFF0F0', border: '#C62828' },
    'public-holiday': { bg: '#FFF0F5', border: '#AD1457' },
    'commemorative': { bg: '#F0F4FF', border: '#1565C0' },
    'professional': { bg: '#F0FFF0', border: '#2E7D32' },
    'social': { bg: '#FFF8F0', border: '#E65100' },
    'international': { bg: '#F8F0FF', border: '#6A1B9A' },
    'solar-term': { bg: '#F0FFFD', border: '#00796B' },
    'lunar-cycle': { bg: '#FFFCF0', border: '#D4AF37' },
  };

  type EventCategoryType =
    | "festival"
    | "public-holiday"
    | "commemorative"
    | "professional"
    | "social"
    | "international"
    | "lunar-cycle";

  type EventCategory = {
    type: EventCategoryType;
    colorHex: string;
    priority: number;
    name: string;
  };

  const CATEGORY_PRIORITY: Record<EventCategoryType, number> = {
    festival: 1,
    "public-holiday": 2,
    commemorative: 3,
    professional: 4,
    social: 5,
    international: 6,
    "lunar-cycle": 7,
  };

  const PHASE_LABEL: Record<number, string> = {
    1: "Sóc",
    8: "Thượng huyền",
    15: "Vọng",
    23: "Hạ huyền",
  };

  function classifyHoliday(holiday: DayCell["holidays"][number], _day: DayCell): EventCategoryType {
    return holiday.category;
  }

  function getDayEventCategories(day: DayCell): EventCategory[] {
    const categories: EventCategory[] = [];

    for (const holiday of day.holidays) {
      const type = classifyHoliday(holiday, day);
      const colorHex = CAT_COLORS[type]?.border ?? "#6B7280";
      categories.push({
        type,
        colorHex,
        priority: CATEGORY_PRIORITY[type],
        name: holiday.name,
      });
    }

    const phase = PHASE_LABEL[day.lunar_day];
    if (phase) {
      categories.push({
        type: "lunar-cycle",
        colorHex: CAT_COLORS["lunar-cycle"].border,
        priority: CATEGORY_PRIORITY["lunar-cycle"],
        name: phase,
      });
    }

    categories.sort((a, b) => a.priority - b.priority);
    return categories;
  }

  function getDayDots(
    _day: DayCell,
    categories: EventCategory[] = [],
  ): { type: EventCategoryType; colorHex: string }[] {
    const seen = new Set<EventCategoryType>();
    const dots: { type: EventCategoryType; colorHex: string }[] = [];

    for (const cat of categories) {
      if (seen.has(cat.type)) continue;
      seen.add(cat.type);
      dots.push({ type: cat.type, colorHex: cat.colorHex });
      if (dots.length >= 4) break;
    }

    return dots;
  }

  const initialNow = new Date();
  let today = $state(new Date());
  let viewYear = $state(initialNow.getFullYear());
  let viewMonth = $state(initialNow.getMonth() + 1);

  $effect(() => {
    const now = new Date();
    const msUntilMidnight =
      new Date(now.getFullYear(), now.getMonth(), now.getDate() + 1).getTime() - now.getTime();
    const timeout = setTimeout(() => { today = new Date(); }, msUntilMidnight + 500);
    return () => clearTimeout(timeout);
  });
  let monthData = $state<MonthData | null>(null);
  let selectedDay = $state<DayCell | null>(null);
  let preferredDayOfMonth: number | null = null;
  let isLoading = $state(false);
  let error = $state<string | null>(null);

  let loadToken = 0;

  $effect(() => {
    const token = ++loadToken;
    loadMonth(viewMonth, viewYear, token);
  });

  async function loadMonth(month: number, year: number, token: number) {
    isLoading = true;
    error = null;
    try {
      const data = await invoke<MonthData>("get_month_data", { month, year });
      if (token !== loadToken) return;
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
      if (token !== loadToken) return;
      error = err instanceof Error ? err.message : String(err);
      monthData = null;
      selectedDay = null;
    } finally {
      if (token !== loadToken) return;
      isLoading = false;
    }
  }

  let selectToken = 0;
  const dayDetailCache = new Map<string, DayCell>();

  async function selectDay(day: DayCell) {
    selectedDay = day;
    preferredDayOfMonth = day.day;

    const k = `${day.year}-${day.month}-${day.day}`;
    const cached = dayDetailCache.get(k);
    if (cached) {
      selectedDay = cached;
      return;
    }

    const token = ++selectToken;
    try {
      const detail = await invoke<DayCell>("get_day_detail", {
        day: day.day,
        month: day.month,
        year: day.year,
      });
      if (token !== selectToken) return;
      dayDetailCache.set(k, detail);
      selectedDay = detail;
      preferredDayOfMonth = detail.day;
    } catch (err) {
      if (token !== selectToken) return;
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

  type DayKey = string;
  const dayKey = (d: { year: number; month: number; day: number }) => `${d.year}-${d.month}-${d.day}`;

  const dayRows = $derived.by(() => {
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
  });

  const monthTitle = $derived(`${monthNames[viewMonth - 1]} ${viewYear}`);

  const catsByDay = $derived.by(() => {
    if (!monthData) return new Map<DayKey, EventCategory[]>();
    const map = new Map<DayKey, EventCategory[]>();
    for (const d of monthData.days) {
      map.set(dayKey(d), getDayEventCategories(d));
    }
    return map;
  });

  const dotsByDay = $derived.by(() => {
    if (!monthData) return new Map<DayKey, { type: EventCategoryType; colorHex: string }[]>();
    const map = new Map<DayKey, { type: EventCategoryType; colorHex: string }[]>();
    for (const d of monthData.days) {
      const k = dayKey(d);
      map.set(k, getDayDots(d, catsByDay.get(k)));
    }
    return map;
  });

  // Cultural Detail Visibility
  let isInsightVisible = $state(false);

  function toggleInsight() {
    isInsightVisible = !isInsightVisible;
  }

  // Month picker popover
  let showMonthPicker = $state(false);
  let pickerYear = $state(initialNow.getFullYear());

  function toggleMonthPicker() {
    if (!showMonthPicker) {
      pickerYear = viewYear;
    }
    showMonthPicker = !showMonthPicker;
  }

  function pickMonth(month: number) {
    preferredDayOfMonth = selectedDay?.day ?? preferredDayOfMonth;
    viewMonth = month;
    viewYear = pickerYear;
    showMonthPicker = false;
  }

  function pickerPrevYear() {
    pickerYear -= 1;
  }

  function pickerNextYear() {
    pickerYear += 1;
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

  const daysBySolarDate = $derived.by(() => {
    if (!monthData) return new Map<string, DayCell>();
    const map = new Map<string, DayCell>();
    for (const d of monthData.days) {
      map.set(d.solar_date, d);
    }
    return map;
  });

  function handleGridClick(event: MouseEvent) {
    const btn = (event.target as HTMLElement).closest<HTMLButtonElement>("button.day-card[data-solar-date]");
    if (!btn) return;
    const solarDate = btn.dataset.solarDate;
    if (!solarDate) return;
    const day = daysBySolarDate.get(solarDate);
    if (day) selectDay(day);
  }

  function handleClickOutside(event: MouseEvent) {
    if (!showSettingsMenu && !showMonthPicker) return;

    const target = event.target as HTMLElement;
    if (showSettingsMenu && !target.closest(".settings-wrapper")) {
      showSettingsMenu = false;
    }
    if (showMonthPicker && !target.closest(".month-pill-wrapper")) {
      showMonthPicker = false;
    }
  }
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
      <div class="month-pill-wrapper">
        <button class="month-pill" onclick={toggleMonthPicker} aria-expanded={showMonthPicker} aria-haspopup="dialog">
          <span class="month-caption">Lịch tháng</span>
          <span class="current-month">
            {monthTitle}
            <svg class="pill-chevron" class:open={showMonthPicker} xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="m6 9 6 6 6-6"/></svg>
          </span>
        </button>

        {#if showMonthPicker}
          <div class="month-picker-popover" role="dialog" aria-label="Chọn tháng và năm">
            <div class="picker-year-strip">
              <button class="picker-year-arrow" onclick={pickerPrevYear} aria-label="Năm trước">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="m15 18-6-6 6-6"/></svg>
              </button>
              <span class="picker-year-label" class:is-current-year={pickerYear === today.getFullYear()}>{pickerYear}</span>
              <button class="picker-year-arrow" onclick={pickerNextYear} aria-label="Năm sau">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="m9 18 6-6-6-6"/></svg>
              </button>
            </div>

            <div class="picker-month-grid">
              {#each monthNames as name, i}
                {@const m = i + 1}
                {@const isSelected = m === viewMonth && pickerYear === viewYear}
                {@const isCurrent = m === today.getMonth() + 1 && pickerYear === today.getFullYear()}
                <button
                  class="picker-month-cell"
                  class:selected={isSelected}
                  class:current={isCurrent && !isSelected}
                  onclick={() => pickMonth(m)}
                >
                  <span class="picker-month-num">{m}</span>
                  <span class="picker-month-name">{name.replace("Tháng ", "Th")}</span>
                </button>
              {/each}
            </div>

            <button class="picker-today-btn" onclick={() => { pickMonth(today.getMonth() + 1); pickerYear = today.getFullYear(); viewYear = today.getFullYear(); preferredDayOfMonth = today.getDate(); }}>
              <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 6v6l4 2"/></svg>
              Hôm nay
            </button>
          </div>
        {/if}
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
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="calendar-grid" onclick={handleGridClick}>
          {#each dayRows as row}
            {#each row as day}
              {#if day}
                {@const k = dayKey(day)}
                {@const categories = catsByDay.get(k) ?? []}
                {@const dots = dotsByDay.get(k) ?? []}
                {@const topCat = categories.length > 0 ? categories[0] : null}
                <button
                  type="button"
                  class="day-card"
                  data-solar-date={day.solar_date}
                  class:selected={selectedDay?.solar_date === day.solar_date}
                  class:today={day.day === today.getDate() &&
                    day.month === today.getMonth() + 1 &&
                    day.year === today.getFullYear()}
                  class:has-holiday={day.holidays.length > 0}
                  class:has-events={categories.length > 0}
                  style={topCat ? `--accent-color: ${topCat.colorHex}; --accent-tint: ${topCat.colorHex}0D;` : ''}
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
              <ZodiacClock goodHours={selectedDay.good_hours} />
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
                    {@const colors = CAT_COLORS[catType] || CAT_COLORS['lunar-cycle']}
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

  .month-pill-wrapper {
    position: relative;
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
    cursor: pointer;
    transition: all 0.2s ease;
    font-family: inherit;
  }

  .month-pill:hover {
    border-color: rgba(212, 175, 55, 0.7);
    box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.9),
      0 4px 16px rgba(212, 175, 55, 0.18);
    transform: translateY(-1px);
  }

  .month-pill:active {
    transform: translateY(0);
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
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .pill-chevron {
    transition: transform 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
    opacity: 0.5;
    flex-shrink: 0;
  }

  .pill-chevron.open {
    transform: rotate(180deg);
    opacity: 0.8;
  }

  .month-pill:hover .pill-chevron {
    opacity: 0.8;
  }

  /* ── Month Picker Popover ── */
  .month-picker-popover {
    position: absolute;
    top: calc(100% + 10px);
    left: 50%;
    transform: translateX(-50%);
    width: 320px;
    background: linear-gradient(180deg, #fffcf5 0%, #fff8ef 100%);
    border: 1px solid rgba(212, 175, 55, 0.35);
    border-radius: 18px;
    box-shadow:
      0 20px 60px rgba(44, 36, 27, 0.15),
      0 4px 16px rgba(212, 175, 55, 0.08),
      inset 0 1px 0 rgba(255, 255, 255, 0.9);
    padding: 16px;
    z-index: 200;
    animation: picker-enter 0.25s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  @keyframes picker-enter {
    from {
      opacity: 0;
      transform: translateX(-50%) translateY(-8px) scale(0.96);
    }
    to {
      opacity: 1;
      transform: translateX(-50%) translateY(0) scale(1);
    }
  }

  .picker-year-strip {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 16px;
    margin-bottom: 14px;
    padding-bottom: 12px;
    border-bottom: 1px solid rgba(212, 175, 55, 0.2);
  }

  .picker-year-arrow {
    width: 30px;
    height: 30px;
    border-radius: 50%;
    border: 1px solid rgba(212, 175, 55, 0.3);
    background: rgba(255, 255, 255, 0.85);
    color: #8f4e1f;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s;
  }

  .picker-year-arrow:hover {
    background: rgba(250, 240, 226, 0.95);
    border-color: rgba(212, 175, 55, 0.5);
    color: var(--primary-red);
    transform: scale(1.1);
  }

  .picker-year-label {
    font-family: var(--font-serif);
    font-size: 1.5rem;
    font-weight: 700;
    color: var(--text-primary);
    min-width: 80px;
    text-align: center;
    font-feature-settings: "tnum";
    letter-spacing: -0.02em;
    transition: color 0.2s;
  }

  .picker-year-label.is-current-year {
    color: var(--accent-jade);
  }

  .picker-month-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 6px;
    margin-bottom: 12px;
  }

  .picker-month-cell {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 10px 4px 8px;
    border-radius: 12px;
    border: 1px solid transparent;
    background: rgba(255, 255, 255, 0.6);
    cursor: pointer;
    transition: all 0.18s ease;
    font-family: var(--font-serif);
    gap: 1px;
  }

  .picker-month-cell:hover {
    background: rgba(255, 255, 255, 0.95);
    border-color: rgba(212, 175, 55, 0.4);
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(212, 175, 55, 0.15);
  }

  .picker-month-cell.selected {
    background: linear-gradient(135deg, #8d251a 0%, #b33325 100%);
    border-color: transparent;
    box-shadow: 0 4px 16px rgba(141, 37, 26, 0.35);
    transform: translateY(-1px);
  }

  .picker-month-cell.selected .picker-month-num {
    color: #fff;
  }

  .picker-month-cell.selected .picker-month-name {
    color: rgba(255, 255, 255, 0.8);
  }

  .picker-month-cell.current {
    border-color: var(--accent-jade);
    background: rgba(42, 110, 100, 0.06);
  }

  .picker-month-cell.current .picker-month-num {
    color: var(--accent-jade);
  }

  .picker-month-num {
    font-size: 1.15rem;
    font-weight: 700;
    color: var(--text-primary);
    line-height: 1;
    transition: color 0.18s;
  }

  .picker-month-name {
    font-size: 0.62rem;
    font-weight: 600;
    color: var(--text-tertiary);
    letter-spacing: 0.02em;
    transition: color 0.18s;
    font-family: var(--font-sans);
  }

  .picker-today-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    width: 100%;
    padding: 9px;
    border-radius: 10px;
    border: 1px solid rgba(42, 110, 100, 0.2);
    background: linear-gradient(140deg, rgba(42, 110, 100, 0.08) 0%, rgba(42, 110, 100, 0.14) 100%);
    color: var(--accent-jade);
    font-family: var(--font-sans);
    font-size: 0.78rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    cursor: pointer;
    transition: all 0.2s;
  }

  .picker-today-btn:hover {
    background: linear-gradient(140deg, rgba(42, 110, 100, 0.14) 0%, rgba(42, 110, 100, 0.22) 100%);
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(42, 110, 100, 0.15);
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

    .month-pill-wrapper {
      flex: 1;
      min-width: 0;
    }

    .month-pill {
      min-width: 0;
      flex: 1;
    }

    .month-picker-popover {
      width: 280px;
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
