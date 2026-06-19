<script lang="ts">
  import { page } from '$app/stores';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import '../app.css';

  let { children } = $props();

  let isHome = $derived($page.url.pathname === '/');
</script>

<div id="page">
  {#if !isHome}
    <a class="home-link" href="/"><span class="arrow" aria-hidden="true">☜</span> home</a>
  {/if}
  <PageHeader title={$page.data.title} />

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

  .home-link {
    position: absolute;
    top: var(--space-6);
    left: var(--frame-content-inset);
    z-index: 3;
    font-family: var(--font-display);
    font-size: var(--font-size-md);
    color: var(--color-text-muted);
    text-decoration: none;
    transition: color var(--duration-fast) var(--ease-out);
  }
  .home-link:hover {
    color: var(--color-primary);
  }
</style>
