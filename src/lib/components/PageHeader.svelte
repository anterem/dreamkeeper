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
    letter-spacing: 0.4em;
    margin-bottom: var(--space-2);
  }

  .page-header h1 {
    font-size: var(--font-size-3xl);
  }

  .page-header .divider {
    max-width: calc(100vw - 2 * var(--frame-content-inset));
    margin-top: var(--space-3);
    margin-bottom: 0;
    margin-inline: auto;
    color: var(--color-primary);
  }
</style>
