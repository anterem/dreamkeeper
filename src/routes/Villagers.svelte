<script lang="ts">
  import { villagers } from '$lib/live.svelte';
  import { roleLabel } from '$lib/utils';
  import Section from '$lib/components/Section.svelte';

  let toLevel = $derived(villagers.data.filter((v) => v.needsGifting && !v.isMaxed));
</script>

<Section
  title="Villagers"
  href="/villagers"
  summary={toLevel.length > 0 ? `${toLevel.length} to level` : undefined}
>
  {#if villagers.error}
    <p class="muted">Couldn't read villagers: <span class="error">{villagers.error}</span></p>
  {:else if toLevel.length > 0}
    <ul class="columns">
      {#each toLevel as villager}
        <li>
          <div class="entry">
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
  .gifts {
    color: var(--color-text-subtle);
  }
</style>
