<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { commands, type SaveFile } from '$lib/bindings';
  import { loadedSaveFile } from '$lib/store';
  import { snapshot } from '$lib/snapshot.svelte';
  import { liveRelativeTime } from '$lib/clock.svelte';

  let saveFiles: SaveFile[] = $state([]);
  let saveFilesError = $state('');
  let loadError = $state('');
  let loading = $state(false);

  async function getSaveFiles() {
    const res = await commands.getSaveFiles();
    if (res.status !== 'ok') {
      saveFilesError = JSON.stringify(res.error);
      return;
    }
    saveFiles = res.data;
    if (saveFiles.length === 1) load(saveFiles[0]);
  }

  async function load(file: SaveFile) {
    loading = true;
    loadError = '';
    const result = await commands.loadSaveFile(file.path, file.storefront);
    if (result.status === 'ok') {
      await snapshot.refresh();
      loadedSaveFile.set(file);
      goto('/');
    } else {
      loading = false;
      loadError = JSON.stringify(result.error);
    }
  }

  onMount(getSaveFiles);
</script>

<main>
  {#if saveFilesError}
    <p class="status">An ill omen: <span class="error">{saveFilesError}</span></p>
  {:else if loadError}
    <p class="status">Could not read save: <span class="error">{loadError}</span></p>
  {:else if loading}
    <p class="status"><em>Reading save file…</em></p>
  {:else if saveFiles.length === 0}
    <p class="status">No save files were found in the usual places.</p>
  {:else}
    <p class="status"><em>Choose one to continue.</em></p>
    <ul class="ruled">
      {#each saveFiles as file}
        <li>
          <label>
            <input type="radio" name="save-file" onchange={() => load(file)} />
            <span>
              <span class="storefront">{file.storefront}</span>
              Last updated <em>{liveRelativeTime(file.lastModified)}</em>
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
    margin-inline: auto;
  }

  .storefront::after {
    content: '•';
    margin-inline: 1rem;
  }

  .storefront {
    text-transform: uppercase;
  }
</style>
