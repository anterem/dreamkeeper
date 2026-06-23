<script lang="ts">
  import { snapshot } from '$lib/snapshot.svelte';
  import Checklist from './Checklist.svelte';
  import Critters from './Critters.svelte';
  import Villagers from './Villagers.svelte';

  let loading = $derived(snapshot.current === null);
</script>

<main>
  {#if loading}
    <p class="status"><em>Reading your save…</em></p>
  {:else}
    <Checklist />
    <Critters />
    <Villagers />
  {/if}
</main>

<style>
  main {
    max-width: 44rem;
    margin-inline: auto;
    container-type: inline-size;
  }
</style>
