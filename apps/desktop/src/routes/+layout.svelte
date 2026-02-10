<script>
  import { onMount } from 'svelte';
  import '../app.css';
  import { checkForAppUpdates } from '$lib/updater';

  onMount(() => {
    checkForAppUpdates();
  });
</script>

<slot />
