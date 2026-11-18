<script lang="ts">
  import { fetchDayView, fetchDayViewMonth } from '$lib/api/invoke';
  import type {
    DayViewDto,
    DayViewMonthDto,
    DayViewPatternItemDto,
    DayViewPersonalContextInput
  } from '$lib/api/types';

  let view: DayViewDto | null = null;
  let monthView: DayViewMonthDto | null = null;
  let error: string | null = null;
  let selected = new Date();
  // S2: the Selected Hour survives date navigation; it never moves the anchor.
  let selectedHour: number | null = null;
  let showGrid = true;
  let jumpInput: HTMLInputElement | null = null;

  // S3 (`amlich-b14l.9`): Personal Context — Progressive Enrichment over
  // the same Day View. Intent is the gate; birth profile and location are
  // optional layers that make their changes explicit.
  let showContext = false;
  let intent = '';
  let birthDate = '';
  let birthTime = '';
  let gender = '';
  let location = '';

  const INTENTS: { key: string; label: string }[] = [
    { key: 'wedding', label: 'Cưới hỏi' },
    { key: 'moving_house', label: 'Chuyển nhà' },
    { key: 'opening_business', label: 'Khai trương' },
    { key: 'contract_signing', label: 'Ký kết hợp đồng' },
    { key: 'travel', label: 'Xuất hành' },
    { key: 'burial', label: 'An táng' },
    { key: 'renovation', label: 'Tu tạo' },
    { key: 'medical_checkup', label: 'Khám bệnh' },
    { key: 'prayer', label: 'Cầu nguyện' }
  ];

  const WEEKDAYS = ['CN', 'T2', 'T3', 'T4', 'T5', 'T6', 'T7'];

  $: day = selected.getDate();
  $: month = selected.getMonth() + 1;
  $: year = selected.getFullYear();

  function todayTriple(): [number, number, number] {
    const now = new Date();
    return [now.getDate(), now.getMonth() + 1, now.getFullYear()];
  }

  function currentChiIndex(date: Date): number {
    return Math.floor(((date.getHours() + 1) % 24) / 2);
  }

  /** S3: build the Personal Context input from the form. No intent →
   * `null` → the anonymous non-verdict view. */
  function personalContext(): DayViewPersonalContextInput | null {
    if (!intent) return null;
    const [by, bm, bd] = birthDate ? birthDate.split('-').map(Number) : [];
    const [bh, bmin] = birthTime ? birthTime.split(':').map(Number) : [];
    return {
      intent,
      birth_year: bd ? by : null,
      birth_month: bd ? bm : null,
      birth_day: bd ? bd : null,
      birth_hour: bmin !== undefined ? bh : null,
      birth_minute: bmin !== undefined ? bmin : null,
      gender: gender || null,
      location_name: location.trim() || null
    };
  }

  async function load() {
    error = null;
    try {
      view = await fetchDayView(
        day,
        month,
        year,
        currentChiIndex(new Date()),
        selectedHour,
        personalContext()
      );
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    }
  }

  async function loadMonth() {
    try {
      monthView = await fetchDayViewMonth(month, year, todayTriple(), [day, month, year]);
    } catch {
      monthView = null;
    }
  }

  function step(delta: number) {
    selected = new Date(selected.getFullYear(), selected.getMonth(), selected.getDate() + delta);
  }

  function goToday() {
    selected = new Date();
  }

  function jumpTo(value: string) {
    const parsed = new Date(`${value}T00:00:00`);
    if (!Number.isNaN(parsed.getTime())) {
      selected = parsed;
    }
  }

  function selectHour(index: number) {
    selectedHour = index;
    void load();
  }

  function stepHour(delta: number) {
    const base = selectedHour ?? view?.hours.current_hour_index ?? 0;
    selectedHour = (base + delta + 12) % 12;
    void load();
  }

  function onKey(event: KeyboardEvent) {
    if (event.target instanceof HTMLInputElement) return;
    if (event.key === 'ArrowLeft') step(-1);
    else if (event.key === 'ArrowRight') step(1);
    else if (event.key === 'ArrowUp') stepHour(-1);
    else if (event.key === 'ArrowDown') stepHour(1);
    else if (event.key === 't' || event.key === 'T') goToday();
    else if (event.key === 'g' || event.key === 'G') {
      event.preventDefault();
      jumpInput?.focus();
      jumpInput?.showPicker?.();
    } else if (event.key === 'm' || event.key === 'M') showGrid = !showGrid;
    else if (event.key === 'p' || event.key === 'P') {
      event.preventDefault();
      showContext = !showContext;
    }
  }

  $: day, month, year, intent, birthDate, birthTime, gender, location, load();
  $: month, year, day, loadMonth();

  const patternGroups: { key: 'supports' | 'constraints' | 'unknowns'; label: string }[] = [
    { key: 'supports', label: 'Hỗ trợ' },
    { key: 'constraints', label: 'Giới hạn' },
    { key: 'unknowns', label: 'Chưa biết' }
  ];

  function familyLabel(family: string): string {
    const labels: Record<string, string> = {
      canchi: 'Can Chi',
      tiet_khi: 'Tiết Khí',
      truc: 'Trực',
      stars: 'Sao',
      day_deity: 'Thần trực',
      taboos: 'Kiêng kỵ',
      day_conflict: 'Xung chi',
      hoang_dao_hours: 'Giờ Hoàng Đạo',
      day_element: 'Ngũ Hành',
      recommendations: 'Khuyến nghị',
      intent: 'Mục đích',
      birth_profile: 'Hồ sơ sinh',
      location: 'Vị trí'
    };
    return labels[family] ?? family;
  }

  const monthNames = [
    'Tháng 1', 'Tháng 2', 'Tháng 3', 'Tháng 4', 'Tháng 5', 'Tháng 6',
    'Tháng 7', 'Tháng 8', 'Tháng 9', 'Tháng 10', 'Tháng 11', 'Tháng 12'
  ];

  function gridCells(
    cells: DayViewMonthDto['cells'],
    firstWeekday: number
  ): (DayViewMonthDto['cells'][number] | null)[] {
    const pad: null[] = Array(firstWeekday).fill(null);
    return [...pad, ...cells];
  }
</script>

<svelte:window on:keydown={onKey} />

<svelte:head>
  <title>Không gian Ngày — Âm Lịch</title>
</svelte:head>

<div class="flex h-full flex-col bg-parchment text-ink">
  <header class="flex items-center justify-between border-b border-ink-border px-6 py-3">
    <div class="font-mono">
      <span class="text-sm font-bold tracking-widest">ÂM LỊCH</span>
      <small class="ml-2 text-ink-light">Đài quan sát ngày</small>
    </div>
    <div class="flex items-center gap-2">
      <button
        class="rounded border border-ink-border px-3 py-1 hover:bg-parchment-dark"
        aria-label="Ngày trước"
        on:click={() => step(-1)}>‹</button
      >
      <div class="min-w-56 text-center">
        {#if view}
          <b class="text-sm">{view.solar.day_of_week_name}, {view.solar.date_string}</b><br />
          <small class="text-ink-light"
            >{view.lunar.date_string}{view.lunar.is_leap_month ? ' (nhuận)' : ''} · ngày
            {view.canchi.day.can} {view.canchi.day.chi}</small
          >
        {:else}
          <span class="text-ink-light">…</span>
        {/if}
      </div>
      <button
        class="rounded border border-ink-border px-3 py-1 hover:bg-parchment-dark"
        aria-label="Ngày sau"
        on:click={() => step(1)}>›</button
      >
      <button class="ml-2 rounded bg-ink px-3 py-1 text-parchment" on:click={goToday}>Hôm nay</button>
      <input
        bind:this={jumpInput}
        class="ml-2 rounded border border-ink-border bg-white px-2 py-1 text-sm"
        type="date"
        aria-label="Đến ngày"
        value={`${year}-${String(month).padStart(2, '0')}-${String(day).padStart(2, '0')}`}
        on:change={(event) => jumpTo(event.currentTarget.value)}
      />
    </div>
  </header>

  <main class="flex-1 overflow-y-auto px-6 py-5">
    {#if error}
      <p class="font-mono text-ky">Không tải được dữ liệu ngày: {error}</p>
    {:else if !view}
      <p class="font-mono italic text-ink-light">Đang tải…</p>
    {:else}
      <div class="grid grid-cols-1 gap-4 xl:grid-cols-[minmax(0,1fr)_320px]">
        <div class="min-w-0">
          <section class="mb-6">
            <p class="mb-1 font-mono text-xs uppercase tracking-widest text-ink-light">Tổng quan ngày</p>
            <p class="text-sm text-ink-light">
              Tiết {view.tiet_khi.name} · {view.tiet_khi.season}. Tổng quan ẩn danh, không kết luận
              phù hợp hay không phù hợp.
            </p>
          </section>

          <section class="mb-6">
            <p class="mb-2 font-mono text-xs uppercase tracking-widest text-ink-light">
              Mẫu Ngày · tín hiệu được nhóm, không quy thành điểm
            </p>
            <div class="grid grid-cols-1 gap-3 md:grid-cols-3">
              {#each patternGroups as group (group.key)}
                <article class="rounded border border-ink-border bg-white p-3">
                  <h3 class="mb-2 flex items-center justify-between text-sm font-semibold">
                    {group.label}
                    <span class="rounded-full bg-parchment-dark px-2 text-xs"
                      >{view.pattern[group.key].length}</span
                    >
                  </h3>
                  <ul class="space-y-2">
                    {#each view.pattern[group.key] as item: DayViewPatternItemDto (item.title)}
                      <li class={`border-l-2 pl-2 ${
                        group.key === 'supports'
                          ? 'border-nen'
                          : group.key === 'constraints'
                            ? 'border-tranh'
                            : 'border-cothe'
                      }`}>
                        <b class="text-sm">{item.title}</b>
                        <p class="text-xs text-ink-light">{item.reason}</p>
                      </li>
                    {/each}
                  </ul>
                </article>
              {/each}
            </div>
          </section>

          <section class="mb-6">
            <p class="mb-2 font-mono text-xs uppercase tracking-widest text-ink-light">
              Giờ trong ngày · mười hai cửa sổ, nổi bật không có nghĩa là phù hợp nhất
            </p>
            <div class="flex gap-2 overflow-x-auto pb-1">
              {#each view.hours.hours as hour (hour.hour_index)}
                <button
                  class={`w-28 shrink-0 rounded border p-2 text-center ${
                    hour.hour_index === selectedHour
                      ? 'border-ink bg-parchment-dark'
                      : hour.is_current
                        ? 'border-hoangdao bg-hoangdao/10'
                        : hour.is_hoang_dao
                          ? 'border-hoangdao/60'
                          : 'border-ink-border'
                  }`}
                  aria-pressed={hour.hour_index === selectedHour}
                  on:click={() => selectHour(hour.hour_index)}
                >
                  <small class="block font-mono text-[10px] text-ink-light">{hour.time_range}</small>
                  <b class="block">{hour.chi}</b>
                  {#if hour.hour_index === selectedHour}
                    <small class="block text-[10px] font-bold">đang chọn</small>
                  {:else if hour.is_current}
                    <small class="block text-[10px] text-hoangdao">đang diễn ra</small>
                  {:else if hour.is_notable}
                    <small class="block text-[10px] text-ink-light">{hour.star}</small>
                  {:else}
                    <small class="block text-[10px] text-ink-border">·</small>
                  {/if}
                </button>
              {/each}
            </div>

            {#if view.hours.detail}
              <article class="mt-3 rounded border border-ink bg-white p-3">
                <h4 class="mb-1 flex flex-wrap items-center gap-2 text-sm font-semibold">
                  <span>Giờ {view.hours.detail.chi} · {view.hours.detail.time_range}</span>
                  <span
                    class={`rounded-full px-2 py-0.5 text-xs ${
                      view.hours.detail.is_hoang_dao
                        ? 'bg-hoangdao/20 text-ink'
                        : 'bg-parchment-dark text-ink-light'
                    }`}
                  >
                    {view.hours.detail.classification}
                  </span>
                  <span class="rounded-full border border-ink-border px-2 py-0.5 text-xs">
                    sao {view.hours.detail.star}
                  </span>
                  {#if view.hours.detail.is_current}
                    <span class="text-xs text-hoangdao">đang diễn ra</span>
                  {/if}
                </h4>
                <ul class="list-inside list-disc space-y-1 text-xs text-ink-light">
                  {#each view.hours.detail.reasons as reason (reason)}
                    <li>{reason}</li>
                  {/each}
                </ul>
              </article>
            {:else}
              <p class="mt-2 text-xs italic text-ink-light">
                Chọn một cửa sổ giờ để xem phân loại Hoàng Đạo, sao cai trị và bối cảnh theo giờ —
                ngày đang xem không đổi.
              </p>
            {/if}
          </section>

          <section class="mb-6">
            <button
              class="flex w-full items-center justify-between rounded border border-ink-border bg-white px-3 py-2 text-left hover:bg-parchment-dark"
              aria-expanded={showContext}
              on:click={() => (showContext = !showContext)}
            >
              <span class="font-mono text-xs uppercase tracking-widest text-ink-light">
                Bối cảnh cá nhân · bổ sung tầng, tổng quan ẩn danh giữ nguyên
              </span>
              <span class="text-xs text-ink-light">{showContext ? '▾' : '▸'} P</span>
            </button>
            {#if showContext}
              <div class="mt-2 grid grid-cols-1 gap-3 rounded border border-ink-border bg-white p-3 md:grid-cols-2">
                <label class="text-xs">
                  Mục đích (bắt buộc để mở Đánh giá Ngày)
                  <select
                    class="mt-1 w-full rounded border border-ink-border bg-white px-2 py-1 text-sm"
                    bind:value={intent}
                  >
                    <option value="">— chưa chọn —</option>
                    {#each INTENTS as item (item.key)}
                      <option value={item.key}>{item.label}</option>
                    {/each}
                  </select>
                </label>
                <label class="text-xs">
                  Ngày sinh
                  <input
                    class="mt-1 w-full rounded border border-ink-border bg-white px-2 py-1 text-sm"
                    type="date"
                    bind:value={birthDate}
                  />
                </label>
                <label class="text-xs">
                  Giờ sinh (tùy chọn)
                  <input
                    class="mt-1 w-full rounded border border-ink-border bg-white px-2 py-1 text-sm"
                    type="time"
                    bind:value={birthTime}
                  />
                </label>
                <label class="text-xs">
                  Giới tính
                  <select
                    class="mt-1 w-full rounded border border-ink-border bg-white px-2 py-1 text-sm"
                    bind:value={gender}
                  >
                    <option value="">— chưa chọn —</option>
                    <option value="nam">Nam</option>
                    <option value="nữ">Nữ</option>
                  </select>
                </label>
                <label class="text-xs md:col-span-2">
                  Vị trí (tùy chọn)
                  <input
                    class="mt-1 w-full rounded border border-ink-border bg-white px-2 py-1 text-sm"
                    type="text"
                    placeholder="ví dụ: Hà Nội"
                    bind:value={location}
                  />
                </label>
              </div>
            {/if}
          </section>

          {#if view.assessment}
            {@const assessment = view.assessment}
            <section class="mb-6">
              <p class="mb-2 font-mono text-xs uppercase tracking-widest text-ink-light">
                Đánh giá Ngày · theo mục đích {assessment.intent_label}, không phải phán quyết
                phổ quát
              </p>
              <article class="rounded border border-ink bg-white p-3">
                <header class="mb-2 flex flex-wrap items-center gap-2">
                  <span
                    class={`rounded-full px-3 py-1 text-sm font-bold ${
                      assessment.result.bucket === 'favorable'
                        ? 'bg-nen/20'
                        : assessment.result.bucket === 'avoid'
                          ? 'bg-tranh/20'
                          : 'bg-parchment-dark'
                    }`}
                  >
                    {assessment.result.bucket_label}
                  </span>
                  <span class="text-xs text-ink-light">
                    độ tin cậy đánh giá: {assessment.confidence.level_label} · mức phủ input, không
                    phải xác suất
                  </span>
                </header>
                <p class="mb-3 text-sm">{assessment.result.primary_conclusion}</p>

                <div class="grid grid-cols-1 gap-3 xl:grid-cols-2">
                  <div>
                    <h4 class="mb-1 text-xs font-semibold uppercase tracking-wide text-ink-light">
                      Lý do ({assessment.reasons.length})
                    </h4>
                    <ul class="list-inside list-disc space-y-1 text-xs text-ink-light">
                      {#each assessment.reasons as reason (reason.text)}
                        <li>{reason.text}</li>
                      {/each}
                    </ul>
                    <h4 class="mb-1 mt-3 text-xs font-semibold uppercase tracking-wide text-ink-light">
                      Vì sao độ tin cậy ở mức này
                    </h4>
                    <ul class="list-inside list-disc space-y-1 text-xs text-ink-light">
                      {#each assessment.confidence.reasons as reason (reason)}
                        <li>{reason}</li>
                      {/each}
                    </ul>
                  </div>
                  <div>
                    <h4 class="mb-1 text-xs font-semibold uppercase tracking-wide text-ink-light">
                      Tầng bổ sung · mỗi tầng nêu rõ thay đổi
                    </h4>
                    <ol class="space-y-2 text-xs">
                      {#each assessment.layers as layer, index (layer.layer)}
                        <li class="rounded border border-ink-border p-2">
                          <b>{index + 1}. {layer.layer_label}</b>
                          <span class={`ml-1 ${layer.applied ? 'text-nen' : 'text-cothe'}`}>
                            {layer.applied ? '✓ đã bổ sung' : '○ chưa bổ sung'}
                          </span>
                          <p class="text-ink-light">{layer.summary}</p>
                          <ul class="mt-1 list-inside list-disc text-ink-light">
                            {#each layer.changes as change (change)}
                              <li>{change}</li>
                            {/each}
                          </ul>
                        </li>
                      {/each}
                    </ol>
                    {#if assessment.unknowns.length > 0}
                      <p class="mt-2 text-xs italic text-cothe">
                        Còn chưa biết: {assessment.unknowns.map((u) => u.title).join(', ')}.
                      </p>
                    {/if}
                  </div>
                </div>
              </article>
            </section>
          {:else if intent}
            <p class="mb-6 text-xs italic text-ink-light">
              Chọn mục đích hợp lệ để mở Đánh giá Ngày.
            </p>
          {/if}

          <section>
            <p class="mb-2 font-mono text-xs uppercase tracking-widest text-ink-light">
              Phủ chứng cứ · mức độ sẵn có, không phải độ chắc chắn
            </p>
            <div class="flex flex-wrap gap-2">
              {#each view.coverage.entries as entry (entry.family)}
                <span
                  class={`rounded-full border px-2 py-0.5 text-xs ${
                    entry.state === 'present'
                      ? 'border-nen/60 text-nen'
                      : 'border-cothe/50 text-cothe'
                  }`}
                >
                  {familyLabel(entry.family)}
                  {entry.state === 'present' ? '✓' : '?'}
                </span>
              {/each}
            </div>
          </section>
        </div>

        {#if showGrid && monthView}
          {@const grid = monthView}
          <aside class="min-w-0">
            <div class="rounded border border-ink-border bg-white p-3">
              <div class="mb-2 flex items-center justify-between">
                <button
                  class="rounded border border-ink-border px-2 hover:bg-parchment-dark"
                  aria-label="Tháng trước"
                  on:click={() => {
                    selected = new Date(grid.year, grid.month - 2, 1);
                  }}>‹</button
                >
                <b class="text-sm">{monthNames[grid.month - 1]} {grid.year}</b>
                <button
                  class="rounded border border-ink-border px-2 hover:bg-parchment-dark"
                  aria-label="Tháng sau"
                  on:click={() => {
                    selected = new Date(grid.year, grid.month, 1);
                  }}>›</button
                >
              </div>
              <div class="grid grid-cols-7 gap-1 text-center font-mono text-[10px] text-ink-light">
                {#each WEEKDAYS as weekday (weekday)}
                  <span>{weekday}</span>
                {/each}
                {#each gridCells(grid.cells, grid.first_weekday) as cell, index (index)}
                  {#if cell}
                    <button
                      class={`flex flex-col rounded p-1 text-center ${
                        cell.is_selected
                          ? 'bg-ink text-parchment'
                          : cell.is_today
                            ? 'border border-hoangdao'
                            : 'hover:bg-parchment-dark'
                      }`}
                      aria-pressed={cell.is_selected}
                      title={`${cell.can_chi_day} · âm lịch ${cell.lunar_label}`}
                      on:click={() => {
                        selected = new Date(grid.year, grid.month - 1, cell.day);
                      }}
                    >
                      <b class="text-xs">{cell.day}</b>
                      <small class={`text-[9px] ${
                        cell.is_selected ? 'text-parchment/80' : 'text-ink-light'
                      }`}>
                        {cell.lunar_day}/{cell.lunar_month}{cell.is_leap_lunar_month ? 'n' : ''}
                      </small>
                    </button>
                  {:else}
                    <span></span>
                  {/if}
                {/each}
              </div>
              <p class="mt-2 font-mono text-[9px] text-ink-light">
                nhãn ngày/tháng âm · hậu tố n = tháng nhuận · M đổi lưới
              </p>
            </div>
          </aside>
        {/if}
      </div>
    {/if}
  </main>

  <footer class="border-t border-ink-border px-6 py-2 font-mono text-[10px] text-ink-light">
    ← / → đổi ngày · ↑ / ↓ chọn giờ · T về hôm nay · G đến ngày · M lưới tháng · P bối cảnh cá nhân ·
    day-view {view?.schema_version}
  </footer>
</div>
