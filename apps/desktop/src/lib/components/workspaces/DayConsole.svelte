<script lang="ts">
    import { selectedDate } from '$lib/stores';
    import { fetchDayBundle } from '$lib/api/invoke';
    import type { DayBundleDto, SynthesizedRecommendationDto } from '$lib/api/types';
    import { onMount } from 'svelte';

    let bundle: DayBundleDto | null = null;
    let loading = true;
    let error: string | null = null;

    $: {
        if ($selectedDate) {
            loadDayBundle($selectedDate);
        }
    }

    async function loadDayBundle(date: Date) {
        loading = true;
        error = null;
        try {
            const d = date.getDate();
            const m = date.getMonth() + 1;
            const y = date.getFullYear();
            bundle = await fetchDayBundle(d, m, y);
        } catch (e: any) {
            console.error("Failed to load day bundle", e);
            error = e.message || "Failed to load data for selected date";
        } finally {
            loading = false;
        }
    }

    function groupRecommendations(activities: SynthesizedRecommendationDto[] | undefined) {
        if (!activities) return { Nen: [], CoThe: [], Tranh: [], KyManh: [] };
        const grouped = {
            Nen: activities.filter(a => a.bucket === 'Nen'),
            CoThe: activities.filter(a => a.bucket === 'CoThe'),
            Tranh: activities.filter(a => a.bucket === 'Tranh'),
            KyManh: activities.filter(a => a.bucket === 'KyManh'),
        };
        return grouped;
    }

    $: groupedRecs = groupRecommendations(bundle?.daily_recommendations?.activities);
</script>

<div class="p-8 h-full overflow-y-auto">
    {#if loading}
        <div class="flex h-full items-center justify-center">
            <span class="text-ink-light font-mono animate-pulse">Calculating...</span>
        </div>
    {:else if error}
        <div class="bg-ky/10 text-ky p-4 rounded font-mono border border-ky/20">
            {error}
        </div>
    {:else if bundle}
        <!-- Top Section: Identity -->
        <div class="mb-12 border-b border-ink-border pb-8">
            <div class="flex items-end justify-between mb-8">
                <div>
                    <h2 class="text-4xl font-sans font-bold text-ink">
                        {bundle.solar.day} / {bundle.solar.month} / {bundle.solar.year}
                    </h2>
                    <p class="text-ink-light font-mono mt-1 text-lg">
                        Ngày {bundle.lunar.day} tháng {bundle.lunar.month} {bundle.lunar.is_leap_month ? '(Nhuận) ' : ''}năm {bundle.lunar.year} ÂL
                    </p>
                </div>
                {#if bundle.tiet_khi}
                <div class="text-right">
                    <div class="inline-block bg-hoangdao/20 text-hoangdao border border-hoangdao/30 px-3 py-1 font-mono text-sm uppercase tracking-widest rounded-sm">
                        {bundle.tiet_khi.name}
                    </div>
                </div>
                {/if}
            </div>

            <!-- Can Chi Triad -->
            {#if bundle.canchi}
            <div class="grid grid-cols-3 gap-6 font-mono">
                <div class="p-4 bg-parchment-dark/50 border border-ink-border text-center">
                    <div class="text-xs text-ink-light uppercase tracking-wider mb-2 border-b border-ink-border pb-2">Năm</div>
                    <div class="text-xl font-bold">{bundle.canchi.year.full}</div>
                </div>
                <div class="p-4 bg-parchment-dark/50 border border-ink-border text-center">
                    <div class="text-xs text-ink-light uppercase tracking-wider mb-2 border-b border-ink-border pb-2">Tháng</div>
                    <div class="text-xl font-bold">{bundle.canchi.month.full}</div>
                </div>
                <div class="p-4 bg-parchment-dark/50 border border-ink-border text-center relative overflow-hidden">
                    <div class="absolute inset-0 bg-hoangdao/5 pointer-events-none"></div>
                    <div class="text-xs text-ink-light uppercase tracking-wider mb-2 border-b border-ink-border pb-2 relative z-10">Ngày</div>
                    <div class="text-xl font-bold text-hoangdao relative z-10">{bundle.canchi.day.full}</div>
                </div>
            </div>
            {/if}
            
            {#if bundle.daily_recommendations?.summary_vi}
            <div class="mt-8 font-mono text-sm leading-relaxed p-4 bg-ink text-parchment italic">
                {bundle.daily_recommendations.summary_vi}
            </div>
            {/if}
        </div>

        <!-- Activity Board -->
        <h3 class="text-2xl font-mono font-bold mb-6">Activity Board</h3>
        
        <div class="grid grid-cols-2 lg:grid-cols-4 gap-6 font-sans">
            <!-- NÊN -->
            <div class="space-y-4">
                <h4 class="font-mono font-bold text-nen uppercase border-b-2 border-nen pb-2 flex items-center justify-between">
                    Nên
                    <span class="text-xs bg-nen/10 px-2 py-0.5 rounded">{groupedRecs.Nen.length}</span>
                </h4>
                {#each groupedRecs.Nen as rec}
                    <div class="border border-ink-border p-3 bg-parchment-dark/30 hover:border-nen/50 transition-colors">
                        <div class="font-bold text-sm mb-2">{rec.label.vi}</div>
                        {#if rec.reasons.length > 0}
                            <div class="text-xs text-ink-light border-t border-ink-border pt-2 mt-2">
                                {rec.reasons[0].summary_vi}
                            </div>
                        {/if}
                    </div>
                {/each}
            </div>

            <!-- CÓ THỂ -->
            <div class="space-y-4">
                <h4 class="font-mono font-bold text-ink-light uppercase border-b-2 border-ink-border pb-2 flex items-center justify-between">
                    Có thể
                    <span class="text-xs bg-ink/5 px-2 py-0.5 rounded">{groupedRecs.CoThe.length}</span>
                </h4>
                {#each groupedRecs.CoThe as rec}
                    <div class="border border-ink-border p-3 bg-parchment-dark/30">
                        <div class="font-bold text-sm mb-2">{rec.label.vi}</div>
                        {#if rec.reasons.length > 0}
                            <div class="text-xs text-ink-light border-t border-ink-border pt-2 mt-2">
                                {rec.reasons[0].summary_vi}
                            </div>
                        {/if}
                    </div>
                {/each}
            </div>

            <!-- TRÁNH -->
            <div class="space-y-4">
                <h4 class="font-mono font-bold text-orange-600 uppercase border-b-2 border-orange-600 pb-2 flex items-center justify-between">
                    Tránh
                    <span class="text-xs bg-orange-600/10 px-2 py-0.5 rounded">{groupedRecs.Tranh.length}</span>
                </h4>
                {#each groupedRecs.Tranh as rec}
                    <div class="border border-ink-border p-3 bg-parchment-dark/30">
                        <div class="font-bold text-sm mb-2">{rec.label.vi}</div>
                        {#if rec.reasons.length > 0}
                            <div class="text-xs text-ink-light border-t border-ink-border pt-2 mt-2">
                                {rec.reasons[0].summary_vi}
                            </div>
                        {/if}
                    </div>
                {/each}
            </div>

            <!-- KỴ MẠNH -->
            <div class="space-y-4">
                <h4 class="font-mono font-bold text-ky uppercase border-b-2 border-ky pb-2 flex items-center justify-between">
                    Kỵ Mạnh
                    <span class="text-xs bg-ky/10 px-2 py-0.5 rounded">{groupedRecs.KyManh.length}</span>
                </h4>
                {#each groupedRecs.KyManh as rec}
                    <div class="border border-ink-border p-3 bg-ky/5 border-ky/30 hover:border-ky/50 transition-colors">
                        <div class="font-bold text-sm mb-2 text-ky">{rec.label.vi}</div>
                        {#if rec.reasons.length > 0}
                            <div class="text-xs text-ky/80 border-t border-ky/20 pt-2 mt-2">
                                {rec.reasons[0].summary_vi}
                            </div>
                        {/if}
                    </div>
                {/each}
            </div>
        </div>
    {/if}
</div>
