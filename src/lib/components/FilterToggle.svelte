<script lang="ts">
  let {
    label,
    glyph,
    tone = 'default',
    active = false,
    onclick
  }: {
    label: string;
    glyph?: string;
    tone?: 'default' | 'primary' | 'accent';
    active?: boolean;
    onclick: () => void;
  } = $props();
</script>

<button type="button" class="filter" class:active aria-pressed={active} {onclick}>
  {#if glyph}<span class="glyph {tone}" aria-hidden="true">{glyph}</span>{/if}{label}
</button>

<style>
  .filter {
    background: none;
    border: 0;
    padding: 0;
    cursor: pointer;
    font-size: var(--font-size-md);
    color: var(--color-text-muted);
    transition: color var(--duration-fast) var(--ease-out);
  }

  .filter:hover {
    color: var(--color-text-subtle);
  }

  .filter.active {
    color: var(--color-primary);
    font-style: italic;
  }

  .glyph {
    margin-right: 0.3em;
  }

  .glyph.primary {
    color: var(--color-primary);
  }

  .glyph.accent {
    color: var(--color-accent);
  }
</style>
