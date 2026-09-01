<script lang="ts">
  import { onMount } from 'svelte';
  import Layout from './lib/components/Layout.svelte';
  import FirstRunDialog from './lib/components/FirstRunDialog.svelte';
  import { settingsStore } from './lib/stores/settings';
  import { timerStore } from './lib/stores/timer';
  import { applyTheme } from './lib/theme';

  let showFirstRun = false;
  let loading = true;

  onMount(async () => {
    const isFirstRun = await settingsStore.checkFirstRun();
    showFirstRun = isFirstRun;

    const loadedSettings = await settingsStore.load();
    applyTheme(loadedSettings.theme);
    await timerStore.load();

    // Start polling if timer is running
    const timerState = await timerStore.load();
    if (timerState?.is_running) {
      timerStore.startPolling();
    }

    loading = false;
  });

  function handleFirstRunComplete() {
    showFirstRun = false;
  }
</script>

{#if loading}
  <div class="loading" data-testid="app-loading">
    <p>Loading TimeFlow...</p>
  </div>
{:else if showFirstRun}
  <FirstRunDialog on:complete={handleFirstRunComplete} />
{:else}
  <Layout />
{/if}

<style>
  .loading {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100vh;
    font-size: 1.2rem;
    color: var(--text-secondary);
  }
</style>
