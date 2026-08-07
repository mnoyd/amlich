<script lang="ts">
    import type {
        EdgeEffectDto,
        NodeKindDto,
        ReasoningEdgeExportDto,
        ReasoningEvidenceEnvelopeDto,
        ReasoningNodeExportDto,
        ReasoningNodeSeverityDto,
    } from '$lib/api/types';

    let {
        nodes,
        edges,
        selectedNodeId = null,
        onSelect,
    }: {
        nodes: ReasoningNodeExportDto[];
        edges: ReasoningEdgeExportDto[];
        selectedNodeId?: string | null;
        onSelect?: (nodeId: string | null) => void;
    } = $props();

    const NODE_WIDTH = 168;
    const NODE_HEIGHT = 56;
    const ROW_GAP = 24;
    const COLUMN_GAP = 240;
    const PADDING = 24;
    const HEADER_HEIGHT = 28;

    const kindLabel: Record<NodeKindDto, string> = {
        fact: 'Yếu tố',
        interpreted_signal: 'Tín hiệu',
        decision_target: 'Quyết định',
    };

    const kindFill: Record<NodeKindDto, string> = {
        fact: '#fcfbf9',
        interpreted_signal: '#fef3c7',
        decision_target: '#ecfccb',
    };

    const kindStroke: Record<NodeKindDto, string> = {
        fact: '#475569',
        interpreted_signal: '#d4af37',
        decision_target: '#2d8a56',
    };

    const severityStroke: Partial<Record<ReasoningNodeSeverityDto, string>> = {
        auspicious: '#2d8a56',
        hoang_dao: '#2d8a56',
        inauspicious: '#d93838',
        hac_dao: '#d93838',
        hard_taboo: '#d93838',
        soft_taboo: '#d97706',
    };

    const effectStroke: Record<EdgeEffectDto, string> = {
        supports: '#2d8a56',
        weakens: '#d97706',
        overrides: '#d93838',
        conflicts_with: '#d93838',
        conditions: '#475569',
    };

    const effectLabel: Record<EdgeEffectDto, string> = {
        supports: 'hỗ trợ',
        weakens: 'làm yếu',
        overrides: 'ghi đè',
        conflicts_with: 'xung đột',
        conditions: 'điều kiện',
    };

    type PositionedNode = {
        node: ReasoningNodeExportDto;
        x: number;
        y: number;
        column: 'left' | 'right';
    };

    const layout = $derived.by(() => {
        const left: ReasoningNodeExportDto[] = [];
        const right: ReasoningNodeExportDto[] = [];
        for (const node of nodes) {
            if (node.kind === 'fact') left.push(node);
            else right.push(node);
        }

        const positioned: PositionedNode[] = [];
        const leftX = PADDING + NODE_WIDTH / 2;
        const rightX = PADDING + COLUMN_GAP + NODE_WIDTH / 2;
        const leftStart = PADDING + HEADER_HEIGHT;
        const rightStart = PADDING + HEADER_HEIGHT;

        left.forEach((node, index) => {
            positioned.push({
                node,
                x: leftX,
                y: leftStart + index * (NODE_HEIGHT + ROW_GAP),
                column: 'left',
            });
        });

        right.forEach((node, index) => {
            positioned.push({
                node,
                x: rightX,
                y: rightStart + index * (NODE_HEIGHT + ROW_GAP),
                column: 'right',
            });
        });

        const positionsById = new Map(positioned.map((p) => [p.node.id, p]));

        const leftRows = Math.max(left.length, 1);
        const rightRows = Math.max(right.length, 1);
        const usedRows = Math.max(leftRows, rightRows);
        const totalHeight = PADDING * 2 + HEADER_HEIGHT + usedRows * (NODE_HEIGHT + ROW_GAP);
        const totalWidth = PADDING * 2 + NODE_WIDTH * 2 + COLUMN_GAP;

        const paths = edges
            .map((edge) => {
                const from = positionsById.get(edge.from_node_id);
                const to = positionsById.get(edge.to_node_id);
                if (!from || !to) return null;
                const sx = from.x + NODE_WIDTH / 2;
                const sy = from.y + NODE_HEIGHT / 2;
                const tx = to.x - NODE_WIDTH / 2;
                const ty = to.y + NODE_HEIGHT / 2;
                const dx = Math.max(40, Math.abs(tx - sx) * 0.5);
                return {
                    edge,
                    d: `M ${sx} ${sy} C ${sx + dx} ${sy}, ${tx - dx} ${ty}, ${tx} ${ty}`,
                };
            })
            .filter((entry): entry is { edge: ReasoningEdgeExportDto; d: string } => entry !== null);

        return { positioned, paths, totalWidth, totalHeight };
    });

    function strokeForNode(node: ReasoningNodeExportDto): string {
        if (node.severity && severityStroke[node.severity]) {
            return severityStroke[node.severity]!;
        }
        return kindStroke[node.kind];
    }

    function strokeForEdge(edge: ReasoningEdgeExportDto): string {
        return effectStroke[edge.effect];
    }

    function strokeWidthForEdge(edge: ReasoningEdgeExportDto): number {
        const absWeight = Math.abs(edge.weight);
        if (absWeight >= 1) return 2.5;
        if (absWeight >= 0.5) return 2;
        return 1.25;
    }

    function opacityForEdge(edge: ReasoningEdgeExportDto): number {
        return edge.from_node_id === selectedNodeId || edge.to_node_id === selectedNodeId ? 1 : 0.55;
    }

    function shortLabel(summary: string, max: number): string {
        if (summary.length <= max) return summary;
        return summary.slice(0, max - 1).trimEnd() + '…';
    }

    function evidenceTag(envelope: ReasoningEvidenceEnvelopeDto): string {
        return envelope.source_family;
    }

    function toggleSelect(nodeId: string) {
        if (!onSelect) return;
        onSelect(selectedNodeId === nodeId ? null : nodeId);
    }

    function onNodeKey(event: KeyboardEvent, nodeId: string) {
        if (event.key === 'Enter' || event.key === ' ') {
            event.preventDefault();
            toggleSelect(nodeId);
        }
    }
</script>

<div class="relative" role="region" aria-label="Sơ đồ minh chứng">
    <div class="overflow-x-auto">
        <svg
            viewBox={`0 0 ${layout.totalWidth} ${layout.totalHeight}`}
            class="w-full min-w-[640px] block"
            preserveAspectRatio="xMidYMin meet"
            role="img"
            aria-label="Sơ đồ nút-liên kết của reasoning graph"
        >
            <text
                x={PADDING + NODE_WIDTH / 2}
                y={PADDING + 16}
                text-anchor="middle"
                class="font-mono uppercase tracking-wider"
                font-size="11"
                fill="#475569"
            >
                {kindLabel.fact}
            </text>
            <text
                x={PADDING + COLUMN_GAP + NODE_WIDTH / 2}
                y={PADDING + 16}
                text-anchor="middle"
                class="font-mono uppercase tracking-wider"
                font-size="11"
                fill="#475569"
            >
                {kindLabel.interpreted_signal}
            </text>

            <g class="edges" aria-hidden="true">
                {#each layout.paths as { edge, d } (edge.from_node_id + '->' + edge.to_node_id)}
                    {@const stroke = strokeForEdge(edge)}
                    {@const isSelected = edge.from_node_id === selectedNodeId || edge.to_node_id === selectedNodeId}
                    <g>
                        <path
                            {d}
                            fill="none"
                            stroke={stroke}
                            stroke-width={strokeWidthForEdge(edge)}
                            stroke-opacity={opacityForEdge(edge)}
                            stroke-linecap="round"
                            marker-end={`url(#arrow-${edge.effect})`}
                        />
                        {#if isSelected}
                            <title>{effectLabel[edge.effect]} · w{edge.weight}</title>
                        {/if}
                    </g>
                {/each}
            </g>

            <defs>
                {#each Object.entries(effectStroke) as [effect, color] (effect)}
                    <marker
                        id={`arrow-${effect}`}
                        viewBox="0 0 10 10"
                        refX="9"
                        refY="5"
                        markerWidth="6"
                        markerHeight="6"
                        orient="auto-start-reverse"
                    >
                        <path d="M 0 0 L 10 5 L 0 10 z" fill={color} />
                    </marker>
                {/each}
            </defs>

            <g class="nodes">
                {#each layout.positioned as entry (entry.node.id)}
                    {@const node = entry.node}
                    {@const isSelected = selectedNodeId === node.id}
                    {@const stroke = strokeForNode(node)}
                    <g
                        role="button"
                        tabindex="0"
                        aria-pressed={isSelected}
                        aria-label={`${kindLabel[node.kind]}: ${node.summary_vi}${node.severity ? `, ${node.severity}` : ''}`}
                        transform={`translate(${entry.x - NODE_WIDTH / 2}, ${entry.y})`}
                        class="cursor-pointer focus:outline-none"
                        onclick={() => toggleSelect(node.id)}
                        onkeydown={(event) => onNodeKey(event, node.id)}
                    >
                        <rect
                            width={NODE_WIDTH}
                            height={NODE_HEIGHT}
                            rx="6"
                            ry="6"
                            fill={kindFill[node.kind]}
                            stroke={stroke}
                            stroke-width={isSelected ? 2.5 : 1.25}
                        />
                        {#if isSelected}
                            <rect
                                x="-2"
                                y="-2"
                                width={NODE_WIDTH + 4}
                                height={NODE_HEIGHT + 4}
                                rx="8"
                                ry="8"
                                fill="none"
                                stroke="#d4af37"
                                stroke-width="1.5"
                                stroke-dasharray="4 3"
                                opacity="0.7"
                            />
                        {/if}
                        <text
                            x="10"
                            y="18"
                            class="font-mono uppercase tracking-wider"
                            font-size="9"
                            fill="#475569"
                        >
                            {kindLabel[node.kind]}
                        </text>
                        <text x="10" y="36" font-size="12" fill="#1a1a1a" font-weight="600">
                            {shortLabel(node.summary_vi, 24)}
                        </text>
                        {#if node.axis}
                            <text x="10" y="49" class="font-mono" font-size="9" fill="#475569">
                                {node.axis.replaceAll('_', ' ')}
                            </text>
                        {/if}
                        {#if node.tags.length}
                            {@const tagText = node.tags.slice(0, 2).map((t) => `#${t}`).join(' ')}
                            <text
                                x={NODE_WIDTH - 10}
                                y="49"
                                text-anchor="end"
                                class="font-mono"
                                font-size="9"
                                fill="#333333"
                            >
                                {shortLabel(tagText, 22)}
                            </text>
                        {/if}
                        {#if node.evidence.length}
                            <text
                                x={NODE_WIDTH - 10}
                                y="18"
                                text-anchor="end"
                                class="font-mono"
                                font-size="9"
                                fill="#3b82f6"
                            >
                                {node.evidence.length} ev
                            </text>
                        {/if}
                    </g>
                {/each}
            </g>
        </svg>
    </div>

    {#if layout.paths.length}
        <ul
            class="flex flex-wrap gap-3 mt-3 text-xs font-mono text-ink-light"
            aria-label="Edge legend"
        >
            {#each Object.entries(effectLabel) as [effect, label] (effect)}
                {@const color = effectStroke[effect as EdgeEffectDto]}
                <li class="inline-flex items-center gap-1.5">
                    <span
                        class="inline-block w-3 h-0.5 rounded-full"
                        style:background-color={color}
                        aria-hidden="true"
                    ></span>
                    <span>{label}</span>
                </li>
            {/each}
        </ul>
    {/if}
</div>

<style>
    svg text {
        font-family: 'Inter', system-ui, sans-serif;
    }
    svg g[role='button']:focus-visible rect {
        stroke-width: 3;
    }
</style>