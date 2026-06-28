<script lang="ts">
  import { snapshot } from '$lib/snapshot.svelte';
  import { clock } from '$lib/clock.svelte';
  import { liveCritter, localWeekday } from '$lib/time';
  import { formatSchedule } from '$lib/utils';
  import Section from '$lib/components/Section.svelte';

  let section = $derived(snapshot.current?.critters ?? null);
  let tz = $derived(snapshot.current?.tzOffset ?? 0);
  let weekday = $derived(localWeekday(clock.nowSecs, tz));

  let critters = $derived(
    section?.status === 'ok' ? section.data.map((c) => liveCritter(c, clock.nowSecs, tz)) : null
  );
  let toFeed = $derived(critters ? critters.filter((c) => c.unlocked && c.needsFeeding) : []);
</script>

<Section
  title="Critters"
  href="/critters"
  summary={toFeed.length > 0 ? `${toFeed.length} to feed` : undefined}
>
  {#if section?.status === 'error'}
    <p class="muted">Couldn't read critters: <span class="error">{section.error}</span></p>
  {:else if toFeed.length === 0}
    <p class="muted"><em>All fed for today.</em></p>
  {:else}
    <ul class="columns">
      {#each toFeed as critter}
        <li class="entry">
          <span class="strong">{critter.name}</span>
          <span class="time">{formatSchedule(critter.schedule[weekday])}</span>
        </li>
      {/each}
    </ul>
  {/if}
</Section>

<style>
  .columns {
    --column-width: 16rem;
    width: calc(2 * var(--column-width) + var(--space-6));
    display: block;
    columns: 2;
    column-gap: var(--space-6);
  }

  @container (max-width: 34rem) {
    .columns {
      columns: 1;
      width: var(--column-width);
    }
  }

  .columns > li {
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

  .time {
    flex: none;
    white-space: nowrap;
    color: var(--color-text-subtle);
  }
</style>
