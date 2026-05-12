<script lang="ts">
  let { title }: { title?: string } = $props();
</script>

<header class="page-header">
  <p class="overline">Dreamkeeper</p>
  {#if title}<h1>{title}</h1>{/if}
  <div class="divider" aria-hidden="true">⏾⋆.</div>
</header>

<style>
  .page-header {
    text-align: center;
    margin-bottom: var(--space-6);
  }

  .overline {
    font-family: var(--font-display);
    font-size: var(--font-size-sm);
    letter-spacing: 0.4em;
    color: var(--color-text-muted);
    text-transform: uppercase;
    margin-bottom: var(--space-2);
  }

  .page-header h1 {
    font-size: var(--font-size-3xl);
  }

  .page-header .divider {
    margin-top: var(--space-3);
    margin-bottom: 0;
    color: var(--color-primary);
  }
</style>
