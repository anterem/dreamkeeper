<script lang="ts">
  import { onMount } from 'svelte';
  import { commands, type Today } from '$lib/bindings';
  import { formatSchedule } from '$lib/utils';
  import Section from '$lib/components/Section.svelte';

  let today = $state<Today | null>(null);
  let error = $state('');
  let loading = $state(true);

  const weekday = new Date().getDay();
  let critters = $derived(today?.critters.status === 'ok' ? today.critters.data : null);
  let toFeed = $derived(critters ? critters.filter((c) => c.needsFeeding) : []);

  onMount(async () => {
    const now = Math.floor(Date.now() / 1000);
    const result = await commands.getToday(now);
    loading = false;
    if (result.status === 'ok') {
      today = result.data;
    } else {
      error = JSON.stringify(result.error);
    }
  });
</script>

<main>
  {#if loading}
    <p class="status"><em>Reading your save…</em></p>
  {:else if error}
    <p class="status">Something went amiss: <span class="error">{error}</span></p>
  {:else if today}
    <Section
      title="Critters"
      href="/critters"
      summary={toFeed.length > 0 ? `${toFeed.length} to feed` : undefined}
    >
      {#if today.critters.status === 'error'}
        <p class="muted">Couldn't read critters: <span class="error">{today.critters.error}</span></p>
      {:else if toFeed.length === 0}
        <p class="muted"><em>All fed for today.</em></p>
      {:else}
        <ul class="columns">
          {#each toFeed as critter}
            <li>
              <span class="strong">{critter.name}</span>
              <span class="muted time">{formatSchedule(critter.schedule[weekday])}</span>
            </li>
          {/each}
        </ul>
      {/if}
    </Section>
  {/if}
</main>

<style>
  main {
    max-width: 44rem;
    margin-inline: auto;
    container-type: inline-size;
  }

  .columns {
    --column-width: 16rem;
    display: block;
    columns: 2;
    column-gap: var(--space-6);
    width: calc(2 * var(--column-width) + var(--space-6));
  }

  @container (max-width: 34rem) {
    .columns {
      columns: 1;
      width: var(--column-width);
    }
  }

  .columns > li {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    gap: var(--space-3);
    padding: var(--space-1) 0;
    break-inside: avoid;
    font-size: var(--font-size-sm);
  }

  .time {
    white-space: nowrap;
  }
</style>
