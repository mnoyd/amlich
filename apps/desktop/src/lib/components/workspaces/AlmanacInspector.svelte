<script lang="ts">
    import { selectedDate } from '$lib/stores';
    import { fetchDayBundle } from '$lib/api/invoke';
    import type {
        DayBundleDto,
        DayTabooDto,
        RuleEvidenceDto,
        StarRuleEvidenceDto,
        TabooInsightItemDto,
        TenGodsEntryInsightDto,
        ThapThanResultDto,
    } from '$lib/api/types';

    let bundle: DayBundleDto | null = null;
    let loading = true;
    let error: string | null = null;
    let loadToken = 0;

    $: {
        if ($selectedDate) {
            loadDayBundle($selectedDate);
        }
    }

    $: fortune = bundle?.day_fortune ?? null;
    $: insight = bundle?.insight ?? null;
    $: starRules = fortune?.stars.matched_rules ?? [];
    $: taboos = mergeTaboos(fortune?.taboos ?? [], insight?.taboos ?? []);
    $: tenGodRows = [
        tenGodRow('To year stem', fortune?.ten_gods?.to_year_stem, insight?.ten_gods?.to_year_stem),
        tenGodRow('To self', fortune?.ten_gods?.to_self, insight?.ten_gods?.to_self),
    ].filter((row) => row.value || row.insight);

    async function loadDayBundle(date: Date) {
        const token = ++loadToken;
        loading = true;
        error = null;

        try {
            const day = date.getDate();
            const month = date.getMonth() + 1;
            const year = date.getFullYear();
            const nextBundle = await fetchDayBundle(day, month, year);
            if (token === loadToken) bundle = nextBundle;
        } catch (e: unknown) {
            if (token !== loadToken) return;
            console.error('Failed to load almanac inspector bundle', e);
            error = e instanceof Error ? e.message : 'Failed to load almanac data';
            bundle = null;
        } finally {
            if (token === loadToken) loading = false;
        }
    }

    function evidenceLabel(evidence?: RuleEvidenceDto | null): string {
        if (!evidence) return 'derived';
        return [evidence.source_id, evidence.method, evidence.profile].filter(Boolean).join(' / ');
    }

    function starEvidenceLabel(rule: StarRuleEvidenceDto): string {
        return [rule.source_id, rule.method, rule.profile].filter(Boolean).join(' / ');
    }

    function qualityClass(value?: string | null): string {
        const normalized = value?.toLowerCase() ?? '';
        if (normalized.includes('cat') || normalized.includes('hoang') || normalized.includes('good')) return 'badge-nen';
        if (normalized.includes('sat') || normalized.includes('hung') || normalized.includes('hac') || normalized.includes('bad')) return 'badge-ky';
        return 'badge-cothe';
    }

    function severityClass(value?: string | null): string {
        const normalized = value?.toLowerCase() ?? '';
        if (normalized.includes('hard') || normalized.includes('high') || normalized.includes('manh')) return 'badge-ky';
        if (normalized.includes('soft') || normalized.includes('medium')) return 'badge-tranh';
        return 'badge-cothe';
    }

    function formatList(items?: string[] | null): string {
        if (!items || items.length === 0) return 'none';
        return items.join(', ');
    }

    function mergeTaboos(fortuneTaboos: DayTabooDto[], insightTaboos: TabooInsightItemDto[]) {
        return fortuneTaboos.map((taboo) => {
            const matchingInsight = insightTaboos.find((item) => item.name === taboo.name);
            return {
                ...taboo,
                insightReason: matchingInsight?.reason,
            };
        });
    }

    function tenGodRow(
        label: string,
        value?: ThapThanResultDto | null,
        insightValue?: TenGodsEntryInsightDto | null,
    ) {
        return {
            label,
            value,
            insight: insightValue,
        };
    }
</script>

<div class="p-8 h-full overflow-y-auto">
    {#if loading}
        <div class="flex h-full items-center justify-center">
            <span class="text-ink-light font-mono animate-pulse">Calculating almanac...</span>
        </div>
    {:else if error}
        <div class="bg-ky/10 text-ky p-4 rounded-sm font-mono border border-ky/20">
            {error}
        </div>
    {:else if bundle && fortune}
        <div class="mb-8 border-b border-ink-border pb-6">
            <div class="flex flex-wrap items-end justify-between gap-4">
                <div>
                    <h2 class="text-3xl font-sans font-bold text-ink">Almanac Inspector</h2>
                    <p class="text-ink-light font-mono mt-1 text-sm">
                        {bundle.solar.date_string} · {bundle.canchi?.day.full ?? 'Can Chi pending'} · profile {fortune.profile}
                    </p>
                </div>
                <div class="flex flex-wrap gap-2 font-mono text-xs">
                    <span class="badge-evidence">{fortune.ruleset_id}</span>
                    <span class="badge-cothe">v{fortune.ruleset_version}</span>
                    <span class={qualityClass(fortune.truc.quality)}>Truc {fortune.truc.name}</span>
                </div>
            </div>
            {#if insight?.na_am?.meaning.vi}
                <p class="mt-5 max-w-5xl text-sm leading-relaxed text-ink-light">
                    {insight.na_am.meaning.vi}
                </p>
            {/if}
        </div>

        <div class="grid grid-cols-1 2xl:grid-cols-[minmax(0,1.35fr)_minmax(360px,0.65fr)] gap-6">
            <div class="space-y-6">
                <section class="grid grid-cols-1 xl:grid-cols-3 gap-4">
                    <div class="card-dense">
                        <div class="text-xs font-mono uppercase text-ink-light mb-2">Nap Am / Day Element</div>
                        <div class="text-2xl font-bold">{fortune.day_element.na_am}</div>
                        <div class="mt-2 grid grid-cols-3 gap-2 text-xs font-mono">
                            <div>
                                <div class="text-ink-light uppercase">Element</div>
                                <div>{fortune.day_element.element}</div>
                            </div>
                            <div>
                                <div class="text-ink-light uppercase">Can</div>
                                <div>{fortune.day_element.can_element}</div>
                            </div>
                            <div>
                                <div class="text-ink-light uppercase">Chi</div>
                                <div>{fortune.day_element.chi_element}</div>
                            </div>
                        </div>
                        <div class="mt-3 badge-evidence">{evidenceLabel(fortune.day_element.evidence)}</div>
                    </div>

                    <div class="card-dense">
                        <div class="text-xs font-mono uppercase text-ink-light mb-2">Conflict / Tuoi Xung</div>
                        <div class="text-2xl font-bold">{fortune.conflict.opposing_chi}</div>
                        <div class="text-sm text-ink-light">{fortune.conflict.opposing_con_giap}</div>
                        <div class="mt-3 text-xs font-mono">Tuoi xung: {formatList(fortune.conflict.tuoi_xung)}</div>
                        <div class="mt-2 text-xs font-mono">Sat huong: {fortune.conflict.sat_huong}</div>
                        <div class="mt-3 badge-evidence">{evidenceLabel(fortune.conflict.evidence)}</div>
                    </div>

                    <div class="card-dense">
                        <div class="text-xs font-mono uppercase text-ink-light mb-2">Travel Directions</div>
                        <div class="grid grid-cols-3 gap-3 font-mono">
                            <div>
                                <div class="text-xs text-ink-light uppercase">Xuat hanh</div>
                                <div class="font-bold">{fortune.travel.xuat_hanh_huong}</div>
                            </div>
                            <div>
                                <div class="text-xs text-ink-light uppercase">Tai than</div>
                                <div class="font-bold">{fortune.travel.tai_than}</div>
                            </div>
                            <div>
                                <div class="text-xs text-ink-light uppercase">Hy than</div>
                                <div class="font-bold">{fortune.travel.hy_than}</div>
                            </div>
                        </div>
                        <div class="mt-3 badge-evidence">{evidenceLabel(fortune.travel.evidence)}</div>
                    </div>
                </section>

                <section class="grid grid-cols-1 xl:grid-cols-2 gap-4">
                    <div class="card-dense">
                        <div class="flex items-start justify-between gap-4 mb-4">
                            <div>
                                <h3 class="text-xl font-mono font-bold">Stars</h3>
                                <p class="text-xs text-ink-light font-mono">{fortune.stars.star_system ?? 'default star system'}</p>
                            </div>
                            {#if fortune.stars.day_star}
                                <span class={qualityClass(fortune.stars.day_star.quality)}>
                                    {fortune.stars.day_star.name}
                                </span>
                            {/if}
                        </div>

                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                            <div>
                                <div class="text-xs font-mono uppercase text-nen border-b border-nen/30 pb-1 mb-2">Cat tinh</div>
                                <div class="flex flex-wrap gap-2">
                                    {#each fortune.stars.cat_tinh as star}
                                        <span class="badge-nen">{star}</span>
                                    {/each}
                                </div>
                            </div>
                            <div>
                                <div class="text-xs font-mono uppercase text-ky border-b border-ky/30 pb-1 mb-2">Sat tinh</div>
                                <div class="flex flex-wrap gap-2">
                                    {#each fortune.stars.sat_tinh as star}
                                        <span class="badge-ky">{star}</span>
                                    {/each}
                                </div>
                            </div>
                        </div>

                        {#if starRules.length > 0}
                            <div class="mt-5 space-y-2">
                                {#each starRules as rule}
                                    <div class="grid grid-cols-[1fr_auto] gap-3 border-t border-ink-border pt-2 text-xs font-mono">
                                        <div>
                                            <span class="font-bold">{rule.name}</span>
                                            <span class="text-ink-light"> · {rule.category}</span>
                                        </div>
                                        <span class={qualityClass(rule.quality)}>{rule.quality}</span>
                                        <div class="col-span-2 text-ink-light">{starEvidenceLabel(rule)}</div>
                                    </div>
                                {/each}
                            </div>
                        {/if}
                    </div>

                    <div class="card-dense">
                        <h3 class="text-xl font-mono font-bold mb-4">Day Deity / Truc</h3>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                            <div>
                                <div class="text-xs font-mono uppercase text-ink-light mb-1">Day Deity</div>
                                {#if fortune.day_deity}
                                    <div class="text-lg font-bold">{fortune.day_deity.name}</div>
                                    <span class={qualityClass(fortune.day_deity.classification)}>{fortune.day_deity.classification}</span>
                                    {#if insight?.day_deity?.classification_meaning.vi}
                                        <p class="mt-3 text-sm text-ink-light leading-relaxed">{insight.day_deity.classification_meaning.vi}</p>
                                    {/if}
                                    <div class="mt-3 badge-evidence">{evidenceLabel(fortune.day_deity.evidence)}</div>
                                {:else}
                                    <div class="text-sm text-ink-light italic">No day deity for this profile.</div>
                                {/if}
                            </div>
                            <div>
                                <div class="text-xs font-mono uppercase text-ink-light mb-1">Truc</div>
                                <div class="text-lg font-bold">{fortune.truc.index}. {fortune.truc.name}</div>
                                <span class={qualityClass(fortune.truc.quality)}>{fortune.truc.quality}</span>
                                {#if insight?.truc?.meaning.vi}
                                    <p class="mt-3 text-sm text-ink-light leading-relaxed">{insight.truc.meaning.vi}</p>
                                {/if}
                                <div class="mt-3 badge-evidence">{evidenceLabel(fortune.truc.evidence)}</div>
                            </div>
                        </div>
                    </div>
                </section>

                <section class="grid grid-cols-1 xl:grid-cols-2 gap-4">
                    <div class="card-dense">
                        <h3 class="text-xl font-mono font-bold mb-4">Xung Hop Hai Hinh</h3>
                        <div class="grid grid-cols-2 md:grid-cols-3 gap-3 text-sm font-mono">
                            <div>
                                <div class="text-xs uppercase text-ink-light">Luc xung</div>
                                <div>{fortune.xung_hop.luc_xung}</div>
                            </div>
                            <div>
                                <div class="text-xs uppercase text-ink-light">Tam hop</div>
                                <div>{formatList(fortune.xung_hop.tam_hop)}</div>
                            </div>
                            <div>
                                <div class="text-xs uppercase text-ink-light">Tu hanh xung</div>
                                <div>{formatList(fortune.xung_hop.tu_hanh_xung)}</div>
                            </div>
                            <div>
                                <div class="text-xs uppercase text-ink-light">Luc hop</div>
                                <div>{fortune.xung_hop.liu_he ?? 'none'}</div>
                            </div>
                            <div>
                                <div class="text-xs uppercase text-ink-light">Tuong hai</div>
                                <div>{fortune.xung_hop.xiang_hai ?? 'none'}</div>
                            </div>
                            <div>
                                <div class="text-xs uppercase text-ink-light">Tuong hinh</div>
                                <div>{formatList(fortune.xung_hop.xiang_xing)}</div>
                            </div>
                        </div>
                    </div>

                    <div class="card-dense">
                        <h3 class="text-xl font-mono font-bold mb-4">Tang Can / Ten Gods</h3>
                        {#if fortune.tang_can}
                            <div class="grid grid-cols-3 gap-3 text-sm font-mono mb-5">
                                <div>
                                    <div class="text-xs uppercase text-ink-light">Main</div>
                                    <div class="font-bold">{fortune.tang_can.main}</div>
                                    <div class="text-ink-light">{fortune.tang_can.strength[0]}</div>
                                </div>
                                <div>
                                    <div class="text-xs uppercase text-ink-light">Central</div>
                                    <div class="font-bold">{fortune.tang_can.central}</div>
                                    <div class="text-ink-light">{fortune.tang_can.strength[1]}</div>
                                </div>
                                <div>
                                    <div class="text-xs uppercase text-ink-light">Residual</div>
                                    <div class="font-bold">{fortune.tang_can.residual}</div>
                                    <div class="text-ink-light">{fortune.tang_can.strength[2]}</div>
                                </div>
                            </div>
                        {/if}

                        {#if tenGodRows.length > 0}
                            <div class="space-y-3">
                                {#each tenGodRows as row}
                                    <div class="border-t border-ink-border pt-3">
                                        <div class="flex flex-wrap items-center justify-between gap-2">
                                            <div class="text-xs font-mono uppercase text-ink-light">{row.label}</div>
                                            {#if row.value}
                                                <span class="badge-evidence">{row.value.label}</span>
                                            {/if}
                                        </div>
                                        {#if row.insight}
                                            <div class="mt-1 font-bold">{row.insight.name.vi}</div>
                                            <p class="mt-1 text-sm text-ink-light leading-relaxed">{row.insight.meaning.vi}</p>
                                        {:else if row.value}
                                            <div class="mt-1 text-sm font-mono">{row.value.relation} · {row.value.same_polarity ? 'same polarity' : 'opposite polarity'}</div>
                                        {/if}
                                    </div>
                                {/each}
                            </div>
                        {:else}
                            <div class="text-sm text-ink-light italic">No ten-gods mapping for this day bundle.</div>
                        {/if}
                    </div>
                </section>

                <section class="card-dense">
                    <div class="flex items-center justify-between gap-4 mb-4">
                        <h3 class="text-xl font-mono font-bold">Taboos</h3>
                        <span class="badge-ky">{taboos.length}</span>
                    </div>
                    {#if taboos.length > 0}
                        <div class="grid grid-cols-1 lg:grid-cols-2 gap-3">
                            {#each taboos as taboo}
                                <div class="border border-ky/20 bg-ky/5 p-3">
                                    <div class="flex items-start justify-between gap-3">
                                        <div class="font-bold">{taboo.name}</div>
                                        <span class={severityClass(taboo.severity)}>{taboo.severity}</span>
                                    </div>
                                    <p class="mt-2 text-sm text-ink-light leading-relaxed">{taboo.insightReason ?? taboo.reason}</p>
                                    <div class="mt-3 text-xs font-mono text-ink-light">{taboo.rule_id} · {evidenceLabel(taboo.evidence)}</div>
                                </div>
                            {/each}
                        </div>
                    {:else}
                        <div class="text-sm text-ink-light italic">No taboos matched this day.</div>
                    {/if}
                </section>
            </div>

            <aside class="space-y-4">
                <section class="card-dense">
                    <h3 class="text-lg font-mono font-bold mb-3">Explanatory Insight</h3>
                    <div class="space-y-4 text-sm leading-relaxed">
                        {#if insight?.day_guidance}
                            <div>
                                <div class="text-xs font-mono uppercase text-nen mb-1">Good for</div>
                                <div>{formatList(insight.day_guidance.good_for.vi)}</div>
                            </div>
                            <div>
                                <div class="text-xs font-mono uppercase text-ky mb-1">Avoid for</div>
                                <div>{formatList(insight.day_guidance.avoid_for.vi)}</div>
                            </div>
                        {/if}
                        {#if insight?.travel}
                            <div class="border-t border-ink-border pt-3">
                                <div class="text-xs font-mono uppercase text-ink-light mb-1">Travel reading</div>
                                <div>Depart {insight.travel.xuat_hanh_huong}; wealth {insight.travel.tai_than}; joy {insight.travel.hy_than}.</div>
                            </div>
                        {/if}
                        {#if insight?.stars}
                            <div class="border-t border-ink-border pt-3">
                                <div class="text-xs font-mono uppercase text-ink-light mb-1">Star reading</div>
                                <div>{formatList(insight.stars.cat_tinh)} support; {formatList(insight.stars.sat_tinh)} pressure.</div>
                            </div>
                        {/if}
                    </div>
                </section>

                <section class="card-dense">
                    <h3 class="text-lg font-mono font-bold mb-3">Rule Evidence</h3>
                    <div class="space-y-2 text-xs font-mono">
                        <div class="border-b border-ink-border pb-2">
                            <div class="text-ink-light uppercase">Day element</div>
                            <div>{evidenceLabel(fortune.day_element.evidence)}</div>
                        </div>
                        <div class="border-b border-ink-border pb-2">
                            <div class="text-ink-light uppercase">Conflict</div>
                            <div>{evidenceLabel(fortune.conflict.evidence)}</div>
                        </div>
                        <div class="border-b border-ink-border pb-2">
                            <div class="text-ink-light uppercase">Travel</div>
                            <div>{evidenceLabel(fortune.travel.evidence)}</div>
                        </div>
                        <div class="border-b border-ink-border pb-2">
                            <div class="text-ink-light uppercase">Stars</div>
                            <div>{evidenceLabel(fortune.stars.evidence)}</div>
                        </div>
                        <div>
                            <div class="text-ink-light uppercase">Truc</div>
                            <div>{evidenceLabel(fortune.truc.evidence)}</div>
                        </div>
                    </div>
                </section>
            </aside>
        </div>
    {:else}
        <div class="flex h-full items-center justify-center text-ink-light font-mono italic">
            Almanac data is unavailable for the selected day.
        </div>
    {/if}
</div>
