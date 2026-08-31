import { writable, derived } from 'svelte/store';
import type { Settings, SettingsUpdate } from '../api/types';
import * as api from '../api/commands';

const _settings = writable<Settings | null>(null);

export const settings = { subscribe: _settings.subscribe };

function createSettingsStore() {
  return {
    subscribe: _settings.subscribe,
    async load() {
      try {
        const settings = await api.getSettings();
        _settings.set(settings);
        return settings;
      } catch (e) {
        console.error('Failed to load settings:', e);
        // Return default settings on error
        const defaultSettings: Settings = {
          theme: 'system',
          first_run_complete: false,
          minimum_task_duration: 1,
          work_day_hours: 7.5,
          default_distribution_strategy: 'proportional',
        };
        _settings.set(defaultSettings);
        return defaultSettings;
      }
    },
    async update(updates: SettingsUpdate) {
      try {
        const settings = await api.updateSettings(updates);
        _settings.set(settings);
        return settings;
      } catch (e) {
        console.error('Failed to update settings:', e);
        throw e;
      }
    },
    async checkFirstRun() {
      try {
        return await api.isFirstRun();
      } catch (e) {
        console.error('Failed to check first run:', e);
        return true; // Assume first run on error
      }
    },
    async completeFirstRun(initialSettings: Partial<Settings>) {
      try {
        const settings = await api.completeFirstRun(initialSettings as Settings);
        _settings.set(settings);
        return settings;
      } catch (e) {
        console.error('Failed to complete first run:', e);
        throw e;
      }
    },
  };
}

export const settingsStore = createSettingsStore();
