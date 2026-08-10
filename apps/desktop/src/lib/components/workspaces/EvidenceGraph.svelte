<script lang="ts">
    import { selectedDate, userProfile } from '$lib/stores';
    import type { UserProfile } from '$lib/stores';
    import { fetchDebugSemanticGraph, fetchPersonalDayReport } from '$lib/api/invoke';
    import GraphView from './GraphView.svelte';
    import type {
        DecisionConfidenceDto,
        DebugSemanticGraphResponseDto,
        DebugVisualizationNodeDto,
        EdgeEffectDto,
        InitiationOpeningDecisionExportDto,
        InitiationRecommendationBucketDto,
        NodeKindDto,
        PersonalDayReportDto,
        ReasoningConclusionSemanticDto,
        ReasoningEdgeExportDto,
        ReasoningEvidenceEnvelopeDto,
        ReasoningEvidenceSourceFamilyDto,
        ReasoningNodeExportDto,
        ReasoningNodeSeverityDto,
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
    let selectedNodeId: string | null = null;
    let yeuToView: 'list' | 'graph' = 'list';

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

    $: graph = report?.graph ?? null;
    $: nodeGroups = groupNodesByKind(graph?.nodes ?? []);
    $: nodesById = new Map((graph?.nodes ?? []).map((n) => [n.id, n]));
    $: selectedNode = selectedNodeId ? nodesById.get(selectedNodeId) ?? null : null;
    $: incomingEdges = edgesTouching(graph?.edges ?? [], selectedNodeId, 'in');
    $: outgoingEdges = edgesTouching(graph?.edges ?? [], selectedNodeId, 'out');
    $: canonicalRows = report ? canonicalAxisRows(report) : [];
    $: reasoningAxes = report?.decision_export?.axis_scores ?? [];
    $: familyRows = report ? sourceFamilyBreakdown(report) : [];
    $: familyTotal = totalFamilyCount(familyRows);
    $: devNodes = graph?.nodes ?? [];
    $: devEdges = graph?.edges ?? [];
    $: devSeverityCounts = severityCounts(devNodes);
    $: devEffectCounts = effectCounts(devEdges);

    // Clustered debug inspection (amlich-4gef). Richer than report.graph:
    // clusters nodes, adds shape_hint, and includes recommendation evidence.
    // Loaded on demand from the Dev lens; defaults off.
    let inspection: DebugSemanticGraphResponseDto | null = null;
    let inspectionLoading = false;
    let inspectionError: string | null = null;
    let inspectionToken = 0;
    let showClustered = false;
    let inspectionDay = 0;
    let inspectionMonth = 0;
    let inspectionYear = 0;

    $: if (report && activeLens === 'dev' && showClustered) {
        const { day, month, year } = currentDayParts($selectedDate);
        if (day !== inspectionDay || month !== inspectionMonth || year !== inspectionYear) {
            loadInspection(day, month, year);
        }
    }

    async function loadInspection(day: number, month: number, year: number) {
        const token = ++inspectionToken;
        inspectionDay = day;
        inspectionMonth = month;
        inspectionYear = year;
        inspectionLoading = true;
        inspectionError = null;
        try {
            inspection = await fetchDebugSemanticGraph(day, month, year, true);
        } catch (e: unknown) {
            if (token !== inspectionToken) return;
            inspection = null;
            inspectionError = e instanceof Error ? e.message : 'Failed to load debug semantic graph';
        } finally {
            if (token === inspectionToken) inspectionLoading = false;
        }
    }

    function toggleClustered() {
        showClustered = !showClustered;
        if (showClustered && !inspection && !inspectionLoading && $selectedDate) {
            const { day, month, year } = currentDayParts($selectedDate);
            loadInspection(day, month, year);
        }
    }

    $: inspectionClusters = inspection
        ? groupInspectionNodesByCluster(inspection.visualization.nodes)
        : [];
    $: inspectionEdges = inspection?.visualization.edges ?? [];

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
        iching: 'Kinh Dịch',
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

    const kindLabel: Record<NodeKindDto, string> = {
        fact: 'Fact',
        interpreted_signal: 'Signal',
        decision_target: 'Decision',
    };

    const kindOrder: NodeKindDto[] = ['decision_target', 'interpreted_signal', 'fact'];

    const severityLabel: Record<ReasoningNodeSeverityDto, string> = {
        auspicious: 'cat',
        inauspicious: 'hung',
        hard_taboo: 'hard taboo',
        soft_taboo: 'soft taboo',
        hoang_dao: 'hoàng đạo',
        hac_dao: 'hạc đạo',
    };

    const severityClass: Record<ReasoningNodeSeverityDto, string> = {
        auspicious: 'text-nen',
        hoang_dao: 'text-nen',
        inauspicious: 'text-ky',
        hac_dao: 'text-ky',
        hard_taboo: 'text-ky',
        soft_taboo: 'text-tranh',
    };

    const effectLabel: Record<EdgeEffectDto, string> = {
        supports: 'hỗ trợ',
        weakens: 'làm yếu',
        overrides: 'ghi đè',
        conflicts_with: 'xung đột',
        conditions: 'điều kiện',
    };

    const effectClass: Record<EdgeEffectDto, string> = {
        supports: 'text-nen',
        weakens: 'text-tranh',
        overrides: 'text-ky',
        conflicts_with: 'text-ky',
        conditions: 'text-ink-light',
    };

    type NodeGroup = { kind: NodeKindDto; label: string; nodes: ReasoningNodeExportDto[] };

    function groupNodesByKind(nodes: ReasoningNodeExportDto[]): NodeGroup[] {
        const buckets: Record<NodeKindDto, ReasoningNodeExportDto[]> = {
            fact: [],
            interpreted_signal: [],
            decision_target: [],
        };
        for (const node of nodes) buckets[node.kind].push(node);
        return kindOrder
            .filter((kind) => buckets[kind].length > 0)
            .map((kind) => ({ kind, label: kindLabel[kind], nodes: buckets[kind] }));
    }

    function edgesTouching(edges: ReasoningEdgeExportDto[], nodeId: string | null, side: 'in' | 'out'): ReasoningEdgeExportDto[] {
        if (!nodeId) return [];
        return edges.filter((edge) => (side === 'in' ? edge.to_node_id === nodeId : edge.from_node_id === nodeId));
    }

    type InspectionClusterGroup = {
        cluster: string;
        nodes: DebugVisualizationNodeDto[];
        kinds: string[];
    };

    function groupInspectionNodesByCluster(nodes: DebugVisualizationNodeDto[]): InspectionClusterGroup[] {
        const buckets = new Map<string, DebugVisualizationNodeDto[]>();
        for (const node of nodes) {
            const list = buckets.get(node.cluster);
            if (list) list.push(node);
            else buckets.set(node.cluster, [node]);
        }
        return [...buckets.entries()]
            .map(([cluster, list]) => ({
                cluster,
                nodes: list,
                kinds: [...new Set(list.map((n) => n.semantic_kind))],
            }))
            .sort((a, b) => b.nodes.length - a.nodes.length);
    }

    function countEntries(map: Record<string, number> | undefined): { key: string; count: number }[] {
        if (!map) return [];
        return Object.entries(map)
            .map(([key, count]) => ({ key, count }))
            .sort((a, b) => b.count - a.count);
    }

    const canonicalAxisLabel: Record<string, string> = {
        generic_day_quality: 'Chất lượng ngày',
        intent_fit: 'Phù hợp ý định',
        personal_alignment: 'Tương hợp cá nhân',
        annual_pressure: 'Áp lực năm',
        evidence_coverage: 'Độ phủ bằng chứng',
    };

    const canonicalAxisOrder = [
        'generic_day_quality',
        'intent_fit',
        'personal_alignment',
        'annual_pressure',
        'evidence_coverage',
    ];

    const reasoningAxisLabel: Record<string, string> = {
        support: 'Hỗ trợ',
        resistance: 'Kháng cự',
        stability: 'Ổn định',
        personal_alignment: 'Tương hợp cá nhân',
        timing_fit: 'Phù hợp thời điểm',
        context_clarity: 'Rõ ngữ cảnh',
    };

    function scoreBarWidth(score: number | null | undefined): number {
        if (score === null || score === undefined || !Number.isFinite(score)) return 0;
        return Math.max(0, Math.min(100, ((score + 1) / 2) * 100));
    }

    function scoreClass(score: number | null | undefined): string {
        if (score === null || score === undefined) return 'text-ink-light';
        if (score >= 0.35) return 'text-nen';
        if (score <= -0.35) return 'text-ky';
        return 'text-ink';
    }

    type CanonicalAxisRow = {
        key: string;
        axis: string;
        score?: number | null;
        verdict: string;
        unavailable_reason?: string | null;
    };

    function canonicalAxisRows(report: PersonalDayReportDto): CanonicalAxisRow[] {
        const axes = report.canonical_assessment?.axes;
        if (!axes) return [];
        const lookup: Record<string, CanonicalAxisRow> = {
            generic_day_quality: { key: 'generic_day_quality', axis: axes.generic_day_quality.axis, score: axes.generic_day_quality.score ?? null, verdict: axes.generic_day_quality.verdict, unavailable_reason: axes.generic_day_quality.unavailable_reason ?? null },
            intent_fit: { key: 'intent_fit', axis: axes.intent_fit.axis, score: axes.intent_fit.score ?? null, verdict: axes.intent_fit.verdict, unavailable_reason: axes.intent_fit.unavailable_reason ?? null },
            personal_alignment: { key: 'personal_alignment', axis: axes.personal_alignment.axis, score: axes.personal_alignment.score ?? null, verdict: axes.personal_alignment.verdict, unavailable_reason: axes.personal_alignment.unavailable_reason ?? null },
            annual_pressure: { key: 'annual_pressure', axis: axes.annual_pressure.axis, score: axes.annual_pressure.score ?? null, verdict: axes.annual_pressure.verdict, unavailable_reason: axes.annual_pressure.unavailable_reason ?? null },
            evidence_coverage: { key: 'evidence_coverage', axis: axes.evidence_coverage.axis, score: axes.evidence_coverage.score ?? null, verdict: axes.evidence_coverage.verdict, unavailable_reason: axes.evidence_coverage.unavailable_reason ?? null },
        };
        return canonicalAxisOrder.map((key) => lookup[key]).filter((row): row is CanonicalAxisRow => row !== null);
    }

    type FamilyBreakdown = {
        family: ReasoningEvidenceSourceFamilyDto;
        label: string;
        nodes: number;
        edges: number;
        notes: number;
        total: number;
    };

    function sourceFamilyBreakdown(report: PersonalDayReportDto): FamilyBreakdown[] {
        const counts = new Map<ReasoningEvidenceSourceFamilyDto, { nodes: number; edges: number; notes: number }>();
        const ensure = (family: ReasoningEvidenceSourceFamilyDto) => {
            let entry = counts.get(family);
            if (!entry) {
                entry = { nodes: 0, edges: 0, notes: 0 };
                counts.set(family, entry);
            }
            return entry;
        };

        for (const node of report.graph?.nodes ?? []) {
            for (const env of node.evidence ?? []) ensure(env.source_family).nodes += 1;
        }
        for (const edge of report.graph?.edges ?? []) {
            for (const env of edge.evidence ?? []) ensure(env.source_family).edges += 1;
        }
        const decision = report.decision_export;
        if (decision) {
            const notes = [
                ...decision.strongest_supports,
                ...decision.strongest_resistances,
                ...decision.override_factors,
                ...decision.conflict_notes,
            ];
            for (const note of notes) {
                for (const env of note.provenance ?? []) ensure(env.source_family).notes += 1;
            }
        }

        const allFamilies: ReasoningEvidenceSourceFamilyDto[] = [
            'snapshot', 'interaction', 'bazi', 'axis', 'almanac_rule', 'insight', 'derived',
        ];
        return allFamilies
            .map((family) => {
                const entry = counts.get(family) ?? { nodes: 0, edges: 0, notes: 0 };
                const total = entry.nodes + entry.edges + entry.notes;
                return {
                    family,
                    label: sourceFamilyLabel[family],
                    nodes: entry.nodes,
                    edges: entry.edges,
                    notes: entry.notes,
                    total,
                };
            })
            .filter((row) => row.total > 0)
            .sort((a, b) => b.total - a.total);
    }

    function totalFamilyCount(rows: FamilyBreakdown[]): number {
        return rows.reduce((sum, row) => sum + row.total, 0);
    }

    function countBy<T extends string>(items: { value: T }[] | undefined, value: T): number {
        return (items ?? []).filter((item) => item.value === value).length;
    }

    type SeverityCount = { severity: ReasoningNodeSeverityDto; label: string; count: number };

    function severityCounts(nodes: ReasoningNodeExportDto[]): SeverityCount[] {
        const order: ReasoningNodeSeverityDto[] = [
            'hard_taboo', 'soft_taboo', 'hac_dao', 'inauspicious', 'hoang_dao', 'auspicious',
        ];
        return order
            .map((severity) => ({
                severity,
                label: severityLabel[severity],
                count: countBy(
                    nodes.map((node) => ({ value: node.severity ?? ('' as ReasoningNodeSeverityDto) })),
                    severity,
                ),
            }))
            .filter((row) => row.count > 0);
    }

    type EffectCount = { effect: EdgeEffectDto; label: string; count: number };

    function effectCounts(edges: ReasoningEdgeExportDto[]): EffectCount[] {
        const order: EdgeEffectDto[] = ['overrides', 'conflicts_with', 'weakens', 'conditions', 'supports'];
        return order
            .map((effect) => ({
                effect,
                label: effectLabel[effect],
                count: edges.filter((edge) => edge.effect === effect).length,
            }))
            .filter((row) => row.count > 0);
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
                {#if graph}
                    <div class="flex items-center justify-end gap-1 mb-4">
                        <span class="text-xs font-mono uppercase text-ink-light mr-2">Chế độ xem</span>
                        <button
                            type="button"
                            class="px-3 py-1.5 text-xs font-mono uppercase tracking-wider border-b-2 focus-ring"
                            class:border-ink={yeuToView === 'list'}
                            class:text-ink={yeuToView === 'list'}
                            class:border-transparent={yeuToView !== 'list'}
                            class:text-ink-light={yeuToView !== 'list'}
                            onclick={() => (yeuToView = 'list')}
                            aria-pressed={yeuToView === 'list'}
                        >
                            Danh sách
                        </button>
                        <button
                            type="button"
                            class="px-3 py-1.5 text-xs font-mono uppercase tracking-wider border-b-2 focus-ring"
                            class:border-ink={yeuToView === 'graph'}
                            class:text-ink={yeuToView === 'graph'}
                            class:border-transparent={yeuToView !== 'graph'}
                            class:text-ink-light={yeuToView !== 'graph'}
                            onclick={() => (yeuToView = 'graph')}
                            aria-pressed={yeuToView === 'graph'}
                        >
                            Đồ thị
                        </button>
                    </div>
                    {#if yeuToView === 'graph'}
                        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                            <div class="lg:col-span-2 card-dense p-4">
                                <GraphView
                                    nodes={graph.nodes}
                                    edges={graph.edges}
                                    {selectedNodeId}
                                    onSelect={(id) => (selectedNodeId = id)}
                                />
                            </div>
                            <div class="lg:sticky lg:top-0 self-start">
                                {#if selectedNode}
                                    {@const node = selectedNode}
                                    <div class="card-dense border-l-4 border-l-hoangdao space-y-4">
                                        <div>
                                            <div class="text-xs font-mono uppercase text-ink-light">{node.id}</div>
                                            <p class="font-bold text-lg mt-1">{node.summary_vi}</p>
                                            <div class="flex flex-wrap gap-1 mt-2">
                                                <span class="badge-cothe">{kindLabel[node.kind]}</span>
                                                {#if node.axis}
                                                    <span class="badge-cothe">{node.axis.replaceAll('_', ' ')}</span>
                                                {/if}
                                                {#if node.severity}
                                                    <span class="badge-evidence">{severityLabel[node.severity]}</span>
                                                {/if}
                                                {#each node.tags as tag (tag)}
                                                    <span class="text-xs font-mono text-ink-light">#{tag}</span>
                                                {/each}
                                            </div>
                                        </div>

                                        {#if node.evidence.length}
                                            <div>
                                                <h4 class="text-xs font-mono uppercase text-ink-light mb-2">Evidence</h4>
                                                <ul class="space-y-1">
                                                    {#each node.evidence as env, i (node.id + '-ev-' + i)}
                                                        <li class="text-xs font-mono text-ink-light">
                                                            <span class="badge-evidence">{sourceFamilyLabel[env.source_family]}</span>
                                                            <span class="ml-1">{env.method} · {env.source_id}</span>
                                                        </li>
                                                    {/each}
                                                </ul>
                                            </div>
                                        {/if}

                                        {#if incomingEdges.length}
                                            <div>
                                                <h4 class="text-xs font-mono uppercase text-ink-light mb-2">Incoming ({incomingEdges.length})</h4>
                                                <ul class="space-y-2">
                                                    {#each incomingEdges as edge, i ('in-' + i)}
                                                        {@const neighbor = nodesById.get(edge.from_node_id)}
                                                        <li class="border-l-2 border-ink-border pl-2">
                                                            <div class="flex items-center gap-2 text-xs font-mono">
                                                                <span class="{effectClass[edge.effect]}">{effectLabel[edge.effect]}</span>
                                                                <span class="text-ink-light">· w{edge.weight}</span>
                                                            </div>
                                                            <p class="text-sm mt-0.5">{neighbor?.summary_vi ?? edge.from_node_id}</p>
                                                        </li>
                                                    {/each}
                                                </ul>
                                            </div>
                                        {/if}

                                        {#if outgoingEdges.length}
                                            <div>
                                                <h4 class="text-xs font-mono uppercase text-ink-light mb-2">Outgoing ({outgoingEdges.length})</h4>
                                                <ul class="space-y-2">
                                                    {#each outgoingEdges as edge, i ('out-' + i)}
                                                        {@const neighbor = nodesById.get(edge.to_node_id)}
                                                        <li class="border-l-2 border-ink-border pl-2">
                                                            <div class="flex items-center gap-2 text-xs font-mono">
                                                                <span class="{effectClass[edge.effect]}">{effectLabel[edge.effect]}</span>
                                                                <span class="text-ink-light">· w{edge.weight}</span>
                                                            </div>
                                                            <p class="text-sm mt-0.5">{neighbor?.summary_vi ?? edge.to_node_id}</p>
                                                        </li>
                                                    {/each}
                                                </ul>
                                            </div>
                                        {/if}
                                    </div>
                                {:else}
                                    <div class="card-dense text-sm text-ink-light italic">
                                        Chọn một nút trên đồ thị để xem chi tiết.
                                    </div>
                                {/if}
                            </div>
                        </div>
                    {:else}
                        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                            <div class="space-y-6">
                                {#each nodeGroups as group (group.kind)}
                                    <section>
                                        <h3 class="text-lg font-mono font-bold mb-3 uppercase tracking-wider text-ink-light flex items-center gap-2">
                                            {group.label}
                                            <span class="text-xs font-normal">({group.nodes.length})</span>
                                        </h3>
                                        <ul class="space-y-2">
                                            {#each group.nodes as node (node.id)}
                                                <li>
                                                    <button
                                                        type="button"
                                                        class="w-full text-left card-dense focus-ring"
                                                        class:border-l-hoangdao={selectedNodeId === node.id}
                                                        class:bg-parchment-dark={selectedNodeId === node.id}
                                                        onclick={() => (selectedNodeId = selectedNodeId === node.id ? null : node.id)}
                                                    >
                                                        <div class="flex items-start justify-between gap-2">
                                                            <span class="text-sm font-medium">{node.summary_vi}</span>
                                                            {#if node.severity}
                                                                <span class="text-xs font-mono uppercase {severityClass[node.severity]}">
                                                                    {severityLabel[node.severity]}
                                                                </span>
                                                            {/if}
                                                        </div>
                                                        {#if node.axis}
                                                            <span class="text-xs text-ink-light font-mono mt-1">{node.axis.replaceAll('_', ' ')}</span>
                                                        {/if}
                                                    </button>
                                                </li>
                                            {/each}
                                        </ul>
                                    </section>
                                {/each}
                            </div>

                            <div class="lg:sticky lg:top-0 self-start">
                                {#if selectedNode}
                                    {@const node = selectedNode}
                                    <div class="card-dense border-l-4 border-l-hoangdao space-y-4">
                                        <div>
                                            <div class="text-xs font-mono uppercase text-ink-light">{node.id}</div>
                                            <p class="font-bold text-lg mt-1">{node.summary_vi}</p>
                                            <div class="flex flex-wrap gap-1 mt-2">
                                                <span class="badge-cothe">{kindLabel[node.kind]}</span>
                                                {#if node.axis}
                                                    <span class="badge-cothe">{node.axis.replaceAll('_', ' ')}</span>
                                                {/if}
                                                {#if node.severity}
                                                    <span class="badge-evidence">{severityLabel[node.severity]}</span>
                                                {/if}
                                                {#each node.tags as tag (tag)}
                                                    <span class="text-xs font-mono text-ink-light">#{tag}</span>
                                                {/each}
                                            </div>
                                        </div>

                                        {#if node.evidence.length}
                                            <div>
                                                <h4 class="text-xs font-mono uppercase text-ink-light mb-2">Evidence</h4>
                                                <ul class="space-y-1">
                                                    {#each node.evidence as env, i (node.id + '-ev-' + i)}
                                                        <li class="text-xs font-mono text-ink-light">
                                                            <span class="badge-evidence">{sourceFamilyLabel[env.source_family]}</span>
                                                            <span class="ml-1">{env.method} · {env.source_id}</span>
                                                        </li>
                                                    {/each}
                                                </ul>
                                            </div>
                                        {/if}

                                        {#if incomingEdges.length}
                                            <div>
                                                <h4 class="text-xs font-mono uppercase text-ink-light mb-2">Incoming ({incomingEdges.length})</h4>
                                                <ul class="space-y-2">
                                                    {#each incomingEdges as edge, i ('in-' + i)}
                                                        {@const neighbor = nodesById.get(edge.from_node_id)}
                                                        <li class="border-l-2 border-ink-border pl-2">
                                                            <div class="flex items-center gap-2 text-xs font-mono">
                                                                <span class="{effectClass[edge.effect]}">{effectLabel[edge.effect]}</span>
                                                                <span class="text-ink-light">· w{edge.weight}</span>
                                                            </div>
                                                            <p class="text-sm mt-0.5">{neighbor?.summary_vi ?? edge.from_node_id}</p>
                                                        </li>
                                                    {/each}
                                                </ul>
                                            </div>
                                        {/if}

                                        {#if outgoingEdges.length}
                                            <div>
                                                <h4 class="text-xs font-mono uppercase text-ink-light mb-2">Outgoing ({outgoingEdges.length})</h4>
                                                <ul class="space-y-2">
                                                    {#each outgoingEdges as edge, i ('out-' + i)}
                                                        {@const neighbor = nodesById.get(edge.to_node_id)}
                                                        <li class="border-l-2 border-ink-border pl-2">
                                                            <div class="flex items-center gap-2 text-xs font-mono">
                                                                <span class="{effectClass[edge.effect]}">{effectLabel[edge.effect]}</span>
                                                                <span class="text-ink-light">· w{edge.weight}</span>
                                                            </div>
                                                            <p class="text-sm mt-0.5">{neighbor?.summary_vi ?? edge.to_node_id}</p>
                                                        </li>
                                                    {/each}
                                                </ul>
                                            </div>
                                        {/if}
                                    </div>
                                {:else}
                                    <div class="card-dense text-sm text-ink-light italic">
                                        Select a node on the left to inspect its evidence and connections.
                                    </div>
                                {/if}
                            </div>
                        </div>
                    {/if}
                {:else}
                    <div class="card-dense text-sm text-ink-light italic">
                        No reasoning graph for this day. Enter a birth date to compute the personal reasoning bundle.
                    </div>
                {/if}
            {:else if activeLens === 'truc'}
                {#if canonicalRows.length}
                    <section class="mb-10">
                        <h3 class="text-2xl font-mono font-bold mb-4">Trục đánh giá</h3>
                        <div class="card-dense space-y-4">
                            {#each canonicalRows as axis (axis.key)}
                                <div>
                                    <div class="flex justify-between items-baseline gap-2 mb-1">
                                        <span class="font-mono text-sm uppercase tracking-wider">
                                            {canonicalAxisLabel[axis.key] ?? axis.axis}
                                        </span>
                                        <span class="font-mono text-sm {scoreClass(axis.score)}">
                                            {#if axis.score === null || axis.score === undefined}
                                                n/a
                                            {:else}
                                                {axis.score.toFixed(2)}
                                            {/if}
                                        </span>
                                    </div>
                                    <div class="w-full bg-parchment-dark rounded-full h-1.5 overflow-hidden">
                                        <div class="bg-hoangdao h-1.5 rounded-full" style="width: {scoreBarWidth(axis.score)}%"></div>
                                    </div>
                                    {#if axis.unavailable_reason}
                                        <p class="text-xs text-tranh font-mono mt-1">⚠ {axis.unavailable_reason}</p>
                                    {:else if axis.verdict}
                                        <p class="text-xs text-ink-light mt-1">{axis.verdict}</p>
                                    {/if}
                                </div>
                            {/each}
                        </div>
                    </section>
                {/if}

                {#if reasoningAxes.length}
                    <section>
                        <h3 class="text-xl font-mono font-bold mb-3 uppercase tracking-wider text-ink-light">
                            {canonicalRows.length ? 'Lens lý luận (6 trục)' : 'Trục lý luận'}
                        </h3>
                        <div class="card-dense space-y-3">
                            {#each reasoningAxes as axis (axis.axis)}
                                <div>
                                    <div class="flex justify-between text-xs font-mono mb-1 uppercase">
                                        <span>{reasoningAxisLabel[axis.axis] ?? axis.axis.replaceAll('_', ' ')}</span>
                                        <span class={scoreClass(axis.score)}>{axis.score.toFixed(2)}</span>
                                    </div>
                                    <div class="w-full bg-parchment-dark rounded-full h-1.5 overflow-hidden">
                                        <div class="bg-hoangdao h-1.5 rounded-full" style="width: {scoreBarWidth(axis.score)}%"></div>
                                    </div>
                                    {#if axis.strongest_summary_vi}
                                        <p class="text-xs text-ink-light mt-1">{axis.strongest_summary_vi}</p>
                                    {/if}
                                </div>
                            {/each}
                        </div>
                    </section>
                {:else if !canonicalRows.length}
                    <div class="card-dense text-sm text-ink-light italic">
                        No axis data available. Enter a birth date to compute personal assessment.
                    </div>
                {/if}
            {:else if activeLens === 'nguon'}
                {#if familyRows.length}
                    <section>
                        <h3 class="text-2xl font-mono font-bold mb-1">Nguồn bằng chứng</h3>
                        <p class="text-sm text-ink-light font-mono mb-4">{familyTotal} envelope{familyTotal === 1 ? '' : 's'} across {familyRows.length} source {familyRows.length === 1 ? 'family' : 'families'}</p>
                        <div class="space-y-2">
                            {#each familyRows as row (row.family)}
                                <div class="card-dense">
                                    <div class="flex items-center justify-between mb-2">
                                        <div class="flex items-center gap-2">
                                            <span class="badge-evidence">{row.label}</span>
                                            <span class="text-xs font-mono text-ink-light uppercase">{row.family}</span>
                                        </div>
                                        <div class="flex items-center gap-2">
                                            <span class="font-mono font-bold">{row.total}</span>
                                            <span class="text-xs font-mono text-ink-light">
                                                ({((row.total / familyTotal) * 100).toFixed(0)}%)
                                            </span>
                                        </div>
                                    </div>
                                    <div class="w-full bg-parchment-dark rounded-full h-1.5 overflow-hidden">
                                        <div class="bg-evidence h-1.5 rounded-full" style="width: {((row.total / familyTotal) * 100).toFixed(2)}%"></div>
                                    </div>
                                    <div class="grid grid-cols-3 gap-2 mt-2 text-xs font-mono text-ink-light">
                                        <div>Nodes: <span class="text-ink">{row.nodes}</span></div>
                                        <div>Edges: <span class="text-ink">{row.edges}</span></div>
                                        <div>Notes: <span class="text-ink">{row.notes}</span></div>
                                    </div>
                                </div>
                            {/each}
                        </div>
                    </section>
                {:else}
                    <div class="card-dense text-sm text-ink-light italic">
                        No evidence envelopes for this day.
                    </div>
                {/if}
            {:else if activeLens === 'dev'}
                <div class="bg-evidence/5 border border-evidence/20 p-3 mb-6 font-mono text-xs">
                    <span class="badge-evidence">DEV LENS</span>
                    <span class="ml-2 text-ink-light">Raw graph dump — internal IDs, justifications, and evidence envelopes as the engine sees them.</span>
                </div>

                <div class="card-dense mb-6 flex flex-wrap items-center justify-between gap-3">
                    <div>
                        <div class="text-sm font-bold">Clustered inspection</div>
                        <p class="text-xs text-ink-light font-mono mt-0.5">
                            DebugSemanticGraphInspection — clusters, shape_hint, recommendation evidence.
                        </p>
                    </div>
                    <button
                        type="button"
                        class="px-3 py-1.5 font-mono text-xs uppercase tracking-wider border border-ink-border focus-ring transition-colors"
                        class:bg-evidence={showClustered}
                        class:text-ink={showClustered}
                        class:text-ink-light={!showClustered}
                        onclick={toggleClustered}
                    >
                        {showClustered ? 'Hide clustered' : 'Show clustered'}
                    </button>
                </div>

                {#if showClustered}
                    {#if inspectionLoading}
                        <div class="card-dense mb-6">
                            <span class="text-ink-light font-mono animate-pulse text-sm">Loading clustered inspection...</span>
                        </div>
                    {:else if inspectionError}
                        <div class="bg-ky/10 text-ky p-3 rounded font-mono border border-ky/20 mb-6 text-sm">
                            {inspectionError}
                        </div>
                    {:else if inspection}
                        {@const clusterCounts = countEntries(inspection.cluster_counts)}
                        {@const kindCounts = countEntries(inspection.semantic_kind_counts)}
                        {@const sevCounts = countEntries(inspection.severity_counts)}

                        <section class="mb-8">
                            <h3 class="text-xl font-mono font-bold mb-3">Clustered summary</h3>
                            <div class="grid grid-cols-2 md:grid-cols-4 gap-3">
                                <div class="card-dense">
                                    <div class="text-xs font-mono uppercase text-ink-light">Nodes</div>
                                    <div class="text-2xl font-bold">{inspection.summary.total_nodes}</div>
                                </div>
                                <div class="card-dense">
                                    <div class="text-xs font-mono uppercase text-ink-light">Edges</div>
                                    <div class="text-2xl font-bold">{inspection.summary.total_edges}</div>
                                </div>
                                <div class="card-dense">
                                    <div class="text-xs font-mono uppercase text-ink-light">Clusters</div>
                                    <div class="text-2xl font-bold">{inspection.summary.clusters.length}</div>
                                </div>
                                <div class="card-dense">
                                    <div class="text-xs font-mono uppercase text-ink-light">Rec. evidence</div>
                                    <div class="text-sm font-bold mt-1">
                                        {inspection.summary.has_recommendation_evidence ? 'yes' : 'no'}
                                    </div>
                                </div>
                            </div>

                            {#if clusterCounts.length}
                                <div class="mt-3 flex flex-wrap gap-2">
                                    {#each clusterCounts as row (row.key)}
                                        <span class="badge-evidence">{row.key}: {row.count}</span>
                                    {/each}
                                </div>
                            {/if}
                            {#if kindCounts.length}
                                <div class="mt-2 flex flex-wrap gap-2">
                                    {#each kindCounts as row (row.key)}
                                        <span class="badge-cothe">{row.key}: {row.count}</span>
                                    {/each}
                                </div>
                            {/if}
                            {#if sevCounts.length}
                                <div class="mt-2 flex flex-wrap gap-2">
                                    {#each sevCounts as row (row.key)}
                                        <span class="badge-cothe">sev:{row.key}: {row.count}</span>
                                    {/each}
                                </div>
                            {/if}
                        </section>

                        <section class="mb-8">
                            <h3 class="text-xl font-mono font-bold mb-3">
                                Nodes by cluster ({inspection.visualization.nodes.length})
                            </h3>
                            <div class="space-y-4">
                                {#each inspectionClusters as group (group.cluster)}
                                    <div>
                                        <h4 class="text-sm font-mono uppercase tracking-wider text-ink-light mb-2">
                                            {group.cluster}
                                            <span class="text-xs font-normal">({group.nodes.length} · kinds: {group.kinds.join(', ')})</span>
                                        </h4>
                                        <div class="space-y-2">
                                            {#each group.nodes as node (node.node_id)}
                                                <div class="card-dense font-mono text-xs">
                                                    <div class="flex flex-wrap items-baseline gap-2">
                                                        <span class="font-bold text-ink">{node.node_id}</span>
                                                        <span class="text-ink-light">{node.semantic_kind}</span>
                                                        {#if node.severity}
                                                            <span class="text-tranh">sev={node.severity}</span>
                                                        {/if}
                                                        {#if node.shape_hint}
                                                            <span class="badge-evidence">shape: {node.shape_hint}</span>
                                                        {/if}
                                                    </div>
                                                    <p class="text-sm font-sans mt-1">{node.label}</p>
                                                </div>
                                            {/each}
                                        </div>
                                    </div>
                                {/each}
                            </div>
                        </section>

                        <section class="mb-8">
                            <h3 class="text-xl font-mono font-bold mb-3">
                                Clustered edges ({inspectionEdges.length})
                            </h3>
                            <div class="space-y-2">
                                {#each inspectionEdges as edge, i ('insp-edge-' + i)}
                                    <div class="card-dense font-mono text-xs">
                                        <div class="flex flex-wrap items-baseline gap-2">
                                            <span class="font-bold text-ink">{edge.from_id}</span>
                                            <span class="text-ink-light">—{edge.semantic_kind}→</span>
                                            <span class="font-bold text-ink">{edge.to_id}</span>
                                            <span class="text-ink-light">w={edge.weight}</span>
                                        </div>
                                        <p class="text-sm font-sans mt-1">{edge.label}</p>
                                    </div>
                                {/each}
                            </div>
                        </section>
                    {/if}
                {/if}

                {#if graph}
                    <section class="mb-8">
                        <h3 class="text-xl font-mono font-bold mb-3">Summary</h3>
                        <div class="grid grid-cols-2 md:grid-cols-4 gap-3">
                            <div class="card-dense">
                                <div class="text-xs font-mono uppercase text-ink-light">Action</div>
                                <div class="text-sm font-bold">{graph.action_id}</div>
                            </div>
                            <div class="card-dense">
                                <div class="text-xs font-mono uppercase text-ink-light">Nodes</div>
                                <div class="text-2xl font-bold">{devNodes.length}</div>
                            </div>
                            <div class="card-dense">
                                <div class="text-xs font-mono uppercase text-ink-light">Edges</div>
                                <div class="text-2xl font-bold">{devEdges.length}</div>
                            </div>
                            <div class="card-dense">
                                <div class="text-xs font-mono uppercase text-ink-light">Action ID</div>
                                <div class="text-sm font-mono">{graph.action_id}</div>
                            </div>
                        </div>

                        {#if devSeverityCounts.length}
                            <div class="mt-3 flex flex-wrap gap-2">
                                {#each devSeverityCounts as row (row.severity)}
                                    <span class="badge-evidence">{row.label}: {row.count}</span>
                                {/each}
                            </div>
                        {/if}
                        {#if devEffectCounts.length}
                            <div class="mt-2 flex flex-wrap gap-2">
                                {#each devEffectCounts as row (row.effect)}
                                    <span class="badge-cothe">{row.label}: {row.count}</span>
                                {/each}
                            </div>
                        {/if}
                    </section>

                    <section class="mb-8">
                        <h3 class="text-xl font-mono font-bold mb-3">Nodes ({devNodes.length})</h3>
                        <div class="space-y-2">
                            {#each devNodes as node (node.id)}
                                <div class="card-dense font-mono text-xs">
                                    <div class="flex flex-wrap items-baseline gap-2">
                                        <span class="font-bold text-ink">{node.id}</span>
                                        <span class="text-ink-light">{node.kind}</span>
                                        {#if node.axis}
                                            <span class="text-ink-light">axis={node.axis}</span>
                                        {/if}
                                        {#if node.severity}
                                            <span class="{severityClass[node.severity]}">sev={node.severity}</span>
                                        {/if}
                                    </div>
                                    <p class="text-sm font-sans mt-1">{node.summary_vi}</p>
                                    {#if node.tags.length}
                                        <div class="mt-1 text-ink-light">tags: [{node.tags.join(', ')}]</div>
                                    {/if}
                                    {#if node.evidence.length}
                                        <ul class="mt-2 space-y-0.5 text-ink-light">
                                            {#each node.evidence as env, i (node.id + '-ev-' + i)}
                                                <li>· {env.source_family}:{env.source_id} ({env.method}){env.note ? ' — ' + env.note : ''}</li>
                                            {/each}
                                        </ul>
                                    {/if}
                                </div>
                            {/each}
                        </div>
                    </section>

                    <section>
                        <h3 class="text-xl font-mono font-bold mb-3">Edges ({devEdges.length})</h3>
                        <div class="space-y-2">
                            {#each devEdges as edge, i ('edge-' + i)}
                                <div class="card-dense font-mono text-xs">
                                    <div class="flex flex-wrap items-baseline gap-2">
                                        <span class="font-bold text-ink">{edge.from_node_id}</span>
                                        <span class="{effectClass[edge.effect]}">—{edge.effect}→</span>
                                        <span class="font-bold text-ink">{edge.to_node_id}</span>
                                        <span class="text-ink-light">w={edge.weight}</span>
                                        <span class="text-ink-light">just={edge.justification}</span>
                                    </div>
                                    {#if edge.tags.length}
                                        <div class="mt-1 text-ink-light">tags: [{edge.tags.join(', ')}]</div>
                                    {/if}
                                    {#if edge.evidence.length}
                                        <ul class="mt-2 space-y-0.5 text-ink-light">
                                            {#each edge.evidence as env, j ('edge-' + i + '-ev-' + j)}
                                                <li>· {env.source_family}:{env.source_id} ({env.method})</li>
                                            {/each}
                                        </ul>
                                    {/if}
                                </div>
                            {/each}
                        </div>
                    </section>
                {:else}
                    <div class="card-dense text-sm text-ink-light italic">
                        No reasoning graph for this day.
                    </div>
                {/if}
            {/if}
        {/if}
    </div>
</div>
