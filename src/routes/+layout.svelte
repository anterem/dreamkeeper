<script lang="ts">
  import { page } from '$app/stores';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import '../app.css';

  let { children } = $props();
</script>

<div id="page">
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
    left: clamp(var(--space-4), 16vw - 5rem, var(--space-7));
  }
  #page::after {
    right: clamp(var(--space-4), 16vw - 5rem, var(--space-7));
  }

  .page-inner {
    flex: 1;
    position: relative;
    max-width: 56rem;
    margin: 0 auto;
  }
</style>
