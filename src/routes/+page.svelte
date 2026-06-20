<script lang="ts">
  import { onMount } from 'svelte';
  import { commands, type Today } from '$lib/bindings';
  import { formatSchedule, roleLabel } from '$lib/utils';
  import Section from '$lib/components/Section.svelte';

  let today = $state<Today | null>(null);
  let error = $state('');
  let loading = $state(true);

  const weekday = new Date().getDay();
  let critters = $derived(today?.critters.status === 'ok' ? today.critters.data : null);
  let toFeed = $derived(critters ? critters.filter((c) => c.needsFeeding) : []);

  let villagers = $derived(today?.villagers.status === 'ok' ? today.villagers.data : null);
  let toGift = $derived(
    villagers
      ? villagers.filter(
          (v) => v.status === 'inVillage' && !v.isMaxed && v.gifts.some((g) => !g.giftedToday)
        )
      : []
  );

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
      {#if today.villagers.status === 'error'}
        <p class="muted">
          Couldn't read villagers: <span class="error">{today.villagers.error}</span>
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
