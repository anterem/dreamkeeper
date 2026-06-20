<script lang="ts">
  import { onMount } from 'svelte';
  import { commands, type Villager } from '$lib/bindings';
  import { formatFriendship } from '$lib/utils';

  let villagers = $state<Villager[]>([]);
  let error = $state('');
  let loading = $state(true);

  onMount(async () => {
    const now = Math.floor(Date.now() / 1000);
    const result = await commands.getVillagers(now);
    loading = false;
    if (result.status === 'ok') {
      villagers = result.data;
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
  {:else if villagers.length === 0}
    <p class="status">
      <em>No villager data found.</em><br />
      Make sure the game files are installed and a save is loaded.
    </p>
  {:else}
    <ol class="ledger">
      {#each villagers as villager (villager.id)}
        {@const inVillage = villager.status === 'inVillage'}
        <li class="entry" class:faded={!inVillage}>
          <span class="strong">{villager.name}</span>
          <span class="leader" aria-hidden="true"></span>
          {#if inVillage}
            <span class="muted friendship">
              {formatFriendship(villager.friendshipLevel, villager.friendshipXp)}
            </span>
          {:else}
            <span class="muted status-label">
              {villager.status === 'inRealm' ? 'In realm' : 'Locked'}
            </span>
          {/if}
        </li>
      {/each}
    </ol>
  {/if}
</main>

<style>
  main {
    max-width: 34rem;
    margin-inline: auto;
  }

  .ledger {
    list-style: none;
    padding: 0;
    display: flex;
    flex-direction: column;
  }

  .entry {
    display: flex;
    align-items: baseline;
    gap: var(--space-2);
    padding: 0.1rem 0;
  }

  .entry.faded {
    opacity: 0.5;
  }

  .leader {
    flex: 1;
    min-width: var(--space-4);
    border-bottom: 1px dotted var(--color-rule);
  }

  .friendship,
  .status-label {
    flex: none;
    font-size: var(--font-size-sm);
  }
</style>
