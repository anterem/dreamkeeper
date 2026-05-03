<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  let saveFiles: string[] = $state([]);
  let saveFilesError = $state('');

  async function getSaveFiles() {
    try {
      saveFiles = await invoke('get_save_files');
    } catch (e) {
      saveFilesError = JSON.stringify(e);
    }
  }

  $effect(() => {
    getSaveFiles();
  });
</script>

<main class="container">
  <h1>Dreamkeeper</h1>

  {#if saveFilesError}
    <p>{saveFilesError}</p>
  {/if}
  {#if saveFiles.length === 1}
    Save file found: {saveFiles[0]}
  {/if}
  {#if saveFiles.length > 1}
    <p>Save files found:</p>
    <ul>
      {#each saveFiles as file}
        <li>{file}</li>
      {/each}
    </ul>
  {/if}
</main>

<style>
</style>
