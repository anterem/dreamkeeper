<script lang="ts">
  import { onMount } from 'svelte';
  import { commands, type SaveFile } from '../lib/bindings';
  import { formatTimestamp } from '../lib/utils';

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

<main class="container">
  <h1>Dreamkeeper</h1>

  {#if saveFilesError}
    <p>{saveFilesError}</p>
  {:else if saveFiles.length === 0}
    No save file found.
  {:else}
    <ul>
      {#each saveFiles as file}
        <li>{file.storefront} ({formatTimestamp(file.lastModified)})</li>
      {/each}
    </ul>
  {/if}
</main>

<style>
</style>
