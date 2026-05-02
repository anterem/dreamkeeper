<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  let path = $state('');
  let pathError = $state('');

  async function getAppdataPath() {
    path = '';
    pathError = '';
    try {
      path = await invoke('get_appdata_path');
    } catch (e) {
      if (e instanceof Error) pathError = e.toString();
      else pathError = String(e);
    }
  }
</script>

<main class="container">
  <h1>Dreamkeeper</h1>

  <button type="button" onclick={getAppdataPath}>Get appdata path</button>
  {#if pathError}
    <p>{pathError}</p>
  {/if}
  <p>Path: {path}</p>
</main>

<style>
</style>
