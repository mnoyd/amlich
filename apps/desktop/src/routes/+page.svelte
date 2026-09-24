<script lang="ts">
  import { dev } from '$app/environment';
  import { page } from '$app/stores';
  import LeftRail from '$lib/components/layout/LeftRail.svelte';
  import RightRail from '$lib/components/layout/RightRail.svelte';
  import BottomStrip from '$lib/components/layout/BottomStrip.svelte';
  import DayConsole from '$lib/components/workspaces/DayConsole.svelte';
  import AlmanacInspector from '$lib/components/workspaces/AlmanacInspector.svelte';
  import BaziLab from '$lib/components/workspaces/BaziLab.svelte';
  import HourStudio from '$lib/components/workspaces/HourStudio.svelte';
  import PersonalLab from '$lib/components/workspaces/PersonalLab.svelte';
  import EvidenceGraph from '$lib/components/workspaces/EvidenceGraph.svelte';
  import SeasonTimeline from '$lib/components/workspaces/SeasonTimeline.svelte';
  import DailyWorkspacePrototype from '$lib/components/prototype/DailyWorkspacePrototype.svelte';
  import InfluenceExplorerPrototype from '$lib/components/prototype/InfluenceExplorerPrototype.svelte';
  import NextDayView from '$lib/components/next/NextDayView.svelte';

  import { activeWorkspace } from '$lib/stores';

  // REPLACEMENT S1 (amlich-b14l.7): live Day View Model behind ?surface=next, dev only.
  $: nextSurface = dev && $page.url.searchParams.get('surface') === 'next';
  // PROTOTYPE: Three daily-workspace IA variants on the existing route, switchable via ?variant=.
  $: requestedVariant = $page.url.searchParams.get('variant')?.toUpperCase() ?? null;
  $: prototypeVariant = dev && requestedVariant && ['A', 'B', 'C'].includes(requestedVariant)
    ? requestedVariant
    : null;
  $: prototypeMode = dev && $page.url.searchParams.get('prototype') === 'influence';
  $: influenceView = $page.url.searchParams.get('view')?.toLowerCase() ?? 'graph';
</script>

{#if nextSurface}
  <NextDayView />
{:else if prototypeMode}
  <InfluenceExplorerPrototype view={influenceView} />
{:else if prototypeVariant}
  <DailyWorkspacePrototype variant={prototypeVariant} />
{:else}
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
      {:else if $activeWorkspace === 'season_timeline'}
        <SeasonTimeline />
      {:else if $activeWorkspace === 'evidence_graph'}
        <EvidenceGraph />
      {:else}
        <div class="p-8 flex items-center justify-center h-full text-ink-light font-mono italic">
          Workspace "{$activeWorkspace}" is under construction.
        </div>
      {/if}
    </main>

    <RightRail />
  </div>

  <BottomStrip />
{/if}
