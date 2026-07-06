<script lang="ts">
  import { critters } from '$lib/live.svelte';
  import Section from '$lib/components/Section.svelte';

  let toFeed = $derived(critters.data.filter((c) => c.unlocked && c.needsFeeding));
</script>

<Section
  title="Critters"
  href="/critters"
  summary={toFeed.length > 0 ? `${toFeed.length} to feed` : undefined}
>
  {#if critters.error}
    <p class="muted">Couldn't read critters: <span class="error">{critters.error}</span></p>
  {:else if toFeed.length === 0}
    <p class="muted"><em>All fed for today.</em></p>
  {:else}
    <ul class="columns">
      {#each toFeed as critter}
        <li class="entry">
          <span class="strong">{critter.name}</span>
          <span class="meta">{critter.biome}</span>
        </li>
      {/each}
    </ul>
  {/if}
</Section>
