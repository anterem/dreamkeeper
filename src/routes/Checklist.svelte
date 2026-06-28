<script lang="ts">
  import { snapshot } from '$lib/snapshot.svelte';
  import { checklistLabel, liveChecklist } from '$lib/checklist';
  import Section from '$lib/components/Section.svelte';

  let section = $derived(snapshot.current?.checklist ?? null);
  let items = $derived(section?.status === 'ok' ? liveChecklist(section.data) : null);
</script>

<Section title="Checklist">
  <div class="content">
    {#if section?.status === 'error'}
      <p class="muted">Couldn't read checklist: <span class="error">{section.error}</span></p>
    {:else if items && items.length === 0}
      <p class="muted"><em>All clear.</em></p>
    {:else if items}
      <ul class="checklist">
        {#each items as item}
          <li class="entry"><span class="strong">{checklistLabel(item)}</span></li>
        {/each}
      </ul>
    {/if}
  </div>
</Section>

<style>
  .content {
    --column-width: 16rem;
    width: calc(2 * var(--column-width) + var(--space-6));
  }

  @container (max-width: 34rem) {
    .content {
      width: var(--column-width);
    }
  }

  .checklist > li {
    padding: var(--space-2) 0;
    break-inside: avoid;
    font-size: var(--font-size-sm);
  }

  .entry {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    gap: var(--space-3);
  }

  .entry .strong {
    font-size: var(--font-size-md);
    line-height: 1.2;
  }
</style>
