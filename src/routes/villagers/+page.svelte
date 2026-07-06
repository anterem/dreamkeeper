<script lang="ts">
  import { type Role } from '$lib/bindings';
  import { villagers } from '$lib/live.svelte';
  import { ROLES, roleLabel } from '$lib/utils';
  import { PersistedState } from '$lib/persisted.svelte';
  import FilterToggle from '$lib/components/FilterToggle.svelte';

  const onlyUnlocked = new PersistedState('villagers.unlocked', true);
  const onlyFriendshipNeeded = new PersistedState('villagers.toLevel', false);
  const onlyGiftable = new PersistedState('villagers.toGift', false);
  const selectedRole = new PersistedState<Role | null>('villagers.role', null);

  let shownVillagers = $derived.by(() => {
    let list = villagers.data;
    if (onlyUnlocked.current) list = list.filter((v) => v.status === 'inVillage');
    if (onlyFriendshipNeeded.current)
      list = list.filter((v) => v.status === 'inVillage' && !v.isMaxed);
    if (onlyGiftable.current) list = list.filter((v) => v.needsGifting);
    if (selectedRole.current !== null)
      list = list.filter((v) => v.role === selectedRole.current);
    return list;
  });

  let roleCounts = $derived.by(() => {
    const counts = new Map<Role, number>();
    for (const v of villagers.data) {
      if (v.status === 'inVillage' && v.role !== null) {
        counts.set(v.role, (counts.get(v.role) ?? 0) + 1);
      }
    }
    return counts;
  });

  function selectRole(role: Role) {
    selectedRole.current = selectedRole.current === role ? null : role;
  }
</script>

<main>
  {#if villagers.loading}
    <p class="status"><em>Reading your save…</em></p>
  {:else if villagers.error}
    <p class="status">Something went amiss: <span class="error">{villagers.error}</span></p>
  {:else if villagers.data.length === 0}
    <p class="status">
      <em>No villager data found.</em><br />
      Make sure the game files are installed and a save is loaded.
    </p>
  {:else}
    <div class="filters">
      <span class="filters-label">filter by:</span>
      <FilterToggle
        label="unlocked"
        active={onlyUnlocked.current}
        onclick={() => (onlyUnlocked.current = !onlyUnlocked.current)}
      />
      <FilterToggle
        label="to level"
        active={onlyFriendshipNeeded.current}
        onclick={() => (onlyFriendshipNeeded.current = !onlyFriendshipNeeded.current)}
      />
      <FilterToggle
        label="to gift"
        active={onlyGiftable.current}
        onclick={() => (onlyGiftable.current = !onlyGiftable.current)}
      />
      {#each ROLES as role}
        <FilterToggle
          label={`${role} (${roleCounts.get(role) ?? 0})`}
          active={selectedRole.current === role}
          onclick={() => selectRole(role)}
        />
      {/each}
    </div>

    {#if shownVillagers.length === 0}
      <p class="status"><em>None match.</em></p>
    {:else}
      <ol class="ledger">
        {#each shownVillagers as villager (villager.id)}
          {@const inVillage = villager.status === 'inVillage'}
          <li class="card" class:faded={!inVillage}>
            <div class="entry">
              <span class="strong">{villager.name}</span>
              {#if inVillage}
                <span class="meta">Lv {villager.friendshipLevel}</span>
              {/if}
              <span class="leader" aria-hidden="true"></span>
              {#if inVillage}
                {#if villager.role}
                  <span class="meta">{roleLabel(villager.role)}</span>
                {/if}
              {:else}
                <span class="meta">Locked</span>
              {/if}
            </div>
            {#if villager.gifts.length > 0}
              <ul class="gifts">
                {#each villager.gifts as gift}
                  <li
                    class:given={gift.giftedToday}
                    title={gift.giftedToday ? 'gifted today' : undefined}
                  >
                    {gift.name}
                  </li>
                {/each}
              </ul>
            {/if}
          </li>
        {/each}
      </ol>
    {/if}
  {/if}
</main>

<style>
  main {
    max-width: 34rem;
    margin-inline: auto;
  }

  .card {
    padding: var(--space-2) 0;
  }

  .card.faded {
    opacity: 0.5;
  }

  .entry {
    display: flex;
    align-items: baseline;
    gap: var(--space-2);
  }

  .meta {
    flex: none;
    font-size: var(--font-size-sm);
    color: var(--color-text-subtle);
  }

  .gifts {
    margin: 0;
    padding-left: var(--space-5);
    list-style: disc;
    color: var(--color-text-subtle);
  }

  .gifts li::marker {
    content: '–\00a0\00a0';
    color: var(--color-text-subtle);
  }

  .gifts li.given::marker {
    content: '✓ ';
    color: var(--color-accent);
  }
</style>
