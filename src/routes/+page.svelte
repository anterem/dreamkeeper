<script lang="ts">
  import { snapshot } from '$lib/snapshot.svelte';
  import { clock } from '$lib/clock.svelte';
  import { liveCritter, liveVillager, localWeekday } from '$lib/time';
  import { checklistLabel, formatSchedule, roleLabel } from '$lib/utils';
  import Section from '$lib/components/Section.svelte';

  let snap = $derived(snapshot.current);
  let tz = $derived(snap?.tzOffset ?? 0);
  let loading = $derived(snap === null);
  let weekday = $derived(localWeekday(clock.nowSecs, tz));

  let checklist = $derived(snap?.checklist.status === 'ok' ? snap.checklist.data : null);

  let critters = $derived(
    snap?.critters.status === 'ok'
      ? snap.critters.data.map((c) => liveCritter(c, clock.nowSecs, tz))
      : null
  );
  let toFeed = $derived(critters ? critters.filter((c) => c.needsFeeding) : []);

  let villagers = $derived(
    snap?.villagers.status === 'ok'
      ? snap.villagers.data.map((v) => liveVillager(v, clock.nowSecs, tz))
      : null
  );
  let toGift = $derived(
    villagers
      ? villagers.filter(
          (v) => v.status === 'inVillage' && !v.isMaxed && v.gifts.some((g) => !g.giftedToday)
        )
      : []
  );
</script>

<main>
  {#if loading}
    <p class="status"><em>Reading your save…</em></p>
  {:else if snap}
    <Section title="Checklist">
      {#if snap.checklist.status === 'error'}
        <p class="muted">
          Couldn't read checklist: <span class="error">{snap.checklist.error}</span>
        </p>
      {:else if checklist && checklist.length === 0}
        <p class="muted"><em>All clear.</em></p>
      {:else if checklist}
        <ul class="checklist">
          {#each checklist as item}
            <li class="entry"><span class="strong">{checklistLabel(item)}</span></li>
          {/each}
        </ul>
      {/if}
    </Section>

    <Section
      title="Critters"
      href="/critters"
      summary={toFeed.length > 0 ? `${toFeed.length} to feed` : undefined}
    >
      {#if snap.critters.status === 'error'}
        <p class="muted">
          Couldn't read critters: <span class="error">{snap.critters.error}</span>
        </p>
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

    <Section
      title="Villagers"
      href="/villagers"
      summary={toGift.length > 0 ? `${toGift.length} to befriend` : undefined}
    >
      {#if snap.villagers.status === 'error'}
        <p class="muted">
          Couldn't read villagers: <span class="error">{snap.villagers.error}</span>
        </p>
      {:else if toGift.length === 0}
        <p class="muted"><em>All gifted for today.</em></p>
      {:else}
        <ul class="columns">
          {#each toGift as villager}
            <li class="card">
              <div class="heading">
                <span class="strong">{villager.name}</span>
                <span class="meta"
                  >Lv {villager.friendshipLevel}{#if villager.role}{' '}{roleLabel(
                      villager.role
                    )}{/if}</span
                >
              </div>
              <p class="gifts">
                {villager.gifts
                  .filter((g) => !g.giftedToday)
                  .map((g) => g.name)
                  .join(' · ')}
              </p>
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

  .columns,
  .checklist {
    --column-width: 16rem;
    width: calc(2 * var(--column-width) + var(--space-6));
  }

  .columns {
    display: block;
    columns: 2;
    column-gap: var(--space-6);
  }

  @container (max-width: 34rem) {
    .columns,
    .checklist {
      columns: 1;
      width: var(--column-width);
    }
  }

  .columns > li,
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

  .entry .strong,
  .heading .strong {
    font-size: var(--font-size-md);
    line-height: 1.2;
  }

  .time {
    flex: none;
    white-space: nowrap;
    color: var(--color-text-subtle);
  }

  .card {
    display: flex;
    flex-direction: column;
  }

  .heading {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    gap: var(--space-3);
  }

  .meta {
    flex: none;
    white-space: nowrap;
    color: var(--color-text-subtle);
  }

  .gifts {
    color: var(--color-text-subtle);
  }
</style>
