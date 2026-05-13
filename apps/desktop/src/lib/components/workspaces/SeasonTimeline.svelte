<script lang="ts">
    import { selectedDate } from '$lib/stores';
    import { fetchDayRange, fetchTietKhiForYear } from '$lib/api/invoke';
    import type {
        DayBundleDto,
        FestivalInsightDto,
        HolidayInsightDto,
        LocalizedListDto,
        LocalizedTextDto,
        TietKhiInsightDto,
        TietKhiTransitionDto,
    } from '$lib/api/types';

    type TimelineKind = 'holiday' | 'festival' | 'lunar_cycle' | 'tiet_khi';
    type CulturalInsight = FestivalInsightDto | HolidayInsightDto | TietKhiInsightDto;

    interface TimelineEvent {
        id: string;
        kind: TimelineKind;
        date: string;
        month: number;
        day: number;
        title: string;
        subtitle: string;
        lunarLabel?: string;
        category: string;
        major: boolean;
        insight?: CulturalInsight | null;
        bundle?: DayBundleDto;
    }

    let year = new Date().getFullYear();
    let loading = true;
    let error: string | null = null;
    let dayBundles: DayBundleDto[] = [];
    let tietKhiTransitions: TietKhiTransitionDto[] = [];
    let majorOnly = false;
    let selectedKind: TimelineKind | 'all' = 'all';
    let selectedEventId: string | null = null;
    let loadToken = 0;

    $: selectedYear = $selectedDate.getFullYear();
    $: if (selectedYear !== year) {
        year = selectedYear;
        loadYear(year);
    }

    $: allEvents = buildTimeline(dayBundles, tietKhiTransitions);
    $: filteredEvents = allEvents.filter((event) => {
        if (selectedKind !== 'all' && event.kind !== selectedKind) return false;
        if (majorOnly && !event.major) return false;
        return true;
    });
    $: selectedEvent = pickSelectedEvent(filteredEvents, selectedEventId);
    $: stats = {
        festival: allEvents.filter((event) => event.kind === 'festival').length,
        holiday: allEvents.filter((event) => event.kind === 'holiday').length,
        lunarCycle: allEvents.filter((event) => event.kind === 'lunar_cycle').length,
        tietKhi: allEvents.filter((event) => event.kind === 'tiet_khi').length,
    };

    loadYear(year);

    async function loadYear(nextYear: number) {
        const token = ++loadToken;
        loading = true;
        error = null;

        try {
            const [range, tietKhi] = await Promise.all([
                fetchDayRange({
                    start: { day: 1, month: 1, year: nextYear },
                    end: { day: 31, month: 12, year: nextYear },
                    includes: ['base', 'tiet_khi', 'insight'],
                }),
                fetchTietKhiForYear(nextYear),
            ]);

            if (token !== loadToken) return;
            dayBundles = range.days;
            tietKhiTransitions = tietKhi.transitions;
            selectedEventId = null;
        } catch (e: unknown) {
            if (token !== loadToken) return;
            console.error('Failed to load season timeline', e);
            error = e instanceof Error ? e.message : 'Failed to load season timeline';
            dayBundles = [];
            tietKhiTransitions = [];
        } finally {
            if (token === loadToken) loading = false;
        }
    }

    function buildTimeline(days: DayBundleDto[], transitions: TietKhiTransitionDto[]): TimelineEvent[] {
        const events: TimelineEvent[] = [];
        const daysByDate = new Map(days.map((day) => [dateKey(day), day]));

        for (const day of days) {
            if (day.insight?.holiday) {
                events.push(createFestivalEvent(day, 'holiday', day.insight.holiday));
            }
            if (day.insight?.festival) {
                events.push(createFestivalEvent(day, 'festival', day.insight.festival));
            }
            if (day.lunar.day === 1 || day.lunar.day === 15) {
                events.push(createLunarCycleEvent(day));
            }
        }

        for (const transition of transitions) {
            const day = daysByDate.get(transition.date);
            events.push({
                id: `tiet-khi-${transition.date}-${transition.term.index}`,
                kind: 'tiet_khi',
                date: transition.date,
                month: parseDatePart(transition.date, 1),
                day: parseDatePart(transition.date, 2),
                title: day?.insight?.tiet_khi?.name.vi ?? transition.term.name,
                subtitle: transition.term.season,
                lunarLabel: day ? lunarLabel(day) : undefined,
                category: 'tiet khi',
                major: true,
                insight: day?.insight?.tiet_khi,
                bundle: day,
            });
        }

        return events.sort((a, b) => a.date.localeCompare(b.date) || kindRank(a.kind) - kindRank(b.kind));
    }

    function createFestivalEvent(day: DayBundleDto, kind: 'holiday' | 'festival', insight: FestivalInsightDto | HolidayInsightDto): TimelineEvent {
        return {
            id: `${kind}-${dateKey(day)}-${slug(insight.names.vi[0] ?? insight.names.en[0] ?? kind)}`,
            kind,
            date: dateKey(day),
            month: day.solar.month,
            day: day.solar.day,
            title: insight.names.vi[0] ?? insight.names.en[0] ?? day.solar.date_string,
            subtitle: insight.names.en[0] ?? insight.category,
            lunarLabel: lunarLabel(day),
            category: insight.category,
            major: insight.is_major,
            insight,
            bundle: day,
        };
    }

    function createLunarCycleEvent(day: DayBundleDto): TimelineEvent {
        const isNewMoon = day.lunar.day === 1;
        return {
            id: `lunar-cycle-${dateKey(day)}-${day.lunar.month}-${day.lunar.day}`,
            kind: 'lunar_cycle',
            date: dateKey(day),
            month: day.solar.month,
            day: day.solar.day,
            title: isNewMoon ? `Mùng 1 tháng ${day.lunar.month}` : `Rằm tháng ${day.lunar.month}`,
            subtitle: isNewMoon ? 'New lunar month' : 'Full moon observance',
            lunarLabel: lunarLabel(day),
            category: 'lunar cycle',
            major: false,
            bundle: day,
        };
    }

    function pickSelectedEvent(events: TimelineEvent[], selectedId: string | null): TimelineEvent | null {
        if (events.length === 0) return null;
        return events.find((event) => event.id === selectedId) ?? events[0];
    }

    function dateKey(day: DayBundleDto): string {
        return `${day.solar.year}-${String(day.solar.month).padStart(2, '0')}-${String(day.solar.day).padStart(2, '0')}`;
    }

    function parseDatePart(date: string, index: number): number {
        return Number(date.split('-')[index] ?? 0);
    }

    function lunarLabel(day: DayBundleDto): string {
        return `${day.lunar.day}/${day.lunar.month}${day.lunar.is_leap_month ? ' nhuận' : ''} ÂL`;
    }

    function kindRank(kind: TimelineKind): number {
        return { holiday: 0, festival: 1, lunar_cycle: 2, tiet_khi: 3 }[kind];
    }

    function kindLabel(kind: TimelineKind): string {
        return {
            holiday: 'Holiday',
            festival: 'Festival',
            lunar_cycle: 'Mùng 1 / Rằm',
            tiet_khi: 'Tiết khí',
        }[kind];
    }

    function kindClass(kind: TimelineKind): string {
        return {
            holiday: 'badge-nen',
            festival: 'badge-evidence',
            lunar_cycle: 'badge-cothe',
            tiet_khi: 'badge-tranh',
        }[kind];
    }

    function hasLocalizedText(value?: LocalizedTextDto | null): value is LocalizedTextDto {
        return Boolean(value?.vi || value?.en);
    }

    function hasLocalizedList(value?: LocalizedListDto | null): value is LocalizedListDto {
        return Boolean(value && (value.vi.length > 0 || value.en.length > 0));
    }

    function localized(value?: LocalizedTextDto | null): string {
        return value?.vi || value?.en || '';
    }

    function localizedList(value?: LocalizedListDto | null): string[] {
        return value?.vi?.length ? value.vi : value?.en ?? [];
    }

    function isHolidayInsight(insight?: CulturalInsight | null): insight is HolidayInsightDto {
        return Boolean(insight && 'significance' in insight);
    }

    function isFestivalInsight(insight?: CulturalInsight | null): insight is FestivalInsightDto | HolidayInsightDto {
        return Boolean(insight && 'names' in insight);
    }

    function isTietKhiInsight(insight?: CulturalInsight | null): insight is TietKhiInsightDto {
        return Boolean(insight && 'agriculture' in insight);
    }

    function slug(value: string): string {
        return value
            .toLowerCase()
            .normalize('NFD')
            .replace(/[\u0300-\u036f]/g, '')
            .replace(/[^a-z0-9]+/g, '-')
            .replace(/^-|-$/g, '');
    }
</script>

<div class="p-8 h-full overflow-y-auto">
    {#if loading}
        <div class="flex h-full items-center justify-center">
            <span class="text-ink-light font-mono animate-pulse">Building season timeline...</span>
        </div>
    {:else if error}
        <div class="bg-ky/10 text-ky p-4 rounded-sm font-mono border border-ky/20">
            {error}
        </div>
    {:else}
        <div class="mb-8 border-b border-ink-border pb-6">
            <div class="flex flex-wrap items-end justify-between gap-4">
                <div>
                    <h2 class="text-3xl font-sans font-bold text-ink">Season Timeline</h2>
                    <p class="text-ink-light font-mono mt-1 text-sm">
                        {year} · {filteredEvents.length} / {allEvents.length} signals
                    </p>
                </div>
                <div class="grid grid-cols-4 gap-2 font-mono text-xs">
                    <span class="badge-evidence">Festivals {stats.festival}</span>
                    <span class="badge-nen">Holidays {stats.holiday}</span>
                    <span class="badge-cothe">Cycle {stats.lunarCycle}</span>
                    <span class="badge-tranh">Tiết khí {stats.tietKhi}</span>
                </div>
            </div>
        </div>

        <div class="mb-6 flex flex-wrap items-center gap-2 font-mono text-xs">
            {#each ['all', 'festival', 'holiday', 'lunar_cycle', 'tiet_khi'] as kind}
                <button
                    class="focus-ring border border-ink-border px-3 py-2 uppercase transition-colors"
                    class:bg-ink={selectedKind === kind}
                    class:text-parchment={selectedKind === kind}
                    class:hover:bg-ink-border={selectedKind !== kind}
                    onclick={() => selectedKind = kind as TimelineKind | 'all'}
                >
                    {kind === 'all' ? 'All' : kindLabel(kind as TimelineKind)}
                </button>
            {/each}
            <button
                class="focus-ring ml-auto border border-ink-border px-3 py-2 uppercase transition-colors"
                class:bg-ink={majorOnly}
                class:text-parchment={majorOnly}
                class:hover:bg-ink-border={!majorOnly}
                onclick={() => majorOnly = !majorOnly}
            >
                Major only
            </button>
        </div>

        <div class="grid grid-cols-1 2xl:grid-cols-[minmax(0,1fr)_minmax(380px,0.72fr)] gap-6">
            <section class="space-y-3">
                {#each filteredEvents as event}
                    <button
                        class="focus-ring w-full border border-ink-border bg-parchment-dark/30 p-3 text-left transition-colors hover:border-ink-light"
                        class:border-ink={selectedEvent?.id === event.id}
                        class:bg-parchment-dark={selectedEvent?.id === event.id}
                        onclick={() => selectedEventId = event.id}
                    >
                        <div class="grid grid-cols-[72px_minmax(0,1fr)_auto] items-start gap-4">
                            <div class="font-mono text-center">
                                <div class="text-xs uppercase text-ink-light">T{event.month}</div>
                                <div class="text-3xl font-bold leading-none">{event.day}</div>
                            </div>
                            <div class="min-w-0">
                                <div class="flex flex-wrap items-center gap-2">
                                    <h3 class="truncate text-lg font-bold">{event.title}</h3>
                                    {#if event.major}
                                        <span class="badge-nen">major</span>
                                    {/if}
                                </div>
                                <div class="mt-1 text-sm text-ink-light">{event.subtitle}</div>
                                <div class="mt-2 flex flex-wrap gap-2 font-mono text-xs">
                                    <span class={kindClass(event.kind)}>{kindLabel(event.kind)}</span>
                                    <span class="badge-cothe">{event.category}</span>
                                    {#if event.lunarLabel}
                                        <span class="badge-cothe">{event.lunarLabel}</span>
                                    {/if}
                                </div>
                            </div>
                            <div class="text-right font-mono text-xs text-ink-light">{event.date}</div>
                        </div>
                    </button>
                {:else}
                    <div class="card-dense text-center font-mono text-sm text-ink-light">
                        No matching timeline entries.
                    </div>
                {/each}
            </section>

            <aside class="space-y-4">
                {#if selectedEvent}
                    <div class="card-dense">
                        <div class="mb-4 flex flex-wrap items-start justify-between gap-3">
                            <div>
                                <div class={kindClass(selectedEvent.kind)}>{kindLabel(selectedEvent.kind)}</div>
                                <h3 class="mt-3 text-2xl font-bold">{selectedEvent.title}</h3>
                                <div class="mt-1 font-mono text-sm text-ink-light">
                                    {selectedEvent.date} · {selectedEvent.lunarLabel ?? 'solar term'}
                                </div>
                            </div>
                            {#if selectedEvent.major}
                                <span class="badge-nen">major</span>
                            {/if}
                        </div>

                        {#if isFestivalInsight(selectedEvent.insight)}
                            {#if hasLocalizedText(selectedEvent.insight.origin)}
                                <section class="border-t border-ink-border py-4">
                                    <h4 class="font-mono text-xs font-bold uppercase text-ink-light">Origin</h4>
                                    <p class="mt-2 text-sm leading-relaxed">{localized(selectedEvent.insight.origin)}</p>
                                </section>
                            {/if}

                            {#if isHolidayInsight(selectedEvent.insight) && hasLocalizedText(selectedEvent.insight.significance)}
                                <section class="border-t border-ink-border py-4">
                                    <h4 class="font-mono text-xs font-bold uppercase text-ink-light">Significance</h4>
                                    <p class="mt-2 text-sm leading-relaxed">{localized(selectedEvent.insight.significance)}</p>
                                </section>
                            {/if}

                            {#if hasLocalizedList(selectedEvent.insight.activities)}
                                <section class="border-t border-ink-border py-4">
                                    <h4 class="font-mono text-xs font-bold uppercase text-ink-light">Activities</h4>
                                    <div class="mt-2 flex flex-wrap gap-2">
                                        {#each localizedList(selectedEvent.insight.activities) as activity}
                                            <span class="badge-cothe">{activity}</span>
                                        {/each}
                                    </div>
                                </section>
                            {/if}

                            {#if isHolidayInsight(selectedEvent.insight) && hasLocalizedList(selectedEvent.insight.traditions)}
                                <section class="border-t border-ink-border py-4">
                                    <h4 class="font-mono text-xs font-bold uppercase text-ink-light">Traditions</h4>
                                    <ul class="mt-2 space-y-1 text-sm">
                                        {#each localizedList(selectedEvent.insight.traditions) as tradition}
                                            <li>{tradition}</li>
                                        {/each}
                                    </ul>
                                </section>
                            {/if}

                            {#if selectedEvent.insight.food.length > 0}
                                <section class="border-t border-ink-border py-4">
                                    <h4 class="font-mono text-xs font-bold uppercase text-ink-light">Food</h4>
                                    <div class="mt-2 space-y-2">
                                        {#each selectedEvent.insight.food.slice(0, 4) as food}
                                            <div class="border-l-2 border-ink-border pl-3 text-sm">
                                                <div class="font-bold">{localized(food.name)}</div>
                                                <div class="text-ink-light">{localized(food.description)}</div>
                                            </div>
                                        {/each}
                                    </div>
                                </section>
                            {/if}

                            {#if selectedEvent.insight.taboos.length > 0}
                                <section class="border-t border-ink-border py-4">
                                    <h4 class="font-mono text-xs font-bold uppercase text-ink-light">Taboos</h4>
                                    <div class="mt-2 space-y-2">
                                        {#each selectedEvent.insight.taboos.slice(0, 4) as taboo}
                                            <div class="text-sm">
                                                <span class="font-bold text-ky">{localized(taboo.action)}</span>
                                                <span class="text-ink-light"> · {localized(taboo.reason)}</span>
                                            </div>
                                        {/each}
                                    </div>
                                </section>
                            {/if}

                            {#if selectedEvent.insight.proverbs.length > 0}
                                <section class="border-t border-ink-border py-4">
                                    <h4 class="font-mono text-xs font-bold uppercase text-ink-light">Proverbs</h4>
                                    <div class="mt-2 space-y-2">
                                        {#each selectedEvent.insight.proverbs.slice(0, 3) as proverb}
                                            <div class="text-sm">
                                                <div class="font-bold">"{proverb.text}"</div>
                                                <div class="text-ink-light">{localized(proverb.meaning)}</div>
                                            </div>
                                        {/each}
                                    </div>
                                </section>
                            {/if}

                            {#if selectedEvent.insight.regions}
                                <section class="border-t border-ink-border pt-4">
                                    <h4 class="font-mono text-xs font-bold uppercase text-ink-light">Regions</h4>
                                    <div class="mt-2 grid grid-cols-1 gap-2 text-sm">
                                        <div><span class="font-bold">North</span> · {localized(selectedEvent.insight.regions.north)}</div>
                                        <div><span class="font-bold">Central</span> · {localized(selectedEvent.insight.regions.central)}</div>
                                        <div><span class="font-bold">South</span> · {localized(selectedEvent.insight.regions.south)}</div>
                                    </div>
                                </section>
                            {/if}
                        {:else if isTietKhiInsight(selectedEvent.insight)}
                            <section class="border-t border-ink-border py-4">
                                <h4 class="font-mono text-xs font-bold uppercase text-ink-light">Meaning</h4>
                                <p class="mt-2 text-sm leading-relaxed">{localized(selectedEvent.insight.meaning)}</p>
                            </section>
                            <section class="border-t border-ink-border py-4">
                                <h4 class="font-mono text-xs font-bold uppercase text-ink-light">Weather</h4>
                                <p class="mt-2 text-sm leading-relaxed">{localized(selectedEvent.insight.weather)}</p>
                            </section>
                            <section class="border-t border-ink-border py-4">
                                <h4 class="font-mono text-xs font-bold uppercase text-ink-light">Agriculture</h4>
                                <div class="mt-2 flex flex-wrap gap-2">
                                    {#each localizedList(selectedEvent.insight.agriculture) as item}
                                        <span class="badge-cothe">{item}</span>
                                    {/each}
                                </div>
                            </section>
                            <section class="border-t border-ink-border pt-4">
                                <h4 class="font-mono text-xs font-bold uppercase text-ink-light">Health</h4>
                                <div class="mt-2 flex flex-wrap gap-2">
                                    {#each localizedList(selectedEvent.insight.health) as item}
                                        <span class="badge-tranh">{item}</span>
                                    {/each}
                                </div>
                            </section>
                        {:else}
                            <section class="border-t border-ink-border py-4">
                                <h4 class="font-mono text-xs font-bold uppercase text-ink-light">Lunar Cycle</h4>
                                <p class="mt-2 text-sm leading-relaxed">
                                    {selectedEvent.bundle?.lunar.day === 1
                                        ? 'First day of the lunar month, commonly used for altar offerings, new intentions, and household observance.'
                                        : 'Full moon day, commonly used for vegetarian practice, temple visits, offerings, and reflection.'}
                                </p>
                            </section>
                            {#if selectedEvent.bundle?.tiet_khi}
                                <section class="border-t border-ink-border pt-4">
                                    <h4 class="font-mono text-xs font-bold uppercase text-ink-light">Current Tiết khí</h4>
                                    <p class="mt-2 text-sm text-ink-light">
                                        {selectedEvent.bundle.tiet_khi.name} · {selectedEvent.bundle.tiet_khi.description}
                                    </p>
                                </section>
                            {/if}
                        {/if}
                    </div>
                {/if}
            </aside>
        </div>
    {/if}
</div>
