<script lang="ts">
  import { settingsStore, settings } from '../stores/settings';
  import type { Theme } from '../api/types';
  import { applyTheme } from '../theme';

  const themes: { value: Theme; label: string; icon: string }[] = [
    { value: 'light', label: 'Light', icon: '☀️' },
    { value: 'dark', label: 'Dark', icon: '🌙' },
    { value: 'system', label: 'System', icon: '💻' },
  ];

  $: currentTheme = $settings?.theme ?? 'system';

  async function selectTheme(theme: Theme) {
    await settingsStore.update({ theme });
    applyTheme(theme);
  }

</script>

<div class="theme-selector" data-testid="theme-selector">
  <span id="theme-selector-label" class="label">Theme</span>
  <div class="theme-options" role="group" aria-labelledby="theme-selector-label">
    {#each themes as theme}
      <button
        type="button"
        class="theme-option"
        class:selected={currentTheme === theme.value}
        on:click={() => selectTheme(theme.value)}
        data-testid="theme-{theme.value}"
      >
        <span class="icon">{theme.icon}</span>
        <span class="name">{theme.label}</span>
      </button>
    {/each}
  </div>
</div>

<style>
  .theme-selector {
    margin-bottom: 1rem;
  }

  .label {
    display: block;
    font-weight: 500;
    color: var(--text-primary);
    margin-bottom: 0.75rem;
  }

  .theme-options {
    display: flex;
    gap: 0.75rem;
  }

  .theme-option {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    padding: 1rem;
    background: var(--bg-primary);
    border: 2px solid var(--border-color);
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .theme-option:hover {
    border-color: var(--accent-color);
  }

  .theme-option.selected {
    border-color: var(--accent-color);
    background: var(--accent-bg);
  }

  .icon {
    font-size: 1.5rem;
  }

  .name {
    font-size: 0.875rem;
    color: var(--text-primary);
  }
</style>
