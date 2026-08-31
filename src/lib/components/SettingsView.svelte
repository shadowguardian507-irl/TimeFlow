<script lang="ts">
  import { onMount } from 'svelte';
  import ThemeSelector from './ThemeSelector.svelte';
  import { settingsStore, settings } from '../stores/settings';
  import * as api from '../api/commands';
  import type { Theme, DistributionStrategy } from '../api/types';

  let minimumTaskDuration = 1;
  let workDayHours = 8;
  let defaultStrategy: DistributionStrategy = 'proportional';
  let saving = false;
  let message = '';

  onMount(async () => {
    await settingsStore.load();
    if ($settings) {
      minimumTaskDuration = $settings.minimum_task_duration;
      workDayHours = $settings.work_day_hours;
      defaultStrategy = $settings.default_distribution_strategy;
    }
  });

  async function handleSave() {
    saving = true;
    message = '';
    try {
      await settingsStore.update({
        minimum_task_duration: minimumTaskDuration,
        work_day_hours: workDayHours,
        default_distribution_strategy: defaultStrategy,
      });
      message = 'Settings saved!';
      setTimeout(() => message = '', 2000);
    } catch (e) {
      message = 'Failed to save settings';
    } finally {
      saving = false;
    }
  }

  async function handleExportBackup() {
    try {
      const path = await api.exportBackup();
      message = `Backup exported to: ${path}`;
    } catch (e) {
      message = 'Failed to export backup';
    }
  }

  async function handleImportBackup() {
    // In a real app, this would open a file picker
    const path = prompt('Enter backup file path:');
    if (!path) return;
    
    try {
      const result = await api.importBackup(path);
      message = `Imported: ${result.templates_count} templates`;
      await settingsStore.load();
    } catch (e) {
      message = 'Failed to import backup';
    }
  }

  async function handleExportCsv() {
    const startDate = prompt('Start date (YYYY-MM-DD):');
    if (!startDate) return;
    const endDate = prompt('End date (YYYY-MM-DD):');
    if (!endDate) return;
    
    try {
      const path = await api.exportCsv(startDate, endDate);
      message = `CSV exported to: ${path}`;
    } catch (e) {
      message = 'Failed to export CSV';
    }
  }
</script>

<div class="settings-view" data-testid="settings-view">
  <div class="view-header">
    <h2>Settings</h2>
  </div>

  {#if message}
    <div class="message" role="status">{message}</div>
  {/if}

  <div class="settings-section">
    <h3>Appearance</h3>
    <ThemeSelector />
  </div>

  <div class="settings-section">
    <h3>Time Tracking</h3>
    
    <div class="setting-row">
      <div class="setting-info">
        <label for="minDuration">Minimum Task Duration</label>
        <p class="description">Tasks must be at least this many minutes</p>
      </div>
      <div class="setting-control">
        <input 
          id="minDuration"
          type="number"
          bind:value={minimumTaskDuration}
          min="1"
          max="60"
        />
        <span class="unit">min</span>
      </div>
    </div>

    <div class="setting-row">
      <div class="setting-info">
        <label for="workDay">Work Day Hours</label>
        <p class="description">Show warning when daily time exceeds this</p>
      </div>
      <div class="setting-control">
        <input 
          id="workDay"
          type="number"
          bind:value={workDayHours}
          min="1"
          max="24"
        />
        <span class="unit">hours</span>
      </div>
    </div>

    <div class="setting-row">
      <div class="setting-info">
        <label for="defaultStrategy">Default Distribution Strategy</label>
        <p class="description">Default strategy for mergeable tasks</p>
      </div>
      <div class="setting-control">
        <select id="defaultStrategy" bind:value={defaultStrategy}>
          <option value="proportional">Proportional</option>
          <option value="even">Even</option>
          <option value="manual">Manual</option>
          <option value="weighted">Weighted</option>
        </select>
      </div>
    </div>

    <div class="save-row">
      <button class="save-btn" on:click={handleSave} disabled={saving}>
        {saving ? 'Saving...' : 'Save Settings'}
      </button>
    </div>
  </div>

  <div class="settings-section">
    <h3>Data Management</h3>
    
    <div class="data-actions">
      <button class="action-btn" on:click={handleExportBackup}>
        📦 Export Backup
      </button>
      <button class="action-btn" on:click={handleImportBackup}>
        📥 Import Backup
      </button>
      <button class="action-btn" on:click={handleExportCsv}>
        📊 Export CSV
      </button>
    </div>
  </div>
</div>

<style>
  .settings-view {
    max-width: 600px;
  }

  .view-header h2 {
    margin: 0 0 1.5rem;
    color: var(--text-primary);
  }

  .message {
    padding: 0.75rem 1rem;
    background: var(--success-bg);
    color: var(--success-color);
    border-radius: 6px;
    margin-bottom: 1.5rem;
  }

  .settings-section {
    background: var(--bg-secondary);
    border-radius: 8px;
    padding: 1.5rem;
    margin-bottom: 1.5rem;
  }

  .settings-section h3 {
    margin: 0 0 1rem;
    font-size: 1rem;
    color: var(--text-primary);
  }

  .setting-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 0;
    border-bottom: 1px solid var(--border-color);
  }

  .setting-row:last-of-type {
    border-bottom: none;
  }

  .setting-info label {
    display: block;
    font-weight: 500;
    color: var(--text-primary);
    margin-bottom: 0.25rem;
  }

  .setting-info .description {
    margin: 0;
    font-size: 0.8rem;
    color: var(--text-secondary);
  }

  .setting-control {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .setting-control input {
    width: 80px;
    padding: 0.5rem;
    border: 1px solid var(--border-color);
    border-radius: 6px;
    background: var(--bg-primary);
    color: var(--text-primary);
    text-align: right;
  }

  .setting-control select {
    padding: 0.5rem;
    border: 1px solid var(--border-color);
    border-radius: 6px;
    background: var(--bg-primary);
    color: var(--text-primary);
  }

  .unit {
    font-size: 0.875rem;
    color: var(--text-secondary);
  }

  .save-row {
    margin-top: 1rem;
    display: flex;
    justify-content: flex-end;
  }

  .save-btn {
    padding: 0.625rem 1.5rem;
    background: var(--accent-color);
    color: white;
    border: none;
    border-radius: 6px;
    cursor: pointer;
  }

  .save-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .data-actions {
    display: flex;
    gap: 1rem;
    flex-wrap: wrap;
  }

  .action-btn {
    padding: 0.75rem 1rem;
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    cursor: pointer;
    color: var(--text-primary);
    font-size: 0.9rem;
  }

  .action-btn:hover {
    background: var(--bg-hover);
  }
</style>
