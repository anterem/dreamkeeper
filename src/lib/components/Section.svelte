<script lang="ts">
  import type { Snippet } from 'svelte';

  let {
    title,
    href,
    summary,
    children
  }: { title: string; href?: string; summary?: string; children: Snippet } = $props();
</script>

<section>
  <h2 class="overline">
    <svelte:element this={href ? 'a' : 'span'} {href} class="heading">
      {title}
      {#if href}<span class="arrow" aria-hidden="true">☞</span>{/if}
      {#if summary}<span class="summary">{summary}</span>{/if}
    </svelte:element>
  </h2>
  {@render children()}
</section>

<style>
  section {
    width: fit-content;
    margin-inline: auto;
    margin-bottom: var(--space-7);
  }

  h2 {
    margin-bottom: var(--space-4);
    font-size: var(--font-size-md);
  }

  .heading {
    color: inherit;
    text-decoration: none;
    display: flex;
    align-items: baseline;
    gap: var(--space-3);
  }

  a.heading:hover {
    color: var(--color-primary);
  }

  .arrow {
    letter-spacing: 0;
  }

  .summary {
    margin-left: auto;
    color: var(--color-text-subtle);
    font-family: var(--font-body);
    letter-spacing: 0;
    text-transform: none;
  }
</style>
