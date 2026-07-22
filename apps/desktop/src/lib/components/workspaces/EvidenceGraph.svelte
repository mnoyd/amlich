<script lang="ts">
    import { selectedDate, userProfile } from '$lib/stores';
    import type { UserProfile } from '$lib/stores';
    import { fetchPersonalDayReport } from '$lib/api/invoke';
    import type {
        DecisionConfidenceDto,
        InitiationOpeningDecisionExportDto,
        InitiationRecommendationBucketDto,
        PersonalDayReportDto,
        ReasoningConclusionSemanticDto,
        ReasoningEvidenceEnvelopeDto,
        ReasoningEvidenceSourceFamilyDto,
        ReasoningNoteDto,
    } from '$lib/api/types';

    type Lens = 'vi_sao' | 'yeu_to' | 'truc' | 'nguon' | 'dev';

    const lenses: { id: Lens; label: string; dev?: boolean }[] = [
        { id: 'vi_sao', label: 'Vì Sao' },
        { id: 'yeu_to', label: 'Yếu Tố' },
        { id: 'truc', label: 'Trục' },
        { id: 'nguon', label: 'Nguồn' },
        { id: 'dev', label: 'Dev', dev: true },
    ];

    let birthYear = '';
    let birthMonth = '';
    let birthDay = '';
    let gender = '';

    let report: PersonalDayReportDto | null = null;
    let loading = true;
    let error: string | null = null;
    let loadToken = 0;
    let activeLens: Lens = 'vi_sao';
    let profileInput: UserProfile = {};

    $: parsedBirthYear = parseOptionalInt(birthYear);
    $: parsedBirthMonth = parseOptionalInt(birthMonth);
    $: parsedBirthDay = parseOptionalInt(birthDay);

    $: profileInput = {
        birthYear: parsedBirthYear ?? undefined,
        birthMonth: parsedBirthMonth ?? undefined,
        birthDay: parsedBirthDay ?? undefined,
        gender: gender || undefined,
    };

    $: userProfile.set(profileInput);

    $: if ($selectedDate) {
        loadReport($selectedDate, profileInput);
    }

    const bucketLabel: Record<InitiationRecommendationBucketDto, string> = {
        avoid: 'Tránh',
        cautious: 'Cẩn trọng',
        mixed: 'Phức tạp',
        favorable: 'Thuận lợi',
    };

    const bucketClass: Record<InitiationRecommendationBucketDto, string> = {
        avoid: 'badge-ky',
        cautious: 'badge-tranh',
        mixed: 'badge-cothe',
        favorable: 'badge-nen',
    };

    const semanticLabel: Record<ReasoningConclusionSemanticDto, string> = {
        override_avoid: 'Ghi đè → Tránh',
        override_cautious: 'Ghi đè → Cẩn trọng',
        conflicted_cautious: 'Xung đột → Cẩn trọng',
        resistance_led_cautious: 'Kháng cự dẫn → Cẩn trọng',
        favorable_clear: 'Thuận lợi rõ',
        favorable_contextual: 'Thuận lợi tuỳ ngữ cảnh',
    };

    const confidenceLabel: Record<DecisionConfidenceDto, string> = {
        low: 'tin cậy thấp',
        medium: 'tin cậy vừa',
        high: 'tin cậy cao',
    };

    const sourceFamilyLabel: Record<ReasoningEvidenceSourceFamilyDto, string> = {
        snapshot: 'Snapshot',
        interaction: 'Tương tác',
        bazi: 'Bazi',
        axis: 'Trục',
        almanac_rule: 'Quy tắc lịch',
        insight: 'Insight',
        derived: 'Phái sinh',
    };

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

    async function loadReport(date: Date, profile: UserProfile) {
        const token = ++loadToken;
        const { day, month, year } = currentDayParts(date);

        loading = true;
        error = null;

        try {
            report = await fetchPersonalDayReport(
                day,
                month,
                year,
                profile.birthYear,
                profile.birthMonth,
                profile.birthDay,
                profile.gender,
            );
        } catch (e: unknown) {
            if (token !== loadToken) return;
            report = null;
            error = e instanceof Error ? e.message : 'Failed to load evidence graph data';
        } finally {
            if (token === loadToken) loading = false;
        }
    }

    function noteFamilies(note: ReasoningNoteDto): ReasoningEvidenceSourceFamilyDto[] {
        const seen = new Set<ReasoningEvidenceSourceFamilyDto>();
        for (const envelope of note.provenance ?? []) {
            seen.add(envelope.source_family);
        }
        return [...seen];
    }

    function envelopeSummary(envelopes: ReasoningEvidenceEnvelopeDto[]): string {
        return envelopes.map((env) => `${env.source_family}:${env.source_id}`).join(' · ') || '—';
    }

    type RoleGroup = {
        role: string;
        notes: ReasoningNoteDto[];
        accent: string;
        empty: string;
    };

    function roleGroups(decision: InitiationOpeningDecisionExportDto): RoleGroup[] {
        return [
            {
                role: 'Ghi đè',
                notes: decision.override_factors,
                accent: 'text-ky',
                empty: 'Không có yếu tố ghi đè.',
            },
            {
                role: 'Xung đột',
                notes: decision.conflict_notes,
                accent: 'text-tranh',
                empty: 'Không có xung đột.',
            },
            {
                role: 'Kháng cự',
                notes: decision.strongest_resistances,
                accent: 'text-tranh',
                empty: 'Không có kháng cự đáng kể.',
            },
            {
                role: 'Hỗ trợ',
                notes: decision.strongest_supports,
                accent: 'text-nen',
                empty: 'Không có hỗ trợ đáng kể.',
            },
        ];
    }
</script>

<div class="h-full flex flex-col overflow-hidden">
    <div class="p-4 border-b border-ink-border bg-parchment-dark/50 shrink-0">
        <div class="flex flex-wrap items-end justify-between gap-4 mb-4">
            <div>
                <h2 class="text-3xl font-sans font-bold text-ink">Evidence Graph</h2>
                <p class="text-ink-light font-mono text-sm mt-1">
                    {#if report}
                        {report.chart.solar.day}/{report.chart.solar.month}/{report.chart.solar.year} · tier {report.computed_metrics.tier}
                    {:else}
                        awaiting date
                    {/if}
                </p>
            </div>
            <div class="flex flex-wrap items-end gap-3">
                <div>
                    <label for="evidence_birth_year" class="block text-xs font-mono uppercase text-ink-light mb-1">Birth Year</label>
                    <input id="evidence_birth_year" bind:value={birthYear} inputmode="numeric" placeholder="1990" class="bg-parchment border border-ink-border px-3 py-1 font-mono text-sm focus-ring w-24" />
                </div>
                <div>
                    <label for="evidence_birth_month" class="block text-xs font-mono uppercase text-ink-light mb-1">Month</label>
                    <input id="evidence_birth_month" bind:value={birthMonth} inputmode="numeric" placeholder="8" class="bg-parchment border border-ink-border px-3 py-1 font-mono text-sm focus-ring w-20" />
                </div>
                <div>
                    <label for="evidence_birth_day" class="block text-xs font-mono uppercase text-ink-light mb-1">Day</label>
                    <input id="evidence_birth_day" bind:value={birthDay} inputmode="numeric" placeholder="15" class="bg-parchment border border-ink-border px-3 py-1 font-mono text-sm focus-ring w-20" />
                </div>
                <div>
                    <label for="evidence_gender" class="block text-xs font-mono uppercase text-ink-light mb-1">Gender</label>
                    <select id="evidence_gender" bind:value={gender} class="bg-parchment border border-ink-border px-3 py-1.5 font-mono text-sm focus-ring h-[30px]">
                        <option value="">Any</option>
                        <option value="male">Nam</option>
                        <option value="female">Nu</option>
                    </select>
                </div>
            </div>
        </div>

        <div class="flex flex-wrap gap-1">
            {#each lenses as lens}
                <button
                    type="button"
                    class="px-3 py-1.5 font-mono text-sm uppercase tracking-wider transition-colors focus-ring border-b-2"
                    class:border-ink={activeLens === lens.id}
                    class:text-ink={activeLens === lens.id}
                    class:border-transparent={activeLens !== lens.id}
                    class:text-ink-light={activeLens !== lens.id}
                    class:hover:text-ink={activeLens !== lens.id}
                    class:bg-evidence={lens.dev && activeLens === lens.id}
                    onclick={() => (activeLens = lens.id)}
                >
                    {lens.label}{lens.dev ? ' ·' : ''}
                </button>
            {/each}
        </div>
    </div>

    <div class="p-8 flex-grow overflow-y-auto">
        {#if loading}
            <div class="flex h-full items-center justify-center">
                <span class="text-ink-light font-mono animate-pulse">Building evidence graph...</span>
            </div>
        {:else if error}
            <div class="bg-ky/10 text-ky p-4 rounded font-mono border border-ky/20">{error}</div>
        {:else if report}
            {#if activeLens === 'vi_sao'}
                {#if report.decision_export}
                    {@const decision = report.decision_export}
                    <section class="mb-10">
                        <div class="card-dense border-l-4 border-l-hoangdao space-y-4">
                            <div class="flex flex-wrap items-center gap-2">
                                <span class={bucketClass[decision.recommendation_bucket]}>
                                    {bucketLabel[decision.recommendation_bucket]}
                                </span>
                                <span class="badge-evidence">{confidenceLabel[decision.confidence]}</span>
                                <span class="badge-cothe">{semanticLabel[decision.semantic]}</span>
                                <span class="badge-cothe">
                                    {decision.context_is_clear ? 'ngữ cảnh rõ' : 'ngữ cảnh hỗn hợp'}
                                </span>
                            </div>
                            <p class="font-bold text-lg leading-snug">{decision.primary_conclusion}</p>
                        </div>
                    </section>

                    <section class="space-y-6">
                        {#each roleGroups(decision) as group (group.role)}
                            <div>
                                <h3 class="font-mono font-bold uppercase tracking-wider mb-3 {group.accent} flex items-center gap-2">
                                    {group.role}
                                    <span class="text-xs font-normal text-ink-light">({group.notes.length})</span>
                                </h3>
                                {#if group.notes.length}
                                    <ul class="space-y-2">
                                        {#each group.notes as note, i (group.role + '-' + i)}
                                            <li class="card-dense">
                                                <p class="text-sm font-medium">{note.summary_vi}</p>
                                                {#if noteFamilies(note).length}
                                                    <div class="flex flex-wrap gap-1 mt-2">
                                                        {#each noteFamilies(note) as family (family)}
                                                            <span class="badge-evidence">{sourceFamilyLabel[family]}</span>
                                                        {/each}
                                                    </div>
                                                {/if}
                                                {#if note.provenance.length}
                                                    <p class="text-xs text-ink-light font-mono mt-2">
                                                        {envelopeSummary(note.provenance)}
                                                    </p>
                                                {/if}
                                            </li>
                                        {/each}
                                    </ul>
                                {:else}
                                    <p class="text-sm text-ink-light italic">{group.empty}</p>
                                {/if}
                            </div>
                        {/each}
                    </section>
                {:else}
                    <div class="card-dense">
                        <p class="font-bold">{report.summary}</p>
                        <p class="text-sm text-ink-light mt-2">Enter at least a birth date to unlock decision reasoning.</p>
                    </div>
                {/if}
            {:else if activeLens === 'yeu_to'}
                <p class="text-ink-light italic font-mono">Yếu Tố lens — nodes and edges tree arrives in the next commit.</p>
            {:else if activeLens === 'truc'}
                <p class="text-ink-light italic font-mono">Trục lens — axis scores arrive in the next commit.</p>
            {:else if activeLens === 'nguon'}
                <p class="text-ink-light italic font-mono">Nguồn lens — source family legend arrives in the next commit.</p>
            {:else if activeLens === 'dev'}
                <p class="text-ink-light italic font-mono">Dev lens — raw graph dump arrives in the next commit.</p>
            {/if}
        {/if}
    </div>
</div>
