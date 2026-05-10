<script lang="ts">
  import { onMount } from 'svelte';
  import { commands, type SaveFile } from '../lib/bindings';
  import { formatRelativeTime } from '../lib/utils';

  let selectedSaveFile = $state<SaveFile | null>(null);
  let saveFiles: SaveFile[] = $state([]);
  let saveFilesError = $state('');

  async function getSaveFiles() {
    let res = await commands.getSaveFiles();
    if (res.status === 'ok') saveFiles = res.data;
    else saveFilesError = JSON.stringify(res.error);
  }

  onMount(() => {
    getSaveFiles();
  });
</script>

<main>
  <header class="page-header">
    <p class="overline">Dreamkeeper</p>
    <h1>Discovering Save Files</h1>
    <div class="divider" aria-hidden="true">⏾⋆.</div>
  </header>

  {#if saveFilesError}
    <p>An ill omen: <span class="error">{saveFilesError}</span></p>
  {:else if saveFiles.length === 0}
    <p>No save files were found in the usual places.</p>
  {:else}
    <p><em>Choose one to continue.</em></p>
    <ul>
      {#each saveFiles as file}
        <li>
          <label>
            <input type="radio" name="save-file" value={file} bind:group={selectedSaveFile} />
            <span>
              <span class="storefront">{file.storefront}</span>
              Last updated <em>{formatRelativeTime(file.lastModified)}</em>
            </span>
          </label>
        </li>
      {/each}
    </ul>
  {/if}
</main>

<style>
  main {
    max-width: 44rem;
  }

  p {
    text-align: center;
    color: var(--color-text-subtle);
    padding-bottom: var(--space-5);
  }

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
  .storefront::after {
    content: '•';
    margin-inline: 1rem;
  }

  .storefront {
    text-transform: uppercase;
  }
</style>
