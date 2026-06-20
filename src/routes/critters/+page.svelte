<script lang="ts">
  import { onMount } from 'svelte';
  import { commands, type Critter } from '$lib/bindings';
  import { WEEKDAY_NAMES, formatSchedule } from '$lib/utils';

  let critters = $state<Critter[]>([]);
  let error = $state('');
  let loading = $state(true);

  // 0–6, starts sunday
  const todayIndex = new Date().getDay();
  let selectedDay = $state(todayIndex);
  let isToday = $derived(selectedDay === todayIndex);
  let prevDayName = $derived(WEEKDAY_NAMES[(selectedDay + 6) % 7]);
  let nextDayName = $derived(WEEKDAY_NAMES[(selectedDay + 1) % 7]);
  let dayCritters = $derived.by(() =>
    critters.filter((c) => c.schedule[selectedDay].length > 0).toSorted(compareOnDay(selectedDay))
  );

  function prevDay() {
    selectedDay = (selectedDay + 6) % 7;
  }

  function nextDay() {
    selectedDay = (selectedDay + 1) % 7;
  }

  function goToToday() {
    selectedDay = todayIndex;
  }

  function compareOnDay(day: number) {
    return (a: Critter, b: Critter) => {
      const sa = a.schedule[day][0];
      const sb = b.schedule[day][0];
      return sa.start - sb.start || sb.end - sa.end || a.speciesRank - b.speciesRank;
    };
  }

  onMount(async () => {
    const now = Math.floor(Date.now() / 1000);
    const result = await commands.getCritters(now);
    loading = false;
    if (result.status === 'ok') {
      critters = result.data;
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
  {:else if critters.length === 0}
    <p class="status">
      <em>No critter data found.</em><br />
      Make sure the game files are installed and a save is loaded.
    </p>
  {:else}
    <nav class="day-nav">
      <button class="arrow prev" onclick={prevDay} aria-label={`Previous day, ${prevDayName}`}
        >☜<span class="day-name">{prevDayName}</span></button
      >
      <div class="day-title">
        {#if isToday}
          <h2>Today</h2>
          <p class="day-note">{WEEKDAY_NAMES[selectedDay]}</p>
        {:else}
          <h2>{WEEKDAY_NAMES[selectedDay]}</h2>
          <button class="today-link" onclick={goToToday}>return to today</button>
        {/if}
      </div>
      <button class="arrow next" onclick={nextDay} aria-label={`Next day, ${nextDayName}`}
        ><span class="day-name">{nextDayName}</span>☞</button
      >
    </nav>

    <p class="legend">
      {#if isToday}
        <span class="now">●</span> available now
        <span>✓</span> fed today
      {/if}
      <span class="tamed">♥</span> tamed
    </p>

    {#if dayCritters.length === 0}
      <p class="status"><em>No creatures stir this day.</em></p>
    {:else}
      <ol class="ledger">
        {#each dayCritters as critter (critter.itemId)}
          {@const available = isToday && critter.availableNow}
          {@const fed = isToday && critter.fedToday}
          <li class="entry" class:faded={isToday && !available}>
            <span
              class="pip"
              class:now={available && !fed}
              title={fed ? 'fed today' : available ? 'available now' : undefined}
              >{fed ? '✓' : available ? '●' : ''}</span
            >
            <span class="strong">
              {critter.name}{#if critter.note}<abbr class="dagger" title={critter.note}>†</abbr
                >{/if}
            </span>
            {#if critter.tamed}<span class="badge" title="tamed">♥</span>{/if}
            <span class="leader" aria-hidden="true"></span>
            <span class="time">{formatSchedule(critter.schedule[selectedDay])}</span>
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

  .day-nav {
    display: grid;
    grid-template-columns: auto 1fr auto;
    align-items: center;
    gap: var(--space-3);
    margin-bottom: var(--space-4);
  }

  .arrow {
    display: inline-flex;
    align-items: center;
    gap: var(--space-2);
    background: none;
    border: 0;
    cursor: pointer;
    padding: var(--space-1) var(--space-3);
    font-size: var(--font-size-xl);
    line-height: 1;
    color: var(--color-text-muted);
    transition:
      color var(--duration-fast) var(--ease-out),
      transform var(--duration-fast) var(--ease-out);
  }

  .day-name {
    font-family: var(--font-display);
    font-size: var(--font-size-md);
  }
  .arrow:hover {
    color: var(--color-primary);
  }
  .arrow.prev:hover {
    transform: translateX(-3px);
  }
  .arrow.next:hover {
    transform: translateX(3px);
  }
  .day-title {
    text-align: center;
  }
  .day-title h2 {
    font-size: var(--font-size-xl);
  }

  .day-note {
    font-style: italic;
    font-size: var(--font-size-sm);
    color: var(--color-primary);
  }

  .today-link {
    background: none;
    border: 0;
    padding: 0;
    cursor: pointer;
    font-family: var(--font-body);
    font-style: italic;
    font-size: var(--font-size-sm);
    color: var(--color-link);
    text-decoration: underline;
    text-decoration-thickness: 1px;
    text-underline-offset: 2px;
  }
  .today-link:hover {
    color: var(--color-primary-hover);
  }

  .legend {
    display: flex;
    justify-content: center;
    gap: var(--space-2);
    align-items: baseline;
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    margin-bottom: var(--space-5);
  }
  .legend .now {
    color: var(--color-primary);
  }
  .legend .tamed {
    color: var(--color-accent);
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

  .pip {
    flex: none;
    width: 1rem;
    text-align: center;
    font-size: var(--font-size-sm);
  }
  .pip.now {
    color: var(--color-primary);
  }

  .dagger {
    color: var(--color-accent);
    text-decoration: none;
    cursor: help;
    margin-left: 0.15em;
  }

  .leader {
    flex: 1;
    min-width: var(--space-4);
    border-bottom: 1px dotted var(--color-rule);
  }

  .time {
    flex: none;
    font-size: var(--font-size-sm);
    color: var(--color-text-subtle);
  }

  .badge {
    flex: none;
    font-size: var(--font-size-sm);
    color: var(--color-accent);
  }
</style>
