<script lang="ts">
    import { selectedDate, userProfile } from '$lib/stores';
    import { fetchPersonalDayMatrixReport, fetchPersonalDayReport } from '$lib/api/invoke';
    import type {
        DirectionEntryDto,
        DomainDayBoostEntryDto,
        InitiationOpeningDecisionExportDto,
        PersonalDayMatrixReportDto,
        PersonalDayReportDto,
        PersonalHourEntryDto,
        PillarInteractionDto,
        ReasoningNoteDto,
        UnavailableSectionDto,
    } from '$lib/api/types';

    let birthYear = '';
    let birthMonth = '';
    let birthDay = '';
    let birthHour = '';
    let birthMinute = '';
    let gender = '';

    let report: PersonalDayReportDto | null = null;
    let matrix: PersonalDayMatrixReportDto | null = null;
    let loadingReport = true;
    let loadingMatrix = false;
    let error: string | null = null;
    let matrixError: string | null = null;
    let loadToken = 0;

    $: parsedBirthYear = parseOptionalInt(birthYear);
    $: parsedBirthMonth = parseOptionalInt(birthMonth);
    $: parsedBirthDay = parseOptionalInt(birthDay);
    $: parsedBirthHour = parseOptionalInt(birthHour);
    $: parsedBirthMinute = parseOptionalInt(birthMinute);
    $: hasBirthDate = parsedBirthYear !== null && parsedBirthMonth !== null && parsedBirthDay !== null;
    $: hasBirthTime = parsedBirthHour !== null && parsedBirthMinute !== null;
    $: canLoadMatrix = hasBirthDate && hasBirthTime;
    $: profileCompleteness = report?.computed_metrics.profile_completeness ?? 0;
    $: topDirections = sortDirections(matrix?.direction_merge?.entries ?? []);
    $: topDomains = sortDomains(matrix?.domain_day_boost?.entries ?? []);
    $: topPersonalHours = sortHours(matrix?.personal_hours?.hours ?? []);

    $: userProfile.set({
        birthYear: parsedBirthYear ?? undefined,
        birthMonth: parsedBirthMonth ?? undefined,
        birthDay: parsedBirthDay ?? undefined,
        birthHour: parsedBirthHour ?? undefined,
        birthMinute: parsedBirthMinute ?? undefined,
        gender: gender || undefined,
    });

    $: if ($selectedDate) {
        loadPersonalReports($selectedDate);
    }

    function parseOptionalInt(value: string): number | null {
        if (!value.trim()) return null;
        const parsed = Number.parseInt(value, 10);
        return Number.isFinite(parsed) ? parsed : null;
    }

    function currentDayParts(date: Date) {
        return {
            day: date.getDate(),
            month: date.getMonth() + 1,
            year: date.getFullYear(),
        };
    }

    async function loadPersonalReports(date: Date) {
        const token = ++loadToken;
        const { day, month, year } = currentDayParts(date);

        loadingReport = true;
        loadingMatrix = canLoadMatrix;
        error = null;
        matrixError = null;

        try {
            report = await fetchPersonalDayReport(
                day,
                month,
                year,
                parsedBirthYear ?? undefined,
                parsedBirthMonth ?? undefined,
                parsedBirthDay ?? undefined,
                gender || undefined,
            );
        } catch (e: unknown) {
            if (token !== loadToken) return;
            report = null;
            error = e instanceof Error ? e.message : 'Failed to load personal day data';
        } finally {
            if (token === loadToken) loadingReport = false;
        }

        if (token !== loadToken) return;

        if (!canLoadMatrix) {
            matrix = null;
            loadingMatrix = false;
            return;
        }

        if (
            parsedBirthYear === null ||
            parsedBirthMonth === null ||
            parsedBirthDay === null ||
            parsedBirthHour === null ||
            parsedBirthMinute === null
        ) {
            matrix = null;
            loadingMatrix = false;
            return;
        }

        try {
            matrix = await fetchPersonalDayMatrixReport(
                day,
                month,
                year,
                parsedBirthYear,
                parsedBirthMonth,
                parsedBirthDay,
                parsedBirthHour,
                parsedBirthMinute,
                gender || undefined,
            );
        } catch (e: unknown) {
            if (token !== loadToken) return;
            matrix = null;
            matrixError = e instanceof Error ? e.message : 'Failed to load personal matrix';
        } finally {
            if (token === loadToken) loadingMatrix = false;
        }
    }

    function bucketClass(bucket: string | null | undefined): string {
        switch (bucket) {
            case 'favorable':
                return 'badge-nen';
            case 'mixed':
                return 'badge-cothe';
            case 'cautious':
                return 'badge-tranh';
            case 'avoid':
                return 'badge-ky';
            default:
                return 'badge-evidence';
        }
    }

    function scoreClass(score: number): string {
        if (score >= 0.35) return 'text-nen';
        if (score <= -0.35) return 'text-ky';
        return 'text-ink';
    }

    function formatAxis(axis: string): string {
        return axis.replaceAll('_', ' ');
    }

    function formatElementInteraction(value: string): string {
        return value.replaceAll('_', ' ');
    }

    function noteSummaries(notes: ReasoningNoteDto[]): string[] {
        return notes.map((note) => note.summary_vi).filter(Boolean);
    }

    function watchNotes(decision: InitiationOpeningDecisionExportDto): string[] {
        return noteSummaries([
            ...decision.strongest_resistances,
            ...decision.override_factors,
            ...decision.conflict_notes,
        ]);
    }

    function sortHours(hours: PersonalHourEntryDto[]): PersonalHourEntryDto[] {
        return [...hours].sort((left, right) => right.score - left.score).slice(0, 5);
    }

    function sortDirections(entries: DirectionEntryDto[]): DirectionEntryDto[] {
        return [...entries].sort((left, right) => right.net_score - left.net_score).slice(0, 5);
    }

    function sortDomains(entries: DomainDayBoostEntryDto[]): DomainDayBoostEntryDto[] {
        return [...entries].sort((left, right) => right.boosted_score - left.boosted_score).slice(0, 4);
    }

    function pillarSummary(pillar: PillarInteractionDto): string {
        const relation = pillar.thap_than?.label ?? pillar.thap_than?.relation ?? 'relation pending';
        return `${pillar.pillar_canchi} · ${relation} · ${formatElementInteraction(pillar.element_interaction)}`;
    }

    function unavailableText(items: UnavailableSectionDto[]): string {
        return items.map((item) => `${item.section}: ${item.reason}`).join(' | ');
    }

    function axisBarWidth(axis: { score: number }): number {
        return Math.max(0, Math.min(100, ((axis.score + 1) / 2) * 100));
    }
</script>

<div class="h-full flex flex-col overflow-hidden">
    <div class="p-4 border-b border-ink-border bg-parchment-dark/50 shrink-0">
        <div class="flex flex-wrap items-end gap-4">
            <div>
                <label for="personal_birth_year" class="block text-xs font-mono uppercase text-ink-light mb-1">Birth Year</label>
                <input id="personal_birth_year" bind:value={birthYear} inputmode="numeric" placeholder="1990" class="bg-parchment border border-ink-border px-3 py-1 font-mono text-sm focus-ring w-24" />
            </div>
            <div>
                <label for="personal_birth_month" class="block text-xs font-mono uppercase text-ink-light mb-1">Month</label>
                <input id="personal_birth_month" bind:value={birthMonth} inputmode="numeric" placeholder="8" class="bg-parchment border border-ink-border px-3 py-1 font-mono text-sm focus-ring w-20" />
            </div>
            <div>
                <label for="personal_birth_day" class="block text-xs font-mono uppercase text-ink-light mb-1">Day</label>
                <input id="personal_birth_day" bind:value={birthDay} inputmode="numeric" placeholder="15" class="bg-parchment border border-ink-border px-3 py-1 font-mono text-sm focus-ring w-20" />
            </div>
            <div>
                <label for="personal_birth_hour" class="block text-xs font-mono uppercase text-ink-light mb-1">Hour</label>
                <input id="personal_birth_hour" bind:value={birthHour} inputmode="numeric" placeholder="9" class="bg-parchment border border-ink-border px-3 py-1 font-mono text-sm focus-ring w-20" />
            </div>
            <div>
                <label for="personal_birth_minute" class="block text-xs font-mono uppercase text-ink-light mb-1">Minute</label>
                <input id="personal_birth_minute" bind:value={birthMinute} inputmode="numeric" placeholder="30" class="bg-parchment border border-ink-border px-3 py-1 font-mono text-sm focus-ring w-20" />
            </div>
            <div>
                <label for="personal_gender" class="block text-xs font-mono uppercase text-ink-light mb-1">Gender</label>
                <select id="personal_gender" bind:value={gender} class="bg-parchment border border-ink-border px-3 py-1.5 font-mono text-sm focus-ring h-[30px]">
                    <option value="">Any</option>
                    <option value="male">Nam</option>
                    <option value="female">Nu</option>
                </select>
            </div>
        </div>
    </div>

    <div class="p-8 flex-grow overflow-y-auto">
        {#if loadingReport}
            <div class="flex h-full items-center justify-center">
                <span class="text-ink-light font-mono animate-pulse">Calculating personal day...</span>
            </div>
        {:else if error}
            <div class="bg-ky/10 text-ky p-4 rounded font-mono border border-ky/20">
                {error}
            </div>
        {:else if report}
            <div class="mb-10 border-b border-ink-border pb-8">
                <div class="flex flex-wrap items-end justify-between gap-4 mb-6">
                    <div>
                        <h2 class="text-4xl font-sans font-bold text-ink">Personal Lab</h2>
                        <p class="text-ink-light font-mono mt-1 text-lg">
                            {report.chart.solar.day}/{report.chart.solar.month}/{report.chart.solar.year} · tier {report.computed_metrics.tier}
                        </p>
                    </div>
                    <div class="text-right font-mono">
                        <div class="text-xs uppercase text-ink-light">Profile</div>
                        <div class="text-2xl font-bold text-hoangdao">{profileCompleteness}/4</div>
                    </div>
                </div>

                {#if report.advisory?.summary}
                    <div class="font-mono text-sm leading-relaxed p-4 bg-ink text-parchment italic">
                        {report.advisory.summary}
                    </div>
                {/if}
            </div>

            <div class="grid grid-cols-1 xl:grid-cols-3 gap-8">
                <div class="xl:col-span-2 space-y-8">
                    <section>
                        <div class="flex items-center justify-between mb-4">
                            <h3 class="text-2xl font-mono font-bold">Decision</h3>
                            {#if report.decision_export}
                                <span class={bucketClass(report.decision_export.recommendation_bucket)}>
                                    {report.decision_export.recommendation_bucket}
                                </span>
                            {/if}
                        </div>

                        {#if report.decision_export}
                            {@const decision = report.decision_export}
                            <div class="card-dense border-l-4 border-l-hoangdao space-y-5">
                                <div>
                                    <div class="flex flex-wrap items-center gap-2 mb-2">
                                        <span class="badge-evidence">{decision.confidence}</span>
                                        <span class="badge-cothe">{decision.semantic}</span>
                                        <span class="badge-cothe">{decision.context_is_clear ? 'clear context' : 'mixed context'}</span>
                                    </div>
                                    <p class="font-bold text-lg">{decision.primary_conclusion}</p>
                                </div>

                                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                    <div>
                                        <h4 class="font-mono font-bold text-nen uppercase border-b-2 border-nen pb-2 mb-3">Support</h4>
                                        <ul class="space-y-2">
                                            {#each noteSummaries(decision.strongest_supports) as note}
                                                <li class="text-sm text-ink-light">{note}</li>
                                            {/each}
                                        </ul>
                                    </div>
                                    <div>
                                        <h4 class="font-mono font-bold text-tranh uppercase border-b-2 border-tranh pb-2 mb-3">Resistance</h4>
                                        <ul class="space-y-2">
                                            {#each watchNotes(decision) as note}
                                                <li class="text-sm text-ink-light">{note}</li>
                                            {/each}
                                        </ul>
                                    </div>
                                </div>

                                {#if decision.suggested_hours.length || decision.suggested_directions.length}
                                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                        <div>
                                            <h4 class="font-mono font-bold text-evidence uppercase border-b-2 border-evidence pb-2 mb-3">Hours</h4>
                                            <div class="flex flex-wrap gap-2">
                                                {#each decision.suggested_hours as hour}
                                                    <span class="badge-nen">{hour}</span>
                                                {/each}
                                            </div>
                                        </div>
                                        <div>
                                            <h4 class="font-mono font-bold text-evidence uppercase border-b-2 border-evidence pb-2 mb-3">Directions</h4>
                                            <div class="flex flex-wrap gap-2">
                                                {#each decision.suggested_directions as direction}
                                                    <span class="badge-cothe">{direction}</span>
                                                {/each}
                                            </div>
                                        </div>
                                    </div>
                                {/if}
                            </div>
                        {:else}
                            <div class="card-dense">
                                <p class="font-bold">{report.summary}</p>
                                <p class="text-sm text-ink-light mt-2">Enter at least a birth date to unlock personal decision reasoning.</p>
                            </div>
                        {/if}
                    </section>

                    <section>
                        <h3 class="text-2xl font-mono font-bold mb-4">Day-Person Matrix</h3>
                        {#if loadingMatrix}
                            <div class="card-dense text-ink-light font-mono animate-pulse">Calculating matrix...</div>
                        {:else if matrixError}
                            <div class="bg-ky/10 text-ky p-4 rounded font-mono border border-ky/20">{matrixError}</div>
                        {:else if matrix}
                            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                                <div class="card-dense space-y-4">
                                    <div>
                                        <div class="text-xs font-mono uppercase text-ink-light">Day to Day Master</div>
                                        <div class="text-xl font-bold text-hoangdao">{matrix.day_person.day_canchi} -> {matrix.day_person.day_master}</div>
                                        <div class="text-sm text-ink-light">{matrix.day_person.day_to_day_master.label}</div>
                                    </div>
                                    <div class="space-y-2">
                                        {#each matrix.day_person.pillars as pillar}
                                            <div class="flex justify-between gap-3 border-t border-ink-border pt-2 text-sm">
                                                <span class="font-mono uppercase text-ink-light">{pillar.pillar}</span>
                                                <span class="text-right">{pillarSummary(pillar)}</span>
                                            </div>
                                        {/each}
                                    </div>
                                </div>

                                <div class="card-dense space-y-3">
                                    <div class="flex items-center justify-between">
                                        <div>
                                            <div class="text-xs font-mono uppercase text-ink-light">Element Resonance</div>
                                            <div class="text-xl font-bold">{matrix.element_resonance.day_element}</div>
                                        </div>
                                        <div class="font-mono text-lg font-bold {scoreClass(matrix.element_resonance.net_resonance)}">
                                            {matrix.element_resonance.net_resonance.toFixed(2)}
                                        </div>
                                    </div>
                                    {#each matrix.element_resonance.entries as entry}
                                        <div>
                                            <div class="flex justify-between text-xs font-mono mb-1 uppercase">
                                                <span>{entry.element}</span>
                                                <span>{entry.effective_resonance.toFixed(2)}</span>
                                            </div>
                                            <div class="w-full bg-parchment-dark rounded-full h-1.5 overflow-hidden">
                                                <div class="bg-ink h-1.5 rounded-full" style="width: {Math.max(0, Math.min(100, entry.effective_resonance * 100))}%"></div>
                                            </div>
                                        </div>
                                    {/each}
                                </div>
                            </div>
                        {:else}
                            <div class="card-dense text-sm text-ink-light">
                                Enter birth date and exact birth time to calculate day-person, element, hour, direction, and domain matrices.
                            </div>
                        {/if}
                    </section>
                </div>

                <aside class="space-y-8">
                    {#if report.decision_export?.axis_scores.length}
                        <section>
                            <h3 class="text-lg font-mono font-bold mb-3 uppercase tracking-wider text-ink-light">Axis Scores</h3>
                            <div class="card-dense space-y-3">
                                {#each report.decision_export.axis_scores as axis (axis.axis)}
                                    <div>
                                        <div class="flex justify-between text-xs font-mono mb-1 uppercase">
                                            <span>{formatAxis(axis.axis)}</span>
                                            <span class={scoreClass(axis.score)}>{axis.score.toFixed(2)}</span>
                                        </div>
                                        <div class="w-full bg-parchment-dark rounded-full h-1.5 overflow-hidden">
                                            <div class="bg-hoangdao h-1.5 rounded-full" style="width: {axisBarWidth(axis)}%"></div>
                                        </div>
                                        {#if axis.strongest_summary_vi}
                                            <p class="text-xs text-ink-light mt-1">{axis.strongest_summary_vi}</p>
                                        {/if}
                                    </div>
                                {/each}
                            </div>
                        </section>
                    {/if}

                    {#if topPersonalHours.length}
                        <section>
                            <h3 class="text-lg font-mono font-bold mb-3 uppercase tracking-wider text-ink-light">Personal Hours</h3>
                            <div class="space-y-2">
                                {#each topPersonalHours as hour}
                                    <div class={hour.is_hoang_dao ? 'card-nen' : 'card-dense'}>
                                        <div class="flex justify-between text-sm font-bold">
                                            <span>{hour.chi} · {hour.canchi}</span>
                                            <span>{hour.score}</span>
                                        </div>
                                        <div class="text-xs text-ink-light font-mono">{hour.time_range} · {hour.star_name}</div>
                                    </div>
                                {/each}
                            </div>
                        </section>
                    {/if}

                    {#if topDirections.length}
                        <section>
                            <h3 class="text-lg font-mono font-bold mb-3 uppercase tracking-wider text-ink-light">Directions</h3>
                            <div class="space-y-2">
                                {#each topDirections as direction}
                                    <div class="card-dense">
                                        <div class="flex justify-between font-bold text-sm">
                                            <span>{direction.direction}</span>
                                            <span class={direction.net_score >= 0 ? 'text-nen' : 'text-ky'}>{direction.net_score}</span>
                                        </div>
                                        <div class="text-xs text-ink-light font-mono">{direction.signals.join(' · ')}</div>
                                    </div>
                                {/each}
                            </div>
                        </section>
                    {/if}

                    {#if topDomains.length}
                        <section>
                            <h3 class="text-lg font-mono font-bold mb-3 uppercase tracking-wider text-ink-light">Domain Boost</h3>
                            <div class="space-y-2">
                                {#each topDomains as domain}
                                    <div class="card-dense">
                                        <div class="flex justify-between font-bold text-sm">
                                            <span>{domain.domain}</span>
                                            <span>{domain.boosted_score.toFixed(1)}</span>
                                        </div>
                                        <div class="text-xs text-ink-light font-mono">
                                            base {domain.base_score.toFixed(1)} · day {domain.day_modifier.toFixed(1)} · han {domain.han_penalty.toFixed(1)}
                                        </div>
                                    </div>
                                {/each}
                            </div>
                        </section>
                    {/if}

                    {#if report.analysis.unavailable_sections.length || matrix?.unavailable_sections.length}
                        <section>
                            <h3 class="text-lg font-mono font-bold mb-3 uppercase tracking-wider text-tranh">Unavailable</h3>
                            <div class="card-tranh text-sm text-ink-light">
                                {#if report.analysis.unavailable_sections.length}
                                    <p>{unavailableText(report.analysis.unavailable_sections)}</p>
                                {/if}
                                {#if matrix?.unavailable_sections.length}
                                    <p class="mt-2">{unavailableText(matrix.unavailable_sections)}</p>
                                {/if}
                            </div>
                        </section>
                    {/if}
                </aside>
            </div>
        {/if}
    </div>
</div>
