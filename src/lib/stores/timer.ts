import { writable } from 'svelte/store';
import type { TimerInfo, TaskType, DistributionStrategy } from '../api/types';
import * as api from '../api/commands';

function createTimerStore() {
  const { subscribe, set } = writable<TimerInfo | null>(null);
  let pollInterval: number | null = null;

  return {
    subscribe,
    async load() {
      try {
        const state = await api.getTimerState();
        set(state);
        return state;
      } catch (e) {
        console.error('Failed to load timer state:', e);
        set(null);
        return null;
      }
    },
    async start(
      taskName: string,
      categoryPath: string | null,
      taskType: TaskType,
      distributionStrategy?: DistributionStrategy
    ) {
      const info = await api.startTimer(
        taskName,
        categoryPath,
        taskType,
        distributionStrategy
      );
      set(info);
      this.startPolling();
      return info;
    },
    async stop() {
      const task = await api.stopTimer();
      set(null);
      this.stopPolling();
      return task;
    },
    async discard() {
      await api.discardTimer();
      set(null);
      this.stopPolling();
    },
    startPolling() {
      if (pollInterval) return;
      pollInterval = window.setInterval(async () => {
        try {
          const state = await api.getTimerState();
          set(state);
        } catch (e) {
          console.error('Timer poll error:', e);
        }
      }, 1000);
    },
    stopPolling() {
      if (pollInterval) {
        clearInterval(pollInterval);
        pollInterval = null;
      }
    },
  };
}

export const timerStore = createTimerStore();
