<script lang="ts">
  import { onMount } from 'svelte';
  import '../app.css';
  import { checkForAppUpdates } from '$lib/updater';

  onMount(() => {
    checkForAppUpdates();
  });
</script>

<div class="h-screen w-screen overflow-hidden flex flex-col bg-parchment text-ink">
  <slot />
</div>
