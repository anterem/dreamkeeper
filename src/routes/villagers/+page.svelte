<script lang="ts">
  import { onMount } from 'svelte';
  import { commands, type Villager } from '$lib/bindings';
  import { roleLabel } from '$lib/utils';

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
        <li class="card" class:faded={!inVillage}>
          <div class="entry">
            <span class="strong">{villager.name}</span>
            {#if inVillage}
              <span class="muted level">Lv {villager.friendshipLevel}</span>
            {/if}
            <span class="leader" aria-hidden="true"></span>
            {#if inVillage}
              {#if villager.role}
                <span class="muted role">{roleLabel(villager.role)}</span>
              {/if}
            {:else}
              <span class="muted status-label">Locked</span>
            {/if}
          </div>
          {#if villager.gifts.length > 0}
            <ul class="gifts">
              {#each villager.gifts as gift}
                <li class:given={gift.giftedToday} title={gift.giftedToday ? 'gifted today' : undefined}>
                  {gift.name}
                </li>
              {/each}
            </ul>
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

  .leader {
    flex: 1;
    min-width: var(--space-4);
    border-bottom: 1px dotted var(--color-rule);
  }

  .level,
  .status-label {
    flex: none;
    font-size: var(--font-size-sm);
  }

  .role {
    flex: none;
    font-size: var(--font-size-sm);
  }

  .gifts {
    margin: 0;
    padding-left: var(--space-5);
    list-style: disc;
    color: var(--color-text-muted);
  }

  .gifts li::marker {
    content: '–\00a0\00a0';
    color: var(--color-text-muted);
  }

  .gifts li.given::marker {
    content: '✓ ';
    color: var(--color-accent);
  }
</style>
