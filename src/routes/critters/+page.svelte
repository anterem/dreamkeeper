<script lang="ts">
  import { snapshot } from '$lib/snapshot.svelte';
  import { clock } from '$lib/clock.svelte';
  import { liveCritter, localWeekday, type LiveCritter } from '$lib/time';
  import { WEEKDAY_NAMES, formatSchedule } from '$lib/utils';
  import { PersistedState } from '$lib/persisted.svelte';
  import FilterToggle from '$lib/components/FilterToggle.svelte';

  let section = $derived(snapshot.current?.critters ?? null);
  let tz = $derived(snapshot.current?.tzOffset ?? 0);
  let loading = $derived(snapshot.current === null);
  let error = $derived(section?.status === 'error' ? section.error : '');
  let critters = $derived(
    section?.status === 'ok' ? section.data.map((c) => liveCritter(c, clock.nowSecs, tz)) : []
  );

  let todayIndex = $derived(localWeekday(clock.nowSecs, tz));
  let selectedDay = $state<number | null>(null);
  let activeDay = $derived(selectedDay ?? todayIndex);
  let isToday = $derived(activeDay === todayIndex);
  let prevDayName = $derived(WEEKDAY_NAMES[(activeDay + 6) % 7]);
  let nextDayName = $derived(WEEKDAY_NAMES[(activeDay + 1) % 7]);

  const onlyAvailable = new PersistedState('critters.availableNow', false);
  const onlyToFeed = new PersistedState('critters.toFeed', false);
  const onlyUntamed = new PersistedState('critters.untamed', false);

  let dayCritters = $derived.by(() => {
    let list = critters.filter((c) => c.schedule[activeDay].length > 0);
    if (isToday && onlyAvailable.current) list = list.filter((c) => c.availableNow);
    if (isToday && onlyToFeed.current) list = list.filter((c) => c.needsFeeding);
    if (onlyUntamed.current) list = list.filter((c) => !c.tamed);
    return list.toSorted(compareOnDay(activeDay));
  });
  let anyFilterActive = $derived(
    onlyUntamed.current || (isToday && (onlyAvailable.current || onlyToFeed.current))
  );

  function prevDay() {
    selectedDay = (activeDay + 6) % 7;
  }

  function nextDay() {
    selectedDay = (activeDay + 1) % 7;
  }

  function goToToday() {
    selectedDay = null;
  }

  function compareOnDay(day: number) {
    return (a: LiveCritter, b: LiveCritter) => {
      const sa = a.schedule[day][0];
      const sb = b.schedule[day][0];
      return sa.start - sb.start || sb.end - sa.end || a.speciesRank - b.speciesRank;
    };
  }
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
          <p class="day-note">{WEEKDAY_NAMES[activeDay]}</p>
        {:else}
          <h2>{WEEKDAY_NAMES[activeDay]}</h2>
          <button class="today-link" onclick={goToToday}>return to today</button>
        {/if}
      </div>
      <button class="arrow next" onclick={nextDay} aria-label={`Next day, ${nextDayName}`}
        ><span class="day-name">{nextDayName}</span>☞</button
      >
    </nav>

    <div class="filters">
      <span class="filters-label">filter by:</span>
      {#if isToday}
        <FilterToggle
          glyph="●"
          tone="primary"
          label="available now"
          active={onlyAvailable.current}
          onclick={() => (onlyAvailable.current = !onlyAvailable.current)}
        />
        <FilterToggle
          glyph="✓"
          label="to feed"
          active={onlyToFeed.current}
          onclick={() => (onlyToFeed.current = !onlyToFeed.current)}
        />
      {/if}
      <FilterToggle
        glyph="♥"
        tone="accent"
        label="untamed"
        active={onlyUntamed.current}
        onclick={() => (onlyUntamed.current = !onlyUntamed.current)}
      />
    </div>

    {#if dayCritters.length === 0}
      <p class="status">
        <em>{anyFilterActive ? 'None match.' : 'No creatures stir this day.'}</em>
      </p>
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
            <span class="biome">{critter.biome}</span>
            <span class="leader" aria-hidden="true"></span>
            <span class="time">{formatSchedule(critter.schedule[activeDay])}</span>
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

  .biome {
    flex: none;
    font-size: var(--font-size-sm);
    color: var(--color-text-subtle);
  }
</style>
