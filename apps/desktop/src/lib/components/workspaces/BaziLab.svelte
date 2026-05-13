<script lang="ts">
    import { fetchBaziReport, fetchBaziDerivedReport } from '$lib/api/invoke';
    import type { BaziReportDto, BaziDerivedReportDto } from '$lib/api/types';

    let dateInput = "1990-01-01";
    let timeInput = "12:00";
    let genderInput = "m"; // "m" or "f"
    
    let report: BaziReportDto | null = null;
    let derived: BaziDerivedReportDto | null = null;
    let loading = false;
    let error: string | null = null;

    async function generateBazi() {
        if (!dateInput) return;
        loading = true;
        error = null;

        try {
            const [y, m, d] = dateInput.split('-').map(Number);
            let hour = 12;
            let minute = 0;
            if (timeInput) {
                const [h, min] = timeInput.split(':').map(Number);
                hour = h;
                minute = min;
            }

            const [baziRep, baziDer] = await Promise.all([
                fetchBaziReport(y, m, d, hour, minute, genderInput),
                fetchBaziDerivedReport(y, m, d, hour, minute, genderInput)
            ]);

            report = baziRep;
            derived = baziDer;
        } catch (e: unknown) {
            console.error("Failed to generate Bazi chart", e);
            error = e instanceof Error ? e.message : "Failed to load Bazi data";
        } finally {
            loading = false;
        }
    }
    
    function mapPillarKind(kind: string): string {
        const mapping: Record<string, string> = {
            'year': 'Năm',
            'month': 'Tháng',
            'day': 'Ngày',
            'hour': 'Giờ'
        };
        return mapping[kind] || kind;
    }
</script>

<div class="h-full flex flex-col overflow-hidden">
    <!-- Top Controls -->
    <div class="p-4 border-b border-ink-border bg-parchment-dark/50 flex flex-wrap items-end gap-4 shrink-0">
        <div>
            <label for="bazi_date" class="block text-xs font-mono uppercase text-ink-light mb-1">Birth Date (Solar)</label>
            <input type="date" id="bazi_date" bind:value={dateInput} class="bg-parchment border border-ink-border px-3 py-1 font-mono text-sm focus-ring" />
        </div>
        <div>
            <label for="bazi_time" class="block text-xs font-mono uppercase text-ink-light mb-1">Birth Time</label>
            <input type="time" id="bazi_time" bind:value={timeInput} class="bg-parchment border border-ink-border px-3 py-1 font-mono text-sm focus-ring" />
        </div>
        <div>
            <label for="bazi_gender" class="block text-xs font-mono uppercase text-ink-light mb-1">Gender</label>
            <select id="bazi_gender" bind:value={genderInput} class="bg-parchment border border-ink-border px-3 py-1.5 font-mono text-sm focus-ring h-[30px]">
                <option value="m">Nam</option>
                <option value="f">Nữ</option>
            </select>
        </div>
        <button onclick={generateBazi} class="bg-ink text-parchment px-4 py-1.5 font-mono text-sm uppercase tracking-wide hover:bg-ink-light transition-colors focus-ring h-[30px]">
            Generate Chart
        </button>
    </div>

    <!-- Scrollable Content -->
    <div class="p-8 flex-grow overflow-y-auto">
        {#if loading}
            <div class="flex h-full items-center justify-center">
                <span class="text-ink-light font-mono animate-pulse">Calculating Bazi...</span>
            </div>
        {:else if error}
            <div class="bg-ky/10 text-ky p-4 rounded font-mono border border-ky/20">
                {error}
            </div>
        {:else if report}
            <!-- Summary Header -->
            <div class="mb-12 border-b border-ink-border pb-8">
                <div class="flex items-end justify-between mb-6">
                    <div>
                        <h2 class="text-4xl font-sans font-bold text-ink">Bazi Chart</h2>
                        <p class="text-ink-light font-mono mt-1 text-lg">
                            Nhật Chủ: <span class="text-hoangdao font-bold">{report.chart.day_master.full}</span>
                        </p>
                    </div>
                </div>
                {#if report.advisory?.summary_vi}
                <div class="font-mono text-sm leading-relaxed p-4 bg-ink text-parchment italic">
                    {report.advisory.summary_vi}
                </div>
                {/if}
            </div>

            <div class="grid grid-cols-1 xl:grid-cols-3 gap-8">
                <!-- Left Column: The Chart -->
                <div class="xl:col-span-2 space-y-8">
                    
                    <!-- Four Pillars -->
                    <div>
                        <h3 class="text-2xl font-mono font-bold mb-4">Four Pillars (Tứ Trụ)</h3>
                        <div class="grid grid-cols-4 gap-4">
                            <!-- Render right to left usually, but left to right is okay for modern UI. Let's do Year -> Hour for l2r -->
                            {#each report.chart.pillars as pillar}
                            <div class="card-dense flex flex-col items-center p-0 overflow-hidden">
                                <div class="w-full text-center bg-ink text-parchment font-mono text-xs uppercase py-1 tracking-wider">
                                    {mapPillarKind(pillar.kind)}
                                </div>
                                <div class="p-4 w-full flex flex-col items-center">
                                    <div class="text-2xl font-bold font-sans text-ink mb-1">{pillar.can_chi.can}</div>
                                    <div class="text-2xl font-bold font-sans text-ink">{pillar.can_chi.chi}</div>
                                </div>
                                
                                <!-- Na Am -->
                                {#if pillar.na_am}
                                <div class="w-full text-center border-t border-ink-border py-2 text-xs font-mono text-ink-light">
                                    {pillar.na_am}
                                </div>
                                {/if}

                                <!-- Hidden Stems -->
                                <div class="w-full border-t border-ink-border p-2 bg-parchment-dark/50 min-h-[80px]">
                                    <div class="text-[10px] text-ink-light uppercase mb-1 text-center">Tàng Can</div>
                                    <div class="flex flex-col gap-1 items-center">
                                        {#each pillar.hidden_stems as hs}
                                            <div class="text-xs font-mono">
                                                {hs.stem_symbol} <span class="text-ink-light opacity-70">({hs.strength}%)</span>
                                            </div>
                                        {/each}
                                    </div>
                                </div>
                            </div>
                            {/each}
                        </div>
                    </div>

                    <!-- Interactions & Advisory -->
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <!-- Useful Gods -->
                        {#if report.advisory?.useful_god_analysis}
                        <div class="space-y-4">
                            <h4 class="font-mono font-bold text-nen uppercase border-b-2 border-nen pb-2">Dụng Thần (Useful Gods)</h4>
                            <div class="card-nen space-y-2">
                                {#if report.advisory.useful_god_analysis.favorable_elements.length > 0}
                                    <div class="text-sm font-sans"><span class="font-bold">Hỉ/Dụng:</span> {report.advisory.useful_god_analysis.favorable_elements.join(', ')}</div>
                                {/if}
                                {#if report.advisory.useful_god_analysis.unfavorable_elements.length > 0}
                                    <div class="text-sm font-sans text-tranh"><span class="font-bold">Kỵ/Cừu:</span> {report.advisory.useful_god_analysis.unfavorable_elements.join(', ')}</div>
                                {/if}
                                <div class="text-xs font-mono text-ink-light pt-2 border-t border-ink-border">
                                    {#each report.advisory.useful_god_analysis.reasons as reason}
                                        <div>• {reason}</div>
                                    {/each}
                                </div>
                            </div>
                        </div>
                        {/if}

                        <!-- Interactions -->
                        {#if report.analysis?.interactions && report.analysis.interactions.length > 0}
                        <div class="space-y-4">
                            <h4 class="font-mono font-bold text-evidence uppercase border-b-2 border-evidence pb-2">Hợp Hoá / Xung Khắc</h4>
                            <div class="flex flex-col gap-2">
                                {#each report.analysis.interactions as interaction}
                                    <div class="card-dense">
                                        <div class="font-bold text-sm mb-1">{interaction.summary_vi || interaction.kind}</div>
                                        <div class="text-xs text-ink-light font-mono">[{interaction.participants.join(' + ')}]</div>
                                    </div>
                                {/each}
                            </div>
                        </div>
                        {/if}
                    </div>

                </div>

                <!-- Right Column: Analysis Metrics -->
                <div class="space-y-8">
                    <!-- Day Master Strength -->
                    {#if report.analysis?.day_master_strength}
                    <div>
                        <h3 class="text-lg font-mono font-bold mb-3 uppercase tracking-wider text-ink-light">Cường Nhược</h3>
                        <div class="card-dense border-l-4 border-l-hoangdao">
                            <div class="text-2xl font-bold mb-2">{report.analysis.day_master_strength.label}</div>
                            <div class="text-xs font-mono text-ink-light space-y-1">
                                {#each report.analysis.day_master_strength.reasons as reason}
                                    <div>• {reason}</div>
                                {/each}
                            </div>
                        </div>
                    </div>
                    {/if}

                    <!-- Element Distribution -->
                    {#if report.analysis?.element_distribution}
                    <div>
                        <h3 class="text-lg font-mono font-bold mb-3 uppercase tracking-wider text-ink-light">Ngũ Hành</h3>
                        <div class="card-dense space-y-3">
                            {#each Object.entries(report.analysis.element_distribution) as [element, value]}
                                <div>
                                    <div class="flex justify-between text-xs font-mono mb-1 uppercase">
                                        <span>{element}</span>
                                        <span>{value}%</span>
                                    </div>
                                    <div class="w-full bg-parchment-dark rounded-full h-1.5 overflow-hidden">
                                        <div class="bg-ink h-1.5 rounded-full" style="width: {value}%"></div>
                                    </div>
                                </div>
                            {/each}
                        </div>
                    </div>
                    {/if}

                    <!-- Warnings -->
                    {#if report.advisory?.warnings && report.advisory.warnings.length > 0}
                    <div>
                        <h3 class="text-lg font-mono font-bold mb-3 uppercase tracking-wider text-tranh">Lưu Ý</h3>
                        <ul class="space-y-2">
                            {#each report.advisory.warnings as warning}
                                <li class="card-tranh text-sm font-sans">{warning}</li>
                            {/each}
                        </ul>
                    </div>
                    {/if}
                </div>
            </div>
        {:else}
            <!-- Empty state -->
            <div class="flex h-full items-center justify-center text-ink-light font-mono italic">
                Enter birth data and click Generate Chart to calculate your Bazi.
            </div>
        {/if}
    </div>
</div>
