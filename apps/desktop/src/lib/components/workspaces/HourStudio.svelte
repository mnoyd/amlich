<script lang="ts">
    import { selectedDate } from '$lib/stores';
    import { fetchHourSelectionReport } from '$lib/api/invoke';
    import type { HourSelectionReportDto } from '$lib/api/types';

    let report: HourSelectionReportDto | null = null;
    let loading = true;
    let error: string | null = null;

    $: {
        if ($selectedDate) {
            loadHourReport($selectedDate);
        }
    }

    async function loadHourReport(date: Date) {
        loading = true;
        error = null;
        try {
            const d = date.getDate();
            const m = date.getMonth() + 1;
            const y = date.getFullYear();
            report = await fetchHourSelectionReport(d, m, y);
        } catch (e: unknown) {
            console.error("Failed to load hour selection report", e);
            error = e instanceof Error ? e.message : "Failed to load hour data";
        } finally {
            loading = false;
        }
    }
</script>

<div class="p-8 h-full overflow-y-auto">
    {#if loading}
        <div class="flex h-full items-center justify-center">
            <span class="text-ink-light font-mono animate-pulse">Calculating Hours...</span>
        </div>
    {:else if error}
        <div class="bg-ky/10 text-ky p-4 rounded font-mono border border-ky/20">
            {error}
        </div>
    {:else if report}
        <!-- Header -->
        <div class="mb-12 border-b border-ink-border pb-8">
            <div class="flex items-end justify-between mb-8">
                <div>
                    <h2 class="text-4xl font-sans font-bold text-ink">Hour Studio</h2>
                    <p class="text-ink-light font-mono mt-1 text-lg">
                        Ngày {report.chart.lunar.day} tháng {report.chart.lunar.month} năm {report.chart.lunar.year} ÂL
                    </p>
                </div>
            </div>

            <!-- Advisory Summary -->
            {#if report.advisory}
            <div class="mt-8 font-mono text-sm leading-relaxed p-4 bg-ink text-parchment italic">
                {report.advisory.summary_vi}
            </div>
            {/if}
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
            <!-- Left: 12 Hours Timeline -->
            <div class="lg:col-span-2 space-y-6">
                <h3 class="text-2xl font-mono font-bold mb-4">Timeline</h3>
                
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    {#each report.chart.gio_hoang_dao.all_hours as hour}
                        <div class={hour.is_good ? 'card-nen' : 'card-dense'}>
                            <div class="flex justify-between items-start mb-2">
                                <div class="font-bold font-mono text-lg">{hour.hour_chi}</div>
                                <div class="text-xs font-mono text-ink-light">{hour.time_range}</div>
                            </div>
                            <div class="flex items-center space-x-2">
                                {#if hour.is_good}
                                    <span class="badge-nen">Hoàng Đạo</span>
                                {/if}
                                <span class="badge-cothe">{hour.star}</span>
                            </div>
                        </div>
                    {/each}
                </div>
            </div>

            <!-- Right: Advisory Details -->
            <div class="space-y-8">
                <h3 class="text-2xl font-mono font-bold mb-4">Advisory</h3>
                
                {#if report.advisory.best_windows && report.advisory.best_windows.length > 0}
                <div class="space-y-3">
                    <h4 class="font-mono font-bold text-nen uppercase border-b-2 border-nen pb-2">Best Windows</h4>
                    <ul class="space-y-2">
                        {#each report.advisory.best_windows as window}
                            <li class="card-nen text-sm font-mono">{window}</li>
                        {/each}
                    </ul>
                </div>
                {/if}

                {#if report.advisory.caution_windows && report.advisory.caution_windows.length > 0}
                <div class="space-y-3 mt-6">
                    <h4 class="font-mono font-bold text-tranh uppercase border-b-2 border-tranh pb-2">Caution Windows</h4>
                    <ul class="space-y-2">
                        {#each report.advisory.caution_windows as window}
                            <li class="card-tranh text-sm font-mono">{window}</li>
                        {/each}
                    </ul>
                </div>
                {/if}
            </div>
        </div>
    {/if}
</div>
