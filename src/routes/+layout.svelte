<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/state';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import { snapshot } from '$lib/snapshot.svelte';
  import { liveRelativeTime } from '$lib/clock.svelte';
  import '../app.css';

  let { children } = $props();

  let isHome = $derived(page.url.pathname === '/');
  let lastUpdated = $derived(
    snapshot.current ? liveRelativeTime(snapshot.current.modifiedSecs) : ''
  );

  onMount(() => {
    snapshot.init();
  });
</script>

<div id="page">
  <div class="top-bar">
    {#if !isHome}
      <a class="home-link" href="/"><span class="arrow" aria-hidden="true">☜</span> home</a>
    {/if}
    {#if snapshot.current}
      <p class="last-updated">updated <span class="time">{lastUpdated}</span></p>
    {/if}
  </div>
  <PageHeader title={page.data.title} />

  <div class="page-inner">
    {@render children()}
  </div>
</div>

<style>
  #page {
    display: flex;
    flex-direction: column;
    position: relative;
    z-index: 2;
    min-height: 100vh;
    padding: var(--space-6);
  }

  #page::before,
  #page::after {
    content: '';
    position: absolute;
    top: 0;
    bottom: 0;
    width: 10px;
    background-color: color-mix(in srgb, var(--color-primary) 45%, transparent);
    -webkit-mask-image: var(--line-v);
    mask-image: var(--line-v);
    -webkit-mask-repeat: no-repeat;
    mask-repeat: no-repeat;
    -webkit-mask-size: 100% 100%;
    mask-size: 100% 100%;
  }

  #page::before {
    left: var(--frame-inset);
  }
  #page::after {
    right: var(--frame-inset);
  }

  .page-inner {
    flex: 1;
    position: relative;
    width: 100%;
    max-width: 56rem;
    margin: 0 auto;
  }

  .top-bar {
    position: absolute;
    top: var(--space-3);
    left: var(--frame-content-inset);
    right: var(--frame-content-inset);
    z-index: 3;
    display: flex;
    align-items: baseline;
    justify-content: space-between;
  }

  .home-link {
    font-family: var(--font-display);
    font-size: var(--font-size-md);
    color: var(--color-text-muted);
    text-decoration: none;
    transition: color var(--duration-fast) var(--ease-out);
  }
  .home-link:hover {
    color: var(--color-primary);
  }

  .last-updated {
    margin: 0;
    margin-left: auto;
    font-family: var(--font-display);
    font-size: var(--font-size-md);
    color: var(--color-text-muted);
  }

  .last-updated .time {
    font-family: var(--font-body);
    font-style: italic;
    font-size: var(--font-size-sm);
  }
</style>
