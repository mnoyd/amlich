<script lang="ts">
    import { activeWorkspace, selectedDate } from '$lib/stores';

    const workspaces = [
        { id: 'day_console', label: 'Day' },
        { id: 'activity_board', label: 'Activity' },
        { id: 'hour_studio', label: 'Hours' },
        { id: 'almanac_inspector', label: 'Almanac' },
        { id: 'season_timeline', label: 'Season' },
        { id: 'personal_lab', label: 'Personal' },
        { id: 'bazi_lab', label: 'Bazi' },
        { id: 'evidence_graph', label: 'Evidence' },
    ];

    function selectWorkspace(id: string) {
        $activeWorkspace = id;
    }
</script>

<aside class="w-64 border-r border-ink-border flex flex-col h-full bg-parchment-dark/30">
    <div class="p-4 border-b border-ink-border">
        <h1 class="text-lg font-mono font-bold tracking-wider">AMLICH</h1>
        <div class="text-xs text-ink-light uppercase">Observatory</div>
    </div>
    
    <div class="p-4 flex-grow overflow-y-auto">
        <ul class="space-y-2">
            {#each workspaces as ws}
                <li>
                    <button 
                        class="w-full text-left px-3 py-2 rounded font-mono text-sm uppercase transition-colors"
                        class:bg-ink={ws.id === $activeWorkspace}
                        class:text-parchment={ws.id === $activeWorkspace}
                        class:hover:bg-ink-border={ws.id !== $activeWorkspace}
                        onclick={() => selectWorkspace(ws.id)}
                    >
                        {ws.label}
                    </button>
                </li>
            {/each}
        </ul>
    </div>
</aside>
