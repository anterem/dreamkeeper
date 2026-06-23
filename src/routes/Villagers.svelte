<script lang="ts">
  import { snapshot } from '$lib/snapshot.svelte';
  import { clock } from '$lib/clock.svelte';
  import { liveVillager } from '$lib/time';
  import { roleLabel } from '$lib/utils';
  import Section from '$lib/components/Section.svelte';

  let section = $derived(snapshot.current?.villagers ?? null);
  let tz = $derived(snapshot.current?.tzOffset ?? 0);

  let villagers = $derived(
    section?.status === 'ok' ? section.data.map((v) => liveVillager(v, clock.nowSecs, tz)) : null
  );
  let toLevel = $derived(
    villagers
      ? villagers.filter(
          (v) => v.status === 'inVillage' && !v.isMaxed && v.gifts.some((g) => !g.giftedToday)
        )
      : []
  );
</script>

<Section
  title="Villagers"
  href="/villagers"
  summary={toLevel.length > 0 ? `${toLevel.length} to level` : undefined}
>
  {#if section?.status === 'error'}
    <p class="muted">Couldn't read villagers: <span class="error">{section.error}</span></p>
  {:else if toLevel.length === 0}
    <p class="muted"><em>All gifted for today.</em></p>
  {:else}
    <ul class="columns">
      {#each toLevel as villager}
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

  .heading .strong {
    font-size: var(--font-size-md);
    line-height: 1.2;
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
