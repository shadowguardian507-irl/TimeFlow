<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import ThemeSelector from './ThemeSelector.svelte';
  import { settingsStore } from '../stores/settings';
  import * as api from '../api/commands';
  import type { Theme } from '../api/types';

  const dispatch = createEventDispatcher();

  let step = 1;
  let selectedTheme: Theme = 'system';
  let importing = false;
  let importError = '';
  let legacyDataAvailable = false;

  onMount(async () => {
    try {
      legacyDataAvailable = await api.hasLegacyData();
    } catch (e) {
      console.error('Failed to check for legacy data:', e);
    }
  });

  async function nextStep() {
    if (step < 3) {
      step++;
    } else {
      await completeSetup();
    }
  }

  function previousStep() {
    if (step > 1) {
      step--;
    }
  }

  async function completeSetup() {
    try {
      await settingsStore.completeFirstRun({
        theme: selectedTheme,
        first_run_complete: true,
        minimum_task_duration: 1,
        work_day_hours: 7.5,
        default_distribution_strategy: 'proportional',
      });
      dispatch('complete');
    } catch (e) {
      console.error('Failed to complete first run:', e);
      // Still dispatch complete to allow user to proceed
      dispatch('complete');
    }
  }

  async function handleImport() {
    const path = prompt('Enter backup file path:');
    if (!path) return;

    importing = true;
    importError = '';

    try {
      await api.importBackup(path);
      await completeSetup();
    } catch (e) {
      importError = 'Failed to import backup. Please check the file path.';
    } finally {
      importing = false;
    }
  }

  async function handleLegacyImport() {
    importing = true;
    importError = '';

    try {
      await api.importLegacyData();
      await settingsStore.load();
      dispatch('complete');
    } catch (e) {
      importError = e instanceof Error ? e.message : 'Failed to import alpha data';
    } finally {
      importing = false;
    }
  }

  function skipImport() {
    completeSetup();
  }
</script>

<div class="dialog-overlay" data-testid="first-run-dialog">
  <div class="dialog">
    {#if step === 1}
      <div class="step">
        <div class="icon">⏱️</div>
        <h2>Welcome to TimeFlow</h2>
        <p>Your personal time tracking companion for ActiTime integration.</p>
        <p class="features">
          Track your time throughout the day, manage categories, and easily copy your entries to ActiTime.
        </p>
        <button class="primary-btn" on:click={nextStep}>
          Get Started
        </button>
      </div>
    {:else if step === 2}
      <div class="step">
        <h2>Choose Your Theme</h2>
        <p>Select how you'd like TimeFlow to look.</p>

        <div class="theme-section">
          <ThemeSelector />
        </div>

        <div class="nav-buttons">
          <button class="secondary-btn" on:click={previousStep}>
            Back
          </button>
          <button class="primary-btn" on:click={nextStep}>
            Continue
          </button>
        </div>
      </div>
    {:else if step === 3}
      <div class="step">
        <h2>Import Existing Data?</h2>
        <p>If you have a backup from a previous installation, you can import it now.</p>

        {#if importError}
          <div class="error-message" role="alert">{importError}</div>
        {/if}

        <div class="import-section">
          {#if legacyDataAvailable}
            <p class="legacy-notice">Data from an earlier alpha installation was found.</p>
            <button
              class="import-btn"
              on:click={handleLegacyImport}
              disabled={importing}
            >
              {importing ? 'Importing...' : 'Import Alpha Data'}
            </button>
          {/if}

          <button
            class="import-btn"
            on:click={handleImport}
            disabled={importing}
          >
            {importing ? 'Importing...' : '📥 Import Backup'}
          </button>
        </div>

        <div class="nav-buttons">
          <button class="secondary-btn" on:click={previousStep}>
            Back
          </button>
          <button class="primary-btn" on:click={skipImport}>
            Start Fresh
          </button>
        </div>
      </div>
    {/if}

    <div class="progress">
      {#each [1, 2, 3] as s}
        <span class="dot" class:active={step >= s}></span>
      {/each}
    </div>
  </div>
</div>

<style>
  .dialog-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 200;
  }

  .dialog {
    background: var(--bg-primary);
    border-radius: 16px;
    padding: 2rem;
    width: 100%;
    max-width: 450px;
    text-align: center;
  }

  .step {
    min-height: 280px;
    display: flex;
    flex-direction: column;
    justify-content: center;
  }

  .icon {
    font-size: 4rem;
    margin-bottom: 1rem;
  }

  h2 {
    margin: 0 0 0.5rem;
    color: var(--text-primary);
  }

  p {
    color: var(--text-secondary);
    margin: 0 0 1rem;
  }

  .features {
    font-size: 0.9rem;
    margin-bottom: 2rem !important;
  }

  .theme-section {
    margin: 1.5rem 0;
    text-align: left;
  }

  .import-section {
    margin: 1.5rem 0;
  }

  .error-message {
    background: var(--error-bg);
    color: var(--error-color);
    padding: 0.75rem;
    border-radius: 6px;
    margin-bottom: 1rem;
    font-size: 0.875rem;
  }

  .nav-buttons {
    display: flex;
    justify-content: center;
    gap: 1rem;
    margin-top: 1.5rem;
  }

  .primary-btn, .secondary-btn, .import-btn {
    padding: 0.75rem 1.5rem;
    border-radius: 8px;
    font-size: 1rem;
    cursor: pointer;
    transition: opacity 0.15s ease;
  }

  .primary-btn {
    background: var(--accent-color);
    color: white;
    border: none;
  }

  .secondary-btn {
    background: var(--bg-secondary);
    color: var(--text-primary);
    border: 1px solid var(--border-color);
  }

  .import-btn {
    background: var(--bg-secondary);
    color: var(--text-primary);
    border: 1px solid var(--border-color);
    width: 100%;
  }

  .import-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .primary-btn:hover, .secondary-btn:hover, .import-btn:hover:not(:disabled) {
    opacity: 0.9;
  }

  .progress {
    display: flex;
    justify-content: center;
    gap: 0.5rem;
    margin-top: 2rem;
  }

  .dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--border-color);
    transition: background 0.15s ease;
  }

  .dot.active {
    background: var(--accent-color);
  }
</style>
