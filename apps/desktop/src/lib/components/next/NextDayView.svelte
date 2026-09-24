<script lang="ts">
  import { onMount } from 'svelte';
  import { fetchDayView } from '$lib/api/invoke';
  import type { DayViewDto, DayViewPatternItemDto } from '$lib/api/types';

  let view: DayViewDto | null = null;
  let error: string | null = null;
  let selected = new Date();

  $: day = selected.getDate();
  $: month = selected.getMonth() + 1;
  $: year = selected.getFullYear();

  function currentChiIndex(date: Date): number {
    return Math.floor(((date.getHours() + 1) % 24) / 2);
  }

  async function load() {
    error = null;
    try {
      const now = new Date();
      view = await fetchDayView(day, month, year, currentChiIndex(now));
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    }
  }

  function step(delta: number) {
    selected = new Date(selected.getFullYear(), selected.getMonth(), selected.getDate() + delta);
  }

  function goToday() {
    selected = new Date();
  }

  function onKey(event: KeyboardEvent) {
    if (event.key === 'ArrowLeft') step(-1);
    else if (event.key === 'ArrowRight') step(1);
    else if (event.key === 't' || event.key === 'T') goToday();
  }

  $: day, month, year, load();

  const patternGroups: { key: 'supports' | 'constraints' | 'unknowns'; label: string; tone: string }[] = [
    { key: 'supports', label: 'Hỗ trợ', tone: 'nen' },
    { key: 'constraints', label: 'Giới hạn', tone: 'tranh' },
    { key: 'unknowns', label: 'Chưa biết', tone: 'cothe' }
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
    </div>
  </header>

  <main class="flex-1 overflow-y-auto px-6 py-5">
    {#if error}
      <p class="font-mono text-ky">Không tải được dữ liệu ngày: {error}</p>
    {:else if !view}
      <p class="font-mono italic text-ink-light">Đang tải…</p>
    {:else}
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
            <div
              class={`w-28 shrink-0 rounded border p-2 text-center ${
                hour.is_current
                  ? 'border-hoangdao bg-hoangdao/10'
                  : hour.is_hoang_dao
                    ? 'border-hoangdao/60'
                    : 'border-ink-border'
              }`}
            >
              <small class="block font-mono text-[10px] text-ink-light">{hour.time_range}</small>
              <b class="block">{hour.chi}</b>
              {#if hour.is_current}
                <small class="block text-[10px] text-hoangdao">đang diễn ra</small>
              {:else if hour.is_notable}
                <small class="block text-[10px] text-ink-light">{hour.star}</small>
              {:else}
                <small class="block text-[10px] text-ink-border">·</small>
              {/if}
            </div>
          {/each}
        </div>
      </section>

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
    {/if}
  </main>

  <footer class="border-t border-ink-border px-6 py-2 font-mono text-[10px] text-ink-light">
    ← / → đổi ngày · T về hôm nay · day-view {view?.schema_version}
  </footer>
</div>
