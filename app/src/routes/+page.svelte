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

<div class="page">
  <header class="hero">
    <div>
      <p class="eyebrow">Am Lich Desktop</p>
      <h1>Lich am va tiet khi</h1>
      <p class="subhead">
        Tra cuu lich am, Can Chi, tiet khi va gio hoang dao theo thang.
      </p>
    </div>
    <div class="hero-actions">
      <button class="ghost" type="button" onclick={goToday}>Hom nay</button>
      <button class="ghost" type="button" onclick={cycleHolidayFilter}>
        {holidayFilter === "all" ? "Tat ca ngay le" : "Ngay le chinh"}
      </button>
      <div class="switcher">
        <button class="nav" type="button" onclick={prevMonth}>◀</button>
        <span>{monthTitle()}</span>
        <button class="nav" type="button" onclick={nextMonth}>▶</button>
      </div>
      <div class="legend">
        <span class="legend-label">Hien thi:</span>
        <span class="legend-chip">
          {holidayFilter === "all" ? "Tat ca ngay le + mung 1/ram" : "Ngay le chinh"}
        </span>
      </div>
    </div>
  </header>

  <section class="content">
    <div class="calendar">
      <div class="weekdays">
        {#each weekLabels as label}
          <div>{label}</div>
        {/each}
      </div>

      {#if isLoading}
        <div class="loading">Dang tai du lieu...</div>
      {:else if error}
        <div class="error">{error}</div>
      {:else}
        <div class="grid">
          {#each dayRows() as row}
            {#each row as day}
              {#if day}
                <button
                  type="button"
                  class:selected={selectedDay?.solar_date === day.solar_date}
                  class:today={
                    day.day === today.getDate() &&
                    day.month === today.getMonth() + 1 &&
                    day.year === today.getFullYear()
                  }
                  onclick={() => selectDay(day)}
                >
                  <span class="solar">{day.day}</span>
                  <span class="lunar">
                    {day.lunar_day}/{day.lunar_month}
                  </span>
                  <span class="canchi">{day.canchi_day}</span>
                  {#if filterHolidays(day.holidays).length}
                    <span class="holiday">{filterHolidays(day.holidays)[0].name}</span>
                  {/if}
                </button>
              {:else}
                <div class="empty"></div>
              {/if}
            {/each}
          {/each}
        </div>
      {/if}
    </div>

    <aside class="detail">
      {#if selectedDay}
        <div class="detail-card">
          <p class="detail-date">{selectedDay.solar_date}</p>
          <h2>
            {selectedDay.day_of_week} · Am {selectedDay.lunar_date}
          </h2>
          <p class="detail-canchi">
            Ngay {selectedDay.canchi_day} · Thang {selectedDay.canchi_month}
          </p>
          <p class="detail-canchi">Nam {selectedDay.canchi_year}</p>

          <div class="detail-section">
            <h3>Ngay le</h3>
            {#if filterHolidays(selectedDay.holidays).length}
              <div class="holiday-list">
                {#each filterHolidays(selectedDay.holidays) as holiday}
                  <div class="holiday-item">
                    <p class="holiday-name">{holiday.name}</p>
                    <p class="holiday-desc">{holiday.description}</p>
                    {#if holiday.lunar_day && holiday.lunar_month}
                      <p class="holiday-lunar">
                        Am: {holiday.lunar_day}/{holiday.lunar_month}
                      </p>
                    {/if}
                  </div>
                {/each}
              </div>
            {:else}
              <p class="detail-muted">Khong co ngay le ghi nhan.</p>
            {/if}
          </div>

          <div class="detail-section">
            <h3>Tiet khi</h3>
            <p class="detail-title">{selectedDay.tiet_khi}</p>
            <p class="detail-subtitle">{selectedDay.tiet_khi_season}</p>
            <p>{selectedDay.tiet_khi_description}</p>
          </div>

          <div class="detail-section">
            <h3>Gio hoang dao</h3>
            <div class="good-hours">
              {#each selectedDay.good_hours as hour}
                <div class="hour">
                  <span>{hour.hour_chi}</span>
                  <span>{hour.time_range}</span>
                  <span>{hour.star}</span>
                </div>
              {/each}
            </div>
          </div>
        </div>
      {:else}
        <div class="detail-card empty-state">
          <p>Chon mot ngay de xem chi tiet.</p>
        </div>
      {/if}
    </aside>
  </section>
</div>

<style>
  @import url("https://fonts.googleapis.com/css2?family=Source+Serif+4:wght@400;600;700&display=swap");
  :global(body) {
    margin: 0;
    font-family: "Source Serif 4", "Iowan Old Style", "Palatino Linotype", serif;
    background: radial-gradient(circle at top, #f3efe7 0%, #efe3cf 35%, #e2d6be 100%);
    color: #1c1a14;
  }

  :global(*) {
    box-sizing: border-box;
  }

  .page {
    min-height: 100vh;
    padding: 32px 40px 48px;
    display: flex;
    flex-direction: column;
    gap: 32px;
  }

  .hero {
    display: flex;
    justify-content: space-between;
    align-items: flex-end;
    gap: 24px;
  }

  .eyebrow {
    text-transform: uppercase;
    letter-spacing: 0.12em;
    font-size: 0.75rem;
    margin: 0 0 8px;
    color: #7a6345;
  }

  h1 {
    font-size: 2.6rem;
    margin: 0;
  }

  .subhead {
    margin: 12px 0 0;
    max-width: 440px;
    color: #5a4a35;
  }

  .hero-actions {
    display: flex;
    flex-direction: column;
    gap: 16px;
    align-items: flex-end;
  }

  .legend {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 0.8rem;
    color: #6b563c;
  }

  .legend-label {
    text-transform: uppercase;
    letter-spacing: 0.1em;
    font-size: 0.7rem;
  }

  .legend-chip {
    padding: 6px 12px;
    border-radius: 999px;
    background: rgba(58, 45, 28, 0.08);
    border: 1px solid rgba(58, 45, 28, 0.2);
    font-weight: 600;
  }

  .switcher {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px 14px;
    background: rgba(255, 255, 255, 0.7);
    border-radius: 999px;
    border: 1px solid rgba(121, 97, 61, 0.25);
    font-weight: 600;
  }

  .switcher span {
    min-width: 120px;
    text-align: center;
  }

  .nav {
    border: none;
    background: #3a2d1c;
    color: #f7efe2;
    width: 34px;
    height: 34px;
    border-radius: 50%;
    font-size: 0.9rem;
    cursor: pointer;
  }

  .ghost {
    background: transparent;
    border: 1px solid rgba(58, 45, 28, 0.35);
    border-radius: 999px;
    padding: 8px 16px;
    font-weight: 600;
    cursor: pointer;
  }

  .content {
    display: grid;
    grid-template-columns: minmax(0, 1.6fr) minmax(0, 1fr);
    gap: 32px;
  }

  .calendar {
    background: rgba(255, 255, 255, 0.82);
    padding: 24px;
    border-radius: 24px;
    box-shadow: 0 24px 60px rgba(41, 30, 16, 0.15);
    backdrop-filter: blur(6px);
  }

  .weekdays {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    text-transform: uppercase;
    font-size: 0.75rem;
    letter-spacing: 0.15em;
    color: #7a6345;
    padding-bottom: 12px;
    border-bottom: 1px solid rgba(122, 99, 69, 0.2);
  }

  .grid {
    margin-top: 16px;
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 12px;
  }

  .grid button,
  .empty {
    min-height: 96px;
    border-radius: 16px;
    padding: 12px;
    border: 1px solid rgba(122, 99, 69, 0.2);
    background: #fff9f0;
    text-align: left;
    display: flex;
    flex-direction: column;
    gap: 6px;
    cursor: pointer;
    transition: transform 0.2s ease, box-shadow 0.2s ease;
  }

  .grid button:hover {
    transform: translateY(-2px);
    box-shadow: 0 12px 20px rgba(41, 30, 16, 0.18);
  }

  .grid button.selected {
    border-color: #3a2d1c;
    background: #f7efe2;
    box-shadow: 0 12px 24px rgba(58, 45, 28, 0.2);
  }

  .grid button.today {
    outline: 2px solid rgba(202, 144, 80, 0.7);
  }

  .empty {
    background: transparent;
    border: 1px dashed rgba(122, 99, 69, 0.2);
  }

  .solar {
    font-size: 1.15rem;
    font-weight: 700;
  }

  .lunar,
  .canchi {
    font-size: 0.78rem;
    color: #6b563c;
  }

  .canchi {
    font-weight: 600;
  }

  .holiday {
    font-size: 0.7rem;
    color: #8a5a1c;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.08em;
  }

  .detail {
    position: sticky;
    top: 24px;
    align-self: start;
  }

  .detail-card {
    background: #1f1710;
    color: #f9f4ea;
    padding: 24px;
    border-radius: 24px;
    box-shadow: 0 20px 40px rgba(31, 23, 16, 0.35);
  }

  .detail-card h2 {
    margin: 8px 0 12px;
    font-size: 1.5rem;
  }

  .detail-date {
    margin: 0;
    font-size: 0.85rem;
    text-transform: uppercase;
    letter-spacing: 0.18em;
    color: rgba(249, 244, 234, 0.7);
  }

  .detail-canchi {
    margin: 0;
    color: rgba(249, 244, 234, 0.82);
  }

  .detail-section {
    margin-top: 20px;
    padding-top: 16px;
    border-top: 1px solid rgba(249, 244, 234, 0.18);
  }

  .holiday-list {
    display: grid;
    gap: 10px;
  }

  .holiday-item {
    background: rgba(249, 244, 234, 0.08);
    padding: 10px 12px;
    border-radius: 12px;
  }

  .holiday-name {
    margin: 0;
    font-weight: 600;
  }

  .holiday-desc,
  .holiday-lunar,
  .detail-muted {
    margin: 4px 0 0;
    color: rgba(249, 244, 234, 0.75);
    font-size: 0.85rem;
  }

  .detail-section h3 {
    margin: 0 0 10px;
    font-size: 0.95rem;
    text-transform: uppercase;
    letter-spacing: 0.1em;
  }

  .detail-title {
    margin: 0;
    font-size: 1.2rem;
    font-weight: 600;
  }

  .detail-subtitle {
    margin: 4px 0 8px;
    color: rgba(249, 244, 234, 0.7);
  }

  .good-hours {
    display: grid;
    gap: 10px;
  }

  .hour {
    display: grid;
    grid-template-columns: 1fr 1fr 1.2fr;
    gap: 8px;
    font-size: 0.88rem;
  }

  .loading,
  .error {
    padding: 20px;
    text-align: center;
    font-weight: 600;
  }

  .error {
    color: #a0342a;
  }

  .empty-state {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 240px;
    text-align: center;
  }

  @media (max-width: 980px) {
    .page {
      padding: 24px;
    }

    .hero {
      flex-direction: column;
      align-items: flex-start;
    }

    .hero-actions {
      align-items: flex-start;
    }

    .content {
      grid-template-columns: 1fr;
    }

    .detail {
      position: static;
    }
  }

  @media (max-width: 640px) {
    h1 {
      font-size: 2rem;
    }

    .grid {
      gap: 8px;
    }

    .grid button,
    .empty {
      min-height: 84px;
    }

    .hour {
      grid-template-columns: 1fr;
    }
  }
</style>
