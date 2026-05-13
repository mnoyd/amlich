<script lang="ts">
  import LeftRail from '$lib/components/layout/LeftRail.svelte';
  import RightRail from '$lib/components/layout/RightRail.svelte';
  import BottomStrip from '$lib/components/layout/BottomStrip.svelte';
  import DayConsole from '$lib/components/workspaces/DayConsole.svelte';
  import AlmanacInspector from '$lib/components/workspaces/AlmanacInspector.svelte';
  import BaziLab from '$lib/components/workspaces/BaziLab.svelte';
  import HourStudio from '$lib/components/workspaces/HourStudio.svelte';
  import PersonalLab from '$lib/components/workspaces/PersonalLab.svelte';
  
  import { activeWorkspace } from '$lib/stores';
</script>

<div class="flex-grow flex overflow-hidden">
  <LeftRail />
  
  <main class="flex-grow overflow-y-auto">
    {#if $activeWorkspace === 'day_console'}
      <DayConsole />
    {:else if $activeWorkspace === 'hour_studio'}
      <HourStudio />
    {:else if $activeWorkspace === 'almanac_inspector'}
      <AlmanacInspector />
    {:else if $activeWorkspace === 'bazi_lab'}
      <BaziLab />
    {:else if $activeWorkspace === 'personal_lab'}
      <PersonalLab />
    {:else}
      <div class="p-8 flex items-center justify-center h-full text-ink-light font-mono italic">
        Workspace "{$activeWorkspace}" is under construction.
      </div>
    {/if}
  </main>
  
  <RightRail />
</div>

<BottomStrip />
